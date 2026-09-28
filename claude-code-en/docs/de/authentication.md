> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Authentifizierung

> Melden Sie sich bei Claude Code an und konfigurieren Sie die Authentifizierung für Einzelpersonen, Teams und Organisationen.

Claude Code unterstützt mehrere Authentifizierungsmethoden je nach Ihrer Einrichtung. Einzelne Benutzer können sich mit einem claude.ai-Konto anmelden, während Teams Claude for Teams oder Enterprise, die Claude Console oder einen Cloud-Anbieter wie Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verwenden können.

<h2 id="log-in-to-claude-code">
  Melden Sie sich bei Claude Code an
</h2>

Nach dem [Installieren von Claude Code](/docs/de/setup#install-claude-code) führen Sie `claude` in Ihrem Terminal aus. Beim ersten Start öffnet Claude Code ein Browserfenster, in dem Sie sich anmelden können. Wenn Sie die Umgebungsvariable `ANTHROPIC_API_KEY` gesetzt haben, überspringt Claude Code die Anmeldeeingabeaufforderung und fordert Sie stattdessen auf, den Schlüssel zu genehmigen.

Wenn der Browser nicht automatisch geöffnet wird, drücken Sie `c`, um die Anmelde-URL in Ihre Zwischenablage zu kopieren, und fügen Sie sie dann in Ihren Browser ein.

Wenn Ihr Browser nach der Anmeldung einen Anmeldecode anzeigt, anstatt Sie zurückzuleiten, fügen Sie ihn an der Eingabeaufforderung `Paste code here if prompted` im Terminal ein. Dies geschieht, wenn der Browser den lokalen Callback-Server von Claude Code nicht erreichen kann, was in WSL2, SSH-Sitzungen und Containern häufig vorkommt.

Wenn die Anmeldung abgeschlossen ist, zeigt das Terminal `Login successful` an und fordert Sie auf, die `Eingabetaste` zu drücken, um fortzufahren.

Sie können sich mit einem dieser Kontotypen authentifizieren:

* **Claude Pro oder Max Abonnement**: Melden Sie sich mit Ihrem claude.ai-Konto an. Abonnieren Sie unter [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams oder Enterprise**: Melden Sie sich mit dem claude.ai-Konto an, zu dem Sie Ihr Team-Administrator eingeladen hat.
* **Claude Console**: Melden Sie sich mit Ihren Console-Anmeldedaten an. Ihr Administrator muss Sie zunächst [eingeladen haben](#claude-console-authentication). Sie können sich mit oder ohne [Erstellen eines API-Schlüssels](#sign-in-without-an-api-key) anmelden.
* **Cloud-Anbieter**: Wenn Ihre Organisation [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry) verwendet, legen Sie die erforderlichen Umgebungsvariablen fest, bevor Sie `claude` ausführen, oder wählen Sie **3rd-party platform** bei der Anmeldeeingabeaufforderung aus, die einen interaktiven Setup-Assistenten für Bedrock und Vertex AI startet. Es ist keine Browser-Anmeldung erforderlich.
* **Cloud-Gateway**: Wenn Ihre Organisation ein selbstgehostetes [Claude Apps Gateway](/docs/de/claude-apps-gateway) betreibt, melden Sie sich über `/login` mit Corporate SSO an. Das vom Gateway ausgegebene Token ist die einzige Anmeldeinformation der Sitzung.

Administratoren können festlegen, welche Anmeldungsmethode Entwickler verwenden, und verlangen, dass claude.ai-Anmeldungen zu einer bestimmten Organisation gehören; siehe [Anmeldung auf Ihre Organisation beschränken](#restrict-login-to-your-organization).

Um sich abzumelden und sich erneut zu authentifizieren, geben Sie `/logout` an der Claude Code-Eingabeaufforderung ein. Das Abmelden setzt auch Ihren Einrichtungsstatus beim ersten Start zurück, sodass Claude Code Sie beim nächsten Ausführen von `claude` erneut durch die Anmeldung und Einrichtung führt.

Wenn Sie Probleme beim Anmelden haben, siehe [Authentifizierungsfehlersuche](/docs/de/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Richten Sie die Team-Authentifizierung ein
</h2>

Für Teams und Organisationen können Sie den Claude Code-Zugriff auf eine der folgenden Arten konfigurieren:

* [Claude for Teams oder Enterprise](#claude-for-teams-or-enterprise), empfohlen für die meisten Teams
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/de/claude-apps-gateway), ein selbst gehostetes Gateway, das Entwickler mit Ihrem IdP anmeldet und Inferenzen an den von Ihnen konfigurierten Cloud-Anbieter weiterleitet
* [Amazon Bedrock](/docs/de/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/de/google-vertex-ai)
* [Microsoft Foundry](/docs/de/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams oder Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) und [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) bieten die beste Erfahrung für Organisationen, die Claude Code verwenden. Team-Mitglieder erhalten Zugriff auf Claude Code und Claude im Web mit zentralisierter Abrechnung und Team-Verwaltung.

* **Claude for Teams**: Self-Service-Plan mit Zusammenarbeitsfunktionen, Admin-Tools, SSO, Abrechnungsverwaltung und [serverseitig verwalteten Einstellungen](/docs/de/server-managed-settings) für organisationsweite Claude Code-Konfiguration. Am besten für kleinere Teams.
* **Claude for Enterprise**: Fügt Domain-Erfassung, rollenbasierte Berechtigungen und die Compliance-API hinzu. Am besten für größere Organisationen mit Sicherheits- und Compliance-Anforderungen.

<Steps>
  <Step title="Abonnieren">
    Abonnieren Sie [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) oder kontaktieren Sie den Vertrieb für [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Team-Mitglieder einladen">
    Laden Sie Team-Mitglieder vom Admin-Dashboard ein.
  </Step>

  <Step title="Installieren und anmelden">
    Team-Mitglieder installieren Claude Code und melden sich mit ihren claude.ai-Konten an.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Claude Console-Authentifizierung
</h3>

Für Organisationen, die API-basierte Abrechnung bevorzugen, können Sie den Zugriff über die Claude Console einrichten.

<Steps>
  <Step title="Erstellen oder verwenden Sie ein Console-Konto">
    Verwenden Sie Ihr vorhandenes Claude Console-Konto oder erstellen Sie ein neues.
  </Step>

  <Step title="Benutzer hinzufügen">
    Sie können Benutzer auf eine der beiden folgenden Arten hinzufügen:

    * Laden Sie Benutzer in Massen aus der Console ein: Einstellungen -> Mitglieder -> Einladen
    * [Richten Sie SSO ein](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="Rollen zuweisen">
    Weisen Sie beim Einladen von Benutzern eine der folgenden Rollen zu:

    * **Claude Code**-Rolle: Benutzer können nur Claude Code API-Schlüssel erstellen
    * **Developer**-Rolle: Benutzer können jede Art von API-Schlüssel erstellen
  </Step>

  <Step title="Benutzer schließen die Einrichtung ab">
    Jeder eingeladene Benutzer muss:

    * Die Console-Einladung akzeptieren
    * [Systemanforderungen überprüfen](/docs/de/setup#system-requirements)
    * [Claude Code installieren](/docs/de/setup#install-claude-code)
    * Sich mit Console-Kontoanmeldedaten anmelden
  </Step>
</Steps>

<h4 id="sign-in-without-an-api-key">
  Anmelden ohne API-Schlüssel
</h4>

Sie können sich bei Ihrem Console-Konto anmelden, ohne einen API-Schlüssel zu erstellen, auch wenn Ihre Organisation Entwicklern nicht erlaubt, diese zu erstellen. Wählen Sie das Anthropic Console-Konto bei der `/login`-Eingabeaufforderung aus, und Claude Code fragt Sie, wie Sie sich anmelden möchten. Erfordert Claude Code v2.1.242 oder später. Beide Routen melden Sie im Browser bei Console an und unterscheiden sich darin, was Claude Code danach speichert:

* **Melden Sie sich mit Ihrem Console-Konto an**, mit der Bezeichnung `(empfohlen)`: Claude Code behält das OAuth-Token aus dieser Anmeldung und speichert es als [Anthropic-Profil](#anthropic-profiles-and-federation-credentials). Es erstellt keinen API-Schlüssel
* **Erstellen Sie einen API-Schlüssel**, mit der Bezeichnung `(veraltet)`: Claude Code erstellt einen Console API-Schlüssel für Sie und speichert ihn mit Ihren anderen Anmeldedaten

In der Praxis speichert das Profil eine OAuth-Anmeldung, während ein API-Schlüssel eine statische Anmeldedaten ist: Claude Code aktualisiert die Anmeldung des Profils automatisch, und wenn die Aktualisierung fehlschlägt, schlagen Anfragen mit [Anthropic-Profil-Anmeldung abgelaufen](/docs/de/errors#anthropic-profile-login-expired) fehl, bis Sie sich erneut anmelden.

Sie erhalten nicht bei jedem Computer die Wahl. Claude Code erstellt einen API-Schlüssel ohne Nachfrage in diesen Fällen:

* Sie führen gegen einen Cloud-Anbieter aus, z. B. [Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry](/docs/de/third-party-integrations) oder [Claude Platform auf AWS](/docs/de/claude-platform-on-aws)
* Eine beliebige Einstellungsdatei setzt [`forceLoginOrgUUID`](#restrict-login-to-your-organization) oder setzt `forceLoginMethod` auf `"claudeai"` oder `"console"`
* Eine verwaltete Einstellungsquelle auf Ihrem Computer, z. B. die verwaltete Einstellungsdatei, ein MDM-Profil oder die zwischengespeicherten serverseitig verwalteten Einstellungen, existiert, aber Claude Code [kann sie nicht lesen](/docs/de/managed-settings#invalid-entries-in-managed-settings) und keine andere verwaltete Quelle liefert eine Richtlinie

Heben Sie `ANTHROPIC_API_KEY` auf, bevor Sie sich ohne Schlüssel anmelden. Ein Profil, das von Claude Code's eigener Console-Anmeldung oder von der Claude Platform CLI's `ant auth login` geschrieben wird, ist die gleiche Art von Anmeldedaten, daher ersetzt die erneute Anmeldung es.

Nach der Anmeldung ohne Schlüssel haben Sie ein Profil anstelle eines gespeicherten API-Schlüssels:

* **Welches Profil es schreibt**: Claude Code schreibt das Profil, das von `ANTHROPIC_PROFILE` benannt wird, oder Ihr aktives Profil, oder `default`. Wenn dieses Profil ein Verbundprofil ist, weigert sich Claude Code, die Anmeldung durchzuführen, anstatt es zu überschreiben
* **Wovon es Sie abmeldet**: Claude Code meldet Sie von jeder claude.ai-Anmeldung ab, die auf dem Computer gespeichert ist
* **Wie man es rückgängig macht**: Führen Sie `/logout` aus, das die Anmeldedaten entfernt und widerruft, die diese Anmeldung geschrieben hat

Wenn Ihre Organisation [serverseitig verwaltete Einstellungen](/docs/de/server-managed-settings) verwendet, gelten diese für diese Anmeldung auf Claude Code v2.1.257 oder später.

Alles andere über Profile gilt für diese Anmeldung, einschließlich wo sie gegen Ihre anderen Anmeldedaten rangiert, die `Profile`-Zeile, die Sie in `/status` erhalten, und die Funktionen, die eine claude.ai-Anmeldung benötigen. Siehe [Anthropic-Profile und Verbundanmeldedaten](#anthropic-profiles-and-federation-credentials).

<h3 id="cloud-provider-authentication">
  Cloud-Anbieter-Authentifizierung
</h3>

Für Teams, die Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verwenden:

<Steps>
  <Step title="Befolgen Sie die Anbieter-Einrichtung">
    Befolgen Sie die [Amazon Bedrock-Dokumentation](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform-Dokumentation](/docs/de/google-vertex-ai) oder [Microsoft Foundry-Dokumentation](/docs/de/microsoft-foundry).
  </Step>

  <Step title="Verteilen Sie die Konfiguration">
    Verteilen Sie die Umgebungsvariablen und Anweisungen zum Generieren von Cloud-Anmeldedaten an Ihre Benutzer. Lesen Sie mehr darüber, wie Sie [die Konfiguration hier verwalten](/docs/de/settings).
  </Step>

  <Step title="Installieren Sie Claude Code">
    Benutzer können [Claude Code installieren](/docs/de/setup#install-claude-code).
  </Step>
</Steps>

<h3 id="restrict-login-to-your-organization">
  Beschränken Sie die Anmeldung auf Ihre Organisation
</h3>

Um zu verlangen, dass die claude.ai-Anmeldungen von Entwicklern zu einer bestimmten Anthropic-Organisation gehören, setzen Sie [`forceLoginMethod`](/docs/de/settings-reference#forceloginmethod) und [`forceLoginOrgUUID`](/docs/de/settings-reference#forceloginorguuid) in [verwalteten Einstellungen](/docs/de/managed-settings). Setzen Sie `forceLoginOrgUUID` auf Ihre Organisations-ID, die in [claude.ai Admin-Einstellungen](https://claude.ai/admin-settings/organization) für Claude for Teams oder Enterprise-Organisationen angezeigt wird. Claude Code meldet einen Fehler für eine claude.ai-Anmeldung bei einer anderen Organisation und beendet sich beim Start, wenn die verwendete claude.ai-Anmeldedaten zu einer Organisation gehört, die nicht aufgelistet ist.

Für Claude Console-Anmeldungen verwendet Claude Code `forceLoginOrgUUID`, um die Organisation auf der Console-Anmeldeseite vorauszuwählen, wenn Sie sie auf eine einzelne Console-Organisations-ID setzen, die unter [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization) angezeigt wird. Es überprüft nicht, zu welcher Organisation die resultierende Console-Anmeldedaten gehört, weder bei der Anmeldung noch beim Start, und ein Entwickler, der sich mit einem Console-Konto angemeldet hat, bevor Sie die Schlüssel bereitgestellt haben, bleibt angemeldet.

Wenn Sie `forceLoginOrgUUID` in einer beliebigen Einstellungsdatei setzen, stoppt Claude Code das Angebot der [schlüssellosen Console-Anmeldung](#sign-in-without-an-api-key) in den Sitzungen, auf die diese Datei zutrifft, und erstellt stattdessen einen API-Schlüssel. Um Entwickler zur claude.ai-Anmeldung zu leiten, setzen Sie `forceLoginMethod` auf `"claudeai"`.

Entwickler können sich von mehreren Pfaden aus anmelden: der Terminal-`/login`-Fluss, die [VS Code-Erweiterung](/docs/de/vs-code), das Agent SDK, `claude setup-token`, `/install-github-app` und [Gateway](/docs/de/claude-apps-gateway)-Anmeldung für Organisationen, die über ein Cloud-Gateway weiterleiten. Auf Claude Code v2.1.212 oder später wendet jeder Pfad `forceLoginMethod` an; vor v2.1.212 wendeten nur Terminal-Anmeldungen einen der Schlüssel an. Auf dem interaktiven Anmeldebildschirm des Terminals, der durch `/login` oder Onboarding beim ersten Start erreicht wird, wählt Claude Code eine `claudeai`- oder `console`-Methode vor, ohne sie zu erzwingen, daher kann ein Entwickler auch mit `forceLoginMethod` auf `"claudeai"` gesetzt immer noch eine Console-Anmeldung dort abschließen. Die Pfade unterscheiden sich bei `forceLoginOrgUUID`:

* **Terminal-, VS Code-Erweiterungs- und Agent SDK-Anmeldungen**: Überprüfen Sie `forceLoginOrgUUID` für claude.ai-Kontoanmeldungen
* **`claude setup-token` und `/install-github-app`**: Erzwingen Sie nur `forceLoginMethod`, daher können sie ein Token in einer anderen Organisation prägen
* **[Gateway](/docs/de/claude-apps-gateway)-Anmeldung**: Wird durch `forceLoginMethod: "gateway"` ausgewählt, anstatt dadurch eingeschränkt zu werden, und authentifiziert sich nicht gegen eine Anthropic-Organisation, daher gilt `forceLoginOrgUUID` nicht; verwenden Sie Ihren Gateway-Identitätsanbieter, um den Zugriff einzuschränken

Stellen Sie die Schlüssel über Ihre Geräteverwaltungstools bereit. [Serverseitig verwaltete Einstellungen](/docs/de/server-managed-settings) erreichen nur Konten, die bereits bei Ihrer Organisation authentifiziert sind, daher können sie die erste Anmeldung eines Entwicklers nicht umleiten. Wenn Ihre Organisation auch serverseitig verwaltete Einstellungen verteilt, setzen Sie die Schlüssel an beiden Orten: Verwaltete Einstellungsquellen [werden nicht zusammengeführt](/docs/de/server-managed-settings#settings-precedence), und zwischengespeicherte serverseitig verwaltete Einstellungen ersetzen die geräteverwaltete Datei, mit Ausnahme von einigen [pro-Schlüssel-Ausnahmen](/docs/de/server-managed-settings#per-key-exceptions-across-managed-sources). `forceLoginOrgUUID` und die `"claudeai"`- und `"console"`-Werte von `forceLoginMethod` sind nicht unter diesen Ausnahmen, daher behalten Sie sie an beiden Orten.

Die Schlüssel entscheiden auch, ob eine Sitzung, die keine Anmeldedaten verwendet, gestartet werden kann. Siehe [`forceLoginOrgUUID`](/docs/de/settings-reference#forceloginorguuid) in der Einstellungsreferenz für das vollständige Verhalten.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper`**: Beim Start blockiert, da die Organisationszugehörigkeit für eine Umgebungsanmeldedaten nicht überprüft werden kann
* **Cloud-Anbieter-Sitzungen wie Amazon Bedrock**: Nicht blockiert, da sie sich gegen Ihren Cloud-Anbieter authentifizieren. Beschränken Sie diese durch Ihre Cloud-IAM-Richtlinien
* **[Anthropic-Profil oder Verbundanmeldedaten](#anthropic-profiles-and-federation-credentials)**: Nicht blockiert, und die Schlüssel überprüfen nicht, zu welcher Organisation das Profil gehört

<h2 id="credential-management">
  Verwaltung von Anmeldedaten
</h2>

Claude Code verwaltet Ihre Authentifizierungsanmeldedaten sicher:

* **Speicherort**:
  * Auf macOS werden Anmeldedaten im verschlüsselten macOS Keychain gespeichert. Wenn der Keychain den Schreibzugriff ablehnt, z. B. wenn er in einer SSH-Sitzung gesperrt ist, speichert Claude Code Ihre Anmeldung stattdessen in `~/.claude/.credentials.json` mit Dateimodus `0600`, dem gleichen Speicher, den es unter Linux verwendet. Eine Console-Anmeldung, die einen API-Schlüssel erstellt, schlägt fehl, bis der Keychain beschreibbar ist. Um Ihre Anmeldung zurück in den Keychain zu verschieben, folgen Sie [den Wiederherstellungsschritten](/docs/de/troubleshoot-install#not-logged-in-or-token-expired).
  * Auf Linux werden Anmeldedaten in `~/.claude/.credentials.json` mit Dateimodus `0600` gespeichert.
  * Unter Windows werden Anmeldedaten in `%USERPROFILE%\.claude\.credentials.json` gespeichert und erben die Zugriffskontrolle Ihres Benutzerprofilverzeichnisses, das die Datei standardmäßig auf Ihr Benutzerkonto beschränkt.
  * Wenn Sie die Umgebungsvariable `CLAUDE_CONFIG_DIR` gesetzt haben, behält Claude Code die Datei `.credentials.json` stattdessen in diesem Verzeichnis, einschließlich der Datei, die der macOS-Fallback schreibt, und schlüsselt den macOS Keychain-Eintrag auch zu diesem Verzeichnis, sodass eine Sitzung mit einem anderen `CLAUDE_CONFIG_DIR` einen anderen Eintrag liest.
  * Claude Code verwaltet `.credentials.json` über `/login` und `/logout`. Um Anfragen über einen benutzerdefinierten API-Endpunkt zu leiten, legen Sie stattdessen die Umgebungsvariable [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) fest.
* **Unterstützte Authentifizierungstypen**: Claude.ai-Anmeldedaten, Claude API-Anmeldedaten, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, Anthropic-Profil und [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) Anmeldedaten sowie [Claude Apps Gateway](/docs/de/claude-apps-gateway) Sitzungs-Tokens.
* **Benutzerdefinierte Anmeldedaten-Skripte**: Konfigurieren Sie die Einstellung [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper), um ein Shell-Skript auszuführen, das einen API-Schlüssel zurückgibt.
* **Aktualisierungsintervalle**: Claude Code führt `apiKeyHelper` standardmäßig nach fünf Minuten erneut aus. Legen Sie die Umgebungsvariable `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` für benutzerdefinierte Aktualisierungsintervalle fest. Siehe [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper) für die anderen Fälle, in denen Claude Code das Hilfsprogramm erneut ausführt.
* **Warnung bei langsamen Hilfsprogrammen**: Wenn `apiKeyHelper` länger als 10 Sekunden benötigt, um einen Schlüssel zurückzugeben, zeigt Claude Code eine Warnmitteilung in der Eingabeaufforderungsleiste an, die die verstrichene Zeit anzeigt. Wenn Sie diese Mitteilung regelmäßig sehen, überprüfen Sie, ob Ihr Anmeldedaten-Skript optimiert werden kann.
* **Fehler bei Hilfsprogrammen**: Wenn das Skript mit einem Fehler beendet wird, das Zeitlimit überschreitet oder nichts ausgibt, schlagen Anfragen mit [`Your apiKeyHelper script is failing`](/docs/de/errors#your-apikeyhelper-script-is-failing) innerhalb von drei Versuchen fehl. Vor v2.1.208 wurden Fehler bei Hilfsprogrammen nach etwa zehn stillen Wiederholungen als generischer 401-Fehler angezeigt.

`apiKeyHelper`, `ANTHROPIC_API_KEY` und `ANTHROPIC_AUTH_TOKEN` gelten für die CLI und die Oberflächen, die sie umhüllen, einschließlich der VS Code-Erweiterung, des Agent SDK und GitHub Actions. Claude Desktop und Cloud-Sitzungen rufen `apiKeyHelper` nicht auf oder lesen diese Umgebungsvariablen nicht: Sie verwenden OAuth, außer Desktop-Sitzungen, die eine [Inferenzkonfiguration eines Drittanbieters](/docs/de/llm-gateway-connect#desktop-app) ausführen, die sich mit den Anmeldedaten dieser Konfiguration authentifizieren.

<h3 id="renew-an-expiring-login">
  Ablauf einer Anmeldung erneuern
</h3>

Wenn die Anmeldung, die Sie mit `/login` erstellt haben, innerhalb von drei Tagen abläuft, zeigt Claude Code beim Start eine Warnung an: `Your login expires in 3 days · run /login to renew`. Erfordert Claude Code v2.1.203 oder später. Vor v2.1.217 erschien die Warnung fünf Tage vorher.

Führen Sie `/login` aus, um zu erneuern. Die Warnung ist informativ und blockiert niemals eine Anfrage: Die Authentifizierung funktioniert weiterhin, bis die Anmeldung tatsächlich abläuft. Die Anmeldelebensdauer selbst bleibt unverändert; die Vorauswarnung ist das, was v2.1.203 hinzufügt.

Sobald die gespeicherte Anmeldung abläuft und nicht aktualisiert werden kann, schlägt jede Modellanfrage mit [`Login expired · Please run /login`](/docs/de/errors#login-expired) fehl, bis Sie sich erneut anmelden. Vor v2.1.206 wurde eine abgelaufene Anmeldung stattdessen als Modellfehler angezeigt.

Sie können diesen Zustand vor einem Anfragefehler überprüfen: [`/status`](/docs/de/commands) zeigt eine Zeile `Login` an, die `Expired — log in again` liest, plus die Organisation und E-Mail, die es für die abgelaufene Anmeldung gespeichert hat. Die Zeile wird nur angezeigt, wenn die gespeicherte claude.ai- oder Claude Console-Anmeldung die aktive Anmeldedaten ist. Die Zeile erfordert Claude Code v2.1.210 oder später.

Die Warnung wird nur angezeigt, wenn eine claude.ai- oder Claude Console-Anmeldung die aktive Anmeldedaten ist, und nicht, wenn ein Cloud-Anbieter, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper` die Anmeldedaten bereitstellt.

Eine frühzeitige Erneuerung ist am wichtigsten für Sitzungen, die unbeaufsichtigt ausgeführt werden. Eine [Hintergrundsitzung in der Agent-Ansicht](/docs/de/agent-view) oder eine [Remote Control](/docs/de/remote-control) Sitzung, die länger als die Anmeldung läuft, stoppt den Fortschritt, sobald die Anmeldedaten ablaufen, und kann sich nicht erholen, bis Sie sich erneut anmelden.

<h3 id="authentication-precedence">
  Authentifizierungspriorität
</h3>

Wenn mehrere Anmeldedaten vorhanden sind, wählt Claude Code eines in dieser Reihenfolge:

1. Cloud-Anbieter-Anmeldedaten, wenn `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` oder `CLAUDE_CODE_USE_FOUNDRY` gesetzt ist. Siehe [Integrationen von Drittanbietern](/docs/de/third-party-integrations) für die Einrichtung.
2. `ANTHROPIC_AUTH_TOKEN` Umgebungsvariable. Wird als `Authorization: Bearer` Header gesendet. Verwenden Sie dies, wenn Sie durch ein [LLM-Gateway oder einen Proxy](/docs/de/llm-gateway) leiten, das sich mit Bearer-Tokens anstelle von Anthropic API-Schlüsseln authentifiziert.
3. `ANTHROPIC_API_KEY` Umgebungsvariable. Wird als `X-Api-Key` Header gesendet. Verwenden Sie dies für direkten Anthropic API-Zugriff mit einem Schlüssel aus der [Claude Console](https://platform.claude.com). Im interaktiven Modus werden Sie einmal aufgefordert, den Schlüssel zu genehmigen oder abzulehnen, und Ihre Wahl wird gespeichert. Um dies später zu ändern, verwenden Sie den Umschalter „Use custom API key" in `/config`. Der Umschalter wird nur angezeigt, während `ANTHROPIC_API_KEY` in Ihrer Umgebung gesetzt ist. Im nicht-interaktiven Modus (`-p`) wird der Schlüssel immer verwendet, wenn er vorhanden ist.
4. [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper) Skriptausgabe. Verwenden Sie dies für dynamische oder rotierende Anmeldedaten, wie kurzlebige Tokens, die aus einem Vault abgerufen werden.
5. `CLAUDE_CODE_OAUTH_TOKEN` Umgebungsvariable. Ein langlebiges OAuth-Token, das von [`claude setup-token`](#generate-a-long-lived-token) generiert wird. Verwenden Sie dies für CI-Pipelines und Skripte, bei denen Browser-Anmeldung nicht verfügbar ist. Wenn Sie `/login` ausführen, während die Variable gesetzt ist, wechselt Claude Code die aktuelle Sitzung zur neuen Anmeldung, liest die Variable aber in jeder neuen Sitzung erneut, bis Sie sie aus Ihrem Shell-Profil oder dem `env` Block einer [Einstellungsdatei](/docs/de/settings) entfernen.
6. Anthropic-Profil und Verbundsanmeldedaten, die Anmeldedaten, die die `ant` CLI und Workload Identity Federation verwenden. Ein Profil, das `ant auth login` geschrieben hat, wird hier nur eingestuft, wenn Sie es in `ANTHROPIC_PROFILE` benennen; andernfalls wird es unter `/login` eingestuft. Siehe [Anthropic-Profile und Verbundsanmeldedaten](#anthropic-profiles-and-federation-credentials).
7. Abonnement-OAuth-Anmeldedaten von `/login`. Dies ist die Standardeinstellung für Claude Pro, Max, Team und Enterprise-Benutzer.

Eine angemeldete [Claude Apps Gateway](/docs/de/claude-apps-gateway) Sitzung steht außerhalb dieser Liste: Sie ist eine Anbieterauswahl wie Amazon Bedrock oder Google Cloud's Agent Platform und hat Vorrang vor ihnen. Wenn eine Gateway-Sitzung vorhanden ist, authentifiziert sich die CLI mit dem Gateway-Token, auch wenn `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` oder `CLAUDE_CODE_USE_FOUNDRY` gesetzt ist, und Anmeldequellen oben wie der Bearer-Token, API-Schlüssel, `apiKeyHelper` und Profile werden nicht verwendet.

Wenn die [verwalteten Einstellungen](/docs/de/managed-settings) Ihres Computers [`forceLoginMethod`](/docs/de/settings-reference#forceloginmethod) auf `"gateway"` setzen oder [`forceLoginGatewayUrl`](/docs/de/settings-reference#forcelogingatewayurl) setzen, und Sie keinen Cloud-Anbieter durch eine Variable wie `CLAUDE_CODE_USE_BEDROCK` oder `CLAUDE_CODE_USE_VERTEX` auswählen, verwendet Ihre Sitzung nur die Gateway-Anmeldung. Claude Code überspringt die anderen Anmeldequellen und fordert Sie auf, sich mit `/login` anzumelden. Siehe [Administrator policy requires a Cloud gateway sign-in](/docs/de/errors#administrator-policy-requires-a-cloud-gateway-sign-in) für das, was Sie mit jeder verbleibenden Anmeldedaten sehen. Vor v2.1.261 oder vor v2.1.265 auf einem Computer, der nur `forceLoginGatewayUrl` setzt, verwendete Claude Code eine verbleibende gespeicherte Anmeldung auf diesen Computern, bis Sie sich beim Gateway anmeldeten.

Wenn Sie ein aktives Claude-Abonnement haben, aber auch `ANTHROPIC_API_KEY` in Ihrer Umgebung gesetzt haben, verwendet Claude Code den API-Schlüssel, sobald Sie ihn genehmigen. Dies kann zu Authentifizierungsfehlern führen, wenn der Schlüssel zu einer deaktivierten oder abgelaufenen Organisation gehört.

Führen Sie `unset ANTHROPIC_API_KEY` aus, um auf Ihr Abonnement zurückzugreifen, und überprüfen Sie `/status`, um zu bestätigen, welche Methode aktiv ist. Wenn sowohl eine Anmeldung als auch ein API-Schlüssel konfiguriert sind, markiert `/status` die Anmeldedaten, die nicht verwendet werden.

[Claude Code im Web](/docs/de/claude-code-on-the-web) verwendet immer Ihre Abonnement-Anmeldedaten. Wenn Sie `ANTHROPIC_API_KEY` oder `ANTHROPIC_AUTH_TOKEN` in der Cloud-Umgebung gesetzt haben, überschreiben diese nicht Ihre Abonnement-Anmeldedaten.

<h4 id="anthropic-profiles-and-federation-credentials">
  Anthropic-Profile und Verbundsanmeldedaten
</h4>

Ein Profil ist eine benannte Anmeldedaten-Konfigurationsdatei in Ihrem [Anthropic-Konfigurationsverzeichnis](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory), standardmäßig `~/.config/anthropic` auf macOS und Linux oder `%APPDATA%\Anthropic` unter Windows. Der Authentifizierungsmodus eines Profils ist `oidc_federation`, wenn Sie ihn für [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) einrichten, oder `user_oauth`, wenn [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication) ihn geschrieben hat oder Sie [sich bei einem Console-Konto ohne API-Schlüssel angemeldet haben](#sign-in-without-an-api-key).

Claude Code liest Profile oder Verbundsvariablen nicht im [Bare Mode](/docs/de/headless#start-faster-with-bare-mode), in Claude Desktop oder in Cloud-Sitzungen. In diesen Sitzungen zeigt `/status` keine `Profile` Zeile an.

Claude Code überprüft drei Quellen in dieser Reihenfolge und stoppt bei der ersten, die gesetzt ist. Die Tabelle zeigt, was jede Quelle setzt und wo sie gegen Ihre `/login` Anmeldedaten eingestuft wird.

| Quelle            | Gesetzt von                                                                                                                                                                     | Rang gegen `/login`                                                                                                                                                   |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Benanntes Profil  | `ANTHROPIC_PROFILE`                                                                                                                                                             | Oben, unabhängig vom Authentifizierungsmodus des Profils                                                                                                              |
| Verbundsvariablen | `ANTHROPIC_FEDERATION_RULE_ID` und `ANTHROPIC_ORGANIZATION_ID`, beide gesetzt                                                                                                   | Oben                                                                                                                                                                  |
| Aktives Profil    | Die [`active_config` Datei](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) in Ihrem Konfigurationsverzeichnis oder ein Profil namens `default` | Oben, wenn sein Authentifizierungsmodus `oidc_federation` ist; unten einer funktionierenden `/login` Anmeldedaten, wenn sein Authentifizierungsmodus `user_oauth` ist |

Die `user_oauth` Regel verhindert, dass ein verwaistes `ant auth login` Profil Ihre Anfragen vom Konto weg verschiebt, bei dem Sie sich mit `/login` angemeldet haben. Für die Verbundsvariablen liest Claude Code auch die anderen Variablen in der [WIF-Referenz](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables), wie `ANTHROPIC_IDENTITY_TOKEN_FILE`, wenn es Ihr Identitäts-Token austauscht. Für das Profildateiformat siehe die [WIF-Referenz](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file).

Um zu bestätigen, welche Quelle Claude Code gewählt hat, führen Sie `/status` aus. Eine `Profile` Zeile benennt die Quelle anstelle der `Login method` Zeile. Wenn die Anmeldedaten die verwendete Anmeldedaten sind, zeigen die Zeilen `Organization` und `Email` ihr Konto an.

Wenn Sie Claude Code mit `--debug` starten, schreibt es auch eine `Using Anthropic profile auth` Zeile mit dem Quellnamen in das Debug-Protokoll unter `~/.claude/debug/<session-id>.txt`. Wenn Claude Code ein `user_oauth` aktives Profil überspringt, weil Sie eine funktionierend `/login` Anmeldedaten haben, schreibt es eine Warnung in das Debug-Protokoll, dass es stattdessen die claude.ai-Anmeldung verwendet.

Wenn die Anmeldung eines `user_oauth` Profils abgelaufen ist und Claude Code sie nicht erneuern kann, schlagen Anfragen mit [Anthropic profile login expired](/docs/de/errors#anthropic-profile-login-expired) fehl.

Funktionen, die Ihre claude.ai-Anmeldung benötigen, wie [claude.ai Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) und [`/schedule`](/docs/de/routines), sind nicht verfügbar, während eine dieser Quellen ausgewählt ist. Um Claude Code davon abzuhalten, eine Quelle auszuwählen:

* **Benanntes Profil oder Verbundsvariablen**: Heben Sie `ANTHROPIC_PROFILE` auf oder heben Sie eine der Verbundsvariablen auf
* **Aktives Profil**: Führen Sie `/logout` für ein `user_oauth` Profil aus, dessen aktuelle Anmeldedaten Sie durch [Anmeldung bei einem Console-Konto ohne API-Schlüssel](#sign-in-without-an-api-key) geschrieben haben, führen Sie `ant auth logout` für eines aus, dessen aktuelle Anmeldedaten `ant auth login` geschrieben hat, oder löschen Sie die Profildatei aus `configs/` in Ihrem Konfigurationsverzeichnis für einen der beiden Authentifizierungsmodi

<h3 id="generate-a-long-lived-token">
  Generieren Sie ein langlebiges Token
</h3>

Für CI-Pipelines, Skripte oder andere Umgebungen, in denen interaktive Browser-Anmeldung nicht verfügbar ist, generieren Sie ein einjähriges OAuth-Token mit `claude setup-token`:

```bash theme={null}
claude setup-token
```

Der Befehl öffnet den gleichen Browser-Autorisierungsfluss wie `/login`, und das Token wird im Terminal ausgegeben, nachdem Sie den Zugriff im Browser genehmigt haben. Es speichert das Token nirgendwo; kopieren Sie es und legen Sie es als `CLAUDE_CODE_OAUTH_TOKEN` Umgebungsvariable überall dort fest, wo Sie sich authentifizieren möchten:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Dieses Token authentifiziert sich mit Ihrem Claude-Abonnement und erfordert einen Pro-, Max-, Team- oder Enterprise-Plan. Es kann nur Modellanfragen stellen, daher kann es keine [Remote Control](/docs/de/remote-control) Sitzungen einrichten oder [claude.ai Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) abrufen. MCP-Server, die Sie lokal konfigurieren, funktionieren weiterhin.

[Bare Mode](/docs/de/headless#start-faster-with-bare-mode) liest `CLAUDE_CODE_OAUTH_TOKEN` nicht. Wenn Ihr Skript `--bare` übergibt, authentifizieren Sie sich stattdessen mit `ANTHROPIC_API_KEY` oder einem `apiKeyHelper`.
