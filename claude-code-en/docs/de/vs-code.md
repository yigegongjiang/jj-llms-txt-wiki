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

* VS Code 1.94.0 oder höher
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

Nach der Installation können Sie Claude Code über die VS Code-Oberfläche verwenden:

<Steps>
  <Step title="Öffnen Sie das Claude Code-Panel">
    In VS Code zeigt das Spark-Symbol Claude Code an: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Spark-Symbol" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Der schnellste Weg, Claude zu öffnen, ist, auf das Spark-Symbol in der **Editor-Symbolleiste** (obere rechte Ecke des Editors) zu klicken. Das Symbol wird nur angezeigt, wenn Sie eine Datei geöffnet haben.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="VS Code-Editor mit dem Spark-Symbol in der Editor-Symbolleiste" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Weitere Möglichkeiten zum Öffnen von Claude Code:

    * **Aktivitätsleiste**: Klicken Sie auf das Spark-Symbol in der linken Seitenleiste, um die Sitzungsliste zu öffnen. Klicken Sie auf eine beliebige Sitzung, um sie an Ihrem [bevorzugten Ort](#extension-settings) zu öffnen, oder starten Sie eine neue. Dieses Symbol ist immer in der Aktivitätsleiste sichtbar.
    * **Befehlspalette**: `Cmd+Shift+P` (Mac) oder `Ctrl+Shift+P` (Windows/Linux), geben Sie „Claude Code" ein und wählen Sie eine Option wie „In neuem Tab öffnen"
    * **Statusleiste**: Wenn Sie [`preferredLocation`](#extension-settings) auf `sidebar` gesetzt haben oder Claude mit **Claude Code: In Seitenleiste öffnen** geöffnet haben, klicken Sie auf **✻ Claude Code** in der unteren rechten Ecke des Fensters. Dies funktioniert auch, wenn keine Datei geöffnet ist.

    Sie können das Claude-Panel ziehen, um es überall in VS Code zu repositionieren. Weitere Informationen finden Sie unter [Passen Sie Ihren Arbeitsablauf an](#customize-your-workflow).
  </Step>

  <Step title="Melden Sie sich an">
    Wenn Sie das Panel zum ersten Mal öffnen, wird ein Anmeldungsbildschirm angezeigt. Klicken Sie auf **Anmelden** und schließen Sie die Autorisierung in Ihrem Browser ab.

    Wenn Sie später **Nicht angemeldet · Bitte führen Sie /login aus** sehen, öffnet die Erweiterung den Anmeldungsbildschirm automatisch erneut. Wenn dieser nicht angezeigt wird, laden Sie das Fenster aus der Befehlspalette mit **Developer: Reload Window** neu.

    Wenn Sie `ANTHROPIC_API_KEY` in Ihrer Shell gesetzt haben, aber immer noch die Anmeldungsaufforderung sehen, hat VS Code möglicherweise Ihre Shell-Umgebung nicht geerbt. Starten Sie VS Code von einem Terminal aus mit `code .`, damit es Ihre Umgebungsvariablen erbt, oder melden Sie sich stattdessen mit Ihrem Claude-Konto an.

    Nach der Anmeldung wird eine **Learn Claude Code**-Checkliste angezeigt. Arbeiten Sie jedes Element durch, indem Sie auf **Show me** klicken, oder schließen Sie es mit dem X. Um es später erneut zu öffnen, deaktivieren Sie **Hide Onboarding** in den VS Code-Einstellungen unter Extensions → Claude Code.
  </Step>

  <Step title="Senden Sie einen Prompt">
    Bitten Sie Claude, Ihnen bei Ihrem Code oder Ihren Dateien zu helfen, sei es zum Erklären, wie etwas funktioniert, zum Debuggen eines Problems oder zum Vornehmen von Änderungen.

    <Tip>Claude sieht automatisch Ihren ausgewählten Text. Drücken Sie `Option+K` (Mac) / `Alt+K` (Windows/Linux), um auch eine @-Mention-Referenz (wie `@file.ts#5-10`) in Ihren Prompt einzufügen.</Tip>

    Hier ist ein Beispiel für eine Frage zu einer bestimmten Zeile in einer Datei:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="VS Code-Editor mit den Zeilen 2-3 ausgewählt in einer Python-Datei und dem Claude Code-Panel mit einer Frage zu diesen Zeilen mit einer @-Mention-Referenz" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Überprüfen Sie die Änderungen">
    Was Sie sehen, hängt vom [Berechtigungsmodus](/docs/de/permission-modes#which-mode-a-session-starts-in) ab, der unten im Prompt-Feld angezeigt wird:

    * Im Modus „Auto" oder „Automatisch bearbeiten" bearbeitet Claude die meisten Dateien in Ihrem Arbeitsbereich ohne Nachfrage.
    * Im Modus „Manuell" zeigt Claude, wenn es eine Datei bearbeiten möchte, einen Vergleich der ursprünglichen und vorgeschlagenen Änderungen nebeneinander an und fragt um Genehmigung. Sie können akzeptieren, ablehnen oder Claude sagen, was es stattdessen tun soll. Wenn Sie den vorgeschlagenen Inhalt direkt in der Diff-Ansicht bearbeiten, bevor Sie akzeptieren, wird Claude mitgeteilt, dass Sie ihn geändert haben, damit es nicht davon ausgeht, dass die Datei seinem ursprünglichen Vorschlag entspricht.

          <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code mit einem Diff von Claudes vorgeschlagenen Änderungen und einer Berechtigungsaufforderung, die fragt, ob die Bearbeitung durchgeführt werden soll" width="3292" height="1876" data-path="images/vs-code-edits.png" />

    Um einen vorgeschlagenen Bearbeitungsvorgang nacheinander zu überprüfen, verwenden Sie die Schaltflächen **Accept this change** und **Reject this change** unter jeder Änderung im Diff. Das Ablehnen einer Änderung setzt sie im vorgeschlagenen Inhalt zurück; das Akzeptieren markiert sie als überprüft. Das Akzeptieren oder Ablehnen der gesamten Datei beendet die Überprüfung dennoch. Ein Diff mit mehr als 100 Änderungen wird ohne die Schaltflächen pro Änderung geöffnet, daher überprüfen Sie ihn als ganze Datei. Die Überprüfung pro Änderung erfordert Claude Code v2.1.275 oder später.

    Die gleichen Aktionen sind vom Cursor aus im Kontextmenü des Editors und in der Befehlspalette als **Claude Code: Accept Change at Cursor** und **Claude Code: Reject Change at Cursor** verfügbar.
  </Step>
</Steps>

Weitere Ideen, was Sie mit Claude Code tun können, finden Sie unter [Häufige Arbeitsabläufe](/docs/de/common-workflows).

<Tip>
  Führen Sie „Claude Code: Open Walkthrough" aus der Befehlspalette aus, um eine geführte Tour durch die Grundlagen zu erhalten.
</Tip>

<h2 id="use-the-prompt-box">
  Verwenden Sie das Eingabefeld
</h2>

Das Eingabefeld unterstützt mehrere Funktionen:

* **Berechtigungsmodi**: Klicken Sie auf den Modusindikator am unteren Rand des Eingabefelds, um zwischen Berechtigungsmodi zu wechseln. In den Pro-, Max- und Team-Plänen ist Auto der integrierte Startberechtigungsmodus. Siehe [wie die Erweiterung den Startberechtigungsmodus wählt](/docs/de/permission-modes#switch-permission-modes), um zu erfahren, was das ändert, und jeden Berechtigungsmodus, den der Indikator anbietet.
  * **Auto**: Ein Klassifizierer überprüft die meisten Aktionen, anstatt Sie zu fragen. Siehe [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), um zu erfahren, was er überprüft und blockiert.
  * **Manuell**: Claude fragt vor Dateibearbeitungen und den meisten Shell-Befehlen um Genehmigung.
  * **Plan**: Claude beschreibt, was es tun wird, und wartet auf Genehmigung, bevor Änderungen vorgenommen werden. VS Code öffnet den Plan automatisch als vollständiges Markdown-Dokument, in dem Sie Inline-Kommentare hinzufügen können, um Feedback zu geben, bevor Claude beginnt.

    Sie können auch `/plan` im Eingabefeld eingeben. Erfordert Claude Code v2.1.280 oder später.

    * `/plan`: wechselt zum Plan-Modus. Wenn Sie bereits im Plan-Modus sind, zeigt es den aktuellen Plan statt.
    * `/plan` mit einer Aufgabe, wie `/plan fix the auth bug`: wechselt zum Plan-Modus und beginnt, diese Aufgabe zu planen.
    * `/plan open`: wenn Sie bereits im Plan-Modus sind, öffnet die Plan-Datei im Editor.
  * **Automatisch bearbeiten**: Claude nimmt Bearbeitungen vor, ohne zu fragen.
* **Modell**: Wählen Sie **Modell wechseln…** aus dem Befehlsmenü, um das Modell während einer Sitzung zu ändern. Sie können auch auf den Modellnamen am unteren Rand des Eingabefelds klicken, um die gleiche Auswahl zu öffnen.

  Wenn das aktuelle Modell [Anstrengungsstufen](/docs/de/model-config#adjust-effort-level) unterstützt, zeigt die Auswahl auch eine Zeile **Anstrengung** und die Schaltfläche für den Modellnamen zeigt die ausgewählte Stufe. Wenn Sie eine andere Stufe als `max` wählen, speichert Claude Code sie für das aktuelle Modell als Standard unter [`modelSettings`](/docs/de/settings-reference#modelsettings) in Ihren Benutzereinstellungen; `max` gilt nur für die aktuelle Sitzung. Die Schaltfläche für den Modellnamen und die Zeile **Anstrengung** erfordern Claude Code v2.1.257 oder später.
* **Befehlsmenü**: Klicken Sie auf `/` oder geben Sie `/` ein, um das Befehlsmenü zu öffnen. Die Optionen umfassen das Anhängen von Dateien, das Wechseln von Modellen und das Umschalten des erweiterten Denkens.

  Der Abschnitt „Anpassen" bietet Zugriff auf MCP-Server, Befehle, Ausgabestile, Hooks, Speicher, Anweisungen, Berechtigungen und Plugins. Elemente mit einem Terminal-Symbol öffnen sich im integrierten Terminal.

  * Um Befehle wie `/usage` oder [`/remote-control`](/docs/de/remote-control) zu durchsuchen, wählen Sie **Slash commands** im Abschnitt „Anpassen". Ein Dialog listet sie mit einem Filterfeld auf. Wählen Sie einen aus, um ihn auszuführen. Das Eingeben von `/` im Eingabefeld schlägt Befehle weiterhin inline vor. Erfordert Claude Code v2.1.257 oder später.

    Das Eingeben von `/skills` öffnet auch diesen Dialog. Jede [Skill](/docs/de/skills)-Zeile zeigt ihre [Sichtbarkeit](/docs/de/skills#override-skill-visibility-from-settings), wie **On** oder **Name only**. Klicken Sie auf die Sichtbarkeit, um sie zu ändern, außer bei Zeilen, die als **locked** gekennzeichnet sind, wie Plugin-Skills. Die `/skills`-Verknüpfung und die Sichtbarkeitssteuerelemente erfordern Claude Code v2.1.280 oder später.
  * Wählen Sie **Output styles** im Abschnitt „Anpassen", um einen [Ausgabestil](/docs/de/output-styles) auszuwählen, einschließlich Ihrer benutzerdefinierten Stile. Erfordert Claude Code v2.1.257 oder später.

    Um stattdessen einen benutzerdefinierten Stil zu erstellen, wählen Sie **Build a custom style** aus dem Menü **Output styles**. Claude Code schreibt die [Stildatei](/docs/de/output-styles#create-a-custom-output-style) für Sie auf Projekt- oder Benutzerebene. Erfordert Claude Code v2.1.261 oder später.
  * Wählen Sie **Hooks** im Abschnitt „Anpassen", um die [Hooks](/docs/de/hooks) anzuzeigen, die in der Sitzung geladen sind, gruppiert nach Ereignis. Sie können Hooks hinzufügen, bearbeiten oder entfernen, die in Ihren Benutzer-, Projekt- und lokalen Einstellungsdateien gespeichert sind. Hooks aus anderen Quellen, wie verwaltete Einstellungen oder Plugins, sind schreibgeschützt. Erfordert Claude Code v2.1.269 oder später.
  * Wählen Sie **Permissions** im Abschnitt „Anpassen", um die [Berechtigungsregeln](/docs/de/permissions) der Sitzung anzuzeigen, gruppiert in „Allow", „Ask" und „Deny". Sie können Regeln zu Ihren Benutzer-, Projekt- oder lokalen Einstellungen hinzufügen und dort gespeicherte Regeln entfernen. Regeln aus anderen Quellen, wie verwaltete Einstellungen oder Genehmigungen, die nur für diese Sitzung gelten, sind schreibgeschützt. Erfordert Claude Code v2.1.269 oder später.
  * Wählen Sie **Memory** im Abschnitt „Anpassen", um [automatisches Speichern](/docs/de/memory#auto-memory) ein- oder auszuschalten. Während es aktiviert ist, können Sie auch die Speicher durchsuchen, die Claude gespeichert hat, und die Ordner, die sie speichern, in Ihrem Datei-Manager anzeigen. Erfordert Claude Code v2.1.274 oder später.

    Klicken Sie auf einen gespeicherten Speicher, um ihn im Dialog zu lesen, wo Sie den Text bearbeiten, den Speicher löschen oder seine Datei im Editor öffnen können. Das Anzeigen, Bearbeiten und Löschen eines Speichers im Dialog erfordern Claude Code v2.1.275 oder später.
  * Wählen Sie **Instructions** im Abschnitt „Anpassen", um die [CLAUDE.md-Dateien](/docs/de/memory#claude-md-files) zu bearbeiten, die Claude liest. Wählen Sie eine Datei aus, um sie im Editor zu öffnen. Wenn die Datei noch nicht vorhanden ist, erstellt Claude Code sie zuerst. Erfordert Claude Code v2.1.274 oder später.
  * Wählen Sie **Status** im Abschnitt „Anpassen", oder geben Sie `/status` ein, um die Claude Code-Version, das Konto, das Modell und die MCP-Server-Details der Sitzung zu überprüfen. Erfordert Claude Code v2.1.280 oder später.
  * Wählen Sie **Sandbox** im Abschnitt „Anpassen", oder geben Sie `/sandbox` ein, um zu sehen, ob Claudes Bash-Befehle [sandboxed](/docs/de/sandboxing) ausgeführt werden. Sie können den Sandbox-Modus dort wechseln und [ausgeschlossene Befehle](/docs/de/settings-reference#sandbox-excludedcommands) hinzufügen. Erfordert Claude Code v2.1.280 oder später.
  * Wählen Sie **Claude in Chrome** im Abschnitt „Anpassen", oder geben Sie `/chrome` ein, um die [Claude in Chrome](/docs/de/chrome)-Verbindung zu überprüfen und zu verwalten. Beide erfordern die Anmeldung mit einem claude.ai-Konto. Erfordert Claude Code v2.1.280 oder später.
  * Wählen Sie **Export conversation** im Abschnitt „Context", oder geben Sie `/export` ein, um das Gespräch als Klartext zu kopieren oder in einer Datei zu speichern. Fügen Sie einen Dateinamen hinzu, wie `/export notes.txt`, um den Dialog zu überspringen und auszuwählen, wo die Datei gespeichert werden soll. Erfordert Claude Code v2.1.280 oder später.
  * Der Abschnitt „Einstellungen" enthält **Enable Remote Control for all sessions**, das [`remoteControlAtStartup`](/docs/de/settings-reference#remotecontrolatstartup) setzt, um zu steuern, ob [neue interaktive Sitzungen automatisch eine Verbindung zur Remote-Steuerung herstellen](/docs/de/remote-control#enable-remote-control-for-all-sessions). Erfordert Claude Code v2.1.203 oder später.

    Wenn Sie den Schalter in einem VS Code-Fenster ein- oder ausschalten, gilt die Änderung für die bereits in diesem VS Code-Fenster geöffneten Sitzungen, nicht nur für Sitzungen, die Sie danach starten. Wenn Sie ihn ausschalten, werden die offenen Sitzungen getrennt. Mit Claude Code v2.1.261 oder später erreicht die Änderung auch Sitzungen, die in Ihren anderen VS Code-Fenstern geöffnet sind.
  * Der Abschnitt „Einstellungen" enthält auch **Focus view**, das Werkzeugaufrufe, Werkzeugergebnisse und Denken hinter erweiterbaren Zeilen verbirgt und nur Ihre Eingaben und Claudes Antworten hinterlässt. Schalten Sie es dort um, mit `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux), oder aus der Befehlspalette mit **Claude Code: Toggle Focus view**. Die Änderung gilt für jede offene Sitzung und bleibt über Sitzungen hinweg erhalten. Erfordert Claude Code v2.1.221 oder später.

    Claudes neueste To-Do-Liste bleibt sichtbar, ebenso wie der Text einer ausstehenden Frage von Claude; dies erfordert Claude Code v2.1.225 oder später. Während Claude [Subagenten](/docs/de/sub-agents) ausführt, erscheinen Live-Fortschrittszeilen mit ihrer neuesten Aktivität unter der Werkzeugaufrufsgruppe, die sie gestartet hat. Dies erfordert Claude Code v2.1.269 oder später.
  * Um sich von Ihrem Anthropic-Konto abzumelden, wählen Sie **Sign out** im Abschnitt „Einstellungen", oder geben Sie `/logout` ein. Bei einem [Drittanbieter](#use-third-party-providers) bietet das Menü keines von beiden. Erfordert Claude Code v2.1.277 oder später.
  * Um einen Fehler zu melden, klicken Sie auf **Report a problem** am unteren Rand des Menüs, oder geben Sie `/bug` oder `/feedback` mit einer optionalen Beschreibung ein, die den Bericht ausfüllt. Wenn Sie den Bericht einreichen und Sie bei Anthropic auf einer First-Party-Verbindung angemeldet sind, sendet Claude Code ihn an Anthropic. Bei einem Drittanbieter oder ohne Anthropic-Anmeldedaten öffnet sich der Dialog trotzdem, aber das Einreichen zeigt einen Fehler und sendet nichts: Im Gegensatz zum CLI `/bug` schreibt die Erweiterung kein lokales Archiv. Erfordert Claude Code v2.1.229 oder später.

    Wenn die Richtlinie Ihrer Organisation Produktfeedback deaktiviert, wird **Report a problem** nicht im Menü angezeigt, und `/bug` und `/feedback` zeigen stattdessen eine `Feedback is turned off by your organization's policy or this environment's settings.` Benachrichtigung an.
* **Nebenfragen**: Geben Sie `/btw` gefolgt von einer Frage ein, um eine Frage zu Ihrer Sitzung zu stellen, [ohne sie zum Gespräch hinzuzufügen](/docs/de/interactive-mode#side-questions-with-%2Fbtw). Die Antwort öffnet sich in einem Panel neben dem Chat, in dem Sie Anschlussfragen stellen können. Der Thread bleibt bei Fenster-Neuladen erhalten. Claude Code behält die neuesten 20 Austausche und läuft gespeicherte Threads nach dem [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays)-Plan ab, solange Claude Code [sicher die Aufbewahrungsfrist bestimmen kann](/docs/de/claude-directory#cleaned-up-automatically). Um einen Thread zu löschen, klicken Sie auf das Papierkorbsymbol im Panel. Erfordert Claude Code v2.1.227 oder später.
* **Antwort kopieren**: Bewegen Sie den Mauszeiger über eine Antwort und klicken Sie auf **Copy response**, um sie in die Zwischenablage zu kopieren, oder geben Sie `/copy` ein, um die neueste Antwort zu kopieren. `/copy 2` kopiert die vorletzte. Erfordert Claude Code v2.1.277 oder später.
* **Kontextindikator**: Das Eingabefeld zeigt, wie viel von Claudes Kontextfenster Sie verwenden. Claude komprimiert automatisch bei Bedarf, oder Sie können `/compact` manuell ausführen.
* **Prompt-Cache-Uhr**: Ein Uhr-Symbol neben dem Kontextindikator schätzt, wie viel Zeit das [Prompt-Cache](/docs/de/prompt-caching) des Gesprächs noch hat, bevor es abläuft. Es zählt von der [Lebensdauer](/docs/de/prompt-caching#cache-lifetime) des Caches von fünf Minuten oder einer Stunde herunter, und jede Antwort, die den Cache verwendet, startet den Countdown neu. Abgesehen von der Komprimierung setzen die [Aktionen, die den Cache ungültig machen](/docs/de/prompt-caching#actions-that-invalidate-the-cache), die Uhr nicht zurück, daher kann sie immer noch Minuten anzeigen, nachdem Sie Modelle wechseln.
  * Bis der Countdown abläuft, zeigt das Symbol die verbleibenden Minuten an, z. B. **12m**.
  * Wenn der Countdown abläuft, verschwinden die Minuten und das Symbol wird rot oder die Fehlerfarbe Ihres Designs, bis zur nächsten Antwort. Der Cache ist wahrscheinlich abgelaufen, daher erwarten Sie eine langsamere, teurere Antwort auf Ihre nächste Nachricht, während der Cache neu aufgebaut wird. Wenn die fünfminütige Lebensdauer zwischen Ihren Nachrichten immer wieder abläuft, siehe [Wählen Sie die TTL selbst](/docs/de/prompt-caching#choose-the-ttl-yourself).
  * Unmittelbar nach dem [Komprimieren](/docs/de/prompt-caching#compacting-the-conversation) des Gesprächs wird das Symbol auch rot ohne Minuten bis zur nächsten Antwort, da der Cache das komprimierte Gespräch noch nicht abdeckt.
* **Agent-Karte**: Wenn das Gespräch [Subagenten](/docs/de/sub-agents) enthält, erscheint eine Agent-Anzahl wie **2 agents** am unteren Rand des Eingabefelds. Sein Punkt zeigt, ob ein Subagent arbeitet oder auf Ihre Genehmigung wartet.

  Klicken Sie auf die Agent-Anzahl, um die Agent-Karte zu öffnen, die die Subagenten des Gesprächs als Baum unter dem Hauptagenten zeichnet, jeweils mit seinem Status, verstrichener Zeit und Token-Anzahl. Klicken Sie auf einen Subagenten, um sein Eingabefeld und seine Werkzeugaufrufe anzuzeigen, sein schreibgeschütztes Transkript zu öffnen oder ihn während der Ausführung zu stoppen. Erfordert Claude Code v2.1.269 oder später.

  Die Karte listet auch die anderen [Hintergrundaufgaben](/docs/de/tools-reference#background-commands) der Sitzung auf, wie Hintergrund-Shell-Befehle und [Monitore](/docs/de/tools-reference#monitor-tool), unter den Agenten. Klicken Sie auf eine Zeile, um die Karte der Aufgabe zu öffnen und sie dort zu stoppen.

  Um die Karte zu öffnen, wenn keine Agent-Anzahl angezeigt wird, z. B. wenn Claude einen Hintergrund-Shell-Befehl gestartet hat, aber keine Subagenten, geben Sie `/tasks` im Eingabefeld ein. Hintergrundaufgaben in der Karte und das eingegebene `/tasks` erfordern Claude Code v2.1.277 oder später.
* **Erweitertes Denken**: Ermöglicht Claude, mehr Zeit für die Überlegung komplexer Probleme aufzuwenden. Schalten Sie es über das Befehlsmenü (`/`) ein. Claudes Überlegungen erscheinen im Gespräch als zusammengeklappte Blöcke: Klicken Sie auf einen Block, um ihn zu lesen, oder drücken Sie `Ctrl+O`, um jeden Denkblock in der Sitzung zu erweitern oder zu reduzieren. Siehe [Erweitertes Denken](/docs/de/model-config#extended-thinking) für Details.
* **Mehrzeilige Eingabe**: Drücken Sie `Shift+Enter`, um eine neue Zeile hinzuzufügen, ohne zu senden. Dies funktioniert auch in der Freitexteingabe „Other" von Frage-Dialogen.

<h3 id="reference-files-and-folders">
  Referenzdateien und -ordner
</h3>

Verwenden Sie @-Erwähnungen, um Claude Kontext über bestimmte Dateien oder Ordner zu geben. Wenn Sie `@` gefolgt von einem Datei- oder Ordnernamen eingeben, liest Claude diesen Inhalt und kann Fragen dazu beantworten oder Änderungen daran vornehmen. Claude Code unterstützt Fuzzy Matching, sodass Sie Teilnamen eingeben können, um zu finden, was Sie benötigen:

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

Bei großen PDFs können Sie Claude bitten, bestimmte Seiten statt der gesamten Datei zu lesen: eine einzelne Seite, einen Bereich wie Seiten 1-10 oder einen offenen Bereich wie Seite 3 und darüber hinaus.

Wenn Sie Text im Editor auswählen, kann Claude Ihren hervorgehobenen Code automatisch sehen. Die Fußzeile des Eingabefelds zeigt, wie viele Zeilen ausgewählt sind. Drücken Sie `Option+K` (Mac) / `Alt+K` (Windows/Linux), um eine @-Erwähnung mit dem Dateipfad und den Zeilennummern einzufügen (z. B. `@app.ts#5-10`). Klicken Sie auf das **X** auf dem Auswahlindikator, um ihn zu entfernen, damit Claude die Auswahl nicht erhält. Der Indikator erscheint wieder, wenn Sie anderen Text auswählen.

Die Erweiterung behält ausgewählten Text aus einigen Dateien zurück. Wenn sich die Datei in Ihrem Arbeitsbereich befindet und Ihren `files.exclude`- oder `search.exclude`-Einstellungen entspricht, erhält Claude höchstens den Dateipfad und nicht den Text, den Sie ausgewählt haben. Das Gleiche gilt für eine Datei, die Git ignoriert, solange die `search.useIgnoreFiles`-Einstellung von VS Code und die [`respectGitIgnore`-Einstellung](#extension-settings) der Erweiterung beide aktiviert sind, was die Standardeinstellung ist. Dieser Filter gilt nur für das Chat-Panel: Wenn Claude Code im integrierten Terminal ausgeführt wird, sendet die CLI Ihren ausgewählten Text unabhängig von der Datei, daher fügen Sie eine [`Read`-Ablehnungsregel](#the-built-in-ide-mcp-server) hinzu, um zu verhindern, dass Claude dort auf die Inhalte einer Datei zugreift.

Claude sieht auch, welche Datei Sie im Editor geöffnet haben, auch wenn nichts ausgewählt ist, und das Eingabefeld zeigt seinen Namen. Um nur Ihren ausgewählten Text hinzuzufügen, schalten Sie die [Attach Open File-Einstellung](vscode://settings/claudeCode.attachOpenFile) aus. Die Einstellung erfordert Claude Code v2.1.271 oder später.

Sie können auch Bilder und Dateien an Ihre Nachricht anhängen:

* Um ein Bild anzuhängen, fügen Sie es aus Ihrer Zwischenablage in das Eingabefeld ein.
* Um Dateien anzuhängen, halten Sie `Shift` gedrückt, während Sie sie in das Eingabefeld ziehen.
* Um einen Anhang aus dem Kontext zu entfernen, klicken Sie auf das X darauf.

<h3 id="paste-text">
  Text einfügen
</h3>

Text, den Sie einfügen, bleibt im Eingabefeld sichtbar, anstatt zu einem Platzhalter zu kollabieren, wie es [im Terminal](/docs/de/terminal-config#paste-large-content) der Fall ist. In Sitzungen, in denen Claude Code [eingefügten Text markiert](/docs/de/terminal-config#how-claude-treats-pasted-text), sieht Claude ein großes Einfügen immer noch als Text, den Sie eingefügt haben, anstatt eingegeben zu haben.

Claude Code entfernt auch [unsichtbare Unicode-Zeichen](/docs/de/interactive-mode#invisible-characters-in-prompts) aus Text, den Sie in das Eingabefeld einfügen, und aus allem anderen, das Sie senden:

* Wenn eine Benachrichtigung wie `Removed 3 invisible characters from the pasted text` angezeigt wird, wenn Sie einfügen, ging der Text ohne diese Zeichen ein.
* Wenn eine Benachrichtigung über entfernte Zeichen angezeigt wird, wenn Sie senden, wurde nichts gesendet. Der bereinigte Text ist zurück im Eingabefeld. Senden Sie erneut, um den Text wie angezeigt zu senden.

<h3 id="resume-past-conversations">
  Frühere Gespräche fortsetzen
</h3>

Klicken Sie auf die Schaltfläche **Session history** oben im Claude Code-Panel, um auf Ihren Gesprächsverlauf zuzugreifen. Sie können nach Schlüsselwort suchen oder nach Zeit durchsuchen.

Klicken Sie auf ein beliebiges Gespräch, um es mit dem vollständigen Nachrichtenverlauf fortzusetzen. Wenn das Gespräch bereits in einer anderen Registerkarte des aktuellen Fensters geöffnet ist, wird durch Klicken darauf zu dieser Registerkarte gewechselt. Weitere Informationen zum Fortsetzen von Sitzungen finden Sie unter [Sitzungen verwalten](/docs/de/sessions).

* **Sitzungstitel**: Neue Sitzungen erhalten KI-generierte Titel basierend auf Ihrer ersten Nachricht.
* **Umbenennen und archivieren**: Bewegen Sie den Mauszeiger über eine Sitzung, um diese Aktionen anzuzeigen. Benennen Sie sie um, um ihr einen beschreibenden Titel zu geben, oder archivieren Sie sie, um sie in die Gruppe **Archived sessions** am unteren Rand der Liste zu verschieben.

Standardmäßig wird eine Sitzung ohne Aktivität für 14 Tage automatisch zu **Archived sessions** verschoben, es sei denn, sie ist offen, ungelesen oder in einer [Gruppe](#organize-sessions-into-groups). Automatisches Archivieren erfordert Claude Code v2.1.265 oder später. Um den Zeitraum zu ändern oder auszuschalten, öffnen Sie die [Archive Inactive Sessions-Einstellung](vscode://settings/claudeCode.archiveInactiveSessions) und wählen Sie eine Anzahl von Tagen oder **Never**.

Um eine archivierte Sitzung wiederherzustellen, erweitern Sie **Archived sessions** und klicken Sie auf **Unarchive session**. Um jede archivierte Sitzung auf einmal wiederherzustellen, bewegen Sie den Mauszeiger über die Kopfzeile **Archived sessions** in der Sitzungsliste in der Aktivitätsleiste und klicken Sie auf sein Unarchive-Symbol, das Claude Code v2.1.277 oder später erfordert. Vor v2.1.257 war die Aktion **Delete session**, die eine Sitzung ohne Möglichkeit zur Wiederherstellung ausblendete. Sitzungen, die Sie dann gelöscht haben, werden nach dem Upgrade unter **Archived sessions** angezeigt.

Wenn das Gespräch, das Sie fortsetzen, im Plan-Modus endete, stellt Claude Code den Plan-Modus wieder her. Erfordert Claude Code v2.1.246 oder später. Claude Code stellt ihn in zwei Fällen nicht wieder her:

* Die Erweiterung [wählt den Startberechtigungsmodus](/docs/de/permission-modes#switch-permission-modes) aus `claudeCode.initialPermissionMode` oder einer Auswahl, die aus einem früheren Gespräch übernommen wird
* Sie haben `claudeCode.claudeProcessWrapper` konfiguriert

<h3 id="resume-cloud-sessions-from-claude-ai">
  Cloud-Sitzungen von Claude.ai fortsetzen
</h3>

Wenn Sie [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) ausführen, können Sie sie direkt in VS Code fortsetzen. Dies erfordert die Anmeldung mit **Claude.ai Subscription**, nicht Anthropic Console.

<Steps>
  <Step title="Open session history">
    Click the **Session history** button at the top of the Claude Code panel.
  </Step>

  <Step title="Select the Web tab">
    The dialog shows two tabs: Local and Web. Click **Web** to see sessions from claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Browse or search your cloud sessions. Click any session to download it and continue the conversation locally.
  </Step>
</Steps>

<Note>
  Only cloud sessions started with a GitHub repository appear in the Web tab. Resuming loads the conversation history locally; changes are not synced back to claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Konto und Nutzung überprüfen
</h3>

Führen Sie `/usage` aus, um das Dialogfeld „Account & usage" zu öffnen. Es zeigt Ihr angemeldetes Konto, und die Nutzung, die es meldet, unterscheidet sich je nach Anmeldung:

* **claude.ai-Plan**: Nutzungsbalken für die Limits Ihres Plans, wie die aktuelle Sitzung und die Woche. Jeder Balken zeigt, wie lange es dauert, bis sein Limit zurückgesetzt wird.

  Das Dialogfeld schlüsselt auch auf, was zu Ihren Planlimits beiträgt. Es kennzeichnet Verhaltensweisen, die 10 % oder mehr der letzten Nutzung ausmachen, wie z. B. Cache-Misses, langer Kontext und Subagent-intensive oder hochgradig parallele Sitzungen, jeweils mit einem Tipp zur Reduzierung. Attributionstabellen zeigen, wie viel Nutzung von jedem Skill, Subagent, Plugin und MCP-Server kam.

  Verwenden Sie den Umschalter „Day" und „Week", um zwischen den letzten 24 Stunden und den letzten 7 Tagen zu wechseln. Die Zahlen sind ungefähr und werden aus lokalen Sitzungen auf diesem Computer berechnet, daher ist die Nutzung von anderen Geräten oder claude.ai nicht enthalten.
* **Andere Anmeldungen**: Wenn Planlimits nicht auf Ihre Anmeldung zutreffen, z. B. bei einem [Drittanbieter](#use-third-party-providers) oder mit einem API-Schlüssel, zeigt der Abschnitt „Usage" stattdessen die Kosten und Token-Nutzung der Sitzung selbst. Die CLI `/usage` zeigt die gleichen Summen in ihrem [Session-Block](/docs/de/costs#track-your-costs). Die Sitzungsliste in der Aktivitätsleiste zeigt auch die Summen der aktiven Sitzung unter ihrer Kopfzeile **Account & usage**. Erfordert Claude Code v2.1.277 oder später.

Weitere Informationen zum Verfolgen und Reduzieren der Nutzung finden Sie unter [Verfolgen Sie Ihre Kosten](/docs/de/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Passen Sie Ihren Arbeitsablauf an
</h2>

Sie können das Claude-Panel neu positionieren, mehrere Gespräche führen, die Sitzungsliste in Gruppen organisieren oder in den Terminalmodus wechseln.

<h3 id="choose-where-claude-lives">
  Wählen Sie, wo Claude sich befindet
</h3>

Sie können das Claude-Panel überall in VS Code neu positionieren. Greifen Sie die Registerkarte oder Titelleiste des Panels und ziehen Sie es zu:

* **Sekundäre Seitenleiste**: die rechte Seite des Fensters. Hält Claude sichtbar, während Sie programmieren.
* **Primäre Seitenleiste**: die linke Seitenleiste mit Symbolen für Explorer, Suche usw.
* **Editor-Bereich**: öffnet Claude als Registerkarte neben Ihren Dateien. Nützlich für Nebenaufgaben.

Wenn Claude eine Registerkarte in einer neuen Editor-Gruppe öffnet, sperrt die Erweiterung diese Gruppe, sodass Dateien, die Sie öffnen, während die Claude-Registerkarte fokussiert ist, stattdessen in eine andere Gruppe gehen.

Um zu verhindern, dass die Erweiterung Gruppen sperrt, deaktivieren Sie die [Einstellung „Lock Editor Groups"](vscode://settings/claudeCode.lockEditorGroups). Gruppen, die bereits gesperrt sind, bleiben gesperrt, bis Sie sie entsperren. Die Einstellung erfordert Claude Code v2.1.274 oder später.

<Tip>
  Verwenden Sie die Seitenleiste für Ihre Haupt-Claude-Sitzung und öffnen Sie zusätzliche Registerkarten für Nebenaufgaben. Claude merkt sich Ihren bevorzugten Ort. Das Symbol der Sitzungsliste in der Aktivitätsleiste ist separat vom Claude-Panel: Die Sitzungsliste ist immer in der Aktivitätsleiste sichtbar, während das Claude-Panel-Symbol nur dort angezeigt wird, wenn das Panel an der linken Seitenleiste angedockt ist.
</Tip>

Nachdem Sie **Developer: Reload Window** ausgeführt oder VS Code neu gestartet haben, hängt es davon ab, wo das Gespräch offen war, ob es mit seiner Konversation zurückkommt:

* **Editor-Registerkarte**: Das Gespräch kommt mit seiner Registerkarte zurück.
* **Seitenleiste**: Das Gespräch kommt zurück, wenn Sie eine Nachricht gesendet oder Claude darin geantwortet hat, innerhalb der letzten 10 Minuten. Wenn es nicht zurückkommt, setzen Sie das Gespräch aus [Sitzungsverlauf](#resume-past-conversations) fort.

Wenn das Neuladen Claude mitten in einem Schritt unterbrochen hat, setzt Claude diesen Schritt fort, wenn das Gespräch zurückkommt, und ein Hinweis im Chat markiert die Fortsetzung. Erfordert Claude Code v2.1.274 oder später. Wenn der Schritt vor mehr als einer Stunde unterbrochen wurde oder die Sitzung an anderer Stelle offen ist, kommt das Gespräch stattdessen im Leerlauf zurück.

Um die Fortsetzung auszuschalten, öffnen Sie die [Einstellung „Continue After Reload"](vscode://settings/claudeCode.continueAfterReload) und deaktivieren Sie sie.

<h3 id="run-multiple-conversations">
  Führen Sie mehrere Gespräche
</h3>

Verwenden Sie **Open in New Tab** oder **Open in New Window** aus der Befehlspalette, um zusätzliche Gespräche zu starten. Jedes Gespräch behält seine eigene Verlauf und seinen eigenen Kontext bei, sodass Sie parallel an verschiedenen Aufgaben arbeiten können.

Bei Verwendung von Registerkarten zeigt ein kleiner farbiger Punkt auf dem Funken-Symbol den Status an: Blau bedeutet, dass eine Berechtigungsanfrage ausstehend ist, Orange bedeutet, dass Claude fertig ist, während die Registerkarte ausgeblendet war.

<h3 id="organize-sessions-into-groups">
  Organisieren Sie Sitzungen in Gruppen
</h3>

In der Sitzungsliste in der Aktivitätsleiste können Sie verwandte Sitzungen in benannte, einklappbare Gruppen sammeln. Erfordert Claude Code v2.1.229 oder später.

* **Gruppieren oder Gruppierung aufheben einer Sitzung**: Klicken Sie mit der rechten Maustaste auf eine Sitzung, um eine Gruppe daraus zu erstellen, sie in eine vorhandene Gruppe zu verschieben oder sie aus ihrer Gruppe zu entfernen. Jede Sitzung gehört jeweils zu einer Gruppe, daher wird sie durch das Verschieben in eine andere Gruppe aus der ersten entfernt.
* **Verschieben Sie mehrere Sitzungen gleichzeitig**: `Cmd`-Klick (Mac) / `Ctrl`-Klick (Windows/Linux) auf jede Sitzung, oder `Shift`-Klick, um einen Bereich auszuwählen, dann Rechtsklick auf die Auswahl.
* **Gruppieren Sie eine Sitzung von ihrer Registerkarte**: Führen Sie **Claude Code: Add Session Tab to Group** aus der Befehlspalette aus, und wählen Sie dann eine Gruppe aus oder erstellen Sie eine. Erfordert Claude Code v2.1.257 oder später.
* **Benennen Sie eine Gruppe um oder löschen Sie sie**: Klicken Sie mit der rechten Maustaste auf einen Gruppenkopf. Das Löschen einer Gruppe entfernt nur die Gruppe, und ihre Sitzungen kehren zur ungruppierten Liste zurück.

Die Erweiterung speichert Gruppen pro Arbeitsbereichsordner, sodass sie Fenster-Neuladen überstehen und in jedem Fenster angezeigt werden, in dem Sie denselben Ordner öffnen. Wenn Sie die Liste durchsuchen, zeigt die Erweiterung Übereinstimmungen in einer flachen Liste über alle Gruppen hinweg an.

<h3 id="switch-to-terminal-mode">
  Wechseln Sie zum Terminalmodus
</h3>

Standardmäßig öffnet die Erweiterung ein grafisches Chat-Panel. Wenn Sie die CLI-ähnliche Schnittstelle bevorzugen, öffnen Sie die [Einstellung „Use Terminal"](vscode://settings/claudeCode.useTerminal) und aktivieren Sie das Kontrollkästchen.

Sie können auch VS Code-Einstellungen öffnen (`Cmd+,` auf Mac oder `Ctrl+,` unter Windows/Linux), zu Erweiterungen → Claude Code gehen und **Use Terminal** aktivieren.

<h2 id="manage-plugins">
  Plugins verwalten
</h2>

Die VS Code-Erweiterung enthält eine grafische Benutzeroberfläche zum Installieren und Verwalten von [Plugins](/docs/de/plugins/overview). Geben Sie `/plugins` in das Eingabefeld ein, um die Schnittstelle **Plugins verwalten** zu öffnen.

<h3 id="install-plugins">
  Plugins installieren
</h3>

Der Plugin-Dialog zeigt zwei Registerkarten: **Plugins** und **Marketplaces**.

Auf der Registerkarte Plugins:

* **Installierte Plugins** werden oben mit Umschaltern angezeigt, um sie zu aktivieren oder zu deaktivieren
* **Verfügbare Plugins** aus Ihren konfigurierten Marketplaces werden darunter angezeigt
* Suchen Sie, um Plugins nach Name oder Beschreibung zu filtern
* Klicken Sie auf **Installieren** für jedes verfügbare Plugin

Wenn Sie ein Plugin installieren, wählen Sie den Installationsbereich:

* **Für Sie installieren**: verfügbar in allen Ihren Projekten (Benutzerbereich)
* **Für dieses Projekt installieren**: geteilt mit Projektmitarbeitern (Projektbereich)
* **Lokal installieren**: nur für Sie, nur in diesem Repository (lokaler Bereich)

<h3 id="share-a-plugin-install-link">
  Plugin-Installationslink teilen
</h3>

Um jemanden direkt zur Installation eines bestimmten Plugins zu führen, geben Sie ihm die `install-plugin`-URL der Erweiterung. Das Öffnen startet oder fokussiert VS Code, öffnet das Claude Code-Panel und öffnet den Dialog **Plugins verwalten** mit der Bereichswahl für dieses Plugin. Nichts wird installiert, bis die Person einen Bereich auswählt. Wenn der Marketplace des Plugins in Claude Code noch nicht konfiguriert ist, fragt der Dialog zuerst, ob er hinzugefügt werden soll.

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

Die URL akzeptiert zwei Abfrageparameter:

| Parameter     | Beschreibung                                                                                                                                                                                              |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugin`      | Der Name des Plugins, wie er im Marketplace aufgelistet ist. Erforderlich.                                                                                                                                |
| `marketplace` | Woher das Plugin kommt: ein GitHub `owner/repo`, eine `https://`-URL oder eine Git-SSH-URL wie `git@github.com:owner/repo.git`. Standardmäßig `anthropics/claude-plugins-official`, wenn nicht angegeben. |

Einige Werte, die die [Registerkarte Marketplaces](#manage-marketplaces) akzeptiert, funktionieren nicht in einem Link, wie z. B. ein lokaler Pfad oder eine `http://`-Adresse. Für diese zeigt VS Code eine Fehlermeldung an und der Dialog wird nicht geöffnet.

Zwei Fälle enden mit einer Nachricht im Dialog statt der Bereichswahl:

* **Der Marketplace listet kein Plugin mit diesem Namen auf**: Der Dialog meldet, dass das Plugin nicht gefunden wurde. Überprüfen Sie den `plugin`-Wert anhand der Auflistung des Marketplace.
* **Das Plugin ist bereits installiert**: Der Dialog teilt dies mit, und es ändert sich nichts.

GitHub-READMEs, Issues und einige andere Markdown-Hosts entfernen Links, deren Schema nicht `http` oder `https` ist, sodass ein `vscode://`-Link dort als Klartext angezeigt wird. Platzieren Sie die URL in einem Codeblock auf diesen Hosts, wie [Der Link wird als Klartext angezeigt, anstatt anklickbar zu sein](/docs/de/deep-links#the-link-renders-as-plain-text-instead-of-being-clickable) für `claude-cli://`-Links beschreibt.

<h3 id="manage-marketplaces">
  Marketplaces verwalten
</h3>

Wechseln Sie zur Registerkarte **Marketplaces**, um Plugin-Quellen hinzuzufügen oder zu entfernen:

* Geben Sie ein GitHub-Repository, eine URL oder einen lokalen Pfad ein, um einen neuen Marketplace hinzuzufügen
* Klicken Sie auf das Aktualisierungssymbol, um die Plugin-Liste eines Marketplace zu aktualisieren
* Klicken Sie auf das Papierkorbsymbol, um einen Marketplace zu entfernen

Plugin-Änderungen, die Sie im Dialog vornehmen, werden sofort auf die Claude Code-Sitzungen angewendet, die in diesem VS Code-Fenster geöffnet sind. Wenn die Sitzung, aus der Sie den Dialog geöffnet haben, ihre Plugins nicht neu laden kann, bietet der Dialog an, es erneut zu versuchen oder Claude in dieser Sitzung neu zu starten.

<Note>
  Die Plugin-Verwaltung in VS Code verwendet unter der Haube die gleichen CLI-Befehle. Plugins und Marketplaces, die Sie in der Erweiterung konfigurieren, sind auch in der CLI verfügbar, und umgekehrt.
</Note>

Weitere Informationen zum Plugin-System finden Sie unter [Plugins](/docs/de/plugins/overview) und [Plugin-Marketplaces](/docs/de/plugins/overview).

<h2 id="automate-browser-tasks-with-chrome">
  Browser-Aufgaben mit Chrome automatisieren
</h2>

Verbinden Sie Claude mit Ihrem Chrome-Browser, um Web-Apps zu testen, mit Konsolenprotokollen zu debuggen und Browser-Workflows zu automatisieren, ohne VS Code zu verlassen. Dies erfordert die [Claude in Chrome-Erweiterung](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) Version 1.0.36 oder höher.

Geben Sie `@browser` in das Eingabefeld ein, gefolgt von dem, was Claude tun soll:

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

Sie können auch das Anlagemenü öffnen, um spezifische Browser-Tools auszuwählen, wie das Öffnen eines neuen Tabs oder das Lesen von Seiteninhalten.

Claude öffnet neue Tabs für Browser-Aufgaben und teilt den Anmeldestatus Ihres Browsers, sodass es auf jede Website zugreifen kann, bei der Sie bereits angemeldet sind.

Anweisungen zur Einrichtung, die vollständige Liste der Funktionen und Fehlerbehebung finden Sie unter [Claude Code mit Chrome verwenden](/docs/de/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  VS Code-Befehle und Tastenkombinationen
</h2>

Öffnen Sie die Befehlspalette (`Cmd+Shift+P` auf Mac oder `Ctrl+Shift+P` unter Windows/Linux) und geben Sie „Claude Code" ein, um alle verfügbaren VS Code-Befehle für die Claude Code-Erweiterung anzuzeigen.

Einige Tastenkombinationen hängen davon ab, welches Panel „fokussiert" ist (Tastatureingaben empfängt). Wenn sich der Cursor in einer Codedatei befindet, ist der Editor fokussiert. Wenn sich der Cursor in Claudes Eingabefeld befindet, ist Claude fokussiert. Verwenden Sie `Cmd+Esc` / `Ctrl+Esc`, um zwischen ihnen zu wechseln.

<Note>
  Dies sind VS Code-Befehle zur Steuerung der Erweiterung. Nicht alle integrierten Claude Code-Befehle sind in der Erweiterung verfügbar. Weitere Informationen finden Sie unter [VS Code-Erweiterung vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli).
</Note>

| Befehl                     | Tastenkombination                                        | Beschreibung                                                                                                                                                                                                                                                                                                           |
| -------------------------- | -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Fokus zwischen Editor und Claude umschalten                                                                                                                                                                                                                                                                            |
| Focus last message         | -                                                        | Verschieben Sie den Tastaturfokus auf die neueste Nachricht in der Konversation oder auf eine ausstehende Genehmigungsaufforderung, damit Sie diese mit der Tastatur oder einem Bildschirmleser lesen können. Nicht verfügbar im [Terminalmodus](#switch-to-terminal-mode). Erfordert Claude Code v2.1.268 oder später |
| Open in Side Bar           | -                                                        | Claude in der Seitenleiste öffnen                                                                                                                                                                                                                                                                                      |
| Open in Terminal           | -                                                        | Claude im Terminalmodus öffnen                                                                                                                                                                                                                                                                                         |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Ein neues Gespräch als Editor-Registerkarte öffnen                                                                                                                                                                                                                                                                     |
| Open in New Window         | -                                                        | Ein neues Gespräch in einem separaten Fenster öffnen                                                                                                                                                                                                                                                                   |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Ein neues Gespräch starten. Erfordert, dass Claude fokussiert ist und `enableNewConversationShortcut` auf `true` gesetzt ist                                                                                                                                                                                           |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Öffnen Sie die zuletzt geschlossene Claude-Sitzungsregisterkarte erneut. Fällt auf VS Codes normales Verhalten zum Wiedereröffnen geschlossener Editoren zurück, wenn die zuletzt geschlossene Registerkarte keine Claude-Sitzung war. Deaktivieren Sie mit `enableReopenClosedSessionShortcut`                        |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Fügen Sie einen Verweis auf die aktuelle Datei und Auswahl ein (erfordert, dass der Editor fokussiert ist)                                                                                                                                                                                                             |
| Accept Change at Cursor    | -                                                        | Akzeptieren Sie die Änderung am Cursor, während Sie [eine vorgeschlagene Bearbeitung überprüfen](#get-started), eine Änderung nach der anderen. Erfordert Claude Code v2.1.275 oder später                                                                                                                             |
| Reject Change at Cursor    | -                                                        | Machen Sie die Änderung am Cursor rückgängig, während Sie eine vorgeschlagene Bearbeitung überprüfen, eine Änderung nach der anderen. Erfordert Claude Code v2.1.275 oder später                                                                                                                                       |
| Toggle Focus view          | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)     | Werkzeugaktivität im Gespräch ausblenden oder anzeigen. Funktioniert, während ein Claude-Panel oder eine Seitenleiste sichtbar ist. Erfordert Claude Code v2.1.221 oder später                                                                                                                                         |
| Rename Session Tab         | -                                                        | Benennen Sie die Sitzung in der aktiven Claude-Registerkarte um. Erfordert Claude Code v2.1.257 oder später                                                                                                                                                                                                            |
| Add Session Tab to Group   | -                                                        | Fügen Sie die Sitzung in der aktiven Claude-Registerkarte zu einer [Sitzungsgruppe](#organize-sessions-into-groups) hinzu, die Sie auswählen oder erstellen. Erfordert Claude Code v2.1.257 oder später                                                                                                                |
| Mark Session as Unread     | -                                                        | Markieren Sie die Sitzung in der aktiven Claude-Registerkarte als ungelesen in der Sitzungsliste. Erfordert Claude Code v2.1.257 oder später                                                                                                                                                                           |
| Show Logs                  | -                                                        | Erweiterungs-Debug-Protokolle anzeigen                                                                                                                                                                                                                                                                                 |
| Logout                     | -                                                        | Melden Sie sich von Ihrem Anthropic-Konto ab                                                                                                                                                                                                                                                                           |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Starten Sie eine VS Code-Registerkarte von anderen Tools aus
</h3>

Die Erweiterung registriert einen URI-Handler unter `vscode://anthropic.claude-code/open`. Verwenden Sie ihn, um eine neue Claude Code-Registerkarte von Ihren eigenen Tools aus zu öffnen: ein Shell-Alias, ein Browser-Bookmarklet oder ein beliebiges Skript, das eine URL öffnen kann. Wenn VS Code nicht bereits ausgeführt wird, wird es beim Öffnen der URL zuerst gestartet. Wenn VS Code bereits ausgeführt wird, wird die URL in dem Fenster geöffnet, das derzeit fokussiert ist.

Rufen Sie den Handler mit dem URL-Öffner Ihres Betriebssystems auf.

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

    Der Befehl `xdg-open` stammt aus dem Paket `xdg-utils`. Wenn die Shell meldet, dass er nicht gefunden wird, siehe [xdg-open is not found on Linux](/docs/de/deep-links#xdg-open-is-not-found-on-linux).
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

| Parameter | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Text zum Vorausfüllen des Eingabefelds. Muss URL-codiert sein. Das Eingabefeld wird vorausgefüllt, aber nicht automatisch übermittelt.                                                                                                                                                                                                                                                                                                                              |
| `session` | Eine Sitzungs-ID zum Fortsetzen statt zum Starten eines neuen Gesprächs. Die Sitzung muss zum derzeit in VS Code geöffneten Arbeitsbereich gehören. Wenn die Sitzung nicht gefunden wird, wird stattdessen ein neues Gespräch gestartet. Wenn die Sitzung bereits in einer Registerkarte geöffnet ist, wird diese Registerkarte fokussiert. Um eine Sitzungs-ID programmgesteuert zu erfassen, siehe [Continue conversations](/docs/de/headless#continue-conversations). |

Um beispielsweise eine Registerkarte mit „review my changes" vorausgefüllt zu öffnen:

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

Die Erweiterung verarbeitet auch `vscode://anthropic.claude-code/install-plugin`, das [den Plugin-Dialog für ein Plugin öffnet](#share-a-plugin-install-link). Um stattdessen eine Terminalsitzung zu starten, verwenden Sie den CLI-Handler `claude-cli://`. Siehe [Launch sessions from links](/docs/de/deep-links).

<h2 id="configure-settings">
  Einstellungen konfigurieren
</h2>

Die Erweiterung hat zwei Arten von Einstellungen:

* **Erweiterungseinstellungen** in VS Code: steuern das Verhalten der Erweiterung in VS Code. Öffnen Sie sie mit `Cmd+,` (Mac) oder `Strg+,` (Windows/Linux), gehen Sie dann zu Erweiterungen → Claude Code. Sie können auch `/` eingeben und **General config…** auswählen, um die Einstellungen zu öffnen.
* **Claude Code-Einstellungen** in `~/.claude/settings.json`: werden zwischen der Erweiterung und der CLI gemeinsam genutzt. Verwenden Sie diese für zulässige Befehle, Umgebungsvariablen, Hooks und MCP-Server. In Pro-, Max- und Team-Plänen ist dies auch eine Eingabe für den Berechtigungsmodus, in dem Gespräche beginnen. [Berechtigungsmodi wechseln](/docs/de/permission-modes#switch-permission-modes) listet die Reihenfolge auf. Weitere Informationen finden Sie unter [Einstellungen](/docs/de/settings).

<Tip>
  Fügen Sie `"$schema": "https://json.schemastore.org/claude-code-settings.json"` zu Ihrer `settings.json` hinzu, um Autovervollständigung und Inline-Validierung für alle verfügbaren Einstellungen direkt in VS Code zu erhalten.
</Tip>

<h3 id="extension-settings">
  Erweiterungseinstellungen
</h3>

VS Code liest `initialPermissionMode` aus Ihren Benutzereinstellungen und ignoriert Workspace-Werte. Vor v2.1.225 setzte VS Code die Einstellung standardmäßig auf `default` und wendete Workspace-Werte an.

| Einstellung                         | Standard | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ----------------------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false`  | Starten Sie Claude im Terminalmodus statt im grafischen Panel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `initialPermissionMode`             | -        | Steuert Genehmigungsaufforderungen für neue Gespräche: `default`, `plan`, `acceptEdits` oder `bypassPermissions`. `manual` ist ein Alias für `default` und wählt den Modus aus, der im Modusindikator als **Manual** gekennzeichnet ist. Wenn Sie diese Option nicht festlegen, wählt die Erweiterung den Startberechtigungsmodus wie in [Berechtigungsmodi wechseln](/docs/de/permission-modes#switch-permission-modes) beschrieben.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `preferredLocation`                 | `panel`  | Wo Claude geöffnet wird: `sidebar` (rechts) oder `panel` (neue Registerkarte)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `lockEditorGroups`                  | `true`   | [Sperren Sie die Editor-Gruppen, die Claude für seine Registerkarten startet](#choose-where-claude-lives), sodass Dateien, die Sie öffnen, während eine Claude-Registerkarte fokussiert ist, in eine andere Gruppe gehen. Wenn diese Option deaktiviert ist, sperrt die Erweiterung niemals eine Editor-Gruppe. Erfordert Claude Code v2.1.274 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `autosave`                          | `true`   | Dateien automatisch speichern, bevor Claude sie liest oder schreibt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `attachOpenFile`                    | `true`   | Fügen Sie die im Editor geöffnete Datei zu Ihren Nachrichten hinzu und zeigen Sie sie im Eingabefeld an. Wenn diese Option deaktiviert ist, wird nur Ihr ausgewählter Text hinzugefügt. Erfordert Claude Code v2.1.271 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `useCtrlEnterToSend`                | `false`  | Verwenden Sie Strg/Cmd+Eingabe statt Eingabe zum Senden von Eingabeaufforderungen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `scrollToBottomOnSend`              | `true`   | Scrollen Sie das Gespräch nach unten, wenn Sie eine Nachricht senden. Wenn diese Option deaktiviert ist, bleibt das Gespräch dort, wo Sie es verlassen haben. Erfordert Claude Code v2.1.275 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `enableNewConversationShortcut`     | `false`  | Aktivieren Sie Cmd/Strg+N, um ein neues Gespräch zu starten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `enableReopenClosedSessionShortcut` | `true`   | Verwenden Sie Cmd/Strg+Umschalt+T, um die zuletzt geschlossene Claude-Sitzungsregisterkarte erneut zu öffnen. Wenn die zuletzt geschlossene Registerkarte keine Claude-Sitzung war, führt die Tastenkombination stattdessen den normalen Befehl zum erneuten Öffnen des geschlossenen Editors von VS Code aus.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `archiveInactiveSessions`           | `14`     | [Archivieren Sie eine Sitzung automatisch](#resume-past-conversations) nach dieser Anzahl von Tagen ohne Aktivität: `1`, `2`, `7` oder `14`. Setzen Sie `0`, um dies auszuschalten. Erfordert Claude Code v2.1.265 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `continueAfterReload`               | `true`   | Nach einem Fenster-Reload setzt Claude [den unterbrochenen Schritt fort](#choose-where-claude-lives) in der wiederhergestellten Sitzung. Erfordert Claude Code v2.1.274 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `hideOnboarding`                    | `false`  | Blenden Sie die Onboarding-Checkliste aus (Abschlusskappe-Symbol)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `focusView`                         | `false`  | Blenden Sie Werkzeugaufrufe, Werkzeugergebnisse und Überlegungen hinter erweiterbaren Zeilen aus, sodass nur Ihre Eingabeaufforderungen und Claudes Antworten sichtbar bleiben. Claudes neueste To-Do-Liste bleibt sichtbar; dies erfordert Claude Code v2.1.225 oder später. Sie können die Fokusansicht auch über das Befehlsmenü umschalten. Erfordert Claude Code v2.1.221 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `respectGitIgnore`                  | `true`   | Schließen Sie .gitignore-Muster aus Dateisuchvorgängen und aus [Auswahlkontext](#reference-files-and-folders) aus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `usePythonEnvironment`              | `true`   | Aktivieren Sie die Python-Umgebung des Workspace beim Ausführen von Claude. Erfordert die Python-Erweiterung.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `environmentVariables`              | `[]`     | Legen Sie Umgebungsvariablen für den Claude-Prozess fest. Verwenden Sie stattdessen Claude Code-Einstellungen für gemeinsame Konfiguration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disableLoginPrompt`                | `false`  | Überspringen Sie Authentifizierungsaufforderungen (für Setups von Drittanbieter-Providern)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `allowDangerouslySkipPermissions`   | `false`  | Fügt dem Moduswahlschalter die Option „Berechtigungen umgehen" hinzu. Verwenden Sie dies nur in Sandboxes ohne Internetzugang.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `claudeProcessWrapper`              | -        | Ausführbare Datei zum Starten des Claude-Prozesses. Der Pfad der gebündelten Binärdatei wird als Argument übergeben, wenn vorhanden. Legen Sie dies auf eine separat installierte `claude`-Binärdatei fest, wenn der Erweiterungsbuild keine für Ihre Plattform enthält. In einem umschlossenen Setup beginnen Gespräche im Manusmodus, es sei denn, Sie legen `initialPermissionMode` fest oder haben in einem früheren Gespräch „Manuell", „Automatisch bearbeiten" oder „Automatisch" ausgewählt, da die Erweiterung die Einstellungen und integrierten Standardschritte dort überspringt; siehe [Berechtigungsmodi wechseln](/docs/de/permission-modes#switch-permission-modes). Ein Fehler „Unsupported platform" bei der Aktivierung bedeutet, dass keine Binärdatei für Ihre Plattform gebündelt ist; siehe [welche Plattformen vorkompilierte Binärdateien haben](/docs/de/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="use-a-screen-reader">
  Verwenden Sie einen Bildschirmleser
</h2>

Das Chat-Panel der Erweiterung funktioniert mit Bildschirmlesern. Sie müssen nichts aktivieren: Die Erweiterung kündigt Gesprächsaktivitäten für jeden Benutzer an, ohne visuelle Änderungen. Dies ist unabhängig vom [Bildschirmlesermodus](/docs/de/accessibility) der CLI, der sich aktivieren lässt und die Terminaloberfläche anpasst.

Die Unterstützung für Bildschirmleser im Chat-Panel erfordert Claude Code v2.1.236 oder später.

Während eines Gesprächs kündigt die Erweiterung an:

* **Antworten von Claude**: Die Erweiterung kündigt jede Antwort einmal an, wenn sie vollständig ist, und bleibt stumm, während Text einströmt. Ihr Bildschirmleser liest Code-Blöcke als Zeilenzahl-Zusammenfassung, liest Links nach ihrem Label und liest Tabellen Zelle für Zelle; die vollständige Antwort bleibt im Transkript lesbar.
* **Berechtigungsanfragen und Fragen**: Die Erweiterung kündigt eine Anfrage an, wenn die Berechtigungsaufforderung angezeigt wird, und nennt das Tool, das Claude verwenden möchte. Sie kündigt auf die gleiche Weise an, wenn Claude Ihnen eine Frage stellt und wenn Claude einen Plan abgeschlossen hat und auf Ihre Überprüfung wartet.
* **Statusänderungen**: Die Erweiterung kündigt an, wenn Claude mit der Arbeit beginnt, wenn Claude bereit für Ihre Eingabe ist, und wenn Claude Code das Gespräch komprimiert.
* **Fehler und Modell-Aufforderungen**: Die Erweiterung kündigt Fehler im Gespräch an und kündigt an, wenn die [Aufforderung zur Zustimmung für Nutzungsguthaben](/docs/de/model-config#fable-and-usage-credits) oder die [Aufforderung für gekennzeichnete Anfragen](/docs/de/model-config#ask-before-switching) angezeigt wird.

Während Claude arbeitet, liest Ihr Bildschirmleser ein Textlabel anstelle der Animation des Fortschrittsspinners.

Wenn Sie eine Sitzung erneut öffnen oder zu einer anderen wechseln, kündigt die Erweiterung nichts an: Wiederhergestellter Verlauf, ausstehende Berechtigungsaufforderungen und laufender Status bleiben stumm, bis etwas Neues geschieht.

<h3 id="use-the-chat-panel-from-the-keyboard">
  Verwenden Sie das Chat-Panel von der Tastatur aus
</h3>

Jede Runde im Transkript beginnt mit einer visuell verborgenen Überschrift, die mit der Eingabeaufforderung gekennzeichnet ist, die die Runde gestartet hat, sodass Sie mit der Überschriftsnavigation Ihres Bildschirmlesers zwischen Runden springen können.

Innerhalb einer Runde kündigt Ihr Bildschirmleser an, wessen Nachricht Sie lesen, während Sie sich durch sie bewegen:

* **Ihre Nachrichten**: „You"
* **Nachrichten von Claude**: „Claude"
* **Tool-Schritte**: „Claude" plus der Tool-Name, z. B. „Claude, Bash"
* **Thinking-Blöcke**: „Claude, thinking"

Da die Erweiterung das Transkript als gekennzeichnete Region verfügbar macht, können Sie den Fokus auch mit `Tab` auf das Transkript selbst verschieben und es in Ihrem eigenen Tempo lesen. Um den Fokus stattdessen auf die neueste Nachricht oder eine ausstehende Berechtigungsaufforderung zu verschieben, führen Sie **Claude Code: Focus last message** aus der [Befehlspalette](#vs-code-commands-and-shortcuts) aus.

Wenn eine Option in einer Berechtigungsaufforderung eine Berechtigungsregel oder einen Verzeichniszugriff speichert, endet ihr Label mit der Angabe, wo die Genehmigung gespeichert ist, z. B. „all projects" oder „this session". Wenn diese Option fokussiert ist, drücken Sie die `Left`- oder `Right`-Pfeiltaste, um das Ziel zu ändern, und die Erweiterung kündigt jedes Ziel an, wenn Sie es erreichen. Sie können das Ziel auch im Label anklicken. Die Pfeiltasten erfordern Claude Code v2.1.268 oder später.

<h2 id="vs-code-extension-vs-claude-code-cli">
  VS Code-Erweiterung vs. Claude Code CLI
</h2>

Claude Code ist sowohl als VS Code-Erweiterung (grafisches Panel) als auch als CLI (Befehlszeilenschnittstelle im Terminal) verfügbar. Einige Funktionen sind nur in der CLI verfügbar. Wenn Sie eine nur in der CLI verfügbare Funktion benötigen, führen Sie `claude` im integrierten Terminal von VS Code aus. Dies erfordert die [eigenständige CLI-Installation](/docs/de/setup): Die Erweiterung fügt `claude` nicht zu Ihrem PATH hinzu. Siehe [CLI in VS Code ausführen](#run-cli-in-vs-code).

| Funktion                | CLI                  | VS Code-Erweiterung                                                                                  |
| ----------------------- | -------------------- | ---------------------------------------------------------------------------------------------------- |
| Befehle und Skills      | [Alle](/docs/de/commands) | Teilmenge (geben Sie `/` ein, um verfügbare anzuzeigen)                                              |
| MCP-Serverkonfiguration | Ja                   | Ja ([Server hinzufügen und verwalten](#connect-to-external-tools-with-mcp) mit `/mcp` im Chat-Panel) |
| Checkpoints             | Ja                   | Ja                                                                                                   |
| `!` Bash-Verknüpfung    | Ja                   | Nein                                                                                                 |
| Tab-Vervollständigung   | Ja                   | Nein                                                                                                 |

<h3 id="rewind-with-checkpoints">
  Mit Checkpoints zurückspulen
</h3>

Die VS Code-Erweiterung unterstützt Checkpoints, die Claudes Dateibearbeitungen verfolgen und es Ihnen ermöglichen, zu einem vorherigen Zustand zurückzuspulen. Bewegen Sie den Mauszeiger über eine beliebige Nachricht, um die Schaltfläche zum Zurückspulen anzuzeigen, und wählen Sie dann aus drei Optionen:

* **Konversation von hier aus verzweigen**: Starten Sie einen neuen Konversationszweig von dieser Nachricht aus, während alle Codeänderungen erhalten bleiben
* **Code bis hier zurückspulen**: Setzen Sie Dateiänderungen auf diesen Punkt in der Konversation zurück, während Sie die vollständige Konversationshistorie behalten
* **Konversation verzweigen und Code zurückspulen**: Starten Sie einen neuen Konversationszweig und setzen Sie Dateiänderungen auf diesen Punkt zurück

Vollständige Details zur Funktionsweise von Checkpoints und deren Einschränkungen finden Sie unter [Checkpointing](/docs/de/checkpointing).

<h3 id="run-cli-in-vs-code">
  CLI in VS Code ausführen
</h3>

Um die CLI zu verwenden und dabei in VS Code zu bleiben, öffnen Sie das integrierte Terminal (`` Ctrl+` `` unter Windows/Linux oder `` Cmd+` `` auf Mac) und führen Sie `claude` aus. Die CLI wird automatisch mit Ihrer IDE für Funktionen wie Diff-Anzeige und Diagnosefreigabe integriert.

Die Installation der Erweiterung fügt `claude` nicht zu Ihrem Shell-PATH hinzu. Die Erweiterung enthält eine private Kopie der CLI für ihr Chat-Panel, aber die Eingabe von `claude` in einem Terminal erfordert die [eigenständige CLI-Installation](/docs/de/setup). Führen Sie die Installation einmal aus und die Befehle auf dieser Seite, einschließlich `claude mcp add` und `claude --resume`, funktionieren in jedem Terminal. Wenn `claude` nach der Installation immer noch nicht gefunden wird, [überprüfen Sie Ihren PATH](/docs/de/troubleshoot-install#verify-your-path).

Wenn Sie ein externes Terminal verwenden, führen Sie `/ide` in Claude Code aus, um es mit VS Code zu verbinden.

<h3 id="switch-between-extension-and-cli">
  Zwischen Erweiterung und CLI wechseln
</h3>

Die Erweiterung und die CLI teilen die gleiche Konversationshistorie. Um eine Erweiterungskonversation in der CLI fortzusetzen, führen Sie `claude --resume` im Terminal aus. Dies öffnet eine interaktive Auswahl, in der Sie Ihre Konversation suchen und auswählen können.

<h3 id="include-terminal-output-in-prompts">
  Terminalausgabe in Eingabeaufforderungen einbeziehen
</h3>

Referenzieren Sie Terminalausgabe in Ihren Eingabeaufforderungen mit `@terminal:name`, wobei `name` der Titel des Terminals ist. Dies ermöglicht Claude, Befehlsausgabe, Fehlermeldungen oder Protokolle zu sehen, ohne sie zu kopieren und einzufügen.

<h3 id="monitor-background-processes">
  Hintergrundprozesse überwachen
</h3>

Geben Sie `/tasks` im Eingabefeld ein, um die [Agent-Karte](#use-the-prompt-box) zu öffnen, die die Hintergrundaufgaben der Sitzung auflistet, z. B. einen Dev-Server, den Claude als Hintergrund-Shell-Befehl ausführt. Klicken Sie auf eine Aufgabe, um ihre Karte zu öffnen und sie dort zu beenden. Erfordert Claude Code v2.1.277 oder später.

<h3 id="connect-to-external-tools-with-mcp">
  Mit MCP eine Verbindung zu externen Tools herstellen
</h3>

MCP (Model Context Protocol)-Server geben Claude Zugriff auf externe Tools, Datenbanken und APIs.

Um MCP-Server zu verwalten, ohne VS Code zu verlassen, geben Sie `/mcp` im Chat-Panel ein. Im angezeigten Dialog können Sie Server hinzufügen, auf lokaler, Benutzer- oder Projekt-[Ebene](/docs/de/mcp#mcp-installation-scopes) gespeicherte Server entfernen, Server aktivieren oder deaktivieren, sich erneut mit einem Server verbinden und die OAuth-Authentifizierung verwalten. Das Hinzufügen und Entfernen von Servern im Dialog erfordert Claude Code v2.1.261 oder später.

Sie können auch `claude mcp add` im integrierten Terminal von VS Code ausführen (`` Ctrl+` `` oder `` Cmd+` ``). Der Dialog und der Terminal-Befehl speichern in der gleichen MCP-Konfiguration, und Änderungen von beiden werden in Konversationen wirksam, die Sie danach starten. Das folgende Beispiel fügt Githubs Remote-MCP-Server hinzu, der sich mit einem [persönlichen Zugriffstoken](https://github.com/settings/personal-access-tokens) authentifiziert, das als Header übergeben wird:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Ersetzen Sie `YOUR_GITHUB_PAT` durch Ihr persönliches Zugriffstoken. Der Befehl `claude mcp add` speichert die Konfiguration, ohne Anmeldedaten zu validieren, daher wird hier ein Platzhalterwert akzeptiert, aber der Server kann sich später nicht verbinden. Um die Verbindung zu überprüfen, starten Sie eine neue Konversation, geben Sie `/mcp` ein und überprüfen Sie, dass der Server **Connected** anzeigt. Ein Server mit ungültigen Anmeldedaten zeigt **Failed**.

Nach der Konfiguration bitten Sie Claude, die Tools zu verwenden (z. B. „Review PR #456").

Um Server zum Verbinden zu finden, siehe [MCP-Server suchen und erstellen](/docs/de/mcp#find-and-build-mcp-servers).

<h2 id="work-with-git">
  Mit git arbeiten
</h2>

Claude Code ist in git integriert, um Versionskontroll-Workflows direkt in VS Code zu unterstützen. Bitten Sie Claude, Änderungen zu committen, Pull Requests zu erstellen oder über Branches hinweg zu arbeiten. Um Claude in einem isolierten Worktree mit eigenen Dateien und Branch zu starten, siehe [Parallele Sitzungen mit Worktrees ausführen](/docs/de/worktrees).

<h3 id="create-commits-and-pull-requests">
  Commits und Pull Requests erstellen
</h3>

Claude kann Änderungen bereitstellen, Commit-Nachrichten schreiben und Pull Requests basierend auf Ihrer Arbeit erstellen:

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

Beim Erstellen von Pull Requests generiert Claude Beschreibungen basierend auf den tatsächlichen Code-Änderungen und kann Kontext zu Tests oder Implementierungsentscheidungen hinzufügen.

<h2 id="use-third-party-providers">
  Verwenden Sie Drittanbieter
</h2>

Standardmäßig verbindet sich Claude Code direkt mit der API von Anthropic. Wenn Ihre Organisation Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry nutzt, um auf Claude zuzugreifen, konfigurieren Sie die Erweiterung, um stattdessen Ihren Anbieter zu verwenden:

<Steps>
  <Step title="Anmeldeeingabeaufforderung deaktivieren">
    Öffnen Sie die [Einstellung „Anmeldeeingabeaufforderung deaktivieren"](vscode://settings/claudeCode.disableLoginPrompt) und aktivieren Sie das Kontrollkästchen.

    Sie können auch die VS Code-Einstellungen öffnen (`Cmd+,` auf Mac oder `Ctrl+,` unter Windows/Linux), nach „Claude Code login" suchen und **Anmeldeeingabeaufforderung deaktivieren** aktivieren.
  </Step>

  <Step title="Konfigurieren Sie Ihren Anbieter">
    Folgen Sie dem Einrichtungsleitfaden für Ihren Anbieter:

    * [Claude Code auf Amazon Bedrock](/docs/de/amazon-bedrock)
    * [Claude Code auf Google Cloud's Agent Platform](/docs/de/google-vertex-ai)
    * [Claude Code auf Microsoft Foundry](/docs/de/microsoft-foundry)

    Diese Leitfäden behandeln die Konfiguration Ihres Anbieters in `~/.claude/settings.json`, was sicherstellt, dass Ihre Einstellungen zwischen der VS Code-Erweiterung und der CLI gemeinsam genutzt werden.
  </Step>
</Steps>

Bei einem Drittanbieter bietet die Erweiterung keine Funktionen an, die ein claude.ai-Konto erfordern, wie z. B. Nutzungsverfolgung, [Sprachdiktat](/docs/de/voice-dictation) und die Registerkarte „Web" für [Cloud-Sitzungen](#resume-cloud-sessions-from-claude-ai). Weitere Informationen zu den Inhalten des Dialogs „Konto und Nutzung" bei diesen Anmeldungen finden Sie unter [Konto und Nutzung überprüfen](#check-account-and-usage).

Eine claude.ai-Anmeldung, die von einem früheren `/login` übrig bleibt, wird nicht verwendet: Die Erweiterung sendet sie nicht mit einer Anfrage.

<h2 id="security-and-privacy">
  Sicherheit und Datenschutz
</h2>

Ihr Code bleibt privat. Claude Code verarbeitet Ihren Code, um Ihnen Unterstützung zu bieten, verwendet ihn aber nicht zum Trainieren von Modellen. Weitere Informationen zur Datenverarbeitung und zum Deaktivieren der Protokollierung finden Sie unter [Daten und Datenschutz](/docs/de/data-usage).

Wenn Auto-Edit-Berechtigungen aktiviert sind, kann Claude Code VS Code-Konfigurationsdateien (wie `settings.json` oder `tasks.json`) ändern, die VS Code möglicherweise automatisch ausführt. Um das Risiko bei der Arbeit mit nicht vertrauenswürdigem Code zu verringern:

* Aktivieren Sie den [VS Code Restricted Mode](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) für nicht vertrauenswürdige Arbeitsbereiche
* Verwenden Sie den manuellen Modus anstelle von „Edit automatically" oder „Auto" für Bearbeitungen
* Überprüfen Sie Änderungen sorgfältig, bevor Sie sie akzeptieren

<h3 id="the-built-in-ide-mcp-server">
  Der integrierte IDE-MCP-Server
</h3>

Wenn die Erweiterung aktiv ist, wird ein lokaler MCP-Server ausgeführt, mit dem sich die CLI automatisch verbindet. Auf diese Weise öffnet die CLI Diffs im nativen Diff-Viewer von VS Code, liest Ihre aktuelle Auswahl für `@`-Erwähnungen und fordert VS Code auf – wenn Sie in einem Jupyter-Notebook arbeiten – Zellen auszuführen.

Der Server heißt `ide` und ist in `/mcp` verborgen, da es nichts zu konfigurieren gibt. Wenn Ihre Organisation jedoch einen `PreToolUse`-Hook verwendet, um MCP-Tools auf eine Allowlist zu setzen, müssen Sie wissen, dass er existiert.

**Auswahl und Kontext offener Dateien.** Während der Verbindung bezieht die CLI Ihre aktuelle Editor-Auswahl und den Pfad der aktiven Datei als Kontext in jeden Prompt ein, den Sie senden. Das Transkript zeigt eine `⧉ Selected N lines from <file>`-Zeile, wenn dies geschieht.

Um eine vertrauliche Datei wie `.env` auszuschließen, fügen Sie eine [`Read`-Ablehnungsregel](/docs/de/permissions#read-and-edit) für ihren Pfad hinzu. Eine entsprechende Ablehnungsregel verhindert, dass sowohl der ausgewählte Text als auch die Benachrichtigung über die offene Datei für diese Datei Claude erreichen.

Wenn Sie die [Einstellung „Attach Open File"](#extension-settings) deaktivieren, erhält die CLI den Pfad der aktiven Datei nur, wenn Sie darin Text ausgewählt haben.

**Transport und Authentifizierung.** Der Server bindet sich an `127.0.0.1` auf einem zufälligen Port im Bereich 10000–65535, und der Port ist nicht konfigurierbar. Der Transport ist unverschlüsseltes `ws://`; da der Socket nur loopback ist, kann jeder Prozess, der den Datenverkehr erfassen kann, auch das Token aus der Lock-Datei lesen, daher würde TLS keinen zusätzlichen Schutz bieten. Jede Erweiterungsaktivierung generiert ein neues zufälliges Auth-Token, schreibt es in eine Lock-Datei unter `~/.claude/ide/<port>.lock`, und die CLI muss es als `X-Claude-Code-Ide-Authorization`-Header präsentieren, um sich zu verbinden. Die Lock-Datei hat `0600`-Berechtigungen in einem `0700`-Verzeichnis, daher kann nur der Benutzer, der VS Code ausführt, sie lesen. Wenn `CLAUDE_CONFIG_DIR` gesetzt ist, wird die Lock-Datei stattdessen in `$CLAUDE_CONFIG_DIR/ide/` geschrieben.

**Dem Modell verfügbar gemachte Tools.** Der Server hostet ein Dutzend Tools, aber nur zwei sind für das Modell sichtbar. Der Rest ist internes RPC, das die CLI für ihre eigene Benutzeroberfläche verwendet – Diffs öffnen, Auswahlen lesen, Dateien speichern – und wird gefiltert, bevor die Toolliste Claude erreicht.

| Tool-Name (wie von Hooks gesehen) | Was es tut                                                                                                                       | Schreibgeschützt |
| --------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| `mcp__ide__getDiagnostics`        | Gibt Sprachserver-Diagnosen zurück – die Fehler und Warnungen im Problems-Panel von VS Code. Optional auf eine Datei beschränkt. | Ja               |
| `mcp__ide__executeCode`           | Führt Python-Code im Kernel des aktiven Jupyter-Notebooks aus. Siehe Bestätigungsfluss unten.                                    | Nein             |

**Jupyter-Ausführung fragt immer zuerst.** `mcp__ide__executeCode` kann nichts stillschweigend ausführen. Bei jedem Aufruf wird der Code als neue Zelle am Ende des aktiven Notebooks eingefügt, VS Code scrollt ihn in die Ansicht, und ein natives Quick Pick fragt Sie, ob Sie **Execute** oder **Cancel** wählen. Abbrechen – oder das Picker mit `Esc` schließen – gibt einen Fehler an Claude zurück und nichts wird ausgeführt. Das Tool weigert sich auch kategorisch, wenn es kein aktives Notebook gibt, wenn die Jupyter-Erweiterung (`ms-toolsai.jupyter`) nicht installiert ist, oder wenn der Kernel nicht Python ist.

<Note>
  Die Quick Pick-Bestätigung ist unabhängig von `PreToolUse`-Hooks. Ein Allowlist-Eintrag für `mcp__ide__executeCode` lässt Claude die Ausführung einer Zelle *vorschlagen*; das Quick Pick in VS Code ist das, was es tatsächlich *ausführen* lässt.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Häufige Probleme beheben
</h2>

<h3 id="extension-won’t-install">
  Erweiterung wird nicht installiert
</h3>

* Stellen Sie sicher, dass Sie eine kompatible Version von VS Code haben (1.94.0 oder später)
* Überprüfen Sie, dass VS Code die Berechtigung zum Installieren von Erweiterungen hat
* Versuchen Sie, die Erweiterung direkt vom [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code) zu installieren

<h3 id="spark-icon-not-visible">
  Spark-Symbol nicht sichtbar
</h3>

Das Spark-Symbol wird in der **Editor-Symbolleiste** (oben rechts im Editor) angezeigt, wenn Sie eine Datei geöffnet haben. Wenn Sie es nicht sehen:

1. **Öffnen Sie eine Datei**: Das Symbol erfordert, dass eine Datei geöffnet ist. Es reicht nicht aus, nur einen Ordner geöffnet zu haben.
2. **Überprüfen Sie die VS Code-Version**: Erfordert 1.94.0 oder höher (Hilfe → Über)
3. **Starten Sie VS Code neu**: Führen Sie „Developer: Reload Window" aus der Befehlspalette aus
4. **Deaktivieren Sie in Konflikt stehende Erweiterungen**: Deaktivieren Sie vorübergehend andere KI-Erweiterungen (Cline, Continue, usw.)
5. **Überprüfen Sie die Workspace-Vertrauenswürdigkeit**: Die Erweiterung funktioniert nicht im eingeschränkten Modus

Alternativ können Sie, wenn Sie [`preferredLocation`](#extension-settings) auf `sidebar` gesetzt haben oder Claude mit **Claude Code: Open in Side Bar** geöffnet haben, auf „✻ Claude Code" in der **Statusleiste** (unten rechts) klicken. Dies funktioniert auch ohne geöffnete Datei. Sie können auch die **Befehlspalette** (`Cmd+Shift+P` / `Ctrl+Shift+P`) verwenden und „Claude Code" eingeben.

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc funktioniert auf macOS nicht
</h3>

Auf macOS Tahoe und später ist die System-Game-Overlay-Verknüpfung standardmäßig an `Cmd+Esc` gebunden und fängt den Tastendruck ab, bevor er VS Code erreicht. So geben Sie die Verknüpfung frei:

1. Öffnen Sie die Systemeinstellungen
2. Gehen Sie zu Tastatur, dann Tastaturkürzel, dann Game Controllers
3. Deaktivieren Sie das Kontrollkästchen für Game Overlay

Alternativ können Sie die Erweiterung an eine andere Taste binden: Öffnen Sie den VS Code [Keyboard Shortcuts Editor](https://code.visualstudio.com/docs/configure/keybindings) (`Cmd+K Cmd+S`), suchen Sie nach `Claude Code: Focus input`, und weisen Sie eine neue Bindung zu.

<h3 id="claude-code-never-responds">
  Claude Code antwortet nie
</h3>

Wenn Claude Code nicht auf Ihre Eingaben antwortet:

1. **Überprüfen Sie Ihre Internetverbindung**: Stellen Sie sicher, dass Sie eine stabile Internetverbindung haben
2. **Starten Sie ein neues Gespräch**: Versuchen Sie, ein neues Gespräch zu starten, um zu sehen, ob das Problem weiterhin besteht
3. **Versuchen Sie die CLI**: Führen Sie `claude` vom Terminal aus, um zu sehen, ob Sie detailliertere Fehlermeldungen erhalten

Wenn die Probleme weiterhin bestehen, [melden Sie ein Problem auf GitHub](https://github.com/anthropics/claude-code/issues) mit Details zum Fehler.

<h2 id="uninstall-the-extension">
  Erweiterung deinstallieren
</h2>

So deinstallieren Sie die Claude Code-Erweiterung:

1. Öffnen Sie die Ansicht „Erweiterungen" (`Cmd+Shift+X` auf Mac oder `Ctrl+Shift+X` auf Windows/Linux)
2. Suchen Sie nach „Claude Code"
3. Klicken Sie auf **Deinstallieren**

Wenn Sie `claude` in einem integrierten VS Code-Terminal ausführen, installiert Claude Code die Erweiterung automatisch neu. Um sie deinstalliert zu halten, deaktivieren Sie **Auto-install IDE extension** in `/config`, oder setzen Sie [`autoInstallIdeExtension`](/docs/de/settings-reference#autoinstallideextension) auf `false`. Sie können auch die Umgebungsvariable [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/de/env-vars) auf `1` setzen.

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

Weitere Hilfe finden Sie im [Troubleshooting-Leitfaden](/docs/de/troubleshooting).

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Claude Code in VS Code eingerichtet haben:

* [Erkunden Sie häufige Workflows](/docs/de/common-workflows), um das Beste aus Claude Code herauszuholen
* [Richten Sie MCP servers ein](/docs/de/mcp), um Claudes Funktionen mit externen Tools zu erweitern. Fügen Sie sie mit `/mcp` im Chat-Panel hinzu und verwalten Sie sie.
* [Konfigurieren Sie Claude Code-Einstellungen](/docs/de/settings), um zulässige Befehle, hooks und mehr anzupassen. Diese Einstellungen werden zwischen der Erweiterung und der CLI geteilt.
