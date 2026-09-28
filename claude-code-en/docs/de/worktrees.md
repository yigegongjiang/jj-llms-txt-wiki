> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Parallele Sitzungen mit Worktrees ausführen

> Isolieren Sie parallele Claude Code-Sitzungen in separaten Git-Worktrees, damit Änderungen nicht kollidieren. Behandelt das Flag `--worktree`, Subagent-Isolation, `.worktreeinclude`, Bereinigung und Non-Git-VCS-Hooks.

Ein [Git-Worktree](https://git-scm.com/docs/git-worktree) ist ein separates Arbeitsverzeichnis mit eigenen Dateien und Branch, das die gleiche Repository-Historie und Remote wie Ihr Haupt-Checkout teilt. Das Ausführen jeder Claude Code-Sitzung in ihrem eigenen Worktree bedeutet, dass Änderungen in einer Sitzung niemals Dateien in einer anderen berühren, sodass eine Sitzung ein Feature entwickeln kann, während eine zweite einen Bug behebt.

<Note>
  Worktrees erfordern ein Git-Repository; für andere Versionskontrollsysteme [konfigurieren Sie Hooks, um die Git-Logik zu ersetzen](#non-git-version-control). In der [Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions) wählen Sie die Option **worktree** aus, wenn Sie eine Sitzung starten, um ihr ihren eigenen Worktree zu geben.
</Note>

Worktrees sind eine von mehreren Möglichkeiten, Claude parallel auszuführen. Sie isolieren Datei-Änderungen. [Subagents](/docs/de/sub-agents) teilen die Arbeit innerhalb einer Sitzung auf, und [sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging) ermöglicht es Claude, Erkenntnisse zwischen den Sitzungen in Ihren Worktrees weiterzugeben. Siehe [Agenten parallel ausführen](/docs/de/agents), um die Ansätze zu vergleichen, oder springen Sie direkt zu [Subagents mit Worktrees isolieren](#isolate-subagents-with-worktrees), um Worktrees und Subagents zusammen zu verwenden.

Die meisten Sitzungen benötigen nur die ersten zwei Abschnitte: [Starten Sie Claude in einem Worktree](#start-claude-in-a-worktree), dann [bereinigen Sie, wenn Sie beenden](#clean-up-worktrees). Kehren Sie zum Rest der Seite zurück, wenn Sie [eine Sitzung fortsetzen](#resume-a-worktree-session), [ändern, wie Worktrees erstellt werden](#customize-worktree-creation), oder [einen Fehler beheben](#troubleshooting) müssen.

<h2 id="start-claude-in-a-worktree">
  Starten Sie Claude in einem Worktree
</h2>

Übergeben Sie `--worktree` oder `-w` mit einem Namen, um einen isolierten Worktree zu erstellen und Claude darin zu starten. Standardmäßig wird der Worktree unter `.claude/worktrees/<name>/` in Ihrem Repository-Root erstellt, auf einem neuen Branch namens `worktree-<name>`:

```bash theme={null}
claude --worktree feature-auth
```

Führen Sie den Befehl erneut mit einem anderen Namen in einem anderen Terminal aus, um eine zweite isolierte Sitzung zu starten. Wenn Sie den Namen weglassen, generiert Claude einen Namen wie `bright-running-fox`.

Interaktive Ausführungen erfordern [Workspace-Vertrauen](/docs/de/security): Wenn Sie Claude in dem Verzeichnis noch nicht ausgeführt haben, führen Sie `claude` einmal dort aus, um den Vertrauensdialog zu akzeptieren, oder `--worktree` beendet sich mit einem Fehler, der Sie dazu auffordert. Nicht-interaktive Ausführungen mit `-p` überspringen die Vertrauensprüfung, sodass `claude -p --worktree` ohne diese fortfährt.

<Tip>
  Fügen Sie `.claude/worktrees/` zu Ihrer `.gitignore` hinzu, damit Worktree-Inhalte nicht als nicht verfolgte Dateien in Ihrem Haupt-Checkout angezeigt werden.
</Tip>

<h3 id="set-up-the-worktree-environment">
  Richten Sie die Worktree-Umgebung ein
</h3>

Ein Worktree ist ein frischer Checkout, daher initialisieren Sie Ihre Entwicklungsumgebung dort: Bitten Sie Claude, Abhängigkeiten zu installieren, oder führen Sie die Einrichtung Ihres Projekts selbst im Worktree-Verzeichnis unter `.claude/worktrees/` durch. Um gitignorierte Dateien wie `.env` automatisch in jeden neuen Worktree zu tragen, fügen Sie eine [`.worktreeinclude`-Datei](#copy-gitignored-files-into-worktrees) hinzu.

<h3 id="ask-claude-to-create-a-worktree">
  Bitten Sie Claude, einen Worktree zu erstellen
</h3>

Sie können Claude auch während einer Sitzung bitten, „in einem Worktree zu arbeiten", und es erstellt einen mit dem [`EnterWorktree`](/docs/de/tools-reference)-Tool. Sobald Sie sich in einem Worktree befinden, kann Claude direkt zu einem anderen unter `.claude/worktrees/` wechseln, indem er `EnterWorktree` mit dem Zielpfad aufruft; der vorherige Worktree bleibt unverändert auf der Festplatte.

Wenn Claude einen Pfad außerhalb des Verzeichnisses `.claude/worktrees/` des Repositories betritt, fragt Claude Code zunächst nach Ihrer Genehmigung, da der Wechsel das Arbeitsverzeichnis der Sitzung, den Schreibzugriff und die Projektkonfiguration wie `CLAUDE.md` und Einstellungen an diesen Ort verschiebt. Eine `EnterWorktree`-[Berechtigung](/docs/de/permissions) oder die Wahl von „nicht mehr fragen" unterdrückt diese Aufforderung nicht; nur der `bypassPermissions`-Modus überspringt sie. Vor v2.1.206 konnte Claude jeden vorhandenen Worktree-Pfad ohne Nachfrage betreten.

<Note>
  **Hook-Pfade folgen dem Worktree nicht.** Nachdem Claude einen Worktree betritt, behält Claude Code `${CLAUDE_PROJECT_DIR}` in Ihren [Hooks](/docs/de/hooks#reference-scripts-by-path) dort, wo er war, und übergibt den Worktree-Pfad auf andere Weise:

  * **`${CLAUDE_PROJECT_DIR}` bleibt an Ort und Stelle**: Es zeigt immer noch auf das Projekt-Root, wo die Sitzung gestartet wurde, sodass ein Hook-Befehl wie `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` das Skript immer noch im Haupt-Checkout ausführt.
  * **`cwd` folgt Claude**: Das Feld `cwd` in der Hook-[Eingabe-JSON](/docs/de/hooks#common-input-fields) ist das Worktree-Root, und es bewegt sich erneut, wenn Claude `cd` ausführt. Lesen Sie es, wenn ein Hook den Worktree-Pfad benötigt.
</Note>

<h2 id="clean-up-worktrees">
  Bereinigen Sie Worktrees
</h2>

Wenn Sie eine interaktive Worktree-Sitzung beenden, prüft Claude den Worktree auf Arbeit, die durch das Entfernen gelöscht würde: geänderte oder nicht verfolgte Dateien, nicht committete Arbeit in ausgecheckten Submodulen und neue Commits.

* **Der Worktree ist sauber**: Für eine unbenannte Sitzung entfernt Claude den Worktree und seinen Branch automatisch. Eine [benannte](/docs/de/sessions#name-your-sessions) Sitzung fordert Sie zunächst auf, damit Sie den Worktree später behalten können
* **Der Worktree enthält Arbeit**: Claude fordert Sie auf, den Worktree zu behalten oder zu entfernen. Das Behalten bewahrt das Verzeichnis und den Branch, sodass Sie später zurückkehren können. Das Entfernen löscht das Worktree-Verzeichnis und seinen Branch zusammen mit der gesamten Arbeit darin
* **Der Zustand des Worktrees kann nicht überprüft werden**: Wenn Claude Code die Änderungen des Worktrees nicht zählen kann oder seine Submodul-Checkouts nicht überprüfen kann, fordert es Sie auf, anstatt den Worktree automatisch zu entfernen. Die Aufforderung benennt, was nicht überprüft werden konnte

Nicht-interaktive Ausführungen mit `-p` haben keine Exit-Aufforderung, sodass Claude ihre Worktrees nicht bereinigt, und Claude Code hinterlässt die Sperre, die es bei der Erstellung auf jedem genommen hat, bis eine spätere Sitzung [stale-lock sweep](#clean-up-subagent-and-background-session-worktrees) sie freigeben kann. Um einen zu entfernen, führen Sie `git worktree remove` aus; wenn Git sich weigert, weil der Worktree gesperrt ist, führen Sie zuerst `git worktree unlock` darauf aus.

Unter Windows löscht das Entfernen eines Worktrees keine Dateien außerhalb davon. Wenn ein Ordner im Worktree ein Link zu anderswo ist, wie eine NTFS-Junction oder ein Verzeichnis-Symlink, löscht Claude Code nur den Link und behält den Ordner, auf den er verweist. Vor v2.1.205 konnte das Entfernen eines Worktrees mit einem Link in einem Unterverzeichnis den Ordner löschen, auf den er verweist.

<h2 id="resume-a-worktree-session">
  Setzen Sie eine Worktree-Sitzung fort
</h2>

Wenn Sie eine Sitzung fortsetzen, die sich in einem Worktree befand, kehrt Claude Code die Sitzung zu diesem Worktree zurück. Dies gilt für interaktive Fortsetzungen, für `--continue` und `--resume` im [nicht-interaktiven Modus](/docs/de/headless) mit `-p`, und für das Agent SDK. Zurück im Worktree kann Claude ihn immer noch mit dem [`ExitWorktree`](/docs/de/tools-reference)-Tool verlassen.

Bevor Claude Code die Sitzung zu ihrem Worktree zurückbringt, überprüft es, dass der Worktree immer noch ein separater Checkout vom Haupt-Checkout ist, und lehnt es ab, einen Worktree erneut zu betreten, der die Prüfung nicht besteht. Für einen Git-Worktree liest die Prüfung seine Git-Metadaten. Ein Worktree ohne Git-Metadaten, wie einer, den ein [`WorktreeCreate`-Hook](#non-git-version-control) erstellt hat, kann die Prüfung bestehen; die Fälle, die Claude Code immer noch ablehnt, sind unter [Claude Code weigert sich, einen Worktree zu verwenden](#claude-code-refuses-to-use-a-worktree) mit ihren Wiederherstellungen aufgelistet. Für die Meldungen und wie Sie sich von jedem erholen, siehe [Die Sitzung wird außerhalb ihres Worktrees fortgesetzt](#the-session-resumes-outside-its-worktree).

Wo Sie starten und wie Sie fortsetzen, ändern, was Claude Code erneut betritt:

* **Startverzeichnis**: Fortsetzen vom Haupt-Checkout oder einem anderen Verzeichnis des Repositories. Claude Code betritt einen Worktree, den es mit Git unter `.claude/worktrees/` erstellt hat, auch wenn Sie von innen starten. Wenn Sie von innen in einem anderen Worktree starten, betritt Claude Code ihn nur, wenn es ihn von dort aus garantieren kann: ein Worktree, der sein eigenes Repository ist, einer ohne Git-Metadaten, oder ein Start aus einem Unterverzeichnis eines Worktrees, den Sie mit `git worktree add` erstellt haben, lehnt ab, daher starten Sie diese vom Haupt-Checkout.
* **`--fork-session`**: Die abgespaltene Sitzung startet in dem Verzeichnis, von dem aus Sie Claude gestartet haben, und Claude Code hinterlässt den Worktree der ursprünglichen Sitzung unverändert.
* **Gelöschter Worktree**: Wenn das Worktree-Verzeichnis nicht mehr existiert, setzt Claude Code die Sitzung in dem Verzeichnis fort, von dem aus Sie Claude gestartet haben. Es teilt Ihnen mit, dass der Worktree weg ist, und löscht die Worktree-Bindung der Sitzung.

<Note>
  Vor v2.1.212 blieb eine nicht-interaktive Fortsetzung im Startverzeichnis und `ExitWorktree` meldete, dass es keine aktive Worktree-Sitzung zum Beenden gab.
</Note>

Wenn Claude einen Worktree betritt oder verlässt, den Claude Code mit Git erstellt hat, folgt das Transkript: Claude Code zeichnet die Sitzung unter dem neuen Arbeitsverzeichnis der Sitzung auf, auf die gleiche Weise wie [`/cd`](/docs/de/commands), sodass `/desktop` und `--resume` sie dort finden. Das Verlassen verschiebt es auf die gleiche Weise zurück. Ein Worktree, der von einem [`WorktreeCreate`-Hook](#non-git-version-control) erstellt wurde, behält sein Transkript im Startverzeichnis. Erfordert Claude Code v2.1.198 oder später.

<h2 id="how-claude-code-enforces-isolation">
  Wie Claude Code Isolation erzwingt
</h2>

Während eine Sitzung in einem Worktree isoliert ist, blockiert Claude Code die Tool-Aufrufe, die die folgenden Prüfungen definieren. Die gleichen Regeln gelten, ob Sie die Sitzung mit `--worktree` gestartet haben, Claude einen Worktree mit `EnterWorktree` betritt, oder Sie eine Worktree-Sitzung fortsetzen.

Die gleiche Erzwingung gilt für jeden Subagent, den Claude aus der isolierten Sitzung erzeugt. Sie gilt, ob die Sitzung interaktiv ist oder im [Hintergrund](/docs/de/agent-view#how-file-edits-are-isolated) läuft. [Subagents, die in ihrem eigenen Worktree laufen](#isolate-subagents-with-worktrees), tragen die gleichen Prüfungen. Ihre Versionsverlauf ist unter [Schreiben Sie Subagent-Dateien](/docs/de/sub-agents#write-subagent-files).

Claude Code wendet vier Prüfungen an:

* **Datei-Änderungen**: Claude Code blockiert einen `Edit`, `Write` oder `NotebookEdit`, der auf einen Pfad im Haupt-Checkout abzielt.
* **Befehl-Arbeitsverzeichnis**: Claude Code blockiert einen Bash-, PowerShell- oder Monitor-Befehl, dessen Arbeitsverzeichnis zum Haupt-Checkout aufgelöst wird, oder dessen Arbeitsverzeichnis es nicht überprüfen kann, dass es außerhalb bleibt.
* **Git-Umleitungen**: Claude Code blockiert einen Bash- oder Monitor-Befehl, der Git in den Haupt-Checkout umleitet. Die Umleitung kann durch `git -C`, `--git-dir`, eine `GIT_DIR`- oder `GIT_WORK_TREE`-Variable oder ein `cd` in den Haupt-Checkout vor dem Ausführen von Git erfolgen.
* **Befehlsform**: Claude Code blockiert einen Bash- oder Monitor-Befehl, wenn es nicht überprüfen kann, dass jedes Git, das der Befehl ausführt, im Worktree bleibt. Das geschieht beispielsweise, wenn der Befehlsname zur Laufzeit berechnet wird, wenn die Syntax nicht analysiert werden kann, oder wenn eine Erweiterung wie `${!name}` oder `${ command; }` einen Befehl ausführen könnte, den der Text nicht ausdrücklich angibt. Claude Code teilt Claude mit, wie der abgelehnte Befehl umgeschrieben werden kann, z. B. durch Aufteilen in einfache, separate Befehle. Sie können diese Prüfung nicht ausschalten.

Die Prüfungen gelten für das Repository, von dem aus Sie Claude Code gestartet haben. Sie decken auch den Haupt-Checkout ab, von dem ein verknüpfter Worktree verknüpft ist. Für PowerShell-Befehle wendet Claude Code nur die Arbeitsverzeichnis-Prüfung an.

Claude sieht jede Ablehnung als einen Tool-Fehler, der den Worktree benennt und sagt, wie man fortfährt. Für einen abgelehnten Befehl siehe [was die Ablehnungsmeldung bedeutet und wie man sie löscht](/docs/de/errors#command-blocked-by-the-worktree-isolation-checks).

<h2 id="isolate-subagents-with-worktrees">
  Isolieren Sie Subagents mit Worktrees
</h2>

Subagents können in ihren eigenen Worktrees laufen, sodass parallele Änderungen nicht kollidieren. Bitten Sie Claude, „Worktrees für Ihre Agenten zu verwenden", oder machen Sie die Isolation dauerhaft für einen [benutzerdefinierten Subagent](/docs/de/sub-agents#supported-frontmatter-fields), indem Sie `isolation: worktree` zu seinem Frontmatter hinzufügen.

Dieser Subagent in `.claude/agents/` läuft immer in seinem eigenen Worktree:

```markdown theme={null}
---
name: refactorer
description: Applies mechanical refactors across many files
isolation: worktree
---

Apply the requested refactor across every affected file, then run the tests
and report the results.
```

Jeder Subagent erhält einen temporären Worktree, den Claude Code automatisch entfernt, wenn der Subagent ohne Änderungen beendet wird; ein Worktree mit Änderungen bleibt auf der Festplatte, bis die [periodische Bereinigung unten](#clean-up-subagent-and-background-session-worktrees) ihn entfernen kann, ohne Arbeit zu verlieren.

Subagent-Worktrees verwenden den gleichen [Basis-Branch](#choose-the-base-branch) wie `--worktree`, sodass sie von dem Standard-Branch Ihres Repositories verzweigen, es sei denn, `worktree.baseRef` ist auf `"head"` gesetzt.

<h3 id="clean-up-subagent-and-background-session-worktrees">
  Bereinigen Sie Subagent- und Hintergrund-Sitzungs-Worktrees
</h3>

Claude Code führt eine periodische Bereinigung durch, die Worktrees entfernt, die Claude für Subagents und [Hintergrund-Sitzungen](/docs/de/agent-view#how-file-edits-are-isolated) erstellt hat, sobald sie älter als Ihre [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays)-Einstellung sind, nach den [Aufbewahrungsbereinigungsregeln](/docs/de/claude-directory#cleaned-up-automatically).

Wenn Sie eine `--worktree`-Sitzung [in den Hintergrund](/docs/de/agent-view#send-the-session-to-the-background) verschieben, wird ihr Worktree zu einem Hintergrund-Sitzungs-Worktree, den die Bereinigung entfernen kann. Die Bereinigung hinterlässt einen Worktree in diesen Fällen:

* Der Worktree hält immer noch Arbeit: geänderte oder nicht verfolgte Dateien oder nicht gepushte Commits.
* Ein ausgechecktes Submodul im Worktree hält geänderte oder nicht verfolgte Dateien, oder Claude Code kann die Submodule des Worktrees nicht inspizieren. Diese Überprüfung erfordert Claude Code v2.1.274 oder später.
* Einer der [vier Fälle, die auch die Worktree-Erstellung blockieren](#git-lfs-content-is-missing-from-a-worktree-claude-code-created) trifft zu: Claude Code kann nicht bestimmen, welche Filter-Treiber die Repository-Konfiguration definiert, oder findet dort eine Einstellung, die es nicht ausschalten kann.
* Der Worktree gehört zu einer `--worktree`-Sitzung, die Sie nicht in den Hintergrund verschoben haben, unabhängig von seinem Alter.
* Sie haben den Worktree selbst mit `git worktree add` erstellt, auch wenn Sie dann eine `--worktree <name>`-Sitzung darin ausgeführt und diese Sitzung in den Hintergrund verschoben haben.

Claude Code schreibt einen Marker in die Git-Metadaten jedes Worktrees, den es mit Git erstellt, und die Bereinigung behält jeden Worktree ohne einen, einschließlich eines, den ein [`WorktreeCreate`-Hook](#non-git-version-control) erstellt hat. Vor v2.1.246 prüfte die Bereinigung nicht auf den Marker und konnte einen Worktree entfernen, den Sie selbst erstellt haben, wenn ein alter Hintergrund-Sitzungs-Datensatz darauf verweist.

Während ein Agent läuft, hält Claude Code eine `git worktree lock` auf seinem Worktree, sodass gleichzeitige Bereinigung ihn nicht entfernen kann, und gibt die Sperre frei, wenn der Agent beendet wird. Claude Code hält die gleiche Sperre auf dem Worktree, den es für eine Hintergrund-Sitzung erstellt hat, während die Sitzung läuft, sodass die Bereinigung den Worktree an Ort und Stelle hinterlässt und `git worktree remove` sich weigert, ihn zu entfernen.

Die Bereinigung gibt auch eine Sperre frei, die Claude Code für eine Sitzung gesetzt hat, deren Prozess beendet wurde, sodass eine getötete Hintergrund-Sitzung ihren Worktree nicht dauerhaft gesperrt hinterlässt. Die Bereinigung gibt niemals eine Sperre frei, die Sie selbst mit `git worktree lock` gesetzt haben. Vor v2.1.210 blieb eine Sperre, die von einer getöteten Sitzung hinterlassen wurde, an Ort und Stelle, bis Sie `git worktree unlock` ausgeführt haben.

Um einen Worktree zu bereinigen, den die Bereinigung behält, führen Sie `git worktree remove` aus und fügen Sie `--force` hinzu, wenn der Worktree nicht committete Änderungen oder nicht verfolgte Dateien hat. Wenn Git sich weigert, weil der Worktree gesperrt ist, führen Sie zuerst `git worktree unlock` darauf aus.

<h2 id="customize-worktree-creation">
  Passen Sie die Worktree-Erstellung an
</h2>

Die Standardwerte von Claude Code für die Erstellung von Worktrees decken die meisten Sitzungen ab: Es erstellt sie unter `.claude/worktrees/`, verzweigt sie vom Standard-Branch Ihres Repositories und checkt nur verfolgte Dateien aus. Die Optionen in diesem Abschnitt ändern diese Standardwerte.

<h3 id="choose-the-base-branch">
  Wählen Sie den Basis-Branch
</h3>

Neue Worktrees verzweigen sich vom Standard-Branch des Repositories, sodass die meisten Sitzungen diese Einstellung nicht benötigen. Setzen Sie `worktree.baseRef` in [Einstellungen](/docs/de/settings-reference#worktree), um stattdessen von Ihrer aktuellen Arbeit zu verzweigen. Die Einstellung akzeptiert zwei Werte:

* `"fresh"` (Standard): Verzweigung vom Standard-Branch des Repositories auf dem Remote, normalerweise `main`, sodass der Worktree von einem sauberen Tree startet, der dem Remote entspricht.
* `"head"`: Verzweigung von Ihrem aktuellen lokalen `HEAD`, sodass der Worktree Ihre nicht gepushten Commits und Feature-Branch-Status trägt. Verwenden Sie dies, wenn Sie Subagents isolieren, die an laufenden Arbeiten arbeiten müssen. Innerhalb eines Worktrees wird `"head"` zu diesem Worktrees `HEAD` aufgelöst, nicht zum `HEAD` des Haupt-Checkouts.

Sie können `worktree.baseRef` nicht auf einen Branch-Namen setzen. Um einen Worktree von einem bestimmten vorhandenen Branch zu starten, [erstellen Sie ihn direkt mit Git](#manage-worktrees-manually).

Für eine `"fresh"`-Basis hält Claude Code `origin/HEAD` aktuell: Wenn das Repository in den letzten 24 Stunden nicht abgerufen wurde, ruft es den Standard-Branch ab, begrenzt auf fünf Sekunden, und verwendet die lokal zwischengespeicherte Ref, wenn der Abruf fehlschlägt. Wenn kein Remote konfiguriert ist oder `origin/HEAD` nicht lokal zwischengespeichert ist und nicht abgerufen werden kann, fällt der Worktree auf Ihren aktuellen lokalen `HEAD` zurück. Vor v2.1.208 verwendete ein neuer Worktree, was immer `origin/HEAD` bereits lokal zwischengespeichert war.

Dieses Beispiel macht jeden neuen Worktree von Ihrer aktuellen Arbeit verzweigen:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

<h3 id="branch-from-a-pull-request">
  Verzweigung von einem Pull Request
</h3>

Um von einem bestimmten Pull Request oder Merge Request zu verzweigen, übergeben Sie `--worktree` die Nummer mit `#` vorangestellt, eine GitHub-Pull-Request-URL oder eine GitLab-Merge-Request-URL wie `https://gitlab.com/group/repo/-/merge_requests/123`. Claude Code ruft den Head-Commit dieser Änderung von `origin` ab und erstellt den Worktree unter `.claude/worktrees/pr-<number>`. Zitieren Sie das Argument, damit Ihre Shell `#` nicht als Anfang eines Kommentars behandelt:

```bash theme={null}
claude --worktree "#1234"
```

Claude Code liest nur die Nummer aus der URL. Es ruft immer von Ihrem Repository-`origin`-Remote ab und wählt den Abruf-Pfad nach dem Host von `origin`:

* **github.com**: ruft `pull/<number>/head` ab
* **gitlab.com**: ruft `merge-requests/<number>/head` ab
* **GitHub Enterprise, selbstverwaltetes GitLab oder ein anderer Host**: versucht zuerst `pull/<number>/head`, dann `merge-requests/<number>/head`

Vor v2.1.233 akzeptierte Claude Code nur `#<number>` und GitHub-Style-Pull-Request-URLs für `--worktree` und rief immer `pull/<number>/head` ab.

<h3 id="copy-gitignored-files-into-worktrees">
  Kopieren Sie gitignorierte Dateien in Worktrees
</h3>

Ein Worktree ist ein frischer Checkout, sodass nicht verfolgte Dateien wie `.env` oder `.env.local` aus Ihrem Haupt-Repository nicht vorhanden sind. Um sie automatisch zu kopieren, wenn Claude einen Worktree erstellt, fügen Sie eine `.worktreeinclude`-Datei zu Ihrem Projekt-Root hinzu.

Die Datei verwendet `.gitignore`-Syntax. Nur Dateien, die einem Muster entsprechen und auch gitignoriert sind, werden kopiert, sodass verfolgte Dateien niemals dupliziert werden.

Wenn Sie ein Muster schreiben, das mit `**/` beginnt, und die Dateien, die Sie möchten, befinden sich in einem Verzeichnis, das als Ganzes gitignoriert ist, kopiert Claude Code sie nur, wenn dieses Verzeichnis selbst dem Muster entspricht, oder wenn der erste Name nach `**/` einer der Namen im Pfad des Verzeichnisses ist. Zum Beispiel, wenn Sie `**/.claude/skills/*.md` schreiben, ist dieser erste Name `.claude`, sodass Claude Code die übereinstimmenden Dateien aus einem ignorierten `.claude/`-Verzeichnis kopiert. Um Dateien aus einem ignorierten Verzeichnis zu kopieren, das ein `**/`-Muster nicht erreicht, benennen Sie das Verzeichnis im Muster: schreiben Sie `vendor/**/config.json` statt `**/config.json`. Vor v2.1.239 kopierte Claude Code Dateien aus einem vollständig ignorierten Verzeichnis für ein `**/`-Muster nur, wenn das Verzeichnis selbst dem Muster entsprach.

Diese `.worktreeinclude` kopiert zwei Env-Dateien und eine Secrets-Konfiguration in jeden neuen Worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Dies gilt für jeden Worktree, den Claude Code mit Git erstellt: `--worktree`-Worktrees, [Subagent-Worktrees](#isolate-subagents-with-worktrees) und parallele Sitzungen in der [Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions). Mit einem [`WorktreeCreate`-Hook](#non-git-version-control) kopieren Sie die Dateien im Hook-Skript.

<h3 id="reuse-a-worktree-name">
  Verwenden Sie einen Worktree-Namen erneut
</h3>

Das Übergeben von `--worktree` eines Namens, dessen Verzeichnis bereits existiert, öffnet diesen vorhandenen Worktree, anstatt einen neuen zu erstellen.

Mit der Standard-`"fresh"`-[Basis](#choose-the-base-branch) wird ein erneut geöffneter Worktree auf den Standard-Branch des Repositories zurückgesetzt, anstatt bei seinem alten Tip fortzufahren, wenn alle folgenden Bedingungen erfüllt sind:

* Es hat keine nicht committeten Änderungen oder nicht verfolgten Dateien.
* Es befindet sich immer noch auf dem Branch, den Claude Code für ihn erstellt hat.
* Es hat keine Commits von sich selbst, oder sein Pull Request oder Merge Request wurde zusammengeführt und sein Remote-Branch gelöscht.

Claude Code erkennt den zusammengeführten Fall allein aus dem Git-Status: Der Remote-Branch, zu dem der Worktree gepusht hat, existiert nicht mehr, und jeder Commit im Worktree befindet sich bereits auf dem Standard-Branch.

In jedem anderen Fall öffnet Claude Code den Worktree bei seinem alten Tip erneut:

* Der Worktree erfüllt eine der Bedingungen nicht.
* Claude Code kann den Status des Worktrees nicht überprüfen.
* `worktree.baseRef` ist `"head"`.
* Der Name ist eine Pull-Request- oder Merge-Request-Referenz.

Vor v2.1.208 öffnete Claude Code beim Wiederverwenden eines Namens immer den alten Worktree bei seinem alten Tip erneut.

<h3 id="replace-worktree-creation-with-a-hook">
  Ersetzen Sie die Worktree-Erstellung durch einen Hook
</h3>

Konfigurieren Sie einen [`WorktreeCreate`-Hook](/docs/de/hooks#worktreecreate), um die Standard-`git worktree`-Logik vollständig zu ersetzen, einschließlich der Platzierung von Worktrees anderswo als `.claude/worktrees/`. Ein vollständiges Beispiel finden Sie unter [Non-Git-Versionskontrolle](#non-git-version-control).

<h2 id="what-worktrees-share-with-the-main-checkout">
  Was Worktrees mit dem Haupt-Checkout teilen
</h2>

Ein Worktree erhält seine eigenen Dateien und Branch, aber es teilt das Folgende mit dem Haupt-Checkout:

* **Das `.git`-Verzeichnis des Repositories**: Git-Befehle in einem Worktree schreiben in das gemeinsame `.git`-Verzeichnis des Haupt-Repositories, und [Sandboxing](/docs/de/sandboxing#filesystem-isolation) erlaubt diese Schreibvorgänge, sodass Befehle wie `git commit` von innen in einem Worktree mit aktivierter Sandbox funktionieren.
* **Plugins**: Plugins, die im [Projekt-Bereich](/docs/de/plugins/loading#find-where-a-plugin-is-enabled) aus dem Haupt-Checkout installiert sind, werden auch in Worktrees desselben Repositories geladen, sodass Sie sie nicht pro Worktree neu installieren müssen. Erfordert Claude Code v2.1.200 oder später.
* **Genehmigungen**: Das Wählen von „Ja, und nicht mehr fragen" für einen Bash-Befehl in einer Worktree-Sitzung speichert die Regel in der `.claude/settings.local.json` des Haupt-Checkouts, sodass sie im Haupt-Checkout und in jedem anderen Worktree des Repositories gilt und das Entfernen des Worktrees überlebt. Unter Windows und in den anderen Fällen, in denen Claude Code [das Repository-Root nicht verwendet](/docs/de/settings#where-claude-code-looks-for-each-file), bleibt die Regel bei diesem Worktree. Vor v2.1.211 wurde eine Genehmigung, die in einem Worktree gewährt wurde, in diesem Worktree gespeichert, galt nicht anderswo und ging verloren, wenn der Worktree entfernt wurde. Siehe [wo Genehmigungen gespeichert werden](/docs/de/permissions#permission-system).
* **Nicht nachverfölgte Skills, Agents und Befehle**: Wenn der Worktree-Checkout kein `.claude/skills`-Verzeichnis in seinem Root hat, zum Beispiel weil Ihr `.claude/skills` gitignoriert ist, lädt Claude Code die [Projekt-Skills](/docs/de/skills#where-skills-live) des Haupt-Checkouts in der Worktree-Sitzung. In einem Worktree mit seinem eigenen `.claude/skills`-Verzeichnis wird nur diese Kopie geladen.

  Dieselbe Durchlesung gilt für `.claude/agents` und `.claude/commands`. Für Skills erfordert die Durchlesung Claude Code v2.1.277 oder später.

All diese gelten, ob Sie den Worktree mit `--worktree`, mit `git worktree add` oder über die [Desktop-App](/docs/de/desktop#work-in-parallel-with-sessions) erstellen.

<h2 id="manage-worktrees-manually">
  Verwalten Sie Worktrees manuell
</h2>

Erstellen Sie Worktrees direkt mit Git, wenn Sie einen bestimmten vorhandenen Branch auschecken oder den Worktree außerhalb des Repositories platzieren müssen.

Erstellen Sie einen Worktree auf einem neuen Branch:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Erstellen Sie einen Worktree aus einem vorhandenen Branch, ersetzen Sie `fix-issue-456` durch einen Branch, der bereits in Ihrem Repository existiert:

```bash theme={null}
git worktree add ../project-bugfix fix-issue-456
```

Starten Sie Claude im Worktree:

```bash theme={null}
cd ../project-feature-a
claude
```

Listen Sie Ihre Worktrees auf:

```bash theme={null}
git worktree list
```

Entfernen Sie einen, wenn Sie damit fertig sind:

```bash theme={null}
git worktree remove ../project-feature-a
```

Siehe die [Git-Worktree-Dokumentation](https://git-scm.com/docs/git-worktree) für die vollständige Befehlsreferenz.

<h2 id="non-git-version-control">
  Non-Git-Versionskontrolle
</h2>

Die Worktree-Isolation verwendet standardmäßig Git. Für SVN, Perforce, Mercurial oder andere Systeme konfigurieren Sie [`WorktreeCreate`- und `WorktreeRemove`-Hooks](/docs/de/hooks#worktreecreate), um benutzerdefinierte Erstellungs- und Bereinigungslogik bereitzustellen. Da der Hook das Standard-Git-Verhalten ersetzt, wird [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) nicht verarbeitet, wenn Sie `--worktree` verwenden. Kopieren Sie stattdessen alle lokalen Konfigurationsdateien in Ihr Hook-Skript.

Dieser `WorktreeCreate`-Hook liest den Worktree-Namen aus der JSON auf stdin mit `jq`, checkt eine frische SVN-Arbeitskopie aus und gibt den Verzeichnispfad aus, damit Claude Code ihn als Arbeitsverzeichnis der Sitzung verwenden kann. Fügen Sie die Konfiguration zu Ihrer [`settings.json`](/docs/de/settings#where-settings-live) hinzu:

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

Ein `WorktreeCreate`-Hook ermöglicht es Ihnen auch, [`/batch`](/docs/de/commands#all-commands) außerhalb eines Git-Repositorys auszuführen. Jeder `/batch`-Subagent veröffentlicht dann seine Änderung mit den Versionskontrollbefehlen Ihres Projekts und meldet, was er veröffentlicht hat, wenn er keinen Pull Request öffnen kann. Das Ausführen von `/batch` außerhalb eines Git-Repositorys erfordert Claude Code v2.1.281 oder später.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Claude Code meldet die folgenden Fehler, wenn es einen Worktree erstellt, einen beim Start betritt oder eine fortgesetzte Sitzung zu einem zurückbringt.

<h3 id="claude-code-can’t-enter-the-worktree-at-startup">
  Claude Code kann den Worktree beim Start nicht betreten
</h3>

Wenn Claude Code den Worktree-Verzeichnis beim Start nicht betreten kann, gibt es einen Fehler aus, der den Pfad benennt, und beendet sich mit Code 1. Dies kann passieren, wenn ein [`WorktreeCreate`-Hook](/docs/de/hooks#worktreecreate) etwas anderes als das erstellte Verzeichnis ausgibt, oder wenn das Verzeichnis nach der Einrichtung gelöscht wurde.

<h3 id="worktree-creation-fails-on-a-symlinked-path">
  Die Worktree-Erstellung schlägt auf einem symlink-Pfad fehl
</h3>

Claude Code weigert sich, einen Worktree zu erstellen, wenn `.claude`, `.claude/worktrees` oder das Worktree-Verzeichnis selbst ein Symlink ist, und der Fehler benennt den symlink-Pfad. Entfernen Sie den Symlink und versuchen Sie es erneut. Vor v2.1.212 folgte die Worktree-Erstellung einem Symlink, wenn das Repository bereits einen committeten Symlink an einem dieser Pfade enthielt, und konnte Dateien außerhalb des Repositories erstellen.

<h3 id="git-lfs-content-is-missing-from-a-worktree-claude-code-created">
  Git LFS-Dateien sind Zeiger-Dateien in einem Worktree, den Claude Code erstellt hat
</h3>

Wenn Sie [Git LFS](https://git-lfs.com) mit `git lfs install --local` einrichten, enthält ein Worktree, den Claude Code erstellt, LFS-Zeiger-Dateien statt der echten Dateien. Das `--local`-Flag schreibt den LFS-Filter in die `.git/config` des Repositories selbst, anstatt in Ihre globale Git-Konfiguration. Ein einfaches `git lfs install` schreibt in Ihre globale Konfiguration und ist nicht betroffen. Das gleiche gilt für jeden anderen [Filter-Treiber](https://git-scm.com/docs/gitattributes), der in der `.git/config` des Repositories selbst definiert ist.

Claude Code überspringt die Filter-Treiber des Repositories selbst, wenn es einen Worktree erstellt, da ein Filter-Treiber ein Shell-Befehl ist, und alles, das in das Repository schreiben kann, einschließlich Claude, könnte einen dort eingefügt haben. Vor v2.1.247 führte Claude Code diese Treiber während der Worktree-Erstellung aus.

Um die echten Dateien zu erhalten, führen Sie `git lfs pull` im Worktree aus.

In vier seltenen Fällen erstellt Claude Code überhaupt keinen Worktree: Es kann nicht feststellen, welche Filter-Treiber die Repository-Konfiguration definiert, oder es findet dort eine Einstellung, die es nicht ausschalten kann. Ordnen Sie den Fehler seiner Behebung zu:

* **`Could not read the repository git config to neutralize filter drivers`**: Claude Code konnte die `.git/config` des Repositories nicht lesen, zum Beispiel wegen ihrer Berechtigungen. Beheben Sie das und versuchen Sie es erneut.
* **`The repository git config defines a filter driver whose name cannot be neutralized (contains "=" or a newline)`**: Benennen Sie diesen Filter-Treiber in `.git/config` um oder entfernen Sie ihn und versuchen Sie es erneut.
* **`The repository git config has a conditional include (includeIf)`**: Verschieben Sie die Einstellungen, die `includeIf` in `.git/config` abruft, direkt in diese Datei, entfernen Sie `includeIf` und versuchen Sie es erneut. Ein `includeIf` in Ihrer globalen Git-Konfiguration löst dies nicht aus.
* **`Git was not run: the repository's own git config sets <key>`**: Die Meldung benennt einen Schlüssel, der Git LFS auf ein auszuführendes Programm verweist, wie `lfs.customtransfer.<name>.path` oder `lfs.standalonetransferagent`. Wenn diese Einstellung Ihnen gehört, verschieben Sie sie in Ihre globale Git-Konfiguration. Wenn Sie sie nicht erkennen, entfernen Sie sie aus der Repository-Konfiguration, da ein Tool oder Checkout, dem Sie nicht vertrauen, sie möglicherweise geschrieben hat. Versuchen Sie es erneut, sobald der Schlüssel aus der Repository-Konfiguration weg ist.

<h3 id="claude-code-refuses-to-use-a-worktree">
  Claude Code weigert sich, einen Worktree zu verwenden
</h3>

Ein Fehler, der mit `Refusing to use <path> as an isolation worktree` beginnt, bedeutet, dass Claude Code die Git-Identität des Verzeichnisses überprüft hat, bevor es es als isolierten Checkout einer Sitzung oder eines Subagents angenommen hat, und es abgelehnt hat. Die Prüfung läuft, ob Claude Code den Worktree erstellt, einen vorhandenen betritt oder einen aus einem früheren Lauf wiederverwenden.

In den meisten Fällen sagt der Rest der Meldung, dass die Git-Metadaten des Verzeichnisses in den Haupt-Checkout aufgelöst werden: Zum Beispiel zeigt seine `.git`-Datei auf das `.git`-Verzeichnis des Haupt-Repositories selbst, oder Git löst sein Arbeitsverzeichnis durch eine `core.worktree`-Umleitung zum Haupt-Checkout auf. Von einem solchen Verzeichnis würde ein gewöhnlicher Git-Befehl wie `git reset --hard` auf den Haupt-Checkout statt auf den Worktree wirken. Claude Code lehnt auch ab, wenn das Verzeichnis einen `.git`-Eintrag hat, den es nicht lesen kann, anstatt anzunehmen, dass der Worktree sicher ist.

Ein Verzeichnis ohne Git-Metadaten überhaupt, wie eines, das Ihr [`WorktreeCreate`-Hook](#non-git-version-control) erstellt, besteht die Prüfung nur, wenn kein Git-Repository es enthält. Wenn der Hook das Verzeichnis in einem Repository erstellt, löst Git es zu dem Checkout dieses Repositories auf und Claude Code lehnt es mit der Meldung `git resolves its working tree to` ab, daher sollte der Hook seine Verzeichnisse außerhalb eines Repositories erstellen.

Claude Code hinterlässt das abgelehnte Verzeichnis an Ort und Stelle, da es Arbeit enthalten kann. Ordnen Sie die Meldung ihrer Wiederherstellung zu, ob sie `Refusing to use <path>` folgt oder in einer [Fortsetzungsmeldung](#the-session-resumes-outside-its-worktree) angezeigt wird; einige Enden treten nur in Fortsetzungsmeldungen auf:

* **Sagt `launch from the parent checkout` oder `Run the resume from the project checkout`**: Sie haben Claude Code von innen im Worktree gestartet. Starten Sie stattdessen vom Haupt-Checkout; der Worktree benötigt keine Neuerstellung.
* **Sagt `it cannot be resumed or re-entered`**: Nichts in dieser Sitzung garantiert den Worktree von dort, wo Sie gestartet haben. Erstellen Sie ihn neu; das Verzeichnis und seine Arbeit bleiben auf der Festplatte zur manuellen Wiederherstellung, und wenn der Worktree einen Haupt-Checkout hat, funktioniert auch das Fortsetzen von dort.
* **Sagt `it contains the protected checkout`**: Das abgelehnte Verzeichnis ist ein übergeordnetes Element Ihres Haupt-Checkouts, wie Ihr Home-Verzeichnis. Löschen Sie es nicht. Ändern Sie den Worktree-Pfad, wie den Pfad, den Ihr `WorktreeCreate`-Hook zurückgibt, oder das `EnterWorktree`-Ziel, sodass der Worktree den Checkout nicht enthält.
* **Sagt `the protected checkout <path> has a .git entry that could not be examined` oder `has git metadata that could not be resolved`**: Das Problem ist die Git-Metadaten des Haupt-Checkouts, nicht die des Worktrees. Löschen Sie den Worktree nicht und ignorieren Sie den nachfolgenden Rat der Meldung, ihn neu zu erstellen, was auf diese beiden Enden nicht zutrifft. Reparieren Sie den Haupt-Checkout, zum Beispiel ein Berechtigungsproblem oder eine Git-`dubious ownership`-Ablehnung auf seinem `.git`, und versuchen Sie es erneut.
* **Sagt `its recorded path has a network spelling`**: Claude Code setzt niemals in einen Worktree auf einem Netzwerkpfad fort. Erstellen Sie den Worktree auf einem lokalen Pfad neu.
* **Jedes andere Ende**: Die Meldung benennt das Problem und seine Behebung, wie das Entfernen einer `core.worktree`-Umleitung oder die Neuerstellung des Worktrees; folgen Sie ihr. Bevor Sie ein Verzeichnis löschen, dessen Meldung sagt, dass seine Git-Identität nicht überprüft werden konnte, beheben Sie zunächst die benannte Ursache, zum Beispiel einen Symlink im Pfad des Worktrees oder Git selbst, das nicht ausgeführt wird, da das Verzeichnis gesund sein kann. Wenn Sie neu erstellen, retten Sie zunächst alle Änderungen, die Sie aus dem alten Verzeichnis benötigen; es bleibt auf der Festplatte.

<h3 id="the-session-resumes-outside-its-worktree">
  Die Sitzung wird außerhalb ihres Worktrees fortgesetzt
</h3>

Wenn Sie eine Sitzung interaktiv fortsetzen und Claude Code sie nicht zu ihrem Worktree zurückbringen kann, teilt Claude Code dies mit einer der folgenden Meldungen mit. Wenn Claude Code die Worktree-Bindung löscht, zeichnet es das Löschen im Sitzungstranskript auf. Wenn Sie [Transkriptschreibvorgänge unterdrücken](/docs/de/sessions#where-transcripts-are-stored), sagt die Meldung stattdessen, dass die Bindung nicht gelöscht werden konnte und dass Claude Code den Worktree bei einer späteren Fortsetzung erneut überprüft.

| Meldung beginnt mit                               | Was passiert ist und was zu tun ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Your worktree <path> no longer exists`           | Das Worktree-Verzeichnis wurde entfernt. Die Sitzung wird im aktuellen Verzeichnis ohne Isolation fortgesetzt, und Claude Code löscht die Worktree-Bindung. Keine Aktion erforderlich.                                                                                                                                                                                                                                                                                                                                                                   |
| `Could not verify your worktree <path> this time` | Claude Code konnte den Worktree nicht überprüfen, normalerweise aus einem vorübergehenden Grund; die Bindung wird beibehalten, und die Sitzung wird im aktuellen Verzeichnis ohne Isolation fortgesetzt. Setzen Sie erneut fort, um es erneut zu versuchen; wenn es weiterhin passiert, betreten Sie den Worktree in einer neuen Sitzung und ordnen Sie die Ablehnungsmeldung unter [Claude Code weigert sich, einen Worktree zu verwenden](#claude-code-refuses-to-use-a-worktree) zu, die stattdessen die Metadaten des Haupt-Checkouts benennen kann. |
| `Did not re-enter your worktree <path>`           | Claude Code lehnte die Worktree-Bindung als unsicher ab; es löscht die Bindung und die Sitzung wird ohne Isolation fortgesetzt. Die Meldung enthält die spezifische Ablehnung: Ordnen Sie sie unter [Claude Code weigert sich, einen Worktree zu verwenden](#claude-code-refuses-to-use-a-worktree) zu, da die Behebung für einige Ablehnungen Neuerstellung und für andere Pfadänderung ist.                                                                                                                                                            |
| `Could not re-enter your worktree <path>`         | Claude Code konnte den Worktree von dort, wo Sie gestartet haben, nicht garantieren, am häufigsten, weil Sie von innen gestartet haben; die Bindung wird beibehalten. Der Rest der Meldung benennt die Behebung; ordnen Sie sie unter [Claude Code weigert sich, einen Worktree zu verwenden](#claude-code-refuses-to-use-a-worktree) zu.                                                                                                                                                                                                                |

Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` und bei Fortsetzungen, die das [Agent SDK](/docs/de/agent-sdk/sessions) ausführt, stoppt Claude Code die Fortsetzung mit einem stderr-Fehler für jede Ablehnung außer einem gegangenen Worktree, anstatt ohne Isolation fortzufahren.

Mit `--output-format stream-json` kommt die Ablehnung auch auf stdout als eine `result`-Meldung mit Subtyp `error_during_execution` an, deren `errors`-Array denselben Text trägt, sodass eine Agent SDK-Anwendung den Grund erhält, anstatt nur einen Nicht-Null-Exit. Vor v2.1.260 erzeugte eine Worktree-Fortsetzungsablehnung keine `result`-Meldung.

Die Meldungen haben andere Formen als die interaktiven Meldungen in der Tabelle:

* `Error: cannot resume into worktree <path>: ...This session was not started.` für eine Ablehnung, die die Tabelle als `Did not re-enter` zeigt. Claude Code löscht die Worktree-Bindung vor dem Beenden, und der Fehler sagt dies; das nächste Mal, wenn Sie die Unterhaltung fortsetzen, wird die Sitzung im aktuellen Verzeichnis ohne Worktree-Isolation fortgesetzt. Vor v2.1.260 schrieb Claude Code das gelöschte Binding nicht, daher schlug jeder Wiederholungsversuch derselben Fortsetzung mit demselben Fehler fehl.

  Wenn Sie [Transkriptschreibvorgänge unterdrücken](/docs/de/sessions#where-transcripts-are-stored), kann das Löschen nicht gespeichert werden. Der Fehler sagt dann, dass derselbe Befehl erneut abgelehnt wird, und benennt `--fork-session` und das Starten einer neuen Unterhaltung als Wege, um ohne den Worktree fortzufahren.
* `Error: could not verify worktree <path> for this resume, so the resume was aborted...` für `Could not verify`
* `Error: ...The worktree binding is kept.` für `Could not re-enter`
* `Notice: the worktree <path> for this session no longer exists...` für einen gegangenen Worktree; Claude Code gibt ihn aus und setzt die Sitzung fort, wie eine interaktive Fortsetzung

Das Ablehnungsende, das in jedem Fehler eingebettet ist, wird mit den interaktiven Hinweisen geteilt, sodass es immer noch seiner Eintrag unter [Claude Code weigert sich, einen Worktree zu verwenden](#claude-code-refuses-to-use-a-worktree) entspricht.

Im stream-json-Ergebnis ist [`startup_failure_reason`](/docs/de/agent-sdk/typescript#startup_failure_reason) `worktree_unverified` für den Fehler `could not verify worktree` und `worktree_resume_refused` für die Fehler `cannot resume into worktree` und `The worktree binding is kept`. Eine Anwendung kann stattdessen darauf verzweigen, anstatt den Fehlertext abzugleichen. Vor v2.1.274 trug das Ergebnis kein `startup_failure_reason`-Feld.

<h2 id="see-also">
  Siehe auch
</h2>

Worktrees handhaben die Datei-Isolation. Die verwandten Seiten unten behandeln die Delegierung von Arbeit in diese isolierten Checkouts, das Weitergeben von Erkenntnissen zwischen ihnen und das Wechseln zwischen den Sitzungen, die Sie erstellen:

* [Subagents](/docs/de/sub-agents): Delegieren Sie Arbeit an isolierte Agenten innerhalb einer Sitzung
* [Sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging): Lassen Sie die Sitzungen in Ihren Worktrees Erkenntnisse aneinander weitergeben
* [Agent-Teams](/docs/de/agent-teams): Koordinieren Sie mehrere Claude-Sitzungen automatisch
* [Sitzungen verwalten](/docs/de/sessions): Benennen, fortsetzen und wechseln Sie zwischen Gesprächen
* [Desktop-Parallelsitzungen](/docs/de/desktop#work-in-parallel-with-sessions): Worktree-gestützte Sitzungen in der Desktop-App
