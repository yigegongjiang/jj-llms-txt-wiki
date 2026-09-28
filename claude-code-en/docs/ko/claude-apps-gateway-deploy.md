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
  **프라이빗 네트워크에 배포하세요.** Claude Code는 주소가 프라이빗인 게이트웨이에만 연결합니다. 이는 보안 가드입니다. 신뢰할 수 있는 게이트웨이는 개발자 머신에서 명령을 실행하는 설정을 푸시할 수 있기 때문입니다. 게이트웨이를 내부 로드 밸런서 또는 VPN 뒤에 배치하고 프라이빗 IP로만 확인되는 호스트명을 지정하세요. 내부 네트워크가 조직이 소유한 공개 IPv4 공간에서 번호가 지정된 경우 [소유한 공개 주소 공간에서 게이트웨이 허용](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)을 참조하세요.
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

<Warning>
  새로고침 토큰을 사용하면 게이트웨이가 개발자를 브라우저로 다시 보내지 않고 개발자의 세션을 자동으로 갱신할 수 있습니다. 또한 IdP가 사용자를 비활성화할 때 다음 새로고침이 실패하고 세션이 `ttl_hours` 내에 종료되므로 프로비저닝 해제를 주도합니다. 게이트웨이는 기본적으로 새로고침 토큰을 얻기 위해 `offline_access`를 요청합니다. IdP가 오프라인 액세스에 대한 명시적 동의를 요구하는 경우 OAuth 클라이언트를 구성하여 이를 허용하세요.

  IdP가 전혀 새로고침 토큰을 발급할 수 없는 경우, 게이트웨이는 여전히 작동하지만 자동 갱신이 없으므로 개발자는 세션이 만료될 때 브라우저 로그인을 다시 실행합니다. 이것이 매시간 발생하지 않도록 하려면 [`session.ttl_hours`](/docs/ko/claude-apps-gateway-config#session)를 `8` 또는 `12`로 올리세요. 트레이드오프는 프로비저닝 해제 지연입니다. 새로고침 토큰이 없으면 비활성화된 사용자는 더 긴 TTL이 경과할 때까지 액세스를 유지합니다.
</Warning>

<h2 id="deployment">
  배포
</h2>

게이트웨이는 단일 상태 비저장 Linux 바이너리이며 Postgres를 통해 조정되므로 환경에서 다른 상태 비저장 서비스를 배포하는 방식대로 배포하세요. 네트워크 내부에 유지하여 개발자와 IdP가 HTTPS를 통해 도달할 수 있도록 하고, 프로덕션 자격 증명을 보유하는 다른 서비스처럼 취급하세요.

배포를 실행 위치 이상으로 형성하는 몇 가지 결정이 있습니다:

* **비용**: 게이트웨이에 대한 별도의 라이선스 또는 사용자당 요금이 없습니다. 게이트웨이는 `claude` 바이너리의 일부이므로 기존 약정을 통해 추론에 대해 비용을 지불하고, 실행되는 컴퓨팅에 대해 비용을 지불합니다.
* **우회**: 게이트웨이는 모델로의 유일한 경로가 이를 통과하도록 강제하지 않습니다. 자신의 자격 증명이 있는 개발자는 여전히 공급자를 직접 호출할 수 있으므로, 해당 경로를 닫는 것은 네트워크 정책 결정입니다(예: `api.anthropic.com`으로의 송신을 게이트웨이를 제외하고 차단). 해당 송신을 차단하면 각 개발자의 머신에서 `api.anthropic.com`을 호출하는 [WebFetch 도메인 안전 확인](/docs/ko/data-usage#webfetch-domain-safety-check)도 중단됩니다. 관리형 정책에서 `skipWebFetchPreflight: true`를 설정하여 비활성화하세요.
* **다중 게이트웨이**: 각 게이트웨이는 자신의 구성을 가진 별도의 배포이며, CLI는 게이트웨이 호스트명별로 신뢰 및 자격 증명을 저장하므로, 팀이 충돌 없이 다른 게이트웨이를 사용할 수 있습니다. 여러 OIDC 발급자를 제공하려면 별도의 인스턴스를 실행하세요.
* **서버리스**: Cloud Run은 `min-instances: 1`을 설정하여 콜드 OIDC 검색을 피하면 작동합니다. Lambda 및 Cloud Functions는 작동하지 않습니다. 게이트웨이는 장기 실행 HTTP 서버이기 때문입니다.

여기의 모든 프로덕션 토폴로지는 일반 HTTP 복제본 앞에 Ingress, Cloud Run의 프론트 엔드 또는 ALB와 같은 L7 프록시를 배치합니다. [`listen.trusted_proxies`](/docs/ko/claude-apps-gateway-config#listen)를 프록시의 소스 범위로 설정하여 게이트웨이가 `X-Forwarded-For`에서 클라이언트 IP를 읽도록 하세요. 게이트웨이는 TCP 피어가 신뢰할 수 있을 때만 헤더를 인정합니다. [Google Cloud](/docs/ko/claude-apps-gateway-on-gcp) 및 [AWS](/docs/ko/claude-apps-gateway-on-aws) 작동 예제는 토폴로지별 구체적인 값을 가지고 있습니다. 신뢰할 수 있는 프록시가 없으면, 모든 요청이 프록시의 IP에서 오는 것으로 나타나므로 IP당 속도 제한이 하나의 공유 버킷으로 축소되고 감사 이벤트에 프록시의 IP가 기록됩니다.

게이트웨이의 디바이스 인증 및 토큰 엔드포인트로의 요청을 리디렉션하지 마세요. 예를 들어 HTTP-to-HTTPS 또는 호스트 정규화 재작성을 사용하는 수신 규칙으로 리디렉션하지 마세요. Claude Code는 해당 요청에서 리디렉션을 따르지 않으므로, 해당 요청을 리디렉션하는 수신 규칙은 로그인 및 토큰 새로고침을 중단합니다.

프록시에 게이트웨이의 keepalive 간격보다 긴 유휴 타임아웃을 제공하세요. 이는 업스트림에 따라 다릅니다:

* `provider: anthropic`을 제외한 모든 업스트림에서, 게이트웨이는 스트림이 약 15초 동안 조용한 후 SSE `ping`을 한 번 작성합니다.
* `provider: anthropic`에서, 게이트웨이는 Anthropic API의 자체 ping을 포함하여 응답을 변경하지 않고 전달합니다.

ALB의 60초와 같은 기본값은 조용한 스트림을 열린 상태로 유지하기에 충분합니다. [AWS 작동 예제](/docs/ko/claude-apps-gateway-on-aws#troubleshooting)는 어쨌든 이를 1시간으로 올리며, 해당 문제 해결 행은 이제 ping을 받는 업스트림에서 조용한 기간 동안 아무것도 보내지 않은 v2.1.229보다 오래된 게이트웨이를 다룹니다.

<h3 id="container-image">
  컨테이너 이미지
</h3>

표준 Claude Code 릴리스의 네이티브 `claude` 바이너리 주위에 자신의 이미지를 빌드하세요:

1. 고정된 릴리스에서 이미지 아키텍처용 Linux 빌드를 다운로드하세요. [특정 버전 설치](/docs/ko/setup#install-a-specific-version)에서 다운로드 URL을 참조하세요.
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

AWS에서 ECS Fargate 또는 EKS, Amazon RDS 및 AWS Secrets Manager를 다루는 완전한 작동 예제는 [AWS에 배포](/docs/ko/claude-apps-gateway-on-aws)를 참조하세요.

정적 키보다 플랫폼의 워크로드 ID를 선호하세요. [`upstreams` 참조](/docs/ko/claude-apps-gateway-config#upstreams)는 플랫폼별 설정 세부사항을 가지고 있습니다. Amazon Bedrock 업스트림을 GKE에서 사용하는 경우와 같은 클라우드 간 페어링의 경우, 업스트림의 `auth` 블록에서 명시적 자격 증명을 설정하세요.

<h3 id="cloud-run">
  Cloud Run
</h3>

서비스를 다음과 같이 구성하세요:

* `listen.port`를 기본값 `8080`으로 유지하세요. 이는 Cloud Run의 기본 `PORT`와 일치하거나 `port: ${PORT}`를 설정하세요
* `public_url`을 외부에서 도달 가능한 원본으로 설정하세요. 프로덕션의 경우 이는 일반적으로 내부 로드 밸런서의 호스트명입니다. `/login`이 [공개 주소를 거부](/docs/ko/claude-apps-gateway#prerequisites)하고 `*.run.app` URL이 하나로 확인되므로, Cloud Run URL 단독은 `curl` 또는 브라우저 스모크 테스트에만 작동합니다. 예외는 `*.run.app`이 Private Service Connect를 통해 프라이빗으로 확인되고 Cloud DNS 프라이빗 영역이 있는 네트워크입니다. 해당 토폴로지에서 Cloud Run URL은 유효한 `public_url`입니다. [Google Cloud 작동 예제](/docs/ko/claude-apps-gateway-on-gcp#deploy-the-gateway)는 둘 다 다룹니다.
* 구성을 시크릿 볼륨으로 마운트하세요
* 첫 번째 요청에서 콜드 OIDC 검색을 피하려면 `min-instances: 1`을 설정하세요

Google Cloud에서 Cloud Run 또는 GKE, Cloud SQL 및 Secret Manager를 다루는 완전한 작동 예제는 [Google Cloud에 배포](/docs/ko/claude-apps-gateway-on-gcp)를 참조하세요.

<h3 id="push-the-gateway-url-to-developer-machines">
  게이트웨이 URL을 개발자 머신으로 푸시
</h3>

게이트웨이가 제공되면, MDM을 통해 또는 OS별 `managed-settings.json`을 직접 작성하여 관리형 설정을 통해 각 개발자의 머신으로 `forceLoginMethod`, `forceLoginGatewayUrl` 및 `parentSettingsBehavior: "merge"`를 푸시하세요. 이 없이는 `/login`이 게이트웨이 옵션이 없는 표준 계정 선택기를 표시합니다.

키를 배포하면 Claude Code는 머신의 남은 API 키 또는 claude.ai 로그인을 사용하지 않으므로, 로그인 지침과 함께 푸시를 계획하세요. [관리자 정책에서 Cloud 게이트웨이 로그인 필요](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)는 개발자가 보는 메시지를 설명합니다.

각 메커니즘이 정책을 저장하는 위치는 [각 메커니즘이 정책을 저장하는 위치](/docs/ko/managed-settings#where-each-mechanism-stores-the-policy)를 참조하고, Claude Desktop `bootstrapUrl` 동등물은 [클라이언트 측 관리형 설정](/docs/ko/claude-apps-gateway-config#client-side-managed-settings)을 참조하세요.

<h3 id="large-rollouts">
  대규모 롤아웃
</h3>

로그인은 클라이언트 IP 주소별로 속도 제한되며, 기본값은 소규모 팀에 적합합니다. 각 주소는 10분마다 30개의 로그인 시작과 10개의 코드 제출을 받습니다. 수천 명의 개발자로의 롤아웃은 첫 번째 아침에 해당 제한에 도달할 수 있으며, 두 가지 이유 중 하나입니다:

* **게이트웨이가 로드 밸런서를 볼 수 없습니다.** [`listen.trusted_proxies`](/docs/ko/claude-apps-gateway-config#listen)가 없으면, 모든 개발자가 로드 밸런서의 주소에서 오는 것으로 나타나고 하나의 제한을 공유합니다. 다른 모든 것보다 먼저 설정하세요. 게이트웨이는 `X-Forwarded-For` 헤더를 무시할 때 처음으로 경고를 기록합니다.
* **많은 개발자가 몇 개의 NAT 또는 VPN 송신 주소를 공유합니다.** `trusted_proxies`가 올바를 때도 해당 주소의 제한을 공유합니다. [`rate_limits`](/docs/ko/claude-apps-gateway-config#http-tuning)를 맞추도록 올리세요.

`max`를 크기 조정하려면, 개발자를 공유하는 송신 주소로 나누세요. 기본값인 10분인 하나의 `window_seconds` 기간 내에 그 중 몇 명이 로그인하는지 추정하세요. 그런 다음 재시도 및 Claude Code와 Claude Desktop 모두에 로그인하는 개발자를 포함하도록 두 배로 늘리세요.

예를 들어, 4개의 송신 주소 뒤에 있는 10,000명의 개발자가 1시간에 걸쳐 균등하게 로그인합니다. 이는 주소당 2,500명의 개발자이고 각 10분마다 약 420명이며, 이를 두 배로 늘리고 1,000으로 올림합니다. 아래 예제는 두 제한을 모두 1,000으로 설정합니다:

```yaml theme={null}
rate_limits:
  device_authorization: { max: 1000, window_seconds: 600 }
  device_verify: { max: 1000, window_seconds: 600 }
```

`device_verify`는 누군가가 다른 개발자의 로그인 코드를 추측하는 것을 막는 것이므로, 추정이 필요한 만큼만 올리세요. 이 제한에서도, 코드는 20자 알파벳에서 8자이고 10분 후에 만료되므로, 추측은 비실용적으로 유지됩니다. [사용자 코드 무차별 대입 공격 저항](#user-code-brute-force-resistance)을 참조하세요.

IdP가 새로고침 토큰을 발급할 때, Claude Code는 세션을 자동으로 갱신하므로, 롤아웃 후 제한을 다시 설정할 수 있습니다. 새로고침 토큰이 없으면, 개발자는 [`session.ttl_hours`](/docs/ko/claude-apps-gateway-config#session)마다 다시 로그인합니다. 해당 정상 상태 속도에 대해 두 제한을 모두 크기 조정하고 올린 상태로 유지하세요.

제한에 도달하면, Claude Code v2.1.274 이상은 `The gateway is limiting sign-in attempts right now`를 표시합니다. v2.1.274 이상의 게이트웨이는 확인 페이지에 `Too many attempts came from your network address`를 표시하며, 확인할 설정이 있습니다. 또한 변경할 설정의 이름을 지정하는 `sign-in refused` 로그 라인을 작성합니다.

<h2 id="operations">
  운영
</h2>

게이트웨이가 트래픽을 제공하면, 일상적인 운영은 로그를 읽고, 상태를 프로브하며, 일정에 따라 시크릿을 로테이션하는 것입니다. 아래 섹션은 각각을 다루고, Postgres가 보유한 것과 업그레이드 및 롤백이 어떻게 동작하는지를 다룹니다.

<h3 id="logs">
  로그
</h3>

게이트웨이는 stderr에 두 개의 스트림을 작성하며, 둘 다 JSON 친화적입니다:

* **감사 이벤트**: 보안 관련 이벤트당 한 줄의 JSON. stderr를 로그 수집기로 파이프하세요.

  내보낸 이벤트는 `config.load`, `session.mint`, `session.refresh`, `device.authorize`, `device.verify`, `device.callback`, `auth.denied`, `access.denied`, `access.public_client`, `inference`, `managed.serve`, `desktop_bootstrap.serve`, `desktop_bootstrap.denied`, `spend.blocked`, `admin.denied`, `admin.limit.upsert` 및 `admin.limit.delete`를 포함합니다. 필드는 이벤트에 따라 다릅니다:

  * 성공적인 mint 및 refresh 이벤트는 `sub`, `email`, `client_ip` 및 결과를 전달합니다
  * `auth.denied` 및 `access.denied`는 이유 및 클라이언트 IP를 전달하며, `auth.denied`의 경우 요청 경로도 전달합니다. 이러한 거부 시 사용자 ID가 없기 때문입니다. 두 개의 `access.denied` 이유는 이벤트가 전달하는 것을 변경합니다:
    * `xff_unparseable`: 이벤트는 또한 읽을 수 없었던 `X-Forwarded-For` 항목을 전달합니다
    * `client_ip_unknown`: `access_control` 목록이 설정되었을 때 연결에 피어 주소가 없었기 때문에 이벤트는 클라이언트 IP를 전달하지 않습니다
  * `access.public_client`는 `access_control.allow_cidrs`가 비어 있는 동안 공개 주소에서 도착한 프로세스당 첫 번째 요청의 클라이언트 IP를 전달합니다. 게이트웨이는 요청을 평소대로 제공합니다. 이벤트는 게이트웨이가 공개 인터넷에서 도달 가능할 수 있음을 신호합니다. 공개로 간주되는 것과 권장 허용 목록에 대해서는 [`access_control` 참조](/docs/ko/claude-apps-gateway-config#http-tuning)를 참조하세요.
  * `inference`는 어느 업스트림이 요청을 제공했는지 및 응답 상태를 기록합니다
  * `desktop_bootstrap.denied`는 거부된 Claude Desktop 부트스트랩 가져오기를 이유(`not_configured`, `policy_not_opted_in` 또는 `no_policy_matched`)와 사용자의 ID와 함께 기록합니다
  * `admin.denied`는 거부된 관리자 API 인증 시도를 클라이언트 IP, 메서드, 경로 및 이유와 함께 기록하며, 제시된 키 자료는 없습니다: `x-api-key`가 제시되었지만 구성된 키와 일치하지 않을 때 `invalid_key`, `Authorization` 헤더만 제시되었고 게이트웨이 세션으로 `admin.admin_groups`에서 검증되지 않을 때 `bearer_rejected`, 또는 어느 헤더도 제시되지 않을 때 `no_credentials`
* **운영 로그**: 부팅, 경고 및 업스트림 오류에 대한 인간이 읽을 수 있는 `[gateway]` 접두사 줄. `CLAUDE_GATEWAY_LOG_LEVEL` 환경 변수는 상세도를 제어하고 `debug`, `info`, `warn` 또는 `error`를 허용하며, 기본값은 `info`입니다. `debug`에서 각 로그인 및 새로고침은 또한 id\_token의 클레임 이름(값이 아님)을 기록하며, `userinfo_fallback`이 제공한 경우 userinfo 클레임의 이름을 기록하므로, PII를 기록하지 않고 `email_claim` 및 `groups_claim` 설정을 진단할 수 있습니다. 감사 이벤트는 항상 내보내지므로 이는 감사 이벤트에 영향을 주지 않습니다.

<h3 id="health">
  상태
</h3>

게이트웨이는 `GET /healthz`를 생존 프로브로 제공하고 `GET /readyz`를 준비 프로브로 제공합니다. `/readyz`는 저장소에 도달 가능한지 확인합니다. 둘 다 `access_control.allow_cidrs`에서 제외되므로 프로브는 잠긴 리스너에서 작동합니다.

`/.well-known/oauth-authorization-server`의 OAuth 검색 문서는 구성 로드, OIDC 검색, 업스트림 클라이언트 구성 및 Postgres 마이그레이션이 모두 성공한 후에만 `200`을 반환하므로, 엔드 투 엔드 부팅 확인으로도 작동합니다.

<h3 id="concurrent-upstream-requests">
  동시 업스트림 요청
</h3>

기본적으로 각 게이트웨이 복제본은 동시에 최대 256개의 요청을 업스트림으로 보냅니다. 스트리밍 응답은 스트림이 끝날 때까지 제한에 대해 계산됩니다.

복제본이 제한에 있을 때 도착하는 요청은 게이트웨이 내에서 빈 슬롯을 기다립니다. 개발자는 시작이 느리거나 중단된 것처럼 보이는 응답을 봅니다. `provider: anthropic` 업스트림에서, [`timeouts.upstream_ttfb_ms`](/docs/ko/claude-apps-gateway-config#http-tuning)보다 오래 기다리는 요청은 해당 업스트림을 포기하고, 나중 업스트림이 이를 제공하지 않으면 502로 실패합니다.

`upstream requests:`를 포함하는 시작 로그 줄은 적용 중인 제한을 보여줍니다. 복제본이 제한보다 더 많은 요청을 열어 두는 동안, 또한 `client requests are open`을 포함하는 경고를 최대 분당 한 번 기록합니다.

동시에 더 많은 요청을 제공하려면 두 가지 옵션이 있습니다:

* 복제본을 추가합니다.
* 각 복제본의 제한을 높입니다. 게이트웨이 컨테이너에서 `BUN_CONFIG_MAX_HTTP_REQUESTS` 환경 변수를 1에서 65535 사이의 정수로 설정한 다음 컨테이너를 다시 시작합니다.

복제본은 요청이 열려 있는 평균 초 수로 나눈 제한 정도의 요청 속도로 제한을 채웁니다. 예를 들어, 요청이 평균 10초 동안 열려 있으면, 기본 제한 256의 복제본은 약 초당 26개 요청으로 제한을 채웁니다.

CPU에서 자동 스케일링하면, 제한의 복제본은 스케일 아웃을 트리거하지 않고 요청을 큐에 넣으므로, 복제본이 `client requests are open` 경고를 기록할 때 표시하는 CPU 수준 아래로 대상을 설정합니다.

<Warning>
  모든 열린 요청은 스트리밍 중 및 슬롯을 기다리는 동안 게이트웨이 프로세스에서 메모리를 보유합니다. 제한을 256으로 유지하면, 과부하 복제본의 메모리는 여전히 증가합니다. 대기 요청이 요청 본문을 유지하기 때문입니다. 컨테이너의 메모리를 피크 시 열린 요청 수에 맞게 크기를 조정하고, 제한을 변경할 때 메모리를 감시합니다. 메모리가 부족한 복제본은 종료되고 보유한 모든 스트림을 삭제합니다.
</Warning>

<h3 id="outage-behavior">
  중단 동작
</h3>

Postgres가 다운되면, 게이트웨이 자체는 로그인한 개발자를 계속 제공하고 새로운 로그인은 실패합니다. 개발자가 실제로 계속 작동하는지는 오케스트레이터가 준비를 처리하는 방식에 따라 다릅니다:

* **기존 세션**: 베어러 토큰은 JWT 시크릿으로 로컬에서 검증되고, 세션 새로고침은 저장소를 건드리지 않으며, 게이트웨이 프로세스는 여전히 추론을 제공할 수 있습니다
* **새로운 로그인**: Postgres가 복구될 때까지 실패합니다. 장치 흐름 및 속도 제한 카운터가 Postgres에 있기 때문입니다
* **[지출 제한 적용](/docs/ko/claude-apps-gateway-spend-limits#postgres-availability)**: 중단 중에 기본적으로 열린 상태로 실패하므로 추론이 계속 흐릅니다. 차단하는 것을 선호하면 닫힌 상태로 뒤집으세요
* **준비**: `/readyz`는 중단 중에 준비되지 않음을 보고하므로, 준비에 대한 트래픽을 게이트하는 오케스트레이터는 Postgres가 복구될 때까지 모든 복제본을 한 번에 로테이션에서 제거합니다. 해당 토폴로지에서 게이트웨이가 여전히 제공할 수 있는 추론을 포함한 모든 트래픽은 로드 밸런서에서 실패합니다. `/healthz`의 생존 프로브는 계속 통과하므로 복제본은 다시 시작되지 않습니다. 로그인한 개발자가 저장소 중단을 통해 계속 작동하도록 하려면 준비 프로브를 `/healthz`로 지정하세요. 비용은 새로운 로그인이 여전히 준비됨을 보고하는 복제본에 대해 실패한다는 것입니다.

IdP가 다운되면, 기존 세션은 `ttl_hours`까지 작동하고, 새로운 로그인은 실패하며, 세션 새로고침은 다시 시도 답변을 받고 IdP가 돌아오면 한 번 진행됩니다. IdP가 자주 유지보수 창을 가지면 더 긴 `ttl_hours`를 설정하세요.

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

게이트웨이는 부팅 시 마이그레이션으로 생성되는 5개의 데이터 테이블과 `_migrations` 테이블을 보유합니다:

| 테이블                | 내용                                            | 보존                                                |
| ------------------ | --------------------------------------------- | ------------------------------------------------- |
| `kv`               | 장치 부여(10분 TTL) 및 속도 제한 카운터                    | 행별 TTL                                            |
| `spend`            | 주요 기간별 누적 지출 카운터(센트)                          | `admin.spend_retention_months`, 기본값 13            |
| `spend_limits`     | 구성된 지출 상한                                     | API를 통해 삭제될 때까지                                   |
| `admin_audit`      | 관리자 API 변경 추적                                 | `admin.audit_retention_days`, 기본값 365             |
| `principal_emails` | 각 주요의 마지막 확인 이메일, 표시 이름 및 IdP 그룹. PII를 포함합니다. | `admin.identity_retention_days` 마지막 활동 이후, 기본값 90 |

30초 루프는 TTL을 지난 `kv` 행을 만료하고, 시간별 스윕은 지출 테이블의 보존 창을 적용하므로 아무것도 무한정 증가하지 않습니다. [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)이 구성되지 않으면 `kv`만 작성됩니다. 게이트웨이는 부팅 시 자신의 스키마 마이그레이션을 적용하고 모든 업그레이드에서 적용하므로, 데이터베이스 역할은 테이블을 생성하고 변경할 권리가 필요합니다. 게이트웨이 전용 데이터베이스 또는 스키마로 지정하여 해당 권한을 좁게 유지하세요.

지출 제한이 사용 중이면, 손실된 데이터베이스는 개발자 재로그인뿐만 아니라 손실된 지출 추적 및 상한을 의미하므로 정기적인 백업을 실행하세요. 보존을 기다리지 않고 떠난 개발자 하나를 즉시 지우려면 `DELETE FROM principal_emails WHERE principal = '<sub>'`을 직접 실행하세요. 이는 이메일, 이름 및 그룹을 보유하는 유일한 테이블을 제거합니다. `spend` 및 `admin_audit` 행은 의사명 OIDC `sub`만 참조합니다.

<h3 id="upgrades">
  업그레이드
</h3>

복제본은 상태 비저장이므로 롤링 재시작은 게이트웨이 상태를 잃지 않습니다. 게이트웨이는 부팅 시 스키마 마이그레이션을 실행하므로, 새 바이너리를 배포하면 데이터베이스가 자동으로 마이그레이션됩니다. 동시 복제본은 Postgres 자문 잠금에서 직렬화되므로 각 마이그레이션을 적용하는 것은 하나뿐입니다.

오케스트레이터가 롤링 재시작 또는 스케일 인에서처럼 `SIGTERM`으로 복제본을 중지할 때, 게이트웨이는 새 연결을 수락하는 것을 중지하고 이미 진행 중인 요청 및 스트림이 종료되기 전에 완료되도록 합니다. 드레인 윈도우라고 불리는 최대 25초를 기다린 다음 여전히 열려 있는 것을 닫습니다. `SIGINT`(예: 터미널의 Ctrl+C)는 동일한 드레인을 시작하고, 드레인 중 두 번째 신호는 열린 요청을 닫고 즉시 종료합니다. 드레인은 게이트웨이 v2.1.274 이상이 필요합니다.

긴 생성은 몇 분 동안 스트리밍할 수 있습니다. Kubernetes 및 Amazon ECS에서 이 둘을 함께 높여 해당 스트림에 더 많은 시간을 제공합니다:

* **드레인 윈도우**: 게이트웨이 컨테이너에서 `CLAUDE_GATEWAY_DRAIN_TIMEOUT_MS` 환경 변수를 `120000`과 같은 양의 정수 밀리초로 설정합니다. 게이트웨이는 `120s`와 같은 다른 형식의 값을 무시하고 25초 기본값을 유지합니다
* **오케스트레이터의 유예 기간**: Kubernetes의 `terminationGracePeriodSeconds` 또는 Amazon ECS의 `stopTimeout`

유예 기간은 두 플랫폼 모두에서 기본값 30초입니다. 드레인 윈도우보다 최소 5초 이상 길게 유지하거나, 오케스트레이터가 드레인이 완료되기 전에 게이트웨이를 종료합니다. Kubernetes에서 유예 기간이 게이트웨이가 `SIGTERM`을 받을 때가 아니라 훅이 실행되기 전에 계산을 시작하기 때문에 `preStop` 훅의 기간도 추가합니다.

플랫폼은 또한 드레인이 실행될 수 있는 기간을 제한할 수 있습니다:

* **Amazon ECS on Fargate**: `stopTimeout`은 최대 120초를 허용합니다
* **Cloud Run**: `SIGTERM` 후 10초 후에 인스턴스를 중지하므로, 열린 스트림은 드레인 윈도우가 무엇이든 최대 10초를 얻습니다

드레인 윈도우가 여전히 열린 요청으로 끝나면, 게이트웨이는 `drain window over after`를 포함하는 경고를 기록하고, 자른 요청을 계산하며, 높일 두 설정의 이름을 지정합니다.

마이그레이션은 추가 전용이므로 더 적은 마이그레이션을 아는 이전 바이너리로 롤백하는 것은 안전합니다. 추가 행을 무시합니다. 롤백은 또한 YAML을 이전 바이너리의 스키마에 대해 재검증하므로, 새 릴리스에서 도입한 키를 채택한 구성은 이전 바이너리에서 부팅이 실패합니다. 롤백 전에 새 키를 제거하세요.

게이트웨이의 버전을 자신의 이미지에 고정하므로, 새 Claude Code 릴리스의 수정사항(보안 수정사항 포함)은 핀을 업데이트하고 재배포할 때만 배포에 도달합니다. 게이트웨이를 프로덕션 자격 증명을 보유하는 다른 서비스에 사용하는 것과 동일한 패칭 주기에 포함하세요.

<h2 id="security">
  보안
</h2>

이 섹션은 보안 검토가 묻는 질문에 답합니다: 어떤 데이터가 게이트웨이를 통해 흐르고 어디로 가는지, 설계가 방어하는 공격, 규정 준수 질문지에 속하는 답변.

<h3 id="data-flow">
  데이터 흐름
</h3>

| 데이터                                                                          | 경로                                                                                                                                                                                                   | 게이트웨이에서 Anthropic으로 전송됨       |
| ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- |
| 추론(프롬프트, 완료)                                                                 | CLI → 게이트웨이 → 업스트림                                                                                                                                                                                   | Anthropic API가 구성된 업스트림인 경우에만 |
| 텔레메트리(OTLP 메트릭, 플러스 [옵트인 로그 및 추적](/docs/ko/claude-apps-gateway-config#telemetry)) | CLI → 게이트웨이 → 수집기                                                                                                                                                                                    | 절대 아님                         |
| ID(이메일, 그룹, sub)                                                             | IdP → 게이트웨이 → JWT → CLI; CLI는 OTLP 내보내기에 스탬프합니다. [`forward_user_identity`](/docs/ko/claude-apps-gateway-config#per-user-identity-headers-for-a-proxy-you-run)를 켜면, 게이트웨이는 개발자의 이메일과 IdP 주제를 헤더로 프록시에 보냅니다 | 절대 아님                         |
| 관리형 설정                                                                       | 게이트웨이 YAML → CLI                                                                                                                                                                                     | 절대 아님                         |
| 감사 로그                                                                        | 게이트웨이 stderr → 수집기                                                                                                                                                                                   | 절대 아님                         |

<h3 id="threat-model-summary">
  위협 모델 요약
</h3>

게이트웨이는 네트워크 경계 내부에 있지만, 개별 개발자 노트북은 신뢰할 수 있는 것으로 취급되지 않습니다. 설계는 3가지 방식으로 이를 설명합니다:

* 개발자는 원시 업스트림 키 대신 단기 JWT를 보유합니다. CLI-게이트웨이 레그는 RFC 8628 장치 부여를 사용하고, 게이트웨이의 IdP와의 인증 코드 교환은 기본 구성에서 PKCE를 실행하므로, 가로챈 IdP 인증 코드는 쓸모가 없습니다.
* 장치 검증 페이지는 동일 출처 POST 및 RFC 8628 §5.1당 IP당 속도 제한을 적용합니다. [사용자 코드 무차별 대입 저항](#user-code-brute-force-resistance)을 참조하세요.
* 게이트웨이의 IdP, OTLP 수집기 및 `provider: anthropic` 업스트림에 대한 요청은 DNS를 확인하고, 링크 로컬 및 클라우드 메타데이터 주소와 기본적으로 루프백을 차단하며, 연결을 확인된 IP에 고정하는 서버 측 요청 위조(SSRF) 가드를 통과합니다. 따라서 운영자 영향 URL은 클라우드 메타데이터 엔드포인트로 리디렉션될 수 없습니다. RFC 1918 프라이빗 범위는 의도적으로 허용됩니다. IdP 및 OTLP 수집기가 일반적으로 프라이빗 IP에 있기 때문입니다. 다른 공급자의 경우, 게이트웨이는 구성을 로드할 때 이러한 주소 또는 메타데이터 호스트명을 지정하는 `base_url`을 거부하고, 공급자의 SDK는 DNS 확인 없이 연결합니다.

  [프록시 전용 송신](/docs/ko/claude-apps-gateway-config#proxy-only-egress)을 켜면, 해당 주소 확인이 포워드 프록시로 이동합니다: 게이트웨이는 호스트명을 전달하고 프록시의 허용 목록은 해당 대상을 거부해야 합니다.

  게이트웨이가 정당하게 도달해야 하는 것이 루프백에 있을 때만 게이트웨이의 환경에서 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`을 설정하세요. 예를 들어 로컬 개발 IdP 또는 `localhost`의 사이드카 OTLP 수집기. 변수는 모든 운영자 구성 URL에 대해 루프백 블록을 완화하고 또한 포드가 클라우드 메타데이터 엔드포인트에 도달할 수 있는지 확인하는 부팅 시간 경고를 건너뜁니다. 따라서 수집기에 자체 내부 주소를 제공하는 것을 선호하세요.

자신의 송신 제어를 추가하면, 게이트웨이는 워크로드 ID와 같은 인스턴스 메타데이터 자격 증명을 사용할 때마다 메타데이터 서버에 도달해야 합니다.

두 가지 위협은 범위를 벗어났습니다. 이는 보안할 인프라입니다:

* **손상된 게이트웨이 호스트**: 호스트는 업스트림 자격 증명을 보유하고 [관리형 설정](/docs/ko/claude-apps-gateway-config#managed)을 모든 연결된 개발자에게 배포하므로, 게이트웨이 구성에 대한 제어는 MDM에 대한 제어와 비교할 수 있습니다. CLI의 [승인 대화](/docs/ko/server-managed-settings#approval-memory)는 셸 가능 설정의 자동 변경을 제한하지만 호스트 보안을 대체하지 않습니다.
* **악의적인 OIDC 공급자**: 공급자는 게이트웨이가 신뢰하는 id\_token에 서명하므로 모든 ID를 주장할 수 있습니다. IdP 검증 및 보안은 귀사의 책임입니다.

<h3 id="user-code-brute-force-resistance">
  사용자 코드 무차별 대입 저항
</h3>

`user_code` 개발자가 `/device` 검증 페이지에 입력하는 것은 20자 알파벳에서 그려진 8자이며, 20⁸ 또는 약 2.56×10¹⁰ 조합을 산출하고 10분 후 만료됩니다.

게이트웨이는 [`rate_limits`](/docs/ko/claude-apps-gateway-config#http-tuning)를 통해 구성 가능한 장치 부여 엔드포인트에 IP당 속도 제한을 적용합니다. 많은 개발자가 단일 공유 회사 NAT 주소에서 로그인하면 제한을 올리세요. [대규모 롤아웃](#large-rollouts)은 크기를 조정하는 방법을 보여줍니다. 제한은 로그인 흐름에만 적용되며 추론에는 적용되지 않습니다.

<h3 id="compliance-posture">
  규정 준수 태세
</h3>

* **데이터 거주지**: 게이트웨이의 자체 데이터 평면은 Anthropic API가 구성된 업스트림인 경우를 제외하고 Anthropic에 아무것도 보내지 않습니다. 그 경우 기존 데이터 처리 계약이 추론 경로에 적용됩니다. 텔레메트리, 감사, ID 및 설정은 구성한 대상으로만 이동합니다.
* **호스트 프로세스 트래픽**: 호스트 프로세스는 Claude Code CLI입니다. `claude gateway`는 Amazon Bedrock 및 Google Cloud의 Agent Platform 배포와 동일한 타사 규칙에 따라 실행되며 Anthropic에 아무것도 보내지 않습니다. v2.1.227 이전에는 호스트 프로세스가 제품 버전 및 플랫폼과 같은 시작 텔레메트리를 보냈으며, 컨테이너 환경에서 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`을 설정하면 꺼집니다. 이러한 릴리스는 또한 부팅 시 본문이나 자격 증명이 없는 하나의 `HEAD` 요청을 `https://api.anthropic.com`의 `/api/hello`로 보냈거나, 환경이 설정한 경우 `ANTHROPIC_BASE_URL`로 보냈습니다. 환경이 `HTTPS_PROXY`와 같은 프록시 변수 또는 mTLS 클라이언트 인증서도 설정하지 않은 경우입니다. 응답을 무시했으므로 송신 방화벽에서 해당 요청을 차단해도 게이트웨이에 영향을 주지 않았습니다.
* **클라이언트 분석**: CLI는 게이트웨이에 로그인하는 동안 자신의 사용 분석 및 오류 보고를 비활성화합니다. 첫 번째 로그인 전에 CLI는 여전히 시작 이벤트를 Anthropic으로 보냅니다. 관리형 설정이 게이트웨이 로그인을 강제하는 머신을 포함합니다. 이것도 끄려면 게이트웨이 로그인을 강제하는 동일한 [클라이언트 측 관리형 설정](/docs/ko/claude-apps-gateway-config#client-side-managed-settings)에서 [`DISABLE_TELEMETRY`](/docs/ko/managed-settings#turn-telemetry-off-for-your-organization)를 제공하세요.
* **오류 보고**: CLI는 모델 요청이 Anthropic의 자사 API가 아닌 Amazon Bedrock 또는 사용자 정의 `ANTHROPIC_BASE_URL`과 같은 다른 엔드포인트로 이동할 때마다 오류 보고를 끕니다.
* **클라이언트 머신**: 개발자의 CLI는 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` 및 `skipWebFetchPreflight: true`가 설정되지 않으면 여전히 WebFetch 호스트명 확인 및 버전 확인을 Anthropic으로 보냅니다. [데이터 사용](/docs/ko/data-usage)을 참조하세요.
* **설문 조사 평가**: 게이트웨이에 로그인하는 동안 CLI는 분석 스트림과 함께 Anthropic 바운드 평가 업로드를 비활성화하므로 평가를 Anthropic으로 보내지 않습니다.
* **트랜스크립트 공유**: 설문 조사의 트랜스크립트 공유 프롬프트에서 예를 선택하면 Anthropic으로 업로드하는 대신 `~/.claude/feedback-bundles/` 아래의 로컬 파일을 작성합니다.
* **클라이언트 업데이트**: 업데이트 확인은 게이트웨이 트래픽과 별개입니다. 자신의 배포를 통해 버전을 고정하고 노트북이 릴리스를 가져오면 안 되면 `DISABLE_UPDATES`를 설정하세요. `DISABLE_AUTOUPDATER`는 `claude update`가 여전히 작동하는 동안 백그라운드 업데이트만 중지합니다.
* **TLS**: 프로덕션에서 `public_url`을 HTTPS를 통해 제공하세요. 게이트웨이의 자체 리스너를 통해 `listen.tls`를 사용하거나 `listen.public_url`이 설정된 일반 HTTP 복제본 앞의 TLS 종료 ingress에서. 두 경우 모두. 게이트웨이는 일반 HTTP를 거부하지 않습니다. IdP는 프로덕션에서 HTTPS를 제공해야 하고, Postgres는 `?sslmode=require`를 지원합니다. ingress에서 `Strict-Transport-Security`를 설정하세요.
* **취약점 공개**: [보안 문제 보고](/docs/ko/security#reporting-security-issues)를 따르세요

<h2 id="troubleshooting">
  문제 해결
</h2>

질문 및 피드백은 [Claude Code 지원](https://support.claude.com/en/collections/14445694-claude-code)을 사용하거나 [Claude Code GitHub 저장소](https://github.com/anthropics/claude-code/issues)에서 이슈를 열어주세요. 문제를 보고할 때 다음을 포함하세요:

* **Gateway 문제**: 해당 창의 gateway stderr, 비밀번호가 제거된 `gateway.yaml`, `/`의 랜딩 페이지에 표시된 gateway 버전, `/managed/settings`의 `x-cc-gateway-version` 응답 헤더, 최근에 변경된 사항
* **로그인 문제**: 개발자가 `claude --debug-file ./claude-debug.txt`를 실행하고 재현한 후 해당 파일과 동일한 창의 gateway 감사 로그를 전송
* **추론 문제**: 요청된 모델, 구성된 업스트림, 요청에 대한 gateway 감사 로그(어느 업스트림이 제공했는지 및 응답 상태 기록)

gateway stderr에는 감사 이벤트 스트림이 포함되고, 감사 로그는 개발자 신원을 기록하며, 디버그 파일은 개발자 머신의 hook 및 MCP 서버 출력을 기록합니다. 공개 이슈에 게시하기 전에 이를 검토하고 수정하세요.

| 증상                                                                                                                                                                                   | 원인                                                                                                                                                                                                                                                                             | 해결 방법                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 개발자의 `/login`이 **Cloud gateway** 화면 대신 표준 계정 선택기를 표시함                                                                                                                                | 해당 머신의 관리 설정에서 `forceLoginMethod` 또는 `forceLoginGatewayUrl`이 설정되지 않음                                                                                                                                                                                                           | [관리 설정 파일](/docs/ko/claude-apps-gateway#set-the-gateway-url)을 기기에 배포하세요. `/login`은 여기서 gateway URL을 읽습니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 개발자의 요청이 `Not signed in to the Cloud gateway — run /login.`으로 실패함                                                                                                                    | 머신의 관리 설정이 `forceLoginMethod: "gateway"` 또는 `forceLoginGatewayUrl`을 설정했고, 세션에 gateway 로그인이 없음. 남은 claude.ai 로그인은 요구사항을 충족하지 않음                                                                                                                                                 | 개발자가 `/login`을 실행하고 gateway 로그인을 완료하도록 하세요. [관리자 정책이 Cloud gateway 로그인을 요구함](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)도 참조하세요.                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Claude Desktop이 부트스트랩 구성을 가져올 수 없다고 보고함                                                                                                                                              | `/user/bootstrap`이 404를 반환함: 사용자와 일치하는 정책이 `desktop` 키를 포함하지 않거나 일치하는 정책이 없음. gateway 감사 로그는 각 거부를 `desktop_bootstrap.denied`로 이유와 함께 기록함                                                                                                                                      | 사용자와 일치하는 정책 또는 `match: {}` 기본 계층에 `desktop` 블록을 추가하세요. 빈 `desktop: {}`으로 충분합니다. [Claude Desktop 오버레이](/docs/ko/claude-apps-gateway-config#claude-desktop-overlay)를 참조하세요.                                                                                                                                                                                                                                                                                                                                                                                                                        |
| 시작 시 `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.`를 표시함                                                      | 설치된 Claude Code 빌드가 gateway 지원보다 이전 버전임                                                                                                                                                                                                                                        | 개발자가 Cloud gateway 지원을 포함하는 릴리스로 Claude Code를 업데이트하도록 하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| 시작이 `Administrator policy requires a Cloud gateway sign-in on this machine`으로 종료됨                                                                                                    | 개발자의 환경이 `ANTHROPIC_API_KEY` 또는 `ANTHROPIC_AUTH_TOKEN`을 설정하거나, 설정이 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper)를 구성하거나, 이전 Claude Console 로그인의 API 키가 여전히 저장되어 있음                                                                                                     | 해당하는 각각을 지우도록 개발자에게 지시하세요: 변수를 설정 해제하거나, `apiKeyHelper` 항목을 제거하거나, `claude auth logout`을 실행하여 저장된 키를 제거합니다. 그런 다음 `claude`를 시작하고 `/login`으로 로그인하도록 하세요. [관리자 정책이 Cloud gateway 로그인을 요구함](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)도 참조하세요.                                                                                                                                                                                                                                                                                                                           |
| 시작 또는 `/login`이 `/managed/settings` 로드에서 403 후 `Claude Code may not be enabled for your organization`을 보고함                                                                           | gateway 또는 그 앞의 무언가가 `/managed/settings` 요청에 403으로 응답함. gateway 자체 설정 경로는 절대 403으로 응답하지 않음. 상태는 [`access_control`](/docs/ko/claude-apps-gateway-config#http-tuning) IP 확인 또는 gateway 앞의 프록시 또는 WAF에서 옴. 감사 로그는 IP 확인 거부를 `access.denied`로 이유와 함께 기록함. 개발자는 로그인 상태를 유지함              | 실패 시간에 `access.denied`에 대한 감사 로그를 확인하고 `access_control` 목록 또는 프론트 엔드를 수정한 후 개발자가 `claude`를 다시 시작하도록 하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| CLI `/login`: `The gateway is limiting sign-in attempts right now`, 또는 이전 버전에서 `Request failed with status code 429`. `/device` 페이지는 이전에 시도하지 않은 개발자에게 `Too many attempts`를 표시할 수 있음 | IP당 로그인 속도 제한에 도달함. `listen.trusted_proxies`가 로드 밸런서를 포함하지 않아 모든 개발자가 해당 주소를 공유하거나, 많은 개발자가 NAT 또는 VPN 송신 주소를 공유함. `result: rate_limited`가 있는 감사 이벤트는 동일한 하나 또는 몇 개의 `client_ip` 값을 표시함.                                                                                       | 먼저 `listen.trusted_proxies`를 로드 밸런서의 소스 범위로 설정한 후, 개발자가 여전히 주소를 공유하는 경우 `rate_limits`를 높이세요. [대규모 롤아웃](#large-rollouts)을 참조하세요.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>`                                           | gateway 호스트명이 최소 하나의 공개 IP 주소로 확인됨. Claude Code는 각 확인된 주소를 확인하고 모든 주소가 비공개여야 함. 일반적인 원인은 한 패밀리가 공개 주소로 확인되는 이중 스택 이름이며, AWS 내부 이중 스택 로드 밸런서를 포함하여 공개 범위 AAAA 주소를 반환함                                                                                                           | gateway 이름이 개발자 머신에서 비공개 주소로만 확인되도록 하세요. 이중 스택 이름의 경우 공개 범위 레코드를 삭제하거나 별도의 내부 전용 DNS 이름을 제공하세요. [비공개 네트워크 전제 조건](/docs/ko/claude-apps-gateway#prerequisites)을 참조하세요. 주소가 조직이 소유하고 내부적으로 사용하는 공개 공간인 경우 [해당 블록을 선언](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)하세요.                                                                                                                                                                                                                                                                                                   |
| CLI `/login`: `Gateway login would go through proxy <proxy>, which is not on a private network`                                                                                      | `HTTPS_PROXY` 또는 `HTTP_PROXY`가 gateway 호스트에 적용되고 프록시의 호스트명이 공개 주소로 확인됨. 호스트가 비공개 주소로만 확인되는 프록시는 허용되며 이 오류를 트리거하지 않음                                                                                                                                                            | 개발자 머신의 `NO_PROXY`에 gateway 호스트를 추가하여 연결이 직접 이루어지도록 하거나, 호스트명이 비공개 주소로 확인되는 프록시를 사용하세요. 메시지는 추가할 정확한 `NO_PROXY` 항목을 이름으로 지정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| CLI `/login`: `Claude Code only signs in to <host> from inside its declared network <block> (managed settings), and this machine is connecting from <ip>, outside it`                | gateway가 [`gatewayInternalNetworks`](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)에 선언된 블록에 있고, 개발자 머신이 해당 블록 외부의 주소에서 도달함: VPN 주소 풀, 컨테이너 또는 WSL2 NAT 세그먼트, 또는 조직의 네트워크가 아님                                                                        | 개발자가 네트워크의 호스트 OS에서 `/login`을 실행하도록 하세요. 표시된 주소도 조직의 공개 공간인 경우 gateway 항목을 두 주소를 모두 포함하는 블록으로 바꾸세요(`/8`까지). 두 번째 겹치는 항목은 거부됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| CLI `/login`: `Every address for gateway host <host> must be inside its declared network <block>, and it also resolves to <ip>`                                                      | gateway 이름이 [`gatewayInternalNetworks`](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)에 선언된 블록 외부의 주소로 확인됨: 두 번째 사이트 또는 이중 스택 이름의 IPv6 레코드. 선언된 블록 아래에서 모든 레코드는 해당 IPv4 블록 내부에 있어야 하며, 비공개 및 IPv6 주소 포함                                              | 개발자 머신의 gateway 이름에 대해 블록 내부의 레코드만 게시하거나 별도의 내부 전용 이름을 제공하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| CLI `/login`: `<host> is on the declared network <block>, which Claude Code checks over a direct connection, not through an HTTP proxy`                                              | `HTTPS_PROXY` 또는 `HTTP_PROXY`가 선언된 블록의 gateway에 적용됨                                                                                                                                                                                                                            | 개발자 머신에서 메시지가 이름으로 지정하는 `NO_PROXY` 항목을 추가하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| CLI `/login`: `gatewayInternalNetworks in managed settings`로 시작하는 메시지                                                                                                                | 값이 [검증 규칙](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) 중 하나를 위반하고 메시지가 어느 것인지 이름으로 지정함. 수정할 때까지 Claude Code는 비공개 주소의 gateway를 포함하여 머신의 모든 새로운 gateway `/login`을 거부함. 기존 로그인은 계속 작동함                                                               | 배포하는 관리 설정 소스에서 메시지가 이름으로 지정하는 항목을 수정한 후 `/login`을 다시 실행하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| CLI `/login`: `Could not resolve the configured HTTP proxy`                                                                                                                          | `HTTPS_PROXY` 또는 `HTTP_PROXY`의 호스트명이 개발자 머신에서 확인되지 않음. 일반적으로 회사 네트워크에 연결되지 않았기 때문                                                                                                                                                                                              | 개발자가 네트워크 또는 VPN에 연결하고 다시 시도하거나 프록시 URL을 수정하도록 하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| CLI `/login`: `Could not resolve gateway host <host>`                                                                                                                                | 머신이 gateway의 내부 DNS 이름을 확인할 수 없음. 일반적으로 회사 네트워크에 없기 때문                                                                                                                                                                                                                         | 개발자가 네트워크 또는 VPN에 연결한 후 `/login`을 다시 시도하도록 하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| 부트가 `store.postgres_url`을 이름으로 지정하는 구성 검증 오류로 종료됨                                                                                                                                    | Postgres가 구성되지 않음. gateway는 Postgres를 요구함                                                                                                                                                                                                                                      | `store.postgres_url`을 설정하세요. 로컬 개발의 경우 일회용 컨테이너를 사용하세요: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 부트 종료: `requires the native binary`                                                                                                                                                  | Node 대신 네이티브 바이너리에서 실행 중                                                                                                                                                                                                                                                       | [독립 실행형 설치 방법](/docs/ko/setup) 중 하나로 Claude Code를 설치하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| 부트가 `config.load` 후 OIDC 검색 오류로 종료됨                                                                                                                                                  | `oidc.issuer`에 도달할 수 없거나 TLS 체인을 신뢰하지 않음                                                                                                                                                                                                                                       | 발급자가 포드에서 도달 가능하고 `/.well-known/openid-configuration`을 제공하는지 확인하세요. 비공개 PKI의 경우 `ca_cert_pem`을 설정하세요. 포드가 정방향 프록시를 통해서만 IdP에 도달하는 경우 [`oidc.use_proxy: true`](/docs/ko/claude-apps-gateway-config#idp-requests-through-a-forward-proxy)를 설정하세요. v2.1.227 이전 버전에서는 대신 포드에 IdP의 각 엔드포인트에 대한 직접 경로를 제공하세요. 포드가 IdP의 호스트명을 확인할 수 없거나 프록시가 IP 주소에 대한 `CONNECT`를 거부하는 경우 [프록시 전용 송신](/docs/ko/claude-apps-gateway-config#proxy-only-egress)을 참조하세요. v2.1.277 이상이 필요합니다.                                                                                                                                      |
| 부트가 Postgres 권한 오류로 종료됨                                                                                                                                                              | 데이터베이스 역할이 스키마에 대한 DDL 권한이 없음                                                                                                                                                                                                                                                  | 부트 시 테이블을 생성하고 변경할 수 있도록 gateway 스키마에 대해 역할에 `CREATE` 권한을 부여하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| 로그: `could not connect to Postgres at boot, attempt 1 of 3`                                                                                                                          | gateway가 시작될 때 데이터베이스에 도달할 수 없었음. 예를 들어 네트워크가 아직 시작 중인 콜드 인스턴스                                                                                                                                                                                                                 | gateway가 부팅을 완료하면 조치가 필요하지 않습니다. 데이터베이스에 도달할 수 없을 때 gateway는 종료되기 전에 2초 간격으로 연결을 3번 시도합니다. `could not connect to Postgres`로 종료되면 `store.postgres_url`과 데이터베이스로의 네트워크 경로를 확인하세요. 시도가 거부되지 않고 시간 초과되면 각 시도에 더 많은 시간을 주기 위해 [`store.connect_timeout_seconds`](/docs/ko/claude-apps-gateway-config#store)를 높이세요.                                                                                                                                                                                                                                                                                      |
| `/oauth/callback`이 "Sign-in could not be completed"를 표시함                                                                                                                             | 이메일 도메인이 거부됨, id\_token 검증 실패, 또는 `email_verified`가 명시적으로 `false`이며 gateway는 항상 재정의 없이 거부함                                                                                                                                                                                     | `allowed_email_domains`을 확인하고 IdP가 확인된 `email` 클레임을 반환하는지 확인하세요. `email_verified: false`의 경우 IdP 측 검증을 수정하세요. IdP가 다른 클레임 이름 아래에서 이메일을 내보내는 경우 `oidc.email_claim`을 설정하세요.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| 로그: `token exchange failed request_id=<id>: id_token missing email claim`                                                                                                            | IdP가 기본적으로 id\_token에 `email`을 포함하지 않음. 이 거부는 `allowed_email_domains`이 설정된 경우에만 발생함. 없으면 누락된 이메일이 이메일 없이 세션을 발행함                                                                                                                                                               | IdP를 구성하여 id\_token에 `email`을 내보내도록 하세요. Okta: 사용자 정의 권한 부여 서버의 ID 토큰 클레임에 `email`을 추가하세요. Entra: 앱 등록에서 `email`을 선택적 클레임으로 추가하세요. PingFederate: `email`을 내보내는 OpenID Connect 정책을 활성화하세요. IdP가 userinfo 엔드포인트에서 `email`을 제공하지만 id\_token에 포함하지 않는 경우(예: Okta org 권한 부여 서버) `oidc.userinfo_fallback: true`를 설정하세요.                                                                                                                                                                                                                                                                            |
| 로그: `refresh failed request_id=<id>: invalid_token (…) (at userinfo_no_id_token, …)`, 개발자가 매 `session.ttl_hours`마다 `Cloud gateway session expired`를 봄                                | IdP가 새로 고침 토큰을 수락했지만 함께 id\_token을 반환하지 않았으므로 gateway가 IdP의 userinfo 엔드포인트에 사용자의 클레임을 요청했습니다. IdP가 새로 고쳐진 액세스 토큰을 거기서 거부했습니다. gateway가 `temporarily_unavailable`으로 응답하므로 Claude Code는 새로 고침 토큰을 유지하지만 세션을 갱신할 수 없습니다. v2.1.260 이전의 gateway 버전은 `(at …)` 세부 정보 없이 동일한 줄을 기록합니다. | [`oidc.scope_on_refresh: true`](/docs/ko/claude-apps-gateway-config#oidc)를 설정하세요. gateway v2.1.260 이상에서 사용 가능하므로 새로 고침 요청이 `openid`를 다시 요청합니다. Okta와 같은 일부 IdP는 요청할 때만 새로 고침 시 id\_token을 반환합니다. PingFederate에서는 대신 **Applications > OAuth > OpenID Connect Policy Management** 아래에서 **Return ID Token On Refresh Grant**를 활성화하세요. 키는 PingFederate의 동작을 변경하지 않습니다. 여전히 생략하는 다른 IdP의 경우 userinfo 엔드포인트가 새로 고침으로 발급된 액세스 토큰을 수락하는지 확인하세요. 임시 방편으로 [`session.ttl_hours`](/docs/ko/claude-apps-gateway-config#session)를 높이세요. 프로비저닝 해제 트레이드오프는 [Identity provider setup](#identity-provider-setup)을 참조하세요. |
| 모든 Amazon Bedrock 요청이 502를 반환함. 로그가 `Could not load credentials from any providers`를 표시함                                                                                             | EC2에서 IMDSv2의 기본 홉 제한 1이 컨테이너 내부의 인스턴스 메타데이터 요청을 차단함. 부트 및 `/readyz`는 AWS SDK가 클라이언트 구성이 아닌 첫 번째 요청에서 인스턴스 자격 증명을 확인하므로 어쨌든 통과함                                                                                                                                                | `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`로 홉 제한을 높이거나 시작 템플릿에서 설정하세요. 변경 사항은 인스턴스의 모든 컨테이너에 적용됩니다. 가능한 경우 ECS 작업 역할을 선호하세요. 이는 ECS 컨테이너 자격 증명 엔드포인트에서 자격 증명을 읽고 변경을 완전히 피하거나, 노출을 제한하기 위해 전용 gateway 인스턴스에 변경을 적용하세요.                                                                                                                                                                                                                                                                                                                    |
| 피크 로드에서 응답이 시작되기 느리거나 중단된 것처럼 보이거나, 업스트림이 정상인데도 502 `all upstreams failed`로 실패함                                                                                                      | 복제본이 업스트림으로 한 번에 보내는 것보다 더 많은 요청을 열어 두었으므로 추가 요청은 gateway 내부에서 대기함. `provider: anthropic` 업스트림에서 `timeouts.upstream_ttfb_ms`보다 오래 대기하는 요청은 해당 업스트림을 포기하며, 이후 업스트림이 제공하지 않으면 502를 생성함. 로그는 `client requests are open`을 포함하는 경고를 표시함.                                            | 복제본을 추가하거나 각 복제본의 제한을 높이세요. [동시 업스트림 요청](#concurrent-upstream-requests)을 참조하세요.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| IdP 오류: unknown or unsupported scope                                                                                                                                                 | IdP가 인식하지 못하는 범위를 거부함                                                                                                                                                                                                                                                          | `oidc.scopes`를 정확히 IdP가 수락하는 목록으로 설정하세요. `openid`를 포함해야 합니다. 기본값은 `openid profile email offline_access`입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `oidc.scopes` 설정 후 세션이 자동으로 갱신되지 않음                                                                                                                                                  | `offline_access`가 재정의에서 삭제됨                                                                                                                                                                                                                                                    | IdP가 지원하는 경우 `offline_access`를 다시 추가하세요. 새로 고침 토큰 없이 개발자는 매 `session.ttl_hours`마다 브라우저 로그인을 다시 실행합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| 브라우저가 "This request came from another site and was blocked"를 표시함                                                                                                                     | 교차 사이트 양식 POST, CSRF 보호로 차단됨. 포함되거나 프록시된 페이지의 경우 예상됨                                                                                                                                                                                                                           | 검증 링크를 직접 열기                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Chrome이 "Refused to send form data … violates … Content Security Policy directive: form-action"으로 승인 버튼을 차단하지만 동일한 페이지가 Safari 또는 Firefox에서 작동함                                      | Chrome은 전체 리디렉션 체인에 대해 `form-action`을 적용합니다. IdP가 허용 목록에 없는 두 번째 호스트로 계속 리디렉션합니다.                                                                                                                                                                                              | 리디렉션 체인의 각 추가 원본을 `oidc.form_action_origins`에 추가하세요. 승인 페이지에서 Chrome DevTools → Console을 열어 어느 원본이 차단되었는지 확인하세요.                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| 로그인이 IdP에서 완료되지만 콜백이 실패하고 Chrome에서 CSP 오류 또는 Safari에서 "this sign-in link has expired"를 표시함                                                                                           | IdP가 `response_mode=form_post`를 통해 코드를 반환했으며, 이는 POST를 통해 `/oauth/callback`으로 교차 원본 자동 제출합니다. Chrome은 엄격한 CSP에서 이를 차단합니다. Safari는 제출을 허용하지만 콜백은 쿼리 문자열만 읽습니다.                                                                                                                  | IdP가 `response_mode=query`를 준수하는지 확인하세요. gateway가 명시적으로 요청하므로 콜백은 일반 리디렉션입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| 로그인이 로컬에서 작동하지만 ALB 뒤에서 실패함                                                                                                                                                          | `public_url`이 여전히 로컬 또는 내부 `http://` 원본을 이름으로 지정하므로 IdP가 잘못된 `redirect_uri`를 받음                                                                                                                                                                                                | `listen.public_url`을 외부 `https://` 원본으로 설정하고 `<public_url>/oauth/callback`을 IdP에 등록하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| 개발자가 신뢰 프롬프트를 반복적으로 봄                                                                                                                                                                | TLS 인증서가 복제본당 또는 요청당 회전함                                                                                                                                                                                                                                                       | 수신 시 안정적인 인증서를 사용하거나 TLS를 한 번 종료하고 내부적으로 일반 HTTP를 통해 복제본을 실행하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| CLI `/login`: "Could not verify the gateway's TLS certificate" 또는 `SELF_SIGNED_CERT_IN_CHAIN`                                                                                        | gateway의 TLS 체인이 CLI 호스트의 신뢰 저장소에 없는 비공개 CA로 서명됨                                                                                                                                                                                                                               | Claude Code는 기본적으로 네이티브 바이너리 및 Node 22.15 이상에서 OS 신뢰 저장소를 읽습니다. [`CLAUDE_CODE_CERT_STORE`](/docs/ko/network-config#ca-certificate-store)는 이 동작을 제어합니다. CA가 OS 신뢰 저장소에 설치된 경우 개발자가 현재 런타임에 있는지 확인하세요. 그렇지 않으면 시작하기 전에 `NODE_EXTRA_CA_CERTS`를 CA 인증서 PEM으로 설정하세요. 첫 연결 지문 프롬프트는 여전히 적용됩니다.                                                                                                                                                                                                                                                                                                          |
| CLI `/login`이 브라우저 로그인을 완료한 후 세션이 `Cloud gateway sign-in was not completed`로 끝나고 TLS 인증서 불일치                                                                                         | 로그인 후 첫 번째 요청에서 gateway가 Claude Code가 고정한 지문과 일치하지 않는 인증서를 제시했으므로 Claude Code는 gateway 자격 증명을 유지하지 않았습니다. 일반적인 원인은 한 주소 뒤의 복제본이 다른 인증서를 제공하거나 네트워크 경로의 무언가가 TLS를 가로챕니다.                                                                                                        | 호스트명에 대해 하나의 인증서를 제공하세요(예: 수신 시 TLS를 한 번 종료). 그런 다음 개발자가 `/login`을 다시 실행하도록 하세요. 해당 인증서가 고정된 인증서와 다르면 Claude Code는 인증서가 변경되었다는 경고와 함께 [신뢰 프롬프트](/docs/ko/claude-apps-gateway#connect-developers)를 다시 표시합니다.                                                                                                                                                                                                                                                                                                                                                                                       |
| CLI `/login`이 `The gateway's TLS certificate changed during sign-in: it no longer matches the one you trusted`로 중지됨                                                                  | 로그인 요청이 개발자가 `/login`을 시작할 때 수락한 인증서와 일치하지 않는 인증서를 제공하는 서버에 도달했습니다: 한 주소 뒤의 복제본이 다른 인증서를 제공하거나, 경로의 TLS 가로채기, 또는 로그인 진행 중 인증서 회전.                                                                                                                                              | 호스트명에 대해 하나의 인증서를 제공한 후 개발자가 로그인을 다시 시작하고 [신뢰 프롬프트](/docs/ko/claude-apps-gateway#connect-developers)에서 새 인증서를 검토하도록 하세요.                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

`Cloud gateway sign-in was not completed` 메시지는 gateway 호스트명을 이름으로 지정합니다. Claude Code가 고정된 지문과 제시된 지문을 모두 가지고 있으면 메시지는 각각의 처음 16자를 표시합니다.

Claude Code가 gateway 로그인 후 `couldn't load your organization's managed settings`를 보고하면 Claude Code는 이유를 이름으로 지정하고, 제자리에서 다시 시작하며, 대화를 재개합니다. Claude Code가 다시 시작할 수 없는 경우(예: 백그라운드 세션) Claude Code는 세션을 종료하고 로그인을 유지합니다.

<h2 id="related">
  관련
</h2>

* [Claude 앱 게이트웨이 개요](/docs/ko/claude-apps-gateway): 빠른 시작 및 개발자 연결
* [구성 참조](/docs/ko/claude-apps-gateway-config): 모든 `gateway.yaml` 옵션
