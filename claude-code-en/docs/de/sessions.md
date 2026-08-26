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

| Befehl                      | Was er tut                                                         |
| :-------------------------- | :----------------------------------------------------------------- |
| `claude --continue`         | Setzt die neueste Sitzung im aktuellen Verzeichnis fort            |
| `claude --resume`           | Öffnet die [Sitzungsauswahl](#use-the-session-picker)              |
| `claude --resume <name>`    | Setzt die benannte Sitzung direkt fort                             |
| `claude --from-pr <number>` | Setzt die mit diesem Pull Request verknüpfte Sitzung fort          |
| `/resume`                   | Wechselt zu einem anderen Gespräch innerhalb einer aktiven Sitzung |

Sitzungen, die mit [`claude -p`](/docs/de/headless) oder dem [Agent SDK](/docs/de/agent-sdk/overview) erstellt wurden, werden nicht in der Sitzungsauswahl angezeigt, aber Sie können eine trotzdem fortsetzen, indem Sie ihre Sitzungs-ID an `claude --resume <session-id>` übergeben. Führen Sie dies aus dem Verzeichnis aus, in dem die Sitzung gestartet wurde: Die Sitzungs-ID-Suche ist auf das aktuelle Projektverzeichnis und seine Git Worktrees beschränkt, daher meldet eine anderswo erstellte Sitzung `No conversation found with session ID: <session-id>`.

<h3 id="where-the-session-picker-looks">
  Wo die Sitzungsauswahl sucht
</h3>

Sitzungen werden pro Projektverzeichnis gespeichert. Standardmäßig zeigt die Sitzungsauswahl interaktive Sitzungen aus dem aktuellen Worktree sowie Sitzungen, die anderswo gestartet wurden und das aktuelle Verzeichnis mit `/add-dir` hinzugefügt haben. Verwenden Sie `Ctrl+W`, um auf alle Worktrees des Repositorys zu erweitern, oder `Ctrl+A`, um auf jedes Projekt auf dieser Maschine zu erweitern.

Ab v2.1.169 verschiebt das Verschieben einer Sitzung mit [`/cd`](/docs/de/commands) diese in den Projektspeicher des neuen Verzeichnisses, sodass sie danach in der Auswahl dieses Verzeichnisses angezeigt wird. Ab v2.1.196 bleibt eine verschobene Sitzung aus der Auswahl des alten Verzeichnisses ausgeschlossen, auch nach einem Absturz oder erzwungenen Beenden. In früheren Versionen konnte sie auch nach einem nicht sauberen Beenden in der Liste des alten Verzeichnisses erneut angezeigt werden, wenn der alte Pfad Sonderzeichen wie Unterstriche enthielt.

Das Auswählen einer Sitzung aus einem anderen Worktree desselben Repositorys setzt sie an Ort und Stelle fort. Das Auswählen einer Sitzung aus einem nicht verwandten Projekt kopiert stattdessen einen `cd`- und Resume-Befehl in Ihre Zwischenablage.

Das Fortsetzen nach Name wird über das aktuelle Repository und seine Worktrees hinweg aufgelöst. Beide Formen suchen nach einer genauen Übereinstimmung und setzen sie direkt fort, auch wenn sie sich in einem anderen Worktree befindet:

| Befehl                   | Genaue Übereinstimmung | Mehrdeutiger Name                                                                             |
| :----------------------- | :--------------------- | :-------------------------------------------------------------------------------------------- |
| `claude --resume <name>` | Setzt direkt fort      | Öffnet die Sitzungsauswahl mit dem Namen als Suchbegriff vorausgefüllt                        |
| `/resume <name>`         | Setzt direkt fort      | Meldet einen Fehler; führen Sie `/resume` ohne Argument aus, um die Sitzungsauswahl zu öffnen |

<h2 id="name-your-sessions">
  Benennen Sie Ihre Sitzungen
</h2>

Geben Sie Sitzungen aussagekräftige Namen, damit sie in der Sitzungsauswahl auffindbar und nach Name wiederaufnehmbar sind. Dies ist am wichtigsten, wenn Sie an mehreren Aufgaben parallel arbeiten.

| Wann                    | So legen Sie den Namen fest                                                                                                                                                                             |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Beim Start              | `claude -n auth-refactor`                                                                                                                                                                               |
| Während einer Sitzung   | `/rename auth-refactor`. Der Name wird auch in der Eingabeaufforderungsleiste angezeigt                                                                                                                 |
| Aus der Sitzungsauswahl | Markieren Sie eine Sitzung und drücken Sie `Ctrl+R`                                                                                                                                                     |
| Bei Plan-Annahme        | Das Akzeptieren eines Plans im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) benennt die Sitzung aus dem Plan-Inhalt, es sei denn, Sie haben bereits einen Namen festgelegt |

