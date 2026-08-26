> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kommunikations-Kit

> Startankündigungen, Drip-Campaign-Nachrichten und FAQ-Antworten für die Einführung von Claude Code in Ihrer Entwicklungsorganisation.

Diese Seite ist für Administratoren und Entwicklungsteamleiter, die Claude Code in einem Team einführen. Sie bietet einsatzbereite Startankündigungen, eine Tipps-und-Tricks-Drip-Campaign und einzeilige FAQ-Antworten für die Fragen, die Sie am häufigsten gestellt bekommen.

<Note>
  Behandeln Sie alles hier als Entwurfskopie, nicht als fertige Kopie. Schreiben Sie jede Nachricht in der Stimme Ihrer Organisation um, ersetzen Sie die Beispielaufgaben durch echte Bugs und Module aus Ihrer eigenen Codebasis und ersetzen Sie die `[eingeklammerten Platzhalter]` vor dem Versand. Die Ankündigungen, die Akzeptanz fördern, sind diejenigen, die sich so lesen, als hätte sie jemand aus Ihrem Unternehmen geschrieben.
</Note>

<h2 id="launch-communications">
  Startmitteilungen
</h2>

Eine Ankündigung in zwei Formaten plus zwei optionale Varianten. Wählen Sie diejenige aus, die zu Ihrem Rollout passt, und schreiben Sie sie von dort aus um.

<h3 id="before-you-send">
  Bevor Sie senden
</h3>

Arbeiten Sie diese Checkliste durch, bevor die Ankündigung rausgeht. Jeder Punkt schließt eine Lücke, die sonst zu einem Support-Thread am Starttag wird.

| Element                                                                                                    | Warum es wichtig ist                                                                                                            |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `#claude-code` Kanal erstellt und in der Nachricht verlinkt                                                | Gibt Fragen einen Ort zum Landen                                                                                                |
| Installationsbefehl auf mindestens einem Computer in Ihrer Umgebung getestet                               | Fängt Proxy- oder Firewall-Probleme ab, bevor alle sie gleichzeitig treffen                                                     |
| Sicherheits- und Datenbehandlungslink bereit ([Datennutzung](/docs/de/data-usage) oder Ihr internes Äquivalent) | „Wo geht mein Code hin?" wird die erste Antwort sein                                                                            |
| Eine konkrete erste Aufgabe ausgewählt, ein echter Bug oder eine Datei in Ihrer Codebasis                  | Generische Beispiele funktionieren nicht; „Behebe den flaky Test in `auth_test.go`" schon                                       |
| Ein benannter Besitzer für den Kanal für die ersten 48 Stunden                                             | Unbeantwortete Fragen am Starttag töten den Schwung                                                                             |
| Ein C-Suite-Sponsor, der die Ankündigung sendet oder mitunterzeichnet                                      | Von Führungskräften gesendete Starts sehen durchweg höhere Adoptionsraten in der ersten Woche als von Administratoren gesendete |

<h3 id="the-announcement">
  Die Ankündigung
</h3>

Verwenden Sie dies als Ihre Standard-Ankündigung für die gesamte Organisation. Sie erklärt, was Claude Code ist, bietet einen zweiminütigen Installationspfad, gibt Lesern eine konkrete Aufgabe zum Ausprobieren und beantwortet „Wo geht mein Code hin?" bevor jemand fragen muss.

