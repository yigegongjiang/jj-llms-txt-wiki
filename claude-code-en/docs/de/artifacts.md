> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungsausgabe als Artefakte freigeben

> Artefakte verwandeln die Arbeit von Claude Code in Live-Seiten, die interaktiv sind und auf claude.ai verfügbar sind. Sie können diese privat halten, mit Ihrer Organisation teilen oder über einen öffentlichen Link veröffentlichen.

<Note>
  Artefakte sind auf Pro-, Max-, Team- und Enterprise-Plänen verfügbar und erfordern eine Sitzung, die mit [`/login`](/docs/de/setup#authenticate) angemeldet ist. Siehe [Verfügbarkeit](#availability) für die vollständige Liste der Anforderungen.
</Note>

Ein [Artefakt](https://claude.com/features/artifacts) ist eine Live-, interaktive Webseite, die Claude Code aus Ihrer Sitzung auf einer privaten URL auf claude.ai veröffentlicht. Sie öffnen sie in einem Browser, und sie wird aktualisiert, während die Sitzung fortgesetzt wird. Teilen Sie sie über die Kopfzeile der Seite, wenn jemand anderes sie auch sehen soll.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Ein Artefakt, das in einem Browser unter claude.ai/code/artifact geöffnet ist. Die Kopfzeile des Viewers zeigt den Artefakttitel acme-funnel-fix, eine Schaltfläche zum Freigeben und den Avatar des Autors. Das Freigabemenü ist offen mit dem Schalter „Immer neueste Version freigeben&#x22;, einer Versionswahl mit der Anzeige „Freigabe Version 2&#x22;, einer Zielgruppenauswahl „Alle bei Acme&#x22; und einer Schaltfläche zum Kopieren des Links. Unter der Kopfzeile zeigt die Artefaktseite zwei mobile Mockups nebeneinander, ein Trichterdiagramm und eine Reihe von Metrik-Karten." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Wann Sie ein Artefakt verwenden sollten
</h2>

Verwenden Sie ein Artefakt, wenn Terminaltext das falsche Medium für das ist, was Claude produziert hat: Ausgabe, die leichter anzusehen und zu interagieren ist als zeilenweise zu lesen. Claude erstellt die Seite aus allem, das Ihre Sitzung erreichen kann, einschließlich Ihrer Codebasis und Daten, die es durch Ihre [verbundenen Tools](/docs/de/mcp) abruft, sodass die Seite Dinge anzeigen kann, die Absätze zu beschreiben erfordern würden. Bitten Sie Claude beispielsweise um:

* Einen Reviewer durch einen Pull Request mit kommentierten Diffs zu führen
* Ein Dashboard aus Daten zu rendern, die die Sitzung bereits abgerufen hat
* Mehrere Design- oder Implementierungsoptionen nebeneinander anzuordnen
* Eine Untersuchungs-Timeline zu führen, die sich füllt, während eine lange Aufgabe läuft
* Einem Teamkollegen einen Link zu senden, anstatt die Ausgabe in Slack einzufügen
* Ein Status-Board zu veröffentlichen, das [bei jedem Öffnen frische Daten durch MCP-Konnektoren abruft](#pull-live-data-with-mcp-connectors)

Siehe [Was Sie erstellen können](#what-you-can-build) für Prompts, die zu diesen Szenarien passen, und [Frische Daten mit MCP-Konnektoren abrufen](#pull-live-data-with-mcp-connectors) für den Prompt des Konnektor-gestützten Boards.

<h3 id="what-an-artifact-is-not">
  Was ein Artefakt nicht ist
</h3>

Ein Artefakt ist eine Erfassung von Arbeit: eine einzelne, in sich geschlossene Seite ohne Backend, daher kann es keine mehreren Routen bedienen. Für ein gehostetes internes Tool mit einem Backend stellen Sie es stattdessen auf Ihrer eigenen Infrastruktur bereit. Siehe [Seitenbeschränkungen](#page-constraints) für die vollständige Liste der Limits.

<h2 id="create-an-artifact">
  Erstellen Sie ein Artefakt
</h2>

Claude kann ein Artefakt von selbst veröffentlichen, wenn die Ausgabe für eine Seite geeignet ist, oder Sie können direkt danach fragen. Um zu fragen, nennen Sie die Funktion oder beschreiben Sie die visuelle Ausgabe, die Sie in einfacher Sprache möchten. Ein guter Kandidat ist alles, das leichter zu sehen als als Text zu lesen ist, wie ein kommentierter Diff, ein Diagramm oder eine Reihe von Optionen zum Vergleichen. Die folgenden Prompts sind zwei Beispiele; siehe [Was Sie erstellen können](#what-you-can-build) für weitere Muster.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Sofern Sie keinen Speicherort angeben, schreibt Claude die Seite in eine HTML- oder Markdown-Datei in einem temporären Verzeichnis außerhalb Ihres Projekts und veröffentlicht sie dann. Das Veröffentlichen eines neuen Artefakts erfolgt über den [Berechtigungsmodus](/docs/de/permission-modes) Ihrer Sitzung:

* **Auto-Modus**: Der Klassifizierer überprüft die Veröffentlichung, anstatt Sie zu fragen, sodass Claude eine Seite veröffentlichen kann, ohne dass Sie eine Aufforderung sehen. Welcher Modus Ihre Sitzungen starten, hängt von Ihrem Plan ab; siehe [Der Startberechtigungsmodus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode).
* **Manueller Modus und Bearbeitungen akzeptieren-Modus**: Claude Code fragt um Genehmigung; es könnte etwa sagen: `Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it`. Wählen Sie **Ja**, um zu veröffentlichen.

Nachdem Sie ein Artefakt einmal genehmigt haben, veröffentlicht Claude Code es erneut, ohne zu fragen, und fragt in einigen Fällen erneut, einschließlich wenn:

* Claude deklariert eine Laufzeitfähigkeit für die Seite, wie [Konnektor-Aufrufe](#pull-live-data-with-mcp-connectors) oder [Datei-Downloads](#offer-a-file-download)
* Sie haben es seitdem [öffentlich freigegeben](#share-an-artifact)
* Sie haben es seitdem mit bestimmten Personen oder Ihrer Organisation freigegeben, wobei die neueste Version als die Version ausgewählt ist, die Betrachter sehen

Nach der ersten Veröffentlichung druckt Claude die URL aus, und Ihr Browser öffnet die neue Seite. Wenn Sie die Aufforderung über [Remote Control](/docs/de/remote-control) von claude.ai, Claude Desktop oder der Claude Mobile-App gesendet haben, öffnet sich auf dem Computer, auf dem die Sitzung läuft, keine Registerkarte. Der Browser öffnet sich dort beim nächsten Mal, wenn Claude das Artefakt aus einer Aufforderung veröffentlicht, die Sie im Terminal eingeben. Drücken Sie `Ctrl+]` jederzeit, um das neueste Artefakt der Sitzung erneut zu öffnen.

Claude wählt den Titel des Artefakts und ein Emoji, und beide werden in Ihrer [Galerie von Artefakten](#share-an-artifact) auf claude.ai und in freigegebenen Links angezeigt. Claude kann auch ein Browser-Tab-Symbol auswählen, das dem entspricht, was die Seite ist, wie ein Diagramm oder ein Kalender. Bitten Sie Claude um einen bestimmten Titel, ein bestimmtes Emoji oder ein bestimmtes Tab-Symbol, wenn Sie einen möchten.

Um zu verhindern, dass der Browser automatisch geöffnet wird, wenn ein neues Artefakt veröffentlicht wird, setzen Sie `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` in Ihrer Umgebung.

Wenn Claude antwortet, dass es nicht veröffentlichen kann, oder eine lokale HTML-Datei ohne Link schreibt, ist das Tool für Ihre Sitzung nicht aktiviert. Überprüfen Sie die [Verfügbarkeits](#availability)-Anforderungen.

<h2 id="update-an-artifact">
  Aktualisieren Sie ein Artefakt
</h2>

Bitten Sie Claude, die Seite zu überarbeiten, oder lassen Sie eine lange laufende Aufgabe erneut veröffentlichen, während sie Fortschritte macht. Claude bearbeitet die zugrunde liegende Datei und veröffentlicht sie erneut unter derselben URL.

```text wrap theme={null}
Add a per-region breakdown below the summary chart and republish.
```

Jeder, der die Seite offen hat, sieht die Aktualisierung an Ort und Stelle. Jede Veröffentlichung wird zu einer Version, und aus der **Freigabe**-Kontrolle im Seitenkopf können Sie auswählen, welche Version Betrachter sehen.

Um ein Artefakt aus einer anderen Sitzung zu aktualisieren, geben Sie Claude die URL des Artefakts, oder fügen Sie es mit [`/artifacts`](#find-an-artifact-again) an. Ohne beides erstellt eine neue Sitzung immer ein neues Artefakt, anstatt ein vorhandenes zu aktualisieren.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="find-an-artifact-again">
  Ein Artefakt erneut finden
</h2>

Führen Sie `/artifacts` in Claude Code aus, um jedes Artefakt aufzulisten, das Sie besitzen, und jedes Artefakt, das mit Ihnen geteilt wurde. Wählen Sie eines aus und drücken Sie `o`, um es in Ihrem Browser zu öffnen, oder `c`, um seinen Link zu kopieren. Drücken Sie `Enter`, um es an die aktuelle Sitzung anzufügen; vor v2.1.216 öffnete `Enter` es in Ihrem Browser. Claude Code liest die Liste aus Ihrem claude.ai-Konto, daher funktioniert es in einer neuen Sitzung und nach `/clear`, wenn der Link aus dem Terminal gescrollt ist. Erfordert Claude Code v2.1.208 oder später.

<h2 id="share-an-artifact">
  Ein Artefakt teilen
</h2>

Ein neues Artefakt ist zunächst nur für Sie sichtbar. Um es zu teilen, öffnen Sie das Artefakt in Ihrem Browser und verwenden Sie die **Freigabe**-Kontrolle in der Seitenkopfzeile. Die Kopfzeile verlinkt auch auf Ihre Galerie unter [claude.ai/code/artifacts](https://claude.ai/code/artifacts), die alle Artefakte auflistet, die Sie erstellt haben.

Betrachter in Ihrer Organisation können sehen, wer die Seite veröffentlicht hat: Bei einem Artefakt, das innerhalb Ihrer Organisation freigegeben ist, ist Ihr Name im Titelmenü, und bei einem öffentlichen Artefakt ist es in der Seitenkopfzeile für angemeldete Betrachter in Ihrer Organisation. Ein Betrachter, der einen öffentlichen Link öffnet, ohne sich anzumelden, oder von außerhalb Ihrer Organisation, sieht das Label `Content is user-generated and unverified.` anstelle Ihres Namens.

Mit wem Sie teilen können, hängt von Ihrem Plan ab:

* **Innerhalb Ihrer Organisation**: Bei Team- und Enterprise-Plänen können Sie Zugriff auf bestimmte Personen in Ihrer Organisation oder auf alle gewähren. Betrachter melden sich bei claude.ai als Mitglieder Ihrer Organisation an, um die Seite zu sehen.
* **Öffentlich**: Teilen Sie einen Link, den jeder im Internet öffnen kann, ohne sich bei claude.ai anmelden zu müssen. Bei Pro- und Max-Plänen ist ein öffentlicher Link die einzige Möglichkeit, ein Artefakt zu teilen. Bei Team- und Enterprise-Plänen ist die öffentliche Freigabe deaktiviert, bis ein Eigentümer [sie für die Organisation aktiviert](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Lassen Sie jemanden mit Ihnen bearbeiten
</h3>

Personen, mit denen Sie teilen, sind standardmäßig Betrachter: Sie sehen jede Version, die Sie veröffentlichen, können aber die Seite nicht ändern. Bei Team- und Enterprise-Plänen können Sie jemanden auch zum Bearbeiter machen. Fügen Sie im Freigabedialog eine Person hinzu und ändern Sie ihre Rolle von **Betrachter** zu **Bearbeiter**.

Ein Bearbeiter veröffentlicht neue Versionen auf die gleiche Weise wie Sie [das Artefakt aus einer anderen Sitzung aktualisieren](#update-an-artifact): Er oder sie gibt Claude die URL des Artefakts, oder fügt es aus [`/artifacts`](#find-an-artifact-again) an, und Claude ruft den aktuellen Inhalt ab und veröffentlicht ihn mit seinen oder ihren Änderungen erneut. Jeder, der die Seite offen hat, sieht jede Aktualisierung live.

<h2 id="read-an-artifact-shared-with-you">
  Ein mit Ihnen geteiltes Artefakt lesen
</h2>

Wenn jemand ein Artefakt mit Ihnen teilt, können Sie Claude es lesen lassen: Geben Sie Claude seine URL oder fügen Sie es aus [`/artifacts`](#find-an-artifact-again) an.

Claude liest eine Seite, die jemand anderes geschrieben hat, auf die gleiche Weise wie eine Webseite mit [WebFetch](/docs/de/tools-reference#webfetch-tool-behavior): Es erhält eine Zusammenfassung dessen, was es gefragt hat, anstelle der rohen Seite, und die Zusammenfassung meldet in die Seite geschriebene Anweisungen, anstatt sie weiterzuleiten. Claude Code speichert auch den vollständigen Quellcode der Seite in einer lokalen Datei, die Claude öffnen kann, wenn es den genauen Inhalt benötigt, z. B. um das Artefakt als [Editor](#let-someone-edit-with-you) erneut zu veröffentlichen.

<h2 id="collect-comments-on-an-artifact">
  Sammeln Sie Kommentare zu einem Artefakt
</h2>

Wenn Sie ein Artefakt innerhalb Ihrer Organisation freigeben, können die Personen, mit denen Sie es teilen, Kommentare auf der Seite hinterlassen, und Sie können Claude diese Kommentare lesen und darauf antworten lassen. Sie benötigen Claude Code v2.1.221 oder später und einen Team- oder Enterprise-Plan, da nur ein Artefakt, das Sie [innerhalb Ihrer Organisation freigeben](#share-an-artifact), Kommentare akzeptiert. Claude liest die Kommentare in zwei Fällen:

* **Sie bitten Claude, sie zu lesen**: Geben Sie Claude die URL des Artefakts und bitten Sie um die Kommentare. Claude listet jeden Thread auf und markiert die Kommentare, die jemand, der das Artefakt bearbeiten kann, an ihn gesendet hat.
* **Jemand, der das Artefakt bearbeiten kann, sendet einen Kommentar an Claude**: In einem Thread auf der Seite sendet er oder sie einen Kommentar mit **An Claude senden**, oder erwähnt `@claude` in einem. Auf beide Arten aktivieren sie den Thread.

Claude kann nur auf einen aktivierten Thread antworten oder ihn auflösen. Andere Threads bleiben offen, bis eine Person sie auf der Seite auflöst. Betrachter sehen jede Antwort, die Claude zugeordnet ist, über Sie.

Wenn Sie ein Artefakt öffentlich freigeben, können Betrachter keine Kommentare hinterlassen: Die Seite sagt `Comments aren't available while this Artifact is shared publicly.` Um ein Artefakt, das bereits Kommentar-Threads hat, auf einen öffentlichen Link umzuschalten, löschen Sie die Threads zuerst.

Um die Kommentare selbst zu erfragen, geben Sie Claude die URL:

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Wenn Claude Ihnen sagt, dass es keine Kommentare lesen kann, bestätigen Sie Ihre Version, Ihre Sitzung und Ihre Feature-Flag-Einstellung:

* Sie führen Claude Code v2.1.221 oder später aus.
* Sie befinden sich nicht in Ihrer ersten Sitzung, seit Sie Claude Code installiert oder von einer Version vor v2.1.221 aktualisiert haben. In dieser [ersten Sitzung nach einer Installation oder einem Upgrade](/docs/de/env-vars#first-session-after-an-install-or-upgrade) kann Claude möglicherweise noch keine Kommentare lesen; starten Sie eine neue Sitzung und fragen Sie erneut.
* Sie haben das Abrufen von Feature-Flags nicht ausgeschaltet.

<h3 id="let-claude-reply-to-comments-on-its-own">
  Lassen Sie Claude auf Kommentare von selbst antworten
</h3>

Nachdem Ihre Sitzung ein Artefakt veröffentlicht hat, überwacht Claude Code dieses Artefakt auf Kommentare, solange die Sitzung läuft. Wenn jemand, der das Artefakt bearbeiten kann, einen Kommentar an Claude sendet, erreicht er Ihre Sitzung sofort, und Claude kann den Thread lesen und antworten, ohne dass Sie fragen.

Sie benötigen Claude Code v2.1.228 oder später. Wenn Sie [das Abrufen von Feature-Flags](/docs/de/env-vars#features-that-need-feature-flag-fetching) ausgeschaltet haben, überwacht Claude Code keine Kommentare.

Ihr [Berechtigungsmodus](/docs/de/permission-modes) entscheidet, was Claude tut, wenn ein gesendeter Kommentar ankommt:

* **Claude antwortet von selbst**: Wenn Ihr Berechtigungsmodus Claude ermöglicht, die Antwort zu posten, ohne Sie zu fragen, liest Claude den Thread und antwortet, und bearbeitet das Artefakt, wenn der Kommentar eine Änderung verlangt. Sie sehen `Auto-replied to comment thread on Artifact: <name>` oder `Auto-edited Artifact: <name> in response to a comment thread`.
* **Claude wartet auf Sie**: Außerhalb des Plan-Modus, wenn das Posten der Antwort Ihre Genehmigung benötigen würde, sehen Sie `Comments are waiting on Artifact: <name>`. Claude fragt Sie dann um Genehmigung, um den Thread zu lesen, und erneut, um die Antwort zu posten.
* **Claude pausiert im Plan-Modus**: Sie sehen `Comments are waiting on Artifact: <name>`, und Claude antwortet nicht, bis Sie den Plan-Modus verlassen und es bitten, zu lesen und zu antworten.

Claude stoppt auch das automatische Antworten auf ein Artefakt, nachdem es 60 gesendete Kommentare oder Thread-Aktivierungen auf diesem Artefakt innerhalb einer Stunde bearbeitet hat. Sie sehen `Comments are waiting on Artifact: <name>` einmal, und Claude nimmt wieder auf, wenn diese Stunde Kommentare veralten.

Führen Sie `/tasks` aus, um jedes Artefakt zu sehen, das Ihre Sitzung überwacht, aufgelistet als eine Live-Updates-Aufgabe. Sie können Claude auf folgende Weise davon abhalten, von selbst zu antworten:

* **Drücken Sie Ctrl+C einmal bei einer leeren Eingabeaufforderung**: Claude pausiert das Antworten auf jedem Artefakt, das Ihre Sitzung überwacht. Antworten beginnen wieder, nachdem Sie Ihre nächste Nachricht senden.
* **Stoppen Sie die Aufgabe in `/tasks`**: Claude stoppt das Antworten auf diesem Artefakt, bis Sie es bitten, Antworten dort zu fortsetzen. Das erneute Veröffentlichen des Artefakts startet Antworten nicht erneut, und der Stopp gilt immer noch, wenn Sie die Sitzung später fortsetzen.
* **Drücken Sie `Ctrl+X Ctrl+K` zweimal innerhalb von 3 Sekunden**: Der Akkord, der [jeden laufenden Hintergrund-Subagenten stoppt](/docs/de/interactive-mode#general-controls), stoppt auch Claude vom Antworten auf jedem Artefakt für den Rest der Sitzung. Das Bitten von Claude, Antworten zu fortsetzen, macht diesen Stopp nicht rückgängig.

Wenn der Dienst, der Kommentare liefert, nicht verfügbar wird oder nicht mehr antwortet, versucht Claude Code, sich eine Weile erneut zu verbinden, und stoppt dann die Überwachung jedes Artefakts, das Ihre Sitzung überwacht hat.

<h2 id="pull-live-data-with-mcp-connectors">
  Live-Daten mit MCP-Konnektoren abrufen
</h2>

Ein Artefakt kann [MCP-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) jedes Mal aufrufen, wenn jemand es anzeigt, sodass die Seite aktuelle Daten anstelle eines Snapshots aus der Sitzung anzeigt, in der sie erstellt wurde. Konnektoraufrufe aus Artefakten sind in den Plänen Pro, Max, Team und Enterprise verfügbar und erfordern Claude Code v2.1.209 oder später. In früheren Versionen veröffentlicht Claude die Seite mit den Daten, die die Sitzung während der Erstellung gesammelt hat.

Um eine Konnektor-gestützte Seite zu erstellen, nennen Sie den Konnektor und die gewünschten Daten in Ihrer Eingabeaufforderung:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude deklariert, welche Konnektoren die Seite aufrufen darf, als Teil der Veröffentlichung, und die Seite kann keine Konnektoren außerhalb dieser Deklaration aufrufen. Nur Konnektoren aus Ihrem claude.ai-Konto kommen in Frage: Claude benennt sie in der Deklaration, und wenn jemand die Seite anzeigt, wird jeder Aufruf [über die eigene Verbindung des anzeigenden Kontos zu diesem Konnektor ausgeführt](#how-connector-calls-work-for-viewers). Lokale MCP-Server, die Sie in Claude Code konfigurieren, wie Server aus `.mcp.json`, können Daten liefern, während Claude die Seite erstellt, aber die veröffentlichte Seite kann sie nicht aufrufen.

Die Seite ruft Daten beim Laden ab und kann in einem Intervall aktualisiert werden oder wenn ein Betrachter ein Aktualisierungssteuerelement auf der Seite verwendet. Antworten werden im Browser des Betrachters zwischengespeichert, sodass eine erneut geöffnete Seite sofort aus den zwischengespeicherten Antworten gerendert wird und sich dann mit frischen Ergebnissen aktualisiert.

<h3 id="how-connector-calls-work-for-viewers">
  Funktionsweise von Konnektoraufrufen für Betrachter
</h3>

Wenn eine veröffentlichte Seite einen Konnektor aufruft, verwendet der Aufruf das Konto der Person, die die Seite anzeigt, nicht das Konto der Person, die sie veröffentlicht hat:

* **Jeder Betrachter verwendet seine eigenen Konnektoren**: Aufrufe erfolgen über die verbundenen Tools des anzeigenden Kontos, sodass zwei Personen, die dasselbe Dashboard öffnen, je nach dem, worauf ihre Konten zugreifen können, unterschiedliche Daten sehen können. Die Seite sieht niemals die Anmeldedaten von jemandem; claude.ai führt die Aufrufe im Namen der Seite aus.
* **Betrachter genehmigen den Zugriff zuerst**: claude.ai fragt jeden Betrachter um Genehmigung, bevor der erste Konnektoraufruf der Seite erfolgt. Ein Betrachter, der ablehnt oder keinen Konnektor verbunden hat, den die Seite verwendet, sieht die Seite immer noch ohne ihre Live-Abschnitte.
* **Aktionen verwenden auch das Konto des Betrachters**: Eine Seite kann Steuerelemente anbieten, die Konnektor-Tools mit Nebenwirkungen aufrufen, z. B. das Posten einer Nachricht oder das Aktualisieren eines Problems. Die Aktion erfolgt über das Konto derjenigen Person, die das Steuerelement auswählt.

Wenn Sie planen, eine Konnektor-gestützte Seite freizugeben, bitten Sie Claude, in jedem Live-Abschnitt eine Fallback-Nachricht einzufügen, die den benötigten Konnektor benennt. Ein Betrachter, dem die Verbindung fehlt, sieht dann, was verbunden werden muss, anstatt eines leeren Abschnitts.

Ein Artefakt, das Konnektoren aufruft, kann in keinem Plan über einen öffentlichen Link freigegeben werden. In den Plänen Team und Enterprise können Sie es privat halten oder [es innerhalb Ihrer Organisation freigeben](#share-an-artifact). In den Plänen Pro und Max, bei denen ein öffentlicher Link die einzige Möglichkeit zum Freigeben ist, bleibt ein Konnektor-gestütztes Artefakt privat für Sie.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  Die Seite zeigt für einen Betrachter keine Live-Daten an
</h3>

Wenn eine Konnektor-gestützte Seite gerendert wird, aber ihre Live-Abschnitte für jemanden, mit dem Sie sie geteilt haben, leer bleiben, arbeiten Sie diese Ursachen durch:

* **Der Betrachter hat den Konnektor nicht verbunden**: Konnektoren sind pro Konto, daher benötigt jeder Betrachter seine eigene Verbindung zu jedem Konnektor, den die Seite aufruft. Sie können einen unter **Einstellungen > Konnektoren** auf claude.ai hinzufügen und dann die Seite neu laden.
* **Der Betrachter hat die Genehmigungsanfrage abgelehnt**: Eine Ablehnung gilt für den Rest dieses Seitenladevorgangs. Das Neuladen der Seite bringt die Genehmigungsanfrage zurück.
* **Konnektoraufrufe sind für die Organisation deaktiviert**: Ein Besitzer steuert den [**Artefakt-Konnektoren aktivieren**-Schalter](#control-connector-calls-from-artifacts) in den Admin-Einstellungen.
* **Die Seite ruft Tool-Namen auf, die der Konnektor nicht verfügbar macht**: Die betroffenen Abschnitte bleiben für alle leer, einschließlich Sie. Dies kann vorkommen, wenn eine Seite die einzelnen Tools hinter einem Gateway-ähnlichen Konnektor benennt, der nur wenige seiner eigenen Tools verfügbar macht. Bitten Sie Claude, die Tool-Namen zu korrigieren, die die Seite aufruft, und veröffentlichen Sie sie erneut.

  Wenn Claude die Seite veröffentlicht und die Tools dieses Konnektors in Ihrer Sitzung verfügbar sind, überprüft Claude Code die Tool-Namen, die die Seite deklariert, anhand dieser, warnt Claude vor Namen, die nicht übereinstimmen, und weigert sich zu veröffentlichen, wenn keine übereinstimmen. Vor v2.1.265 wurde die Seite ohne Überprüfung veröffentlicht.

<h2 id="offer-a-file-download">
  Dateidownload anbieten
</h2>

Ein Artefakt kann Betrachtern eine Datei anbieten, die die Seite generiert, z. B. einen CSV-Export einer Tabelle oder ein PNG eines Diagramms. Der Betrachter speichert sie über ein Download-Steuerelement auf der Seite, z. B. eine Schaltfläche. Dateidownloads sind eine Laufzeitfunktion, die claude.ai pro Konto aktiviert, daher prüft Claude, ob Ihr Konto diese Funktion hat, bevor es das Steuerelement erstellt.

Betrachter können eine Datei nicht über einen gewöhnlichen Download-Link oder ein Skript auf der Seite speichern, da der Artefakt-Viewer auf claude.ai jeden Download blockiert, den die Seite selbst startet, einschließlich Links zu `data:`- oder `blob:`-URLs. Wenn eine Seite auf diese Weise erstellte Download-Schaltflächen hat, bitten Sie Claude, diese mit der Downloads-Funktion neu zu erstellen.

Um eine Datei anzubieten, fordern Sie das Steuerelement und das Dateiformat in Ihrer Eingabeaufforderung an:

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude deklariert die Downloads-Funktion als Teil der Veröffentlichung, auf die gleiche Weise wie es [Konnektoren deklariert](#pull-live-data-with-mcp-connectors).

<h2 id="what-you-can-build">
  Was Sie erstellen können
</h2>

Ein Artefakt ist eine einzelne HTML-Seite, daher ist alles, das Sie in HTML, CSS und Inline-JavaScript ausdrücken können, im Umfang enthalten. Die folgenden Muster treten am häufigsten auf.

<h3 id="walk-through-a-change">
  Gehen Sie durch eine Änderung
</h3>

Bitten Sie um eine Seite, die einen Diff oder eine Designänderung mit Anmerkungen neben den relevanten Zeilen rendert, damit Reviewer Ihre Begründung neben dem Code lesen können, anstatt sie aus einer Beschreibung zu rekonstruieren.

```text wrap theme={null}
Make an artifact that walks through this PR. Render the diff with margin annotations and color-code findings by severity.
```

<h3 id="compare-alternatives">
  Vergleichen Sie Alternativen
</h3>

Bitten Sie um mehrere Varianten auf einer Seite, damit Sie sie gegeneinander bewerten können. Dies funktioniert für Layouts, Kopien, API-Formen oder Implementierungspläne.

```text wrap theme={null}
Make an artifact with four distinctly different layouts for the settings panel. Vary density and grouping, and lay them out as a grid with a one-line tradeoff under each.
```

<h3 id="tune-with-interactive-controls">
  Optimieren Sie mit interaktiven Steuerelementen
</h3>

Bitten Sie um Schieberegler, Umschalter oder Eingabefelder, die an das gebunden sind, das Sie anpassen, damit Sie Werte direkt erkunden können, anstatt sie zu beschreiben.

```text wrap theme={null}
Build an artifact with sliders for the easing curve, duration, and delay so I can try values on this transition. Show the animation live as I move them.
```

<h3 id="bring-the-result-back-to-your-session">
  Bringen Sie das Ergebnis zurück zu Ihrer Sitzung
</h3>

Ein Artefakt kann als leichter Editor für eine Entscheidung fungieren, die Sie dann an Claude zurückgeben. Bitten Sie um ein Exportsteuerelement, das Text erzeugt, den Sie in das Terminal einfügen können, damit das Ergebnis der Interaktion mit der Seite zurück in die Sitzung fließt, anstatt auf der Seite zu bleiben.

```text wrap theme={null}
Make a triage board artifact with each open issue as a draggable card across Now, Next, Later, and Cut columns. Add a "Copy as prompt" button that gives me the final ordering to paste back here.
```

<h3 id="track-work-in-progress">
  Verfolgen Sie laufende Arbeiten
</h3>

Bitten Sie Claude, ein Artefakt aktuell zu halten, während eine lange Aufgabe läuft, damit jeder mit dem Link folgen kann, ohne das Terminal zu lesen.

```text wrap theme={null}
Turn this migration plan into a checklist artifact. Check items off as you complete them and add a note for anything you skip.
```

<h2 id="improve-the-visual-design">
  Verbessern Sie das visuelle Design
</h2>

Claude wendet eine integrierte Design-Fähigkeit an, wenn es ein Artefakt erstellt, sodass Seiten eine absichtliche Palette, Typografie und Layout ohne zusätzliche Aufforderung erhalten. Diese Fähigkeit sucht auch nach einem vorhandenen Design-System in Ihrem Projekt, bevor es sein eigenes auswählt. Design-Token sind die benannten Farb-, Typografie- und Abstands-Werte, die Ihr Design-System wiederverwenden. Um Artefakte konsistent mit dem Branding Ihres Produkts zu halten, notieren Sie sie dort, wo Claude sie finden kann, wie in der [CLAUDE.md](/docs/de/memory) des Projekts oder einer Theme-Datei in Ihrem Repository:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude behandelt Ihr Design-System als höhere Priorität als seine eigenen Entscheidungen, und Ihren Prompt als höhere Priorität als beide. Die Überschrift und das Format oben sind ein Beispiel; jede klare Liste von Farben, Schriftarten und Abstände funktioniert.

Für Typografie kann Claude eine Schriftart von Google Fonts laden, der einzigen externen Schriftquelle, die eine Artefakt-Seite laden kann. Claude inline jede andere Schriftart als `@font-face` Data-URI und gibt jeder Schriftart einen Fallback-Stack, sodass die Seite immer noch gerendert wird, wenn eine Schriftart nicht geladen wird. Um eine bestimmte Schriftart zu verwenden, nennen Sie sie in Ihrem Prompt oder Ihrem Design-System.

<h2 id="draft-a-design-canvas">
  Entwerfen Sie eine Design-Canvas
</h2>

Um eine Benutzeroberfläche, einen Bildschirmfluss, eine Landing Page oder ein Poster zu skizzieren, anstatt eine Seite zu erstellen, führen Sie `/design` mit einer kurzen Beschreibung aus. Claude entwirft das Design als Artboards auf einer Canvas und veröffentlicht die Canvas als Design-Artefakt. Die Beschreibung nennt, was Sie gezeichnet haben möchten:

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

Öffnen Sie das veröffentlichte Artefakt in einem Desktop-Browser, um die Artboards zu überprüfen. Wählen Sie ein Element auf einem Artboard aus und ändern Sie es, und Ihre Änderungen werden automatisch gespeichert. Sie können jedes Artboard als PNG oder PDF exportieren.

`/design` erfordert eine Sitzung, in der [Artefakte verfügbar sind](#availability), und Claude Code v2.1.265 oder später.

<h2 id="page-constraints">
  Seitenbeschränkungen
</h2>

Jedes Artefakt ist eine einzelne, in sich geschlossene Seite. Claude Code umhüllt die Datei, die Sie veröffentlichen, in einer HTML-Dokumentshell und bedient sie unter einer strikten Content Security Policy (CSP), die formt, was die Seite tun kann.

| Beschränkung     | Auswirkung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Externe Anfragen | Die Seite kann Schriftarten von Google Fonts laden und Skripte von [fünf öffentlichen CDN-Hosts](#allowlist-the-viewer-domain): cdnjs, unpkg, den Tailwind- und jQuery-CDNs und ausgewählten Pfaden auf jsDelivr wie `/npm/`. Die CSP blockiert jedes externe Bild und alle anderen externen Skripte, Stylesheets und Schriftarten, und lässt `fetch`, XHR und WebSocket-Aufrufe nur den Ursprung der Seite selbst und die Google Fonts-Hosts erreichen. Claude lädt daher jede Bibliothek, die die Seite benötigt, von einem dieser CDNs, inline alle anderen CSS und JavaScript, und bettet Bilder als Data-URIs ein. [Connector-Aufrufe](#pull-live-data-with-mcp-connectors) erfolgen über claude.ai, das den Netzwerkaufruf selbst tätigt. |
| Kein Backend     | Ein Artefakt ist eine statische Seite. Es kann Betrachter nicht selbst authentifizieren.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Downloads        | Die Seite kann einen Download nicht selbst starten. Um Betrachtern zu ermöglichen, eine Datei zu speichern, die die Seite generiert, erklärt Claude die Downloads-Funktion. Siehe [Eine Datei zum Download anbieten](#offer-a-file-download).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Einzelne Seite   | Relative Links werden nicht aufgelöst, da nichts neben der Seite bereitgestellt wird. Für mehrteilige Inhalte verwendet Claude In-Page-Anker anstelle von separaten Dateien.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Quelldateitypen  | Die veröffentlichte Datei muss `.html`, `.htm` oder `.md` sein und muss als UTF-8 oder als Little-Endian-UTF-16 durch ihre Byte-Order-Marke dekodierbar sein. Markdown-Dateien werden als gestyltes Dokument mit Syntax-Highlighting für Code gerendert. Eine Datei, die nicht dekodierbar ist oder das Ersatzzeichen `U+FFFD` enthält, wird [mit der Zeile und Spalte zum Beheben abgelehnt](/docs/de/errors#the-source-file-is-not-valid-utf-8-text).                                                                                                                                                                                                                                                                                              |
| Gerenderte Größe | Die gerenderte Seite muss 16 MiB oder kleiner sein. Große eingebettete Bilder sind die übliche Ursache, wenn eine Veröffentlichung aus Größengründen fehlschlägt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

Das Generieren eines Artefakts verwendet Ausgabe-Token wie jede andere Antwort, und eine gestylte Seite ist token-intensiver als derselbe Inhalt als Terminaltext. Inline-CSS, JavaScript für interaktive Steuerelemente und besonders Bilder, die als Data-URIs eingebettet sind, sind die Hauptbeiträge. Um die Token-Kosten eines Artefakts zu reduzieren:

* Bevorzugen Sie SVG oder HTML und CSS für Diagramme gegenüber eingebetteten Rasterbildern
* Lassen Sie Interaktivität weg, die Sie nicht benötigen
* Lassen Sie die Seite große Datensätze zusammenfassen, anstatt sie vollständig inline zu verwenden

<h2 id="availability">
  Verfügbarkeit
</h2>

Artefakte erfordern jede Bedingung unten. Wenn eine nicht erfüllt ist, schreibt Claude eine lokale HTML-Datei oder sagt, dass es nicht veröffentlichen kann.

| Anforderung             | Verfügbar wenn                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan                    | Pro, Max, Team oder Enterprise. Bei Pro- und Max-Plänen sind Artefakte privat für Sie, bis Sie sie freigeben, und es gilt keine Admin-Verwaltung. Bei Team-Plänen sind Artefakte standardmäßig aktiviert. Bei Enterprise-Plänen [aktiviert ein Owner sie](#manage-artifacts-for-your-organization) in den claude.ai-Admin-Einstellungen.                                                                                                               |
| Authentifizierung       | Die Sitzung wird durch ein claude.ai-Konto unterstützt: Melden Sie sich mit `/login` in der CLI oder Desktop-App an. Claude Tag-Sitzungen sind durch die Identität des Agenten angemeldet, daher ist kein Schritt erforderlich. Sitzungen, die einen API-Schlüssel, [Gateway-Token](/docs/de/llm-gateway) oder Cloud-Provider-Anmeldedaten verwenden, können nicht veröffentlichen.                                                                         |
| Modell-Provider         | Anthropic API. Nicht verfügbar auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry).                                                                                                                                                                                                                                                                        |
| Organisationsrichtlinie | Customer-managed encryption keys (CMEK), HIPAA und [Zero Data Retention](/docs/de/zero-data-retention) sind nicht für die Organisation aktiviert.                                                                                                                                                                                                                                                                                                           |
| Oberfläche              | Claude Code CLI oder die Claude Desktop-App Version 1.13576.0 oder später. [Claude Tag](https://claude.com/docs/claude-tag/overview)-Sitzungen können auch Artefakte veröffentlichen, wenn sowohl Claude Tag als auch Artefakte für die Organisation aktiviert sind. Standardmäßig aus in [Agent SDK](/docs/de/agent-sdk/overview), GitHub Action und MCP-Server-Kontexten und wenn [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars) gesetzt ist. |

Ob Artefakte für Ihre Organisation zulässig sind, ergibt sich aus der Richtlinie Ihrer Organisation, die Claude Code von `api.anthropic.com` lädt. Wenn Claude Code die Richtlinie nicht laden kann, sind Artefakte nicht verfügbar. Wenn Sie um eines bitten, sagt Claude, warum.

Wenn ein Proxy, VPN oder Web-Filter beteiligt ist, bitten Sie Ihren IT-Administrator, `api.anthropic.com` durchzulassen. Claude Code versucht es im Hintergrund weiterhin, und Artefakte werden verfügbar, sobald die Richtlinie geladen wird und sie zulässt.

<h2 id="disable-artifacts">
  Deaktivieren Sie Artefakte
</h2>

Um Artefakte für Ihre eigenen Sitzungen unabhängig von der Einstellung Ihrer Organisation auszuschalten, verwenden Sie eine der folgenden Optionen:

| Wo                                    | Was zu tun ist                                                                                                                                          |
| :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [`/config`](/docs/de/commands)             | Schalten Sie die Zeile **Artefakte** aus, die [`"enableArtifact": false`](/docs/de/settings-reference#enableartifact) in Ihre Benutzereinstellungen schreibt |
| [Einstellungsdatei](/docs/de/settings)     | Setzen Sie `"enableArtifact": false`. Das veraltete `"disableArtifact": true` schaltet auch Artefakte aus                                               |
| [Umgebungsvariable](/docs/de/env-vars)     | Setzen Sie `CLAUDE_CODE_DISABLE_ARTIFACT=1`                                                                                                             |
| [Berechtigungsregel](/docs/de/permissions) | Fügen Sie `Artifact` zu `permissions.deny` hinzu                                                                                                        |

Sobald Sie Artefakte in einer [`--settings`](/docs/de/cli-reference#cli-flags)-Datei oder mit `CLAUDE_CODE_DISABLE_ARTIFACT` ausschalten, oder Ihr Administrator schaltet sie in [verwalteten Einstellungen](/docs/de/server-managed-settings) aus, schaltet keine Einstellungsdatei sie wieder ein. Vor v2.1.242 konnte eine Datei höher im [Prioritätsstapel](/docs/de/settings#settings-precedence) Artefakte wieder einschalten, selbst wenn eine Datei mit niedrigerer Priorität `"enableArtifact": false` setzte.

Sie können auch `"enableArtifact": false` in der `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts setzen, um Artefakte für Sitzungen in diesem Projekt auszuschalten. Ein `"enableArtifact": true` in einer der beiden Dateien schaltet sie nicht wieder ein. Das Ehren des Schlüssels in Projekt- und lokalen Einstellungen erfordert Claude Code v2.1.242 oder später.

Wenn Sie eine `WebFetch`-Ablehnungs- oder Anfrage-Regel ohne `domain:`-Teil hinzufügen, schaltet sie Artefakte nicht aus und blockiert auch keine Artefakt-Lesevorgänge. Eine [`WebFetch(domain:claude.ai)`-Regel in `deny` oder `ask` gilt für Artefakt-Lesevorgänge](/docs/de/permissions#allow-or-deny-every-fetch).

<h2 id="manage-artifacts-for-your-organization">
  Verwalten Sie Artefakte für Ihre Organisation
</h2>

Inhaber bei Team- und Enterprise-Plänen steuern Artefakte aus [claude.ai Admin-Einstellungen](https://claude.ai/admin-settings/claude-code). Der Artefaktinhalt wird auf von Anthropic betriebener Infrastruktur gespeichert und ist nur für authentifizierte Mitglieder der veröffentlichenden Organisation sichtbar, es sei denn, das Artefakt wird [öffentlich freigegeben](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Aktivieren oder deaktivieren Sie Artefakte
</h3>

Um Artefakte für die gesamte Organisation zu aktivieren oder zu deaktivieren, gehen Sie zu [**Einstellungen > Claude Code > Funktionen**](https://claude.ai/admin-settings/claude-code) und verwenden Sie den Umschalter **Artefakte**. Bei Enterprise-Plänen mit rollenbasierter Zugriffskontrolle können Sie Artefakte zusätzlich auf bestimmte Rollen beschränken: Gehen Sie zu [**Einstellungen > Rollen**](https://claude.ai/admin-settings/roles), bearbeiten Sie eine Rolle und setzen Sie die Berechtigung **Artefakte** unter der Gruppe **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Steuern Sie Connector-Aufrufe aus Artefakten
</h3>

[Connector-Aufrufe aus Artefakten](#pull-live-data-with-mcp-connectors) haben ihren eigenen Umschalter, getrennt vom Umschalter **Artefakte**, der Artefakte ein- oder ausschaltet. Gehen Sie zu [**Einstellungen > Funktionen**](https://claude.ai/admin-settings/capabilities) und verwenden Sie den Umschalter **Artefakt-Konnektoren aktivieren**. Der gleiche Umschalter regelt Connector-Aufrufe aus Artefakten, die in claude.ai-Gesprächen erstellt wurden, weshalb er unter **Einstellungen > Funktionen** statt unter **Einstellungen > Claude Code** angeordnet ist.

<h3 id="control-public-sharing">
  Steuern Sie die öffentliche Freigabe
</h3>

Die öffentliche Freigabe ist standardmäßig bei Team- und Enterprise-Plänen deaktiviert, sodass Mitglieder Artefakte nur innerhalb der Organisation freigeben können, bis ein Inhaber sie aktiviert. Um Mitgliedern zu ermöglichen, Artefakte auf öffentliche Links zu veröffentlichen, die jeder ohne Anmeldung anzeigen kann, gehen Sie zu **Einstellungen > Claude Code > Funktionen** und aktivieren Sie **Externe Freigabe** unter dem Umschalter **Artefakte**. Wenn Sie ihn wieder ausschalten, wird der Zugriff über vorhandene öffentliche Links blockiert, ohne die Zielgruppe jedes Artefakts zu ändern; der Zugriff wird wiederhergestellt, wenn Sie ihn erneut aktivieren.

<h3 id="set-a-retention-policy">
  Legen Sie eine Aufbewahrungsrichtlinie fest
</h3>

Um festzulegen, wie lange Artefakte vor automatischer Löschung aufbewahrt werden, gehen Sie zu [**Einstellungen > Datenschutz- und Datenschutzkontrollen**](https://claude.ai/admin-settings/data-privacy-controls). Sie können separate Aufbewahrungszeiträume für Artefakte festlegen, die noch privat für ihren Autor sind, und Artefakte, die freigegeben wurden.

<h3 id="review-the-audit-log">
  Überprüfen Sie das Audit-Protokoll
</h3>

Das Veröffentlichen, Freigeben und Löschen eines Artefakts wird jeweils in dem Audit-Protokoll Ihrer Organisation unter den `claude_artifact_*`-Ereignistypen angezeigt, der gleichen Familie, die für Artefakte verwendet wird, die in claude.ai-Gesprächen erstellt werden.

<h3 id="allowlist-the-viewer-domain">
  Allowlist die Viewer-Domain
</h3>

Der Viewer auf claude.ai lädt jedes Artefakt aus einer Sandbox-`*.claudeusercontent.com`-Origin. Wenn Ihre Organisation den ausgehenden Netzwerkzugriff einschränkt, fügen Sie diese Domain zu Ihrer Allowlist neben `claude.ai` hinzu. Siehe [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements) für die vollständige Liste.

Ein Artefakt, das eine Schriftart von [Google Fonts](#improve-the-visual-design) lädt, fordert auch `fonts.googleapis.com` und `fonts.gstatic.com` an. Beide Hosts sind optional. Wenn Sie sie blockieren, werden Artefakte in Fallback-Schriftarten gerendert. Blockieren Sie mit einer schnellen Ablehnung, anstatt eines stillen Drops, damit die Schriftartanfrage sofort fehlschlägt, anstatt das erste Rendern der Seite zu verzögern.

Artefakte können auch JavaScript-Bibliotheken wie React oder ein Charting-Paket von `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` und `unpkg.com` laden, und von keinem anderen externen Host. Wenn Sie diese Hosts blockieren, funktionieren die Teile eines Artefakts, die von einer Bibliothek abhängen, nicht, und im Gegensatz zu einer blockierten Schriftart hat eine blockierte Bibliothek keinen Fallback. Blockieren Sie auch hier mit einer schnellen Ablehnung, damit eine blockierte Bibliotheksanfrage sofort fehlschlägt, anstatt zu hängen, bis sie abläuft.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Listen Sie Artefakte mit der Compliance API auf und löschen Sie sie
</h3>

Die [Compliance API](https://docs.claude.com/en/api/compliance) bietet Endpunkte zum Auflisten der Artefakte einer Organisation, zum Abrufen des Inhalts einer bestimmten Version und zum Löschen eines Artefakts:

| Methode  | Endpunkt                                                            |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Für die Request- und Response-Schemas siehe die [Compliance API-Referenz](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* Durchsuchen Sie [Prompting-Muster und Workflows](/docs/de/prompt-library), die mit Artefakten gepaart sind
* Verwandeln Sie einen Artefakt-Prompt, den Sie wiederverwenden, in einen [Skill](/docs/de/skills), damit Sie ihn als Befehl aufrufen können
* [Verbinden Sie MCP-Server](/docs/de/mcp), damit Claude Daten in ein Artefakt abrufen kann, während es die Seite erstellt
