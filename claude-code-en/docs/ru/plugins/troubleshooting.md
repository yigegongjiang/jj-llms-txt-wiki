> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Устранение неполадок плагинов

> Исправьте ошибки плагинов в Claude Code. Найдите точное сообщение об ошибке, сгруппированное по этапам от запуска /plugin до установки и политики организации.

На этой странице перечислены сообщения об ошибках и симптомы для плагинов Claude Code и для маркетплейсов — каталогов, из которых Claude Code устанавливает плагины. Каждая запись содержит причину, одно решение и то, что вы видите после применения исправления.

Если сообщение содержит имя плагина или маркетплейса, запись показывает заполнитель, такой как `<name>`.

Используйте эту страницу, устанавливаете ли вы плагины, создаёте их, размещаете маркетплейс или администрируете плагины для организации.

<Note>
  Эти случаи рассматриваются на других страницах:

  * **Почему области видимости, кэш и приоритет ведут себя так, как они себя ведут**: прочитайте [Справочник по загрузке плагинов](/docs/ru/plugins/loading)
  * **Поиск флага, поля или команды**: используйте [справочник команд плагинов](/docs/ru/plugins/cli-reference), [справочник манифеста](/docs/ru/plugins/manifest-reference) или [справочник маркетплейса](/docs/ru/plugins/marketplace-reference)
</Note>

