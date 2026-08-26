> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Übersicht zur Enterprise-Bereitstellung

> Erfahren Sie, wie Claude Code mit verschiedenen Drittanbieterdiensten und Infrastrukturen integriert werden kann, um Enterprise-Bereitstellungsanforderungen zu erfüllen.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

Organisationen können Claude Code direkt über Anthropic oder über einen Cloud-Anbieter bereitstellen. Diese Seite hilft Ihnen, die richtige Konfiguration auszuwählen.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  Bereitstellungsoptionen vergleichen
</h2>

Für die meisten Organisationen bieten Claude for Teams oder Claude for Enterprise die beste Erfahrung. Teammitglieder erhalten Zugriff auf sowohl Claude Code als auch Claude im Web mit einem einzigen Abonnement, zentralisierte Abrechnung und ohne erforderliche Infrastruktureinrichtung.

**Claude for Teams** ist Self-Service und umfasst Zusammenarbeitsfunktionen, Admin-Tools und Abrechnungsverwaltung. Am besten für kleinere Teams, die schnell starten möchten.

**Claude for Enterprise** fügt SSO und Domain-Erfassung, rollenbasierte Berechtigungen, Compliance-API-Zugriff und verwaltete Richtlinieneinstellungen für die Bereitstellung von organisationsweiten Claude Code-Konfigurationen hinzu. Am besten für größere Organisationen mit Sicherheits- und Compliance-Anforderungen.

