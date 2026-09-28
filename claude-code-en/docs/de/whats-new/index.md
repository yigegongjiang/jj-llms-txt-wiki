> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Neuigkeiten

> Eine wöchentliche Zusammenfassung der bemerkenswertesten Claude Code-Funktionen mit Code-Snippets, Demos und Kontext, warum sie wichtig sind.

Die wöchentliche Entwickler-Zusammenfassung hebt die Funktionen hervor, die am ehesten ändern, wie Sie arbeiten. Jeder Eintrag enthält ausführbaren Code, eine kurze Demo und einen Link zur vollständigen Dokumentation. Für jeden Fehlerbehebung und kleinere Verbesserung siehe das [Changelog](/docs/en/changelog).

<Update label="Woche 37" description="7.–11. September 2026" tags={["v2.1.263–v2.1.269"]}>
  **`claude plugin eval`**: Führen Sie Ihr Plugin gegen eine Suite von Testfällen aus, bewerten Sie die Ergebnisse und vergleichen Sie sie mit einer Baseline ohne Plugin. `claude plugin eval init` entwirft die Fälle und Bewerter für Sie.

  Auch diese Woche: Pop any **Claude Code Desktop-Bereich** in sein eigenes Fenster und docken Sie es später wieder an; die **`maxEffortLevel`**-Einstellung begrenzt die Aufwandsstufe auf jedem Provider; und eine Seite, die **WebFetch** nicht innerhalb von fünf Minuten heruntergeladen hat, schlägt fehl, anstatt zu hängen.

  [Lesen Sie die Woche-37-Zusammenfassung →](/docs/de/whats-new/2026-w37)
</Update>

<Update label="Woche 36" description="31. August – 4. September 2026" tags={["v2.1.251–v2.1.261"]}>
  **Claude Fable 5.1**: verfügbar in Claude Code mit einem 1-Million-Token-Kontextfenster.

  Auch diese Woche: Auf Pro- und Max-Plänen funktioniert **Computernutzung in der Desktop-App** im Hintergrund auf macOS, während Sie weiterarbeiten; beim Fullscreen-Rendering öffnet **`/diff`** ein Live-Panel neben dem Gespräch, das sich aktualisiert, während Claude bearbeitet; und **`/skill-doctor`** zeigt, was jede Ihrer Skills im Kontext kostet und wie oft sie verwendet wird.

  [Lesen Sie die Woche-36-Zusammenfassung →](/docs/de/whats-new/2026-w36)
</Update>

<Update label="Woche 35" description="24.–28. August 2026" tags={["v2.1.240–v2.1.250"]}>
  **Terminal-Sitzungen in der Desktop-App fortsetzen**: Geben Sie `/resume` in das Claude Code Desktop-Eingabefeld ein, um eine beliebige Sitzung aufzugreifen, die Sie von der CLI aus gestartet haben, mit dem vollständigen Gespräch und Kontext intakt.

  Auch diese Woche: **Von Claude entworfenes Feedback** lässt Claude einen Feedbackbericht schreiben, wenn etwas in einer Sitzung schiefgeht, den Sie überprüfen und von `/feedback` aus senden; **`--restricted`** startet eine Sitzung ohne die Befehlsausführungs-Tools oder Ihre Benutzer- und Projekteinstellungen, für Evaluierungs-Harnesses auf gemeinsamen Maschinen; und die **`modelPicker`**-Einstellung steuert, welche Modelle der `/model`-Picker auflistet.

  [Lesen Sie die Woche-35-Zusammenfassung →](/docs/de/whats-new/2026-w35)
</Update>

<Update label="Woche 34" description="17.–21. August 2026" tags={["v2.1.234–v2.1.239"]}>
  **`/design`**: eine Forschungsvorschau, die Claudes Design-Artboard-Workflow in die CLI und Claude Code Desktop bringt, basierend auf Artifacts, sodass Claude bearbeitbare Artboards für Ihre Benutzeroberfläche entwirft und die von Ihnen ausgewählte implementiert.

  Auch diese Woche: Der integrierte **Concise-Ausgabestil** führt Claude mit dem Ergebnis an und überspringt die Präambel; jede Maschine, auf der `claude remote-control` läuft, wird als **Gerätekarte** auf Ihrem Telefon angezeigt, sodass Sie eine Sitzung darauf von der Code-Registerkarte aus starten können; und **`ANTHROPIC_DEFAULT_MODEL`** legt das Modell fest, mit dem neue Sitzungen beginnen.

  [Lesen Sie die Woche-34-Zusammenfassung →](/docs/de/whats-new/2026-w34)
