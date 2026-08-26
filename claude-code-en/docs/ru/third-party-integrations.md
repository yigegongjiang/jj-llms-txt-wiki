> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Обзор корпоративного развертывания

> Узнайте, как Claude Code может интегрироваться с различными сторонними сервисами и инфраструктурой для удовлетворения требований корпоративного развертывания.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

Организации могут развертывать Claude Code непосредственно через Anthropic или через поставщика облачных услуг. Эта страница поможет вам выбрать правильную конфигурацию.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  Сравнение вариантов развертывания
</h2>

Для большинства организаций Claude for Teams или Claude for Enterprise обеспечивает лучший опыт. Члены команды получают доступ как к Claude Code, так и к Claude в веб-версии с одной подпиской, централизованным выставлением счетов и без необходимости настройки инфраструктуры.

**Claude for Teams** — это самообслуживаемое решение, которое включает функции сотрудничества, инструменты администратора и управление выставлением счетов. Лучше всего подходит для небольших команд, которым нужно быстро начать работу.

**Claude for Enterprise** добавляет SSO и захват домена, разрешения на основе ролей, доступ к API соответствия требованиям и управляемые параметры политики для развертывания конфигураций Claude Code на уровне организации. Лучше всего подходит для крупных организаций с требованиями безопасности и соответствия требованиям.

