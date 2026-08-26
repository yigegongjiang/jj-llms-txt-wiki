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

Claude Code обеспечивает мощные рабочие процессы CI/CD, которые преобразуют способ работы с кодом:

* Создание и обновление MR из описаний проблем или комментариев
* Анализ регрессий производительности и предложение оптимизаций
* Прямая реализация функций в ветви, затем открытие MR
* Исправление ошибок и регрессий, выявленных тестами или комментариями
* Ответ на последующие комментарии для итерации по запрошенным изменениям

<h2 id="setup">
  Настройка
</h2>

<h3 id="quick-setup">
  Быстрая настройка
</h3>

Самый быстрый способ начать работу — добавить минимальное задание в ваш `.gitlab-ci.yml` и установить ваш ключ API как замаскированную переменную.

1. **Добавьте замаскированную переменную CI/CD**
   * Перейдите в **Settings** → **CI/CD** → **Variables**
   * Добавьте `ANTHROPIC_API_KEY` (замаскирована, защищена по мере необходимости)

2. **Добавьте задание Claude в `.gitlab-ci.yml`**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Отрегулируйте правила в соответствии с тем, как вы хотите запустить задание:
  # - ручные запуски
  # - события merge request
  # - веб/API триггеры, когда комментарий содержит '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    # Опционально: запустите сервер GitLab MCP, если ваша настройка его предоставляет
    - /bin/gitlab-mcp-server || true
    # Используйте переменные AI_FLOW_* при вызове через веб/API триггеры с полезными нагрузками контекста
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

После добавления задания и переменной `ANTHROPIC_API_KEY` протестируйте, запустив задание вручную из **CI/CD** → **Pipelines**, или запустите его из MR, чтобы позволить Claude предложить обновления в ветви и открыть MR при необходимости.

<Note>
  Для запуска на Amazon Bedrock или Google Cloud's Agent Platform вместо Claude API см. раздел [Использование с Amazon Bedrock и Google Cloud](#using-with-amazon-bedrock-and-google-cloud) ниже для настройки аутентификации и окружения.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Ручная настройка (рекомендуется для производства)
</h3>

Если вы предпочитаете более контролируемую настройку или вам нужны поставщики для предприятия:

1. **Настройте доступ поставщика**:
   * **Claude API**: Создайте и сохраните `ANTHROPIC_API_KEY` как замаскированную переменную CI/CD
   * **Amazon Bedrock**: **Настройте GitLab** → **AWS OIDC** и создайте роль IAM для Amazon Bedrock
   * **Google Cloud's Agent Platform**: **Настройте Workload Identity Federation для GitLab** → **GCP**

2. **Добавьте учетные данные проекта для операций GitLab API**:
   * Используйте `CI_JOB_TOKEN` по умолчанию или создайте Project Access Token с областью `api`
   * Сохраните как `GITLAB_ACCESS_TOKEN` (замаскирована), если используете PAT

3. **Добавьте задание Claude в `.gitlab-ci.yml`** (см. примеры ниже)

4. **(Опционально) Включите триггеры, управляемые упоминаниями**:
   * Добавьте webhook проекта для "Comments (notes)" к вашему прослушивателю событий (если вы его используете)
   * Попросите прослушиватель вызвать API триггера конвейера с переменными, такими как `AI_FLOW_INPUT` и `AI_FLOW_CONTEXT`, когда комментарий содержит `@claude`

<h2 id="example-use-cases">
  Примеры использования
</h2>

<h3 id="turn-issues-into-mrs">
  Превратите проблемы в MR
</h3>

В комментарии проблемы:

```text theme={null}
@claude implement this feature based on the issue description
```

Claude анализирует проблему и кодовую базу, записывает изменения в ветви и открывает MR для рецензирования.

<h3 id="get-implementation-help">
  Получите помощь в реализации
</h3>

В обсуждении MR:

```text theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude предлагает изменения, добавляет код с соответствующим кешированием и обновляет MR.

<h3 id="fix-bugs-quickly">
  Быстро исправляйте ошибки
</h3>

В комментарии проблемы или MR:

```text theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude находит ошибку, реализует исправление и обновляет ветвь или открывает новый MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Использование с Amazon Bedrock и Google Cloud
</h2>

