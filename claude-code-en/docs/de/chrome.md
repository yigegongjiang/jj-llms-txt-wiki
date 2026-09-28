> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code mit Chrome verwenden

> Verbinden Sie Claude Code mit Ihrem Chrome-Browser, um Web-Apps zu testen, mit Konsolenprotokollen zu debuggen, Formularausfüllungen zu automatisieren und Daten von Webseiten zu extrahieren.

Claude Code integriert sich mit der [Claude in Chrome Browser-Erweiterung](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn), um Ihnen Browser-Automatisierungsfunktionen über die CLI oder die [VS Code-Erweiterung](/docs/de/vs-code#automate-browser-tasks-with-chrome) bereitzustellen. Erstellen Sie Ihren Code und testen und debuggen Sie ihn dann im Browser, ohne den Kontext zu wechseln.

Claude öffnet neue Registerkarten für Browser-Aufgaben und teilt den Anmeldestatus Ihres Browsers, sodass er auf alle Websites zugreifen kann, bei denen Sie bereits angemeldet sind. Browser-Aktionen werden in Echtzeit in einem sichtbaren Chrome-Fenster ausgeführt. Wenn Claude auf eine Anmeldeseite oder ein CAPTCHA trifft, wird es angehalten und fordert Sie auf, es manuell zu bearbeiten.

Die Erweiterung sammelt die Registerkarten, die Claude öffnet, in einer Chrome-Registerkartengruppe, die an Ihre Sitzung gebunden ist. In lokalen Sitzungen hängt davon ab, ob Claude Code diese Gruppe beim Beenden der Sitzung schließt, wie die Sitzung endet:

* Wenn Sie `/clear` eingeben, schließt Claude Code die Gruppe einschließlich offener Seiten, es sei denn, Arbeit, die das Löschen übersteht, wird noch ausgeführt
* Wenn Sie Sitzungen mit einem Befehl wie `/resume` wechseln, Claude Code beenden oder `/clear` ausführen, während Arbeit, die es übersteht, noch läuft, schließt Claude Code die Gruppe nur, wenn sie nichts als leere neue Registerkarten enthält, sodass Seiten, die Sie möglicherweise noch lesen, offen bleiben

<Note>
  Die Chrome-Integration funktioniert mit Google Chrome und Microsoft Edge. Claude Code erkennt die Erweiterung auch in anderen Chromium-basierten Browsern, einschließlich Brave, Arc, Vivaldi und Opera, und richtet die Verbindung ein. Die Chrome-Integration wird in Windows Subsystem for Linux (WSL) nicht unterstützt.
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
* **Datei-Uploads**: Fügen Sie Dateien von Ihrem Computer an Upload-Felder auf Webseiten an
* **Sitzungsaufzeichnung**: Zeichnen Sie Browser-Interaktionen als GIFs auf, um zu dokumentieren oder zu teilen, was passiert ist

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Bevor Sie Claude Code mit Chrome verwenden, benötigen Sie:

* [Google Chrome](https://www.google.com/chrome/), [Microsoft Edge](https://www.microsoft.com/edge) oder einen anderen Chromium-basierten Browser wie Brave, Arc, Vivaldi oder Opera
* [Claude in Chrome-Erweiterung](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) Version 1.0.36 oder höher, verfügbar im Chrome Web Store
* [Claude Code](/docs/de/quickstart#step-1-install-claude-code)
* Einen direkten Anthropic-Plan (Pro, Max, Team oder Enterprise)

Die Chrome-Integration erfordert auch die Anmeldung mit `/login`. Wenn Sie sich mit einem API-Schlüssel oder einem langlebigen Token von [`claude setup-token`](/docs/de/authentication#generate-a-long-lived-token) authentifizieren, behält Claude Code die Chrome-Integration aus, auch wenn Sie `--chrome` übergeben, da die Browser-Erweiterung sich nicht mit diesen Anmeldedaten authentifizieren kann. Vor v2.1.216 konnten diese Sitzungen die Chrome-Integration aktivieren, aber jeder Versuch, sich mit der Browser-Erweiterung zu verbinden, schlug mit einem 403-Fehler fehl.

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

    Beim ersten Start mit Chrome zeigt Claude Code einen einmaligen Dialog an, der die Integration vorstellt und erklärt, wie Website-Berechtigungen funktionieren. Drücken Sie die Eingabetaste, um fortzufahren.

    Um Chrome für zukünftige Sitzungen ohne das Flag zu aktivieren, siehe [Chrome standardmäßig aktivieren](#enable-chrome-by-default).
  </Step>

  <Step title="Bitten Sie Claude, den Browser zu verwenden">
    Dieses Beispiel navigiert zu einer Seite, interagiert mit ihr und meldet, was es findet, alles von Ihrem Terminal oder Editor aus:

    ```text wrap theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    Wenn Claude Code vor einer Browser-Aktion um Berechtigung fragt, genehmigen Sie diese. Der Dialog beginnt mit `Claude in Chrome wants to` und bietet eine Option, alle Aktionen auf dieser Website für die Sitzung zuzulassen. Claude öffnet einen neuen Tab und startet die Aufgabe.
  </Step>
</Steps>

Führen Sie `/chrome` jederzeit aus, um den Verbindungsstatus zu überprüfen, Berechtigungen zu verwalten, die Erweiterung erneut zu verbinden oder auszuwählen, welcher verbundene Browser verwendet werden soll. Die Integration funktioniert, wenn das Statusfeld „Status: Enabled" und „Extension: Installed" anzeigt.

Wenn mehr als ein Browser verbunden ist, wählen Sie aus, welchen Claude verwendet. Wenn eine Browser-Aktion startet, bevor Sie einen ausgewählt haben, fordert Claude Sie auf, einen auszuwählen. Um später Browser zu wechseln, führen Sie `/chrome` aus und wählen Sie **Select browser…**. Claude verwendet Ihre Wahl weiterhin, auch wenn ein anderer Browser verbunden wird.

Für VS Code siehe [Browser-Automatisierung in VS Code](/docs/de/vs-code#automate-browser-tasks-with-chrome).

<h3 id="install-the-extension-when-claude-asks">
  Installieren Sie die Erweiterung, wenn Claude Sie dazu auffordert
</h3>

Wenn Claude Ihren Browser in einer interaktiven Sitzung benötigt und Claude Code die Erweiterung nicht erkennt, zeigt Claude Code eine Installationsaufforderung mit dem Titel „Claude wants to use your browser" an. Claude Code fragt höchstens einmal pro Sitzung.

Die Aufforderung bietet drei Optionen:

* **Install extension**: öffnet die Seite zur Erweiterungsinstallation in Ihrem Browser und startet ein geführtes Setup. Claude Code wartet auf die Installation, verbindet die Erweiterung und aktiviert Browser-Tools in derselben Sitzung. Wenn die Verbindung hergestellt ist, wählen Sie „Continue with browser tools" und Claude setzt die Aufgabe in Ihrem Browser fort. Sie können das Setup beenden, indem Sie „Continue without browser tools" wählen und es später mit `/chrome` abschließen.
* **Not now**: setzt die Aufgabe ohne Browser-Tools fort. Claude Code kann in einer späteren Sitzung erneut fragen.
* **Don't ask again**: stoppt die Aufforderung in zukünftigen Sitzungen. Sie können die Integration jederzeit mit `/chrome` einrichten.

Wenn Ihre Organisation den `claude-in-chrome` MCP-Server mit der [`deniedMcpServers` verwalteten Einstellung](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists) blockiert, zeigt Claude Code die Installationsaufforderung nicht an.

<h3 id="enable-chrome-by-default">
  Chrome standardmäßig aktivieren
</h3>

Um zu vermeiden, dass Sie `--chrome` jede Sitzung übergeben müssen, führen Sie `/chrome` aus und wählen Sie „Enabled by default".

Claude Code startet normal, wenn Chrome nicht ausgeführt wird. Vor v2.1.211 konnte der Start hängen bleiben, wenn Chrome-Integration aktiviert war, aber Chrome nicht ausgeführt wurde.

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

Im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) wird eine Genehmigungsaufforderung angezeigt, bevor Claude eine GIF aufzeichnet, einen neuen Tab öffnet oder eine Verknüpfung ausführt. Wenn der [Bypass-Berechtigungsmodus](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) in Ihrer Sitzung verfügbar ist und das [Feature-Flag-Abrufen](/docs/de/env-vars#features-that-need-feature-flag-fetching) deaktiviert ist, werden diese Aufrufe ohne Aufforderung ausgeführt.

Ein `tabs_context_mcp`-Aufruf fordert auch eine Genehmigung an, wenn er `createIfEmpty` setzt, und ebenso ein `browser_batch`-Aufruf, der eine dieser Aktionen enthält.

<h2 id="example-workflows">
  Beispiel-Workflows
</h2>

Diese Beispiele zeigen häufige Möglichkeiten, Browser-Aktionen mit Codierungsaufgaben zu kombinieren. Führen Sie `/mcp` aus, wählen Sie `claude-in-chrome`, und wählen Sie dann **Tools anzeigen**, um die vollständige Liste der verfügbaren Browser-Tools anzuzeigen.

<h3 id="test-a-local-web-application">
  Testen Sie eine lokale Web-Anwendung
</h3>

Wenn Sie eine Web-App entwickeln, bitten Sie Claude, zu überprüfen, ob Ihre Änderungen ordnungsgemäß funktionieren:

```text wrap theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude navigiert zu Ihrem lokalen Server, interagiert mit dem Formular und meldet, was es beobachtet.

<h3 id="debug-with-console-logs">
  Debuggen mit Konsolenprotokollen
</h3>

Claude kann Konsolenausgaben lesen, um Probleme zu diagnostizieren. Teilen Sie Claude mit, welche Muster zu suchen sind, anstatt alle Konsolenausgaben anzufordern, da Protokolle ausführlich sein können:

```text wrap theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude liest die Konsolenmeldungen und kann nach bestimmten Mustern oder Fehlertypen filtern.

<h3 id="automate-form-filling">
  Automatisieren Sie die Formularausfüllung
</h3>

Beschleunigen Sie wiederholte Dateneingabeaufgaben:

```text wrap theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude liest Ihre lokale Datei, navigiert die Web-Schnittstelle und gibt die Daten für jeden Datensatz ein.

<h3 id="upload-files-to-web-pages">
  Dateien auf Webseiten hochladen
</h3>

Claude kann Dateien von Ihrem Computer an Upload-Felder auf einer Seite anhängen. Claude Code liest die Datei und sendet ihren Inhalt an den Browser, sodass Uploads sowohl in lokalen als auch in Remote-Sitzungen funktionieren. Erfordert Claude Code v2.1.211 oder später.

Dieses Beispiel hängt eine Protokolldatei an ein Formular an:

```text wrap theme={null}
Open the bug tracker at bugs.example.com, create a new issue,
and attach logs/session.log to it
```

Drei Einschränkungen gelten für Uploads:

* **Berechtigungen**: Claude kann eine Datei nur hochladen, wenn die Sitzung berechtigt ist, sie zu lesen. Daher blockieren [Berechtigungsregeln](/docs/de/settings-reference#permission-settings), die `Read`-Zugriff auf eine Datei verweigern, auch das Hochladen.
* **Größe**: Ein einzelner Upload kann insgesamt bis zu 10 MB Dateien enthalten.
* **Hardlinks**: Claude lehnt Dateien ab, die mehrere Hardlinks haben, was in Package-Manager-Speichern wie `node_modules` häufig vorkommt. Kopieren Sie die Datei und laden Sie die Kopie hoch.

<h3 id="draft-content-in-google-docs">
  Entwurf von Inhalten in Google Docs
</h3>

Verwenden Sie Claude, um direkt in Ihren Dokumenten zu schreiben, ohne API-Setup:

```text wrap theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude öffnet das Dokument, klickt in den Editor und gibt den Inhalt ein. Dies funktioniert mit jeder Web-App, bei der Sie angemeldet sind: Gmail, Notion, Sheets und mehr.

<h3 id="extract-data-from-web-pages">
  Extrahieren Sie Daten von Webseiten
</h3>

Extrahieren Sie strukturierte Informationen von Websites:

```text wrap theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude navigiert zur Seite, liest den Inhalt und kompiliert die Daten in ein strukturiertes Format.

<h3 id="run-multi-site-workflows">
  Führen Sie Multi-Site-Workflows aus
</h3>

Koordinieren Sie Aufgaben über mehrere Websites hinweg:

```text wrap theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude arbeitet über Registerkarten hinweg, um Informationen zu sammeln und den Workflow abzuschließen.

<h3 id="record-a-demo-gif">
  Zeichnen Sie eine Demo-GIF auf
</h3>

Erstellen Sie teilbare Aufzeichnungen von Browser-Interaktionen:

```text wrap theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude zeichnet die Interaktionssequenz auf und speichert sie als GIF-Datei. Die Aufzeichnung erfasst alles, was im Browser sichtbar ist, einschließlich Kontodaten auf angemeldeten Seiten. Überprüfen Sie sie daher, bevor Sie sie außerhalb Ihres Teams freigeben.

<h3 id="save-screenshots-to-disk">
  Speichern Sie Screenshots auf der Festplatte
</h3>

Bitten Sie Claude, einen Screenshot als Datei zu speichern:

```text wrap theme={null}
Take a screenshot of the checkout page and save it to disk
```

Claude speichert das Bild auf der Festplatte und meldet den Dateipfad. Vor v2.1.211 schrieb die `save_to_disk`-Option des Screenshot-Tools keine Datei.

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

Claude Code öffnet beim ersten Installieren einen Browser-Tab, der Sie auffordert, die Erweiterung zu verbinden. Claude Code öffnet ihn nicht erneut, wenn eine spätere Sitzung die Konfigurationsdatei neu schreibt, z. B. nach dem Wechsel von Builds oder Konfigurationsverzeichnissen.

Wenn die Verbindung weiterhin fehlschlägt, überprüfen Sie, ob die Host-Konfigurationsdatei vorhanden ist unter:

Für Chrome:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: Überprüfen Sie `HKCU\Software\Google\Chrome\NativeMessagingHosts\` in der Windows-Registrierung

Für Edge:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: Überprüfen Sie `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` in der Windows-Registrierung

Andere Chromium-basierte Browser lesen die gleiche Datei aus ihrem eigenen Konfigurationsverzeichnis, das nach dem Browser benannt ist. Beispielsweise verwendet Brave auf macOS `~/Library/Application Support/BraveSoftware/Brave-Browser/NativeMessagingHosts/`, und unter Windows hat jeder Browser seinen eigenen Registrierungsschlüssel, z. B. `HKCU\Software\BraveSoftware\Brave-Browser\NativeMessagingHosts\`.

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
* **Setup-Seiten können nicht geöffnet werden**: Aktualisieren Sie Claude Code. Vor v2.1.211 konnte der Browser-Tab, der Sie auffordert, die Erweiterung zu verbinden, unter Windows nicht geöffnet werden.

<h3 id="common-error-messages">
  Häufige Fehlermeldungen
</h3>

Dies sind die am häufigsten auftretenden Fehler und wie man sie behebt:

| Fehler                                         | Ursache                                                                                                                                                            | Behebung                                                                                                                                                                                                                                                                                                               |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| „Browser-Erweiterung ist nicht verbunden"      | Der native Messaging-Host kann die Erweiterung nicht erreichen, oder die IP-Allowlist Ihrer Organisation lehnt die Verbindung zu `bridge.claudeusercontent.com` ab | Starten Sie Chrome und Claude Code neu und führen Sie dann `/chrome` aus, um die Verbindung wiederherzustellen. Wenn Ihre Organisation IP-Allowlisting verwendet und der Fehler weiterhin besteht, siehe [Organization IP allowlists and proxy egress](/docs/de/network-config#organization-ip-allowlists-and-proxy-egress) |
| Erweiterung zeigt „Nicht erkannt" in `/chrome` | Chrome-Erweiterung ist nicht installiert oder deaktiviert                                                                                                          | Installieren oder aktivieren Sie die Erweiterung in `chrome://extensions`                                                                                                                                                                                                                                              |
| „Keine Registerkarte verfügbar"                | Claude versuchte zu handeln, bevor eine Registerkarte bereit war                                                                                                   | Bitten Sie Claude, eine neue Registerkarte zu erstellen und es erneut zu versuchen                                                                                                                                                                                                                                     |
| „Empfänger existiert nicht"                    | Der Service Worker der Erweiterung ist in den Leerlauf gegangen                                                                                                    | Führen Sie `/chrome` aus und wählen Sie „Erweiterung erneut verbinden"                                                                                                                                                                                                                                                 |

<h2 id="see-also">
  Siehe auch
</h2>

* [Computernutzung](/docs/de/computer-use): Steuern Sie native macOS-Apps, wenn eine Aufgabe nicht in einem Browser ausgeführt werden kann
* [Claude Code in VS Code verwenden](/docs/de/vs-code#automate-browser-tasks-with-chrome): Browser-Automatisierung in der VS Code-Erweiterung
* [CLI-Referenz](/docs/de/cli-reference): Befehlszeilenflags einschließlich `--chrome`
* [Häufige Workflows](/docs/de/common-workflows): Weitere Möglichkeiten zur Verwendung von Claude Code
* [Daten und Datenschutz](/docs/de/data-usage): Wie Claude Code Ihre Daten verarbeitet
* [Erste Schritte mit Claude in Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): Vollständige Dokumentation für die Chrome-Erweiterung, einschließlich Verknüpfungen, Planung und Berechtigungen
