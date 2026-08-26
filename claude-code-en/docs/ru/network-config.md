> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Конфигурация сети для предприятия

> Настройте Claude Code для корпоративных сред с прокси-серверами, пользовательскими центрами сертификации (CA) и взаимной аутентификацией Transport Layer Security (mTLS).

Claude Code поддерживает различные конфигурации сети и безопасности предприятия через переменные окружения. Это включает маршрутизацию трафика через корпоративные прокси-серверы, доверие пользовательским центрам сертификации (CA) и аутентификацию с помощью сертификатов взаимного Transport Layer Security (mTLS) для повышенной безопасности.

<Note>
  Все переменные окружения, показанные на этой странице, также можно настроить в [`settings.json`](/docs/ru/settings).
</Note>

<h2 id="proxy-configuration">
  Конфигурация прокси
</h2>

<h3 id="environment-variables">
  Переменные окружения
</h3>

Claude Code соответствует стандартным переменным окружения прокси:

```bash theme={null}
# HTTPS прокси (рекомендуется)
export HTTPS_PROXY=https://proxy.example.com:8080

# HTTP прокси (если HTTPS недоступен)
export HTTP_PROXY=http://proxy.example.com:8080

# Обход прокси для конкретных запросов - формат с разделением пробелом
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Обход прокси для конкретных запросов - формат с разделением запятой
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Обход прокси для всех запросов
export NO_PROXY="*"
```

<Note>
  Claude Code не поддерживает SOCKS прокси.
</Note>

<h3 id="basic-authentication">
  Базовая аутентификация
</h3>

Если ваш прокси требует базовую аутентификацию, включите учетные данные в URL прокси:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Избегайте жесткого кодирования паролей в скриптах. Используйте переменные окружения или безопасное хранилище учетных данных вместо этого.
</Warning>

<Tip>
  Для прокси, требующих расширенную аутентификацию (NTLM, Kerberos и т. д.), рассмотрите использование сервиса LLM Gateway, который поддерживает ваш метод аутентификации.
</Tip>

<h2 id="ca-certificate-store">
  Хранилище сертификатов CA
</h2>

По умолчанию Claude Code доверяет как своему встроенному набору сертификатов Mozilla CA, так и хранилищу сертификатов вашей операционной системы. Чтение хранилища ОС требует среды выполнения с `tls.getCACertificates`: встроенный установщик всегда его имеет, а установки npm требуют Node 22.15 или более поздней версии. На более старых версиях Node применяются только встроенный набор и `NODE_EXTRA_CA_CERTS`. Корпоративные прокси с TLS-инспекцией, такие как CrowdStrike Falcon и Zscaler, работают без дополнительной конфигурации, когда их корневой сертификат установлен в хранилище доверия ОС и среда выполнения может его прочитать.

`CLAUDE_CODE_CERT_STORE` принимает список источников, разделенный запятыми. Признанные значения: `bundled` для набора Mozilla CA, поставляемого с Claude Code, и `system` для хранилища доверия операционной системы. По умолчанию используется `bundled,system`.

Для доверия только встроенному набору Mozilla CA:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Для доверия только хранилищу сертификатов ОС:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` не имеет выделенного ключа схемы `settings.json`. Установите его через блок `env` в `~/.claude/settings.json` или непосредственно в окружении процесса.
</Note>

<h2 id="custom-ca-certificates">
  Пользовательские сертификаты CA
</h2>

Если ваша корпоративная среда использует пользовательский CA, настройте Claude Code для доверия ему напрямую:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Аутентификация mTLS
</h2>

Для корпоративных сред, требующих аутентификацию с помощью сертификата клиента:

```bash theme={null}
# Сертификат клиента для аутентификации
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Приватный ключ клиента
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Опционально: Парольная фраза для зашифрованного приватного ключа
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code читает файлы сертификата и ключа при запуске и перечитывает их каждый раз при применении параметров, включая изменение параметров во время сеанса. Для ротации сертификата и ключа замените файлы по тем же путям.

<h2 id="network-access-requirements">
  Требования к доступу в сети
</h2>

Claude Code требует доступ к следующим URL. Добавьте их в белый список в конфигурации прокси и правилах брандмауэра, особенно в контейнеризованных или ограниченных сетевых средах.

