> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник по ошибкам

> Найдите сообщения об ошибках runtime Claude Code, узнайте, что они означают и как их исправить.

На этой странице перечислены ошибки runtime, которые отображает Claude Code, и способы восстановления после каждой из них, а также что проверить, когда ответы кажутся неправильными без ошибки. Для ошибок установки, таких как `command not found` или сбои TLS во время установки, см. [Troubleshoot installation and login](/docs/ru/troubleshoot-install).

Эти ошибки и команды восстановления применяются во всех интерфейсах: CLI, [Desktop app](/docs/ru/desktop) и [Claude Code on the web](/docs/ru/claude-code-on-the-web), поскольку все три используют один и тот же Claude Code CLI. Для проблем, специфичных для конкретного интерфейса, см. раздел troubleshooting на странице этого интерфейса.

<Note>
  Claude Code вызывает Claude API для получения ответов модели, поэтому большинство ошибок runtime соответствуют базовому коду ошибки API. На этой странице описано, что каждая ошибка означает в Claude Code и как восстановиться. Для определений кодов состояния HTTP в исходном виде см. [Claude Platform error reference](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Найдите вашу ошибку
</h2>

Сопоставьте сообщение, которое вы видите в терминале, с разделом ниже.

| Сообщение                                                                                          | Раздел                                                                                                                           |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                             | [Server errors](#api-error-500-internal-server-error)                                                                            |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Server errors](#api-error-repeated-529-overloaded-errors)                                                                       |
| `Request timed out`                                                                                | [Server errors](#request-timed-out), или [Network](#unable-to-connect-to-api), если сообщение упоминает вашу интернет-соединение |
| `Server error mid-response. The response above may be incomplete.`                                 | [Server errors](#the-response-above-may-be-incomplete)                                                                           |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Server errors](#the-response-above-may-be-incomplete)                                                                           |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                             |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                             |
| `Auto mode classifier transcript exceeded context window`                                          | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                             |
| `Agent terminated early due to an API error`                                                       | [Server errors](#agent-terminated-early-due-to-an-api-error)                                                                     |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Usage limits](#youve-hit-your-session-limit)                                                                                    |
| `Usage credits required for 1M context`                                                            | [Usage limits](#usage-credits-required-for-1m-context)                                                                           |
| `Server is temporarily limiting requests`                                                          | [Usage limits](#server-is-temporarily-limiting-requests)                                                                         |
| `Request rejected (429)`                                                                           | [Usage limits](#request-rejected-429)                                                                                            |
| `Credit balance is too low`                                                                        | [Usage limits](#credit-balance-is-too-low)                                                                                       |
| `Not logged in · Please run /login`                                                                | [Authentication](#not-logged-in)                                                                                                 |
| `Could not resolve authentication method`                                                          | [Authentication](#could-not-resolve-authentication-method)                                                                       |
| `Invalid API key`                                                                                  | [Authentication](#invalid-api-key)                                                                                               |
| `Your apiKeyHelper script is failing`                                                              | [Authentication](#your-apikeyhelper-script-is-failing)                                                                           |
| `This organization has been disabled`                                                              | [Authentication](#this-organization-has-been-disabled)                                                                           |
| `Your organization has disabled API key authentication`                                            | [Authentication](#your-organization-has-disabled-api-key-authentication)                                                         |
| `Your organization has disabled Claude subscription access`                                        | [Authentication](#your-organization-has-disabled-claude-subscription-access)                                                     |
| `Routines are disabled by your organization's policy`                                              | [Authentication](#routines-are-disabled-by-your-organizations-policy)                                                            |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Authentication](#remote-control-requires-the-anthropic-api)                                                                     |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Authentication](#oauth-token-revoked-or-expired)                                                                                |
| `Login expired · Please run /login`                                                                | [Authentication](#login-expired)                                                                                                 |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Authentication](#login-expired)                                                                                                 |
| `does not meet scope requirement user:profile`                                                     | [Authentication](#oauth-scope-requirement)                                                                                       |
| `AWS credentials expired or invalid`                                                               | [Authentication](#aws-credentials-expired-or-invalid)                                                                            |
| `AWS authentication failed`                                                                        | [Authentication](#aws-authentication-failed)                                                                                     |
| `AWS default-chain credential resolve timed out`                                                   | [Authentication](#aws-default-chain-credential-resolve-timed-out)                                                                |
| `Unable to connect to API`                                                                         | [Network](#unable-to-connect-to-api)                                                                                             |
| `Waiting for API response · will retry in`                                                         | [Automatic retries](#automatic-retries), или [Network](#unable-to-connect-to-api), если это продолжается                         |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Network](#bedrock-streaming-response-has-an-unexpected-content-type)                                                            |
| `SSL certificate verification failed`                                                              | [Network](#ssl-certificate-errors)                                                                                               |
| `SSL certificate error (...)` during login or startup                                              | [Network](#ssl-certificate-errors)                                                                                               |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Network](#host-not-allowed-in-a-cloud-session)                                                                                  |
| `Couldn't reconnect to your Remote Control session`                                                | [Network](#couldnt-reconnect-to-your-remote-control-session)                                                                     |
| `Prompt is too long`                                                                               | [Request errors](#prompt-is-too-long)                                                                                            |
| `Error during compaction: Conversation too long`                                                   | [Request errors](#error-during-compaction-conversation-too-long)                                                                 |
| `Request too large`                                                                                | [Request errors](#request-too-large)                                                                                             |
| `Image was too large`                                                                              | [Request errors](#image-was-too-large)                                                                                           |
| `Unable to resize image`                                                                           | [Request errors](#unable-to-resize-image)                                                                                        |
| `PDF too large` / `PDF is password protected`                                                      | [Request errors](#pdf-errors)                                                                                                    |
| `Extra inputs are not permitted`                                                                   | [Request errors](#extra-inputs-are-not-permitted)                                                                                |
| `There's an issue with the selected model`                                                         | [Request errors](#theres-an-issue-with-the-selected-model)                                                                       |
| `Model ... is not a recognized model id`                                                           | [Request errors](#model-is-not-a-recognized-model-id)                                                                            |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Request errors](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                         |
| `Model ... is restricted by your organization's settings`                                          | [Request errors](#model-is-restricted-by-your-organizations-settings)                                                            |
| `thinking.type.enabled is not supported for this model`                                            | [Request errors](#thinking-type-enabled-is-not-supported-for-this-model)                                                         |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Request errors](#thinking-budget-exceeds-output-limit)                                                                          |
| `API Error: 400 due to tool use concurrency issues`                                                | [Request errors](#tool-use-or-thinking-block-mismatch)                                                                           |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Request errors](#usage-policy-refusal)                                                                                          |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                                                                 |
| `Installation was killed before it could finish (exit code 137)`                                   | [Installation errors](#installation-was-killed-before-it-could-finish)                                                           |
| `The connection dropped while downloading the update`                                              | [Installation errors](#the-connection-dropped-while-downloading-the-update)                                                      |
| `Download timed out: exceeded the total deadline`                                                  | [Installation errors](#the-connection-dropped-while-downloading-the-update)                                                      |
| `--bg and --print conflict`                                                                        | [Command-line errors](#command-line-errors)                                                                                      |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Command-line errors](#command-line-errors)                                                                                      |
| `Could not import <server>: <reason>`                                                              | [Command-line errors](#could-not-import-a-server-from-claude-desktop)                                                            |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Command-line errors](#mcp-permission-prompt-tool-not-found)                                                                     |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Plugin errors](#marketplace-is-registered-from-an-untrusted-source)                                                             |
| `references ${user_config.*} in a shell-form command`                                              | [Plugin errors](#plugin-command-references-user-config)                                                                          |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Plugin errors](#plugin-command-references-user-config)                                                                          |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Plugin errors](#plugin-command-references-user-config)                                                                          |
| `would be spawned with zero tools — refusing`                                                      | [Tool errors](#agent-would-be-spawned-with-zero-tools)                                                                           |
| `File is covered by a Read deny rule in your permission settings`                                  | [Tool errors](#file-is-covered-by-a-read-deny-rule)                                                                              |
| `Can't open MCP settings in a background session`                                                  | [Background session errors](#commands-refused-in-a-background-session)                                                           |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Background session errors](#claude_code_process_wrapper-launcher-errors)                                                        |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Configuration warnings](#workspace-has-not-been-trusted)                                                                        |
| Responses seem lower quality than usual                                                            | [Response quality](#responses-seem-lower-quality-than-usual)                                                                     |

<h2 id="automatic-retries">
  Automatic retries
</h2>

Claude Code повторяет попытки при временных сбоях перед отображением ошибки. Ошибки сервера, перегруженные ответы, тайм-ауты запросов, временные дроссели 429 и разорванные соединения повторяются до 10 раз с экспоненциальной задержкой. Начиная с версии 2.1.198, это охватывает соединения, которые разрываются в середине ответа перед любым видимым выводом: Claude Code повторно отправляет запрос с той же задержкой и ход продолжается вместо остановки с ошибкой соединения. Начиная с версии 2.1.199, временные дроссели 429, которые не содержат заголовки квоты вашего плана, также повторяются, когда вы вошли с подписью claude.ai; более ранние версии повторяли их только для входов по ключу API и Enterprise.

Некоторые классы сбоев не повторяются, потому что повторная попытка не может быть успешной:

* Начиная с версии 2.1.199, сбой проверки сертификата TLS, такой как прокси, проверяющий TLS, отсутствующий пакет `NODE_EXTRA_CA_CERTS` или истекший сертификат, завершается ошибкой при первой попытке, поэтому исправление появляется немедленно вместо полного бюджета повторных попыток. См. [SSL certificate errors](#ssl-certificate-errors). Временные условия TLS, такие как тайм-аут рукопожатия, все еще повторяются.
* Начиная с версии 2.1.199, ошибка сервера, которая поступает после того, как Claude уже передал видимый вывод, сохраняет частичный ответ и добавляет [incomplete-response notice](#the-response-above-may-be-incomplete) вместо повторной попытки, так как повторный запрос может выполнить те же вызовы инструментов дважды. Более ранние версии отбрасывали частичный вывод и сообщали о ходе как об ошибке.
* [Amazon Bedrock streaming response with an unexpected content-type](#bedrock-streaming-response-has-an-unexpected-content-type) завершается ошибкой при первой попытке, потому что шлюз или прокси, переписывающий ответ, переписал бы повторную попытку таким же образом. Требуется Claude Code версии 2.1.208 или позже.

Во время повторных попыток спиннер показывает обратный отсчет `Retrying in Ns · attempt x/y` после метки ошибки. Метка называет конкретную причину первой попытки для сбоев, на которые вы можете действовать немедленно: сеть отключена, рукопожатие TLS не удалось, или вы достигли лимита скорости. Для других ошибок она читается как `API error` вначале. Начиная с версии 2.1.198, она переключается на конкретную причину третьей попытки, или при последней попытке, когда `CLAUDE_CODE_MAX_RETRIES` позволяет менее трёх; более ранние версии переключаются только при последней попытке.

Начиная с версии 2.1.198, обычная подсказка спиннера подавляется во время повторных попыток. После того как причина ошибки раскрыта, если сбой — это перегрузка 529, строка ниже обратного отсчета также указывает, где проверить статус сервиса: `status.claude.com` на Anthropic API, или хост провайдера или шлюза, указанный в сообщении, на других конфигурациях.

Если данные не поступают на поток ответов в течение 20 секунд, пока запрос все еще ожидает, спиннер показывает `Waiting for API response · will retry in … · check your network` перед началом любой повторной попытки. Запрос еще не завершился с ошибкой: обратный отсчет продолжается до момента, когда Claude Code прерывает зависшее соединение и повторяет попытку, поэтому баннер исчезает самостоятельно, когда данные возобновляются или повторная попытка успешна. Начиная с версии 2.1.185 пороговое значение составляет 20 секунд; в более ранних версиях баннер отображается через 10 секунд с другой формулировкой. Если он появляется при каждой попытке, рассматривайте это как [проблему с сетью](#unable-to-connect-to-api).

Когда вы видите одну из ошибок на этой странице, эти повторные попытки уже исчерпаны, если только она не принадлежит к классу, который не повторяется, такому как сбой проверки сертификата. Вы можете настроить поведение с помощью этих переменных окружения:

| Переменная                                   | По умолчанию   | Эффект                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/ru/env-vars)    | 10             | Количество попыток повтора. Ограничено 15 начиная с версии 2.1.186; начиная с версии 2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` повышает значение по умолчанию и удаляет ограничение. Снизьте его, чтобы быстрее выявлять сбои в скриптах.                                                                                                                                                                                                                                                                                     |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/ru/env-vars) | не установлено | Установите значение `1` в автоматических сеансах, таких как задания CI, чтобы повторять попытки при ошибках емкости `429` и `529` бесконечно вместо отказа после `CLAUDE_CODE_MAX_RETRIES` попыток. Начиная с версии 2.1.199, это также повышает количество повторных попыток по умолчанию для других временных ошибок, таких как ошибки сервера, тайм-ауты и разорванные соединения, до 300, примерно три часа задержки, и удаляет ограничение 15 на `CLAUDE_CODE_MAX_RETRIES`, если вы явно установите эту переменную. |
| [`API_TIMEOUT_MS`](/docs/ru/env-vars)             | 600000         | Тайм-аут для каждого запроса в миллисекундах. Повысьте его для медленных сетей или прокси.                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h2 id="server-errors">
  Ошибки сервера
</h2>

Эти ошибки поступают от поставщика услуг вывода, а не от вашей учетной записи или запроса. На Anthropic API это означает инфраструктуру Anthropic. На Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или пользовательском шлюзе это означает инфраструктуру этого поставщика.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code отображает код состояния и сообщение об ошибке API для любого ответа 5xx. Пример ниже показывает ответ 500 на Anthropic API:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

Завершающее предложение указывает, где проверить состояние службы, и варьируется в зависимости от поставщика. Конфигурации Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry указывают страницу состояния этого поставщика. Пользовательский `ANTHROPIC_BASE_URL` указывает хост шлюза.

Это указывает на неожиданный сбой внутри API. Это не вызвано вашим запросом, параметрами или учетной записью.

**Что делать:**

* Проверьте [status.claude.com](https://status.claude.com) или страницу состояния поставщика, указанную в сообщении, на предмет активных инцидентов
* Подождите минуту, затем отправьте сообщение еще раз. Ваше исходное сообщение все еще находится в беседе, поэтому для длинного запроса вы можете ввести `try again` вместо вставки всего текста.
* Если ошибка сохраняется без опубликованного инцидента, запустите `/feedback`, чтобы Anthropic могла провести расследование с деталями вашего запроса. См. [Report an error](#report-an-error), если `/feedback` недоступен в вашей среде.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

API временно работает на полную мощность для всех пользователей. Claude Code уже несколько раз повторил попытку перед отображением этого сообщения:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

Завершающее предложение варьируется в зависимости от поставщика так же, как ошибка 500 выше.

529 — это не ваш лимит использования и не учитывается в вашей квоте.

**Что делать:**

* Проверьте [status.claude.com](https://status.claude.com) или страницу состояния поставщика, указанную в сообщении, на предмет уведомлений о емкости
* Повторите попытку через несколько минут
* Запустите `/model` и переключитесь на другую модель, чтобы продолжить работу, так как емкость отслеживается для каждой модели. Claude Code предлагает вам это сделать, когда одна модель испытывает особенно высокую нагрузку, например `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

API не ответил до истечения срока подключения.

```text theme={null}
Request timed out
```

Это может произойти в периоды высокой нагрузки или когда модель генерирует очень большой ответ. Время ожидания запроса по умолчанию составляет 10 минут.

**Что делать:**

* Повторите запрос
* Для долгосрочных задач разбейте работу на более мелкие запросы
* Если причина в медленной сети или прокси, увеличьте `API_TIMEOUT_MS`, как описано в [Automatic retries](#automatic-retries)
* Если тайм-ауты частые и ваша сеть в остальном здорова, см. [Network and connection errors](#network-and-connection-errors) ниже

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Потоковый ответ не удался после того, как Claude уже произвел видимый результат. Повторная отправка запроса может привести к двойному выполнению одних и тех же вызовов инструментов, поэтому Claude Code сохраняет то, что уже было передано, и добавляет это уведомление вместо отказа от хода. Какой вариант вы видите, указывает на причину:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* `Server error mid-response`: ошибка перегрузки или 5xx сервера в середине потока. Этот вариант требует Claude Code v2.1.199 или позже; до этого этот случай отбрасывал частичный результат и сообщал весь ход как ошибку.
* `Connection closed mid-response`: соединение разорвалось.
* `Response stalled mid-stream`: поток перестал отправлять данные.

**Что делать:**

* Прочитайте ответ, который был передан. Ничего не потеряно, но последние предложения или вызовы инструментов могут отсутствовать.
* Ответьте `continue`, чтобы Claude продолжил с того места, где остановился
* Если одна и та же ошибка появляется до какого-либо видимого результата, Claude Code повторяет запрос вместо его завершения. См. [Automatic retries](#automatic-retries).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

Модель, которую [auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) использует для классификации действий, не смогла принять решение, поэтому auto mode не одобрила действие автоматически. Сообщение, которое вы видите, зависит от того, почему классификатор не сработал.

Чтения, поиски и редактирования в вашем рабочем каталоге пропускают классификатор, поэтому они продолжают работать во всех этих случаях.

Когда модель классификатора перегружена:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**Что делать:**

* Повторите попытку через несколько секунд; Claude видит то же сообщение и обычно повторяет попытку самостоятельно
* Если повторные попытки продолжают не удаваться, продолжайте с задачами только для чтения и вернитесь к заблокированному действию позже
* Это временно и не связано с [auto mode eligibility](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode); вам не нужно менять параметры

Когда классификатор вернул непарсируемый ответ:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Что делать:**

* Повторите действие; это обычно успешно при следующей попытке
* Запустите `claude --debug` и повторите действие, чтобы увидеть основной ответ классификатора в журнале отладки

Когда отдельная проверка безопасности API заблокировала запрос классификатора из-за более раннего содержимого беседы:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**Что делать:**

* Это не решение о вашем действии. Содержимое, уже находящееся в вашей беседе, вызвало фильтр безопасности на API, когда auto mode отправил беседу классификатору
* Повторная попытка не поможет; то же содержимое беседы снова вызовет фильтр
* Переключитесь на другой [permission mode](/docs/ru/permission-modes), чтобы вы могли одобрить действие при появлении запроса, или начните новую беседу без содержимого, вызывающего проблему

Когда беседа выросла больше, чем контекстное окно классификатора:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

В интерактивном сеансе auto mode возвращается к обычному запросу разрешения для этого действия, чтобы вы могли одобрить или отклонить его вручную. В [non-interactive mode](/docs/ru/headless) запуск прерывается, потому что стенограмма только растет и повторная попытка не может быть успешной.

**Что делать:**

* Одобрите или отклоните действие в появившемся запросе
* Запустите `/compact`, чтобы уменьшить размер беседы, чтобы последующие действия снова поместились в окне классификатора

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

Запрос API [subagent](/docs/ru/sub-agents) не удался окончательно, например, потому что был достигнут лимит использования или повторные попытки для ошибки сервера закончились, поэтому subagent остановился до завершения своей задачи. Это сообщение требует Claude Code v2.1.199 или позже; до этого текст ошибки API был возвращен Claude как если бы это был результат subagent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Что делать:**

* Сопоставьте деталь ошибки после двоеточия с ее собственным разделом на этой странице, например [Usage limits](#usage-limits) или [Server errors](#server-errors), и следуйте шагам этого раздела
* После того как основная ошибка исчезнет, попросите Claude повторить задачу или [resume the subagent](/docs/ru/sub-agents#resume-subagents)

Когда ограничение скорости, перегрузка или ошибка сервера прерывает foreground subagent, который уже произвел текстовый результат, Claude получает этот частичный результат, отмеченный как неполный, вместо этой ошибки. Subagent, единственным результатом которого были вызовы инструментов, также получает эту ошибку; в v2.1.199 эта форма возвращала пустой частичный результат вместо этого. См. [API errors in subagents](/docs/ru/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Ограничения использования
</h2>

Эти ошибки означают, что достигнута квота, связанная с вашей учетной записью или планом. Они отличаются от [ошибок сервера](#server-errors), которые влияют на всех.

<h3 id="youve-hit-your-session-limit">
  Вы достигли лимита сеанса
</h3>

Планы подписки включают скользящий лимит использования. Когда он исчерпывается, вы видите одно из этих сообщений:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code блокирует дальнейшие запросы до времени сброса, указанного в сообщении. Лимиты сеанса и недельного использования являются общими для всех моделей, поэтому переключение моделей не восстанавливает доступ. Лимит Opus применяется только к запросам Opus, поэтому переключение на другую модель с помощью `/model` позволяет вам продолжить работу.

Использование учитывается в отношении лимитов сеанса и недельного использования одновременно. Одиночный всплеск интенсивной активности, такой как крупный fanout рабочего процесса, может исчерпать недельный лимит до того, как окно сеанса сбросится.

**Что делать:**

* Дождитесь времени сброса, указанного в ошибке
* Для лимита Opus запустите `/model` и переключитесь на другую модель, чтобы продолжить работу
* Запустите `/usage` для просмотра лимитов вашего плана и времени их сброса
* Запустите `/usage-credits` для покупки дополнительного использования на Pro и Max, или для запроса у администратора на Team и Enterprise. См. [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) для информации о выставлении счетов.
* Для обновления вашего плана на более высокие базовые лимиты см. [claude.com/pricing](https://claude.com/pricing)

Чтобы отслеживать оставшийся лимит перед его исчерпанием, добавьте поля `rate_limits` в [пользовательскую строку состояния](/docs/ru/statusline#rate-limit-usage), или в приложении Desktop нажмите на [кольцо использования](/docs/ru/desktop#check-usage) рядом с выбором модели.

<h3 id="usage-credits-required-for-1m-context">
  Требуются кредиты использования для контекста 1M
</h3>

Выбранная модель использует расширенное окно контекста на 1M токенов, и ваш план включает его только через кредиты использования.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

Это проверка прав доступа, а не исчерпание квоты. Она срабатывает даже когда ваши лимиты сеанса и недельного использования имеют оставшуюся емкость. См. [Extended context](/docs/ru/model-config#extended-context) для информации о том, какие планы включают контекст 1M напрямую и какие требуют кредитов использования.

Когда эта ошибка появляется в середине разговора, потому что контекст вырос более 200K токенов, Claude Code автоматически сжимает разговор обратно под стандартный лимит контекста и сохраняет сеанс на этом уровне впоследствии, поэтому никаких действий не требуется. На версиях до v2.1.172 ошибка повторялась при каждом последующем запросе, включая `/compact`; запустите `/clear` на этих версиях для восстановления. Приведенные ниже шаги применяются, когда вы явно выбрали модель `[1m]`.

**Что делать:**

* Запустите `/model` и выберите вариант без суффикса `[1m]` для возврата к стандартному окну контекста
* Запустите `/usage-credits` для включения тарифицированного выставления счетов для варианта 1M на Pro и Max, или для запроса у администратора на Team и Enterprise
* Если ошибка сохраняется после `/model`, ID модели 1M может быть установлен в другом месте. См. [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) для проверки мест конфигурации в порядке приоритета.
* Чтобы полностью удалить варианты 1M из выбора модели, установите [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/ru/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  Сервер временно ограничивает запросы
</h3>

API применил кратковременное дросселирование, которое не связано с квотой вашего плана.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code различает это от вашего лимита плана по отсутствию унифицированных заголовков квоты, которые несет реальный ответ лимита. Начиная с v2.1.199 это [автоматически повторяется](#automatic-retries) с отступом перед отображением, независимо от способа аутентификации. На более ранних версиях сеанс, подписанный на подписку claude.ai, не прошел ход при первом возникновении; только API ключ и аутентификация Enterprise повторили попытку.

**Что делать:**

* Подождите немного и попробуйте снова
* Проверьте [status.claude.com](https://status.claude.com) если проблема сохраняется

<h3 id="request-rejected-429">
  Запрос отклонен (429)
</h3>

Вы достигли лимита скорости, настроенного для вашего API ключа, проекта Amazon Bedrock или проекта Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

Завершающее предложение указывает, где проверить здоровье сервиса и варьируется в зависимости от поставщика. Конфигурации Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry указывают на страницу статуса этого поставщика вместо страницы статуса Anthropic. Пользовательский `ANTHROPIC_BASE_URL` указывает на хост шлюза.

**Что делать:**

* Запустите `/status` и подтвердите, что активные учетные данные - это те, которые вы ожидаете. Случайный `ANTHROPIC_API_KEY` в вашей среде может маршрутизировать запросы через ключ низкого уровня вместо вашей подписки.
* Проверьте консоль вашего поставщика для активных лимитов и запросите более высокий уровень, если необходимо
* Для API ключей Anthropic см. [rate limits reference](https://platform.claude.com/docs/en/api/rate-limits) для информации о том, как работают уровни и как установить ограничения расходов для каждого рабочего пространства
* Снизьте параллелизм: понизьте [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/ru/env-vars), избегайте запуска множества параллельных подагентов, или переключитесь на меньшую модель с `/model` для высокообъемных скриптовых запусков

<h3 id="credit-balance-is-too-low">
  Баланс кредитов слишком низкий
</h3>

Ваша организация Console исчерпала предоплаченные кредиты.

```text theme={null}
Credit balance is too low
```

**Что делать:**

* Добавьте кредиты на [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), и рассмотрите возможность включения автоматической перезагрузки там, чтобы баланс пополнялся до того, как он упадет до нуля
* Переключитесь на аутентификацию подписки с `/login` если у вас есть план Pro, Max, Team или Enterprise
* Установите ограничения расходов для каждого рабочего пространства в Console, чтобы предотвратить истощение баланса организации одним проектом. См. [Manage costs effectively](/docs/ru/costs).

<h2 id="authentication-errors">
  Ошибки аутентификации
</h2>

Эти ошибки означают, что Claude Code не может подтвердить вашу личность перед API. Запустите `/status` в любой момент, чтобы увидеть, какие учетные данные в настоящее время активны.

<h3 id="not-logged-in">
  Not logged in
</h3>

Для этого сеанса нет действительных учетных данных.

```text theme={null}
Not logged in · Please run /login
```

**Что делать:**

* Запустите `/login` для аутентификации с помощью вашей подписки Claude или учетной записи Console
* Если вы ожидали, что переменная окружения будет вас аутентифицировать, убедитесь, что `ANTHROPIC_API_KEY` установлена и экспортирована в оболочке, где вы запустили `claude`
* Для CI или автоматизации, где интерактивный вход невозможен, настройте скрипт [`apiKeyHelper`](/docs/ru/settings#available-settings), который получает ключ при запуске
* См. [Authentication precedence](/docs/ru/authentication#authentication-precedence), чтобы понять, какие учетные данные использует Claude Code, когда присутствует несколько

Если вам предлагается войти повторно, см. [Not logged in or token expired](/docs/ru/troubleshoot-install#not-logged-in-or-token-expired) для исправления системных часов и macOS Keychain.

<h3 id="could-not-resolve-authentication-method">
  Could not resolve authentication method
</h3>

Сеанс достиг клиента API без каких-либо учетных данных. Это появляется в [background sessions](/docs/ru/agent-view), облачных сеансах и контекстах Agent SDK, где проверка интерактивного входа не выполняется перед первым запросом.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

До версии 2.1.174 фоновый или облачный сеанс, назначенный неактивному предварительно инициализированному рабочему процессу, мог завершиться ошибкой таким образом, даже если были настроены действительные учетные данные. Обновитесь для восстановления. В текущих версиях ошибка означает, что рабочему процессу не было доступно никаких учетных данных.

**Что делать:**

* Обновитесь до версии 2.1.174 или более поздней, если это появляется в фоновом или облачном сеансе и ваши учетные данные уже настроены
* Убедитесь, что `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` или учетные данные вашего облачного провайдера установлены в окружении, которое запускает рабочий процесс, а не только в вашей интерактивной оболочке
* Для Agent SDK см. [authentication setup](/docs/ru/agent-sdk/overview#get-started)
* Запустите `/status` в интерактивном сеансе в том же окружении, чтобы подтвердить, какой источник учетных данных разрешается

<h3 id="invalid-api-key">
  Invalid API key
</h3>

Переменная окружения `ANTHROPIC_API_KEY` или скрипт `apiKeyHelper` вернули ключ, который API отклонил.

```text theme={null}
Invalid API key · Fix external API key
```

**Что делать:**

* Проверьте опечатки и убедитесь, что ключ не был отозван в [Console](https://platform.claude.com/settings/keys)
* Запустите `env | grep ANTHROPIC` в той же оболочке. Такие инструменты, как direnv, плагины dotenv shell и терминалы IDE, могут загружать устаревший ключ из файла `.env` в вашем проекте без явной установки.
* Отмените установку `ANTHROPIC_API_KEY` и запустите `/login` для использования аутентификации подписки
* Если ключ поступает из скрипта [`apiKeyHelper`](/docs/ru/settings#available-settings), запустите скрипт напрямую, чтобы подтвердить, что он выводит действительный ключ на stdout
* Запустите `/status`, чтобы подтвердить, какой источник учетных данных на самом деле использует Claude Code

<h3 id="your-apikeyhelper-script-is-failing">
  Your apiKeyHelper script is failing
</h3>

Команда, настроенная в параметре [`apiKeyHelper`](/docs/ru/settings#available-settings), завершилась с ошибкой, истекла по времени или ничего не вывела на stdout. Без ключа из скрипта запрос достигает API с заполнителем учетных данных, и API отклоняет его с `401`.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code повторно запускает скрипт и повторяет попытку запроса до двух раз перед отображением этого сообщения, поэтому сбой проявляется в течение трех попыток. До версии 2.1.208 Claude Code тратил весь [retry budget](#automatic-retries) на повторную отправку запроса с заполнителем учетных данных, а затем сообщал об общей ошибке аутентификации `401` вместо сбоя скрипта.

Запуск `/login` здесь не помогает: вывод помощника [имеет приоритет](/docs/ru/authentication#authentication-precedence) над сохраненным входом, пока параметр присутствует.

**Что делать:**

* Запустите команду, настроенную в `apiKeyHelper`, непосредственно в вашей оболочке, чтобы воспроизвести сбой
* Если команда сообщает об истекшей сессии, повторно аутентифицируйтесь у вашего поставщика учетных данных, например, снова войдя в ваш SSO или хранилище секретов
* Исправьте команду так, чтобы она выводила ключ на stdout и выходила с кодом 0. См. [rotate credentials with apiKeyHelper](/docs/ru/llm-gateway-connect#rotate-credentials-with-apikeyhelper) для рабочей настройки.
* Запустите `/status`, чтобы подтвердить, что `apiKeyHelper` является активным источником учетных данных. Каждый раз, когда команда не выполняется, ее код выхода и вывод ошибки появляются в панели `Cloud authentication` в терминале.

<h3 id="this-organization-has-been-disabled">
  This organization has been disabled
</h3>

Устаревший `ANTHROPIC_API_KEY` из отключенной организации Console переопределяет вашу подписку входа.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

Переменные окружения имеют приоритет над `/login`, поэтому ключ, экспортированный в профиль вашей оболочки или загруженный из файла `.env`, используется даже если у вас есть рабочая подписка Pro или Max. В неинтерактивном режиме (`-p`) ключ всегда используется, когда он присутствует.

**Что делать:**

* Отмените установку `ANTHROPIC_API_KEY` в текущей оболочке и удалите его из профиля вашей оболочки, затем перезапустите `claude`
* Запустите `/status` после этого, чтобы подтвердить, что активные учетные данные — это ваша подписка
* Если переменная окружения не установлена и ошибка сохраняется, отключенная организация — это та, которая связана с вашим `/login`. Свяжитесь с поддержкой или войдите с другой учетной записью.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Your organization has disabled API key authentication
</h3>

Это сообщение требует Claude Code версии 2.1.169 или более поздней. Администратор организации Console отключил аутентификацию по ключу API, поэтому API отклоняет ключ, который отправляет Claude Code. Подсказка восстановления после `·` варьируется в зависимости от того, откуда поступил ключ:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Переменные окружения и `apiKeyHelper` имеют приоритет над `/login`, поэтому запуск только `/login` не помогает, пока один из них все еще предоставляет ключ. См. [Authentication precedence](/docs/ru/authentication#authentication-precedence).

**Что делать:**

* Если сообщение называет `ANTHROPIC_API_KEY`, отмените его установку в текущей оболочке и удалите его из профиля вашей оболочки или файла `.env`, затем перезапустите `claude`
* Если сообщение называет `apiKeyHelper`, удалите параметр [`apiKeyHelper`](/docs/ru/settings#available-settings) из вашего `settings.json`
* Запустите `/login` для входа с помощью вашей учетной записи claude.ai
* Запустите `/status` после этого, чтобы подтвердить, что активные учетные данные — это ваша подписка, а не ключ API
* Если вам нужна аутентификация по ключу API для автоматизации, попросите администратора вашей организации повторно включить ее в Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Your organization has disabled Claude subscription access
</h3>

Ваша организация Claude не позволяет входить в Claude Code с помощью подписки. Повторный запуск `/login` с той же учетной записью возвращает ту же ошибку.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Это параметр организации на стороне сервера, поэтому его нельзя переопределить из локальных параметров, переменных окружения или флагов CLI.

Agent SDK и неинтерактивный режим `-p` представляют это как код ошибки `oauth_org_not_allowed`.

**Что делать:**

* Попросите администратора включить доступ Claude Code для вашей организации
* Аутентифицируйтесь с помощью ключа API Console вместо вашей подписки. См. [Claude Console authentication](/docs/ru/authentication#claude-console-authentication) для настройки.
* Если вы администратор и не видите опцию для включения доступа, свяжитесь с [Anthropic support](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Routines are disabled by your organization's policy
</h3>

Владелец в вашей организации Team или Enterprise отключил routines на уровне организации. Ошибка появляется при попытке создать или запустить routine, включая из `/schedule` и пользовательского интерфейса [Routines](/docs/ru/routines) на claude.ai/code.

```text theme={null}
Routines are disabled by your organization's policy.
```

Это параметр на стороне сервера, поэтому его нельзя переопределить из локальных параметров, переменных окружения или флагов CLI.

**Что делать:**

* Попросите владельца в вашей организации включить переключатель **Routines** на [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Для одноразовой запланированной работы, которая не требует routines на уровне организации, см. [scheduled tasks](/docs/ru/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control requires the Anthropic API
</h3>

Сеанс не взаимодействует с Anthropic API напрямую, поэтому нет бэкенда claude.ai для [Remote Control](/docs/ru/remote-control) для сопряжения.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

Это появляется на Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry. Начиная с версии 2.1.196, это также появляется, когда [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars) указывает на хост, отличный от `api.anthropic.com`, такой как [LLM gateway](/docs/ru/llm-gateway) или прокси, даже если вы входите с claude.ai.

**Что делать:**

* Отмените установку `ANTHROPIC_BASE_URL` и перезагрузите сеанс, или запустите Remote Control из сеанса, который взаимодействует с Anthropic API напрямую
* Для этого и других сообщений запуска Remote Control см. [Troubleshoot Remote Control](/docs/ru/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  OAuth token revoked or expired
</h3>

Ваш сохраненный вход больше не действителен. Отозванный токен означает, что вы вышли везде или администратор удалил доступ; истекший токен означает, что автоматическое обновление не удалось в середине сеанса.

Оба сообщения сообщают об отклонении, которое API вернул для запроса, отправленного Claude Code. Когда сохраненный вход уже был очищен после неудачного обновления, вы видите [Login expired](#login-expired) вместо этого.

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**Что делать:**

* Запустите `/login` для повторного входа
* Если ошибка возвращается в том же сеансе после повторной аутентификации, сначала запустите `/logout` для полной очистки сохраненного токена, затем `/login`
* Для повторных запросов на вход при запусках см. проверки системных часов и macOS Keychain в [Troubleshooting](/docs/ru/troubleshoot-install#not-logged-in-or-token-expired)
* Для других сбоев, включая `403 Forbidden` и проблемы с браузером OAuth, см. [Login and authentication](/docs/ru/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  Login expired
</h3>

Claude Code попытался обновить ваш сохраненный вход claude.ai или Claude Console, и служба OAuth отклонила сохраненный токен обновления, поэтому Claude Code очистила сохраненные учетные данные. После этого каждый запрос останавливается локально перед тем, как достичь API, потому что только `/login` может создать новые учетные данные. До версии 2.1.206 Claude Code отправляла запрос в любом случае с любыми учетными данными, оставшимися в окружении, и каждая модель затем не выполнялась с [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) или 401 вместо запроса на вход.

```text theme={null}
Login expired · Please run /login
```

В [non-interactive mode](/docs/ru/headless) (`-p`) и [Agent SDK](/docs/ru/agent-sdk/overview) сообщение читается следующим образом, и код структурированной ошибки — `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Это не то же состояние, что [OAuth token revoked or expired](#oauth-token-revoked-or-expired). Эти сообщения сообщают о 401, который вернул API. Claude Code сама создает `Login expired` для входа, который она уже не смогла обновить, поэтому она не отправляет запрос.

Сеансы, аутентифицированные с помощью ключа API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ru/env-vars) или поставщика третьей стороны, не используют сохраненный вход и никогда не видят это сообщение.

**Что делать:**

* Запустите `/login` для повторного входа. Повторная попытка без входа показывает то же сообщение при каждом запросе.
* В неинтерактивном режиме запустите `claude` в том же окружении, завершите `/login`, затем повторно запустите вашу команду. Для автоматизации, которая не может войти интерактивно, аутентифицируйтесь с помощью `ANTHROPIC_API_KEY` или [generate a long-lived token with `claude setup-token`](/docs/ru/authentication#generate-a-long-lived-token).
* Если вход продолжает не выполняться, см. [Login and authentication](/docs/ru/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  OAuth scope requirement
</h3>

Сохраненный токен предшествует области разрешений, которая требуется более новой функции. Вы видите это чаще всего из `/usage` и индикатора использования в строке состояния:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Что делать:**

* Запустите `/login` для получения нового токена с текущими областями. Вам не нужно предварительно выходить.

<h3 id="aws-credentials-expired-or-invalid">
  AWS credentials expired or invalid
</h3>

Это сообщение требует Claude Code версии 2.1.198 или более поздней и появляется только когда [`awsAuthRefresh`](/docs/ru/amazon-bedrock#advanced-credential-configuration) установлен в вашем файле параметров. Ваш токен сеанса AWS истек или был отклонен, и автоматическое обновление, которое уже запустил Claude Code, не создало учетные данные, которые API принимает. Это появляется при 401 от [Claude Platform on AWS](/docs/ru/claude-platform-on-aws) или [Mantle endpoint](/docs/ru/amazon-bedrock#use-the-mantle-endpoint), что является тем, как эти провайдеры сообщают об истекшем токене безопасности.

Подсказка действия в середине называет команду `awsAuthRefresh` из ваших параметров, поэтому она варьируется. Стабильная часть — это начальная `AWS credentials expired or invalid`:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Без настроенного `awsAuthRefresh`, тот же 401 показывает вместо этого общее сообщение `Please run /login`, которое не может обновить учетные данные AWS.

**Что делать:**

* Запустите команду `awsAuthRefresh`, названную в сообщении, такую как `aws sso login --profile myprofile`, в другом терминале и завершите вход в браузер, затем повторите попытку
* В интерактивном сеансе запустите `/login`, выберите **3rd-party platform**, затем выберите **Claude Platform on AWS · refresh credentials** в разделе **Using 3rd-party platforms** для запуска той же команды без перезагрузки Claude Code. См. [Configure AWS credentials](/docs/ru/claude-platform-on-aws#1-configure-aws-credentials)
* Если ошибка повторяется после успешного выполнения команды обновления, подтвердите, что идентификатор действителен вне Claude Code с помощью `aws sts get-caller-identity` в той же оболочке и профиле

<h3 id="aws-authentication-failed">
  AWS authentication failed
</h3>

Это сообщение требует Claude Code версии 2.1.198 или более поздней и появляется только когда [`awsAuthRefresh`](/docs/ru/amazon-bedrock#advanced-credential-configuration) установлен в вашем файле параметров. Ваш провайдер AWS вернул 403, или [Amazon Bedrock](/docs/ru/amazon-bedrock) вернул 401.

Claude Code не может определить, какую причину вы получили. Amazon Bedrock сообщает об истекшем токене безопасности как 403, но 403 также является тем, как он сообщает об отказе в авторизации, такой как `AccessDeniedException` из-за отсутствующего разрешения IAM или модели, которая не включена для вашей учетной записи.

401 от Amazon Bedrock также попадает сюда, а не в [AWS credentials expired or invalid](#aws-credentials-expired-or-invalid), потому что Amazon Bedrock не сообщает об истекшем токене как 401. 401 от этой конечной точки обычно поступает из чего-то другого в пути запроса, такого как корпоративный прокси.

Обновление учетных данных исправляет истекший токен и не может исправить другие причины, поэтому сообщение предлагает оба:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

Подсказка действия в середине называет команду `awsAuthRefresh` из ваших параметров, поэтому она варьируется. Стабильная часть — это начальная `AWS authentication failed`.

**Что делать:**

* Запустите команду `awsAuthRefresh`, названную в сообщении, или `aws sso login`, на случай, если причиной является истекшие учетные данные
* Если ваши учетные данные актуальны, подтвердите, что разрешения IAM в [IAM configuration](/docs/ru/amazon-bedrock#iam-configuration) присоединены к идентификатору, который вы используете, и что выбранная модель включена для вашей учетной записи и региона
* Запустите `aws sts get-caller-identity` для подтверждения того, какой идентификатор используют ваши запросы; устаревший `AWS_PROFILE` или профиль по умолчанию — это частая причина несоответствия разрешений

<h3 id="aws-default-chain-credential-resolve-timed-out">
  AWS default-chain credential resolve timed out
</h3>

Цепь поставщика учетных данных AWS по умолчанию не создала учетные данные в течение 60 секунд, поэтому Claude Code остановила разрешение и не выполнила запрос. Сбой — это локальное разрешение учетных данных: запрос никогда не достиг [Amazon Bedrock](/docs/ru/amazon-bedrock), [Claude Platform on AWS](/docs/ru/claude-platform-on-aws) или [Mantle endpoint](/docs/ru/amazon-bedrock#use-the-mantle-endpoint). Claude Code очищает свой [credential cache](/docs/ru/amazon-bedrock#credential-caching-and-resolution-timeout) и повторяет попытку перед тем, как эта ошибка проявляется, поэтому к тому времени, когда вы ее видите, цепь зависла при повторных попытках.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

Частые причины — это команда `credential_process` в вашем профиле AWS, которая ждет входа, который она не может получить, и контейнер или виртуальная машина, служба метаданных экземпляра (IMDS) которых никогда не отвечает на зонд цепи. До версии 2.1.207 зависшая цепь оставляла запрос ожидающим неопределенно долго вместо того, чтобы не выполняться с этим сообщением.

**Что делать:**

* Запустите `aws sts get-caller-identity` в той же оболочке с тем же `AWS_PROFILE`. Если он также зависает, исправьте профиль; команда `credential_process`, которая запрашивает интерактивно, — это частая причина.
* Завершите шаг входа перед запуском Claude Code, например `aws sso login --profile myprofile`, чтобы цепь разрешилась из локального кэша SSO вместо ожидания потока браузера
* Если ваша цепь запускает интерактивный вход, который законно требует более 60 секунд, такой как SSO с MFA через оболочку, как `aws-vault`, повысьте лимит в миллисекундах с помощью [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ru/env-vars)

<h2 id="network-and-connection-errors">
  Ошибки сети и подключения
</h2>

Эти ошибки означают, что сетевой запрос от Claude Code не смог достичь пункта назначения, или что-то между Claude Code и API изменило ответ на пути обратно. Обычно они возникают в вашей локальной сети, прокси или брандмауэре, либо в политике сети облачной среды.

<h3 id="unable-to-connect-to-api">
  Unable to connect to API
</h3>

TCP-соединение с API не удалось или никогда не завершилось.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

Распространённые причины включают отсутствие доступа в Интернет, VPN, который блокирует `api.anthropic.com`, или требуемый корпоративный прокси, который не настроен.

**Что делать:**

* Убедитесь, что вы можете достичь хоста API из той же оболочки, выполнив `curl -I https://api.anthropic.com`. В Windows PowerShell используйте `curl.exe -I https://api.anthropic.com`, чтобы встроенный псевдоним `Invoke-WebRequest` не использовался.
* Если вы находитесь за корпоративным прокси, установите `HTTPS_PROXY` перед запуском Claude Code и см. [Конфигурация сети](/docs/ru/network-config)
* Если вы маршрутизируете через шлюз LLM или ретранслятор, установите [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars) на его адрес. См. [Подключение Claude Code к шлюзу LLM](/docs/ru/llm-gateway-connect) для настройки.
* Убедитесь, что ваш брандмауэр разрешает хосты, указанные в [Требования к доступу в сеть](/docs/ru/network-config#network-access-requirements)
* Перебойные сбои [автоматически повторяются](#automatic-retries); постоянные сбои указывают на локальную проблему с сетью

Если `curl` работает успешно, но Claude Code всё ещё не работает, причина обычно находится между средой выполнения и сетью, а не в самой сети:

* На Linux и WSL проверьте `/etc/resolv.conf` на наличие недостижимого сервера имён. WSL в частности может унаследовать неработающий распознаватель от хоста.
* На macOS клиент VPN, который был отключен или удален, может оставить интерфейс туннеля или правило маршрутизации. Проверьте `ifconfig` на наличие устаревших интерфейсов `utun` и удалите сетевое расширение VPN в Параметрах системы.
* Docker Desktop и аналогичные среды выполнения контейнеров могут перехватывать исходящий трафик. Закройте их и повторите попытку, чтобы исключить это.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock streaming response has an unexpected content-type
</h3>

Шлюз или прокси между Claude Code и [Amazon Bedrock](/docs/ru/amazon-bedrock) преобразует тело потокового ответа или его заголовок `Content-Type`. Amazon Bedrock передаёт ответы потоком как `application/vnd.amazon.eventstream`, и Claude Code отклоняет успешный потоковый ответ, который сообщает о другом типе содержимого вместо декодирования тела, которое он не может прочитать. Запрос не повторяется.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

До версии 2.1.208 та же неправильная конфигурация проявлялась как `API Error: Truncated event message received` после того, как весь ответ был буферизирован.

**Что делать:**

* Настройте шлюз для передачи тела ответа `InvokeModelWithResponseStream` и его заголовка `Content-Type` без изменений. Промежуточный узел, который повторно отправляет поток как события, отправляемые сервером, является распространённой причиной.
* Если шлюз переписывает только заголовок и передаёт двоичное тело без изменений, установите [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/ru/env-vars) для пропуска проверки до исправления шлюза. См. [Ошибки потоковой передачи за шлюзом или прокси](/docs/ru/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  SSL certificate errors
</h3>

Прокси или устройство безопасности в вашей сети перехватывает трафик TLS с помощью собственного сертификата, и Claude Code ему не доверяет.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

Начиная с версии 2.1.199, сбой проверки сертификата не повторяется, поэтому эта ошибка появляется при первой попытке вместо того, чтобы появиться после полного [бюджета повторных попыток](#automatic-retries). Более ранние версии потратили несколько минут на повторные попытки перед её отображением. Переходящие условия TLS, такие как истечение времени ожидания при установлении соединения, всё ещё повторяются.

Во время `/login` и проверки подключения при запуске тот же сбой сообщается с кодом OpenSSL и исправлением в строке:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**Что делать:**

* Экспортируйте пакет CA вашей организации и укажите Claude Code на него с помощью `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* См. [Конфигурация сети](/docs/ru/network-config#custom-ca-certificates) для полных инструкций по настройке
* Не устанавливайте `NODE_TLS_REJECT_UNAUTHORIZED=0`, что полностью отключает проверку сертификата

<h3 id="host-not-allowed-in-a-cloud-session">
  Host not allowed in a cloud session
</h3>

Исходящий HTTP-запрос из облачной сессии или подпрограммы был заблокирован политикой сети среды.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Вы также можете увидеть сертификат TLS, который не соответствует реальному сертификату пункта назначения. Облачная среда маршрутизирует исходящий трафик через прокси, который применяет политику сети, поэтому несоответствующий сертификат означает, что прокси завершил соединение, а не пункт назначения.

Это не проблема сети на стороне клиента. Облачные сессии и [подпрограммы](/docs/ru/routines) работают внутри изолированной среды, исходящий трафик которой фильтруется в соответствии со списком разрешений среды. Среда **Default** использует доступ **Trusted**, который разрешает [список разрешений по умолчанию](/docs/ru/claude-code-on-the-web#default-allowed-domains) реестров пакетов, API поставщиков облачных услуг, реестров контейнеров и распространённых доменов разработки, но блокирует всё остальное.

**Что делать:**

* Откройте подпрограмму для редактирования или запустите облачную сессию. Выберите облачный значок, показывающий имя вашей среды, например **Default**, чтобы открыть селектор. Наведите указатель на вашу среду и нажмите значок параметров.
* В диалоговом окне **Update cloud environment** измените **Network access** с **Trusted** на **Custom**, затем добавьте заблокированный домен в **Allowed domains**. Введите один домен в строку. Установите флажок **Also include default list of common package managers**, чтобы сохранить [список разрешений по умолчанию](/docs/ru/claude-code-on-the-web#default-allowed-domains) вместе с вашими пользовательскими доменами. Выберите **Full** вместо этого, если вы хотите неограниченный доступ.
* Нажмите **Save changes**. Следующий запуск использует обновленный список разрешений.

См. [Доступ в сеть](/docs/ru/claude-code-on-the-web#network-access) для уровней доступа и списка разрешений по умолчанию. Локальные сессии CLI не затронуты этой политикой.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Couldn't reconnect to your Remote Control session
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Возобновление с помощью `claude --resume` или `claude --continue` переподключается к сессии [Remote Control](/docs/ru/remote-control), записанной в этом разговоре. Это сообщение означает, что переподключение не удалось по причине, которая может быть временной, такой как сетевой сбой или ошибка сервера, поэтому Claude Code не может подтвердить, существует ли удалённая сессия. Ваша локальная сессия продолжает работать без Remote Control.

**Что делать:**

* Запустите `/remote-control` для повторной попытки подключения
* Запустите Claude Code без `--resume` для создания новой сессии Remote Control
* Для других сообщений при запуске Remote Control см. [Troubleshoot Remote Control](/docs/ru/remote-control#troubleshooting)

Вы не увидите это сообщение, когда сервер подтвердит, что предыдущая сессия больше не существует; Claude Code создаст новую в этом случае. До версии 2.1.200 любой сбой переподключения создавал новую сессию Remote Control, которая оставляла дополнительные сессии в списке сессий на claude.ai/code.

<h2 id="request-errors">
  Ошибки запроса
</h2>

Эти ошибки связаны с содержимым вашего запроса. Большинство из них возвращаются API после отклонения запроса; несколько производятся локально Claude Code перед отправкой запроса.

<h3 id="prompt-is-too-long">
  Prompt is too long
</h3>

Разговор плюс прикреплённые файлы превышают контекстное окно модели.

```text theme={null}
Prompt is too long
```

**Что делать:**

* Запустите `/compact` для суммирования предыдущих ходов и освобождения места, или `/clear` для начала заново
* Запустите `/context` для просмотра разбивки того, что потребляет окно: системный prompt, инструменты, файлы памяти и сообщения
* Отключите MCP серверы, которые вы не используете, с помощью `/mcp disable <name>` для удаления их определений инструментов из контекста
* Обрежьте большие файлы памяти `CLAUDE.md`, или переместите инструкции в [правила с областью действия пути](/docs/ru/memory#path-specific-rules), которые загружаются только при необходимости
* Подагенты наследуют каждое определение инструмента MCP из родительской сессии, что может заполнить их контекстное окно до первого хода. Отключите MCP серверы, которые вы не используете, перед созданием подагентов.
* Auto-compact включен по умолчанию и обычно предотвращает эту ошибку. Если вы установили [`DISABLE_AUTO_COMPACT`](/docs/ru/env-vars), переактивируйте его или запустите `/compact` вручную перед заполнением окна.

Смотрите [Explore the context window](/docs/ru/context-window) для интерактивного просмотра того, как заполняется контекст.

<h3 id="error-during-compaction-conversation-too-long">
  Error during compaction: Conversation too long
</h3>

`/compact` сам по себе не удался, потому что недостаточно свободного контекста для хранения создаваемого им резюме.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Это может произойти, когда окно уже полно в момент срабатывания auto-compact, или когда вы запускаете `/compact` после появления `Prompt is too long`.

**Что делать:**

* Нажмите Esc дважды, чтобы открыть список сообщений и вернуться на несколько ходов назад. Это удаляет самые последние сообщения из контекста. Затем запустите `/compact` снова.
* Если отступление не освобождает достаточно места, запустите `/clear` для начала новой сессии. Ваш предыдущий разговор сохраняется и может быть переоткрыт с помощью `/resume`.

<h3 id="request-too-large">
  Request too large
</h3>

Тело необработанного запроса превысило лимит байтов API перед токенизацией, обычно из-за большого вставленного файла или вложения.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

Это ограничение размера HTTP запроса, отдельное от [лимита контекстного окна](#prompt-is-too-long).

**Что делать:**

* Нажмите Esc дважды и вернитесь на несколько ходов назад, пройдя ход, который добавил содержимое с избыточным размером
* Ссылайтесь на большие файлы по пути вместо вставки их содержимого, чтобы Claude мог читать их по частям
* Для изображений смотрите [Image was too large](#image-was-too-large) ниже

<h3 id="image-was-too-large">
  Image was too large
</h3>

Вставленное или прикреплённое изображение превышает ограничения размера или размеров API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code заменяет необработанное изображение текстовым заполнителем и повторяет попытку, поэтому последующие сообщения успешны. В версиях до 2.1.142 вставленное изображение могло остаться в разговоре и повторять одну и ту же ошибку при каждом последующем сообщении. Для восстановления в этих версиях нажмите Esc дважды и вернитесь на несколько ходов назад, пройдя ход, где было добавлено изображение.

**Что делать:**

* Измените размер изображения перед вставкой. API принимает изображения размером до 8000 пикселей по самому длинному краю для одного изображения, или 2000 пикселей, когда в контексте много изображений.
* Сделайте более плотный скриншот соответствующей области вместо полного экрана

<h3 id="unable-to-resize-image">
  Unable to resize image
</h3>

Claude Code не смог уменьшить масштаб прикреплённого изображения перед отправкой его в API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code обычно автоматически изменяет размер больших изображений. Эти ошибки означают, что встроенный обработчик изображений не смог загрузиться или вернул ошибку, поэтому изображение не удалось изменить в размер, чтобы соответствовать лимитам API.

**Что делать:**

* Если сообщение просит вас преобразовать изображение, преобразуйте его в PNG, JPEG, GIF или WebP и прикрепите снова. Claude Code может проверить размеры для этих форматов без обработчика изображений.
* Если сообщение сообщает об ограничении размера или размеров, измените размер или перекомпрессируйте изображение ниже этого лимита перед прикреплением.

<h3 id="pdf-errors">
  PDF errors
</h3>

Прикреплённый PDF не удалось обработать.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**Что делать:**

* Для больших PDF попросите Claude прочитать диапазон страниц с помощью инструмента Read вместо прикрепления всего файла, или извлеките текст с помощью инструмента, такого как `pdftotext`, и ссылайтесь на выходной файл по пути
* Для защищённых или недействительных PDF удалите пароль или переэкспортируйте файл из исходного приложения, затем попробуйте снова

<h3 id="extra-inputs-are-not-permitted">
  Extra inputs are not permitted
</h3>

Прокси или LLM шлюз между Claude Code и API удалил заголовок запроса `anthropic-beta`, поэтому API отклонил поля, которые от него зависят.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code отправляет поля только для бета-версии, такие как `context_management`, `effort` и примеры инструментов `input_examples`, вместе с заголовком `anthropic-beta`, который их включает. Когда шлюз пересылает тело, но удаляет заголовок, API видит поля, которые он не распознаёт.

**Что делать:**

* Настройте ваш шлюз для пересылки заголовка `anthropic-beta`. Смотрите [feature pass-through](/docs/ru/llm-gateway-protocol#feature-pass-through) для того, что шлюзы должны пересылать.
* В качестве резервного варианта установите [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ru/env-vars) перед запуском. Это отключает функции, которые требуют заголовка бета-версии, чтобы запросы успешно проходили через шлюз, который не может его пересылать.

<h3 id="theres-an-issue-with-the-selected-model">
  There's an issue with the selected model
</h3>

Имя настроенной модели не было распознано, или ваша учётная запись не имеет доступа к ней. Начиная с v2.1.160 конечная подсказка, показанная здесь в её интерактивной форме, варьируется в зависимости от поверхности.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Что делать:**

* **Interactive CLI**: запустите `/model` для выбора из моделей, доступных вашей учётной записи.
* **Non-interactive mode (`-p`)**: передайте `--model` с действительным псевдонимом или ID, или установите [`ANTHROPIC_MODEL`](/docs/ru/env-vars). Текст ошибки показывает `Run --model` на этой поверхности.
* **Agent SDK**: текст ошибки опускает подсказку, потому что модель установлена программно. Установите [`model` на `Options`](/docs/ru/agent-sdk/typescript#options) в TypeScript или [`ClaudeAgentOptions(model=...)`](/docs/ru/agent-sdk/python#claudeagentoptions) в Python, и обработайте структурированную ошибку `model_not_found` для отображения вашего собственного повтора или средства выбора модели.
* Используйте псевдоним, такой как `sonnet` или `opus`, вместо полного версионного ID. Псевдонимы разрешаются в поддерживаемое значение по умолчанию, поэтому они не устаревают. Смотрите [Model configuration](/docs/ru/model-config).
* Если неправильная модель продолжает возвращаться в CLI, где-то установлен устаревший ID. Проверьте в [порядке приоритета](/docs/ru/model-config#setting-your-model): флаг `--model`, переменная окружения `ANTHROPIC_MODEL`, затем поле `model` в `.claude/settings.local.json`, файл вашего проекта `.claude/settings.json` и `~/.claude/settings.json`. Удалите устаревшее значение, и Claude Code вернётся к умолчанию вашей учётной записи.
* Claude Code сообщает об истёкшем входе claude.ai как [Login expired](#login-expired), а не как об этой ошибке. До v2.1.206 истёкший вход, который больше не удалось обновить, не удавался для каждой модели с этой ошибкой; запустите `/login`, если вы видите это в более старой версии.
* Для развёртываний Google Cloud's Agent Platform смотрите [Google Cloud's Agent Platform troubleshooting](/docs/ru/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Model is not a recognized model id
</h3>

Строка модели, которую вы передали переключателю модели, не является псевдонимом модели, ID модели, который знает эта версия Claude Code, или ID, который начинается с `claude-`. Обычные причины — опечатка в ID, отображаемое имя, такое как `Sonnet 5`, где ожидается ID `claude-sonnet-5`, или псевдоним, который распознают только более новые версии Claude Code. Claude Code немедленно отклоняет переключение. До v2.1.200 Claude Code сохранял строку и не удавался при следующем запросе с [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

Конечная подсказка называет ближайший совпадающий псевдоним или ID модели. Когда ничего не совпадает достаточно близко, она читает `Run /model to see available models.` вместо этого.

Claude Code производит эту ошибку локально в момент запроса переключения, перед любым запросом API. Она применяется, когда модель установлена через метод [Agent SDK](/docs/ru/agent-sdk/typescript) `setModel()` или приложением, таким как [Desktop app](/docs/ru/desktop), которое запускает Claude Code CLI для вас.

**Что делать:**

* Запустите `/model` без аргумента, чтобы открыть средство выбора и выбрать из моделей, доступных вашей учётной записи, затем передайте показанный там псевдоним или ID
* Если вы использовали псевдоним, который поддерживает более новая версия Claude Code, запустите `claude update`. Полный ID, который начинается с `claude-`, проходит эту проверку даже когда модель новее вашей версии Claude Code, поэтому обновление не требуется для них.
* Модель, сохранённая до v2.1.200, не восстанавливается этой проверкой. Если устаревшее значение продолжает возвращаться, удалите его из мест, перечисленных в [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).
* Проверка выполняется только на Anthropic API. На Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/ru/claude-platform-on-aws) и за [LLM gateway](/docs/ru/llm-gateway) или пользовательским `ANTHROPIC_BASE_URL`, ваш поставщик или шлюз определяет имена моделей, поэтому Claude Code принимает любую строку и пропускает её.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus is not available with the Claude Pro plan
</h3>

Ваш активный план подписки не включает выбранную модель.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**Что делать:**

* Запустите `/model` и выберите модель, которую включает ваш план
* Если вы недавно обновили свой план и всё ещё видите это, запустите `/logout`, затем `/login`. Сохранённый токен отражает ваш план на момент входа, поэтому обновление в интернете не вступает в силу в существующей сессии до переаутентификации.
* Смотрите [claude.com/pricing](https://claude.com/pricing) для того, какие модели включает каждый план

<h3 id="model-is-restricted-by-your-organizations-settings">
  Model is restricted by your organization's settings
</h3>

Администратор вашей организации отключил эту модель в консоли администратора claude.ai, или она исключена [`availableModels`](/docs/ru/model-config#restrict-model-selection) списком разрешений в управляемых параметрах. Когда ограниченная модель была установлена с `--model`, `ANTHROPIC_MODEL` или параметром `model`, Claude Code подставляет разрешённую модель и продолжает. Ввод `/model <name>` для ограниченной модели отклоняется с `Run /model to choose a different model.` и сессия сохраняет свою текущую модель.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code рассматривает псевдоним семейства моделей, один из `opus`, `sonnet`, `haiku` или `fable`, как запрос для этого семейства, а не для его новейшей версии. На Anthropic API и на [Claude Platform on AWS](/docs/ru/claude-platform-on-aws) ограниченный псевдоним семейства разрешается в новейшую версию семейства, которую разрешают ваша организация и список разрешений `availableModels`, и уведомление о подстановке называет эту версию. Claude Code отклоняет `/model <alias>` только когда каждая версия семейства ограничена. До v2.1.205 псевдоним семейства был подставлен или отклонен на основе только его новейшей версии, даже когда была разрешена более старая версия того же семейства.

**Что делать:**

* Запустите `/model` для выбора из моделей, которые разрешает ваша организация. Ограниченные модели скрыты от средства выбора.
* Если ограниченная модель была установлена в `--model`, `ANTHROPIC_MODEL` или поле `model` файла параметров, удалите или обновите это значение, чтобы уведомление не повторялось при каждом запуске
* Если вам нужен доступ к ограниченной модели, попросите администратора вашей организации включить её. Смотрите [Organization model restrictions](/docs/ru/model-config#organization-model-restrictions).

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled is not supported for this model
</h3>

Ваша версия Claude Code старше минимума для Sonnet 5, Opus 4.8 или Opus 4.7. CLI отправил конфигурацию мышления, которую модель больше не принимает.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Что делать:**

* Запустите `claude update` и перезагрузите Claude Code. Opus 4.7 требует v2.1.111 или позже. Opus 4.8 требует v2.1.154 или позже. Sonnet 5 требует v2.1.197 или позже
* Если вы не можете обновиться, запустите `/model` и выберите Opus 4.6 или Sonnet 4.6 вместо этого
* Если вы столкнулись с этим в [Agent SDK](/docs/ru/agent-sdk/overview), обновите пакет SDK вместо этого. Opus 4.8 требует TypeScript SDK v0.3.154 или позже и Python SDK v0.2.88 или позже. Sonnet 5 требует TypeScript SDK v0.3.197 или позже

<h3 id="thinking-budget-exceeds-output-limit">
  Thinking budget exceeds output limit
</h3>

Настроенный бюджет расширенного мышления превышает максимальную длину ответа, поэтому для фактического ответа не остаётся места.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code автоматически регулирует эти значения на Anthropic API. Вы обычно видите эту ошибку на Amazon Bedrock или Google Cloud's Agent Platform, когда [`MAX_THINKING_TOKENS`](/docs/ru/env-vars) установлен выше лимита вывода поставщика, или когда режим плана повышает бюджет мышления.

**Что делать:**

* Понизьте `MAX_THINKING_TOKENS`, или повысьте [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/ru/env-vars) выше бюджета мышления
* Смотрите [Extended thinking](/docs/ru/model-config#extended-thinking) для того, как бюджет взаимодействует с длиной вывода

<h3 id="tool-use-or-thinking-block-mismatch">
  Tool use or thinking block mismatch
</h3>

История разговора достигла API в несогласованном состоянии, обычно после прерывания вызова инструмента или редактирования хода в процессе.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Все три варианта означают одно и то же: последовательность блоков `tool_use`, `tool_result` и `thinking` в истории больше не совпадает с тем, что ожидает API.

**Что делать:**

* Если вы используете Opus 4.7 или Opus 4.8, сначала запустите `claude update`. Версии до v2.1.156 могут вызвать эту ошибку при нормальном использовании инструмента, и `/rewind` её не очищает.
* Запустите `/rewind`, или нажмите Esc дважды, чтобы вернуться к контрольной точке перед повреждённым ходом и продолжить оттуда. Смотрите [Checkpointing](/docs/ru/checkpointing) для того, как создаются и восстанавливаются контрольные точки.

<h3 id="usage-policy-refusal">
  Usage Policy refusal
</h3>

API отказался отвечать, потому что содержимое в разговоре вызвало проверку [Usage Policy](https://www.anthropic.com/legal/aup). Сообщение включает ID запроса, который вы можете цитировать в поддержку, если вы считаете, что отказ неправильный.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

Проверка оценивает весь разговор, а не только ваш последний prompt, поэтому отправка нового сообщения в той же сессии обычно повторно вызывает тот же отказ. То же самое применяется после выхода и переоткрытия сессии с `--continue` или `--resume`, так как стенограмма на диске всё ещё содержит вызывающее содержимое. На [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) и [Microsoft Foundry](/docs/ru/microsoft-foundry) это сообщение также охватывает запросы, которые меры безопасности модели отметили как тему кибербезопасности. Смотрите [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

**Что делать:**

* Нажмите Esc дважды или запустите `/rewind` для возврата к контрольной точке перед ходом, который вызвал отказ, затем переформулируйте или примите другой подход. Смотрите [Checkpointing](/docs/ru/checkpointing).
* Если вы не можете определить, какой ход вызвал это, запустите `/clear` для начала новой разговора в том же проекте. Ваш предыдущий разговор сохраняется на диске и остаётся доступным в `/resume`.
* В [non-interactive mode](/docs/ru/headless) (`-p`), где перемотка недоступна, повторите попытку с переформулированным prompt в новой сессии без `--continue`. Проверки политики варьируются в зависимости от модели, поэтому переключение на другую модель с `--model` также может разрешить отказ в некоторых случаях.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Safety measures flagged a cybersecurity topic
</h3>

Меры безопасности модели отметили содержимое в разговоре как тему кибербезопасности. Сообщение называет модель, которая отметила запрос:

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

Сообщение ссылается на [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), который предоставляет доступ для законной работы в области кибербезопасности. Сама защита находится на стороне сервера и предшествует v2.1.203; этот выпуск изменил только формулировку сообщения и страницу, на которую оно ссылается.

То, что вы видите, зависит от вашего поставщика и режима:

* На [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) и [Microsoft Foundry](/docs/ru/microsoft-foundry) флаг кибербезопасности производит сообщение [Usage Policy refusal](#usage-policy-refusal) вместо этого.
* [Non-interactive mode](/docs/ru/headless) опускает предложение `/feedback`.

До v2.1.203 сообщение читалось `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` с последующей ссылкой на форму исключения.

**Что делать:**

* Если ваша работа требует этого содержимого, подайте заявку на доступ через [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Если ваш запрос не был о теме кибербезопасности, запустите `/feedback` для сообщения о ложном срабатывании
* Чтобы продолжить работу в той же сессии, нажмите Esc дважды или запустите `/rewind` для возврата к контрольной точке перед ходом, который вызвал флаг, затем примите другой подход. Смотрите [Checkpointing](/docs/ru/checkpointing).

<h2 id="installation-errors">
  Ошибки установки
</h2>

Эти ошибки появляются при установке или обновлении Claude Code из [скрипта установки](/docs/ru/setup#install-claude-code), `claude install` или `claude update`. Для проблем с `command not found`, PATH, разрешениями и TLS во время установки см. [Устранение неполадок установки и входа](/docs/ru/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  Установка была прервана до завершения
</h3>

Скрипт установки сообщает, когда этап `claude install` завершается сигналом. На Linux код выхода 137 означает, что процесс получил SIGKILL, а на хосте с низким объёмом памяти это обычно означает, что ядро активировало средство защиты от нехватки памяти (OOM killer). Скрипт выводит это объяснение и завершается с кодом 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Для любого другого фатального сигнала и для кода выхода 137 на macOS скрипт выводит `Installation was killed before it could finish (exit code <N>)` с фактическим кодом выхода и опускает объяснение о нехватке памяти. Сообщение поступает из скрипта установки, который используют macOS и Linux, и также охватывает установки внутри WSL; встроенные скрипты установки Windows никогда его не выводят. До версии 2.1.200 скрипт завершался только с простой строкой `Killed` оболочки.

**Что делать:**

* Остановите другие процессы, чтобы освободить память, затем повторно запустите установщик
* Добавьте пространство подкачки или перейдите на экземпляр большего размера. См. [Установка прервана на серверах Linux с низким объёмом памяти](/docs/ru/troubleshoot-install#install-killed-on-low-memory-linux-servers) для команд файла подкачки.

<h3 id="the-connection-dropped-while-downloading-the-update">
  Соединение разорвалось при загрузке обновления
</h3>

Соединение с сервером загрузки закрылось, пока `claude install`, `claude update` или [автоматический обновитель](/docs/ru/setup#auto-updates) загружал двоичный файл Claude Code, и повторные попытки не восстановили соединение. Claude Code повторяет загрузку, когда соединение разрывается, передача зависает или загруженный файл не проходит проверку контрольной суммы, всего до трёх попыток. Завершённая ошибка HTTP, такая как 404, не повторяется, потому что сервер уже ответил. До версии 2.1.202 одно разорванное соединение немедленно приводило к сбою загрузки с простой ошибкой `aborted` вместо повторной попытки.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Текст в скобках указывает, какая попытка не удалась, и основную ошибку сети. `claude update` предваряет сообщение с `Error: Failed to install native update` на stderr.

Загрузка, которая остаётся подключённой, но не завершается в течение 10 минут, завершается с ошибкой `Download timed out: exceeded the total deadline`. Claude Code не повторяет истёкшую по времени загрузку, потому что соединение, которое слишком медленно для завершения в установленный срок, не завершится и при немедленной повторной попытке. Приведённые ниже шаги применяются к обоим сообщениям. До версии 2.1.205 тот же 10-минутный срок сообщался как универсальный `timeout of 600000ms exceeded` HTTP-клиента.

Обычная причина — прокси или шлюз, который закрывает длительную передачу до её завершения. Двоичный файл Claude Code — это большая загрузка, поэтому ограничение соединения прокси, которое никогда не влияет на обычный трафик API, всё равно может его прервать.

**Что делать:**

* Запустите `claude update` снова. На в остальном здоровой сети загрузка обычно успешна при следующем запуске. Для сообщения об истечении времени запустите его снова из более быстрой или менее ограниченной сети.
* Если ваша сеть требует прокси, установите `HTTPS_PROXY` перед запуском установщика или `claude update`. См. [Проверка подключения к сети](/docs/ru/troubleshoot-install#check-network-connectivity).
* Если корпоративный прокси продолжает закрывать передачу, попросите вашу команду сети разрешить полную загрузку с `downloads.claude.ai`. См. [Требования к доступу в сеть](/docs/ru/network-config#network-access-requirements).
* Запустите `claude doctor` из вашей оболочки для диагностики установки

<h2 id="command-line-errors">
  Ошибки командной строки
</h2>

Эти ошибки поступают из командной строки `claude` и её подкоманд. Claude Code выводит их перед запуском вашего запроса или отправкой любого запроса API.

<h3 id="conflict-between-bg-and-print">
  Конфликт между --bg и --print
</h3>

Это сообщение требует Claude Code v2.1.198 или позже. Вы объединили `--bg` с `-p` или `--print` в одном вызове `claude`. `--bg` запускает [фоновый сеанс](/docs/ru/agent-view#from-your-shell), к которому вы позже подключаетесь с помощью `claude agents`, а `--print` запускает [неинтерактивно](/docs/ru/headless) и никогда не запускает интерактивный сеанс, к которому подключается `claude agents`. До версии v2.1.198 эта комбинация молча создавала фоновое задание, которое никогда не могло быть подключено.

```text theme={null}
--bg и --print конфликтуют: --print никогда не запускает интерактивный сеанс, к которому подключается `claude agents`, поэтому задание было бы недоступным. Запрос — это позиционный аргумент — удалите --print: `claude --bg '<task>'`.
```

**Что делать:**

* Удалите `-p` или `--print`. `--bg` принимает запрос в качестве позиционного аргумента, поэтому `claude --bg "<task>"` — это полная команда. См. [Dispatch new agents from your shell](/docs/ru/agent-view#from-your-shell).
* Чтобы запустить запрос неинтерактивно и вывести результат вместо создания фонового сеанса, удалите `--bg` и запустите `claude -p "<task>"`

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Значение --json-schema не является допустимой JSON Schema
</h3>

Схема, которую вы передали в [`--json-schema`](/docs/ru/cli-reference#cli-flags) в [неинтерактивном режиме](/docs/ru/headless#get-structured-output), не прошла компиляцию JSON Schema, поэтому `claude` завершает работу с кодом 1 вместо запуска запроса. До версии v2.1.205 недопустимая схема выдавала неструктурированный вывод без ошибки, и любая схема, использующая ключевое слово `format`, считалась недопустимой.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Текст после второго двоеточия — это диагностика валидатора и указывает на ключевое слово или местоположение, которое не прошло проверку. Схемы, использующие ключевое слово `format`, такие как `"format": "email"`, являются допустимыми: Claude Code принимает `format` как аннотацию и не применяет её.

Claude Code выполняет две проверки перед компиляцией схемы: он отклоняет значение, которое не является парсируемым JSON, с помощью `Error: --json-schema is not valid JSON`, и допустимый JSON, который не является объектом, с помощью `Error: --json-schema must be a JSON object`.

**Что делать:**

* Исправьте часть схемы, которую указывает диагностика, затем повторно запустите команду
* Если диагностика — `schema too large`, уменьшите вложенность схемы и повторное использование `$ref`
* См. [Get structured output](/docs/ru/headless#get-structured-output) для рабочей схемы и команды

<h3 id="could-not-import-a-server-from-claude-desktop">
  Не удалось импортировать сервер из Claude Desktop
</h3>

Claude Code не смог добавить один из серверов, которые вы выбрали в `claude mcp add-from-claude-desktop`. Команда по-прежнему импортирует другие выбранные серверы и выводит одну строку для каждого сервера, который она не смогла добавить. До версии v2.1.205 первый сервер, который не прошёл проверку, останавливал импорт и ни один из выбранных серверов не был добавлен.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Текст после имени сервера — это причина. Наиболее распространённая — это проверка имени: Claude Desktop допускает символы в именах серверов, такие как пробелы и точки, которые `claude mcp` ограничивает буквами, цифрами, дефисами и подчёркиваниями. Другие причины включают конфигурацию сервера, которая не прошла валидацию, и сервер, заблокированный [политикой MCP](/docs/ru/managed-mcp) вашей организации.

**Что делать:**

* Переименуйте сервер в `claude_desktop_config.json`, используя только буквы, цифры, дефисы и подчёркивания, затем снова запустите `claude mcp add-from-claude-desktop`
* Добавьте этот сервер напрямую с помощью `claude mcp add` или `claude mcp add-json` под допустимым именем. См. [Import MCP servers from Claude Desktop](/docs/ru/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="mcp-permission-prompt-tool-not-found">
  Инструмент запроса разрешения MCP не найден
</h3>

Инструмент, который вы передали в [`--permission-prompt-tool`](/docs/ru/cli-reference#cli-flags), не был среди подключённых инструментов MCP, когда запуск впервые нуждался в решении о разрешении, либо потому, что его сервер никогда не подключался, либо потому, что ни один подключённый сервер не предоставляет инструмент с таким именем. Claude Code по-прежнему отправляет ваш запрос: [неинтерактивный](/docs/ru/headless) запуск завершается с этой ошибкой и кодом выхода 1 при первом вызове инструмента, который требует одобрения, поэтому он не выдаёт ответ, хотя запрос был сделан. Перед первым запросом Claude Code ждёт до 30 секунд, установленного [`MCP_TIMEOUT`](/docs/ru/env-vars) для подключения этого сервера. До версии v2.1.206 запуск не ждал завершения подключения сервера, поэтому медленно запускающийся, но здоровый сервер также выдавал эту ошибку.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

Список после `Available MCP tools:` указывает инструменты MCP, которые были подключены, когда ожидание закончилось.

**Что делать:**

* Проверьте, что сервер запускается и остаётся подключённым: запустите `claude mcp list` в том же каталоге и подтвердите, что сервер указан как подключённый
* Подтвердите, что имя инструмента совпадает с именем `mcp__<server>__<tool>`, которое предоставляет сервер
* Если серверу требуется более 30 секунд для запуска, увеличьте [`MCP_TIMEOUT`](/docs/ru/env-vars)

<h2 id="plugin-errors">
  Ошибки плагинов
</h2>

Эти ошибки возникают из конфигурации [плагина](/docs/ru/plugins) и [маркетплейса](/docs/ru/plugin-marketplaces). Для проблем с плагинами, которые не выдают одно из сообщений на этой странице, например маркетплейс, который не загружается, или плагин, который устанавливается, но не отображается, см. [Устранение неполадок плагинов](/docs/ru/discover-plugins#troubleshooting).

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

Маркетплейс зарегистрирован под именем, которое [зарезервировано для официальных маркетплейсов Anthropic](/docs/ru/plugin-marketplaces#marketplace-schema), но его зарегистрированный источник не является репозиторием GitHub `anthropics`. Claude Code повторно проверяет зарезервированные имена каждый раз при загрузке или обновлении маркетплейса, поэтому маркетплейс и плагины, установленные из него, перестают загружаться. До версии 2.1.205 имя проверялось только при добавлении маркетплейса, поэтому запись, зарегистрированная до того, как её имя было зарезервировано, продолжала загружаться.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**Что делать:**

* Выполните `claude plugin marketplace remove <name>`, затем снова добавьте маркетплейс из официального репозитория `github.com/anthropics`
* Если вы публикуете маркетплейс третьей стороны, который использовал это имя до того, как оно было зарезервировано, переименуйте его и попросите пользователей добавить его из вашего источника
* См. список зарезервированных имён в разделе [Marketplace schema](/docs/ru/plugin-marketplaces#marketplace-schema)

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Хук плагина, [monitor](/docs/ru/plugins-reference#monitors), или MCP команда [`headersHelper`](/docs/ru/mcp#use-dynamic-headers-for-custom-authentication) ссылается на [опцию плагина](/docs/ru/plugins-reference#user-configuration) `${user_config.KEY}`, и подставленная строка будет передана в оболочку. Настроенное значение, содержащее `$(...)`, обратные кавычки или `;`, будет выполнено как код там, поэтому Claude Code отказывается запускать компонент вместо подстановки значения. Проверка выполняется на шаблоне команды, поэтому ошибка появляется даже когда значение ещё не настроено. До версии 2.1.207 значение подставлялось в команду оболочки.

Формулировка зависит от того, какая поверхность ссылалась на опцию. Хук в форме оболочки сообщает:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Monitor сообщает:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

MCP `headersHelper` сообщает:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Что делать:**

* Для хука добавьте массив `args`, чтобы он выполнялся в [exec форме](/docs/ru/hooks#exec-form-and-shell-form), где каждый `${user_config.KEY}` становится одним аргументом без оболочки между ними. Или удалите ссылку и прочитайте переменную окружения `$CLAUDE_PLUGIN_OPTION_<KEY>` внутри скрипта
* Для monitor удалите ссылку и пусть скрипт monitor прочитает значение из файла конфигурации
* Для `headersHelper` переместите `${user_config.KEY}` в поле `headers` сервера, которое не анализируется оболочкой, или прочитайте значение внутри скрипта помощника

<h2 id="tool-errors">
  Ошибки инструментов
</h2>

Эти ошибки возникают, когда встроенные инструменты Claude отказывают во входных данных. Claude самостоятельно исправляет большинство ошибок инструментов; два приведённых ниже требуют изменений с вашей стороны, так как они происходят из определения подагента или правила разрешений, которыми вы управляете.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

Ничего в [списке `tools` подагента](/docs/ru/sub-agents#supported-frontmatter-fields) не разрешилось в инструмент, поэтому Claude Code отказывается запускать подагента, а не запускать его без возможности действовать. Сообщение группирует записи по причине, по которой они не разрешились: неизвестный инструмент, инструмент, который недоступен подагентам, или распознанный, но не соответствующий ни одному инструменту в текущей сессии. Пропуск поля `tools` никогда не вызывает этот отказ. Шаблон сервера MCP, такой как `mcp__github__*`, не исключён: когда ни один подключённый инструмент не поступает с этого сервера, запуск отказывается с шаблоном в группе, которая ничего не совпала. До версии 2.1.208 подагент запускался без инструментов и возвращал пустой или запутанный результат.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Что делать:**

* Исправьте каждую запись, которую ошибка называет, в соответствии с [инструментами, доступными подагентам](/docs/ru/sub-agents#available-tools)
* Удалите записи для инструментов, которые сессия не имеет, такие как инструменты MCP с сервера, который не подключён
* Чтобы дать подагенту каждый инструмент, который есть у родителя, удалите поле `tools` вместо перечисления инструментов

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

Инструмент Edit был вызван на пути, соответствующем [правилу отказа `Read`](/docs/ru/permissions#read-and-edit), включая создание нового файла по этому пути. Редактирование переписывает содержимое, которое Claude должен иметь возможность прочитать обратно, поэтому вызов отказывается до любого доступа к файлу. Правило блокирует только инструмент Edit: Write и NotebookEdit не охватываются правилами отказа `Read`. До версии 2.1.208 только правило отказа `Edit` блокировало редактирование, и правило отказа `Read` само по себе не блокировало.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

**Что делать:**

* Если Claude должен иметь возможность редактировать файл, удалите или сузьте правило отказа `Read` в `/permissions` или в [параметрах](/docs/ru/settings#permission-settings)
* Если файл должен остаться нетронутым, сохраните правило и добавьте правило отказа `Edit` для того же пути, чтобы инструменты Write и NotebookEdit также были заблокированы

<h2 id="background-session-errors">
  Ошибки фоновой сессии
</h2>

[Фоновые сессии](/docs/ru/agent-view) работают без собственного интерактивного терминала, поэтому команды, которым он требуется, ведут себя там иначе. Эти сообщения появляются в стенограмме фоновой сессии в представлении агента или после присоединения.

<h3 id="commands-refused-in-a-background-session">
  Команды, отклоненные в фоновой сессии
</h3>

Команды, которые открывают интерактивное диалоговое окно, отклоняются в фоновой сессии с сообщением, в котором указывается форма, которая там работает, или вам предлагается запустить команду из обычного терминала. `/install-github-app`, список параметров `/mcp` и действия аутентификации в меню сервера MCP все отклоняются таким образом. До версии 2.1.208 они открывали свое диалоговое окно внутри фоновой сессии.
В версии 2.1.208 только средство выбора `/model` также было отклонено в фоновой сессии, а `/upgrade` вывел URL обновления вместо открытия браузера.

Формулировка указывает команду, которая была отклонена. Список параметров `/mcp` сообщает:

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**Что делать:**

* Используйте форму, указанную в сообщении, например `/mcp reconnect <server>`, `/mcp enable` или `/mcp disable`
* Для потоков входа и авторизации запустите команду из обычной сессии `claude` в терминале

<h3 id="claude_code_process_wrapper-launcher-errors">
  Ошибки средства запуска CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/ru/corporate-launcher) установлен, и его значение невозможно использовать, поэтому Claude Code отказывается запускать затронутый процесс вместо того, чтобы запустить его без средства запуска. Проблемы конфигурации сообщаются сообщением, которое начинается с имени переменной и указывает причину, например:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Средство запуска, которое запускается, но завершается без замены себя на Claude Code, приводит к сбою сессии, которую оно запускало, и строка сессии в представлении агента сообщает, что средство запуска `must exec, not daemonize`, за которым следует все, что вывело средство запуска. Сессия, которая не может запуститься или достичь фоновой службы из-за средства запуска, сообщает о проблеме средства запуска как о причине внутри `Couldn't reach the background service (...)`.

**Что делать:**

* Установите переменную на абсолютный путь исполняемого файла, который заканчивается вызовом `exec "$@"`. Полный контракт см. в разделе [контракт средства запуска](/docs/ru/corporate-launcher#the-launcher-contract)
* Проверьте `/status`, который показывает разрешенную команду запуска в записи Self-exec и предупреждает, когда работающая фоновая служба не совпадает с ней, или запустите `claude daemon status` из оболочки
* После исправления значения в блоке `env` [параметров](/docs/ru/corporate-launcher#set-up-the-launcher) перезагрузите фоновую службу с помощью `claude daemon stop --any`, чтобы следующая отправка запустила завернутую

<h2 id="configuration-warnings">
  Предупреждения о конфигурации
</h2>

Claude Code выводит эти сообщения в stderr при запуске, а не показывает ошибку в диалоговом окне. Они сообщают о конфигурации, которую он прочитал, но не применил.

<h3 id="workspace-has-not-been-trusted">
  Рабочее пространство не было доверено
</h3>

Claude Code обнаружил правила `permissions.allow` или записи `permissions.additionalDirectories` в файле `.claude/settings.json` или `.claude/settings.local.json` проекта и не применил их, потому что [правила разрешения из параметров проекта требуют доверия рабочему пространству](/docs/ru/permissions#project-allow-rules-and-workspace-trust). Количество, имя параметра и имя файла в сообщении варьируются в зависимости от вашей конфигурации. На правила `deny` и `ask` это не влияет.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Что делать:**

* Запустите `claude` в каталоге и примите диалоговое окно доверия. Диалоговое окно появляется даже если родительский каталог уже доверен, отображает удерживаемые правила и позволяет вам отклонить их и продолжить работу без них. До версии 2.1.200 диалоговое окно не появлялось в этой ситуации, поэтому этот шаг не мог быть завершен там.
* В [неинтерактивном режиме](/docs/ru/headless) с флагом `-p` диалоговое окно не отображается. Установите запись `hasTrustDialogAccepted` в `~/.claude.json`, используя точный ключ `projects`, который выводит сообщение.
* Если сообщение указывает на `.claude/settings.local.json` и вы запустили Claude Code вне репозитория git или в вашем домашнем каталоге, обновитесь до версии 2.1.200 или более поздней. Версии 2.1.196 по 2.1.199 рассматривали ваш собственный `.claude/settings.local.json` как предоставленный репозиторием в этих рабочих пространствах. На версии 2.1.207 и более поздних обновление недостаточно вне репозитория git, если вы не доверили папке: определение того, что папка находится вне репозитория, запускает git, и Claude Code запускает эту проверку только после того, как вы примете диалоговое окно доверия, поэтому используйте первый шаг. Ваш домашний каталог и любой другой [домашний каталог конфигурации](/docs/ru/permissions#project-allow-rules-and-workspace-trust) исключены и не ждут диалогового окна. См. [Правила разрешения проекта и доверие рабочему пространству](/docs/ru/permissions#project-allow-rules-and-workspace-trust).

<h2 id="responses-seem-lower-quality-than-usual">
  Ответы кажутся менее качественными, чем обычно
</h2>

Если ответы Claude кажутся менее способными, чем вы ожидаете, но ошибка не отображается, причина обычно заключается в состоянии разговора, а не в самой модели. Claude Code не молча меняет версии модели. Он может переключиться на резервную модель в трёх конкретных случаях:

* Настроенный [`--fallback-model`](/docs/ru/cli-reference#cli-flags) берёт на себя управление после ошибки доступности только для этого хода с уведомлением в расшифровке
* Проверка запуска Amazon Bedrock или Google Cloud's Agent Platform обнаруживает, что ваша модель по умолчанию недоступна
* [Автоматический fallback модели](/docs/ru/model-config#automatic-model-fallback) на Fable 5 переводит сеанс на модель Opus по умолчанию и показывает уведомление в расшифровке

Проверка выбора модели ниже ловит второй и третий случаи; первый появляется как уведомление в расшифровке, а не как изменение `/model`. [Конфигурация модели](/docs/ru/model-config) объясняет, когда применяется каждый fallback.

Сначала проверьте следующее:

* **Выбор модели**: запустите `/model`, чтобы подтвердить, что вы используете ожидаемую модель. Предыдущий выбор `/model` или переменная окружения `ANTHROPIC_MODEL` могут привести вас к меньшей модели, чем вы предполагали.
* **Уровень усилий**: запустите `/effort`, чтобы проверить текущий уровень рассуждений и повысить его для сложной отладки или работы над дизайном. Значения по умолчанию варьируются в зависимости от модели, поэтому проверьте перед тем, как предполагать, что вы ниже максимума. См. [Adjust effort level](/docs/ru/model-config#adjust-effort-level) для значений по умолчанию для каждой модели и сокращение `ultrathink`.
* **Давление контекста**: запустите `/context`, чтобы увидеть, насколько заполнено окно. Если оно близко к ёмкости, запустите `/compact` в естественной точке разрыва или `/clear`, чтобы начать заново. См. [Explore the context window](/docs/ru/context-window) для того, как auto-compact влияет на предыдущие ходы.
* **Устаревшие инструкции**: большие или устаревшие файлы `CLAUDE.md` и определения инструментов MCP потребляют контекст и могут направлять ответы. Проверка `/doctor` отмечает файлы памяти большого размера и неиспользуемые расширения, а `/context` показывает использование токенов инструментов MCP. До версии 2.1.205 `/doctor` открывал экран диагностики, который отмечал файлы памяти большого размера и определения подагентов.

Когда ответ идёт неправильно, откат обычно работает лучше, чем ответ с исправлениями. Нажмите Esc дважды или запустите `/rewind`, чтобы вернуться к моменту перед неправильным ходом, затем переформулируйте подсказку с большей конкретикой. Исправление в потоке сохраняет неправильную попытку в контексте, что может привязать более поздние ответы к ней. См. [Checkpointing](/docs/ru/checkpointing).

Если качество всё ещё кажется неправильным после проверки вышеуказанного, запустите `/feedback` и опишите, что вы ожидали в сравнении с тем, что вы получили. Обратная связь, отправленная таким образом, включает расшифровку разговора, что является самым быстрым способом для Anthropic диагностировать реальную регрессию. См. [Report an error](#report-an-error), если `/feedback` недоступен в вашей среде.

Если Claude предупреждает о подозреваемой инъекции подсказки или отказывает в запросе из-за подозреваемой инъекции, и текст, который называет предупреждение, — это контекст, который Claude Code добавляет в разговор автоматически, а не содержимое файла или веб-сайта, запустите `claude update` и повторите попытку. Если предупреждение повторяется после обновления, [сообщите об этом](#report-an-error) вместо того, чтобы вставлять отмеченное содержимое обратно в подсказку. До версии 2.1.201 Sonnet 5 отказывал в некоторых запросах таким же образом.

<h2 id="report-an-error">
  Сообщить об ошибке
</h2>

Для ошибок компонентов, которые не рассматриваются на этой странице, см. соответствующее руководство:

* Серверу MCP не удалось подключиться или пройти аутентификацию: [MCP](/docs/ru/mcp)
* Скрипт hook не выполнился или заблокировал инструмент: [Debug hooks](/docs/ru/hooks#debug-hooks)
* Отказано в доступе или ошибки файловой системы при установке: [Troubleshoot installation and login](/docs/ru/troubleshoot-install)

Если ошибка не указана здесь или предложенное исправление не помогает:

* Запустите `/feedback` внутри Claude Code, чтобы отправить стенограмму и описание в Anthropic. Команда также предлагает открыть предварительно заполненную проблему GitHub. Отправка в Anthropic требует [аутентификации](/docs/ru/authentication). На Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry и других сторонних поставщиков, или когда учетные данные Anthropic не настроены, `/feedback` сохраняет локальный архив, который вы можете отправить представителю вашей учетной записи Anthropic.
* Запустите `claude doctor` из вашей оболочки для диагностики только для чтения вашей установки или запустите проверку `/doctor` внутри Claude Code, чтобы найти и исправить проблемы настройки
* Проверьте [status.claude.com](https://status.claude.com) на наличие активных инцидентов
* Поищите [существующие проблемы](https://github.com/anthropics/claude-code/issues) на GitHub
