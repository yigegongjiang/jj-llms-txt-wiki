> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 앱 게이트웨이 구성

> 모든 gateway.yaml 옵션에 대한 참조: 리스너 및 TLS, OIDC, 세션, Postgres 저장소, Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry 업스트림, 모델 라우팅, 관리형 정책 및 텔레메트리.

Claude 앱 게이트웨이 배포는 하나의 YAML 파일(관례상 `gateway.yaml`)로 구성됩니다. 이 파일은 게이트웨이가 수행하는 모든 작업을 정의합니다: 어디서 수신 대기하는지, 개발자가 어떻게 로그인하는지, 추론이 어디로 가는지, 어떤 정책과 텔레메트리가 적용되는지입니다. 이 페이지는 해당 파일의 모든 옵션에 대한 참조입니다.

첫 번째 파일을 작성하려면 [빠른 시작](/docs/ko/claude-apps-gateway#quickstart)에서 시작하세요. 이 페이지는 최소한의 작동 구성을 구축하고 실행합니다. 만족스러운 구성이 있으면 [배포 가이드](/docs/ko/claude-apps-gateway-deploy)에서 Kubernetes, Cloud Run 또는 자신의 플랫폼에서 컨테이너화 및 호스팅하는 방법을 다룹니다.

게이트웨이는 `claude gateway --config /path/to/gateway.yaml`을 사용하여 시작 시 파일을 한 번 읽습니다. 모든 옵션은 부팅 시 스키마에 대해 검증되므로 잘못된 구성은 첫 사용 시가 아니라 필드 수준 오류로 시작 시 실패합니다.

이 페이지 끝의 [완전한 예제](#complete-example)는 모든 섹션을 다룹니다.

<h2 id="file-structure">
  파일 구조
</h2>

5개 섹션이 [필수](#required-sections)입니다. 다른 모든 섹션은 [선택 사항](#optional-sections)이며, 생략된 섹션은 기본값을 사용합니다. 알 수 없는 키는 부팅을 실패하므로 오타는 자동으로 무시되는 설정이 아니라 명명된 오류로 표시됩니다.

**필수 섹션:**

* [`listen`](#listen): 바인드 주소, 공개 URL, TLS 종료
* [`oidc`](#oidc): ID 공급자(IdP), 발급자, 클라이언트, 클레임 매핑 및 로그인 가능 사용자 포함
* [`session`](#session): 게이트웨이가 발급하는 베어러 토큰, 비밀 및 수명 포함
* [`store`](#store): 장치 권한 부여 및 속도 제한 카운터용 PostgreSQL
* [`upstreams`](#upstreams): 추론이 가는 위치, Anthropic, Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform 또는 Microsoft Foundry 여부

**선택 사항 섹션:**

* [`admin`](#admin): 관리자 API 인증 및 지출 한도 보유
* [`enforcement`](#enforcement): 지출 한도 실패 개방 또는 실패 폐쇄 동작
* [`models`](#models) 및 `auto_include_builtin_models`: 관리자 선별 모델 목록 및 업스트림별 ID
* [`managed`](#managed): IdP 그룹별 관리형 설정 정책
* [`telemetry`](#telemetry): 관찰성 스택으로의 OTLP 전달
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP 허용/거부, 요청 크기 제한, 업스트림 첫 바이트까지의 시간 및 IP당 로그인 한도

<h2 id="secret-expansion">
  비밀 확장
</h2>

`client_secret`, `jwt_secret` 또는 `postgres_url`과 같은 비밀을 `gateway.yaml`에 직접 작성하지 마세요. 아래 형식 중 하나로 참조하면 게이트웨이가 부팅 시 환경 변수 또는 파일에서 값을 확인합니다:

| 형식              | 확인 대상                        | 사용 대상                                       |
| --------------- | ---------------------------- | ------------------------------------------- |
| `${VAR}`        | 환경 변수 `VAR`. 정의되지 않으면 부팅 실패. | 컨테이너 환경 변수, 환경 주입을 통한 AWS Secrets Manager   |
| `${file:/path}` | 파일 내용, 정리됨                   | Kubernetes Secret 볼륨 마운트, Vault Agent, SOPS |

<h2 id="required-sections">
  필수 섹션
</h2>

<h3 id="listen">
  `listen`
</h3>

`listen` 블록은 게이트웨이가 제공하는 위치를 제어합니다: 바인드 주소 및 포트, 외부에서 보이는 원본 및 선택적 TLS 종료.

| 필드                     | 필수    | 설명                                                                                                                                                                                                                                                                                                                                          |
| ---------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | 아니요   | 바인드 주소. 기본값 `0.0.0.0`.                                                                                                                                                                                                                                                                                                                      |
| `port`                 | 아니요   | 바인드 포트. 기본값 `8080`.                                                                                                                                                                                                                                                                                                                         |
| `public_url`           | 프록시 뒤 | 외부에서 보이는 `https://` 원본, IdP `redirect_uri` 및 검색 메타데이터를 구축하는 데 사용됩니다. ALB, Ingress 또는 Cloud Run과 같은 TLS 종료 프록시 뒤에 필수입니다. 게이트웨이는 자신의 원본을 구성할 때 `X-Forwarded-*` 헤더를 신뢰하지 않기 때문입니다. 이들은 클라이언트가 스푸핑할 수 있습니다. 아래의 `trusted_proxies`는 클라이언트 IP 확인만 제어합니다. 또한 [텔레메트리](#telemetry)를 활성화하려면 필수입니다. 게이트웨이가 클라이언트에 푸시하는 OTLP 엔드포인트를 이 URL에서 구축하기 때문입니다. |
| `tls.cert` / `tls.key` | 아니요   | 게이트웨이가 자체적으로 TLS를 종료하는 경우 PEM 경로                                                                                                                                                                                                                                                                                                            |
| `trusted_proxies`      | 아니요   | 게이트웨이 앞의 로드 밸런서의 CIDR 또는 IP. 설정하면 게이트웨이는 이 피어에서만 `X-Forwarded-For`를 신뢰하고 IP당 속도 제한 및 감사를 위해 실제 클라이언트 IP를 기록합니다. nginx `set_real_ip_from`과 동등합니다.                                                                                                                                                                                            |

<h3 id="oidc">
  `oidc`
</h3>

`oidc` 블록은 게이트웨이를 ID 공급자에 연결하고 로그인할 수 있는 사용자를 결정합니다. 발급자 및 OAuth 클라이언트의 이름을 지정하고, 이메일 및 그룹을 전달하는 클레임을 매핑하며, 이메일 도메인 또는 그룹별로 로그인을 제한합니다.

OpenID Connect(OIDC)는 게이트웨이가 ID 공급자와 함께 사용하는 SSO 프로토콜입니다. IdP 측에서 등록할 내용은 [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하세요.

| 필드                              | 필수  | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------------- | --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | 예   | OIDC 검색 기본. `/.well-known/openid-configuration`에서 검색을 제공해야 합니다. 프로덕션에서는 HTTPS를 사용하세요. 게이트웨이는 `http://` 발급자를 허용합니다. `http://localhost:8081`과 같은 루프백 발급자는 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`이 게이트웨이의 환경에 설정되지 않으면 [SSRF 가드](/docs/ko/claude-apps-gateway-deploy#threat-model-summary)에 의해 거부됩니다.                                                                                                                                              |
| `client_id` / `client_secret`   | 예   | OAuth 클라이언트 등록에서                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `allowed_email_domains`         | 아니요 | `email` 클레임이 이 도메인 중 하나에 없는 id\_token을 거부합니다(대소문자 구분 안 함). 다중 테넌트 IdP 오구성에 대한 심층 방어. 이 설정과 무관하게 `email_verified` 클레임이 명시적으로 `false`인 id\_token은 항상 거부됩니다.                                                                                                                                                                                                                                                                             |
| `allowed_groups`                | 아니요 | 로그인을 이 IdP 그룹의 구성원으로 제한하며, `groups_claim`과 비교합니다. 허용된 이메일 도메인에 있지만 이 그룹 중 어느 것도 아닌 사용자는 거부됩니다. IdP가 그룹 클레임을 내보내야 합니다.                                                                                                                                                                                                                                                                                                                 |
| `groups_claim`                  | 아니요 | 어느 id\_token 클레임이 그룹 멤버십을 전달하는지. 기본값 `groups`. Microsoft Entra는 `roles` 아래에 앱 역할을 내보냅니다. 평면 키 또는 `/resource_access/gateway/roles`와 같은 중첩된 클레임에 대한 RFC 6901 JSON 포인터를 허용합니다.                                                                                                                                                                                                                                                           |
| `google_groups`                 | 아니요 | Google Workspace Admin SDK Directory API를 통해 로그인한 사용자의 그룹을 조회합니다. Google의 id\_token은 그룹 클레임을 전달하지 않기 때문입니다. `service_account_json_path`를 `https://www.googleapis.com/auth/admin.directory.group.readonly` 범위에 대한 도메인 전체 위임이 있는 서비스 계정 키 파일로 설정하고, `admin_email`을 서비스 계정이 가장하는 Workspace 관리자로 설정합니다. Directory API는 실제 관리자 주체가 필요합니다. 각 사용자의 그룹 이메일 주소는 그룹 클레임이 되므로 `allowed_groups` 및 `managed.policies.match.groups`는 그룹 이메일과 일치합니다. |
| `email_claim`                   | 아니요 | 어느 id\_token 클레임이 사용자의 이메일을 전달하는지. 기본값 `email`. ADFS 및 Entra B2C와 같은 일부 IdP는 대신 `upn` 또는 `preferred_username`을 내보냅니다. 평면 키, JSON 포인터 또는 첫 번째 존재하는 키가 사용되는 폴백 키 목록을 허용합니다.                                                                                                                                                                                                                                                             |
| `scopes`                        | 아니요 | 게이트웨이가 요청하는 OIDC 범위의 전체 재정의. 기본값 `[openid, profile, email, offline_access]`. IdP가 인식하지 못하는 범위를 거부하거나 그룹 또는 이메일을 내보내기 위해 사용자 정의 범위가 필요한 경우 설정합니다. `openid`를 포함해야 합니다. `offline_access`를 삭제하면 새로 고침 토큰이 비활성화되므로 개발자는 `session.ttl_hours`마다 브라우저 로그인을 다시 실행합니다. Google의 새로 고침 토큰 흐름과 같은 IdP별 범위 레시피는 [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하세요.                                                         |
| `extra_auth_params`             | 아니요 | IdP 인증 요청에 그대로 추가되는 추가 쿼리 매개변수. 이것은 Google 새로 고침 토큰의 `access_type: offline`, 일부 Entra 테넌트의 `domain_hint` 또는 단계별 흐름의 `acr_values`와 같은 IdP 특정 동작에 대한 재정의 메커니즘입니다. 게이트웨이 관리 프로토콜 매개변수는 재정의할 수 없습니다: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` 및 `client_id`.                                                                                                                                     |
| `userinfo_fallback`             | 아니요 | id\_token이 이메일 또는 그룹을 생략할 때 `/userinfo`에서 가져옵니다. Keycloak 경량 액세스 토큰, Okta org 서버 및 ADFS 최소 토큰에 필요합니다. id\_token은 권한이 있는 상태로 유지됩니다. userinfo는 간격만 채웁니다. 기본값 `false`.                                                                                                                                                                                                                                                                   |
| `use_pkce`                      | 아니요 | 인증 요청에 PKCE(S256) 챌린지를 보냅니다. 기본값 `true`. IdP가 이 기밀 클라이언트에 대해 PKCE를 거부하는 경우에만 `false`로 설정합니다.                                                                                                                                                                                                                                                                                                                                          |
| `clock_skew_seconds`            | 아니요 | id\_token 시간 클레임을 검증할 때 클록 드리프트를 허용합니다. 기본값 `0`(엄격함). 로그인 직후 "토큰 만료됨 / 아직 유효하지 않음" 오류가 표시되면 호스트/IdP 클록 스큐로 인해 올립니다.                                                                                                                                                                                                                                                                                                                   |
| `token_endpoint_auth_method`    | 아니요 | 토큰 엔드포인트 인증 방법을 재정의합니다. `client_secret_basic` 또는 `client_secret_post`를 허용합니다. 기본적으로 자동 협상됩니다.                                                                                                                                                                                                                                                                                                                                         |
| `id_token_signed_response_alg`  | 아니요 | 예상 id\_token 서명 알고리즘. 기본값 `RS256`. IdP가 ES256, PS256 또는 EdDSA로 서명하는 경우 설정합니다.                                                                                                                                                                                                                                                                                                                                                         |
| `additional_authorized_parties` | 아니요 | `client_id` 외에 허용할 추가 `azp` 값, Keycloak 브로커 및 토큰 교환 흐름의 경우                                                                                                                                                                                                                                                                                                                                                                            |
| `discovery_url`                 | 아니요 | 발급자에서 파생하는 대신 이 URL에서 검색 문서를 가져옵니다. IdP가 발급자 호스트를 다시 쓰는 프록시 뒤에 있는 경우. 경로는 `/.well-known/`을 포함해야 합니다.                                                                                                                                                                                                                                                                                                                                  |
| `form_action_origins`           | 아니요 | `/device` 페이지의 `Content-Security-Policy: form-action` 지시문에 대한 추가 원본. 게이트웨이는 이미 `'self'` 및 검색된 `authorization_endpoint` 원본을 허용하지만 Chrome은 전체 리디렉션 체인에 대해 `form-action`을 적용합니다. IdP가 Azure AD가 ADFS로 페더레이션되거나 허브 스포크 Okta 또는 회사 SSO 인터셉터와 같은 두 번째 호스트를 통해 리디렉션하는 경우 인증 요청이 리디렉션될 수 있는 모든 원본을 나열합니다.                                                                                                                                     |
| `ca_cert_pem`                   | 아니요 | IdP 요청에만 시스템 신뢰 저장소를 대체하는 PEM CA 인증서. 회사 PKI 뒤의 Keycloak 또는 Dex에 사용합니다.                                                                                                                                                                                                                                                                                                                                                               |

<h3 id="session">
  `session`
</h3>

`session` 블록은 로그인 후 게이트웨이가 발급하는 베어러 토큰의 형태를 결정합니다: 서명하는 비밀과 수명.

| 필드           | 필수  | 설명                                                                                                                                                                                                                                                           |
| ------------ | --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `jwt_secret` | 예   | 최소 32바이트의 엔트로피(예: `openssl rand -base64 32`에서). 게이트웨이의 HS256 베어러 토큰에 서명합니다. 단일 문자열 또는 회전용 배열을 허용합니다: 인덱스 0이 서명하고 모든 항목이 검증합니다. 회전하려면 새 비밀을 앞에 추가하고 `ttl_hours`를 기다린 후 이전 비밀을 삭제합니다.                                                                          |
| `ttl_hours`  | 아니요 | 게이트웨이 베어러 토큰 수명. 기본값 `1`. CLI는 IdP가 새로 고침 토큰을 발급할 때 만료 전에 자동으로 새로 고칩니다. 더 짧은 수명은 더 빠르게 프로비저닝을 해제합니다. 더 긴 수명은 더 적은 IdP 왕복을 만듭니다. IdP가 `offline_access`를 사용할 수 없어서 새로 고침 토큰을 발급할 수 없으면 자동 새로 고침이 없으므로 개발자가 매시간 브라우저 로그인으로 돌아가는 것을 피하기 위해 이를 `8` 또는 `12`로 올립니다. |

<h3 id="store">
  `store`
</h3>

`store` 블록은 게이트웨이를 PostgreSQL 데이터베이스로 지정합니다. 이 데이터베이스는 장치 권한 부여 및 속도 제한 카운터를 보유합니다.

| 필드                | 필수  | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ----------------- | --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | 예   | `postgres://` 또는 `postgresql://` URL. 필수: 장치 권한 부여 랑데부(브라우저 콜백이 쓰고 폴링 CLI가 읽음)는 교차 복제본 상태가 필요합니다. 게이트웨이는 부팅 시 자체 스키마 마이그레이션을 실행하므로 역할은 대상 스키마에 대해 `CREATE TABLE`이 필요합니다. 보안 정책이 애플리케이션 역할의 DDL을 금지하는 경우 관리자 역할로 마이그레이션을 실행하고(초기에 그리고 새 릴리스가 마이그레이션을 제공할 때마다 다시) 앱 역할에 게이트웨이의 테이블에 대해 `SELECT, INSERT, UPDATE, DELETE`를 부여합니다. [업그레이드](/docs/ko/claude-apps-gateway-deploy#upgrades) 및 [Postgres](/docs/ko/claude-apps-gateway-deploy#postgres)를 참조하세요. |
| `username`        | 아니요 | `postgres_url`의 사용자를 재정의합니다                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `password`        | 아니요 | 데이터베이스 자격증명. 자격증명이 URL에서 벗어나도록 `postgres_url`이 아닌 여기에 설정합니다. 모든 문자를 허용하고 URL 자격증명보다 우선합니다.                                                                                                                                                                                                                                                                                                                                                   |
| `max_connections` | 아니요 | 복제본당 Postgres 연결 풀 크기. 기본값 `5`(보수적이고 공유 데이터베이스에 친화적). [지출 한도](#admin)가 활성화되면 핫 경로는 추론 요청당 몇 가지 작업을 수행하므로 로드 아래의 전용 데이터베이스에 대해 올리고 복제본 × 이를 데이터베이스의 `max_connections` 아래로 유지합니다.                                                                                                                                                                                                                                                              |

로컬 개발의 경우 `postgres_url`을 일회용 Postgres 컨테이너로 지정합니다(예: `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`).

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams`는 정렬된 목록입니다. 게이트웨이는 요청된 모델을 확인하는 첫 번째 업스트림으로 추론을 전달합니다. `5xx`, `429`, `401`, `403`, `404` 또는 시간 초과 시 다음으로 장애 조치합니다. 다른 `4xx`는 그렇지 않습니다. 이러한 오류는 업스트림이 아닌 요청에 기인하기 때문입니다. `401` 또는 `403`은 게이트웨이 자신의 자격증명이 해당 업스트림에 대해 실패했음을 의미하고, `404`는 해당 업스트림이 요청된 모델을 제공하지 않음을 의미하므로 목록의 나중 업스트림이 여전히 제공할 수 있습니다.

`404`에서의 장애 조치는 게이트웨이 v2.1.198 이상이 필요합니다. 이전 릴리스는 목록의 나중 업스트림이 모델을 제공했을 때도 첫 번째 `404`를 클라이언트에 반환했습니다.

동일한 공급자의 여러 업스트림은 고유한 `name:`을 설정해야 합니다.

Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform 및 Microsoft Foundry 클라이언트는 시작 시 한 번 구축되며 SDK는 자격증명을 내부적으로 새로 고치므로 클라우드 자격증명을 회전해도 재시작이 필요하지 않습니다. 정적 Anthropic API 키 및 베어러는 시작 시 읽혀집니다. [Anthropic API](#anthropic-api)를 참조하세요.

<h4 id="anthropic-api">
  Anthropic API
</h4>

최소 Anthropic 업스트림은 [Claude 콘솔](https://platform.claude.com)의 API 키입니다:

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # 또는 OAuth 베어러(예: Workload-Identity-Federation 교환 토큰):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # 기본값; 전달 프록시에 대해 재정의
```

두 자격증명 형식은 전송하는 헤더가 다릅니다:

* **`api_key`**: `x-api-key`를 보냅니다. Claude 콘솔에서 회전하고 환경 변수를 업데이트합니다.
* **`oauth_token`**: `Authorization: Bearer`를 보냅니다. 조직이 장기 API 키 대신 단기 토큰을 발급할 때 베어러 형식을 사용합니다. 베어러는 시작 시 한 번 읽혀지므로 비밀을 다시 마운트하고 재시작하여 새로 고칩니다.

정적 키 또는 베어러 대신 Workload Identity Federation을 사용할 수 있습니다. [Workload Identity Federation 가이드](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)를 따라 페더레이션 규칙을 만든 후 Kubernetes 프로젝트된 서비스 계정 토큰 또는 CI 플랫폼의 id-token과 같은 파일로 워크로드의 OIDC JWT를 마운트합니다. 게이트웨이는 JWT를 단기 베어러로 교환하고 자동으로 새로 고칩니다. 토큰 파일은 모든 교환에서 다시 읽혀지므로 회전된 프로젝트된 토큰은 재시작 없이 선택됩니다.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # 규칙이 >1 워크스페이스를 다루는 경우 필수
      # service_account_id: svac_...   # 선택적 예상 대상 확인
```

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

게이트웨이가 대체하거나 앞에 있는 클라이언트 측 Bedrock 배포의 경우 [Amazon Bedrock의 Claude Code](/docs/ko/amazon-bedrock)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # 선호: AWS 기본 자격증명 체인
    # 또는 명시적 자격증명:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # 또는 Bedrock API 베어러 토큰:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # FIPS 또는 VPC 엔드포인트 배포를 위해 bedrock-runtime 엔드포인트를 재정의합니다:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

빈 `auth` 블록은 AWS SDK의 기본 자격증명 체인을 사용합니다: 환경 변수, `~/.aws/credentials`, ECS 작업 역할, EC2 인스턴스 메타데이터 또는 EKS의 IRSA. 프로덕션에서는 컨테이너 이미지에 정적 키를 포함하는 대신 게이트웨이 포드에 IAM 역할을 제공합니다.

명시적 자격증명은 완전해야 합니다: 게이트웨이는 `aws_access_key_id` 및 `aws_secret_access_key`가 함께 설정되지 않거나 `aws_session_token`이 이들 없이 설정된 경우 부팅 시 실패합니다. v2.1.207 이전에는 부분 `auth:` 블록이 검증을 통과했습니다.

| 설정        | 방법                                                                                                                                                                                                                                                                    |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 권한    | 게이트웨이의 주체에 추론 프로필 ARN 및 기본 기초 모델 ARN 모두에 대해 `bedrock:InvokeModel` 및 `bedrock:InvokeModelWithResponseStream`을 부여합니다. US 지역의 기본 제공 카탈로그의 경우: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` 및 `arn:aws:bedrock:*::foundation-model/anthropic.*`. |
| 모델 액세스    | Bedrock 콘솔에서 지역별로 원하는 Claude 모델에 대한 모델 액세스를 요청하고 활성화합니다. 교차 지역 추론 프로필(`us.anthropic.*`)은 프로필이 걸쳐 있는 각 지역에서 모델 액세스가 필요합니다.                                                                                                                                             |
| EKS(IRSA) | 위의 정책과 클러스터의 OIDC 공급자로 범위가 지정된 신뢰 정책이 있는 IAM 역할을 만듭니다. 게이트웨이의 서비스 계정에 `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`로 주석을 답니다. `auth: {}`가 선택합니다.                                                                                              |
| ECS / EC2 | IAM 역할을 작업 정의 또는 인스턴스 프로필에 연결합니다. `auth: {}`가 선택합니다.                                                                                                                                                                                                                  |
| 다른 곳      | `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` 및 `AWS_SESSION_TOKEN` 환경 변수를 통해 자격증명을 전달하거나 `auth:`에서 `${VAR}` 확장으로 명시적으로 설정합니다                                                                                                                                          |
| 지역        | `region:`은 API 엔드포인트 지역입니다. 교차 지역 추론 프로필은 선택한 지역과 무관하게 지역(US, EU, APAC)을 통해 라우팅합니다. US가 아닌 지역 또는 프로비저닝된 처리량 ARN의 경우 올바른 업스트림별 ID가 있는 [`models:`](#models) 블록을 추가합니다.                                                                                                  |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS는 `aws-external-anthropic.<region>.api.aws`에서 AWS 인프라의 퍼스트파티 Anthropic API를 제공합니다. 퍼스트파티 모델 ID를 사용하고, 전송된 대로 `anthropic-beta` 헤더를 준수하며, `count_tokens`를 제공하므로 Bedrock 특정 변환이 적용되지 않습니다. `anthropicAws` 공급자는 Claude Code v2.1.198 이상이 필요합니다. 이전 게이트웨이 릴리스는 부팅 시 이를 거부합니다.

동일한 플랫폼의 클라이언트 측 배포의 경우 [Claude Platform on AWS의 Claude Code](/docs/ko/claude-platform-on-aws)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # x-api-key로 전송됨
    # 또는 AWS 기본 자격증명 체인을 통한 SigV4:
    # auth: {}
    # 또는 명시적 SigV4 자격증명:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # 파생된 엔드포인트를 재정의합니다:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

플랫폼은 Amazon Bedrock과 별도의 AWS 계정에서 실행되며 자신의 서비스 이름인 `aws-external-anthropic`에 대해 SigV4 요청에 서명하므로 Bedrock 범위의 IAM 역할이 이를 인증하지 않습니다. `auth.api_key`의 API 키는 SigV4 자격증명도 설정된 경우 우선합니다. 빈 `auth` 블록은 AWS SDK의 기본 자격증명 체인을 사용합니다. 이는 [Amazon Bedrock](#amazon-bedrock) 업스트림이 사용하는 동일한 체인입니다.

| 필드                                                      | 필수  | 설명                                                                                             |
| ------------------------------------------------------- | --- | ---------------------------------------------------------------------------------------------- |
| `region`                                                | 예   | AWS 지역, 소문자, 숫자 및 하이픈. 게이트웨이는 `https://aws-external-anthropic.<region>.api.aws`로 엔드포인트를 파생합니다. |
| `workspace_id`                                          | 예   | 모든 요청에서 헤더로 전송됨; 플랫폼이 필요로 함                                                                    |
| `auth.api_key`                                          | 아니요 | 플랫폼의 API 키, `x-api-key`로 전송됨. 베어러 토큰이 아님: 두 인증 모드는 API 키 또는 SigV4입니다.                          |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | 아니요 | 명시적 SigV4 자격증명. 하나를 다른 하나 없이 설정하면 부팅 시 실패합니다. `auth.aws_session_token`은 이들과 함께 허용됩니다.          |
| `base_url`                                              | 아니요 | 파생된 엔드포인트를 재정의합니다                                                                              |

플랫폼이 퍼스트파티 모델 ID를 확인하므로 기본 제공 카탈로그는 [`models:`](#models) 블록 없이 이로 라우팅합니다. `models:` 목록을 큐레이션할 때 항목을 `anthropicAws:`로 퍼스트파티 ID로 키합니다.

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

동등한 클라이언트 측 설정의 경우 [Google Cloud의 Claude Code](/docs/ko/google-vertex-ai)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # 선호: Application Default Credentials
    # 또는 서비스 계정 키 파일:
    # auth: { service_account_json: /secrets/sa.json }
    # Private Service Connect를 위해 aiplatform 엔드포인트를 재정의합니다:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

빈 `auth` 블록은 Application Default Credentials를 사용합니다: `GOOGLE_APPLICATION_CREDENTIALS`, GCE 메타데이터 또는 GKE Workload Identity. 서비스 계정 JSON 키 파일은 지원되지만 권장되지 않습니다. Workload Identity를 사용하거나 GCE 또는 Cloud Run 인스턴스에 서비스 계정을 연결합니다.

`region: global`을 설정하여 지역 엔드포인트 대신 [Google Cloud의 Agent Platform의 글로벌 엔드포인트](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)를 사용합니다. Google은 각 요청을 사용 가능한 지역으로 라우팅하므로 지역별 모델 가용성을 추적하지 않습니다. 특정 지역을 설정하면 모든 요청이 해당 지역에 고정됩니다.

| 설정                     | 방법                                                                                                                                                                   |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 권한                 | 게이트웨이의 서비스 계정에 프로젝트에 대해 `roles/aiplatform.user`를 부여하거나 `aiplatform.endpoints.predict`가 있는 사용자 정의 역할을 부여합니다. Agent Platform API(`aiplatform.googleapis.com`)를 활성화합니다. |
| 모델 액세스                 | Model Garden에서 프로젝트에 대해 Claude 모델을 활성화합니다. 특정 지역에 게시합니다. 지원되는 지역은 모델 카드를 확인하세요.                                                                                      |
| GKE(Workload Identity) | GCP 서비스 계정을 게이트웨이의 Kubernetes 서비스 계정에 바인드하고 KSA에 `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`으로 주석을 답니다. `auth: {}`가 선택합니다.       |
| Cloud Run / GCE        | 서비스의 서비스 계정을 `roles/aiplatform.user`가 있는 것으로 설정합니다. `auth: {}`가 선택합니다.                                                                                               |
| 다른 곳                   | `auth: { service_account_json: /secrets/sa.json }`, JSON 키 파일의 경로가 비밀로 마운트됩니다. 필드는 키 내용이 아닌 파일 경로를 사용하므로 `${file:…}` 확장이 관련되지 않습니다.                                  |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

클라이언트 측 Foundry 배포의 경우 [Microsoft Foundry의 Claude Code](/docs/ko/microsoft-foundry)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # 선호: DefaultAzureCredential / Managed Identity
    # 또는 API 키:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true`는 `DefaultAzureCredential`을 통해 확인합니다: AKS, ACI 또는 App Service의 Managed Identity, Azure CLI 또는 환경 자격증명. API 키는 작동하지만 프로젝트 전체이며 자동으로 회전하지 않습니다. Foundry의 엔드포인트는 `resource:`에서 파생됩니다. Azure Government와 같은 소주권 클라우드에 대해 선택적 `base_url`을 설정하여 재정의합니다.

| 설정                | 방법                                                                                                                                    |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC              | 게이트웨이의 ID에 Foundry 리소스에 대해 `Azure AI User` 또는 `Cognitive Services User`를 부여합니다                                                        |
| 배포                | Foundry는 정규 모델 ID가 아닌 관리자 선택 배포 이름을 사용합니다. 각 정규 ID를 배포 이름에 매핑하는 [`models:`](#models) 블록을 추가합니다.                                       |
| AKS(워크로드 ID)      | 사용자 할당 Managed Identity를 클러스터의 OIDC 발급자와 페더레이션하고 게이트웨이의 서비스 계정에 바인드합니다. `use_azure_ad: true`가 `WorkloadIdentityCredential`을 통해 선택합니다. |
| ACI / App Service | 리소스에서 시스템 할당 또는 사용자 할당 관리 ID를 활성화합니다. `use_azure_ad: true`가 선택합니다.                                                                    |
| 다른 곳              | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. `{ }` 내에서 `${…}`를 인용합니다.                                                                   |

<h4 id="multiple-upstreams">
  여러 업스트림
</h4>

동일한 공급자는 고유한 `name:`으로 두 번 이상 나타날 수 있습니다. 이는 다양한 지역, 다양한 자격증명 체인을 통한 다양한 계정, 프로비저닝된 처리량 대 온디맨드 및 교차 공급자 장애 조치를 다룹니다.

게이트웨이는 순서대로 업스트림을 시도합니다. `5xx`, `429`, `401`, `403`, `404`, 시간 초과 및 누락된 엔드포인트(`501`)는 장애 조치합니다. 다른 `4xx`는 그렇지 않습니다. `429`는 업스트림별 용량이므로 프로비저닝된 처리량(PT) 소진은 온디맨드로 장애 조치합니다. `404`는 업스트림별 모델 가용성이므로 모델을 활성화하지 않은 업스트림은 해당 모델을 제공하는 나중 업스트림을 차단하지 않습니다. 요청된 모델을 확인할 수 없는 업스트림은 네트워크 왕복 없이 건너뜁니다.

이 예제는 프로비저닝된 처리량 Bedrock 할당을 먼저 라우팅하고, 온디맨드 및 두 번째 계정으로 오버플로우하며, 마지막으로 Anthropic API로 장애 조치합니다:

```yaml theme={null}
upstreams:
  # 기본: 홈 지역의 프로비저닝된 처리량.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # 오버플로우: 온디맨드 교차 지역.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # 다른 계정: 가정된 역할 자격증명을 통한 별도의 Bedrock 할당.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # 최후의 수단: 직접 Anthropic API.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# 업스트림별 모델 ID는 업스트림의 `name:`으로 키가 지정됩니다. `name:`이 없는 업스트림은
# 기본값으로 공급자 문자열(예: `bedrock`)을 사용합니다. 모델에 대해 나열되지 않은 모든
# 업스트림은 건너뜁니다. 이것이 모델을 프로비저닝된 처리량으로 라우팅하는 방법입니다.
# 다른 모든 것은 온디맨드로 유지됩니다.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| 레버               | 방법                                                                                                                                                        |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 다른 지역            | 지역당 하나의 Bedrock 업스트림, 각각 자신의 `region:`. [`auto_include_builtin_models: true`](#models)를 사용하면 교차 지역 추론 프로필이 자동으로 라우팅됩니다. 지역 고정 배포의 경우 `models:` 블록을 사용합니다. |
| 다른 계정            | 계정당 하나의 Bedrock 업스트림, 각각 `auth:`의 자신의 자격증명. 기본 체인(`auth: {}`)은 포드의 ID를 사용합니다. 두 번째 계정의 경우 명시적 자격증명 또는 베어러 토큰을 설정합니다.                                      |
| 프로비저닝된 처리량       | 해당 업스트림의 이름에 대해 `models:`에서 모델을 프로비저닝된 처리량 ARN에 매핑합니다. 다른 업스트림은 온디맨드 ID를 유지하므로 PT 용량이 장애 조치 전에 소진됩니다.                                                     |
| VPC / FIPS 엔드포인트 | 업스트림에서 `base_url:`을 VPC 엔드포인트 또는 FIPS 엔드포인트 URL로 설정합니다                                                                                                    |
| 모델 범위 라우팅        | 모델의 `upstream_model:` 맵에서 업스트림을 생략하면 해당 업스트림은 해당 모델에 대해 건너뜁니다. 예를 들어 Opus를 프로비저닝된 처리량으로 라우팅하고 Sonnet 및 Haiku를 온디맨드로 라우팅합니다.                               |

클라우드 공급자 간 또는 직접 Anthropic API로 장애 조치하면 요청을 관리하는 계약, 지역 및 기타 약관이 변경됩니다.

CLI는 어느 업스트림이 주어진 요청을 제공하는지와 무관하게 게이트웨이에 동일한 기능 게이팅을 적용하므로 장애 조치는 업스트림이 거부할 본문 필드를 보내지 않습니다.

<h2 id="optional-sections">
  선택 사항 섹션
</h2>

<h3 id="admin">
  `admin`
</h3>

선택 사항. Anthropic의 공개 관리자 API를 미러링하는 `/v1/organizations/spend_limits`를 활성화하고 `/v1/messages`에서 개발자별 지출 적용을 활성화합니다. 한도가 설정되고 적용되는 방법은 [지출 한도](/docs/ko/claude-apps-gateway-spend-limits)를 참조하세요. 이 섹션은 기능을 켜고 조정하는 `gateway.yaml` 키를 다룹니다.

```yaml theme={null}
admin:
  # 관리자 엔드포인트에 대한 명명된 정적 API 키, x-api-key로 전송됨.
  # id는 감사 로그에 admin-key:<id>로 나타나므로 각 키는
  # 귀속 가능합니다. 회전용 배열: 새 키를 추가하고 클라이언트를 롤링하고
  # 이전 키를 제거합니다.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # 일반 게이트웨이 JWT를 통해 전체 관리자 권한이 부여된 IdP 그룹(API 키 없음).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| 필드                        | 필수  | 설명                                                                                                                                                                                            |
| ------------------------- | --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | 아니요 | `{id, key}` 배열. 이 중 하나와 일치하는 `x-api-key`는 지출 한도를 나열, 설정 및 삭제할 수 있습니다. 키 값은 최소 32자여야 합니다. `id`는 `read_keys` 및 `write_keys`에서 고유해야 합니다.                                                         |
| `read_keys`               | 아니요 | `{id, key}` 배열. 읽기 전용: 모든 `GET` 엔드포인트, 한도 나열, ID별 가져오기 및 [`/effective`](/docs/ko/claude-apps-gateway-spend-limits#%2Feffective) 및 [`/audit`](/docs/ko/claude-apps-gateway-spend-limits#%2Faudit) 읽기 포함. |
| `admin_groups`            | 아니요 | IdP 그룹 이름. `groups` 클레임이 이 중 하나를 포함하는 게이트웨이 JWT는 전체 관리자 액세스(읽기 및 쓰기)를 가지며 `oidc:<sub>`로 감사합니다. 인간 관리자에게 사용합니다. 기계에는 API 키를 사용합니다.                                                             |
| `blocked_message`         | 아니요 | 차단된 개발자가 보는 `429 billing_error`에 그대로 추가됩니다. URL 또는 Slack 채널과 같은 전체 지침을 작성합니다. 설정하지 않으면 오류는 `spend limit reached`입니다.                                                                          |
| `audit_retention_days`    | 아니요 | 기본값 `365`. 더 오래된 `admin_audit` 행은 정리됩니다.                                                                                                                                                      |
| `spend_retention_months`  | 아니요 | 기본값 `13`. 이보다 오래된 `spend` 카운터 행은 정리됩니다. 기본값은 연간 비교 보고를 위해 전체 연도와 현재 부분 월을 유지합니다.                                                                                                              |
| `identity_retention_days` | 아니요 | 기본값 `90`. `principal_emails` 행의 마지막 표시 TTL(각 개발자의 이메일, 표시 이름 및 그룹 포함, PII). 의도적으로 지출 보유보다 짧으므로 프로비저닝 해제된 ID는 익명 지출 카운터가 유지되는 동안 만료됩니다.                                                        |
| `group_limit_mode`        | 아니요 | `min`(기본값) 또는 `max`. 개발자가 한도가 있는 여러 그룹에 있을 때 `min`은 가장 제한적인 것을 적용하고 `max`는 가장 제한적이지 않은 것을 적용합니다. 적용 및 `/effective` 모두에서 사용됩니다.                                                                |

<h3 id="enforcement">
  `enforcement`
</h3>

`enforcement` 블록은 저장소를 사용할 수 없을 때 지출 한도 확인이 어떻게 동작하는지 제어합니다.

| 필드                     | 필수  | 설명                                                                                                                                                 |
| ---------------------- | --- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | 아니요 | 기본값 `false`. 지출 적용은 Postgres 중단 시 개방되므로 추론이 계속됩니다. `true`로 설정하여 폐쇄: 초과 용량 개발자는 차단되지만 저장소에 도달할 수 없으면 모두가 차단됩니다. [`admin:`](#admin) 블록 없이는 효과가 없습니다. |

<h3 id="models">
  `models`
</h3>

`models` 블록은 선택적 관리자 선별 모델 목록이며 `/v1/models`에서 제공되고 업스트림별 모델 ID를 변환하는 데 사용됩니다. US가 아닌 Amazon Bedrock 지역, Amazon Bedrock 프로비저닝된 처리량 ARN 및 Microsoft Foundry 배포 이름에 필수입니다.

```yaml theme={null}
auto_include_builtin_models: true   # false: 아래 목록만 노출
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: 선택적 텍스트가 표시되는 클라이언트에 표시됨
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # 또는 추론 프로필 ARN
      foundry: your-opus-deployment-name
```

<h3 id="managed">
  `managed`
</h3>

`managed` 블록은 IdP 그룹 또는 이메일 도메인을 기반으로 하는 역할 기반 액세스 정책을 정의합니다. 정책은 순서대로 평가됩니다. 첫 번째 일치가 선택되고 아래에 설명된 `match: {}` 캐치올 기본에 병합됩니다. 사용자별로 `GET /managed/settings`에서 ETag/304 캐싱으로 제공됩니다.

```yaml theme={null}
managed:
  policies:
    # 특정 그룹을 먼저.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # 기본 캐치올 마지막: 인증된 모든 사용자와 일치.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

`match: {}` 캐치올(관례상 마지막에 나열됨)은 기본 계층으로 처리됩니다. 모든 다른 정책은 설정하지 않은 모든 키를 캐치올에서 상속하므로 역할별 항목은 조직 기본값과 다른 것만 나열하면 됩니다. 병합 규칙은 키 유형에 따라 다릅니다:

* **허용 목록**: `availableModels` 및 `permissions.allow`. 특정 정책의 목록은 기본값을 완전히 대체합니다.
* **거부 목록 및 후크 배열**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` 및 모든 `hooks` 이벤트 유형 배열. 이들은 기본값과 정책의 합집합을 사용하므로 조직 전체 거부 또는 감사 후크는 역할별 재정의로 실수로 삭제될 수 없습니다.
* **레코드 유형 키**: `env`, `modelOverrides` 및 `skillOverrides`. 이들은 얕게 병합되므로 역할별 `env` 블록은 설정하는 키를 재정의하고 나머지는 기본값에서 상속합니다.

`availableModels`는 또한 `/v1/messages`에서 서버 측으로 적용되므로 거부된 모델은 클라이언트가 보내는 것과 무관하게 `400`을 반환합니다.

| 매처                                                  | 동작                                                                                |
| --------------------------------------------------- | --------------------------------------------------------------------------------- |
| `match: {}`                                         | 인증된 모든 사용자와 일치합니다. 이 중 하나로 시작하고 나중에 그룹 범위 정책을 위에 추가합니다.                           |
| `match: { groups: [a, b] }`                         | JWT의 `groups` 클레임이 나열된 그룹 중 하나를 포함하면 일치합니다. 대소문자 구분: 그룹은 IdP의 정확한 대소문자와 일치해야 합니다. |
| `match: { email_domain: example.com }`              | JWT의 `email` 클레임에서 마지막 `@` 뒤의 부분과 일치합니다(대소문자 구분 안 함). 정책당 하나의 도메인을 허용합니다.         |
| `match: { groups: [a], email_domain: example.com }` | 두 조건 모두 일치해야 합니다                                                                  |

정책과 일치하지 않는 인증된 사용자는 게이트웨이의 기본값을 가져옵니다. 이는 카탈로그의 모든 모델과 관리형 설정이 없음을 의미합니다. 보장된 기본 정책을 원하면 마지막에 `match: {}` 캐치올을 추가합니다.

<Note>
  게이트웨이는 자신의 사용자 디렉토리를 유지하지 않습니다. 사용자의 IdP 토큰에서 각 요청을 승인하고 토큰의 `groups` 클레임에서 그룹 멤버십을 읽으며 이에 대해 정책을 평가합니다. 열거할 명단이 없고 사전 생성할 계정이 없으므로 SCIM 엔드포인트가 없습니다. 동기화할 것이 없기 때문입니다.

  사용자 및 그룹 수명 주기 관리를 진실의 원본(IdP의 기본 SCIM 프로비저닝 또는 전용 ID 거버넌스 플랫폼)에서 실행합니다. 거기서 관리되는 멤버십 및 프로비저닝 해제는 토큰을 통해 게이트웨이에 자동으로 흐릅니다. Claude 계정 자체의 SCIM 프로비저닝을 원하면 이는 [Claude for Enterprise](/docs/ko/admin-setup) 기능입니다.

  두 가지 전파 시계가 적용됩니다:

  * **정책 내용**: 정책을 편집하고 재배포하면 연결된 클라이언트가 다음 관리형 설정 폴링 시 1시간 이내에 도달합니다.
  * **그룹 멤버십**: 사용자의 그룹 멤버십을 변경하면 어떤 정책이 일치하는지 변경됩니다. 이는 다음 세션 재발급 시(다음 자동 새로 고침 의미)에 적용되며 `session.ttl_hours`로 제한됩니다.
</Note>

<h4 id="what-goes-in-cli">
  `cli`에 들어가는 것
</h4>

각 `cli` 값은 완전한 Claude Code `managed-settings.json` 문서이며, MDM 또는 `/etc/claude-code/managed-settings.json`을 통해 배포할 동일한 스키마이며, 여기서는 YAML로 표현됩니다. CLI는 관리형 계층에서 전달된 문서를 적용합니다. 사용자 및 프로젝트 설정 위에 있습니다.

게이트웨이는 부팅 시 각 문서를 CLI의 설정 스키마에 대해 검증하므로 인식되지 않는 최상위 키 또는 인식되지만 잘못된 형식의 값이 있는 키는 모든 위반 키의 이름을 지정하는 오류로 부팅을 실패합니다. 의도적으로 개방된 스키마 부분은 여전히 임의의 값을 허용합니다. 더 새로운 클라이언트가 게이트웨이의 스키마가 인식하지 못하는 항목을 인식할 수 있기 때문입니다. 이러한 개방 키는 `env`, `pluginConfigs` 및 `permissions` 아래에 중첩된 키입니다.

검증이 게이트웨이의 설치된 버전과 함께 번들된 스키마를 사용하므로 더 새로운 Claude Code 릴리스에서 도입된 최상위 설정 키를 관리형 구성에 넣으려면 먼저 게이트웨이를 업그레이드해야 합니다. 한 클라이언트에서 새 정책을 연기 테스트한 후 롤아웃합니다.

전체 키 참조는 [Claude Code 설정](/docs/ko/settings#available-settings)에 있습니다. 운영자가 먼저 도달하는 키:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # 모델 액세스(/v1/messages에서도 서버 측으로 적용됨)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # 권한 정책
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # --dangerously-skip-permissions 차단
        allowManagedPermissionRulesOnly: true     # 사용자/프로젝트 권한 규칙 무시

        # CLI 프로세스에 푸시된 환경. DISABLE_UPDATES는 배경 및 수동 업데이트를 차단합니다.
        # DISABLE_AUTOUPDATER는 배경 업데이트만 중지합니다.
        env:
          DISABLE_UPDATES: "1"                    # 자신의 배포를 통해 버전 고정

        # 조직 전체 후크. 후크 명령은 게이트웨이가 아닌 개발자 머신에서 실행되므로
        # 경로는 정책의 모든 클라이언트 OS에 존재해야 합니다.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| 키                                          | 적용 대상       | 효과                                                                                                                                                      |
| ------------------------------------------ | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | 게이트웨이 + CLI | 모델 허용 목록. `/v1/messages`에서도 확인되므로 패치된 클라이언트는 이를 우회할 수 없습니다.                                                                                             |
| `permissions.allow` / `.deny`              | CLI         | 도구 및 명령 규칙. [권한](/docs/ko/permissions)을 참조하세요.                                                                                                               |
| `permissions.disableBypassPermissionsMode` | CLI         | [`bypassPermissions`](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode) 모드를 차단하도록 `disable`으로 설정하고 `--dangerously-skip-permissions` 플래그 |
| `allowManagedPermissionRulesOnly`          | CLI         | `true`일 때 사용자 및 프로젝트 권한 규칙은 무시됩니다. 이 문서의 규칙만 적용됩니다                                                                                                      |
| `env`                                      | CLI         | CLI 프로세스에 병합된 환경 변수. 텔레메트리, 자동 업데이트 및 모델 이름 재정의에 사용합니다.                                                                                                 |
| `hooks`                                    | CLI         | 조직 전체 [후크](/docs/ko/hooks)                                                                                                                                   |

이러한 설정은 네트워크를 통해 도착하므로 CLI는 각 개발자에게 셸 명령을 실행하거나 트래픽이 가는 위치를 변경할 수 있는 모든 것을 적용하기 전에 일회성 보안 승인 대화를 표시합니다. 대화는 다음을 다룹니다:

* `hooks`
* CLI의 기본 제공 안전 목록에 없는 `env` 변수
* `apiKeyHelper` 및 `statusLine`과 같은 셸 실행 설정
* 관리형 CLAUDE.md 내용

안전 목록은 어떤 `env` 변수가 승인 없이 적용되는지 결정합니다:

* **안전 목록에 있음**: 자동 업데이트 및 모델 이름 변수
* **안전 목록에 없음**: 프록시 변수, 기본 URL 변수 및 `OTEL_EXPORTER_OTLP_ENDPOINT`

게이트웨이의 [텔레메트리](#telemetry) 구성은 `OTEL_EXPORTER_OTLP_ENDPOINT`를 푸시하므로 `telemetry.forward_to`를 설정하면 각 대화형 클라이언트에서 대화를 트리거합니다. 대화는 조직으로부터 개발자를 보호하지 않고 손상되거나 적대적인 게이트웨이로부터 개발자의 머신을 보호합니다.

`-p` 플래그를 사용한 비대화형 실행은 대화를 표시할 수 없습니다. 해당 실행에 대해서만 푸시된 설정을 적용하고 승인된 것으로 기록하지 않으므로 개발자의 다음 대화형 세션은 여전히 대화를 표시합니다. v2.1.207 이전에는 비대화형 실행이 설정을 승인된 것으로 저장했고 이후 대화형 세션은 이에 대한 대화를 표시하지 않았습니다.

개발자가 거부하면 Claude Code는 정책을 적용하지 않고 종료됩니다. 새 후크 또는 비안전 env 변수를 광범위한 정책에 푸시하면 일치하는 모든 개발자의 다음 시작 시 승인 프롬프트가 표시됩니다.

`cli` 키는 이전 릴리스에서 `settings`로 명명되었습니다. 해당 철자는 여전히 별칭으로 허용되지만 새 배포는 `cli`를 사용해야 합니다.

<h4 id="precedence-with-other-managed-sources">
  다른 관리형 소스와의 우선순위
</h4>

장치에 로컬 `managed-settings.json` 또는 MDM 전달 정책도 있으면 관리형 소스는 병합되지 않습니다. 가장 높은 우선순위 소스는 모든 정책 설정을 제공하며 다음 순서로 순위가 지정됩니다(가장 높은 우선순위 먼저):

1. [정책 도우미](/docs/ko/settings#compute-managed-settings-with-a-policy-helper)
2. 게이트웨이 전달 설정
3. MDM, Windows의 HKLM 레지스트리 또는 macOS의 plist를 통해
4. `managed-settings.json` 파일
5. HKCU 레지스트리(Windows만 해당)

포함 호스트는 SDK `managedSettings` 옵션을 통해 정책을 제공할 수 있습니다. 기본적으로 무시되며 관리형 소스가 [`parentSettingsBehavior: "merge"`](/docs/ko/settings#available-settings)로 옵트인할 때만 적용되며, 정책을 강화할 수 있지만 완화할 수 없도록 필터링됩니다.

예외는 모든 관리형 소스가 설정할 때 적용되는 작은 키 집합입니다. 사용자 쓰기 가능 HKCU 계층은 제외됩니다:

* `sandbox.network.allowManagedDomainsOnly` 및 `sandbox.filesystem.allowManagedReadPathsOnly`: 잠금 시 해당 허용 목록은 소스 전체에서 합집합됩니다.
* [`allowAllClaudeAiMcps`](/docs/ko/settings#available-settings): claude.ai MCP 서버 허용 목록에 대한 허용 전용 재정의
* `sandbox.bwrapPath` 및 `sandbox.socatPath`: [샌드박스](/docs/ko/sandboxing) 도우미 바이너리의 파일 시스템 경로
* [`forceRemoteSettingsRefresh`](/docs/ko/server-managed-settings): 원격 관리형 설정이 새로 가져올 때까지 시작을 차단하므로 키가 없는 캐시된 원격 페이로드가 가장 높은 우선순위 소스인 경우에도 MDM 또는 파일 정책이 적용됩니다.

`allowManagedPermissionRulesOnly` 및 `disableBypassPermissionsMode`는 교차 소스가 아니므로 승리한 소스의 값만 적용됩니다. 설정 페이지의 동일한 규칙은 [설정 우선순위](/docs/ko/settings#settings-precedence)를 참조하세요.

게이트웨이 정책은 비대화형 `claude -p` 실행 및 Agent SDK에서 생성된 세션을 포함하여 머신의 모든 Claude Code 호출에 적용됩니다. 게이트웨이가 시작 시 도달할 수 없으면 서명된 세션은 정책 없이 실행하지 않고 오류로 종료됩니다.

<Warning>
  정책의 `cli` 블록 내의 `mcpServers`는 게이트웨이 부팅 시 거부됩니다. 그룹별 MCP 배포는 사용할 수 없습니다. 각 장치에서 파일 기반 `managed-mcp.json`을 통해 MCP 서버를 배포하거나 개발자가 로컬로 추가하도록 합니다.
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

CLI는 OpenTelemetry Protocol(OTLP) over HTTP 메트릭, 로그 및 활성화 시 추적을 게이트웨이로 보내며, 게이트웨이는 각 구성된 대상으로 그대로 중계합니다. CLI가 내보내는 메트릭 및 이벤트는 [사용 모니터링](/docs/ko/monitoring-usage)을 참조하세요.

CLI는 각 내보내기에 게이트웨이 발급 JWT에서 읽은 인증된 사용자의 ID를 스탬프합니다: `user.id`, `user.email` 및 `user.groups` 속성. 개발자별 비용 및 사용 귀속은 개발자 측 구성 없이 작동합니다.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # 신호별 옵트인. 기본값: 메트릭만.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  각 대상은 `metrics`, `logs` 및 `traces`에 독립적으로 옵트인하며 기본값은 메트릭만입니다. 신호는 민감도가 다릅니다:

  * **메트릭**: 토큰 수, 요청 수 및 지연 시간과 같은 집계 카운터
  * **로그 및 추적**: 전체 bash 명령, 도구 입력 및 파일 경로를 전달할 수 있으며 Claude Code가 개발자 머신에서 수행하는 모든 것을 다룹니다.

  로그 및 추적은 해당 데이터가 보증하는 액세스 제어 및 보유 정책이 있는 대상에서만 활성화합니다.
</Warning>

텔레메트리는 CLI에서 기본적으로 꺼져 있습니다. `telemetry.forward_to`를 `listen.public_url`과 함께 구성하면 켜집니다. 게이트웨이는 `/managed/settings`를 통해 모든 연결된 클라이언트에 5개의 환경 변수를 푸시합니다:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

푸시된 엔드포인트는 공개 URL에서 구축되므로 메트릭 및 로그는 개발자 또는 정책의 OTEL 구성이 필요하지 않습니다. 푸시된 구성은 관리형 계층에서 적용되어 개발자가 로컬로 설정하는 `OTEL_*` 변수를 재정의합니다.

[추적](/docs/ko/monitoring-usage#traces-beta)은 추가로 각 클라이언트에서 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`이 필요합니다. 게이트웨이는 해당 변수를 푸시하지 않으므로 관리형 정책의 `env` 블록을 통해 설정합니다. CLI의 안전 목록에 없으므로 정책을 통해 전달하는 것은 푸시된 OTLP 엔드포인트가 이미 트리거하는 동일한 [보안 승인 대화](#managed)로 다룹니다.

Protobuf 및 JSON OTLP 인코딩 모두 중계되며 모든 OpenTelemetry 호환 백엔드가 대상으로 작동합니다.

<h3 id="http-tuning">
  HTTP 조정
</h3>

4개의 선택적 최상위 블록 `access_control`, `limits`, `timeouts` 및 `rate_limits`는 HTTP 표면을 조정합니다. 기본값은 대부분의 배포에 적합합니다.

| 블록               | 키                                              | 기본값      | 설명                                                                                                                                                                                                                                |
| ---------------- | ---------------------------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | 비어 있음    | 클라이언트 주소별 인바운드 IP 허용/거부, `trusted_proxies` 확인 후. `deny_cidrs`가 먼저 확인됩니다. 일치하는 클라이언트는 `allow_cidrs`도 일치하더라도 거부됩니다. `allow_cidrs`가 비어 있지 않으면 게이트웨이는 기본 거부입니다. `/healthz` 및 `/readyz`는 `allow_cidrs`에서 제외됩니다.                        |
| `limits`         | `max_request_bytes`                            | 32 MiB   | 최대 인바운드 요청 본문. 크기 초과 요청은 본문이 버퍼링되기 전에 `413`을 가져옵니다. 큰 파일 또는 이미지 요청에 대해 올립니다.                                                                                                                                                      |
| `limits`         | `max_request_header_bytes`                     | 설정 안 함   | 설정하면 크기 초과 헤더는 `431`을 반환합니다                                                                                                                                                                                                       |
| `limits`         | `max_url_length`                               | 설정 안 함   | 설정하면 과도하게 긴 URL은 `414`를 반환합니다                                                                                                                                                                                                     |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000   | 업스트림의 응답 헤더(첫 바이트까지의 시간)를 기다리는 최대 시간. 응답 본문은 그 후 벽시계 제한 없이 스트리밍됩니다. 직접 Anthropic 업스트림 경로에 적용됩니다. 다른 모든 공급자는 공급자 SDK의 자체 시간 초과로 제한됩니다.                                                                                             |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600 | 인증되지 않은 장치 인증 엔드포인트에 대한 IP당 속도 제한. 공유 송신 IP 또는 NAT 뒤의 큰 조직에 대해 올립니다. 이러한 한도는 장치 권한 부여 로그인 흐름에만 적용되며 `/v1/messages` 추론에는 적용되지 않습니다. [사용자 코드 무차별 대입 공격 저항](/docs/ko/claude-apps-gateway-deploy#user-code-brute-force-resistance)을 참조하세요. |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600 | `/device`에서 `user_code` 제출에 대한 IP당 속도 제한                                                                                                                                                                                          |

<h2 id="complete-example">
  완전한 예제
</h2>

이 전체 참조 구성은 모든 핵심 섹션을 다룹니다. [HTTP 조정 블록](#http-tuning)은 기본값을 유지합니다. 복사하고 필요하지 않은 것을 삭제하고 값을 채웁니다. [빠른 시작](/docs/ko/claude-apps-gateway#quickstart)의 구성은 이것의 최소 버전입니다.

```yaml gateway.yaml theme={null}
# 실행:
#   claude gateway --config gateway.yaml
#
# 운영 로그 상세도는 CLAUDE_GATEWAY_LOG_LEVEL
# 환경 변수(info | warn | error; 기본 info)로 제어됩니다. 감사 이벤트는
# 항상 내보내지므로 영향을 주지 않습니다.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # TLS 종료 ingress 뒤에서 실행할 때 tls 블록을 생략합니다.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Okta org 서버가 발급자인 경우 필수. id_token은
  # 이메일 및 그룹을 생략할 수 있습니다. 게이트웨이는 /userinfo에서 채웁니다.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta는 `groups` 범위가 요청되고 앱의 그룹 클레임 필터가
  # 허용할 때만 그룹을 내보냅니다. 아래의 계약자 정책은
  # 그룹과 일치하므로 범위가 여기에서 요청됩니다.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Entra 앱 역할: `roles` 사용
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# /v1/organizations/spend_limits(Anthropic Admin API를 미러링함)를 활성화합니다.
# 및 /v1/messages에서 개발자별 지출 적용. 비활성화하려면 생략합니다.
# 한도 자체는 관리자 API를 통해 설정되며 여기에서는 설정되지 않습니다.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # 기본 선택기 옵션을 계약자가 기본값을 얻지 않도록
        # availableModels로 제한합니다. 그래서 계약자는 기본값에 400을 얻지 않습니다.
        enforceAvailableModels: true
        # allow는 이 도구를 자동 승인합니다. 나머지를 차단하지 않습니다.
        # 도구를 제한하려면 거부 규칙을 추가합니다.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  클라이언트 측 관리형 설정
</h2>

위의 모든 것은 게이트웨이 서버를 구성합니다. 개발자 머신을 지정하는 것은 각 장치에서 Claude Code의 [관리형 설정](/docs/ko/settings#settings-files)을 통해 별도로 구성됩니다. 게이트웨이는 클라이언트가 게이트웨이가 어디에 있는지 알려주는 키를 푸시할 수 없습니다.

CLI의 경우 OS별 `managed-settings.json`에서 두 키를 모두 설정합니다:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

일반적으로 MDM 플랫폼을 통해 각 장치에 해당 파일을 배포합니다. 파일 경로는 플랫폼마다 다릅니다:

| 플랫폼         | 경로                                                                                                          |
| ----------- | ----------------------------------------------------------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-settings.json` 또는 `com.anthropic.claudecode` 관리형 기본 설정 도메인 |
| Linux 및 WSL | `/etc/claude-code/managed-settings.json`                                                                    |
| Windows     | `C:\Program Files\ClaudeCode\managed-settings.json` 또는 HKLM 레지스트리를 통한 그룹 정책                                 |

`forceLoginGatewayUrl` 및 `forceLoginMethod`의 `"gateway"` 값은 관리자 제어 관리형 계층에서만 적용됩니다. 개발자가 자신의 `~/.claude/settings.json`에서 설정하는 것은 효과가 없습니다.

<h2 id="related">
  관련
</h2>

* [Claude 앱 게이트웨이 개요](/docs/ko/claude-apps-gateway): 빠른 시작 및 개발자 연결
* [배포 가이드](/docs/ko/claude-apps-gateway-deploy): IdP 설정, 컨테이너 이미지, Kubernetes 및 Cloud Run, 운영
* [지출 한도](/docs/ko/claude-apps-gateway-spend-limits): 개발자별 한도 및 관리자 API
