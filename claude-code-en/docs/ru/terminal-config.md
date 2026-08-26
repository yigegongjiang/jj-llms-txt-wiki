> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Настройте ваш терминал для Claude Code

> Исправьте Shift+Enter для разрывов строк, получайте звуковой сигнал терминала когда Claude завершает работу, настройте tmux, сопоставьте цветовую тему и включите режим Vim в CLI Claude Code.

Claude Code работает в любом терминале без конфигурации. Эта страница предназначена для случаев, когда что-то конкретное ведёт себя не так, как вы ожидаете. Найдите ваш симптом ниже. Если всё уже работает правильно, вам не нужна эта страница.

* [Shift+Enter отправляет вместо вставки разрыва строки](#enter-multiline-prompts)
* [Сочетания клавиш с клавишей Option ничего не делают на macOS](#enable-option-key-shortcuts-on-macos)
* [Нет звука или оповещения когда Claude завершает работу](#get-a-terminal-bell-or-notification)
* [Вы запускаете Claude Code внутри tmux](#configure-tmux)
* [Дисплей мерцает или позиция прокрутки прыгает](#switch-to-fullscreen-rendering)
* [Вы хотите клавиши Vim в приглашении](#edit-prompts-with-vim-keybindings)

Эта страница посвящена тому, чтобы ваш терминал отправлял правильные сигналы в Claude Code. Чтобы изменить, на какие клавиши сам Claude Code реагирует, см. вместо этого [сочетания клавиш](/docs/ru/keybindings).

<h2 id="enter-multiline-prompts">
  Ввод многострочных приглашений
</h2>

Нажатие Enter отправляет ваше сообщение. Чтобы добавить разрыв строки без отправки, нажмите Ctrl+J или введите `\` и затем нажмите Enter. Оба варианта работают в каждом терминале без настройки.

В большинстве терминалов вы также можете нажать Shift+Enter, но поддержка варьируется в зависимости от эмулятора терминала:

| Терминал                                                                | Shift+Enter для разрыва строки                     |
| :---------------------------------------------------------------------- | :------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal | Работает без настройки                             |
| VS Code, Cursor, Devin Desktop, Alacritty, Zed                          | Запустите `/terminal-setup` один раз               |
| gnome-terminal, JetBrains IDEs такие как PyCharm и Android Studio       | Недоступно; используйте Ctrl+J или `\` затем Enter |

Для VS Code, Cursor, Devin Desktop, Alacritty и Zed, `/terminal-setup` записывает Shift+Enter и другие сочетания клавиш в файл конфигурации терминала. Существующие привязки остаются на месте; если вы видите сообщение, такое как `VSCode terminal Shift+Enter key binding already configured`, никаких изменений не было сделано. Запустите `/terminal-setup` непосредственно в хост-терминале, а не внутри tmux или screen, так как ему нужно записать в конфигурацию хост-терминала.

В VS Code, Cursor и Devin Desktop, `/terminal-setup` также обновляет два параметра редактора: он устанавливает `terminal.integrated.gpuAcceleration` в `"off"` для предотвращения искажённого текста в интегрированном терминале и устанавливает `terminal.integrated.mouseWheelScrollSensitivity` для более плавной прокрутки в [полноэкранном режиме](/docs/ru/fullscreen). Чтобы отменить изменение ускорения GPU, установите его обратно на `"auto"` и перезагрузите окно редактора.

Если вы работаете внутри tmux, Shift+Enter также требует [конфигурацию tmux ниже](#configure-tmux) даже когда внешний терминал её поддерживает.

Чтобы привязать разрыв строки к другой клавише или поменять поведение так, чтобы Enter вставлял разрыв строки, а Shift+Enter отправлял, сопоставьте действия `chat:newline` и `chat:submit` в вашем [файле сочетаний клавиш](/docs/ru/keybindings).

<h2 id="enable-option-key-shortcuts-on-macos">
  Включите сочетания клавиш Option на macOS
</h2>

Некоторые сочетания клавиш Claude Code используют клавишу Option, такие как Option+Enter для разрыва строки или Option+P для переключения моделей. На macOS большинство терминалов не отправляют Option как модификатор по умолчанию, поэтому эти сочетания клавиш ничего не делают, пока вы это не включите. Параметр терминала для этого обычно называется "Use Option as Meta Key"; Meta — это историческое имя Unix для клавиши, которая теперь называется Option или Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Откройте Settings → Profiles → Keyboard и установите флажок "Use Option as Meta Key".

    Если вы приняли первое приглашение Claude Code, которое предлагало "Option+Enter для разрывов строк и визуального звонка", это уже сделано. Это приглашение запускает `/terminal-setup` для вас, что включает Option как Meta и переключает звуковой звонок на визуальную вспышку экрана в вашем профиле Apple Terminal.
  </Tab>

  <Tab title="iTerm2">
    Откройте Settings → Profiles → Keys → General и установите Left Option key и Right Option key на "Esc+".

    Запуск `/terminal-setup` в iTerm2 включает "Applications in terminal may access clipboard" в Settings → General → Selection, чтобы команда `/copy` могла писать в буфер обмена вашей системы. Команда обнаруживает iTerm2 даже при запуске изнутри tmux. Перезагрузите iTerm2, чтобы изменение вступило в силу.
  </Tab>

  <Tab title="VS Code">
    Добавьте `"terminal.integrated.macOptionIsMeta": true` в ваши настройки VS Code.
  </Tab>
</Tabs>

Для Ghostty, Kitty и других терминалов ищите параметр Option-as-Alt или Option-as-Meta в файле конфигурации терминала.

<h2 id="get-a-terminal-bell-or-notification">
  Получите звуковой сигнал терминала или уведомление
</h2>

Когда Claude завершает задачу или делает паузу для приглашения разрешения, он отправляет событие уведомления. Отображение этого как звукового сигнала терминала или уведомления рабочего стола позволяет вам переключиться на другую работу, пока выполняется длительная задача.

По умолчанию Claude Code отправляет уведомление рабочего стола только в Ghostty, Kitty и iTerm2. В других терминалах установите [`preferredNotifChannel`](/docs/ru/settings#available-settings) на `"terminal_bell"` для звукового сигнала терминала или настройте [хук Notification](#play-a-sound-with-a-notification-hook) для пользовательского звука или команды.

Уведомление рабочего стола достигает вашей локальной машины через SSH, поэтому удалённый сеанс всё ещё может вас оповестить. Ghostty и Kitty перенаправляют его в центр уведомлений вашей ОС без дополнительной настройки. iTerm2 требует, чтобы вы включили перенаправление:

<Steps>
  <Step title="Откройте параметры уведомлений iTerm2">
    Перейдите в Settings → Profiles → Terminal.
  </Step>

  <Step title="Включите оповещения">
    Установите флажок "Notification Center Alerts", затем нажмите "Filter Alerts" и включите "Send escape sequence-generated alerts".
  </Step>
</Steps>

Если уведомления всё ещё не появляются, убедитесь, что ваше приложение терминала имеет разрешение на уведомления в настройках вашей ОС, и если вы работаете внутри tmux, [включите passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Воспроизведите звук с помощью хука Notification
</h3>

В любом терминале вы можете настроить [хук Notification](/docs/ru/hooks-guide#get-notified-when-claude-needs-input) для воспроизведения звука или запуска пользовательской команды, когда Claude нуждается в вашем внимании. Хуки работают вместе с уведомлением рабочего стола, а не заменяя его, поэтому терминалы, которые не получают уведомление рабочего стола, такие как Warp или интегрированный терминал VS Code, могут использовать хук или установить `preferredNotifChannel` на `"terminal_bell"` вместо этого.

Пример ниже воспроизводит системный звук на macOS. Связанное руководство содержит команды уведомлений рабочего стола для macOS, Linux и Windows.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  Настройте tmux
</h2>

Когда Claude Code работает внутри tmux, две вещи ломаются по умолчанию: Shift+Enter отправляет вместо вставки разрыва строки, и уведомления рабочего стола и [полоса прогресса](/docs/ru/settings#available-settings) никогда не достигают внешнего терминала. Добавьте эти строки в `~/.tmux.conf`, затем запустите `tmux source-file ~/.tmux.conf` чтобы применить их к работающему серверу:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

Строка `allow-passthrough` позволяет уведомлениям и обновлениям прогресса достичь внешний терминал вместо того, чтобы быть поглощёнными tmux. Строки `extended-keys` позволяют tmux различать Shift+Enter от простого Enter, чтобы сочетание клавиш разрыва строки работало.

<h2 id="match-the-color-theme">
  Сопоставьте цветовую тему
</h2>

Используйте команду `/theme` или средство выбора темы в `/config` для выбора темы Claude Code, которая соответствует вашему терминалу. Выбор опции auto обнаруживает светлый или тёмный фон вашего терминала, поэтому тема следует изменениям внешнего вида ОС всякий раз, когда это делает ваш терминал. Claude Code не управляет цветовой схемой самого терминала, которая устанавливается приложением терминала.

Чтобы настроить то, что отображается в нижней части интерфейса, настройте [пользовательскую строку состояния](/docs/ru/statusline), которая показывает текущую модель, рабочий каталог, ветку git или другой контекст.

<h3 id="create-a-custom-theme">
  Создайте пользовательскую тему
</h3>

<Note>
  Пользовательские темы требуют Claude Code v2.1.118 или более поздней версии.
</Note>

В дополнение к встроенным предустановкам, `/theme` отображает любые пользовательские темы, которые вы определили, и любые темы, предоставленные установленными [plugins](/docs/ru/plugins-reference#themes). Выберите **New custom theme…** в конце списка, чтобы создать её интерактивно: вы назовёте тему, а затем выберете отдельные цветовые токены для переопределения. Нажмите `Ctrl+E`, пока выделена пользовательская тема, чтобы отредактировать её.

Каждая пользовательская тема — это JSON-файл в `~/.claude/themes/`. Имя файла без расширения `.json` — это слаг темы, и выбор темы сохраняет `custom:<slug>` как ваше предпочтение темы. Файл имеет три необязательных поля:

| Field       | Type   | Description                                                                                                                                     |
| :---------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | Display label shown in `/theme`. Defaults to the filename slug                                                                                  |
| `base`      | string | Built-in preset the theme starts from: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, or `light-ansi`. Defaults to `dark` |
| `overrides` | object | Map of color token names to color values. Tokens not listed here fall through to the base preset                                                |

Цветовые значения принимают `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)` или `ansi:<name>`, где `<name>` — одно из 16 стандартных имён цветов ANSI, таких как `red` или `cyanBright`. Неизвестные токены и недопустимые цветовые значения игнорируются, поэтому опечатка не может нарушить отрисовку.

Следующий пример определяет тему, которая сохраняет тёмную предустановку, но перекрашивает акцент приглашения, текст ошибки и текст успеха:

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code отслеживает `~/.claude/themes/` и перезагружает при изменении файла, поэтому изменения, сделанные в вашем редакторе, применяются к работающему сеансу без перезагрузки.

Ниже приведён полный список настроек, которые вы можете установить в `overrides`. Интерактивный редактор в `/theme` показывает те же токены с живым предпросмотром, плюс несколько однозначных акцентов, таких как цвета экрана адаптации, которые опущены здесь.

<Accordion title="Color token reference">
  Следующий пример объединяет токены из нескольких групп ниже: фирменный акцент, граница режима Plan Mode, фоны diff и фон полноэкранного сообщения.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    Text and accent colors
  </h4>

  Управляйте основным фирменным акцентом и оттенками переднего плана текста, используемыми во всём интерфейсе.

  | Token         | Controls                                                         |
  | :------------ | :--------------------------------------------------------------- |
  | `claude`      | Primary brand accent, used for the spinner and assistant label   |
  | `text`        | Default foreground text                                          |
  | `inverseText` | Text drawn on top of a colored background, such as status badges |
  | `inactive`    | Secondary text such as hints, timestamps, and disabled items     |
  | `subtle`      | Faint borders and de-emphasized secondary text                   |
  | `suggestion`  | Autocomplete suggestions and selection highlight in pickers      |
  | `permission`  | Dialog borders, including permission prompts and pickers         |
  | `remember`    | Memory and `CLAUDE.md` indicators                                |

  <h4 id="status-colors">
    Status colors
  </h4>

  Сигнализируйте об успехе, сбое и состояниях предупреждения в сообщениях и индикаторах.

  | Token     | Controls                                             |
  | :-------- | :--------------------------------------------------- |
  | `success` | Success messages and passing checks                  |
  | `error`   | Error messages and failures                          |
  | `warning` | Warnings, caution messages, and the auto mode border |
  | `merged`  | Merged pull request status                           |

  <h4 id="input-box-and-mode-indicators">
    Input box and mode indicators
  </h4>

  Установите цвет границы поля ввода и акцент, отображаемый при активном режиме разрешения или индикаторе.

  | Token          | Controls                                           |
  | :------------- | :------------------------------------------------- |
  | `promptBorder` | Input box border in the default permission mode    |
  | `planMode`     | Plan mode accent and border                        |
  | `autoAccept`   | Accept-edits mode accent and border                |
  | `bashBorder`   | Input box border when entering a `!` shell command |
  | `ide`          | IDE connection indicator                           |
  | `fastMode`     | Fast mode indicator                                |

  <h4 id="diff-rendering">
    Diff rendering
  </h4>

  Раскрасьте добавленный и удалённый код в редактировании файлов и рецензировании.

  | Token               | Controls                                           |
  | :------------------ | :------------------------------------------------- |
  | `diffAdded`         | Background of added lines                          |
  | `diffRemoved`       | Background of removed lines                        |
  | `diffAddedDimmed`   | Background of unchanged context near added lines   |
  | `diffRemovedDimmed` | Background of unchanged context near removed lines |
  | `diffAddedWord`     | Word-level highlight within an added line          |
  | `diffRemovedWord`   | Word-level highlight within a removed line         |

  <h4 id="fullscreen-mode">
    Fullscreen mode
  </h4>

  Применяется только в [режиме полноэкранной отрисовки](/docs/ru/fullscreen), где сообщения имеют заливку фона.

  | Token                        | Controls                                                           |
  | :--------------------------- | :----------------------------------------------------------------- |
  | `userMessageBackground`      | Background behind your messages in the transcript                  |
  | `userMessageBackgroundHover` | Background behind a message while hovered or expanded              |
  | `messageActionsBackground`   | Background behind the selected message when the action bar is open |
  | `bashMessageBackgroundColor` | Background behind `!` shell command entries in the transcript      |
  | `memoryBackgroundColor`      | Background behind `#` memory entries in the transcript             |
  | `selectionBg`                | Background of text selected with the mouse                         |

  <h4 id="usage-meter-and-speaker-labels">
    Usage meter and speaker labels
  </h4>

  Отрегулируйте полосу, показанную в представлении `/usage`, и метки, которые отличают ваши сообщения от сообщений Claude.

  | Token              | Controls                                          |
  | :----------------- | :------------------------------------------------ |
  | `rate_limit_fill`  | Filled portion of the usage meter                 |
  | `rate_limit_empty` | Unfilled portion of the usage meter               |
  | `briefLabelYou`    | Color of the `You` label on your messages         |
  | `briefLabelClaude` | Color of the `Claude` label on assistant messages |

  <h4 id="shimmer-variants-and-subagent-colors">
    Shimmer variants and subagent colors
  </h4>

  Несколько токенов имеют парный вариант shimmer, который предоставляет более светлый цвет, используемый в анимированном градиенте спиннера. Переопределите shimmer вместе с его базовым токеном, если анимация выглядит несоответствующей.

  * `claude` и `claudeShimmer`
  * `warning` и `warningShimmer`
  * `permission` и `permissionShimmer`
  * `promptBorder` и `promptBorderShimmer`
  * `inactive` и `inactiveShimmer`
  * `fastMode` и `fastModeShimmer`

  Каждый [subagent](/docs/ru/sub-agents) и параллельная задача отображаются в одном из восьми именованных цветов, чтобы вы могли различить их в транскрипте. Имена токенов следуют шаблону `<color>_FOR_SUBAGENTS_ONLY`, где `<color>` — это `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` или `cyan`. Переопределите их, чтобы изменить внешний вид каждого именованного цвета. Например, subagent с `color: blue` в его определении отображается с использованием значения `blue_FOR_SUBAGENTS_ONLY`.

  Ключевые слова [`ultrathink`](/docs/ru/model-config#use-ultrathink-for-one-off-deep-reasoning) и [`ultraplan`](/docs/ru/ultraplan) в поле ввода приглашения отображаются с семицветным радужным градиентом. Имена токенов следуют шаблону `rainbow_<color>` и `rainbow_<color>_shimmer`, где `<color>` — это `red`, `orange`, `yellow`, `green`, `blue`, `indigo` или `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Переключитесь на полноэкранный рендеринг
</h2>

Если дисплей мерцает или позиция прокрутки прыгает, пока Claude работает, переключитесь на [режим полноэкранного рендеринга](/docs/ru/fullscreen). Он рисует на отдельном экране, который терминал зарезервировал для полноэкранных приложений, вместо добавления в вашу обычную прокрутку, что сохраняет использование памяти на плоском уровне и добавляет поддержку мыши для прокрутки и выделения. В этом режиме вы прокручиваете с помощью мыши или PageUp внутри Claude Code, а не с помощью встроенной прокрутки вашего терминала; см. [страницу полноэкранного режима](/docs/ru/fullscreen#search-and-review-the-conversation) для того, как искать и копировать.

Запустите `/tui fullscreen` для переключения и сохранения предпочтения. Ваш разговор перезапускается нетронутым, и будущие сеансы начинаются в полноэкранном режиме. Вы также можете установить переменную окружения `CLAUDE_CODE_NO_FLICKER` перед запуском Claude Code:

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  Вставьте большое содержимое
</h2>

Когда вы вставляете более 10 000 символов в приглашение, Claude Code сворачивает ввод в заполнитель `[Pasted text]`, чтобы поле ввода оставалось пригодным для использования. Полное содержимое всё ещё отправляется Claude при отправке.

Встроенный терминал VS Code может отбросить символы из очень больших вставок, прежде чем они достигнут Claude Code, поэтому предпочитайте рабочие процессы на основе файлов там. Для очень больших входных данных, таких как целые файлы или длинные журналы, запишите содержимое в файл и попросите Claude прочитать его вместо вставки. Это сохраняет стенограмму разговора читаемой и позволяет Claude ссылаться на файл по пути в более поздних ходах.

<h2 id="edit-prompts-with-vim-keybindings">
  Редактируйте приглашения с помощью сочетаний клавиш Vim
</h2>

Claude Code включает режим редактирования в стиле Vim для ввода приглашения. Включите его через `/config` → Editor mode или установив [`editorMode`](/docs/ru/settings#available-settings) на `"vim"` в `~/.claude/settings.json`. Установите Editor mode обратно на `normal`, чтобы отключить его.

Режим Vim поддерживает подмножество движений и операторов режима NORMAL и VISUAL, таких как навигация `hjkl`, выделение `v`/`V` и `d`/`c`/`y` с текстовыми объектами. См. [справочник режима редактора Vim](/docs/ru/interactive-mode#vim-editor-mode) для полной таблицы клавиш. Движения Vim не переназначаются через файл сочетаний клавиш.

Нажатие Enter всё ещё отправляет ваше приглашение в режиме INSERT, в отличие от стандартного Vim. Используйте `o` или `O` в режиме NORMAL или Ctrl+J для вставки разрыва строки вместо этого.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Интерактивный режим](/docs/ru/interactive-mode): полный справочник сочетаний клавиш и таблица клавиш Vim
* [Сочетания клавиш](/docs/ru/keybindings): переназначьте любое сочетание клавиш Claude Code, включая Enter и Shift+Enter
* [Полноэкранный рендеринг](/docs/ru/fullscreen): детали прокрутки, поиска и копирования в полноэкранном режиме
* [Руководство по hooks](/docs/ru/hooks-guide): больше примеров хуков Notification для Linux и Windows
* [Troubleshooting](/docs/ru/troubleshooting): исправления для проблем вне конфигурации терминала
