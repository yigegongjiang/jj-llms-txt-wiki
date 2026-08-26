> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sicherheit

> Erfahren Sie mehr über die Sicherheitsvorkehrungen von Claude Code und Best Practices für sichere Nutzung.

<h2 id="how-we-approach-security">
  Wie wir Sicherheit angehen
</h2>

<h3 id="security-foundation">
  Sicherheitsfundament
</h3>

Die Sicherheit Ihres Codes ist von größter Bedeutung. Claude Code ist mit Sicherheit im Kern entwickelt worden, gemäß Anthropics umfassendem Sicherheitsprogramm. Erfahren Sie mehr und greifen Sie auf Ressourcen zu (SOC 2 Type 2 Bericht, ISO 27001 Zertifikat usw.) im [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="permission-based-architecture">
  Berechtigungsbasierte Architektur
</h3>

Claude Code verwendet standardmäßig strikte Nur-Lesen-Berechtigungen. Wenn zusätzliche Aktionen erforderlich sind (Dateien bearbeiten, Tests ausführen, Befehle ausführen), fordert Claude Code explizite Genehmigung an. Benutzer kontrollieren, ob sie Aktionen einmalig genehmigen oder automatisch zulassen möchten.

Claude Code erfordert Genehmigung vor der Ausführung von Bash-Befehlen, die Ihr System ändern können. Ein integrierter Satz von [Nur-Lesen-Befehlen](/docs/de/permissions#read-only-commands) wie `ls`, `cat` und `git status` wird ohne Aufforderung ausgeführt. Dieser Ansatz ermöglicht es Benutzern und Organisationen, Berechtigungen direkt zu konfigurieren.

Für detaillierte Berechtigungskonfiguration siehe [Berechtigungen](/docs/de/permissions).

<h3 id="built-in-protections">
  Integrierte Schutzmaßnahmen
</h3>

Um Risiken in agentengestützten Systemen zu mindern:

* **Sandbox-Bash-Tool**: [Sandbox](/docs/de/sandboxing) Bash-Befehle mit Dateisystem- und Netzwerkisolation, wodurch Berechtigungsaufforderungen reduziert werden, während die Sicherheit gewährleistet bleibt. Aktivieren Sie mit `/sandbox`, um Grenzen zu definieren, in denen Claude Code autonom arbeiten kann
* **Arbeitsverzeichnis-Grenze**: Claude Code kann nur in den Ordner schreiben, in dem es gestartet wurde, und in dessen Unterordner, und kann Dateien in übergeordneten Verzeichnissen nicht ohne explizite Genehmigung ändern. Das Lesen von Pfaden außerhalb dieser Grenze mit den Tools Read, Grep und Glob ist nach einer Genehmigungsaufforderung möglich. Erweitern Sie die Grenze mit [zusätzlichen Verzeichnissen](/docs/de/permissions#working-directories), um die Aufforderung zu überspringen, oder beschränken Sie den breiteren Lesezugriff, der für Nur-Lesen-Bash-Befehle verfügbar ist, mit [Sandbox-`denyRead`-Regeln](/docs/de/sandboxing#filesystem-isolation), die nur gelten, wenn Sandboxing aktiviert ist
* **Minderung von Genehmigungsmüdigkeit**: Unterstützung für das Zulassen häufig verwendeter sicherer Befehle pro Benutzer, pro Codebasis oder pro Organisation
* **Accept Edits-Modus**: Genehmigt automatisch Dateibearbeitungen und einen festen Satz von Dateisystem-Bash-Befehlen wie `mkdir`, `touch`, `rm`, `mv`, `cp` und `sed` für Pfade im Arbeitsverzeichnis. Andere Bash-Befehle und Pfade außerhalb des Umfangs werden weiterhin angefordert

<h3 id="user-responsibility">
  Benutzerverantwortung
</h3>

Claude Code hat nur die Berechtigungen, die Sie ihm gewähren. Sie sind verantwortlich für die Überprüfung vorgeschlagener Code und Befehle auf Sicherheit vor der Genehmigung.

<h2 id="protect-against-prompt-injection">
  Schutz vor Prompt-Injection
</h2>

Prompt-Injection ist eine Technik, bei der ein Angreifer versucht, die Anweisungen eines KI-Assistenten durch das Einfügen bösartiger Texte zu überschreiben oder zu manipulieren. Claude Code enthält mehrere Schutzmaßnahmen gegen diese Angriffe:

<h3 id="core-protections">
  Kernschutzmaßnahmen
</h3>

* **Berechtigungssystem**: Sensible Operationen erfordern explizite Genehmigung
* **Kontextbewusste Analyse**: Erkennt potenziell schädliche Anweisungen durch Analyse der vollständigen Anfrage
* **Eingabebereinigung**: Verhindert Befehlsinjektionen durch Verarbeitung von Benutzereingaben
* **Genehmigung von Netzwerkbefehlen**: Befehle, die Inhalte aus dem Web abrufen, wie `curl` und `wget`, werden standardmäßig nicht automatisch genehmigt. Sie werden wie jeder andere nicht-schreibgeschützte Bash-Befehl behandelt, sodass Sie diese trotzdem genehmigen oder eine explizite Zulassungsregel wie `Bash(curl *)` hinzufügen können. Um sie vollständig zu blockieren, fügen Sie sie zu [`permissions.deny`](/docs/de/permissions#tool-specific-permission-rules) hinzu

<h3 id="privacy-safeguards">
  Datenschutzvorkehrungen
</h3>

Wir haben mehrere Schutzmaßnahmen implementiert, um Ihre Daten zu schützen, einschließlich:

* Begrenzte Aufbewahrungszeiträume für sensible Informationen (siehe [Privacy Center](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data), um mehr zu erfahren)
* Eingeschränkter Zugriff auf Benutzersitzungsdaten
* Benutzerkontrolle über Datenschulungspräferenzen. Verbraucherbenutzer können ihre [Datenschutzeinstellungen](https://claude.ai/settings/privacy) jederzeit ändern.

Für vollständige Details überprüfen Sie bitte unsere [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms) (für Team-, Enterprise- und API-Benutzer) oder [Consumer Terms](https://www.anthropic.com/legal/consumer-terms) (für Free-, Pro- und Max-Benutzer) und [Privacy Policy](https://www.anthropic.com/legal/privacy).

<h3 id="additional-safeguards">
  Zusätzliche Schutzmaßnahmen
</h3>

* **Genehmigung von Netzwerkanfragen**: Tools, die Netzwerkanfragen stellen, erfordern standardmäßig Benutzergenehmigung
* **Isolierte Kontextfenster**: Web Fetch verwendet ein separates Kontextfenster, um die Injection potenziell bösartiger Prompts zu vermeiden
* **Vertrauensüberprüfung**: Erste Codebasis-Ausführungen und neue MCP-Server erfordern Vertrauensüberprüfung
  * Hinweis: Vertrauensüberprüfung ist deaktiviert, wenn nicht-interaktiv mit dem `-p`-Flag ausgeführt wird
  * Hinweis: Wenn Sie Claude Code direkt in Ihrem Home-Verzeichnis starten, wird die Vertrauensannahme nur für die aktuelle Sitzung beibehalten und nicht auf die Festplatte geschrieben, sodass die Eingabeaufforderung bei jedem Start erneut angezeigt wird. Es gibt keine Einstellung, um sie beizubehalten. Starten Sie Claude Code stattdessen aus einem Projektunterverzeichnis, in dem die Vertrauensannahme pro Verzeichnis gespeichert wird
* **Erkennung von Befehlsinjektionen**: Verdächtige Bash-Befehle erfordern manuelle Genehmigung, auch wenn sie zuvor auf die Zulassungsliste gesetzt wurden
* **Fail-Closed-Matching**: Nicht übereinstimmende Befehle erfordern standardmäßig manuelle Genehmigung
* **Beschreibungen in natürlicher Sprache**: Komplexe Bash-Befehle enthalten Erklärungen zum Verständnis des Benutzers
* **Sichere Anmeldedatenspeicherung**: API-Schlüssel und Token werden im macOS Keychain gespeichert, wenn verfügbar, und durch Dateiberechtigungen unter Windows und Linux geschützt. Siehe [Credential Management](/docs/de/authentication#credential-management)

<Warning>
  **Windows WebDAV-Sicherheitsrisiko**: Wenn Sie Claude Code unter Windows ausführen, empfehlen wir, WebDAV nicht zu aktivieren oder Claude Code keinen Zugriff auf Pfade wie `\\*` zu gewähren, die WebDAV-Unterverzeichnisse enthalten können. [WebDAV wurde von Microsoft als veraltet eingestuft](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated) aufgrund von Sicherheitsrisiken. Das Aktivieren von WebDAV kann Claude Code ermöglichen, Netzwerkanfragen an Remote-Hosts auszulösen und das Berechtigungssystem zu umgehen.
</Warning>

**Best Practices für die Arbeit mit nicht vertrauenswürdigem Inhalt**:

1. Überprüfen Sie vorgeschlagene Befehle vor der Genehmigung
2. Vermeiden Sie es, nicht vertrauenswürdige Inhalte direkt an Claude zu pipen
3. Überprüfen Sie vorgeschlagene Änderungen an kritischen Dateien
4. Verwenden Sie virtuelle Maschinen (VMs), um Skripte auszuführen und Tool-Aufrufe zu tätigen, besonders bei der Interaktion mit externen Webdiensten
5. Melden Sie verdächtiges Verhalten mit `/feedback`

<Warning>
  Während diese Schutzmaßnahmen das Risiko erheblich reduzieren, ist kein System
  vollständig immun gegen alle Angriffe. Halten Sie immer gute Sicherheitspraktiken
  bei der Arbeit mit einem KI-Tool ein.
</Warning>

<h2 id="mcp-security">
  MCP-Sicherheit
</h2>

Claude Code ermöglicht es Benutzern, Model Context Protocol (MCP)-Server zu konfigurieren. Die Liste der zulässigen MCP-Server wird in Ihrem Quellcode konfiguriert, als Teil der Claude Code-Einstellungen, die Ingenieure in die Versionskontrolle einchecken.

Wir ermutigen Sie, entweder Ihre eigenen MCP-Server zu schreiben oder MCP-Server von Anbietern zu verwenden, denen Sie vertrauen. Sie können Claude Code-Berechtigungen für MCP-Server konfigurieren. Anthropic überprüft Konnektoren anhand seiner [Auflistungskriterien](https://claude.com/docs/connectors/building/review-criteria), bevor sie zum [Anthropic-Verzeichnis](https://claude.ai/directory) hinzugefügt werden, führt jedoch keine Sicherheitsprüfung durch und verwaltet keinen MCP-Server.

<h2 id="ide-security">
  IDE-Sicherheit
</h2>

Siehe [VS Code-Sicherheit und Datenschutz](/docs/de/vs-code#security-and-privacy) für weitere Informationen zum Ausführen von Claude Code in einer IDE.

<h2 id="cloud-execution-security">
  Cloud-Ausführungssicherheit
</h2>

Bei Verwendung von [Claude Code im Web](/docs/de/claude-code-on-the-web) sind zusätzliche Sicherheitskontrollen vorhanden:

* **Isolierte virtuelle Maschinen**: Jede Cloud-Sitzung wird in einer isolierten, von Anthropic verwalteten VM ausgeführt
* **Netzwerkzugriffskontrollen**: Der Netzwerkzugriff ist standardmäßig begrenzt und kann so konfiguriert werden, dass er deaktiviert ist oder nur bestimmte Domänen zulässt
* **Anmeldedatenschutz**: Die Authentifizierung wird über einen sicheren Proxy durchgeführt, der einen scoped Credential in der Sandbox verwendet, der dann in Ihr tatsächliches GitHub-Authentifizierungstoken übersetzt wird
* **Branch-Einschränkungen**: Git-Push-Operationen sind auf den aktuellen Arbeitsbranch beschränkt
* **Audit-Protokollierung**: Alle Operationen in Cloud-Umgebungen werden zu Compliance- und Audit-Zwecken protokolliert
* **Automatische Bereinigung**: Cloud-Umgebungen werden nach Abschluss der Sitzung automatisch beendet

Weitere Details zur Cloud-Ausführung finden Sie unter [Claude Code im Web](/docs/de/claude-code-on-the-web).

[Remote Control](/docs/de/remote-control)-Sitzungen funktionieren anders: Die Weboberfläche verbindet sich mit einem Claude Code-Prozess, der auf Ihrem lokalen Computer ausgeführt wird. Alle Code-Ausführungen und Dateizugriffe bleiben lokal, und der Sitzungsdatenverkehr wird über die Anthropic API über TLS übertragen. Während der Verbindung wird das Sitzungstranskript auf Anthropic-Servern gespeichert, um die Konversation über Geräte hinweg zu synchronisieren, wie in [Verbindung und Sicherheit](/docs/de/remote-control#connection-and-security) beschrieben. Es sind keine Cloud-VMs oder Sandboxing beteiligt. Die Verbindung verwendet mehrere kurzlebige, eng begrenzte Anmeldedaten, die jeweils auf einen bestimmten Zweck beschränkt sind und unabhängig ablaufen, um den Blast-Radius eines einzelnen kompromittierten Credentials zu begrenzen.

<h2 id="security-best-practices">
  Best Practices für Sicherheit
</h2>

<h3 id="working-with-sensitive-code">
  Arbeiten mit sensiblem Code
</h3>

* Überprüfen Sie alle vorgeschlagenen Änderungen vor der Genehmigung
* Verwenden Sie projektspezifische Berechtigungseinstellungen für sensible Repositories
* Erwägen Sie die Verwendung von [Dev Containern](/docs/de/devcontainer) für zusätzliche Isolierung
* Überprüfen Sie regelmäßig Ihre Berechtigungseinstellungen mit `/permissions`

<h3 id="team-security">
  Team-Sicherheit
</h3>

* Verwenden Sie [verwaltete Einstellungen](/docs/de/settings#settings-files), um organisatorische Standards durchzusetzen
* Teilen Sie genehmigte Berechtigungskonfigurationen über Versionskontrolle
* Schulen Sie Teammitglieder in Best Practices für Sicherheit
* Überwachen Sie die Claude Code-Nutzung durch [OpenTelemetry-Metriken](/docs/de/monitoring-usage)
* Überprüfen oder blockieren Sie Einstellungsänderungen während Sitzungen mit [`ConfigChange`-Hooks](/docs/de/hooks#configchange)

<h3 id="reporting-security-issues">
  Meldung von Sicherheitsproblemen
</h3>

Wenn Sie eine Sicherheitslücke in Claude Code entdecken:

1. Offenbaren Sie sie nicht öffentlich
2. Melden Sie sie über unser [HackerOne-Programm](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new)
3. Fügen Sie detaillierte Reproduktionsschritte ein
4. Geben Sie uns Zeit, das Problem zu beheben, bevor Sie es öffentlich offenbaren

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Security guidance plugin](/docs/de/security-guidance): Claude kann Sicherheitslücken in seinen eigenen Code-Änderungen während der Sitzung überprüfen und beheben
* [Sandbox-Umgebungen](/docs/de/sandbox-environments): Vergleichen Sie Isolierungsansätze und wählen Sie einen für Ihr Bedrohungsmodell
* [Sandboxing](/docs/de/sandboxing): Dateisystem- und Netzwerkisolation für Bash-Befehle
* [Berechtigungen](/docs/de/permissions): Konfigurieren Sie Berechtigungen und Zugriffskontrolle
* [Nutzungsüberwachung](/docs/de/monitoring-usage): Verfolgen und überprüfen Sie Claude Code-Aktivität
* [Entwicklungscontainer](/docs/de/devcontainer): Sichere, isolierte Umgebungen
* [Anthropic Trust Center](https://trust.anthropic.com): Sicherheitszertifizierungen und Compliance
