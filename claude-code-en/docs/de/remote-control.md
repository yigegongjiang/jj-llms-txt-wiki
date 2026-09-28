> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lokale Sitzungen von jedem Gerät aus mit Remote Control fortsetzen

> Setzen Sie eine lokale Claude Code-Sitzung von Ihrem Telefon, Tablet oder einem beliebigen Browser aus mit Remote Control fort. Funktioniert mit claude.ai/code und der Claude-Mobile-App.

<Note>
  Remote Control ist auf allen Plänen verfügbar. Bei Team und Enterprise ist es standardmäßig deaktiviert, bis ein Inhaber den Remote Control-Schalter in den [Claude Code-Admin-Einstellungen](https://claude.ai/admin-settings/claude-code) aktiviert.
</Note>

Remote Control verbindet [claude.ai/code](https://claude.ai/code) oder die Claude-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) und [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) mit einer Claude Code-Sitzung, die auf Ihrem Computer ausgeführt wird. Starten Sie eine Aufgabe an Ihrem Schreibtisch und setzen Sie sie dann von Ihrem Telefon auf der Couch oder einem Browser auf einem anderen Computer fort.

Wenn Sie eine Remote Control-Sitzung auf Ihrem Computer starten, wird Claude die ganze Zeit lokal ausgeführt, sodass Ihre Code-Ausführung und Ihr Dateisystem-Zugriff auf Ihrem Computer bleiben. Mit Remote Control können Sie:

* **Ihre vollständige lokale Umgebung remote nutzen**: Ihr Dateisystem, [MCP servers](/docs/de/mcp), Tools und Projektkonfiguration bleiben verfügbar, und durch Eingabe von `@` werden Dateipfade aus Ihrem lokalen Projekt automatisch vervollständigt.
* **Von beiden Oberflächen gleichzeitig arbeiten**: Das Gespräch und der Fortschritt von [Subagenten](/docs/de/sub-agents) und [dynamischen Workflows](/docs/de/workflows) bleiben auf allen verbundenen Geräten synchronisiert, sodass Sie Nachrichten von Ihrem Terminal, Browser und Telefon austauschbar senden können.
* **Bilder und Dateien von Ihrem Telefon oder Browser senden**: Fügen Sie ein Foto oder eine Datei in der Claude-App oder auf claude.ai/code an, mit oder ohne Beschriftung. Claude sieht angehängte Fotos direkt als Teil Ihrer Nachricht. Claude Code lädt andere Dateien auf Ihren Computer herunter und übergibt sie Claude als `@`-Dateireferenzen.
* **Unterbrechungen überstehen**: Wenn Ihr Laptop in den Ruhezustand wechselt oder Ihr Netzwerk ausfällt, wird Claude Code automatisch wiederhergestellt, wenn Ihr Computer wieder online ist. Während die Verbindung wiederhergestellt wird, stellt Claude Code Nachrichten, Berechtigungsaufforderungen und Status-Updates von Subagenten und Workflows in die Warteschlange und liefert sie, sobald die Verbindung wiederhergestellt ist.

Im Gegensatz zu [Claude Code im Web](/docs/de/claude-code-on-the-web), das auf Cloud-Infrastruktur ausgeführt wird, werden Remote Control-Sitzungen direkt auf Ihrem Computer ausgeführt und interagieren mit Ihrem lokalen Dateisystem. Die Web- und Mobile-Schnittstellen sind ein Fenster in diese lokale Sitzung.

Diese Seite behandelt die Einrichtung, das Starten und Verbinden mit Sitzungen sowie den Vergleich von Remote Control mit Claude Code im Web.

<h2 id="requirements">
  Anforderungen
</h2>

Bevor Sie Remote Control verwenden, bestätigen Sie, dass Ihre Umgebung diese Bedingungen erfüllt:

* **Abonnement**: verfügbar in Pro-, Max-, Team- und Enterprise-Plänen. API-Schlüssel werden nicht unterstützt. Bei Team und Enterprise muss ein Owner zunächst den Remote Control-Schalter in den [Claude Code-Admin-Einstellungen](https://claude.ai/admin-settings/claude-code) aktivieren.
* **Authentifizierung**: Führen Sie `claude` aus und verwenden Sie `/login`, um sich über claude.ai anzumelden, falls Sie dies noch nicht getan haben. Ohne eine berechtigte Anmeldung wird `claude remote-control` mit einem Fehler beendet, während `claude --remote-control` weiterhin eine interaktive Sitzung startet und kurz nach dem Start eine Remote Control-Fehlerbenachrichtigung anzeigt.
* **API-Endpunkt**: nicht verfügbar in einer dieser Konfigurationen:
  * Sie verwenden Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry.
  * Sie verweisen [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf einen anderen Host als `api.anthropic.com`, z. B. ein [LLM-Gateway](/docs/de/llm-gateway) oder einen Proxy. Heben Sie die Variablenzuweisung auf, um Remote Control zu verwenden. Vor v2.1.196 erlaubte Claude Code Remote Control mit einer benutzerdefinierten `ANTHROPIC_BASE_URL`.
  * Sie melden sich über ein Enterprise-[Claude-Apps-Gateway](/docs/de/claude-apps-gateway) an.
* **Feature-Flag-Evaluierung**: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` und `DISABLE_GROWTHBOOK`](/docs/de/env-vars) deaktivieren jeweils die Feature-Flag-Evaluierung, von der die Remote Control-Verfügbarkeit abhängt. Heben Sie die Variablenzuweisung überall auf, wo sie gesetzt ist, in Ihrer Shell-Umgebung oder im `env`-Block einer [`settings.json`-Datei](/docs/de/settings-reference#all-settings), um Remote Control zu verwenden.
* **Workspace-Vertrauen**: Führen Sie `claude` mindestens einmal in Ihrem Projektverzeichnis aus, um den Workspace-Vertrauensdialog zu akzeptieren. Der Startup-Vertrauensdialog speichert niemals Vertrauen für Ihr Home-Verzeichnis, daher starten Sie Remote Control aus einem Projektverzeichnis.

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

    Bis Sie Remote Controls einmalige Bestätigung akzeptieren, erklärt `claude remote-control`, was es tut, und fragt `Enable Remote Control? (y/n)`, bevor der Server gestartet wird. Antworten Sie mit `y`, um zu akzeptieren und den Server zu starten. Wenn Sie ablehnen, beendet Claude Code den Vorgang ohne Starten des Servers und fragt beim nächsten Ausführen des Befehls erneut.

    Der Prozess läuft weiterhin in Ihrem Terminal im Server-Modus und wartet auf Remote-Verbindungen. Er zeigt eine Sitzungs-URL an, die Sie zum [Verbinden von einem anderen Gerät](#connect-from-another-device) verwenden können, und Sie können die Leertaste drücken, um einen QR-Code für schnellen Zugriff von Ihrem Telefon anzuzeigen. Während eine Remote-Sitzung aktiv ist, zeigt das Terminal den Verbindungsstatus und die Tool-Aktivität an.

    Verfügbare Flags:

    | Flag                                            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
    | ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Legen Sie einen benutzerdefinierten Sitzungstitel fest, der in der Sitzungsliste unter claude.ai/code sichtbar ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
    | `--remote-control-session-name-prefix <prefix>` | Präfix für automatisch generierte Sitzungsnamen, wenn kein expliziter Name festgelegt ist. Standardmäßig der Hostname Ihres Computers, was Namen wie `myhost-graceful-unicorn` erzeugt. Setzen Sie `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` für denselben Effekt.                                                                                                                                                                                                                                                                                                       |
    | `-c`, `--continue`                              | Bringen Sie die Sitzung zurück, die der letzte Server in diesem Verzeichnis gestartet hat, anstatt eine neue zu erstellen. Siehe [Sitzungen nach dem Stoppen des Servers fortsetzen](#resume-sessions-after-stopping-the-server). Kann nicht mit `--session-id`, `--spawn`, `--capacity` oder `--create-session-in-dir` kombiniert werden. Erfordert Claude Code v2.1.200 oder später; frühere Versionen lehnen das Flag als unbekanntes Argument ab.                                                                                                                      |
    | `--session-id <id>`                             | Bringen Sie eine Sitzung anhand ihrer ID zurück. Siehe [Sitzungen nach dem Stoppen des Servers fortsetzen](#resume-sessions-after-stopping-the-server). Kann nicht mit `--continue`, `--spawn`, `--capacity` oder `--create-session-in-dir` kombiniert werden. Erfordert Claude Code v2.1.200 oder später; frühere Versionen lehnen das Flag als unbekanntes Argument ab.                                                                                                                                                                                                  |
    | `--spawn <mode>`                                | Wie der Server Sitzungen erstellt.<br />• `same-dir` (Standard): Alle Sitzungen teilen sich das aktuelle Arbeitsverzeichnis, sodass sie in Konflikt geraten können, wenn dieselben Dateien bearbeitet werden.<br />• `worktree`: Jede On-Demand-Sitzung erhält ihren eigenen [git worktree](/docs/de/worktrees). Erfordert ein Git-Repository.<br />• `session`: Single-Session-Modus. Bedient genau eine Sitzung und lehnt zusätzliche Verbindungen ab. Wird nur beim Start festgelegt.<br />Drücken Sie `w` zur Laufzeit, um zwischen `same-dir` und `worktree` umzuschalten. |
    | `--capacity <N>`                                | Maximale Anzahl gleichzeitiger Sitzungen. Standard ist 32. Kann nicht mit `--spawn=session` verwendet werden.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
    | `--[no-]create-session-in-dir`                  | Erstellen Sie vorab eine Sitzung im aktuellen Verzeichnis, wenn der Server startet, damit Sie sofort einen Ort zum Eingeben haben. Im `worktree`-Modus bleibt diese Sitzung im aktuellen Verzeichnis, während On-Demand-Sitzungen isolierte Worktrees erhalten. Standardmäßig aktiviert. Wenn Sie `--no-create-session-in-dir` übergeben, um ohne zu starten, archiviert Claude Code die Sitzungen des Servers, wenn Sie ihn stoppen, sodass es nichts zum [Fortsetzen](#resume-sessions-after-stopping-the-server) gibt.                                                  |
    | `--permission-mode <mode>`                      | Legen Sie den Startmodus für [Berechtigungen](/docs/de/permission-modes) für die Sitzungen des Servers fest, z. B. `acceptEdits`. Akzeptiert `manual` als Alias für `default`; ein nicht erkannter Modus stoppt den Server beim Start und listet die gültigen Modi auf.                                                                                                                                                                                                                                                                                                         |
    | `--debug-file <path>`                           | Schreiben Sie Debug-Protokolle in die angegebene Datei.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
    | `--verbose`                                     | Zeigen Sie detaillierte Verbindungs- und Sitzungsprotokolle an.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    | `--sandbox` / `--no-sandbox`                    | Aktivieren oder deaktivieren Sie [Sandboxing](/docs/de/sandboxing) für Dateisystem- und Netzwerkisolation. Standardmäßig deaktiviert.                                                                                                                                                                                                                                                                                                                                                                                                                                           |

    Geben Sie diese Flags nach `remote-control` an.

    Wenn Sie ein globales `claude`-Flag vor `remote-control` übergeben, oder ein Wrapper-Skript fügt eines hinzu, trägt Claude Code das Flag nicht auf die Sitzungen über, die der Server erstellt. Claude Code lässt das Flag nur durch, wenn bekannt ist, dass das Löschen nicht ändert, was diese Sitzungen tun können, z. B. `--verbose` oder `--model`. Für jedes andere Flag, z. B. `--settings`, [weigert sich Claude Code zu starten](/docs/de/errors#not-carried-over-to-the-sessions-remote-control-starts) und nennt das zu entfernende Flag. Vor v2.1.248 führte jede Option vor `remote-control` dazu, dass Claude Code die Flags danach mit einem `unknown option`-Fehler ablehnte.

    Claude Code überprüft die Remote Control-Berechtigung vor dem Drucken der Hilfe, daher gibt `claude remote-control --help` einen Fehler zurück, anstatt diese Flag-Liste anzuzeigen, wenn Sie nicht mit einem berechtigten Konto angemeldet sind.
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

    Bis Sie Remote Controls einmalige Bestätigung akzeptieren, wird ein Dialog angezeigt, bevor `/remote-control` verbunden wird. Wählen Sie **Enable Remote Control**, um zu akzeptieren und zu verbinden. Wenn Sie **Never mind** wählen oder Esc drücken, verbindet Claude Code nicht und fragt beim nächsten Ausführen von `/remote-control` erneut.

    Die Flags `--verbose`, `--sandbox` und `--no-sandbox` sind mit diesem Befehl nicht verfügbar.
  </Tab>

  <Tab title="VS Code">
    In der [Claude Code VS Code-Erweiterung](/docs/de/vs-code) geben Sie `/remote-control` oder `/rc` in das Eingabefeld ein.

    ```text theme={null}
    /remote-control
    ```

    Während Remote Control aktiviert ist, zeigt Claude Code einen **Remote Control**-Indikator in der Fußzeile des Eingabefelds an. Sobald sich die Sitzung verbindet, klicken Sie auf den Indikator, um direkt zur Sitzung zu gehen, oder finden Sie sie in der Sitzungsliste unter [claude.ai/code](https://claude.ai/code). Claude Code postet auch die Sitzungs-URL im Gespräch. Um die Verbindung zu trennen, führen Sie `/remote-control` erneut aus.

    Im Gegensatz zur CLI akzeptiert der VS Code-Befehl kein Namensargument und zeigt keinen QR-Code an. Der Sitzungstitel wird aus Ihrem Gesprächsverlauf oder der ersten Eingabeaufforderung abgeleitet.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verbindungsstatus überprüfen
</h3>

In einer interaktiven Sitzung zeigt das Terminal während der Remote Control-Verbindung einen `/rc active`-Indikator an, der auf die Sitzung auf claude.ai verlinkt. Der Indikator ist ausgeblendet, wenn das Terminal zu schmal ist, um ihn anzuzeigen. Um die Sitzungs-URL und einen QR-Code zum [Verbinden von einem anderen Gerät](#connect-from-another-device) anzuzeigen, führen Sie `/remote-control` erneut aus, um das Statusfenster zu öffnen. Das Fenster bietet auch eine Trennungsoption, die Remote Control ausschaltet, während Ihre lokale Sitzung weiterhin im Terminal läuft.

<span id="session-ended-elsewhere" />Wenn die Verbindung in einer interaktiven Sitzung fehlschlägt, ändert sich der Indikator, um den Fehler anzuzeigen, und Claude Code zeigt den Grund in einer Benachrichtigung an und fügt ihn zum Gespräch hinzu. Führen Sie `/remote-control` aus, um erneut zu verbinden, es sei denn, der Grund besagt, dass die Sitzung anderswo übernommen oder beendet wurde:

* **Another connection took over this session**: Ein anderes Gerät oder eine Claude Code-Sitzung hat sie jetzt. Führen Sie `/remote-control` nur aus, wenn Sie sie von diesem Gerät zurücknehmen möchten.
* **This session was ended or archived from another device or app**: Führen Sie `/remote-control` nur aus, wenn Sie sie zurück möchten. Claude Code öffnet eine archivierte Sitzung erneut.
* **The server no longer reports this session**: Sie wurde möglicherweise von einem anderen Gerät oder einer App gelöscht.

<h3 id="session-url-reminders">
  Sitzungs-URL-Erinnerungen
</h3>

Während Remote Control verbunden ist, erinnert Claude Code Sie an die Sitzungs-URL, wenn der Wechsel zu Ihrem Telefon oder Browser am hilfreichsten ist, damit Sie den Link nicht in `/remote-control` suchen müssen. Eine Erinnerung wird über dem Eingabefeld in einem dieser Momente angezeigt:

* **Lange Runde**: Wenn eine Runde länger als ein vom Server abgestimmter Schwellenwert läuft, zeigt Claude Code eine **Still working**-Benachrichtigung mit einem **Check in from your phone**-Link an, damit Sie die Runde von Ihrem Telefon oder Browser aus verfolgen können, anstatt am Terminal zu warten. Claude Code entfernt sie, wenn die Runde endet.
* **Wiederholte Berechtigungsaufforderungen**: Nachdem Sie mehrere [Berechtigungsaufforderungen](/docs/de/permissions) in einer Sitzung beantwortet haben, wird eine **Approve tool calls from your phone**-Benachrichtigung mit der Sitzungs-URL angezeigt. Claude Code entfernt sie, wenn Ihre nächste Runde beginnt.

Die Erinnerungen können in jeder verbundenen Sitzung angezeigt werden, einschließlich solcher, bei denen Remote Control [automatisch verbunden wird](#enable-remote-control-for-all-sessions). Sie werden nicht jedes Mal angezeigt, wenn diese Bedingungen auftreten, und jede wird insgesamt nur wenige Male über Sitzungen hinweg angezeigt. Sie können sie nicht konfigurieren oder ausschalten; jede wird von selbst gelöscht.

<h3 id="connect-from-another-device">
  Verbinden Sie sich von einem anderen Gerät
</h3>

Sobald eine Remote Control-Sitzung aktiv ist, haben Sie mehrere Möglichkeiten, sich von einem anderen Gerät aus zu verbinden:

* **Öffnen Sie die Sitzungs-URL** in einem beliebigen Browser, um direkt zur Sitzung auf [claude.ai/code](https://claude.ai/code) zu gehen.
* **Scannen Sie den QR-Code**, der neben der Sitzungs-URL angezeigt wird, um ihn direkt in der Claude-App zu öffnen. Mit `claude remote-control` drücken Sie die Leertaste, um die QR-Code-Anzeige umzuschalten.
* **Öffnen Sie [claude.ai/code](https://claude.ai/code) oder die Claude-App** und finden Sie die Sitzung nach Name in der Sitzungsliste. In der mobilen Claude-App tippen Sie auf **Code** in der Navigation, um die Sitzungsliste zu erreichen. Remote Control-Sitzungen zeigen ein Computersymbol mit einem grünen Statusindikator an, wenn sie online sind.

Wenn Sie sich verbinden, zeigt das Gerät alle Subagenten und Workflows an, die die Sitzung bereits im Hintergrund ausführt. Stoppen Sie einen von ihnen vom Gerät aus, und Claude Code stoppt diese Aufgabe auf Ihrem Computer.

Der Titel der Remote-Sitzung wird in dieser Reihenfolge gewählt:

1. Der Name, den Sie an `--name`, `--remote-control` oder `/remote-control` übergeben haben
2. Der Titel, den Sie mit `/rename` festgelegt haben
3. Die letzte aussagekräftige Nachricht im vorhandenen Gesprächsverlauf
4. Ein automatisch generierter Name wie `myhost-graceful-unicorn`, wobei `myhost` der Hostname Ihres Computers oder das Präfix ist, das Sie mit `--remote-control-session-name-prefix` festgelegt haben

Wenn Sie keinen expliziten Namen festgelegt haben, aktualisiert Claude Code den Titel, um Ihre Eingabeaufforderung widerzuspiegeln, sobald Sie eine senden. Claude Code stimmt automatisch generierte Titel mit der Sprache Ihres Gesprächs oder der [`language`](/docs/de/settings-reference#language)-Einstellung ab, falls eine konfiguriert ist.

Wenn Sie eine Sitzung von claude.ai oder der Claude-App aus umbenennen, aktualisiert Claude Code auch den lokalen Titel, der in `claude --resume` angezeigt wird. Claude Code wendet dieselbe Umbenennung auf den Sitzungsnamen an, der in der Eingabeaufforderungsleiste angezeigt wird, und in der `claude agents`-Auflistung, wenn die Sitzung [im Hintergrund läuft](/docs/de/agent-view). Vor v2.1.221 aktualisierte die Umbenennung von der Sitzungsliste unter claude.ai oder in der Claude-App nur den Titel, und die CLI behielt ihren vorherigen Sitzungsnamen; `/rename`, das in der CLI selbst läuft, setzt den Namen auf jeder Version.

Wenn Sie die Claude-App noch nicht haben, verwenden Sie den Befehl `/mobile` in Claude Code, um einen QR-Code für [claude.ai/mobile](https://claude.ai/mobile) anzuzeigen, das die richtige App-Store-Seite für Ihr Telefon öffnet.

<h3 id="what-connected-devices-see">
  Was verbundene Geräte sehen
</h3>

Ein verbundenes Gerät zeigt das Gespräch in Ihrem Terminal, während es passiert. Diese Fälle gehen über gewöhnliche Nachrichten hinaus:

* **Komprimierung und `/clear`**: Während Claude Code [das Gespräch komprimiert](/docs/de/context-window#what-survives-compaction), zeigen verbundene Geräte den Fortschritt und dann an, wo das Gespräch komprimiert wurde. Wenn Sie `/clear` ausführen, wird das Gespräch auch auf verbundenen Geräten zurückgesetzt.
* **Wechsel von Gesprächen mit `/resume`**: Das verbundene Gerät empfängt nicht den Titel oder die frühere Verlauf des gewechselten Gesprächs, aber neue Nachrichten in beide Richtungen gehen zu und von dem Gespräch, das in Ihrem Terminal offen ist. Um von dem Gerät aus wieder an dem ursprünglichen Gespräch zu arbeiten, führen Sie `/resume` in Ihrem Terminal aus und wechseln Sie zurück.
* **Ziehen Sie eine Sitzung mit `/teleport`**: Wenn Sie eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web#from-cloud-to-terminal) mit `/teleport` in Ihr Terminal ziehen, empfängt das verbundene Gerät nicht den früheren Verlauf des gezogenen Gesprächs. Neue Nachrichten in beide Richtungen gehen zu und von dem gezogenen Gespräch, das jetzt das in Ihrem Terminal offene ist.
* **Nachrichten von Ihren anderen Sitzungen**: Mit [sitzungsübergreifendem Messaging](/docs/de/cross-session-messaging) trägt dieselbe Verbindung Nachrichten zwischen Ihren eigenen Sitzungen auf verschiedenen Maschinen und von Ihren [Cloud-Sitzungen](/docs/de/claude-code-on-the-web), über Anthropic-Server wie der Rest des Remote Control-Verkehrs. [Nachrichtensitzungen auf anderen Maschinen](/docs/de/cross-session-messaging#message-sessions-on-other-machines) behandelt die Lieferregeln und [Kontrollieren Sie eingehende Nachrichten](/docs/de/cross-session-messaging#control-inbound-messages) behandelt die eingehenden Kontrollen. Erfordert Claude Code v2.1.224 oder später.
* **Eingabeaufforderungen, die Sie während einer Runde senden**: Wenn Sie eine Eingabeaufforderung von einem verbundenen Gerät senden, bevor die aktuelle Runde endet, stellt Claude Code sie in die Warteschlange und behält sie im Transkript des Geräts, nachdem diese Runde endet.
* **Diff Ihrer Änderungen**: Wenn sich das Verzeichnis der Sitzung in einem Git-Repository befindet, zeigt der Diff-Bereich eines verbundenen Geräts Ihre Änderungen. Das Gerät fordert den Diff über die Verbindung an, und Claude Code berechnet ihn auf Ihrem Computer. Auf einem Branch, der Commits vor dem Standard-Branch des Repositories hat, zeigt der Bereich die Änderungen seit dem Abzweigen des Branches an, einschließlich Ihrer nicht committeten Änderungen. Auf dem Standard-Branch selbst oder auf einem Branch, der nicht vor ihm liegt, zeigt der Bereich nur Ihre nicht committeten Änderungen. Vor v2.1.247 meldete Claude Code den Diff an verbundene Geräte nur in Sitzungen, die von `claude remote-control` bedient wurden.
* **Modell**: Wenn Sie ein [Modell](/docs/de/model-config) von einem verbundenen Gerät aus auswählen, führt Claude Code die Sitzung auf diesem Modell aus. Die Auswahl `/model` des Terminals, `/status` und `/config` zeigen dieses Modell. Erfordert Claude Code v2.1.238 oder später auf Ihrem Computer.
  * Ein Modell, das Sie vom Modellsteuerelement des Geräts auswählen, gilt nur für die aktuelle Sitzung. Wenn Sie `/model <name>` vom Gerät an eine interaktive Sitzung senden, setzt Claude Code auch Ihren Standard für neue Sitzungen.
  * Wenn Sie einen Namen senden, den Claude Code nicht erkennt, z. B. einen Anzeigenamen, wenn eine Modell-ID erwartet wird, [weigert sich Claude Code, die Auswahl zu akzeptieren](/docs/de/errors#model-is-not-a-recognized-model-id) und die Sitzung behält ihr aktuelles Modell. Vor v2.1.260 speicherte Claude Code eine nicht erkannte Auswahl vom Modellsteuerelement des Geräts, und Ihre nächste Nachricht schlug fehl.
* **Anstrengungsstufe**: Wenn Sie die [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) von einem verbundenen Gerät aus mit `/effort` oder dem Anstrengungssteuerelement des Geräts einstellen, wendet Claude Code sie auf die Sitzung auf Ihrem Computer an, und claude.ai/code zeigt die Stufe an, die die Sitzung verwendet. Wenn Sie eine Stufe mit `CLAUDE_CODE_EFFORT_LEVEL` angeheftet haben, behält die Sitzung diese Stufe, und Claude Code weigert sich, eine andere Auswahl vom Anstrengungssteuerelement zu treffen. Eine Stufe vom Anstrengungssteuerelement auszuwählen erfordert Claude Code v2.1.234 oder später auf Ihrem Computer.
* **Erneute Verbindung nach einem Verbindungsfehler**: Führen Sie `/remote-control` aus, um erneut zu verbinden. Wenn die Komprimierung das Gespräch umgeschrieben hat oder Sie Gespräche mit `/resume` in der Zwischenzeit gewechselt haben, archiviert Claude Code die Server-Sitzung, die es verwendete, anstatt sie in der Sitzungsliste zu belassen. Sie können sie immer noch finden, indem Sie [nach archivierten Sitzungen filtern](/docs/de/claude-code-on-the-web#archive-sessions). Das Wechseln von Gesprächen, während ein Gerät noch verbunden ist, archiviert die Sitzung nicht.

<h3 id="enable-remote-control-for-all-sessions">
  Aktivieren Sie Remote Control für alle Sitzungen
</h3>

Remote Control wird nur aktiviert, wenn Sie explizit `claude remote-control`, `claude --remote-control` oder `/remote-control` ausführen, es sei denn, die automatische Verbindung ist aktiviert. Um die automatische Verbindung für jede interaktive Sitzung zu aktivieren, führen Sie `/config` in Claude Code aus und setzen Sie **Enable Remote Control for all sessions**. Der Schalter nimmt drei Werte an:

* **`true`**: Verbinden Sie sich automatisch, wenn eine interaktive Sitzung startet.
* **`false`**: Schalten Sie die automatische Verbindung aus, obwohl ein `true` von [verwalteten Einstellungen](/docs/de/managed-settings) es übertrumpft, da Claude Code die Auswahl in Ihren Benutzereinstellungen speichert. Ein `false` in Projekt- oder lokalen Einstellungen (`.claude/settings.json`, `.claude/settings.local.json`) schaltet die automatische Verbindung aus, auch über einem verwalteten `true`.
* **`default`**: Löschen Sie Ihre Auswahl und folgen Sie dem Standard Ihrer Organisation, falls einer festgelegt ist, andernfalls Claude Codes aktueller Standard.

Derselbe Schalter wird außerhalb der CLI angezeigt:

* **Desktop-App**: **Einstellungen > Claude Code > Enable remote control by default**.
* **VS Code-Erweiterung**: **Enable Remote Control for all sessions** im Abschnitt „Einstellungen" des [Befehlsmenüs](/docs/de/vs-code#use-the-prompt-box). Erfordert Claude Code v2.1.203 oder später.

Um die automatische Verbindung stattdessen aus einer Einstellungsdatei zu aktivieren, setzen Sie [`remoteControlAtStartup`](/docs/de/settings-reference#remotecontrolatstartup) auf `true` in Ihrer Benutzer-`~/.claude/settings.json` oder in [verwalteten Einstellungen](/docs/de/managed-settings). In Projekt- oder lokalen Einstellungen (`.claude/settings.json`, `.claude/settings.local.json`) ehrt Claude Code ein `false` und schaltet die automatische Verbindung für dieses Repository aus, ignoriert aber ein `true`, sodass eine eingecheckte Datei Remote Control nicht für alle aktivieren kann, die das Repository öffnen.

Die automatische Verbindung meldet sich mit Ihrem eigenen claude.ai-Konto an, daher wird eine Sitzung, die sie startet, nur in Ihren eigenen Claude-Apps angezeigt und gewährt niemandem sonst Zugriff.

Mit dieser Einstellung registriert jeder interaktive Claude Code-Prozess eine Remote-Sitzung. Wenn Sie mehrere Instanzen ausführen, erhält jede ihre eigene Remote-Sitzung. Um mehrere gleichzeitige Sitzungen aus einem einzelnen Prozess auszuführen, verwenden Sie stattdessen den [Server-Modus](#start-a-remote-control-session).

<h3 id="resume-sessions-after-stopping-the-server">
  Sitzungen nach dem Stoppen des Servers fortsetzen
</h3>

Wenn Sie `claude remote-control` mit Strg+C stoppen, reagieren die Sitzungen, die es bediente, nicht mehr von Ihrem Telefon oder Browser. Solange Sie nicht einen anderen `claude remote-control` im selben Verzeichnis ausführten und diesen nicht mit `--no-create-session-in-dir` starteten, archiviert Claude Code sie nicht. Um sie zurückzubringen, führen Sie einen dieser Befehle im selben Verzeichnis aus:

* **`claude remote-control`**: Bringt jede Sitzung zurück, die der Server bediente.
* **`claude remote-control --continue`**: Bringt nur die Sitzung zurück, die der Server startete, und beendet sich, wenn diese Sitzung endet. Wenn dieses Verzeichnis keinen Datensatz hat, verwendet Claude Code den neuesten aus den anderen Git-Worktrees dieses Repositories.
* **`claude remote-control --session-id <id>`**: Bringt nur die Sitzung zurück, deren ID Sie übergeben, und beendet sich, wenn diese Sitzung endet. Die ID ist der Teil der Sitzungs-URL unter claude.ai/code zwischen `/code/` und jedem `?`.

Diese Befehle funktionieren etwa vier Stunden, nachdem der Server gestoppt wurde. Danach führen Sie `claude remote-control` aus, um eine neue Sitzung zu starten. Wenn Sie eine Sitzung in der Zwischenzeit archiviert haben, entarchivieren `--continue` und `--session-id` sie auf Claude Code v2.1.228 oder später.

Um eine Sitzung zurückzubringen, die Sie mit `claude --remote-control` oder `/remote-control` gestartet haben, setzen Sie das Gespräch mit `claude --continue` oder `claude --resume` fort. Ob Claude Code erneut verbindet und zu welcher Sitzung, hängt vom [Wiederverbindungsdatensatz](#resume-outcomes) des Gesprächs ab.

Wenn Sie das Gespräch in einem zweiten Terminal fortsetzen, während das erste noch Remote Control aktiviert hat, druckt Claude Code eine Benachrichtigung im zweiten Terminal und lässt Remote Control dort stattdessen aus, anstatt die Sitzung vom ersten zu nehmen. Während Remote Control dort aus bleibt, sieht Claude in diesem Terminal nicht [Ihre Sitzungen auf anderen Maschinen](/docs/de/cross-session-messaging#see-which-sessions-claude-can-reach), und sie können es nicht erreichen. Führen Sie `/remote-control` im zweiten Terminal aus, um Remote Control dorthin zu verschieben.

Wenn Sie ein Gespräch in Claude Desktop oder einer IDE-Erweiterung fortsetzen, die Remote Control aktiviert hatte, hängt Claude Code es erneut an die bestehende claude.ai-Sitzung an, anstatt eine neue zur Sitzungsliste hinzuzufügen.

<h2 id="connection-and-security">
  Verbindung und Sicherheit
</h2>

Ihre lokale Claude Code-Sitzung stellt nur ausgehende HTTPS-Anfragen und öffnet niemals eingehende Ports auf Ihrem Computer. Wenn Sie Remote Control starten, registriert es sich bei der Anthropic-API und fragt nach Arbeit ab. Wenn Sie sich von einem anderen Gerät aus verbinden, leitet der Server Nachrichten zwischen dem Web- oder Mobile-Client und Ihrer lokalen Sitzung über eine Streaming-Verbindung weiter.

Der gesamte Datenverkehr verläuft über die Anthropic-API über TLS, die gleiche Transportsicherheit wie jede Claude Code-Sitzung. Die Verbindung verwendet mehrere kurzlebige Anmeldeinformationen, die jeweils auf einen einzelnen Zweck beschränkt sind und unabhängig ablaufen. Wenn die Registrierungsanmeldeinformation eines `claude remote-control`-Servers abläuft, registriert sich der Server erneut bei der Anthropic-API und bedient weiterhin seine Sitzungen.

Während Remote Control verbunden ist, werden das Sitzungstranskript, einschließlich Ihrer Nachrichten, Claudes Antworten und Werkzeugaktivität, auf Anthropic-Servern gespeichert. Das gespeicherte Transkript hält die Konversation auf Ihren Geräten synchron und ermöglicht es der Sitzung, sich nach einem Netzwerkausfall erneut zu verbinden. Ausführung und Dateisystemzugriff bleiben auf Ihrem Computer, und gespeicherte Transkripte werden gemäß der [Datenutzungsrichtlinie](/docs/de/data-usage) beibehalten.

Um Remote Control vollständig auszuschalten, verwenden Sie die Einstellung [`disableRemoteControl`](/docs/de/settings-reference#disableremotecontrol). Organisationen mit Compliance-Anforderungen wie Zero Data Retention können Remote Control nicht aktivieren.

<h2 id="trusted-devices">
  Vertrauenswürdige Geräte
</h2>

<Note>
  Vertrauenswürdige Geräte befinden sich derzeit in der Beta-Phase. Funktionen und Möglichkeiten können sich weiterentwickeln, während die Erfahrung verfeinert wird.

  Vertrauenswürdige Geräte sind auf Pro-, Max-, Team- und Enterprise-Plänen verfügbar und sind standardmäßig deaktiviert. Bei Team- und Enterprise-Plänen aktiviert ein Inhaber die Funktion für die Organisation. Bei Pro- und Max-Plänen aktivieren Sie **Vertrauenswürdige Geräte erforderlich** selbst in Ihren Einstellungen auf der Seite „Zusammenarbeit" oder „Konto".
</Note>

Vertrauenswürdige Geräte erfordern, dass jedes Mitglied Ihrer Organisation oder Sie allein bei einem Pro- oder Max-Plan ihr Gerät überprüfen, bevor sie Remote-Control-Sitzungen von claude.ai, den Claude-Mobile-Apps oder Claude Desktop anzeigen oder steuern können. Es bindet den Remote-Control-Zugriff an ein bekanntes Gerät und eine aktuelle Authentifizierung, nicht nur an ein angemeldetes Konto.

Wenn die Einstellung aktiviert ist, erfordert die Interaktion mit einer Remote-Control-Sitzung beide der folgenden Voraussetzungen:

* **Ein registriertes Gerät**: Jeder Browser, jedes Telefon oder jede Desktop-App, die ein Mitglied für Remote Control verwendet, registriert seine eigenen Anmeldedaten. Die Registrierung wird nur kurz nach einer vollständigen Anmeldung angeboten, sodass ein Gerät als Teil einer echten Authentifizierung zur vertrauenswürdigen Liste hinzugefügt wird, anstatt stillschweigend im Hintergrund.
* **Eine aktuelle Anmeldung**: Die Anmeldung des Mitglieds darf nicht älter als 18 Stunden sein. Anstatt sich jeden Tag erneut anzumelden, bestätigen Mitglieder ihre Anwesenheit mit Face ID, Touch ID, Windows Hello oder einem Passkey. Dieser biometrische Schritt aktualisiert die Sitzung sofort.

Biometrische Überprüfungen werden auf dem Gerät über das Betriebssystem oder den Browser durchgeführt, denselben Mechanismus wie die Passkey-Anmeldung. Anthropic erhält oder speichert niemals Fingerabdrücke, Gesichtsdaten oder andere biometrische Informationen. Nur der öffentliche Schlüssel des Geräts und grundlegende Metadaten wie Anzeigename, Plattform und Registrierungszeit werden gespeichert.

Die Einstellung gilt nur für Remote Control. Regulärer Claude-Chat, Claude Code im Terminal und API-Nutzung sind nicht betroffen.

<h3 id="enable-trusted-devices-for-your-organization">
  Aktivieren Sie Vertrauenswürdige Geräte für eine Team- oder Enterprise-Organisation
</h3>

Ein Inhaber aktiviert die Einstellung über die claude.ai-Organisationseinstellungen.

<Steps>
  <Step title="Gehen Sie zur Seite „Funktionen&#x22;">
    Gehen Sie zu [**Organisationseinstellungen > Funktionen > Remote-Sitzungen**](https://claude.ai/admin-settings/capabilities). Der Umschalter **Vertrauenswürdige Geräte erforderlich** wird in diesem Abschnitt angezeigt.
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

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control vs. Cloud-Sitzungen
</h2>

Remote Control und [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) verwenden beide die claude.ai/code-Schnittstelle. Der Hauptunterschied liegt darin, wo die Sitzung ausgeführt wird: Remote Control wird auf Ihrem Computer ausgeführt, sodass Ihre lokalen MCP-Server, Tools und Projektkonfiguration verfügbar bleiben. Eine Cloud-Sitzung wird in Cloud-Infrastruktur ausgeführt, standardmäßig von Anthropic verwaltet.

Verwenden Sie Remote Control, wenn Sie sich mitten in lokaler Arbeit befinden und von einem anderen Gerät aus weitermachen möchten. Verwenden Sie eine Cloud-Sitzung, wenn Sie eine Aufgabe ohne lokale Einrichtung starten möchten, an einem Repository arbeiten, das Sie nicht geklont haben, oder mehrere Aufgaben parallel ausführen möchten.

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
* **Lokaler Prozess muss weiterhin ausgeführt werden**: Remote Control wird als lokaler Prozess ausgeführt. Wenn Sie das Terminal schließen, VS Code beenden oder den `claude`-Prozess anderweitig beenden, geht die Sitzung offline, bis Sie sie [wieder aktivieren](#resume-sessions-after-stopping-the-server). Sofern Claude nicht gerade eine Aufgabe ausführt, zeigen claude.ai und die Claude-App die Sitzung innerhalb von Sekunden nach dem Beenden des Prozesses als offline an. Um eine Sitzung auf einem Remote-Computer nach dem Trennen von SSH weiterhin auszuführen, starten Sie sie in `tmux` oder `screen`.
* **Abgestürzte Sitzungen im Server-Modus**: Wenn eine von `claude remote-control` bereitgestellte Sitzung abstürzt, senden Sie ihr eine Nachricht von einem verbundenen Gerät. Claude Code stellt sie erneut bereit. Sie müssen den Server nicht neu starten. Erfordert Claude Code v2.1.238 oder später.
* **HTTP 403-Ablehnungen bei einer verbundenen Sitzung**: Sobald eine interaktive Sitzung verbunden ist, versucht Claude Code bis zu drei Minuten lang erneut, wenn etwas zwischen Ihrem Computer und den Servern von Anthropic mit HTTP 403 antwortet, was nach einem VPN- oder Netzwerkwechsel vorkommen kann. Wenn die Ablehnungen länger andauern, trennt Claude Code die Verbindung, und der Grund gibt an, was abgelehnt hat: ein Netzwerk-Edge, oder ein Proxy, VPN oder eine Firewall in Ihrem eigenen Netzwerk.
* **Längerer Netzwerkausfall**: Wenn Ihr Computer aktiv ist, aber das Netzwerk nicht erreichen kann, hängt das, was Sie als Nächstes tun, vom Modus ab:
  * **Server-Modus**: Claude Code gibt nach etwa 10 Minuten auf und der `claude remote-control`-Prozess wird beendet. Führen Sie `claude remote-control` erneut aus, um eine neue Sitzung zu starten.
  * **Interaktive Sitzung**: Arbeiten Sie lokal weiter. Claude Code versucht es erneut, solange der Ausfall andauert, und verbindet sich automatisch wieder, wenn das Netzwerk zurückkommt.
* **Presence-Heartbeats schlagen fehl**: Wenn sich eine interaktive Sitzung mit `could not reach the Remote Control server for about 30 minutes` trennt, führen Sie `/remote-control` aus, um die Verbindung wiederherzustellen. Claude Code zeigt diese Nachricht nur an, wenn die Presence-Heartbeats der Sitzung fehlgeschlagen sind, während der Rest der Verbindung bestehen blieb; es registriert die Sitzung für etwa 30 Minuten erneut, bevor die Verbindung getrennt wird.
* **Weitergeleitete Dialoge verfallen**: Claude Code hält Berechtigungsaufforderungen und `AskUserQuestion`-Fragen offen, bis Sie sie beantworten. Wenn Claude Code eine andere Art von Dialog an die Remote-Sitzung weiterleitet, z. B. die Modellauswahlmeldung, die nach einer Sicherheitsablehnung angezeigt wird, wartet es standardmäßig fünf Minuten, schließt dann den Dialog und fährt mit dem Standardwert „Keine Aktion" des Dialogs fort. Legen Sie [`dialogExpiry`](/docs/de/settings-reference#dialogexpiry) fest, um die Frist anzupassen oder zu deaktivieren. Erfordert Claude Code v2.1.224 oder später.
* **Die Fable-Zustimmungsaufforderung für Nutzungsguthaben wird nicht weitergeleitet**: Claude Code zeigt die [Fable-Zustimmungsaufforderung für Nutzungsguthaben](/docs/de/model-config#fable-and-usage-credits) in der Mitte der Sitzung nur dort an, wo die Sitzung ausgeführt wird, nicht auf Ihrem Gerät. Wenn die Sitzung in einem Terminal ausgeführt wird und niemand dort antwortet, bevor Claude Code die Aufforderung schließt, endet der Zug, ohne die Anfrage zu senden; siehe [Die Aufforderung zur Bestätigung blieb unbeantwortet](/docs/de/errors#the-prompt-to-confirm-went-unanswered).
* **Einige Befehle sind nur lokal verfügbar**: Befehle, die nur in der Terminal-Schnittstelle ausgeführt werden, wie `/plugin` oder `/resume`, funktionieren nur über die lokale CLI, unabhängig davon, ob Sie ein Argument übergeben oder nicht. Die folgenden funktionieren von mobil und Web aus:
  * Textausgabe-Befehle: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits`, `/recap` und `/reload-plugins`. `/usage-credits` gibt die Abrechnungs-URL aus, anstatt einen Browser zu öffnen. `/reload-plugins` funktioniert nur, wenn die Sitzung in einem interaktiven Terminal ausgeführt wird; eine Sitzung ohne eines lehnt es ab.
  * `/model`, `/effort`, `/fast`, `/color` und `/rename`: übergeben Sie den Wert als Argument, zum Beispiel `/model sonnet` oder `/effort high`. Von mobil und Web aus akzeptieren `/model` und `/effort` das Argument anstelle der Terminal-Auswahl oder des Schiebereglers.
  * `/mcp`: Von der mobilen App aus gibt es eine Textzusammenfassung des Server-Status zurück, anstatt die Auswahl zu öffnen. Im Web öffnet `/mcp` allein ein Verzeichnis von [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai), anstatt die Zusammenfassung zurückzugeben. Die `reconnect`-, `enable`- und `disable`-[Unterbefehle](/docs/de/commands#all-commands) funktionieren von beiden aus. Im Gegensatz zur lokalen CLI verbindet `/mcp reconnect` ohne Servernamen jeden Server wieder, der fehlgeschlagen ist oder eine Authentifizierung benötigt.
  * `/config`: Von der mobilen App aus übergeben Sie `key=value`, um eine Einstellung festzulegen, oder führen Sie es ohne Argument aus, um die Schlüssel aufzulisten, die Sie festlegen können. Im Web öffnet `/config` stattdessen den Claude Code-Bereich Ihrer Einstellungen und ignoriert Text nach dem Befehl.
  * Auf Team und Enterprise sendet `/usage-credits` von mobil oder Web keine [Anfrage für Nutzungsguthaben an Ihren Administrator](/docs/de/costs#add-usage-credits-to-your-subscription). Das Senden erfordert eine Bestätigung, die nur in der interaktiven CLI angezeigt wird, daher teilt Ihnen der Befehl mit, dass Sie ihn stattdessen dort ausführen sollen. Vor v2.1.211 sendete die Textform die Anfrage ohne Bestätigung.
  * `/autocompact`, ab v2.1.221: übergeben Sie die Fenstergröße als Argument, zum Beispiel `/autocompact 500k`. Ohne Argument gibt es die aktuelle Fenstergröße als Text aus, anstatt den Dialog zu öffnen, den der Befehl in einer Terminal-Sitzung anzeigt.
  * `/advisor`, ab v2.1.260: übergeben Sie das Modell als Argument, zum Beispiel `/advisor opus`, oder übergeben Sie `off`, um den Advisor auszuschalten. Beide Formen gelten nur für die aktuelle Sitzung und lassen Ihren gespeicherten Standard unverändert. Ohne Argument gibt es den aktuellen Advisor als Text aus, anstatt die Auswahl zu öffnen.
  * `/output-style`, ab v2.1.269: übergeben Sie den Stilnamen als Argument, zum Beispiel `/output-style concise`, oder führen Sie es ohne Argument aus, um die Stile aufzulisten. Von mobil und Web aus können Sie nur [integrierte Stile](/docs/de/output-styles#built-in-output-styles) auflisten und auswählen. Um einen [benutzerdefinierten Stil](/docs/de/output-styles#create-a-custom-output-style) zu verwenden, wählen Sie ihn in der Sitzung selbst aus.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  „Remote Control erfordert ein claude.ai-Abonnement"
</h3>

Sie sind nicht mit einem claude.ai-Konto angemeldet, oder eine andere Anmeldeinformation hat Vorrang vor Ihrer Anmeldung. Die Meldung nimmt eine dieser Formen an:

* Abgemeldet, von `/remote-control` oder `--remote-control`: `Remote Control requires a claude.ai subscription.` oder `/remote-control requires a claude.ai subscription.`
* Abgemeldet, von `claude remote-control`: `You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.`
* Angemeldet, aber ein API-Schlüssel oder Token wird verwendet: `Remote Control requires claude.ai subscription auth.` gefolgt von der verwendeten Anmeldeinformation, wie `ANTHROPIC_API_KEY is set, so this session is using API-key auth`. Eine `apiKeyHelper`-Einstellung und `ANTHROPIC_AUTH_TOKEN` werden auf die gleiche Weise benannt.

Führen Sie `claude auth login` aus und wählen Sie die claude.ai-Option. Wenn die Meldung `ANTHROPIC_API_KEY` oder `ANTHROPIC_AUTH_TOKEN` nennt, heben Sie die Festlegung überall dort auf, wo sie festgelegt ist: in Ihrer Shell-Umgebung oder im `env`-Block einer [Einstellungsdatei](/docs/de/settings-reference#env). Wenn sie `apiKeyHelper` nennt, entfernen Sie diese Einstellung.

Vor v2.1.206 meldete das Ausführen von `/remote-control` während der Abmeldung `Unknown command: /remote-control` statt dieser Meldung.

<h3 id="remote-control-requires-a-full-scope-login-token">
  „Remote Control erfordert ein Token mit vollständigem Umfang"
</h3>

Sie sind mit einem langlebigen Token von `claude setup-token` oder der Umgebungsvariable `CLAUDE_CODE_OAUTH_TOKEN` authentifiziert. Diese Token können nur Modellanfragen stellen, daher können sie keine Remote Control-Sitzungen einrichten. Führen Sie `claude auth login` aus, um sich stattdessen mit einem Token mit vollständigem Umfang zu authentifizieren.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  „Ihre Organisation für die Remote Control-Berechtigung konnte nicht bestimmt werden"
</h3>

Ihre zwischengespeicherten Kontoinformationen sind veraltet oder unvollständig. Führen Sie `claude auth login` aus, um sie zu aktualisieren.

<h3 id="remote-control-isn’t-enabled-for-this-account">
  „Remote Control ist für dieses Konto nicht aktiviert"
</h3>

Claude Code hat die Remote Control-Verfügbarkeit für das Konto überprüft, mit dem Sie angemeldet sind, und die Überprüfung kam als deaktiviert zurück. Die übliche Ursache sind zwischengespeicherte Berechtigungen, die nach einer Planänderung veraltet sind. Führen Sie `claude auth logout` und dann `claude auth login` aus, um sie zu aktualisieren, und aktualisieren Sie Claude Code, wenn Sie eine alte Version verwenden.

Führen Sie `claude doctor` aus, um zu sehen, welche einzelne Berechtigungsprüfung fehlgeschlagen ist. Umgebungsvariablenkonflikte, unerreichbare Prüfungen und die Remote Control-Einstellung Ihrer Organisation erzeugen jeweils ihre eigene Meldung, daher bedeutet dieser Fehler die Kontoebenen-Prüfung selbst.

Vor v2.1.239 lautete diese Meldung „Remote Control ist für Ihr Konto noch nicht aktiviert". Vor v2.1.154 erzeugte auch eine Variable, die die Feature-Flag-Auswertung deaktiviert, wie `DISABLE_TELEMETRY` oder `DO_NOT_TRACK`, diese Meldung; der Eintrag „Remote Control erfordert Feature-Flag-Auswertung" unten behandelt diese Konfiguration.

<h3 id="couldn’t-verify-remote-control-eligibility">
  „Remote Control-Berechtigung konnte nicht überprüft werden"
</h3>

Claude Code konnte den Feature-Flag-Service nicht erreichen, um zu überprüfen, ob Remote Control für Ihr Konto aktiviert ist, normalerweise weil Sie offline sind oder ein Proxy die Anfrage blockiert. Versuchen Sie es erneut, wenn Sie Netzwerkzugriff haben, oder führen Sie `claude doctor` aus, um Details zu erhalten. Die zugehörige Meldung „Remote Control-Richtlinie Ihrer Organisation konnte nicht überprüft werden" bedeutet, dass Claude Code diese Richtlinie nicht lesen konnte, und hat die gleiche Lösung. Beide Meldungen wurden in v2.1.178 hinzugefügt.

<h3 id="remote-control-requires-feature-flag-evaluation">
  „Remote Control erfordert Feature-Flag-Auswertung"
</h3>

Eine dieser Variablen ist festgelegt: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` oder `DISABLE_GROWTHBOOK`](/docs/de/env-vars). Jede von ihnen deaktiviert die Feature-Flag-Auswertung, von der die Remote Control-Verfügbarkeit abhängt, und die vollständige Meldung nennt die Variable, die Claude Code gefunden hat. Heben Sie diese Variable überall dort auf, wo sie festgelegt ist, in Ihrer Shell-Umgebung oder im `env`-Block einer [`settings.json`-Datei](/docs/de/settings-reference#all-settings). Bei Versionen vor 2.1.154 erzeugt die gleiche Konfiguration stattdessen „Remote Control ist für Ihr Konto noch nicht aktiviert".

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  „Remote Control ist nur verfügbar, wenn Sie Claude über api.anthropic.com verwenden"
</h3>

Die Sitzung kommuniziert nicht direkt mit der Anthropic-API, daher gibt es kein claude.ai-Backend zum Koppeln. Dies geschieht auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry. Dies geschieht auch, wenn [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf einen Host verweist, der nicht `api.anthropic.com` ist, wie ein [LLM-Gateway](/docs/de/llm-gateway) oder Proxy, auch wenn Sie sich mit claude.ai anmelden. Vor v2.1.196 zeigte Claude Code diese Meldung nicht für eine benutzerdefinierte `ANTHROPIC_BASE_URL`. Siehe die [Fehlerreferenz](/docs/de/errors#remote-control-requires-the-anthropic-api) für die vollständige Ursachenliste.

Die Meldung nennt, was die Sitzung von der Anthropic-API abgelenkt hat, wie `CLAUDE_CODE_USE_BEDROCK` oder eine benutzerdefinierte `ANTHROPIC_BASE_URL`. Wenn Sie eine berechtigte claude.ai-Anmeldung haben, heben Sie die genannte Variable auf, entfernen Sie sie aus dem `env`-Schlüssel in [Einstellungen](/docs/de/settings), wenn Sie sie dort festgelegt haben, und starten Sie die Sitzung neu. Vor v2.1.219 war die Meldung nur der Satz in der Kopfzeile dieses Abschnitts, daher überprüfen Sie bei älteren Versionen Ihre Umgebung selbst auf Provider-Variablen wie `CLAUDE_CODE_USE_BEDROCK` und `CLAUDE_CODE_USE_VERTEX` sowie auf `ANTHROPIC_BASE_URL`.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  „Remote Control ist durch die Richtlinie Ihrer Organisation deaktiviert"
</h3>

Eine Richtlinie blockiert Remote Control, oder Claude Code konnte die Richtlinie Ihrer Organisation auf diesem Computer nicht laden und hält Remote Control vorerst deaktiviert. Überprüfen Sie diese Ursachen in dieser Reihenfolge:

* **Der Fehler erwähnt `disableRemoteControl`**: Ihr IT-Administrator hat Remote Control auf diesem Gerät über [verwaltete Einstellungen](/docs/de/managed-settings) deaktiviert, unabhängig vom organisationsweiten Schalter und davon, wie Sie angemeldet sind.
* **Ihr claude.ai-Plan ist Pro oder Max**: Claude Code ist immer noch unter einer Team- oder Enterprise-Organisation von einer früheren Anmeldung angemeldet, daher überprüft es die Remote Control-Richtlinie dieser Organisation. Führen Sie `/status` aus, um zu sehen, welcher Plan und welche Organisation Ihre Anmeldung verwendet. Führen Sie `claude auth logout` und dann `claude auth login` aus, um sich erneut unter Ihrem aktuellen Plan anzumelden.
* **Die Organisationsrichtlinie wurde auf diesem Computer nicht geladen**: Führen Sie `claude doctor` aus und lesen Sie die Zeile `Organization policy`. Wenn die Zeile zeigt, dass die Richtlinie nicht geladen ist, ist das das, was Remote Control deaktiviert hält. Vor v2.1.261 druckte `claude doctor` diese Zeile nicht.
* **Die Meldung sagt nicht, dass Sie Ihren Organisationsadministrator kontaktieren sollen**: Ihre Organisation hat eine HIPAA-Konfiguration, die mit Remote Control nicht kompatibel ist, und `/status` listet `HIPAA` in seiner Zeile `Compliance` auf. In diesem Zustand ist der Schalter Remote Control im Admin-Panel ausgegraut, daher kann ein Inhaber ihn dort nicht ändern. Kontaktieren Sie den Anthropic-Support, um Optionen zu besprechen. Vor v2.1.267 zeigte dieser Fall „Remote Control ist für Ihre Organisation aufgrund ihrer Compliance-Richtlinie nicht verfügbar" statt.
* **Andernfalls hat ein Inhaber es für Ihre Organisation nicht aktiviert**: Remote Control ist standardmäßig in Team- und Enterprise-Plänen deaktiviert. Ein Inhaber kann es unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) aktivieren, indem er den Schalter **Remote Control** einschaltet. Dieser Schalter ist eine serverseitige Organisationseinstellung.

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

Ein veraltetes Anmelde-Token verursacht diesen Fehler nicht. Wenn die Anthropic-API das gespeicherte Token ablehnt, beispielsweise weil ein anderer Claude Code-Prozess es bereits aktualisiert hat, aktualisiert Claude Code das Token und versucht es automatisch erneut. Vor v2.1.224 führte ein veraltetes Token zum Fehler beim Remote Control-Start mit dieser Meldung, daher konnten Sitzungen, die auf [automatische Verbindung](#enable-remote-control-for-all-sessions) eingestellt waren, beim Start intermittierend fehlschlagen.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  „Konnte nicht erneut mit Ihrer Remote Control-Sitzung verbunden werden"
</h3>

Wenn Sie ein Gespräch mit `claude --resume` oder `claude --continue` fortsetzen, verbindet sich Claude Code erneut mit der Remote Control-Sitzung, die in diesem Gespräch aufgezeichnet wurde. Diese Meldung bedeutet, dass die Wiederverbindung aus einem Grund fehlgeschlagen ist, der möglicherweise vorübergehend ist, wie eine Netzwerkunterbrechung oder ein Serverfehler, daher kann Claude Code nicht bestätigen, ob die Remote-Sitzung noch vorhanden ist.

Führen Sie `/remote-control` aus, um die Verbindung erneut zu versuchen, oder starten Sie eine neue Sitzung mit `claude --remote-control`, um eine neue Remote Control-Sitzung zu erstellen. Ihre lokale Sitzung läuft ohne Remote Control weiter.

<span id="resume-outcomes" />Wenn Sie fortsetzen, können Sie auch eines dieser Ergebnisse statt dieser Meldung erhalten:

* **Der Server meldet die aufgezeichnete Sitzung als weg, oder der Wiederverbindungsdatensatz nennt ein anderes Konto**: Claude Code geht nach dem aus, was der Wiederverbindungsdatensatz des Gesprächs sagt:
  * **Der Datensatz nennt Ihr angemeldetes Konto**: Claude Code startet eine Ersatzsitzung mit einem automatisch generierten Namen und lässt die früheren Meldungen des Gesprächs aus. Sie erhalten dies, nachdem Sie die Sitzung von claude.ai oder der Claude-App gelöscht haben, zum Beispiel.
  * **Der Datensatz nennt ein anderes Konto**: Claude Code startet eine neue Sitzung ohne die früheren Meldungen des Gesprächs und ohne eine Meldung anzuzeigen, unabhängig davon, ob die aufgezeichnete Sitzung noch vorhanden ist.
  * **Der Datensatz sagt nicht, welches Konto die Sitzung besaß, oder Claude Code kann Ihre gespeicherte Anmeldung nicht lesen**: Claude Code zeigt stattdessen [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable), startet nichts und entfernt den Datensatz aus dem Gespräch.
* **Sie haben Remote Control vor dem Fortsetzen ausgeschaltet**: Sofern die App, die Claude Code hostet, Claude Code nicht mitgeteilt hat, dass die App die claude.ai-Sitzung besitzt, entfernte Claude Code den Wiederverbindungsdatensatz, als Sie Remote Control vom CLI-[Statusbereich](#check-connection-status), der VS Code-Erweiterung oder einem Host, der auf dem [Agent SDK](/docs/de/agent-sdk/overview) basiert, ausschalteten, daher verbindet es sich nicht erneut. Wenn eine besitzende App es ausschaltete, behielt Claude Code den Datensatz und verbindet sich erneut.
* **Ein anderer Claude Code auf diesem Computer hat immer noch die Sitzung**: Sie sehen einen Hinweis, der mit `Remote Control not started here` beginnt, und Claude Code [lässt Remote Control in der fortgesetzten Sitzung ausgeschaltet](#resume-sessions-after-stopping-the-server). Führen Sie dort `/remote-control` aus, um es zu verschieben.

<span id="reconnect-history" />Vor v2.1.232 reagierte Claude Code anders, wenn der Server die aufgezeichnete Sitzung als weg meldete. Von v2.1.227 bis v2.1.231 weigerte sich Claude Code, eine Ersatzsitzung zu starten, auch wenn der Datensatz Ihrem Konto entsprach. Bis v2.1.226 startete Claude Code eine Ersatzsitzung, unabhängig davon, ob der Datensatz Ihrem Konto entsprach, und erstellte sie in v2.1.224 bis v2.1.226 unter dem auf diesem Computer angemeldeten Konto, niemals unter einem anderen Konto, ohne die früheren Meldungen des Gesprächs hochzuladen. Vor v2.1.200 erstellte Claude Code nach jedem Wiederverbindungsfehler eine neue Sitzung.

<h3 id="previous-session-is-unavailable">
  „Previous session is unavailable — run /remote-control to start a new one"
</h3>

Claude Code konnte die vorherige Remote Control-Sitzung nicht zurückbringen und stoppte, anstatt von selbst eine neue zu starten. Sie können diese Meldung sehen, nachdem Sie ein Gespräch mit `claude --resume` oder `claude --continue` fortsetzen, oder nachdem Claude Code [sich selbst nach einer Trennung erneut verbindet](/docs/de/errors#remote-control-couldnt-refresh-your-login).

Führen Sie `/remote-control` aus, um eine neue Remote Control-Sitzung unter der aktuellen Anmeldung zu starten; Ihre lokale Sitzung läuft ohne Remote Control weiter. Die zugehörige Meldung `Remote Control could not verify the signed-in account — run /remote-control to reconnect` hat die gleiche Lösung; Claude Code zeigt sie, wenn sich das angemeldete Konto zwischen der Validierung und der Wiederverbindung geändert hat oder nicht gelesen werden konnte. Wenn Sie `/remote-control` nach `Previous session is unavailable` ausführen, ohne Claude Code zuerst neu zu starten, lässt Claude Code die früheren Meldungen des Gesprächs aus der neuen Sitzung aus.

Beim Fortsetzen startet Claude Code [eine neue Sitzung an seiner Stelle](#resume-outcomes) nur, wenn der Wiederverbindungsdatensatz des Gesprächs das Konto nennt, das die Sitzung besaß, weil der Server eine Sitzung, die Sie gelöscht haben, und eine Sitzung, die ein anderes Konto besitzt, auf die gleiche Weise meldet. Claude Code vor v2.1.227 zeichnete dieses Konto nicht auf, und Claude Code kann den Datensatz nicht überprüfen, wenn es Ihre gespeicherte Anmeldung nicht lesen kann. Claude Code vor v2.1.232 zeigte stattdessen `Remote Control could not resume the previous session under the current login — run /remote-control to start fresh` in [einem anderen Satz von Fällen](#reconnect-history).

<h3 id="remote-control-got-an-unexpected-server-response">
  „Remote Control got an unexpected server response"
</h3>

Der Remote Control-Server akzeptierte eine Anfrage, antwortete aber in einer Form, die diese Version von Claude Code nicht lesen konnte, während die Remote-Sitzung erstellt oder ihre Anmeldeinformationen abgerufen wurden. Das erneute Versuchen in der gleichen Version schlägt auf die gleiche Weise fehl. Führen Sie `claude update` aus und führen Sie dann `/remote-control` aus, um sich erneut zu verbinden. Diese Meldung wurde in v2.1.225 hinzugefügt.

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

Claude Code bietet mehrere Möglichkeiten, um zu arbeiten, wenn Sie nicht an Ihrem Terminal sind. Sie unterscheiden sich darin, was die Arbeit auslöst, wo Claude ausgeführt wird und wie viel Setup Sie benötigen.

|                                                          | Auslöser                                                                                                    | Claude wird ausgeführt auf                                                                    | Setup                                                                                                                                      | Am besten geeignet für                                               |
| :------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| [Dispatch](/docs/de/desktop#sessions-from-dispatch)           | Senden Sie eine Aufgabe aus der Claude Mobile-App                                                           | Ihr Computer (Desktop)                                                                        | [Koppeln Sie die Mobile-App mit Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegieren von Arbeit, wenn Sie weg sind, minimales Setup            |
| [Remote Control](/docs/de/remote-control)                     | Steuern Sie eine laufende Sitzung von [claude.ai/code](https://claude.ai/code) oder der Claude Mobile-App   | Ihr Computer (CLI oder VS Code)                                                               | Führen Sie `claude remote-control` aus                                                                                                     | Steuerung laufender Arbeiten von einem anderen Gerät                 |
| [Channels](/docs/de/channels)                                 | Pushen Sie Ereignisse aus einer Chat-App wie Telegram oder Discord oder Ihrem eigenen Server                | Ihr Computer (CLI)                                                                            | [Installieren Sie ein Channel-Plugin](/docs/de/channels#quickstart) oder [erstellen Sie Ihr eigenes](/docs/de/channels-reference)                    | Reagieren auf externe Ereignisse wie CI-Fehler oder Chat-Nachrichten |
| [Slack](/docs/de/slack)                                       | Erwähnen Sie `@Claude` in einem Team-Kanal                                                                  | Anthropic Cloud                                                                               | [Installieren Sie die Slack-App](/docs/de/slack#setting-up-claude-code-in-slack) mit [Claude Code im Web](/docs/de/claude-code-on-the-web) aktiviert | PRs und Reviews aus Team-Chat                                        |
| [Self-hosted environments](/docs/de/self-hosted-environments) | Starten Sie eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) und wählen Sie die Umgebung Ihrer Organisation | Infrastruktur Ihrer Organisation                                                              | [Stellen Sie Runner bereit](/docs/de/self-hosted-environments-quickstart), in Team- und Enterprise-Plänen                                       | Cloud-Sitzungen, die in Ihrem Netzwerk ausgeführt werden müssen      |
| [Scheduled tasks](/docs/de/scheduled-tasks)                   | Legen Sie einen Zeitplan fest                                                                               | [CLI](/docs/de/scheduled-tasks), [Desktop](/docs/de/desktop-scheduled-tasks) oder [Cloud](/docs/de/routines) | Wählen Sie eine Häufigkeit                                                                                                                 | Wiederkehrende Automatisierung wie tägliche Reviews                  |

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Claude Code im Web](/docs/de/claude-code-on-the-web): Führen Sie Sitzungen in der Cloud aus, anstatt auf Ihrem Computer, konfiguriert über [Cloud-Umgebungen](/docs/de/cloud-environments)
* [Sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging): Lassen Sie Claude Ihre Sitzungen auf anderen Computern oder auf [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) anschreiben
* [Kanäle](/docs/de/channels): Leiten Sie Telegram, Discord oder iMessage in eine Sitzung weiter, damit Claude auf Nachrichten reagiert, während Sie weg sind
* [Dispatch](/docs/de/desktop#sessions-from-dispatch): Senden Sie eine Aufgabe von Ihrem Telefon aus, und sie kann eine Desktop-Sitzung spawnen, um sie zu bearbeiten
* [Authentifizierung](/docs/de/authentication): Richten Sie `/login` ein und verwalten Sie Anmeldeinformationen für claude.ai
* [CLI-Referenz](/docs/de/cli-reference): Vollständige Liste von Flags und Befehlen einschließlich `claude remote-control`
* [Sicherheit](/docs/de/security): Wie Remote Control-Sitzungen in das Claude Code-Sicherheitsmodell passen
* [Datennutzung](/docs/de/data-usage): Welche Daten während lokaler, Remote Control- und Cloud-Sitzungen durch die Anthropic-API fließen
