> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Parallele Sitzungen mit Worktrees ausführen

> Isolieren Sie parallele Claude Code-Sitzungen in separaten Git-Worktrees, damit Änderungen nicht kollidieren. Behandelt das Flag `--worktree`, Subagent-Isolation, `.worktreeinclude`, Bereinigung und Non-Git-VCS-Hooks.

Ein [Git-Worktree](https://git-scm.com/docs/git-worktree) ist ein separates Arbeitsverzeichnis mit eigenen Dateien und Branch, das die gleiche Repository-Historie und Remote wie Ihr Haupt-Checkout teilt. Das Ausführen jeder Claude Code-Sitzung in ihrem eigenen Worktree bedeutet, dass Änderungen in einer Sitzung niemals Dateien in einer anderen berühren, sodass Claude in einem Terminal ein Feature entwickeln kann, während Sie in einem zweiten einen Bug beheben.

Diese Seite behandelt die Worktree-Isolation in der CLI. Alles unten setzt ein Git-Repository voraus. Für andere Versionskontrollsysteme siehe [Non-Git-Versionskontrolle](#non-git-version-control). Die [Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions) erstellt für jede neue Sitzung automatisch einen Worktree.

Worktrees sind eine von mehreren Möglichkeiten, Claude parallel auszuführen. Sie isolieren Datei-Änderungen, während [Subagents](/docs/de/sub-agents) und [Agent-Teams](/docs/de/agent-teams) die Arbeit selbst koordinieren. Siehe [Agenten parallel ausführen](/docs/de/agents), um die Ansätze zu vergleichen, oder springen Sie direkt zu [Subagents mit Worktrees isolieren](#isolate-subagents-with-worktrees), um Worktrees und Subagents zusammen zu verwenden.

<h2 id="start-claude-in-a-worktree">
  Starten Sie Claude in einem Worktree
</h2>

Übergeben Sie `--worktree` oder `-w`, um einen isolierten Worktree zu erstellen und Claude darin zu starten. Standardmäßig wird der Worktree unter `.claude/worktrees/<value>/` in Ihrem Repository-Root erstellt, auf einem neuen Branch namens `worktree-<value>`:

```bash theme={null}
claude --worktree feature-auth
```

Um Worktrees an anderer Stelle zu platzieren, konfigurieren Sie einen [`WorktreeCreate`-Hook](#non-git-version-control). Führen Sie den Befehl erneut mit einem anderen Namen in einem anderen Terminal aus, um eine zweite isolierte Sitzung zu starten:

```bash theme={null}
claude --worktree bugfix-123
```

Wenn Sie den Namen weglassen, generiert Claude einen Namen wie `bright-running-fox`:

```bash theme={null}
claude --worktree
```

Sie können Claude auch während einer Sitzung bitten, „in einem Worktree zu arbeiten", und es wird einen mit dem [`EnterWorktree`](/docs/de/tools-reference)-Tool erstellen. Sobald Sie sich in einem Worktree befinden, kann Claude direkt zu einem anderen unter `.claude/worktrees/` wechseln, indem er `EnterWorktree` mit dem Zielpfad aufruft. Der vorherige Worktree bleibt unverändert auf der Festplatte.

Das Betreten eines Pfads außerhalb des Verzeichnisses `.claude/worktrees/` des Repositories erfordert zunächst Ihre Genehmigung, da es das Arbeitsverzeichnis der Sitzung, den Schreibzugriff und die Projektkonfiguration wie `CLAUDE.md` und Einstellungen an diesen Ort verschiebt. Eine [`EnterWorktree`-Berechtigung](/docs/de/permissions) oder die Wahl von „nicht mehr fragen" unterdrückt diese Aufforderung nicht; nur der `bypassPermissions`-Modus überspringt sie. Vor v2.1.206 konnte Claude jeden vorhandenen Worktree-Pfad ohne Nachfrage betreten.

Ab v2.1.198 werden beim Betreten oder Verlassen eines Worktrees auch das Sitzungstranskript in das Projektverzeichnis dieses Verzeichnisses verschoben, auf die gleiche Weise wie [`/cd`](/docs/de/commands), sodass `/desktop` und `--resume` die Sitzung danach dort finden. Worktrees, die von einem [`WorktreeCreate`-Hook](#non-git-version-control) erstellt wurden, sind ausgeschlossen und behalten das Transkript im Startverzeichnis.

Worktrees funktionieren mit aktiviertem [Sandboxing](/docs/de/sandboxing#filesystem-isolation): Die Sandbox erlaubt Schreibvorgänge in das gemeinsame `.git`-Verzeichnis des Haupt-Repositories, sodass Befehle wie `git commit` Refs und den Index von innerhalb eines verknüpften Worktrees aktualisieren können.

Bevor Sie `--worktree` interaktiv in einem Verzeichnis zum ersten Mal verwenden, akzeptieren Sie den Dialog zum Vertrauen des Arbeitsbereichs, indem Sie `claude` einmal in diesem Verzeichnis ausführen. Wenn das Vertrauen noch nicht akzeptiert wurde, beendet sich `--worktree` mit einem Fehler und fordert Sie auf, zuerst `claude` im Verzeichnis auszuführen. Nicht-interaktive Ausführungen mit `-p` überspringen die [Vertrauensprüfung](/docs/de/security), sodass `claude -p --worktree` ohne diese fortfährt.

Wenn Claude Code beim Start nicht in das Worktree-Verzeichnis wechseln kann, beispielsweise weil ein [`WorktreeCreate`-Hook](/docs/de/hooks#worktreecreate) etwas anderes als das erstellte Verzeichnis ausgegeben hat oder weil das Verzeichnis nach der Einrichtung gelöscht wurde, gibt Claude Code einen Fehler aus, der den Pfad benennt, und beendet sich mit Code 1. Vor v2.1.205 stürzte dies die Sitzung ab, und mit `-p` stagnierte es etwa 30 Sekunden lang, bevor es mit Code 0 beendet wurde.

Plugins, die im [Projektbereich](/docs/de/plugins-reference#plugin-installation-scopes) aus dem Haupt-Checkout installiert sind, werden auch in Worktrees desselben Repositories geladen, sodass Sie sie nicht pro Worktree neu installieren müssen. Dies gilt, ob Sie den Worktree mit `--worktree` oder mit `git worktree add` erstellen. Erfordert Claude Code v2.1.200 oder später.

<Tip>
  Fügen Sie `.claude/worktrees/` zu Ihrer `.gitignore` hinzu, damit Worktree-Inhalte nicht als nicht verfolgte Dateien in Ihrem Haupt-Checkout angezeigt werden.
</Tip>

<h3 id="choose-the-base-branch">
  Wählen Sie den Basis-Branch
</h3>

Worktrees verzweigen sich vom Standard-Branch Ihres Repositories, `origin/HEAD`, sodass sie von einem sauberen Tree starten, der dem Remote entspricht. Wenn in den letzten 24 Stunden nichts das Repository abgerufen hat, aktualisiert Claude Code `origin/HEAD` mit einem Abruf des Standard-Branches, begrenzt auf fünf Sekunden, und verwendet die lokal zwischengespeicherte Ref, wenn der Abruf fehlschlägt. Wenn kein Remote konfiguriert ist oder `origin/HEAD` nicht lokal zwischengespeichert ist und nicht abgerufen werden kann, fällt der Worktree auf Ihren aktuellen lokalen `HEAD` zurück.

Die Aktualisierung erfordert Claude Code v2.1.208 oder später; davor verwendete ein neuer Worktree, was immer `origin/HEAD` bereits lokal zwischengespeichert war.

Um immer vom lokalen `HEAD` zu verzweigen, setzen Sie `worktree.baseRef` auf `"head"` in [Einstellungen](/docs/de/settings#worktree-settings). Das Setzen von `baseRef` auf `"head"` führt dazu, dass neue Worktrees Ihre nicht gepushten Commits und den Feature-Branch-Status tragen, was nützlich ist, wenn Sie Subagents isolieren, die an laufenden Arbeiten arbeiten müssen. Wenn die Sitzung in einem verknüpften Worktree ausgeführt wird, wird `"head"` zu diesem Worktrees `HEAD` aufgelöst, nicht zum `HEAD` des Haupt-Checkouts. Die Einstellung akzeptiert nur `"fresh"` oder `"head"`, nicht beliebige Git-Refs:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Um von einem bestimmten Pull Request zu verzweigen, übergeben Sie die PR-Nummer mit `#` vorangestellt oder eine vollständige GitHub-Pull-Request-URL. Claude Code ruft `pull/<number>/head` von `origin` ab und erstellt den Worktree unter `.claude/worktrees/pr-<number>`:

```bash theme={null}
claude --worktree "#1234"
```

Für vollständige Kontrolle über die Erstellung von Worktrees konfigurieren Sie einen [`WorktreeCreate`-Hook](/docs/de/hooks#worktreecreate), der die Standard-`git worktree`-Logik vollständig ersetzt.

<h3 id="reuse-a-worktree-name">
  Worktree-Namen wiederverwenden
</h3>

Das Wiederverwenden eines Worktree-Namens, dessen Verzeichnis bereits vorhanden ist, setzt diesen Worktree fort.

Ein fortgesetzter Worktree wird auf die [aktuelle Basis](#choose-the-base-branch) zurückgesetzt, anstatt bei seinem alten Tip fortzufahren, wenn alle folgenden Bedingungen erfüllt sind:

* Es hat keine nicht committeten Änderungen oder nicht verfolgten Dateien.
* Es befindet sich immer noch auf dem Branch, den Claude Code für ihn erstellt hat.
* Es hat nie committed, oder sein Pull Request wurde zusammengeführt und sein Remote-Branch gelöscht.

Vor v2.1.208 setzte ein wiederverwendeter Name immer den alten Worktree bei seinem alten Tip fort.

<h2 id="copy-gitignored-files-into-worktrees">
  Kopieren Sie gitignorierte Dateien in Worktrees
</h2>

Ein Worktree ist ein frischer Checkout, daher sind nicht verfolgte Dateien wie `.env` oder `.env.local` aus Ihrem Haupt-Repository nicht vorhanden. Um sie automatisch zu kopieren, wenn Claude einen Worktree erstellt, fügen Sie eine `.worktreeinclude`-Datei zu Ihrem Projekt-Root hinzu.

Die Datei verwendet `.gitignore`-Syntax. Nur Dateien, die einem Muster entsprechen und auch gitignoriert sind, werden kopiert, sodass verfolgte Dateien niemals dupliziert werden.

Diese `.worktreeinclude` kopiert zwei Env-Dateien und eine Secrets-Konfiguration in jeden neuen Worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Dies gilt für Worktrees, die mit `--worktree` erstellt werden, [Subagent-Worktrees](#isolate-subagents-with-worktrees) und parallele Sitzungen in der [Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions).

<h2 id="isolate-subagents-with-worktrees">
  Isolieren Sie Subagents mit Worktrees
</h2>

Subagents können in ihren eigenen Worktrees ausgeführt werden, sodass parallele Änderungen nicht kollidieren. Bitten Sie Claude, „Worktrees für Ihre Agenten zu verwenden", oder setzen Sie es dauerhaft auf einem [benutzerdefinierten Subagent](/docs/de/sub-agents#supported-frontmatter-fields), indem Sie `isolation: worktree` zum Frontmatter hinzufügen. Jeder Subagent erhält einen temporären Worktree, der automatisch entfernt wird, wenn der Subagent ohne Änderungen beendet wird.

Subagent-Worktrees verwenden denselben [Basisbranch](#choose-the-base-branch) wie `--worktree`, sodass sie von dem Standard-Branch Ihres Repositorys verzweigen, es sei denn, `worktree.baseRef` ist auf `"head"` gesetzt.

<h2 id="clean-up-worktrees">
  Worktrees bereinigen
</h2>

Wenn Sie eine Worktree-Sitzung beenden, hängt die Bereinigung davon ab, ob Sie Änderungen vorgenommen haben:

* **Keine nicht committeten Änderungen, keine nicht verfolgten Dateien und keine neuen Commits**: Der Worktree und sein Branch werden automatisch entfernt. Wenn die Sitzung einen [Namen](/docs/de/sessions#name-your-sessions) hat, fordert Claude Sie stattdessen auf, damit Sie den Worktree später behalten können
* **Nicht committete Änderungen, nicht verfolgte Dateien oder neue Commits vorhanden**: Claude fordert Sie auf, den Worktree zu behalten oder zu entfernen. Das Behalten bewahrt das Verzeichnis und den Branch, sodass Sie später zurückkehren können. Das Entfernen löscht das Worktree-Verzeichnis und seinen Branch und verwirft alle nicht committeten Änderungen, nicht verfolgten Dateien und Commits
* **Nicht-interaktive Ausführungen**: Worktrees, die mit `--worktree` zusammen mit `-p` erstellt werden, werden nicht automatisch bereinigt, da es keine Exit-Eingabeaufforderung gibt. Entfernen Sie sie mit `git worktree remove`

Worktrees, die Claude für Subagenten und [Hintergrundsitzungen](/docs/de/agent-view#how-file-edits-are-isolated) erstellt hat, werden automatisch entfernt, sobald sie älter als Ihre [`cleanupPeriodDays`](/docs/de/settings#available-settings)-Einstellung sind, sofern sie keine nicht committeten Änderungen, keine nicht verfolgten Dateien und keine nicht gepushten Commits haben. Worktrees, die Sie mit `--worktree` erstellen, werden niemals durch diese Bereinigung entfernt.

Während ein Agent ausgeführt wird, führt Claude `git worktree lock` auf seinem Worktree aus, damit eine gleichzeitige Bereinigung ihn nicht entfernen kann. Die Sperre wird freigegeben, wenn der Agent beendet wird. Um einen Worktree zu bereinigen, den die Bereinigung beibehält, führen Sie `git worktree remove` aus und fügen Sie `--force` hinzu, wenn der Worktree nicht committete Änderungen oder nicht verfolgte Dateien hat.

Unter Windows entfernt Claude Code vor dem Entfernen eines Worktrees alle NTFS-Junctions oder Verzeichnis-Symlinks in beliebiger Tiefe darin als Link-Eintrag, sodass das Entfernen des Worktrees nicht die Dateien löscht, auf die ein Link verweist. Vor v2.1.205 entfernte Claude Code nur Links auf oberster Ebene als Link-Einträge, und das Entfernen eines Worktrees mit einer Junction in einem Unterverzeichnis könnte den Inhalt des Verzeichnisses löschen, auf das der Link außerhalb des Worktrees verweist.

<h2 id="manage-worktrees-manually">
  Worktrees manuell verwalten
</h2>

Für vollständige Kontrolle über den Worktree-Speicherort und die Branch-Konfiguration erstellen Sie Worktrees direkt mit Git. Dies ist nützlich, wenn Sie einen bestimmten vorhandenen Branch auschecken oder den Worktree außerhalb des Repositories platzieren müssen.

Erstellen Sie einen Worktree auf einem neuen Branch:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Erstellen Sie einen Worktree aus einem vorhandenen Branch:

```bash theme={null}
git worktree add ../project-bugfix bugfix-123
```

Starten Sie Claude im Worktree:

```bash theme={null}
cd ../project-feature-a && claude
```

Listen Sie Ihre Worktrees auf:

```bash theme={null}
git worktree list
```

Entfernen Sie einen, wenn Sie damit fertig sind:

```bash theme={null}
git worktree remove ../project-feature-a
```

Siehe die [Git-Worktree-Dokumentation](https://git-scm.com/docs/git-worktree) für die vollständige Befehlsreferenz. Denken Sie daran, Ihre Entwicklungsumgebung in jedem neuen Worktree zu initialisieren: Installieren Sie Abhängigkeiten, richten Sie virtuelle Umgebungen ein oder führen Sie aus, was die Einrichtung Ihres Projekts erfordert.

<h2 id="non-git-version-control">
  Non-Git-Versionskontrolle
</h2>

Die Worktree-Isolation verwendet standardmäßig Git. Für SVN, Perforce, Mercurial oder andere Systeme konfigurieren Sie [`WorktreeCreate`- und `WorktreeRemove`-Hooks](/docs/de/hooks#worktreecreate), um benutzerdefinierte Erstellungs- und Bereinigungslogik bereitzustellen. Da der Hook das Standard-Git-Verhalten ersetzt, wird [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) nicht verarbeitet, wenn Sie `--worktree` verwenden. Kopieren Sie stattdessen alle lokalen Konfigurationsdateien in Ihr Hook-Skript.

Dieser `WorktreeCreate`-Hook liest den Worktree-Namen aus stdin, checkt eine frische SVN-Arbeitskopie aus und gibt den Verzeichnispfad aus, damit Claude Code ihn als Arbeitsverzeichnis der Sitzung verwenden kann:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Kombinieren Sie es mit einem `WorktreeRemove`-Hook, um die Bereinigung durchzuführen, wenn die Sitzung endet. Siehe die [Hooks-Referenz](/docs/de/hooks#worktreecreate) für das Eingabeschema und ein Entfernungsbeispiel.

<h2 id="see-also">
  Siehe auch
</h2>

Worktrees handhaben die Datei-Isolation. Die verwandten Seiten unten behandeln die Delegierung von Arbeit in diese isolierten Checkouts und das Wechseln zwischen den Sitzungen, die Sie erstellen:

* [Subagents](/docs/de/sub-agents): Delegieren Sie Arbeit an isolierte Agenten innerhalb einer Sitzung
* [Agent-Teams](/docs/de/agent-teams): Koordinieren Sie mehrere Claude-Sitzungen automatisch
* [Sitzungen verwalten](/docs/de/sessions): Benennen, fortsetzen und wechseln Sie zwischen Gesprächen
* [Desktop-Parallelsitzungen](/docs/de/desktop#work-in-parallel-with-sessions): Worktree-gestützte Sitzungen in der Desktop-App
