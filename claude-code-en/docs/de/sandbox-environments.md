> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wählen Sie eine Sandbox-Umgebung

> Vergleichen Sie Claude Code Sandbox-Optionen: das integrierte Bash-Tool mit Sandbox, Sandbox-Runtime, Dev Container, Docker und VMs. Wählen Sie die richtige Isolation für Ihr Bedrohungsmodell.

Die Isolierung von Claude Code begrenzt, was eine Sitzung auf dem Dateisystem lesen, schreiben und im Netzwerk erreichen kann. Dies ist besonders wichtig, wenn Sie Claude mit weniger Genehmigungseingaben arbeiten lassen, es unbeaufsichtigt ausführen oder es auf Code verweisen, dem Sie nicht vollständig vertrauen.

Claude Code kann in mehreren Arten isolierter Umgebungen ausgeführt werden, von einer leichtgewichtigen Sandbox pro Befehl bis zu einer vollständig separaten virtuellen Maschine. Diese Seite vergleicht sie danach, was sie isolieren und was sie erfordern, hilft Ihnen, eine für Ihr Bedrohungsmodell auszuwählen, und zeigt, wie Sie diese Wahl in einer Organisation durchsetzen.

<Info>
  Für das umfassendere Sicherheitsmodell siehe [Sicherheit](/docs/de/security). Für Agent SDK-Bereitstellungen siehe [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment).
</Info>

<h2 id="compare-sandboxing-approaches">
  Sandboxing-Ansätze vergleichen
</h2>

Die ersten beiden Ansätze in der folgenden Tabelle werden auf dem Host-Betriebssystem ohne Container ausgeführt. Die übrigen platzieren Claude Code in einem Container oder einer virtuellen Maschine.

