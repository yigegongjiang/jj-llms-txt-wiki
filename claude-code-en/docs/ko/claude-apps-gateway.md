> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Amazon Bedrock, AWS의 Claude Platform, Google Cloud 및 Microsoft Foundry용 Claude 앱 게이트웨이

> SSO 로그인, 그룹별 모델 액세스, OTLP 텔레메트리를 갖춘 자체 호스팅 게이트웨이를 통해 Amazon Bedrock, AWS의 Claude Platform, Google Cloud 또는 Microsoft Foundry에서 Claude Code를 실행합니다.

<Note>
  Claude 앱 게이트웨이는 자신의 클라우드 제공자를 통해 추론을 라우팅해야 하거나 선호하는 조직을 위해 설계되었습니다. 예를 들어 [데이터 거주지](/docs/ko/claude-apps-gateway-deploy#compliance-posture) 요구사항을 충족하기 위해서입니다. 이러한 요구사항이 없으며 SCIM 프로비저닝 또는 웹 및 모바일의 Claude Code와 같은 다른 기능에 액세스하려면 Claude Enterprise가 더 적합할 수 있습니다. 모든 배포 방법의 전체 비교는 [기능 가용성](/docs/ko/feature-availability) 페이지를 참조하십시오.
</Note>

Claude 앱 게이트웨이는 개발자의 Claude Code 클라이언트와 모델 제공자 사이에 위치하는 자체 호스팅 서비스입니다. 개발자는 API 키나 클라우드 자격증명을 보유하는 대신 회사 ID 제공자(IdP)로 로그인합니다. 게이트웨이는 업스트림 자격증명을 보유하고, IdP 그룹별로 모델 액세스 및 [관리 설정](/docs/ko/permissions#managed-settings)을 적용하며, 사용 현황 텔레메트리를 자신의 관찰성 스택으로 전달합니다.

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
* **액세스 제어**: IdP 그룹은 모델 허용 목록 및 [관리 설정](/docs/ko/permissions#managed-settings) 정책에 매핑됩니다. 게이트웨이는 모델 액세스를 서버 측에서 적용하여 부여되지 않은 모델에 대한 요청을 거부하고, 각 그룹의 관리 설정 정책을 선택하며, CLI는 [관리 설정 계층](/docs/ko/settings#settings-precedence)에서 이를 적용합니다. 다른 팀은 다른 모델, 도구, 권한을 얻고, 개발자는 정책이 잠근 것을 재정의할 수 없습니다.
* **설정 전달**: 게이트웨이는 관리 설정을 로그인한 클라이언트에 자체적으로 전달하여 claude.ai 관리 콘솔의 [서버 관리 설정](/docs/ko/server-managed-settings)을 대체합니다.
* **텔레메트리**: Datadog, Splunk 또는 ClickHouse와 같은 각 구성된 대상은 기본적으로 토큰 수, 모델, 사용자 ID 및 지연 시간이 포함된 [OpenTelemetry Protocol (OTLP) 메트릭](/docs/ko/monitoring-usage)을 받으며, 로그 및 추적은 대상별 옵트인입니다.
* **업스트림 라우팅**: 클라이언트는 Anthropic Messages API를 게이트웨이에 말하고, 게이트웨이는 Amazon Bedrock, [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws), Google Cloud의 Agent Platform, Microsoft Foundry 또는 Anthropic API 중 각 업스트림에 대해 변환하며, 그들 사이에 장애 조치가 있습니다. 개발자가 알아차리거나 재구성하지 않고도 지역, 제공자 또는 장애 조치 순서를 변경할 수 있습니다.

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="Claude Code 클라이언트가 HTTPS와 베어러 토큰으로 인프라 내 자체 호스팅 Claude 앱 게이트웨이에 연결되고, IdP에 대해 사용자에게 로그인하고, PostgreSQL에 인증 상태를 저장하고, OTLP 수집기에 텔레메트리를 중계하고, Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry 또는 Anthropic API로 추론을 전달하는 것을 보여주는 다이어그램" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  게이트웨이의 자체 데이터 평면은 Anthropic API가 구성된 업스트림이 아닌 한 Anthropic 인프라에 아무것도 보내지 않습니다. 텔레메트리, 감사 로그, 관리 설정 및 개발자의 IdP 신원이 어디로 가는지 제어하고, 게이트웨이는 이 중 어느 것도 Anthropic에 보내지 않습니다. 나머지 트래픽의 경우 CLI 프로세스가 보낼 수 있는 것과 이를 닫는 방법은 [규정 준수 태세](/docs/ko/claude-apps-gateway-deploy#compliance-posture)를 참조하세요.
</Note>

Claude Code의 어떤 기능이 게이트웨이를 통해 작동하고 서버 자체가 무엇을 지원하는지는 아래 [가용성 및 제한사항](#availability-and-limitations)을 참조하세요. 비용, 우회, 여러 게이트웨이 실행, 서버리스 플랫폼과 같은 결정은 [배포 가이드](/docs/ko/claude-apps-gateway-deploy#deployment)를 참조하세요.

<h3 id="other-gateway-implementations">
  다른 게이트웨이 구현
</h3>

이미 필요를 충족하는 LLM 게이트웨이 또는 API 게이트웨이를 실행 중인 경우, 계속 사용하세요; [다른 LLM 게이트웨이](/docs/ko/llm-gateway)는 Claude Code를 이에 대해 구성하는 것을 다룹니다.

[게이트웨이 프로토콜 참조](/docs/ko/llm-gateway-protocol)는 Claude Code가 모든 게이트웨이에서 기대하는 계약을 문서화합니다: 호출하는 엔드포인트, 전달할 헤더 및 본문 필드, 그리고 제거될 때 작동하지 않는 것입니다. 실행 중인 Claude 앱 게이트웨이는 `GET /protocol`에서 해당 계약의 상위 집합을 제공하며, SSO 로그인, 관리 설정 전달 및 텔레메트리를 위한 Claude 앱 게이트웨이 특정 엔드포인트를 추가합니다. 배포된 게이트웨이(예: 아래 [빠른 시작](#quickstart)이 생성하는 것)에서 `curl https://claude-gateway.internal.example.com/protocol`로 가져옵니다. 프로토콜의 주요 변경사항은 미리 공지되지만, 무한 역호환성은 보장되지 않습니다.

<h2 id="quickstart">
  빠른 시작
</h2>

이 빠른 시작은 최소 경로를 안내합니다: IdP에서 OAuth 클라이언트를 등록하고, `gateway.yaml`을 작성하고, Docker Compose로 게이트웨이를 Postgres와 함께 실행하고, 엔드 투 엔드 로그인을 확인합니다. Amazon Bedrock 업스트림을 사용합니다; Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry 및 Anthropic API는 [구성 참조](/docs/ko/claude-apps-gateway-config#upstreams)에 표시된 대로 `upstreams` 블록을 교체하여 동등하게 지원됩니다. 끝에서 개발자가 `/login`할 수 있는 게이트웨이가 있습니다.

<Note>
  **개인 네트워크에 배포하세요.** Claude Code는 주소가 개인인 게이트웨이에만 연결합니다. 이는 신뢰할 수 있는 게이트웨이가 개발자 머신에서 명령을 실행하는 설정을 푸시할 수 있기 때문에 보안 가드입니다. 게이트웨이를 내부 로드 밸런서 또는 VPN 뒤에 놓고 개인 IP로만 확인되는 호스트명을 제공하세요.

  Anthropic이 운영하는 공개 게이트웨이 엔드포인트는 예외입니다: `/login`은 `https://`를 통해 이들을 허용합니다. 이들은 Anthropic 자체가 운영하는 작은 고정 게이트웨이 세트입니다; 이들은 선택하거나 구성할 수 있는 배포 옵션이 아닙니다. 목록은 Claude Code에 컴파일되므로 구성이 호스트명을 추가할 수 없고 호스팅하는 게이트웨이는 면제 대상이 될 수 없습니다. v2.1.206 이전에는 `/login`이 다른 공개 주소처럼 이러한 엔드포인트를 거부했습니다.
</Note>

<h3 id="prerequisites">
  필수 조건
</h3>

시작하기 전에 다음을 준비하세요:

| 필요한 것                        | 세부사항                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 이상      | `claude gateway` 서브명령 및 게이트웨이 로그인 흐름은 v2.1.195에서 제공됩니다. 이전 공개 빌드는 이를 포함하지 않습니다. 게이트웨이 서버를 실행하는 머신과 각 개발자의 머신 모두 v2.1.195 이상이어야 합니다; `claude update`를 실행하여 최신 릴리스를 받으세요. [Claude Platform on AWS 업스트림](/docs/ko/claude-apps-gateway-config#claude-platform-on-aws)은 게이트웨이 서버에서 Claude Code v2.1.198 이상이 필요합니다.                                                                                                                                                                                                                                                |
| OpenID Connect (OIDC) ID 제공자 | Okta, Microsoft Entra ID, Google Workspace, Keycloak, Dex 또는 PingFederate와 같은 다른 OIDC 호환 IdP. 게이트웨이는 표준 OIDC 검색 및 인증 코드 흐름을 이에 대해 실행합니다. SAML 및 LDAP는 지원되지 않습니다.                                                                                                                                                                                                                                                                                                                                                                                        |
| PostgreSQL 14 이상             | 브라우저 콜백이 쓰고 폴링 CLI가 읽는 장치 로그인 흐름, 그리고 속도 제한 카운터를 지원합니다. 가장 작은 계층을 포함한 모든 관리 Postgres가 작동합니다. 지출 제한이 구성되지 않으면 게이트웨이는 몇 KB의 단기 인증 상태를 저장합니다; [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)을 사용하면 백업해야 하는 지속적인 지출, 감사 및 ID 테이블도 보유합니다. `?sslmode=require`를 통한 TLS가 권장됩니다.                                                                                                                                                                                                                                                                                  |
| 모델 업스트림                      | Amazon Bedrock 자격증명, Claude Platform on AWS 자격증명, Google Cloud 자격증명, Microsoft Foundry 리소스 또는 Anthropic API 키. 장애 조치를 사용한 여러 업스트림이 지원됩니다.                                                                                                                                                                                                                                                                                                                                                                                                               |
| HTTPS                        | 게이트웨이는 개발자 노트북과 로그인에 사용되는 모든 브라우저에서 `https://`를 통해 도달 가능해야 합니다; 게이트웨이는 동일한 리스너에서 장치 확인 페이지를 제공합니다. `listen.tls`를 통해 TLS 인증서를 제공하거나, TLS 종료 수신 대기 뒤에서 실행하고 `listen.public_url`을 설정하세요. 일반 `http://` 원본은 로컬 개발을 위해 루프백에서만 허용됩니다.                                                                                                                                                                                                                                                                                                                          |
| 개인 네트워크 주소                   | `/login`에서 Claude Code는 게이트웨이의 호스트명 또는 IP 주소가 개인 주소로만 확인되도록 요구합니다: RFC 1918, CGNAT `100.64.0.0/10`, IPv6 ULA `fc00::/7` 또는 로컬 개발을 위한 루프백. 확인은 각 확인된 IP에서 실행되므로, 이름이 확인되는 주소가 공개인 경우 `/login`은 URL을 거부합니다. 개발자 머신이 HTTPS를 회사 프록시를 통해 라우팅하는 경우, 로그인은 프록시 호스트도 개인 주소로 확인되도록 요구합니다; 그렇지 않으면 게이트웨이 호스트를 `NO_PROXY`에 추가하여 CLI가 직접 연결하도록 하세요. Anthropic이 운영하는 공개 게이트웨이 엔드포인트는 개인 주소 및 프록시 확인에서 면제됩니다: `/login`은 정확한 호스트명 일치로 `https://`를 통해 이들을 허용하므로, 개인 네트워크 요구사항은 직접 호스팅하는 게이트웨이에만 적용됩니다. v2.1.206 이전에는 `/login`이 Anthropic이 운영하는 엔드포인트를 다른 공개 주소처럼 거부했습니다. |
| Linux 런타임                    | 게이트웨이 서버는 네이티브 Linux 바이너리에서만 실행됩니다. macOS는 로컬 개발에 작동합니다. Windows는 서버 플랫폼으로 지원되지 않습니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

게이트웨이 서버는 네이티브 `claude` 바이너리가 필요합니다; [Claude Code 설치](/docs/ko/setup)에 설명된 대로 고정된 릴리스를 다운로드하세요. 서버는 Claude Code가 Node 아래에서 실행될 때 사용 불가능한 런타임 기능을 사용합니다. 부팅 시 `requires the native binary`가 표시되면 독립형 설치 방법 중 하나로 전환하세요.

<h3 id="steps">
  단계
</h3>

<Steps>
  <Step title="IdP에서 OAuth 클라이언트 등록">
    게이트웨이의 호스트명을 먼저 결정하세요. 리다이렉트 URI가 일치해야 하기 때문입니다. 새 OIDC 웹 애플리케이션을 만들고 리다이렉트 URI를 `https://claude-gateway.<your-domain>/oauth/callback`으로 설정하세요. 여기서 호스트는 3단계에서 [`listen.public_url`](/docs/ko/claude-apps-gateway-config#listen)로 설정하는 동일한 값입니다. `client_id` 및 `client_secret`을 기록하세요. IdP별 지침은 [ID 제공자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)에 있습니다.
  </Step>

  <Step title="PostgreSQL 데이터베이스 프로비저닝">
    가장 작은 관리 계층을 포함한 모든 Postgres 14 이상이 작동합니다. 게이트웨이는 부팅 시 자체 스키마 마이그레이션을 실행하므로 데이터베이스 사용자는 `CREATE TABLE` 권한이 필요합니다. 보안 정책이 애플리케이션 역할의 DDL을 금지하는 경우, 대신 스키마를 미리 만드세요; [`store`](/docs/ko/claude-apps-gateway-config#store)를 참조하세요.
  </Step>

  <Step title="gateway.yaml 작성">
    비밀은 `${ENV_VAR}` 확장을 통해 읽혀지므로 파일 자체는 버전 제어에 있을 수 있습니다. `/login`이 공개 주소를 거부하기 때문에 개인 IP로 확인되는 `public_url` 호스트명을 사용하세요. 최소 구성에는 5개 섹션이 있고, 다른 모든 필드는 기본값을 가집니다:

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # TLS 종료 프록시 뒤에서 필수. IdP
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
      Bedrock 업스트림은 `bedrock:InvokeModel` 및 `bedrock:InvokeModelWithResponseStream`을 `inference-profile/us.anthropic.*` ARN과 기본 `foundation-model/anthropic.*` ARN 모두에 가진 AWS 주체가 필요하며, Bedrock 콘솔에서 원하는 Claude 모델에 대해 모델 액세스가 활성화되어야 합니다. EKS의 IRSA, ECS 작업 역할 또는 EC2 인스턴스 프로필보다는 정적 키를 사용하여 자격증명을 제공하세요. [`upstreams` 참조](/docs/ko/claude-apps-gateway-config#upstreams)는 전체 IAM 세부사항, 클라우드 간 자격증명 매트릭스, 다른 제공자의 `auth` 블록을 가집니다.
    </Note>
  </Step>

  <Step title="실행">
    [이미지 요구사항](/docs/ko/claude-apps-gateway-deploy#container-image)을 충족하는 `claude` 바이너리 주위에 컨테이너 이미지를 빌드한 다음 Postgres와 함께 실행하세요:

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: <your-registry>/claude-gateway:<version>
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

    게이트웨이는 구성을 읽고, IdP에 대해 OIDC 검색을 실행하고, Postgres 스키마 마이그레이션을 적용하고, 업스트림 클라이언트를 빌드하고, 수신 대기를 시작하는 단일 Linux 바이너리입니다. 부팅은 구성, Postgres 연결(5초 타임아웃), OIDC 검색 및 업스트림 클라이언트 구성에 대해 실패 폐쇄됩니다. 이 중 하나가 도달 불가능하거나 잘못 구성된 경우, 게이트웨이는 저하된 상태에서 트래픽을 제공하는 대신 오류로 종료됩니다.

    성공적인 부팅은 Bedrock 및 Agent Platform 인스턴스 자격증명이 부팅 시가 아닌 첫 요청에서 확인되기 때문에 추론 경로를 검증하지 않습니다.

    부팅 시퀀스에 대해 stderr를 감시하세요. 로그 라인은 `[gateway] <timestamp> <level> <message>` 형식을 사용하고, 감사 이벤트는 `evt` 필드가 있는 단일 라인 JSON이며, 시작 배너(아래 생략됨)는 마이그레이션과 수신 대기 라인 사이에 인쇄됩니다. 순서대로 다음을 볼 수 있습니다:

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

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
    이 마지막 단계는 서버가 아닌 개발자 머신에서 발생합니다. 해당 머신의 [관리 설정 파일](/docs/ko/settings#settings-files)에서 `forceLoginMethod`를 `"gateway"`로 설정하고 `forceLoginGatewayUrl`을 게이트웨이의 `public_url`로 설정한 다음 `/login`을 실행하고, **Cloud gateway** 화면에서 Enter를 누르고, 브라우저 로그인을 완료하세요. 아래 [게이트웨이 URL 설정](#set-the-gateway-url)은 규모에 따라 두 키를 배포하는 것을 다룹니다.
  </Step>
</Steps>

<h2 id="connect-developers">
  개발자 연결
</h2>

개발자는 자신의 노트북에서 한 번의 브라우저 로그인으로 회사 업무 계정을 사용하여 연결합니다. 요청이 조직의 업스트림 자격증명을 사용하여 게이트웨이를 통해 모델로 가기 때문에 claude.ai 계정, API 키 또는 구독이 필요하지 않습니다. 연결은 MDM을 통해 푸시하는 [클라이언트 측 관리 설정](/docs/ko/claude-apps-gateway-config#client-side-managed-settings)에 의해 구동되므로, 개발자 측에는 수동 설정이 없습니다; 이 섹션은 관리자가 구성하는 것을 다룹니다.

CLI는 첫 연결 시 게이트웨이의 TLS 리프 인증서를 지문 처리하고 호스트명별로 고정합니다. 예상 SHA-256 지문을 게이트웨이 URL과 함께 게시하여 개발자가 비교할 것이 있도록 하세요. `openssl x509 -noout -fingerprint -sha256 -in cert.pem`으로 인증서 파일에서 지문을 가져오세요; `/login` 프롬프트는 다이제스트의 처음 16자를 구분 기호 없이 소문자 16진수로 표시합니다.

인증서가 회전하면 모든 개발자가 신뢰 프롬프트를 다시 보므로, 회전을 계획된 이벤트로 취급하고 지문을 다시 게시하세요.

로그인하면, [모델 선택기](/docs/ko/model-config)는 개발자의 `availableModels` 허용 목록의 모델을 표시하고, 관리 설정은 시작 시 적용되고 매시간 새로 고쳐지며, 텔레메트리는 수집기로 라우팅됩니다. 세션은 `ttl_hours` 만료 전에 자동으로 새로 고쳐지고, IdP 프로비저닝 해제 후 새로 고침 실패는 다시 로그인을 프롬프트합니다.

<h3 id="set-the-gateway-url">
  게이트웨이 URL 설정
</h3>

MDM을 통해 또는 디스크에 직접 배포하는 OS별 [관리 설정 파일](/docs/ko/settings#settings-files)에서 두 키를 모두 설정하면, `/login`은 URL이 채워진 **Cloud gateway** 화면에서 직접 열립니다:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

개발자는 Enter를 눌러 연결합니다. 첫 연결 TLS 지문 프롬프트는 여전히 나타납니다.

개발자가 수동으로 선택할 수 있는 로그인 선택기의 게이트웨이 옵션이 없으며, `forceLoginGatewayUrl`은 개발자의 자신의 설정 파일에서 무시됩니다. `forceLoginMethod`만, URL 없이, 개발자를 "IT 관리자에게 문의하세요" 메시지에 남깁니다. 두 키 모두 게이트웨이의 `managed.policies[].cli` 블록이 아닌 머신으로 푸시하는 파일에 속하며, 이는 이미 연결된 클라이언트에만 도달합니다.

<h3 id="ci-pipelines-and-remote-machines">
  CI 파이프라인 및 원격 머신
</h3>

무인 파이프라인을 위한 서비스 토큰 흐름이 없습니다. 게이트웨이 로그인은 항상 브라우저 장치 흐름을 실행하므로, 로그인을 승인할 개발자가 없는 CI 작업은 인증할 수 없습니다; 제공자에 대해 직접 구성하세요.

개발자가 로그인하면, 해당 머신의 모든 Claude Code 호출은 게이트웨이 세션을 사용하며, 비대화형 `claude -p` 실행 및 Agent SDK에서 시작된 세션을 포함하고, [게이트웨이 정책은 모두에 적용됩니다](/docs/ko/claude-apps-gateway-config#managed).

장치 흐름은 폴링 CLI를 승인하는 브라우저에서 분리하므로, 디스플레이가 없는 원격 개발 상자도 작동합니다: 개발자는 원격 머신에서 SSH를 통해 `/login`을 실행하고 노트북의 브라우저에서 확인 링크를 엽니다.

<h3 id="what’s-enforced-on-developers">
  개발자에게 적용되는 것
</h3>

이러한 보장은 모든 로그인한 게이트웨이 세션에 적용됩니다.

* **모델 액세스**: 정책이 부여하지 않는 모델에 대한 요청은 400을 반환하고, `/model` 선택기는 정책의 `availableModels` 허용 목록으로 필터링됩니다. 정책에서 [`enforceAvailableModels: true`](/docs/ko/model-config#default-model-behavior)를 설정하여 Default 옵션이 `availableModels` 내의 모델로 확인되도록 하세요. 없으면 Default는 선택 가능하게 유지되고 해당 모델이 부여되지 않으면 요청 시 거부됩니다.
* **텔레메트리 대상**: [텔레메트리 전달](/docs/ko/claude-apps-gateway-config#telemetry)이 구성되면, OTLP 내보내기 엔드포인트는 게이트웨이에 고정되고, 게이트웨이 푸시 구성은 로컬로 설정된 `OTEL_*` 변수를 재정의합니다.
* **자격증명**: 게이트웨이 토큰은 세션의 유일한 자격증명입니다. `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, `apiKeyHelper` 및 이전 claude.ai 로그인은 로그인 중에 무시되므로, 개발자는 먼저 claude.ai에서 로그아웃할 필요가 없습니다.
* **관리 설정**: 잠긴 키는 로컬로 재정의될 수 없습니다. CLI는 시작 시 정책을 적용하고 각 시간별 폴에서 적용합니다.
* **시작**: 로그인한 세션은 게이트웨이가 도달 불가능할 때 약 10초 후 시작 시 오류로 종료되며, 설정 없이 시작하는 대신입니다.
* **프로비저닝 해제**: 사용자가 IdP에서 비활성화된 세션은 다음 새로 고침이 실패할 때 `ttl_hours` 내에 만료됩니다.

<h3 id="what-the-organization-can-see">
  조직이 볼 수 있는 것
</h3>

사용 현황 텔레메트리는 개발자의 신원, 토큰 수, 모델 및 지연 시간을 조직의 수집기로 전달합니다. 게이트웨이는 프롬프트 또는 완료 콘텐츠를 기록하거나 저장하지 않습니다. 명령 및 파일 경로를 포함할 수 있는 로그 및 추적과 같은 더 풍부한 텔레메트리를 수집할지 여부는 조직의 [대상별 선택](/docs/ko/claude-apps-gateway-config#telemetry)입니다.

<h2 id="availability-and-limitations">
  가용성 및 제한사항
</h2>

표는 개발자가 게이트웨이를 통해 연결할 때 작동하는 Claude Code 기능과 게이트웨이 서버 자체가 지원하는 것을 다룹니다. 지원되지 않는 경우, Notes 열은 대안을 제공합니다.

게이트웨이는 CLI가 모든 업스트림으로 보내는 [`anthropic-beta`](https://platform.claude.com/docs/ko/api/beta-headers) 값을 전달하므로, 운영자는 베타 허용 목록을 유지하지 않습니다. Amazon Bedrock의 경우 헤더를 무시하므로, 게이트웨이는 값을 요청 본문의 `anthropic_beta` 필드로 이동합니다; 다른 업스트림은 보낸 대로 헤더를 받습니다. CLI의 게이트웨이 세션 베타 집합은 자사 전용 베타 및 확장 캐시 TTL 베타를 생략하므로, 아래 해당 행은 사용 불가능으로 표시됩니다.

| 기능                                                                                                         | 상태      | 참고                                                                                                                                                                                                                                                                                                                         |
| ---------------------------------------------------------------------------------------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 추론 전달 (Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry, Anthropic) | 사용 가능   | 업스트림별 모델 변환 및 장애 조치 포함. Amazon Bedrock 업스트림은 `bedrock-runtime` 엔드포인트 및 AWS 기본 자격증명 체인을 사용합니다; Amazon Bedrock [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint)는 지원되는 업스트림이 아닙니다. [Claude Platform on AWS 업스트림](/docs/ko/claude-apps-gateway-config#claude-platform-on-aws)은 게이트웨이 서버에서 Claude Code v2.1.198 이상이 필요합니다. |
| IdP 그룹별 모델 액세스 및 관리 설정                                                                                     | 사용 가능   | 모델 액세스는 서버 측에서 적용됩니다; 관리 설정은 IdP 그룹별로 전달되고 CLI에서 [관리 설정 계층](/docs/ko/settings#settings-precedence)에서 적용됩니다.                                                                                                                                                                                                                     |
| 텔레메트리 팬아웃 (OTLP/HTTP)                                                                                      | 사용 가능   | 내보내기별 ID 스탬프; protobuf 및 JSON 인코딩 모두                                                                                                                                                                                                                                                                                       |
| OIDC 신원 제공자                                                                                                | 사용 가능   | 모든 OIDC 호환 IdP; 게이트웨이는 표준 OIDC 검색 및 인증 코드 흐름을 실행합니다. [신원 제공자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)에서 IdP별 구성을 참조하세요.                                                                                                                                                                                 |
| 사용자별 및 그룹별 지출 제한                                                                                           | 사용 가능   | [지출 제한](/docs/ko/claude-apps-gateway-spend-limits) 참조                                                                                                                                                                                                                                                                           |
| 서버 측 웹 검색                                                                                                  | 사용 불가능  | CLI는 게이트웨이가 라우팅하는 업스트림 제공자를 볼 수 없으므로 웹 검색 지원을 확인할 수 없고 게이트웨이 세션에서 WebSearch를 비활성화합니다.                                                                                                                                                                                                                                      |
| 표준 프롬프트 캐싱                                                                                                 | 사용 가능   | `cache_control` 중단점은 모든 업스트림으로 전달됩니다.                                                                                                                                                                                                                                                                                      |
| 1시간 캐시 TTL                                                                                                 | 사용 불가능  | CLI는 게이트웨이 세션에서 확장 캐시 TTL 베타를 생략합니다. 게이트웨이가 라우팅할 수 있는 모든 업스트림이 1시간 TTL을 지원하지 않기 때문에, 게이트웨이를 통한 프롬프트 캐싱은 5분 TTL을 사용합니다; 베타 헤더 참고 참조                                                                                                                                                                                         |
| Auto 모드                                                                                                    | 사용 가능   | [타사 제공자 규칙](/docs/ko/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry) 따름: 타사 제공자에서 적격인 모델만 사용 가능합니다. v2.1.207 이전에는 게이트웨이 세션의 auto 모드에서 `CLAUDE_CODE_ENABLE_AUTO_MODE=1` 설정이 필요했으며, 관리 정책 `env` 블록을 통해 전달 가능했습니다.                                                                                         |
| 글로벌 캐시 범위 및 토큰 효율적인 도구와 같은 자사 전용 최적화                                                                       | 사용 불가능  | CLI는 게이트웨이 세션에서 이를 활성화하지 않습니다; 베타 헤더 참고 참조                                                                                                                                                                                                                                                                                 |
| OTLP/gRPC                                                                                                  | 지원되지 않음 | HTTP를 통한 OTLP만                                                                                                                                                                                                                                                                                                             |
| SAML, LDAP 및 기타 비 OIDC 인증                                                                                  | 지원되지 않음 | OIDC만. 필요한 경우 OIDC 브리지로 프론트                                                                                                                                                                                                                                                                                                |
| 다중 테넌트 (여러 OIDC 발급자)                                                                                       | 지원되지 않음 | 게이트웨이당 하나의 발급자. 별도 인스턴스 실행                                                                                                                                                                                                                                                                                                 |
| Windows 서버                                                                                                 | 지원되지 않음 | Linux에 배포. 로컬 개발만 macOS                                                                                                                                                                                                                                                                                                    |
| Helm 차트                                                                                                    | 사용 불가능  | 게이트웨이는 표준 상태 비저장 배포로 실행됩니다; [배포 가이드](/docs/ko/claude-apps-gateway-deploy#kubernetes) 참조                                                                                                                                                                                                                                         |
| 관리 UI                                                                                                      | 사용 불가능  | 구성은 YAML 파일입니다; 변경하려면 다시 배포하세요.                                                                                                                                                                                                                                                                                            |

<h2 id="next-steps">
  다음 단계
</h2>

빠른 시작은 Docker Compose에서 실행되는 최소 구성을 남깁니다. 더 나아가려면:

* 예를 들어 그룹별 RBAC, 다중 업스트림 장애 조치 또는 텔레메트리 대상을 추가하여 `gateway.yaml`을 최소 구성 이상으로 확장하세요. [구성 참조](/docs/ko/claude-apps-gateway-config)는 모든 옵션을 다룹니다.
* Compose에서 Kubernetes 또는 Cloud Run의 프로덕션 배포로 이동하고, IdP를 올바르게 설정하고, 보안 모델을 검토하세요. [배포 및 운영 가이드](/docs/ko/claude-apps-gateway-deploy)는 IdP별 설정, 컨테이너 이미지 요구사항, 상태 프로브 및 문제 해결을 다룹니다.
* 개별 개발자 또는 그룹에 지출 상한을 설정하여 폭주 워크로드가 전체 약정을 소비할 수 없도록 하세요. [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)은 관리 API 및 적용 방식을 다룹니다.
* Google Cloud의 완전한 작동 예제(Cloud Run, Cloud SQL 및 Secret Manager 포함)는 [Google Cloud에 배포](/docs/ko/claude-apps-gateway-on-gcp)를 참조하세요.
