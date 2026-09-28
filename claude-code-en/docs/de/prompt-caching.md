> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wie Claude Code Prompt Caching nutzt

> Claude Code verwaltet Prompt Caching automatisch. Erfahren Sie, warum ein Modellwechsel einen langsamen unkachedten Turn auslöst, was `/compact` kostet, warum CLAUDE.md-Änderungen mid-session nicht angewendet werden, und wie Sie Ihre Cache-Hit-Rate überprüfen.

Prompt Caching macht Claude Code schneller und kostengünstiger. Ohne Caching würde die API Ihre vollständige Historie bei jedem Turn neu verarbeiten. Mit Caching nutzt sie das bereits Verarbeitete wieder, berechnet das erneute Lesen zum [Cached-Token-Satz](https://platform.claude.com/docs/en/about-claude/pricing) ab und verarbeitet vollständig nur das, was sich geändert hat.

Claude Code verwaltet Prompt Caching für Sie, es sei denn, Sie [deaktivieren es](#disable-prompt-caching). Es ist dennoch nützlich zu verstehen, wie Prompt Caching funktioniert, da einige Aktionen den Cache ungültig machen und die nächste Antwort langsamer und teurer machen, während er sich neu aufbaut. Diese Seite behandelt, welche Aktionen das sind, warum einige Einstellungen auf einen Neustart warten, um angewendet zu werden, und wie Sie die Cache-Leistung überprüfen, wenn die Nutzung hoch aussieht.

<h2 id="how-the-cache-is-organized">
  Wie der Cache organisiert ist
</h2>

Jedes Mal, wenn Sie eine Nachricht in Claude Code senden, wird eine neue API-Anfrage gestellt. Das Modell merkt sich nichts zwischen Anfragen, daher sendet Claude Code den vollständigen Kontext erneut: die Systemaufforderung, Ihren Projektkontext, jede vorherige Nachricht und jedes Werkzeugergebnis sowie Ihre neue Nachricht. Neuer Inhalt wird am Ende angefügt, was bedeutet, dass der größte Teil jeder Anfrage identisch mit der vorherigen ist. Prompt Caching ist die Methode, mit der die API vermeidet, den Teil zu verarbeiten, der sich nicht geändert hat.

Die API speichert durch Abgleich des Anfangs jeder Anfrage, genannt das Präfix, gegen kürzlich verarbeitete Inhalte. Bei einem normalen Zug ist das Präfix die gesamte vorherige Anfrage und nur der neueste Austausch ist neu. Der Abgleich ist exakt, daher wird alles nach dem Präfix neu berechnet, wenn sich etwas im Präfix ändert. Es gibt kein Pro-Datei- oder Pro-Segment-Caching. Siehe [wie Prompt Caching funktioniert](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works) in der API-Referenz für den zugrunde liegenden Mechanismus.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Vier Züge werden als wachsende horizontale Balken angezeigt. Die Anfrage jedes Zugs enthält alles aus dem vorherigen Zug plus den neuesten Austausch am Ende. Bei den Zügen zwei und drei wird das unveränderte Präfix aus dem Cache gelesen und nur der neue Austausch wird verarbeitet. Bei Zug vier hat sich die Systemaufforderung geändert, daher stimmt das Präfix nicht mehr überein und die gesamte Anfrage wird neu verarbeitet und geschrieben." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Vier Züge werden als wachsende horizontale Balken angezeigt. Die Anfrage jedes Zugs enthält alles aus dem vorherigen Zug plus den neuesten Austausch am Ende. Bei den Zügen zwei und drei wird das unveränderte Präfix aus dem Cache gelesen und nur der neue Austausch wird verarbeitet. Bei Zug vier hat sich die Systemaufforderung geändert, daher stimmt das Präfix nicht mehr überein und die gesamte Anfrage wird neu verarbeitet und geschrieben." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Um das Beste aus dem Präfix-Abgleich herauszuholen, ordnet Claude Code jede Anfrage so, dass Inhalte, die sich zwischen Zügen selten ändern, zuerst kommen:

| Ebene              | Inhalt                                                  | Ändert sich wenn                                        |
| ------------------ | ------------------------------------------------------- | ------------------------------------------------------- |
| Systemaufforderung | Kernanweisungen, Werkzeugdefinitionen                   | Der Satz der geladenen Werkzeugdefinitionen ändert sich |
| Projektkontext     | CLAUDE.md, automatisches Gedächtnis, unscoped-Regeln    | Sitzung startet, oder nach `/clear` oder `/compact`     |
| Konversation       | Ihre Nachrichten, Claudes Antworten, Werkzeugergebnisse | Jeder Zug                                               |

Eine Änderung der Konversationsebene lässt die Systemaufforderung und den Projektkontext zwischengespeichert. Eine Änderung der Systemaufforderung invalidiert alles, da der gesamte spätere Inhalt nun hinter einem anderen Präfix sitzt. Die dritte Spalte gibt häufige Auslöser statt einer vollständigen Liste an, und die folgenden Abschnitte behandeln den vollständigen Satz.

Die Präfix-Abgleich-Regel erklärt die meisten Verhaltensweisen auf dieser Seite. [Plan Mode](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) und [Skill-Laden](/docs/de/skills) hängen beispielsweise ihre Anweisungen als Konversationsnachrichten an, sodass das zwischengespeicherte Präfix intakt bleibt.

Zwei Einstellungen erscheinen nicht in der Ebenen-Tabelle, beeinflussen aber dennoch, was zwischengespeichert bleibt:

* **Modell**: Jedes Modell hat seinen eigenen Cache. Das Wechseln von Modellen berechnet die gesamte Anfrage neu, auch wenn der Inhalt identisch ist. Siehe [Modelle wechseln](#switching-models) unten.
* **Aufwandsstufe**: Bei den meisten Modellen hat jede Aufwandsstufe ihren eigenen Cache, daher wird die gesamte Anfrage neu berechnet, wenn Sie die Aufwandsstufe während einer Sitzung ändern. Bei Opus 5.5 und Fable 5.1 mit einem API-Schlüssel oder einem Claude-Abonnement bleibt der Cache standardmäßig intakt. Siehe [Aufwandsstufe ändern](#changing-effort-level) unten.

<Tip>
  Wählen Sie Ihr Modell und Ihre Aufwandsstufe am Anfang einer Sitzung aus, und speichern Sie dann `/compact` für natürliche Pausen zwischen Aufgaben. Je weniger Änderungen Sie während einer Aufgabe vornehmen, desto höher ist Ihre Cache-Hit-Rate.
</Tip>

<h3 id="where-the-cache-lives">
  Wo der Cache lebt
</h3>

Das Caching erfolgt serverseitig in der Infrastruktur, die Ihr Modell bereitstellt. Wo das ist, hängt davon ab, wie Sie sich authentifizieren:

* **API-Schlüssel, Claude-Abonnement oder [Claude Platform on AWS](/docs/de/claude-platform-on-aws)**: Der Cache lebt in der Infrastruktur von Anthropic und wird über die [Claude API](https://platform.claude.com/docs) aufgerufen
* **Amazon Bedrock oder Google Cloud's Agent Platform**: Der Cache lebt in der Serving-Infrastruktur Ihres Cloud-Anbieters
* **Microsoft Foundry**: Hängt von der [Hosting-Option](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) der Bereitstellung ab. Auf Azure bereitgestellte Deployments werden auf Azure-Infrastruktur bereitgestellt; auf Anthropic bereitgestellte Deployments werden auf der Infrastruktur von Anthropic bereitgestellt
* **Benutzerdefinierte `ANTHROPIC_BASE_URL` oder [LLM-Gateway](/docs/de/llm-gateway)**: Der Cache lebt dort, wo Ihre Anfragen weitergeleitet werden, und ob Caching funktioniert, hängt vom Gateway ab

Claude Code hängt auch Systemkontext während der Konversation an, wie z. B. Dateiänderungsmitteilungen, und markiert diesen Block zum Caching auf jedem Anbieter und jeder Verbindung, es sei denn, Sie setzen [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/de/llm-gateway-protocol#disable-pre-release-capabilities), in welchem Fall dieser Block unzwischengespeichert gesendet wird.

Am eigenen Endpunkt des Anbieters, Amazon Bedrock und seinem [Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint), Google Cloud's Agent Platform und Microsoft Foundry speichern den Block auf die gleiche Weise wie die Claude API.

Wenn Ihre Anfragen durch ein [LLM-Gateway](/docs/de/llm-gateway), eine benutzerdefinierte `ANTHROPIC_BASE_URL` oder eine Cloud-Provider-Basis-URL-Überschreibung wie [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/de/env-vars) geleitet werden, hängt das, was zwischengespeichert bleibt, davon ab, wie das Gateway die [`cache_control`-Marker](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints) handhabt, die Claude Code sendet:

* **Leitet sie unverändert weiter**: Der Block und Ihre Konversation speichern auf die gleiche Weise wie am eigenen Endpunkt des Anbieters.
* **Lehnt die markierte Anfrage mit einem `400`-Fehler ab, der `cache_control` nennt**: Claude Code sendet die Anfrage erneut mit dem Marker, der vom Block auf Ihre letzte Konversationsnachricht verschoben wird, und behält ihn dort für den Rest der Konversation. Der Block wird als unzwischengespeicherte Eingabe abgerechnet; Ihre Konversation bleibt zwischengespeichert.
* **Entfernt die Marker bei erfolgreicher Rückgabe**: Ihr gesamter Konversationsverlauf wird auf jedem Zug als unzwischengespeicherte Eingabe abgerechnet. Ein Gateway, das Block-Form-Systeminhalt in einen einfachen String konvertiert, lässt den Marker auf die gleiche Weise fallen.

Für das, was jeder Anbieter speichert und verarbeitet, siehe [Datennutzung](/docs/de/data-usage). Wo immer der Cache lebt, Einträge verfallen nach einer Inaktivitätsperiode, und [Cache-Lebensdauer](#cache-lifetime) unten behandelt die TTL und wie man sie verlängert.

<h2 id="actions-that-invalidate-the-cache">
  Aktionen, die den Cache ungültig machen
</h2>

Diese Aktionen führen dazu, dass die nächste Anfrage einen Teil oder den gesamten Cache verfehlt. Sie sehen einen einmalig langsameren, teureren Turn, danach wird das neue Präfix zwischengespeichert. Die meisten davon sind vermeidbar, wenn Sie während einer Aufgabe wissen, dass sie Kosten verursachen. Ein Modellwechsel kann sich kostenlos anfühlen, bis Sie den langsameren Turn bemerken, der folgt.

* [Modelle wechseln](#switching-models)
* [Anstrengungsstufe ändern](#changing-effort-level)
* [Schnellmodus aktivieren](#turning-on-fast-mode)
* [MCP-Server verbinden oder trennen](#connecting-or-disconnecting-an-mcp-server)
* [Plugin aktivieren oder deaktivieren](#enabling-or-disabling-a-plugin)
* [Ein ganzes Tool verweigern](#denying-an-entire-tool)
* [Konversation komprimieren](#compacting-the-conversation)
* [Viele Bilder sammeln](#accumulating-many-images)
* [Claude Code aktualisieren](#upgrading-claude-code)

<h3 id="switching-models">
  Modelle wechseln
</h3>

Jedes Modell hat seinen eigenen Cache. Das Wechseln mit [`/model`](/docs/de/model-config#setting-your-model) bedeutet, dass die nächste Anfrage die gesamte Konversationshistorie ohne Cache-Treffer liest, obwohl der Inhalt identisch ist.

Wenn Sie `/model` im Terminal ausführen, fordert Claude Code Sie auf, den Wechsel nur zu bestätigen, während der Cache noch warm ist und das neue Modell nicht das ist, das die letzte Antwort produziert hat. Der Cache bleibt warm für einen [Cache-TTL](#cache-lifetime) nach dem letzten Request, den Claude Code in dieser Konversation gesendet hat, oder nach der letzten Antwort von Claude. Sobald diese Zeit verstrichen ist, ist der Cache abgelaufen, sodass Claude Code ohne Nachfrage wechselt.

Vor v2.1.238 überprüfte Claude Code die Cache-TTL nicht und fragte auch nach Ablauf des Cache.

Sie können diese Bestätigung auch mit einem [PreModelSwitch Hook](/docs/de/hooks#premodelswitch-decision-control) erzwingen oder überspringen.

Die [`opusplan` Modelleinstellung](/docs/de/model-config#opusplan-model-setting) wird während des Plan-Modus zu Opus und während der Ausführung zu Sonnet aufgelöst, sodass jeder Plan-Modus-Toggle ein Modellwechsel ist und einen frischen Cache startet.

[Automatisches Modell-Fallback](/docs/de/model-config#automatic-model-fallback) auf Fable-Modellen, Opus 5.5 und Opus 5 ist auch ein Modellwechsel. Wenn ein Sicherheitsklassifizierer eine Anfrage in einer Kategorie mit einem Fallback-Modell kennzeichnet, führt Claude Code die Anfrage auf diesem Modell erneut aus und die Sitzung wird dort fortgesetzt.

Wenn die Frontmatter eines Skills oder Befehls ein [`model`](/docs/de/skills#frontmatter-reference) benennt, das nicht das aktuelle Modell der Sitzung ist, ist dieser Turn auch ein Modellwechsel: die nächste Anfrage liest die gesamte Konversationshistorie ohne Cache-Treffer. Das Sitzungsmodell wird bei Ihrer nächsten Eingabeaufforderung fortgesetzt. Ein `context: fork` Skill setzt stattdessen das [Modell des abgespaltenen Subagenten](/docs/de/skills#run-skills-in-a-subagent).

<h3 id="changing-effort-level">
  Anstrengungsstufe ändern
</h3>

Bei den meisten Modellen bedeutet das Ändern der [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) während einer Sitzung, dass die nächste Anfrage die gesamte Konversationshistorie ohne Cache-Treffer liest. Während der Cache noch warm ist, fordert Claude Code Sie auf, die Änderung zuerst zu bestätigen.

Bei Opus 5.5 und Fable 5.1 mit einem API-Schlüssel oder Claude-Abonnement behält das Ändern der Anstrengung den Cache, und Claude Code wendet die neue Stufe ohne Nachfrage an. Dies gilt nicht für Amazon Bedrock, Google Cloud's Agent Platform oder ein [Claude Apps Gateway](/docs/de/claude-apps-gateway), oder wenn Sie [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/de/llm-gateway-protocol#disable-pre-release-capabilities) setzen oder Ihre Organisation eine HIPAA-Konfiguration hat.

Vor v2.1.260 machte das Ändern der Anstrengung auf Fable 5.1 mit einem API-Schlüssel oder Claude-Abonnement auch den Cache ungültig.

<h3 id="turning-on-fast-mode">
  Schnellmodus aktivieren
</h3>

Das Aktivieren des [Schnellmodus](/docs/de/fast-mode) fügt einen Request-Header hinzu, der Teil des Cache-Schlüssels ist, sodass die erste Anfrage, die Claude Code mit aktiviertem Schnellmodus sendet, die gesamte Konversationshistorie ohne Cache-Treffer liest. Claude Code setzt diesen Header einmal, wenn ein Turn beginnt, und behält ihn für den gesamten Turn, sodass wenn Sie den Schnellmodus aktivieren, während Claude arbeitet, der Cache-Miss des Headers bei der ersten Anfrage Ihres nächsten Turns auftritt. Diese nicht zwischengespeicherten Input-Token werden zu [Schnellmodus-Raten](/docs/de/fast-mode#understand-the-cost-tradeoff) abgerechnet, weshalb das Aktivieren am Anfang einer Sitzung weniger kostet als das Aktivieren tief in einer langen Sitzung. Wenn Ihr aktuelles Modell den Schnellmodus nicht unterstützt, führt das Aktivieren des Schnellmodus auch zu einem [Modellwechsel](#switching-models), und dieser Wechsel startet von selbst einen frischen Cache ab der nächsten Anfrage im laufenden Turn.

Die Kosten fallen einmal pro Konversation an. Nach dem ersten Schnellmodus-Turn sendet Claude Code weiterhin den Header und variiert nur die Geschwindigkeitseinstellung der Anfrage, die nicht Teil des Cache-Schlüssels ist. Das Ausschalten des Schnellmodus, das [automatische Fallback auf Standardgeschwindigkeit](/docs/de/fast-mode#handle-rate-limits) nach einem Rate Limit und das spätere Wiedereinschalten behalten alle den Cache. Wenn Sie [während einer Sitzung die Nutzungsguthaben aufbrauchen](/docs/de/fast-mode#handle-rate-limits), versucht Claude Code jeden abgelehnten Schnellmodus-Request auf die gleiche Weise mit Standardgeschwindigkeit erneut, sodass dieses Fallback auch den Cache behält. `/clear` und `/compact` setzen dies zurück, da sie den Cache an diesen Punkten ohnehin neu aufbauen.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  MCP-Server verbinden oder trennen
</h3>

Tool-Definitionen befinden sich in der System-Prompt-Schicht, sodass der Cache ungültig wird, wenn sich die Menge der Tool-Definitionen in der Anfrage zwischen Turns ändert. Das Umschalten des [Advisor-Tools](/docs/de/advisor) ist eine Ausnahme: seine Definition befindet sich nach dem Cache-Breakpoint, sodass das Aktivieren oder Deaktivieren von `/advisor` das zwischengespeicherte Präfix intakt hält. Ob eine [MCP-Server](/docs/de/mcp)-Änderung dies tut, hängt davon ab, ob ihre Tools durch [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) aufgeschoben oder in das Präfix geladen werden:

* **Aufgeschobene Tools**, die Standardeinstellung auf unterstützten Modellen: Ein Server, der sich verbindet, trennt oder seine Tool-Liste ändert, hängt nur neue Inhalte an und stört nichts, das bereits zwischengespeichert ist.
* **Tools, die in das Präfix geladen werden**: Jede Änderung daran macht den Cache ungültig. Dies geschieht, wenn [Tool-Suche nicht verfügbar oder deaktiviert ist](/docs/de/mcp#configure-tool-search), z. B. auf Google Cloud's Agent Platform-Modellen vor der Claude 4.5-Generation, mit einem benutzerdefinierten `ANTHROPIC_BASE_URL` Gateway oder auf einer Microsoft Foundry [auf Azure gehosteten Bereitstellung](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), sobald Claude Code erkennt, dass die Bereitstellung Tool-Suche ablehnt. Es geschieht auch für einen Server oder ein Tool, das als [`alwaysLoad`](/docs/de/mcp#exempt-a-server-from-deferral) gekennzeichnet ist, und für Definitionen, die durch [schwellenwertbasiertes Laden](/docs/de/mcp#configure-tool-search) vorne gehalten werden.

Wenn Tools in das Präfix geladen werden, ist die häufigste Ursache einer Ungültigmachung ein Server, der sich während einer Sitzung verbindet oder trennt, was ohne Ihre Aktion geschehen kann: Der Prozess eines Stdio-Servers wird beendet, eine HTTP-Sitzung läuft ab oder ein Server [verbindet sich automatisch nach einem vorübergehenden Fehler wieder](/docs/de/mcp#automatic-reconnection). Ein verbundener Server kann auch ein [dynamisches Tool-Update](/docs/de/mcp#dynamic-tool-updates) pushen, das seine Tool-Liste ändert.

Das Bearbeiten Ihrer MCP-Konfiguration ändert den Cache nicht von selbst. Die neue Konfiguration wird erst nach einem Neustart wirksam, wenn sich der Server verbindet oder trennt.

<h3 id="enabling-or-disabling-a-plugin">
  Plugin aktivieren oder deaktivieren
</h3>

Wenn Sie ein [Plugin](/docs/de/plugins/overview) aktivieren oder deaktivieren, hängt das, was die Änderung kostet, davon ab, welche Komponententypen das Plugin bereitstellt. Die folgenden Fälle behandeln jeden Komponententyp, wann Claude Code die Änderung anwendet und was passiert, wenn Sie ein Plugin später in derselben Sitzung wieder deaktivieren.

<h4 id="plugin-components-that-keep-the-cache">
  Plugin-Komponenten, die den Cache behalten
</h4>

Claude Code macht den Cache für die Skills, Befehle, Agenten, Hooks, Monitore oder Themes eines Plugins nie ungültig. Es hängt ihren Inhalt nach der bestehenden Konversation an, sodass die nächste Anfrage für diesen Inhalt bezahlt und alles davor immer noch aus dem Cache liest.

<h4 id="plugins-that-provide-mcp-servers">
  Plugins, die MCP-Server bereitstellen
</h4>

Wenn Sie ein Plugin aktivieren oder deaktivieren, das [MCP-Server](/docs/de/plugins/components#mcp-servers) bereitstellt, folgt Claude Code den gleichen Regeln wie beim [Verbinden oder Trennen eines MCP-Servers](#connecting-or-disconnecting-an-mcp-server):

* Wenn Claude Code die Tools des Servers aufschiebt, behält es den Cache.
* Wenn Claude Code sie in das Präfix lädt, liest die nächste Anfrage die gesamte Konversation erneut.

<h4 id="code-intelligence-plugins">
  Code-Intelligence-Plugins
</h4>

Wenn Sie ein [Code-Intelligence-Plugin](/docs/de/plugins/code-intelligence) aktivieren, erhält Claude das [LSP-Tool](/docs/de/tools-reference#lsp-tool-behavior).

<h4 id="when-plugin-changes-apply">
  Wann Plugin-Änderungen angewendet werden
</h4>

Eine Änderung, die Sie im `/plugin`-Menü vornehmen, wird durch [`/reload-plugins`](/docs/de/plugins/cli-reference#reload-plugins) durchgeführt, das Claude Code für Sie ausführt, wenn Sie das Menü schließen. Sie zahlen die Kosten, ob angehängte Ankündigungen oder ein vollständiges erneutes Lesen, beim ersten Turn nach Anwendung der Änderung. Claude Code kann eine Änderung auch selbst anwenden:

* Für ein Plugin mit einer `command`-Quelle kann Claude Code [das Plugin selbst neu laden](/docs/de/plugins/loading#when-a-command-source-re-runs).
* Wenn Sie [ein Plugin aus der `/plugin`-Schnittstelle installieren](/docs/de/plugins/install#install-a-plugin), kann Claude Code es während der Installation aktivieren. Die Installationszusammenfassung teilt Ihnen mit, ob es das getan hat.
* Wenn Sie [die Sitzung mit `/cd` verschieben](/docs/de/permissions#move-the-session-to-another-directory) auf v2.1.246 oder später, wendet Claude Code die Plugins an, die die Einstellungen des neuen Verzeichnisses aktivieren, als Teil des Verschiebens, ohne die vollständige Neulesens-Warnung, die ein `/reload-plugins` hält.
* In interaktiven Sitzungen, wenn Sie ein Plugin in einem [Ordner von Plugins](/docs/de/plugins/create#load-a-directory-or-archive-for-one-session) hinzufügen oder entfernen, den Sie mit `--plugin-dir` übergeben haben, wird die Änderung sofort angewendet. Wenn die Anwendung ein vollständiges erneutes Lesen auslösen würde, hält Claude Code die Änderung stattdessen und zeigt einen Hinweis an, um `/reload-plugins` auszuführen. Erfordert Claude Code v2.1.265 oder später.

Wenn `/reload-plugins` ausgeführt wird und das Neuladen ein vollständiges erneutes Lesen auslösen würde, zeigt Claude Code eine Warnung an und wendet das Neuladen nicht an. Führen Sie `/reload-plugins --force` aus, um es trotzdem anzuwenden.

`/reload-plugins` wird auch in Sitzungen ohne interaktives Terminal ausgeführt, z. B. die Desktop-App, das Agent SDK und [nicht-interaktiver Modus](/docs/de/headless) mit `-p`, wenn Sie es direkt in die Sitzung eingeben. Erfordert Claude Code v2.1.260 oder später.

In diesen Sitzungen wendet das Neuladen alles außer Plugin-MCP-Server-Änderungen an, die [in Ihrer nächsten Sitzung wirksam werden](/docs/de/plugins/cli-reference#reload-plugins) und daher nie während einer Sitzung ein vollständiges erneutes Lesen kosten.

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  Plugins, die Sie in einer Sitzung aktivieren und dann deaktivieren
</h4>

Wenn Sie ein Plugin deaktivieren, das Sie früher in der Sitzung aktiviert haben, stellt Claude Code die vorherige Request-Form wieder her. Wenn dieses Präfix sich noch innerhalb seiner [Cache-Lebensdauer](#cache-lifetime) befindet, liest die nächste Anfrage stattdessen den älteren Cache-Eintrag, anstatt ihn neu zu erstellen.

<h3 id="denying-an-entire-tool">
  Ein ganzes Tool verweigern
</h3>

Das Hinzufügen eines bloßen Tool-Namens wie `Bash` oder `WebFetch` als [Deny-Regel](/docs/de/permissions#manage-permissions) bedeutet, dass Claude dieses Tool ab Ihrer nächsten Anfrage nicht aufrufen kann, ob Sie die Regel durch `/permissions` hinzufügen oder durch [direktes Bearbeiten einer Einstellungsdatei](/docs/de/settings#when-edits-take-effect). Das schließt eine Regel ein, die Sie durch `/permissions` in der Mitte eines Turns hinzufügen.

Wenn [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) aktiv ist, was auf unterstützten Modellen die Standardeinstellung ist, ändern sich die Tool-Definitionen der Anfrage nicht und das zwischengespeicherte Präfix bleibt erhalten. Wenn Tool-Suche nicht verfügbar oder deaktiviert ist, entfernt Claude Code die Definition aus der nächsten Anfrage, was den Cache ungültig macht, und das Entfernen der Regel später auch.

Nur eine Deny-Regel, die in der Tool-Name-Position passt, blockiert ein Tool auf diese Weise: ein bloßer Tool-Name, die äquivalente `Bash(*)` Form oder ein [Tool-Name-Glob](/docs/de/permissions#tool-name-wildcards) wie `"*"`. Ein Glob, der nur MCP-Tools passt, z. B. `"mcp__*"`, blockiert diese Tools auf die gleiche Weise. Scoped Deny-Regeln wie `Bash(rm *)` und alle Allow- und Ask-Regeln ändern nicht, welche Tools Claude sieht. Claude Code überprüft sie, wenn Claude einen Aufruf versucht, wobei das Präfix intakt bleibt.

<h3 id="compacting-the-conversation">
  Konversation komprimieren
</h3>

[Komprimierung](/docs/de/context-window#what-survives-compaction) ersetzt Ihren Nachrichtenverlauf durch eine Zusammenfassung. Dies macht die Konversationsschicht absichtlich ungültig, da die nächste Anfrage einen neuen, kürzeren Verlauf hat, der kein Präfix mit dem alten teilt. Claude Code verwendet die System-Prompt-Schicht wieder, es sei denn, die Konversation wurde [fortgesetzt, während ein System-Prompt beibehalten wurde, der sich sonst geändert hätte](#resuming-a-session); in diesem Fall wechselt die erste Komprimierung zum aktuellen Prompt und diese Schicht wird einmal neu erstellt. Es lädt Projektkontext von der Festplatte neu, was nur Cache-Treffer hat, wenn CLAUDE.md und Memory seit Sitzungsbeginn unverändert sind.

Um die Zusammenfassung zu erstellen, sendet Claude Code eine separate Anfrage mit dem gleichen System-Prompt, den gleichen Tools und dem gleichen Verlauf wie Ihre Konversation, plus eine Zusammenfassungsanweisung, die als letzte Benutzernachricht angehängt wird. Während der Cache warm ist, liest diese Anfrage Ihr Präfix aus dem Cache, sodass ein `/compact` während einer Sitzung einen Bruchteil dessen kostet, was die Kontextgröße nahelegt, und verbringt die meiste Zeit mit der Generierung der Zusammenfassung.

Nach einer Pause länger als die [Cache-Lebensdauer](#cache-lifetime) gibt es keinen Cache mehr zum Lesen, sodass die Zusammenfassungsanfrage den vollständigen Verlauf als nicht zwischengespeicherte Eingabe erneut verarbeitet. Dies ist der Grund, warum `/compact` am meisten kostet, wenn Sie [eine alte Sitzung fortsetzen](/docs/de/sessions#resume-from-a-summary). In beiden Fällen, warm und kalt, erstellt der Turn nach der Komprimierung den Konversations-Cache nur für die viel kürzere Zusammenfassung neu, sodass dieser Turn nicht der langsame Teil ist.

<Tip>
  Komprimierung funktioniert zu Ihrem Vorteil, wenn der Kontext, den Sie verwerfen, Inhalte sind, die Sie nicht mehr benötigen. Um zu wählen, wann sein Overhead auftritt, führen Sie `/compact` an einer natürlichen Pause in Ihrer Arbeit aus, z. B. zwischen Aufgaben, anstatt zu warten, bis Auto-Komprimierung während einer Aufgabe ausgelöst wird. Wenn Sie einen Weg gehen, den Sie vollständig aufgeben möchten, [`/rewind`](#rewinding-the-conversation) stattdessen zu einem früheren Turn. Das Zurückspulen schneidet auf ein Präfix zurück, das bereits zwischengespeichert ist, anstatt ein neues zu erstellen, wie es die Komprimierung tut.
</Tip>

<h3 id="accumulating-many-images">
  Viele Bilder sammeln
</h3>

Die API begrenzt, wie viele Bilder und PDFs jede Anfrage tragen kann. Für die aktuellen Zahlen siehe [Request-Limits](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits) in der API-Dokumentation. Claude Code begrenzt auch die Gesamtgröße der Bilder und PDFs in einer Anfrage, sodass große Screenshots die Grenze mit weniger Bildern erreichen als kleine.

Wenn die nächste Anfrage eines der Limits überschreiten würde, entfernt Claude Code einen Batch der ältesten Bilder und PDFs aus dem, was es sendet, was Platz für mehr schafft, bevor es wieder welche entfernen muss. Claude kann die entfernten Bilder nicht mehr sehen. Wenn Claude eines davon wieder benötigt, teilen Sie es erneut.

Das Entfernen von Bildern ändert die Nachrichten, die sie hielten, sodass die nächste Anfrage die Konversation ab der frühesten dieser Nachrichten erneut verarbeitet. Weil Claude Code einen Batch auf einmal entfernt, sehen Sie einen langsameren Turn pro Batch, anstatt einen mit jedem neuen Screenshot.

<h3 id="upgrading-claude-code">
  Claude Code aktualisieren
</h3>

Eine neue Claude Code-Version aktualisiert normalerweise den System-Prompt oder die Tool-Definitionen, sodass die erste Konversation, die Sie nach einem Upgrade starten, ihren Cache von oben aufbaut. [Auto-Update](/docs/de/setup#auto-updates) lädt neue Versionen im Hintergrund herunter, wendet sie aber erst beim nächsten Start an, nie während einer Sitzung, sodass Sie dies als einen nicht zwischengespeicherten ersten Turn nach dem Neustart sehen, anstatt als eine Überraschung während einer Sitzung. Setzen Sie `DISABLE_AUTOUPDATER=1`, um zu kontrollieren, wann Upgrades angewendet werden.

<Note>
  Für das, was es kostet, eine Konversation fortzusetzen, die Sie vor dem Upgrade gestartet haben, siehe [Sitzung fortsetzen](#resuming-a-session).
</Note>

<h2 id="actions-that-keep-the-cache">
  Aktionen, die den Cache beibehalten
</h2>

Diese Aktionen hängen entweder am Ende des Gesprächs an oder berühren die Anfrage überhaupt nicht. Einige von ihnen, wie das Bearbeiten von CLAUDE.md, behalten den Cache aus demselben Grund, aus dem die Änderung die laufende Sitzung erst nach `/clear`, `/compact` oder einem Neustart erreicht.

* [Bearbeiten von Dateien in Ihrem Repository](#editing-files-in-your-repository)
* [Bearbeiten von CLAUDE.md während der Sitzung](#editing-claude-md-mid-session)
* [Ändern des Berechtigungsmodus](#changing-permission-mode)
* [Ändern des Ausgabestils](#changing-output-style)
* [Aufrufen von Skills und Befehlen](#invoking-skills-and-commands)
* [Ausführen von `/recap`](#running-%2Frecap)
* [Rückgängigmachen des Gesprächs](#rewinding-the-conversation)
* [Starten eines Subagenten](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Bearbeiten von Dateien in Ihrem Repository
</h3>

Dateiinhalte werden nur dann in den Kontext aufgenommen, wenn Claude sie liest, und Lesevorgänge hängen sich an das Gespräch an. Das Bearbeiten einer Datei, die Claude zuvor gelesen hat, ändert nicht rückwirkend den früheren Lesevorgang in der Historie. Stattdessen hängt Claude Code eine `<system-reminder>` an, die notiert, dass sich die Datei geändert hat, und Claude liest sie bei Bedarf erneut.

<h3 id="editing-claude-md-mid-session">
  Bearbeiten von CLAUDE.md während der Sitzung
</h3>

Ihre CLAUDE.md-Dateien auf Projektebene und Benutzerebene werden einmal beim Sitzungsstart gelesen und im Speicher gehalten. Das Bearbeiten während der Sitzung invalidiert den Cache nicht, aber die Bearbeitung wird auch nicht angewendet. Claude arbeitet weiterhin mit der Version, die beim Sitzungsstart geladen wurde. Der neue Inhalt wird beim nächsten `/clear`, `/compact` oder Neustart geladen.

[Verschachtelte CLAUDE.md-Dateien in Unterverzeichnissen](/docs/de/memory) und [Regeln mit `paths:`-Frontmatter](/docs/de/memory#path-specific-rules) werden später geladen, wenn Claude zum ersten Mal eine entsprechende Datei liest. Das Bearbeiten einer Datei, bevor sie geladen wird, wird wirksam. Nach dem Laden ist der Inhalt Teil der Gesprächshistorie, daher ändert eine Bearbeitung während der Sitzung ihn nicht rückwirkend.

<h3 id="changing-permission-mode">
  Ändern des Berechtigungsmodus
</h3>

Das Wechseln zwischen [Berechtigungsmodi](/docs/de/permission-modes), z. B. von Manuell zu Bearbeitungen akzeptieren, ändert nicht die Systemaufforderung oder Werkzeugdefinitionen, daher sind Modusänderungen Cache-sicher. Die Ausnahme ist der Plan-Modus mit der [`opusplan`](/docs/de/model-config#opusplan-model-setting)-Modelleinstellung, die das Modell zwischen Opus und Sonnet wechselt, wenn Sie den Plan-Modus betreten oder verlassen. Das macht den Moduswechsel zu einem [Modellwechsel](#switching-models).

<h3 id="changing-output-style">
  Ändern des Ausgabestils
</h3>

Wenn Sie [Ausgabestile](/docs/de/output-styles) während der Sitzung mit [`/output-style`](/docs/de/output-styles#change-your-output-style), `/config` oder der `outputStyle`-Einstellung wechseln, verwendet Claude den neuen Stil ab Ihrer nächsten Nachricht. Claude Code liefert die Anweisungen des neuen Stils als Nachricht im Gespräch, daher liest diese Anfrage die Systemaufforderung und das frühere Gespräch aus dem Cache.

Vor v2.1.251 behielt ein Stilwechsel während der Sitzung den Cache bei, wurde aber erst angewendet, wenn Sie `/clear` ausführten oder eine neue Sitzung starteten.

<h3 id="invoking-skills-and-commands">
  Aufrufen von Skills und Befehlen
</h3>

[Skills](/docs/de/skills) und [Befehle](/docs/de/commands) injizieren ihre Anweisungen als Benutzernachrichten zum Zeitpunkt des Aufrufs. Nichts Früheres im Gespräch ändert sich. Ein Skill oder Befehl, dessen Frontmatter ein `model` benennt, kann für diesen Zug ein [Modellwechsel](#switching-models) sein.

<h3 id="running-/recap">
  Ausführen von `/recap`
</h3>

[`/recap`](/docs/de/interactive-mode#session-recap) generiert eine Zusammenfassung zur Anzeige in Ihrem Terminal. Im Gegensatz zu `/compact` hängt es die Zusammenfassung als Befehlsausgabe an, anstatt Ihren Nachrichtenverlauf zu ersetzen, daher bleibt das zwischengespeicherte Präfix intakt.

<h3 id="rewinding-the-conversation">
  Rückgängigmachen des Gesprächs
</h3>

[`/rewind`](/docs/de/checkpointing) kürzt Ihr Gespräch auf einen früheren Zug. Die verbleibende Historie ist derselbe Inhalt, aus dem der Cache zu diesem Zeitpunkt erstellt wurde, und die Systemaufforderung und Projektkontext-Ebenen sind unverändert, daher trifft die nächste Anfrage auf den früheren Cache-Eintrag. Jeder Zug seitdem hat dieses Präfix gelesen, das den Eintrag warm hielt, auch wenn der ursprüngliche Zug länger her war als die TTL.

Das Wiederherstellen von Datei-Checkpoints zusammen mit dem Gespräch hat keine separate Auswirkung auf den Cache. Dateiinhalte werden nur dann in den Kontext aufgenommen, wenn Claude sie liest, genauso wie beim [Bearbeiten von Dateien in Ihrem Repository](#editing-files-in-your-repository).

<h2 id="resuming-a-session">
  Fortsetzen einer Sitzung
</h2>

Wenn Sie [eine Sitzung fortsetzen](/docs/de/sessions#resume-a-session), sendet Claude Code das gesamte Gespräch erneut, und die Anfrage liest aus dem Cache, welcher Teil ihres Präfix unverändert ist und sich noch innerhalb der [Cache-Lebensdauer](#cache-lifetime) befindet. Die Schichttabelle oben auf dieser Seite zeigt, welche Änderungen jede Schicht vornimmt.

Die Systemaufforderung würde sich nach einem [Claude Code-Upgrade](#upgrading-claude-code) oder mit anderem [`--append-system-prompt`](/docs/de/cli-reference#system-prompt-flags)-Text beim Fortsetzen ändern. Standardmäßig behält das fortgesetzte Gespräch die Systemaufforderung bei, mit der es begonnen hat, sodass sein Verlauf immer noch hinter derselben Aufforderung liegt, und die Änderung wird wirksam, sobald das Gespräch komprimiert wird oder in einem neuen Gespräch stattfindet. [Systemaufforderungs-Flags in fortgesetzten Gesprächen](/docs/de/cli-reference#system-prompt-flags-in-resumed-conversations) behandelt die Fälle, in denen Claude Code die Aufforderung bei jeder Anfrage neu erstellt.

<h2 id="cache-lifetime">
  Cache-Lebensdauer
</h2>

Gecachte Präfixe verfallen nach einer Inaktivitätsperiode. Jede Anfrage, die den Cache trifft, setzt den Timer zurück, daher bleibt der Cache warm, solange Sie arbeiten. Nach einer langen genug Pause berechnet die nächste Anfrage die vollständige Eingabe neu und stellt den Cache wieder her, weshalb der erste Turn nach einer Pause noticeably langsamer sein kann.

Bei einem Pro- oder Max-Plan bietet Claude Code beim Fortsetzen einer großen Sitzung nach einer langen Pause [die Möglichkeit, von einer Zusammenfassung fortzufahren](/docs/de/sessions#resume-from-a-summary), damit spätere Anfragen nicht die vollständige Historie tragen.

Die Time to Live (TTL) kontrolliert, wie lange eine Pause der Cache überlebt. Die API bietet zwei: eine fünf-Minuten-TTL und eine [eine-Stunden-TTL](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration), die den Cache durch längere Pausen warm hält, aber [Cache-Schreibvorgänge zu einer höheren Rate abrechnet](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). Die längere TTL hilft, wenn Sie eine Sitzung untätig lassen und später zu ihr zurückkehren, da Sie die Neuverarbeitung eines abgelaufenen Präfix sparen. Sie kostet mehr bei kurzen Arbeitsphasen, die nie länger als fünf Minuten untätig sind, wo die höhere Schreibrate gilt und die längere Cache-Lebensdauer ungenutzt bleibt.

<h3 id="which-ttl-each-request-gets">
  Welche TTL jede Anfrage erhält
</h3>

Claude Code entscheidet die TTL pro Anfrage, und jede Anfrage fällt in einen von zwei festen Buckets:

* **Hauptkonversation**: Ihre interaktiven Turns, nicht-interaktive `-p`-Läufe und Agent SDK Turns, plus die Helfer, die Claude Code inline mit ihnen ausführt
* **Alles andere**: die Anfragen, die Claude Code außerhalb dieser Konversation macht, wie [Subagenten](/docs/de/sub-agents), [Workflows](/docs/de/workflows), In-Process-[Teammates](/docs/de/agent-teams), Forks, Komprimierung und Sitzungstitel

Sofern Sie nicht selbst eine TTL wählen, fordert Claude Code die eine-Stunden-TTL nur bei einem Claude-Abonnement innerhalb der in Ihrem Plan enthaltenen Nutzung an. Dort fordert es die Stunde für die Hauptkonversation an, plus eine kleine Menge von Helfer-Anfragen, die Anthropic serverseitig kontrolliert. Diese Tabelle gibt die Standard-TTL jedes Buckets unter beiden Arten der Abrechnung an.

| Anfrage-Bucket    | Claude-Abonnement, innerhalb der Plan-Nutzung                                                 | Nutzungsguthaben, API-Schlüssel oder Cloud-Provider |
| ----------------- | --------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| Hauptkonversation | Eine Stunde                                                                                   | Fünf Minuten                                        |
| Alles andere      | Fünf Minuten, außer den serverseitig kontrollierten Helfer-Anfragen, die eine Stunde erhalten | Fünf Minuten                                        |

Sobald Sie das Nutzungslimit Ihres Plans überschreiten und Claude Code auf [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) zurückgreift, wird diese Nutzung für Sie abgerechnet, daher senkt Claude Code die Hauptkonversation auf die günstigere fünf-Minuten-TTL. Um die eine-Stunden-TTL dort zu behalten, [wählen Sie die TTL selbst](#choose-the-ttl-yourself).

<h3 id="choose-the-ttl-yourself">
  Wählen Sie die TTL selbst
</h3>

Sie können eine TTL für jeden Bucket setzen. Jedes Steuerelement nimmt `5m` oder `1h` an, und Claude Code ignoriert jeden anderen Wert.

* **Hauptkonversation**: die [`promptCacheTtl`](/docs/de/settings-reference#promptcachettl)-Einstellung oder die `CLAUDE_CODE_PROMPT_CACHE_TTL`-[Umgebungsvariable](/docs/de/env-vars)
* **Alles andere**: die [`subagentPromptCacheTtl`](/docs/de/settings-reference#subagentpromptcachettl)-Einstellung oder die `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`-Umgebungsvariable

Beide Einstellungen und beide Umgebungsvariablen erfordern Claude Code v2.1.242 oder später. Wenn Sie sich mit einem API-Schlüssel anmelden oder einen Cloud-Provider verwenden, setzen Sie `promptCacheTtl` auf `1h`, um der Hauptkonversation einen einstündigen Cache zu geben. Anfragen außerhalb davon behalten die fünf-Minuten-Standard, bis Sie auch für diesen Bucket eine TTL wählen.

Wenn mehr als ein Steuerelement zutrifft, nimmt Claude Code die erste Übereinstimmung in dieser Reihenfolge:

1. `FORCE_PROMPT_CACHING_5M=1`, das fünf Minuten für beide Buckets erzwingt
2. Die Umgebungsvariable des Buckets
3. Die Einstellung des Buckets
4. Für die Anfragen eines Subagenten der `cacheTtl`-Wert im [`experimental`-Frontmatter-Feld](/docs/de/sub-agents#supported-frontmatter-fields) des Subagenten, das Claude Code v2.1.248 oder später erfordert. Claude Code ignoriert ein `1h` dort, während Ihr Claude-Abonnement Nutzungsguthaben verwendet
5. `ENABLE_PROMPT_CACHING_1H=1`, das eine Stunde für beide Buckets anfordert
6. Der [Standard für den Bucket der Anfrage](#which-ttl-each-request-gets)

Setzen Sie `FORCE_PROMPT_CACHING_5M=1`, wenn Sie das Cache-Verhalten debuggen, die beiden TTLs vergleichen oder eine längere TTL überschreiben, die in [verwalteten Einstellungen](/docs/de/managed-settings) gesetzt ist.

Um zu bestätigen, welche TTL die Cache-Schreibvorgänge Ihrer Hauptkonversation verwendet haben, führen Sie `claude -p "hello" --output-format json` aus und lesen Sie `usage.cache_creation` im Ergebnis. Claude Code meldet einstündige Cache-Schreibvorgänge unter `ephemeral_1h_input_tokens` und fünf-Minuten-Cache-Schreibvorgänge unter `ephemeral_5m_input_tokens`.

Durch ein LLM-Gateway, das Sie mit `ANTHROPIC_BASE_URL` setzen, reist ein Teil der einstündigen Anfrage im `anthropic-beta`-Header, daher konfigurieren Sie das Gateway, um [diesen Header unverändert weiterzuleiten](/docs/de/llm-gateway-protocol#request-headers). Die eine-Stunden-TTL ist nicht über das [Claude-Apps-Gateway](/docs/de/claude-apps-gateway#availability-and-limitations) verfügbar. Bei Amazon Bedrock variieren Prompt-Caching-Unterstützung, minimale cacheable Präfixlänge und eine-Stunden-TTL-Verfügbarkeit je nach Modell. Wenn Cache-Token-Zählungen bei Null bleiben, überprüfen Sie [unterstützte Modelle, Regionen und Limits](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) in der Amazon Bedrock-Dokumentation.

<h2 id="cache-scope">
  Cache-Umfang
</h2>

In Claude Code ist der Cache effektiv auf einen Computer und ein Verzeichnis beschränkt. Jede Konversation trägt das Arbeitsverzeichnis, die Plattform, die Shell und die OS-Version mit sich, und der System-Prompt benennt Ihre Auto-Memory-Pfade, daher bauen zwei Sessions in verschiedenen Verzeichnissen unterschiedliche Präfixe auf und verfehlen den Cache des anderen. Das schließt Worktrees desselben Repositorys ein, da jeder Worktree sein eigenes Arbeitsverzeichnis hat.

Sessions, die Sie parallel im selben Verzeichnis ausführen, bauen passende Präfixe auf und lesen den Cache des anderen. Sequenzielle Sessions teilen das Präfix nur, wenn der Git-Status-Snapshot beim Start übereinstimmt, da jede Konversation auch den Branch und aktuelle Commits aus diesem Snapshot trägt.

Der zugrunde liegende API-Cache ist breiter. Caches sind zwischen Organisationen isoliert, und bei einigen Providern [zwischen Workspaces innerhalb einer Organisation](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#cache-storage-and-sharing). Innerhalb dieser Grenzen lesen zwei Anfragen mit demselben Modell und Präfix denselben Cache. Für Agent SDK-Aufrufer, die Flotten automatisierter Prozesse ausführen, siehe [Prompt Caching über Benutzer und Maschinen verbessern](/docs/de/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines), um die Pro-Maschinen-Abschnitte des System-Prompts zu unterdrücken und den Cache über Maschinen zu teilen.

<h2 id="check-cache-performance">
  Cache-Leistung überprüfen
</h2>

Cache-Leistung zeigt sich als zwei Token-Zählungen, die die API bei jeder Antwort meldet. Der direkteste Weg, sie live zu beobachten, ist ein [Statusline-Skript](/docs/de/statusline), das das `current_usage`-Objekt liest:

| Feld                          | Bedeutung                                                                                                                                                                                                      |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Tokens, die in diesem Turn in den Cache geschrieben werden, abgerechnet zum Cache-Schreibsatz                                                                                                                  |
| `cache_read_input_tokens`     | Tokens, die in diesem Turn aus dem Cache bereitgestellt werden, abgerechnet zum [Cached-Token-Satz des Modells](https://platform.claude.com/docs/en/about-claude/pricing), unterhalb des Standard-Input-Satzes |

Ein hohes Lese-zu-Erstellungs-Verhältnis bedeutet, dass Caching gut funktioniert. Wenn die Erstellung Turn für Turn hoch bleibt, ändert sich etwas in Ihrem Präfix. Der Abschnitt [Aktionen, die den Cache ungültig machen](#actions-that-invalidate-the-cache) listet die üblichen Ursachen auf.

Für eine Zusammenfassung pro Session führen Sie `/usage` aus. Nach der ersten Antwort der Hauptkonversation fügt Claude Code eine [`Prompt cache (main)`-Zeile](/docs/de/costs#prompt-cache-statistics) zum Session-Block hinzu, die das Hit-Verhältnis der Session, die Anzahl der Misses und an, ob der Cache gerade warm ist. Ein Statusline-Skript kann die gleichen Zahlen aus dem [`prompt_cache`-Objekt](/docs/de/statusline#prompt-cache-fields) lesen. Beide erfordern Claude Code v2.1.251 oder später.

Die `Prompt cache (main)`-Zeile nennt auch die wahrscheinliche Ursache des letzten Miss, wenn Claude Code eine identifizieren kann, zum Beispiel `likely cause: tool definitions changed`. Der Text zur wahrscheinlichen Ursache erfordert Claude Code v2.1.260 oder später.

Für Sichtbarkeit über eine Organisation hinweg meldet der OpenTelemetry-Exporter Cache-Lese- und Erstellungs-Tokens pro Benutzer und Session. Siehe [Nutzung überwachen](/docs/de/monitoring-usage) für die Metrik- und Event-Attribut-Referenz.

<h2 id="subagents-and-the-cache">
  Subagents und der Cache
</h2>

Ein [Subagent](/docs/de/sub-agents) startet seine eigene Konversation mit seinem eigenen System-Prompt und Tool-Set, getrennt vom Parent. Sein erster Request liest den Cache des Parents nicht, weil sich die beiden Präfixe unterscheiden, und er wärmt seinen eigenen Cache über seine Turns auf. Subagents fallen außerhalb des Haupt-Konversations-[TTL-Buckets](#which-ttl-each-request-gets), daher erhalten sie fünf Minuten auch bei einem Abonnement, bis Sie [einen längeren wählen](#choose-the-ttl-yourself).

Der Cache des Parents ist unberührt. Von der Parent-Seite hängen der Aufruf und das Ergebnis des Subagents an die Konversation an, wobei das Parent-Präfix intakt bleibt.

Ein [Fork](/docs/de/sub-agents#fork-the-current-conversation) erbt dagegen den System-Prompt, die Tools und die Konversationshistorie des Parents genau, daher liest sein erster Request den Cache des Parents.

Andere Requests können auch ein Präfix lesen, das ein früherer Request gecacht hat:

* **Session-Kopien**: Eine Session, die Sie [mit `/fork` kopieren](/docs/de/agent-view#copy-the-session-with-%2Ffork), erhält ihre Isolationsanweisung als Nachricht am Ende der kopierten Konversation, daher bleibt der Cache, den die ursprüngliche Konversation aufgebaut hat, intakt.
* **Komprimierung**: Der Zusammenfassungs-Call, der in [Konversation komprimieren](#compacting-the-conversation) beschrieben wird, verwendet denselben Präfix-Sharing-Ansatz.
* **Wiederaufgenommene Subagents**: Wenn Claude einen [Subagent wiederaufnimmt](/docs/de/sub-agents#resume-subagents), kann der erste Request des wiederaufgenommenen Laufs den Cache lesen, den der ursprüngliche Lauf gewärmt hat.
* **Workflow-Fan-Outs**: Bei einem [Workflow-Fan-Out](/docs/de/workflows#prompt-caching-in-a-fan-out) von Agents mit gleichem Präfix hält Claude Code alle außer dem ersten standardmäßig bis zu 5 Sekunden lang, damit ihre ersten Requests das Präfix lesen können, das der erste Agent gecacht hat.

<h2 id="disable-prompt-caching">
  Prompt Caching deaktivieren
</h2>

Das Deaktivieren von Caching ist gelegentlich nützlich, wenn Sie das Caching-Verhalten mit einem bestimmten Modell oder Provider debuggen. Um es auszuschalten, setzen Sie eine dieser Umgebungsvariablen auf `1`:

| Variable                        | Effekt                        |
| ------------------------------- | ----------------------------- |
| `DISABLE_PROMPT_CACHING`        | Für alle Modelle deaktivieren |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Nur für Haiku deaktivieren    |
| `DISABLE_PROMPT_CACHING_SONNET` | Nur für Sonnet deaktivieren   |
| `DISABLE_PROMPT_CACHING_OPUS`   | Nur für Opus deaktivieren     |
| `DISABLE_PROMPT_CACHING_FABLE`  | Nur für Fable deaktivieren    |

Um die Caching-Richtlinie über eine Organisation hinweg festzulegen, setzen Sie eine dieser oder die [TTL-Variablen](#cache-lifetime) in den `env`-Block von [verwalteten Einstellungen](/docs/de/managed-settings). Für normale Nutzung lassen Sie Caching aktiviert.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Lektionen aus dem Aufbau von Claude Code: Prompt Caching ist alles](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): die Designrationale für Plan Mode, aufgeschobenes Tool-Laden und Komprimierung
* [Erkunden Sie das Kontextfenster](/docs/de/context-window): was in den Kontext geladen wird und wann
* [Token-Nutzung reduzieren](/docs/de/costs#reduce-token-usage): Strategien jenseits von Caching zur Verwaltung der Kontextgröße
* [Kosten verfolgen und reduzieren](/docs/de/agent-sdk/cost-tracking): Cache-Token-Verfolgung und TTL-Konfiguration für Agent SDK-Aufrufer
* [Prompt Caching](https://platform.claude.com/docs/de/build-with-claude/prompt-caching): der zugrunde liegende API-Mechanismus, Breakpoints und Preisgestaltung
