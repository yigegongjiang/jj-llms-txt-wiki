> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Erste Schritte mit Claude Code in der Cloud

> Führen Sie Claude Code in der Cloud aus Ihrem Browser oder Telefon aus. Verbinden Sie ein GitHub-Repository, übermitteln Sie eine Aufgabe und überprüfen Sie den PR ohne lokales Setup.

<Note>
  Cloud-Sitzungen sind in Pro-, Max- und Team-Plänen sowie für Enterprise-Benutzer mit Premium-Sitzen oder Chat + Claude Code-Sitzen verfügbar.
</Note>

Eine Cloud-Sitzung führt Claude Code auf Cloud-Infrastruktur anstelle Ihres Computers aus, standardmäßig von Anthropic verwaltet. Dieser Schnellstart startet eine Sitzung von [claude.ai/code](https://claude.ai/code) in Ihrem Browser. Sie können auch eine Sitzung aus der Claude-Mobilanwendung, der Desktop-Anwendung oder Ihrem Terminal mit `claude --cloud` starten.

Sie benötigen ein GitHub-Repository, um [zu beginnen](#connect-github). Claude klont es in eine isolierte virtuelle Maschine, nimmt Änderungen vor und pusht einen Branch zur Überprüfung. Sitzungen bleiben über Geräte hinweg bestehen, sodass eine Aufgabe, die Sie auf Ihrem Laptop starten, später von Ihrem Telefon aus überprüft werden kann.

Cloud-Sitzungen funktionieren gut für:

* **Parallele Aufgaben**: Führen Sie mehrere unabhängige Aufgaben gleichzeitig aus, jede in ihrer eigenen Sitzung und ihrem eigenen Branch, ohne mehrere Worktrees zu verwalten
* **Repositories, die Sie nicht lokal haben**: Claude klont das Repository bei jeder Sitzung neu, sodass Sie es nicht auschecken müssen
* **Aufgaben, die keine häufige Steuerung benötigen**: Übermitteln Sie eine gut definierte Aufgabe, machen Sie etwas anderes und überprüfen Sie das Ergebnis, wenn Claude fertig ist
* **Code-Fragen und Erkundung**: Verstehen Sie eine Codebasis oder verfolgen Sie, wie eine Funktion implementiert wird, ohne einen lokalen Checkout

Für Arbeiten, die Ihre lokale Konfiguration, Tools oder Umgebung benötigen, ist die lokale Ausführung von Claude Code oder die Verwendung von [Remote Control](/docs/de/remote-control) besser geeignet.

<h2 id="how-sessions-run">
  Wie Sitzungen ablaufen
</h2>

Die folgenden Schritte beschreiben von Anthropic gehostete Sitzungen. In einer [selbst gehosteten Umgebung](/docs/de/self-hosted-environments) werden der Klon und alles danach auf den eigenen Runnern Ihrer Organisation ausgeführt, wobei Netzwerkgrenzen, Setup und Push-Verhalten vom Betreiber konfiguriert werden. Wenn Sie eine Aufgabe übermitteln:

1. **Klonen und vorbereiten**: Ihr Repository wird auf eine von Anthropic verwaltete VM geklont und Ihr [Setup-Skript](/docs/de/cloud-environments#setup-scripts) wird ausgeführt, falls konfiguriert.
2. **Netzwerk konfigurieren**: Der Internetzugriff wird basierend auf der [Zugriffsstufe](/docs/de/cloud-environments#access-levels) Ihrer Umgebung festgelegt.
3. **Arbeit**: Claude analysiert Code, nimmt Änderungen vor, führt Tests aus und überprüft seine Arbeit. Sie können zuschauen und die ganze Zeit über steuern oder weggehen und zurückkommen, wenn es fertig ist.
4. **Branch pushen**: Wenn Claude einen Haltepunkt erreicht, pusht es seinen Branch zu GitHub. Sie überprüfen den Diff, hinterlassen Inline-Kommentare, erstellen einen PR oder senden eine weitere Nachricht, um weiterzumachen.

Die Sitzung wird nicht geschlossen, wenn der Branch gepusht wird. PR-Erstellung und weitere Bearbeitungen erfolgen alle innerhalb desselben Gesprächs.

<h2 id="compare-ways-to-run-claude-code">
  Vergleichen Sie die Möglichkeiten, Claude Code auszuführen
</h2>

Claude Code verhält sich überall gleich. Was sich ändert, ist, wo die Sitzung ausgeführt wird und ob Ihre lokale Konfiguration verfügbar ist:

|                                                   | Cloud-Sitzung                                                                                                                   | Lokale Sitzung                                                                                                                      | Lokale Sitzung mit [Remote Control](/docs/de/remote-control)                        |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| **Code wird ausgeführt auf**                      | Cloud-VM, standardmäßig von Anthropic verwaltet                                                                                 | Ihrem Computer                                                                                                                      | Ihrem Computer                                                                 |
| **Sie starten sie von**                           | claude.ai/code, der Claude-Mobilanwendung, der Desktop-App mit **Cloud** ausgewählt, oder `claude --cloud`                      | Ihrem Terminal, Ihrer IDE oder der Desktop-App mit **Lokal** ausgewählt                                                             | Ihrem Terminal, der VS Code-Erweiterung oder der Desktop-App                   |
| **Sie chatten von**                               | claude.ai, der Mobilanwendung oder der Desktop-App                                                                              | Wo Sie sie gestartet haben                                                                                                          | claude.ai oder der Mobilanwendung, sowie von wo Sie sie gestartet haben        |
| **Verwendet Ihre lokale Konfiguration**           | Nein, nur Repository                                                                                                            | Ja                                                                                                                                  | Ja                                                                             |
| **Erfordert GitHub**                              | Ja, oder [bündeln Sie ein lokales Repository](/docs/de/claude-code-on-the-web#send-local-repositories-without-github) über `--cloud` | Nein                                                                                                                                | Nein                                                                           |
| **Läuft weiter, wenn Sie die Verbindung trennen** | Ja                                                                                                                              | Nein                                                                                                                                | Während die Sitzung auf Ihrem Computer offen bleibt                            |
| **[Berechtigungsmodi](/docs/de/permission-modes)**     | Änderungen akzeptieren, Plan, Auto                                                                                              | Alle Modi im Terminal; siehe [Berechtigungsmodi wechseln](/docs/de/permission-modes#switch-permission-modes) für die IDE und Desktop-App | Manuell, Änderungen akzeptieren oder Plan von claude.ai und der Mobilanwendung |
| **Netzwerkzugriff**                               | Konfigurierbar pro Umgebung                                                                                                     | Netzwerk Ihres Computers                                                                                                            | Netzwerk Ihres Computers                                                       |

Siehe die Dokumentation zu [Terminal-Schnellstart](/docs/de/quickstart), [Desktop-App](/docs/de/desktop) oder [Remote Control](/docs/de/remote-control), um lokale Sitzungen einzurichten.

<h2 id="connect-github">
  GitHub verbinden
</h2>

Das Verbinden von GitHub ist ein einmaliger Schritt. Wenn Sie bereits die GitHub CLI verwenden, können Sie [dies von Ihrem Terminal aus tun](#connect-from-your-terminal), anstatt den Browser zu verwenden.

<Note>
  Bei Team- und Enterprise-Plänen funktioniert der Schritt **Mit GitHub anmelden** nur, nachdem ein [Inhaber](/docs/de/server-managed-settings#access-control) Ihrer Claude-Organisation den GitHub-Connector unter [**Admin-Einstellungen > Connectors**](https://claude.ai/admin-settings/connectors) aktiviert hat. Bis dahin zeigt dieser Schritt „GitHub-Zugriff ist für Claude Code im Web erforderlich" anstelle einer Anmeldeschaltfläche an. Nachdem der Connector aktiviert ist, laden Sie [claude.ai/code](https://claude.ai/code) neu und beginnen Sie erneut mit dem ersten Schritt. Ein zweiter Schalter, [Schnelles Web-Setup](/docs/de/claude-code-on-the-web#github-authentication-options) unter [**Admin-Einstellungen > Claude Code**](https://claude.ai/admin-settings/claude-code), ist optional: Wenn er aktiviert ist, funktioniert `/web-setup` und das Onboarding erstellt die Umgebung für Mitglieder.
</Note>

<Steps>
  <Step title="Besuchen Sie claude.ai/code">
    Gehen Sie zu [claude.ai/code](https://claude.ai/code) und melden Sie sich mit Ihrem claude.ai-Konto an.
  </Step>

  <Step title="Mit GitHub anmelden">
    Nach der Anmeldung fordert Sie claude.ai/code auf, GitHub zu verbinden. Folgen Sie der Aufforderung, und claude.ai/code sendet Sie zur Autorisierungsseite von GitHub. Genehmigen Sie die Autorisierungsanfrage, und GitHub bringt Sie zurück zu claude.ai/code. Cloud-Sitzungen funktionieren mit vorhandenen GitHub-Repositories. Um ein neues Projekt zu starten, [erstellen Sie zunächst ein leeres Repository auf GitHub](https://github.com/new).

    Mit dieser Verbindung kann eine Sitzung jedes öffentliche Repository klonen, kann aber nur dann in einem privaten Repository arbeiten, wenn die Claude GitHub App darauf installiert ist. [Installieren Sie die Claude GitHub App](https://github.com/apps/claude/installations/new) auf jedem GitHub-Konto oder jeder Organisation, deren private Repositories Sie verwenden möchten. Bei einer GitHub-Organisation muss möglicherweise ein Organisationsinhaber die Installation genehmigen. Die Installation der App ermöglicht auch [Auto-fix](/docs/de/claude-code-on-the-web#auto-fix-pull-requests), das Claude ermöglicht, auf CI-Fehler und Review-Kommentare zu Pull Requests in diesen Repositories zu reagieren.

    Wenn das Onboarding Sie an dieser Stelle auffordert, die Claude GitHub App zu installieren, und Sie dies lieber später tun möchten, klicken Sie auf **Überspringen**.
  </Step>

  <Step title="Richten Sie Ihre Standardumgebung ein">
    Eine [Cloud-Umgebung](/docs/de/cloud-environments) ist die gespeicherte Konfiguration, die steuert, welchen Netzwerkzugriff Claude während Sitzungen hat und was ausgeführt wird, wenn eine Sitzung startet. Was nach dem Verbinden von GitHub passiert, hängt von Ihrem Plan ab:

    * **Pro und Max**: Das Onboarding erstellt eine Umgebung namens **Standard** für Sie.
    * **Team und Enterprise**: Das Onboarding zeigt ein Formular **Erstellen Sie Ihre erste Cloud-Umgebung**. Lassen Sie den vorausgefüllten Namen und Netzwerkzugriff unverändert und klicken Sie auf **Erstellen und fertig**, um die Umgebung **Standard** zu erstellen. Wenn ein Inhaber das [Schnelle Web-Setup](/docs/de/claude-code-on-the-web#github-authentication-options) aktiviert hat, erstellt das Onboarding stattdessen **Standard** für Sie.

    **Standard** verwendet [`Trusted` Netzwerkzugriff](/docs/de/cloud-environments#access-levels): Sitzungen erreichen [häufige Paketregistries](/docs/de/cloud-environments#default-allowed-domains) und andere auf die Whitelist gesetzte Domains, und nichts anderes über das Netzwerk der Sitzung. Siehe [Installierte Tools](/docs/de/cloud-environments#installed-tools) für das, was ohne Konfiguration verfügbar ist.

    Für ein erstes Projekt funktioniert die Umgebung **Standard** wie sie ist. Um ihren Netzwerkzugriff zu ändern, Umgebungsvariablen hinzuzufügen oder ein [Setup-Skript](/docs/de/cloud-environments#setup-scripts) vor dem Start von Sitzungen auszuführen, [bearbeiten Sie sie oder erstellen Sie zusätzliche Umgebungen](/docs/de/cloud-environments#configure-your-environment).
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Von Ihrem Terminal aus verbinden
</h3>

Wenn Sie bereits die GitHub CLI (`gh`) verwenden, können Sie GitHub für Cloud-Sitzungen von Ihrem Terminal aus verbinden. Dies erfordert die [Claude Code CLI](/docs/de/quickstart). Bei Team- und Enterprise-Plänen ist `/web-setup` nur verfügbar, nachdem ein Inhaber das [Schnelle Web-Setup](/docs/de/claude-code-on-the-web#github-authentication-options) aktiviert hat.

Wenn Sie `/web-setup` ausführen, liest Claude Code das Token, das `gh auth token` ausgibt, fordert Sie auf zu bestätigen, und sendet das Token an Anthropic. Anthropic speichert es verschlüsselt mit Ihrem claude.ai-Konto, und Ihre Cloud-Sitzungen verwenden es für GitHub-Zugriff, bis Sie es [entfernen](#remove-the-web-setup-token). Eine Cloud-Sitzung, die Sie selbst starten, kann dann auf jedes Repository zugreifen, auf das dieses Token zugreifen kann, ohne dass eine Claude GitHub App installiert werden muss. Threads in einem [Projekt](/docs/de/claude-projects#set-up-github-access) benötigen weiterhin die Claude GitHub App.

Wenn Sie GitHub bereits im Browser verbunden haben, warnt Sie `/web-setup`, dass das Fortfahren diese Verbindung für Ihre Cloud-Sitzungen ersetzt.

<Note>
  Organisationen mit aktivierter [Zero Data Retention](/docs/de/zero-data-retention) können `/web-setup` oder andere Cloud-Sitzungsfunktionen nicht verwenden. Wenn die GitHub CLI nicht installiert oder nicht authentifiziert ist, öffnet Claude Code stattdessen den Browser-Onboarding-Flow.
</Note>

<Steps>
  <Step title="Authentifizieren Sie sich mit der GitHub CLI">
    Authentifizieren Sie in Ihrer Shell die GitHub CLI, falls Sie dies noch nicht getan haben:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Melden Sie sich bei Claude an">
    Führen Sie in der Claude Code CLI `/login` aus, um sich mit Ihrem claude.ai-Konto anzumelden. Überspringen Sie diesen Schritt, wenn Sie bereits mit einem claude.ai-Konto angemeldet sind. Die Authentifizierung mit einem API-Schlüssel zählt nicht. Um zu überprüfen, führen Sie `/status` aus und bestätigen Sie, dass die Zeile **Login-Methode** ein claude.ai-Konto anzeigt.
  </Step>

  <Step title="Führen Sie /web-setup aus">
    Führen Sie in der Claude Code CLI Folgendes aus:

    ```text theme={null}
    /web-setup
    ```

    Bestätigen Sie die Aufforderung, um Ihr `gh`-Token an Ihr Claude-Konto zu senden. Bei Erfolg gibt Claude Code `Connected as <your-github-username>` aus und öffnet [claude.ai/code](https://claude.ai/code) in Ihrem Browser. Wenn Sie noch keine Cloud-Umgebung haben, erstellt `/web-setup` eine mit Trusted-Netzwerkzugriff und ohne Setup-Skript. Sie können [die Umgebung bearbeiten oder Variablen hinzufügen](/docs/de/cloud-environments#configure-your-environment) danach. Sobald `/web-setup` abgeschlossen ist, können Sie Cloud-Sitzungen von Ihrem Terminal aus mit [`--cloud`](/docs/de/claude-code-on-the-web#from-terminal-to-cloud) starten oder wiederkehrende Aufgaben mit [`/schedule`](/docs/de/routines) einrichten.
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  Entfernen Sie das `/web-setup`-Token
</h4>

Um das Token aus Ihrem Claude-Konto zu entfernen, trennen Sie GitHub unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Das Trennen löscht die GitHub-Anmeldedaten, die Ihre Cloud-Sitzungen verwenden, unabhängig davon, ob sie aus dem Browser oder von `/web-setup` stammen, sodass Cloud-Sitzungen den GitHub-Zugriff verlieren, bis Sie sich erneut verbinden. Ihr lokales `gh` bleibt angemeldet, und das Token bleibt auf GitHub gültig.

Um das Token selbst ungültig zu machen, widerrufen Sie es auf GitHub. Wenn Sie sich bei `gh` über den Browser angemeldet haben, gehört das Token zum Eintrag **GitHub CLI** unter [**Einstellungen > Anwendungen > Autorisierte OAuth-Apps**](https://github.com/settings/applications) auf GitHub, und das Widerrufen dieses Eintrags meldet auch die GitHub CLI auf Ihren Maschinen ab. Cloud-Sitzungen verlieren dann den GitHub-Zugriff, bis Sie `gh auth login` und `/web-setup` erneut ausführen.

<h2 id="start-a-task">
  Starten Sie eine Aufgabe
</h2>

Mit GitHub verbunden und einer erstellten Umgebung können Sie Aufgaben übermitteln.

<Steps>
  <Step title="Wählen Sie ein Repository und einen Branch">
    Von [claude.ai/code](https://claude.ai/code) oder der Code-Registerkarte in der Claude-Mobilanwendung klicken Sie auf den Repository-Selector unter dem Eingabefeld und wählen Sie ein Repository aus, in dem Claude arbeiten soll. Jedes Repository zeigt einen Branch-Selector. Ändern Sie ihn, um Claude von einem Feature-Branch anstelle des Standards zu starten. Sie können mehrere Repositories hinzufügen, um in einer Sitzung über sie hinweg zu arbeiten.
  </Step>

  <Step title="Wählen Sie einen Berechtigungsmodus">
    Der Modus-Dropdown neben der Eingabe zeigt den Modus, in dem die Sitzung ausgeführt wird:

    * **Auto**: Ein Klassifizierer überprüft Claudes Aktionen, anstatt Sie zu fragen. Wird angezeigt, wenn Ihre Organisation den Auto-Modus zulässt und das ausgewählte Modell ihn unterstützt
    * **Änderungen akzeptieren**: Claude nimmt Änderungen vor und pusht einen Branch, ohne auf Genehmigung zu warten
    * **Plan**: Claude schlägt einen Ansatz vor und wartet auf Ihre Genehmigung, bevor Dateien bearbeitet werden

    Cloud-Sitzungen bieten keine Manual- oder Bypass-Berechtigungen. Siehe die [vollständige Liste der Berechtigungsmodi](/docs/de/permission-modes#available-modes) für das, was jeder Modus erlaubt.
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
| `environment`  | Name oder ID der [Umgebung](#connect-github) zum Vorauswählen.                                                                                                                                                                           |

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

    Der Diff vergleicht die Änderungen der Sitzung standardmäßig mit ihrem Basis-Branch. Um gegen einen anderen Branch zu vergleichen, wählen Sie **Vergleichen gegen** und wählen Sie einen aus.
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

Wenn Sie GitHub im Browser verbunden haben, können Sitzungen jedes öffentliche Repository klonen, aber ein privates Repository wird nur angezeigt, wenn die Claude GitHub App auf dem Konto oder der Organisation installiert ist, das/die es besitzt, und der Zugriff der Installation auf Repositories es einschließt. [Installieren Sie die Claude GitHub App](https://github.com/apps/claude/installations/new) dort, oder bitten Sie einen Organisationsinhaber, sie zu installieren oder zu genehmigen.

Wenn Sie sich mit `/web-setup` verbunden haben, erreichen Sitzungen jedes Repository, auf das Ihr `gh`-Token zugreifen kann. Führen Sie `gh repo view OWNER/REPO` in Ihrer Shell aus, um zu überprüfen, dass Ihr GitHub CLI-Login das Repository sehen kann, und führen Sie `/web-setup` erneut aus, wenn Sie seit der Verbindung die `gh`-Konten gewechselt haben.

<h3 id="the-page-only-shows-a-github-login-button">
  Die Seite zeigt nur eine GitHub-Anmeldeschaltfläche
</h3>

Cloud-Sitzungen erfordern ein verbundenes GitHub-Konto. Verbinden Sie sich über den oben beschriebenen Browser-Flow oder führen Sie `/web-setup` von Ihrem Terminal aus aus, wenn Sie die GitHub CLI verwenden. Wenn Sie GitHub lieber gar nicht verbinden möchten, siehe [Remote Control](/docs/de/remote-control), um Claude Code auf Ihrem eigenen Computer auszuführen und es vom Browser oder Telefon aus zu überwachen.

<h3 id="not-available-for-the-selected-organization">
  „Nicht verfügbar für die ausgewählte Organisation"
</h3>

Enterprise-Organisationen müssen möglicherweise von einem Owner Cloud-Sitzungen aktivieren lassen. Kontaktieren Sie Ihr Anthropic-Account-Team.

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup` sagt „Nicht angemeldet bei Claude"
</h3>

Wenn `/web-setup` mit „Nicht angemeldet bei Claude. Führen Sie zuerst /login aus." antwortet, hat die CLI keine gültige claude.ai-Anmeldung. Dies kann auch vorkommen, wenn eine vorherige Anmeldung abgelaufen ist. Führen Sie `/login` aus, melden Sie sich mit Ihrem claude.ai-Konto an und führen Sie dann `/web-setup` erneut aus.

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup` warnt, dass Ihr Token nicht den `workflow`-Bereich hat
</h3>

Wenn `/web-setup` sagt, dass Ihr GitHub CLI-Token nicht den `workflow`-Bereich hat, können Sie fortfahren, aber GitHub kann einige Pushes mit diesem Token ablehnen, z. B. Pushes, die GitHub Actions-Workflow-Dateien ändern. Um den Bereich hinzuzufügen, führen Sie `gh auth refresh -s workflow` in Ihrer Shell aus und führen Sie dann `/web-setup` erneut aus.

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` zeigt „Keine Befehle stimmen überein" oder „Unbekannter Befehl"
</h3>

`/web-setup` wird in der Claude Code CLI ausgeführt, nicht in Ihrer Shell. Starten Sie zunächst `claude` und geben Sie dann `/web-setup` an der Eingabeaufforderung ein.

Wenn Sie es in Claude Code eingegeben haben und das Befehlsmenü `Keine Befehle stimmen überein "/web-setup"` anzeigt oder das Absenden `Unbekannter Befehl: /web-setup` zurückgibt, ist der Befehl verborgen, weil eine Anforderung nicht erfüllt ist. Die Ursache ist normalerweise, dass Sie mit einem API-Schlüssel oder einem Drittanbieter-Provider authentifiziert sind, anstatt mit einem claude.ai-Abonnement. Führen Sie `/login` aus, um sich mit Ihrem claude.ai-Konto anzumelden.

Bei Team- und Enterprise-Plänen ist der Befehl standardmäßig verborgen: Der [Quick web setup toggle](/docs/de/claude-code-on-the-web#github-authentication-options) ist ausgeschaltet, bis ein Owner ihn einschaltet. Während er ausgeschaltet ist, [verbinden Sie GitHub stattdessen vom Browser](#connect-github) aus.

Der Befehl ist auch verborgen in zwei weiteren Fällen:

* Ein Administrator hat Cloud-Sitzungen für Ihre Organisation deaktiviert. In diesem Fall gibt das Absenden von `/web-setup` [`Cloud-Sitzungen sind durch die Richtlinie Ihrer Organisation deaktiviert`](/docs/de/errors#cloud-sessions-are-disabled-by-your-organizations-policy) zurück. Vor v2.1.268 gab dieser Fall auch `Unbekannter Befehl: /web-setup` zurück.
* Ihre Enterprise-Organisation hat [Zero Data Retention](/docs/de/zero-data-retention) aktiviert, was Cloud-Sitzungen nicht verfügbar macht.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  „Cloud-Umgebung konnte nicht erstellt werden" oder „Keine Cloud-Umgebung verfügbar" bei Verwendung von `--cloud`
</h3>

Cloud-Sitzungsfunktionen erstellen automatisch eine Standard-Cloud-Umgebung, wenn Sie noch keine haben. Wenn Sie „Cloud-Umgebung konnte nicht erstellt werden" sehen, ist die automatische Erstellung fehlgeschlagen. Wenn Sie „Keine Cloud-Umgebung verfügbar" sehen, ist Ihre CLI älter als die automatische Erstellung. Führen Sie in beiden Fällen `/web-setup` in der Claude Code CLI aus, oder fügen Sie eine Umgebung aus dem [Umgebungsauswahl](/docs/de/cloud-environments#configure-your-environment) unter [claude.ai/code](https://claude.ai/code) hinzu.

<h3 id="setup-script-failed">
  Setup-Skript fehlgeschlagen
</h3>

Das Setup-Skript wurde mit einem Nicht-Null-Status beendet, was den Start der Sitzung blockiert. Häufige Ursachen:

* Eine Paketinstallation ist fehlgeschlagen, weil die Registry nicht in Ihrer [Zugriffsstufe](/docs/de/cloud-environments#access-levels) enthalten ist. `Trusted` deckt die meisten Paketmanager ab; `None` blockiert sie alle.
* Das Skript verweist auf eine Datei oder einen Pfad, der in einem frischen Klon nicht vorhanden ist.
* Ein Befehl, der lokal funktioniert, benötigt einen anderen Aufruf auf Ubuntu.

Zum Debuggen fügen Sie `set -x` oben im Skript hinzu, um zu sehen, welcher Befehl fehlgeschlagen ist. Für nicht kritische Befehle fügen Sie `|| true` an, damit sie den Sitzungsstart nicht blockieren.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Neue Sitzungen hängen oder treten während des Setups in einen Timeout auf
</h3>

Wenn neue Sitzungen beim Setup-Skript-Schritt steckenbleiben oder mit einem generischen Container-Fehler fehlschlagen, bevor das Skript fertig ist, überschreitet das Skript wahrscheinlich das ungefähre fünfminütige Zeitbudget für die Erstellung des [Umgebungs-Cache](/docs/de/cloud-environments#environment-caching). Schwere Schritte wie das Abrufen großer Docker-Images, das Synchronisieren vollständiger Abhängigkeitsbäume oder das Herunterladen von Modellgewichten überschreiten oft die Grenze, besonders wenn sie nacheinander ausgeführt werden.

Um dies zu beheben, kürzen Sie das Skript, damit es zuverlässig in unter fünf Minuten fertig wird:

* Führen Sie unabhängige Installationen parallel mit `&` und einem abschließenden `wait` aus, anstatt sie nacheinander auszuführen.
* Verschieben Sie die größten Downloads aus dem Setup-Skript in einen [SessionStart Hook](/docs/de/cloud-environments#setup-scripts-vs-sessionstart-hooks), der sie im Hintergrund startet, damit die Sitzung nutzbar wird, während sie fertig werden.
* Entfernen Sie lange Wiederholungs-Sleeps aus dem Setup-Skript, da eine steckengebliebene Wiederholungsschleife gegen das Budget zählt.

<h3 id="session-keeps-running-after-closing-the-tab">
  Sitzung läuft weiter nach dem Schließen der Registerkarte
</h3>

Dies ist beabsichtigt. Das Schließen der Registerkarte oder das Navigieren weg stoppt die Sitzung nicht. Sie läuft im Hintergrund weiter, bis Claude die aktuelle Aufgabe beendet, dann wird sie untätig. Aus der Seitenleiste können Sie [eine Sitzung archivieren](/docs/de/claude-code-on-the-web#archive-sessions), um sie aus Ihrer Liste auszublenden, oder [sie löschen](/docs/de/claude-code-on-the-web#delete-sessions), um sie dauerhaft zu entfernen.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Aufgaben übermitteln und überprüfen können, behandeln diese Seiten das, was als Nächstes kommt: Cloud-Sitzungen von Ihrem Terminal aus starten, wiederkehrende Arbeiten planen und Claude ständige Anweisungen geben.

* [Verwenden Sie Claude Code im Web](/docs/de/claude-code-on-the-web): die vollständige Referenz, einschließlich Teleportieren von Sitzungen zu Ihrem Terminal, Sitzungsfreigabe und automatisches Beheben von Pull Requests
* [Konfigurieren Sie Cloud-Umgebungen](/docs/de/cloud-environments): Netzwerkzugriffsstufen, Umgebungsvariablen und Setup-Skripte für Cloud-Sitzungen
* [Routinen](/docs/de/routines): Automatisieren Sie Arbeiten nach einem Zeitplan, über einen API-Aufruf oder als Reaktion auf GitHub-Ereignisse
* [CLAUDE.md](/docs/de/memory): Geben Sie Claude ständige Anweisungen und Kontext, die zu Beginn jeder Sitzung geladen werden
* Installieren Sie die Claude-Mobilanwendung für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) oder [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude), um Sitzungen von Ihrem Telefon aus zu überwachen. Aus der Claude Code CLI zeigt `/mobile` einen QR-Code für [claude.ai/mobile](https://claude.ai/mobile) an, der den richtigen App Store für Ihr Telefon öffnet.
