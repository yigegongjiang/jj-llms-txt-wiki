> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Развертывание Claude apps gateway на Google Cloud

> Практический пример запуска Claude apps gateway на Google Cloud: Cloud Run или GKE, Cloud SQL для PostgreSQL, Secret Manager и аутентификация через сервисный аккаунт для Agent Platform Google Cloud.

<Note>
  На этой странице описан один из способов запуска Claude apps gateway на Google Cloud. Конфигурация является рабочим примером для инфраструктуры, управляемой клиентом, а не поддерживаемым развертыванием для производства; используйте её, чтобы понять, как компоненты работают вместе, прежде чем адаптировать её к своей среде. Требования, независимые от платформы, см. в [руководстве по развертыванию](/docs/ru/claude-apps-gateway-deploy).
</Note>

Этот пример развертывает Claude apps gateway на Google Cloud с использованием Agent Platform Google Cloud в качестве вышестоящего поставщика моделей, используя либо Cloud Run, либо GKE для вычислений. Google Workspace используется в качестве примера поставщика идентификации (IdP), но работает любой поставщик, совместимый с OpenID Connect (OIDC); изменяется только блок `oidc`. Подробности для каждого IdP см. в разделе [Настройка поставщика идентификации](/docs/ru/claude-apps-gateway-deploy#identity-provider-setup).

<h2 id="what-you’ll-build">
  Что вы создадите
</h2>

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/claude-gateway-gcp-architecture.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=cb705151c69128ac0da235852d5600ab" alt="Диаграмма Claude apps gateway на Google Cloud: клиенты Claude Code подключаются по HTTPS к шлюзу (Cloud Run или GKE), который работает внутри VPC рядом с приватной базой данных Cloud SQL для состояния сеанса. Шлюз выполняет вход пользователей через OIDC для Google Workspace, читает конфигурацию и секреты из Secret Manager, перенаправляет запросы моделей в Agent Platform и извлекает свой образ из Artifact Registry при развертывании." width="760" height="400" data-path="images/claude-gateway-gcp-architecture.svg" />
</Frame>

Эталонная конфигурация подготавливает:

* Сервис **Cloud Run** или развертывание **GKE**, запускающие контейнер шлюза
* Репозиторий **Artifact Registry** для образа шлюза
* Экземпляр **Cloud SQL для PostgreSQL**, только с приватным IP, для [хранилища](/docs/ru/claude-apps-gateway-config#store) шлюза
* Секреты **Secret Manager** для `gateway.yaml`, ключа подписи JWT, секрета клиента OIDC и URL Postgres
* **Сервисный аккаунт** с `roles/aiplatform.user`, подключённый непосредственно на Cloud Run или привязанный через Workload Identity на GKE
* **Внутренний Application Load Balancer** на Cloud Run или внутренний **GKE Ingress** класса `gce-internal` на GKE для HTTPS

<h2 id="prerequisites">
  Предварительные требования
</h2>

* Проект GCP с включённым биллингом и разрешением на создание указанных выше ресурсов
* CLI `gcloud`, аутентифицированный с помощью `gcloud auth login`, и Docker, установленный локально
* Для трека GKE: `kubectl` и кластер GKE на VPC, созданный в пошаговом руководстве ниже
* Доступ к необходимым моделям Claude в Model Garden в регионе, который их публикует
* Веб-приложение OAuth 2.0 Google Workspace с URI перенаправления `https://<gateway-host>/oauth/callback`; см. [Настройка поставщика идентификации](/docs/ru/claude-apps-gateway-deploy#identity-provider-setup)
* TLS имя хоста для шлюза, обычно внутреннее имя DNS, указывающее на балансировщик нагрузки

Установите проект и регион один раз:

```bash theme={null}
export PROJECT_ID=<your-project>
export REGION=us-east5   # a region where the Claude models you need are published in Model Garden
gcloud config set project "$PROJECT_ID"
```

<h2 id="deploy-the-gateway">
  Развертывание шлюза
</h2>

Приведённые ниже шаги подготавливают полное развертывание с помощью команд `gcloud`.

<Steps>
  <Step title="Включение API">
    Включите API сервисов, которые использует пошаговое руководство:

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

    Необходимые API зависят от пути развертывания:

    * `compute` и `servicenetworking`: требуются для пути приватного IP Cloud SQL
    * `run`: только Cloud Run
    * `container`: только GKE
  </Step>

  <Step title="Создание сервисного аккаунта и предоставление IAM">
    Шлюз работает как выделенный сервисный аккаунт с разрешением на вызов Agent Platform. Он достигает Cloud SQL через VPC с пользователем пароля, поэтому роль Cloud SQL IAM не требуется:

    ```bash theme={null}
    gcloud iam service-accounts create claude-gateway --display-name="Claude apps gateway"
    SA="claude-gateway@${PROJECT_ID}.iam.gserviceaccount.com"

    gcloud projects add-iam-policy-binding "$PROJECT_ID" \
      --member="serviceAccount:${SA}" --role="roles/aiplatform.user" --condition=None
    ```

    Затем включите модели Claude для проекта в Model Garden; модели публикуются в определённых регионах, поэтому проверьте каждую карточку модели.
  </Step>

  <Step title="Построение и отправка образа в Artifact Registry">
    Постройте образ в соответствии с [требованиями к образу контейнера](/docs/ru/claude-apps-gateway-deploy#container-image), используя двоичный файл `linux-x64` glibc, и отправьте его:

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

  <Step title="Подготовка Cloud SQL для PostgreSQL">
    Создайте экземпляр на VPC через Private Services Access, чтобы он не имел публичного IP; это также удовлетворяет проектам, где применяется `constraints/sql.restrictPublicIp`:

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

    Среда выполнения Cloud Run или GKE должна находиться на этом VPC или маршрутизироваться в него.
  </Step>

  <Step title="Написание gateway.yaml">
    Блок `upstreams` указывает на Agent Platform с `auth: {}`, поэтому шлюз аутентифицируется через Application Default Credentials из сервисного аккаунта среды выполнения. Полное описание каждого поля см. в [справочнике конфигурации](/docs/ru/claude-apps-gateway-config).

    Два поля `listen` зависят от того, что находится перед шлюзом:

    * `public_url`: требуется за Cloud Run или GKE Ingress. Шлюз строит `redirect_uri` IdP и документ обнаружения только из этого значения, никогда из заголовков `X-Forwarded-*`.
    * `trusted_proxies`: диапазоны источников переднего конца. Шлюз соблюдает `X-Forwarded-For` только когда TCP-пир находится в этом списке, затем проходит по цепи мимо доверенных переходов, поэтому ограничения скорости входа на IP и события аудита записывают IP-адреса разработчиков вместо адреса балансировщика нагрузки.

    Установите `trusted_proxies` в соответствии с вашим передним концом. Внешний GKE Ingress класса `gce` не указан: он подготавливает адрес публичного правила переадресации, который проверка [приватной сети](/docs/ru/claude-apps-gateway#prerequisites) `/login` отклоняет.

    | Передний конец                                               | `trusted_proxies`                                          |
    | ------------------------------------------------------------ | ---------------------------------------------------------- |
    | Cloud Run, достигнутый напрямую, без балансировщика нагрузки | `[169.254.0.0/16]`                                         |
    | Internal Application Load Balancer перед Cloud Run           | `169.254.0.0/16` плюс CIDR вашей подсети только для прокси |
    | GKE внутренний Ingress, класс `gce-internal`                 | CIDR вашей подсети только для прокси                       |

    Пример ниже использует значения внутреннего балансировщика нагрузки перед Cloud Run.

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
      Google id\_tokens не содержат претензию `groups`. Для использования политик на основе групп в [`managed.policies`](/docs/ru/claude-apps-gateway-config#managed) с Google Workspace в качестве IdP настройте [`oidc.google_groups`](/docs/ru/claude-apps-gateway-config#oidc), которая ищет группы каждого пользователя через Admin SDK Directory API, используя сервисный аккаунт с делегированием на уровне домена. Без этого вместо этого сопоставляйте по `email_domain`.
    </Note>
  </Step>

  <Step title="Сохранение секретов в Secret Manager">
    Создайте четыре секрета и предоставьте `roles/secretmanager.secretAccessor` сервисному аккаунту `claude-gateway`:

    | Секрет                       | Источник                                  |
    | ---------------------------- | ----------------------------------------- |
    | `gateway-jwt-secret`         | `openssl rand -base64 32`                 |
    | `gateway-oidc-client-secret` | Google Cloud Console → OAuth client       |
    | `gateway-postgres-url`       | `$GATEWAY_POSTGRES_URL` из шага Cloud SQL |
    | `gateway-config`             | полный `gateway.yaml` из предыдущего шага |

    Способ доставки секретов в контейнер отличается в зависимости от трека:

    * На GKE они монтируются как файлы через драйвер Secret Manager CSI, и `gateway.yaml` ссылается на `${file:/secrets/...}`.
    * На Cloud Run, который не может монтировать несколько секретов в один каталог, `gateway.yaml` монтируется как файл, а остальные три внедряются как переменные окружения, поэтому `gateway.yaml` ссылается на `${GATEWAY_JWT_SECRET}`, `${OIDC_CLIENT_SECRET}` и `${GATEWAY_POSTGRES_URL}` вместо этого.
  </Step>

  <Step title="Развертывание">
    <Tabs>
      <Tab title="Cloud Run">
        Команда ниже развертывает для производства за внутренним балансировщиком нагрузки.

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

        Прямой исход VPC через `--network`, `--subnet` и `--vpc-egress=private-ranges-only` позволяет сервису достичь приватного IP Cloud SQL напрямую. Публичный исход к конечным точкам Agent Platform и `accounts.google.com` идёт непосредственно в интернет, а не через VPC, поэтому Cloud NAT не требуется.

        Проверка IAM вызывающего должна быть открыта или отключена. Шлюз запускает свой собственный OIDC, и его клиенты не несут токен GCP, поэтому проверка вызывающего Cloud Run должна допускать неаутентифицированные запросы. Аутентификация запроса OIDC шлюза происходит после того, как он достигает контейнера, с `allowed_email_domains`, ограничивающим, какие домены могут входить.

        Два флага допускают неаутентифицированные запросы:

        * `--no-invoker-iam-check`: отключает проверку без привязки `allUsers` для управления и работает в рамках Domain Restricted Sharing
        * `--allow-unauthenticated`: предоставляет `allUsers` роль `run.invoker`; используйте, если ваша организация не позволяет `--no-invoker-iam-check`

        Ограничение входа через `--ingress` — это отдельный, независимый слой от проверки вызывающего; держите его установленным, чтобы ограничить сервис вашей корпоративной сетью.

        По умолчанию URL Cloud Run `*.run.app` разрешается на публичный адрес, который проверка [приватной сети](/docs/ru/claude-apps-gateway#prerequisites) `/login` отклоняет. Две топологии дают разработчикам приватно разрешаемое имя хоста, и Cloud Run не подготавливает ни одну для вас:

        * **Internal Application Load Balancer**, топология, которую предполагает команда развертывания выше: развертывайте с `--ingress=internal-and-cloud-load-balancing`, подготовьте внутренний Application Load Balancer перед сервисом с внутренним именем DNS и сертификатом, и установите `listen.public_url` на это имя хоста.
        * **Внутренний вход только без балансировщика нагрузки**: развертывайте с `--ingress=internal` и оставьте `listen.public_url` как URL `*.run.app`, значение по умолчанию в [эталонных активах](#terraform-reference) ниже. Чтобы `*.run.app` разрешался приватно, ваша команда сети должна уже работать с конечной точкой Private Service Connect для Google APIs, приватной зоной Cloud DNS, разрешающей `*.run.app` для неё, и маршрутизацией на месте к этой конечной точке.

        [Руководство Google по приватной сети для Cloud Run](https://cloud.google.com/run/docs/securing/private-networking) охватывает инфраструктуру, которая требуется обоим вариантам. Проверьте вход после того, как шлюз обслуживает приватное имя хоста; до этого подтвердите загрузку контейнера из его журналов в Cloud Run.

        Обновите URI перенаправления авторизации клиента OAuth на `<public_url>/oauth/callback` перед первым входом. Переразверните после изменения `public_url`, потому что шлюз строит свой публичный источник только из этого параметра и игнорирует `X-Forwarded-Host` и `X-Forwarded-Proto`. `X-Forwarded-For` соблюдается для IP-адресов клиентов только когда установлен `listen.trusted_proxies`.
      </Tab>

      <Tab title="GKE">
        Кластер должен находиться на `$VPC`, созданном на шаге Cloud SQL, чтобы поды могли достичь приватного IP базы данных; одного пиринга VPC недостаточно, потому что приватный IP Cloud SQL сам по себе является пиринговой сетью, и пиринг не является транзитивным. Чтобы создать новый кластер на этом VPC, передайте `--network="$VPC" --subnetwork=cc-gateway-subnet` в `gcloud container clusters create`.

        Включите Workload Identity на кластере и его пулах узлов, затем привяжите сервисный аккаунт Google к сервисному аккаунту Kubernetes, чтобы поды наследовали его учётные данные:

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

        Развертывайте шлюз как стандартное развертывание плюс сервис и внутренний Ingress, класс `gce-internal`, как описано в [развертывании Kubernetes](/docs/ru/claude-apps-gateway-deploy#kubernetes), с:

        * `serviceAccountName: gateway`
        * драйвер Secret Manager CSI, монтирующий секреты в `/secrets`
        * зонд готовности, указывающий на `GET /readyz`

        Присоедините BackendConfig с повышенным `timeoutSec` к сервису шлюза: сервис бэкенда балансировщика нагрузки за GKE Ingress по умолчанию имеет тайм-аут 30 секунд, который отсекает длинные потоковые ответы.

        Не применяйте исходящую NetworkPolicy, которая блокирует `169.254.169.254` на кластере Workload Identity; под должен достичь сервера метаданных для учётных данных. Встроенная [защита SSRF](/docs/ru/claude-apps-gateway-deploy#threat-model-summary) шлюза — это защита там.

        Шлюз регистрирует предупреждение при загрузке о том, что конечная точка метаданных доступна и предлагает применить исходящую NetworkPolicy. При Workload Identity это предупреждение ожидается, потому что под нуждается в конечной точке.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Отправка URL шлюза на машины разработчиков">
    Шлюз теперь работает, но разработчики не могут достичь его из `/login` до тех пор, пока URL шлюза не будет на их машинах. Установите `forceLoginMethod` и `forceLoginGatewayUrl` в [файле управляемых параметров](/docs/ru/claude-apps-gateway#set-the-gateway-url), который вы развертываете на каждом устройстве через MDM. В средстве выбора входа нет опции шлюза для разработчика, чтобы выбрать вручную.
  </Step>
</Steps>

<h2 id="terraform-reference">
  Справочник Terraform
</h2>

[Эталонные активы развертывания](https://github.com/anthropics/claude-code/tree/main/examples/gateway/gcp) автоматизируют трек Cloud Run на этой странице; активы конфигурации и образа применяются к обоим трекам:

* `setup.sh`: идемпотентный подготовщик `gcloud`, который проходит полный путь Cloud Run, от включения API до первого развертывания
* `terraform/`: то же развертывание как инфраструктура как код, для развертывания на зелёном поле: целевое применение для создания репозитория Artifact Registry, затем построение и отправка образа, затем полное применение
* `gateway.yaml.example` и `Dockerfile` для образа среды выполнения distroless

Артефакты по умолчанию используют входящий трафик Cloud Run `internal`, поэтому балансировщик нагрузки не требуется. Чтобы соответствовать развертыванию этой страницы за ALB для производства, запустите `setup.sh` с `INGRESS=internal-and-cloud-load-balancing` или установите переменную Terraform `ingress` на `INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER`. Артефакты также по умолчанию используют слой вызывающего как предоставление `allUsers` `run.invoker` вместо `--no-invoker-iam-check`, обратное пошаговому руководству этой страницы; оба работают, и выбор зависит от ограничений политики вашей организации.

Активы предоставляются как рабочие примеры, а не как поддерживаемый артефакт для производства; проверьте и адаптируйте их к своей среде.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Для ошибок загрузки и входа шлюза см. независимую от платформы [таблицу troubleshooting](/docs/ru/claude-apps-gateway-deploy#troubleshooting). Записи ниже относятся к Google Cloud.

| Симптом                                                                               | Причина                                                                                                                                        | Исправление                                                                                                                                                                                                                            |
| ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cloud Run возвращает `403 Forbidden` перед достижением контейнера                     | Проверка IAM вызывающего всё ещё включена                                                                                                      | Развертывайте с `--no-invoker-iam-check` или предоставьте `allUsers` роль `run.invoker` с `--allow-unauthenticated`                                                                                                                    |
| `--no-invoker-iam-check` отклонён с `invoker_iam_disabled is not currently available` | Заблокировано `constraints/run.managed.requireInvokerIam`                                                                                      | Используйте `--allow-unauthenticated`. Если Domain Restricted Sharing через `constraints/iam.allowedPolicyMemberDomains` также блокирует это, используйте трек GKE, который предоставляет шлюз на уровне сети без привязки `allUsers`. |
| `Container manifest type … must support amd64/linux` при развертывании                | Образ был построен на хосте, отличном от amd64, или buildx выдал индекс образа OCI                                                             | Постройте с `--platform=linux/amd64 --provenance=false`                                                                                                                                                                                |
| Загрузка шлюза завершается с ошибкой тайм-аута подключения Postgres на Cloud Run      | Сервис не подключён к VPC или Cloud SQL не имеет приватного IP на этом VPC; хранилище перестаёт ждать после 5 секунд                           | Развертывайте с `--network` и `--subnet` для прямого исхода VPC и создавайте экземпляр Cloud SQL с `--no-assign-ip` и `--network`, указывающим на тот же VPC                                                                           |
| Запросы Agent Platform возвращают `403 PERMISSION_DENIED`                             | Среда выполнения не использует сервисный аккаунт `claude-gateway` или модель не включена в Model Garden для проекта                            | Установите `--service-account` на Cloud Run или привяжите Workload Identity на GKE и включите каждую модель Claude в Model Garden для целевого региона                                                                                 |
| Потоковые ответы обрезаны после фиксированной продолжительности                       | Тайм-аут запроса переднего конца: сервис бэкенда балансировщика нагрузки за GKE Ingress по умолчанию имеет 30 секунд, а Cloud Run — 300 секунд | Присоедините BackendConfig с повышенным `timeoutSec` на GKE или развертывайте с `--timeout=3600` на Cloud Run                                                                                                                          |

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Справочник конфигурации](/docs/ru/claude-apps-gateway-config): каждый параметр `gateway.yaml`, включая `managed.policies` и `telemetry`
* [Развертывание и операции](/docs/ru/claude-apps-gateway-deploy): настройка IdP, проверки здоровья, ротация секрета JWT, обновления и модель безопасности
* [Обзор Claude apps gateway](/docs/ru/claude-apps-gateway): быстрый старт и подключение разработчиков