Узнайте больше о [планах Team](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) и [планах Enterprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

Если ваша организация имеет специфические требования к инфраструктуре, сравните варианты ниже:

<table>
  <thead>
    <tr>
      <th>Функция</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform, ранее Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>Лучше всего подходит для</td>
      <td>Большинства организаций (рекомендуется)</td>
      <td>Отдельных разработчиков</td>
      <td>Развертываний, собственных для AWS</td>
      <td>Выставление счетов AWS Marketplace с функциями Claude API</td>
      <td>Развертываний, собственных для GCP</td>
      <td>Развертываний, собственных для Azure</td>
    </tr>

    <tr>
      <td>Выставление счетов</td>
      <td><strong>Teams:</strong> \$150/место (Premium) с доступной оплатой по мере использования<br /><strong>Enterprise:</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">Свяжитесь с отделом продаж</a></td>
      <td>Оплата по мере использования</td>
      <td>Оплата по мере использования через AWS</td>
      <td>Оплата по мере использования через AWS Marketplace</td>
      <td>Оплата по мере использования через GCP</td>
      <td>Оплата по мере использования через Azure</td>
    </tr>

    <tr>
      <td>Регионы</td>
      <td>Поддерживаемые [страны](https://www.anthropic.com/supported-countries)</td>
      <td>Поддерживаемые [страны](https://www.anthropic.com/supported-countries)</td>
      <td>Несколько AWS [регионов](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html)</td>
      <td>Несколько регионов AWS</td>
      <td>Несколько GCP [регионов](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)</td>
      <td>Несколько Azure [регионов](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/)</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>Включено по умолчанию</td>
      <td>Включено по умолчанию</td>
      <td>Включено по умолчанию</td>
      <td>Включено по умолчанию</td>
      <td>Включено по умолчанию</td>
      <td>Включено по умолчанию</td>
    </tr>

    <tr>
      <td>Аутентификация</td>
      <td>Claude.ai SSO или электронная почта</td>
      <td>API ключ</td>
      <td>API ключ или учетные данные AWS</td>
      <td>API ключ или учетные данные AWS</td>
      <td>Учетные данные GCP</td>
      <td>API ключ или Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>Отслеживание затрат</td>
      <td>Панель использования</td>
      <td>Панель использования</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>GCP Billing</td>
      <td>Azure Cost Management</td>
    </tr>

    <tr>
      <td>Включает Claude в веб-версии</td>
      <td>Да</td>
      <td>Нет</td>
      <td>Нет</td>
      <td>Нет</td>
      <td>Нет</td>
      <td>Нет</td>
    </tr>

    <tr>
      <td>Функции Enterprise</td>
      <td>Управление командой, SSO, мониторинг использования</td>
      <td>Нет</td>
      <td>Политики IAM, CloudTrail</td>
      <td>Политики IAM, CloudTrail</td>
      <td>Роли IAM, Cloud Audit Logs</td>
      <td>Политики RBAC, Azure Monitor</td>
    </tr>
  </tbody>
</table>

Для подробного сравнения функций, доступных в каждом варианте, см. [Доступность функций](/docs/ru/feature-availability).

Выберите вариант развертывания для просмотра инструкций по настройке:

* [Claude for Teams или Enterprise](/docs/ru/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/ru/authentication#claude-console-authentication)
* [Claude apps gateway](/docs/ru/claude-apps-gateway), самостоятельно размещаемый шлюз, который добавляет вход через IdP перед Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry или Anthropic API
* [Amazon Bedrock](/docs/ru/amazon-bedrock)
* [Claude Platform on AWS](/docs/ru/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai)
* [Microsoft Foundry](/docs/ru/microsoft-foundry)

Для Amazon Bedrock и Google Vertex AI вы также можете запустить `claude` и выбрать **3rd-party platform** в приглашении входа для запуска интерактивного мастера настройки.

<h2 id="configure-proxies-and-gateways">
  Настройка прокси и шлюзов
</h2>

Большинство организаций могут использовать поставщика облачных услуг напрямую без дополнительной конфигурации. Однако вам может потребоваться настроить корпоративный прокси или шлюз LLM, если ваша организация имеет специфические требования к сети или управлению. Это разные конфигурации, которые можно использовать вместе:

* **Корпоративный прокси**: маршрутизирует трафик через прокси HTTP/HTTPS. Используйте это, если ваша организация требует, чтобы весь исходящий трафик проходил через прокси-сервер для мониторинга безопасности, соответствия требованиям или обеспечения политики сети. Настройте с помощью переменных окружения `HTTPS_PROXY` или `HTTP_PROXY`. Узнайте больше в разделе [Конфигурация корпоративной сети](/docs/ru/network-config).
* **Шлюз LLM**: сервис, который находится между Claude Code и поставщиком облачных услуг для обработки аутентификации и маршрутизации. Используйте это, если вам нужно централизованное отслеживание использования между командами, пользовательское ограничение скорости или бюджеты, или централизованное управление аутентификацией. Настройте с помощью переменных окружения `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL` или `ANTHROPIC_FOUNDRY_BASE_URL`. Узнайте больше в разделе [Шлюзы LLM](/docs/ru/llm-gateway).

Следующие примеры показывают переменные окружения для установки в вашей оболочке или профиле оболочки (`.bashrc`, `.zshrc`). См. раздел [Параметры](/docs/ru/settings) для других методов конфигурации.

<h3 id="amazon-bedrock">
  Amazon Bedrock
</h3>

<Tabs>
  <Tab title="Корпоративный прокси">
    Маршрутизируйте трафик Amazon Bedrock через ваш корпоративный прокси, установив следующие [переменные окружения](/docs/ru/env-vars):

    ```bash theme={null}
    # Включить Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1
    export AWS_REGION=us-east-1

    # Настроить корпоративный прокси
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Шлюз LLM">
    Маршрутизируйте трафик Amazon Bedrock через ваш шлюз LLM, установив следующие [переменные окружения](/docs/ru/env-vars):

    ```bash theme={null}
    # Включить Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1

    # Настроить шлюз LLM
    export ANTHROPIC_BEDROCK_BASE_URL='https://your-llm-gateway.com/bedrock'
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1  # Если шлюз обрабатывает аутентификацию AWS
    ```
  </Tab>
</Tabs>

<h3 id="microsoft-foundry">
  Microsoft Foundry
</h3>

<Tabs>
  <Tab title="Корпоративный прокси">
    Маршрутизируйте трафик Microsoft Foundry через ваш корпоративный прокси, установив следующие [переменные окружения](/docs/ru/env-vars):

    ```bash theme={null}
    # Включить Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1
    export ANTHROPIC_FOUNDRY_RESOURCE=your-resource
    export ANTHROPIC_FOUNDRY_API_KEY=your-api-key  # Или опустите для аутентификации Entra ID

    # Настроить корпоративный прокси
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Шлюз LLM">
    Маршрутизируйте трафик Microsoft Foundry через ваш шлюз LLM, установив следующие [переменные окружения](/docs/ru/env-vars):

    ```bash theme={null}
    # Включить Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1

    # Настроить шлюз LLM
    export ANTHROPIC_FOUNDRY_BASE_URL='https://your-llm-gateway.com'
    export ANTHROPIC_FOUNDRY_API_KEY=your-gateway-key  # Отправляется как x-api-key
    ```
  </Tab>
</Tabs>

<h3 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h3>

<Tabs>
  <Tab title="Корпоративный прокси">
    Маршрутизируйте трафик Google Cloud's Agent Platform через ваш корпоративный прокси, установив следующие [переменные окружения](/docs/ru/env-vars):

    ```bash theme={null}
    # Включить Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id

    # Настроить корпоративный прокси
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="Шлюз LLM">
    Маршрутизируйте трафик Google Cloud's Agent Platform через ваш шлюз LLM, установив следующие [переменные окружения](/docs/ru/env-vars):

    ```bash theme={null}
    # Включить Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1

    # Настроить шлюз LLM
    export ANTHROPIC_VERTEX_BASE_URL='https://your-llm-gateway.com/vertex'
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1  # Если шлюз обрабатывает аутентификацию GCP
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>
</Tabs>

<Tip>
  Используйте `/status` в Claude Code для проверки того, что конфигурация прокси и шлюза применена правильно. Например, с конфигурацией шлюза Bedrock выше, вывод включает строки вроде:

  ```
  API provider: Amazon Bedrock
  Bedrock base URL: https://your-llm-gateway.com/bedrock
  AWS region: us-east-1
  AWS auth skipped
  ```

  Если вы настроили корпоративный прокси, `/status` также показывает строку `Proxy` с URL вашего прокси.
</Tip>

<h2 id="best-practices-for-organizations">
  Лучшие практики для организаций
</h2>

<h3 id="invest-in-documentation-and-memory">
  Инвестируйте в документацию и память
</h3>

Мы настоятельно рекомендуем инвестировать в документацию, чтобы Claude Code понимал вашу кодовую базу. Организации могут развертывать файлы CLAUDE.md на нескольких уровнях:

* **На уровне организации**: развертывайте в системные каталоги, такие как `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS), `/etc/claude-code/CLAUDE.md` (Linux и WSL) или `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) для стандартов компании
* **На уровне репозитория**: создавайте файлы `CLAUDE.md` в корнях репозиториев, содержащие архитектуру проекта, команды сборки и рекомендации по внесению вклада. Проверяйте их в систему контроля версий, чтобы все пользователи получали выгоду

Узнайте больше в разделе [Память и файлы CLAUDE.md](/docs/ru/memory).

<h3 id="simplify-deployment">
  Упростите развертывание
</h3>

Если у вас есть пользовательская среда разработки, мы считаем, что создание "одноклик" способа установки Claude Code является ключом к расширению внедрения в организации.

<h3 id="start-with-guided-usage">
  Начните с управляемого использования
</h3>

Поощряйте новых пользователей попробовать Claude Code для вопросов и ответов по кодовой базе, или на небольших исправлениях ошибок или запросах функций. Попросите Claude Code составить план. Проверьте предложения Claude и дайте обратную связь, если что-то не так. Со временем, по мере того как пользователи лучше поймут эту новую парадигму, они будут более эффективны в том, чтобы позволить Claude Code работать более агентивно.

<h3 id="pin-model-versions-for-cloud-providers">
  Закрепите версии моделей для поставщиков облачных услуг
</h3>

Если вы развертываете через [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai), [Microsoft Foundry](/docs/ru/microsoft-foundry) или [Claude Platform on AWS](/docs/ru/claude-platform-on-aws), закрепите конкретные версии моделей, используя `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` и `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Без закрепления, псевдонимы моделей разрешаются на встроенное значение по умолчанию Claude Code для этого поставщика, что может отставать от последнего выпуска и может быть еще не включено в вашей учетной записи. Закрепление позволяет вам контролировать, когда ваши пользователи переходят на новую модель. См. раздел [Конфигурация модели](/docs/ru/model-config#pin-models-for-third-party-deployments) для получения информации о том, что делает каждый поставщик, когда значение по умолчанию недоступно.

<h3 id="configure-security-policies">
  Настройте политики безопасности
</h3>

Команды безопасности могут настроить управляемые разрешения для того, что Claude Code может и не может делать, которые не могут быть переопределены локальной конфигурацией. [Узнайте больше](/docs/ru/security).

<h3 id="leverage-mcp-for-integrations">
  Используйте MCP для интеграций
</h3>

MCP — это отличный способ предоставить Claude Code больше информации, такую как подключение к системам управления билетами или журналам ошибок. Мы рекомендуем, чтобы одна центральная команда настроила MCP servers и проверила конфигурацию `.mcp.json` в кодовую базу, чтобы все пользователи получали выгоду. [Узнайте больше](/docs/ru/mcp).

В Anthropic мы доверяем Claude Code для питания разработки во всех кодовых базах Anthropic. Мы надеемся, что вам понравится использовать Claude Code так же, как и нам.

<h2 id="next-steps">
  Следующие шаги
</h2>

После того как вы выбрали вариант развертывания и настроили доступ для вашей команды:

1. **Развертывание в вашей команде**: поделитесь инструкциями по установке и попросите членов команды [установить Claude Code](/docs/ru/setup) и аутентифицироваться с помощью своих учетных данных.
2. **Настройте общую конфигурацию**: создайте [файл CLAUDE.md](/docs/ru/memory) в ваших репозиториях, чтобы помочь Claude Code понять вашу кодовую базу и стандарты кодирования.
3. **Настройте разрешения**: просмотрите [параметры безопасности](/docs/ru/security) для определения того, что Claude Code может и не может делать в вашей среде.
