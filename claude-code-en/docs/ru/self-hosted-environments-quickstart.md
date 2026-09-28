> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Быстрый старт для самостоятельно размещаемых окружений

> Настройте своё первое самостоятельно размещаемое окружение: установите Claude Code, создайте окружение, запустите runner и маршрутизируйте сеанс на него.

<Note>
  Самостоятельно размещаемые окружения находятся в публичной бета-версии на планах Team и Enterprise; [Доступность и ограничения](/docs/ru/self-hosted-environments#availability-and-limitations) охватывает путь включения. На этой странице вы запустите свой первый сеанс; см. [Самостоятельно размещаемые окружения](/docs/ru/self-hosted-environments) для понимания того, что это такое, и [Развёртывание в production](/docs/ru/self-hosted-environments-deploy) для укрепления и рецептов флота.
</Note>

[Самостоятельно размещаемое окружение](/docs/ru/self-hosted-environments) запускает [облачные сеансы](/docs/ru/claude-code-on-the-web) Claude Code на инфраструктуре, которой управляет ваша организация, выполняемые процессами runner, которые вы развёртываете. Этот быстрый старт настраивает ваше первое окружение, самое маленькое из работающих: один runner на одном хосте, запускающий один тестовый сеанс. Есть два шага: [создайте окружение, запустите runner и маршрутизируйте сеанс на него](#set-up-an-environment-and-runner), затем [отправьте сообщение этому сеансу из вашего терминала](#send-a-follow-up-message-to-a-running-session). Вы будете переключаться между двумя интерфейсами: claude.ai для создания окружения, проверки его статуса и маршрутизации сеанса, и терминалом на хосте для всего, что делает runner.

К концу у вас будет окружение на [странице администратора **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), runner, опрашивающий работу, и сеанс, работающий на вашем хосте. Прежде чем подключать реальные репозитории или внутренние системы, пройдите [Развёртывание в production](/docs/ru/self-hosted-environments-deploy), которое охватывает позицию безопасности, контроль исходящего трафика, учётные данные git и оркестрацию.

<h2 id="prerequisites">
  Предварительные требования
</h2>

<h3 id="organization-and-roles">
  Организация и роли
</h3>

Для claude.ai требуется:

* **Allow self-hosted environments** включено [Владельцем](/docs/ru/cloud-environments#organization-shared-environments) на [странице администратора **Cloud environments**](https://claude.ai/admin-settings/cloud-environments); кнопка **New** не появляется, пока это не будет сделано. Если у вас нет этой роли, кто-то, кто её имеет, может создать окружение и передать вам его секрет; шаги runner и terminal на этой странице не требуют роли claude.ai, и там, где шаг проверяет статус в интерфейсе администратора, собственные строки логов runner дают вам тот же сигнал.
* [Подключение GitHub](/docs/ru/claude-code-on-the-web#github-authentication-options) для вашей организации, чтобы разработчики могли выбирать репозитории при запуске сеансов.

<h3 id="host-and-network">
  Хост и сеть
</h3>

Хост runner требует:

* Хост или контейнер Linux или macOS с исходящим HTTPS к `api.anthropic.com`, к `claude.ai` и хостам загрузки, на которые он перенаправляет для шага установки ниже, и к вашему git-хосту для клонирования; [таблица требований к сети](/docs/ru/self-hosted-environments-deploy#network-requirements) содержит полный список. Windows не поддерживается в качестве хоста runner; запустите runner в контейнере Linux вместо этого. Рабочие станции разработчиков не затронуты, так как сеансы запускаются из claude.ai в браузере.
* Часы, синхронизированные с реальным временем, например с помощью NTP. Аутентификация не удаётся, когда часы отстают или спешат более чем на пять минут; см. [Troubleshooting](/docs/ru/self-hosted-environments-deploy#troubleshooting).

<h3 id="software-on-the-runner-host">
  Программное обеспечение на хосте runner
</h3>

Установите на хост перед началом:

* **Claude Code v2.1.224 или позже**, с любым из [стандартных методов установки](/docs/ru/setup). Runner является частью стандартного бинарного файла `claude`, и более ранние версии не распознают подкоманду `self-hosted-runner`. Канал `latest` встроенного установщика содержит каждый выпуск сразу после его публикации; канал `stable`, cask Homebrew `claude-code` и стабильные репозитории apt, dnf и apk отстают примерно на неделю. Чтобы зафиксировать точную версию, которую запускает ваш парк, см. [Install a specific version](/docs/ru/setup#install-a-specific-version). Для образов контейнеров см. Dockerfile в [Deploy to production](/docs/ru/self-hosted-environments-deploy#build-the-runner-image).
* **Git 2.24 или новее**. Некоторые опции git на странице развёртывания требуют более новых версий; [Configure git](/docs/ru/self-hosted-environments-deploy#configure-git) указывает каждый минимум.

Подтвердите, что хост готов:

```bash theme={null}
claude self-hosted-runner --help
```

Готовый хост выводит текст использования runner, перечисляя флаги такие как `--environment-secret-file`. На версиях старше 2.1.224 команда выводит вместо этого общий вывод `claude --help`; обновитесь с помощью `claude update` или переустановите из канала `latest`.

<h2 id="set-up-an-environment-and-runner">
  Настройка окружения и runner
</h2>

Claude Code включает управляемую установку: интерактивный сеанс Claude Code, который проведёт вас через создание окружения в админ-интерфейсе, запустит локальный runner с файлом секрета, который вы сохраняете, подтвердит, что runner регистрируется, и напишет шпаргалку в `./runner-setup/CHEAT-SHEET.md`. Запустите его на машине, где вы вошли с помощью `claude auth login`, используя учётную запись, которая имеет роль Owner; это недоступно с API ключами или поставщиками моделей третьих сторон. На хостах, где интерактивный сеанс невозможен, используйте вместо этого ручные шаги ниже. Сначала подтвердите, что [проверка версии](#software-on-the-runner-host) прошла: на версиях старше 2.1.224 эта команда запускает обычный сеанс Claude со словами в качестве подсказки вместо управляемой установки. Чтобы запустить управляемую установку, запустите подкоманду setup и следуйте подсказкам:

```bash theme={null}
claude self-hosted-runner setup
```

Для ручной настройки вместо этого:

<Steps>
  <Step title="Создайте окружение">
    Перейдите на [страницу **Cloud environments**](https://claude.ai/admin-settings/cloud-environments) в параметрах администратора. В разделе **Self-hosted environments** выберите **New**, назовите окружение и выберите **Create**. На втором шаге мастера выберите **Copy environment key**, чтобы скопировать секрет окружения, который админ-интерфейс обозначает как ключ окружения. claude.ai показывает секрет один раз, и вы не можете получить его позже; он истекает через 365 дней после создания. ID окружения `ccpool_...` остаётся видимым в его диалоговом окне деталей; вам понадобится он для проверки `aud` в [проверке токена](/docs/ru/self-hosted-environments-identity) и для отправки [тестовых сеансов из CI](/docs/ru/self-hosted-environments-testing#run-the-test-loop).

    Если вы потеряли секрет или вам нужно его ротировать, создайте новый секрет на вкладке **Configuration** окружения, разверните новый секрет на ваших runners, затем отозовите старый. Runners, держащие отозванный секрет, не пройдут свой следующий аутентифицированный опрос и выйдут, логируя `poll auth failed`, и ваш оркестратор перезапустит их с новым секретом.
  </Step>

  <Step title="Запустите runner">
    Создайте директорию секрета. Этот шаг и следующий требуют root для пути `/etc/claude`; любой путь, который процесс runner может читать, работает, поэтому отрегулируйте обе команды и значение `--environment-secret-file` вместе, если вы используете другой.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    Запишите секрет окружения в файл. Команда ниже читает из вашего терминала, поэтому секрет остаётся вне истории shell: вставьте значение, которое вы скопировали, нажмите Enter, затем Ctrl-D, и `umask` подоболочки делает файл читаемым только его владельцем.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    Выберите базовую директорию, заменив `<writable-dir>` в команде runner ниже абсолютным путём, который runner может писать или создавать. Runner создаёт директорию при запуске, затем проверяет репозитории и создаёт директории для каждого сеанса под ней. Без `--base-dir` он использует `/workspace`, что работает только если эта директория уже существует и доступна для записи или вы запускаете runner как root.

    Если runner не может создать или писать в путь, он выходит при запуске с ошибкой, называющей директорию вместо регистрации. См. [Troubleshooting](/docs/ru/self-hosted-environments-deploy#troubleshooting).

    Затем запустите runner с `--environment-secret-file` и `--base-dir`. Runner регистрируется в вашем окружении и начинает опрашивать работу. Если runner выходит, перезапустите его вручную. Production развёртывания запускают runner под оркестратором, который перезапускает вышедшие runners, обычно со свежей файловой системой при каждом перезапуске; [Reuse a pre-warmed checkout](/docs/ru/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) охватывает поддерживаемую настройку постоянного диска.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="Проверьте, что runner появился">
    Вернитесь на [страницу **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Статус вашего окружения изменяется с **No runners deployed** на **Healthy** в течение нескольких секунд после запуска runner; откройте окружение и выберите **Activity**, чтобы увидеть сам runner.
  </Step>

  <Step title="Маршрутизируйте сеанс на окружение">
    Запустите сеанс на claude.ai/code и выберите ваше окружение из средства выбора окружения, где самостоятельно размещаемые окружения появляются рядом с размещаемыми Anthropic. Runner клонирует с любыми учётными данными git, которые хост уже имеет, поэтому выберите репозиторий, который этот хост уже может клонировать, или публичный; опции учётных данных для приватных репозиториев в production находятся на [Configure git](/docs/ru/self-hosted-environments-deploy#configure-git). Следующий доступный runner подхватывает поставленный в очередь сеанс и логирует `Picked up session <session-id>` вместе с его активным счётом и ёмкостью, поэтому вы можете подтвердить из собственного вывода runner, какой хост взял сеанс. Смотрите, как работает сеанс, и читайте ответы Claude на [claude.ai/code](https://claude.ai/code). Если сеанс остаётся в очереди вместо этого, см. [Troubleshooting](/docs/ru/self-hosted-environments-deploy#troubleshooting).
  </Step>
</Steps>

Runner выходит по дизайну после завершения его активных сеансов; см. [Runner lifecycle](/docs/ru/self-hosted-environments#runner-lifecycle). Для production развёртывайте его под оркестратором, который перезапускает его при выходе. См. [Развёртывание в production](/docs/ru/self-hosted-environments-deploy).

<h2 id="send-a-follow-up-message-to-a-running-session">
  Отправьте follow-up сообщение работающему сеансу
</h2>

Когда сеанс работает на вашем окружении, отправьте ему follow-up из CLI `claude` на любой машине, где вы вошли с помощью `claude auth login`; команда не должна запускаться с машины, которая запустила сеанс. Команда отправляет одно сообщение:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Для `<session-id>` передайте голый ID `session_...` или `cse_...` или URL сеанса claude.ai/code. Успешная отправка выводит `Sent to cloud session.` с ID сеанса и ссылкой просмотра. Принятые формы ID, вывод JSON, требования учётной записи и политики, и справочник ошибок находятся на [Send follow-ups from the CLI](/docs/ru/claude-code-on-the-web#send-follow-ups-from-the-cli), так как команда работает одинаково против сеансов, размещаемых Anthropic.

<h2 id="what’s-next">
  Что дальше
</h2>

* [Развёртывание в production](/docs/ru/self-hosted-environments-deploy): укрепите развёртывание, контролируйте исходящий трафик, настройте учётные данные git и запустите флот под Kubernetes или Compose
* [Customize sessions](/docs/ru/self-hosted-environments-configuration): скрипты-обёртки, хуки жизненного цикла, runners по требованию, MCP серверы и разрешения
* [Test end to end](/docs/ru/self-hosted-environments-testing): CI smoke тест, который отправляет сеанс и читает ответы Claude