Для корпоративных сред вы можете запустить Claude Code полностью на вашей облачной инфраструктуре с тем же опытом разработчика.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Предварительные требования

    Перед настройкой Claude Code с Amazon Bedrock вам потребуется:

    1. Учетная запись AWS с доступом Amazon Bedrock к желаемым моделям Claude
    2. GitLab, настроенный как поставщик идентификации OIDC в AWS IAM
    3. Роль IAM с разрешениями Amazon Bedrock и политикой доверия, ограниченной вашим проектом/ссылками GitLab
    4. Переменные GitLab CI/CD для предположения роли:
       * `AWS_ROLE_TO_ASSUME` (ARN роли)
       * `AWS_REGION` (регион Amazon Bedrock)

    ### Инструкции по настройке

    Настройте AWS, чтобы позволить заданиям GitLab CI предположить роль IAM через OIDC (без статических ключей).

    **Требуемая настройка:**

    1. Включите Amazon Bedrock и запросите доступ к целевым моделям Claude
    2. Создайте поставщика IAM OIDC для GitLab, если он еще не присутствует
    3. Создайте роль IAM, доверяющую поставщику GitLab OIDC, ограниченную вашим проектом и защищенными ссылками
    4. Прикрепите разрешения с наименьшими привилегиями для API вызова Amazon Bedrock

    **Требуемые значения для сохранения в переменных CI/CD:**

    * `AWS_ROLE_TO_ASSUME`
    * `AWS_REGION`

    Добавьте переменные в Settings → CI/CD → Variables:

    ```yaml theme={null}
    # Для Amazon Bedrock:
    - AWS_ROLE_TO_ASSUME
    - AWS_REGION
    ```

    Используйте пример задания Amazon Bedrock выше для обмена токена задания GitLab на временные учетные данные AWS во время выполнения.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Предварительные требования

    Перед настройкой Claude Code с Google Cloud's Agent Platform вам потребуется:

    1. Проект Google Cloud с:
       * Включенным API Google Cloud's Agent Platform
       * Настроенной Workload Identity Federation для доверия GitLab OIDC
    2. Выделенная учетная запись сервиса только с требуемыми ролями Google Cloud's Agent Platform
    3. Переменные GitLab CI/CD для WIF:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (полное имя ресурса)
       * `GCP_SERVICE_ACCOUNT` (адрес электронной почты учетной записи сервиса)

    ### Инструкции по настройке

    Настройте Google Cloud, чтобы позволить заданиям GitLab CI олицетворять учетную запись сервиса через Workload Identity Federation.

    **Требуемая настройка:**

    1. Включите IAM Credentials API, STS API и Google Cloud's Agent Platform API
    2. Создайте пул Workload Identity и поставщика для GitLab OIDC
    3. Создайте выделенную учетную запись сервиса с ролями Google Cloud's Agent Platform
    4. Предоставьте основному принципу WIF разрешение на олицетворение учетной записи сервиса

    **Требуемые значения для сохранения в переменных CI/CD:**

    * `GCP_WORKLOAD_IDENTITY_PROVIDER`
    * `GCP_SERVICE_ACCOUNT`

    Добавьте переменные в Settings → CI/CD → Variables:

    ```yaml theme={null}
    # Для Google Cloud's Agent Platform:
    - GCP_WORKLOAD_IDENTITY_PROVIDER
    - GCP_SERVICE_ACCOUNT
    - CLOUD_ML_REGION (например, us-east5)
    ```

    Используйте пример задания Google Cloud's Agent Platform выше для аутентификации без сохранения ключей.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Примеры конфигурации
</h2>

Ниже приведены готовые к использованию фрагменты, которые вы можете адаптировать к вашему конвейеру.

<h3 id="basic-gitlab-ci-yml-claude-api">
  Базовый .gitlab-ci.yml (Claude API)
</h3>

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Summarize recent changes and suggest improvements'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  # Claude Code будет использовать ANTHROPIC_API_KEY из переменных CI/CD
```

<h3 id="amazon-bedrock-job-example-oidc">
  Пример задания Amazon Bedrock (OIDC)
</h3>

**Предварительные требования:**

* Amazon Bedrock включен с доступом к выбранной модели Claude
* GitLab OIDC настроен в AWS с ролью, которая доверяет вашему проекту GitLab и ссылкам
* Роль IAM с разрешениями Amazon Bedrock (рекомендуется наименьшие привилегии)

**Требуемые переменные CI/CD:**

* `AWS_ROLE_TO_ASSUME`: ARN роли IAM для доступа к Amazon Bedrock
* `AWS_REGION`: Регион Amazon Bedrock (например, `us-west-2`)

```yaml theme={null}
claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apk add --no-cache bash curl jq git python3 py3-pip
    - pip install --no-cache-dir awscli
    - curl -fsSL https://claude.ai/install.sh | bash
    # Обменяйте токен GitLab OIDC на учетные данные AWS
    - export AWS_WEB_IDENTITY_TOKEN_FILE="${CI_JOB_JWT_FILE:-/tmp/oidc_token}"
    - if [ -n "${CI_JOB_JWT_V2}" ]; then printf "%s" "$CI_JOB_JWT_V2" > "$AWS_WEB_IDENTITY_TOKEN_FILE"; fi
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
* Учетная запись сервиса с разрешениями Agent Platform Google Cloud

**Требуемые переменные CI/CD:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: Полное имя ресурса поставщика
* `GCP_SERVICE_ACCOUNT`: Адрес электронной почты учетной записи сервиса
* `CLOUD_ML_REGION`: Регион Agent Platform Google Cloud (например, `us-east5`)

```yaml theme={null}
claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # Аутентифицируйтесь в Google Cloud через WIF (без загруженных ключей)
    - >
      gcloud auth login --cred-file=<(cat <<EOF
      {
        "type": "external_account",
        "audience": "${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken",
        "token_url": "https://sts.googleapis.com/v1/token"
      }
      EOF
      )
    - gcloud config set project "$(gcloud projects list --format='value(projectId)' --filter="name:${CI_PROJECT_NAMESPACE}" | head -n1)" || true
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
```

