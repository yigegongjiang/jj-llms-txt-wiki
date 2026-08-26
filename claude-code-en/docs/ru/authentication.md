> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Аутентификация

> Войдите в Claude Code и настройте аутентификацию для отдельных пользователей, команд и организаций.

Claude Code поддерживает несколько методов аутентификации в зависимости от вашей конфигурации. Отдельные пользователи могут войти с помощью учетной записи Claude.ai, а команды могут использовать Claude for Teams или Enterprise, Claude Console или облачного провайдера, такого как Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Вход в Claude Code
</h2>

После [установки Claude Code](/docs/ru/setup#install-claude-code) запустите `claude` в вашем терминале. При первом запуске Claude Code откроет окно браузера для входа.

Если браузер не откроется автоматически, нажмите `c`, чтобы скопировать URL входа в буфер обмена, а затем вставьте его в браузер.

Если ваш браузер показывает код входа вместо перенаправления после входа, вставьте его в терминал в приглашение `Paste code here if prompted`. Это происходит, когда браузер не может достичь локального сервера обратного вызова Claude Code, что часто встречается в WSL2, сеансах SSH и контейнерах.

Когда вход завершится, терминал отобразит `Login successful` и предложит вам нажать `Enter` для продолжения.

Вы можете аутентифицироваться с помощью любого из этих типов учетных записей:

* **Подписка Claude Pro или Max**: войдите с помощью вашей учетной записи Claude.ai. Подпишитесь на [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams или Enterprise**: войдите с помощью учетной записи Claude.ai, на которую вас пригласил администратор вашей команды.
* **Claude Console**: войдите с помощью ваших учетных данных Console. Ваш администратор должен был [пригласить вас](#claude-console-authentication) предварительно.
* **Облачные провайдеры**: если ваша организация использует [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) или [Microsoft Foundry](/docs/ru/microsoft-foundry), установите необходимые переменные окружения перед запуском `claude`, или выберите **3rd-party platform** в приглашении входа, которое запускает интерактивный мастер настройки для Bedrock и Vertex AI. Вход через браузер не требуется.
* **Облачный шлюз**: если ваша организация запускает самостоятельно размещенный [шлюз приложений Claude](/docs/ru/claude-apps-gateway), войдите с помощью корпоративного SSO через `/login`. Токен, выданный шлюзом, является единственным учетным данием сеанса.

Администраторы могут ограничить интерактивный вход с помощью управляемых параметров [`forceLoginMethod` и `forceLoginOrgUUID`](/docs/ru/settings#available-settings). Когда установлен один из них, сеансы, аутентифицированные с помощью `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper`, блокируются при запуске; сеансы облачных провайдеров не затрагиваются.

Чтобы выйти и повторно аутентифицироваться, введите `/logout` в приглашение Claude Code. Выход также сбрасывает состояние первоначальной настройки, поэтому при следующем запуске `claude` вас проведут через вход и настройку снова.

Если у вас возникли проблемы с входом, см. [устранение неполадок аутентификации](/docs/ru/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Настройка аутентификации команды
</h2>

Для команд и организаций вы можете настроить доступ Claude Code одним из следующих способов:

* [Claude for Teams или Enterprise](#claude-for-teams-or-enterprise), рекомендуется для большинства команд
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/ru/claude-apps-gateway), самостоятельно размещаемый шлюз, который подписывает разработчиков с помощью вашего IdP и маршрутизирует вывод к облачному провайдеру, который вы настраиваете
* [Amazon Bedrock](/docs/ru/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai)
* [Microsoft Foundry](/docs/ru/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams или Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) и [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) обеспечивают лучший опыт для организаций, использующих Claude Code. Члены команды получают доступ как к Claude Code, так и к Claude в веб-версии с централизованным выставлением счетов и управлением командой.

* **Claude for Teams**: план самообслуживания с функциями сотрудничества, инструментами администратора и управлением выставлением счетов. Лучше всего подходит для небольших команд.
* **Claude for Enterprise**: добавляет SSO, захват домена, разрешения на основе ролей, API соответствия и управляемые параметры политики для конфигураций Claude Code на уровне организации. Лучше всего подходит для крупных организаций с требованиями безопасности и соответствия.

<Steps>
  <Step title="Подпишитесь">
    Подпишитесь на [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) или свяжитесь с отделом продаж для [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Пригласите членов команды">
    Пригласите членов команды из панели администратора.
  </Step>

  <Step title="Установите и войдите">
    Члены команды устанавливают Claude Code и входят с помощью своих учетных записей Claude.ai.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Аутентификация Claude Console
</h3>

Для организаций, которые предпочитают выставление счетов на основе API, вы можете настроить доступ через Claude Console.

<Steps>
  <Step title="Создайте или используйте учетную запись Console">
    Используйте существующую учетную запись Claude Console или создайте новую.
  </Step>

  <Step title="Добавьте пользователей">
    Вы можете добавлять пользователей любым из следующих способов:

    * Массовое приглашение пользователей из Console: Settings -> Members -> Invite
    * [Настройте SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="Назначьте роли">
    При приглашении пользователей назначьте одну из следующих ролей:

    * **Claude Code** роль: пользователи могут создавать только ключи API Claude Code
    * **Developer** роль: пользователи могут создавать любой вид ключа API
  </Step>

  <Step title="Пользователи завершают настройку">
    Каждый приглашенный пользователь должен:

    * Принять приглашение Console
    * [Проверить системные требования](/docs/ru/setup#system-requirements)
    * [Установить Claude Code](/docs/ru/setup#install-claude-code)
    * Войти с учетными данными учетной записи Console
  </Step>
</Steps>

<h3 id="cloud-provider-authentication">
  Аутентификация облачного провайдера
</h3>

Для команд, использующих Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry:

<Steps>
  <Step title="Следуйте настройке провайдера">
    Следуйте [документации Amazon Bedrock](/docs/ru/amazon-bedrock), [документации Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) или [документации Microsoft Foundry](/docs/ru/microsoft-foundry).
  </Step>

  <Step title="Распределите конфигурацию">
    Распределите переменные окружения и инструкции по созданию облачных учетных данных среди ваших пользователей. Узнайте больше о том, как [управлять конфигурацией здесь](/docs/ru/settings).
  </Step>

  <Step title="Установите Claude Code">
    Пользователи могут [установить Claude Code](/docs/ru/setup#install-claude-code).
  </Step>
</Steps>

<h2 id="credential-management">
  Управление учетными данными
</h2>

Claude Code безопасно управляет вашими учетными данными аутентификации:

* **Место хранения**:
  * На macOS учетные данные хранятся в зашифрованной цепочке ключей macOS Keychain.
  * На Linux учетные данные хранятся в `~/.claude/.credentials.json` с режимом файла `0600`.
  * На Windows учетные данные хранятся в `%USERPROFILE%\.claude\.credentials.json` и наследуют элементы управления доступом из каталога профиля пользователя, что по умолчанию ограничивает доступ к файлу вашей учетной записью.
  * Если вы установили переменную окружения `CLAUDE_CONFIG_DIR` на Linux или Windows, файл `.credentials.json` находится в этом каталоге.
  * Claude Code управляет `.credentials.json` через `/login` и `/logout`. Чтобы маршрутизировать запросы через пользовательскую конечную точку API, установите переменную окружения [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars).
* **Поддерживаемые типы аутентификации**: учетные данные Claude.ai, учетные данные API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth и токены сеанса [Claude apps gateway](/docs/ru/claude-apps-gateway).
* **Пользовательские скрипты учетных данных**: параметр [`apiKeyHelper`](/docs/ru/settings#available-settings) можно настроить для запуска скрипта оболочки, который возвращает ключ API.
* **Интервалы обновления**: по умолчанию `apiKeyHelper` вызывается через 5 минут или при ответе HTTP 401. Установите переменную окружения `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` для пользовательских интервалов обновления.
* **Уведомление о медленном помощнике**: если `apiKeyHelper` требует более 10 секунд для возврата ключа, Claude Code отображает предупреждающее уведомление в строке приглашения, показывающее прошедшее время. Если вы видите это уведомление регулярно, проверьте, можно ли оптимизировать ваш скрипт учетных данных.
* **Сбои помощника**: когда скрипт завершается с ошибкой, истекает время ожидания или не выводит ничего, запросы завершаются с ошибкой [`Your apiKeyHelper script is failing`](/docs/ru/errors#your-apikeyhelper-script-is-failing) в течение трех попыток. До версии 2.1.208 сбои помощника отображались как общая ошибка 401 после примерно десяти молчаливых повторных попыток.

`apiKeyHelper`, `ANTHROPIC_API_KEY` и `ANTHROPIC_AUTH_TOKEN` применяются к CLI и поверхностям, которые его оборачивают, включая расширение VS Code, Agent SDK и GitHub Actions. Claude Desktop и облачные сеансы не вызывают `apiKeyHelper` и не читают эти переменные окружения: они используют OAuth, за исключением сеансов рабочего стола, работающих с [конфигурацией вывода третьей стороны](/docs/ru/llm-gateway-connect#desktop-app), которые аутентифицируются с помощью учетных данных этой конфигурации.

<h3 id="renew-an-expiring-login">
  Обновление истекающего входа
</h3>

Когда вход, созданный с помощью `/login`, находится в пределах пяти дней до истечения срока действия, Claude Code показывает предупреждение при запуске: `Your login expires in 3 days · run /login to renew`. Требуется Claude Code v2.1.203 или позже.

Запустите `/login` для обновления. Предупреждение носит информационный характер и никогда не блокирует запрос: аутентификация продолжает работать до фактического истечения срока действия входа. Сам срок действия входа не изменяется; предварительное предупреждение — это то, что добавляет v2.1.203.

После истечения срока действия сохраненного входа и невозможности его обновления каждый запрос завершается с ошибкой [`Login expired · Please run /login`](/docs/ru/errors#login-expired) до тех пор, пока вы снова не войдете. До версии 2.1.206 истекший вход отображался как ошибка модели.

Предупреждение появляется только когда вход claude.ai или Claude Console является активным учетным данием, а не когда облачный провайдер, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper` предоставляет учетные данные.

Раннее обновление наиболее важно для сеансов, которые работают без присмотра. [Фоновый сеанс в представлении агента](/docs/ru/agent-view) или сеанс [Remote Control](/docs/ru/remote-control), который пережил вход, прекращает прогресс после истечения срока действия учетных данных и не может восстановиться, пока вы снова не войдете.

<h3 id="authentication-precedence">
  Приоритет аутентификации
</h3>

Когда присутствуют несколько учетных данных, Claude Code выбирает одно в этом порядке:

1. Учетные данные облачного провайдера, когда установлены `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` или `CLAUDE_CODE_USE_FOUNDRY`. См. [интеграции третьих сторон](/docs/ru/third-party-integrations) для настройки.
2. Переменная окружения `ANTHROPIC_AUTH_TOKEN`. Отправляется как заголовок `Authorization: Bearer`. Используйте это при маршрутизации через [шлюз LLM или прокси](/docs/ru/llm-gateway), который аутентифицируется с помощью токенов-носителей, а не ключей API Anthropic.
3. Переменная окружения `ANTHROPIC_API_KEY`. Отправляется как заголовок `X-Api-Key`. Используйте это для прямого доступа к API Anthropic с ключом из [Claude Console](https://platform.claude.com). В интерактивном режиме вам предлагается один раз одобрить или отклонить ключ, и ваш выбор запоминается. Чтобы изменить его позже, используйте переключатель "Use custom API key" в `/config`. Переключатель появляется только при установке `ANTHROPIC_API_KEY` в вашей среде. В неинтерактивном режиме (`-p`) ключ всегда используется при наличии.
4. Выход скрипта [`apiKeyHelper`](/docs/ru/settings#available-settings). Используйте это для динамических или ротирующихся учетных данных, таких как краткосрочные токены, полученные из хранилища.
5. Переменная окружения `CLAUDE_CODE_OAUTH_TOKEN`. Долгоживущий токен OAuth, созданный [`claude setup-token`](#generate-a-long-lived-token). Используйте это для конвейеров CI и скриптов, где вход через браузер недоступен.
6. Учетные данные OAuth подписки из `/login`. Это значение по умолчанию для пользователей Claude Pro, Max, Team и Enterprise.

Подписанный сеанс [Claude apps gateway](/docs/ru/claude-apps-gateway) находится вне этого списка: это выбор провайдера, как Amazon Bedrock или Google Cloud's Agent Platform, и он имеет приоритет над ними. Когда существует сеанс шлюза, CLI аутентифицируется с помощью токена шлюза, даже если установлены `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` или `CLAUDE_CODE_USE_FOUNDRY`, и записи токена-носителя, ключа API и `apiKeyHelper` выше не используются.

Если у вас есть активная подписка Claude, но также установлен `ANTHROPIC_API_KEY` в вашей среде, ключ API имеет приоритет после одобрения. Это может привести к сбоям аутентификации, если ключ принадлежит отключенной или истекшей организации. Запустите `unset ANTHROPIC_API_KEY`, чтобы вернуться к вашей подписке, и проверьте `/status`, чтобы подтвердить, какой метод активен. Строка `Login method` показывает вашу учетную запись подписки, и строка `API key` появляется при использовании ключа API.

[Claude Code в веб-версии](/docs/ru/claude-code-on-the-web) всегда использует учетные данные вашей подписки. Если вы установите `ANTHROPIC_API_KEY` или `ANTHROPIC_AUTH_TOKEN` в среде песочницы, это не переопределит учетные данные вашей подписки.

<h3 id="generate-a-long-lived-token">
  Создание долгоживущего токена
</h3>

Для конвейеров CI, скриптов или других сред, где интерактивный вход через браузер недоступен, создайте однолетний токен OAuth с помощью `claude setup-token`:

```bash theme={null}
claude setup-token
```

Команда проведет вас через авторизацию OAuth и выведет токен в терминал. Она не сохраняет токен нигде; скопируйте его и установите его как переменную окружения `CLAUDE_CODE_OAUTH_TOKEN` везде, где вы хотите аутентифицироваться:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Этот токен аутентифицируется с помощью вашей подписки Claude и требует план Pro, Max, Team или Enterprise. Он ограничен только выводом и не может устанавливать сеансы [Remote Control](/docs/ru/remote-control).

[Bare mode](/docs/ru/headless#start-faster-with-bare-mode) не читает `CLAUDE_CODE_OAUTH_TOKEN`. Если ваш скрипт передает `--bare`, аутентифицируйтесь с помощью `ANTHROPIC_API_KEY` или `apiKeyHelper` вместо этого.
