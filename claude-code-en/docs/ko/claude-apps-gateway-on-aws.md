> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# AWS에서 Claude 앱 게이트웨이 배포

> AWS에서 Claude 앱 게이트웨이를 실행하는 실제 예제입니다: ECS Fargate 또는 EKS, PostgreSQL용 Amazon RDS, AWS Secrets Manager, Amazon Bedrock에 대한 IAM 역할 인증.

<Note>
  이 페이지는 AWS에서 Claude 앱 게이트웨이를 실행하는 한 가지 방법을 설명합니다. 이 구성은 지원되는 프로덕션 배포가 아닌 고객 관리 인프라의 작동 예제입니다. 이를 사용하여 각 부분이 어떻게 함께 작동하는지 확인한 후 자신의 환경에 맞게 조정하십시오. 플랫폼 독립적인 요구 사항은 [배포 가이드](/docs/ko/claude-apps-gateway-deploy)를 참조하십시오.
</Note>

이 예제는 Amazon Bedrock을 모델 업스트림으로 사용하여 AWS에서 Claude 앱 게이트웨이를 프로비저닝합니다. 컴퓨팅을 위해 [Amazon ECS](https://aws.amazon.com/ecs/)의 [AWS Fargate](https://aws.amazon.com/fargate/) 또는 [Amazon EKS](https://aws.amazon.com/eks/)를 사용합니다. [Okta](https://www.okta.com/)는 예제 ID 공급자(IdP)이지만 모든 OpenID Connect(OIDC) 호환 IdP가 작동합니다. IdP별 세부 정보는 [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하십시오.

<Note>
  Bedrock은 AWS의 유일한 Claude 업스트림이 아닙니다. 게이트웨이는 AWS 인증 및 AWS Marketplace 청구를 사용하는 Anthropic 운영 Claude API인 Claude Platform on AWS도 지원합니다. Bedrock 대신 또는 함께 사용할 수 있습니다. 업스트림 항목, 자격 증명 및 IAM 권한이 이 페이지의 Bedrock 범위 항목과 다릅니다. [Claude Platform on AWS 업스트림 참조](/docs/ko/claude-apps-gateway-config#claude-platform-on-aws)에서 변경 사항을 다룹니다. 이 페이지의 나머지 부분은 변경 없이 적용됩니다.
</Note>

<h2 id="architecture">
  아키텍처
</h2>

<Frame caption="Amazon Bedrock을 모델 업스트림으로 하는 예제 아키텍처입니다. Claude Platform on AWS 업스트림이 동일한 위치를 차지합니다.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="AWS의 Claude 앱 게이트웨이 다이어그램: Claude Code 클라이언트는 HTTPS를 통해 게이트웨이(ECS Fargate 또는 EKS)를 지원하는 내부 Application Load Balancer에 연결되며, 이는 세션 상태를 위한 Amazon RDS for PostgreSQL 인스턴스와 함께 프라이빗 서브넷에서 실행됩니다. 게이트웨이는 기업 IdP에 대해 OIDC를 통해 사용자를 로그인시키고, AWS Secrets Manager에서 시크릿을 읽으며, IAM 역할을 사용하여 Amazon Bedrock으로 모델 요청을 전달하고, 배포 시 Amazon ECR에서 이미지를 가져옵니다." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

게이트웨이는 개발자가 IdP를 통해 로그인하는 네트워크의 프라이빗 HTTPS 엔드포인트로 실행됩니다. Claude Code 세션은 게이트웨이의 IAM 역할을 통해 Amazon Bedrock의 Claude 모델에 도달하므로, 모델 자격 증명이 개발자 머신에 전달되지 않습니다. 참조 구성은 다음을 프로비저닝합니다:

* **AWS Fargate의 Amazon ECS** 서비스 또는 게이트웨이 컨테이너를 실행하는 **Amazon EKS** Deployment
* 게이트웨이 이미지를 위한 **Amazon ECR** 리포지토리
* 게이트웨이의 [저장소](/docs/ko/claude-apps-gateway-config#store)를 위해 프라이빗 서브넷에 있고 공개적으로 접근할 수 없는 **Amazon RDS for PostgreSQL** 인스턴스
* JWT 서명 키, OIDC 클라이언트 시크릿, Postgres URL을 위한 **AWS Secrets Manager** 시크릿
* ECS 작업 역할로 연결되거나 EKS의 IAM Roles for Service Accounts(IRSA)를 통해 바인딩된 `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:CountTokens`를 포함한 **IAM 역할**
* HTTPS를 위한 **내부 Application Load Balancer**

<h2 id="prerequisites">
  전제 조건
</h2>

이 연습은 게이트웨이의 자체 리소스를 생성하지만 이미 있는 네트워크 및 ID 인프라를 기반으로 합니다. 시작하기 전에 다음이 필요합니다:

* [위의 리소스](#architecture)를 생성할 권한이 있는 AWS 계정
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)가 설치되고 [인증](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html)되었으며, [Docker](https://docs.docker.com/get-started/get-docker/)가 로컬에 설치됨
* 서로 다른 가용 영역에 최소 2개의 [프라이빗 서브넷](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html)이 있는 [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html). [NAT 게이트웨이](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html)를 통한 아웃바운드 인터넷 액세스가 있어야 합니다. 내부 로드 밸런서는 2개의 AZ에 서브넷이 필요하고, 게이트웨이는 Bedrock 및 IdP로의 이그레스가 필요합니다.
* 리다이렉트 URI가 `https://<gateway-host>/oauth/callback`인 Okta OIDC 웹 애플리케이션. [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하십시오.
* 게이트웨이용 TLS 호스트명. 일반적으로 [Route 53 프라이빗 호스팅 영역](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html)의 내부 DNS 이름으로 로드 밸런서를 가리키며, 해당 이름에 대한 [ACM 인증서](https://docs.aws.amazon.com/acm/latest/userguide/gs.html)가 있어야 합니다. [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)에서 가져오거나 발급받아야 합니다.

<h3 id="set-your-environment-variables">
  환경 변수 설정
</h3>

이 페이지의 모든 명령은 셸에서 4개의 값을 읽습니다: `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID`, `PRIVATE_SUBNETS`.

필요한 Claude 모델을 제공하는 US 지역을 선택하십시오. 이 연습은 게이트웨이의 기본 제공 모델 카탈로그에 의존하며, 이는 `us.anthropic.*` 추론 프로필로 확인되고 IAM 정책이 해당 ARN을 부여합니다. US가 아닌 지역에서는 해당 지역의 추론 프로필 ID를 포함하는 [`models:` 블록](/docs/ko/claude-apps-gateway-config#models)을 추가하고 IAM 정책의 ARN 접두사를 일치하도록 변경하십시오.

VPC ID가 없으면 `aws ec2 describe-vpcs`로 VPC를 나열한 다음 해당 VPC의 서브넷을 나열하여 서로 다른 가용 영역에 있는 2개의 프라이빗 서브넷을 찾으십시오:

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

계속하기 전에 4개를 모두 내보내십시오:

```bash theme={null}
export AWS_REGION=us-east-1   # Bedrock이 필요한 Claude 모델을 제공하는 US 지역
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  게이트웨이 배포
</h2>

아래 단계는 `aws` 명령으로 전체 배포를 프로비저닝합니다.

<Steps>
  <Step title="보안 그룹 생성">
    3개의 보안 그룹이 트래픽 경로를 연결합니다: 회사 네트워크는 443에서 로드 밸런서에 도달하고, 로드 밸런서는 8080에서 게이트웨이에 도달하며, 게이트웨이는 5432에서 Postgres에 도달합니다. 다른 것은 도달할 수 없습니다. 이를 연결하는 방법은 컴퓨팅 트랙에 따라 다릅니다:

    * ECS Fargate에서 배포 단계는 `$ALB_SG`를 로드 밸런서에 연결하고 `$GW_SG`를 서비스에 연결합니다.
    * EKS에서 AWS Load Balancer Controller는 ALB용 자체 프론트엔드 보안 그룹을 생성하므로 `$ALB_SG`와 `$GW_SG`는 사용되지 않습니다: 배포 단계의 `inbound-cidrs` 주석이 리스너를 회사 네트워크로 제한하고, 데이터베이스 보안 그룹은 `$GW_SG` 대신 클러스터의 보안 그룹을 허용합니다.

    ```bash theme={null}
    ALB_SG="$(aws ec2 create-security-group --group-name claude-gateway-alb \
      --description "Claude gateway ALB" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    GW_SG="$(aws ec2 create-security-group --group-name claude-gateway-svc \
      --description "Claude gateway service" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"
    DB_SG="$(aws ec2 create-security-group --group-name claude-gateway-db \
      --description "Claude gateway Postgres" --vpc-id "$VPC_ID" \
      --query GroupId --output text)"

    aws ec2 authorize-security-group-ingress --group-id "$ALB_SG" \
      --protocol tcp --port 443 --cidr <your-corporate-cidr>
    aws ec2 authorize-security-group-ingress --group-id "$GW_SG" \
      --protocol tcp --port 8080 --source-group "$ALB_SG"
    aws ec2 authorize-security-group-ingress --group-id "$DB_SG" \
      --protocol tcp --port 5432 --source-group "$GW_SG"
    ```
  </Step>

  <Step title="IAM 역할 생성 및 사용 사례 양식 제출">
    게이트웨이는 Bedrock에서 Claude 모델을 호출하는 유일한 권한을 가진 전용 작업 역할로 실행됩니다. [Bedrock 업스트림 참조](/docs/ko/claude-apps-gateway-config#amazon-bedrock)에 따르면 정책은 교차 지역 추론 프로필 ARN과 기본 기초 모델 ARN을 모두 포함해야 합니다:

    ```bash theme={null}
    cat > bedrock-invoke.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["bedrock:InvokeModel", "bedrock:InvokeModelWithResponseStream", "bedrock:CountTokens"],
        "Resource": [
          "arn:aws:bedrock:${AWS_REGION}:${ACCOUNT_ID}:inference-profile/us.anthropic.*",
          "arn:aws:bedrock:*::foundation-model/anthropic.*"
        ]
      }]
    }
    EOF
    cat > ecs-trust.json <<'EOF'
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Principal": { "Service": "ecs-tasks.amazonaws.com" },
        "Action": "sts:AssumeRole"
      }]
    }
    EOF

    aws iam create-role --role-name claude-gateway-task \
      --assume-role-policy-document file://ecs-trust.json
    aws iam put-role-policy --role-name claude-gateway-task \
      --policy-name bedrock-invoke --policy-document file://bedrock-invoke.json
    ```

    ECS는 또한 실행 역할이 필요합니다. ECS 에이전트 자체가 ECR에서 이미지를 가져오고 나중에 생성된 Secrets Manager 값을 주입하는 데 사용합니다. 이는 게이트웨이의 AWS SDK가 런타임에 사용하는 작업 역할과 별개입니다:

    ```bash theme={null}
    aws iam create-role --role-name claude-gateway-execution \
      --assume-role-policy-document file://ecs-trust.json
    aws iam attach-role-policy --role-name claude-gateway-execution \
      --policy-arn arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy
    cat > secrets-read.json <<EOF
    {
      "Version": "2012-10-17",
      "Statement": [{
        "Effect": "Allow",
        "Action": ["secretsmanager:GetSecretValue", "secretsmanager:DescribeSecret"],
        "Resource": [
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-jwt-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-oidc-client-secret-??????",
          "arn:aws:secretsmanager:${AWS_REGION}:${ACCOUNT_ID}:secret:gateway-postgres-url-??????"
        ]
      }]
    }
    EOF
    aws iam put-role-policy --role-name claude-gateway-execution \
      --policy-name read-gateway-secrets --policy-document file://secrets-read.json
    ```

    정책은 각 비밀마다 하나의 ARN을 이름으로 지정하며, 공유 계정에서 관련 없는 비밀과도 일치하는 일반 `gateway-*` 와일드카드가 아닙니다. 뒤의 `-??????`는 Secrets Manager가 모든 비밀의 ARN에 추가하는 임의의 6자 접미사와 정확히 일치합니다. 뒤의 `-*`는 일반 접두사 glob이며 `gateway-postgres-url-prod`와 같은 더 긴 이름과도 일치합니다.

    IAM 정책은 게이트웨이에 Bedrock을 호출할 권한을 부여하고, Bedrock은 상용 지역에서 기본적으로 모델 액세스를 활성화합니다. 남은 계정 수준 게이트는 Anthropic의 일회성 사용 사례 양식입니다: 계정의 누구도 제출하지 않았다면 [Amazon Bedrock 콘솔](https://console.aws.amazon.com/bedrock/)을 열고 모델 카탈로그에서 Anthropic 모델을 선택한 후 양식을 완료하십시오. 제출 직후 액세스가 부여됩니다. [Amazon Bedrock의 Claude Code](/docs/ko/amazon-bedrock#1-submit-use-case-details)에서 AWS Organizations 양식 및 제출자가 필요한 IAM 권한을 참조하십시오.

    EKS 트랙은 2개의 ECS 역할 대신 IRSA 역할에서 두 정책 문서를 재사용합니다. 배포 단계를 참조하십시오.
  </Step>

  <Step title="PostgreSQL용 Amazon RDS 프로비저닝">
    인스턴스는 공개 주소가 없는 프라이빗 서브넷에서 실행되며 스토리지 암호화가 켜져 있습니다. 엔진 버전은 Postgres 16으로 고정되어 있으며, 이는 게이트웨이의 지원되는 최소값인 PostgreSQL 14를 충족하고 아래 매개변수 그룹 패밀리가 인스턴스가 실행하는 엔진 주 버전과 일치함을 보장합니다.

    먼저 프라이빗 서브넷에 데이터베이스를 배치하는 서브넷 그룹과 `rds.force_ssl=1`을 사용하는 매개변수 그룹을 생성하여 서버가 일반 텍스트 연결을 거부하도록 합니다. 엔진 버전은 매개변수 그룹의 패밀리가 인스턴스가 실행하는 엔진 주 버전과 일치해야 하므로 한 번만 고정됩니다:

    ```bash theme={null}
    aws rds create-db-subnet-group --db-subnet-group-name claude-gateway-db \
      --db-subnet-group-description "Claude gateway" --subnet-ids $PRIVATE_SUBNETS

    PG_VERSION=16
    PG_FAMILY="postgres${PG_VERSION}"
    aws rds create-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --db-parameter-group-family "$PG_FAMILY" \
      --description "Claude gateway - require TLS on every connection"
    aws rds modify-db-parameter-group --db-parameter-group-name claude-gateway-db \
      --parameters "ParameterName=rds.force_ssl,ParameterValue=1,ApplyMethod=immediate"
    ```

    그런 다음 생성된 마스터 암호로 인스턴스를 생성합니다:

    ```bash theme={null}
    PGPASS="$(openssl rand -hex 24)"
    aws rds create-db-instance --db-instance-identifier claude-gateway-db \
      --engine postgres --engine-version "$PG_VERSION" \
      --db-instance-class db.t4g.micro \
      --allocated-storage 20 --db-name claude_gateway \
      --master-username gateway --master-user-password "$PGPASS" \
      --db-subnet-group-name claude-gateway-db \
      --db-parameter-group-name claude-gateway-db \
      --vpc-security-group-ids "$DB_SG" \
      --no-publicly-accessible --storage-encrypted
    ```

    리터럴 `--master-user-password` 인수는 명령이 실행되는 동안 프로세스 테이블 및 감사/EDR 로그에 표시됩니다. 공유 또는 모니터링되는 호스트에서는 번들의 `setup.sh`가 하는 방식처럼 `0600` 파일에서 `--cli-input-json`을 통해 암호를 전달하십시오.

    인스턴스가 시작될 때까지 기다리십시오. 몇 분이 걸릴 수 있습니다. 그런 다음 프라이빗 엔드포인트를 읽고 게이트웨이가 사용할 연결 문자열을 조합하십시오:

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full`은 게이트웨이가 RDS 서버 인증서의 체인과 호스트명을 확인하도록 하며, 암호화만 하지 않습니다. 신뢰 앵커는 [AWS RDS 인증서 번들](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem)이며, 아래의 이미지 빌드 단계는 이를 `/etc/claude/rds-global-bundle.pem`에 복사하고 `NODE_EXTRA_CA_CERTS`를 통해 신뢰합니다. libpq 스타일 `sslrootcert=` 매개변수를 URL에 추가하지 마십시오: 게이트웨이의 드라이버는 쿼리 문자열에서 `sslmode`만 읽고 `sslrootcert`를 Postgres 시작 매개변수로 전달하며, 서버가 이를 거부합니다.

    ECS 서비스 또는 EKS 포드는 인스턴스의 프라이빗 엔드포인트에 도달할 수 있도록 이 VPC에서 실행되어야 하며, `claude-gateway-db` 보안 그룹은 게이트웨이의 보안 그룹만 허용합니다.
  </Step>

  <Step title="gateway.yaml 작성">
    `upstreams` 블록은 `auth: {}`로 Bedrock을 가리키므로 게이트웨이는 ECS의 작업 역할 또는 EKS의 IRSA 역할에서 AWS 기본 자격 증명 체인을 통해 인증합니다. 모든 필드는 [구성 참조](/docs/ko/claude-apps-gateway-config)를 참조하십시오.

    2개의 `listen` 필드는 게이트웨이 앞에 있는 것에 따라 다릅니다:

    * `public_url`: 외부 `https://` 원점이며, 비루프백 바인드에 필수입니다. [listen 참조](/docs/ko/claude-apps-gateway-config#listen)를 참조하십시오. 게이트웨이는 IdP `redirect_uri`와 검색 문서를 이 값에서만 빌드하며, `X-Forwarded-*` 헤더에서는 빌드하지 않습니다.
    * `trusted_proxies`: 프론트 엔드의 소스 범위입니다. 게이트웨이는 TCP 피어가 이 목록에 있을 때만 `X-Forwarded-For`를 준수하고, 신뢰할 수 있는 홉을 지나 체인을 걷습니다. 따라서 IP별 로그인 속도 제한 및 감사 이벤트는 로드 밸런서의 IP가 아닌 개발자 IP를 기록합니다.

    두 트랙 모두에서 프론트 엔드는 직접 생성되거나 AWS Load Balancer Controller에 의해 생성되는 내부 ALB이며, ALB의 노드는 연결된 서브넷에서 주소를 가져오므로 `trusted_proxies`를 해당 서브넷의 CIDR로 설정하십시오. 이는 해당 서브넷의 모든 호스트를 프록시로 신뢰합니다. ALB의 수신 소스인 회사 CIDR이 이들과 겹치지 않도록 유지하고, 신뢰할 수 없는 워크로드와 서브넷을 공유하지 마십시오. 이들은 `X-Forwarded-For`를 통해 클라이언트 IP를 스푸핑할 수 있습니다.

    ALB의 클라이언트 포트 보존 속성인 `routing.http.xff_client_port.enabled`는 어느 설정이든 유지할 수 있습니다: 켜져 있으면 ALB는 클라이언트를 `203.0.113.7:54321` 또는 `[2001:db8::1]:54321`으로 작성하고, 게이트웨이는 포트가 제거된 둘 다를 읽습니다.

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      public_url: https://claude-gateway.internal.example.com
      trusted_proxies: [<your-alb-subnet-cidrs>]

    oidc:
      issuer: https://example.okta.com
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}           # EKS: ${file:/secrets/oidc-client-secret}
      allowed_email_domains: [example.com]
      # Okta org 인증 서버는 이메일과 그룹을 생략하는 얇은 id_token을 반환합니다.
      # 게이트웨이는 /userinfo에서 이들을 채웁니다.
      userinfo_fallback: true
      # Okta는 `groups` 범위가 요청되고 앱의 그룹 클레임 필터가 이를 허용할 때만 그룹을 내보냅니다.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # 프로비저닝 해제 지연을 제한합니다. 더 엄격한 취소를 위해 1로 낮추십시오.

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # IAM 정책의 ARN이 이를 포함하도록 $AWS_REGION과 일치합니다.
        auth: {} # AWS 기본 자격 증명 체인: ECS 작업 역할 또는 EKS의 IRSA
    ```

    <Note>
      `oidc` 블록만 Okta 특정입니다. Microsoft Entra ID를 대신 사용하려면 `issuer`를 `https://login.microsoftonline.com/<tenant-id>/v2.0`으로 설정하고, `userinfo_fallback`과 `groups` 범위를 제거하며, Entra가 그룹 이름이 아닌 그룹 Object ID를 내보낸다는 점에 유의하십시오. 따라서 [`managed.policies`](/docs/ko/claude-apps-gateway-config#managed)는 GUID에서 일치하거나 `oidc.groups_claim: roles`를 사용하는 App Roles에서 일치해야 합니다. [ID 공급자 설정](/docs/ko/claude-apps-gateway-deploy#identity-provider-setup)을 참조하십시오.
    </Note>
  </Step>

  <Step title="AWS Secrets Manager에 비밀 저장">
    3개의 비밀을 생성합니다. IAM 단계의 실행 역할은 이미 이들을 읽을 수 있습니다:

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    각 호출이 인쇄하는 ARN을 기록하십시오. ECS 작업 정의는 ARN으로 비밀을 참조합니다.

    <Note>
      리터럴 `--secret-string` 인수는 각 명령이 실행되는 동안 프로세스 테이블 및 감사/EDR 로그에 표시됩니다. 공유 또는 모니터링되는 호스트에서는 값을 `0600` 파일에 넣고 대신 `--secret-string file://<path>`를 전달하십시오. 번들의 `setup.sh`는 `--cli-input-json`에 `0600` 임시 파일을 전달하는 방식으로 비밀 값을 프로세스 argv에 노출하지 않습니다.
    </Note>

    비밀과 달리 `gateway.yaml` 자체는 모든 자격 증명이 [`${VAR}` 또는 `${file:...}` 확장](/docs/ko/claude-apps-gateway-config#secret-expansion)을 통해 부팅 시 확인되므로 비밀 값을 포함하지 않습니다. 모든 것이 컨테이너에 도달하는 방식은 트랙에 따라 다릅니다:

    * ECS에서 다음 단계의 빌드는 `gateway.yaml`을 이미지에 `/etc/claude/gateway.yaml`로 복사하고, 작업 정의는 3개의 비밀을 `secrets` 필드를 통해 환경 변수로 주입하므로 YAML은 `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}`, `${GATEWAY_POSTGRES_URL}`을 참조합니다.
    * EKS에서 ConfigMap에서 `gateway.yaml`을 마운트하고 비밀을 `/secrets`의 파일로 마운트하며, `${file:/secrets/...}`로 참조합니다. External Secrets Operator 또는 Secrets Store CSI 드라이버의 AWS 공급자를 사용하여 Secrets Manager에서 Kubernetes Secrets를 소싱하거나 `kubectl`로 직접 생성하십시오.
  </Step>

  <Step title="이미지를 Amazon ECR에 빌드 및 푸시">
    [컨테이너 이미지 요구 사항](/docs/ko/claude-apps-gateway-deploy#container-image)에 따라 이미지를 빌드하고, `linux-x64` glibc 바이너리를 빌드 컨텍스트의 `./claude`에 배치하십시오. 해당 요구 사항에 따라 자신의 Dockerfile을 작성하거나 번들의 [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile)에서 시작하십시오. 이는 이전 단계의 채워진 `gateway.yaml`을 이미지의 `/etc/claude/gateway.yaml`에 복사합니다. ECS에서 이 임베드된 복사본은 구성이 컨테이너에 도달하는 방식이며, 이것이 빌드가 파일 작성 후에 오는 이유입니다. EKS 트랙은 대신 배포 시 ConfigMap에서 `gateway.yaml`을 마운트하므로 임베드된 복사본은 거기서 사용되지 않습니다.

    이미지는 또한 연결 문자열의 `sslmode=verify-full`에 대한 신뢰 앵커로 AWS RDS 인증서 번들을 전달합니다. 따라서 빌드 컨텍스트에 먼저 다운로드하십시오. AWS는 번들을 회전시킵니다(새 지역 CA가 추가됨). 따라서 체크섬을 고정하거나 커밋하기보다는 빌드당 다운로드하십시오:

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    컨테이너 이미지 요구 사항은 번들을 다루지 않으므로 자신의 Dockerfile을 작성하면 번들을 복사하고 신뢰하는 2개의 줄을 추가하십시오. 번들의 `Dockerfile`은 이미 둘 다 포함합니다:

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    ECR 리포지토리를 생성하고 Docker를 로그인하십시오. 불변 태그는 배포 단계가 고정하는 `<version>` 태그를 나중에 다른 이미지로 자동으로 다시 가리킬 수 없음을 의미합니다:

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    이미지를 빌드하고 푸시하십시오. 아래의 작업 정의는 `linux/amd64`를 실행하므로 플랫폼이 여기서 일치해야 합니다. Fargate on ARM64(Graviton)의 경우 `linux-arm64` 바이너리로 `linux/arm64`를 빌드하고 `cpuArchitecture`를 대신 `ARM64`로 설정하십시오:

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="배포">
    <Tabs>
      <Tab title="ECS Fargate">
        클러스터와 게이트웨이의 stderr에 대한 로그 그룹을 생성합니다. 이는 감사 이벤트와 운영 로그를 모두 전달합니다. 보존은 별도의 호출이며, 보존이 없으면 CloudWatch는 로그를 영구적으로 유지합니다. 90일을 감사 보존 정책과 정렬하십시오:

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        작업 정의를 작성하십시오. 작업 역할은 Bedrock 권한을 전달하고 실행 역할은 비밀을 주입합니다. Secrets Manager 단계의 비밀 ARN을 사용하십시오:

        ```json claude-gateway-task.json theme={null}
        {
          "family": "claude-gateway",
          "networkMode": "awsvpc",
          "requiresCompatibilities": ["FARGATE"],
          "cpu": "1024",
          "memory": "2048",
          "runtimePlatform": { "cpuArchitecture": "X86_64", "operatingSystemFamily": "LINUX" },
          "executionRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-execution",
          "taskRoleArn": "arn:aws:iam::<account-id>:role/claude-gateway-task",
          "containerDefinitions": [
            {
              "name": "gateway",
              "image": "<account-id>.dkr.ecr.<region>.amazonaws.com/claude-gateway:<version>",
              "portMappings": [{ "containerPort": 8080 }],
              "secrets": [
                { "name": "GATEWAY_JWT_SECRET",   "valueFrom": "<gateway-jwt-secret ARN>" },
                { "name": "OIDC_CLIENT_SECRET",   "valueFrom": "<gateway-oidc-client-secret ARN>" },
                { "name": "GATEWAY_POSTGRES_URL", "valueFrom": "<gateway-postgres-url ARN>" }
              ],
              "logConfiguration": {
                "logDriver": "awslogs",
                "options": {
                  "awslogs-group": "/ecs/claude-gateway",
                  "awslogs-region": "<region>",
                  "awslogs-stream-prefix": "gateway"
                }
              }
            }
          ]
        }
        ```

        등록하십시오:

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        게이트웨이를 상태 확인하는 대상 그룹으로 내부 ALB를 앞단에 두십시오. `--ip-address-type ipv4`는 중요합니다: 내부 이중 스택 ALB는 공개 범위 AAAA 레코드를 게시하며, `/login` 프라이빗 네트워크 확인이 이를 거부합니다:

        ```bash theme={null}
        ALB_ARN="$(aws elbv2 create-load-balancer --name claude-gateway \
          --scheme internal --type application --ip-address-type ipv4 \
          --subnets $PRIVATE_SUBNETS --security-groups "$ALB_SG" \
          --query 'LoadBalancers[0].LoadBalancerArn' --output text)"

        TG_ARN="$(aws elbv2 create-target-group --name claude-gateway \
          --protocol HTTP --port 8080 --vpc-id "$VPC_ID" --target-type ip \
          --health-check-path /readyz \
          --query 'TargetGroups[0].TargetGroupArn' --output text)"
        ```

        HTTPS 리스너를 추가합니다. `--ssl-policy`는 최신 TLS 하한을 고정합니다. 생략하면 여전히 TLS 1.0/1.1을 허용하는 레거시 `ELBSecurityPolicy-2016-08` 기본값으로 돌아갑니다.

        ALB는 기본적으로 60초 동안 데이터가 없는 연결을 닫습니다. 게이트웨이의 keepalive 핑은 스트림을 해당 기본값 내에 유지하므로 시간 초과를 높이면 핑 주기 위에 여유를 추가합니다. [문제 해결](#troubleshooting) 행에서 끊어진 스트림을 다룹니다. 아래 명령은 리스너를 추가하고 시간 초과를 높입니다:

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        서비스를 생성하십시오. 배포 회로 차단기는 나쁜 이미지 또는 부팅할 수 없는 구성으로 인해 작업이 계속 실패하는 배포를 마지막 안정 상태로 롤백합니다. 실패한 작업을 영구적으로 다시 시작하지 않습니다:

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        60초 유예 기간은 콜드 작업이 이미지를 가져오고, 저장소에 연결하고, 첫 번째 상태 확인에 응답할 시간을 제공합니다. ECS가 배포에 대한 실패를 계산하기 시작하기 전입니다. 대상 그룹의 `GET /readyz`에 대한 상태 확인은 저장소에 도달할 수 있는지 확인하므로 Postgres에 도달할 수 없는 작업은 회전에 들어가지 않습니다. [중단 동작](/docs/ko/claude-apps-gateway-deploy#outage-behavior)에서 트레이드오프와 `/healthz` 대안을 참조하십시오.

        작업은 공개 IP가 없는 프라이빗 서브넷에서 실행되므로 모든 이그레스(Bedrock, IdP, Secrets Manager, ECR, CloudWatch Logs로)는 NAT 게이트웨이를 통해 이동합니다. Bedrock 트래픽을 공개 경로에서 벗어나게 하려면 `bedrock-runtime` 인터페이스 VPC 엔드포인트를 생성하고 업스트림의 `base_url`을 가리키십시오. [Bedrock 업스트림 참조](/docs/ko/claude-apps-gateway-config#amazon-bedrock)에 표시됩니다. IdP는 여전히 인터넷 이그레스가 필요합니다.

        개발자에게 프라이빗으로 확인할 수 있는 호스트명을 제공하여 마무리하십시오: Route 53 프라이빗 호스팅 영역에서 게이트웨이의 내부 DNS 이름을 ALB에 별칭으로 지정하고 `listen.public_url`을 해당 호스트명으로 설정하십시오. ALB의 자체 `*.elb.amazonaws.com` 이름은 내부 ALB에서 프라이빗 주소로 확인되지만 ACM 인증서를 전달할 수 없으므로 자신의 이름을 사용하십시오.

        첫 번째 로그인 전에 OAuth 클라이언트의 인증된 리다이렉트 URI를 `<public_url>/oauth/callback`으로 업데이트하십시오. `public_url`을 변경한 후 새 태그 아래에서 이미지를 다시 빌드하고 푸시하고, 새 작업 정의 개정을 등록하고, 다시 배포하십시오. ECS에서 설정은 이미지의 임베드된 `gateway.yaml`에 있으며, 게이트웨이는 해당 설정에서만 공개 원점을 빌드하고 `X-Forwarded-Host` 및 `X-Forwarded-Proto`를 무시합니다. `X-Forwarded-For`는 `listen.trusted_proxies`가 설정되었을 때만 클라이언트 IP에 대해 준수됩니다.
      </Tab>

      <Tab title="EKS">
        이 트랙은 `kubectl` 및 `eksctl`이 로컬에 설치되어 있어야 하며, IAM OIDC 공급자와 AWS Load Balancer Controller가 설치된 기존 EKS 클러스터가 필요합니다. 클러스터는 포드가 RDS 프라이빗 엔드포인트에 도달할 수 있도록 `$VPC_ID`에 있어야 하며, `claude-gateway-db` 보안 그룹은 `$GW_SG` 대신 클러스터의 포드 또는 노드 보안 그룹을 허용해야 합니다.

        EKS에서 게이트웨이는 ECS 역할 대신 IRSA를 통해 Bedrock 자격 증명을 가져옵니다. IAM 단계의 `ecs-tasks.amazonaws.com` 신뢰 정책은 여기에 적용되지 않습니다. IRSA는 클러스터의 OIDC 공급자에서 페더레이션하는 신뢰 정책이 필요하며, `system:serviceaccount:claude-gateway:gateway`로 범위가 지정됩니다. `eksctl create iamserviceaccount`는 해당 역할을 생성하고, 정책을 연결하고, Kubernetes 서비스 계정에 한 단계로 역할 ARN을 주석으로 답니다. IAM 단계의 2개 정책 문서를 관리형 정책으로 전환하여 연결할 수 있습니다:

        ```bash theme={null}
        BEDROCK_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-bedrock-invoke \
          --policy-document file://bedrock-invoke.json --query Policy.Arn --output text)"
        SECRETS_POLICY_ARN="$(aws iam create-policy --policy-name claude-gateway-secrets-read \
          --policy-document file://secrets-read.json --query Policy.Arn --output text)"

        kubectl create namespace claude-gateway
        eksctl create iamserviceaccount --cluster <your-cluster> --region "$AWS_REGION" \
          --namespace claude-gateway --name gateway --role-name claude-gateway \
          --attach-policy-arn "$BEDROCK_POLICY_ARN" \
          --attach-policy-arn "$SECRETS_POLICY_ARN" \
          --approve
        ```

        비밀 정책은 Secrets Store CSI 드라이버의 AWS 공급자가 마운팅 포드의 서비스 계정을 사용하여 수행하는 것처럼 포드가 Secrets Manager 자체를 읽을 때만 필요합니다. 다른 방식으로 Kubernetes Secrets를 생성하면 제거하십시오. 공급자는 정책의 두 작업이 모두 필요합니다: 회전된 비밀을 조정할 때 `DescribeSecret`을 호출하므로 `GetSecretValue` 전용 부여는 첫 번째 배포에서 마운트되지만 회전 선택을 중지합니다.

        [Kubernetes 배포](/docs/ko/claude-apps-gateway-deploy#kubernetes)에 설명된 대로 표준 Deployment, Service 및 Ingress로 게이트웨이를 배포하십시오. 다음을 포함합니다:

        * `serviceAccountName: gateway`
        * ConfigMap에서 마운트된 `gateway.yaml` 및 `/secrets`에 파일로 마운트된 비밀
        * `GET /readyz`를 가리키는 준비 프로브

        프론트 엔드의 경우 AWS Load Balancer Controller에서 관리하는 Ingress가 내부 ALB를 프로비저닝합니다. 다음으로 주석을 달아야 합니다:

        * `alb.ingress.kubernetes.io/scheme: internal` 및 `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`. 따라서 `/login` [프라이빗 네트워크 확인](/docs/ko/claude-apps-gateway#prerequisites)이 거부할 공개 범위 AAAA 레코드가 게시되지 않습니다.
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`. 따라서 컨트롤러 관리 프론트엔드 보안 그룹은 `0.0.0.0/0` 기본값 대신 회사 네트워크만 허용합니다.
        * `alb.ingress.kubernetes.io/certificate-arn`과 ACM 인증서
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`. 따라서 리스너는 TLS 1.0 및 1.1을 허용하는 레거시 기본 정책으로 돌아가지 않습니다.
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`. 게이트웨이의 스트리밍 keepalive 위의 여유입니다. [문제 해결](#troubleshooting)을 참조하십시오.

        IRSA를 사용하면 AWS SDK는 프로젝션된 서비스 계정 토큰을 읽고 AWS STS와 교환하므로 포드는 EC2 인스턴스 메타데이터 서비스가 필요하지 않습니다. 이그레스 NetworkPolicy는 게이트웨이 포드에 대해 `169.254.169.254`를 차단할 수 있습니다. 아래 [문제 해결](#troubleshooting)의 노드 홉 제한 문제는 IRSA를 건너뛰고 노드 인스턴스 역할에 의존하는 클러스터에만 적용됩니다.
      </Tab>
    </Tabs>
  </Step>

  <Step title="게이트웨이 URL을 개발자 머신에 푸시">
    게이트웨이가 이제 실행 중이지만 개발자는 게이트웨이 URL이 머신에 있을 때까지 `/login`에서 도달할 수 없습니다. [관리형 설정 파일](/docs/ko/claude-apps-gateway#set-the-gateway-url)에서 `forceLoginMethod` 및 `forceLoginGatewayUrl`을 설정하고 MDM을 통해 각 디바이스에 배포하십시오. 개발자가 수동으로 선택할 수 있는 로그인 선택기의 게이트웨이 옵션이 없습니다.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Terraform 참조
</h2>

[`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws)의 동반 번들은 이 페이지를 코드로 패키징합니다:

* **`setup.sh`** 는 ECS Fargate 트랙에서 동일한 `aws` 명령으로 위의 프로비저닝 연습을 스크립트화합니다. 멱등성을 가집니다: 기존 리소스는 감지되고 건너뛰어지므로 다시 실행해도 안전하며, 모든 기본값은 환경 변수를 통해 재정의할 수 있습니다. Okta OIDC 클라이언트 시크릿과 ACM 인증서는 여전히 직접 생성합니다: 이들 없이 실행하면 ECS/ALB 배포를 건너뛰고, 누락된 입력을 이름 지으며, `create-secret` 명령을 출력합니다. 둘 다 생성하고 다시 실행하세요. Bedrock 사용 사례 양식과 Route 53 별칭은 자동으로 실행되지 않고 다음 단계로 출력되며, 클라이언트 MDM 푸시는 이 페이지에서 수동 단계로 유지됩니다.
* **`gateway.yaml.example`** 는 gateway.yaml 단계의 구성 템플릿이며, 선택적 키는 주석 처리되어 포함됩니다. 이를 `gateway.yaml`로 복사하고 빌드하기 전에 모든 `REPLACE_ME`를 바꾸세요.
* **`Dockerfile`** 은 미리 빌드된 `linux-x64` 바이너리에서 런타임 이미지를 빌드하고 채워진 `gateway.yaml`을 `/etc/claude/gateway.yaml`에 복사하며, 저장소의 `sslmode=verify-full`을 고정하는 AWS RDS 인증서 번들을 복사합니다. `setup.sh`는 번들이 빌드 컨텍스트에 아직 없을 때만 번들을 다운로드합니다. AWS CA 로테이션을 선택하려면 파일을 삭제하고 새 태그로 다시 빌드하세요. 구성 파일은 모든 자격 증명이 부팅 시 `${VAR}` 확장을 통해 해결되므로 시크릿 값을 보유하지 않습니다. 따라서 구성 편집은 새 태그로 다시 빌드하는 것을 의미합니다. `setup.sh`는 파일의 해시로 이미지에 태그를 지정하여 이를 자동화합니다.
* **`terraform/`** 는 동일한 ECS Fargate 범위를 선언적으로 프로비저닝합니다: 보안 그룹, IAM 역할, ECR 리포지토리, RDS 인스턴스, Secrets Manager 시크릿, 및 내부 ALB 뒤의 ECS 서비스. VPC와 프라이빗 서브넷은 전제 조건으로 유지되며, 변수로 전달됩니다. Terraform은 ECR 리포지토리를 생성하지만 이미지를 빌드하지 않으며, 서비스 정의는 이미지를 참조하므로 적용은 두 번의 패스입니다: 리포지토리에 대한 대상 적용, 그 다음 빌드 및 푸시, 그 다음 전체 적용. 번들의 `terraform/README.md`는 변수, 원격 상태, 및 정리를 다룹니다.

이 페이지와 마찬가지로 번들은 지원되는 프로덕션 배포가 아닌 고객 관리 인프라에 대한 작동 예제입니다. 이에 의존하기 전에 자신의 환경에 맞게 검토하고 조정하세요.

<h2 id="troubleshooting">
  문제 해결
</h2>

게이트웨이 부팅 및 로그인 오류는 플랫폼 독립적인 [문제 해결 테이블](/docs/ko/claude-apps-gateway-deploy#troubleshooting)을 참조하십시오. 아래 항목은 AWS에 특정합니다.

| 증상                                                                                                                                         | 원인                                                                                                                                                                                                                                                                                                                                       | 해결                                                                                                                                                                                                                                   |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | 게이트웨이 이름이 최소 하나의 공개 주소로 확인됩니다. 이중 스택 내부 ALB는 공개 범위 AAAA 레코드를 게시하며, [프라이빗 네트워크 확인](/docs/ko/claude-apps-gateway#prerequisites)은 모든 확인된 주소가 프라이빗이어야 합니다.                                                                                                                                                                                        | `--ip-address-type ipv4`로 ALB를 생성하거나 공개 AAAA 레코드가 없는 별도의 내부 전용 DNS 이름을 제공하십시오.                                                                                                                                                       |
| 모든 Bedrock 요청이 502를 반환합니다. 로그는 `Could not load credentials from any providers`를 표시합니다.                                                     | 작업이 작업 역할 없이 ECS EC2 시작 유형에서 실행되거나, 포드가 IRSA 없이 EKS 노드에서 실행되므로 자격 증명은 인스턴스 메타데이터에서 오며, IMDSv2의 기본 홉 제한 1이 컨테이너 내부에서 중지됩니다. 이 페이지의 두 트랙 모두 영향을 받지 않습니다: Fargate 작업 역할과 IRSA는 인스턴스 메타데이터를 사용하지 않습니다.                                                                                                                                       | 작업 역할과 IRSA를 선호하십시오. 인스턴스 자격 증명이 불가피한 경우 `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`로 홉 제한을 높이십시오. [플랫폼 독립적인 테이블](/docs/ko/claude-apps-gateway-deploy#troubleshooting)은 트레이드오프를 다룹니다.   |
| Bedrock 요청이 `403 AccessDeniedException`을 반환합니다.                                                                                            | 계정이 Anthropic의 일회성 사용 사례 양식을 제출하지 않았거나, 계정의 첫 호출에서 시작되는 자동 AWS Marketplace 구독이 아직 완료되지 않았거나, 작업 역할의 정책이 추론 프로필 또는 기초 모델 ARN을 누락했습니다.                                                                                                                                                                                                     | Bedrock 콘솔의 모델 카탈로그에서 사용 사례 양식을 제출하십시오. 방금 제출되었거나 이것이 계정의 첫 호출인 경우 몇 분 후에 다시 시도하십시오. `bedrock:InvokeModel` 및 `bedrock:InvokeModelWithResponseStream`을 두 ARN 패밀리에 부여하십시오.                                                             |
| Bedrock이 온디맨드 처리량이 지원되지 않는다고 말하는 `ValidationException`을 반환합니다.                                                                             | 사용자 정의 `models:` 항목이 지역이 추론 프로필을 통해서만 제공하는 일반 기초 모델 ID로 매핑됩니다.                                                                                                                                                                                                                                                                           | 모델을 교차 지역 추론 프로필 ID(`us.anthropic.*`)로 매핑하십시오. 기본 제공 카탈로그는 이미 이를 수행합니다.                                                                                                                                                              |
| ECS 작업이 게이트웨이가 아무것도 로깅하기 전에 `ResourceInitializationError`로 중지됩니다.                                                                          | 실행 역할이 Secrets Manager 비밀을 읽을 수 없거나, 프라이빗 서브넷이 Secrets Manager 또는 ECR로의 경로가 없습니다.                                                                                                                                                                                                                                                        | 실행 역할에 3개의 `gateway-` 비밀 ARN에 대해 `secretsmanager:GetSecretValue`를 부여하고, NAT 게이트웨이를 통해 이그레스를 제공하거나, 없으면 Secrets Manager, ECR, CloudWatch Logs에 대한 인터페이스 엔드포인트를 제공하십시오. `awslogs` 드라이버는 동일한 단계에서 이들이 필요합니다. 또한 S3 게이트웨이 엔드포인트를 제공하십시오. |
| 게이트웨이 부팅이 Postgres 연결 시간 초과 오류로 종료됩니다.                                                                                                     | 데이터베이스 보안 그룹이 5432에서 게이트웨이의 보안 그룹을 허용하지 않거나, 서비스가 데이터베이스의 VPC 외부에서 실행됩니다.                                                                                                                                                                                                                                                                | 데이터베이스의 보안 그룹에서 게이트웨이의 보안 그룹의 5432를 허용하고, 서비스를 DB 서브넷 그룹과 동일한 VPC에서 실행하십시오.                                                                                                                                                          |
| 게이트웨이 부팅이 Postgres TLS 인증서 확인 오류로 종료됩니다.                                                                                                   | 연결 문자열이 `sslmode=verify-full`을 설정하지만 이미지가 RDS CA 번들을 신뢰하지 않습니다: 번들이 이미지에 복사되지 않았거나 `NODE_EXTRA_CA_CERTS`가 이를 가리키지 않습니다.                                                                                                                                                                                                                  | 번들을 복사하고 `NODE_EXTRA_CA_CERTS`를 설정하는 빌드 단계의 2개 Dockerfile 줄을 추가하고, 다시 빌드하고, 새 태그 아래에서 푸시하고, 다시 배포하십시오.                                                                                                                               |
| 스트리밍 응답이 조용한 기간 후 중간에 떨어집니다.                                                                                                               | v2.1.229보다 오래된 게이트웨이가 Bedrock 또는 AWS 업스트림의 Claude Platform에서 업스트림이 조용할 때 아무것도 보내지 않습니다. 예를 들어 스트리밍된 출력이 없는 확장 사고 중입니다. ALB는 기본적으로 60초 동안 데이터가 없는 연결을 닫으므로 그 간격에서 스트림을 자릅니다. v2.1.229 이상의 게이트웨이는 조용한 스트림을 해당 시간 초과 아래에서 유지합니다: 이러한 업스트림에서 게이트웨이는 스트림 데이터가 없는 약 15초가 지나면 SSE `ping` 이벤트를 한 번 내보내고, Anthropic API 업스트림에서는 API 자체의 핑을 중계합니다. | 게이트웨이를 v2.1.229 이상으로 업데이트하거나, `idle_timeout.timeout_seconds` 속성을 `3600`으로 설정하십시오. `modify-load-balancer-attributes`를 통해 또는 EKS의 `load-balancer-attributes` Ingress 주석을 통해 설정하십시오.                                                    |

<h2 id="telemetry">
  원격 측정
</h2>

게이트웨이는 머신별 OTEL 구성 없이 개발자별 사용 메트릭을 제공합니다. Claude Code는 OpenTelemetry(OTLP) 메트릭, 로그 및 옵트인 추적을 내보냅니다. [사용 모니터링](/docs/ko/monitoring-usage)은 CLI가 보고하는 모든 것을 다룹니다. 게이트웨이 세션에서 CLI는 각 내보내기에 인증된 IdP ID 속성 `user.id`, `user.email`, `user.groups`를 스탬프하므로 사용이 `OTEL_RESOURCE_ATTRIBUTES` 배관 없이 개발자별로 롤업됩니다.

게이트웨이 자체는 인증된 OTLP 릴레이입니다. [`telemetry.forward_to`](/docs/ko/claude-apps-gateway-config#telemetry)를 `listen.public_url`과 함께 설정하면 모든 연결된 클라이언트에 OTEL 내보내기 설정을 푸시하고 OTLP 트래픽을 나열하는 각 대상으로 그대로 전달합니다. 각 대상은 메트릭, 로그, 추적을 독립적으로 옵트인하며, 기본값은 메트릭만입니다. 신호별 필드와 민감도 트레이드오프는 [`telemetry` 참조](/docs/ko/claude-apps-gateway-config#telemetry)를 참조하십시오. 게이트웨이는 원격 측정을 버퍼링, 집계 또는 저장하지 않으므로 데이터가 도달하는 위치는 전적으로 수집기의 내보내기 구성입니다.

클라이언트 원격 측정은 기본적으로 꺼져 있습니다. `telemetry.forward_to`를 구성하는 것이 연결된 개발자에 대해 이를 켜는 것이며, 각 대화형 클라이언트는 [구성 참조](/docs/ko/claude-apps-gateway-config#telemetry)에 설명된 대로 푸시된 설정에 대한 보안 승인 대화를 표시합니다. AWS에서 각 신호는 다음과 같이 대상으로 매핑됩니다.

<h3 id="client-metrics-logs-and-traces">
  클라이언트 메트릭, 로그 및 추적
</h3>

`telemetry.forward_to`를 [AWS Distro for OpenTelemetry(ADOT) 수집기](https://aws-otel.github.io/)와 같은 OpenTelemetry 수집기로 가리키고 거기서 Amazon CloudWatch, Amazon Managed Service for Prometheus 또는 모든 OTLP 백엔드로 내보내십시오.

수집기를 `https://`를 통해 도달할 수 있는 자체 내부 서비스로 실행하십시오. [`telemetry` 참조](/docs/ko/claude-apps-gateway-config#telemetry)는 루프백 예외 및 `CLAUDE_GATEWAY_ALLOW_LOOPBACK`을 다룹니다.

<h3 id="gateway-logs">
  게이트웨이 로그
</h3>

ECS Fargate에서 추가 설정이 없습니다: `awslogs` 드라이버는 게이트웨이의 stderr를 전달하며, 이는 감사 이벤트 및 운영 로그를 전달하고 위에서 생성된 `/ecs/claude-gateway` 로그 그룹으로 전달합니다. EKS에서 포드 로그는 기본적으로 CloudWatch에 도달하지 않으므로 감사 추적은 로그 수집을 설치할 때까지 손실됩니다: 컨테이너 로그 캡처가 활성화된 Amazon CloudWatch Observability 추가 기능 또는 Fluent Bit DaemonSet. 두 트랙 모두에서 CloudWatch Logs Insights로 로그를 쿼리하고 메트릭 필터에서 알람을 구동하십시오.

<h3 id="container-metrics">
  컨테이너 메트릭
</h3>

클러스터에서 Container Insights를 활성화하십시오. `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled`로 작업별 CPU, 메모리 및 네트워크를 활성화하십시오. EKS에서 Amazon CloudWatch Observability 추가 기능을 설치하십시오.

<h3 id="spend">
  지출
</h3>

원격 측정은 사용을 사후에 표시합니다. [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)은 게이트웨이의 공유 업스트림 자격 증명 위의 라이브 개발자별 보기 및 적용입니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [구성 참조](/docs/ko/claude-apps-gateway-config): 모든 `gateway.yaml` 옵션, `managed.policies` 및 `telemetry` 포함
* [배포 및 운영](/docs/ko/claude-apps-gateway-deploy): IdP 설정, 상태 확인, JWT 비밀 회전, 업그레이드 및 보안 모델
* [Claude 앱 게이트웨이 개요](/docs/ko/claude-apps-gateway): 빠른 시작 및 개발자 연결
* [Claude 앱 게이트웨이용 AWS 샘플](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway): 다양한 고객 환경을 다루는 AWS 유지 배포 샘플
