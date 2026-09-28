> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code intelligence plugins

> Установите плагин языкового сервера, чтобы Claude видел ошибки типов после редактирования и навигировал по коду по символам, и ответьте на диалог рекомендации плагина LSP.

Плагин code intelligence предоставляет Claude живую диагностику и переход к определению, которые есть в вашем редакторе, поэтому Claude перехватывает ошибки типов и отсутствующие импорты, которые вводят его собственные правки, прежде чем вы запустите сборку, и находит определения и ссылки по символам вместо поиска по тексту.

Каждый плагин подключает Claude Code к языковому серверу для одного языка через Language Server Protocol (LSP). Вы устанавливаете плагин из официального marketplace Anthropic и двоичный файл языкового сервера на вашей машине.

<Note>
  Code intelligence plugins работают в сеансах терминала. В [облачных сеансах](/docs/ru/claude-code-on-the-web) Claude Code не запускает языковые серверы плагинов, поэтому Claude не получает диагностику или навигацию по коду там. Чтобы написать свой собственный плагин языкового сервера или подключить языковой сервер, у которого нет плагина, см. [LSP servers in plugin components](/docs/ru/plugins/components#lsp-servers).
</Note>

Чтобы начать работу, найдите свой язык в таблице в разделе [Установка плагина code intelligence](#install-a-code-intelligence-plugin). Плагины в этой таблице поступают из [официального marketplace плагинов](/docs/ru/plugins/anthropic-marketplaces) Anthropic.

Если вы уже видели диалог **LSP plugin recommendation**, см. [Принять или отклонить диалог рекомендации](#accept-or-dismiss-the-recommendation-dialog), чтобы узнать, что делает каждый выбор.

<h2 id="install-a-code-intelligence-plugin">
  Установка плагина code intelligence
</h2>

Плагин code intelligence сообщает Claude Code, какая команда запускает языковой сервер и какие расширения файлов он обрабатывает. Он не включает языковой сервер. Сначала установите двоичный файл языкового сервера, затем плагин, затем подтвердите, что сервер запускается.

<Steps>
  <Step title="Установка двоичного файла языкового сервера">
    Найдите свой язык в таблице ниже и установите двоичный файл в его строке. Если вашего языка нет в списке, см. [Добавление языка без официального плагина](#add-a-language-without-an-official-plugin).

    | Язык                    | Плагин                                                                                                           | Двоичный файл                |
    | :---------------------- | :--------------------------------------------------------------------------------------------------------------- | :--------------------------- |
    | C/C++                   | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                     |
    | C#                      | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                  |
    | Go                      | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                      |
    | Java                    | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                      |
    | Kotlin                  | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                 |
    | Liquid                  | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, из Shopify CLI    |
    | Lua                     | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`        |
    | PHP                     | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`               |
    | Python                  | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`         |
    | Ruby                    | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                   |
    | Rust                    | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`              |
    | Swift                   | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`              |
    | TypeScript и JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server` |

    Anthropic поддерживает каждый плагин в таблице, кроме `liquid-lsp`, который поддерживает Shopify и который официальный marketplace указывает.

    Чтобы найти команду, которая устанавливает двоичный файл, перейдите по ссылке плагина в таблице на его README. Для TypeScript эта команда: `npm install -g typescript-language-server typescript`.

    После установки двоичного файла подтвердите, что он находится в `PATH` оболочки, из которой вы запускаете `claude`, например с помощью `which typescript-language-server` или `Get-Command typescript-language-server` в PowerShell.
  </Step>

  <Step title="Установка плагина">
    Чтобы установить плагин, указанный для вашего языка в таблице шага 1, запустите `/plugin install` в сеансе Claude Code, заменив `typescript-lsp` на имя этого плагина:

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    Сообщение подтверждения указывает, активен ли плагин сейчас или требуется `/reload-plugins`. Если установка не удалась с ошибкой `Marketplace "claude-plugins-official" not found`, см. [запись по устранению неполадок для этой ошибки](/docs/ru/plugins/troubleshooting#marketplace-claude-plugins-official-not-found). Чтобы контролировать, где устанавливается плагин, или запустить установку из вашей оболочки вместо Claude Code, см. [Установка плагинов](/docs/ru/plugins/install).
  </Step>

  <Step title="Подтверждение запуска сервера">
    Языковой сервер запускается в первый раз, когда Claude редактирует файл с одним из расширений плагина. Чтобы увидеть это в действии, попросите Claude внести ошибку типа в файл этого языка, а затем исправить её. Затем проверьте разговор на наличие строки диагностики:

    * **Появляется строка диагностики**: `Found N new diagnostic issues in M files (ctrl+o to expand)` под редактированием, которое внесло ошибку, означает, что сервер запустился.
    * **Строка диагностики не появляется**: запустите `/plugin` и откройте вкладку **Errors**. Строка, читающая `Executable not found in $PATH: "<binary>"`, называет двоичный файл для установки. Если вкладка не содержит такую строку, см. [Устранение неполадок code intelligence](#troubleshoot-code-intelligence).

    После установки отсутствующего двоичного файла Claude Code повторит попытку в следующий раз, когда Claude отредактирует соответствующий файл. Если вы установили двоичный файл в каталог, который не находится в `PATH` оболочки, из которой вы запустили `claude`, запустите новый сеанс из оболочки, где он находится.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  Посмотрите, что получает Claude
</h2>

С запущенным языковым сервером Claude получает диагностику и навигацию по коду:

* **Диагностика после редактирования**: каждый раз, когда Claude редактирует или записывает файл, который обрабатывает сервер, Claude получает ошибки и предупреждения, которые сообщает сервер. Он видит ошибку типа, отсутствующий импорт или синтаксическую ошибку, которую он внёс, без запуска компилятора.
* **Навигация по коду**: Claude получает инструмент `LSP`, который ищет символы через сервер вместо поиска текста. Инструмент доступен только для чтения. Для того, что Claude может искать с помощью инструмента и как разрешения применяются к нему, см. [LSP tool behavior](/docs/ru/tools-reference#lsp-tool-behavior).

<h3 id="read-the-diagnostics-yourself">
  Прочитайте диагностику самостоятельно
</h3>

После того как Claude отредактирует файл, который обрабатывает сервер, разговор показывает только сводку `Found N new diagnostic issues`. Чтобы прочитать сами проблемы, нажмите **Ctrl+O**.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  Принять или отклонить диалог рекомендации
</h2>

Если двоичный файл языкового сервера уже находится в вашем `PATH` и плагин, который его использует, не установлен, Claude Code предлагает установить плагин для вас в диалоге с названием **LSP plugin recommendation**.

<h3 id="when-the-recommendation-dialog-appears">
  Когда появляется диалог рекомендации
</h3>

Диалог **LSP plugin recommendation** может появиться после того, как Claude отредактирует файл. Эти условия определяют, появляется ли он и какой плагин он предлагает:

* **Плагин соответствует файлу**: один из добавленных вами marketplaces или официальный marketplace, который Claude Code зарегистрировал для вас, содержит плагин code intelligence для расширения этого файла, и двоичный файл плагина установлен.
* **Официальный первым**: когда более одного marketplace предлагает плагин для расширения, диалог предлагает плагин официального marketplace.
* **Один раз за сеанс**: диалог появляется максимум один раз в сеансе, для первого соответствующего файла, который редактирует Claude.
* **Не для облачных сеансов**: диалог никогда не появляется, когда ваш терминал подключен к облачному сеансу, например к тому, который вы запустили с помощью [`claude --cloud`](/docs/ru/claude-code-on-the-web#from-terminal-to-cloud).

<h3 id="respond-to-the-recommendation-dialog">
  Ответьте на диалог рекомендации
</h3>

Диалог **LSP plugin recommendation** называет плагин и предлагает эти варианты:

* **Yes, install**: Claude Code устанавливает плагин для вашей учётной записи пользователя и выводит `<plugin> installed · restart to apply`. Запустите новый сеанс для загрузки сервера.
* **No, not now**: диалог закрывается, и более поздний сеанс может предложить плагин снова. Нажатие **Esc** делает то же самое.
* **Never for this plugin**: диалог перестаёт появляться для этого плагина и по-прежнему появляется для других.
* **Disable all LSP recommendations**: диалог перестаёт появляться для каждого языка.

Если вы не выберете опцию, Claude Code закроет её через 30 секунд и будет считать это игнорированием. Счёт ведётся между сеансами. После пяти игнорируемых диалогов Claude Code перестаёт рекомендовать плагины, как если бы вы выбрали **Disable all LSP recommendations**.

<h3 id="turn-recommendations-back-on">
  Включить рекомендации снова
</h3>

Диалог **LSP plugin recommendation** перестаёт появляться после того, как вы выберете **Disable all LSP recommendations** или игнорируете его пять раз.

* **Отключено или игнорировано пять раз**: чтобы включить его снова в любом случае, удалите ключи `lspRecommendationDisabled` и `lspRecommendationIgnoredCount` из `~/.claude.json`, собственного файла конфигурации Claude Code.
* **Never for this plugin**: если вы выбрали **Never for this plugin** и хотите, чтобы этот плагин был предложен снова, удалите его идентификатор `name@marketplace` из списка `lspRecommendationNeverPlugins` в том же файле.

<h2 id="troubleshoot-code-intelligence">
  Устранение неполадок code intelligence
</h2>

Страница устранения неполадок плагинов охватывает симптомы, специфичные для плагинов code intelligence, в разделе [Language server doesn't start, uses too much memory, or reports wrong diagnostics](/docs/ru/plugins/troubleshooting#language-server-doesnt-start):

* **Языковой сервер не запускается**: вы видите `Executable not found in $PATH` на вкладке **Errors** в `/plugin`, или Claude никогда не сообщает диагностику для языка.
* **Высокое использование памяти**: использование памяти увеличивается, пока сервер индексирует проект.
* **Ложные положительные диагностики в monorepo**: диагностика сообщает об импортах как неразрешённых, когда они разрешены.

<h2 id="add-a-language-without-an-official-plugin">
  Добавление языка без официального плагина
</h2>

Если вашего языка нет в [таблице официальных плагинов](#install-a-code-intelligence-plugin), вы всё равно можете подключить языковой сервер.

1. Напишите плагин с файлом `.lsp.json`, который называет команду сервера и расширения файлов, которые он обрабатывает.
2. Затем загрузите плагин с помощью [`--plugin-dir`](/docs/ru/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) или опубликуйте его на marketplace.

Для полей файла и рабочего примера см. [LSP servers in plugin components](/docs/ru/plugins/components#lsp-servers).

<h2 id="next-steps">
  Следующие шаги
</h2>

* [LSP servers in plugin components](/docs/ru/plugins/components#lsp-servers): напишите `.lsp.json` для языкового сервера, у которого нет официального плагина
* [Install and manage plugins](/docs/ru/plugins/install): области, обновления и удаление
* [Troubleshoot plugins](/docs/ru/plugins/troubleshooting): ошибки загрузки, выходящие за рамки языковых серверов на этой странице
* [Find plugins in the official marketplace](/docs/ru/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): где просмотреть остальную часть официального marketplace
