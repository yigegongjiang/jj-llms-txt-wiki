> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Neuigkeiten

> Eine wöchentliche Zusammenfassung der bemerkenswertesten Claude Code-Funktionen mit Code-Snippets, Demos und Kontext, warum sie wichtig sind.

Die wöchentliche Entwickler-Zusammenfassung hebt die Funktionen hervor, die am ehesten ändern, wie Sie arbeiten. Jeder Eintrag enthält ausführbaren Code, eine kurze Demo und einen Link zur vollständigen Dokumentation. Für jeden Fehlerbehebung und kleinere Verbesserung siehe das [Changelog](/docs/de/changelog).

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