</Update>

<Update label="Woche 33" description="10.–14. August 2026" tags={["v2.1.225–v2.1.233"]}>
  **Auto-Fortsetzen nach einem Nutzungslimit auf Desktop**: Wenn Sie Ihr Sitzungslimit in Claude Code Desktop erreichen, aktivieren Sie **Auto-Fortsetzen, wenn Limits zurückgesetzt werden** auf der Limit-Karte und die App wiederholt den unterbrochenen Durchgang, sobald das Limit zurückgesetzt wird.

  Auch diese Woche: **Fork-Modus** ist standardmäßig in interaktiven Sitzungen aktiviert, sodass Claude eine Nebenaufgabe an einen Subagenten übergeben kann, der das vollständige Gespräch erbt; **GitLab**-Merge-Request-URLs funktionieren mit `--worktree` und der `claude agents`-Ansicht, und Marktplätze klonen nackte `gitlab.com`-URLs; und das Eingeben von **`@`** in der Eingabeaufforderung erwähnt eine andere Claude-Sitzung nach Name.

  [Lesen Sie die Woche-33-Zusammenfassung →](/docs/de/whats-new/2026-w33)
</Update>

<Update label="Woche 32" description="3.–7. August 2026" tags={["v2.1.220–v2.1.224"]}>
  **Sitzungsübergreifendes Messaging**: Auf macOS und Linux können Ihre Claude Code-Sitzungen sich jetzt gegenseitig Nachrichten senden, sodass Claude eine Erkenntnis oder eine Entscheidung von einer Sitzung zu einer anderen übergibt, anstatt dass Sie sie erneut erklären.

  Auch diese Woche: **Selbst gehostete Umgebungen** führen Claude Code Cloud-Sitzungen auf einer Infrastruktur aus, die Ihre Organisation betreibt, in öffentlicher Beta auf Team- und Enterprise-Plänen; **Auto-Modus** wird zum Standard-Berechtigungsmodus für neue Sitzungen auf Pro-, Max- und Team-Plänen ab 14. August; und die **VS Code-Erweiterung** erhält Focus-Ansicht.

  [Lesen Sie die Woche-32-Zusammenfassung →](/docs/de/whats-new/2026-w32)
</Update>

<Update label="Woche 30" description="20.–24. Juli 2026" tags={["v2.1.214–v2.1.219"]}>
  **Claude Opus 5**: das neue Standard-Opus-Modell in Claude Code, mit einem 1-Million-Token-Kontextfenster und Fast-Modus bei \$10/\$50 pro MTok.

  Auch diese Woche: **Claude Code Desktop** öffnet einen iOS-Simulator-Bereich in öffentlicher Beta, sodass Claude Ihre App ausführen und durchklicken kann, während Sie zuschauen; das **Claude Security Plugin** führt einen Multi-Agent-Schwachstellenscan Ihrer Codebasis durch und verwandelt die von Ihnen ausgewählten Ergebnisse in Patches, die Sie selbst anwenden; und **`/code-review`** läuft als Hintergrund-Subagent.

  [Lesen Sie die Woche-30-Zusammenfassung →](/docs/de/whats-new/2026-w30)
</Update>

<Update label="Woche 29" description="13.–17. Juli 2026" tags={["v2.1.207–v2.1.212"]}>
  **Artifacts rufen Ihre MCP-Konnektoren auf**: Ein veröffentlichtes Artifact kann Live-Daten abrufen und Aktionen durch die eigenen MCP-Konnektoren jedes Betrachters durchführen, wenn dieser die Seite öffnet, und diese Woche fügt auch öffentliche Freigabelinks, Editor-Rollen auf Team und Enterprise und Artifacts hinzu, die aus Claude Tag-Sitzungen erstellt wurden.

  Auch diese Woche: **Bildschirmlesemodus** ersetzt die visuelle Terminal-Schnittstelle durch einfachen, linearen Text für Bildschirmleser wie VoiceOver und NVDA; **`/fork`** kopiert Ihr Gespräch in eine neue Hintergrund-Sitzung, während Sie weiterarbeiten; und **Auto-Modus** benötigt keine Opt-in-Variable mehr auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry.

  [Lesen Sie die Woche-29-Zusammenfassung →](/docs/de/whats-new/2026-w29)
</Update>

