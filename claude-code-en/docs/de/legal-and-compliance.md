> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rechtliche Bestimmungen und Compliance

> Rechtliche Vereinbarungen, Compliance-Zertifizierungen und Sicherheitsinformationen für Claude Code.

<h2 id="legal-agreements">
  Rechtliche Vereinbarungen
</h2>

<h3 id="license">
  Lizenz
</h3>

Ihre Nutzung von Claude Code unterliegt:

* [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms) - für Team-, Enterprise- und Claude API-Nutzer
* [Consumer Terms of Service](https://www.anthropic.com/legal/consumer-terms) - für Free-, Pro- und Max-Nutzer

<h3 id="commercial-agreements">
  Kommerzielle Vereinbarungen
</h3>

Unabhängig davon, ob Sie die Claude API direkt (1P) nutzen oder über Amazon Bedrock oder Google Cloud's Agent Platform (3P) darauf zugreifen, gilt Ihre bestehende kommerzielle Vereinbarung für die Nutzung von Claude Code, sofern wir nicht etwas anderes gegenseitig vereinbart haben.

<h3 id="can-customers-offer-claude-code-in-their-products">
  Können Kunden Claude Code in ihren Produkten anbieten?
</h3>

Sofern wir nicht etwas anderes gegenseitig vereinbart haben, erfordert die Vorinstallation oder das Ausführen von Claude Code in Ihren Produkten oder Diensten (z. B. in gehosteten Sandboxes oder anderer Agent-Infrastruktur) die Zustimmung zu unseren [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms) und die Einhaltung der folgenden Bedingungen:

* **Die Claude Code-Binärdatei darf nicht geändert werden.** Claude Code muss so installiert und ausgeführt werden, wie es von Anthropic veröffentlicht wird, und Kunden dürfen keine in Claude Code integrierten Authentifizierungsmethoden entfernen, deaktivieren oder einschränken (einschließlich Methoden, die die Anmeldung mit einem Claude-Konto oder dem eigenen API-Schlüssel des Benutzers ermöglichen).
* **Kunden dürfen nicht für Claude-Nutzung bezahlen, diese weiterverkaufen oder als Vermittler für ihre Endbenutzer fungieren.** Jeder Endbenutzer muss sich mit seinem eigenen Anthropic API-Schlüssel, seinen Claude-Abonnementplan-Anmeldedaten oder seinen Anmeldedaten des 3P-Inferenzanbieters (Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry) authentifizieren. Diese Nutzung wird direkt dem Endbenutzer unter seiner eigenen Vereinbarung mit Anthropic oder, für Inferenzanbieter von Drittanbietern, mit dem entsprechenden Anbieter in Rechnung gestellt.

**Verwendung des Claude Code-Namens und -Logos.** Sie können korrekt in Klartext angeben, dass Ihr Produkt Claude Code vorinstalliert hat oder dass es Claude Code ausführt. Sie können jedoch die Namen oder Logos von Claude Code oder Anthropic nicht als Teil Ihres eigenen Produkts, einer Funktion oder eines Unternehmensnamens, in Ihrem eigenen Logo oder auf eine Weise verwenden, die darauf hindeutet, dass Anthropic Ihr Produkt gebaut hat, befürwortet oder damit verbunden ist. Jede andere Verwendung von Namen oder Logos von Anthropic wird durch unsere [Trademark Guidelines](https://www.anthropic.com/legal/trademark-guidelines) geregelt und erfordert unsere schriftliche Genehmigung.

Claude Code unterliegt weiterhin den Standard-Bedingungen von Anthropic (siehe die Abschnitte Lizenz und Kommerzielle Vereinbarungen oben), unabhängig von der Plattform, über die darauf zugegriffen wird.

<h2 id="compliance">
  Compliance
</h2>

<h3 id="healthcare-compliance-baa">
  Healthcare-Compliance (BAA)
</h3>

Wenn ein Kunde eine Business Associate Agreement (BAA) mit Anthropic abgeschlossen hat und [Zero Data Retention (ZDR)](/docs/de/zero-data-retention) für die relevante Organisation aktiviert hat, erstreckt sich diese BAA auf den API-Datenverkehr des Kunden durch Claude Code.

<h2 id="usage-policy">
  Nutzungsrichtlinie
</h2>

<h3 id="acceptable-use">
  Akzeptable Nutzung
</h3>

Die Nutzung von Claude Code unterliegt der [Anthropic Usage Policy](https://www.anthropic.com/legal/aup). Die beworbenen Nutzungslimits für Pro- und Max-Pläne gehen von einer gewöhnlichen, individuellen Nutzung von Claude Code und dem Agent SDK aus.

<h3 id="authentication-and-credential-use">
  Authentifizierung und Anmeldedatenverwaltung
</h3>

Claude Code authentifiziert sich bei Anthropic-Servern mit OAuth-Token oder API-Schlüsseln. Diese Authentifizierungsmethoden dienen unterschiedlichen Zwecken:

* **OAuth-Authentifizierung** ist ausschließlich für Käufer von Claude Free-, Pro-, Max-, Team- und Enterprise-Abonnementplänen vorgesehen und soll die gewöhnliche Nutzung von Claude Code und anderen nativen Anthropic-Anwendungen unterstützen. Weitere Informationen zu den Anmeldeschritten finden Sie unter [Anmelden bei Ihrem Claude-Konto](https://support.claude.com/en/articles/13189465-logging-in-to-your-claude-account); Informationen dazu, wie Claude Code die OAuth-Authentifizierung durchführt, finden Sie unter [Authentifizierung](/docs/de/authentication).
* **Entwickler**, die Produkte oder Services entwickeln, die mit Claudes Funktionen interagieren, einschließlich derjenigen, die das [Agent SDK](/docs/de/agent-sdk/overview) nutzen, sollten API-Schlüssel-Authentifizierung über die [Claude Console](https://platform.claude.com/) oder einen unterstützten Cloud-Anbieter verwenden. Anthropic gestattet Drittentwicklern nicht, Claude.ai-Anmeldungen in ihren eigenen Anwendungen anzubieten oder Anfragen über Free-, Pro- oder Max-Plan-Anmeldedaten im Namen ihrer Nutzer weiterzuleiten. Darüber hinaus dürfen Entwickler Claude.ai-Anmeldedaten oder Sitzungs-Token nicht erfassen, speichern oder vermitteln – die Anmeldung bei einem Claude-Konto muss über Anthropics eigenen Ablauf erfolgen.

Dies schränkt nicht ein, wie Kunden ihre eigenen API-Schlüssel oder Anmeldedaten von Drittanbieter-Inferenz-Anbietern bereitstellen und verwalten – beispielsweise durch Konfigurieren eines API-Schlüssels in einer Entwicklungsumgebung, einem Secrets Manager oder einem Machine Image zur Nutzung durch die autorisierten Nutzer des Kunden – vorausgesetzt, die resultierende Nutzung wird dem Schlüsseleigentümer unter seiner Vereinbarung mit Anthropic (oder dem anwendbaren Anbieter) in Rechnung gestellt und wird nicht wie oben beschrieben weiterverkauft oder vermittelt. Es verhindert auch nicht, dass ein Endnutzer sich mit seinem eigenen Claude-Abonnement bei der unveränderten Claude Code-Binärdatei anmeldet, auch wenn eine Plattform Claude Code wie oben unter *Können Kunden Claude Code in ihren Produkten anbieten?* beschrieben anbietet.

Anthropic behält sich das Recht vor, Maßnahmen zur Durchsetzung dieser Einschränkungen zu ergreifen, und kann dies ohne vorherige Ankündigung tun.

Bei Fragen zu zulässigen Authentifizierungsmethoden für Ihren Anwendungsfall wenden Sie sich bitte an den [Vertrieb](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=legal_compliance_contact_sales).

<h2 id="security-and-trust">
  Sicherheit und Vertrauen
</h2>

<h3 id="trust-and-safety">
  Vertrauen und Sicherheit
</h3>

Weitere Informationen finden Sie im [Anthropic Trust Center](https://trust.anthropic.com) und im [Transparency Hub](https://www.anthropic.com/transparency).

<h3 id="security-vulnerability-reporting">
  Meldung von Sicherheitslücken
</h3>

Anthropic verwaltet unser Sicherheitsprogramm über HackerOne. [Verwenden Sie dieses Formular, um Sicherheitslücken zu melden](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new).

***

© Anthropic PBC. Alle Rechte vorbehalten. Die Nutzung unterliegt den geltenden Anthropic Terms of Service.
