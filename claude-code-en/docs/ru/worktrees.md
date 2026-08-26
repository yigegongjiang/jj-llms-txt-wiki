> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Запуск параллельных сеансов с worktrees

> Изолируйте параллельные сеансы Claude Code в отдельных git worktrees, чтобы изменения не конфликтовали. Охватывает флаг `--worktree`, изоляцию subagent, `.worktreeinclude`, очистку и hooks для не-git VCS.

[git worktree](https://git-scm.com/docs/git-worktree) — это отдельный рабочий каталог с собственными файлами и веткой, использующий ту же историю репозитория и удалённый сервер, что и ваша основная копия. Запуск каждого сеанса Claude Code в собственном worktree означает, что правки в одном сеансе никогда не затрагивают файлы в другом, поэтому вы можете одновременно работать с Claude, который создаёт функцию в одном терминале и исправляет ошибку во втором.

На этой странице рассматривается изоляция worktree в CLI. Всё ниже предполагает наличие git-репозитория. Для других систем контроля версий см. [Не-git системы контроля версий](#non-git-version-control). [Десктопное приложение](/docs/ru/desktop#work-in-parallel-with-sessions) автоматически создаёт worktree для каждого нового сеанса.

Worktrees — один из нескольких способов запуска Claude параллельно. Они изолируют правки файлов, в то время как [subagents](/docs/ru/sub-agents) и [agent teams](/docs/ru/agent-teams) координируют саму работу. См. [Запуск агентов параллельно](/docs/ru/agents), чтобы сравнить подходы, или перейдите к [Изолируйте subagents с помощью worktrees](#isolate-subagents-with-worktrees), чтобы использовать worktrees и subagents вместе.

<h2 id="start-claude-in-a-worktree">
  Запустите Claude в worktree
</h2>

Передайте `--worktree` или `-w`, чтобы создать изолированный worktree и запустить Claude в нём. По умолчанию worktree создаётся в `.claude/worktrees/<value>/` в корне вашего репозитория на новой ветке с именем `worktree-<value>`:

```bash theme={null}
claude --worktree feature-auth
```

Чтобы поместить worktrees в другое место, настройте hook [`WorktreeCreate`](#non-git-version-control). Запустите команду снова с другим именем в другом терминале, чтобы запустить второй изолированный сеанс:

```bash theme={null}
claude --worktree bugfix-123
```

Если вы опустите имя, Claude сгенерирует одно, например `bright-running-fox`:

```bash theme={null}
claude --worktree
```

Вы также можете попросить Claude "работать в worktree" во время сеанса, и он создаст один с помощью инструмента [`EnterWorktree`](/docs/ru/tools-reference). После входа в worktree Claude может переключиться непосредственно на другой worktree в `.claude/worktrees/`, вызвав `EnterWorktree` с целевым путём. Предыдущий worktree остаётся на диске нетронутым.

Вход в путь вне каталога `.claude/worktrees/` репозитория требует вашего одобрения в первую очередь, потому что это перемещает рабочий каталог сеанса, доступ на запись и конфигурацию проекта, такую как `CLAUDE.md` и settings, в это место. Правило разрешения `EnterWorktree` [permission rule](/docs/ru/permissions) или выбор "не спрашивать снова" не подавляет это приглашение; только режим `bypassPermissions` пропускает его. До версии 2.1.206 Claude мог войти в любой существующий путь worktree без запроса.

Начиная с версии 2.1.198, вход или выход из worktree также перемещает стенограмму сеанса в хранилище проекта этого каталога, так же как это делает [`/cd`](/docs/ru/commands), поэтому `/desktop` и `--resume` находят сеанс там после этого. Worktrees, созданные hook [`WorktreeCreate`](#non-git-version-control), исключены и сохраняют стенограмму в каталоге запуска.

Worktrees работают с включённым [sandboxing](/docs/ru/sandboxing#filesystem-isolation): песочница позволяет записывать в общий каталог `.git` основного репозитория, поэтому команды, такие как `git commit`, могут обновлять refs и индекс изнутри связанного worktree.

Перед использованием `--worktree` интерактивно в каталоге в первый раз примите диалог доверия рабочей области, запустив `claude` один раз в этом каталоге. Если доверие ещё не было принято, `--worktree` завершится с ошибкой и предложит вам сначала запустить `claude` в каталоге. Неинтерактивные запуски с `-p` пропускают [проверку доверия](/docs/ru/security), поэтому `claude -p --worktree` продолжает работу без неё.

Если Claude Code не может войти в каталог worktree при запуске, например потому что hook [`WorktreeCreate`](/docs/ru/hooks#worktreecreate) вывел что-то другое, чем каталог, который он создал, или потому что каталог был удалён после его настройки, Claude Code выводит ошибку с указанием пути и завершается с кодом 1. До версии 2.1.205 это приводило к сбою сеанса, а с `-p` это зависало примерно на 30 секунд перед выходом с кодом 0.

Плагины, установленные в [области проекта](/docs/ru/plugins-reference#plugin-installation-scopes) из основной копии, также загружаются в worktrees того же репозитория, поэтому вам не нужно переустанавливать их для каждого worktree. Это применяется независимо от того, создаёте ли вы worktree с помощью `--worktree` или с помощью `git worktree add`. Требуется Claude Code версии 2.1.200 или позже.

<Tip>
  Добавьте `.claude/worktrees/` в ваш `.gitignore`, чтобы содержимое worktree не отображалось как неотслеживаемые файлы в вашей основной копии.
</Tip>

<h3 id="choose-the-base-branch">
  Выберите базовую ветку
</h3>

Worktrees ветвятся от ветки репозитория по умолчанию, `origin/HEAD`, поэтому они начинаются с чистого дерева, соответствующего удалённому серверу. Когда ничего не загружало репозиторий в течение последних 24 часов, Claude Code обновляет `origin/HEAD` с помощью загрузки ветки по умолчанию, ограниченной пятью секундами, и использует локально кэшированный ref, если загрузка не удалась. Если удалённый сервер не настроен или `origin/HEAD` не кэширован локально и не может быть загружен, worktree переходит на вашу текущую локальную `HEAD`.

Обновление требует Claude Code версии 2.1.208 или позже; до этого новый worktree использовал то, что уже было кэшировано локально в `origin/HEAD`.

Чтобы всегда ветвиться от локальной `HEAD`, установите `worktree.baseRef` на `"head"` в [settings](/docs/ru/settings#worktree-settings). Установка `baseRef` на `"head"` делает новые worktrees содержащими ваши неотправленные коммиты и состояние ветки функции, что полезно при изоляции subagents, которым нужно работать с незавершённой работой. Когда сеанс работает внутри связанного worktree, `"head"` разрешается в `HEAD` этого worktree, а не в основной копии. Параметр принимает только `"fresh"` или `"head"`, а не произвольные git refs:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Чтобы ветвиться от конкретного pull request, передайте номер PR с префиксом `#` или полный URL GitHub pull request. Claude Code загружает `pull/<number>/head` из `origin` и создаёт worktree в `.claude/worktrees/pr-<number>`:

```bash theme={null}
claude --worktree "#1234"
```

Для полного контроля над тем, как создаются worktrees, настройте hook [`WorktreeCreate`](/docs/ru/hooks#worktreecreate), который полностью заменяет логику `git worktree` по умолчанию.

<h3 id="reuse-a-worktree-name">
  Повторное использование имени worktree
</h3>

Повторное использование имени worktree, каталог которого уже существует, возобновляет этот worktree.

Возобновленный worktree переходит к [текущей базе](#choose-the-base-branch) вместо возобновления на его старой вершине, когда выполняются все следующие условия:

* Он не имеет незафиксированных изменений или неотслеживаемых файлов.
* Он всё ещё находится на ветке, которую создал Claude Code.
* Он никогда не коммитил, или его pull request был объединён и его удалённая ветка удалена.

До версии 2.1.208 повторное использование имени всегда возобновляло старый worktree на его старой вершине.

<h2 id="copy-gitignored-files-into-worktrees">
  Скопируйте игнорируемые git файлы в worktrees
</h2>

Worktree — это свежая копия, поэтому неотслеживаемые файлы, такие как `.env` или `.env.local` из вашего основного репозитория, отсутствуют. Чтобы скопировать их автоматически при создании Claude worktree, добавьте файл `.worktreeinclude` в корень вашего проекта.

Файл использует синтаксис `.gitignore`. Копируются только файлы, которые соответствуют шаблону и также игнорируются git, поэтому отслеживаемые файлы никогда не дублируются.

Этот `.worktreeinclude` копирует два файла env и конфиг секретов в каждый новый worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Это применяется к worktrees, созданным с помощью `--worktree`, [worktrees subagent](#isolate-subagents-with-worktrees) и параллельным сеансам в [десктопном приложении](/docs/ru/desktop#work-in-parallel-with-sessions).

<h2 id="isolate-subagents-with-worktrees">
  Изолируйте subagents с помощью worktrees
</h2>

Subagents могут работать в собственных worktrees, чтобы параллельные правки не конфликтовали. Попросите Claude "использовать worktrees для ваших агентов" или установите это постоянно на [пользовательском subagent](/docs/ru/sub-agents#supported-frontmatter-fields), добавив `isolation: worktree` в frontmatter. Каждый subagent получает временный worktree, который автоматически удаляется, когда subagent завершает работу без изменений.

Worktrees subagents используют ту же [базовую ветку](#choose-the-base-branch), что и `--worktree`, поэтому они ветвятся от ветки по умолчанию вашего репозитория, если только `worktree.baseRef` не установлен на `"head"`.

<h2 id="clean-up-worktrees">
  Очистка worktrees
</h2>

Когда вы выходите из сеанса worktree, очистка зависит от того, внесли ли вы изменения:

* **Нет неотправленных изменений, нет неотслеживаемых файлов и нет новых коммитов**: worktree и его ветка удаляются автоматически. Если сеанс имеет [имя](/docs/ru/sessions#name-your-sessions), Claude вместо этого предлагает вам сохранить worktree на потом
* **Существуют неотправленные изменения, неотслеживаемые файлы или новые коммиты**: Claude предлагает вам сохранить или удалить worktree. Сохранение сохраняет каталог и ветку, чтобы вы могли вернуться позже. Удаление удаляет каталог worktree и его ветку, отбрасывая все неотправленные изменения, неотслеживаемые файлы и коммиты
* **Неинтерактивные запуски**: worktrees, созданные с помощью `--worktree` вместе с `-p`, не очищаются автоматически, так как нет подсказки выхода. Удалите их с помощью `git worktree remove`

Worktrees, которые Claude создал для subagents и [фоновых сеансов](/docs/ru/agent-view#how-file-edits-are-isolated), удаляются автоматически, как только они становятся старше вашего параметра [`cleanupPeriodDays`](/docs/ru/settings#available-settings), при условии, что они не имеют неотправленных изменений, неотслеживаемых файлов и неотправленных коммитов. Worktrees, которые вы создаёте с помощью `--worktree`, никогда не удаляются этой очисткой.

Пока агент работает, Claude запускает `git worktree lock` на его worktree, чтобы одновременная очистка не могла его удалить. Блокировка снимается, когда агент завершает работу. Чтобы очистить worktree, который очистка сохраняет, запустите `git worktree remove`, добавив `--force`, если worktree содержит неотправленные изменения или неотслеживаемые файлы.

В Windows перед удалением worktree Claude Code удаляет любое соединение NTFS или символическую ссылку на каталог на любой глубине внутри него как запись ссылки, чтобы удаление worktree не удаляло файлы, на которые указывает ссылка. До версии 2.1.205 Claude Code удалял только ссылки верхнего уровня как записи ссылок, и удаление worktree с соединением, вложенным в подкаталог, могло удалить содержимое каталога, на который указывала ссылка вне worktree.

<h2 id="manage-worktrees-manually">
  Управляйте worktrees вручную
</h2>

Для полного контроля над расположением worktree и конфигурацией ветки создавайте worktrees непосредственно с помощью Git. Это полезно, когда вам нужно проверить конкретную существующую ветку или поместить worktree вне репозитория.

Создайте worktree на новой ветке:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Создайте worktree из существующей ветки:

```bash theme={null}
git worktree add ../project-bugfix bugfix-123
```

Запустите Claude в worktree:

```bash theme={null}
cd ../project-feature-a && claude
```

Перечислите ваши worktrees:

```bash theme={null}
git worktree list
```

Удалите один, когда вы с ним закончите:

```bash theme={null}
git worktree remove ../project-feature-a
```

Полный справочник команд см. в [документации Git worktree](https://git-scm.com/docs/git-worktree). Помните, что нужно инициализировать вашу среду разработки в каждом новом worktree: установить зависимости, настроить виртуальные окружения или запустить всё, что требует настройка вашего проекта.

<h2 id="non-git-version-control">
  Не-git системы контроля версий
</h2>

Изоляция worktrees использует git по умолчанию. Для SVN, Perforce, Mercurial или других систем настройте hooks [`WorktreeCreate` и `WorktreeRemove`](/docs/ru/hooks#worktreecreate), чтобы предоставить пользовательскую логику создания и очистки. Поскольку hook заменяет поведение git по умолчанию, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) не обрабатывается при использовании `--worktree`. Скопируйте любые локальные файлы конфигурации внутри вашего скрипта hook вместо этого.

Этот hook `WorktreeCreate` читает имя worktree из stdin, проверяет свежую рабочую копию SVN и выводит путь каталога, чтобы Claude Code мог использовать его как рабочий каталог сеанса:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Объедините его с hook `WorktreeRemove` для очистки при завершении сеанса. Схему входных данных и пример удаления см. в [справочнике hooks](/docs/ru/hooks#worktreecreate).

<h2 id="see-also">
  См. также
</h2>

Worktrees обрабатывают изоляцию файлов. Связанные страницы ниже охватывают делегирование работы в эти изолированные копии и переключение между созданными вами сеансами:

* [Subagents](/docs/ru/sub-agents): делегируйте работу изолированным агентам внутри сеанса
* [Agent teams](/docs/ru/agent-teams): координируйте несколько сеансов Claude автоматически
* [Manage sessions](/docs/ru/sessions): назовите, возобновите и переключайтесь между беседами
* [Desktop parallel sessions](/docs/ru/desktop#work-in-parallel-with-sessions): сеансы на основе worktree в десктопном приложении