<Note>
  С Workload Identity Federation вам не нужно сохранять ключи учетной записи сервиса. Используйте условия доверия, специфичные для репозитория, и учетные записи сервиса с наименьшими привилегиями.
</Note>

<h2 id="best-practices">
  Лучшие практики
</h2>

<h3 id="claude-md-configuration">
  Конфигурация CLAUDE.md
</h3>

Создайте файл `CLAUDE.md` в корне репозитория, чтобы определить стандарты кодирования, критерии рецензирования и правила, специфичные для проекта. Claude читает этот файл во время запусков и следует вашим соглашениям при предложении изменений.

<h3 id="security-considerations">
  Соображения безопасности
</h3>

**Никогда не фиксируйте ключи API или учетные данные облака в вашем репозитории**. Всегда используйте переменные GitLab CI/CD:

* Добавьте `ANTHROPIC_API_KEY` как замаскированную переменную (и защитите ее при необходимости)
* Используйте OIDC, специфичный для поставщика, где возможно (без долгоживущих ключей)
* Ограничьте разрешения задания и исходящий трафик сети
* Рецензируйте MR Claude, как любого другого участника

<h3 id="optimizing-performance">
  Оптимизация производительности
</h3>

* Держите `CLAUDE.md` сосредоточенным и кратким
* Предоставляйте четкие описания проблем/MR, чтобы снизить количество итераций
* Настройте разумные тайм-ауты заданий, чтобы избежать неконтролируемых запусков
* Кешируйте npm и установки пакетов на runners, где возможно

<h3 id="ci-costs">
  Затраты CI
</h3>

При использовании Claude Code с GitLab CI/CD помните о связанных затратах:

* **Время GitLab Runner**:
  * Claude работает на ваших GitLab runners и потребляет минуты вычислений
  * Подробности о выставлении счетов за runner см. в плане GitLab

* **Затраты на API**:
  * Каждое взаимодействие Claude потребляет токены на основе размера подсказки и ответа
  * Использование токенов варьируется в зависимости от сложности задачи и размера кодовой базы
  * Подробности см. в [ценообразовании Anthropic](https://platform.claude.com/docs/ru/about-claude/pricing)

* **Советы по оптимизации затрат**:
  * Используйте конкретные команды `@claude` для снижения ненужных ходов
  * Установите соответствующие значения `max_turns` и тайм-аут задания
  * Ограничьте параллелизм для управления параллельными запусками

<h2 id="security-and-governance">
  Безопасность и управление
</h2>

* Каждое задание выполняется в изолированном контейнере с ограниченным доступом в сеть
* Изменения Claude проходят через MR, чтобы рецензенты видели каждый diff
* Правила защиты ветвей и утверждения применяются к коду, созданному AI
* Claude Code использует разрешения с областью действия рабочего пространства для ограничения записей
* Затраты остаются под вашим контролем, потому что вы приносите свои собственные учетные данные поставщика

<h2 id="troubleshooting">
  Устранение неполадок
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude не отвечает на команды @claude
</h3>

* Убедитесь, что ваш конвейер запускается (вручную, событие MR или через прослушиватель событий/webhook примечания)
* Убедитесь, что переменные CI/CD (`ANTHROPIC_API_KEY` или параметры облачного поставщика) присутствуют и не замаскированы
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
* **Для Amazon Bedrock или Google Cloud's Agent Platform**: Проверьте конфигурацию OIDC/WIF, олицетворение роли и имена секретов; подтвердите доступность региона и модели

<h2 id="advanced-configuration">
  Расширенная конфигурация
</h2>

<h3 id="common-parameters-and-variables">
  Общие параметры и переменные
</h3>

Claude Code поддерживает эти часто используемые входные данные:

* `prompt` / `prompt_file`: Предоставьте инструкции встроенно (`-p`) или через файл
* `max_turns`: Ограничьте количество взаимных итераций
* `timeout_minutes`: Ограничьте общее время выполнения
* `ANTHROPIC_API_KEY`: Требуется для Claude API (не используется для Amazon Bedrock или Google Cloud's Agent Platform)
* Окружение, специфичное для поставщика: `AWS_REGION`, переменные проекта/региона для Google Cloud's Agent Platform

<Note>
  Точные флаги и параметры могут варьироваться в зависимости от версии `@anthropic-ai/claude-code`. Запустите `claude --help` в вашем задании, чтобы увидеть поддерживаемые опции.
</Note>

<h3 id="customizing-claude’s-behavior">
  Настройка поведения Claude
</h3>

Вы можете направлять Claude двумя основными способами:

1. **CLAUDE.md**: Определите стандарты кодирования, требования безопасности и соглашения проекта. Claude читает это во время запусков и следует вашим правилам.
2. **Пользовательские подсказки**: Передайте инструкции, специфичные для задачи, через `prompt`/`prompt_file` в задании. Используйте разные подсказки для разных заданий (например, рецензирование, реализация, рефакторинг).
