> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Erstellen Sie ein Claude Code-Plugin

> Erstellen Sie Ihr erstes Claude Code-Plugin aus einem leeren Verzeichnis, testen Sie es ohne einen Marketplace und konvertieren Sie ein vorhandenes .claude/-Setup.

Ein Plugin ist ein Verzeichnis von Skills, Agents, Hooks und MCP-Servern sowie eine `plugin.json`-Datei, die als Manifest bezeichnet wird und das Plugin benennt. Claude Code lädt das Verzeichnis als eine Einheit, sodass Sie es mit Teamkollegen teilen, in mehreren Projekten installieren oder in einem Marketplace veröffentlichen können.

Diese Seite ist für Personen, die ihre eigenen Plugins schreiben.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Installation eines Plugins von jemand anderem**: siehe [Plugins installieren](/docs/de/plugins/install)
  * **Sie sind sich nicht sicher, ob Sie ein Plugin benötigen**: siehe [Entscheiden Sie, ob Sie ein Plugin benötigen](/docs/de/plugins/overview#decide-whether-you-need-a-plugin) in der Übersicht
  * **Die Benutzer Ihres Plugins sind auf claude.ai oder in Cowork**: derselbe Ordner wird dort mit einer anderen Teilmenge von Komponenten installiert. Siehe [Plugins auf claude.ai und in Cowork](https://claude.com/docs/plugins/overview)
</Note>

Beginnen Sie mit dem Abschnitt, der dem entspricht, was Sie bereits haben:

* **Noch nichts**: folgen Sie [Erstellen Sie Ihr erstes Plugin](#create-your-first-plugin), dann [Entwickeln Sie ohne einen Marketplace](#develop-without-a-marketplace) und [Testen und Debuggen](#test-and-debug).
* **Dateien unter `.claude/` bereits vorhanden**: führen Sie die Anleitung zum ersten Plugin einmal durch, um das Layout zu verstehen, und folgen Sie dann [Konvertieren Sie ein vorhandenes `.claude/`-Setup](#convert-an-existing-claude-setup).

<h2 id="decide-when-to-use-a-plugin">
  Entscheiden Sie, wann Sie ein Plugin verwenden
</h2>

Skills, Agents, Hooks und MCP-Server funktionieren alle eigenständig in Ihrem Projekt oder Ihrem Home-Verzeichnis. Behalten Sie dieses eigenständige Setup bei, während es einem Projekt oder nur Ihnen dient. Erstellen Sie ein Plugin, wenn Sie das Setup mit Teamkollegen teilen, es in mehreren Projekten installieren oder versionierte Releases veröffentlichen möchten.

Wenn Sie eigenständige Skills, Agents, Hooks und MCP-Konfiguration in ein Plugin verschieben, ändern sich ihr Speicherort und ihre Namen:

* **Wo die Dateien hingehen**: unter das eigene Verzeichnis des Plugins, das als Plugin-Root bezeichnet wird, als `skills/`, `agents/`, `hooks/hooks.json` und `.mcp.json`.
* **Wie sie benannt werden**: Plugin-Skills und Agents erhalten den Plugin-Namen als Präfix, z. B. `/my-plugin:hello`, sodass zwei Plugins jeweils einen `hello`-Skill bereitstellen können, ohne zu kollidieren.

Um ein vorhandenes Setup in ein Plugin zu verschieben, siehe [Konvertieren Sie ein vorhandenes `.claude/`-Setup](#convert-an-existing-claude-setup).

<h2 id="create-your-first-plugin">
  Erstellen Sie Ihr erstes Plugin
</h2>

In dieser Anleitung erstellen Sie ein Plugin, dessen einzige Komponente ein Skill ist – eine Begrüßung – und führen es mit `--plugin-dir` aus, das ein Plugin für eine Sitzung lädt, ohne es zu installieren. Ein Plugin kann eine beliebige Mischung von [Komponenten](/docs/de/plugins/components) enthalten, z. B. Skills, Agents, Hooks und MCP-Server, und keine ist erforderlich; ein Skill ist das kleinste Beispiel, das das Layout zeigt.

Sie benötigen Claude Code [installiert und angemeldet](/docs/de/quickstart#step-1-install-claude-code).

Öffnen Sie ein Terminal in dem Verzeichnis, in dem Sie das Plugin behalten möchten, z. B. `~/projects`, und führen Sie die Befehle in diesen Schritten von dort aus aus. Sie können ein Plugin überall behalten, da Sie seinen Pfad an Claude Code übergeben, wenn Sie eine Sitzung starten.

<Steps>
  <Step title="Erstellen Sie das Plugin-Verzeichnis">
    Erstellen Sie das Plugin-Verzeichnis mit einem `.claude-plugin/`-Ordner darin, um das Manifest zu halten:

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="Schreiben Sie das Manifest">
    Das [Manifest](/docs/de/plugins/manifest-reference) ist eine JSON-Datei namens `plugin.json`, die Claude Code den Namen des Plugins mitteilt und es beschreibt. Speichern Sie dieses als `my-first-plugin/.claude-plugin/plugin.json`:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    Die vier Felder tun folgendes:

    * **`name`**: erforderlich. Es identifiziert das Plugin und wird zum Präfix für jeden Skill und Agent, den das Plugin bereitstellt. Setzen Sie keine Leerzeichen hinein.
    * **`description`**: der Text, den Benutzer für das Plugin in `/plugin` sehen.
    * **`version`**: optional. Das Setzen hält Benutzer auf dieser Version, bis Sie es ändern; [Veröffentlichen Sie eine neue Version](/docs/de/plugins/host-marketplace#release-a-new-version) sagt, wann Sie es setzen oder weglassen sollten.
    * **`author`**: wem Anerkennung gebührt. `name` ist erforderlich darin; `email` und `url` sind optional.

    Jedes andere Feld ist in der [Manifest-Referenz](/docs/de/plugins/manifest-reference#fields).

    Nur `plugin.json` geht in `.claude-plugin/`. Der Skill, den Sie als nächstes hinzufügen, geht direkt unter `my-first-plugin/`, neben diesem Ordner.
  </Step>

  <Step title="Fügen Sie einen Skill hinzu">
    Die einzige Komponente dieses Plugins ist ein Skill. Jeder Skill ist ein Verzeichnis unter `skills/`, das eine `SKILL.md`-Datei enthält. Erstellen Sie das Verzeichnis des Skills:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Erstellen Sie dann `my-first-plugin/skills/hello/SKILL.md` mit diesem Inhalt:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    Die Zeile `disable-model-invocation: true` bedeutet, dass Claude den Skill nicht von selbst ausführt, sodass nur Sie ihn auslösen. Entfernen Sie diese Zeile aus einem Skill, den Claude von selbst ausführen soll. Der Befehl des Skills kombiniert den Plugin-Namen und den Namen des Skills, sodass Sie diesen als `/my-first-plugin:hello` ausführen. Für die anderen Frontmatter-Felder siehe die [Skill-Frontmatter-Referenz](/docs/de/skills#frontmatter-reference).
  </Step>

  <Step title="Validieren Sie das Plugin">
    Überprüfen Sie das Manifest und das Frontmatter des Skills, bevor Sie etwas ausführen:

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    Der Befehl gibt den Manifest-Pfad aus, den er überprüft hat, und `✔ Validation passed`. Wenn er stattdessen `✘ Validation failed` ausgibt, benennt jede Zeile über dieser Ergebniszeile das zu behebende Feld. Schauen Sie jede Nachricht unter [`claude plugin validate` meldet Fehler](/docs/de/plugins/troubleshooting#claude-plugin-validate-reports-errors) nach.
  </Step>

  <Step title="Führen Sie Claude Code mit dem Plugin aus">
    Starten Sie eine Sitzung mit dem geladenen Plugin:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Sobald Claude Code startet, führen Sie den Skill aus:

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude antwortet mit einer Begrüßung.
  </Step>
</Steps>

Das Plugin wird nur in Sitzungen geladen, die Sie mit `--plugin-dir` starten. Um weiterhin daran zu arbeiten, ohne das Flag zu verwenden, oder um einen `.zip`-Build zu testen, siehe [Entwickeln Sie ohne einen Marketplace](#develop-without-a-marketplace).

<h3 id="share-the-plugin">
  Teilen Sie Ihr Plugin
</h3>

Ein Plugin, das Sie mit [Erstellen Sie Ihr erstes Plugin](#create-your-first-plugin) erstellt haben, existiert nur auf Ihrem Computer. Wenn es für andere Personen bereit ist, gibt es drei Möglichkeiten, es ihnen zu geben:

* **Senden Sie es direkt an ein paar Personen**: geben Sie ihnen das Plugin-Verzeichnis oder eine `.zip` davon, und nichts muss veröffentlicht werden. Siehe [Teilen Sie ein Plugin ohne einen Marketplace](/docs/de/plugins/publish#share-a-plugin-without-a-marketplace).
* **Listen Sie es in Ihrem eigenen Marketplace auf**: Teamkollegen fügen Ihren Marketplace einmal hinzu und installieren das Plugin nach Name, und sie erhalten Ihre Updates. Siehe [Veröffentlichen Sie über Ihren eigenen Marketplace](/docs/de/plugins/publish#publish-through-your-own-marketplace).
* **Reichen Sie es bei Anthropics Community-Marketplace ein**: Sobald es aufgelistet ist, kann jeder, der diesen Marketplace hinzufügt, es installieren. Siehe [Reichen Sie beim Community-Marketplace ein](/docs/de/plugins/publish#submit-to-the-community-marketplace).

<h3 id="plugin-layout">
  Plugin-Layout
</h3>

Jede Art von [Komponente](/docs/de/plugins/components), z. B. Skills, Agents, Hooks und MCP-Server, geht in ein festes Verzeichnis unter dem Plugin-Root, das das Verzeichnis ist, das Sie an `--plugin-dir` übergeben. Fügen Sie nur die Verzeichnisse hinzu, die Sie verwenden. Um durch ein vollständiges Plugin-Verzeichnis zu klicken und zu lesen, was jede Datei tut, öffnen Sie den [Plugin-Explorer](/docs/de/plugins/components#explore-the-plugin-directory).

Die Tabelle listet die Verzeichnisse auf, mit denen die meisten Plugins beginnen, und das [vollständige Layout](/docs/de/plugins/manifest-reference#standard-layout) listet den Rest auf.

| Speicherort                  | Inhalt                                                                                                                                      |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `.claude-plugin/plugin.json` | Das Manifest. Wenn Sie ein Plugin mit `--plugin-dir` laden und es kein Manifest hat, benennt Claude Code das Plugin nach seinem Verzeichnis |
| `skills/`                    | Ein `<name>/SKILL.md`-Verzeichnis pro Skill                                                                                                 |
| `commands/`                  | Flache Markdown-Dateien, die ältere Form von Skills. Verwenden Sie `skills/` für neue Plugins                                               |
| `agents/`                    | Eine Markdown-Datei pro Subagent                                                                                                            |
| `hooks/hooks.json`           | Hook-Konfiguration: ein Top-Level-`"hooks"`-Schlüssel, dessen Wert die gleiche Form wie `hooks` in einer Einstellungsdatei hat              |
| `.mcp.json`                  | MCP-Server-Definitionen                                                                                                                     |

<Warning>
  Nur `plugin.json` geht in `.claude-plugin/`. Komponenten, die dort gespeichert sind, werden nicht geladen.

  Der Plugin-Root ist das eigene Verzeichnis des Plugins, nicht `~/.claude/` selbst. Eine `.mcp.json`, die unter `~/.claude/.mcp.json` gespeichert ist, wird nicht geladen.
</Warning>

<h2 id="develop-without-a-marketplace">
  Entwickeln Sie ohne einen Marketplace
</h2>

Sie benötigen keinen [Marketplace](/docs/de/plugins/overview#get-plugins-from-a-marketplace), um ein Plugin auszuführen, das Sie schreiben. Laden Sie es stattdessen direkt von der Festplatte oder einer URL:

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): lädt ein Verzeichnis oder `.zip`-Archiv für eine Sitzung.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): ruft ein `.zip`-Archiv von einer URL für eine Sitzung ab.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): erstellt ein Plugin unter `~/.claude/skills/`, das in jeder Sitzung geladen wird.

Wenn zwei Plugins, die auf unterschiedliche Weise geladen werden, denselben Namen haben, siehe [Name-Konflikte](/docs/de/plugins/loading#name-conflicts), um zu sehen, welches Claude Code behält.

<h3 id="load-a-directory-or-archive-for-one-session">
  Laden Sie ein Plugin für eine Sitzung
</h3>

Sie können ein Plugin für eine einzelne Sitzung auf drei Arten laden: von einem Verzeichnis oder `.zip`-Archiv auf der Festplatte mit `--plugin-dir`, von einer URL mit `--plugin-url` oder von einer Umgebungsvariablen, wenn Sie kein Flag hinzufügen können. Jedes Plugin wird nur für diese Sitzung geladen, und nichts wird in Ihren Einstellungen dafür geschrieben. Wenn Sie die Dateien des Plugins während der Sitzung bearbeiten, führen Sie `/reload-plugins` aus, um die Änderungen zu laden.

<h4 id="from-a-directory-or-zip">
  Von einem Verzeichnis oder `.zip`
</h4>

Wenn Sie `claude` von Ihrer Shell aus starten, übergeben Sie `--plugin-dir` mit dem Plugin-Root-Verzeichnis oder einem `.zip`-Archiv davon. Wiederholen Sie das Flag, um mehrere Plugins zu laden:

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  Von einem Ordner mit Plugins
</h4>

Um mehrere Plugins von einem Ort zu laden, übergeben Sie einen Ordner, der sie enthält, z. B. `--plugin-dir ./plugins`. Das Laden eines Ordners mit Plugins erfordert Claude Code v2.1.265 oder später.

Wenn der Ordner kein `.claude-plugin/`-Verzeichnis und keine Plugin-Komponenten auf seiner obersten Ebene hat, behandelt Claude Code ihn als einen Ordner mit Plugins. Jeder unmittelbare Unterordner, der ein `.claude-plugin/plugin.json`-Manifest hat, wird dann als separates Plugin geladen. Alles andere im Ordner wird ohne Fehler übersprungen, einschließlich eines Unterordners, der kein Manifest hat. Wenn ein Plugin im Ordner nicht geladen wird, überprüfen Sie, dass sein Unterordner ein `.claude-plugin/plugin.json` hat.

In einer interaktiven Sitzung können Sie auch Plugins im Ordner nach dem Start hinzufügen und entfernen:

* Ein Unterordner, den Sie hinzufügen, wird als neues Plugin geladen, sobald sein Manifest vorhanden ist.
* Wenn Sie einen Unterordner entfernen, wird sein Plugin entladen.

Eine Nachricht erscheint in der Sitzung für jede dieser Änderungen. Wenn das Laden oder Entladen eines Plugins während des Gesprächs den [Prompt-Cache ungültig machen würde](/docs/de/prompt-caching#enabling-or-disabling-a-plugin), wird die Änderung stattdessen gehalten, und die Nachricht teilt Ihnen mit, dass Sie `/reload-plugins` ausführen sollen, um sie anzuwenden.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  Von einer URL
</h4>

Wenn Sie `claude` von Ihrer Shell aus starten, übergeben Sie `--plugin-url` mit der Adresse eines `.zip`-Archivs, z. B. eines Build-Artefakts, das Ihre CI veröffentlicht:

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code lädt das Archiv beim Start herunter. Um mehrere zu laden, wiederholen Sie das Flag oder übergeben Sie die URLs durch Leerzeichen getrennt in einem Argument in Anführungszeichen.

Zeigen Sie das Flag nur auf Archive, die Sie kontrollieren oder denen Sie vertrauen.

Wenn Claude Code das Archiv nicht abrufen kann oder das Archiv ungültig ist, startet es ohne das Plugin und zeichnet einen Plugin-Ladefehler auf, den Sie auf der Registerkarte **Errors** des `/plugin`-Managers überprüfen können.

<h4 id="from-an-environment-variable">
  Von einer Umgebungsvariablen
</h4>

Um Plugins in einer Sitzung zu laden, in der Sie das Flag `--plugin-dir` nicht hinzufügen können, listen Sie ihre absoluten Pfade stattdessen in der Umgebungsvariablen [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/de/env-vars#variables) auf. Claude Code lädt jeden Pfad, wie es einen `--plugin-dir`-Pfad lädt. Diese Plugins werden zusätzlich zu allen geladen, die Sie mit `--plugin-dir` übergeben. [Projekt- und lokale Einstellungen können diese Variable nicht setzen](/docs/de/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS` erfordert Claude Code v2.1.280 oder später.

Verwaltete Einstellungen können `--plugin-dir` und `CLAUDE_CODE_PLUGIN_DIRS` ausschalten. Siehe [Flags, die ein Plugin für eine Sitzung laden](/docs/de/plugins/cli-reference#flags-that-load-a-plugin-for-one-session). Um ein Plugin zusammen mit einem Plugin zu testen, von dem es abhängt, siehe [Testen Sie ein Plugin und seine Abhängigkeit lokal](/docs/de/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="scaffold-a-plugin-that-loads-every-session">
  Machen Sie ein Plugin in jeder Sitzung geladen
</h3>

Ihr persönliches Skills-Verzeichnis ist `~/.claude/skills/`. Claude Code lädt jeden Ordner dort, der ein `.claude-plugin/plugin.json` enthält, als Plugin in jeder Sitzung, ohne Flag und ohne Installationsschritt. `claude plugin init` erstellt ein solches Plugin für Sie.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  Erstellen Sie das Plugin mit `claude plugin init`
</h4>

`claude plugin init` schreibt ein Starter-Plugin unter `~/.claude/skills/`. Erfordert Claude Code v2.1.157 oder später. Erstellen Sie eines von Ihrer Shell aus:

```bash theme={null}
claude plugin init my-tool
```

Der Befehl erstellt `~/.claude/skills/my-tool/` mit einem `.claude-plugin/plugin.json` und einem Root-`SKILL.md`. Er gibt `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` gefolgt von `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.` aus.

Übergeben Sie `--with skills`, um `claude plugin init` zu haben, um einen Skill unter `skills/` für Sie zu erstellen. Die anderen `--with`-Werte sind in der [Plugin-Befehls-Referenz](/docs/de/plugins/cli-reference#plugin-init).

<h4 id="skill-names-in-a-scaffolded-plugin">
  Benennen Sie die Skills des Plugins
</h4>

Der Root-Skill unter `~/.claude/skills/my-tool/SKILL.md` ist auch ein persönlicher Skill, sodass Sie ihn als `/my-tool` aufrufen, nicht als `/my-tool:my-tool`. Skills, die Sie unter `skills/` im Plugin hinzufügen, erhalten das Plugin-Namen-Präfix, z. B. `/my-tool:example`.

<h4 id="stop-loading-the-plugin">
  Stoppen Sie das Laden des Plugins
</h4>

Um das Laden eines erstellten Plugins zu stoppen, löschen Sie sein Verzeichnis, oder führen Sie `claude plugin disable my-tool@skills-dir` in Ihrer Shell mit dem Namen `my-tool@skills-dir` aus, den `claude plugin init` gedruckt hat. In der ID `my-tool@skills-dir` steht `skills-dir` an der Stelle, wo ein Marketplace-Name stehen würde, da das Plugin von Ihrem Skills-Verzeichnis geladen wird, nicht von einem Marketplace.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  Teilen Sie das Plugin über ein Repository
</h4>

`claude plugin init` schreibt das Plugin in Ihr persönliches Skills-Verzeichnis unter `~/.claude/skills/`, sodass es für Sie in jedem Projekt geladen wird. Um ein Plugin für alle in einem Repository zu laden, erstellen Sie das gleiche Layout selbst unter `<project>/.claude/skills/<name>/`, einschließlich seines `.claude-plugin/plugin.json`. Siehe [Plugins, die über ein Repository geteilt werden](/docs/de/plugins/loading#plugins-shared-through-a-repository), um die Bedingungen zu sehen, unter denen Claude Code es lädt.

<h2 id="test-and-debug">
  Testen und Debuggen
</h2>

Wenn eine Änderung an Ihrem Plugin nicht angezeigt wird, arbeiten Sie diese Überprüfungen der Reihe nach durch. Jede teilt Ihnen mit, was Claude Code mit dem Plugin getan hat:

1. Führen Sie in Ihrer Shell `claude plugin validate <path>` aus. Es überprüft das Manifest und das Frontmatter jeder Skill-, Agent- und Command-Datei und beendet sich mit `0` bei `Validation passed`. Fügen Sie `--strict` hinzu, um auch bei Warnungen fehlzuschlagen. Exit-Codes und Verzeichnisbehandlung sind in der [Plugin-Befehls-Referenz](/docs/de/plugins/cli-reference#plugin-validate).
2. Führen Sie in der laufenden Sitzung `/reload-plugins` aus, um Änderungen anzuwenden, die Sie auf der Festplatte vorgenommen haben. Es gibt eine `Reloaded:`-Zeile mit Zählungen aus. Bestätigen Sie dann, dass ein Skill geladen wurde, indem Sie seinen `/plugin-name:skill`-Befehl eingeben, oder indem Sie das Plugin auf der Registerkarte **Installed** von `/plugin` finden.
3. Führen Sie in der gleichen Sitzung `/plugin` aus. Die Registerkarte **Installed** listet Ihr Plugin auf und zeigt in den Details des Plugins die Komponenten, die Claude Code gefunden hat. Die Registerkarte **Errors** listet auf, was nicht geladen wurde und warum, z. B. ein Pfad in Ihrem Manifest, der nicht existiert.
4. Führen Sie in Ihrer Shell `claude plugin list` aus. Es gibt Session-only- und Skills-Directory-Plugins in ihren eigenen Abschnitten mit `Status: ✔ loaded` oder dem Ladefehler aus. Um das Plugin einzubeziehen, das Sie entwickeln, übergeben Sie `--plugin-dir` mit seinem Pfad vor `plugin list`.

Um einen MCP-Server zu überprüfen, führen Sie `/mcp` in der Sitzung aus, um den Status des Servers zu sehen. Wenn der Server gesund ist, listet `/mcp` ihn als verbunden auf. Wenn nicht, siehe [MCP-Server, die nicht starten](/docs/de/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Um einen Hook zu überprüfen, lösen Sie das Ereignis aus, das er abgleicht. Bitten Sie Claude beispielsweise, eine Datei zu bearbeiten, um einen `PostToolUse`-Hook auszulösen. Lesen Sie dann das [Debug-Protokoll](/docs/de/hooks#debug-hooks), das zeigt, welche Hooks abgeglichen wurden, ihre Exit-Codes und ihre Ausgabe.

Die nächsten Abschnitte behandeln die Fehler, auf die Sie bei der Entwicklung am ehesten stoßen, und die [Seite zur Fehlerbehebung](/docs/de/plugins/troubleshooting#build-a-plugin) hat den vollständigen Eintrag für jeden.

<h3 id="a-component-path-isn’t-found">
  Ein Komponentenpfad wird nicht gefunden
</h3>

Die Registerkarte **Errors** von `/plugin` zeigt `<component> path not found: <path>`, z. B. `commands path not found`. Ein Komponentenpfad in Ihrem Manifest, z. B. `commands`, `skills`, `agents` oder `hooks`, zeigt auf nichts. Beheben Sie den Pfad oder erstellen Sie das Verzeichnis, und führen Sie dann `/reload-plugins` in der Sitzung aus. Siehe [`commands path not found`](/docs/de/plugins/troubleshooting#commands-path-not-found).

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` bei einem Marketplace-Root lädt die Plugins unter `plugins/` nicht
</h3>

`--plugin-dir` nimmt das Plugin-Root-Verzeichnis, das `.claude-plugin/plugin.json` und die Komponentenverzeichnisse wie `skills/` enthält. Wenn Sie es stattdessen auf einen Marketplace-Root zeigen, liest Claude Code `marketplace.json` nicht, sodass ein Plugin unter `plugins/` nicht geladen wird, und Sie sehen keinen Fehler. Zeigen Sie das Flag auf den Ordner eines Plugins, oder fügen Sie den Marketplace hinzu. Siehe [den Eintrag zur Fehlerbehebung](/docs/de/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components).

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  Das Plugin wird geladen, aber seine Skills fehlen
</h3>

Das Verzeichnis `skills/` befindet sich in `.claude-plugin/`, oder ein `skills`-Eintrag im Manifest zeigt auf eine Datei. Verschieben Sie `skills/` zum Plugin-Root, zeigen Sie jeden `skills`-Eintrag auf ein Verzeichnis, das `SKILL.md` enthält, und führen Sie `/reload-plugins` in der Sitzung aus. Siehe [Plugin wird geladen, aber seine Skills fehlen](/docs/de/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing).

<h3 id="the-userconfig-dialog-never-appears">
  Der `userConfig`-Dialog wird nie angezeigt
</h3>

Der Dialog für die [`userConfig`](/docs/de/plugins/components#user-configuration)-Optionen Ihres Plugins ist Teil der Installation über `/plugin` in einer Sitzung. Das Laden mit `--plugin-dir` zeigt ihn nicht, und auch nicht `claude plugin install` in der Shell. Führen Sie mit dem geladenen Plugin `/plugin configure <plugin-name>` in der Sitzung aus, um ihn zu öffnen. Siehe [Der `userConfig`-Dialog wird nie angezeigt](/docs/de/plugins/troubleshooting#the-userconfig-dialog-never-appears).

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  Überprüfen Sie, dass das Plugin Claudes Verhalten ändert
</h3>

Ein Plugin, das ohne Fehler geladen wird, kann immer noch nicht steuern, wie Claude auf die beabsichtigte Weise funktioniert. `claude plugin eval`, das Sie in Ihrer Shell ausführen, führt Ihre Testfälle mit und ohne das Plugin aus und bewertet den Unterschied. Siehe [Testen Sie Plugins mit Evals](/docs/de/plugin-evals), beginnend mit [Erstellen Sie Ihre erste Eval-Suite](/docs/de/plugin-evals#create-your-first-eval-suite).

<h2 id="convert-an-existing-claude-setup">
  Konvertieren Sie ein vorhandenes `.claude/`-Setup
</h2>

Wenn Sie bereits Skills, Agents oder Hooks unter dem `.claude/`-Verzeichnis eines Projekts haben, können Sie sie in ein Plugin verschieben, ohne sie umzuschreiben.

Führen Sie die Befehle in diesen Schritten vom Projekt-Root aus, das das Verzeichnis ist, das `.claude/` enthält, da die `cp`-Pfade relativ dazu sind.

<Steps>
  <Step title="Erstellen Sie die Plugin-Struktur">
    Erstellen Sie das Plugin-Verzeichnis und seinen `.claude-plugin/`-Ordner neben `.claude/`. Sie können das Plugin danach überall verschieben.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Erstellen Sie `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Kopieren Sie Ihre vorhandenen Dateien">
    Kopieren Sie jedes Konfigurationsverzeichnis, das Sie haben, zum Plugin-Root, und überspringen Sie den Befehl für jedes Verzeichnis, das Sie nicht haben.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    Führen Sie `ls -a my-plugin` aus, um zu bestätigen, dass jedes Verzeichnis, das Sie kopiert haben, neben `.claude-plugin` angezeigt wird.
  </Step>

  <Step title="Verschieben Sie Ihre Hooks">
    Wenn Sie Hooks in `.claude/settings.json` oder `.claude/settings.local.json` haben, erstellen Sie ein Hooks-Verzeichnis:

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    Erstellen Sie `my-plugin/hooks/hooks.json` und kopieren Sie das `hooks`-Objekt aus Ihrer Einstellungsdatei hinein. Das Format ist das gleiche.

    Dieses Beispiel zeigt die Form mit einem Hook, der einen Linter auf jede Datei ausführt, die Claude schreibt oder bearbeitet. Ersetzen Sie das Beispiel durch Ihr eigenes `hooks`-Objekt.

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Testen Sie das migrierte Plugin">
    Laden Sie das Plugin für eine Sitzung:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Überprüfen Sie jede Komponente unter ihrem neuen Namen:

    * **Skills**: führen Sie `/my-plugin:deploy` für einen Skill aus, der `/deploy` war.
    * **Subagents**: bitten Sie Claude, den `my-plugin:reviewer`-Agent für einen Agent zu verwenden, der `reviewer` war.
    * **Hooks**: lösen Sie das Ereignis aus, das jeder Hook abgleicht.

    Wenn etwas fehlt, arbeiten Sie sich durch [Testen und Debuggen](#test-and-debug).
  </Step>
</Steps>

Während die Originale noch unter `.claude/` sind, bleiben sie neben den Kopien des Plugins geladen:

* **Skills und Agents**: die beiden Sätze kollidieren nicht, da die Skills und Agents des Plugins das Präfix `my-plugin:` tragen. `/deploy` und `/my-plugin:deploy` funktionieren beide, und Claude sieht `reviewer` und `my-plugin:reviewer` als zwei Subagents.
* **Hooks**: Hooks haben kein Präfix, sodass ein Hook, der sowohl in Ihrer Einstellungsdatei als auch in `hooks/hooks.json` ist, jedes Mal zweimal ausgeführt wird, wenn sein Ereignis auslöst.

Nachdem Sie bestätigt haben, dass das Plugin funktioniert, löschen Sie die Originale aus `.claude/` und entfernen Sie das `hooks`-Objekt aus Ihrer Einstellungsdatei.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Plugin-Komponenten](/docs/de/plugins/components): fügen Sie Agents, Hooks, MCP-Server, LSP-Server und Benutzerkonfiguration zu Ihrem Plugin hinzu
* [Testen Sie Plugins mit Evals](/docs/de/plugin-evals): schreiben Sie Eval-Fälle und führen Sie sie mit `claude plugin eval` aus, um zu überprüfen, wie zuverlässig das Plugin Claudes Verhalten steuert
* [Veröffentlichen Sie ein Plugin](/docs/de/plugins/publish): versionieren Sie es, setzen Sie es in einen Marketplace und reichen Sie es beim Community-Marketplace ein
* [Plugins auf claude.ai und in Cowork](https://claude.com/docs/plugins/overview): derselbe Plugin-Ordner wird auf claude.ai und in Cowork installiert. Einige Komponenten sind nur für Claude Code
* [Plugin-Manifest-Referenz](/docs/de/plugins/manifest-reference): jedes `plugin.json`-Feld, jede Pfadregel und jedes Verzeichnis
* [Skills](/docs/de/skills): schreiben Sie die Skills, die Ihr Plugin bereitstellt
* [Anthropics Plugins im claude-code-Repository](https://github.com/anthropics/claude-code/tree/main/plugins): vollständige durchgearbeitete Beispiele des Layouts auf dieser Seite, z. B. `feature-dev` und `code-review`
