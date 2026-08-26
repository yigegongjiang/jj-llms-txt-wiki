> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# So funktioniert Claude Code

> Verstehen Sie die agentengesteuerte Schleife, integrierte Tools und wie Claude Code mit Ihrem Projekt interagiert.

Claude Code ist ein agentengesteuerter Assistent, der in Ihrem Terminal läuft. Obwohl er sich bei der Codierung auszeichnet, kann er bei allem helfen, was Sie von der Befehlszeile aus tun können: Dokumentation schreiben, Builds ausführen, Dateien durchsuchen, Themen recherchieren und vieles mehr.

Dieser Leitfaden behandelt die Kernarchitektur, integrierte Funktionen und [Tipps für effektive Zusammenarbeit mit Claude Code](#work-effectively-with-claude-code). Für schrittweise Anleitungen siehe [Häufige Workflows](/docs/de/common-workflows). Für Erweiterungsfunktionen wie skills, MCP und hooks siehe [Claude Code erweitern](/docs/de/features-overview).

<h2 id="the-agentic-loop">
  Die agentengesteuerte Schleife
</h2>

Wenn Sie Claude eine Aufgabe geben, arbeitet er durch drei Phasen: **Kontext sammeln**, **Maßnahmen ergreifen** und **Ergebnisse überprüfen**. Diese Phasen verschmelzen miteinander. Claude nutzt Tools durchgehend, ob beim Durchsuchen von Dateien zum Verständnis Ihres Codes, beim Bearbeiten zur Vornahme von Änderungen oder beim Ausführen von Tests zur Überprüfung seiner Arbeit.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agentic-loop.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=4a30fb7ce2815012a9f27c955e2c6bb0" alt="Diagramm der agentengesteuerten Schleife: Ihre Eingabeaufforderung führt dazu, dass Claude Kontext sammelt, Maßnahmen ergreift, Ergebnisse überprüft und wiederholt, bis die Aufgabe abgeschlossen ist. Sie können jederzeit unterbrechen." width="720" height="280" data-path="images/agentic-loop.svg" />

Die Schleife passt sich an das an, was Sie fragen. Eine Frage zu Ihrer Codebasis könnte nur Kontextsammlung erfordern. Eine Fehlerbehebung durchläuft alle drei Phasen wiederholt. Eine Umstrukturierung könnte umfangreiche Überprüfung beinhalten. Claude entscheidet, was jeder Schritt erfordert, basierend auf dem, was er aus dem vorherigen Schritt gelernt hat, verkettet Dutzende von Aktionen zusammen und korrigiert seinen Kurs unterwegs.

Sie sind auch Teil dieser Schleife. Sie können jederzeit unterbrechen, um Claude in eine andere Richtung zu lenken, zusätzlichen Kontext bereitzustellen oder ihn zu bitten, einen anderen Ansatz zu versuchen. Claude arbeitet autonom, bleibt aber responsiv gegenüber Ihrer Eingabe.

Die agentengesteuerte Schleife wird von zwei Komponenten angetrieben: [Modellen](#models), die denken, und [Tools](#tools), die handeln. Claude Code dient als **agentengesteuerte Umgebung** um Claude: Sie bietet die Tools, Kontextverwaltung und Ausführungsumgebung, die ein Sprachmodell in einen fähigen Codierungs-Agenten verwandeln.

<h3 id="models">
  Modelle
</h3>

Claude Code nutzt Claude-Modelle, um Ihren Code zu verstehen und über Aufgaben nachzudenken. Claude kann Code in jeder Sprache lesen, verstehen, wie Komponenten verbunden sind, und herausfinden, was sich ändern muss, um Ihr Ziel zu erreichen. Bei komplexen Aufgaben unterteilt er die Arbeit in Schritte, führt sie aus und passt sich basierend auf dem an, was er lernt.

[Mehrere Modelle](/docs/de/model-config) sind mit unterschiedlichen Kompromissen verfügbar. Sonnet bewältigt die meisten Codierungsaufgaben gut. Opus bietet stärkeres Denken für komplexe architektonische Entscheidungen. Wechseln Sie mit `/model` während einer Sitzung oder starten Sie mit `claude --model <name>`.

Wenn dieser Leitfaden sagt „Claude wählt" oder „Claude entscheidet", ist es das Modell, das die Überlegung durchführt.

<h3 id="tools">
  Tools
</h3>

Tools sind das, was Claude Code agentengesteuert macht. Ohne Tools kann Claude nur mit Text antworten. Mit Tools kann Claude handeln: Ihren Code lesen, Dateien bearbeiten, Befehle ausführen, das Web durchsuchen und mit externen Diensten interagieren. Jede Tool-Nutzung gibt Informationen zurück, die in die Schleife fließen und Claudes nächste Entscheidung informieren.

Die integrierten Tools fallen im Allgemeinen in fünf Kategorien, die jeweils eine andere Art von Agentur darstellen.

| Kategorie            | Was Claude tun kann                                                                                                                                                          |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Dateivorgänge**    | Dateien lesen, Code bearbeiten, neue Dateien erstellen, umbenennen und reorganisieren                                                                                        |
| **Suche**            | Dateien nach Muster finden, Inhalte mit Regex durchsuchen, Codebases erkunden                                                                                                |
| **Ausführung**       | Shell-Befehle ausführen, Server starten, Tests ausführen, git verwenden                                                                                                      |
| **Web**              | Das Web durchsuchen, Dokumentation abrufen, Fehlermeldungen nachschlagen                                                                                                     |
| **Code-Intelligenz** | Typfehler und Warnungen nach Bearbeitungen sehen, zu Definitionen springen, Referenzen finden (erfordert [Code-Intelligenz-Plugins](/docs/de/discover-plugins#code-intelligence)) |

Dies sind die primären Funktionen. Claude hat auch Tools zum Spawnen von Subagenten, zum Stellen von Fragen und für andere Orchestrierungsaufgaben. Siehe [Tools verfügbar für Claude](/docs/de/tools-reference) für die vollständige Liste.

Claude wählt basierend auf Ihrer Eingabeaufforderung und dem, was er unterwegs lernt, aus, welche Tools er verwenden soll. Wenn Sie sagen „beheben Sie die fehlgeschlagenen Tests", könnte Claude:

1. Die Test-Suite ausführen, um zu sehen, was fehlschlägt
2. Die Fehlerausgabe lesen
3. Nach den relevanten Quelldateien suchen
4. Diese Dateien lesen, um den Code zu verstehen
5. Die Dateien bearbeiten, um das Problem zu beheben
6. Die Tests erneut ausführen, um zu überprüfen

Jede Tool-Nutzung gibt Claude neue Informationen, die den nächsten Schritt informieren. Dies ist die agentengesteuerte Schleife in Aktion.

**Erweitern der Basisfunktionen:** Die integrierten Tools sind die Grundlage. Sie können das, was Claude weiß, mit [skills](/docs/de/skills) erweitern, sich mit externen Diensten mit [MCP](/docs/de/mcp) verbinden, Workflows mit [hooks](/docs/de/hooks) automatisieren und Aufgaben an [subagents](/docs/de/sub-agents) delegieren. Diese Erweiterungen bilden eine Schicht auf der Grundlage der agentengesteuerten Schleife. Siehe [Claude Code erweitern](/docs/de/features-overview) für Anleitung zur Auswahl der richtigen Erweiterung für Ihre Anforderungen.

<h2 id="what-claude-can-access">
  Worauf Claude zugreifen kann
</h2>

Dieser Leitfaden konzentriert sich auf das Terminal. Claude Code läuft auch in [VS Code](/docs/de/vs-code), [JetBrains IDEs](/docs/de/jetbrains) und anderen Umgebungen.

Wenn Sie `claude` in einem Verzeichnis ausführen, erhält Claude Code Zugriff auf:

* **Ihr Projekt.** Dateien in Ihrem Verzeichnis und Unterverzeichnissen sowie Dateien an anderer Stelle mit Ihrer Genehmigung.
* **Ihr Terminal.** Jeden Befehl, den Sie ausführen könnten: Build-Tools, git, Paketmanager, Systemdienstprogramme, Skripte. Wenn Sie es von der Befehlszeile aus tun können, kann Claude es auch.
* **Ihren git-Status.** Aktueller Branch, nicht committete Änderungen und aktuelle Commit-Historie.
* **Ihre [CLAUDE.md](/docs/de/memory).** Eine Markdown-Datei, in der Sie projektspezifische Anweisungen, Konventionen und Kontext speichern, den Claude jede Sitzung kennen sollte.
* **[Auto-Speicher](/docs/de/memory#auto-memory).** Erkenntnisse, die Claude automatisch speichert, während Sie arbeiten, wie Projektmuster und Ihre Vorlieben. Die ersten 200 Zeilen oder 25 KB von MEMORY.md, je nachdem, was zuerst kommt, werden zu Beginn jeder Sitzung geladen.
* **Erweiterungen, die Sie konfigurieren.** [MCP-Server](/docs/de/mcp) für externe Dienste, [skills](/docs/de/skills) für Workflows, [subagents](/docs/de/sub-agents) für delegierte Arbeit und [Claude in Chrome](/docs/de/chrome) für Browser-Interaktion.

Da Claude Ihr gesamtes Projekt sieht, kann er darin arbeiten. Wenn Sie Claude bitten, „den Authentifizierungsfehler zu beheben", sucht er nach relevanten Dateien, liest mehrere Dateien, um den Kontext zu verstehen, nimmt koordinierte Bearbeitungen vor, führt Tests aus, um die Behebung zu überprüfen, und committed die Änderungen, wenn Sie es fragen. Dies unterscheidet sich von Inline-Code-Assistenten, die nur die aktuelle Datei sehen.

<h2 id="environments-and-interfaces">
  Umgebungen und Schnittstellen
</h2>

Die agentengesteuerte Schleife, Tools und Funktionen, die oben beschrieben sind, sind überall gleich, wo Sie Claude Code verwenden. Was sich ändert, ist, wo der Code ausgeführt wird und wie Sie damit interagieren.

<h3 id="execution-environments">
  Ausführungsumgebungen
</h3>

Claude Code läuft in drei Umgebungen, jede mit unterschiedlichen Kompromissen für die Ausführung Ihres Codes.

| Umgebung           | Wo Code läuft                             | Anwendungsfall                                                       |
| ------------------ | ----------------------------------------- | -------------------------------------------------------------------- |
| **Lokal**          | Ihr Computer                              | Standard. Vollständiger Zugriff auf Ihre Dateien, Tools und Umgebung |
| **Cloud**          | Von Anthropic verwaltete VMs              | Aufgaben auslagern, an Repos arbeiten, die Sie nicht lokal haben     |
| **Remote Control** | Ihr Computer, gesteuert von einem Browser | Verwenden Sie die Web-UI, während Sie alles lokal halten             |

<h3 id="interfaces">
  Schnittstellen
</h3>

Sie können auf Claude Code über das Terminal, die [Desktop-App](/docs/de/desktop), [IDE-Erweiterungen](/docs/de/vs-code), [claude.ai/code](https://claude.ai/code), [Remote Control](/docs/de/remote-control), [Slack](/docs/de/slack) und [CI/CD-Pipelines](/docs/de/github-actions) zugreifen. Die Schnittstelle bestimmt, wie Sie Claude sehen und damit interagieren, aber die zugrunde liegende agentengesteuerte Schleife ist identisch. Siehe [Claude Code überall verwenden](/docs/de/overview#use-claude-code-everywhere) für die vollständige Liste.

<h2 id="work-with-sessions">
  Mit Sitzungen arbeiten
</h2>

Claude Code speichert Ihre Konversation lokal, während Sie arbeiten. Jede Nachricht, Tool-Nutzung und jedes Ergebnis wird in einer Klartextdatei im JSONL-Format unter `~/.claude/projects/` geschrieben, was [Zurückspulen](#undo-changes-with-checkpoints), [Fortsetzen und Verzweigen](#resume-or-fork-sessions) von Sitzungen ermöglicht. Bevor Claude Code-Änderungen vornimmt, erstellt er auch einen Snapshot der betroffenen Dateien, damit Sie bei Bedarf zurückrollen können. Für Pfade, Aufbewahrung und wie Sie diese Daten löschen, siehe [Anwendungsdaten in `~/.claude`](/docs/de/claude-directory#application-data).

**Sitzungen sind unabhängig.** Jede neue Sitzung beginnt mit einem frischen Kontextfenster, ohne die Konversationshistorie aus vorherigen Sitzungen. Claude kann Erkenntnisse über Sitzungen hinweg mit [Auto-Speicher](/docs/de/memory#auto-memory) beibehalten, und Sie können Ihre eigenen persistenten Anweisungen in [CLAUDE.md](/docs/de/memory) hinzufügen.

<h3 id="work-across-branches">
  Über Branches arbeiten
</h3>

Jede Claude Code-Konversation ist eine Sitzung, die an Ihr aktuelles Verzeichnis gebunden ist. Die `/resume`-Auswahl zeigt standardmäßig Sitzungen aus dem aktuellen Worktree an, mit Tastaturkürzeln zum Erweitern der Liste auf andere Worktrees oder Projekte. Siehe [Sitzungen verwalten](/docs/de/sessions#use-the-session-picker) für die vollständige Liste der Auswahl-Tastaturkürzeln und wie die Namensauflösung funktioniert.

Claude sieht die Dateien Ihres aktuellen Branches. Wenn Sie Branches wechseln, sieht Claude die Dateien des neuen Branches, aber Ihre Konversationshistorie bleibt gleich. Claude erinnert sich an das, was Sie besprochen haben, auch nach dem Wechsel.

Da Sitzungen an Verzeichnisse gebunden sind, können Sie parallele Claude-Sitzungen ausführen, indem Sie [git worktrees](/docs/de/worktrees) verwenden, die separate Verzeichnisse für einzelne Branches erstellen.

<h3 id="resume-or-fork-sessions">
  Sitzungen fortsetzen oder verzweigen
</h3>

Wenn Sie eine Sitzung mit `claude --continue` oder `claude --resume` fortsetzen, setzen Sie diese unter derselben Sitzungs-ID fort und hängen neue Nachrichten an die bestehende Konversation an. Wenn Sie mit `--fork-session` oder `/branch` verzweigen, wird die Historie in eine neue Sitzungs-ID kopiert, wobei die ursprüngliche unverändert bleibt.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/session-continuity.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=04ed0984a58e4127e05b3640265241a3" alt="Sitzungskontinuität: Fortsetzen setzt dieselbe Sitzung fort, Verzweigung erstellt einen neuen Branch mit einer neuen ID." width="560" height="280" data-path="images/session-continuity.svg" />

Für die Fortsetzen-Flags, die `/resume`-Auswahl, Benennung und was passiert, wenn dieselbe Sitzung in zwei Terminals offen ist, siehe [Sitzungen verwalten](/docs/de/sessions).

<h3 id="the-context-window">
  Das Kontextfenster
</h3>

Claudes Kontextfenster enthält Ihre Konversationshistorie, Dateiinhalte, Befehlsausgaben, [CLAUDE.md](/docs/de/memory), [Auto-Speicher](/docs/de/memory#auto-memory), geladene Skills und Systemanweisungen. Während Sie arbeiten, füllt sich der Kontext. Claude komprimiert automatisch, aber Anweisungen von früh in der Konversation können verloren gehen. Legen Sie persistente Regeln in CLAUDE.md ab, und führen Sie `/context` aus, um zu sehen, was Platz verbraucht.

Für eine interaktive Anleitung zu dem, was geladen wird und wann, siehe [Erkunden Sie das Kontextfenster](/docs/de/context-window).

<h4 id="when-context-fills-up">
  Wenn der Kontext voll wird
</h4>

Claude Code verwaltet den Kontext automatisch, wenn Sie sich dem Limit nähern. Es löscht zuerst ältere Tool-Ausgaben, dann fasst die Konversation zusammen, falls erforderlich. Ihre Anfragen und wichtige Code-Snippets werden beibehalten; detaillierte Anweisungen von früh in der Konversation können verloren gehen. Legen Sie persistente Regeln in CLAUDE.md ab, anstatt sich auf die Konversationshistorie zu verlassen.

Um zu kontrollieren, was während der Komprimierung beibehalten wird, fügen Sie einen Abschnitt „Compact Instructions" zu CLAUDE.md hinzu oder führen Sie `/compact` mit einem Fokus aus (wie `/compact focus on the API changes`).

Wenn eine einzelne Datei oder Tool-Ausgabe so groß ist, dass sich der Kontext unmittelbar nach jeder Zusammenfassung wieder füllt, stoppt Claude Code die automatische Komprimierung nach einigen Versuchen und zeigt stattdessen einen Fehler an. Siehe [Auto-Komprimierung stoppt mit einem Thrashing-Fehler](/docs/de/troubleshooting#auto-compaction-stops-with-a-thrashing-error) für Wiederherstellungsschritte.

Führen Sie `/context` aus, um zu sehen, was Platz verbraucht. MCP-Tool-Definitionen werden standardmäßig aufgeschoben und bei Bedarf über [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) geladen, daher verbrauchen nur Tool-Namen Kontext, bis Claude ein bestimmtes Tool verwendet. Führen Sie `/mcp` aus, um die Kosten pro Server zu überprüfen.

<h4 id="manage-context-with-skills-and-subagents">
  Kontext mit Skills und Subagents verwalten
</h4>

Über die Komprimierung hinaus können Sie andere Funktionen verwenden, um zu kontrollieren, was in den Kontext geladen wird.

[Skills](/docs/de/skills) werden bei Bedarf geladen. Claude sieht Skill-Beschreibungen zu Sitzungsbeginn, aber der vollständige Inhalt wird nur geladen, wenn ein Skill verwendet wird. Für Skills, die Sie manuell aufrufen, setzen Sie `disable-model-invocation: true`, um Beschreibungen aus dem Kontext zu halten, bis Sie sie benötigen. Für Skills, die Sie nicht geschrieben haben, verwenden Sie [`skillOverrides`](/docs/de/skills#override-skill-visibility-from-settings), um dasselbe aus den Einstellungen zu tun.

[Subagents](/docs/de/sub-agents) erhalten ihren eigenen frischen Kontext, völlig getrennt von Ihrer Hauptkonversation. Ihre Arbeit bläht Ihren Kontext nicht auf. Wenn sie fertig sind, geben sie eine Zusammenfassung zurück. Diese Isolation ist der Grund, warum Subagents bei langen Sitzungen helfen.

Siehe [Kontextkosten](/docs/de/features-overview#understand-context-costs) für die Kosten jeder Funktion und [Token-Nutzung reduzieren](/docs/de/costs#reduce-token-usage) für Tipps zur Verwaltung des Kontexts.

<h2 id="stay-safe-with-checkpoints-and-permissions">
  Sicher bleiben mit Checkpoints und Berechtigungen
</h2>

Claude hat zwei Sicherheitsmechanismen: Checkpoints ermöglichen es Ihnen, Dateiänderungen rückgängig zu machen, und Berechtigungen kontrollieren, was Claude ohne Nachfrage tun kann.

<h3 id="undo-changes-with-checkpoints">
  Änderungen mit Checkpoints rückgängig machen
</h3>

**Jede Dateibearbeitung ist reversibel.** Bevor Claude eine Datei bearbeitet, erstellt er einen Snapshot des aktuellen Inhalts. Wenn etwas schief geht, drücken Sie zweimal `Esc`, um zu einem vorherigen Zustand zurückzuspulen, oder bitten Sie Claude, rückgängig zu machen.

Checkpoints sind getrennt von git und bleiben verfügbar, wenn Sie ein Gespräch fortsetzen. Sie decken nur Dateiänderungen ab. Aktionen, die sich auf Remote-Systeme auswirken (Datenbanken, APIs, Bereitstellungen), können nicht checkpointed werden, weshalb Claude vor dem Ausführen von Befehlen mit externen Nebenwirkungen fragt.

<h3 id="control-what-claude-can-do">
  Kontrollieren Sie, was Claude tun kann
</h3>

Drücken Sie `Shift+Tab`, um durch die Berechtigungsmodi zu wechseln:

* **Manual**: Claude fragt vor Dateibearbeitungen und Shell-Befehlen
* **Accept edits**: Claude bearbeitet Dateien und führt häufige Dateisystem-Befehle wie `mkdir` und `mv` ohne Nachfrage aus, fragt aber immer noch nach anderen Befehlen
* **Plan**: Claude erkundet und schlägt einen Plan vor, ohne Ihre Quelldateien zu bearbeiten
* **Auto**: Claude bewertet alle Aktionen mit Hintergrund-Sicherheitsprüfungen

Sie können auch spezifische Befehle in `.claude/settings.json` zulassen, damit Claude nicht jedes Mal fragt. Dies ist nützlich für vertrauenswürdige Befehle wie `npm test` oder `git status`. Einstellungen können von organisationsweiten Richtlinien bis zu persönlichen Vorlieben reichen. Siehe [Berechtigungen](/docs/de/permissions) für Details.

***

<h2 id="work-effectively-with-claude-code">
  Effektiv mit Claude Code arbeiten
</h2>

Diese Tipps helfen Ihnen, bessere Ergebnisse von Claude Code zu erhalten.

<h3 id="ask-claude-code-for-help">
  Fragen Sie Claude Code um Hilfe
</h3>

Claude Code kann Ihnen beibringen, wie man ihn verwendet. Stellen Sie Fragen wie „Wie richte ich hooks ein?" oder „Was ist der beste Weg, meine CLAUDE.md zu strukturieren?" und Claude wird erklären.

Integrierte Befehle führen Sie auch durch die Einrichtung:

* `/init` führt Sie durch die Erstellung einer CLAUDE.md für Ihr Projekt
* `/doctor` führt eine Einrichtungsüberprüfung durch, die Installations- und Konfigurationsprobleme diagnostiziert und diese beheben kann

<h3 id="it’s-a-conversation">
  Es ist eine Konversation
</h3>

Claude Code ist konversativ. Sie benötigen keine perfekten Eingabeaufforderungen. Beginnen Sie mit dem, was Sie möchten, und verfeinern Sie dann:

```text theme={null}
Beheben Sie den Login-Fehler
```

\[Claude untersucht, versucht etwas]

```text theme={null}
Das ist nicht ganz richtig. Das Problem liegt in der Sitzungsverwaltung.
```

\[Claude passt seinen Ansatz an]

Wenn der erste Versuch nicht richtig ist, müssen Sie nicht von vorne anfangen. Sie iterieren.

<h4 id="interrupt-and-steer">
  Unterbrechen und lenken
</h4>

Sie können Claude jederzeit unterbrechen, ohne auf das Ende des Durchgangs zu warten oder von vorne anzufangen:

* **Drücken Sie `Esc`**, um Claude sofort zu stoppen. Der laufende Toolaufruf wird abgebrochen und Claude wartet auf Ihre nächste Anweisung.
* **Geben Sie eine Korrektur ein und drücken Sie `Enter`**, um sie zu senden, ohne das laufende Tool zu stoppen. Claude liest sie, sobald die aktuelle Aktion abgeschlossen ist, und passt sich an, bevor er seinen nächsten Schritt entscheidet.

<h3 id="be-specific-upfront">
  Seien Sie von Anfang an spezifisch
</h3>

Je präziser Ihre anfängliche Eingabeaufforderung ist, desto weniger Korrektionen benötigen Sie. Verweisen Sie auf spezifische Dateien, erwähnen Sie Einschränkungen und zeigen Sie auf Beispielmuster.

```text theme={null}
Der Checkout-Fluss ist für Benutzer mit abgelaufenen Karten unterbrochen.
Überprüfen Sie src/payments/ auf das Problem, besonders Token-Aktualisierung.
Schreiben Sie zuerst einen fehlgeschlagenen Test, dann beheben Sie ihn.
```

Vage Eingabeaufforderungen funktionieren, aber Sie werden mehr Zeit mit Lenkung verbringen. Spezifische Eingabeaufforderungen wie die obige gelingen oft beim ersten Versuch.

<h3 id="give-claude-something-to-verify-against">
  Geben Sie Claude etwas zum Überprüfen
</h3>

Claude funktioniert besser, wenn er seine eigene Arbeit überprüfen kann. Fügen Sie Testfälle ein, fügen Sie Screenshots des erwarteten UI ein oder definieren Sie die gewünschte Ausgabe.

```text theme={null}
Implementieren Sie validateEmail. Testfälle: 'user@example.com' → true,
'invalid' → false, 'user@.com' → false. Führen Sie die Tests danach aus.
```

Für visuelle Arbeit fügen Sie einen Screenshot des Designs ein und bitten Sie Claude, seine Implementierung dagegen zu vergleichen.

<h3 id="explore-before-implementing">
  Vor der Implementierung erkunden
</h3>

Bei komplexen Problemen trennen Sie Forschung von Codierung. Verwenden Sie Plan Mode (`Shift+Tab` zweimal), um die Codebasis zuerst zu analysieren:

```text theme={null}
Lesen Sie src/auth/ und verstehen Sie, wie wir Sitzungen handhaben.
Erstellen Sie dann einen Plan zum Hinzufügen von OAuth-Unterstützung.
```

Überprüfen Sie den Plan, verfeinern Sie ihn durch Konversation, dann lassen Sie Claude implementieren. Dieser zweiphasige Ansatz erzeugt bessere Ergebnisse als direkt zum Code zu springen.

<h3 id="delegate-don’t-dictate">
  Delegieren, nicht diktieren
</h3>

Denken Sie daran, an einen fähigen Kollegen zu delegieren. Geben Sie Kontext und Richtung, dann vertrauen Sie Claude, die Details herauszufinden:

```text theme={null}
Der Checkout-Fluss ist für Benutzer mit abgelaufenen Karten unterbrochen.
Der relevante Code ist in src/payments/. Können Sie ihn untersuchen und beheben?
```

Sie müssen nicht angeben, welche Dateien zu lesen sind oder welche Befehle auszuführen sind. Claude findet das heraus.

<h2 id="what’s-next">
  Nächste Schritte
</h2>

<CardGroup cols={2}>
  <Card title="Mit Funktionen erweitern" icon="puzzle-piece" href="/docs/de/features-overview">
    Fügen Sie Skills, MCP-Verbindungen und benutzerdefinierte Befehle hinzu
  </Card>

  <Card title="Häufige Workflows" icon="graduation-cap" href="/docs/de/common-workflows">
    Schritt-für-Schritt-Anleitungen für typische Aufgaben
  </Card>
</CardGroup>