<Tabs>
  <Tab title="E-Mail">
    ```text theme={null}
    Betreff: Claude Code ist live für [Engineering / Ihr Team]

    Team,

    Ab heute haben Sie Zugriff auf Claude Code, einen KI-Coding-Agent, der in
    Ihrem Terminal läuft, Ihre tatsächliche Codebasis liest und echte Aufgaben
    von Anfang bis Ende durcharbeitet: Debugging, Refactorings, Tests, PRs. Es
    ist keine Autovervollständigung und kein Chat-Fenster. Es bearbeitet
    Dateien, führt Ihre Befehle aus und fragt die Berechtigung ab, bevor etwas
    Riskantes passiert.

    Starten Sie in zwei Minuten:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <your-repo>
        claude

    Führen Sie dann `/init` einmal aus. Claude liest Ihr Projekt und schreibt
    eine CLAUDE.md mit Ihren Build-Befehlen und Konventionen, sodass Sie
    aufhören, die Grundlagen zu wiederholen.

    Versuchen Sie dann eines davon im Repo, an dem Sie bereits arbeiten:

      - 'Der Test in [Datei] ist flaky. Finde heraus, warum und behebe es"
      - „Zeige mir, wie [Modul] [X] handhabt"
      - „Schau dir meinen funktionierenden Diff an und sag mir, was riskant ist,
        bevor ich pushe"

    Wo Ihr Code hingeht: Claude Code läuft in Ihrem Terminal und spricht direkt
    mit Anthropics API, ohne dass dritte Server in der Schleife sind. Es fragt,
    bevor es Dateien bearbeitet oder Befehle ausführt. Unter unserer
    Enterprise-Vereinbarung verwendet Anthropic Ihren Code oder Prompts nicht
    zum Trainieren seiner Modelle.
    Details: https://code.claude.com/docs/de/data-usage
             https://code.claude.com/docs/de/security

    Wo Sie mit Fragen hingehen: #claude-code. [Besitzername] beobachtet es
    diese Woche.

    - [Name]

    P.S. Bevorzugen Sie Ihren Editor? Es gibt eine VS Code-Erweiterung und ein
    JetBrains-Plugin. Derselbe Agent, kein Terminal erforderlich.
    ```
  </Tab>

  <Tab title="Slack oder Teams">
    ```markdown theme={null}
    🚀 *Claude Code ist live für [Team]*

    KI-Coding-Agent, läuft in Ihrem Terminal, liest Ihr Repo, macht echte
    Arbeit: Bugs, Refactorings, Tests, PRs. Fragt, bevor es etwas anfasst.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` → `claude`

    *Erste Sache zum Ausprobieren* → führen Sie `/init` aus, dann: 'Der Test in
    [Datei] ist flaky, finde heraus, warum und behebe es."

    🔒 Läuft in Ihrem Terminal, spricht nur mit Anthropics API. Unter unserem
    Enterprise-Plan werden Ihr Code und Prompts nicht zum Trainieren von
    Modellen verwendet.
    Datennutzung → https://code.claude.com/docs/de/data-usage

    📚 Schnellstart · VS Code · Kostenloser 1-Stunden-Kurs
       https://code.claude.com/docs/de/quickstart
       https://code.claude.com/docs/de/vs-code
       https://anthropic.skilljar.com/claude-code-in-action

    Fragen → dieser Thread. [Besitzer] ist verantwortlich.
    ```
  </Tab>
</Tabs>

<h3 id="executive-sponsor-variant">
  Variante für Executive Sponsor
</h3>

Senden Sie dies von Ihrem sponsernden Executive, wie dem CTO, CIO oder SVP Engineering, unter seinem Namen und von seinem Konto. Starts, die unter dem Namen eines Executives rausgehen, sehen durchweg höhere Öffnungsraten und schnellere Aktivierung in der ersten Woche als dieselbe Nachricht von einem Administrator oder Tooling-Team. Es signalisiert eine Unternehmenspriorität statt eines optionalen Experiments.

Diese Version ist absichtlich auf eine Anfrage reduziert: Installieren Sie es und führen Sie es auf einer echten Aufgabe aus. Die Aufgabe des Executives ist es, die Anfrage landen zu lassen; die Standard-Ankündigung und `#claude-code` kümmern sich um das Wie.

