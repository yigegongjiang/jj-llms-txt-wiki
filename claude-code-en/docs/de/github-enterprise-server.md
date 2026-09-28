> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code mit GitHub Enterprise Server

> Verbinden Sie Claude Code mit Ihrer selbstgehosteten GitHub Enterprise Server-Instanz für Web-Sitzungen, Code-Review und Plugin-Marktplätze.

<Note>
  Die Unterstützung für GitHub Enterprise Server ist für Team- und Enterprise-Pläne verfügbar.
</Note>

Die Unterstützung für GitHub Enterprise Server (GHES) ermöglicht es Ihrer Organisation, Claude Code mit Repositories zu verwenden, die auf Ihrer selbstverwalteten GitHub-Instanz gehostet werden, anstatt auf github.com. Sobald ein Administrator Ihre GHES-Instanz verbindet, können Entwickler Web-Sitzungen ausführen und automatisierte Code-Reviews erhalten, ohne dass eine Konfiguration pro Repository erforderlich ist. Plugin-Marktplätze, die auf Ihrer Instanz gehostet werden, werden ebenfalls unterstützt; die Anforderungen an Anmeldedaten variieren je nach Oberfläche, wie in [Plugin-Marktplätze auf GHES](#plugin-marketplaces-on-ghes) beschrieben.

Für Repositories auf github.com siehe [Claude Code im Web](/docs/de/claude-code-on-the-web) und [Code Review](/docs/de/code-review). Um Claude in Ihrer eigenen CI-Infrastruktur auszuführen, siehe [GitHub Actions](/docs/de/github-actions).

<h2 id="what-works-with-github-enterprise-server">
  Was mit GitHub Enterprise Server funktioniert
</h2>

Die folgende Tabelle zeigt, welche Claude Code-Funktionen GHES unterstützen und welche Unterschiede zum Verhalten von github.com bestehen.

| Funktion           | GHES-Unterstützung  | Hinweise                                                                                                                                       |
| :----------------- | :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code im Web | ✅ Unterstützt       | Ein Owner verbindet die GHES-Instanz einmalig; Entwickler verwenden `claude --cloud` oder [claude.ai/code](https://claude.ai/code) wie gewohnt |
| Code Review        | ✅ Unterstützt       | Gleiche automatisierte PR-Reviews wie github.com                                                                                               |
| Claude Security    | ✅ Unterstützt       | Verfügbar in öffentlicher Beta für Enterprise-Pläne unter [claude.ai/security](https://claude.ai/security)                                     |
| Teleport-Sitzungen | ✅ Unterstützt       | Verschieben Sie Sitzungen zwischen Web und Terminal mit `--teleport`                                                                           |
| Plugin-Marktplätze | ✅ Unterstützt       | Anforderungen an Anmeldedaten unterscheiden sich je nach Oberfläche. Siehe [Plugin-Marktplätze auf GHES](#plugin-marketplaces-on-ghes)         |
| Beitragskennzahlen | ✅ Unterstützt       | Bereitgestellt über Webhooks zum [Analytics-Dashboard](/docs/de/analytics)                                                                          |
| GitHub Actions     | ✅ Unterstützt       | Erfordert manuelle Workflow-Einrichtung; `/install-github-app` ist nur für github.com                                                          |
| GitHub MCP-Server  | ❌ Nicht unterstützt | Der GitHub MCP-Server funktioniert nicht mit GHES-Instanzen                                                                                    |

<h2 id="admin-setup">
  Admin-Einrichtung
</h2>

Ein Administrator verbindet Ihre GHES-Instanz einmalig mit Claude Code. Danach können Entwickler in Ihrer Organisation GHES-Repositories ohne zusätzliche Konfiguration verwenden. Sie benötigen die Rolle „Administrator" oder „Primärer Administrator" in Ihrer Claude-Organisation und die Berechtigung, GitHub Apps auf Ihrer GHES-Instanz zu erstellen.

Die geführte Einrichtung generiert ein GitHub App-Manifest und leitet Sie zu Ihrer GHES-Instanz weiter, um die App in einem Klick zu erstellen. Wenn Ihre Umgebung den Umleitungsfluss blockiert, ist eine [alternative manuelle Einrichtung](#manual-setup) verfügbar.

<Steps>
  <Step title="Öffnen Sie die Claude Code-Admin-Einstellungen">
    Gehen Sie zu [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) und suchen Sie den Abschnitt GitHub Enterprise Server.
  </Step>

  <Step title="Starten Sie die geführte Einrichtung">
    Klicken Sie auf **Verbinden**. Geben Sie einen Anzeigenamen von bis zu 20 Zeichen für die Verbindung und Ihren GHES-Hostnamen ein, z. B. `github.example.com`. Wenn Ihre GHES-Instanz ein selbstsigniertes oder privates Zertifikat einer Zertifizierungsstelle verwendet, fügen Sie das CA-Zertifikat in das optionale Feld ein.
  </Step>

  <Step title="Erstellen Sie die GitHub App">
    Klicken Sie auf **Weiter zu GitHub Enterprise**. Ihr Browser wird zu Ihrer GHES-Instanz mit einem vorausgefüllten App-Manifest weitergeleitet. Überprüfen Sie die Konfiguration und klicken Sie auf **GitHub App erstellen**. GHES leitet Sie mit den automatisch gespeicherten App-Anmeldedaten zurück zu Claude.
  </Step>

  <Step title="Installieren Sie die App auf Ihren Repositories">
    Installieren Sie die App auf der GitHub App-Seite Ihrer GHES-Instanz auf den Repositories oder Organisationen, auf die Claude zugreifen soll. Sie können mit einer Teilmenge beginnen und später weitere hinzufügen.
  </Step>

  <Step title="Aktivieren Sie Funktionen">
    Kehren Sie zu [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) zurück und aktivieren Sie [Code Review](/docs/de/code-review#set-up-code-review), Claude Security und [Beitragskennzahlen](/docs/de/analytics#enable-contribution-metrics) für Ihre GHES-Repositories mit der gleichen Konfiguration wie github.com.
  </Step>
</Steps>

<h3 id="github-app-permissions">
  GitHub App-Berechtigungen
</h3>

Das Manifest konfiguriert die GitHub App mit den Berechtigungen und Webhook-Ereignissen unten, die zusammen Web-Sitzungen, Code Review, Claude Security, Plugin-Marktplätze und Beitragskennzahlen abdecken:

| Berechtigung         | Zugriff             | Verwendet für                                                                                                                                                                                                                |
| :------------------- | :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Contents             | Lesen und Schreiben | Klonen von Repositories und Pushen von Branches                                                                                                                                                                              |
| Pull requests        | Lesen und Schreiben | Erstellen von PRs und Posten von Review-Kommentaren                                                                                                                                                                          |
| Issues               | Lesen und Schreiben | Antworten auf Issue-Erwähnungen                                                                                                                                                                                              |
| Checks               | Lesen und Schreiben | Posten von Code Review-Check-Läufen                                                                                                                                                                                          |
| Actions              | Lesen               | Lesen des CI-Status für Auto-Fix                                                                                                                                                                                             |
| Commit statuses      | Lesen               | Lesen des CI-Status von Anbietern, die Commit-Status statt Check-Läufe melden                                                                                                                                                |
| Repository hooks     | Lesen und Schreiben | Erstellen eines Webhooks auf einem Plugin-Marktplatz-Repository, wenn **Automatisch synchronisieren** für einen Marktplatz in [Organisationseinstellungen > Plugins](https://claude.ai/admin-settings/plugins) aktiviert ist |
| Metadata             | Lesen               | Von GitHub für alle Apps erforderlich                                                                                                                                                                                        |
| Organization members | Lesen               | Abgleich der Claude GitHub App auf github.com, die dies verwendet, um die Organisationsrolle eines verbindenden Benutzers beim Verknüpfen einer Installation zu überprüfen                                                   |

Die App abonniert `pull_request`, `issue_comment`, `pull_request_review_comment`, `pull_request_review`, `check_run` und `status`-Ereignisse.

GitHub wendet ein Manifest nur an, wenn die App erstellt wird, daher behält eine App, die aus einer früheren Version des Manifests erstellt wurde, die Berechtigungen und Ereignisse, mit denen sie erstellt wurde. Wenn Ihrer App eine der oben genannten Berechtigungen oder Ereignisse fehlt, fügen Sie diese in den App-Einstellungen auf Ihrer GHES-Instanz hinzu. GitHub fordert dann einen Besitzer jeder Installation auf, die neuen Berechtigungen zu genehmigen, und die Installation behält ihre alten Berechtigungen, bis dies geschieht.

<h3 id="manual-setup">
  Manuelle Einrichtung
</h3>

Wenn der geführte Umleitungsfluss durch Ihre Netzwerkkonfiguration blockiert wird, klicken Sie auf **Manuell hinzufügen** anstelle von Verbinden. Erstellen Sie eine GitHub App auf Ihrer GHES-Instanz mit den [oben genannten Berechtigungen und Ereignissen](#github-app-permissions) und geben Sie dann die Verbindungsdetails in das Formular ein: einen Anzeigenamen, Ihren GHES-Hostnamen und optionalen Port sowie die App-ID, Client-ID, Client-Geheimnis, Webhook-Geheimnis und privaten Schlüssel der App. Das Formular akzeptiert auch ein optionales benutzerdefiniertes CA-Zertifikat und Read-Replica-Hostnamen.

Claude generiert die Webhook-URL der App, wenn Sie die Verbindung speichern. Nachdem Sie auf **Konfiguration hinzufügen** geklickt haben, öffnen Sie das Menü **Weitere Optionen** der Verbindung, wählen Sie **Webhook-URL kopieren** und fügen Sie die URL in die Webhook-Einstellungen der App auf Ihrer GHES-Instanz ein. Verwenden Sie das gleiche Webhook-Geheimnis, das Sie in das Formular eingegeben haben.

<h3 id="network-requirements">
  Netzwerkanforderungen
</h3>

Für von Anthropic gehostete Sitzungen muss Ihre GHES-Instanz von der Anthropic-Infrastruktur erreichbar sein, damit Claude Repositories klonen und Review-Kommentare posten kann. Wenn Ihre GHES-Instanz hinter einer Firewall liegt, fügen Sie die [ausgehenden IP-Adressen von Anthropic](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) zur Whitelist hinzu. Sitzungen in einer [selbst gehosteten Umgebung](/docs/de/self-hosted-environments-deploy#configure-git) klonen von innerhalb Ihres Netzwerks, es sei denn, der Runner entscheidet sich für den [Anthropic Git-Proxy](/docs/de/self-hosted-environments-deploy#use-the-anthropic-git-proxy), der von der Seite von Anthropic abruft und die gleiche Erreichbarkeit benötigt; der [SCM-Connector](/docs/de/self-hosted-environments-reference#scm-connector-flags) deckt die gehosteten Pre-Session-Flows ab, wie z. B. die Repository-Auswahl, für einen GHES-Host, der nur intern routbar ist.

<h2 id="developer-workflow">
  Entwickler-Workflow
</h2>

Sobald ein Inhaber die GHES-Instanz verbunden hat, ist keine Konfiguration auf der Entwicklerseite erforderlich. Claude Code erkennt Ihren GHES-Hostnamen automatisch aus dem Git-Remote in Ihrem Arbeitsverzeichnis.

Klonen Sie ein Repository von Ihrer GHES-Instanz wie gewohnt, und ersetzen Sie `github.example.com` und den Repository-Pfad durch Ihren GHES-Hostnamen und das Repository:

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

Starten Sie dann eine Web-Sitzung. Claude erkennt den GHES-Host aus Ihrem Git-Remote und leitet die Sitzung durch Ihre konfigurierte Organisationsinstanz:

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

Die Sitzung klont Ihr Repository von GHES und pusht Änderungen zurück zu einem Branch. Überwachen Sie den Fortschritt unter [claude.ai/code](https://claude.ai/code). Siehe [Claude Code im Web](/docs/de/claude-code-on-the-web) für den vollständigen Cloud-Sitzungs-Workflow einschließlich Diff-Review, Auto-Fix und Routinen.

<h3 id="teleport-sessions-to-your-terminal">
  Teleport-Sitzungen zu Ihrem Terminal
</h3>

Ziehen Sie eine Web-Sitzung mit `claude --teleport` in Ihr lokales Terminal. Teleport überprüft, ob Sie sich in einem Checkout des gleichen GHES-Repositories befinden, bevor der Branch abgerufen und die Sitzungshistorie geladen wird. Siehe [Teleport-Anforderungen](/docs/de/claude-code-on-the-web#teleport-requirements) für Details.

<h2 id="plugin-marketplaces-on-ghes">
  Plugin-Marktplätze auf GHES
</h2>

Hosten Sie Plugin-Marktplätze auf Ihrer GHES-Instanz, um interne Tools in Ihrer Organisation zu verteilen. Die Marktplatzstruktur ist identisch mit auf github.com gehosteten Marktplätzen, aber die Installation funktioniert unterschiedlich, je nachdem, wo Sie den Marktplatz hinzufügen, und die Anmeldedaten unterscheiden sich zwischen den Oberflächen:

| Oberfläche                                          | Funktionsweise der Installation                                                                                                                                                                                                                                      | Was jeder Benutzer benötigt                                                                                                                                                                                                                                             |
| :-------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code CLI und Desktop                         | Claude Code klont das Marktplatz-Repository mit den vorhandenen Git-Anmeldedaten des Computers                                                                                                                                                                       | Git-Zugriff auf Ihren GHES-Host von ihrem Computer                                                                                                                                                                                                                      |
| Verwaltete Einstellungen (`extraKnownMarketplaces`) | Claude Code registriert den Eintrag und klont das Repository mit den vorhandenen Git-Anmeldedaten des Computers                                                                                                                                                      | Git-Zugriff auf Ihren GHES-Host von ihrem Computer                                                                                                                                                                                                                      |
| claude.ai-Organisationseinstellungen für Plugins    | Ein Eigentümer wählt die GHES-Instanz als Quelle aus; Anthropics Backend ruft das Repository ab und synchronisiert es mit der GitHub App aus [Admin-Setup](#admin-setup)                                                                                             | Nichts pro Benutzer nach dem Hinzufügen. Der Eigentümer, der es hinzufügt, benötigt sein eigenes GitHub Enterprise-Konto als Zugriffsprüfung, und die GitHub App muss im Marktplatz-Repository installiert sein                                                         |
| claude.ai-Benutzereinstellungen                     | Anthropics Backend ruft das Repository mit der GitHub Enterprise-Verbindung des einreichenden Benutzers ab                                                                                                                                                           | Sein eigenes GitHub Enterprise-Konto, das mit Claude verbunden ist                                                                                                                                                                                                      |
| Claude Code im Web                                  | Cloud-Sitzungen klonen Marktplätze innerhalb der Sitzungs-Sandbox. Die Sandbox kann Ihre GHES-Instanz nur erreichen, wenn sich das Repository der Sitzung auf derselben Instanz befindet, und ihre Git-Anmeldedaten sind auf die Repositories der Sitzung beschränkt | Nicht zuverlässig für GHES-gehostete Marktplätze: Ein anderer Host als das Repository der Sitzung ist nicht erreichbar, und selbst Installationen auf derselben Instanz können fehlschlagen. Verwenden Sie stattdessen die CLI, verwaltete Einstellungen oder claude.ai |

<Warning>
  GitHub Enterprise-Verbindungen auf claude.ai sind pro Benutzer, wenn ein Marktplatz aus den Benutzereinstellungen hinzugefügt wird. Das [Admin-Setup](#admin-setup) verbindet Ihre GHES-Instanz mit Ihrer Organisation, verbindet aber keine einzelnen Benutzerkonten: Jeder Benutzer, der einen GHES-Marktplatz aus seinen eigenen Einstellungen hinzufügt, muss zunächst sein eigenes GitHub Enterprise-Konto verbinden, und die Verbindung eines Benutzers, einschließlich des Eigentümers, deckt niemand anderen ab. Marktplätze, die von einem Eigentümer in den Organisationseinstellungen für Plugins hinzugefügt werden, stellen diese Anforderung nicht an Benutzer, da laufende Abrufe die GitHub App der Organisation verwenden. Der Eigentümer, der den Marktplatz hinzufügt, benötigt zum Zeitpunkt des Hinzufügens immer noch sein eigenes GitHub Enterprise-Konto.
</Warning>

<h3 id="add-a-ghes-marketplace">
  Fügen Sie einen GHES-Marktplatz hinzu
</h3>

Die `owner/repo`-Kurzform wird immer zu github.com aufgelöst. Für GHES-gehostete Marktplätze verwenden Sie die vollständige Git-URL und ersetzen Sie `github.example.com` und den Repository-Pfad durch Ihre eigenen. HTTPS-URLs werden empfohlen:

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

SSH-URLs funktionieren, wenn der Computer Ihren GHES-Host bereits vertraut:

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code führt Git nicht interaktiv aus und lehnt SSH-Verbindungen zu Hosts ab, die sich nicht in der `known_hosts`-Datei des Computers befinden. Eine HTTPS-URL mit einem Git-Credential-Helper vermeidet die `known_hosts`-Anforderung.

Siehe [Erstellen und Verteilen eines Plugin-Marktplatzes](/docs/de/plugin-marketplaces) für die vollständige Anleitung zum Erstellen von Marktplätzen.

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  Registrieren Sie GHES-Marktplätze vorab mit verwalteten Einstellungen
</h3>

Die `extraKnownMarketplaces`-Einstellung registriert einen Marktplatz vorab, damit Entwickler ihn ohne manuelle Einrichtung erhalten. Sie funktioniert aus [jeder Einstellungsdatei](/docs/de/settings-reference#extraknownmarketplaces), einschließlich der `.claude/settings.json` eines Repositories; verwaltete Einstellungen liefern sie organisationsweit:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code installiert diese Marktplätze lokal: Es registriert jeden Eintrag und klont das Repository mit den vorhandenen Git-Anmeldedaten des Computers. Dieser Pfad verläuft nicht über claude.ai, daher ist die GitHub Enterprise-Verbindung pro Benutzer nicht erforderlich. Für einen erfolgreichen Rollout:

* **Verwenden Sie eine vollständige Git-URL.** Die `owner/repo`-Kurzform wird immer zu github.com aufgelöst und kann nicht auf einen GHES-Host verweisen.
* **Bevorzugen Sie HTTPS-URLs.** SSH-Klone schlagen auf Computern fehl, die Ihren GHES-Host-Schlüssel nicht bereits vertrauen. Eine HTTPS-URL mit dem Standard-Git-Credential-Helper Ihrer Organisation funktioniert auf jedem Computer mit konfigurierten Anmeldedaten.
* **Bestätigen Sie, dass jeder Computer von Ihrem GHES-Host klonen kann.** Wenn ein Computer keine Anmeldedaten hat, wird der Marktplatz registriert, aber nie installiert, und seine Plugins werden als nicht gefunden gemeldet, anstatt nach Anmeldedaten zu fragen.
* **Bestätigen Sie, dass die Einstellung jeden Computer erreicht.** Eine verwaltete Einstellungsdatei wird nur auf Computern wirksam, auf denen sie bereitgestellt wird, beispielsweise über Ihr Geräteverwaltungssystem. Siehe [Verwaltete Einstellungen bereitstellen](/docs/de/managed-settings#delivery-mechanisms) für Dateispeicherorte.

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  Whitelist GHES-Marktplätze in verwalteten Einstellungen
</h3>

Wenn Ihre Organisation [verwaltete Einstellungen](/docs/de/settings) verwendet, um einzuschränken, welche Marktplätze Entwickler hinzufügen können, verwenden Sie den `hostPattern`-Quellentyp, um alle Marktplätze von Ihrer GHES-Instanz zuzulassen, ohne jedes Repository aufzuzählen. Siehe [Bereitstellungsmechanismen](/docs/de/managed-settings#delivery-mechanisms) für Dateispeicherorte auf jeder Plattform. Fügen Sie das JSON zu Ihrer `managed-settings.json`-Datei oder einer entsprechenden MDM-Richtlinie hinzu:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Siehe die Referenz zu den Einstellungen [strictKnownMarketplaces](/docs/de/settings-reference#strictknownmarketplaces) und [extraKnownMarketplaces](/docs/de/settings-reference#extraknownmarketplaces) für das vollständige Schema.

<h2 id="limitations">
  Einschränkungen
</h2>

Einige Funktionen verhalten sich auf GHES anders als auf github.com. Die [Funktionstabelle](#what-works-with-github-enterprise-server) fasst die Unterstützung zusammen; dieser Abschnitt behandelt die Workarounds.

* **`/install-github-app`-Befehl**: Folgen Sie stattdessen dem [Admin-Einrichtungs](#admin-setup)-Fluss auf claude.ai. Wenn Sie auch GitHub Actions-Workflows auf GHES möchten, passen Sie den [Beispiel-Workflow](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) manuell an.
* **GitHub MCP-Server**: Verwenden Sie stattdessen die `gh` CLI, die für Ihren GHES-Host konfiguriert ist. Führen Sie `gh auth login --hostname github.example.com` aus, um sich zu authentifizieren, dann kann Claude `gh`-Befehle in Sitzungen verwenden.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="web-session-fails-to-clone-repository">
  Web-Sitzung kann Repository nicht klonen
</h3>

Wenn `claude --cloud` mit einem Klonfehler fehlschlägt, überprüfen Sie, ob ein Owner die Einrichtung für Ihre GHES-Instanz abgeschlossen hat und ob die GitHub App auf dem Repository installiert ist, an dem Sie arbeiten. Bitten Sie den Owner, der die Instanz verbunden hat, zu bestätigen, dass der in den Claude-Einstellungen registrierte Hostname mit dem Hostnamen in Ihrem Git-Remote übereinstimmt.

<h3 id="marketplace-add-fails-with-a-policy-error">
  Marktplatz-Hinzufügen schlägt mit Richtlinienfehler fehl
</h3>

Wenn `/plugin marketplace add` für Ihre GHES-URL blockiert wird, hat Ihre Organisation Marktplatzquellen eingeschränkt. Bitten Sie Ihren Administrator, einen `hostPattern`-Eintrag für Ihren GHES-Hostnamen in [verwalteten Einstellungen](#allowlist-ghes-marketplaces-in-managed-settings) hinzuzufügen.

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  Marktplatz-Hinzufügen auf claude.ai schlägt mit GitHub-Zugriffsfehler fehl
</h3>

Wenn das Hinzufügen eines GHES-Marktplatzes aus Ihren Benutzereinstellungen mit einem generischen Fehler wie „Marktplatz konnte nicht hinzugefügt werden" fehlschlägt, überprüfen Sie zunächst Ihre GitHub Enterprise-Verbindung. Dies ist das, was angezeigt wird, wenn Ihr eigenes GitHub Enterprise-Konto nicht mit Claude verbunden ist, auch wenn Ihre GHES-Instanz der Organisation konfiguriert ist und andere Benutzer verbunden sind. Der Dialog verweist nicht auf den GitHub Enterprise-Verbindungsfluss, und die Option „Mit GitHub verbinden" auf der Registerkarte „Durchsuchen" meldet sich bei github.com an, was keinen Zugriff auf GHES-Repositories gewährt.

Um Ihr GitHub Enterprise-Konto zu verbinden: Die Repository-Auswahl auf [claude.ai/code](https://claude.ai/code) bietet eine Verbindungsoption für jede konfigurierte GHES-Instanz, und Owners können sich auch aus dem GitHub Enterprise-Bereich der [Claude Code-Administratoreinstellungen](https://claude.ai/admin-settings/claude-code) verbinden. Fügen Sie dann den Marktplatz erneut hinzu. Alternativ können Sie einen Owner bitten, den Marktplatz in den Organisationsplug-in-Einstellungen hinzuzufügen, was die Anforderung der Benutzerverbindung pro Benutzer entfernt.

Auf anderen claude.ai-Oberflächen deutet ein Fehler „Repository nicht gefunden. Falls es privat ist, ist GitHub-Zugriff erforderlich" auf einem GHES-Marktplatz normalerweise auf dieselbe fehlende Verbindung hin. Verbinden Sie Ihr GitHub Enterprise-Konto über einen der oben genannten Pfade und versuchen Sie es dann erneut.

<h3 id="ghes-instance-not-reachable">
  GHES-Instanz nicht erreichbar
</h3>

Wenn Reviews oder von Anthropic gehostete Web-Sitzungen zeitüberschritten werden, ist Ihre GHES-Instanz möglicherweise nicht von der Anthropic-Infrastruktur erreichbar. Bestätigen Sie, dass Ihre Firewall eingehende Verbindungen von Anthropics [ausgehenden IP-Adressen](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) zulässt. Sitzungen in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) erreichen GHES von innerhalb Ihres Netzwerks, überprüfen Sie daher für diese den eigenen Netzwerkpfad des Runners und den [SCM-Connector](/docs/de/self-hosted-environments-reference#scm-connector-flags) stattdessen.

<h3 id="session-start-fails-with-unable-to-get-organization-uuid">
  Sitzungsstart schlägt mit `Unable to get organization UUID` fehl
</h3>

Web-Sitzungen erfordern eine Team- oder Enterprise-Organisation. Melden Sie sich mit `/login` mit Ihrem Organisationskonto an. Wenn Sie sich stattdessen mit einem API-Schlüssel authentifizieren, schlagen Web-Sitzungen früher fehl mit einer Nachricht, die Sie auffordert, `/login` auszuführen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Diese Seiten behandeln die in diesem Leitfaden referenzierten Funktionen ausführlicher:

* [Claude Code im Web](/docs/de/claude-code-on-the-web): Führen Sie Claude Code-Sitzungen auf Cloud-Infrastruktur aus
* [Code Review](/docs/de/code-review): Automatisierte PR-Reviews
* [Plugin-Marktplätze](/docs/de/plugin-marketplaces): Erstellen und Verteilen von Plugin-Katalogen
* [Analytics](/docs/de/analytics): Verfolgen Sie Nutzung und Beitragskennzahlen
* [Verwaltete Einstellungen](/docs/de/settings): Organisationsweite Richtlinienkonfiguration
* [Netzwerkkonfiguration](/docs/de/network-config): Firewall- und IP-Whitelist-Anforderungen
