> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Entwicklungscontainer

> Führen Sie Claude Code in einem Entwicklungscontainer aus, um konsistente, isolierte Umgebungen für Ihr Team zu schaffen.

Ein [Entwicklungscontainer](https://containers.dev/), oder Dev Container, ermöglicht es Ihnen, eine identische, isolierte Umgebung zu definieren, die jeder Ingenieur in Ihrem Team ausführen kann. Mit Claude Code, das in diesem Container installiert ist, werden die Befehle, die Claude ausführt, darin ausgeführt, anstatt auf dem Host-Rechner, während Änderungen an Ihren Projektdateien in Ihrem lokalen Repository angezeigt werden, während Sie arbeiten.

Diese Seite behandelt [die Installation von Claude Code in einem Entwicklungscontainer](#add-claude-code-to-your-dev-container) und dann eine Reihe von eigenständigen Konfigurationsthemen: Authentifizierung über Neuerstellungen hinweg beibehalten, Organisationsrichtlinie durchsetzen, Netzwerk-Egress einschränken und Ausführung ohne Berechtigungsaufforderungen. Lesen Sie diejenigen, die Ihrem Setup entsprechen.

<Warning>
  Obwohl der Entwicklungscontainer erhebliche Schutzmaßnahmen bietet, ist kein System vollständig immun gegen alle Angriffe.
  Bei Ausführung mit `--dangerously-skip-permissions` verhindern Entwicklungscontainer nicht, dass ein bösartiges Projekt alles exfiltriert, das im Container zugänglich ist, einschließlich der Claude Code-Anmeldedaten, die in [`~/.claude`](/docs/de/claude-directory) gespeichert sind.
  Verwenden Sie Entwicklungscontainer nur bei der Entwicklung mit vertrauenswürdigen Repositories, und überwachen Sie die Aktivitäten von Claude.
  Vermeiden Sie das Einbinden von Host-Geheimnissen wie `~/.ssh` oder Cloud-Anmeldedatendateien in den Container; bevorzugen Sie Repository-bezogene oder kurzlebige Token.
</Warning>

<Accordion title="Wie Entwicklungscontainer mit Ihrem Editor funktionieren">
  <img src="https://mintcdn.com/claude-code/YvJyjZfd9yMihr0i/images/devcontainer-architecture.svg?fit=max&auto=format&n=YvJyjZfd9yMihr0i&q=85&s=9017b1d16a446c6cc37ba562f35b9aae" className="dark:hidden" alt="Diagramm, das einen Editor auf dem Host zeigt, der sich mit einem Docker-Entwicklungscontainer verbindet. Claude Code, das Terminal und Build-Tools werden im Container ausgeführt. Das Host-Repository wird als Arbeitsbereich in den Container eingebunden." width="640" height="300" data-path="images/devcontainer-architecture.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/devcontainer-architecture-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=a0a340b1f2afc6a590696102c8acaaca" className="hidden dark:block" alt="Diagramm, das einen Editor auf dem Host zeigt, der sich mit einem Docker-Entwicklungscontainer verbindet. Claude Code, das Terminal und Build-Tools werden im Container ausgeführt. Das Host-Repository wird als Arbeitsbereich in den Container eingebunden." width="640" height="300" data-path="images/devcontainer-architecture-dark.svg" />

  Ein Entwicklungscontainer wird als Docker-Container ausgeführt, entweder auf Ihrem Rechner oder auf einem Cloud-Host wie GitHub Codespaces. Ein Editor, der die Dev Containers-Spezifikation unterstützt, wie VS Code, GitHub Codespaces, eine JetBrains IDE oder Cursor, verbindet sich mit diesem Container: Sie durchsuchen und bearbeiten Dateien im Editor wie gewohnt, aber das integrierte Terminal, die Sprachserver und Build-Tools werden alle im Container ausgeführt, anstatt auf Ihrem Host. Editoren ohne Dev Container-Unterstützung, wie einfaches Vim, sind nicht Teil dieses Workflows.

  Claude Code wird im Container ausgeführt, daher sieht es die gleichen Dateien, Abhängigkeiten und Tools wie der Rest Ihres Projekt-Toolchains. In VS Code können Sie entweder das [Claude Code-Erweiterungspanel](/docs/de/vs-code) verwenden oder `claude` im integrierten Terminal ausführen; beide werden im Container ausgeführt und teilen die gleiche `~/.claude`-Konfiguration.
</Accordion>

<h2 id="add-claude-code-to-your-dev-container">
  Claude Code zu Ihrem Entwicklungscontainer hinzufügen
</h2>

Claude Code wird in jeden Entwicklungscontainer durch die [Claude Code Dev Container Feature](https://github.com/anthropics/devcontainer-features/tree/main/src/claude-code) installiert.

Die Einstellungen funktionieren mit jedem Tool, das die Dev Containers-Spezifikation unterstützt, wie VS Code, GitHub Codespaces oder JetBrains IDEs. Die folgenden Schritte verwenden VS Code als Beispiel.

Wenn Sie den Container in VS Code oder Codespaces öffnen, fügt die Feature auch die Claude Code VS Code-Erweiterung hinzu; andere Editoren ignorieren diesen Teil.

<Tip>
  Neu bei Entwicklungscontainern? Das [VS Code Dev Containers-Tutorial](https://code.visualstudio.com/docs/devcontainers/tutorial) führt Sie durch die Installation von Docker, der Erweiterung und dem Öffnen Ihres ersten Containers. Für ein vollständigeres gehärtetes Beispiel mit einer Firewall und persistenten Volumes siehe [Probieren Sie den Referenz-Container aus](#try-the-reference-container).
</Tip>

<Steps>
  <Step title="Erstellen oder aktualisieren Sie devcontainer.json">
    Speichern Sie das Folgende als `.devcontainer/devcontainer.json` in Ihrem Repository, oder fügen Sie den `features`-Block zu Ihrer vorhandenen Datei hinzu.

    Das Versions-Tag am Ende, wie `:1.0`, fixiert das Installationsskript der Feature, nicht die Claude Code-Version. Die Feature installiert die neueste Claude Code-Version, und Claude Code aktualisiert sich standardmäßig selbst im Container.

    Um die CLI-Version zu fixieren oder die automatische Aktualisierung zu deaktivieren, siehe [Organisationsrichtlinie durchsetzen](#enforce-organization-policy).

    ```json .devcontainer/devcontainer.json theme={null}
    {
      "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
      "features": {
        "ghcr.io/anthropics/devcontainer-features/claude-code:1.0": {}
      }
    }
    ```

    Ersetzen Sie die `image`-Zeile durch das Basis-Image Ihres Projekts, oder entfernen Sie sie, wenn Ihre vorhandene Datei ein Dockerfile verwendet.
  </Step>

  <Step title="Erstellen Sie den Container neu">
    Öffnen Sie die VS Code-Befehlspalette mit `Cmd+Shift+P` auf Mac oder `Ctrl+Shift+P` auf Windows und Linux, und führen Sie **Dev Containers: Rebuild Container** aus.

    Für andere Tools folgen Sie der Neuerstellungsaktion dieses Tools: siehe [Neuerstellung in GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-a-codespace/rebuilding-the-container-in-a-codespace), die [Dev Containers CLI](https://github.com/devcontainers/cli), oder die Dev Container-Dokumentation Ihrer IDE.
  </Step>

  <Step title="Melden Sie sich bei Claude Code an">
    Öffnen Sie ein Terminal im neu erstellten Container und führen Sie `claude` aus, dann folgen Sie der Authentifizierungsaufforderung.
  </Step>
</Steps>

Was Sie bei der Authentifizierungsaufforderung sehen, hängt von Ihrem Anbieter ab:

* **Anthropic**: Melden Sie sich über einen Browser mit Ihrem Claude- oder Anthropic Console-Konto an
* **[Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry](/docs/de/third-party-integrations)**: Claude Code verwendet Ihre Cloud-Anbieter-Anmeldedaten, ohne Browser-Aufforderung

Für Cloud-Anbieter übergeben Sie Anmeldedaten an den Container als Umgebungsvariablen über `containerEnv`, ein Codespaces-Geheimnis oder die Workload-Identität Ihrer Cloud, anstatt Anmeldedatendateien vom Host einzubinden. Siehe [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry) für die Anmeldedatenkette, die Claude Code liest.

Siehe [Wählen Sie Ihren API-Anbieter](/docs/de/admin-setup#choose-your-api-provider), um zu entscheiden, welcher Weg zu Ihrer Organisation passt.

<Note>
  Wenn die Browser-Anmeldung abgeschlossen ist, aber der Rückruf den Container nie erreicht, kopieren Sie den im Browser angezeigten Code und fügen Sie ihn bei der Aufforderung `Paste code here if prompted` im Terminal ein. Dies kann vorkommen, wenn die Port-Weiterleitung des Editors den localhost-Rückruf nicht leitet.
</Note>

<h2 id="persist-authentication-and-settings-across-rebuilds">
  Authentifizierung und Einstellungen über Neuerstellungen hinweg beibehalten
</h2>

Standardmäßig wird das Home-Verzeichnis des Containers bei einer Neuerstellung verworfen, daher müssen sich Ingenieure jedes Mal erneut anmelden. Claude Code speichert sein Authentifizierungs-Token, Benutzereinstellungen und Sitzungsverlauf unter [`~/.claude`](/docs/de/claude-directory). Binden Sie ein benanntes Volume an diesem Pfad ein, um diesen Status über Neuerstellungen hinweg zu bewahren.

Das folgende Beispiel bindet ein Volume im Home-Verzeichnis des `node`-Benutzers ein:

```json devcontainer.json theme={null}
"mounts": [
  "source=claude-code-config,target=/home/node/.claude,type=volume"
]
```

Ersetzen Sie `/home/node` durch das Home-Verzeichnis des `remoteUser` Ihres Containers. Wenn Sie das Volume an einem anderen Ort als `~/.claude` einbinden, setzen Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) auf den Mount-Pfad, damit Claude Code dort liest und schreibt.

Um den Status pro Projekt zu isolieren, anstatt ein Volume über alle Repositories hinweg zu teilen, fügen Sie die Variable `${devcontainerId}` in den Quellnamen ein. Die [Referenzkonfiguration](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) verwendet `source=claude-code-config-${devcontainerId}` zu diesem Zweck.

In GitHub Codespaces bleibt `~/.claude` über das Stoppen und Starten eines Codespace erhalten, wird aber immer noch gelöscht, wenn Sie den Container neu erstellen, daher gilt die obige Volume-Einbindung auch dort. Um die Authentifizierung über Codespaces hinweg zu tragen, speichern Sie `ANTHROPIC_API_KEY` oder ein `CLAUDE_CODE_OAUTH_TOKEN` von [`claude setup-token`](/docs/de/authentication#generate-a-long-lived-token) als [Codespaces-Geheimnis](https://docs.github.com/en/codespaces/managing-your-codespaces/managing-your-account-specific-secrets-for-github-codespaces); Codespaces macht Geheimnisse automatisch als Umgebungsvariablen im Container verfügbar.

<h2 id="enforce-organization-policy">
  Organisationsrichtlinie durchsetzen
</h2>

Ein Entwicklungscontainer ist ein bequemer Ort, um Organisationsrichtlinie anzuwenden, da das gleiche Image und die gleiche Konfiguration auf jedem Rechner eines Ingenieurs ausgeführt werden.

Claude Code liest `/etc/claude-code/managed-settings.json` auf Linux und wendet es mit der höchsten Priorität in der [Einstellungshierarchie](/docs/de/settings#how-scopes-interact) an, daher überschreiben Werte dort alles, das ein Ingenieur in `~/.claude` oder dem `.claude/`-Verzeichnis des Projekts setzt. Kopieren Sie die Datei von Ihrem Dockerfile an den richtigen Ort:

```dockerfile Dockerfile theme={null}
RUN mkdir -p /etc/claude-code
COPY managed-settings.json /etc/claude-code/managed-settings.json
```

Da das Dockerfile im Repository lebt, kann jeder mit Schreibzugriff diesen Schritt ändern oder entfernen. Für Richtlinien, die Ingenieure nicht umgehen können, indem sie Repository-Dateien bearbeiten, liefern Sie verwaltete Einstellungen über [server-verwaltete Einstellungen](/docs/de/server-managed-settings) oder Ihr MDM statt. Siehe [verwaltete Einstellungsdateien](/docs/de/settings#settings-files) für die verfügbaren Schlüssel und die anderen Lieferwege.

Um [Umgebungsvariablen](/docs/de/env-vars) zu setzen, die für jede Claude Code-Sitzung im Container gelten, fügen Sie sie zu `containerEnv` in Ihrer `devcontainer.json` hinzu. Das folgende Beispiel deaktiviert Telemetrie und Fehlerberichterstattung und verhindert, dass Claude Code sich nach der Installation automatisch aktualisiert:

```json devcontainer.json theme={null}
"containerEnv": {
  "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
  "DISABLE_AUTOUPDATER": "1"
}
```

Die Dev Container Feature installiert immer die neueste Claude Code-Version. Um eine bestimmte Claude Code-Version für reproduzierbare Builds zu fixieren, installieren Sie sie von Ihrem Dockerfile mit `npm install -g @anthropic-ai/claude-code@X.Y.Z`, anstatt die Feature zu verwenden, und setzen Sie `DISABLE_AUTOUPDATER` wie oben gezeigt.

Für die vollständige Liste der Richtlinienkontrollen, einschließlich Berechtigungsregeln, Tool-Einschränkungen und MCP-Server-Allowlists, siehe [Richten Sie Claude Code für Ihre Organisation ein](/docs/de/admin-setup).

Um [MCP-Server](/docs/de/mcp) im Container verfügbar zu machen, definieren Sie sie im [Projekt-Scope](/docs/de/mcp#mcp-installation-scopes) in einer `.mcp.json`-Datei im Repository-Root, damit sie zusammen mit Ihrer Dev Container-Konfiguration eingecheckt werden. Installieren Sie alle Binärdateien, von denen lokale Stdio-Server abhängen, in Ihrem Dockerfile, und fügen Sie Remote-Server-Domains zu Ihrer Netzwerk-Allowlist hinzu.

<h2 id="restrict-network-egress">
  Netzwerk-Egress einschränken
</h2>

Sie können den ausgehenden Datenverkehr des Containers auf nur die Domains beschränken, die Claude Code benötigt. Siehe [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements) für die Inferenz- und Authentifizierungsdomains, und [Telemetrie-Dienste](/docs/de/data-usage#telemetry-services) für die optionalen Telemetrie- und Fehlerberichterstattungsverbindungen und wie man sie deaktiviert.

Der Referenz-Container enthält ein [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh)-Skript, das den gesamten ausgehenden Datenverkehr außer den Domains blockiert, die Claude Code und Ihre Entwicklungstools benötigen. Das Ausführen einer Firewall in einem Container erfordert zusätzliche Berechtigungen, daher fügt die Referenz die `NET_ADMIN`- und `NET_RAW`-Fähigkeiten über `runArgs` hinzu. Das Firewall-Skript und diese Fähigkeiten sind nicht erforderlich für Claude Code selbst: Sie können sie weglassen und sich stattdessen auf Ihre eigenen Netzwerkkontrollen verlassen.

<h2 id="run-without-permission-prompts">
  Ohne Berechtigungsaufforderungen ausführen
</h2>

Da der Container Claude Code als Nicht-Root-Benutzer ausführt und die Befehlsausführung auf den Container beschränkt, können Sie `--dangerously-skip-permissions` für unbeaufsichtigte Operationen übergeben. Die CLI lehnt dieses Flag ab, wenn es als Root gestartet wird, daher bestätigen Sie, dass `remoteUser` auf ein Nicht-Root-Konto gesetzt ist.

Das Überspringen von Berechtigungsaufforderungen entfernt Ihre Gelegenheit, Tool-Aufrufe zu überprüfen, bevor sie ausgeführt werden. Claude kann immer noch jede Datei im eingebundenen Arbeitsbereich ändern, die direkt auf Ihrem Host angezeigt wird, und alles erreichen, das die Netzwerkrichtlinie des Containers erlaubt. Kombinieren Sie dieses Flag mit den [Netzwerk-Egress-Einschränkungen](#restrict-network-egress) oben, um zu begrenzen, was eine umgangene Sitzung erreichen kann.

Wenn Sie weniger Aufforderungen ohne Deaktivierung von Sicherheitsprüfungen möchten, erwägen Sie stattdessen [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), der einen Klassifizierer hat, der Aktionen überprüft, bevor sie ausgeführt werden. Um zu verhindern, dass Ingenieure `--dangerously-skip-permissions` überhaupt verwenden, setzen Sie `permissions.disableBypassPermissionsMode` auf `"disable"` in [verwalteten Einstellungen](/docs/de/settings#permission-settings).

<h2 id="try-the-reference-container">
  Probieren Sie den Referenz-Container aus
</h2>

Das [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/.devcontainer)-Repository enthält einen Beispiel-Entwicklungscontainer, der die CLI, die Egress-Firewall, persistente Volumes und eine Zsh-basierte Shell kombiniert. Es wird als funktionierendes Beispiel bereitgestellt, anstatt als verwaltetes Basis-Image; verwenden Sie es, um zu sehen, wie die Teile zusammenpassen, bevor Sie sie auf Ihre eigene Konfiguration anwenden.

<Steps>
  <Step title="Installieren Sie Voraussetzungen">
    Installieren Sie VS Code und die [Dev Containers-Erweiterung](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).
  </Step>

  <Step title="Klonen Sie die Referenz">
    Klonen Sie das [Claude Code-Repository](https://github.com/anthropics/claude-code) und öffnen Sie es in VS Code.
  </Step>

  <Step title="Im Container erneut öffnen">
    Wenn Sie dazu aufgefordert werden, klicken Sie auf **Im Container erneut öffnen**, oder führen Sie **Dev Containers: Im Container erneut öffnen** aus der Befehlspalette aus.
  </Step>

  <Step title="Starten Sie Claude Code">
    Sobald der Container fertig ist, öffnen Sie ein Terminal mit `` Ctrl+` `` und führen Sie `claude` aus, um sich anzumelden und Ihre erste Sitzung zu starten.
  </Step>
</Steps>

Um diese Konfiguration mit Ihrem eigenen Projekt zu verwenden, kopieren Sie das `.devcontainer/`-Verzeichnis in Ihr Repository und passen Sie das Dockerfile für Ihren Toolchain an, oder kehren Sie zu [Claude Code zu Ihrem Entwicklungscontainer hinzufügen](#add-claude-code-to-your-dev-container) zurück, um nur die Feature zu einer Einrichtung hinzuzufügen, die Sie bereits haben.

Die Referenzkonfiguration besteht aus drei Dateien. Keine davon sind erforderlich, wenn Sie Claude Code über die Feature zu Ihrem eigenen Entwicklungscontainer hinzufügen, aber sie zeigen eine Möglichkeit, die Teile zu kombinieren.

| Datei                                                                                                      | Zweck                                                                           |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| [`devcontainer.json`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) | Volume-Mounts, `runArgs`-Fähigkeiten, VS Code-Erweiterungen und `containerEnv`  |
| [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/Dockerfile)               | Basis-Image, Entwicklungstools und die Claude Code-Installation                 |
| [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh)   | Blockiert den gesamten ausgehenden Netzwerkverkehr außer den zulässigen Domains |

<h2 id="next-steps">
  Nächste Schritte
</h2>

Sobald Claude Code in Ihrem Entwicklungscontainer ausgeführt wird, behandeln die folgenden Seiten den Rest eines Organisations-Rollouts: Auswahl eines Authentifizierungswegs, Lieferung verwalteter Richtlinien außerhalb des Repositories, Überwachung der Nutzung und Verständnis dessen, was Claude Code speichert und sendet.

* [Richten Sie Claude Code für Ihre Organisation ein](/docs/de/admin-setup): Wählen Sie einen Authentifizierungsanbieter, entscheiden Sie, wie Richtlinien Geräte erreichen, und planen Sie den Rollout
* [Server-verwaltete Einstellungen](/docs/de/server-managed-settings): Liefern Sie verwaltete Richtlinien aus der Claude.ai-Admin-Konsole, damit Ingenieure sie nicht umgehen können, indem sie Repository-Dateien bearbeiten
* [Überwachen Sie die Nutzung und überprüfen Sie die Aktivität](/docs/de/monitoring-usage): Exportieren Sie OpenTelemetry-Metriken und überprüfen Sie, was Ihr Team ausführt
* [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements): Die vollständige Domain-Allowlist für Proxys und Firewalls
* [Telemetrie-Dienste und Opt-out](/docs/de/data-usage#telemetry-services): Was Claude Code standardmäßig sendet und die Umgebungsvariablen, die es deaktivieren
* [Erkunden Sie das `.claude`-Verzeichnis](/docs/de/claude-directory): Was die Volume-Einbindung enthält, einschließlich Anmeldedaten, Einstellungen und Sitzungsverlauf
* [Sandbox-Umgebungen](/docs/de/sandbox-environments): Vergleichen Sie Entwicklungscontainer mit der integrierten Bash-Sandbox, benutzerdefinierten Containern und VMs
* [Sicherheitsmodell](/docs/de/security): Wie Claude Codes Berechtigungssystem, Sandboxing und Prompt-Injection-Schutz zusammenpassen
* [Berechtigungsmodi](/docs/de/permission-modes): Die vollständige Spanne von Plan Mode bis Auto Mode bis Bypass, und wann man jeden verwendet