<Tabs>
  <Tab title="E-Mail">
    ```text theme={null}
    Betreff: Eine Sache, die ich diese Woche von jedem Engineer ausprobiert
    haben möchte

    Team,

    Wir haben Claude Code für alle Engineering-Teams aktiviert. Es ist ein
    KI-Agent, der direkt in Ihrem Terminal auf Ihrer tatsächlichen Codebasis
    arbeitet, und die frühen Ergebnisse von Teams, die es bereits verwenden,
    sind stark genug, dass ich diese Woche alle darauf haben möchte.

    Ich bitte um zehn Minuten:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <your-repo>
        claude

    Geben Sie ihm dann eine echte Aufgabe: den Bug, den Sie aufgeschoben haben,
    oder 'Zeige mir, wie [Modul] funktioniert."

    Das ist die ganze Anfrage. [Besitzername] und Team sind in #claude-code für
    alles, das Sie unterwegs treffen.

    - [Executive Name]
      [Titel]
    ```
  </Tab>

  <Tab title="Slack oder Teams">
    ```markdown theme={null}
    📣 *Von [Executive Name]: eine Sache zum Ausprobieren diese Woche*

    Wir haben *Claude Code* für alle Engineering-Teams aktiviert. Frühe
    Ergebnisse sind stark genug, dass ich alle bitte, diese Woche zehn Minuten
    auf echte Arbeit zu geben.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` →
    `claude` → geben Sie ihm eine echte Aufgabe.

    Das ist alles. Fragen → #claude-code.
    ```
  </Tab>
</Tabs>

<h3 id="pilot-group-variant">
  Variante für Pilotgruppe
</h3>

Verwenden Sie für einen gestaffelten Rollout. Senden Sie nur an die Pilot-Kohorte.

```text theme={null}
Betreff: Sie sind im Claude Code Pilot

[Name / Team],

Sie sind in der ersten Welle von Claude Code bei [Unternehmen]. Wir haben
diese Gruppe ausgewählt, weil Sie es auf echte Probleme anwenden und uns
die Wahrheit darüber sagen werden.

Die Anfrage: Verwenden Sie es diese Woche auf mindestens einer echten
Aufgabe, dann schreiben Sie eine Notiz in #claude-code-pilot, die abdeckt,
was funktioniert hat, was nervig war und was Sie überrascht hat. Dieses
Feedback entscheidet, wie wir es für alle anderen einführen.

