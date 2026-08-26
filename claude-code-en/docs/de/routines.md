> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatisieren Sie Arbeitsabläufe mit Routinen

> Setzen Sie Claude Code auf Autopilot. Definieren Sie Routinen, die nach einem Zeitplan ausgeführt werden, durch API-Aufrufe ausgelöst werden oder auf GitHub-Ereignisse von der von Anthropic verwalteten Cloud-Infrastruktur reagieren.

<Note>
  Routinen befinden sich in der Forschungsvorschau. Verhalten, Limits und die API-Oberfläche können sich ändern.
</Note>

Eine Routine ist eine gespeicherte Claude Code-Konfiguration: ein Prompt, ein oder mehrere Repositories und eine Reihe von [Konnektoren](/docs/de/mcp), die einmal verpackt und automatisch ausgeführt werden. Routinen werden auf der von Anthropic verwalteten Cloud-Infrastruktur ausgeführt, sodass sie weiterhin funktionieren, wenn Ihr Laptop geschlossen ist.

Jede Routine kann einen oder mehrere Trigger haben:

* **Geplant**: Ausführung nach einem wiederkehrenden Zeitplan wie stündlich, nächtlich oder wöchentlich, oder einmalig zu einem bestimmten zukünftigen Zeitpunkt
* **API**: Auslösung auf Anforderung durch Senden eines HTTP POST an einen Routine-spezifischen Endpunkt mit einem Bearer-Token
* **GitHub**: Automatische Ausführung als Reaktion auf Repository-Ereignisse wie Pull Requests oder Releases

Eine einzelne Routine kann Trigger kombinieren. Beispielsweise kann eine PR-Review-Routine nachts ausgeführt werden, von einem Deploy-Skript ausgelöst werden und auch auf jeden neuen PR reagieren.

Routinen sind auf Pro-, Max-, Team- und Enterprise-Plänen mit aktiviertem [Claude Code im Web](/docs/de/claude-code-on-the-web) verfügbar. Erstellen und verwalten Sie sie unter [claude.ai/code/routines](https://claude.ai/code/routines) oder über die CLI mit `/schedule`.

Team- und Enterprise-Administratoren können Routinen für alle Mitglieder mit dem Routinen-Toggle unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) deaktivieren. Wenn deaktiviert, werden vorhandene Routinen nicht mehr ausgeführt und Mitglieder können keine neuen erstellen.

Diese Seite behandelt das Erstellen einer Routine, das Konfigurieren jedes Trigger-Typs, das Verwalten von Ausführungen und wie Nutzungslimits angewendet werden.

<h2 id="example-use-cases">
  Beispiel-Anwendungsfälle
</h2>

Jedes Beispiel kombiniert einen Trigger-Typ mit der Art von Arbeit, für die Routinen geeignet sind: unbeaufsichtigt, wiederholbar und an ein klares Ergebnis gebunden.

**Backlog-Wartung.** Ein Schedule-Trigger wird jede Wochnacht gegen Ihren Issue-Tracker über einen Konnektor ausgeführt. Die Routine liest seit der letzten Ausführung geöffnete Issues, wendet Labels an, weist Besitzer basierend auf dem referenzierten Code-Bereich zu und postet eine Zusammenfassung an Slack, damit das Team den Tag mit einer gepflegten Warteschlange beginnt.

**Alert-Triage.** Ihr Monitoring-Tool ruft den API-Endpunkt der Routine auf, wenn ein Error-Schwellenwert überschritten wird, und übergibt den Alert-Body als `text`. Die Routine zieht den Stack Trace, korreliert ihn mit kürzlichen Commits im Repository und öffnet einen Draft-Pull-Request mit einem vorgeschlagenen Fix und einem Link zurück zum Alert. Der On-Call-Mitarbeiter überprüft den PR, anstatt von einem leeren Terminal zu beginnen.

**Benutzerdefinierte Code-Review.** Ein GitHub-Trigger wird auf `pull_request.opened` ausgeführt. Die Routine wendet die eigene Review-Checkliste Ihres Teams an, hinterlässt Inline-Kommentare zu Sicherheits-, Performance- und Style-Problemen und fügt einen Zusammenfassungskommentar hinzu, damit sich menschliche Reviewer auf Design statt auf mechanische Überprüfungen konzentrieren können.

**Deploy-Verifizierung.** Ihre CD-Pipeline ruft den API-Endpunkt der Routine nach jedem Production-Deploy auf. Die Routine führt Smoke-Tests gegen den neuen Build durch, scannt Error-Logs auf Regressionen und postet ein Go oder No-Go zum Release-Channel, bevor das Deploy-Fenster schließt.

**Dokumentations-Drift.** Ein Schedule-Trigger wird wöchentlich ausgeführt. Die Routine scannt seit der letzten Ausführung zusammengeführte PRs, kennzeichnet Dokumentation, die auf geänderte APIs verweist, und öffnet Update-PRs gegen das Dokumentations-Repository zur Überprüfung durch einen Editor.

**Library-Port.** Ein GitHub-Trigger wird auf `pull_request.closed` ausgeführt, gefiltert auf zusammengeführte PRs in einem SDK-Repository. Die Routine portiert die Änderung zu einem parallelen SDK in einer anderen Sprache und öffnet einen entsprechenden PR, um die beiden Bibliotheken synchron zu halten, ohne dass ein Mensch jede Änderung neu implementiert.

