> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Настройка Claude Code для вашей организации

> Карта решений для администраторов, развертывающих Claude Code, охватывающая поставщиков API, управляемые параметры, принудительное применение политики, мониторинг использования и обработку данных.

Claude Code обеспечивает соблюдение политики организации через управляемые параметры, которые имеют приоритет над локальной конфигурацией разработчика. Вы доставляете эти параметры из консоли администратора Claude, вашей системы управления мобильными устройствами (MDM) или файла на диске. Параметры контролируют, какие инструменты, команды, серверы и сетевые назначения может достичь Claude.

На этой странице рассматриваются решения по развертыванию по порядку. Каждая строка ссылается на раздел ниже и на справочную страницу для этой области.

<Note>
  SSO, подготовка SCIM и назначение мест настраиваются на уровне учетной записи Claude. См. [Руководство администратора Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) и [назначение мест](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) для этих шагов.
</Note>

| Решение                                                                         | Что вы выбираете                                                | Справка                                                                                                                                                                       |
| :------------------------------------------------------------------------------ | :-------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Выберите поставщика API](#choose-your-api-provider)                            | Где Claude Code аутентифицируется и как это выставляется счетом | [Authentication](/docs/ru/authentication), [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai), [Microsoft Foundry](/docs/ru/microsoft-foundry) |
| [Решите, как параметры достигают устройств](#decide-how-settings-reach-devices) | Как управляемая политика достигает машин разработчиков          | [Server-managed settings](/docs/ru/server-managed-settings), [Settings files](/docs/ru/settings#settings-files)                                                                         |
| [Решите, что принудительно применять](#decide-what-to-enforce)                  | Какие инструменты, команды и интеграции разрешены               | [Permissions](/docs/ru/permissions), [Sandboxing](/docs/ru/sandboxing)                                                                                                                  |
| [Настройте видимость использования](#set-up-usage-visibility)                   | Как вы отслеживаете расходы и внедрение                         | [Analytics](/docs/ru/analytics), [Monitoring](/docs/ru/monitoring-usage), [Costs](/docs/ru/costs)                                                                                            |
| [Проверьте обработку данных](#review-data-handling)                             | Хранение данных и статус соответствия                           | [Data usage](/docs/ru/data-usage), [Security](/docs/ru/security)                                                                                                                        |

<h2 id="choose-your-api-provider">
  Выберите поставщика API
</h2>

Claude Code подключается к Claude через одного из нескольких поставщиков API. Ваш выбор влияет на выставление счетов, аутентификацию, какой статус соответствия вы наследуете и какие функции Claude Code доступны вашим разработчикам.

| Поставщик                     | Выберите это, когда                                                                                                                      |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| Claude for Teams / Enterprise | Вы хотите Claude Code и claude.ai в одной подписке на одного пользователя без инфраструктуры для запуска. Это рекомендация по умолчанию. |
| Claude Console                | Вы ориентированы на API или хотите выставление счетов по мере использования                                                              |
| Amazon Bedrock                | Вы хотите наследовать существующие элементы управления соответствием AWS и выставление счетов                                            |
| Google Cloud's Agent Platform | Вы хотите наследовать существующие элементы управления соответствием GCP и выставление счетов                                            |
| Microsoft Foundry             | Вы хотите наследовать существующие элементы управления соответствием Azure и выставление счетов                                          |

Некоторые функции Claude Code требуют учетной записи claude.ai. [Claude Code on the web](/docs/ru/claude-code-on-the-web), [Routines](/docs/ru/routines), [Code Review](/docs/ru/code-review), [Remote Control](/docs/ru/remote-control) и [Chrome extension](/docs/ru/chrome) недоступны только через ключи Console API или учетные данные поставщика облачных услуг. Если вы развертываете через Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry, спланируйте, нужны ли разработчикам также места Claude for Teams или Enterprise. На каждой странице функции указаны требования к плану.

Для полного сравнения поставщиков, охватывающего аутентификацию, регионы и паритет функций, см. [обзор развертывания предприятия](/docs/ru/third-party-integrations). Настройка аутентификации каждого поставщика находится в [Authentication](/docs/ru/authentication).

Требования прокси и брандмауэра в [Network configuration](/docs/ru/network-config) применяются независимо от поставщика. Если вы хотите единую конечную точку перед несколькими поставщиками или централизованное логирование запросов, см. [LLM gateway](/docs/ru/llm-gateway).

<h2 id="decide-how-settings-reach-devices">
  Решите, как параметры достигают устройств
</h2>

Управляемые параметры определяют политику, которая имеет приоритет над локальной конфигурацией разработчика. Claude Code проверяет четыре источника ниже в порядке приоритета и применяет первый, который возвращает непустую конфигурацию, с одним исключением: небольшой набор [ключей блокировки между источниками](/docs/ru/settings#settings-precedence), таких как блокировки списка разрешений sandbox, соблюдается, когда любой управляемый администратором источник их устанавливает.

| Механизм                | Доставка                                                                                                                                                                                            | Приоритет  | Платформы      |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------- | :------------- |
| Server-managed          | Консоль администратора claude.ai или самостоятельно размещенный [Claude apps gateway](/docs/ru/claude-apps-gateway) для входов через gateway                                                             | Наивысший  | Все            |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                           | Высокий    | macOS, Windows |
| File-based managed      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux и WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | Средний    | Все            |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                 | Наименьший | Только Windows |

Настроенный [`policyHelper`](/docs/ru/settings#compute-managed-settings-with-a-policy-helper) имеет приоритет над всеми четырьмя источниками: его вывод становится единственной управляемой конфигурацией для запуска. См. [Settings precedence](/docs/ru/settings#settings-precedence).

Server-managed параметры достигают устройств во время аутентификации и обновляются каждый час во время активных сеансов без инфраструктуры конечной точки. Доставка через консоль администратора claude.ai требует плана Claude for Teams или Enterprise. Развертывания на Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry могут получить ту же удаленную доставку, запустив [Claude apps gateway](/docs/ru/claude-apps-gateway), или вместо этого использовать один из механизмов на основе файлов или уровня ОС.

Если ваша организация смешивает поставщиков, настройте [server-managed settings](/docs/ru/server-managed-settings) для пользователей claude.ai плюс [резервный вариант на основе файлов или plist/registry](/docs/ru/settings#settings-files), чтобы другие пользователи по-прежнему получали управляемую политику.

Расположения plist и HKLM registry работают с любым поставщиком и устойчивы к несанкционированному доступу, потому что требуют привилегий администратора для записи. Реестр пользователя Windows в HKCU доступен для записи без повышения прав, поэтому рассматривайте его как удобный вариант по умолчанию, а не как канал принудительного применения.

По умолчанию WSL читает только путь Linux в `/etc/claude-code`. Чтобы расширить вашу политику реестра Windows и `C:\Program Files\ClaudeCode` на WSL на одной машине, установите [`wslInheritsWindowsSettings: true`](/docs/ru/settings#available-settings) в одном из этих источников, доступных только администратору Windows.

Какой бы механизм вы ни выбрали, управляемые значения имеют приоритет над параметрами пользователя и проекта. Параметры массива, такие как `permissions.allow` и `permissions.deny`, объединяют записи из всех источников, поэтому разработчики могут расширять управляемые списки, но не удалять из них. Для [двух исключений](/docs/ru/settings#settings-precedence), `fallbackModel` и `availableModels`, управляемое значение заменяет нижние слои, а не объединяется.

См. [Server-managed settings](/docs/ru/server-managed-settings) и [Settings files and precedence](/docs/ru/settings#settings-files).

<h3 id="wsl-sessions-in-claude-code-desktop">
  WSL сеансы в Claude Code Desktop
</h3>

На Windows [Claude Code Desktop может запускать сеансы Code внутри распределения WSL 2](/docs/ru/desktop-wsl). Процесс Claude Code сеанса работает внутри распределения, поэтому он разрешает управляемые параметры через путь обнаружения WSL выше: источники, доступные только Windows, не достигают его, если `wslInheritsWindowsSettings: true` не развернут.

На устройствах, где присутствуют управляемые параметры, сеансы Desktop WSL недоступны по умолчанию. Если ваша организация хочет их включить, свяжитесь с командой вашего аккаунта Anthropic. Когда они включены:

* Разверните `wslInheritsWindowsSettings: true` через реестр HKLM или файл `C:\Program Files\ClaudeCode`, чтобы сеансы WSL наследовали ту же политику, что и сеансы хоста.
* Проверьте, запустив `/status` внутри сеанса WSL: строка `Setting sources` должна показывать `Enterprise managed settings` с источником Windows, который вы развернули, `(HKLM)` или `(file)`.

Процессы внутри виртуальной машины WSL 2 не видны датчикам обнаружения конечных точек на стороне Windows. Если вы используете CrowdStrike Falcon, включите датчик Falcon для Linux на WSL 2 с двумя исключениями, которые требует документация WSL CrowdStrike, для процесса виртуальной машины WSL и образа диска ВМ, чтобы активность процесса и файла в дистрибутиве была видна. [OpenTelemetry телеметрия выполнения инструментов](/docs/ru/monitoring-usage) Claude Code излучается одинаково для сеансов WSL и собственных сеансов.

<h2 id="decide-what-to-enforce">
  Решите, что принудительно применять
</h2>

Управляемые параметры могут заблокировать инструменты, изоляцию песочницы, ограничить серверы MCP и источники плагинов, а также контролировать, какие hooks запускаются. Каждая строка — это поверхность управления с ключами параметров, которые ее управляют.

| Управление                                                                             | Что оно делает                                                                                                                                                                                                                                                                                          | Ключевые параметры                                                                                             |
| :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------- |
| [Permission rules](/docs/ru/permissions)                                                    | Разрешить, спросить или запретить определенные инструменты и команды                                                                                                                                                                                                                                    | `permissions.allow`, `permissions.deny`                                                                        |
| [Permission lockdown](/docs/ru/permissions#managed-only-settings)                           | Применяются только управляемые правила разрешений; отключить `--dangerously-skip-permissions`                                                                                                                                                                                                           | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                  |
| [Sandboxing](/docs/ru/sandboxing)                                                           | Изоляция файловой системы и сети на уровне ОС с разрешенными списками доменов                                                                                                                                                                                                                           | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                            |
| [Managed policy CLAUDE.md](/docs/ru/memory#deploy-organization-wide-claude-md)              | Инструкции на уровне организации, загруженные в каждый сеанс, не могут быть исключены                                                                                                                                                                                                                   | Файл по пути управляемой политики                                                                              |
| [MCP server control](/docs/ru/managed-mcp)                                                  | Ограничить, какие серверы MCP пользователи могут добавлять или подключать, или развернуть фиксированный набор                                                                                                                                                                                           | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly`, или развернутый файл `managed-mcp.json` |
| [Plugin marketplace control](/docs/ru/plugin-marketplaces#managed-marketplace-restrictions) | Ограничить, какие источники маркетплейса пользователи могут добавлять и устанавливать, отклонить флаги CLI, которые загружают плагины, агентов и серверы MCP для одного запуска, и разрешить список, какие плагины маркетплейсов могут быть предложены                                                  | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`, `pluginSuggestionMarketplaces`       |
| [Customization lockdown](/docs/ru/settings#strictpluginonlycustomization)                   | Заблокировать skills, agents, hooks и серверы MCP из источников пользователя и проекта, чтобы они могли поступать только из плагинов или управляемых параметров                                                                                                                                         | `strictPluginOnlyCustomization`                                                                                |
| [Hook restrictions](/docs/ru/settings#hook-configuration)                                   | Загружаются только управляемые hooks; ограничить URL-адреса HTTP hook                                                                                                                                                                                                                                   | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                 |
| [Login enforcement](/docs/ru/settings#available-settings)                                   | Ограничить интерактивный вход на определенный метод или организацию Anthropic. Когда установлено, сеансы, аутентифицированные с помощью `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper`, блокируются при запуске; сеансы поставщиков облачных услуг не затрагиваются                     | `forceLoginMethod`, `forceLoginOrgUUID`                                                                        |
| [Disable agent view](/docs/ru/agent-view#how-background-sessions-are-hosted)                | Отключить `claude agents`, `--bg`, `/background` и встроенного супервизора по требованию                                                                                                                                                                                                                | `disableAgentView`                                                                                             |
| [Model restrictions](/docs/ru/model-config#restrict-model-selection)                        | `availableModels` фильтрует, какие модели отображаются в средстве выбора. Добавление `enforceAvailableModels` также ограничивает автоматически выбранную модель по умолчанию. См. [surface coverage](/docs/ru/model-config#surface-coverage) для того, как этот параметр достигает CLI, веб-интерфейса и IDE | `availableModels`, `enforceAvailableModels`                                                                    |
| [Version floor](/docs/ru/settings)                                                          | Предотвратить автоматическое обновление от установки ниже минимума на уровне организации                                                                                                                                                                                                                | `minimumVersion`                                                                                               |
| [Required version range](/docs/ru/settings)                                                 | Отказать в запуске полностью, когда запущенная версия находится вне одобренного организацией диапазона. Более строгий, чем `minimumVersion`, который только блокирует понижение версии                                                                                                                  | `requiredMinimumVersion`, `requiredMaximumVersion`                                                             |

Организации, члены которых проходят аутентификацию через claude.ai или Anthropic API, также могут управлять моделями без развертывания параметров: [ограничения модели организации](/docs/ru/model-config#organization-model-restrictions) отключают отдельные модели, [модель по умолчанию организации](/docs/ru/model-config#organization-default-model) устанавливает, на какой модели начинаются новые сеансы, и [ограничения усилий организации](/docs/ru/model-config#organization-effort-limits) ограничивают уровни усилий по ролям. Все три элемента управления требуют плана Claude Enterprise. Ограничения модели и ограничения усилий применяются на стороне сервера; модель по умолчанию — это отправная точка, которую пользователи могут изменить, если только организация не применяет ее принудительно. Принудительное применение доступно для ограниченного набора организаций; обратитесь к команде вашего аккаунта Anthropic о доступности. Ни один из этих элементов управления не достигает сеансов на Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или [Claude Platform on AWS](/docs/ru/claude-platform-on-aws); на этих поставщиках используйте `availableModels` выше для ограничений и ключ `model` в управляемых параметрах для значения по умолчанию.

[Claude Code on the web](/docs/ru/claude-code-on-the-web) имеет собственную поверхность администратора: на странице облачных сред в параметрах администратора владельцы и администраторы создают [организационные общие среды](/docs/ru/claude-code-on-the-web#organization-shared-environments), которые устанавливают [уровень сетевого доступа](/docs/ru/claude-code-on-the-web#network-access), переменные окружения и скрипт настройки для облачных сеансов членов, а также выбирают среду организации по умолчанию.

Правила разрешений и песочница охватывают разные слои. Запрет WebFetch блокирует инструмент fetch Claude, но если Bash разрешен, `curl` и `wget` все еще могут достичь любого URL-адреса. Песочница закрывает этот пробел с разрешенным списком сетевых доменов, принудительно применяемым на уровне ОС.

Для модели угроз, которую защищают эти элементы управления, см. [Security](/docs/ru/security).

<h2 id="set-up-usage-visibility">
  Настройте видимость использования
</h2>

Выберите мониторинг на основе того, что вам нужно сообщить. Панели управления, API и элементы управления расходами различаются между планами Claude for Teams или Enterprise и организациями Claude Console, поэтому перед планированием отчётности проверьте столбец «Доступность».

| Возможность            | Что вы получаете                                                                                                                         | Доступность                                                                                                                                                                                                                                                                                                        | С чего начать                                         |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| Usage monitoring       | Экспорт OpenTelemetry сеансов, инструментов и токенов                                                                                    | Все поставщики                                                                                                                                                                                                                                                                                                     | [Monitoring usage](/docs/ru/monitoring-usage)              |
| Analytics dashboard    | Метрики принятия и вклада с таблицей лидеров на Teams / Enterprise; метрики использования и расходов для каждого пользователя на Console | Teams / Enterprise на [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console на [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                                                                                                        | [Analytics](/docs/ru/analytics)                            |
| Programmatic reporting | Данные об использовании и затратах для каждого пользователя через API                                                                    | [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) для Enterprise, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) для Console                                                                                       | [Costs](/docs/ru/costs#manage-costs-for-your-organization) |
| Spend controls         | Ограничения расходов и ограничения скорости                                                                                              | Параметры администратора для Teams / Enterprise, ограничения рабочей области для Console; на облаках третьих сторон, элементы управления бюджетом облака или [Claude apps gateway](/docs/ru/claude-apps-gateway) с ограничениями расходов для каждого пользователя [spend limits](/docs/ru/claude-apps-gateway-spend-limits) | [Costs](/docs/ru/costs#manage-costs-for-your-organization) |

На Teams и Enterprise данные об использовании и расходах для каждого пользователя поступают из [отчёта о расходах](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) в параметрах аналитики вашей организации, а не из панели аналитики. Облачные поставщики раскрывают расходы через AWS Cost Explorer, GCP Billing или Azure Cost Management. Для планирования корпоративных бюджетов в Claude chat, Claude Code и Cowork см. [руководство по потреблению Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide).

<h2 id="review-data-handling">
  Проверьте обработку данных
</h2>

На планах Team, Enterprise, Claude API и облачных поставщиков Anthropic не обучает модели на вашем коде или подсказках. Ваш поставщик API определяет хранение и статус соответствия.

| Тема                      | Что нужно знать                                                                          | С чего начать                                  |
| :------------------------ | :--------------------------------------------------------------------------------------- | :--------------------------------------------- |
| Data usage policy         | Что собирает Anthropic, как долго это хранится, что никогда не используется для обучения | [Data usage](/docs/ru/data-usage)                   |
| Zero Data Retention (ZDR) | Ничего не хранится после завершения запроса. Доступно на Claude for Enterprise           | [Zero data retention](/docs/ru/zero-data-retention) |
| Security architecture     | Сетевая модель, шифрование, аутентификация, журнал аудита                                | [Security](/docs/ru/security)                       |

Если вам нужно логирование аудита на уровне запроса или маршрутизация трафика по чувствительности данных, поместите шлюз между разработчиками и вашим поставщиком: самостоятельно размещённый [Claude apps gateway](/docs/ru/claude-apps-gateway) записывает журнал аудита для каждого запроса с идентификацией IdP, или используйте другой [LLM gateway](/docs/ru/llm-gateway). Для нормативных требований и сертификаций см. [Legal and compliance](/docs/ru/legal-and-compliance).

<h2 id="verify-and-onboard">
  Проверьте и подключите
</h2>

После настройки управляемых параметров попросите разработчика запустить `/status` внутри Claude Code. На вкладке **Status** строка `Setting sources` показывает `Enterprise managed settings`, за которой следует источник в скобках, один из `(remote)`, `(plist)`, `(HKLM)`, `(HKCU)` или `(file)`. См. [Проверка активных параметров](/docs/ru/settings#verify-active-settings).

Поделитесь этими ресурсами, чтобы помочь разработчикам начать работу:

* [Quickstart](/docs/ru/quickstart): пошаговое руководство первого сеанса от установки до работы с проектом
* [Common workflows](/docs/ru/common-workflows): шаблоны для повседневных задач, таких как проверка кода, рефакторинг и отладка
* [Claude 101](https://anthropic.skilljar.com/claude-101) и [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action): самостоятельные курсы Anthropic Academy

Для проблем с входом направьте разработчиков на [troubleshooting аутентификации](/docs/ru/troubleshoot-install#login-and-authentication). Наиболее распространенные исправления:

* Запустите `/logout`, а затем `/login` для переключения учетных записей
* Запустите `claude update`, если отсутствует опция аутентификации предприятия
* Перезагрузите терминал после обновления

Если разработчик видит "You haven't been added to your organization yet," его место не включает доступ к Claude Code и должно быть обновлено в консоли администратора.

<h2 id="next-steps">
  Следующие шаги
</h2>

После выбора поставщика и механизма доставки переходите к подробной конфигурации:

* [Server-managed settings](/docs/ru/server-managed-settings): доставка управляемой политики из консоли администратора Claude
* [Settings reference](/docs/ru/settings): каждый ключ параметра, расположение файла и правило приоритета
* [Monorepos and large repos](/docs/ru/large-codebases): шаблоны конфигурации для каждого каталога для организаций, развертывающих в монорепозиторий
* [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai), [Microsoft Foundry](/docs/ru/microsoft-foundry): развертывание для конкретного поставщика
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, управление местами и сценарий развертывания
