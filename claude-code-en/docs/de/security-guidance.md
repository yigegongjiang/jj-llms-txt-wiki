> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sicherheitsprobleme erfassen, während Claude Code schreibt

> Installieren Sie das security-guidance-Plugin, damit Claude seine eigenen Code-Änderungen auf Sicherheitslücken überprüft und diese in derselben Sitzung behebt.

Das Security-Guidance-Plugin veranlasst Claude, seine eigenen Code-Änderungen auf häufige Sicherheitslücken zu überprüfen, während es arbeitet, und behebt die gefundenen Probleme in derselben Sitzung. Das Plugin erfasst Probleme wie Injection, unsichere Deserialisierung und unsichere DOM-APIs, bevor der Code einen Pull Request erreicht, und reduziert damit die Sicherheitsüberprüfung, die auf menschliche Reviewer nachgelagert fällt.

Nach der Installation wird das Plugin automatisch ausgeführt. Es gibt nichts zu aufzurufen und keinen separaten Befehl zu merken.

Das Plugin ist der In-Session-Begleiter zu [Code Review](/docs/de/code-review), das auf Pull Requests ausgeführt wird. Dieses Plugin reduziert, was den PR erreicht. Code Review erfasst, was es tut. Wie das Plugin mit On-Demand-Review und CI-Scanning zusammenarbeitet, oder um Code zu scannen, den Sie bereits haben, anstatt Änderungen, die Claude schreibt, finden Sie unter [Wie dies mit anderen Sicherheitstools zusammenpasst](#how-this-fits-with-other-security-tools).

<h2 id="prerequisites">
  Voraussetzungen
</h2>

* Python 3.7 oder später auf Ihrem `PATH`. Die agentengestützte Commit-Überprüfung benötigt Python 3.10 oder später, ebenso wie alle modellgestützten Überprüfungen, wenn Claude Code einen Drittanbieter wie Amazon Bedrock oder Google Cloud's Agent Platform nutzt. Das Plugin bevorzugt die versionierten Interpreter `python3.13` bis `python3.10`, fällt dann auf `python3`, `python` und `py -3` zurück
* Ein Git-Repository für das Verzeichnis, in dem Sie arbeiten. Die End-of-Turn- und Commit-Überprüfungen führen einen Diff gegen den Git-Status durch und werden außerhalb eines Repositories stillschweigend übersprungen. Die Per-Edit-Pattern-Überprüfung funktioniert überall

Beim ersten Ausführen erstellt das Plugin eine virtuelle Umgebung unter `~/.claude/security/` und installiert das Claude Agent SDK darin, was `pip` und Netzwerkzugriff erfordert. Wenn diese Installation fehlschlägt oder das verfügbare Python älter als 3.10 ist, wird die Commit-Überprüfung bei Authentifizierung durch Erstanbieter auf eine einmalige Überprüfung statt auf die agentengestützte zurückgestuft; bei einem Drittanbieter wie Amazon Bedrock oder Google Cloud's Agent Platform benötigen die modellgestützten Überprüfungen das SDK selbst, daher werden sie übersprungen. Das Plugin zeigt eine einmalige Benachrichtigung an, wenn eine ältere Python-Version die Ursache ist.

<h2 id="install-the-plugin">
  Plugin installieren
</h2>

Installieren Sie in einer Claude Code-Sitzung aus dem [offiziellen Anthropic-Marketplace](/docs/de/plugins/anthropic-marketplaces):

```text theme={null}
/plugin install security-guidance@claude-plugins-official
```

`/plugin` öffnet ein interaktives Panel in der Terminal-CLI. Wenn Claude meldet, dass `/plugin` in dieser Umgebung nicht verfügbar ist, installieren Sie auf andere Weise:

* **Claude-Desktop-App, lokale oder SSH-Sitzung**: öffnen Sie den [Plugin-Browser](/docs/de/desktop#install-plugins), indem Sie auf die Schaltfläche **+** neben der Eingabeaufforderung klicken, dann auf **Plugins** und dann auf **Plugin hinzufügen**
* **VS Code-Erweiterung**: installieren Sie aus dem [Dialog **Plugins verwalten**](/docs/de/vs-code#manage-plugins)
* **Cloud-Sitzungen**: eine Cloud-Sitzung lädt keine Plugins aus Ihren Benutzereinstellungen oder aus der Datei `.claude/settings.json` des Repositorys, wie unter [Was wird aus Ihrem Setup übernommen](/docs/de/cloud-environments#what-carries-over-from-your-setup) erläutert. Für Plugins, die Ihre Organisation über verwaltete Einstellungen verteilt, siehe [Plugins für Ihre Organisation verwalten](/docs/de/plugins/org)

Die Terminal-Installation fordert Sie auf, einen Bereich auszuwählen. Wählen Sie Benutzerbereich, um das Plugin in Ihre Benutzereinstellungen zu schreiben, sodass es in jeder neuen lokalen Sitzung geladen wird, die Sie auf diesem Computer starten.

Wenn die Installation fehlschlägt, stimmen Sie die Meldung ab, die Claude Code meldet:

* `Marketplace "claude-plugins-official" not found`: fügen Sie den Marketplace mit `/plugin marketplace add anthropics/claude-plugins-official` hinzu und versuchen Sie dann erneut, das Plugin zu installieren.
* Das Plugin wird [nicht im Marketplace gefunden](/docs/de/plugins/install#install-a-plugin): überprüfen Sie den Plugin-Namen.

Überprüfen Sie die Installationszusammenfassung. Wenn sie `Run /reload-plugins to activate.` meldet, siehe [Plugin-Änderungen ohne Neustart anwenden](/docs/de/plugins/cli-reference#reload-plugins), um das Plugin in Ihrer aktuellen Sitzung zu aktivieren.

<h3 id="enable-for-your-team-in-local-sessions">
  Plugin für Ihr Team in lokalen Sitzungen aktivieren
</h3>

Um das Plugin in den lokalen Sitzungen einzuschalten, die Ihre Teamkollegen im Repository starten, deklarieren Sie es in den eingecheckten Einstellungen des Projekts:

```json .claude/settings.json theme={null}
{
  "enabledPlugins": {
    "security-guidance@claude-plugins-official": true
  }
}
```

Administratoren können das Plugin organisationsweit aktivieren, indem sie [`enabledPlugins`](/docs/de/settings-reference#enabledplugins) in [verwalteten Einstellungen](/docs/de/admin-setup) festlegen.

<h2 id="what-the-plugin-checks">
  Was das Plugin überprüft
</h2>

Das Plugin überprüft Claudes Arbeit an drei Punkten, jeweils mit unterschiedlicher Tiefe:

* [Bei jeder Dateibearbeitung](#on-each-file-edit): ein schneller Pattern-Abgleich für riskante Aufrufe, ohne Modellaufruf
* [Am Ende jeder Runde](#at-the-end-of-each-turn): eine Hintergrund-Modellüberprüfung aller Änderungen dieser Runde
* [Bei jedem Commit oder Push, den Claude macht](#on-each-commit-or-push-claude-makes): eine tiefere agentengestützte Überprüfung, die umgebenden Code liest

Sie können jede Ebene erweitern, indem Sie [Ihre eigenen Regeln hinzufügen](#add-your-own-rules). Integrierte Überprüfungen können nicht einzeln entfernt werden, aber Sie können [jede Ebene unabhängig deaktivieren](#disable-or-uninstall).

<h3 id="on-each-file-edit">
  Bei jeder Dateibearbeitung
</h3>

Wenn Claude in eine Datei schreibt, scannt das Plugin den neuen Inhalt auf bekannte riskante Muster. Dies ist ein Pattern-Abgleich ohne Modellaufruf, daher entstehen keine Nutzungskosten.

Beispiel-Pattern-Kategorien:

* Dynamische Code-Ausführung: `eval(`, `new Function`, `os.system`, `child_process.exec`
* Unsichere Deserialisierung: `pickle`
* DOM-Injection: `dangerouslySetInnerHTML`, `.innerHTML =`, `document.write`
* Workflow-Dateien: Bearbeitungen unter `.github/workflows/`, die Repository-Berechtigungen gewähren können

Die Überprüfung wird nach dem Landen der Bearbeitung ausgeführt und fügt die Warnung an Claudes Kontext für den nächsten Schritt an. Jede Warnung wird einmal pro Pattern pro Datei pro Sitzung ausgelöst, sodass wiederholte Übereinstimmungen in derselben Datei das Gespräch nicht überfluten.

Sie können [Ihre eigenen Muster hinzufügen](#add-custom-per-edit-patterns) zu dieser Ebene mit einer `security-patterns.yaml`-Datei.

<h3 id="at-the-end-of-each-turn">
  Am Ende jeder Runde
</h3>

Eine Runde ist eine Runde von Claudes Antwort: Sie senden eine Nachricht, Claude arbeitet und antwortet, und die Runde endet. Nach jeder Runde berechnet das Plugin einen Git-Diff aller Änderungen, die während der Runde im Arbeitsverzeichnis vorgenommen wurden, einschließlich Änderungen von Claudes Edit-Tools, Bash-Befehlen und Subagenten, und sendet ihn an eine separate Claude-Überprüfung, die sich auf Sicherheit konzentriert. Die Überprüfung wird im Hintergrund ausgeführt, sodass Claudes Antwort nicht verzögert wird. Wenn die Überprüfung Probleme findet, wird Claude mit den Ergebnissen erneut aufgefordert und behebt sie als Folgemaßnahme.

Dies erfasst Probleme, die ein String-Abgleich nicht kann, wie zum Beispiel:

* Autorisierungsumgehung
* Unsichere direkte Objektreferenzen
* Injection
* Server-seitige Request-Forgery
* Schwache Kryptographie

Sie sehen sowohl die Erkenntnis als auch Claudes Lösung direkt in Ihrer Sitzung. Die Überprüfung umfasst bis zu 30 geänderte Dateien pro Runde und wird höchstens dreimal hintereinander ausgeführt, bevor sie an Sie zurückgegeben wird.

<h3 id="on-each-commit-or-push-claude-makes">
  Bei jedem Commit oder Push, den Claude macht
</h3>

Wenn Claude `git commit` oder `git push` über sein Bash-Tool ausführt, führt das Plugin eine tiefere agentengestützte Überprüfung der Änderung im Hintergrund aus. Diese Überprüfung liest umgebenden Code, einschließlich Aufrufer, Sanitizer und verwandter Dateien, um zu entscheiden, ob eine Erkenntnis real ist, bevor sie gemeldet wird. Der zusätzliche Kontext hält falsch positive Ergebnisse bei Mustern niedrig, die isoliert gefährlich aussehen, aber in Ihrer Codebasis sicher sind.

Diese Ebene wird nur bei Commits und Pushes ausgelöst, die Claude über sein Bash-Tool macht. Commits, die Sie von Ihrer eigenen Shell aus ausführen, einschließlich des `!`-Shell-Escape in einer Sitzung, werden nicht überprüft. Commit- und Push-Überprüfungen sind auf 20 pro rollende Stunde begrenzt. Wenn die Commit-Überprüfung Erkenntnisse findet, die denen der End-of-Turn-Überprüfung duplizieren, wird Claude nicht erneut aufgefordert, sodass ein sauberer Commit keine sichtbare Ausgabe aus dieser Ebene erzeugt.

<h3 id="review-independence-and-limits">
  Überprüfungsunabhängigkeit und Limits
</h3>

Das Plugin fragt nicht die gleiche Claude-Instanz, die den Code geschrieben hat, um sich selbst zu bewerten. Die Per-Edit-Überprüfung ist ein deterministischer String-Abgleich ohne Modellbeteiligung. Die End-of-Turn- und Commit-Überprüfungen werden als separater Claude-Aufruf mit frischem Kontext und sicherheitsorientiertem Prompt ausgeführt: Der Reviewer beginnt mit dem Diff, hat keine Investition in den ursprünglichen Ansatz und wird nur angewiesen, Probleme zu finden.

Keine der Ebenen blockiert Schreibvorgänge oder Commits. Erkenntnisse erreichen Claude als Anweisungen, Claude behebt sie im Gespräch, und das Review-Modell kann Probleme übersehen. Behandeln Sie das Plugin als eine Ebene der Verteidigungstiefe, nicht als vollständige Sicherheitslösung. Siehe [Wie dies mit anderen Sicherheitstools zusammenpasst](#how-this-fits-with-other-security-tools).

<h2 id="add-your-own-rules">
  Fügen Sie Ihre eigenen Regeln hinzu
</h2>

Das Plugin hat zwei Erweiterungspunkte: eine Markdown-Guidance-Datei für die modellgestützten Überprüfungen und eine YAML- oder JSON-Patterns-Datei für den Per-Edit-String-Abgleich. Beide sind additiv. Sie können Überprüfungen hinzufügen, aber keine integrierten aus diesen Dateien deaktivieren.

<h3 id="add-guidance-for-the-model-backed-reviews">
  Fügen Sie Guidance für die modellgestützten Überprüfungen hinzu
</h3>

Erstellen Sie `.claude/claude-security-guidance.md` in Ihrem Projekt und beschreiben Sie Ihr Bedrohungsmodell und Ihre Überprüfungscheckliste in einfacher Sprache. Die modellgestützten Überprüfungen laden sie als zusätzlichen Kontext neben der integrierten Sicherheitslückencheckliste.

Das folgende Beispiel ist für einen Webdienst mit rollengesteuerten Admin-Routen und einer Kundendaten-Logging-Richtlinie:

```markdown .claude/claude-security-guidance.md theme={null}
# Sicherheitsguidance für dieses Repository

- Protokollieren Sie `customer_id` oder `account_number` nicht auf INFO-Ebene oder höher.
- Alle Routen unter `/admin` müssen `require_role("admin")` aufrufen, bevor ein Datenbanklesevorgang erfolgt.
- Verwenden Sie `crypto.timingSafeEqual` für Token-Vergleich statt `===`.
```

Diese Regeln sind Guidance für den Reviewer, keine deterministischen Schutzvorrichtungen. Das Plugin zeigt Verstöße als Erkenntnisse für Claude an, um sie zu beheben, blockiert aber keine Schreibvorgänge und garantiert nicht, dass jeder Verstoß erfasst wird. Die Guidance ist nur additiv: Eine Regel, die besagt, dass eine Sicherheitslückenklasse ignoriert werden soll, unterdrückt diese Erkenntnisse nicht. Für harte Durchsetzung kombinieren Sie das Plugin mit einem [Hook, der Bearbeitungen blockiert](/docs/de/hooks-guide#block-edits-to-protected-files) oder einer CI-Überprüfung.

<h3 id="add-custom-per-edit-patterns">
  Fügen Sie benutzerdefinierte Per-Edit-Muster hinzu
</h3>

Erstellen Sie `.claude/security-patterns.yaml`, um Regex- oder Substring-Regeln zur [Per-Edit-Pattern-Überprüfung](#on-each-file-edit) hinzuzufügen. Diese werden als deterministische String-Abgleiche neben den integrierten Mustern ausgeführt:

```yaml .claude/security-patterns.yaml theme={null}
patterns:
  - rule_name: internal_api_key
    substrings: ["sk_live_", "AKIA"]
    reminder: "Hardcodierter API-Schlüsselpräfix. Laden Sie Anmeldedaten aus dem Secret Manager."
  - rule_name: tenant_unfiltered_query
    regex: "\\.objects\\.all\\(\\)"
    paths: ["**/src/tenants/**"]
    reminder: "Multi-Tenant-Code muss nach org_id filtern."
```

| Feld            | Typ    | Beschreibung                                                                                                                                                                                      |
| :-------------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `rule_name`     | string | Identifier, der in der Warnung angezeigt wird                                                                                                                                                     |
| `reminder`      | string | Warntext, der an Claudes Kontext angehängt wird, begrenzt auf 1 KB                                                                                                                                |
| `regex`         | string | Python-Regex, die gegen den bearbeiteten Inhalt abgeglichen wird                                                                                                                                  |
| `substrings`    | list   | Literale Substrings; geben Sie dies oder `regex` an                                                                                                                                               |
| `paths`         | list   | Optionale Glob-Muster; die Regel gilt nur für übereinstimmende Dateien. Globs werden gegen den vollständigen Dateipfad abgeglichen, daher müssen Sie projektrelative Muster mit `**/` präfixieren |
| `exclude_paths` | list   | Optionale Glob-Muster zum Überspringen; gleiches Matching wie `paths`                                                                                                                             |

Das Plugin liest auch `.claude/security-patterns.yml` und `.claude/security-patterns.json` mit dem gleichen Schema. JSON funktioniert auf jeder Python-Installation. Die YAML-Formen erfordern, dass PyYAML importierbar ist, was das Plugin nicht für Sie installiert. Das Plugin lädt bis zu 50 benutzerdefinierte Regeln und überspringt Regexes, die anfällig für katastrophales Backtracking aussehen.

<h3 id="rule-file-lookup-locations">
  Lookup-Speicherorte für Regeldateien
</h3>

Das Plugin sucht nach `claude-security-guidance.md` und `security-patterns.yaml` an den gleichen Speicherorten, unabhängig davon, wie das Plugin aktiviert wurde:

| Bereich       | Pfad                                        | Notizen                                                             |
| :------------ | :------------------------------------------ | :------------------------------------------------------------------ |
| Benutzer      | `~/.claude/claude-security-guidance.md`     | Gilt für jedes Projekt auf Ihrem Computer                           |
| Projekt       | `.claude/claude-security-guidance.md`       | Mit dem Repository eingecheckt                                      |
| Projekt lokal | `.claude/claude-security-guidance.local.md` | Für persönliche Overrides; fügen Sie es zu Ihrer `.gitignore` hinzu |

Das Plugin lädt alle Speicherorte, die vorhanden sind, und verkettet sie mit einer kombinierten Obergrenze von 8 KB für die Guidance-Datei. Administratoren können organisationsweit Regeln verteilen, indem sie die Benutzerbereichsdatei über Geräteverwaltung zu `~/.claude/` pushen. Die gleichen Pfade gelten für `security-patterns.yaml`.

<h2 id="usage-cost">
  Nutzungskosten
</h2>

Die [Per-Edit-Pattern-Überprüfung](#on-each-file-edit) macht keinen Modellaufruf und fügt keine Kosten hinzu. Die [End-of-Turn](#at-the-end-of-each-turn)- und [Commit](#on-each-commit-or-push-claude-makes)-Überprüfungen verbrauchen jeweils zusätzliche Modellnutzung, die wie jede andere Claude-Anfrage zu Ihrer [Nutzung](/docs/de/costs) zählt. Die Commit-Überprüfung ist agentengestützt und kann mehrere Modellrunden pro Commit dauern. Erwarten Sie ungefähr einen Review-Aufruf pro Runde, die Dateien ändert, und einen tieferen Review pro Commit, beide unterliegen den oben genannten Limits.

Beide modellgestützten Überprüfungen verwenden standardmäßig Claude Opus 4.7. Setzen Sie `SECURITY_REVIEW_MODEL`, um ein anderes Modell für die End-of-Turn-Überprüfung zu wählen, und `SG_AGENTIC_MODEL` für die Commit-Überprüfung.

Das Plugin ist auf allen Plänen verfügbar.

<h2 id="disable-or-uninstall">
  Deaktivieren oder deinstallieren
</h2>

Um einzelne Ebenen zu deaktivieren und den Rest zu behalten, setzen Sie die entsprechende Umgebungsvariable:

| Variable                        | Effekt                                                                                    |
| :------------------------------ | :---------------------------------------------------------------------------------------- |
| `ENABLE_PATTERN_RULES=0`        | Deaktivieren Sie die [Per-Edit-Pattern-Überprüfung](#on-each-file-edit)                   |
| `ENABLE_STOP_REVIEW=0`          | Deaktivieren Sie die [End-of-Turn-Diff-Überprüfung](#at-the-end-of-each-turn)             |
| `ENABLE_COMMIT_REVIEW=0`        | Deaktivieren Sie die [Commit- und Push-Überprüfung](#on-each-commit-or-push-claude-makes) |
| `ENABLE_CODE_SECURITY_REVIEW=0` | Deaktivieren Sie alle modellgestützten Überprüfungen auf einmal                           |
| `SECURITY_GUIDANCE_DISABLE=1`   | Deaktivieren Sie das Plugin vollständig, ohne es zu deinstallieren                        |

Um das Plugin in Ihrem Benutzerbereich zu pausieren:

```text theme={null}
/plugin disable security-guidance@claude-plugins-official
```

Um es aus Ihrem Benutzerbereich zu entfernen:

```text theme={null}
/plugin uninstall security-guidance@claude-plugins-official
```

Wenn das Plugin durch die `.claude/settings.json` eines Projekts aktiviert wurde, schreibt das Deaktivieren von `/plugin` einen Override in Ihre `.claude/settings.local.json`, anstatt die eingecheckte Datei zu bearbeiten, sodass das Plugin für Sie ausgeschaltet bleibt, während Ihre Teamkollegen nicht betroffen sind. Derselbe Dialog bietet auch an, das Plugin für alle zu deinstallieren, indem es aus der gemeinsamen `.claude/settings.json` entfernt wird. Wenn es durch [verwaltete Einstellungen](/docs/de/admin-setup) aktiviert wurde, kann nur ein Administrator es deaktivieren.

<h2 id="how-the-plugin-integrates-with-claude-code">
  Wie das Plugin mit Claude Code integriert wird
</h2>

Das Plugin ist vollständig auf [Hooks](/docs/de/hooks) aufgebaut, dem Mechanismus zum Ausführen Ihres eigenen Codes an bestimmten Punkten in Claudes Schleife. Es registriert:

| Hook-Ereignis                                                       | Zweck                                                                                            |
| :------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------- |
| `SessionStart`                                                      | Bootstrap der Python-Umgebung des Plugins                                                        |
| `UserPromptSubmit`                                                  | Erfassen Sie die Baseline des Arbeitsverzeichnisses, gegen die die End-of-Turn-Überprüfung diffs |
| `PostToolUse` auf `Edit`, `Write` und `NotebookEdit`                | Per-Edit-Pattern-Abgleich                                                                        |
| `Stop`                                                              | End-of-Turn-Diff-Überprüfung, im Hintergrund ausgeführt                                          |
| `PostToolUse` auf `Bash`, gefiltert auf `git commit` und `git push` | Commit- und Push-Überprüfung, im Hintergrund ausgeführt                                          |

Wenn Sie Ihre eigenen Hooks erstellen, ist der [Quellcode des Plugins](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/security-guidance) ein funktionierendes Beispiel für das Ausführen eines separaten Modellaufrufs aus einem Hook und das Zurückführen des Ergebnisses in die Sitzung.

<h2 id="how-this-fits-with-other-security-tools">
  Wie dies mit anderen Sicherheitstools zusammenpasst
</h2>

Das Plugin ist eine Ebene in einem Verteidigungstiefe-Ansatz. Es erfasst Probleme am frühesten, während der Code noch im Editor ist, aber es ist keine Garantie und ersetzt keine späteren Überprüfungen. Ein typischer Stack:

| Phase                          | Tool                                                          | Was es abdeckt                                                                                                   |
| :----------------------------- | :------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------- |
| In Sitzung                     | Security-Guidance-Plugin                                      | Häufige Sicherheitslücken in Code, den Claude schreibt, behoben in derselben Sitzung                             |
| On Demand, einzelner Durchlauf | [`/security-review`](/docs/de/commands#all-commands)               | Einmalige Sicherheitsüberprüfung auf dem aktuellen Branch, ausgeführt, wenn Sie es anfordern                     |
| On Demand, tiefe Überprüfung   | [Claude Security Plugin](/docs/de/claude-security)                 | Multi-Agent-Schwachstellenscan eines Repositorys oder Diffs mit unabhängig überprüften Erkenntnissen und Patches |
| Bei Pull Request               | [Code Review](/docs/de/code-review), Team- und Enterprise-Pläne    | Multi-Agent-Korrektheit und Sicherheitsüberprüfung mit vollständigem Codebase-Kontext                            |
| In CI                          | Ihre vorhandenen statischen Analysen und Abhängigkeitsscanner | Sprachspezifische Regeln, Supply-Chain-Überprüfungen und Richtliniendurchsetzung, die das Plugin nicht versucht  |

Um Sicherheitsprobleme in Code zu finden, den Sie bereits haben, anstatt in Änderungen, die Claude schreibt, bitten Sie Claude in einer Sitzung, eine bestimmte Datei oder ein Verzeichnis auf Sicherheitslücken zu überprüfen, oder verwenden Sie das [Claude Security Plugin](/docs/de/claude-security) für einen tieferen Multi-Agent-Scan des gesamten Repositorys; [`/security-review`](/docs/de/commands#all-commands) deckt nur die Änderungen auf Ihrem aktuellen Branch ab. In jedem Fall liest die Überprüfung den Quellcode in Ihrem Checkout, nicht eine laufende Website oder einen bereitgestellten Service.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Das Plugin schreibt Laufzeit-Diagnosen in `~/.claude/security/log.txt`. Überprüfen Sie dort zuerst, wenn Überprüfungen nicht angezeigt werden.

Häufige Gründe, warum eine Überprüfungsebene ohne Nachricht im Gespräch übersprungen wird:

* Das Verzeichnis ist kein Git-Repository: Die End-of-Turn- und Commit-Überprüfungen erfordern Git-Status und werden außerhalb eines Repositories übersprungen
* Die Sitzung hat keine Anthropic-Authentifizierung und keinen konfigurierten Drittanbieter: Die modellgestützten Überprüfungen werden übersprungen und nur die Per-Edit-Pattern-Überprüfung wird ausgeführt
* Eine `security-patterns.yaml`-Datei ist vorhanden, aber PyYAML ist nicht importierbar: Die Datei wird ignoriert. Verwenden Sie stattdessen `security-patterns.json`

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Um tiefer in die Themen einzusteigen, die diese Seite berührt:

* [Code Review](/docs/de/code-review): Richten Sie die Multi-Agent-Überprüfung zur PR-Zeit ein
* [Automatisieren Sie Workflows mit Hooks](/docs/de/hooks-guide): Erstellen Sie Ihre eigenen Überprüfungen an den gleichen Lebenszykluspunkten
* [Finden Sie Plugins im offiziellen Marketplace](/docs/de/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): Durchsuchen Sie andere offizielle Plugins