<Update label="Woche 28" description="6.–10. Juli 2026" tags={["v2.1.202–v2.1.206"]}>
  **In-App-Browser auf Desktop**: Claude Code auf dem Desktop erhält einen integrierten Browser, sodass Claude Dokumentationen, Designs oder andere Websites aufrufen und mit Seiten auf die gleiche Weise interagieren kann wie mit Ihren lokalen Dev-Server-Vorschauen.

  Auch diese Woche: **`/doctor`** ist eine vollständige Setup-Überprüfung, die Probleme diagnostiziert und beheben kann, mit `/checkup` als Alias; **Auto-Modus** blockiert Transkript-Manipulation und fragt vor `rm -rf` bei ungelösten Variablen; und **Agent-View-Zeilen** zeigen ein farbiges Statuswort und eine von einem Klassifizierer geschriebene Überschrift.

  [Lesen Sie die Woche-28-Zusammenfassung →](/docs/de/whats-new/2026-w28)
</Update>

<Update label="Woche 27" description="29. Juni – 3. Juli 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**: das neue Standardmodell für Pro-, Team Standard- und Enterprise-Abonnementplätze, mit erstklassiger Codierung und Tool-Nutzung zu Sonnet-Preisen, einem nativen 1-Million-Token-Kontextfenster und adaptivem Denken standardmäßig aktiviert.

  Auch diese Woche: **Claude in Chrome** ist allgemein verfügbar auf allen direkten Anthropic-Plänen; **Subagenten laufen standardmäßig im Hintergrund**, sodass Claude weiterarbeitet, während sie laufen; **Claude Desktop auf Linux** landet in Beta auf Ubuntu und Debian; und **`/radio`** stimmt sich auf Claude FM Lo-Fi-Radio ein.

  [Lesen Sie die Woche-27-Zusammenfassung →](/docs/de/whats-new/2026-w27)
</Update>

<Update label="Woche 26" description="22.–26. Juni 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**: Authentifizieren Sie einen konfigurierten MCP-Server von Ihrer Shell aus, anstatt das interaktive `/mcp`-Menü zu verwenden, und löschen Sie später seine gespeicherten Anmeldedaten mit `claude mcp logout`.

  Auch diese Woche: **Shell-Modus reagiert auf Befehlsausgabe** (`! npm test` erhält eine Erklärung ohne eine zweite Eingabeaufforderung); **`/rewind`** kann ein Gespräch von vor dem Ausführen von `/clear` fortsetzen; und **Hintergrund-Subagenten** zeigen Genehmigungsaufforderungen jetzt in der Hauptsitzung an, anstatt sie automatisch abzulehnen.

  [Lesen Sie die Woche-26-Zusammenfassung →](/docs/de/whats-new/2026-w26)
</Update>

<Update label="Woche 25" description="15.–19. Juni 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**: Verwandeln Sie die Ausgabe einer Sitzung in eine Live-, teilbare Seite auf claude.ai, die sich aktualisiert, während die Sitzung funktioniert, jetzt in Beta auf Team- und Enterprise-Plänen.

  Auch diese Woche: **Deny- und Ask-Regeln stimmen mit Tool-Parametern überein** mit `Tool(param:value)`, zum Beispiel `Agent(model:opus)`; **`/config key=value`** setzt jede Einstellung von der Eingabeaufforderung aus, im `-p`-Modus und von Remote Control; und **Auto-Modus blockiert destruktive Git-Befehle**, wenn Sie nicht gefragt haben, lokale Arbeit zu verwerfen.

  [Lesen Sie die Woche-25-Zusammenfassung →](/docs/de/whats-new/2026-w25)
</Update>

<Update label="Woche 24" description="8.–12. Juni 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**: Verschieben Sie die aktuelle Sitzung in ein neues Arbeitsverzeichnis mitten im Gespräch, ohne den Prompt-Cache neu zu erstellen.

  Auch diese Woche: **Sub-Agenten können ihre eigenen Sub-Agenten spawnen** (Hintergrund-Ketten sind auf fünf Ebenen begrenzt); **`--safe-mode`** startet Claude Code mit allen Anpassungen deaktiviert zur Fehlerbehebung; und **`fallbackModel`** konfiguriert bis zu drei Fallback-Modelle, die der Reihe nach versucht werden.

  [Lesen Sie die Woche-24-Zusammenfassung →](/docs/de/whats-new/2026-w24)
</Update>

