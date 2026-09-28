> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungen verwalten

> Benennen, fortsetzen, verzweigen und wechseln Sie zwischen Claude Code-Gesprächen. Behandelt `--continue`, `--resume`, `--from-pr`, die `/resume`-Auswahl, Sitzungsbenennung, Exportieren von Transkripten und wo Transkripte gespeichert werden.

Eine Sitzung ist ein gespeichertes Gespräch, das an ein Projektverzeichnis gebunden ist. Claude Code speichert es lokal während Sie arbeiten, sodass Sie dort weitermachen können, wo Sie aufgehört haben, zu einem anderen Ansatz verzweigen oder zwischen Aufgaben wechseln können.

Die [Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions), [Claude Code im Web](/docs/de/claude-code-on-the-web) und die [VS Code-Erweiterung](/docs/de/vs-code#resume-past-conversations) verwalten jeweils ihre eigene Sitzungsverlauf. Diese Seite behandelt die CLI.

<h2 id="resume-a-session">
  Sitzung fortsetzen
</h2>

Sitzungen werden kontinuierlich in [lokale Transkriptdateien](#export-and-locate-session-data) gespeichert, während Sie arbeiten, sodass Sie nach dem Beenden oder Ausführen von `/clear` zu einer zurückkehren können. Verwenden Sie diese Einstiegspunkte:

| Befehl                              | Was er tut                                                                                                                                    |
| :---------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude --continue`                 | Setzt die neueste Konversation im aktuellen Verzeichnis fort                                                                                  |
| `claude --resume`                   | Öffnet die [Sitzungsauswahl](#use-the-session-picker)                                                                                         |
| `claude --resume <name>`            | Setzt die benannte Sitzung direkt fort                                                                                                        |
| `claude --resume <transcript-path>` | Setzt die Konversation fort, die in der `.jsonl` [Transkriptdatei](#where-transcripts-are-stored) unter diesem absoluten Pfad gespeichert ist |
| `claude --from-pr <number>`         | Öffnet die Sitzungsauswahl gefiltert nach Sitzungen, die mit diesem Pull Request verknüpft sind                                               |
| `/resume`                           | Wechselt zu einem anderen Gespräch innerhalb einer aktiven Sitzung                                                                            |

Claude Code lässt Sitzungen, die mit [`claude -p`](/docs/de/headless) oder dem [Agent SDK](/docs/de/agent-sdk/overview) erstellt wurden, aus der Sitzungsauswahl und aus `claude --continue` aus. Sie können eine trotzdem fortsetzen, indem Sie ihre Sitzungs-ID an `claude --resume <session-id>` übergeben. Mit `claude --continue` überspringt Claude Code auch [Sitzungen, deren erste Eingabeaufforderung `/loop` war](#where-the-session-picker-looks). Wenn Sie [`claude -p --continue`](/docs/de/headless#continue-conversations) ausführen, bezieht Claude Code `-p`-, SDK- und `/loop`-Sitzungen ein.

`claude --continue` öffnet eine [Hintergrund-Sitzung](/docs/de/agent-view), die beendet wurde, aber nicht eine, die noch läuft; das Öffnen beendeter Hintergrund-Sitzungen erfordert Claude Code v2.1.257 oder später. Wenn Ihre neueste Konversation eine ist, die Sie [in den Hintergrund verschoben haben](/docs/de/agent-view#send-the-session-to-the-background), und sie läuft dort noch, beendet Claude Code mit `Your most recent conversation is running in the background` und der ID dieser Sitzung. Hängen Sie sich an die Sitzung von [`claude agents`](/docs/de/agent-view#attach-to-a-session) an, oder führen Sie `claude --resume` aus, um eine andere auszuwählen.

Sie können `claude --resume <session-id>` aus jedem Verzeichnis ausführen: Claude Code sucht nach der ID zuerst im aktuellen Projektverzeichnis und seinen Git Worktrees, dann in jedem anderen Projekt auf dieser Maschine, sodass es eine Sitzung findet, die anderswo gestartet wurde oder sich mit [`/cd`](/docs/de/commands) verschoben hat. Die projektübergreifende Suche löst die ID nur auf, wenn genau ein anderes Projekt ein Transkript mit Nachrichten dafür enthält, sodass eine manuell kopierte Kopie Claude Code veranlasst, nicht gefunden zu melden, anstatt eine beliebige Kopie fortzusetzen. Wenn keine gespeicherte Sitzung der ID entspricht, meldet Claude Code `No conversation found with session ID: <session-id>`. Vor v2.1.223 stoppte die Suche im aktuellen Projektverzeichnis und seinen Git Worktrees, sodass Sie die Sitzung aus dem Verzeichnis fortsetzen mussten, in dem sie zuletzt funktioniert hat.

<h3 id="what-a-resumed-session-restores">
  Was eine fortgesetzte Sitzung wiederherstellt
</h3>

Eine fortgesetzte Sitzung stellt das Gespräch zusammen mit dem darin gespeicherten Zustand wieder her:

* Gesprächsverlauf: der vollständige Verlauf, einschließlich Werkzeugaufrufe und Ergebnisse. Ein Werkzeug, das noch lief, als der vorherige Prozess endete, beispielsweise bei einem Absturz, wird nicht beendet oder erneut ausgeführt, wenn Sie fortsetzen. Claude sieht den Aufruf als unterbrochen markiert, bevor sein Ergebnis aufgezeichnet wurde, und wird angewiesen, zu überprüfen, ob er wirksam wurde, bevor er ihn erneut ausführt, es sei denn, [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/de/env-vars#variables) ist gesetzt. Vor v2.1.281 ließ Claude Code den unterbrochenen Aufruf aus dem Gespräch fallen oder zeigte ihn Claude als einen an, den Sie unterbrochen haben.
* Modell: Die Sitzung wird auf dem Modell fortgesetzt, das sie verwendet hat. Das Modell wird nicht wiederhergestellt, wenn es eingestellt wurde oder nicht von `availableModels` zulässig ist, wenn ein `--model`-Flag oder eine `ANTHROPIC_MODEL`-Familie-Umgebungsvariable beim Start eine auswählt, oder bei Anbietern, die anbieterspezifische Bereitstellungs-IDs verwenden, wie [Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry](/docs/de/third-party-integrations); siehe [Modellkonfiguration](/docs/de/model-config#setting-your-model) für die Auflösungsreihenfolge.
* Agent: Eine Sitzung, die mit [`--agent`](/docs/de/sub-agents#invoke-subagents-explicitly) oder der `agent`-Einstellung gestartet wurde, wird als dieser Agent fortgesetzt und behält seine Werkzeugbeschränkungen und sein Modell. Übergeben Sie `--agent` beim Fortsetzen, um einen anderen auszuwählen; für die Systemaufforderung in beiden Fällen siehe [Systemaufforderungs-Flags in fortgesetzten Konversationen](/docs/de/cli-reference#system-prompt-flags-in-resumed-conversations). Claude Code sucht nach dem Agent an zwei Stellen: im ursprünglichen Verzeichnis der Sitzung, sofern Sie [diesem Arbeitsbereich vertraut haben](/docs/de/permissions#project-allow-rules-and-workspace-trust), und dann im Verzeichnis, aus dem Sie fortsetzen, sodass ein projektbezogener Agent weiterhin geladen wird, wenn Sie aus einem anderen Verzeichnis fortsetzen. Wenn Claude Code den Agent an keiner Stelle findet, wird die Sitzung mit den Standard-Werkzeugen fortgesetzt und zeigt eine [Warnung mit dem Namen des Agenten](/docs/de/errors#session-agent-no-longer-available).
* Berechtigungsmodus: Wenn Sie aus einem Terminal mit `claude --continue`, `claude --resume <session-id>` oder `claude --resume <name>` fortsetzen, wenn der Name einer Sitzung entspricht, ohne `-p`, stellt Claude Code den Berechtigungsmodus wieder her, in dem sich die Sitzung befand, außer in den Fällen in [Berechtigungsmodus beim Fortsetzen](#permission-mode-on-resume), die auch die Sitzungsauswahl, `/resume` und das Fortsetzen mit `claude -p` abdecken. Übergeben Sie `--permission-mode` oder `--dangerously-skip-permissions`, um den wiederhergestellten Modus zu überschreiben.
* Aktives Ziel: Ein [Ziel](/docs/de/goal#resume-with-an-active-goal), das noch aktiv war, als die Sitzung endete, wird übertragen; seine Rundenzahl, sein Timer und seine Token-Ausgaben-Baseline werden zurückgesetzt.
* Geplante Aufgaben: [Aufgaben, die nicht abgelaufen sind](/docs/de/scheduled-tasks#limitations), werden wiederhergestellt. Hintergrund-Bash- und Monitor-Aufgaben nicht.

Nicht jedes Konfigurationsflag aus dem ursprünglichen Start wird wiederhergestellt. Wenn die Sitzung von `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model` oder mit `--add-dir` hinzugefügten Verzeichnissen abhängig war, übergeben Sie diese erneut, wenn Sie fortsetzen; mit `/add-dir` während der Sitzung hinzugefügte Verzeichnisse werden ebenfalls nicht wiederhergestellt, obwohl die Sitzungsauswahl sie weiterhin verwendet, um die Sitzung zu lokalisieren. Die Standard-Einstellungsdateien wie `settings.json` und `settings.local.json` werden beim Start erneut gelesen, sodass Konfigurationen, die sich darin befinden, nicht erneut übergeben werden müssen. Für `--system-prompt` und `--append-system-prompt` siehe [Systemaufforderungs-Flags in fortgesetzten Konversationen](/docs/de/cli-reference#system-prompt-flags-in-resumed-conversations).

<h4 id="permission-mode-on-resume">
  Berechtigungsmodus beim Fortsetzen
</h4>

Welcher Berechtigungsmodus Claude Code eine fortgesetzte Sitzung startet, hängt davon ab, wie Sie fortsetzen:

* Terminal: `claude --continue`, `claude --resume <session-id>` oder `claude --resume <name>`, wenn der Name einer Sitzung entspricht, ohne `-p`. Claude Code stellt den Berechtigungsmodus wieder her, in dem sich die Sitzung befand, außer in den Fällen in der Tabelle. Übergeben Sie `--permission-mode` oder `--dangerously-skip-permissions`, um den wiederhergestellten Modus zu überschreiben.
* Nicht-interaktiv: `claude -p --resume` oder `claude -p --continue`. Claude Code startet den Lauf im Berechtigungsmodus, in dem ein neuer `claude -p`-Lauf gestartet würde, außer dass eine Sitzung, die im Plan-Modus endete, unter den [unten stehenden Bedingungen](#resume-in-plan-mode-with-p) im Plan-Modus fortgesetzt wird.
* VS Code: Das Gesprächsfenster der Erweiterung. Die Tabelle behandelt nur ein Gespräch, das im Plan-Modus endete; für den Rest siehe [Frühere Gespräche fortsetzen](/docs/de/vs-code#resume-past-conversations).
* Sitzungsauswahl beim Start: Eine Sitzung, die Sie aus der [Sitzungsauswahl](#use-the-session-picker) auswählen, unabhängig davon, ob Sie sie mit `claude --resume` allein, `claude --from-pr` oder einem Namen öffnet, der mehr als einer Sitzung entspricht. Claude Code stellt den gespeicherten Berechtigungsmodus nicht wieder her. Es startet die Sitzung im Berechtigungsmodus, in dem es eine neue Sitzung aus derselben Befehlszeile starten würde.
* `/resume` innerhalb einer Sitzung, mit oder ohne Argument: Claude Code stellt den gespeicherten Berechtigungsmodus nicht wieder her. Das Gespräch, zu dem Sie wechseln, wird im Berechtigungsmodus Ihrer aktuellen Sitzung fortgesetzt.

Das Wiederherstellen des Plan-Modus auf den nicht-interaktiven und VS Code-Pfaden erfordert Claude Code v2.1.246 oder später. Jede Zeile benennt den Berechtigungsmodus, in dem die Sitzung endete, welche der Terminal-, nicht-interaktiven und VS Code-Pfade Sie ihn fortsetzen, und den Berechtigungsmodus, in dem Claude Code die fortgesetzte Sitzung startet.

| Sitzung endete in   | Wie Sie fortsetzen                                                                     | Berechtigungsmodus nach dem Fortsetzen                                                                                                                                                                                                                                                                                                                                                         |
| :------------------ | :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bypassPermissions` | Terminal                                                                               | Der Berechtigungsmodus, in dem eine neue Sitzung gestartet würde. Um [Berechtigungen zu umgehen](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode), aktivieren Sie es beim Start mit einem seiner Start-Flags oder `permissions.defaultMode: "bypassPermissions"` in [Benutzer-, `--settings`- oder verwalteten Einstellungen](/docs/de/settings-reference#permissions-defaultmode) |
| `plan`              | Terminal                                                                               | Der Berechtigungsmodus, in dem eine neue Sitzung gestartet würde                                                                                                                                                                                                                                                                                                                               |
| `auto`              | Terminal                                                                               | `auto`, nur wenn Ihr Konto weiterhin die [Auto-Modus-Anforderungen](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) erfüllt                                                                                                                                                                                                                                                             |
| Manuell             | Terminal                                                                               | Manuell, wenn eine neue Sitzung im Auto-Modus aus dem [integrierten Standard](/docs/de/permission-modes#which-mode-a-session-starts-in) gestartet würde. Wenn ein `defaultMode` aus einer Einstellungsdatei [wirksam wird](/docs/de/permission-modes#which-mode-a-session-starts-in), startet Claude Code die fortgesetzte Sitzung stattdessen in diesem Modus                                           |
| `plan`              | Nicht-interaktiv, unter den [unten stehenden Bedingungen](#resume-in-plan-mode-with-p) | Plan-Modus                                                                                                                                                                                                                                                                                                                                                                                     |
| Beliebiger Modus    | Nicht-interaktiv, in jedem anderen Fall                                                | Der Berechtigungsmodus, in dem ein neuer `claude -p`-Lauf gestartet würde                                                                                                                                                                                                                                                                                                                      |
| `plan`              | VS Code                                                                                | Plan-Modus, mit [den Ausnahmen auf der VS Code-Seite](/docs/de/vs-code#resume-past-conversations)                                                                                                                                                                                                                                                                                                   |

<h5 id="resume-in-plan-mode-with-p">
  Plan-Modus mit `-p` fortsetzen
</h5>

Ein `claude -p --resume`- oder `claude -p --continue`-Lauf wird nur im Plan-Modus fortgesetzt, wenn alle vier Bedingungen erfüllt sind:

* Sie übergeben [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags), damit Claude Code den Plan zur Genehmigung präsentieren kann
* Sie übergeben nicht `--permission-mode` oder `--dangerously-skip-permissions`
* Sie übergeben nicht `--fork-session`
* Der Lauf wird nicht über [Kanäle](/docs/de/channels) gestartet

<h3 id="resume-from-a-summary">
  Aus einer Zusammenfassung fortsetzen
</h3>

Bei einem Pro- oder Max-Plan öffnet Claude Code einen Dialog, wenn Sie eine Sitzung fortsetzen, die länger als etwa eine Stunde inaktiv war und über 100.000 Token umfasst, bevor Sie Ihre erste Nachricht senden. Der [Prompt-Cache](/docs/de/prompt-caching#cache-lifetime) der Sitzung ist bis dahin abgelaufen, sodass die nächste Anfrage die vollständige Verlauf einmal verarbeitet, unabhängig davon, welche Option des Dialogs Sie wählen.

Der Dialog bietet drei Möglichkeiten, die Sitzung fortzusetzen. Sie unterscheiden sich darin, wie viel des Gesprächs jede in spätere Anfragen überträgt, was ein Kompromiss zwischen dem Beibehalten aller Details und dem Senden weniger Token pro Anfrage ist:

* **Aus Zusammenfassung fortsetzen**: Führt [`/compact`](/docs/de/context-window#what-survives-compaction) sofort aus. Claude Code sendet eine Zusammenfassungsanfrage über die vollständige Verlauf, ersetzt dann die Verlauf durch die Zusammenfassung, Ihre letzten Austausche und bis zu fünf kürzlich gelesene Dateien. Spätere Anfragen tragen die Zusammenfassung anstelle der vollständigen Verlauf.
* **Vollständige Sitzung unverändert fortsetzen**: Lädt das Gespräch unverändert. Nachdem Sie Ihre erste Nachricht senden, verarbeitet Claude Code die vollständige Verlauf erneut und speichert sie zwischen, liest sie dann aus dem Cache bei späteren Anfragen, während der Cache warm bleibt.
* **Nicht mehr fragen**: Setzt die vollständige Sitzung fort und stoppt die Anzeige des Dialogs bei allen zukünftigen Fortsetzungen.

Das Fortsetzen unverändert behält alle Details des Gesprächs zur Verfügung, mit Kosten pro Anfrage, die mit der Größe des Gesprächs skalieren. Das Fortsetzen aus der Zusammenfassung kostet weniger bei jeder späteren Anfrage, da es die Zusammenfassung anstelle der vollständigen Verlauf trägt, aber was die Zusammenfassung auslässt, ist nicht mehr in Claudes Kontext. Siehe [Warum die Nutzung in einer langen Sitzung steigt](/docs/de/costs#why-usage-climbs-in-a-long-session), um zu erfahren, woher diese Kosten pro Anfrage kommen.

<h3 id="where-the-session-picker-looks">
  Wo die Sitzungsauswahl sucht
</h3>

Claude Code speichert Sitzungen pro Projektverzeichnis. Standardmäßig zeigt die Sitzungsauswahl:

* Sitzungen aus dem aktuellen Worktree, einschließlich [Hintergrund-Sitzungen](/docs/de/agent-view), die in der Liste als `bg` gekennzeichnet sind
* Sitzungen, die anderswo gestartet wurden und das aktuelle Verzeichnis mit `/add-dir` hinzugefügt haben

Verwenden Sie `Ctrl+W`, um auf alle Worktrees des Repositorys zu erweitern, oder `Ctrl+A`, um auf jedes Projekt auf dieser Maschine zu erweitern.

Sitzungen, deren erste Eingabeaufforderung ein [`/loop`](/docs/de/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop)-Befehl war, werden nicht in der Auswahl angezeigt, und `claude --continue` überspringt sie ebenfalls. Das Ausführen von `/loop` später in einem Gespräch versteckt die Sitzung nicht. Vor v2.1.211 versteckte ein `/loop`-Lauf früh in einem Gespräch die Sitzung dauerhaft aus der Auswahl.

Das Verschieben einer Sitzung mit [`/cd`](/docs/de/commands) verlagert sie in den Projektspeicher des neuen Verzeichnisses, sodass sie danach in der Auswahl dieses Verzeichnisses angezeigt wird. Ab v2.1.196 bleibt eine verschobene Sitzung aus der Auswahl des alten Verzeichnisses ausgeschlossen, auch nach einem Absturz oder erzwungenen Beenden. In früheren Versionen konnte sie auch nach einem nicht sauberen Beenden in der Liste des alten Verzeichnisses erneut angezeigt werden, wenn der alte Pfad Sonderzeichen wie Unterstriche enthielt.

Wenn Sie eine Sitzung aus einem anderen Worktree desselben Repositorys auswählen, setzt Claude Code sie an Ort und Stelle fort; wenn der Worktree der Sitzung nicht mehr existiert, [setzt Claude Code sie in Ihrem aktuellen Verzeichnis fort](/docs/de/worktrees#resume-a-worktree-session). Wenn Sie eine Sitzung aus einem nicht verwandten Projekt auswählen, kopiert Claude Code stattdessen einen `cd`- und Resume-Befehl in Ihre Zwischenablage. Wenn das Verzeichnis dieses Projekts nicht mehr existiert, setzt Claude Code die Sitzung in Ihrem aktuellen Verzeichnis fort, anstatt einen `cd`-Befehl zu kopieren, der fehlschlagen würde.

Das Fortsetzen nach Name wird über das aktuelle Repository und seine Worktrees hinweg aufgelöst. Beide Formen suchen nach einer genauen Übereinstimmung und setzen sie direkt fort, auch wenn sie sich in einem anderen Worktree befindet:

| Befehl                   | Genaue Übereinstimmung | Mehrdeutiger Name                                                                             |
| :----------------------- | :--------------------- | :-------------------------------------------------------------------------------------------- |
| `claude --resume <name>` | Setzt direkt fort      | Öffnet die Sitzungsauswahl mit dem Namen als Suchbegriff vorausgefüllt                        |
| `/resume <name>`         | Setzt direkt fort      | Meldet einen Fehler; führen Sie `/resume` ohne Argument aus, um die Sitzungsauswahl zu öffnen |

<h2 id="name-your-sessions">
  Benennen Sie Ihre Sitzungen
</h2>

Geben Sie Sitzungen aussagekräftige Namen, damit sie in der Sitzungsauswahl auffindbar und nach Name wiederaufnehmbar sind. Dies ist am wichtigsten, wenn Sie an mehreren Aufgaben parallel arbeiten.

| Wann                              | So legen Sie den Namen fest                                                                                                                                                                                                     |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Beim Start                        | `claude -n auth-refactor`                                                                                                                                                                                                       |
| Während einer Sitzung             | `/rename auth-refactor`. Der Name wird auch in der Eingabeaufforderungsleiste angezeigt                                                                                                                                         |
| Aus der Sitzungsauswahl           | Markieren Sie eine Sitzung und drücken Sie `Ctrl+R`                                                                                                                                                                             |
| Bei Plan-Annahme                  | Das Akzeptieren eines Plans im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) gibt der Sitzung einen generierten Titel basierend auf dem Plan, es sei denn, Sie haben bereits einen Namen festgelegt |
| Von claude.ai oder der Claude-App | Benennen Sie eine [Remote-Control-Sitzung](/docs/de/remote-control#connect-from-another-device) um; Claude Code wendet denselben Namen in der CLI an. Erfordert Claude Code v2.1.221 oder später                                     |
| Von der Desktop-App               | Benennen Sie eine Sitzung in der [Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions) um                                                                                                                                   |

Sobald Sie eine Sitzung über eine CLI-Route oder von claude.ai aus benannt haben, kehren Sie mit `claude --resume <name>` oder `/resume <name>` zu ihr zurück; eine Desktop-App-Sitzung wird in der App fortgesetzt, die ihre eigene Sitzungshistorie führt. Siehe [Sitzung fortsetzen](#resume-a-session), um zu erfahren, wie die Namensauflösung über Worktrees hinweg funktioniert.

Wenn Sie eine interaktive Sitzung mit einem Namen starten oder fortsetzen, den bereits eine andere aktive Sitzung auf diesem Computer verwendet, oder eine Sitzung in einen solchen Namen umbenennen, behält Claude Code den Namen bei der Sitzung, die ihn bereits hat, benennt Ihre Sitzung in eine Variante mit einem zweistelligen Suffix um, z. B. `auth-refactor-graceful-unicorn`, und teilt Ihnen dies mit. Führen Sie `/rename` mit einem neuen Namen aus, wenn Sie lieber selbst einen auswählen möchten. Vor v2.1.232 behielten beide Sitzungen den Namen.

In drei Fällen benennt Claude Code das Duplikat nicht um, sodass Sie in Auflistungen immer noch zwei Sitzungen mit demselben Namen sehen können:

* Es prüft nicht auf KI-generierte Titel oder Standard-Anzeigenamen.
* Es prüft nicht den `--name` einer [Hintergrund-](/docs/de/agent-view#from-your-shell) oder `-p`-Sitzung beim Start.
* Es kann eine Sitzung auf einer früheren Version von Claude Code nicht umbenennen.

Sitzungen, die Sie nicht benennen, erhalten immer noch zwei Bezeichnungen, die Claude Code zuweist. Nur der generierte Titel funktioniert als Resume-Handle:

* Standard-Anzeigename: Interaktive Sitzungen, die Sie nie benennen, erhalten beim Start einen Standard-Anzeigenamen. Erfordert Claude Code v2.1.196 oder später. Der Standard kombiniert den Namen des Arbeitsverzeichnisses mit einem zweistelligen Suffix, beispielsweise `my-app-3f`, und identifiziert die Sitzung in Auflistungen laufender Sitzungen, wie z. B. [Agent-Ansicht](/docs/de/agent-view) und `claude agents --json` Ausgabe. Der Standard ist kein Resume-Handle. Wenn Sie ihn an `claude --resume` oder `/resume` übergeben, findet Claude Code die Sitzung nicht. Das Benennen der Sitzung ersetzt den Standard in diesen Auflistungen, ebenso wie das Akzeptieren eines Plans.
* Generierter Titel: Wenn Sie eine Sitzung nicht benennen, generiert Claude Code einen Sitzungstitel für sie. Der Titel ist eine kurze Zusammenfassung Ihres ersten Prompts, geschrieben durch eine Hintergrundanfrage an das kleine/schnelle Modell, normalerweise ein Haiku-Klasse-Modell. Eine `claude -p`-Ausführung, die Sie direkt von einer Shell oder einem Skript aus starten, erhält keinen. Das Akzeptieren eines Plans ersetzt den generierten Titel durch einen Titel basierend auf dem Plan. Das Benennen der Sitzung ersetzt ihn ebenfalls. Sie sehen den Titel des ersten Prompts in der [Sitzungsauswahl](#use-the-session-picker) und im Statusline-Feld [`session_name`](/docs/de/statusline), wenn kein Name festgelegt ist. Der Plan-Titel wird an denselben zwei Stellen angezeigt und auch in den Auflistungen laufender Sitzungen, wo er den Platz des Standard-Anzeigenamens einnimmt. Sie können jeden Titel an `claude --resume` oder `/resume` übergeben, und Claude Code löst ihn auf dieselbe Weise auf wie einen Namen, den Sie festgelegt haben.

<h2 id="use-the-session-picker">
  Verwenden Sie die Sitzungsauswahl
</h2>

Führen Sie `/resume` innerhalb einer Sitzung oder `claude --resume` ohne Argumente aus, um die interaktive Sitzungsauswahl zu öffnen. Verwenden Sie diese Tastaturkürzel zum Navigieren, Suchen und Erweitern der Liste:

| Tastaturkürzel                                           | Aktion                                                                                                                                                                                                     |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                                | Navigieren Sie zwischen Sitzungen                                                                                                                                                                          |
| `→` / `←`                                                | Erweitern oder reduzieren Sie gruppierte Sitzungen                                                                                                                                                         |
| `Enter`                                                  | Setzt die markierte Sitzung fort                                                                                                                                                                           |
| `Space`                                                  | Zeigt eine Vorschau des Sitzungsinhalts an. `Ctrl+V` funktioniert auch auf Terminals, die es nicht als Einfügen erfassen                                                                                   |
| `Ctrl+R`                                                 | Benennen Sie die markierte Sitzung um                                                                                                                                                                      |
| `/` oder ein beliebiges druckbares Zeichen außer `Space` | Geben Sie den Suchmodus ein und filtern Sie Sitzungen. Fügen Sie eine GitHub-, GitHub Enterprise-, GitLab- oder Bitbucket-Pull- oder Merge-Request-URL ein, um die Sitzung zu finden, die sie erstellt hat |
| `Ctrl+A`                                                 | Zeigen Sie Sitzungen aus allen Projekten auf dieser Maschine an. Drücken Sie erneut, um zum aktuellen Repository zurückzukehren                                                                            |
| `Ctrl+W`                                                 | Zeigen Sie Sitzungen aus allen Worktrees des aktuellen Repositorys an. Drücken Sie erneut, um zum aktuellen Worktree zurückzukehren. Wird nur in Multi-Worktree-Repositorys angezeigt                      |
| `Ctrl+B`                                                 | Filtern Sie zu Sitzungen aus dem aktuellen Git-Branch. Drücken Sie erneut, um alle Branches anzuzeigen                                                                                                     |
| `Esc`                                                    | Beenden Sie die Sitzungsauswahl oder den Suchmodus                                                                                                                                                         |

Jede Zeile zeigt den Sitzungsnamen, falls festgelegt, andernfalls den KI-generierten Sitzungstitel, die Gesprächszusammenfassung oder die erste Eingabeaufforderung, zusammen mit der Zeit seit der letzten Aktivität, dem Git-Branch und der Dateigröße. Erweitern Sie auf alle Projekte mit `Ctrl+A`, um auch den Projektpfad jeder Sitzung anzuzeigen.

Sitzungen, die mit `/branch` oder `--fork-session` erstellt wurden, erhalten ihre eigenen Sitzungs-IDs und werden als separate Zeilen angezeigt. Wenn die Auswahl mehr als einen Eintrag für dieselbe Sitzung findet, werden diese unter einer einzelnen Zeile gruppiert. Drücken Sie `→`, um eine Gruppe zu erweitern.

Wenn Claude Code die Sitzung, die Sie aus der `claude --resume`-Auswahl auswählen, nicht laden kann, wird [`Failed to resume the conversation`](/docs/de/errors#failed-to-resume-the-conversation) mit einem Befehl zum Wiederholen ausgegeben, dann wird mit Code 1 beendet. Aus der `/resume`-Auswahl innerhalb einer Sitzung meldet Claude Code den Fehler, und Ihr aktuelles Gespräch läuft weiter.

<h2 id="branch-a-session">
  Verzweigen Sie eine Sitzung
</h2>

Das Verzweigen erstellt eine Kopie des bisherigen Gesprächs und wechselt Sie hinein, wobei das Original intakt bleibt. Verwenden Sie es, um einen anderen Ansatz zu versuchen, ohne den Weg zu verlieren, auf dem Sie waren.

Führen Sie innerhalb einer Sitzung `/branch` mit einem optionalen Namen aus:

```text theme={null}
/branch try-streaming-approach
```

Wenn Sie den Namen weglassen, benennt Claude Code den neuen Branch nach der ersten Eingabeaufforderung im Gespräch. Ab v2.1.198 gilt dies auch nach [Komprimierung](/docs/de/how-claude-code-works#when-context-fills-up); frühere Versionen fielen auf den wörtlichen Namen `Branched conversation` zurück, anstatt die Komprimierungszusammenfassung zu überschreiten, um die ursprüngliche erste Eingabeaufforderung zu finden.

Kombinieren Sie von der Befehlszeile aus `--continue` oder `--resume` mit `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

Die `/branch`-Bestätigung gibt zwei Sitzungs-IDs aus: den neuen Branch, in dem Sie sich jetzt befinden, und das Original. Das Original bleibt unverändert auf der Festplatte und bleibt in der Sitzungsauswahl verfügbar; kehren Sie mit `/resume <original-name>` oder durch Übergabe seiner ID an `/resume` zu ihm zurück.

`/branch` kopiert das Transkript und wechselt den laufenden Claude Code-Prozess, um darin zu schreiben. Diese Unterscheidung bestimmt, was der Branch erbt:

| Status                                                                                                                                                                    | Nach `/branch`                                                                                                                                                                                                                                     |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Gesprächsverlauf                                                                                                                                                          | In den Branch bis zu dem Punkt kopiert, an dem Sie `/branch` ausgeführt haben                                                                                                                                                                      |
| Berechtigungen für „Für diese Sitzung zulassen"                                                                                                                           | Übertragen; der Branch läuft im selben Prozess, daher gelten Ihre bestehenden Genehmigungen weiterhin. Wenn Sie mit `--fork-session` in einen separaten Prozess verzweigen, startet der neue Prozess ohne diese und Sie genehmigen sie dort erneut |
| Laufende [Hintergrund-Subagenten](/docs/de/sub-agents#run-subagents-in-foreground-or-background) und [Hintergrund-Bash-Befehle](/docs/de/interactive-mode#background-bash-commands) | Laufen weiter. Ihre Ausgabe wird in dem neuen Branch angezeigt, in den Sie gewechselt haben, nicht in der ursprünglichen Sitzung                                                                                                                   |
| [Remote Control](/docs/de/remote-control)-Verbindung                                                                                                                           | Bleibt verbunden. Ein Telefon oder Browser, das mit der Sitzung verbunden ist, folgt Ihnen in den Branch und empfängt dort weiterhin neue Nachrichten                                                                                              |

Wenn Sie dieselbe Sitzung in zwei Terminals ohne Verzweigung fortsetzen, werden Nachrichten von beiden in ein Transkript verschachtelt. Für Checkpoint-basiertes Zurückspulen innerhalb einer einzelnen Sitzung siehe [Checkpointing](/docs/de/checkpointing).

<h2 id="manage-context-within-a-session">
  Verwalten Sie den Kontext innerhalb einer Sitzung
</h2>

Diese Befehle steuern, was sich im Kontextfenster befindet, ohne die Sitzung zu verlassen:

* **`/clear`**: Beginnen Sie mit einem leeren Kontext von vorne. Claude Code speichert das vorherige Gespräch; nehmen Sie es mit `/resume` wieder auf, oder, im selben Claude Code-Prozess, aus [dem Eintrag der vorherigen Sitzung im Rewind-Menü](/docs/de/checkpointing#rewind-past-a-cleared-conversation). Ohne Argument behält das neue Gespräch einen Namen bei, den Sie mit `--name` oder `/rename` festgelegt haben, aber nicht einen von der KI generierten Sitzungstitel. Um stattdessen das Gespräch zu benennen, das Sie verlassen, übergeben Sie den Namen, wie in `/clear release-prep`; das neue Gespräch beginnt dann ohne Namen
* **`/compact [instructions]`**: Ersetzen Sie den Verlauf durch eine Zusammenfassung, optional fokussiert auf das, was Sie angeben
* **`/context`**: Zeigen Sie an, was derzeit Kontext verbraucht

Wie die Komprimierung mit CLAUDE.md, Skills und Regeln interagiert, finden Sie im [Kontextfenster-Leitfaden](/docs/de/context-window). Strategien, wann Sie löschen oder komprimieren sollten, finden Sie unter [Best Practices](/docs/de/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Exportieren und lokalisieren Sie Sitzungsdaten
</h2>

Führen Sie `/export` aus, um ein Menü zu öffnen, das Ihnen ermöglicht, das aktuelle Gespräch in Ihre Zwischenablage zu kopieren oder als Nur-Text-Datei zu speichern, wobei Nachrichten und Tool-Ausgaben als lesbarer Text gerendert werden. Übergeben Sie einen Dateinamen, um das Menü zu überspringen und direkt in diese Datei zu schreiben.

<h3 id="access-conversations-from-scripts">
  Zugriff auf Gespräche aus Skripten
</h3>

`/export` erzeugt ein gerendertes Transkript zum Lesen durch eine Person. Die folgenden Schnittstellen erzeugen strukturierte Daten zum Analysieren durch ein Skript: ein JSON-Ergebnis aus einer Ausführung, der Pfad zur Transkriptdatei einer Sitzung oder ein Live-Stream von Ereignissen. Wählen Sie basierend darauf, was das Skript auslöst:

* **Claude einmal ausführen und das Ergebnis erfassen**: Rufen Sie `claude -p` mit [`--output-format json` oder `stream-json`](/docs/de/headless#get-structured-output) auf, um das Ergebnis, die Sitzungs-ID, die Nutzung und die Kosten einer nicht-interaktiven Ausführung als strukturiertes JSON zu erfassen.
* **Eine vorhandene Sitzung eine Frage stellen**: Übergeben Sie eine Sitzungs-ID an [`claude -p --resume`](/docs/de/headless#continue-conversations), um eine Folgeanfrage zu senden, z. B. eine Zusammenfassungsanfrage, und die strukturierte Antwort zu erfassen.
* **Auf Sitzungsereignisse reagieren**: Lesen Sie das Feld `transcript_path`, das [Hooks](/docs/de/hooks#common-input-fields) und [Statuszeilen-Befehle](/docs/de/statusline#available-data) als Eingabe erhalten. Ein `SessionEnd`-Hook kann das Transkript archivieren, wenn eine Sitzung endet.
* **Claude in eine TypeScript- oder Python-App einbetten**: Verwenden Sie das [Agent SDK](/docs/de/agent-sdk/overview), um jede Nachricht programmgesteuert zu empfangen.

Das folgende Beispiel verwendet die zweite Schnittstelle. Es sendet eine Folgeanfrage an eine vorhandene Sitzung und liest die Antwort mit `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Wo Transkripte gespeichert sind
</h3>

Standardmäßig speichert Claude Code Transkripte als JSONL unter `~/.claude/projects/<project>/<session-id>.jsonl`, wobei `<project>` Ihr Arbeitsverzeichnispfad mit nicht-alphanumerischen Zeichen ist, die durch `-` ersetzt wurden. Für ein Arbeitsverzeichnis, dessen konvertierter Name 200 Zeichen überschreitet, kürzt Claude Code den Namen auf 200 Zeichen und hängt einen Hash des vollständigen Pfads an, damit der Verzeichnisname innerhalb der Dateisystem-Limits bleibt.

Jede Zeile ist ein JSON-Objekt für eine Nachricht, Tool-Verwendung oder Metadateneintrag. Das Eintragsformat ist intern für Claude Code und ändert sich zwischen Versionen, daher können Skripte, die diese Dateien direkt analysieren, bei jeder Veröffentlichung unterbrochen werden. Um auf Sitzungsdaten aufzubauen, verwenden Sie stattdessen `/export` oder die [Skript-Schnittstellen](#access-conversations-from-scripts).

Der Speicherort, die Aufbewahrung und das Schreibverhalten sind konfigurierbar:

| Zu                                                                                                                        | Einstellen                                                                                  | Wo                                                                |
| ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Speicher von `~/.claude` verschieben                                                                                      | [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars)                                                         | Umgebungsvariable                                                 |
| [Benennen Sie das Verzeichnis `<project>` selbst](#name-the-project-directory-yourself)                                   | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/de/env-vars)                                              | Umgebungsvariable                                                 |
| Ändern Sie die 30-Tage-Aufbewahrung                                                                                       | [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays)                             | `settings.json`                                                   |
| Legen Sie ein Alterslimit für [Claude Desktop und Cowork-Transkripte](/docs/de/claude-directory#cleaned-up-automatically) fest | [`desktopSessionCleanupPeriodDays`](/docs/de/settings-reference#desktopsessioncleanupperioddays) | Benutzereinstellungen, verwaltete Einstellungen oder `--settings` |
| Transkriptschreibvorgänge in allen Modi unterdrücken                                                                      | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/de/env-vars)                                           | Umgebungsvariable                                                 |
| Schreibvorgänge für eine nicht-interaktive Ausführung unterdrücken                                                        | [`--no-session-persistence`](/docs/de/cli-reference)                                             | CLI-Flag mit `claude -p`                                          |

<h3 id="delete-session-data">
  Sitzungsdaten löschen
</h3>

Transkripte werden unter den [Aufbewahrungsbereinigungsregeln](/docs/de/claude-directory#cleaned-up-automatically) gelöscht. Um die Transkripte eines Projekts und den zugehörigen Status schneller zu löschen, führen Sie [`claude project purge`](/docs/de/claude-directory#clear-local-data) aus. Wenn Sie eine [Hintergrund-Sitzung](/docs/de/agent-view) mit [`claude rm <id>`](/docs/de/agent-view#what-deleting-a-session-removes) löschen, bleibt ihr Transkript auf der Festplatte und bleibt über `claude --resume` verfügbar.

<h3 id="name-the-project-directory-yourself">
  Benennen Sie das Verzeichnis `<project>` selbst
</h3>

Standardmäßig leitet Claude Code den Namen `<project>` vom gesamten Arbeitsverzeichnispfad ab. Um den Namen selbst zu wählen, setzen Sie `CLAUDE_CODE_PROJECT_DIR_NAME` zusammen mit `CLAUDE_CONFIG_DIR`. Claude Code speichert dann die Transkripte dieser Sitzung und [Auto-Memory](/docs/de/memory#auto-memory) unter Ihrem Namen. Dies eignet sich für einen Host, der Claude Code einbettet und jeder Sitzung ein eigenes Konfigurationsverzeichnis gibt. Erfordert Claude Code v2.1.234 oder später.

Beispielsweise behält dieser Start die Daten von Mandant A unter `/srv/tenant-a` und benennt sein Projektverzeichnis `work`:

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code schreibt die Transkripte der Sitzung in `/srv/tenant-a/projects/work/` und sein Auto-Memory in `/srv/tenant-a/projects/work/memory/`, unabhängig davon, welches Arbeitsverzeichnis verwendet wird.

Drei Regeln gelten, wenn Sie es setzen:

* **Setzen Sie auch `CLAUDE_CONFIG_DIR`**: Der Name variiert nicht mit dem Arbeitsverzeichnis, daher würde er unter dem Standard `~/.claude` die Transkripte und das Auto-Memory aller Projekte in ein Verzeichnis zusammenführen. Claude Code ignoriert `CLAUDE_CODE_PROJECT_DIR_NAME`, wenn `CLAUDE_CONFIG_DIR` nicht gesetzt ist.
* **Verwenden Sie 1-64 Buchstaben, Ziffern, Bindestriche oder Unterstriche**: Verwenden Sie keinen Windows-Gerätenamen wie `con`. Claude Code ignoriert jeden anderen Wert und verwendet den abgeleiteten Namen.
* **Setzen Sie es in der Shell-Umgebung, die `claude` startet**: Claude Code liest es beim Start einmal aus dieser Umgebung, daher kann ein `env`-Block in einer Einstellungsdatei es nicht setzen.

Nachdem Sie ein Projektverzeichnis eines Konfigurationsverzeichnisses benannt haben, starten Sie weiterhin mit diesem Namen. Wenn Sie Claude Code mit demselben `CLAUDE_CONFIG_DIR` starten, aber ohne `CLAUDE_CODE_PROJECT_DIR_NAME`, liest und schreibt es das abgeleitete Verzeichnis erneut. Die unter Ihrem Namen gespeicherten Sitzungen bleiben auf der Festplatte: Drücken Sie `Ctrl+A` in der [Sitzungsauswahl](#use-the-session-picker), um Sitzungen aus jedem Projektverzeichnis unter diesem Konfigurationsverzeichnis aufzulisten, einschließlich des angehefteten, und unabhängig davon, wie Sie starten, findet [`claude --resume <session-id>`](#resume-a-session) eine Sitzung, die unter beiden Namen gespeichert ist.

<h2 id="see-also">
  Siehe auch
</h2>

Diese Seiten behandeln verwandte Sitzungs- und Parallelisierungsmechaniken:

* [Worktrees](/docs/de/worktrees): Führen Sie isolierte parallele Sitzungen auf separaten Branches aus
* [Checkpointing](/docs/de/checkpointing): Spulen Sie Code und Gespräch zu einem früheren Punkt zurück
* [Kontextfenster](/docs/de/context-window): Was füllt den Kontext und was überlebt die Komprimierung
* [Nicht-interaktiver Modus](/docs/de/headless): Sitzungsverhalten unter `claude -p`
