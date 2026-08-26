> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Troubleshooting

> Исправьте высокое использование CPU или памяти, зависания, auto-compact thrashing и проблемы поиска в Claude Code, и найдите нужную страницу для других проблем.

Эта страница охватывает проблемы производительности, стабильности и поиска после того, как Claude Code запущен. Для других проблем начните со страницы, которая соответствует тому, где вы застряли:

| Симптом                                                                                                                                                 | Перейти к                                                                                |
| :------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------- |
| `command not found`, ошибка установки, проблемы PATH, `EACCES`, ошибки TLS                                                                              | [Troubleshoot installation and login](/docs/ru/troubleshoot-install)                          |
| Обновление или установка загрузки не удаётся с `The connection dropped while downloading the update` или `aborted`                                      | [Error reference](/docs/ru/errors#the-connection-dropped-while-downloading-the-update)        |
| Циклы входа, ошибки OAuth, `403 Forbidden`, "organization disabled", учётные данные Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry | [Troubleshoot installation and login](/docs/ru/troubleshoot-install#login-and-authentication) |
| Параметры не применяются, hooks не срабатывают, MCP servers не загружаются                                                                              | [Debug your configuration](/docs/ru/debug-your-config)                                        |
| `API Error: 5xx`, `529 Overloaded`, `429`, ошибки валидации запроса                                                                                     | [Error reference](/docs/ru/errors)                                                            |
| `model not found` или `you may not have access to it`                                                                                                   | [Error reference](/docs/ru/errors#theres-an-issue-with-the-selected-model)                    |
| Расширение VS Code не подключается или не обнаруживает Claude                                                                                           | [VS Code integration](/docs/ru/vs-code#fix-common-issues)                                     |
| Плагин JetBrains или IDE не обнаружена                                                                                                                  | [JetBrains integration](/docs/ru/jetbrains#troubleshooting)                                   |
| Высокое использование CPU или памяти, медленные ответы, зависания, поиск не находит файлы                                                               | [Performance and stability](#performance-and-stability) ниже                             |

Если вы не уверены, какой применяется, запустите `/doctor` внутри Claude Code для автоматической проверки вашей установки, параметров, расширений и использования контекста; он предлагает исправления, которые может применить после вашего подтверждения. Если `claude` вообще не запускается, запустите `claude doctor` из вашей оболочки вместо этого. Запустите `/mcp` для проверки статуса MCP server.

<h2 id="performance-and-stability">
  Performance and stability
</h2>

Эти разделы охватывают проблемы, связанные с использованием ресурсов, отзывчивостью и поведением поиска.

<h3 id="high-cpu-or-memory-usage">
  High CPU or memory usage
</h3>

Claude Code разработан для работы с большинством сред разработки, но может потреблять значительные ресурсы при обработке больших кодовых баз. Если вы испытываете проблемы с производительностью:

1. Используйте `/compact` регулярно, чтобы уменьшить размер контекста
2. Закройте и перезагрузите Claude Code между основными задачами
3. Рассмотрите добавление больших директорий сборки в ваш файл `.gitignore`
4. Перезагрузитесь с помощью [`claude --safe-mode`](/docs/ru/cli-reference#cli-flags), чтобы проверить, является ли источником plugin, MCP server или hook. Это отключает все настройки на время сеанса; если использование снизится, см. [Debug your configuration](/docs/ru/debug-your-config#test-against-a-clean-configuration), чтобы найти, какой именно

Если использование памяти остаётся высоким после этих шагов, запустите `/heapdump`, чтобы записать снимок кучи JavaScript и разбор памяти на `~/Desktop`. На Linux без папки Desktop файлы записываются в вашу домашнюю директорию.

Разбор показывает размер резидентного набора, кучу JS, буферы массивов и неучтённую нативную память, что помогает определить, находится ли рост в объектах JavaScript или в нативном коде. Чтобы проверить удерживающие ссылки, откройте файл `.heapsnapshot` в Chrome DevTools в разделе Memory → Load; разбор находится в файле, заканчивающемся на `-diagnostics.json`.

<Warning>
  Файл `.heapsnapshot` содержит каждую строку в процессе. Не прикрепляйте его к публичной проблеме и не делитесь им. Прикрепляйте только файл `-diagnostics.json` при сообщении о проблеме с памятью на [GitHub](https://github.com/anthropics/claude-code/issues). Этот файл содержит статистику памяти и не содержит содержимого разговора или учётных данных.
</Warning>

<h3 id="large-tables-are-cut-off-in-the-terminal">
  Large tables are cut off in the terminal
</h3>

Таблица Markdown с более чем 200 строками отображает первые 200 строк, за которыми следует строка `… N more rows not shown`. Отображение ограничено только визуально: полная таблица остаётся в разговоре, и [`/copy`](/docs/ru/commands) копирует каждую строку. Для таблицы, которая слишком велика для чтения в терминале, попросите Claude записать её в файл вместо этого. До версии v2.1.208 Claude Code отображал каждую строку, поэтому возобновление сеанса, содержащего очень большую таблицу, могло зависнуть при её повторном отображении.

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  Auto-compaction stops with a thrashing error
</h3>

Если вы видите `Autocompact is thrashing: the context refilled to the limit...`, автоматическое сжатие прошло успешно, но файл или вывод инструмента немедленно заполнили окно контекста несколько раз подряд. Claude Code останавливает повторные попытки, чтобы избежать траты вызовов API на цикл, который не делает прогресс.

Чтобы восстановиться:

1. Попросите Claude прочитать большой файл в меньших фрагментах, таких как конкретный диапазон строк или функция, вместо всего файла
2. Запустите `/compact` с фокусом, который удаляет большой вывод, например `/compact keep only the plan and the diff`
3. Переместите работу с большим файлом на [subagent](/docs/ru/sub-agents), чтобы она работала в отдельном окне контекста
4. Запустите `/clear`, если более ранний разговор больше не нужен

<h3 id="command-hangs-or-freezes">
  Command hangs or freezes
</h3>

Если Claude Code кажется неотзывчивым:

1. Нажмите Ctrl+C, чтобы попытаться отменить текущую операцию
2. Если неотзывчив, вам может потребоваться закрыть терминал и перезагрузить

Перезагрузка не теряет вашу беседу. Запустите `claude --resume` в той же директории, чтобы продолжить сеанс.

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  Garbled or corrupted text in an editor's integrated terminal
</h3>

Если символы отображаются как прямоугольники, размазанные линии или неправильные глифы при запуске Claude Code в интегрированном терминале VS Code, Cursor или Devin Desktop, причиной, вероятно, является GPU-рендерер терминала. Запустите `/terminal-setup` внутри Claude Code, чтобы установить `terminal.integrated.gpuAcceleration` на `"off"`, или установите это вручную в настройках вашего редактора и перезагрузите окно. Смотрите [Terminal configuration](/docs/ru/terminal-config) для других настроек, которые записывает `/terminal-setup`.

<h3 id="search-and-discovery-issues">
  Search and discovery issues
</h3>

Если инструмент Search, упоминания `@file`, пользовательские агенты или пользовательские skills не находят файлы, встроенный двоичный файл `ripgrep` может не работать на вашей системе. Установите пакет `ripgrep` вашей платформы и скажите Claude Code использовать его вместо этого:

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

Затем установите `USE_BUILTIN_RIPGREP=0` в вашем [окружении](/docs/ru/env-vars).

<h3 id="slow-or-incomplete-search-results-on-wsl">
  Slow or incomplete search results on WSL
</h3>

Штрафы производительности чтения диска при [работе с файловыми системами на WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems) могут привести к меньшему количеству совпадений, чем ожидается, при использовании Claude Code на WSL. Поиск всё ещё функционирует, но возвращает меньше результатов, чем на собственной файловой системе.

<Note>
  `claude doctor` показывает Search как OK в этом случае.
</Note>

**Решения:**

1. **Отправляйте более конкретные поиски**: уменьшите количество файлов, которые ищутся, указав директории или типы файлов: "Search for JWT validation logic in the auth-service package" или "Find use of md5 hash in JS files".

2. **Переместите проект на файловую систему Linux**: если возможно, убедитесь, что ваш проект находится на файловой системе Linux (`/home/`) вместо файловой системы Windows (`/mnt/c/`).

3. **Используйте нативный Windows вместо этого**: рассмотрите запуск Claude Code нативно на Windows вместо WSL для лучшей производительности файловой системы.

<h2 id="get-more-help">
  Получить дополнительную помощь
</h2>

Если вы испытываете проблемы, не охватываемые здесь:

1. Запустите `/doctor` для проверки установки и `/mcp` для проверки статуса MCP сервера
2. Используйте команду `/feedback` в Claude Code, чтобы сообщить о проблемах непосредственно в Anthropic
3. Проверьте [репозиторий GitHub](https://github.com/anthropics/claude-code) на известные проблемы
4. Спросите Claude напрямую о его возможностях и функциях. Claude имеет встроенный доступ к своей документации.
