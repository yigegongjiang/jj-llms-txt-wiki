> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# iOS-Apps im Simulator testen

> Claude Code Desktop öffnet Ihre App im iOS-Simulator-Bereich, wenn Claude sie erstellt, ausführt oder überprüft. Jede Sitzung hat einen separaten Simulator.

<Note>
  Der iOS-Simulator-Bereich befindet sich in der öffentlichen Beta in Claude Code Desktop auf macOS. Er ist in den Pro-, Max-, Team- und Enterprise-Plänen verfügbar, außer in Enterprise-Organisationen mit aktivierter HIPAA-Konfiguration.
</Note>

Der iOS-Simulator-Bereich zeigt Ihre App, die in Apples iOS-Simulator läuft, neben Ihrer Unterhaltung in Claude Code Desktop. Wenn Claude Ihre App in einem Simulator erstellt, installiert, startet oder überprüft, öffnet sich der Bereich automatisch und streamt den Gerätebildschirm live. Nutzen Sie ihn, um Claude beim Ausführen und Testen Ihrer App zu beobachten, oder tippen Sie selbst durch die App, während Claude weiterarbeitet.

Der Simulator-Bereich steuert den Simulator direkt, daher benötigt er keine [Computernutzung](/docs/de/desktop#let-claude-use-your-computer) und übernimmt niemals Ihren Bildschirm oder verbirgt Ihre anderen Fenster. Über die CLI erreicht Claude den iOS-Simulator durch [Computernutzung](/docs/de/computer-use#test-a-simulator-flow), die den Simulator auf Ihrem Bildschirm auf die gleiche Weise steuert wie Sie mit einer Maus.

<h2 id="requirements">
  Anforderungen
</h2>

Der Simulator-Bereich nutzt Apples Simulator-Tools, die die Desktop-App nicht enthält. Stellen Sie vor dem Starten einer Sitzung sicher, dass Sie folgende Voraussetzungen erfüllen:

* Claude Desktop v1.24012.0 oder später
* Einen Mac, da Apples iOS-Simulator nur auf macOS läuft
* [Xcode](https://developer.apple.com/xcode/) mit installierter iOS-Plattform, die die Simulator-Geräte bereitstellt. Wenn Xcode noch keine Simulatoren auflistet, siehe [Der Simulator-Bereich sagt, dass keine Simulatoren gefunden wurden](#the-simulator-pane-says-no-simulators-were-found)
  * Verwenden Sie Xcode 26.x. Der Bereich funktioniert noch nicht mit Xcode 27, das die Simulator-App durch Device Hub ersetzt. Wenn `xcode-select` auf Ihrem Mac auf Xcode 27 verweist, siehe [Der Simulator-Bereich schlägt mit Xcode 27 fehl](#the-simulator-pane-fails-with-xcode-27)

<Note>
  Auf dieser Seite bezieht sich „Gerät" auf ein simuliertes iPhone oder iPad, eines der gleichen Simulator-Geräte, die Sie in Xcode unter **Window → Devices and Simulators** verwalten, nicht auf physische Hardware.
</Note>

Der Simulator-Bereich ist nur in lokalen Sitzungen verfügbar. In [Cloud-](/docs/de/desktop#run-long-running-tasks-in-the-cloud) und [SSH-](/docs/de/desktop#ssh-sessions) Sitzungen läuft Claude auf einem Computer, der die Simulatoren auf Ihrem Mac nicht erreichen kann.

<h2 id="run-your-app-in-the-simulator">
  Führen Sie Ihre App im Simulator aus
</h2>

Sie benötigen keinen Befehl oder keine Einstellung, um den Simulator-Bereich zu öffnen. Claude öffnet ihn, wenn er Ihre App in einem Simulator ausführt.

<Steps>
  <Step title="Öffnen Sie Ihr iOS-Projekt">
    Öffnen Sie in Claude Code Desktop die Registerkarte **Code** und starten Sie eine Sitzung mit dem Projektordner Ihrer App als [Projektordner](/docs/de/desktop#start-a-session). Jedes Projekt, das eine App für den iOS-Simulator erstellt, funktioniert.
  </Step>

  <Step title="Bitten Sie Claude, die App auszuführen oder zu testen">
    Formulieren Sie die Aufgabe rund um das Ausführen oder Überprüfen der App. Zum Beispiel:

    ```text theme={null}
    Build the app and run it in the simulator to check the onboarding flow.
    ```
  </Step>

  <Step title="Beobachten Sie die App im Simulator-Bereich">
    Wenn die App in einem Simulator startet, öffnet sich der iOS-Simulator-Bereich neben der Unterhaltung. Wenn Claude ein Gerät zum ersten Mal nutzt, fragt die Desktop-App Sie um Erlaubnis; siehe [Gewähren Sie Claude Zugriff auf ein Gerät](#grant-claude-access-to-a-device). Claude installiert die App, tippt sie durch und liest den Bildschirm, um seine eigenen Änderungen zu überprüfen, während Sie zuschauen.
  </Step>
</Steps>

Der Simulator-Bereich öffnet sich, wenn Claude die App in einem Simulator startet, an jedem Punkt in der Sitzung. Wenn Ihre Anfrage darum geht, die App zu sehen, zum Beispiel „sieht der neue Bildschirm richtig aus?", startet Claude einen Simulator, bevor er mit der Arbeit beginnt. Nachdem Claude einen Fehler behoben oder einen Bildschirm geändert hat, bitten Sie ihn, die Änderung zu überprüfen: Das Neustarten der App öffnet den Bereich erneut, wenn er nicht offen ist.

Der Simulator-Bereich zeigt das Gerät, auf dem die App tatsächlich gestartet wurde. Um auf einem bestimmten Gerät zu testen, nennen Sie es in Ihrer Anfrage, zum Beispiel „führe es auf dem iPhone SE Simulator aus", und Claude zielt auf dieses Gerät ab, wenn es erstellt und startet.

Ein Gerät, das Claude startet, erscheint auch in Apples Simulator-App, und Claude kann die App auf einem Gerät installieren, das Sie bereits gestartet haben.

Sie können den Simulator-Bereich auch selbst öffnen. Sobald die Sitzung einen Simulator angehängt hat oder Swift-Dateien bearbeitet hat, zeigt das Menü **Views** in der Sitzungs-Symbolleiste einen Eintrag **iOS Simulator**. Wenn der Bereich noch kein Gerät anzeigt, klicken Sie auf **Attach simulator** oder wählen Sie ein bestimmtes Gerät aus dem Geräte-Menü daneben; wenn Sie ein ausgeschaltetes Gerät auswählen, wird es gestartet. Wenn Xcode oder seine Simulatoren fehlen, zeigt der Bereich stattdessen die Einrichtungsschritte an und markiert sie, wenn Sie sie abschließen.

<h2 id="control-the-simulator-yourself">
  Steuern Sie den Simulator selbst
</h2>

Der Simulator-Bereich ist interaktiv, nicht nur ein Viewer. Während Claude arbeitet oder zwischen Aufgaben können Sie:

* Tippen und wischen, indem Sie auf dem Gerätebildschirm klicken und ziehen
* Hardware-Tasten mit den gleichen Tastenkombinationen wie in Apples Simulator-App drücken: **Cmd+Shift+H** für Home, **Cmd+L** zum Sperren, **Cmd+Up Arrow** und **Cmd+Down Arrow** für Lautstärke
* Das Gerät um eine Vierteldrehung im Uhrzeigersinn mit der Schaltfläche drehen oder **Cmd+Right Arrow**
* Wechseln Sie das Gerät, das der Bereich anzeigt, über das Geräte-Menü, das die Betriebssystemversion jedes Simulators und seinen Status anzeigt
* Speichern Sie einen Screenshot mit **Cmd+S** oder eine Bildschirmaufzeichnung mit **Cmd+R**, indem Sie die Erfassungsschaltflächen des Bereichs oder die Tastenkombinationen verwenden; die Dateien werden auf Ihrem Desktop gespeichert
* Beenden Sie das Streaming eines Geräts, ohne es auszuschalten, indem Sie auf **Detach simulator** klicken, das den Bereich in seinen Zustand **Attach simulator** zurückversetzt

Die Zeile unter dem Gerätenamen optimiert den Videostrom vom Simulator. Senken Sie **Frame rate** oder **Resolution**, wenn der Bereich Ihren Mac belastet, wechseln Sie **Encoding** zwischen H.264 und JPEG, oder aktivieren Sie **FPS**, um die Bildrate anzuzeigen, die der Bereich empfängt. Diese Einstellungen ändern, wie der Bereich das Gerät anzeigt, nicht wie die App läuft.

Sie und Claude steuern das gleiche Gerät, daher ändern Ihre Taps den App-Status, den Claude sieht. Um Claude einen bestimmten Bildschirm überprüfen zu lassen, navigieren Sie dorthin, indem Sie tippen, und fragen Sie dann. Während Claude das Gerät steuert, zeigt der Bereich ein Badge **Claude is using this device** über dem Bildschirm an; halten Sie mit dem Tippen an, bis das Badge verschwindet, damit das Ergebnis die App und nicht Ihre Eingabe widerspiegelt.

<h2 id="how-sessions-manage-devices">
  Wie Sitzungen Geräte verwalten
</h2>

Jedes Gerät gehört der Sitzung, die es gestartet hat, daher teilen sich [parallele Sitzungen](/docs/de/desktop#work-in-parallel-with-sessions) kein Gerät: Was Sie im Bereich einer Sitzung sehen, spiegelt die Arbeit dieser Sitzung wider, nicht die einer anderen. Das Wechseln von Sitzungen in der Seitenleiste wechselt die Simulator-Ansicht zusammen mit der Unterhaltung, und das Zurückwechseln setzt das gleiche Gerät fort, wo es aufgehört hat. Wenn Claude mit mehr als einem Gerät arbeitet, öffnet jedes seinen eigenen Bereich, bis zu 4 pro Sitzung.

Claude Code Desktop fährt die Simulatoren herunter, die es gestartet hat, sobald sie nicht mehr verwendet werden: wenn Sie die App beenden, wenn Sie die Sitzung archivieren, oder 10 Minuten, nachdem Sie ein Gerät von seinem Bereich trennen. Geräte, die Sie selbst starten, ob vom Bereich oder in Apples Simulator-App, werden niemals automatisch heruntergefahren. Um das angehängte Gerät sofort herunterzufahren, verwenden Sie die Schaltfläche zum Herunterfahren im Bereich.

<h2 id="grant-claude-access-to-a-device">
  Claude Zugriff auf ein Gerät gewähren
</h2>

Claude fragt nach Ihrer Zustimmung, bevor es ein Gerät steuert, während das Erstellen der App oder das Öffnen einer URL darauf Ihrem Sitzungs-Berechtigungsmodus folgt. Sie oder Ihre Organisation können Claudes Zugriff auch vollständig deaktivieren.

<h3 id="allow-a-device-the-first-time">
  Ein Gerät beim ersten Mal zulassen
</h3>

Wenn Claude einen Simulator zum ersten Mal verwendet, fragt Sie die Desktop-App, ob Sie ihn zulassen möchten. Die Zustimmung umfasst die Steuerung dieses Geräts und das Erstellen von Screenshots davon, und Sie erteilen sie einmal pro Gerät statt einmal pro Sitzung. Claudes Screenshots des Geräts werden an Anthropic gesendet und unterliegen Ihren normalen Einstellungen zur Aufbewahrung von Gesprächen, daher melden Sie sich nicht bei echten Konten auf einem Gerät an, das Claude verwendet.

Nachdem Sie ein Gerät zulassen, laufen Claudes Aktionen darauf, wie Tippen, Eingeben, Starten der App und Erstellen von Screenshots, ohne weitere Aufforderungen ab. Sie haben das gleiche Vertrauen wie Ihr Klicken im Bereich, und sie berühren nur das simulierte Gerät, daher benötigt der Bereich nicht die macOS-Barrierefreiheits- und Bildschirmaufzeichnungsberechtigungen, die die Computernutzung erfordert.

Wenn Sie ablehnen, startet das Gerät immer noch und der Bereich funktioniert immer noch für Ihre eigenen Taps; nur Claudes Zugriff bleibt deaktiviert. Um Ihre Meinung später zu ändern, klicken Sie auf **Let Claude use it** im Bereich.

<h3 id="actions-that-follow-your-permission-mode">
  Aktionen, die Ihrem Berechtigungsmodus folgen
</h3>

Zwei Aktionen folgen Ihrem Sitzungs-[Berechtigungsmodus](/docs/de/permissions#permission-modes) statt der einmaligen Zustimmung:

* Das Öffnen einer URL auf dem Gerät, beispielsweise zum Testen eines Deep Links oder zum Laden einer Seite in Safaris des Geräts, da eine URL Daten vom Gerät tragen kann.
* Das Erstellen der App, da `xcodebuild` die Build-Skripte Ihres Projekts auf Ihrem Mac ausführt. Das Überprüfen eines bereits laufenden Builds wird nicht aufgefordert.

<h3 id="turn-off-simulator-access">
  Simulator-Zugriff deaktivieren
</h3>

Sie können Claudes Simulator-Zugriff in den Einstellungen der Desktop-App deaktivieren. Organisationen haben zwei Möglichkeiten, ihn für alle zu deaktivieren:

* Die `disableMobileSimulatorTools` [verwaltete Einstellung](/docs/de/desktop#managed-settings) blockiert Claudes Simulator-Tools. Der Simulator-Bereich bleibt für Ihre eigenen Taps nutzbar, und die Einstellung kann nicht von innerhalb der App überschrieben werden.
* Der `requireCoworkFullVmSandbox` Policy-Schlüssel, der Claudes Tools in einer isolierten virtuellen Maschine statt auf Ihrem Mac ausführt, deaktiviert den Simulator-Bereich und Claudes Simulator-Tools vollständig, sodass der Bereich kein Gerät anschließen kann, während er gesetzt ist.

Claude teilt Ihnen mit, wenn einer dieser Fälle zutrifft.

<h2 id="limitations">
  Einschränkungen
</h2>

Claude steuert nur simulierte Geräte und kann kein physisches iPhone oder iPad steuern. Um auf einem zu testen, führen Sie die App selbst von Xcode aus, beschreiben Sie dann, was Sie sehen, oder hängen Sie einen Screenshot an die Unterhaltung an, damit Claude daraus arbeiten kann.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  Der Simulator-Bereich öffnet sich nicht, wenn Claude die App ausführt
</h3>

Claude hat möglicherweise nicht erkannt, dass Sie die App ausführen oder testen möchten, oder die Simulator-Tools fehlen möglicherweise. Überprüfen Sie folgende Punkte:

* Geben Sie das Ziel explizit an, zum Beispiel „führe die App im iOS-Simulator aus und tippe durch den Signup-Flow".
* Bestätigen Sie, dass Xcode und die iOS-Simulatoren installiert sind und dass Ihre Xcode-Version die [Anforderungen](#requirements) erfüllt.
* Wenn Ihre Organisation Claude Code verwaltet, können die [Simulator-Tools durch Policy deaktiviert sein](#turn-off-simulator-access).
* Wenn Sie sich in einer Enterprise-Organisation mit aktivierter HIPAA-Konfiguration befinden, ist der Simulator-Bereich für Sie nicht verfügbar.
* Der Simulator-Bereich erfordert Claude Desktop v1.24012.0 oder später. Öffnen Sie **Claude → Check for Updates** und starten Sie die App neu.

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  Der Simulator-Bereich sagt, dass keine Simulatoren gefunden wurden
</h3>

Wenn `xcode-select` auf Xcode 27 verweist, kann der Bereich melden, dass keine Simulatoren gefunden wurden, obwohl Geräte vorhanden sind; siehe [Der Simulator-Bereich schlägt mit Xcode 27 fehl](#the-simulator-pane-fails-with-xcode-27). Andernfalls ist Xcode installiert, hat aber keine iOS-Simulatoren zum Auflisten. Der Simulator-Bereich zeigt die zu befolgenden Einrichtungsschritte an und markiert sie, wenn jeder abgeschlossen ist. Um das fehlende Teil manuell zu installieren, laden Sie die iOS-Simulator-Laufzeit aus den Einstellungen von Xcode herunter, oder führen Sie `xcodebuild -downloadPlatform iOS` aus.

<h3 id="the-simulator-pane-fails-with-xcode-27">
  Der Simulator-Bereich schlägt mit Xcode 27 fehl
</h3>

Der Bereich funktioniert noch nicht mit Xcode 27, das die Simulator-App durch Device Hub ersetzt. Mit Xcode 27 ausgewählt schlägt das Angehängen eines Geräts fehl, oder der Bereich meldet, dass keine Simulatoren gefunden wurden, obwohl Geräte vorhanden sind.

Der Bereich nutzt das Xcode, auf das `xcode-select` verweist. Wenn Xcode 27 Ihre einzige Installation ist, installieren Sie zuerst Xcode 26.x daneben. Wählen Sie dann die 26.x-Installation nach ihrem Pfad aus. Wenn es beispielsweise als `/Applications/Xcode-26.4.app` installiert ist:

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

Führen Sie `xcode-select -p` aus, um zu überprüfen, welche Installation ausgewählt ist.

<h2 id="see-also">
  Siehe auch
</h2>

* [Computernutzung in Desktop](/docs/de/desktop#let-claude-use-your-computer): Bildschirmsteuerung für Apps ohne einen dedizierten Bereich
* [Computernutzung über die CLI](/docs/de/computer-use): wie die CLI den iOS-Simulator erreicht
* [Arbeiten Sie parallel mit Sitzungen](/docs/de/desktop#work-in-parallel-with-sessions): wie Sitzungen Änderungen isolieren
* [Erste Schritte mit Claude Code Desktop](/docs/de/desktop-quickstart)
