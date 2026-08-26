> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code im Web verwenden

> Konfigurieren Sie Cloud-Umgebungen, Setup-Skripte, Netzwerkzugriff und Docker in Anthropics Sandbox. Verschieben Sie Sitzungen zwischen Web und Terminal mit `--cloud` und `--teleport`.

<Note>
  Claude Code im Web befindet sich in der Forschungsvorschau für Pro-, Max- und Team-Benutzer sowie für Enterprise-Benutzer mit Premium-Sitzen oder Chat + Claude Code-Sitzen.
</Note>

Claude Code im Web führt Aufgaben auf von Anthropic verwalteter Cloud-Infrastruktur unter [claude.ai/code](https://claude.ai/code) aus. Sitzungen bleiben bestehen, auch wenn Sie Ihren Browser schließen, und Sie können sie über die Claude Mobile-App überwachen.

<Tip>
  Neu bei Claude Code im Web? Beginnen Sie mit [Erste Schritte](/docs/de/web-quickstart), um Ihr GitHub-Konto zu verbinden und Ihre erste Aufgabe einzureichen.
</Tip>

Diese Seite behandelt:

* [GitHub-Authentifizierungsoptionen](#github-authentication-options): zwei Möglichkeiten, GitHub zu verbinden
* [Die Cloud-Umgebung](#the-cloud-environment): welche Konfiguration übertragen wird, welche Tools installiert sind und wie Umgebungen konfiguriert werden
* [Setup-Skripte](#setup-scripts) und Abhängigkeitsverwaltung
* [Netzwerkzugriff](#network-access): Ebenen, Proxys und die Standard-Allowlist
* [Aufgaben zwischen Web und Terminal verschieben](#move-tasks-between-web-and-terminal) mit `--cloud` und `--teleport`
* [Mit Sitzungen arbeiten](#work-with-sessions): Überprüfung, Freigabe, Archivierung, Löschung
* [Auto-fix Pull Requests](#auto-fix-pull-requests): automatische Reaktion auf CI-Fehler und Review-Kommentare
* [Sicherheit und Isolation](#security-and-isolation): wie Sitzungen isoliert sind
* [Einschränkungen](#limitations): Ratenlimits und Plattformbeschränkungen

<h2 id="github-authentication-options">
  GitHub-Authentifizierungsoptionen
</h2>

Cloud-Sitzungen benötigen Zugriff auf Ihre GitHub-Repositories, um Code zu klonen und Branches zu pushen. Sie können Zugriff auf zwei Arten gewähren:

| Methode          | Funktionsweise                                                                                                          | Am besten für                                                              |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| **GitHub App**   | Autorisieren Sie die Claude GitHub App während des [Web-Onboardings](/docs/de/web-quickstart).                               | Browser-Onboarding; Teams, die [Auto-fix](#auto-fix-pull-requests) möchten |
| **`/web-setup`** | Führen Sie `/web-setup` in Ihrem Terminal aus, um Ihr lokales `gh` CLI-Token mit Ihrem Claude-Konto zu synchronisieren. | Einzelne Entwickler, die bereits `gh` verwenden                            |

<Note>
  Bei beiden Methoden kann eine Cloud-Sitzung auf jedes Repository zugreifen, das das verbundene GitHub-Konto sehen kann, nicht nur auf die Repositories, auf denen die Claude GitHub App installiert ist. Die App-Installation ermöglicht PR-Webhooks für [Auto-fix](#auto-fix-pull-requests); es ist keine Zugriffskontrolle auf Sitzungsebene. Um einzuschränken, welche Repositories Ihr Team von Cloud-Sitzungen aus erreichen kann, beschränken Sie den Zugriff auf GitHub selbst, beispielsweise durch Einschränkung der Team- oder Repository-Mitgliedschaft für die verbundenen GitHub-Konten.
</Note>

Beide Methoden funktionieren. [`/schedule`](/docs/de/routines) überprüft auf beide Formen des Zugriffs und fordert Sie auf, `/web-setup` auszuführen, wenn keines konfiguriert ist. Siehe [Vom Terminal verbinden](/docs/de/web-quickstart#connect-from-your-terminal) für die `/web-setup`-Anleitung.

Die GitHub App ist erforderlich für [Auto-fix](#auto-fix-pull-requests), das die App verwendet, um PR-Webhooks zu empfangen. Wenn Sie sich mit `/web-setup` verbinden und später Auto-fix möchten, installieren Sie die App auf diesen Repositories.

Team- und Enterprise-Administratoren können `/web-setup` mit dem Quick web setup-Umschalter unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) deaktivieren.

<Note>
  Organisationen mit aktivierter [Zero Data Retention](/docs/de/zero-data-retention) können `/web-setup` oder andere Cloud-Sitzungsfunktionen nicht verwenden.
</Note>

<h2 id="the-cloud-environment">
  Die Cloud-Umgebung
</h2>

Jede Sitzung wird in einer frischen, von Anthropic verwalteten VM mit Ihrem geklonten Repository ausgeführt. Dieser Abschnitt behandelt, was verfügbar ist, wenn eine Sitzung startet, und wie Sie sie anpassen können.

<h3 id="what’s-available-in-cloud-sessions">
  Was in Cloud-Sitzungen verfügbar ist
</h3>

Cloud-Sitzungen starten von einem frischen Klon Ihres Repositories. Alles, was zum Repo committed ist, ist verfügbar. Alles, was Sie nur auf Ihrem eigenen Computer installiert oder konfiguriert haben, ist nicht verfügbar. Die Richtlinie Ihrer Organisation kommt separat über [server-verwaltete Einstellungen](/docs/de/server-managed-settings).

|                                                                                    | Verfügbar in Cloud-Sitzungen | Warum                                                                                                                                                                                                                                                                                                                                                                                         |
| :--------------------------------------------------------------------------------- | :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ihr Repo's `CLAUDE.md`                                                             | Ja                           | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                |
| Ihr Repo's `.claude/settings.json` Hooks                                           | Ja                           | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                |
| Ihr Repo's `.mcp.json` MCP-Server                                                  | Ja                           | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                |
| Ihr Repo's `.claude/rules/`                                                        | Ja                           | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                |
| Ihr Repo's `.claude/skills/`, `.claude/agents/`, `.claude/commands/`               | Ja                           | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                |
| In `.claude/settings.json` deklarierte Plugins                                     | Ja                           | Installiert beim Sitzungsstart vom [Marketplace](/docs/de/plugin-marketplaces), den Sie deklariert haben. Erfordert Netzwerkzugriff, um die Marketplace-Quelle zu erreichen                                                                                                                                                                                                                        |
| Ihre Organisationen [server-verwaltete Einstellungen](/docs/de/server-managed-settings) | Ja                           | Werden von Anthropic's Servern abgerufen, wenn die Sitzung startet. Siehe [Oberflächenabdeckung](/docs/de/model-config#surface-coverage) für die Durchsetzung von `availableModels` in Cloud-Sitzungen. Einstellungen, die auf Ihrem Gerät über MDM oder verwaltete Einstellungsdateien bereitgestellt werden, gelten nicht, da die Sitzung auf einer von Anthropic verwalteten VM ausgeführt wird |
| Ihr Benutzer `~/.claude/CLAUDE.md`                                                 | Nein                         | Lebt auf Ihrem Computer, nicht im Repo                                                                                                                                                                                                                                                                                                                                                        |
| Ihr Benutzer `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/`       | Nein                         | Leben auf Ihrem Computer, nicht im Repo. Committen Sie sie stattdessen ins Verzeichnis `.claude/` des Repos. Skills, die Sie auf claude.ai aktivieren, werden automatisch in Cloud-Sitzungen geladen                                                                                                                                                                                          |
| Plugins, die nur in Ihren Benutzereinstellungen aktiviert sind                     | Nein                         | Benutzer-scoped `enabledPlugins` lebt in `~/.claude/settings.json`. Deklarieren Sie sie stattdessen in der `.claude/settings.json` des Repos                                                                                                                                                                                                                                                  |
| MCP-Server, die Sie mit `claude mcp add` hinzugefügt haben                         | Nein                         | Diese schreiben in Ihre lokale Benutzerkonfiguration, nicht ins Repo. Deklarieren Sie den Server stattdessen in [`.mcp.json`](/docs/de/mcp#project-scope)                                                                                                                                                                                                                                          |
| Statische API-Token und Anmeldedaten                                               | Nein                         | Es existiert noch kein dedizierter Secrets-Store. Siehe unten                                                                                                                                                                                                                                                                                                                                 |
| Interaktive Authentifizierung wie AWS SSO                                          | Nein                         | Nicht unterstützt. SSO erfordert browserbasierte Anmeldung, die nicht in einer Cloud-Sitzung ausgeführt werden kann                                                                                                                                                                                                                                                                           |

Um Ihre eigene Konfiguration in Cloud-Sitzungen verfügbar zu machen, committen Sie sie ins Repo; die Organisationsrichtlinie kommt separat über [server-verwaltete Einstellungen](/docs/de/server-managed-settings).

Ein dedizierter Secrets-Store ist noch nicht verfügbar. Sowohl Umgebungsvariablen als auch Setup-Skripte werden in der Umgebungskonfiguration gespeichert, sichtbar für jeden, der diese Umgebung bearbeiten kann. Wenn Sie Secrets in einer Cloud-Sitzung benötigen, fügen Sie sie als Umgebungsvariablen mit dieser Sichtbarkeit im Hinterkopf hinzu.

<h3 id="installed-tools">
  Installierte Tools
</h3>

Cloud-Sitzungen werden mit gängigen Sprachlaufzeiten, Build-Tools und Datenbanken vorinstalliert geliefert. Die folgende Tabelle fasst zusammen, was nach Kategorie enthalten ist.

| Kategorie       | Enthalten                                                                         |
| :-------------- | :-------------------------------------------------------------------------------- |
| **Python**      | Python 3.x mit pip, poetry, uv, black, mypy, pytest, ruff                         |
| **Node.js**     | 20, 21 und 22 über nvm, mit npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**        | 3.1, 3.2, 3.3 mit gem, bundler, rbenv                                             |
| **PHP**         | 8.4 mit Composer                                                                  |
| **Java**        | OpenJDK 21 mit Maven und Gradle                                                   |
| **Go**          | neueste stabile Version mit Modulunterstützung                                    |
| **Rust**        | rustc und cargo                                                                   |
| **C/C++**       | GCC, Clang, cmake, ninja, conan                                                   |
| **Docker**      | docker, dockerd, docker compose                                                   |
| **Datenbanken** | PostgreSQL 16, Redis 7.0                                                          |
| **Utilities**   | git, jq, yq, ripgrep, tmux, vim, nano                                             |

¹ Bun ist installiert, hat aber bekannte [Proxy-Kompatibilitätsprobleme](#install-dependencies-with-a-sessionstart-hook) beim Paketabruf.

Für genaue Versionen bitten Sie Claude, `check-tools` in einer Cloud-Sitzung auszuführen. Dieser Befehl existiert nur in Cloud-Sitzungen.

<h3 id="work-with-github-issues-and-pull-requests">
  Mit GitHub-Issues und Pull Requests arbeiten
</h3>

Cloud-Sitzungen enthalten integrierte GitHub-Tools, mit denen Claude Issues lesen, Pull Requests auflisten, Diffs abrufen und Kommentare posten kann, ohne Setup. Diese Tools authentifizieren sich über den [GitHub-Proxy](#github-proxy) mit der Methode, die Sie unter [GitHub-Authentifizierungsoptionen](#github-authentication-options) konfiguriert haben, sodass Ihr Token niemals in den Container gelangt.

Sie können `GH_TOKEN` oder `GITHUB_TOKEN` selbst in [Umgebungseinstellungen](#configure-your-environment) setzen, oder beide ungesetzt lassen und den [GitHub-Proxy](#github-proxy) für Sie authentifizieren lassen:

* Wenn Sie ein Token setzen, wird es unverändert an den Container weitergeleitet, sodass `gh` und Ihre Skripte es direkt verwenden.
* Wenn Sie keines setzen, setzt der Container beide Variablen auf die Platzhalterzeichenkette `proxy-injected` und der Proxy ersetzt Ihre echten Anmeldedaten bei ausgehenden GitHub-Anfragen. `gh` funktioniert ohne ein eigenes Token, aber ein Skript, das `GITHUB_TOKEN` direkt liest, erhält den Platzhalter, nicht ein verwendbares Token.

Um zu überprüfen, welcher Fall für Ihre Sitzung zutrifft, bitten Sie Claude, `echo $GH_TOKEN` auszuführen.

Die `gh` CLI ist nicht vorinstalliert. Wenn Sie einen `gh`-Befehl benötigen, den die integrierten Tools nicht abdecken, wie `gh release` oder `gh workflow run`, installieren und authentifizieren Sie ihn selbst:

<Steps>
  <Step title="Installieren Sie gh in Ihrem Setup-Skript">
    Fügen Sie `apt update && apt install -y gh` zu Ihrem [Setup-Skript](#setup-scripts) hinzu.
  </Step>

  <Step title="Stellen Sie ein Token bereit, wenn der Proxy die Authentifizierung nicht übernimmt">
    Wenn `echo $GH_TOKEN` `proxy-injected` ausgibt, authentifiziert der [GitHub-Proxy](#github-proxy) `gh` für Sie und dieser Schritt ist nicht erforderlich. Andernfalls fügen Sie eine `GH_TOKEN`-Umgebungsvariable zu Ihren [Umgebungseinstellungen](#configure-your-environment) mit einem GitHub Personal Access Token hinzu. `gh` liest `GH_TOKEN` automatisch, daher ist kein `gh auth login`-Schritt erforderlich.
  </Step>
</Steps>

<h3 id="link-output-back-to-the-session">
  Verknüpfen Sie Ausgabe zurück zur Sitzung
</h3>

Jede Cloud-Sitzung hat eine Transkript-URL auf claude.ai, und die Sitzung kann ihre eigene ID aus der Umgebungsvariablen `CLAUDE_CODE_REMOTE_SESSION_ID` lesen. Verwenden Sie dies, um einen nachverfolgbaren Link in PR-Bodies, Commit-Nachrichten, Slack-Posts oder generierten Berichten zu platzieren, damit ein Reviewer den Lauf öffnen kann, der sie produziert hat.

Ab v2.1.179 enthalten Commits, die Claude in einer Web-Sitzung erstellt, einen `Claude-Session: <url>` Git-Trailer, und PR-Bodies enthalten die Sitzungs-URL auf einer eigenen Zeile. Ab v2.1.182 können Sie [`attribution.sessionUrl`](/docs/de/settings#attribution-settings) auf `false` setzen, um den Trailer und den PR-Body-Link wegzulassen.

Um den Sitzungs-Link in etwas anderem als einem Commit oder PR einzufügen, wie z. B. eine Slack-Nachricht, die Claude postet, oder eine Berichtsdatei, die es schreibt, lassen Sie Claude den folgenden Befehl ausführen und verwenden Sie seine Ausgabe. Der Befehl konvertiert das `cse_`-Präfix im Wert der Umgebungsvariablen in das `session_`-Präfix, das die Transkript-URL erwartet:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Tests ausführen, Services starten und Pakete hinzufügen
</h3>

Claude führt Tests als Teil der Arbeit an einer Aufgabe aus. Bitten Sie darum in Ihrem Prompt, wie „fix the failing tests in `tests/`" oder „run pytest after each change." Test-Runner wie pytest, jest und cargo test funktionieren sofort, da sie vorinstalliert sind.

PostgreSQL und Redis sind vorinstalliert, aber nicht standardmäßig ausgeführt. Bitten Sie Claude, jeden während der Sitzung zu starten:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker ist für die Ausführung containerisierter Services verfügbar. Bitten Sie Claude, `docker compose up` auszuführen, um die Services Ihres Projekts zu starten. Der Netzwerkzugriff zum Abrufen von Images folgt der [Zugriffsstufe](#access-levels) Ihrer Umgebung, und die [Vertrauenswürdigen Standards](#default-allowed-domains) enthalten Docker Hub und andere gängige Registries.

Wenn Ihre Images groß oder langsam zum Abrufen sind, fügen Sie `docker compose pull` oder `docker compose build` zu Ihrem [Setup-Skript](#setup-scripts) hinzu. Die abgerufenen Images werden in der [gecachten Umgebung](#environment-caching) gespeichert, daher hat jede neue Sitzung sie auf der Festplatte. Der Cache speichert nur Dateien, keine laufenden Prozesse, daher startet Claude die Container immer noch jede Sitzung.

Um Pakete hinzuzufügen, die nicht vorinstalliert sind, verwenden Sie ein [Setup-Skript](#setup-scripts). Die Ausgabe des Skripts wird [gecacht](#environment-caching), daher sind Pakete, die Sie dort installieren, am Anfang jeder Sitzung verfügbar, ohne jedes Mal neu installiert zu werden. Sie können Claude auch bitten, Pakete während der Sitzung zu installieren, aber diese Installationen bleiben nicht über Sitzungen hinweg bestehen.

<h3 id="resource-limits">
  Ressourcenlimits
</h3>

Cloud-Sitzungen werden mit ungefähren Ressourcengrenzen ausgeführt, die sich im Laufe der Zeit ändern können:

* 4 vCPUs
* 16 GB RAM
* 30 GB Festplatte

Aufgaben, die erheblich mehr Speicher erfordern, wie große Build-Jobs oder speicherintensive Tests, können fehlschlagen oder beendet werden. Für Workloads jenseits dieser Limits verwenden Sie [Remote Control](/docs/de/remote-control), um Claude Code auf Ihrer eigenen Hardware auszuführen.

<h3 id="configure-your-environment">
  Konfigurieren Sie Ihre Umgebung
</h3>

Umgebungen steuern [Netzwerkzugriff](#network-access), Umgebungsvariablen und das [Setup-Skript](#setup-scripts), das vor einer Sitzung ausgeführt wird. Siehe [Installierte Tools](#installed-tools) für das, was ohne Konfiguration verfügbar ist. Sie können Umgebungen über die Web-Oberfläche oder das Terminal verwalten:

| Aktion                                     | Wie                                                                                                                                                                                                                                                                   |
| :----------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Umgebung hinzufügen                        | Wählen Sie die aktuelle Umgebung, um die Auswahl zu öffnen, dann wählen Sie **Umgebung hinzufügen**. Der Dialog enthält Name, Netzwerkzugriffsstufe, Umgebungsvariablen und Setup-Skript.                                                                             |
| Umgebung bearbeiten                        | Wählen Sie das Cloud-Symbol mit dem Namen der aktuellen Umgebung, um die Auswahl zu öffnen, bewegen Sie den Mauszeiger über eine Umgebung und klicken Sie auf das Einstellungssymbol, das auf der rechten Seite erscheint.                                            |
| Umgebung archivieren                       | Öffnen Sie die Umgebung zum Bearbeiten und wählen Sie **Archivieren**. Archivierte Umgebungen sind in der Auswahl ausgeblendet, aber vorhandene Sitzungen werden weiterhin ausgeführt.                                                                                |
| Standard für CLI-Cloud-Sitzungen festlegen | Führen Sie `/remote-env` in Ihrem Terminal aus. Wenn Sie eine einzelne Umgebung haben, zeigt dieser Befehl Ihre aktuelle Konfiguration. `/remote-env` wählt nur den Standard; fügen Sie Umgebungen über die Web-Oberfläche hinzu, bearbeiten und archivieren Sie sie. |

Umgebungsvariablen verwenden das `.env`-Format mit einem `KEY=value`-Paar pro Zeile. Wickeln Sie Werte nicht in Anführungszeichen ein, da Anführungszeichen als Teil des Werts gespeichert werden. Dieses Beispiel definiert drei Variablen:

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

<h3 id="organization-shared-environments">
  Von der Organisation gemeinsam genutzte Umgebungen
</h3>

Besitzer und Administratoren in Team- und Enterprise-Plänen können Cloud-Umgebungen erstellen, die mit jedem Mitglied der Organisation gemeinsam genutzt werden. Gemeinsam genutzte Umgebungen erscheinen in der Umgebungsauswahl jedes Mitglieds neben seinen persönlichen, sodass ein Team sich auf eine Konfiguration einigen kann, anstatt dass jedes Mitglied sie neu erstellt.

Verwalten Sie gemeinsam genutzte Umgebungen von der Seite **Cloud-Umgebungen** in [Admin-Einstellungen](https://claude.ai/admin-settings). Von dort aus können Sie:

* Gemeinsam genutzte Umgebungen erstellen, bearbeiten und archivieren. Jede hat die gleichen Felder wie eine persönliche Umgebung: einen Namen, eine [Netzwerkzugriffsstufe](#access-levels), [Umgebungsvariablen](#configure-your-environment) im `.env`-Format und ein [Setup-Skript](#setup-scripts).
* Die Standard-Umgebung für die Organisation festlegen.

Werte in einer gemeinsam genutzten Umgebung erreichen die Sitzungen jedes Mitglieds in dieser Umgebung. Wie persönliche Umgebungen haben gemeinsam genutzte Umgebungen keinen dedizierten Secrets-Store, daher sollten Sie keine Secrets einbeziehen.

<h2 id="setup-scripts">
  Setup-Skripte
</h2>

Ein Setup-Skript ist ein Bash-Skript, das ausgeführt wird, wenn eine neue Cloud-Sitzung startet, bevor Claude Code startet. Verwenden Sie Setup-Skripte, um Abhängigkeiten zu installieren, Tools zu konfigurieren oder alles zu holen, das die Sitzung benötigt und nicht vorinstalliert ist.

Skripte werden als Root auf Ubuntu 24.04 ausgeführt, daher funktionieren `apt install` und die meisten Sprachpaketmanager.

Um ein Setup-Skript hinzuzufügen, öffnen Sie den Dialog Umgebungseinstellungen und geben Sie Ihr Skript in das Feld **Setup-Skript** ein.

Dieses Beispiel installiert die `gh` CLI, die nicht vorinstalliert ist:

```bash theme={null}
#!/bin/bash
apt update && apt install -y gh
```

Wenn das Skript mit einem Nicht-Null-Wert beendet wird, schlägt die Sitzung fehl zu starten. Fügen Sie `|| true` an nicht kritische Befehle an, um zu vermeiden, dass die Sitzung bei einem fehlerhaften Install blockiert wird.

Halten Sie die Gesamtlaufzeit des Skripts unter ungefähr fünf Minuten, damit der [Umgebungs-Cache](#environment-caching) erstellt werden kann. Führen Sie unabhängige Installationen parallel mit `&` und `wait` aus. Wenn ein einzelner Download nicht in das Fünf-Minuten-Limit passt, verschieben Sie ihn zu einem [SessionStart-Hook](#setup-scripts-vs-sessionstart-hooks), der ihn im Hintergrund startet.

<Note>
  Setup-Skripte, die Pakete installieren, benötigen Netzwerkzugriff, um Registries zu erreichen. Der Standard-**Trusted**-Netzwerkzugriff ermöglicht Verbindungen zu [gängigen Paketregistries](#default-allowed-domains), einschließlich npm, PyPI, RubyGems und crates.io. Skripte schlagen fehl, Pakete zu installieren, wenn Ihre Umgebung **None**-Netzwerkzugriff verwendet.
</Note>

<h3 id="environment-caching">
  Umgebungs-Caching
</h3>

Das Setup-Skript wird beim ersten Starten einer Sitzung in einer Umgebung ausgeführt. Nach Abschluss erstellt Anthropic einen Snapshot des Dateisystems und verwendet diesen Snapshot als Ausgangspunkt für spätere Sitzungen. Neue Sitzungen starten mit Ihren Abhängigkeiten, Tools und Docker-Images bereits auf der Festplatte, und der Setup-Skript-Schritt wird übersprungen. Dies hält den Start schnell, auch wenn das Skript große Toolchains installiert oder Container-Images abruft.

Der Cache erfasst Dateien, keine laufenden Prozesse. Alles, das das Setup-Skript auf die Festplatte schreibt, wird übertragen. Services oder Container, die es startet, nicht, daher starten Sie diese pro Sitzung, indem Sie Claude bitten oder einen [SessionStart-Hook](#setup-scripts-vs-sessionstart-hooks) verwenden.

Das Setup-Skript wird erneut ausgeführt, um den Cache neu zu erstellen, wenn Sie das Setup-Skript der Umgebung oder die zulässigen Netzwerk-Hosts ändern, und wenn der Cache nach ungefähr sieben Tagen abläuft. Das Fortsetzen einer vorhandenen Sitzung führt das Setup-Skript niemals erneut aus.

Sie müssen Caching nicht aktivieren oder Snapshots selbst verwalten.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Setup-Skripte vs. SessionStart-Hooks
</h3>

Verwenden Sie ein Setup-Skript, um Dinge zu installieren, die die Cloud benötigt, aber Ihr Laptop bereits hat, wie eine Sprachlaufzeit oder ein CLI-Tool. Verwenden Sie einen [SessionStart-Hook](/docs/de/hooks#sessionstart) für Projekt-Setup, das überall ausgeführt werden sollte, Cloud und lokal, wie `npm install`.

Beide werden am Anfang einer Sitzung ausgeführt, aber sie gehören an verschiedene Orte:

|                 | Setup-Skripte                                                                                 | SessionStart-Hooks                                                       |
| --------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| Angehängt an    | Die Cloud-Umgebung                                                                            | Ihr Repository                                                           |
| Konfiguriert in | Cloud-Umgebungs-UI                                                                            | `.claude/settings.json` in Ihrem Repo                                    |
| Wird ausgeführt | Bevor Claude Code startet, wenn keine [gecachte Umgebung](#environment-caching) verfügbar ist | Nach Claude Code startet, bei jeder Sitzung einschließlich fortgesetzter |
| Umfang          | Nur Cloud-Umgebungen                                                                          | Sowohl lokal als auch Cloud                                              |

SessionStart-Hooks können auch in Ihrer Benutzer-Level-Datei `~/.claude/settings.json` lokal definiert werden, aber Benutzer-Level-Einstellungen werden nicht zu Cloud-Sitzungen übertragen. In der Cloud werden Hooks aus dem Repo und aus den [server-verwalteten Einstellungen](/docs/de/server-managed-settings) Ihrer Organisation ausgeführt.

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Abhängigkeiten mit einem SessionStart-Hook installieren
</h3>

Um Abhängigkeiten nur in Cloud-Sitzungen zu installieren, fügen Sie einen SessionStart-Hook zu Ihrer Repo's `.claude/settings.json` hinzu:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Erstellen Sie das Skript unter `scripts/install_pkgs.sh` und machen Sie es ausführbar mit `chmod +x`. Die Umgebungsvariable `CLAUDE_CODE_REMOTE` ist in Cloud-Sitzungen auf `true` gesetzt, daher können Sie sie verwenden, um die lokale Ausführung zu überspringen:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

SessionStart-Hooks haben einige Einschränkungen in Cloud-Sitzungen:

* **Keine Cloud-Only-Scoping**: Hooks werden in lokalen und Cloud-Sitzungen ausgeführt. Um die lokale Ausführung zu überspringen, überprüfen Sie die Umgebungsvariable `CLAUDE_CODE_REMOTE` wie oben gezeigt.
* **Erfordert Netzwerkzugriff**: Installationsbefehle benötigen Zugriff auf Paketregistries. Wenn Ihre Umgebung **None**-Netzwerkzugriff verwendet, schlagen diese Hooks fehl. Die [Standard-Allowlist](#default-allowed-domains) unter **Trusted** deckt npm, PyPI, RubyGems und crates.io ab.
* **Proxy-Kompatibilität**: Der gesamte ausgehende Datenverkehr läuft durch einen [Sicherheits-Proxy](#security-proxy). Einige Paketmanager funktionieren mit diesem Proxy nicht korrekt. Bun ist ein bekanntes Beispiel.
* **Fügt Startup-Latenz hinzu**: Hooks werden jedes Mal ausgeführt, wenn eine Sitzung startet oder fortgesetzt wird, im Gegensatz zu Setup-Skripten, die von [Umgebungs-Caching](#environment-caching) profitieren. Halten Sie Installationsskripte schnell, indem Sie überprüfen, ob Abhängigkeiten bereits vorhanden sind, bevor Sie sie neu installieren.

Um Umgebungsvariablen für nachfolgende Bash-Befehle beizubehalten, schreiben Sie in die Datei unter `$CLAUDE_ENV_FILE`. Siehe [SessionStart-Hooks](/docs/de/hooks#sessionstart) für Details.

Das Ersetzen des Basis-Images durch Ihr eigenes Docker-Image wird noch nicht unterstützt. Verwenden Sie ein Setup-Skript, um zu installieren, was Sie auf dem [bereitgestellten Image](#installed-tools) benötigen, oder führen Sie Ihr Image als Container neben Claude mit `docker compose` aus.

<h2 id="network-access">
  Netzwerkzugriff
</h2>

Der Netzwerkzugriff steuert ausgehende Verbindungen aus der Cloud-Umgebung. Jede Umgebung gibt eine Zugriffsstufe an, und Sie können sie mit benutzerdefinierten zulässigen Domains erweitern. Der Standard ist **Trusted**, das Paketregistries und andere [Allowlist-Domains](#default-allowed-domains) ermöglicht.

Um den Netzwerkzugriff einer Umgebung zu ändern, [öffnen Sie sie zum Bearbeiten](#configure-your-environment) und verwenden Sie den **Netzwerkzugriff**-Selektor im Dialog. Es gibt keine separate Seite für Umgebungen. Das Cloud-Symbol wird überall angezeigt, wo Sie eine Cloud-Sitzung starten oder eine [Routine](/docs/de/routines#environments-and-network-access) konfigurieren.

<Note>
  MCP-Connector-Datenverkehr wird über Anthropics Server geleitet, daher funktionieren die Connectors, die Sie auf einer Sitzung oder Routine aktivieren, ohne ihre Hosts zu **Zulässigen Domains** hinzuzufügen. Connectors werden pro Sitzung oder pro Routine konfiguriert; entfernen Sie alle, die Sie nicht benötigen, um zu begrenzen, welche Tools Claude erreichen kann. Dies basiert auf dem gleichen Anthropic-gebundenen Kanal, der unter [Sicherheit und Isolation](#security-and-isolation) erwähnt wird.
</Note>

<h3 id="access-levels">
  Zugriffsstufen
</h3>

Wählen Sie eine Zugriffsstufe, wenn Sie eine Umgebung erstellen oder bearbeiten:

| Stufe       | Ausgehende Verbindungen                                                                |
| :---------- | :------------------------------------------------------------------------------------- |
| **None**    | Kein ausgehender Netzwerkzugriff                                                       |
| **Trusted** | [Allowlist-Domains](#default-allowed-domains) nur: Paketregistries, GitHub, Cloud-SDKs |
| **Full**    | Jede Domain                                                                            |
| **Custom**  | Ihre eigene Allowlist, optional einschließlich der Standards                           |

GitHub-Operationen verwenden einen [separaten Proxy](#github-proxy), der unabhängig von dieser Einstellung ist.

<h3 id="allow-specific-domains">
  Spezifische Domains zulassen
</h3>

Um Domains zuzulassen, die nicht in der Trusted-Liste enthalten sind, wählen Sie **Custom** in den Netzwerkzugriffseinstellungen der Umgebung. Ein Feld **Zulässige Domains** wird angezeigt. Geben Sie eine Domain pro Zeile ein:

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Verwenden Sie `*.` für Wildcard-Subdomain-Matching. Aktivieren Sie **Auch Standard-Liste der gängigen Paketmanager einschließen**, um die [Trusted-Domains](#default-allowed-domains) neben Ihren benutzerdefinierten Einträgen zu behalten, oder lassen Sie es deaktiviert, um nur das zuzulassen, was Sie auflisten.

Zulässige Domains werden pro Umgebung konfiguriert. Es gibt keine Allowlist auf Organisationsebene, die Besitzer an alle Umgebungen der Benutzer übertragen können; [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) können Cloud-Sitzungen einschränken, können aber keine zulässigen Domains hinzufügen.

<h3 id="github-proxy">
  GitHub-Proxy
</h3>

Aus Sicherheitsgründen gehen alle GitHub-Operationen durch einen dedizierten Proxy-Service, der Ihre echten GitHub-Anmeldedaten außerhalb der Sandbox hält. Der Proxy authentifiziert zwei Arten von Datenverkehr:

* Git-Interaktionen: Der Git-Client innerhalb der Sandbox verwendet ein benutzerdefiniertes Scoped-Credential, das der Proxy überprüft und in Ihr tatsächliches GitHub-Authentifizierungstoken übersetzt
* GitHub-API-Anfragen: Der Proxy ersetzt Ihre echten Anmeldedaten bei Anfragen von den integrierten GitHub-Tools und von `gh`, wenn Ihre Sitzung den in [Arbeiten mit GitHub-Issues und Pull Requests](#work-with-github-issues-and-pull-requests) beschriebenen `proxy-injected`-Platzhalter setzt

Der Proxy beschränkt auch Git-Push-Operationen auf den aktuellen Arbeitsbranch aus Sicherheitsgründen und ermöglicht Klonen, Abrufen und PR-Operationen bei Beibehaltung von Sicherheitsgrenzen.

Der Proxy begrenzt GitHub-API- und Release-Asset-Anfragen auf Repositories, die an die Sitzung angehängt sind, unabhängig von der [Zugriffsstufe](#access-levels) der Umgebung. Setup-Skripte, die Release-Assets aus nicht angehängten Repositories herunterladen, geben einen 403-Fehler zurück. Committed-Dateien aus öffentlichen Repositories werden über `raw.githubusercontent.com` abgerufen, das der [Sicherheits-Proxy](#security-proxy) stattdessen verwaltet. Diese Domain befindet sich in der Standard-[Trusted-Liste](#default-allowed-domains), daher bleiben die Dateien erreichbar, es sei denn, die [Zugriffsstufe](#access-levels) der Umgebung schließt sie aus.

<h3 id="security-proxy">
  Sicherheits-Proxy
</h3>

Umgebungen werden aus Sicherheits- und Missbrauchspräventionsgründen hinter einem HTTP/HTTPS-Netzwerk-Proxy ausgeführt. Der gesamte ausgehende Internetdatenverkehr läuft durch diesen Proxy, der Folgendes bietet:

* Schutz vor böswilligen Anfragen
* Ratenbegrenzung und Missbrauchsprävention
* Inhaltsfilterung für erhöhte Sicherheit
* Ein DNS-Audit-Trail der angeforderten Hostnamen

<h3 id="default-allowed-domains">
  Standard-Allowlist-Domains
</h3>

Bei Verwendung von **Trusted**-Netzwerkzugriff sind die folgenden Domains standardmäßig zulässig. Domains, die mit `*` gekennzeichnet sind, zeigen Wildcard-Subdomain-Matching an, daher erlaubt `*.gcr.io` jede Subdomain von `gcr.io`.

<AccordionGroup>
  <Accordion title="Anthropic-Services">
    * api.anthropic.com
    * statsig.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Versionskontrolle">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Container-Registries">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Cloud-Plattformen">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
  </Accordion>

  <Accordion title="JavaScript und Node-Paketmanager">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
  </Accordion>

  <Accordion title="Python-Paketmanager">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Ruby-Paketmanager">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Rust-Paketmanager">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Go-Paketmanager">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="JVM-Paketmanager">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Andere Paketmanager">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Linux-Distributionen">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Entwicklungstools und Plattformen">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Cloud-Services und Überwachung">
    * statsig.com
    * [www.statsig.com](http://www.statsig.com)
    * api.statsig.com
    * sentry.io
    * \*.sentry.io
    * downloads.sentry-cdn.com
    * http-intake.logs.datadoghq.com
    * browser-intake-us5-datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Content Delivery und Mirrors">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Schema und Konfiguration">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="move-tasks-between-web-and-terminal">
  Aufgaben zwischen Web und Terminal verschieben
</h2>

Diese Workflows erfordern die [Claude Code CLI](/docs/de/quickstart), die bei demselben claude.ai-Konto angemeldet ist. Sie können neue Cloud-Sitzungen von Ihrem Terminal aus starten oder Cloud-Sitzungen in Ihr Terminal ziehen, um lokal fortzufahren. Cloud-Sitzungen bleiben bestehen, auch wenn Sie Ihren Laptop schließen, und Sie können sie von überall aus überwachen, einschließlich der Claude Mobile-App.

<Note>
  Von der CLI ist die Sitzungsübergabe unidirektional: Sie können Cloud-Sitzungen mit `--teleport` in Ihr Terminal ziehen, aber Sie können keine vorhandene Terminal-Sitzung ins Web verschieben. Das Flag `--cloud` erstellt eine neue Cloud-Sitzung für Ihr aktuelles Repository. Die [Desktop-App](/docs/de/desktop#continue-in-another-surface) bietet ein Continue in-Menü, das eine lokale Sitzung ins Web senden kann.
</Note>

<h3 id="from-terminal-to-web">
  Vom Terminal zum Web
</h3>

Starten Sie eine Cloud-Sitzung von der Befehlszeile mit dem Flag `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Dies erstellt eine neue Cloud-Sitzung auf claude.ai. Die Sitzung klont Ihr aktuelles Verzeichnis's GitHub-Remote bei Ihrem aktuellen Branch, daher pushen Sie zuerst, wenn Sie lokale Commits haben, da die VM von GitHub klont, nicht von Ihrem Computer. `--cloud` funktioniert mit einem Repository auf einmal. Die Aufgabe wird in der Cloud ausgeführt, während Sie lokal weiterarbeiten. Die ältere Schreibweise `--remote` funktioniert immer noch als veralteter Alias für `--cloud`.

Ab v2.1.195 zeigt die CLI eine Live-Checkliste von Setup-Schritten an, wie z. B. das Klonen des Repositories und das Ausführen Ihres [Setup-Skripts](#setup-scripts), während der Cloud-Container startet. Nachrichten, die Sie eingeben, während der Container bereitgestellt wird, werden in die Warteschlange eingereiht und gesendet, sobald die Sitzung bereit ist.

<Note>
  `--cloud` erstellt Cloud-Sitzungen. `--remote-control` ist nicht verwandt: Es stellt eine lokale CLI-Sitzung zur Überwachung vom Web aus bereit. Siehe [Remote Control](/docs/de/remote-control).
</Note>

Verwenden Sie `/tasks` in der Claude Code CLI, um den Fortschritt zu überprüfen, oder öffnen Sie die Sitzung auf claude.ai oder der Claude Mobile-App, um direkt zu interagieren. Von dort aus können Sie Claude steuern, Feedback geben oder Fragen beantworten, genau wie in jedem anderen Gespräch.

<h4 id="tips-for-cloud-tasks">
  Tipps für Cloud-Aufgaben
</h4>

**Planen Sie lokal, führen Sie remote aus**: Für komplexe Aufgaben starten Sie Claude im Plan Mode, um den Ansatz zu besprechen, und senden Sie dann die Arbeit ins Web:

```bash theme={null}
claude --permission-mode plan
```

Im Plan Mode liest Claude Dateien, führt Befehle aus, um zu erkunden, und schlägt einen Plan vor, ohne Quellcode zu bearbeiten. Sobald Sie mit dem Plan zufrieden sind, speichern Sie den Plan im Repo, committen und pushen Sie, damit die Cloud-VM ihn klonen kann. Dann starten Sie eine Cloud-Sitzung für autonome Ausführung:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

Dieses Muster gibt Ihnen Kontrolle über die Strategie, während Claude autonom in der Cloud ausgeführt wird.

**Planen Sie in der Cloud mit ultraplan**: Um den Plan selbst in einer Web-Sitzung zu entwerfen und zu überprüfen, verwenden Sie [ultraplan](/docs/de/ultraplan). Claude generiert den Plan auf Claude Code im Web, während Sie weiterarbeiten, dann kommentieren Sie Abschnitte in Ihrem Browser und wählen, ob Sie remote ausführen oder den Plan zurück zu Ihrem Terminal senden.

**Führen Sie Aufgaben parallel aus**: Jeder `--cloud`-Befehl erstellt seine eigene Cloud-Sitzung, die unabhängig ausgeführt wird. Sie können mehrere Aufgaben starten und sie werden alle gleichzeitig in separaten Sitzungen ausgeführt:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Überwachen Sie alle Sitzungen mit `/tasks` in der Claude Code CLI. Wenn eine Sitzung abgeschlossen ist, können Sie einen PR aus der Web-Oberfläche erstellen oder [die Sitzung teleportieren](#from-web-to-terminal), um lokal fortzufahren.

<h4 id="send-local-repositories-without-github">
  Senden Sie lokale Repositories ohne GitHub
</h4>

Wenn Sie `claude --cloud` aus einem Repository ausführen, das nicht mit GitHub verbunden ist, bündelt Claude Code Ihr lokales Repository und lädt es direkt in die Cloud-Sitzung hoch. Das Bündel enthält Ihre vollständige Repository-Historie über alle Branches hinweg, plus alle nicht committeten Änderungen an verfolgten Dateien.

Dieses Fallback wird automatisch aktiviert, wenn GitHub-Zugriff nicht verfügbar ist. Um es zu erzwingen, auch wenn GitHub verbunden ist, setzen Sie `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Gebündelte Repositories müssen diese Limits erfüllen:

* Das Verzeichnis muss ein Git-Repository mit mindestens einem Commit sein
* Das gebündelte Repository muss unter 100 MB liegen. Größere Repositories fallen auf das Bündeln nur des aktuellen Branches zurück, dann auf einen einzelnen gequetschten Snapshot des Arbeitsbaums, und schlagen nur fehl, wenn der Snapshot immer noch zu groß ist
* Nicht verfolgte Dateien sind nicht enthalten; führen Sie `git add` auf Dateien aus, die die Cloud-Sitzung sehen soll
* Sitzungen, die aus einem Bündel erstellt wurden, können nicht zurück zu einem Remote pushen, es sei denn, Sie haben auch [GitHub-Authentifizierung](#github-authentication-options) konfiguriert

<h3 id="from-web-to-terminal">
  Vom Web zum Terminal
</h3>

Ziehen Sie eine Cloud-Sitzung in Ihr Terminal mit einer dieser Methoden:

* **Mit `--teleport`**: Führen Sie von der Befehlszeile `claude --teleport` für eine interaktive Sitzungsauswahl aus, oder `claude --teleport <session-id>`, um eine bestimmte Sitzung direkt fortzusetzen. Wenn Sie nicht committete Änderungen haben, werden Sie aufgefordert, diese zuerst zu stashen.
* **Mit `/teleport`**: Führen Sie innerhalb einer vorhandenen CLI-Sitzung `/teleport` oder `/tp` aus, um die gleiche Sitzungsauswahl zu öffnen, ohne Claude Code neu zu starten.
* **Von `/tasks`**: Führen Sie `/tasks` aus, um Ihre Hintergrund-Sitzungen zu sehen, drücken Sie dann `t`, um in eine zu teleportieren.
* **Von der Web-Oberfläche**: Wählen Sie **Open in CLI**, um einen Befehl zu kopieren, den Sie in Ihr Terminal einfügen können.

Wenn Sie eine Sitzung teleportieren, überprüft Claude, dass Sie sich im richtigen Repository befinden, ruft den Branch aus der Cloud-Sitzung ab und checkt ihn aus, und lädt die vollständige Gesprächshistorie in Ihr Terminal.

`--teleport` unterscheidet sich von `--resume`. `--resume` öffnet ein Gespräch aus der lokalen Historie dieser Maschine und listet keine Cloud-Sitzungen auf; `--teleport` zieht eine Cloud-Sitzung und ihren Branch.

<h4 id="teleport-requirements">
  Teleport-Anforderungen
</h4>

Teleport überprüft diese Anforderungen, bevor eine Sitzung fortgesetzt wird. Wenn eine Anforderung nicht erfüllt ist, sehen Sie einen Fehler oder werden aufgefordert, das Problem zu beheben.

| Anforderung          | Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sauberer Git-Status  | Ihr Arbeitsverzeichnis darf keine nicht committeten Änderungen haben. Teleport fordert Sie auf, Änderungen zu stashen, falls erforderlich.                                                                                                                                                                                                                                                                                                                                                           |
| Korrektes Repository | Sie müssen `--teleport` aus einem Checkout desselben Repositories ausführen, nicht aus einem Fork. Ab v2.1.199 akzeptiert Claude Code einen Checkout auch dann, wenn es das Remote nicht in einen Hostnamen analysieren kann, wie z. B. einen SSH-Host-Alias wie `git@work:owner/repo.git` oder eine `insteadOf`-umgeschriebene Kurzform. Es zeigt zuerst eine Bestätigungsaufforderung an und nur, wenn der Owner und der Repository-Name des Remote mit dem Repository der Sitzung übereinstimmen. |
| Branch verfügbar     | Der Branch aus der Cloud-Sitzung muss in das Remote gepusht worden sein. Teleport ruft ihn automatisch ab und checkt ihn aus.                                                                                                                                                                                                                                                                                                                                                                        |
| Gleiches Konto       | Sie müssen sich bei demselben claude.ai-Konto authentifizieren, das in der Cloud-Sitzung verwendet wurde.                                                                                                                                                                                                                                                                                                                                                                                            |

<h4 id="teleport-is-unavailable">
  `--teleport` ist nicht verfügbar
</h4>

Teleport erfordert claude.ai-Abonnement-Authentifizierung. Wenn Sie sich über API-Schlüssel, Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry authentifizieren, führen Sie `/login` aus, um sich stattdessen mit Ihrem claude.ai-Konto anzumelden. Wenn Sie bereits über claude.ai angemeldet sind und `--teleport` immer noch nicht verfügbar ist, hat Ihre Organisation möglicherweise Cloud-Sitzungen deaktiviert.

<h2 id="work-with-sessions">
  Mit Sitzungen arbeiten
</h2>

Sitzungen werden in der Seitenleiste unter claude.ai/code angezeigt. Von dort aus können Sie Änderungen überprüfen, mit Teamkollegen teilen, abgeschlossene Arbeiten archivieren oder Sitzungen dauerhaft löschen.

<h3 id="manage-context">
  Kontext verwalten
</h3>

Cloud-Sitzungen unterstützen [integrierte Befehle](/docs/de/commands), die Textausgabe erzeugen. Befehle, die nur in der Terminal-Schnittstelle ausgeführt werden, wie `/plugin` oder `/resume`, sind nicht verfügbar. Befehle, die eine Auswahl oder ein Panel in der Terminal-Schnittstelle öffnen, verhalten sich in Cloud-Sitzungen unterschiedlich:

* **`/model`, `/effort`, `/fast`, `/color` und `/rename`**: Übergeben Sie den Wert als Argument, zum Beispiel `/model sonnet`, anstatt die Terminal-Auswahl oder den Schieberegler zu öffnen. Die Argumentformen erfordern Claude Code v2.1.205 oder später in der Umgebung der Sitzung und folgen den [Verfügbarkeitshinweisen](/docs/de/commands#all-commands) jedes Befehls: `/effort` meldet `Not applied`, während ein [Launch-Standard-Effort-Hold](/docs/de/model-config#adjust-effort-level) eines Modells in Kraft ist, und `/fast` funktioniert nur in einer Sitzung, die mit aktiviertem Fast-Modus gestartet wurde.
* **`/config`**: Im Web öffnet dies den Claude Code-Bereich Ihrer Einstellungen, anstatt einen Wert zu setzen, und Text nach dem Befehl, einschließlich `key=value`, wird ignoriert. Um Einstellungen für eine Cloud-Sitzung zu ändern, verwenden Sie [Umgebungsvariablen](#configure-your-environment) oder committen Sie [Einstellungsdateien](/docs/de/settings) in das Repository.

Für Kontextverwaltung speziell:

| Befehl     | Funktioniert in Cloud-Sitzungen | Notizen                                                                                                                         |
| :--------- | :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ |
| `/compact` | Ja                              | Fasst das Gespräch zusammen, um Kontext freizugeben. Akzeptiert optionale Fokus-Anweisungen wie `/compact keep the test output` |
| `/context` | Ja                              | Zeigt, was sich derzeit im Kontextfenster befindet                                                                              |
| `/clear`   | Nein                            | Starten Sie stattdessen eine neue Sitzung aus der Seitenleiste                                                                  |

Auto-Kompaktierung wird automatisch ausgeführt, wenn sich das Kontextfenster der Kapazität nähert. Um es früher auszulösen, setzen Sie [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/de/env-vars) in Ihren [Umgebungsvariablen](#configure-your-environment). Zum Beispiel kompaktiert `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` bei 70% Kapazität statt des Wartens, bis das Fenster fast voll ist. Um die effektive Fenstergröße für Kompaktierungsberechnungen zu ändern, verwenden Sie [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/de/env-vars).

[Subagents](/docs/de/sub-agents) funktionieren genauso wie lokal. Claude kann sie mit dem Task-Tool spawnen, um Forschung oder parallele Arbeit in ein separates Kontextfenster auszulagern, um das Hauptgespräch leichter zu halten. Subagents, die in Ihrem Repo's `.claude/agents/` definiert sind, werden automatisch aufgegriffen.

[Agent-Teams](/docs/de/agent-teams) sind standardmäßig deaktiviert, können aber aktiviert werden, indem Sie `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` zu Ihren [Umgebungsvariablen](#configure-your-environment) hinzufügen.

<h3 id="review-changes">
  Änderungen überprüfen
</h3>

Jede Sitzung zeigt einen Diff-Indikator mit hinzugefügten und entfernten Zeilen, wie `+42 -18`. Wählen Sie ihn, um die Diff-Ansicht zu öffnen, hinterlassen Sie Inline-Kommentare zu bestimmten Zeilen und senden Sie sie mit Ihrer nächsten Nachricht an Claude. Siehe [Überprüfung und Iteration](/docs/de/web-quickstart#review-and-iterate) für die vollständige Anleitung, einschließlich PR-Erstellung. Um Claude den PR auf CI-Fehler und Review-Kommentare automatisch überwachen zu lassen, siehe [Auto-fix Pull Requests](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Sitzungen teilen
</h3>

Um eine Sitzung zu teilen, schalten Sie ihre Sichtbarkeit gemäß den folgenden Kontotypen um. Danach teilen Sie den Sitzungslink wie gewohnt. Empfänger sehen den neuesten Status, wenn sie den Link öffnen, aber ihre Ansicht wird nicht in Echtzeit aktualisiert.

<h4 id="share-from-an-enterprise-or-team-account">
  Teilen von einem Enterprise- oder Team-Konto
</h4>

Für Enterprise- und Team-Konten sind die beiden Sichtbarkeitsoptionen **Private** und **Team**. Team-Sichtbarkeit macht die Sitzung für andere Mitglieder Ihrer claude.ai-Organisation sichtbar. [Claude in Slack](/docs/de/slack)-Sitzungen werden automatisch mit Team-Sichtbarkeit geteilt.

Die Überprüfung des Repository-Zugriffs ist standardmäßig aktiviert, basierend auf dem GitHub-Konto, das mit dem Konto des Empfängers verbunden ist. Der Anzeigename Ihres Kontos ist für alle Empfänger mit Zugriff sichtbar.

<h4 id="share-from-a-max-or-pro-account">
  Teilen von einem Max- oder Pro-Konto
</h4>

Für Max- und Pro-Konten sind die beiden Sichtbarkeitsoptionen **Private** und **Public**. Public-Sichtbarkeit macht die Sitzung für jeden Benutzer sichtbar, der bei claude.ai angemeldet ist.

Überprüfen Sie Ihre Sitzung auf sensible Inhalte, bevor Sie sie teilen. Sitzungen können Code und Anmeldedaten aus privaten GitHub-Repositories enthalten. Die Überprüfung des Repository-Zugriffs ist standardmäßig nicht aktiviert.

Um zu verlangen, dass Empfänger Repository-Zugriff haben, oder um Ihren Namen aus gemeinsamen Sitzungen auszublenden, gehen Sie zu Einstellungen > Claude Code > Freigabeeinstellungen.

<h3 id="archive-sessions">
  Sitzungen archivieren
</h3>

Sie können Sitzungen archivieren, um Ihre Sitzungsliste organisiert zu halten. Archivierte Sitzungen sind in der Standard-Sitzungsliste ausgeblendet, können aber durch Filtern nach archivierten Sitzungen angezeigt werden.

Um eine Sitzung zu archivieren, bewegen Sie den Mauszeiger über die Sitzung in der Seitenleiste und wählen Sie das Archiv-Symbol.

<h3 id="delete-sessions">
  Sitzungen löschen
</h3>

Das Löschen einer Sitzung entfernt die Sitzung und ihre Daten dauerhaft. Diese Aktion kann nicht rückgängig gemacht werden. Sie können eine Sitzung auf zwei Arten löschen:

* **Von der Seitenleiste**: Filtern Sie nach archivierten Sitzungen, bewegen Sie dann den Mauszeiger über die Sitzung, die Sie löschen möchten, und wählen Sie das Lösch-Symbol
* **Vom Sitzungsmenü**: Öffnen Sie eine Sitzung, wählen Sie das Dropdown-Menü neben dem Sitzungstitel und wählen Sie **Löschen**

Sie werden aufgefordert, vor dem Löschen einer Sitzung zu bestätigen.

<h2 id="auto-fix-pull-requests">
  Auto-fix Pull Requests
</h2>

Claude kann einen Pull Request überwachen und automatisch auf CI-Fehler und Review-Kommentare reagieren. Claude abonniert GitHub-Aktivitäten auf dem PR, und wenn eine Überprüfung fehlschlägt oder ein Reviewer einen Kommentar hinterlässt, untersucht Claude das Problem und pusht eine Lösung, wenn eine klar ist.

<Note>
  Auto-fix erfordert, dass die Claude GitHub App auf Ihrem Repository installiert ist. Falls noch nicht geschehen, installieren Sie sie von der [GitHub App-Seite](https://github.com/apps/claude) oder wenn Sie dazu während des [Setups](/docs/de/web-quickstart#connect-github-and-create-an-environment) aufgefordert werden.
</Note>

Es gibt mehrere Möglichkeiten, Auto-fix zu aktivieren, je nachdem, woher der PR stammt und welches Gerät Sie verwenden:

* **PRs, die in Claude Code im Web erstellt wurden**: Öffnen Sie die CI-Statusleiste und wählen Sie **Auto-fix**
* **Von Ihrem Terminal**: Führen Sie [`/autofix-pr`](/docs/de/commands) aus, während Sie auf dem PR's Branch sind. Claude Code erkennt den offenen PR mit `gh`, spawnt eine Web-Sitzung und aktiviert Auto-fix in einem Schritt
* **Von der Mobile-App**: Sagen Sie Claude, den PR zu auto-fixen, zum Beispiel „watch this PR and fix any CI failures or review comments"
* **Jeder vorhandene PR**: Fügen Sie die PR-URL in eine Sitzung ein und sagen Sie Claude, den PR zu auto-fixen

Auto-fix ist ein Pro-PR-Toggle. Um die Überwachung zu beenden, öffnen Sie die CI-Statusleiste in der Web-Sitzung und deaktivieren Sie den **Auto-fix**-Toggle, oder sagen Sie Claude, die Überwachung des PR zu beenden.

<h3 id="how-claude-responds-to-pr-activity">
  Wie Claude auf PR-Aktivität reagiert
</h3>

Wenn Auto-fix aktiv ist, empfängt Claude GitHub-Events für den PR, einschließlich neuer Review-Kommentare und CI-Check-Fehler. Für jedes Event untersucht Claude das Problem und entscheidet, wie vorgegangen wird:

* **Klare Fixes**: Wenn Claude sich einer Lösung sicher ist und sie nicht mit früheren Anweisungen in Konflikt steht, nimmt Claude die Änderung vor, pusht sie und erklärt, was getan wurde, in der Sitzung
* **Mehrdeutige Anfragen**: Wenn ein Reviewer-Kommentar auf mehrere Arten interpretiert werden könnte oder etwas architektonisch Bedeutsames betrifft, fragt Claude Sie, bevor er handelt
* **Doppelte oder keine Aktion erforderlich Events**: Wenn ein Event ein Duplikat ist oder keine Änderung erfordert, notiert Claude es in der Sitzung und fährt fort

GitHub gibt keinen Webhook aus, wenn der Basis-Branch voranschreitet und einen Merge-Konflikt erzeugt, daher kann Auto-fix nicht von selbst auf Konflikte reagieren. Um einen Konflikt zu beheben, öffnen Sie die Sitzung und bitten Sie Claude, einen Rebase durchzuführen.

Claude kann als Teil der Auflösung auf Review-Kommentar-Threads auf GitHub antworten. Diese Antworten werden mit Ihrem GitHub-Konto gepostet, sodass sie unter Ihrem Benutzernamen erscheinen, aber jede Antwort ist als von Claude Code stammend gekennzeichnet, damit Reviewer wissen, dass sie vom Agent geschrieben wurde und nicht direkt von Ihnen.

<Warning>
  Wenn Ihr Repository Kommentar-ausgelöste Automatisierung wie Atlantis, Terraform Cloud oder benutzerdefinierte GitHub Actions verwendet, die auf `issue_comment`-Events ausgeführt werden, beachten Sie, dass Claude auf Ihrem Behalf antworten kann, was diese Workflows auslösen kann. Überprüfen Sie die Automatisierung Ihres Repositories, bevor Sie Auto-fix aktivieren, und erwägen Sie, Auto-fix für Repositories zu deaktivieren, in denen ein PR-Kommentar Infrastruktur bereitstellen oder privilegierte Operationen ausführen kann.
</Warning>

<h2 id="security-and-isolation">
  Sicherheit und Isolation
</h2>

Jede Cloud-Sitzung ist von Ihrem Computer und von anderen Sitzungen durch mehrere Schichten getrennt:

* **Isolierte virtuelle Maschinen**: Jede Sitzung wird in einer isolierten, von Anthropic verwalteten VM ausgeführt
* **Netzwerkzugriffskontrolle**: Der Netzwerkzugriff ist standardmäßig begrenzt und kann deaktiviert werden. Wenn Claude Code mit deaktiviertem Netzwerkzugriff ausgeführt wird, kann Claude Code immer noch mit der Anthropic API kommunizieren, was möglicherweise ermöglicht, dass Daten die VM verlassen.
* **Schutz von Anmeldedaten**: Sensible Anmeldedaten wie Git-Anmeldedaten oder Signaturschlüssel befinden sich niemals in der Sandbox mit Claude Code. Die Authentifizierung wird über einen sicheren Proxy mit Scoped-Credentials verwaltet.
* **Sichere Analyse**: Code wird in isolierten VMs analysiert und geändert, bevor PRs erstellt werden

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Für Runtime-API-Fehler, die im Gespräch angezeigt werden, wie `API Error: 500`, `529 Overloaded`, `429` oder `Prompt is too long`, siehe die [Fehlerreferenz](/docs/de/errors). Diese Fehler und ihre Lösungen werden mit der CLI und der Desktop-App geteilt. Die folgenden Abschnitte behandeln Probleme, die spezifisch für Cloud-Sitzungen sind.

<h3 id="session-creation-failed">
  Sitzungserstellung fehlgeschlagen
</h3>

Wenn eine neue Sitzung mit `Session creation failed` fehlschlägt oder bei der Bereitstellung steckenbleibt, konnte Claude Code eine Cloud-Umgebung nicht zuordnen.

* Überprüfen Sie [status.claude.com](https://status.claude.com) auf Cloud-Sitzungs-Incidents
* Versuchen Sie es nach einer Minute erneut, da die Kapazität bei Bedarf bereitgestellt wird
* Bestätigen Sie, dass Ihr Repository erreichbar ist. Das verbindende GitHub-Konto muss Zugriff auf das Repository auf GitHub haben, entweder durch die Claude GitHub App-Autorisierung oder ein `gh`-Token, das über `/web-setup` synchronisiert wird. Die Installation der App auf dem Repository ist nicht erforderlich. Siehe [GitHub-Authentifizierungsoptionen](#github-authentication-options).

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control-Sitzung abgelaufen oder Zugriff verweigert
</h3>

`--teleport` verbindet sich über die gleiche Remote Control-Sitzungsinfrastruktur, die Cloud-Sitzungen verwenden, daher werden Authentifizierungs- und Sitzungs-Ablauf-Fehler mit Remote Control-Wording angezeigt. Sie können `Remote Control session expired` oder `Access denied` sehen. Das Verbindungs-Token ist kurzlebig und auf Ihr Konto begrenzt.

* Führen Sie `/login` lokal aus, um Ihre Anmeldedaten zu aktualisieren, und verbinden Sie sich dann erneut
* Bestätigen Sie, dass Sie sich bei demselben Konto angemeldet haben, das die Sitzung besitzt
* Wenn Sie `Remote Control may not be available for this organization` sehen, hat ein Owner Cloud-Sitzungen für Ihre Organisation nicht aktiviert

<h3 id="environment-expired">
  Umgebung abgelaufen
</h3>

Cloud-Sitzungen werden nach einer Inaktivitätszeit beendet und die zugrunde liegende Umgebung wird freigegeben. Von einem lokalen Terminal aus wird dies als `Could not resume session ... its environment has expired. Creating a fresh session instead.` angezeigt. Im Web wird die Sitzung in der Sitzungsliste als abgelaufen markiert.

Öffnen Sie die Sitzung erneut von [claude.ai/code](https://claude.ai/code), um eine frische Umgebung mit Ihrer wiederhergestellten Gesprächshistorie bereitzustellen.

<h2 id="limitations">
  Einschränkungen
</h2>

Bevor Sie Cloud-Sitzungen für einen Workflow verwenden, berücksichtigen Sie diese Einschränkungen:

* **Ratenlimits**: Claude Code im Web teilt Ratenlimits mit allen anderen Claude- und Claude Code-Nutzungen in Ihrem Konto. Das Ausführen mehrerer Aufgaben parallel verbraucht proportional mehr Ratenlimits. Es gibt keine separate Compute-Gebühr für die Cloud-VM.
* **Repository-Authentifizierung**: Sie können Sitzungen nur vom Web zum lokalen Computer verschieben, wenn Sie sich bei demselben Konto authentifizieren
* **Plattformbeschränkungen**: Repository-Klonen und Pull Request-Erstellung erfordern GitHub. Selbstgehostete [GitHub Enterprise Server](/docs/de/github-enterprise-server)-Instanzen werden für Team- und Enterprise-Pläne unterstützt. GitLab, Bitbucket und andere Nicht-GitHub-Repositories können als lokales [Bündel](#send-local-repositories-without-github) zu Cloud-Sitzungen gesendet werden, aber die Sitzung kann nicht zurück zum Remote pushen
* **Organisations-IP-Allowlist**: Cloud-Sitzungen rufen die Anthropic API von Anthropic-verwalteter Infrastruktur auf, nicht von Ihrem Netzwerk. Wenn Ihre Organisation [IP-Allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) aktiviert hat, schlägt jede Cloud-Sitzung mit einem Authentifizierungsfehler fehl. Das gleiche gilt für [Code Review](/docs/de/code-review) und [Routines](/docs/de/routines). Kontaktieren Sie [Anthropic Support](https://support.claude.com/), um Anthropic-gehostete Services von der IP-Allowlist Ihrer Organisation auszunehmen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Ultraplan](/docs/de/ultraplan): Entwerfen Sie einen Plan in einer Cloud-Sitzung und überprüfen Sie ihn in Ihrem Browser
* [Ultrareview](/docs/de/ultrareview): Führen Sie eine tiefe Multi-Agent-Code-Review in einer Cloud-Sandbox aus
* [Routines](/docs/de/routines): Automatisieren Sie Arbeiten nach einem Zeitplan, über API-Aufruf oder als Reaktion auf GitHub-Events
* [Hooks-Konfiguration](/docs/de/hooks): Führen Sie Skripte bei Sitzungs-Lifecycle-Events aus
* [Einstellungsreferenz](/docs/de/settings): Alle Konfigurationsoptionen
* [Sicherheit](/docs/de/security): Isolationsgarantien und Datenverarbeitung
* [Datennutzung](/docs/de/data-usage): Was Anthropic aus Cloud-Sitzungen behält
* [Claude Tag](https://claude.com/docs/claude-tag/overview): Ein von der Organisation verwaltetes @Claude in Slack, das in derselben Cloud-Umgebung ausgeführt wird
