> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfiguration debuggen

> Diagnostizieren Sie, warum CLAUDE.md, Einstellungen, Hooks, MCP-Server oder Skills nicht wirksam werden. Verwenden Sie /context, /doctor, /hooks und /mcp, um zu sehen, was tatsächlich geladen wurde.

Wenn Claude eine Anweisung ignoriert oder eine konfigurierte Funktion nicht angezeigt wird, liegt die Ursache normalerweise darin, dass die Datei nicht geladen wurde, sie von einem anderen Ort als erwartet geladen wurde oder eine andere Datei sie überschrieben hat. Diese Anleitung zeigt, wie Sie überprüfen, was Claude Code tatsächlich geladen hat, damit Sie eingrenzen können, welcher Fall zutrifft.

Für Installations-, Authentifizierungs- und Konnektivitätsprobleme siehe stattdessen [Troubleshooting bei Installation und Anmeldung](/docs/de/troubleshoot-install).

<h2 id="see-what-loaded-into-context">
  Sehen Sie, was in den Kontext geladen wurde
</h2>

Der Befehl `/context` zeigt alles, was das Kontextfenster für die aktuelle Sitzung belegt, aufgeschlüsselt nach Kategorie: Systemaufforderung, Speicherdateien, Skills, benutzerdefinierte Subagenten mit der Quelle, aus der jeder geladen wurde, MCP-Tools und Konversationsnachrichten. Führen Sie ihn zuerst aus, um zu bestätigen, ob Ihre `CLAUDE.md`, Regeln oder Skill-Beschreibungen überhaupt vorhanden sind.

Für Details zu einer bestimmten Kategorie folgen Sie mit dem dedizierten Befehl:

| Befehl           | Zeigt                                                                                                                                                                                                                                                                                     |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/memory`        | Welche `CLAUDE.md`- und Regeldateien geladen wurden, plus Auto-Memory-Einträge                                                                                                                                                                                                            |
| `/skills`        | Verfügbare Skills aus Projekt-, Benutzer- und Plugin-Quellen                                                                                                                                                                                                                              |
| `/hooks`         | Aktive Hook-Konfigurationen                                                                                                                                                                                                                                                               |
| `/mcp`           | Verbundene MCP-Server und ihr Status                                                                                                                                                                                                                                                      |
| `/permissions`   | Aufgelöste Allow- und Deny-Regeln, die derzeit wirksam sind                                                                                                                                                                                                                               |
| `/doctor`        | Konfigurationsdiagnose: Installationsintegrität, ungültige Einstellungsdateien, ungenutzte Erweiterungen, doppelte [Subagenten](/docs/de/sub-agents)-Namen im gleichen Verzeichnis und eingecheckte `CLAUDE.md`-Inhalte, die Claude aus der Codebasis ableiten kann, mit vorgeschlagenen Fixes |
| `/debug [issue]` | Aktiviert Debug-Protokollierung für die Sitzung und fordert Claude auf, die Diagnose mithilfe der Protokollausgabe und Einstellungspfade durchzuführen                                                                                                                                    |
| `/status`        | Aktive Einstellungsquellen, einschließlich ob verwaltete Einstellungen wirksam sind                                                                                                                                                                                                       |

Wenn eine Speicherdatei in `/memory` fehlt, überprüfen Sie ihren Speicherort anhand von [wie CLAUDE.md-Dateien geladen werden](/docs/de/memory#how-claude-md-files-load). Unterverzeichnis-`CLAUDE.md`-Dateien werden bei Bedarf geladen, wenn Claude eine Datei in diesem Verzeichnis mit dem Read-Tool liest, nicht beim Sitzungsstart.

Wenn `/memory` bestätigt, dass die Datei geladen wurde, Claude aber immer noch einer bestimmten Anweisung nicht folgt, liegt das Problem wahrscheinlich darin, wie die Anweisung geschrieben ist, nicht darin, ob sie geladen wurde. CLAUDE.md funktioniert gut für die Art von Anleitung, die Sie einem neuen Teamkollegen geben würden, wie z. B. Projektkonventionen, Build-Befehle und wo Dateien hingehören.

Die Einhaltung sinkt, wenn eine Anweisung mehrdeutig genug ist, um mehrere Interpretationen zuzulassen, wenn zwei Dateien widersprüchliche Anweisungen geben, oder wenn die Datei so lang geworden ist, dass einzelne Regeln weniger Aufmerksamkeit erhalten. [Schreiben Sie effektive Anweisungen](/docs/de/memory#write-effective-instructions) behandelt die Spezifität, Größe und Strukturmuster, die die Einhaltung hoch halten.

<Note>
  CLAUDE.md und Berechtigungen lösen unterschiedliche Probleme. CLAUDE.md teilt Claude mit, wie Ihr Projekt funktioniert, damit es gute Entscheidungen trifft. [Berechtigungen](/docs/de/permissions) und [Hooks](/docs/de/hooks) erzwingen Grenzen unabhängig davon, was Claude entscheidet. Verwenden Sie CLAUDE.md für „wir machen es hier so". Verwenden Sie Berechtigungen oder Hooks für Sicherheitsgrenzen und alles, das niemals passieren darf, wenn Sie eine Garantie statt einer Anleitung benötigen.
</Note>

<h2 id="check-resolved-settings">
  Überprüfen Sie aufgelöste Einstellungen
</h2>

Einstellungen werden über verwaltete, Benutzer-, Projekt- und lokale Bereiche zusammengeführt. Verwaltete Einstellungen gewinnen immer, wenn sie vorhanden sind. Bei den übrigen überschreibt der nähere Bereich den breiteren in der Reihenfolge lokal, dann Projekt, dann Benutzer. Einige Einstellungen können auch durch Befehlszeilenflaggen oder [Umgebungsvariablen](/docs/de/env-vars) festgelegt werden, die als eine weitere Überschreibungsebene fungieren. Wenn eine Einstellung nicht angewendet zu werden scheint, wird der von Ihnen festgelegte Wert normalerweise durch einen anderen Bereich oder eine Umgebungsvariable überschrieben.

Führen Sie `/doctor` aus, um Ihre Konfiguration und Installation zu überprüfen. Es meldet, was es findet, einschließlich ungültiger Einstellungsdateien, doppelter Installationen, ungenutzter Erweiterungen und eingecheckter `CLAUDE.md`-Inhalte, die Claude aus der Codebasis ableiten kann, und schlägt dann Fixes vor, die es nur nach Ihrer Bestätigung anwendet. Die `CLAUDE.md`-Kürzungsprüfung erfordert Claude Code v2.1.206 oder später. Vor v2.1.205 öffnete `/doctor` einen schreibgeschützten Diagnosebildschirm und das Drücken von `f` sendete den Bericht an Claude zur Behebung.

Aus dem Terminal druckt `claude doctor` schreibgeschützte Installations- und Einstellungsdiagnosen, ohne eine Sitzung zu starten.

Führen Sie `/status` aus, um zu sehen, welche Einstellungsquellen aktiv sind, einschließlich ob verwaltete Einstellungen wirksam sind. Um zu verstehen, welcher Bereich für einen bestimmten Schlüssel gewinnt, siehe [Wie Bereiche interagieren](/docs/de/settings#how-scopes-interact).

<h2 id="check-mcp-servers">
  Überprüfen Sie MCP-Server
</h2>

Führen Sie `/mcp` aus, um jeden konfigurierten Server, seinen Verbindungsstatus und ob Sie ihn für das aktuelle Projekt genehmigt haben, zu sehen. Ein Server kann korrekt definiert sein, aber aus einigen häufigen Gründen immer noch keine Tools bereitstellen:

* Projektbezogene Server in `.mcp.json` erfordern eine einmalige Genehmigung. Wenn die Aufforderung verworfen wurde, bleibt der Server deaktiviert, bis Sie ihn von `/mcp` aus genehmigen.
* Ein Server, der nicht startet, wird in `/mcp` als fehlgeschlagen angezeigt. Relative Dateipfade in `command` oder `args` sind eine häufige Ursache, da sie gegen das Verzeichnis aufgelöst werden, von dem aus Sie Claude Code gestartet haben, nicht gegen den Speicherort von `.mcp.json`.
* Ein Server, der als verbunden angezeigt wird, aber null Tools auflistet, hat erfolgreich gestartet, gibt aber keine Toolliste zurück. Wählen Sie **Reconnect** von `/mcp`. Wenn die Anzahl bei null bleibt, führen Sie `claude --debug mcp` aus, um die Stderr-Ausgabe des Servers zu sehen.

Für Konfigurationsspeicherorte und Bereichsregeln siehe [MCP](/docs/de/mcp).

<h2 id="check-hooks">
  Überprüfen Sie Hooks
</h2>

Führen Sie `/hooks` aus, um jeden Hook aufzulisten, der für die aktuelle Sitzung registriert ist, gruppiert nach Ereignis. Wenn ein von Ihnen definierter Hook nicht angezeigt wird, wird er nicht gelesen: Hooks gehen unter den Schlüssel `"hooks"` in einer Einstellungsdatei, nicht in einer eigenständigen Datei.

Wenn der Hook angezeigt wird, aber nicht ausgelöst wird, ist der Matcher die übliche Ursache. Überprüfen Sie ihn auf diese Fehler:

* Das Feld `matcher` ist eine einzelne Zeichenkette, die `|` verwendet, um mehrere Tool-Namen zu entsprechen, z. B. `"Edit|Write"`. Ein `,`-Trennzeichen ist gleichwertig, sodass `"Edit,Write"` dieselben Tools entspricht. Vor v2.1.191 wurde ein Komma zur Regex-Auswertung durchgeleitet und der Matcher stimmte nie überein, daher verwenden Sie `|`, wenn Sie nicht auf v2.1.191 sind.
* Ein falsch geschriebener Tool-Name erzeugt einen Matcher, der nichts entspricht, sodass der Hook stillschweigend fehlschlägt.
* Ein Array-Wert ist ein Schemafehler: Claude Code zeigt einen Einstellungsfehler an und lehnt die gesamte Benutzer-, Projekt- oder lokale Einstellungsdatei ab, `claude doctor` meldet den Validierungsfehler, und kein Hook aus dieser Datei wird in `/hooks` angezeigt. In [verwalteten Einstellungen](/docs/de/settings#settings-files) wird nur der ungültige Eintrag entfernt und die anderen Hooks der Datei gelten weiterhin.

Änderungen an `settings.json` werden in der laufenden Sitzung nach einer kurzen Dateistabilitätsverzögerung wirksam. Sie müssen nicht neu starten. Wenn `/hooks` einige Sekunden nach dem Speichern immer noch die alte Definition anzeigt, führen Sie `/hooks` erneut aus, um die Ansicht zu aktualisieren.

Wenn `/hooks` den Hook anzeigt, aber er wird immer noch nicht ausgelöst, besteht der nächste Schritt darin, die Hook-Auswertung live zu beobachten. Starten Sie eine Sitzung mit `claude --debug hooks` und lösen Sie den Tool-Aufruf aus. Das Debug-Protokoll zeichnet jedes Ereignis, welche Matcher überprüft wurden, und den Exit-Code und die Ausgabe des Hooks auf. Siehe [Debug Hooks](/docs/de/hooks#debug-hooks) für das Protokollformat und [Hooks Troubleshooting](/docs/de/hooks-guide#limitations-and-troubleshooting) für häufige Fehlermuster.

<h2 id="test-against-a-clean-configuration">
  Testen Sie gegen eine saubere Konfiguration
</h2>

Beginnen Sie mit [`claude --safe-mode`](/docs/de/cli-reference#cli-flags), das eine Sitzung mit allen deaktivierten Anpassungen startet, einschließlich `CLAUDE.md`, Skills, Plugins, Hooks, MCP-Servern und benutzerdefinierten Befehlen und Agenten. Authentifizierung, Modellauswahl, integrierte Tools und Berechtigungen funktionieren normal. Wenn das Problem im abgesicherten Modus verschwindet, ist eine dieser Oberflächen die Ursache; verwenden Sie die gezielten Überprüfungen oben, um herauszufinden, welche. Der abgesicherte Modus wendet immer noch verwaltete Hooks und die Einstellungsrichtlinie Ihrer Organisation an. Verwaltete Plugins, Skills, `CLAUDE.md` und MCP-Server sind ausgeschaltet.

Wenn das Problem im abgesicherten Modus bestehen bleibt oder Ihre Einstellungen selbst verdächtig sind, vergleichen Sie mit einer Sitzung, die nichts aus Ihrem üblichen Setup lädt. Zeigen Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) auf ein leeres Verzeichnis, um alles unter `~/.claude` zu umgehen, und starten Sie von einem Verzeichnis, das keinen `.claude`-Ordner, keine `.mcp.json` oder `CLAUDE.md` hat, damit die Projektkonfiguration auch übersprungen wird.

```bash theme={null}
cd /tmp && CLAUDE_CONFIG_DIR=/tmp/claude-clean claude
```

Die saubere Sitzung hat keine Benutzer- oder Projekteinstellungen, Hooks, MCP-Server, Plugins oder Speicher.

* Verwaltete Einstellungen gelten immer noch, wenn Ihre Organisation sie bereitstellt, da sie sich unter einem Systempfad außerhalb von `~/.claude` befinden
* Unter Linux und Windows werden Sie aufgefordert, sich erneut anzumelden, da Anmeldedaten unter dem Konfigurationsverzeichnis gespeichert sind
* Unter macOS befinden sich Anmeldedaten im Keychain und werden in die saubere Sitzung übernommen

Wenn das Problem hier verschwindet, liegt die Ursache irgendwo in Ihren echten `~/.claude`- oder Projekt-`.claude`-Dateien. Führen Sie sie einzeln wieder ein, indem Sie Dateien in das temporäre Verzeichnis kopieren oder von Ihrem Projekt aus starten, um herauszufinden, welche. Wenn es in der sauberen Sitzung bestehen bleibt, liegt die Ursache außerhalb Ihrer Benutzer- und Projektkonfiguration. Führen Sie `/status` aus, um zu überprüfen, ob verwaltete Einstellungen wirksam sind, suchen Sie nach [Umgebungsvariablen](/docs/de/env-vars), die Claude Code beeinflussen, und siehe dann [Troubleshooting](/docs/de/troubleshooting).

<h2 id="check-common-causes">
  Überprüfen Sie häufige Ursachen
</h2>

Die meisten Konfigurationsüberraschungen lassen sich auf eine kleine Anzahl von Speicherort- und Syntaxregeln zurückführen. Überprüfen Sie diese, bevor Sie einen Fehler annehmen:

| Symptom                                                                           | Ursache                                                                                                                                                     | Behebung                                                                                                                                                                                                                                                                                                           |
| :-------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Hook wird nie ausgelöst                                                           | `matcher` ist ein JSON-Array statt einer Zeichenkette                                                                                                       | Verwenden Sie eine einzelne Zeichenkette mit `\|`, um mehrere Tools zu entsprechen, z. B. `"Edit\|Write"`. Siehe [Matcher-Muster](/docs/de/hooks#matcher-patterns).                                                                                                                                                     |
| Hook wird nie ausgelöst                                                           | `matcher` verwendet `,` als Trennzeichen in einer Version vor v2.1.191                                                                                      | Claude Code v2.1.191 oder später behandelt `,` als Listentrennzeichen wie `\|`. Frühere Versionen bewerten ein Komma als Literalzeichen, daher entspricht `"Edit,Write"` nichts. Verwenden Sie stattdessen `\|`, oder aktualisieren Sie Claude Code.                                                               |
| Hook wird nie ausgelöst                                                           | `matcher`-Wert ist Kleinbuchstaben, z. B. `"bash"`                                                                                                          | Matching ist Groß-/Kleinschreibung-empfindlich. Tool-Namen werden großgeschrieben: `Bash`, `Edit`, `Write`, `Read`.                                                                                                                                                                                                |
| Hook wird nie ausgelöst                                                           | Hooks befinden sich in einer eigenständigen Datei statt in `settings.json`                                                                                  | Es gibt keine eigenständige Hooks-Datei für Projekt- oder Benutzerkonfiguration. Definieren Sie Hooks unter dem Schlüssel `"hooks"` in `settings.json`. Nur [Plugins](/docs/de/plugins-reference#hooks) laden eine separate `hooks/hooks.json`. Siehe [Hook-Konfiguration](/docs/de/hooks).                                  |
| Berechtigungen, Hooks oder global gesetzte Umgebungsvariablen werden ignoriert    | Konfiguration wurde zu `~/.claude.json` hinzugefügt                                                                                                         | `~/.claude.json` enthält App-Status und UI-Umschalter. `permissions`, `hooks` und `env` gehören zu `~/.claude/settings.json`. Dies sind zwei verschiedene Dateien.                                                                                                                                                 |
| Ein `settings.json`-Wert scheint ignoriert zu werden                              | Derselbe Schlüssel ist in `settings.local.json` gesetzt                                                                                                     | `settings.local.json` überschreibt `settings.json`, und beide überschreiben `~/.claude/settings.json`. Siehe [Einstellungspriorität](/docs/de/settings#how-scopes-interact).                                                                                                                                            |
| Skill erscheint nicht in `/skills`                                                | Skill-Datei befindet sich unter `.claude/skills/name.md` statt in einem Ordner                                                                              | Verwenden Sie einen Ordner mit `SKILL.md` darin: `.claude/skills/name/SKILL.md`.                                                                                                                                                                                                                                   |
| Skill erscheint in `/skills`, aber Claude ruft ihn nie auf                        | Skill hat `disable-model-invocation: true` in seinem Frontmatter, oder seine Beschreibung stimmt nicht damit überein, wie Sie die Anfrage formulieren       | Überprüfen Sie das Badge in `/skills`: Ein Label „user-only" bedeutet, dass Claude es nicht von selbst auslöst. Siehe [Skill-Aufruf](/docs/de/skills).                                                                                                                                                                  |
| Anweisungen im Unterverzeichnis `CLAUDE.md` scheinen ignoriert zu werden          | Unterverzeichnisdateien werden bei Bedarf geladen, nicht beim Sitzungsstart                                                                                 | Sie werden geladen, wenn Claude eine Datei in diesem Verzeichnis mit dem Read-Tool liest, nicht beim Start und nicht beim Schreiben oder Erstellen von Dateien dort. Siehe [wie CLAUDE.md-Dateien geladen werden](/docs/de/memory#how-claude-md-files-load).                                                            |
| Subagent ignoriert `CLAUDE.md`-Anweisungen                                        | Die integrierten Explore- und Plan-Agenten überspringen `CLAUDE.md`. Benutzerdefinierte Subagenten laden es auf die gleiche Weise wie die Hauptkonversation | Für Explore oder Plan wiederholen Sie die Anweisung in Ihrer delegierenden Aufforderung. Für einen benutzerdefinierten Subagenten setzen Sie kritische Anweisungen in den Agent-Dateitext, der zur Systemaufforderung des Agenten wird. Siehe [was beim Start geladen wird](/docs/de/sub-agents#what-loads-at-startup). |
| Cleanup-Logik wird am Sitzungsende nie ausgeführt                                 | Kein `SessionEnd`-Hook konfiguriert                                                                                                                         | Fügen Sie einen `SessionEnd`-Hook in `settings.json` hinzu. Siehe die [Hook-Ereignisliste](/docs/de/hooks#hook-events).                                                                                                                                                                                                 |
| MCP-Server in `.mcp.json` werden nie geladen                                      | Datei befindet sich unter `.claude/` oder verwendet das Konfigurationsformat von Claude Desktop                                                             | Projekt-MCP-Konfiguration geht an die Repository-Root als `.mcp.json`, nicht in `.claude/`. Siehe [MCP-Konfiguration](/docs/de/mcp).                                                                                                                                                                                    |
| MCP-Server unter `mcpServers` in `settings.json` hinzugefügt, aber erscheinen nie | `settings.json` liest keinen `mcpServers`-Schlüssel                                                                                                         | Definieren Sie Projekt-Server in `.mcp.json` an der Repository-Root, oder führen Sie `claude mcp add --scope user` für benutzergesteuerte Server aus. Siehe [MCP-Konfiguration](/docs/de/mcp).                                                                                                                          |
| Projekt-MCP-Server hinzugefügt, aber erscheint nicht                              | Die einmalige Genehmigungsaufforderung wurde verworfen                                                                                                      | Projektbezogene Server erfordern Genehmigung. Führen Sie `/mcp` aus, um den Status zu sehen und zu genehmigen.                                                                                                                                                                                                     |
| MCP-Server kann nicht von einigen Verzeichnissen aus gestartet werden             | `command` oder `args` verwendet einen relativen Dateipfad                                                                                                   | Verwenden Sie absolute Pfade für lokale Skripte. Ausführbare Dateien auf Ihrem `PATH` wie `npx` oder `uvx` funktionieren wie gewohnt.                                                                                                                                                                              |
| MCP-Server startet ohne erwartete Umgebungsvariablen                              | Variablen befinden sich in `settings.json` `env`, die nicht an MCP-Kindprozesse weitergegeben werden                                                        | Setzen Sie stattdessen pro-Server `env` in `.mcp.json`.                                                                                                                                                                                                                                                            |
| `Bash(rm *)`-Deny-Regel blockiert nicht `/bin/rm` oder `find -delete`             | Präfix-Regeln entsprechen der wörtlichen Befehlszeichenkette, nicht der zugrunde liegenden ausführbaren Datei                                               | Fügen Sie explizite Muster für jede Variante hinzu, oder verwenden Sie einen [PreToolUse-Hook](/docs/de/hooks-guide) oder die [Sandbox](/docs/de/sandboxing) für eine harte Garantie.                                                                                                                                        |

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Für vollständige Referenzen zu jeder Konfigurationsoberfläche siehe die dedizierte Seite:

* **[`.claude`-Verzeichnisreferenz](/docs/de/claude-directory)**: jeder Konfigurationsdateispeicherort und wer ihn liest
* **[Einstellungen](/docs/de/settings)**: Prioritätsreihenfolge und die vollständige Schlüsselliste
* **[Hooks-Referenz](/docs/de/hooks)**: Ereignisnamen, Payloads und `--debug hooks`-Ausgabeformat
* **[MCP](/docs/de/mcp)**: Server-Konfiguration, Genehmigung und `/mcp`-Ausgabe
* **[Troubleshooting bei Installation und Anmeldung](/docs/de/troubleshoot-install)**: `command not found`, PATH und Authentifizierungsprobleme
* **[Troubleshooting](/docs/de/troubleshooting)**: Leistung, Hängen und Suchprobleme