[Fortfahren mit 'Starten Sie in zwei Minuten" aus der Standard-Ankündigung]

Eine zusätzliche Sache für Piloten: Bei Ihrer ersten Änderung mit mehreren
Dateien drücken Sie Shift+Tab, bis Sie „plan" sehen. Claude wird genau
aufzeigen, was es beabsichtigt zu tun, bevor es eine Datei anfasst. Es ist
der schnellste Weg, um zu kalibrieren, wie viel Sie ihm vertrauen können.
```

<h3 id="champion-recruitment-dm">
  Champion-Rekrutierungs-DM
</h3>

Nach dem Start, DM an die zwei oder drei aktivsten Personen in `#claude-code`.

```text theme={null}
Hey [Name], Ihre #claude-code-Posts tun mehr für die Akzeptanz als meine
Ankündigung. Ein paar Leute haben mir gesagt, dass Ihr [Thread / Screenshot]
der Grund war, warum sie es tatsächlich ausprobiert haben.

Möchten Sie das halboffiziell machen? Wenig Aufwand: Posten Sie größtenteils
weiterhin, was Sie posten, plus erste Chance auf neue Features und eine
direkte Leitung zum Anthropic-Team. Ich kann ein kurzes Playbook teilen,
wenn Sie dabei sind.
```

<h2 id="tips-and-tricks-campaign">
  Tipps und Tricks Campaign
</h2>

Einsatzbereite Slack- oder Teams-Nachrichten, die nach dem Start die Feature-Aktivierung fördern. Jede folgt demselben Muster: ein Hook, der Gewinn, eine „Probieren Sie es jetzt"-Aufforderung und ein Dokumentationslink. Verteilen Sie sie eine oder zwei pro Woche in `#claude-code`, oder wählen Sie die Handvoll aus, die zu den Lücken Ihres Teams passt. Sie stehen allein ohne erforderliche Reihenfolge.

Kopieren Sie den Nachrichtentext aus jedem Block direkt in Slack oder Teams. Ersetzen Sie `[eingeklammerte Platzhalter]` vor dem Versand.

<h3 id="get-started">
  Erste Schritte
</h3>

**Das richtige Modell wählen**

```markdown theme={null}
🎯 *Tipp: Passen Sie das Modell zum Moment an*

Opus zu verwenden, um einen Tippfehler zu beheben, verschwendet Rechenleistung.
Haiku für ein 12-Datei-Refactoring zu verwenden, ist eine Wiederholung
verlangen.

Claude Code läuft auf denselben Modellen wie die Claude-App, und Sie können
mid-session wechseln. *Sonnet* ist der zuverlässige Standard für alltägliche
Feature-Arbeit, Bugs, Tests und Reviews. Greifen Sie zu *Opus* bei großen
Refactorings, kniffligem Debugging oder allem mit hohem Einsatz. Fallen Sie
auf *Haiku* für schnelle Fragen, Formatierung und mechanische Bearbeitungen,
bei denen Geschwindigkeit gewinnt. *Fable 5* ist das fähigste Modell für Ihre
schwierigsten, längsten Aufgaben; es ist nicht der Standard, also wählen Sie
es mit `/model fable` aus, und beachten Sie, dass Cybersicherheits- und
Biologieinhalte automatisch auf Opus zurückfallen.

*Probieren Sie es jetzt:* Geben Sie `/model` ein und wählen Sie Sonnet, wenn
Sie es noch nicht getan haben. Es ist der richtige Standard für die meisten
Aufgaben.

📖 Modellkonfiguration → https://code.claude.com/docs/de/model-config
```

| Modell  | Am besten für                                                                                                                                                                                                            |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Fable 5 | Die schwierigsten, längsten Aufgaben. Nur auf Opt-in-Basis: Wählen Sie es mit `/model fable` aus. Cybersicherheits- oder Biologieinhalte [fallen automatisch auf Opus zurück](/docs/de/model-config#automatic-model-fallback) |
| Opus    | Großflächige Refactorings, komplexes Debugging, Architekturentscheidungen, hochriskante Änderungen                                                                                                                       |
| Sonnet  | Alltägliche Feature-Arbeit, Bug-Fixes, Tests, Dokumentation, Code-Review. Empfohlener Standard.                                                                                                                          |
| Haiku   | Schnelle Fragen, Formatierung, mechanische Bearbeitungen, schnelle Iteration                                                                                                                                             |

**Schnelle Gewinne zum ersten Ausprobieren**

```markdown theme={null}
🚀 *Tipp: Drei Dinge zum Ausprobieren in Ihren ersten 10 Minuten*

Claude Code installiert, aber nicht sicher, was Sie tatsächlich fragen
sollen? Beginnen Sie mit dem Zeug, das Sie die ganze Woche über nervt.

  - Beheben Sie etwas Nerviges: „Der Test in [Datei] ist flaky, finde heraus,
    warum"
  - Orientieren Sie sich in Code, den Sie nicht geschrieben haben: „Zeige mir,
    wie [Modul] funktioniert"
  - Sanity-Check vor dem Push: „Schau dir meinen funktionierenden Diff an und
    sag mir, was riskant aussieht"

Keine davon benötigt Setup. Einfach `cd` in Ihr Repo und führen Sie `claude`
aus.

*Probieren Sie es jetzt:* Wählen Sie den Bug, den Sie vermieden haben, und
fügen Sie die Fehlermeldung ein.

📖 Schnellstart → https://code.claude.com/docs/de/quickstart
```

<h3 id="project-memory">
  Projektgedächtnis
</h3>

**`/init` und CLAUDE.md**

```markdown theme={null}
📁 *Tipp: Hören Sie auf, Ihr Repo jede Sitzung zu erklären*

Claude zum fünften Mal sagen „wir verwenden pnpm, nicht npm"? Es gibt eine
einmalige Lösung.

Führen Sie `/init` einmal pro Repo aus. Claude liest Ihre Projektstruktur
und schreibt eine CLAUDE.md-Datei mit Ihren Build-Befehlen, Architektur und
Konventionen. Jede zukünftige Sitzung in diesem Repo beginnt automatisch mit
dieser Datei. Halten Sie sie unter zwei Bildschirmen. Es ist ein Spickzettel,
keine Dokumentation.

*Probieren Sie es jetzt:* Öffnen Sie Ihr Haupt-Repo, führen Sie `claude` aus,
geben Sie `/init` ein. Dreißig Sekunden, zahlt sich nach jeder Sitzung aus.

📖 CLAUDE.md und Projektgedächtnis → https://code.claude.com/docs/de/memory
```

**@-Referenzen**

```markdown theme={null}
📎 *Tipp: Hören Sie auf, Dateiinhalte in den Chat zu kopieren*

200 Zeilen einer Komponente in Ihren Prompt kopieren, damit Claude sie
„sehen" kann? Das müssen Sie nicht.

Geben Sie `@` ein, dann einen Dateipfad. Claude zieht die Datei direkt in
den Kontext. Funktioniert auch für ganze Verzeichnisse.

> die Stile in @src/components/Button.tsx sehen falsch aus, überprüfen Sie
> gegen @docs/design-system.md

*Probieren Sie es jetzt:* Geben Sie `@` ein, dann Tab. Autovervollständigung
zeigt Ihnen jede Datei in Reichweite.

📖 Dateien referenzieren → https://code.claude.com/docs/de/common-workflows
```

<h3 id="control-and-safety">
  Kontrolle und Sicherheit
</h3>

**Berechtigungsmodi**

```markdown theme={null}
🛡️ *Tipp: Ein Tastendruck zwischen „Schauen, aber nicht anfassen" und
„Mach es einfach"*

Manchmal möchten Sie, dass Claude vor jeder Bearbeitung fragt. Manchmal
möchten Sie einfach, dass es ausgeliefert wird. Sie sollten sich nicht
für immer für einen entscheiden müssen.

*Shift+Tab* durchläuft, wie viel Spielraum Claude bekommt: *Manual* (die
`default` Einstellungswert) fragt vor jeder Aktion, *acceptEdits* lässt
Dateibearbeitungen und häufige Dateisystem-Befehle durchfließen, während
immer noch vor anderen Shell-Befehlen überprüft wird, und *plan* schlägt
Änderungen zur Genehmigung vor, bevor etwas angefasst wird. Plan-Modus ist
der Vertrauensaufbauer, also beginnen Sie damit für alles, das mehrere
Dateien anfasst.

*Probieren Sie es jetzt:* Bei Ihrem nächsten Refactoring drücken Sie
Shift+Tab, bis Sie „plan" sehen, dann beschreiben Sie die Änderung. Sie
erhalten einen vollständigen Vorschlag, bevor eine einzelne Datei sich
bewegt.

📖 Berechtigungsmodi → https://code.claude.com/docs/de/permissions
```

**Checkpointing und `/rewind`**

```markdown theme={null}
⏪ *Tipp: Es gibt einen Rückgängig-Button für das ganze Gespräch*

Claude ist vor drei Zügen den falschen Weg gegangen und jetzt entwirren Sie
es? Sie müssen nicht vorwärts reparieren.

`/rewind` rollt zu einem früheren Punkt im Gespräch zurück, einschließlich
der Dateiänderungen, die Claude unterwegs gemacht hat. Checkpointing ist
automatisch; Sie richten nichts ein.

*Probieren Sie es jetzt:* Drücken Sie *Esc* zweimal, um das Rewind-Menü zu
öffnen, oder geben Sie `/rewind` ein. Wählen Sie den Punkt, bevor die Dinge
schiefgingen.

📖 Checkpointing → https://code.claude.com/docs/de/checkpointing
```

<h3 id="connect-your-tools">
  Verbinden Sie Ihre Tools
</h3>

**MCP-Konnektoren**

```markdown theme={null}
🔌 *Tipp: Lassen Sie Claude Ihren Issue-Tracker lesen, damit Sie keine
Tickets einfügen müssen*

Jira-Tickets ins Terminal kopieren und einfügen fühlt sich wie ein Schritt
zurück an. Das ist es.

Eine Konfigurationsdatei (`.mcp.json` in Ihrem Projekt-Root) verbindet Claude
mit GitHub, Jira, Linear oder welchem Tracker Sie auch verwenden. Dann
passieren „Was ist das Top-Priority-Issue, das mir zugewiesen ist?" und
„Gehen Sie voran und beheben Sie es" im selben Gespräch.

*Probieren Sie es jetzt:* Fragen Sie Claude „Richten Sie einen MCP-Konnektor
für [GitHub/Jira/Linear] in diesem Repo ein". Es wird die Konfiguration für
Sie schreiben.

📖 MCP-Konnektoren → https://code.claude.com/docs/de/mcp
```

<h3 id="automate-your-workflows">
  Automatisieren Sie Ihre Workflows
</h3>

**Skills**

```markdown theme={null}
⚡ *Tipp: Verwandeln Sie diesen Prompt, den Sie immer wieder eingeben, in
einen Befehl*

Diese Woche dreimal „Fasse zusammen, woran ich heute aus git log gearbeitet
habe, formatiere es für Standup" eingegeben? Das ist ein Slash-Befehl, der
darauf wartet.

Eine SKILL.md-Datei in `.claude/skills/<name>/` wird zu einem wiederverwendbaren
Prompt; geben Sie `/<name>` ein, um ihn auszuführen. Erstellen Sie einen,
wenn Sie zum zweiten Mal einen mehrstufigen Prompt eingeben, den Sie vorher
eingegeben haben. Einfachster Weg: Fragen Sie Claude, es für Sie zu machen.

*Probieren Sie es jetzt:* Geben Sie „Machen Sie mir einen /standup Skill, der
zusammenfasst, woran ich heute aus git log gearbeitet habe", dann führen Sie
morgen früh `/standup` aus.

📖 Skills → https://code.claude.com/docs/de/skills
```

**Hooks**

```markdown theme={null}
🔔 *Tipp: Erhalten Sie eine Benachrichtigung, wenn Ihr Refactoring fertig
ist*

Sitzen Sie an Ihrem Schreibtisch und beobachten Claude bei einer langen
Aufgabe? Sie haben bessere Dinge zu tun für diese acht Minuten.

Hooks sind Shell-Befehle, die bei Claude Code-Ereignissen ausgelöst werden.
Ein Stop-Hook, der eine Desktop-Benachrichtigung sendet, bedeutet, dass Sie
ein langes Refactoring starten, weggehen und eine Benachrichtigung erhalten
können, sobald es fertig ist.

*Probieren Sie es jetzt:* Fragen Sie Claude „Fügen Sie einen Stop-Hook hinzu,
der eine Desktop-Benachrichtigung sendet, wenn Sie fertig sind". Es wird das
Skript schreiben und es verdrahten.

📖 Hooks-Anleitung → https://code.claude.com/docs/de/hooks-guide
```

<h3 id="day-to-day-development">
  Alltägliche Entwicklung
</h3>

**Screenshots und Bilder**

```markdown theme={null}
📸 *Tipp: Hören Sie auf, den Fehler-Dialog zu beschreiben. Zeigen Sie ihn
einfach.*

„Es gibt eine rote Box, die etwas über eine Null-Referenz sagt und auf
Zeile 47-ish zeigt" eingeben? Screenshot es.

Ziehen Sie einen Screenshot direkt ins Terminal und Claude sieht ihn:
Fehler-Dialoge, UI-Mockups, Whiteboard-Fotos, Figma-Exporte. *Ctrl+V*
fügt aus der Zwischenablage ein (verwenden Sie auch Ctrl+V auf macOS, nicht
Cmd+V).

*Probieren Sie es jetzt:* Wenn das nächste Mal etwas Visuelles bricht,
machen Sie einen Screenshot und fügen Sie ihn direkt in den Prompt ein.
Geben Sie dann einfach „Was ist hier falsch?" ein.

📖 Mit Bildern arbeiten → https://code.claude.com/docs/de/common-workflows
```

**Git-Workflows**

```markdown theme={null}
🌿 *Tipp: Übergeben Sie die ganze Git-Zeremonie*

Die Behebung dauerte 5 Minuten. Die Commit-Nachricht, der Branch und die
PR-Beschreibung dauerten 15. Dieses Verhältnis ist falsch.

Claude handhabt den vollständigen Git-Flow: Commits mit konventionellen
Nachrichten, Branches, PRs mit ordentlichen Zusammenfassungen. Eine Anfrage:
„Behebe den Off-by-One, committe mit einer konventionellen Commit-Nachricht
und öffne einen PR." Überprüfen Sie die Arbeit von jemand anderem? Fügen Sie
die PR-URL ein und fragen Sie Claude, Sie durch den Diff zu führen.

*Probieren Sie es jetzt:* Nach Ihrer nächsten Behebung, statt zu Ihrem
Git-Client zu wechseln, geben Sie einfach „Committe dies mit einer guten
Nachricht und öffne einen PR" ein.

📖 Pull Requests erstellen → https://code.claude.com/docs/de/common-workflows
```

<h3 id="share-and-scale">
  Teilen und skalieren
</h3>

**Plugins**

```markdown theme={null}
📦 *Tipp: Jemand hat diesen Skill wahrscheinlich bereits erstellt*

Dabei, eine Stunde damit zu verbringen, einen `/deploy`-Befehl zu erstellen?
Überprüfen Sie, ob er bereits existiert.

Skills werden gebündelt und als Plugins geteilt. `/plugin` durchsucht, was
verfügbar ist, und installiert in einem Schritt. Fünf Minuten Durchsuchen
können eine Stunde Bauen sparen.

*Probieren Sie es jetzt:* Geben Sie `/plugin` ein und scrollen Sie durch.
Sie werden mindestens eine Sache finden, die Sie nicht wussten, dass Sie sie
wollten.

📖 Plugins → https://code.claude.com/docs/de/plugins
```

<h3 id="security-and-admin">
  Sicherheit und Admin
</h3>

**Sicherheitsarchitektur**

```markdown theme={null}
🔐 *Tipp: Die Antwort auf „Ist das sicher?" für das nächste Mal, wenn Sie
gefragt werden*

Jemand in Ihrem Team wird fragen „Warte, wo geht mein Code hin?"
Hier ist die Kurzversion, die Sie einfügen können.

Berechtigungsorientiert nach Design. Jede Dateibearbeitung, jeder
Shell-Befehl und jeder externe Aufruf wird durch Ihre Genehmigung
kontrolliert. Die CLI läuft in Ihrem Terminal und spricht direkt mit
Anthropics API, ohne dritte Server, und unterstützt optionales
Betriebssystem-Level-Sandboxing für Shell-Befehle. Unter unserem
Enterprise-Plan verwendet Anthropic Ihren Code oder Prompts nicht zum
Trainieren seiner Modelle.

*Probieren Sie es jetzt:* Speichern Sie diese zwei Links für das nächste
Mal, wenn die Frage kommt. Sie beantworten die meisten
Sicherheitsüberprüfungsfragen.

📖 https://code.claude.com/docs/de/security
📖 https://code.claude.com/docs/de/data-usage
```

**Best Practices**

```markdown theme={null}
✅ *Tipp: Die 4 Gewohnheiten, die „Ausprobiert einmal" von „Verwende es
täglich" trennen*

Die meisten Menschen, die Claude Code abbrechen, haben eine davon übersprungen.
Die meisten Menschen, die bleiben, haben alle vier in der ersten Woche gemacht.

  - Beginnen Sie im Plan-Modus für alles, das mehrere Dateien anfasst
  - Führen Sie /init früh aus; Kontext verstärkt sich
  - Überprüfen Sie Diffs vor dem Committen; Claude kann zuversichtlich falsch
    sein
  - Überprüfen Sie Änderungen, die kritische Pfade anfassen; behandeln Sie es
    wie einen scharfsinnigen Junior, nicht wie ein Orakel

*Probieren Sie es jetzt:* Wenn Sie nur eines oder zwei davon gemacht haben,
wählen Sie das aus, das Sie vermissen, und machen Sie es bei Ihrer nächsten
Aufgabe. Posten Sie, was sich in #claude-code geändert hat.

📖 Best Practices → https://code.claude.com/docs/de/best-practices
```

<h2 id="quick-reference">
  Schnellreferenz
</h2>

<h3 id="faq-responses">
  FAQ-Antworten
</h3>

Einzeilige Antworten für die Fragen, die Sie am häufigsten gestellt bekommen.

| Frage                                     | Antwort                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| „Funktioniert es in VS Code?"             | Ja. Es gibt eine VS Code-Erweiterung und ein JetBrains-Plugin mit denselben Features, eingebettet in Ihrem Editor. [VS Code →](/docs/de/vs-code)                                                                                                                                                                                                                                                                                                                                           |
| „Muss ich vorher etwas konfigurieren?"    | Nein. Installieren Sie, führen Sie dann `claude` in einem beliebigen Repo aus. Führen Sie `/init` einmal aus und Sie sind fertig. [Schnellstart →](/docs/de/quickstart)                                                                                                                                                                                                                                                                                                                    |
| „Wo geht mein Code hin?"                  | Die CLI läuft in Ihrem Terminal und sendet Kontext an Anthropics API für Inferenz, ohne dritte Server. Unter Ihrem Enterprise-Plan werden Ihr Code und Prompts nicht zum Trainieren von Modellen verwendet. [Datennutzung →](/docs/de/data-usage)                                                                                                                                                                                                                                          |
| „Kann es mein ganzes Repo sehen?"         | Es liest, worauf Sie ihm Zugriff geben. Dateilesevorgänge in Ihrem Arbeitsverzeichnis erfordern keine Aufforderung; Berechtigungsaufforderungen kontrollieren Bearbeitungen, Shell-Befehle und alles außerhalb dieses Verzeichnisses. Ein integrierter Satz von schreibgeschützten Shell-Befehlen wie `ls` und `cat` wird ohne Aufforderung ausgeführt; beschränken Sie ihn mit [Sandbox-`denyRead`-Regeln](/docs/de/sandboxing#filesystem-isolation). [Berechtigungen →](/docs/de/permissions) |
| „Wie unterscheidet sich das von Copilot?" | Copilot vervollständigt Zeilen. Claude Code ist ein Agent, der Dateien liest, Befehle ausführt und mehrdatei-Bearbeitungen macht. [Übersicht →](/docs/de/overview)                                                                                                                                                                                                                                                                                                                         |
| „Was sollte ich zuerst ausprobieren?"     | Ein Bug, den Sie aufgeschoben haben, weil er mühsam ist. „Der Test in \[Datei] ist flaky, finde heraus, warum." [Schnellstart →](/docs/de/quickstart)                                                                                                                                                                                                                                                                                                                                      |

<h3 id="prompt-templates">
  Prompt-Vorlagen
</h3>

Teilen Sie diese Starter-Prompts mit Engineers, die installiert haben, aber nicht sicher sind, was sie fragen sollen. Jeder ist so formuliert, wie er in einer echten Sitzung eingegeben würde; ersetzen Sie die eingeklammerten Teile durch Dateien aus Ihrem eigenen Repo.

| Aufgabe                        | Prompt                                                                                                |
| ------------------------------ | ----------------------------------------------------------------------------------------------------- |
| Beheben Sie einen Bug          | „Die Tests in \[Datei] schlagen fehl, finde heraus, warum und behebe es"                              |
| Verstehen Sie Code             | „Zeige mir, wie \[Modul] funktioniert, dann sag mir, wo der Einstiegspunkt ist"                       |
| Sicheres Refactoring           | „Refaktoriere \[Modul] zu \[Ziel], verwende Plan-Modus, damit ich zuerst überprüfen kann"             |
| Schreiben Sie Tests            | „Schreibe Tests für \[Datei], die die Edge Cases um \[Szenario] abdecken"                             |
| Überprüfen Sie vor dem Commit  | „Schau dir meinen funktionierenden Diff an und sag mir, was riskant aussieht"                         |
| Öffnen Sie einen PR            | „Behebe \[Issue], schreibe einen konventionellen Commit und öffne einen PR mit einer Zusammenfassung" |
| Erstellen Sie einen Skill      | „Machen Sie mir einen /ship Skill, der Tests und Lint vor dem Commit ausführt"                        |
| Debuggen Sie einen Stack Trace | „Hier ist der Stack Trace, finde die Grundursache, papiere es nicht einfach über"                     |

<Tip>
  Claude Code wird häufig aktualisiert. Überprüfen Sie versionsspezifische Details gegen die [Dokumentationsstartseite](/docs/de/overview), bevor Sie intern verteilen.
</Tip>