| URL                            | Требуется для                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`            | Запросы Claude API                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `claude.ai`                    | Аутентификация учетной записи claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `platform.claude.com`          | Аутентификация учетной записи Anthropic Console                                                                                                                                                                                                                                                                                                                                                                                                          |
| `mcp-proxy.anthropic.com`      | [MCP connectors из claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai), включая connectors, которые настраивает администратор организации. Трафик Connector маршрутизируется через этот прокси; connectors включены по умолчанию для пользователей, прошедших аутентификацию через claude.ai. Чтобы отключить, установите [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/ru/env-vars) или параметр [`disableClaudeAiConnectors`](/docs/ru/settings#available-settings) |
| `downloads.claude.ai`          | Загрузки исполняемых файлов плагинов; встроенный установщик и встроенное автоматическое обновление                                                                                                                                                                                                                                                                                                                                                       |
| `storage.googleapis.com`       | Счетчики установок и метаданные плагинов, отображаемые в `/plugin`. Подписанные [artifact](/docs/ru/artifacts) загрузки сначала пытаются использовать этот хост; публикация переходит на `api.anthropic.com`, когда он заблокирован                                                                                                                                                                                                                           |
| `storage.googleapis.com`       | Встроенный установщик и встроенное автоматическое обновление в версиях до 2.1.116                                                                                                                                                                                                                                                                                                                                                                        |
| `bridge.claudeusercontent.com` | Мост WebSocket расширения [Claude в Chrome](/docs/ru/chrome)                                                                                                                                                                                                                                                                                                                                                                                                  |
| `*.claudeusercontent.com`      | Просмотр [артефактов](/docs/ru/artifacts) на claude.ai. Средство просмотра загружает содержимое каждого артефакта из изолированного поддомена этого источника. Требуется в браузере средства просмотра, а не в самом CLI                                                                                                                                                                                                                                      |
| `raw.githubusercontent.com`    | Лента журнала изменений для [`/release-notes`](/docs/ru/commands) и примечания к выпуску, отображаемые после обновления                                                                                                                                                                                                                                                                                                                                       |

Если вы устанавливаете Claude Code через npm или управляете собственным распределением бинарных файлов, конечным пользователям не требуется встроенный установщик и встроенное автоматическое обновление из `downloads.claude.ai`. Остальные использования в таблице применяются независимо от метода установки.

Claude Code также отправляет дополнительную операционную телеметрию по умолчанию, которую вы можете отключить с помощью переменных окружения. См. [Услуги телеметрии](/docs/ru/data-usage#telemetry-services), чтобы узнать, как отключить её перед окончательным формированием вашего белого списка.

При использовании [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai), [Microsoft Foundry](/docs/ru/microsoft-foundry) или сеанса [шлюза приложений Claude](/docs/ru/claude-apps-gateway) с входом, трафик модели и аутентификация идут к вашему поставщику или шлюзу вместо `api.anthropic.com`, `claude.ai` или `platform.claude.com`. Инструмент WebFetch по-прежнему вызывает `api.anthropic.com` для своей [проверки безопасности домена](/docs/ru/data-usage#webfetch-domain-safety-check), если вы не установите `skipWebFetchPreflight: true` в [параметрах](/docs/ru/settings).

[Claude Code в веб-версии](/docs/ru/claude-code-on-the-web) и [Code Review](/docs/ru/code-review) подключаются к вашим репозиториям из управляемой Anthropic инфраструктуры. Если ваша организация GitHub Enterprise Cloud ограничивает доступ по IP-адресу, включите [наследование списка разрешенных IP для установленных GitHub Apps](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps). GitHub App Claude регистрирует свои диапазоны IP, поэтому включение этого параметра позволяет получить доступ без ручной конфигурации. Чтобы [добавить диапазоны в список разрешенных вручную](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) вместо этого, или для настройки других брандмауэров, см. [IP-адреса Anthropic API](https://platform.claude.com/docs/en/api/ip-addresses).

Для самостоятельно размещаемых экземпляров [GitHub Enterprise Server](/docs/ru/github-enterprise-server) за брандмауэром добавьте в белый список те же [IP-адреса Anthropic API](https://platform.claude.com/docs/en/api/ip-addresses), чтобы инфраструктура Anthropic могла достичь вашего хоста GHES для клонирования репозиториев и публикации комментариев к рецензиям.

<h3 id="desktop-and-claude-ai">
  Desktop и claude.ai
</h3>

Предыдущая таблица в основном охватывает автономный CLI. Приложение Claude Desktop и claude.ai в браузере загружают код приложения с дополнительных хостов CDN Anthropic, включая `assets-proxy.anthropic.com`. Разрешение `claude.ai` при блокировке этих хостов приводит к пустой странице, а не к ошибке. См. [требования к доступу в сети](/docs/ru/desktop#network-access-requirements) на странице Desktop.

<h2 id="additional-resources">
  Дополнительные ресурсы
</h2>

* [Параметры Claude Code](/docs/ru/settings)
* [Справочник переменных окружения](/docs/ru/env-vars)
* [Руководство по устранению неполадок](/docs/ru/troubleshooting)
