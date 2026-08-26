> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Google Cloud에 Claude 앱 게이트웨이 배포

> Google Cloud에서 Claude 앱 게이트웨이를 실행하는 실제 예제: Cloud Run 또는 GKE, Cloud SQL for PostgreSQL, Secret Manager, 그리고 Agent Platform에 대한 서비스 계정 인증.

<Note>
  이 페이지는 Google Cloud에서 Claude 앱 게이트웨이를 실행하는 한 가지 방법을 안내합니다. 이 구성은 지원되는 프로덕션 배포가 아닌 고객 관리 인프라에 대한 작동 예제입니다. 이를 사용하여 자신의 환경에 맞게 조정하기 전에 각 부분이 어떻게 함께 작동하는지 확인하세요. 플랫폼 독립적인 요구 사항은 [배포 가이드](/docs/ko/claude-apps-gateway-deploy)를 참조하세요.
</Note>

이 예제는 Google Cloud의 Agent Platform을 모델 업스트림으로 사용하여 Google Cloud에 Claude 앱 게이트웨이를 프로비저닝합니다. 컴퓨팅을 위해 Cloud Run 또는 GKE를 사용합니다. Google Workspace는 예제 ID 공급자(IdP)이지만, OpenID Connect(OIDC) 호환 IdP는 모두 작동합니다. `oidc` 블록만 변경됩니다. IdP별 세부 사항은 [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하세요.

<h2 id="what-you’ll-build">
  구축할 내용
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Google Cloud의 Claude 앱 게이트웨이 다이어그램: Claude Code 클라이언트는 HTTPS를 통해 게이트웨이(Cloud Run 또는 GKE)에 연결되며, 이는 세션 상태를 위한 프라이빗 IP Cloud SQL 데이터베이스와 함께 VPC 내부에서 실행됩니다. 게이트웨이는 OIDC를 통해 Google Workspace에 대해 사용자에게 로그인하도록 하고, Secret Manager에서 구성 및 비밀을 읽고, 모델 요청을 Google Cloud의 Agent Platform으로 전달하며, 배포 시 Artifact Registry에서 이미지를 가져옵니다." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

참조 구성은 다음을 프로비저닝합니다:

* 게이트웨이 컨테이너를 실행하는 **Cloud Run** 서비스 또는 **GKE** Deployment
* 게이트웨이 이미지를 위한 **Artifact Registry** 저장소
* 게이트웨이의 [저장소](/docs/ko/claude-apps-gateway-config#store)를 위한 **Cloud SQL for PostgreSQL** 인스턴스(프라이빗 IP만)
* `gateway.yaml`, JWT 서명 키, OIDC 클라이언트 비밀, Postgres URL에 대한 **Secret Manager** 비밀
* `roles/aiplatform.user`를 가진 **서비스 계정**(Cloud Run에 직접 연결되거나 GKE에서 Workload Identity를 통해 바인딩됨)
* Cloud Run의 **내부 Application Load Balancer** 또는 GKE의 `gce-internal` 클래스 내부 **GKE Ingress**(HTTPS용)

<h2 id="prerequisites">
  전제 조건
</h2>

* 청구가 활성화된 GCP 프로젝트 및 위의 리소스를 생성할 권한
* `gcloud auth login`으로 인증된 `gcloud` CLI 및 로컬에 설치된 Docker
* GKE 트랙의 경우: `kubectl` 및 아래 연습에서 생성된 VPC의 GKE 클러스터
* Model Garden에서 필요한 Claude 모델에 대한 액세스 및 이를 게시하는 지역
* 리다이렉트 URI `https://<gateway-host>/oauth/callback`를 가진 Google Workspace OAuth 2.0 웹 애플리케이션 클라이언트. [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하세요.
* 게이트웨이를 위한 TLS 호스트명(일반적으로 로드 밸런서를 가리키는 내부 DNS 이름)

프로젝트와 지역을 한 번 설정합니다:

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  게이트웨이 배포
</h2>

아래 단계는 `gcloud` 명령으로 전체 배포를 프로비저닝합니다.

<Steps>
  <Step title="API 활성화">
    연습에서 사용하는 서비스 API를 활성화합니다:

    ```bash theme={null}
    gcloud services enable \
      aiplatform.googleapis.com \
      artifactregistry.googleapis.com \
      sqladmin.googleapis.com \
      secretmanager.googleapis.com \
      iamcredentials.googleapis.com \
      iam.googleapis.com \
      compute.googleapis.com \
      servicenetworking.googleapis.com \
      run.googleapis.com \
      container.googleapis.com
    ```

    필요한 API는 배포 경로에 따라 다릅니다:

    * `compute` 및 `servicenetworking`: 프라이빗 IP Cloud SQL 경로에 필요
    * `run`: Cloud Run만
    * `container`: GKE만
  </Step>

  <Step title="서비스 계정 생성 및 IAM 권한 부여">
    게이트웨이는 Google Cloud의 Agent Platform을 호출할 권한이 있는 전용 서비스 계정으로 실행됩니다. VPC를 통해 Cloud SQL에 도달하는 암호 사용자이므로 Cloud SQL IAM 역할이 필요하지 않습니다:

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    그런 다음 Model Garden에서 프로젝트에 대해 Claude 모델을 활성화합니다. 모델은 특정 지역에 게시되므로 각 모델 카드를 확인하세요.
  </Step>

  <Step title="이미지를 Artifact Registry에 빌드 및 푸시">
    [컨테이너 이미지 요구 사항](/docs/ko/claude-apps-gateway-deploy#container-image)에 따라 `linux-x64` glibc 바이너리를 사용하여 이미지를 빌드하고 푸시합니다:

    ```bash theme={null}
    gcloud artifacts repositories create claude-gateway \
      --repository-format=docker --location="$REGION"
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

    # Cloud Run requires linux/amd64. --provenance=false avoids a buildx OCI
    # image index that Cloud Run rejects.
    docker build --platform=linux/amd64 --provenance=false \
      -t "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" .
    docker push "${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>"
    ```
  </Step>

  <Step title="Cloud SQL for PostgreSQL 프로비저닝">
    Private Services Access를 통해 VPC에 인스턴스를 생성하여 공개 IP가 없도록 합니다. 이는 `constraints/sql.restrictPublicIp`가 적용되는 프로젝트도 만족합니다:

    ```bash theme={null}
    VPC=cc-gateway-vpc
    gcloud compute networks create "$VPC" --subnet-mode=custom
    gcloud compute networks subnets create cc-gateway-subnet \
      --network="$VPC" --region="$REGION" --range=10.0.0.0/24

    # Private Services Access: one-time per VPC
    gcloud compute addresses create "google-managed-services-${VPC}" \
      --global --purpose=VPC_PEERING --prefix-length=16 --network="$VPC"
    gcloud services vpc-peerings connect \
      --service=servicenetworking.googleapis.com \
      --ranges="google-managed-services-${VPC}" --network="$VPC"

    gcloud sql instances create claude-gateway-db \
      --database-version=POSTGRES_16 --tier=db-g1-small --region="$REGION" \
      --network="projects/${PROJECT_ID}/global/networks/${VPC}" --no-assign-ip
    gcloud sql databases create claude_gateway --instance=claude-gateway-db
    PGPASS="$(openssl rand -hex 24)"
    gcloud sql users create gateway --instance=claude-gateway-db --password="$PGPASS"

    PRIVATE_IP="$(gcloud sql instances describe claude-gateway-db \
      --format='value(ipAddresses[0].ipAddress)')"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${PRIVATE_IP}:5432/claude_gateway?sslmode=require"
    ```

    Cloud Run 또는 GKE 런타임은 이 VPC에 있거나 이 VPC로 라우팅되어야 합니다.
  </Step>

  <Step title="gateway.yaml 작성">
    `upstreams` 블록은 `auth: {}`를 사용하여 Google Cloud의 Agent Platform을 가리키므로 게이트웨이는 런타임 서비스 계정의 Application Default Credentials를 통해 인증합니다. 모든 필드는 [구성 참조](/docs/ko/claude-apps-gateway-config)를 참조하세요.

    두 개의 `listen` 필드는 게이트웨이 앞에 무엇이 있는지에 따라 다릅니다:

    * `public_url`: Cloud Run 또는 GKE Ingress 뒤에 필요합니다. 게이트웨이는 IdP `redirect_uri`와 검색 문서를 이 값에서만 빌드하며 `X-Forwarded-*` 헤더에서는 빌드하지 않습니다.
    * `trusted_proxies`: 프론트 엔드의 소스 범위입니다. 게이트웨이는 TCP 피어가 이 목록에 있을 때만 `X-Forwarded-For`를 인정하고, 신뢰할 수 있는 홉을 지나 체인을 따라가므로 IP별 로그인 속도 제한 및 감사 이벤트는 로드 밸런서의 IP 대신 개발자 IP를 기록합니다.

    `trusted_proxies`를 프론트 엔드와 일치하도록 설정합니다. `gce` 클래스의 외부 GKE Ingress는 나열되지 않습니다. 공개 전달 규칙 주소를 프로비저닝하며, 이는 `/login` [프라이빗 네트워크 확인](/docs/ko/claude-apps-gateway#prerequisites)을 거부합니다.

    | 프론트 엔드                                    | `trusted_proxies`                     |
    | ----------------------------------------- | ------------------------------------- |
    | 로드 밸런서 없이 직접 도달하는 Cloud Run               | `[169.254.0.0/16]`                    |
    | Cloud Run 앞의 내부 Application Load Balancer | `169.254.0.0/16` 더하기 프록시 전용 서브넷의 CIDR |
    | GKE 내부 Ingress, 클래스 `gce-internal`        | 프록시 전용 서브넷의 CIDR                      |

    아래 예제는 내부 로드 밸런서 앞의 Cloud Run 값을 사용합니다.

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [169.254.0.0/16, <your-proxy-only-subnet-cidr>]

    oidc:
      issuer: https://accounts.google.com
      client_id: <your-oauth-client-id>
      client_secret: ${OIDC_CLIENT_SECRET}           # GKE: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # Google ignores offline_access; these yield refresh tokens:
      scopes: [openid, profile, email]
      extra_auth_params: { access_type: offline, prompt: consent }

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # GKE: ${file:/secrets/jwt-secret}

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # GKE: ${file:/secrets/postgres-url}

    upstreams:
      - provider: vertex
        region: <your-region>                        # must match $REGION
        project_id: <your-project>
        auth: {} # ADC via the runtime service account
    ```

    <Note>
      Google id\_tokens는 `groups` 클레임을 포함하지 않습니다. Google Workspace를 IdP로 사용하여 [`managed.policies`](/docs/ko/claude-apps-gateway-config#managed)에서 그룹 기반 정책을 사용하려면 [`oidc.google_groups`](/docs/ko/claude-apps-gateway-config#oidc)를 구성하세요. 이는 도메인 전체 위임이 있는 서비스 계정을 사용하여 Admin SDK Directory API를 통해 각 사용자의 그룹을 조회합니다. 없으면 `email_domain`으로 일치시킵니다.
    </Note>
  </Step>

  <Step title="Secret Manager에 비밀 저장">
    네 개의 비밀을 생성하고 `claude-gateway` 서비스 계정에 `roles/secretmanager.secretAccessor`를 부여합니다:

    | 비밀                           | 소스                                    |
    | ---------------------------- | ------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`             |
    | `gateway-oidc-client-secret` | Google Cloud Console → OAuth 클라이언트    |
    | `gateway-postgres-url`       | Cloud SQL 단계의 `$GATEWAY_POSTGRES_URL` |
    | `gateway-config`             | 이전 단계의 전체 `gateway.yaml`              |

    비밀이 컨테이너에 도달하는 방식은 트랙에 따라 다릅니다:

    * GKE에서는 Secret Manager CSI 드라이버를 통해 `/secrets`로 마운트되며, `gateway.yaml`은 `${file:/secrets/...}`를 참조합니다.
    * Cloud Run에서는 여러 비밀을 한 디렉토리에 마운트할 수 없으므로 `gateway.yaml`은 파일로 마운트되고 다른 세 개는 환경 변수로 주입되므로 `gateway.yaml`은 `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}`, `${GATEWAY_POSTGRES_URL}`을 대신 참조합니다.
  </Step>

  <Step title="배포">
    <Tabs>
      <Tab title="Cloud Run">
        아래 명령은 내부 로드 밸런서 뒤의 프로덕션을 위해 배포합니다.

        ```bash theme={null}
        gcloud run deploy claude-gateway \
          --image="${REGION}-docker.pkg.dev/${PROJECT_ID}/claude-gateway/gateway:<version>" \
          --region="$REGION" \
          --service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --min-instances=1 \
          --timeout=3600 \
          --ingress=internal-and-cloud-load-balancing \
          --network="$VPC" --subnet=cc-gateway-subnet --vpc-egress=private-ranges-only \
          --set-secrets=/etc/claude/gateway.yaml=gateway-config:latest,GATEWAY_JWT_SECRET=gateway-jwt-secret:latest,OIDC_CLIENT_SECRET=gateway-oidc-client-secret:latest,GATEWAY_POSTGRES_URL=gateway-postgres-url:latest \
          --no-invoker-iam-check
        ```

        Direct VPC egress(`--network`, `--subnet`, `--vpc-egress=private-ranges-only`)를 통해 서비스는 Cloud SQL 프라이빗 IP에 직접 도달할 수 있습니다. Google Cloud의 Agent Platform 엔드포인트 및 `accounts.google.com`에 대한 공개 egress는 VPC를 통하지 않고 인터넷으로 직접 이동하므로 Cloud NAT이 필요하지 않습니다.

        invoker IAM 확인은 열려 있거나 비활성화되어야 합니다. 게이트웨이는 자체 OIDC를 실행하고 클라이언트는 GCP 토큰을 가지지 않으므로 Cloud Run의 invoker 확인은 인증되지 않은 요청을 허용해야 합니다. 게이트웨이의 OIDC 로그인은 컨테이너에 도달하면 요청을 인증하며, `allowed_email_domains`는 어떤 도메인이 로그인할 수 있는지를 제어합니다.

        두 플래그는 인증되지 않은 요청을 허용합니다:

        * `--no-invoker-iam-check`: 관리할 `allUsers` 바인딩 없이 확인을 비활성화하며 Domain Restricted Sharing에서 작동합니다.
        * `--allow-unauthenticated`: `allUsers`에 `run.invoker` 역할을 부여합니다. 조직이 `--no-invoker-iam-check`를 허용하지 않으면 사용하세요.

        `--ingress`를 통한 Ingress 제한은 invoker 확인과 별개의 독립적인 계층입니다. 서비스를 회사 네트워크로 제한하도록 설정된 상태로 유지합니다.

        기본적으로 Cloud Run `*.run.app` URL은 공개 주소로 확인되며, 이는 `/login` [프라이빗 네트워크 확인](/docs/ko/claude-apps-gateway#prerequisites)을 거부합니다. 두 가지 토폴로지는 개발자에게 프라이빗으로 확인 가능한 호스트명을 제공하며, Cloud Run은 둘 다 프로비저닝하지 않습니다:

        * **내부 Application Load Balancer**(위의 배포 명령이 가정하는 토폴로지): `--ingress=internal-and-cloud-load-balancing`으로 배포하고, 서비스 앞에 내부 Application Load Balancer를 프로비저닝하며 내부 DNS 이름과 인증서를 사용하고, `listen.public_url`을 해당 호스트명으로 설정합니다.
        * **로드 밸런서 없는 내부 전용 ingress**: `--ingress=internal`로 배포하고 `listen.public_url`을 `*.run.app` URL로 유지합니다. 아래 [참조 자산](#terraform-reference)의 기본값입니다. `*.run.app`이 프라이빗으로 확인되려면 네트워크 팀이 이미 Google API에 대한 Private Service Connect 엔드포인트를 운영하고, `*.run.app`을 이에 확인하는 Cloud DNS 프라이빗 영역을 운영하고, 해당 엔드포인트로의 온프레미스 라우팅을 운영해야 합니다.

        Google의 [Cloud Run용 프라이빗 네트워킹 가이드](https://cloud.google.com/run/docs/securing/private-networking)는 두 옵션이 필요로 하는 인프라를 다룹니다. 게이트웨이가 프라이빗 호스트명에서 제공되면 로그인을 확인합니다. 그때까지 Cloud Run의 로그에서 컨테이너가 부팅되었는지 확인합니다.

        첫 번째 로그인 전에 OAuth 클라이언트의 승인된 리다이렉트 URI를 `<public_url>/oauth/callback`으로 업데이트합니다. `public_url`을 변경한 후 다시 배포합니다. 게이트웨이는 해당 설정에서만 공개 원본을 빌드하고 `X-Forwarded-Host` 및 `X-Forwarded-Proto`를 무시합니다. `X-Forwarded-For`는 `listen.trusted_proxies`가 설정되었을 때만 클라이언트 IP에 대해 인정됩니다.
      </Tab>

      <Tab title="GKE">
        클러스터는 Cloud SQL 단계에서 생성된 `$VPC`에 있어야 하므로 포드가 데이터베이스의 프라이빗 IP에 도달할 수 있습니다. VPC 피어링만으로는 작동하지 않습니다. Cloud SQL 프라이빗 IP 자체가 피어링된 네트워크이고 피어링은 비전이적이기 때문입니다. 해당 VPC에 새 클러스터를 생성하려면 `gcloud container clusters create`에 `--network="$VPC" --subnetwork=cc-gateway-subnet`을 전달합니다.

        클러스터 및 노드 풀에서 Workload Identity를 활성화한 다음 Google 서비스 계정을 Kubernetes 서비스 계정에 바인딩하여 포드가 해당 자격 증명을 상속하도록 합니다:

        ```bash theme={null}
        gcloud container clusters update <cluster> --region="$REGION" \
          --workload-pool="${PROJECT_ID}.svc.id.goog"
        # On a Standard cluster, existing node pools also need GKE_METADATA;
        # Autopilot enables this by default.
        gcloud container node-pools update <pool> --cluster=<cluster> \
          --region="$REGION" --workload-metadata=GKE_METADATA

        kubectl create namespace claude-gateway
        kubectl create serviceaccount gateway -n claude-gateway

        gcloud iam service-accounts add-iam-policy-binding \
          "claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com" \
          --role roles/iam.workloadIdentityUser \
          --member "serviceAccount:${PROJECT_ID}.svc.id.goog[claude-gateway/gateway]"

        kubectl annotate serviceaccount gateway -n claude-gateway \
          iam.gke.io/gcp-service-account="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"
        ```

        [Kubernetes 배포](/docs/ko/claude-apps-gateway-deploy#kubernetes)에 설명된 대로 게이트웨이를 표준 Deployment, Service, 내부 Ingress(클래스 `gce-internal`)로 배포합니다:

        * `serviceAccountName: gateway`
        * Secret Manager CSI 드라이버가 `/secrets`에 비밀을 마운트
        * readiness 프로브가 `GET /readyz`를 가리킴

        게이트웨이 Service에 BackendConfig를 연결하고 `timeoutSec`을 높입니다. GKE Ingress 뒤의 로드 밸런서 백엔드 서비스는 기본적으로 30초 타임아웃이며, 이는 긴 스트리밍 응답을 차단합니다.

        Workload Identity 클러스터에서 `169.254.169.254`를 차단하는 egress NetworkPolicy를 적용하지 마세요. 포드는 자격 증명을 위해 메타데이터 서버에 도달해야 합니다. 게이트웨이의 내장 [SSRF 가드](/docs/ko/claude-apps-gateway-deploy#threat-model-summary)가 방어입니다.

        게이트웨이는 메타데이터 엔드포인트에 도달 가능하고 egress NetworkPolicy를 적용할 것을 제안하는 부팅 경고를 기록합니다. Workload Identity에서 해당 경고는 예상되며, 포드가 엔드포인트를 필요로 하기 때문입니다.
      </Tab>
    </Tabs>
  </Step>

  <Step title="게이트웨이 URL을 개발자 머신에 푸시">
    게이트웨이가 이제 실행 중이지만 개발자는 게이트웨이 URL이 머신에 있을 때까지 `/login`에서 도달할 수 없습니다. MDM을 통해 각 디바이스에 배포하는 [관리 설정 파일](/docs/ko/claude-apps-gateway#set-the-gateway-url)에서 `forceLoginMethod` 및 `forceLoginGatewayUrl`을 설정합니다. 개발자가 수동으로 선택할 수 있는 로그인 선택기의 게이트웨이 옵션이 없습니다.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Terraform 참조
</h2>

[참조 배포 자산](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp)은 이 페이지의 Cloud Run 트랙을 자동화합니다. 구성 및 이미지 자산은 두 트랙 모두에 적용됩니다:

* `setup.sh`: API 활성화부터 첫 번째 배포까지 전체 Cloud Run 경로를 거치는 멱등성 `gcloud` 프로비저너
* `terraform/`: 인프라 코드로서의 동일한 배포(greenfield 배포용): Artifact Registry 저장소를 생성하기 위한 대상 적용, 이미지를 빌드 및 푸시, 그 다음 전체 적용
* `gateway.yaml.example` 및 distroless 런타임 이미지를 위한 `Dockerfile`

자산은 기본적으로 Cloud Run ingress를 `internal`로 설정하므로 로드 밸런서가 필요하지 않습니다. 이 페이지의 프로덕션 뒤 ALB 배포와 일치하려면 `setup.sh`를 `INGRESS=internal-and-cloud-load-balancing`으로 실행하거나 Terraform 변수 `ingress`를 `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER`로 설정합니다. 자산은 또한 invoker 계층을 이 페이지의 연습과 반대로 `--no-invoker-iam-check` 대신 `allUsers` `run.invoker` 부여로 기본 설정합니다. 둘 다 작동하며 선택은 조직의 정책 제약에 따라 다릅니다.

자산은 지원되는 프로덕션 아티팩트가 아닌 작동 예제로 제공됩니다. 환경에 맞게 검토하고 조정하세요.

<h2 id="troubleshooting">
  문제 해결
</h2>

게이트웨이 부팅 및 로그인 오류는 플랫폼 독립적인 [문제 해결 표](/docs/ko/claude-apps-gateway-deploy#troubleshooting)를 참조하세요. 아래 항목은 Google Cloud에 특정합니다.

| 증상                                                                               | 원인                                                                         | 해결                                                                                                                                                                             |
| -------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Cloud Run이 컨테이너에 도달하기 전에 `403 Forbidden`을 반환                                     | invoker IAM 확인이 여전히 활성화됨                                                   | `--no-invoker-iam-check`로 배포하거나 `--allow-unauthenticated`로 `allUsers`에 `run.invoker` 역할을 부여                                                                                    |
| `--no-invoker-iam-check`가 `invoker_iam_disabled is not currently available`로 거부됨 | `constraints/run.managed.requireInvokerIam`에 의해 차단됨                        | `--allow-unauthenticated`를 사용합니다. `constraints/iam.allowedPolicyMemberDomains`를 통한 Domain Restricted Sharing도 차단하면 GKE 트랙을 사용하세요. 이는 `allUsers` 바인딩 없이 네트워크 계층에서 게이트웨이를 노출합니다. |
| 배포 시 `Container manifest type … must support amd64/linux`                        | 이미지가 비 amd64 호스트에서 빌드되었거나 buildx가 OCI 이미지 인덱스를 내보냄                         | `--platform=linux/amd64 --provenance=false`로 빌드                                                                                                                                |
| 게이트웨이 부팅이 Cloud Run의 Postgres 연결 타임아웃 오류로 종료됨                                    | 서비스가 VPC에 연결되지 않았거나 Cloud SQL이 해당 VPC에 프라이빗 IP가 없음. 저장소는 5초 후 대기를 중지       | Direct VPC egress를 위해 `--network` 및 `--subnet`으로 배포하고 Cloud SQL 인스턴스를 `--no-assign-ip` 및 동일한 VPC를 가리키는 `--network`로 생성                                                         |
| Google Cloud의 Agent Platform 요청이 `403 PERMISSION_DENIED`를 반환                     | 런타임이 `claude-gateway` 서비스 계정을 사용하지 않거나 모델이 프로젝트의 Model Garden에서 활성화되지 않음   | Cloud Run에서 `--service-account`를 설정하거나 GKE에서 Workload Identity를 바인딩하고 각 Claude 모델을 대상 지역의 Model Garden에서 활성화                                                                   |
| 스트리밍 응답이 고정 기간 후 차단됨                                                             | 프론트 엔드 요청 타임아웃: GKE Ingress 뒤의 로드 밸런서 백엔드 서비스는 기본적으로 30초이고 Cloud Run은 300초 | GKE에서 `timeoutSec`이 높은 BackendConfig를 연결하거나 Cloud Run에서 `--timeout=3600`으로 배포                                                                                                  |

<h2 id="next-steps">
  다음 단계
</h2>

* [구성 참조](/docs/ko/claude-apps-gateway-config): 모든 `gateway.yaml` 옵션(예: `managed.policies` 및 `telemetry`)
* [배포 및 운영](/docs/ko/claude-apps-gateway-deploy): IdP 설정, 상태 확인, JWT 비밀 로테이션, 업그레이드, 보안 모델
* [Claude 앱 게이트웨이 개요](/docs/ko/claude-apps-gateway): 빠른 시작 및 개발자 연결
