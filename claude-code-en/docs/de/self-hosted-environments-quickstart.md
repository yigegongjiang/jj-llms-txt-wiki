> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Schnellstart für selbstgehostete Umgebungen

> Richten Sie Ihre erste selbstgehostete Umgebung ein: Installieren Sie Claude Code, erstellen Sie die Umgebung, starten Sie einen Runner und leiten Sie eine Sitzung dorthin weiter.

<Note>
  Selbstgehostete Umgebungen befinden sich in der öffentlichen Beta für Team- und Enterprise-Pläne; [Verfügbarkeit und Einschränkungen](/docs/de/self-hosted-environments#availability-and-limitations) behandelt den Aktivierungspfad. Diese Seite bringt Ihre erste Sitzung zum Laufen; siehe [Selbstgehostete Umgebungen](/docs/de/self-hosted-environments) für deren Funktionsweise und [Bereitstellung in der Produktion](/docs/de/self-hosted-environments-deploy) für Härtung und Fleet-Rezepte.
</Note>

Eine [selbstgehostete Umgebung](/docs/de/self-hosted-environments) führt Claude Code [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) auf einer Infrastruktur aus, die Ihre Organisation betreibt, ausgeführt durch Runner-Prozesse, die Sie bereitstellen. Dieser Schnellstart richtet Ihre erste ein, die kleinste, die funktioniert: ein Runner auf einem einzelnen Host, der eine Test-Sitzung ausführt. Es gibt zwei Schritte: [erstellen Sie die Umgebung, starten Sie einen Runner und leiten Sie eine Sitzung dorthin weiter](#set-up-an-environment-and-runner), dann [senden Sie eine Nachricht an diese Sitzung von Ihrem Terminal](#send-a-follow-up-message-to-a-running-session). Sie werden zwischen zwei Oberflächen wechseln: claude.ai zum Erstellen der Umgebung, Überprüfen ihres Status und Weiterleiten einer Sitzung, und ein Terminal auf dem Host für alles, was der Runner tut.

Am Ende haben Sie eine Umgebung auf der [**Cloud-Umgebungen** Admin-Seite](https://claude.ai/admin-settings/cloud-environments), einen Runner, der auf Arbeit wartet, und eine Sitzung, die auf Ihrem Host läuft. Bevor Sie echte Repositories oder interne Systeme verbinden, arbeiten Sie [Bereitstellung in der Produktion](/docs/de/self-hosted-environments-deploy) durch, die die Sicherheitslage, Egress-Kontrolle, Git-Anmeldedaten und Orchestrierung behandelt.

<h2 id="prerequisites">
  Voraussetzungen
</h2>

<h3 id="organization-and-roles">
  Organisation und Rollen
</h3>

Die claude.ai-Seite benötigt:

* **Selbstgehostete Umgebungen zulassen** aktiviert durch einen [Owner](/docs/de/cloud-environments#organization-shared-environments) auf der [**Cloud-Umgebungen** Admin-Seite](https://claude.ai/admin-settings/cloud-environments); die Schaltfläche **Neu** wird erst angezeigt, wenn dies der Fall ist. Wenn Sie diese Rolle nicht haben, kann jemand, der sie hat, die Umgebung erstellen und Ihnen ihr Geheimnis übergeben; die Runner- und Terminal-Schritte auf dieser Seite benötigen keine claude.ai-Rolle, und wo ein Schritt den Status in der Admin-Benutzeroberfläche überprüft, geben Ihnen die eigenen Protokollzeilen des Runners das gleiche Signal.
* Eine [GitHub-Verbindung](/docs/de/claude-code-on-the-web#github-authentication-options) für Ihre Organisation, damit Entwickler Repositories auswählen können, wenn sie Sitzungen starten.

<h3 id="host-and-network">
  Host und Netzwerk
</h3>

Der Runner-Host benötigt:

* Einen Linux- oder macOS-Host oder Container mit ausgehendem HTTPS zu `api.anthropic.com`, zu `claude.ai` und den Download-Hosts, auf die es für den Installationsschritt unten umleitet, und zu Ihrem Git-Host für den Klon; die [Netzwerkanforderungstabelle](/docs/de/self-hosted-environments-deploy#network-requirements) hat die vollständige Liste. Windows wird nicht als Runner-Host unterstützt; führen Sie den Runner stattdessen in einem Linux-Container aus. Entwickler-Workstations sind nicht betroffen, da Sitzungen von claude.ai in einem Browser aus gestartet werden.
* Eine Uhr, die mit der Realzeit synchronisiert ist, beispielsweise mit NTP. Die Authentifizierung schlägt fehl, wenn die Uhr um mehr als fünf Minuten abweicht; siehe [Troubleshooting](/docs/de/self-hosted-environments-deploy#troubleshooting).

<h3 id="software-on-the-runner-host">
  Software auf dem Runner-Host
</h3>

Installieren Sie auf dem Host, bevor Sie beginnen:

* **Claude Code v2.1.224 oder später**, mit einer der [Standard-Installationsmethoden](/docs/de/setup). Der Runner ist Teil der Standard-`claude`-Binärdatei, und frühere Versionen erkennen den `self-hosted-runner`-Unterbefehl nicht. Der native Installer's Standard-`latest`-Kanal trägt jede Version, sobald sie veröffentlicht wird; der `stable`-Kanal, das Homebrew `claude-code`-Cask und die stabilen apt-, dnf- und apk-Repositories liegen etwa eine Woche hinterher. Um die genaue Version zu fixieren, die Ihre Fleet ausführt, siehe [Installieren Sie eine bestimmte Version](/docs/de/setup#install-a-specific-version). Für Container-Images siehe die Dockerfile in [Bereitstellung in der Produktion](/docs/de/self-hosted-environments-deploy#build-the-runner-image).
* **Git 2.24 oder neuer**. Einige Git-Optionen auf der Deploy-Seite benötigen neuere Versionen; [Git konfigurieren](/docs/de/self-hosted-environments-deploy#configure-git) gibt jede Untergrenze an.

Bestätigen Sie, dass der Host bereit ist:

```bash theme={null}
claude self-hosted-runner --help
```

Ein bereiter Host gibt den Verwendungstext des Runners aus und listet Flags wie `--environment-secret-file` auf. Bei Versionen älter als 2.1.224 gibt der Befehl stattdessen die allgemeine `claude --help`-Ausgabe aus; aktualisieren Sie mit `claude update` oder installieren Sie neu vom `latest`-Kanal.

<h2 id="set-up-an-environment-and-runner">
  Richten Sie eine Umgebung und einen Runner ein
</h2>

Claude Code enthält ein geführtes Setup: eine interaktive Claude Code-Sitzung, die Sie durch das Erstellen der Umgebung in der Admin-Benutzeroberfläche führt, einen lokalen Runner mit der Geheimnis-Datei startet, die Sie speichern, bestätigt, dass sich der Runner registriert, und ein Spickzettel zu `./runner-setup/CHEAT-SHEET.md` schreibt. Führen Sie es auf einem Computer aus, auf dem Sie sich mit `claude auth login` mit einem Konto angemeldet haben, das eine Owner-Rolle hat; es ist nicht mit API-Schlüsseln oder Drittanbieter-Modellanbietern verfügbar. Auf Hosts, wo eine interaktive Sitzung nicht möglich ist, verwenden Sie stattdessen die manuellen Schritte unten. Bestätigen Sie zuerst, dass die [Versionsüberprüfung](#software-on-the-runner-host) bestanden wurde: Bei Versionen älter als 2.1.224 startet dieser Befehl eine gewöhnliche Claude-Sitzung mit den Wörtern als Eingabeaufforderung statt des geführten Setups. Um das geführte Setup zu starten, führen Sie den Setup-Unterbefehl aus und folgen Sie den Eingabeaufforderungen:

```bash theme={null}
claude self-hosted-runner setup
```

Um stattdessen manuell einzurichten:

<Steps>
  <Step title="Erstellen Sie eine Umgebung">
    Gehen Sie zur [**Cloud-Umgebungen** Seite](https://claude.ai/admin-settings/cloud-environments) in den Admin-Einstellungen. Wählen Sie unter **Selbstgehostete Umgebungen** die Option **Neu**, benennen Sie die Umgebung und wählen Sie **Erstellen**. Wählen Sie im zweiten Schritt des Assistenten **Umgebungsschlüssel kopieren**, um das Umgebungsgeheimnis zu kopieren, das die Admin-Benutzeroberfläche als Umgebungsschlüssel bezeichnet. claude.ai zeigt das Geheimnis einmal an, und Sie können es später nicht abrufen; es läuft 365 Tage nach der Erstellung ab. Die `ccpool_...`-ID der Umgebung bleibt in ihrem Detaildialog sichtbar; Sie benötigen sie für die `aud`-Überprüfung in [Token-Verifizierung](/docs/de/self-hosted-environments-identity) und zum Versenden von [Test-Sitzungen aus CI](/docs/de/self-hosted-environments-testing#run-the-test-loop).

    Wenn Sie das Geheimnis verlieren oder es rotieren müssen, erstellen Sie ein neues Geheimnis auf der Registerkarte **Konfiguration** der Umgebung, rollen Sie das neue Geheimnis auf Ihren Runnern aus und widerrufen Sie dann das alte. Runner, die ein widerrufenes Geheimnis halten, schlagen ihre nächste authentifizierte Abfrage fehl und beenden sich, protokollieren `poll auth failed`, und Ihr Orchestrator startet sie mit dem neuen Geheimnis neu.
  </Step>

  <Step title="Starten Sie einen Runner">
    Erstellen Sie das Geheimnis-Verzeichnis. Dieser Schritt und der nächste benötigen Root für den `/etc/claude`-Pfad; jeder Pfad, den der Runner-Prozess lesen kann, funktioniert, also passen Sie beide Befehle und den `--environment-secret-file`-Wert zusammen an, wenn Sie einen anderen verwenden.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    Schreiben Sie das Umgebungsgeheimnis in eine Datei. Der Befehl unten liest von Ihrem Terminal, damit das Geheimnis aus der Shell-Historie bleibt: Fügen Sie den Wert ein, den Sie kopiert haben, drücken Sie Enter, dann Strg-D, und die `umask` der Subshell macht die Datei nur für ihren Besitzer lesbar.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    Wählen Sie ein Basisverzeichnis und ersetzen Sie `<writable-dir>` im Runner-Befehl unten durch einen absoluten Pfad, in den der Runner schreiben oder erstellen kann. Der Runner erstellt das Verzeichnis beim Start, überprüft dann Repositories aus und erstellt Pro-Sitzungs-Verzeichnisse darunter. Ohne `--base-dir` verwendet er `/workspace`, was nur funktioniert, wenn dieses Verzeichnis bereits existiert und beschreibbar ist oder Sie den Runner als Root starten.

    Wenn der Runner nicht in den Pfad erstellen oder schreiben kann, beendet er sich beim Start mit einem Fehler, der das Verzeichnis benennt, statt sich zu registrieren. Siehe [Troubleshooting](/docs/de/self-hosted-environments-deploy#troubleshooting).

    Starten Sie dann den Runner mit `--environment-secret-file` und `--base-dir`. Der Runner registriert sich bei Ihrer Umgebung und beginnt, auf Arbeit zu warten. Wenn der Runner beendet wird, starten Sie ihn manuell neu. Produktionsbereitstellungen führen den Runner unter einem Orchestrator aus, der beendete Runner neu startet, normalerweise mit einem frischen Dateisystem pro Neustart; [Wiederverwendung eines vorgewärmten Checkouts](/docs/de/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) behandelt das unterstützte Persistent-Disk-Setup.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="Überprüfen Sie, ob der Runner angezeigt wird">
    Kehren Sie zur [**Cloud-Umgebungen** Seite](https://claude.ai/admin-settings/cloud-environments) zurück. Der Status Ihrer Umgebung ändert sich innerhalb weniger Sekunden nach dem Start des Runners von **Keine Runner bereitgestellt** zu **Healthy**; öffnen Sie die Umgebung und wählen Sie **Aktivität**, um den Runner selbst zu sehen.
  </Step>

  <Step title="Leiten Sie eine Sitzung zur Umgebung weiter">
    Starten Sie eine Sitzung unter claude.ai/code und wählen Sie Ihre Umgebung aus dem Umgebungs-Picker, wo selbstgehostete Umgebungen neben von Anthropic gehosteten angezeigt werden. Der Runner klont mit den Git-Anmeldedaten, die der Host bereits hat, also wählen Sie ein Repository, das dieser Host bereits klonen kann, oder ein öffentliches; Anmeldedaten-Optionen für private Repositories in der Produktion befinden sich auf [Git konfigurieren](/docs/de/self-hosted-environments-deploy#configure-git). Der nächste verfügbare Runner nimmt die wartende Sitzung auf und protokolliert `Picked up session <session-id>` zusammen mit seiner aktiven Anzahl und Kapazität, damit Sie aus der eigenen Ausgabe des Runners bestätigen können, welcher Host die Sitzung übernahm. Beobachten Sie die Sitzungsarbeit und lesen Sie Claudes Antworten unter [claude.ai/code](https://claude.ai/code). Wenn die Sitzung stattdessen in der Warteschlange sitzt, siehe [Troubleshooting](/docs/de/self-hosted-environments-deploy#troubleshooting).
  </Step>
</Steps>

Der Runner beendet sich absichtlich, sobald seine aktiven Sitzungen beendet sind; siehe [Runner-Lebenszyklus](/docs/de/self-hosted-environments#runner-lifecycle). Für die Produktion stellen Sie ihn unter einem Orchestrator bereit, der ihn beim Beenden neu startet. Siehe [Bereitstellung in der Produktion](/docs/de/self-hosted-environments-deploy).

<h2 id="send-a-follow-up-message-to-a-running-session">
  Senden Sie eine Nachricht an eine laufende Sitzung
</h2>

Sobald eine Sitzung in Ihrer Umgebung läuft, senden Sie ihr eine Nachricht von der `claude` CLI auf einem beliebigen Computer, auf dem Sie sich mit `claude auth login` angemeldet haben; der Befehl muss nicht auf dem Computer ausgeführt werden, der die Sitzung gestartet hat. Der Befehl sendet eine Nachricht:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Für `<session-id>` übergeben Sie die bloße `session_...` oder `cse_...`-ID oder die claude.ai/code-URL der Sitzung. Ein erfolgreicher Versand gibt `Sent to cloud session.` mit der Sitzungs-ID und einem Ansicht-Link aus. Akzeptierte ID-Formulare, JSON-Ausgabe, die Konto- und Richtlinienanforderungen und die Fehlerreferenz befinden sich auf [Senden Sie Nachfolgen von der CLI](/docs/de/claude-code-on-the-web#send-follow-ups-from-the-cli), da der Befehl gleich gegen von Anthropic gehostete Sitzungen funktioniert.

<h2 id="what’s-next">
  Was kommt als Nächstes
</h2>

* [Bereitstellung in der Produktion](/docs/de/self-hosted-environments-deploy): Härtung der Bereitstellung, Kontrolle des Egress, Konfiguration von Git-Anmeldedaten und Ausführung der Fleet unter Kubernetes oder Compose
* [Passen Sie Sitzungen an](/docs/de/self-hosted-environments-configuration): Wrapper-Skripte, Lifecycle-Hooks, On-Demand-Runner, MCP-Server und Berechtigungen
* [Testen Sie End-to-End](/docs/de/self-hosted-environments-testing): ein CI-Smoke-Test, der eine Sitzung versendet und Claudes Antworten liest
