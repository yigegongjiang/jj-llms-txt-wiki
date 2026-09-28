> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Запускайте Claude Code в рабочих процессах GitHub Actions для ответа на упоминания @claude, автоматизации задач и преобразования issues в pull requests

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) — это GitHub Action, который запускает Claude Code внутри рабочих процессов вашего репозитория. Упомяните `@claude` в комментарии pull request или issue, чтобы Claude анализировал код, реализовывал изменения и отправлял коммиты. Вы также можете дать Claude Code GitHub Action prompt для автоматического запуска на любом событии GitHub. Используйте его для преобразования issues в pull requests, исправления ошибок из комментария или автоматизации повторяющихся задач.

Несколько продуктов используют имя Claude Code. На этой странице рассматривается интеграция рабочего процесса `claude-code-action`, которую вы настраиваете с помощью файлов рабочего процесса в вашем репозитории. Для связанных продуктов см.:

* [Code Review](/docs/ru/code-review): автоматическая проверка на каждом pull request без написания рабочего процесса
* [Claude Code в облаке](/docs/ru/claude-code-on-the-web): сеансы Claude Code, которые работают на облачной инфраструктуре вместо вашей машины
* [Claude Agent SDK](/docs/ru/agent-sdk/overview): пользовательская автоматизация вне GitHub Actions. Claude Code GitHub Action построен на основе SDK
* [GitHub Enterprise Server](/docs/ru/github-enterprise-server): Claude Code с самостоятельно размещённым GitHub

<h2 id="setup">
  Настройка
</h2>

Вы можете настроить Claude Code GitHub Action одним из двух способов:

* **Быстрая настройка**: запустите `/install-github-app` из Claude Code. Claude Code устанавливает GitHub App, добавляет ваш секрет аутентификации и подготавливает pull request рабочего процесса для вас
* **Ручная настройка**: установите приложение, добавьте секрет и скопируйте файл рабочего процесса в ваш репозиторий самостоятельно. Используйте этот путь, когда вы не запускаете Claude Code локально, когда команда не работает или когда вы хотите полный контроль над файлами рабочего процесса

Для любого пути вам нужен доступ администратора к репозиторию.

<h3 id="quick-setup">
  Быстрая настройка
</h3>

`/install-github-app` работает только с репозиториями github.com. Если удалённый репозиторий находится на gitlab.com или bitbucket.org, команда выводит уведомление и выходит вместо начала настройки. Чтобы запустить Claude Code из конвейеров GitLab, см. [Claude Code GitLab CI/CD](/docs/ru/gitlab-ci-cd).