Die folgenden Abschnitte führen Sie durch das Erstellen einer Routine und das Konfigurieren jedes dieser Trigger-Typen.

<h2 id="create-a-routine">
  Erstellen Sie eine Routine
</h2>

Erstellen Sie eine Routine aus dem Web unter [claude.ai/code/routines](https://claude.ai/code/routines), aus der Desktop-App oder aus der CLI. Alle drei Oberflächen schreiben auf dasselbe Cloud-Konto, sodass eine Routine, die Sie in einer erstellen, sofort in den anderen angezeigt wird. Klicken Sie in der Desktop-App auf **Routinen** in der Seitenleiste und dann auf **Neue Routine**, und wählen Sie **Remote**; wenn Sie stattdessen **Lokal** wählen, wird eine [Desktop-geplante Aufgabe](/docs/de/desktop-scheduled-tasks) erstellt, die auf Ihrem Computer ausgeführt wird, anstatt in der Cloud.

Das Erstellungsformular richtet den Prompt der Routine, Repositories, Umgebung, Konnektoren und Trigger ein.

Routinen werden autonom als vollständige Claude Code-Cloud-Sitzungen ausgeführt: Es gibt keinen Berechtigungsmodus-Picker und keine Genehmigungsaufforderungen während einer Ausführung. Die Sitzung kann Shell-Befehle ausführen, [Skills](/docs/de/skills) verwenden, die im geklonten Repository committed sind, und alle Konnektoren aufrufen, die Sie einbeziehen. Was eine Routine erreichen kann, wird durch die Repositories bestimmt, die Sie auswählen, und deren Branch-Push-Einstellung, den [Netzwerkzugriff und die Variablen der Umgebung](/docs/de/claude-code-on-the-web#the-cloud-environment) und die Konnektoren, die Sie einbeziehen. Beschränken Sie jeden dieser Punkte auf das, was die Routine tatsächlich benötigt.

Routinen gehören zu Ihrem individuellen claude.ai-Konto. Sie werden nicht mit Teamkollegen geteilt und zählen gegen die tägliche Ausführungszulage Ihres Kontos. Alles, was eine Routine durch Ihre verbundene GitHub-Identität oder Konnektoren tut, erscheint als Sie: Commits und Pull Requests tragen Ihren GitHub-Benutzer, und Slack-Nachrichten, Linear-Tickets oder andere Konnektor-Aktionen verwenden Ihre verknüpften Konten für diese Dienste.

<h3 id="create-from-the-web">
  Erstellen aus dem Web
</h3>

<Steps>
  <Step title="Öffnen Sie das Erstellungsformular">
    Besuchen Sie [claude.ai/code/routines](https://claude.ai/code/routines) und klicken Sie auf **Neue Routine**.
  </Step>

  <Step title="Benennen Sie die Routine und schreiben Sie den Prompt">
    Geben Sie der Routine einen aussagekräftigen Namen und schreiben Sie den Prompt, den Claude jedes Mal ausführt. Der Prompt ist der wichtigste Teil: Die Routine wird autonom ausgeführt, daher muss der Prompt in sich geschlossen und explizit darüber sein, was zu tun ist und wie Erfolg aussieht.

    Die Prompt-Eingabe enthält einen Modell-Selector. Claude verwendet das ausgewählte Modell bei jeder Ausführung.
  </Step>

  <Step title="Wählen Sie Repositories aus">
    Fügen Sie ein oder mehrere GitHub-Repositories hinzu, in denen Claude arbeiten kann. Jedes Repository wird zu Beginn einer Ausführung geklont, beginnend mit dem Standard-Branch. Claude erstellt `claude/`-prefixierte Branches für seine Änderungen.
  </Step>

  <Step title="Wählen Sie eine Umgebung aus">
    Wählen Sie eine [Cloud-Umgebung](/docs/de/claude-code-on-the-web#the-cloud-environment) für die Routine. Umgebungen steuern, worauf die Cloud-Sitzung Zugriff hat:

    * **Netzwerkzugriff**: Legen Sie die Stufe des Internet-Zugriffs fest, der während jeder Ausführung verfügbar ist
    * **Umgebungsvariablen**: Stellen Sie API-Schlüssel, Tokens oder andere Geheimnisse bereit, die Claude verwenden kann
    * **Setup-Skript**: Installieren Sie Abhängigkeiten und Tools, die die Routine benötigt. Das Ergebnis wird [zwischengespeichert](/docs/de/claude-code-on-the-web#environment-caching), sodass das Skript nicht bei jeder Sitzung erneut ausgeführt wird

    Eine **Standard**-Umgebung wird mit **Vertrauenswürdigem** Netzwerkzugriff bereitgestellt, der den [Standard-Satz](/docs/de/claude-code-on-the-web#default-allowed-domains) von Paket-Registries, Cloud-Provider-APIs, Container-Registries und häufigen Entwicklungsdomänen ermöglicht, aber alles andere blockiert. Wenn Ihre Routine Ihre eigenen Dienste oder eine Domain außerhalb dieser Liste erreichen muss, bearbeiten Sie den [Netzwerkzugriff](/docs/de/claude-code-on-the-web#network-access) der Umgebung vor der Ausführung. Um eine separate Umgebung zu verwenden, [erstellen Sie eine](/docs/de/claude-code-on-the-web#configure-your-environment) zuerst.
  </Step>

  <Step title="Wählen Sie einen Trigger aus">
    Wählen Sie unter **Trigger auswählen**, wie die Routine startet. Sie können einen Trigger-Typ auswählen oder mehrere kombinieren.

    <Tabs>
      <Tab title="Zeitplan">
        Wählen Sie eine voreingestellte Häufigkeit für eine wiederkehrende Ausführung, oder planen Sie eine einmalige Ausführung zu einem bestimmten Zeitstempel. Siehe [Zeitplan-Trigger hinzufügen](#add-a-schedule-trigger) für Zeitzonenbehandlung, Staffelung, benutzerdefinierte Cron-Intervalle und einmalige Ausführungen.
      </Tab>

      <Tab title="GitHub-Ereignis">
        Wählen Sie das Repository, das Ereignis, auf das reagiert werden soll, und optionale Filter aus. Siehe [GitHub-Trigger hinzufügen](#add-a-github-trigger) für die vollständige Liste der unterstützten Ereignisse und Filterfelder.
      </Tab>

      <Tab title="API">
        Wählen Sie hier **API** aus und speichern Sie dann die Routine. Die URL und der Token werden nach dem Speichern der Routine generiert, da sie von der Routine-ID abhängen. Siehe [API-Trigger hinzufügen](#add-an-api-trigger), um die URL zu kopieren und einen Token zu generieren.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Überprüfen Sie Konnektoren und Berechtigungen">
    Die Registerkarten **Konnektoren** und **Berechtigungen** am unteren Ende des Formulars steuern, worauf die Routine zugreifen kann.

    Unter Konnektoren sind alle Ihre verbundenen [MCP-Konnektoren](/docs/de/mcp) standardmäßig enthalten. Entfernen Sie alle, die die Routine nicht benötigt. Claude kann alle Tools aus einem eingebundenen Konnektor verwenden, einschließlich Schreibvorgänge, ohne während einer Ausführung um Genehmigung zu fragen.

    Unter Berechtigungen aktivieren Sie **Uneingeschränkte Branch-Pushes zulassen** für alle Repositories, in denen Claude zu bestehenden Branches pushen können soll, anstatt nur zu `claude/`-prefixierten.
  </Step>

  <Step title="Erstellen Sie die Routine">
    Klicken Sie auf **Erstellen**. Die Routine wird in der Liste angezeigt und wird das nächste Mal ausgeführt, wenn einer ihrer Trigger passt. Um eine Ausführung sofort zu starten, klicken Sie auf **Jetzt ausführen** auf der Detail-Seite der Routine.

    Jede Ausführung erstellt eine neue Sitzung neben Ihren anderen Sitzungen, in der Sie sehen können, was Claude getan hat, Änderungen überprüfen und einen Pull Request erstellen können.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Erstellen aus der CLI
</h3>

Führen Sie `/schedule` in einer beliebigen Sitzung aus, um eine geplante Routine im Gespräch zu erstellen. Sie können auch eine Beschreibung direkt übergeben, für eine wiederkehrende Routine wie `/schedule daily PR review at 9am` oder eine einmalige wie `/schedule clean up feature flag in one week`. Claude führt Sie durch die gleichen Informationen, die das Web-Formular sammelt, und speichert dann die Routine in Ihrem Konto.

Eine erfolgreiche Ausführung sieht wie ein Gespräch aus: Claude stellt Folgefragen zum Zeitplan, zu Repositories und zum Prompt, bevor die Routine gespeichert wird. Wenn Claude stattdessen antwortet, dass Sie sich authentifizieren müssen oder dass es keine Verbindung zu Ihrem Remote-claude.ai-Konto herstellen kann, wurde keine Routine erstellt; siehe [Fehlerbehebung](#troubleshooting).

`/schedule` in der CLI erstellt nur geplante Routinen. Um einen API- oder GitHub-Trigger hinzuzufügen, bearbeiten Sie die Routine im Web unter [claude.ai/code/routines](https://claude.ai/code/routines).

Die CLI unterstützt auch die Verwaltung vorhandener Routinen. Führen Sie `/schedule list` aus, um alle Routinen anzuzeigen, `/schedule update`, um eine zu ändern, oder `/schedule run`, um sie sofort auszulösen.

<h2 id="configure-triggers">
  Trigger konfigurieren
</h2>

Eine Routine startet, wenn einer ihrer Trigger passt. Sie können jede Kombination von Schedule-, API- und GitHub-Triggern an die gleiche Routine anhängen und sie jederzeit aus dem Abschnitt **Trigger auswählen** des Bearbeitungsformulars der Routine hinzufügen oder entfernen.

<h3 id="add-a-schedule-trigger">
  Schedule-Trigger hinzufügen
</h3>

Ein Schedule-Trigger führt die Routine nach einem wiederkehrenden Zeitplan oder einmalig zu einem bestimmten zukünftigen Zeitpunkt aus. Wählen Sie eine voreingestellte Häufigkeit im Abschnitt **Trigger auswählen**: stündlich, täglich, Wochentage oder wöchentlich. Zeiten werden in Ihrer lokalen Zone eingegeben und automatisch konvertiert, sodass die Routine zu dieser Wanduhr-Zeit unabhängig davon ausgeführt wird, wo sich die Cloud-Infrastruktur befindet.

Ausführungen können aufgrund von Staffelung einige Minuten nach der geplanten Zeit beginnen. Der Offset ist für jede Routine konsistent.

Für ein benutzerdefiniertes Intervall wie alle zwei Stunden oder den ersten jedes Monats wählen Sie die nächste Voreinstellung im Formular aus und führen dann `/schedule update` in der CLI aus, um einen spezifischen Cron-Ausdruck festzulegen. Das Mindestintervall beträgt eine Stunde; Ausdrücke, die häufiger ausgeführt werden, werden abgelehnt.

<h4 id="schedule-a-one-off-run">
  Einmalige Ausführung planen
</h4>

Ein einmaliger Schedule-Trigger führt die Routine zu einem bestimmten Zeitstempel aus. Verwenden Sie ihn, um sich später in der Woche selbst zu erinnern, um einen Cleanup-PR nach Abschluss eines Rollouts zu öffnen, oder um eine Folgeaufgabe zu starten, wenn eine vorgelagerte Änderung ankommt. Nach der Ausführung der Routine wird sie automatisch deaktiviert und die Web-UI markiert sie als **Ausgeführt**. Um sie erneut auszuführen, bearbeiten Sie die Routine und legen Sie einen neuen einmaligen Zeitpunkt fest.

<Note>
  Die einmalige Planung über die CLI wird schrittweise eingeführt und ist möglicherweise noch nicht auf Ihrem Konto verfügbar. Wenn `/schedule` nur wiederkehrende Zeitpläne anbietet, erstellen Sie die einmalige Ausführung stattdessen über das Web unter [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

Erstellen Sie eine einmalige Ausführung aus der CLI, indem Sie die Zeit in natürlicher Sprache beschreiben. Claude löst den Ausdruck gegen die aktuelle Zeit auf und bestätigt den absoluten Zeitstempel vor dem Speichern.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

Die gleiche lokale-zu-UTC-Konvertierung wie bei wiederkehrenden Schedules gilt auch für einmalige Zeitstempel.

Einmalige Ausführungen zählen nicht gegen das tägliche Routine-Ausführungs-Limit. Sie verbrauchen die reguläre Abonnement-Nutzung Ihres Plans wie jede andere Sitzung. Weitere Informationen finden Sie unter [Nutzung und Limits](#usage-and-limits).

<h3 id="add-an-api-trigger">
  API-Trigger hinzufügen
</h3>

Ein API-Trigger gibt einer Routine einen dedizierten HTTP-Endpunkt. Das Posten zum Endpunkt mit dem Bearer-Token der Routine startet eine neue Sitzung und gibt eine Sitzungs-URL zurück. Verwenden Sie dies, um Claude Code in Alerting-Systeme, Deploy-Pipelines, interne Tools oder überall dort zu integrieren, wo Sie eine authentifizierte HTTP-Anfrage stellen können.

API-Trigger werden einer vorhandenen Routine aus dem Web hinzugefügt. Die CLI kann derzeit keine Tokens erstellen oder widerrufen.

<Steps>
  <Step title="Öffnen Sie die Routine zur Bearbeitung">
    Gehen Sie zu [claude.ai/code/routines](https://claude.ai/code/routines), klicken Sie auf die Routine, die Sie über API auslösen möchten, und klicken Sie dann auf das Stiftsymbol, um **Routine bearbeiten** zu öffnen.
  </Step>

  <Step title="Fügen Sie einen API-Trigger hinzu">
    Scrollen Sie zum Abschnitt **Trigger auswählen** unter dem Feld **Anweisungen**, klicken Sie auf **Weiteren Trigger hinzufügen** und wählen Sie **API**.
  </Step>

  <Step title="Kopieren Sie die URL und generieren Sie einen Token">
    Das Modal zeigt die URL für diese Routine zusammen mit einem Beispiel-curl-Befehl. Kopieren Sie die URL, klicken Sie dann auf **Token generieren** und kopieren Sie den Token sofort. Der Token wird einmal angezeigt und kann später nicht abgerufen werden, daher speichern Sie ihn an einem sicheren Ort wie dem Secret Store Ihres Alerting-Tools.
  </Step>

  <Step title="Rufen Sie den Endpunkt auf">
    Senden Sie den Token im `Authorization: Bearer`-Header, wenn Sie zur URL posten. Der Abschnitt [Routine auslösen](#trigger-a-routine) unten zeigt ein vollständiges Beispiel.
  </Step>
</Steps>

Jede Routine hat ihren eigenen Token, der nur zum Auslösen dieser Routine begrenzt ist. Um ihn zu rotieren oder zu widerrufen, kehren Sie zum gleichen Modal zurück und klicken Sie auf **Neu generieren** oder **Widerrufen**.

<h4 id="trigger-a-routine">
  Routine auslösen
</h4>

Senden Sie eine POST-Anfrage an den `/fire`-Endpunkt mit dem Bearer-Token im `Authorization`-Header. Der Request-Body akzeptiert ein optionales `text`-Feld für Ausführungs-spezifischen Kontext wie einen Alert-Body oder ein fehlgeschlagenes Log, das der Routine zusammen mit ihrem gespeicherten Prompt übergeben wird. Der Wert ist freier Text und wird nicht geparst: Wenn Sie JSON oder eine andere strukturierte Payload senden, erhält die Routine sie als wörtliche Zeichenkette.

Das Beispiel unten löst eine Routine aus einer Shell aus. Die angezeigte Routine-ID und der Token sind Platzhalter: Ersetzen Sie sie mit der URL und dem Token, die Sie beim [Hinzufügen des API-Triggers](#add-an-api-trigger) kopiert haben, oder die Anfrage schlägt mit einem `401`-Authentifizierungsfehler fehl:

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

Eine erfolgreiche Anfrage gibt einen JSON-Body mit der neuen Sitzungs-ID und URL zurück:

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

Öffnen Sie die Sitzungs-URL in einem Browser, um die Ausführung in Echtzeit zu beobachten, Änderungen zu überprüfen oder das Gespräch manuell fortzusetzen.

<Warning>
  Der `/fire`-Endpunkt wird unter dem `experimental-cc-routine-2026-04-01`-Beta-Header ausgeliefert. Request- und Response-Formen, Rate Limits und Token-Semantik können sich ändern, während sich die Funktion in der Forschungsvorschau befindet. Breaking Changes werden hinter neuen datierten Beta-Header-Versionen ausgeliefert, und die zwei neuesten vorherigen Header-Versionen funktionieren weiterhin, damit Aufrufer Zeit zur Migration haben.
</Warning>

<h4 id="api-reference">
  API-Referenz
</h4>

Für die vollständige API-Referenz, einschließlich aller Error-Responses, Validierungsregeln und Feldlimits, siehe [Routine über API auslösen](https://platform.claude.com/docs/de/api/claude-code/routines-fire) in der Claude Platform-Dokumentation.

Der `/fire`-Endpunkt ist nur für claude.ai-Benutzer verfügbar und ist nicht Teil der Claude Platform API-Oberfläche.

<h3 id="add-a-github-trigger">
  GitHub-Trigger hinzufügen
</h3>

Ein GitHub-Trigger startet automatisch eine neue Sitzung, wenn ein passendes Ereignis in einem verbundenen Repository auftritt. Jedes passende Ereignis startet seine eigene Sitzung.

<Note>
  Während der Forschungsvorschau unterliegen GitHub-Webhook-Ereignisse pro-Routine und pro-Konto stündlichen Limits. Ereignisse über dem Limit werden gelöscht, bis sich das Fenster zurückgesetzt hat. Sehen Sie Ihre aktuellen Limits unter [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

GitHub-Trigger werden nur über die Web-UI konfiguriert.

<Steps>
  <Step title="Öffnen Sie die Routine zur Bearbeitung">
    Gehen Sie zu [claude.ai/code/routines](https://claude.ai/code/routines), klicken Sie auf die Routine und klicken Sie dann auf das Stiftsymbol, um **Routine bearbeiten** zu öffnen.
  </Step>

  <Step title="Fügen Sie einen GitHub-Ereignis-Trigger hinzu">
    Scrollen Sie zum Abschnitt **Trigger auswählen**, klicken Sie auf **Weiteren Trigger hinzufügen** und wählen Sie **GitHub-Ereignis**.
  </Step>

  <Step title="Installieren Sie die Claude GitHub App">
    Die Claude GitHub App muss auf dem Repository installiert sein, das Sie abonnieren möchten. Das Trigger-Setup fordert Sie auf, es zu installieren, falls es nicht bereits installiert ist.

    <Note>
      Das Ausführen von `/web-setup` in der CLI gewährt Repository-Zugriff zum Klonen, installiert aber nicht die Claude GitHub App und aktiviert nicht die Webhook-Bereitstellung. GitHub-Trigger erfordern die Installation der Claude GitHub App, die das Trigger-Setup Sie auffordert zu tun.
    </Note>
  </Step>

  <Step title="Konfigurieren Sie den Trigger">
    Wählen Sie das Repository, wählen Sie ein Ereignis aus der Liste [Unterstützte Ereignisse](#supported-events) und fügen Sie optional Filter hinzu. Speichern Sie den Trigger.
  </Step>
</Steps>

<h4 id="supported-events">
  Unterstützte Ereignisse
</h4>

GitHub-Trigger können sich auf eine der folgenden Ereigniskategorien abonnieren. Innerhalb jeder Kategorie können Sie eine spezifische Aktion wie `pull_request.opened` auswählen oder auf alle Aktionen in der Kategorie reagieren.

| Ereignis     | Wird ausgelöst, wenn                                                                                        |
| :----------- | :---------------------------------------------------------------------------------------------------------- |
| Pull Request | Ein PR wird geöffnet, geschlossen, zugewiesen, gekennzeichnet, synchronisiert oder anderweitig aktualisiert |
| Release      | Ein Release wird erstellt, veröffentlicht, bearbeitet oder gelöscht                                         |

<h4 id="filter-pull-requests">
  Pull Requests filtern
</h4>

Verwenden Sie Filter, um einzugrenzen, welche Pull Requests eine neue Sitzung starten. Alle Filterbedingungen müssen übereinstimmen, damit die Routine ausgelöst wird. Die verfügbaren Filterfelder sind:

| Filter              | Passt auf                         |
| :------------------ | :-------------------------------- |
| Autor               | GitHub-Benutzername des PR-Autors |
| Titel               | PR-Titeltext                      |
| Body                | PR-Beschreibungstext              |
| Base-Branch         | Branch, auf den der PR abzielt    |
| Head-Branch         | Branch, von dem der PR kommt      |
| Labels              | Auf den PR angewendete Labels     |
| Ist Entwurf         | Ob der PR im Entwurfszustand ist  |
| Ist zusammengeführt | Ob der PR zusammengeführt wurde   |

Jeder Filter kombiniert ein Feld mit einem Operator: gleich, enthält, beginnt mit, ist einer von, ist nicht einer von oder passt Regex.

Der Operator `matches regex` testet den gesamten Feldwert, nicht eine Teilzeichenkette darin. Um einen Titel zu finden, der `hotfix` enthält, schreiben Sie `.*hotfix.*`. Ohne die umgebenden `.*` passt der Filter nur auf einen Titel, der genau `hotfix` ist, ohne etwas davor oder danach. Für wörtliches Substring-Matching ohne Regex-Syntax verwenden Sie stattdessen den Operator `contains`.

Ein paar Beispiel-Filterkombinationen:

* **Auth-Modul-Review**: Base-Branch `main`, Head-Branch enthält `auth-provider`. Sendet jeden PR, der Authentifizierung berührt, an einen fokussierten Reviewer.
* **Nur bereit zur Überprüfung**: Ist Entwurf ist `false`. Überspringt Entwürfe, sodass die Routine nur ausgeführt wird, wenn der PR zur Überprüfung bereit ist.
* **Label-gesteuerter Backport**: Labels enthalten `needs-backport`. Löst eine Port-zu-anderem-Branch-Routine nur aus, wenn ein Maintainer den PR kennzeichnet.

<h4 id="how-sessions-map-to-events">
  Wie Sitzungen zu Ereignissen zugeordnet werden
</h4>

Jedes passende GitHub-Ereignis startet eine neue Sitzung. Sitzungswiederverwendung über Ereignisse hinweg ist nicht für GitHub-ausgelöste Routinen verfügbar, daher produzieren zwei PR-Updates zwei unabhängige Sitzungen.

<h2 id="manage-routines">
  Routinen verwalten
</h2>

Klicken Sie auf eine Routine in der Liste, um ihre Detailseite zu öffnen. Die Detailseite zeigt die Repositories der Routine, Konnektoren, Prompt, Schedule, API-Tokens, GitHub-Trigger und eine Liste vergangener Ausführungen.

<h3 id="view-and-interact-with-runs">
  Ausführungen anzeigen und mit ihnen interagieren
</h3>

Klicken Sie auf eine beliebige Ausführung, um sie als vollständige Sitzung zu öffnen. Von dort aus können Sie sehen, was Claude getan hat, Änderungen überprüfen, einen Pull Request erstellen oder das Gespräch fortsetzen. Jede Ausführungssitzung funktioniert wie jede andere Sitzung: Verwenden Sie das Dropdown-Menü neben dem Sitzungstitel, um sie umzubenennen, zu archivieren oder zu löschen.

<Note>
  Ein grüner Status in der Ausführungsliste bedeutet, dass die Sitzung gestartet und beendet wurde, ohne dass ein Infrastrukturfehler auftrat. Dies bedeutet nicht, dass die Aufgabe in Ihrem Prompt erfolgreich war. Öffnen Sie die Ausführung, um das Transkript zu lesen und zu bestätigen, was Claude tatsächlich getan hat. Blockierte Netzwerkanfragen, fehlende Konnektoren-Tools und Fehler auf Aufgabenebene werden dort angezeigt, anstatt im Status-Indikator.
</Note>

<h3 id="edit-and-control-routines">
  Routinen bearbeiten und steuern
</h3>

Von der Routine-Detailseite können Sie:

* Auf **Jetzt ausführen** klicken, um eine Ausführung sofort zu starten, ohne auf die nächste geplante Zeit zu warten.
* Den Toggle im Abschnitt **Wiederholt** verwenden, um den Schedule zu pausieren oder fortzusetzen. Pausierte Routinen behalten ihre Konfiguration, werden aber nicht ausgeführt, bis Sie sie erneut aktivieren.
* Auf das Stiftsymbol klicken, um **Routine bearbeiten** zu öffnen und den Namen, Prompt, Repositories, Umgebung, Konnektoren oder einen der Trigger der Routine zu ändern. Der Abschnitt **Trigger auswählen** ist der Ort, an dem Sie Schedules, API-Tokens und GitHub-Ereignis-Trigger hinzufügen oder entfernen.
* Auf das Löschsymbol klicken, um die Routine zu entfernen. Vergangene Sitzungen, die von der Routine erstellt wurden, bleiben in Ihrer Sitzungsliste.

<h3 id="repositories-and-branch-permissions">
  Repositories und Branch-Berechtigungen
</h3>

Routinen benötigen GitHub-Zugriff zum Klonen von Repositories. Wenn Sie eine Routine aus der CLI mit `/schedule` erstellen, überprüft Claude, ob Ihr Konto GitHub verbunden hat, und fordert Sie auf, `/web-setup` auszuführen, falls nicht. Siehe [GitHub-Authentifizierungsoptionen](/docs/de/claude-code-on-the-web#github-authentication-options) für die zwei Möglichkeiten, Zugriff zu gewähren.

Jedes Repository, das Sie hinzufügen, wird bei jeder Ausführung geklont. Claude startet vom Standard-Branch des Repositories, es sei denn, Ihr Prompt gibt etwas anderes an.

Standardmäßig kann Claude nur zu Branches mit dem Präfix `claude/` pushen. Dies verhindert, dass Routinen versehentlich geschützte oder langlebige Branches ändern. Um diese Einschränkung für ein bestimmtes Repository zu entfernen, aktivieren Sie **Uneingeschränkte Branch-Pushes zulassen** für dieses Repository beim Erstellen oder Bearbeiten der Routine.

<h3 id="connectors">
  Konnektoren
</h3>

Routinen können Ihre verbundenen MCP-Konnektoren verwenden, um während jeder Ausführung von externen Diensten zu lesen und zu schreiben. Beispielsweise könnte eine Routine, die Support-Anfragen triagiert, aus einem Slack-Channel lesen und Issues in Linear erstellen.

Konnektoren sind die [claude.ai-Integrationen](/docs/de/mcp#use-mcp-servers-from-claude-ai) auf Ihrem Konto. MCP-Server, die Sie lokal in der CLI mit `claude mcp add` hinzugefügt haben, werden auf Ihrem Computer gespeichert, anstatt auf Ihrem claude.ai-Konto, daher werden sie nicht in der Konnektoren-Liste angezeigt. Um einen dieser Server in einer Routine zu verwenden, fügen Sie ihn als Konnektoren unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors) hinzu, oder deklarieren Sie ihn in einer committed [`.mcp.json`](/docs/de/mcp#project-scope), damit er Teil des geklonten Repositories ist.

Wenn Sie eine Routine erstellen, sind alle Ihre derzeit verbundenen Konnektoren standardmäßig enthalten. Entfernen Sie alle, die nicht benötigt werden, um zu begrenzen, auf welche Tools Claude während der Ausführung Zugriff hat. Sie können auch Konnektoren direkt aus dem Routine-Formular hinzufügen.

Um Konnektoren außerhalb des Routine-Formulars zu verwalten oder hinzuzufügen, besuchen Sie **Einstellungen > Konnektoren** auf claude.ai oder verwenden Sie `/schedule update` in der CLI.

<h3 id="environments-and-network-access">
  Umgebungen und Netzwerkzugriff
</h3>

Jede Routine wird in einer [Cloud-Umgebung](/docs/de/claude-code-on-the-web#the-cloud-environment) ausgeführt, die Netzwerkzugriff, Umgebungsvariablen und Setup-Skripte steuert. Die Routine erbt die Netzwerk-Richtlinie der Umgebung bei jeder Ausführung.

Die **Standard**-Umgebung verwendet **Vertrauenswürdigen** Netzwerkzugriff: Die [Standard-Zulassungsliste](/docs/de/claude-code-on-the-web#default-allowed-domains) von Paket-Registries, Cloud-Provider-APIs, Container-Registries und häufigen Entwicklungs-Domains ist erreichbar, aber beliebige Domains sind nicht. Ausgehende Anfragen an andere Hosts schlagen mit `403` und `x-deny-reason: host_not_allowed` fehl. MCP-Konnektoren-Datenverkehr wird über Anthropic-Server geleitet, daher funktionieren die Konnektoren, die Sie der Routine hinzufügen, ohne dass Sie ihre Hosts zu **Zulässige Domains** hinzufügen müssen. Entfernen Sie alle Konnektoren, die Sie nicht benötigen, unter [Konnektoren](#connectors).

Um zusätzliche Domains zuzulassen:

<Steps>
  <Step title="Öffnen Sie die Routine zur Bearbeitung">
    Klicken Sie auf der Detailseite der Routine auf das Stiftsymbol, um **Routine bearbeiten** zu öffnen.
  </Step>

  <Step title="Öffnen Sie die Umgebungsauswahl">
    Wählen Sie unter dem Feld **Anweisungen** das Cloud-Symbol aus, das den Namen Ihrer Umgebung anzeigt, z. B. **Standard**.
  </Step>

  <Step title="Öffnen Sie die Umgebungseinstellungen">
    Bewegen Sie den Mauszeiger über die Umgebung in der Liste und klicken Sie auf das Einstellungssymbol, das auf der rechten Seite angezeigt wird.
  </Step>

  <Step title="Ändern Sie die Netzwerkzugriff-Ebene">
    Ändern Sie im Dialog **Cloud-Umgebung aktualisieren** den **Netzwerkzugriff** zu **Benutzerdefiniert** und geben Sie Ihre Domains in **Zulässige Domains** ein. Aktivieren Sie **Auch Standard-Liste häufiger Paket-Manager einschließen**, um die [Standard-Zulassungsliste](/docs/de/claude-code-on-the-web#default-allowed-domains) neben Ihren benutzerdefinierten Domains zu behalten. Wählen Sie stattdessen **Vollständig** für uneingeschränkten Zugriff.
  </Step>

  <Step title="Speichern">
    Klicken Sie auf **Änderungen speichern**. Die neue Richtlinie wird ab der nächsten Ausführung angewendet.
  </Step>
</Steps>

Siehe [Netzwerkzugriff](/docs/de/claude-code-on-the-web#network-access) für Details zu Zugriffsstufen und der Standard-Zulassungsliste.

<h2 id="usage-and-limits">
  Nutzung und Limits
</h2>

Routinen verbrauchen Abonnement-Nutzung auf die gleiche Weise wie interaktive Sitzungen. Zusätzlich zu den Standard-Abonnement-Limits haben Routinen eine tägliche Obergrenze für die Anzahl der Ausführungen, die pro Konto starten können. Sehen Sie Ihren aktuellen Verbrauch und verbleibende tägliche Routine-Ausführungen unter [claude.ai/code/routines](https://claude.ai/code/routines) oder [claude.ai/settings/usage](https://claude.ai/settings/usage).

Wenn eine Routine das tägliche Limit oder Ihr Abonnement-Nutzungslimit erreicht, können Organisationen mit aktivierter Nutzungsguthaben Routinen weiterhin auf gemessener Überschreitung ausführen. Ohne Nutzungsguthaben werden weitere Ausführungen abgelehnt, bis sich das Fenster zurückgesetzt hat. Aktivieren Sie Nutzungsguthaben unter **Einstellungen > Abrechnung** auf claude.ai.

Einmalige Ausführungen werden nicht auf das tägliche Routine-Ausführungslimit angerechnet. Sie verbrauchen Ihre reguläre Abonnement-Nutzung wie jede andere Sitzung, sind aber von der täglichen Routine-Ausführungszulage pro Konto ausgenommen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="/schedule-returns-unknown-command">
  `/schedule` zeigt "Unknown command" an
</h3>

Die CLI blendet `/schedule` aus, wenn eine ihrer Anforderungen nicht erfüllt ist: Das Befehlsmenü zeigt `No commands match "/schedule"` während der Eingabe an, und das Absenden gibt `Unknown command: /schedule` zurück. Die Ursache ist normalerweise eine der folgenden:

* Sie sind mit einem Console-API-Schlüssel oder einem Cloud-Anbieter wie Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry authentifiziert. `/schedule` erfordert eine claude.ai-Abonnement-Anmeldung. Wenn `ANTHROPIC_API_KEY` oder `ANTHROPIC_AUTH_TOKEN` in Ihrer Shell oder `apiKeyHelper` in `settings.json` gesetzt ist, entfernen Sie es zuerst, da diese Vorrang vor einer claude.ai-Anmeldung haben
* `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` oder `DISABLE_GROWTHBOOK` ist in Ihrer Shell-Umgebung oder im `env`-Block einer [`settings.json`-Datei](/docs/de/settings#available-settings) gesetzt. Diese deaktivieren das Abrufen von Feature-Flags, auf das `/schedule` angewiesen ist
* Sie befinden sich in einer Claude Code-Sitzung im Web. Verwalten Sie Routinen stattdessen über die [Web-Benutzeroberfläche](https://claude.ai/code/routines)

Sie können Routinen jederzeit unter [claude.ai/code/routines](https://claude.ai/code/routines) erstellen und verwalten, unabhängig davon, wie die CLI konfiguriert ist.

<h3 id="/schedule-asks-you-to-authenticate">
  `/schedule` fordert Sie auf, sich zu authentifizieren
</h3>

Wenn `/schedule` ausgeführt wird, aber Claude antwortet, dass Sie sich zuerst mit einem claude.ai-Konto authentifizieren müssen, hat die CLI keine gespeicherte claude.ai-Anmeldung. API-Konten werden für Routinen nicht unterstützt. Führen Sie `/login` aus, melden Sie sich mit Ihrem claude.ai-Konto an, und führen Sie dann `/schedule` erneut aus.

<h3 id="routines-are-disabled-by-your-organization’s-policy">
  "Routinen sind durch die Richtlinie Ihrer Organisation deaktiviert"
</h3>

Ein Inhaber in Ihrer Team- oder Enterprise-Organisation hat wahrscheinlich den **Routinen**-Schalter unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) ausgeschaltet. Dies ist eine serverseitige Organisationseinstellung, daher kann sie nicht aus Ihrer lokalen Konfiguration überschrieben werden. Bitten Sie einen Inhaber, Routinen für Ihre Organisation zu aktivieren.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [`/loop` und In-Session-Planung](/docs/de/scheduled-tasks): Planen Sie lokale Aufgaben innerhalb einer offenen CLI-Sitzung
* [Desktop-geplante Aufgaben](/docs/de/desktop-scheduled-tasks): Lokale geplante Aufgaben, die auf Ihrem Computer mit Zugriff auf lokale Dateien ausgeführt werden
* [Cloud-Umgebung](/docs/de/claude-code-on-the-web#the-cloud-environment): Konfigurieren Sie die Laufzeitumgebung für Cloud-Sitzungen
* [MCP-Konnektoren](/docs/de/mcp): Verbinden Sie externe Dienste wie Slack, Linear und Google Drive
* [GitHub Actions](/docs/de/github-actions): Führen Sie Claude in Ihrer CI-Pipeline bei Repository-Ereignissen aus
