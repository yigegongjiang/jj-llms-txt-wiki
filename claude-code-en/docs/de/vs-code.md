> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code in VS Code verwenden

> Installieren und konfigurieren Sie die Claude Code-Erweiterung für VS Code. Erhalten Sie KI-Codierungshilfe mit Inline-Diffs, @-Erwähnungen, Planüberprüfung und Tastaturkürzeln.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="VS Code-Editor mit dem geöffneten Claude Code-Erweiterungspanel auf der rechten Seite, das ein Gespräch mit Claude zeigt" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

Die VS Code-Erweiterung bietet eine native grafische Benutzeroberfläche für Claude Code, die direkt in Ihre IDE integriert ist. Dies ist die empfohlene Methode, um Claude Code in VS Code zu verwenden.

Mit der Erweiterung können Sie Claudes Pläne überprüfen und bearbeiten, bevor Sie sie akzeptieren, Bearbeitungen automatisch akzeptieren, während sie vorgenommen werden, @-Erwähnungen für Dateien mit bestimmten Zeilenbereichen aus Ihrer Auswahl hinzufügen, auf Gesprächsverlauf zugreifen und mehrere Gespräche in separaten Registerkarten oder Fenstern öffnen.

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Stellen Sie vor der Installation sicher, dass Sie folgende Voraussetzungen erfüllen:

