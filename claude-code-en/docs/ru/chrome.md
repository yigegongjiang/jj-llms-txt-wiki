> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Использование Claude Code с Chrome

> Подключите Claude Code к браузеру Chrome для тестирования веб-приложений, отладки с помощью логов консоли, автоматизации заполнения форм и извлечения данных со страниц.

Claude Code интегрируется с [расширением Claude in Chrome для браузера](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn), чтобы предоставить вам возможности автоматизации браузера из CLI или [расширения VS Code](/docs/ru/vs-code#automate-browser-tasks-with-chrome). Создавайте свой код, а затем тестируйте и отлаживайте его в браузере без переключения контекста.

Claude открывает новые вкладки для задач браузера и использует состояние входа вашего браузера, поэтому он может получить доступ к любому сайту, на который вы уже вошли. Действия браузера выполняются в видимом окне Chrome в реальном времени. Когда Claude встречает страницу входа или CAPTCHA, он приостанавливается и просит вас обработать это вручную.

<Note>
  Интеграция с Chrome работает с Google Chrome и Microsoft Edge. Она еще не поддерживается на Brave, Arc или других браузерах на основе Chromium. Она также не поддерживается в Windows Subsystem for Linux (WSL).
</Note>

<h2 id="capabilities">
  Возможности
</h2>

С подключенным Chrome вы можете объединять действия браузера с задачами кодирования в единый рабочий процесс:

* **Живая отладка**: читайте ошибки консоли и состояние DOM напрямую, а затем исправьте код, который их вызвал
* **Проверка дизайна**: создайте пользовательский интерфейс на основе макета Figma, а затем откройте его в браузере, чтобы проверить соответствие
* **Тестирование веб-приложений**: тестируйте валидацию форм, проверяйте визуальные регрессии или проверяйте потоки пользователей
* **Аутентифицированные веб-приложения**: взаимодействуйте с Google Docs, Gmail, Notion или любым приложением, в которое вы вошли, без коннекторов API
* **Извлечение данных**: извлекайте структурированную информацию со страниц и сохраняйте её локально
* **Автоматизация задач**: автоматизируйте повторяющиеся задачи браузера, такие как ввод данных, заполнение форм или многосайтовые рабочие процессы
* **Запись сеанса**: записывайте взаимодействия браузера в виде GIF-файлов для документирования или обмена информацией о том, что произошло

<h2 id="prerequisites">
  Предварительные требования
</h2>

Перед использованием Claude Code с Chrome вам необходимо:

* Браузер [Google Chrome](https://www.google.com/chrome/) или [Microsoft Edge](https://www.microsoft.com/edge)
* Расширение [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) версии 1.0.36 или выше, доступное в Chrome Web Store для обоих браузеров
* [Claude Code](/docs/ru/quickstart#step-1-install-claude-code)
* Прямой план Anthropic (Pro, Max, Team или Enterprise)

<Note>
  Интеграция с Chrome недоступна через сторонних поставщиков, таких как Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry. Если вы получаете доступ к Claude исключительно через стороннего поставщика, вам нужна отдельная учетная запись claude.ai для использования этой функции.
</Note>

<h2 id="get-started-in-the-cli">
  Начало работы в CLI
</h2>

<Steps>
  <Step title="Запустите Claude Code с Chrome">
    Запустите Claude Code с флагом `--chrome`:

    ```bash theme={null}
    claude --chrome
    ```

    Вы также можете включить Chrome в существующем сеансе, выполнив `/chrome`.
  </Step>

  <Step title="Попросите Claude использовать браузер">
    Этот пример переходит на страницу, взаимодействует с ней и сообщает, что он находит, всё из вашего терминала или редактора:

    ```text theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    Первое действие браузера запрашивает разрешение на использование навыка `claude-in-chrome`. Одобрите его, и Claude откроет новую вкладку и начнёт выполнение задачи.
  </Step>
</Steps>

Выполните `/chrome` в любое время, чтобы проверить статус подключения, управлять разрешениями, переподключить расширение или выбрать, какой подключённый браузер использовать. Если при запуске действия браузера подключено более одного браузера, Claude предложит вам выбрать один.

Для VS Code см. [автоматизацию браузера в VS Code](/docs/ru/vs-code#automate-browser-tasks-with-chrome).

<h3 id="enable-chrome-by-default">
  Включение Chrome по умолчанию
</h3>

Чтобы избежать передачи `--chrome` в каждом сеансе, выполните `/chrome` и выберите "Включено по умолчанию".

В [расширении VS Code](/docs/ru/vs-code#automate-browser-tasks-with-chrome) Chrome доступен всякий раз, когда установлено расширение Chrome. Дополнительный флаг не требуется.

<Note>
  Включение Chrome по умолчанию в CLI увеличивает использование контекста, поскольку инструменты браузера всегда загружены. Если вы заметили увеличение потребления контекста, отключите этот параметр и используйте `--chrome` только при необходимости.
</Note>

<h3 id="manage-site-permissions">
  Управление разрешениями сайта
</h3>

Разрешения на уровне сайта наследуются из расширения Chrome. Управляйте разрешениями в параметрах расширения Chrome, чтобы контролировать, какие сайты Claude может просматривать, нажимать и вводить текст.

<h3 id="browser-tools-in-plan-mode">
  Инструменты браузера в режиме плана
</h3>

В [режиме плана](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode) вызовы инструментов браузера, которые только читают страницу или состояние браузера, выполняются без запроса разрешения, а вызовы, которые изменяют состояние, запрашивают одобрение.

* **Вызовы только для чтения**: `read_page`, `get_page_text`, `find`, чтение сообщений консоли или сетевых запросов и создание снимка экрана
* **Вызовы, изменяющие состояние**: клики, ввод текста, навигация, управление вкладками и окнами, а также запись GIF

Начиная с версии 2.1.199, вызов, который в остальном предназначен только для чтения, но устанавливает флаг входа, изменяющий состояние, такой как `createIfEmpty` на `tabs_context_mcp`, `clear` на средствах чтения консоли и сети или `save_to_disk` на снимке экрана, также запрашивает одобрение. Вызов `browser_batch` выполняется без запроса только в том случае, если каждое действие внутри него предназначено только для чтения.

<h2 id="example-workflows">
  Примеры рабочих процессов
</h2>

Эти примеры показывают распространённые способы объединения действий браузера с задачами кодирования. Выполните `/mcp`, выберите `claude-in-chrome`, затем выберите **View tools**, чтобы увидеть полный список доступных инструментов браузера.

<h3 id="test-a-local-web-application">
  Тестирование локального веб-приложения
</h3>

При разработке веб-приложения попросите Claude проверить, что ваши изменения работают правильно:

```text theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude переходит на ваш локальный сервер, взаимодействует с формой и сообщает, что он наблюдает.

<h3 id="debug-with-console-logs">
  Отладка с помощью логов консоли
</h3>

Claude может читать вывод консоли, чтобы помочь диагностировать проблемы. Скажите Claude, какие шаблоны искать, а не просите весь вывод консоли, так как логи могут быть многословными:

```text theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude читает сообщения консоли и может фильтровать по определённым шаблонам или типам ошибок.

<h3 id="automate-form-filling">
  Автоматизация заполнения форм
</h3>

Ускорьте повторяющиеся задачи ввода данных:

```text theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude читает ваш локальный файл, переходит по веб-интерфейсу и вводит данные для каждой записи.

<h3 id="draft-content-in-google-docs">
  Создание контента в Google Docs
</h3>

Используйте Claude для прямого написания в ваших документах без настройки API:

```text theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude открывает документ, нажимает в редактор и вводит контент. Это работает с любым веб-приложением, в которое вы вошли: Gmail, Notion, Sheets и многое другое.

<h3 id="extract-data-from-web-pages">
  Извлечение данных со страниц
</h3>

Извлекайте структурированную информацию с веб-сайтов:

```text theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude переходит на страницу, читает контент и компилирует данные в структурированный формат.

<h3 id="run-multi-site-workflows">
  Запуск многосайтовых рабочих процессов
</h3>

Координируйте задачи на нескольких веб-сайтах:

```text theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude работает на разных вкладках, чтобы собрать информацию и завершить рабочий процесс.

<h3 id="record-a-demo-gif">
  Запись демо-GIF
</h3>

Создавайте общедоступные записи взаимодействий браузера:

```text theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude записывает последовательность взаимодействий и сохраняет её как GIF-файл.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="extension-not-detected">
  Расширение не обнаружено
</h3>

Если Claude Code не может обнаружить расширение Chrome:

1. Убедитесь, что расширение Chrome установлено и включено в `chrome://extensions`
2. Убедитесь, что Claude Code обновлён, выполнив `claude --version`
3. Проверьте, что Chrome запущен
4. Выполните `/chrome` и выберите "Reconnect extension", чтобы переустановить соединение
5. Если проблема сохраняется, перезагрузите Claude Code и Chrome

При первом включении интеграции с Chrome, Claude Code устанавливает файл конфигурации хоста собственного обмена сообщениями. Chrome читает этот файл при запуске, поэтому если расширение не обнаружено при первой попытке, перезагрузите Chrome, чтобы подобрать новую конфигурацию.

Начиная с версии 2.1.199, Claude Code открывает вкладку браузера с предложением подключить расширение только при первой установке. Последующие сеансы, которые переписывают файл конфигурации, например после переключения сборок Claude Code или каталогов конфигурации, не открывают его повторно.

Если соединение по-прежнему не удаётся, убедитесь, что файл конфигурации хоста существует в:

Для Chrome:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: проверьте `HKCU\Software\Google\Chrome\NativeMessagingHosts\` в реестре Windows

Для Edge:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: проверьте `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` в реестре Windows

<h3 id="browser-not-responding">
  Браузер не отвечает
</h3>

Если команды браузера Claude перестают работать:

1. Проверьте, не блокирует ли модальное диалоговое окно (alert, confirm, prompt) страницу. Диалоговые окна JavaScript блокируют события браузера и препятствуют получению команд Claude. Закройте диалоговое окно вручную, а затем скажите Claude продолжить.
2. Попросите Claude создать новую вкладку и повторить попытку
3. Перезагрузите расширение Chrome, отключив и повторно включив его в `chrome://extensions`

<h3 id="connection-drops-during-long-sessions">
  Разрыв соединения во время длительных сеансов
</h3>

Service worker расширения Chrome может перейти в режим ожидания во время расширенных сеансов, что нарушает соединение. Если инструменты браузера перестают работать после периода неактивности, выполните `/chrome` и выберите "Reconnect extension".

<h3 id="windows-specific-issues">
  Проблемы, специфичные для Windows
</h3>

На Windows вы можете столкнуться с:

* **Конфликты именованных каналов (EADDRINUSE)**: если другой процесс использует тот же именованный канал, перезагрузите Claude Code. Закройте все остальные сеансы Claude Code, которые могут использовать Chrome.
* **Ошибки хоста собственного обмена сообщениями**: если хост собственного обмена сообщениями падает при запуске, попробуйте переустановить Claude Code, чтобы восстановить конфигурацию хоста.

<h3 id="common-error-messages">
  Распространённые сообщения об ошибках
</h3>

Это наиболее часто встречающиеся ошибки и способы их решения:

| Ошибка                               | Причина                                                          | Решение                                                                           |
| ------------------------------------ | ---------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| "Browser extension is not connected" | Хост собственного обмена сообщениями не может достичь расширение | Перезагрузите Chrome и Claude Code, затем выполните `/chrome` для переподключения |
| "Extension not detected"             | Расширение Chrome не установлено или отключено                   | Установите или включите расширение в `chrome://extensions`                        |
| "No tab available"                   | Claude попытался действовать до того, как вкладка была готова    | Попросите Claude создать новую вкладку и повторить попытку                        |
| "Receiving end does not exist"       | Service worker расширения перешёл в режим ожидания               | Выполните `/chrome` и выберите "Reconnect extension"                              |

<h2 id="see-also">
  See also
</h2>

* [Использование компьютера](/docs/ru/computer-use): управление собственными приложениями macOS, когда задача не может быть выполнена в браузере
* [Использование Claude Code в VS Code](/docs/ru/vs-code#automate-browser-tasks-with-chrome): автоматизация браузера в расширении VS Code
* [Справочник CLI](/docs/ru/cli-reference): флаги командной строки, включая `--chrome`
* [Распространённые рабочие процессы](/docs/ru/common-workflows): дополнительные способы использования Claude Code
* [Данные и конфиденциальность](/docs/ru/data-usage): как Claude Code обрабатывает ваши данные
* [Начало работы с Claude in Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): полная документация расширения Chrome, включая сочетания клавиш, планирование и разрешения
