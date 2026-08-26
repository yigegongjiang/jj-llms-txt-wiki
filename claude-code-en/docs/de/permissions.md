> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Berechtigungen konfigurieren

> Kontrollieren Sie, worauf Claude Code zugreifen kann und was es mit granularen Berechtigungsregeln, Modi und verwalteten Richtlinien tun kann.

Claude Code unterstützt granulare Berechtigungen, sodass Sie genau angeben können, was der Agent tun darf und was nicht. Berechtigungseinstellungen können in die Versionskontrolle eingecheckt und an alle Entwickler in Ihrer Organisation verteilt werden, sowie von einzelnen Entwicklern angepasst werden.

<h2 id="permission-system">
  Berechtigungssystem
</h2>

Claude Code verwendet ein gestuftes Berechtigungssystem, um Leistung und Sicherheit auszugleichen:

| Werkzeugtyp   | Beispiel                     | Genehmigung erforderlich                                                                         | Verhalten „Ja, nicht mehr fragen"           |
| :------------ | :--------------------------- | :----------------------------------------------------------------------------------------------- | :------------------------------------------ |
| Nur Lesen     | Dateilesevorgänge, Grep      | Nein, innerhalb des [Arbeitsverzeichnisses und zusätzlicher Verzeichnisse](#working-directories) | N/A                                         |
| Bash-Befehle  | Shell-Ausführung             | Ja, außer einer integrierten Reihe von [schreibgeschützten Befehlen](#read-only-commands)        | Dauerhaft pro Projektverzeichnis und Befehl |
| Dateiänderung | Dateien bearbeiten/schreiben | Ja                                                                                               | Bis zum Ende der Sitzung                    |

Bei einer Bash- oder PowerShell-Berechtigungsaufforderung drücken Sie `Ctrl+E`, um eine Erklärung des Befehls anzuzeigen: was er tut, warum Claude ihn ausführt und was schiefgehen könnte, gekennzeichnet als **Niedriges Risiko**, **Mittleres Risiko** oder **Hohes Risiko**. Claude Code sendet den Befehl und Claudes eigene Beschreibung des Aufrufs an das Modell, um die Erklärung nur zu generieren, wenn Sie `Ctrl+E` drücken, nicht bei jeder Aufforderung. Das Anzeigen der Erklärung führt den Befehl nicht aus; drücken Sie `Ctrl+E` erneut, um sie auszublenden.

Um die Verknüpfung zu deaktivieren, setzen Sie [`permissionExplainerEnabled`](/docs/de/settings#global-config-settings) auf `false` in `~/.claude.json`.

<h2 id="manage-permissions">
  Berechtigungen verwalten
</h2>

Sie können Claude Code's Werkzeugberechtigungen mit `/permissions` anzeigen und verwalten. Diese Benutzeroberfläche listet alle Berechtigungsregeln und die `settings.json`-Dateien auf, aus denen sie stammen.

* **Allow**-Regeln ermöglichen Claude Code, das angegebene Werkzeug ohne manuelle Genehmigung zu verwenden.
* **Ask**-Regeln fordern eine Bestätigung auf, wenn Claude Code versucht, das angegebene Werkzeug zu verwenden.
* **Deny**-Regeln verhindern, dass Claude Code das angegebene Werkzeug verwendet.

Regeln werden in dieser Reihenfolge ausgewertet: deny, dann ask, dann allow. Die erste Übereinstimmung in dieser Reihenfolge bestimmt das Ergebnis, und die Regelspezifität ändert die Reihenfolge nicht.

Eine breite deny-Regel wie `Bash(aws *)` blockiert jeden übereinstimmenden Aufruf, einschließlich Aufrufen, die auch einer engeren allow-Regel wie `Bash(aws s3 ls)` entsprechen, daher kann eine deny-Regel keine Allowlist-Ausnahmen enthalten. Die gleiche Priorität gilt zwischen ask und allow: eine übereinstimmende ask-Regel fordert eine Bestätigung auf, auch wenn eine spezifischere allow-Regel denselben Aufruf ebenfalls erfüllt.

Deny-Regeln verhalten sich unterschiedlich, je nachdem, ob sie ein Werkzeug benennen oder ein Muster darin eingrenzen. Ein einfacher Werkzeugname wie `Bash` entfernt das Werkzeug vollständig aus Claudes Kontext, sodass Claude es nie sieht. Eine eingegrenzte Regel wie `Bash(rm *)` lässt das Werkzeug verfügbar und blockiert übereinstimmende Aufrufe, wenn Claude sie versucht.

<Note>
  Berechtigungsregeln werden von Claude Code durchgesetzt, nicht vom Modell. Anweisungen in Ihrem Prompt oder `CLAUDE.md` bestimmen, was Claude versucht zu tun, aber sie ändern nicht, was Claude Code erlaubt. Um Zugriff zu gewähren oder zu widerrufen, verwenden Sie `/permissions`, die hier beschriebenen Regeln, einen [Berechtigungsmodus](/docs/de/permission-modes) oder einen [PreToolUse-Hook](#extend-permissions-with-hooks).
</Note>

<h2 id="permission-modes">
  Berechtigungsmodi
</h2>

Claude Code unterstützt mehrere Berechtigungsmodi, die steuern, wie Werkzeugaufrufe genehmigt werden. Siehe [Berechtigungsmodi](/docs/de/permission-modes) für den Zeitpunkt der Verwendung jedes Modus. Legen Sie den `defaultMode` in Ihren [Einstellungsdateien](/docs/de/settings#settings-files) fest:

| Modus               | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Standardverhalten: fordert Genehmigung bei der ersten Verwendung jedes Werkzeugs auf. Im CLI und in den VS Code- und JetBrains-Erweiterungen sowie in der Desktop-App als „Manual" gekennzeichnet, und Claude Code akzeptiert `manual` als Alias. Die Bezeichnung und der Alias erfordern Claude Code v2.1.200 oder später. Die Bezeichnung der Desktop-App hängt nicht von Ihrer CLI-Version ab                                                                       |
| `acceptEdits`       | Akzeptiert automatisch Dateiberechtigungen und häufige Dateisystem-Befehle wie `mkdir`, `touch`, `mv` und `cp` für Pfade im Arbeitsverzeichnis oder `additionalDirectories`                                                                                                                                                                                                                                                                                            |
| `plan`              | Claude liest Dateien und führt schreibgeschützte Shell-Befehle aus, um zu erkunden, bearbeitet aber nicht Ihre Quelldateien. Im CLI und in der VS Code-Erweiterung als Plan gekennzeichnet                                                                                                                                                                                                                                                                             |
| `auto`              | Genehmigt Werkzeugaufrufe automatisch mit Hintergrund-Sicherheitsprüfungen, die überprüfen, ob Aktionen mit Ihrer Anfrage übereinstimmen                                                                                                                                                                                                                                                                                                                               |
| `dontAsk`           | Verweigert Werkzeuge automatisch, es sei denn, sie sind vorab über `/permissions` oder `permissions.allow`-Regeln genehmigt. `AskUserQuestion`, Connector-Werkzeuge [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Werkzeuge, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden verweigert, auch wenn Sie diese genehmigt haben                   |
| `bypassPermissions` | Überspringt Berechtigungsaufforderungen, außer denen, die durch explizite `ask`-Regeln erzwungen werden, Connector-Werkzeuge [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Werkzeuge, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind. Entfernungen von Dateisystem-Root oder Home-Verzeichnis wie `rm -rf /` fordern weiterhin auf als Schutzschalter |

<Warning>
  Der Modus `bypassPermissions` überspringt Berechtigungsaufforderungen, einschließlich Schreibvorgänge in `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` und `.mvn`. Verwenden Sie diesen Modus nur in isolierten Umgebungen wie Containern oder VMs, in denen Claude Code keinen Schaden anrichten kann.

  Einige wenige Aufforderungen werden in diesem Modus weiterhin angezeigt. Explizite `ask`-Regeln, Connector-Werkzeuge [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Werkzeuge, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, fordern weiterhin auf. Entfernungen, die auf das Dateisystem-Root oder Home-Verzeichnis abzielen, wie `rm -rf /` und `rm -rf ~`, fordern weiterhin auf als Schutzschalter gegen Modellfehler, einschließlich wenn der Befehl Befehlsersetzung mit `$(...)` oder Backticks oder Prozessersetzung mit `<(...)` enthält. Vor v2.1.208 forderten nur die einfache Form auf, wie `rm -rf ~` als eigenständiger Befehl eingegeben, auf; Befehle, die die Entfernung durch eine Ersetzung erreichten, forderten nicht auf.
</Warning>

Um zu verhindern, dass der Modus `bypassPermissions` oder `auto` verwendet wird, setzen Sie `permissions.disableBypassPermissionsMode` oder `permissions.disableAutoMode` auf `"disable"` in einer beliebigen [Einstellungsdatei](/docs/de/settings#settings-files). Diese sind am nützlichsten in [verwalteten Einstellungen](#managed-settings), wo sie nicht überschrieben werden können.

<h2 id="permission-rule-syntax">
  Berechtigungsregelsyntax
</h2>

Berechtigungsregeln folgen dem Format `Tool` oder `Tool(specifier)`.

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

Ablehnungs- und Anfrage-Regeln können einen Eingabeparameter auf oberster Ebene auf jedem Werkzeug mit `Tool(param:value)` abgleichen. Die Regel passt, wenn Claude das Werkzeug mit diesem Parameter aufruft, der auf diesen genauen Wert gesetzt ist. Eine Zulassungsregel für einen Parameterwert würde nicht feststellen, dass der Aufruf insgesamt sicher ist, daher verwenden Zulassungsregeln weiterhin die eigene Spezifizierer-Syntax jedes Werkzeugs. Dies funktioniert für jeden Skalarparameter, den das Werkzeug akzeptiert:

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

Felder, die ein Werkzeug bereits mit seinen eigenen kanonisierenden Regeln abgleicht, können auf diese Weise nicht abgeglichen werden: `command` für Bash und PowerShell, `file_path` für Read, Edit und Write, `path` für Grep und Glob, `notebook_path` für NotebookEdit und `url` für WebFetch. Eine Regel wie `Bash(command:rm *)` könnte durch einen zusammengesetzten Befehl umgangen werden, daher ignoriert Claude Code sie und gibt eine Startwarnmeldung aus. Verwenden Sie stattdessen `Bash(rm *)`, `Read(./path)` oder `WebFetch(domain:host)`.

<h3 id="wildcard-patterns">
  Wildcard-Muster
</h3>

Bash-Regeln unterstützen Glob-Muster mit `*`. Platzhalter können an jeder Position im Befehl erscheinen. Diese Konfiguration ermöglicht npm- und git-Commit-Befehle, blockiert aber git push:

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)",
      "Bash(git * main)",
      "Bash(* --version)",
      "Bash(* --help *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Das Leerzeichen vor `*` ist wichtig: `Bash(ls *)` gleicht `ls -la` ab, aber nicht `lsof`, während `Bash(ls*)` beide abgleicht. Das Suffix `:*` ist eine gleichwertige Möglichkeit, einen nachgestellten Platzhalter zu schreiben, daher gleicht `Bash(ls:*)` die gleichen Befehle ab wie `Bash(ls *)`.

Der Berechtigungsdialog schreibt die durch Leerzeichen getrennte Form, wenn Sie „Ja, nicht mehr fragen" für ein Befehlspräfix auswählen. Die Form `:*` wird nur am Ende eines Musters erkannt. In einem Muster wie `Bash(git:* push)` wird der Doppelpunkt als Literalzeichen behandelt und passt nicht zu git-Befehlen.

<h3 id="tool-name-wildcards">
  Werkzeugnamen-Wildcards
</h3>

Ablehnungs- und Anfrage-Regeln akzeptieren auch Glob-Muster in der Werkzeugnamen-Position. Das Muster muss dem vollständigen Werkzeugnamen entsprechen: `"*"` gleicht jedes Werkzeug ab, und `"mcp__*"` gleicht jedes MCP-Werkzeug über alle Server hinweg ab. Ein Werkzeug, das durch eine Ablehnungsregel mit bloßem Namen abgeglichen wird, wird aus Claudes Kontext entfernt, genauso wie ein bloßer Werkzeugname. Diese Konfiguration lehnt jedes MCP-Werkzeug ab:

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

Eine Ablehnungs- oder Anfrage-Regel, deren Werkzeugname mit keinem bekannten Werkzeug übereinstimmt, erzeugt eine Startwarnmeldung, um Tippfehler zu erfassen. Werkzeugnamen, die `_` oder `*` enthalten, sind von der Überprüfung ausgenommen.

Das Etikett, das für ein Werkzeug im Transkript und im Berechtigungsdialog angezeigt wird, kann sich vom kanonischen Namen unterscheiden. Beispielsweise hat das Werkzeug mit der Bezeichnung `Stop Task` im Transkript den kanonischen Namen `TaskStop`. Berechtigungsregeln und [Hook-Matcher](/docs/de/hooks) gleichen nur den kanonischen Namen ab, daher passt eine Regel, die als `Stop Task` geschrieben ist, nicht. Für Ablehnungs- und Anfrage-Regeln erfasst die obige Startwarnmeldung die Nichtübereinstimmung. Verwenden Sie die kanonischen Namen, die in der [Werkzeugreferenz](/docs/de/tools-reference) aufgelistet sind.

<h2 id="tool-specific-permission-rules">
  Werkzeugspezifische Berechtigungsregeln
</h2>

<h3 id="bash">
  Bash
</h3>

Bash-Berechtigungsregeln unterstützen Wildcard-Abgleich mit `*`. Platzhalter können an jeder Position im Befehl erscheinen, einschließlich am Anfang, in der Mitte oder am Ende:

* `Bash(npm run build)` gleicht den genauen Bash-Befehl `npm run build` ab
* `Bash(npm run test *)` gleicht Bash-Befehle ab, die mit `npm run test` beginnen
* `Bash(npm *)` gleicht jeden Befehl ab, der mit `npm ` beginnt
* `Bash(* install)` gleicht jeden Befehl ab, der mit ` install` endet
* `Bash(git * main)` gleicht Befehle wie `git checkout main` und `git log --oneline main` ab

Ein einzelnes `*` gleicht jede Zeichenfolge ab, einschließlich Leerzeichen, daher kann ein Platzhalter mehrere Argumente umfassen. `Bash(git *)` gleicht `git log --oneline --all` ab, und `Bash(git * main)` gleicht sowohl `git push origin main` als auch `git merge main` ab.

Wenn `*` am Ende mit einem Leerzeichen davor erscheint (wie `Bash(ls *)`), wird eine Wortgrenze erzwungen, die erfordert, dass dem Präfix ein Leerzeichen oder das Ende der Zeichenkette folgt. Zum Beispiel gleicht `Bash(ls *)` `ls -la` ab, aber nicht `lsof`. Im Gegensatz dazu gleicht `Bash(ls*)` ohne Leerzeichen sowohl `ls -la` als auch `lsof` ab, da es keine Wortgrenzbeschränkung gibt.

<h4 id="compound-commands">
  Zusammengesetzte Befehle
</h4>

<Tip>
  Claude Code ist sich Shell-Operatoren bewusst, daher gibt eine Regel wie `Bash(safe-cmd *)` ihm nicht die Berechtigung, den Befehl `safe-cmd && other-cmd` auszuführen. Die erkannten Befehlstrennzeichen sind `&&`, `||`, `;`, `|`, `|&`, `&` und Zeilenumbrüche. Eine Regel muss jeden Unterbefehl unabhängig abgleichen.
</Tip>

Wenn Sie einen zusammengesetzten Befehl mit „Ja, nicht mehr fragen" genehmigen, speichert Claude Code eine separate Regel für jeden Unterbefehl, der Genehmigung erfordert, anstelle einer einzelnen Regel für die vollständige zusammengesetzte Zeichenkette. Zum Beispiel speichert das Genehmigen von `git status && npm test` eine Regel für `npm test`, sodass zukünftige `npm test`-Aufrufe erkannt werden, unabhängig davon, was dem `&&` vorausgeht. Unterbefehle wie `cd` in ein Unterverzeichnis generieren ihre eigene Read-Regel für diesen Pfad. Für einen einzelnen zusammengesetzten Befehl können bis zu 5 Regeln gespeichert werden.

<h4 id="process-wrappers">
  Prozess-Wrapper
</h4>

Vor dem Abgleich von Bash-Regeln entfernt Claude Code einen festen Satz von Prozess-Wrappern, daher gleicht eine Regel wie `Bash(npm test *)` auch `timeout 30 npm test` ab. Die erkannten Wrapper sind `timeout`, `time`, `nice`, `nohup` und `stdbuf`.

Auch bloßes `xargs` wird entfernt, daher gleicht `Bash(grep *)` `xargs grep pattern` ab. Das Entfernen gilt nur, wenn `xargs` keine Flags hat: Ein Aufruf wie `xargs -n1 grep pattern` wird als `xargs`-Befehl abgeglichen, daher decken Regeln, die für den inneren Befehl geschrieben wurden, ihn nicht ab.

Diese Wrapper-Liste ist integriert und nicht konfigurierbar. Entwicklungsumgebungs-Runner wie `direnv exec`, `devbox run`, `mise exec`, `npx` und `docker exec` sind nicht in der Liste. Da diese Tools ihre Argumente als Befehl ausführen, gleicht eine Regel wie `Bash(devbox run *)` alles ab, was nach `run` kommt, einschließlich `devbox run rm -rf .`. Um Arbeit innerhalb eines Umgebungs-Runners zu genehmigen, schreiben Sie eine spezifische Regel, die sowohl den Runner als auch den inneren Befehl enthält, wie `Bash(devbox run npm test)`. Fügen Sie eine Regel pro innerem Befehl hinzu, den Sie zulassen möchten.

Exec-Wrapper wie `watch`, `setsid`, `ionice` und `flock` fordern immer auf und können nicht durch eine Präfixregel wie `Bash(watch *)` automatisch genehmigt werden. Das gleiche gilt für `find` mit `-exec` oder `-delete`: Eine `Bash(find *)` Regel deckt diese Formen nicht ab. Um einen spezifischen Aufruf zu genehmigen, schreiben Sie eine exakte Übereinstimmungsregel für die vollständige Befehlszeichenkette.

<h4 id="read-only-commands">
  Schreibgeschützte Befehle
</h4>

Claude Code erkennt einen integrierten Satz von Bash-Befehlen als schreibgeschützt und führt sie ohne Berechtigungsaufforderung in jedem Modus aus. Diese umfassen `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` und schreibgeschützte Formen von `git`. Der Satz ist nicht konfigurierbar; um eine Aufforderung für einen dieser Befehle zu erfordern, fügen Sie eine `ask`- oder `deny`-Regel dafür hinzu.

Unquotierte Glob-Muster sind für Befehle zulässig, deren jedes Flag schreibgeschützt ist, daher laufen `ls *.ts` und `wc -l src/*.py` ohne Aufforderung. Befehle mit schreibfähigen oder ausführungsfähigen Flags, wie `find`, `sort`, `sed` und `git`, fordern immer noch auf, wenn ein unquotiertes Glob vorhanden ist, da das Glob zu einem Flag wie `-delete` expandieren könnte.

Ein `cd` in einen Pfad innerhalb Ihres Arbeitsverzeichnisses oder eines [zusätzlichen Verzeichnisses](#working-directories) ist auch schreibgeschützt. Ein zusammengesetzter Befehl wie `cd packages/api && ls` läuft ohne Aufforderung, wenn jeder Teil auf eigene Faust qualifiziert. Das Kombinieren von `cd` mit `git` in einem zusammengesetzten Befehl fordert auf, wenn die `cd` in ein anderes Verzeichnis wechselt, da das Ausführen von `git` in einem neuen Verzeichnis die Hooks dieses Verzeichnisses ausführen kann. Ein `cd` dessen Ziel zum aktuellen Arbeitsverzeichnis aufgelöst wird, ist ein No-Op und löst diese Aufforderung nicht aus.

Das Kombinieren von `cd` mit einer Ausgabeumleitungen in einem zusammengesetzten Befehl fordert auch auf, wenn Claude Code nicht bestimmen kann, in welches Verzeichnis das Umleitungsziel nach der Ausführung von `cd` aufgelöst wird. Ein Befehl, dessen einziges Umleitungsziel `/dev/null` ist, wie `cd app; grep -r pattern . 2>/dev/null`, löst diese Aufforderung nicht aus, da `/dev/null` nicht vom Arbeitsverzeichnis abhängt. Vor v2.1.207 forderte ein zusammengesetzter Befehl mit `cd` für jede Ausgabeumleitungen auf, einschließlich einer, deren einziges Ziel `/dev/null` war.

<Warning>
  Bash-Berechtigungsmuster, die versuchen, Befehlsargumente einzuschränken, sind fragil. Zum Beispiel beabsichtigt `Bash(curl http://github.com/ *)`, curl auf GitHub-URLs zu beschränken, wird aber Variationen nicht abgleichen wie:

  * Optionen vor URL: `curl -X GET http://github.com/...`
  * Anderes Protokoll: `curl https://github.com/...`
  * Umleitungen: `curl -L http://bit.ly/xyz`, die zu GitHub umleitet
  * Variablen: `URL=http://github.com && curl $URL`
  * Zusätzliche Leerzeichen: `curl  http://github.com`

  Für zuverlässigere URL-Filterung sollten Sie erwägen:

  * **Bash-Netzwerkwerkzeuge einschränken**: Verwenden Sie Deny-Regeln, um `curl`, `wget` und ähnliche Befehle zu blockieren, verwenden Sie dann das WebFetch-Werkzeug mit `WebFetch(domain:github.com)`-Berechtigung für zulässige Domänen
  * **PreToolUse-Hooks verwenden**: Implementieren Sie einen Hook, der URLs in Bash-Befehlen validiert und nicht zulässige Domänen blockiert
  * **CLAUDE.md-Anleitung hinzufügen**: Beschreiben Sie Ihre zulässigen curl-Muster in `CLAUDE.md`. Dies beeinflusst, was Claude versucht, erzwingt aber keine Grenze, daher kombinieren Sie es mit einer der obigen Optionen

  Beachten Sie, dass die alleinige Verwendung von WebFetch keinen Netzwerkzugriff verhindert. Wenn Bash zulässig ist, kann Claude immer noch `curl`, `wget` oder andere Werkzeuge verwenden, um auf jede URL zuzugreifen.
</Warning>

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

`Edit`-Regeln gelten für alle integrierten Werkzeuge, die Dateien bearbeiten. Claude versucht nach besten Kräften, `Read`-Regeln auf alle integrierten Werkzeuge anzuwenden, die Dateien lesen, wie Grep und Glob, auf `@file`-Erwähnungen in Ihren Eingabeaufforderungen und auf die Auswahl und offene Datei-Kontexte, die ein verbundener [IDE](/docs/de/vs-code#the-built-in-ide-mcp-server) mit Claude teilt.

Eine `Read`-Deny-Regel blockiert auch das [Edit-Werkzeug](/docs/de/errors#file-is-covered-by-a-read-deny-rule) auf demselben Pfad, einschließlich der Erstellung einer neuen Datei dort. Write und NotebookEdit sind nicht abgedeckt, daher fügen Sie eine `Edit`-Deny-Regel für Pfade hinzu, die kein Werkzeug ändern darf. Erfordert Claude Code v2.1.208 oder später.

<Warning>
  Read- und Edit-Deny-Regeln gelten für Claude's integrierte Dateiwerkzeuge und für Dateibefehle, die Claude Code in Bash erkennt, wie `cat`, `head`, `tail` und `sed`. Sie gelten nicht für beliebige Unterprozesse, die Dateien indirekt lesen oder schreiben, wie ein Python- oder Node-Skript, das Dateien selbst öffnet. Für OS-Ebenen-Durchsetzung, die alle Prozesse daran hindert, auf einen Pfad zuzugreifen, [aktivieren Sie die Sandbox](/docs/de/sandboxing).
</Warning>

Read- und Edit-Regeln folgen beide der [gitignore](https://git-scm.com/docs/gitignore)-Spezifikation mit vier unterschiedlichen Mustertypen:

| Muster               | Bedeutung                              | Beispiel                         | Gleicht ab                                           |
| -------------------- | -------------------------------------- | -------------------------------- | ---------------------------------------------------- |
| `//path`             | Absoluter Pfad vom Dateisystem-Root    | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                            |
| `~/path`             | Pfad vom Home-Verzeichnis              | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                       |
| `/path`              | Pfad relativ zur Einstellungsquelle    | `Edit(/src/**/*.ts)`             | `<project root>/src/**/*.ts` in Projekteinstellungen |
| `path` oder `./path` | Pfad relativ zum aktuellen Verzeichnis | `Read(*.env)`                    | `<cwd>/*.env`                                        |

<Warning>
  Ein Muster wie `/Users/alice/file` ist kein absoluter Pfad. Der einzelne führende Schrägstrich verankert sich an der Einstellungsquelle, nicht am Dateisystem-Root. Verwenden Sie `//Users/alice/file` für absolute Pfade.
</Warning>

Ein `/path`-Muster verankert sich am Verzeichnis, das der Einstellungsdatei zugeordnet ist, die es definiert, daher gleicht die gleiche Regel verschiedene Orte ab, je nachdem, wo Sie sie platzieren:

| Regel definiert in                                              | `/path` wird aufgelöst zu  |
| :-------------------------------------------------------------- | :------------------------- |
| Projekt- oder lokale Einstellungen, wie `.claude/settings.json` | `<project root>/path`      |
| Benutzereinstellungen unter `~/.claude/settings.json`           | `~/.claude/path`           |
| Eine Datei, die mit `--settings <file>` übergeben wird          | `<directory of file>/path` |
| CLI-Flags, `/permissions` oder Sitzungsregeln                   | `<original cwd>/path`      |

Eine Deny-Regel wie `Read(/secrets/**)` in Benutzereinstellungen blockiert `~/.claude/secrets/**`, nicht ein `secrets`-Verzeichnis in Ihrem Projekt. Um eine Regel in Benutzereinstellungen zu schreiben, die sich in jedem Projekt anwendet, verwenden Sie stattdessen einen `//` absoluten Pfad oder einen `~/` Home-relativen Pfad.

Unter Windows werden Pfade vor dem Abgleich in POSIX-Form normalisiert. `C:\Users\alice` wird zu `/c/Users/alice`, verwenden Sie also `//c/**/.env`, um `.env`-Dateien überall auf diesem Laufwerk abzugleichen. Um über alle Laufwerke hinweg abzugleichen, verwenden Sie `//**/.env`.

Beispiele:

* `Edit(/docs/**)`: Bearbeitungen in `<project>/docs/`, nicht `/docs/` oder `<project>/.claude/docs/`
* `Read(~/.zshrc)`: liest die `.zshrc` Ihres Home-Verzeichnisses
* `Edit(//tmp/scratch.txt)`: bearbeitet den absoluten Pfad `/tmp/scratch.txt`
* `Read(src/**)`: liest aus `<current-directory>/src/`

Eine Regel gleicht nur Dateien unter ihrem Anker ab, daher bestimmt der Anker, wie weit eine Deny-Regel reicht. Bare Dateinamen folgen gitignore-Semantik und gleichen in jeder Tiefe ab, daher sind `Read(.env)` und `Read(**/.env)` gleichwertig:

| Deny-Regel                        | Blockiert                                           | Blockiert nicht                                                       |
| --------------------------------- | --------------------------------------------------- | --------------------------------------------------------------------- |
| `Read(.env)` oder `Read(**/.env)` | jede `.env` im oder unter dem aktuellen Verzeichnis | `.env` in einem übergeordneten Verzeichnis oder einem anderen Projekt |
| `Read(//**/.env)`                 | jede `.env` überall im Dateisystem                  | nichts; die Regel ist am Dateisystem-Root verankert                   |

<Note>
  In gitignore-Mustern gleicht `*` Dateien in einem einzelnen Verzeichnis ab, während `**` rekursiv über Verzeichnisse hinweg abgleicht. Um allen Dateizugriff zu ermöglichen, verwenden Sie einfach den Werkzeugnamen ohne Klammern: `Read`, `Edit` oder `Write`.
</Note>

Wenn Claude auf einen Symlink zugreift, überprüfen Berechtigungsregeln zwei Pfade: den Symlink selbst und die Datei, auf die er verweist. Allow- und Deny-Regeln behandeln dieses Paar unterschiedlich: Allow-Regeln fallen auf Aufforderungen zurück, während Deny-Regeln direkt blockieren.

* **Allow-Regeln**: gelten nur, wenn sowohl der Symlink-Pfad als auch sein Ziel übereinstimmen. Ein Symlink in einem zulässigen Verzeichnis, der außerhalb davon verweist, fordert Sie immer noch auf.
* **Deny-Regeln**: gelten, wenn entweder der Symlink-Pfad oder sein Ziel übereinstimmt. Ein Symlink, der auf eine verweigerte Datei verweist, ist selbst verweigert.

Zum Beispiel mit `Read(./project/**)` zulässig und `Read(~/.ssh/**)` verweigert, wird ein Symlink bei `./project/key`, der auf `~/.ssh/id_rsa` verweist, blockiert: Das Ziel schlägt die Allow-Regel fehl und passt zur Deny-Regel.

<h3 id="webfetch">
  WebFetch
</h3>

WebFetch-Regeln verwenden ein `domain:`-Präfix und gleichen gegen den Hostnamen der angeforderten URL ab. Der Abgleich ist case-insensitiv, unterstützt `*`-Platzhalter und entfernt einen nachgestellten `.` sowohl aus der Regel als auch aus dem Hostnamen, daher werden `example.com.` und `example.com` gleich behandelt.

* `WebFetch(domain:example.com)` gleicht Anfragen an `example.com` ab
* `WebFetch(domain:*.example.com)` gleicht jede Subdomain in jeder Tiefe ab, wie `api.example.com` oder `a.b.example.com`, aber nicht `example.com` selbst
* `WebFetch(domain:*)` gleicht jede Domain ab und ist gleichwertig mit einer bloßen `WebFetch`-Regel

An jeder Position außer einem führenden `*.` oder einem bloßen `*` gleicht der Platzhalter nur den Text zwischen zwei Punkten ab. `WebFetch(domain:example.*)` gleicht `example.org` ab, wobei `*` zu `org` wird, aber nicht `example.evil.com`, wobei `*` zu `evil.com` werden müsste und einen Punkt überschreiten würde. Dies verhindert, dass ein nachgestellter Platzhalter Domänen abgleicht, die ein Angreifer registrieren könnte.

<h3 id="mcp">
  MCP
</h3>

MCP-Regeln verwenden den Servernamen wie in Claude Code konfiguriert, optional gefolgt vom Namen eines Werkzeugs von diesem Server.

* `mcp__puppeteer` gleicht jedes Werkzeug ab, das vom `puppeteer`-Server bereitgestellt wird
* `mcp__puppeteer__*` verwendet Wildcard-Syntax und gleicht auch alle Werkzeuge vom `puppeteer`-Server ab
* `mcp__puppeteer__puppeteer_navigate` gleicht das `puppeteer_navigate`-Werkzeug ab, das vom `puppeteer`-Server bereitgestellt wird

Wenn Ihre Organisation ein [claude.ai-Connector](/docs/de/mcp#organization-controls-on-connector-tools)-Werkzeug auf `ask` gesetzt hat, nehmen Allow-Regeln für dieses Werkzeug keine Wirkung: Claude Code fordert bei jedem Aufruf auf, auch in `auto`- und `bypassPermissions`-Modi. Im `dontAsk`-Modus, der niemals auffordert, verweigert Claude Code den Aufruf stattdessen. Connector-Werkzeuge erscheinen als `mcp__claude_ai_<server>__<tool>`.

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

Eine bloße `Cd`-Deny-Regel deaktiviert `/cd` vollständig. Eine `Cd(<path-pattern>)`-Deny-Regel blockiert übereinstimmende Ziele. Deny-Regeln überprüfen jede Schreibweise des Ziels, einschließlich jedes Symlink-Hops, durch den es sich auflöst, daher blockiert eine Regel, die für einen Pfad geschrieben wurde, auch Ziele, die sich darin auflösen.

Das Hinzufügen einer `Cd`-Allow-Regel schaltet `/cd` in den Allowlist-Modus: Das aufgelöste Zielverzeichnis muss einer Ihrer Allow-Regeln entsprechen, oder `/cd` weigert sich. Ohne konfigurierte `Cd`-Regeln behält `/cd` sein Standardverhalten bei und fordert Sie auf, einem unbekannten Verzeichnis zu vertrauen.

Pfadmuster teilen die `//`, `~/` und `/` Anker von [Read- und Edit-Regeln](#read-and-edit), aber der Abgleich ist am gesamten Verzeichnispfad verankert, nicht im gitignore-Stil. `*` gleicht genau ein Pfadsegment ab und `**` gleicht über Segmente hinweg ab. Ein nachgestelltes `/**` gleicht auch seinen benannten Root ab.

| Regel                 | Gleicht ab                                      | Gleicht nicht ab                     |
| --------------------- | ----------------------------------------------- | ------------------------------------ |
| `Cd(~/code/*)`        | `~/code/app`                                    | `~/code/app/src`, `~/code`           |
| `Cd(~/code/**)`       | `~/code` und jedes Verzeichnis darunter         | Verzeichnisse außerhalb von `~/code` |
| `Cd(**/node_modules)` | jedes `node_modules`-Verzeichnis in jeder Tiefe | `node_modules/pkg`                   |

<h2 id="extend-permissions-with-hooks">
  Berechtigungen mit Hooks erweitern
</h2>

[Claude Code Hooks](/docs/de/hooks-guide) bieten eine Möglichkeit, benutzerdefinierte Shell-Befehle zu registrieren, um die Berechtigungsevaluierung zur Laufzeit durchzuführen. Wenn Claude Code einen Werkzeugaufruf tätigt, werden PreToolUse-Hooks vor dem Berechtigungssystem ausgeführt. Die Hook-Ausgabe kann den Werkzeugaufruf verweigern, eine Aufforderung erzwingen oder die Aufforderung überspringen, um den Aufruf fortzufahren.

Hook-Entscheidungen umgehen keine Berechtigungsregeln. Deny- und Ask-Regeln werden unabhängig davon ausgewertet, was ein PreToolUse-Hook zurückgibt, daher blockiert eine übereinstimmende Deny-Regel den Aufruf und eine übereinstimmende Ask-Regel fordert immer noch auf, selbst wenn der Hook `"allow"` oder `"ask"` zurückgegeben hat. Dies bewahrt die Deny-First-Priorität, die in [Berechtigungen verwalten](#manage-permissions) beschrieben ist, einschließlich Deny-Regeln, die in verwalteten Einstellungen festgelegt sind.

Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden auch weiterhin aufgefordert, wenn ein Hook `"allow"` zurückgibt.

Ein blockierender Hook hat auch Vorrang vor Allow-Regeln. Ein Hook, der mit Code 2 beendet wird, stoppt den Werkzeugaufruf, bevor Berechtigungsregeln ausgewertet werden, daher gilt die Blockierung auch dann, wenn eine Allow-Regel den Aufruf sonst zulassen würde. Um alle Bash-Befehle ohne Aufforderungen auszuführen, außer für einige, die Sie blockieren möchten, fügen Sie `"Bash"` zu Ihrer Allow-Liste hinzu und registrieren Sie einen PreToolUse-Hook, der diese spezifischen Befehle ablehnt. Siehe [Bearbeitungen geschützter Dateien blockieren](/docs/de/hooks-guide#block-edits-to-protected-files) für ein Hook-Skript, das Sie anpassen können.

<h2 id="working-directories">
  Arbeitsverzeichnisse
</h2>

Standardmäßig hat Claude Zugriff auf Dateien in dem Verzeichnis, in dem es gestartet wurde. Sie können diesen Zugriff erweitern:

* **Beim Start**: Verwenden Sie das CLI-Argument `--add-dir <path>`
* **Während der Sitzung**: Verwenden Sie den Befehl `/add-dir`
* **Persistente Konfiguration**: Fügen Sie zu `additionalDirectories` in [Einstellungsdateien](/docs/de/settings#settings-files) hinzu

Dateien in zusätzlichen Verzeichnissen folgen den gleichen Berechtigungsregeln wie das ursprüngliche Arbeitsverzeichnis: Sie werden lesbar ohne Aufforderungen, und Dateiberechtigungen folgen dem aktuellen Berechtigungsmodus.

In Hintergrund-Sitzungen auf macOS fordert der Sitzungs-Host Zugriff auf geschützte Ordner wie `~/Desktop`, `~/Documents` und `~/Downloads` separat von Ihrem Terminal an, wenn Claude dort Dateien lesen oder schreiben muss; wenn Lesevorgänge dort mit `Operation not permitted` fehlschlagen, siehe [wie man Ordnerzugriff auf Hintergrund-Sitzungen auf macOS gewährt](/docs/de/agent-view#background-sessions-can't-read-desktop-documents-or-downloads-on-macos).

Um das primäre Arbeitsverzeichnis der Sitzung zu ändern, anstatt ein weiteres hinzuzufügen, verwenden Sie [`/cd`](/docs/de/commands). Der Befehl `/cd` erfordert Claude Code v2.1.169 oder später. Im Gegensatz zu `/add-dir` verlagert er die Sitzung: Die `CLAUDE.md` des neuen Verzeichnisses wird geladen und `--resume` findet die Sitzung von dort aus.

<h3 id="additional-directories-grant-file-access-not-configuration">
  Zusätzliche Verzeichnisse gewähren Dateizugriff, keine Konfiguration
</h3>

Das Hinzufügen eines Verzeichnisses erweitert, wo Claude Dateien lesen und bearbeiten kann. Es macht dieses Verzeichnis nicht zu einem vollständigen Konfigurationsroot: Die meisten `.claude/`-Konfigurationen werden nicht aus zusätzlichen Verzeichnissen erkannt, obwohl einige Typen als Ausnahmen geladen werden.

Diese Ausnahmen gelten nur für Verzeichnisse, die mit dem Flag `--add-dir` oder dem Befehl `/add-dir` hinzugefügt wurden. Verzeichnisse, die in `permissions.additionalDirectories` in einer Einstellungsdatei aufgelistet sind, gewähren nur Dateizugriff und laden keine der folgenden Konfigurationen.

Die folgenden Konfigurationstypen werden aus `--add-dir`-Verzeichnissen geladen:

| Konfiguration                                                                              | Geladen aus `--add-dir`                                                                                                                                                       |
| :----------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/de/skills) in `.claude/skills/`                                                  | Ja, mit Live-Reload                                                                                                                                                           |
| [Subagents](/docs/de/sub-agents) in `.claude/agents/`                                           | Ja                                                                                                                                                                            |
| [Einstellungen](/docs/de/settings) in `.claude/settings.json` und `.claude/settings.local.json` | Nur `enabledPlugins` und `extraKnownMarketplaces` Schlüssel                                                                                                                   |
| [CLAUDE.md](/docs/de/memory)-Dateien, `.claude/rules/` und `CLAUDE.local.md`                    | Nur wenn `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` gesetzt ist. `CLAUDE.local.md` erfordert zusätzlich die `local`-Einstellungsquelle, die standardmäßig aktiviert ist |

Befehle und Ausgabestile werden aus dem aktuellen Arbeitsverzeichnis und seinen übergeordneten Verzeichnissen, Ihrem Benutzerverzeichnis unter `~/.claude/` und verwalteten Einstellungen erkannt. Hooks und andere `settings.json`-Schlüssel werden aus dem `.claude/`-Ordner des aktuellen Arbeitsverzeichnisses ohne Fallback für übergeordnete Verzeichnisse geladen, zusammen mit Ihren Benutzer-`~/.claude/settings.json` und verwalteten Einstellungen. Um diese Konfiguration über Projekte hinweg zu teilen, verwenden Sie einen dieser Ansätze:

* **Benutzergesteuerte Konfiguration**: Platzieren Sie Dateien in `~/.claude/agents/`, `~/.claude/output-styles/` oder `~/.claude/settings.json`, um sie in jedem Projekt verfügbar zu machen
* **Plugins**: Verpacken und verteilen Sie Konfiguration als [Plugin](/docs/de/plugins), das Teams installieren können
* **Starten Sie aus dem Konfigurationsverzeichnis**: Führen Sie Claude Code aus dem Verzeichnis aus, das die `.claude/`-Konfiguration enthält, die Sie verwenden möchten

<h2 id="how-permissions-interact-with-sandboxing">
  Wie Berechtigungen mit Sandboxing interagieren
</h2>

Berechtigungen und [Sandboxing](/docs/de/sandboxing) sind komplementäre Sicherheitsebenen:

* **Berechtigungen** steuern, welche Werkzeuge Claude Code verwenden kann und auf welche Dateien oder Domänen es zugreifen kann. Sie gelten für alle Werkzeuge, einschließlich Bash, Read, Edit, WebFetch und MCP.
* **Sandboxing** bietet OS-Ebenen-Durchsetzung, die den Zugriff des Bash-Werkzeugs auf das Dateisystem und das Netzwerk einschränkt. Es gilt nur für Bash-Befehle und ihre untergeordneten Prozesse.

Verwenden Sie beide für Defense-in-Depth:

* Berechtigungs-Deny-Regeln blockieren Claude daran, überhaupt zu versuchen, auf eingeschränkte Ressourcen zuzugreifen
* Sandbox-Einschränkungen verhindern, dass Bash-Befehle Ressourcen außerhalb definierter Grenzen erreichen, selbst wenn eine Prompt-Injection Claude's Entscheidungsfindung umgeht
* Dateisystem-Einschränkungen in der Sandbox kombinieren die [`sandbox.filesystem`](/docs/de/sandboxing)-Einstellungen mit Read- und Edit-Deny-Regeln; beide werden in die endgültige Sandbox-Grenze zusammengeführt
* Netzwerk-Einschränkungen kombinieren WebFetch-Berechtigungsregeln mit den `allowedDomains`- und `deniedDomains`-Listen der Sandbox

Wenn Sandboxing mit `autoAllowBashIfSandboxed: true` aktiviert ist, was die Standardeinstellung ist, laufen sandboxed Bash-Befehle ohne Aufforderung, selbst wenn Ihre Berechtigungen eine einfache `Bash` Ask-Regel oder die [äquivalente `Bash(*)` Form](#match-all-uses-of-a-tool) enthalten: die Sandbox-Grenze ersetzt diese Ganz-Werkzeug-Aufforderung. Diese Überprüfungen gelten weiterhin:

* Inhaltsgebundene Ask-Regeln wie `Bash(git push *)` erzwingen weiterhin eine Aufforderung
* Explizite Deny-Regeln gelten weiterhin
* `rm`- oder `rmdir`-Befehle, die auf `/`, Ihr Home-Verzeichnis oder andere kritische Systempfade abzielen, lösen weiterhin eine Aufforderung aus

Befehle, die nicht sandboxed ausgeführt werden können, wie ausgeschlossene Befehle, respektieren die einfache `Bash` Ask-Regel wie gewöhnlich. Siehe [Sandbox-Modi](/docs/de/sandboxing#sandbox-modes), um dieses Verhalten zu ändern.

<h2 id="managed-settings">
  Verwaltete Einstellungen
</h2>

Für Organisationen, die eine zentralisierte Kontrolle über die Claude Code-Konfiguration benötigen, können Administratoren verwaltete Einstellungen bereitstellen, die nicht von Benutzer- oder Projekteinstellungen überschrieben werden können. Diese Richtlinieneinstellungen folgen dem gleichen Format wie reguläre Einstellungsdateien und können über MDM/OS-Ebenen-Richtlinien, verwaltete Einstellungsdateien, [servergesteuerte Einstellungen](/docs/de/server-managed-settings) oder ein selbstgehostetes [Claude Apps Gateway](/docs/de/claude-apps-gateway) bereitgestellt werden. Siehe [Einstellungsdateien](/docs/de/settings#settings-files) für Bereitstellungsmechanismen und Dateispeicherorte.

<h3 id="managed-only-settings">
  Nur verwaltete Einstellungen
</h3>

Die folgenden Einstellungen sind nur in verwalteten Einstellungen wirksam. Das Platzieren in Benutzer- oder Projekteinstellungsdateien hat keine Auswirkung.

| Einstellung                                    | Beschreibung                                                                                                                                                                                                                                                                                                                                           |
| :--------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowAllClaudeAiMcps`                         | Wenn `true`, werden claude.ai-Konnektoren zusammen mit einer bereitgestellten `managed-mcp.json` geladen, anstatt durch ihre exklusive Kontrolle unterdrückt zu werden. Siehe [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp)                                                                                                                          |
| `allowedChannelPlugins`                        | Zulassungsliste von Channel-Plugins, die Nachrichten pushen dürfen. Ersetzt die Standard-Anthropic-Zulassungsliste, wenn gesetzt. Erfordert `channelsEnabled: true`. Siehe [Einschränken Sie, welche Channel-Plugins ausgeführt werden können](/docs/de/channels#restrict-which-channel-plugins-can-run)                                                    |
| `allowManagedHooksOnly`                        | Wenn `true`, werden nur verwaltete Hooks, SDK-Hooks und Hooks aus Plugins, die in verwalteten Einstellungen `enabledPlugins` erzwungen sind, geladen. Benutzer-, Projekt- und alle anderen Plugin-Hooks werden blockiert                                                                                                                               |
| `allowManagedMcpServersOnly`                   | Wenn `true`, werden nur `allowedMcpServers` aus verwalteten Einstellungen berücksichtigt. `deniedMcpServers` wird immer noch aus allen Quellen zusammengeführt. Siehe [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp)                                                                                                                                  |
| `allowManagedPermissionRulesOnly`              | Wenn `true`, verhindert, dass Benutzer- und Projekteinstellungen `allow`-, `ask`- oder `deny`-Berechtigungsregeln definieren. Nur Regeln in verwalteten Einstellungen gelten. Dies wirkt sich nicht auf die MCP-Server-Zulassungsliste aus; dafür setzen Sie `allowManagedMcpServersOnly`                                                              |
| `blockedMarketplaces`                          | Blocklist von Marketplace-Quellen. Blockierte Quellen werden vor dem Download überprüft, sodass sie das Dateisystem nie berühren. Siehe [verwaltete Marketplace-Einschränkungen](/docs/de/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                             |
| `channelsEnabled`                              | Ermöglichen Sie [Channels](/docs/de/channels) für die Organisation. Siehe [Enterprise-Kontrollen](/docs/de/channels#enterprise-controls) für die Standardeinstellung bei jedem Plan                                                                                                                                                                              |
| `disableSideloadFlags`                         | Lehnen Sie die CLI-Flags `--plugin-dir`, `--plugin-url`, `--agents` und `--mcp-config` beim Start ab. Ohne dies können Benutzer `strictKnownMarketplaces` für einen einzelnen Durchlauf umgehen, indem sie diese Flags übergeben. Siehe [`disableSideloadFlags`](/docs/de/settings#available-settings). Erfordert Claude Code v2.1.193 oder später          |
| `forceRemoteSettingsRefresh`                   | Wenn `true`, blockiert CLI-Start, bis remote verwaltete Einstellungen aktuell abgerufen werden, und beendet sich, wenn der Abruf fehlschlägt. Siehe [Fail-Closed-Durchsetzung](/docs/de/server-managed-settings#enforce-fail-closed-startup)                                                                                                                |
| `pluginTrustMessage`                           | Benutzerdefinierte Nachricht, die der vor der Installation angezeigten Plugin-Vertrauenswarnung hinzugefügt wird                                                                                                                                                                                                                                       |
| `sandbox.filesystem.allowManagedReadPathsOnly` | Wenn `true`, werden nur `filesystem.allowRead`-Pfade aus verwalteten Einstellungen berücksichtigt. `denyRead` wird immer noch aus allen Quellen zusammengeführt                                                                                                                                                                                        |
| `sandbox.network.allowManagedDomainsOnly`      | Wenn `true`, werden nur `allowedDomains` und `WebFetch(domain:...)`-Allow-Regeln aus verwalteten Einstellungen berücksichtigt. Nicht zulässige Domänen werden automatisch blockiert, ohne den Benutzer zu fragen. Verweigerte Domänen werden immer noch aus allen Quellen zusammengeführt                                                              |
| `strictKnownMarketplaces`                      | Steuert, welche Plugin-Marketplace-Quellen Benutzer hinzufügen und Plugins installieren können. Siehe [verwaltete Marketplace-Einschränkungen](/docs/de/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                                                               |
| `strictPluginOnlyCustomization`                | Blockieren Sie Skills, Agents, Hooks und MCP-Server aus Benutzer- und Projektquellen, sodass sie nur aus Plugins oder verwalteten Einstellungen stammen können. `true` sperrt alle vier Oberflächen; ein Array wie `["skills", "hooks"]` sperrt nur die benannten. Siehe [`strictPluginOnlyCustomization`](/docs/de/settings#strictpluginonlycustomization) |
| `wslInheritsWindowsSettings`                   | Wenn `true` im Windows HKLM-Registrierungsschlüssel oder in `C:\Program Files\ClaudeCode\managed-settings.json`, liest WSL verwaltete Einstellungen aus der Windows-Richtlinienkette zusätzlich zu `/etc/claude-code`. Siehe [Einstellungsdateien](/docs/de/settings#settings-files)                                                                        |

`disableBypassPermissionsMode` wird normalerweise in verwalteten Einstellungen platziert, um Organisationsrichtlinien durchzusetzen, funktioniert aber aus jedem Bereich. Ein Benutzer kann es in seinen eigenen Einstellungen festlegen, um sich selbst aus dem Bypass-Modus auszusperren.

<Note>
  Bei Team- und Enterprise-Plänen aktiviert oder deaktiviert ein Owner [Remote Control](/docs/de/remote-control) und [Web-Sitzungen](/docs/de/claude-code-on-the-web) organisationsweit in [Claude Code-Administratoreinstellungen](https://claude.ai/admin-settings/claude-code). Remote Control kann zusätzlich pro Gerät mit der Einstellung [`disableRemoteControl`](/docs/de/settings#available-settings) deaktiviert werden. Web-Sitzungen haben keinen verwalteten Einstellungsschlüssel pro Gerät.
</Note>

<h2 id="settings-precedence">
  Einstellungspriorität
</h2>

Berechtigungsregeln folgen der gleichen [Einstellungspriorität](/docs/de/settings#settings-precedence) wie alle anderen Claude Code-Einstellungen:

1. **Verwaltete Einstellungen**: können von keiner anderen Ebene überschrieben werden, einschließlich Befehlszeilenargumenten
2. **Befehlszeilenargumente**: temporäre Sitzungsüberschreibungen
3. **Lokale Projekteinstellungen** (`.claude/settings.local.json`)
4. **Gemeinsame Projekteinstellungen** (`.claude/settings.json`)
5. **Benutzereinstellungen** (`~/.claude/settings.json`)

Wenn ein Werkzeug auf einer beliebigen Ebene verweigert wird, kann keine andere Ebene es zulassen. Zum Beispiel kann eine verwaltete Einstellungs-Deny nicht durch `--allowedTools` überschrieben werden, und `--disallowedTools` kann Einschränkungen über das hinaus hinzufügen, was verwaltete Einstellungen definieren.

Das Gleiche gilt für Einstellungsbereiche: Wenn Benutzereinstellungen eine Berechtigung zulassen und Projekteinstellungen sie verweigern, blockiert die Deny-Regel sie. Das Gegenteil ist auch wahr: eine Deny-Regel auf Benutzerebene blockiert eine Allow-Regel auf Projektebene, da Deny-Regeln aus jedem Bereich vor Allow-Regeln ausgewertet werden.

Embedding-Hosts können zusätzliche verwaltete Richtlinien über die SDK-Option `managedSettings` bereitstellen, wenn [`parentSettingsBehavior`](/docs/de/settings#settings-precedence) auf `"merge"` gesetzt ist; Embedder-Werte können die Richtlinie verschärfen, aber nicht lockern.

<h2 id="project-allow-rules-and-workspace-trust">
  Projekterlaubnisregeln und Workspace-Vertrauen
</h2>

`permissions.allow`-Regeln und `permissions.additionalDirectories`-Einträge in der `.claude/settings.json` eines Projekts gewähren Funktionen, daher wendet Claude Code sie nur an, nachdem Sie den [Workspace-Vertrauensdialog](/docs/de/security#additional-safeguards) für diesen Workspace akzeptiert haben. Bis dahin liest Claude Code die Regeln, wendet sie aber nicht an. Der Vertrauensdialog listet die Erlaubnisregeln und zusätzlichen Verzeichnisse auf, die der Ordner gewähren würde, damit Sie diese vor dem Akzeptieren überprüfen können. `deny`- und `ask`-Regeln sind nicht betroffen, da sie nur einschränken.

Claude Code speichert Vertrauen pro Workspace, indiziert nach dem Git-Repository-Root oder außerhalb eines Repositorys nach dem Verzeichnis, von dem aus Sie Claude Code gestartet haben. Wenn Sie in Ihrem Home-Verzeichnis starten, wird das Vertrauen nur für die aktuelle Sitzung gespeichert und nicht auf die Festplatte geschrieben; siehe die Notiz zu [zusätzlichen Schutzmaßnahmen](/docs/de/security#additional-safeguards). Das Vertrauen in ein übergeordnetes Verzeichnis wendet die Erlaubnisregeln eines verschachtelten Projekts nicht an.

`.claude/settings.local.json` ist Ihre eigene Datei, daher gilt die Workspace-Vertrauensprüfung normalerweise nicht dafür. Wenn ein Repository die Datei hätte bereitstellen können, z. B. wenn sie in Git committed ist oder `.claude` ein Symlink ist, durchlaufen ihre Erlaubnisregeln und zusätzlichen Verzeichnisse die Vertrauensprüfung wie Projekteinstellungen.

Claude Code führt Git aus, um zu überprüfen, ob das Repository die Datei bereitgestellt hat, und führt diese Überprüfung nur in einem Ordner durch, der von einem akzeptierten Vertrauensdialog abgedeckt ist, für diesen Ordner oder für eines seiner übergeordneten Verzeichnisse. In einer interaktiven Sitzung in einem Ordner, dem Sie noch nicht vertraut haben, durchlaufen Erlaubnisregeln und zusätzliche Verzeichnisse in `.claude/settings.local.json` die Vertrauensprüfung wie Projekteinstellungen, bis Sie den Dialog akzeptieren, es sei denn, die Sitzung wird in Ihrem eigenen Konfigurationsheim ausgeführt, wie unten beschrieben. Von den zwei Ausnahmen unten gilt nur die Konfigurationsheim-Ausnahme vor dem Dialog, da sie Git nicht ausführen muss. Die Feststellung, dass sich ein Verzeichnis nicht in einem Git-Repository befindet, verwendet die gleiche Git-Überprüfung, daher tritt die Nicht-in-Repository-Ausnahme in Kraft, sobald ein Vertrauensdialog, der den Ordner abdeckt, akzeptiert wird. Vor v2.1.207 wendete eine nicht nachverfollte `.claude/settings.local.json` ihre Erlaubnisregeln in diesem Ordner an, bevor Sie den Dialog akzeptierten.

Erlaubnisregeln und zusätzliche Verzeichnisse in `.claude/settings.local.json` gelten auch ohne Workspace-Vertrauen in zwei Fällen:

* Das Verzeichnis, von dem aus Sie Claude Code gestartet haben, befindet sich nicht in einem Git-Repository.
* Die Sitzung wird in Ihrem eigenen Konfigurationsheim ausgeführt: Ihr Home-Verzeichnis oder ein beliebiges Verzeichnis, dessen `.claude`-Unterverzeichnis Sie als [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) festgelegt haben.

In beiden Fällen ist die Datei eine, die Sie erstellt haben, anstatt eine, die ein Repository hätte bereitstellen können, und ein in das Repository committetes `.claude/settings.local.json` erfordert immer noch Workspace-Vertrauen. Die Versionen 2.1.196 bis 2.1.199 behandelten die Datei in diesen Workspaces als vom Repository bereitgestellt, ignorierten ihre Erlaubnisregeln und gaben eine [`this workspace has not been trusted`](/docs/de/errors#workspace-has-not-been-trusted)-Warnung auf stderr aus. Die beiden oben genannten Ausnahmen entsprechen v2.1.195 und früher und wurden in v2.1.200 wiederhergestellt.

Auch ab v2.1.200 zeigt ein Workspace, dessen Erlaubnisregeln oder zusätzliche Verzeichnisse immer noch nicht angewendet werden, aber nie den Vertrauensdialog angezeigt hat, weil ein übergeordnetes Verzeichnis bereits vertraut war, den Dialog beim nächsten interaktiven Start von Claude Code dort an. Der Dialog bietet zwei Optionen:

* **Ja, ich vertraue diesem Ordner**: speichert Vertrauen für diesen Workspace und wendet die Regeln in derselben Sitzung an.
* **Nein, ohne diese Berechtigungen fortfahren**: funktioniert weiterhin mit diesen Regeln ignoriert. Der Dialog wird in der nächsten Sitzung erneut angezeigt.

Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` wird kein Dialog angezeigt und die Regeln bleiben ignoriert.

<h2 id="example-configurations">
  Beispielkonfigurationen
</h2>

Dieses [Repository](https://github.com/anthropics/claude-code/tree/main/examples/settings) enthält Starter-Einstellungskonfigurationen für häufige Bereitstellungsszenarien. Verwenden Sie diese als Ausgangspunkte und passen Sie sie an Ihre Anforderungen an.

<h2 id="see-also">
  Siehe auch
</h2>

* [Einstellungen](/docs/de/settings): vollständige Konfigurationsreferenz einschließlich der Berechtigungseinstellungstabelle
* [Konfigurieren Sie den Auto-Mode](/docs/de/auto-mode-config): Teilen Sie dem Auto-Mode-Klassifizierer mit, welche Infrastruktur Ihre Organisation vertraut
* [Sandboxing](/docs/de/sandboxing): OS-Ebenen-Dateisystem- und Netzwerkisolation für Bash-Befehle
* [Authentifizierung](/docs/de/authentication): Richten Sie Benutzerzugriff auf Claude Code ein
* [Sicherheit](/docs/de/security): Sicherheitsvorkehrungen und Best Practices
* [Hooks](/docs/de/hooks-guide): Automatisieren Sie Workflows und erweitern Sie die Berechtigungsevaluierung
