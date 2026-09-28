> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Подключите Claude Code к инструментам через MCP

> Узнайте, как подключить Claude Code к вашим инструментам с помощью Model Context Protocol.

Claude Code может подключаться к сотням внешних инструментов и источников данных через [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), открытый стандарт для интеграции AI с инструментами. MCP servers предоставляют Claude Code доступ к вашим инструментам, базам данных и API.

Подключите server, когда вы обнаружите, что копируете данные в чат из другого инструмента, например из трекера проблем или панели мониторинга. После подключения Claude может читать и действовать на этой системе напрямую вместо работы с тем, что вы вставляете.

Если вы подключаете свой первый server, начните с [MCP quickstart](/docs/ru/mcp-quickstart) для пошагового руководства. Эта страница является полным справочником.

<h2 id="what-you-can-do-with-mcp">
  Что вы можете делать с MCP
</h2>

С подключенными MCP servers вы можете попросить Claude Code:

* **Реализовать функции из трекеров проблем**: "Добавьте функцию, описанную в задаче JIRA ENG-4521, и создайте PR на GitHub."
* **Анализировать данные мониторинга**: "Проверьте Sentry и Statsig, чтобы проверить использование функции, описанной в ENG-4521."
* **Запрашивать базы данных**: "Найдите адреса электронной почты 10 случайных пользователей, которые использовали функцию ENG-4521, на основе нашей базы данных PostgreSQL."
* **Интегрировать дизайны**: "Обновите наш стандартный шаблон электронного письма на основе новых дизайнов Figma, которые были опубликованы в Slack"
* **Автоматизировать рабочие процессы**: "Создайте черновики Gmail, приглашающие этих 10 пользователей на сеанс обратной связи о новой функции."
* **Реагировать на внешние события**: MCP server также может действовать как [канал](/docs/ru/channels), который отправляет сообщения в вашу сессию, поэтому Claude реагирует на сообщения Telegram, чаты Discord или события webhook, пока вас нет.

<h2 id="find-and-build-mcp-servers">
  Поиск и создание MCP servers
</h2>

