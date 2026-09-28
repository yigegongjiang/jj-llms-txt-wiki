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

5개 섹션은 [필수](#required-sections)입니다. 다른 모든 섹션은 [선택사항](#optional-sections)이며, 생략된 섹션은 기본값을 사용합니다. 알 수 없는 키는 부팅에 실패하므로, 오타는 자동으로 무시되는 설정이 아니라 명명된 오류로 표시됩니다.

**필수 섹션:**

* [`listen`](#listen): 바인드 주소, 공개 URL, TLS 종료
* [`oidc`](#oidc): 발급자, 클라이언트, 클레임 매핑 및 로그인 가능 대상을 포함한 ID 공급자(IdP)
* [`session`](#session): 게이트웨이가 발급하는 베어러 토큰, 비밀 및 수명 포함
* [`store`](#store): 디바이스 권한 부여 및 속도 제한 카운터용 PostgreSQL
* [`upstreams`](#upstreams): 추론이 이동하는 위치, Anthropic, Amazon Bedrock, AWS의 Claude Platform, Google Cloud의 Agent Platform 또는 Microsoft Foundry 여부

**선택사항 섹션:**

* [`admin`](#admin): Admin API 인증 및 지출 한도 보유
* [`enforcement`](#enforcement): 지출 한도 실패 개방 또는 실패 폐쇄 동작
* [`pricing`](#pricing): 계약된 요금 및 지출 미터 및 개발자가 보는 비용 수치에 대한 승수
* [`models`](#models) 및 `auto_include_builtin_models`: 관리자 선별 모델 목록 및 업스트림별 ID
* [`managed`](#managed): IdP 그룹별 관리형 설정 정책
* [`telemetry`](#telemetry): 관찰성 스택으로의 OTLP 전달
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP 허용/거부, 요청 크기 제한, 업스트림 첫 바이트까지의 시간 및 IP별 로그인 한도
* [`load_test_mode`](#load_test_mode): 모델 공급자를 호출하지 않고 게이트웨이 부하 테스트

<h2 id="secret-expansion">
  비밀 확장
</h2>

`client_secret`, `jwt_secret` 또는 `postgres_url`과 같은 비밀을 `gateway.yaml`에 직접 작성하지 마세요. 아래 형식 중 하나로 참조하면 게이트웨이가 부팅 시 환경 변수 또는 파일에서 값을 확인합니다:

| 형식              | 확인 대상                                                                                                                                          | 사용 대상                                       |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| `${VAR}`        | 환경 변수 `VAR`. 정의되지 않으면 부팅 실패.                                                                                                                   | 컨테이너 환경 변수, 환경 주입을 통한 AWS Secrets Manager   |
| `${file:/path}` | 해당 절대 경로의 파일 내용, 정리됨. 참조는 필드의 전체 값이어야 합니다: `${VAR}`와 달리 더 긴 문자열 내에서 확장되지 않으므로, 데이터베이스 암호의 경우 `postgres_url`에 포함시키지 말고 `store.password`를 설정하세요. | Kubernetes Secret 볼륨 마운트, Vault Agent, SOPS |

<h2 id="required-sections">
  필수 섹션
</h2>

<h3 id="listen">
  `listen`
</h3>

`listen` 블록은 게이트웨이가 서비스하는 위치를 제어합니다: 바인드 주소와 포트, 외부에서 보이는 원본(origin), 그리고 선택적 TLS 종료입니다.

| 필드                     | 필수                 | 설명                                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | 아니오                | 바인드 주소입니다. 기본값 `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                         |
| `port`                 | 아니오                | 바인드 포트입니다. 기본값 `8080`.                                                                                                                                                                                                                                                                                                                                                                                            |
| `public_url`           | `host`가 루프백이 아닌 경우 | 외부에서 보이는 `https://` 원본으로, IdP `redirect_uri`와 검색 메타데이터를 구축하는 데 사용됩니다. `host`가 루프백 주소가 아닐 때마다 필수이며, TLS가 ALB, Ingress, Cloud Run 같은 프록시에서 종료되든 게이트웨이 자체에서 `tls`를 통해 종료되든 상관없습니다. 게이트웨이는 `X-Forwarded-*` 헤더에서 자신의 원본을 파생시키지 않기 때문입니다. 이들은 클라이언트가 스푸핑할 수 있습니다. 이것 없이는 부팅이 실패합니다. 아래의 `trusted_proxies`는 클라이언트 IP 해석만 제어합니다. 또한 [텔레메트리](#telemetry)를 활성화하려면 필수입니다. 게이트웨이가 클라이언트에 푸시하는 OTLP 엔드포인트를 이 URL에서 구축하기 때문입니다. |
| `tls.cert` / `tls.key` | 아니오                | 게이트웨이가 TLS를 자체적으로 종료하는 경우 PEM 경로                                                                                                                                                                                                                                                                                                                                                                                  |
| `trusted_proxies`      | 아니오                | 게이트웨이 앞의 로드 밸런서의 CIDR 또는 IP입니다. 설정되면 게이트웨이는 이 피어들로부터만 `X-Forwarded-For`를 신뢰하고 IP별 속도 제한 및 감사를 위해 실제 클라이언트 IP를 기록합니다. nginx `set_real_ip_from`과 동등합니다. `X-Forwarded-For` 항목이 `ipv4:port` 또는 `[ipv6]:port`로 작성되면(일부 로드 밸런서가 하는 것처럼) 포트가 제거된 상태로 읽혀집니다. 포트가 추가되고 괄호가 없는 IPv6 주소는 다른 주소로 읽히거나 전혀 읽히지 않을 수 있으므로, 해당 형식을 작성하는 프록시의 포트 옵션을 끕니다.                                                                          |

<h3 id="oidc">
  `oidc`
</h3>

`oidc` 블록은 게이트웨이를 ID 공급자에 연결하고 누가 로그인할 수 있는지 결정합니다. 발급자와 OAuth 클라이언트의 이름을 지정하고, 이메일과 그룹을 전달하는 클레임을 매핑하며, 이메일 도메인 또는 그룹별로 로그인을 제한합니다.

OpenID Connect(OIDC)는 게이트웨이가 ID 공급자와 함께 사용하는 SSO 프로토콜입니다. IdP 측에서 등록할 사항은 [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하세요.

| 필드                              | 필수  | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------- | --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | 예   | OIDC 검색 기본입니다. `/.well-known/openid-configuration`에서 검색을 제공해야 합니다. 프로덕션에서는 HTTPS를 사용하세요. 게이트웨이는 `http://` 발급자를 수락합니다. `http://localhost:8081` 같은 루프백 발급자는 [SSRF 가드](/docs/ko/claude-apps-gateway-deploy#threat-model-summary)에 의해 거부됩니다. `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`이 게이트웨이의 환경에 설정되어 있지 않은 경우입니다.                                                                                                                                                                                                                                                                                                                                             |
| `client_id` / `client_secret`   | 예   | OAuth 클라이언트 등록에서 가져옵니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowed_email_domains`         | 아니오 | `email` 클레임이 이 도메인 중 하나에 없는 id\_token을 거부합니다(대소문자 구분 안 함). 다중 테넌트 IdP 오구성에 대한 심층 방어입니다. 이 설정과 무관하게, `email_verified` 클레임이 명시적으로 `false`인 id\_token은 항상 거부됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `allowed_groups`                | 아니오 | 로그인을 이 IdP 그룹의 멤버로 제한하며, `groups_claim`에 대해 일치합니다. 허용된 이메일 도메인에 있지만 이 그룹 중 어느 것에도 없는 사용자는 거부됩니다. IdP가 그룹 클레임을 내보내야 합니다. 일치는 해당 클레임의 값에 대한 정확한 대소문자 구분 문자열 비교이며, 게이트웨이는 중첩된 그룹을 확장하지 않습니다. 하위 그룹의 멤버를 허용하려면 여기에 하위 그룹을 나열하거나 IdP를 구성하여 평탄화된 멤버십을 내보냅니다.                                                                                                                                                                                                                                                                                                                                                                                          |
| `groups_claim`                  | 아니오 | 어느 id\_token 클레임이 그룹 멤버십을 전달하는지입니다. 기본값 `groups`. Microsoft Entra는 `roles` 아래에 앱 역할을 내보냅니다. 평탄한 키 또는 `/resource_access/gateway/roles` 같은 RFC 6901 JSON 포인터를 중첩된 클레임에 대해 수락합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `google_groups`                 | 아니오 | Google Workspace Admin SDK Directory API를 통해 로그인한 사용자의 그룹을 조회합니다. Google의 id\_token은 그룹 클레임을 전달하지 않기 때문입니다. `service_account_json_path`를 `https://www.googleapis.com/auth/admin.directory.group.readonly` 범위에 대한 도메인 전체 위임이 있는 서비스 계정 키 파일로 설정하고, `admin_email`을 서비스 계정이 가장하는 Workspace 관리자로 설정합니다. Directory API는 실제 관리자 주체가 필요합니다. 각 사용자의 그룹 이메일 주소는 그룹 클레임이 되므로, `allowed_groups`와 `managed.policies.match.groups`는 그룹 이메일에 대해 일치합니다.                                                                                                                                                                                                        |
| `email_claim`                   | 아니오 | 어느 id\_token 클레임이 사용자의 이메일을 전달하는지입니다. 기본값 `email`. ADFS 및 Entra B2C 같은 일부 IdP는 대신 `upn` 또는 `preferred_username`을 내보냅니다. 평탄한 키, JSON 포인터, 또는 첫 번째 존재하는 키가 사용되는 폴백 키 목록을 수락합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `scopes`                        | 아니오 | 게이트웨이가 요청하는 OIDC 범위의 전체 재정의입니다. 기본값 `[openid, profile, email, offline_access]`. IdP가 인식하지 못하는 범위를 거부하거나 그룹 또는 이메일을 내보내기 위해 사용자 정의 범위가 필요한 경우 설정합니다. `openid`을 포함해야 합니다. `offline_access`를 제거하면 새로 고침 토큰이 비활성화되므로 개발자는 `session.ttl_hours`마다 브라우저 로그인을 다시 실행합니다. IdP별 범위 레시피(예: Google의 새로 고침 토큰 흐름)는 [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하세요.                                                                                                                                                                                                                                                                |
| `scope_on_refresh`              | 아니오 | 게이트웨이가 새로 고침 토큰을 교환할 때 로그인 요청과 동일한 목록으로 `scope`도 보냅니다. 기본값 `false`: 새로 고침 요청은 `scope`를 생략합니다. 대부분의 IdP는 모든 새로 고침에서 id\_token을 반환하고 이것이 필요하지 않습니다. IdP가 `openid`을 다시 요청할 때만 새로 고침 시 id\_token을 반환하는 경우 `true`로 설정합니다. Okta는 새로 고침 부여에 대해 이를 문서화합니다. id\_token이 없으면 모든 새로 고침은 IdP의 userinfo 엔드포인트가 새로 고쳐진 액세스 토큰을 수락하는 데 달려 있습니다. 로그인을 게이트하거나 그룹의 정책을 일치시키고 IdP의 새로 고침 시간 id\_token이 이들을 생략하는 경우, `userinfo_fallback: true`도 설정하여 게이트웨이가 userinfo 엔드포인트에서 이들을 채웁니다. 요청된 것보다 적은 범위를 부여한 IdP는 기존 세션에 대해서도 `invalid_scope`로 새로 고침을 거부할 수 있습니다. 이것을 설정한 후 새로 고침이 `token_endpoint`에서 실패하기 시작하면 키를 설정 해제합니다. 게이트웨이 서버에서 Claude Code v2.1.260 이상이 필요합니다. |
| `extra_auth_params`             | 아니오 | IdP 인증 요청에 그대로 추가되는 추가 쿼리 매개변수입니다. 이것은 Google 새로 고침 토큰의 `access_type: offline`, 일부 Entra 테넌트의 `domain_hint`, 또는 단계별 흐름의 `acr_values` 같은 IdP별 동작에 대한 재정의 메커니즘입니다. 게이트웨이 관리 프로토콜 매개변수는 재정의할 수 없습니다: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode`, 및 `client_id`.                                                                                                                                                                                                                                                                                                                                             |
| `userinfo_fallback`             | 아니오 | id\_token이 이메일 또는 그룹을 생략할 때 `/userinfo`에서 가져옵니다. Keycloak 경량 액세스 토큰, Okta org 서버, 및 ADFS 최소 토큰에 필요합니다. id\_token은 권한이 있으며, userinfo는 간격만 채웁니다. 기본값 `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `use_pkce`                      | 아니오 | 인증 요청에 PKCE(S256) 챌린지를 보냅니다. 기본값 `true`. IdP가 이 기밀 클라이언트에 대해 PKCE를 거부하는 경우에만 `false`로 설정합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `clock_skew_seconds`            | 아니오 | id\_token 시간 클레임을 검증할 때 클록 드리프트를 허용합니다. 기본값 `0`으로 엄격합니다. 로그인 직후 호스트/IdP 클록 스큐로 인해 "token expired / not yet valid" 오류가 표시되면 올립니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `token_endpoint_auth_method`    | 아니오 | 토큰 엔드포인트 인증 방법을 재정의합니다. `client_secret_basic` 또는 `client_secret_post`를 수락합니다. 기본적으로 자동 협상됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `id_token_signed_response_alg`  | 아니오 | 예상되는 id\_token 서명 알고리즘입니다. 기본값 `RS256`. ES256, PS256, 또는 EdDSA로 서명하는 IdP에 대해 설정합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `additional_authorized_parties` | 아니오 | `client_id` 이외에 수락할 추가 `azp` 값입니다. Keycloak 브로커 및 토큰 교환 흐름의 경우입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `discovery_url`                 | 아니오 | `issuer`에서 파생시키는 대신 이 URL에서 검색 문서를 가져옵니다. 발급자 호스트를 다시 작성하는 프록시 뒤의 IdP의 경우입니다. 경로는 `/.well-known/`을 포함해야 합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `use_proxy`                     | 아니오 | 게이트웨이의 자체 IdP 요청을 `HTTPS_PROXY` 또는 `HTTP_PROXY`의 정방향 프록시를 통해 보내고, `NO_PROXY`를 준수합니다. `false`로 설정하면 이 요청들은 직접 이동합니다. v2.1.227 이상이 필요합니다. 아래의 [정방향 프록시를 통한 IdP 요청](#idp-requests-through-a-forward-proxy)을 참조하세요.                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `form_action_origins`           | 아니오 | `/device` 페이지의 `Content-Security-Policy: form-action` 지시문에 대한 추가 원본입니다. 게이트웨이는 이미 `'self'`와 검색된 `authorization_endpoint` 원본을 허용하지만, Chrome은 전체 리디렉션 체인에 대해 `form-action`을 적용합니다. IdP가 Azure AD가 ADFS로 페더레이션되거나, 허브-스포크 Okta, 또는 회사 SSO 인터셉터 같은 두 번째 호스트를 통해 리디렉션하는 경우, 인증 요청이 리디렉션될 수 있는 모든 원본을 나열합니다.                                                                                                                                                                                                                                                                                                                                          |
| `ca_cert_pem`                   | 아니오 | PEM 인코딩된 CA 인증서 자체이며, 파일 경로가 아닙니다. IdP 요청에만 시스템 신뢰 저장소를 대체합니다. 마운트된 파일을 로드하려면 `${file:/etc/gateway/idp-ca.pem}`을 작성합니다. 회사 PKI 뒤의 Keycloak 또는 Dex에 사용합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

<h4 id="idp-requests-through-a-forward-proxy">
  정방향 프록시를 통한 IdP 요청
</h4>

추론 업스트림은 모든 버전에서 `HTTPS_PROXY` 및 `HTTP_PROXY`를 준수합니다. 게이트웨이의 자체 IdP, 검색, JWKS, 토큰, 및 userinfo 요청은 `oidc.use_proxy: true`를 설정하지 않는 한 직접 이동합니다. v2.1.227 이상이 필요합니다. 프록시 변수가 설정되고, `use_proxy`가 설정 해제되고, 발급자가 `NO_PROXY`로 적용되지 않으면, 게이트웨이는 이 요청들을 직접 유지하고 부팅 시 선택하도록 요청하는 공지를 기록합니다. `use_proxy: false`는 이들을 직접 유지하고 공지를 침묵시킵니다.

`use_proxy: true`를 사용하면, 포드는 각 IdP 엔드포인트의 호스트명을 자체적으로 해석하고 프록시에 해석된 IP 주소로 `CONNECT`하도록 요청합니다. 따라서 프록시는 발급자뿐만 아니라 검색 문서가 이름을 지정하는 모든 호스트의 IP 주소로 `CONNECT`를 수락해야 합니다. `http://` 프록시 URL을 사용합니다. `ca_cert_pem`과 [SSRF 가드](/docs/ko/claude-apps-gateway-deploy#threat-model-summary)는 프록시된 경로에도 적용됩니다.

[프록시 전용 이그레스](#proxy-only-egress)는 이 둘을 변경합니다: 활성화되는 동안, IdP 요청은 `use_proxy: false`를 설정하지 않는 한 프록시를 따르며, 게이트웨이는 먼저 해석하지 않고 프록시에 각 IdP 호스트명을 전달합니다.

<h4 id="proxy-only-egress">
  프록시 전용 이그레스
</h4>

게이트웨이의 환경에서 `HTTPS_PROXY` 옆에 `CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1`을 설정합니다. 포드가 해당 정방향 프록시를 통해서만 다른 호스트에 도달하고 공개 DNS 이름을 자체적으로 해석할 수 없거나, 프록시가 IP 주소로 `CONNECT`를 거부할 때입니다. v2.1.277 이상이 필요합니다. 이것은 `gateway.yaml` 키가 아닌 환경 변수이므로 구성 파일의 아무것도 게이트웨이의 주소 확인을 완화할 수 없습니다.

```bash theme={null}
export HTTPS_PROXY=http://proxy.corp.example.com:3128
export NO_PROXY=
export no_proxy=
export CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1
```

게이트웨이는 프록시 전용 이그레스가 활성화되는 동안 부팅 시 하나의 `network:` 줄을 기록합니다.

아래의 각 행은 `HTTPS_PROXY`가 설정된 게이트웨이의 아웃바운드 요청 클래스 하나이며, 기본적으로 그리고 프록시 전용 이그레스가 활성화되는 동안입니다.

| 아웃바운드 요청                                                                                                     | 기본값                                                                                                | 프록시 전용 이그레스 활성화                                                 |
| ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| `provider: anthropic` 업스트림, Workload Identity Federation 토큰 교환, `telemetry.forward_to` 내보내기                  | 로컬에서 해석되고 확인된 후 확인된 IP 주소로 프록시를 통해 `CONNECT`. `NO_PROXY`에 나열된 텔레메트리 수집기는 대신 직접 도달합니다.              | 프록시에 전달된 호스트명                                                   |
| IdP 검색, JWKS, 토큰, 및 userinfo                                                                                 | [`oidc.use_proxy: true`](#idp-requests-through-a-forward-proxy)가 아닌 한 직접, 그러면 확인된 IP 주소로 `CONNECT` | 호스트명이 프록시에 전달됩니다. `oidc.use_proxy: false`가 내부 IdP를 직접 유지하지 않는 한 |
| Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, 및 Microsoft Foundry 업스트림; Google 그룹 조회 | 호스트명이 프록시에 전달됩니다.                                                                                  | 변경되지 않음                                                         |

프록시 전용 이그레스는 게이트웨이의 환경이 이 세 가지 조건을 모두 충족하지 않는 한 꺼져 있습니다:

* `HTTPS_PROXY` 또는 `HTTP_PROXY`가 설정됩니다.
* `NO_PROXY` 및 `no_proxy`는 비어 있습니다. 플랫폼이 둘 중 하나를 포드에 주입하면, 게이트웨이 컨테이너에서 둘 다 빈 값으로 설정합니다. `NO_PROXY`에 텔레메트리 수집기를 나열하면 프록시 전용 이그레스가 꺼져 있습니다.
* `CLAUDE_GATEWAY_ALLOW_LOOPBACK`이 켜져 있지 않습니다. 포드의 자체 루프백의 수집기 또는 IdP는 프록시 전용 이그레스와 결합될 수 없습니다. 루프백 주소가 프록시에 전달되면 프록시 호스트 자신의 것이 되기 때문입니다. 대신 이 서비스들에 프록시가 도달할 수 있는 주소를 제공합니다. 같은 이유로 게이트웨이는 프록시 전용 이그레스가 활성화되는 동안 `localhost` 스타일 이름을 완전히 거부합니다.

이 조건 중 하나가 충족되지 않으면, 게이트웨이는 부팅 시 경고를 기록하고 이를 중지한 변수의 이름을 지정하며 기본 동작을 유지합니다.

프록시 전용 이그레스가 활성화되면, 내부 수집기 및 IP 주소로 구성된 모든 호스트를 포함하여 프록시의 모든 대상을 허용합니다. [`oidc.use_proxy: false`](#idp-requests-through-a-forward-proxy)로 내부 IdP를 직접 유지할 수 있습니다.

<Warning>
  프록시의 허용 목록이 게이트웨이의 자체 확인만큼 엄격할 때만 이것을 켭니다. 프록시는 `169.254.169.254` 및 `metadata.google.internal` 같은 클라우드 메타데이터 엔드포인트, 링크 로컬 주소, 및 프록시 호스트의 자체 루프백을 거부해야 하며, 이름뿐만 아니라 이름이 해석되는 주소로 이들을 거부해야 합니다. 게이트웨이는 더 이상 호스트명이 이들 중 하나로 해석되는 것을 잡지 않기 때문입니다. 요청한 곳 어디든 연결하는 프록시는 이 요청들에 대해 게이트웨이의 [SSRF 가드](/docs/ko/claude-apps-gateway-deploy#threat-model-summary)를 제거합니다.
</Warning>

<h3 id="session">
  `session`
</h3>

`session` 블록은 게이트웨이가 로그인 후 발행하는 베어러 토큰을 형성합니다: 이들에 서명하는 비밀과 얼마나 오래 살아있는지입니다.

| 필드           | 필수  | 설명                                                                                                                                                                                                                                                                   |
| ------------ | --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | 예   | 최소 32바이트의 엔트로피입니다. 예를 들어 `openssl rand -base64 32`에서 가져옵니다. 게이트웨이의 HS256 베어러 토큰에 서명합니다. 단일 문자열 또는 회전을 위한 배열을 수락합니다: 인덱스 0이 서명하고 모든 항목이 검증합니다. 회전하려면 새 비밀을 앞에 추가하고, `ttl_hours`를 기다린 후, 이전 항목을 제거합니다.                                                                 |
| `ttl_hours`  | 아니오 | 게이트웨이 베어러 토큰 수명입니다. 기본값 `1`. CLI는 IdP가 새로 고침 토큰을 발행할 때 만료 전에 자동으로 새로 고칩니다. 더 짧은 수명은 더 빠르게 프로비저닝을 해제합니다. 더 긴 수명은 더 적은 IdP 왕복을 만듭니다. IdP가 `offline_access`를 사용할 수 없기 때문에 새로 고침 토큰을 발행할 수 없으면, 자동 새로 고침이 없으므로 개발자가 매시간 브라우저 로그인으로 돌아가는 것을 피하기 위해 이것을 `8` 또는 `12`로 올립니다. |

<h3 id="store">
  `store`
</h3>

`store` 블록은 게이트웨이를 PostgreSQL 데이터베이스로 지정합니다. 이는 장치 부여 및 속도 제한 카운터를 보유합니다.

| 필드                        | 필수  | 설명                                                                                                                                                                                                                                                                                                 |
| ------------------------- | --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`            | 예   | `postgres://` 또는 `postgresql://` URL입니다. 필수: 장치 부여 랑데부(브라우저 콜백이 작성하고 폴링 CLI가 읽는)는 교차 복제본 상태가 필요합니다. 게이트웨이는 부팅 및 업그레이드 시 자체 스키마 마이그레이션을 실행하므로, 역할은 대상 스키마에서 테이블을 생성하고 변경할 권리가 필요합니다. [업그레이드](/docs/ko/claude-apps-gateway-deploy#upgrades) 및 [Postgres](/docs/ko/claude-apps-gateway-deploy#postgres)를 참조하세요. |
| `username`                | 아니오 | `postgres_url`의 사용자를 재정의합니다.                                                                                                                                                                                                                                                                       |
| `password`                | 아니오 | 데이터베이스 자격증명입니다. 자격증명이 URL에서 벗어나도록 `postgres_url`이 아닌 여기에 설정합니다. 모든 문자를 수락하고 URL 자격증명보다 우선합니다.                                                                                                                                                                                                      |
| `max_connections`         | 아니오 | 복제본당 Postgres 연결 풀 크기입니다. 기본값 `5`로 보수적이고 공유 데이터베이스에 친화적입니다. [지출 제한](#admin)이 활성화되면, 핫 경로는 추론 요청당 몇 가지 작업을 수행하므로, 로드 아래의 전용 데이터베이스에 대해 이것을 올리고, 복제본 × 이것을 데이터베이스의 `max_connections` 아래로 유지합니다.                                                                                                      |
| `connect_timeout_seconds` | 아니오 | 게이트웨이가 Postgres 연결을 열 때 대기하는 초입니다. `1`에서 `60` 사이의 정수이며, 기본값 `5`입니다. 새 게이트웨이 인스턴스가 시작될 때 연결 시도가 시간 초과되면 올립니다. 게이트웨이 서버에서 Claude Code v2.1.274 이상이 필요합니다. 이전 버전은 키가 설정되어 있을 때 시작을 거부합니다.                                                                                                             |

로컬 개발의 경우, `postgres_url`을 일회용 Postgres 컨테이너로 지정합니다. 예를 들어 `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams`는 정렬된 목록입니다. 게이트웨이는 요청된 모델을 해석하는 첫 번째 업스트림으로 추론을 전달합니다.

`5xx`, `429`, `401`, `403`, `404`, 또는 타임아웃에서 게이트웨이는 다음 업스트림으로 장애 조치합니다. 다른 `4xx`는 장애 조치하지 않습니다. 이 오류들은 업스트림이 아닌 요청에 기인하기 때문입니다. `401` 또는 `403`은 게이트웨이의 자체 자격증명이 해당 업스트림에 대해 실패했음을 의미합니다. `404`는 해당 업스트림이 요청된 모델을 제공하지 않으므로, 목록의 나중 업스트림이 여전히 할 수 있음을 의미합니다.

업스트림에서 `forward_user_identity: true`를 설정하면, 개발자의 이메일을 전달한 요청에 반환하는 `429`는 장애 조치하지 않습니다. [개발자에게 도달하는 사용자별 제한 거부](#per-user-identity-headers-for-a-proxy-you-run)를 참조하세요.

`404`에서의 장애 조치는 게이트웨이 v2.1.198 이상이 필요합니다. 이전 릴리스는 목록의 나중 업스트림이 모델을 제공할 때도 첫 번째 `404`를 클라이언트에 반환했습니다.

동일한 공급자의 여러 업스트림은 고유한 `name:`을 설정해야 합니다.

Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, 및 Microsoft Foundry 클라이언트는 시작 시 한 번 구축되며, 이들의 SDK는 자격증명을 내부적으로 새로 고치므로, 클라우드 자격증명을 회전해도 재시작이 필요하지 않습니다. 정적 Anthropic API 키 및 베어러는 시작 시 읽혀집니다. [Anthropic API](#anthropic-api)를 참조하세요.

<h4 id="upstream-error-messages">
  업스트림 오류 메시지
</h4>

게이트웨이는 업스트림이 응답한 방식에 따라 한 업스트림의 오류 응답 또는 자체 `502`를 반환합니다:

* **업스트림이 게이트웨이가 [장애 조치](#multiple-upstreams)하지 않는 상태를 반환했습니다**: 해당 업스트림의 응답입니다. 게이트웨이는 추가 업스트림을 시도하지 않습니다.
* **게이트웨이가 시도한 모든 업스트림이 [장애 조치](#multiple-upstreams)하는 방식으로 실패했습니다**: 마지막 `429`. 아무도 `429`를 반환하지 않으면, 게이트웨이는 순서대로 마지막 `401` 또는 `403`, 마지막 `404`, 및 마지막 `501`을 선호합니다. 아무도 이들 중 어느 것도 반환하지 않으면, 게이트웨이의 자체 `502`, `all upstreams failed (N attempted)`. N은 게이트웨이가 요청된 모델을 제공하지 않기 때문에 건너뛴 항목을 포함하여 [`upstreams`](#upstreams)의 모든 항목을 계산합니다.

게이트웨이가 업스트림의 응답을 반환할 때, 업스트림의 상태 코드를 유지합니다. 업스트림의 메시지를 유지하는지 여부는 공급자에 따라 다릅니다. Anthropic API 업스트림의 오류 본문은 개발자에게 변경되지 않은 상태로 도달합니다.

Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, 및 Microsoft Foundry 업스트림은 오류 텍스트에서 계정 ID, 역할 ARN, 및 프로젝트 ID의 이름을 지정할 수 있습니다. 게이트웨이는 해당 전체 텍스트를 [운영 로그](/docs/ko/claude-apps-gateway-deploy#logs)에 기록합니다. 개발자가 이 업스트림들에서 보는 것은 거부에 따라 다릅니다:

* Anthropic의 표준 오류 봉투의 `400` 또는 `413`: `prompt is too long` 같은 업스트림의 자체 메시지입니다. Claude Platform on AWS, Agent Platform, 및 Microsoft Foundry는 모델 API 거부에 대해 이 봉투를 반환합니다.
* 공급자의 자체 형태의 `400` 또는 `413`: `capability_rejected:` 토큰입니다. 게이트웨이가 거부를 분류할 수 없으면, `400`에서 `upstream rejected the request` 또는 `413`에서 `request too large for this upstream`.
* 다른 상태: `429`에서 `upstream rate limit exceeded` 같은 상태별 일반 복사입니다.

예를 들어, 게이트웨이는 Amazon Bedrock의 `Input is too long for requested model.`을 `capability_rejected: prompt_too_long`으로 대체합니다. Claude Code는 `prompt is too long`에서처럼 해당 토큰에서 [자동으로 압축합니다](/docs/ko/errors#prompt-is-too-long).

클라우드 업스트림의 `400` 또는 `413` 메시지를 유지하거나 `capability_rejected:` 토큰으로 대체하려면 게이트웨이 v2.1.233 이상이 필요합니다.

<h4 id="anthropic-api">
  Anthropic API
</h4>

최소 Anthropic 업스트림은 [Claude Console](https://platform.claude.com)의 API 키입니다:

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # OR an OAuth bearer (e.g. a Workload-Identity-Federation-exchanged token):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # default; override for a forward proxy
```

두 자격증명 형식은 전송하는 헤더에서 다릅니다:

* **`api_key`**: `x-api-key`를 보냅니다. Claude Console에서 회전하고 환경 변수를 업데이트합니다.
* **`oauth_token`**: `Authorization: Bearer`를 보냅니다. 조직이 장기 API 키 대신 단기 토큰을 발행할 때 베어러 형식을 사용합니다. 베어러는 시작 시 한 번 읽혀지므로, 비밀을 다시 마운트하고 재시작하여 새로 고칩니다.

정적 키 또는 베어러 대신, Workload Identity Federation을 사용할 수 있습니다. [Workload Identity Federation 가이드](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)를 따라 페더레이션 규칙을 생성한 후, 워크로드의 OIDC JWT를 파일로 마운트합니다. 예를 들어 Kubernetes 프로젝션된 서비스 계정 토큰 또는 CI 플랫폼의 id-token입니다. 게이트웨이는 JWT를 단기 베어러로 교환하고 자동으로 새로 고칩니다. 토큰 파일은 모든 교환에서 다시 읽혀지므로, 회전된 프로젝션된 토큰은 재시작 없이 선택됩니다.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # required if the rule covers >1 workspace
      # service_account_id: svac_...   # optional expected-target check
```

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  사용자가 실행하는 프록시에 대한 사용자별 ID 헤더
</h5>

`provider: anthropic` 업스트림의 `base_url`을 Anthropic API 대신 실행하는 프록시로 지정할 수 있습니다. 해당 프록시에 각 요청을 보낸 개발자를 알리려면, 해당 업스트림에서 `forward_user_identity: true`를 설정합니다. 그러면 프록시는 개발자별로 지출을 속성화할 수 있습니다. 게이트웨이 서버에서 Claude Code v2.1.233 이상이 필요합니다.

예를 들어, `upstream-gateway.internal.example.com`의 프록시의 경우:

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # default false
```

게이트웨이는 해당 업스트림으로 전달하는 모든 요청에 이 헤더들을 추가합니다.

| 헤더                            | 값                               |
| ----------------------------- | ------------------------------- |
| `x-litellm-end-user-id`       | IdP가 제공한 경우 개발자의 이메일입니다.        |
| `x-claude-gateway-user-id`    | 토큰의 `sub` 클레임에서 개발자의 IdP 주체입니다. |
| `x-claude-gateway-user-email` | IdP가 제공한 경우 개발자의 이메일입니다.        |

IdP 토큰이 이메일을 전달하지 않으면, 게이트웨이는 `x-claude-gateway-user-id`만 보내고 두 이메일 헤더를 생략합니다. IdP가 이메일을 다른 클레임에 넣으면, [`oidc.email_claim`](#oidc)을 해당 클레임으로 설정합니다.

프록시가 개발자의 이메일을 전달한 요청에 `429`로 응답하면, 게이트웨이는 해당 응답을 개발자에게 그대로 반환하고 다음 업스트림으로 장애 조치하지 않으므로, 프록시의 사용자별 예산 또는 속도 제한이 유지됩니다. 프록시의 다른 응답은 일반적인 [장애 조치 규칙](#multiple-upstreams)을 따릅니다. 개발자의 IdP 토큰이 이메일을 전달하지 않으면, 게이트웨이는 이메일 헤더 없이 요청을 전달하므로, 이 요청 중 하나에 대한 `429`는 업스트림 용량으로 계산되고 장애 조치합니다. 게이트웨이 서버에서 v2.1.267 이전에는 모든 `429`가 장애 조치했습니다.

`forward_user_identity`를 운영하는 프록시인 업스트림의 `base_url`에만 설정합니다. 게이트웨이는 개발자 이메일을 해당 `base_url`이 이름을 지정하는 모든 서버로 보냅니다. `base_url`이 기본값인 Anthropic API인 경우, 게이트웨이는 시작을 거부합니다.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

클라이언트 측 Amazon Bedrock 배포(게이트웨이가 대체하거나 앞에 있는)의 경우, [Amazon Bedrock의 Claude Code](/docs/ko/amazon-bedrock)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # preferred: AWS default credential chain
    # OR explicit credentials:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # OR a Bedrock API bearer token:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Override the bedrock-runtime endpoint for FIPS or VPC-endpoint deployments:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

빈 `auth` 블록은 AWS SDK의 기본 자격증명 체인을 사용합니다: 환경 변수, `~/.aws/credentials`, ECS 작업 역할, EC2 인스턴스 메타데이터, 또는 EKS의 IRSA입니다. 프로덕션에서는 컨테이너 이미지에 정적 키를 포함하는 대신 게이트웨이 포드에 IAM 역할을 제공합니다.

명시적 자격증명은 완전해야 합니다: `aws_access_key_id`와 `aws_secret_access_key`가 함께 설정되지 않거나 `aws_session_token`이 이들 없이 설정되면 게이트웨이는 부팅 시 실패합니다. v2.1.207 이전에는 부분 `auth:` 블록이 검증을 통과했습니다.

| 설정        | 방법                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 권한    | 게이트웨이의 주체에 추론 프로필 ARN과 기본 기초 모델 ARN 모두에 `bedrock:InvokeModel` 및 `bedrock:InvokeModelWithResponseStream`을 부여합니다. US 지역의 기본 제공 카탈로그의 경우: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` 및 `arn:aws:bedrock:*::foundation-model/anthropic.*`. 또한 기초 모델 ARN에 `bedrock:CountTokens`를 부여합니다. 게이트웨이는 이를 사용하여 클라이언트가 포기한 요청의 입력 토큰을 계산하므로, [지출 제한](#admin)이 정확하게 유지됩니다. 이것 없이 게이트웨이는 해당 계산을 위해 일회용 Bedrock 요청으로 폴백합니다. |
| 모델 액세스    | Amazon Bedrock은 상용 지역에서 기본적으로 모델 액세스를 활성화합니다. 남은 계정 수준 게이트는 Anthropic의 일회용 사용 사례 양식입니다: AWS 계정의 누구도 제출하지 않았으면, Amazon Bedrock 콘솔을 열고, 모델 카탈로그에서 Anthropic 모델을 선택하고, 양식을 완료합니다. AWS Organizations 양식 및 제출자가 필요한 권한은 [사용 사례 세부 정보 제출](/docs/ko/amazon-bedrock#1-submit-use-case-details)을 참조하세요.                                                                                                                                             |
| EKS(IRSA) | 위의 정책과 클러스터의 OIDC 공급자로 범위가 지정된 신뢰 정책이 있는 IAM 역할을 생성합니다. 게이트웨이의 서비스 계정에 `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`로 주석을 답니다. `auth: {}`가 이를 선택합니다.                                                                                                                                                                                                                                                          |
| ECS / EC2 | IAM 역할을 작업 정의 또는 인스턴스 프로필에 연결합니다. `auth: {}`가 이를 선택합니다.                                                                                                                                                                                                                                                                                                                                                                               |
| 다른 곳      | `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, 및 `AWS_SESSION_TOKEN` 환경 변수를 통해 자격증명을 전달하거나, `${VAR}` 확장으로 `auth:`에서 명시적으로 설정합니다.                                                                                                                                                                                                                                                                                                       |
| 지역        | `region:`은 API 엔드포인트 지역입니다. 교차 지역 추론 프로필은 선택한 것과 무관하게 지역(US, EU, APAC)을 통해 라우팅합니다. 비 US 지역 또는 프로비저닝된 처리량 ARN의 경우, 올바른 업스트림별 ID가 있는 [`models:`](#models) 블록을 추가합니다.                                                                                                                                                                                                                                                                    |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS는 `aws-external-anthropic.<region>.api.aws`에서 AWS 인프라의 일차 Anthropic API를 제공합니다. 일차 모델 ID를 사용하고, 전송된 대로 `anthropic-beta` 헤더를 준수하며, `count_tokens`를 제공하므로, Bedrock별 번역이 적용되지 않습니다. `anthropicAws` 공급자는 Claude Code v2.1.198 이상이 필요합니다. 이전 게이트웨이 릴리스는 부팅 시 이를 거부합니다.

동일한 플랫폼의 클라이언트 측 배포의 경우, [Claude Platform on AWS의 Claude Code](/docs/ko/claude-platform-on-aws)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # sent as x-api-key
    # OR SigV4 via the AWS default credential chain:
    # auth: {}
    # OR explicit SigV4 credentials:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Override the derived endpoint:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

플랫폼은 Amazon Bedrock과 별도의 AWS 계정에서 실행되고 자체 서비스 이름 `aws-external-anthropic`에 대해 SigV4 요청에 서명하므로, Bedrock 범위 IAM 역할은 이를 인증하지 않습니다. `auth.api_key`의 API 키는 SigV4 자격증명도 설정되어 있을 때 우선합니다. 빈 `auth` 블록은 AWS SDK의 기본 자격증명 체인을 사용합니다. [Amazon Bedrock](#amazon-bedrock) 업스트림이 사용하는 동일한 체인입니다.

| 필드                                                      | 필수  | 설명                                                                                                    |
| ------------------------------------------------------- | --- | ----------------------------------------------------------------------------------------------------- |
| `region`                                                | 예   | AWS 지역으로, 소문자, 숫자, 및 하이픈입니다. 게이트웨이는 `https://aws-external-anthropic.<region>.api.aws`로 엔드포인트를 파생시킵니다. |
| `workspace_id`                                          | 예   | 모든 요청에서 헤더로 전송됩니다. 플랫폼이 필요합니다.                                                                        |
| `auth.api_key`                                          | 아니오 | 플랫폼의 API 키로, `x-api-key`로 전송됩니다. 베어러 토큰이 아닙니다: 두 인증 모드는 API 키 또는 SigV4입니다.                            |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | 아니오 | 명시적 SigV4 자격증명입니다. 하나를 다른 것 없이 설정하면 부팅 시 실패합니다. `auth.aws_session_token`은 이들과 함께 수락됩니다.               |
| `base_url`                                              | 아니오 | 파생된 엔드포인트를 재정의합니다.                                                                                    |

플랫폼이 일차 모델 ID를 해석하므로, 기본 제공 카탈로그는 [`models:`](#models) 블록 없이 이로 라우팅합니다. `models:` 목록을 큐레이션할 때, 항목을 `anthropicAws:`로 일차 ID로 키합니다.

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

동등한 클라이언트 측 설정의 경우, [Google Cloud의 Claude Code](/docs/ko/google-vertex-ai)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # preferred: Application Default Credentials
    # OR a service account key file:
    # auth: { service_account_json: /secrets/sa.json }
    # Override the aiplatform endpoint for Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

빈 `auth` 블록은 Application Default Credentials를 사용합니다: `GOOGLE_APPLICATION_CREDENTIALS`, GCE 메타데이터, 또는 GKE Workload Identity입니다. 서비스 계정 JSON 키 파일은 지원되지만 권장되지 않습니다. Workload Identity를 사용하거나 GCE 또는 Cloud Run 인스턴스에 서비스 계정을 연결합니다.

Google Cloud의 Agent Platform에 대해 [글로벌 엔드포인트](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)를 사용하려면 `region: global`을 설정합니다. Google은 각 요청을 사용 가능한 지역으로 라우팅하므로, 지역별 모델 가용성을 추적할 필요가 없습니다. 특정 지역을 설정하면 모든 요청을 이에 고정합니다.

| 설정                     | 방법                                                                                                                                                                               |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 권한                 | 게이트웨이의 서비스 계정에 프로젝트에 `roles/aiplatform.user`를 부여하거나, `aiplatform.endpoints.predict`가 있는 사용자 정의 역할을 부여합니다. Google Cloud의 Agent Platform API(`aiplatform.googleapis.com`)를 활성화합니다. |
| 모델 액세스                 | Model Garden에서 프로젝트에 대해 Claude 모델을 활성화합니다. 이들은 특정 지역에 게시됩니다. 지원되는 지역은 모델 카드를 확인합니다.                                                                                              |
| GKE(Workload Identity) | GCP 서비스 계정을 게이트웨이의 Kubernetes 서비스 계정에 바인드하고 KSA에 `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`으로 주석을 답니다. `auth: {}`가 이를 선택합니다.                |
| Cloud Run / GCE        | 서비스의 서비스 계정을 `roles/aiplatform.user`가 있는 것으로 설정합니다. `auth: {}`가 이를 선택합니다.                                                                                                        |
| 다른 곳                   | `auth: { service_account_json: /secrets/sa.json }`. JSON 키 파일로 마운트된 비밀의 경로입니다. 필드는 파일 경로를 취하고, 키 내용이 아니므로, `${file:…}` 확장이 관련되지 않습니다.                                            |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

클라이언트 측 Microsoft Foundry 배포의 경우, [Microsoft Foundry의 Claude Code](/docs/ko/microsoft-foundry)를 참조하세요. 게이트웨이 측 업스트림:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # preferred: DefaultAzureCredential / Managed Identity
    # OR an API key:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true`는 `DefaultAzureCredential`을 통해 해석합니다: AKS, ACI, 또는 App Service의 Managed Identity, Azure CLI, 또는 환경 자격증명입니다. API 키는 작동하지만 프로젝트 전체이고 자동으로 회전하지 않습니다. Microsoft Foundry의 엔드포인트는 `resource:`에서 파생됩니다. Azure Government 같은 소주권 클라우드에 대해 선택적 `base_url`을 설정하여 재정의합니다.

| 설정                | 방법                                                                                                                                       |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC              | 게이트웨이의 ID에 Microsoft Foundry 리소스에 `Azure AI User` 또는 `Cognitive Services User`를 부여합니다.                                                   |
| 배포                | Microsoft Foundry는 정규 모델 ID가 아닌 관리자 선택 배포 이름을 사용합니다. 각 정규 ID를 배포 이름으로 매핑하는 [`models:`](#models) 블록을 추가합니다.                               |
| AKS(워크로드 ID)      | 클러스터의 OIDC 발급자와 사용자 할당 Managed Identity를 페더레이션하고 게이트웨이의 서비스 계정에 바인드합니다. `use_azure_ad: true`는 `WorkloadIdentityCredential`을 통해 이를 선택합니다. |
| ACI / App Service | 리소스에서 시스템 할당 또는 사용자 할당 관리 ID를 활성화합니다. `use_azure_ad: true`는 이를 선택합니다.                                                                    |
| 다른 곳              | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. `{ }` 내에서 `${…}`를 인용합니다.                                                                      |

<h4 id="static-headers-on-upstream-requests">
  업스트림의 정적 헤더
</h4>

게이트웨이가 한 업스트림으로 보내는 요청에 고정 헤더를 추가하려면, 해당 업스트림에서 `headers:`를 설정합니다. 실행하는 프록시가 헤더로 트래픽을 라우팅하거나 속성화할 때 사용합니다.

`headers:`는 게이트웨이 서버에서 Claude Code v2.1.277 이상이 필요합니다. 이전 게이트웨이는 키를 찾으면 시작을 거부합니다. 키를 추가하기 전에 모든 복제본을 업그레이드하고, 이전 버전으로 롤백하기 전에 키를 제거합니다.

헤더는 `base_url`이 이름을 지정하는 서버로 이동하거나, `base_url`이 설정 해제되어 있을 때 공급자의 자체 엔드포인트로 이동합니다. 공급자는 프록시가 제거하지 않는 한 이들도 받습니다.

이 예제는 `upstream-proxy.internal.example.com`의 프록시를 통해 `provider: vertex` 업스트림에 도달합니다. 프록시가 읽는 `x-source` 헤더를 설정하고, `PROXY_TOKEN` 환경 변수의 토큰을 `x-proxy-token`으로 보냅니다:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    base_url: https://upstream-proxy.internal.example.com
    auth: {}
    headers:
      x-source: claude-apps-gateway
      x-proxy-token: ${PROXY_TOKEN}
```

값은 양쪽 끝에 공백이 없는 인쇄 가능한 ASCII 텍스트입니다. 숫자, `true`, 또는 `false`를 인용하여 YAML이 이를 텍스트로 읽도록 합니다.

비밀을 구성 파일에서 벗어나도록 유지하려면, [비밀 확장](#secret-expansion)을 사용하여 `${VAR}`로 환경 변수에서 또는 `${file:/path}`로 파일에서 값을 로드합니다. 빈 값으로 해석되는 `${VAR}`은 게이트웨이가 시작되는 것을 중지합니다.

`headers:`는 모든 공급자에서 작동하며, 각 업스트림은 자신의 것만 보냅니다.

게이트웨이가 업스트림으로 보내는 모든 요청이 이들을 전달하지는 않습니다:

| 게이트웨이가 이 업스트림으로 보내는 요청                                    | `headers:` 전달          |
| --------------------------------------------------------- | ---------------------- |
| `/v1/messages`, 스트리밍 또는 아님, 및 `/v1/messages/count_tokens` | 예                      |
| 다른 업스트림에서 장애 조치된 요청                                       | 예, 이 업스트림의 `headers:`만 |
| 클라이언트가 포기한 요청에 대한 Amazon Bedrock의 `CountTokens` 호출        | 아니오                    |
| Workload Identity Federation 토큰 교환                        | 아니오                    |

AWS SigV4로 요청에 서명하는 Amazon Bedrock 또는 Claude Platform on AWS 업스트림에서, 이 헤더들은 서명의 일부이므로, 프록시는 이들을 변경되지 않은 상태로 통과시켜야 합니다.

게이트웨이가 예약한 이름을 사용하면, 시작을 거부하고 시작 오류가 헤더의 이름을 지정합니다. 예약된 이름은 다음을 포함합니다:

* `authorization` 및 `x-api-key`
* `host`, `content-type`, 및 `user-agent`
* `anthropic-`, `x-goog-`, `x-amz-`, 또는 `x-amzn-`으로 시작하는 모든 이름

<h4 id="multiple-upstreams">
  여러 업스트림
</h4>

동일한 공급자는 고유한 `name:`으로 두 번 이상 나타날 수 있습니다. 이는 다양한 지역, 다양한 자격증명 체인을 통한 다양한 계정, 프로비저닝된 처리량 대 온디맨드, 및 교차 공급자 폴백을 다룹니다.

게이트웨이는 순서대로 업스트림을 시도합니다. `5xx`, `429`, `401`, `403`, `404`, 타임아웃, 및 누락된 엔드포인트(`501`)는 장애 조치합니다. 다른 `4xx`는 장애 조치하지 않습니다.

`429`는 업스트림별 용량이므로, 프로비저닝된 처리량(PT) 소진은 온디맨드로 장애 조치합니다. 업스트림에서 [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run)를 설정하면, 개발자의 이메일을 전달한 요청에 대한 `429`는 사용자별 거부이고 장애 조치하지 않습니다.

모든 요청은 첫 번째 업스트림에서 시작합니다. 요청은 앞의 모든 업스트림이 실패했거나 요청된 모델을 제공하지 않을 때만 나중 업스트림에 도달합니다.

게이트웨이는 실패한 업스트림의 기록을 유지하지 않으므로, 업스트림이 다운되는 동안, 이에 도달하는 모든 요청은 여전히 이를 시도하고 실패할 때까지 기다립니다.

Anthropic API 업스트림의 경우, [`timeouts.upstream_ttfb_ms`](#http-tuning)는 다운된 업스트림에서의 대기를 제한합니다. 이 설정은 다른 공급자에게 적용되지 않으며, 게이트웨이는 업스트림이 응답하기 시작할 때까지 최대 1시간을 기다립니다.

`404`는 업스트림별 모델 가용성이므로, 모델을 활성화하지 않은 업스트림은 이를 제공하는 나중 업스트림을 차단하지 않습니다. 요청된 모델을 해석할 수 없는 업스트림은 네트워크 왕복 없이 건너뜁니다.

이 예제는 프로비저닝된 처리량 Amazon Bedrock 할당을 먼저 라우팅하고, 온디맨드 및 두 번째 계정으로 오버플로우하며, 마지막으로 Anthropic API로 폴백합니다:

```yaml theme={null}
upstreams:
  # Primary: provisioned throughput in your home region.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Overflow: on-demand cross-region.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Different account: a separate Bedrock allotment via assumed-role creds.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Last resort: direct Anthropic API.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Per-upstream model IDs are keyed on the upstream's `name:`.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| 레버               | 방법                                                                                                                                                                                                                                                                           |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 다양한 지역           | 지역당 하나의 Amazon Bedrock 업스트림으로, 각각 자체 `region:`을 가집니다. [`auto_include_builtin_models: true`](#models)를 사용하면 교차 지역 추론 프로필이 자동으로 라우팅됩니다. 지역 고정 배포의 경우 `models:` 블록을 사용합니다.                                                                                                      |
| 다양한 계정           | 계정당 하나의 Amazon Bedrock 업스트림으로, 각각 `auth:`에서 자체 자격증명을 가집니다. 기본 체인(`auth: {}`)은 포드의 ID를 사용합니다. 두 번째 계정의 경우, 명시적 자격증명 또는 베어러 토큰을 설정합니다.                                                                                                                                         |
| 프로비저닝된 처리량       | 해당 업스트림의 이름에 대해 `models:`의 프로비저닝된 처리량 ARN으로 모델을 매핑합니다. 다른 업스트림은 온디맨드 ID를 유지하므로, PT 용량이 폴백 전에 소진됩니다.                                                                                                                                                                          |
| VPC / FIPS 엔드포인트 | 업스트림의 `base_url:`을 VPC 엔드포인트 또는 FIPS 엔드포인트 URL로 설정합니다.                                                                                                                                                                                                                       |
| 모델 범위 라우팅        | 기본 제공 Claude 모델이 아닌 사용자 정의 모델 `id`만 `upstream_model:` 맵에서 부재한 업스트림을 건너뜁니다. 게이트웨이는 순서대로 모든 업스트림에서 기본 제공 모델을 시도하고 맵에 항목이 없는 경우 공급자의 기본 ID를 사용하므로, 기본 제공 모델의 경우 맵은 업스트림이 시도되는지 여부가 아닌 업스트림이 받는 ID를 변경합니다. ID를 거부하는 업스트림은 다른 업스트림 오류와 동일한 [장애 조치 규칙](#multiple-upstreams)을 따릅니다. |

클라우드 공급자 간 또는 직접 Anthropic API로 장애 조치하면 요청을 관리하는 계약, 지역, 및 기타 약관이 변경됩니다.

CLI는 주어진 요청을 제공하는 업스트림과 무관하게 게이트웨이에 동일한 기능 게이팅을 적용하므로, 장애 조치는 업스트림이 거부할 본문 필드를 보내지 않습니다.

<h2 id="optional-sections">
  선택적 섹션
</h2>

<h3 id="admin">
  `admin`
</h3>

선택적입니다. `/v1/organizations/spend_limits`를 활성화하며, 이는 Anthropic의 공개 Admin API를 미러링하고 `/v1/messages`에서 개발자별 지출 강제를 수행합니다. [지출 한도](/docs/ko/claude-apps-gateway-spend-limits)에서 상한이 어떻게 설정되고 강제되는지 확인하세요. 이 섹션은 기능을 켜고 조정하는 `gateway.yaml` 키를 다룹니다.

```yaml theme={null}
admin:
  # Named static API keys for the admin endpoints, sent as x-api-key.
  # The id appears in the audit log as admin-key:<id> so each key is
  # attributable. Array for rotation: add the new key, roll clients,
  # remove the old.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # IdP groups granted full admin via the normal gateway JWT (no API key).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| 필드                        | 필수  | 설명                                                                                                                                                                                                                                                              |
| ------------------------- | --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | 아니요 | `{id, key}` 배열입니다. 이 중 하나와 일치하는 `x-api-key`는 지출 한도를 나열, 설정 및 삭제할 수 있습니다. 키 값은 최소 32자 이상이어야 하며, `id`는 `read_keys`와 `write_keys` 전체에서 고유해야 합니다.                                                                                                                   |
| `read_keys`               | 아니요 | `{id, key}` 배열입니다. 읽기 전용: 상한 나열, ID로 하나 가져오기, [`/effective`](/docs/ko/claude-apps-gateway-spend-limits#%2Feffective) 및 [`/audit`](/docs/ko/claude-apps-gateway-spend-limits#%2Faudit) 읽기를 포함한 모든 `GET` 엔드포인트입니다.                                                          |
| `admin_groups`            | 아니요 | IdP 그룹 이름입니다. `groups` 클레임이 이 중 하나를 포함하는 gateway JWT는 전체 관리자 액세스(읽기 및 쓰기)를 가지며 `oidc:<sub>`로 감사됩니다. 인간 관리자에게는 이를 사용하고, 머신에는 API 키를 사용하세요. 이 목록의 빈 항목은 부팅 시 gateway를 중지합니다. [gateway 부팅 시 중지되는 Matcher 값](#matcher-values-that-stop-the-gateway-at-boot)을 참조하세요. |
| `blocked_message`         | 아니요 | 차단된 개발자가 보는 `429 billing_error`에 그대로 추가됩니다. URL이나 Slack 채널과 같은 전체 지시사항을 작성하세요. 설정하지 않으면 gateway는 기본 메시지만 보냅니다. [강제 작동 방식](/docs/ko/claude-apps-gateway-spend-limits#how-enforcement-works)을 참조하세요.                                                                   |
| `audit_retention_days`    | 아니요 | 기본값 `365`입니다. 더 오래된 `admin_audit` 행은 정리됩니다.                                                                                                                                                                                                                     |
| `spend_retention_months`  | 아니요 | 기본값 `13`입니다. 이보다 오래된 `spend` 카운터 행은 정리됩니다. 기본값은 연간 비교 보고를 위해 전체 연도와 현재 부분 월을 유지합니다.                                                                                                                                                                             |
| `identity_retention_days` | 아니요 | 기본값 `90`입니다. 각 개발자의 이메일, 표시 이름 및 그룹(PII)을 보유하는 `principal_emails` 행의 마지막 확인 TTL입니다. 의도적으로 지출 보존보다 짧아서 프로비저닝 해제된 ID가 익명 지출 카운터가 남아있는 동안 만료됩니다.                                                                                                                   |
| `group_limit_mode`        | 아니요 | `min`(기본값) 또는 `max`입니다. 개발자가 상한이 있는 여러 그룹에 속할 때, `min`은 가장 제한적인 것을 강제하고 `max`는 가장 제한적이지 않은 것을 강제합니다. 강제 및 `/effective` 모두에서 사용됩니다.                                                                                                                              |

<h3 id="enforcement">
  `enforcement`
</h3>

`enforcement` 블록은 저장소를 사용할 수 없을 때 지출 한도 확인이 어떻게 작동하는지 제어합니다.

| 필드                     | 필수  | 설명                                                                                                                                                                                                                                    |
| ---------------------- | --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | 아니요 | 기본값 `false`입니다. Postgres 중단 시 지출 강제는 열린 상태로 실패하므로 추론이 계속 작동합니다. `true`로 설정하여 닫힌 상태로 실패: 초과 용량 개발자는 차단되지만, 저장소에 도달할 수 없으면 모든 사람이 차단됩니다. [`admin:`](#admin) 블록이 필요합니다. 지출 강제는 `admin`이 구성될 때만 실행되며, 이를 `true`로 설정하면 gateway는 시작을 거부합니다. |

<h3 id="pricing">
  `pricing`
</h3>

`pricing` 블록은 지출 미터에 USD 정가 대신 청구할 금액을 알려주므로 상한과 [`/effective`](/docs/ko/claude-apps-gateway-spend-limits#%2Feffective)는 계약 요금을 반영합니다. 금액은 USD로 유지되며 청구서가 아닌 추정치입니다. 두 가지 전제 조건:

* gateway 서버의 Claude Code v2.1.227 이상입니다. 이전 버전은 부팅 시 알 수 없는 키를 거부합니다.
* [`admin:`](#admin) 블록 또는 v2.1.268 이상에서 최소 하나의 정책이 있는 [`managed:`](#managed) 블록입니다. gateway는 `pricing`이 설정되었지만 두 블록 모두 없으면 시작을 거부합니다.

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| 필드           | 필수  | 설명                                                                                                                                       |
| ------------ | --- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | 아니요 | 기본값 `1`입니다. 미터는 정가 또는 재정의 여부에 관계없이 모든 미터링된 금액에 이를 곱하므로 `0.85`는 가격의 85%를 청구합니다. 0보다 크고 최대 10이어야 하며, 1 이상의 값은 [가격 인상](#mark-prices-up)입니다. |
| `overrides`  | 아니요 | 백만 토큰당 USD의 `{upstream, model, input, output, cache_read, cache_write}` 행입니다. 네 가지 요금 모두 필수입니다. 각각 0보다 크고 최대 10000이어야 합니다.               |

미터가 재정의 행과 일치하는 방식:

* 행은 `upstream`(즉, [`upstreams[].name`](#upstreams))이 `model`에 대해 제공하는 요청의 정가를 대체합니다. 여기에는 더 높은 [빠른 모드](/docs/ko/fast-mode#understand-the-cost-tradeoff) 요금이 포함되므로 빠른 요청과 표준 요청은 동일한 네 가지 요금으로 미터링됩니다.
* `claude-sonnet-4-6`과 같은 기본 제공 ID는 [`models[].id`](#models)처럼 일치하며, 미터가 해당 모델로 가격을 책정하는 모든 날짜 형식, 지역 Amazon Bedrock 형식 또는 Google Cloud의 Agent Platform 형식을 포함합니다. 다른 문자열(예: 별칭 또는 추론 프로필 ARN)은 클라이언트가 보낸 ID 또는 upstream으로 보낸 문자열과 대소문자를 구분하지 않고 일치합니다.
* 행이 겹칠 경우, 미터는 첫 번째 행이 아닌 가장 구체적인 행을 선택합니다. upstream으로 보낸 정확한 모델 문자열인 행, 그 다음 클라이언트가 보낸 정확한 ID와 일치하는 행, 그 다음 기본 제공 모델을 명명하는 행입니다.
* 알 수 없는 upstream 이름은 부팅을 실패하게 하며, 하나의 upstream에 대해 동일한 모델을 명명하는 두 행도 마찬가지입니다(기본 제공 모델의 두 가지 철자 포함). gateway는 부팅 시 요청 가능한 모델이 사용할 수 없는 행에 대해 경고합니다.
* 웹 검색 요청은 \$0.01 정가로 유지됩니다. 승수는 여전히 이에 적용됩니다.

지역별 요금의 경우, 각 지역에 자체 명명된 upstream을 제공하고 upstream당 하나의 행을 제공하세요.

<h4 id="mark-prices-up">
  가격 인상
</h4>

gateway 서버의 v2.1.271 이상에서는 `multiplier`를 1 이상 10까지 설정하여 공급자가 청구하는 것보다 더 많이 미터링할 수 있습니다(예: 내부 청구 요금). 이 예제는 모든 요청을 가격의 120%로 미터링합니다:

```yaml theme={null}
pricing:
  multiplier: 1.2
```

[`admin:`](#admin) 블록이 있으면 인상은 지출 한도에도 적용됩니다. 미터는 가격의 120%를 계산하므로 개발자는 상한에 더 빨리 도달합니다. gateway는 부팅 시 그렇다고 말하는 경고를 기록합니다.

승수는 upstream 공급자가 요청에 대해 청구하는 금액을 변경하지 않습니다.

gateway가 [서명된 클라이언트에 요금을 보내는](#send-the-rates-to-signed-in-clients) 경우, 개발자는 인상을 보기 위해 Claude Code v2.1.271 이상이 필요합니다. 이전 클라이언트는 1 이상의 `multiplier`를 무시하고 이 없이 비용을 표시합니다.

v2.1.271보다 이전인 gateway 서버는 1 이상의 `multiplier`를 설정하면 시작을 거부합니다.

<h4 id="send-the-rates-to-signed-in-clients">
  서명된 클라이언트에 요금 보내기
</h4>

gateway 서버의 v2.1.268 이상에서는 gateway가 `pricing`의 요금을 제공하는 [`managed`](#managed) 정책에 [`modelPricing`](/docs/ko/settings-reference#modelpricing) 관리 설정으로 넣습니다. 정책과 일치하는 개발자는 `/usage`, 상태 줄 및 OpenTelemetry에서 각 모델 ID를 제공하는 첫 번째 upstream의 `pricing` 요금을 봅니다. 정책과 일치하지 않는 개발자는 관리 설정을 받지 않으므로 해당 수치는 정가로 유지됩니다. 클라이언트는 Claude Code v2.1.242 이상에서 설정을 적용합니다.

* gateway가 추가하는 것: 정책의 `cli` 블록이 이미 `modelPricing`을 설정하지 않으면, gateway는 `multiplier`와 클라이언트가 요청할 수 있는 모든 모델 ID에 대해 해당 ID를 제공하는 첫 번째 upstream의 재정의 행을 추가합니다. 장애 조치 upstream만 청구하는 요금은 gateway에 유지됩니다.
* 하나의 정책 제외: 정책의 `cli` 블록에서 `modelPricing`을 `{}`로 설정하면, 해당 개발자는 정가로 유지됩니다.
* 정책의 자체 요금 유지: `cli` 블록이 자체 `multiplier` 또는 `overrides`로 `modelPricing`을 설정하는 정책은 해당 `modelPricing`을 유지하며, gateway는 자체 요금을 추가하지 않습니다.

<h3 id="models">
  `models`
</h3>

`models` 블록은 선택적 관리자 큐레이션 모델 목록이며, `/v1/models`에서 제공되고 upstream당 모델 ID를 변환하는 데 사용됩니다. 미국 이외의 Amazon Bedrock 지역, Amazon Bedrock 프로비저닝된 처리량 ARN 및 Microsoft Foundry 배포 이름에 필수입니다.

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

`upstream_model` 아래의 각 키는 구성된 upstream의 `name`과 일치해야 하며, 기본값은 공급자 이름입니다. upstream과 일치하지 않는 키는 부팅을 실패하게 하므로 사용하지 않는 공급자의 줄은 생략하세요.

<h3 id="managed">
  `managed`
</h3>

`managed` 블록은 IdP 그룹 또는 이메일 도메인을 기반으로 한 역할 기반 액세스 정책을 정의합니다. 정책은 순서대로 평가되며, 첫 번째 일치가 선택된 후 `match: {}` catch-all 기본값에 병합됩니다. 이들은 ETag/304 캐싱과 함께 `GET /managed/settings`에서 사용자별로 제공됩니다.

```yaml theme={null}
managed:
  policies:
    # Specific groups first.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Default catch-all last: matches everyone who authenticated.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

`match: {}` catch-all은 관례상 마지막에 나열되며 기본 계층으로 취급됩니다. 다른 모든 정책은 설정하지 않은 모든 키를 catch-all에서 상속하므로 역할별 항목은 조직 기본값과 다른 것만 나열하면 됩니다. 병합 규칙은 키 유형에 따라 다릅니다:

* **허용 목록**: `availableModels` 및 `permissions.allow`입니다. 특정 정책의 목록은 기본값의 목록을 완전히 대체합니다.
* **거부 목록 및 후크 배열**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` 및 모든 `hooks` 이벤트 유형 배열입니다. 이들은 기본값과 정책의 합집합을 취하므로 조직 전체 거부 또는 감사 후크는 역할별 재정의로 실수로 삭제될 수 없습니다.
* **레코드 유형 키**: `env`, `modelOverrides` 및 `skillOverrides`입니다. 이들은 얕게 병합되므로 역할별 `env` 블록은 설정하는 키를 재정의하고 나머지는 기본값에서 상속합니다.

`availableModels`는 `/v1/messages`에서 서버 측으로도 강제되므로 거부된 모델은 클라이언트가 보내는 것에 관계없이 `400`을 반환합니다.

gateway는 요청을 릴레이하기 전에 `model` 값 자체를 검증하므로 잘못된 형식의 값은 upstream에 도달하지 않습니다. 두 가지 경우에 `400`으로 요청을 거부합니다:

* 값이 누락되었거나 비어있을 때, gateway는 `model is required` 메시지로 요청을 거부합니다. 이 확인에는 Claude Code v2.1.228 이상을 실행하는 gateway가 필요합니다.
* 값이 있지만 문자열이 아닐 때, gateway는 `model must be a string` 메시지로 요청을 거부합니다. Claude Code v2.1.221 이상을 실행하는 gateway가 필요합니다.

| Matcher                                             | 동작                                                                                |
| --------------------------------------------------- | --------------------------------------------------------------------------------- |
| `match: {}`                                         | 모든 인증된 사용자와 일치합니다. 이 중 하나로 시작하고 나중에 위에 그룹 범위 정책을 추가하세요.                           |
| `match: { groups: [a, b] }`                         | JWT의 `groups` 클레임이 나열된 그룹 중 하나를 포함하면 일치합니다. 대소문자 구분: 그룹은 IdP의 정확한 대소문자와 일치해야 합니다. |
| `match: { email_domain: example.com }`              | JWT의 `email` 클레임에서 마지막 `@` 뒤의 부분과 일치하며, 대소문자를 구분하지 않습니다. 정책당 하나의 도메인을 허용합니다.      |
| `match: { groups: [a], email_domain: example.com }` | 두 조건 모두 일치해야 합니다                                                                  |

인증된 사용자가 정책과 일치하지 않으면 gateway의 기본값을 받으며, 이는 카탈로그의 모든 모델과 관리 설정이 없음을 의미합니다. 보장된 기본 정책을 원하면 마지막에 `match: {}` catch-all을 추가하세요.

<Note>
  gateway는 자체 사용자 디렉토리를 유지하지 않습니다. 사용자의 IdP 토큰에서 각 요청을 인증하여 토큰의 `groups` 클레임에서 그룹 멤버십을 읽고 이에 대해 정책을 평가합니다. 열거할 명단이 없고 사전 생성할 계정이 없으므로 SCIM 엔드포인트가 없습니다. SCIM이 동기화할 것이 없기 때문입니다.

  사용자 및 그룹 수명 주기 관리를 진실의 원천인 IdP의 기본 SCIM 프로비저닝 또는 전용 ID 거버넌스 플랫폼에서 실행하세요. 거기서 관리되는 멤버십 및 프로비저닝 해제는 토큰을 통해 gateway로 자동으로 흐릅니다. Claude 계정 자체의 SCIM 프로비저닝을 원하면 이는 [Claude for Enterprise](/docs/ko/admin-setup) 기능입니다.

  두 가지 전파 시계가 적용됩니다:

  * **정책 내용**: 정책을 편집하고 재배포하면 연결된 클라이언트의 다음 관리 설정 폴에서 1시간 이내에 도달합니다([다음 시작에만 적용되는 변경](/docs/ko/server-managed-settings#fetch-and-caching-behavior) 제외).
  * **그룹 멤버십**: 사용자의 그룹 멤버십을 변경하면 어떤 정책이 일치하는지 변경됩니다. 이는 다음 세션 재발급, 즉 다음 자동 새로고침에서 적용되며, `session.ttl_hours`로 제한됩니다.
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  gateway 부팅 시 중지되는 Matcher 값
</h4>

부팅 시 gateway는 모든 정책의 `match` 블록과 [`admin_groups`](#admin) 목록을 확인합니다. 이 값 중 하나라도 필드를 명명하는 오류로 gateway를 중지합니다:

* 빈 `groups` 목록
* `groups` 또는 `admin_groups`의 빈 항목
* 빈 `email_domain`
* `@`, 공백 또는 쉼표를 포함하는 `email_domain`입니다. gateway는 값을 자르고 이 확인 전에 선행 `@` 하나를 제거합니다. `example.com`과 같은 하나의 베어 도메인을 작성하세요.

v2.1.232 이전에는 gateway가 이 값으로 시작했습니다. 각 값은 다음과 같은 효과를 가졌습니다:

* 빈 `email_domain`: gateway는 도메인 확인을 건너뛰었으므로 빈 `email_domain`과 `groups` 목록이 없는 정책은 모든 인증된 사용자와 일치했습니다.
* 빈 `groups` 목록: 정책은 아무도 일치하지 않았습니다.
* `@`, 공백 또는 쉼표를 포함하는 `email_domain`: 정책은 아무도 일치하지 않았습니다.
* `groups` 또는 `admin_groups`의 빈 항목: 항목은 해당 사용자의 IdP `groups` 클레임도 빈 항목을 포함할 때만 사용자와 일치했습니다. `admin_groups`에서 해당 일치는 관리자 액세스를 부여했습니다. `admin_groups` 목록이 빈 항목을 포함하지 않으면 아무도 이 방식으로 관리자 액세스를 얻지 못했습니다.

<h4 id="what-goes-in-cli">
  `cli`에 들어가는 것
</h4>

각 `cli` 값은 완전한 Claude Code `managed-settings.json` 문서이며, MDM을 통해 배포하거나 `/etc/claude-code/managed-settings.json`에 배포할 동일한 스키마이며, 여기서는 YAML로 표현됩니다. CLI는 전달된 문서를 관리 계층에서 적용하며, 사용자 및 프로젝트 설정 위에 있고, 서버 관리 설정 대신입니다. 따라서 `policyHelper` 및 `wslInheritsWindowsSettings`와 같이 [OS 수준 정책 소스로 제한된 설정](/docs/ko/server-managed-settings#current-limitations)을 무시합니다.

gateway는 부팅 시 각 문서를 CLI의 설정 스키마에 대해 검증하므로 인식되지 않는 최상위 키는 모든 위반 키를 명명하는 오류로 부팅을 실패합니다. 스키마의 의도적으로 열린 부분은 여전히 임의의 값을 허용합니다. 더 새로운 클라이언트가 gateway의 스키마가 인식하지 못하는 항목을 인식할 수 있기 때문입니다. 이 열린 키에는 `env`, `pluginConfigs` 및 `permissions` 아래에 중첩된 키가 포함됩니다.

검증은 gateway의 설치된 버전과 함께 번들된 스키마를 사용하므로, 더 새로운 Claude Code 릴리스에서 도입한 최상위 설정 키를 관리 구성에 넣으려면 먼저 gateway를 업그레이드해야 합니다. 전체 조직에 배포하기 전에 하나의 클라이언트에서 새 정책을 스모크 테스트하세요.

전체 키 참조는 [Claude Code 설정](/docs/ko/settings-reference#all-settings)에 있습니다. 운영자가 가장 먼저 찾는 키:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Model access (also enforced server-side at /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Permission policy
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Environment pushed into the CLI process. DISABLE_UPDATES blocks
        # background and manual updates; DISABLE_AUTOUPDATER stops only
        # background updates.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Org-wide hooks. Hook commands run on developer machines, not the
        # gateway, so the path must exist on every client OS in the policy.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| 키                                          | 강제 대상         | 효과                                                                                                                                                                                                                                     |
| ------------------------------------------ | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Gateway + CLI | 모델 허용 목록입니다. `/v1/messages`에서도 확인되므로 패치된 클라이언트는 이를 우회할 수 없습니다.                                                                                                                                                                         |
| `permissions.allow` / `.deny`              | CLI           | 도구 및 명령 규칙입니다. [권한](/docs/ko/permissions)을 참조하세요.                                                                                                                                                                                           |
| `permissions.disableBypassPermissionsMode` | CLI           | [`bypassPermissions`](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode) 모드(권한 프롬프트를 건너뛰는 모드)와 `--dangerously-skip-permissions` 플래그를 차단하려면 `disable`로 설정하세요.                                                            |
| `allowManagedPermissionRulesOnly`          | CLI           | `true`일 때, 관리 설정은 권한 규칙의 유일한 설정 소스가 됩니다. [`allowManagedPermissionRulesOnly`](/docs/ko/settings-reference#allowmanagedpermissionrulesonly) 항목은 Claude Code가 무시하는 모든 소스를 나열합니다.                                                               |
| `env`                                      | CLI           | CLI 프로세스에 병합된 환경 변수입니다. 원격 측정, 자동 업데이트 및 모델 이름 재정의에 사용하세요.                                                                                                                                                                             |
| `hooks`                                    | CLI           | 조직 전체 [후크](/docs/ko/hooks)                                                                                                                                                                                                                  |
| `managedMcpServers`                        | CLI           | 정책과 일치하는 모든 개발자에게 [제공되는](/docs/ko/managed-mcp#provide-servers-through-managed-settings) 원격 MCP 서버(`http` 및 `sse`만). [정책의 MCP 서버](#mcp-servers-in-a-policy)를 참조하세요. gateway 서버 및 클라이언트에서 Claude Code v2.1.259 이상이 필요합니다. 이전 클라이언트는 키를 무시합니다. |

이 설정은 네트워크를 통해 도착하므로 CLI는 아래 나열된 설정을 적용하기 전에 각 개발자에게 보안 승인 대화 상자를 표시합니다:

* `hooks`
* 프록시 및 기본 URL 변수와 같이 개발자의 승인이 필요한 `env` 변수
* `apiKeyHelper` 및 `statusLine`과 같은 셸 실행 설정
* 샌드박스 바이너리 설정 `sandbox.bwrapPath`, `sandbox.socatPath` 및 `sandbox.ripgrep`
* `sandbox.network.tlsTerminate` 및 프록시 포트 설정과 같이 트래픽을 가로채고, 자격 증명을 주입하거나, 격리를 약화시키는 샌드박스 설정입니다. [보안 승인 대화 상자](/docs/ko/server-managed-settings#security-approval-dialogs)는 모두 나열합니다.

[승인 메모리](/docs/ko/server-managed-settings#approval-memory)는 승인이 얼마나 오래 지속되는지와 대화 상자가 다시 나타나는 시기를 다룹니다.

Claude Code는 모델 선택 설정 및 숫자 제한과 같이 개발자에게 승인 대화 상자를 표시하지 않고 전달된 일부 `env` 변수를 적용합니다. 다른 전달된 변수는 적용 전에 개발자의 승인이 필요할 수 있습니다. 비어있지 않은 프록시, 기본 URL 또는 `OTEL_EXPORTER_OTLP_ENDPOINT` 값은 항상 그렇습니다. 전달된 변수가 승인이 필요하면 대화 상자가 이를 명명합니다.

[환경 변수 및 승인 대화 상자](/docs/ko/server-managed-settings#environment-variables-and-the-approval-dialog)는 세부 사항과 전달된 값이 승인이 필요한지 여부를 결정하는 네 가지 개인 정보 보호 토글을 포함합니다. v2.1.218 이전에는 Claude Code가 더 적은 변수를 개발자에게 묻지 않고 적용했으므로 더 많은 전달된 변수가 대화 상자를 트리거했습니다.

gateway의 [원격 측정](#telemetry) 구성은 `OTEL_EXPORTER_OTLP_ENDPOINT`를 푸시하므로 `telemetry.forward_to`를 설정하면 각 대화형 클라이언트에서 대화 상자를 트리거합니다. 대화 상자는 손상되었거나 적대적인 gateway로부터 개발자의 머신을 보호하며, 개발자로부터 조직을 보호하지 않습니다.

`-p` 플래그가 있는 비대화형 실행은 대화 상자를 표시할 수 없습니다. 해당 실행에 대해서만 푸시된 설정을 적용하고 이를 승인된 것으로 기록하지 않으므로 개발자의 다음 대화형 세션은 여전히 대화 상자를 표시합니다. v2.1.207 이전에는 비대화형 실행이 설정을 승인된 것으로 저장했고 나중의 대화형 세션은 이에 대한 대화 상자를 표시하지 않았습니다.

개발자가 거부하면 Claude Code는 정책을 적용하지 않고 해당 세션을 종료합니다. 새 후크 또는 대화 상자를 트리거하는 env 변수를 광범위한 정책에 푸시하면 Claude Code는 일치하는 모든 개발자에게 대화 상자를 표시합니다. 실행 중인 세션에서 다음 시간별 폴에 대화 상자를 표시하고, 그렇지 않으면 개발자의 다음 시작 시 표시합니다.

`cli` 키는 이전 릴리스에서 `settings`로 명명되었습니다. 해당 철자는 여전히 별칭으로 허용되지만 새 배포는 `cli`를 사용해야 합니다.

<h4 id="mcp-servers-in-a-policy">
  정책의 MCP 서버
</h4>

정책이 일치하는 Claude Code 클라이언트에 MCP 서버를 제공하려면 해당 정책의 `cli` 블록에서 [`managedMcpServers`](/docs/ko/managed-mcp#provide-servers-through-managed-settings)를 설정하세요. gateway 서버 및 클라이언트에서 Claude Code v2.1.259 이상이 필요합니다.

gateway는 [Claude Code가 클라이언트에서 적용하는 동일한 규칙](/docs/ko/managed-mcp#what-an-entry-can-contain)으로 부팅 시 각 항목을 확인하며, 항목이 확인을 실패하면 gateway는 시작을 거부하고 항목을 명명합니다.

`gateway.yaml`에 `${VAR}` 참조를 작성하면 gateway는 [비밀 확장](#secret-expansion)을 통해 부팅 시 환경에서 이를 해결하므로 일치하는 모든 클라이언트는 리터럴 값을 받고 읽을 수 있습니다. [제공된 서버에 대한 헤더 지침](/docs/ko/managed-mcp#provide-servers-through-managed-settings)은 확장된 값에 적용됩니다.

gateway는 `cli` 블록에서 `.mcp.json` 철자 `mcpServers`를 거부하며, 부팅 오류는 `managedMcpServers`를 사용할 키로 명명합니다. v2.1.259 이전에는 gateway가 `cli` 블록의 모든 MCP 서버 정의를 거부했습니다.

<h4 id="claude-desktop-overlay">
  Claude Desktop 오버레이
</h4>

조직이 [Claude Desktop](/docs/ko/desktop)도 배포하면 동일한 gateway가 두 클라이언트를 제공합니다. Claude Desktop의 [관리 구성](https://claude.com/docs/third-party/claude-desktop/configuration)에서 `bootstrapUrl`을 `<listen.public_url>/user/bootstrap`으로 지정하세요. Claude Desktop은 해당 URL에서 OAuth 발급자를 파생하고, 이 gateway에 대해 동일한 장치 코드 로그인을 실행하고, 응답에서 구성을 가져옵니다.

<Note>
  gateway 서버의 Claude Code v2.1.203 이상이 필요하며, 명시적 옵트인이 필요합니다. 정책이 사용자와 일치하는 `desktop` 키를 전달하지 않으면 `/user/bootstrap`은 404를 반환합니다. 빈 `desktop: {}`은 정책을 옵트인하며, `match: {}` 기본 계층의 `desktop` 키는 이를 상속하는 모든 정책을 옵트인합니다. 감사 로그는 각 요청을 `desktop_bootstrap.serve` 또는 `desktop_bootstrap.denied`로 기록합니다.
</Note>

gateway는 일치하는 정책의 `cli` 블록과 최상위 gateway 구성에서 응답의 대부분을 파생합니다:

* `availableModels`의 모델 목록
* 베어 도구 이름 `permissions.deny` 항목의 비활성화된 도구입니다. 정책의 `desktop` 블록에서 `disabledBuiltinTools`를 설정하면 gateway는 파생된 목록과 값의 합집합을 제공하므로 이 방식으로 더 많은 도구를 비활성화할 수 있지만 `permissions.deny`를 통해 비활성화한 도구를 다시 활성화할 수 없습니다.
* `sandbox.network.allowedDomains`의 송신 허용 목록입니다. 정책의 `desktop` 블록에서 `coworkEgressAllowedHosts`를 설정하면 gateway는 파생된 목록 대신 해당 값을 사용합니다.
* gateway 자체를 가리키는 OTLP 엔드포인트 및 서명된 사용자의 ID 속성입니다. gateway는 해당 엔드포인트에서 받는 내보내기를 `forward_to` 대상으로 릴레이합니다. [`telemetry.forward_to`](#telemetry) 및 `listen.public_url`을 모두 설정할 때 엔드포인트 및 속성을 포함합니다.

  Claude Desktop은 모든 신호를 하나의 인코딩으로 내보냅니다: `http/protobuf` 또는 `OTEL_EXPORTER_OTLP_PROTOCOL` 또는 해당 신호별 변형 중 하나를 `http/json`으로 설정할 때 `http/json`입니다. gateway 서버의 Claude Code v2.1.261 이전에는 응답이 관계없이 `http/json`을 설정했으므로 protobuf만 허용하는 수집기는 Claude Desktop의 내보내기를 거부했습니다.

정책의 `desktop` 블록에서 `disabledBuiltinTools`, `coworkEgressAllowedHosts` 또는 Claude Desktop의 자체 `managedMcpServers` 설정을 설정하려면 gateway 서버의 Claude Code v2.1.232 이상이 필요합니다. Claude Desktop의 `managedMcpServers`는 객체가 아닌 배열 값을 취합니다.

gateway는 Claude Desktop 동등물이 없는 키(예: `hooks` 및 `Bash(npm *)` 같은 범위 지정 권한 규칙)를 부트스트랩 응답에서 생략합니다.

`cli` 옆에 선택적 `desktop` 블록을 추가하여 Claude Desktop 설정을 직접 설정하세요. Claude Desktop의 [관리 구성 참조](https://claude.com/docs/third-party/claude-desktop/configuration)의 설정을 평면 키 이름으로 작성하세요. `bootstrapUrl`과 같이 Claude Desktop이 MDM 또는 로컬 파일에서만 읽는 키는 생략하세요. gateway는 부팅 시 이를 거부합니다. v2.1.232 이전에는 gateway가 `chatTabEnabled` 및 `disableAutoUpdates`와 같은 고정된 11개의 기능 게이트 키 목록을 허용했고 부팅 시 다른 모든 키를 거부했습니다. v2.1.227 이전에는 gateway가 부팅 시 `chatTabEnabled` 및 `chatAdvancedFileAnalysisEnabled`도 거부했습니다.

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

모든 키는 선택적입니다. Claude Desktop은 생략한 모든 키에 대해 자체 기본값을 적용합니다. gateway는 부팅 시 각 `desktop` 블록을 Claude Desktop 자체가 사용하는 구성 스키마에 대해 검증하므로 실수는 전체 연결된 데스크톱에 도달하지 않고 키를 명명하는 오류로 gateway 시작에 표시됩니다. gateway는 블록이 다음을 포함할 때 부팅을 실패합니다:

* 알 수 없는 키
* Claude Desktop이 거부하거나 자동으로 삭제할 값을 가진 인식된 키(예: 빈 값 또는 중첩된 항목 내의 오타 부분 키). v2.1.260 이전에는 gateway가 `managedMcpServers` 또는 `orgPluginSettings` 항목의 중첩된 객체 내에서 오타 필드를 자동으로 삭제했습니다.
* gateway가 자체 계산하는 키: 추론 연결, 모델 목록 및 OTLP 릴레이입니다. [`upstreams`](#upstreams), [`models`](#models) 및 [`telemetry`](#telemetry) 섹션의 `forward_to`를 통해 이를 구성하세요.
* 현재 키의 레거시 별칭입니다. 부팅 오류에서 gateway는 작성할 정규 키를 명명합니다.

더 이상 사용되지 않는 값 또는 항목 형태(예: `transport` 없는 `managedMcpServers` 항목)를 사용하면 gateway는 시작하고 대체를 명명하는 경고를 기록합니다.

gateway는 `cli` 블록과 마찬가지로 설치된 버전과 함께 번들된 스키마에 대해 `desktop` 블록을 검증합니다. 더 새로운 Claude Desktop 릴리스에서 도입한 설정을 전달하려면 먼저 gateway를 업그레이드하세요. 예를 들어 `userPluginMarketplacesEnabled` 및 `userPluginUploadsEnabled`는 gateway 서버의 Claude Code v2.1.260 이상과 멤버 머신의 Claude Desktop 1.37937.0 이상이 필요합니다.

정책의 `desktop` 블록에서 `orgPluginSettings`를 설정하면 gateway는 Claude Desktop 1.15200.0 이상이 읽는 배열 형식으로 제공합니다. 더 오래된 데스크톱은 배열을 무시하고 플러그인 도구 정책을 강제하지 않으므로 이에 의존하기 전에 멤버를 1.15200.0 이상으로 업데이트하세요.

gateway는 정책의 `desktop` 블록이 설정하지 않은 키를 `match: {}` catch-all의 `desktop` 블록에서 채웁니다. 기본값의 `cli` 블록을 채우는 방식과 동일합니다. 기본값과 역할 정책 모두에서 `disabledBuiltinTools` 또는 `builtinToolPolicy`를 설정하면 gateway는 기본값의 제한을 유지합니다:

* `disabledBuiltinTools`: gateway는 기본값의 목록과 정책의 목록의 합집합을 사용합니다.
* `builtinToolPolicy`: 기본값에서 도구를 `allow` 이외의 값으로 설정하면 역할 정책에서 동일한 도구에 대해 `allow`를 설정해도 gateway는 해당 값을 유지합니다.

다른 모든 키의 경우 역할 정책에서 설정하면 gateway는 역할 정책의 값을 사용합니다. gateway는 배열 또는 `banner`와 같은 중첩된 객체를 전체적으로 대체하므로 역할 정책에서 `banner.text`를 설정하면 gateway는 기본값의 `banner.backgroundColor`를 삭제합니다.

Claude Desktop을 배포하지 않으면 정책에서 `desktop`을 완전히 생략하세요. gateway는 모든 사용자에 대해 `/user/bootstrap`에서 404를 반환합니다.

<h4 id="precedence-with-other-managed-sources">
  다른 관리 소스와의 우선 순위
</h4>

장치에 MDM 전달 정책 또는 로컬 `managed-settings.json`도 있으면 gateway 전달 설정이 우선합니다. 관리 설정 페이지의 [관리 계층 내 우선 순위](/docs/ko/managed-settings#precedence-within-the-managed-tier)는 로컬 소스가 적용되는 시기를 말하며, 샌드박스 잠금 키, `forceRemoteSettingsRefresh` 및 변수별 `env` 병합과 같이 어떤 소스를 선택했는지 관계없이 Claude Code가 모든 관리 소스에서 읽는 [키](/docs/ko/managed-settings#keys-read-from-every-admin-source)를 포함합니다. MDM 프로필 또는 관리 설정 파일에서 구성된 [`policyHelper`](/docs/ko/settings-reference#policyhelper)는 gateway가 설정을 전달하지 않을 때만 실행됩니다. 항목은 해당 출력이 대체하는 것을 말합니다.

[Claude Desktop](/docs/ko/desktop)과 같은 임베딩 호스트는 SDK `managedSettings` 옵션을 통해 정책을 제공할 수 있습니다. [임베딩 호스트의 부모 설정](/docs/ko/managed-settings#parent-settings-from-embedding-hosts)은 Claude Code가 이를 적용하는 시기를 말하며, [부모 설정 제한](/docs/ko/claude-apps-gateway#restrict-parent-settings)은 `allowManaged*Only` 잠금 없이도 여전히 적용되는 허용 방향 설정을 나열합니다.

gateway 정책은 비대화형 `claude -p` 실행 및 Agent SDK에서 생성된 세션을 포함하여 머신의 모든 Claude Code 호출에 적용됩니다. gateway가 시작 시 도달할 수 없으면 서명된 세션은 정책 없이 실행하지 않고 오류로 종료됩니다.

<h3 id="telemetry">
  `telemetry`
</h3>

CLI는 메트릭, 로그 및 활성화되면 추적을 gateway로 보내며, gateway는 이를 각 구성된 대상으로 그대로 릴레이합니다. 내보내기는 OpenTelemetry Protocol(OTLP)을 HTTP를 통해 사용합니다. 릴레이를 건너뛰고 세션이 수집기로 직접 내보내도록 하려면 [정책에서 수집기를 명명하세요](#export-directly-to-your-collector). 사용 모니터링]\(/ko/monitoring-usage)에서 CLI가 내보내는 메트릭 및 이벤트를 참조하세요.

CLI는 gateway 발급 JWT에서 읽은 인증된 사용자의 ID로 각 내보내기에 스탬프를 찍습니다: `user.id`, `user.email` 및 `user.groups` 속성입니다. 개발자별 비용 및 사용 귀속은 따라서 개발자 측 구성 없이 작동합니다.

[Claude Desktop](#claude-desktop-overlay) 및 gateway를 통해 서명된 Cowork 세션은 `user.email` 및 `user.groups`와 함께 `enduser.id`로 원격 측정에 스탬프를 찍으므로 `user.email` 또는 `user.groups`에 대한 하나의 쿼리로 터미널, Desktop 및 Cowork 사용을 포함할 수 있습니다. `user.groups`는 쉼표로 구분된 IdP 그룹 목록입니다.

Desktop 및 Cowork 원격 측정은 또한 `enduser.sub`를 전달하며, 이는 사용자의 이메일이 변경될 때 동일하게 유지되는 ID 공급자가 사용자에게 발급하는 `sub` 클레임입니다. 터미널 세션은 동일한 값을 `user.id` 아래에 스탬프를 찍으므로 `enduser.sub`를 터미널 `user.id`와 일치시키는 쿼리는 한 사용자의 터미널, Desktop 및 Cowork 사용을 함께 포함합니다. Desktop 및 Cowork 내보내기에서 `user.id`는 주제가 아닌 익명 식별자입니다.

Claude Code의 모든 OpenTelemetry 데이터와 마찬가지로 이 속성은 조직이 구성하는 대상으로만 이동하며 Anthropic으로는 이동하지 않습니다.

사용자의 그룹 목록이 퍼센트 인코딩 후 255자보다 길거나 그룹 이름에 쉼표 또는 등호 기호가 포함되면 gateway는 이를 자르지 않고 해당 사용자의 Desktop 및 Cowork 원격 측정에서 `user.groups`를 생략합니다. 해당 사용자의 터미널 세션은 여전히 전체 목록을 전달합니다.

주제가 퍼센트 인코딩 후 255자보다 길거나 공백, 인쇄 가능한 ASCII 외의 문자 또는 `,` `;` `=` `\` `"` `%` 중 하나를 포함하면 gateway는 `enduser.sub`를 생략합니다. 해당 사용자의 Desktop 및 Cowork 원격 측정은 다른 속성을 유지합니다.

Desktop 및 Cowork 원격 측정에서 `user.email` 및 `user.groups`를 위해 gateway 서버의 Claude Code v2.1.265 이상이 필요하며, 각 개발자의 머신에서 `user.groups`를 위해 Claude Desktop 1.24012 이상이 필요합니다.

`enduser.sub`를 위해 gateway 서버의 Claude Code v2.1.274 이상이 필요합니다.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Per-signal opt-in. Default: metrics only.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  각 대상은 `metrics`, `logs` 및 `traces`에 독립적으로 옵트인하며, 기본값은 메트릭만입니다. 신호는 민감도가 다릅니다:

  * **메트릭**: 토큰 수, 요청 수 및 지연 시간과 같은 집계 카운터
  * **로그 및 추적**: 전체 Bash 명령, 도구 입력 및 파일 경로를 전달할 수 있으며, Claude Code가 개발자의 머신에서 수행하는 모든 것을 포함합니다.

  로그 및 추적을 해당 데이터가 보증하는 액세스 제어 및 보존 정책이 있는 대상에서만 활성화하세요.
</Warning>

각 `forward_to` URL은 gateway의 자체 루프백 인터페이스의 수집기에 대한 하나의 예외를 제외하고 `https://`를 사용해야 합니다:

* `http://localhost:<port>`는 구성 검증을 통과하지만 [SSRF 가드](/docs/ko/claude-apps-gateway-deploy#threat-model-summary)는 `ECONNREFUSED_SSRF`로 모든 내보내기를 차단합니다. gateway의 환경에서 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`을 설정하지 않으면 차단됩니다.
* `http://127.0.0.1:<port>` 또는 `http://[::1]:<port>`는 해당 변수가 설정되지 않으면 부팅을 실패합니다.

클러스터 내 수집기의 경우 자체 내부 주소에서 HTTPS를 통해 노출하거나 변수가 설정된 사이드카로 실행하세요.

`HTTPS_PROXY`가 설정되면 gateway는 해당 프록시를 통해 내보내기를 보냅니다.

내부 수집기에 직접 도달하려면 호스트 이름으로 또는 `.internal.example.com`과 같은 선행 점이 있는 도메인으로 `NO_PROXY`에 추가하세요. gateway 서버의 Claude Code v2.1.277 이상이 필요합니다. gateway가 프록시 없이 수집기에 도달할 수 있는지 확인하세요. 선행 점이 없는 항목은 정확한 이름만 일치하며 그 아래의 이름은 일치하지 않습니다. CIDR 범위는 일치하지 않습니다.

[프록시 전용 송신](#proxy-only-egress)이 켜져 있으면 프록시 전용 송신이 꺼지므로 프록시에서 수집기를 허용하세요.

원격 측정은 CLI에서 기본적으로 꺼져 있습니다. `telemetry.forward_to` 및 `listen.public_url`을 모두 설정하면 gateway는 `/managed/settings`를 통해 6개의 환경 변수를 푸시하여 연결된 클라이언트에 대해 켭니다:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` 및 `OTEL_TRACES_EXPORTER`는 각각 최소 하나의 `forward_to` 대상이 해당 신호를 활성화하면 `otlp`로 설정되고, 그렇지 않으면 `none`으로 설정됩니다.
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

gateway 서버의 Claude Code v2.1.265 이전에는 gateway가 신호가 옵트인하지 않은 신호를 포함하여 세 개의 내보내기 선택기를 모두 `otlp`로 푸시했습니다.

푸시된 엔드포인트는 공개 URL에서 빌드되므로 메트릭 및 로그는 개발자 또는 정책의 OTEL 구성이 필요하지 않습니다.

`/login`을 통해 서명된 개발자는 자체 OTEL 구성으로 내보내기를 리디렉션할 수 없습니다:

* **로컬로 설정된 변수**: Claude Code는 푸시된 변수를 관리 계층에서 적용하므로 각 변수는 개발자가 로컬로 설정한 값을 재정의합니다.
* **로컬로 구성된 엔드포인트**: OTLP/HTTP 내보내기가 활성화되면 CLI는 gateway가 원격 측정 변수를 푸시했는지 여부에 관계없이 로컬로 구성된 엔드포인트를 무시합니다. 정책이 [수집기를 엔드포인트로 명명](#export-directly-to-your-collector)하지 않으면 내보내기는 gateway로 이동합니다.

신호에 대한 `forward_to` 대상이 없으면 gateway는 이를 수락하고 삭제합니다. 개발자가 이미 Claude Code 원격 측정을 수집기 중 하나로 내보내면 `forward_to` 대상으로 추가하고 로그 또는 추적을 내보내면 활성화하여 서명 후 데이터를 계속 받도록 하세요. 릴레이를 건너뛰려면 [정책에서 수집기를 명명하세요](#export-directly-to-your-collector).

[추적](/docs/ko/monitoring-usage#traces-beta)은 또한 각 클라이언트에서 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`이 필요합니다. gateway가 푸시하지 않으므로 관리 정책의 `env` 블록에서 설정하세요. 개발자는 푸시된 엔드포인트가 이미 트리거하는 동일한 [보안 승인 대화 상자](#managed)에서 이를 승인합니다.

추적하려는 그룹의 정책에서만 `1`로 설정하세요. 정책이 설정하지 않으면 `match: {}` catch-all 정책이 설정한 경우 해당 값을 상속합니다([병합 규칙](#managed) 참조). 개발자가 로컬로 변수를 설정해도 그룹의 클라이언트가 추적을 보내지 않도록 하려면 해당 그룹의 정책에서 `0`으로 설정하세요.

protobuf 및 JSON OTLP 인코딩 모두 릴레이되며 모든 OpenTelemetry 호환 백엔드가 대상으로 작동합니다.

<h4 id="export-directly-to-your-collector">
  수집기로 직접 내보내기
</h4>

`/login`을 통해 서명된 세션이 릴레이를 통해 수집기로 직접 원격 측정을 보내도록 하려면 [관리 정책](#managed)의 `env` 블록에서 `OTEL_EXPORTER_OTLP_ENDPOINT`를 수집기의 `https://` 기본 URL로 설정하세요. Claude Code는 `/v1/metrics`, `/v1/logs` 또는 `/v1/traces`를 설정한 URL에 추가합니다(예: `https://otel-collector.example.com:4318`). 각 신호는 OTLP/HTTP를 통해 거기로 내보냅니다. 각 개발자의 머신에서 Claude Code v2.1.265 이상이 필요합니다. 이전 클라이언트는 릴레이를 통해 내보냅니다.

수집기에 인증하려면 동일한 `env` 블록에서 `OTEL_EXPORTER_OTLP_HEADERS`를 설정하세요. 세션은 이 방식으로 명명된 수집기에 개발자의 gateway 세션 토큰을 보내지 않습니다.

정책에서 이 엔드포인트를 추가하거나 변경하면 Claude Code는 각 개발자에게 [보안 승인 대화 상자](#managed)에서 이를 승인하도록 요청한 후 대화형 세션에서 이를 적용합니다.

Claude Code는 신호를 직접 내보내기 전에 엔드포인트를 확인하고 확인이 실패하면 해당 신호를 릴레이에 유지합니다. 확인에는 다음이 포함됩니다:

* 엔드포인트는 gateway 자체에서 옵니다. MDM 프로필 또는 로컬 `managed-settings.json`에서 동일한 변수를 설정하면 내보내기는 릴레이에 유지됩니다.
* URL은 `https://`를 사용하거나 루프백 주소에 `http://`를 사용합니다.
* URL은 쿼리 또는 조각이 없는 `/v1/<signal>`로 끝나는 경로로 확인됩니다. Claude Code는 일반 변수에서 해당 경로를 자체 빌드합니다. `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`와 같은 신호별 변수를 작성된 대로 사용하므로 전체 경로를 거기에 포함하세요.
* URL은 gateway의 자체 호스트가 아닙니다. gateway로 주소 지정된 엔드포인트는 릴레이 경로와 세션 토큰을 유지합니다.
* 어떤 설정 소스에서도 [`otelHeadersHelper`](/docs/ko/settings-reference#otelheadershelper)를 구성하지 않았습니다. 도우미가 구성되면 모든 신호는 릴레이에 유지됩니다.

명명한 엔드포인트는 내보내기가 가는 위치만 변경합니다. 여전히 `OTEL_*_EXPORTER` 선택기로 어떤 신호를 내보낼지 선택합니다.

엔드포인트 자체는 내보내기를 켜지 않으므로 gateway가 이미 푸시하지 않으면 변수도 설정하세요:

* gateway가 이미 [원격 측정 변수를 푸시](#telemetry)하면 활성화, 선택기 및 프로토콜을 포함하고 푸시된 `<public_url>` 값을 재정의합니다. `forward_to` 대상이 활성화하지 않는 신호에 대해서만 `OTEL_*_EXPORTER` 선택기를 `otlp`로 직접 설정하세요.
* 그렇지 않으면 `CLAUDE_CODE_ENABLE_TELEMETRY=1`, `OTEL_*_EXPORTER` 선택기 및 `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`도 설정하세요.

개발자가 로그아웃하거나 다른 gateway에 로그인하면 수집기로의 내보내기가 중지되고 Claude Code는 각 남은 배치를 늦게 전달하지 않고 삭제합니다.

<h4 id="when-a-destination-fails">
  대상이 실패할 때
</h4>

gateway는 버퍼링, 재시도 또는 원격 측정 저장을 하지 않으므로 대상에 도달하지 않는 내보내기는 늦게 전달되지 않고 삭제됩니다. 각 대상은 독립적으로 성공하거나 실패하며 내보내는 클라이언트는 어느 쪽이든 성공 응답을 받으므로 실패한 전달은 gateway의 로그에만 나타납니다.

대상에 대해 5번 연속 실패한 후 gateway는 30초 단위로 전달을 일시 중지하고 각 일시 중지를 기록하며 전달이 성공할 때까지 계속합니다. 모든 오류 응답, 시간 초과 또는 연결 오류는 실패한 전달로 계산됩니다. `400`, `413`, `415`, `422` 및 `431`은 제외되며, 이들은 수집기가 해당 내보내기의 페이로드를 잘못되었거나 너무 크다고 거부했음을 의미합니다.

거부된 페이로드는 실패 카운트를 진행하거나 재설정하지 않습니다. gateway는 대상으로 전달을 계속하고 첫 거부 및 그 후 100번마다 경고를 기록하며 대상을 명명합니다.

<h3 id="http-tuning">
  HTTP 튜닝
</h3>

4개의 선택적 최상위 블록 `access_control`, `limits`, `timeouts` 및 `rate_limits`는 HTTP 표면을 조정합니다. 기본값은 대부분의 배포에 적합합니다.

| 블록               | 키                                              | 기본값      | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---------------- | ---------------------------------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | 비어있음     | `trusted_proxies` 해결 후 클라이언트 주소별 인바운드 IP 허용/거부입니다. `deny_cidrs`가 먼저 확인됩니다. 일치하는 클라이언트는 `allow_cidrs`도 일치해도 거부됩니다. `allow_cidrs`가 비어있지 않으면 gateway는 기본 거부입니다. `/healthz` 및 `/readyz`는 `allow_cidrs`에서 제외됩니다. 신뢰할 수 있는 프록시가 IP 주소가 아닌 `X-Forwarded-For` 항목을 보내면 실제 클라이언트는 알 수 없으며 gateway는 확인할 내용을 명명하는 경고를 한 번 기록합니다. 목록이 요청에 적용되는 경우 `403`으로 거부하고 감사 이유 `xff_unparseable`입니다. 어느 것도 적용되지 않으면 요청을 제공하고 프록시의 자체 주소를 IP별 요금 제한 및 감사의 클라이언트 IP로 사용합니다. |
| `limits`         | `max_request_bytes`                            | 32 MiB   | 최대 인바운드 요청 본문입니다. 크기 초과 요청은 본문이 버퍼링되기 전에 `413`을 받습니다. 큰 파일 또는 이미지 요청에 대해 올립니다.                                                                                                                                                                                                                                                                                                                                                                     |
| `limits`         | `max_request_header_bytes`                     | 설정 해제    | 설정하면 크기 초과 헤더는 `431`을 반환합니다.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `limits`         | `max_url_length`                               | 설정 해제    | 설정하면 과도하게 긴 URL은 `414`를 반환합니다.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000   | upstream의 응답 헤더(첫 바이트까지의 시간)를 기다리는 최대 시간입니다. 응답 본문은 그 후 벽시계 상한 없이 스트리밍됩니다. 직접 Anthropic upstream 경로에 적용됩니다. 다른 모든 공급자에서 gateway는 응답이 시작될 때까지 최대 1시간을 기다립니다.                                                                                                                                                                                                                                                                                        |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600 | 인증되지 않은 장치 인증 엔드포인트의 IP별 요금 제한입니다. 공유 송신 IP 또는 NAT 뒤의 큰 조직에 대해 올립니다. [대규모 배포](/docs/ko/claude-apps-gateway-deploy#large-rollouts)는 크기를 조정하는 방법을 보여줍니다. 이 제한은 장치 부여 로그인 흐름에만 적용되며 `/v1/messages` 추론에는 적용되지 않습니다. [사용자 코드 무차별 대입 공격 저항](/docs/ko/claude-apps-gateway-deploy#user-code-brute-force-resistance)을 참조하세요.                                                                                                                                          |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600 | `/device`의 `user_code` 제출에 대한 IP별 요금 제한입니다. 이것이 누군가가 다른 개발자의 코드를 추측하는 것을 중지합니다. [대규모 배포](/docs/ko/claude-apps-gateway-deploy#large-rollouts)는 얼마나 올릴지 보여줍니다.                                                                                                                                                                                                                                                                                            |

두 `access_control` 목록을 비어있게 두면(기본값) gateway는 모든 클라이언트 주소를 제공하므로 네트워크만 도달할 수 있는 사람을 제한합니다. gateway는 개발자 머신에서 명령을 실행하는 [관리 설정](#managed)을 푸시할 수 있기 때문에 중요합니다.

`allow_cidrs`가 비어있는 동안 gateway는 요청에 응답하는 방식을 변경하지 않고 두 위치에서 경고합니다:

* **부팅 시**: 운영 로그의 경고는 개인 범위 `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `100.64.0.0/10`, `127.0.0.0/8`, `::1/128` 및 `fc00::/7`만 허용하고 개발자가 연결하는 다른 내부 범위를 권장합니다. gateway를 루프백 주소에 바인드하고 `trusted_proxies` 또는 `public_url`을 설정하지 않으면(로컬 개발처럼) 경고가 나타나지 않습니다.
* **런타임**: 요청이 처음 해당 개인 범위 외의 주소에서 도착하면 gateway는 경고를 기록하고 클라이언트 IP를 전달하는 [`access.public_client` 감사 이벤트](/docs/ko/claude-apps-gateway-deploy#logs)를 내보냅니다. 둘 다 프로세스당 한 번 실행됩니다. 링크 로컬 주소 `169.254.0.0/16` 및 `fe80::/10`은 공개로 계산되지 않습니다. gateway는 이 확인이 실행되기 전에 `/healthz` 및 `/readyz`에 응답하므로 공개 범위의 상태 프로브는 이를 트리거하지 않습니다.

두 신호 모두 gateway가 해결하는 클라이언트 주소를 사용합니다. 로드 밸런서, 포트 포워드 또는 터널이 트래픽을 릴레이하고 `listen.trusted_proxies`에 나열되지 않으면 gateway는 릴레이의 주소를 보며, 이는 보통 개인이므로 런타임 경고나 개인 허용 목록이 이를 포착하지 않습니다.

그러한 프론트 엔드 뒤에서 먼저 [`listen.trusted_proxies`](#listen)를 설정하여 gateway가 실제 클라이언트 주소를 보도록 하고 gateway와 그 앞의 모든 것을 공개 인터넷에서 도달할 수 없도록 유지하세요.

<h3 id="load_test_mode">
  `load_test_mode`
</h3>

`load_test_mode` 블록을 사용하면 모델 공급자를 호출하지 않고 gateway를 부하 테스트할 수 있습니다. 켜져 있는 동안 gateway는 각 공급자 요청을 평소대로 빌드하고 서명하며, 보내지 않고 버리고, 정상적인 응답 경로를 통해 통조림 회신을 다시 스트리밍합니다. 회신은 통조림임을 말하는 문장으로 시작하는 채우기 텍스트입니다.

v2.1.283 이상이 필요합니다. 이전 버전은 키가 설정되면 시작을 거부하므로 모든 복제본을 업그레이드한 후 블록을 추가하고 롤백하기 전에 제거하세요.

아래 예제는 기본값으로 모드를 켜며, 약 10초에 걸쳐 스트리밍되는 750개 출력 토큰의 회신입니다:

```yaml theme={null}
load_test_mode:
  enabled: true
  reply_tokens: 750     # roughly how many tokens of text each canned reply carries
  reply_seconds: 9.5    # how long a streamed reply takes
```

| 필드              | 필수  | 설명                                                                                               |
| --------------- | --- | ------------------------------------------------------------------------------------------------ |
| `enabled`       | 예   | `true`는 모드를 켭니다. `false`는 모드를 끈 상태로 파일에 숫자를 유지합니다. 블록이 없으면 gateway는 시작을 거부합니다.                   |
| `reply_tokens`  | 아니요 | 기본값 `750`입니다. 각 통조림 회신이 전달하는 텍스트의 토큰 수(대략), 1에서 100000 사이의 정수입니다.                                |
| `reply_seconds` | 아니요 | 기본값 `9.5`입니다. 스트리밍된 회신이 걸리는 시간(0에서 600 사이). `0`은 전체 회신을 한 번에 보냅니다. 비스트리밍 요청에 대한 회신은 항상 한 번에 옵니다. |

이 모드의 부하 테스트는 gateway, Postgres 및 gateway 앞의 모든 것을 포함합니다. 공급자의 한도, 속도 또는 네트워크 경로는 포함하지 않습니다.

모드가 켜져 있는 동안 요청은 최대 7자리의 정수를 보유하는 `x-load-test-user` 헤더를 전달할 수 있으며, gateway는 각 숫자를 요청과 함께 온 개발자의 이메일 및 그룹을 가진 별도의 개발자로 계산합니다. 부하 테스트 배포에 자체 빈 데이터베이스를 제공하세요. gateway는 모드가 켜져 있고 개발자가 이미 무언가를 지출한 데이터베이스에 대해 시작을 거부합니다.

<Warning>
  개발자가 사용하는 gateway에 대해 이를 켜지 마세요. 모든 요청은 통조림 회신을 받고 모델은 호출되지 않습니다. gateway는 부팅 시 `load_test_mode is on` 경고를 기록하고 모드가 켜져 있는 동안 각 `inference` [감사 이벤트](/docs/ko/claude-apps-gateway-deploy#logs)를 `load_test: true`로 표시합니다.
</Warning>

<h2 id="complete-example">
  완전한 예제
</h2>

이 전체 참조 구성은 모든 핵심 섹션을 다룹니다. [HTTP 조정 블록](#http-tuning)은 기본값을 유지합니다. 복사하고 필요하지 않은 것을 삭제하고 값을 채웁니다. [빠른 시작](/docs/ko/claude-apps-gateway#quickstart)의 구성은 이것의 최소 버전입니다.

```yaml gateway.yaml theme={null}
# 실행:
#   claude gateway --config gateway.yaml
#
# 운영 로그 상세도는 CLAUDE_GATEWAY_LOG_LEVEL
# 환경 변수(debug | info | warn | error; 기본값 info)로 제어됩니다. debug는
# 또한 각 id_token의 클레임 이름을 기록하여 groups_claim 진단을 위해 사용됩니다.
# 감사 이벤트는 항상 내보내지므로 영향을 주지 않습니다.

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
  # Okta org 서버가 발급자인 경우 필수입니다. id_token은
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
  # connect_timeout_seconds: 5

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

# 모델 제공자를 호출하지 않고 이 배포를 부하 테스트합니다. 개발자가 사용하는
# 게이트웨이에서는 절대 사용하지 마십시오. 모든 요청이 미리 정해진 응답을 받습니다.
# load_test_mode:
#   enabled: true
#   # reply_tokens: 750
#   # reply_seconds: 9.5

# 계약 요금으로 미터링하고 USD 정가 대신 사용합니다. admin: 또는 managed: 정책이 필요합니다.
# managed:를 사용하면 동일한 요금이 로그인한 클라이언트로도 이동합니다.
# 아래 요금은 자리 표시자이며 실제 계약 가격이 아닙니다.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

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
        # 기본 선택기 옵션을 availableModels로 제한하여 계약자가
        # 기본값에 400을 얻지 않도록 합니다.
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

위의 모든 것은 게이트웨이 서버를 구성합니다. 개발자 머신을 지정하는 것은 각 장치에서 Claude Code의 [관리형 설정](/docs/ko/managed-settings)을 통해 별도로 구성됩니다. 게이트웨이는 로그인 키를 직접 푸시할 수 없습니다. 왜냐하면 이 키들이 클라이언트에게 게이트웨이가 어디에 있는지 알려주기 때문입니다.

CLI의 경우 OS별 `managed-settings.json`에서 이 키들을 설정합니다. 두 개의 로그인 키는 각 개발자의 `/login`을 게이트웨이로 라우팅합니다:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"`는 Claude Desktop의 이그레스 허용 목록 전달이 임베드된 Claude Code 세션에서 작동하도록 유지합니다. [Claude Desktop 세션에 정책 전달](/docs/ko/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions)에서 메커니즘과 옵트인이 위치해야 하는 곳을 설명합니다.

`managed-settings.json` 파일을 각 장치에 배포합니다. 일반적으로 MDM 플랫폼을 통해 배포합니다. 각 메커니즘이 정책을 저장하는 위치는 [정책을 저장하는 각 메커니즘의 위치](/docs/ko/managed-settings#where-each-mechanism-stores-the-policy)를 참조하십시오.

기본적으로 Windows의 레지스트리 정책 또는 macOS의 관리형 기본 설정 plist는 [위의 예외 키 및 교차 소스 확인](#precedence-with-other-managed-sources)을 제외하고 `managed-settings.json` 파일을 병합하지 않고 대체합니다. 이 스니펫의 모든 세 키는 최고 우선순위 소스 규칙을 따르므로 그룹 정책 또는 구성 프로필을 통해 정책을 전달하는 플릿은 대신 해당 메커니즘에 모두 세 개를 모두 배치해야 합니다.

Claude Desktop의 경우 Claude Desktop의 자체 [관리형 구성](https://claude.com/docs/third-party/claude-desktop/configuration)에서 `bootstrapUrl` 키를 `<listen.public_url>/user/bootstrap`으로 설정합니다. 로그인 흐름 및 그룹별 정책은 정책이 `desktop` 키로 서버 측에서 옵트인되면 CLI의 정책과 일치합니다. 옵트인이 없으면 `/user/bootstrap`은 404를 반환합니다. 서버 측 절반은 [Claude Desktop 오버레이](#claude-desktop-overlay)를 참조하십시오.

Claude Code는 [`forceLoginGatewayUrl`](/docs/ko/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/ko/settings-reference#gatewayinternalnetworks), 및 [`forceLoginMethod`](/docs/ko/settings-reference#forceloginmethod)의 `"gateway"` 값을 머신의 관리형 소스에서만 인정합니다: `managed-settings.json`, macOS plist 또는 Windows HKLM 레지스트리, 또는 정책 헬퍼. 개발자가 자신의 `~/.claude/settings.json`에서 이들을 설정하는 것은 효과가 없으며, 게이트웨이 페이로드에서 설정하는 것도 마찬가지입니다.

<h2 id="related">
  관련
</h2>

* [Claude 앱 게이트웨이 개요](/docs/ko/claude-apps-gateway): 빠른 시작 및 개발자 연결
* [배포 가이드](/docs/ko/claude-apps-gateway-deploy): IdP 설정, 컨테이너 이미지, Kubernetes 및 Cloud Run, 운영
* [지출 한도](/docs/ko/claude-apps-gateway-spend-limits): 개발자별 한도 및 관리자 API