Sobald eine Sitzung benannt ist, kehren Sie mit `claude --resume <name>` oder `/resume <name>` zu ihr zurück. Siehe [Sitzung fortsetzen](#resume-a-session), um zu erfahren, wie die Namensauflösung über Worktrees hinweg funktioniert.

Interaktive Sitzungen, die Sie nie benennen, erhalten beim Start automatisch einen Standard-Anzeigenamen. Erfordert Claude Code v2.1.196 oder später. Der Standard kombiniert den Namen des Arbeitsverzeichnisses mit einem zweistelligen Suffix, beispielsweise `my-app-3f`, und identifiziert die Sitzung in Auflistungen laufender Sitzungen, wie z. B. [Agent-Ansicht](/docs/de/agent-view) und `claude agents --json` Ausgabe.

Der Standard ist kein Resume-Handle: `claude --resume <name>`, `/resume <name>` und die Sitzungsauswahl stimmen nur mit Namen überein, die Sie festgelegt haben. Das Benennen der Sitzung ersetzt den Standard.

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

Jede Zeile zeigt den Sitzungsnamen, falls festgelegt, andernfalls die Gesprächszusammenfassung oder erste Eingabeaufforderung, zusammen mit der Zeit seit der letzten Aktivität, der Nachrichtenanzahl und dem Git-Branch. Der Projektpfad wird angezeigt, nachdem Sie mit `Ctrl+A` auf alle Projekte erweitert haben.

Verzweigte Sitzungen, die mit `/branch`, `/rewind` oder `--fork-session` erstellt wurden, werden unter ihrer Root-Sitzung gruppiert. Drücken Sie `→`, um eine Gruppe zu erweitern.

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

Die ursprüngliche Sitzung bleibt unverändert und bleibt in der Sitzungsauswahl verfügbar. Die `/branch`-Bestätigung gibt zwei Sitzungs-IDs aus: den neuen Branch, in dem Sie sich jetzt befinden, und das Original. Um zum Original zurückzukehren, übergeben Sie seine ID an `/resume`, verwenden Sie die Sitzungsauswahl oder führen Sie `/resume <original-name>` aus. Berechtigungen, die Sie mit „für diese Sitzung zulassen" genehmigt haben, werden nicht auf den neuen Branch übertragen. Wenn Sie dieselbe Sitzung in zwei Terminals ohne Verzweigung fortsetzen, werden Nachrichten von beiden in ein Transkript verschachtelt.

Für Checkpoint-basiertes Zurückspulen innerhalb einer einzelnen Sitzung siehe [Checkpointing](/docs/de/checkpointing).

<h2 id="manage-context-within-a-session">
  Verwalten Sie den Kontext innerhalb einer Sitzung
</h2>

Diese Befehle steuern, was sich im Kontextfenster befindet, ohne die Sitzung zu verlassen:

* **`/clear`**: Beginnen Sie mit einem leeren Kontext von vorne. Das vorherige Gespräch wird gespeichert und kann mit `/resume` wiederaufgenommen werden, oder, im selben Claude Code-Prozess, aus [dem Eintrag der vorherigen Sitzung im Rewind-Menü](/docs/de/checkpointing#rewind-past-a-cleared-conversation)
* **`/compact [instructions]`**: Ersetzen Sie den Verlauf durch eine Zusammenfassung, optional fokussiert auf das, was Sie angeben
* **`/context`**: Zeigen Sie an, was derzeit Kontext verbraucht

Wie die Komprimierung mit CLAUDE.md, Skills und Regeln interagiert, finden Sie im [Kontextfenster-Leitfaden](/docs/de/context-window). Strategien, wann Sie löschen oder komprimieren sollten, finden Sie unter [Best Practices](/docs/de/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Exportieren und lokalisieren Sie Sitzungsdaten
</h2>

Führen Sie `/export` aus, um das aktuelle Gespräch in Ihre Zwischenablage zu kopieren oder als Nur-Text-Datei zu speichern, wobei Nachrichten und Tool-Ausgaben als lesbarer Text gerendert werden. Übergeben Sie einen Dateinamen, um direkt in diese Datei zu schreiben.

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

Standardmäßig werden Transkripte als JSONL unter `~/.claude/projects/<project>/<session-id>.jsonl` gespeichert, wobei `<project>` Ihr Arbeitsverzeichnispfad mit nicht-alphanumerischen Zeichen ist, die durch `-` ersetzt wurden. Jede Zeile ist ein JSON-Objekt für eine Nachricht, Tool-Verwendung oder Metadateneintrag. Das Eintragsformat ist intern für Claude Code und ändert sich zwischen Versionen, daher können Skripte, die diese Dateien direkt analysieren, bei jeder Veröffentlichung unterbrochen werden. Um auf Sitzungsdaten aufzubauen, verwenden Sie stattdessen `/export` oder die [Skript-Schnittstellen](#access-conversations-from-scripts).

Der Speicherort, die Aufbewahrung und das Schreibverhalten sind konfigurierbar:

| Zu                                                                 | Einstellen                                             | Wo                       |
| ------------------------------------------------------------------ | ------------------------------------------------------ | ------------------------ |
| Speicher von `~/.claude` verschieben                               | [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars)                    | Umgebungsvariable        |
| Ändern Sie die 30-Tage-Aufbewahrung                                | [`cleanupPeriodDays`](/docs/de/settings#available-settings) | `settings.json`          |
| Transkriptschreibvorgänge in allen Modi unterdrücken               | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/de/env-vars)      | Umgebungsvariable        |
| Schreibvorgänge für eine nicht-interaktive Ausführung unterdrücken | [`--no-session-persistence`](/docs/de/cli-reference)        | CLI-Flag mit `claude -p` |

<h2 id="see-also">
  Siehe auch
</h2>

Diese Seiten behandeln verwandte Sitzungs- und Parallelisierungsmechaniken:

* [Worktrees](/docs/de/worktrees): Führen Sie isolierte parallele Sitzungen auf separaten Branches aus
* [Checkpointing](/docs/de/checkpointing): Spulen Sie Code und Gespräch zu einem früheren Punkt zurück
* [Kontextfenster](/docs/de/context-window): Was füllt den Kontext und was überlebt die Komprimierung
* [Nicht-interaktiver Modus](/docs/de/headless): Sitzungsverhalten unter `claude -p`
