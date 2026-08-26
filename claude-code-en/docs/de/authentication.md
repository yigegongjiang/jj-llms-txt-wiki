> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Authentifizierung

> Melden Sie sich bei Claude Code an und konfigurieren Sie die Authentifizierung für Einzelpersonen, Teams und Organisationen.

Claude Code unterstützt mehrere Authentifizierungsmethoden je nach Ihrer Einrichtung. Einzelne Benutzer können sich mit einem Claude.ai-Konto anmelden, während Teams Claude for Teams oder Enterprise, die Claude Console oder einen Cloud-Anbieter wie Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verwenden können.

<h2 id="log-in-to-claude-code">
  Melden Sie sich bei Claude Code an
</h2>

Nach dem [Installieren von Claude Code](/docs/de/setup#install-claude-code) führen Sie `claude` in Ihrem Terminal aus. Beim ersten Start öffnet Claude Code ein Browserfenster, in dem Sie sich anmelden können.

Wenn der Browser nicht automatisch geöffnet wird, drücken Sie `c`, um die Anmelde-URL in Ihre Zwischenablage zu kopieren, und fügen Sie sie dann in Ihren Browser ein.

Wenn Ihr Browser nach der Anmeldung einen Anmeldecode anzeigt, anstatt Sie zurückzuleiten, fügen Sie ihn an der Eingabeaufforderung `Paste code here if prompted` im Terminal ein. Dies geschieht, wenn der Browser den lokalen Callback-Server von Claude Code nicht erreichen kann, was in WSL2, SSH-Sitzungen und Containern häufig vorkommt.

Wenn die Anmeldung abgeschlossen ist, zeigt das Terminal `Login successful` an und fordert Sie auf, die `Eingabetaste` zu drücken, um fortzufahren.

Sie können sich mit einem dieser Kontotypen authentifizieren:

* **Claude Pro oder Max Abonnement**: Melden Sie sich mit Ihrem Claude.ai-Konto an. Abonnieren Sie unter [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams oder Enterprise**: Melden Sie sich mit dem Claude.ai-Konto an, zu dem Sie Ihr Team-Administrator eingeladen hat.
* **Claude Console**: Melden Sie sich mit Ihren Console-Anmeldedaten an. Ihr Administrator muss Sie zunächst [eingeladen haben](#claude-console-authentication).
* **Cloud-Anbieter**: Wenn Ihre Organisation [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry) verwendet, legen Sie die erforderlichen Umgebungsvariablen fest, bevor Sie `claude` ausführen, oder wählen Sie **3rd-party platform** bei der Anmeldeeingabeaufforderung aus, die einen interaktiven Setup-Assistenten für Bedrock und Vertex AI startet. Es ist keine Browser-Anmeldung erforderlich.
* **Cloud-Gateway**: Wenn Ihre Organisation ein selbstgehostetes [Claude Apps Gateway](/docs/de/claude-apps-gateway) betreibt, melden Sie sich über `/login` mit Corporate SSO an. Das vom Gateway ausgegebene Token ist die einzige Anmeldeinformation der Sitzung.

Administratoren können die interaktive Anmeldung mit den verwalteten Einstellungen [`forceLoginMethod` und `forceLoginOrgUUID`](/docs/de/settings#available-settings) einschränken. Wenn eine dieser Einstellungen gesetzt ist, werden Sitzungen, die von `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper` authentifiziert werden, beim Start blockiert; Cloud-Provider-Sitzungen sind nicht betroffen.

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

* **Claude for Teams**: Self-Service-Plan mit Zusammenarbeitsfunktionen, Admin-Tools und Abrechnungsverwaltung. Am besten für kleinere Teams.
* **Claude for Enterprise**: Fügt SSO, Domain-Erfassung, rollenbasierte Berechtigungen, Compliance-API und verwaltete Richtlinieneinstellungen für organisationsweite Claude Code-Konfigurationen hinzu. Am besten für größere Organisationen mit Sicherheits- und Compliance-Anforderungen.

<Steps>
  <Step title="Abonnieren">
    Abonnieren Sie [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) oder kontaktieren Sie den Vertrieb für [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Team-Mitglieder einladen">
    Laden Sie Team-Mitglieder vom Admin-Dashboard ein.
  </Step>

  <Step title="Installieren und anmelden">
    Team-Mitglieder installieren Claude Code und melden sich mit ihren Claude.ai-Konten an.
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

<h2 id="credential-management">
  Verwaltung von Anmeldedaten
</h2>

Claude Code verwaltet Ihre Authentifizierungsanmeldedaten sicher:

* **Speicherort**:
  * Auf macOS werden Anmeldedaten im verschlüsselten macOS Keychain gespeichert.
  * Auf Linux werden Anmeldedaten in `~/.claude/.credentials.json` mit Dateimodus `0600` gespeichert.
  * Unter Windows werden Anmeldedaten in `%USERPROFILE%\.claude\.credentials.json` gespeichert und erben die Zugriffskontrolle Ihres Benutzerprofilverzeichnisses, das die Datei standardmäßig auf Ihr Benutzerkonto beschränkt.
  * Wenn Sie die Umgebungsvariable `CLAUDE_CONFIG_DIR` unter Linux oder Windows gesetzt haben, befindet sich die Datei `.credentials.json` stattdessen in diesem Verzeichnis.
  * Claude Code verwaltet `.credentials.json` über `/login` und `/logout`. Um Anfragen über einen benutzerdefinierten API-Endpunkt zu leiten, legen Sie stattdessen die Umgebungsvariable [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) fest.
* **Unterstützte Authentifizierungstypen**: Claude.ai-Anmeldedaten, Claude API-Anmeldedaten, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth und [Claude Apps Gateway](/docs/de/claude-apps-gateway) Sitzungs-Tokens.
* **Benutzerdefinierte Anmeldedaten-Skripte**: Die Einstellung [`apiKeyHelper`](/docs/de/settings#available-settings) kann so konfiguriert werden, dass ein Shell-Skript ausgeführt wird, das einen API-Schlüssel zurückgibt.
* **Aktualisierungsintervalle**: Standardmäßig wird `apiKeyHelper` nach 5 Minuten oder bei HTTP 401-Antwort aufgerufen. Legen Sie die Umgebungsvariable `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` für benutzerdefinierte Aktualisierungsintervalle fest.
* **Warnung bei langsamen Hilfsprogrammen**: Wenn `apiKeyHelper` länger als 10 Sekunden benötigt, um einen Schlüssel zurückzugeben, zeigt Claude Code eine Warnmitteilung in der Eingabeaufforderungsleiste an, die die verstrichene Zeit anzeigt. Wenn Sie diese Mitteilung regelmäßig sehen, überprüfen Sie, ob Ihr Anmeldedaten-Skript optimiert werden kann.
* **Fehler bei Hilfsprogrammen**: Wenn das Skript mit einem Fehler beendet wird, das Zeitlimit überschreitet oder nichts ausgibt, schlagen Anfragen mit [`Your apiKeyHelper script is failing`](/docs/de/errors#your-apikeyhelper-script-is-failing) innerhalb von drei Versuchen fehl. Vor v2.1.208 wurden Fehler bei Hilfsprogrammen nach etwa zehn stillen Wiederholungen als generischer 401-Fehler angezeigt.

`apiKeyHelper`, `ANTHROPIC_API_KEY` und `ANTHROPIC_AUTH_TOKEN` gelten für die CLI und die Oberflächen, die sie umhüllen, einschließlich der VS Code-Erweiterung, des Agent SDK und GitHub Actions. Claude Desktop und Cloud-Sitzungen rufen `apiKeyHelper` nicht auf oder lesen diese Umgebungsvariablen nicht: Sie verwenden OAuth, außer Desktop-Sitzungen, die eine [Inferenzkonfiguration eines Drittanbieters](/docs/de/llm-gateway-connect#desktop-app) ausführen, die sich mit den Anmeldedaten dieser Konfiguration authentifizieren.

<h3 id="renew-an-expiring-login">
  Ablauf einer Anmeldung erneuern
</h3>

Wenn die Anmeldung, die Sie mit `/login` erstellt haben, innerhalb von fünf Tagen abläuft, zeigt Claude Code beim Start eine Warnung an: `Your login expires in 3 days · run /login to renew`. Erfordert Claude Code v2.1.203 oder später.

Führen Sie `/login` aus, um zu erneuern. Die Warnung ist informativ und blockiert niemals eine Anfrage: Die Authentifizierung funktioniert weiterhin, bis die Anmeldung tatsächlich abläuft. Die Anmeldelebensdauer selbst bleibt unverändert; die Vorauswarnung ist das, was v2.1.203 hinzufügt.

Sobald die gespeicherte Anmeldung abläuft und nicht aktualisiert werden kann, schlägt jede Anfrage mit [`Login expired · Please run /login`](/docs/de/errors#login-expired) fehl, bis Sie sich erneut anmelden. Vor v2.1.206 wurde eine abgelaufene Anmeldung stattdessen als Modellfehler angezeigt.

Die Warnung wird nur angezeigt, wenn eine claude.ai- oder Claude Console-Anmeldung die aktive Anmeldedaten ist, und nicht, wenn ein Cloud-Anbieter, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper` die Anmeldedaten bereitstellt.

Eine frühzeitige Erneuerung ist am wichtigsten für Sitzungen, die unbeaufsichtigt ausgeführt werden. Eine [Hintergrundsitzung in der Agent-Ansicht](/docs/de/agent-view) oder eine [Remote Control](/docs/de/remote-control) Sitzung, die länger als die Anmeldung läuft, stoppt den Fortschritt, sobald die Anmeldedaten ablaufen, und kann sich nicht erholen, bis Sie sich erneut anmelden.

<h3 id="authentication-precedence">
  Authentifizierungspriorität
</h3>

Wenn mehrere Anmeldedaten vorhanden sind, wählt Claude Code eines in dieser Reihenfolge:

1. Cloud-Anbieter-Anmeldedaten, wenn `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` oder `CLAUDE_CODE_USE_FOUNDRY` gesetzt ist. Siehe [Integrationen von Drittanbietern](/docs/de/third-party-integrations) für die Einrichtung.
2. `ANTHROPIC_AUTH_TOKEN` Umgebungsvariable. Wird als `Authorization: Bearer` Header gesendet. Verwenden Sie dies, wenn Sie durch ein [LLM-Gateway oder einen Proxy](/docs/de/llm-gateway) leiten, das sich mit Bearer-Tokens anstelle von Anthropic API-Schlüsseln authentifiziert.
3. `ANTHROPIC_API_KEY` Umgebungsvariable. Wird als `X-Api-Key` Header gesendet. Verwenden Sie dies für direkten Anthropic API-Zugriff mit einem Schlüssel aus der [Claude Console](https://platform.claude.com). Im interaktiven Modus werden Sie einmal aufgefordert, den Schlüssel zu genehmigen oder abzulehnen, und Ihre Wahl wird gespeichert. Um dies später zu ändern, verwenden Sie den Umschalter „Use custom API key" in `/config`. Der Umschalter wird nur angezeigt, während `ANTHROPIC_API_KEY` in Ihrer Umgebung gesetzt ist. Im nicht-interaktiven Modus (`-p`) wird der Schlüssel immer verwendet, wenn er vorhanden ist.
4. [`apiKeyHelper`](/docs/de/settings#available-settings) Skriptausgabe. Verwenden Sie dies für dynamische oder rotierende Anmeldedaten, wie kurzlebige Tokens, die aus einem Vault abgerufen werden.
5. `CLAUDE_CODE_OAUTH_TOKEN` Umgebungsvariable. Ein langlebiges OAuth-Token, das von [`claude setup-token`](#generate-a-long-lived-token) generiert wird. Verwenden Sie dies für CI-Pipelines und Skripte, bei denen Browser-Anmeldung nicht verfügbar ist.
6. Abonnement-OAuth-Anmeldedaten von `/login`. Dies ist die Standardeinstellung für Claude Pro, Max, Team und Enterprise-Benutzer.

Eine angemeldete [Claude Apps Gateway](/docs/de/claude-apps-gateway) Sitzung steht außerhalb dieser Liste: Sie ist eine Anbieterauswahl wie Amazon Bedrock oder Google Cloud's Agent Platform und hat Vorrang vor ihnen. Wenn eine Gateway-Sitzung vorhanden ist, authentifiziert sich die CLI mit dem Gateway-Token, auch wenn `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` oder `CLAUDE_CODE_USE_FOUNDRY` gesetzt ist, und die Bearer-Token-, API-Schlüssel- und `apiKeyHelper`-Einträge oben werden nicht verwendet.

Wenn Sie ein aktives Claude-Abonnement haben, aber auch `ANTHROPIC_API_KEY` in Ihrer Umgebung gesetzt haben, hat der API-Schlüssel Vorrang, sobald er genehmigt ist. Dies kann zu Authentifizierungsfehlern führen, wenn der Schlüssel zu einer deaktivierten oder abgelaufenen Organisation gehört. Führen Sie `unset ANTHROPIC_API_KEY` aus, um auf Ihr Abonnement zurückzugreifen, und überprüfen Sie `/status`, um zu bestätigen, welche Methode aktiv ist. Die Zeile `Login method` zeigt Ihr Abonnementkonto an, und eine Zeile `API key` wird angezeigt, wenn ein API-Schlüssel verwendet wird.

[Claude Code im Web](/docs/de/claude-code-on-the-web) verwendet immer Ihre Abonnement-Anmeldedaten. Wenn Sie `ANTHROPIC_API_KEY` oder `ANTHROPIC_AUTH_TOKEN` in der Sandbox-Umgebung gesetzt haben, überschreiben diese nicht Ihre Abonnement-Anmeldedaten.

<h3 id="generate-a-long-lived-token">
  Generieren Sie ein langlebiges Token
</h3>

Für CI-Pipelines, Skripte oder andere Umgebungen, in denen interaktive Browser-Anmeldung nicht verfügbar ist, generieren Sie ein einjähriges OAuth-Token mit `claude setup-token`:

```bash theme={null}
claude setup-token
```

Der Befehl führt Sie durch die OAuth-Autorisierung und gibt ein Token im Terminal aus. Er speichert das Token nirgendwo; kopieren Sie es und legen Sie es als `CLAUDE_CODE_OAUTH_TOKEN` Umgebungsvariable überall dort fest, wo Sie sich authentifizieren möchten:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Dieses Token authentifiziert sich mit Ihrem Claude-Abonnement und erfordert einen Pro-, Max-, Team- oder Enterprise-Plan. Es ist auf Inferenz beschränkt und kann keine [Remote Control](/docs/de/remote-control) Sitzungen einrichten.

[Bare Mode](/docs/de/headless#start-faster-with-bare-mode) liest `CLAUDE_CODE_OAUTH_TOKEN` nicht. Wenn Ihr Skript `--bare` übergibt, authentifizieren Sie sich stattdessen mit `ANTHROPIC_API_KEY` oder einem `apiKeyHelper`.
