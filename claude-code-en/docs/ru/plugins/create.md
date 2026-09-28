> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Создание плагина Claude Code

> Создайте свой первый плагин Claude Code с нуля, протестируйте его без marketplace и преобразуйте существующую конфигурацию .claude/.

Плагин — это директория с skills, agents, hooks и MCP серверами, а также файл `plugin.json`, называемый манифестом, который называет плагин. Claude Code загружает директорию как одно целое, поэтому вы можете поделиться ею с коллегами, установить её в несколько проектов или опубликовать на marketplace.

Эта страница предназначена для людей, которые пишут свои собственные плагины.

<Note>
  Эти случаи рассмотрены на других страницах:

  * **Установка плагина другого человека**: см. [Установка плагинов](/docs/ru/plugins/install)
  * **Не уверены, нужен ли вам плагин**: см. [Решите, нужен ли вам плагин](/docs/ru/plugins/overview#decide-whether-you-need-a-plugin) в обзоре
  * **Пользователи вашего плагина находятся на claude.ai или в Cowork**: одна и та же папка устанавливается там с другим подмножеством компонентов. См. [Плагины на claude.ai и в Cowork](https://claude.com/docs/plugins/overview)
</Note>

Начните с раздела, который соответствует тому, что у вас уже есть:

* **Ничего ещё**: следуйте [Создайте свой первый плагин](#create-your-first-plugin), затем [Разработка без marketplace](#develop-without-a-marketplace) и [Тестирование и отладка](#test-and-debug).
* **Файлы под `.claude/` уже есть**: пройдите пошаговое руководство первого плагина один раз, чтобы изучить макет, затем следуйте [Преобразование существующей конфигурации `.claude/`](#convert-an-existing-claude-setup).

<h2 id="decide-when-to-use-a-plugin">
  Решите, когда использовать плагин
</h2>

Skills, agents, hooks и MCP серверы работают отдельно в вашем проекте или домашней директории. Сохраняйте эту отдельную конфигурацию, пока она служит одному проекту или только вам. Создайте плагин, когда вы хотите поделиться конфигурацией с коллегами, установить её в несколько проектов или опубликовать версионные релизы.

Когда вы перемещаете отдельные skills, agents, hooks и MCP конфигурацию в плагин, их местоположение и имена меняются:

* **Где находятся файлы**: под собственной директорией плагина, называемой корнем плагина, как `skills/`, `agents/`, `hooks/hooks.json` и `.mcp.json`.
* **Как они названы**: skills и agents плагина получают имя плагина в качестве префикса, например `/my-plugin:hello`, поэтому два плагина могут каждый предоставить skill `hello` без конфликтов.

Чтобы переместить существующую конфигурацию в плагин, см. [Преобразование существующей конфигурации `.claude/`](#convert-an-existing-claude-setup).

<h2 id="create-your-first-plugin">
  Создайте свой первый плагин
</h2>

В этом пошаговом руководстве вы создаёте плагин, единственным компонентом которого является один skill — приветствие, и запускаете его с `--plugin-dir`, который загружает плагин на одну сессию без установки. Плагин может содержать любую комбинацию [компонентов](/docs/ru/plugins/components), таких как skills, agents, hooks и MCP серверы, и ни один не требуется; один skill — это наименьший пример, который показывает макет.

Вам нужен Claude Code [установленный и авторизованный](/docs/ru/quickstart#step-1-install-claude-code).

Откройте терминал в директории, где вы хотите хранить плагин, например `~/projects`, и запустите команды в этих шагах из неё. Вы можете хранить плагин где угодно, потому что вы передаёте его путь Claude Code при запуске сессии.

<Steps>
  <Step title="Создайте директорию плагина">
    Создайте директорию плагина с папкой `.claude-plugin/` внутри неё для хранения манифеста:

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="Напишите манифест">
    [Манифест](/docs/ru/plugins/manifest-reference) — это JSON файл с именем `plugin.json`, который сообщает Claude Code имя плагина и описывает его. Сохраните этот файл как `my-first-plugin/.claude-plugin/plugin.json`:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    Четыре поля делают следующее:

    * **`name`**: обязательно. Оно идентифицирует плагин и становится префиксом для каждого skill и agent, который предоставляет плагин. Не ставьте в нём пробелы.
    * **`description`**: текст, который пользователи видят для плагина в `/plugin`.
    * **`version`**: опционально. Установка его сохраняет пользователей на этой версии, пока вы не измените её; [Выпустить новую версию](/docs/ru/plugins/host-marketplace#release-a-new-version) говорит, когда установить или опустить его.
    * **`author`**: кого кредитовать. `name` обязателен внутри него; `email` и `url` опциональны.

    Каждое другое поле находится на [справочнике манифеста](/docs/ru/plugins/manifest-reference#fields).

    Только `plugin.json` находится внутри `.claude-plugin/`. Skill, который вы добавите далее, находится непосредственно под `my-first-plugin/`, рядом с этой папкой.
  </Step>

  <Step title="Добавьте skill">
    Единственным компонентом этого плагина является skill. Каждый skill — это директория под `skills/`, которая содержит файл `SKILL.md`. Создайте директорию skill:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Затем создайте `my-first-plugin/skills/hello/SKILL.md` с этим содержимым:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    Строка `disable-model-invocation: true` означает, что Claude не запускает skill самостоятельно, поэтому только вы его запускаете. Удалите эту строку из skill, который вы хотите, чтобы Claude запускал самостоятельно. Команда skill объединяет имя плагина и имя skill, поэтому вы запускаете этот как `/my-first-plugin:hello`. Для других полей frontmatter см. [справочник frontmatter skill](/docs/ru/skills#frontmatter-reference).
  </Step>

  <Step title="Проверьте плагин">
    Проверьте манифест и frontmatter skill перед запуском чего-либо:

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    Команда выводит путь манифеста, который она проверила, и `✔ Validation passed`. Если вместо этого она выводит `✘ Validation failed`, каждая строка выше строки результата называет поле для исправления. Посмотрите каждое сообщение под [`claude plugin validate` сообщает об ошибках](/docs/ru/plugins/troubleshooting#claude-plugin-validate-reports-errors).
  </Step>

  <Step title="Запустите Claude Code с плагином">
    Запустите сессию с загруженным плагином:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    После запуска Claude Code запустите skill:

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude ответит приветствием.
  </Step>
</Steps>

Плагин загружается только в сессиях, которые вы запускаете с `--plugin-dir`. Чтобы продолжить работу над ним без флага или протестировать сборку `.zip`, см. [Разработка без marketplace](#develop-without-a-marketplace).

<h3 id="share-the-plugin">
  Поделитесь своим плагином
</h3>

Плагин, который вы создали с помощью [Создайте свой первый плагин](#create-your-first-plugin), существует только на вашей машине. Когда он готов для других людей, есть три способа доставить его им:

* **Отправьте его нескольким людям напрямую**: дайте им директорию плагина или `.zip` её, и ничего не нужно публиковать. См. [Поделитесь плагином без marketplace](/docs/ru/plugins/publish#share-a-plugin-without-a-marketplace).
* **Перечислите его в своём собственном marketplace**: коллеги добавляют ваш marketplace один раз и устанавливают плагин по имени, и они получают ваши обновления. См. [Опубликуйте через свой собственный marketplace](/docs/ru/plugins/publish#publish-through-your-own-marketplace).
* **Отправьте его на community marketplace Anthropic**: после того как он будет указан, любой, кто добавит этот marketplace, сможет его установить. См. [Отправьте на community marketplace](/docs/ru/plugins/publish#submit-to-the-community-marketplace).

<h3 id="plugin-layout">
  Макет плагина
</h3>

Каждый вид [компонента](/docs/ru/plugins/components), такой как skills, agents, hooks и MCP серверы, находится в фиксированной директории под корнем плагина, который является директорией, которую вы передаёте `--plugin-dir`. Добавляйте только директории, которые вы используете. Чтобы пройти через полную директорию плагина и прочитать, что делает каждый файл, откройте [обозреватель плагинов](/docs/ru/plugins/components#explore-the-plugin-directory).

Таблица перечисляет директории, с которых начинают большинство плагинов, и [полный макет](/docs/ru/plugins/manifest-reference#standard-layout) перечисляет остальные.

| Местоположение               | Содержимое                                                                                                                       |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `.claude-plugin/plugin.json` | Манифест. Когда вы загружаете плагин с `--plugin-dir` и у него нет манифеста, Claude Code называет плагин в честь его директории |
| `skills/`                    | Одна директория `<name>/SKILL.md` на skill                                                                                       |
| `commands/`                  | Плоские файлы Markdown, более старая форма skills. Используйте `skills/` для новых плагинов                                      |
| `agents/`                    | Один файл Markdown на subagent                                                                                                   |
| `hooks/hooks.json`           | Конфигурация hook: ключ верхнего уровня `"hooks"`, значение которого имеет ту же форму, что и `hooks` в файле settings           |
| `.mcp.json`                  | Определения MCP сервера                                                                                                          |

<Warning>
  Только `plugin.json` находится внутри `.claude-plugin/`. Компоненты, сохранённые там, не загружаются.

  Корень плагина — это собственная директория плагина, а не сама `~/.claude/`. `.mcp.json`, сохранённый в `~/.claude/.mcp.json`, не загружается.
</Warning>

<h2 id="develop-without-a-marketplace">
  Разработка без marketplace
</h2>

Вам не нужен [marketplace](/docs/ru/plugins/overview#get-plugins-from-a-marketplace) для запуска плагина, который вы пишете. Загружайте его непосредственно с диска или URL вместо этого:

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): загружает директорию или архив `.zip` на одну сессию.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): загружает архив `.zip` с URL на одну сессию.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): создаёт плагин под `~/.claude/skills/`, который загружается в каждую сессию.

Если два плагина, загруженные разными способами, имеют одно имя, см. [Конфликты имён](/docs/ru/plugins/loading#name-conflicts) для того, какой из них Claude Code сохраняет.

<h3 id="load-a-directory-or-archive-for-one-session">
  Загрузите плагин на одну сессию
</h3>

Вы можете загрузить плагин на одну сессию тремя способами: из директории или архива `.zip` на диске с `--plugin-dir`, с URL с `--plugin-url` или из переменной окружения, когда вы не можете добавить флаг. Каждый плагин загружается только на эту сессию, и ничего не записывается в ваши settings для него. Когда вы редактируете файлы плагина во время сессии, запустите `/reload-plugins` для загрузки изменений.

<h4 id="from-a-directory-or-zip">
  Из директории или `.zip`
</h4>

Когда вы запускаете `claude` из вашей оболочки, передайте `--plugin-dir` с корневой директорией плагина или архивом `.zip` её. Повторите флаг для загрузки нескольких плагинов:

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  Из папки плагинов
</h4>

Чтобы загрузить несколько плагинов из одного места, передайте папку, которая их содержит, например `--plugin-dir ./plugins`. Загрузка папки плагинов требует Claude Code v2.1.265 или позже.

Если папка не имеет директории `.claude-plugin/` и нет компонентов плагина на её верхнем уровне, Claude Code рассматривает её как папку плагинов. Каждая непосредственная подпапка, которая имеет манифест `.claude-plugin/plugin.json`, затем загружается как отдельный плагин. Всё остальное в папке пропускается без ошибки, включая подпапку, которая не имеет манифеста. Если плагин в папке не загружается, проверьте, что его подпапка имеет `.claude-plugin/plugin.json`.

В интерактивной сессии вы также можете добавлять и удалять плагины в папке после запуска:

* Подпапка, которую вы добавляете, загружается как новый плагин, как только существует её манифест.
* Когда вы удаляете подпапку, её плагин выгружается.

Сообщение появляется в сессии для каждого из этих изменений. Если загрузка или выгрузка плагина в середине разговора [инвалидирует кэш подсказки](/docs/ru/prompt-caching#enabling-or-disabling-a-plugin), изменение вместо этого удерживается, и сообщение говорит вам запустить `/reload-plugins` для применения его.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  С URL
</h4>

Когда вы запускаете `claude` из вашей оболочки, передайте `--plugin-url` с адресом архива `.zip`, например артефакта сборки, который ваш CI публикует:

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code загружает архив при запуске. Чтобы загрузить несколько, повторите флаг или передайте URL разделённые пробелом в одном цитируемом аргументе.

Указывайте флаг только на архивы, которые вы контролируете или доверяете.

Если Claude Code не может загрузить архив или архив недействителен, он запускается без плагина и записывает ошибку загрузки плагина, которую вы можете просмотреть на вкладке **Errors** менеджера `/plugin`.

<h4 id="from-an-environment-variable">
  Из переменной окружения
</h4>

Чтобы загрузить плагины в сессии, где вы не можете добавить флаг `--plugin-dir`, перечислите их абсолютные пути в переменной окружения [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ru/env-vars#variables) вместо этого. Claude Code загружает каждый путь, как загружает путь `--plugin-dir`. Эти плагины загружаются в дополнение к любым, которые вы передаёте с `--plugin-dir`. [Параметры проекта и локальные параметры не могут установить эту переменную](/docs/ru/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS` требует Claude Code v2.1.280 или позже.

Управляемые параметры могут отключить `--plugin-dir` и `CLAUDE_CODE_PLUGIN_DIRS`. См. [Флаги, которые загружают плагин на одну сессию](/docs/ru/plugins/cli-reference#flags-that-load-a-plugin-for-one-session). Чтобы протестировать плагин вместе с плагином, от которого он зависит, см. [Протестируйте плагин и его зависимость локально](/docs/ru/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="scaffold-a-plugin-that-loads-every-session">
  Сделайте плагин загружаемым в каждую сессию
</h3>

Ваша личная директория skills — это `~/.claude/skills/`. Claude Code загружает любую папку там, которая содержит `.claude-plugin/plugin.json` как плагин в каждую сессию, без флага и без шага установки. `claude plugin init` создаёт один из этих плагинов для вас.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  Создайте плагин с `claude plugin init`
</h4>

`claude plugin init` пишет стартовый плагин под `~/.claude/skills/`. Требует Claude Code v2.1.157 или позже. Создайте один из вашей оболочки:

```bash theme={null}
claude plugin init my-tool
```

Команда создаёт `~/.claude/skills/my-tool/` с `.claude-plugin/plugin.json` и корневым `SKILL.md`. Она выводит `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` затем `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`

Передайте `--with skills` для того, чтобы `claude plugin init` создал skill под `skills/` для вас. Другие значения `--with` находятся на [справочнике команд плагина](/docs/ru/plugins/cli-reference#plugin-init).

<h4 id="skill-names-in-a-scaffolded-plugin">
  Назовите skills плагина
</h4>

Корневой skill в `~/.claude/skills/my-tool/SKILL.md` также является личным skill, поэтому вы вызываете его как `/my-tool`, а не `/my-tool:my-tool`. Skills, которые вы добавляете под `skills/` внутри плагина, получают префикс имени плагина, например `/my-tool:example`.

<h4 id="stop-loading-the-plugin">
  Остановите загрузку плагина
</h4>

Чтобы остановить загрузку созданного плагина, удалите его директорию или запустите `claude plugin disable my-tool@skills-dir` в вашей оболочке с именем `my-tool@skills-dir`, которое `claude plugin init` вывел. В ID `my-tool@skills-dir`, `skills-dir` стоит там, где было бы имя marketplace, потому что плагин загружается из вашей директории skills, а не из marketplace.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  Поделитесь плагином через репозиторий
</h4>

`claude plugin init` пишет плагин в вашу личную директорию skills в `~/.claude/skills/`, поэтому он загружается для вас в каждом проекте. Чтобы сделать плагин загружаемым для всех в одном репозитории, создайте тот же макет самостоятельно в `<project>/.claude/skills/<name>/`, включая его `.claude-plugin/plugin.json`. См. [Плагины, общие через репозиторий](/docs/ru/plugins/loading#plugins-shared-through-a-repository) для условий, при которых Claude Code его загружает.

<h2 id="test-and-debug">
  Тестирование и отладка
</h2>

Когда изменение вашего плагина не отображается, работайте через эти проверки по порядку. Каждая говорит вам, что Claude Code сделал с плагином:

1. В вашей оболочке запустите `claude plugin validate <path>`. Она проверяет манифест и frontmatter каждого файла skill, agent и command, и выходит `0` на `Validation passed`. Добавьте `--strict` для отказа и на предупреждения. Коды выхода и обработка директорий находятся на [справочнике команд плагина](/docs/ru/plugins/cli-reference#plugin-validate).
2. В запущенной сессии запустите `/reload-plugins` для применения правок, которые вы сделали на диске. Она выводит одну строку `Reloaded:` с подсчётами. Затем подтвердите, что skill загружен, введя его команду `/plugin-name:skill`, или найдя плагин на вкладке **Installed** в `/plugin`.
3. В той же сессии запустите `/plugin`. Вкладка **Installed** перечисляет ваш плагин и, в деталях плагина, компоненты, которые Claude Code нашёл. Вкладка **Errors** перечисляет то, что не загрузилось и почему, например путь в вашем манифесте, который не существует.
4. Вернитесь в вашу оболочку и запустите `claude plugin list`. Она выводит плагины только для сессии и директории skills в их собственных разделах с `Status: ✔ loaded` или ошибкой загрузки. Чтобы включить плагин, который вы разрабатываете, передайте `--plugin-dir` с его путём перед `plugin list`.

Чтобы проверить MCP сервер, запустите `/mcp` в сессии для просмотра статуса сервера. Когда сервер здоров, `/mcp` перечисляет его как подключённый. Если нет, см. [MCP серверы, которые не запускаются](/docs/ru/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Чтобы проверить hook, запустите событие, которое он соответствует. Например, попросите Claude отредактировать файл для запуска hook `PostToolUse`. Затем прочитайте [журнал отладки](/docs/ru/hooks#debug-hooks), который показывает, какие hooks соответствовали, их коды выхода и их вывод.

Следующие разделы охватывают сбои, которые вы, вероятно, столкнётесь при разработке, и [страница устранения неполадок](/docs/ru/plugins/troubleshooting#build-a-plugin) имеет полную запись для каждого.

<h3 id="a-component-path-isn’t-found">
  Путь компонента не найден
</h3>

Вкладка **Errors** в `/plugin` показывает `<component> path not found: <path>`, например `commands path not found`. Путь компонента в вашем манифесте, такой как `commands`, `skills`, `agents` или `hooks`, указывает на ничто. Исправьте путь или создайте директорию, затем запустите `/reload-plugins` в сессии. См. [`commands path not found`](/docs/ru/plugins/troubleshooting#commands-path-not-found).

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` в корне marketplace не загружает плагины под `plugins/`
</h3>

`--plugin-dir` принимает корневую директорию плагина, ту, которая содержит `.claude-plugin/plugin.json` и директории компонентов, такие как `skills/`. Если вы вместо этого указываете его на корень marketplace, Claude Code не читает `marketplace.json`, поэтому плагин под `plugins/` не загружается, и вы не видите ошибку. Указывайте флаг на папку одного плагина или добавьте marketplace. См. [запись устранения неполадок](/docs/ru/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components).

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  Плагин загружается, но его skills отсутствуют
</h3>

Директория `skills/` находится внутри `.claude-plugin/`, или запись `skills` в манифесте указывает на файл. Переместите `skills/` в корень плагина, укажите каждую запись `skills` на директорию, которая содержит `SKILL.md`, и запустите `/reload-plugins` в сессии. См. [Плагин загружается, но его skills отсутствуют](/docs/ru/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing).

<h3 id="the-userconfig-dialog-never-appears">
  Диалог `userConfig` никогда не появляется
</h3>

Диалог для опций [`userConfig`](/docs/ru/plugins/components#user-configuration) вашего плагина является частью установки через `/plugin` в сессии. Загрузка с `--plugin-dir` не показывает его, и `claude plugin install` в оболочке тоже. С загруженным плагином запустите `/plugin configure <plugin-name>` в сессии для открытия его. См. [Диалог `userConfig` никогда не появляется](/docs/ru/plugins/troubleshooting#the-userconfig-dialog-never-appears).

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  Проверьте, что плагин изменяет поведение Claude
</h3>

Плагин, который загружается без ошибок, всё ещё может не направить Claude так, как вы намеревались. `claude plugin eval`, который вы запускаете в вашей оболочке, запускает ваши тестовые случаи с плагином и без него и оценивает разницу. См. [Протестируйте плагины с evals](/docs/ru/plugin-evals), начиная с [Создайте свой первый набор eval](/docs/ru/plugin-evals#create-your-first-eval-suite).

<h2 id="convert-an-existing-claude-setup">
  Преобразование существующей конфигурации `.claude/`
</h2>

Если у вас уже есть skills, agents или hooks под директорией `.claude/` проекта, вы можете переместить их в плагин без переписывания их.

Запустите команды в этих шагах из корня проекта, который является директорией, которая содержит `.claude/`, потому что пути `cp` относительны к нему.

<Steps>
  <Step title="Создайте структуру плагина">
    Создайте директорию плагина и её папку `.claude-plugin/` рядом с `.claude/`. Вы можете переместить плагин куда угодно впоследствии.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Создайте `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Скопируйте ваши существующие файлы">
    Скопируйте каждую директорию конфигурации, которая у вас есть, в корень плагина, и пропустите команду для любой директории, которой у вас нет.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    Запустите `ls -a my-plugin` для подтверждения, что каждая директория, которую вы скопировали, появляется рядом с `.claude-plugin`.
  </Step>

  <Step title="Переместите ваши hooks">
    Если у вас есть hooks в `.claude/settings.json` или `.claude/settings.local.json`, создайте директорию hooks:

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    Создайте `my-plugin/hooks/hooks.json` и скопируйте объект `hooks` из вашего файла settings в него. Формат тот же.

    Этот пример показывает форму с одним hook, который запускает linter на каждом файле, который Claude пишет или редактирует. Замените пример на ваш собственный объект `hooks`.

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Протестируйте перенесённый плагин">
    Загрузите плагин на сессию:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Проверьте каждый компонент под его новым именем:

    * **Skills**: запустите `/my-plugin:deploy` для skill, который был `/deploy`.
    * **Subagents**: попросите Claude использовать agent `my-plugin:reviewer` для agent, который был `reviewer`.
    * **Hooks**: запустите событие, которое каждый hook соответствует.

    Если что-то отсутствует, работайте через [Тестирование и отладка](#test-and-debug).
  </Step>
</Steps>

Пока оригиналы всё ещё находятся под `.claude/`, они остаются загруженными рядом с копиями плагина:

* **Skills и agents**: два набора не конфликтуют, потому что skills и agents плагина несут префикс `my-plugin:`. `/deploy` и `/my-plugin:deploy` оба работают, и Claude видит `reviewer` и `my-plugin:reviewer` как два subagents.
* **Hooks**: hooks не имеют префикса, поэтому hook, который находится и в вашем файле settings, и в `hooks/hooks.json`, запускается дважды каждый раз, когда его событие срабатывает.

После того как вы подтвердили, что плагин работает, удалите оригиналы из `.claude/` и удалите объект `hooks` из вашего файла settings.

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Компоненты плагина](/docs/ru/plugins/components): добавьте agents, hooks, MCP серверы, LSP серверы и конфигурацию пользователя в ваш плагин
* [Протестируйте плагины с evals](/docs/ru/plugin-evals): напишите случаи eval и запустите их с `claude plugin eval` для проверки того, насколько надёжно плагин направляет поведение Claude
* [Опубликуйте плагин](/docs/ru/plugins/publish): версионируйте его, поместите его в marketplace и отправьте на community marketplace
* [Плагины на claude.ai и в Cowork](https://claude.com/docs/plugins/overview): одна и та же папка плагина устанавливается на claude.ai и в Cowork. Некоторые компоненты только для Claude Code
* [Справочник манифеста плагина](/docs/ru/plugins/manifest-reference): каждое поле `plugin.json`, правило пути и директория
* [Skills](/docs/ru/skills): напишите skills, которые предоставляет ваш плагин
* [Плагины Anthropic в репозитории claude-code](https://github.com/anthropics/claude-code/tree/main/plugins): полные рабочие примеры макета на этой странице, такие как `feature-dev` и `code-review`