Перед началом установите [GitHub CLI](https://cli.github.com) и аутентифицируйте его с помощью `gh auth login`. Claude Code проверяет его наличие и предупреждает вас, если он отсутствует.

Откройте `claude` в репозитории, который вы хотите подключить, запустите `/install-github-app` и следуйте подсказкам. Claude Code устанавливает Claude GitHub App, затем настраивает секрет аутентификации для рабочих процессов:

* Если Claude Code уже имеет API ключ, он повторно использует этот ключ и предлагает сохранить существующий секрет `ANTHROPIC_API_KEY` репозитория, если он уже установлен
* В противном случае выберите между созданием долгоживущего токена с вашей подпиской Claude и вставкой API ключа

Claude Code сохраняет учётные данные как секрет репозитория с именем `ANTHROPIC_API_KEY` для API ключа или `CLAUDE_CODE_OAUTH_TOKEN` для токена подписки.

Claude Code затем отправляет ветку с выбранными файлами рабочего процесса, уже настроенными на использование этого секрета, и открывает GitHub в вашем браузере с готовым к созданию pull request. Создайте и объедините этот pull request, и `@claude` будет работать в репозитории.

Если вы выберете рабочий процесс проверки, Claude размещает каждую проверку на самом pull request как встроенный комментарий к каждой найденной проблеме или как один сводный комментарий, когда он не находит ничего. Claude пропускает некоторые pull requests, такие как черновики. Пример [рабочего процесса проверки](#run-a-skill) использует тот же skill и перечисляет их. До версии 2.1.229 Claude писал свою проверку только в журнал выполнения рабочего процесса.

Чтобы обновить рабочий процесс проверки, созданный более ранней версией, выполните одно из следующих действий:

* Запустите `/install-github-app` снова. Когда репозиторий уже имеет `claude.yml`, выберите **Update workflow file with latest version**. Claude Code отправляет свежие копии файлов рабочего процесса в новую ветку и открывает pull request, как при первой установке.
* Добавьте аргумент `--comment` и строку `claude_args` из [примера рабочего процесса проверки](#run-a-skill) в зафиксированный файл самостоятельно, что сохраняет любые другие внесённые вами изменения.

После установки GitHub App Claude Code спрашивает, продолжить ли с настройкой GitHub Actions. Выберите **Skip for now**, чтобы остановиться только с установленным GitHub App. Запустите `/install-github-app` снова позже, чтобы завершить шаги рабочего процесса и секрета.

<Note>
  * Когда вы устанавливаете GitHub App, вы предоставляете ему несколько разрешений. Полный набор см. в [разрешениях GitHub App](#github-app-permissions)
  * Быстрая настройка работает с Claude API и подписками Claude. Если вы используете Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry, см. [Использование Claude Code GitHub Actions с облачными провайдерами](/docs/ru/github-actions-cloud-providers)
</Note>

<h3 id="manual-setup">
  Ручная настройка
</h3>

Чтобы настроить Claude Code GitHub Action без запуска `/install-github-app`, установите приложение, добавьте секрет и скопируйте файл рабочего процесса самостоятельно:

<Steps>
  <Step title="Установите Claude GitHub App">
    Установите [Claude GitHub App](https://github.com/apps/claude) в ваш репозиторий. Claude Code GitHub Action полагается на три разрешения приложения:

    * **Contents**: чтение и запись, чтобы Claude мог изменять файлы репозитория
    * **Issues**: чтение и запись, чтобы Claude мог отвечать на issues
    * **Pull requests**: чтение и запись, чтобы Claude мог создавать PR и отправлять изменения

    Во время установки вы также предоставляете разрешения, которые используют другие функции Claude. Полный набор см. в [разрешениях GitHub App](#github-app-permissions).
  </Step>

  <Step title="Добавьте секрет аутентификации">
    Добавьте один из следующих секретов в ваш репозиторий в зависимости от того, как вы аутентифицируетесь. См. руководство GitHub по [использованию секретов в GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    * `ANTHROPIC_API_KEY`: Claude API ключ из [Claude Console](https://platform.claude.com)
    * `CLAUDE_CODE_OAUTH_TOKEN`: OAuth токен, который аутентифицируется с вашей подпиской Claude, доступный в планах Pro, Max, Team и Enterprise. Создайте его, запустив `claude setup-token` локально. См. [Создание долгоживущего токена](/docs/ru/authentication#generate-a-long-lived-token)

    В файлах рабочего процесса передайте секрет соответствующему входу: `anthropic_api_key` для API ключа или `claude_code_oauth_token` для OAuth токена.
  </Step>

  <Step title="Скопируйте файл рабочего процесса">
    Скопируйте [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) в каталог `.github/workflows/` вашего репозитория. Файл является рабочим рабочим процессом, а не просто примером. Как зафиксировано, Claude отвечает всякий раз, когда кто-то упоминает `@claude` в issue или pull request, аутентифицируясь с помощью секрета `ANTHROPIC_API_KEY`. Если вы добавили `CLAUDE_CODE_OAUTH_TOKEN` вместо этого, измените строку `anthropic_api_key` рабочего процесса на `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.
  </Step>
</Steps>

<Tip>
  После настройки протестируйте Claude Code GitHub Action, отметив `@claude` в комментарии issue или PR.
</Tip>

<h3 id="set-up-for-an-organization">
  Настройка для организации
</h3>

С быстрой настройкой или ручной настройкой вы настраиваете один репозиторий за раз. Чтобы развернуть Claude Code GitHub Action по всей организации:

* Установите [Claude GitHub App](https://github.com/apps/claude) один раз на уровне организации, выбрав все репозитории или выбранный список
* Сохраните секрет аутентификации как секрет Actions на уровне организации, чтобы каждому репозиторию не нужна была своя копия
* Добавьте файл рабочего процесса в каждый репозиторий, который должен запускать Claude Code GitHub Action, или определите задание один раз как [переиспользуемый рабочий процесс](https://docs.github.com/en/actions/using-workflows/reusing-workflows), который каждый репозиторий вызывает

Для секрета, общего для репозиториев, аутентифицируйтесь с помощью API ключа из [Claude Console](https://platform.claude.com) вместо OAuth токена, так как OAuth токен привязан к подписке человека, который запустил `claude setup-token`.

Чтобы избежать сохранения долгоживущего секрета вообще, аутентифицируйтесь через федерацию рабочей идентификации, где Claude Code GitHub Action обменивает токен GitHub OpenID Connect (OIDC) рабочего процесса на доступ к Claude API через сервисный аккаунт Claude Console. Установите эти входы:

* `anthropic_federation_rule_id`: ID правила федерации, `fdrl_...`
* `anthropic_organization_id`: ID вашей организации Anthropic
* `anthropic_service_account_id`: ID сервисного аккаунта, `svac_...`. Опционально, так как правило федерации, которое вы создаёте в Console, уже нацелено на сервисный аккаунт
* `anthropic_workspace_id`: ID рабочей области, `wrkspc_...`. Опционально, когда правило федерации нацелено на одну рабочую область

Предоставьте рабочему процессу разрешение `id-token: write`, которое Claude Code GitHub Action требует для обмена федерации даже когда вы передаёте свой собственный `github_token`. См. [руководство по настройке Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) для конфигурации на стороне Console.

Для вопросов об обработке данных и сохранении в проверке безопасности см. [использование данных](/docs/ru/data-usage) и [безопасность](/docs/ru/security).

<h3 id="uninstall">
  Удаление
</h3>

Чтобы удалить Claude Code GitHub Action, отмените каждую часть настройки, которая применяется к вашей установке:

* **Файлы рабочего процесса**: удалите рабочие процессы, которые используют `anthropics/claude-code-action` из `.github/workflows/`. Если вы использовали быструю настройку, ищите `claude.yml` и, если вы выбрали рабочий процесс проверки, `claude-code-review.yml`. С удалёнными рабочими процессами Claude Code GitHub Action больше не запускается
* **Секреты**: удалите секрет `ANTHROPIC_API_KEY` или `CLAUDE_CODE_OAUTH_TOKEN` из репозитория и из секретов Actions на уровне организации, если вы [поделились им между репозиториями](#set-up-for-an-organization). Если вы удалите секрет, учётные данные, которые он содержал, остаются действительными. Чтобы полностью отозвать API ключ, также удалите ключ в [Claude Console](https://platform.claude.com)
* **GitHub App**: удалите Claude GitHub App в параметрах вашего репозитория или организации в разделе GitHub Apps, но только если вы не используете его для другой функции Claude, такой как Code Review или веб-автоисправление

Если вы настроили [облачного провайдера](/docs/ru/github-actions-cloud-providers), также удалите секреты провайдера, такие как `AWS_ROLE_TO_ASSUME`, секреты `GCP_*` или секреты `AZURE_*`, и удалите пользовательский GitHub App вместе с его секретами `APP_ID` и `APP_PRIVATE_KEY`.

<h3 id="github-app-permissions">
  Разрешения GitHub App
</h3>

[Claude GitHub App](https://github.com/apps/claude) используется каждой функцией Claude, которая интегрируется с GitHub, включая Claude Code GitHub Action, [Code Review](/docs/ru/code-review) и [автоисправление для pull requests](/docs/ru/claude-code-on-the-web#auto-fix-pull-requests) в облачных сеансах. GitHub App имеет единый набор разрешений, охватывающий все его функции, поэтому набор включает некоторые разрешения, которые Claude Code GitHub Action не использует.

Когда вы устанавливаете приложение, вы предоставляете следующие разрешения:

| Разрешение       | Доступ          |
| ---------------- | --------------- |
| Actions          | Чтение и запись |
| Checks           | Чтение и запись |
| Contents         | Чтение и запись |
| Discussions      | Чтение и запись |
| Issues           | Чтение и запись |
| Members          | Чтение          |
| Metadata         | Чтение          |
| Pull requests    | Чтение и запись |
| Repository hooks | Чтение и запись |
| Statuses         | Чтение          |
| Workflows        | Чтение и запись |

Набор разрешений также может измениться раньше функций, которые его используют. Когда приложение запрашивает разрешение, которое оно не имело раньше, GitHub предлагает владельцу аккаунта одобрить его, владельцу организации для установки организации, и установка сохраняет свои старые разрешения до тех пор, пока они это не сделают. Например, когда доступ Actions изменяется с чтения на запись, приложение может повторно запускать рабочие процессы вместо только просмотра запусков и журналов, поэтому GitHub просит владельца одобрить изменение.

Когда вы устанавливаете приложение, вы принимаете его полный набор разрешений. GitHub не позволяет вам принять подмножество. Если ваша организация требует только разрешений, которые использует Claude Code GitHub Action, создайте пользовательский GitHub App с Contents, Issues и Pull requests вместо этого, следуя [руководству по настройке Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md). Пользовательское приложение охватывает только Claude Code GitHub Action. Code Review и веб-автоисправление по-прежнему требуют официального приложения.

Для деталей о том, как Claude Code GitHub Action ограничивает то, что Claude может делать с этими разрешениями, см. [документацию по безопасности](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h2 id="interactive-and-automation-modes">
  Интерактивный и режимы автоматизации
</h2>

Claude Code GitHub Action обнаруживает, как запускаться из конфигурации вашего рабочего процесса:

* **Интерактивный режим**: когда рабочий процесс не предоставляет вход `prompt`, Claude ждёт фразу триггера, `@claude` по умолчанию, в комментарии issue или pull request, в проверке pull request или в теле или заголовке вновь открытого issue, затем отвечает на этот запрос. Прогресс и результаты появляются как комментарий к вызывающему issue или PR.
* **Режим автоматизации**: когда рабочий процесс предоставляет вход `prompt`, Claude запускается без ожидания упоминания, подчиняясь только [проверкам того, кто может запускать запуски](#who-can-trigger-runs). По умолчанию результаты появляются в журнале выполнения рабочего процесса, а не в комментарии. Claude может размещать на issue или pull request, когда prompt направляет его и у него есть инструмент, который может размещать, как в [примере code-review](#run-a-skill).

<h3 id="who-can-trigger-runs">
  Кто может запускать запуски
</h3>

В обоих режимах Claude Code GitHub Action запускает две проверки на действующего лица перед началом Claude, и запуск не удаётся, когда любая проверка его отклоняет:

* **Доступ на запись**: на событиях issue и pull request пользователь, запускающий, должен иметь доступ на запись к репозиторию. Чтобы разрешить конкретным пользователям без доступа на запись, установите `allowed_non_write_users` и передайте свой собственный вход `github_token`. События, которые не создаёт ни один пользователь, такие как триггер `schedule`, пропускают эту проверку.
* **Человеческий актёр**: на каждом событии Claude Code GitHub Action отклоняет актёра-бота, если вы не перечислите его в `allowed_bots`, что предотвращает запуск ботов Claude в цикле. Эта проверка также применяется к запланированным запускам, которые GitHub приписывает пользователю репозитория, обычно тому, кто последний изменил расписание `cron` рабочего процесса. Если этот пользователь является ботом, перечислите его в `allowed_bots`.

<h2 id="example-use-cases">
  Примеры использования
</h2>

Каталог [примеров](https://github.com/anthropics/claude-code-action/tree/main/examples) содержит готовые к использованию рабочие процессы для различных сценариев.

Примеры на этой странице показывают аутентификацию API ключа. Если вы аутентифицируетесь с подпиской Claude, замените строку `anthropic_api_key` в любом примере на `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.

<h3 id="respond-to-claude-mentions">
  Ответ на упоминания @claude
</h3>

Этот рабочий процесс запускает Claude Code GitHub Action в интерактивном режиме, поэтому Claude отвечает всякий раз, когда кто-то упоминает `@claude` в комментарии issue или PR.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

Части этого рабочего процесса, которые не являются шаблонным кодом:

* `id-token: write`: требуется для аутентификации GitHub App Claude Code GitHub Action по умолчанию
* `actions: read`: позволяет Claude читать результаты CI на PR
* `actions/checkout`: даёт Claude локальную копию репозитория для работы
* `if`: предотвращает запуск runners на комментариях, которые не упоминают `@claude`. Claude Code GitHub Action также проверяет саму фразу триггера перед ответом

После того как рабочий процесс установлен, упомяните `@claude` в любом комментарии issue или PR с запросом:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude отвечает в комментарии на том же issue или PR и обновляет его по мере работы.

<h3 id="run-a-skill">
  Запуск skill
</h3>

Вход `prompt` принимает [вызов skill](/docs/ru/skills) а также простой текст:

* Для skill в каталоге `.claude/skills/` вашего репозитория запустите `actions/checkout` перед шагом `anthropics/claude-code-action`, чтобы файлы skill были доступны на runner, затем передайте `/skill-name` как `prompt`.
* Для skill, упакованного в [plugin](/docs/ru/plugins/overview), установите plugin с входами `plugin_marketplaces` и `plugins`, затем передайте пространство имён `/plugin-name:skill-name` как `prompt`. Вход `plugins` принимает `plugin-name@marketplace-name`, где имя маркетплейса поступает из собственного манифеста маркетплейса, а не из URL его репозитория.

Следующий рабочий процесс устанавливает plugin `code-review` и запускает его skill, когда pull request открывается, обновляется, переоткрывается или отмечается как готовый к проверке. Он запускает тот же plugin, что и рабочий процесс проверки из быстрой настройки. Используйте рабочий процесс, подобный этому, когда вы хотите контролировать prompt, модель и триггеры самостоятельно. Для автоматических проверок без поддержания файла рабочего процесса см. [Code Review](/docs/ru/code-review). На публичных репозиториях GitHub скрывает секреты от запусков, запущенных pull requests из форков, поэтому проверка запускается только на pull requests из веток в том же репозитории.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

Две строки в этом рабочем процессе контролируют, где идёт проверка:

* **`--comment`**: Claude размещает свою проверку на pull request как встроенный комментарий к каждой найденной проблеме или как один сводный комментарий, когда он не находит ничего. Без него Claude ничего не размещает, и вы читаете результаты в журнале выполнения рабочего процесса.
* **`claude_args`**: сохраняйте эту строку, даже хотя собственный `allowed-tools` frontmatter skill называет тот же инструмент, потому что Claude Code GitHub Action запускает MCP сервер, который размещает встроенные комментарии только когда `--allowedTools` в `claude_args` его называет.

Claude пропускает черновики и закрытые pull requests, pull requests, которые он судит не нуждаются в проверке, такие как автоматизированные или тривиальные, и pull requests, которые уже имеют комментарий от Claude.

<h3 id="run-on-a-schedule">
  Запуск по расписанию
</h3>

С входом `prompt` Claude Code GitHub Action запускается в режиме автоматизации на любом событии GitHub, включая расписание cron. Для простого текстового prompt Claude не имеет доступа к shell или GitHub API до тех пор, пока вы не предоставите инструменты, которые требует prompt, с `--allowedTools` в `claude_args` или правилом [`permissions.allow`](/docs/ru/permissions#permission-rule-syntax) во входе `settings`. Если вы вместо этого вызываете skill, Claude может использовать инструменты, которые его [`allowed-tools` frontmatter](/docs/ru/skills#pre-approve-tools-for-a-skill) предоставляет. GitHub запускает запланированные рабочие процессы только из ветки по умолчанию и, в публичных репозиториях, отключает расписание после 60 дней без активности репозитория.

Этот рабочий процесс генерирует отчёт в журнале выполнения рабочего процесса в 09:00 UTC каждый день. Его строка `claude_args` [передаёт аргументы CLI](#pass-cli-arguments), которые выбирают модель и разрешают два инструмента GitHub MCP. Claude читает коммиты и issues через GitHub API с этими инструментами, поэтому вы можете опустить шаг checkout:

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  Лучшие практики
</h2>

<h3 id="define-project-standards-in-claude-md">
  Определите стандарты проекта в CLAUDE.md
</h3>

Создайте файл `CLAUDE.md` в корне вашего репозитория для определения рекомендаций по стилю кода, критериев проверки, правил, специфичных для проекта, и предпочитаемых паттернов. Claude следует этим рекомендациям при создании PR и ответе на запросы. Подробности см. в [документации по памяти](/docs/ru/memory).

<h3 id="protect-your-credentials">
  Защитите ваши учётные данные
</h3>

<Warning>
  Никогда не коммитьте API ключи или OAuth токены непосредственно в ваш репозиторий. Всегда сохраняйте их как GitHub Secrets и ссылайтесь на них в рабочих процессах, например `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

Предоставьте рабочему процессу только разрешения, которые ему нужны, и проверьте изменения Claude перед слиянием.

Для полного руководства по безопасности, включая разрешения и аутентификацию, см. [документацию по безопасности Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h3 id="manage-costs">
  Управляйте затратами
</h3>

Каждый запуск потребляет два вида ресурсов:

* **Минуты GitHub Actions**: Claude Code GitHub Action запускается на размещённых GitHub runners, которые потребляют ваши минуты GitHub Actions. Цены и лимиты минут см. в [документации по выставлению счётов GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions).
* **Токены API**: каждое взаимодействие потребляет токены на основе длины prompts и ответов, сложности задачи и размера кодовой базы. Текущие ставки токенов см. на [странице цен Claude](https://claude.com/platform/api). Если вы аутентифицируетесь с OAuth токеном, запуски используют вашу подписку Claude вместо выставления счётов API.

Вы можете снизить оба вида затрат, дав Claude более чёткий контекст и ограничив, сколько работы может выполнить каждый запуск:

* Напишите специфичные запросы `@claude`, чтобы Claude нужно было меньше ходов для завершения
* Используйте шаблоны issues для предоставления контекста заранее
* Держите ваш `CLAUDE.md` кратким, так как Claude читает его на каждом запуске
* Установите `--max-turns` в `claude_args` для ограничения итераций
* Установите тайм-ауты на уровне рабочего процесса, чтобы избежать неконтролируемых заданий
* Используйте элементы управления параллелизмом GitHub для ограничения параллельных запусков

Для отслеживания использования по всей организации см. [панель аналитики](/docs/ru/analytics) и [мониторинг](/docs/ru/monitoring-usage). Для того, как измеряется использование и выставляется счёт, см. [затраты](/docs/ru/costs).

<h2 id="use-a-cloud-provider">
  Используйте облачного провайдера
</h2>

По умолчанию Claude Code GitHub Action вызывает Claude API напрямую с вашим API ключом или OAuth токеном. Чтобы маршрутизировать вывод через ваш собственный облачный аккаунт вместо этого, установите вход для вашего провайдера и следуйте [Использование Claude Code GitHub Actions с облачными провайдерами](/docs/ru/github-actions-cloud-providers):

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Со всеми тремя провайдерами вы аутентифицируетесь через федерацию рабочей идентификации OIDC вместо Claude API ключа, поэтому вы не сохраняете статические облачные учётные данные в вашем репозитории.

<h2 id="troubleshooting">
  Устранение неполадок
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude не отвечает на команды @claude
</h3>

* Проверьте, что GitHub App установлен на репозитории
* Проверьте, что рабочие процессы включены для репозитория
* Убедитесь, что ваш API ключ или OAuth токен установлен в секретах репозитория
* Подтвердите, что комментарий содержит `@claude` как полное слово, а не `/claude` или `@claude-bot`
* Подтвердите, что пользователь, оставляющий комментарий, имеет доступ на запись к репозиторию. Исключения см. в [Кто может запускать запуски](#who-can-trigger-runs)

<h3 id="ci-not-running-on-claude’s-commits">
  CI не запускается на коммитах Claude
</h3>

* GitHub не запускает рабочие процессы на коммитах, сделанных с помощью `GITHUB_TOKEN` по умолчанию. Если вы передаёте `github_token: ${{ secrets.GITHUB_TOKEN }}` в Claude Code GitHub Action, удалите его, чтобы он аутентифицировался как Claude GitHub App, или передайте токен пользовательского приложения вместо этого
* Проверьте, что триггеры рабочего процесса CI включают события, которые производят отправки Claude, такие как `push` или `pull_request`

<h3 id="authentication-errors">
  Ошибки аутентификации
</h3>

* Подтвердите, что API ключ или OAuth токен действителен, протестировав его локально с `claude` перед отладкой рабочего процесса
* Для Bedrock, Agent Platform и Foundry см. раздел [troubleshooting](/docs/ru/github-actions-cloud-providers#troubleshooting) страницы облачного провайдера

Для дополнительных решений см. [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) Claude Code GitHub Action.

<h2 id="advanced-configuration">
  Расширенная конфигурация
</h2>

<h3 id="action-parameters">
  Параметры действия
</h3>

Это наиболее часто используемые входы. Каждый соответствует ключу `with:` в шаге `anthropics/claude-code-action`.

| Параметр                  | Описание                                                                                                                                                                  | Требуется                                                                                                                                                                                    |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Инструкции для Claude как простой текст или [вызов skill](/docs/ru/skills). Когда опущено, Claude отвечает на [фразу триггера](#interactive-and-automation-modes) вместо этого | Нет                                                                                                                                                                                          |
| `claude_args`             | Аргументы CLI, передаваемые в Claude Code                                                                                                                                 | Нет                                                                                                                                                                                          |
| `anthropic_api_key`       | Claude API ключ                                                                                                                                                           | Для Claude API, если вы не используете `claude_code_oauth_token` или [федерацию рабочей идентификации](#set-up-for-an-organization). Не используется для Bedrock, Agent Platform или Foundry |
| `claude_code_oauth_token` | OAuth токен для аутентификации с подпиской Claude, созданный с помощью `claude setup-token`                                                                               | Нет                                                                                                                                                                                          |
| `github_token`            | Токен для операций GitHub. Когда опущено, Claude Code GitHub Action аутентифицируется как Claude GitHub App                                                               | Нет                                                                                                                                                                                          |
| `plugin_marketplaces`     | Список URL-адресов Git маркетплейсов плагинов, разделённый переносами строк                                                                                               | Нет                                                                                                                                                                                          |
| `plugins`                 | Список имён плагинов для установки перед выполнением, разделённый переносами строк                                                                                        | Нет                                                                                                                                                                                          |
| `settings`                | Параметры Claude Code как строка JSON или путь к файлу JSON параметров                                                                                                    | Нет                                                                                                                                                                                          |
| `trigger_phrase`          | Фраза триггера, на которую Claude отвечает. По умолчанию: `@claude`                                                                                                       | Нет                                                                                                                                                                                          |
| `use_bedrock`             | Используйте Amazon Bedrock вместо Claude API                                                                                                                              | Нет                                                                                                                                                                                          |
| `use_vertex`              | Используйте Google Cloud's Agent Platform вместо Claude API                                                                                                               | Нет                                                                                                                                                                                          |
| `use_foundry`             | Используйте Microsoft Foundry вместо Claude API                                                                                                                           | Нет                                                                                                                                                                                          |

Для полного списка входов см. [справочник конфигурации](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) Claude Code GitHub Action.

<h3 id="pass-cli-arguments">
  Передайте аргументы CLI
</h3>

Параметр `claude_args` принимает любой [аргумент Claude Code CLI](/docs/ru/cli-reference):

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Распространённые аргументы:

* `--max-turns`: ограничить количество ходов разговора
* `--model`: модель для использования, например `claude-sonnet-5`. Без этого аргумента Claude Code GitHub Action использует Claude Code [модель по умолчанию](/docs/ru/model-config)
* `--mcp-config`: путь к [конфигурации MCP](/docs/ru/mcp)
* `--allowedTools`: список разрешённых инструментов, разделённый запятыми. Также работает псевдоним `--allowed-tools`
* `--debug`: включить вывод отладки

<h2 id="upgrade-from-beta">
  Обновление с бета-версии
</h2>

Если ваши рабочие процессы по-прежнему ссылаются на `anthropics/claude-code-action@beta`, обновите их до v1:

1. Измените `@beta` на `@v1` в строке `uses`
2. Удалите вход `mode`, так как Claude Code GitHub Action теперь [автоматически обнаруживает режим](#interactive-and-automation-modes)
3. Замените `direct_prompt` на `prompt`
4. Переместите параметры CLI, такие как `max_turns` и `model`, в `claude_args`. `custom_instructions` не имеет флага с тем же именем и становится `--append-system-prompt`

Для полного сопоставления входов и примеров до и после см. [руководство по миграции](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md).

<h2 id="what’s-next">
  Что дальше
</h2>

* [Использование Claude Code GitHub Actions с облачными провайдерами](/docs/ru/github-actions-cloud-providers): маршрутизируйте вывод через Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry
* [Справочник конфигурации](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs): полный список входов действия
* [Каталог примеров](https://github.com/anthropics/claude-code-action/tree/main/examples): готовые к использованию рабочие процессы для дополнительных сценариев
* [Code Review](/docs/ru/code-review): автоматическая проверка pull request без поддержания файла рабочего процесса
