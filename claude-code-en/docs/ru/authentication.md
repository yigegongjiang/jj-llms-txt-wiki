> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Аутентификация

> Войдите в Claude Code и настройте аутентификацию для отдельных пользователей, команд и организаций.

Claude Code поддерживает несколько методов аутентификации в зависимости от вашей конфигурации. Отдельные пользователи могут войти с помощью учетной записи claude.ai, а команды могут использовать Claude for Teams или Enterprise, Claude Console или облачного провайдера, такого как Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Вход в Claude Code
</h2>

После [установки Claude Code](/docs/ru/setup#install-claude-code) запустите `claude` в вашем терминале. При первом запуске Claude Code откроет окно браузера для входа. Если вы установили переменную окружения `ANTHROPIC_API_KEY`, Claude Code пропустит приглашение входа и вместо этого попросит вас одобрить ключ.

Если браузер не откроется автоматически, нажмите `c`, чтобы скопировать URL входа в буфер обмена, а затем вставьте его в браузер.

Если ваш браузер показывает код входа вместо перенаправления после входа, вставьте его в терминал в приглашение `Paste code here if prompted`. Это происходит, когда браузер не может достичь локального сервера обратного вызова Claude Code, что часто встречается в WSL2, сеансах SSH и контейнерах.

Когда вход завершится, терминал отобразит `Login successful` и предложит вам нажать `Enter` для продолжения.

Вы можете аутентифицироваться с помощью любого из этих типов учетных записей:

* **Подписка Claude Pro или Max**: войдите с помощью вашей учетной записи claude.ai. Подпишитесь на [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams или Enterprise**: войдите с помощью учетной записи claude.ai, на которую вас пригласил администратор вашей команды.
* **Claude Console**: войдите с помощью ваших учетных данных Console. Ваш администратор должен был [пригласить вас](#claude-console-authentication) предварительно. Вы можете войти с или без [создания API ключа](#sign-in-without-an-api-key).
* **Облачные провайдеры**: если ваша организация использует [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) или [Microsoft Foundry](/docs/ru/microsoft-foundry), установите необходимые переменные окружения перед запуском `claude`, или выберите **3rd-party platform** в приглашении входа, которое запускает интерактивный мастер настройки для Bedrock и Vertex AI. Вход через браузер не требуется.
* **Облачный шлюз**: если ваша организация запускает самостоятельно размещенный [шлюз приложений Claude](/docs/ru/claude-apps-gateway), войдите с помощью корпоративного SSO через `/login`. Токен, выданный шлюзом, является единственным учетным данием сеанса.

Администраторы могут указать, какой метод входа используют разработчики, и требовать, чтобы входы claude.ai принадлежали определенной организации; см. [Ограничение входа для вашей организации](#restrict-login-to-your-organization).

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

* **Claude for Teams**: план самообслуживания с функциями сотрудничества, инструментами администратора, SSO, управлением выставлением счетов и [параметрами, управляемыми сервером](/docs/ru/server-managed-settings) для конфигурации Claude Code на уровне организации. Лучше всего подходит для небольших команд.
* **Claude for Enterprise**: добавляет захват домена, разрешения на основе ролей и API соответствия. Лучше всего подходит для крупных организаций с требованиями безопасности и соответствия.

<Steps>
  <Step title="Подпишитесь">
    Подпишитесь на [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) или свяжитесь с отделом продаж для [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Пригласите членов команды">
    Пригласите членов команды из панели администратора.
  </Step>

  <Step title="Установите и войдите">
    Члены команды устанавливают Claude Code и входят с помощью своих учетных записей claude.ai.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Claude Console authentication
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

<h4 id="sign-in-without-an-api-key">
  Sign in without an API key
</h4>

Вы можете войти в свою учетную запись Console без создания ключа API, даже если ваша организация не позволяет разработчикам их создавать. Выберите учетную запись Anthropic Console в приглашении `/login` и Claude Code спросит, как вы хотите войти. Требуется Claude Code v2.1.242 или позже. Оба маршрута подписывают вас в Console в браузере и отличаются тем, что Claude Code сохраняет впоследствии:

* **Вход с помощью вашей учетной записи Console**, помечено как `(рекомендуется)`: Claude Code сохраняет токен OAuth из этого входа и сохраняет его как [профиль Anthropic](#anthropic-profiles-and-federation-credentials). Он не создает ключ API
* **Создать ключ API**, помечено как `(устаревший)`: Claude Code создает для вас ключ API Console и сохраняет его с вашими другими учетными данными

На практике профиль хранит вход OAuth, а ключ API — это статические учетные данные: Claude Code автоматически обновляет вход профиля, и когда обновление не удается, запросы не выполняются с [истекшим входом профиля Anthropic](/docs/ru/errors#anthropic-profile-login-expired) до тех пор, пока вы снова не войдете.

Вы не получаете выбор на каждой машине. Claude Code создает ключ API без запроса в этих случаях:

* Вы работаете с облачным провайдером, таким как [Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry](/docs/ru/third-party-integrations) или [Claude Platform на AWS](/docs/ru/claude-platform-on-aws)
* Любой файл параметров устанавливает [`forceLoginOrgUUID`](#restrict-login-to-your-organization), или устанавливает `forceLoginMethod` на `"claudeai"` или `"console"`
* На вашей машине существует управляемый источник параметров, такой как файл управляемых параметров, профиль MDM или кэшированные параметры, управляемые сервером, но Claude Code [не может его прочитать](/docs/ru/managed-settings#invalid-entries-in-managed-settings) и никакой другой управляемый источник не предоставляет политику

Отмените установку `ANTHROPIC_API_KEY` перед входом без ключа. Профиль, написанный собственным входом Claude Code в Console, или входом Claude Platform CLI `ant auth login`, — это один и тот же вид учетных данных, поэтому повторный вход заменяет его.

После входа без ключа у вас есть профиль вместо сохраненного ключа API:

* **Какой профиль он записывает**: Claude Code записывает профиль, названный `ANTHROPIC_PROFILE`, или ваш активный профиль, или `default`. Если этот профиль является профилем федерации, Claude Code отказывает в входе вместо его перезаписи
* **Из чего он вас выходит**: Claude Code выходит из любого входа claude.ai, сохраненного на машине
* **Как это отменить**: запустите `/logout`, который удаляет и отзывает учетные данные, которые этот вход записал

Если ваша организация использует [параметры, управляемые сервером](/docs/ru/server-managed-settings), они применяются к этому входу на Claude Code v2.1.257 или позже.

Все остальное о профилях применяется к этому входу, включая его ранжирование по сравнению с вашими другими учетными данными, строку `Profile`, которую вы получаете в `/status`, и функции, которые требуют входа claude.ai. См. [Профили Anthropic и учетные данные федерации](#anthropic-profiles-and-federation-credentials).

<h3 id="cloud-provider-authentication">
  Cloud provider authentication
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

<h3 id="restrict-login-to-your-organization">
  Restrict login to your organization
</h3>

Чтобы требовать, чтобы входы claude.ai разработчиков принадлежали определенной организации Anthropic, установите [`forceLoginMethod`](/docs/ru/settings-reference#forceloginmethod) и [`forceLoginOrgUUID`](/docs/ru/settings-reference#forceloginorguuid) в [управляемых параметрах](/docs/ru/managed-settings). Установите `forceLoginOrgUUID` на ваш ID организации, показанный в [параметрах администратора claude.ai](https://claude.ai/admin-settings/organization) для организаций Claude for Teams или Enterprise. Claude Code сообщает об ошибке для входа claude.ai в любую другую организацию и выходит при запуске, если учетные данные claude.ai в использовании принадлежат организации, которая не указана в списке.

Для входов Claude Console, Claude Code использует `forceLoginOrgUUID` для предварительного выбора организации на странице входа Console, когда вы устанавливаете его на один ID организации Console, показанный на [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization). Он не проверяет, какой организации принадлежат полученные учетные данные Console, при входе или при запуске, и разработчик, который вошел с учетной записью Console до развертывания ключей, остается в системе.

Если вы установите `forceLoginOrgUUID` в любом файле параметров, Claude Code прекратит предлагать [вход в Console без ключа](#sign-in-without-an-api-key) в сеансах, к которым применяется этот файл, и вместо этого создаст ключ API. Чтобы направить разработчиков на вход claude.ai, установите `forceLoginMethod` на `"claudeai"`.

Разработчики могут входить несколькими путями: поток терминала `/login`, [расширение VS Code](/docs/ru/vs-code), Agent SDK, `claude setup-token`, `/install-github-app` и [вход через шлюз](/docs/ru/claude-apps-gateway) для организаций, которые маршрутизируют через облачный шлюз. На Claude Code v2.1.212 или позже каждый путь применяет `forceLoginMethod`; до v2.1.212 только входы терминала применяли любой ключ. На интерактивном экране входа терминала, доступном через `/login` или первоначальную настройку, Claude Code предварительно выбирает метод `claudeai` или `console` без его принудительного применения, поэтому даже с установленным `forceLoginMethod` на `"claudeai"`, разработчик все еще может завершить вход Console там. Пути отличаются на `forceLoginOrgUUID`:

* **Входы терминала, расширения VS Code и Agent SDK**: проверяют `forceLoginOrgUUID` для входов учетной записи claude.ai
* **`claude setup-token` и `/install-github-app`**: применяют только `forceLoginMethod`, поэтому они могут создать токен в другой организации
* **[Вход через шлюз](/docs/ru/claude-apps-gateway)**: выбран `forceLoginMethod: "gateway"` вместо ограничения им, и не аутентифицируется против организации Anthropic, поэтому `forceLoginOrgUUID` не применяется; используйте поставщика идентификации вашего шлюза для ограничения доступа

Развертывайте ключи через инструменты управления устройствами. [Параметры, управляемые сервером](/docs/ru/server-managed-settings), достигают только учетных записей, которые уже аутентифицированы в вашей организации, поэтому они не могут перенаправить первый вход разработчика. Если ваша организация также распределяет параметры, управляемые сервером, установите ключи в обоих местах: управляемые источники параметров [не объединяются](/docs/ru/server-managed-settings#settings-precedence), и кэшированные параметры, управляемые сервером, заменяют файл, управляемый устройством, за исключением нескольких [исключений для каждого ключа](/docs/ru/server-managed-settings#per-key-exceptions-across-managed-sources). `forceLoginOrgUUID` и значения `"claudeai"` и `"console"` `forceLoginMethod` не входят в эти исключения, поэтому держите их в обоих местах.

Ключи также определяют, может ли сеанс, который не использует учетные данные входа, начаться. См. [`forceLoginOrgUUID`](/docs/ru/settings-reference#forceloginorguuid) в справочнике параметров для полного поведения.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper`**: заблокированы при запуске, так как членство в организации не может быть проверено для учетных данных окружения
* **Сеансы облачного провайдера, такие как Amazon Bedrock**: не заблокированы, потому что они аутентифицируются против вашего облачного провайдера. Ограничьте их через ваши политики облачного IAM
* **[Профиль Anthropic или учетные данные федерации](#anthropic-profiles-and-federation-credentials)**: не заблокированы, и ключи не проверяют, какой организации принадлежит профиль

<h2 id="credential-management">
  Управление учетными данными
</h2>

Claude Code безопасно управляет вашими учетными данными аутентификации:

* **Место хранения**:
  * На macOS учетные данные хранятся в зашифрованной цепочке ключей macOS Keychain. Когда Keychain отклоняет запись, например когда она заблокирована в сеансе SSH, Claude Code вместо этого сохраняет ваш вход в `~/.claude/.credentials.json` с режимом файла `0600`, то же хранилище, которое он использует на Linux. Вход через Console, который создает ключ API, завершается ошибкой до тех пор, пока Keychain не станет доступным для записи. Чтобы переместить ваш вход обратно в Keychain, следуйте [шагам восстановления](/docs/ru/troubleshoot-install#not-logged-in-or-token-expired).
  * На Linux учетные данные хранятся в `~/.claude/.credentials.json` с режимом файла `0600`.
  * На Windows учетные данные хранятся в `%USERPROFILE%\.claude\.credentials.json` и наследуют элементы управления доступом из каталога профиля пользователя, что по умолчанию ограничивает доступ к файлу вашей учетной записью.
  * Если вы установили переменную окружения `CLAUDE_CONFIG_DIR`, Claude Code хранит файл `.credentials.json` в этом каталоге вместо этого, включая файл, который записывает резервный вариант macOS, и также привязывает запись macOS Keychain к этому каталогу, поэтому сеанс с другим `CLAUDE_CONFIG_DIR` читает другую запись.
  * Claude Code управляет `.credentials.json` через `/login` и `/logout`. Чтобы маршрутизировать запросы через пользовательскую конечную точку API, установите переменную окружения [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars).
* **Поддерживаемые типы аутентификации**: учетные данные claude.ai, учетные данные Claude API, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, учетные данные профиля Anthropic и [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), а также токены сеанса [Claude apps gateway](/docs/ru/claude-apps-gateway).
* **Пользовательские скрипты учетных данных**: настройте параметр [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) для запуска скрипта оболочки, который возвращает ключ API.
* **Интервалы обновления**: Claude Code повторно запускает `apiKeyHelper` через пять минут по умолчанию. Установите переменную окружения `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` для пользовательских интервалов обновления. Смотрите [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) для других случаев, в которых Claude Code повторно запускает помощник.
* **Уведомление о медленном помощнике**: если `apiKeyHelper` требует более 10 секунд для возврата ключа, Claude Code отображает предупреждающее уведомление в строке приглашения, показывающее прошедшее время. Если вы видите это уведомление регулярно, проверьте, можно ли оптимизировать ваш скрипт учетных данных.
* **Сбои помощника**: когда скрипт завершается с ошибкой, истекает время ожидания или не выводит ничего, запросы завершаются с ошибкой [`Your apiKeyHelper script is failing`](/docs/ru/errors#your-apikeyhelper-script-is-failing) в течение трех попыток. До версии 2.1.208 сбои помощника отображались как общая ошибка 401 после примерно десяти молчаливых повторных попыток.

`apiKeyHelper`, `ANTHROPIC_API_KEY` и `ANTHROPIC_AUTH_TOKEN` применяются к CLI и поверхностям, которые его оборачивают, включая расширение VS Code, Agent SDK и GitHub Actions. Claude Desktop и облачные сеансы не вызывают `apiKeyHelper` и не читают эти переменные окружения: они используют OAuth, за исключением сеансов рабочего стола, работающих с [конфигурацией вывода третьей стороны](/docs/ru/llm-gateway-connect#desktop-app), которые аутентифицируются с помощью учетных данных этой конфигурации.

<h3 id="renew-an-expiring-login">
  Обновление истекающего входа
</h3>

Когда вход, созданный с помощью `/login`, находится в пределах трех дней до истечения срока действия, Claude Code показывает предупреждение при запуске: `Your login expires in 3 days · run /login to renew`. Требуется Claude Code v2.1.203 или позже. До версии 2.1.217 предупреждение появлялось за пять дней.

Запустите `/login` для обновления. Предупреждение носит информационный характер и никогда не блокирует запрос: аутентификация продолжает работать до фактического истечения срока действия входа. Сам срок действия входа не изменяется; предварительное предупреждение — это то, что добавляет v2.1.203.

После истечения срока действия сохраненного входа и невозможности его обновления каждый запрос модели завершается с ошибкой [`Login expired · Please run /login`](/docs/ru/errors#login-expired) до тех пор, пока вы снова не войдете. До версии 2.1.206 Claude Code сообщал об истекшем входе при запросах модели как об ошибке модели.

Вы можете проверить это состояние перед тем, как запрос завершится ошибкой: [`/status`](/docs/ru/commands) показывает строку `Login`, читающую `Expired — log in again`, плюс организацию и адрес электронной почты, которые он сохранил для истекшего входа. Строка появляется только когда сохраненный вход claude.ai или Claude Console является активным учетным данием. Строка требует Claude Code v2.1.210 или позже.

Предупреждение появляется только когда вход claude.ai или Claude Console является активным учетным данием, а не когда облачный провайдер, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper` предоставляет учетные данные.

Раннее обновление наиболее важно для сеансов, которые работают без присмотра. [Фоновый сеанс в представлении агента](/docs/ru/agent-view) или сеанс [Remote Control](/docs/ru/remote-control), который пережил вход, прекращает прогресс после истечения срока действия учетных данных и не может восстановиться, пока вы снова не войдете.

<h3 id="authentication-precedence">
  Приоритет аутентификации
</h3>

Когда присутствуют несколько учетных данных, Claude Code выбирает одно в этом порядке:

1. Учетные данные облачного провайдера, когда установлены `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` или `CLAUDE_CODE_USE_FOUNDRY`. Смотрите [интеграции третьих сторон](/docs/ru/third-party-integrations) для настройки.
2. Переменная окружения `ANTHROPIC_AUTH_TOKEN`. Отправляется как заголовок `Authorization: Bearer`. Используйте это при маршрутизации через [шлюз LLM или прокси](/docs/ru/llm-gateway), который аутентифицируется с помощью токенов-носителей, а не ключей API Anthropic.
3. Переменная окружения `ANTHROPIC_API_KEY`. Отправляется как заголовок `X-Api-Key`. Используйте это для прямого доступа к API Anthropic с ключом из [Claude Console](https://platform.claude.com). В интерактивном режиме вам предлагается один раз одобрить или отклонить ключ, и ваш выбор запоминается. Чтобы изменить его позже, используйте переключатель "Use custom API key" в `/config`. Переключатель появляется только при установке `ANTHROPIC_API_KEY` в вашей среде. В неинтерактивном режиме (`-p`) ключ всегда используется при наличии.
4. Выход скрипта [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper). Используйте это для динамических или ротирующихся учетных данных, таких как краткосрочные токены, полученные из хранилища.
5. Переменная окружения `CLAUDE_CODE_OAUTH_TOKEN`. Долгоживущий токен OAuth, созданный [`claude setup-token`](#generate-a-long-lived-token). Используйте это для конвейеров CI и скриптов, где вход через браузер недоступен. Если вы запустите `/login` при установленной переменной, Claude Code переключит текущий сеанс на новый вход, но будет читать переменную снова в каждом новом сеансе, пока вы не удалите ее из профиля оболочки или блока `env` [файла параметров](/docs/ru/settings).
6. Учетные данные профиля Anthropic и федерации, учетные данные, которые используют CLI `ant` и Workload Identity Federation. Профиль, который написал `ant auth login`, ранжируется здесь только когда вы называете его в `ANTHROPIC_PROFILE`; в противном случае он ранжируется ниже `/login`. Смотрите [Профили Anthropic и учетные данные федерации](#anthropic-profiles-and-federation-credentials).
7. Учетные данные OAuth подписки из `/login`. Это значение по умолчанию для пользователей Claude Pro, Max, Team и Enterprise.

Подписанный сеанс [Claude apps gateway](/docs/ru/claude-apps-gateway) находится вне этого списка: это выбор провайдера, как Amazon Bedrock или Google Cloud's Agent Platform, и он имеет приоритет над ними. Когда существует сеанс шлюза, CLI аутентифицируется с помощью токена шлюза, даже если установлены `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` или `CLAUDE_CODE_USE_FOUNDRY`, и источники учетных данных выше, такие как токен-носитель, ключ API, `apiKeyHelper` и профили, не используются.

Если [управляемые параметры](/docs/ru/managed-settings) вашей машины устанавливают [`forceLoginMethod`](/docs/ru/settings-reference#forceloginmethod) на `"gateway"` или устанавливают [`forceLoginGatewayUrl`](/docs/ru/settings-reference#forcelogingatewayurl), и вы не выбираете облачного провайдера через переменную, такую как `CLAUDE_CODE_USE_BEDROCK` или `CLAUDE_CODE_USE_VERTEX`, ваш сеанс использует только вход через шлюз. Claude Code пропускает другие источники учетных данных и просит вас войти с помощью `/login`. Смотрите [Administrator policy requires a Cloud gateway sign-in](/docs/ru/errors#administrator-policy-requires-a-cloud-gateway-sign-in) для того, что вы видите с каждым оставшимся учетным данием. До версии 2.1.261 или до версии 2.1.265 на машине, которая устанавливает только `forceLoginGatewayUrl`, Claude Code использовал оставшийся сохраненный вход на этих машинах, пока вы не вошли в шлюз.

Если у вас есть активная подписка Claude, но также установлен `ANTHROPIC_API_KEY` в вашей среде, Claude Code использует ключ API после одобрения. Это может привести к сбоям аутентификации, если ключ принадлежит отключенной или истекшей организации.

Запустите `unset ANTHROPIC_API_KEY`, чтобы вернуться к вашей подписке, и проверьте `/status`, чтобы подтвердить, какой метод активен. Когда вход и ключ API оба настроены, `/status` отмечает учетное данные, которое не используется.

[Облачные сеансы](/docs/ru/claude-code-on-the-web) всегда используют учетные данные вашей подписки. Если вы установите `ANTHROPIC_API_KEY` или `ANTHROPIC_AUTH_TOKEN` в облачной среде, это не переопределит учетные данные вашей подписки.

<h4 id="anthropic-profiles-and-federation-credentials">
  Профили Anthropic и учетные данные федерации
</h4>

Профиль — это именованный файл конфигурации учетных данных в вашем [каталоге конфигурации Anthropic](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory), по умолчанию `~/.config/anthropic` на macOS и Linux или `%APPDATA%\Anthropic` на Windows. Режим аутентификации профиля — это `oidc_federation` когда вы настраиваете его для [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) или `user_oauth` когда [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication) написал его или вы [вошли в учетную запись Console без ключа API](#sign-in-without-an-api-key).

Claude Code не читает профили или переменные федерации в [bare mode](/docs/ru/headless#start-faster-with-bare-mode), в Claude Desktop или в облачных сеансах. В этих сеансах `/status` не показывает строку `Profile`.

Claude Code проверяет три источника в этом порядке и останавливается на первом установленном. Таблица показывает, что устанавливает каждый источник и где он ранжируется против вашего учетного данного `/login`.

| Источник             | Установлено                                                                                                                                                         | Ранг против `/login`                                                                                                                           |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Именованный профиль  | `ANTHROPIC_PROFILE`                                                                                                                                                 | Выше, независимо от режима аутентификации профиля                                                                                              |
| Переменные федерации | `ANTHROPIC_FEDERATION_RULE_ID` и `ANTHROPIC_ORGANIZATION_ID`, оба установлены                                                                                       | Выше                                                                                                                                           |
| Активный профиль     | Файл [`active_config`](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) в вашем каталоге конфигурации или профиль с именем `default` | Выше когда его режим аутентификации — `oidc_federation`; ниже рабочего учетного данного `/login` когда его режим аутентификации — `user_oauth` |

Правило `user_oauth` предотвращает перемещение оставшегося профиля `ant auth login` с запросов с учетной записи, на которую вы вошли с помощью `/login`. Для переменных федерации Claude Code также читает другие переменные в [справочнике WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables), такие как `ANTHROPIC_IDENTITY_TOKEN_FILE`, когда он обменивает ваш токен идентификации. Для формата файла профиля смотрите [справочник WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file).

Чтобы подтвердить, какой источник выбрал Claude Code, запустите `/status`. Строка `Profile` называет источник вместо строки `Login method`. Когда профиль является используемым учетным данием, строки `Organization` и `Email` показывают его учетную запись.

Если вы запустите Claude Code с `--debug`, он также записывает строку `Using Anthropic profile auth` с именем источника в журнал отладки в `~/.claude/debug/<session-id>.txt`. Когда Claude Code пропускает активный профиль `user_oauth` потому что у вас есть рабочее учетное данные `/login`, он записывает предупреждение в журнал отладки, говоря, что использует вход claude.ai вместо этого.

Когда вход профиля `user_oauth` истек и Claude Code не может его обновить, запросы завершаются с ошибкой [Anthropic profile login expired](/docs/ru/errors#anthropic-profile-login-expired).

Функции, которые требуют вашего входа claude.ai, такие как [соединители claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai) и [`/schedule`](/docs/ru/routines), недоступны при выборе одного из этих источников. Чтобы остановить Claude Code от выбора источника:

* **Именованный профиль или переменные федерации**: отмените установку `ANTHROPIC_PROFILE` или отмените установку любой переменной федерации
* **Активный профиль**: запустите `/logout` для профиля `user_oauth` чье текущее учетное данные вы написали [войдя в учетную запись Console без ключа API](#sign-in-without-an-api-key), запустите `ant auth logout` для одного чье текущее учетное данные написал `ant auth login`, или удалите файл профиля из `configs/` в вашем каталоге конфигурации для любого режима аутентификации

<h3 id="generate-a-long-lived-token">
  Создание долгоживущего токена
</h3>

Для конвейеров CI, скриптов или других сред, где интерактивный вход через браузер недоступен, создайте однолетний токен OAuth с помощью `claude setup-token`:

```bash theme={null}
claude setup-token
```

Команда открывает тот же поток авторизации браузера, что и `/login`, и токен выводится в терминал после того, как вы одобрите доступ в браузере. Она не сохраняет токен нигде; скопируйте его и установите его как переменную окружения `CLAUDE_CODE_OAUTH_TOKEN` везде, где вы хотите аутентифицироваться:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Этот токен аутентифицируется с помощью вашей подписки Claude и требует план Pro, Max, Team или Enterprise. Он может только выполнять запросы модели, поэтому он не может устанавливать сеансы [Remote Control](/docs/ru/remote-control) или получать [соединители claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai). Серверы MCP, которые вы настраиваете локально, все еще работают.

[Bare mode](/docs/ru/headless#start-faster-with-bare-mode) не читает `CLAUDE_CODE_OAUTH_TOKEN`. Если ваш скрипт передает `--bare`, аутентифицируйтесь с помощью `ANTHROPIC_API_KEY` или `apiKeyHelper` вместо этого.
