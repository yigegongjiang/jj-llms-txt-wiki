> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Развёртывание Claude apps gateway на AWS

> Практический пример запуска Claude apps gateway на AWS: ECS Fargate или EKS, Amazon RDS для PostgreSQL, AWS Secrets Manager и аутентификация на основе IAM-роли к Amazon Bedrock.

<Note>
  На этой странице описан один из способов запуска Claude apps gateway на AWS. Конфигурация представляет собой рабочий пример для инфраструктуры, управляемой клиентом, а не поддерживаемое развёртывание в production; используйте её, чтобы понять, как компоненты работают вместе, прежде чем адаптировать её к своей среде. Требования, независимые от платформы, см. в [руководстве по развёртыванию](/docs/ru/claude-apps-gateway-deploy).
</Note>

Этот пример развёртывает Claude apps gateway на AWS с Amazon Bedrock в качестве upstream модели, используя либо [Amazon ECS](https://aws.amazon.com/ecs/) на [AWS Fargate](https://aws.amazon.com/fargate/), либо [Amazon EKS](https://aws.amazon.com/eks/) для вычислений. [Okta](https://www.okta.com/) — это пример поставщика идентификации (IdP), но подходит любой поставщик, совместимый с OpenID Connect (OIDC); см. [Настройка поставщика идентификации](/docs/ru/claude-apps-gateway-deploy#identity-provider-setup) для получения деталей для каждого IdP.

<Note>
  Bedrock — не единственный Claude upstream на AWS. Gateway также поддерживает Claude Platform on AWS, управляемый Anthropic Claude API с аутентификацией AWS и выставлением счётов через AWS Marketplace, вместо Bedrock или наряду с ним. Его upstream запись, учётные данные и разрешения IAM отличаются от ориентированных на Bedrock на этой странице; [справочник upstream Claude Platform on AWS](/docs/ru/claude-apps-gateway-config#claude-platform-on-aws) охватывает, что изменяется, а остальная часть этой страницы применяется без изменений.
</Note>

<h2 id="architecture">
  Архитектура
</h2>

<Frame caption="Пример архитектуры с Amazon Bedrock в качестве upstream модели. Upstream Claude Platform on AWS занимает ту же позицию.">
  <img src="https://mintcdn.com/claude-code/PHweeRmDUYEKff49/images/claude-gateway-aws-architecture.svg?fit=max&auto=format&n=PHweeRmDUYEKff49&q=85&s=8599cc34aa28522cde208ee831439bb4" alt="Диаграмма Claude apps gateway на AWS: клиенты Claude Code подключаются по HTTPS к внутреннему Application Load Balancer, который находится перед gateway (ECS Fargate или EKS), работающим в приватных подсетях рядом с экземпляром Amazon RDS для PostgreSQL для состояния сессии. Gateway выполняет вход пользователей через OIDC по корпоративному IdP, читает секреты из AWS Secrets Manager, перенаправляет запросы модели в Amazon Bedrock, используя свою IAM-роль, и извлекает свой образ из Amazon ECR при развёртывании." width="820" height="430" data-path="images/claude-gateway-aws-architecture.svg" />
</Frame>

Gateway работает как приватная HTTPS конечная точка в вашей сети, на которую разработчики входят через ваш IdP. Их сессии Claude Code достигают моделей Claude на Amazon Bedrock через IAM-роль gateway, поэтому учётные данные модели не попадают на машины разработчиков. Эталонная конфигурация предусматривает:

* Сервис **Amazon ECS на AWS Fargate** или **Amazon EKS** Deployment, запускающий контейнер gateway
* Репозиторий **Amazon ECR** для образа gateway
* Экземпляр **Amazon RDS для PostgreSQL** в приватных подсетях, не доступный публично, для [хранилища](/docs/ru/claude-apps-gateway-config#store) gateway
* Секреты **AWS Secrets Manager** для ключа подписи JWT, секрета клиента OIDC и URL Postgres
* **IAM-роль** с `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream` и `bedrock:CountTokens`, присоединённая как роль задачи ECS или привязанная через IAM Roles for Service Accounts (IRSA) на EKS
* **Внутренний Application Load Balancer** для HTTPS

<h2 id="prerequisites">
  Предварительные требования
</h2>

Пошаговое руководство создаёт собственные ресурсы gateway, но строится на сетевой и инфраструктуре идентификации, которая у вас уже есть. Прежде чем начать, вам нужно:

* Учётная запись AWS с разрешением на создание [ресурсов выше](#architecture)
* [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) установлен и [аутентифицирован](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-authentication.html), а также [Docker](https://docs.docker.com/get-started/get-docker/) установлен локально
* [VPC](https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html) с минимум двумя [приватными подсетями](https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html) в разных Availability Zones с исходящим доступом в интернет через [NAT gateway](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html); внутреннему load balancer нужны подсети в двух AZ, а gateway нужен исходящий доступ к Bedrock и вашему IdP
* Веб-приложение Okta OIDC с URI перенаправления `https://<gateway-host>/oauth/callback`; см. [Настройка поставщика идентификации](/docs/ru/claude-apps-gateway-deploy#identity-provider-setup)
* TLS имя хоста для gateway, обычно внутреннее имя DNS в [приватной зоне Route 53](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html), указывающее на load balancer, с [сертификатом ACM](https://docs.aws.amazon.com/acm/latest/userguide/gs.html) для этого имени, импортированным или выданным [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html)

<h3 id="set-your-environment-variables">
  Установите переменные окружения
</h3>

Каждая команда на этой странице читает четыре значения из вашей оболочки: `AWS_REGION`, `ACCOUNT_ID`, `VPC_ID` и `PRIVATE_SUBNETS`.

Выберите регион США, где Bedrock обслуживает нужные вам модели Claude. Пошаговое руководство полагается на встроенный каталог моделей gateway, который разрешается в профили вывода `us.anthropic.*`, и политика IAM предоставляет эти ARN. В регионе, отличном от США, добавьте [`models:` блок](/docs/ru/claude-apps-gateway-config#models) с ID профилей вывода этого региона и измените префикс ARN в политике IAM, чтобы он совпадал.

Если у вас нет ID VPC под рукой, перечислите ваши VPC с помощью `aws ec2 describe-vpcs`, затем перечислите подсети этого VPC, чтобы найти две приватные в разных Availability Zones:

```bash theme={null}
aws ec2 describe-subnets --filters "Name=vpc-id,Values=<your-vpc-id>" \
  --query 'Subnets[].{ID:SubnetId,AZ:AvailabilityZone,CIDR:CidrBlock}' --output table
```

Экспортируйте все четыре перед продолжением:

```bash theme={null}
export AWS_REGION=us-east-1   # регион США, где Bedrock обслуживает нужные вам модели Claude
export ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
export VPC_ID=<your-vpc-id>
export PRIVATE_SUBNETS="<subnet-id-a> <subnet-id-b>"
```

<h2 id="deploy-the-gateway">
  Развёртывание gateway
</h2>

Шаги ниже предусматривают полное развёртывание с командами `aws`.

<Steps>
  <Step title="Создайте группы безопасности">
    Три группы безопасности связывают путь трафика: ваша корпоративная сеть достигает load balancer на 443, load balancer достигает gateway на 8080, и gateway достигает Postgres на 5432. Ничего больше не доступно. Как вы их присоединяете, зависит от трека вычислений:

    * На ECS Fargate шаг развёртывания присоединяет `$ALB_SG` к load balancer и `$GW_SG` к сервису.
    * На EKS AWS Load Balancer Controller создаёт свою собственную группу безопасности фронтенда для ALB, поэтому `$ALB_SG` и `$GW_SG` не используются: аннотация `inbound-cidrs` шага развёртывания ограничивает слушатель вашей корпоративной сетью, а группа безопасности базы данных допускает группу безопасности кластера вместо `$GW_SG`.

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

  <Step title="Создайте IAM-роли и отправьте форму использования">
    Gateway работает с выделённой ролью задачи, единственное разрешение которой — вызывать модели Claude на Bedrock. Согласно [справочнику upstream Bedrock](/docs/ru/claude-apps-gateway-config#amazon-bedrock), политика должна охватывать как ARN профилей вывода между регионами, так и базовые ARN моделей фундамента:

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

    ECS также нуждается в роли выполнения, которую сам агент ECS использует для извлечения образа из ECR и внедрения значений Secrets Manager, созданных позже. Она отделена от роли задачи, которую AWS SDK gateway использует во время выполнения:

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

    Политика называет один ARN на секрет, а не простой подстановочный знак `gateway-*`, который в общей учётной записи также совпадал бы с несвязанными секретами; конечный `-??????` совпадает ровно с случайным суффиксом из шести символов, который Secrets Manager добавляет к ARN каждого секрета. Конечный `-*` был бы простым глобусом префикса и также совпадал бы с более длинными именами, такими как `gateway-postgres-url-prod`.

    Политика IAM предоставляет gateway разрешение на вызов Bedrock, и Bedrock включает доступ к модели по умолчанию в коммерческих регионах. Оставшиеся ворота на уровне учётной записи — это одноразовая форма использования Anthropic: если никто в вашей учётной записи её не отправил, откройте [консоль Amazon Bedrock](https://console.aws.amazon.com/bedrock/), выберите модель Anthropic из каталога моделей и заполните форму. Доступ предоставляется сразу после отправки; см. [Claude Code на Amazon Bedrock](/docs/ru/amazon-bedrock#1-submit-use-case-details) для формы AWS Organizations и разрешений IAM, которые нужны отправителю.

    Трек EKS повторно использует оба документа политики на роли IRSA вместо двух ролей ECS; см. шаг развёртывания.
  </Step>

  <Step title="Подготовьте Amazon RDS для PostgreSQL">
    Экземпляр работает в приватных подсетях без публичного адреса и с включённым шифрованием хранилища. Версия движка закреплена на Postgres 16, что удовлетворяет поддерживаемому минимуму gateway PostgreSQL 14 и гарантирует, что семейство группы параметров ниже совпадает с экземпляром.

    Сначала создайте группу подсетей, которая размещает базу данных в приватных подсетях, и группу параметров с `rds.force_ssl=1`, чтобы сервер отклонял открытые соединения. Версия движка закреплена один раз, потому что семейство группы параметров должно совпадать с основной версией движка, которую запускает экземпляр:

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

    Затем создайте экземпляр с сгенерированным главным паролем:

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

    Буквальный аргумент `--master-user-password` виден в таблице процессов и в журналах аудита/EDR во время выполнения команды, то же самое воздействие, которое охватывает примечание шага секретов. На общем или контролируемом хосте передайте пароль через `--cli-input-json` из файла `0600` вместо этого, так же как это делает `setup.sh` пакета.

    Дождитесь, пока экземпляр запустится, что может занять несколько минут, затем прочитайте его приватную конечную точку и соберите строку подключения, которую будет использовать gateway:

    ```bash theme={null}
    aws rds wait db-instance-available --db-instance-identifier claude-gateway-db
    DB_HOST="$(aws rds describe-db-instances --db-instance-identifier claude-gateway-db \
      --query 'DBInstances[0].Endpoint.Address' --output text)"
    GATEWAY_POSTGRES_URL="postgres://gateway:${PGPASS}@${DB_HOST}:5432/claude_gateway?sslmode=verify-full"
    ```

    `sslmode=verify-full` заставляет gateway проверять цепь сертификата сервера RDS и имя хоста, а не только шифровать. Якорь доверия — это [пакет сертификатов AWS RDS](https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem), который шаг сборки образа ниже копирует в `/etc/claude/rds-global-bundle.pem` и доверяет через `NODE_EXTRA_CA_CERTS`. Не добавляйте параметр `sslrootcert=` в стиле libpq к URL: драйвер gateway читает только `sslmode` из строки запроса и передал бы `sslrootcert` Postgres как параметр запуска, который сервер отклоняет.

    Сервис ECS или поды EKS должны работать в этом VPC, чтобы они могли достичь приватной конечной точки экземпляра, и группа безопасности `claude-gateway-db` допускает только группу безопасности gateway.
  </Step>

  <Step title="Напишите gateway.yaml">
    Блок `upstreams` указывает на Bedrock с `auth: {}`, поэтому gateway аутентифицируется через цепь учётных данных AWS по умолчанию из роли задачи на ECS или роли IRSA на EKS. См. [справочник конфигурации](/docs/ru/claude-apps-gateway-config) для каждого поля.

    Два поля `listen` описывают, что находится перед gateway:

    * `public_url`: внешний источник `https://`, требуется для любого привязывания, отличного от loopback; см. [справочник `listen`](/docs/ru/claude-apps-gateway-config#listen). Gateway строит `redirect_uri` IdP и его документ обнаружения только из этого значения, никогда из заголовков `X-Forwarded-*`.
    * `trusted_proxies`: диапазоны источников фронтенда. Gateway соблюдает `X-Forwarded-For` только когда TCP-пир находится в этом списке, затем проходит цепь мимо доверенных переходов, поэтому ограничения скорости входа на IP и события аудита записывают IP разработчиков вместо load balancer.

    На обоих треках фронтенд — это внутренний ALB, создан ли он напрямую или AWS Load Balancer Controller, и узлы ALB берут адреса из подсетей, к которым он присоединён, поэтому установите `trusted_proxies` на CIDR этих подсетей. Это доверяет каждому хосту в этих подсетях как прокси. Не допускайте, чтобы источник входа ALB, ваша корпоративная CIDR, перекрывался с ними, и не делитесь подсетями с ненадёжными рабочими нагрузками, которые могли бы подделать IP клиентов через `X-Forwarded-For`.

    Атрибут сохранения клиентского порта ALB, `routing.http.xff_client_port.enabled`, может остаться в любом параметре: с ним включённым, ALB записывает клиента как `203.0.113.7:54321` или `[2001:db8::1]:54321`, и gateway читает оба с опущенным портом.

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
      # Сервер авторизации организации Okta возвращает тонкий id_token, который опускает
      # email и groups; gateway заполняет их из /userinfo.
      userinfo_fallback: true
      # Okta выдаёт groups только когда запрашивается область `groups` и
      # фильтр утверждения groups приложения их позволяет.
      scopes: [openid, profile, email, offline_access, groups]

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}              # EKS: ${file:/secrets/jwt-secret}
      ttl_hours: 8 # ограничивает задержку отзыва; снизьте
    # к 1 для более жёсткого отзыва

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}          # EKS: ${file:/secrets/postgres-url}

    upstreams:
      - provider: bedrock
        region: <your-region>                        # совпадайте с $AWS_REGION, чтобы ARN политики IAM
    # охватывали его
        auth: {} # цепь учётных данных AWS по умолчанию:
    # роль задачи ECS или IRSA на EKS
    ```

    <Note>
      Только блок `oidc` специфичен для Okta. Чтобы использовать Microsoft Entra ID вместо этого, установите `issuer` на `https://login.microsoftonline.com/<tenant-id>/v2.0`, удалите `userinfo_fallback` и область `groups`, и обратите внимание, что Entra выдаёт Object ID группы, а не имена, поэтому [`managed.policies`](/docs/ru/claude-apps-gateway-config#managed) должны совпадать на GUID, или на App Roles с `oidc.groups_claim: roles`. См. [Настройка поставщика идентификации](/docs/ru/claude-apps-gateway-deploy#identity-provider-setup).
    </Note>
  </Step>

  <Step title="Сохраните секреты в AWS Secrets Manager">
    Создайте три секрета; роль выполнения из шага IAM уже может их читать:

    ```bash theme={null}
    aws secretsmanager create-secret --name gateway-jwt-secret \
      --secret-string "$(openssl rand -base64 32)"
    aws secretsmanager create-secret --name gateway-oidc-client-secret \
      --secret-string '<your-okta-client-secret>'
    aws secretsmanager create-secret --name gateway-postgres-url \
      --secret-string "$GATEWAY_POSTGRES_URL"
    ```

    Обратите внимание на ARN, который печатает каждый вызов; определение задачи ECS ссылается на секреты по ARN.

    <Note>
      Буквальные аргументы `--secret-string` видны в таблице процессов и в журналах аудита/EDR во время выполнения каждой команды. На общем или контролируемом хосте поместите значение в файл `0600` и передайте `--secret-string file://<path>` вместо этого. `setup.sh` пакета держит значения секретов вне argv процесса так же, передавая временные файлы `0600` в `--cli-input-json`.
    </Note>

    В отличие от секретов, сам `gateway.yaml` не содержит значений секретов, потому что каждое учётное данные разрешается при загрузке через [`${VAR}` или `${file:...}` расширение](/docs/ru/claude-apps-gateway-config#secret-expansion). Как всё достигает контейнера, отличается по треку:

    * На ECS, сборка следующего шага копирует `gateway.yaml` в образ в `/etc/claude/gateway.yaml`, и определение задачи внедряет три секрета как переменные окружения через его поле `secrets`, поэтому YAML ссылается на `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` и `${GATEWAY_POSTGRES_URL}`.
    * На EKS, смонтируйте `gateway.yaml` из ConfigMap и секреты как файлы в `/secrets`, на которые ссылаются как `${file:/secrets/...}`. Получите Kubernetes Secrets из Secrets Manager с External Secrets Operator или поставщиком AWS драйвера Secrets Store CSI, или создайте их напрямую с помощью `kubectl`.
  </Step>

  <Step title="Соберите и отправьте образ в Amazon ECR">
    Соберите образ согласно [требованиям образа контейнера](/docs/ru/claude-apps-gateway-deploy#container-image), разместив двоичный файл `linux-x64` glibc в `./claude` в контексте сборки. Напишите свой собственный Dockerfile согласно этим требованиям или начните с [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/examples/gateway/aws/Dockerfile) пакета, который копирует заполненный `gateway.yaml` из предыдущих шагов в образ в `/etc/claude/gateway.yaml`. На ECS эта встроенная копия — это то, как конфигурация достигает контейнера, поэтому сборка идёт после написания файла. Трек EKS вместо этого монтирует `gateway.yaml` из ConfigMap при развёртывании, поэтому встроенная копия там не используется.

    Образ также содержит пакет сертификатов AWS RDS как якорь доверия для `sslmode=verify-full` строки подключения, поэтому загрузите его в контекст сборки сначала. AWS ротирует пакет (новые региональные CA добавляются), поэтому загружайте его при каждой сборке, а не закрепляйте контрольную сумму или фиксируйте её:

    ```bash theme={null}
    curl -fL --proto '=https' -o rds-global-bundle.pem \
      https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
    ```

    Требования образа контейнера не охватывают пакет, поэтому если вы напишете свой собственный Dockerfile, добавьте две строки, которые копируют и доверяют ему; `Dockerfile` пакета уже включает оба:

    ```dockerfile theme={null}
    COPY rds-global-bundle.pem /etc/claude/rds-global-bundle.pem
    ENV NODE_EXTRA_CA_CERTS=/etc/claude/rds-global-bundle.pem
    ```

    Создайте репозиторий ECR и подпишите Docker в него. Неизменяемые теги означают, что тег `<version>`, который закрепляет шаг развёртывания, не может позже молча переуказываться на другой образ:

    ```bash theme={null}
    aws ecr create-repository --repository-name claude-gateway \
      --image-tag-mutability IMMUTABLE \
      --image-scanning-configuration scanOnPush=true
    aws ecr get-login-password --region "$AWS_REGION" \
      | docker login --username AWS --password-stdin \
        "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
    ```

    Соберите и отправьте образ. Определение задачи ниже запускает `linux/amd64`, поэтому платформа должна совпадать здесь; для Fargate на ARM64 (Graviton), соберите `linux/arm64` с двоичным файлом `linux-arm64` и установите `cpuArchitecture` на `ARM64` вместо этого:

    ```bash theme={null}
    docker build --platform=linux/amd64 \
      -t "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>" .
    docker push "${ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/claude-gateway:<version>"
    ```
  </Step>

  <Step title="Развёртывание">
    <Tabs>
      <Tab title="ECS Fargate">
        Создайте кластер и группу журналов для stderr gateway, которая содержит как события аудита, так и операционные журналы. Удержание — это отдельный вызов, и без него CloudWatch хранит журналы вечно; выровняйте 90 дней с вашей политикой удержания аудита:

        ```bash theme={null}
        aws ecs create-cluster --cluster-name claude-gateway
        aws logs create-log-group --log-group-name /ecs/claude-gateway
        aws logs put-retention-policy --log-group-name /ecs/claude-gateway \
          --retention-in-days 90
        ```

        Напишите определение задачи. Роль задачи несёт разрешение Bedrock, а роль выполнения внедряет секреты; используйте ARN секретов из шага Secrets Manager:

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

        Зарегистрируйте его:

        ```bash theme={null}
        aws ecs register-task-definition --cli-input-json file://claude-gateway-task.json
        ```

        Поместите внутренний ALB впереди с целевой группой, которая проверяет здоровье gateway. `--ip-address-type ipv4` имеет значение: внутренний двойной стек ALB публикует записи AAAA общественного диапазона, которые проверка приватной сети `/login` отклоняет:

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

        Добавьте слушатель HTTPS. `--ssl-policy` закрепляет современный минимум TLS, так как его опущение возвращается к устаревшему значению по умолчанию `ELBSecurityPolicy-2016-08`, которое всё ещё принимает TLS 1.0/1.1.

        ALB закрывает соединение после 60 секунд без данных по умолчанию. Keepalive пинги gateway держат потоки внутри этого значения по умолчанию, поэтому повышение времени ожидания добавляет запас выше кадра пинга; строка [Troubleshooting](#troubleshooting) на разорванных потоках охватывает механизм и более старые gateway. Команды ниже добавляют слушатель и повышают время ожидания:

        ```bash theme={null}
        aws elbv2 create-listener --load-balancer-arn "$ALB_ARN" \
          --protocol HTTPS --port 443 \
          --ssl-policy ELBSecurityPolicy-TLS13-1-2-2021-06 \
          --certificates CertificateArn=<your-acm-certificate-arn> \
          --default-actions Type=forward,TargetGroupArn="$TG_ARN"

        aws elbv2 modify-load-balancer-attributes --load-balancer-arn "$ALB_ARN" \
          --attributes Key=idle_timeout.timeout_seconds,Value=3600
        ```

        Создайте сервис. Выключатель развёртывания откатывает развёртывание, чьи задачи продолжают отказывать, из-за плохого образа или неустойчивой конфигурации, обратно к последнему стабильному состоянию вместо перезапуска отказывающих задач вечно:

        ```bash theme={null}
        aws ecs create-service --cluster claude-gateway --service-name claude-gateway \
          --task-definition claude-gateway --desired-count 1 --launch-type FARGATE \
          --deployment-configuration "deploymentCircuitBreaker={enable=true,rollback=true}" \
          --health-check-grace-period-seconds 60 \
          --network-configuration "awsvpcConfiguration={subnets=[$(echo $PRIVATE_SUBNETS | tr ' ' ',')],securityGroups=[$GW_SG],assignPublicIp=DISABLED}" \
          --load-balancers "targetGroupArn=$TG_ARN,containerName=gateway,containerPort=8080"
        ```

        Период благодати в 60 секунд даёт холодной задаче время на извлечение образа, подключение к хранилищу и ответ на первую проверку здоровья перед тем, как ECS начнёт считать отказы против развёртывания. Проверка здоровья целевой группы на `GET /readyz` проверяет, что хранилище доступно, поэтому задача, которая не может достичь Postgres, никогда не входит в ротацию; см. [Поведение при сбое](/docs/ru/claude-apps-gateway-deploy#outage-behavior) для компромисса и альтернативы `/healthz`.

        Задачи работают в приватных подсетях без публичного IP, поэтому весь исходящий трафик (в Bedrock, ваш IdP, Secrets Manager, ECR и CloudWatch Logs) проходит через NAT gateway. Чтобы держать трафик Bedrock вне публичного пути, создайте интерфейсную конечную точку VPC `bedrock-runtime` и укажите `base_url` upstream на неё, как показано в [справочнике upstream Bedrock](/docs/ru/claude-apps-gateway-config#amazon-bedrock); IdP всё ещё нуждается в исходящем доступе в интернет.

        Завершите, дав разработчикам приватно разрешаемое имя хоста: в приватной зоне Route 53 создайте псевдоним внутреннего имени DNS gateway на ALB и установите `listen.public_url` на это имя хоста. Собственное имя `*.elb.amazonaws.com` ALB разрешается на приватные адреса на внутреннем ALB, но оно не может нести ваш сертификат ACM, поэтому используйте своё имя.

        Обновите URI перенаправления авторизованного клиента OAuth на `<public_url>/oauth/callback` перед первым входом. После изменения `public_url`, пересоберите и отправьте образ под новым тегом, зарегистрируйте новую редакцию определения задачи и переразвёртывайте. На ECS параметр живёт в встроенном `gateway.yaml` образа, и gateway строит свой публичный источник только из этого параметра, игнорируя `X-Forwarded-Host` и `X-Forwarded-Proto`. `X-Forwarded-For` соблюдается для IP клиентов только когда установлен `listen.trusted_proxies`.
      </Tab>

      <Tab title="EKS">
        Этот трек нуждается в установленных `kubectl` и `eksctl` локально, и существующем кластере EKS с поставщиком IAM OIDC и установленным AWS Load Balancer Controller. Кластер должен быть на `$VPC_ID`, чтобы поды могли достичь приватной конечной точки RDS, и группа безопасности `claude-gateway-db` должна допускать группу безопасности пода или узла кластера вместо `$GW_SG`.

        На EKS gateway получает свои учётные данные Bedrock через IRSA, а не роли ECS. Политика доверия `ecs-tasks.amazonaws.com` из шага IAM не применяется здесь; IRSA нуждается в роли, чья политика доверия федерирует на поставщика OIDC кластера, ограниченном `system:serviceaccount:claude-gateway:gateway`. `eksctl create iamserviceaccount` создаёт эту роль, присоединяет политики и аннотирует учётную запись сервиса Kubernetes с ARN роли в один шаг. Превратите два документа политики из шага IAM в управляемые политики, которые он может присоединить:

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

        Политика секретов нужна только когда поды читают Secrets Manager сами, как поставщик AWS драйвера Secrets Store CSI делает, используя учётную запись сервиса монтирующего пода; удалите её, если вы создаёте Kubernetes Secrets другим способом. Поставщик нуждается в обоих действиях политики: он вызывает `DescribeSecret`, когда он согласует ротированные секреты, поэтому грант только `GetSecretValue` монтирует при первом развёртывании, но перестаёт подбирать ротации.

        Развёртывайте gateway как стандартный Deployment плюс Service и Ingress, как описано в [развёртывании Kubernetes](/docs/ru/claude-apps-gateway-deploy#kubernetes), с:

        * `serviceAccountName: gateway`
        * `gateway.yaml` смонтирован из ConfigMap и секреты смонтированы в `/secrets`
        * зонд готовности указан на `GET /readyz`

        Для фронтенда, Ingress, управляемый AWS Load Balancer Controller, предусматривает внутренний ALB. Аннотируйте его с:

        * `alb.ingress.kubernetes.io/scheme: internal` и `alb.ingress.kubernetes.io/target-type: ip`
        * `alb.ingress.kubernetes.io/ip-address-type: ipv4`, поэтому записи AAAA общественного диапазона не публикуются для проверки приватной сети `/login` [private-network check](/docs/ru/claude-apps-gateway#prerequisites) отклонить
        * `alb.ingress.kubernetes.io/inbound-cidrs: <your-corporate-cidr>`, поэтому контроллер-управляемая группа безопасности фронтенда допускает только вашу корпоративную сеть вместо значения по умолчанию `0.0.0.0/0`
        * `alb.ingress.kubernetes.io/certificate-arn` с сертификатом ACM
        * `alb.ingress.kubernetes.io/ssl-policy: ELBSecurityPolicy-TLS13-1-2-2021-06`, поэтому слушатель не возвращается к устаревшей политике по умолчанию, которая принимает TLS 1.0 и 1.1
        * `alb.ingress.kubernetes.io/load-balancer-attributes: idle_timeout.timeout_seconds=3600`, запас выше keepalive потоков gateway; см. [Troubleshooting](#troubleshooting)

        С IRSA, AWS SDK читает спроецированный токен учётной записи сервиса и обменивает его с AWS STS, поэтому под никогда не нуждается в сервисе метаданных экземпляра EC2; NetworkPolicy исходящего трафика может блокировать `169.254.169.254` для подов gateway. Проблема с лимитом переходов узла в [Troubleshooting](#troubleshooting) ниже применяется только к кластерам, которые пропускают IRSA и полагаются на роли экземпляра узла.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Отправьте URL gateway на машины разработчиков">
    Gateway теперь работает, но разработчики не могут достичь его из `/login` до тех пор, пока URL gateway не будет на их машинах. Установите `forceLoginMethod` и `forceLoginGatewayUrl` в [файле управляемых параметров](/docs/ru/claude-apps-gateway#set-the-gateway-url), который вы развёртываете на каждом устройстве через MDM. Нет опции gateway в средстве выбора входа для разработчика, чтобы выбрать вручную.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Справочник Terraform
</h2>

Сопутствующий пакет в [`examples/gateway/aws`](https://github.com/anthropics/claude-code/tree/main/examples/gateway/aws) упаковывает эту страницу как код:

* **`setup.sh`** скриптирует пошаговое руководство по подготовке выше с теми же командами `aws`, на треке ECS Fargate. Это идемпотентно: существующие ресурсы обнаруживаются и пропускаются, поэтому повторный запуск безопасен, и любое значение по умолчанию может быть переопределено через переменную окружения. Вы всё ещё создаёте секрет клиента Okta OIDC и сертификат ACM сами: запуск без них пропускает развёртывание ECS/ALB, называет отсутствующие входы и печатает команду `create-secret`; создайте оба и переразвёртывайте. Форма использования Bedrock и псевдоним Route 53 печатаются как следующие шаги, а не запускаются автоматически, и push MDM клиента остаётся ручным шагом с этой страницы.
* **`gateway.yaml.example`** — это шаблон конфигурации из шага gateway.yaml, с дополнительными ключами, включёнными в комментарии. Скопируйте его в `gateway.yaml` и замените каждый `REPLACE_ME` перед сборкой.
* **`Dockerfile`** собирает образ среды выполнения из предварительно собранного двоичного файла `linux-x64` и копирует ваш заполненный `gateway.yaml` в `/etc/claude/gateway.yaml`, плюс пакет сертификатов AWS RDS, который якорирует `sslmode=verify-full` хранилища. `setup.sh` загружает пакет только когда его ещё нет в контексте сборки; удалите файл и пересоберите под новым тегом, чтобы подобрать ротацию AWS CA. Файл конфигурации не содержит значений секретов, так как каждое учётное данные разрешается при загрузке через расширение `${VAR}`. Редактирование конфигурации, следовательно, означает пересборку под новым тегом; `setup.sh` автоматизирует это, помечая образы хешем файла.
* **`terraform/`** предусматривает ту же область ECS Fargate декларативно: группы безопасности, роли IAM, репозиторий ECR, экземпляр RDS, секреты Secrets Manager и сервис ECS за внутренним ALB. VPC и приватные подсети остаются предварительными условиями, переданными как переменные. Terraform создаёт репозиторий ECR, но не собирает образ, и определение сервиса ссылается на образ, поэтому применение — это два прохода: целевое применение для репозитория, затем сборка и отправка, затем полное применение. `terraform/README.md` пакета охватывает переменные, удалённое состояние и разборку.

Как эта страница, пакет — это рабочий пример для инфраструктуры, управляемой клиентом, а не поддерживаемое развёртывание в production; просмотрите и адаптируйте его к своей среде перед тем, как полагаться на него.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Для ошибок загрузки gateway и входа см. независимую от платформы [таблицу troubleshooting](/docs/ru/claude-apps-gateway-deploy#troubleshooting). Записи ниже специфичны для AWS.

| Симптом                                                                                                                                    | Причина                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Исправление                                                                                                                                                                                                                                                                                                             |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI `/login`: `Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | Имя gateway разрешается по крайней мере на один публичный адрес. Двойной стек внутреннего ALB публикует записи AAAA общественного диапазона, и [проверка приватной сети](/docs/ru/claude-apps-gateway#prerequisites) требует, чтобы каждый разрешённый адрес был приватным                                                                                                                                                                                                                                                                          | Создайте ALB с `--ip-address-type ipv4` или обслуживайте отдельное внутреннее имя DNS без публичной записи AAAA                                                                                                                                                                                                         |
| Каждый запрос Bedrock возвращает 502; журнал показывает `Could not load credentials from any providers`                                    | Задача работает на типе запуска ECS EC2 без роли задачи, или под работает на узле EKS без IRSA, поэтому учётные данные поступают из метаданных экземпляра, которые лимит переходов IMDSv2 по умолчанию 1 останавливает внутри контейнера. Ни один трек на этой странице не затронут: роли задачи Fargate и IRSA не используют метаданные экземпляра                                                                                                                                                                                            | Предпочитайте роли задачи и IRSA. Где учётные данные экземпляра неизбежны, поднимите лимит переходов с `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2`; [таблица независимая от платформы](/docs/ru/claude-apps-gateway-deploy#troubleshooting) охватывает компромиссы         |
| Запросы Bedrock возвращают `403 AccessDeniedException`                                                                                     | Учётная запись не отправила одноразовую форму использования Anthropic, автоматическая подписка AWS Marketplace, которая начинается при первом вызове учётной записи, ещё не завершена, или политика роли задачи отсутствует ARN профиля вывода или модели фундамента                                                                                                                                                                                                                                                                           | Отправьте форму использования из каталога моделей консоли Bedrock; если она была только что отправлена или это первый вызов учётной записи, повторите попытку через несколько минут. Предоставьте `bedrock:InvokeModel` и `bedrock:InvokeModelWithResponseStream` на обоих семействах ARN.                              |
| Bedrock возвращает `ValidationException`, говоря, что пропускная способность по требованию не поддерживается                               | Пользовательская запись `models:` отображается на голый ID модели фундамента, который регион обслуживает только через профили вывода                                                                                                                                                                                                                                                                                                                                                                                                           | Отобразите модель на её ID профиля вывода между регионами (`us.anthropic.*`) вместо этого; встроенный каталог уже это делает                                                                                                                                                                                            |
| Задача ECS останавливается с `ResourceInitializationError` перед тем, как gateway что-либо регистрирует                                    | Роль выполнения не может читать секреты Secrets Manager, или приватные подсети не имеют пути к Secrets Manager или ECR                                                                                                                                                                                                                                                                                                                                                                                                                         | Предоставьте `secretsmanager:GetSecretValue` на ARN трёх секретов `gateway-` роли выполнения и обеспечьте исходящий доступ через NAT gateway, или, без него, интерфейсные конечные точки для Secrets Manager, ECR и CloudWatch Logs, которые драйвер `awslogs` нуждается на той же стадии, плюс конечная точка шлюза S3 |
| Загрузка gateway выходит с ошибкой тайм-аута подключения Postgres                                                                          | Группа безопасности базы данных не допускает группу безопасности gateway на 5432, или сервис работает вне VPC базы данных                                                                                                                                                                                                                                                                                                                                                                                                                      | Разрешите 5432 от группы безопасности gateway на группе безопасности базы данных и запустите сервис в том же VPC, что и группа подсетей БД                                                                                                                                                                              |
| Загрузка gateway выходит с ошибкой проверки сертификата TLS Postgres                                                                       | Строка подключения устанавливает `sslmode=verify-full`, но образ не доверяет пакету RDS CA: пакет не был скопирован в образ, или `NODE_EXTRA_CA_CERTS` не указывает на него                                                                                                                                                                                                                                                                                                                                                                    | Добавьте две строки Dockerfile шага сборки, которые копируют пакет и устанавливают `NODE_EXTRA_CA_CERTS`, затем пересоберите, отправьте под новым тегом и переразвёртывайте                                                                                                                                             |
| Потоковые ответы прерываются в середине потока после тихого периода                                                                        | Gateway старше v2.1.229 на upstream Bedrock или Claude Platform on AWS не отправляет ничего, пока upstream молчит, например во время расширенного мышления без потокового вывода. ALB закрывает соединение после 60 секунд без данных по умолчанию, поэтому он отрезает поток в этом разрыве. Gateway v2.1.229 и позже держат молчащий поток под этим временем ожидания: на этих upstream gateway выдаёт событие SSE `ping` примерно один раз через 15 секунд без данных потока, и на upstream Anthropic API он передаёт собственные пинги API | Обновите gateway до v2.1.229 или позже, или установите атрибут `idle_timeout.timeout_seconds` на `3600`, через `modify-load-balancer-attributes` или аннотацию `load-balancer-attributes` Ingress на EKS                                                                                                                |

<h2 id="telemetry">
  Телеметрия
</h2>

Gateway даёт вам метрики использования на разработчика без какой-либо конфигурации OTEL на машину. Claude Code выдаёт метрики OpenTelemetry (OTLP), журналы и дополнительные трассировки; [Мониторинг использования](/docs/ru/monitoring-usage) охватывает всё, что сообщает CLI. На сессиях gateway CLI штампует каждый экспорт с аутентифицированными атрибутами идентификации IdP `user.id`, `user.email` и `user.groups`, поэтому использование накапливается на разработчика без сантехники `OTEL_RESOURCE_ATTRIBUTES`.

Gateway сам является аутентифицированным реле OTLP. Установите [`telemetry.forward_to`](/docs/ru/claude-apps-gateway-config#telemetry) вместе с `listen.public_url`, и он отправляет параметры экспортера OTEL каждому подключённому клиенту и перенаправляет их трафик OTLP дословно каждому пункту назначения, который вы указываете. Каждый пункт назначения независимо выбирает метрики, журналы и трассировки, и значение по умолчанию — только метрики; см. [справочник `telemetry`](/docs/ru/claude-apps-gateway-config#telemetry) для полей на сигнал и их компромиссы чувствительности. Gateway не буферизирует, не агрегирует и не хранит телеметрию, поэтому то, где данные приземляются, полностью конфигурация экспортера сборщика.

Телеметрия клиента отключена по умолчанию; конфигурирование `telemetry.forward_to` — это то, что включает её для подключённых разработчиков, и каждый интерактивный клиент показывает диалог одобрения безопасности один раз для отправленных параметров, как описано в [справочнике конфигурации](/docs/ru/claude-apps-gateway-config#telemetry). На AWS каждый сигнал отображается на пункт назначения следующим образом.

<h3 id="client-metrics-logs-and-traces">
  Метрики, журналы и трассировки клиента
</h3>

Укажите `telemetry.forward_to` на сборщик OpenTelemetry, такой как [AWS Distro for OpenTelemetry (ADOT) collector](https://aws-otel.github.io/), и экспортируйте оттуда в Amazon CloudWatch, Amazon Managed Service for Prometheus или любой backend OTLP.

Запустите сборщик как его собственный внутренний сервис, доступный через `https://`; [справочник `telemetry`](/docs/ru/claude-apps-gateway-config#telemetry) охватывает исключение loopback и `CLAUDE_GATEWAY_ALLOW_LOOPBACK`.

<h3 id="gateway-logs">
  Журналы Gateway
</h3>

На ECS Fargate, никакой дополнительной установки: драйвер `awslogs` доставляет stderr gateway, который содержит события аудита и операционные журналы, в группу журналов `/ecs/claude-gateway`, созданную выше. На EKS, журналы подов не достигают CloudWatch по умолчанию, поэтому аудит теряется до тех пор, пока вы не установите сбор журналов: надстройка Amazon CloudWatch Observability с включённым захватом журналов контейнера или DaemonSet Fluent Bit. На любом треке запросите журналы с помощью CloudWatch Logs Insights и управляйте сигналами тревоги из фильтров метрик.

<h3 id="container-metrics">
  Метрики контейнера
</h3>

Включите Container Insights на кластере с `aws ecs update-cluster-settings --cluster claude-gateway --settings name=containerInsights,value=enabled` для CPU, памяти и сети на задачу. На EKS установите надстройку Amazon CloudWatch Observability.

<h3 id="spend">
  Расходы
</h3>

Телеметрия показывает использование после факта; [ограничения расходов](/docs/ru/claude-apps-gateway-spend-limits) — это живой вид gateway на разработчика и принуждение поверх общего учётного данные upstream.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Справочник конфигурации](/docs/ru/claude-apps-gateway-config): каждая опция `gateway.yaml`, включая `managed.policies` и `telemetry`
* [Развёртывание и операции](/docs/ru/claude-apps-gateway-deploy): настройка IdP, проверки здоровья, ротация секрета JWT, обновления и модель безопасности
* [Обзор Claude apps gateway](/docs/ru/claude-apps-gateway): быстрый старт и подключение разработчиков
* [Примеры AWS для Claude apps gateway](https://github.com/aws-samples/anthropic-on-aws/tree/main/claude-apps-gateway): примеры развёртывания, поддерживаемые AWS, охватывающие диапазон сред клиентов
