> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wie Claude Code Prompt Caching nutzt

> Claude Code verwaltet Prompt Caching automatisch. Erfahren Sie, warum ein Modellwechsel einen langsamen unkachedten Turn auslöst, was `/compact` kostet, warum CLAUDE.md-Änderungen mid-session nicht angewendet werden, und wie Sie Ihre Cache-Hit-Rate überprüfen.

Prompt Caching macht Claude Code schneller und kostengünstiger. Ohne Caching würde die API Ihre vollständige Historie bei jedem Turn neu verarbeiten. Mit Caching nutzt sie das bereits Verarbeitete wieder und führt nur neue Arbeit für das Geänderte durch.

Claude Code verwaltet Prompt Caching für Sie, es sei denn, Sie [deaktivieren es](#disable-prompt-caching). Es ist dennoch nützlich zu verstehen, wie Prompt Caching funktioniert, da einige Aktionen den Cache ungültig machen und die nächste Antwort langsamer und teurer machen, während er sich neu aufbaut. Diese Seite behandelt, welche Aktionen das sind, warum einige Einstellungen auf einen Neustart warten, um angewendet zu werden, und wie Sie die Cache-Leistung überprüfen, wenn die Nutzung hoch aussieht.

<h2 id="how-the-cache-is-organized">
  Wie der Cache organisiert ist
</h2>

Jedes Mal, wenn Sie eine Nachricht in Claude Code senden, wird eine neue API-Anfrage gestellt. Das Modell merkt sich nichts zwischen Anfragen, daher sendet Claude Code den vollständigen Kontext erneut: den System-Prompt, Ihren Projektkontext, jede vorherige Nachricht und jedes Tool-Ergebnis sowie Ihre neue Nachricht. Neuer Inhalt wird am Ende angehängt, was bedeutet, dass der größte Teil jeder Anfrage identisch mit der vorherigen ist. Prompt Caching ist, wie die API vermeidet, den Teil neu zu verarbeiten, der sich nicht geändert hat.

Die API cached durch Abgleich des Anfangs jeder Anfrage, genannt das Präfix, gegen kürzlich verarbeitete Inhalte. Bei einem normalen Turn ist das Präfix die gesamte vorherige Anfrage und nur der neueste Austausch ist neu. Der Abgleich ist exakt, daher wird alles nach einer Änderung im Präfix neu berechnet. Es gibt kein Pro-Datei- oder Pro-Segment-Caching. Siehe [wie Prompt Caching funktioniert](https://platform.claude.com/docs/de/build-with-claude/prompt-caching#how-prompt-caching-works) in der API-Referenz für den zugrunde liegenden Mechanismus.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Vier Turns werden als wachsende horizontale Balken angezeigt. Die Anfrage jedes Turns enthält alles aus dem vorherigen Turn plus den neuesten Austausch am Ende angehängt. Bei den Turns zwei und drei wird das unveränderte Präfix aus dem Cache gelesen und nur der neue Austausch verarbeitet. Bei Turn vier hat sich der System-Prompt geändert, daher stimmt das Präfix nicht mehr überein und die gesamte Anfrage wird neu verarbeitet und geschrieben." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Vier Turns werden als wachsende horizontale Balken angezeigt. Die Anfrage jedes Turns enthält alles aus dem vorherigen Turn plus den neuesten Austausch am Ende angehängt. Bei den Turns zwei und drei wird das unveränderte Präfix aus dem Cache gelesen und nur der neue Austausch verarbeitet. Bei Turn vier hat sich der System-Prompt geändert, daher stimmt das Präfix nicht mehr überein und die gesamte Anfrage wird neu verarbeitet und geschrieben." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Um das Beste aus dem Präfix-Abgleich herauszuholen, ordnet Claude Code jede Anfrage so, dass Inhalte, die sich zwischen Turns selten ändern, zuerst kommen:

| Ebene          | Inhalt                                               | Ändert sich wenn                                                                         |
| -------------- | ---------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| System-Prompt  | Kernanleitungen, Tool-Definitionen, Ausgabestil      | Der Satz der geladenen Tool-Definitionen ändert sich, oder Claude Code wird aktualisiert |
| Projektkontext | CLAUDE.md, automatisches Memory, unscoped Rules      | Session startet, oder nach `/clear` oder `/compact`                                      |
| Konversation   | Ihre Nachrichten, Claudes Antworten, Tool-Ergebnisse | Jeden Turn                                                                               |

Eine Änderung der Konversationsebene lässt den System-Prompt und Projektkontext gecacht. Eine Änderung des System-Prompts macht alles ungültig, da der gesamte spätere Inhalt nun hinter einem anderen Präfix sitzt. Die dritte Spalte gibt häufige Auslöser statt einer vollständigen Liste an, und die Abschnitte unten behandeln den vollständigen Satz, einschließlich Inhalte wie Ausgabestil, die am Anfang der Session festgelegt sind.

Die Präfix-Abgleich-Regel erklärt die meisten Verhaltensweisen auf dieser Seite. [Plan Mode](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) und [Skill Loading](/docs/de/skills) hängen beispielsweise ihre Anweisungen als Konversationsnachrichten an, daher bleibt das gecachte Präfix intakt.

Zwei Einstellungen sind überhaupt nicht Teil des Prompt-Textes, daher erscheinen sie nicht in der Ebenen-Tabelle, aber beide sind Teil des Cache-Schlüssels:

* **Modell**: Jedes Modell hat seinen eigenen Cache. Das Wechseln von Modellen berechnet die gesamte Anfrage neu, auch wenn der Inhalt identisch ist. Siehe [Modelle wechseln](#switching-models) unten.
* **Effort Level**: Jedes Effort Level hat seinen eigenen Cache für dasselbe Modell. Das Ändern mid-session berechnet die gesamte Anfrage neu, und Claude Code bittet Sie, die Änderung zu bestätigen, bevor Sie sie anwenden. Siehe [Effort Level ändern](#changing-effort-level) unten.

<Tip>
  Wählen Sie Ihr Modell und Effort Level am Anfang einer Session, dann speichern Sie `/compact` für natürliche Pausen zwischen Aufgaben. Je weniger Änderungen Sie mid-task vornehmen, desto höher ist Ihre Cache-Hit-Rate.
</Tip>

<h3 id="where-the-cache-lives">
  Wo der Cache lebt
</h3>

Caching findet server-seitig in der Infrastruktur statt, die Ihr Modell bedient. Wo das ist, hängt davon ab, wie Sie sich authentifizieren:

* **API-Schlüssel, Claude-Abonnement oder [Claude Platform on AWS](/docs/de/claude-platform-on-aws)**: Der Cache lebt in Anthropics Infrastruktur, zugänglich über die [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock oder Google Cloud's Agent Platform**: Der Cache lebt in der Serving-Infrastruktur Ihres Cloud-Providers
* **Microsoft Foundry**: Anfragen werden an Anthropics Infrastruktur weitergeleitet
* **Benutzerdefinierte `ANTHROPIC_BASE_URL` oder [LLM Gateway](/docs/de/llm-gateway)**: Der Cache lebt dort, wo Ihre Anfragen weitergeleitet werden, und ob Caching funktioniert, hängt vom Gateway ab

Für das, was jeder Provider speichert und verarbeitet, siehe [Datennutzung](/docs/de/data-usage). Wo immer der Cache lebt, Einträge verfallen nach einer Inaktivitätsperiode, und [Cache-Lebensdauer](#cache-lifetime) unten behandelt die TTL und wie Sie sie verlängern.

<h2 id="actions-that-invalidate-the-cache">
  Aktionen, die den Cache ungültig machen
</h2>

Diese Aktionen führen dazu, dass die nächste Anfrage einen Teil oder den gesamten Cache verfehlt. Sie sehen einen einmaligen langsameren, teureren Turn, danach wird das neue Präfix gecacht. Die meisten davon sind mid-task vermeidbar, sobald Sie wissen, dass sie Kosten haben. Ein Modellwechsel kann sich kostenlos anfühlen, bis Sie den langsameren Turn bemerken, der folgt.

* [Modelle wechseln](#switching-models)
* [Anstrengungsstufe ändern](#changing-effort-level)
* [Fast Mode aktivieren](#turning-on-fast-mode)
* [Verbinden oder Trennen eines MCP-Servers](#connecting-or-disconnecting-an-mcp-server)
* [Ein Plugin aktivieren oder deaktivieren](#enabling-or-disabling-a-plugin)
* [Ein ganzes Tool ablehnen](#denying-an-entire-tool)
* [Konversation komprimieren](#compacting-the-conversation)
* [Claude Code aktualisieren](#upgrading-claude-code)

<h3 id="switching-models">
  Modelle wechseln
</h3>

Jedes Modell hat seinen eigenen Cache. Das Wechseln mit [`/model`](/docs/de/model-config#setting-your-model) bedeutet, dass die nächste Anfrage die gesamte Konversationshistorie ohne Cache-Hits liest, obwohl der Inhalt identisch ist.

Die [`opusplan` Modelleinstellung](/docs/de/model-config#opusplan-model-setting) wird zu Opus während Plan Mode und Sonnet während der Ausführung aufgelöst, daher ist jeder Plan-Mode-Toggle ein Modellwechsel und startet einen frischen Cache.

[Automatisches Modell-Fallback](/docs/de/model-config#automatic-model-fallback) auf Fable 5 ist auch ein Modellwechsel. Wenn ein Sicherheitsklassifizierer eine Anfrage kennzeichnet, führt Claude Code sie auf dem Standard-Opus-Modell erneut aus und die Session wird dort fortgesetzt.

<h3 id="changing-effort-level">
  Anstrengungsstufe ändern
</h3>

Der Cache wird nach [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) sowie Modell verschlüsselt, daher bedeutet das Wechseln mit `/effort`, dass die nächste Anfrage die gesamte Konversationshistorie ohne Cache-Hits liest. Sobald eine Konversation gestartet wurde, zeigt Claude Code einen Bestätigungsdialog an, bevor eine Anstrengungsänderung angewendet wird, die den Cache ungültig machen würde. Eine Änderung, die sich auf die gleiche bereits gültige Stufe auflöst, z. B. das explizite Festlegen des Modellstandards, überspringt den Dialog und behält den Cache.

<h3 id="turning-on-fast-mode">
  Fast Mode aktivieren
</h3>

Das Aktivieren von [Fast Mode](/docs/de/fast-mode) fügt einen Request-Header hinzu, der Teil des Cache-Schlüssels ist, daher liest die nächste Anfrage die gesamte Konversationshistorie ohne Cache-Hits. Diese unkachedten Input-Token werden zu [Fast-Mode-Raten](/docs/de/fast-mode#understand-the-cost-tradeoff) abgerechnet, weshalb das Aktivieren zu Beginn einer Session weniger kostet als das Aktivieren tief in einer langen Session. Das Aktivieren von Fast Mode von einem Nicht-Opus-Modell aus [wechselt auch Ihr Modell](#switching-models), was von selbst einen frischen Cache startet.

Die Kosten fallen einmal pro Konversation an. Nach dem ersten Fast-Mode-Turn sendet Claude Code weiterhin den Header und variiert nur die Speed-Einstellung der Anfrage, die nicht Teil des Cache-Schlüssels ist. Das Ausschalten von Fast Mode, das [automatische Fallback auf Standardgeschwindigkeit](/docs/de/fast-mode#handle-rate-limits) nach einem Rate Limit und das spätere Wiedereinschalten behalten den Cache. `/clear` und `/compact` setzen dies zurück, da sie den Cache an diesen Punkten ohnehin neu aufbauen.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Verbinden oder Trennen eines MCP-Servers
</h3>

Tool-Definitionen sitzen in der System-Prompt-Ebene, daher wird der Cache ungültig, wenn sich die Menge der Tool-Definitionen in der Anfrage zwischen Turns ändert. Das Umschalten des [Advisor-Tools](/docs/de/advisor) ist eine Ausnahme: Seine Definition sitzt nach dem Cache-Breakpoint, daher behält das Aktivieren oder Deaktivieren von `/advisor` das gecachte Präfix intakt. Ob eine [MCP-Server](/docs/de/mcp)-Änderung dies bewirkt, hängt davon ab, ob ihre Tools durch [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) aufgeschoben werden oder in das Präfix geladen werden:

* **Aufgeschobene Tools**, die Standardeinstellung auf unterstützten Modellen: Ein Server, der sich verbindet, trennt oder seine Tool-Liste ändert, hängt nur neue Inhalte an und stört nichts, das bereits gecacht ist.
* **Tools, die in das Präfix geladen werden**: Jede Änderung daran macht den Cache ungültig. Dies geschieht, wenn [Tool-Suche nicht verfügbar oder deaktiviert ist](/docs/de/mcp#configure-tool-search), z. B. auf Google Cloud's Agent Platform oder mit einem benutzerdefinierten `ANTHROPIC_BASE_URL`-Gateway. Es geschieht auch für einen Server oder ein Tool, das als [`alwaysLoad`](/docs/de/mcp#exempt-a-server-from-deferral) markiert ist, und für Definitionen, die von [schwellenwertbasiertem Laden](/docs/de/mcp#configure-tool-search) im Voraus beibehalten werden.

Wenn Tools in das Präfix geladen werden, ist die häufigste Ursache einer Ungültigmachung ein Server, der sich mid-session verbindet oder trennt, was ohne Ihre Aktion geschehen kann: Ein Stdio-Server-Prozess beendet sich, eine HTTP-Session läuft ab, oder ein Server [verbindet sich automatisch nach einem vorübergehenden Fehler wieder](/docs/de/mcp#automatic-reconnection). Ein verbundener Server kann auch ein [dynamisches Tool-Update](/docs/de/mcp#dynamic-tool-updates) pushen, das seine Tool-Liste ändert.

Das Bearbeiten Ihrer MCP-Konfiguration ändert den Cache nicht von selbst. Die neue Konfiguration wird erst nach einem Neustart wirksam, wenn sich der Server verbindet oder trennt.

<h3 id="enabling-or-disabling-a-plugin">
  Ein Plugin aktivieren oder deaktivieren
</h3>

[Plugins](/docs/de/plugins) bündeln mehrere Komponententypen, und die Kosten einer Änderung hängen davon ab, welche Komponenten das Plugin bereitstellt. Skills, Commands, Agents, Hooks, LSP-Server, Monitore und Themes machen den Cache niemals ungültig: Alles, das sie zur Anfrage hinzufügen, wird nach der vorhandenen Konversation angehängt, daher zahlt die nächste Anfrage für den neuen Inhalt, liest aber alles davor immer noch aus dem Cache.

Die Ausnahme ist ein Plugin, das [MCP-Server](/docs/de/plugins-reference#mcp-servers) bereitstellt. Das Aktivieren oder Deaktivieren eines solchen folgt den gleichen Regeln wie [Verbinden oder Trennen eines MCP-Servers](#connecting-or-disconnecting-an-mcp-server): Der Cache bleibt erhalten, wenn die Tools des Servers aufgeschoben werden, und die nächste Anfrage liest die gesamte Konversation neu, wenn sie in das Präfix geladen werden.

Plugin-Änderungen werden angewendet, wenn Sie [`/reload-plugins`](/docs/de/discover-plugins#apply-plugin-changes-without-restarting) ausführen oder eine neue Session starten. Die Kosten, ob angehängte Ankündigungen oder ein vollständiges Neueinlesen, zeigen sich beim ersten Turn nach dem Neustart, nicht wenn Sie `/plugin install`, `/plugin enable` oder `/plugin disable` ausführen. Ab v2.1.163 zeigt `/reload-plugins` eine Warnung an und wendet das Neustart nicht an, wenn ein Neustart das vollständige Neueinlesen auslösen würde. Übergeben Sie `--force`, um es trotzdem anzuwenden.

Das Deaktivieren eines Plugins, das Sie früher in der Session aktiviert haben, stellt die vorherige Anfragefom wieder her. Wenn dieses Präfix noch innerhalb seiner [Cache-Lebensdauer](#cache-lifetime) liegt, liest die nächste Anfrage stattdessen den älteren Cache-Eintrag, anstatt ihn neu zu erstellen.

<h3 id="denying-an-entire-tool">
  Ein ganzes Tool ablehnen
</h3>

Das Hinzufügen eines bloßen Tool-Namens wie `Bash` oder `WebFetch` als [Ablehnungsregel](/docs/de/permissions#manage-permissions) entfernt dieses Tool vollständig aus Claudes Kontext. Tool-Definitionen werden in die System-Prompt-Ebene geladen, daher macht das Hinzufügen oder Entfernen einer dieser Regeln mid-session den Cache ungültig. Die Änderung wird beim nächsten Turn wirksam, egal ob Sie sie über `/permissions` hinzufügen oder durch [direktes Bearbeiten einer Einstellungsdatei](/docs/de/settings#when-edits-take-effect).

Nur eine Ablehnungsregel, die in der Tool-Name-Position passt, hat diese Auswirkung: ein bloßer Tool-Name, die äquivalente `Bash(*)` Form oder ein [Tool-Name-Glob](/docs/de/permissions#tool-name-wildcards) wie `"*"`. Ein Glob, der nur MCP-Tools passt, z. B. `"mcp__*"`, entfernt diese Tools auf die gleiche Weise, behält aber den Cache intakt, wenn die gefundenen Tools [aufgeschoben](#connecting-or-disconnecting-an-mcp-server) sind, die Standardeinstellung, da aufgeschobene Definitionen nie im gecachten Präfix waren. Scoped Ablehnungsregeln wie `Bash(rm *)` und alle Allow- und Ask-Regeln ändern nicht, welche Tools Claude sieht. Claude Code prüft sie, wenn Claude einen Aufruf versucht, wobei das Präfix intakt bleibt.

<h3 id="compacting-the-conversation">
  Konversation komprimieren
</h3>

[Komprimierung](/docs/de/context-window#what-survives-compaction) ersetzt Ihre Nachrichtenhistorie durch eine Zusammenfassung. Dies macht absichtlich die Konversationsebene ungültig, da die nächste Anfrage eine neue, kürzere Historie hat, die kein Präfix mit der alten teilt. Claude Code nutzt die System-Prompt-Ebene wieder und lädt den Projektkontext von der Festplatte neu, was nur Cache-Hits, wenn CLAUDE.md und Memory seit dem Session-Start unverändert sind.

Um die Zusammenfassung zu erstellen, sendet Claude Code eine einmalige Anfrage mit demselben System-Prompt, Tools und History wie Ihre Konversation, plus eine Zusammenfassungsanweisung, die als letzte Benutzernachricht angehängt wird. Da sie Ihr Präfix teilt, liest diese Anfrage den vorhandenen Cache, anstatt die vollständige Historie neu zu verarbeiten. Der größte Teil der Komprimierungszeit geht in die Generierung der Zusammenfassung, nicht in einen Cache-Miss. Der Turn, der folgt, baut den Konversations-Cache nur für die viel kürzere Zusammenfassung neu auf, daher ist der Post-Komprimierungs-Turn nicht der langsame Teil.

<Tip>
  Komprimierung funktioniert zu Ihrem Vorteil, wenn der Kontext, den Sie verwerfen, Inhalte sind, die Sie nicht mehr benötigen. Um zu wählen, wann sein Overhead auftritt, führen Sie `/compact` bei einer natürlichen Pause in Ihrer Arbeit aus, z. B. zwischen Aufgaben, anstatt zu warten, bis Auto-Komprimierung mid-task auslöst. Wenn Sie einen Weg gegangen sind, den Sie ganz aufgeben möchten, [`/rewind`](#rewinding-the-conversation) stattdessen zu einem früheren Turn. Das Zurückspulen schneidet auf ein Präfix zurück, das bereits gecacht ist, anstatt ein neues wie Komprimierung zu erstellen.
</Tip>

<h3 id="upgrading-claude-code">
  Claude Code aktualisieren
</h3>

Eine neue Claude Code-Version aktualisiert typischerweise den System-Prompt oder Tool-Definitionen, daher baut die erste Anfrage nach einem Upgrade den Cache von oben auf. [Auto-Update](/docs/de/setup#auto-updates) lädt neue Versionen im Hintergrund herunter, wendet sie aber beim nächsten Start an, nie mid-session, daher sehen Sie dies als einen unkachedten ersten Turn nach dem Neustart statt als Überraschung während einer Session. Setzen Sie `DISABLE_AUTOUPDATER=1`, um zu kontrollieren, wann Upgrades angewendet werden.

<Note>
  [Eine Session nach einem Upgrade fortsetzen](/docs/de/sessions#resume-a-session) verarbeitet die gesamte Konversationshistorie ohne Cache-Hits neu, da die Historie nun hinter einem anderen System-Prompt sitzt. Die Kosten skalieren mit der Länge der fortgesetzten Konversation, daher kann der erste Turn zurück in eine lange Session die teuerste Anfrage sein, die Sie senden.
</Note>

<h2 id="actions-that-keep-the-cache">
  Aktionen, die den Cache behalten
</h2>

Diese Aktionen hängen entweder am Ende der Konversation an oder berühren die Anfrage überhaupt nicht. Einige davon, wie das Bearbeiten von CLAUDE.md oder das Ändern des Ausgabestils, sind auch der Grund, warum eine Einstellungsänderung auf einen Neustart wartet, um angewendet zu werden.

* [Dateien in Ihrem Repository bearbeiten](#editing-files-in-your-repository)
* [CLAUDE.md mid-session bearbeiten](#editing-claude-md-mid-session)
* [Ausgabestil ändern](#changing-output-style)
* [Permission Mode ändern](#changing-permission-mode)
* [Skills und Commands aufrufen](#invoking-skills-and-commands)
* [Ausführen von `/recap`](#running-%2Frecap)
* [Konversation zurückspulen](#rewinding-the-conversation)
* [Einen Subagent spawnen](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Dateien in Ihrem Repository bearbeiten
</h3>

Dateiinhalte treten in den Kontext nur ein, wenn Claude sie liest, und Lesevorgänge hängen an der Konversation an. Das Bearbeiten einer Datei, die Claude zuvor gelesen hat, ändert nicht rückwirkend das frühere Lesen in der Historie. Stattdessen hängt Claude Code eine `<system-reminder>` an, die bemerkt, dass die Datei geändert wurde, und Claude liest sie erneut, wenn nötig.

<h3 id="editing-claude-md-mid-session">
  CLAUDE.md mid-session bearbeiten
</h3>

Ihre Projekt-Root- und Benutzer-Level-CLAUDE.md-Dateien werden einmal am Session-Start gelesen und im Speicher gehalten. Das Bearbeiten von ihnen mid-session macht den Cache nicht ungültig, aber die Bearbeitung wird auch nicht angewendet. Claude arbeitet weiterhin mit der Version, die am Session-Start geladen wurde. Der neue Inhalt wird beim nächsten `/clear`, `/compact` oder Neustart geladen.

[Verschachtelte CLAUDE.md-Dateien in Unterverzeichnissen](/docs/de/memory) und [Rules mit `paths:` Frontmatter](/docs/de/memory#path-specific-rules) werden später geladen, wenn Claude zum ersten Mal eine passende Datei liest. Das Bearbeiten einer vor dem Laden hat Auswirkungen. Nach dem Laden ist der Inhalt Teil der Konversationshistorie, daher ändert eine mid-session-Bearbeitung ihn nicht rückwirkend.

<h3 id="changing-output-style">
  Ausgabestil ändern
</h3>

[Ausgabestil](/docs/de/output-styles) ist Teil des System-Prompts, den Claude Code einmal am Session-Start liest. Das Ändern über `/config` oder die `outputStyle`-Einstellung mid-session macht den Cache nicht ungültig, aber die Änderung wird auch nicht angewendet. Claude verwendet weiterhin den Stil, der am Session-Start geladen wurde. Der neue Stil wird beim nächsten `/clear` oder Neustart geladen.

<h3 id="changing-permission-mode">
  Permission Mode ändern
</h3>

Das Wechseln zwischen [Permission Modes](/docs/de/permission-modes), z. B. von Standard zu Accept Edits, ändert den System-Prompt oder Tool-Definitionen nicht, daher sind Mode-Wechsel Cache-sicher. Die Ausnahme ist Plan Mode mit der [`opusplan`](/docs/de/model-config#opusplan-model-setting) Modelleinstellung, die das Modell zwischen Opus und Sonnet wechselt, wenn Sie Plan Mode betreten oder verlassen. Das macht den Mode-Toggle zu einem [Modellwechsel](#switching-models).

<h3 id="invoking-skills-and-commands">
  Skills und Commands aufrufen
</h3>

[Skills](/docs/de/skills) und [Commands](/docs/de/commands) injizieren ihre Anweisungen als Benutzernachrichten am Punkt der Aufrufe. Nichts früher in der Konversation ändert sich.

<h3 id="running-/recap">
  Ausführen von `/recap`
</h3>

[`/recap`](/docs/de/interactive-mode#session-recap) generiert eine Zusammenfassung zur Anzeige in Ihrem Terminal. Im Gegensatz zu `/compact` hängt es die Zusammenfassung als Befehlsausgabe an, anstatt Ihre Nachrichtenhistorie zu ersetzen, daher bleibt das gecachte Präfix intakt.

<h3 id="rewinding-the-conversation">
  Konversation zurückspulen
</h3>

[`/rewind`](/docs/de/checkpointing) schneidet Ihre Konversation auf einen früheren Turn zurück. Die verbleibende Historie ist derselbe Inhalt, aus dem der Cache zu diesem Punkt gebaut wurde, und die System-Prompt- und Projektkontext-Ebenen sind unverändert, daher trifft die nächste Anfrage den früheren Cache-Eintrag. Jeder Turn seitdem hat dieses Präfix gelesen, das den Eintrag warm hielt, auch wenn der ursprüngliche Turn länger her war als die TTL.

Das Wiederherstellen von Datei-Checkpoints zusammen mit der Konversation hat keine separate Auswirkung auf den Cache. Dateiinhalte treten in den Kontext nur ein, wenn Claude sie liest, dasselbe wie [Dateien in Ihrem Repository bearbeiten](#editing-files-in-your-repository).

<h2 id="cache-lifetime">
  Cache-Lebensdauer
</h2>

Gecachte Präfixe verfallen nach einer Inaktivitätsperiode. Jede Anfrage, die den Cache trifft, setzt den Timer zurück, daher bleibt der Cache warm, solange Sie arbeiten. Nach einer langen genug Pause berechnet die nächste Anfrage die vollständige Eingabe neu und stellt den Cache wieder her, weshalb der erste Turn nach einer Pause noticeably langsamer sein kann.

Die Time to Live (TTL) kontrolliert, wie lange eine Pause der Cache überlebt. Die API bietet zwei: eine fünf-Minuten-TTL und eine [eine-Stunden-TTL](https://platform.claude.com/docs/de/build-with-claude/prompt-caching#1-hour-cache-duration), die den Cache durch längere Pausen warm hält, aber [Cache-Schreibvorgänge zu einer höheren Rate abrechnet](https://platform.claude.com/docs/de/build-with-claude/prompt-caching#pricing). Claude Code wählt die TTL für Sie basierend auf Ihrer Authentifizierung, und Sie können sie mit Umgebungsvariablen überschreiben.

<h3 id="on-a-claude-subscription">
  Bei einem Claude-Abonnement
</h3>

Bei einem Claude-Abonnement fordert Claude Code die eine-Stunden-TTL automatisch an. Die Nutzung ist in Ihrem Plan enthalten, anstatt pro Token abgerechnet zu werden, daher kostet die längere TTL Sie nichts extra und beeinflusst nur, wie lange Ihr Cache warm bleibt.

Wenn Sie das Nutzungslimit Ihres Plans überschritten haben und Claude Code auf [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) zurückgreift, wird diese Nutzung für Sie abgerechnet, daher senkt Claude Code die TTL automatisch auf fünf Minuten.

<h3 id="on-an-api-key-or-third-party-provider">
  Bei einem API-Schlüssel oder Drittanbieter-Provider
</h3>

Bei einem API-Schlüssel, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder Claude Platform on AWS zahlen Sie die Pro-Token-Raten, daher bleibt die TTL standardmäßig bei den günstigeren fünf Minuten. Um sich für die [eine-Stunden-TTL](https://platform.claude.com/docs/de/build-with-claude/prompt-caching#1-hour-cache-duration) anzumelden, setzen Sie `ENABLE_PROMPT_CACHING_1H=1`.

Bei Amazon Bedrock variieren Prompt Caching-Unterstützung, minimale cacheable Präfixlänge und eine-Stunden-TTL-Verfügbarkeit je nach Modell. Wenn Cache-Token-Zählungen bei Null bleiben, überprüfen Sie [unterstützte Modelle, Regionen und Limits](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) in der Amazon Bedrock-Dokumentation.

<h3 id="override-the-ttl">
  TTL überschreiben
</h3>

Setzen Sie `FORCE_PROMPT_CACHING_5M=1`, um die fünf-Minuten-TTL unabhängig von der Authentifizierung zu erzwingen. Dies ist nützlich, wenn Sie das Cache-Verhalten debuggen, die beiden TTLs vergleichen oder ein in [verwalteten Einstellungen](/docs/de/settings#settings-files) gesetztes `ENABLE_PROMPT_CACHING_1H` überschreiben.

<h2 id="cache-scope">
  Cache-Umfang
</h2>

In Claude Code ist der Cache effektiv auf einen Computer und ein Verzeichnis beschränkt. Der System-Prompt bettet das Arbeitsverzeichnis, die Plattform, die Shell, die OS-Version und Auto-Memory-Pfade ein, daher bauen zwei Sessions in verschiedenen Verzeichnissen unterschiedliche Präfixe auf und verfehlen den Cache des anderen. Das schließt Worktrees desselben Repositorys ein, da jeder Worktree sein eigenes Arbeitsverzeichnis hat.

Sessions, die Sie parallel im selben Verzeichnis ausführen, bauen passende Präfixe auf und lesen den Cache des anderen. Sequenzielle Sessions teilen das Präfix nur, wenn der Git-Status-Snapshot beim Start übereinstimmt, da der System-Prompt auch Branch und aktuelle Commits erfasst.

Der zugrunde liegende API-Cache ist breiter. Caches sind zwischen Organisationen isoliert, und bei einigen Providern [zwischen Workspaces innerhalb einer Organisation](https://platform.claude.com/docs/de/build-with-claude/prompt-caching#cache-storage-and-sharing). Innerhalb dieser Grenzen lesen zwei Anfragen mit demselben Modell und Präfix denselben Cache. Für Agent SDK-Aufrufer, die Flotten automatisierter Prozesse ausführen, siehe [Prompt Caching über Benutzer und Maschinen verbessern](/docs/de/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines), um die Pro-Maschinen-Abschnitte des System-Prompts zu unterdrücken und den Cache über Maschinen zu teilen.

<h2 id="check-cache-performance">
  Cache-Leistung überprüfen
</h2>

Cache-Leistung zeigt sich als zwei Token-Zählungen, die die API bei jeder Antwort meldet. Der direkteste Weg, sie live zu beobachten, ist ein [Statusline-Skript](/docs/de/statusline), das das `current_usage`-Objekt liest:

| Feld                          | Bedeutung                                                                                                         |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Tokens, die in diesem Turn in den Cache geschrieben werden, abgerechnet zum Cache-Schreibsatz                     |
| `cache_read_input_tokens`     | Tokens, die in diesem Turn aus dem Cache bereitgestellt werden, abgerechnet zu etwa 10% des Standard-Input-Satzes |

Ein hohes Lese-zu-Erstellungs-Verhältnis bedeutet, dass Caching gut funktioniert. Wenn die Erstellung Turn für Turn hoch bleibt, ändert sich etwas in Ihrem Präfix. Der Abschnitt [Aktionen, die den Cache ungültig machen](#actions-that-invalidate-the-cache) listet die üblichen Ursachen auf.

Für Sichtbarkeit über eine Organisation hinweg meldet der OpenTelemetry-Exporter Cache-Lese- und Erstellungs-Tokens pro Benutzer und Session. Siehe [Nutzung überwachen](/docs/de/monitoring-usage) für die Metrik- und Event-Attribut-Referenz.

<h2 id="subagents-and-the-cache">
  Subagents und der Cache
</h2>

Ein [Subagent](/docs/de/sub-agents) startet seine eigene Konversation mit seinem eigenen System-Prompt und Tool-Set, getrennt vom Parent. Er baut seinen eigenen Cache auf, beginnend ohne Cache-Hits bei seinem ersten Aufruf und wärmend über seine eigenen Turns. Subagents verwenden die fünf-Minuten-TTL auch bei einem Abonnement, da die automatische eine-Stunden-TTL für die Hauptkonversation gilt.

Der Cache des Parents ist unberührt. Von der Parent-Seite hängen der Aufruf und das Ergebnis des Subagents an die Konversation an, wobei das Parent-Präfix intakt bleibt.

Ein [Fork](/docs/de/sub-agents#fork-the-current-conversation) erbt dagegen den System-Prompt, die Tools und die Konversationshistorie des Parents genau, daher liest seine erste Anfrage den Cache des Parents. Der Komprimierungs-Zusammenfassungsaufruf, der in [Konversation komprimieren](#compacting-the-conversation) beschrieben wird, verwendet denselben Präfix-Sharing-Ansatz.

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

Um die Caching-Richtlinie über eine Organisation hinweg festzulegen, setzen Sie eine dieser oder die [TTL-Variablen](#cache-lifetime) in den `env`-Block von [verwalteten Einstellungen](/docs/de/settings#settings-files). Für normale Nutzung lassen Sie Caching aktiviert.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Lektionen aus dem Aufbau von Claude Code: Prompt Caching ist alles](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): die Designrationale für Plan Mode, aufgeschobenes Tool-Laden und Komprimierung
* [Erkunden Sie das Kontextfenster](/docs/de/context-window): was in den Kontext geladen wird und wann
* [Token-Nutzung reduzieren](/docs/de/costs#reduce-token-usage): Strategien jenseits von Caching zur Verwaltung der Kontextgröße
* [Kosten verfolgen und reduzieren](/docs/de/agent-sdk/cost-tracking): Cache-Token-Verfolgung und TTL-Konfiguration für Agent SDK-Aufrufer
* [Prompt Caching](https://platform.claude.com/docs/de/build-with-claude/prompt-caching): der zugrunde liegende API-Mechanismus, Breakpoints und Preisgestaltung
