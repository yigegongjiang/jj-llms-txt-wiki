> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude mit Skills erweitern

> Erstellen, verwalten und teilen Sie Skills, um die Funktionen von Claude in Claude Code zu erweitern. Umfasst benutzerdefinierte Befehle und gebündelte Skills.

Skills erweitern die Möglichkeiten von Claude. Erstellen Sie eine `SKILL.md`-Datei mit Anweisungen, und Claude fügt sie zu seinem Toolkit hinzu. Claude verwendet Skills, wenn sie relevant sind, oder Sie können einen direkt mit `/skill-name` aufrufen.

Erstellen Sie einen Skill, wenn Sie dieselben Anweisungen, Checklisten oder mehrstufige Verfahren immer wieder in den Chat einfügen, oder wenn ein Abschnitt von CLAUDE.md zu einem Verfahren statt zu einer Tatsache geworden ist. Im Gegensatz zu CLAUDE.md-Inhalten wird der Text eines Skills nur geladen, wenn er verwendet wird, sodass umfangreiches Referenzmaterial fast nichts kostet, bis Sie es benötigen.

<Note>
  Informationen zu integrierten Befehlen wie `/help` und `/compact` sowie zu gebündelten Skills wie `/debug` und `/code-review` finden Sie in der [Befehlsreferenz](/docs/de/commands).

  **Benutzerdefinierte Befehle wurden in Skills zusammengeführt.** Eine Datei unter `.claude/commands/deploy.md` und ein Skill unter `.claude/skills/deploy/SKILL.md` erstellen beide `/deploy` und funktionieren auf die gleiche Weise. Ihre vorhandenen `.claude/commands/`-Dateien funktionieren weiterhin. Skills bieten optionale Funktionen: ein Verzeichnis für unterstützende Dateien, Frontmatter zur [Kontrolle, ob Sie oder Claude sie aufrufen](#control-who-invokes-a-skill), und die Möglichkeit für Claude, sie automatisch zu laden, wenn sie relevant sind.
</Note>

Claude Code Skills folgen dem [Agent Skills](https://agentskills.io) offenen Standard, der über mehrere KI-Tools hinweg funktioniert. Claude Code erweitert den Standard um zusätzliche Funktionen wie [Aufrufersteuerung](#control-who-invokes-a-skill), [Subagent-Ausführung](#run-skills-in-a-subagent) und [dynamische Kontexteinspeisung](#inject-dynamic-context). Siehe [Verwendung von Skill-Frontmatter außerhalb von Claude Code](#using-skill-frontmatter-outside-claude-code) für die Frontmatter-Felder, die Teil des Standards sind, und welche Claude Code-Erweiterungen sind.

<h2 id="bundled-skills">
  Gebündelte Skills
</h2>

Claude Code enthält eine Reihe von gebündelten Skills wie `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` und `/claude-api`. Gebündelte Skills sind prompt-basiert: Sie geben Claude detaillierte Anweisungen und ermöglichen es ihm, die Arbeit mit seinen Tools zu orchestrieren. Die meisten integrierten Befehle führen stattdessen direkt eine feste Logik aus.

Sie rufen einen gebündelten Skill auf die gleiche Weise auf wie jeden anderen Skill, indem Sie `/` gefolgt vom Skill-Namen eingeben. Claude ruft einige gebündelte Skills automatisch auf, wenn sie relevant sind; andere, einschließlich `/verify`, werden nur ausgeführt, wenn Sie sie aufrufen, was Ihnen die Kontrolle darüber gibt, wann diese längeren Überprüfungen Zeit und Token aufwenden.

Die meisten gebündelten Skills sind in jeder Sitzung verfügbar. Einige hängen von einer bestimmten Funktion ab: `/workflow-authoring` ist beispielsweise nur verfügbar, wenn [dynamische Workflows](/docs/de/workflows) aktiviert sind.

Um gebündelte Skills auszuschalten, verwenden Sie die Einstellung [`disableBundledSkills`](/docs/de/settings-reference#disablebundledskills).

<Note>
  Die Einrichtungsüberprüfung [`/doctor`](/docs/de/commands#all-commands) bleibt eingabbar, wenn `disableBundledSkills` aktiviert ist, in Claude Code v2.1.205 und später. Um sie auszublenden, setzen Sie die Umgebungsvariable `DISABLE_DOCTOR_COMMAND` oder einen [`skillOverrides`](#override-skill-visibility-from-settings)-Eintrag von `"doctor": "off"`. Vor v2.1.205 war `/doctor` ein integrierter Befehl und kein gebündelter Skill.
</Note>

Gebündelte Skills werden zusammen mit integrierten Befehlen in der [Befehlsreferenz](/docs/de/commands) aufgelistet, gekennzeichnet mit **Skill** in der Spalte „Zweck".

<h3 id="run-and-verify-your-app">
  Führen Sie Ihre App aus und überprüfen Sie sie
</h3>

Drei gebündelte Skills arbeiten zusammen, um Ihre App zu starten und Änderungen gegen die laufende App zu bestätigen, anstatt nur gegen Tests:

| Skill                  | Zweck                                                                                                                                                       |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Starten und steuern Sie Ihre App, um eine Änderung in Aktion zu sehen                                                                                       |
| `/verify`              | Erstellen und führen Sie Ihre App aus, um zu bestätigen, dass eine Codeänderung das tut, was sie soll, ohne auf Tests oder Typüberprüfungen zurückzugreifen |
| `/run-skill-generator` | Lehren Sie `/run` und `/verify`, wie Sie Ihr Projekt erstellen und starten                                                                                  |

`/run` und `/verify` funktionieren ohne Einrichtung. Sie leiten den Start von Ihrem Projekttyp ab (CLI, Server, TUI, Browser-gesteuert) und von dem, was sich in Ihrer README, `package.json` oder `Makefile` befindet. Diese Ableitung wird unzuverlässig für Projekte, die mehr als einen Standard-Start benötigen: eine Datenbank, eine Env-Datei, eine grafische Sitzung, einen mehrstufigen Build.

`/run-skill-generator` zeichnet stattdessen das Rezept auf. Es bringt Ihre App aus einer sauberen Umgebung zum Laufen, erfasst, was funktioniert hat (die Installationsbefehle, die Umgebungsvariablen, das Startskript), und speichert es als projektspezifischen Skill unter `.claude/skills/run-<name>/`. Danach folgen `/run`, `/verify` und alle anderen Agenten im Repository dem aufgezeichneten Rezept, anstatt es neu zu entdecken. Führen Sie `/run-skill-generator` einmal pro Projekt aus, und erneut, wenn sich der Build- oder Startprozess ändert.

`/verify` kann auch sein eigenes Rezept aufzeichnen. Wenn es Ihre App ohne ein aufgezeichnetes Rezept erstellen und steuern muss, schreibt es, was funktioniert hat, in `.claude/skills/verify/SKILL.md` im Repository-Root oder im betroffenen Paketverzeichnis in einem Monorepo, damit spätere Läufe und andere Agenten die gleichen Schritte befolgen. Im Repository-Root ersetzt das aufgezeichnete Skill den gebündelten `/verify`. Dies erfordert Claude Code v2.1.200 oder später.

Claude bearbeitet die aufgezeichnete Datei nur, wenn es einen Lauf falsch gesteuert hat, z. B. einen Befehl, der fehlgeschlagen ist, oder einen fehlenden Schritt, damit Sie die Datei ohne sitzungsspezifische Diffs committen können. Vor v2.1.205 sagte der gebündelte Skill Claude, dass es alles einbeziehen sollte, was ein Lauf gelernt hat, was häufige Merge-Konflikte verursachte.

<h2 id="getting-started">
  Erste Schritte
</h2>

<h3 id="create-your-first-skill">
  Erstellen Sie Ihre erste Skill
</h3>

Dieses Beispiel erstellt eine Skill, die die nicht committeten Änderungen in Ihrem Git-Repository zusammenfasst und alles Riskante kennzeichnet. Sie zieht den Live-Diff in den Prompt, bevor Claude ihn liest, sodass die Antwort in Ihrem tatsächlichen Arbeitsbaum verankert ist, anstatt auf dem, was Claude aus offenen Dateien erraten kann. Claude lädt die Skill automatisch, wenn Sie nach Ihren Änderungen fragen, oder Sie können sie direkt mit `/summarize-changes` aufrufen.

<Steps>
  <Step title="Erstellen Sie das Skill-Verzeichnis">
    Erstellen Sie ein Verzeichnis für die Skill in Ihrem persönlichen Skills-Ordner. Persönliche Skills sind in allen Ihren Projekten verfügbar.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Schreiben Sie SKILL.md">
    Jede Skill benötigt eine `SKILL.md`-Datei mit zwei Teilen: YAML-Frontmatter zwischen `---`-Markierungen, das Claude mitteilt, wann die Skill verwendet werden soll, und Markdown-Inhalt mit den Anweisungen, die Claude befolgt, wenn die Skill ausgeführt wird. Der Verzeichnisname wird zum Befehl, den Sie eingeben, und die `description` hilft Claude zu entscheiden, wann die Skill automatisch geladen werden soll.

    Speichern Sie dies unter `~/.claude/skills/summarize-changes/SKILL.md`:

    ```yaml theme={null}
    ---
    description: Fasst nicht committete Änderungen zusammen und kennzeichnet alles Riskante. Verwenden Sie dies, wenn der Benutzer fragt, was sich geändert hat, eine Commit-Nachricht möchte oder seinen Diff überprüfen möchte.
    ---

    ## Aktuelle Änderungen

    !`git diff HEAD`

    ## Anweisungen

    Fassen Sie die obigen Änderungen in zwei oder drei Aufzählungspunkten zusammen, listen Sie dann alle Risiken auf, die Sie bemerken, wie fehlende Fehlerbehandlung, hartcodierte Werte oder Tests, die aktualisiert werden müssen. Wenn der Diff leer ist, sagen Sie, dass es keine nicht committeten Änderungen gibt.
    ```

    Die Zeile `` !`git diff HEAD` `` verwendet [dynamische Kontextinjektion](#inject-dynamic-context): Claude Code führt den Befehl aus und ersetzt die Zeile durch seine Ausgabe, bevor Claude den Skill-Inhalt sieht, sodass die Anweisungen mit dem aktuellen Diff bereits inline ankommen.
  </Step>

  <Step title="Testen Sie die Skill">
    Öffnen Sie ein Git-Projekt, nehmen Sie eine kleine Änderung an einer beliebigen Datei vor, und starten Sie Claude Code, indem Sie `claude` ausführen. Sie können die Skill auf zwei Arten testen.

    **Lassen Sie Claude sie automatisch aufrufen**, indem Sie etwas eingeben, das der Beschreibung entspricht:

    ```text theme={null}
    What did I change?
    ```

    **Oder rufen Sie sie direkt auf** mit dem Skill-Namen:

    ```text theme={null}
    /summarize-changes
    ```

    In beiden Fällen sollte Claude mit einer kurzen Zusammenfassung Ihrer Änderung und einer Liste von Risiken antworten.
  </Step>
</Steps>

<h2 id="where-skills-live">
  Wählen Sie, wo Skills geladen werden
</h2>

Wo Sie einen Skill speichern, entscheidet, welche Sitzungen ihn laden. Speichern Sie ihn in Ihrem Home-Verzeichnis, um ihn in jedem Projekt zu erhalten, committen Sie ihn in ein Repository, um ihn mit allen zu teilen, die dort arbeiten, oder verteilen Sie ihn über ein Plugin oder verwaltete Einstellungen, um ein ganzes Team zu erreichen.

| Speicherort              | Pfad                                                                                                                           | Wird geladen in                                                                                                                                                                                                                                          |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Enterprise               | `.claude/skills/<skill-name>/SKILL.md` im [Verzeichnis für verwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms) | Alle Benutzer auf Maschinen, auf denen Ihre Organisation es bereitstellt                                                                                                                                                                                 |
| Persönlich               | `~/.claude/skills/<skill-name>/SKILL.md`                                                                                       | Alle Ihre Projekte auf dieser Maschine, aber nicht [Cowork- oder Cloud-Sitzungen](#skills-in-cowork-and-cloud-sessions)                                                                                                                                  |
| Projekt                  | `.claude/skills/<skill-name>/SKILL.md`                                                                                         | Sitzungen in diesem Repository. Committen Sie es, damit Ihr Team es auch erhält                                                                                                                                                                          |
| Verschachtelt            | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                                | Sitzungen, die in oder unter `<subdir>` gestartet werden. Eine Sitzung, die darüber gestartet wird, lädt den Skill einmal, wenn Claude an Dateien dort arbeitet. Siehe [Monorepos und Unterverzeichnisse](#discovery-from-parent-and-nested-directories) |
| Zusätzliches Verzeichnis | `.claude/skills/<skill-name>/SKILL.md` in einem Verzeichnis, das Sie mit `--add-dir` übergeben                                 | Diese Sitzung. Siehe [Verzeichnisse außerhalb des Projekts](#skills-from-additional-directories)                                                                                                                                                         |
| Plugin                   | `<plugin>/skills/<skill-name>/SKILL.md`                                                                                        | Überall dort, wo das [Plugin](/docs/de/plugins/overview) aktiviert ist, als `/plugin-name:skill-name`                                                                                                                                                         |
| claude.ai-Konto          | Skills, die für Ihr claude.ai-Konto aktiviert sind                                                                             | Cowork-Sitzungen, Cloud-Sitzungen und Terminal-Sitzungen, in denen Sie sich mit diesem Konto anmelden. Siehe [Skills, die von claude.ai synchronisiert werden](#how-synced-skills-behave)                                                                |

Skill-Ordner folgen auch diesen Regeln:

* **Symverlinkte Ordner**: Ein `<skill-name>`-Eintrag am Enterprise-, Personal- oder Projekt-Speicherort kann ein Symlink zu einem Verzeichnis an anderer Stelle auf der Festplatte sein. Claude Code liest `SKILL.md` aus dem Ziel und lädt den Skill einmal, auch wenn mehrere Speicherorte auf dasselbe Ziel verweisen. Plugin-Skills [handhaben Symlinks anders](/docs/de/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks).
* **Reservierter Name**: Benennen Sie einen Skill-Ordner nicht `synced`, in keiner Schreibweise. Claude Code verwendet `~/.claude/skills/synced/` für [Skills, die von claude.ai heruntergeladen werden](#where-synced-skills-load) und überspringt einen Skill, den Sie unter diesem Namen an den Enterprise-, Personal- und Projekt-Speicherorten erstellen.
* **Befehlsdateien**: Eine Markdown-Datei in `.claude/commands/` ist das ältere Format und funktioniert immer noch. Sie unterstützt dieselbe [Frontmatter](#frontmatter-reference) außer `name` und `paths`. Um den Namen zu finden, den Sie eingeben, um ihn aufzurufen, siehe [Wie ein Skill seinen Befehlsnamen erhält](#how-a-skill-gets-its-command-name). Bevorzugen Sie einen Skill für neue Arbeiten, da Skills auch [unterstützende Dateien](#add-supporting-files) unterstützen.
* **Skill-Ordner als Plugin**: Fügen Sie eine `.claude-plugin/plugin.json` zu einem Skill-Ordner hinzu und er wird als [Plugin](/docs/de/plugins/loading#plugins-shared-through-a-repository) mit dem Namen `<name>@skills-dir` geladen, sodass er Agents, Hooks und MCP-Server bündeln kann. In einem Projekt `.claude/skills/` ist dies erforderlich, um zuerst den Workspace-Trust-Dialog zu akzeptieren.

<h3 id="discovery-from-parent-and-nested-directories">
  Skills in Monorepos und Unterverzeichnissen laden
</h3>

Claude Code lädt Projekt-Skills aus `.claude/skills/` in dem Verzeichnis, in dem Sie ihn starten, und in jedem übergeordneten Verzeichnis bis zur Repository-Root, sodass das Starten in `packages/frontend/` immer noch Skills aufgreift, die in der Root definiert sind. Wenn Sie [die Sitzung mit `/cd`](/docs/de/permissions#move-the-session-to-another-directory) auf v2.1.246 oder später verschieben, fügt Claude Code die Projekt-Skills des neuen Verzeichnisses hinzu.

In einer verknüpften [Git Worktree](/docs/de/worktrees) sucht Claude Code übergeordnete Verzeichnisse nur bis zur Worktree-Root. Auf Claude Code v2.1.277 oder später lädt Claude Code, wenn der Worktree-Checkout kein `.claude/skills`-Verzeichnis in seiner Root hat, stattdessen die Projekt-Skills des Haupt-Checkouts. Siehe [Was Worktrees mit dem Haupt-Checkout teilen](/docs/de/worktrees#what-worktrees-share-with-the-main-checkout).

Skills in einem `.claude/skills/`-Verzeichnis unter dem Ort, an dem Sie gestartet haben, werden beim Start nicht geladen. Sie werden geladen, wenn Claude zum ersten Mal eine Datei in diesem Unterverzeichnis liest oder bearbeitet, und bleiben für den Rest der Sitzung verfügbar. Bis dahin erscheinen sie nicht im `/`-Menü und Sie können sie nicht nach Name aufrufen. Um sie früher zu laden, führen Sie `/add-dir` mit dem Pfad des Unterverzeichnisses aus, was Claude Code v2.1.257 oder später erfordert.

Wenn ein verschachtelter Skill denselben Namen wie ein anderer Skill hat, bleiben beide verfügbar. Mit einem `deploy`-Skill in der Repository-Root und einem anderen in `apps/web/.claude/skills/`:

* `/deploy` führt den Root-Skill aus. Claude Code listet auch die verzeichnisqualifizierten Varianten für Claude auf, mit einer Anweisung, den aufzurufen, dessen Verzeichnis die Dateien enthält, an denen es arbeitet, sodass der verschachtelte Skill immer noch auf Arbeiten in `apps/web/` angewendet wird.
* `/apps/web:deploy` führt den verschachtelten Skill allein aus. Seine Beschreibung benennt das Verzeichnis, auf das es angewendet wird.

<h3 id="skills-from-additional-directories">
  Skills aus einem Verzeichnis außerhalb des Projekts laden
</h3>

Wenn Sie ein Verzeichnis mit `--add-dir` oder `/add-dir` hinzufügen, lädt Claude Code die Skills in `.claude/skills/` dieses Verzeichnisses zusammen mit `.claude/commands/` und `.claude/agents/`. Verzeichnisse, die das Agent SDK durch [`additionalDirectories`](/docs/de/agent-sdk/typescript#options) in TypeScript oder [`add_dirs`](/docs/de/agent-sdk/python#claudeagentoptions) in Python hinzufügt, werden auf die gleiche Weise geladen, da das SDK sie als `--add-dir` übergibt. Die Einstellung `permissions.additionalDirectories` in `settings.json` gewährt nur Dateizugriff und lädt keine dieser.

Claude Code überwacht `.claude/skills/` in einem Verzeichnis, das Sie mit `--add-dir` beim Start übergeben, wie [Bearbeiten Sie einen Skill während einer Sitzung](#live-change-detection) beschreibt. Es überwacht nicht `.claude/commands/` oder `.claude/agents/` des hinzugefügten Verzeichnisses, daher starten Sie die Sitzung nach dem Ändern einer Datei dort neu.

Diese Ladevorgänge hängen von der `project` [Einstellungsquelle](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) ab, die standardmäßig aktiviert ist. Eine [`strictPluginOnlyCustomization`](/docs/de/settings-reference#strictpluginonlycustomization)-Richtlinie, [Bare Mode](/docs/de/headless#start-faster-with-bare-mode) und [`--safe-mode`](/docs/de/cli-reference#cli-flags) beschränken sie weiter, wie diese Seiten beschreiben. Siehe [Zusätzliche Verzeichnisse gewähren Dateizugriff, keine Konfiguration](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) für die vollständige Tabelle, was ein hinzugefügtes Verzeichnis lädt, einschließlich `CLAUDE.md` und Plugin-Einstellungen.

<h3 id="resolve-skills-that-share-a-name">
  Beheben Sie Skills, die denselben Namen haben
</h3>

Wenn zwei Skills denselben Namen haben, entscheidet, woher jeder kommt, welcher `/name` ausführt. Die Tabelle behandelt die Enterprise-, Personal-, Projekt-, verschachtelte, Plugin- und claude.ai-Speicherorte, gebündelte Skills und Befehlsdateien:

| Gleicher Name in                                                                                     | Welcher wird ausgeführt                                                                                                                                                                                                                                   |
| :--------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Zwei von Enterprise, Personal und Projekt                                                            | Enterprise über Personal und Personal über Projekt. Mit `deploy` in beiden `~/.claude/skills/` und `.claude/skills/` des Projekts führt `/deploy` den Personal-Skill aus                                                                                  |
| Einer dieser Speicherorte und ein [gebündelter Skill](#bundled-skills)                               | Ihr Skill ersetzt den gebündelten Befehl, aber nicht seine Aliase. Ein Projekt-`code-review`-Skill ersetzt `/code-review`, und der gebündelte Alias `/review` führt Ihren Skill nie aus                                                                   |
| Ein Skill und eine Datei in `.claude/commands/`                                                      | Der Skill                                                                                                                                                                                                                                                 |
| Ein Projekt-Root-Skill und ein verschachtelter Skill                                                 | Beide werden geladen. Siehe [Monorepos und Unterverzeichnisse](#discovery-from-parent-and-nested-directories)                                                                                                                                             |
| Ein Plugin-Skill und ein Skill an einem der obigen Speicherorte                                      | Beide werden geladen, da Plugin-Skills als `/plugin-name:skill-name` namensgebunden sind                                                                                                                                                                  |
| Einer der obigen und ein Skill [von Ihrem claude.ai-Konto synchronisiert](#how-synced-skills-behave) | Der andere Skill oder Befehl. Der synchronisierte Skill wird immer noch als `/anthropic-skills:<name>` ausgeführt. Siehe [Wenn ein synchronisierter Skill-Name mit einem anderen Befehl übereinstimmt](#when-a-synced-skill-name-matches-another-command) |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Verwenden Sie Skills in Cowork- und Cloud-Sitzungen
</h3>

[Cowork](https://claude.com/product/cowork)-Sitzungen und [Cloud-Sitzungen](/docs/de/cloud-environments#what-carries-over-from-your-setup), einschließlich [Routinen](/docs/de/routines), lesen nicht `~/.claude/skills/` auf Ihrer Maschine. Sowohl interaktive als auch geplante Cowork-Sitzungen laden die Skills, die für Ihr claude.ai-Konto aktiviert sind, synchronisiert beim Sitzungsstart; verwalten Sie sie über **Customize** in der Desktop-App-Seitenleiste oder in den Skill-Einstellungen auf claude.ai. Cloud-Sitzungen laden zusätzlich Projekt-Skills, die in `.claude/skills/` des geklonten Repositorys committet sind.

Wenn ein Skill nur in `~/.claude/skills/` auf Ihrer Maschine vorhanden ist, meldet Claude Code, dass der Skill nicht gefunden wurde, wenn eine [Routine](/docs/de/routines) ihn aufruft, da jede Routine-Ausführung als frische Cloud-Sitzung startet. Um einen persönlichen Skill in diesen Sitzungen verfügbar zu machen:

* Für Cowork- und Cloud-Sitzungen aktivieren Sie den Skill für Ihr claude.ai-Konto.
* Für Cloud-Sitzungen können Sie den Skill stattdessen in `.claude/skills/` des Repositorys committen. Plugins, die in `.claude/settings.json` des Repositorys deklariert sind, [werden beim Sitzungsstart geladen](/docs/de/cloud-environments#what-carries-over-from-your-setup); Plugins, die nur in Ihren Benutzereinstellungen aktiviert sind, werden nicht übertragen.

[Desktop-geplante Aufgaben](/docs/de/desktop-scheduled-tasks) werden lokal auf Ihrer Maschine ausgeführt, daher laden sie `~/.claude/skills/`.

<h3 id="how-synced-skills-behave">
  Skills, die von claude.ai synchronisiert werden
</h3>

Dieser Abschnitt gilt für Sie, wenn Sie Cowork- oder Cloud-Sitzungen verwenden oder sich in Ihrem Terminal mit einem claude.ai-Konto bei Claude Code anmelden. In diesen Sitzungen lädt Claude Code die Skills, die für Ihr claude.ai-Konto aktiviert sind, ohne Setup auf Ihrer Seite, wie [Wo synchronisierte Skills geladen werden](#where-synced-skills-load) beschreibt. Diese Skills umfassen die Skills, die Sie in Ihren claude.ai-Einstellungen erstellen oder aktivieren, Skills, die Ihre Organisation dort bereitstellt, und Anthropics integrierte Skills wie `pdf` und `xlsx`.

Claude Code lädt einen synchronisierten Skill von Ihrem Konto herunter, anstatt eine Datei zu lesen, die Sie auf der Maschine geschrieben haben, auf der die Sitzung läuft, daher wendet es Regeln auf synchronisierte Skills an, die nicht auf die Skills angewendet werden, die Sie in den [Skill-Speicherorten](#where-skills-live) speichern.

<h4 id="where-synced-skills-load">
  Wo synchronisierte Skills geladen werden
</h4>

In einer Cowork- oder Cloud-Sitzung lädt Claude Code die Skills, die für Ihr claude.ai-Konto aktiviert sind, und [Skills in Cowork- und Cloud-Sitzungen](#skills-in-cowork-and-cloud-sessions) sagt, wie Sie wählen, welche Skills diese Sitzungen erhalten.

In Ihrem Terminal synchronisiert Claude Code diese Skills in Sitzungen, in denen Sie sich mit Ihrem claude.ai-Konto anmelden. Wenn die Sitzung startet, lädt Claude Code Ihre Account-Skills im Hintergrund in `~/.claude/skills/synced/` herunter und prüft dann etwa alle 10 Minuten auf Änderungen auf claude.ai, während die Sitzung läuft. Wenn eine Überprüfung feststellt, dass ein Skill auf claude.ai hinzugefügt, bearbeitet oder deaktiviert wurde, fügt Claude Code ihn in der laufenden Sitzung hinzu, aktualisiert oder entfernt ihn ohne Neustart. Die Synchronisierung in Terminal-Sitzungen erfordert Claude Code v2.1.273 oder später.

Die Synchronisierung verzögert niemals den Start, da Claude auf den Download eines Skills nur wartet, wenn es diesen Skill aufruft. Ein kurzer [nicht-interaktiver](/docs/de/headless) Lauf kann daher beendet werden, bevor ein neu hinzugefügter Skill heruntergeladen wird. In diesem Fall lädt eine spätere Sitzung ihn herunter. Um einen nicht-interaktiven Lauf dazu zu bringen, Ihre Skills herunterzuladen und auf die Liste zu warten, bevor er die Eingabeaufforderung beantwortet, setzen Sie [`CLAUDE_CODE_SYNC_SKILLS`](/docs/de/env-vars#variables) auf `1`.

Claude Code synchronisiert nur in einer Sitzung, die sich mit Ihrem claude.ai-Konto anmeldet und [Feature-Flags von Anthropic abruft](/docs/de/env-vars#features-that-need-feature-flag-fetching). Es synchronisiert nicht in diesen Sitzungen:

* Eine Sitzung, die keine von `/login` gespeicherte Anmeldung verwendet, wie eine, die sich mit einem API-Schlüssel authentifiziert, oder eine, bei der `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN` oder ein `apiKeyHelper`-Skript die Anmeldedaten bereitstellt
* Eine Sitzung, die keine Feature-Flags abruft, wie eine auf Amazon Bedrock oder eine, bei der Sie `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` setzen
* Eine Sitzung im [Bare Mode](/docs/de/headless#start-faster-with-bare-mode) oder eine, die Sie mit `--safe-mode` starten
* Eine Sitzung, bei der die verwalteten Einstellungen Ihrer Organisation [Skills auf Plugin-Quellen sperren](/docs/de/settings-reference#strictpluginonlycustomization-skills), oder eine, die Sie mit einer [`--setting-sources`](/docs/de/cli-reference#cli-flags)-Liste starten, die `user` auslässt

Wenn Sie sich während einer Sitzung mit `/login` anmelden, starten Sie Claude Code neu, um die Synchronisierung zu starten.

Skills, die eine frühere Sitzung synchronisiert hat, bleiben auf der Festplatte. Claude Code lädt sie in späteren Sitzungen, die sich mit demselben Konto anmelden, auch wenn es claude.ai nicht erreichen kann.

Claude Code lädt synchronisierte Skills herunter und lädt sie nie hoch. Wenn Sie oder Claude eine Datei unter `~/.claude/skills/synced/` bearbeiten, wird die Änderung nicht in Ihrem claude.ai-Konto gespeichert, und eine spätere Synchronisierung kann sie überschreiben oder entfernen. Um einen synchronisierten Skill zu ändern, aktualisieren Sie ihn auf claude.ai; die nächste Synchronisierung lädt die neue Version herunter.

Um zu sehen, welche Skills synchronisiert wurden, führen Sie `/skills` aus. Das Menü listet sie unter `claude.ai sync` auf.

Einige von Anthropics Skills, wie `pdf` und `xlsx`, werden immer synchronisiert. Für die übrigen aktivieren oder deaktivieren Sie einen Skill in Ihren Skill-Einstellungen auf claude.ai, um zu ändern, ob er synchronisiert wird.

Um die Synchronisierung auf einer Maschine zu beenden, setzen Sie [`syncClaudeAiSkills`](/docs/de/settings-reference#syncclaudeaiskills) in Ihren Benutzereinstellungen auf `false`. Claude Code stoppt das Herunterladen, und beim nächsten Start verschiebt es die Skills, die es bereits synchronisiert hat, in `~/.claude/skills/.trash/` und lädt sie nicht mehr. Ihre Organisation kann die Synchronisierung für alle ausschalten, indem sie Skills auf claude.ai ausschaltet. Um die Synchronisierung zu beenden und Skills eingeschaltet zu lassen, kann sie denselben Schlüssel in [verwalteten Einstellungen](/docs/de/managed-settings) setzen.

Wenn Ihre Organisation Skills auf claude.ai ausschaltet, entfernt Claude Code die heruntergeladenen Skills und sie werden nicht mehr geladen. Die entfernten Skills werden in `~/.claude/skills/.trash/` verschoben, wo Sie die Dateien wiederherstellen können, bis die [Aufbewahrungslöschung](/docs/de/claude-directory#cleaned-up-automatically) sie löscht. Sobald Ihre Organisation Skills wieder einschaltet, lädt Claude Code die Skills herunter, die Sie beim nächsten Synchronisierungsvorgang aktiviert haben.

<h4 id="when-a-synced-skill-name-matches-another-command">
  Wenn ein synchronisierter Skill-Name mit einem anderen Befehl übereinstimmt
</h4>

Sie können einen synchronisierten Skill mit seinem vollständigen Namen, `/anthropic-skills:<name>`, oder seinem Kurznamen, `/<name>`, aufrufen. Wenn ein anderer Befehl diesen Kurznamen verwendet, führt `/<name>` den anderen Befehl aus, und der synchronisierte Skill wird nur als `/anthropic-skills:<name>` ausgeführt. Mit einem lokalen `deploy`-Skill und einem synchronisierten `deploy` führt `/deploy` den lokalen Skill aus und `/anthropic-skills:deploy` führt den synchronisierten aus. Vor v2.1.269 hatte ein synchronisierter Skill nur seinen Kurznamen.

Der andere Befehl kann einer dieser sein:

* Ein integrierter Befehl oder ein [gebündelter Skill](#bundled-skills), einschließlich eines, der in Ihrer Sitzung nicht verfügbar ist, zum Beispiel nachdem Sie gebündelte Skills ausschalten
* Ein Skill auf einer beliebigen [lokalen Ebene](#where-skills-live) oder eine Datei in `.claude/commands/`
* Ein Plugin-Skill
* Ein [MCP-Prompt](/docs/de/mcp#use-mcp-prompts-as-commands)

Claude Code kennzeichnet synchronisierte Skills, damit Sie sehen können, woher sie kommen. Das `/skills`-Menü und `/context` gruppieren synchronisierte Skills unter `claude.ai sync`, und das `/`-Befehlsmenü kennzeichnet sie als von claude.ai kommend.

Beim Vergleich von Namen ignoriert Claude Code Groß-/Kleinschreibung, Abstände und unsichtbare Zeichen und behandelt Kompatibilitätsformen wie Vollbreitenbuchstaben und Bindestrich-Varianten als ihre einfachen Äquivalente. Zum Beispiel zählt ein synchronisierter Skill namens `Commit` und ein lokaler Skill namens `commit` als derselbe Name, daher führt `/commit` weiterhin Ihren lokalen Skill aus.

Ein Name, der sich nur durch einen ähnlich aussehenden Buchstaben aus einem anderen Alphabet unterscheidet, zählt als ein anderer Name, und das `claude.ai sync`-Label ist, wie Sie die beiden unterscheiden. Diese Überprüfungen und Labels erfordern Claude Code v2.1.228 oder später.

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Wie Claude Code die Frontmatter eines synchronisierten Skills handhabt
</h4>

Claude Code wendet zwei Regeln auf die Frontmatter eines synchronisierten Skills an:

* Claude Code respektiert die Frontmatter in jeder Art von Sitzung, daher geht eine `allowed-tools`-Gewährung durch den normalen [Berechtigungsfluss](/docs/de/permissions).
* Claude Code bereinigt den Anzeigetext, den der Skill liefert, wie seine Beschreibung. Es entfernt Steuerzeichen und in Text, der Claude erreicht, wie die Beschreibung, maskiert es auch spitzklammern, damit der Text nicht Claude Codes interne Formatierung imitieren kann. Diese Bereinigung erfordert Claude Code v2.1.228 oder später.

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Wie Claude Code den Body eines synchronisierten Skills handhabt
</h4>

Was Claude Code mit dem Body eines synchronisierten Skills tut, hängt davon ab, wo die Sitzung läuft:

* In einer Cloud-Sitzung behält der Body das Verhalten, das ein lokaler Skill hat, da die Sitzung in einem isolierten Container läuft.
* In einer Cowork-Sitzung auf Ihrem Desktop behält der Body das Verhalten, das ein lokaler Skill hat, außer dass Claude Code jede `!`-Befehlszeile durch den [`disableSkillShellExecution`-Platzhalter](#inject-dynamic-context) ersetzt, wie es für jeden Skill tut, den Sie dort liefern.
* In jeder anderen Sitzung auf Ihrer Maschine führt Claude Code keine [`!`-Befehle](#inject-dynamic-context) aus, hängt nicht die Dateien an, die `@`-Referenzen benennen, wie es für einen lokalen Skill tut, und ersetzt nicht die Platzhalter `${CLAUDE_PROJECT_DIR}` und `${CLAUDE_SESSION_ID}`, daher erreichen die `@`-Referenzen und beide Platzhalter Claude als wörtlicher Text. Eine `!`-Befehlszeile erreicht Claude auch als wörtlicher Text oder als dieser Platzhalter, wenn `disableSkillShellExecution` aktiviert ist. Diese Handhabung erfordert Claude Code v2.1.228 oder später.

<h3 id="live-change-detection">
  Bearbeiten Sie einen Skill während einer Sitzung
</h3>

Claude Code überwacht Skill-Verzeichnisse auf Dateiänderungen, außer im [Bare Mode](/docs/de/headless#start-faster-with-bare-mode). Wenn Sie einen Skill unter `~/.claude/skills/`, dem Projekt `.claude/skills/` oder einem `.claude/skills/` in einem `--add-dir`-Verzeichnis hinzufügen, bearbeiten oder entfernen, nimmt Claude Code die Änderung in der aktuellen Sitzung auf, ohne einen Neustart. Wenn Sie ein Top-Level-Skills-Verzeichnis erstellen, das beim Start der Sitzung nicht vorhanden war, starten Sie Claude Code neu, damit es das neue Verzeichnis überwachen kann.

Die Live-Änderungserkennung deckt nur `SKILL.md`-Text ab. Für einen Skill-Ordner, der auch ein [Plugin](/docs/de/plugins/loading#plugins-shared-through-a-repository) ist, benötigen Änderungen an `hooks/`, `.mcp.json`, `agents/` und `output-styles/` `/reload-plugins`, um wirksam zu werden.

<h3 id="remove-a-skill">
  Entfernen Sie einen Skill
</h3>

Wie Sie einen Skill entfernen, hängt davon ab, woher er kommt:

* **Persönlicher oder Projekt-Skill**: Löschen Sie das Verzeichnis des Skills, `~/.claude/skills/<skill-name>/` oder `.claude/skills/<skill-name>/`. Claude Code [entfernt ihn aus `/skills` in der aktuellen Sitzung](#live-change-detection); Inhalte, die Claude Code bereits daraus geladen hat, folgen dem [Skill-Content-Lebenszyklus](#skill-content-lifecycle).
* **Enterprise-Skill**: Ein Administrator löscht das Verzeichnis des Skills aus `.claude/skills/` im [Verzeichnis für verwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms), zum Beispiel `/etc/claude-code/.claude/skills/<skill-name>/` auf Linux.
* **Plugin-Skill**: Deaktivieren oder deinstallieren Sie das Plugin, das ihn bereitstellt, aus dem `/plugin`-Menü oder mit `/plugin uninstall <plugin-name>@<marketplace-name>`. Claude Code entlädt die Skills des Plugins, wenn [die Änderung angewendet wird](/docs/de/plugins/cli-reference#reload-plugins) oder wenn Sie neu starten.
* **Skill, der von claude.ai synchronisiert wird**: Schalten Sie den Skill für Ihr claude.ai-Konto aus, an derselben Stelle, an der Sie ihn [aktiviert haben](#skills-in-cowork-and-cloud-sessions). Claude Code entfernt ihn aus `~/.claude/skills/synced/` beim nächsten Mal, wenn es [Ihre Skills synchronisiert](#where-synced-skills-load). Wenn Sie das Verzeichnis stattdessen von Hand löschen, lädt die nächste Synchronisierung es erneut herunter, während der Skill auf claude.ai aktiviert bleibt.
* **Gebündelter Skill**: Setzen Sie [`disableBundledSkills`](#bundled-skills) auf `true`, um gebündelte Skills auszuschalten, oder setzen Sie einen Skill auf `"off"` in [`skillOverrides`](#override-skill-visibility-from-settings), um ihn auszublenden.

Um einen persönlichen oder Projekt-Skill zu behalten, aber Claude daran zu hindern, ihn von selbst aufzurufen, setzen Sie [`disable-model-invocation: true`](#control-who-invokes-a-skill) in seiner Frontmatter oder `"user-invocable-only"` in [`skillOverrides`](#override-skill-visibility-from-settings), wenn Sie die Datei nicht bearbeiten möchten.

<h2 id="configure-skills">
  Fähigkeiten konfigurieren
</h2>

Fähigkeiten werden durch YAML-Frontmatter am Anfang von `SKILL.md` und den darauffolgenden Markdown-Inhalt konfiguriert.

<h3 id="types-of-skill-content">
  Arten von Fähigkeitsinhalten
</h3>

Fähigkeitsdateien können beliebige Anweisungen enthalten, aber das Nachdenken darüber, wie Sie sie aufrufen möchten, hilft bei der Entscheidung, was Sie einbeziehen:

**Referenzinhalte** fügen Wissen hinzu, das Claude auf Ihre aktuelle Arbeit anwendet. Konventionen, Muster, Stilrichtlinien, Domänenwissen. Dieser Inhalt wird inline ausgeführt, sodass Claude ihn zusammen mit Ihrem Gesprächskontext verwenden kann.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Aufgabeninhalte** geben Claude Schritt-für-Schritt-Anweisungen für eine bestimmte Aktion, wie Bereitstellungen, Commits oder Code-Generierung. Dies sind oft Aktionen, die Sie direkt mit `/skill-name` aufrufen möchten, anstatt Claude entscheiden zu lassen, wann sie ausgeführt werden. Fügen Sie `disable-model-invocation: true` hinzu, um zu verhindern, dass Claude sie automatisch auslöst. Das folgende Beispiel fügt `context: fork` hinzu, das die Fähigkeit in ihrem eigenen Subagent-Kontext ausführt; siehe [Fähigkeiten in einem Subagent ausführen](#run-skills-in-a-subagent).

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

Halten Sie den Text selbst prägnant. Sobald eine Fähigkeit geladen ist, bleibt ihr Inhalt [über Turns hinweg im Kontext](#skill-content-lifecycle), sodass jede Zeile wiederkehrende Token-Kosten verursacht. Geben Sie an, was zu tun ist, anstatt zu erzählen, wie oder warum, und wenden Sie denselben Prägnanztest an, den Sie für [CLAUDE.md-Inhalte](/docs/de/best-practices#write-an-effective-claude-md) verwenden würden.

<h3 id="frontmatter-reference">
  Frontmatter-Referenz
</h3>

Konfigurieren Sie eine Fähigkeit mit YAML-[Frontmatter](/docs/de/glossary#frontmatter) zwischen `---`-Markierungen am Anfang von `SKILL.md`, und schreiben Sie die Anweisungen der Fähigkeit als Markdown nach dem schließenden `---`. Feldnamen verwenden Kleinbuchstaben-Wörter, die durch Bindestriche getrennt sind, außer `when_to_use`. Eine [Befehlsdatei](#where-skills-live) in `.claude/commands/` akzeptiert die gleichen Felder außer `name` und `paths`. Dieses Beispiel setzt vier Felder:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Alle Felder sind optional. Nur `description` wird empfohlen, damit Claude weiß, wann die Fähigkeit verwendet werden soll. Ein Feldname muss genau mit der Tabelle übereinstimmen, Bindestriche eingeschlossen: Claude Code ignoriert ein Feld, das es nicht erkennt, ohne einen Fehler zu melden.

Claude Code liest das Frontmatter nur, wenn die öffnende `---` die erste Zeile der Datei ist. Andernfalls behandelt es die gesamte Datei, einschließlich `---`-Markierungen, als Fähigkeitsinhalt. Wenn das YAML zwischen den Markierungen nicht analysiert wird, wird die Fähigkeit immer noch geladen, ohne dass Felder gesetzt werden; siehe [Fähigkeit wird nicht ausgelöst](#skill-not-triggering), um den Fehler zu finden und zu beheben.

Boolesche Felder akzeptieren `yes`, `no`, `on`, `off`, `1` und `0` in beliebiger Schreibweise, zusätzlich zu `true` und `false`. Vor v2.1.218 erkannte Claude Code nur `true` und `false`.

| Feld                       | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Nein         | Anzeigename, der in Fähigkeitsauflistungen angezeigt wird. Standardmäßig der Verzeichnisname. Siehe [Wie eine Fähigkeit ihren Befehlsnamen erhält](#how-a-skill-gets-its-command-name), um zu sehen, wie das Feld mit dem Namen interagiert, den Sie eingeben, um die Fähigkeit aufzurufen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `description`              | Empfohlen    | Was die Fähigkeit tut und wann sie verwendet werden soll. Claude verwendet dies, um zu entscheiden, wann die Fähigkeit angewendet werden soll. Wenn weggelassen, wird die erste nicht leere Zeile des Markdown-Inhalts verwendet. Setzen Sie den wichtigsten Anwendungsfall zuerst: Der kombinierte `description`- und `when_to_use`-Text wird in der Fähigkeitsauflistung auf 1.536 Zeichen gekürzt, um die Kontextnutzung zu reduzieren.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `when_to_use`              | Nein         | Zusätzlicher Kontext für den Zeitpunkt, zu dem Claude die Fähigkeit aufrufen sollte, z. B. Trigger-Phrasen oder Beispielanfragen. An `description` in der Fähigkeitsauflistung angehängt und zählt zur 1.536-Zeichen-Obergrenze.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `argument-hint`            | Nein         | Hinweis, der während der Autovervollständigung angezeigt wird, um erwartete Argumente anzuzeigen. Beispiel: `[issue-number]` oder `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `arguments`                | Nein         | Benannte Positionsargumente für [`$name`-Substitution](#available-string-substitutions) im Fähigkeitsinhalt. Akzeptiert eine durch Leerzeichen getrennte Zeichenkette oder eine YAML-Liste. Namen werden in Reihenfolge Argumentpositionen zugeordnet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `disable-model-invocation` | Nein         | Setzen Sie auf `true`, um zu verhindern, dass Claude diese Fähigkeit automatisch lädt. Verwenden Sie für Workflows, die Sie manuell mit `/name` auslösen möchten. Verhindert auch, dass die Fähigkeit [in Subagents vorgeladen wird](/docs/de/sub-agents#preload-skills-into-subagents). Ab v2.1.196 verhindert dies auch, dass die Fähigkeit ausgeführt wird, wenn eine [geplante Aufgabe](/docs/de/scheduled-tasks) mit der Fähigkeit als Eingabeaufforderung ausgelöst wird. Standard: `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `user-invocable`           | Nein         | Setzen Sie auf `false`, wenn nur Claude die Fähigkeit aufrufen sollte: Claude Code blendet sie aus dem `/`-Menü aus und führt sie nicht aus, wenn Sie `/name` eingeben. Verwenden Sie für Hintergrundwissen, das Benutzer nicht direkt aufrufen sollten. Standard: `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowed-tools`            | Nein         | Tools, die Claude ohne Genehmigung während des Turns verwenden kann, der diese Fähigkeit aufruft. Die Genehmigung wird gelöscht, wenn Sie Ihre nächste Nachricht senden. Akzeptiert eine durch Leerzeichen oder Komma getrennte Zeichenkette oder eine YAML-Liste. Siehe [Tools für eine Fähigkeit vorab genehmigen](#pre-approve-tools-for-a-skill).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `disallowed-tools`         | Nein         | Tools, die aus Claudes verfügbarem Pool entfernt werden, während diese Fähigkeit aktiv ist. Verwenden Sie für autonome Fähigkeiten, die niemals bestimmte Tools aufrufen sollten, z. B. `AskUserQuestion` für eine Hintergrundschleife. Akzeptiert eine durch Leerzeichen oder Komma getrennte Zeichenkette oder eine YAML-Liste. Die Einschränkung wird gelöscht, wenn Sie Ihre nächste Nachricht senden. Wie Ablehnungsregeln kann das Feld [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior) nicht entfernen, während ein anderes Tool verfügbar bleibt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `model`                    | Nein         | Modell, das verwendet werden soll, wenn diese Fähigkeit aktiv ist. Die Außerkraftsetzung gilt für den Rest des aktuellen Turns und wird nicht in den Einstellungen gespeichert; das Sitzungsmodell wird bei Ihrer nächsten Eingabeaufforderung fortgesetzt. Akzeptiert die gleichen Werte wie [`/model`](/docs/de/model-config), oder `inherit`, um das aktive Modell beizubehalten. Ein Wert, der durch die [`availableModels`](/docs/de/model-config#restrict-model-selection)-Zulassungsliste Ihrer Organisation ausgeschlossen ist, wird nicht verwendet und die Sitzung behält ihr aktuelles Modell. Im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) und im [Plan-Modus, während der Klassifizierer Befehle überprüft](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode), wird ein Modell, das der Auto-Modus nicht unterstützt, auch nicht verwendet, und die Sitzung behält ihr aktuelles Modell. Mit `context: fork` setzt der Wert stattdessen das [Modell des verzweigten Subagents](#run-skills-in-a-subagent) und ein ausgeschlossener Wert folgt den [gleichen Regeln wie eine Subagent-Modellüberschreibung](/docs/de/model-config#restrict-model-selection). |
| `effort`                   | Nein         | [Aufwandsstufe](/docs/de/model-config#adjust-effort-level), wenn diese Fähigkeit aktiv ist. Überschreibt die Sitzungsaufwandsstufe. Standard: erbt von Sitzung. Optionen: `low`, `medium`, `high`, `xhigh`, `max`; verfügbare Stufen hängen vom Modell ab.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `context`                  | Nein         | Setzen Sie auf `fork`, um in einem verzweigten Subagent-Kontext ausgeführt zu werden. Siehe [Fähigkeiten in einem Subagent ausführen](#run-skills-in-a-subagent).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `agent`                    | Nein         | Welcher Subagent-Typ verwendet werden soll, wenn `context: fork` gesetzt ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `background`               | Nein         | Gilt nur mit `context: fork`. Setzen Sie auf `false`, um auf das Ergebnis des verzweigten Subagents im Turn zu warten, der die Fähigkeit aufgerufen hat, anstatt [es im Hintergrund auszuführen](#run-skills-in-a-subagent). Standard: `true`. Erfordert Claude Code v2.1.218 oder später.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `hooks`                    | Nein         | Hooks, die Claude Code registriert, wenn die Fähigkeit aufgerufen wird, und die für den Rest der Sitzung weiterhin ausgeführt werden. Siehe [Hooks in Fähigkeiten und Agenten](/docs/de/hooks#hooks-in-skills-and-agents) für das Konfigurationsformat und die `once`-Option.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `paths`                    | Nein         | Glob-Muster, die einschränken, wann diese Fähigkeit aktiviert wird. Akzeptiert eine durch Komma getrennte Zeichenkette oder eine YAML-Liste. Wenn gesetzt, lädt Claude die Fähigkeit automatisch nur, wenn mit Dateien arbeitet, die den Mustern entsprechen. Verwendet das gleiche Format wie [pfadspezifische Regeln](/docs/de/memory#path-specific-rules).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `shell`                    | Nein         | Shell, die für `` !`command` `` und ` ```! ` Blöcke in dieser Fähigkeit verwendet werden soll. Akzeptiert `bash` (Standard) oder `powershell`. Das Setzen von `powershell` führt Inline-Shell-Befehle über PowerShell aus, wenn das [PowerShell-Tool](/de/tools-reference#powershell-tool) aktiviert ist: Es ist standardmäßig unter Windows ohne Git Bash aktiviert, standardmäßig mit Git Bash für claude.ai und Console-Konten aktiviert, und benötigt `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` in Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry-Sitzungen sowie auf macOS, Linux und WSL. Setzen Sie es auf `0`, um das Tool auszuschalten.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `metadata`                 | Nein         | Freie YAML-Zuordnung für Ihre eigenen Schlüssel-Wert-Daten, z. B. Berechtigung oder Katalogfelder, die von Ihrem eigenen Tooling aus `SKILL.md` gelesen werden. Claude Code handelt nicht nach ihrem Inhalt und verwirft einen Wert, der keine Zuordnung ist. Verwenden Sie keine Frontmatter-Feldnamen wie `paths` als Schlüssel erneut.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `license`                  | Nein         | Lizenz, die die Fähigkeit abdeckt. Teil der [Agent Skills](https://agentskills.io)-Spezifikation; siehe [Fähigkeits-Frontmatter außerhalb von Claude Code verwenden](#using-skill-frontmatter-outside-claude-code). Claude Code akzeptiert das Feld, handelt aber nicht danach.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `compatibility`            | Nein         | Umgebungsanforderungen für die Fähigkeit, z. B. beabsichtigte Produkte oder Systemvoraussetzungen, wie in der [Agent Skills](https://agentskills.io)-Spezifikation definiert; siehe [Fähigkeits-Frontmatter außerhalb von Claude Code verwenden](#using-skill-frontmatter-outside-claude-code). Akzeptiert eine Zeichenkette von bis zu 500 Zeichen. Claude Code akzeptiert das Feld, handelt aber nicht danach.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Fähigkeits-Frontmatter außerhalb von Claude Code verwenden
</h4>

Claude Code akzeptiert jedes Feld in der obigen Tabelle. Außerhalb von Claude Code können Sie nur die Felder in der [Agent Skills](https://agentskills.io)-Spezifikation verwenden:

| Verteilungspfad                                                                                                                                  | Frontmatter-Felder, die Sie verwenden können                                   |
| :----------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Claude Code-Fähigkeiten auf [beliebiger Ebene](#where-skills-live), einschließlich [Plugin](/docs/de/plugins/overview)-Fähigkeiten                    | Jedes Feld in der obigen Tabelle                                               |
| claude.ai-Fähigkeits-Uploads, die Skills-API und Verpackung mit `package_skill.py` aus [anthropics/skills](https://github.com/anthropics/skills) | `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools` |

Wenn Sie eine persönliche Fähigkeit für Ihr claude.ai-Konto aktivieren, um sie beispielsweise in [Cowork- und Cloud-Sitzungen](#skills-in-cowork-and-cloud-sessions) und Routinen zu verwenden, laden Sie sie auf claude.ai hoch, sodass die gleichen Regeln gelten.

Wenn Sie ein Feld einbeziehen, das die Spezifikation nicht zulässt, schlägt die Verpackung oder der Upload mit einem schwerwiegenden Fehler fehl, anstatt das Feld zu ignorieren:

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

Das Einschränken des Frontmatters auf die sechs Felder der Spezifikation vermeidet den obigen Fehler „unexpected-key". Die [Agent Skills-Spezifikation](https://agentskills.io) und die [Skills-API-Anforderungen](https://docs.claude.com/en/api/skills-guide) definieren alles andere, das diese Pfade validieren. Claude Code-spezifische Body-Funktionen, wie [dynamische Kontexteinspritzung](#inject-dynamic-context), funktionieren nicht in claude.ai-Chat oder über die API. Claude Code akzeptiert alle sechs Felder, sodass Frontmatter, das der Spezifikation folgt, ohne Änderungen in Claude Code geladen wird.

<h4 id="how-a-skill-gets-its-command-name">
  Wie eine Fähigkeit ihren Befehlsnamen erhält
</h4>

Der Befehl, den Sie eingeben, um eine Fähigkeit aufzurufen, kommt von dem Ort, an dem die Fähigkeitsdatei lebt, und für Plugin-Fähigkeiten auch vom Frontmatter-Feld `name`. In einer persönlichen oder Projektfähigkeit setzt `name` nur das Anzeigelabel, das in Fähigkeitsauflistungen angezeigt wird, und der Befehl kommt immer noch vom Verzeichnisnamen. In einer Plugin-Fähigkeit setzt `name` das letzte Segment des Befehls und das Plugin-Präfix bleibt bestehen.

Die folgende Tabelle zeigt, woher der Befehlsname für jedes Layout kommt:

| Fähigkeitsort                                                                                                             | Befehlsnamenquelle                                                                                                 | Beispiel                                                                                                                                     |
| :------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| Fähigkeitsverzeichnis unter `~/.claude/skills/` oder `.claude/skills/`                                                    | Verzeichnisname                                                                                                    | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                                 |
| [Verschachteltes](#where-skills-live) `.claude/skills/`-Verzeichnis, wenn der Name mit einer anderen Fähigkeit kollidiert | Unterverzzeichnisspfad relativ zum Arbeitsverzeichnis, dann der Fähigkeitsverzeichnisname                          | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                               |
| Datei unter `.claude/commands/`                                                                                           | Dateiname ohne Erweiterung                                                                                         | `.claude/commands/deploy.md` → `/deploy`                                                                                                     |
| Datei in einem Unterverzeichnis von `.claude/commands/`                                                                   | Unterverzzeichnisspfad relativ zu `commands/` mit jedem `/` ersetzt durch `:`, dann der Dateiname ohne Erweiterung | `.claude/commands/frontend/component.md` → `/frontend:component`                                                                             |
| Plugin `skills/` Unterverzeichnis                                                                                         | Frontmatter `name` oder der Verzeichnisname, mit Namespace durch Plugin                                            | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`, oder `/my-plugin:fancy` mit `name: fancy`                                          |
| Plugin-Root `SKILL.md`                                                                                                    | Frontmatter `name`, mit dem Plugin-Verzeichnisnamen als Fallback                                                   | `my-plugin/SKILL.md` mit `name: review` → `/my-plugin:review`. Siehe [eine einzelne Fähigkeit im Plugin-Root](/docs/de/plugins/components#skills) |
| Fähigkeit [synchronisiert von claude.ai](#how-synced-skills-behave)                                                       | Der Name der Fähigkeit auf Ihrem claude.ai-Konto, mit dem Präfix `anthropic-skills:`                               | Kontofähigkeit `deploy` → `/anthropic-skills:deploy`, oder `/deploy`, wenn kein anderer Befehl diesen Namen verwendet                        |

In einer Plugin-Fähigkeit ersetzt das Frontmatter-Feld `name` den Verzeichnisnamen im letzten Segment des Befehls, sodass `my-plugin/skills/review/SKILL.md` mit `name: fancy` zu `/my-plugin:fancy` wird. Der bloße `/fancy` ruft die Fähigkeit auch auf, es sei denn, ein anderer Befehl verwendet bereits diesen Namen. Wenn der `name`, den Sie schreiben, bereits mit dem eigenen Präfix des Plugins beginnt, fügt Claude Code das Präfix nicht erneut auf v2.1.246 oder später hinzu. Zum Beispiel wird `name: my-plugin:fancy` immer noch zu `/my-plugin:fancy`. Von v2.1.216 bis v2.1.245 verdoppelte Claude Code das Präfix, wenn der `name` es bereits trug.

In [nicht-interaktiven Sitzungen](/docs/de/headless) sind die Namen `help` und `feedback` nicht für ihre nur-Terminal-Befehle reserviert, sodass eine Plugin-Fähigkeit mit einem dieser Namen ihren bloßen Befehl dort behält. Jeder andere nur-Terminal-Befehl, wie `/login`, bleibt reserviert, obwohl der Befehl in diesen Sitzungen nicht ausgeführt werden kann.

Für eine Plugin-Root `SKILL.md` gibt es kein Fähigkeitsverzeichnis, aus dem der Name genommen werden kann, sodass `name` das ganze letzte Segment liefert. Ohne ein `name`-Feld fällt Claude Code auf den Plugin-Verzeichnisnamen zurück.

<h4 id="available-string-substitutions">
  Verfügbare Zeichenkettensubstitutionen
</h4>

Fähigkeiten unterstützen Zeichenkettensubstitution für dynamische Werte im Fähigkeitsinhalt:

| Variable                | Beschreibung                                                                                                                                                                                                                                                                                                                                                  |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$ARGUMENTS`            | Alle Argumente, die beim Aufrufen der Fähigkeit übergeben werden. Wenn kein Platzhalter ein Argument empfängt, hängt Claude Code `ARGUMENTS: <value>` am Ende des Fähigkeitsinhalts an. Siehe [Argumente an Fähigkeiten übergeben](#pass-arguments-to-skills).                                                                                                |
| `$ARGUMENTS[N]`         | Greifen Sie auf ein bestimmtes Argument nach 0-basiertem Index zu, z. B. `$ARGUMENTS[0]` für das erste Argument.                                                                                                                                                                                                                                              |
| `$N`                    | Kurzform für `$ARGUMENTS[N]`, z. B. `$0` für das erste Argument oder `$1` für das zweite.                                                                                                                                                                                                                                                                     |
| `$name`                 | Benanntes Argument, das in der [`arguments`](#frontmatter-reference)-Frontmatter-Liste deklariert ist. Namen werden in Reihenfolge Positionen zugeordnet, sodass mit `arguments: [issue, branch]` der Platzhalter `$issue` zum ersten Argument und `$branch` zum zweiten expandiert.                                                                          |
| `${CLAUDE_SESSION_ID}`  | Die aktuelle Sitzungs-ID. Nützlich für Protokollierung, Erstellen von sitzungsspezifischen Dateien oder Korrelation von Fähigkeitsausgabe mit Sitzungen.                                                                                                                                                                                                      |
| `${CLAUDE_EFFORT}`      | Die aktuelle Aufwandsstufe: `low`, `medium`, `high`, `xhigh` oder `max`. Ultracode ist keine separate Stufe und wird als `xhigh` gemeldet. Verwenden Sie dies, um Fähigkeitsanweisungen an die aktive Aufwandseinstellung anzupassen.                                                                                                                         |
| `${CLAUDE_SKILL_DIR}`   | Das Verzeichnis, das die `SKILL.md`-Datei der Fähigkeit enthält. Für Plugin-Fähigkeiten ist dies das Fähigkeitsunterverzeichnis innerhalb des Plugins, nicht die Plugin-Root. Verwenden Sie dies in Bash-Injektionsbefehlen, um auf Skripte oder Dateien zu verweisen, die mit der Fähigkeit gebündelt sind, unabhängig vom aktuellen Arbeitsverzeichnis.     |
| `${CLAUDE_PROJECT_DIR}` | Das Projektroot-Verzeichnis. Dies ist der gleiche Pfad, den [Hooks](/docs/de/hooks#reference-scripts-by-path) und MCP-Server als `CLAUDE_PROJECT_DIR` erhalten. Verwenden Sie dies, um auf projektlokale Skripte oder Dateien zu verweisen, z. B. `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, unabhängig davon, wo die Fähigkeit installiert ist.             |
| `${CLAUDE_PLUGIN_ROOT}` | Das Installationsverzeichnis des Plugins. Wird nur in Plugin-Fähigkeiten ersetzt. Verwenden Sie dies, um auf Skripte oder Dateien zu verweisen, die überall im Plugin gebündelt sind, einschließlich Ressourcen, die zwischen den Plugin-Fähigkeiten geteilt werden. Siehe [Plugin-Umgebungsvariablen](/docs/de/plugins/manifest-reference#environment-variables). |
| `${CLAUDE_PLUGIN_DATA}` | Das [persistente Datenverzeichnis](/docs/de/plugins/components#path-variables-and-persistent-data) des Plugins, das Plugin-Updates überlebt. Wird nur in Plugin-Fähigkeiten ersetzt. Verwenden Sie dies, um auf installierte Abhängigkeiten, generierte Dateien oder Caches zu verweisen, die ein Update überleben müssen.                                         |

Claude Code ersetzt `${CLAUDE_SKILL_DIR}` und `${CLAUDE_PROJECT_DIR}` an zwei Stellen: im Markdown-Inhalt der Fähigkeit und in Bash-Regeln im [`allowed-tools`](#frontmatter-reference)-Frontmatter. In einer Plugin-Fähigkeit ersetzt Claude Code `${CLAUDE_PLUGIN_ROOT}` und `${CLAUDE_PLUGIN_DATA}` an den gleichen zwei Stellen. Die Verwendung der gleichen Variable an beiden Stellen ermöglicht es einer Fähigkeit, ein gebündeltes Skript ohne Genehmigungsaufforderung auszuführen. Die folgende Fähigkeit zeigt das Muster:

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

Wenn diese Fähigkeit unter `~/.claude/skills/render-chart/` installiert ist, expandieren beide Vorkommen von `${CLAUDE_SKILL_DIR}` zu diesem Verzeichnis. Die `allowed-tools`-Regel stimmt dann mit dem genauen Befehl überein, den der Fähigkeitstext Claude ausführen sagt, sodass das Skript ohne Aufforderung ausgeführt wird.

Die `${CLAUDE_PROJECT_DIR}`-Substitution erfordert Claude Code v2.1.196 oder später.

Indizierte Argumente verwenden Shell-ähnliche Anführungszeichen, sodass mehrteilige Werte in Anführungszeichen einschließen, um sie als einzelnes Argument zu übergeben. Zum Beispiel macht `/my-skill "hello world" second` `$0` zu `hello world` und `$1` zu `second`. Der `$ARGUMENTS`-Platzhalter expandiert immer zur vollständigen Argumentzeichenkette wie eingegeben.

Ein indizierter Platzhalter ohne entsprechendes Argument, z. B. `$2`, wenn nur ein Argument übergeben wurde, bleibt im Inhalt unverändert. Ein benannter Platzhalter aus dem [`arguments`](#frontmatter-reference)-Frontmatter ohne übereinstimmendes Argument expandiert zu einer leeren Zeichenkette.

Wenn Sie einen Argumentwert übergeben, der selbst Text wie `$1` oder `$ARGUMENTS` enthält, fügt Claude Code ihn als Literaltext ein und expandiert ihn nicht. Zum Beispiel, wenn der Fähigkeitstext `Summarize $0` enthält und Sie `/summarize "$ARGUMENTS from yesterday"` ausführen, empfängt Claude `Summarize $ARGUMENTS from yesterday`. Claude Code ersetzt immer noch `${CLAUDE_*}`-Variablen wie `${CLAUDE_SKILL_DIR}`, nachdem es die Argumente eingefügt hat.

Um ein Literal `$` vor einer Ziffer, `ARGUMENTS` oder einem deklarierten Argumentnamen einzubeziehen, z. B. `$1.00` in Prosa, maskieren Sie es mit einem Backslash: `\$1.00`. Ein Backslash vor jedem anderen `$` bleibt unverändert. Nur ein einzelner Backslash direkt vor dem Token maskiert ihn. Ein verdoppelter Backslash wie `\\$1` lässt beide Backslashes an Ort und Stelle, und `$1` expandiert immer noch zum Argumentwert. Die Backslash-Maskierung deckt nur diese Argumentplatzhalter ab. Ein Backslash verhindert nicht die Substitution einer `${CLAUDE_*}`-Variable, wo die Variable angewendet wird.

**Beispiel mit Substitutionen:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Unterstützende Dateien hinzufügen
</h3>

Fähigkeiten können mehrere Dateien in ihrem Verzeichnis enthalten. Dies hält `SKILL.md` auf das Wesentliche konzentriert, während Claude auf detailliertes Referenzmaterial nur bei Bedarf zugreifen kann. Große Referenzdokumente, API-Spezifikationen oder Beispielsammlungen müssen nicht jedes Mal geladen werden, wenn die Fähigkeit ausgeführt wird.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Verweisen Sie auf unterstützende Dateien aus `SKILL.md`, damit Claude weiß, was jede Datei enthält und wann sie geladen werden soll:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Halten Sie `SKILL.md` unter 500 Zeilen. Verschieben Sie detailliertes Referenzmaterial in separate Dateien.</Tip>

<h3 id="control-who-invokes-a-skill">
  Kontrollieren Sie, wer eine Fähigkeit aufruft
</h3>

Standardmäßig können Sie und Claude jede Fähigkeit aufrufen. Sie können `/skill-name` eingeben, um sie direkt aufzurufen, und Claude kann sie automatisch laden, wenn sie für Ihr Gespräch relevant ist. Zwei Frontmatter-Felder ermöglichen es Ihnen, dies einzuschränken:

* **`disable-model-invocation: true`**: Nur Sie können die Fähigkeit aufrufen. Verwenden Sie dies für Workflows mit Nebenwirkungen oder die Sie zeitlich kontrollieren möchten, wie `/commit`, `/deploy` oder `/send-slack-message`. Sie möchten nicht, dass Claude bereitstellt, weil Ihr Code bereit aussieht.

* **`user-invocable: false`**: Nur Claude kann die Fähigkeit aufrufen. Verwenden Sie dies für Hintergrundwissen, das nicht als Befehl umsetzbar ist. Eine `legacy-system-context`-Fähigkeit erklärt, wie ein altes System funktioniert. Claude sollte dies kennen, wenn relevant, aber `/legacy-system-context` ist keine aussagekräftige Aktion für Benutzer.

Dieses Beispiel erstellt eine Deploy-Fähigkeit, die nur Sie auslösen können. Wenn Sie `disable-model-invocation: true` setzen, kann Claude die Fähigkeit nicht automatisch ausführen:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Wenn Claude es trotzdem versucht, blockiert Claude Code den Aufruf und weist es an, die Deploy-Schritte nicht auf andere Weise zu reproduzieren, sodass Sie erwarten können, dass Claude vorschlägt, `/deploy` selbst auszuführen.

Hier ist, wie die beiden Felder Aufrufe und Kontextladung beeinflussen:

| Frontmatter                      | Sie können aufrufen | Claude kann aufrufen | Wann in Kontext geladen                                                                    |
| :------------------------------- | :------------------ | :------------------- | :----------------------------------------------------------------------------------------- |
| (Standard)                       | Ja                  | Ja                   | Beschreibung immer im Kontext, vollständige Fähigkeit wird beim Aufrufen geladen           |
| `disable-model-invocation: true` | Ja                  | Nein                 | Beschreibung nicht im Kontext, vollständige Fähigkeit wird beim Aufrufen durch Sie geladen |
| `user-invocable: false`          | Nein                | Ja                   | Beschreibung immer im Kontext, vollständige Fähigkeit wird beim Aufrufen geladen           |

<Note>
  In einer regulären Sitzung werden Fähigkeitsbeschreibungen in den Kontext geladen, damit Claude weiß, was verfügbar ist, aber vollständiger Fähigkeitsinhalt wird nur beim Aufrufen geladen. [Subagents mit vorgeladenen Fähigkeiten](/docs/de/sub-agents#preload-skills-into-subagents) funktionieren anders: Der vollständige Fähigkeitsinhalt wird beim Start eingespritzt.
</Note>

<h3 id="skill-content-lifecycle">
  Fähigkeitsinhalts-Lebenszyklus
</h3>

Wenn Sie oder Claude eine Fähigkeit aufrufen, tritt der gerenderte `SKILL.md`-Inhalt als einzelne Nachricht in das Gespräch ein und bleibt über spätere Turns hinweg bestehen. Diese Persistenz gilt für die Anweisungen der Fähigkeit, nicht ihre Berechtigungen: Eine [`allowed-tools`](#pre-approve-tools-for-a-skill)-Genehmigung wird gelöscht, wenn Sie Ihre nächste Nachricht senden. Claude Code liest die Fähigkeitsdatei bei späteren Turns nicht erneut, daher schreiben Sie Anleitung, die während einer Aufgabe gelten sollte, als stehende Anweisungen anstelle von einmaligen Schritten.

Wenn Claude eine Fähigkeit erneut aufruft, deren gerenderter Inhalt identisch mit der bereits im Kontext vorhandenen Kopie ist, fügt Claude Code einen kurzen Hinweis hinzu, dass die Fähigkeit bereits geladen ist, anstatt eine zweite Kopie des Inhalts zu erstellen. Wenn sich der gerenderte Inhalt unterscheidet, weil sich die Argumente geändert haben oder ein [dynamischer Kontext](#inject-dynamic-context)-Befehl neue Ausgabe erzeugt hat, hängt Claude Code den vollständigen Inhalt erneut an.

[Auto-Komprimierung](/docs/de/how-claude-code-works#when-context-fills-up) trägt aufgerufene Fähigkeiten innerhalb eines Token-Budgets vorwärts. Wenn das Gespräch zusammengefasst wird, um Kontext freizugeben, hängt Claude Code die neueste Aufrufinvokation jeder Fähigkeit nach der Zusammenfassung erneut an, wobei die ersten 5.000 Token jeder beibehalten werden. Erneut angehängte Fähigkeiten teilen sich ein kombiniertes Budget von 25.000 Token. Claude Code füllt dieses Budget beginnend mit der zuletzt aufgerufenen Fähigkeit, sodass ältere Fähigkeiten vollständig gelöscht werden können, nachdem die Komprimierung erfolgt ist, wenn Sie viele in einer Sitzung aufgerufen haben.

Wenn eine Fähigkeit nach der ersten Antwort zu beeinflussen zu stoppen scheint, ist der Inhalt normalerweise immer noch vorhanden und das Modell wählt andere Tools oder Ansätze. Stärken Sie die `description` und Anweisungen der Fähigkeit, damit das Modell sie weiterhin bevorzugt, oder verwenden Sie [Hooks](/docs/de/hooks), um Verhalten deterministisch zu erzwingen. Wenn die Fähigkeit groß ist oder Sie mehrere andere danach aufgerufen haben, rufen Sie sie nach der Komprimierung erneut auf, um den vollständigen Inhalt wiederherzustellen.

<h3 id="pre-approve-tools-for-a-skill">
  Tools für eine Fähigkeit vorab genehmigen
</h3>

Das Feld `allowed-tools` gewährt Genehmigung für die aufgelisteten Tools während des Turns, der die Fähigkeit aufruft, sodass Claude sie ohne Genehmigungsaufforderung verwenden kann. Die Genehmigung wird gelöscht, wenn Sie Ihre nächste Nachricht senden, obwohl der Fähigkeitsinhalt [im Kontext bleibt](#skill-content-lifecycle); das erneute Aufrufen der Fähigkeit wendet es für diesen Turn erneut an. Es schränkt nicht ein, welche Tools verfügbar sind: Jedes Tool bleibt aufrufbar, und Ihre [Berechtigungseinstellungen](/docs/de/permissions) regieren immer noch Tools, die nicht aufgelistet sind. Um Tools für die ganze Sitzung vorab zu genehmigen, anstatt einen einzelnen Turn, fügen Sie stattdessen Zulassungsregeln zu diesen Berechtigungseinstellungen hinzu.

Workspace-Vertrauen gatet dieses Feld nicht. Claude Code wendet die `allowed-tools` einer Projektfähigkeit an, wann immer Sie oder Claude die Fähigkeit aufrufen, einschließlich in einem `-p`-Lauf in einem Ordner, dem Sie nie vertraut haben. Eine Fähigkeit kann sich selbst breiten Tool-Zugriff gewähren, daher überprüfen Sie die `allowed-tools` von Fähigkeiten, die in ein Repository eingecheckt sind, bevor Sie Claude Code dort ausführen.

Diese Fähigkeit ermöglicht es Claude, Git-Befehle ohne Genehmigung pro Verwendung auszuführen, wann immer Sie sie aufrufen:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Um Tools aus Claudes verfügbarem Pool zu entfernen, während eine Fähigkeit aktiv ist, listen Sie sie im `disallowed-tools` im Frontmatter der Fähigkeit auf. Die Einschränkung wird gelöscht, wenn Sie Ihre nächste Nachricht senden. Wie Ablehnungsregeln kann das Feld [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior) nicht entfernen, während ein anderes Tool verfügbar bleibt. Um Tools über alle Fähigkeiten und Eingabeaufforderungen hinweg zu blockieren, fügen Sie Ablehnungsregeln in Ihren [Berechtigungseinstellungen](/docs/de/permissions) hinzu.

<h3 id="pass-arguments-to-skills">
  Argumente an Fähigkeiten übergeben
</h3>

Sowohl Sie als auch Claude können Argumente beim Aufrufen einer Fähigkeit übergeben. Argumente sind über den `$ARGUMENTS`-Platzhalter verfügbar.

Diese Fähigkeit behebt ein GitHub-Problem nach Nummer. Der `$ARGUMENTS`-Platzhalter wird durch alles ersetzt, das dem Fähigkeitsnamen folgt:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Wenn Sie `/fix-issue 123` ausführen, empfängt Claude „Fix GitHub issue 123 following our coding standards..."

Wenn Sie eine Fähigkeit mit Argumenten aufrufen, aber kein Platzhalter im Fähigkeitsinhalt ein Argument empfängt, hängt Claude Code `ARGUMENTS: <your input>` am Ende des Fähigkeitsinhalts an, damit Claude immer noch sieht, was Sie eingegeben haben. Ein Platzhalter ist `$ARGUMENTS`, eine indizierte Form wie `$1` oder ein benanntes Argument. Ein indizierter Platzhalter ohne Argument an seiner Position bleibt als Literaltext und zählt nicht als empfangen. Ein benannter Platzhalter zählt auch, wenn seine Position kein Argument hat, weil er zu einer leeren Zeichenkette expandiert.

Sie können auch mehrere Fähigkeiten am Anfang einer Nachricht stapeln. Das Eingeben von `/write-tests /fix-issue 123` lädt beide Fähigkeiten und übergibt den nachfolgenden Text `123` als `$ARGUMENTS` an jede von ihnen. Vor v2.1.199 wurde nur die erste Fähigkeit geladen und erhielt `/fix-issue 123` als Literalargumenttext.

Claude Code expandiert die erste Fähigkeit plus bis zu fünf weitere, die danach gestapelt sind. Die Expansion stoppt beim ersten Token, das keine Inline-Benutzer-aufgerufene Fähigkeit ist, sodass eine Fähigkeit, die als [verzweigter Subagent](#run-skills-in-a-subagent) ausgeführt wird, wie [`/code-review`](/docs/de/code-review#review-a-diff-locally), oder eine, deren Argumente selbst mit einem Schrägstrich-Befehl beginnen können, wie `/loop`, auch dort endet. Dieser Token und alles danach werden der Argumenttext für jede expandierte Fähigkeit. `/code-review` wird ab v2.1.218 als verzweigter Subagent ausgeführt; in früheren Versionen wurde es inline ausgeführt und gestapelt.

Um auf einzelne Argumente nach Position zuzugreifen, verwenden Sie `$ARGUMENTS[N]` oder die kürzere `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Das Ausführen von `/migrate-component SearchBar JavaScript TypeScript` ersetzt `$ARGUMENTS[0]` durch `SearchBar`, `$ARGUMENTS[1]` durch `JavaScript` und `$ARGUMENTS[2]` durch `TypeScript`. Die gleiche Fähigkeit mit der `$N`-Kurzform:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Erweiterte Muster
</h2>

<h3 id="inject-dynamic-context">
  Dynamischen Kontext injizieren
</h3>

Die Syntax `` !`<command>` `` führt Shell-Befehle aus, bevor der Skill-Inhalt an Claude gesendet wird. Die Befehlsausgabe ersetzt den Platzhalter, sodass Claude tatsächliche Daten erhält, nicht den Befehl selbst. Claude Code führt diese Befehle auf Ihrem Computer nicht aus, wenn der Skill [von Ihrem claude.ai-Konto synchronisiert wird](#how-claude-code-handles-the-body-of-a-synced-skill). Diese Einschränkung erfordert Claude Code v2.1.228 oder später.

Dieser Skill fasst einen Pull Request zusammen, indem er Live-PR-Daten mit der GitHub CLI abruft. Die Befehle `` !`gh pr diff` `` und andere werden zuerst ausgeführt, und ihre Ausgabe wird in den Prompt eingefügt:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

Die Ersetzung wird einmal über die ursprüngliche Datei ausgeführt. Die Befehlsausgabe wird als Klartext eingefügt und wird nicht erneut nach weiteren `` !`<command>` ``-Platzhaltern gescannt, sodass ein Befehl keinen Platzhalter für einen späteren Durchgang ausgeben kann.

Die Inline-Form wird nur erkannt, wenn `!` am Anfang einer Zeile oder unmittelbar nach Leerzeichen erscheint. Wenn `!` auf ein anderes Zeichen folgt, wie in `` KEY=!`cmd` ``, wird der Platzhalter als Literaltext belassen und der Befehl wird nicht ausgeführt.

Verwenden Sie für mehrzeilige Befehle einen eingezäunten Code-Block, der mit ` ```! ` statt der Inline-Form geöffnet wird:

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

Um dieses Verhalten für Skills und benutzerdefinierte Befehle aus Benutzer-, Projekt-, Plugin- oder [zusätzlichen Verzeichnisquellen](#skills-from-additional-directories) zu deaktivieren, setzen Sie `"disableSkillShellExecution": true` in [settings](/docs/de/settings). Jeder Befehl wird durch `[shell command execution disabled by policy]` ersetzt, anstatt ausgeführt zu werden. Gebündelte und verwaltete Skills sind nicht betroffen. Diese Einstellung ist am nützlichsten in [verwalteten Einstellungen](/docs/de/managed-settings), wo Benutzer sie nicht überschreiben können.

Claude Code führt diese Befehle auf Ihrem Computer niemals aus, wenn sie in Skills [von Ihrem claude.ai-Konto synchronisiert werden](#how-synced-skills-behave), unabhängig von dieser Einstellung. Diese Einschränkung erfordert Claude Code v2.1.228 oder später. [Wie Claude Code den Text eines synchronisierten Skills verarbeitet](#how-claude-code-handles-the-body-of-a-synced-skill) sagt, was Claude anstelle des Befehls in jeder Art von Sitzung erhält.

<Tip>
  Um tiefere Überlegungen anzufordern, wenn ein Skill ausgeführt wird, fügen Sie `ultrathink` irgendwo im Skill-Inhalt ein. Siehe [Verwenden Sie ultrathink für einmalige tiefe Überlegungen](/docs/de/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h4 id="how-injected-commands-run">
  Wie injizierte Befehle ausgeführt werden
</h4>

Claude Code wählt das Tool, das die injizierte Befehle eines Skills ausführt, aus dem `shell`-Schlüssel in der Frontmatter des Skills und Ihrer Umgebung aus. Jede Kombination führt die Befehle durch das Bash-Tool oder das PowerShell-Tool aus, mit Ausnahme einer Kombination, die den Aufruf sofort fehlschlagen lässt:

* `shell: powershell`, mit dem [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) aktiviert: Die Befehle werden durch das PowerShell-Tool ausgeführt.
* `shell: bash`, wenn bash nicht verfügbar ist: Der Aufruf schlägt fehl, bevor ein Befehl ausgeführt wird. Dies geschieht unter Windows ohne Git Bash. Claude Code zeigt ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``.
* Jede andere Kombination: Die Befehle werden durch das Bash-Tool ausgeführt, wenn bash verfügbar ist. Wenn nicht, werden sie durch das PowerShell-Tool ausgeführt.

Beide Tools führen die Befehle auf die gleiche Weise aus wie Claudes eigene Shell-Befehle. Sie teilen sich das Arbeitsverzeichnis, das Timeout und die Ausgabeverarbeitung:

* **Arbeitsverzeichnis**: Claude Code führt jeden Befehl im aktuellen Arbeitsverzeichnis der Session-Shell aus. Dieses Verzeichnis wechselt, wenn Claude `cd` ausführt. Verwenden Sie [`${CLAUDE_SKILL_DIR}` oder `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) in Pfaden, die sich jedes Mal auf die gleiche Weise auflösen müssen.
* **stderr**: Mit der Standard-`bash`-Shell führt Claude Code stderr in stdout zusammen. Alles, was der Befehl in stderr schreibt, erscheint im eingefügten Text.
* **Timeout**: Jeder Befehl wird unter dem Standard-2-Minuten-[Timeout](/docs/de/tools-reference#timeout-and-output-limits) des Bash-Tools ausgeführt. Wenn das Bash-Tool [einen Befehl mit Timeout in den Hintergrund verschiebt](/docs/de/tools-reference#background-commands), wird der Skill trotzdem gerendert. Der eingefügte Text meldet die Verschiebung und nennt die Hintergrund-Task und die Datei, die die Ausgabe des Befehls sammelt. Wenn der Befehl einer ist, den das Bash-Tool niemals automatisch in den Hintergrund verschiebt, beendet Claude Code ihn beim Timeout. Dieser Fehler [bricht den Aufruf ab](#when-an-injected-command-fails).
* **Ausgabegröße**: Ausgabe, die die Inline-Obergrenze des Bash-Tools überschreitet, kommt als Dateipfad plus kurze Vorschau an, nicht als gekürzter Text. [Ausgabegrenzen](/docs/de/tools-reference#output-limits) behandelt die Obergrenze und wie man jede Grenze anpasst.

Das PowerShell-Tool wendet das gleiche Timeout-, Backgrounding- und Output-Ceiling-Verhalten auf die Befehle an, die es ausführt. Siehe den Abschnitt [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) für seine Besonderheiten.

<h4 id="when-an-injected-command-fails">
  Wenn ein injizierter Befehl fehlschlägt
</h4>

Ein fehlgeschlagener Befehl bricht den gesamten Skill-Aufruf ab, nicht nur seinen eigenen Platzhalter. Claude sieht den Skill-Inhalt für diesen Aufruf nie. Der Abbruch zeigt `Shell command failed for pattern "..."`. Die Fehlermeldung enthält die Ausgabe des Befehls unter `[stderr]`.

Mit der Standard-`bash`-Shell zählt jeder Exit-Code ungleich Null als Fehler. Eine Ausnahme gilt: Claude Code behandelt Exit-Code 1 von [Such- und Vergleichsbefehlen](/docs/de/tools-reference#output-limits) als normales Ergebnis und fügt ihre Ausgabe ein. Exit-Codes von 2 oder höher schlagen auch für diese Befehle fehl.

Welche Befehle die Ausnahme erhalten, hängt von der Shell ab:

* Standard-`bash`-Shell: Die Befehle, die unter [Ausgabegrenzen](/docs/de/tools-reference#output-limits) aufgelistet sind
* `shell: powershell`, wenn das PowerShell-Tool aktiviert ist: Ein [anderer Satz](/docs/de/tools-reference#shell-selection-in-settings-hooks-and-skills), der `grep` und `git diff` enthält, aber nicht `find` oder `diff`

Mit der Standard-`bash`-Shell fügen Sie `|| true` an jeden anderen Befehl an, von dem Sie erwarten, dass er mit einem Exit-Code ungleich Null endet. Ein Überprüfungsskript, das 1 beendet, wenn es Probleme findet, ist ein Beispiel.

<h4 id="permission-checks-on-injected-commands">
  Berechtigungsprüfungen für injizierte Befehle
</h4>

Injizierte Befehle fordern niemals Genehmigung an, während der Skill gerendert wird. Claude Code prüft jeden gegen Ihre [Berechtigungsregeln](/docs/de/permissions) zuerst. Ein Befehl, den eine Deny-Regel erfasst, bricht den Aufruf mit `Shell command permission check failed for pattern "..."` ab.

Außerhalb des [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), wenn die Berechtigungsprüfung eines Befehls etwas anderes als Zulassung zurückgibt, bricht Claude Code den Aufruf ab. Dies schließt eine Regel ein, die normalerweise fragen würde. Um zu verhindern, dass ein nicht übereinstimmender Befehl hier abbricht, genehmigen Sie ihn vorher mit [`allowed-tools`](#pre-approve-tools-for-a-skill). Deny- und Ask-Regeln überschreiben immer noch `allowed-tools`. Siehe [Berechtigungen verwalten](/docs/de/permissions#manage-permissions).

Im Auto-Modus bricht ein Befehl, der sonst Ihre Genehmigung benötigen würde, den Aufruf nicht ab. Der Skill wird mit einer Anweisung geladen, die Claude anweist, den Befehl zuerst auszuführen, und Claudes eigener Aufruf wird dann durch [die üblichen Prüfungen des Auto-Modus](/docs/de/permission-modes#how-the-classifier-evaluates-actions) durchgeführt. Der Aufruf bricht immer noch in einem [verzweigten Skill](#run-skills-in-a-subagent) ab, der `agent` setzt, und in einer Sitzung, in der Claude nicht das [Shell-Tool hat, das injizierte Befehle ausführt](#how-injected-commands-run).

<h3 id="run-skills-in-a-subagent">
  Skills in einem Subagenten ausführen
</h3>

Fügen Sie `context: fork` zu Ihrer Frontmatter hinzu, wenn Sie möchten, dass ein Skill isoliert ausgeführt wird. Claude Code startet einen neuen Subagenten des im `agent`-Feld festgelegten Typs und gibt ihm den Skill-Inhalt als seinen Prompt. Der Subagent sieht Ihren Gesprächsverlauf nicht, daher müssen die Anweisungen des Skills eigenständig sein.

<Note>
  Trotz des Namens wird ein Skill mit `context: fork` nicht in einer [Verzweigung des aktuellen Gesprächs](/docs/de/sub-agents#fork-the-current-conversation) ausgeführt, was dem Subagenten alles geben würde, das Sie bisher besprochen haben. Wenn die Task von diesem Verlauf abhängt, verzweigen Sie das Gespräch, anstatt `context: fork` zu verwenden.
</Note>

Der verzweigte Subagent wird im [Hintergrund](/docs/de/sub-agents#run-subagents-in-foreground-or-background) ausgeführt: Sie arbeiten weiter, während er läuft, und sein Ergebnis kommt in Ihr Gespräch, wenn es abgeschlossen ist. Setzen Sie `background: false` in der Frontmatter, um stattdessen auf das Ergebnis in dem Zug zu warten, der den Skill aufgerufen hat. Vor v2.1.218 blockierten verzweigte Skills den Zug immer, bis sie fertig waren.

Claude Code wartet auch auf das Ergebnis, selbst wenn der Skill `background: false` nicht setzt, in Fällen wie diesen:

* Im nicht-interaktiven Modus mit dem `-p`-Flag oder dem Agent SDK
* Wenn Sie [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/de/env-vars) auf `1` setzen, was auch alle anderen Hintergrund-Task-Funktionen ausschaltet
* Wenn Sie einen verzweigten Skill aufrufen, während ein früherer Aufruf desselben Skills noch läuft
* Wenn eine [geplante Task](/docs/de/scheduled-tasks) mit dem Skill als Prompt ausgelöst wird

Ein hintergrund-verzweigter Skill wird auch mit dem [engeren Tool-Set ausgeführt, das für Hintergrund-Subagenten gilt](/docs/de/sub-agents#run-subagents-in-foreground-or-background): Der Subagent des Skills ist ein regulärer Agent-Typ, daher gilt die Ausnahme für Subagenten, die das Gespräch verzweigen, nicht. Wenn die Schritte Ihres Skills von einem Tool außerhalb dieses Sets abhängen, setzen Sie `background: false`, um das vollständige Tool-Set beizubehalten.

Ein verzweigter Skill, der im Hintergrund läuft, wendet seine Änderungen außerhalb der [Checkpoints](/docs/de/checkpointing) Ihrer Session an, sodass `/rewind` sie nicht rückgängig macht; verwenden Sie git, um sie rückgängig zu machen.

<Warning>
  `context: fork` macht nur Sinn für Skills mit expliziten Anweisungen. Wenn Ihr Skill Richtlinien wie „verwenden Sie diese API-Konventionen" ohne eine Task enthält, erhält der Subagent die Richtlinien, aber keinen umsetzbaren Prompt, und gibt ohne aussagekräftige Ausgabe zurück.
</Warning>

Skills und [Subagenten](/docs/de/sub-agents) arbeiten in zwei Richtungen zusammen:

| Ansatz                     | System-Prompt                | Task                         | Lädt auch                                                                                                        |
| :------------------------- | :--------------------------- | :--------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| Skill mit `context: fork`  | Vom Agent-Typ                | SKILL.md-Inhalt              | CLAUDE.md, gemäß dem [Startup-Kontext](/docs/de/sub-agents#what-loads-at-startup) des Agenten                         |
| Subagent mit `skills`-Feld | Markdown-Text des Subagenten | Claudes Delegationsnachricht | Vorgeladene Skills + CLAUDE.md, gemäß dem [Startup-Kontext](/docs/de/sub-agents#what-loads-at-startup) des Subagenten |

Mit `context: fork` schreiben Sie die Task in Ihren Skill und wählen einen Agent-Typ aus, um sie auszuführen. Die integrierten Explore- und Plan-Agenten [überspringen CLAUDE.md und git status](/docs/de/sub-agents#what-loads-at-startup), um ihren Kontext klein zu halten, sodass ein verzweigter Skill mit `agent: Explore` nur den SKILL.md-Inhalt und den eigenen System-Prompt des Agenten sieht. Für das Gegenteil, bei dem Sie einen benutzerdefinierten Subagenten definieren, der Skills als Referenzmaterial verwendet, siehe [Subagenten](/docs/de/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Beispiel: Research-Skill mit Explore-Agent
</h4>

Dieser Skill führt Recherchen in einem verzweigten Explore-Agent aus. Der Skill-Inhalt wird zur Task, und der Agent bietet schreibgeschützte Tools, die für die Codebase-Exploration optimiert sind:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Wenn dieser Skill ausgeführt wird:

1. Ein neuer isolierter Kontext wird erstellt
2. Der Subagent erhält den Skill-Inhalt als seinen Prompt (die „Research \$ARGUMENTS thoroughly..."-Anweisungen)
3. Das `agent`-Feld bestimmt die Ausführungsumgebung (Modell, Tools und Berechtigungen)
4. Der Subagent fasst seine Ergebnisse zusammen und gibt sie an Ihr Hauptgespräch zurück, wenn er fertig ist

Das `agent`-Feld gibt an, welche Subagenten-Konfiguration verwendet werden soll. Optionen sind integrierte Agenten (`Explore`, `Plan`, `general-purpose`) oder ein beliebiger benutzerdefinierter Subagent aus `.claude/agents/`. Wenn nicht angegeben, wird `general-purpose` verwendet.

<h3 id="restrict-claude’s-skill-access">
  Claudes Skill-Zugriff einschränken
</h3>

Standardmäßig kann Claude jeden Skill aufrufen, der nicht `disable-model-invocation: true` gesetzt hat. Skills, die `allowed-tools` definieren, gewähren Claude Zugriff auf diese Tools ohne Genehmigung pro Verwendung während des Zugs, der den Skill aufruft; die Genehmigung wird gelöscht, wenn Sie Ihre nächste Nachricht senden. Ihre [Berechtigungseinstellungen](/docs/de/permissions) regeln immer noch das Baseline-Genehmigungsverhalten für alle anderen Tools. Einige integrierte Befehle sind auch über das Skill-Tool verfügbar, einschließlich `/init` und `/security-review`. Andere integrierte Befehle wie `/compact` sind nicht verfügbar.

Drei Möglichkeiten, um zu kontrollieren, welche Skills Claude aufrufen kann:

**Alle Skills deaktivieren**, indem Sie das Skill-Tool in `/permissions` ablehnen:

```text theme={null}
# Add to deny rules:
Skill
```

**Spezifische Skills zulassen oder ablehnen** mit [Berechtigungsregeln](/docs/de/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Berechtigungssyntax: `Skill(name)` für exakte Übereinstimmung, `Skill(name *)` für Präfix-Übereinstimmung mit beliebigen Argumenten.

Wenn Ihre `deny`-Regel einen Alias oder einen unqualifizierten Namen anstelle des eigenen Namens des Skills benennt, blockiert Claude Code den Skill trotzdem: Mit `Skill(review)` blockiert es den gebündelten `/code-review` durch seinen `/review`-Alias, und mit `Skill(deploy)` blockiert es einen [verschachtelten Skill](#where-skills-live), der als `apps/web:deploy` aufgelistet ist, durch seinen unqualifizierten Namen. Vor v2.1.260 blockierte Claude Code einen verschachtelten Skill, der unter seinem qualifizierten Namen aufgelistet ist, nicht, wenn die deny-Regel nur den unqualifizierten Namen benannte.

Claude Code stimmt einer `allow`-Regel nur gegen den eigenen Namen des Skills und den Namen in Claudes Aufruf ab.

**Einzelne Skills ausblenden**, indem Sie `disable-model-invocation: true` zu ihrer Frontmatter hinzufügen. Dies entfernt den Skill vollständig aus Claudes Kontext.

<Note>
  Mit `user-invocable: false` können Sie den Skill nicht aufrufen, aber Claude kann. Um zu verhindern, dass Claude ihn über das Skill-Tool aufruft, setzen Sie `disable-model-invocation: true`.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Skill-Sichtbarkeit aus Einstellungen überschreiben
</h3>

Die `skillOverrides`-Einstellung steuert die Skill-Sichtbarkeit aus Ihren [Einstellungen](/docs/de/settings) anstelle der eigenen Frontmatter des Skills. Verwenden Sie sie für Skills, deren SKILL.md Sie nicht bearbeiten möchten, wie z. B. solche, die in ein gemeinsames Projekt-Repo eingecheckt sind. Das `/skills`-Menü schreibt es für Sie: Markieren Sie einen Skill und drücken Sie `Space`, um die Zustände zu durchlaufen, dann `Esc`, um in `.claude/settings.local.json` zu speichern.

Jeder Schlüssel ist ein Skill-Name und jeder Wert ist einer von vier Zuständen:

| Wert                    | Aufgelistet für Claude | Im `/`-Menü |
| :---------------------- | :--------------------- | :---------- |
| `"on"`                  | Name und Beschreibung  | Ja          |
| `"name-only"`           | Nur Name               | Ja          |
| `"user-invocable-only"` | Versteckt              | Ja          |
| `"off"`                 | Versteckt              | Versteckt   |

Das `/skills`-Menü kennzeichnet den `"user-invocable-only"`-Zustand als `user-only`.

Ab v2.1.199 versteckt `"off"` den Skill auch vor den Befehlslisten, die [Remote Control](/docs/de/remote-control)-Clients und [Agent SDK](/docs/de/agent-sdk/skills#discover-available-commands)-Aufrufer erhalten, zusätzlich zum Terminal-`/`-Menü. Das Aufrufen eines versteckten Skills mit seinem vollständigen Namen gibt stattdessen den `skillOverrides`-Fehler zurück, anstatt ihn auszuführen.

Ein Skill, der in `skillOverrides` fehlt, wird als `"on"` behandelt. Das folgende Beispiel reduziert einen Skill auf seinen Namen und schaltet einen anderen ganz aus:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Einige gebündelte Skills haben Aliase, wie z. B. `checkup` für `/doctor`. Wenn Sie einen `skillOverrides`-Eintrag unter einem Alias in [verwalteten Einstellungen](/docs/de/managed-settings) oder in einer Datei, die Sie mit dem `--settings`-Flag übergeben, festlegen, wendet Claude Code ihn auf den Skill hinter dem Alias an. Sie können einen Skill nur durch einen Alias weiter einschränken, ihn niemals sichtbarer machen, und wenn Sie auch einen Eintrag unter dem eigenen Namen des Skills in verwalteten Einstellungen festlegen, hat dieser Eintrag Vorrang. Vor v2.1.260 wendete Claude Code einen Eintrag unter einem Alias nicht auf den Skill in einer Einstellungsquelle an.

In Benutzer-, Projekt- und lokalen Einstellungen stimmt Claude Code Einträge nur gegen Skill-Namen ab. Wenn Sie dort einen Eintrag für `review` festlegen, gilt er für einen Skill namens `review`, nicht für den gebündelten `/code-review` durch seinen `/review`-Alias.

Plugin-Skills sind nicht von `skillOverrides` betroffen. Verwalten Sie diese stattdessen über `/plugin`.

<h3 id="find-unused-skills">
  Ungenutzte Skills finden
</h3>

Jeder Skill in der [Skill-Auflistung](#skill-descriptions-are-cut-short) trägt zu Ihrem Kontext bei jedem Zug bei, unabhängig davon, ob Claude ihn jemals verwendet. Führen Sie `/skill-doctor` aus, um zu sehen, was jeder Ihrer Skills kostet und wie oft er verwendet wird, damit Sie entscheiden können, welche Sie ausschalten möchten. In einer interaktiven Sitzung wird der Bericht in der Registerkarte **Stats** des `/plugin`-Managers geöffnet. Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` gibt Claude Code ihn als Text aus.

Der Bericht behandelt die Skills in Ihrer Sitzung außer gebündelten Skills und Enterprise-Skills. Er kennzeichnet Skills in der Auflistung, die nie aufgerufen wurden, und sagt, wo man sie ausschalten kann. Von den Skills, bei denen er sagt, wo man sie ausschalten kann, beginnen Sie mit denen, die die höchsten Kontextkosten haben. Der Bericht listet auch Plugins auf, die Sie kürzlich nicht verwendet haben.

`/skill-doctor` erfordert Claude Code v2.1.252 oder später und ist nicht in Sitzungen verfügbar, die [Feature-Flag-Abruf](/docs/de/env-vars#features-that-need-feature-flag-fetching) überspringen. Wenn Sie `/skill-doctor` über [Remote Control](/docs/de/remote-control) von Ihrem Telefon oder Browser aus ausführen, antwortet Claude Code stattdessen [`Skill usage reports are not available on this connection.`](/docs/de/errors#skill-usage-reports-are-not-available-on-this-connection). Führen Sie `/skill-doctor` im Terminal auf dem Computer aus, auf dem die Sitzung läuft.

<h2 id="evaluate-and-iterate-on-a-skill">
  Skill evaluieren und iterieren
</h2>

Das Sehen eines Skill-Triggers zeigt dir, dass Claude ihn gefunden hat, nicht dass er das getan hat, was du beabsichtigt hast. Um zu wissen, dass ein Skill funktioniert, musst du zwei Dinge separat messen: ob Claude ihn bei den Prompts aufruft, bei denen er sollte, und ob die Ausgabe dem entspricht, was du erwartest, wenn er es tut.

Die Überprüfung beider ist ein Baseline-Vergleich. Sammle ein paar realistische Prompts, führe jeden in einer neuen Sitzung mit dem verfügbaren Skill aus und wiederhole dies mit ihm [deaktiviert](#override-skill-visibility-from-settings), und vergleiche die Ergebnisse. Eine neue Sitzung ist wichtig, da der verbleibende Kontext aus der Erstellung des Skills Lücken in den geschriebenen Anweisungen verdeckt.

Zwei Tools automatisieren diesen Vergleich. Für einen Skill, der in einem [Plugin](/docs/de/plugins/overview) ausgeliefert wird, führt [`claude plugin eval`](/docs/de/plugin-evals) jeden Prompt in einer isolierten Sitzung mit und ohne das Plugin aus, bewertet ihn mit Gradern, die du definierst oder die er für dich schreibt, und beendet sich mit einem Nicht-Null-Wert unterhalb eines Schwellwerts, sodass du CI darauf abstimmen kannst. Um an einem einzelnen Skill in einer Claude Code-Konversation zu iterieren, führt das unten stehende skill-creator-Plugin eine ähnliche Schleife mit seinem eigenen `evals/evals.json`-Format aus. Die beiden Formate sind nicht austauschbar.

<h3 id="run-evals-with-skill-creator">
  Evals mit skill-creator ausführen
</h3>

Das [`skill-creator` Plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatisiert die Vergleichsschleife in Claude Code. Installiere es vom offiziellen Marketplace:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Wenn die Installation fehlschlägt, vergleiche die Nachricht, die Claude Code meldet:

* `Marketplace "claude-plugins-official" not found`: Füge den Marketplace mit `/plugin marketplace add anthropics/claude-plugins-official` hinzu und versuche dann die Installation erneut.
* Das Plugin wird [nicht im Marketplace gefunden](/docs/de/plugins/install#install-a-plugin): Überprüfe den Plugin-Namen.

Wenn die Installationszusammenfassung `Run /reload-plugins to activate.` meldet, führt Claude Code dann diesen Reload für dich aus. Wenn der Reload warnt, dass deine nächste Nachricht die Konversation erneut lesen würde, führe `/reload-plugins --force` aus, um die Skills des Plugins in der aktuellen Sitzung verfügbar zu machen. Bitte dann Claude, einen vorhandenen Skill zu evaluieren, zum Beispiel `evaluate my summarize-changes skill with skill-creator`. Das Plugin führt dich durch das Schreiben von Testfällen und führt die Schleife aus:

* **Testfälle**: speichert Prompts, Eingabedateien und erwartetes Verhalten in `evals/evals.json` im Skill-Verzeichnis
* **Isolierte Ausführungen**: spawnt einen [Subagent](/docs/de/sub-agents) pro Testfall, sodass jede Ausführung mit einem sauberen Kontext beginnt, und zeichnet Token-Anzahl und Dauer auf
* **Bewertung**: überprüft jede Assertion gegen die Ausgabe und schreibt Bestanden oder Nicht bestanden mit Beweis in `grading.json`
* **Benchmark**: aggregiert Erfolgsquote, Zeit und Tokens für mit-Skill versus ohne-Skill in `benchmark.json`, sodass du die Verbesserung der Erfolgsquote gegen den Token- und Zeit-Overhead vergleichen kannst
* **Versionsvergleich**: führt einen blinden A/B zwischen zwei Versionen des Skills durch, sodass du bestätigen kannst, dass eine Bearbeitung eine Verbesserung ist, bevor du sie commitest
* **Beschreibungsoptimierung**: generiert sollte-auslösen und sollte-nicht-auslösen Prompts, misst die Trefferquote und schlägt Beschreibungsbearbeitungen vor, wenn der Skill bei falschen Anfragen aktiviert wird
* **Review-Viewer**: öffnet einen HTML-Bericht, in dem du jede Ausgabe inspizieren und qualitatives Feedback aufzeichnen kannst, das die nächste Iteration liest

Für das Eval-Dateiformat und den vollständigen Iterations-Workflow siehe [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) auf agentskills.io. Für Hintergrundinformationen zum Benchmark- und Vergleichsmodus siehe die [skill-creator Ankündigung](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Fähigkeiten teilen
</h2>

Fähigkeiten können je nach Zielgruppe in verschiedenen Bereichen verteilt werden:

* **Projektfähigkeiten**: Commit `.claude/skills/` zur Versionskontrolle
* **Plugins**: Erstellen Sie ein `skills/`-Verzeichnis in Ihrem [Plugin](/docs/de/plugins/overview)
* **Verwaltet**: Bereitstellung organisationsweit über [verwaltete Einstellungen](/docs/de/managed-settings)

<h3 id="generate-visual-output">
  Visuelle Ausgabe generieren
</h3>

Fähigkeiten können Skripte in jeder Sprache bündeln und ausführen und Claude Funktionen geben, die über das hinausgehen, was in einer einzelnen Eingabeaufforderung möglich ist. Ein Muster ist die Generierung visueller Ausgabe: interaktive HTML-Dateien, die in Ihrem Browser geöffnet werden, um Daten zu erkunden, Fehler zu beheben oder Berichte zu erstellen.

Dieses Beispiel erstellt einen Codebase-Explorer: eine interaktive Baumansicht, in der Sie Verzeichnisse erweitern und reduzieren können, Dateigröße auf einen Blick sehen und Dateitypen nach Farbe identifizieren können.

Erstellen Sie das Fähigkeitsverzeichnis:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Speichern Sie dies unter `~/.claude/skills/codebase-visualizer/SKILL.md`. Die Beschreibung teilt Claude mit, wann diese Fähigkeit aktiviert werden soll, und die Anweisungen teilen Claude mit, das gebündelte Skript auszuführen. Der Skriptpfad verwendet [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions), damit er korrekt aufgelöst wird, unabhängig davon, ob die Fähigkeit auf persönlicher, Projekt- oder Plugin-Ebene installiert ist:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Speichern Sie dies unter `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Dieses Skript scannt einen Verzeichnisbaum und generiert eine in sich geschlossene HTML-Datei mit:

* Eine **Zusammenfassungs-Seitenleiste** mit Dateianzahl, Verzeichnisanzahl, Gesamtgröße und Anzahl der Dateitypen
* Ein **Balkendiagramm**, das die Codebase nach Dateityp aufschlüsselt (Top 8 nach Größe)
* Ein **zusammenklappbarer Baum**, in dem Sie Verzeichnisse erweitern und reduzieren können, mit farbcodierten Dateityp-Indikatoren

Das Skript erfordert Python 3, verwendet aber nur integrierte Bibliotheken, daher müssen keine Pakete installiert werden:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Zum Testen öffnen Sie Claude Code in einem beliebigen Projekt und fragen Sie „Visualize this codebase." Claude führt das Skript aus, das den Pfad der generierten Datei ausgibt, z. B. `Generated /path/to/codebase-map.html`, und öffnet es in Ihrem Browser. Wenn Sie in einer Umgebung ohne Kopf arbeiten, in der kein Browser geöffnet wird, bestätigt der gedruckte Pfad, dass das Skript erfolgreich war.

Dieses Muster funktioniert für jede visuelle Ausgabe: Abhängigkeitsgraphen, Testabdeckungsberichte, API-Dokumentation oder Datenbankschema-Visualisierungen. Das gebündelte Skript erledigt die Arbeit, während Claude die Orchestrierung übernimmt.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="skill-not-triggering">
  Skill wird nicht ausgelöst
</h3>

Wenn Claude Ihren Skill nicht wie erwartet verwendet:

1. Überprüfen Sie, dass die Beschreibung Schlüsselwörter enthält, die Benutzer natürlicherweise sagen würden
2. Stellen Sie sicher, dass der Skill in `What skills are available?` angezeigt wird
3. Versuchen Sie, Ihre Anfrage umzuformulieren, um sie besser an die Beschreibung anzupassen
4. Rufen Sie ihn direkt mit `/skill-name` auf, wenn der Skill vom Benutzer aufgerufen werden kann

Wenn die Frontmatter-YAML fehlerhaft ist, lädt Claude Code den Skill-Body mit leeren Metadaten, sodass `/skill-name` weiterhin funktioniert, aber Claude keine `description` zum Abgleichen hat. Führen Sie mit `--debug` aus, um den Parse-Fehler zu sehen.

Wenn der Skill in einem Plugin enthalten ist, können Sie messen, wie oft er bei realistischen Prompts ausgelöst wird, anstatt ihn einzeln zu überprüfen: Schreiben Sie einen Eval-Fall mit einem [`tool_used: Skill` Grader](/docs/de/plugin-evals#create-your-first-eval-suite) und führen Sie ihn mit `claude plugin eval` nach jeder Beschreibungsänderung aus.

Um `SKILL.md`-Dateien zu finden, deren Frontmatter nicht geparst wird, führen Sie [`claude plugin validate`](/docs/de/plugins/cli-reference#validate-a-directory) im Skills-Verzeichnis aus, beispielsweise `claude plugin validate .claude/skills` für Projekt-Skills oder `claude plugin validate ~/.claude/skills` für persönliche Skills. Erfordert Claude Code v2.1.233 oder später.

<h3 id="skill-triggers-too-often">
  Skill wird zu oft ausgelöst
</h3>

Wenn Claude Ihren Skill verwendet, wenn Sie das nicht möchten:

1. Machen Sie die Beschreibung spezifischer
2. Fügen Sie `disable-model-invocation: true` hinzu, wenn Sie nur manuelle Aufrufe möchten

<h3 id="skill-descriptions-are-cut-short">
  Skill-Beschreibungen werden gekürzt
</h3>

Claude Code lädt eine Auflistung von Skill-Namen und Beschreibungen in den Kontext, damit Claude weiß, was verfügbar ist. Die Auflistung enthält immer jeden Skill-Namen, aber wenn Sie viele Skills haben, kürzt Claude Code die Beschreibungen, um in das Zeichenbudget der Auflistung zu passen, was die Schlüsselwörter entfernen kann, die Claude zum Abgleichen Ihrer Anfrage benötigt. Das Budget skaliert mit 1 % des Kontextfensters des Modells. Wenn die Auflistung überläuft, löscht Claude Code Beschreibungen beginnend mit den Skills, die Sie am wenigsten aufrufen, sodass die Skills, die Sie am meisten verwenden, ihren vollständigen Text behalten.

Führen Sie `/doctor` aus, um eine Schätzung der Kontextkosten der Auflistung und ihrer größten Beitragenden zu erhalten. Um Skills zu finden, die sich lohnen auszuschalten, führen Sie [`/skill-doctor`](#find-unused-skills) aus. Wenn die Auflistung ihr Budget überschreitet, schreibt Claude Code auch eine Warnung in das Debug-Protokoll, das mit [`--debug`](/docs/de/cli-reference#cli-flags) sichtbar ist.

Die Skills-Zeile in `/context` meldet die Größe der Auflistung nach Anwendung des Budgets, sodass sie dem entspricht, was das Modell erhält. Vor v2.1.196 zählte die Zeile den vollständigen Text jeder Beschreibung und konnte einen Wert anzeigen, der mehrmals größer als das konfigurierte Budget war.

Um das Budget zu erhöhen, legen Sie die Einstellung [`skillListingBudgetFraction`](/docs/de/settings-reference#skilllistingbudgetfraction) (z. B. `0.02` = 2 %) oder die Umgebungsvariable `SLASH_COMMAND_TOOL_CHAR_BUDGET` auf eine feste Zeichenanzahl fest. Um Budget für andere Skills freizugeben, legen Sie Einträge mit niedriger Priorität auf `"name-only"` in [`skillOverrides`](#override-skill-visibility-from-settings) fest, sodass sie ohne Beschreibung aufgelistet werden. Sie können auch den Text `description` und `when_to_use` an der Quelle kürzen: Stellen Sie den wichtigsten Anwendungsfall zuerst, da der kombinierte Text jedes Eintrags unabhängig vom Budget auf 1.536 Zeichen begrenzt ist. Die Obergrenze ist mit [`skillListingMaxDescChars`](/docs/de/settings-reference#skilllistingmaxdescchars) konfigurierbar.

<h3 id="personal-skills-disappeared">
  Persönliche Skills sind verschwunden
</h3>

Wenn Skill-Ordner, die Sie in `~/.claude/skills/` erstellt haben, weg sind, schauen Sie in `~/.claude/skills/.trash/`. Wenn Claude Code [Skills von claude.ai synchronisiert](#how-synced-skills-behave), lädt es sie in den separaten `synced`-Unterordner herunter und verschiebt oder löscht nicht die Ordner, die Sie erstellen.

Vor v2.1.280 führte eine Datei namens `manifest.json` in `~/.claude/skills/` dazu, dass Claude Code die Skill-Ordner, die diese Datei auflistete, in einen mit Zeitstempel versehenen Ordner unter `~/.claude/skills/.trash/` verschob, und diese Skills wurden nicht mehr geladen.

Um einen Skill wiederherzustellen, verschieben Sie seinen Ordner aus dem mit Zeitstempel versehenen Ordner zurück in `~/.claude/skills/`. Tun Sie dies vor dem [Aufbewahrungssweep](/docs/de/claude-directory#cleaned-up-automatically), der Trash-Einträge löscht, standardmäßig 30 Tage nach dem Verschieben in den Papierkorb.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* **[Debuggen Sie Ihre Konfiguration](/docs/de/debug-your-config)**: Diagnostizieren Sie, warum ein Skill nicht angezeigt oder ausgelöst wird
* **[Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills)**: das Eval-Dateiformat und Iterations-Workflow auf agentskills.io
* **[Skill authoring best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: Schreibanleitung, die über Claude-Produkte hinweg gilt
* **[Subagenten](/docs/de/sub-agents)**: Delegieren Sie Aufgaben an spezialisierte Agenten
* **[Plugins](/docs/de/plugins/overview)**: Packen und verteilen Sie Skills mit anderen Erweiterungen
* **[Hooks](/docs/de/hooks)**: Automatisieren Sie Workflows um Tool-Ereignisse
* **[Memory](/docs/de/memory)**: Verwalten Sie CLAUDE.md-Dateien für persistenten Kontext
* **[Befehle](/docs/de/commands)**: Referenz für integrierte Befehle und gebündelte Skills
* **[Berechtigungen](/docs/de/permissions)**: Steuern Sie Tool- und Skill-Zugriff
* **[Claude Tag Skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: Projekt-Skills, die in ein Repository übernommen wurden, werden auch geladen, wenn dieses Repository in einem Claude Tag-Kanal verwendet wird
