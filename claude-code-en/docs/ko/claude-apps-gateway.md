> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Amazon Bedrock, AWS의 Claude Platform, Google Cloud 및 Microsoft Foundry용 Claude 앱 게이트웨이

> SSO 로그인, 그룹별 모델 액세스, OTLP 텔레메트리를 갖춘 자체 호스팅 게이트웨이를 통해 Amazon Bedrock, AWS의 Claude Platform, Google Cloud 또는 Microsoft Foundry에서 Claude Code를 실행합니다.

<Note>
  Claude 앱 게이트웨이는 자신의 클라우드 제공자를 통해 추론을 라우팅해야 하거나 선호하는 조직을 위해 설계되었습니다. 예를 들어 [데이터 거주지](/docs/ko/claude-apps-gateway-deploy#compliance-posture) 요구사항을 충족하기 위해서입니다. 이러한 요구사항이 없으며 SCIM 프로비저닝 또는 웹 및 모바일의 Claude Code와 같은 다른 기능에 액세스하려면 Claude Enterprise가 더 적합할 수 있습니다. 모든 배포 방법의 전체 비교는 [기능 가용성](/docs/ko/feature-availability) 페이지를 참조하십시오.
</Note>

Claude 앱 게이트웨이는 개발자의 Claude Code 클라이언트와 모델 제공자 사이에 위치하는 자체 호스팅 서비스입니다. 개발자는 API 키나 클라우드 자격증명을 보유하는 대신 회사 ID 제공자(IdP)로 로그인합니다. 게이트웨이는 업스트림 자격증명을 보유하고, IdP 그룹별로 모델 액세스 및 [관리 설정](/docs/ko/managed-settings)을 적용하며, 사용 현황 텔레메트리를 자신의 관찰성 스택으로 전달합니다.

이는 `claude` 바이너리에 포함되어 있으므로, 노트북에서 Claude Code를 실행하는 동일한 실행 파일이 `claude gateway --config gateway.yaml`로 게이트웨이 서버를 실행합니다.

이 페이지는 다음을 다룹니다:

* [Claude 앱 게이트웨이를 사용하는 이유](#why-claude-apps-gateway), 자체 실행보다 추가되는 것, 그리고 다른 것이 더 적합한 경우
* [전제조건](#prerequisites)이 포함된 [빠른 시작](#quickstart)으로 게이트웨이를 0에서 로그인한 개발자까지 진행
* [개발자 연결](#connect-developers), 관리 설정을 통해 게이트웨이 URL 설정 포함
* [가용성 및 제한사항](#availability-and-limitations) - 게이트웨이를 통해 작동하는 Claude Code 기능 및 서버가 지원하는 것 포함

동반 페이지는 더 깊이 있게 다룹니다. [구성 참조](/docs/ko/claude-apps-gateway-config)는 빠른 시작이 작성하는 YAML 파일의 모든 옵션을 다루고, [배포 가이드](/docs/ko/claude-apps-gateway-deploy)는 IdP별 설정, Kubernetes 및 Cloud Run 배포, 그리고 운영을 다룹니다.

<h2 id="why-claude-apps-gateway">
  Claude 앱 게이트웨이를 사용하는 이유
</h2>

[게이트웨이 개요](/docs/ko/gateways)는 게이트웨이가 무엇을 하는지, 왜 하나를 실행하는지 다룹니다. Claude 앱 게이트웨이는 Anthropic의 자체 게이트웨이로, `claude` 바이너리에 내장되어 있고 각 Claude Code 릴리스와 함께 테스트되므로, Claude Code가 보내는 헤더와 요청 필드를 운영자가 별도의 허용 목록을 유지하지 않고도 전달합니다. 배포되면 다음을 제공합니다:

* **자격증명**: 업스트림 API 키 또는 클라우드 자격증명은 인프라에만 존재합니다. 개발자는 회사 SSO로 인증하고 단기 베어러 토큰을 받으므로, 오프보딩은 IdP에서 발생합니다. 사용자를 프로비저닝 해제하면 게이트웨이 액세스는 기본값인 1시간 내에 세션 수명 내에 만료됩니다.
* **액세스 제어**: IdP 그룹은 모델 허용 목록 및 [관리 설정](/docs/ko/managed-settings) 정책에 매핑됩니다. 게이트웨이는 모델 액세스를 서버 측에서 적용하여 부여되지 않은 모델에 대한 요청을 거부하고, 각 그룹의 관리 설정 정책을 선택하며, CLI는 [관리 설정 계층](/docs/ko/settings#settings-precedence)에서 이를 적용합니다. 다른 팀은 다른 모델, 도구, 권한을 얻고, 개발자는 정책이 잠근 것을 재정의할 수 없습니다.
* **설정 전달**: 게이트웨이는 관리 설정을 로그인한 클라이언트에 자체적으로 전달하여 [서버 관리 설정](/docs/ko/server-managed-settings)을 대체합니다.
* **텔레메트리**: 각 구성된 대상은 기본적으로 토큰 수, 모델, 사용자 ID 및 지연 시간이 포함된 [OpenTelemetry Protocol (OTLP) 메트릭](/docs/ko/monitoring-usage)을 받으며, 로그 및 추적은 대상별 옵트인입니다.
* **업스트림 라우팅**: 클라이언트는 Anthropic Messages API를 게이트웨이에 말하고, 게이트웨이는 Amazon Bedrock, [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws), Google Cloud의 Agent Platform, Microsoft Foundry 또는 Anthropic API 중 각 업스트림에 대해 변환하며, 그들 사이에 장애 조치가 있습니다. 개발자가 알아차리거나 재구성하지 않고도 지역, 제공자 또는 장애 조치 순서를 변경할 수 있습니다.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Claude Code 클라이언트와 Claude Desktop의 Chat, Cowork, Code 탭이 HTTPS와 베어러 토큰으로 인프라 내 자체 호스팅 Claude 앱 게이트웨이에 연결되고, IdP에 대해 사용자에게 로그인하고, PostgreSQL에 인증 상태를 저장하고, OTLP 수집기에 텔레메트리를 중계하고, Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry 또는 Anthropic API로 추론을 전달하는 것을 보여주는 다이어그램" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  게이트웨이의 자체 데이터 평면은 Anthropic API가 구성된 업스트림이 아닌 한 Anthropic 인프라에 아무것도 보내지 않습니다. 텔레메트리, 감사 로그, 관리 설정 및 개발자의 IdP 신원이 어디로 가는지 제어하고, 게이트웨이는 이 중 어느 것도 Anthropic에 보내지 않습니다. 나머지 트래픽의 경우 CLI 프로세스가 보낼 수 있는 것과 이를 닫는 방법은 [규정 준수 태세](/docs/ko/claude-apps-gateway-deploy#compliance-posture)를 참조하세요.
</Note>

Claude Code의 어떤 기능이 게이트웨이를 통해 작동하고 서버 자체가 무엇을 지원하는지는 아래 [가용성 및 제한사항](#availability-and-limitations)을 참조하세요. 비용, 우회, 여러 게이트웨이 실행, 서버리스 플랫폼과 같은 결정은 [배포 가이드](/docs/ko/claude-apps-gateway-deploy#deployment)를 참조하세요.

<h3 id="other-gateway-implementations">
  다른 게이트웨이 구현
</h3>

이미 필요를 충족하는 LLM 게이트웨이 또는 API 게이트웨이를 실행 중인 경우, 계속 사용하세요; [다른 LLM 게이트웨이](/docs/ko/llm-gateway)는 Claude Code를 이에 대해 구성하는 것을 다룹니다.

[게이트웨이 호환성 가이드](/docs/ko/llm-gateway-protocol)는 Claude Code가 모든 게이트웨이에서 기대하는 것을 문서화합니다: 호출하는 엔드포인트, 전달할 헤더 및 본문 필드, 그리고 제거될 때 작동하지 않는 것입니다. 실행 중인 Claude 앱 게이트웨이는 `GET /protocol`에서 자체 프로토콜 참조를 제공하며, Claude Code 클라이언트에 노출하는 엔드포인트를 설명합니다: SSO 로그인, 추론, 관리 설정 전달, 모델 검색, 텔레메트리. 배포된 게이트웨이(예: 아래 [빠른 시작](#quickstart)이 생성하는 것)에서 `curl https://claude-gateway.internal.example.com/protocol`로 가져옵니다.

프로토콜의 주요 변경사항은 미리 공지되지만, 무한 역호환성은 보장되지 않습니다.

<h2 id="quickstart">
  빠른 시작
</h2>

이 빠른 시작은 최소 경로를 안내합니다: IdP에서 OAuth 클라이언트를 등록하고, `gateway.yaml`을 작성하고, Docker Compose로 게이트웨이를 Postgres와 함께 실행하고, 엔드 투 엔드 로그인을 확인합니다. Amazon Bedrock 업스트림을 사용합니다; Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry 및 Anthropic API는 [구성 참조](/docs/ko/claude-apps-gateway-config#upstreams)에 표시된 대로 `upstreams` 블록을 교체하여 동등하게 지원됩니다. 끝에서 개발자가 `/login`할 수 있는 게이트웨이가 있습니다.

<Note>
  **개인 네트워크에 배포하세요.** Claude Code는 주소가 개인인 게이트웨이에만 연결합니다. 이는 신뢰할 수 있는 게이트웨이가 개발자 머신에서 명령을 실행하는 설정을 푸시할 수 있기 때문에 보안 가드입니다. 게이트웨이를 내부 로드 밸런서 또는 VPN 뒤에 놓고 개인 IP로만 확인되는 호스트명을 제공하세요. 내부 네트워크가 조직이 소유한 공개 IPv4 공간에서 번호가 지정된 경우 [소유한 공개 주소 공간에서 게이트웨이 허용](#allow-a-gateway-on-public-address-space-you-own)을 참조하세요.
</Note>

<h3 id="prerequisites">
  필수 조건
</h3>

시작하기 전에 다음을 준비하세요:

| 필요한 것                        | 세부사항                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Claude Code v2.1.195 이상      | `claude gateway` 서브명령 및 게이트웨이 로그인 흐름은 v2.1.195에서 제공됩니다. 이전 공개 빌드는 이를 포함하지 않습니다. 게이트웨이 서버를 실행하는 머신과 각 개발자의 머신 모두 v2.1.195 이상이어야 합니다; `claude update`를 실행하여 최신 릴리스를 받으세요. [Claude Platform on AWS 업스트림](/docs/ko/claude-apps-gateway-config#claude-platform-on-aws)은 게이트웨이 서버에서 Claude Code v2.1.198 이상이 필요합니다.                                                                                                                                                                                                                             |
| OpenID Connect (OIDC) ID 제공자 | Okta, Microsoft Entra ID, Google Workspace, Keycloak, Dex 또는 PingFederate와 같은 다른 OIDC 호환 IdP. 게이트웨이는 표준 OIDC 검색 및 인증 코드 흐름을 이에 대해 실행합니다. SAML 및 LDAP는 지원되지 않습니다.                                                                                                                                                                                                                                                                                                                                                                     |
| PostgreSQL 14 이상             | 브라우저 콜백이 쓰고 폴링 CLI가 읽는 장치 로그인 흐름, 그리고 속도 제한 카운터를 지원합니다. 가장 작은 계층을 포함한 모든 관리 Postgres가 작동합니다. 지출 제한이 구성되지 않으면 게이트웨이는 몇 KB의 단기 인증 상태를 저장합니다; [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)을 사용하면 백업해야 하는 지속적인 지출, 감사 및 ID 테이블도 보유합니다. `?sslmode=require`를 통한 TLS가 권장됩니다.                                                                                                                                                                                                                                                               |
| 모델 업스트림                      | Amazon Bedrock 자격증명, Claude Platform on AWS 자격증명, Google Cloud 자격증명, Microsoft Foundry 리소스 또는 Anthropic API 키. 장애 조치를 사용한 여러 업스트림이 지원됩니다.                                                                                                                                                                                                                                                                                                                                                                                            |
| HTTPS                        | 게이트웨이는 개발자 노트북과 로그인에 사용되는 모든 브라우저에서 `https://`를 통해 도달 가능해야 합니다; 게이트웨이는 동일한 리스너에서 장치 확인 페이지를 제공합니다. `listen.tls`를 통해 TLS 인증서를 제공하거나, TLS 종료 수신 대기 뒤에서 실행하고, 두 경우 모두 `listen.public_url`을 외부 원본으로 설정하세요. 일반 `http://` 원본은 게이트웨이 호스트가 루프백인 경우에만 허용됩니다: `localhost`, `127.0.0.1` 또는 `::1`.                                                                                                                                                                                                                                               |
| 개인 네트워크 주소                   | `/login`에서 Claude Code는 게이트웨이의 호스트명 또는 IP 주소가 개인 주소로만 확인되도록 요구합니다: RFC 1918, 링크 로컬, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7` 또는 루프백. 호스팅하는 게이트웨이의 경우 선언한 블록 외의 모든 공개 주소는 거부됩니다; 배포 가이드의 [위협 모델](/docs/ko/claude-apps-gateway-deploy#threat-model-summary)을 참조하세요. 개발자 머신이 HTTPS를 회사 프록시를 통해 라우팅하는 경우, 로그인은 프록시 호스트도 개인 주소로 확인되도록 요구합니다; 그렇지 않으면 게이트웨이 호스트를 `NO_PROXY`에 추가하여 CLI가 직접 연결하도록 하세요. 내부 네트워크가 조직이 소유한 공개 IPv4 공간에서 번호가 지정된 경우 [해당 블록을 선언](#allow-a-gateway-on-public-address-space-you-own)하여 `/login`이 거기서 게이트웨이를 수락하도록 하세요. |
| Linux 런타임                    | 게이트웨이 서버는 네이티브 Linux 바이너리에서만 실행됩니다. macOS는 로컬 개발에 작동합니다. Windows는 서버 플랫폼으로 지원되지 않습니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h3 id="steps">
  단계
</h3>

<Steps>
  <Step title="IdP에서 OAuth 클라이언트 등록">
    게이트웨이의 호스트명을 먼저 결정하세요. 리다이렉트 URI가 일치해야 하기 때문입니다. 새 OIDC 웹 애플리케이션을 만들고 리다이렉트 URI를 `https://claude-gateway.<your-domain>/oauth/callback`으로 설정하세요. 여기서 호스트는 3단계에서 [`listen.public_url`](/docs/ko/claude-apps-gateway-config#listen)로 설정하는 동일한 값입니다. `client_id` 및 `client_secret`을 기록하세요. IdP별 지침은 [ID 제공자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)에 있습니다.
  </Step>

  <Step title="PostgreSQL 데이터베이스 프로비저닝">
    가장 작은 관리 계층을 포함한 모든 Postgres 14 이상이 작동합니다. 게이트웨이는 부팅 시 자체 스키마 마이그레이션을 실행하므로 데이터베이스 역할은 테이블을 생성하고 변경할 권한이 필요합니다; [`store`](/docs/ko/claude-apps-gateway-config#store)를 참조하세요.
  </Step>

  <Step title="gateway.yaml 작성">
    비밀은 `${ENV_VAR}` 확장을 통해 읽혀지므로 파일 자체는 버전 제어에 있을 수 있습니다. `/login`이 공개 주소를 거부하기 때문에 개인 IP로 확인되는 `public_url` 호스트명을 사용하세요. 최소 구성에는 5개 섹션이 있고, 다른 모든 필드는 기본값을 가집니다:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # 호스트가 루프백 주소가 아닌 경우 필수. IdP
      # redirect_uri 및 검색 문서에 사용됨.
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # /.well-known/openid-configuration을 제공해야 함
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # 조직 외부의 id_tokens 거부
      userinfo_fallback: true                  # email/groups를 생략하는 IdP의 경우; 그 외에는 무해함

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # IdP 프로비저닝 해제 시 취소 지연도 제한

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # 관리 Postgres의 경우 ?sslmode=require 추가

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # 비어있음: AWS 기본 자격증명 체인
    # (IRSA, EC2/ECS 작업 역할, 환경 변수, ~/.aws)

    # 모델은 업스트림별로 자동으로 변환됩니다. 기본 제공 카탈로그
    # claude-opus-4-8을 us.anthropic.claude-opus-4-8으로 매핑하고 모든
    # Bedrock 지원 Claude 모델에 대해 동일하게 합니다. false로 설정하고 `models:` 목록을 추가하여
    # 특정 모델만 노출하세요.
    auto_include_builtin_models: true
    ```

    이 구성은 기본 Bedrock 모델 카탈로그로 작동하는 로그인 루프에 충분합니다. 실행되면 [`managed.policies`](/docs/ko/claude-apps-gateway-config#managed)를 통해 그룹별 RBAC 및 관리 설정을 추가하고, [`telemetry`](/docs/ko/claude-apps-gateway-config#telemetry)를 통해 텔레메트리 팬아웃을 추가하고, [`models`](/docs/ko/claude-apps-gateway-config#models)를 통해 다중 업스트림 장애 조치, 프로비저닝된 처리량 ARN 또는 미국 이외 지역을 추가하세요.

    <Note>
      Amazon Bedrock 업스트림은 `bedrock:InvokeModel` 및 `bedrock:InvokeModelWithResponseStream`을 `inference-profile/us.anthropic.*` ARN과 기본 `foundation-model/anthropic.*` ARN 모두에 가진 AWS 주체가 필요합니다. 또한 Bedrock 콘솔의 모델 카탈로그에서 계정에 대해 제출된 Anthropic의 일회성 사용 사례 양식이 필요합니다.

      정적 키보다는 EKS의 IRSA, ECS 작업 역할 또는 EC2 인스턴스 프로필을 사용하여 자격증명을 제공하세요. [`upstreams` 참조](/docs/ko/claude-apps-gateway-config#upstreams)는 전체 IAM 세부사항, 클라우드 간 자격증명 매트릭스, 다른 제공자의 `auth` 블록을 가집니다.
    </Note>
  </Step>

  <Step title="실행">
    [이미지 요구사항](/docs/ko/claude-apps-gateway-deploy#container-image)을 충족하는 `claude` 바이너리 주위에 컨테이너 이미지를 빌드한 다음 Postgres와 함께 실행하세요. Compose 파일은 이미지를 `registry.example.com/claude-gateway:2.1.198`로 참조합니다; 자신의 레지스트리 및 이미지 태그를 대체하세요:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: registry.example.com/claude-gateway:2.1.198
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # AWS 자격증명: 프로덕션에서는 이를 생략하고 인스턴스
          # 역할을 사용하세요. 로컬 Compose 테스트의 경우 자신의 것을 전달하세요:
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    게이트웨이는 구성을 읽고, Postgres에 연결하고 스키마 마이그레이션을 적용하고, IdP에 대해 OIDC 검색을 실행하고, 업스트림 클라이언트를 빌드하고, 수신 대기를 시작하는 단일 Linux 바이너리입니다. 부팅은 구성, Postgres 연결, OIDC 검색 및 업스트림 클라이언트 구성에 대해 실패 폐쇄됩니다. 이 중 하나가 도달 불가능하거나 잘못 구성된 경우, 게이트웨이는 저하된 상태에서 트래픽을 제공하는 대신 오류로 종료됩니다.

    성공적인 부팅은 Amazon Bedrock 및 Google Cloud의 Agent Platform 인스턴스 자격증명이 부팅 시가 아닌 첫 요청에서 확인되기 때문에 추론 경로를 검증하지 않습니다.

    부팅 시퀀스에 대해 stderr를 감시하세요. 로그 라인은 `[gateway] <timestamp> <level> <message>` 형식을 사용하고, 감사 이벤트는 `evt` 필드가 있는 단일 라인 JSON이며, 시작 배너(아래 생략됨)는 마이그레이션과 수신 대기 라인 사이에 인쇄됩니다. 신규 데이터베이스는 스키마 마이그레이션당 하나의 `migration N applied` 라인을 인쇄합니다; 이미 마이그레이션된 데이터베이스는 없습니다. 순서대로 다음을 볼 수 있습니다:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.395Z info waiting for migration lock (another replica may be migrating; check pg_locks for key 6775156 if this persists)
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    …
    [gateway] 2026-06-10T17:03:21.431Z info migration 6 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    게이트웨이는 또한 `access_control.allow_cidrs`가 비어있다는 경고를 기록합니다. 게이트웨이가 제공하는 클라이언트 주소를 제한하는 것이 없기 때문에 여기서는 예상됩니다. [`access_control` 참조](/docs/ko/claude-apps-gateway-config#http-tuning)는 권장 범위를 가집니다.

    부팅이 `claude gateway listening on` 라인 전에 종료되면, stderr의 마지막 라인이 문제를 이름 지정합니다:

    * 도달 불가능한 Postgres
    * DDL 권한이 없는 Postgres 역할
    * 도달 불가능하거나 유효하지 않은 OIDC 검색 문서
    * 잘못된 필드 경로가 있는 구성 스키마 위반

    수정하고 다시 시작하세요.

    이미 TLS 종료 수신 대기가 있는 경우, Compose를 건너뛰고 `claude gateway --config gateway.yaml`로 바이너리를 직접 실행하세요. `public_url`을 수신 대기 원본으로 설정하고 `listen`을 루프백 또는 클러스터 내부 주소에 바인드하세요.
  </Step>

  <Step title="인증 표면 확인">
    세 가지 확인은 게이트웨이가 개발자에게 전달하기 전에 실제 사용자를 인증할 수 있음을 확인합니다.

    예제는 게이트웨이의 공개 URL을 사용합니다; 수신 대기가 없는 로컬 Compose 설정의 경우, 처음 두 확인에서 `http://localhost:8080`을 대체하세요. 세 번째 확인은 `verification_uri_complete`를 열며, 이는 `public_url`에서 빌드되므로, 로컬 Compose의 경우 `gateway.yaml`에서 `public_url: http://localhost:8080`을 설정하고, 게이트웨이가 IdP `redirect_uri`를 `public_url`에서 빌드하기 때문에 1단계의 OAuth 클라이언트에 두 번째 리다이렉트 URI로 `http://localhost:8080/oauth/callback`을 추가하세요. 확인 링크는 로컬 브라우저에서 열립니다.

    Windows PowerShell에서 `curl.exe`를 실행하세요; 일반 `curl`은 `Invoke-WebRequest`의 별칭이며 이 플래그를 거부합니다.

    먼저 검색 문서를 가져오세요. 이는 게이트웨이가 작동 중이고, 구성이 유효하며, 모든 부팅 확인이 통과했음을 확인합니다:

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    응답은 `response_types_supported` 및 `scopes_supported`와 같은 추가 필드를 포함합니다.

    둘째, 장치 인증을 요청하세요. 이는 장치 로그인 흐름이 작동하고 Postgres가 도달 가능하고 쓰기 가능함을 확인합니다:

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    셋째, 브라우저 다리를 테스트하세요. 브라우저에서 `verification_uri_complete`를 열고 코드를 확인하세요. IdP의 로그인 페이지로 리다이렉트되어야 하고, 로그인 후 게이트웨이로 돌아와 로그인 확인을 받아야 합니다.

    첫 번째 실패한 확인을 사용하여 문제를 찾으세요:

    * **첫 번째 확인 실패**: 부팅이 완료되지 않음; stderr 확인
    * **두 번째 확인 실패**: Postgres가 게이트웨이에서 도달 불가능하거나 역할이 쓸 수 없음; 연결 문자열 및 권한 확인
    * **세 번째 확인이 IdP에 도달하지 않음**: IdP의 리다이렉트 URI가 `https://<gateway>/oauth/callback`과 정확히 일치하는지 확인
    * **세 번째 확인이 IdP에 도달하지만 오류로 반송됨**: 게이트웨이의 감사 로그를 읽으세요. 이는 `email domain not allowed`와 같은 이유로 모든 인증 거부를 기록합니다.
  </Step>

  <Step title="개발자 로그인">
    이 마지막 단계는 서버가 아닌 개발자 머신에서 발생합니다. 해당 머신의 [관리 설정 파일](/docs/ko/managed-settings#delivery-mechanisms)에서 `forceLoginMethod`를 `"gateway"`로 설정하고 `forceLoginGatewayUrl`을 게이트웨이의 `public_url`로 설정한 다음 `/login`을 실행하고, **Cloud gateway** 화면에서 Enter를 누르고, 브라우저 로그인을 완료하세요. [게이트웨이 URL 설정](#set-the-gateway-url) 아래는 두 키를 모든 개발자 머신에 배포하는 것을 다룹니다.
  </Step>
</Steps>

<h2 id="connect-developers">
  개발자 연결
</h2>

개발자는 자신의 노트북에서 한 번의 브라우저 로그인으로 회사 업무 계정을 사용하여 연결합니다. 요청이 조직의 업스트림 자격증명을 사용하여 게이트웨이를 통해 모델로 가기 때문에 claude.ai 계정, API 키 또는 구독이 필요하지 않습니다. 연결은 MDM을 통해 푸시하는 [클라이언트 측 관리 설정](/docs/ko/claude-apps-gateway-config#client-side-managed-settings)에 의해 구동되므로, 개발자 측에는 수동 설정이 없습니다; 이 섹션은 관리자가 구성하는 것을 다룹니다.

CLI는 첫 연결 시 게이트웨이의 TLS 리프 인증서를 지문 처리하고 호스트명별로 고정합니다. 로그인 중, 자동 세션 새로 고침 중, 관리 설정 가져오기 중에 해당 핀을 다시 확인하는 반면, 추론 요청은 핀 없이 표준 TLS 검증을 사용합니다. HTTPS 프록시를 통해 라우팅된 요청은 핀 검사를 건너뛰므로, 게이트웨이 호스트를 `NO_PROXY`에 추가하여 직접 연결을 유지하세요.

예상 SHA-256 지문을 게이트웨이 URL과 함께 게시하여 개발자가 비교할 것이 있도록 하세요. `/login` 프롬프트는 지문의 처음 16자를 소문자 16진수로 콜론 없이 표시합니다. 인증서 파일에서 해당 형식의 전체 지문을 인쇄하려면 다음을 실행하세요:

```bash theme={null}
openssl x509 -noout -fingerprint -sha256 -in cert.pem | cut -d= -f2 | tr -d : | tr 'A-F' 'a-f'
```

인증서가 회전하면 모든 개발자가 신뢰 프롬프트를 다시 보므로, 회전을 계획된 이벤트로 취급하고 지문을 다시 게시하세요. 게이트웨이 정책에 [승인이 필요한 설정](/docs/ko/server-managed-settings#security-approval-dialogs)이 포함되어 있으면, 개발자는 새 인증서를 수락한 후 승인 대화상자를 다시 보게 됩니다. Claude Code는 [승인 메모리](/docs/ko/server-managed-settings#approval-memory)를 고정된 인증서에 키 지정하기 때문입니다.

게이트웨이는 토큰 응답에서 선택적 `email` 필드를 반환하여 로그인이 사용한 계정의 이름을 지정할 수 있습니다. 이렇게 하면, 개발자는 Claude Code가 자격증명을 저장하기 전에 계정을 확인합니다. 확인된 로그인 후, `/status`는 계정을 표시합니다.

확인은 개발자 머신의 Claude Code v2.1.275 이상이 필요합니다; 해당 버전 아래의 클라이언트는 필드를 무시합니다. `claude` 바이너리의 게이트웨이 서버는 필드를 반환하지 않으므로, 해당 로그인은 확인 없이 완료됩니다.

개발자가 로그인하면, [모델 선택기](/docs/ko/model-config)는 개발자의 `availableModels` 허용 목록의 모델을 표시합니다. 관리 설정은 시작 시 적용되고 매시간 새로 고쳐지며, 텔레메트리는 수집기로 라우팅됩니다.

세션은 `ttl_hours` 만료 전에 자동으로 새로 고쳐집니다. IdP 프로비저닝 해제 후 새로 고침이 실패하면, Claude Code는 개발자에게 다시 로그인하도록 프롬프트합니다.

<h3 id="set-the-gateway-url">
  게이트웨이 URL 설정
</h3>

MDM을 통해 또는 디스크에 직접 배포하는 OS별 [관리 설정 파일](/docs/ko/managed-settings#delivery-mechanisms)에 세 개의 키가 들어갑니다. `forceLoginMethod`와 `forceLoginGatewayUrl`은 URL이 채워진 **Cloud gateway** 화면에서 직접 `/login`을 열고, `parentSettingsBehavior: "merge"`는 Claude Desktop이 게이트웨이의 송신 허용 목록을 시작하는 Claude Code 세션에 전달하도록 하며, 이는 [Claude Desktop 세션에 정책 전달](#deliver-policy-to-claude-desktop-sessions)에서 설명합니다:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

개발자는 Enter를 눌러 연결합니다. [첫 연결 TLS 지문 프롬프트](#connect-developers)는 여전히 나타납니다. 파일이 머신에 있으면, 게이트웨이 로그인을 완료하지 않은 개발자는 [Administrator policy requires a Cloud gateway sign-in](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)에서 설명한 메시지 중 하나를 봅니다. `CLAUDE_CODE_USE_BEDROCK` 같은 환경 변수를 통해 클라우드 제공자를 선택하는 개발자는 게이트웨이 로그인이 필요하지 않습니다.

개발자가 수동으로 이를 설정할 수 없습니다. 로그인 선택기에는 게이트웨이 옵션이 없으며, `forceLoginGatewayUrl`은 개발자의 자신의 설정 파일에서 무시됩니다. `forceLoginMethod`만, URL 없이, 개발자를 "IT 관리자에게 문의하세요" 메시지에 남깁니다. 로그인 키는 머신으로 푸시하는 파일에 속하며, 게이트웨이의 `managed.policies[].cli` 블록에는 속하지 않습니다. 이 블록은 이미 연결된 클라이언트에만 도달합니다.

<h3 id="allow-a-gateway-on-public-address-space-you-own">
  공개 주소 공간에서 소유한 게이트웨이 허용
</h3>

일부 조직은 자신이 소유한 공개 IPv4 블록(예: 통신사의 자신의 주소 공간 또는 레거시 `/8`)에서 내부 네트워크를 번호 지정하므로, 게이트웨이는 개인 주소를 가질 수 없습니다. 이러한 블록을 `gatewayInternalNetworks` 관리 설정에 나열하세요. `/login`은 개발자의 머신이 동일한 블록 내의 주소에서 연결할 때 나열된 블록 내의 게이트웨이를 수락합니다. 이는 개발자 머신의 Claude Code v2.1.268 이상이 필요합니다; 이전 버전은 키를 무시하고 개인 주소 규칙을 적용합니다.

<Warning>
  `gatewayInternalNetworks`는 공개 주소 공간에서 번호 지정되는 내부 네트워크용입니다. 게이트웨이를 인터넷에 노출하는 것을 안전하게 만들지 않습니다: 신뢰할 수 있는 게이트웨이는 개발자 머신에서 명령을 실행하는 설정을 푸시할 수 있습니다.

  방화벽 또는 로드 밸런서 규칙으로 게이트웨이를 네트워크 외부에서 도달할 수 없게 유지하세요. 게이트웨이의 [`access_control.allow_cidrs`](/docs/ko/claude-apps-gateway-config#http-tuning)를 여기에서 선언한 동일한 블록으로 설정하여 게이트웨이 자체가 다른 곳의 클라이언트를 거부하도록 하세요. 로드 밸런서 또는 수신 뒤에서, 게이트웨이가 개발자의 주소가 아닌 프론트 엔드의 자신의 주소에 대해 `allow_cidrs`를 일치시키기 때문에 `listen.trusted_proxies`를 해당 프론트 엔드로 설정하세요.
</Warning>

키를 로그인 키와 동일한 관리 설정 소스에 추가하세요: 관리 설정 파일, MDM 프로필 또는 레지스트리 정책. Claude Code는 사용자, 프로젝트, 서버 관리 설정에서 이를 무시합니다.

이 예제는 하나의 블록을 선언합니다. `203.0.113.0/24`를 자신의 블록으로 바꾸세요. 이는 문서 범위이며, Claude Code는 이를 거부합니다.

```json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Claude Code는 게이트웨이에 연결하기 전에 `/login`에서 목록을 검증합니다:

* 각 항목은 첫 주소와 `/8`에서 `/32` 사이의 접두사로 작성된 IPv4 블록입니다.
* 목록은 최대 4개의 블록을 보유하며, 2개는 겹치지 않습니다.
* 블록은 개인 주소 공간과 겹치지 않습니다: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `127.0.0.0/8`, `169.254.0.0/16`, `100.64.0.0/10`. `/login`은 이미 이 키 없이 거기에서 게이트웨이를 수락합니다.
* 블록은 조직의 네트워크가 될 수 없는 공간과 겹치지 않습니다: VPN 및 NAT64 클라이언트가 로컬 주소로 보유하는 `198.18.0.0/15` 및 `192.0.0.0/24`; 문서 범위 `192.0.2.0/24`, `198.51.100.0/24`, `203.0.113.0/24`; 예약된 범위 `0.0.0.0/8`, `192.88.99.0/24`, 멀티캐스트 `224.0.0.0/4`. `240.0.0.0/4` 내의 블록을 선언할 수 있으며, 일부 대규모 네트워크는 이를 내부 유니캐스트 공간으로 사용합니다.

`managed-settings.json` 및 해당 `managed-settings.d/` 드롭인 파일의 블록은 하나의 목록으로 결합되며, 이러한 제한은 결합된 목록에 적용됩니다. 블록을 좁히려면, 드롭인에서 두 번째 겹치는 항목을 추가하는 대신 항목을 바꾸세요; `/login`은 겹침을 거부합니다.

항목이 규칙을 위반하거나 값이 문자열 목록이 아니면, Claude Code는 해당 머신에서 모든 새 게이트웨이 로그인을 거부하고 메시지에서 문제를 지정합니다. 개인 주소의 게이트웨이로 로그인도 실패하며, 기존 로그인은 계속 작동합니다. 배포하기 전에 한 머신에서 값을 시도하세요. Claude Code는 또한 잘못 입력된 값을 [보고하는 잘못된 관리 설정](/docs/ko/managed-settings#keys-that-fail-closed) 중에 나열합니다.

유효한 목록으로, `/login`은 주소가 나열된 블록 내에 있는 게이트웨이에 3가지 검사를 적용합니다:

* 게이트웨이의 호스트명이 확인하는 모든 주소는 해당 하나의 블록 내에 있습니다. Claude Code는 개인 및 IPv6 주소를 포함하여 블록 외부에도 레코드가 있는 이름을 거부합니다.
* 개발자의 머신은 동일한 블록 내의 주소에서 연결합니다. Claude Code는 NAT 뒤, 컨테이너 또는 WSL2 내, 주소 풀이 블록 외부에 있는 VPN의 머신을 거부하고, 머신이 연결한 주소를 지정합니다.
* 연결은 직접입니다. `HTTPS_PROXY`가 게이트웨이 호스트에 적용되면, `/login`은 거부하고 추가할 `NO_PROXY` 항목을 지정합니다.

3가지 모두 통과하면, [신뢰 프롬프트](#connect-developers)는 머신의 주소, 게이트웨이의 주소, 둘 다를 포함하는 선언된 블록을 지정하는 줄을 추가합니다.

키는 다른 게이트웨이에 대해 아무것도 변경하지 않습니다: 개인 주소의 게이트웨이로 로그인은 이전처럼 작동하며, 모든 나열된 블록 외부의 공개 주소의 게이트웨이로 로그인은 이전처럼 거부됩니다.

선언된 블록은 누가 로그인할 수 있는지를 좁히지만 머신이 어디에 있는지를 증명하지 않으므로, 조직이 제어하는 주소 공간만 선언하세요. 클라우드 제공자의 공개 범위와 같이 다른 테넌트와 공유되는 블록은 이를 통과하는 누구나 동일한 검사를 통과하도록 합니다.

<h3 id="deliver-policy-to-claude-desktop-sessions">
  Claude Desktop 세션에 정책 전달
</h3>

Claude Desktop은 Cowork 및 Code 탭과 Chat 탭(활성화한 경우)을 포함된 Claude Code 세션에서 실행하고 해당 모델 요청을 게이트웨이를 통해 보냅니다. 게이트웨이가 `/user/bootstrap`에서 제공하는 구성에서 구축된 정책을 각 세션에 전달합니다: 모델 허용 목록, 비활성화된 도구, 일치하는 정책의 `cli` 블록에서 파생된 송신 허용 목록, 그리고 [`desktop` 오버레이](/docs/ko/claude-apps-gateway-config#claude-desktop-overlay).

hooks, `env`, `Bash(npm *)` 같은 범위가 지정된 권한 규칙 등 다른 `cli` 키는 `/login`을 통해 로그인하는 클라이언트에만 도달합니다. Claude Desktop은 게이트웨이 URL을 자신의 관리 구성에서 읽고 [게이트웨이 URL 설정](#set-the-gateway-url)의 `forceLoginMethod` 및 `forceLoginGatewayUrl` 키와 별개인 자신의 흐름으로 로그인합니다.

시작 프로세스에서 전달된 설정은 부모 설정입니다. Claude Code는 정책을 전달하는 [소스](/docs/ko/managed-settings#which-managed-source-claude-code-uses)가 `parentSettingsBehavior: "merge"`를 설정하지 않는 한, 관리 배포된 관리 소스가 있는 모든 머신에서 부모 설정을 무시합니다.

<h4 id="which-machines-need-the-opt-in">
  옵트인이 필요한 머신
</h4>

Claude Desktop만 실행하는 머신에는 옵트인이 필요합니다. Claude Desktop은 모델 목록과 비활성화된 도구 목록을 포함된 세션에 자체적으로 적용하지만, 송신 허용 목록은 `WebFetch` 도메인 규칙 및 샌드박스 네트워크 규칙 형태의 부모 설정으로만 도달합니다. 옵트인 없이, 이러한 세션은 송신 제한 없이 실행되며, 아무것도 경고하지 않습니다. 게이트웨이는 여전히 정책이 부여하지 않는 모델에 대한 추론 요청을 거부합니다.

개발자가 `/login`을 통해 로그인하는 머신에는 옵트인이 필요하지 않습니다; 각 Claude Code 세션은 게이트웨이에서 정책을 가져옵니다.

[`policyHelper`](/docs/ko/settings-reference#policyhelper)가 관리 설정을 제공하는 플릿은 이를 사용할 수 없습니다: Claude Code는 도우미의 출력에서만 관리 설정을 읽기 때문에 이러한 플릿에서 부모 설정을 병합하지 않습니다.

<h4 id="set-the-opt-in">
  옵트인 설정
</h4>

[게이트웨이 URL 설정](#set-the-gateway-url)에서 관리 설정 스니펫을 배포하고, 파일을 능가하는 모든 클라이언트 측 소스로 미러링한 다음, 확인하세요.

<Steps>
  <Step title="관리 설정 파일에 옵트인 배포">
    [위의 스니펫](#set-the-gateway-url)은 이미 `parentSettingsBehavior: "merge"`를 포함하므로, 머신으로 푸시하는 파일이 이를 전달합니다.
  </Step>

  <Step title="파일을 능가하는 모든 소스로 스니펫 미러링">
    Claude Code는 [선택된 소스](/docs/ko/managed-settings#which-managed-source-claude-code-uses)에서만 `parentSettingsBehavior`를 읽습니다. 클라이언트 측 소스에 정책 키를 추가하면 해당 소스가 선택된 소스가 될 수 있으므로, `parentSettingsBehavior`만이 아닌 전체 스니펫을 미러링하세요. [클라이언트 측 관리 설정](/docs/ko/claude-apps-gateway-config#client-side-managed-settings)은 Group Policy 또는 구성 프로필을 통해 정책을 전달하는 플릿을 다룹니다. macOS의 관리 기본 설정 plist 또는 Windows의 HKLM 정책은 `managed-settings.json` 파일을 능가하며, 게이트웨이의 자체 원격 관리 설정은 둘 다를 능가하므로, 게이트웨이에 로그인하는 머신에서도 게이트웨이 정책의 [`cli` 블록](/docs/ko/claude-apps-gateway-config#managed)에서 `parentSettingsBehavior`를 설정하세요.
  </Step>

  <Step title="선택된 소스 확인">
    Claude Desktop만 실행하는 머신에서 Agent SDK의 [`resolveSettings()`](/docs/ko/agent-sdk/typescript#resolvesettings)를 호출하고 `sources` 목록의 `managed` 항목에서 `policyOrigin`을 읽으세요. 값은 선택된 클라이언트 측 소스인 `plist`, `hklm` 또는 `file`의 이름을 지정하며, 이는 스니펫을 전달해야 하는 소스입니다. Claude Desktop의 포함된 세션은 게이트웨이 정책을 가져오지 않으므로, 게이트웨이의 `cli` 블록은 이들에 대해 선택된 소스로 계산되지 않습니다.
  </Step>
</Steps>

<h3 id="restrict-parent-settings">
  부모 설정 제한
</h3>

`parentSettingsBehavior: "merge"`를 배포하면, Claude Desktop뿐만 아니라 Agent SDK 애플리케이션 또는 IDE 확장도 Claude Code를 시작하는 모든 호스트 프로세스가 부모 설정을 제공할 수 있습니다.

Claude Code는 부모 설정을 제한적 키의 허용 목록에 대해 필터링하지만, 일부 허용된 키는 제한하기보다는 액세스를 부여할 수 있습니다. `allowManaged*Only` 잠금을 설정하지 않으면, 호스트에서 제공한 권한 허용 규칙 및 샌드박스 허용 목록이 여전히 적용됩니다. 정책의 거부 및 요청 규칙은 어느 쪽이든 적용됩니다; [이들은 모든 허용 규칙 전에 평가됩니다](/docs/ko/permissions#manage-permissions).

Claude Code는 부모 제공 [`sandbox.credentials`](/docs/ko/settings-reference#sandbox-credentials) 항목을 제거된 형태로 전달합니다:

* **`deny` 항목**: `path` 또는 `name`과 모드만으로 전달됩니다.
* **[`mode: mask`](/docs/ko/sandboxing#mask-credential-files)가 있는 파일 항목**: 전체 파일 마스크로 센티널만 전달되며, `injectHosts`는 빈 목록이므로, 프록시는 모든 플랫폼에서 부모 제공 항목에 대해 실제 값을 대체하지 않습니다. 모든 구조화된 마스킹 필드도 삭제되므로, 부모 제공 추출 패턴은 다른 소스가 동일한 경로에 대해 설정한 더 엄격한 마스크를 대체할 수 없습니다.
* **`mode: mask`가 있는 `envVars` 항목**: 전달되지 않습니다. `deny`는 부모 채널이 `envVars` 항목을 통해 표현할 수 있는 유일한 제한입니다.
* **[`awsPairs` 및 `sigv4`](/docs/ko/sandboxing#re-sign-aws-requests)**: 제한만 전달됩니다. `sigv4`에서는 `deny` 값만 유지되며, 부모가 `sigv4` 블록을 정의하면 세 가지 요청 형식 모두 `streaming`, `presigned`, `sigv4a`를 `deny`로 고정합니다. `awsPairs` 쌍은 다시 서명할 수 있는 형태로 전달되지 않습니다; 기존 AWS 변수 중 하나의 이름을 지정하는 쌍은 `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_SESSION_TOKEN`의 자동 쌍 지정을 억제된 상태로 유지하는 비활성 항목으로 대체됩니다.

<h4 id="deploy-the-locks">
  잠금 배포
</h4>

부모 설정을 필터가 지원하는 제한만큼 가깝게 유지하려면, 다섯 개의 `allowManaged*Only` 잠금과 이들이 관리하는 허용 목록을 모두 병합 옵트인과 동일한 소스에 추가하세요:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge",
  "allowManagedPermissionRulesOnly": true,
  "allowManagedMcpServersOnly": true,
  "allowManagedHooksOnly": true,
  "allowedMcpServers": [{ "serverUrl": "https://mcp.internal.example.com/*" }],
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    },
    "filesystem": {
      "allowManagedReadPathsOnly": true,
      "denyRead": ["~/"],
      "allowRead": ["~/projects"]
    }
  }
}
```

OS 정책(예: HKLM 레지스트리 정책 또는 관리 기본 설정 plist)은 이 파일을 능가하므로, 파일 대신 전체 스니펫을 통해 전달하세요. 게이트웨이의 원격 관리 설정은 OS 정책 및 파일 소스를 능가하지만 연결된 클라이언트에만 도달합니다. 잠금, 허용 목록, 병합 옵트인을 정책의 [`cli` 블록](/docs/ko/claude-apps-gateway-config#managed)으로 미러링하고 이 파일을 배포된 상태로 유지하세요. 연결되지 않은 머신(Claude Desktop만 실행하는 머신 포함)은 파일에서만 정책을 가져옵니다.

<h4 id="lock-behavior-across-sources">
  소스 전체의 잠금 동작
</h4>

한 잠금을 설정해도 다른 잠금을 제한하지 않습니다; 각 키는 [설정 참조](/docs/ko/settings-reference#all-settings)에 문서화되어 있습니다. 우승자 아래의 관리 소스에서, 두 샌드박스 잠금은 여전히 적용되며, `allowManagedPermissionRulesOnly`는 여전히 부모 제공 허용 규칙 및 `additionalDirectories`를 차단합니다. Claude Code v2.1.273 이상에서, MCP 서버 잠금도 우승자 아래의 소스에서 적용되며, 이것이 켜져 있는 동안 관리 `allowedMcpServers` 목록은 하나를 설정하는 가장 높은 우선순위 관리 소스에서 나옵니다.

hooks 잠금과 `allowManagedPermissionRulesOnly`의 개발자 자신의 규칙에 대한 영향은 기본적으로 우승 소스가 필요합니다; [Claude Code가 관리 소스를 결합하는 방법](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)의 `managedSourcesBehavior` 병합 옵트인에서, Claude Code는 모든 잠금에 대해 모든 소스가 설정하는 가장 엄격한 값을 적용합니다. [`policyHelper`](/docs/ko/settings-reference#policyhelper) 플릿에서, Claude Code는 도우미의 출력에서만 잠금을 읽습니다.

각 잠금은 Claude Code가 해당 설정에 대한 개발자의 자신의 항목을 무시하도록 하므로, 조직의 허용 목록을 잠금 옆에 포함하세요:

* **네트워크 도메인**: 빈 관리 도메인 목록으로 잠그면 모든 샌드박스 아웃바운드 트래픽이 차단됩니다.
* **MCP 서버**: 관리되거나 부모 제공 `allowedMcpServers` 없이 잠그면 `deniedMcpServers`가 차단하지 않는 모든 서버가 로드됩니다.
* **읽기 경로**: `allowRead` 항목은 `denyRead` 영역 내의 경로만 다시 허용하므로, 관리 `denyRead`와 쌍을 이루세요.

<h4 id="settings-the-locks-don’t-cover">
  잠금이 다루지 않는 설정
</h4>

여섯 개의 부모 제공 설정은 다섯 개의 잠금이 모두 설정되어도 필터를 통과합니다. 기본 첫 번째 우승 설정에서, 부모를 차단하는 관리 값은 가장 높은 우선순위 관리 소스의 값입니다. [MCP 서버 잠금](#lock-behavior-across-sources)이 켜져 있는 동안 `allowedMcpServers` 제외. `managedSourcesBehavior` 병합 옵트인에서, [Claude Code가 관리 소스를 결합하는 방법](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)은 대신 어느 소스의 값이 적용되는지 말합니다.

* **`forceLoginOrgUUID`**: 가장 높은 우선순위 관리 소스가 조직 UUID를 설정하지 않을 때 Claude Code는 부모 제공 값을 준수합니다. 게이트웨이 로그인은 이 키를 확인하지 않으므로, 첫 번째 당사자 Anthropic 로그인도 사용하는 플릿에만 중요합니다. 가장 높은 우선순위 관리 소스의 조직 UUID는 부모의 값을 차단하며 Claude Code가 적용하는 값이므로, 거기에 `forceLoginOrgUUID`를 설정하세요.
* **`allowedMcpServers`**: 가장 높은 우선순위 관리 소스가 허용 목록을 설정하지 않을 때 Claude Code는 부모 제공 허용 목록을 준수하며, `allowManagedMcpServersOnly`는 이를 차단하지 않습니다. 잠금은 우승 목록을 관리 값으로 적용하기 때문입니다. 가장 높은 우선순위 관리 소스의 목록은 부모의 목록을 차단하며 Claude Code가 적용하는 목록이므로, 잠금 옆에 거기에 `allowedMcpServers`를 설정하세요. v2.1.223 이전에는, 모든 관리 소스의 어느 키에 대한 값이든 부모의 값을 차단했습니다.
* **`availableModels`**: Claude Code는 우승 관리 소스가 모델 목록을 설정하지 않을 때 부모 제공 모델 목록을 준수합니다. 플릿이 모델을 제한하면, 우승 소스에 `availableModels`를 설정하세요.
* **`strictKnownMarketplaces`**: Claude Code는 우승 관리 소스가 플러그인 마켓플레이스 허용 목록을 설정하지 않을 때 부모 제공 플러그인 마켓플레이스 허용 목록을 준수합니다. 플릿이 마켓플레이스를 제한하면, 우승 소스에 `strictKnownMarketplaces`를 설정하세요. Claude Code v2.1.282 이상이 필요합니다.
* **`blockedMarketplaces`**: 부모 제공 마켓플레이스 차단 목록은 통과하고 관리 소스가 설정한 모든 차단 목록에 추가됩니다. 차단 목록은 더 제한할 수만 있기 때문입니다. Claude Code v2.1.282 이상이 필요합니다.
* **`strictPluginOnlyCustomization`**: 이 키는 모든 잠금과 관계없이 필터를 통과하며, Claude Code가 개발자의 자신의 사용자 정의(보호 hooks 포함)를 무시하도록 합니다. 이를 차단하는 잠금이 없습니다.

<h3 id="connect-claude-desktop">
  Claude Desktop 연결
</h3>

[Claude Desktop](/docs/ko/desktop)은 다른 MDM 키를 통해 동일한 게이트웨이에 연결합니다: Claude Desktop의 [관리 구성](https://claude.com/docs/third-party/claude-desktop/configuration)에서 `bootstrapUrl`을 `<listen.public_url>/user/bootstrap`으로 설정하고, 사용자의 정책을 `desktop` 키로 옵트인하세요. [Claude Desktop 오버레이](/docs/ko/claude-apps-gateway-config#claude-desktop-overlay)는 둘 다를 다룹니다. 게이트웨이 서버에서 Claude Code v2.1.203 이상이 필요합니다.

Claude Desktop은 동일한 브라우저 SSO 단계로 게이트웨이의 ID 제공자를 통해 개발자에게 로그인하고, Anthropic 대신 게이트웨이에서 구성을 가져옵니다. 모델 액세스 및 정책은 CLI와 동일한 그룹별 규칙을 따릅니다. CLI와 Claude Desktop을 모두 사용하는 개발자는 각각에 별도로 로그인합니다; 게이트웨이 세션은 이들 사이에 공유되지 않습니다.

연결되면, Claude Desktop은 활성화된 모든 탭에서 모델 요청을 게이트웨이를 통해 보냅니다. 기본적으로 Cowork 및 Code 탭을 표시합니다. Chat 탭도 켜려면, Claude Desktop의 [관리 구성](https://claude.com/docs/third-party/claude-desktop/configuration)에서 `chatTabEnabled`를 `true`로 설정하거나, Claude Code v2.1.227 이상을 실행하는 게이트웨이의 정책 [`desktop` 블록](/docs/ko/claude-apps-gateway-config#claude-desktop-overlay)에서 설정하세요.

<h3 id="ci-pipelines-and-remote-machines">
  CI 파이프라인 및 원격 머신
</h3>

무인 파이프라인을 위한 서비스 토큰 흐름이 없습니다. 게이트웨이 로그인은 항상 브라우저 장치 흐름을 실행하므로, 로그인을 승인할 개발자가 없는 CI 작업은 인증할 수 없습니다; 제공자에 대해 직접 구성하세요.

개발자가 로그인하면, 해당 머신의 모든 Claude Code 세션은 게이트웨이 세션을 사용하며, 비대화형 `claude -p` 실행 및 Agent SDK에서 시작된 세션을 포함합니다. Claude Code는 [게이트웨이 정책](/docs/ko/claude-apps-gateway-config#managed)을 각각에 적용합니다.

장치 흐름은 폴링 CLI를 승인하는 브라우저에서 분리하므로, 디스플레이가 없는 원격 개발 상자도 작동합니다: 개발자는 원격 머신에서 SSH를 통해 `/login`을 실행하고 노트북의 브라우저에서 확인 링크를 엽니다.

<h3 id="whats-enforced-on-developers">
  개발자에게 적용되는 것
</h3>

이러한 보장은 `/login`을 통해 로그인한 모든 세션에 적용됩니다. Claude Desktop이 시작하는 포함된 세션은 [Claude Desktop 세션에 정책 전달](#deliver-policy-to-claude-desktop-sessions)에서 설명한 대로 정책을 가져오며, 텔레메트리 글머리 기호는 내보내기가 어디로 가는지 말합니다.

* **모델 액세스**: 정책이 부여하지 않는 모델에 대한 요청은 400을 반환하고, `/model` 선택기는 정책의 `availableModels` 허용 목록으로 필터링됩니다. 정책에서 [`enforceAvailableModels: true`](/docs/ko/model-config#default-model-behavior)를 설정하여 Default 옵션이 `availableModels` 내의 모델로 확인되도록 하세요. 없으면 Default는 선택 가능하게 유지되고 해당 모델이 부여되지 않으면 요청 시 거부됩니다.
* **텔레메트리 대상**: `/login`을 통해 로그인한 세션에서, CLI는 로컬로 설정된 `OTEL_EXPORTER_OTLP_ENDPOINT`와 관계없이 OTLP/HTTP 내보내기를 게이트웨이로 보내며, 정책이 [수집기를 엔드포인트로 지정](/docs/ko/claude-apps-gateway-config#export-directly-to-your-collector)하지 않는 한, 게이트웨이는 이들을 [`telemetry.forward_to`](/docs/ko/claude-apps-gateway-config#telemetry)의 대상으로 중계합니다.
  * [Claude Desktop이 시작하는](#connect-claude-desktop) 포함된 세션에서, CLI는 구성된 `OTEL_EXPORTER_OTLP_ENDPOINT`로 내보내기를 보냅니다. CLI는 해당 엔드포인트가 게이트웨이 자체를 가리킬 때만 이러한 내보내기에 게이트웨이 세션 토큰을 첨부합니다.
  * 신호에 대해 구성된 대상이 없으면, 게이트웨이는 이를 수락하고 폐기합니다.
  * 이미 Claude Code 텔레메트리를 직접 수집하면, 수집기를 `forward_to` 대상으로 추가하거나, 정책에서 이를 엔드포인트로 지정하여 중계를 건너뛰세요.
* **자격증명**: 게이트웨이 토큰은 세션의 유일한 자격증명입니다. [Anthropic 프로필](/docs/ko/authentication#anthropic-profiles-and-federation-credentials) 및 이전 claude.ai 로그인은 로그인 중에 무시되므로, 개발자는 먼저 claude.ai에서 로그아웃할 필요가 없습니다. 구성된 `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, 또는 `apiKeyHelper` 자격증명의 경우, [Administrator policy requires a Cloud gateway sign-in](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)을 참조하세요.
* **관리 설정**: 잠긴 키는 로컬로 재정의될 수 없습니다. CLI는 시작 시 정책을 적용하고 각 시간별 폴에서 변경을 적용하며, [다음 시작에만 적용되는 변경](/docs/ko/server-managed-settings#fetch-and-caching-behavior)은 제외합니다.
* **게이트웨이에 도달할 수 없는 상태에서 시작**: 로그인한 세션은 설정 없이 시작하는 대신 약 10초 후 시작 시 오류로 종료됩니다.
* **게이트웨이가 세션을 종료한 후 시작**: [실패 폐쇄 시작 적용](/docs/ko/server-managed-settings#enforce-fail-closed-startup)을 참조하여 어느 시작이 게이트웨이에서 로그아웃된 상태로 열리고 어느 시작이 게이트웨이가 `401`로 응답할 때 종료되는지 확인하세요.
* **프로비저닝 해제**: 사용자가 IdP에서 비활성화된 세션은 다음 새로 고침이 실패할 때 `ttl_hours` 내에 만료됩니다.
* **로그아웃**: `/logout`은 개발자의 머신에서 게이트웨이 자격증명을 삭제합니다.
  * 게이트웨이의 검색 문서가 게이트웨이 URL의 자신의 스킴, 호스트, 포트에서 `revocation_endpoint`를 광고할 때, `/logout`은 또한 저장된 토큰을 해당 엔드포인트로 보내므로 게이트웨이가 자신의 측에서 세션을 종료할 수 있습니다. 요청은 최선의 노력이므로, 엔드포인트가 응답하는지 여부와 관계없이 로그아웃은 개발자의 머신에서 완료됩니다. 취소는 개발자 머신의 Claude Code v2.1.275 이상이 필요합니다.
  * `claude` 바이너리의 게이트웨이 서버는 광고하지 않으므로, 로그아웃은 개발자의 머신에서만 세션을 종료합니다. 서버 측에서 세션을 강제로 종료하려면, [JWT 비밀 회전](/docs/ko/claude-apps-gateway-deploy#jwt-secret-rotation)을 참조하세요.

<h3 id="what-the-organization-can-see">
  조직이 볼 수 있는 것
</h3>

사용 현황 텔레메트리는 개발자의 신원, 토큰 수, 모델 및 지연 시간을 조직의 수집기로 전달합니다. 게이트웨이는 프롬프트 또는 완료 콘텐츠를 기록하거나 저장하지 않습니다. 명령 및 파일 경로를 포함할 수 있는 로그 및 추적과 같은 더 풍부한 텔레메트리를 수집할지 여부는 조직의 [대상별 선택](/docs/ko/claude-apps-gateway-config#telemetry)입니다.

<h2 id="availability-and-limitations">
  가용성 및 제한사항
</h2>

표는 개발자가 게이트웨이를 통해 연결할 때 작동하는 Claude Code 기능과 게이트웨이 서버 자체가 지원하는 것을 다룹니다. 지원되지 않는 경우, Notes 열은 대안을 제공합니다.

게이트웨이는 CLI가 모든 업스트림으로 보내는 [`anthropic-beta`](https://platform.claude.com/docs/ko/api/beta-headers) 값을 전달하므로, 운영자는 베타 허용 목록을 유지하지 않습니다. Amazon Bedrock의 경우 헤더를 무시하므로, 게이트웨이는 값을 요청 본문의 `anthropic_beta` 필드로 이동합니다; 다른 업스트림은 보낸 대로 헤더를 받습니다.

| 기능                                                                                                         | 상태      | 참고                                                                                                                                                                                                                                                                                                                             |
| ---------------------------------------------------------------------------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 추론 전달 (Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry, Anthropic) | 사용 가능   | 업스트림별 모델 변환 및 장애 조치 포함. Amazon Bedrock 업스트림은 `bedrock-runtime` 엔드포인트 및 AWS 기본 자격증명 체인을 사용합니다; Amazon Bedrock [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint)는 지원되는 업스트림이 아닙니다. [Claude Platform on AWS 업스트림](/docs/ko/claude-apps-gateway-config#claude-platform-on-aws)은 게이트웨이 서버에서 Claude Code v2.1.198 이상이 필요합니다.     |
| IdP 그룹별 모델 액세스 및 관리 설정                                                                                     | 사용 가능   | 모델 액세스는 서버 측에서 적용됩니다; 관리 설정은 IdP 그룹별로 전달되고 CLI에서 [관리 설정 계층](/docs/ko/settings#settings-precedence)에서 적용됩니다.                                                                                                                                                                                                                         |
| Claude Desktop                                                                                             | 옵트인 가능  | 게이트웨이는 정책이 [`desktop` 키로 옵트인](/docs/ko/claude-apps-gateway-config#claude-desktop-overlay)하면 `/user/bootstrap`에서 Claude Desktop의 구성을 제공하며, Claude Desktop은 Cowork 및 Code 탭에서, 그리고 Chat 탭에서 활성화할 때 게이트웨이를 통해 모델 요청을 보냅니다. Chat 탭을 켜려면 [Claude Desktop 연결](#connect-claude-desktop)을 참조하세요. 게이트웨이 서버에서 Claude Code v2.1.203 이상이 필요합니다. |
| 텔레메트리 팬아웃 (OTLP/HTTP)                                                                                      | 사용 가능   | 내보내기별 ID 스탬프; protobuf 및 JSON 인코딩 모두                                                                                                                                                                                                                                                                                           |
| OIDC 신원 제공자                                                                                                | 사용 가능   | 모든 OIDC 호환 IdP; 게이트웨이는 표준 OIDC 검색 및 인증 코드 흐름을 실행합니다. [신원 제공자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)에서 IdP별 구성을 참조하세요.                                                                                                                                                                                     |
| 사용자별 및 그룹별 지출 제한                                                                                           | 사용 가능   | [지출 제한](/docs/ko/claude-apps-gateway-spend-limits) 참조                                                                                                                                                                                                                                                                               |
| 서버 측 웹 검색                                                                                                  | 사용 불가능  | CLI는 게이트웨이가 라우팅하는 업스트림 제공자를 볼 수 없으므로 웹 검색 지원을 확인할 수 없고 게이트웨이 세션에서 WebSearch를 비활성화합니다.                                                                                                                                                                                                                                          |
| [Remote Control](/docs/ko/remote-control)                                                                       | 사용 불가능  | CLI는 [게이트웨이를 명명하는 오류](/docs/ko/errors#remote-control-requires-the-anthropic-api)를 표시합니다.                                                                                                                                                                                                                                            |
| [`/design-sync`](/docs/ko/commands#all-commands) 및 `/design-login`                                              | 사용 불가능  | 둘 다 claude.ai가 필요하며, CLI는 게이트웨이 세션에서 이를 연결하지 않으므로 두 명령 모두 나타나지 않습니다.                                                                                                                                                                                                                                                           |
| `/import` 및 `claude import`와 같은 기능 플래그 가져오기가 필요한 기능                                                        | 사용 불가능  | CLI는 게이트웨이 세션에서 플래그 가져오기를 건너뜁니다. [기능 플래그 가져오기가 필요한 기능](/docs/ko/env-vars#features-that-need-feature-flag-fetching)은 이것이 비활성화하는 것을 나열합니다.                                                                                                                                                                                            |
| 표준 프롬프트 캐싱                                                                                                 | 사용 가능   | 게이트웨이는 `cache_control` 중단점을 모든 업스트림으로 전달합니다. [캐시가 있는 위치](/docs/ko/prompt-caching#where-the-cache-lives)는 CLI가 표시하는 블록을 다룹니다. 여기에는 대화 중간에 추가하는 시스템 컨텍스트가 포함됩니다.                                                                                                                                                                      |
| 1시간 캐시 TTL                                                                                                 | 사용 불가능  | CLI는 게이트웨이 세션에서 확장 캐시 TTL 베타를 생략합니다. 게이트웨이가 라우팅할 수 있는 모든 업스트림이 1시간 TTL을 지원하지 않기 때문에, 게이트웨이를 통한 프롬프트 캐싱은 5분 TTL을 사용합니다; 베타 헤더 참고를 참조하세요.                                                                                                                                                                                        |
| Auto 모드                                                                                                    | 사용 가능   | [타사 제공자 규칙](/docs/ko/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry)을 따릅니다: 타사 제공자에서 적격인 모델만 사용할 수 있습니다. v2.1.207 이전에는 게이트웨이 세션의 auto 모드에서 `CLAUDE_CODE_ENABLE_AUTO_MODE=1` 설정이 필요했으며, 관리 정책 `env` 블록을 통해 전달 가능했습니다.                                                                                        |
| 글로벌 캐시 범위 및 토큰 효율적인 도구와 같은 자사 전용 최적화                                                                       | 사용 불가능  | CLI는 게이트웨이 세션에서 이를 활성화하지 않습니다; 베타 헤더 참고를 참조하세요.                                                                                                                                                                                                                                                                                |
| OTLP/gRPC                                                                                                  | 지원되지 않음 | HTTP를 통한 OTLP만                                                                                                                                                                                                                                                                                                                 |
| SAML, LDAP 및 기타 비 OIDC 인증                                                                                  | 지원되지 않음 | OIDC만. 필요한 경우 OIDC 브리지로 프론트                                                                                                                                                                                                                                                                                                    |
| 다중 테넌트 (여러 OIDC 발급자)                                                                                       | 지원되지 않음 | 게이트웨이당 하나의 발급자. 별도 인스턴스 실행                                                                                                                                                                                                                                                                                                     |
| Windows 서버                                                                                                 | 지원되지 않음 | Linux에 배포. 로컬 개발만 macOS                                                                                                                                                                                                                                                                                                        |
| Helm 차트                                                                                                    | 사용 불가능  | 게이트웨이는 표준 상태 비저장 배포로 실행됩니다; [배포 가이드](/docs/ko/claude-apps-gateway-deploy#kubernetes) 참조                                                                                                                                                                                                                                             |
| 관리 UI                                                                                                      | 사용 불가능  | 구성은 YAML 파일입니다; 변경하려면 다시 배포하세요.                                                                                                                                                                                                                                                                                                |

<h2 id="next-steps">
  다음 단계
</h2>

빠른 시작은 Docker Compose에서 실행되는 최소 구성을 남깁니다. 더 나아가려면:

* 예를 들어 그룹별 RBAC, 다중 업스트림 장애 조치 또는 텔레메트리 대상을 추가하여 `gateway.yaml`을 최소 구성 이상으로 확장하세요. [구성 참조](/docs/ko/claude-apps-gateway-config)는 모든 옵션을 다룹니다.
* Compose에서 Kubernetes 또는 Cloud Run의 프로덕션 배포로 이동하고, IdP를 올바르게 설정하고, 보안 모델을 검토하세요. [배포 및 운영 가이드](/docs/ko/claude-apps-gateway-deploy)는 IdP별 설정, 컨테이너 이미지 요구사항, 상태 프로브 및 문제 해결을 다룹니다.
* 개별 개발자 또는 그룹에 지출 상한을 설정하여 폭주 워크로드가 전체 약정을 소비할 수 없도록 하세요. [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)은 관리 API 및 적용 방식을 다룹니다.
* AWS의 완전한 작동 예제(ECS Fargate 또는 EKS, Amazon RDS 및 Secrets Manager 포함)는 [AWS에 배포](/docs/ko/claude-apps-gateway-on-aws)를 참조하세요.
* Google Cloud의 완전한 작동 예제(Cloud Run, Cloud SQL 및 Secret Manager 포함)는 [Google Cloud에 배포](/docs/ko/claude-apps-gateway-on-gcp)를 참조하세요.