Найдите точное сообщение, которое вы видели. Каждое сообщение указано под этапом, который его создаёт, что не всегда совпадает с командой, которую вы запустили. Например, установка может завершиться ошибкой, потому что маркетплейс отсутствует, поэтому это сообщение находится в разделе [Добавить маркетплейс](#add-a-marketplace).

<h2 id="find-where-/plugin-runs">
  Найдите, где запускается `/plugin`
</h2>

`/plugin` — это команда, которую вы вводите в работающем сеансе терминала Claude Code, и она открывает интерактивную панель. Записи в этом разделе охватывают места, где вы можете её ввести, но она не может запуститься, и написания команд, которые не существуют.

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

Вы ввели `/plugin` где-то вне сеанса терминала Claude Code, и Claude ответил этой строкой вместо открытия чего-либо.

Вы получаете этот ответ в сеансе, у которого нет терминала для отрисовки панели `/plugin`: [неинтерактивный режим](/docs/ru/headless) с `claude -p`, Agent SDK, вкладка Code в приложении Claude для рабочего стола, панель расширения VS Code и браузер на claude.ai/code.

В панели расширения VS Code только строка `/plugin` с чем-то после неё, такая как `/plugin install <plugin>@<marketplace>`, получает этот ответ. `/plugin` или `/plugins`, введённые отдельно, открывают диалог **Управление плагинами**.

Установите плагин с поверхности, на которой вы находитесь:

* **Приложение Claude для рабочего стола, локальный или SSH-сеанс**: нажмите кнопку **+** рядом с приглашением, затем **Плагины**, затем **Добавить плагин**, чтобы открыть [браузер плагинов](/docs/ru/desktop#install-plugins)
* **Расширение VS Code**: используйте вкладку **VS Code** в разделе [Установить плагин](/docs/ru/plugins/install#install-a-plugin)
* **Claude Code в веб-браузере или облачный сеанс рабочего стола**: облачный сеанс не имеет браузера плагинов. См. вкладку **Cloud session** в разделе [Установить плагин](/docs/ru/plugins/install#install-a-plugin), чтобы узнать, что загружает облачный сеанс
* **Терминал, к которому у вас есть доступ**: запустите `claude` и введите `/plugin` там, или запустите `claude plugin install <plugin>@<marketplace>` в вашей оболочке без запуска сеанса

Когда установка терминала работает, `/plugin` выводит сводку установки, которая начинается с `✓ Installed <plugin>.` и `claude plugin install` выводит `Successfully installed plugin: <plugin>@<marketplace>`.

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

Вы ввели `/plugin ...` в приглашение оболочки, и оболочка сообщила, что файла с именем `/plugin` не существует. Bash сообщает `bash: /plugin: No such file or directory`.

`/plugin` — это команда, которую вы вводите в сеансе Claude Code, а не в приглашение оболочки. Запустите сеанс и введите ту же команду там:

```shell theme={null}
claude
```

Затем в приглашении Claude Code:

```text theme={null}
/plugin install <plugin>@<marketplace>
```

Успешная установка выводит сводку, которая начинается с `✓ Installed <plugin>.` Если сама установка затем не пройдёт, её сообщение находится в разделе [Добавить маркетплейс](#add-a-marketplace) или [Установить плагин](#install-a-plugin).

Чтобы установить из оболочки без запуска сеанса, запустите `claude plugin install <plugin>@<marketplace>`.

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

Вы ввели `/plugin ...` в приглашение PowerShell, и `/plugin` — это команда Claude Code, а не программа. Bash и Zsh сообщают [свою форму этой ошибки](#zsh-no-such-file-or-directory-plugin).

Используйте вместо этого одно из следующих:

* Запустите `claude`, затем введите `/plugin` в приглашение Claude Code
* Запустите `claude plugin install <plugin>@<marketplace>` в PowerShell без запуска сеанса

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` после `claude plugin ...`
</h3>

Вы запустили `claude plugin install ...` в вашей оболочке, и оболочка вообще не смогла найти `claude`. На Windows сообщение: `'claude' is not recognized as the name of a cmdlet` или `'claude' is not recognized as an internal or external command`.

Причина не в команде плагина. Либо Claude Code не установлен, либо его каталог установки не находится в вашем `PATH` в этой оболочке. Следуйте [`command not found: claude` после установки](/docs/ru/troubleshoot-install#command-not-found-claude-after-installation), затем повторите команду плагина.

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` и написания команд, которые не существуют
</h3>

Вы ввели команду плагина, которую видели где-то, и получили `Unknown command: /<name>` в сеансе, или `error: unknown command '<name>'` или `error: unknown option '<flag>'` из двоичного файла `claude` в вашей оболочке.

Используются несколько написаний команд, которые Claude Code не имеет. Таблица ниже сопоставляет каждое с реальной командой. [Справочник команд плагинов](/docs/ru/plugins/cli-reference) перечисляет все подкоманды и флаги.

| Вы ввели                                   | Что говорит Claude Code                                                      | Используйте вместо этого                                                                                                                      |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | `claude plugin marketplace add <source>` для добавления маркетплейса или `claude plugin install <plugin>@<marketplace>` для установки плагина |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                                                |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                                      |
| `/plugin add <source>`                     | Панель `/plugin` открывается на вкладке **Discover**                         | `/plugin marketplace add <source>`                                                                                                            |
| `marketplace.anthropic.com` как источник   | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` для официального маркетплейса                                                                            |

Эти написания выглядят неправильно, но работают:

* `claude plugins` — это псевдоним `claude plugin`
* `claude plugin remove` — это псевдоним `claude plugin uninstall`
* `/plugins` и `/marketplace` в сеансе открывают ту же панель, что и `/plugin`

<h2 id="add-a-marketplace">
  Добавить маркетплейс
</h2>

Маркетплейс — это каталог, который вы добавляете в Claude Code из репозитория git, URL или локального пути. Эти записи охватывают сообщения, которые вы получаете, когда добавление не удаётся или более позднее обновление не удаётся.

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

Вы запустили `/plugin install <plugin>@claude-plugins-official` в сеансе, и Claude Code сообщил, что у него нет маркетплейса с таким именем.

Официальный маркетплейс ещё не зарегистрирован на этой машине. Claude Code обычно регистрирует его самостоятельно при первом запуске интерактивного сеанса терминала. Он ещё не запустился, если вы использовали Claude Code только через расширение VS Code, и он пропускает или откладывает этот шаг:

* Когда политика блокирует источник
* Когда установлена переменная `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`
* После неудачной попытки, которая ждёт повтора

Команды оболочки `claude plugin` никогда не регистрируют его для вас.

Добавьте его, затем повторите установку:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code выводит `Successfully added marketplace: claude-plugins-official`, и `/plugin marketplace list` показывает маркетплейс с его источником.

Для любого другого имени маркетплейса в этом сообщении см. [`Marketplace "<name>" not found`](#marketplace-not-found).

Та же строка также появляется на вкладке **Errors** в `/plugin`, списке сбоев загрузки панели, когда плагин, указанный в ваших параметрах, называет маркетплейс, который вы не добавили.

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

Вы запустили `/plugin install <plugin>@<name>` в сеансе, часто из строки установки, которую кто-то вам отправил, и Claude Code сообщил, что у него нет маркетплейса с таким именем.

Если имя начинается с `claudeai-`, маркетплейс размещён на claude.ai, и вы добавляете его по имени из вашей оболочки с помощью `claude plugin marketplace add --claudeai <name>`. См. [Добавить маркетплейс с claude.ai](/docs/ru/plugins/install#add-from-claude-ai).

Для любого другого имени строка установки называет маркетплейс, но не говорит, где он размещён, и Claude Code не имеет индекса для поиска имени маркетплейса. Попросите у того, кто отправил строку, источник маркетплейса, который является GitHub `owner/repo`, URL git или путём. Затем [добавьте маркетплейс](/docs/ru/plugins/install#add-a-marketplace) и запустите строку установки снова.

Маркетплейс, который кто-то вам отправляет, является сторонним, поэтому [просмотрите плагин перед его установкой](/docs/ru/plugins/security#review-a-plugin-before-you-install).

Если вы уже добавили маркетплейс, проверьте написание в `/plugin marketplace list`.

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

Вы запустили `/plugin marketplace add <source>` или `claude plugin marketplace add <source>`, и Claude Code ответил `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`.

Claude Code принимает источник в одной из этих форм:

* Сокращение GitHub `owner/repo`
* URL `https://` или `http://`
* SSH URL `user@host:path`
* Локальный путь, начинающийся с `./`, `../`, `/` или `~`

Простое имя, такое как `claude-plugins-official`, не соответствует ни одному из них. Также не соответствует простое имя хоста, такое как `marketplace.anthropic.com`.

Переведите источник в одну из принятых форм:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code выводит `Successfully added marketplace: <name>`, когда добавление работает.

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

Вы передали источник с косой чертой, который не является `owner/repo`, такой как `github.com/owner/repo` или путь `gitlab.example.com/group/project`. Claude Code отказал ему с этим сообщением и списком принятых форм.

Сокращение `owner/repo` предназначено только для GitHub и должно следовать правилам именования GitHub, поэтому имя хоста или дополнительный сегмент пути не работает. Передайте источник в форме, которая соответствует тому, где размещён маркетплейс:

* **Репозиторий на любом хосте**: полный URL клонирования
* **Размещённый `marketplace.json`**: его URL `https://`
* **Локальная копия**: `./path` или абсолютный путь

Например, чтобы добавить официальный маркетплейс по его URL клонирования, в сеансе:

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

Успешное добавление выводит `Successfully added marketplace: <name>`.

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

Вы передали локальный путь в `marketplace add`, и ничего не существует по этому пути. Относительный путь разрешается относительно вашего текущего каталога.

Проверьте разрешённый путь в сообщении. Затем запустите команду из каталога, с которого начинается относительный путь, или передайте абсолютный путь к каталогу маркетплейса. Успешное добавление выводит `Successfully added marketplace: <name>`.

Claude Code принимает каталог, содержащий `.claude-plugin/marketplace.json`, или путь к файлу `.json`. Путь к любому другому файлу не работает с `File path must point to a .json file (marketplace.json)`.

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code клонировал или загрузил маркетплейс, но не нашёл `marketplace.json` по ожидаемому пути внутри него. Команда добавления сообщает об этом как `Failed to add marketplace: Marketplace file not found at ...`.

Расположение по умолчанию — `.claude-plugin/marketplace.json` в корне репозитория, и [справочник маркетплейса](/docs/ru/plugins/marketplace-reference) перечисляет принятые расположения.

Исправление отличается для владельца и для всех остальных:

* **Вы владеете маркетплейсом**: поместите файл в это расположение и повторно добавьте маркетплейс
* **Кто-то другой размещает его**: попросите у владельца точный источник, который они публикуют

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` или `HTTPS authentication failed`
</h3>

Вы добавили или обновили маркетплейс из репозитория git, и клонирование не удалось с `Failed to clone marketplace repository:`, за которым следует одна из этих строк.

Сначала проверьте сам репозиторий: неправильный `owner/repo`, репозиторий, который не существует, или приватный репозиторий, который вы не можете видеть, также заканчивается этим сообщением. Откройте URL репозитория в вашем браузере или запустите `git ls-remote <url>` в вашем терминале, чтобы подтвердить, что он существует и у вас есть доступ.

Если репозиторий правильный, причина — учётные данные. Claude Code запускает git с отключёнными интерактивными приглашениями, поэтому он не может попросить вас пароль, парольную фразу ключа или учётные данные так, как это делал бы ваш терминал. Если git нужно приглашение, вы видите `fatal: Cannot prompt because user interactivity has been disabled` или `terminal prompts disabled` в исходной ошибке. Работают только учётные данные, которые уже работают неинтерактивно:

* **SSH**: `ssh -T git@<host>` должен успешно выполниться без запроса парольной фразы, и хост должен уже находиться в `known_hosts`
* **HTTPS**: ваш помощник учётных данных должен содержать токен для хоста. Для GitHub запустите `gh auth login` и `gh auth setup-git`. Для другого хоста сохраните личный токен доступа в помощнике учётных данных git. Протестируйте с помощью `git ls-remote <url>`

Как только `git ls-remote` успешно выполнится в вашем терминале без приглашения, запустите добавление или обновление снова. Успешное добавление выводит `Successfully added marketplace: <name>`. Успешное обновление выводит `Successfully updated marketplace: <name>` из вашей оболочки или `✔ Updated 1 marketplace` в сеансе.

Чтобы Claude Code пропустил SSH для источников GitHub `owner/repo`, установите `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`. Без этого Claude Code клонирует эти источники через SSH, когда SSH-ключ для `github.com` выглядит настроенным, и возвращается к HTTPS, когда клонирование SSH не удаётся.

Для того, что фоновое автообновление может и не может делать с вашими учётными данными, см. [Что фоновое автообновление делает с учётными данными](/docs/ru/plugins/host-marketplace#what-background-auto-update-does-with-credentials).

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

Вы добавили маркетплейс через SSH с хоста, к которому вы никогда не подключались, и клонирование не удалось с этой строкой и подсказкой `ssh -T git@<host>`. Для хоста, чей ключ изменился, сообщение: `SSH host key has changed` с подсказкой `ssh-keygen -R <host>` вместо этого.

Claude Code клонирует с `StrictHostKeyChecking=yes`, поэтому он отказывает хосту, чей ключ вы ещё не приняли, вместо автоматического принятия ключа. Подключитесь один раз из вашего терминала, чтобы принять отпечаток, затем повторите попытку:

```shell theme={null}
ssh -T git@github.com
```

Для публичного репозитория добавьте маркетплейс по его URL `https://` вместо этого, чтобы полностью избежать SSH.

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

На Windows вы добавили маркетплейс, и Claude Code сообщил `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)`.

Claude Code ищет `git` в вашем `PATH` и отказывается запускать найденный только в текущем каталоге. Чтобы исправить это, установите Git и повторите попытку:

<Steps>
  <Step title="Установить Git для Windows">
    Установите Git для Windows, чтобы `git` находился в вашем `PATH`.
  </Step>

  <Step title="Откройте новый терминал">
    Откройте новый терминал, чтобы обновлённый `PATH` применился.
  </Step>

  <Step title="Подтвердите, что git работает">
    Подтвердите, что `git --version` выводит версию.
  </Step>

  <Step title="Повторите добавление">
    Запустите команду `marketplace add` снова.
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

Вы добавили или обновили маркетплейс, и это не удалось с `Git clone timed out after 120s`, за которым следует подсказка установить `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`.

Клонирование маркетплейса и повторное клонирование для его обновления получает 120 секунд по умолчанию. Для большого репозитория или медленного соединения повысьте лимит. Значение указано в миллисекундах:

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

Затем повторите попытку в той же оболочке.

Если репозиторий — это монорепо, ограничьте проверку каталогами, которые вы называете с помощью `claude plugin marketplace add <source> --sparse <paths>`.

<h3 id="marketplace-updates-keep-failing-offline">
  Обновления маркетплейса продолжают не работать в автономном режиме
</h3>

Вы работаете в среде, где хост git маркетплейса недоступен, и каждый сеанс повторяет неудачное обновление в фоне. Ваша существующая копия маркетплейса остаётся на месте, и запуск не задерживается.

Каждый сеанс для маркетплейса с [включённым автообновлением](/docs/ru/plugins/loading#which-marketplaces-and-plugins-auto-update) Claude Code проверяет хост git маркетплейса на предмет новых коммитов в фоне. Когда эта проверка не может достичь хоста, она пытается клонировать маркетплейс снова, и в автономном режиме это клонирование также не удаётся.

Установите эту переменную, чтобы пропустить попытку повторного клонирования и продолжить использовать существующую копию, когда проверка не может достичь хоста:

<Tabs>
  <Tab title="Bash или Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

С установленной переменной Claude Code пропускает повторное клонирование только для копии, которая уже содержит `.claude-plugin/marketplace.json`. Маркетплейс, который никогда не был клонирован или чьё клонирование остановилось на полпути, всё ещё получает попытку клонирования, поэтому добавьте его один раз в сети.

Для полностью автономного развёртывания вместо этого предварительно заполните каталог плагинов во время сборки образа с помощью `CLAUDE_CODE_PLUGIN_SEED_DIR`, следуя [Seed контейнеры и CI](/docs/ru/plugins/org#seed-containers-and-ci).

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  Добавление маркетплейса не работает на хосте GitHub Enterprise Server
</h3>

Вы добавили маркетплейс с URL GitHub Enterprise Server (GHES) и получили ошибку политики, или вы добавили его с claude.ai и получили ошибку доступа GitHub.

Оба случая находятся на странице GHES:

* [Ошибка политики](/docs/ru/github-enterprise-server#marketplace-add-fails-with-a-policy-error) означает, что ваша организация ограничила источники маркетплейса, и администратор должен добавить `hostPattern` для хоста
* [Ошибка доступа GitHub на claude.ai](/docs/ru/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error) означает, что ваша собственная учётная запись GitHub Enterprise ещё не подключена

<h2 id="install-a-plugin">
  Установить плагин
</h2>

Вы добавили маркетплейс и запустили установку, и установка остановилась с сообщением вместо установки чего-либо. Эти записи охватывают эти сообщения. Они также охватывают связанные сообщения, которые появляются позже на вкладке **Errors** в `/plugin` или как пустая вкладка **Discover**, когда плагин или его маркетплейс не могут быть найдены, прочитаны или доверены.

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

Вы запустили `/plugin install <name>@<marketplace>` или `claude plugin install <name>@<marketplace>`, и имя плагина отсутствует в копии каталога этого маркетплейса на вашей машине.

`claude plugin install` в вашей оболочке выводит то же сообщение, когда вы вообще не добавили маркетплейс. Если `claude plugin marketplace update <marketplace>` затем ответит `Marketplace '<marketplace>' not found`, сначала [добавьте маркетплейс](#add-a-marketplace).

<h4 id="the-message-ends-with-a-refresh-hint">
  `not found in marketplace` с подсказкой обновления
</h4>

Подсказка гласит `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` или `The marketplace couldn't be refreshed (...)`. Claude Code не обновил маркетплейс перед поиском, например, когда вы в автономном режиме, поэтому ваша копия каталога может быть устаревшей. Обновите с именем маркетплейса, затем установите снова:

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` выводит `Successfully updated marketplace: <name>`, и `/plugin marketplace update` показывает `✔ Updated 1 marketplace`. Если повторная установка выводит то же сообщение, проверьте имя, как описано в [`not found in marketplace` без подсказки](#the-message-has-no-hint). [Когда Claude Code обновляет маркетплейс перед установкой](/docs/ru/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install) перечисляет другие случаи, когда обновление не запускается.

<h4 id="the-message-has-no-hint">
  `not found in marketplace` без подсказки
</h4>

Имя — наиболее вероятная проблема. Откройте `/plugin`, перейдите на **Discover** и скопируйте имя из списка.

До v2.1.232 Claude Code обновлял названный маркетплейс только после того, как поиск не удавался, и только когда для него было включено автообновление.

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

Вы запустили `/plugin install <name>` без `@marketplace`, и ни один зарегистрированный маркетплейс не имеет этого плагина. `claude plugin install <name>` сообщает `Plugin "<name>" not found in any configured marketplace`.

Без имени маркетплейса `claude plugin install` ищет каталоги, которые он уже имеет, и не обновляет их в первую очередь, и `/plugin install` обновляет только маркетплейсы, у которых включено автообновление. Назовите маркетплейс, и Claude Code обновит его перед поиском плагина:

```text theme={null}
/plugin install <name>@<marketplace>
```

Когда установка работает, вы видите `✓ Installed <plugin>.` в сеансе или `Successfully installed plugin: <plugin>@<marketplace>` из `claude plugin install`.

Если вы не знаете, какой маркетплейс содержит плагин, запустите `/plugin marketplace list` для маркетплейсов, которые у вас есть, и просмотрите **Discover** в `/plugin` для имени плагина.

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

Вы запустили `/plugin install` для плагина, который уже установлен в области пользователя или управляемыми параметрами, и Claude Code отказал с `Use '/plugin' to manage existing plugins.` Если вы ввели имя плагина без `@<marketplace>`, сообщение опускает `globally`.

Плагин уже доступен в каждом проекте, поэтому нечего добавлять. Чтобы изменить его [область видимости](/docs/ru/plugins/install), включить или отключить его или настроить его, откройте `/plugin` и перейдите на **Installed**.

Плагин, установленный только в области проекта или локальной области, не вызывает это сообщение. Claude Code позволяет вам установить его также в области пользователя, поэтому он доступен в других проектах.

`claude plugin install` в вашей оболочке выводит другое сообщение. Для плагина, уже установленного в целевой области, он выводит `Plugin "<name>@<marketplace>" is already installed (scope: user)` и выходит с кодом 0. Если его каталог кэша отсутствует, та же команда повторно загружает его.

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

Вы установили плагин, чья запись маркетплейса использует тип источника, который эта версия Claude Code не может получить, и Claude Code остановился с этим сообщением и `Update Claude Code and try again.`

Обновите Claude Code, затем повторите установку. Типы источников находятся в [справочнике маркетплейса](/docs/ru/plugins/marketplace-reference).

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

Вы установили плагин, который распространяется как zip-архив, и Claude Code отказал ему с этой строкой и `The archive was not installed.` Запись маркетплейса плагина использует [`archive` источник](/docs/ru/plugins/marketplace-reference) с закреплением `sha256`, и дайджест загруженного файла не совпадает с закреплением.

Полное сообщение выглядит так:

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

Исправление отличается для издателя и установщика:

* **Вы публикуете плагин**: пересчитайте дайджест точного файла, который служит URL, и обновите `sha256` в записи маркетплейса. Используйте `shasum -a 256 my-plugin.zip` или `Get-FileHash -Algorithm SHA256 my-plugin.zip` в PowerShell
* **Вы устанавливаете плагин**: запустите `/plugin marketplace update <name>` в сеансе, чтобы обновить каталог на случай, если запись была исправлена, затем повторите установку. Если дайджесты всё ещё не совпадают после обновления, попросите владельца маркетплейса, какой файл они закрепили перед установкой

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

Маркетплейс, который вы добавили ранее, перестал загружаться, и его плагины тоже. Эта строка появляется на вкладке **Errors** в `/plugin` или при следующем обновлении.

Маркетплейс зарегистрирован под именем, которое [зарезервировано для официальных маркетплейсов Anthropic](/docs/ru/plugins/marketplace-reference), но его зарегистрированный источник не является репозиторием GitHub `anthropics`. Зарезервированные имена повторно проверяются каждый раз, когда маркетплейс загружается или обновляется, поэтому маркетплейс и плагины, установленные из него, перестают загружаться.

Полное сообщение называет зарезервированное имя и исправление:

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Исправление отличается для пользователей и издателей:

* **Вы используете маркетплейс**: в вашей оболочке запустите `claude plugin marketplace remove <name>`, затем добавьте маркетплейс снова из официального репозитория `github.com/anthropics`
* **Вы публикуете сторонний маркетплейс, который использовал имя до того, как оно было зарезервировано**: переименуйте его и попросите пользователей повторно добавить его из вашего источника

До v2.1.205 Claude Code проверял имя только при добавлении маркетплейса, поэтому запись, зарегистрированная до того, как её имя было зарезервировано, продолжала загружаться.

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` или `has an invalid manifest file`
</h3>

Claude Code получил плагин, затем не смог прочитать его `.claude-plugin/plugin.json`. В оболочке `<name>` в этой строке может быть временным именем каталога; префикс `Failed to install plugin "<name>@<marketplace>"` содержит реальное имя плагина. Формулировка говорит, какая проверка не удалась:

* **`corrupt manifest file`, за которым следует `JSON parse error:`**: файл не является действительным JSON
* **`invalid manifest file`, за которым следует `Validation errors:`**: файл анализируется, но не проходит схему, такую как `name: Invalid input` для отсутствующего обязательного поля

`claude plugin install` сообщает либо как `Failed to install plugin "<name>@<marketplace>":` и выходит с кодом 1.

Автор плагина должен исправить файл, и плагин не может быть установлен до этого:

* **Если это вы**: запустите `claude plugin validate <plugin-directory>` в вашей оболочке, чтобы увидеть ту же ошибку с нарушающим путём, затем исправьте файл
* **Если это не вы**: сообщите сообщение владельцу маркетплейса

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

Вкладка **Errors** в `/plugin` показывает это для включённого плагина, который его маркетплейс указывает относительным путём, такой как `./plugins/my-plugin`, когда в этом пути внутри маркетплейса не существует каталога. Если вы поддерживаете маркетплейс, исправьте путь `source` записи или восстановите папку. В противном случае сообщите сообщение владельцу маркетплейса.

`Marketplace directory not found at path: <path>` означает, что вместо этого отсутствует собственный каталог маркетплейса. Для маркетплейса, который вы добавили из локального пути, этот каталог переместился или был удалён. Восстановите его или удалите маркетплейс и добавьте его снова из его нового расположения.

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` или `No marketplaces configured`
</h3>

Вы открыли `/plugin` и вкладка **Discover** пуста, или `claude plugin marketplace list` вывел `No marketplaces configured`.

Ни один маркетплейс не зарегистрирован, поэтому нет каталога для отображения. В сеансе добавьте официальный маркетплейс, `anthropics/claude-plugins-official`:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code выводит `Successfully added marketplace: claude-plugins-official`, и **Discover** перечисляет его плагины. На странице [Маркетплейсы Anthropic](/docs/ru/plugins/anthropic-marketplaces) перечислены другие маркетплейсы, которые вы можете добавить.

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

Вы подтвердили добавление маркетплейса через [`/plugin install <plugin> --marketplace <source>`](/docs/ru/plugins/install#add-a-marketplace-and-install-in-one-command), и каталог, который Claude Code получил из этого источника, имеет то же имя, что и маркетплейс, который вы уже добавили из другого источника. Claude Code сохраняет существующий маркетплейс вместо его замены, и плагин не устанавливается.

Полное сообщение выглядит так:

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

Выберите, какой источник вы хотите:

* **Маркетплейс, который вы уже добавили**: установите из него по имени с помощью `/plugin install <plugin>@<name>`
* **Новый источник**: запустите `/plugin marketplace remove <name>`, затем повторите установку

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

Вы запустили `marketplace add`, и каталог в этом источнике имеет то же имя, что и маркетплейс, который файл параметров уже объявляет в [`extraKnownMarketplaces`](/docs/ru/settings-reference#extraknownmarketplaces) с другим источником. Claude Code отказывает добавлению и ничего не регистрирует.

Сообщение заканчивается исправлением: источник должен совпадать с тем, который объявлен для этого имени в параметрах, или вы изменяете объявление. Сравните источник, который вы передали, с записью `extraKnownMarketplaces` для этого имени, включая его `ref`, `path` и `headers`, затем сделайте одно из следующего:

* **Используйте объявленный источник**: добавьте маркетплейс из источника, который называет запись параметров
* **Используйте новый источник**: отредактируйте или удалите запись `extraKnownMarketplaces`, затем добавьте маркетплейс снова. Если управляемые параметры объявляют это, попросите вашего администратора

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

Вы выбрали плагины для установки в меню `/plugin`, ни один из них не установился, и меню закрылось с этой сводкой того, что не удалось.

Некоторые причины, такие как вывод git после неудачного клонирования, показывают только их первую строку. Когда такая причина была сокращена, сводка заканчивается `Installing a plugin from its details (Enter) in /plugin shows its full error.`

Что делать, зависит от того, была ли сокращена сводка причины:

* Исправьте то, что называет причина в скобках
* Когда причина была сокращена, запустите `/plugin`, выберите плагин на вкладке **Discover** и нажмите **Enter**, чтобы установить его из его деталей. Если установка там не удаётся, представление деталей показывает всю ошибку

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

Когда вы устанавливаете плагин, Claude Code загружает свежую копию его файлов и перемещает её в папку этой версии в [кэше плагинов](/docs/ru/plugins/loading#find-plugins-on-disk). Это сообщение означает, что перемещение не удалось, обычно потому, что другая программа использовала папку во время установки. Код файловой системы появляется в скобках:

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

Сообщение говорит, что произошло с копией, которая была установлена раньше, что говорит вам, работает ли плагин:

* `The previously installed copy was moved back`: версия, которая у вас была, всё ещё установлена
* `had to be removed first`, `was not moved back` или `could not be moved back`: эта версия плагина не установлена до успешной установки
* Нет такого предложения: не было более ранней копии, поэтому версия ещё не установлена

На Windows, когда другая программа удерживает саму установленную копию, сообщение вместо этого говорит, что эта копия `could not be replaced` и что `It was not replaced and the new copy was discarded`, поэтому версия, которая у вас была, всё ещё установлена.

Список `Left on disk` называет отложенные папки внутри кэша. Более позднее обновление этой версии или очистка кэша плагинов удаляет их, поэтому вам не нужно их удалять.

Чтобы исправить установку:

* Закройте другие сеансы Claude Code, редакторы и терминалы, которые используют папку плагина в `~/.claude/plugins/cache`, затем запустите установку снова
* Когда сообщение говорит проверить разрешения папки кэша плагинов, восстановите разрешение на запись в папку, которую оно называет, и освободите место на диске, затем запустите установку снова

<h3 id="dependency-errors">
  Ошибки зависимостей
</h3>

Плагин, который объявляет зависимости, может не установиться или установиться и остаться отключённым, когда зависимость не может быть удовлетворена. Сообщение достигает вас во время установки или во время загрузки:

* **Во время установки**: отказ возвращается как сообщение об ошибке установки
* **Когда плагин загружается**: проблема появляется в `claude plugin list` и на вкладке **Errors** в `/plugin`, и Claude Code держит затронутый плагин отключённым до тех пор, пока вы не разрешите это

Таблица перечисляет каждое сообщение и его исправление. Чтобы объявить зависимости как автор, см. [Зависимости плагинов](/docs/ru/plugins/dependencies).

| Сообщение                                                                                        | Значение                                                                                                   | Как разрешить                                                                                                                                                                                                                                                                                         |
| :----------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Dependency "<dep>" is not installed`                                                            | Объявленная зависимость не установлена.                                                                    | Установите её в вашей оболочке с помощью `claude plugin install <dep>@<marketplace>` или удалите плагин. Если маркетплейс зависимости ещё не зарегистрирован, добавьте его и запустите `/reload-plugins` в вашем сеансе, который устанавливает отсутствующие зависимости, которые он может разрешить. |
| `Dependency "<dep>" is disabled`                                                                 | Зависимость установлена, но отключена.                                                                     | Включите зависимость или удалите плагин, который её требует.                                                                                                                                                                                                                                          |
| `Requires "<dep>" <range>, installed <version>`                                                  | Версия установленной зависимости находится вне объявленного диапазона плагина.                             | Обновите зависимость до версии в диапазоне или удалите плагин.                                                                                                                                                                                                                                        |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                           | Ни одна версия не удовлетворяет каждому диапазону, который её закрепляет. Сообщение перечисляет диапазоны. | Удалите или обновите один из конфликтующих плагинов или попросите автора вышестоящего уровня расширить его ограничение.                                                                                                                                                                               |
| `... has version requirements too complex to intersect` или `has an invalid version requirement` | Диапазон не является действительным semver или объединённые диапазоны не могут быть пересечены.            | Исправьте недействительный диапазон или упростите длинные цепочки `\|\|`.                                                                                                                                                                                                                             |
| `... has no git tag satisfying <range>`                                                          | Репозиторий зависимости не имеет тега `<name>--v*` в диапазоне.                                            | Проверьте, что вышестоящий уровень помечает выпуски этим соглашением, или ослабьте диапазон.                                                                                                                                                                                                          |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist`   | Зависимость находится в другом маркетплейсе, и разрешение между маркетплейсами отключено по умолчанию.     | Установите зависимость самостоятельно в той же области видимости, в вашей оболочке с помощью `claude plugin install <dep>@<marketplace>` плюс `--scope`, в котором вы устанавливаете плагин, затем повторите попытку.                                                                                 |

Чтобы увидеть это программно, запустите `claude plugin list --json` в вашей оболочке. Плагины с проблемами содержат поле `errors` с сообщениями и поле `errorDetails` с `type` для каждого: первые две строки — `dependency-unsatisfied`, а третья — `dependency-version-unsatisfied`.

<h2 id="plugin-installed-but-not-working">
  Плагин установлен, но не работает
</h2>

Установка прошла успешно, но skills, hooks или серверы плагина ничего не делают. Начните с [Плагин не появляется или его skills не отображаются](#plugin-doesnt-appear-or-its-skills-dont-show-up), что говорит вам, где Claude Code сообщает, что он загрузил, затем сопоставьте сообщение.

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  Плагин не появляется или его skills не отображаются
</h3>

Вы установили плагин и ввели `/`, ожидая его skills, или попросили Claude использовать его, и ничего не произошло.

Проверьте состояние плагина перед изменением чего-либо:

<Steps>
  <Step title="Подтвердите, что плагин установлен и включён">
    Запустите `/plugin` и откройте **Installed**. Подтвердите, что плагин указан и включён. `claude plugin list` в вашей оболочке выводит тот же список с версией каждого плагина, областью видимости и `Status: ✔ enabled`.
  </Step>

  <Step title="Прочитайте вкладку Errors">
    Откройте вкладку **Errors** в той же панели. Каждая запись связывает сообщение с строкой руководства. Большинство сообщений в остальной части этого раздела поступают с этой вкладки.
  </Step>

  <Step title="Перезагрузитесь, если вы установили во время этого сеанса">
    Если плагин установлен и без ошибок, но вы установили его во время этого сеанса, запустите `/reload-plugins`. Он выводит `Reloaded:` с подсчётом плагинов, skills, агентов, hooks и серверов. Когда что-то не удалось, он добавляет `N errors during load. Run /plugin for details.`
  </Step>
</Steps>

Если плагин загружается без ошибок и его skills всё ещё не появляются, следующий шаг отличается для вашего собственного плагина и для чужого:

* **Плагин, который вы создаёте**: см. [Плагин загружается, но его skills отсутствуют](#plugin-loads-but-its-skills-are-missing)
* **Плагин, который кто-то другой опубликовал**: откройте **Installed** в `/plugin` и откройте панель деталей плагина, которая перечисляет, что содержит плагин. Плагин, который не перечисляет skills там, не имеет их для предложения, когда вы вводите `/`

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

Сводка установки в `/plugin` закончилась с `Run /reload-plugins to activate.` вместо `Plugin is now active.`

Claude Code не активировал плагин во время установки, либо потому, что активация его [инвалидирует кэш подсказок](/docs/ru/prompt-caching#enabling-or-disabling-a-plugin), либо потому, что попытка активации не удалась.

Вам не нужно вводить команду. Панель закрывается и Claude Code запускает `/reload-plugins` для вас, или ставит её в очередь до завершения потоковой передачи ответа.

Прочитайте, что выводит эта перезагрузка:

* **`Reloaded:` с подсчётом плагинов, skills, агентов, hooks и серверов**: плагин теперь активен. Когда что-то не удалось загрузить, строка добавляет `N errors during load. Run /plugin for details.`
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**: перезагрузка добавит или удалит сервер MCP плагина или инструмент `LSP`, и инвалидирует ваш кэш подсказок. Для случая LSP строка начинается с `This reload adds the LSP tool` или `This reload removes the LSP tool`. Запустите его с `--force`, чтобы активировать плагин в любом случае, или запустите новый сеанс

До v2.1.268 установка, которая не активировалась во время установки, оставалась в ожидании до тех пор, пока вы не запустили `/reload-plugins` самостоятельно.

До v2.1.246 подсчёт skills в этой сводке включал только записи `commands/` плагина, поэтому перезагрузка могла загрузить skills `SKILL.md` плагина и всё ещё сообщать `0 skills`.

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

Вкладка **Errors** показывает эту строку с руководством `Run /plugin to refresh the plugin cache`. Claude Code имеет запись об установке плагина, но каталог, на который указывает запись, отсутствует, например, после очистки кэша.

Переустановите плагин из вашей оболочки. `claude plugin install <name>@<marketplace>` повторно загружает плагин, чей каталог установки отсутствует, даже если его запись существует:

```shell theme={null}
claude plugin install <name>@<marketplace>
```

Затем запустите `/reload-plugins` в вашем сеансе. Запись на вкладке **Errors** исчезает и плагин вернулся в **Installed**.

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

Вы установили плагин на `false` в `~/.claude/settings.json`, и его строка в `claude plugin list` или `/plugin` показывает это сообщение, за которым следует источник, который его включает, такой как `— project settings enable it, which overrides your user setting`. `true` в этом источнике с более высоким приоритетом переопределяет ваш пользовательский параметр.

Чтобы отказаться от плагина, включённого проектом, на вашей машине установите id на `false` в `.claude/settings.local.json`, который имеет более высокий приоритет, чем файл проекта. Для других источников, которые может назвать сообщение, см. [Отключено в пользовательских параметрах, но всё ещё загружается](/docs/ru/plugins/loading#disabled-in-user-settings-but-still-loads).

Если `claude plugin list` вместо этого помечает плагин `required by your org`, никакой файл параметров не задействован: ваша организация помечает этот синхронизированный плагин как требуемый на claude.ai, и он загружается, даже если вы отключили его ранее. См. [Плагины, синхронизированные с claude.ai](/docs/ru/plugins/loading#synced-plugins).

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

Вкладка **Errors** показывает эту строку для плагина, который `.claude/settings.json` вашего проекта включает, с руководством `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`.

Параметры репозитория могут включать плагин для всех, кто его открывает, но они его не устанавливают. Когда плагин поступает из внешнего источника, такого как репозиторий GitHub или пакет npm, Claude Code не загружает его до тех пор, пока вы не установите его самостоятельно. Запустите команду из строки руководства в вашей оболочке, затем перезагрузитесь:

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

После запуска `/reload-plugins` в вашем сеансе запись на вкладке **Errors** исчезает и плагин указан в **Installed**.

Если ваша организация предварительно устанавливает плагины для вас, она делает это через управляемые параметры вместо этого. См. [Предварительная установка и требование плагинов](/docs/ru/plugins/org#pre-install-and-require-plugins).

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` и hooks, которые не срабатывают
</h3>

Hooks плагина не запускаются. Либо вкладка **Errors** показывает ошибку загрузки для них, hooks загружаются и вы видите уведомления `<Event> hook error` в стенограмме, либо hook загружается без ошибки и никогда не срабатывает.

<h4 id="hooks-fail-to-load">
  Hooks не загружаются
</h4>

Вкладка **Errors** показывает одно из этих сообщений:

* **`Failed to load hooks from <path>: <reason>`**: `hooks/hooks.json` не является действительным JSON или не проходит схему hooks. Причина называет ошибку анализа или проверки. Исправьте файл. Чтобы поймать проблему синтаксиса JSON в `hooks/hooks.json` перед публикацией плагина, запустите `claude plugin validate <plugin-directory>` в вашей оболочке
* **`hooks path not found: <path>`**: поле `hooks` манифеста называет файл, который не существует по этому пути относительно корня плагина. Исправьте путь или добавьте файл

<h4 id="hook-error-notices-in-the-transcript">
  Уведомления `hook error` в стенограмме
</h4>

Уведомление формы `... hook error: Failed with non-blocking status code: <stderr>` означает, что hook запустился и его команда не удалась. Например, `Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` означает, что оболочка, которую Claude Code создал, не смогла найти `node`. Установите его или убедитесь, что он находится в `PATH` терминала, из которого вы запускаете `claude`.

Для любой другой ошибки запустите команду hook самостоятельно из каталога плагина, чтобы увидеть полный вывод, или захватите полный stderr с помощью [отладочного логирования](/docs/ru/hooks#debug-hooks).

<h4 id="hook-loads-but-never-fires">
  Hook загружается, но никогда не срабатывает
</h4>

Если hook загружается без ошибки, но никогда не срабатывает, проверьте его определение, а затем посмотрите, как он запускается:

<Steps>
  <Step title="Проверьте имя события">
    Имена событий чувствительны к регистру, поэтому подтвердите, что ваше совпадает точно, например `PostToolUse`.
  </Step>

  <Step title="Проверьте matcher">
    Подтвердите, что `matcher` hook совпадает с именем инструмента.
  </Step>

  <Step title="Запустите событие специально">
    Для hook `PostToolUse` попросите Claude отредактировать файл.
  </Step>

  <Step title="Прочитайте отладочный журнал">
    Откройте [отладочный журнал](/docs/ru/hooks#debug-hooks), который записывает, какие hooks совпадали. Hook, который запустился, появляется там с его кодом выхода.
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` и MCP серверы, которые не запускаются
</h3>

Плагин содержит MCP сервер, и вкладка **Errors** показывает `Invalid MCP server config for "<server>": <error>`, или сервер указан, но `/mcp` никогда не показывает его подключённым.

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

Конфигурация сервера проходит проверку схемы, но Claude Code не может разрешить её для этого сеанса. Текст после двоеточия называет причину и решает исправление:

* **`Missing environment variables: <names>`**: установите эти переменные в оболочке, из которой вы запускаете Claude Code, затем запустите новый сеанс
* **`URL is unset or invalid`**: опция `${user_config.*}`, которую использует URL, не установлена. Запустите `/plugin configure <plugin>`, чтобы установить её
* **`has an invalid MCP url`** или **`headersHelper for MCP server '<server>' references ${user_config.*}`**: конфигурация самого плагина виновата. Исправьте `url` или `headersHelper` в конфигурации MCP вашего плагина или сообщите об этом автору плагина, если плагин не ваш. Случай `headersHelper` имеет свою собственную запись в разделе [команда плагина ссылается на user\_config](/docs/ru/errors#plugin-command-references-user-config)

<h4 id="server-is-configured-but-never-connects">
  Сервер настроен, но никогда не подключается
</h4>

Запустите `/mcp`, чтобы увидеть статус сервера. Когда сервер здоров, `/mcp` перечисляет его как подключённый.

Чтобы прочитать ошибку, которую сервер вывел при запуске, запустите `claude --debug` и откройте журнал в `~/.claude/debug/<session-id>.txt`. Флаг `--debug` не выводит на терминал.

Запись сервера в `.mcp.json`, которая не проходит схему, не появляется на вкладке **Errors**. Claude Code удаляет этот сервер и записывает `Invalid MCP server config for <server> in <path>` только в этот отладочный журнал. Чтобы найти запись без загрузки плагина, запустите `claude plugin validate` в вашей оболочке в каталоге плагина, который сообщает об этом как об ошибке.

До v2.1.281 `claude plugin validate` не проверял `.mcp.json`.

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  Сервер работает с `--plugin-dir`, но не работает после установки
</h4>

Вы автор плагина, и сервер запускается, когда вы загружаете плагин из его исходного каталога с `--plugin-dir`, но не работает после установки плагина.

Claude Code копирует установленный плагин в его кэш, поэтому путь, который работает только из исходного каталога, ломается. Напишите пути внутри плагина с помощью `${CLAUDE_PLUGIN_ROOT}`.

Для путей, которые достигают вне каталога плагина, см. [Файлы, на которые ссылается плагин вне его каталога, не найдены](#files-the-plugin-references-outside-its-directory-arent-found).

<h3 id="language-server-doesnt-start">
  Языковой сервер не запускается, использует слишком много памяти или сообщает неправильные диагностики
</h3>

Вы установили [плагин интеллекта кода](/docs/ru/plugins/code-intelligence) и Claude не видит диагностики, или языковой сервер использует слишком много памяти или сообщает об ошибках, которые не являются реальными.

<h4 id="language-server-doesn’t-start">
  Языковой сервер не запускается
</h4>

Плагин подключается к двоичному файлу языкового сервера, который вы устанавливаете отдельно, и Claude Code создаёт его по имени команды из вашего `PATH`.

Вкладка **Errors** в `/plugin` показывает ошибку с её причиной, такой как `Executable not found in $PATH: "<binary>"`, и `claude --debug` логирует её как `LSP server <name> failed to start: <reason>`.

Установите двоичный файл и подтвердите, что он находится в `PATH` терминала, из которого вы запускаете `claude`, например, с помощью `which typescript-language-server`. Затем запустите новый сеанс.

<h4 id="language-server-uses-too-much-memory">
  Языковой сервер использует слишком много памяти
</h4>

Языковые серверы, такие как `rust-analyzer` и `pyright`, индексируют весь проект. Отключите плагин с помощью `/plugin disable <plugin>` в сеансе и вместо этого полагайтесь на встроенные инструменты поиска Claude.

<h4 id="false-positive-diagnostics-in-a-monorepo">
  Ложные положительные диагностики в монорепо
</h4>

Языковой сервер, который не настроен для рабочей области, может сообщать о неразрешённых импортах для внутренних пакетов. На стороне Claude Code нечего исправлять, и диагностики не мешают Claude редактировать код.

<h2 id="build-a-plugin">
  Создать плагин
</h2>

Вы разрабатываете плагин и загружаете его с помощью `--plugin-dir` или устанавливаете из локального маркетплейса. Эти записи охватывают ошибки, которые вы получаете при разработке плагина. Для проверок, которые запускаются после каждого изменения, см. [Тестирование и отладка](/docs/ru/plugins/create#test-and-debug).

Две ошибки, которые также достигают пользователей плагина, имеют свои записи в разделе [Плагин установлен, но не работает](#plugin-installed-but-not-working):

* **Hook, который не срабатывает**: см. [hooks, которые не срабатывают](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **MCP сервер, который не запускается**: см. [MCP серверы, которые не запускаются](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

Вкладка **Errors** показывает `commands path not found: <absolute path>` с руководством `Check that the path in your manifest or marketplace config is correct`. То же сообщение появляется для `skills`, `agents` и `hooks`.

Claude Code разрешил путь из вашего `plugin.json` или записи маркетплейса относительно корня плагина и ничего там не нашёл. Путь в сообщении — это абсолютный путь, который он проверил, поэтому сравните его с тем, что находится на диске. Исправьте путь или создайте каталог, затем запустите `/reload-plugins`.

Пути в манифесте относительны корню плагина и начинаются с `./`. Путь, который разрешается вне корня плагина, сообщается как `<component> path escapes plugin directory` вместо этого и удаляется.

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` в корне маркетплейса не загружает плагины в `plugins/`
</h3>

Вы запустили `claude --plugin-dir <path>` и не видите ошибку, но skills, агенты и hooks плагина отсутствуют.

`--plugin-dir` принимает каталог корня плагина, тот, который содержит `.claude-plugin/plugin.json` и каталоги компонентов, такие как `skills/`. Если вы вместо этого указываете его на корень маркетплейса, Claude Code не читает `marketplace.json`, поэтому плагин в `plugins/` не загружается, и вы не видите ошибку. До v2.1.281 Claude Code загружал корень маркетплейса как один пустой плагин с именем этого каталога. Укажите флаг на сам каталог плагина:

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

Затем откройте **Installed** в `/plugin`, где панель деталей плагина перечисляет его компоненты.

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  Файлы, на которые ссылается плагин вне его каталога, не найдены
</h3>

Плагин работает из его исходного каталога с `--plugin-dir`, но не работает после установки, с ошибками о пути, такой как `../shared-utils`.

Claude Code копирует установленный плагин в его кэш и загружает его оттуда, поэтому путь, который достигает вне собственного каталога плагина, указывает на ничто в кэше. Переместите общие файлы внутри каталога плагина или ссылайтесь на них через символическую ссылку внутри него. Для того, где находится кэш и как разрешаются пути, см. [Найдите плагины на диске](/docs/ru/plugins/loading#find-plugins-on-disk).

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` показывает прямые косые черты на Windows
</h3>

На Windows hook плагина получает `${CLAUDE_PLUGIN_ROOT}` как `C:/Users/you/...` вместо `C:\Users\you\...`, и скрипт, который ожидал обратных косых черт, ломается.

Claude Code запускает hooks в форме оболочки через Git Bash на Windows и подставляет корень плагина в форме Win32 с прямыми косыми чертами специально. Встроенные функции Bash, инструменты MSYS и собственные двоичные файлы Windows все принимают эту форму.

Если вашему скрипту нужны обратные косые черты, переключитесь на одну из форм, которые сохраняют собственные пути, описанные в разделе [exec форма и shell форма](/docs/ru/hooks#exec-form-and-shell-form):

* Hook в exec-форме, который создаёт процесс непосредственно с массивом `args`
* Hook с `"shell": "powershell"`

<h3 id="plugin-loads-but-its-skills-are-missing">
  Плагин загружается, но его skills отсутствуют
</h3>

Ваш плагин указан в **Installed** без ошибок, но его skills не предлагаются, когда вы вводите `/`.

Skills загружаются из `skills/` в корне плагина и команды из `commands/` в корне плагина. Только `plugin.json` принадлежит внутри `.claude-plugin/`, и каталог `skills/` внутри `.claude-plugin/` не сканируется. Переместите каталоги в корень плагина и запустите `/reload-plugins`. После этого панель деталей плагина в `/plugin` перечисляет skills, и ввод `/` предлагает их.

Каждый skill — это каталог, содержащий `SKILL.md`. Запись `skills` в манифесте, которая указывает на файл `SKILL.md` вместо его каталога, сообщается как `path is a file; skills entries must be directories containing SKILL.md`.

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  Skill загружается, но Claude никогда не вызывает skill
</h3>

Skill вашего плагина запускается, когда вы вводите его команду `/<plugin>:<skill>`, но Claude никогда не вызывает его в ответ на простой запрос.

Проверьте эти причины по порядку:

* **Skill устанавливает `disable-model-invocation: true`**: с этим полем установленным только вы можете вызвать skill. Шаблонный skill в [Создайте свой первый плагин](/docs/ru/plugins/create#create-your-first-plugin) устанавливает его. Удалите строку из skill, который вы хотите, чтобы Claude вызвал самостоятельно. [Контролируйте, кто вызывает skill](/docs/ru/skills#control-who-invokes-a-skill) охватывает поле
* **Описание не совпадает с тем, как люди спрашивают**: пройдите проверки в [Skill не срабатывает](/docs/ru/skills#skill-not-triggering)
* **Описание обрезано**: когда установлено много skills, Claude Code сокращает описания, чтобы они поместились в бюджет символов списка, что может удалить ключевые слова, которые Claude нужны для сопоставления запроса. См. [Описания skills обрезаны](/docs/ru/skills#skill-descriptions-are-cut-short)

Чтобы измерить, как часто skill срабатывает на реалистичных подсказках, а не проверять по одной, напишите случай eval с [оценщиком `tool_used: Skill`](/docs/ru/plugin-evals#create-your-first-eval-suite) и запустите его с `claude plugin eval` после каждого изменения описания.

<h3 id="is-not-a-plugin-or-skill-folder">
  `<directory> is not a plugin or skill folder` из `claude plugin eval init`
</h3>

Вы запустили `claude plugin eval init` из каталога, который не является корнем плагина, такой как ваш домашний каталог или корень репозитория, который хранит плагин в подкаталоге. `init` пишет набор в рабочий каталог, поэтому он останавливается вместо создания каталога `evals/`, который плагин никогда не увидит.

Перейдите в корень плагина, каталог, который содержит `.claude-plugin/plugin.json` или `SKILL.md` skill, и запустите команду снова. Чтобы создать набор где-то ещё специально, передайте `--eval-dir`. См. [Тестируйте плагины с помощью evals](/docs/ru/plugin-evals).

<h3 id="the-userconfig-dialog-never-appears">
  Диалог `userConfig` никогда не появляется
</h3>

Ваш плагин объявляет опции `userConfig`, но диалог конфигурации не появляется при его установке.

Интерактивная установка показывает диалог, и команда оболочки принимает значения как флаги вместо этого:

* **`/plugin install` в сеансе или вкладка Discover в `/plugin`**: диалог является частью этой интерактивной установки
* **`claude plugin install` в вашей оболочке**: никогда не запрашивает значения `userConfig`. Он сохраняет любые значения `--config KEY=VALUE`, которые вы передаёте, и когда опции остаются неустановленными, он выводит `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` Когда любая из неустановленных опций требуется, `(M required)` следует за `not yet set`.

Если вы установили из оболочки, передайте значения с `--config`, один флаг на опцию:

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

Когда каждая опция установлена, вывод установки не содержит строку `not yet set`. Чтобы открыть диалог позже вместо этого, запустите `/plugin configure my-plugin@my-marketplace` в сеансе.

Если вы передаёте ключ `--config`, который манифест не объявляет, плагин всё ещё устанавливается, и команда выводит `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` за которым следуют ключи, которые плагин объявляет.

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` сообщает об ошибках
</h3>

Вы запустили `claude plugin validate <path>` или `/plugin validate <path>` в сеансе, и он вывел `Found N errors` и `Validation failed`, затем вышел с кодом 1.

Валидатор читает манифест по пути, который вы даёте ему: `.claude-plugin/plugin.json` для каталога плагина или `.claude-plugin/marketplace.json` для каталога маркетплейса. Для маркетплейса он предваряет проблемы в собственном манифесте записи с индексом записи, как `plugins[1] plugin.json → json: ...`.

Таблица охватывает сообщения, которые останавливают валидацию, и два предупреждения, `No frontmatter block found` и `Unknown field '<key>'`, которые останавливают её только при передаче `--strict`. Другие предупреждения, такие как отсутствующее описание, не указаны.

| Сообщение                                                                                                | Причина                                                                                   | Исправление                                                                                                               |
| :------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `File not found: <path>`                                                                                 | Путь не имеет манифеста или не существует.                                                | Запустите команду против корня плагина или маркетплейса, каталога, который содержит `.claude-plugin/`.                    |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | Каталог не имеет манифеста `.claude-plugin/`.                                             | Создайте манифест или укажите на правильный каталог.                                                                      |
| `Invalid JSON syntax: <parse error>`                                                                     | Манифест или `hooks/hooks.json` не является действительным JSON.                          | Исправьте JSON. До тех пор, пока вы не исправите `hooks/hooks.json`, сеанс загружает плагин без hooks в этом файле.       |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | Путь компонента в манифесте не существует.                                                | Исправьте путь или создайте каталог.                                                                                      |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | Путь компонента выходит за пределы каталога плагина.                                      | Используйте пути внутри корня плагина.                                                                                    |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | Запись `skills` указывает на `SKILL.md` вместо его каталога.                              | Укажите на родительский каталог или `.` для `SKILL.md` на уровне корня.                                                   |
| `No frontmatter block found` или `YAML frontmatter failed to parse: <error>`                             | Файл skill, агента или команды имеет отсутствующий или недействительный YAML frontmatter. | Добавьте или исправьте frontmatter между разделителями `---`. Сообщается при валидации каталога плагина.                  |
| `Unknown field '<key>'`                                                                                  | Манифест имеет поле, которое схема не определяет.                                         | Удалите его или используйте имя, которое предлагает сообщение. Claude Code игнорирует неизвестные поля во время загрузки. |

Запустите команду снова после каждого исправления до тех пор, пока она не выведет никаких ошибок.

Поля `plugin.json` находятся в [справочнике манифеста](/docs/ru/plugins/manifest-reference), и сообщения на уровне маркетплейса находятся в разделе [Ошибки валидации маркетплейса](#marketplace-validation-errors).

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

Плагин не загружается с `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

Плагин имеет свой собственный `plugin.json`, и его запись маркетплейса устанавливает `strict: false`, одновременно объявляя любой из `commands`, `agents`, `skills`, `hooks`, `outputStyles` или `themes`. Удалите эти поля из записи или установите `strict: true` в записи, чтобы Claude Code добавил их к `plugin.json`. См. [Strict режим](/docs/ru/plugins/marketplace-reference#strict-mode).

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

Когда плагин загружается, журнал `claude --debug` в `~/.claude/debug/<session-id>.txt` записывает `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` Ничего не появляется в сеансе или на вкладке **Errors**.

Путь `commands` в манифесте существует, но не содержит файлов `.md` и не содержит `SKILL.md` в подкаталоге. Добавьте файлы команд или удалите путь из манифеста.

<h2 id="host-a-marketplace">
  Размещение маркетплейса
</h2>

Вы публикуете маркетплейс и пользователь сообщает об ошибке, или ваша собственная валидация не удаётся. Эти записи предназначены для владельца маркетплейса.

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Плагины с относительными путями не работают в маркетплейсах на основе URL
</h3>

Пользователи добавили ваш маркетплейс с URL `https://example.com/marketplace.json`. Установки плагинов, чей `source` — это относительный путь, такой как `./plugins/my-plugin`, не работают с `its marketplace entry path does not stay inside the marketplace directory`. Уже установленные плагины не загружаются с `Plugin source path refused`. Оба сообщения имеют [запись справочника ошибок](/docs/ru/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory).

Когда пользователь добавляет маркетплейс на основе URL, Claude Code загружает только сам файл `marketplace.json`. Он не загружает файлы плагинов по относительному пути с этого сервера, поэтому относительный путь в записи указывает на каталог, который никогда не был загружен. Дайте каждой записи источник, который Claude Code может получить самостоятельно, такой как репозиторий GitHub:

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

Альтернативно, разместите маркетплейс в репозитории git и скажите пользователям добавить его с URL репозитория. Для источника git Claude Code клонирует весь репозиторий, поэтому относительные пути разрешаются. Типы источников находятся в [справочнике маркетплейса](/docs/ru/plugins/marketplace-reference).

<h3 id="marketplace-validation-errors">
  Ошибки валидации маркетплейса
</h3>

Вы запустили `claude plugin validate .` из каталога вашего маркетплейса и он сообщил об ошибках или предупреждениях в самом файле маркетплейса.

`claude plugin validate` также валидирует каждую запись, чей `source` — это локальный путь, и предупреждает, когда `version` записи не совпадает с собственным манифестом плагина.

Таблица перечисляет сообщения на уровне маркетплейса. Сообщения на уровне записи — это сообщения плагина в разделе [`claude plugin validate` сообщает об ошибках](#claude-plugin-validate-reports-errors), предваренные `plugins[N] plugin.json →`.

| Сообщение                                                                                                                  | Вид            | Исправление                                                                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Duplicate plugin name "<name>" found in marketplace`                                                                      | Ошибка         | Дайте каждому плагину уникальное имя `name`.                                                                                                      |
| `Path contains "..": <path>` в `plugins[N].source`                                                                         | Ошибка         | Используйте пути относительно корня маркетплейса без сегментов `..`.                                                                              |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                           | Ошибка         | Удалите символ из имени, такой как escape или новая строка.                                                                                       |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                | Ошибка         | Удалите символ из имени плагина `name`.                                                                                                           |
| `Marketplace has no plugins defined`                                                                                       | Предупреждение | Добавьте по крайней мере одну запись в `plugins`.                                                                                                 |
| `No marketplace description provided`                                                                                      | Предупреждение | Добавьте описание на уровне верхнего уровня `description`.                                                                                        |
| `Plugin name "<name>" is not kebab-case` в `plugins[N] plugin.json → name`                                                 | Предупреждение | Переименуйте в строчные буквы, цифры и дефисы. Claude Code принимает другие формы, но синхронизация маркетплейса claude.ai их отклоняет.          |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                           | Предупреждение | Обновите запись, чтобы совпадать с `plugin.json`, который является авторитетным во время установки.                                               |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                  | Предупреждение | Переименуйте маркетплейс. Синхронизация управляемого маркетплейса Claude Desktop отклоняет `org`, `org-provisioned` и `unknown` в любом регистре. |
| `Marketplace name "<name>" is not accepted by Claude Desktop` или `Plugin name "<name>" is not accepted by Claude Desktop` | Предупреждение | Переименуйте максимум на 128 символов букв, цифр, `.`, `_` и `-`, начиная с буквы или цифры.                                                      |

До v2.1.247 имя маркетплейса, содержащее управляющие или двунаправленные символы форматирования, сообщалось только как `Marketplace name impersonates an official Anthropic/Claude marketplace`.

<h2 id="blocked-by-your-organization">
  Заблокировано вашей организацией
</h2>

Ваша организация развёртывает управляемые параметры, которые ограничивают плагины, и команда была отклонена с сообщением политики. Эти записи называют параметр, стоящий за каждым отказом, чтобы вы знали, что попросить у администратора. Для стороны администратора см. [Управляйте плагинами для вашей организации](/docs/ru/plugins/org).

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

Вы запустили `/plugin marketplace add`, `update` или установку, и Claude Code отказал с этой строкой. Для источника GitHub или git хост следует за источником в скобках, как в `'github:owner/repo' (github.com)`.

Ваш администратор установил `blockedMarketplaces` или `strictKnownMarketplaces` в управляемых параметрах, и этот источник не разрешён. Попросите вашего администратора разрешить источник или добавить один из разрешённых источников, которые перечисляет сообщение.

Сопоставьте остальную часть сообщения, чтобы увидеть, какой вид политики заблокировал источник:

* **`Allowed sources: <list>`**: блокировка поступает из списка разрешений `strictKnownMarketplaces` вместо списка блокировок `blockedMarketplaces`
* **`No external marketplaces are allowed.`**: список разрешений `strictKnownMarketplaces` пуст
* **`Tip:` о том, что сокращение предполагает github.com**: список разрешений разрешает хост git по имени хоста, и сокращение `owner/repo`, которое вы передали, указывает на github.com. Если репозиторий находится на вашем внутреннем хосте, добавьте его снова с его полным URL, такой как `git@your-git-host.com:owner/repo.git`

Маркетплейс, который вы добавили до того, как политика стала более ограничивающей, также перестаёт обновляться, потому что политика применяется при каждом обновлении.

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

Вкладка **Errors** показывает эту строку или `Marketplace "<name>" is blocked by enterprise policy` для маркетплейса, который у вас уже есть зарегистрирован.

Те же управляемые параметры, которые блокируют [источник маркетплейса](#marketplace-source-is-blocked-by-enterprise-policy), применяются во время загрузки. `strictKnownMarketplaces` не включает этот маркетплейс, или `blockedMarketplaces` его называет, поэтому Claude Code перестаёт его загружать и его плагины. Для варианта списка разрешений строка руководства показывает разрешённые источники или `Contact your administrator to configure allowed marketplace sources`. Для варианта списка блокировок она гласит `This marketplace source is explicitly blocked by your administrator`.

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

Установка была отклонена с этой строкой, включение с той же строкой, заканчивающейся `cannot be enabled`, или установка или обновление с одной, называющей причину: `Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy` или `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`.

Управляемые параметры блокируют этот плагин, его маркетплейс или зависимость, которая ему нужна. Попросите вашего администратора, какая запись применяется. Заблокированная зависимость означает, что плагин не может установиться до тех пор, пока маркетплейс зависимости не будет разрешён.

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

Вы запустили `claude` с `--plugin-dir`, `--plugin-url`, `--agents` или `--mcp-config`. Claude Code вышел с этим сообщением и `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

Ваш администратор установил `disableSideloadFlags` в управляемых параметрах, что отключает флаги, которые загружают плагины, агентов и серверы из произвольных путей. Загрузите плагин из одобренного маркетплейса вместо этого или попросите вашего администратора удалить параметр.

Связанное сообщение на вкладке **Errors** в `/plugin` — это `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`. Управляемые параметры включают или отключают этот плагин по имени, и Claude Code игнорирует вашу копию `--plugin-dir` его, чтобы флаг не мог переопределить политику.

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

Вы запустили `claude plugin init` или `claude plugin enable`, и это остановилось с этой строкой. Сообщение называет `strictKnownMarketplaces or blockedMarketplaces` и просит вашего администратора добавить `{"source":"skills-dir"}` в `strictKnownMarketplaces` или удалить его из `blockedMarketplaces`.

Источник `skills-dir` обозначает плагины, которые Claude Code загружает из вашего каталога `~/.claude/skills/`. Попросите вашего администратора сделать изменение, которое называет сообщение.

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

Вы установили или обновили плагин с источником `command`, и это остановилось с этой строкой и `The plugin was not installed or updated and its command was not run.`

Ваш администратор установил `disableCommandPluginSources`, поэтому Claude Code отказывает запуск команды, объявленной маркетплейсом, которая производит плагин. Установка `allowManagedHooksOnly` одна имеет тот же эффект, когда `disableCommandPluginSources` не установлена. Попросите вашего администратора, может ли плагин быть опубликован из типа источника, который разрешает политика.

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

Вы запустили `claude plugin marketplace update <name>`, и это не удалось с `Marketplace '<name>' is seed-managed (<dir>)` и подсказкой попросить вашего администратора.

Оператор предварительно заполнил этот маркетплейс через `CLAUDE_CODE_PLUGIN_SEED_DIR`, и Claude Code рассматривает управляемый семенем маркетплейс как доступный только для чтения. Массовое `marketplace update` пропускает его и обновляет остальные.

Чтобы изменить содержимое маркетплейса, попросите человека, который поддерживает образ семени, обновить его. Для процедуры см. [Seed контейнеры и CI](/docs/ru/plugins/org#seed-containers-and-ci).

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Справочник по загрузке плагинов](/docs/ru/plugins/loading): почему области видимости, кэш и приоритет ведут себя так, как они ведут себя
* [Справочник команд плагинов](/docs/ru/plugins/cli-reference): флаги, значения по умолчанию, вывод и коды выхода для команд `claude plugin`
* [Установка и управление плагинами](/docs/ru/plugins/install): шаги установки с начала
* [Управляйте плагинами для вашей организации](/docs/ru/plugins/org#troubleshoot-policy): устранение неполадок политики для администраторов
