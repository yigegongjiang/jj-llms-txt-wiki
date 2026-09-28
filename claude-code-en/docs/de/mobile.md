> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code auf Mobilgeräten

> Starten, überwachen und steuern Sie Claude Code-Aufgaben von Ihrem Telefon aus mit der Claude-App für iOS und Android.

Die Claude-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) und [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) ist ein Client für Claude Code-Sitzungen und nicht ein Ort, an dem Code ausgeführt wird. Von Ihrem Telefon aus erreichen Sie [Cloud-Sitzungen](#start-and-monitor-cloud-sessions) und [Projekte](/docs/de/claude-projects) in der Cloud, eine Sitzung, die auf Ihrem eigenen Computer über [Remote Control](#continue-a-local-session-with-remote-control) läuft, oder die Desktop-App über [Dispatch](/docs/de/desktop#sessions-from-dispatch).

<Note>
  Claude Code hat keine separate Mobile-App: Cloud-Sitzungen und Remote Control befinden sich beide im Tab **Code** in der Claude-App, und Dispatch ist eine Aufgabe, die Sie in der App anschreiben.
</Note>

<h2 id="get-the-app">
  App herunterladen
</h2>

<Steps>
  <Step title="Claude-App herunterladen">
    Installieren Sie die Claude-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) oder [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Auf einem iPad installieren Sie dieselbe iOS-App.

    <Tip>
      Führen Sie `/mobile` in einer Claude Code-Sitzung aus, um einen QR-Code für [claude.ai/mobile](https://claude.ai/mobile) anzuzeigen, der den richtigen App Store für Ihr Telefon öffnet. `/ios` und `/android` machen dasselbe.
    </Tip>
  </Step>

  <Step title="Anmelden">
    Melden Sie sich mit demselben claude.ai-Konto und derselben Organisation an, die Sie für Claude Code verwenden. Cloud-Sitzungen und Remote Control erfordern ein claude.ai-Konto, daher sind sie nicht mit einem Anthropic Console API-Schlüssel oder von einem Drittanbieter wie Amazon Bedrock erreichbar.
  </Step>

  <Step title="Öffnen Sie den Code-Tab">
    Tippen Sie in der Navigation der App auf **Code**, um Ihre Sitzungen zu erreichen, oder öffnen Sie [claude.ai/code/new](https://claude.ai/code/new) auf Ihrem Telefon, um eine neue Code-Sitzung in der App zu starten. Wenn Sie den Code-Tab nicht sehen, enthält Ihr Plan oder Ihre Organisation möglicherweise diese Funktionen nicht; siehe [Verfügbarkeit nach Abonnementplan](/docs/de/feature-availability#availability-by-subscription-plan).
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  Von Ihrem Telefon aus arbeiten
</h2>

Von der App aus können Sie Cloud-Sitzungen starten, ein Projekt öffnen, eine Claude Code-Sitzung auf Ihrem Computer steuern oder Dispatch eine Aufgabe anschreiben. Die App ist für alle gleich; sie unterscheiden sich darin, wo die Arbeit stattfindet.

| Funktion                                       | Womit Sie sich verbinden                                                           | Wann zu verwenden                                                                                                                                                                                               |
| :--------------------------------------------- | :--------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Cloud-Sitzungen](/docs/de/claude-code-on-the-web)  | Eine Sitzung auf Cloud-Infrastruktur, standardmäßig von Anthropic verwaltet        | Ihr Repository befindet sich auf GitHub und die Aufgabe sollte weiterhin ausgeführt werden, nachdem Sie Ihr Telefon weglegen. Siehe [Cloud-Schnellstart](/docs/de/web-quickstart), um die Einrichtung durchzuführen. |
| [Projekte](/docs/de/claude-projects)                | Eine Unterhaltung, in der Claude parallele Cloud-Sitzungen als Threads koordiniert | Sie haben einen Strom zusammenhängender Arbeit statt einer einzelnen Aufgabe und möchten sehen, welche Threads beendet wurden oder Sie benötigen.                                                               |
| [Remote Control](/docs/de/remote-control)           | Eine Claude Code-Sitzung, die auf Ihrem Computer läuft                             | Die Arbeit benötigt Ihr lokales Dateisystem, Tools oder MCP-Server.                                                                                                                                             |
| [Dispatch](/docs/de/desktop#sessions-from-dispatch) | Die Desktop-App auf Ihrem Computer                                                 | Sie möchten eine Aufgabe anschreiben und Dispatch entscheiden lassen, wie sie ausgeführt wird. Erfordert einen Pro- oder Max-Plan.                                                                              |

Wenn Ihr Computer ausgeschaltet ist, verwenden Sie Cloud-Sitzungen oder ein Projekt, die in der Cloud laufen und mit geschlossenem Laptop weiterlaufen. Remote Control und Dispatch steuern Ihren eigenen Computer, daher muss dieser eingeschaltet bleiben und Claude Code oder die Desktop-App muss laufen. Wenn Ihr Computer während einer Remote Control-Sitzung in den Ruhezustand wechselt, wird Claude Code wiederhergestellt, wenn der Computer wieder online kommt.

Einen umfassenderen Vergleich finden Sie unter [Arbeiten, wenn Sie nicht am Terminal sind](/docs/de/platforms#work-when-you-are-away-from-your-terminal).

Cloud-Sitzungen und Remote Control werden vom Tab **Code** aus ausgeführt. Für Dispatch, das Sie als Aufgabe in der App anschreiben, siehe [Sitzungen von Dispatch](/docs/de/desktop#sessions-from-dispatch).

<h3 id="start-and-monitor-cloud-sessions">
  Cloud-Sitzungen starten und überwachen
</h3>

Cloud-Sitzungen führen Aufgaben auf Cloud-Infrastruktur aus, standardmäßig von Anthropic verwaltet, daher wird eine Sitzung fortgesetzt, nachdem Sie Ihr Telefon weglegen. Wählen Sie im Code-Tab ein Repository und einen Branch aus, beschreiben Sie die Aufgabe und reichen Sie sie ein. Sitzungen bleiben über Geräte hinweg erhalten: Eine Aufgabe, die Sie auf Ihrem Laptop starten, ist bereit zur Überprüfung von Ihrem Telefon aus, und eine, die Sie von Ihrem Telefon aus starten, wartet, wenn Sie wieder an Ihrem Schreibtisch sind.

Öffnen Sie eine Sitzung in der App, um den Fortschritt zu überprüfen, Fragen von Claude zu beantworten oder sie in eine neue Richtung zu lenken. Sie können Claude auch anweisen, [einen Pull Request zu überwachen](/docs/de/claude-code-on-the-web#auto-fix-pull-requests) und CI-Fehler oder Überprüfungskommentare zu beheben, wenn sie eintreffen. Um GitHub zu verbinden und Ihre Umgebung einzurichten, folgen Sie dem [Cloud-Schnellstart](/docs/de/web-quickstart), und siehe [Claude Code im Web](/docs/de/claude-code-on-the-web) für alles, was Cloud-Sitzungen tun können.

<h3 id="continue-a-local-session-with-remote-control">
  Setzen Sie eine lokale Sitzung mit Remote Control fort
</h3>

Remote Control verbindet die Claude-App mit einer Claude Code-Sitzung auf Ihrem Computer, sodass die Code-Ausführung und der Dateisystemzugriff lokal bleiben, während Sie die Sitzung von Ihrem Telefon aus steuern. Starten Sie die Sitzung auf Ihrem Computer mit `claude remote-control`, oder führen Sie `/remote-control` in einer bereits offenen Sitzung aus. Scannen Sie dann den QR-Code, den das Terminal anzeigen kann, oder öffnen Sie die Claude-App, tippen Sie auf **Code** und wählen Sie die Sitzung aus der Liste aus. Siehe [Von einem anderen Gerät verbinden](/docs/de/remote-control#connect-from-another-device) für jede Option.

Wenn Sie einen Anhang in der Claude-App hinzufügen, erreicht er auch die lokale Sitzung:

* **Fotos**: Claude sieht angehängte Fotos direkt als Teil Ihrer Nachricht. Claude Code speichert auch jedes Foto unter `~/.claude/uploads/` und teilt Claude den gespeicherten Dateipfad mit, damit Claude das Bild in Dateien kopieren kann, die es erstellt.
* **Andere Dateien**: Claude Code lädt sie auf Ihren Computer herunter und übergibt sie Claude als `@`-Dateireferenzen.

Anforderungen, Aufrufmodi und Fehlerbehebung finden Sie in der [Remote Control-Übersicht](/docs/de/remote-control).

<h3 id="get-push-notifications">
  Push-Benachrichtigungen erhalten
</h3>

Wenn Remote Control aktiv ist, kann Claude Push-Benachrichtigungen an Ihr Telefon senden, normalerweise wenn eine lange laufende Aufgabe beendet wird oder wenn eine Entscheidung von Ihnen erforderlich ist. Sie können auch eine in Ihrem Prompt anfordern, z. B. `notify me when the tests finish`. Siehe [Mobile Push-Benachrichtigungen](/docs/de/remote-control#mobile-push-notifications) für die zwei `/config`-Umschalter und Fehlerbehebung bei der Zustellung.

Dispatch sendet seine eigene Benachrichtigung, wenn eine Code-Sitzung, die es erzeugt hat, beendet wird oder Ihre Genehmigung benötigt, beschrieben in [Sitzungen von Dispatch](/docs/de/desktop#sessions-from-dispatch).

<h2 id="limitations">
  Einschränkungen
</h2>

Der mobile Client deckt die meisten Anforderungen einer Sitzung ab, mit einigen Einschränkungen:

* **Nur lokal verfügbare Befehle**: Befehle, die nur in der Terminalschnittstelle ausgeführt werden, wie `/plugin` und `/resume`, funktionieren nicht aus der App. Die [Einschränkungen der Fernsteuerung](/docs/de/remote-control#limitations) listen die Befehle auf, die von mobilen Geräten aus funktionieren, und wie sich ihr Verhalten unterscheidet.
* **Berechtigungsmodi**: Cloud-Sitzungen bieten Bearbeitungen akzeptieren, Plan und Auto im Modus-Dropdown, und Remote Control-Sitzungen bieten Manuell, Bearbeitungen akzeptieren und Plan. Sie können Bypass-Berechtigungen nicht aus der App auswählen, in beiden Fällen nicht, und Sie können Auto nicht für eine Remote Control-Sitzung auswählen. Siehe [Berechtigungsmodi wechseln](/docs/de/permission-modes#switch-permission-modes).
* **Dispatch-Pläne**: Dispatch erfordert einen Pro- oder Max-Plan und ist nicht auf Team oder Enterprise verfügbar.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Plattformen und Integrationen](/docs/de/platforms): Vergleichen Sie jede Oberfläche, auf der Claude Code läuft
* [Claude Code im Web](/docs/de/claude-code-on-the-web): Wie Cloud-Sitzungen laufen und wie Sie Arbeit zu und von Ihrem Terminal verschieben
* [Cloud-Umgebungen konfigurieren](/docs/de/cloud-environments): Netzwerkzugriffsstufen, Umgebungsvariablen und Setup-Skripte für Cloud-Sitzungen
* [Remote Control](/docs/de/remote-control): Setzen Sie eine lokale Sitzung von jedem Gerät aus fort
* [Sitzungen von Dispatch](/docs/de/desktop#sessions-from-dispatch): Wie Dispatch-Aufgaben zu Code-Sitzungen in der Desktop-App werden
* [Channels](/docs/de/channels): Fragen Sie Claude von Ihrem Telefon aus über Telegram, Discord oder iMessage, während die Arbeit auf Ihrem Computer läuft
* [Claude Code in Slack](/docs/de/slack): Delegieren Sie Codierungsaufgaben von Ihrem Slack-Arbeitsbereich, indem Sie `@Claude` erwähnen