| Ansatz                                            | Was wird isoliert                                                                 | Erfordert Docker | Setup-Aufwand                                     |
| :------------------------------------------------ | :-------------------------------------------------------------------------------- | :--------------- | :------------------------------------------------ |
| [Sandboxed Bash tool](#sandboxed-bash-tool)       | Bash-Befehle und ihre untergeordneten Prozesse                                    | Nein             | Minimal auf macOS; niedrig auf Linux und WSL2     |
| [Sandbox runtime](#sandbox-runtime)               | Der gesamte Claude Code-Prozess, einschließlich Datei-Tools, MCP-Server und Hooks | Nein             | Niedrig                                           |
| [Dev container](#dev-containers)                  | Vollständige Entwicklungsumgebung                                                 | Ja               | Mittel                                            |
| [Custom container](#custom-container)             | Vollständige Entwicklungsumgebung                                                 | Ja               | Mittel bis hoch                                   |
| [Virtual machine](#virtual-machine)               | Vollständiges Betriebssystem                                                      | Nein             | Hoch                                              |
| [Claude Code on the web](#claude-code-on-the-web) | Vollständiges Betriebssystem, gehostet von Anthropic                              | Nein             | Keine; erfordert ein Claude-Abonnement und GitHub |

Das [Sandboxed Bash tool](/docs/de/sandboxing) ist in Claude Code integriert und beschränkt nur Bash-Befehle. Integrierte Datei-Tools, MCP-Server und Hooks werden weiterhin direkt auf Ihrem Host ausgeführt. Jeder andere Ansatz in der Tabelle platziert den gesamten Claude Code-Prozess innerhalb der Isolierungsgrenze, sodass auch Datei-Tools, MCP-Server und Hooks eingeschränkt sind.

<Warning>
  Sandbox-Isolation reduziert die Auswirkungen einer Sicherheitsverletzung, beseitigt aber nicht das Risiko. Jeder Ansatz, der Netzwerk-Egress ermöglicht, kann immer noch Daten durchsickern lassen, die der Agent lesen kann, und jeder Ansatz, der Ihr Projektverzeichnis beschreibbar bereitstellt, kann immer noch diesen Code ändern. Überprüfen Sie die [Sicherheitsbeschränkungen](/docs/de/sandboxing#security-limitations), bevor Sie sich auf eine Sandbox als harte Kontrolle verlassen.

  Isolation ändert auch nicht, was an das Modell gesendet wird. Ihre Eingabeaufforderungen und die Dateien, die Claude liest, werden an die Anthropic API oder Ihren konfigurierten Anbieter mit oder ohne Sandbox übertragen. Siehe [Datennutzung](/docs/de/data-usage) für das, was Claude Code sendet und wie Sie es reduzieren können.
</Warning>

<h2 id="choose-an-approach">
  Wählen Sie einen Ansatz
</h2>

Ordnen Sie Ihr Ziel einer Zeile unten zu und lesen Sie dann den folgenden Detailabschnitt.

| Sie möchten                                                                                 | Beginnen Sie mit                                                                                                                                                    |
| :------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Genehmigungseingaben während der täglichen Arbeit auf Ihrem eigenen Computer reduzieren     | Das [Sandboxed Bash tool](/docs/de/sandboxing), aktiviert mit `/sandbox`                                                                                                 |
| Claude unbeaufsichtigt mit `--dangerously-skip-permissions` oder Auto-Modus arbeiten lassen | Der vorkonfigurierte [Dev Container](/docs/de/devcontainer), ein beliebiger Container oder VM, oder die [Sandbox Runtime](#sandbox-runtime)                              |
| MCP-Server und Hooks sowie Bash isolieren, ohne Docker                                      | Die Sandbox Runtime                                                                                                                                                 |
| An einem nicht vertrauenswürdigen Repository arbeiten                                       | Eine dedizierte virtuelle Maschine oder [Claude Code on the web](/docs/de/claude-code-on-the-web), wenn Sie ein Claude-Abonnement und ein verbundenes GitHub-Konto haben |
| Eine Sandbox-Umgebung über ein Team standardisieren                                         | Der vorkonfigurierte [Dev Container](/docs/de/devcontainer), kopiert in Ihr Repository                                                                                   |
| Claude Code von einem Gerät ohne lokales Setup verwenden                                    | [Claude Code on the web](/docs/de/claude-code-on-the-web), das ein Claude-Abonnement und ein verbundenes GitHub-Konto erfordert                                          |
| Isolation für jeden Entwickler in Ihrer Organisation erfordern                              | [Isolation über eine Organisation erzwingen](#enforce-isolation-across-an-organization)                                                                             |
| Auf einem nativen Windows-Host arbeiten                                                     | Ein Container oder VM, oder führen Sie die Bash-Sandbox in WSL2 aus                                                                                                 |

<h3 id="how-isolation-relates-to-permission-modes">
  Wie Isolation mit Berechtigungsmodi zusammenhängt
</h3>

[Berechtigungsmodi](/docs/de/permission-modes) entscheiden, ob ein Tool-Aufruf ausgeführt wird und ob Sie zuerst aufgefordert werden. Isolation beschränkt, was ein Befehl nach der Ausführung zugreifen kann. Die beiden arbeiten zusammen: Wenn ein Berechtigungsmodus Aktionen ohne Nachfrage ausführen lässt, begrenzt eine Isolierungsgrenze, was diese Aktionen erreichen können.

Wenn Sie `--dangerously-skip-permissions` übergeben, handelt Claude ohne vorherige Nachfrage; Sie werden nur aufgefordert für explizite [ask rules](/docs/de/permissions#manage-permissions), Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), MCP-Tools mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet, und Löschungen, die auf `/` oder Ihr Home-Verzeichnis abzielen. Ohne Eingabeaufforderungen, um Fehler zu erkennen, ist die Isolierungsgrenze, die Sie wählen, das, was Ihr System schützt. Führen Sie `--dangerously-skip-permissions`-Sitzungen immer in einem Container, einer VM oder der [Sandbox Runtime](#sandbox-runtime) aus, damit Datei-Tools, MCP-Server und Hooks auch innerhalb der Grenze liegen.

[Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) ersetzt die Eingabeaufforderung durch einen Klassifizierer, der Aktionen überprüft und solche blockiert, die über die Anfrage hinausgehen, auf nicht erkannte Infrastruktur abzielen oder von feindseligem Inhalt angetrieben zu sein scheinen, den Claude gelesen hat. Der Klassifizierer ist eine Kontrolle pro Aktion, keine Isolierungsgrenze, daher fügt eine Isolierungsgrenze immer noch Verteidigungstiefe für unbeaufsichtigte Läufe hinzu und ist nicht erforderlich, wie es für `--dangerously-skip-permissions` der Fall ist.

Das [Sandboxed Bash tool](#sandboxed-bash-tool) allein beschränkt nur Bash, daher ist es nicht ausreichend für vollständig unbeaufsichtigte Läufe in beiden Modi. Sie können Ansätze schichten: Das Ausführen des Sandboxed Bash tool in einem Container oder VM gibt Ihnen OS-Ebenen-Befehlsbeschränkungen zusätzlich zur äußeren Umgebungsgrenze. Für die Interaktion der Bash-Sandbox selbst mit Berechtigungsregeln und Modi siehe [Wie Sandboxing mit Berechtigungen und Berechtigungsmodi zusammenhängt](/docs/de/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Sandboxed Bash tool
</h2>

<Note>
  Diese Option unterstützt nicht natives Windows. Verwenden Sie auf Windows-Hosts WSL2 oder einen der Container- oder VM-Ansätze unten.
</Note>

Das Sandboxed Bash tool ist in Claude Code integriert. Es verwendet Betriebssystem-Primitive, um den Dateisystem- und Netzwerkzugriff jedes Bash-Befehls, den Claude ausführt, einzuschränken: Seatbelt, die integrierte macOS-Sandbox, und [bubblewrap](https://github.com/containers/bubblewrap) auf Linux und WSL2. Standardmäßig erlaubt es Schreibvorgänge im Arbeitsverzeichnis und fordert Sie auf, wenn ein Befehl zum ersten Mal eine neue Netzwerk-Domain benötigt.

Aktivieren Sie es mit dem `/sandbox`-Befehl. Der [Sandboxing](/docs/de/sandboxing)-Leitfaden behandelt die Genehmigungsmodi, die Standardgrenze und wie Sie sie erweitern oder einengen.

Die Sandbox pro Befehl deckt nicht alles ab, das in einer Sitzung ausgeführt wird:

* Andere [integrierte Tools](/docs/de/tools-reference) wie Read, Edit und WebFetch werden im Claude Code-Prozess ausgeführt und führen keinen beliebigen Code aus. [Berechtigungsregeln](/docs/de/permissions) für Pfad oder Domain kontrollieren sie stattdessen.
* [MCP](/docs/de/mcp)-Server und Hooks sind separate Prozesse, die unkontrolliert auf dem Host ausgeführt werden.

Um integrierte Tools, MCP-Server und Hooks alle hinter einer OS-Grenze zu platzieren, führen Sie den gesamten Claude Code-Prozess in der [Sandbox Runtime](#sandbox-runtime), dem [Dev Container](#dev-containers) oder einem [Custom Container](#custom-container) aus.

<h2 id="sandbox-runtime">
  Sandbox Runtime
</h2>

Das [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime)-Paket umhüllt einen gesamten Prozess mit der gleichen Seatbelt- oder bubblewrap-Isolation, die die integrierte Bash-Sandbox verwendet. Das Ausführen von Claude Code durch sie beschränkt jedes Tool, jeden Hook und jeden MCP-Server in der Sitzung, nicht nur Bash. Die Runtime ist eine Beta-Forschungsvorschau, und ihr Konfigurationsformat kann sich ändern, wenn sich das Paket weiterentwickelt.

Die Runtime verweigert standardmäßig alle Schreib- und Netzwerkzugriffe, daher konfigurieren Sie sie vor dem Starten von Claude Code durch sie. In `~/.srt-settings.json` oder einer Datei, die Sie mit `--settings` übergeben, erlauben Sie Schreibzugriff auf mindestens Ihr Projektverzeichnis und Claude Codes Konfigurationspfade `~/.claude` und `~/.claude.json`. Erlauben Sie die Netzwerk-Domains, die Ihre Sitzung benötigt, einschließlich `api.anthropic.com` oder des Endpunkts Ihres konfigurierten Anbieters. Siehe die Paket-[README](https://github.com/anthropic-experimental/sandbox-runtime) für das vollständige Konfigurationsschema.

Sobald die Einstellungsdatei vorhanden ist, starten Sie Claude Code mit `npx` und übergeben Sie `claude` als den zu umhüllenden Befehl:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code startet in der Sandbox mit den Dateisystem- und Netzwerkgrenzen, die Sie konfiguriert haben. Der gleiche Befehl funktioniert zum Sandboxing eigenständiger MCP-Server oder anderer Hilfsprozesse.

<h2 id="dev-containers">
  Dev Container
</h2>

Ein Dev Container führt Claude Code in einem Docker-Container aus, den VS Code oder ein kompatibler Editor verwaltet, mit Ihrem Projekt bereitgestellt. Sie können Ihren eigenen mit einem `.devcontainer/`-Verzeichnis in Ihrem Repository definieren.

Das claude-code-Repository veröffentlicht einen [Beispiel-Dev-Container](/docs/de/devcontainer) mit einer Standard-Deny-iptables-Firewall als Ausgangspunkt. Kopieren Sie ihn in Ihr Repository und passen Sie die Firewall-Allowlist, das Basis-Image und die angeheftete Claude Code-Version an Ihre Umgebung an. Da die Firewall nicht genehmigten Egress blockiert, unterstützt eine Konfiguration wie diese das Ausführen von Claude Code mit `--dangerously-skip-permissions` für unbeaufsichtigte Arbeit.

<h2 id="custom-container">
  Custom Container
</h2>

Sie können Claude Code in einem beliebigen Docker- oder OCI-Container-Image mit Ihren eigenen Netzwerkrichtlinien, bereitgestellten Volumes und seccomp-Profilen ausführen. Dies ist der häufigste Weg für Organisationen mit bestehender Container-Infrastruktur oder CI-Runnern.

Mehrere verwaltete Sandbox- und Remote-Ausführungsdienste können den Container für Sie hosten. Die gleiche Checkliste gilt wie für jeden Container, den Sie betreiben: Überprüfen Sie, was beschreibbar bereitgestellt ist, welche Anmeldedaten und Token darin erreichbar sind, und welche Netzwerk-Egress-Richtlinie erlaubt ist.

Sie können die integrierte Bash-Sandbox im Container schichten, um Befehlsbeschränkungen pro Befehl zu erhalten. Unprivilegierte Container benötigen die nested-sandbox-Einstellung, die in [Sandboxing-Fehlerbehebung](/docs/de/sandboxing#troubleshooting) beschrieben ist.

<h2 id="virtual-machine">
  Virtuelle Maschine
</h2>

Eine dedizierte virtuelle Maschine bietet die stärkste Trennung mit ihrem eigenen Kernel und in Cloud- oder microVM-Bereitstellungen ihrer eigenen virtualisierten Hardware. Optionen umfassen Cloud-Instanzen, lokale Hypervisoren und microVMs wie Firecracker.

Verwenden Sie diesen Ansatz, wenn Sie nicht vertrauenswürdigen Code evaluieren, wenn Ihre Sicherheitsrichtlinie Kernel-Ebenen-Trennung zwischen dem Agent und dem Host erfordert, oder wenn kein Host-Ebenen-Ansatz Ihre Compliance-Anforderungen erfüllt. Das [Sandboxes-Feature](https://docs.docker.com/ai/sandboxes/) von Docker Desktop bietet eine microVM mit ihrem eigenen Docker-Daemon und Workspace-Synchronisierung, die Claude Code auf Hosts ausführen kann, die bereits Docker Desktop haben.

<h2 id="claude-code-on-the-web">
  Claude Code on the web
</h2>

[Claude Code on the web](/docs/de/claude-code-on-the-web) führt jede Sitzung in einer isolierten, von Anthropic verwalteten virtuellen Maschine aus. Ein Netzwerk-Proxy erzwingt eine Standard-Allowlist, und ein separater Proxy hält Ihren GitHub-Token außerhalb der Sandbox, während er scoped Anmeldedaten für Repository-Zugriff darin ausstellt.

Verwenden Sie diesen Ansatz, wenn Sie vollständige VM-Isolation ohne Bereitstellung von Infrastruktur selbst möchten, oder wenn Sie Aufgaben von einem Gerät delegieren, das keine lokale Entwicklungsumgebung hat. Es erfordert ein Claude-Abonnement und ein verbundenes GitHub-Konto, und Sitzungen klonen Ihr Repository von GitHub. Siehe [Claude Code on the web](/docs/de/claude-code-on-the-web) für Planverfügbarkeit und GitHub-Authentifizierungsoptionen.

<h2 id="enforce-isolation-across-an-organization">
  Isolation über eine Organisation erzwingen
</h2>

Einzelne Entwickler können sich für jeden der oben genannten Ansätze anmelden. Was eine Organisation erzwingen kann und mit welchen Tools hängt vom Ansatz ab:

* **Integrierte Bash-Sandbox**: der einzige Ansatz, den Claude Code selbst erzwingt. Liefern Sie die `sandbox`-Einstellungsschlüssel durch [verwaltete Einstellungen](/docs/de/settings#settings-files), entweder als eine Datei, die von Ihrem MDM verwaltet wird, oder durch [servergesteuerte Einstellungen](/docs/de/server-managed-settings) auf Claude.ai. Siehe [Sandboxing mit verwalteten Einstellungen erzwingen](/docs/de/sandboxing#enforce-sandboxing-with-managed-settings) für die bereitzustellenden Schlüssel und wie Sie Entwickler davon abhalten, die Richtlinie zu erweitern.
* **Dev Container**: Committen Sie den [Beispiel-Dev-Container](/docs/de/devcontainer) in Ihre Repositories, um die Umgebung über ein Team zu standardisieren. Dies ist eher eine Konvention als eine Erzwingungsgrenze, da Claude Code keinen Container erfordert. Wenn Entwickler Claude Code nicht außerhalb davon ausführen sollten, erzwingen Sie dies mit den Gerätemanagement- oder Software-Allowlisting-Tools Ihrer Organisation.
* **Custom Container und VMs**: Verteilen Sie Claude Code über das genehmigte Image und verwenden Sie die Gerätemanagement- oder Software-Allowlisting-Tools Ihrer Organisation, um die Installation außerhalb davon zu verhindern.

<h2 id="see-also">
  Siehe auch
</h2>

Diese Seiten behandeln Konfiguration und Richtliniendetails für die oben genannten Ansätze.

* [Sandboxing](/docs/de/sandboxing): Konfigurieren Sie das integrierte Sandboxed Bash tool
* [Dev Container](/docs/de/devcontainer): Der vorkonfigurierte Docker-Entwicklungs-Container
* [Sicherheit](/docs/de/security): Das vollständige Claude Code-Sicherheitsmodell
* [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment): Isolierungsleitfaden für Agent SDK-Anwendungen
* [Einstellungen](/docs/de/settings#sandbox-settings): Alle Sandbox-Konfigurationsschlüssel, einschließlich verwalteter Einstellungsbereitstellung
