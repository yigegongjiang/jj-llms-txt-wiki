> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Расширение Claude с помощью skills

> Создавайте, управляйте и делитесь skills для расширения возможностей Claude в Claude Code. Включает пользовательские команды и встроенные skills.

Skills расширяют возможности Claude. Создайте файл `SKILL.md` с инструкциями, и Claude добавит его в свой набор инструментов. Claude использует skills при необходимости, или вы можете вызвать один напрямую с помощью `/skill-name`.

Создавайте skill, когда вы постоянно вставляете одни и те же инструкции, контрольный список или многошаговую процедуру в чат, или когда раздел CLAUDE.md превратился в процедуру, а не в факт. В отличие от содержимого CLAUDE.md, тело skill загружается только при его использовании, поэтому длинный справочный материал стоит почти ничего, пока вам он не понадобится.

<Note>
  Для встроенных команд, таких как `/help` и `/compact`, и встроенных skills, таких как `/debug` и `/code-review`, см. [справочник команд](/docs/ru/commands).

  **Пользовательские команды были объединены в skills.** Файл в `.claude/commands/deploy.md` и skill в `.claude/skills/deploy/SKILL.md` оба создают `/deploy` и работают одинаково. Ваши существующие файлы `.claude/commands/` продолжают работать. Skills добавляют дополнительные функции: каталог для вспомогательных файлов, frontmatter для [управления тем, кто вызывает skill — вы или Claude](#control-who-invokes-a-skill), и возможность для Claude автоматически загружать их при необходимости.
</Note>

Skills Claude Code следуют открытому стандарту [Agent Skills](https://agentskills.io), который работает с несколькими инструментами AI. Claude Code расширяет стандарт дополнительными функциями, такими как [управление вызовом](#control-who-invokes-a-skill), [выполнение в подагенте](#run-skills-in-a-subagent) и [динамическое внедрение контекста](#inject-dynamic-context). См. [Использование frontmatter skill вне Claude Code](#using-skill-frontmatter-outside-claude-code) для информации о том, какие поля frontmatter являются частью стандарта, а какие — расширениями Claude Code.

<h2 id="bundled-skills">
  Встроенные skills
</h2>

Claude Code включает набор встроенных skills, таких как `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` и `/claude-api`. Встроенные skills основаны на промптах: они дают Claude подробные инструкции и позволяют ему организовать работу, используя его инструменты. Большинство встроенных команд вместо этого выполняют фиксированную логику напрямую.

Вы вызываете встроенный skill так же, как любой другой skill, введя `/` и затем имя skill. Claude автоматически вызывает некоторые встроенные skills, когда это уместно; другие, включая `/verify`, запускаются только при вашем вызове, что позволяет вам контролировать, когда эти более длительные проверки тратят время и токены.

Большинство встроенных skills доступны в каждой сессии. Несколько зависят от конкретной функции: `/workflow-authoring`, например, доступен только когда [динамические workflows](/docs/ru/workflows) включены.

Чтобы отключить встроенные skills, используйте параметр [`disableBundledSkills`](/docs/ru/settings-reference#disablebundledskills).

<Note>
  Проверка настройки [`/doctor`](/docs/ru/commands#all-commands) остается доступной для ввода, когда `disableBundledSkills` включен, в Claude Code v2.1.205 и позже. Чтобы скрыть её, установите переменную окружения `DISABLE_DOCTOR_COMMAND` или запись [`skillOverrides`](#override-skill-visibility-from-settings) `"doctor": "off"`. До v2.1.205 `/doctor` была встроенной командой, а не встроенным skill.
</Note>

Встроенные skills перечислены вместе со встроенными командами в [справочнике команд](/docs/ru/commands), отмечены как **Skill** в столбце Purpose.

<h3 id="run-and-verify-your-app">
  Запуск и проверка вашего приложения
</h3>

Три встроенных skills работают вместе, чтобы запустить ваше приложение и подтвердить изменения в работающем приложении вместо просто тестов:

| Skill                  | Purpose                                                                                                                                     |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `/run`                 | Запустить и управлять вашим приложением, чтобы увидеть работающее изменение                                                                 |
| `/verify`              | Собрать и запустить ваше приложение, чтобы подтвердить, что изменение кода делает то, что должно, без возврата к тестам или проверкам типов |
| `/run-skill-generator` | Научить `/run` и `/verify` собирать и запускать ваш проект                                                                                  |

`/run` и `/verify` работают без настройки. Они определяют запуск из типа вашего проекта (CLI, сервер, TUI, управляемый браузером) и из того, что находится в вашем README, `package.json` или `Makefile`. Это определение становится ненадежным для проектов, которым нужно что-то большее, чем стандартный запуск: база данных, файл env, графический сеанс, многошаговая сборка.

`/run-skill-generator` вместо этого записывает рецепт. Он запускает ваше приложение из чистой среды, захватывает то, что сработало (команды установки, переменные env, скрипт запуска), и фиксирует это как skill для каждого проекта в `.claude/skills/run-<name>/`. После этого `/run`, `/verify` и любой другой агент в репозитории следуют записанному рецепту вместо его переоткрытия. Запустите `/run-skill-generator` один раз на проект, и снова, если процесс сборки или запуска изменится.

`/verify` также может записать свой собственный рецепт. Когда ему нужно собрать и управлять вашим приложением без записанного рецепта, он записывает то, что сработало, в `.claude/skills/verify/SKILL.md` в корне репозитория, или в затронутом каталоге пакета в монорепозитории, чтобы более поздние запуски и другие агенты следовали тем же шагам. В корне репозитория записанный skill заменяет встроенный `/verify`. Это требует Claude Code v2.1.200 или позже.

Claude редактирует записанный файл только когда он неправильно направил запуск, например команду, которая не удалась, или отсутствующий шаг, поэтому вы можете зафиксировать файл без различий для каждой сессии. До v2.1.205 встроенный skill говорил Claude складывать все, что запуск узнал, что вызывало частые конфликты слияния.

<h2 id="getting-started">
  Начало работы
</h2>

<h3 id="create-your-first-skill">
  Создайте свой первый skill
</h3>

Этот пример создает skill, который суммирует незафиксированные изменения в вашем git-репозитории и отмечает все рискованные моменты. Он загружает живой diff в подсказку перед тем, как Claude его прочитает, поэтому ответ основан на вашем фактическом рабочем дереве, а не на том, что Claude может предположить из открытых файлов. Claude автоматически загружает skill, когда вы спрашиваете об изменениях, или вы можете вызвать его напрямую с помощью `/summarize-changes`.

<Steps>
  <Step title="Создайте директорию skill">
    Создайте директорию для skill в папке ваших личных skills. Личные skills доступны во всех ваших проектах.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Напишите SKILL.md">
    Каждому skill нужен файл `SKILL.md` с двумя частями: YAML frontmatter между маркерами `---`, который говорит Claude, когда использовать skill, и содержимое markdown с инструкциями, которые Claude следует при запуске skill. Имя директории становится командой, которую вы вводите, а `description` помогает Claude решить, когда автоматически загружать skill.

    Сохраните это в `~/.claude/skills/summarize-changes/SKILL.md`:

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    Строка `` !`git diff HEAD` `` использует [динамическое внедрение контекста](#inject-dynamic-context): Claude Code запускает команду и заменяет строку её выводом перед тем, как Claude увидит содержимое skill, поэтому инструкции приходят с уже встроенным текущим diff.
  </Step>

  <Step title="Протестируйте skill">
    Откройте git-проект, внесите небольшое изменение в любой файл и запустите Claude Code, выполнив `claude`. Вы можете протестировать skill двумя способами.

    **Позвольте Claude вызвать его автоматически**, задав вопрос, который соответствует описанию:

    ```text theme={null}
    What did I change?
    ```

    **Или вызовите его напрямую** с именем skill:

    ```text theme={null}
    /summarize-changes
    ```

    В любом случае Claude должен ответить с кратким резюме вашего изменения и списком рисков.
  </Step>
</Steps>

<h2 id="where-skills-live">
  Выберите, где загружаются skills
</h2>

Место сохранения skill определяет, какие сеансы его загружают. Сохраните его в домашнем каталоге, чтобы получить его во всех проектах, зафиксируйте его в репозитории, чтобы поделиться им со всеми, кто там работает, или распространяйте его через plugin или управляемые параметры, чтобы охватить всю команду.

| Расположение         | Путь                                                                                                                 | Загружается в                                                                                                                                                                                                       |
| :------------------- | :------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Enterprise           | `.claude/skills/<skill-name>/SKILL.md` в [каталоге управляемых параметров](/docs/ru/managed-settings#delivery-mechanisms) | Все пользователи на машинах, где ваша организация его развертывает                                                                                                                                                  |
| Personal             | `~/.claude/skills/<skill-name>/SKILL.md`                                                                             | Все ваши проекты на этой машине, но не в [сеансах Cowork или облачных сеансах](#skills-in-cowork-and-cloud-sessions)                                                                                                |
| Project              | `.claude/skills/<skill-name>/SKILL.md`                                                                               | Сеансы в этом репозитории. Зафиксируйте его, чтобы ваша команда тоже его получила                                                                                                                                   |
| Nested               | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                      | Сеансы, запущенные в `<subdir>` или ниже. Сеанс, запущенный выше, загружает skill один раз, когда Claude работает с файлами там. См. [монорепозитории и подкаталоги](#discovery-from-parent-and-nested-directories) |
| Additional directory | `.claude/skills/<skill-name>/SKILL.md` в каталоге, который вы передаете с `--add-dir`                                | Этот сеанс. См. [каталоги вне проекта](#skills-from-additional-directories)                                                                                                                                         |
| Plugin               | `<plugin>/skills/<skill-name>/SKILL.md`                                                                              | Везде, где [plugin](/docs/ru/plugins/overview) включен, как `/plugin-name:skill-name`                                                                                                                                    |
| claude.ai account    | Skills, включенные для вашей учетной записи claude.ai                                                                | Сеансы Cowork, облачные сеансы и сеансы терминала, в которых вы входите с этой учетной записью. См. [Skills, синхронизированные с claude.ai](#how-synced-skills-behave)                                             |

Папки skill также следуют этим правилам:

* **Символические ссылки на папки**: запись `<skill-name>` в расположении enterprise, personal или project может быть символической ссылкой на каталог в другом месте на диске. Claude Code читает `SKILL.md` из целевого объекта и загружает skill один раз, даже если несколько расположений указывают на один и тот же целевой объект. Plugin skills [обрабатывают символические ссылки по-другому](/docs/ru/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks).
* **Зарезервированное имя**: не называйте папку skill `synced` в любом регистре. Claude Code использует `~/.claude/skills/synced/` для [skills, загруженных с claude.ai](#where-synced-skills-load), и пропускает skill, который вы создали с этим именем в расположениях enterprise, personal и project.
* **Файлы команд**: файл Markdown в `.claude/commands/` — это более старый формат, который все еще работает. Он поддерживает то же [frontmatter](#frontmatter-reference), кроме `name` и `paths`. Чтобы найти имя, которое вы вводите для его вызова, см. [Как skill получает имя команды](#how-a-skill-gets-its-command-name). Предпочитайте skill для новой работы, так как skills также поддерживают [вспомогательные файлы](#add-supporting-files).
* **Папка skill как plugin**: добавьте `.claude-plugin/plugin.json` в папку skill, и она загружается как [plugin](/docs/ru/plugins/loading#plugins-shared-through-a-repository) с именем `<name>@skills-dir`, поэтому она может объединять agents, hooks и MCP servers. В `.claude/skills/` проекта это требует предварительного принятия диалога доверия рабочей области.

<h3 id="discovery-from-parent-and-nested-directories">
  Загружайте skills в монорепозиториях и подкаталогах
</h3>

Claude Code загружает project skills из `.claude/skills/` в каталоге, где вы его запускаете, и в каждом родительском каталоге вплоть до корня репозитория, поэтому запуск в `packages/frontend/` все еще подхватывает skills, определенные в корне. Когда вы [перемещаете сеанс с `/cd`](/docs/ru/permissions#move-the-session-to-another-directory) на v2.1.246 или позже, Claude Code добавляет project skills нового каталога.

В сеансе, работающем в связанном [git worktree](/docs/ru/worktrees), Claude Code ищет родительские каталоги только до корня worktree. На Claude Code v2.1.277 или позже, когда checkout worktree не имеет каталога `.claude/skills` в его корне, Claude Code вместо этого загружает project skills основного checkout. См. [Что worktrees совместно используют с основным checkout](/docs/ru/worktrees#what-worktrees-share-with-the-main-checkout).

Skills в каталоге `.claude/skills/` ниже того места, где вы запустили, не загружаются при запуске. Они загружаются в первый раз, когда Claude читает или редактирует файл в этом подкаталоге, и остаются доступными для остальной части сеанса. До этого они не появляются в меню `/` и вы не можете вызвать их по имени. Чтобы загрузить их раньше, запустите `/add-dir` с путем подкаталога, что требует Claude Code v2.1.257 или позже.

Когда вложенный skill имеет то же имя, что и другой skill, оба остаются доступными. С skill `deploy` в корне репозитория и другим в `apps/web/.claude/skills/`:

* `/deploy` запускает skill корня. Claude Code также перечисляет варианты с квалификацией каталога для Claude с инструкцией вызвать тот, чей каталог содержит файлы, над которыми он работает, поэтому вложенный skill все еще применяется к работе в `apps/web/`.
* `/apps/web:deploy` запускает вложенный skill самостоятельно. Его описание называет каталог, к которому он применяется.

<h3 id="skills-from-additional-directories">
  Загружайте skills из каталога вне проекта
</h3>

Когда вы добавляете каталог с `--add-dir` или `/add-dir`, Claude Code загружает skills в `.claude/skills/` этого каталога, вместе с его `.claude/commands/` и `.claude/agents/`. Каталоги, которые Agent SDK добавляет через [`additionalDirectories`](/docs/ru/agent-sdk/typescript#options) в TypeScript или [`add_dirs`](/docs/ru/agent-sdk/python#claudeagentoptions) в Python, загружаются так же, потому что SDK передает их как `--add-dir`. Параметр `permissions.additionalDirectories` в `settings.json` предоставляет доступ только к файлам и не загружает ничего из этого.

Claude Code отслеживает `.claude/skills/` в каталоге, который вы передаете с `--add-dir` при запуске, как описано в [Редактируйте skill во время сеанса](#live-change-detection). Он не отслеживает `.claude/commands/` или `.claude/agents/` добавленного каталога, поэтому перезапустите сеанс после изменения файла там.

Эти загрузки зависят от источника параметра `project` [setting source](/docs/ru/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources), который включен по умолчанию. Политика [`strictPluginOnlyCustomization`](/docs/ru/settings-reference#strictpluginonlycustomization), [bare mode](/docs/ru/headless#start-faster-with-bare-mode) и [`--safe-mode`](/docs/ru/cli-reference#cli-flags) каждая ограничивает их дальше, как описано на этих страницах. См. [Дополнительные каталоги предоставляют доступ к файлам, а не конфигурацию](/docs/ru/permissions#additional-directories-grant-file-access-not-configuration) для полной таблицы того, что загружает добавленный каталог, включая `CLAUDE.md` и параметры plugin.

<h3 id="resolve-skills-that-share-a-name">
  Разрешайте skills, которые имеют одно имя
</h3>

Когда два skills имеют одно имя, откуда каждый из них пришел, определяет, какой из них запускает `/name`. Таблица охватывает расположения enterprise, personal, project, nested, plugin и claude.ai, встроенные skills и файлы команд:

| Одно имя в                                                                                                            | Какой из них запускается                                                                                                                                                                                                        |
| :-------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Два из enterprise, personal и project                                                                                 | Enterprise над personal, и personal над project. С `deploy` в обоих `~/.claude/skills/` и `.claude/skills/` проекта, `/deploy` запускает personal                                                                               |
| Любое из этих расположений и [встроенный skill](#bundled-skills)                                                      | Ваш skill заменяет встроенную команду, но не ее aliases. Project skill `code-review` заменяет `/code-review`, и встроенный alias `/review` никогда не запускает ваш skill                                                       |
| Skill и файл в `.claude/commands/`                                                                                    | Skill                                                                                                                                                                                                                           |
| Project-root skill и вложенный skill                                                                                  | Оба загружаются. См. [монорепозитории и подкаталоги](#discovery-from-parent-and-nested-directories)                                                                                                                             |
| Plugin skill и skill в любом из расположений выше                                                                     | Оба загружаются, потому что plugin skills имеют пространство имен как `/plugin-name:skill-name`                                                                                                                                 |
| Любое из вышеперечисленного и skill [синхронизированный с вашей учетной записью claude.ai](#how-synced-skills-behave) | Другой skill или команда. Синхронизированный skill все еще запускается как `/anthropic-skills:<name>`. См. [Когда имя синхронизированного skill совпадает с другой командой](#when-a-synced-skill-name-matches-another-command) |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Используйте skills в сеансах Cowork и облачных сеансах
</h3>

Сеансы [Cowork](https://claude.com/product/cowork) и [облачные сеансы](/docs/ru/cloud-environments#what-carries-over-from-your-setup), включая [routines](/docs/ru/routines), не читают `~/.claude/skills/` на вашей машине. Как интерактивные, так и запланированные сеансы Cowork загружают skills, включенные для вашей учетной записи claude.ai, синхронизированные при запуске сеанса; управляйте ими из **Customize** в боковой панели Desktop app или из параметров skills на claude.ai. Облачные сеансы дополнительно загружают project skills, зафиксированные в `.claude/skills/` клонированного репозитория.

Если skill существует только в `~/.claude/skills/` на вашей машине, Claude Code сообщает, что skill не найден, когда [routine](/docs/ru/routines) его вызывает, потому что каждый запуск routine запускается как свежий облачный сеанс. Чтобы сделать personal skill доступным в этих сеансах:

* Для сеансов Cowork и облачных сеансов включите skill для вашей учетной записи claude.ai.
* Для облачных сеансов вы можете вместо этого зафиксировать skill в `.claude/skills/` репозитория. Plugins, объявленные в `.claude/settings.json` репозитория и plugins, включенные только в ваши пользовательские параметры, [не загружаются в облачных сеансах](/docs/ru/cloud-environments#what-carries-over-from-your-setup).

[Desktop scheduled tasks](/docs/ru/desktop-scheduled-tasks) запускаются локально на вашей машине, поэтому они загружают `~/.claude/skills/`.

<h3 id="how-synced-skills-behave">
  Skills, синхронизированные с claude.ai
</h3>

Этот раздел применяется к вам, если вы используете сеансы Cowork или облачные сеансы, или входите в Claude Code в своем терминале с учетной записью claude.ai. В этих сеансах Claude Code загружает skills, включенные для вашей учетной записи claude.ai, без какой-либо настройки с вашей стороны, как описано в [Где загружаются синхронизированные skills](#where-synced-skills-load). Эти skills включают те, которые вы создаете или включаете в своих параметрах claude.ai, skills, которые предоставляет ваша организация, и встроенные skills Anthropic, такие как `pdf` и `xlsx`.

Claude Code загружает синхронизированный skill с вашей учетной записи, а не читает файл, который вы написали на машине, где работает сеанс, поэтому он применяет правила к синхронизированным skills, которые не применяются к skills, которые вы храните в [расположениях skills](#where-skills-live).

<h4 id="where-synced-skills-load">
  Где загружаются синхронизированные skills
</h4>

В сеансе Cowork или облачном сеансе Claude Code загружает skills, включенные для вашей учетной записи claude.ai, и [Skills в сеансах Cowork и облачных сеансах](#skills-in-cowork-and-cloud-sessions) говорит, как выбрать, какие skills получают эти сеансы.

В вашем терминале Claude Code синхронизирует эти skills в сеансах, в которых вы входите с вашей учетной записью claude.ai. Когда сеанс запускается, Claude Code загружает skills вашей учетной записи в `~/.claude/skills/synced/` в фоновом режиме, затем проверяет claude.ai на предмет изменений примерно каждые 10 минут во время работы сеанса. Когда проверка обнаруживает, что skill был добавлен, отредактирован или отключен на claude.ai, Claude Code добавляет, обновляет или удаляет его в работающем сеансе без перезагрузки. Синхронизация в сеансах терминала требует Claude Code v2.1.273 или позже.

Синхронизация никогда не задерживает запуск, потому что Claude ждет загрузки skill только при его вызове. Короткий [неинтерактивный](/docs/ru/headless) запуск может поэтому завершиться до загрузки вновь добавленного skill, в этом случае более поздний сеанс его загружает. Чтобы неинтерактивный запуск загрузил ваши skills и дождался списка перед ответом на подсказку, установите [`CLAUDE_CODE_SYNC_SKILLS`](/docs/ru/env-vars#variables) на `1`.

Claude Code синхронизирует только в сеансе, который входит с вашей учетной записью claude.ai и [получает флаги функций от Anthropic](/docs/ru/env-vars#features-that-need-feature-flag-fetching). Он не синхронизирует в этих сеансах:

* Сеанс, который не использует вход, сохраненный `/login`, например тот, который аутентифицируется с помощью API key, или тот, где `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN` или скрипт `apiKeyHelper` предоставляет учетные данные
* Сеанс, который не получает флаги функций, например тот, что на Amazon Bedrock, или тот, где вы установили `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* Сеанс в [bare mode](/docs/ru/headless#start-faster-with-bare-mode) или тот, который вы запускаете с `--safe-mode`
* Сеанс, где управляемые параметры вашей организации [блокируют skills для источников plugin](/docs/ru/settings-reference#strictpluginonlycustomization-skills), или тот, который вы запускаете со списком [`--setting-sources`](/docs/ru/cli-reference#cli-flags), который исключает `user`

Если вы входите с `/login` во время сеанса, перезапустите Claude Code, чтобы начать синхронизацию.

Skills, которые синхронизировал более ранний сеанс, остаются на диске. Claude Code загружает их в более поздних сеансах, вошедших в ту же учетную запись, даже когда он не может достичь claude.ai.

Claude Code загружает синхронизированные skills и никогда их не выгружает. Если вы или Claude отредактируете файл в `~/.claude/skills/synced/`, изменение не сохраняется в вашу учетную запись claude.ai, и более поздняя синхронизация может перезаписать или удалить его. Чтобы изменить синхронизированный skill, обновите его на claude.ai; следующая синхронизация загружает новую версию.

Чтобы увидеть, какие skills синхронизировались, запустите `/skills`. Меню перечисляет их под `claude.ai sync`.

Некоторые skills Anthropic, такие как `pdf` и `xlsx`, всегда синхронизируются. Для остальных включите или отключите skill в параметрах skills на claude.ai, чтобы изменить, синхронизируется ли он.

Чтобы остановить синхронизацию на машине, установите [`syncClaudeAiSkills`](/docs/ru/settings-reference#syncclaudeaiskills) на `false` в ваших пользовательских параметрах. Claude Code прекращает загрузку, и при следующем запуске он перемещает skills, которые он уже синхронизировал, в `~/.claude/skills/.trash/` и больше их не загружает. Ваша организация может отключить синхронизацию для всех, отключив Skills на claude.ai. Чтобы остановить синхронизацию, оставляя Skills включенными, она может установить тот же ключ в [управляемые параметры](/docs/ru/managed-settings).

Если ваша организация отключит Skills на claude.ai, Claude Code удалит загруженные skills и они перестанут загружаться. Удаленные skills перемещаются в `~/.claude/skills/.trash/`, где вы можете восстановить файлы до того, как [sweep удержания](/docs/ru/claude-directory#cleaned-up-automatically) их удалит. Как только ваша организация снова включит Skills, Claude Code загружает skills, которые вы включили при следующей синхронизации.

<h4 id="when-a-synced-skill-name-matches-another-command">
  Когда имя синхронизированного skill совпадает с другой командой
</h4>

Вы можете вызвать синхронизированный skill по его полному имени, `/anthropic-skills:<name>`, или по его короткому имени, `/<name>`. Когда другая команда использует это короткое имя, `/<name>` запускает другую команду, и синхронизированный skill запускается только как `/anthropic-skills:<name>`. С локальным skill `deploy` и синхронизированным `deploy`, `/deploy` запускает локальный skill и `/anthropic-skills:deploy` запускает синхронизированный. До v2.1.269 синхронизированный skill имел только свое короткое имя.

Другая команда может быть любой из этих:

* Встроенная команда или [встроенный skill](#bundled-skills), включая тот, который недоступен в вашем сеансе, например после отключения встроенных skills
* Skill на любом [локальном уровне](#where-skills-live) или файл в `.claude/commands/`
* Plugin skill
* [MCP prompt](/docs/ru/mcp#use-mcp-prompts-as-commands)

Claude Code помечает синхронизированные skills, чтобы вы могли сказать, откуда они пришли. Меню `/skills` и `/context` группируют синхронизированные skills под `claude.ai sync`, и меню команды `/` помечает их как поступающие с claude.ai.

При сравнении имен Claude Code игнорирует регистр, пробелы и невидимые символы и рассматривает формы совместимости, такие как полноширинные буквы и варианты тире, как их простые эквиваленты. Например, синхронизированный skill с именем `Commit` и локальный skill с именем `commit` считаются одним и тем же именем, поэтому `/commit` продолжает запускать ваш локальный skill.

Имя, которое отличается только похожей буквой из другого алфавита, считается другим именем, и метка `claude.ai sync` — это то, как вы различаете эти два. Эти проверки и метки требуют Claude Code v2.1.228 или позже.

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Как Claude Code обрабатывает frontmatter синхронизированного skill
</h4>

Claude Code применяет два правила к frontmatter синхронизированного skill:

* Claude Code соблюдает frontmatter в каждом виде сеанса, поэтому грант `allowed-tools` проходит через нормальный [поток разрешений](/docs/ru/permissions).
* Claude Code санитизирует отображаемый текст, который предоставляет skill, такой как его описание. Он удаляет управляющие символы, и в тексте, который достигает Claude, такой как описание, он также экранирует угловые скобки, чтобы текст не мог имитировать внутреннее форматирование Claude Code. Эта санитизация требует Claude Code v2.1.228 или позже.

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Как Claude Code обрабатывает тело синхронизированного skill
</h4>

То, что Claude Code делает с телом синхронизированного skill, зависит от того, где работает сеанс:

* В облачном сеансе тело сохраняет поведение, которое имеет локальный skill, потому что сеанс работает в изолированном контейнере.
* В сеансе Cowork на вашем рабочем столе тело сохраняет поведение локального skill, за исключением того, что Claude Code заменяет каждую строку команды `!` на заполнитель [`disableSkillShellExecution`](#inject-dynamic-context), как это делается для каждого skill, который вы предоставляете там.
* В любом другом сеансе на вашей машине Claude Code не запускает команды [`!`](#inject-dynamic-context), не прикрепляет файлы, которые ссылки `@` называют так же, как для локального skill, и не подставляет заполнители `${CLAUDE_PROJECT_DIR}` и `${CLAUDE_SESSION_ID}`, поэтому ссылки `@` и оба заполнителя достигают Claude как буквальный текст. Строка команды `!` также достигает Claude как буквальный текст, или как этот заполнитель, когда `disableSkillShellExecution` включен. Эта обработка требует Claude Code v2.1.228 или позже.

<h3 id="live-change-detection">
  Редактируйте skill во время сеанса
</h3>

Claude Code отслеживает каталоги skills на предмет изменений файлов, кроме [bare mode](/docs/ru/headless#start-faster-with-bare-mode). Когда вы добавляете, редактируете или удаляете skill в `~/.claude/skills/`, project `.claude/skills/` или `.claude/skills/` внутри каталога `--add-dir`, Claude Code подхватывает изменение в текущем сеансе без перезагрузки. Если вы создаете каталог skills верхнего уровня, который не существовал при запуске сеанса, перезапустите Claude Code, чтобы он мог отслеживать новый каталог.

Обнаружение изменений в реальном времени охватывает только текст `SKILL.md`. Для папки skill, которая также является [plugin](/docs/ru/plugins/loading#plugins-shared-through-a-repository), изменения в `hooks/`, `.mcp.json`, `agents/` и `output-styles/` требуют `/reload-plugins` для вступления в силу.

<h3 id="remove-a-skill">
  Удалите skill
</h3>

Способ удаления skill зависит от того, откуда он пришел:

* **Personal или project skill**: удалите каталог skill, `~/.claude/skills/<skill-name>/` или `.claude/skills/<skill-name>/`. Claude Code [удаляет его из `/skills` в текущем сеансе](#live-change-detection); содержимое, которое Claude Code уже загрузил из него, следует [жизненному циклу содержимого skill](#skill-content-lifecycle).
* **Enterprise skill**: администратор удаляет каталог skill из `.claude/skills/` внутри [каталога управляемых параметров](/docs/ru/managed-settings#delivery-mechanisms), например `/etc/claude-code/.claude/skills/<skill-name>/` на Linux.
* **Plugin skill**: отключите или удалите plugin, который его предоставляет, из меню `/plugin` или с помощью `/plugin uninstall <plugin-name>@<marketplace-name>`. Claude Code выгружает skills plugin, когда [изменение применяется](/docs/ru/plugins/cli-reference#reload-plugins) или когда вы перезагружаетесь.
* **Skill, синхронизированный с claude.ai**: отключите skill для вашей учетной записи claude.ai в том же месте, где вы его [включили](#skills-in-cowork-and-cloud-sessions). Claude Code удаляет его из `~/.claude/skills/synced/` при следующей [синхронизации ваших skills](#where-synced-skills-load). Если вы вместо этого удалите каталог вручную, следующая синхронизация загружает его снова, пока skill остается включенным на claude.ai.
* **Встроенный skill**: установите [`disableBundledSkills`](#bundled-skills) на `true`, чтобы отключить встроенные skills, или установите один skill на `"off"` в [`skillOverrides`](#override-skill-visibility-from-settings), чтобы скрыть его.

Чтобы сохранить personal или project skill, но остановить Claude от его вызова самостоятельно, установите [`disable-model-invocation: true`](#control-who-invokes-a-skill) в его frontmatter, или `"user-invocable-only"` в [`skillOverrides`](#override-skill-visibility-from-settings), когда вы не хотите редактировать файл.

<h2 id="configure-skills">
  Настройка skills
</h2>

Skills настраиваются через YAML frontmatter в начале `SKILL.md` и содержимое markdown, которое следует за ним.

<h3 id="types-of-skill-content">
  Типы содержимого skill
</h3>

Файлы skill могут содержать любые инструкции, но размышление о том, как вы хотите их вызывать, помогает определить, что включить:

**Справочное содержимое** добавляет знания, которые Claude применяет к вашей текущей работе. Соглашения, паттерны, руководства по стилю, знания предметной области. Это содержимое выполняется встроенным образом, поэтому Claude может использовать его вместе с контекстом вашего разговора.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Содержимое Task** дает Claude пошаговые инструкции для конкретного действия, такого как развертывания, коммиты или генерация кода. Это часто действия, которые вы хотите вызвать напрямую с помощью `/skill-name`, а не позволять Claude решать, когда их запускать. Добавьте `disable-model-invocation: true`, чтобы предотвратить автоматическое срабатывание Claude. Пример ниже добавляет `context: fork`, который запускает skill в собственном контексте подагента; см. [Запуск skills в подагенте](#run-skills-in-a-subagent).

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

Держите само тело кратким. После загрузки skill его содержимое [остается в контексте между ходами](#skill-content-lifecycle), поэтому каждая строка — это повторяющаяся стоимость токена. Указывайте, что делать, а не рассказывайте, как или почему, и применяйте тот же тест краткости, который вы бы применили к [содержимому CLAUDE.md](/docs/ru/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Справочник Frontmatter
</h3>

Настройте skill с помощью YAML [frontmatter](/docs/ru/glossary#frontmatter) между маркерами `---` в начале `SKILL.md`, и напишите инструкции skill как Markdown после закрывающего `---`. Имена полей используют строчные слова, разделенные дефисами, за исключением `when_to_use`. [Файл команды](#where-skills-live) в `.claude/commands/` принимает те же поля, кроме `name` и `paths`. Этот пример устанавливает четыре поля:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Все поля являются необязательными. Рекомендуется только `description`, чтобы Claude знал, когда использовать skill. Имя поля должно точно совпадать с таблицей, включая дефисы: Claude Code игнорирует поле, которое не распознает, без сообщения об ошибке.

Claude Code читает frontmatter только когда открывающий `---` является первой строкой файла. В противном случае он обрабатывает весь файл, включая маркеры `---`, как содержимое skill. Если YAML между маркерами не анализируется, skill все равно загружается без установленных полей; см. [Skill не срабатывает](#skill-not-triggering), чтобы найти и исправить ошибку.

Логические поля принимают `yes`, `no`, `on`, `off`, `1` и `0` в любом регистре, в дополнение к `true` и `false`. До v2.1.218 Claude Code распознавал только `true` и `false`.

| Поле                       | Обязательно   | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------- | :------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Нет           | Отображаемое имя, показываемое в списках skills. По умолчанию используется имя каталога. См. [Как skill получает имя команды](#how-a-skill-gets-its-command-name), чтобы узнать, как это поле взаимодействует с именем, которое вы вводите для вызова skill.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `description`              | Рекомендуется | Что делает skill и когда его использовать. Claude использует это, чтобы решить, когда применить skill. Если опущено, использует первую непустую строку содержимого markdown. Поместите основной вариант использования в первую очередь: объединенный текст `description` и `when_to_use` усекается на 1536 символов в списке skills для снижения использования контекста.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `when_to_use`              | Нет           | Дополнительный контекст для того, когда Claude должен вызвать skill, такой как фразы-триггеры или примеры запросов. Добавляется к `description` в списке skills и учитывается в ограничении 1536 символов.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `argument-hint`            | Нет           | Подсказка, показываемая при автодополнении, чтобы указать ожидаемые аргументы. Пример: `[issue-number]` или `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `arguments`                | Нет           | Именованные позиционные аргументы для [`$name` подстановки](#available-string-substitutions) в содержимом skill. Принимает строку, разделенную пробелами, или список YAML. Имена соответствуют позициям аргументов по порядку.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disable-model-invocation` | Нет           | Установите значение `true`, чтобы предотвратить автоматическую загрузку этого skill Claude. Используйте для рабочих процессов, которые вы хотите запустить вручную с помощью `/name`. Также предотвращает [предварительную загрузку skill в подагентов](/docs/ru/sub-agents#preload-skills-into-subagents). Начиная с v2.1.196, также предотвращает запуск skill при срабатывании [запланированной задачи](/docs/ru/scheduled-tasks) с skill в качестве подсказки. По умолчанию: `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `user-invocable`           | Нет           | Установите значение `false`, когда только Claude должен вызвать skill: Claude Code скрывает его из меню `/` и не запускает его при вводе `/name`. Используйте для фоновых знаний, которые пользователи не должны вызывать напрямую. По умолчанию: `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowed-tools`            | Нет           | Tools, которые Claude может использовать без запроса разрешения во время хода, который вызывает этот skill. Разрешение очищается при отправке следующего сообщения. Принимает строку, разделенную пробелами или запятыми, или список YAML. См. [Предварительное одобрение tools для skill](#pre-approve-tools-for-a-skill).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disallowed-tools`         | Нет           | Tools, удаленные из доступного пула Claude во время активности этого skill. Используйте для автономных skills, которые никогда не должны вызывать определенные tools, такие как `AskUserQuestion` для фонового цикла. Принимает строку, разделенную пробелами или запятыми, или список YAML. Ограничение очищается при отправке следующего сообщения. Как и правила отказа, это поле не может удалить [`EndConversation`](/docs/ru/tools-reference#endconversation-tool-behavior) пока остаются другие tools.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `model`                    | Нет           | Модель для использования, когда этот skill активен. Переопределение применяется для остальной части текущего хода и не сохраняется в параметрах. Модель сеанса возобновляется при отправке следующей подсказки. Принимает те же значения, что и [`/model`](/docs/ru/model-config), или `inherit`, чтобы сохранить активную модель. Значение, исключенное списком разрешений [`availableModels`](/docs/ru/model-config#restrict-model-selection) вашей организации, не используется, и сеанс сохраняет свою текущую модель. В [режиме auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) и в [режиме plan, пока классификатор проверяет команды](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode), модель, которую режим auto не поддерживает, также не используется, и сеанс сохраняет свою текущую модель. С `context: fork` значение устанавливает [модель подагента fork](#run-skills-in-a-subagent), и исключенное значение следует [тем же правилам, что и переопределение модели подагента](/docs/ru/model-config#restrict-model-selection). |
| `effort`                   | Нет           | [Уровень усилий](/docs/ru/model-config#adjust-effort-level) при активности этого skill. Переопределяет уровень усилий сеанса. По умолчанию: наследуется из сеанса. Опции: `low`, `medium`, `high`, `xhigh`, `max`; доступные уровни зависят от модели.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `context`                  | Нет           | Установите значение `fork`, чтобы запустить в контексте подагента fork. См. [Запуск skills в подагенте](#run-skills-in-a-subagent).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agent`                    | Нет           | Какой тип подагента использовать, когда установлен `context: fork`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `background`               | Нет           | Применяется только с `context: fork`. Установите значение `false`, чтобы ждать результата подагента fork в ходе, который вызвал skill, вместо [запуска его в фоне](#run-skills-in-a-subagent). По умолчанию: `true`. Требует Claude Code v2.1.218 или позже.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `hooks`                    | Нет           | Hooks, которые Claude Code регистрирует при вызове skill и продолжает запускать для остальной части сеанса. См. [Hooks в skills и agents](/docs/ru/hooks#hooks-in-skills-and-agents) для формата конфигурации и опции `once`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `paths`                    | Нет           | Glob-паттерны, которые ограничивают, когда этот skill активируется. Принимает строку, разделенную запятыми, или список YAML. Когда установлено, Claude загружает skill автоматически только при работе с файлами, соответствующими паттернам. Использует тот же формат, что и [правила для конкретных путей](/docs/ru/memory#path-specific-rules).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `shell`                    | Нет           | Shell для использования в `` !`command` `` и ` ```! ` блоках в этом skill. Принимает `bash` (по умолчанию) или `powershell`. Установка `powershell` запускает встроенные shell-команды через PowerShell, когда [инструмент PowerShell](/ru/tools-reference#powershell-tool) включен: он включен по умолчанию на Windows без Git Bash, включен по умолчанию с Git Bash для claude.ai и учетных записей Console, и требует `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` в сеансах Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry, а также на macOS, Linux и WSL. Установите значение `0`, чтобы отключить инструмент.                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `metadata`                 | Нет           | Свободная карта YAML для ваших собственных данных ключ-значение, таких как поля прав доступа или каталога, читаемые вашим собственным инструментарием из `SKILL.md`. Claude Code не действует на его содержимое и отбрасывает значение, которое не является картой. Не переиспользуйте имена полей frontmatter, такие как `paths`, в качестве ключей.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `license`                  | Нет           | Лицензия, охватывающая skill. Часть спецификации [Agent Skills](https://agentskills.io); см. [Использование skill frontmatter вне Claude Code](#using-skill-frontmatter-outside-claude-code). Claude Code принимает это поле, но не действует на него.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `compatibility`            | Нет           | Требования к окружению для skill, такие как предполагаемые продукты или системные предварительные условия, как определено спецификацией [Agent Skills](https://agentskills.io); см. [Использование skill frontmatter вне Claude Code](#using-skill-frontmatter-outside-claude-code). Принимает строку до 500 символов. Claude Code принимает это поле, но не действует на него.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Использование skill frontmatter вне Claude Code
</h4>

Claude Code принимает каждое поле в таблице выше. Вне Claude Code вы можете использовать только поля в спецификации [Agent Skills](https://agentskills.io):

| Путь распространения                                                                                                                  | Поля Frontmatter, которые вы можете использовать                               |
| :------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------- |
| Claude Code skills на [любом уровне](#where-skills-live), включая [plugin](/docs/ru/plugins/overview) skills                               | Каждое поле в таблице выше                                                     |
| Загрузки skills на claude.ai, Skills API и упаковка с `package_skill.py` из [anthropics/skills](https://github.com/anthropics/skills) | `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools` |

Когда вы включаете личный skill для вашего аккаунта claude.ai, например для использования в [сеансах Cowork и cloud](#skills-in-cowork-and-cloud-sessions) и процедурах, вы загружаете его на claude.ai, поэтому применяются те же правила.

Если вы включите какое-либо поле, которое спецификация не разрешает, упаковка или загрузка завершится с жесткой ошибкой вместо игнорирования поля:

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

Ограничение frontmatter шестью полями спецификации избегает ошибки неожиданного ключа выше. [Спецификация Agent Skills](https://agentskills.io) и [требования Skills API](https://docs.claude.com/en/api/skills-guide) определяют все остальное, что эти пути проверяют. Функции тела, специфичные для Claude Code, такие как [динамическое внедрение контекста](#inject-dynamic-context), не работают в чате claude.ai или через API. Claude Code принимает все шесть полей, поэтому frontmatter, который следует спецификации, загружается в Claude Code без изменений.

<h4 id="how-a-skill-gets-its-command-name">
  Как skill получает имя команды
</h4>

Команда, которую вы вводите для вызова skill, зависит от того, где находится файл skill и, для plugin skills, также от поля frontmatter `name`. В личном или проектном skill `name` устанавливает только отображаемый ярлык, показываемый в списках skills, а команда все еще поступает из имени каталога. В plugin skill `name` устанавливает последний сегмент команды, а префикс plugin остается на месте.

Таблица ниже показывает, откуда берется имя команды для каждого макета:

| Расположение Skill                                                                              | Источник имени команды                                                                                | Пример                                                                                                                           |
| :---------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| Каталог Skill под `~/.claude/skills/` или `.claude/skills/`                                     | Имя каталога                                                                                          | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                     |
| [Вложенный](#where-skills-live) каталог `.claude/skills/`, когда имя конфликтует с другим skill | Путь подкаталога относительно рабочего каталога, затем имя каталога skill                             | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                   |
| Файл под `.claude/commands/`                                                                    | Имя файла без расширения                                                                              | `.claude/commands/deploy.md` → `/deploy`                                                                                         |
| Файл в подкаталоге `.claude/commands/`                                                          | Путь подкаталога относительно `commands/` с каждым `/` заменен на `:`, затем имя файла без расширения | `.claude/commands/frontend/component.md` → `/frontend:component`                                                                 |
| Подкаталог Plugin `skills/`                                                                     | Frontmatter `name` или имя каталога, с пространством имен по plugin                                   | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`, или `/my-plugin:fancy` с `name: fancy`                                 |
| Plugin root `SKILL.md`                                                                          | Frontmatter `name`, с именем каталога plugin в качестве резервного варианта                           | `my-plugin/SKILL.md` с `name: review` → `/my-plugin:review`. См. [одиночный skill в корне plugin](/docs/ru/plugins/components#skills) |
| Skill [синхронизированный с claude.ai](#how-synced-skills-behave)                               | Имя skill на вашем аккаунте claude.ai, с префиксом `anthropic-skills:`                                | Skill аккаунта `deploy` → `/anthropic-skills:deploy`, или `/deploy`, если никакая другая команда не использует это имя           |

В plugin skill frontmatter `name` заменяет имя каталога в последнем сегменте команды, поэтому `my-plugin/skills/review/SKILL.md` с `name: fancy` становится `/my-plugin:fancy`. Голая команда `/fancy` также вызывает skill, если другая команда еще не использует это имя. Если `name`, который вы пишете, уже начинается с собственного префикса plugin, Claude Code не добавляет префикс снова на v2.1.246 или позже. Например, `name: my-plugin:fancy` все еще становится `/my-plugin:fancy`. С v2.1.216 по v2.1.245 Claude Code удваивал префикс, когда `name` уже его содержал.

В [неинтерактивных сеансах](/docs/ru/headless) имена `help` и `feedback` не зарезервированы для их встроенных команд, специфичных для терминала, поэтому plugin skill с одним из этих имен сохраняет свою голую команду там. Каждое другое встроенное имя, специфичное для терминала, такое как `/login`, остается зарезервированным, даже если команда не может запуститься в этих сеансах.

Для plugin-root `SKILL.md` нет каталога skill, из которого можно взять имя, поэтому `name` предоставляет весь последний сегмент. Без поля `name` Claude Code возвращается к имени каталога plugin.

<h4 id="available-string-substitutions">
  Доступные подстановки строк
</h4>

Skills поддерживают подстановку строк для динамических значений в содержимом skill:

| Переменная              | Описание                                                                                                                                                                                                                                                                                                                        |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$ARGUMENTS`            | Все аргументы, переданные при вызове skill. Когда ни один заполнитель не получает аргумент, Claude Code добавляет их как `ARGUMENTS: <value>`. См. [Передача аргументов в skills](#pass-arguments-to-skills).                                                                                                                   |
| `$ARGUMENTS[N]`         | Доступ к конкретному аргументу по индексу на основе 0, такому как `$ARGUMENTS[0]` для первого аргумента.                                                                                                                                                                                                                        |
| `$N`                    | Сокращение для `$ARGUMENTS[N]`, такое как `$0` для первого аргумента или `$1` для второго.                                                                                                                                                                                                                                      |
| `$name`                 | Именованный аргумент, объявленный в списке frontmatter [`arguments`](#frontmatter-reference). Имена соответствуют позициям по порядку, поэтому с `arguments: [issue, branch]` заполнитель `$issue` расширяется до первого аргумента, а `$branch` — до второго.                                                                  |
| `${CLAUDE_SESSION_ID}`  | Текущий ID сеанса. Полезно для логирования, создания файлов, специфичных для сеанса, или корреляции выходных данных skill с сеансами.                                                                                                                                                                                           |
| `${CLAUDE_EFFORT}`      | Текущий уровень усилий: `low`, `medium`, `high`, `xhigh` или `max`. Ultracode не является отдельным уровнем и сообщается как `xhigh`. Используйте это, чтобы адаптировать инструкции skill к активной настройке усилий.                                                                                                         |
| `${CLAUDE_SKILL_DIR}`   | Каталог, содержащий файл `SKILL.md` skill. Для plugin skills это подкаталог skill в plugin, а не корень plugin. Используйте это в bash-командах внедрения для ссылки на скрипты или файлы, поставляемые с skill, независимо от текущего рабочего каталога.                                                                      |
| `${CLAUDE_PROJECT_DIR}` | Корневой каталог проекта. Это тот же путь, который [hooks](/docs/ru/hooks#reference-scripts-by-path) и MCP-серверы получают как `CLAUDE_PROJECT_DIR`. Используйте это для ссылки на скрипты или файлы, специфичные для проекта, такие как `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, независимо от того, где установлен skill. |
| `${CLAUDE_PLUGIN_ROOT}` | Каталог установки plugin. Подставляется только в plugin skills. Используйте это для ссылки на скрипты или файлы, поставляемые в любом месте plugin, включая ресурсы, общие для skills plugin. См. [переменные окружения plugin](/docs/ru/plugins/manifest-reference#environment-variables).                                          |
| `${CLAUDE_PLUGIN_DATA}` | [Каталог постоянных данных](/docs/ru/plugins/components#path-variables-and-persistent-data) plugin, который сохраняется при обновлении plugin. Подставляется только в plugin skills. Используйте это для ссылки на установленные зависимости, сгенерированные файлы или кэши, которые должны пережить обновление.                    |

Claude Code подставляет `${CLAUDE_SKILL_DIR}` и `${CLAUDE_PROJECT_DIR}` в двух местах: содержимое markdown skill и Bash-правила в frontmatter [`allowed-tools`](#frontmatter-reference). В plugin skill Claude Code подставляет `${CLAUDE_PLUGIN_ROOT}` и `${CLAUDE_PLUGIN_DATA}` в тех же двух местах. Использование одной и той же переменной в обоих местах позволяет skill запустить поставляемый скрипт без подсказки разрешения. Следующий skill показывает паттерн:

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

Если этот skill установлен в `~/.claude/skills/render-chart/`, обе вхождения `${CLAUDE_SKILL_DIR}` расширяются до этого каталога. Правило `allowed-tools` затем соответствует точной команде, которую тело skill говорит Claude запустить, поэтому скрипт запускается без подсказки.

Подстановка `${CLAUDE_PROJECT_DIR}` требует Claude Code v2.1.196 или позже.

Индексированные аргументы используют кавычки в стиле shell, поэтому оборачивайте многословные значения в кавычки, чтобы передать их как один аргумент. Например, `/my-skill "hello world" second` делает `$0` расширяющимся до `hello world`, а `$1` — до `second`. Заполнитель `$ARGUMENTS` всегда расширяется до полной строки аргументов в том виде, в котором она была введена.

Индексированный заполнитель без соответствующего аргумента, такой как `$2`, когда был передан только один аргумент, остается в содержимом неизменным. Именованный заполнитель из frontmatter [`arguments`](#frontmatter-reference) без соответствующего аргумента расширяется до пустой строки.

Если вы передаете значение аргумента, которое само содержит текст, такой как `$1` или `$ARGUMENTS`, Claude Code вставляет его как буквальный текст и не расширяет его. Например, если тело skill содержит `Summarize $0` и вы запускаете `/summarize "$ARGUMENTS from yesterday"`, Claude получает `Summarize $ARGUMENTS from yesterday`. Claude Code все еще заменяет переменные `${CLAUDE_*}`, такие как `${CLAUDE_SKILL_DIR}`, после вставки аргументов.

Чтобы включить буквальный `$` перед цифрой, `ARGUMENTS` или объявленным именем аргумента, такой как `$1.00` в прозе, экранируйте его обратной косой чертой: `\$1.00`. Обратная косая черта перед любым другим `$` остается неизменной. Только одна обратная косая черта непосредственно перед токеном экранирует его. Удвоенная обратная косая черта, такая как `\\$1`, оставляет обе обратные косые черты на месте, и `$1` все еще расширяется до значения аргумента. Экранирование обратной косой чертой охватывает только эти заполнители аргументов. Обратная косая черта не предотвращает подстановку переменной `${CLAUDE_*}`, где переменная применяется.

**Пример использования подстановок:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Добавление вспомогательных файлов
</h3>

Skills могут включать несколько файлов в их каталог. Это держит `SKILL.md` сосредоточенным на основном, позволяя Claude получать доступ к подробному справочному материалу только при необходимости. Большие справочные документы, спецификации API или коллекции примеров не нужно загружать в контекст каждый раз при запуске skill.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Ссылайтесь на вспомогательные файлы из `SKILL.md`, чтобы Claude знал, что содержит каждый файл и когда его загружать:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Держите `SKILL.md` под 500 строк. Переместите подробный справочный материал в отдельные файлы.</Tip>

<h3 id="control-who-invokes-a-skill">
  Контроль того, кто вызывает skill
</h3>

По умолчанию как вы, так и Claude можете вызвать любой skill. Вы можете ввести `/skill-name`, чтобы вызвать его напрямую, и Claude может загрузить его автоматически, когда это актуально для вашего разговора. Два поля frontmatter позволяют вам ограничить это:

* **`disable-model-invocation: true`**: Только вы можете вызвать skill. Используйте это для рабочих процессов с побочными эффектами или которые вы хотите контролировать по времени, такие как `/commit`, `/deploy` или `/send-slack-message`. Вы не хотите, чтобы Claude решал развертываться, потому что ваш код выглядит готовым.

* **`user-invocable: false`**: Только Claude может вызвать skill. Используйте это для фоновых знаний, которые не являются действенными как команда. Skill `legacy-system-context` объясняет, как работает старая система. Claude должен знать это, когда это актуально, но `/legacy-system-context` не является значимым действием для пользователей.

Этот пример создает skill развертывания, который может запустить только вы. Если вы установите `disable-model-invocation: true`, Claude не сможет запустить skill автоматически:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Если Claude все равно попытается, Claude Code заблокирует вызов и инструктирует его не воспроизводить шаги развертывания другим способом, поэтому ожидайте, что Claude предложит запустить `/deploy` самостоятельно.

Вот как два поля влияют на вызов и загрузку контекста:

| Frontmatter                      | Вы можете вызвать | Claude может вызвать | Когда загружается в контекст                                      |
| :------------------------------- | :---------------- | :------------------- | :---------------------------------------------------------------- |
| (по умолчанию)                   | Да                | Да                   | Описание всегда в контексте, полный skill загружается при вызове  |
| `disable-model-invocation: true` | Да                | Нет                  | Описание не в контексте, полный skill загружается при вызове вами |
| `user-invocable: false`          | Нет               | Да                   | Описание всегда в контексте, полный skill загружается при вызове  |

<Note>
  В обычном сеансе описания skills загружаются в контекст, чтобы Claude знал, что доступно, но полное содержимое skill загружается только при вызове. [Подагенты с предварительно загруженными skills](/docs/ru/sub-agents#preload-skills-into-subagents) работают иначе: полное содержимое skill внедряется при запуске.
</Note>

<h3 id="skill-content-lifecycle">
  Жизненный цикл содержимого skill
</h3>

Когда вы или Claude вызываете skill, отрендеренное содержимое `SKILL.md` входит в разговор как одно сообщение и остается там в последующих ходах. Эта постоянность применяется к инструкциям skill, а не к его разрешениям: разрешение [`allowed-tools`](#pre-approve-tools-for-a-skill) очищается при отправке следующего сообщения. Claude Code не перечитывает файл skill в последующих ходах, поэтому пишите руководство, которое должно применяться на протяжении всей задачи, как постоянные инструкции, а не одноразовые шаги.

Когда Claude повторно вызывает skill, чье отрендеренное содержимое идентично копии, уже находящейся в контексте, Claude Code добавляет короткую заметку о том, что skill уже загружен, вместо второй копии содержимого. Когда отрендеренное содержимое отличается, потому что аргументы изменились или команда [динамического контекста](#inject-dynamic-context) произвела новый выход, Claude Code добавляет полное содержимое снова.

[Auto-compact](/docs/ru/how-claude-code-works#when-context-fills-up) переносит вызванные skills в рамках бюджета токенов. Когда разговор суммируется для освобождения контекста, Claude Code повторно прикрепляет самый последний вызов каждого skill после резюме, сохраняя первые 5000 токенов каждого. Повторно прикрепленные skills делят объединенный бюджет 25000 токенов. Claude Code заполняет этот бюджет, начиная с самого недавно вызванного skill, поэтому старые skills могут быть полностью удалены после компактирования, если вы вызвали много в одном сеансе.

Если skill кажется перестает влиять на поведение после первого ответа, содержимое обычно все еще присутствует, и модель выбирает другие tools или подходы. Усильте `description` skill и инструкции, чтобы модель продолжала его предпочитать, или используйте [hooks](/docs/ru/hooks), чтобы детерминированно обеспечить поведение. Если skill большой или вы вызвали несколько других после него, повторно вызовите его после компактирования, чтобы восстановить полное содержимое.

<h3 id="pre-approve-tools-for-a-skill">
  Предварительное одобрение tools для skill
</h3>

Поле `allowed-tools` предоставляет разрешение для перечисленных tools во время хода, который вызывает skill, поэтому Claude может использовать их без запроса вашего одобрения. Разрешение очищается при отправке следующего сообщения, даже если содержимое skill [остается в контексте](#skill-content-lifecycle); повторный вызов skill повторно применяет его для этого хода. Это не ограничивает, какие tools доступны: каждый tool остается вызываемым, и ваши [параметры разрешений](/docs/ru/permissions) все еще управляют tools, которые не указаны. Чтобы предварительно одобрить tools для всего сеанса, а не одного хода, добавьте правила разрешения к этим параметрам разрешений вместо этого.

Доверие рабочего пространства не ограничивает это поле. Claude Code применяет `allowed-tools` skill проекта всякий раз, когда вы или Claude вызываете skill, включая в запуск `-p` в папке, которой вы никогда не доверяли. Skill может предоставить себе широкий доступ к tools, поэтому проверьте `allowed-tools` skills, зафиксированных в репозитории, перед запуском Claude Code там.

Этот skill позволяет Claude запускать git-команды без одобрения за использование всякий раз, когда вы вызываете его:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Чтобы удалить tools из доступного пула Claude во время активности skill, перечислите их в `disallowed-tools` в frontmatter skill. Ограничение очищается при отправке следующего сообщения. Как и правила отказа, это поле не может удалить [`EndConversation`](/docs/ru/tools-reference#endconversation-tool-behavior) пока остаются другие tools. Чтобы заблокировать tools во всех skills и подсказках, добавьте правила отказа в ваши [параметры разрешений](/docs/ru/permissions).

<h3 id="pass-arguments-to-skills">
  Передача аргументов в skills
</h3>

Как вы, так и Claude можете передавать аргументы при вызове skill. Аргументы доступны через заполнитель `$ARGUMENTS`.

Этот skill исправляет проблему GitHub по номеру. Заполнитель `$ARGUMENTS` заменяется на все, что следует за именем skill:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Когда вы запускаете `/fix-issue 123`, Claude получает "Fix GitHub issue 123 following our coding standards..."

Если вы вызываете skill с аргументами, но ни один заполнитель в содержимом skill не получает один, Claude Code добавляет `ARGUMENTS: <your input>` в конец содержимого skill, чтобы Claude все еще видел, что вы ввели. Заполнитель — это `$ARGUMENTS`, индексированная форма, такая как `$1`, или именованный аргумент. Индексированный заполнитель без аргумента в его позиции остается как буквальный текст и не считается получившим один. Именованный заполнитель считается даже когда его позиция не имеет аргумента, потому что он расширяется до пустой строки.

Вы также можете складывать несколько skills в начале одного сообщения. Ввод `/write-tests /fix-issue 123` загружает оба skills и передает конечный текст `123` как `$ARGUMENTS` каждому из них. До v2.1.199 только первый skill загружался и получал `/fix-issue 123` как буквальный текст аргумента.

Claude Code расширяет первый skill плюс до пяти дополнительных, сложенных после него. Расширение останавливается на первом токене, который не является встроенным skill, вызываемым пользователем, поэтому skill, который запускается как [подагент fork](#run-skills-in-a-subagent), такой как [`/code-review`](/docs/ru/code-review#review-a-diff-locally), или тот, чьи аргументы сами могут начинаться с команды slash, такой как `/loop`, также заканчивается там. Этот токен и все, что после него, становятся текстом аргумента для каждого расширенного skill. `/code-review` запускается как подагент fork с v2.1.218; на более ранних версиях он запускался встроенным и складывался.

Чтобы получить доступ к отдельным аргументам по позиции, используйте `$ARGUMENTS[N]` или более короткий `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Запуск `/migrate-component SearchBar JavaScript TypeScript` заменяет `$ARGUMENTS[0]` на `SearchBar`, `$ARGUMENTS[1]` на `JavaScript` и `$ARGUMENTS[2]` на `TypeScript`. Тот же skill, использующий сокращение `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Продвинутые паттерны
</h2>

<h3 id="inject-dynamic-context">
  Внедрение динамического контекста
</h3>

Синтаксис `` !`<command>` `` запускает команды оболочки перед отправкой содержимого навыка Claude. Вывод команды заменяет заполнитель, поэтому Claude получает фактические данные, а не саму команду. Claude Code не запускает эти команды на вашей машине, когда навык [синхронизирован с вашего аккаунта claude.ai](#how-claude-code-handles-the-body-of-a-synced-skill). Это ограничение требует Claude Code v2.1.228 или позже.

Этот навык суммирует pull request, получая живые данные PR с помощью GitHub CLI. Команды `` !`gh pr diff` `` и другие запускаются первыми, и их вывод вставляется в подсказку:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

Подстановка выполняется один раз над исходным файлом. Вывод команды вставляется как простой текст и не переканализируется для дальнейших заполнителей `` !`<command>` ``, поэтому команда не может выдать заполнитель для последующего прохода расширения.

Встроенная форма распознается только когда `!` появляется в начале строки или сразу после пробела. Если `!` следует за другим символом, как в `` KEY=!`cmd` ``, заполнитель остается буквальным текстом и команда не запускается.

Для многострочных команд используйте блок кода в ограде, открытый с ` ```! ` вместо встроенной формы:

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

Чтобы отключить это поведение для навыков и пользовательских команд из источников пользователя, проекта, плагина или [additional-directory](#skills-from-additional-directories), установите `"disableSkillShellExecution": true` в [settings](/docs/ru/settings). Каждая команда заменяется на `[shell command execution disabled by policy]` вместо запуска. Встроенные и управляемые навыки не затрагиваются. Этот параметр наиболее полезен в [managed settings](/docs/ru/managed-settings), где пользователи не могут его переопределить.

Claude Code никогда не запускает эти команды на вашей машине, когда они появляются в навыках [синхронизированных с вашего аккаунта claude.ai](#how-synced-skills-behave), независимо от этого параметра. Это ограничение требует Claude Code v2.1.228 или позже. [How Claude Code handles the body of a synced skill](#how-claude-code-handles-the-body-of-a-synced-skill) говорит, что Claude получает вместо команды в каждом виде сеанса.

<Tip>
  Чтобы запросить более глубокое рассуждение при запуске навыка, включите `ultrathink` где-нибудь в содержимое навыка. См. [Use ultrathink for one-off deep reasoning](/docs/ru/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h4 id="how-injected-commands-run">
  Как запускаются внедренные команды
</h4>

Claude Code выбирает инструмент, который запускает внедренные команды навыка, из ключа `shell` в frontmatter навыка и вашей среды. Каждая комбинация запускает команды через инструмент Bash или инструмент PowerShell, кроме одной, которая полностью не проходит вызов:

* `shell: powershell`, с включенным [инструментом PowerShell](/docs/ru/tools-reference#powershell-tool): команды запускаются через инструмент PowerShell.
* `shell: bash` когда bash недоступен: вызов не проходит перед запуском любой команды. Это происходит на Windows без Git Bash. Claude Code показывает ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``.
* Любая другая комбинация: команды запускаются через инструмент Bash, когда bash доступен. Когда его нет, они запускаются через инструмент PowerShell.

Каждый инструмент запускает команды так же, как он запускает собственные команды оболочки Claude. Они совместно используют рабочий каталог, тайм-аут и обработку вывода:

* **Рабочий каталог**: Claude Code запускает каждую команду в текущем рабочем каталоге оболочки сеанса. Этот каталог перемещается, когда Claude запускает `cd`. Используйте [`${CLAUDE_SKILL_DIR}` или `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) в путях, которые должны разрешаться одинаково каждый раз.
* **stderr**: с оболочкой `bash` по умолчанию Claude Code объединяет stderr в stdout. Все, что команда записывает в stderr, появляется во внедренном тексте.
* **Тайм-аут**: каждая команда запускается под тайм-аутом по умолчанию инструмента Bash в 2 минуты [timeout](/docs/ru/tools-reference#timeout-and-output-limits). Когда инструмент Bash [перемещает команду с истекшим тайм-аутом в фоновый режим](/docs/ru/tools-reference#background-commands), навык все еще отображается. Внедренный текст сообщает о перемещении и называет фоновую задачу и файл, собирающий вывод команды. Когда команда — это та, которую инструмент Bash никогда не переводит в фоновый режим автоматически, Claude Code убивает ее при тайм-ауте. Этот отказ [прерывает вызов](#when-an-injected-command-fails).
* **Размер вывода**: вывод, превышающий встроенный потолок инструмента Bash, поступает как путь к файлу плюс краткий предпросмотр, а не усеченный текст. [Output limits](/docs/ru/tools-reference#output-limits) охватывает потолок и способы регулировки каждой границы.

Инструмент PowerShell применяет то же поведение тайм-аута, фонового режима и потолка вывода к командам, которые он запускает. Подробности см. в разделе [инструмент PowerShell](/docs/ru/tools-reference#powershell-tool).

<h4 id="when-an-injected-command-fails">
  Когда внедренная команда не выполняется
</h4>

Неудачная команда прерывает весь вызов навыка, а не только свой собственный заполнитель. Claude никогда не видит содержимое навыка для этого вызова. Прерывание показывает `Shell command failed for pattern "..."`. Сообщение об ошибке включает вывод команды под `[stderr]`.

С оболочкой `bash` по умолчанию любой ненулевой код выхода считается отказом. Применяется одно исключение: Claude Code рассматривает код выхода 1 из [команд поиска и сравнения](/docs/ru/tools-reference#output-limits) как нормальный результат и внедряет их вывод. Коды выхода 2 или выше не проходят даже для этих команд.

Какие команды получают исключение, зависит от оболочки:

* Оболочка `bash` по умолчанию: команды, перечисленные в [Output limits](/docs/ru/tools-reference#output-limits)
* `shell: powershell`, когда включен инструмент PowerShell: [другой набор](/docs/ru/tools-reference#shell-selection-in-settings-hooks-and-skills), который включает `grep` и `git diff`, но не `find` или `diff`

С оболочкой `bash` по умолчанию добавьте `|| true` к любой другой команде, которая, как вы ожидаете, выйдет с ненулевым кодом. Скрипт проверки, который выходит с кодом 1 при обнаружении проблем, является одним примером.

<h4 id="permission-checks-on-injected-commands">
  Проверки разрешений для внедренных команд
</h4>

Внедренные команды никогда не запрашивают разрешение во время отображения навыка. Claude Code проверяет каждую из них против ваших [правил разрешений](/docs/ru/permissions) сначала. Команда, которой соответствует правило deny, прерывает вызов с `Shell command permission check failed for pattern "..."`.

Вне [режима auto](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode), когда проверка разрешения команды возвращает что-либо, кроме разрешения, Claude Code прерывает вызов с той же ошибкой. Это включает правило, которое обычно вас спрашивает. Чтобы предотвратить прерывание несовпадающей команды здесь, предварительно одобрите ее с помощью [`allowed-tools`](#pre-approve-tools-for-a-skill). Правила deny и ask все равно переопределяют `allowed-tools`. См. [Manage permissions](/docs/ru/permissions#manage-permissions).

В режиме auto команда, которая в противном случае требовала бы вашего одобрения, не прерывает вызов. Навык загружается с инструкцией, указывающей Claude запустить команду первой, и собственный вызов Claude затем проходит через [обычные проверки режима auto](/docs/ru/permission-modes#how-the-classifier-evaluates-actions). Вызов все еще прерывается в [разветвленном навыке](#run-skills-in-a-subagent), который устанавливает `agent`, и в сеансе, где Claude не имеет [инструмента оболочки, который запускает внедренные команды](#how-injected-commands-run).

<h3 id="run-skills-in-a-subagent">
  Запуск навыков в подагенте
</h3>

Добавьте `context: fork` в ваш frontmatter, когда вы хотите, чтобы навык запускался в изоляции. Claude Code запускает новый подагент типа, установленного в поле `agent`, и дает ему содержимое навыка в качестве его подсказки. Подагент не видит историю вашего разговора, поэтому инструкции навыка должны быть самостоятельными.

<Note>
  Несмотря на название, навык с `context: fork` не запускается в [fork текущего разговора](/docs/ru/sub-agents#fork-the-current-conversation), который передал бы подагенту все, что вы обсуждали до сих пор. Когда задача зависит от этой истории, вместо использования `context: fork` разветвите разговор.
</Note>

Разветвленный подагент запускается в [фоновом режиме](/docs/ru/sub-agents#run-subagents-in-foreground-or-background): вы продолжаете работать, пока он запускается, и его результат поступает в ваш разговор при завершении. Установите `background: false` в frontmatter, чтобы вместо этого дождаться результата в ходу, который вызвал навык. До v2.1.218 разветвленные навыки всегда блокировали ход до завершения.

Claude Code также ждет результата, даже когда навык не устанавливает `background: false`, в случаях, подобных этим:

* В неинтерактивном режиме с флагом `-p` или Agent SDK
* Когда вы устанавливаете [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/ru/env-vars) на `1`, что также отключает все другие функции фоновых задач
* Когда вы вызываете разветвленный навык, пока более ранний вызов того же навыка все еще выполняется
* Когда [запланированная задача](/docs/ru/scheduled-tasks) срабатывает с навыком в качестве его подсказки

Разветвленный fork также запускается с [более узким набором инструментов, который применяется к фоновым подагентам](/docs/ru/sub-agents#run-subagents-in-foreground-or-background): подагент навыка — это обычный тип агента, поэтому исключение для подагентов, которые разветвляют разговор, его не охватывает. Если шаги вашего навыка зависят от инструмента вне этого набора, установите `background: false`, чтобы сохранить полный набор инструментов.

Разветвленный навык, который запускается в фоновом режиме, применяет свои правки вне [контрольных точек](/docs/ru/checkpointing) вашего сеанса, поэтому `/rewind` их не отменяет; используйте git для их отката.

<Warning>
  `context: fork` имеет смысл только для навыков с явными инструкциями. Если ваш навык содержит рекомендации, такие как "используйте эти соглашения API" без задачи, подагент получает рекомендации, но не имеет действенной подсказки и возвращается без значимого вывода.
</Warning>

Навыки и [подагенты](/docs/ru/sub-agents) работают вместе в двух направлениях:

| Подход                    | Системная подсказка     | Задача                         | Также загружает                                                                                                           |
| :------------------------ | :---------------------- | :----------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| Навык с `context: fork`   | Из типа агента          | Содержимое SKILL.md            | CLAUDE.md, согласно [startup context](/docs/ru/sub-agents#what-loads-at-startup) агента                                        |
| Подагент с полем `skills` | Тело markdown подагента | Сообщение делегирования Claude | Предварительно загруженные навыки + CLAUDE.md, согласно [startup context](/docs/ru/sub-agents#what-loads-at-startup) подагента |

С `context: fork` вы пишете задачу в своем навыке и выбираете тип агента для ее выполнения. Встроенные агенты Explore и Plan [пропускают CLAUDE.md и статус git](/docs/ru/sub-agents#what-loads-at-startup), чтобы сохранить их контекст небольшим, поэтому разветвленный навык, использующий `agent: Explore`, видит только содержимое SKILL.md и собственную системную подсказку агента. Для обратного варианта, где вы определяете пользовательский подагент, который использует навыки в качестве справочного материала, см. [Подагенты](/docs/ru/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Пример: навык исследования с использованием агента Explore
</h4>

Этот навык запускает исследование в разветвленном агенте Explore. Содержимое навыка становится задачей, а агент предоставляет инструменты только для чтения, оптимизированные для исследования кодовой базы:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Когда этот навык запускается:

1. Создается новый изолированный контекст
2. Подагент получает содержимое навыка в качестве его подсказки (инструкции "Research \$ARGUMENTS thoroughly")
3. Поле `agent` определяет среду выполнения (модель, инструменты и разрешения)
4. Подагент суммирует свои результаты и возвращает их в ваш основной разговор при завершении

Поле `agent` указывает, какую конфигурацию подагента использовать. Опции включают встроенные агенты (`Explore`, `Plan`, `general-purpose`) или любой пользовательский подагент из `.claude/agents/`. Если опущено, использует `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Ограничение доступа Claude к навыкам
</h3>

По умолчанию Claude может вызывать любой навык, у которого не установлено `disable-model-invocation: true`. Навыки, которые определяют `allowed-tools`, предоставляют Claude доступ к этим инструментам без одобрения за использование во время хода, который вызывает навык; грант очищается при отправке следующего сообщения. Ваши [параметры разрешений](/docs/ru/permissions) по-прежнему управляют поведением базового одобрения для всех остальных инструментов. Несколько встроенных команд также доступны через инструмент Skill, включая `/init` и `/security-review`. Другие встроенные команды, такие как `/compact`, нет.

Три способа контролировать, какие навыки может вызывать Claude:

**Отключить все навыки** отклонив инструмент Skill в `/permissions`:

```text theme={null}
# Add to deny rules:
Skill
```

**Разрешить или запретить определенные навыки** с помощью [правил разрешений](/docs/ru/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Синтаксис разрешений: `Skill(name)` для точного совпадения, `Skill(name *)` для совпадения префикса с любыми аргументами.

Если ваше правило `deny` называет псевдоним или неквалифицированное имя, а не собственное имя навыка, Claude Code все равно блокирует навык: с `Skill(review)` он блокирует встроенный `/code-review` через его псевдоним `/review`, и с `Skill(deploy)` он блокирует [вложенный навык](#where-skills-live), указанный как `apps/web:deploy` через его неквалифицированное имя. До v2.1.260 Claude Code не блокировал вложенный навык, указанный под его квалифицированным именем, когда правило deny называло только неквалифицированное имя.

Claude Code совпадает с правилом `allow` только с собственным именем навыка и именем в вызове Claude.

**Скрыть отдельные навыки** добавив `disable-model-invocation: true` в их frontmatter. Это полностью удаляет навык из контекста Claude.

<Note>
  С `user-invocable: false` вы не можете вызвать навык, но Claude все еще может. Чтобы предотвратить вызов Claude через инструмент Skill, установите `disable-model-invocation: true`.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Переопределение видимости навыка из параметров
</h3>

Параметр `skillOverrides` управляет видимостью навыка из ваших [параметров](/docs/ru/settings) вместо собственного frontmatter навыка. Используйте его для навыков, чей SKILL.md вы не хотите редактировать, например для тех, которые проверены в общем репозитории проекта. Меню `/skills` пишет его для вас: выделите навык и нажмите `Space` для циклирования состояний, затем `Esc` для сохранения в `.claude/settings.local.json`.

Каждый ключ — это имя навыка, и каждое значение — одно из четырех состояний:

| Значение                | Указано Claude | В меню `/` |
| :---------------------- | :------------- | :--------- |
| `"on"`                  | Имя и описание | Да         |
| `"name-only"`           | Только имя     | Да         |
| `"user-invocable-only"` | Скрыто         | Да         |
| `"off"`                 | Скрыто         | Скрыто     |

Меню `/skills` обозначает состояние `"user-invocable-only"` как `user-only`.

Начиная с v2.1.199, `"off"` также скрывает навык из списков команд, объявленных [Remote Control](/docs/ru/remote-control) клиентам и вызывающим [Agent SDK](/docs/ru/agent-sdk/skills#discover-available-commands), в дополнение к терминальному меню `/`. Вызов скрытого навыка по его полному имени все равно возвращает ошибку `skillOverrides` вместо его запуска.

Навык, отсутствующий в `skillOverrides`, рассматривается как `"on"`. Пример ниже сворачивает один навык до его имени и полностью отключает другой:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Некоторые встроенные навыки имеют псевдонимы, такие как `checkup` для `/doctor`. Если вы установите запись `skillOverrides` под псевдонимом в [managed settings](/docs/ru/managed-settings) или в файле, который вы передаете с флагом `--settings`, Claude Code применит его к навыку за псевдонимом. Вы можете только ограничить навык дальше через псевдоним, никогда не сделать его более видимым, и если вы также установите запись под собственным именем навыка в managed settings, эта запись имеет приоритет. До v2.1.260 Claude Code не применял запись под псевдонимом к навыку в каком-либо источнике параметров.

В пользовательских, проектных и локальных параметрах Claude Code совпадает с записями только с именами навыков. Если вы установите запись для `review` там, она применяется к навыку с именем `review`, а не к встроенному `/code-review` через его псевдоним `/review`.

Навыки плагинов не затрагиваются `skillOverrides`. Управляйте ими через `/plugin` вместо этого.

<h3 id="find-unused-skills">
  Поиск неиспользуемых навыков
</h3>

Каждый навык в [списке навыков](#skill-descriptions-are-cut-short) добавляет к вашему контексту на каждом ходу, независимо от того, использует ли Claude его когда-либо. Запустите `/skill-doctor`, чтобы увидеть, что стоит каждый из ваших навыков и как часто он используется, чтобы вы могли решить, какие из них отключить. В интерактивном сеансе отчет открывается на вкладке **Stats** менеджера `/plugin`. В [неинтерактивном режиме](/docs/ru/headless) с `-p` Claude Code печатает его как текст.

Отчет охватывает навыки в вашем сеансе, кроме встроенных навыков и корпоративных навыков. Он отмечает навыки в списке, которые никогда не были вызваны, и говорит, где их отключить. Из навыков, которые он говорит вам, где отключить, начните с тех, которые имеют наивысшую стоимость контекста. Отчет также перечисляет плагины, которые вы не использовали недавно.

`/skill-doctor` требует Claude Code v2.1.252 или позже и недоступен в сеансах, которые пропускают [получение флагов функций](/docs/ru/env-vars#features-that-need-feature-flag-fetching). Если вы запустите `/skill-doctor` через [Remote Control](/docs/ru/remote-control) со своего телефона или браузера, Claude Code ответит [`Skill usage reports are not available on this connection.`](/docs/ru/errors#skill-usage-reports-are-not-available-on-this-connection) вместо этого. Запустите `/skill-doctor` в терминале на машине, где запущен сеанс.

<h2 id="evaluate-and-iterate-on-a-skill">
  Оценка и итерация навыка
</h2>

Видение срабатывания навыка говорит вам, что Claude его нашел, но не то, что он сделал то, что вы намеревались. Чтобы узнать, работает ли навык, измерьте две вещи отдельно: вызывает ли Claude его на подсказках, которые он должен, и соответствует ли результат тому, что вы ожидаете, когда он это делает.

Проверка обоих — это сравнение базовых показателей. Соберите несколько реалистичных подсказок, запустите каждую в свежем сеансе с доступным навыком и снова с ним [отключенным](#override-skill-visibility-from-settings), и сравните результаты. Свежий сеанс важен, потому что оставшийся контекст от создания навыка скроет пробелы в написанных инструкциях.

Два инструмента автоматизируют это сравнение. Для навыка, который поставляется в [плагине](/docs/ru/plugins/overview), [`claude plugin eval`](/docs/ru/plugin-evals) запускает каждую подсказку в изолированном сеансе с плагином и без него, оценивает его с помощью оценщиков, которые вы определяете или которые он пишет для вас, и выходит с ненулевым кодом ниже порога, чтобы вы могли заблокировать CI на нем. Для итерации над одним навыком внутри разговора Claude Code плагин skill-creator ниже запускает аналогичный цикл с собственным форматом `evals/evals.json`. Эти два формата не взаимозаменяемы.

<h3 id="run-evals-with-skill-creator">
  Запуск оценок с skill-creator
</h3>

[Плагин `skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) автоматизирует цикл сравнения внутри Claude Code. Установите его из официального marketplace:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Если установка не удалась, сопоставьте сообщение, которое сообщает Claude Code:

* `Marketplace "claude-plugins-official" not found`: добавьте marketplace с помощью `/plugin marketplace add anthropics/claude-plugins-official`, затем повторите попытку установки.
* Плагин [не найден в marketplace](/docs/ru/plugins/install#install-a-plugin): проверьте имя плагина.

Если сводка установки сообщает `Run /reload-plugins to activate.`, Claude Code затем запускает эту перезагрузку для вас. Если перезагрузка предупреждает, что ваше следующее сообщение повторно прочитает разговор, запустите `/reload-plugins --force`, чтобы сделать навыки плагина доступными в текущем сеансе. Затем попросите Claude оценить существующий навык, например `evaluate my summarize-changes skill with skill-creator`. Плагин проведет вас через написание тестовых случаев и запустит цикл:

* **Тестовые случаи**: сохраняет подсказки, входные файлы и ожидаемое поведение в `evals/evals.json` внутри каталога навыка
* **Изолированные запуски**: порождает [подагента](/docs/ru/sub-agents) для каждого тестового случая, чтобы каждый запуск начинался с чистого контекста, и записывает количество токенов и продолжительность
* **Оценка**: проверяет каждое утверждение против результата и записывает успех или неудачу с доказательствами в `grading.json`
* **Эталон**: агрегирует процент успеха, время и токены для с навыком и без навыка в `benchmark.json`, чтобы вы могли сравнить улучшение процента успеха с накладными расходами на токены и время
* **Сравнение версий**: запускает слепое A/B между двумя версиями навыка, чтобы вы могли подтвердить, что редактирование является улучшением перед его фиксацией
* **Настройка описания**: генерирует подсказки should-trigger и should-not-trigger, измеряет процент попаданий и предлагает редактирование описания, когда навык активируется на неправильных запросах
* **Средство просмотра отзывов**: открывает отчет HTML, где вы проверяете каждый результат и записываете качественный отзыв, который следующая итерация читает

Для формата файла оценки и полного рабочего процесса итерации см. [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) на agentskills.io. Для справки о режимах эталона и сравнения см. [объявление skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Совместное использование skills
</h2>

Skills можно распространять на разных уровнях в зависимости от вашей аудитории:

* **Project skills**: Зафиксируйте `.claude/skills/` в системе контроля версий
* **Plugins**: Создайте директорию `skills/` в вашем [plugin](/docs/ru/plugins/overview)
* **Managed**: Разверните на уровне организации через [managed settings](/docs/ru/managed-settings)

<h3 id="generate-visual-output">
  Генерация визуального вывода
</h3>

Skills могут объединять и запускать скрипты на любом языке, предоставляя Claude возможности, которые невозможны в одном запросе. Один из паттернов — генерация визуального вывода: интерактивные HTML-файлы, которые открываются в вашем браузере для исследования данных, отладки или создания отчётов.

Этот пример создаёт обозреватель кодовой базы: интерактивное древовидное представление, где вы можете разворачивать и сворачивать директории, видеть размеры файлов с первого взгляда и определять типы файлов по цвету.

Создайте директорию Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Сохраните это в `~/.claude/skills/codebase-visualizer/SKILL.md`. Описание сообщает Claude, когда активировать этот Skill, а инструкции указывают Claude запустить встроенный скрипт. Путь к скрипту использует [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions), поэтому он правильно разрешается независимо от того, установлен ли skill на личном, проектном или уровне plugin:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Сохраните это в `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Этот скрипт сканирует дерево директорий и генерирует самодостаточный HTML-файл с:

* **Боковой панелью сводки**, показывающей количество файлов, количество директорий, общий размер и количество типов файлов
* **Столбчатой диаграммой**, разбивающей кодовую базу по типам файлов (топ 8 по размеру)
* **Сворачиваемым деревом**, где вы можете разворачивать и сворачивать директории, с цветовыми индикаторами типов файлов

Скрипт требует Python 3, но использует только встроенные библиотеки, поэтому нет пакетов для установки:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Для тестирования откройте Claude Code в любом проекте и попросите "Visualize this codebase." Claude запускает скрипт, который выводит путь к созданному файлу, например `Generated /path/to/codebase-map.html`, и открывает его в вашем браузере. Если вы работаете в среде без графического интерфейса, где браузер не открывается, выведенный путь подтверждает, что скрипт выполнен успешно.

Этот паттерн работает для любого визуального вывода: графики зависимостей, отчёты о покрытии тестами, документация API или визуализация схемы базы данных. Встроенный скрипт выполняет работу, а Claude управляет оркестрацией.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="skill-not-triggering">
  Skill not triggering
</h3>

Если Claude не использует ваш skill, когда это ожидается:

1. Проверьте, что описание включает ключевые слова, которые пользователи естественно произносят
2. Убедитесь, что skill появляется в `What skills are available?`
3. Попробуйте переформулировать ваш запрос, чтобы он лучше соответствовал описанию
4. Вызовите его напрямую с помощью `/skill-name`, если skill можно вызывать пользователю

Если frontmatter YAML неправильно сформирован, Claude Code загружает тело skill с пустыми метаданными, поэтому `/skill-name` все еще работает, но Claude не может сопоставить с вашим `description`. Запустите с `--debug`, чтобы увидеть ошибку парсинга.

Если skill поставляется в плагине, вы можете измерить, как часто он срабатывает на реалистичных запросах, вместо того чтобы проверять по одному: напишите eval case с [`tool_used: Skill` grader](/docs/ru/plugin-evals#create-your-first-eval-suite) и запустите его с `claude plugin eval` после каждого изменения описания.

Чтобы найти файлы `SKILL.md`, frontmatter которых не парсится, запустите [`claude plugin validate`](/docs/ru/plugins/cli-reference#validate-a-directory) в директории skills, например `claude plugin validate .claude/skills` для skills проекта или `claude plugin validate ~/.claude/skills` для личных skills. Требуется Claude Code v2.1.233 или позже.

<h3 id="skill-triggers-too-often">
  Skill triggers too often
</h3>

Если Claude использует ваш skill, когда вы этого не хотите:

1. Сделайте описание более специфичным
2. Добавьте `disable-model-invocation: true`, если вы хотите только ручной вызов

<h3 id="skill-descriptions-are-cut-short">
  Skill descriptions are cut short
</h3>

Claude Code загружает список имен skills и описаний в контекст, чтобы Claude знал, что доступно. Список всегда содержит каждое имя skill, но если у вас много skills, Claude Code сокращает описания, чтобы они поместились в бюджет символов списка, что может удалить ключевые слова, необходимые Claude для сопоставления вашего запроса. Бюджет масштабируется на 1% от окна контекста модели. Когда список переполняется, Claude Code удаляет описания, начиная с skills, которые вы вызываете реже всего, поэтому skills, которые вы используете чаще всего, сохраняют полный текст.

Запустите `/doctor` для оценки стоимости контекста списка и его основных участников. Чтобы найти skills, стоящие отключения, запустите [`/skill-doctor`](#find-unused-skills). Когда список превышает свой бюджет, Claude Code также записывает предупреждение в журнал отладки, видимый с [`--debug`](/docs/ru/cli-reference#cli-flags).

Строка Skills в `/context` сообщает размер списка после применения бюджета, поэтому он соответствует тому, что получает модель. До v2.1.196 строка считала полный текст каждого описания и могла показать значение в несколько раз больше, чем настроенный бюджет.

Чтобы увеличить бюджет, установите параметр [`skillListingBudgetFraction`](/docs/ru/settings-reference#skilllistingbudgetfraction) (например `0.02` = 2%) или переменную окружения `SLASH_COMMAND_TOOL_CHAR_BUDGET` на фиксированное количество символов. Чтобы освободить бюджет для других skills, установите записи с низким приоритетом на `"name-only"` в [`skillOverrides`](#override-skill-visibility-from-settings), чтобы они отображались без описания. Вы также можете сократить текст `description` и `when_to_use` в источнике: поместите основной вариант использования в первую очередь, так как объединённый текст каждой записи ограничен 1536 символами независимо от бюджета. Ограничение настраивается с помощью [`skillListingMaxDescChars`](/docs/ru/settings-reference#skilllistingmaxdescchars).

<h3 id="personal-skills-disappeared">
  Personal skills disappeared
</h3>

Если папки skills, которые вы создали в `~/.claude/skills/`, исчезли, посмотрите в `~/.claude/skills/.trash/`. Когда Claude Code [синхронизирует skills из claude.ai](#how-synced-skills-behave), он загружает их в отдельную подпапку `synced` и не перемещает и не удаляет папки, которые вы создаёте.

До v2.1.280 файл с именем `manifest.json` в `~/.claude/skills/` заставлял Claude Code перемещать папки skills, которые этот файл указывал, в папку с временной меткой под `~/.claude/skills/.trash/`, и эти skills перестали загружаться.

Чтобы восстановить skill, переместите его папку из папки с временной меткой обратно в `~/.claude/skills/`. Сделайте это до того, как [очистка хранилища](/docs/ru/claude-directory#cleaned-up-automatically) удалит записи из корзины, по умолчанию через 30 дней после их перемещения в корзину.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* **[Отладка вашей конфигурации](/docs/ru/debug-your-config)**: диагностируйте, почему skill не появляется или не срабатывает
* **[Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills)**: формат файла eval и рабочий процесс итерации на agentskills.io
* **[Skill authoring best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: рекомендации по написанию, которые применяются во всех продуктах Claude
* **[Subagents](/docs/ru/sub-agents)**: делегируйте задачи специализированным агентам
* **[Plugins](/docs/ru/plugins/overview)**: упакуйте и распространяйте skills с другими расширениями
* **[Hooks](/docs/ru/hooks)**: автоматизируйте рабочие процессы вокруг событий инструментов
* **[Memory](/docs/ru/memory)**: управляйте файлами CLAUDE.md для постоянного контекста
* **[Commands](/docs/ru/commands)**: справочник для встроенных команд и встроенных skills
* **[Permissions](/docs/ru/permissions)**: управляйте доступом к инструментам и skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: skills проекта, зафиксированные в репозитории, также загружаются при использовании этого репозитория в канале Claude Tag
