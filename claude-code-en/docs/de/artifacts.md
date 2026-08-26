> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungsausgabe als Artefakte freigeben

> Artefakte verwandeln die Arbeit von Claude Code in Live-Seiten, die interaktiv sind und auf claude.ai verfügbar sind. Sie können diese privat halten, mit Ihrer Organisation teilen oder über einen öffentlichen Link veröffentlichen.

<Note>
  Artefakte sind auf Pro-, Max-, Team- und Enterprise-Plänen verfügbar und erfordern eine Sitzung, die mit [`/login`](/docs/de/setup#authenticate) angemeldet ist. Siehe [Verfügbarkeit](#availability) für die vollständige Liste der Anforderungen.
</Note>

Ein Artefakt ist eine Live-, interaktive Webseite, die Claude Code aus Ihrer Sitzung auf einer privaten URL auf claude.ai veröffentlicht. Sie öffnen sie in einem Browser, und sie wird aktualisiert, während die Sitzung fortgesetzt wird. Teilen Sie sie über die Kopfzeile der Seite, wenn jemand anderes sie auch sehen soll. Verwenden Sie beispielsweise ein Artefakt, um einen Reviewer durch einen Pull Request mit kommentierten Diffs zu führen, ein Dashboard aus Sitzungsdaten zu erstellen oder eine Untersuchungs-Timeline zu führen, die sich füllt, während Claude arbeitet.

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

Ein Artefakt ist eine Erfassung von Arbeit, keine Anwendung. Es ist eine einzelne, in sich geschlossene Seite ohne Backend, daher kann es keine Formulareingaben speichern oder mehrere Routen bedienen, und sein einziger Weg zu externen Daten, wenn jemand es anzeigt, ist das [Aufrufen von MCP-Konnektoren](#pull-live-data-with-mcp-connectors). Für ein gehostetes internes Tool mit einem Backend stellen Sie es stattdessen auf Ihrer eigenen Infrastruktur bereit. Siehe [Seitenbeschränkungen](#page-constraints) für die vollständige Liste der Limits.

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

Claude schreibt die Seite in eine HTML- oder Markdown-Datei in Ihrem Projekt und veröffentlicht sie dann. Bevor Claude Code ein neues Artefakt veröffentlicht, fragt es um Genehmigung; es könnte etwa sagen: `Claude wants to publish "Deploy failures by service" (deploy-failures.html) to a private page on claude.ai`. Das erneute Veröffentlichen eines Artefakts, das Sie bereits genehmigt haben, wird nicht erneut angefordert.

Wählen Sie **Ja**, um zu veröffentlichen. Claude gibt die URL aus, und Ihr Browser öffnet die neue Seite. Drücken Sie `Ctrl+]` jederzeit, um das neueste Artefakt aus dem Terminal erneut zu öffnen.

Claude wählt den Titel des Artefakts und ein Emoji für sein Browser-Tab-Symbol. Beide werden in Ihrer [Galerie von Artefakten](#share-an-artifact) auf claude.ai und in freigegebenen Links angezeigt, daher bitten Sie Claude, einen bestimmten Titel oder ein bestimmtes Symbol zu verwenden, wenn Sie einen möchten.

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

Um ein Artefakt aus einer anderen Sitzung zu aktualisieren, geben Sie Claude die URL des Artefakts und bitten Sie es, es zu überarbeiten. Ohne die URL erstellt eine neue Sitzung immer ein neues Artefakt, anstatt ein vorhandenes zu aktualisieren.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="share-an-artifact">
  Ein Artefakt teilen
</h2>

Ein neues Artefakt ist zunächst nur für Sie sichtbar. Um es zu teilen, öffnen Sie das Artefakt in Ihrem Browser und verwenden Sie die **Freigabe**-Kontrolle in der Seitenkopfzeile. Die Kopfzeile nennt Sie als Autor des Artefakts, sodass jeder, mit dem Sie es teilen, sehen kann, wer die Seite veröffentlicht hat. Sie verlinkt auch auf Ihre Galerie unter [claude.ai/code/artifacts](https://claude.ai/code/artifacts), die alle Artefakte auflistet, die Sie erstellt haben.

Mit wem Sie teilen können, hängt von Ihrem Plan ab:

* **Innerhalb Ihrer Organisation**: Bei Team- und Enterprise-Plänen können Sie Zugriff auf bestimmte Personen in Ihrer Organisation oder auf alle gewähren. Betrachter melden sich bei claude.ai als Mitglieder Ihrer Organisation an, um die Seite zu sehen.
* **Öffentlich**: Teilen Sie einen Link, den jeder im Internet öffnen kann, ohne sich bei claude.ai anmelden zu müssen. Bei Pro- und Max-Plänen ist ein öffentlicher Link die einzige Möglichkeit, ein Artefakt zu teilen. Bei Team- und Enterprise-Plänen ist die öffentliche Freigabe deaktiviert, bis ein Eigentümer [sie für die Organisation aktiviert](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Lassen Sie jemanden mit Ihnen bearbeiten
</h3>

Personen, mit denen Sie teilen, sind standardmäßig Betrachter: Sie sehen jede Version, die Sie veröffentlichen, können aber die Seite nicht ändern. Bei Team- und Enterprise-Plänen können Sie jemanden auch zum Bearbeiter machen. Fügen Sie im Freigabedialog eine Person hinzu und ändern Sie ihre Rolle von **Betrachter** zu **Bearbeiter**.

Ein Bearbeiter veröffentlicht neue Versionen auf die gleiche Weise wie Sie [das Artefakt aus einer anderen Sitzung aktualisieren](#update-an-artifact): Er oder sie gibt Claude die URL des Artefakts in seiner oder ihrer eigenen Sitzung, und Claude ruft den aktuellen Inhalt ab und veröffentlicht ihn mit seinen oder ihren Änderungen erneut. Jeder, der die Seite offen hat, sieht jede Aktualisierung live.

<h2 id="pull-live-data-with-mcp-connectors">
  Live-Daten mit MCP-Konnektoren abrufen
</h2>

Ein Artefakt kann [MCP-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) jedes Mal aufrufen, wenn jemand es anzeigt, sodass die Seite aktuelle Daten anstelle eines Snapshots aus der Sitzung anzeigt, in der sie erstellt wurde. Konnektoraufrufe aus Artefakten sind in den Plänen Pro, Max, Team und Enterprise verfügbar und erfordern Claude Code v2.1.209 oder später. In früheren Versionen veröffentlicht Claude die Seite mit den Daten, die die Sitzung während der Erstellung gesammelt hat.

Um eine Konnektoren-gestützte Seite zu erstellen, nennen Sie den Konnektor und die gewünschten Daten in Ihrer Eingabeaufforderung:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude deklariert, welche Konnektoren die Seite aufrufen darf, als Teil der Veröffentlichung, und die Seite kann keine Konnektoren außerhalb dieser Deklaration aufrufen. Nur Konnektoren aus Ihrem claude.ai-Konto kommen in Frage: Claude nennt sie in der Deklaration, und wenn jemand die Seite anzeigt, wird jeder Aufruf [über die eigene Verbindung des anzeigenden Kontos zu diesem Konnektor ausgeführt](#how-connector-calls-work-for-viewers). Lokale MCP-Server, die Sie in Claude Code konfigurieren, wie Server aus `.mcp.json`, können Daten liefern, während Claude die Seite erstellt, aber die veröffentlichte Seite kann sie nicht aufrufen.

Die Seite ruft Daten beim Laden ab und kann in einem Intervall aktualisiert werden oder wenn ein Betrachter ein Aktualisierungssteuerelement auf der Seite verwendet. Antworten werden im Browser des Betrachters zwischengespeichert, sodass eine erneut geöffnete Seite sofort aus den zwischengespeicherten Antworten gerendert wird und sich dann mit frischen Ergebnissen aktualisiert.

<h3 id="how-connector-calls-work-for-viewers">
  Funktionsweise von Konnektoraufrufen für Betrachter
</h3>

Wenn eine veröffentlichte Seite einen Konnektor aufruft, verwendet der Aufruf das Konto der Person, die die Seite anzeigt, nicht das Konto der Person, die sie veröffentlicht hat:

* **Jeder Betrachter verwendet seine eigenen Konnektoren**: Aufrufe erfolgen über die verbundenen Tools des anzeigenden Kontos, sodass zwei Personen, die dasselbe Dashboard öffnen, je nach dem, worauf ihre Konten zugreifen können, unterschiedliche Daten sehen können. Die Seite sieht niemals die Anmeldedaten von jemandem; claude.ai führt die Aufrufe im Namen der Seite aus.
* **Betrachter genehmigen den Zugriff zuerst**: claude.ai fragt jeden Betrachter um Genehmigung, bevor der erste Konnektoraufruf der Seite erfolgt. Ein Betrachter, der ablehnt oder keinen Konnektor verbunden hat, den die Seite verwendet, sieht die Seite immer noch ohne ihre Live-Abschnitte.
* **Aktionen verwenden auch das Konto des Betrachters**: Eine Seite kann Steuerelemente anbieten, die Konnektortools mit Nebenwirkungen aufrufen, z. B. das Posten einer Nachricht oder das Aktualisieren eines Problems. Die Aktion erfolgt über das Konto derjenigen Person, die das Steuerelement auswählt.

Wenn Sie planen, eine Konnektoren-gestützte Seite freizugeben, bitten Sie Claude, in jedem Live-Abschnitt eine Fallback-Nachricht einzufügen, die den benötigten Konnektor nennt. Ein Betrachter, dem die Verbindung fehlt, sieht dann, was verbunden werden muss, anstatt eines leeren Abschnitts.

Ein Artefakt, das Konnektoren aufruft, kann in keinem Plan über einen öffentlichen Link freigegeben werden. In den Plänen Team und Enterprise können Sie es privat halten oder [es innerhalb Ihrer Organisation freigeben](#share-an-artifact). In den Plänen Pro und Max, bei denen ein öffentlicher Link die einzige Möglichkeit zum Freigeben ist, bleibt ein Konnektoren-gestütztes Artefakt privat für Sie.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  Die Seite zeigt keine Live-Daten für einen Betrachter
</h3>

Wenn eine Konnektoren-gestützte Seite gerendert wird, aber ihre Live-Abschnitte für jemanden, mit dem Sie sie geteilt haben, leer bleiben, arbeiten Sie diese Ursachen durch:

* **Der Betrachter hat den Konnektor nicht verbunden**: Konnektoren sind pro Konto, daher benötigt jeder Betrachter seine eigene Verbindung zu jedem Konnektor, den die Seite aufruft. Sie können einen unter **Einstellungen > Konnektoren** auf claude.ai hinzufügen und dann die Seite neu laden.
* **Der Betrachter hat die Genehmigungsanfrage abgelehnt**: Eine Ablehnung gilt für den Rest dieses Seitenladegangs. Das Neuladen der Seite bringt die Genehmigungsanfrage zurück.
* **Konnektoraufrufe sind für die Organisation deaktiviert**: Ein Besitzer steuert den [**Artefakt-Konnektoren aktivieren**-Schalter](#control-connector-calls-from-artifacts) in den Admin-Einstellungen.

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

Ab Claude Code v2.1.183 wendet Claude eine integrierte Design-Fähigkeit an, wenn es ein Artefakt erstellt, sodass Seiten eine absichtliche Palette, Typografie und Layout ohne zusätzliche Aufforderung erhalten. Diese Fähigkeit sucht auch nach einem vorhandenen Design-System in Ihrem Projekt, bevor es sein eigenes auswählt. Um Artefakte konsistent mit dem Branding Ihres Produkts zu halten, notieren Sie Ihre Design-Token dort, wo Claude sie finden kann, wie in der [CLAUDE.md](/docs/de/memory) des Projekts oder einer Theme-Datei in Ihrem Repository:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude behandelt Ihr Design-System als höhere Priorität als seine eigenen Entscheidungen, und Ihren Prompt als höhere Priorität als beide. Die Überschrift und das Format oben sind ein Beispiel; jede klare Liste von Farben, Schriftarten und Abstände funktioniert.

<h2 id="page-constraints">
  Seitenbeschränkungen
</h2>

Jedes Artefakt ist eine einzelne, in sich geschlossene Seite. Claude Code umhüllt die Datei, die Sie veröffentlichen, in einer HTML-Dokumentshell und bedient sie unter einer strikten Content Security Policy (CSP), die formt, was die Seite tun kann.

| Beschränkung            | Auswirkung                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Keine externen Anfragen | Die CSP blockiert Skripte, Stylesheets, Schriftarten und Bilder, die von einem anderen Host geladen werden, zusammen mit `fetch`, XHR und WebSocket-Aufrufen. Claude inline CSS und JavaScript und bettet Bilder als Data-URIs ein, damit die Seite ohne externe Anfrage rendert. [Connector-Aufrufe](#pull-live-data-with-mcp-connectors) sind die Ausnahme: die Seite übergibt sie an claude.ai, das selbst den Netzwerkaufruf tätigt. |
| Kein Backend            | Ein Artefakt ist eine statische Seite. Es kann keine Daten speichern, die über ein Formular eingereicht werden, oder Betrachter selbst authentifizieren. Die einzige Möglichkeit, Daten abzurufen, wenn jemand die Seite anzeigt, ist das [Aufrufen von MCP-Connectoren](#pull-live-data-with-mcp-connectors), nicht eine eigene API.                                                                                                    |
| Einzelne Seite          | Relative Links werden nicht aufgelöst, da nichts neben der Seite bereitgestellt wird. Für mehrteilige Inhalte verwendet Claude In-Page-Anker anstelle von separaten Dateien.                                                                                                                                                                                                                                                             |
| Quelldateitypen         | Die veröffentlichte Datei muss `.html`, `.htm` oder `.md` sein. Markdown-Dateien werden als gestyltes HTML gerendert.                                                                                                                                                                                                                                                                                                                    |
| Gerenderte Größe        | Die gerenderte Seite muss 16 MiB oder kleiner sein. Große eingebettete Bilder sind die übliche Ursache, wenn eine Veröffentlichung aus Größengründen fehlschlägt.                                                                                                                                                                                                                                                                        |

Das Generieren eines Artefakts verwendet Ausgabe-Token wie jede andere Antwort, und eine gestylte Seite ist token-intensiver als derselbe Inhalt als Terminaltext. Inline-CSS, JavaScript für interaktive Steuerelemente und besonders Bilder, die als Data-URIs eingebettet sind, sind die Hauptbeiträge. Um die Token-Kosten eines Artefakts zu reduzieren:

* Bevorzugen Sie SVG oder HTML und CSS für Diagramme gegenüber eingebetteten Rasterbildern
* Lassen Sie Interaktivität weg, die Sie nicht benötigen
* Lassen Sie die Seite große Datensätze zusammenfassen, anstatt sie vollständig inline zu verwenden

<h2 id="availability">
  Verfügbarkeit
</h2>

Artefakte erfordern jede Bedingung unten. Wenn eine nicht erfüllt ist, schreibt Claude eine lokale HTML-Datei oder sagt, dass es nicht veröffentlichen kann.

| Anforderung             | Verfügbar wenn                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan                    | Pro, Max, Team oder Enterprise. Bei Pro- und Max-Plänen sind Artefakte privat für Sie, bis Sie sie freigeben, und es gilt keine Admin-Verwaltung. Bei Team-Plänen sind Artefakte standardmäßig aktiviert. Bei Enterprise-Plänen aktiviert ein Owner sie [](#manage-artifacts-for-your-organization) in den claude.ai-Admin-Einstellungen.                                                                                                                                          |
| Authentifizierung       | Die Sitzung wird durch ein claude.ai-Konto unterstützt: Melden Sie sich mit `/login` in der CLI oder Desktop-App an. Claude Tag-Sitzungen sind durch die Identität des Agenten angemeldet, daher ist kein Schritt erforderlich. Sitzungen, die einen API-Schlüssel, [Gateway-Token](/docs/de/llm-gateway) oder Cloud-Provider-Anmeldedaten verwenden, können nicht veröffentlichen.                                                                                                     |
| Modell-Provider         | Anthropic API. Nicht verfügbar auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry).                                                                                                                                                                                                                                                                                                    |
| Organisationsrichtlinie | Customer-managed encryption keys (CMEK), HIPAA und [Zero Data Retention](/docs/de/zero-data-retention) sind nicht für die Organisation aktiviert.                                                                                                                                                                                                                                                                                                                                       |
| Oberfläche              | Claude Code CLI Version 2.1.183 oder später oder die Claude Desktop-App Version 1.13576.0 oder später. [Claude Tag](https://claude.com/docs/claude-tag/overview)-Sitzungen können auch Artefakte veröffentlichen, wenn sowohl Claude Tag als auch Artefakte für die Organisation aktiviert sind. Standardmäßig aus in [Agent SDK](/docs/de/agent-sdk/overview), GitHub Action und MCP-Server-Kontexten und wenn [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars) gesetzt ist. |

<h2 id="disable-artifacts">
  Deaktivieren Sie Artefakte
</h2>

Um Artefakte für Ihre eigenen Sitzungen unabhängig von der Einstellung Ihrer Organisation auszuschalten, verwenden Sie eine der folgenden Optionen:

| Methode                               | Einstellung                                      |
| :------------------------------------ | :----------------------------------------------- |
| [Einstellungsdatei](/docs/de/settings)     | `"disableArtifact": true`                        |
| [Umgebungsvariable](/docs/de/env-vars)     | `CLAUDE_CODE_DISABLE_ARTIFACT=1`                 |
| [Berechtigungsregel](/docs/de/permissions) | Fügen Sie `Artifact` zu `permissions.deny` hinzu |

<h2 id="manage-artifacts-for-your-organization">
  Verwalten Sie Artefakte für Ihre Organisation
</h2>

Inhaber bei Team- und Enterprise-Plänen steuern Artefakte aus [claude.ai Admin-Einstellungen](https://claude.ai/admin-settings/claude-code). Der Artefaktinhalt wird auf von Anthropic betriebener Infrastruktur gespeichert und ist nur für authentifizierte Mitglieder der veröffentlichenden Organisation sichtbar, es sei denn, das Artefakt wird [öffentlich freigegeben](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Aktivieren oder deaktivieren Sie Artefakte
</h3>

Um Artefakte für die gesamte Organisation zu aktivieren oder zu deaktivieren, gehen Sie zu **Einstellungen > Claude Code > Funktionen** und verwenden Sie den Umschalter **Artefakte**. Bei Enterprise-Plänen mit rollenbasierter Zugriffskontrolle können Sie Artefakte zusätzlich auf bestimmte Rollen beschränken: Gehen Sie zu **Einstellungen > Rollen**, bearbeiten Sie eine Rolle und setzen Sie die Berechtigung **Artefakte** unter der Gruppe **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Steuern Sie Connector-Aufrufe aus Artefakten
</h3>

[Connector-Aufrufe aus Artefakten](#pull-live-data-with-mcp-connectors) haben ihren eigenen Umschalter, getrennt vom Umschalter **Artefakte**, der Artefakte ein- oder ausschaltet. Gehen Sie zu [**Einstellungen > Funktionen**](https://claude.ai/admin-settings/capabilities) und verwenden Sie den Umschalter **Artefakt-Connectoren aktivieren**. Der gleiche Umschalter regelt Connector-Aufrufe aus Artefakten, die in claude.ai-Gesprächen erstellt wurden, weshalb er unter **Einstellungen > Funktionen** statt unter **Einstellungen > Claude Code** angeordnet ist.

<h3 id="control-public-sharing">
  Steuern Sie die öffentliche Freigabe
</h3>

Die öffentliche Freigabe ist standardmäßig bei Team- und Enterprise-Plänen deaktiviert, sodass Mitglieder Artefakte nur innerhalb der Organisation freigeben können, bis ein Inhaber sie aktiviert. Um Mitgliedern zu ermöglichen, Artefakte auf öffentliche Links zu veröffentlichen, die jeder ohne Anmeldung anzeigen kann, gehen Sie zu **Einstellungen > Claude Code > Funktionen** und aktivieren Sie **Externe Freigabe** unter dem Umschalter **Artefakte**. Wenn Sie ihn wieder ausschalten, wird der Zugriff über vorhandene öffentliche Links blockiert, ohne die Zielgruppe jedes Artefakts zu ändern; der Zugriff wird wiederhergestellt, wenn Sie ihn erneut aktivieren.

<h3 id="set-a-retention-policy">
  Legen Sie eine Aufbewahrungsrichtlinie fest
</h3>

Um festzulegen, wie lange Artefakte vor automatischer Löschung aufbewahrt werden, gehen Sie zu **Einstellungen > Datenschutz- und Datenschutzkontrollen**. Sie können separate Aufbewahrungszeiträume für Artefakte festlegen, die noch privat für ihren Autor sind, und Artefakte, die freigegeben wurden.

<h3 id="review-the-audit-log">
  Überprüfen Sie das Audit-Protokoll
</h3>

Das Veröffentlichen, Freigeben und Löschen eines Artefakts wird jeweils in dem Audit-Protokoll Ihrer Organisation unter den `claude_artifact_*`-Ereignistypen angezeigt, der gleichen Familie, die für Artefakte verwendet wird, die in claude.ai-Gesprächen erstellt werden.

<h3 id="allowlist-the-viewer-domain">
  Allowlist die Viewer-Domain
</h3>

Der Viewer auf claude.ai lädt jedes Artefakt aus einer Sandbox-`*.claudeusercontent.com`-Origin. Wenn Ihre Organisation den ausgehenden Netzwerkzugriff einschränkt, fügen Sie diese Domain zu Ihrer Allowlist neben `claude.ai` hinzu. Siehe [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements) für die vollständige Liste.

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
