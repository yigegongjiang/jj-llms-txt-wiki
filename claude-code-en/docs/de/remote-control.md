> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lokale Sitzungen von jedem Gerät aus mit Remote Control fortsetzen

> Setzen Sie eine lokale Claude Code-Sitzung von Ihrem Telefon, Tablet oder einem beliebigen Browser aus mit Remote Control fort. Funktioniert mit claude.ai/code und der Claude-Mobile-App.

<Note>
  Remote Control ist in der Forschungsvorschau verfügbar und auf allen Plänen verfügbar. Bei Team und Enterprise ist es standardmäßig deaktiviert, bis ein Inhaber den Remote Control-Schalter in den [Claude Code-Admin-Einstellungen](https://claude.ai/admin-settings/claude-code) aktiviert.
</Note>

Remote Control verbindet [claude.ai/code](https://claude.ai/code) oder die Claude-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) und [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) mit einer Claude Code-Sitzung, die auf Ihrem Computer ausgeführt wird. Starten Sie eine Aufgabe an Ihrem Schreibtisch und setzen Sie sie dann von Ihrem Telefon auf der Couch oder einem Browser auf einem anderen Computer fort.

Wenn Sie eine Remote Control-Sitzung auf Ihrem Computer starten, wird Claude die ganze Zeit lokal ausgeführt, sodass Ihre Code-Ausführung und Ihr Dateisystem-Zugriff auf Ihrem Computer bleiben. Mit Remote Control können Sie:

* **Ihre vollständige lokale Umgebung remote nutzen**: Ihr Dateisystem, [MCP servers](/docs/de/mcp), Tools und Projektkonfiguration bleiben verfügbar, und durch Eingabe von `@` werden Dateipfade aus Ihrem lokalen Projekt automatisch vervollständigt
* **Von beiden Oberflächen gleichzeitig arbeiten**: Das Gespräch und der Fortschritt von [Subagenten](/docs/de/sub-agents) und [dynamischen Workflows](/docs/de/workflows) bleiben auf allen verbundenen Geräten synchronisiert, sodass Sie Nachrichten von Ihrem Terminal, Browser und Telefon austauschbar senden können. Vor v2.1.207 sendeten Sitzungen, die von der [Desktop-App](/docs/de/desktop) gehostet wurden, keinen Fortschritt von Subagenten oder Workflows an verbundene Geräte.
* **Bilder und Dateien von Ihrem Telefon oder Browser senden**: Wenn Sie einen Anhang in der Claude-App oder auf claude.ai/code hinzufügen, lädt Claude Code ihn auf Ihren Computer herunter und übergibt ihn Claude als `@`-Dateireferenz mit oder ohne Beschriftung. Vor v2.1.202 konnte Claude Code einen Anhang, der ohne Beschriftung gesendet wurde, vor Erreichen der Sitzung verwerfen.
* **Unterbrechungen überstehen**: Wenn Ihr Laptop in den Ruhezustand wechselt oder Ihr Netzwerk ausfällt, wird die Sitzung automatisch wiederhergestellt, wenn Ihr Computer wieder online ist. Claude Code stellt Status-Updates von Subagenten und Workflows in die Warteschlange, während die Verbindung wiederhergestellt wird, und liefert sie, sobald sie wiederhergestellt ist. Vor v2.1.207 konnte ein Update, das während einer Wiederverbindung oder Anmeldedaten-Aktualisierung gesendet wurde, verloren gehen, sodass das verbundene Gerät eine abgeschlossene Aufgabe weiterhin als laufend anzeigte.

Im Gegensatz zu [Claude Code im Web](/docs/de/claude-code-on-the-web), das auf Cloud-Infrastruktur ausgeführt wird, werden Remote Control-Sitzungen direkt auf Ihrem Computer ausgeführt und interagieren mit Ihrem lokalen Dateisystem. Die Web- und Mobile-Schnittstellen sind nur ein Fenster in diese lokale Sitzung.

Diese Seite behandelt die Einrichtung, das Starten und Verbinden mit Sitzungen sowie den Vergleich von Remote Control mit Claude Code im Web.

<h2 id="requirements">
  Anforderungen
</h2>

Bevor Sie Remote Control verwenden, bestätigen Sie, dass Ihre Umgebung diese Bedingungen erfüllt:

* **Abonnement**: verfügbar in Pro-, Max-, Team- und Enterprise-Plänen. API-Schlüssel werden nicht unterstützt. Bei Team und Enterprise muss ein Owner zunächst den Remote Control-Schalter in den [Claude Code-Admin-Einstellungen](https://claude.ai/admin-settings/claude-code) aktivieren.
* **Authentifizierung**: Führen Sie `claude` aus und verwenden Sie `/login`, um sich über claude.ai anzumelden, falls Sie dies noch nicht getan haben.
* **API-Endpunkt**: nicht verfügbar auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry. Ab v2.1.196 ist Remote Control auch deaktiviert, wenn [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf einen anderen Host als `api.anthropic.com` verweist, z. B. eine [LLM-Gateway](/docs/de/llm-gateway) oder einen Proxy. Heben Sie die Variablenzuweisung auf, um Remote Control zu verwenden.
* **Workspace-Vertrauen**: Führen Sie `claude` mindestens einmal in Ihrem Projektverzeichnis aus, um den Workspace-Vertrauensdialog zu akzeptieren.

<h2 id="start-a-remote-control-session">
  Starten Sie eine Remote Control-Sitzung
</h2>

Sie können eine Remote Control-Sitzung über die CLI oder die VS Code-Erweiterung starten. Die CLI bietet drei Aufrufmodi; VS Code verwendet den Befehl `/remote-control`.

<Tabs>
  <Tab title="Server-Modus">
    Navigieren Sie zu Ihrem Projektverzeichnis und führen Sie aus:

    ```bash theme={null}
    claude remote-control
    ```

    Der Prozess läuft weiterhin in Ihrem Terminal im Server-Modus und wartet auf Remote-Verbindungen. Er zeigt eine Sitzungs-URL an, die Sie zum [Verbinden von einem anderen Gerät](#connect-from-another-device) verwenden können, und Sie können die Leertaste drücken, um einen QR-Code für schnellen Zugriff von Ihrem Telefon anzuzeigen. Während eine Remote-Sitzung aktiv ist, zeigt das Terminal den Verbindungsstatus und die Tool-Aktivität an.

    Verfügbare Flags:

    | Flag                                            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
    | ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Legen Sie einen benutzerdefinierten Sitzungstitel fest, der in der Sitzungsliste unter claude.ai/code sichtbar ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
    | `--remote-control-session-name-prefix <prefix>` | Präfix für automatisch generierte Sitzungsnamen, wenn kein expliziter Name festgelegt ist. Standardmäßig der Hostname Ihres Computers, was Namen wie `myhost-graceful-unicorn` erzeugt. Setzen Sie `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` für denselben Effekt.                                                                                                                                                                                                                                                                                                       |
    | `-c`, `--continue`                              | Setzen Sie die zuletzt gestartete Remote Control-Sitzung aus diesem Verzeichnis fort, anstatt eine neue zu erstellen. Kann nicht mit `--session-id`, `--spawn`, `--capacity` oder `--create-session-in-dir` kombiniert werden. Erfordert Claude Code v2.1.200 oder später; frühere Versionen lehnen das Flag als unbekanntes Argument ab.                                                                                                                                                                                                                                  |
    | `--session-id <id>`                             | Setzen Sie eine bestimmte Remote Control-Sitzung anhand ihrer ID fort. Kann nicht mit `--continue`, `--spawn`, `--capacity` oder `--create-session-in-dir` kombiniert werden. Erfordert Claude Code v2.1.200 oder später; frühere Versionen lehnen das Flag als unbekanntes Argument ab.                                                                                                                                                                                                                                                                                   |
    | `--spawn <mode>`                                | Wie der Server Sitzungen erstellt.<br />• `same-dir` (Standard): Alle Sitzungen teilen sich das aktuelle Arbeitsverzeichnis, sodass sie in Konflikt geraten können, wenn dieselben Dateien bearbeitet werden.<br />• `worktree`: Jede On-Demand-Sitzung erhält ihren eigenen [git worktree](/docs/de/worktrees). Erfordert ein Git-Repository.<br />• `session`: Single-Session-Modus. Bedient genau eine Sitzung und lehnt zusätzliche Verbindungen ab. Wird nur beim Start festgelegt.<br />Drücken Sie `w` zur Laufzeit, um zwischen `same-dir` und `worktree` umzuschalten. |
    | `--capacity <N>`                                | Maximale Anzahl gleichzeitiger Sitzungen. Standard ist 32. Kann nicht mit `--spawn=session` verwendet werden.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
    | `--[no-]create-session-in-dir`                  | Erstellen Sie vorab eine Sitzung im aktuellen Verzeichnis, wenn der Server startet, damit Sie sofort einen Ort zum Eingeben haben. Im `worktree`-Modus bleibt diese Sitzung im aktuellen Verzeichnis, während On-Demand-Sitzungen isolierte Worktrees erhalten. Standardmäßig aktiviert; übergeben Sie `--no-create-session-in-dir`, um ohne zu starten.                                                                                                                                                                                                                   |
    | `--verbose`                                     | Zeigen Sie detaillierte Verbindungs- und Sitzungsprotokolle an.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    | `--sandbox` / `--no-sandbox`                    | Aktivieren oder deaktivieren Sie [sandboxing](/docs/de/sandboxing) für Dateisystem- und Netzwerkisolation. Standardmäßig deaktiviert.                                                                                                                                                                                                                                                                                                                                                                                                                                           |
  </Tab>

  <Tab title="Interaktive Sitzung">
    Um eine normale interaktive Claude Code-Sitzung mit aktiviertem Remote Control zu starten, verwenden Sie das Flag `--remote-control` (oder `--rc`):

    ```bash theme={null}
    claude --remote-control
    ```

    Geben Sie optional einen Namen für die Sitzung an:

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    Dies gibt Ihnen eine vollständige interaktive Sitzung in Ihrem Terminal, die Sie auch von claude.ai oder der Claude-App aus steuern können. Im Gegensatz zu `claude remote-control` (Server-Modus) können Sie lokal Nachrichten eingeben, während die Sitzung auch remote verfügbar ist.
  </Tab>

  <Tab title="Aus einer bestehenden Sitzung">
    Wenn Sie sich bereits in einer Claude Code-Sitzung befinden und diese remote fortsetzen möchten, verwenden Sie den Befehl `/remote-control` (oder `/rc`):

    ```text theme={null}
    /remote-control
    ```

    Übergeben Sie einen Namen als Argument, um einen benutzerdefinierten Sitzungstitel festzulegen:

    ```text theme={null}
    /remote-control My Project
    ```

    Dies startet eine Remote Control-Sitzung, die Ihren aktuellen Gesprächsverlauf überträgt.

    Die Flags `--verbose`, `--sandbox` und `--no-sandbox` sind mit diesem Befehl nicht verfügbar.
  </Tab>

  <Tab title="VS Code">
    In der [Claude Code VS Code-Erweiterung](/docs/de/vs-code) geben Sie `/remote-control` oder `/rc` in das Eingabefeld ein, oder öffnen Sie das Befehlsmenü mit `/` und wählen Sie es aus.

    ```text theme={null}
    /remote-control
    ```

    Ein Banner wird über dem Eingabefeld angezeigt und zeigt den Verbindungsstatus. Nach der Verbindung klicken Sie auf **Im Browser öffnen** im Banner, um direkt zur Sitzung zu gehen, oder finden Sie sie in der Sitzungsliste unter [claude.ai/code](https://claude.ai/code). Die Sitzungs-URL wird auch im Gespräch gepostet.

    Um die Verbindung zu trennen, klicken Sie auf das Schließsymbol im Banner oder führen Sie `/remote-control` erneut aus.

    Im Gegensatz zur CLI akzeptiert der VS Code-Befehl kein Namensargument und zeigt keinen QR-Code an. Der Sitzungstitel wird aus Ihrem Gesprächsverlauf oder der ersten Eingabeaufforderung abgeleitet.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verbindungsstatus überprüfen
</h3>

In einer interaktiven Terminal-Sitzung sitzt ein `/rc active`-Indikator in der Fußzeile unter dem Eingabefeld, während die Verbindung besteht, und wird ausgeblendet, wenn das Terminal zu schmal ist, um ihn anzuzeigen. Der Indikatortext ist ein Link zur Sitzung auf claude.ai. Wählen Sie ihn mit der Abwärts-Pfeiltaste aus und drücken Sie die Eingabetaste, oder führen Sie `/remote-control` erneut aus, um ein Statusfenster mit der Sitzungs-URL und einem QR-Code zu öffnen, den Sie zum [Verbinden von einem anderen Gerät](#connect-from-another-device) verwenden können.

Wenn die Verbindung fehlschlägt, wird eine Benachrichtigung mit dem Fehlergrund angezeigt und der Indikator verschwindet aus der Fußzeile. Führen Sie `/remote-control` erneut aus, um es erneut zu versuchen.

<h3 id="connect-from-another-device">
  Verbinden Sie sich von einem anderen Gerät
</h3>

Sobald eine Remote Control-Sitzung aktiv ist, haben Sie mehrere Möglichkeiten, sich von einem anderen Gerät aus zu verbinden:

* **Öffnen Sie die Sitzungs-URL** in einem beliebigen Browser, um direkt zur Sitzung auf [claude.ai/code](https://claude.ai/code) zu gehen.
* **Scannen Sie den QR-Code**, der neben der Sitzungs-URL angezeigt wird, um ihn direkt in der Claude-App zu öffnen. Mit `claude remote-control` drücken Sie die Leertaste, um die QR-Code-Anzeige umzuschalten.
* **Öffnen Sie [claude.ai/code](https://claude.ai/code) oder die Claude-App** und finden Sie die Sitzung nach Name in der Sitzungsliste. In der mobilen Claude-App tippen Sie auf **Code** in der Navigation, um die Sitzungsliste zu erreichen. Remote Control-Sitzungen zeigen ein Computersymbol mit einem grünen Statusindikator an, wenn sie online sind.

Wenn Sie sich verbinden, zeigt das Gerät alle Subagenten und Workflows an, die die Sitzung bereits im Hintergrund ausführt. Vor v2.1.208 zeigte ein Gerät, das sich mit einer in einem interaktiven Terminal gehosteten Sitzung verbindet, keine Subagenten und Workflows an, die bereits ausgeführt wurden, bis einer von ihnen gestartet oder gestoppt wurde.

Der Titel der Remote-Sitzung wird in dieser Reihenfolge gewählt:

1. Der Name, den Sie an `--name`, `--remote-control` oder `/remote-control` übergeben haben
2. Der Titel, den Sie mit `/rename` festgelegt haben
3. Die letzte aussagekräftige Nachricht im vorhandenen Gesprächsverlauf
4. Ein automatisch generierter Name wie `myhost-graceful-unicorn`, wobei `myhost` der Hostname Ihres Computers oder das Präfix ist, das Sie mit `--remote-control-session-name-prefix` festgelegt haben

Wenn Sie keinen expliziten Namen festgelegt haben, wird der Titel aktualisiert, um Ihre Eingabeaufforderung widerzuspiegeln, sobald Sie eine senden. Ab Claude Code v2.1.176 entsprechen automatisch generierte Titel der Sprache Ihres Gesprächs oder der [`language`](/docs/de/settings#available-settings)-Einstellung, falls eine konfiguriert ist. Das Umbenennen einer Sitzung von claude.ai oder der Claude-App aktualisiert auch den lokalen Titel, der in `claude --resume` angezeigt wird.

Wenn die Umgebung bereits eine aktive Sitzung hat, werden Sie gefragt, ob Sie diese fortsetzen oder eine neue starten möchten.

Wenn Sie die Claude-App noch nicht haben, verwenden Sie den Befehl `/mobile` in Claude Code, um einen Download-QR-Code für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) oder [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) anzuzeigen.

<h3 id="enable-remote-control-for-all-sessions">
  Aktivieren Sie Remote Control für alle Sitzungen
</h3>

Remote Control wird nur aktiviert, wenn Sie explizit `claude remote-control`, `claude --remote-control` oder `/remote-control` ausführen, es sei denn, die automatische Verbindung ist aktiviert. Um es automatisch für jede interaktive Sitzung zu aktivieren, führen Sie `/config` in Claude Code aus und setzen Sie **Enable Remote Control for all sessions** auf `true`. Setzen Sie es auf `false`, um es nie automatisch zu verbinden, oder lassen Sie es ungesetzt, um dem Standard Ihrer Organisation zu folgen. In der Desktop-App können Sie dies auch unter **Einstellungen → Claude Code → Enable remote control by default** umschalten. In der [VS Code-Erweiterung](/docs/de/vs-code#use-the-prompt-box) wird derselbe Schalter als **Enable Remote Control for all sessions** im Abschnitt „Einstellungen" des Befehlsmenüs angezeigt; erfordert Claude Code v2.1.203 oder später.

Mit dieser Einstellung registriert jeder interaktive Claude Code-Prozess eine Remote-Sitzung. Wenn Sie mehrere Instanzen ausführen, erhält jede ihre eigene Umgebung und Sitzung. Um mehrere gleichzeitige Sitzungen aus einem einzelnen Prozess auszuführen, verwenden Sie stattdessen den [Server-Modus](#start-a-remote-control-session).

<h2 id="connection-and-security">
  Verbindung und Sicherheit
</h2>

Ihre lokale Claude Code-Sitzung stellt nur ausgehende HTTPS-Anfragen und öffnet niemals eingehende Ports auf Ihrem Computer. Wenn Sie Remote Control starten, registriert es sich bei der Anthropic-API und fragt nach Arbeit ab. Wenn Sie sich von einem anderen Gerät aus verbinden, leitet der Server Nachrichten zwischen dem Web- oder Mobile-Client und Ihrer lokalen Sitzung über eine Streaming-Verbindung weiter.

Der gesamte Datenverkehr verläuft über die Anthropic-API über TLS, die gleiche Transportsicherheit wie jede Claude Code-Sitzung. Die Verbindung verwendet mehrere kurzlebige Anmeldeinformationen, die jeweils auf einen einzelnen Zweck beschränkt sind und unabhängig ablaufen.

Während Remote Control verbunden ist, werden das Sitzungstranskript, einschließlich Ihrer Nachrichten, Claudes Antworten und Werkzeugaktivität, auf Anthropic-Servern gespeichert. Das gespeicherte Transkript hält die Konversation auf Ihren Geräten synchron und ermöglicht es der Sitzung, sich nach einem Netzwerkausfall erneut zu verbinden. Ausführung und Dateisystemzugriff bleiben auf Ihrem Computer, und gespeicherte Transkripte werden gemäß der [Datenutzungsrichtlinie](/docs/de/data-usage) beibehalten.

Um Remote Control vollständig auszuschalten, verwenden Sie die Einstellung [`disableRemoteControl`](/docs/de/settings#available-settings). Organisationen mit Compliance-Anforderungen wie Zero Data Retention können Remote Control nicht aktivieren.

<h2 id="trusted-devices">
  Vertrauenswürdige Geräte
</h2>

<Note>
  Vertrauenswürdige Geräte befinden sich derzeit in der Beta-Phase. Funktionen und Möglichkeiten können sich weiterentwickeln, während die Erfahrung verfeinert wird.

  Vertrauenswürdige Geräte sind auf Team- und Enterprise-Plänen verfügbar. Die Funktion ist standardmäßig deaktiviert, bis ein Administrator sie aktiviert.
</Note>

Vertrauenswürdige Geräte ist eine organisationsweite Einstellung, die erfordert, dass Mitglieder ihr Gerät überprüfen, bevor sie Remote-Control-Sitzungen von claude.ai, den Claude-Mobile-Apps oder Claude Desktop anzeigen oder steuern können. Sie bindet den Remote-Control-Zugriff an ein bekanntes Gerät und eine aktuelle Authentifizierung, nicht nur an ein angemeldetes Konto.

Wenn die Einstellung aktiviert ist, erfordert die Interaktion mit einer Remote-Control-Sitzung beide der folgenden Voraussetzungen:

* **Ein registriertes Gerät**: Jeder Browser, jedes Telefon oder jede Desktop-App, die ein Mitglied für Remote Control verwendet, registriert seine eigenen Anmeldedaten. Die Registrierung wird nur kurz nach einer vollständigen Anmeldung angeboten, sodass ein Gerät als Teil einer echten Authentifizierung zur vertrauenswürdigen Liste hinzugefügt wird, anstatt stillschweigend im Hintergrund.
* **Eine aktuelle Anmeldung**: Die Anmeldung des Mitglieds darf nicht älter als 18 Stunden sein. Anstatt sich jeden Tag erneut anzumelden, bestätigen Mitglieder ihre Anwesenheit mit Face ID, Touch ID, Windows Hello oder einem Passkey. Dieser biometrische Schritt aktualisiert die Sitzung sofort.

Biometrische Überprüfungen werden auf dem Gerät über das Betriebssystem oder den Browser durchgeführt, denselben Mechanismus wie die Passkey-Anmeldung. Anthropic erhält oder speichert niemals Fingerabdrücke, Gesichtsdaten oder andere biometrische Informationen. Nur der öffentliche Schlüssel des Geräts und grundlegende Metadaten wie Anzeigename, Plattform und Registrierungszeit werden gespeichert.

Die Einstellung gilt nur für Remote Control. Regulärer Claude-Chat, Claude Code im Terminal und API-Nutzung sind nicht betroffen.

<h3 id="enable-trusted-devices-for-your-organization">
  Aktivieren Sie Vertrauenswürdige Geräte für Ihre Organisation
</h3>

Administratoren aktivieren die Einstellung über die Claude Code Admin-Konsole.

<Steps>
  <Step title="Öffnen Sie die Claude Code Admin-Einstellungen">
    Gehen Sie zu [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Der Umschalter **Vertrauenswürdige Geräte erforderlich** wird unter der Remote-Control-Einstellung angezeigt.
  </Step>

  <Step title="Aktivieren Sie Vertrauenswürdige Geräte erforderlich">
    Die Einstellung gilt für alle Mitglieder der Organisation und für Remote-Control-Sitzungen, die nach der Aktivierung gestartet werden. Sitzungen, die bereits vor dem Aktivieren des Umschalters ausgeführt wurden, sind nicht rückwirkend geschützt und werden ohne die Geräte-Anforderung fortgesetzt, bis sie beendet werden. Pro-Team- oder Pro-Projekt-Bereichsfestlegung ist nicht verfügbar.
  </Step>

  <Step title="Informieren Sie Mitglieder, was sie erwarten können">
    Wenn ein Mitglied zum ersten Mal eine neue Remote-Control-Sitzung von einem Browser, Telefon oder einer Desktop-App aus anzeigt oder steuert, nachdem die Einstellung aktiviert wurde, wird es aufgefordert, dieses Gerät zu registrieren. Wenn Sie sie vorher informieren, vermeiden Sie Verwirrung.
  </Step>
</Steps>

<h3 id="what-members-see">
  Was Mitglieder sehen
</h3>

Die Registrierung ist ein einmaliger Schritt pro Gerät. Danach ist die einzige sichtbare Änderung eine gelegentliche biometrische Aufforderung.

* **Erste Verwendung auf jedem Gerät**: Das Mitglied wird aufgefordert, sich zu registrieren. Wenn die Anmeldung nicht aktuell ist, meldet es sich zunächst über Ihren normalen Ablauf an, einschließlich SSO, falls konfiguriert, und bestätigt dann die Registrierung.
* **Täglich**: Mitglieder mit einem registrierten Gerät und einer aktuellen Anmeldung sehen keine Aufforderungen. Wenn die Anmeldung älter als 18 Stunden wird, zeigt die nächste Remote-Control-Interaktion eine einzelne Face ID-, Touch ID-, Windows Hello- oder Passkey-Aufforderung.
* **Nicht registrierte Geräte**: Remote-Control-Sitzungen können nicht angezeigt oder gesteuert werden, bis das Gerät registriert ist. Regulärer Claude-Chat auf diesem Gerät ist nicht betroffen.
* **Kein Plattform-Authentifizierer**: Mitglieder auf einem Computer ohne Face ID, Touch ID oder Windows Hello können einen Hardware-Sicherheitsschlüssel verwenden oder sich stattdessen erneut anmelden.
* **Im Terminal**: Der Computer, auf dem Claude Code ausgeführt wird, erhält automatisch seine eigenen Anmeldedaten, wenn sich der Entwickler bei der CLI anmeldet. Es gibt keinen separaten Registrierungsschritt im Terminal.

<h3 id="manage-enrolled-devices">
  Verwalten Sie registrierte Geräte
</h3>

Mitglieder können ihre eigenen Geräte in den Kontoeinstellungen überprüfen und widerrufen.

Öffnen Sie [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) und suchen Sie den Abschnitt **Vertrauenswürdige Geräte**, um alle registrierten Geräte mit ihrem Namen, ihrer Plattform und ihrem Registrierungsdatum anzuzeigen. Das Entfernen eines Geräts widerruft seine Anmeldedaten sofort, und das Gerät kann sich später nach einer neuen Anmeldung erneut registrieren. Anmeldedaten verfallen auch von selbst, wenn sie nicht erneuert werden, sodass ein ungenutztes Gerät automatisch von der vertrauenswürdigen Liste verschwindet.

Bei einem verlorenen oder gestohlenen Gerät entfernt das Mitglied es von dieser Seite. Wenn sich das Mitglied nicht anmelden kann, kann ein Administrator **Überall abmelden** in der Admin-Konsole verwenden, um alle Sitzungen und registrierten Geräte für dieses Mitglied zu widerrufen. Danach registriert sich das Mitglied die Geräte erneut, die es noch besitzt.

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control vs. Claude Code im Web
</h2>

Remote Control und [Claude Code im Web](/docs/de/claude-code-on-the-web) verwenden beide die Schnittstelle claude.ai/code. Der Hauptunterschied liegt darin, wo die Sitzung ausgeführt wird: Remote Control wird auf Ihrem Computer ausgeführt, sodass Ihre lokalen MCP servers, Tools und Projektkonfiguration verfügbar bleiben. Claude Code im Web wird in von Anthropic verwalteter Cloud-Infrastruktur ausgeführt.

Verwenden Sie Remote Control, wenn Sie sich mitten in lokaler Arbeit befinden und von einem anderen Gerät aus weitermachen möchten. Verwenden Sie Claude Code im Web, wenn Sie eine Aufgabe ohne lokale Einrichtung starten möchten, an einem Repository arbeiten, das Sie nicht geklont haben, oder mehrere Aufgaben parallel ausführen möchten.

<h2 id="mobile-push-notifications">
  Mobile Push-Benachrichtigungen
</h2>

Wenn Remote Control aktiv ist, kann Claude Push-Benachrichtigungen an Ihr Telefon senden.

Claude entscheidet, wann eine Push-Benachrichtigung gesendet wird. Sie wird normalerweise gesendet, wenn eine lange laufende Aufgabe abgeschlossen ist oder wenn Claude eine Entscheidung von Ihnen benötigt, um fortzufahren. Sie können auch eine Push-Benachrichtigung in Ihrer Eingabeaufforderung anfordern, zum Beispiel `notify me when the tests finish`. Über die beiden Ein-/Aus-Schalter unten gibt es keine Pro-Event-Konfiguration.

So richten Sie Mobile Push-Benachrichtigungen ein:

<Steps>
  <Step title="Installieren Sie die Claude-Mobile-App">
    Laden Sie die Claude-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) oder [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) herunter.
  </Step>

  <Step title="Melden Sie sich mit Ihrem Claude Code-Konto an">
    Verwenden Sie dasselbe Konto und die gleiche Organisation, die Sie für Claude Code im Terminal verwenden.
  </Step>

  <Step title="Benachrichtigungen zulassen">
    Akzeptieren Sie die Benachrichtigungsberechtigungsaufforderung des Betriebssystems.
  </Step>

  <Step title="Aktivieren Sie Push in Claude Code">
    Führen Sie in Ihrem Terminal `/config` aus und aktivieren Sie **Push when Claude decides** für proaktive Benachrichtigungen, **Push when actions required** für Berechtigungsaufforderungen und Fragen oder beides.
  </Step>
</Steps>

Wenn Benachrichtigungen nicht ankommen:

* Wenn `/config` **No mobile registered** anzeigt, öffnen Sie die Claude-App auf Ihrem Telefon, damit sie ihr Push-Token aktualisieren kann. Die Warnung wird beim nächsten Verbinden von Remote Control gelöscht.
* Auf iOS können Focus-Modi und Benachrichtigungszusammenfassungen Push-Benachrichtigungen unterdrücken oder verzögern. Überprüfen Sie Einstellungen → Benachrichtigungen → Claude.
* Auf Android kann aggressive Batterieoptimierung die Zustellung verzögern. Befreien Sie die Claude-App von der Batterieoptimierung in den Systemeinstellungen.

Claude Code überspringt Mobile Push-Benachrichtigungen, während Sie im verbundenen Terminal tippen oder sich darauf konzentrieren. Ab v2.1.181 können Sie [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/de/env-vars) auf einen Markierungsdateipfad setzen, um dies auf jede Zeit auszudehnen, in der Sie sich am Computer befinden, auch in einem anderen Fenster: Benachrichtigungen werden übersprungen, während die Datei vorhanden ist. Konfigurieren Sie einen Bildschirmsperr-Listener oder ein ähnliches Tool, um die Datei zu erstellen, wenn Ihr Bildschirm entsperrt wird, und löschen Sie sie, wenn Ihr Bildschirm gesperrt wird.

<h2 id="limitations">
  Einschränkungen
</h2>

* **Eine Remote-Sitzung pro interaktivem Prozess**: Außerhalb des Server-Modus unterstützt jede Claude Code-Instanz jeweils eine Remote-Sitzung. Verwenden Sie den [Server-Modus](#start-a-remote-control-session), um mehrere gleichzeitige Sitzungen aus einem einzelnen Prozess auszuführen.
* **Lokaler Prozess muss weiterhin ausgeführt werden**: Remote Control wird als lokaler Prozess ausgeführt. Wenn Sie das Terminal schließen, VS Code beenden oder den `claude`-Prozess anderweitig beenden, endet die Sitzung.
* **Längerer Netzwerkausfall**: Wenn Ihr Computer aktiv ist, aber länger als etwa 10 Minuten das Netzwerk nicht erreichen kann, läuft die Sitzung ab und der Prozess wird beendet. Führen Sie `claude remote-control` erneut aus, um eine neue Sitzung zu starten.
* **Ultraplan trennt Remote Control**: Das Starten einer [ultraplan](/docs/de/ultraplan)-Sitzung trennt jede aktive Remote Control-Sitzung, da beide Funktionen die Schnittstelle claude.ai/code belegen und nur eine gleichzeitig verbunden sein kann.
* **Einige Befehle sind nur lokal verfügbar**: Befehle, die nur in der Terminal-Schnittstelle ausgeführt werden, wie `/plugin` oder `/resume`, funktionieren nur über die lokale CLI, unabhängig davon, ob Sie ein Argument übergeben oder nicht. Die folgenden funktionieren von mobil und Web aus:
  * Textausgabe-Befehle: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits` (führt die Textform aus, anstatt den Dialog in der CLI zu öffnen), `/recap`, `/reload-plugins`
  * `/model`, `/effort`, `/fast`, `/color` und `/rename`: übergeben Sie den Wert als Argument, zum Beispiel `/model sonnet` oder `/effort high`. Von mobil und Web aus akzeptieren `/model` und `/effort` das Argument anstelle der Terminal-Auswahl oder des Schiebereglers.
  * `/mcp`, ab v2.1.166: gibt die mobile App eine Textzusammenfassung des Server-Status zurück, anstatt die Auswahl zu öffnen. Im Web öffnet `/mcp` allein ein Verzeichnis von [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai), anstatt die Zusammenfassung zurückzugeben. Die `reconnect`-, `enable`- und `disable`-[Unterbefehle](/docs/de/commands#all-commands) funktionieren von beiden aus. Im Gegensatz zur lokalen CLI verbindet `/mcp reconnect` ohne Servernamen jeden Server wieder, der fehlgeschlagen ist oder eine Authentifizierung benötigt.
  * `/config`, ab v2.1.181: übergeben Sie von der mobilen App aus `key=value`, um eine Einstellung festzulegen, oder führen Sie es ohne Argument aus, um die Schlüssel aufzulisten, die Sie festlegen können. Im Web öffnet `/config` stattdessen den Claude Code-Bereich Ihrer Einstellungen und ignoriert Text nach dem Befehl.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  'Remote Control erfordert ein claude.ai-Abonnement"
</h3>

Sie sind nicht mit einem claude.ai-Konto authentifiziert. Führen Sie `claude auth login` aus und wählen Sie die claude.ai-Option. Wenn `ANTHROPIC_API_KEY` in Ihrer Umgebung festgelegt ist, heben Sie die Festlegung zuerst auf.

Vor v2.1.206 meldete das Ausführen von `/remote-control` während der Abmeldung `Unknown command: /remote-control` statt dieser Meldung.

<h3 id="remote-control-requires-a-full-scope-login-token">
  „Remote Control erfordert ein Token mit vollständigem Umfang"
</h3>

Sie sind mit einem langlebigen Token von `claude setup-token` oder der Umgebungsvariable `CLAUDE_CODE_OAUTH_TOKEN` authentifiziert. Diese Token sind auf Inferenz-only beschränkt und können keine Remote Control-Sitzungen einrichten. Führen Sie `claude auth login` aus, um sich stattdessen mit einem Token mit vollständigem Umfang zu authentifizieren.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  „Ihre Organisation für die Remote Control-Berechtigung konnte nicht bestimmt werden"
</h3>

Ihre zwischengespeicherten Kontoinformationen sind veraltet oder unvollständig. Führen Sie `claude auth login` aus, um sie zu aktualisieren.

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  „Remote Control ist für Ihr Konto noch nicht aktiviert"
</h3>

Die Remote Control-Bereitstellung hat Ihr Konto noch nicht erreicht, oder Ihre zwischengespeicherten Berechtigungen sind veraltet. Wenn Sie kürzlich Ihren Plan geändert haben, führen Sie `claude auth logout` und dann `claude auth login` aus, um sie zu aktualisieren. Führen Sie `claude doctor` aus, um zu sehen, welche einzelne Berechtigungsprüfung fehlgeschlagen ist. Umgebungsvariablenkonflikte, unerreichbare Prüfungen und Organisationsrichtlinien erzeugen jeweils ihre eigene Meldung, daher bedeutet dieser Fehler das Rollout-Gate selbst.

<h3 id="couldn’t-verify-remote-control-eligibility">
  „Remote Control-Berechtigung konnte nicht überprüft werden"
</h3>

Claude Code konnte den Feature-Flag-Service nicht erreichen, um zu überprüfen, ob Remote Control für Ihr Konto aktiviert ist, normalerweise weil Sie offline sind oder ein Proxy die Anfrage blockiert. Versuchen Sie es erneut, wenn Sie Netzwerkzugriff haben, oder führen Sie `claude doctor` aus, um Details zu erhalten. Die zugehörige Meldung „Organisationsrichtlinie für Remote Control konnte nicht überprüft werden" hat die gleiche Ursache und die gleiche Lösung. Beide Meldungen wurden in v2.1.178 hinzugefügt.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  „Remote Control ist nur verfügbar, wenn Sie Claude über api.anthropic.com verwenden"
</h3>

Die Sitzung kommuniziert nicht direkt mit der Anthropic-API, daher gibt es kein claude.ai-Backend zum Koppeln. Dies geschieht auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry. Ab v2.1.196 geschieht dies auch, wenn [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf einen anderen Host als `api.anthropic.com` verweist, z. B. ein [LLM-Gateway](/docs/de/llm-gateway) oder Proxy, auch wenn Sie sich mit claude.ai anmelden. Heben Sie `ANTHROPIC_BASE_URL` auf und starten Sie die Sitzung neu, um Remote Control zu verwenden.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  „Remote Control ist durch die Richtlinie Ihrer Organisation deaktiviert"
</h3>

Dieser Fehler hat vier unterschiedliche Ursachen. Führen Sie zunächst `/status` aus, um zu sehen, welche Anmeldemethode und welches Abonnement Sie verwenden.

* **Sie sind mit einem API-Schlüssel oder Console-Konto authentifiziert**: Remote Control erfordert claude.ai OAuth. Führen Sie `/login` aus und wählen Sie die claude.ai-Option. Wenn `ANTHROPIC_API_KEY` in Ihrer Umgebung festgelegt ist, heben Sie die Festlegung auf.
* **Ein Inhaber hat es für Ihre Organisation nicht aktiviert**: Remote Control ist standardmäßig in Team- und Enterprise-Plänen deaktiviert. Ein Inhaber kann es unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) aktivieren, indem er den Schalter **Remote Control** einschaltet. Dieser Schalter ist eine serverseitige Organisationseinstellung.
* **Der Admin-Schalter ist ausgegraut**: Ihre Organisation hat eine Datenspeicherungs- oder Compliance-Konfiguration, die mit Remote Control nicht kompatibel ist. Dies kann nicht über das Admin-Panel geändert werden. Kontaktieren Sie den Anthropic-Support, um Optionen zu besprechen.
* **Der Fehler erwähnt `disableRemoteControl`**: Ihr IT-Administrator hat Remote Control auf diesem Gerät über [verwaltete Einstellungen](/docs/de/settings#settings-files) deaktiviert, unabhängig vom organisationsweiten Schalter.

<h3 id="remote-credentials-fetch-failed">
  „Remote credentials fetch failed"
</h3>

Claude Code konnte keine kurzlebige Anmeldeinformation von der Anthropic-API abrufen, um die Verbindung herzustellen. Führen Sie erneut mit `--verbose` aus, um den vollständigen Fehler zu sehen:

```bash theme={null}
claude remote-control --verbose
```

Häufige Ursachen:

* Nicht angemeldet: Führen Sie `claude` aus und verwenden Sie `/login`, um sich mit Ihrem claude.ai-Konto zu authentifizieren. API-Schlüssel-Authentifizierung wird für Remote Control nicht unterstützt.
* Netzwerk- oder Proxy-Problem: Eine Firewall oder ein Proxy blockiert möglicherweise die ausgehende HTTPS-Anfrage. Remote Control erfordert Zugriff auf die Anthropic-API auf Port 443.
* Sitzungserstellung fehlgeschlagen: Wenn Sie auch `Session creation failed — see debug log` sehen, ist der Fehler früher in der Einrichtung aufgetreten. Überprüfen Sie, dass Ihr Abonnement aktiv ist.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  „Konnte nicht erneut mit Ihrer Remote Control-Sitzung verbunden werden"
</h3>

Wenn Sie ein Gespräch mit `claude --resume` oder `claude --continue` fortsetzen, verbindet sich Claude Code erneut mit der Remote Control-Sitzung, die in diesem Gespräch aufgezeichnet wurde. Diese Meldung bedeutet, dass die Wiederverbindung aus einem Grund fehlgeschlagen ist, der möglicherweise vorübergehend ist, z. B. eine Netzwerkunterbrechung oder ein Serverfehler, daher kann Claude Code nicht bestätigen, ob die Remote-Sitzung noch vorhanden ist. Wenn der Server bestätigt, dass die vorherige Sitzung nicht mehr vorhanden ist, erstellt Claude Code eine neue Remote Control-Sitzung, ohne diese Meldung anzuzeigen.

Ihre lokale Sitzung läuft ohne Remote Control weiter. Führen Sie `/remote-control` aus, um die Verbindung erneut zu versuchen, oder starten Sie Claude Code ohne `--resume`, um eine neue Remote Control-Sitzung zu erstellen.

Vor v2.1.200 hat ein Wiederverbindungsfehler eine neue Remote Control-Sitzung erstellt, anstatt diese Meldung anzuzeigen, was zusätzliche Sitzungen in der Sitzungsliste unter claude.ai/code hinterlassen hat.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  „Ihre Organisation erfordert vertrauenswürdige Geräte für Remote Control, aber dieses Gerät ist nicht registriert"
</h3>

Ihre Organisation hat [Vertrauenswürdige Geräte](#trusted-devices) aktiviert und dieser Computer hat sich noch nicht registriert. Führen Sie `/login` in Claude Code aus. Die Registrierung erfolgt als Teil der Anmeldung, und es gibt keinen separaten Registrierungsbefehl.

<h3 id="session-expired-for-trusted-device-check">
  „session expired for trusted-device check"
</h3>

Ihre Anmeldung ist mehr als 18 Stunden alt. Führen Sie `/login` in Claude Code aus, oder bestätigen Sie mit Face ID, Touch ID, Windows Hello oder einem Passkey, wenn claude.ai oder die Mobile-App Sie auffordert. Siehe [Vertrauenswürdige Geräte](#trusted-devices).

<h2 id="choose-the-right-approach">
  Wählen Sie den richtigen Ansatz
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Claude Code im Web](/docs/de/claude-code-on-the-web): Führen Sie Sitzungen in von Anthropic verwalteten Cloud-Umgebungen aus, anstatt auf Ihrem Computer
* [Ultraplan](/docs/de/ultraplan): Starten Sie eine Cloud-Planungssitzung von Ihrem Terminal aus und überprüfen Sie den Plan in Ihrem Browser
* [Kanäle](/docs/de/channels): Leiten Sie Telegram, Discord oder iMessage in eine Sitzung weiter, damit Claude auf Nachrichten reagiert, während Sie weg sind
* [Dispatch](/docs/de/desktop#sessions-from-dispatch): Senden Sie eine Aufgabe von Ihrem Telefon aus, und sie kann eine Desktop-Sitzung spawnen, um sie zu bearbeiten
* [Authentifizierung](/docs/de/authentication): Richten Sie `/login` ein und verwalten Sie Anmeldeinformationen für claude.ai
* [CLI-Referenz](/docs/de/cli-reference): Vollständige Liste von Flags und Befehlen einschließlich `claude remote-control`
* [Sicherheit](/docs/de/security): Wie Remote Control-Sitzungen in das Claude Code-Sicherheitsmodell passen
* [Datennutzung](/docs/de/data-usage): Welche Daten während lokaler und Remote-Sitzungen durch die Anthropic-API fließen
