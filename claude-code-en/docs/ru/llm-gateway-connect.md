> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Подключение Claude Code к шлюзу LLM

> Направьте Claude Code на шлюз LLM вашей организации. Проверьте, уже ли администратор его настроил, или установите базовый URL и учетные данные самостоятельно, затем проверьте соединение и исправьте ошибки шлюза.

[Шлюз LLM](/docs/ru/llm-gateway) — это прокси, который ваша организация запускает между Claude Code и поставщиком модели. Когда ваша организация использует его, Claude Code аутентифицируется на шлюзе с учетными данными, которые выдает ваша организация, вместо вашего личного входа claude.ai.

Эта страница предназначена для разработчиков, запускающих Claude Code через шлюз, который управляет их организация. Она охватывает два пути: [проверка того, уже ли администратор его настроил для вас](#check-for-an-existing-configuration), и [самостоятельная настройка](#configure-claude-code-yourself), если он этого не сделал.

<Note>
  * Чтобы развернуть шлюз для вашей организации, см. [Развертывание шлюза LLM](/docs/ru/llm-gateway-rollout)
  * Для информации о том, что Claude Code отправляет на шлюз, см. [справочник протокола шлюза](/docs/ru/llm-gateway-protocol)
</Note>

<h2 id="check-for-an-existing-configuration">
  Проверка существующей конфигурации
</h2>

Администраторы могут распространять адрес шлюза и учетные данные через [управляемые параметры](/docs/ru/managed-settings), управление устройствами или [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), чтобы Claude Code подхватил их при запуске без каких-либо действий с вашей стороны. Чтобы проверить, уже ли ваша организация это сделала:

<Steps>
  <Step title="Запустите Claude Code">
    Запустите `claude`. Если он открывается на экран входа вместо сеанса, учетные данные шлюза не были распространены; [настройте его самостоятельно](#configure-claude-code-yourself) ниже.
  </Step>

  <Step title="Проверьте вкладку Status">
    Если Claude Code запустил сеанс без отображения экрана входа, запустите `/status`, откройте вкладку **Status** и проверьте две строки:

    * `Anthropic base URL`: эта строка появляется только при установке адреса шлюза. Если ее нет, Claude Code не указан на шлюз; [настройте его самостоятельно](#configure-claude-code-yourself) ниже.
    * `Auth token` или `API key`: строка, называющая `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY` или `apiKeyHelper`, подтверждает, что учетные данные шлюза активны. Строка `Login method`, называющая учетную запись claude.ai, означает, что учетные данные не были распространены; [установите их самостоятельно](#set-the-credential-variable).
  </Step>

  <Step title="Отправьте тестовое сообщение">
    Закройте меню `/status` и отправьте любой запрос в Claude Code. Обычный ответ от Claude без ошибок подтверждает, что соединение с шлюзом работает.
  </Step>
</Steps>

Если обе строки в меню `/status` выглядят правильно, но сообщение Claude не проходит, см. [таблицу устранения неполадок](#troubleshoot-gateway-errors).

<h2 id="configure-claude-code-yourself">
  Самостоятельная настройка Claude Code
</h2>

Чтобы самостоятельно настроить Claude Code для шлюза, вам нужны от команды шлюза:

* Базовый URL шлюза
* Учетные данные: строка ключа или токена, или команда, которая их получает
  * Если команда шлюза не указала, какой тип учетных данных это, раздел [переменная учетных данных](#set-the-credential-variable) ниже охватывает, что попробовать

Разделы ниже охватывают конфигурацию по порядку:

* [Установка переменной учетных данных](#set-the-credential-variable) и [установка базового URL](#set-the-base-url-and-credential): две переменные, которые нужны каждому соединению с шлюзом
* [Проверка соединения](#verify-the-connection): подтвердите, что оно работает, прежде чем что-либо сохранять
* [Настройка каждой поверхности](#configure-each-surface): если вы используете поверхность, отличную от CLI Claude Code, такую как VS Code, см., как настроить ее с учетными данными шлюза
* [Дополнительная конфигурация](#additional-configuration): переменные, которые некоторым шлюзам нужны помимо базового URL и учетных данных, такие как пользовательский заголовок, помощник учетных данных, обнаружение модели, базовый URL в формате поставщика или отключение трафика вне пути шлюза. Устанавливайте их только если администратор их назвал или ваша сеть ограничивает исходящий трафик

<h3 id="set-the-credential-variable">
  Установка переменной учетных данных
</h3>

Чтобы аутентифицировать Claude Code на шлюзе, установите ваши учетные данные в переменную окружения. Какая переменная зависит от того, что вам сказала команда шлюза:

| Установите учетные данные в                             | Используйте когда                                               |
| :------------------------------------------------------ | :-------------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | Команда шлюза сказала "bearer token" или "Authorization header" |
| `ANTHROPIC_API_KEY`                                     | Команда шлюза сказала "API key" или "x-api-key"                 |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | Учетные данные ротируются или поступают из хранилища            |

Если вам не сказали, какой тип, используйте `ANTHROPIC_AUTH_TOKEN`; [запрос проверки](#verify-the-connection) ниже показывает, как определить, нужно ли переключиться.

<h3 id="set-the-base-url-and-credential">
  Установка базового URL и учетных данных
</h3>

Установите базовый URL шлюза и переменную учетных данных, которую вы выбрали выше, как переменные окружения. Примеры используют `ANTHROPIC_AUTH_TOKEN`; замените его на `ANTHROPIC_API_KEY`, если это [переменная, которую вы выбрали](#set-the-credential-variable). Вы можете установить их [в вашей оболочке](#set-as-shell-environment-variables), что длится один сеанс терминала, или [в файле параметров Claude Code](#set-in-a-settings-file), что сохраняется везде, где работает Claude Code.

Для вашего первого соединения начните с экспорта оболочки и запустите [запрос проверки](#verify-the-connection) перед перемещением значений в файл параметров.

<h4 id="set-as-shell-environment-variables">
  Установка как переменные окружения оболочки
</h4>

Замените значения на те, которые вам дала команда шлюза:

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

Экспорты оболочки применяются только к этому сеансу терминала и программам, запущенным из него. Редактор, запущенный с док-станции или меню "Пуск", их не увидит. Чтобы они сохранялись в новых терминалах, добавьте те же строки в профиль вашей оболочки, такой как `~/.zshrc`, `~/.bashrc` или ваш PowerShell `$PROFILE`.

Если вы экспортируете шлюз только в вашей оболочке, он не надежно достигает фоновых агентов, размещенных [супервизором](/docs/ru/agent-view#how-background-sessions-are-hosted); см. [как каждый фоновый сеанс получает свой шлюз](/docs/ru/agent-view#llm-gateway). Используйте файл параметров для любого шлюза, через который фоновые агенты должны всегда маршрутизироваться.

<h4 id="set-in-a-settings-file">
  Установка в файле параметров
</h4>

Чтобы конфигурация применялась везде, где работает Claude Code, включая [фоновых агентов](/docs/ru/agent-view#how-background-sessions-are-hosted), установите переменные в блоке `env` [файла параметров](/docs/ru/settings) вместо того, чтобы полагаться на вашу оболочку. Файлы параметров имеют разные области:

* `~/.claude/settings.json` применяется ко всем вашим проектам. На Windows путь — `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` применяется к одному проекту. Claude Code добавляет его в ваш глобальный gitignore при сохранении параметра там; если вы создаете его вручную или позволяете Claude написать его, сначала добавьте его в gitignore самостоятельно, чтобы случайно не зафиксировать ваши учетные данные

<Warning>
  Не помещайте учетные данные в `.claude/settings.json` проекта. Этот файл фиксируется и совместно используется со всеми, кто клонирует репозиторий.
</Warning>

Блок `env` выглядит одинаково в любом файле:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

Когда и экспорт оболочки, и блок `env` файла параметров устанавливают одну и ту же переменную, применяется значение файла параметров. Запустите `/status`, чтобы увидеть, какой базовый URL и источник учетных данных использует Claude Code.

<h3 id="verify-the-connection">
  Проверка соединения
</h3>

С переменными, экспортированными в вашей оболочке, отправьте однотокенный запрос на шлюз напрямую. Это подтверждает, что URL и учетные данные работают, прежде чем вы откроете Claude Code, поэтому сбой указывает на шлюз, а не на вашу конфигурацию. Команды ниже читают переменные оболочки, поэтому им нужны [экспорты оболочки](#set-as-shell-environment-variables) даже если вы также поместили значения в файл параметров.

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

Если ваш шлюз ожидает ключи в заголовке `x-api-key`, замените заголовок `Authorization` на `x-api-key: $ANTHROPIC_API_KEY` в команде Bash или запись таблицы хеша `"Authorization"` на `"x-api-key" = "$env:ANTHROPIC_API_KEY"` в команде PowerShell.

Ответ JSON, начинающийся с `{"id":"msg_` и включающий поле `"content":[...]`, означает, что шлюз доступен и учетные данные работают. Ошибка, называющая неизвестную модель, все еще доказывает, что URL и учетные данные работают, так как шлюз аутентифицировал запрос перед отклонением имени модели; вам не нужно находить модель, которую обслуживает ваш шлюз, для этого теста. `401` означает, что учетные данные были отклонены: если вы угадали переменную, переключитесь на другую и переэкспортируйте.

<h4 id="confirm-in-claude-code">
  Подтверждение в Claude Code
</h4>

Запустите `claude` из той же оболочки, чтобы он унаследовал экспорты, отправьте сообщение и запустите `/status`.

На вкладке **Status** строка `Anthropic base URL` должна показывать адрес вашего шлюза, что подтверждает маршрутизацию запросов туда; если строки нет, переменная не достигла сеанса. Строка `Auth token` или `API key`, называющая переменную, которую вы установили, подтверждает, что учетные данные шлюза активны, а не сохраненный вход claude.ai.

Если сообщение не проходит или `/status` не показывает URL шлюза, см. [таблицу устранения неполадок](#troubleshoot-gateway-errors) ниже.

<h3 id="how-the-credential-variable-maps-to-a-header">
  Как переменная учетных данных отображается на заголовок
</h3>

Каждая переменная отправляет учетные данные в другом HTTP-заголовке: `ANTHROPIC_AUTH_TOKEN` в `Authorization: Bearer`, `ANTHROPIC_API_KEY` в `x-api-key` и `apiKeyHelper` в обоих. Учетные данные в неправильной переменной достигают шлюза в заголовке, который он не читает, и запрос не проходит с `401`. Если запрос проверки вернул `401`, переключитесь на другую переменную и попробуйте снова.

<h3 id="conflicts-with-an-existing-login">
  Конфликты с существующим входом
</h3>

Переменная учетных данных шлюза имеет приоритет над сохраненным входом claude.ai или ключом Console. Ваш вход claude.ai остается сохраненным и неиспользуемым, пока переменная установлена; отмените установку переменной, и Claude Code вернется к нему. С `ANTHROPIC_AUTH_TOKEN` переменная имеет приоритет немедленно. С `ANTHROPIC_API_KEY` вам предлагается один раз в интерактивном режиме одобрить ключ перед тем, как он возьмет верх.

Запустите `/status`, чтобы подтвердить, какой источник учетных данных активен. Если при запуске показывается предупреждение о конфликте аутентификации, называющее два источника, см. первую строку [таблицы устранения неполадок](#troubleshoot-gateway-errors), чтобы узнать, какой из них отбросить. Чтобы очистить сохраненный вход, чтобы остались только учетные данные шлюза, запустите `/logout`.

<h2 id="configure-each-surface">
  Настройка каждой поверхности
</h2>

CLI читает переменные окружения и файлы параметров выше. Другие поверхности — это расширение VS Code, настольное приложение, GitHub Actions, Agent SDK и облачные поверхности, такие как Slack и веб; разделы ниже охватывают, достигают ли эти параметры каждую.

<h3 id="vs-code-extension">
  Расширение VS Code
</h3>

Установите переменные шлюза для [расширения VS Code](/docs/ru/vs-code) в `claudeCode.environmentVariables` в собственных параметрах пользователя VS Code, открытых командой **Preferences: Open User Settings (JSON)**. Расширение проверяет учетные данные из этого параметра перед запуском, поэтому это надежное место для учетных данных шлюза; значения в `~/.claude/settings.json` достигают порожденного процесса, но не проверку входа самого расширения.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  Настольное приложение
</h3>

Настольное приложение читает маршрутизацию шлюза из [конфигурации сторонних поставщиков](https://claude.com/docs/third-party/claude-desktop/gateway), а не из `ANTHROPIC_BASE_URL` или `settings.json`. Эта конфигурация может поступить от вашей организации или из формы в самом приложении:

* **Распределено администратором**: если ваша организация [развернула конфигурацию](/docs/ru/llm-gateway-rollout#distribute-through-managed-settings), настольное приложение маршрутизирует через шлюз без каких-либо действий с вашей стороны
* **Настроено локально**: для устройств без распределенной администратором конфигурации откройте Help → Troubleshooting → Включить режим разработчика, который перезагружает приложение с меню Developer. Затем откройте Developer → Configure Third-Party Inference и введите базовый URL вашего шлюза. Распределенная администратором конфигурация имеет приоритет и делает эту форму доступной только для чтения

При активной конфигурации шлюза настольное приложение запускает сеансы только на вашем локальном компьютере: средство выбора окружения не предлагает сеансы SSH или размещенные в облаке окружения Anthropic, и [Remote Control](/docs/ru/remote-control) недоступен. Чтобы использовать Claude Code на удаленном хосте через шлюз, запустите CLI на этом хосте с установленными [`ANTHROPIC_BASE_URL` и учетными данными шлюза](#set-the-base-url-and-credential).

Если настольное приложение показывает `Gateway was unreachable`, приложение не смогло достичь настроенный базовый URL при запуске; проверьте URL и сетевой путь с помощью [теста curl выше](#verify-the-connection).

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/ru/github-actions) читает `ANTHROPIC_BASE_URL` и `ANTHROPIC_CUSTOM_HEADERS` из блока `env` рабочего процесса. Передайте учетные данные как вход `anthropic_api_key` действия; действие устанавливает его как `ANTHROPIC_API_KEY`, поэтому он достигает шлюза в заголовке `x-api-key`.

Для шлюза `x-api-key` установите базовый URL в `env` и передайте ключ шлюза как вход:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Для шлюза bearer-token передайте один и тот же секрет как вход `anthropic_api_key` и `ANTHROPIC_AUTH_TOKEN` в блоке `env` рабочего процесса. Действие требует `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN` или федерацию рабочей нагрузки перед запуском Claude Code, и оно не читает `ANTHROPIC_AUTH_TOKEN`, поэтому вход удовлетворяет эту проверку запуска. Переменная окружения помещает ключ в заголовок `Authorization`, который читает шлюз; копия в `x-api-key` игнорируется:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Для других вариантов аутентификации действия, включая `CLAUDE_CODE_OAUTH_TOKEN` и федерацию рабочей нагрузки, см. [Claude Code GitHub Actions](/docs/ru/github-actions) и [README](https://github.com/anthropics/claude-code-action#readme) действия.

<h3 id="agent-sdk">
  Agent SDK
</h3>

[Agent SDK](/docs/ru/agent-sdk/overview) не имеет опций, специфичных для шлюза; он передает переменные окружения процессу Claude Code, который он порождает. Каждый SDK принимает опцию `env`, которая устанавливает окружение порожденного процесса, и SDK TypeScript и Python обрабатывают это по-разному:

* TypeScript: порожденный процесс по умолчанию наследует родительское окружение, но установка `options.env` полностью заменяет окружение. Распределите `process.env` в него, чтобы сохранить переменные шлюза.
* Python: `ClaudeAgentOptions(env=...)` объединяется поверх унаследованного окружения, поэтому переменные шлюза, установленные в родительском процессе, проходят без распределения.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-cloud-sessions-and-remote-control">
  Slack, облачные сеансы и Remote Control
</h3>

[Claude Code в Slack](/docs/ru/slack) и [облачные сеансы](/docs/ru/claude-code-on-the-web) всегда используют API Anthropic; они не являются частью развертывания шлюза. Переменные шлюза, установленные в конфигурации окружения облачного сеанса, не применяются. Если ваш трафик должен оставаться на шлюзе, не включайте эти поверхности для этих пользователей.

[Remote Control](/docs/ru/remote-control) и [голосовая диктовка](/docs/ru/voice-dictation) оба полагаются на идентичность claude.ai: Remote Control для связи живого сеанса с вашей учетной записью, и голосовая диктовка для достижения конечной точки транскрипции claude.ai. Они недоступны, пока активны `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper`. Remote Control также отключен, пока `ANTHROPIC_BASE_URL` указывает на хост, не принадлежащий Anthropic, поэтому входа с claude.ai недостаточно само по себе. До версии 2.1.196 не-Anthropic базовый URL не блокировал Remote Control.

Чтобы восстановить любую из этих функций, войдите с claude.ai и отмените установку переменных шлюза, которые проверяет эта функция. Раздел Remote Control в `claude doctor` называет то, что в настоящее время блокирует Remote Control.

* Голосовая диктовка: отмените установку учетных данных шлюза
* Remote Control: отмените установку учетных данных шлюза и `ANTHROPIC_BASE_URL`

<h2 id="additional-configuration">
  Дополнительная конфигурация
</h2>

Эти параметры охватывают случаи, выходящие за рамки базового URL и учетных данных. Устанавливайте их только если инструкции администратора, правила исходящего трафика вашей сети или [таблица устранения неполадок](#troubleshoot-gateway-errors) требуют одного.

<h3 id="send-additional-headers">
  Отправка дополнительных заголовков
</h3>

Некоторые шлюзы маршрутизируют или помечают запросы, используя пользовательский заголовок в дополнение к учетным данным, например идентификатор клиента или ключ маршрутизации. Чтобы отправить один, установите [`ANTHROPIC_CUSTOM_HEADERS`](/docs/ru/env-vars) с одной парой `Name: Value` на строку. Пример ниже добавляет заголовок маршрутизации с именем `X-Org-Route`:

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

Вы также можете установить `ANTHROPIC_CUSTOM_HEADERS` в блоке `env` файла параметров. Используйте `\n` между парами там, так как строки JSON не могут охватывать несколько строк:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

Заголовки маршрутизации и клиента, подобные этим, считаются [заголовками, требующими одобрения](/docs/ru/server-managed-settings#environment-variables-and-the-approval-dialog). Когда заголовки поступают из файла параметров проекта, Claude Code применяет их в соответствии с [правилами применения значений `env`](/docs/ru/settings-reference#when-claude-code-applies-env-values).

<h3 id="add-gateway-models-to-the-model-picker">
  Добавление моделей шлюза в средство выбора модели
</h3>

При включенном обнаружении модели Claude Code запрашивает шлюз для его списка моделей при запуске и добавляет эти имена в средство выбора `/model` наряду со встроенными записями. Если вы или администратор установили `replaceBuiltInOptions` в [линейке `modelPicker`](/docs/ru/settings-reference#modelpicker), Claude Code также скрывает обнаруженные имена. Он сохраняет строку для модели, которую сеанс уже использует.

Включите его, если ваш шлюз обслуживает имена моделей, которых нет во встроенном списке Claude Code, и вы хотите выбрать их из средства выбора. Если встроенные модели — это то, что вы используете, вам не нужно обнаружение; администратор также мог уже включить его через управляемые параметры.

Чтобы включить его, установите `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` в вашей оболочке или в блоке `env` `~/.claude/settings.json`.

Обнаруженные модели появляются как дополнительные записи `/model`. Каждая запись показывает описание, которое предоставляет ваш шлюз для модели, или `From gateway`, когда оно не предоставляется.

Чтобы подтвердить, что обнаружение запустилось, запустите `claude --debug` и ищите строки `[gatewayDiscovery]` в журнале отладки в `~/.claude/debug/<session-id>.txt`. При первом успешном обнаружении Claude Code регистрирует, сколько моделей было кэшировано, и регистрирует снова только при изменении списка шлюза. `404`, timeout или перенаправление также отображаются там. Для информации о том, когда запускается обнаружение, что оно фильтрует и формат ответа, который обслуживают шлюзы, см. [справочник обнаружения модели](/docs/ru/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Ротация учетных данных с apiKeyHelper
</h3>

`apiKeyHelper` — это команда, которую Claude Code запускает для получения ваших учетных данных шлюза, вместо чтения их из статической переменной окружения.

Используйте помощника, когда учетные данные истекают по расписанию, поступают из хранилища или команды SSO, или администратор сказал вам настроить один. Если ваши учетные данные — это фиксированная строка, которую вы устанавливаете один раз, [переменная учетных данных](#set-the-credential-variable) — это все, что вам нужно, и вы можете пропустить этот раздел.

Помощник — это любая команда оболочки, которая выводит текущие учетные данные на stdout. Claude Code запускает ее через вашу системную оболочку, поэтому на Windows это может быть исполняемый файл или вызов PowerShell. Сделайте так, чтобы команда не выводила ничего, кроме учетных данных. На Claude Code v2.1.227 или позже баннер или строка журнала, выведенные вместе с ключом, приводят к [отказу помощника](/docs/ru/errors#your-apikeyhelper-script-is-failing). Напишите скрипт, сделайте его исполняемым и ссылайтесь на него из `apiKeyHelper` в вашем [файле параметров](/docs/ru/settings):

<Tabs>
  <Tab title="Bash или Zsh">
    Например, скрипт, который читает из хранилища:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Ссылайтесь на его путь в `~/.claude/settings.json`:

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    Например, скрипт, который читает из хранилища:

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Ссылайтесь на вызов PowerShell в `%USERPROFILE%\.claude\settings.json`, экранируя обратные косые черты в строке JSON:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code кэширует вывод помощника в течение пяти минут по умолчанию и переустанавливает помощника после истечения времени жизни кэша. Чтобы изменить время жизни, установите `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` в миллисекундах, например `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` на 15 минут.

См. [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) для других случаев, в которых Claude Code переустанавливает помощника.

Значение помощника отправляется в оба заголовка `Authorization` и `x-api-key`, поэтому оно работает, какой бы заголовок ни читал ваш шлюз.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Отключение трафика вне пути шлюза
</h3>

Шлюз переносит запросы модели, но Claude Code также отправляет несущественный фоновый трафик вне пути шлюза в Anthropic и в сторонние сервисы, такие как GitHub: проверки версий, телеметрия, примечания к выпуску и аналогичные запросы. В сети, которая разрешает исходящий трафик только на шлюз, эти запросы не выполняются и могут отображаться как заблокированные соединения в вашем мониторинге исходящего трафика.

Claude Code прикрепляет учетные данные к запросу телеметрии или метрик использования только когда запрос идет на хост, к которому принадлежат учетные данные. Пока `ANTHROPIC_BASE_URL` указывает на шлюз, Claude Code отправляет свои события телеметрии в Anthropic без учетных данных вашего шлюза. С активной [переменной учетных данных](#set-the-credential-variable) или `apiKeyHelper`, Claude Code не сообщает метрики использования на [панель аналитики](/docs/ru/analytics#access-analytics-for-api-customers) Console. До v2.1.246 Claude Code мог прикреплять учетные данные шлюза к запросам телеметрии и метрик использования, предназначенным для хостов Anthropic; запросы модели всегда шли на шлюз с учетными данными, которые ожидает шлюз.

Чтобы отключить этот трафик, установите `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` наряду с переменными шлюза, в том же блоке экспорта оболочки или блоке `env` файла параметров:

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

Установка переменной имеет эти эффекты и ограничения:

* Она отключает автоматические обновления, поэтому планируйте другой путь обновления, такой как ваш менеджер пакетов или управляемое распределение.
* Она подавляет проверку доступности [быстрого режима](/docs/ru/fast-mode). Если предыдущая проверка уже не включила быстрый режим на машине, `/fast` сообщает, что быстрый режим недоступен.
* Она не влияет на [обнаружение модели шлюза](#add-gateway-models-to-the-model-picker), которое запрашивает только ваш шлюз. До v2.1.257 переменная также останавливала обновление обнаружения, поэтому средство выбора сохраняло ранее кэшированный список.
* [Проверка безопасности домена](/docs/ru/data-usage#webfetch-domain-safety-check) инструмента WebFetch не затрагивается и по-прежнему вызывает `api.anthropic.com`. Отключите ее отдельно с помощью `skipWebFetchPreflight: true` в [параметрах](/docs/ru/settings), если ваша сеть блокирует этот хост.
* Для каждого потока телеметрии и переменной, которая его контролирует, см. [сервисы телеметрии](/docs/ru/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Маршрутизация к облачному поставщику через шлюз
</h3>

Эти конфигурации направляют Claude Code на шлюз через переменную базового URL, специфичную для поставщика, вместо `ANTHROPIC_BASE_URL`. Шлюзы Amazon Bedrock и Google Cloud's Agent Platform принимают собственные форматы запросов этих поставщиков; шлюзы Microsoft Foundry и Claude Platform на AWS принимают формат Anthropic Messages. На маршрутах Amazon Bedrock и Google Cloud's Agent Platform Claude Code также ограничивает бета-заголовки и поля запроса, которые он отправляет, набором, который принимает поставщик. Для информации о том, что получает ваш шлюз на каждом маршруте, см. [руководство совместимости шлюза](/docs/ru/llm-gateway-protocol).

Используйте один только если команда шлюза специально назвала Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или Claude Platform на AWS. Если [запрос проверки](#verify-the-connection) выше вернул JSON, вы можете пропустить этот раздел.

Установите блок для поставщика, которого назвала команда шлюза. Переменные skip-auth в блоках Amazon Bedrock, Google Cloud's Agent Platform и Claude Platform на AWS говорят Claude Code не подписывать запросы с учетными данными поставщика облака, так как шлюз их держит. Если шлюзу также нужен свой собственный токен, где вы его размещаете, зависит от поставщика:

* **Amazon Bedrock, Google Cloud's Agent Platform или Claude Platform на AWS**: добавьте `ANTHROPIC_AUTH_TOKEN` после блока. Claude Code отправляет его на шлюз как заголовок `Authorization: Bearer`. Для учетных данных в другой схеме или заголовке используйте [`ANTHROPIC_CUSTOM_HEADERS`](#send-additional-headers) вместо этого. Сохраняйте переменную skip-auth установленной в любом случае, так как без нее Claude Code удаляет любой заголовок `Authorization`, который добавили бы `ANTHROPIC_AUTH_TOKEN`, [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) или `ANTHROPIC_CUSTOM_HEADERS`.
* **Microsoft Foundry**: используйте `ANTHROPIC_FOUNDRY_API_KEY`, как показано в [его блоке](#microsoft-foundry)

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Оставьте `AWS_BEARER_TOKEN_BEDROCK` неустановленным, когда шлюз выдает свои собственные учетные данные. Если вы установите его, Claude Code отправляет этот [ключ API Amazon Bedrock](/docs/ru/amazon-bedrock#2-configure-aws-credentials) как заголовок `Authorization` вместо токена вашего шлюза, даже с установленным `CLAUDE_CODE_SKIP_BEDROCK_AUTH`.

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h4>

Замените ID проекта и регион на свои собственные значения. Claude Code включает оба в путь каждого запроса, который он отправляет на шлюз:

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

Блок охватывает маршрутизацию и аутентификацию. Переопределения региона и закрепления модели из [настройки Agent Platform](/docs/ru/google-vertex-ai#4-configure-claude-code) применяются через шлюз также:

* **Регионы для каждой модели**: если ваш шлюз обслуживает некоторые модели из региона, отличного от `CLOUD_ML_REGION`, установите соответствующую переменную `VERTEX_REGION_CLAUDE_*` для каждой, например `VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1`. [Справочник переменных окружения](/docs/ru/env-vars) перечисляет точные имена.
* **Версии моделей**: закрепите `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` и `ANTHROPIC_DEFAULT_HAIKU_MODEL`, как в [Закрепление версий моделей](/docs/ru/google-vertex-ai#5-pin-model-versions). Установка `ANTHROPIC_DEFAULT_HAIKU_MODEL` также перемещает фоновые задачи, такие как заголовки сеансов, на эту модель, и этот раздел объясняет, какая модель их запускает в противном случае.
* **Возможности модели**: если вы закрепляете ID модели, который ваша версия Claude Code не распознает, функции, такие как уровни усилий или расширенное мышление, могут оставаться отключенными на нем. Объявите, что поддерживает модель, с помощью [`ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES`](/docs/ru/model-config#customize-pinned-model-display-and-capabilities) и его аналогов для Sonnet и Haiku.

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Поместите учетные данные шлюза в `ANTHROPIC_FOUNDRY_API_KEY`; они отправляются на шлюз как заголовок `x-api-key`. Шлюз, который ожидает токен-носитель, может принять [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/ru/env-vars) вместо этого. Claude Code отправляет это значение как заголовок `Authorization: Bearer`, и оно имеет приоритет над `ANTHROPIC_FOUNDRY_API_KEY`, когда оба установлены. Требует Claude Code v2.1.203 или позже.

Для шлюза, который внедряет свой собственный заголовок `Authorization`, установите `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` и оставьте обе переменные учетных данных неустановленными. Claude Code затем отправляет запросы без учетных данных Azure и сохраняет заголовок `Authorization`, который вы предоставляете, например через `ANTHROPIC_CUSTOM_HEADERS`. До v2.1.203 `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` без ключа API оставлял клиент Microsoft Foundry неспособным отправлять запросы.

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  Claude Platform на AWS
</h4>

См. [Claude Platform на AWS](/docs/ru/claude-platform-on-aws) для ID рабочего пространства.

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h4 id="confirm-the-provider-route">
  Подтверждение маршрута поставщика
</h4>

Запустите `claude` из оболочки, где вы установили блок, и запустите `/status`. С блоком Amazon Bedrock вкладка **Status** показывает строки, подобные этим:

```text theme={null}
API provider: Amazon Bedrock
Bedrock base URL: https://llm-gateway.example.com/bedrock
AWS auth skipped
```

Другие блоки производят те же строки под названиями своих поставщиков, например `Vertex base URL` и `GCP auth skipped` для Google Cloud's Agent Platform; блок Microsoft Foundry показывает строку пропуска аутентификации только если вы установили `CLAUDE_CODE_SKIP_FOUNDRY_AUTH`. Если вы также маршрутизируете через корпоративный прокси, строка `Proxy` показывает URL прокси. Если строка базового URL отсутствует, переменная не достигла сеанса.

<h2 id="troubleshoot-gateway-errors">
  Устранение неполадок с ошибками шлюза
</h2>

Это наиболее распространенные ошибки при запуске Claude Code через шлюз, с причиной на стороне шлюза и исправлением:

| Ошибка                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Причина                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Исправление                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Предупреждение при запуске, называющее два источника учетных данных и заканчивающееся на `auth may not work as expected`. Более старые версии показывают `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` вместо этого.                                                                                                                                                                                                                                        | Учетные данные шлюза и сохраненный вход оба активны; переменная используется для запросов, но устаревший вход может вызвать неожиданное поведение аутентификации                                                                                                                                                                                                                                                                                                    | Отмените установку переменной, чтобы использовать сохраненный вход, или запустите `/logout`, чтобы использовать учетные данные шлюза                                                                                                                                                                                                                                                                                             |
| Ошибки `401`, называющие недействительный или неузнанный токен                                                                                                                                                                                                                                                                                                                                                                                                                       | Учетные данные не являются теми, которые выдал шлюз, или они находятся в заголовке, который шлюз не читает                                                                                                                                                                                                                                                                                                                                                          | Подтвердите, что переменная соответствует вашему типу учетных данных в [таблице учетных данных](#set-the-credential-variable), и переустановите ключ на шлюзе, если он был отозван                                                                                                                                                                                                                                               |
| `Your apiKeyHelper script is failing`, или `apiKeyHelper failed:` на stderr в неинтерактивном режиме                                                                                                                                                                                                                                                                                                                                                                                 | Команда в параметре [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) не выдала пригодный ключ, поэтому запросы содержат заполнитель ключа                                                                                                                                                                                                                                                                                                                      | Запустите команду напрямую, чтобы увидеть, почему она не работает, и повторно аутентифицируйтесь у поставщика учетных данных, если он сообщает об истекшем сеансе; см. [справочник по ошибкам](/docs/ru/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                                   |
| `Connection refused — a firewall or proxy may be blocking it (ConnectionRefused)` когда ничто не отвечает по адресу, или `Can't reach the API server — check your internet or DNS (ENOTFOUND)` когда имя хоста не разрешается, часто после молчаливой паузы, пока Claude Code [повторяет с отступом](/docs/ru/errors#automatic-retries). Код в скобках варьируется; [Unable to connect to API](/docs/ru/errors#unable-to-connect-to-api) охватывает варианты кодов и более раннюю формулировку | Ничто не ответило на базовый URL: адрес неправильный, или VPN или брандмауэр блокирует путь к шлюзу                                                                                                                                                                                                                                                                                                                                                                 | Запустите [тест curl выше](#verify-the-connection), который немедленно не проходит с той же причиной, и подтвердите URL и сетевой путь с командой шлюза                                                                                                                                                                                                                                                                          |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                                                                                                                                                                                                                                                                                             | Шлюз или промежуточный прокси вернул ответ, отличный от API, часто HTML-ошибку или страницу входа                                                                                                                                                                                                                                                                                                                                                                   | Протестируйте с [запросом curl выше](#verify-the-connection); исправьте маршрут шлюза, который возвращает что-то иное, чем ответ Claude API. [Справочник по ошибкам](/docs/ru/errors#api-returned-an-empty-or-malformed-response) объясняет детали, которые сообщает сообщение                                                                                                                                                        |
| Ошибки `400`, называющие `context_management`, `Extra inputs are not permitted` или другие неузнанные поля                                                                                                                                                                                                                                                                                                                                                                           | Шлюз пересылает запросы на вышестоящий, который отклоняет поля, которые Claude Code отправляет на конечные точки формата Anthropic                                                                                                                                                                                                                                                                                                                                  | Установите `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`, что подавляет большинство полей предварительного выпуска; см. [передача функций](/docs/ru/llm-gateway-protocol#feature-pass-through). Некоторые бета-версии не управляются этим флагом; для них установите соответствующую переменную поставщика `CLAUDE_CODE_USE_*`, чтобы Claude Code отправлял только то, что этот поставщик принимает                                      |
| Ошибки `400`, называющие `thinking` или `adaptive`, такие как `Input tag 'adaptive' found`                                                                                                                                                                                                                                                                                                                                                                                           | Сборка вышестоящей модели не принимает адаптивное рассуждение, которое Claude Code запрашивает для моделей Claude 4.6 и позже                                                                                                                                                                                                                                                                                                                                       | Обновите вышестоящий шлюз. На Opus 4.6 и Sonnet 4.6 вместо этого работает `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1`. Переменные возможностей [конфигурации модели](/docs/ru/model-config) применяются только к конфигурациям поставщика, таким как `CLAUDE_CODE_USE_BEDROCK` и `CLAUDE_CODE_USE_VERTEX`, а не за шлюзом `ANTHROPIC_BASE_URL`                                                                                          |
| Ошибки `400`, указывающие контекст или лимит токена в собственных словах шлюза, такие как `ContextWindowExceededError` или `prompt token count of N exceeds the limit of M`                                                                                                                                                                                                                                                                                                          | Шлюз применяет меньший контекст, чем собственное окно модели, и переписывает ошибку вышестоящего, поэтому Claude Code не распознает это как [слишком длинную ошибку](/docs/ru/errors#prompt-is-too-long) и не выполняет автоматическое сжатие и повтор                                                                                                                                                                                                                   | Запустите `/compact`, чтобы восстановить сеанс. Чтобы предотвратить это, установите `CLAUDE_CODE_AUTO_COMPACT_WINDOW` на лимит шлюза; Claude Code зажимает значение как минимум на 100 000 токенов и максимум на контекстное окно модели, поэтому лимит шлюза ниже 100 000 не может быть согласован, и `/compact` остается восстановлением там. Также установите `CLAUDE_CODE_MAX_OUTPUT_TOKENS` ниже лимита вывода модели шлюза |
| Ошибки `400` на каждом запросе, в собственных словах шлюза отклоняющие входную схему инструмента или его `pattern`, на Claude Code v2.1.265 через v2.1.267                                                                                                                                                                                                                                                                                                                           | При постепенном развертывании на этих версиях схема [инструмента Artifact](/docs/ru/artifacts#availability) содержит регулярное выражение с классами символов Unicode `\p{...}`. API Anthropic его принимает, но шлюз или вышестоящий, который проверяет `pattern` каждой схемы инструмента с помощью собственного механизма regex, отклоняет весь запрос                                                                                                                | Обновитесь до v2.1.268 или позже, которая не отправляет регулярное выражение. На затронутой версии [отключите артефакты](/docs/ru/artifacts#disable-artifacts), что удаляет инструмент и его схему из запросов                                                                                                                                                                                                                        |
| Ошибки `400` на каждом запросе, в собственных словах шлюза отклоняющие неузнанный тип инструмента, такие как `Input tag 'advisor_20260301'`, на Claude Code v2.1.275                                                                                                                                                                                                                                                                                                                 | При постепенном развертывании на этой версии запросы содержат запись [инструмента advisor](/docs/ru/advisor) даже с отключенным advisor. API Anthropic его принимает, но шлюз или вышестоящий, который проверяет типы инструментов, отклоняет весь запрос; тот, который [пересылает поля тела запроса без изменений](/docs/ru/llm-gateway-protocol#forward-as-open-lists), пропускает его без влияния. Запись является объявлением, которое не содержит содержимого разговора | Обновитесь до v2.1.276 или позже, которая не отправляет запись за шлюзом `ANTHROPIC_BASE_URL`, если вы не включите advisor. На v2.1.275 установите [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`](/docs/ru/env-vars), что удаляет запись из запросов                                                                                                                                                                                          |
| Модели отсутствуют в средстве выбора `/model`                                                                                                                                                                                                                                                                                                                                                                                                                                        | Имена моделей шлюза отсутствуют во встроенном списке Claude Code, или Claude Code показывает [`modelPicker`](/docs/ru/settings-reference#modelpicker) набор, который заменяет встроенные параметры                                                                                                                                                                                                                                                                       | Включите [обнаружение модели шлюза](#add-gateway-models-to-the-model-picker) или добавьте имена с переменными [конфигурации модели](/docs/ru/model-config). Если Claude Code показывает заменяющий набор `modelPicker`, добавьте модели шлюза в него, или попросите администратора добавить их, когда управляемые параметры его предоставляют                                                                                         |
| `/fast` сообщает `Fast mode unavailable due to network connectivity issues` в то время как запросы вывода работают                                                                                                                                                                                                                                                                                                                                                                   | Проверка доступности [быстрого режима](/docs/ru/fast-mode) идет непосредственно на `api.anthropic.com` и не следует `ANTHROPIC_BASE_URL`, поэтому заблокированный прямой исходящий трафик не проходит проверку. То же сообщение появляется в открытой сети, когда проверка представляет ключ, выданный шлюзом, из `ANTHROPIC_API_KEY` или `apiKeyHelper`, и Anthropic его отклоняет                                                                                      | Добавьте в список разрешений `api.anthropic.com`, если исходящий трафик заблокирован, или установите переменную пропуска; для отклоненного ключа шлюза помогают только переменные пропуска. См. [использование быстрого режима за прокси и LLM шлюзами](/docs/ru/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                             |
| `/fast` сообщает `Fast mode has been disabled by your organization` в сеансе, аутентифицированном с помощью `ANTHROPIC_AUTH_TOKEN`, даже если организация имеет включенный быстрый режим                                                                                                                                                                                                                                                                                             | Проверка доступности требует входа claude.ai или ключа API Anthropic; только с токеном-носителем Claude Code рассматривает быстрый режим как отключенный без отправки проверки                                                                                                                                                                                                                                                                                      | Установите `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`; см. [использование быстрого режима за прокси и LLM шлюзами](/docs/ru/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                                                                                                                                    |
| Claude Code просит вас войти, даже если [тест curl](#verify-the-connection) успешен                                                                                                                                                                                                                                                                                                                                                                                                  | CLI не имеет собственных учетных данных: достижимый базовый URL не является одним, и в интерактивном сеансе блок `env` в `.claude/settings.json` или `.claude/settings.local.json` проекта применяется только после мастера первого запуска и [подсказки доверия](/docs/ru/permissions#what-runs-before-you-trust-a-folder)                                                                                                                                              | Установите `ANTHROPIC_AUTH_TOKEN` где-то, где Claude Code читает перед первоначальной настройкой: экспорт оболочки, блок `env` в `~/.claude/settings.json` или управляемые параметры                                                                                                                                                                                                                                             |
| `ANTHROPIC_API_KEY` установлен, но игнорируется, без подсказки                                                                                                                                                                                                                                                                                                                                                                                                                       | Ключ требует одобрения один раз в интерактивных сеансах, и ранее отклоненный ключ игнорируется без повторного запроса                                                                                                                                                                                                                                                                                                                                               | Включите его под `/config` с опцией `Use custom API key`                                                                                                                                                                                                                                                                                                                                                                         |
| `This machine's managed settings require a first-party login`                                                                                                                                                                                                                                                                                                                                                                                                                        | Управляемые параметры включают `forceLoginMethod` или `forceLoginOrgUUID`, которые не могут сосуществовать с `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper`                                                                                                                                                                                                                                                                                         | Администратор должен удалить `forceLoginMethod` и `forceLoginOrgUUID` из управляемых параметров, чтобы использовать учетные данные шлюза, или удалить учетные данные шлюза, чтобы использовать вход первой стороны. Эти два не могут быть объединены                                                                                                                                                                             |
| `403` с телом HTML, такое как `403 Forbidden`, когда собственные журналы шлюза не показывают полученный запрос                                                                                                                                                                                                                                                                                                                                                                       | Веб-приложение брандмауэра или обратный прокси перед шлюзом заблокировал тело запроса перед тем, как оно достигло шлюза. Подсказки Claude Code включают теги в стиле XML и исходный код, которые соответствуют правилам тела межсайтового скриптинга, поэтому короткий тест curl проходит, а реальный сеанс нет                                                                                                                                                     | Исключите путь `/v1/messages` шлюза из проверки тела запроса. На AWS WAF это управляемое правило `CrossSiteScripting_Body`; на nginx с ModSecurity это эквивалентные правила тела OWASP CRS                                                                                                                                                                                                                                      |
| Ошибки сертификата или TLS, такие как `SSL certificate verification failed` или `Self-signed certificate detected`, когда [тест curl](#verify-the-connection) успешен                                                                                                                                                                                                                                                                                                                | Среда выполнения Claude Code не доверяет тому же центру сертификации, что и `curl`. Обычно за корпоративными прокси-серверами проверки TLS                                                                                                                                                                                                                                                                                                                          | Установите `NODE_EXTRA_CA_CERTS` на путь пакета CA; см. [хранилище сертификатов CA](/docs/ru/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                     |

Если Claude Code повторно просит вас войти после удаления конфигурации шлюза, причина обычно в хранилище учетных данных, а не в шлюзе; см. [ошибки аутентификации](/docs/ru/errors#authentication-errors).

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Обзор шлюзов LLM](/docs/ru/llm-gateway): что такое шлюз и как он взаимодействует с подписками claude.ai
* [Развертывание шлюза LLM для вашей организации](/docs/ru/llm-gateway-rollout): контрольный список, обращенный к администратору, для развертывания и распределения конфигурации шлюза
* [Справочник протокола шлюза](/docs/ru/llm-gateway-protocol): что Claude Code отправляет на шлюз, включая заголовки и поля, которые шлюз должен пересылать
* [Параметры](/docs/ru/settings): где находятся файлы параметров и как читается блок `env`
* [Аутентификация](/docs/ru/authentication): как переменные учетных данных, `apiKeyHelper` и вход OAuth взаимодействуют
