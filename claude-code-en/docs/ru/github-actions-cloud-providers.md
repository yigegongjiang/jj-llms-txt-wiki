> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Использование Claude Code GitHub Actions с облачными провайдерами

> Запускайте Claude Code GitHub Actions через Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry вместо Claude API

[Claude Code GitHub Actions](/docs/ru/github-actions) по умолчанию вызывает Claude API. Чтобы маршрутизировать вывод через собственный облачный аккаунт, установите входной параметр провайдера Claude Code GitHub Action и настройте облако на доверие токену OpenID Connect (OIDC) рабочего процесса. Рабочий процесс аутентифицируется с помощью этого токена, поэтому вы не сохраняете долгоживущие облачные учетные данные в своем репозитории.

<Info>
  Эта страница основана на [настройке GitHub Actions](/docs/ru/github-actions#setup). Предполагается, что вы уже знакомы с файлом рабочего процесса и шагом `anthropics/claude-code-action`, и здесь рассматривается только то, что меняется для облачного провайдера.
</Info>

<h2 id="choose-your-provider">
  Выберите своего провайдера
</h2>

Claude Code GitHub Action поддерживает трех провайдеров, и шаги настройки ниже отличаются только конфигурацией на стороне облака. Используйте того, у которого ваша организация уже имеет доступ к моделям Claude. Вы указываете Claude Code GitHub Action, какого провайдера использовать, с помощью одного входного параметра в блоке `with:` шага `anthropics/claude-code-action`:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Полные примеры рабочих процессов в разделе [Настройка интеграции](#set-up-the-integration) уже включают входной параметр для каждого провайдера.

<h2 id="prerequisites">
  Предварительные требования
</h2>

Перед началом вам потребуется:

* Доступ администратора к репозиторию, где работает Claude Code GitHub Action, для установки GitHub App и добавления секретов
* Разрешение на создание ресурсов идентификации в вашем облачном аккаунте: роли IAM и поставщики идентификации OIDC на AWS, ресурсы Workload Identity Federation и сервисные аккаунты на Google Cloud, или приложения Microsoft Entra на Azure
* Доступ к моделям Claude на вашем провайдере:
  * **Amazon Bedrock**: доступ предоставлен моделям Claude. Профили кросс-региональной инференции, такие как идентификаторы моделей `us.` в примерах на этой странице, требуют доступа, предоставленного в каждом регионе их группы регионов. См. [Claude Code на Amazon Bedrock](/docs/ru/amazon-bedrock)
  * **Google Cloud's Agent Platform**: проект с включенным Agent Platform API и доступом к моделям Claude. См. [Claude Code на Google Cloud's Agent Platform](/docs/ru/google-vertex-ai)
  * **Microsoft Foundry**: ресурс Foundry с развертыванием модели Claude. См. [Claude Code на Microsoft Foundry](/docs/ru/microsoft-foundry)

<h2 id="set-up-the-integration">
  Настройка интеграции
</h2>

Помимо предварительных требований, вы создаете идентификацию GitHub для Claude Code GitHub Action, конфигурацию доверия на стороне облака, секреты репозитория и файл рабочего процесса. Шаги ниже проходят через каждый из них.

<Steps>
  <Step title="Выберите идентификацию GitHub">
    Claude Code GitHub Action отправляет коммиты и публикует комментарии через идентификацию GitHub. [Быстрая настройка](/docs/ru/github-actions#quick-setup) устанавливает официальное приложение Claude GitHub App для этого. С облачным провайдером вы выбираете идентификацию сами:

    * **Официальное [приложение Claude GitHub App](https://github.com/apps/claude)**: установите его на репозиторий, или пропустите на следующий шаг, если оно уже установлено
    * **Пользовательское приложение GitHub**: создайте свое собственное приложение, когда вам нужны только три разрешения, которые использует Claude Code GitHub Action, а не [полный набор официального приложения](/docs/ru/github-actions#github-app-permissions)
    * **Автоматический `GITHUB_TOKEN` GitHub**: нет приложения для создания или установки, но GitHub не запускает ваши CI рабочие процессы на коммитах, сделанных с его помощью

    Примеры рабочих процессов на четвертом шаге аутентифицируются с помощью пользовательского приложения. Этот шаг также указывает, что изменить для двух других вариантов.

    Чтобы создать пользовательское приложение, [зарегистрируйте новое приложение GitHub](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) с отключенными вебхуками, так как эта интеграция их не использует. Предоставьте ему три разрешения репозитория:

    * **Contents**: чтение и запись
    * **Issues**: чтение и запись
    * **Pull requests**: чтение и запись

    После регистрации приложения создайте приватный ключ и сохраните загруженный файл `.pem`, запишите ID приложения со страницы настроек приложения и [установите приложение](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app) на репозиторий, где работает Claude Code GitHub Action. Вы добавляете ключ и ID как секреты на третьем шаге.
  </Step>

  <Step title="Настройте облачную аутентификацию">
    Настройте облако на доверие токену OIDC, который GitHub выдает рабочему процессу, чтобы каждый запуск рабочего процесса получал краткосрочные облачные учетные данные. Маркеры в каждой вкладке суммируют то, что нужно создать, и каждая вкладка ссылается на собственное руководство облачного провайдера для шагов на уровне консоли.

    <Tabs>
      <Tab title="Amazon Bedrock">
        Создайте конфигурацию доверия в вашем аккаунте AWS, следуя [руководству AWS по созданию поставщиков идентификации OIDC](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html):

        * Добавьте поставщика идентификации GitHub OIDC с URL провайдера `https://token.actions.githubusercontent.com` и аудиторией `sts.amazonaws.com`
        * Создайте роль IAM, которой доверяет этот провайдер как веб-идентификация, и присоедините политику ограниченного вызова из [конфигурации IAM](/docs/ru/amazon-bedrock#iam-configuration), которая предоставляет `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles` и `bedrock:GetInferenceProfile`, а также два действия подписки `aws-marketplace`
        * Ограничьте политику доверия роли вашим репозиторием с условием субъекта, таким как `repo:your-org/your-repo:*`. См. [руководство GitHub по усилению безопасности OIDC](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect) для формата утверждения

        Запишите ARN роли. Вы добавляете его как секрет на следующем шаге.
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Создайте ресурсы федерации в вашем проекте Google Cloud, следуя [документации Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation):

        * Включите три API: IAM Credentials, Security Token Service (STS) и Agent Platform API, имя сервиса которого `aiplatform.googleapis.com`
        * Создайте пул Workload Identity с поставщиком GitHub OIDC, издатель которого `https://token.actions.githubusercontent.com`, и добавьте условие атрибута, которое ограничивает пул вашим репозиторием
        * Создайте выделенный сервисный аккаунт только с ролью `Vertex AI User`, которая является `roles/aiplatform.user`, и разрешите пулу его олицетворять

        Запишите полное имя ресурса провайдера и адрес электронной почты сервисного аккаунта. Вы добавляете их как секреты на следующем шаге.
      </Tab>

      <Tab title="Microsoft Foundry">
        Создайте приложение Microsoft Entra с федеративным учетным данием для вашего репозитория, следуя [руководству Microsoft по аутентификации из GitHub Actions](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect):

        * Зарегистрируйте приложение Microsoft Entra и добавьте федеративное учетное дание идентификации, которое доверяет токенам, выданным GitHub вашему репозиторию. Управляемое удостоверение, назначенное пользователем, работает вместо приложения. Оба имеют ID клиента, который вы запишите ниже
        * Назначьте приложению роль `Azure AI User` на вашем ресурсе Foundry. См. [конфигурацию Azure RBAC](/docs/ru/microsoft-foundry#azure-rbac-configuration) для более узкой пользовательской роли

        Запишите ID клиента приложения, ID вашего тенанта и ID вашей подписки. Вы добавляете их как секреты на следующем шаге.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Добавьте секреты репозитория">
    В репозитории, где работает Claude Code GitHub Action, добавьте секреты для вашего провайдера, плюс два секрета приложения, если вы создали пользовательское приложение GitHub на первом шаге. См. руководство GitHub по [использованию секретов в GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    | Секрет                           | Требуется для                      | Значение                                    |
    | -------------------------------- | ---------------------------------- | ------------------------------------------- |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock                     | ARN роли IAM                                |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud's Agent Platform      | Полное имя ресурса провайдера               |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud's Agent Platform      | Адрес электронной почты сервисного аккаунта |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry                  | ID клиента приложения Entra                 |
    | `AZURE_TENANT_ID`                | Microsoft Foundry                  | ID вашего тенанта Microsoft Entra           |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry                  | ID вашей подписки Azure                     |
    | `APP_ID`                         | Пользовательское приложение GitHub | ID приложения GitHub                        |
    | `APP_PRIVATE_KEY`                | Пользовательское приложение GitHub | Содержимое файла приватного ключа `.pem`    |
  </Step>

  <Step title="Создайте файл рабочего процесса">
    Создайте файл рабочего процесса для вашего провайдера, такой как `.github/workflows/claude.yml`. Каждый пример отвечает на упоминания `@claude`, аутентифицируется на GitHub с помощью пользовательского приложения и включает разрешение `id-token: write`, которое GitHub требует для выдачи токена OIDC, который ваш облачный провайдер обменивает на учетные данные.

    Если вы выбрали другую идентификацию GitHub на первом шаге, отрегулируйте пример:

    * **Официальное приложение Claude GitHub App**: удалите шаг Generate GitHub App token и строку `github_token`
    * **Автоматический токен GitHub**: удалите шаг генерации токена и измените строку `github_token` на `github_token: ${{ secrets.GITHUB_TOKEN }}`

    <Warning>
      В публичных репозиториях комментарий, содержащий фразу триггера от любого пользователя, запускает этот рабочий процесс. Шаги учетных данных выполняются до того, как Claude Code GitHub Action проверит доступ на запись комментатора, поэтому действие отклоняет неавторизованных пользователей только после того, как рабочий процесс создал токен приложения и вошел в ваш облачный провайдер, что оставляет записи в журнале аудита и потребляет минуты Actions. Чтобы избежать таких запусков, добавьте шаг, который проверяет доступ на запись комментатора перед шагами учетных данных.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        Замените значение `aws-region` на свое собственное. Шаг учетных данных экспортирует его как `AWS_REGION` для остальной части задания.

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Configure AWS Credentials (OIDC)
                uses: aws-actions/configure-aws-credentials@v4
                with:
                  role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
                  aws-region: us-west-2

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_bedrock: "true"
                  claude_args: '--model us.anthropic.claude-sonnet-4-6'
        ```

        <Tip>
          Идентификаторы моделей Bedrock включают префикс профиля кросс-региональной инференции, такой как `us.`. Используйте префикс для группы регионов, где вы предоставили доступ к модели.
        </Tip>
      </Tab>

      <Tab title="Google Cloud's Agent Platform">
        Замените значение `CLOUD_ML_REGION` на свое собственное. Вам не нужно жестко кодировать ID проекта, потому что рабочий процесс читает его из выходных данных шага `auth`.

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Google Cloud
                id: auth
                uses: google-github-actions/auth@v2
                with:
                  workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
                  service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
        ```
      </Tab>

      <Tab title="Microsoft Foundry">
        Замените `your-resource-name` на имя вашего ресурса Foundry. Claude Code строит URL конечной точки из него. Шаг `azure/login` входит с помощью токена OIDC рабочего процесса, и Claude Code подхватывает учетные данные через цепь [учетных данных по умолчанию](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) Azure.

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Azure
                uses: azure/login@v2
                with:
                  client-id: ${{ secrets.AZURE_CLIENT_ID }}
                  tenant-id: ${{ secrets.AZURE_TENANT_ID }}
                  subscription-id: ${{ secrets.AZURE_SUBSCRIPTION_ID }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_foundry: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_FOUNDRY_RESOURCE: your-resource-name
        ```

        <Tip>
          Используйте ID модели, который соответствует развертыванию Claude в вашем ресурсе Foundry. См. [Claude Code на Microsoft Foundry](/docs/ru/microsoft-foundry) для конфигурации модели и закрепления версии.
        </Tip>
      </Tab>
    </Tabs>

    С любым провайдером вы можете ограничить длину запуска и стоимость, добавив `--max-turns` к `claude_args`. См. [Управление затратами](/docs/ru/github-actions#manage-costs).
  </Step>

  <Step title="Протестируйте настройку">
    Упомяните `@claude` в комментарии проблемы или PR, затем смотрите запуск на вкладке Actions репозитория. Claude отвечает в комментарии на той же проблеме или PR.
  </Step>
</Steps>

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Неудачный запуск обычно разбивается в одном из двух мест:

* **Ошибки аутентификации**: обычно неправильная конфигурация OIDC. Проверьте, что рабочий процесс включает разрешение `id-token: write`, что условие репозитория конфигурации доверия точно соответствует вашему репозиторию и что имена секретов в вашем рабочем процессе совпадают с добавленными вами
* **Проблемы триггера и CI**: они ведут себя так же, как когда Claude Code GitHub Action вызывает Claude API. См. [раздел troubleshooting](/docs/ru/github-actions#troubleshooting) главной страницы и [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) Claude Code GitHub Action

<h2 id="what’s-next">
  Что дальше
</h2>

* [Claude Code GitHub Actions](/docs/ru/github-actions) для примеров, параметров и лучших практик
* [Claude Code на Amazon Bedrock](/docs/ru/amazon-bedrock) для идентификаторов и регионов моделей Bedrock
* [Claude Code на Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) для идентификаторов и регионов моделей Agent Platform
* [Claude Code на Microsoft Foundry](/docs/ru/microsoft-foundry) для конфигурации модели и конечной точки Foundry
