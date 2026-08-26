> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude von der CLI aus Ihren Computer nutzen lassen

> Aktivieren Sie die Computernutzung in der Claude Code CLI, damit Claude Apps öffnen, klicken, tippen und Ihren Bildschirm auf macOS sehen kann. Testen Sie native Apps, debuggen Sie visuelle Probleme und automatisieren Sie GUI-only-Tools, ohne Ihr Terminal zu verlassen.

<Note>
  Die Computernutzung ist eine Forschungsvorschau auf macOS, die einen Pro- oder Max-Plan erfordert. Sie ist nicht auf Team- oder Enterprise-Plänen verfügbar. Sie erfordert eine interaktive Sitzung, daher ist sie nicht im nicht-interaktiven Modus mit dem Flag `-p` verfügbar.
</Note>

Die Computernutzung ermöglicht es Claude, Apps zu öffnen, Ihren Bildschirm zu steuern und auf Ihrem Computer so zu arbeiten, wie Sie es tun würden. Von der CLI aus kann Claude eine Swift-App kompilieren, sie starten, jeden Button durchklicken und das Ergebnis screenshooten – alles in derselben Konversation, in der es den Code geschrieben hat.

Diese Seite behandelt, wie die Computernutzung in der CLI funktioniert. Für die Desktop-App auf macOS oder Windows siehe [Computernutzung in Desktop](/docs/de/desktop#let-claude-use-your-computer).

<h2 id="what-you-can-do-with-computer-use">
  Was Sie mit der Computernutzung tun können
</h2>

Die Computernutzung bewältigt Aufgaben, die eine GUI erfordern: alles, was Sie normalerweise das Terminal verlassen und von Hand tun müssten.

* **Native Apps erstellen und validieren**: Bitten Sie Claude, eine macOS-Menüleisten-App zu erstellen. Claude schreibt Swift, kompiliert es, startet die App und klickt jeden Control durch, um zu überprüfen, dass es funktioniert, bevor Sie es jemals öffnen.
* **End-to-End-UI-Tests**: Zeigen Sie Claude eine lokale Electron-App und sagen Sie „teste den Onboarding-Flow." Claude öffnet die App, klickt sich durch die Anmeldung und macht einen Screenshot von jedem Schritt. Keine Playwright-Konfiguration, kein Test-Harness.
* **Visuelle und Layout-Probleme debuggen**: Sagen Sie Claude „das Modal wird bei kleinen Fenstern abgeschnitten." Claude ändert die Fenstergröße, reproduziert den Bug, macht einen Screenshot, patcht das CSS und überprüft die Korrektur. Claude sieht, was Sie sehen.
* **GUI-only-Tools steuern**: Interagieren Sie mit Design-Tools, Hardware-Kontrollpanelen, dem iOS Simulator oder proprietären Apps, die keine CLI oder API haben.

<h2 id="when-computer-use-applies">
  Wann die Computernutzung angewendet wird
</h2>

Claude hat mehrere Möglichkeiten, mit einer App oder einem Service zu interagieren. Die Computernutzung ist die breiteste und langsamste, daher versucht Claude zuerst das präziseste Tool:

* Wenn Sie einen [MCP-Server](/docs/de/mcp) für den Service haben, verwendet Claude diesen.
* Wenn die Aufgabe ein Shell-Befehl ist, verwendet Claude Bash.
* Wenn die Aufgabe Browser-Arbeit ist und Sie [Claude in Chrome](/docs/de/chrome) eingerichtet haben, verwendet Claude das.
* Wenn keine dieser Optionen zutrifft, verwendet Claude die Computernutzung.

Die Bildschirmsteuerung ist für Dinge reserviert, die nichts anderes erreichen kann: native Apps, Simulatoren und Tools ohne API.

<h2 id="enable-computer-use">
  Computernutzung aktivieren
</h2>

Die Computernutzung ist als integrierter MCP-Server namens `computer-use` verfügbar. Sie ist standardmäßig deaktiviert, bis Sie sie aktivieren.

<Steps>
  <Step title="Öffnen Sie das MCP-Menü">
    Führen Sie in einer interaktiven Claude Code-Sitzung Folgendes aus:

    ```text theme={null}
    /mcp
    ```

    Finden Sie `computer-use` in der Serverliste. Es wird als deaktiviert angezeigt.
  </Step>

  <Step title="Aktivieren Sie den Server">
    Wählen Sie `computer-use` und wählen Sie **Aktivieren**. Die Einstellung bleibt pro Projekt bestehen, daher müssen Sie dies nur einmal für jedes Projekt tun, in dem Sie die Computernutzung möchten.
  </Step>

  <Step title="Gewähren Sie macOS-Berechtigungen">
    Wenn Claude Ihren Computer zum ersten Mal nutzen möchte, sehen Sie eine Aufforderung, zwei macOS-Berechtigungen zu gewähren:

    * **Barrierefreiheit**: ermöglicht Claude zu klicken, zu tippen und zu scrollen
    * **Bildschirmaufzeichnung**: ermöglicht Claude zu sehen, was auf Ihrem Bildschirm angezeigt wird

    Die Aufforderung enthält Links zum Öffnen des relevanten System-Einstellungsbereichs. Gewähren Sie beide, wählen Sie dann **Erneut versuchen** in der Aufforderung. macOS erfordert möglicherweise, dass Sie Claude Code nach dem Gewähren der Bildschirmaufzeichnung neu starten.
  </Step>
</Steps>

Nach der Einrichtung bitten Sie Claude, etwas zu tun, das die GUI benötigt:

```text theme={null}
Erstellen Sie das App-Ziel, starten Sie es und klicken Sie durch jeden Tab, um
sicherzustellen, dass nichts abstürzt. Machen Sie einen Screenshot von Fehlerzuständen, die Sie finden.
```

<h2 id="approve-apps-per-session">
  Genehmigen Sie Apps pro Sitzung
</h2>

Das Aktivieren des `computer-use`-Servers gewährt Claude nicht automatisch Zugriff auf jede App auf Ihrem Computer. Wenn Claude eine bestimmte App zum ersten Mal in einer Sitzung benötigt, erscheint eine Aufforderung in Ihrem Terminal mit:

* Welche Apps Claude steuern möchte
* Alle zusätzlichen angeforderten Berechtigungen, wie z. B. Zwischenablage-Zugriff
* Wie viele andere Apps ausgeblendet werden, während Claude arbeitet

Wählen Sie **Für diese Sitzung zulassen** oder **Ablehnen**. Genehmigungen gelten für die aktuelle Sitzung. Sie können mehrere Apps auf einmal genehmigen, wenn Claude sie zusammen anfordert.

Apps mit großer Reichweite zeigen eine zusätzliche Warnung in der Aufforderung, damit Sie wissen, welche Zugriffe das Genehmigen gewährt:

| Warnung                              | Gilt für                                                     |
| :----------------------------------- | :----------------------------------------------------------- |
| Gleichbedeutend mit Shell-Zugriff    | Terminal, iTerm, VS Code, Warp und andere Terminals und IDEs |
| Kann jede Datei lesen oder schreiben | Finder                                                       |
| Kann Systemeinstellungen ändern      | Systemeinstellungen                                          |

Diese Apps sind nicht blockiert. Die Warnung hilft Ihnen zu entscheiden, ob die Aufgabe dieses Zugriffsniveau rechtfertigt.

Claudes Kontrollebene variiert auch je nach App-Kategorie: Browser und Handelsplattformen sind nur zum Anschauen, Terminals und IDEs sind nur zum Klicken und alles andere erhält vollständige Kontrolle. Siehe [App-Berechtigungen in Desktop](/docs/de/desktop#app-permissions) für die vollständige Tier-Aufschlüsselung.

<h2 id="how-claude-works-on-your-screen">
  Wie Claude auf Ihrem Bildschirm arbeitet
</h2>

Das Verständnis des Ablaufs hilft Ihnen zu antizipieren, was Claude tun wird und wie Sie eingreifen können.

<h3 id="one-session-at-a-time">
  Eine Sitzung auf einmal
</h3>

Die Computernutzung hält eine maschinenweite Sperre vom ersten Computernutzungs-Aktion bis die Sitzung, die sie erworben hat, beendet wird. Ab v2.1.195 gibt das Beenden der Aufgabe die Sperre nicht frei; nur das Beenden der Sitzung tut dies. Wenn eine andere Claude Code-Sitzung bereits Ihren Computer nutzt, schlagen neue Versuche mit einer Nachricht fehl, die Ihnen mitteilt, welche Sitzung die Sperre hält. Beenden Sie diese Sitzung zuerst.

<h3 id="apps-are-hidden-while-claude-works">
  Apps werden ausgeblendet, während Claude arbeitet
</h3>

Wenn Claude beginnt, Ihren Bildschirm zu steuern, werden andere sichtbare Apps ausgeblendet, damit Claude nur mit den genehmigten Apps interagiert. Ihr Terminal-Fenster bleibt sichtbar und ist von Screenshots ausgeschlossen, daher können Sie die Sitzung beobachten und Claude sieht seine eigene Ausgabe nie.

Wenn Claude die Runde beendet, werden ausgeblendete Apps automatisch wiederhergestellt.

<h3 id="screenshots-are-downscaled-automatically">
  Screenshots werden automatisch herunterskaliert
</h3>

Claude Code skaliert jeden Screenshot herunter, bevor er ihn an das Modell sendet. Sie müssen Ihre Anzeigeauflösung nicht senken oder Fenster auf Retina- oder anderen hochauflösenden Displays ändern. Ein 16-Zoll-MacBook Pro mit nativer Retina-Auflösung erfasst bei 3456×2234 und skaliert auf ungefähr 1372×887 herunter, wobei das Seitenverhältnis beibehalten wird.

Es gibt keine Einstellung zum Ändern der Zielgröße. Wenn Text oder Steuerelemente auf dem Bildschirm nach dem Herunterskalieren zu klein für Claude zum Lesen sind, vergrößern Sie diese in der App, anstatt Ihre Anzeigeauflösung zu ändern.

<h3 id="stop-at-any-time">
  Jederzeit stoppen
</h3>

Wenn Claude die Sperre erhält, erscheint eine macOS-Benachrichtigung: 'Claude nutzt Ihren Computer · drücken Sie Esc zum Stoppen." Drücken Sie `Esc` überall, um die aktuelle Aktion sofort abzubrechen, oder drücken Sie `Ctrl+C` im Terminal. In beiden Fällen stoppt Claude, blendet Ihre Apps wieder ein und gibt Ihnen die Kontrolle zurück. Die Sitzung behält die [Computernutzungssperre](#one-session-at-a-time) bis sie beendet wird.

Eine zweite Benachrichtigung erscheint, wenn Claude fertig ist.

<h2 id="safety-and-the-trust-boundary">
  Sicherheit und die Vertrauensgrenze
</h2>

<Warning>
  Im Gegensatz zum [isolierten Bash-Tool](/docs/de/sandboxing) läuft die Computernutzung auf Ihrem tatsächlichen Desktop mit Zugriff auf die Apps, die Sie genehmigen. Claude überprüft jede Aktion und kennzeichnet potenzielle Prompt-Injection aus Inhalten auf dem Bildschirm, aber die Vertrauensgrenze ist unterschiedlich. Siehe den [Sicherheitsleitfaden für Computernutzung](https://support.claude.com/en/articles/14128542) für Best Practices.
</Warning>

Die integrierten Schutzvorrichtungen reduzieren das Risiko ohne Konfiguration:

* **Pro-App-Genehmigung**: Claude kann nur Apps steuern, die Sie in der aktuellen Sitzung genehmigt haben.
* **Sentinel-Warnungen**: Apps, die Shell-, Dateisystem- oder Systemeinstellungszugriff gewähren, werden gekennzeichnet, bevor Sie sie genehmigen.
* **Terminal ausgeschlossen von Screenshots**: Claude sieht Ihr Terminal-Fenster nie, daher können On-Screen-Aufforderungen in Ihrer Sitzung nicht in das Modell zurückfließen.
* **Globales Escape**: Die `Esc`-Taste bricht die Computernutzung von überall ab, und der Tastendruck wird verbraucht, daher kann Prompt-Injection ihn nicht zum Schließen von Dialogen verwenden.
* **Sperrdatei**: Nur eine Sitzung kann Ihren Computer auf einmal steuern.

<h2 id="example-workflows">
  Beispiel-Workflows
</h2>

Diese Beispiele zeigen häufige Möglichkeiten, die Computernutzung mit Coding-Aufgaben zu kombinieren.

<h3 id="validate-a-native-build">
  Einen nativen Build validieren
</h3>

Nach Änderungen an einer macOS- oder iOS-App lassen Sie Claude in einem Durchgang kompilieren und überprüfen:

```text theme={null}
Erstellen Sie das MenuBarStats-Ziel, starten Sie es, öffnen Sie das Einstellungsfenster,
und überprüfen Sie, dass der Interval-Schieberegler das Label aktualisiert. Machen Sie einen Screenshot des
Einstellungsfensters, wenn Sie fertig sind.
```

Claude führt `xcodebuild` aus, startet die App, interagiert mit der UI und berichtet, was es findet.

<h3 id="reproduce-a-layout-bug">
  Einen Layout-Bug reproduzieren
</h3>

Wenn ein visueller Bug nur bei bestimmten Fenstergrößen auftritt, lassen Sie Claude ihn finden:

```text theme={null}
Das Einstellungsmodal schneidet seine Fußzeile bei schmalen Fenstern ab. Ändern Sie die Größe des App-Fensters,
bis Sie es reproduzieren können, machen Sie einen Screenshot des abgeschnittenen Zustands,
überprüfen Sie dann das CSS für den Modal-Container.
```

Claude ändert die Fenstergröße, erfasst den fehlerhaften Zustand und liest die relevanten Stylesheets.

<h3 id="test-a-simulator-flow">
  Einen Simulator-Flow testen
</h3>

Steuern Sie den iOS Simulator, ohne XCTest zu schreiben:

```text theme={null}
Öffnen Sie den iOS Simulator, starten Sie die App, tippen Sie sich durch die Onboarding-Bildschirme,
und sagen Sie mir, ob ein Bildschirm länger als eine Sekunde zum Laden braucht.
```

Claude steuert den Simulator auf die gleiche Weise wie Sie mit einer Maus.

<h2 id="differences-from-the-desktop-app">
  Unterschiede zur Desktop-App
</h2>

Die CLI- und Desktop-Oberflächen teilen sich die gleiche Computernutzungs-Engine, mit ein paar Unterschieden:

| Funktion               | Desktop                                                             | CLI                                     |
| :--------------------- | :------------------------------------------------------------------ | :-------------------------------------- |
| Plattformen            | macOS und Windows                                                   | Nur macOS                               |
| Aktivieren             | Umschalter in **Einstellungen > Allgemein** (unter **Desktop-App**) | Aktivieren Sie `computer-use` in `/mcp` |
| Liste abgelehnter Apps | Konfigurierbar in Einstellungen                                     | Noch nicht verfügbar                    |
| Auto-Unhide-Umschalter | Optional                                                            | Immer aktiviert                         |
| Dispatch-Integration   | Von Dispatch gestartete Sitzungen können Computernutzung verwenden  | Nicht anwendbar                         |

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="computer-use-is-in-use-by-another-claude-session">
  „Computernutzung wird von einer anderen Claude-Sitzung verwendet"
</h3>

Eine andere Claude Code-Sitzung hält die Sperre, die sie behält, bis sie beendet wird. Beenden Sie diese Sitzung. Wenn die andere Sitzung abgestürzt ist, wird die Sperre automatisch freigegeben, wenn Claude erkennt, dass der Prozess nicht mehr ausgeführt wird.

<h3 id="macos-permissions-prompt-keeps-reappearing">
  macOS-Berechtigungsaufforderung erscheint immer wieder
</h3>

macOS erfordert manchmal einen Neustart des anfordernden Prozesses, nachdem Sie die Bildschirmaufzeichnung gewähren. Beenden Sie Claude Code vollständig und starten Sie eine neue Sitzung. Wenn die Aufforderung weiterhin angezeigt wird, öffnen Sie **Systemeinstellungen > Datenschutz & Sicherheit > Bildschirmaufzeichnung** und bestätigen Sie, dass Ihre Terminal-App aufgelistet und aktiviert ist.

<h3 id="computer-use-doesn’t-appear-in-/mcp">
  `computer-use` erscheint nicht in `/mcp`
</h3>

Der Server erscheint nur auf berechtigten Setups. Überprüfen Sie, dass:

* Sie auf macOS sind. Die Computernutzung in der CLI ist nicht auf Linux oder Windows verfügbar. Unter Windows verwenden Sie stattdessen [Computernutzung in Desktop](/docs/de/desktop#let-claude-use-your-computer).
* Sie einen Pro- oder Max-Plan haben. Führen Sie `/status` aus, um Ihr Abonnement zu bestätigen.
* Sie sich über claude.ai authentifiziert haben. Die Computernutzung ist nicht mit Drittanbieter-Providern wie Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar. Wenn Sie Claude ausschließlich über einen Drittanbieter-Provider nutzen, benötigen Sie ein separates claude.ai-Konto, um diese Funktion zu nutzen.
* Sie sind in einer interaktiven Sitzung. Die Computernutzung ist nicht im nicht-interaktiven Modus mit dem Flag `-p` verfügbar.

<h2 id="see-also">
  Siehe auch
</h2>

* [Computernutzung in Desktop](/docs/de/desktop#let-claude-use-your-computer): die gleiche Funktion mit einer grafischen Einstellungsseite
* [Claude in Chrome](/docs/de/chrome): Browser-Automatisierung für webbasierte Aufgaben
* [MCP](/docs/de/mcp): Verbinden Sie Claude mit strukturierten Tools und APIs
* [Sandboxing](/docs/de/sandboxing): wie Claudes Bash-Tool Dateisystem- und Netzwerkzugriff isoliert
* [Sicherheitsleitfaden für Computernutzung](https://support.claude.com/en/articles/14128542): Best Practices für sichere Computernutzung