<Update label="Woche 23" description="1.–5. Juni 2026" tags={["v2.1.158–v2.1.165"]}>
  **Auto-Modus auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry**: Auto-Modus ist jetzt auf Drittanbieter-Providern für Opus 4.7 und Opus 4.8 verfügbar und ersetzt Genehmigungsaufforderungen durch Hintergrund-Sicherheitsprüfungen.

  Auch diese Woche: **sicherere automatische Bearbeitungen** fordern auf, bevor Dateien geschrieben werden, die Code im `acceptEdits`-Modus ausführen können; **`/plugin list`** druckt Ihre installierten Plugins inline; und **Versionsanforderungen** ermöglichen es verwalteten Bereitstellungen, einen genehmigten Claude Code-Versionsbereich zu erfordern.

  [Lesen Sie die Woche-23-Zusammenfassung →](/docs/de/whats-new/2026-w23)
</Update>

<Update label="Woche 22" description="25.–29. Mai 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**: das neue Standardmodell für Max, Team Premium, Enterprise Pay-as-you-go und Anthropic API-Konten, mit hohem Aufwand standardmäßig und `/effort xhigh` für die schwierigsten Aufgaben.

  Auch diese Woche: **dynamische Workflows** orchestrieren Dutzende bis Hunderte von Subagenten aus einem Skript, das Claude schreibt; das **Security-Guidance-Plugin** überprüft Claudes Änderungen auf Sicherheitslücken während der Arbeit; und **Fast-Modus** läuft auf Opus 4.8 bei \$10/\$50 pro MTok.

  [Lesen Sie die Woche-22-Zusammenfassung →](/docs/de/whats-new/2026-w22)
</Update>

<Update label="Woche 21" description="18.–22. Mai 2026" tags={["v2.1.143–v2.1.149"]}>
  **Auto-Modus im Pro-Plan**: Auto-Modus läuft jetzt auf Pro-Konten und unterstützt Sonnet 4.6 neben Opus, ersetzt Genehmigungsaufforderungen durch Hintergrund-Sicherheitsprüfungen.

  Auch diese Woche: **`/usage`** schlüsselt auf, was Ihre Plan-Limits nach Skill, Subagent, Plugin und MCP-Server antreibt; der neue **`/code-review`**-Befehl meldet Korrektheitsfehler; und **Hintergrund-Sitzungen** erscheinen in `/resume` und bleiben aktiv, wenn sie angeheftet sind.

  [Lesen Sie die Woche-21-Zusammenfassung →](/docs/de/whats-new/2026-w21)
</Update>

<Update label="Woche 20" description="11.–15. Mai 2026" tags={["v2.1.139–v2.1.142"]}>
  **Agent-Ansicht**: `claude agents` öffnet einen Bildschirm für jede Claude Code-Sitzung und zeigt, was läuft, was auf Sie wartet und was erledigt ist.

  Auch diese Woche: **`/goal`** hält Claude über mehrere Durchläufe hinweg arbeiten, bis eine Abschlussbedingung erfüllt ist; **Fast-Modus** läuft jetzt standardmäßig auf Opus 4.7; und das **Rewind-Menü** kann früheren Kontext mit „Zusammenfassen bis hier" komprimieren.

  [Lesen Sie die Woche-20-Zusammenfassung →](/docs/de/whats-new/2026-w20)
</Update>

<Update label="Woche 19" description="4.–8. Mai 2026" tags={["v2.1.128–v2.1.136"]}>
  **Plugins laden aus `.zip`-Archiven und URLs**: `--plugin-dir` akzeptiert jetzt `.zip`-Dateien, und `--plugin-url` ruft ein Plugin-Archiv für die aktuelle Sitzung ab.

  Auch diese Woche: **`worktree.baseRef`** wählt, ob neue Worktrees vom Remote-Standard oder lokalen `HEAD` verzweigen; **Auto-Modus Hard-Deny-Regeln** blockieren Aktionen bedingungslos unabhängig von Allow-Ausnahmen; und **Hooks sehen die aktive Aufwandsstufe** über `effort.level` und `$CLAUDE_EFFORT`.

  [Lesen Sie die Woche-19-Zusammenfassung →](/docs/de/whats-new/2026-w19)
</Update>

<Update label="Woche 18" description="27. April – 1. Mai 2026" tags={["v2.1.120–v2.1.126"]}>
  **Windows ohne Git Bash**: Git für Windows ist nicht mehr erforderlich, und Claude Code verwendet PowerShell als Shell-Tool, wenn Bash nicht vorhanden ist.

  Auch diese Woche: **`claude ultrareview`** bringt Cloud-Code-Überprüfung zu CI und Skripten; **`claude project purge`** bereinigt den lokalen Status für ein Projekt; und das Einfügen einer **PR-URL in `/resume`** findet die Sitzung, die sie erstellt hat.

  [Lesen Sie die Woche-18-Zusammenfassung →](/docs/de/whats-new/2026-w18)