Просмотрите проверенные коннекторы в [Anthropic Directory](https://claude.ai/directory). Коннекторы Directory используют ту же инфраструктуру MCP, что и Claude Code, поэтому вы можете добавить любой удаленный server из списка с помощью `claude mcp add`.

<Warning>
  Убедитесь, что вы доверяете каждому server перед подключением. Servers, которые получают внешний контент, могут подвергнуть вас [риску prompt injection](/docs/ru/security#protect-against-prompt-injection).
</Warning>

Чтобы создать свой собственный server, см. [руководство по MCP server](https://modelcontextprotocol.io/docs/develop/build-server) для основ протокола и [документацию по созданию Claude connector](https://claude.com/docs/connectors/building) для аутентификации, тестирования и отправки в Directory.

Вы также можете попросить Claude создать server для вас с помощью официального плагина [`mcp-server-dev`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev).

<Steps>
  <Step title="Установите плагин">
    В сеансе Claude Code выполните:

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Если установка не удается, сопоставьте сообщение, которое сообщает Claude Code:

    * `Marketplace "claude-plugins-official" not found`: добавьте marketplace с помощью `/plugin marketplace add anthropics/claude-plugins-official`, затем повторите попытку установки.
    * The plugin is [not found in the marketplace](/docs/ru/plugins/install#install-a-plugin): проверьте имя плагина.

    Если в сводке установки указано `Run /reload-plugins to activate.`, Claude Code затем запустит эту перезагрузку для вас. Если перезагрузка предупреждает, что ваше следующее сообщение повторно прочитает разговор, выполните `/reload-plugins --force`.
  </Step>

  <Step title="Запустите skill сборки">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude спросит о вашем варианте использования и создаст удаленный HTTP или локальный stdio server.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  Установка MCP servers
</h2>

MCP servers можно настроить несколькими способами в зависимости от ваших потребностей:

<h3 id="option-1-add-a-remote-http-server">
  Вариант 1: Добавьте удаленный HTTP server
</h3>

HTTP servers — это рекомендуемый вариант для подключения к удаленным MCP servers. Это наиболее широко поддерживаемый транспорт для облачных сервисов.

```bash theme={null}
# Базовый синтаксис
claude mcp add --transport http <name> <url>

# Реальный пример: подключение к Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Пример с токеном Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

При настройке MCP servers через JSON в `.mcp.json`, `~/.claude.json` или `claude mcp add-json`, поле `type` принимает `streamable-http` как псевдоним для `http`. Спецификация MCP использует имя `streamable-http` для этого транспорта, поэтому конфигурации, скопированные из документации server, работают без изменений.

Запись JSON, которая имеет `url`, но не имеет `type`, является ошибкой конфигурации, потому что Claude Code читает запись без `type` как stdio server. Claude Code пропускает этот server и сообщает `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. До версии 2.1.202 Claude Code сообщал об этой неправильной конфигурации как `command: expected string, received undefined`.

В запусках `--output-format stream-json` Claude Code также сообщает о пропущенной записи `--mcp-config` в поле [`mcp_server_errors`](/docs/ru/headless#stream-responses) события `system/init`, поэтому скрипты могут обнаружить, что server никогда не загружался. Это требует Claude Code версии 2.1.219 или позже.

<h3 id="option-2-add-a-remote-sse-server">
  Вариант 2: Добавьте удаленный SSE server
</h3>

<Warning>
  Транспорт SSE (Server-Sent Events) устарел. Используйте вместо этого HTTP servers, где они доступны.
</Warning>

Некоторые сервисы по-прежнему предоставляют только SSE endpoint. Добавьте их с помощью той же команды `claude mcp add --transport http <name> <url>`, что и [HTTP server](#option-1-add-a-remote-http-server). Claude Code сначала пытается использовать HTTP транспорт и переключается на SSE, когда server его не принимает. Автоматическое переключение требует Claude Code версии 2.1.265 или позже.

На более ранней версии или для прямого подключения через SSE передайте `--transport sse` вместо этого:

```bash theme={null}
# Базовый синтаксис
claude mcp add --transport sse <name> <url>

# Реальный пример: подключение к Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Пример с заголовком аутентификации
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Вариант 3: Добавьте локальный stdio server
</h3>

Stdio servers работают как локальные процессы на вашей машине. Они идеальны для инструментов, которым требуется прямой доступ к системе или пользовательские скрипты.

Claude Code устанавливает `CLAUDE_PROJECT_DIR` в окружение порожденного server в корень проекта, поэтому ваш server может разрешать пути относительно проекта без зависимости от рабочей директории. Это та же директория, которую hooks получают в своей переменной `CLAUDE_PROJECT_DIR`. Прочитайте ее изнутри вашего процесса server, например `process.env.CLAUDE_PROJECT_DIR` в Node или `os.environ["CLAUDE_PROJECT_DIR"]` в Python.

`CLAUDE_PROJECT_DIR` — это стабильный корень проекта и не изменяется при добавлении или удалении рабочих директорий во время сеанса. Server, который ограничивает собственный доступ к файловой системе набором разрешенных директорий, должен вместо этого реализовать MCP запрос `roots/list`. Claude Code отвечает на `roots/list` с директорией запуска сеанса плюс каждая [дополнительная рабочая директория](/docs/ru/permissions#working-directories), которую вы предоставили с помощью `--add-dir`, `/add-dir` или параметра `additionalDirectories`. Claude Code отправляет `notifications/roots/list_changed` при изменении этого набора. До версии 2.1.203 `roots/list` возвращал только директорию запуска и Claude Code не отправлял `notifications/roots/list_changed`.

Эта переменная устанавливается в окружение server, а не в собственное окружение Claude Code, поэтому ссылка на нее через расширение `${VAR}` в `command` или `args` записи `.mcp.json` с областью действия проекта или записи server с локальной или пользовательской областью действия в `~/.claude.json` требует значения по умолчанию, такого как `${CLAUDE_PROJECT_DIR:-.}`. Конфигурации MCP, предоставляемые плагинами, подставляют `${CLAUDE_PROJECT_DIR}` напрямую и не требуют значения по умолчанию.

```bash theme={null}
# Базовый синтаксис
claude mcp add [options] <name> -- <command> [args...]

# Реальный пример: добавление Airtable server
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Важно: разделение аргументов server с помощью `--`**

  Для stdio servers `--` (двойной дефис) разделяет собственные опции Claude, такие как `--transport`, `--env` и `--scope`, от команды и аргументов, которые запускают server. Все, что идет после `--`, передается server без изменений.

  Например:

  * `claude mcp add --transport stdio myserver -- npx server` → запускает `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → запускает `python server.py --port 8080` с `KEY=value` в окружении

  Без `--` Claude Code попытался бы разобрать флаги server, такие как `--port` выше, как свои собственные опции.

  `--env` принимает несколько пар `KEY=value`. Если имя server идет сразу после `--env`, CLI читает имя как еще одну пару и отклоняет его, поэтому поместите хотя бы одну другую опцию, такую как `--transport stdio`, между `--env` и именем server.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Вариант 4: Добавьте удаленный WebSocket server
</h3>

WebSocket servers поддерживают постоянное двусторонее соединение, которое подходит для удаленных MCP servers, которые отправляют события в Claude без запроса. Используйте HTTP вместо этого, когда ваш server только отвечает на запросы, так как HTTP поддерживает OAuth и флаг `claude mcp add --transport`, в то время как WebSocket не поддерживает ни то, ни другое.

Настройте WebSocket servers в `.mcp.json` или с помощью `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

Запись `type: "ws"` принимает те же поля `url`, `headers`, `headersHelper`, `timeout` и `alwaysLoad`, что и `http`. Аутентификация только через заголовки, поэтому передайте статический токен в `headers` или сгенерируйте его во время подключения с помощью [`headersHelper`](#use-dynamic-headers-for-custom-authentication). Флаг `claude mcp add --transport` не принимает `ws`.

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  Добавьте server из инструкций по настройке, написанных для другого клиента
</h3>

MCP servers не специфичны для Claude Code, поэтому инструкции по настройке server могут быть написаны для Claude Desktop, Cursor или другого MCP клиента и не содержать команду `claude mcp add`. Чтобы добавить server в любом случае, посмотрите в этих инструкциях одно из этих трех:

* **URL**, такой как `https://mcp.example.com/mcp`: server удален.
* **Команда запуска**, такая как `npx -y @example/mcp-server`: server работает на вашей машине.
* **Блок JSON `mcpServers`**: конфигурация, написанная для файла параметров другого клиента.

Каждый из них является одним из входов, которые четыре варианта в [Установка MCP servers](#installing-mcp-servers) принимают. Найдите форму, которую вы имеете ниже, чтобы превратить ее в команду, которую Claude Code принимает. Каждая команда записывает в [локальную область](#local-scope), если вы не добавите `--scope project` или `--scope user`.

<h4 id="from-a-url">
  Из URL
</h4>

URL означает, что server удален. Для endpoint `https://`, добавьте его с помощью `--transport http`, или следуйте [Вариант 2](#option-2-add-a-remote-sse-server), когда инструкции говорят, что endpoint использует SSE. Для endpoint `wss://`, используйте [Вариант 4](#option-4-add-a-remote-websocket-server) вместо этого, так как `--transport` не принимает `ws`:

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

Если инструкции также дают API ключ или заголовок токена, передайте его с помощью `--header`, как показано в [Вариант 1](#option-1-add-a-remote-http-server).

<h4 id="from-an-npx-uvx-or-binary-command">
  Из команды `npx`, `uvx` или binary
</h4>

Команда запуска означает, что server работает как локальный stdio процесс. Поместите всю команду после `--`, чтобы Claude Code передал флаги, такие как `-y`, команде, которая запускает server, вместо того чтобы читать их как свои собственные опции. Передайте любые переменные окружения, которые инструкции просят, с помощью `--env`, после имени server и перед `--`:

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[Вариант 3](#option-3-add-a-local-stdio-server) полностью охватывает разделитель `--`.

<h4 id="from-an-mcpservers-json-block">
  Из блока JSON `mcpServers`
</h4>

Блок `mcpServers`, написанный для другого MCP клиента, такого как Claude Desktop, использует ключ-обертку и форму записи, которую Claude Code читает. Передайте `claude mcp add-json` объект внутри `mcpServers`, а не обертку. Две записи нуждаются в исправлении сначала:

* **`url` без `type`**: добавьте `"type": "http"`, `"type": "sse"` или `"type": "ws"` для соответствия endpoint. Claude Code читает запись без `type` как stdio server, поэтому запись `url` без `type` не работает.
* **Ключ с символами, отличными от букв, цифр, дефисов и подчеркиваний**: выберите имя server, которое использует только эти символы. В противном случае ключ является именем server.

Например, этот блок:

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

становится этой командой:

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[Добавьте MCP servers из конфигурации JSON](#add-mcp-servers-from-json-configuration) охватывает экранирование shell и флаг `--scope` для `add-json`. Чтобы поделиться server с вашей командой вместо этого, добавьте `--scope project`, или добавьте запись под `mcpServers` в `.mcp.json` в корне вашего проекта и зафиксируйте ее. [Область действия проекта](#project-scope) охватывает, как Claude Code загружает и одобряет этот файл.

Каждая команда `claude mcp add` и `claude mcp add-json` выводит строку `Added ...`. Чтобы проверить, что Claude Code подключился, запустите `claude mcp get <name>`; [Статус server](#server-status) охватывает статусы, которые он показывает, и шаг одобрения для servers `.mcp.json`.

<h3 id="managing-your-servers">
  Управление вашими servers
</h3>

После настройки вы можете управлять своими MCP servers с помощью этих команд:

```bash theme={null}
# Список всех настроенных servers
claude mcp list

# Получить детали для конкретного server
claude mcp get notion

# Удалить server
claude mcp remove notion

# (в Claude Code) Проверить статус server
/mcp
```

Когда вы удаляете удаленный server, Claude Code также удаляет OAuth токены и регистрацию клиента, которые он хранил для этого server.

<h4 id="server-status">
  Статус server
</h4>

`claude mcp add` подтверждает успешное добавление, выводя строку `Added ...`, что означает, что конфигурация была записана. `claude mcp list` затем показывает статус здоровья рядом с каждым server, который он перечисляет, такой как `✔ Connected`, `! Needs authentication` или `✘ Failed to connect`. Статус отказа означает, что Claude Code не мог подключиться к этому server, а не то, что команда list не удалась.

Статусы в этом списке сообщают о решении конфигурации, а не о попытке подключения, поэтому Claude Code выводит их без подключения к server:

* ``⏸ Pending approval (run `claude` to approve)``: server с областью действия проекта из `.mcp.json`, который вы еще не одобрили. Claude Code показывает его как в `claude mcp list`, так и в `claude mcp get <name>`. Запустите `claude` интерактивно, чтобы просмотреть и одобрить его.
* `✘ Rejected (see disabledMcpjsonServers in settings)`: server `.mcp.json`, который запись [`disabledMcpjsonServers`](/docs/ru/settings-reference#disabledmcpjsonservers) отклоняет. Claude Code показывает его только в `claude mcp get <name>`.
* `⊘ Disabled for this project (re-enable via /mcp)`: server, который список [`disabledMcpServers`](#disable-a-server-without-removing-it) проекта называет. Claude Code показывает его как в `claude mcp list`, так и в `claude mcp get <name>`. Включите server обратно из панели `/mcp`. До версии 2.1.238 обе команды подключались к отключенному server для проверки здоровья и сообщали результат подключения.

WebSocket servers не отображаются в выводе `claude mcp list`. Используйте `claude mcp get <name>` или панель `/mcp` для их проверки.

<h4 id="project-server-approvals-and-workspace-trust">
  Одобрения servers проекта и доверие рабочей области
</h4>

Начиная с версии 2.1.196, `claude mcp list` и `claude mcp get` читают одобрения `.mcp.json` только из файлов параметров, которые не проверены в репозитории, пока вы не доверите рабочей области, запустив `claude` в ней и приняв диалог доверия рабочей области. Клонированный репозиторий не может одобрить свои собственные servers: [`enableAllProjectMcpServers`](/docs/ru/settings-reference#enableallprojectmcpservers) или [`enabledMcpjsonServers`](/docs/ru/settings-reference#enabledmcpjsonservers), зафиксированные в `.claude/settings.json` проекта, игнорируются в недоверенной папке, и server остается в состоянии `⏸ Pending approval` вместо подключения и проверки здоровья.

Одобрения из этих источников все еще применяются в недоверенной папке:

* ваш пользовательский `~/.claude/settings.json`
* управляемые параметры
* параметры, переданные с помощью `--settings`

Claude Code также применяет одобрения из неотслеживаемого `.claude/settings.local.json`, но он запускает git для проверки того, отслеживается ли файл, и выполняет эту проверку только в [доверенной папке](/docs/ru/permissions#project-allow-rules-and-workspace-trust). В папке, которой вы никогда не доверяли, Claude Code ждет диалога доверия перед применением одобрений файла, если только папка не является вашей конфигурационной домашней директорией: вашей домашней директорией или директорией, чей `.claude` вы установили как [`CLAUDE_CONFIG_DIR`](/docs/ru/env-vars). До версии 2.1.207 Claude Code применял одобрения из неотслеживаемого `.claude/settings.local.json` даже в папке, которой вы никогда не доверяли.

Запись `disabledMcpjsonServers` в любом файле параметров все еще отклоняет server.

<h4 id="server-status-detail">
  Деталь статуса server
</h4>

В `/mcp`, включая меню server там, и в [менеджере `/plugin`](/docs/ru/plugins/install), удаленный HTTP или SSE server, который вы использовали раньше, может показать статус `cached`, такой как `cached 2h ago · connects on first use · 5 tools`. Claude Code загрузил список инструментов server из своего кэша обнаружения, сохраненного в предыдущем сеансе, вместо подключения при запуске, и Claude Code подключает server в первый раз, когда Claude вызывает один из инструментов server. Инструменты доступны с вашего первого сообщения, поэтому вам не нужно ничего делать. Кэш обнаружения и его статус `cached` требуют Claude Code версии 2.1.221 или позже.

Кэш обнаружения отключен по умолчанию, если постепенный rollout не включил его для вашей учетной записи. Установите [`MCP_DISCOVERY_CACHE=1`](/docs/ru/env-vars) для его включения или `0` для его отключения, даже если rollout включил его. До версии 2.1.238 кэш был включен по умолчанию.

Два действия в меню server в `/mcp` также влияют на запись кэша этого server:

* **Reconnect**: на `cached` server Claude Code подключает его сейчас, а не при его первом вызове инструмента, и сохраняет запись. На подключенном или неудачном server Claude Code переподключает его и также отбрасывает запись.
* **Clear authentication**: Claude Code отзывает аутентификацию server и также отбрасывает запись.

После отбрасывания записи Claude Code получает список инструментов server из server вместо кэша.

Когда статус server — `✘ Failed to connect`, `claude mcp list` добавляет деталь отказа к этой строке статуса, и `claude mcp get <name>` показывает ее на строке `Issue:`: HTTP статус или код ошибки, плюс любой текст ошибки, который server вернул. Представление деталей server в `/mcp` включает тот же текст, сообщенный server, в его строке `Issue:`. Claude Code редактирует текст, похожий на учетные данные, из этой детали и никогда не включает развернутый URL server, который может содержать секреты. Claude Code не добавляет деталь к статусу `✘ Connection error`, потому что текст исключения, который он выводил бы там, может встроить этот URL. До версии 2.1.219 обе команды показывали только статус отказа, без кода статуса или текста ошибки server.

Когда вы завершаете аутентификацию из `/mcp` и подключение все еще не удается с HTTP статусом или кодом ошибки транспорта, Claude Code добавляет этот код и источник URL server к сообщению, которое он выводит после попытки. Источник — это схема и хост, плюс порт, когда URL называет один, такой как `https://mcp.example.com`.

* Путь и запрос никогда не появляются в этом сообщении.
* Для server в локальной, проектной или пользовательской [области](#mcp-installation-scopes) или в управляемой конфигурации MCP источник показывает хост, как написано в этой конфигурации, поэтому ссылка `${VAR}` в хосте не развернута в сообщении.
* Для отказа без статуса или кода ошибки Claude Code показывает текст ошибки без источника.

Удаленный server, конфигурация которого имеет пустой `url`, отображается как `not configured` в `/mcp`, в `claude mcp list` и в [менеджере `/plugin`](/docs/ru/plugins/install), и Claude Code не пытается подключиться к нему. Плагин может включать запись-заполнитель, подобную этой, для коннектора, который вы настроите позже, поэтому Claude Code не сообщает об этом как об ошибке или проблеме настройки. Представление деталей server в `/mcp` читает `No URL configured for this server`; установите `url` записи для подключения. До версии 2.1.208 Claude Code сообщал об пустом `url` как о проблеме конфигурации с подсказкой переподключиться.

<h4 id="configuration-warnings">
  Предупреждения конфигурации
</h4>

Claude Code предупреждает о проблемах конфигурации ниже. Каждая запись говорит, что Claude Code проверяет и как очистить предупреждение:

* **Скрытое пробельное пространство**: Claude Code предупреждает, когда значение конфигурации MCP содержит скрытое ведущее или конечное пробельное пространство, которое часто поступает из вставки токена с конечным переводом строки. Claude Code проверяет `command`, `url`, каждую запись `args` и значения и имена ключей под `env` и `headers`. Claude Code показывает предупреждение в выводе `claude mcp list` и в `/mcp`, называя затронутые поля без повторения их значений, например `Leading or trailing whitespace in: headers.Authorization`. Claude Code не обрезает пробельное пространство и использует значения точно так, как написано, поэтому отредактируйте конфигурацию, чтобы удалить его.
* **Одно имя в более чем одной области**: если вы определяете одно имя server в более чем одной [области](#mcp-installation-scopes) с разными endpoint, Claude Code предупреждает о конфликте в выводе `claude mcp list` и в `/mcp`. Claude Code хранит OAuth входы для каждого endpoint, поэтому когда вы аутентифицируете определение, которое загружается в одном проекте, вам все еще нужно войти отдельно в проекте, где загружается другое определение. Сохраните endpoint, который вы хотите, и удалите остальные с помощью `claude mcp remove <name> --scope <scope>`. В предупреждении Claude Code цитирует endpoint каждой области, как написано в вашей конфигурации, с неразвернутыми ссылками [`${VAR}`](#environment-variable-expansion-in-mcp-json), поэтому он никогда не показывает разрешенное значение, такое как API ключ.
* **Зарезервированные имена**: Claude Code резервирует имена своих встроенных servers, включая `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview` и `Claude Browser`. Если ваша конфигурация определяет server с зарезервированным именем, Claude Code пропускает его при загрузке и показывает предупреждение с просьбой переименовать его. `claude mcp add` отклоняет зарезервированное имя с ошибкой. `Claude Preview` и `Claude Browser` оба называют встроенный server, который использует [панель предпросмотра приложения Claude Code desktop](/docs/ru/desktop#preview-your-app). До версии 2.1.205 `Claude Browser` не был зарезервирован, поэтому пользовательский server мог зарегистрироваться под этим именем.
* **Отсутствующая переменная окружения**: если ссылка [`${VAR}`](#environment-variable-expansion-in-mcp-json) в конфигурации server называет переменную, которая не установлена и не имеет `:-default`, Claude Code предупреждает в выводе `claude mcp list` и в `/mcp`, называя переменную, и все еще загружает server с текстом `${VAR}` неразвернутым. Установите переменную или добавьте fallback `${VAR:-default}`. В URL и `headers` удаленного server некоторые переменные учетных данных [читаются как пустые](#credential-variables-that-read-as-empty) вместо этого, без предупреждения.

<h4 id="tool-availability">
  Доступность инструментов
</h4>

Панель `/mcp` показывает количество инструментов рядом с каждым подключенным server и отмечает servers, которые объявляют возможность tools, но не предоставляют никаких инструментов.

Если ваш запрос требует инструментов от server, который все еще подключается в фоновом режиме, Claude ждет подключения этого server перед продолжением. Как происходит ожидание, зависит от вашей конфигурации:

* **С [поиском инструментов](#scale-with-mcp-tool-search), по умолчанию**: ожидание происходит внутри вызова `ToolSearch`.
* **Без поиска инструментов**: Claude использует инструмент `WaitForMcpServers` вместо этого. Конфигурации без поиска инструментов включают пользовательский `ANTHROPIC_BASE_URL`, `ENABLE_TOOL_SEARCH=false` и модель более раннюю, чем поколение Claude 4.5 на Google Cloud's Agent Platform.
* **На развертывании Microsoft Foundry [размещенном на Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**: Claude начинает на пути поиска инструментов, а не с `WaitForMcpServers`, так как Claude Code обнаруживает отказ server-side развертывания только из API. После того как Claude Code переключит это развертывание на [upfront loading](#scale-with-mcp-tool-search), инструменты от server, который завершает подключение, становятся доступными при следующем запросе Claude.

С включенным поиском инструментов, когда server завершает подключение, пока Claude работает, Claude Code перечисляет имена инструментов server для Claude при его следующем запросе в том же ходу. Claude затем может искать и вызывать эти инструменты без ожидания вашего следующего сообщения.

<h3 id="disable-a-server-without-removing-it">
  Отключите server без его удаления
</h3>

Переключите server в панели `/mcp`, чтобы остановить Claude Code от подключения к нему без потери его конфигурации. Claude Code все еще перечисляет server в `/mcp`, отмеченный как отключенный.

Когда вы переключаете server, Claude Code записывает ваш выбор для каждого проекта в `~/.claude.json`, в одном из двух списков, которые охватывают непересекающиеся наборы servers:

* `disabledMcpServers`: список opt-out для пользовательских servers, plugin servers, servers, которые ваша организация [предоставляет через управляемые параметры](/docs/ru/managed-mcp#provide-servers-through-managed-settings), claude.ai коннекторов, которые Claude Code [получает сам](#how-connectors-reach-claude-code), и встроенных servers, которые по умолчанию включены. Claude Code не подключается к server, который вы перечисляете здесь. Когда вы отключаете claude.ai коннектор с помощью переключателя `/mcp` для каждого проекта, описанного в [Отключите claude.ai коннекторы](#disable-claude-ai-connectors), Claude Code записывает его в этот список под его отображаемым именем, например `claude.ai Slack`.
* `enabledMcpServers`: список opt-in для встроенных servers, которые по умолчанию отключены, такие как `computer-use`. Claude Code подключается к server, который по умолчанию отключен, только когда вы перечисляете его здесь.

Claude Code консультирует ровно один из двух списков для каждого server, поэтому ни один список не переопределяет другой. Если вы добавляете обычный server в `enabledMcpServers` или server, который по умолчанию отключен, в `disabledMcpServers`, Claude Code игнорирует запись.

`disabledMcpServers` и `enabledMcpServers` не связаны с [`enabledMcpjsonServers`](/docs/ru/settings-reference#enabledmcpjsonservers) и [`disabledMcpjsonServers`](/docs/ru/settings-reference#disabledmcpjsonservers), которые контролируют одобрение servers, определенных в файле `.mcp.json` проекта.

<h3 id="mcp-client-runtimes">
  MCP client runtimes
</h3>

Claude Code подключается к MCP servers через один из двух client runtimes. Runtime v1 построен на MCP TypeScript SDK 1.x. Runtime v2 — это тот же код на [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/), который добавляет MCP protocol revision 2026-07-28. Остальная часть этой страницы применяется к обоим runtimes, кроме случаев, когда раздел называет runtime v2.

Claude Code выбирает runtime каждый раз, когда вы его запускаете, и сохраняет его до выхода. В сеансах, где он [получает feature flags](/docs/ru/env-vars#features-that-need-feature-flag-fetching), он использует runtime v2 на Claude Code версии 2.1.232 или позже.

В сеансах, где он не получает feature flags, Claude Code использует runtime v2 по умолчанию на Claude Code версии 2.1.274 или позже:

* Сеансы на Amazon Bedrock, Claude Platform на AWS, Google Cloud's Agent Platform или Microsoft Foundry, если платформа-хост, которая встраивает Claude Code, не установит [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ru/env-vars)
* Сеансы, вошедшие через [Claude apps gateway](/docs/ru/claude-apps-gateway)
* Сеансы, где вы отключаете телеметрию или получение feature-flag, например с `DISABLE_TELEMETRY`

На v2 Claude Code также:

* Спрашивает HTTP servers, поддерживают ли они более новую revision, и использует ее с теми, которые это делают. Он также спрашивает claude.ai connector servers в сеансах, где он получает feature flags. Чтобы он спрашивал stdio servers или connector servers в каждом сеансе, установите [`MCP_PROTOCOL_NEGOTIATION`](/docs/ru/env-vars) на `auto`. Он подключается к каждому другому server, как v1 это делает.
* Получает уведомления `list_changed` от servers на более новой revision через [stream, который он держит открытым](#notification-streams-on-the-v2-runtime).
* Не регистрирует [channel](#push-messages-with-channels) server, который подключается на более новой revision, потому что эта revision не может переносить channel сообщения.
* Не удается [MCP OAuth sign-in](#authenticate-with-remote-mcp-servers), чей ответ авторизации называет неожиданный issuer.

Anthropic может сохранить конкретный server на более ранней protocol или вне этого stream с помощью feature flag, который Claude Code получает.

Чтобы выбрать runtime самостоятельно, установите [`MCP_SDK_GENERATION`](/docs/ru/env-vars) на `v1` или `v2`. Чтобы решить, спрашивает ли Claude Code, установите [`MCP_PROTOCOL_NEGOTIATION`](/docs/ru/env-vars) на `auto` или `legacy`.

<h3 id="dynamic-tool-updates">
  Динамические обновления инструментов
</h3>

Claude Code поддерживает MCP `list_changed` уведомления, позволяя MCP servers динамически обновлять свои доступные инструменты, подсказки и ресурсы без необходимости отключения и переподключения. Когда MCP server отправляет уведомление `list_changed`, Claude Code автоматически обновляет доступные возможности от этого server.

Если запрос обновления не удается, Claude Code сохраняет ранее обнаруженные инструменты, подсказки и ресурсы server до тех пор, пока более позднее обновление не удастся. До версии 2.1.214 временная ошибка во время обновления заменяла инструменты, подсказки и ресурсы server пустым списком.

<h4 id="notification-streams-on-the-v2-runtime">
  Notification streams на runtime v2
</h4>

На [runtime v2](#mcp-client-runtimes) Claude Code получает уведомления `list_changed` от server на более новой protocol revision через stream, который он держит открытым. Когда stream закрывается, Claude Code переоткрывает его с двумя ограничениями:

* **Stream закрывается снова в течение 10 секунд**: Claude Code переоткрывает его до трех раз, затем останавливается для этого подключения.
* **Stream остается открытым дольше 10 секунд, затем закрывается**, как streams к serverless хостам обычно делают: после пяти переоткрытий в час Claude Code ждет примерно шесть часов перед следующим.

До переоткрытия stream вы сохраняете последний полученный инструменты, подсказки и ресурсы server. Чтобы подобрать его изменения раньше, переподключите server из `/mcp`.

<h3 id="automatic-reconnection">
  Автоматическое переподключение
</h3>

Claude Code переподключает удаленный server, который отключается во время сеанса, и повторяет попытку первого подключения HTTP или SSE server после временной ошибки. Stdio servers — это локальные процессы, и Claude Code не переподключает их автоматически.

<h4 id="mid-session-drops-of-a-remote-server">
  Mid-session drops удаленного server
</h4>

Claude Code переподключает отключенный удаленный server с экспоненциальной задержкой: до пяти попыток, начиная с задержки в одну секунду и удваивая ее каждый раз. Что вы видите, зависит от того, как вы запускаете Claude Code:

* **В интерактивном сеансе**: `/mcp` показывает server как ожидающий, пока Claude Code переподключается. После пяти неудачных попыток Claude Code помечает server как неудачный или как нуждающийся в аутентификации, когда server нуждается в авторизации снова. Вы можете повторить попытку вручную из `/mcp`.
* **В [`claude -p`](/docs/ru/headless) запусках и [Agent SDK](/docs/ru/agent-sdk/overview) сеансах**: Claude Code переподключается по тому же расписанию, без панели `/mcp` для показа попыток.

<h4 id="failed-first-connections">
  Неудачные первые подключения
</h4>

Когда первое подключение HTTP или SSE server не удается с временной ошибкой, такой как ответ 5xx, отказ в соединении или timeout, Claude Code повторяет попытку до трех раз. Если подключение все еще не удается, Claude Code помечает server как неудачный. Claude Code повторяет попытку таким образом при запуске и когда server добавляется во время сеанса. Это включает server, который Claude Code добавляет к [cloud session](/docs/ru/claude-code-on-the-web) из его конфигурации, и server, который вы добавляете с помощью Agent SDK's [`setMcpServers()`](/docs/ru/agent-sdk/typescript).

Claude Code не повторяет попытку в этих случаях:

* Первое подключение WebSocket server
* Ошибка аутентификации или не найдено, потому что это требует изменения конфигурации для разрешения. Когда [`headersHelper`](#use-dynamic-headers-for-custom-authentication) является единственным источником server заголовка `Authorization`, Claude Code повторяет попытку ошибки аутентификации в любом случае, потому что он повторно запускает helper при каждой попытке и может подобрать свежие учетные данные

<h4 id="failed-discovery-requests">
  Неудачные запросы обнаружения
</h4>

После подключения server Claude Code отправляет ему запросы обнаружения возможностей, такие как `tools/list`, `prompts/list` и `resources/list`. Claude Code повторяет эти запросы до трех раз с короткой задержкой после временной сетевой или серверной ошибки. Он не повторяет ошибки аутентификации, ответы 4xx или timeout запросов.

<h4 id="how-claude-learns-that-a-server-failed">
  Как Claude узнает, что server не удался
</h4>

Зависит ли то, что Claude Code сообщает Claude о настроенном server, который не удалось подключить, от [поиска инструментов](#scale-with-mcp-tool-search), который включен по умолчанию:

* С поиском инструментов Claude Code сообщает Claude, какой server не удалось подключить и его ошибку подключения, поэтому Claude сообщает об ошибке подключения в своем ответе. Claude Code включает ту же информацию в результаты `ToolSearch`, которые не находят соответствующий инструмент.
* В любой [конфигурации без поиска инструментов](#configure-tool-search) Claude Code не сообщает Claude об ошибках подключения неудачных servers.

<h3 id="push-messages-with-channels">
  Отправка сообщений через каналы
</h3>

MCP server также может отправлять сообщения непосредственно в вашу сессию, чтобы Claude мог реагировать на внешние события, такие как результаты CI, оповещения мониторинга или сообщения чата. Чтобы включить это, ваш server объявляет возможность `claude/channel` и вы включаете ее с флагом `--channels` при запуске. См. [Каналы](/docs/ru/channels) для использования официально поддерживаемого канала или [Справочник каналов](/docs/ru/channels-reference) для создания собственного.

На [runtime v2](#mcp-client-runtimes), если вы установите [`MCP_PROTOCOL_NEGOTIATION`](/docs/ru/env-vars) на `auto` и channel server согласует MCP protocol revision 2026-07-28, он не может доставлять channel сообщения, поэтому Claude Code не регистрирует его как канал. Оставление переменной неустановленной или установка ее на `legacy` сохраняет stdio servers на более ранней handshake.

<Tip>
  Советы:

  * Используйте флаг `-s` или `--scope` для указания места хранения конфигурации:
    * `local` (по умолчанию): доступно только вам в текущем проекте
    * `project`: общий доступ для всех в проекте через файл `.mcp.json`
    * `user`: доступно вам во всех проектах
  * Установите переменные окружения с флагами `-e` или `--env` (например, `-e KEY=value`)
  * Флаги `--transport` и `--header` также принимают короткие формы `-t` и `-H`
  * Настройте timeout запуска MCP server, используя переменную окружения `MCP_TIMEOUT` (например, `MCP_TIMEOUT=10000 claude` устанавливает timeout в 10 секунд)
  * Установите timeout выполнения инструмента для каждого server, добавив поле `timeout` в миллисекундах в запись этого server в `.mcp.json`, например `"timeout": 600000` для десяти минут. Это переопределяет переменную окружения `MCP_TOOL_TIMEOUT` только для этого server
  * Claude Code отобразит предупреждение, когда выход инструмента MCP превышает 10 000 токенов и ограничивает выход до 25 000 токенов по умолчанию. Чтобы увеличить этот лимит, установите переменную окружения `MAX_MCP_OUTPUT_TOKENS` (например, `MAX_MCP_OUTPUT_TOKENS=50000`); порог предупреждения фиксирован. См. [Лимиты выхода MCP и предупреждения](#mcp-output-limits-and-warnings)
  * Используйте `/mcp` для аутентификации с удаленными servers, которые требуют аутентификацию OAuth 2.0
</Tip>

Timeout для каждого server — это жесткий лимит реального времени для каждого вызова инструмента, и уведомления о прогрессе от server не продлевают его. Значения ниже 1000 игнорируются и переходят к `MCP_TOOL_TIMEOUT`, или к его значению по умолчанию примерно 28 часов, когда эта переменная не установлена. Для HTTP, SSE или [claude.ai connector](/docs/ru/mcp#use-mcp-servers-from-claude-ai) server существует также второй таймер для каждого запроса, который охватывает каждый запрос до первого байта ответа server. Claude Code устанавливает этот таймер на наибольшее из трех значений: 60 секунд, timeout инструмента, который применяется к server, и `MCP_TIMEOUT`. Значение по умолчанию 28 часов неустановленного `MCP_TOOL_TIMEOUT` не входит в это сравнение, и значение ниже 60 секунд не сокращает таймер. Stdio и WebSocket servers не имеют таймера для каждого запроса.

Timeout для каждого server не менее 1000 также действует как нижний предел для timeout неактивности, описанного ниже: Claude Code никогда не прерывает вызовы инструментов этого server из-за неактивности раньше, чем timeout для каждого server. Требуется Claude Code версии 2.1.203 или позже.

Вызов инструмента к MCP server, который не отправляет ответ и не отправляет уведомление о прогрессе в течение окна неактивности, прерывается с ошибкой вместо ожидания лимита реального времени. Он применяется к каждому типу server, кроме IDE servers и SDK in-process servers. Окно неактивности по умолчанию составляет пять минут для HTTP, SSE, WebSocket и [claude.ai connector](#use-mcp-servers-from-claude-ai) servers, и 30 минут для stdio servers. До версии 2.1.203 stdio servers были освобождены от timeout неактивности.

Установите переменную окружения [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/ru/env-vars) в миллисекундах, чтобы изменить окно неактивности, или установите ее на `0`, чтобы отключить проверку.

Эти timeouts ограничивают, как долго вызов может работать, не всегда как долго он блокирует сеанс: вызов основного разговора, который работает более двух минут, сначала переходит в фоновую задачу. См. [Автоматическое фоновое выполнение долгих вызовов инструментов](#automatic-backgrounding-of-long-tool-calls).

<h3 id="automatic-backgrounding-of-long-tool-calls">
  Автоматическое фоновое выполнение долгих вызовов инструментов
</h3>

Вызов инструмента MCP в основном разговоре, который все еще работает после двух минут, переходит в фоновую задачу вместо блокирования сеанса. Claude получает ID задачи немедленно и продолжает работать, и результат приходит как уведомление задачи, когда вызов завершается. Автоматическое фоновое выполнение требует Claude Code версии 2.1.212 или позже.

Задача отображается в [`/tasks`](/docs/ru/commands#all-commands), где вы также можете ее остановить, и она не сохраняется при выходе из сеанса. Лимиты для каждого вызова все еще применяются, пока вызов работает в фоне: лимит реального времени, установленный timeout для каждого server или [`MCP_TOOL_TIMEOUT`](/docs/ru/env-vars), и timeout неактивности, установленный [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/ru/env-vars).

Установите переменную окружения [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/ru/env-vars) в миллисекундах, чтобы изменить порог, или установите ее на `0`, чтобы отключить автоматическое фоновое выполнение. Установка `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` на `1` также отключает его, наряду со всеми другими функциями фоновых задач.

Некоторые вызовы никогда не переходят в фон:

* Вызовы от [subagents](/docs/ru/sub-agents); Claude Code фоновое выполнение только вызовов основного разговора
* Вызовы к IDE servers
* Вызовы в [non-interactive mode](/docs/ru/headless), если `CLAUDE_AUTO_BACKGROUND_TASKS` не установлен на `1`, так как одноразовый запуск может завершиться до прихода результата

Вызов, ожидающий открытого [диалога elicitation](#respond-to-mcp-elicitation-requests), не фоновое выполнение, пока диалог открыт; server заблокирован на вашем вводе, а не медленный, поэтому Claude Code откладывает перемещение до закрытия диалога.

<h3 id="plugin-provided-mcp-servers">
  Plugin-provided MCP servers
</h3>

[Плагины](/docs/ru/plugins/overview) могут включать MCP servers, которые предоставляют инструменты и интеграции при включении плагина. Plugin MCP servers работают идентично пользовательским настроенным servers.

**Как работают plugin MCP servers**:

* Плагины определяют MCP servers в `.mcp.json` в корне плагина или встроенные в `plugin.json`
* Когда вы включаете плагин, Claude Code автоматически запускает его MCP servers
* Claude Code предлагает plugin MCP tools рядом с вручную настроенными MCP tools
* Вы добавляете и удаляете plugin servers путем установки или удаления плагина, а не с помощью команд `/mcp`. Вы все еще можете [отключить установленный plugin server](#disable-a-server-without-removing-it) в `/mcp`, что останавливает Claude Code от подключения к нему без удаления плагина

**Пример конфигурации plugin MCP**:

В `.mcp.json` в корне плагина:

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

Или встроенные в `plugin.json`:

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Функции plugin MCP**:

* **Автоматический жизненный цикл**: servers подключаются и отключаются в этих точках:
  * При запуске сеанса Claude Code автоматически подключает servers для включенных плагинов. В `/mcp` удаленный (HTTP или SSE) plugin server, который вы использовали раньше, может показать статус [`cached`](#server-status-detail) вместо этого; Claude Code подключает его, когда Claude впервые вызывает один из его инструментов
  * Если вы включаете или отключаете плагин во время сеанса, Claude Code подключает или отключает его MCP servers, когда изменение применяется. [Применить изменения плагина без перезагрузки](/docs/ru/plugins/cli-reference#reload-plugins) описывает, когда это происходит. В сеансе без интерактивного терминала `/reload-plugins` не подключает или отключает plugin MCP servers; эти изменения вступают в силу в вашем следующем сеансе
  * Когда вы перезагружаете, Claude Code сохраняет живые подключения plugin servers, конфигурация которых не изменилась, и делает то же самое, когда вы [заменяете список MCP servers сеанса](/docs/ru/agent-sdk/typescript#mcpsetserversresult) из Agent SDK без их названия
  * Когда вы [перемещаете сеанс с `/cd`](/docs/ru/permissions#move-the-session-to-another-directory) на версии 2.1.246 или позже, Claude Code подключает servers плагинов, которые включают параметры новой директории, и отключает servers плагинов, которые больше не включены, поэтому вам не нужно запускать `/reload-plugins` после перемещения
  * В [cloud sessions](/docs/ru/claude-code-on-the-web) вызов MCP к plugin server, который еще не подключен, такой как сразу после пробуждения неактивного сеанса, запускает server по требованию и ждет его подключения
* **Заполнители пути**: `${CLAUDE_PLUGIN_ROOT}` разрешается в директорию установки плагина, `${CLAUDE_PLUGIN_DATA}` в его [директорию постоянного состояния](/docs/ru/plugins/components#path-variables-and-persistent-data), и `${CLAUDE_PROJECT_DIR}` в стабильный корень проекта. Подстановка применяется к:
  * `stdio` servers: `command`, `args`, `env`
  * `http`, `sse` и `ws` servers: `url`, `headers` и `headersHelper`. До версии 2.1.195 `headersHelper` передавал заполнитель как буквальную строку
* **Доступ к переменным окружения пользователя**: доступ к тем же переменным окружения, что и вручную настроенные servers
* **Несколько типов транспорта**: поддержка stdio, SSE, HTTP и WebSocket транспортов, хотя поддержка транспорта может варьироваться в зависимости от server

Plugin servers отображаются в `/mcp` с индикаторами, показывающими, что они поступают из плагинов.

**Имена plugin MCP tools**:

Инструменты из MCP server, включенного в плагин, содержат как имя плагина, так и ключ server в их вызываемом имени. Полная форма — `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, где любой символ вне `A-Z`, `a-z`, `0-9`, `_` и `-` заменяется на `_`. Для server `database-tools`, включенного в плагин с именем `my-plugin`, инструмент `query` вызывается как:

```
mcp__plugin_my-plugin_database-tools__query
```

Используйте это полное имя при ссылке на инструмент в [правилах разрешений](/docs/ru/permissions), в списке `allowed-tools` skill, в [поле `tools` subagent](/docs/ru/sub-agents#available-tools) или в [matcher hook](/docs/ru/hooks#match-mcp-tools). Matcher hook, написанный против простого ключа server, такой как `mcp__database-tools__.*`, никогда не срабатывает для plugin-bundled server.

Server сам регистрируется под scoped именем `plugin:<plugin-name>:<server-name>`, такой как `plugin:my-plugin:database-tools`. Используйте это имя там, где ожидается имя настроенного server, такой как [поле `server` hook `mcp_tool`](/docs/ru/hooks#mcp-tool-hook-fields).

См. [справочник компонентов плагина](/docs/ru/plugins/components#mcp-servers) для получения подробной информации о включении MCP servers в плагины.

<h2 id="mcp-installation-scopes">
  Области установки MCP
</h2>

MCP servers можно настроить на трех различных уровнях области. Область, которую вы выбираете, контролирует, в каких проектах загружается server и является ли конфигурация общей с вашей командой. Администраторы также могут развертывать или предоставлять servers для каждого пользователя через [управляемую конфигурацию](#managed-mcp-configuration).

| Область                     | Загружается в         | Общий доступ с командой   | Хранится в                  |
| --------------------------- | --------------------- | ------------------------- | --------------------------- |
| [Локальная](#local-scope)   | Только текущий проект | Нет                       | `~/.claude.json`            |
| [Проект](#project-scope)    | Только текущий проект | Да, через контроль версий | `.mcp.json` в корне проекта |
| [Пользователь](#user-scope) | Все ваши проекты      | Нет                       | `~/.claude.json`            |

<h3 id="local-scope">
  Локальная область
</h3>

Локальная область — это область по умолчанию. Server с локальной областью загружается только в проекте, где вы его добавили, и остается приватным для вас. Claude Code хранит его в `~/.claude.json` в пути вашего проекта, поэтому один и тот же server не будет отображаться в ваших других проектах. Используйте локальную область для личных development servers, экспериментальных конфигураций или servers с учетными данными, которые вы не хотите в контроле версий.

<Note>
  Термин "локальная область" для MCP servers отличается от общих локальных параметров. MCP servers с локальной областью хранятся в `~/.claude.json` (ваш домашний каталог), в то время как общие локальные параметры используют `.claude/settings.local.json` (в каталоге проекта). См. [Параметры](/docs/ru/settings#where-settings-live) для получения подробной информации о расположении файлов параметров.
</Note>

```bash theme={null}
# Добавить server с локальной областью (по умолчанию)
claude mcp add --transport http stripe https://mcp.stripe.com

# Явно указать локальную область
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

Команда записывает server в запись для вашего текущего проекта внутри `~/.claude.json`. Пример ниже показывает результат при запуске из `/path/to/your/project`:

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  Область проекта
</h3>

Servers с областью проекта позволяют командной работе, сохраняя конфигурации в файле `.mcp.json` в корневом каталоге вашего проекта. Когда вы добавляете server с областью проекта, Claude Code автоматически создает или обновляет этот файл с соответствующей структурой конфигурации. Проверьте `.mcp.json` в контроль версий, чтобы все члены вашей команды получили одни и те же MCP tools и сервисы.

```bash theme={null}
# Добавить server с областью проекта
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

Результирующий файл `.mcp.json` следует стандартизированному формату:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

По соображениям безопасности Claude Code запрашивает одобрение в интерактивных сеансах перед использованием servers с областью проекта из файлов `.mcp.json`. Чтобы сбросить эти выборы одобрения, запустите `claude mcp reset-project-choices`.

В запусках `claude -p`, сеансах [Agent SDK](/docs/ru/headless) и [облачных сеансах](/docs/ru/claude-code-on-the-web), Claude Code не может показать этот запрос: он загружает servers с областью проекта без запроса. Claude Code также пропускает запрос в сеансе, который вы запускаете в режиме `bypassPermissions` с [`skipDangerousModePermissionPrompt`](/docs/ru/settings-reference#skipdangerousmodepermissionprompt), установленным в ваших пользовательских параметрах или в управляемых параметрах. Чтобы все равно исключить server:

* Добавьте его в [`disabledMcpjsonServers`](/docs/ru/settings-reference#disabledmcpjsonservers), что блокирует его в каждом режиме разрешений.
* Исключите параметры проекта полностью с помощью [`--setting-sources`](/docs/ru/cli-reference#cli-flags) или опции `settingSources` SDK.
* Запустите сеанс с [`--strict-mcp-config`](/docs/ru/cli-reference#cli-flags). Claude Code затем использует только MCP servers, которые вы передаете с `--mcp-config`. Пропуск запроса одобрения для servers с областью проекта, которые Claude Code не загружает, требует Claude Code v2.1.246 или позже; до v2.1.246 строгий сеанс все еще ждал одобрения для них, что оставляло фоновые сеансы в ожидании при запуске. См. [Исключительный контроль с managed-mcp.json](/docs/ru/managed-mcp#exclusive-control-with-managed-mcp-json) для того, что флаг делает под управляемым файлом MCP.

[Одобрения servers проекта и доверие рабочей области](#project-server-approvals-and-workspace-trust) охватывает, как одобрения, зафиксированные в репозитории, взаимодействуют с доверием рабочей области.

<h3 id="user-scope">
  Область пользователя
</h3>

Servers с областью пользователя хранятся в `~/.claude.json` и обеспечивают доступность между проектами, делая их доступными во всех проектах на вашей машине, оставаясь приватными для вашей учетной записи пользователя. Эта область хорошо работает для личных utility servers, инструментов разработки или сервисов, которые вы часто используете в разных проектах.

```bash theme={null}
# Добавить server пользователя
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  Иерархия области и приоритет
</h3>

Когда один и тот же server определен в более чем одном месте, Claude Code подключается к нему один раз, используя определение из источника с наивысшим приоритетом. Вся запись server из этого источника используется; поля не объединяются между областями.

1. Локальная область
2. Область проекта
3. Область пользователя
4. [Plugin-provided servers](/docs/ru/plugins/components#mcp-servers)
5. [claude.ai connectors](#use-mcp-servers-from-claude-ai)

Три области совпадают дубликаты по имени. Плагины и соединители совпадают по конечной точке, поэтому тот, который указывает на тот же URL или команду, что и server выше, рассматривается как дубликат.

Server, который ваша организация предоставляет через управляемый параметр [`managedMcpServers`](/docs/ru/managed-mcp#provide-servers-through-managed-settings), занимает место выше всех этих, поэтому когда один из них дублирует его, Claude Code подключает определение организации. Требует Claude Code v2.1.259 или позже.

Если вы откроете локальный сеанс на [вкладке Code приложения Desktop](/docs/ru/desktop#mcp-servers-from-the-claude-desktop-chat-app) с одним и тем же именем stdio server на верхнем уровне `~/.claude.json` (область пользователя) и в `.mcp.json`, вкладка Code использует определение `~/.claude.json`.

<h3 id="environment-variable-expansion-in-mcp-json">
  Расширение переменных окружения в `.mcp.json`
</h3>

Claude Code поддерживает расширение переменных окружения в файлах `.mcp.json`, позволяя командам делиться конфигурациями, сохраняя гибкость для путей, специфичных для машины, и чувствительных значений, таких как ключи API.

<h4 id="supported-syntax">
  Поддерживаемый синтаксис
</h4>

* `${VAR}`: расширяется до значения переменной окружения `VAR`
* `${VAR:-default}`: расширяется до `VAR`, если установлена, иначе использует `default`

<h4 id="expansion-locations">
  Места расширения
</h4>

Переменные окружения могут быть расширены в:

* `command`: путь к исполняемому файлу server
* `args`: аргументы командной строки
* `env`: переменные окружения, передаваемые server
* `url`: для типов HTTP server
* `headers`: для аутентификации HTTP server

<h4 id="example-with-variable-expansion">
  Пример с расширением переменных
</h4>

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

<h4 id="unset-variables-without-a-default">
  Неустановленные переменные без значения по умолчанию
</h4>

Если требуемая переменная окружения не установлена и не имеет значения по умолчанию, конфигурация все еще загружается: Claude Code сообщает предупреждение об отсутствующей переменной для этого server в выводе `claude mcp list` и использует неразвернутый текст `${VAR}` как есть. Установите переменную или добавьте резервное значение `:-default`, чтобы server запустился с предполагаемым значением. В `url` и `headers` удаленного server некоторые переменные учетных данных [читаются как пустые](#credential-variables-that-read-as-empty) вместо этого, без предупреждения.

<h4 id="credential-variables-that-read-as-empty">
  Переменные учетных данных, которые читаются как пустые
</h4>

В `url` и `headers` удаленного server Claude Code читает переменные учетных данных из вашего окружения как пустые, а не расширяет их. Это предотвращает отправку конфигурации `.mcp.json` проекта или плагина ваших учетных данных Claude Code или облачного провайдера на server, который он называет. Если вы напишете `Bearer ${ANTHROPIC_AUTH_TOKEN}`, server получит `Bearer ` без учетных данных и отклонит запрос, обычно с `401`. Claude Code сообщает об этом как о неудачном подключении.

Охватываемые имена:

* Собственные учетные данные Claude Code, такие как `ANTHROPIC_API_KEY` и `ANTHROPIC_AUTH_TOKEN`
* Учетные данные вашего облачного провайдера, такие как `AWS_BEARER_TOKEN_BEDROCK`
* Другие учетные данные, которые несет ваше окружение, такие как `HTTPS_PROXY` и `NPM_TOKEN`

Охватываемое имя читается как пустое независимо от того, установили ли вы переменную, и резервное значение `:-default` на нем игнорируется. URL базы провайдера, такой как `ANTHROPIC_BASE_URL`, все еще расширяется, поэтому `"url": "${ANTHROPIC_BASE_URL}/mcp"` работает, если только значение URL не встраивает учетные данные, такие как имя пользователя и пароль.

Имя вне этого набора, такое как `API_KEY`, расширяется как написано. Чтобы дать server одно из охватываемых учетных данных, скопируйте его в переменную с именем вашего собственного и ссылайтесь на это имя вместо этого.

Когда `url` или `headers` удаленного server ссылаются на охватываемую переменную, которую вы установили, Claude Code называет ее в строке журнала отладки. Чтобы прочитать строку, запустите `claude --debug-file /tmp/claude-debug.log` и найдите в этом файле `never expanded toward a remote server`.

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  Как ссылки отображаются в `/mcp` и выводе CLI
</h4>

Для server в локальной, проектной или пользовательской [области](#mcp-installation-scopes) следующие поверхности показывают ссылку `${VAR}` по имени, а не как ее разрешенное значение:

* URL или командная строка в представлении деталей `/mcp` server
* Вывод `claude mcp list` и `claude mcp get`

Представление деталей `/mcp` показывает ссылки таким образом в Claude Code v2.1.268 или позже.

Для server, который ваша организация предоставляет через параметр `managedMcpServers`, эти поверхности показывают [только хост URL](/docs/ru/managed-mcp#what-users-can-see-and-change).

Чтобы проверить, что показывают `claude mcp list`, `claude mcp get` и `/mcp` при сбое подключения, см. [Деталь статуса server](#server-status-detail).

<h2 id="practical-examples">
  Практические примеры
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  Пример: подключение к GitHub для проверки кода
</h3>

GitHub's remote MCP server аутентифицируется с помощью токена личного доступа GitHub, переданного как заголовок. Чтобы получить его, откройте [параметры токена GitHub](https://github.com/settings/personal-access-tokens), создайте новый детальный токен с доступом к репозиториям, с которыми вы хотите, чтобы Claude работал, затем добавьте сервер:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Замените `YOUR_GITHUB_PAT` на ваш токен личного доступа. Команда `claude mcp add` сохраняет конфигурацию без проверки учетных данных, поэтому здесь принимается значение-заполнитель, но сервер не подключится позже. Чтобы проверить соединение, запустите `/mcp` и убедитесь, что сервер показывает `connected`. Сервер с неправильными учетными данными показывает `failed`, и детали сбоя включают HTTP-статус, возвращаемый сервером, например 401.

Затем работайте с GitHub:

```text wrap theme={null}
Проверьте PR #456 и предложите улучшения
```

```text wrap theme={null}
Создайте новую проблему для найденной нами ошибки
```

```text wrap theme={null}
Покажите мне все открытые PR, назначенные мне
```

<h3 id="example-query-your-postgresql-database">
  Пример: запрос к базе данных PostgreSQL
</h3>

[DBHub](https://github.com/bytebase/dbhub), пакет `@bytebase/dbhub`, — это MCP сервер, который подключает Claude к реляционной базе данных через строку подключения, которую вы передаете в `--dsn`. Используйте пользователя базы данных только для чтения в строке подключения, чтобы запросы, которые запускает Claude, не могли изменять данные:

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Чтобы подтвердить, что сервер запущен, запустите `/mcp` и убедитесь, что `db` показывает `connected`.

Затем запрашивайте вашу базу данных естественным образом:

```text wrap theme={null}
Какой у нас общий доход в этом месяце?
```

```text wrap theme={null}
Покажите мне схему для таблицы orders
```

```text wrap theme={null}
Найдите клиентов, которые не совершали покупку в течение 90 дней
```

<h2 id="authenticate-with-remote-mcp-servers">
  Аутентификация с удаленными MCP servers
</h2>

Многие облачные MCP servers требуют аутентификации. Claude Code поддерживает OAuth 2.0 для безопасных соединений.

Claude Code отмечает удаленный server как требующий аутентификации, когда server отвечает с `401 Unauthorized` или `403 Forbidden`. То, что показывает Claude Code, зависит от server:

* Для server, на который вы еще не вошли, любой из этих кодов состояния отмечает его в `/mcp`, чтобы вы могли завершить поток OAuth.
* Для [разъема claude.ai](#use-mcp-servers-from-claude-ai), `401`, вызванный отклонением claude.ai вашего токена сеанса, не отмечает разъем, потому что повторная авторизация разъема не может исправить вашу учетную запись. Claude Code показывает [состояние отклонения токена сеанса](/docs/ru/errors#claude-ai-rejected-the-session-token) вместо этого.
* Для server, чей заголовок `Authorization` вы настроили в `headers` или через [`headersHelper`](#use-dynamic-headers-for-custom-authentication), `401` или `403` при подключении не отмечает server, потому что учетные данные для исправления — это те, которые вы настроили. Claude Code сообщает о соединении как о неудачном вместо этого. Если вы установили этот заголовок из ссылки `${VAR}`, проверьте, является ли эта переменная одной из тех, которые Claude Code [читает как пустые](#credential-variables-that-read-as-empty).
* Для разъема [доставленного в облачный сеанс](#how-connectors-reach-claude-code), Claude Code не запускает поток входа, потому что прокси сеанса аутентифицируется с разъемом с авторизацией, которую вы предоставили в claude.ai. Когда разъем там требует авторизации снова, переподключите его на [claude.ai/customize/connectors](https://claude.ai/customize/connectors) вместо сеанса.

Когда запрос к OAuth server, на который вы уже вошли, возвращает `401 Unauthorized`, Claude Code обновляет сохраненный токен, переподключается и повторяет запрос один раз. Он отмечает server в `/mcp` только если этот повторный запрос также не удается. До v2.1.206 обновление токена, которое не удалось по временной причине, такой как ошибка сети, отмечало OAuth server как требующий аутентификации для остальной части сеанса, даже если его токен обновления был все еще действителен.

Когда server отклоняет сохраненный токен обновления, Claude Code немедленно показывает уведомление, указывающее на `/mcp`. Откройте `/mcp` и выберите **Re-authenticate** на server, чтобы снова войти перед следующим вызовом инструмента.

Пользовательский server, который возвращает заголовок `WWW-Authenticate`, указывающий на его сервер авторизации, получает то же автоматическое обнаружение, как и любой другой удаленный server.

Claude Code также показывает уведомление при запуске, когда один или несколько настроенных servers требуют аутентификации, поэтому вам не нужно открывать `/mcp` для обнаружения, какие servers требуют входа. Уведомление требует Claude Code v2.1.193 или позже. Оно считает только servers, на которые вы можете войти из Claude Code. До v2.1.218 оно также считало [разъемы claude.ai](#use-mcp-servers-from-claude-ai), которые не были подключены в claude.ai, которые вы можете подключить только из параметров claude.ai.

Уведомление объявляет каждый server один раз и исключает его из подсчета при последующих запусках, пока этот server не подключится и не потребует входа снова. `/mcp` по-прежнему перечисляет каждый server, который требует входа.

В неинтерактивном режиме нет панели `/mcp`, поэтому Claude Code не может запустить поток OAuth для вас. Начиная с v2.1.196, когда настроенный server требует аутентификации во время запуска `claude -p` или Agent SDK с включенным [поиском инструментов](#scale-with-mcp-tool-search), что является значением по умолчанию, Claude Code сообщает Claude, что инструменты server недоступны, пока вы его не авторизуете. Claude затем может назвать server, который требует входа, вместо того чтобы отвечать так, как будто server не был настроен. Завершите вход из интерактивного сеанса с помощью `/mcp` или `claude mcp login <name>`.

Если вы настроили `headers.Authorization` для server и server отклонил этот заголовок, Claude Code сообщает о соединении как о неудачном вместо того, чтобы вернуться к OAuth. Проверьте, что токен действителен для конечной точки MCP, или удалите заголовок, чтобы использовать поток OAuth.

<Steps>
  <Step title="Добавьте server, который требует аутентификации">
    Если вы уже добавили server `sentry` в [быстром старте MCP](/docs/ru/mcp-quickstart#connect-a-server-that-requires-sign-in), пропустите этот шаг: запуск `claude mcp add` снова с тем же именем server в той же области не удается с `MCP server sentry already exists in local config`. В противном случае запустите:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Используйте команду /mcp в Claude Code">
    В Claude Code используйте команду:

    ```text wrap theme={null}
    /mcp
    ```

    Затем следуйте инструкциям в вашем браузере для входа.
  </Step>
</Steps>

<Tip>
  Советы:

  * Токены аутентификации хранятся безопасно и автоматически обновляются
  * Используйте "Clear authentication" в меню `/mcp` для отзыва доступа
  * Если ваш браузер не открывается автоматически, скопируйте предоставленный URL и откройте его вручную
  * Если перенаправление браузера не удается с ошибкой соединения после аутентификации, вставьте полный URL обратного вызова из адресной строки браузера в приглашение URL, которое появляется в Claude Code
  * Аутентификация OAuth работает с HTTP servers
</Tip>

<h3 id="authenticate-from-the-command-line">
  Аутентификация из командной строки
</h3>

Команда `claude mcp login <name>` запускает поток OAuth настроенного server прямо из вашей оболочки, поэтому вам не нужно открывать панель `/mcp` внутри сеанса.

```bash theme={null}
claude mcp login sentry
```

Чтобы позже очистить сохраненные учетные данные, запустите `claude mcp logout <name>`.

`claude mcp login` обнаруживает, когда локальный браузер недоступен, например во время сеанса SSH или на Linux без сервера отображения, и выводит URL авторизации вместо попытки открыть браузер. Откройте URL на вашей локальной машине, затем вставьте полный URL перенаправления из адресной строки браузера обратно в приглашение. Команде требуется интерактивный терминал для шага вставки, поэтому подключитесь с помощью `ssh -t`. Передайте `--no-browser` для принудительного приглашения URL даже при обнаружении локального браузера.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Используйте фиксированный порт обратного вызова OAuth
</h3>

Некоторые MCP servers требуют конкретный URI перенаправления, зарегистрированный заранее. По умолчанию Claude Code выбирает случайный доступный порт для обратного вызова OAuth. Используйте `--callback-port` для фиксации порта, чтобы он соответствовал предварительно зарегистрированному URI перенаправления формы `http://localhost:PORT/callback`. Если вход на Claude Code v2.1.229 не удается с несоответствием URI перенаправления, см. примечание версии в разделе [Используйте предварительно настроенные учетные данные OAuth](#use-pre-configured-oauth-credentials).

Вы можете использовать `--callback-port` самостоятельно (с динамической регистрацией клиента) или вместе с `--client-id` (с предварительно настроенными учетными данными).

```bash theme={null}
# Фиксированный порт обратного вызова с динамической регистрацией клиента
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  Используйте предварительно настроенные учетные данные OAuth
</h3>

Некоторые MCP servers не поддерживают автоматическую настройку OAuth через Dynamic Client Registration. Если вы видите ошибку типа "Incompatible auth server: does not support dynamic client registration", server требует предварительно настроенные учетные данные. Claude Code также поддерживает servers, которые используют Client ID Metadata Document (CIMD) вместо Dynamic Client Registration, и обнаруживает их автоматически. Если автоматическое обнаружение не удается, сначала зарегистрируйте приложение OAuth через портал разработчика server, затем предоставьте учетные данные при добавлении server.

<Steps>
  <Step title="Зарегистрируйте приложение OAuth с помощью server">
    Создайте приложение через портал разработчика server и запишите ваш client ID и client secret.

    Многие servers также требуют URI перенаправления. Если это так, выберите порт и зарегистрируйте URI перенаправления в формате `http://localhost:PORT/callback`. Используйте тот же порт с `--callback-port` на следующем шаге.

    В v2.1.229 Claude Code отправлял `http://127.0.0.1:PORT/callback` вместо этого, и servers, которые точно совпадают с зарегистрированным URI перенаправления, отклоняли вход с несоответствием URI перенаправления. Claude Code v2.1.231 восстановил форму `localhost`. Чтобы восстановиться на v2.1.229, обновите Claude Code или временно добавьте форму `http://127.0.0.1:PORT/callback` к зарегистрированным URI перенаправления server.
  </Step>

  <Step title="Добавьте server с вашими учетными данными">
    Выберите один из следующих методов. Порт, используемый для `--callback-port`, может быть любым доступным портом. Он должен соответствовать URI перенаправления, который вы зарегистрировали на предыдущем шаге.

    <Tabs>
      <Tab title="claude mcp add">
        Используйте `--client-id` для передачи client ID вашего приложения. Флаг `--client-secret` запрашивает secret с замаскированным вводом:

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        Включите объект `oauth` в конфигурацию JSON и передайте `--client-secret` как отдельный флаг:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (только порт обратного вызова)">
        Используйте `--callback-port` без client ID для фиксации порта при использовании динамической регистрации клиента:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / переменная окружения">
        Установите secret через переменную окружения, чтобы пропустить интерактивное приглашение:

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Аутентифицируйтесь в Claude Code">
    Запустите `/mcp` в Claude Code и следуйте потоку входа браузера.
  </Step>
</Steps>

<Tip>
  Советы:

  * Client secret хранится безопасно в вашей системной связке ключей (macOS) или файле учетных данных, а не в вашей конфигурации
  * Вы можете установить client secret только при добавлении server. Когда вы аутентифицируетесь с помощью `claude mcp login` или из `/mcp`, Claude Code использует сохраненный secret и не запрашивает его или не читает `MCP_CLIENT_SECRET`
  * Чтобы добавить или изменить secret позже, удалите server с помощью `claude mcp remove <name>`, затем добавьте его снова с помощью `--client-secret` и той же `--scope`
  * Если server использует публичный OAuth клиент без secret, используйте только `--client-id` без `--client-secret`
  * Эти флаги применяются только к HTTP и SSE транспортам. Они не влияют на stdio servers
  * Используйте `claude mcp get <name>` для проверки того, что учетные данные OAuth настроены для server
</Tip>

<h3 id="override-oauth-metadata-discovery">
  Переопределите обнаружение метаданных OAuth
</h3>

Укажите Claude Code на конкретный URL метаданных сервера авторизации OAuth, чтобы обойти цепочку обнаружения по умолчанию. Установите `authServerMetadataUrl`, когда стандартные конечные точки MCP server выдают ошибку, или когда вы хотите направить обнаружение через внутренний прокси. По умолчанию Claude Code сначала проверяет метаданные защищенного ресурса RFC 9728 на `/.well-known/oauth-protected-resource`, затем возвращается к метаданным сервера авторизации RFC 8414 на `/.well-known/oauth-authorization-server`.

Установите `authServerMetadataUrl` в объекте `oauth` конфигурации вашего server в `.mcp.json`:

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

URL должен использовать `https://`. `scopes_supported` URL метаданных переопределяет области, которые объявляет upstream server.

<h3 id="restrict-oauth-scopes">
  Ограничьте области OAuth
</h3>

Установите `oauth.scopes` для фиксации областей, которые Claude Code запрашивает во время потока авторизации. Это поддерживаемый способ ограничить MCP server подмножеством, одобренным командой безопасности, когда upstream сервер авторизации объявляет больше областей, чем вы хотите предоставить. Значение — это одна строка, разделенная пробелами, соответствующая формату параметра `scope` в RFC 6749 §3.3.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` имеет приоритет над `authServerMetadataUrl` и областями, которые server обнаруживает на `/.well-known`. Оставьте его неустановленным, чтобы позволить MCP server определить запрашиваемый набор областей.

Начиная с v2.1.196, когда `oauth.scopes` не установлен, Claude Code запрашивает область, предоставленную заголовком `WWW-Authenticate` server или его метаданными защищенного ресурса, и не отправляет параметр `scope`, когда ни один из них не предоставляет его. Он больше не запрашивает полный каталог `scopes_supported` из автоматически обнаруженных метаданных сервера авторизации. Запрос этого каталога заставил поставщиков идентификации, которые объявляют области только для администраторов или шаблонов, отклонить запрос авторизации с ошибкой `invalid_scope`. Метаданные, полученные из настроенного `authServerMetadataUrl`, по-прежнему предоставляют свой `scopes_supported` как запрашиваемые области.

Если сервер авторизации объявляет `offline_access` в `scopes_supported`, Claude Code добавляет его к фиксированным областям, чтобы токен доступа мог быть обновлен без нового входа в браузер.

Если server позже возвращает 403 `insufficient_scope` для вызова инструмента, вызов не удается с сообщением [`needs additional permissions`](/docs/ru/errors#mcp-server-needs-you-to-sign-in-again), которое называет область, которую запрашивает server. Server показывается как требующий аутентификации в `/mcp`.

Если эта область не находится в вашем фиксированном `oauth.scopes`, добавьте ее, затем запустите `/mcp` и аутентифицируйте server снова. Claude Code запрашивает фиксированные области, а не область, которую назвал server, поэтому если вы аутентифицируетесь снова без добавления ее, токен, который вы получите, по-прежнему ей не хватает.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Используйте динамические заголовки для пользовательской аутентификации
</h3>

Если ваш MCP server использует схему аутентификации, отличную от OAuth (такую как Kerberos, краткосрочные токены или внутреннее SSO), используйте `headersHelper` для генерации заголовков запроса во время подключения. Claude Code запускает команду и объединяет ее выход в заголовки подключения.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

Команда также может быть встроенной:

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**Требования:**

* Команда должна записать объект JSON пар строк ключ-значение в stdout
* Claude Code запускает команду в оболочке и отказывается от нее через 10 секунд
* Claude Code выбирает рабочий каталог команды по [месту, где вы настроили server](#where-the-helper-runs), поэтому дайте скрипт как абсолютный путь или поместите его на `PATH`
* Динамические заголовки переопределяют любые статические `headers` с тем же именем

Claude Code запускает помощника заново при каждом подключении, при запуске сеанса и при переподключении, как только [правило доверия для servers проекта и локальной области](#trust-a-folder-before-its-headershelper-runs) позволяет ему запуститься. Он не кэширует результат, поэтому ваш скрипт отвечает за любое повторное использование токена.

Если вызов инструмента возвращает `401 Unauthorized` или `403 Forbidden`, Claude Code автоматически повторно запускает помощника под тем же правилом, переподключается со свежими заголовками и повторяет вызов один раз. Claude Code отмечает server как требующий аутентификации в `/mcp` только если этот повторный вызов также не удается.

Когда выход помощника включает заголовок `Authorization`, Claude Code использует это учетное данные как аутентификацию server и не возвращается к OAuth для server.

Если server отклоняет учетное данные помощника при подключении, Claude Code сообщает о соединении как о неудачном, а не отмечает server как требующий аутентификации. Исправьте учетное данные, которое возвращает ваш помощник, затем переподключитесь из `/mcp` для повторного запуска помощника.

Claude Code устанавливает эти переменные окружения при выполнении помощника:

| Переменная                    | Значение                                                                                                             |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | имя MCP server                                                                                                       |
| `CLAUDE_CODE_MCP_SERVER_URL`  | URL MCP server                                                                                                       |
| `CLAUDE_PLUGIN_ROOT`          | корневой каталог плагина. Установлено только когда [плагин](/docs/ru/plugins/components#mcp-servers) предоставляет server |

Используйте их для написания одного скрипта помощника, который служит нескольким MCP servers.

Помощник `headersHelper`, предоставленный плагином, не может ссылаться на значения [`${user_config.*}`](/docs/ru/plugins/manifest-reference#user-configuration) плагина, потому что команда запускается через оболочку. Claude Code сообщает о server как о неправильно настроенном с [ошибкой](/docs/ru/errors#plugin-command-references-user-config) и не подставляет значение. Поместите `${user_config.KEY}` в поле `headers` server вместо этого, которое не анализируется оболочкой, или попросите скрипт помощника прочитать значение из файла конфигурации. До v2.1.207 `headersHelper` подставлял значения `${user_config.*}`.

<h4 id="where-the-helper-runs">
  Где запускается помощник
</h4>

Claude Code выбирает рабочий каталог команды `headersHelper` из конфигурации, которая объявляет server. `cd`, который Claude запускает в Bash, не перемещает его, и [`/cd`](/docs/ru/permissions#move-the-session-to-another-directory) перемещает его только для servers, которые запускаются из основного рабочего каталога сеанса. Каждая строка ниже дает каталог, в котором относительный путь в вашей команде `headersHelper` разрешается.

| Где вы настроили server                                                                                                                                                                                   | Рабочий каталог                                                                                 |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| [Плагин](/docs/ru/plugins/components#mcp-servers)                                                                                                                                                              | Корневой каталог плагина. Требует Claude Code v2.1.195 или позже                                |
| Проект `.mcp.json` или [server локальной области](#local-scope)                                                                                                                                           | Каталог проекта, в котором объявлен server                                                      |
| Файл агента в вашем проекте, server из опции `mcpServers` SDK или метода `setMcpServers()`, или [`--mcp-config`](/docs/ru/cli-reference)                                                                       | [Основной рабочий каталог](/docs/ru/permissions#working-directories) сеанса                          |
| [Область пользователя](#user-scope), [управляемый MCP](/docs/ru/managed-mcp), [разъем claude.ai](#use-mcp-servers-from-claude-ai), или файл агента из вне вашего проекта, включая один из каталога `--add-dir` | Ваш каталог конфигурации, `~/.claude` если вы не установили [`CLAUDE_CONFIG_DIR`](/docs/ru/env-vars) |

До v2.1.238 Claude Code также запускал помощников servers пользовательской области, управляемых и разъемов claude.ai, и файлов агентов из вне вашего проекта из каталога, в котором вы его запустили.

<h4 id="which-variables-a-helper-can-read">
  Какие переменные может читать помощник
</h4>

`headersHelper`, который предоставляет репозиторий или плагин, — это команда, которую вы не писали, поэтому Claude Code запускает ее без переменных учетных данных из вашего окружения, таких как `ANTHROPIC_API_KEY`. Место, где вы настроили server, определяет, применяется ли это:

* **Удалено**: server в проекте `.mcp.json` или в плагине, и встроенный server в файле агента из вашего проекта или из каталога `--add-dir`
* **Не удалено**: server в [пользовательской](#user-scope) или [локальной области](#local-scope), в [управляемом MCP](/docs/ru/managed-mcp), из [разъема claude.ai](#use-mcp-servers-from-claude-ai), или предоставленный SDK или [`--mcp-config`](/docs/ru/cli-reference), и встроенный server в файле агента из `~/.claude/agents/`, из управляемых параметров или переданный с `--agents`

Помимо переменных `GIT_CONFIG_KEY_<n>` Git, Claude Code удаляет каждую переменную из вашего окружения, чье имя выглядит как учетное данные, такое как имя с `TOKEN`, `SECRET`, `PASSWORD`, `KEY` или `AUTH` в нем в любом регистре, поэтому `ANTHROPIC_API_KEY` и `MY_REGISTRY_TOKEN` оба удаляются. Claude Code также удаляет фиксированный список переменных учетных данных, чьи имена не следуют этому шаблону, такие как `ANTHROPIC_CUSTOM_HEADERS`.

Когда это применяется к вашему помощнику, попросите скрипт прочитать его учетное данные из файла или хранилища учетных данных. Если `url` server [расширяет одну из этих переменных](#environment-variable-expansion-in-mcp-json), значение `CLAUDE_CODE_MCP_SERVER_URL`, которое получает помощник, имеет эту часть заменена на `REDACTED` также.

<h4 id="trust-a-folder-before-its-headershelper-runs">
  Доверьте папке перед запуском ее headersHelper
</h4>

Claude Code выполняет `headersHelper` как произвольную команду оболочки. Для server в проекте `.mcp.json` или в [локальной области](#local-scope), он запускает помощника только после того, как вы примете [диалог доверия](/docs/ru/permissions#project-allow-rules-and-workspace-trust) для каталога проекта, в котором объявлен server. До v2.1.238 сеанс `claude -p` или SDK запускал этих помощников без проверки доверия, и интерактивный сеанс запускал их после того, как вы доверили родительской папке.

* **Доверие, которое не считается**: доверие родительской папки и автоматическое доверие, которое получает сеанс `claude -p` или SDK для [hooks в файлах параметров](/docs/ru/permissions#what-runs-before-you-trust-a-folder)
* **До тех пор, пока вы не доверите папке**: Claude Code подключает server только с его статическими `headers`. В сеансе `claude -p` или SDK он также выводит одну строку [`headersHelper not run`](/docs/ru/errors#headershelper-not-run) на stderr, сообщая вам, как предоставить доверие.
* **Доверие без диалога**: установите `projects["<path>"].hasTrustDialogAccepted` на `true` в `~/.claude.json`. `<path>` — это папка, на которой [Project allow rules and workspace trust](/docs/ru/permissions#project-allow-rules-and-workspace-trust) говорит Claude Code ключ доверия.

Claude Code применяет то же правило к server, объявленному встроенным в [файл агента](/docs/ru/sub-agents#scope-mcp-servers-to-a-subagent), проверяя, откуда этот файл агента пришел: ваш проект, для файла в его каталоге `.claude/agents/`, или каталог `--add-dir`. До тех пор, пока вы не [доверите этому проекту или каталогу самому](/docs/ru/permissions#what-runs-before-you-trust-a-folder), Claude Code не загружает server вообще, поэтому его помощник никогда не запускается либо.

<h2 id="add-mcp-servers-from-json-configuration">
  Добавьте MCP servers из конфигурации JSON
</h2>

Если у вас есть конфигурация JSON для MCP server, вы можете добавить ее напрямую:

<Steps>
  <Step title="Добавьте MCP server из JSON">
    ```bash theme={null}
    # Базовый синтаксис
    claude mcp add-json <name> '<json>'

    # Пример: добавление HTTP server с конфигурацией JSON
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # Пример: добавление stdio server с конфигурацией JSON
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # Пример: добавление HTTP server с предварительно настроенными учетными данными OAuth
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="Проверьте, что server был добавлен">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Убедитесь, что JSON правильно экранирован в вашей оболочке
  * JSON должен соответствовать схеме конфигурации MCP server
  * Вы можете использовать `--scope user` для добавления server в вашу конфигурацию пользователя вместо конфигурации, специфичной для проекта
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Импортируйте MCP servers из Claude Desktop
</h2>

Если вы уже настроили MCP servers в Claude Desktop, вы можете их импортировать:

<Steps>
  <Step title="Импортируйте servers из Claude Desktop">
    ```bash theme={null}
    # Базовый синтаксис 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="Выберите, какие servers импортировать">
    После запуска команды вы увидите интерактивный диалог, который позволяет вам выбрать, какие servers вы хотите импортировать.
  </Step>

  <Step title="Проверьте, что servers были импортированы">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

Имена servers, добавленные через команды `claude mcp`, могут содержать только буквы, цифры, дефисы и подчеркивания. Claude Desktop не применяет это ограничение, поэтому server Claude Desktop, имя которого содержит любой другой символ, например пробел, не может быть импортирован. Импорт сообщает о каждом отклоненном имени и все еще импортирует другие выбранные вами servers. До версии 2.1.205 первое недопустимое имя останавливало импорт и ни один из выбранных servers не был добавлен.

<Tip>
  Советы:

  * Эта функция работает только на macOS и Windows Subsystem for Linux (WSL)
  * Она читает файл конфигурации Claude Desktop из его стандартного расположения на этих платформах
  * Используйте флаг `--scope user` для добавления servers в вашу конфигурацию пользователя
  * Импортированные servers сохраняют те же имена, что и в Claude Desktop, когда имя содержит только буквы, цифры, дефисы и подчеркивания. Claude Code сообщает о server, имя которого содержит любой другой символ, и пропускает его
  * Если servers с одинаковыми именами уже существуют, они получат числовой суффикс (например, `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  Использование MCP серверов из claude.ai
</h2>

Если вы вошли в Claude Code с помощью учётной записи [claude.ai](https://claude.ai), MCP серверы, которые вы добавили в claude.ai, известные как [connectors](https://claude.com/docs/connectors), автоматически доступны в Claude Code:

<Steps>
  <Step title="Настройте MCP серверы в claude.ai">
    Добавьте серверы на [claude.ai/customize/connectors](https://claude.ai/customize/connectors). В планах Team и Enterprise только администраторы могут добавлять серверы.
  </Step>

  <Step title="Аутентифицируйте MCP сервер">
    Выполните все необходимые шаги аутентификации в claude.ai.
  </Step>

  <Step title="Просмотрите и управляйте серверами в Claude Code">
    В Claude Code используйте команду:

    ```text wrap theme={null}
    /mcp
    ```

    Серверы из claude.ai появляются в списке с индикаторами, показывающими, что они поступают из claude.ai.
  </Step>
</Steps>

Claude Code помечает connector как `managed` в `/mcp` и в менеджере [`/plugin`](/docs/ru/plugins/install) когда ваша организация управляет его аутентификацией в claude.ai. Статус managed не изменяет способ подключения Claude Code к connector или применение [инструментов управления](#organization-controls-on-connector-tools) вашей организации.

Connectors, в которые вы никогда не входили, свёрнуты за строкой `Show unused connectors` в конце раздела claude.ai, поэтому список, предоставленный организацией, не заполняет панель. Выберите строку, чтобы развернуть их. Connector, в который вы входили ранее, остаётся видимым даже если в настоящий момент требуется повторная аутентификация.

Connectors из claude.ai загружаются только когда ваш активный [метод аутентификации](/docs/ru/authentication#authentication-precedence) — это вход по подписке claude.ai. Они не загружаются, даже если вы ранее запустили `/login`, когда:

* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper` активны
* Активен сторонний поставщик, такой как Amazon Bedrock или Agent Platform Google Cloud
* `ANTHROPIC_PROFILE`, переменные федерации или активный [профиль Anthropic](/docs/ru/authentication#anthropic-profiles-and-federation-credentials) предоставляют учётные данные
* `CLAUDE_CODE_OAUTH_TOKEN` содержит токен из [`claude setup-token`](/docs/ru/authentication#generate-a-long-lived-token), который может только делать запросы к модели

Если `/mcp` не отображает connector, который вы добавили, запустите `/status`, чтобы подтвердить, какой метод аутентификации активен. Отмените установку этой переменной окружения, удалите параметр `apiKeyHelper` или [отключите профиль](/docs/ru/authentication#anthropic-profiles-and-federation-credentials), затем запустите `/login`, чтобы выбрать вашу учётную запись claude.ai.

Если временная проблема с сетью препятствует загрузке списка connectors при запуске сеанса, Claude Code повторяет попытку загрузки до трёх раз в фоновом режиме, и connectors появляются после успешной повторной попытки. Если они всё ещё не появились, перезагрузите Claude Code, чтобы загрузить список снова.

Если `/mcp` показывает connector как `connected · session token rejected` или его подробное представление показывает [`claude.ai rejected the session token`](/docs/ru/errors#claude-ai-rejected-the-session-token), claude.ai отклонил токен из вашего входа в Claude Code, обычно потому что вход истёк и не мог быть обновлён. Повторная авторизация connector не очищает это состояние, потому что собственная авторизация connector в claude.ai — это не то, что было отклонено. Чтобы очистить это:

1. Запустите `/login`, чтобы войти снова.
2. Переподключите connector из `/mcp`.

До версии 2.1.222 Claude Code помечал connectors как требующие аутентификации, и их авторизация не решала проблему.

Сервер, который вы добавили в Claude Code, имеет [приоритет](#scope-hierarchy-and-precedence) над connector из claude.ai, который указывает на тот же URL. Когда это происходит, `/mcp` отображает connector как скрытый и показывает, как удалить дубликат, если вы предпочитаете использовать connector.

Некоторые размещённые Anthropic connectors, такие как Microsoft 365, Gmail и Google Calendar, не поддерживают локальный OAuth из Claude Code, потому что вышестоящий поставщик идентификации принимает только URL перенаправления, зарегистрированный claude.ai. Когда сервер, который вы добавили с помощью `claude mcp add` или в `.mcp.json`, указывает на один из этих хостов и вы входите в него из `/mcp` или с помощью `claude mcp login`, Claude Code показывает [`is Anthropic-hosted and doesn't support local OAuth`](/docs/ru/errors#anthropic-hosted-and-doesnt-support-local-oauth), направляя вас подключить сервис на [claude.ai/customize/connectors](https://claude.ai/customize/connectors) вместо этого.

После удаления вашей записи с помощью `claude mcp remove <name>` и подключения сервиса на claude.ai, connector появляется в Claude Code автоматически.

<h3 id="how-connectors-reach-claude-code">
  Как connectors достигают Claude Code
</h3>

Какие параметры управляют connector из claude.ai, зависит от того, где работает ваш сеанс, потому что только некоторые сеансы сами загружают connectors из claude.ai. Каждая строка ниже указывает, как connectors поступают в один вид сеанса и что ими управляет там. [WSL сеансы](/docs/ru/desktop-wsl#what-works-in-a-wsl-session) настольного приложения не имеют строки, потому что connectors в них пока недоступны.

| Где работает сеанс                                                                                                    | Как поступают connectors                       | Что ими управляет                                                                                                                                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Сеансы Terminal, [VS Code](/docs/ru/vs-code), [JetBrains](/docs/ru/jetbrains) и [Agent SDK](/docs/ru/agent-sdk/claude-code-features) | Claude Code загружает их из claude.ai          | Параметры в этом разделе и [управляемая конфигурация MCP](/docs/ru/managed-mcp)                                                                                                                                                       |
| [Облачные сеансы](/docs/ru/claude-code-on-the-web)                                                                         | Удалённый хост передаёт их                     | Параметры организации claude.ai, плюс параметры [allowlist и denylist](/docs/ru/managed-mcp#policy-based-control-with-allowlists-and-denylists), которые достигают сеанса, и любой `managed-mcp.json` на хосте, который его запускает |
| [Настольное приложение](/docs/ru/desktop) локальные и SSH сеансы                                                           | Настольное приложение доставляет их в процессе | Записи `blocked` в [инструментах управления connector](#organization-controls-on-connector-tools) вашей организации                                                                                                              |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors), `ENABLE_CLAUDEAI_MCP_SERVERS` и [`allowAllClaudeAiMcps`](/docs/ru/settings-reference#allowallclaudeaimcps) действуют только на первую строку, connectors, которые Claude Code загружает сам. Две другие строки отличаются от неё следующим образом:

* **Облачные сеансы**: записи `allowedMcpServers` и `deniedMcpServers`, которые достигают сеанса, например через [параметры, управляемые сервером](/docs/ru/server-managed-settings), также фильтруют доставленные connectors. Прокси сеанса переписывает URL каждого connector, поэтому шаблон `serverUrl`, написанный для собственного URL connector, не совпадает с ним. Чтобы допустить доставленные connectors наряду со списком разрешений URL в самостоятельной среде, добавьте записи `serverUrl`, указанные в разделе [Трафик Connector покидает вашу сеть](/docs/ru/self-hosted-environments-deploy#connector-traffic-leaves-your-network). Claude Code отбрасывает доставленные connectors, когда на хосте, который запускает сеанс, присутствует `managed-mcp.json`, например [хост самостоятельного runner](/docs/ru/self-hosted-environments-configuration#mcp-servers), независимо от того, установили ли вы `allowAllClaudeAiMcps`.
* **Локальные и SSH сеансы настольного приложения**: настольное приложение регистрирует connectors как внутрипроцессные серверы `type: "sdk"`, и никакой параметр MCP или `managed-mcp.json` не достигает их. Пользователь держит connector вне своих собственных сеансов, отключив его на [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Организация блокирует [инструменты](#organization-controls-on-connector-tools) connector или полностью отключает [Claude Code в настольном приложении](/docs/ru/desktop#admin-console-controls).

<h3 id="organization-controls-on-connector-tools">
  Элементы управления организацией для инструментов connector
</h3>

Ваша организация может установить элементы управления для каждого инструмента на [connectors claude.ai](https://claude.com/docs/connectors). Claude Code читает эти параметры при запуске и применяет их локально, кроме как в [локальных и SSH сеансах](#how-connectors-reach-claude-code) настольного приложения. Там настольное приложение скрывает инструменты `blocked` перед доставкой connector, и параметр `ask` не достигает Claude Code, поэтому он применяет обычные [правила разрешений](/docs/ru/permissions) сеанса к этим инструментам вместо запроса при каждом вызове. В сеансах, где Claude Code загружает connectors сам, запустите `/mcp`, чтобы увидеть, какой параметр применяется к каждому инструменту на connector.

* **Инструмент установлен на `ask`**: Claude Code запрашивает при каждом вызове с причиной `Your organization requires approval for this tool`. Запрос появляется даже в режимах разрешений `acceptEdits`, `auto` и `bypassPermissions` [permission modes](/docs/ru/permissions#permission-modes), и никогда не предлагает опцию запомнить ваш выбор. [Правила разрешений](/docs/ru/permissions), которые совпадают с инструментом, также не пропускают запрос. В режиме `dontAsk`, который никогда не запрашивает, Claude Code отклоняет вызов вместо этого.
* **Инструмент установлен на `blocked`**: Claude Code фильтрует инструмент перед тем, как Claude его видит, поэтому он никогда не появляется в списке инструментов. Настольное приложение и чат claude.ai применяют тот же параметр `blocked`, поэтому Claude не может использовать инструмент там либо, и вы не можете скрыть инструмент из сеансов настольного приложения, сохраняя его доступным в чате. Настольное приложение пропускает connector, все инструменты которого заблокированы.

<h3 id="disable-claude-ai-connectors">
  Отключение connectors claude.ai
</h3>

Claude Code применяет [`disableClaudeAiConnectors`](/docs/ru/settings-reference#disableclaudeaiconnectors) только к connectors, которые он [загружает сам](#how-connectors-reach-claude-code), а не к connectors, которые доставляет облачный хост или настольное приложение. Чтобы отключить connectors, которые он загружает, установите параметр на `true` в любой области параметров:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Этот параметр использует семантику any-source-true: `true` в любом источнике параметров имеет приоритет. Проверенный в репозитории `.claude/settings.json` проекта может отключить connectors, которые Claude Code загружает сам, но уровень проекта `false` не может повторно включить connectors, которые уровень пользователя или политики `true` отключил. Серверы, переданные явно через `--mcp-config`, не затронуты.

Вы также можете установить переменную окружения `ENABLE_CLAUDEAI_MCP_SERVERS` на `false`, что имеет тот же эффект для текущего сеанса оболочки:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Чтобы заблокировать отдельные connectors claude.ai вместо всех них, добавьте их в [`deniedMcpServers`](/docs/ru/managed-mcp) по имени или по шаблону URL. Например, запись `serverName` из `"claude.ai Slack"` блокирует connector Slack. Вы также можете запустить `/mcp`, чтобы переключить любой connector, который Claude Code загружает, включить или отключить только для текущего проекта.

<h2 id="use-claude-code-as-an-mcp-server">
  Использование Claude Code в качестве MCP сервера
</h2>

Вы можете использовать Claude Code в качестве MCP сервера, к которому могут подключаться другие приложения:

```bash theme={null}
# Запустить Claude как stdio MCP сервер
claude mcp serve
```

Команда ничего не выводит при запуске. Stdio MCP сервер взаимодействует через stdin и stdout, поэтому молчаливый, заблокированный терминал означает, что сервер работает и ожидает подключения клиента.

Вы можете использовать это в Claude Desktop, добавив эту конфигурацию в claude\_desktop\_config.json:

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **Настройка пути к исполняемому файлу**: поле `command` должно ссылаться на исполняемый файл Claude Code. Если команда `claude` отсутствует в PATH вашей системы, вам потребуется указать полный путь к исполняемому файлу.

  Чтобы найти полный путь:

  ```bash theme={null}
  which claude
  ```

  Затем используйте полный путь в вашей конфигурации:

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  Без правильного пути к исполняемому файлу вы столкнётесь с ошибками вроде `spawn claude ENOENT`.
</Warning>

<Tip>
  Советы:

  * В Claude Desktop попробуйте попросить Claude прочитать файлы в каталоге, внести изменения и многое другое.
  * Этот MCP сервер предоставляет только инструменты Claude Code вашему MCP клиенту, поэтому ваш собственный клиент отвечает за реализацию подтверждения пользователя для отдельных вызовов инструментов.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Ограничения и предупреждения выходных данных MCP
</h2>

Когда инструменты MCP производят большие объемы выходных данных, Claude Code помогает управлять использованием токенов, чтобы не перегружать контекст вашего разговора:

* **Порог предупреждения выходных данных**: Claude Code отображает предупреждение, когда выходные данные любого инструмента MCP превышают 10 000 токенов
* **Настраиваемый лимит**: вы можете отрегулировать максимально допустимое количество токенов выходных данных MCP, используя переменную окружения `MAX_MCP_OUTPUT_TOKENS`
* **Лимит по умолчанию**: максимум по умолчанию составляет 25 000 токенов
* **Область действия**: переменная окружения применяется к инструментам, которые не объявляют свой собственный лимит. Инструменты, которые устанавливают [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool), используют это значение вместо этого для текстового содержимого, независимо от того, какое значение установлено для `MAX_MCP_OUTPUT_TOKENS`. Инструменты, которые возвращают данные изображений, по-прежнему подчиняются `MAX_MCP_OUTPUT_TOKENS`
* **Превышение лимита**: когда результат без содержимого изображения превышает лимит, Claude Code сохраняет его в файл и заменяет его в разговоре сообщением, которое указывает путь к файлу, чтобы Claude прочитал файл, когда ему нужно содержимое. Файл находится в директории `tool-results` сеанса в [`~/.claude/projects/`](/docs/ru/claude-directory#cleaned-up-automatically).

Чтобы увеличить лимит для инструментов, которые производят большие объемы выходных данных:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  Повысить лимит для конкретного инструмента
</h3>

Если вы создаете сервер MCP, вы можете разрешить отдельным инструментам возвращать результаты, превышающие порог сохранения на диск по умолчанию, установив `_meta["anthropic/maxResultSizeChars"]` в записи ответа `tools/list` инструмента. Claude Code повышает порог этого инструмента до аннотированного значения, вплоть до жесткого потолка в 500 000 символов.

Это полезно для инструментов, которые возвращают по своей природе большие, но необходимые выходные данные, такие как схемы баз данных или полные деревья файлов. Без аннотации результаты, превышающие порог по умолчанию, сохраняются на диск и заменяются ссылкой на файл в разговоре.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

Аннотация применяется независимо от `MAX_MCP_OUTPUT_TOKENS` для текстового содержимого, поэтому пользователям не нужно повышать переменную окружения для инструментов, которые ее объявляют. Инструменты, которые возвращают данные изображений, по-прежнему подчиняются лимиту токенов.

<Warning>
  Если вы часто сталкиваетесь с предупреждениями о выходных данных с конкретными серверами MCP, которыми вы не управляете, рассмотрите возможность увеличения лимита `MAX_MCP_OUTPUT_TOKENS`. Вы также можете попросить автора сервера добавить аннотацию `anthropic/maxResultSizeChars` или разбить свои ответы на страницы. Аннотация не влияет на инструменты, которые возвращают содержимое изображений; для них единственный вариант — повысить `MAX_MCP_OUTPUT_TOKENS`.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Схемы входных данных инструментов с комбинатором корневого уровня
</h2>

Некоторые серверы MCP объявляют схему входных данных инструмента как объединение JSON Schema с `anyOf`, `oneOf` или `allOf` на верхнем уровне схемы. Claude API не принимает эти ключевые слова в корне схемы. Он принимает комбинаторы, вложенные в `properties`, которые Claude Code отправляет без изменений.

Инструменты с комбинатором корневого уровня остаются доступными. Перед отправкой инструмента в API, Claude Code преобразует схему в один объект и добавляет предложение к описанию инструмента, которое указывает Claude, какие группы параметров принадлежат друг другу:

* `allOf`: свойства из каждой ветви объединяются, и список `required` каждой ветви по-прежнему применяется
* `anyOf` и `oneOf`: свойства из каждой ветви объединяются, и список `required` каждой ветви описывается в описании инструмента вместо того, чтобы быть принудительно применяемым схемой

Ваш сервер получает любые аргументы, которые выбрал Claude, поэтому продолжайте проверять комбинацию на стороне сервера.

Когда Claude Code не может создать схему, которую принимает API, или при развёртывании, которое не получает удалённую конфигурацию, включающую переписывание, он пропускает этот инструмент, записывает причину в журнал сервера и оставляет другие инструменты сервера доступными. Версии более ранние, чем v2.1.195, пропускают каждый инструмент, входная схема которого имеет корневой `anyOf`, `oneOf` или `allOf`.

<h2 id="tools-with-invalid-input-schemas">
  Инструменты с недействительными схемами входных данных
</h2>

Claude API проверяет схему входных данных каждого инструмента в запросе и отклоняет весь запрос, когда любая схема не проходит проверку, поэтому один инструмент MCP с неправильной схемой приведет к тому, что каждый запрос, который его включает, завершится ошибкой 400. Claude Code запускает две проверки API самостоятельно при загрузке инструментов сервера и исключает каждый инструмент, который не пройдет их, поэтому другие инструменты сервера продолжают работать:

* Имена свойств верхнего уровня должны быть длиной от 1 до 64 символов и использовать только буквы и цифры ASCII, `_`, `.` и `-`
* Схема должна быть действительной в соответствии с метасхемой JSON Schema draft 2020-12. Claude Code применяет эту проверку к схемам, которые не объявляют `$schema`, и к схемам, которые объявляют draft 2020-12. Схема, которая объявляет любой другой диалект, пропускает эту проверку, хотя проверка имен свойств выше все еще применяется

Claude Code запускает проверки после [переписывания комбинатора корневого уровня](#tool-input-schemas-with-a-root-level-combinator), на схеме, которую он фактически отправит.

Когда Claude Code исключает инструмент, он записывает причину в журнал сервера и сообщает Claude, какие инструменты он исключил и почему, чтобы вы могли спросить Claude, почему инструмент отсутствует. Если вы исправите схему на сервере, инструмент вернется в следующий раз, когда Claude Code загрузит инструменты сервера.

Claude Code включает исключение через флаг функции, который он получает от Anthropic. На [развертывании, где получение флагов отключено](/docs/ru/env-vars#features-that-need-feature-flag-fetching), или на машине, флаги которой никогда не поступали, например на изолированной машине, Claude Code все еще запускает проверки и записывает в журнал сервера, какой инструмент будет отклонен, но отправляет схему инструмента в API в любом случае. API отклоняет запрос, который включает эту схему с [ошибкой 400, называющей инструмент по его позиции](/docs/ru/errors#tool-input-schema-is-invalid). До версии 2.1.216 ни одно развертывание не запускало эти проверки.

[Обработка комбинатора корневого уровня](#tool-input-schemas-with-a-root-level-combinator) отделена и сохраняет свое собственное поведение, когда получение флагов отключено или флаги никогда не поступали.

<h2 id="require-approval-for-a-specific-tool">
  Требование одобрения для конкретного инструмента
</h2>

Если вы создаёте MCP сервер, вы можете отметить инструмент как требующий явного одобрения при каждом вызове, установив `_meta["anthropic/requiresUserInteraction"]` в значение `true` в записи инструмента в ответе `tools/list`. Значение должно быть логическим значением JSON `true`; любое другое значение игнорируется.

Claude Code показывает запрос разрешения этого инструмента при каждом вызове, даже в режимах разрешений `acceptEdits`, `auto` и `bypassPermissions` [режимы разрешений](/docs/ru/permissions#permission-modes), и не предлагает опцию "не спрашивать снова" для него. [Правила разрешения](/docs/ru/permissions#permission-rule-syntax), которые соответствуют инструменту, также не пропускают запрос. В режиме `dontAsk`, который никогда не запрашивает, Claude Code отклоняет вызов вместо этого.

Запрос должен достичь человека. В неинтерактивном режиме с [`--permission-prompt-tool`](/docs/ru/cli-reference#cli-flags), результат `allow` из инструмента запроса разрешения для отмеченного инструмента преобразуется в отклонение с сообщением `MCP tool requires user interaction; not supported via --permission-prompt-tool`. Обратный вызов [`canUseTool`](/docs/ru/agent-sdk/permissions) Agent SDK получает эти вызовы и может их одобрить, потому что ваше приложение SDK должно показывать их пользователю.

Используйте это для инструментов, чей запрос разрешения сам по себе является целью, например шаг согласия или предоставления доступа, где автоматическое одобрение означало бы, что ни один человек никогда не согласился. Другие инструменты с того же сервера сохраняют своё обычное поведение разрешений.

Следующая запись `tools/list` отмечает один инструмент как всегда требующий одобрения.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

Аннотация `anthropic/requiresUserInteraction` требует Claude Code v2.1.199 или более поздней версии. Более ранние версии игнорируют её и применяют стандартный поток разрешений.

Некоторые поверхности, такие как [Remote Control](/docs/ru/remote-control) и приложения, созданные на основе [Agent SDK](/docs/ru/agent-sdk/overview), обычно позволяют вам одобрять вызовы инструментов одним касанием. Для инструмента, отмеченного этой аннотацией, Claude Code скрывает действие одного касания и вместо этого показывает полный запрос разрешения инструмента, поэтому одобрение по-прежнему исходит от человека, отвечающего на запрос, а не от касания.

Claude Code скрывает одобрение одним касанием таким же образом для любого запроса разрешения, который только диалог терминала может полностью отобразить, например того, который содержит предупреждение безопасности или опцию всегда разрешить, которую удалённая поверхность не может показать. Вы отвечаете на этот запрос в диалоге терминала, а не из Remote Control. Требует Claude Code v2.1.214 или более поздней версии.

<h2 id="respond-to-mcp-elicitation-requests">
  Ответ на запросы elicitation MCP
</h2>

MCP серверы могут запрашивать у вас структурированный ввод во время выполнения задачи, используя elicitation. Когда серверу требуется информация, которую он не может получить самостоятельно, Claude Code отображает интерактивный диалог и передает ваш ответ обратно серверу. С вашей стороны не требуется никакой конфигурации: диалоги elicitation появляются автоматически, когда сервер их запрашивает.

Серверы могут запрашивать ввод двумя способами:

* **Режим формы**: Claude Code показывает диалог с полями формы, определенными сервером (например, запрос имени пользователя и пароля). Заполните поля и отправьте.
* **Режим URL**: Claude Code открывает URL браузера для аутентификации или одобрения. Завершите процесс в браузере, затем подтвердите в CLI.

В режиме URL Claude Code передает URL в качестве аргумента командной строки обработчику URL вашей системы и ограничивает длину этого аргумента. Когда URL, после экранирования для командной строки, превышает это ограничение, вы можете только отклонить запрос. Каждый символ, который требует экранирования, такой как `%` или `&`, считается четыре раза в сторону ограничения: сам символ плюс три символа экранирования. URL без них достигает ограничения примерно на 8000 символов. URL, построенный в основном из процентных экранирований, где каждый третий символ — это `%`, достигает его примерно на 4000.

Для автоматического ответа на запросы elicitation без отображения диалога используйте [hook `Elicitation`](/docs/ru/hooks#elicitation).

Если вы создаете MCP сервер, который использует elicitation, см. [спецификацию MCP elicitation](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) для деталей протокола и примеров схемы.

<h2 id="use-mcp-resources">
  Использование ресурсов MCP
</h2>

Серверы MCP могут предоставлять ресурсы, на которые вы можете ссылаться с помощью упоминаний @, аналогично тому, как вы ссылаетесь на файлы.

<h3 id="reference-mcp-resources">
  Ссылка на ресурсы MCP
</h3>

<Steps>
  <Step title="Список доступных ресурсов">
    Введите `@` в вашу подсказку, чтобы увидеть доступные ресурсы со всех подключённых серверов MCP. Ресурсы отображаются рядом с файлами в меню автодополнения.
  </Step>

  <Step title="Ссылка на конкретный ресурс">
    Используйте формат `@server:protocol://resource/path` для ссылки на ресурс:

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Несколько ссылок на ресурсы">
    Вы можете ссылаться на несколько ресурсов в одной подсказке:

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * Ресурсы автоматически загружаются и включаются в качестве вложений при ссылке на них
  * Пути ресурсов поддерживают нечёткий поиск в автодополнении упоминаний @
  * Claude Code автоматически предоставляет инструменты для списания и чтения ресурсов MCP, когда серверы их поддерживают
  * Ресурсы могут содержать любой тип контента, который предоставляет сервер MCP (текст, JSON, структурированные данные и т. д.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Масштабирование с помощью поиска инструментов MCP
</h2>

Поиск инструментов снижает использование контекста MCP, отложив определения инструментов до момента, когда они понадобятся Claude. При запуске сеанса загружаются только имена инструментов и инструкции сервера, поэтому добавление дополнительных серверов MCP имеет минимальное влияние на окно контекста. Claude Code не устанавливает фиксированный лимит инструментов на сервер; практический лимит определяется бюджетом окна контекста.

<Note>
  Поиск инструментов не поддерживается на Microsoft Foundry [развертываниях, размещенных на Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), которые отклоняют его на стороне сервера: Claude Code обнаруживает отклонение и загружает инструменты MCP заранее для этого развертывания. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) не может переопределить это, так как отклонение исходит от самого развертывания.
</Note>

<h3 id="for-mcp-server-authors">
  Для авторов серверов MCP
</h3>

Если вы создаете сервер MCP, поле инструкций сервера становится более полезным при включенном поиске инструментов. Инструкции сервера помогают Claude понять, когда следует искать ваши инструменты, аналогично тому, как работают [skills](/docs/ru/skills).

Добавьте четкие, описательные инструкции сервера, которые объясняют:

* Какую категорию задач обрабатывают ваши инструменты
* Когда Claude должен искать ваши инструменты
* Ключевые возможности, которые предоставляет ваш сервер

Claude Code усекает описания инструментов и инструкции сервера на 2 048 символов по умолчанию. Держите их в краткой форме, и поместите критические детали в начало.

Чтобы изменить лимит для каждого сервера MCP в вашем сеансе, установите [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/ru/env-vars#variables) на количество символов. Эта переменная требует Claude Code v2.1.280 или позже.

<h3 id="configure-tool-search">
  Настройка поиска инструментов
</h3>

Поиск инструментов включен по умолчанию: инструменты MCP отложены и обнаруживаются по требованию. Claude Code отключает его, когда `ANTHROPIC_BASE_URL` указывает на хост, не принадлежащий первой стороне, так как большинство прокси не пересылают блоки `tool_reference`. Установите `ENABLE_TOOL_SEARCH` явно, чтобы переопределить этот резервный вариант.

Установка [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ru/env-vars) отключает поиск инструментов. Вы не можете переопределить это, установив `ENABLE_TOOL_SEARCH` самостоятельно. Ваша организация может оставить поиск инструментов включенным через [управляемые параметры](/docs/ru/managed-settings), на Claude Code v2.1.227 или позже. [Отключение предварительных возможностей](/docs/ru/llm-gateway-protocol#disable-pre-release-capabilities) охватывает, где применяется переопределение и что переменная удаляет.

Поиск инструментов требует модель, которая поддерживает блоки `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 и более поздние модели. См. [совместимость моделей в документации API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) для получения текущего списка.

На Agent Platform Google Cloud, Claude Code решает по поколению модели:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 и позже**: поиск инструментов включен по умолчанию, как и на API Anthropic.
* **Более ранние модели Agent Platform**: Claude Code загружает все инструменты MCP заранее, потому что их стеки обслуживания отклоняют требуемый заголовок бета-версии. `ENABLE_TOOL_SEARCH=true` не переопределяет это.

До версии 2.1.221 Claude Code отключал поиск инструментов для всех моделей на Agent Platform Google Cloud, если вы не установили `ENABLE_TOOL_SEARCH=true`.

Управляйте поведением поиска инструментов с помощью переменной окружения `ENABLE_TOOL_SEARCH`:

| Значение         | Поведение                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (не установлено) | Все инструменты MCP отложены и загружаются по требованию. Переходит на загрузку заранее на моделях Agent Platform Google Cloud более ранних, чем поколение Claude 4.5, когда `ANTHROPIC_BASE_URL` является хостом, не принадлежащим первой стороне, или на развертывании Microsoft Foundry, размещенном на Azure                                                                                                                                           |
| `true`           | Все инструменты MCP отложены, кроме развертывания Microsoft Foundry, размещенного на Azure, где отклонение на стороне сервера все еще вынуждает загрузку заранее, и на моделях Agent Platform Google Cloud более ранних, чем поколение Claude 4.5, где Claude Code продолжает загружать инструменты заранее. Claude Code отправляет заголовок бета-версии через прокси, и запросы не выполняются на прокси, которые не поддерживают блоки `tool_reference` |
| `auto`           | Режим порога: Claude Code загружает инструменты, которые он иначе отложил бы, заранее, пока их определения составляют менее 10% окна контекста, и отложит все из них, как только определения достигнут 10%                                                                                                                                                                                                                                                 |
| `auto:N`         | Режим порога с пользовательским процентом, где `N` — это 0-100. Например, `auto:5` для 5%                                                                                                                                                                                                                                                                                                                                                                  |
| `false`          | Все инструменты MCP загружены заранее, без отложения                                                                                                                                                                                                                                                                                                                                                                                                       |

```bash theme={null}
# Использование пользовательского порога 5%
ENABLE_TOOL_SEARCH=auto:5 claude

# Полное отключение поиска инструментов
ENABLE_TOOL_SEARCH=false claude
```

Или установите значение в поле [settings.json `env`](/docs/ru/settings-reference#env).

Вы также можете отключить инструмент `ToolSearch` специально:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Исключение сервера из отложения
</h3>

Если инструменты сервера всегда должны быть видны Claude без этапа поиска, установите `alwaysLoad` в значение `true` в конфигурации этого сервера. Каждый инструмент с этого сервера затем загружается в контекст при запуске сеанса независимо от параметра `ENABLE_TOOL_SEARCH`. Используйте это для небольшого количества инструментов, которые Claude нужны на каждом ходу, так как каждый заранее загруженный инструмент потребляет контекст, который иначе был бы доступен для вашего разговора.

Следующая запись `.mcp.json` исключает один HTTP-сервер, оставляя другие серверы отложенными:

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

Поле `alwaysLoad` доступно на всех типах серверов. Сервер MCP также может отметить отдельные инструменты как всегда загружаемые, включив `"anthropic/alwaysLoad": true` в объект `_meta` инструмента, что имеет тот же эффект только для этого инструмента.

Установка `alwaysLoad: true` также заставляет запуск ждать инструментов сервера, ограниченных стандартным тайм-аутом подключения в 5 секунд, так как они должны присутствовать при построении первого запроса. Удаленный сервер с действительной записью [`cached`](#server-status-detail) предоставляет свои инструменты из кэша без подключения, поэтому он не задерживает запуск. Другие серверы подключаются в фоновом режиме по умолчанию; установите [`MCP_CONNECTION_NONBLOCKING=0`](/docs/ru/env-vars), чтобы запуск ждал и их тоже.

<h2 id="use-mcp-prompts-as-commands">
  Использование MCP prompts как команд
</h2>

MCP серверы могут предоставлять prompts, которые становятся доступными как команды в Claude Code.

<h3 id="execute-mcp-prompts">
  Выполнение MCP prompts
</h3>

<Steps>
  <Step title="Обнаружение доступных prompts">
    Введите `/` чтобы увидеть доступные вам команды, включая те, которые поступают с MCP серверов. Claude Code отображает каждый MCP prompt как `/servername:promptname (MCP)`. Ввод `/mcp__servername__promptname` также запускает его.
  </Step>

  <Step title="Выполнение prompt без аргументов">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Выполнение prompt с аргументами">
    Многие prompts принимают аргументы. Передавайте их через пробел после команды. Claude Code разделяет аргументы по пробелам, поэтому каждый аргумент — это один токен:

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  Советы:

  * MCP prompts динамически обнаруживаются с подключённых серверов
  * Аргументы анализируются на основе определённых параметров prompt
  * Результаты prompt вводятся непосредственно в беседу
  * В форме `/mcp__servername__promptname` Claude Code заменяет любой символ в имени сервера вне `A-Z`, `a-z`, `0-9`, `_` и `-` на `_`, и использует имя prompt так, как его объявляет сервер
</Tip>

<h2 id="managed-mcp-configuration">
  Управляемая конфигурация MCP
</h2>

Для организаций, которым требуется централизованный контроль над тем, какие серверы MCP могут подключать пользователи, см. [Управляемая конфигурация MCP](/docs/ru/managed-mcp). В ней описывается развертывание фиксированного набора серверов с помощью `managed-mcp.json`, предоставление серверов каждому пользователю с помощью `managedMcpServers`, ограничение серверов с помощью `allowedMcpServers` и `deniedMcpServers`, а также то, что видят пользователи, когда сервер заблокирован.
