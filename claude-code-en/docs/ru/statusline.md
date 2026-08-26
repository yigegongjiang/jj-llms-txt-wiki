> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Настройка строки состояния

> Настройте пользовательскую строку состояния для мониторинга использования контекстного окна, затрат и статуса git в Claude Code

Строка состояния — это настраиваемая панель в нижней части Claude Code, которая запускает любой скрипт оболочки, который вы настроите. Она получает данные сеанса в формате JSON через stdin и отображает всё, что выводит ваш скрипт, предоставляя вам постоянный, видимый с первого взгляда обзор использования контекста, затрат, статуса git или чего-либо ещё, что вы хотите отслеживать.

Строки состояния полезны, когда вы:

* Хотите отслеживать использование контекстного окна во время работы
* Нужно отслеживать затраты сеанса
* Работаете в нескольких сеансах и нужно их различать
* Хотите, чтобы ветка git и статус всегда были видны

Строка состояния отображается в собственной строке над встроенными значками нижнего колонтитула и не заменяет их. Чтобы добавить интерактивные значки ссылок в нижний колонтитул, когда в беседе появляется ID, без написания скрипта, вместо этого настройте [`footerLinksRegexes`](/docs/ru/settings#footer-link-badges).

Вот пример [многострочной строки состояния](#display-multiple-lines), которая отображает информацию git в первой строке и цветовую полосу контекста во второй.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Многострочная строка состояния, показывающая имя модели, каталог, ветку git в первой строке и полосу прогресса использования контекста с затратами и продолжительностью во второй строке" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

На этой странице описано [настройка базовой строки состояния](#set-up-a-status-line), объясняется [как данные передаются](#how-status-lines-work) из Claude Code в ваш скрипт, перечисляются [все поля, которые вы можете отображать](#available-data), и предоставляются [готовые примеры](#examples) для распространённых паттернов, таких как статус git, отслеживание затрат и полосы прогресса.

<h2 id="set-up-a-status-line">
  Настройка строки состояния
</h2>

Используйте [команду `/statusline`](#use-the-%2Fstatusline-command) для автоматического создания скрипта Claude Code, или [вручную создайте скрипт](#manually-configure-a-status-line) и добавьте его в ваши настройки.

<h3 id="use-the-/statusline-command">
  Использование команды /statusline
</h3>

Команда `/statusline` принимает инструкции на естественном языке, описывающие то, что вы хотите отображать. Claude Code генерирует файл скрипта в `~/.claude/` и автоматически обновляет ваши настройки:

```text theme={null}
/statusline show model name and context percentage with a progress bar
```

<h3 id="manually-configure-a-status-line">
  Ручная настройка строки состояния
</h3>

Добавьте поле `statusLine` в ваши пользовательские настройки (`~/.claude/settings.json`, где `~` — это ваш домашний каталог) или [настройки проекта](/docs/ru/settings#settings-files). Установите `type` на `"command"` и укажите `command` на путь скрипта или встроенную команду оболочки. Для полного пошагового руководства по созданию скрипта см. [Построение строки состояния пошагово](#build-a-status-line-step-by-step).

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline.sh",
    "padding": 2
  }
}
```

Поле `command` запускается в оболочке, поэтому вы также можете использовать встроенные команды вместо файла скрипта. Этот пример использует `jq` для разбора входных данных JSON и отображения имени модели и процента контекста:

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'"
  }
}
```

Необязательное поле `padding` добавляет дополнительное горизонтальное расстояние (в символах) к содержимому строки состояния. По умолчанию `0`. Это заполнение добавляется к встроенному расстоянию интерфейса, поэтому оно управляет относительным отступом, а не абсолютным расстоянием от края терминала.

Необязательное поле `refreshInterval` повторно запускает вашу команду каждые N секунд в дополнение к [обновлениям, управляемым событиями](#how-status-lines-work). Минимум — `1`. Установите это, когда ваша строка состояния отображает данные на основе времени, такие как часы, или когда фоновые подагенты изменяют состояние git, пока основной сеанс неактивен. Оставьте это не установленным, чтобы запускать только при событиях.

Необязательное поле `hideVimModeIndicator` подавляет встроенный текст `-- INSERT --` под приглашением. Установите это на `true`, когда ваш скрипт отображает [`vim.mode`](#available-data) самостоятельно, чтобы режим не отображался дважды.

<h3 id="disable-the-status-line">
  Отключение строки состояния
</h3>

Запустите `/statusline` и попросите её удалить или очистить вашу строку состояния (например, `/statusline delete`, `/statusline clear`, `/statusline remove it`). Вы также можете вручную удалить поле `statusLine` из вашего settings.json.

<h2 id="build-a-status-line-step-by-step">
  Построение строки состояния пошагово
</h2>

Это пошаговое руководство показывает, что происходит под капотом, путём ручного создания строки состояния, которая отображает текущую модель, рабочий каталог и процент использования контекстного окна.

<Note>Запуск [`/statusline`](#use-the-%2Fstatusline-command) с описанием того, что вы хотите, настраивает всё это автоматически.</Note>

Эти примеры используют скрипты Bash, которые работают на macOS и Linux. На Windows см. [Конфигурация Windows](#windows-configuration) для примеров PowerShell и Git Bash.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-quickstart.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=696445e59ca0059213250651ad23db6b" alt="Строка состояния, показывающая имя модели, каталог и процент контекста" width="726" height="164" data-path="images/statusline-quickstart.png" />
</Frame>

<Steps>
  <Step title="Создайте скрипт, который читает JSON и выводит результат">
    Claude Code отправляет данные JSON в ваш скрипт через stdin. Этот скрипт использует [`jq`](https://jqlang.github.io/jq/), парсер JSON командной строки, который вам может потребоваться установить, для извлечения имени модели, каталога и процента контекста, а затем выводит отформатированную строку.

    Сохраните это в `~/.claude/statusline.sh` (где `~` — это ваш домашний каталог, например `/Users/username` на macOS или `/home/username` на Linux):

    ```bash theme={null}
    #!/bin/bash
    # Read JSON data that Claude Code sends to stdin
    input=$(cat)

    # Extract fields using jq
    MODEL=$(echo "$input" | jq -r '.model.display_name')
    DIR=$(echo "$input" | jq -r '.workspace.current_dir')
    # The "// 0" provides a fallback if the field is null
    PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

    # Output the status line - ${DIR##*/} extracts just the folder name
    echo "[$MODEL] 📁 ${DIR##*/} | ${PCT}% context"
    ```
  </Step>

  <Step title="Сделайте его исполняемым">
    Отметьте скрипт как исполняемый, чтобы ваша оболочка могла его запустить:

    ```bash theme={null}
    chmod +x ~/.claude/statusline.sh
    ```
  </Step>

  <Step title="Добавьте в настройки">
    Скажите Claude Code запустить ваш скрипт как строку состояния. Добавьте эту конфигурацию в `~/.claude/settings.json`, которая устанавливает `type` на `"command"` (означает «запустить эту команду оболочки») и указывает `command` на ваш скрипт:

    ```json theme={null}
    {
      "statusLine": {
        "type": "command",
        "command": "~/.claude/statusline.sh"
      }
    }
    ```

    Ваша строка состояния появляется в нижней части интерфейса. Настройки перезагружаются автоматически, но изменения не появятся до вашего следующего взаимодействия с Claude Code.
  </Step>
</Steps>

<h2 id="how-status-lines-work">
  Как работают строки состояния
</h2>

Claude Code запускает ваш скрипт и передаёт [данные сеанса JSON](#available-data) в него через stdin. Ваш скрипт читает JSON, извлекает то, что ему нужно, и выводит текст в stdout. Claude Code отображает всё, что выводит ваш скрипт.

**Когда это обновляется**

Ваш скрипт запускается после каждого нового сообщения ассистента, после завершения `/compact`, когда изменяется режим разрешений или когда переключается режим vim. Обновления дебаунсятся на 300 мс, что означает, что быстрые изменения объединяются вместе и ваш скрипт запускается один раз, когда всё стабилизируется. Если новое обновление срабатывает, пока ваш скрипт всё ещё работает, выполнение в полёте отменяется. Если вы отредактируете свой скрипт, изменения не появятся до вашего следующего взаимодействия с Claude Code, которое срабатывает обновление.

Эти триггеры могут затихнуть, когда основной сеанс неактивен, например, пока координатор ждёт фоновых подагентов. Чтобы сохранить текущими сегменты на основе времени или из внешних источников во время периодов неактивности, установите [`refreshInterval`](#manually-configure-a-status-line) для также повторного запуска команды на фиксированный таймер.

**Что может выводить ваш скрипт**

* **Несколько строк**: каждый оператор `echo` или `print` отображается как отдельная строка. См. [пример многострочности](#display-multiple-lines).
* **Цвета**: используйте [коды экранирования ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors), такие как `\033[32m` для зелёного (терминал должен их поддерживать). См. [пример статуса git](#git-status-with-colors).
* **Ссылки**: используйте [последовательности экранирования OSC 8](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC) для создания кликабельного текста (Cmd+клик на macOS, Ctrl+клик на Windows/Linux). Требуется терминал, поддерживающий гиперссылки, такой как iTerm2, Kitty или WezTerm. См. [пример кликабельных ссылок](#clickable-links).

**Размер вывода в соответствии с терминалом**

Claude Code захватывает вывод вашего скрипта вместо прямого подключения к терминалу, поэтому `tput cols` и определение ширины на уровне языка не могут прочитать размер терминала изнутри скрипта. Вместо этого читайте переменные окружения `COLUMNS` и `LINES`. Claude Code устанавливает эти значения в соответствии с текущими размерами терминала перед запуском вашего скрипта. Требуется Claude Code v2.1.153 или позже.

<Note>Строка состояния работает локально и не потребляет токены API. Она временно скрывается во время определённых взаимодействий с пользовательским интерфейсом, включая предложения автодополнения, меню справки и запросы разрешений.</Note>

<h2 id="available-data">
  Доступные данные
</h2>

Claude Code отправляет следующие поля JSON в ваш скрипт через stdin:

| Поле                                                                             | Описание                                                                                                                                                                                                                                                                                        |
| -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model.id`, `model.display_name`                                                 | Текущий идентификатор модели и отображаемое имя                                                                                                                                                                                                                                                 |
| `cwd`, `workspace.current_dir`                                                   | Текущий рабочий каталог. Оба поля содержат одно и то же значение; `workspace.current_dir` предпочтительнее для согласованности с `workspace.project_dir`.                                                                                                                                       |
| `workspace.project_dir`                                                          | Каталог, в котором был запущен Claude Code, который может отличаться от `cwd`, если рабочий каталог изменяется во время сеанса                                                                                                                                                                  |
| `workspace.added_dirs`                                                           | Дополнительные каталоги, добавленные через `/add-dir` или `--add-dir`. Пустой массив, если ничего не было добавлено                                                                                                                                                                             |
| `workspace.git_worktree`                                                         | Имя Git worktree, когда текущий каталог находится внутри связанного worktree, созданного с помощью `git worktree add`. Отсутствует в основном рабочем дереве. Заполняется для любого git worktree, в отличие от `worktree.*`, который применяется только к сеансам `--worktree`                 |
| `workspace.repo.host`, `workspace.repo.owner`, `workspace.repo.name`             | Идентификация репозитория, проанализированная из удалённого хранилища `origin`, например `"github.com"`, `"anthropics"`, `"claude-code"`. Отсутствует вне git репозитория или когда удалённое хранилище `origin` не настроено                                                                   |
| `cost.total_cost_usd`                                                            | Предполагаемая стоимость сеанса в USD, вычисляется на клиенте. Может отличаться от вашего фактического счёта                                                                                                                                                                                    |
| `cost.total_duration_ms`                                                         | Общее реальное время с момента начала сеанса в миллисекундах                                                                                                                                                                                                                                    |
| `cost.total_api_duration_ms`                                                     | Общее время, потраченное на ожидание ответов API в миллисекундах                                                                                                                                                                                                                                |
| `cost.total_lines_added`, `cost.total_lines_removed`                             | Строки кода, которые были изменены                                                                                                                                                                                                                                                              |
| `context_window.total_input_tokens`, `context_window.total_output_tokens`        | Количество токенов, находящихся в контекстном окне, из последнего ответа API. Входные данные включают чтение и запись кэша. До версии 2.1.132 это были совокупные итоги сеанса                                                                                                                  |
| `context_window.context_window_size`                                             | Максимальный размер контекстного окна в токенах. По умолчанию 200000 или 1000000 для моделей с расширенным контекстом.                                                                                                                                                                          |
| `context_window.used_percentage`                                                 | Предварительно рассчитанный процент использованного контекстного окна                                                                                                                                                                                                                           |
| `context_window.remaining_percentage`                                            | Предварительно рассчитанный процент оставшегося контекстного окна                                                                                                                                                                                                                               |
| `context_window.current_usage`                                                   | Подсчёты токенов из последнего вызова API, описанные в [полях контекстного окна](#context-window-fields)                                                                                                                                                                                        |
| `exceeds_200k_tokens`                                                            | Превышает ли общее количество токенов (входные, кэшированные и выходные токены в сумме) из последнего ответа API 200k. Это фиксированный порог независимо от фактического размера контекстного окна.                                                                                            |
| `effort.level`                                                                   | Текущий уровень рассуждений (`low`, `medium`, `high`, `xhigh` или `max`). Отражает текущее значение сеанса, включая изменения `/effort` во время сеанса. Ultracode не является отдельным уровнем и сообщается как `xhigh`. Отсутствует, когда текущая модель не поддерживает параметр effort    |
| `thinking.enabled`                                                               | Включено ли расширенное мышление для сеанса                                                                                                                                                                                                                                                     |
| `rate_limits.five_hour.used_percentage`, `rate_limits.seven_day.used_percentage` | Процент лимита скорости за 5 часов или 7 дней, потреблённый от 0 до 100                                                                                                                                                                                                                         |
| `rate_limits.five_hour.resets_at`, `rate_limits.seven_day.resets_at`             | Секунды эпохи Unix, когда окно лимита скорости за 5 часов или 7 дней сбрасывается                                                                                                                                                                                                               |
| `session_id`                                                                     | Уникальный идентификатор сеанса                                                                                                                                                                                                                                                                 |
| `session_name`                                                                   | Пользовательское имя сеанса, установленное с флагом `--name` или `/rename`. Отсутствует, если пользовательское имя не было установлено                                                                                                                                                          |
| `prompt_id`                                                                      | UUID, идентифицирующий пользовательский запрос, который в настоящее время обрабатывается. Совпадает с атрибутом [`prompt.id` на событиях OpenTelemetry](/docs/ru/monitoring-usage#event-correlation-attributes). Отсутствует до первого ввода пользователя. Требуется Claude Code v2.1.196 или позже |
| `transcript_path`                                                                | Путь к файлу стенограммы разговора                                                                                                                                                                                                                                                              |
| `version`                                                                        | Версия Claude Code                                                                                                                                                                                                                                                                              |
| `output_style.name`                                                              | Имя текущего стиля вывода                                                                                                                                                                                                                                                                       |
| `vim.mode`                                                                       | Текущий режим vim (`NORMAL`, `INSERT`, `VISUAL` или `VISUAL LINE`), когда [режим vim](/docs/ru/interactive-mode#vim-editor-mode) включен                                                                                                                                                             |
| `agent.name`                                                                     | Имя агента при запуске с флагом `--agent` или настроенными параметрами агента                                                                                                                                                                                                                   |
| `pr.number`, `pr.url`                                                            | Открытый pull request для текущей ветки. Отражает значок PR в нижней строке состояния. Отсутствует до тех пор, пока PR не будет найден, когда не в git репозитории, или после того как PR будет объединён или закрыт                                                                            |
| `pr.review_state`                                                                | Статус проверки открытого PR: `approved`, `pending`, `changes_requested` или `draft`. Может быть независимо отсутствующим даже когда `pr` присутствует                                                                                                                                          |
| `worktree.name`                                                                  | Имя активного worktree. Присутствует только во время сеансов `--worktree`                                                                                                                                                                                                                       |
| `worktree.path`                                                                  | Абсолютный путь к каталогу worktree                                                                                                                                                                                                                                                             |
| `worktree.branch`                                                                | Имя ветки Git для worktree (например, `"worktree-my-feature"`). Отсутствует для worktrees на основе hooks                                                                                                                                                                                       |
| `worktree.original_cwd`                                                          | Каталог, в котором находился Claude перед входом в worktree                                                                                                                                                                                                                                     |
| `worktree.original_branch`                                                       | Ветка Git, проверенная перед входом в worktree. Отсутствует для worktrees на основе hooks                                                                                                                                                                                                       |

<Accordion title="Полная схема JSON">
  Ваша команда строки состояния получает эту структуру JSON через stdin:

  ```json theme={null}
  {
    "cwd": "/current/working/directory",
    "session_id": "abc123...",
    "session_name": "my-session",
    "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
    "transcript_path": "/path/to/transcript.jsonl",
    "model": {
      "id": "claude-opus-4-8",
      "display_name": "Opus"
    },
    "workspace": {
      "current_dir": "/current/working/directory",
      "project_dir": "/original/project/directory",
      "added_dirs": [],
      "git_worktree": "feature-xyz",
      "repo": {
        "host": "github.com",
        "owner": "anthropics",
        "name": "claude-code"
      }
    },
    "version": "2.1.90",
    "output_style": {
      "name": "default"
    },
    "cost": {
      "total_cost_usd": 0.01234,
      "total_duration_ms": 45000,
      "total_api_duration_ms": 2300,
      "total_lines_added": 156,
      "total_lines_removed": 23
    },
    "context_window": {
      "total_input_tokens": 15500,
      "total_output_tokens": 1200,
      "context_window_size": 200000,
      "used_percentage": 8,
      "remaining_percentage": 92,
      "current_usage": {
        "input_tokens": 8500,
        "output_tokens": 1200,
        "cache_creation_input_tokens": 5000,
        "cache_read_input_tokens": 2000
      }
    },
    "exceeds_200k_tokens": false,
    "effort": {
      "level": "high"
    },
    "thinking": {
      "enabled": true
    },
    "rate_limits": {
      "five_hour": {
        "used_percentage": 23.5,
        "resets_at": 1738425600
      },
      "seven_day": {
        "used_percentage": 41.2,
        "resets_at": 1738857600
      }
    },
    "vim": {
      "mode": "NORMAL"
    },
    "agent": {
      "name": "security-reviewer"
    },
    "pr": {
      "number": 1234,
      "url": "https://github.com/anthropics/claude-code/pull/1234",
      "review_state": "pending"
    },
    "worktree": {
      "name": "my-feature",
      "path": "/path/to/.claude/worktrees/my-feature",
      "branch": "worktree-my-feature",
      "original_cwd": "/path/to/project",
      "original_branch": "main"
    }
  }
  ```

  **Поля, которые могут отсутствовать** (не присутствуют в JSON):

  * `session_name`: появляется только когда пользовательское имя было установлено с `--name` или `/rename`
  * `prompt_id`: появляется только после первого ввода пользователя
  * `workspace.git_worktree`: появляется только когда текущий каталог находится внутри связанного git worktree
  * `workspace.repo`: появляется только внутри git репозитория с настроенным удалённым хранилищем `origin`
  * `effort`: появляется только когда текущая модель поддерживает параметр reasoning effort
  * `vim`: появляется только когда режим vim включен
  * `agent`: появляется только при запуске с флагом `--agent` или настроенными параметрами агента
  * `pr`: появляется только пока открытый PR найден для текущей ветки, и удаляется после того как PR будет объединён или закрыт. `pr.review_state` может быть независимо отсутствующим
  * `worktree`: появляется только во время сеансов `--worktree`. Когда присутствует, `branch` и `original_branch` также могут отсутствовать для worktrees на основе hooks
  * `rate_limits`: появляется только для подписчиков Claude.ai (Pro/Max) после первого ответа API в сеансе. Каждое окно (`five_hour`, `seven_day`) может быть независимо отсутствующим. Используйте `jq -r '.rate_limits.five_hour.used_percentage // empty'` для корректной обработки отсутствия.

  **Поля, которые могут быть `null`**:

  * `context_window.current_usage`: `null` перед первым вызовом API в сеансе и снова после `/compact` до следующего вызова API, который его переполнит
  * `context_window.used_percentage`, `context_window.remaining_percentage`: могут быть `null` в начале сеанса

  Обрабатывайте отсутствующие поля с условным доступом и нулевые значения с резервными значениями по умолчанию в ваших скриптах.
</Accordion>

<h3 id="context-window-fields">
  Поля контекстного окна
</h3>

Объект `context_window` описывает живое контекстное окно из последнего ответа API. Начиная с версии 2.1.132, `total_input_tokens` и `total_output_tokens` отражают текущее использование контекста, а не совокупные итоги сеанса.

* **Совокупные итоги** (`total_input_tokens`, `total_output_tokens`): токены, находящиеся в контекстном окне. `total_input_tokens` — это сумма `input_tokens`, `cache_creation_input_tokens` и `cache_read_input_tokens`; `total_output_tokens` — это выходные токены из последнего ответа. Оба равны `0` перед первым ответом API.
* **Использование по компонентам** (`current_usage`): те же подсчёты токенов, разбитые по категориям. Используйте это, когда вам нужны попадания в кэш отдельно от свежего входа.

Объект `current_usage` содержит:

* `input_tokens`: входные токены в текущем контексте
* `output_tokens`: выходные токены, которые были сгенерированы
* `cache_creation_input_tokens`: токены, записанные в кэш
* `cache_read_input_tokens`: токены, прочитанные из кэша

Для получения информации о том, что означают поля кэша и как они выставляются счётом, см. [проверка производительности кэша](/docs/ru/prompt-caching#check-cache-performance).

Поле `used_percentage` рассчитывается только из входных токенов: `input_tokens + cache_creation_input_tokens + cache_read_input_tokens`. Оно не включает `output_tokens`.

Если вы рассчитываете процент контекста вручную из `current_usage`, используйте ту же формулу только для входных данных, чтобы соответствовать `used_percentage`.

Объект `current_usage` равен `null` перед первым вызовом API в сеансе и снова сразу после `/compact` до следующего вызова API, который его переполнит.

<h2 id="examples">
  Примеры
</h2>

Эти примеры показывают распространённые паттерны строк состояния. Чтобы использовать любой пример:

1. Сохраните скрипт в файл, например `~/.claude/statusline.sh` (или `.py`/`.js`)
2. Сделайте его исполняемым: `chmod +x ~/.claude/statusline.sh`
3. Добавьте путь в ваши [настройки](#manually-configure-a-status-line)

Примеры Bash используют [`jq`](https://jqlang.github.io/jq/) для разбора JSON. Python и Node.js имеют встроенный разбор JSON.

<h3 id="context-window-usage">
  Использование контекстного окна
</h3>

Отобразите текущую модель и использование контекстного окна с визуальной полосой прогресса. Каждый скрипт читает JSON из stdin, извлекает поле `used_percentage` и строит 10-символьную полосу, где заполненные блоки (▓) представляют использование:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-context-window-usage.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=15b58ab3602f036939145dde3165c6f7" alt="Строка состояния, показывающая имя модели и полосу прогресса с процентом" width="448" height="152" data-path="images/statusline-context-window-usage.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  # Read all of stdin into a variable
  input=$(cat)

  # Extract fields with jq, "// 0" provides fallback for null
  MODEL=$(echo "$input" | jq -r '.model.display_name')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

  # Build progress bar: printf -v creates a run of spaces, then
  # ${var// /▓} replaces each space with a block character
  BAR_WIDTH=10
  FILLED=$((PCT * BAR_WIDTH / 100))
  EMPTY=$((BAR_WIDTH - FILLED))
  BAR=""
  [ "$FILLED" -gt 0 ] && printf -v FILL "%${FILLED}s" && BAR="${FILL// /▓}"
  [ "$EMPTY" -gt 0 ] && printf -v PAD "%${EMPTY}s" && BAR="${BAR}${PAD// /░}"

  echo "[$MODEL] $BAR $PCT%"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  # json.load reads and parses stdin in one step
  data = json.load(sys.stdin)
  model = data['model']['display_name']
  # "or 0" handles null values
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)

  # String multiplication builds the bar
  filled = pct * 10 // 100
  bar = '▓' * filled + '░' * (10 - filled)

  print(f"[{model}] {bar} {pct}%")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  // Node.js reads stdin asynchronously with events
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      // Optional chaining (?.) safely handles null fields
      const pct = Math.floor(data.context_window?.used_percentage || 0);

      // String.repeat() builds the bar
      const filled = Math.floor(pct * 10 / 100);
      const bar = '▓'.repeat(filled) + '░'.repeat(10 - filled);

      console.log(`[${model}] ${bar} ${pct}%`);
  });
  ```
</CodeGroup>

<h3 id="git-status-with-colors">
  Статус git с цветами
</h3>

Показывает ветку git с цветовыми индикаторами для подготовленных и изменённых файлов. Этот скрипт использует [коды экранирования ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) для цветов терминала: `\033[32m` — зелёный, `\033[33m` — жёлтый и `\033[0m` — сброс на значение по умолчанию.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-git-context.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e656f34f90d1d9a1d0e220988914345f" alt="Строка состояния, показывающая модель, каталог, ветку git и цветные индикаторы для подготовленных и изменённых файлов" width="742" height="178" data-path="images/statusline-git-context.png" />
</Frame>

Каждый скрипт проверяет, является ли текущий каталог репозиторием git, подсчитывает подготовленные и изменённые файлы и отображает цветные индикаторы:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')

  GREEN='\033[32m'
  YELLOW='\033[33m'
  RESET='\033[0m'

  if git rev-parse --git-dir > /dev/null 2>&1; then
      BRANCH=$(git branch --show-current 2>/dev/null)
      STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
      MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')

      GIT_STATUS=""
      [ "$STAGED" -gt 0 ] && GIT_STATUS="${GREEN}+${STAGED}${RESET}"
      [ "$MODIFIED" -gt 0 ] && GIT_STATUS="${GIT_STATUS}${YELLOW}~${MODIFIED}${RESET}"

      echo -e "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH $GIT_STATUS"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])

  GREEN, YELLOW, RESET = '\033[32m', '\033[33m', '\033[0m'

  try:
      subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
      staged_output = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
      modified_output = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
      staged = len(staged_output.split('\n')) if staged_output else 0
      modified = len(modified_output.split('\n')) if modified_output else 0

      git_status = f"{GREEN}+{staged}{RESET}" if staged else ""
      git_status += f"{YELLOW}~{modified}{RESET}" if modified else ""

      print(f"[{model}] 📁 {directory} | 🌿 {branch} {git_status}")
  except:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);

      const GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RESET = '\x1b[0m';

      try {
          execSync('git rev-parse --git-dir', { stdio: 'ignore' });
          const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
          const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
          const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;

          let gitStatus = staged ? `${GREEN}+${staged}${RESET}` : '';
          gitStatus += modified ? `${YELLOW}~${modified}${RESET}` : '';

          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} ${gitStatus}`);
      } catch {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="cost-and-duration-tracking">
  Отслеживание затрат и продолжительности
</h3>

Отслеживайте затраты API вашего сеанса и прошедшее время. Поле `cost.total_cost_usd` накапливает стоимость всех вызовов API в текущем сеансе. Поле `cost.total_duration_ms` измеряет общее прошедшее время с момента начала сеанса, а `cost.total_api_duration_ms` отслеживает только время, потраченное на ожидание ответов API.

Каждый скрипт форматирует стоимость как валюту и преобразует миллисекунды в минуты и секунды:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-cost-tracking.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e3444a51fe6f3440c134bd5f1f08ad29" alt="Строка состояния, показывающая имя модели, стоимость сеанса и продолжительность" width="588" height="180" data-path="images/statusline-cost-tracking.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  COST_FMT=$(printf '$%.2f' "$COST")
  DURATION_SEC=$((DURATION_MS / 1000))
  MINS=$((DURATION_SEC / 60))
  SECS=$((DURATION_SEC % 60))

  echo "[$MODEL] 💰 $COST_FMT | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  duration_sec = duration_ms // 1000
  mins, secs = duration_sec // 60, duration_sec % 60

  print(f"[{model}] 💰 ${cost:.2f} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const cost = data.cost?.total_cost_usd || 0;
      const durationMs = data.cost?.total_duration_ms || 0;

      const durationSec = Math.floor(durationMs / 1000);
      const mins = Math.floor(durationSec / 60);
      const secs = durationSec % 60;

      console.log(`[${model}] 💰 $${cost.toFixed(2)} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="display-multiple-lines">
  Отображение нескольких строк
</h3>

Ваш скрипт может выводить несколько строк для создания более богатого отображения. Каждый оператор `echo` создаёт отдельную строку в области состояния.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Многострочная строка состояния, показывающая имя модели, каталог, ветку git в первой строке и полосу прогресса использования контекста с затратами и продолжительностью во второй строке" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Этот пример объединяет несколько методов: цвета на основе порогов (зелёный ниже 70%, жёлтый 70-89%, красный 90%+), полосу прогресса и информацию о ветке git. Каждый оператор `print` или `echo` создаёт отдельную строку:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; RESET='\033[0m'

  # Pick bar color based on context usage
  if [ "$PCT" -ge 90 ]; then BAR_COLOR="$RED"
  elif [ "$PCT" -ge 70 ]; then BAR_COLOR="$YELLOW"
  else BAR_COLOR="$GREEN"; fi

  FILLED=$((PCT / 10)); EMPTY=$((10 - FILLED))
  printf -v FILL "%${FILLED}s"; printf -v PAD "%${EMPTY}s"
  BAR="${FILL// /█}${PAD// /░}"

  MINS=$((DURATION_MS / 60000)); SECS=$(((DURATION_MS % 60000) / 1000))

  BRANCH=""
  git rev-parse --git-dir > /dev/null 2>&1 && BRANCH=" | 🌿 $(git branch --show-current 2>/dev/null)"

  echo -e "${CYAN}[$MODEL]${RESET} 📁 ${DIR##*/}$BRANCH"
  COST_FMT=$(printf '$%.2f' "$COST")
  echo -e "${BAR_COLOR}${BAR}${RESET} ${PCT}% | ${YELLOW}${COST_FMT}${RESET} | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  CYAN, GREEN, YELLOW, RED, RESET = '\033[36m', '\033[32m', '\033[33m', '\033[31m', '\033[0m'

  bar_color = RED if pct >= 90 else YELLOW if pct >= 70 else GREEN
  filled = pct // 10
  bar = '█' * filled + '░' * (10 - filled)

  mins, secs = duration_ms // 60000, (duration_ms % 60000) // 1000

  try:
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True, stderr=subprocess.DEVNULL).strip()
      branch = f" | 🌿 {branch}" if branch else ""
  except:
      branch = ""

  print(f"{CYAN}[{model}]{RESET} 📁 {directory}{branch}")
  print(f"{bar_color}{bar}{RESET} {pct}% | {YELLOW}${cost:.2f}{RESET} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const cost = data.cost?.total_cost_usd || 0;
      const pct = Math.floor(data.context_window?.used_percentage || 0);
      const durationMs = data.cost?.total_duration_ms || 0;

      const CYAN = '\x1b[36m', GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RED = '\x1b[31m', RESET = '\x1b[0m';

      const barColor = pct >= 90 ? RED : pct >= 70 ? YELLOW : GREEN;
      const filled = Math.floor(pct / 10);
      const bar = '█'.repeat(filled) + '░'.repeat(10 - filled);

      const mins = Math.floor(durationMs / 60000);
      const secs = Math.floor((durationMs % 60000) / 1000);

      let branch = '';
      try {
          branch = execSync('git branch --show-current', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          branch = branch ? ` | 🌿 ${branch}` : '';
      } catch {}

      console.log(`${CYAN}[${model}]${RESET} 📁 ${dir}${branch}`);
      console.log(`${barColor}${bar}${RESET} ${pct}% | ${YELLOW}$${cost.toFixed(2)}${RESET} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="clickable-links">
  Кликабельные ссылки
</h3>

Этот пример создаёт кликабельную ссылку на ваш репозиторий GitHub. Он читает URL удалённого репозитория, преобразует формат SSH в HTTPS с помощью `sed` и оборачивает имя репозитория в коды экранирования OSC 8. Удерживайте Cmd (macOS) или Ctrl (Windows/Linux) и нажмите, чтобы открыть ссылку в браузере.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-links.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=4bcc6e7deb7cf52f41ab85a219b52661" alt="Строка состояния, показывающая кликабельную ссылку на репозиторий GitHub" width="726" height="198" data-path="images/statusline-links.png" />
</Frame>

Каждый скрипт получает URL удалённого репозитория, преобразует формат SSH в HTTPS и оборачивает имя репозитория в коды экранирования OSC 8. Версия Bash использует `printf '%b'`, которая более надёжно интерпретирует экранирующие последовательности, чем `echo -e` в разных оболочках:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')

  # Convert git SSH URL to HTTPS
  REMOTE=$(git remote get-url origin 2>/dev/null | sed 's/git@github.com:/https:\/\/github.com\//' | sed 's/\.git$//')

  if [ -n "$REMOTE" ]; then
      REPO_NAME=$(basename "$REMOTE")
      # OSC 8 format: \e]8;;URL\a then TEXT then \e]8;;\a
      # printf %b interprets escape sequences reliably across shells
      printf '%b' "[$MODEL] 🔗 \e]8;;${REMOTE}\a${REPO_NAME}\e]8;;\a\n"
  else
      echo "[$MODEL]"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, re, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  # Get git remote URL
  try:
      remote = subprocess.check_output(
          ['git', 'remote', 'get-url', 'origin'],
          stderr=subprocess.DEVNULL, text=True
      ).strip()
      # Convert SSH to HTTPS format
      remote = re.sub(r'^git@github\.com:', 'https://github.com/', remote)
      remote = re.sub(r'\.git$', '', remote)
      repo_name = os.path.basename(remote)
      # OSC 8 escape sequences
      link = f"\033]8;;{remote}\a{repo_name}\033]8;;\a"
      print(f"[{model}] 🔗 {link}")
  except:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      try {
          let remote = execSync('git remote get-url origin', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          // Convert SSH to HTTPS format
          remote = remote.replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '');
          const repoName = path.basename(remote);
          // OSC 8 escape sequences
          const link = `\x1b]8;;${remote}\x07${repoName}\x1b]8;;\x07`;
          console.log(`[${model}] 🔗 ${link}`);
      } catch {
          console.log(`[${model}]`);
      }
  });
  ```
</CodeGroup>

<h3 id="rate-limit-usage">
  Использование лимита скорости
</h3>

Отобразите использование лимита скорости подписки Claude.ai в строке состояния. Объект `rate_limits` содержит `five_hour` (5-часовое скользящее окно) и `seven_day` (еженедельное) окна. Каждое окно предоставляет `used_percentage` (0-100) и `resets_at` (секунды эпохи Unix, когда окно сбрасывается).

Это поле присутствует только для подписчиков Claude.ai (Pro/Max) после первого ответа API. Каждый скрипт корректно обрабатывает отсутствующее поле:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  # "// empty" produces no output when rate_limits is absent
  FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
  WEEK=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')

  LIMITS=""
  [ -n "$FIVE_H" ] && LIMITS="5h: $(printf '%.0f' "$FIVE_H")%"
  [ -n "$WEEK" ] && LIMITS="${LIMITS:+$LIMITS }7d: $(printf '%.0f' "$WEEK")%"

  [ -n "$LIMITS" ] && echo "[$MODEL] | $LIMITS" || echo "[$MODEL]"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  parts = []
  rate = data.get('rate_limits', {})
  five_h = rate.get('five_hour', {}).get('used_percentage')
  week = rate.get('seven_day', {}).get('used_percentage')

  if five_h is not None:
      parts.append(f"5h: {five_h:.0f}%")
  if week is not None:
      parts.append(f"7d: {week:.0f}%")

  if parts:
      print(f"[{model}] | {' '.join(parts)}")
  else:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      const parts = [];
      const fiveH = data.rate_limits?.five_hour?.used_percentage;
      const week = data.rate_limits?.seven_day?.used_percentage;

      if (fiveH != null) parts.push(`5h: ${Math.round(fiveH)}%`);
      if (week != null) parts.push(`7d: ${Math.round(week)}%`);

      console.log(parts.length ? `[${model}] | ${parts.join(' ')}` : `[${model}]`);
  });
  ```
</CodeGroup>

<h3 id="cache-expensive-operations">
  Кэширование дорогостоящих операций
</h3>

Ваш скрипт строки состояния запускается часто во время активных сеансов. Команды, такие как `git status` или `git diff`, могут быть медленными, особенно в больших репозиториях. Этот пример кэширует информацию git во временный файл и обновляет её только каждые 5 секунд.

Имя файла кэша должно быть стабильным во время вызовов строки состояния в рамках сеанса, но уникальным для разных сеансов, чтобы одновременные сеансы в разных репозиториях не читали состояние git друг друга. Идентификаторы на основе процесса, такие как `$$`, `os.getpid()` или `process.pid`, изменяются при каждом вызове и нарушают кэш. Вместо этого используйте `session_id` из входных данных JSON: он стабилен в течение жизни сеанса и уникален для каждого сеанса.

Каждый скрипт проверяет, отсутствует ли файл кэша или он старше 5 секунд, перед запуском команд git:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  SESSION_ID=$(echo "$input" | jq -r '.session_id')

  CACHE_FILE="/tmp/statusline-git-cache-$SESSION_ID"
  CACHE_MAX_AGE=5  # seconds

  cache_is_stale() {
      [ ! -f "$CACHE_FILE" ] || \
      # stat -f %m is macOS, stat -c %Y is Linux
      [ $(($(date +%s) - $(stat -f %m "$CACHE_FILE" 2>/dev/null || stat -c %Y "$CACHE_FILE" 2>/dev/null || echo 0))) -gt $CACHE_MAX_AGE ]
  }

  if cache_is_stale; then
      if git rev-parse --git-dir > /dev/null 2>&1; then
          BRANCH=$(git branch --show-current 2>/dev/null)
          STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
          MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')
          echo "$BRANCH|$STAGED|$MODIFIED" > "$CACHE_FILE"
      else
          echo "||" > "$CACHE_FILE"
      fi
  fi

  IFS='|' read -r BRANCH STAGED MODIFIED < "$CACHE_FILE"

  if [ -n "$BRANCH" ]; then
      echo "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH +$STAGED ~$MODIFIED"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os, time

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  session_id = data['session_id']

  CACHE_FILE = f"/tmp/statusline-git-cache-{session_id}"
  CACHE_MAX_AGE = 5  # seconds

  def cache_is_stale():
      if not os.path.exists(CACHE_FILE):
          return True
      return time.time() - os.path.getmtime(CACHE_FILE) > CACHE_MAX_AGE

  if cache_is_stale():
      try:
          subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
          branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
          staged = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
          modified = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
          staged_count = len(staged.split('\n')) if staged else 0
          modified_count = len(modified.split('\n')) if modified else 0
          with open(CACHE_FILE, 'w') as f:
              f.write(f"{branch}|{staged_count}|{modified_count}")
      except:
          with open(CACHE_FILE, 'w') as f:
              f.write("||")

  with open(CACHE_FILE) as f:
      branch, staged, modified = f.read().strip().split('|')

  if branch:
      print(f"[{model}] 📁 {directory} | 🌿 {branch} +{staged} ~{modified}")
  else:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const fs = require('fs');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const sessionId = data.session_id;

      const CACHE_FILE = `/tmp/statusline-git-cache-${sessionId}`;
      const CACHE_MAX_AGE = 5; // seconds

      const cacheIsStale = () => {
          if (!fs.existsSync(CACHE_FILE)) return true;
          return (Date.now() / 1000) - fs.statSync(CACHE_FILE).mtimeMs / 1000 > CACHE_MAX_AGE;
      };

      if (cacheIsStale()) {
          try {
              execSync('git rev-parse --git-dir', { stdio: 'ignore' });
              const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
              const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              fs.writeFileSync(CACHE_FILE, `${branch}|${staged}|${modified}`);
          } catch {
              fs.writeFileSync(CACHE_FILE, '||');
          }
      }

      const [branch, staged, modified] = fs.readFileSync(CACHE_FILE, 'utf8').trim().split('|');

      if (branch) {
          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} +${staged} ~${modified}`);
      } else {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="windows-configuration">
  Конфигурация Windows
</h3>

На Windows Claude Code запускает команды строки состояния через Git Bash, когда Git Bash установлен, или через PowerShell, когда Git Bash отсутствует.

Git Bash обрабатывает неэкранированные обратные слэши как символы экранирования, поэтому путь в стиле Windows, такой как `C:\Users\username\script.mjs`, достигает средства запуска скрипта с удалёнными разделителями и команда не выполняется без видимой ошибки. Записывайте пути к файлам в строке `command` с прямыми слэшами, как показано в примерах ниже. Сокращение `~` также работает и расширяется до вашего домашнего каталога Windows.

Чтобы запустить скрипт PowerShell как вашу строку состояния, вызовите его через `powershell`. Это работает независимо от того, маршрутизирует ли Claude Code команду через Git Bash или PowerShell:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "powershell -NoProfile -File C:/Users/username/.claude/statusline.ps1"
    }
  }
  ```

  ```powershell statusline.ps1 theme={null}
  $input_json = $input | Out-String | ConvertFrom-Json
  $cwd = $input_json.cwd
  $model = $input_json.model.display_name
  $used = $input_json.context_window.used_percentage
  $dirname = Split-Path $cwd -Leaf

  if ($used) {
      Write-Host "$dirname [$model] ctx: $used%"
  } else {
      Write-Host "$dirname [$model]"
  }
  ```
</CodeGroup>

Или запустите скрипт Bash напрямую, когда Git Bash установлен:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "~/.claude/statusline.sh"
    }
  }
  ```

  ```bash statusline.sh theme={null}
  #!/usr/bin/env bash
  input=$(cat)
  cwd=$(echo "$input" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
  model=$(echo "$input" | grep -o '"display_name":"[^"]*"' | cut -d'"' -f4)
  dirname="${cwd##*[/\\]}"
  echo "$dirname [$model]"
  ```
</CodeGroup>

<h2 id="subagent-status-lines">
  Строки состояния подагентов
</h2>

Параметр `subagentStatusLine` отображает пользовательское тело строки для каждого [подагента](/docs/ru/sub-agents), показанного на панели агентов ниже приглашения. Используйте его для замены строки по умолчанию `name · description · token count` на ваше собственное форматирование.

```json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "~/.claude/subagent-statusline.sh"
  }
}
```

Команда запускается один раз за цикл обновления со всеми видимыми строками подагентов, переданными как один объект JSON на stdin. Входные данные включают [базовые поля hooks](/docs/ru/hooks#common-input-fields), поле `columns` с используемой шириной строки и массив `tasks`. Каждая задача имеет `id`, `name`, `type`, `status`, `description`, `label`, `startTime`, `model`, `contextWindowSize`, `tokenCount`, `tokenSamples` и `cwd`.

Поле `model` для каждой задачи — это разрешённый ID модели, на которой выполняется задача. `contextWindowSize` — это контекстное окно этой модели в токенах, вычисляемое так же, как `context_window.context_window_size` в строке состояния основного интерфейса, поэтому вы можете отобразить процент для каждой строки из `tokenCount`. Оба поля требуют Claude Code v2.1.205 или более поздней версии и опускаются для задачи, модель которой ещё не разрешена.

Напишите одну строку JSON в stdout для каждой строки, которую вы хотите переопределить, в форме `{"id": "<task id>", "content": "<row body>"}`. Строка `content` отображается как есть, включая цвета ANSI и гиперссылки OSC 8. Пропустите `id` задачи, чтобы сохранить отображение по умолчанию для этой строки; выведите пустую строку `content`, чтобы скрыть её.

Те же ворота доверия и `disableAllHooks`, которые применяются к `statusLine`, применяются здесь. Плагины могут поставлять `subagentStatusLine` по умолчанию в своём [`settings.json`](/docs/ru/plugins-reference#standard-plugin-layout).

<h2 id="tips">
  Советы
</h2>

* **Тестирование с макетными входными данными**: `echo '{"model":{"display_name":"Opus"},"workspace":{"current_dir":"/home/user/project"},"context_window":{"used_percentage":25},"session_id":"test-session-abc"}' | ./statusline.sh`
* **Сохраняйте вывод коротким**: строка состояния имеет ограниченную ширину, поэтому длинный вывод может быть обрезан или неправильно обёрнут
* **Кэшируйте медленные операции**: ваш скрипт запускается часто во время активных сеансов, поэтому команды, такие как `git status`, могут вызвать задержку. См. [пример кэширования](#cache-expensive-operations) для того, как это обработать.

Проекты сообщества, такие как [ccstatusline](https://github.com/sirmalloc/ccstatusline) и [starship-claude](https://github.com/martinemde/starship-claude), предоставляют предварительно построенные конфигурации с темами и дополнительными функциями.

<h2 id="troubleshooting">
  Устранение неполадок
</h2>

**Строка состояния не отображается**

* Убедитесь, что ваш скрипт исполняемый: `chmod +x ~/.claude/statusline.sh`
* Проверьте, что ваш скрипт выводит в stdout, а не stderr
* Запустите ваш скрипт вручную, чтобы убедиться, что он выводит результат
* В Windows с установленным Git Bash обратные слэши в пути `command` вероятно будут использованы как символы экранирования перед запуском скрипта. Используйте прямые слэши в пути. См. [Конфигурация Windows](#windows-configuration).
* Если `disableAllHooks` установлен на `true` в ваших настройках, строка состояния также отключена. Удалите эту настройку или установите её на `false`, чтобы повторно включить.
* Запустите `claude --debug`, чтобы записать код выхода и stderr из первого вызова строки состояния в сеансе
* Попросите Claude прочитать ваш файл настроек и выполнить команду `statusLine` напрямую, чтобы выявить ошибки

**Строка состояния показывает `--` или пустые значения**

* Поля могут быть `null` перед завершением первого ответа API
* Обрабатывайте нулевые значения в вашем скрипте с резервными значениями, такими как `// 0` в jq
* Перезагрузите Claude Code, если значения остаются пустыми после нескольких сообщений

**Процент контекста показывает неожиданные значения**

* Используйте `used_percentage` для простейшего точного состояния контекста
* Процент контекста может отличаться от вывода `/context` из-за времени расчёта каждого

**Ссылки OSC 8 не кликабельны**

* Убедитесь, что ваш терминал поддерживает гиперссылки OSC 8 (iTerm2, Kitty, WezTerm)

* Terminal.app не поддерживает кликабельные ссылки

* Если текст ссылки отображается, но не кликабелен, Claude Code может не обнаружить поддержку гиперссылок в вашем терминале. Это часто влияет на Windows Terminal и другие эмуляторы, не входящие в список автоматического обнаружения. Установите переменную окружения `FORCE_HYPERLINK` для переопределения обнаружения перед запуском Claude Code:

  ```bash theme={null}
  FORCE_HYPERLINK=1 claude
  ```

  В PowerShell сначала установите переменную в текущем сеансе:

  ```powershell theme={null}
  $env:FORCE_HYPERLINK = "1"; claude
  ```

* Сеансы SSH и tmux могут удалять последовательности OSC в зависимости от конфигурации

* Если последовательности экранирования отображаются как буквальный текст, например `\e]8;;`, используйте `printf '%b'` вместо `echo -e` для более надёжной обработки экранирования

**Глюки отображения с последовательностями экранирования**

* Сложные последовательности экранирования (цвета ANSI, ссылки OSC 8) могут иногда вызывать искажённый вывод, если они перекрываются с другими обновлениями пользовательского интерфейса
* Если вы видите повреждённый текст, попробуйте упростить ваш скрипт до простого текстового вывода
* Многострочные строки состояния с кодами экранирования более подвержены проблемам отображения, чем однострочный простой текст

**Требуется доверие рабочей области**

* Команда строки состояния запускается только если вы приняли диалог доверия рабочей области для текущего каталога. Поскольку `statusLine` выполняет команду оболочки, она требует того же принятия доверия, что и hooks и другие параметры, выполняющие оболочку.
* Если доверие не принято, вы увидите уведомление `statusline skipped · restart to fix` вместо вывода вашей строки состояния. Перезагрузите Claude Code и примите запрос доверия, чтобы включить его.

**Ошибки скрипта или зависания**

* Скрипты, которые выходят с ненулевыми кодами или не выводят результат, вызывают пустую строку состояния
* Медленные скрипты блокируют обновление строки состояния до их завершения. Держите скрипты быстрыми, чтобы избежать устаревшего вывода.
* Если новое обновление срабатывает, пока медленный скрипт работает, выполнение скрипта отменяется
* Протестируйте ваш скрипт независимо с макетными входными данными перед его настройкой

**Уведомления делят строку состояния**

* Системные уведомления, такие как ошибки MCP сервера и автоматические обновления, отображаются на правой стороне той же строки, что и ваша строка состояния. Временные уведомления, такие как предупреждение о низком контексте, также циклируют через эту область.
* Включение подробного режима добавляет счётчик токенов в эту область
* На узких терминалах эти уведомления могут обрезать вывод вашей строки состояния
