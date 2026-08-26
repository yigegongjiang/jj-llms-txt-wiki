> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Правовые и нормативные требования

> Правовые соглашения, сертификаты соответствия и информация о безопасности для Claude Code.

<h2 id="legal-agreements">
  Правовые соглашения
</h2>

<h3 id="license">
  Лицензия
</h3>

Ваше использование Claude Code подчиняется:

* [Коммерческие условия](https://www.anthropic.com/legal/commercial-terms) - для пользователей Team, Enterprise и Claude API
* [Условия обслуживания для потребителей](https://www.anthropic.com/legal/consumer-terms) - для пользователей Free, Pro и Max

<h3 id="commercial-agreements">
  Коммерческие соглашения
</h3>

Независимо от того, используете ли вы Claude API напрямую (1P) или получаете доступ через Amazon Bedrock или Google Cloud's Agent Platform (3P), ваше существующее коммерческое соглашение будет применяться к использованию Claude Code, если мы не договорились об ином.

<h2 id="compliance">
  Соответствие нормативным требованиям
</h2>

<h3 id="healthcare-compliance-baa">
  Соответствие требованиям здравоохранения (BAA)
</h3>

Если у клиента есть соглашение о деловом партнере (BAA) с нами и он хочет использовать Claude Code, BAA автоматически распространится на Claude Code, если клиент заключил BAA и активировал [Zero Data Retention (ZDR)](/docs/ru/zero-data-retention). BAA будет применяться к трафику API этого клиента, проходящему через Claude Code. ZDR включается на основе организации, поэтому каждая организация должна иметь отдельно включенный ZDR, чтобы быть охваченной BAA.

<h2 id="usage-policy">
  Политика использования
</h2>

<h3 id="acceptable-use">
  Допустимое использование
</h3>

Использование Claude Code подчиняется [Политике использования Anthropic](https://www.anthropic.com/legal/aup). Объявленные ограничения использования для планов Pro и Max предполагают обычное индивидуальное использование Claude Code и Agent SDK.

<h3 id="authentication-and-credential-use">
  Аутентификация и использование учетных данных
</h3>

Claude Code аутентифицируется на серверах Anthropic с использованием токенов OAuth или ключей API. Эти методы аутентификации служат разным целям:

* **Аутентификация OAuth** предназначена исключительно для покупателей планов Claude Free, Pro, Max, Team и Enterprise и разработана для поддержки обычного использования Claude Code и других встроенных приложений Anthropic. Инструкции по входу см. в разделе [Вход в вашу учетную запись Claude](https://support.claude.com/en/articles/13189465-logging-in-to-your-claude-account); информацию о том, как Claude Code выполняет аутентификацию OAuth, см. в разделе [Authentication](/docs/ru/authentication).
* **Разработчики**, создающие продукты или сервисы, которые взаимодействуют с возможностями Claude, включая те, которые используют [Agent SDK](/docs/ru/agent-sdk/overview), должны использовать аутентификацию по ключу API через [Claude Console](https://platform.claude.com/) или поддерживаемого облачного провайдера. Anthropic не разрешает сторонним разработчикам предлагать вход Claude.ai или маршрутизировать запросы через учетные данные планов Free, Pro или Max от имени своих пользователей.

Anthropic оставляет за собой право принимать меры для обеспечения соблюдения этих ограничений и может делать это без предварительного уведомления.

По вопросам о разрешенных методах аутентификации для вашего случая использования, пожалуйста, [свяжитесь с отделом продаж](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=legal_compliance_contact_sales).

<h2 id="security-and-trust">
  Безопасность и доверие
</h2>

<h3 id="trust-and-safety">
  Доверие и безопасность
</h3>

Вы можете найти дополнительную информацию в [Центре доверия Anthropic](https://trust.anthropic.com) и [Центре прозрачности](https://www.anthropic.com/transparency).

<h3 id="security-vulnerability-reporting">
  Отчетность об уязвимостях безопасности
</h3>

Anthropic управляет нашей программой безопасности через HackerOne. [Используйте эту форму для отчета об уязвимостях](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new).

***

© Anthropic PBC. Все права защищены. Использование подчиняется применимым Условиям обслуживания Anthropic.
