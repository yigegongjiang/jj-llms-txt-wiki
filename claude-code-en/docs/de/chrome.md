> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code mit Chrome verwenden

> Verbinden Sie Claude Code mit Ihrem Chrome-Browser, um Web-Apps zu testen, mit Konsolenprotokollen zu debuggen, Formularausfüllungen zu automatisieren und Daten von Webseiten zu extrahieren.

Claude Code integriert sich mit der [Claude in Chrome Browser-Erweiterung](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn), um Ihnen Browser-Automatisierungsfunktionen über die CLI oder die [VS Code-Erweiterung](/docs/de/vs-code#automate-browser-tasks-with-chrome) bereitzustellen. Erstellen Sie Ihren Code und testen und debuggen Sie ihn dann im Browser, ohne den Kontext zu wechseln.

Claude öffnet neue Registerkarten für Browser-Aufgaben und teilt den Anmeldestatus Ihres Browsers, sodass er auf alle Websites zugreifen kann, bei denen Sie bereits angemeldet sind. Browser-Aktionen werden in Echtzeit in einem sichtbaren Chrome-Fenster ausgeführt. Wenn Claude auf eine Anmeldeseite oder ein CAPTCHA trifft, wird es angehalten und fordert Sie auf, es manuell zu bearbeiten.

<Note>
  Die Chrome-Integration funktioniert mit Google Chrome und Microsoft Edge. Sie wird noch nicht auf Brave, Arc oder anderen Chromium-basierten Browsern unterstützt. Sie wird auch nicht unter Windows Subsystem for Linux (WSL) unterstützt.
</Note>

<h2 id="capabilities">
  Funktionen
</h2>

Mit verbundenem Chrome können Sie Browser-Aktionen mit Codierungsaufgaben in einem einzigen Workflow verketten:

* **Live-Debugging**: Lesen Sie Konsolenfehler und DOM-Status direkt aus und beheben Sie dann den Code, der sie verursacht hat
* **Design-Verifizierung**: Erstellen Sie eine Benutzeroberfläche aus einem Figma-Mock und öffnen Sie sie dann im Browser, um zu überprüfen, ob sie übereinstimmt
* **Web-App-Tests**: Testen Sie die Formularvalidierung, überprüfen Sie auf visuelle Regressionen oder überprüfen Sie Benutzerflüsse
* **Authentifizierte Web-Apps**: Interagieren Sie mit Google Docs, Gmail, Notion oder einer beliebigen App, bei der Sie angemeldet sind, ohne API-Konnektoren
* **Datenextraktion**: Extrahieren Sie strukturierte Informationen von Webseiten und speichern Sie sie lokal
* **Task-Automatisierung**: Automatisieren Sie wiederholte Browser-Aufgaben wie Dateneingabe, Formularausfüllung oder Multi-Site-Workflows
* **Sitzungsaufzeichnung**: Zeichnen Sie Browser-Interaktionen als GIFs auf, um zu dokumentieren oder zu teilen, was passiert ist

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Bevor Sie Claude Code mit Chrome verwenden, benötigen Sie:

* [Google Chrome](https://www.google.com/chrome/) oder [Microsoft Edge](https://www.microsoft.com/edge) Browser
* [Claude in Chrome-Erweiterung](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) Version 1.0.36 oder höher, verfügbar im Chrome Web Store für beide Browser
* [Claude Code](/docs/de/quickstart#step-1-install-claude-code)
* Einen direkten Anthropic-Plan (Pro, Max, Team oder Enterprise)

<Note>
  Die Chrome-Integration ist nicht über Drittanbieter wie Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar. Wenn Sie Claude ausschließlich über einen Drittanbieter nutzen, benötigen Sie ein separates claude.ai-Konto, um diese Funktion zu verwenden.
</Note>

<h2 id="get-started-in-the-cli">
  Erste Schritte in der CLI
</h2>

<Steps>
  <Step title="Claude Code mit Chrome starten">
    Starten Sie Claude Code mit dem Flag `--chrome`:

    ```bash theme={null}
    claude --chrome
    ```

    Sie können Chrome auch innerhalb einer bestehenden Sitzung aktivieren, indem Sie `/chrome` ausführen.
  </Step>

  <Step title="Bitten Sie Claude, den Browser zu verwenden">
    Dieses Beispiel navigiert zu einer Seite, interagiert mit ihr und meldet, was es findet, alles von Ihrem Terminal oder Editor aus:

    ```text theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    Die erste Browser-Aktion fordert die Berechtigung zur Verwendung der `claude-in-chrome`-Fähigkeit an. Genehmigen Sie sie und Claude öffnet einen neuen Tab und startet die Aufgabe.
  </Step>
</Steps>

Führen Sie `/chrome` jederzeit aus, um den Verbindungsstatus zu überprüfen, Berechtigungen zu verwalten, die Erweiterung erneut zu verbinden oder auszuwählen, welcher verbundene Browser verwendet werden soll. Wenn mehr als ein Browser verbunden ist, wenn eine Browser-Aktion startet, fordert Claude Sie auf, einen auszuwählen.

Für VS Code siehe [Browser-Automatisierung in VS Code](/docs/de/vs-code#automate-browser-tasks-with-chrome).

<h3 id="enable-chrome-by-default">
  Chrome standardmäßig aktivieren
</h3>

Um zu vermeiden, dass Sie `--chrome` jede Sitzung übergeben müssen, führen Sie `/chrome` aus und wählen Sie „Standardmäßig aktiviert".

In der [VS Code-Erweiterung](/docs/de/vs-code#automate-browser-tasks-with-chrome) ist Chrome verfügbar, wenn die Chrome-Erweiterung installiert ist. Kein zusätzliches Flag ist erforderlich.

<Note>
  Das standardmäßige Aktivieren von Chrome in der CLI erhöht die Kontextnutzung, da Browser-Tools immer geladen werden. Wenn Sie eine erhöhte Kontextnutzung bemerken, deaktivieren Sie diese Einstellung und verwenden Sie `--chrome` nur bei Bedarf.
</Note>

<h3 id="manage-site-permissions">
  Verwalten Sie Website-Berechtigungen
</h3>

Website-Berechtigungen werden von der Chrome-Erweiterung geerbt. Verwalten Sie Berechtigungen in den Einstellungen der Chrome-Erweiterung, um zu steuern, welche Websites Claude durchsuchen, anklicken und eingeben kann.

<h3 id="browser-tools-in-plan-mode">
  Browser-Tools im Plan-Modus
</h3>

Im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) werden Browser-Tool-Aufrufe, die nur die Seite oder den Browser-Status lesen, ohne Genehmigungsaufforderung ausgeführt, und Aufrufe, die den Status ändern, fordern eine Genehmigung an.

* **Schreibgeschützte Aufrufe**: `read_page`, `get_page_text`, `find`, Lesen von Konsolenmeldungen oder Netzwerkanfragen und Erstellen eines Screenshots
* **Statusändernde Aufrufe**: Klicks, Eingaben, Navigation, Tab- und Fensterverwaltung sowie Aufzeichnung einer GIF

Ab v2.1.199 fordert ein ansonsten schreibgeschützter Aufruf, der ein statusänderndes Eingabe-Flag setzt, wie z. B. `createIfEmpty` auf `tabs_context_mcp`, `clear` auf den Konsolen- und Netzwerk-Lesern oder `save_to_disk` auf einem Screenshot, auch eine Genehmigung an. Ein `browser_batch`-Aufruf wird nur dann ohne Aufforderung ausgeführt, wenn jede Aktion darin schreibgeschützt ist.

<h2 id="example-workflows">
  Beispiel-Workflows
</h2>

Diese Beispiele zeigen häufige Möglichkeiten, Browser-Aktionen mit Codierungsaufgaben zu kombinieren. Führen Sie `/mcp` aus, wählen Sie `claude-in-chrome`, und wählen Sie dann **Tools anzeigen**, um die vollständige Liste der verfügbaren Browser-Tools anzuzeigen.

<h3 id="test-a-local-web-application">
  Testen Sie eine lokale Web-Anwendung
</h3>

Wenn Sie eine Web-App entwickeln, bitten Sie Claude, zu überprüfen, ob Ihre Änderungen ordnungsgemäß funktionieren:

```text theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude navigiert zu Ihrem lokalen Server, interagiert mit dem Formular und meldet, was es beobachtet.

<h3 id="debug-with-console-logs">
  Debuggen mit Konsolenprotokollen
</h3>

Claude kann Konsolenausgaben lesen, um Probleme zu diagnostizieren. Teilen Sie Claude mit, welche Muster zu suchen sind, anstatt alle Konsolenausgaben anzufordern, da Protokolle ausführlich sein können:

```text theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude liest die Konsolenmeldungen und kann nach bestimmten Mustern oder Fehlertypen filtern.

<h3 id="automate-form-filling">
  Automatisieren Sie die Formularausfüllung
</h3>

Beschleunigen Sie wiederholte Dateneingabeaufgaben:

```text theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude liest Ihre lokale Datei, navigiert die Web-Schnittstelle und gibt die Daten für jeden Datensatz ein.

<h3 id="draft-content-in-google-docs">
  Entwurf von Inhalten in Google Docs
</h3>

Verwenden Sie Claude, um direkt in Ihren Dokumenten zu schreiben, ohne API-Setup:

```text theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude öffnet das Dokument, klickt in den Editor und gibt den Inhalt ein. Dies funktioniert mit jeder Web-App, bei der Sie angemeldet sind: Gmail, Notion, Sheets und mehr.

<h3 id="extract-data-from-web-pages">
  Extrahieren Sie Daten von Webseiten
</h3>

Extrahieren Sie strukturierte Informationen von Websites:

```text theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude navigiert zur Seite, liest den Inhalt und kompiliert die Daten in ein strukturiertes Format.

<h3 id="run-multi-site-workflows">
  Führen Sie Multi-Site-Workflows aus
</h3>

Koordinieren Sie Aufgaben über mehrere Websites hinweg:

```text theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude arbeitet über Registerkarten hinweg, um Informationen zu sammeln und den Workflow abzuschließen.

<h3 id="record-a-demo-gif">
  Zeichnen Sie eine Demo-GIF auf
</h3>

Erstellen Sie teilbare Aufzeichnungen von Browser-Interaktionen:

```text theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude zeichnet die Interaktionssequenz auf und speichert sie als GIF-Datei.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="extension-not-detected">
  Erweiterung nicht erkannt
</h3>

Wenn Claude Code die Chrome-Erweiterung nicht erkennen kann:

1. Überprüfen Sie, ob die Chrome-Erweiterung in `chrome://extensions` installiert und aktiviert ist
2. Überprüfen Sie, ob Claude Code aktuell ist, indem Sie `claude --version` ausführen
3. Überprüfen Sie, ob Chrome ausgeführt wird
4. Führen Sie `/chrome` aus und wählen Sie „Erweiterung erneut verbinden", um die Verbindung wiederherzustellen
5. Wenn das Problem weiterhin besteht, starten Sie sowohl Claude Code als auch Chrome neu

Wenn Sie die Chrome-Integration zum ersten Mal aktivieren, installiert Claude Code eine Konfigurationsdatei für den nativen Messaging-Host. Chrome liest diese Datei beim Start, daher sollten Sie Chrome neu starten, um die neue Konfiguration zu übernehmen, wenn die Erweiterung beim ersten Versuch nicht erkannt wird.

Ab v2.1.199 öffnet Claude Code beim ersten Installieren einen Browser-Tab, der Sie auffordert, die Erweiterung zu verbinden. Spätere Sitzungen, die die Konfigurationsdatei neu schreiben, z. B. nach dem Wechsel von Claude Code-Builds oder Konfigurationsverzeichnissen, öffnen diese nicht erneut.

Wenn die Verbindung weiterhin fehlschlägt, überprüfen Sie, ob die Host-Konfigurationsdatei vorhanden ist unter:

Für Chrome:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: Überprüfen Sie `HKCU\Software\Google\Chrome\NativeMessagingHosts\` in der Windows-Registrierung

Für Edge:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: Überprüfen Sie `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` in der Windows-Registrierung

<h3 id="browser-not-responding">
  Browser antwortet nicht
</h3>

Wenn Claudes Browser-Befehle nicht mehr funktionieren:

1. Überprüfen Sie, ob ein modales Dialogfeld (Warnung, Bestätigung, Eingabeaufforderung) die Seite blockiert. JavaScript-Dialoge blockieren Browser-Ereignisse und verhindern, dass Claude Befehle empfängt. Schließen Sie das Dialogfeld manuell und teilen Sie Claude mit, dass es fortfahren soll.
2. Bitten Sie Claude, eine neue Registerkarte zu erstellen und es erneut zu versuchen
3. Starten Sie die Chrome-Erweiterung neu, indem Sie sie in `chrome://extensions` deaktivieren und erneut aktivieren

<h3 id="connection-drops-during-long-sessions">
  Verbindung wird während langer Sitzungen unterbrochen
</h3>

Der Service Worker der Chrome-Erweiterung kann während längerer Sitzungen in den Leerlauf gehen, was die Verbindung unterbricht. Wenn Browser-Tools nach einer Inaktivitätsphase nicht mehr funktionieren, führen Sie `/chrome` aus und wählen Sie „Erweiterung erneut verbinden".

<h3 id="windows-specific-issues">
  Windows-spezifische Probleme
</h3>

Unter Windows können folgende Probleme auftreten:

* **Named Pipe-Konflikte (EADDRINUSE)**: Wenn ein anderer Prozess das gleiche Named Pipe verwendet, starten Sie Claude Code neu. Schließen Sie alle anderen Claude Code-Sitzungen, die möglicherweise Chrome verwenden.
* **Fehler beim nativen Messaging-Host**: Wenn der native Messaging-Host beim Start abstürzt, versuchen Sie, Claude Code neu zu installieren, um die Host-Konfiguration zu regenerieren.

<h3 id="common-error-messages">
  Häufige Fehlermeldungen
</h3>

Dies sind die am häufigsten auftretenden Fehler und wie man sie behebt:

| Fehler                                    | Ursache                                                          | Behebung                                                                                                       |
| ----------------------------------------- | ---------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| „Browser-Erweiterung ist nicht verbunden" | Der native Messaging-Host kann die Erweiterung nicht erreichen   | Starten Sie Chrome und Claude Code neu und führen Sie dann `/chrome` aus, um die Verbindung wiederherzustellen |
| „Erweiterung nicht erkannt"               | Chrome-Erweiterung ist nicht installiert oder deaktiviert        | Installieren oder aktivieren Sie die Erweiterung in `chrome://extensions`                                      |
| „Keine Registerkarte verfügbar"           | Claude versuchte zu handeln, bevor eine Registerkarte bereit war | Bitten Sie Claude, eine neue Registerkarte zu erstellen und es erneut zu versuchen                             |
| „Empfänger existiert nicht"               | Der Service Worker der Erweiterung ist in den Leerlauf gegangen  | Führen Sie `/chrome` aus und wählen Sie „Erweiterung erneut verbinden"                                         |

<h2 id="see-also">
  Siehe auch
</h2>

* [Computernutzung](/docs/de/computer-use): Steuern Sie native macOS-Apps, wenn eine Aufgabe nicht in einem Browser ausgeführt werden kann
* [Claude Code in VS Code verwenden](/docs/de/vs-code#automate-browser-tasks-with-chrome): Browser-Automatisierung in der VS Code-Erweiterung
* [CLI-Referenz](/docs/de/cli-reference): Befehlszeilenflags einschließlich `--chrome`
* [Häufige Workflows](/docs/de/common-workflows): Weitere Möglichkeiten zur Verwendung von Claude Code
* [Daten und Datenschutz](/docs/de/data-usage): Wie Claude Code Ihre Daten verarbeitet
* [Erste Schritte mit Claude in Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): Vollständige Dokumentation für die Chrome-Erweiterung, einschließlich Verknüpfungen, Planung und Berechtigungen