Erfahren Sie mehr über [Team-Pläne](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) und [Enterprise-Pläne](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

Wenn Ihre Organisation spezifische Infrastrukturanforderungen hat, vergleichen Sie die folgenden Optionen:

<table>
  <thead>
    <tr>
      <th>Funktion</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform, ehemals Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>Am besten für</td>
      <td>Die meisten Organisationen (empfohlen)</td>
      <td>Einzelne Entwickler</td>
      <td>AWS-native Bereitstellungen</td>
      <td>AWS Marketplace-Abrechnung mit Claude API-Funktionen</td>
      <td>GCP-native Bereitstellungen</td>
      <td>Azure-native Bereitstellungen</td>
    </tr>

    <tr>
      <td>Abrechnung</td>
      <td><strong>Teams:</strong> 150 USD/Platz (Premium) mit PAYG verfügbar<br /><strong>Enterprise:</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">Kontaktieren Sie den Vertrieb</a></td>
      <td>PAYG</td>
      <td>PAYG über AWS</td>
      <td>PAYG über AWS Marketplace</td>
      <td>PAYG über GCP</td>
      <td>PAYG über Azure</td>
    </tr>

    <tr>
      <td>Regionen</td>
      <td>Unterstützte [Länder](https://www.anthropic.com/supported-countries)</td>
      <td>Unterstützte [Länder](https://www.anthropic.com/supported-countries)</td>
      <td>Mehrere AWS [Regionen](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html)</td>
      <td>Mehrere AWS-Regionen</td>
      <td>Mehrere GCP [Regionen](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)</td>
      <td>Mehrere Azure [Regionen](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/)</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>Standardmäßig aktiviert</td>
      <td>Standardmäßig aktiviert</td>
      <td>Standardmäßig aktiviert</td>
      <td>Standardmäßig aktiviert</td>
      <td>Standardmäßig aktiviert</td>
      <td>Standardmäßig aktiviert</td>
    </tr>

    <tr>
      <td>Authentifizierung</td>
      <td>Claude.ai SSO oder E-Mail</td>
      <td>API-Schlüssel</td>
      <td>API-Schlüssel oder AWS-Anmeldedaten</td>
      <td>API-Schlüssel oder AWS-Anmeldedaten</td>
      <td>GCP-Anmeldedaten</td>
      <td>API-Schlüssel oder Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>Kostenverfolgung</td>
      <td>Nutzungs-Dashboard</td>
      <td>Nutzungs-Dashboard</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>GCP Billing</td>
      <td>Azure Cost Management</td>
    </tr>

    <tr>
      <td>Umfasst Claude im Web</td>
      <td>Ja</td>
      <td>Nein</td>
      <td>Nein</td>
      <td>Nein</td>
      <td>Nein</td>
      <td>Nein</td>
    </tr>

    <tr>
      <td>Enterprise-Funktionen</td>
      <td>Teamverwaltung, SSO, Nutzungsüberwachung</td>
      <td>Keine</td>
      <td>IAM-Richtlinien, CloudTrail</td>
      <td>IAM-Richtlinien, CloudTrail</td>
      <td>IAM-Rollen, Cloud Audit Logs</td>
      <td>RBAC-Richtlinien, Azure Monitor</td>
    </tr>
  </tbody>
</table>

Für eine Funktion-für-Funktion-Aufschlüsselung der verfügbaren Optionen siehe [Funktionsverfügbarkeit](/docs/de/feature-availability).

Wählen Sie eine Bereitstellungsoption aus, um Setupanweisungen anzuzeigen:

* [Claude for Teams oder Enterprise](/docs/de/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/de/authentication#claude-console-authentication)
* [Claude Apps Gateway](/docs/de/claude-apps-gateway), ein selbst gehostetes Gateway, das IdP-Anmeldung vor Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry oder der Anthropic API hinzufügt
* [Amazon Bedrock](/docs/de/amazon-bedrock)
* [Claude Platform on AWS](/docs/de/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/de/google-vertex-ai)
* [Microsoft Foundry](/docs/de/microsoft-foundry)

Für Amazon Bedrock und Google Vertex AI können Sie auch `claude` ausführen und **3rd-party platform** bei der Anmeldeeingabeaufforderung auswählen, um einen interaktiven Setup-Assistenten zu starten.

<h2 id="configure-proxies-and-gateways">
  Proxys und Gateways konfigurieren
</h2>

Die meisten Organisationen können einen Cloud-Anbieter direkt ohne zusätzliche Konfiguration nutzen. Möglicherweise müssen Sie jedoch einen Unternehmens-Proxy oder LLM-Gateway konfigurieren, wenn Ihre Organisation spezifische Netzwerk- oder Verwaltungsanforderungen hat. Dies sind unterschiedliche Konfigurationen, die zusammen verwendet werden können:

* **Unternehmens-Proxy**: Leitet Datenverkehr über einen HTTP/HTTPS-Proxy weiter. Verwenden Sie dies, wenn Ihre Organisation verlangt, dass der gesamte ausgehende Datenverkehr einen Proxy-Server für Sicherheitsüberwachung, Compliance oder Netzwerkrichtliniendurchsetzung durchläuft. Konfigurieren Sie mit den Umgebungsvariablen `HTTPS_PROXY` oder `HTTP_PROXY`. Erfahren Sie mehr in [Enterprise-Netzwerkkonfiguration](/docs/de/network-config).
* **LLM-Gateway**: Ein Dienst, der sich zwischen Claude Code und dem Cloud-Anbieter befindet, um Authentifizierung und Routing zu verwalten. Verwenden Sie dies, wenn Sie eine zentralisierte Nutzungsverfolgung über Teams, benutzerdefinierte Ratenbegrenzung oder Budgets oder zentralisierte Authentifizierungsverwaltung benötigen. Konfigurieren Sie mit den Umgebungsvariablen `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL` oder `ANTHROPIC_FOUNDRY_BASE_URL`. Erfahren Sie mehr in [LLM-Gateways](/docs/de/llm-gateway).

Die folgenden Beispiele zeigen die Umgebungsvariablen, die in Ihrer Shell oder Shell-Profildatei (`.bashrc`, `.zshrc`) gesetzt werden sollen. Siehe [Einstellungen](/docs/de/settings) für andere Konfigurationsmethoden.

<h3 id="amazon-bedrock">
  Amazon Bedrock
</h3>

<Tabs>
  <Tab title="Unternehmens-Proxy">
    Leiten Sie Amazon Bedrock-Datenverkehr über Ihren Unternehmens-Proxy weiter, indem Sie die folgenden [Umgebungsvariablen](/docs/de/env-vars) setzen:

    ```bash theme={null}
    # Enable Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1
    export AWS_REGION=us-east-1

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM-Gateway">
    Leiten Sie Amazon Bedrock-Datenverkehr über Ihr LLM-Gateway weiter, indem Sie die folgenden [Umgebungsvariablen](/docs/de/env-vars) setzen:

    ```bash theme={null}
    # Enable Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1

    # Configure LLM gateway
    export ANTHROPIC_BEDROCK_BASE_URL='https://your-llm-gateway.com/bedrock'
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1  # If gateway handles AWS auth
    ```
  </Tab>
</Tabs>

<h3 id="microsoft-foundry">
  Microsoft Foundry
</h3>

<Tabs>
  <Tab title="Unternehmens-Proxy">
    Leiten Sie Microsoft Foundry-Datenverkehr über Ihren Unternehmens-Proxy weiter, indem Sie die folgenden [Umgebungsvariablen](/docs/de/env-vars) setzen:

    ```bash theme={null}
    # Enable Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1
    export ANTHROPIC_FOUNDRY_RESOURCE=your-resource
    export ANTHROPIC_FOUNDRY_API_KEY=your-api-key  # Or omit for Entra ID auth

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM-Gateway">
    Leiten Sie Microsoft Foundry-Datenverkehr über Ihr LLM-Gateway weiter, indem Sie die folgenden [Umgebungsvariablen](/docs/de/env-vars) setzen:

    ```bash theme={null}
    # Enable Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1

    # Configure LLM gateway
    export ANTHROPIC_FOUNDRY_BASE_URL='https://your-llm-gateway.com'
    export ANTHROPIC_FOUNDRY_API_KEY=your-gateway-key  # Sent as x-api-key
    ```
  </Tab>
</Tabs>

<h3 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h3>

<Tabs>
  <Tab title="Unternehmens-Proxy">
    Leiten Sie Google Cloud's Agent Platform-Datenverkehr über Ihren Unternehmens-Proxy weiter, indem Sie die folgenden [Umgebungsvariablen](/docs/de/env-vars) setzen:

    ```bash theme={null}
    # Enable Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM-Gateway">
    Leiten Sie Google Cloud's Agent Platform-Datenverkehr über Ihr LLM-Gateway weiter, indem Sie die folgenden [Umgebungsvariablen](/docs/de/env-vars) setzen:

    ```bash theme={null}
    # Enable Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1

    # Configure LLM gateway
    export ANTHROPIC_VERTEX_BASE_URL='https://your-llm-gateway.com/vertex'
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1  # If gateway handles GCP auth
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>
</Tabs>

<Tip>
  Verwenden Sie `/status` in Claude Code, um zu überprüfen, ob Ihre Proxy- und Gateway-Konfiguration korrekt angewendet wird. Beispielsweise enthält die Ausgabe mit der obigen Bedrock-Gateway-Konfiguration Zeilen wie:

  ```
  API provider: Amazon Bedrock
  Bedrock base URL: https://your-llm-gateway.com/bedrock
  AWS region: us-east-1
  AWS auth skipped
  ```

  Wenn Sie einen Unternehmens-Proxy konfiguriert haben, zeigt `/status` auch eine `Proxy`-Zeile mit Ihrer Proxy-URL an.
</Tip>

<h2 id="best-practices-for-organizations">
  Best Practices für Organisationen
</h2>

<h3 id="invest-in-documentation-and-memory">
  Investieren Sie in Dokumentation und Memory
</h3>

Wir empfehlen dringend, in Dokumentation zu investieren, damit Claude Code Ihre Codebasis versteht. Organisationen können CLAUDE.md-Dateien auf mehreren Ebenen bereitstellen:

* **Organisationsweit**: Bereitstellen in Systemverzeichnissen wie `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS), `/etc/claude-code/CLAUDE.md` (Linux und WSL) oder `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) für unternehmensweite Standards
* **Repository-Ebene**: Erstellen Sie `CLAUDE.md`-Dateien in Repository-Wurzeln mit Projektarchitektur, Build-Befehlen und Beitragsleitlinien. Checken Sie diese in die Versionskontrolle ein, damit alle Benutzer davon profitieren

Erfahren Sie mehr in [Memory und CLAUDE.md-Dateien](/docs/de/memory).

<h3 id="simplify-deployment">
  Vereinfachen Sie die Bereitstellung
</h3>

Wenn Sie eine benutzerdefinierte Entwicklungsumgebung haben, stellen wir fest, dass die Schaffung einer „Ein-Klick"-Möglichkeit zur Installation von Claude Code der Schlüssel zum Wachstum der Akzeptanz in einer Organisation ist.

<h3 id="start-with-guided-usage">
  Beginnen Sie mit gesteuerter Nutzung
</h3>

Ermutigen Sie neue Benutzer, Claude Code für Codebasis-Fragen oder bei kleineren Fehlerbehebungen oder Funktionsanfragen zu versuchen. Bitten Sie Claude Code, einen Plan zu erstellen. Überprüfen Sie Claudes Vorschläge und geben Sie Feedback, wenn es nicht stimmt. Mit der Zeit, wenn Benutzer dieses neue Paradigma besser verstehen, werden sie effektiver darin, Claude Code agentischer laufen zu lassen.

<h3 id="pin-model-versions-for-cloud-providers">
  Pinnen Sie Modellversionen für Cloud-Anbieter
</h3>

Wenn Sie über [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) oder [Claude Platform on AWS](/docs/de/claude-platform-on-aws) bereitstellen, pinnen Sie spezifische Modellversionen mit `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` und `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Ohne Pinning werden Modellaliase zu Claude Codes integriertem Standard für diesen Anbieter aufgelöst, was hinter der neuesten Version zurückbleiben kann und möglicherweise noch nicht in Ihrem Konto aktiviert ist. Pinning ermöglicht es Ihnen, zu kontrollieren, wann Ihre Benutzer zu einem neuen Modell wechseln. Siehe [Modellkonfiguration](/docs/de/model-config#pin-models-for-third-party-deployments) für Details zu dem, was jeder Anbieter tut, wenn die Standardversion nicht verfügbar ist.

<h3 id="configure-security-policies">
  Konfigurieren Sie Sicherheitsrichtlinien
</h3>

Sicherheitsteams können verwaltete Berechtigungen für das konfigurieren, was Claude Code darf und nicht darf, was nicht durch lokale Konfiguration überschrieben werden kann. [Erfahren Sie mehr](/docs/de/security).

<h3 id="leverage-mcp-for-integrations">
  Nutzen Sie MCP für Integrationen
</h3>

MCP ist eine großartige Möglichkeit, Claude Code mehr Informationen zu geben, z. B. die Verbindung mit Ticketverwaltungssystemen oder Fehlerprotokollen. Wir empfehlen, dass ein zentrales Team MCP-Server konfiguriert und eine `.mcp.json`-Konfiguration in die Codebasis eincheckt, damit alle Benutzer davon profitieren. [Erfahren Sie mehr](/docs/de/mcp).

Bei Anthropic vertrauen wir Claude Code, um die Entwicklung in jeder Anthropic-Codebasis zu unterstützen. Wir hoffen, dass Sie Claude Code genauso gerne verwenden wie wir.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Nachdem Sie eine Bereitstellungsoption ausgewählt und den Zugriff für Ihr Team konfiguriert haben:

1. **Rollout für Ihr Team**: Teilen Sie Installationsanweisungen mit und lassen Sie Teammitglieder [Claude Code installieren](/docs/de/setup) und sich mit ihren Anmeldedaten authentifizieren.
2. **Richten Sie gemeinsame Konfiguration ein**: Erstellen Sie eine [CLAUDE.md-Datei](/docs/de/memory) in Ihren Repositories, um Claude Code dabei zu helfen, Ihre Codebasis und Codierungsstandards zu verstehen.
3. **Konfigurieren Sie Berechtigungen**: Überprüfen Sie [Sicherheitseinstellungen](/docs/de/security), um zu definieren, was Claude Code in Ihrer Umgebung darf und nicht darf.
