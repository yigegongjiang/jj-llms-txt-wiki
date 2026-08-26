> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ereignisse mit Kanälen in eine laufende Sitzung übertragen

> Verwenden Sie Kanäle, um Nachrichten, Benachrichtigungen und Webhooks von einem MCP-Server in Ihre Claude Code-Sitzung zu übertragen. Leiten Sie CI-Ergebnisse, Chat-Nachrichten und Überwachungsereignisse weiter, damit Claude reagieren kann, während Sie weg sind.

<Note>
  Kanäle befinden sich in der [Forschungsvorschau](#research-preview). Sie erfordern eine Anthropic-Authentifizierung über claude.ai oder einen Console-API-Schlüssel und sind nicht auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar. Team- und Enterprise-Organisationen müssen [sie explizit aktivieren](#enterprise-controls).
</Note>

Ein Kanal ist ein MCP-Server, der Ereignisse in Ihre laufende Claude Code-Sitzung überträgt, damit Claude auf Dinge reagieren kann, die passieren, während Sie nicht am Terminal sind. Kanäle können bidirektional sein: Claude liest das Ereignis und antwortet über denselben Kanal zurück, wie eine Chat-Brücke. Ereignisse treffen nur ein, während die Sitzung offen ist. Für ein Always-On-Setup führen Sie Claude in einem Hintergrundprozess oder persistenten Terminal aus.

Im Gegensatz zu Integrationen, die eine neue Cloud-Sitzung starten oder auf Abruf warten, kommt das Ereignis in der Sitzung an, die Sie bereits offen haben: siehe [wie Kanäle sich vergleichen](#how-channels-compare).

Sie installieren einen Kanal als Plugin und konfigurieren ihn mit Ihren eigenen Anmeldedaten. Telegram, Discord und iMessage sind in der Forschungsvorschau enthalten.

Wenn Claude über einen Kanal antwortet, sehen Sie die eingehende Nachricht in Ihrem Terminal, aber nicht den Antworttext. Das Terminal zeigt den Tool-Aufruf und eine Bestätigung (wie „gesendet"), und die eigentliche Antwort erscheint auf der anderen Plattform.

Wenn Sie ein Team, Enterprise oder Console-Org verwalten, siehe [Kanäle für Ihre Organisation aktivieren](#enterprise-controls). Um Ihren eigenen Kanal zu erstellen, siehe die [Kanäle-Referenz](/docs/de/channels-reference).

<h2 id="supported-channels">
  Unterstützte Kanäle
</h2>

Jeder unterstützte Kanal ist ein Plugin, das [Bun](https://bun.sh) erfordert. Für eine praktische Demo des Plugin-Flows, bevor Sie eine echte Plattform verbinden, versuchen Sie die [fakechat-Schnellstart](#quickstart).

<Tabs>
  <Tab title="Telegram">
    Sehen Sie sich den vollständigen [Telegram-Plugin-Quellcode](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) an.

    <Steps>
      <Step title="Erstellen Sie einen Telegram-Bot">
        Öffnen Sie [BotFather](https://t.me/BotFather) in Telegram und senden Sie `/newbot`. Geben Sie ihm einen Anzeigenamen und einen eindeutigen Benutzernamen, der auf `bot` endet. Kopieren Sie das Token, das BotFather zurückgibt.
      </Step>

      <Step title="Installieren Sie das Plugin">
        In Claude Code führen Sie aus:

        ```
        /plugin install telegram@claude-plugins-official
        ```

        Wenn Claude Code meldet, dass das Plugin in keinem Marketplace gefunden wird, fehlt Ihr Marketplace oder ist veraltet. Führen Sie `/plugin marketplace update claude-plugins-official` aus, um ihn zu aktualisieren, oder `/plugin marketplace add anthropics/claude-plugins-official`, wenn Sie ihn noch nicht hinzugefügt haben. Versuchen Sie dann die Installation erneut.

        Nach der Installation führen Sie `/reload-plugins` aus, um den Konfigurationsbefehl des Plugins zu aktivieren.
      </Step>

      <Step title="Konfigurieren Sie Ihr Token">
        Führen Sie den Konfigurationsbefehl mit dem Token von BotFather aus:

        ```
        /telegram:configure <token>
        ```

        Dies speichert es in `~/.claude/channels/telegram/.env`. Sie können auch `TELEGRAM_BOT_TOKEN` in Ihrer Shell-Umgebung setzen, bevor Sie Claude Code starten.
      </Step>

      <Step title="Starten Sie mit aktivierten Kanälen neu">
        Beenden Sie Claude Code und starten Sie mit dem Kanal-Flag neu. Dies startet das Telegram-Plugin, das mit dem Abrufen von Nachrichten von Ihrem Bot beginnt:

        ```bash theme={null}
        claude --channels plugin:telegram@claude-plugins-official
        ```
      </Step>

      <Step title="Koppeln Sie Ihr Konto">
        Öffnen Sie Telegram und senden Sie eine beliebige Nachricht an Ihren Bot. Der Bot antwortet mit einem Kopplungscode.

        <Note>Wenn Ihr Bot nicht antwortet, stellen Sie sicher, dass Claude Code mit `--channels` aus dem vorherigen Schritt ausgeführt wird. Der Bot kann nur antworten, während der Kanal aktiv ist.</Note>

        Zurück in Claude Code führen Sie aus:

        ```
        /telegram:access pair <code>
        ```

        Dann sperren Sie den Zugriff, damit nur Ihr Konto Nachrichten senden kann:

        ```
        /telegram:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="Discord">
    Sehen Sie sich den vollständigen [Discord-Plugin-Quellcode](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) an.

    <Steps>
      <Step title="Erstellen Sie einen Discord-Bot">
        Gehen Sie zum [Discord Developer Portal](https://discord.com/developers/applications), klicken Sie auf **New Application** und benennen Sie ihn. Im Abschnitt **Bot** erstellen Sie einen Benutzernamen, klicken dann auf **Reset Token** und kopieren das Token.
      </Step>

      <Step title="Aktivieren Sie Message Content Intent">
        In den Einstellungen Ihres Bots scrollen Sie zu **Privileged Gateway Intents** und aktivieren **Message Content Intent**.
      </Step>

      <Step title="Laden Sie den Bot auf Ihren Server ein">
        Gehen Sie zu **OAuth2 > URL Generator**. Wählen Sie den `bot`-Bereich und aktivieren Sie diese Berechtigungen:

        * View Channels
        * Send Messages
        * Send Messages in Threads
        * Read Message History
        * Attach Files
        * Add Reactions

        Öffnen Sie die generierte URL, um den Bot zu Ihrem Server hinzuzufügen.
      </Step>

      <Step title="Installieren Sie das Plugin">
        In Claude Code führen Sie aus:

        ```
        /plugin install discord@claude-plugins-official
        ```

        Wenn Claude Code meldet, dass das Plugin in keinem Marketplace gefunden wird, fehlt Ihr Marketplace oder ist veraltet. Führen Sie `/plugin marketplace update claude-plugins-official` aus, um ihn zu aktualisieren, oder `/plugin marketplace add anthropics/claude-plugins-official`, wenn Sie ihn noch nicht hinzugefügt haben. Versuchen Sie dann die Installation erneut.

        Nach der Installation führen Sie `/reload-plugins` aus, um den Konfigurationsbefehl des Plugins zu aktivieren.
      </Step>

      <Step title="Konfigurieren Sie Ihr Token">
        Führen Sie den Konfigurationsbefehl mit dem Bot-Token aus, den Sie kopiert haben:

        ```
        /discord:configure <token>
        ```

        Dies speichert es in `~/.claude/channels/discord/.env`. Sie können auch `DISCORD_BOT_TOKEN` in Ihrer Shell-Umgebung setzen, bevor Sie Claude Code starten.
      </Step>

      <Step title="Starten Sie mit aktivierten Kanälen neu">
        Beenden Sie Claude Code und starten Sie mit dem Kanal-Flag neu. Dies verbindet das Discord-Plugin, damit Ihr Bot Nachrichten empfangen und beantworten kann:

        ```bash theme={null}
        claude --channels plugin:discord@claude-plugins-official
        ```
      </Step>

      <Step title="Koppeln Sie Ihr Konto">
        Schreiben Sie Ihrem Bot auf Discord eine Direktnachricht. Der Bot antwortet mit einem Kopplungscode.

        <Note>Wenn Ihr Bot nicht antwortet, stellen Sie sicher, dass Claude Code mit `--channels` aus dem vorherigen Schritt ausgeführt wird. Der Bot kann nur antworten, während der Kanal aktiv ist.</Note>

        Zurück in Claude Code führen Sie aus:

        ```
        /discord:access pair <code>
        ```

        Dann sperren Sie den Zugriff, damit nur Ihr Konto Nachrichten senden kann:

        ```
        /discord:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="iMessage">
    Sehen Sie sich den vollständigen [iMessage-Plugin-Quellcode](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) an.

    Der iMessage-Kanal liest Ihre Messages-Datenbank direkt und sendet Antworten über AppleScript. Er erfordert macOS und benötigt kein Bot-Token oder externen Service.

    <Steps>
      <Step title="Gewähren Sie vollständigen Festplattenzugriff">
        Die Messages-Datenbank unter `~/Library/Messages/chat.db` ist durch macOS geschützt. Wenn der Server sie zum ersten Mal liest, fordert macOS Zugriff an: klicken Sie auf **Allow**. Die Aufforderung nennt die App, die Bun gestartet hat, wie Terminal, iTerm oder Ihre IDE.

        Wenn die Aufforderung nicht angezeigt wird oder Sie auf „Don't Allow" geklickt haben, gewähren Sie den Zugriff manuell unter **System Settings > Privacy & Security > Full Disk Access** und fügen Sie Ihr Terminal hinzu. Ohne dies beendet sich der Server sofort mit `authorization denied`.
      </Step>

      <Step title="Installieren Sie das Plugin">
        In Claude Code führen Sie aus:

        ```
        /plugin install imessage@claude-plugins-official
        ```

        Wenn Claude Code meldet, dass das Plugin in keinem Marketplace gefunden wird, fehlt Ihr Marketplace oder ist veraltet. Führen Sie `/plugin marketplace update claude-plugins-official` aus, um ihn zu aktualisieren, oder `/plugin marketplace add anthropics/claude-plugins-official`, wenn Sie ihn noch nicht hinzugefügt haben. Versuchen Sie dann die Installation erneut.
      </Step>

      <Step title="Starten Sie mit aktivierten Kanälen neu">
        Beenden Sie Claude Code und starten Sie mit dem Kanal-Flag neu:

        ```bash theme={null}
        claude --channels plugin:imessage@claude-plugins-official
        ```
      </Step>

      <Step title="Schreiben Sie sich selbst">
        Öffnen Sie Messages auf einem beliebigen Gerät, das in Ihrer Apple ID angemeldet ist, und senden Sie sich selbst eine Nachricht. Sie erreicht Claude sofort: Self-Chat umgeht die Zugriffskontrolle ohne Setup.

        <Note>Die erste Antwort, die Claude sendet, löst eine macOS-Automatisierungsaufforderung aus, die fragt, ob Ihr Terminal Messages steuern kann. Klicken Sie auf **OK**.</Note>
      </Step>

      <Step title="Erlauben Sie anderen Absendern">
        Standardmäßig passieren nur Ihre eigenen Nachrichten durch. Um einem anderen Kontakt zu ermöglichen, Claude zu erreichen, fügen Sie seinen Handle hinzu:

        ```
        /imessage:access allow +15551234567
        ```

        Handles sind Telefonnummern im Format `+country` oder Apple ID-E-Mails wie `user@example.com`.
      </Step>
    </Steps>
  </Tab>
</Tabs>

Sie können auch [Ihren eigenen Kanal erstellen](/docs/de/channels-reference) für Systeme, die noch kein Plugin haben.

<h2 id="quickstart">
  Schnellstart
</h2>

Fakechat ist ein offiziell unterstützter Demo-Kanal, der eine Chat-Benutzeroberfläche auf localhost ausführt, ohne dass etwas authentifiziert werden muss und kein externer Service konfiguriert werden muss.

Sobald Sie fakechat installieren und aktivieren, können Sie im Browser eingeben und die Nachricht kommt in Ihrer Claude Code-Sitzung an. Claude antwortet, und die Antwort erscheint zurück im Browser. Nachdem Sie die fakechat-Benutzeroberfläche getestet haben, versuchen Sie [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram), [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) oder [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

Um die fakechat-Demo zu versuchen, benötigen Sie:

* Claude Code [installiert und authentifiziert](/docs/de/quickstart#step-1-install-claude-code) mit einem claude.ai-Konto oder einem Claude Console-API-Schlüssel
* [Bun](https://bun.sh) installiert. Die vorgefertigten Kanal-Plugins sind Bun-Skripte. Überprüfen Sie mit `bun --version`; wenn das fehlschlägt, [installieren Sie Bun](https://bun.sh/docs/installation).
* **Team, Enterprise oder verwaltete Console-Org**: Ihr Administrator muss [Kanäle aktivieren](#enterprise-controls) in verwalteten Einstellungen

<Steps>
  <Step title="Installieren Sie das fakechat-Kanal-Plugin">
    Starten Sie eine Claude Code-Sitzung und führen Sie den Installationsbefehl aus:

    ```text theme={null}
    /plugin install fakechat@claude-plugins-official
    ```

    Wenn Claude Code meldet, dass das Plugin in keinem Marketplace gefunden wird, fehlt Ihr Marketplace oder ist veraltet. Führen Sie `/plugin marketplace update claude-plugins-official` aus, um ihn zu aktualisieren, oder `/plugin marketplace add anthropics/claude-plugins-official`, wenn Sie ihn noch nicht hinzugefügt haben. Versuchen Sie dann die Installation erneut.
  </Step>

  <Step title="Starten Sie mit dem aktivierten Kanal neu">
    Beenden Sie Claude Code und starten Sie dann mit `--channels` neu und übergeben Sie das fakechat-Plugin, das Sie installiert haben:

    ```bash theme={null}
    claude --channels plugin:fakechat@claude-plugins-official
    ```

    Der fakechat-Server startet automatisch.

    <Tip>
      Sie können mehrere Plugins an `--channels` übergeben, durch Leerzeichen getrennt.
    </Tip>
  </Step>

  <Step title="Übertragen Sie eine Nachricht">
    Öffnen Sie die fakechat-Benutzeroberfläche unter [http://localhost:8787](http://localhost:8787) und geben Sie eine Nachricht ein:

    ```text theme={null}
    hey, what's in my working directory?
    ```

    Die Nachricht kommt in Ihrer Claude Code-Sitzung als `<channel source="fakechat">` Ereignis an. Claude liest es, macht die Arbeit und ruft das `reply`-Tool von fakechat auf. Die Antwort erscheint in der Chat-Benutzeroberfläche.
  </Step>
</Steps>

Wenn Claude auf eine Berechtigungsaufforderung trifft, während Sie weg vom Terminal sind, pausiert die Sitzung, bis Sie antworten. Kanal-Server, die die [Berechtigungsweiterleitungsfähigkeit](/docs/de/channels-reference#relay-permission-prompts) deklarieren, können diese Aufforderungen an Sie weiterleiten, damit Sie remote genehmigen oder ablehnen können. Für unbeaufsichtigte Nutzung umgeht [`--dangerously-skip-permissions`](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) die meisten Aufforderungen, aber verwenden Sie es nur in Umgebungen, denen Sie vertrauen. Explizite Frageregel, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden weiterhin angefordert.

Wenn Sie Kanäle im nicht-interaktiven Modus mit `-p` ausführen, sind Tools, die Terminaleingaben benötigen, wie Multiple-Choice-Fragen und Plan Mode-Genehmigung, deaktiviert, damit die Sitzung nie auf Eingaben wartet.

<h2 id="security">
  Sicherheit
</h2>

Jedes genehmigte Kanal-Plugin verwaltet eine Sender-Allowlist: Nur IDs, die Sie hinzugefügt haben, können Nachrichten übertragen, und alle anderen werden stillschweigend gelöscht.

Telegram und Discord starten die Liste durch Kopplung:

1. Finden Sie Ihren Bot in Telegram oder Discord und senden Sie ihm eine beliebige Nachricht
2. Der Bot antwortet mit einem Kopplungscode
3. In Ihrer Claude Code-Sitzung genehmigen Sie den Code, wenn Sie dazu aufgefordert werden
4. Ihre Sender-ID wird zur Allowlist hinzugefügt

iMessage funktioniert anders: Sich selbst zu schreiben umgeht das Gate automatisch, und Sie fügen andere Kontakte mit `/imessage:access allow` nach Handle hinzu.

Darüber hinaus kontrollieren Sie, welche Server in jeder Sitzung mit `--channels` aktiviert sind, und Ihre Organisation kontrolliert die Verfügbarkeit mit [`channelsEnabled`](#enterprise-controls) auf claude.ai Team- und Enterprise-Plänen und auf Console-Organisationen, die verwaltete Einstellungen bereitstellen.

In `.mcp.json` zu sein reicht nicht aus, um Nachrichten zu übertragen: Ein Server muss auch in `--channels` benannt werden.

Die Allowlist kontrolliert auch [Berechtigungsweiterleitungen](/docs/de/channels-reference#relay-permission-prompts), wenn der Kanal sie deklariert. Jeder, der über den Kanal antworten kann, kann die Tool-Nutzung in Ihrer Sitzung genehmigen oder ablehnen, daher sollten Sie nur Sender zur Allowlist hinzufügen, denen Sie diese Autorität vertrauen.

<h2 id="enterprise-controls">
  Enterprise-Steuerelemente
</h2>

Administratoren kontrollieren die Verfügbarkeit durch zwei [verwaltete Einstellungen](/docs/de/settings), die Benutzer nicht überschreiben können. Der Standard hängt davon ab, wie Sie sich authentifizieren:

* **claude.ai Team und Enterprise**: Kanäle sind blockiert, bis ein Administrator sie aktiviert.
* **Anthropic Console mit API-Schlüssel-Authentifizierung**: Kanäle sind standardmäßig zulässig. Sie benötigen diese Einstellung nur, wenn Ihre Organisation verwaltete Einstellungen bereitstellt.

In allen Fällen wird kein Kanal ausgeführt, bis ein Benutzer ihn für die Sitzung mit `--channels` aktiviert.

| Einstellung             | Zweck                                                                                                                                                                                                                                                                                                     | Wenn nicht konfiguriert                                                                                                                                                                                                  |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `channelsEnabled`       | Hauptschalter. Muss `true` sein, damit ein Kanal Nachrichten liefert. Legen Sie über den [claude.ai Admin-Konsole](https://claude.ai/admin-settings/claude-code) Umschalter oder direkt in verwalteten Einstellungen fest. Blockiert alle Kanäle einschließlich des Entwicklungs-Flags, wenn deaktiviert. | claude.ai Team und Enterprise: Kanäle blockiert. Console: Kanäle zulässig, es sei denn, Ihre Organisation stellt verwaltete Einstellungen bereit. In diesem Fall sind Kanäle blockiert, bis dieser Schlüssel gesetzt ist |
| `allowedChannelPlugins` | Welche Plugins sich registrieren können, sobald Kanäle aktiviert sind. Ersetzt die von Anthropic verwaltete Liste, wenn gesetzt. Gilt nur, wenn `channelsEnabled` `true` ist.                                                                                                                             | Anthropic-Standardliste gilt                                                                                                                                                                                             |

Pro- und Max-Benutzer ohne Organisation überspringen diese Überprüfungen vollständig: Kanäle sind verfügbar und Benutzer aktivieren sie pro Sitzung mit `--channels`.

<h3 id="enable-channels-for-your-organization">
  Aktivieren Sie Kanäle für Ihre Organisation
</h3>

Aktivieren Sie Kanäle für Ihre Organisation von [**claude.ai → Admin settings → Claude Code → Channels**](https://claude.ai/admin-settings/claude-code), was die Rolle des Administrators erfordert, oder indem Sie `channelsEnabled` in verwalteten Einstellungen auf `true` setzen.

Nach der Aktivierung können Benutzer in Ihrer Organisation `--channels` verwenden, um Kanal-Server in einzelne Sitzungen zu aktivieren. Wenn die Einstellung deaktiviert oder nicht gesetzt ist, verbindet sich der MCP-Server immer noch und seine Tools funktionieren, aber Kanal-Nachrichten kommen nicht an. Eine Startwarnmeldung teilt dem Benutzer mit, dass ein Administrator die Einstellung aktivieren muss.

<h3 id="restrict-which-channel-plugins-can-run">
  Beschränken Sie, welche Kanal-Plugins ausgeführt werden können
</h3>

Standardmäßig kann jedes Plugin auf der von Anthropic verwalteten Allowlist sich als Kanal registrieren. Administratoren auf Team- und Enterprise-Plänen können diese Allowlist durch Setzen von `allowedChannelPlugins` in verwalteten Einstellungen durch ihre eigene ersetzen. Verwenden Sie dies, um zu beschränken, welche offiziellen Plugins zulässig sind, Kanäle aus Ihrem eigenen internen Marketplace zu genehmigen oder beides. Jeder Eintrag benennt ein Plugin und den Marketplace, aus dem es stammt:

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

Wenn `allowedChannelPlugins` gesetzt ist, ersetzt es die Anthropic-Allowlist vollständig: Nur die aufgelisteten Plugins können sich registrieren. Lassen Sie es ungesetzt, um auf die Standard-Anthropic-Allowlist zurückzufallen. Ein leeres Array blockiert alle Kanal-Plugins aus der Allowlist, aber `--dangerously-load-development-channels` kann es immer noch für lokale Tests umgehen. Um Kanäle vollständig einschließlich des Entwicklungs-Flags zu blockieren, lassen Sie stattdessen `channelsEnabled` ungesetzt.

Diese Einstellung erfordert `channelsEnabled: true`. Wenn ein Benutzer ein Plugin an `--channels` übergibt, das nicht auf Ihrer Liste steht, startet Claude Code normal, aber der Kanal registriert sich nicht, und die Startnachricht erklärt, dass das Plugin nicht auf der genehmigten Liste Ihrer Organisation steht.

<h2 id="research-preview">
  Forschungsvorschau
</h2>

Kanäle sind eine Forschungsvorschau-Funktion. Die Verfügbarkeit wird schrittweise ausgerollt, und die `--channels`-Flag-Syntax und der Protokollvertrag können sich basierend auf Feedback ändern.

Während der Vorschau akzeptiert `--channels` nur Plugins von einer von Anthropic verwalteten Allowlist oder von der Allowlist Ihrer Organisation, wenn ein Administrator [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run) gesetzt hat. Die Kanal-Plugins in [claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) sind die Standard-genehmigte Menge. Wenn Sie etwas anderes übergeben, das nicht auf der geltenden Allowlist steht, startet Claude Code normal, aber der Kanal registriert sich nicht, und die Startnachricht teilt Ihnen mit, warum.

Um einen Kanal zu testen, den Sie erstellen, verwenden Sie `--dangerously-load-development-channels`. Siehe [Test während der Forschungsvorschau](/docs/de/channels-reference#test-during-the-research-preview) für Informationen zum Testen benutzerdefinierter Kanäle, die Sie erstellen.

Melden Sie Probleme oder Feedback im [Claude Code GitHub-Repository](https://github.com/anthropics/claude-code/issues).

<h2 id="how-channels-compare">
  Wie Kanäle sich vergleichen
</h2>

Mehrere Claude Code-Funktionen verbinden sich mit Systemen außerhalb des Terminals, jede für eine andere Art von Arbeit geeignet:

| Funktion                                         | Was sie tut                                                                       | Gut für                                                                            |
| ------------------------------------------------ | --------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| [Claude Code im Web](/docs/de/claude-code-on-the-web) | Führt Aufgaben in einer neuen Cloud-Sandbox aus, geklont von GitHub               | Delegieren von in sich geschlossener asynchroner Arbeit, die Sie später überprüfen |
| [Claude in Slack](/docs/de/slack)                     | Startet eine Web-Sitzung von einer `@Claude`-Erwähnung in einem Kanal oder Thread | Starten von Aufgaben direkt aus dem Kontext von Team-Gesprächen                    |
| Standard-[MCP-Server](/docs/de/mcp)                   | Claude fragt ihn während einer Aufgabe ab; nichts wird in die Sitzung übertragen  | Claude auf Abruf Zugriff zum Lesen oder Abfragen eines Systems geben               |
| [Remote Control](/docs/de/remote-control)             | Sie steuern Ihre lokale Sitzung von claude.ai oder der Claude Mobile App          | Steuern einer laufenden Sitzung, während Sie weg von Ihrem Schreibtisch sind       |

Kanäle füllen die Lücke in dieser Liste, indem sie Ereignisse von Nicht-Claude-Quellen in Ihre bereits laufende lokale Sitzung übertragen.

* **Chat-Brücke**: Fragen Sie Claude etwas von Ihrem Telefon über Telegram, Discord oder iMessage, und die Antwort kommt im selben Chat zurück, während die Arbeit auf Ihrem Computer gegen Ihre echten Dateien läuft.
* **[Webhook-Empfänger](/docs/de/channels-reference#example-build-a-webhook-receiver)**: Ein Webhook von CI, Ihrem Error Tracker, einer Deploy-Pipeline oder einem anderen externen Service kommt dort an, wo Claude bereits Ihre Dateien offen hat und sich erinnert, was Sie debuggt haben.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Sobald Sie einen Kanal ausgeführt haben, erkunden Sie diese verwandten Funktionen:

* [Erstellen Sie Ihren eigenen Kanal](/docs/de/channels-reference) für Systeme, die noch keine Plugins haben
* [Remote Control](/docs/de/remote-control), um eine lokale Sitzung von Ihrem Telefon aus zu steuern, anstatt Ereignisse darin zu übertragen
* [Geplante Aufgaben](/docs/de/scheduled-tasks), um auf einem Timer abzurufen, anstatt auf übertragene Ereignisse zu reagieren
