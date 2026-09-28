> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Размещение Agent SDK

> Развертывание Agent SDK в production: архитектура подпроцессов, сохранение сеансов, масштабирование, наблюдаемость и изоляция нескольких арендаторов для Docker, Kubernetes и поставщиков песочниц.

Agent SDK порождает и контролирует подпроцесс `claude` CLI, который владеет оболочкой, рабочей директорией и файлами сеансов на диске. Размещение его отличается от размещения stateless API-обертки. Каждый работающий агент — это долгоживущий процесс, привязанный к локальному состоянию, что определяет способ выделения ресурсов, сохранения сеансов и масштабирования между арендаторами.

На этой странице рассматривается самостоятельное размещение на вашей собственной инфраструктуре. Для развертываемых Dockerfile и манифестов Kubernetes см. [hosting cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Если вам не требуется запускать сам цикл агента на вашей собственной инфраструктуре, рассмотрите вместо этого [Managed Agents](https://platform.claude.com/docs/en/managed-agents/overview). Anthropic размещает цикл агента, и ваше приложение отправляет события и получает потоковые результаты через клиентские SDK или REST API. Выполнение инструментов происходит в управляемой Anthropic облачной песочнице или в [самостоятельно размещаемой песочнице](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) на вашей собственной инфраструктуре.

<h2 id="the-subprocess-model">
  Модель подпроцесса
</h2>

Каждое решение о хостинге на этой странице следует из того, как SDK запускает агента. Когда ваш код вызывает `query()`, SDK порождает отдельный процесс `claude` CLI и взаимодействует с ним через stdio. Этот подпроцесс владеет оболочкой, рабочей директорией и транскриптами сеанса JSONL на локальном диске.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" className="dark:hidden" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/hosting-subprocess-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=3fdeff3d7f44b2b67762668acfbb25f5" className="hidden dark:block" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess-dark.svg" />

Один сеанс агента соответствует одному подпроцессу. Запуск N одновременных сеансов означает N подпроцессов, каждый со своим деревом процессов и файлом транскрипта. По умолчанию все они наследуют рабочую директорию вашего приложения. Когда сеансам требуются отдельные файловые системы, передайте отдельный `cwd` в параметрах каждого вызова `query()` сеанса:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the files in this directory",
    options: { cwd: "/work/session-a" },
  })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt="Summarize the files in this directory",
          options=ClaudeAgentOptions(cwd="/work/session-a"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

Примеры TypeScript на этой странице используют top-level `await`, поэтому сохраняйте их как файлы `.mts` или установите `"type": "module"` в `package.json`.

<h3 id="state-that-lives-on-local-disk">
  Состояние, которое находится на локальном диске
</h3>

Три вида состояния агента по умолчанию находятся в файловой системе контейнера. Ни одно из них не сохраняется при перезагрузке контейнера, масштабировании вниз или перемещении на другой узел.

| Состояние                    | Расположение по умолчанию                                                                    |
| ---------------------------- | -------------------------------------------------------------------------------------------- |
| Транскрипты сеанса           | `~/.claude/projects/`, или директория `projects/` под `CLAUDE_CONFIG_DIR`, если установлена  |
| Файлы памяти `CLAUDE.md`     | `~/.claude/CLAUDE.md` для уровня пользователя и рабочая директория сеанса для уровня проекта |
| Артефакты рабочей директории | Рабочая директория сеанса                                                                    |

Чтобы сохранить транскрипты между хостами, настройте адаптер [`SessionStore`](/docs/ru/agent-sdk/session-storage). Файлы памяти и другие артефакты рабочей директории требуют собственной стратегии хранения, такой как смонтированный том или синхронизация хранилища объектов.

Для информации о том, как сеансы, возобновление и ветвление работают на уровне API, см. [Sessions](/docs/ru/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Выберите паттерн сеанса
</h2>

Эти четыре паттерна охватывают жизненный цикл сеанса: как долго контейнер существует относительно обслуживаемых им сеансов. Для информации о том, где запускается контейнер, в [кулинарной книге по хостингу](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) есть [развертываемый код](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) для локального Docker, Modal и Kubernetes. Выберите паттерн сеанса здесь и цель развертывания из кулинарной книги.

<h3 id="ephemeral-sessions">
  Ephemeral sessions
</h3>

Создайте контейнер для каждой задачи пользователя и уничтожьте его после завершения задачи. Лучше всего подходит для одноразовых задач. Пользователь может по-прежнему взаимодействовать с ИИ во время выполнения задачи, но после завершения контейнер уничтожается.

Примеры рабочих нагрузок включают исследование и исправление ошибок, извлечение счетов и квитанций, перевод документов и преобразование медиа.

Контейнер запускает одноразовую точку входа, которая читает задачу из переменной окружения `TASK_PROMPT`, вызывает SDK и завершает работу.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompt = process.env.TASK_PROMPT!;
  for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt=os.environ["TASK_PROMPT"],
          options=ClaudeAgentOptions(max_turns=20),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

Скрипт выводит каждое сообщение по мере его поступления, включая сообщение результата, чей `subtype` равен `success` при завершении задачи в пределах лимита ходов. Если задача достигает лимита в 20 ходов, `subtype` сообщения результата будет `error_max_turns` и вызов `query()` вызовет ошибку после его выдачи, поэтому оберните цикл в блок try, если контейнер должен завершить работу корректно. Смотрите [Handle the result](/docs/ru/agent-sdk/agent-loop#handle-the-result) для типов ошибок.

<h3 id="long-running-sessions">
  Long-running sessions
</h3>

Запускайте постоянные экземпляры контейнеров, часто размещая несколько процессов SDK на контейнер, для обслуживания текущей работы. Лучше всего подходит для агентов, которые принимают автономные действия, предоставляют контент или обрабатывают высокообъемные потоки сообщений.

Примеры рабочих нагрузок включают агента электронной почты, который сортирует и отвечает на входящую почту, конструктор сайтов, который размещает редактируемый пользователем сайт через порты контейнера, и чат-бота, который обрабатывает непрерывный трафик с платформы, такой как Slack.

Контейнер предоставляет конечную точку HTTP или WebSocket и сопоставляет каждый активный сеанс с долгоживущим запросом и подпроцессом позади него. В TypeScript используйте [`streamInput()`](/docs/ru/agent-sdk/typescript#query-object) для добавления ходов к активному сеансу и [`startup()`](/docs/ru/agent-sdk/typescript#startup) для предварительного прогрева подпроцессов перед входящим трафиком. В Python используйте [`ClaudeSDKClient`](/docs/ru/agent-sdk/python#claudesdkclient) для сохранения сеанса открытым между ходами. Размер контейнера должен быть таким, чтобы он мог вмещать максимальное количество одновременных сеансов в памяти.

<h3 id="hybrid-sessions">
  Hybrid sessions
</h3>

Ephemeral контейнеры, которые гидратируются из [`SessionStore`](/docs/ru/agent-sdk/session-storage) при запуске и сохраняют обновления обратно. Лучше всего подходит для сеансов, которые охватывают много взаимодействий, но остаются неактивными между ними. Контейнер выключается в периоды простоя и снова включается, когда пользователь возвращается.

Примеры рабочих нагрузок включают личного менеджера проектов с периодическими проверками, глубокие исследования, которые приостанавливаются и возобновляются в течение часов, и агента поддержки клиентов, который загружает историю билетов между взаимодействиями.

Настройте тайм-аут простоя вашего провайдера в соответствии с тем, как часто вы ожидаете возвращения пользователей. Выключение контейнера без настроенного `SessionStore` приводит к потере транскрипта вместе с ним, поэтому хранилище требуется для этого паттерна, а не опционально.

Паттерн зависит от возобновления сеанса по ID с подключенным общим хранилищем:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // an object store, key-value store, database, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, SessionStore
  import asyncio

  user_input: str = ...
  session_id: str = ...              # looked up from your database by user
  session_store: SessionStore = ...  # an object store, key-value store, database, or your own adapter


  async def main():
      async for message in query(
          prompt=user_input,
          options=ClaudeAgentOptions(
              resume=session_id,
              session_store=session_store,
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="multi-agent-container">
  Multi-agent container
</h3>

Запускайте несколько подпроцессов SDK внутри одного контейнера. Лучше всего подходит для агентов, которые должны тесно сотрудничать, например для многоагентных симуляций, где агенты взаимодействуют друг с другом в общей среде.

Дайте каждому агенту свой рабочий каталог, чтобы они не перезаписывали файлы друг друга, и изолируйте загрузку параметров, чтобы файлы `CLAUDE.md` для каждого агента не просачивались между агентами. Смотрите [Multi-tenant isolation](#multi-tenant-isolation) для конкретных опций.

<h2 id="provision-the-container">
  Подготовка контейнера
</h2>

<h3 id="container-based-sandboxing">
  Контейнеризованная изоляция
</h3>

Запустите SDK внутри изолированного контейнера для изоляции процессов, ограничения ресурсов, управления сетью и эфемерной файловой системы.

Вопросы, которые следует рассмотреть при выборе поставщика:

* **Кто управляет песочницей**: поставщик услуг песочницы управляет инфраструктурой для вас, в то время как самостоятельно размещаемые варианты предоставляют вам программное обеспечение для запуска на собственном сервере.
* **Задержка холодного запуска**: время от "создания песочницы" до "готовности принять первый запрос". Эфемерные паттерны требуют запусков менее чем за секунду. Долгоживущие паттерны допускают большую задержку.
* **Постоянное хранилище**: предоставляет ли поставщик долговечные тома или только эфемерный диск. Гибридный паттерн требует долговечного хранилища где-то, будь то в песочнице или рядом с ней.
* **Модель ценообразования**: оплата за секунду, за запрос или фиксированная почасовая оплата. Ценообразование за секунду подходит для нестабильных эфемерных рабочих нагрузок. Почасовая оплата подходит для долгоживущих сеансов.
* **Сетевые возможности**: поддержка пользовательских правил исходящего трафика, исходящих прокси и приватного VPC пиринга для регулируемых сред.

Для самостоятельно размещаемых вариантов, таких как Docker, gVisor и Firecracker, а также подробной конфигурации изоляции, см. [Isolation Technologies](/docs/ru/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Зависимости среды выполнения
</h3>

Контейнер требует языковой среды выполнения вашего SDK:

* Python 3.10+ для Python SDK или Node.js 18+ для TypeScript SDK
* Оба SDK TypeScript и Python поставляются с нативным бинарным файлом Claude Code для большинства установок, и порожденный CLI не требует отдельной установки Node.js. См. [примечание об установке в quickstart](/docs/ru/agent-sdk/quickstart) для установок, которым требуется отдельная нативная установка Claude Code.

Поставляемый бинарный файл привязан к версии пакета SDK, поэтому обновление SDK — это способ обновления CLI. SDK следует semver: берите патч-релизы непрерывно и просмотрите [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) или [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) changelog перед тем, как брать минорный релиз.

<h3 id="resources">
  Ресурсы
</h3>

1 ГиБ ОЗУ, 5 ГиБ диска и 1 CPU на агента — это разумная отправная точка для вновь запущенного экземпляра. Использование памяти растет с длительностью сеанса и активностью инструментов, поэтому выбирайте размер в соответствии с фактическими длительностями сеансов и параллелизмом, которые вам нужны, а не с базовым уровнем простоя. См. [Scaling and concurrency](#scaling-and-concurrency) для того, как рассчитать количество агентов на хост.

<h3 id="network">
  Сеть
</h3>

SDK требует исходящего HTTPS к `api.anthropic.com` или к региональной конечной точке вашего поставщика при запуске на Amazon Bedrock или Google Cloud's Agent Platform. Если ваши агенты используют [MCP servers](/docs/ru/agent-sdk/mcp) или внешние инструменты, им также требуется исходящий доступ к этим конечным точкам. Для production маршрутизируйте исходящий трафик через прокси выхода, который обеспечивает списки разрешенных доменов, внедряет учетные данные и регистрирует запросы. См. [Secure Deployment](/docs/ru/agent-sdk/secure-deployment) для полного паттерна.

Для входящего трафика откройте HTTP или WebSocket порт на контейнере. Ваше приложение обрабатывает запросы клиентов на этом порту и вызывает SDK внутренне; сам подпроцесс не прослушивает сеть.

<h2 id="handle-production-concerns">
  Решение проблем в production
</h2>

Примите эти решения перед развертыванием самостоятельно размещаемого агента.

<h3 id="session-and-state-persistence">
  Сохранение сеанса и состояния
</h3>

Локальный диск по умолчанию теряется при перезагрузке, масштабировании или перемещении на другой узел. Для любого сеанса, который пользователь ожидает возобновить, зеркалируйте транскрипт в долговечное хранилище с помощью адаптера [`SessionStore`](/docs/ru/agent-sdk/session-storage). См. [Эталонные реализации](/docs/ru/agent-sdk/session-storage#reference-implementations) для адаптеров хранилища объектов, хранилища ключ-значение и базы данных, а также набор соответствия для вашего собственного.

Три важных момента о том, как ведет себя `SessionStore`:

* **Только транскрипты**: `SessionStore` зеркалирует транскрипты, а не файлы памяти `CLAUDE.md` или другие артефакты рабочего каталога. Подключите общий том или синхронизируйте их отдельно.
* **Зеркалирование, а не замена**: подпроцесс сначала записывает на локальный диск, а SDK пересылает копию каждого пакета в хранилище. Транскрипт нового сеанса на локальном диске пережидает запуск; сеанс, возобновленный из хранилища, удаляет его локальную копию в конце, поэтому хранилище содержит единственную долговечную копию. См. [Архитектура двойной записи](/docs/ru/agent-sdk/session-storage#dual-write-architecture).
* **Сообщения `mirror_error`**: когда SDK не может доставить пакет в хранилище, он отбрасывает пакет, выдает сообщение `{ type: "system", subtype: "mirror_error" }` и продолжает запрос. Установите оповещение на эти события, если долговечность хранилища имеет значение. См. [Зеркальные записи — это лучшее усилие](/docs/ru/agent-sdk/session-storage#mirror-writes-are-best-effort) для поведения повтора и тайм-аута.

<h3 id="observability">
  Наблюдаемость
</h3>

Агенты Agent SDK — это долгоживущие процессы, которые порождают вызовы инструментов во многих раундах API. Без телеметрии вы не сможете увидеть, какие инструменты запустились, сколько времени они заняли или где сеанс застрял.

SDK наследует конфигурацию OpenTelemetry из окружения. Установите переменные окружения OTEL на уровне контейнера или оркестратора, чтобы каждый вызов `query()` экспортировал spans, метрики и события логов в ваш сборщик. Пример ниже включает экспорт OTLP для всех трех сигналов. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` требуется только для трассировок; опустите его, если вы экспортируете только метрики и логи.

```bash title=".env" theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

Текст подсказки и входные данные инструмента по умолчанию не включаются в экспорты. См. [Управление конфиденциальными данными в экспортах](/docs/ru/agent-sdk/observability#control-sensitive-data-in-exports) для флагов согласия и [Наблюдаемость](/docs/ru/agent-sdk/observability) для полного каталога сигналов.

<h3 id="auth-and-secrets">
  Аутентификация и секреты
</h3>

Три проблемы аутентификации имеют значение во время размещения:

* **Anthropic API**: подпроцесс читает `ANTHROPIC_API_KEY` из своего окружения. Предоставьте его из вашего менеджера секретов или установите `ANTHROPIC_BASE_URL` для маршрутизации вызовов модели через прокси, который внедряет ключ вне контейнера. См. [Управление учетными данными](/docs/ru/agent-sdk/secure-deployment#credential-management) для паттерна прокси и [Настройка в быстром старте SDK](/docs/ru/agent-sdk/quickstart#setup) для поддерживаемых методов аутентификации.
* **Входящие**: поместите аутентификацию на шлюз перед контейнером агента. Агент должен получать предварительно аутентифицированные запросы и не должен быть компонентом, который проверяет токены пользователя.
* **Исходящие инструменты**: держите учетные данные инструмента вне окружения агента. Маршрутизируйте исходящие вызовы через прокси, который внедряет ключи API после того, как запрос покидает контейнер. Агент делает вызов; прокси добавляет учетные данные.

<h3 id="scaling-and-concurrency">
  Масштабирование и параллелизм
</h3>

Каждый сеанс работает в своем собственном подпроцессе, поэтому параллелизм на хосте ограничен тем, сколько подпроцессов может вместить его ОЗУ.

Размер каждого хоста по этой формуле:

```text theme={null}
agents per host = (host RAM - overhead) / (per-session RAM ceiling)
```

Измерьте потолок оперативной памяти на сеанс, запустив репрезентативный сеанс до целевой длины при ожидаемой нагрузке инструмента и записав пиковое RSS. Начальная точка 1 ГиБ в [Ресурсы](#resources) — это минимум, а не потолок.

Маршрутизация горизонтального масштабирования зависит от вашего паттерна. Для долгоживущих сеансов, где контейнеры содержат много сеансов, запустите пул контейнеров за балансировщиком нагрузки и закрепите каждый сеанс на одном контейнере, используя согласованное хеширование на `sessionId`. Закрепленный сеанс продолжает обращаться к одному контейнеру, и, следовательно, к одному работающему подпроцессу, пока он не будет вытеснен или контейнер не перезагрузится.

<h3 id="cost">
  Стоимость
</h3>

Стоимость токенов Anthropic обычно превосходит стоимость инфраструктуры контейнера на порядок или больше. Минимально подготовленный контейнер работает примерно за \$0,05 в час, в то время как один долгий сеанс агента может потратить доллары на токены. См. [Отслеживание стоимости](/docs/ru/agent-sdk/cost-tracking) для учета токенов на сеанс.

<h3 id="multi-tenant-isolation">
  Изоляция мультитенантности
</h3>

Поведение SDK по умолчанию читает параметры и файлы памяти `CLAUDE.md` из файловой системы. В общем контейнере, обслуживающем нескольких арендаторов, эти файлы могут утечь контекст одного арендатора в сеанс другого арендатора.

Для изоляции арендаторов внутри общего контейнера:

* Передайте `settingSources: []` в TypeScript или `setting_sources=[]` в Python, чтобы пропустить пользовательские, проектные и локальные параметры.
* Установите `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` в `env`. [Автоматическая память](/docs/ru/memory#auto-memory) в `~/.claude/projects/<project>/memory/` загружается в системную подсказку независимо от `settingSources`. См. [Что settingSources не контролирует](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) для других входных данных, которые загружаются безусловно.
* Укажите `CLAUDE_CONFIG_DIR` на каталог для каждого арендатора, чтобы арендаторы не делили глобальный конфиг `~/.claude.json`. Когда каждый каталог конфигурации обслуживает один рабочий каталог и вы не делите [`SessionStore`](/docs/ru/agent-sdk/session-storage) между арендаторами, вы также можете установить [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ru/sessions#name-the-project-directory-yourself) в `env`, чтобы сохранить пути транскрипта под ним короткими. Требуется TypeScript Agent SDK v0.3.234 или позже, или Python Agent SDK v0.2.140 или позже.
* Используйте рабочий каталог для каждого арендатора. Передайте `cwd` явно при каждом вызове `query()`.
* Применяйте правила исходящего трафика для каждого арендатора на вашем прокси, такие как различные исходящие IP-адреса, учетные данные или списки разрешенных доменов, чтобы скомпрометированный арендатор не мог экспортировать данные через политику исходящего трафика другого арендатора.

Пример ниже применяет параметры, автоматическую память, каталог конфигурации и параметры рабочего каталога вместе. Создайте `tenantDir` и `configDir` так, чтобы каждый арендатор получил путь, который не может прочитать никакой другой арендатор. В TypeScript `env` заменяет окружение подпроцесса, поэтому распространите `...process.env`, чтобы сохранить унаследованные переменные, такие как `PATH` и `ANTHROPIC_API_KEY`. В Python `env` объединяется с унаследованным окружением.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio

  prompt: str = ...
  tenant_dir: str = ...
  config_dir: str = ...


  async def main():
      async for message in query(
          prompt=prompt,
          options=ClaudeAgentOptions(
              cwd=tenant_dir,
              setting_sources=[],
              env={
                  "CLAUDE_CONFIG_DIR": config_dir,
                  "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="known-limitations">
  Известные ограничения
</h2>

Учитывайте их при проектировании развертывания.

| Ограничение                                                                      | Что делать                                                                                                                                                                                                                                                                                                  |
| -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Отсутствие тайм-аута сеанса верхнего уровня                                      | Сеанс не истекает автоматически. Установите `maxTurns` в TypeScript или `max_turns` в Python, чтобы ограничить количество раундов использования инструментов, которые агент выполняет перед остановкой.                                                                                                     |
| Рост памяти при длительных сеансах                                               | Ограничьте длину сеанса или периодически перезапускайте подпроцессы. См. [Масштабирование и параллелизм](#scaling-and-concurrency).                                                                                                                                                                         |
| Большие параллельные развертывания подагентов могут достичь ограничений скорости | Разбивайте работу на меньшие пакеты вместо выполнения одного широкого развертывания.                                                                                                                                                                                                                        |
| Отсутствие крайнего срока по времени стены для каждого подагента                 | Ограничьте каждый [подагент](/docs/ru/agent-sdk/subagents) с помощью `maxTurns` в его `AgentDefinition`. `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` устанавливает сторожевой таймер зависания, который срабатывает, когда подагент перестает выдавать выходные данные; это не крайний срок общего времени выполнения. |

<h2 id="troubleshoot-deployment-failures">
  Troubleshoot deployment failures
</h2>

Используйте этот раздел, когда агент, который работает на вашей машине, не работает в развёрнутом сервисе. Каждый элемент ниже называет сбой и ссылается на запись, которая его охватывает:

* **CLI not found at service start**: в Python контейнер или менеджер сервисов запускает ваше приложение с другим `PATH`, чем ваша оболочка, поэтому установка, которая работает локально, не видна процессу. В TypeScript сборка образа пропустила дополнительные зависимости SDK, или `pathToClaudeCodeExecutable` указывает на файл, который не существует в образе. См. [Claude Code not found](/docs/ru/agent-sdk/troubleshooting#clinotfounderror-claude-code-not-found).
* **CLI present in the image but won't launch**: Claude Code не может запуститься из двоичного файла, который не соответствует архитектуре контейнера или libc, или из файла, который потерял разрешение на выполнение при сборке образа. См. [Failed to start Claude Code](/docs/ru/agent-sdk/troubleshooting#cliconnectionerror-failed-to-start-claude-code).
* **Claude Code process exits mid-run**: ошибка, которую получает ваше приложение, зависит от языка SDK и от того, сообщил ли CLI об ошибке результата в первую очередь. Записи в разделе [CLI process exit](/docs/ru/agent-sdk/troubleshooting#cli-process-exit) охватывают каждое сообщение.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Hosting cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): пошаговое руководство в виде notebook с [развертываемым кодом](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) для Docker, Modal и Kubernetes.
* [Session storage](/docs/ru/agent-sdk/session-storage): сохранение транскриптов между хостами с помощью адаптера `SessionStore`.
* [Observability](/docs/ru/agent-sdk/observability): экспорт OTEL трассировок, метрик и логов в ваш сборщик.
* [Secure deployment](/docs/ru/agent-sdk/secure-deployment): сетевые элементы управления, управление учетными данными и усиление изоляции.
* [Cost tracking](/docs/ru/agent-sdk/cost-tracking): учет токенов и затрат для каждого сеанса.
