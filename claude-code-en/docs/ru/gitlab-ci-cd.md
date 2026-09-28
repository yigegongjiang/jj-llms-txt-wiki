> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Узнайте об интеграции Claude Code в ваш рабочий процесс разработки с GitLab CI/CD

<Info>
  Claude Code для GitLab CI/CD в настоящее время находится в бета-версии. Функции и возможности могут развиваться по мере совершенствования опыта.

  Эта интеграция поддерживается GitLab. Для получения поддержки см. следующий [вопрос GitLab](https://gitlab.com/gitlab-org/gitlab/-/issues/573776).
</Info>

<Note>
  Эта интеграция построена на основе [Claude Code CLI и Agent SDK](/docs/ru/agent-sdk/overview), обеспечивая программное использование Claude в ваших заданиях CI/CD и пользовательских рабочих процессах автоматизации.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  Почему использовать Claude Code с GitLab?
</h2>

* **Мгновенное создание MR**: Опишите, что вам нужно, и Claude предложит полный MR с изменениями и объяснением
* **Автоматизированная реализация**: Превратите проблемы в рабочий код с помощью одной команды или упоминания
* **Осведомленность о проекте**: Claude следует вашим рекомендациям `CLAUDE.md` и существующим шаблонам кода
* **Простая настройка**: Добавьте одно задание в `.gitlab-ci.yml` и замаскированную переменную CI/CD
* **Готово для предприятия**: Выберите Claude API, Amazon Bedrock или Google Cloud's Agent Platform для соответствия требованиям к месторасположению данных и закупкам
* **Безопасно по умолчанию**: Работает на ваших GitLab runners с вашей защитой ветвей и утверждениями

<h2 id="how-it-works">
  Как это работает
</h2>

Claude Code использует GitLab CI/CD для запуска задач AI в изолированных заданиях и фиксации результатов обратно через MR:

1. **Оркестровка, управляемая событиями**: GitLab прослушивает выбранные вами триггеры (например, комментарий, упоминающий `@claude` в проблеме, MR или потоке рецензирования). Задание собирает контекст из потока и репозитория, создает подсказки из этого ввода и запускает Claude Code.

2. **Абстракция поставщика**: Используйте поставщика, который подходит для вашей среды:
   * Claude API (SaaS)
   * Amazon Bedrock (доступ на основе IAM, опции между регионами)
   * Google Cloud's Agent Platform (собственный GCP, Workload Identity Federation)

3. **Изолированное выполнение**: Каждое взаимодействие выполняется в контейнере со строгими правилами сети и файловой системы. Claude Code обеспечивает разрешения с областью действия рабочего пространства для ограничения записей. Каждое изменение проходит через MR, чтобы рецензенты видели diff и применялись утверждения.

Выберите региональные конечные точки, чтобы снизить задержку и соответствовать требованиям суверенитета данных при использовании существующих облачных соглашений.

<h2 id="what-can-claude-do">
  Что может делать Claude?
</h2>

В конвейере GitLab Claude Code может:

* Создавать и обновлять MR на основе описаний проблем или комментариев
* Анализировать регрессии производительности и предлагать оптимизации
* Реализовывать функции непосредственно в ветке, а затем открывать MR
* Исправлять ошибки и регрессии, выявленные тестами или комментариями
* Отвечать на последующие комментарии для итерации по запрошенным изменениям

<h2 id="setup">
  Настройка
</h2>

<h3 id="quick-setup">
  Быстрая настройка
</h3>

Самый быстрый способ начать работу — добавить минимальное задание в ваш `.gitlab-ci.yml` и установить ваш API ключ как замаскированную переменную.

1. **Добавьте замаскированную переменную CI/CD**
   * Перейдите в **Settings** → **CI/CD** → **Variables**
   * Добавьте `ANTHROPIC_API_KEY` (замаскирована, защищена при необходимости)

2. **Добавьте задание Claude в `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Adjust rules to fit how you want to trigger the job:
  # - manual runs
  # - merge request events
  # - web/API triggers when a comment contains '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # Optional: start a GitLab MCP server if your setup provides one
    - /bin/gitlab-mcp-server || true
    # Use AI_FLOW_* variables when invoking via web/API triggers with context payloads
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

После добавления задания и переменной `ANTHROPIC_API_KEY` протестируйте, запустив задание вручную из **CI/CD** → **Pipelines**, или запустите его из MR, чтобы Claude предложил обновления в ветке и открыл MR при необходимости.

<Note>
  Для запуска на Amazon Bedrock или Google Cloud's Agent Platform вместо Claude API см. раздел [Using with Amazon Bedrock and Google Cloud](#using-with-amazon-bedrock-and-google-cloud) ниже для аутентификации и настройки окружения.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Ручная настройка (рекомендуется для production)
</h3>

Если вы предпочитаете более контролируемую настройку или вам нужны поставщики услуг для предприятий:

1. **Настройте доступ к поставщику услуг**:
   * **Claude API**: Создайте и сохраните `ANTHROPIC_API_KEY` как замаскированную переменную CI/CD
   * **Amazon Bedrock**: **Configure GitLab** → **AWS OIDC** и создайте роль IAM для Amazon Bedrock
   * **Google Cloud's Agent Platform**: **Configure Workload Identity Federation for GitLab** → **GCP**

2. **Добавьте учетные данные проекта для операций GitLab API**:
   * Используйте `CI_JOB_TOKEN` по умолчанию или создайте Project Access Token с областью `api`
   * Сохраните как `GITLAB_ACCESS_TOKEN` (замаскирована), если используете PAT

3. **Добавьте задание Claude в `.gitlab-ci.yml`**: используйте задание [Quick setup](#quick-setup) для Claude API или задание поставщика услуг из [Configuration examples](#configuration-examples)

4. **(Опционально) Включите триггеры, управляемые упоминаниями**:
   * Добавьте webhook проекта для "Comments (notes)" к вашему обработчику событий (если вы его используете)
   * Пусть обработчик вызывает API триггера pipeline с переменными, такими как `AI_FLOW_INPUT` и `AI_FLOW_CONTEXT`, когда комментарий содержит `@claude`

<h2 id="example-use-cases">
  Примеры использования
</h2>

<h3 id="turn-issues-into-mrs">
  Преобразование проблем в MR
</h3>

В комментарии к проблеме:

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude анализирует проблему и кодовую базу, вносит изменения в ветку и открывает MR для проверки.

<h3 id="get-implementation-help">
  Получение помощи с реализацией
</h3>

В обсуждении MR:

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude предлагает изменения, добавляет код с надлежащим кешированием и обновляет MR.

<h3 id="fix-bugs-quickly">
  Быстрое исправление ошибок
</h3>

В комментарии к проблеме или MR:

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude находит ошибку, реализует исправление и обновляет ветку или открывает новый MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Использование с Amazon Bedrock и Google Cloud
</h2>

Для корпоративных сред вы можете запустить Claude Code полностью на инфраструктуре вашего облака с тем же опытом разработчика.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Предварительные требования

    Перед настройкой Claude Code с Amazon Bedrock вам потребуется:

    1. Учетная запись AWS с доступом к Amazon Bedrock для нужных моделей Claude
    2. GitLab, настроенный как поставщик идентификации OIDC в AWS IAM
    3. Роль IAM с разрешениями Amazon Bedrock и политикой доверия, ограниченной вашим проектом/ссылками GitLab
    4. Переменные GitLab CI/CD для предположения роли:
       * `AWS_ROLE_TO_ASSUME` (ARN роли)
       * `AWS_REGION` (регион Amazon Bedrock)

    ### Инструкции по настройке

    Настройте AWS, чтобы разрешить заданиям GitLab CI предполагать роль IAM через OIDC (без статических ключей).

    **Требуемая настройка:**

    1. Включите Amazon Bedrock и запросите доступ к целевым моделям Claude
    2. Создайте поставщика OIDC IAM для GitLab, если он еще не существует
    3. Создайте роль IAM, доверяющую поставщику OIDC GitLab, ограниченную вашим проектом и защищенными ссылками
    4. Присоедините разрешения с минимальными привилегиями для API вызова Amazon Bedrock

    Используйте [пример задания Amazon Bedrock](#configuration-examples) для обмена токена OIDC задания на временные учетные данные AWS во время выполнения.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Предварительные требования

    Перед настройкой Claude Code с Google Cloud's Agent Platform вам потребуется:

    1. Проект Google Cloud с:
       * включенным API Google Cloud's Agent Platform
       * настроенной федерацией рабочей нагрузки для доверия OIDC GitLab
    2. Выделенная учетная запись службы только с требуемыми ролями Google Cloud's Agent Platform
    3. Переменные GitLab CI/CD:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (имя ресурса поставщика без префикса `//iam.googleapis.com/`, например `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`)
       * `GCP_SERVICE_ACCOUNT` (адрес электронной почты учетной записи службы)
       * `GCP_PROJECT_ID` (идентификатор проекта Google Cloud)

    ### Инструкции по настройке

    Настройте Google Cloud, чтобы разрешить заданиям GitLab CI олицетворять учетную запись службы через федерацию рабочей нагрузки.

    **Требуемая настройка:**

    1. Включите API учетных данных IAM, API STS и API Google Cloud's Agent Platform
    2. Создайте пул рабочей нагрузки и поставщика для OIDC GitLab
    3. Создайте выделенную учетную запись службы с ролями Google Cloud's Agent Platform
    4. Предоставьте субъекту WIF разрешение на олицетворение учетной записи службы

    Используйте [пример задания Agent Platform](#configuration-examples) для аутентификации без сохранения ключей.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Примеры конфигурации
</h2>

Ниже приведены готовые к использованию фрагменты кода, которые вы можете адаптировать для своего конвейера.

<h3 id="amazon-bedrock-job-example-oidc">
  Пример задания Amazon Bedrock (OIDC)
</h3>

**Предварительные требования:**

* Amazon Bedrock включен с доступом к выбранной модели Claude
* GitLab OIDC настроен в AWS с ролью, которая доверяет вашему проекту GitLab и refs
* Роль IAM с разрешениями Amazon Bedrock (рекомендуется принцип наименьших привилегий)

**Требуемые переменные CI/CD:**

* `AWS_ROLE_TO_ASSUME`: ARN роли IAM для доступа к Amazon Bedrock
* `AWS_REGION`: регион Amazon Bedrock (например, `us-west-2`)

GitLab создает токен OIDC задания из блока `id_tokens:` и предоставляет его как `GITLAB_OIDC_TOKEN`. Установите `aud` на значение аудитории, которое вы настроили на поставщике идентификации OIDC IAM в AWS, например URL вашего экземпляра GitLab.

```yaml theme={null}
stages:
  - ai

claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apk add --no-cache bash curl jq git aws-cli
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Exchange the job's OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="/tmp/oidc_token"
    - printf "%s" "$GITLAB_OIDC_TOKEN" > "$AWS_WEB_IDENTITY_TOKEN_FILE"
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
    CLAUDE_CODE_USE_BEDROCK: "1"
```

<Note>
  Идентификаторы моделей для Amazon Bedrock включают префиксы, специфичные для региона (например, `us.anthropic.claude-sonnet-4-6`). Передайте желаемую модель через конфигурацию задания или подсказку, если ваш рабочий процесс это поддерживает.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Пример задания Agent Platform (Workload Identity Federation)
</h3>

**Предварительные требования:**

* API Agent Platform Google Cloud включен в вашем проекте GCP
* Workload Identity Federation настроена для доверия GitLab OIDC
* Учетная запись сервиса с разрешениями Google Cloud Agent Platform

**Требуемые переменные CI/CD:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: имя ресурса поставщика без префикса `//iam.googleapis.com/`, например `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT`: адрес электронной почты учетной записи сервиса
* `GCP_PROJECT_ID`: идентификатор проекта Google Cloud
* `CLOUD_ML_REGION`: регион Google Cloud Agent Platform (например, `us-east5`)

GitLab создает токен OIDC задания из блока `id_tokens:` и предоставляет его как `GITLAB_OIDC_TOKEN`. Установите `aud` на значение аудитории, которое вы настроили на поставщике Workload Identity Pool, например URL вашего экземпляра GitLab. Задание записывает токен в файл, и запись `credential_source` конфигурации учетных данных указывает библиотекам аутентификации Google читать его оттуда. Установка `GOOGLE_APPLICATION_CREDENTIALS` на файл конфигурации учетных данных делает его доступным для Claude Code через [Application Default Credentials](/docs/ru/google-vertex-ai#3-configure-gcp-credentials).

```yaml theme={null}
stages:
  - ai

claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Write the job's OIDC token where credential_source expects it
    - printf "%s" "$GITLAB_OIDC_TOKEN" > /tmp/oidc_token
    # Write the WIF credential configuration to a file (no downloaded keys)
    - |
      cat > /tmp/cred.json <<EOF
      {
        "type": "external_account",
        "audience": "//iam.googleapis.com/${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "token_url": "https://sts.googleapis.com/v1/token",
        "credential_source": {
          "file": "/tmp/oidc_token"
        },
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken"
      }
      EOF
    # Expose the credentials to Claude Code via Application Default Credentials
    - export GOOGLE_APPLICATION_CREDENTIALS=/tmp/cred.json
    # Authenticate the gcloud CLI with the same credential configuration
    - gcloud auth login --cred-file=/tmp/cred.json
    - gcloud config set project "$GCP_PROJECT_ID"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
    CLAUDE_CODE_USE_VERTEX: "1"
    ANTHROPIC_VERTEX_PROJECT_ID: "$GCP_PROJECT_ID"
```

<Note>
  С Workload Identity Federation вам не нужно хранить ключи учетной записи сервиса. Используйте условия доверия, специфичные для репозитория, и учетные записи сервиса с наименьшими привилегиями.
</Note>

<h2 id="best-practices">
  Лучшие практики
</h2>

<h3 id="claude-md-configuration">
  Конфигурация CLAUDE.md
</h3>

Создайте файл `CLAUDE.md` в корне репозитория для определения стандартов кодирования, критериев проверки и правил, специфичных для проекта. Claude читает этот файл во время запусков и следует вашим соглашениям при предложении изменений.

<h3 id="security-considerations">
  Соображения безопасности
</h3>

**Никогда не коммитьте API ключи или облачные учетные данные в ваш репозиторий**. Всегда используйте переменные GitLab CI/CD:

* Добавьте `ANTHROPIC_API_KEY` как замаскированную переменную (и защитите её при необходимости)
* Используйте OIDC, специфичный для поставщика, где это возможно (без долгоживущих ключей)
* Ограничьте разрешения заданий и исходящий сетевой трафик
* Проверяйте MR Claude как любого другого участника

<h3 id="optimizing-performance">
  Оптимизация производительности
</h3>

* Держите `CLAUDE.md` сосредоточенным и кратким
* Предоставляйте четкие описания проблем/MR для сокращения итераций
* Кэшируйте установки npm и пакетов в runners где это возможно

<h3 id="ci-costs">
  Затраты на CI
</h3>

При использовании Claude Code с GitLab CI/CD помните о связанных затратах:

* **Время GitLab Runner**:
  * Claude работает на ваших GitLab runners и потребляет минуты вычислений
  * Подробнее см. в информации о биллинге runner вашего плана GitLab

* **Затраты на API**:
  * Каждое взаимодействие Claude потребляет токены на основе размера запроса и ответа
  * Использование токенов варьируется в зависимости от сложности задачи и размера кодовой базы
  * Подробнее см. в [ценообразовании Anthropic](https://platform.claude.com/docs/en/about-claude/pricing)

* **Советы по оптимизации затрат**:
  * Используйте специфичные команды `@claude` для сокращения ненужных ходов
  * Установите подходящие значения `--max-turns` и `timeout` задания
  * Ограничьте параллелизм для контроля параллельных запусков

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude не отвечает на команды @claude
</h3>

* Убедитесь, что ваш pipeline запускается (вручную, событием MR или через слушатель событий note/webhook)
* Убедитесь, что переменные `ANTHROPIC_API_KEY` или переменные облачного провайдера присутствуют
* Проверьте, что комментарий содержит `@claude` (не `/claude`) и что ваш триггер упоминания настроен

<h3 id="job-can’t-write-comments-or-open-mrs">
  Задание не может писать комментарии или открывать MR
</h3>

* Убедитесь, что `CI_JOB_TOKEN` имеет достаточные разрешения для проекта, или используйте Project Access Token с областью `api`
* Проверьте, что инструмент `mcp__gitlab` включен в `--allowedTools`
* Подтвердите, что задание выполняется в контексте MR или имеет достаточный контекст через переменные `AI_FLOW_*`

<h3 id="authentication-errors">
  Ошибки аутентификации
</h3>

* **Для Claude API**: Подтвердите, что `ANTHROPIC_API_KEY` действителен и не истек
* **Для Amazon Bedrock или Agent Platform Google Cloud**: Проверьте конфигурацию OIDC/WIF, олицетворение роли и имена секретов; подтвердите доступность региона и модели

<h2 id="advanced-configuration">
  Расширенная конфигурация
</h2>

<h3 id="common-parameters-and-variables">
  Общие параметры и переменные
</h3>

Управляйте запусками Claude Code в ваших заданиях с помощью этих флагов CLI, ключевых слов GitLab и переменных:

* `-p`: предоставьте инструкции встроенным образом, например `claude -p "Review this MR"`
* `--max-turns`: ограничьте количество итераций туда и обратно
* `timeout`: ограничьте общее время выполнения задания с помощью ключевого слова `timeout` на уровне задания GitLab, например `timeout: 30m`
* `ANTHROPIC_API_KEY`: требуется для Claude API (не используется для Amazon Bedrock или Google Cloud's Agent Platform)
* Специфичная для провайдера среда: `AWS_REGION`, переменные проекта/региона для Google Cloud's Agent Platform

<Note>
  Точные флаги и параметры могут отличаться в зависимости от версии `@anthropic-ai/claude-code`. Запустите `claude --help` в вашем задании, чтобы увидеть поддерживаемые опции.
</Note>

<h3 id="customizing-claude’s-behavior">
  Настройка поведения Claude
</h3>

Вы можете направлять Claude двумя основными способами:

1. **CLAUDE.md**: определите стандарты кодирования, требования безопасности и соглашения проекта. Claude читает это во время запусков и следует вашим правилам.
2. **Пользовательские подсказки**: передайте инструкции, специфичные для задачи, через `-p` в задании. Используйте разные подсказки для разных заданий (например, review, implement, refactor).
