> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Erste Schritte mit Claude Code im Web

> Führen Sie Claude Code in der Cloud aus Ihrem Browser oder Telefon aus. Verbinden Sie ein GitHub-Repository, übermitteln Sie eine Aufgabe und überprüfen Sie den PR ohne lokales Setup.

<Note>
  Claude Code im Web befindet sich in der Forschungsvorschau für Pro-, Max- und Team-Benutzer sowie für Enterprise-Benutzer mit Premium-Sitzen oder Chat + Claude Code-Sitzen.
</Note>

Claude Code im Web wird auf von Anthropic verwalteter Cloud-Infrastruktur ausgeführt, anstatt auf Ihrem Computer. Übermitteln Sie Aufgaben von [claude.ai/code](https://claude.ai/code) in Ihrem Browser oder der Claude-Mobilanwendung.

Sie benötigen ein GitHub-Repository, um [zu beginnen](#connect-github-and-create-an-environment). Claude klont es in eine isolierte virtuelle Maschine, nimmt Änderungen vor und pusht einen Branch zur Überprüfung. Sitzungen bleiben über Geräte hinweg bestehen, sodass eine Aufgabe, die Sie auf Ihrem Laptop starten, später von Ihrem Telefon aus überprüft werden kann.

Claude Code im Web funktioniert gut für:

* **Parallele Aufgaben**: Führen Sie mehrere unabhängige Aufgaben gleichzeitig aus, jede in ihrer eigenen Sitzung und ihrem eigenen Branch, ohne mehrere Worktrees zu verwalten
* **Repositories, die Sie nicht lokal haben**: Claude klont das Repository bei jeder Sitzung neu, sodass Sie es nicht auschecken müssen
* **Aufgaben, die keine häufige Steuerung benötigen**: Übermitteln Sie eine gut definierte Aufgabe, machen Sie etwas anderes und überprüfen Sie das Ergebnis, wenn Claude fertig ist
* **Code-Fragen und Erkundung**: Verstehen Sie eine Codebasis oder verfolgen Sie, wie eine Funktion implementiert wird, ohne einen lokalen Checkout

Für Arbeiten, die Ihre lokale Konfiguration, Tools oder Umgebung benötigen, ist die lokale Ausführung von Claude Code oder die Verwendung von [Remote Control](/docs/de/remote-control) besser geeignet.

<h2 id="how-sessions-run">
  Wie Sitzungen ablaufen
</h2>

Wenn Sie eine Aufgabe übermitteln:

1. **Klonen und vorbereiten**: Ihr Repository wird auf eine von Anthropic verwaltete VM geklont und Ihr [Setup-Skript](/docs/de/claude-code-on-the-web#setup-scripts) wird ausgeführt, falls konfiguriert.
2. **Netzwerk konfigurieren**: Der Internetzugriff wird basierend auf der [Zugriffsstufe](/docs/de/claude-code-on-the-web#access-levels) Ihrer Umgebung festgelegt.
3. **Arbeit**: Claude analysiert Code, nimmt Änderungen vor, führt Tests aus und überprüft seine Arbeit. Sie können zuschauen und die ganze Zeit über steuern oder weggehen und zurückkommen, wenn es fertig ist.
4. **Branch pushen**: Wenn Claude einen Haltepunkt erreicht, pusht es seinen Branch zu GitHub. Sie überprüfen den Diff, hinterlassen Inline-Kommentare, erstellen einen PR oder senden eine weitere Nachricht, um weiterzumachen.

Die Sitzung wird nicht geschlossen, wenn der Branch gepusht wird. PR-Erstellung und weitere Bearbeitungen erfolgen alle innerhalb desselben Gesprächs.

<h2 id="compare-ways-to-run-claude-code">
  Vergleichen Sie die Möglichkeiten, Claude Code auszuführen
</h2>

Claude Code verhält sich überall gleich. Was sich ändert, ist, wo Code ausgeführt wird und ob Ihre lokale Konfiguration verfügbar ist. Die Desktop-App bietet sowohl lokale als auch Cloud-Sitzungen, daher hängen die Antworten unten davon ab, welche Sie wählen:

|                                                   | Im Web                                                                                                                          | Remote Control                                   | Terminal CLI             | Desktop-App                    |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------- | :----------------------- | :----------------------------- |
| **Code wird ausgeführt auf**                      | Anthropic Cloud VM                                                                                                              | Ihrem Computer                                   | Ihrem Computer           | Ihrem Computer oder Cloud VM   |
| **Sie chatten von**                               | claude.ai oder Mobilanwendung                                                                                                   | claude.ai oder Mobilanwendung                    | Ihrem Terminal           | Der Desktop-Benutzeroberfläche |
| **Verwendet Ihre lokale Konfiguration**           | Nein, nur Repository                                                                                                            | Ja                                               | Ja                       | Ja für lokal, nein für Cloud   |
| **Erfordert GitHub**                              | Ja, oder [bündeln Sie ein lokales Repository](/docs/de/claude-code-on-the-web#send-local-repositories-without-github) über `--cloud` | Nein                                             | Nein                     | Nur für Cloud-Sitzungen        |
| **Läuft weiter, wenn Sie die Verbindung trennen** | Ja                                                                                                                              | Während das Terminal offen bleibt                | Nein                     | Hängt vom Sitzungstyp ab       |
| **[Berechtigungsmodi](/docs/de/permission-modes)**     | Änderungen akzeptieren, Plan, Auto                                                                                              | Fragen, Änderungen automatisch akzeptieren, Plan | Alle Modi                | Hängt vom Sitzungstyp ab       |
| **Netzwerkzugriff**                               | Konfigurierbar pro Umgebung                                                                                                     | Netzwerk Ihres Computers                         | Netzwerk Ihres Computers | Hängt vom Sitzungstyp ab       |

Siehe die Dokumentation zu [Terminal-Schnellstart](/docs/de/quickstart), [Desktop-App](/docs/de/desktop) oder [Remote Control](/docs/de/remote-control), um diese einzurichten.

<h2 id="connect-github-and-create-an-environment">
  GitHub verbinden und eine Umgebung erstellen
</h2>

Das Setup ist ein einmaliger Prozess. Wenn Sie bereits die GitHub CLI verwenden, können Sie [dies von Ihrem Terminal aus tun](#connect-from-your-terminal), anstatt den Browser zu verwenden.

<Steps>
  <Step title="Besuchen Sie claude.ai/code">
    Gehen Sie zu [claude.ai/code](https://claude.ai/code) und melden Sie sich mit Ihrem Anthropic-Konto an.
  </Step>

  <Step title="Installieren Sie die Claude GitHub App">
    Nach der Anmeldung fordert Sie claude.ai/code auf, GitHub zu verbinden. Folgen Sie der Aufforderung, um die Claude GitHub App zu installieren und ihr Zugriff auf Ihre Repositories zu gewähren. Cloud-Sitzungen funktionieren mit vorhandenen GitHub-Repositories. Um ein neues Projekt zu starten, [erstellen Sie zunächst ein leeres Repository auf GitHub](https://github.com/new).
  </Step>

  <Step title="Erstellen Sie Ihre Umgebung">
    Nach dem Verbinden von GitHub werden Sie aufgefordert, eine Cloud-Umgebung zu erstellen. Die Umgebung steuert, welchen Netzwerkzugriff Claude während Sitzungen hat und was ausgeführt wird, wenn eine neue Sitzung erstellt wird. Siehe [Installierte Tools](/docs/de/claude-code-on-the-web#installed-tools) für das, was ohne Konfiguration verfügbar ist.

    Das Formular hat diese Felder:

    * **Name**: eine Anzeigebeschriftung. Nützlich, wenn Sie mehrere Umgebungen für verschiedene Projekte oder Zugriffsstufen haben.
    * **Netzwerkzugriff**: steuert, was die Sitzung im Internet erreichen kann. Der Standard, `Trusted`, ermöglicht Verbindungen zu [häufigen Paketregistern](/docs/de/claude-code-on-the-web#default-allowed-domains) wie npm, PyPI und RubyGems, während der allgemeine Internetzugriff blockiert wird.
    * **Umgebungsvariablen**: optionale Variablen, die in jeder Sitzung verfügbar sind, im `.env`-Format. Umschließen Sie Werte nicht mit Anführungszeichen, da Anführungszeichen als Teil des Werts gespeichert werden. Diese sind für jeden sichtbar, der diese Umgebung bearbeiten kann.
    * **Setup-Skript**: ein optionales Bash-Skript, das vor dem Start von Claude Code ausgeführt wird. Verwenden Sie es, um System-Tools zu installieren, die die Cloud VM nicht enthält, wie `apt install -y gh`. Das Ergebnis wird [zwischengespeichert](/docs/de/claude-code-on-the-web#environment-caching), sodass das Skript nicht bei jeder Sitzung erneut ausgeführt wird. Siehe [Setup-Skripte](/docs/de/claude-code-on-the-web#setup-scripts) für Beispiele und Debugging-Tipps.

    Lassen Sie für ein erstes Projekt die Standardeinstellungen und klicken Sie auf **Umgebung erstellen**. Sie können [sie später bearbeiten oder zusätzliche Umgebungen erstellen](/docs/de/claude-code-on-the-web#configure-your-environment) für verschiedene Projekte.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Von Ihrem Terminal aus verbinden
</h3>

Wenn Sie bereits die GitHub CLI (`gh`) verwenden, können Sie Claude Code im Web ohne das Öffnen eines Browsers einrichten. Dies erfordert die [Claude Code CLI](/docs/de/quickstart). `/web-setup` liest Ihr lokales `gh`-Token, verknüpft es mit Ihrem Claude-Konto und erstellt eine Standard-Cloud-Umgebung, wenn Sie noch keine haben.

<Note>
  Organisationen mit aktivierter [Zero Data Retention](/docs/de/zero-data-retention) können `/web-setup` oder andere Cloud-Sitzungsfunktionen nicht verwenden. Wenn die GitHub CLI nicht installiert oder authentifiziert ist, öffnet `/web-setup` stattdessen den Browser-Onboarding-Flow.
</Note>

<Steps>
  <Step title="Authentifizieren Sie sich mit der GitHub CLI">
    Authentifizieren Sie in Ihrer Shell die GitHub CLI, falls Sie dies noch nicht getan haben:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Melden Sie sich bei Claude an">
    Führen Sie in der Claude Code CLI `/login` aus, um sich mit Ihrem claude.ai-Konto anzumelden. Überspringen Sie diesen Schritt, wenn Sie bereits angemeldet sind.
  </Step>

  <Step title="Führen Sie /web-setup aus">
    Führen Sie in der Claude Code CLI Folgendes aus:

    ```text theme={null}
    /web-setup
    ```

    Dies synchronisiert Ihr `gh`-Token mit Ihrem Claude-Konto. Wenn Sie noch keine Cloud-Umgebung haben, erstellt `/web-setup` eine mit Trusted-Netzwerkzugriff und ohne Setup-Skript. Sie können [die Umgebung bearbeiten oder Variablen hinzufügen](/docs/de/claude-code-on-the-web#configure-your-environment) danach. Sobald `/web-setup` abgeschlossen ist, können Sie Cloud-Sitzungen von Ihrem Terminal aus mit [`--cloud`](/docs/de/claude-code-on-the-web#from-terminal-to-web) starten oder wiederkehrende Aufgaben mit [`/schedule`](/docs/de/routines) einrichten.
  </Step>
</Steps>

<h2 id="start-a-task">
  Starten Sie eine Aufgabe
</h2>

Mit GitHub verbunden und einer erstellten Umgebung können Sie Aufgaben übermitteln.

<Steps>
  <Step title="Wählen Sie ein Repository und einen Branch">
    Von [claude.ai/code](https://claude.ai/code) oder der Code-Registerkarte in der Claude-Mobilanwendung klicken Sie auf den Repository-Selector unter dem Eingabefeld und wählen Sie ein Repository aus, in dem Claude arbeiten soll. Jedes Repository zeigt einen Branch-Selector. Ändern Sie ihn, um Claude von einem Feature-Branch anstelle des Standards zu starten. Sie können mehrere Repositories hinzufügen, um in einer Sitzung über sie hinweg zu arbeiten.
  </Step>

  <Step title="Wählen Sie einen Berechtigungsmodus">
    Der Modus-Dropdown neben der Eingabe ist standardmäßig auf **Änderungen automatisch akzeptieren** eingestellt, wobei Claude Änderungen vornimmt und einen Branch pusht, ohne auf Genehmigung zu warten. Wechseln Sie zu **Plan-Modus**, wenn Claude einen Ansatz vorschlagen und auf Ihr Okay warten soll, bevor Dateien bearbeitet werden. Cloud-Sitzungen bieten keine Ask-Berechtigungen oder Bypass-Berechtigungen. Siehe die [vollständige Liste der Berechtigungsmodi](/docs/de/permission-modes#available-modes) für das, was jeder Modus erlaubt.
  </Step>

  <Step title="Beschreiben Sie die Aufgabe und übermitteln Sie sie">
    Geben Sie eine Beschreibung dessen ein, was Sie möchten, und drücken Sie die Eingabetaste. Seien Sie spezifisch:

    * Nennen Sie die Datei oder Funktion: „Fügen Sie eine README mit Setup-Anweisungen hinzu" oder „Beheben Sie den fehlgeschlagenen Auth-Test in `tests/test_auth.py`" ist besser als „Tests beheben"
    * Fügen Sie Fehlerausgabe ein, falls vorhanden
    * Beschreiben Sie das erwartete Verhalten, nicht nur das Symptom

    Claude klont die Repositories, führt Ihr Setup-Skript aus, falls konfiguriert, und beginnt zu arbeiten. Jede Aufgabe erhält ihre eigene Sitzung und ihren eigenen Branch, sodass Sie nicht warten müssen, bis eine fertig ist, bevor Sie eine andere starten.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  Sitzungen vorausfüllen
</h2>

Sie können die Eingabeaufforderung, Repositories und Umgebung für eine neue Sitzung vorausfüllen, indem Sie Abfrageparameter zur [claude.ai/code](https://claude.ai/code)-URL hinzufügen. Verwenden Sie dies, um Integrationen wie eine Schaltfläche in Ihrem Issue-Tracker zu erstellen, die Claude Code mit der Issue-Beschreibung als Eingabeaufforderung öffnet.

| Parameter      | Beschreibung                                                                                                                                                                                                                             |
| :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`       | Eingabeaufforderungstext zum Vorausfüllen im Eingabefeld. Der Alias `q` wird ebenfalls akzeptiert.                                                                                                                                       |
| `prompt_url`   | URL zum Abrufen des Eingabeaufforderungstexts, für Eingabeaufforderungen, die zu lang sind, um sie in eine Abfragezeichenfolge einzubetten. Die URL muss Cross-Origin-Anfragen zulassen. Wird ignoriert, wenn `prompt` auch gesetzt ist. |
| `repositories` | Kommagetrennte Liste von `owner/repo`-Slugs zum Vorauswählen. Der Alias `repo` wird ebenfalls akzeptiert.                                                                                                                                |
| `environment`  | Name oder ID der [Umgebung](#connect-github-and-create-an-environment) zum Vorauswählen.                                                                                                                                                 |

URL-codieren Sie jeden Wert. Das folgende Beispiel öffnet das Formular mit einer bereits ausgewählten Eingabeaufforderung und einem Repository:

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  Überprüfen und iterieren
</h2>

Wenn Claude fertig ist, überprüfen Sie die Änderungen, hinterlassen Sie Feedback zu bestimmten Zeilen und fahren Sie fort, bis der Diff richtig aussieht.

<Steps>
  <Step title="Öffnen Sie die Diff-Ansicht">
    Ein Diff-Indikator zeigt Zeilen, die über die Sitzung hinweg hinzugefügt und entfernt wurden, z. B. `+42 -18`. Wählen Sie ihn aus, um die Diff-Ansicht zu öffnen, mit einer Dateiliste auf der linken Seite und Änderungen auf der rechten Seite.
  </Step>

  <Step title="Hinterlassen Sie Inline-Kommentare">
    Wählen Sie eine beliebige Zeile im Diff aus, geben Sie Ihr Feedback ein und drücken Sie die Eingabetaste. Kommentare werden in die Warteschlange eingereiht, bis Sie Ihre nächste Nachricht senden, dann werden sie damit gebündelt. Claude sieht „bei `src/auth.ts:47`, den Fehler hier nicht abfangen" neben Ihrer Hauptanweisung, sodass Sie nicht beschreiben müssen, wo das Problem liegt.
  </Step>

  <Step title="Erstellen Sie einen Pull Request">
    Wenn der Diff richtig aussieht, wählen Sie **PR erstellen** oben in der Diff-Ansicht. Sie können ihn als vollständigen PR, als Entwurf öffnen oder zur Seite zum Verfassen von GitHub mit einem generierten Titel und einer Beschreibung springen.
  </Step>

  <Step title="Fahren Sie nach dem PR mit der Iteration fort">
    Die Sitzung bleibt nach der Erstellung des PR aktiv. Fügen Sie CI-Fehlerausgabe oder Reviewer-Kommentare in den Chat ein und bitten Sie Claude, sie zu beheben. Um Claude den PR automatisch überwachen zu lassen, siehe [Auto-fix Pull Requests](/docs/de/claude-code-on-the-web#auto-fix-pull-requests).
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  Beheben Sie Setup-Probleme
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  Nach dem Verbinden von GitHub werden keine Repositories angezeigt
</h3>

Eine Cloud-Sitzung kann jedes Repository verwenden, das das verbundene GitHub-Konto sehen kann, unabhängig davon, auf welchen Repositories die Claude GitHub App installiert ist. Wenn ein Repository fehlt, überprüfen Sie, ob das verbundene GitHub-Konto auf GitHub Zugriff darauf hat. Wenn Sie auch [Auto-fix](/docs/de/claude-code-on-the-web#auto-fix-pull-requests) für ein Repository möchten, installieren Sie die App darauf: Öffnen Sie auf github.com **Einstellungen → Anwendungen → Claude → Konfigurieren** und überprüfen Sie, ob das Repository unter **Repository-Zugriff** aufgeführt ist. Private Repositories benötigen die gleiche Autorisierung wie öffentliche.

<h3 id="the-page-only-shows-a-github-login-button">
  Die Seite zeigt nur eine GitHub-Anmeldeschaltfläche
</h3>

Cloud-Sitzungen erfordern ein verbundenes GitHub-Konto. Verbinden Sie sich über den oben beschriebenen Browser-Flow oder führen Sie `/web-setup` von Ihrem Terminal aus aus, wenn Sie die GitHub CLI verwenden. Wenn Sie GitHub lieber gar nicht verbinden möchten, siehe [Remote Control](/docs/de/remote-control), um Claude Code auf Ihrem eigenen Computer auszuführen und es vom Web aus zu überwachen.

<h3 id="not-available-for-the-selected-organization">
  „Nicht verfügbar für die ausgewählte Organisation"
</h3>

Enterprise-Organisationen müssen möglicherweise von einem Owner Claude Code im Web aktivieren lassen. Kontaktieren Sie Ihr Anthropic-Account-Team.

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` zeigt „Keine Befehle stimmen überein" oder „Unbekannter Befehl"
</h3>

`/web-setup` wird in der Claude Code CLI ausgeführt, nicht in Ihrer Shell. Starten Sie zunächst `claude` und geben Sie dann `/web-setup` an der Eingabeaufforderung ein.

Wenn Sie es in Claude Code eingegeben haben und das Befehlsmenü `Keine Befehle stimmen überein "/web-setup"` anzeigt oder das Absenden `Unbekannter Befehl: /web-setup` zurückgibt, ist der Befehl verborgen, weil eine Anforderung nicht erfüllt ist. Die Ursache ist normalerweise, dass Sie mit einem API-Schlüssel oder einem Drittanbieter-Provider authentifiziert sind, anstatt mit einem claude.ai-Abonnement. Führen Sie `/login` aus, um sich mit Ihrem claude.ai-Konto anzumelden.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  „Cloud-Umgebung konnte nicht erstellt werden" oder „Keine Cloud-Umgebung verfügbar" bei Verwendung von `--cloud` oder ultraplan
</h3>

Remote-Sitzungsfunktionen erstellen automatisch eine Standard-Cloud-Umgebung, wenn Sie noch keine haben. Wenn Sie „Cloud-Umgebung konnte nicht erstellt werden" sehen, ist die automatische Erstellung fehlgeschlagen. Wenn Sie „Keine Cloud-Umgebung verfügbar" sehen, ist Ihre CLI älter als die automatische Erstellung. Führen Sie in beiden Fällen `/web-setup` in der Claude Code CLI aus, um eine manuell zu erstellen, oder besuchen Sie [claude.ai/code](https://claude.ai/code) und folgen Sie dem Schritt **Erstellen Sie Ihre Umgebung** oben.

<h3 id="setup-script-failed">
  Setup-Skript fehlgeschlagen
</h3>

Das Setup-Skript wurde mit einem Nicht-Null-Status beendet, was den Start der Sitzung blockiert. Häufige Ursachen:

* Eine Paketinstallation ist fehlgeschlagen, weil die Registry nicht in Ihrer [Zugriffsstufe](/docs/de/claude-code-on-the-web#access-levels) enthalten ist. `Trusted` deckt die meisten Paketmanager ab; `None` blockiert sie alle.
* Das Skript verweist auf eine Datei oder einen Pfad, der in einem frischen Klon nicht vorhanden ist.
* Ein Befehl, der lokal funktioniert, benötigt einen anderen Aufruf auf Ubuntu.

Zum Debuggen fügen Sie `set -x` oben im Skript hinzu, um zu sehen, welcher Befehl fehlgeschlagen ist. Für nicht kritische Befehle fügen Sie `|| true` an, damit sie den Sitzungsstart nicht blockieren.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Neue Sitzungen hängen oder treten während des Setups in einen Timeout auf
</h3>

Wenn neue Sitzungen beim Setup-Skript-Schritt steckenbleiben oder mit einem generischen Container-Fehler fehlschlagen, bevor das Skript fertig ist, überschreitet das Skript wahrscheinlich das ungefähre fünfminütige Zeitbudget für die Erstellung des [Umgebungs-Cache](/docs/de/claude-code-on-the-web#environment-caching). Schwere Schritte wie das Abrufen großer Docker-Images, das Synchronisieren vollständiger Abhängigkeitsbäume oder das Herunterladen von Modellgewichten überschreiten oft die Grenze, besonders wenn sie nacheinander ausgeführt werden.

Um dies zu beheben, kürzen Sie das Skript, damit es zuverlässig in unter fünf Minuten fertig wird:

* Führen Sie unabhängige Installationen parallel mit `&` und einem abschließenden `wait` aus, anstatt sie nacheinander auszuführen.
* Verschieben Sie die größten Downloads aus dem Setup-Skript in einen [SessionStart Hook](/docs/de/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks), der sie im Hintergrund startet, damit die Sitzung nutzbar wird, während sie fertig werden.
* Entfernen Sie lange Wiederholungs-Sleeps aus dem Setup-Skript, da eine steckengebliebene Wiederholungsschleife gegen das Budget zählt.

<h3 id="session-keeps-running-after-closing-the-tab">
  Sitzung läuft weiter nach dem Schließen der Registerkarte
</h3>

Dies ist beabsichtigt. Das Schließen der Registerkarte oder das Navigieren weg stoppt die Sitzung nicht. Sie läuft im Hintergrund weiter, bis Claude die aktuelle Aufgabe beendet, dann wird sie untätig. Aus der Seitenleiste können Sie [eine Sitzung archivieren](/docs/de/claude-code-on-the-web#archive-sessions), um sie aus Ihrer Liste auszublenden, oder [sie löschen](/docs/de/claude-code-on-the-web#delete-sessions), um sie dauerhaft zu entfernen.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Aufgaben übermitteln und überprüfen können, behandeln diese Seiten das, was als Nächstes kommt: Cloud-Sitzungen von Ihrem Terminal aus starten, wiederkehrende Arbeiten planen und Claude ständige Anweisungen geben.

* [Verwenden Sie Claude Code im Web](/docs/de/claude-code-on-the-web): die vollständige Referenz, einschließlich Teleportieren von Sitzungen zu Ihrem Terminal, Setup-Skripte, Umgebungsvariablen und Netzwerkkonfiguration
* [Routinen](/docs/de/routines): Automatisieren Sie Arbeiten nach einem Zeitplan, über einen API-Aufruf oder als Reaktion auf GitHub-Ereignisse
* [CLAUDE.md](/docs/de/memory): Geben Sie Claude ständige Anweisungen und Kontext, die zu Beginn jeder Sitzung geladen werden
* Installieren Sie die Claude-Mobilanwendung für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) oder [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude), um Sitzungen von Ihrem Telefon aus zu überwachen. Aus der Claude Code CLI zeigt `/mobile` einen QR-Code an.