* VS Code 1.98.0 oder höher
* Ein Anthropic-Konto: Jedes bezahlte Claude-Abonnement (Pro, Max, Team oder Enterprise) oder ein Claude Console-Konto funktioniert, und es ist kein API-Schlüssel erforderlich. Sie [melden sich an](/docs/de/authentication#log-in-to-claude-code) mit diesem Konto an, wenn Sie die Erweiterung zum ersten Mal öffnen. Wenn Sie Claude über einen Drittanbieter wie Amazon Bedrock oder Google Cloud's Agent Platform nutzen, siehe [Drittanbieter verwenden](#use-third-party-providers) für Setupanweisungen.

<Tip>
  Die Erweiterung enthält eine eigene Kopie der CLI (Befehlszeilenschnittstelle) für das Chat-Panel. Um `claude` im integrierten Terminal von VS Code auszuführen, benötigen Sie auch die [eigenständige CLI-Installation](/docs/de/setup). Siehe [VS Code-Erweiterung vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) für Details.
</Tip>

<h2 id="install-the-extension">
  Erweiterung installieren
</h2>

Klicken Sie auf den Link für Ihre IDE, um direkt zu installieren:

* [Für VS Code installieren](vscode:extension/anthropic.claude-code)
* [Für Cursor installieren](cursor:extension/anthropic.claude-code)

Oder drücken Sie in VS Code `Cmd+Shift+X` (Mac) oder `Ctrl+Shift+X` (Windows/Linux), um die Ansicht „Erweiterungen" zu öffnen, suchen Sie nach „Claude Code" und klicken Sie auf **Installieren**.

Die Erweiterung wird auch in anderen VS Code-Forks wie Devin Desktop oder Kiro installiert. Suchen Sie nach „Claude Code" in der Ansicht „Erweiterungen" des Editors, oder installieren Sie aus der [Open VSX-Registrierung](https://open-vsx.org/extension/Anthropic/claude-code). Wenn Ihr Editor die Erweiterung nicht installieren kann, [installieren Sie die CLI](/docs/de/quickstart) und führen Sie `claude` in dessen integriertem Terminal aus. Die CLI funktioniert in jedem Terminal.

<Note>Wenn die Erweiterung nach der Installation nicht angezeigt wird, starten Sie VS Code neu oder führen Sie „Developer: Reload Window" aus der Befehlspalette aus.</Note>

<h2 id="get-started">
  Erste Schritte
</h2>

Nach der Installation können Sie Claude Code über die VS Code-Benutzeroberfläche verwenden:

<Steps>
  <Step title="Öffnen Sie das Claude Code-Panel">
    In VS Code zeigt das Spark-Symbol Claude Code an: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Spark-Symbol" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Die schnellste Möglichkeit, Claude zu öffnen, besteht darin, auf das Spark-Symbol in der **Editor-Symbolleiste** (obere rechte Ecke des Editors) zu klicken. Das Symbol wird nur angezeigt, wenn Sie eine Datei geöffnet haben.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="VS Code-Editor mit dem Spark-Symbol in der Editor-Symbolleiste" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Weitere Möglichkeiten, Claude Code zu öffnen:

    * **Aktivitätsleiste**: Klicken Sie auf das Spark-Symbol in der linken Seitenleiste, um die Sitzungsliste zu öffnen. Klicken Sie auf eine beliebige Sitzung, um sie als vollständige Editor-Registerkarte zu öffnen, oder starten Sie eine neue. Dieses Symbol ist immer in der Aktivitätsleiste sichtbar.
    * **Befehlspalette**: `Cmd+Shift+P` (Mac) oder `Ctrl+Shift+P` (Windows/Linux), geben Sie „Claude Code" ein und wählen Sie eine Option wie „In neuer Registerkarte öffnen"
    * **Statusleiste**: Klicken Sie auf **✱ Claude Code** in der unteren rechten Ecke des Fensters. Dies funktioniert auch, wenn keine Datei geöffnet ist.

    Sie können das Claude-Panel ziehen, um es überall in VS Code zu repositionieren. Siehe [Passen Sie Ihren Workflow an](#customize-your-workflow) für Details.
  </Step>

  <Step title="Melden Sie sich an">
    Wenn Sie das Panel zum ersten Mal öffnen, wird ein Anmeldungsbildschirm angezeigt. Klicken Sie auf **Anmelden** und schließen Sie die Autorisierung in Ihrem Browser ab.

    Wenn Sie später **Nicht angemeldet · Bitte führen Sie /login aus** sehen, öffnet die Erweiterung den Anmeldungsbildschirm automatisch erneut. Wenn er nicht angezeigt wird, laden Sie das Fenster aus der Befehlspalette mit **Developer: Reload Window** neu.

    Wenn Sie `ANTHROPIC_API_KEY` in Ihrer Shell gesetzt haben, aber immer noch die Anmeldungsaufforderung sehen, hat VS Code möglicherweise Ihre Shell-Umgebung nicht geerbt. Starten Sie VS Code von einem Terminal mit `code .` aus, damit es Ihre Umgebungsvariablen erbt, oder melden Sie sich stattdessen mit Ihrem Claude-Konto an.

    Nach der Anmeldung wird eine **Claude Code erlernen**-Checkliste angezeigt. Arbeiten Sie jedes Element durch, indem Sie auf **Zeig mir** klicken, oder schließen Sie es mit dem X. Um es später erneut zu öffnen, deaktivieren Sie **Onboarding ausblenden** in den VS Code-Einstellungen unter Erweiterungen → Claude Code.
  </Step>

  <Step title="Senden Sie eine Eingabeaufforderung">
    Bitten Sie Claude, Ihnen bei Ihrem Code oder Ihren Dateien zu helfen, sei es zum Erklären, wie etwas funktioniert, zum Debuggen eines Problems oder zum Vornehmen von Änderungen.

    <Tip>Claude sieht Ihren ausgewählten Text automatisch. Drücken Sie `Option+K` (Mac) / `Alt+K` (Windows/Linux), um auch eine @-Erwähnungsreferenz (wie `@file.ts#5-10`) in Ihre Eingabeaufforderung einzufügen.</Tip>

    Hier ist ein Beispiel für eine Frage zu einer bestimmten Zeile in einer Datei:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="VS Code-Editor mit den Zeilen 2-3, die in einer Python-Datei ausgewählt sind, und das Claude Code-Panel zeigt eine Frage zu diesen Zeilen mit einer @-Erwähnungsreferenz" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Überprüfen Sie Änderungen">
    Wenn Claude eine Datei bearbeiten möchte, zeigt es einen Vergleich der ursprünglichen und vorgeschlagenen Änderungen nebeneinander an und fordert dann die Genehmigung an. Sie können akzeptieren, ablehnen oder Claude sagen, was stattdessen zu tun ist. Wenn Sie den vorgeschlagenen Inhalt direkt in der Diff-Ansicht bearbeiten, bevor Sie akzeptieren, wird Claude mitgeteilt, dass Sie ihn geändert haben, sodass er nicht davon ausgeht, dass die Datei seinem ursprünglichen Vorschlag entspricht.

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code zeigt einen Diff von Claudes vorgeschlagenen Änderungen mit einer Genehmigungsaufforderung, die fragt, ob die Bearbeitung vorgenommen werden soll" width="3292" height="1876" data-path="images/vs-code-edits.png" />
  </Step>
</Steps>

Weitere Ideen, was Sie mit Claude Code tun können, finden Sie unter [Häufige Workflows](/docs/de/common-workflows).

<Tip>
  Führen Sie 'Claude Code: Open Walkthrough" aus der Befehlspalette aus, um eine geführte Tour durch die Grundlagen zu erhalten.
</Tip>

<h2 id="use-the-prompt-box">
  Verwenden Sie das Eingabefeld
</h2>

Das Eingabefeld unterstützt mehrere Funktionen:

* **Genehmigungsmodi**: Klicken Sie auf den Modusindikator am unteren Rand des Eingabefelds, um Modi zu wechseln, oder legen Sie den Standard in den VS Code-Einstellungen unter `claudeCode.initialPermissionMode` fest. Siehe [Genehmigungsmodi](/docs/de/permission-modes#switch-permission-modes) für jeden Modus, den der Indikator bietet.
  * **Manuell**: Claude fragt vor Dateibearbeitungen und den meisten Shell-Befehlen um Genehmigung.
  * **Plan**: Claude beschreibt, was es tun wird, und wartet auf Genehmigung, bevor es Änderungen vornimmt. VS Code öffnet den Plan automatisch als vollständiges Markdown-Dokument, in dem Sie Inline-Kommentare hinzufügen können, um Feedback zu geben, bevor Claude beginnt.
  * **Automatisch bearbeiten**: Claude nimmt Bearbeitungen vor, ohne zu fragen.
* **Befehlsmenü**: Klicken Sie auf `/` oder geben Sie `/` ein, um das Befehlsmenü zu öffnen. Zu den Optionen gehören das Anhängen von Dateien, das Wechseln von Modellen, das Umschalten von erweitertem Denken, das Anzeigen der Plannutzung (`/usage`) und das Starten einer [Remote Control](/docs/de/remote-control)-Sitzung (`/remote-control`). Der Abschnitt „Anpassen" bietet Zugriff auf MCP servers, hooks, memory, permissions und plugins. Elemente mit einem Terminal-Symbol werden im integrierten Terminal geöffnet.
  * Der Abschnitt „Einstellungen" enthält **Remote Control für alle Sitzungen aktivieren**, das [`remoteControlAtStartup`](/docs/de/settings#available-settings) setzt, sodass [jede neue interaktive Sitzung automatisch eine Verbindung zu Remote Control herstellt](/docs/de/remote-control#enable-remote-control-for-all-sessions). Erfordert Claude Code v2.1.203 oder später.
* **Kontextindikator**: Das Eingabefeld zeigt, wie viel von Claudes context window Sie verwenden. Claude komprimiert automatisch bei Bedarf, oder Sie können `/compact` manuell ausführen.
* **Erweitertes Denken**: Ermöglicht Claude, mehr Zeit für die Überlegung komplexer Probleme aufzuwenden. Aktivieren Sie es über das Befehlsmenü (`/`). Claudes Überlegungen werden im Gespräch als zusammengeklappte Blöcke angezeigt: Klicken Sie auf einen Block, um ihn zu lesen, oder drücken Sie `Ctrl+O`, um jeden Denkblock in der Sitzung zu erweitern oder zu reduzieren. Siehe [Erweitertes Denken](/docs/de/model-config#extended-thinking) für Details.
* **Mehrzeilige Eingabe**: Drücken Sie `Shift+Enter`, um eine neue Zeile hinzuzufügen, ohne zu senden. Dies funktioniert auch in der Freitexteingabe „Sonstiges" von Frage-Dialogen.

<h3 id="reference-files-and-folders">
  Referenzdateien und Ordner
</h3>

Verwenden Sie @-Erwähnungen, um Claude Kontext über bestimmte Dateien oder Ordner zu geben. Wenn Sie `@` gefolgt von einem Datei- oder Ordnernamen eingeben, liest Claude diesen Inhalt und kann Fragen dazu beantworten oder Änderungen daran vornehmen. Claude Code unterstützt Fuzzy Matching, sodass Sie Teilnamen eingeben können, um das zu finden, was Sie benötigen:

```text theme={null}
> Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
> What's in @src/components/ (include a trailing slash for folders)
```

Für große PDFs können Sie Claude bitten, bestimmte Seiten statt der gesamten Datei zu lesen: eine einzelne Seite, einen Bereich wie Seiten 1-10 oder einen offenen Bereich wie Seite 3 und darüber hinaus.

Wenn Sie Text im Editor auswählen, kann Claude Ihren hervorgehobenen Code automatisch sehen. Die Fußzeile des Eingabefelds zeigt, wie viele Zeilen ausgewählt sind. Drücken Sie `Option+K` (Mac) / `Alt+K` (Windows/Linux), um eine @-Erwähnung mit dem Dateipfad und den Zeilennummern einzufügen (z. B. `@app.ts#5-10`). Klicken Sie auf den Auswahlindikator, um umzuschalten, ob Claude Ihren hervorgehobenen Text sehen kann – das Symbol mit durchgestrichenem Auge bedeutet, dass die Auswahl vor Claude verborgen ist.

Sie können auch `Shift` gedrückt halten, während Sie Dateien in das Eingabefeld ziehen, um sie als Anhänge hinzuzufügen. Klicken Sie auf das X bei einem Anhang, um ihn aus dem Kontext zu entfernen.

<h3 id="resume-past-conversations">
  Frühere Gespräche fortsetzen
</h3>

Klicken Sie auf die Schaltfläche **Sitzungsverlauf** oben im Claude Code-Panel, um auf Ihren Gesprächsverlauf zuzugreifen. Sie können nach Schlüsselwort suchen oder nach Zeit durchsuchen (Heute, Gestern, Letzte 7 Tage usw.). Klicken Sie auf ein beliebiges Gespräch, um es mit dem vollständigen Nachrichtenverlauf fortzusetzen. Neue Sitzungen erhalten KI-generierte Titel basierend auf Ihrer ersten Nachricht. Bewegen Sie den Mauszeiger über eine Sitzung, um Umbenennungs- und Entfernungsaktionen anzuzeigen: Benennen Sie um, um ihr einen beschreibenden Titel zu geben, oder entfernen Sie sie, um sie aus der Liste zu löschen. Weitere Informationen zum Fortsetzen von Sitzungen finden Sie unter [Sitzungen verwalten](/docs/de/sessions).

<h3 id="resume-cloud-sessions-from-claude-ai">
  Fortsetzen von Remote-Sitzungen von Claude.ai
</h3>

Wenn Sie [Claude Code im Web](/docs/de/claude-code-on-the-web) verwenden, können Sie diese Remote-Sitzungen direkt in VS Code fortsetzen. Dies erfordert die Anmeldung mit **Claude.ai Subscription**, nicht Anthropic Console.

<Steps>
  <Step title="Öffnen Sie den Sitzungsverlauf">
    Klicken Sie auf die Schaltfläche **Sitzungsverlauf** oben im Claude Code-Panel.
  </Step>

  <Step title="Wählen Sie die Registerkarte Remote">
    Der Dialog zeigt zwei Registerkarten: Lokal und Remote. Klicken Sie auf **Remote**, um Sitzungen von claude.ai anzuzeigen.
  </Step>

  <Step title="Wählen Sie eine Sitzung zum Fortsetzen">
    Durchsuchen oder suchen Sie Ihre Remote-Sitzungen. Klicken Sie auf eine beliebige Sitzung, um sie herunterzuladen und das Gespräch lokal fortzusetzen.
  </Step>
</Steps>

<Note>
  Nur Web-Sitzungen, die mit einem GitHub-Repository gestartet wurden, werden auf der Registerkarte Remote angezeigt. Das Fortsetzen lädt den Gesprächsverlauf lokal; Änderungen werden nicht mit claude.ai synchronisiert.
</Note>

<h3 id="check-account-and-usage">
  Konto und Nutzung überprüfen
</h3>

Führen Sie `/usage` aus dem Befehlsmenü aus, um das Dialogfeld „Konto & Nutzung" zu öffnen. Es zeigt Ihr angemeldetes Konto, Ihren Plan und Nutzungsbalken für die aktuelle Sitzung und Woche mit der verbleibenden Zeit bis zum Zurücksetzen jedes Limits.

Das Dialogfeld zeigt auch auf, was zu Ihren Planlimits beiträgt. Es kennzeichnet Verhaltensweisen, die 10 % oder mehr der letzten Nutzung ausmachen, wie z. B. Cache-Misses, langer Kontext und Subagent-intensive oder hochgradig parallele Sitzungen, jeweils mit einem Tipp zur Reduzierung. Attributionstabellen zeigen, wie viel Nutzung von jedem Skill, Subagent, Plugin und MCP server kam. Erfordert Claude Code v2.1.174 oder später.

Verwenden Sie den Umschalter „Tag" und „Woche", um zwischen den letzten 24 Stunden und den letzten 7 Tagen zu wechseln. Die Zahlen sind ungefähr und werden aus lokalen Sitzungen auf diesem Computer berechnet, daher ist die Nutzung von anderen Geräten oder claude.ai nicht enthalten. Weitere Informationen zum Verfolgen und Reduzieren der Nutzung finden Sie unter [Verfolgen Sie Ihre Kosten](/docs/de/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Passen Sie Ihren Workflow an
</h2>

Sobald Sie einsatzbereit sind, können Sie das Claude-Panel repositionieren, mehrere Sitzungen ausführen oder zum Terminal-Modus wechseln.

<h3 id="choose-where-claude-lives">
  Wählen Sie, wo Claude lebt
</h3>

Sie können das Claude-Panel ziehen, um es überall in VS Code zu repositionieren. Greifen Sie die Registerkarte oder Titelleiste des Panels und ziehen Sie es zu:

* **Sekundäre Seitenleiste**: die rechte Seite des Fensters. Hält Claude sichtbar, während Sie codieren.
* **Primäre Seitenleiste**: die linke Seitenleiste mit Symbolen für Explorer, Suche usw.
* **Editor-Bereich**: öffnet Claude als Registerkarte neben Ihren Dateien. Nützlich für Nebenaufgaben.

<Tip>
  Verwenden Sie die Seitenleiste für Ihre Haupt-Claude-Sitzung und öffnen Sie zusätzliche Registerkarten für Nebenaufgaben. Claude merkt sich Ihren bevorzugten Ort. Das Symbol der Sitzungsliste in der Aktivitätsleiste ist separat vom Claude-Panel: Die Sitzungsliste ist immer in der Aktivitätsleiste sichtbar, während das Claude-Panel-Symbol nur dort angezeigt wird, wenn das Panel an der linken Seitenleiste angedockt ist.
</Tip>

<h3 id="run-multiple-conversations">
  Führen Sie mehrere Gespräche aus
</h3>

Verwenden Sie **In neuer Registerkarte öffnen** oder **In neuem Fenster öffnen** aus der Befehlspalette, um zusätzliche Gespräche zu starten. Jedes Gespräch behält seinen eigenen Verlauf und Kontext bei, sodass Sie parallel an verschiedenen Aufgaben arbeiten können.

Bei Verwendung von Registerkarten zeigt ein kleiner farbiger Punkt auf dem Spark-Symbol den Status an: Blau bedeutet, dass eine Genehmigungsanfrage ausstehend ist, Orange bedeutet, dass Claude fertig ist, während die Registerkarte verborgen war.

<h3 id="switch-to-terminal-mode">
  Wechseln Sie zum Terminal-Modus
</h3>

Standardmäßig öffnet die Erweiterung ein grafisches Chat-Panel. Wenn Sie die CLI-ähnliche Benutzeroberfläche bevorzugen, öffnen Sie die [Einstellung 'Terminal verwenden"](vscode://settings/claudeCode.useTerminal) und aktivieren Sie das Kontrollkästchen.

Sie können auch VS Code-Einstellungen öffnen (`Cmd+,` auf Mac oder `Ctrl+,` auf Windows/Linux), zu Erweiterungen → Claude Code gehen und **Terminal verwenden** aktivieren.

<h2 id="manage-plugins">
  Verwalten Sie Plugins
</h2>

Die VS Code-Erweiterung enthält eine grafische Benutzeroberfläche zum Installieren und Verwalten von [plugins](/docs/de/plugins). Geben Sie `/plugins` in das Eingabefeld ein, um die Benutzeroberfläche **Plugins verwalten** zu öffnen.

<h3 id="install-plugins">
  Installieren Sie Plugins
</h3>

Der Plugin-Dialog zeigt zwei Registerkarten: **Plugins** und **Marketplaces**.

Auf der Registerkarte „Plugins":

* **Installierte Plugins** werden oben mit Umschaltern angezeigt, um sie zu aktivieren oder zu deaktivieren
* **Verfügbare Plugins** aus Ihren konfigurierten Marketplaces werden unten angezeigt
* Suchen Sie, um Plugins nach Name oder Beschreibung zu filtern
* Klicken Sie auf **Installieren** bei einem beliebigen verfügbaren Plugin

Wenn Sie ein Plugin installieren, wählen Sie den Installationsumfang:

* **Für Sie installieren**: verfügbar in allen Ihren Projekten (Benutzerumfang)
* **Für dieses Projekt installieren**: geteilt mit Projektmitarbeitern (Projektumfang)
* **Lokal installieren**: nur für Sie, nur in diesem Repository (lokaler Umfang)

<h3 id="manage-marketplaces">
  Verwalten Sie Marketplaces
</h3>

Wechseln Sie zur Registerkarte **Marketplaces**, um Plugin-Quellen hinzuzufügen oder zu entfernen:

* Geben Sie ein GitHub-Repo, eine URL oder einen lokalen Pfad ein, um einen neuen Marketplace hinzuzufügen
* Klicken Sie auf das Aktualisierungssymbol, um die Plugin-Liste eines Marketplace zu aktualisieren
* Klicken Sie auf das Papierkorbsymbol, um einen Marketplace zu entfernen

Nach Änderungen fordert ein Banner Sie auf, Claude Code neu zu starten, um die Aktualisierungen anzuwenden.

<Note>
  Die Plugin-Verwaltung in VS Code verwendet unter der Haube die gleichen CLI-Befehle. Plugins und Marketplaces, die Sie in der Erweiterung konfigurieren, sind auch in der CLI verfügbar, und umgekehrt.
</Note>

Weitere Informationen zum Plugin-System finden Sie unter [Plugins](/docs/de/plugins) und [Plugin-Marketplaces](/docs/de/plugin-marketplaces).

<h2 id="automate-browser-tasks-with-chrome">
  Automatisieren Sie Browser-Aufgaben mit Chrome
</h2>

Verbinden Sie Claude mit Ihrem Chrome-Browser, um Web-Apps zu testen, mit Konsolenprotokollen zu debuggen und Browser-Workflows zu automatisieren, ohne VS Code zu verlassen. Dies erfordert die [Claude in Chrome-Erweiterung](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) Version 1.0.36 oder höher.

Geben Sie `@browser` in das Eingabefeld ein, gefolgt von dem, was Claude tun soll:

```text theme={null}
@browser go to localhost:3000 and check the console for errors
```

Sie können auch das Anhang-Menü öffnen, um bestimmte Browser-Tools wie das Öffnen einer neuen Registerkarte oder das Lesen von Seiteninhalten auszuwählen.

Claude öffnet neue Registerkarten für Browser-Aufgaben und teilt den Anmeldestatus Ihres Browsers, sodass es auf jede Website zugreifen kann, bei der Sie bereits angemeldet sind.

Anweisungen zum Einrichten, die vollständige Liste der Funktionen und Fehlerbehebung finden Sie unter [Claude Code mit Chrome verwenden](/docs/de/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  VS Code-Befehle und Tastaturkürzel
</h2>

Öffnen Sie die Befehlspalette (`Cmd+Shift+P` auf Mac oder `Ctrl+Shift+P` auf Windows/Linux) und geben Sie „Claude Code" ein, um alle verfügbaren VS Code-Befehle für die Claude Code-Erweiterung anzuzeigen.

Einige Tastaturkürzel hängen davon ab, welches Panel „fokussiert" ist (Tastatureingaben empfängt). Wenn sich Ihr Cursor in einer Codedatei befindet, ist der Editor fokussiert. Wenn sich Ihr Cursor im Eingabefeld von Claude befindet, ist Claude fokussiert. Verwenden Sie `Cmd+Esc` / `Ctrl+Esc`, um zwischen ihnen zu wechseln.

<Note>
  Dies sind VS Code-Befehle zum Steuern der Erweiterung. Nicht alle integrierten Claude Code-Befehle sind in der Erweiterung verfügbar. Siehe [VS Code-Erweiterung vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) für Details.
</Note>

| Befehl                        | Tastaturkürzel                                           | Beschreibung                                                                                                                                                                                                                                                              |
| ----------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                   | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Fokus zwischen Editor und Claude umschalten                                                                                                                                                                                                                               |
| In Seitenleiste öffnen        | -                                                        | Öffnen Sie Claude in der linken Seitenleiste                                                                                                                                                                                                                              |
| Im Terminal öffnen            | -                                                        | Öffnen Sie Claude im Terminal-Modus                                                                                                                                                                                                                                       |
| In neuer Registerkarte öffnen | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Öffnen Sie ein neues Gespräch als Editor-Registerkarte                                                                                                                                                                                                                    |
| In neuem Fenster öffnen       | -                                                        | Öffnen Sie ein neues Gespräch in einem separaten Fenster                                                                                                                                                                                                                  |
| Neues Gespräch                | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Starten Sie ein neues Gespräch. Erfordert, dass Claude fokussiert ist und `enableNewConversationShortcut` auf `true` gesetzt ist                                                                                                                                          |
| Reopen Closed Session         | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Öffnen Sie die zuletzt geschlossene Claude-Sitzungsregisterkarte erneut. Fällt auf VS Codes normales Öffnen geschlossener Editoren zurück, wenn die zuletzt geschlossene Registerkarte keine Claude-Sitzung war. Deaktivieren Sie mit `enableReopenClosedSessionShortcut` |
| Insert @-Mention Reference    | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Fügen Sie eine Referenz zur aktuellen Datei und Auswahl ein (erfordert, dass der Editor fokussiert ist)                                                                                                                                                                   |
| Protokolle anzeigen           | -                                                        | Anzeigen von Erweiterungs-Debug-Protokollen                                                                                                                                                                                                                               |
| Abmelden                      | -                                                        | Melden Sie sich von Ihrem Anthropic-Konto ab                                                                                                                                                                                                                              |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Starten Sie eine VS Code-Registerkarte von anderen Tools aus
</h3>

Die Erweiterung registriert einen URI-Handler unter `vscode://anthropic.claude-code/open`. Verwenden Sie ihn, um eine neue Claude Code-Registerkarte von Ihrem eigenen Tooling aus zu öffnen: ein Shell-Alias, ein Browser-Lesezeichen oder ein beliebiges Skript, das eine URL öffnen kann. Wenn VS Code nicht bereits ausgeführt wird, wird es beim Öffnen der URL zuerst gestartet. Wenn VS Code bereits ausgeführt wird, wird die URL in dem Fenster geöffnet, das derzeit fokussiert ist.

Rufen Sie den Handler mit dem URL-Opener Ihres Betriebssystems auf.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Windows">
    In PowerShell:

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    In `cmd.exe` behandelt `start` sein erstes Argument in Anführungszeichen als Fenstertitel, daher übergeben Sie einen leeren Titel vor der URL:

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

Der Handler akzeptiert zwei optionale Abfrageparameter:

| Parameter | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Text zum Vorausfüllen im Eingabefeld. Muss URL-codiert sein. Das Eingabefeld wird vorausgefüllt, aber nicht automatisch gesendet.                                                                                                                                                                                                                                                                                                                                 |
| `session` | Eine Sitzungs-ID zum Fortsetzen statt zum Starten eines neuen Gesprächs. Die Sitzung muss zum derzeit in VS Code geöffneten Arbeitsbereich gehören. Wenn die Sitzung nicht gefunden wird, wird stattdessen ein neues Gespräch gestartet. Wenn die Sitzung bereits in einer Registerkarte geöffnet ist, wird diese Registerkarte fokussiert. Um eine Sitzungs-ID programmgesteuert zu erfassen, siehe [Gespräche fortsetzen](/docs/de/headless#continue-conversations). |

Um beispielsweise eine Registerkarte mit „review my changes" vorausgefüllt zu öffnen:

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

Um stattdessen eine Terminal-Sitzung zu starten, verwenden Sie den CLI-Handler `claude-cli://`. Siehe [Sitzungen von Links aus starten](/docs/de/deep-links).

<h2 id="configure-settings">
  Einstellungen konfigurieren
</h2>

Die Erweiterung hat zwei Arten von Einstellungen:

* **Erweiterungseinstellungen** in VS Code: Steuern Sie das Verhalten der Erweiterung in VS Code. Öffnen Sie mit `Cmd+,` (Mac) oder `Ctrl+,` (Windows/Linux), dann gehen Sie zu Erweiterungen → Claude Code. Sie können auch `/` eingeben und **General Config** auswählen, um Einstellungen zu öffnen.
* **Claude Code-Einstellungen** in `~/.claude/settings.json`: geteilt zwischen der Erweiterung und der CLI. Verwenden Sie für zulässige Befehle, Umgebungsvariablen, hooks und MCP servers. Siehe [Einstellungen](/docs/de/settings) für Details.

<Tip>
  Fügen Sie `"$schema": "https://json.schemastore.org/claude-code-settings.json"` zu Ihrer `settings.json` hinzu, um Autovervollständigung und Inline-Validierung für alle verfügbaren Einstellungen direkt in VS Code zu erhalten.
</Tip>

<h3 id="extension-settings">
  Erweiterungseinstellungen
</h3>

| Einstellung                         | Standard  | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ----------------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false`   | Starten Sie Claude im Terminal-Modus statt im grafischen Panel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `initialPermissionMode`             | `default` | Steuert Genehmigungsaufforderungen für neue Gespräche: `default`, `plan`, `acceptEdits` oder `bypassPermissions`. `manual` ist ein Alias für `default` und wählt den Modus mit der Bezeichnung **Manual** im Modusindikator. Erfordert Claude Code v2.1.200 oder später. Siehe [Genehmigungsmodi](/docs/de/permission-modes).                                                                                                                                                                                                                          |
| `preferredLocation`                 | `panel`   | Wo Claude öffnet: `sidebar` (rechts) oder `panel` (neue Registerkarte)                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `autosave`                          | `true`    | Speichern Sie Dateien automatisch, bevor Claude sie liest oder schreibt                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `useCtrlEnterToSend`                | `false`   | Verwenden Sie Ctrl/Cmd+Enter statt Enter, um Eingabeaufforderungen zu senden                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `enableNewConversationShortcut`     | `false`   | Aktivieren Sie Cmd/Ctrl+N, um ein neues Gespräch zu starten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `enableReopenClosedSessionShortcut` | `true`    | Verwenden Sie Cmd/Ctrl+Shift+T, um die zuletzt geschlossene Claude-Sitzungsregisterkarte erneut zu öffnen. Wenn die zuletzt geschlossene Registerkarte keine Claude-Sitzung war, führt die Tastenkombination stattdessen den normalen Befehl zum erneuten Öffnen des geschlossenen Editors von VS Code aus.                                                                                                                                                                                                                                       |
| `hideOnboarding`                    | `false`   | Blenden Sie die Onboarding-Checkliste aus (Abschlusskappe-Symbol)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `respectGitIgnore`                  | `true`    | Schließen Sie .gitignore-Muster aus Dateisuchvorgängen aus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `usePythonEnvironment`              | `true`    | Aktivieren Sie die Python-Umgebung des Arbeitsbereichs beim Ausführen von Claude. Erfordert die Python-Erweiterung.                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `environmentVariables`              | `[]`      | Legen Sie Umgebungsvariablen für den Claude-Prozess fest. Verwenden Sie stattdessen Claude Code-Einstellungen für gemeinsame Konfiguration.                                                                                                                                                                                                                                                                                                                                                                                                       |
| `disableLoginPrompt`                | `false`   | Überspringen Sie Authentifizierungsaufforderungen (für Setups von Drittanbietern)                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowDangerouslySkipPermissions`   | `false`   | Fügt Bypass-Berechtigungen zum Moduswahlschalter hinzu. Verwenden Sie es nur in Sandboxes ohne Internetzugang.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `claudeProcessWrapper`              | -         | Ausführbare Datei, die zum Starten des Claude-Prozesses verwendet wird. Der Pfad der gebündelten Binärdatei wird als Argument übergeben, wenn vorhanden. Setzen Sie dies auf eine separat installierte `claude`-Binärdatei, wenn der Erweiterungsbuild keine für Ihre Plattform enthält. Ein Fehler „Unsupported platform" bei der Aktivierung bedeutet, dass keine Binärdatei für Ihre Plattform gebündelt ist; siehe [welche Plattformen vorgefertigte Binärdateien haben](/docs/de/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="vs-code-extension-vs-claude-code-cli">
  VS Code-Erweiterung vs. Claude Code CLI
</h2>

Claude Code ist sowohl als VS Code-Erweiterung (grafisches Panel) als auch als CLI (Befehlszeilenschnittstelle im Terminal) verfügbar. Einige Funktionen sind nur in der CLI verfügbar. Wenn Sie eine CLI-only-Funktion benötigen, führen Sie `claude` im integrierten Terminal von VS Code aus. Dies erfordert die [eigenständige CLI-Installation](/docs/de/setup): Die Erweiterung fügt `claude` nicht zu Ihrem PATH hinzu. Siehe [CLI in VS Code ausführen](#run-cli-in-vs-code).

| Funktion                 | CLI                  | VS Code-Erweiterung                                                                                   |
| ------------------------ | -------------------- | ----------------------------------------------------------------------------------------------------- |
| Befehle und Skills       | [Alle](/docs/de/commands) | Teilmenge (geben Sie `/` ein, um verfügbare anzuzeigen)                                               |
| MCP-Server-Konfiguration | Ja                   | Teilweise (fügen Sie Server über CLI hinzu; verwalten Sie vorhandene Server mit `/mcp` im Chat-Panel) |
| Checkpoints              | Ja                   | Ja                                                                                                    |
| `!` Bash-Tastaturkürzel  | Ja                   | Nein                                                                                                  |
| Tab-Vervollständigung    | Ja                   | Nein                                                                                                  |

<h3 id="rewind-with-checkpoints">
  Zurückspulen mit Checkpoints
</h3>

Die VS Code-Erweiterung unterstützt Checkpoints, die Claudes Dateibearbeitungen verfolgen und es Ihnen ermöglichen, zu einem vorherigen Zustand zurückzuspulen. Bewegen Sie den Mauszeiger über eine beliebige Nachricht, um die Schaltfläche zum Zurückspulen anzuzeigen, und wählen Sie dann aus drei Optionen:

* **Gespräch von hier aus verzweigen**: Starten Sie einen neuen Gesprächszweig aus dieser Nachricht, während Sie alle Codeänderungen intakt halten
* **Code hier zurückspulen**: Revert-Dateiänderungen zu diesem Punkt im Gespräch, während Sie den vollständigen Gesprächsverlauf behalten
* **Gespräch verzweigen und Code zurückspulen**: Starten Sie einen neuen Gesprächszweig und revert-Dateiänderungen zu diesem Punkt

Vollständige Details zur Funktionsweise von Checkpoints und deren Einschränkungen finden Sie unter [Checkpointing](/docs/de/checkpointing).

<h3 id="run-cli-in-vs-code">
  CLI in VS Code ausführen
</h3>

Um die CLI zu verwenden und in VS Code zu bleiben, öffnen Sie das integrierte Terminal (`` Ctrl+` `` auf Windows/Linux oder `` Cmd+` `` auf Mac) und führen Sie `claude` aus. Die CLI wird automatisch mit Ihrer IDE für Funktionen wie Diff-Anzeige und Diagnosefreigabe integriert.

Die Installation der Erweiterung fügt `claude` nicht zu Ihrem Shell-PATH hinzu. Die Erweiterung enthält eine private Kopie der CLI für ihr Chat-Panel, aber die Eingabe von `claude` in einem Terminal erfordert die [eigenständige CLI-Installation](/docs/de/setup). Führen Sie die Installation einmal aus und die Befehle auf dieser Seite, einschließlich `claude mcp add` und `claude --resume`, funktionieren in jedem Terminal. Wenn `claude` nach der Installation immer noch nicht gefunden wird, [überprüfen Sie Ihren PATH](/docs/de/troubleshoot-install#verify-your-path).

Wenn Sie ein externes Terminal verwenden, führen Sie `/ide` in Claude Code aus, um es mit VS Code zu verbinden.

<h3 id="switch-between-extension-and-cli">
  Wechseln Sie zwischen Erweiterung und CLI
</h3>

Die Erweiterung und die CLI teilen den gleichen Gesprächsverlauf. Um ein Erweiterungsgespräch in der CLI fortzusetzen, führen Sie `claude --resume` im Terminal aus. Dies öffnet eine interaktive Auswahl, in der Sie Ihr Gespräch suchen und auswählen können.

<h3 id="include-terminal-output-in-prompts">
  Beziehen Sie Terminal-Ausgabe in Eingabeaufforderungen ein
</h3>

Referenzieren Sie Terminal-Ausgabe in Ihren Eingabeaufforderungen mit `@terminal:name`, wobei `name` der Titel des Terminals ist. Dies ermöglicht Claude, Befehlsausgabe, Fehlermeldungen oder Protokolle zu sehen, ohne zu kopieren und einzufügen.

<h3 id="monitor-background-processes">
  Überwachen Sie Hintergrundprozesse
</h3>

Wenn Claude lange laufende Befehle ausführt, zeigt die Erweiterung den Fortschritt in der Statusleiste an. Die Sichtbarkeit für Hintergrundaufgaben ist jedoch im Vergleich zur CLI begrenzt. Für bessere Sichtbarkeit lassen Sie Claude den Befehl ausgeben, damit Sie ihn im integrierten Terminal von VS Code ausführen können.

<h3 id="connect-to-external-tools-with-mcp">
  Verbinden Sie sich mit externen Tools mit MCP
</h3>

MCP (Model Context Protocol) Server geben Claude Zugriff auf externe Tools, Datenbanken und APIs.

Um einen MCP-Server hinzuzufügen, öffnen Sie das integrierte Terminal (`` Ctrl+` `` oder `` Cmd+` ``) und führen Sie `claude mcp add` aus. Das folgende Beispiel fügt Githubs Remote-MCP-Server hinzu, der sich mit einem [persönlichen Zugangstoken](https://github.com/settings/personal-access-tokens) authentifiziert, das als Header übergeben wird:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Nach der Konfiguration bitten Sie Claude, die Tools zu verwenden (z. B. „Review PR #456").

Um MCP-Server zu verwalten, ohne VS Code zu verlassen, geben Sie `/mcp` in das Chat-Panel ein. Der MCP-Verwaltungsdialog ermöglicht es Ihnen, Server zu aktivieren oder zu deaktivieren, sich erneut mit einem Server zu verbinden und OAuth-Authentifizierung zu verwalten. Siehe die [MCP-Dokumentation](/docs/de/mcp) für verfügbare Server.

<h2 id="work-with-git">
  Arbeiten Sie mit Git
</h2>

Claude Code wird mit Git integriert, um direkt in VS Code bei Versionskontroll-Workflows zu helfen. Bitten Sie Claude, Änderungen zu committen, Pull Requests zu erstellen oder über Branches zu arbeiten.

<h3 id="create-commits-and-pull-requests">
  Erstellen Sie Commits und Pull Requests
</h3>

Claude kann Änderungen bereitstellen, Commit-Nachrichten schreiben und Pull Requests basierend auf Ihrer Arbeit erstellen:

```text theme={null}
> commit my changes with a descriptive message
> create a pr for this feature
> summarize the changes I've made to the auth module
```

Beim Erstellen von Pull Requests generiert Claude Beschreibungen basierend auf den tatsächlichen Codeänderungen und kann Kontext über Tests oder Implementierungsentscheidungen hinzufügen.

<h3 id="use-git-worktrees-for-parallel-tasks">
  Verwenden Sie Git worktrees für parallele Aufgaben
</h3>

Verwenden Sie das Flag `--worktree` (`-w`), um Claude in einem isolierten worktree mit seinen eigenen Dateien und Branch zu starten:

```bash theme={null}
claude --worktree feature-auth
```

Jeder worktree behält einen unabhängigen Dateizustand bei, während er die Git-Historie teilt. Dies verhindert, dass Claude-Instanzen sich gegenseitig beeinflussen, wenn sie an verschiedenen Aufgaben arbeiten. Weitere Details finden Sie unter [Führen Sie parallele Sitzungen mit Git worktrees aus](/docs/de/worktrees).

<h2 id="use-third-party-providers">
  Verwenden Sie Drittanbieter
</h2>

Standardmäßig verbindet sich Claude Code direkt mit der API von Anthropic. Wenn Ihre Organisation Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verwendet, um auf Claude zuzugreifen, konfigurieren Sie die Erweiterung, um stattdessen Ihren Anbieter zu verwenden:

<Steps>
  <Step title="Deaktivieren Sie die Anmeldungsaufforderung">
    Öffnen Sie die [Einstellung 'Anmeldungsaufforderung deaktivieren'](vscode://settings/claudeCode.disableLoginPrompt) und aktivieren Sie das Kontrollkästchen.

    Sie können auch VS Code-Einstellungen öffnen (`Cmd+,` auf Mac oder `Ctrl+,` auf Windows/Linux), nach 'Claude Code login" suchen und **Anmeldungsaufforderung deaktivieren** aktivieren.
  </Step>

  <Step title="Konfigurieren Sie Ihren Anbieter">
    Folgen Sie dem Setup-Leitfaden für Ihren Anbieter:

    * [Claude Code auf Amazon Bedrock](/docs/de/amazon-bedrock)
    * [Claude Code auf Google Cloud's Agent Platform](/docs/de/google-vertex-ai)
    * [Claude Code auf Microsoft Foundry](/docs/de/microsoft-foundry)

    Diese Leitfäden behandeln die Konfiguration Ihres Anbieters in `~/.claude/settings.json`, was sicherstellt, dass Ihre Einstellungen zwischen der VS Code-Erweiterung und der CLI geteilt werden.
  </Step>
</Steps>

<h2 id="security-and-privacy">
  Sicherheit und Datenschutz
</h2>

Ihr Code bleibt privat. Claude Code verarbeitet Ihren Code, um Unterstützung zu bieten, verwendet ihn aber nicht zum Trainieren von Modellen. Weitere Informationen zur Datenbehandlung und zum Deaktivieren der Protokollierung finden Sie unter [Daten und Datenschutz](/docs/de/data-usage).

Mit aktivierten Auto-Edit-Berechtigungen kann Claude Code VS Code-Konfigurationsdateien (wie `settings.json` oder `tasks.json`) ändern, die VS Code möglicherweise automatisch ausführt. Um das Risiko bei der Arbeit mit nicht vertrauenswürdigem Code zu verringern:

* Aktivieren Sie [VS Code Restricted Mode](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) für nicht vertrauenswürdige Arbeitsbereiche
* Verwenden Sie den manuellen Genehmigungsmodus statt Auto-Accept für Bearbeitungen
* Überprüfen Sie Änderungen sorgfältig, bevor Sie sie akzeptieren

<h3 id="the-built-in-ide-mcp-server">
  Der integrierte IDE MCP server
</h3>

Wenn die Erweiterung aktiv ist, wird ein lokaler MCP server ausgeführt, mit dem sich die CLI automatisch verbindet. Dies ist, wie die CLI Diffs in VS Codes nativem Diff-Viewer öffnet, Ihre aktuelle Auswahl für `@`-Erwähnungen liest und – wenn Sie in einem Jupyter-Notebook arbeiten – VS Code auffordert, Zellen auszuführen.

Der Server heißt `ide` und ist in `/mcp` verborgen, da es nichts zu konfigurieren gibt. Wenn Ihre Organisation jedoch einen `PreToolUse` Hook verwendet, um MCP-Tools auf eine Allowlist zu setzen, müssen Sie wissen, dass er existiert.

**Auswahl und Kontext offener Dateien.** Während der Verbindung bezieht die CLI Ihre aktuelle Editor-Auswahl und den Pfad der aktiven Datei als Kontext in jeden Prompt ein, den Sie senden. Das Transkript zeigt eine `⧉ Selected N lines from <file>`-Zeile, wenn dies geschieht. Um eine sensible Datei wie `.env` auszuschließen, fügen Sie eine [`Read` Deny-Regel](/docs/de/permissions#read-and-edit) für ihren Pfad hinzu. Eine entsprechende Deny-Regel verhindert sowohl, dass der ausgewählte Text als auch die Benachrichtigung über die offene Datei für diese Datei Claude erreichen.

**Transport und Authentifizierung.** Der Server bindet sich an `127.0.0.1` auf einem zufälligen Port im Bereich 10000–65535, und der Port ist nicht konfigurierbar. Der Transport ist unverschlüsseltes `ws://`; da der Socket nur Loopback ist, kann jeder Prozess, der den Datenverkehr erfassen könnte, auch das Token aus der Lock-Datei lesen, daher würde TLS keinen Schutz bieten. Jede Erweiterungsaktivierung generiert ein frisches zufälliges Auth-Token, schreibt es in eine Lock-Datei unter `~/.claude/ide/<port>.lock`, und die CLI muss es als `X-Claude-Code-Ide-Authorization` Header präsentieren, um sich zu verbinden. Die Lock-Datei hat `0600` Berechtigungen in einem `0700` Verzeichnis, daher kann nur der Benutzer, der VS Code ausführt, sie lesen. Wenn `CLAUDE_CONFIG_DIR` gesetzt ist, wird die Lock-Datei stattdessen in `$CLAUDE_CONFIG_DIR/ide/` geschrieben.

**Tools, die dem Modell ausgesetzt sind.** Der Server hostet ein Dutzend Tools, aber nur zwei sind für das Modell sichtbar. Der Rest ist interner RPC, den die CLI für ihre eigene Benutzeroberfläche verwendet – Diffs öffnen, Auswahlen lesen, Dateien speichern – und wird gefiltert, bevor die Tool-Liste Claude erreicht.

| Tool-Name (wie von Hooks gesehen) | Was es tut                                                                                                                          | Schreibgeschützt |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| `mcp__ide__getDiagnostics`        | Gibt Language-Server-Diagnosen zurück – die Fehler und Warnungen im Problembereich von VS Code. Optional auf eine Datei beschränkt. | Ja               |
| `mcp__ide__executeCode`           | Führt Python-Code im Kernel des aktiven Jupyter-Notebooks aus. Siehe Bestätigungsfluss unten.                                       | Nein             |

**Jupyter-Ausführung fragt immer zuerst.** `mcp__ide__executeCode` kann nichts stillschweigend ausführen. Bei jedem Aufruf wird der Code als neue Zelle am Ende des aktiven Notebooks eingefügt, VS Code scrollt ihn in die Ansicht, und eine native Quick Pick fragt Sie, ob Sie **Ausführen** oder **Abbrechen** möchten. Abbrechen – oder das Picker mit `Esc` schließen – gibt einen Fehler an Claude zurück und nichts wird ausgeführt. Das Tool weigert sich auch kategorisch, wenn es kein aktives Notebook gibt, wenn die Jupyter-Erweiterung (`ms-toolsai.jupyter`) nicht installiert ist, oder wenn der Kernel nicht Python ist.

<Note>
  Die Quick Pick-Bestätigung ist separat von `PreToolUse` Hooks. Ein Allowlist-Eintrag für `mcp__ide__executeCode` lässt Claude eine Zelle *vorschlagen*; die Quick Pick in VS Code ist das, was sie tatsächlich *ausführen* lässt.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Beheben Sie häufige Probleme
</h2>

<h3 id="extension-won’t-install">
  Erweiterung wird nicht installiert
</h3>

* Stellen Sie sicher, dass Sie eine kompatible Version von VS Code haben (1.98.0 oder später)
* Überprüfen Sie, dass VS Code die Berechtigung zum Installieren von Erweiterungen hat
* Versuchen Sie, direkt vom [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code) zu installieren

<h3 id="spark-icon-not-visible">
  Spark-Symbol nicht sichtbar
</h3>

Das Spark-Symbol wird in der **Editor-Symbolleiste** (oben rechts des Editors) angezeigt, wenn Sie eine Datei geöffnet haben. Wenn Sie es nicht sehen:

1. **Öffnen Sie eine Datei**: Das Symbol erfordert, dass eine Datei geöffnet ist. Nur einen Ordner zu öffnen reicht nicht aus.
2. **Überprüfen Sie die VS Code-Version**: Erfordert 1.98.0 oder höher (Hilfe → Über)
3. **Starten Sie VS Code neu**: Führen Sie „Developer: Reload Window" aus der Befehlspalette aus
4. **Deaktivieren Sie konfliktverursachende Erweiterungen**: Deaktivieren Sie vorübergehend andere KI-Erweiterungen (Cline, Continue usw.)
5. **Überprüfen Sie die Arbeitsbereichsvertrauenswürdigkeit**: Die Erweiterung funktioniert nicht im Restricted Mode

Alternativ klicken Sie auf „✱ Claude Code" in der **Statusleiste** (untere rechte Ecke). Dies funktioniert auch ohne offene Datei. Sie können auch die **Befehlspalette** (`Cmd+Shift+P` / `Ctrl+Shift+P`) verwenden und „Claude Code" eingeben.

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc funktioniert auf macOS nicht
</h3>

Auf macOS Tahoe und später ist die System-Game-Overlay-Verknüpfung standardmäßig an `Cmd+Esc` gebunden und fängt den Tastendruck ab, bevor er VS Code erreicht. Um die Verknüpfung freizugeben:

1. Öffnen Sie die Systemeinstellungen
2. Gehen Sie zu Tastatur, dann Tastaturkürzel, dann Game Controller
3. Deaktivieren Sie das Kontrollkästchen für Game Overlay

Alternativ können Sie die Erweiterung an eine andere Taste binden: Öffnen Sie den VS Code [Tastaturkürzel-Editor](https://code.visualstudio.com/docs/configure/keybindings) (`Cmd+K Cmd+S`), suchen Sie nach `Claude Code: Focus input`, und weisen Sie eine neue Bindung zu.

<h3 id="claude-code-never-responds">
  Claude Code antwortet nie
</h3>

Wenn Claude Code nicht auf Ihre Eingabeaufforderungen antwortet:

1. **Überprüfen Sie Ihre Internetverbindung**: Stellen Sie sicher, dass Sie eine stabile Internetverbindung haben
2. **Starten Sie ein neues Gespräch**: Versuchen Sie, ein neues Gespräch zu starten, um zu sehen, ob das Problem weiterhin besteht
3. **Versuchen Sie die CLI**: Führen Sie `claude` vom Terminal aus, um zu sehen, ob Sie detailliertere Fehlermeldungen erhalten

Wenn Probleme weiterhin bestehen, [melden Sie ein Problem auf GitHub](https://github.com/anthropics/claude-code/issues) mit Details zum Fehler.

<h2 id="uninstall-the-extension">
  Deinstallieren Sie die Erweiterung
</h2>

So deinstallieren Sie die Claude Code-Erweiterung:

1. Öffnen Sie die Ansicht „Erweiterungen" (`Cmd+Shift+X` auf Mac oder `Ctrl+Shift+X` auf Windows/Linux)
2. Suchen Sie nach „Claude Code"
3. Klicken Sie auf **Deinstallieren**

Wenn Sie `claude` in einem integrierten VS Code-Terminal ausführen, wird die Erweiterung automatisch neu installiert. Um sie deinstalliert zu halten, deaktivieren Sie **Auto-install IDE extension** in `/config`, oder setzen Sie [`autoInstallIdeExtension`](/docs/de/settings#global-config-settings) auf `false`. Sie können auch die Umgebungsvariable [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/de/env-vars) auf `1` setzen.

Um auch Erweiterungsdaten zu entfernen und alle Einstellungen zurückzusetzen, löschen Sie das Speicherverzeichnis der Erweiterung für Ihre Plattform.

Auf macOS:

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

Auf Linux:

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

Auf Windows in PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

Weitere Hilfe finden Sie im [Fehlerbehebungsleitfaden](/docs/de/troubleshooting).

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Claude Code in VS Code eingerichtet haben:

* [Erkunden Sie häufige Workflows](/docs/de/common-workflows), um das Beste aus Claude Code herauszuholen
* [Richten Sie MCP servers ein](/docs/de/mcp), um Claudes Funktionen mit externen Tools zu erweitern. Fügen Sie Server über die CLI hinzu, verwalten Sie sie dann mit `/mcp` im Chat-Panel.
* [Konfigurieren Sie Claude Code-Einstellungen](/docs/de/settings), um zulässige Befehle, hooks und mehr anzupassen. Diese Einstellungen werden zwischen der Erweiterung und der CLI geteilt.