</Update>

<Update label="Woche 17" description="20.–24. April 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** öffnet sich als öffentliche Forschungsvorschau: Eine Flotte von Fehlersuche-Agenten läuft in der Cloud und die Ergebnisse landen automatisch in Ihrer CLI oder Desktop zurück.

  Auch diese Woche: **Sitzungsrückblick** zeigt Ihnen, was passiert ist, während ein Terminal nicht fokussiert war; **benutzerdefinierte Designs** ermöglichen es Ihnen, Farbpaletten von `/theme` oder einem Plugin zu erstellen und bereitzustellen; und **Claude Code im Web** erhält ein Redesign mit einer neuen Sitzungsseitenleiste und Drag-and-Drop-Layout.

  [Lesen Sie die Woche-17-Zusammenfassung →](/docs/de/whats-new/2026-w17)
</Update>

<Update label="Woche 16" description="13.–17. April 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** landet als neue Standardeinstellung auf Max und Team Premium, mit einer neuen `xhigh`-Aufwandsstufe, die die empfohlene Einstellung für die meisten Codierungsarbeiten ist, und einem interaktiven `/effort`-Schieberegler zum Einstellen.

  Auch diese Woche: **Routinen** auf Claude Code im Web starten vorlagengesteuerte Cloud-Agenten nach einem Zeitplan, GitHub-Ereignis oder API-Aufruf; **mobile Push-Benachrichtigungen** benachrichtigen Ihr Telefon, wenn eine lange Aufgabe abgeschlossen ist oder Claude Sie braucht; `/usage` zeigt, was Ihre Limits antreibt; und die CLI wechselt zu nativen Binärdateien.

  [Lesen Sie die Woche-16-Zusammenfassung →](/docs/de/whats-new/2026-w16)
</Update>

<Update label="Woche 15" description="6.–10. April 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** tritt in frühe Vorschau ein: Entwerfen Sie einen Plan in der Cloud von Ihrer CLI aus, überprüfen und kommentieren Sie ihn in einem Web-Editor, führen Sie ihn dann remote aus oder ziehen Sie ihn lokal zurück. Der erste Durchlauf erstellt jetzt automatisch eine Cloud-Umgebung für Sie.

  Auch diese Woche: Das **Monitor**-Tool streamt Hintergrund-Ereignisse in das Gespräch, damit Claude Protokolle verfolgen und live reagieren kann, `/loop` passt sich selbst an, wenn Sie das Intervall weglassen, `/team-onboarding` verpackt Ihr Setup in einen wiederholbaren Leitfaden, und `/autofix-pr` aktiviert PR-Autofix von Ihrem Terminal aus.

  [Lesen Sie die Woche-15-Zusammenfassung →](/docs/de/whats-new/2026-w15)
</Update>

<Update label="Woche 14" description="30. März – 3. April 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computernutzung** kommt zur CLI in Forschungsvorschau: Claude kann native Apps öffnen, durch die Benutzeroberfläche klicken und Änderungen von Ihrem Terminal aus überprüfen. Am besten zum Schließen der Schleife bei Dingen, die nur eine GUI überprüfen kann.

  Auch diese Woche: `/powerup` interaktive Lektionen, flimmerfreies Alt-Screen-Rendering, eine Pro-Tool-MCP-Ergebnisgröße-Überschreibung bis zu 500K und Plugin-Ausführbare auf dem `PATH` des Bash-Tools.

  [Lesen Sie die Woche-14-Zusammenfassung →](/docs/de/whats-new/2026-w14)
</Update>

<Update label="Woche 13" description="23.–27. März 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto-Modus** landet in Forschungsvorschau: Ein Klassifizierer verwaltet Ihre Genehmigungsaufforderungen, sodass sichere Aktionen ohne Unterbrechung ausgeführt werden und riskante blockiert werden. Der Mittelweg zwischen dem Genehmigen von allem und `--dangerously-skip-permissions`.

  Auch diese Woche: Computernutzung in der Desktop-App, PR-Autofix im Web, Transkriptsuche mit `/`, ein natives PowerShell-Tool für Windows und bedingte `if`-Hooks.

  [Lesen Sie die Woche-13-Zusammenfassung →](/docs/de/whats-new/2026-w13)
</Update>
