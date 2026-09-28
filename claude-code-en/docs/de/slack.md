> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code in Slack

> Delegieren Sie Codierungsaufgaben direkt aus Ihrem Slack-Arbeitsbereich. Anthropic stellt diese frühere Version für Team- und Enterprise-Arbeitsbereiche zugunsten von Claude Tag ein; sie bleibt der Einrichtungspfad für Pro- und Max-Pläne.

<Warning>
  Diese Seite dokumentiert die frühere Claude Code in Slack, die jede Sitzung unter dem Konto eines einzelnen Benutzers ausführt.

  * **Team- und Enterprise-Pläne:** Anthropic stellt diese Version zugunsten von [Claude Tag](https://claude.com/product/tag) ein, das @Claude als die gemeinsame Identität Ihrer Organisation mit vom Administrator konfiguriertem Zugriff ausführt. Ihre bestehende Slack-App und @Claude-Handle bleiben erhalten, und Ihr Anthropic-Kontoteam kann Ihnen das Umstellungsdatum mitteilen. [Richten Sie Claude Tag ein](https://claude.com/docs/claude-tag/overview) für einen neuen Arbeitsbereich ein; um einen zu verschieben, der bereits diese Version verwendet, siehe [Migration von der früheren Claude in Slack](https://claude.com/docs/claude-tag/admins/migrate-from-earlier).
  * **Pro- und Max-Pläne:** Claude Tag ist nicht auf Einzelplänen verfügbar, daher bleibt diese Seite der Einrichtungspfad.
</Warning>

Claude Code in Slack bringt die Leistung von Claude Code direkt in Ihren Slack-Arbeitsbereich. Wenn Sie `@Claude` mit einer Codierungsaufgabe erwähnen, erkennt Claude automatisch die Absicht und erstellt eine Claude Code-Sitzung im Web, sodass Sie Entwicklungsarbeiten delegieren können, ohne Ihre Team-Gespräche zu verlassen.

Diese Integration basiert auf der bestehenden Claude for Slack-App, fügt aber intelligentes Routing zu Claude Code im Web für codierungsbezogene Anfragen hinzu. Jede Sitzung wird unter Ihrem eigenen Claude-Konto ausgeführt und nutzt Ihre verbundenen Repositories und Ihre Plan-Limits.

<h2 id="use-cases">
  Anwendungsfälle
</h2>

* **Fehleruntersuchung und -behebung**: Bitten Sie Claude, Fehler zu untersuchen und zu beheben, sobald sie in Slack-Kanälen gemeldet werden.
* **Schnelle Code-Reviews und Änderungen**: Lassen Sie Claude kleine Funktionen implementieren oder Code basierend auf Team-Feedback umgestalten.
* **Kollaboratives Debugging**: Wenn Team-Diskussionen wichtigen Kontext bieten (z. B. Fehlerreproduzierungen oder Benutzerberichte), kann Claude diese Informationen nutzen, um seinen Debugging-Ansatz zu informieren.
* **Parallele Aufgabenausführung**: Starten Sie Codierungsaufgaben in Slack, während Sie andere Arbeiten fortsetzen, und erhalten Sie Benachrichtigungen nach Abschluss.

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Bevor Sie Claude Code in Slack verwenden, stellen Sie sicher, dass Sie über Folgendes verfügen:

| Anforderung             | Details                                                                                                  |
| :---------------------- | :------------------------------------------------------------------------------------------------------- |
| Claude Plan             | Pro, Max, Team oder Enterprise mit Claude Code-Zugriff (Premium-Plätze oder Chat + Claude Code-Plätze)   |
| Cloud-Sitzungen         | [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) sind für Ihr Konto aktiviert                               |
| GitHub-Konto            | Mit [claude.ai/code](https://claude.ai/code) verbunden mit mindestens einem authentifizierten Repository |
| Slack-Authentifizierung | Ihr Slack-Konto ist über die Claude-App mit Ihrem Claude-Konto verknüpft                                 |

<h2 id="setting-up-claude-code-in-slack">
  Einrichten von Claude Code in Slack
</h2>

<Steps>
  <Step title="Installieren Sie die Claude-App in Slack">
    Ein Workspace-Administrator muss die Claude-App aus dem Slack App Marketplace installieren. Besuchen Sie den [Slack App Marketplace](https://slack.com/marketplace/A08SF47R6P4) und klicken Sie auf „Zu Slack hinzufügen", um den Installationsprozess zu starten.
  </Step>

  <Step title="Verbinden Sie Ihr Claude-Konto">
    Nach der Installation der App authentifizieren Sie Ihr individuelles Claude-Konto:

    1. Öffnen Sie die Claude-App in Slack, indem Sie auf „Claude" in Ihrem Apps-Bereich klicken
    2. Öffnen Sie die Registerkarte „App Home"
    3. Klicken Sie auf „Verbinden", um Ihr Slack-Konto mit Ihrem Claude-Konto zu verknüpfen
    4. Schließen Sie den Authentifizierungsfluss in Ihrem Browser ab
  </Step>

  <Step title="Konfigurieren Sie Cloud-Sitzungen">
    Stellen Sie sicher, dass Cloud-Sitzungen ordnungsgemäß für Ihr Konto konfiguriert sind:

    * Besuchen Sie [claude.ai/code](https://claude.ai/code) und melden Sie sich mit dem gleichen Konto an, das Sie mit Slack verbunden haben
    * Verbinden Sie Ihr GitHub-Konto, falls noch nicht geschehen
    * Authentifizieren Sie mindestens ein Repository, mit dem Claude arbeiten soll
  </Step>

  <Step title="Wählen Sie Ihren Routing-Modus">
    Nach dem Verbinden Ihrer Konten konfigurieren Sie, wie Claude Ihre Nachrichten in Slack verarbeitet. Öffnen Sie die Claude App Home in Slack, um die Einstellung **Routing Mode** zu finden.

    | Modus           | Verhalten                                                                                                                                                                                                                                                                        |
    | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | **Nur Code**    | Claude leitet alle @mentions zu Claude Code-Sitzungen weiter. Am besten für Teams, die Claude in Slack ausschließlich für Entwicklungsaufgaben verwenden.                                                                                                                        |
    | **Code + Chat** | Claude analysiert jede Nachricht und leitet intelligent zwischen Claude Code (für Codierungsaufgaben) und Claude Chat (für Schreiben, Analyse und allgemeine Fragen) weiter. Am besten für Teams, die einen einzigen @Claude-Einstiegspunkt für alle Arten von Arbeiten möchten. |

    <Note>
      Im Code + Chat-Modus können Sie, wenn Claude eine Nachricht zu Chat leitet, aber Sie wollten eine Codierungssitzung, auf „Als Code erneut versuchen" klicken, um stattdessen eine Claude Code-Sitzung zu erstellen. Wenn es zu Code weitergeleitet wird, aber Sie wollten eine Chat-Sitzung, können Sie diese Option in diesem Thread wählen.
    </Note>
  </Step>

  <Step title="Fügen Sie Claude zu Kanälen hinzu">
    Claude wird nach der Installation nicht automatisch zu Kanälen hinzugefügt. Um Claude in einem Kanal zu verwenden, laden Sie ihn ein, indem Sie `/invite @Claude` in diesem Kanal eingeben. Claude kann nur auf @mentions in Kanälen antworten, zu denen es hinzugefügt wurde.
  </Step>
</Steps>

<h2 id="how-it-works">
  Wie es funktioniert
</h2>

<h3 id="automatic-detection">
  Automatische Erkennung
</h3>

Im Code + Chat-Routing-Modus erkennt Claude automatisch, ob Ihre Nachricht eine Codierungsaufgabe ist, wenn Sie @Claude in einem Slack-Kanal oder Thread erwähnen. Codierungsaufgaben werden zu einer Claude Code Cloud-Sitzung weitergeleitet. Alles andere erhält eine reguläre Chat-Antwort. Im Code-only-Modus wird jede @Erwähnung zu Claude Code weitergeleitet.

Sie können Claude auch explizit anweisen, eine Anfrage als Codierungsaufgabe zu behandeln, auch wenn es diese nicht automatisch erkennt.

<Note>
  Claude Code in Slack funktioniert nur in Kanälen (öffentlich oder privat). Es funktioniert nicht in direkten Nachrichten (DMs).
</Note>

<h3 id="context-gathering">
  Kontexterfassung
</h3>

**Aus Threads**: Wenn Sie Claude in einem Thread @erwähnen, sammelt es Kontext aus allen Nachrichten in diesem Thread, um das vollständige Gespräch zu verstehen.

**Aus Kanälen**: Wenn es direkt in einem Kanal erwähnt wird, schaut Claude sich aktuelle Kanalnachrichten auf relevanten Kontext an.

Dieser Kontext hilft Claude, das Problem zu verstehen, das entsprechende Repository auszuwählen und seinen Ansatz zur Aufgabe zu informieren.

<Warning>
  Wenn @Claude in Slack aufgerufen wird, erhält Claude Zugriff auf den Gesprächskontext, um Ihre Anfrage besser zu verstehen. Claude kann Anweisungen aus anderen Nachrichten im Kontext befolgen, daher sollten Benutzer sicherstellen, dass sie Claude nur in vertrauenswürdigen Slack-Gesprächen verwenden.
</Warning>

<h3 id="session-flow">
  Sitzungsfluss
</h3>

1. **Initiierung**: Sie @erwähnen Claude mit einer Codierungsanfrage
2. **Erkennung**: Claude analysiert Ihre Nachricht und erkennt Codierungsabsicht
3. **Sitzungserstellung**: Eine neue Claude Code-Sitzung wird auf claude.ai/code erstellt
4. **Fortschritts-Updates**: Claude veröffentlicht Status-Updates in Ihrem Slack-Thread, während die Arbeit fortschreitet
5. **Abschluss**: Nach Abschluss @erwähnt Claude Sie mit einer Zusammenfassung und Aktionsschaltflächen
6. **Überprüfung**: Klicken Sie auf „Sitzung anzeigen", um das vollständige Transkript zu sehen, oder auf „PR erstellen", um einen Pull Request zu öffnen

<h2 id="user-interface-elements">
  Benutzeroberflächenelemente
</h2>

<h3 id="message-actions">
  Nachrichtenaktionen
</h3>

* **Sitzung anzeigen**: Öffnet die vollständige Claude Code-Sitzung in Ihrem Browser, wo Sie alle durchgeführten Arbeiten sehen, die Sitzung fortsetzen oder zusätzliche Anfragen stellen können.
* **PR erstellen**: Erstellt einen Pull Request direkt aus den Änderungen der Sitzung.
* **Als Code erneut versuchen**: Wenn Claude zunächst als Chat-Assistent antwortet, aber Sie wollten eine Codierungssitzung, klicken Sie auf diese Schaltfläche, um die Anfrage als Claude Code-Aufgabe erneut zu versuchen.
* **Repository ändern**: Ermöglicht es Ihnen, ein anderes Repository auszuwählen, wenn Claude falsch gewählt hat.

<h3 id="repository-selection">
  Repository-Auswahl
</h3>

Claude wählt automatisch ein Repository basierend auf dem Kontext aus Ihrem Slack-Gespräch aus. Wenn mehrere Repositories zutreffen könnten, zeigt Claude möglicherweise ein Dropdown-Menü an, mit dem Sie das richtige auswählen können.

<h2 id="access-and-permissions">
  Zugriff und Berechtigungen
</h2>

<h3 id="user-level-access">
  Zugriff auf Benutzerebene
</h3>

| Zugriffstyp           | Anforderung                                                                        |
| :-------------------- | :--------------------------------------------------------------------------------- |
| Claude Code-Sitzungen | Jeder Benutzer führt Sitzungen unter seinem eigenen Claude-Konto aus               |
| Nutzung & Ratenlimits | Sitzungen werden gegen die individuellen Plan-Limits des Benutzers angerechnet     |
| Repository-Zugriff    | Benutzer können nur auf Repositories zugreifen, die sie persönlich verbunden haben |
| Sitzungsverlauf       | Sitzungen erscheinen in Ihrem Claude Code-Verlauf auf claude.ai/code               |

<h3 id="workspace-level-access">
  Zugriff auf Workspace-Ebene
</h3>

Slack-Workspace-Administratoren kontrollieren, ob die Claude-App in ihrem Workspace verfügbar ist:

| Kontrolle                  | Beschreibung                                                                                                                             |
| :------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| App-Installation           | Workspace-Administratoren entscheiden, ob die Claude-App aus dem Slack App Marketplace installiert wird                                  |
| Enterprise Grid-Verteilung | Für Enterprise Grid-Organisationen können Organisationsadministratoren kontrollieren, welche Workspaces Zugriff auf die Claude-App haben |
| App-Entfernung             | Das Entfernen der App aus einem Workspace entzieht sofort allen Benutzern in diesem Workspace den Zugriff                                |

<h3 id="channel-based-access-control">
  Kanalbasierte Zugriffskontrolle
</h3>

Die Installation der App fügt Claude nicht automatisch zu Kanälen hinzu. Claude antwortet auf @mentions nur in Kanälen, zu denen es hinzugefügt wurde; laden Sie es mit `/invite @Claude` ein. Es funktioniert in öffentlichen und privaten Kanälen. Administratoren können die Nutzung von Claude Code kontrollieren, indem sie verwalten, zu welchen Kanälen Claude eingeladen wird und wer Zugriff auf diese Kanäle hat. Dies bietet eine zusätzliche Ebene der Zugriffskontrolle über Workspace-Berechtigungen hinaus.

<h2 id="what’s-accessible-where">
  Was wo zugänglich ist
</h2>

**In Slack**: Sie sehen Status-Updates, Abschluss-Zusammenfassungen und Aktionsschaltflächen. Das vollständige Transkript wird beibehalten und ist immer zugänglich.

**Bei claude.ai/code**: Die vollständige Claude Code-Sitzung mit vollständigem Gesprächsverlauf, alle Code-Änderungen und Dateivorgänge. Sitzungen bleiben in Ihrem Claude Code-Verlauf unter [claude.ai/code](https://claude.ai/code), wo Sie vergangene Sitzungen fortsetzen, auf sie verweisen oder Pull Requests erstellen können.

Für Enterprise- und Team-Konten sind Sitzungen, die von Claude in Slack erstellt werden, automatisch für die Organisation sichtbar. Weitere Informationen finden Sie unter [Cloud-Sitzungsfreigabe](/docs/de/claude-code-on-the-web#share-sessions).

<h2 id="best-practices">
  Best practices
</h2>

<h3 id="writing-effective-requests">
  Writing effective requests
</h3>

* **Be specific**: Include file names, function names, or error messages when relevant.
* **Provide context**: Mention the repository or project if it's not clear from the conversation.
* **Define success**: Explain what "done" looks like. Should Claude write tests? Update documentation? Create a PR?
* **Use threads**: Reply in threads when discussing bugs or features so Claude can gather the full context.

<h3 id="when-to-use-slack-vs-web">
  When to use Slack vs. web
</h3>

**Use Slack when**: Context already exists in a Slack discussion, you want to start a task asynchronously, or you're collaborating with teammates who need visibility.

**Use the web directly when**: You need to upload files, want real-time interaction during development, or are working on longer, more complex tasks.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  „Claude Code ist für Ihr Konto nicht aktiviert"
</h3>

Dieser Fehler bedeutet, dass Ihr Claude-Konto noch keine Cloud-Umgebung hat. Melden Sie sich unter [claude.ai/code](https://claude.ai/code) einmal mit demselben Konto an, das Sie mit Slack verbunden haben, und schließen Sie das [Web-Onboarding](/docs/de/web-quickstart#connect-github) ab, das Ihre Standard-Cloud-Umgebung erstellt oder Sie auffordert, eine zu erstellen. Der Fehler wird bei Ihrer nächsten Erwähnung behoben. Jeder Benutzer muss dies einzeln durchführen.

<h3 id="sessions-not-starting">
  Sitzungen starten nicht
</h3>

1. Überprüfen Sie, ob Ihr Claude-Konto in der Claude App Home verbunden ist
2. Überprüfen Sie, ob Cloud-Sitzungen für Ihr Konto aktiviert sind
3. Stellen Sie sicher, dass Sie mindestens ein GitHub-Repository mit Claude Code verbunden haben

<h3 id="sessions-from-a-claude-tag-channel-fail-to-start">
  Sitzungen aus einem Claude Tag-Kanal können nicht gestartet werden
</h3>

Dieser Eintrag gilt für Arbeitsbereiche, die [Claude Tag](https://claude.com/docs/claude-tag/overview) verwenden, wobei Claude in Kanälen als gemeinsame Identität Ihrer Organisation arbeitet, nicht als Konto eines Mitglieds. Wenn Sie die Cloud-Umgebung des Kanals unter [claude.ai/code](https://claude.ai/code) erstellt haben, gehört sie zu Ihrem persönlichen Konto, und Claude kann keine Kanalsitzungen in einer persönlichen Umgebung starten. Claude Code schlägt die Sitzung sofort fehl, und ein erneuter Versuch hilft nicht.

Wenn Sie ein Besitzer sind und die Umgebung Ihre eigene ist, [teilen Sie sie mit der Organisation](/docs/de/cloud-environments#organization-shared-environments) über die Umgebungsauswahl. Andernfalls erstellt ein Besitzer sie als organisationsfreigegeben Umgebung auf der Seite **Cloud-Umgebungen** in den [Admin-Einstellungen](https://claude.ai/admin-settings) neu.

Sie können sie auf zwei Arten anwenden:

* Legen Sie sie als Organisationsstandardwert unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) fest.
* [Legen Sie sie im Kanal](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) in den Claude Tag-Admin-Einstellungen fest.

Wenn Sie kein Besitzer sind, senden Sie diesen Eintrag an einen.

<h3 id="repository-not-showing">
  Repository wird nicht angezeigt
</h3>

1. Verbinden Sie das Repository unter [claude.ai/code](https://claude.ai/code)
2. Überprüfen Sie Ihre GitHub-Berechtigungen für dieses Repository
3. Versuchen Sie, Ihr GitHub-Konto zu trennen und erneut zu verbinden

<h3 id="wrong-repository-selected">
  Falsches Repository ausgewählt
</h3>

1. Klicken Sie auf die Schaltfläche „Repository ändern", um ein anderes Repository auszuwählen
2. Geben Sie den Repository-Namen in Ihrer Anfrage an, um eine genauere Auswahl zu erhalten

<h3 id="authentication-errors">
  Authentifizierungsfehler
</h3>

1. Trennen Sie Ihr Claude-Konto in der App Home und verbinden Sie es erneut
2. Stellen Sie sicher, dass Sie in Ihrem Browser mit dem richtigen Claude-Konto angemeldet sind
3. Überprüfen Sie, ob Ihr Claude-Plan Claude Code-Zugriff umfasst

<h2 id="current-limitations">
  Aktuelle Einschränkungen
</h2>

* **Nur GitHub**: Repositories müssen auf GitHub sein.
* **Ein PR gleichzeitig**: Jede Sitzung kann einen Pull Request erstellen.
* **Cloud-Sitzungszugriff erforderlich**: Benutzer benötigen Zugriff auf [Cloud-Sitzungen](/docs/de/claude-code-on-the-web); ohne diesen antwortet Claude mit Standard-Chat-Antworten.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

<CardGroup>
  <Card title="Claude Code in der Cloud" icon="cloud" href="/docs/de/claude-code-on-the-web">
    Erfahren Sie mehr über Cloud-Sitzungen
  </Card>

  <Card title="Claude für Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Allgemeine Claude for Slack-Dokumentation
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    Von der Organisation verwaltetes @Claude in Slack mit vom Administrator konfiguriertem Zugriff
  </Card>

  <Card title="Slack App Marketplace" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Installieren Sie die Claude-App aus dem Slack Marketplace
  </Card>

  <Card title="Claude Help Center" icon="circle-question" href="https://support.claude.com">
    Erhalten Sie zusätzliche Unterstützung
  </Card>
</CardGroup>
