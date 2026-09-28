> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Berechtigungen konfigurieren

> Kontrollieren Sie, worauf Claude Code zugreifen kann und was es mit granularen Berechtigungsregeln, Modi und verwalteten Richtlinien tun kann.

Claude Code unterstützt granulare Berechtigungen, sodass Sie genau angeben können, was der Agent tun darf und was nicht. Sie können Berechtigungseinstellungen in die Versionskontrolle einchecken, um sie mit jedem Entwickler in Ihrer Organisation zu teilen, und jeder Entwickler kann seine eigenen Einstellungen anpassen.

<h2 id="permission-system">
  Berechtigungssystem
</h2>

Claude Code verwendet ein gestuftes Berechtigungssystem, um Leistung und Sicherheit auszugleichen. Die Tabelle zeigt für jeden Werkzeugtyp, ob der manuelle Modus vor der Ausführung der Aktion fragt. Die anderen [Berechtigungsmodi](#permission-modes) ändern, welche dieser Aufforderungen Sie sehen; im automatischen Modus überprüft ein Klassifizierer Aktionen statt Ihnen, und [wie der Klassifizierer Aktionen bewertet](/docs/de/permission-modes#how-the-classifier-evaluates-actions) listet auf, welche er sieht.

| Werkzeugtyp   | Beispiel                     | Genehmigung erforderlich                                                                                                     | Verhalten „Ja, nicht mehr fragen"           |
| :------------ | :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------ |
| Nur Lesen     | Dateilesevorgänge, Grep      | Nein, innerhalb des [Arbeitsverzeichnisses und zusätzlicher Verzeichnisse](#working-directories)                             | N/A                                         |
| Bash-Befehle  | Shell-Ausführung             | Ja, außer einer integrierten Reihe von [schreibgeschützten Befehlen](#read-only-commands)                                    | Dauerhaft pro Projektverzeichnis und Befehl |
| Dateiänderung | Dateien bearbeiten/schreiben | Ja                                                                                                                           | Bis zum Ende der Sitzung                    |
| Web-Abruf     | WebFetch                     | Ja, außer einer integrierten Reihe von [vorab genehmigten Dokumentationsdomänen](/docs/de/tools-reference#webfetch-tool-behavior) | Dauerhaft pro Projektverzeichnis und Domäne |
| Websuche      | WebSearch                    | Ja                                                                                                                           | Dauerhaft pro Projektverzeichnis            |

Wenn Sie „Ja, nicht mehr fragen" wählen und die Genehmigung dauerhaft gespeichert wird, z. B. für einen Bash-Befehl oder eine WebFetch-Domäne, speichert Claude Code die Regel in `.claude/settings.local.json` im Stammverzeichnis des Git-Repositorys, aufgelöst durch [Worktrees](/docs/de/worktrees) zum Haupt-Checkout. Die Regel gilt für zukünftige Sitzungen überall in diesem Projektverzeichnis, einschließlich Sitzungen, die in Unterverzeichnissen und in Worktrees gestartet werden. Eine Dateiänderungsgenehmigung wird nicht in der Datei gespeichert: wie die Tabelle zeigt, gilt sie bis zum Ende der Sitzung. In einigen Fällen, z. B. außerhalb eines Git-Repositorys oder unter Windows, verwendet Claude Code nicht das Projektverzeichnis-Stammverzeichnis; [Wo Claude Code nach jeder Datei sucht](/docs/de/settings#where-claude-code-looks-for-each-file) listet diese Fälle auf und wo es die Regel stattdessen speichert.

Vor v2.1.211 speicherte Claude Code die Regel immer im Startverzeichnis, sodass eine in einem Worktree oder Unterverzeichnis gewährte Genehmigung nicht auf den Rest des Repositorys angewendet wurde. Regeln, die frühere Versionen in einem Unterverzeichnis oder Worktree gespeichert haben, gelten weiterhin für Sitzungen, die dort gestartet werden.

Manchmal bietet eine Berechtigungsaufforderung nur eine einmalige Genehmigung an, ohne Option „nicht mehr fragen" und ohne Option, die Aktion für den Rest der Sitzung zuzulassen. Claude Code bietet diese Optionen nur an, wenn die Aufforderung Ihnen alles zeigen kann, was sie zulassen würden, sodass eine Regel, die Sie aus einer Aufforderung speichern, nur das abdeckt, was ihre benannte Option zulässt. Wenn eine Aufforderung nur die einmalige Genehmigung bietet, genehmigen Sie die Aktion einmal, oder fügen Sie die Regel selbst in [`/permissions`](#manage-permissions) hinzu.

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  Fügen Sie einen Kommentar hinzu, wenn Sie auf eine Berechtigungsaufforderung antworten
</h3>

Sie können Claude eine Notiz anhängen, wenn Sie eine einzelne Aktion genehmigen oder ablehnen. Bei den meisten Berechtigungsaufforderungen, einschließlich Bash-, PowerShell-, Datei- und MCP-Tool-Aufforderungen, wechseln Sie zu **Ja** oder **Nein** und drücken `Tab`, um ein Kommentarfeld für diese Option zu öffnen. WebFetch- und Browser-Aufforderungen bieten das Feld nicht an. Die Optionen, die die Aktion für den Rest der Sitzung zulassen oder eine Regel speichern, akzeptieren auch keine.

Wenn das Feld offen ist, geben Sie den Kommentar ein und drücken dann eine dieser Tasten:

* `Enter`: sendet Ihre Antwort mit dem angehängten Kommentar. Wenn Sie das Feld leer lassen, sendet Claude Code die Antwort ohne Kommentar.
* `Tab`: schließt das Feld ohne Antwort. Claude Code behält den eingegebenen Text und sendet ihn immer noch, wenn Sie mit dieser Option antworten.
* `Shift+Tab`: bei einer Datei-Aufforderung, z. B. einer Bearbeitungs- oder Schreib-Aufforderung, schließt das Feld genauso wie `Tab`. Vor v2.1.235 wählte das Drücken von `Shift+Tab` im Feld stattdessen die Option aus, die die Aktion für den Rest der Sitzung zulässt, sodass Claude Code die Aktion für den Rest der Sitzung genehmigte und den Kommentar verwarf.

Claude Code liefert den Kommentar unterschiedlich, je nachdem, wie Sie geantwortet haben:

* **Ja**: Claude Code führt die Aktion aus und sendet dann Ihren Kommentar an Claude nach dem Ergebnis.
* **Nein**: Claude Code sendet Ihren Kommentar an Claude als Grund für die Ablehnung, und Claude arbeitet weiter. Wenn Sie **Nein** ohne Kommentar bei einer Aufforderung aus der Hauptkonversation wählen, stoppt Claude Code den Zug.

<h2 id="manage-permissions">
  Berechtigungen verwalten
</h2>

Sie können Claude Code's Werkzeugberechtigungen mit `/permissions` anzeigen und verwalten. Diese Benutzeroberfläche listet alle Berechtigungsregeln und die `settings.json`-Dateien auf, aus denen sie stammen. Sie können den Dialog öffnen, während Claude arbeitet: Wenn Sie eine Regel hinzufügen oder entfernen, wendet Claude Code die Änderung ab Claude's nächstem Werkzeugaufruf in demselben Zug an. Vor v2.1.234 stellte Claude Code den Befehl in die Warteschlange, bis der Zug beendet war.

* **Allow**-Regeln ermöglichen Claude Code, das angegebene Werkzeug ohne manuelle Genehmigung zu verwenden.
* **Ask**-Regeln fordern eine Bestätigung auf, wenn Claude Code versucht, das angegebene Werkzeug zu verwenden.
* **Deny**-Regeln verhindern, dass Claude Code das angegebene Werkzeug verwendet.

Regeln werden in dieser Reihenfolge ausgewertet: deny, dann ask, dann allow. Die erste Übereinstimmung in dieser Reihenfolge bestimmt das Ergebnis, und die Regelspezifität ändert die Reihenfolge nicht.

Eine breite deny-Regel wie `Bash(aws *)` blockiert jeden übereinstimmenden Aufruf, einschließlich Aufrufen, die auch einer engeren allow-Regel wie `Bash(aws s3 ls)` entsprechen. Eine allow-Regel kann keine Ausnahme aus einer deny-Regel herausschneiden. Die gleiche Priorität gilt zwischen ask und allow: eine übereinstimmende ask-Regel fordert eine Bestätigung auf, auch wenn eine spezifischere allow-Regel denselben Aufruf ebenfalls erfüllt.

Deny-Regeln verhalten sich unterschiedlich, je nachdem, ob sie ein Werkzeug benennen oder ein Muster darin eingrenzen. Ein einfacher Werkzeugname wie `Bash` entfernt das Werkzeug vollständig aus Claudes Kontext, sodass Claude es nie sieht. Wenn Sie eine solche Regel während einer Sitzung hinzufügen, kann Claude das Werkzeug ab seinem nächsten Werkzeugaufruf nicht mehr aufrufen; [Ein ganzes Werkzeug ablehnen](/docs/de/prompt-caching#denying-an-entire-tool) behandelt, was mit einer Definition geschieht, die Claude bereits gesehen hat. Eine eingegrenzte Regel wie `Bash(rm *)` lässt das Werkzeug verfügbar und blockiert übereinstimmende Aufrufe, wenn Claude sie versucht.

Die Entfernung mit einfachem Namen gilt für jedes Werkzeug außer [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior): eine deny-Regel kann es nicht entfernen, während ein anderes Werkzeug verbleibt, und eine ask-Regel fordert nie eine Bestätigung dafür auf.

<Note>
  Berechtigungsregeln werden von Claude Code durchgesetzt, nicht vom Modell. Anweisungen in Ihrem Prompt oder `CLAUDE.md` bestimmen, was Claude versucht zu tun, aber sie ändern nicht, was Claude Code erlaubt. Um Zugriff zu gewähren oder zu widerrufen, verwenden Sie `/permissions`, die hier beschriebenen Regeln, einen [Berechtigungsmodus](/docs/de/permission-modes) oder einen [PreToolUse-Hook](#extend-permissions-with-hooks).
</Note>

Wenn der [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) für Ihre Sitzung verfügbar ist, enthält der Dialog auch die [Auto-Modus-Klassifiziererregeln](/docs/de/auto-mode-config#edit-rules-from-permissions). Wählen Sie die Registerkarte **Auto mode** aus, um sie anzuzeigen.

<h2 id="permission-modes">
  Berechtigungsmodi
</h2>

Claude Code unterstützt mehrere Berechtigungsmodi, die steuern, wie Werkzeugaufrufe genehmigt werden. Siehe [Berechtigungsmodi](/docs/de/permission-modes) für den Zeitpunkt der Verwendung jedes Modus. Um den Modus zu ändern, in dem Sitzungen starten, legen Sie `defaultMode` in Ihren [Einstellungsdateien](/docs/de/settings#where-settings-live) fest. [Welcher Modus eine Sitzung startet](/docs/de/permission-modes#which-mode-a-session-starts-in) behandelt die integrierte Standardeinstellung für jeden Plan und was die VS Code-Erweiterung liest.

| Modus               | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Fordert Genehmigung bei der ersten Verwendung jedes Werkzeugs auf. Im CLI, in den VS Code- und JetBrains-Erweiterungen sowie in der Desktop-App als „Manual" gekennzeichnet, und Claude Code akzeptiert `manual` als Alias. Die Bezeichnung und der Alias erfordern Claude Code v2.1.200 oder später. Die Bezeichnung der Desktop-App hängt nicht von Ihrer CLI-Version ab                                                                                                                                                                                                                                                                                                                                        |
| `acceptEdits`       | Akzeptiert automatisch Dateibearbeitungen und häufige Dateisystem-Befehle wie `mkdir`, `touch`, `mv` und `cp` für Pfade im Arbeitsverzeichnis oder `additionalDirectories`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `plan`              | Claude liest Dateien und führt schreibgeschützte Shell-Befehle aus, um zu erkunden, bearbeitet aber nicht Ihre Quelldateien; mit [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) verfügbar, Klassifizierer-genehmigte Befehle werden auch ausgeführt. Im CLI und in der VS Code-Erweiterung als Plan gekennzeichnet                                                                                                                                                                                                                                                                                                                                                                           |
| `auto`              | Genehmigt Werkzeugaufrufe automatisch mit Hintergrund-Sicherheitsprüfungen, die überprüfen, ob Aktionen mit Ihrer Anfrage übereinstimmen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `dontAsk`           | Verweigert automatisch jeden Aufruf, der sonst eine Aufforderung auslösen würde; Dateilesevorgänge in Ihren Arbeitsverzeichnissen und andere Aktionen, die keine Genehmigung benötigen, werden weiterhin ausgeführt, ebenso wie Werkzeuge, die vorab über `/permissions` oder `permissions.allow`-Regeln genehmigt wurden. `AskUserQuestion`, MCP-Werkzeuge, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, und Connector-Werkzeuge [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) in Sitzungen, in denen diese Einstellung Claude Code erreicht, werden verweigert, auch wenn Sie diese genehmigt haben |
| `bypassPermissions` | Überspringt Berechtigungsaufforderungen, außer für die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

<Warning>
  Der Modus `bypassPermissions` überspringt Berechtigungsaufforderungen, einschließlich Schreibvorgänge in [geschützten Pfaden](/docs/de/permission-modes#protected-paths) wie `.git` und `.claude`. Die [Schutzmaßnahmen für sitzungsübergreifendes Messaging](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) gelten weiterhin. Verwenden Sie diesen Modus nur in isolierten Umgebungen wie Containern oder VMs, in denen Claude Code keinen Schaden anrichten kann.
</Warning>

Um zu verhindern, dass der Modus `bypassPermissions` oder `auto` verwendet wird, legen Sie `permissions.disableBypassPermissionsMode` oder `permissions.disableAutoMode` auf `"disable"` in einer beliebigen [Einstellungsdatei](/docs/de/settings#where-settings-live) fest. Diese sind am nützlichsten in [verwalteten Einstellungen](#managed-settings), wo sie nicht überschrieben werden können.

<h2 id="permission-rule-syntax">
  Berechtigungsregelsyntax
</h2>

Berechtigungsregeln folgen dem Format `Tool` oder `Tool(specifier)`. Klammern innerhalb des Spezifizierers sind literal, daher benötigt ein Befehl oder Pfad, der sie enthält, keine Escapezeichen.

<h3 id="match-all-uses-of-a-tool">
  Alle Verwendungen eines Werkzeugs abgleichen
</h3>

Um alle Verwendungen eines Werkzeugs abzugleichen, verwenden Sie einfach den Werkzeugnamen ohne Klammern:

| Regel      | Effekt                             |
| :--------- | :--------------------------------- |
| `Bash`     | Gleicht alle Bash-Befehle ab       |
| `WebFetch` | Gleicht alle Web-Fetch-Anfragen ab |
| `Read`     | Gleicht alle Dateilesevorgänge ab  |

`Bash(*)` ist gleichwertig mit `Bash` und gleicht alle Bash-Befehle ab. Als Ablehnungsregel entfernen beide Formen das Werkzeug aus Claudes Kontext.

<h3 id="use-specifiers-for-fine-grained-control">
  Verwenden Sie Spezifizierer für granulare Kontrolle
</h3>

Fügen Sie einen Spezifizierer in Klammern hinzu, um bestimmte Werkzeugverwendungen abzugleichen:

| Regel                          | Effekt                                                         |
| :----------------------------- | :------------------------------------------------------------- |
| `Bash(npm run build)`          | Gleicht den genauen Befehl `npm run build` ab                  |
| `Read(./.env)`                 | Gleicht das Lesen der `.env`-Datei im aktuellen Verzeichnis ab |
| `WebFetch(domain:example.com)` | Gleicht Fetch-Anfragen an example.com ab                       |

<h3 id="match-by-input-parameter">
  Abgleich nach Eingabeparameter
</h3>

Ablehnungs- und Anfrage-Regeln können einen Eingabeparameter auf oberster Ebene auf jedem integrierten Werkzeug mit `Tool(param:value)` abgleichen.

Um einen Parameter auf einem MCP-Werkzeug abzugleichen, übergeben Sie eine Ablehnungsregel mit [`--disallowedTools`](/docs/de/cli-reference#cli-flags). Wenn Claude Code eine Einstellungsdatei lädt, überspringt es alle `mcp__`-Regeln, die Klammern haben. Claude Code listet die übersprungene Regel im Dialog für ungültige Einstellungen auf, wenn eine interaktive Sitzung startet, und in der Ausgabe von [`claude doctor`](/docs/de/debug-your-config#check-resolved-settings).

Eine Parameterregel passt, wenn Claude das Werkzeug mit diesem Parameter aufruft, der auf diesen genauen Wert gesetzt ist. Eine Zulassungsregel für einen Parameterwert würde nicht feststellen, dass der Aufruf insgesamt sicher ist, daher verwenden Zulassungsregeln weiterhin die eigene Spezifizierer-Syntax jedes Werkzeugs. Dies funktioniert für jeden Skalarparameter, den das Werkzeug akzeptiert:

| Regel                          | Passt                                              |
| :----------------------------- | :------------------------------------------------- |
| `Agent(model:opus)`            | Agent-Aufrufe, die das Opus-Modell-Tier anfordern  |
| `Agent(isolation:worktree)`    | Agent-Aufrufe, die ein Git-Worktree anfordern      |
| `Bash(run_in_background:true)` | Bash-Aufrufe, die im Hintergrund ausgeführt werden |

Der Parameterabgleich folgt diesen Regeln:

* Der Parametername muss ein direktes Feld der Werkzeugeingabe sein, wie `model` auf dem Agent-Werkzeug. Felder, die in einem Objekt oder Array verschachtelt sind, können nicht abgeglichen werden
* Jede Regel benennt einen Parameter. Um sowohl `model` als auch `isolation` zu steuern, schreiben Sie zwei Regeln, `Agent(model:opus)` und `Agent(isolation:worktree)`, anstatt sie in einer Regel zu kombinieren
* Der Wert unterstützt `*` als Platzhalter, der jede Zeichenfolge abgleicht, daher gleicht `Agent(isolation:*)` jeden expliziten Isolationswert ab. Ohne `*` ist der Abgleich exakt
* Ein Parameter, den das Modell auslässt, wird nie abgeglichen, daher gleicht `Agent(model:*)` einen Aufruf nicht ab, der `model` nicht gesetzt lässt
* Der Wert wird mit der literalen Eingabe verglichen, die Claude sendet, bevor eine Normalisierung erfolgt. `Agent(model:opus)` gleicht den Alias `opus` ab, aber nicht eine vollständige Modell-ID. Führen Sie mit [`--verbose`](/docs/de/cli-reference) aus, um die genauen Parameternamen und Werte in jedem Werkzeugaufruf zu sehen
* Leerzeichen um den Doppelpunkt werden ignoriert

Sie können ein primäres Inhaltsfeld eines Werkzeugs auf diese Weise nicht abgleichen: `command` für Bash und PowerShell, `file_path` für Read, Edit und Write, `path` für Grep und Glob, `notebook_path` für NotebookEdit und `url` für WebFetch. Eine Regel wie `Bash(command:rm *)` könnte durch einen zusammengesetzten Befehl umgangen werden, daher ignoriert Claude Code sie und gibt eine Startwarnmeldung aus. Verwenden Sie stattdessen `Bash(rm *)`, `Read(./path)` oder `WebFetch(domain:host)`.

<h3 id="wildcard-patterns">
  Wildcard-Muster
</h3>

Ein `*` in einer Bash-Regel gleicht jeden Text ab, einschließlich Leerzeichen, daher deckt eine Regel eine Familie von Befehlen ab. Eine Regel ohne `*` gleicht einen genauen Befehl ab.

<Warning>
  Setzen Sie das `*` nach dem Unterbefehl. In `git log --oneline main` ist `git` das Programm und `log` ist der Unterbefehl, das Wort, das bestimmt, was das Programm tut. Claude Code gleicht alles vor dem ersten `*` wie geschrieben ab, daher sind diese Wörter das, was die Regel einschränkt: `Bash(git log *)` erlaubt nur `git log`-Befehle, und `Bash(git *)` erlaubt jeden git-Befehl. Claude Code [warnt beim Start](/docs/de/errors#has-a-wildcard-before-the-rest-of-the-command) vor einer Zulassungsregel mit einem `*` vor dem Unterbefehl, wie `Bash(git * main)`.
</Warning>

Schreiben Sie den Befehl, den Claude ausführen soll, ohne zu fragen, und ersetzen Sie die Teile, die variieren, durch `*`. Mit dieser Konfiguration führt Claude Code npm-Skripte und git-Commits aus, ohne zu fragen, und lehnt Befehle ab, die mit `git push` beginnen. Ein Push, der auf andere Weise geschrieben ist, wie `git -C . push`, wird nicht abgeglichen; siehe [was eine Bash-Regel nicht abgleicht](#bash-rule-limits).

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Ein `*` kann überall in der Regel stehen: am Anfang, in der Mitte oder am Ende. Jede Zeile zeigt eine Regel, Befehle, die sie abgleicht, und nahegelegene Befehle, die sie nicht abgleicht:

| Sie schreiben          | Gleicht ab                                                                           | Gleicht nicht ab                       |
| :--------------------- | :----------------------------------------------------------------------------------- | :------------------------------------- |
| `Bash(npm run build)`  | `npm run build`                                                                      | `npm run build --watch`                |
| `Bash(npm run *)`      | `npm run build`, `npm run test --watch`, `npm run`                                   | `npm install`                          |
| `Bash(git log * main)` | `git log --oneline main`, `git log -5 main`, `git log --output=<file> main`          | `git log main`, `git push origin main` |
| `Bash(git * main)`     | `git merge main`, `git push origin main`, `git -c core.fsmonitor=<script> diff main` | `git log`                              |
| `Bash(* --version)`    | `node --version`, `bash -c 'echo hi' --version`                                      | `node -v`                              |
| `Bash(ls *)`           | `ls -la`, `ls`                                                                       | `lsof`                                 |
| `Bash(ls*)`            | `ls -la`, `lsof`                                                                     |                                        |
| `Bash(* --help *)`     | `npm --help x`                                                                       | `npm --help`                           |

Drei Abgleichsregeln erzeugen diese Zeilen:

* **Das `*` steht für den Text an seiner Stelle.** In `Bash(git * main)` steht es für den Unterbefehl, daher gleicht Claude Code jeden git-Unterbefehl und jede Option davor ab. Das schließt `-c` ein, das git veranlasst, ein Programm auszuführen, das Sie benennen. In `Bash(* --version)` steht das `*` für das Programm, daher passt jedes Programm.
* **Ein `*` am Ende mit einem Leerzeichen davor gleicht auch den bloßen Befehl ab.** `Bash(ls *)` gleicht `ls` ab, und `Bash(git log *)` gleicht `git log` ab. Das gilt nur, wenn das nachgestellte `*` der einzige Platzhalter der Regel ist: `Bash(* --help *)` gleicht `npm --help x` ab, aber nicht `npm --help`.
* **Das Leerzeichen vor einem nachgestellten `*` ist Teil der Regel.** `Bash(ls *)` erfordert ein Leerzeichen nach `ls`, daher passt `lsof` nicht. `Bash(ls*)` hat kein Leerzeichen, daher passt es auch zu `lsof`.

Das Suffix `:*` ist eine gleichwertige Möglichkeit, einen nachgestellten Platzhalter zu schreiben, daher gleicht `Bash(ls:*)` die gleichen Befehle ab wie `Bash(ls *)`.

Der Berechtigungsdialog schreibt die durch Leerzeichen getrennte Form, wenn Sie „Ja, nicht mehr fragen" für ein Befehlspräfix auswählen. Die Form `:*` wird nur am Ende eines Musters erkannt. In einem Muster wie `Bash(git:* push)` wird der Doppelpunkt als Literalzeichen behandelt und passt nicht zu git-Befehlen.

<h3 id="tool-name-wildcards">
  Werkzeugnamen-Wildcards
</h3>

Ablehnungs- und Anfrage-Regeln akzeptieren auch Glob-Muster in der Werkzeugnamen-Position. Das Muster muss dem vollständigen Werkzeugnamen entsprechen: `"*"` gleicht jedes Werkzeug ab, und `"mcp__*"` gleicht jedes MCP-Werkzeug über alle Server hinweg ab. Ein Werkzeug, das durch eine Ablehnungsregel mit bloßem Namen abgeglichen wird, wird aus Claudes Kontext entfernt, genauso wie ein bloßer Werkzeugname, einschließlich der [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior)-Ausnahme: eine Glob-Ablehnung kann sie nicht entfernen, während ein anderes Werkzeug verbleibt, und eine Glob-Anfrage fordert sie nie auf. Diese Konfiguration lehnt jedes MCP-Werkzeug ab:

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

Zulassungsregeln akzeptieren Werkzeugnamen-Globs nur nach einem literalen `mcp__<server>__`-Präfix. Das Server-Segment muss glob-frei sein, damit die Regel einen bestimmten Server benennt, den Sie konfiguriert haben. `mcp__puppeteer__*` gleicht jedes Werkzeug vom `puppeteer`-Server ab, und `mcp__github__get_*` gleicht seine `get_`-Werkzeuge ab. Ein unverankerte Zulassungs-Glob wie `"*"`, `"B*"` oder `"mcp__*"` wird mit einer Warnung übersprungen und genehmigt nichts automatisch.

Eine Ablehnungs- oder Anfrage-Regel, deren Werkzeugname mit keinem bekannten Werkzeug übereinstimmt, erzeugt eine Startwarnmeldung, um Tippfehler zu erfassen. Werkzeugnamen, die `_` oder `*` enthalten, sind von der Überprüfung ausgenommen, und ebenso die Namen von Werkzeugen, die Claude Code entfernt hat, wie `TaskOutput`.

Das Etikett, das für ein Werkzeug im Transkript und im Berechtigungsdialog angezeigt wird, kann sich vom kanonischen Namen unterscheiden. Beispielsweise hat das Werkzeug mit der Bezeichnung `Stop Task` im Transkript den kanonischen Namen `TaskStop`. Berechtigungsregeln und [Hook-Matcher](/docs/de/hooks) gleichen das Etikett nicht ab, daher passt eine Regel, die als `Stop Task` geschrieben ist, nicht. Für Ablehnungs- und Anfrage-Regeln erfasst die obige Startwarnmeldung die Nichtübereinstimmung. Verwenden Sie die kanonischen Namen, die in der [Werkzeugreferenz](/docs/de/tools-reference) aufgelistet sind.

<h2 id="tool-specific-permission-rules">
  Werkzeugspezifische Berechtigungsregeln
</h2>

<h3 id="bash">
  Bash
</h3>

Bash-Berechtigungsregeln gleichen den gesamten Befehlstext ab, wobei `*` für beliebigen Text steht. [Wildcard-Muster](#wildcard-patterns) zeigt, welche Befehle jede Regelform abgleicht und wo `*` platziert werden sollte. Der Rest dieses Abschnitts behandelt, wie Claude Code zusammengesetzte Befehle und Wrapper abgleicht, was eine Regel nicht abgleicht, schreibgeschützte Befehle und Umleitungen.

<h4 id="compound-commands">
  Zusammengesetzte Befehle
</h4>

<Tip>
  Claude Code ist sich Shell-Operatoren bewusst, daher gibt eine Regel wie `Bash(safe-cmd *)` ihm nicht die Berechtigung, den Befehl `safe-cmd && other-cmd` auszuführen. Die erkannten Befehlstrennzeichen sind `&&`, `||`, `;`, `|`, `|&`, `&` und Zeilenumbrüche. Eine Regel muss jeden Unterbefehl unabhängig abgleichen.
</Tip>

Deny- und Ask-Regeln gelten, wenn ein beliebiger Unterbefehl sie abgleicht, einschließlich eines Befehls, der in einer Subshell verschachtelt ist, einer Befehlsersetzung oder eines Kontrollfluss-Bodys wie einer `for`-Schleife. Eine Ask-Regel wie `Bash(git clean *)` fordert Sie immer noch auf für `cd /tmp && git clean -f` oder `echo "$(git clean -f)"`, auch im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode).

Wenn `&&` oder `||` nichts danach hat, wie in `npm test &&`, behandelt Claude Code den Befehl als nicht analysierbar und teilt ihn nicht in Unterbefehle für Allow-Regel-Abgleich auf, daher genehmigt eine Regel wie `Bash(npm *)` ihn nicht.

Wenn Sie einen zusammengesetzten Befehl mit „Ja, nicht mehr fragen" genehmigen, speichert Claude Code eine separate Regel für jeden Unterbefehl, der Genehmigung erfordert, anstelle einer einzelnen Regel für die vollständige zusammengesetzte Zeichenkette. Zum Beispiel speichert das Genehmigen von `git status && npm test` eine Regel für `npm test`, sodass zukünftige `npm test`-Aufrufe erkannt werden, unabhängig davon, was dem `&&` vorausgeht. Unterbefehle wie `cd` in ein Unterverzeichnis generieren ihre eigene Read-Regel für diesen Pfad. Für einen einzelnen zusammengesetzten Befehl können bis zu 5 Regeln gespeichert werden.

<h4 id="process-wrappers">
  Wrapper
</h4>

Vor dem Abgleich von Bash-Regeln entfernt Claude Code einen festen Satz von Wrappern, daher gleicht eine Regel wie `Bash(npm test *)` auch `timeout 30 npm test` ab. Die entfernten Wrapper sind `timeout`, `time`, `nice`, `nohup` und `stdbuf`, plus die Shell-Builtins `command` und `builtin` sowie zsh's `noglob`. Jeder führt sein Argument als den tatsächlichen Befehl aus. Zwei verwandte Formen werden nicht entfernt: die Abfrageform `command -v`, die einen Befehl nachschlägt, anstatt ihn auszuführen, und zsh's `nocorrect`.

Claude Code entfernt auch eine führende Zuweisung bestimmter bekannter sicherer Umgebungsvariablen, daher gleicht `Bash(npm test *)` `NODE_ENV=test npm test` ab. Eine Allow-Regel gleicht nicht über eine Zuweisung einer anderen Variablen hinweg ab. Eine Deny- oder Ask-Regel gleicht über jede führende Zuweisung hinweg ab, daher gleicht `Bash(rm *)` in Deny immer noch `FOO=bar rm -rf tmp/` ab.

Bloßes `xargs` wird auch entfernt, daher gleicht `Bash(grep *)` `xargs grep pattern` ab. Das Entfernen gilt nur, wenn `xargs` keine Flags hat: Ein Aufruf wie `xargs -n1 grep pattern` wird als `xargs`-Befehl abgeglichen, daher decken Regeln, die für den inneren Befehl geschrieben wurden, ihn nicht ab.

Diese Wrapper-Liste ist integriert und nicht konfigurierbar. Entwicklungsumgebungs-Runner wie `direnv exec`, `devbox run`, `mise exec`, `npx` und `docker exec` sind nicht in der Liste. Da diese Tools ihre Argumente als Befehl ausführen, gleicht eine Regel wie `Bash(devbox run *)` alles ab, was nach `run` kommt, einschließlich `devbox run rm -rf .`. Um Arbeit innerhalb eines Umgebungs-Runners zu genehmigen, schreiben Sie eine spezifische Regel, die sowohl den Runner als auch den inneren Befehl enthält, wie `Bash(devbox run npm test)`. Fügen Sie eine Regel pro innerem Befehl hinzu, den Sie zulassen möchten.

Exec-Wrapper wie `watch`, `setsid`, `ionice` und `flock` können nicht durch eine Präfixregel wie `Bash(watch *)` automatisch genehmigt werden, daher fordern sie im Manual-Modus immer auf. Das gleiche gilt für `find` mit `-exec` oder `-delete`: Eine `Bash(find *)` Regel deckt diese Formen nicht ab. Um einen spezifischen Aufruf zu genehmigen, schreiben Sie eine exakte Übereinstimmungsregel für die vollständige Befehlszeichenkette.

<h4 id="bash-rule-limits">
  Was eine Bash-Regel nicht abgleicht
</h4>

Eine Bash-Regel gleicht den Befehlstext ab, den Claude schreibt, nachdem Claude Code [zusammengesetzte Befehle](#compound-commands) aufgeteilt und [Wrapper](#process-wrappers) entfernt hat. Sie gleicht nicht das gleiche Programm ab, das in einer anderen Form aufgerufen wird, daher deckt eine Deny- oder Ask-Regel den Aufruf ab, den Claude normalerweise erzeugt, und ist keine Sicherheitsgrenze um das Programm. Diese Regeln in `deny` oder `ask` stoppen die erste Form und nicht die anderen:

| Regel              | Stoppt                     | Stoppt nicht                                                                                          |
| :----------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`, `sh -c 'curl https://example.com'`                               |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`, `bash -c 'rm -rf build/'`                                                       |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`, `git -c push.default=current push origin main`, `git 'push' origin main` |

Ihre anderen Regeln und der Berechtigungsmodus entscheiden über die Befehle in der letzten Spalte.

Für Dateisystem- und Netzwerkdurchsetzung, die nicht vom Befehlstext abhängt, verwenden Sie [Sandboxing](/docs/de/sandboxing). Um den vollständigen Befehlstext mit Ihrer eigenen Logik vor der Ausführung zu überprüfen, verwenden Sie einen [PreToolUse-Hook](#extend-permissions-with-hooks).

<h4 id="read-only-commands">
  Schreibgeschützte Befehle
</h4>

Claude Code erkennt einen integrierten Satz von Bash-Befehlen als schreibgeschützt und führt sie ohne Berechtigungsaufforderung in jedem Modus aus, außer für einen Pfad, den [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories) einschränkt. Der Satz umfasst `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` und schreibgeschützte Formen von `git`. Der Satz ist nicht konfigurierbar; um eine Aufforderung für einen dieser Befehle zu erfordern, fügen Sie eine `ask`- oder `deny`-Regel dafür hinzu. Im Auto-Modus können diese Befehle auch auf die Überprüfung durch den Klassifizierer warten; siehe [wie der Klassifizierer Aktionen bewertet](/docs/de/permission-modes#how-the-classifier-evaluates-actions).

Eine Umleitung wie `ls > out.txt` fügt eine Überprüfung des Ziels hinzu. Siehe [Umleitungen](#redirections).

Unquotierte Glob-Muster sind für Befehle zulässig, deren jedes Flag schreibgeschützt ist, daher laufen `ls *.ts` und `wc -l src/*.py` ohne Aufforderung.

Im Manual-Modus fordern Befehle aus diesem Satz immer noch in diesen Fällen auf:

* **Unquotierte Globs für Befehle mit schreibfähigen Flags**: Befehle mit schreibfähigen oder ausführungsfähigen Flags, wie `find`, `sort`, `sed` und `git`, fordern auf, wenn ein unquotiertes Glob vorhanden ist, da das Glob zu einem Flag wie `-delete` expandieren könnte.
* **`docker` auf einen anderen Daemon ausgerichtet**: Schreibgeschützte Formen von `docker` fordern auf, wenn der Befehl ein Flag trägt, das einen anderen Daemon auswählt, wie `-H`, `--context` oder Podman's `--url` und `--connection`.
* **`file` mit Pfad-öffnenden Flags**: `file` fordert auf, wenn es `-m`/`--magic-file` oder `-f`/`--files-from` übergibt, da diese Flags `file` veranlassen, die in der Flag-Wert benannten Pfade zu öffnen.
* **Netzwerkpfade unter Windows**: Ein Befehl, dessen Argumente einen Netzwerk-(UNC-)Pfad enthalten, wie `\\server\share\file`, fordert auf, da der Zugriff auf einen Netzwerkpfad Ihre Windows-Anmeldedaten an den Host senden kann, den er benennt. Die gleiche Überprüfung gilt für [PowerShell-Werkzeug](/docs/de/tools-reference#powershell-tool)-Befehle.
* **Befehle, die die Analyse nicht analysieren kann**: Wenn Claude Code einen Befehl nicht vollständig analysieren kann, fordert es zur Genehmigung auf, anstatt den Befehl als schreibgeschützt zu behandeln. Befehle, die länger als 10.000 Zeichen sind, fordern immer auf, da sie das überschreiten, was die Analyse analysiert.

Ein `cd` in einen Pfad innerhalb Ihres Arbeitsverzeichnisses oder eines [zusätzlichen Verzeichnisses](#working-directories) ist auch schreibgeschützt, und ein zusammengesetzter Befehl wie `cd packages/api && ls` läuft ohne Aufforderung, wenn jeder Teil auf eigene Faust qualifiziert. Diese Kombinationen fordern auf, auch wenn jeder Teil schreibgeschützt ist:

* **`cd` mit `git`**: fordert auf, wenn die `cd` in ein anderes Verzeichnis wechselt, da das Ausführen von `git` in einem neuen Verzeichnis die Hooks dieses Verzeichnisses ausführen kann. Ein `cd` dessen Ziel zum aktuellen Arbeitsverzeichnis aufgelöst wird, ist ein No-Op und löst die Aufforderung nicht aus.
* **`cd` mit einer Umleitung**: fordert auf, wenn Claude Code nicht bestimmen kann, in welches Verzeichnis das Umleitungsziel nach der Ausführung von `cd` aufgelöst wird. Ein Befehl, dessen einziges Umleitungsziel `/dev/null` ist, wie `cd app; grep -r pattern . 2>/dev/null`, fordert nicht auf, da `/dev/null` nicht vom Arbeitsverzeichnis abhängt.

<Warning>
  Bash-Berechtigungsmuster, die versuchen, Befehlsargumente einzuschränken, sind fragil. Zum Beispiel beabsichtigt `Bash(curl http://github.com/ *)`, curl auf GitHub-URLs zu beschränken, wird aber Variationen nicht abgleichen wie:

  * Optionen vor URL: `curl -X GET http://github.com/...`
  * Anderes Protokoll: `curl https://github.com/...`
  * Umleitungen: `curl -L http://short.example.com/xyz`, die zu GitHub umleitet
  * Variablen: `URL=http://github.com && curl $URL`

  Für zuverlässigere URL-Filterung sollten Sie erwägen:

  * **Bash-Netzwerkwerkzeuge einschränken**: Verwenden Sie Deny-Regeln, um `curl`, `wget` und ähnliche Befehle zu blockieren, verwenden Sie dann das WebFetch-Werkzeug mit `WebFetch(domain:github.com)`-Berechtigung für zulässige Domänen. Eine Deny-Regel gleicht nicht das gleiche Programm nach Pfad oder innerhalb von `sh -c` ab, daher kombinieren Sie es mit der [Sandbox-Netzwerk-Allowlist](/docs/de/sandboxing#network-isolation), wenn die Einschränkung gelten muss; siehe [was eine Bash-Regel nicht abgleicht](#bash-rule-limits)
  * **PreToolUse-Hooks verwenden**: Implementieren Sie einen Hook, der URLs in Bash-Befehlen validiert und nicht zulässige Domänen blockiert
  * **CLAUDE.md-Anleitung hinzufügen**: Beschreiben Sie Ihre zulässigen curl-Muster in `CLAUDE.md`. Dies beeinflusst, was Claude versucht, erzwingt aber keine Grenze, daher kombinieren Sie es mit einer der obigen Optionen

  Beachten Sie, dass die alleinige Verwendung von WebFetch keinen Netzwerkzugriff verhindert. Wenn Bash zulässig ist, kann Claude immer noch `curl`, `wget` oder andere Werkzeuge verwenden, um auf jede URL zuzugreifen.
</Warning>

<h4 id="redirections">
  Umleitungen
</h4>

Wenn ein Befehl Ausgabe oder Eingabe umleitet, überprüft Claude Code das Umleitungsziel gegen Ihre Dateiregelns, als hätte Claude diese Datei direkt geschrieben oder gelesen:

* **Ausgabeumleitungen**: für `> file`, `>> file` oder `2> file` deckt die Überprüfung Ihre `Edit` Allow- und Deny-Regeln, [geschützte Pfade](/docs/de/permission-modes#protected-paths) und die [Arbeitsverzeichnisse](#working-directories) ab. Eine Regel wie `Bash(git commit *)` genehmigt den Befehl, nicht das Ziel. Ein Ziel, das mit `~` beginnt oder ein Glob-Zeichen enthält, benötigt Ihre Genehmigung.
* **Eingabeumleitungen**: für `< file` deckt die Überprüfung Ihre `Read` Allow- und Deny-Regeln und die Arbeitsverzeichnisse ab. Ein Ziel außerhalb der Arbeitsverzeichnisse benötigt Ihre Genehmigung, es sei denn, eine Allow-Regel deckt es ab. Ein Ziel, das ein Glob-Muster enthält, oder einen relativen Pfad, der einer `cd` im gleichen Befehl folgt, benötigt Ihre Genehmigung, auch wenn eine Allow-Regel es deckt. Claude Code überprüft Eingabeziele in v2.1.257 und später.

Ziele ohne Datei dahinter werden nicht überprüft: `/dev/null`, Dateideskriptor-Formen wie `2>&1` und `<&3`, und Here-Docs und Here-Strings.

Claude Code überprüft auch die Dateien, die ein `tee`-Befehl schreibt, einschließlich in einer Pipeline wie `make | tee build.log`. Die Überprüfung deckt Ihre `Edit` Allow- und Deny-Regeln, [geschützte Pfade](/docs/de/permission-modes#protected-paths) und die [Arbeitsverzeichnisse](#working-directories) ab. Eine Allow-Regel wie `Bash(tee *)` deckt ein Ziel außerhalb der Arbeitsverzeichnisse nicht ab. Claude Code überprüft `tee`-Ziele in v2.1.269 und später.

<h3 id="powershell">
  PowerShell
</h3>

PowerShell-Berechtigungsregeln verwenden die gleiche Form wie Bash-Regeln. Platzhalter mit `*` gleichen an jeder Position ab, das Suffix `:*` ist gleichwertig mit einem nachgestellten ` *`, und ein bloßes `PowerShell` oder `PowerShell(*)` gleicht jeden Befehl ab. Diese Konfiguration ermöglicht `Get-ChildItem`- und `git commit`-Befehle, blockiert aber `Remove-Item`:

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

Häufige Aliase werden vor dem Abgleich kanonisiert. Eine Regel, die für den Cmdlet-Namen geschrieben wurde, gleicht auch seine Aliase ab, daher gleicht `PowerShell(Get-ChildItem *)` auch `gci`, `ls` und `dir` ab. Der Abgleich ist nicht case-sensitiv.

Claude Code analysiert die PowerShell-AST und überprüft jeden Befehl in einem zusammengesetzten Befehl unabhängig. Pipeline-Operatoren `|`, Anweisungstrennzeichen `;` und auf PowerShell 7+ die Kettenoperatoren `&&` und `||` teilen einen zusammengesetzten Befehl in Unterbefehle auf. Eine Regel muss jeden Unterbefehl abgleichen, damit der zusammengesetzte Befehl zulässig ist.

<h3 id="read-and-edit">
  Read und Edit
</h3>

Um Claude's Dateiwerkzeuge daran zu hindern, eine Datei oder ein Verzeichnis zu lesen, fügen Sie eine `Read` Deny-Regel für seinen Pfad hinzu, wie `Read(./.env)` oder `Read(./secrets/**)`; [Sensible Dateien ausschließen](/docs/de/settings-reference#exclude-sensitive-files) hat ein einfügendes Beispiel.

`Edit`-Regeln gelten für alle integrierten Werkzeuge, die Dateien bearbeiten. Claude versucht nach besten Kräften, `Read`-Regeln auf alle integrierten Werkzeuge anzuwenden, die Dateien lesen, wie Grep und Glob, auf `@file`-Erwähnungen in Ihren Eingabeaufforderungen und auf die Auswahl und offene Datei-Kontexte, die ein verbundener [IDE](/docs/de/vs-code#the-built-in-ide-mcp-server) mit Claude teilt.

Eine `Read` Deny-Regel blockiert auch das [Edit- und Write-Werkzeug](/docs/de/errors#file-is-covered-by-a-read-deny-rule) auf demselben Pfad, einschließlich der Erstellung einer neuen Datei dort. NotebookEdit ist nicht abgedeckt, daher fügen Sie eine `Edit` Deny-Regel für Pfade hinzu, die kein Werkzeug ändern darf. Die Überprüfung erfordert Claude Code v2.1.208 oder später bei Bearbeitungen, und v2.1.228 oder später bei Schreibvorgängen.

Claude Code überprüft Dateiberechtigungen nur gegen `Edit(path)` und `Read(path)` Regeln. Wenn Sie eine Pfadregel für `Write`, `NotebookEdit`, `Glob` oder das veraltete `MultiEdit`-Werkzeug schreiben, akzeptiert Claude Code die Regel, konsultiert sie aber nie, und [warnt beim Start](/docs/de/errors#is-not-matched-by-file-permission-checks), außer für eine `Glob`-Regel, die in `--allowedTools` übergeben wird. Verwenden Sie `Edit(docs/**)` anstelle von `Write(docs/**)`, `NotebookEdit(docs/**)` oder `MultiEdit(docs/**)`, und `Read(docs/**)` anstelle von `Glob(docs/**)`. Claude Code warnt nicht vor einer Werkzeugnamen-Regel ohne Pfad, wie einer Deny-Regel für `Write`; sie passt diese Regel überall auf Werkzeugebene an. Erfordert Claude Code v2.1.210 oder später.

<Warning>
  Read- und Edit-Deny-Regeln gelten für Claude's integrierte Dateiwerkzeuge, für Dateibefehle, die Claude Code in Bash erkennt, wie `cat`, `head`, `tail`, `sed` und `tee`, und für die Ziele von Bash [Umleitungen](#redirections) wie `> file` und `< file`. Sie gelten nicht für Befehle, die Dateien ohne Benennung lesen, wie `grep -r pattern .` von dem Verzeichnis aus ausgeführt, das die Datei enthält, oder für beliebige Unterprozesse, die Dateien indirekt lesen oder schreiben, wie ein Python- oder Node-Skript, das Dateien selbst öffnet. Für OS-Ebenen-Durchsetzung, die alle Prozesse daran hindert, auf einen Pfad zuzugreifen, [aktivieren Sie die Sandbox](/docs/de/sandboxing).
</Warning>

Read- und Edit-Regeln folgen beide der [gitignore](https://git-scm.com/docs/gitignore)-Spezifikation mit vier unterschiedlichen Mustertypen; für Muster mit einzelnem Verzeichnissegment hängt die Abgleichtiefe auch vom Regeltyp ab, der später in diesem Abschnitt beschrieben wird:

| Muster               | Bedeutung                              | Beispiel                         | Gleicht ab                                                        |
| -------------------- | -------------------------------------- | -------------------------------- | ----------------------------------------------------------------- |
| `//path`             | Absoluter Pfad vom Dateisystem-Root    | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                         |
| `~/path`             | Pfad vom Home-Verzeichnis              | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                                    |
| `/path`              | Pfad relativ zur Einstellungsquelle    | `Edit(/src/**/*.ts)`             | `<primary working directory>/src/**/*.ts` in Projekteinstellungen |
| `path` oder `./path` | Pfad relativ zum aktuellen Verzeichnis | `Read(*.env)`                    | `<cwd>/*.env`                                                     |

<Warning>
  Ein Muster wie `/Users/alice/file` ist kein absoluter Pfad. Der einzelne führende Schrägstrich verankert sich an der Einstellungsquelle, nicht am Dateisystem-Root. Verwenden Sie `//Users/alice/file` für absolute Pfade.
</Warning>

Ein `/path`-Muster verankert sich an einem Verzeichnis, das der Einstellungsquelle zugeordnet ist, die es definiert, daher gleicht die gleiche Regel verschiedene Orte ab, je nachdem, wo Sie sie platzieren:

| Regel definiert in                                       | `/path` wird aufgelöst zu          |
| :------------------------------------------------------- | :--------------------------------- |
| Projekteinstellungen unter `.claude/settings.json`       | `<primary working directory>/path` |
| Lokale Einstellungen unter `.claude/settings.local.json` | `<primary working directory>/path` |
| Benutzereinstellungen unter `~/.claude/settings.json`    | `~/.claude/path`                   |
| Eine Datei, die mit `--settings <file>` übergeben wird   | `<directory of file>/path`         |
| CLI-Flags oder Sitzungsregeln                            | `<primary working directory>/path` |

Eine Regel, die Sie über `/permissions` hinzufügen, folgt der Zeile für die Einstellungsdatei, in der Sie sie speichern.

Lokale Einstellungsregeln verankern sich am [primären Arbeitsverzeichnis](#working-directories) der Sitzung, nicht am Repository-Root, wo Claude Code [die Datei speichert](#permission-system) in v2.1.211 und später. In einer Sitzung, die am Repository-Root gestartet wird, sind die beiden Verzeichnisse gleich; in einer [Worktree](/docs/de/worktrees)-Sitzung gleicht eine gemeinsame Regel wie `Edit(/src/**)` das eigene `src/`-Verzeichnis dieser Worktree.

Eine Deny-Regel wie `Read(/secrets/**)` in Benutzereinstellungen blockiert `~/.claude/secrets/**`, nicht ein `secrets`-Verzeichnis in Ihrem Projekt. Um eine Regel in Benutzereinstellungen zu schreiben, die sich in jedem Projekt anwendet, verwenden Sie stattdessen einen `//` absoluten Pfad oder einen `~/` Home-relativen Pfad.

Unter Windows werden Pfade vor dem Abgleich in POSIX-Form normalisiert. `C:\Users\alice` wird zu `/c/Users/alice`, verwenden Sie also `//c/**/.env`, um `.env`-Dateien überall auf diesem Laufwerk abzugleichen. Um über alle Laufwerke hinweg abzugleichen, verwenden Sie `//**/.env`.

Beispiele:

* `Edit(/docs/**)`: Bearbeitungen in `<primary working directory>/docs/`, nicht `/docs/` oder `<primary working directory>/.claude/docs/`
* `Read(~/.zshrc)`: liest die `.zshrc` Ihres Home-Verzeichnisses
* `Edit(//tmp/scratch.txt)`: bearbeitet den absoluten Pfad `/tmp/scratch.txt`
* `Read(src/**)`: als Allow-Regel liest aus `<current-directory>/src/` nur; als Deny- oder Ask-Regel gleicht ein `src`-Verzeichnis in jeder Tiefe unter dem aktuellen Verzeichnis ab

Eine Regel gleicht nur Dateien unter ihrem Anker ab; innerhalb dieser Grenze hängt die Abgleichtiefe vom Mustermuster und, für Muster mit einzelnem Verzeichnissegment, vom Regeltyp ab, der unten beschrieben wird. Bare Dateinamen folgen gitignore-Semantik und gleichen in jeder Tiefe ab, daher sind `Read(.env)` und `Read(**/.env)` gleichwertig:

| Deny-Regel                        | Blockiert                                           | Blockiert nicht                                                       |
| --------------------------------- | --------------------------------------------------- | --------------------------------------------------------------------- |
| `Read(.env)` oder `Read(**/.env)` | jede `.env` im oder unter dem aktuellen Verzeichnis | `.env` in einem übergeordneten Verzeichnis oder einem anderen Projekt |
| `Read(//**/.env)`                 | jede `.env` überall im Dateisystem                  | nichts; die Regel ist am Dateisystem-Root verankert                   |

Ein relatives Muster mit einem einzelnen Verzeichnissegment, wie `src/**`, gleicht in verschiedenen Tiefen ab, je nach Regeltyp:

* **Allow-Regeln**: `Edit(src/**)` gleicht nur `<cwd>/src` und die Dateien darunter ab. Um einen Verzeichnisnamen in jeder Tiefe zuzulassen, schreiben Sie `Edit(**/src/**)`.
* **Deny- und Ask-Regeln**: `Read(secrets/**)` gleicht ein Verzeichnis namens `secrets` in jeder Tiefe unter dem aktuellen Verzeichnis ab, daher gilt die Regel auch für verschachtelte Kopien.

Jede andere Mustermuster gleicht in jeder Regeltyp in der gleichen Tiefe ab: `Edit(/src/**)` und `Edit(src/components/**)` gleichen nur an ihrer verankerten Position ab, während `Edit(**/src/**)` in jeder Tiefe gleicht.

Das folgende Beispiel zeigt jede Mustermuster gegen ein Projekt mit einem Top-Level-`src/`-Verzeichnis und einer verschachtelte Kopie unter `vendor/`:

```text theme={null}
<current-directory>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| Regel                                   | Gleicht `src/app.ts` ab | Gleicht `vendor/pkg/src/lib.js` ab |
| :-------------------------------------- | :---------------------- | :--------------------------------- |
| `Edit(src/**)` als Allow-Regel          | Ja                      | Nein                               |
| `Edit(src/**)` als Deny- oder Ask-Regel | Ja                      | Ja                                 |
| `Edit(/src/**)` in jedem Regeltyp       | Ja                      | Nein                               |
| `Edit(**/src/**)` in jedem Regeltyp     | Ja                      | Ja                                 |

<Note>
  In gitignore-Mustern gleicht `*` innerhalb eines einzelnen Pfadsegments und kann an jeder Position im Muster erscheinen, während `**` über Verzeichnisse hinweg gleicht.
</Note>

Wenn Sie einen Dateipfad mit „Ja, nicht mehr fragen" genehmigen, entweicht Claude Code gitignore-Mustzeichen in diesem Pfad, wie `[`, `]` und `*`, daher gleicht die generierte Regel nur den wörtlichen Pfad, den Sie genehmigt haben. Regeln, die Sie selbst schreiben, werden nicht entweicht. Vor v2.1.202 speicherte Claude Code den Pfad unentweicht, daher konnte eine generierte Regel für ein Verzeichnis namens `[2024-06] Reports` seinen eigenen Pfad nicht abgleichen oder unbeabsichtigte Geschwisterverzeichnisse abgleichen.

Sie müssen Klammern in einem Pfad nicht entweichen, daher gleicht `Edit(./Finance (2024)/**)` den `Finance (2024)`-Ordner wie geschrieben ab.

Eine Deny- oder Ask-Regel, deren Pfad nicht als gitignore-Muster verwendbar ist, schützt immer noch diesen genauen Pfad. Eine Allow-Regel mit einem nicht verwendbaren Muster genehmigt nichts.

Eine Deny- oder Ask-Regel, die ein Negationsmuster mit `!` beginnt, ist eine gitignore-Negation. Sie schneidet die Pfade, die sie abgleicht, aus den `path` oder `./path` Regeln aus, die davor aufgelistet sind. In einer Einstellungsdatei's `deny`-Liste blockiert `Read(*.env)` gefolgt von `Read(!sample.env)` jede Datei, deren Name mit `.env` endet, in jeder Tiefe, außer Dateien namens `sample.env`. Eine `!`-Regel, die zuerst aufgelistet ist, schneidet nichts aus.

Die Ausschneidung erreicht nur Regeln aus der gleichen Quelle. Ein `Read(!.env)` in Projekteinstellungen oder in `--disallowedTools` hebt ein `Read(./.env)` Deny aus verwalteten Einstellungen oder einer anderen Einstellungsdatei nicht auf.

Zwei Grenzen verengen, was ein `!`-Muster ausschneiden kann:

* Claude Code liest ein `!`-Muster relativ zum aktuellen Verzeichnis, auch wenn `/`, `~/` oder `//` dem `!` folgt, daher kann das Muster eine Regel, die mit einem dieser Präfixe verankert ist, nicht erreichen. `Read(!~/notes/public/**)` schneidet nichts aus `Read(~/notes/**)` aus.
* Ein Ausschnitt kann eine Datei innerhalb eines Verzeichnisses, das eine Regel als Ganzes blockiert, nicht wieder öffnen. Mit `Read(secrets/**)` und `Read(!secrets/public/**)` blockiert Claude Code immer noch `secrets/public` zusammen mit dem Rest von `secrets`.

Wenn Claude auf einen Symlink zugreift, überprüfen Berechtigungsregeln zwei Pfade: den Symlink selbst und die Datei, auf die er verweist. Allow- und Deny-Regeln behandeln dieses Paar unterschiedlich: Allow-Regeln fallen auf Aufforderungen zurück, während Deny-Regeln direkt blockieren.

* **Allow-Regeln**: gelten nur, wenn sowohl der Symlink-Pfad als auch sein Ziel übereinstimmen. Ein Symlink in einem zulässigen Verzeichnis, der außerhalb davon verweist, fordert Sie immer noch auf.
* **Deny-Regeln**: gelten, wenn entweder der Symlink-Pfad oder sein Ziel übereinstimmt. Ein Symlink, der auf eine verweigerte Datei verweist, ist selbst verweigert. Zum Beispiel mit `Read(./project/**)` zulässig und `Read(~/.ssh/**)` verweigert, wird ein Symlink bei `./project/key`, der auf `~/.ssh/id_rsa` verweist, blockiert: Das Ziel schlägt die Allow-Regel fehl und passt zur Deny-Regel.

Auf macOS und Linux gilt eine Deny- oder Ask-Regel, die durch ein symverlinktes Verzeichnis mit einem `//`, `~/` oder `/` Muster geschrieben wurde, auch am echten Ort des Verzeichnisses. Zum Beispiel auf macOS, wo `/etc` zu `/private/etc` aufgelöst wird, blockiert `Read(//etc/**)` auch `/private/etc/hosts`. Vor v2.1.268 galt eine Deny- oder Ask-Regel, die durch ein symverlinktes Verzeichnis geschrieben wurde, nicht für einen Pfad, der durch seinen echten Ort gegeben wurde.

Wenn ein Werkzeug eine genehmigte Datei öffnet, [bestätigt Claude Code, dass der Pfad immer noch zum Ort aufgelöst wird, den die Berechtigungsprüfung genehmigt hat](/docs/de/errors#refusing-after-a-symlink-changed).

Grep und Glob durchsuchen das Verzeichnis, zu dem das `path`-Argument aufgelöst wird. Claude Code wendet `Read` Deny-Regeln auf dieses Verzeichnis an.

<h3 id="webfetch">
  WebFetch
</h3>

WebFetch-Regeln verwenden ein `domain:`-Präfix und gleichen gegen den Hostnamen der angeforderten URL ab. Der Abgleich ist case-insensitiv, unterstützt `*`-Platzhalter und entfernt einen nachgestellten `.` sowohl aus der Regel als auch aus dem Hostnamen, daher werden `example.com.` und `example.com` gleich behandelt.

* `WebFetch(domain:example.com)` gleicht Anfragen an `example.com` ab
* `WebFetch(domain:*.example.com)` gleicht jede Subdomain in jeder Tiefe ab, wie `api.example.com` oder `a.b.example.com`, aber nicht `example.com` selbst
* `WebFetch(domain:*)` gleicht jede Domain ab. Es ist nicht das gleiche wie eine bare `WebFetch`-Regel; siehe [Jeden Abruf zulassen oder verweigern](#allow-or-deny-every-fetch)

An jeder Position außer einem führenden `*.` oder einem bloßen `*` gleicht der Platzhalter nur den Text zwischen zwei Punkten. `WebFetch(domain:example.*)` gleicht `example.org` ab, wobei `*` zu `org` wird, aber nicht `example.evil.com`, wobei `*` zu `evil.com` werden müsste und einen Punkt überschreiten würde. Dies verhindert, dass ein nachgestellter Platzhalter Domänen abgleicht, die ein Angreifer registrieren könnte.

Platzhalter in `WebFetch`-Regeln erfordern Claude Code v2.1.172 oder später, um Abrufe abzugleichen.

<h4 id="allow-or-deny-every-fetch">
  Jeden Abruf zulassen oder verweigern
</h4>

Eine bare `WebFetch`-Regel ist der Werkzeugname ohne `domain:`-Teil, wie `"deny": ["WebFetch"]`. Sowohl sie als auch `WebFetch(domain:*)` decken jede URL ab, aber Claude Code wendet sie unterschiedlich an, und nur die `domain:`-Form fügt ihre Domain auch zur [zulässigen oder verweigerten Domain-Liste](/docs/de/sandboxing#network-isolation) der Sandbox hinzu. Dieser Abschnitt listet die Wildcard-Formen auf, die die Sandbox ehrt, und die Version, die bare `*` hinzugefügt hat.

Jede Zeile zeigt, was eine Regel in der `allow`-Liste und in der `deny`-Liste tut:

| Regel                | In `allow`                                                                                            | In `deny`                                                                                                                                               |
| :------------------- | :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `WebFetch`           | Claude ruft ab, ohne Sie aufzufordern. Ändert nicht, welche Hosts sandboxed-Befehle erreichen können. | Claude Code entfernt das `WebFetch`-Werkzeug, daher kann Claude überhaupt nicht abrufen. Ändert nicht, welche Hosts sandboxed-Befehle erreichen können. |
| `WebFetch(domain:*)` | Claude ruft ab, ohne Sie aufzufordern, und sandboxed-Befehle können jeden Host erreichen.             | Claude Code behält das Werkzeug und weigert jeden Abruf, und sandboxed-Befehle können keinen Host erreichen.                                            |

Die beiden Formen unterscheiden sich auch bei Lesevorgängen von [Artefakten](/docs/de/artifacts), den Seiten, die das Artifact-Werkzeug auf claude.ai veröffentlicht. Eine bare `WebFetch` Deny- oder Ask-Regel gilt nicht für diese Lesevorgänge. Eine `domain:`-Regel, die `claude.ai` oder den `*.claudeusercontent.com` Content-Host abdeckt, wie `WebFetch(domain:claude.ai)` oder `WebFetch(domain:*)`, verweigert jeden Lesevorgang oder fordert davor auf. Eine [`Artifact`-Regel](/docs/de/artifacts#disable-artifacts) tut das gleiche.

Wenn eine Regel einen Lesevorgang blockiert, benennt die Ablehnung die Regel. Vor v2.1.268 blockierte eine bare `WebFetch` Deny-Regel jeden Artefakt-Lesevorgang, und eine bare Ask-Regel forderte vor jedem auf.

Um Claude frei abrufen zu lassen, während die Sandbox-Allowlist wie sie ist bleibt, verwenden Sie die bare Form. Diese `settings.json` tut das:

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Wenn Sie Claude bitten, eine Seite abzurufen, ruft es ab, ohne Sie aufzufordern. Wenn Sie es bitten, einen [sandboxed](/docs/de/sandboxing) `curl` gegen einen Host außerhalb der Sandbox-Allowlist auszuführen, fordert Claude Code Sie immer noch für diesen Host auf, da die bare Regel den Host nicht zur Allowlist hinzugefügt hat. Im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) benennt Claude stattdessen den Host in den [pro-Befehl zulässigen Domänen](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode) des Befehls für den Klassifizierer zur Überprüfung.

<h3 id="mcp">
  MCP
</h3>

MCP-Regeln verwenden den Servernamen wie in Claude Code konfiguriert, optional gefolgt vom Namen eines Werkzeugs von diesem Server.

* `mcp__puppeteer` gleicht jedes Werkzeug ab, das vom `puppeteer`-Server bereitgestellt wird
* `mcp__puppeteer__*` verwendet Wildcard-Syntax und gleicht auch alle Werkzeuge vom `puppeteer`-Server ab
* `mcp__puppeteer__puppeteer_navigate` gleicht das `puppeteer_navigate`-Werkzeug ab, das vom `puppeteer`-Server bereitgestellt wird

Wenn Ihre Organisation ein [claude.ai-Connector](/docs/de/mcp#organization-controls-on-connector-tools)-Werkzeug auf `ask` gesetzt hat und diese Einstellung Claude Code in Ihrer Sitzung erreicht, nehmen Allow-Regeln für dieses Werkzeug keine Wirkung: Claude Code fordert bei jedem Aufruf auf, auch in `auto`- und `bypassPermissions`-Modi. Im `dontAsk`-Modus, der niemals auffordert, verweigert Claude Code den Aufruf stattdessen. Werkzeuge von Connectoren, die Claude Code selbst abruft, erscheinen als `mcp__claude_ai_<server>__<tool>`.

In einer [Cowork](https://claude.com/docs/cowork/overview)-Sitzung in der Claude Desktop-App führt Claude Shell-Befehle über Cowork's `mcp__workspace__bash`-Werkzeug anstelle des integrierten `Bash`-Werkzeugs aus, und Cowork stellt ebenso `mcp__workspace__web_fetch` für Web-Abrufe bereit. Claude Code wendet auch Deny-Regeln an, die das ganze `Bash`- oder `WebFetch`-Werkzeug benennen, auf diese Cowork-Werkzeuge an, daher stoppt eine verwaltete `Bash`-Deny-Regel Claude daran, Shell-Befehle in Cowork auszuführen. Wenn Claude Code einen solchen Aufruf blockiert, benennt die Nachricht das Cowork-Werkzeug: `Permission to use mcp__workspace__bash has been denied.` Allow-Regeln tragen nicht über: Claude Code wendet nie eine `Bash`-Allow-Regel auf `mcp__workspace__bash` an.

<h3 id="agent-subagents">
  Agent (Subagents)
</h3>

Verwenden Sie `Agent(AgentName)`-Regeln, um zu steuern, welche [Subagents](/docs/de/sub-agents) Claude verwenden kann:

* `Agent(Explore)` gleicht den Explore-Subagent ab
* `Agent(Plan)` gleicht den Plan-Subagent ab
* `Agent(my-custom-agent)` gleicht einen benutzerdefinierten Subagent namens `my-custom-agent` ab

Fügen Sie diese Regeln zum `deny`-Array in Ihren Einstellungen hinzu oder verwenden Sie das `--disallowedTools`-CLI-Flag, um bestimmte Agenten zu deaktivieren. Um den Explore-Agenten zu deaktivieren:

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

`Cd`-Regeln steuern, in welche Verzeichnisse der [`/cd`-Befehl](/docs/de/commands) die Sitzung verschieben kann. `Cd` ist kein vom Modell aufzurufendes Werkzeug: Claude kann es nicht aufrufen, und die Regeln gelten nur, wenn Sie `/cd` selbst ausführen.

Eine bare `Cd`-Deny-Regel deaktiviert `/cd` vollständig. Eine `Cd(<path-pattern>)`-Deny-Regel blockiert übereinstimmende Ziele. Deny-Regeln überprüfen jede Schreibweise des Ziels, einschließlich jedes Symlink-Hops, durch den es sich auflöst, daher blockiert eine Regel, die für einen Pfad geschrieben wurde, auch Ziele, die sich darin auflösen.

Das Hinzufügen einer `Cd`-Allow-Regel schaltet `/cd` in den Allowlist-Modus: Das aufgelöste Zielverzeichnis muss einer Ihrer Allow-Regeln entsprechen, oder `/cd` weigert sich. Ohne konfigurierte `Cd`-Regeln behält `/cd` sein Standardverhalten bei und fordert Sie auf, einem unbekannten Verzeichnis zu vertrauen.

Pfadmuster teilen die `//`, `~/` und `/` Anker von [Read- und Edit-Regeln](#read-and-edit), aber der Abgleich ist am gesamten Verzeichnispfad verankert, nicht im gitignore-Stil. `*` gleicht genau ein Pfadsegment ab und `**` gleicht über Segmente hinweg ab. Ein nachgestelltes `/**` gleicht auch seinen benannten Root ab.

| Regel                 | Gleicht ab                                                                      | Gleicht nicht ab                     |
| --------------------- | ------------------------------------------------------------------------------- | ------------------------------------ |
| `Cd(~/code/*)`        | `~/code/app`                                                                    | `~/code/app/src`, `~/code`           |
| `Cd(~/code/**)`       | `~/code` und jedes Verzeichnis darunter                                         | Verzeichnisse außerhalb von `~/code` |
| `Cd(**/node_modules)` | jedes `node_modules`-Verzeichnis in jeder Tiefe unter dem aktuellen Verzeichnis | `node_modules/pkg`                   |

<h2 id="extend-permissions-with-hooks">
  Berechtigungen mit Hooks erweitern
</h2>

[Claude Code Hooks](/docs/de/hooks-guide) ermöglichen es Ihnen, benutzerdefinierte Shell-Befehle zu registrieren, die Berechtigungen zur Laufzeit evaluieren. Wenn Claude Code einen Werkzeugaufruf tätigt, werden PreToolUse-Hooks vor dem Berechtigungssystem ausgeführt, für jedes Werkzeug außer [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior). Die Hook-Ausgabe kann den Werkzeugaufruf verweigern, eine Aufforderung erzwingen oder die Aufforderung überspringen, um den Aufruf fortzufahren.

Hook-Entscheidungen umgehen keine Berechtigungsregeln. Claude Code evaluiert Deny- und Ask-Regeln unabhängig davon, was ein PreToolUse-Hook zurückgibt: Eine übereinstimmende Deny-Regel blockiert den Aufruf, und eine übereinstimmende Ask-Regel fordert immer noch auf, selbst wenn der Hook `"allow"` oder `"ask"` zurückgegeben hat. Dies bewahrt die Deny-First-Priorität, die in [Berechtigungen verwalten](#manage-permissions) beschrieben ist, einschließlich Deny-Regeln, die in verwalteten Einstellungen festgelegt sind.

MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden auch weiterhin aufgefordert, wenn ein Hook `"allow"` zurückgibt, ebenso wie Connector-Tools, [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) in Sitzungen, in denen diese Einstellung Claude Code erreicht.

Ein blockierender Hook hat auch Vorrang vor Allow-Regeln. Ein Hook, der mit Code 2 beendet wird, stoppt den Werkzeugaufruf, bevor Berechtigungsregeln evaluiert werden, daher gilt die Blockierung auch dann, wenn eine Allow-Regel den Aufruf sonst zulassen würde. Um alle Bash-Befehle ohne Aufforderungen auszuführen, außer für einige, die Sie blockieren möchten, fügen Sie `"Bash"` zu Ihrer Allow-Liste hinzu und registrieren Sie einen PreToolUse-Hook, der diese spezifischen Befehle ablehnt. Siehe [Bearbeitungen geschützter Dateien blockieren](/docs/de/hooks-guide#block-edits-to-protected-files) für ein Hook-Skript, das Sie anpassen können.

<h2 id="working-directories">
  Arbeitsverzeichnisse
</h2>

Standardmäßig hat Claude Zugriff auf Dateien in dem Verzeichnis, in dem Sie es gestartet haben. Dieses Verzeichnis ist das primäre Arbeitsverzeichnis der Sitzung, bis Sie [die Sitzung mit `/cd` verschieben](#move-the-session-to-another-directory). Sie können diesen Zugriff erweitern:

* **Beim Start**: Verwenden Sie das CLI-Argument `--add-dir <path>`
* **Während der Sitzung**: Verwenden Sie den Befehl `/add-dir`
* **Persistente Konfiguration**: Fügen Sie zu `additionalDirectories` in [Einstellungsdateien](/docs/de/settings#where-settings-live) hinzu

Dateien in zusätzlichen Verzeichnissen folgen den gleichen Berechtigungsregeln wie das ursprüngliche Arbeitsverzeichnis: Sie werden lesbar ohne Aufforderungen, und Dateiberechtigungen folgen dem aktuellen Berechtigungsmodus.

Sie können die meisten [Netzwerkpfade](/docs/de/errors#working-directory-is-a-network-path), wie die UNC-Freigabe `\\server\share`, nicht als Arbeitsverzeichnisse hinzufügen, da das Nachschlagen einen Kontakt zum benannten Host verursachen kann. Unter Windows ordnen Sie die Freigabe stattdessen einem Laufwerksbuchstaben zu und übergeben das Laufwerk mit `--add-dir` beim Start.

Setzen Sie [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories), um die Dateiwerkzeuge zu veranlassen, die Pfade, die es einzäunt, in jedem Berechtigungsmodus abzulehnen. Im Auto-Modus bietet Claude Code an, es beim ersten Mal einzuschalten, wenn Claude [außerhalb der Arbeitsverzeichnisse liest](/docs/de/permission-modes#first-read-outside-the-working-directories).

In Hintergrund-Sitzungen auf macOS fordert der Sitzungs-Host Zugriff auf geschützte Ordner wie `~/Desktop`, `~/Documents` und `~/Downloads` separat von Ihrem Terminal an, wenn Claude dort Dateien lesen oder schreiben muss; wenn Lesevorgänge dort mit `Operation not permitted` fehlschlagen, siehe [wie man Ordnerzugriff auf Hintergrund-Sitzungen auf macOS gewährt](/docs/de/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos).

<h3 id="move-the-session-to-another-directory">
  Sitzung in ein anderes Verzeichnis verschieben
</h3>

Um die Sitzung in ein anderes primäres Arbeitsverzeichnis zu verschieben, anstatt [ein Verzeichnis hinzuzufügen](#working-directories) neben dem aktuellen, führen Sie `/cd <path>` aus. Claude Code behält die Konversation bei, lädt die `CLAUDE.md` des neuen Verzeichnisses und fordert Sie auf, [dem Arbeitsbereich zu vertrauen](#project-allow-rules-and-workspace-trust), wenn Sie darin noch nicht gearbeitet haben. Danach [findet Claude Code die verschobene Sitzung](/docs/de/sessions#resume-a-session), wenn Sie `--resume` aus dem neuen Verzeichnis ausführen.

Sobald Sie verschieben, wendet Claude Code die Projektkonfiguration des neuen Verzeichnisses an:

* Seine Projekteinstellungen, einschließlich ihrer Berechtigungsregeln und [Hooks](/docs/de/hooks)
* Seine [`.mcp.json`-Server](/docs/de/mcp#project-scope), unterliegen der gleichen [Server-Genehmigung](/docs/de/mcp#project-server-approvals-and-workspace-trust) wie beim Start, und die [lokalen](/docs/de/mcp#local-scope) MCP-Server, die Sie darin registriert haben
* Die [Plugins](/docs/de/plugins/overview), die seine Einstellungen aktivieren, seine [Skills](/docs/de/skills#discovery-from-parent-and-nested-directories) und seine [Subagents](/docs/de/sub-agents)
* Seine [`env`](/docs/de/settings-reference#env)-Werte, angewendet auf die Umgebungsvariablen aus den Einstellungen des vorherigen Verzeichnisses, die weiterhin gültig sind

Claude Code trennt auch die MCP-Server des vorherigen Verzeichnisses und [lokalen](/docs/de/mcp#local-scope) ab, sowie die Server von [Plugins](/docs/de/mcp#plugin-provided-mcp-servers), die nach dem Verschieben nicht mehr aktiviert sind. Es nimmt [zusätzliche Verzeichnisse](#working-directories) aus den Einstellungen des neuen Verzeichnisses statt aus denen des vorherigen, und behält die Verzeichnisse bei, die Sie mit `--add-dir` oder `/add-dir` hinzugefügt haben. Hooks, die das Verschieben aktiviert, erhalten immer noch [`${CLAUDE_PROJECT_DIR}`](/docs/de/hooks#reference-scripts-by-path) auf das Projektverzeichnis gesetzt, in dem die Sitzung gestartet wurde.

Wenn das neue Verzeichnis noch nicht vertraut ist, listet Claude Code in der Vertrauensaufforderung die Allow-Regeln, zusätzlichen Verzeichnisse, Hooks und Hilfsbefehle auf, die die Einstellungen des Verzeichnisses aktivieren würden, damit Sie diese überprüfen können, bevor Sie akzeptieren. Wenn Sie ablehnen, bleibt die Sitzung dort, wo sie ist. Vor v2.1.246 wendete `/cd` die Einstellungen, Hooks, MCP-Server oder Skills des neuen Verzeichnisses nicht an, bis Sie die Sitzung wiederaufnahmen, und die Vertrauensaufforderung listete nicht auf, was die Einstellungen des Verzeichnisses aktivieren würden.

Beschränken oder deaktivieren Sie `/cd`-Ziele mit [`Cd`-Berechtigungsregeln](#cd).

<h3 id="additional-directories-grant-file-access-not-configuration">
  Zusätzliche Verzeichnisse gewähren Dateizugriff, keine Konfiguration
</h3>

Das Hinzufügen eines Verzeichnisses erweitert, wo Claude Dateien lesen und bearbeiten kann. Es macht dieses Verzeichnis nicht zu einem vollständigen Konfigurationsroot: Die meisten `.claude/`-Konfigurationen werden nicht aus zusätzlichen Verzeichnissen erkannt, obwohl einige Typen als Ausnahmen geladen werden.

Diese Ausnahmen gelten nur für Verzeichnisse, die mit dem Flag `--add-dir` oder dem Befehl `/add-dir` hinzugefügt wurden, einschließlich Verzeichnisse, die das Agent SDK durch das Flag hinzufügt. Verzeichnisse, die in `permissions.additionalDirectories` in einer Einstellungsdatei aufgelistet sind, gewähren nur Dateizugriff und laden keine der folgenden Konfigurationen.

Das Agent SDK's [`additionalDirectories`](/docs/de/agent-sdk/typescript#options)-Option in TypeScript und [`add_dirs`](/docs/de/agent-sdk/python#claudeagentoptions)-Option in Python erhalten die Ausnahmen ebenfalls, obwohl die TypeScript-Option ihren Namen mit dem Einstellungsschlüssel teilt. Das SDK übergibt jeden Eintrag an Claude Code als `--add-dir`, sodass diese Verzeichnisse sich wie Flag-hinzugefügte Verzeichnisse verhalten. Skills, Befehle und Subagents aus jedem Flag-hinzugefügten Verzeichnis werden durch die `project`-[Einstellungsquelle](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) geladen, sodass sie nicht geladen werden, wenn Sie diese Quelle mit [`--setting-sources`](/docs/de/cli-reference) in der CLI oder `settingSources` im SDK ausschließen, und [Bare Mode](/docs/de/headless#start-faster-with-bare-mode) überspringt die Befehle und Subagents unter ihnen.

Die folgenden Konfigurationstypen werden aus `--add-dir`-Verzeichnissen geladen:

| Konfiguration                                                                              | Geladen aus `--add-dir`                                                                                                                                                       |
| :----------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/de/skills) in `.claude/skills/`                                                  | Ja, mit Live-Reload                                                                                                                                                           |
| [Befehlsdateien](/docs/de/skills#where-skills-live) in `.claude/commands/`                      | Ja, ohne Live-Reload. Wenn das hinzugefügte Verzeichnis und Ihr Projekt beide einen Befehl mit demselben Namen definieren, führt Claude Code Ihren Projektbefehl aus          |
| [Subagents](/docs/de/sub-agents) in `.claude/agents/`                                           | Ja, ohne Live-Reload                                                                                                                                                          |
| [Einstellungen](/docs/de/settings) in `.claude/settings.json` und `.claude/settings.local.json` | Nur `enabledPlugins` und [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces)-Schlüssel                                                                  |
| [CLAUDE.md](/docs/de/memory)-Dateien, `.claude/rules/` und `CLAUDE.local.md`                    | Nur wenn `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` gesetzt ist. `CLAUDE.local.md` erfordert zusätzlich die `local`-Einstellungsquelle, die standardmäßig aktiviert ist |

Um die Skills, Befehle und Subagents aus einem Unterverzeichnis Ihres [primären Arbeitsverzeichnisses](#working-directories) während der Sitzung zu laden, führen Sie `/add-dir` mit dem Pfad dieses Unterverzeichnisses aus. Claude Code lädt sie für den Rest der Sitzung, ohne Sie aufzufordern oder ein Arbeitsverzeichnis hinzuzufügen, da das Unterverzeichnis bereits lesbar ist. Dies erfordert Claude Code v2.1.257 oder später.

Claude Code erkennt Ausgabestile aus dem aktuellen Arbeitsverzeichnis und seinen übergeordneten Verzeichnissen, Ihrem Benutzerverzeichnis unter `~/.claude/` und verwalteten Einstellungen. Hooks und andere `.claude/settings.json`-Schlüssel werden aus dem `.claude/`-Ordner des aktuellen Arbeitsverzeichnisses ohne Fallback für übergeordnete Verzeichnisse geladen, zusammen mit Ihren Benutzer-`~/.claude/settings.json` und verwalteten Einstellungen. `.claude/settings.local.json` wird stattdessen aus dem Git-Repository-Root geladen, auch wenn Sie Claude Code in einem Unterverzeichnis starten, außer in den Fällen, in denen Claude Code [den Repository-Root nicht verwendet](/docs/de/settings#where-claude-code-looks-for-each-file), wie unter Windows; vor v2.1.211 wurde es auch nur aus dem aktuellen Arbeitsverzeichnis geladen. [Agent SDK](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)-Sitzungen laden es in allen Versionen aus dem Arbeitsverzeichnis.

Um diese Konfiguration über Projekte hinweg zu teilen, verwenden Sie einen dieser Ansätze:

* **Benutzergesteuerte Konfiguration**: Platzieren Sie Dateien in `~/.claude/agents/`, `~/.claude/output-styles/` oder `~/.claude/settings.json`, um sie in jedem Projekt verfügbar zu machen
* **Plugins**: Verpacken und verteilen Sie Konfiguration als [Plugin](/docs/de/plugins/overview), das Teams installieren können
* **Starten Sie aus dem Konfigurationsverzeichnis**: Führen Sie Claude Code aus dem Verzeichnis aus, das die `.claude/`-Konfiguration enthält, die Sie verwenden möchten

<h2 id="how-permissions-interact-with-sandboxing">
  Wie Berechtigungen mit Sandboxing interagieren
</h2>

Berechtigungen und [Sandboxing](/docs/de/sandboxing) sind komplementäre Sicherheitsebenen:

* **Berechtigungen** steuern, welche Werkzeuge Claude Code verwenden kann und auf welche Dateien oder Domänen es zugreifen kann. Sie gelten für Bash, Read, Edit, WebFetch, MCP und alle anderen Werkzeuge, außer dass eine Deny- oder Ask-Regel [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior) nicht blockieren kann, während ein anderes Werkzeug aktiv bleibt.
* **Sandboxing** bietet OS-Ebenen-Durchsetzung, die den Zugriff von Shell-Befehlen auf das Dateisystem und das Netzwerk einschränkt. Es gilt nur für Bash, PowerShell und [Monitor](/docs/de/tools-reference#monitor-tool) Befehle und ihre untergeordneten Prozesse.

Verwenden Sie beide für Defense-in-Depth, da Sandbox-Einschränkungen weiterhin gelten, selbst wenn eine Prompt-Injection Claude's Entscheidungsfindung umgeht. Pfade und Domänen sowohl aus Sandbox-Einstellungen als auch aus Berechtigungsregeln werden [in die endgültige Sandbox-Konfiguration zusammengeführt](/docs/de/sandboxing#permission-rules).

Wenn Sie Sandboxing aktivieren und `autoAllowBashIfSandboxed` auf seinen Standardwert von `true` belassen, laufen sandboxed Bash-Befehle ohne Aufforderung, selbst wenn Ihre Berechtigungen eine einfache `Bash` Ask-Regel oder die [äquivalente `Bash(*)` Form](#match-all-uses-of-a-tool) enthalten: die Sandbox-Grenze ersetzt diese Ganz-Werkzeug-Aufforderung.

Im [Plan Mode](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) überspringt Claude Code diese Substitution. Ohne eine Ask-Regel laufen die [integrierten schreibgeschützten Befehle](#read-only-commands) weiterhin ohne Aufforderung, und alle anderen Shell-Befehle durchlaufen den regulären Berechtigungsfluss, während Sie noch planen; siehe [Plan Mode](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) für die Gating-Mechanismen von Claude Code dort. Mit einer einfachen `Bash` Ask-Regel wird jeder Bash-Befehl aufgefordert, einschließlich sandboxed schreibgeschützter Befehle, genauso wie außerhalb von Sandboxing. Vor v2.1.212 galt die Substitution auch im Plan Mode.

Diese Überprüfungen gelten weiterhin:

* Inhaltsgebundene Ask-Regeln wie `Bash(git push *)` erzwingen weiterhin eine Aufforderung
* Explizite Deny-Regeln gelten weiterhin
* `rm`- oder `rmdir`-Befehle, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen, durchlaufen weiterhin den regulären Berechtigungsfluss

Befehle, die nicht sandboxed ausgeführt werden können, wie ausgeschlossene Befehle, respektieren die einfache `Bash` Ask-Regel wie gewöhnlich. Siehe [Sandbox-Modi](/docs/de/sandboxing#sandbox-modes), um dieses Verhalten zu ändern.

<span id="managed-only-settings" />

<h2 id="managed-settings">
  Verwaltete Einstellungen
</h2>

Für Organisationen, die eine zentralisierte Kontrolle benötigen, stellen Administratoren verwaltete Einstellungen bereit, die Benutzer- und Projekteinstellungen nicht überschreiben können, mit Ausnahme einiger [sicherheitssensibler Schlüssel](/docs/de/settings#exceptions-to-managed-settings-precedence). [Verwaltete Einstellungen bereitstellen](/docs/de/managed-settings) behandelt die Bereitstellungsmechanismen, die Rangfolge innerhalb der verwalteten Ebene und die [Schlüssel, die nur verwaltete Einstellungen setzen können](/docs/de/managed-settings#managed-only-settings).

Einer dieser Schlüssel, [`allowManagedPermissionRulesOnly`](/docs/de/settings-reference#allowmanagedpermissionrulesonly), macht verwaltete Einstellungen zur einzigen Einstellungsquelle für Berechtigungsregeln. Sein Eintrag listet jede Quelle auf, die Claude Code dann ignoriert.

`disableBypassPermissionsMode` wird normalerweise in verwalteten Einstellungen platziert, um Organisationsrichtlinien durchzusetzen, funktioniert aber aus jedem Bereich. Ein Benutzer kann es in seinen eigenen Einstellungen festlegen, um sich selbst aus dem Bypass-Modus auszusperren.

<h2 id="settings-precedence">
  Einstellungspriorität
</h2>

Berechtigungsregeln folgen der gleichen [Einstellungspriorität](/docs/de/settings#settings-precedence) wie alle anderen Claude Code-Einstellungen, wobei verwaltete Einstellungen am höchsten sind: Keine andere Ebene, einschließlich Befehlszeilenargumenten, kann eine verwaltete Berechtigungsregel überschreiben.

Wenn ein Werkzeug auf einer beliebigen Ebene verweigert wird, kann keine andere Ebene es zulassen. Zum Beispiel kann eine verwaltete Einstellungs-Deny nicht durch `--allowedTools` überschrieben werden, und `--disallowedTools` kann Einschränkungen über das hinaus hinzufügen, was verwaltete Einstellungen definieren.

Das Gleiche gilt für Einstellungsbereiche: Wenn Benutzereinstellungen eine Berechtigung zulassen und Projekteinstellungen sie verweigern, blockiert die Deny-Regel sie. Das Gegenteil ist auch wahr: eine Deny-Regel auf Benutzerebene blockiert eine Allow-Regel auf Projektebene, da Deny-Regeln aus jedem Bereich vor Allow-Regeln ausgewertet werden.

Embedding-Hosts können zusätzliche verwaltete Richtlinien über die SDK-Option `managedSettings` bereitstellen, einschließlich Berechtigungsallow-Regeln, es sei denn, der Administrator setzt die `allowManaged*Only`-Sperren; [Richtlinie an Claude Desktop-Sitzungen bereitstellen](/docs/de/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) behandelt, wann die Embedder-Richtlinie überhaupt angewendet wird.

<h2 id="project-allow-rules-and-workspace-trust">
  Projekterlaubnisregeln und Workspace-Vertrauen
</h2>

`permissions.allow`-Regeln und `permissions.additionalDirectories`-Einträge in der `.claude/settings.json` eines Projekts gewähren Funktionen, daher wendet Claude Code sie nur an, nachdem Sie den [Workspace-Vertrauensdialog](/docs/de/security#additional-safeguards) für diesen Ordner akzeptiert haben. Der Dialog listet die Regeln und Verzeichnisse auf, die der Ordner gewähren würde, damit Sie diese zuerst überprüfen können. `deny`- und `ask`-Regeln sind nicht betroffen, da sie nur einschränken.

Claude Code speichert das Vertrauen, das Sie akzeptieren, je nachdem, wo Sie es starten:

* In einem Repository speichert Claude Code das Vertrauen auf der Git-Repository-Root, sodass das Vertrauen das gesamte Repository abdeckt, mit Ausnahme von verschachtelten Git-Repositories darin, wie z. B. Submodule. In einem [Worktree](/docs/de/worktrees) verwendet es die Root des Haupt-Checkouts, wie es auch für [gespeicherte Regeln](#permission-system) der Fall ist.
* Außerhalb eines Repositorys speichert Claude Code das Vertrauen auf dem Verzeichnis, von dem aus Sie es gestartet haben, und das Vertrauen deckt alle Unterverzeichnisse dieses Verzeichnisses ab, mit Ausnahme von verschachtelten Git-Repositories darin, wie z. B. Klone. Jedes abgedeckte Unterverzeichnis zählt dann als ein Ordner, dessen übergeordnetes Verzeichnis Sie vertraut haben.
* Wenn Sie in Ihrem Home-Verzeichnis starten, speichert Claude Code das Vertrauen nur für die aktuelle Sitzung und schreibt es nicht auf die Festplatte; siehe die Notiz zu [zusätzlichen Schutzmaßnahmen](/docs/de/security#additional-safeguards).

Claude Code zeigt den Vertrauensdialog nur in interaktiven Sitzungen an. Ein `claude -p`-Lauf oder eine SDK-Sitzung zeigt ihn nie an, und das Vertrauen in ein übergeordnetes Verzeichnis zählt nicht für diese Regeln, daher sagt [Was läuft ab, bevor Sie einem Ordner vertrauen](#what-runs-before-you-trust-a-folder), welche Repository-Inhalte Claude Code in jeder dieser beiden Situationen noch verwendet.

<h3 id="when-your-local-settings-file-needs-trust">
  Wenn Ihre lokale Einstellungsdatei Vertrauen benötigt
</h3>

`.claude/settings.local.json` ist normalerweise Ihre eigene Datei, daher wendet Claude Code ihre Allow-Regeln und zusätzlichen Verzeichnisse ohne den Vertrauensschritt an. Wenn die Datei in Git verfolgt wird oder `.claude` ein Symlink ist, behandelt Claude Code sie stattdessen als vom Repository bereitgestellt und hält ihre Regeln zurück, bis Sie dem Ordner vertrauen.

Claude Code führt Git aus, um die beiden zu unterscheiden, und führt Git nur aus, nachdem Sie dem Ordner vertraut haben: Sie haben den Vertrauensdialog dafür oder für ein übergeordnetes Verzeichnis, dessen Vertrauen sich darauf erstreckt, akzeptiert, oder Sie befinden sich in einer `-p`- oder SDK-Sitzung, die als akzeptiert zählt. Bis dahin entscheidet, wo Sie Claude Code gestartet haben, was mit den Regeln der Datei geschieht:

* **In Ihrem Konfigurationsheim:** Claude Code wendet die `.claude/settings.local.json` dieses Ordners sofort an, ohne Git auszuführen. Ihr Konfigurationsheim ist Ihr Home-Verzeichnis oder ein Verzeichnis, dessen `.claude`-Unterverzeichnis Sie als [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars#variables) festgelegt haben. Wenn dieses `CLAUDE_CONFIG_DIR`-Verzeichnis in einem Git-Repository sitzt und Claude Code [Ihre lokalen Einstellungen in der Repository-Root speichert](/docs/de/settings#where-claude-code-looks-for-each-file), hält es die Regeln wie überall sonst zurück.
* **Überall sonst:** Claude Code hält die Regeln der Datei wie Projekteinstellungen zurück. Sobald die Überprüfung ausgeführt wurde, wendet Claude Code die Regeln einer nicht verfolgten Datei oder einer Datei in einem Verzeichnis außerhalb eines Git-Repositorys an, obwohl Sie diesem genauen Ordner nicht vertraut haben.

<Note>
  Die Konfigurationsheim-Ausnahme überspringt nur den Vertrauensschritt. `~/.claude/settings.local.json` hat immer noch [lokalen Umfang](/docs/de/settings#compare-the-scope-of-each-settings-file), daher liest Claude Code sie nur in Sitzungen, die Sie in Ihrem Home-Verzeichnis selbst starten, nicht in jedem Projekt. Um Berechtigungsregeln auf alle Ihre Projekte anzuwenden, fügen Sie sie stattdessen zu Ihren Benutzereinstellungen hinzu: `~/.claude/settings.json` oder `$CLAUDE_CONFIG_DIR/settings.json`, wenn `CLAUDE_CONFIG_DIR` gesetzt ist.
</Note>

In den Versionen 2.1.196 bis 2.1.199 speicherte Claude Code die Regeln der Datei in Ihrem Konfigurationsheim und außerhalb von Git-Repositorys und gab die Warnung [`this workspace has not been trusted`](/docs/de/errors#workspace-has-not-been-trusted) dort aus. Vor v2.1.207 wendete Claude Code die Regeln einer nicht verfolgten Datei an, bevor Sie den Dialog akzeptierten.

<h3 id="what-runs-before-you-trust-a-folder">
  Was läuft ab, bevor Sie einem Ordner vertrauen
</h3>

Jede Zeile ist eine Art von Inhalten, die ein Repository bereitstellen kann. Die Spalten sind die beiden Situationen, in denen Sie dem Ordner selbst nicht vertraut haben: Sie haben nur einem übergeordneten Ordner vertraut, oder Sie haben `claude -p` oder das SDK dort ausgeführt, was nie den Vertrauensdialog anzeigt. Die Spalte für übergeordnete Ordner gilt nicht in einem [verschachtelten Repository](#project-allow-rules-and-workspace-trust): In einer interaktiven Sitzung zeigt Claude Code den Vertrauensdialog dafür an, und ein `claude -p`- oder SDK-Lauf dort folgt der `claude -p`-Spalte.

| Was das Repository bereitstellt                                                                                                                                                                                                                                                                                                          | Sie haben nur einem übergeordneten Ordner vertraut                                                                                                                                            | `claude -p` oder das SDK, Ordner wurde nie vertraut                                                                                                                                                                |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Hooks](/docs/de/hooks) in Einstellungsdateien, der [`env`](/docs/de/settings-reference#env)-Block und Hilfsbefehle wie [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper), und die [Hooks](/docs/de/hooks#hooks-in-skills-and-agents) und [`allowed-tools`](/docs/de/skills#pre-approve-tools-for-a-skill) eines Projektskills                           | Verwendet                                                                                                                                                                                     | Verwendet. Workspace-Vertrauen gatet nie die `allowed-tools` eines Skills in einer Sitzung                                                                                                                         |
| `permissions.allow`-Regeln und `additionalDirectories` in `.claude/settings.json`                                                                                                                                                                                                                                                        | Nicht verwendet, bis Sie den Vertrauensdialog akzeptieren, der sie erneut auflistet                                                                                                           | Nicht verwendet. Claude Code gibt eine [`this workspace has not been trusted`](/docs/de/errors#workspace-has-not-been-trusted)-Warnung auf stderr aus                                                                   |
| Frontmatter-Hooks in einem Projekt-[Subagent](/docs/de/sub-agents#hooks-in-subagent-frontmatter), einem Projekt-[`@skills-dir`-Plugin](/docs/de/plugins/loading#plugins-shared-through-a-repository) und [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces)-Einträgen aus dem Repository oder einem `--add-dir`-Verzeichnis | Nicht verwendet, und es wird kein Dialog angeboten                                                                                                                                            | Nicht verwendet                                                                                                                                                                                                    |
| Inline-[`mcpServers`](/docs/de/sub-agents#scope-mcp-servers-to-a-subagent) im Frontmatter eines Subagents aus dem Repository oder einem `--add-dir`-Verzeichnis. Vor v2.1.238 lud Claude Code diese Server in beiden Situationen                                                                                                              | Nicht verwendet, und es wird kein Dialog angeboten                                                                                                                                            | Nicht verwendet                                                                                                                                                                                                    |
| Server in `.mcp.json`, einschließlich derjenigen, die das Repository [in seinen eigenen Einstellungen genehmigt](/docs/de/mcp#project-server-approvals-and-workspace-trust)                                                                                                                                                                   | Claude Code fragt Sie, bevor er sich mit ihnen verbindet. Die eigenen Genehmigungen des Repositorys zählen nicht                                                                              | Verbunden ohne zu fragen, genehmigt oder nicht. Das SDK lädt sie nur, wenn `settingSources` Projekteinstellungen enthält. `claude mcp list` im selben Ordner meldet einen solchen Server immer noch als ausstehend |
| Ein [`headersHelper`](/docs/de/mcp#trust-a-folder-before-its-headershelper-runs) auf einem Server in `.mcp.json`. Vor v2.1.238 führte Claude Code den Helper in beiden Situationen aus                                                                                                                                                        | Nicht ausgeführt, bis Sie den Vertrauensdialog akzeptieren, der erneut anzeigt, wo der Helper deklariert ist. Claude Code verbindet den Server nur mit seinen statischen `headers`, bis dahin | Nicht ausgeführt. Claude Code verbindet den Server mit seinen statischen `headers` und gibt eine [`headersHelper not run`](/docs/de/errors#headershelper-not-run)-Zeile pro Server auf stderr aus                       |

Für die Zeilen, die diesen genauen Ordner vertraut benötigen, vertrauen Sie ihm manuell: Setzen Sie `projects["<path>"].hasTrustDialogAccepted` auf `true` in `~/.claude.json`, wobei `<path>` die Repository-Root oder der Ordner selbst außerhalb eines Repositorys ist. Claude Code gibt den genauen Schlüssel in der Debug-Log-Zeile für einen übersprungenen Subagent-Hook oder Inline-MCP-Server, in der stderr-Warnung für übersprungene Allow-Regeln und in der `headersHelper not run`-Zeile für einen übersprungenen Helper aus.

Bevor Sie `claude -p` in einem Repository ausführen, das Sie nicht geschrieben haben, entscheiden Sie, was es auf Ihrem Computer ausführen kann:

* Übergeben Sie `--setting-sources user` oder setzen Sie die `settingSources` des SDK ohne Projekteinstellungen, damit Claude Code weder die Einstellungsdateien des Projekts noch seine `.mcp.json` liest
* Starten Sie mit [`--bare`](/docs/de/headless#start-faster-with-bare-mode), damit Claude Code keine Hooks, Skills, benutzerdefinierten Befehle, Subagents, Plugins oder `.mcp.json`-Server aus dem Projekt liest. Der `env`-Block des Projekts und Helfer wie `awsAuthRefresh` in seinen Einstellungsdateien gelten immer noch, und Claude Code liest `apiKeyHelper` nur aus `--settings`
* Übergeben Sie `--settings '{"disableAllHooks": true}'`, um [Hooks auszuschalten](/docs/de/hooks#disable-or-remove-hooks) für diesen Lauf. Es nur in Ihren Benutzereinstellungen zu setzen, reicht nicht aus, da die Projekteinstellungen des Repositorys Vorrang vor Ihren haben und es auf `false` zurücksetzen können
* Fügen Sie einen [`disabledMcpjsonServers`](/docs/de/settings-reference#disabledmcpjsonservers)-Eintrag hinzu, um einen `.mcp.json`-Server nach Name in jedem Sitzungstyp abzulehnen

<h2 id="example-configurations">
  Beispielkonfigurationen
</h2>

Dieses [Repository](https://github.com/anthropics/claude-code/tree/main/examples/settings) enthält Starter-Einstellungskonfigurationen für häufige Bereitstellungsszenarien. Verwenden Sie diese als Ausgangspunkte und passen Sie sie an Ihre Anforderungen an.

<h2 id="see-also">
  Siehe auch
</h2>

* [Alle Einstellungen](/docs/de/settings-reference#permission-settings): jeden Einstellungsschlüssel, einschließlich der Berechtigungsschlüssel
* [Auto-Mode konfigurieren](/docs/de/auto-mode-config): teilen Sie dem Auto-Mode-Klassifizierer mit, welche Infrastruktur Ihre Organisation vertraut
* [Sandboxing](/docs/de/sandboxing): Dateisystem- und Netzwerkisolation auf Betriebssystemebene für Bash-Befehle
* [Authentifizierung](/docs/de/authentication): richten Sie Benutzerzugriff auf Claude Code ein
* [Sicherheit](/docs/de/security): Sicherheitsvorkehrungen und Best Practices
* [Hooks](/docs/de/hooks-guide): automatisieren Sie Workflows und erweitern Sie die Berechtigungsevaluierung
