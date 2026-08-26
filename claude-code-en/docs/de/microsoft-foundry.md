> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code auf Microsoft Foundry

> Erfahren Sie, wie Sie Claude Code über Microsoft Foundry konfigurieren, einschließlich Setup, Konfiguration und Fehlerbehebung.

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

<ContactSalesCard surface="foundry" />

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Bevor Sie Claude Code mit Microsoft Foundry konfigurieren, stellen Sie sicher, dass Sie über Folgendes verfügen:

* Ein Azure-Abonnement mit Zugriff auf Microsoft Foundry
* RBAC-Berechtigungen zum Erstellen von Microsoft Foundry-Ressourcen und Bereitstellungen
* Azure CLI installiert und konfiguriert (optional - nur erforderlich, wenn Sie keinen anderen Mechanismus zum Abrufen von Anmeldedaten haben)

<Note>
  Wenn Sie Claude Code für mehrere Benutzer bereitstellen, [fixieren Sie Ihre Modellversionen](#4-pin-model-versions), bevor Sie die Bereitstellung durchführen.
</Note>

<h2 id="setup">
  Einrichtung
</h2>

<h3 id="1-provision-microsoft-foundry-resource">
  1. Microsoft Foundry-Ressource bereitstellen
</h3>

Erstellen Sie zunächst eine Claude-Ressource in Azure:

1. Navigieren Sie zum [Microsoft Foundry-Portal](https://ai.azure.com/)
2. Erstellen Sie eine neue Ressource und notieren Sie sich Ihren Ressourcennamen
3. Erstellen Sie Bereitstellungen für die Claude-Modelle und notieren Sie sich den Bereitstellungsnamen, den Sie jedem geben; Sie werden diese Namen in Schritt 4 als Modellvariablen festlegen:
   * Claude Opus
   * Claude Sonnet
   * Claude Haiku

<h3 id="2-configure-azure-credentials">
  2) Azure-Anmeldedaten konfigurieren
</h3>

Claude Code unterstützt drei Authentifizierungsmethoden für Microsoft Foundry. Wählen Sie die Methode, die Ihren Sicherheitsanforderungen am besten entspricht.

**Option A: API-Schlüssel-Authentifizierung**

1. Navigieren Sie zu Ihrer Ressource im Microsoft Foundry-Portal
2. Gehen Sie zum Abschnitt **Endpunkte und Schlüssel**
3. Kopieren Sie **API-Schlüssel**
4. Legen Sie die Umgebungsvariable fest und ersetzen Sie `your-azure-api-key` durch den Schlüssel, den Sie kopiert haben:

```bash theme={null}
export ANTHROPIC_FOUNDRY_API_KEY=your-azure-api-key
```

**Option B: Microsoft Entra ID-Authentifizierung**

Wenn weder `ANTHROPIC_FOUNDRY_API_KEY` noch `ANTHROPIC_FOUNDRY_AUTH_TOKEN` gesetzt ist, verwendet Claude Code automatisch die Azure SDK [Standard-Anmeldekette](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview).
Dies unterstützt eine Vielzahl von Methoden zur Authentifizierung lokaler und Remote-Workloads.

In lokalen Umgebungen können Sie häufig die Azure CLI verwenden:

```bash theme={null}
az login
```

**Option C: Bearer-Token-Authentifizierung**

Claude Code sendet den Wert von `ANTHROPIC_FOUNDRY_AUTH_TOKEN` bei jeder Anfrage als `Authorization: Bearer`-Header. Verwenden Sie diese Option, wenn ein anderer Prozess, z. B. eine Host-Anwendung oder ein Anmeldeskript, bereits ein Zugriffstoken für Sie erhalten hat. Erfordert Claude Code v2.1.203 oder später.

Legen Sie die Variable auf ein Bearer-Token fest, das Microsoft Entra ID für Ihre Ressource ausgestellt hat:

```bash theme={null}
export ANTHROPIC_FOUNDRY_AUTH_TOKEN=your-entra-access-token
```

`ANTHROPIC_FOUNDRY_AUTH_TOKEN` hat Vorrang vor `ANTHROPIC_FOUNDRY_API_KEY` und vor der Standard-Anmeldekette.

<Note>
  Bei Verwendung von Microsoft Foundry ist der `/logout`-Befehl nicht verfügbar, da die Authentifizierung über Azure-Anmeldedaten erfolgt.
</Note>

<h3 id="3-configure-claude-code">
  3. Claude Code konfigurieren
</h3>

Legen Sie die folgenden Umgebungsvariablen fest, um Microsoft Foundry zu aktivieren:

```bash theme={null}
# Microsoft Foundry-Integration aktivieren
export CLAUDE_CODE_USE_FOUNDRY=1

# Azure-Ressourcenname (ersetzen Sie {resource} durch Ihren Ressourcennamen)
export ANTHROPIC_FOUNDRY_RESOURCE={resource}
# Oder geben Sie die vollständige Basis-URL an:
# export ANTHROPIC_FOUNDRY_BASE_URL=https://{resource}.services.ai.azure.com/anthropic
```

<h3 id="4-pin-model-versions">
  4. Modellversionen fixieren
</h3>

<Warning>
  Fixieren Sie spezifische Modellversionen für jede Bereitstellung. Ohne Fixierung werden Modellaliase wie `sonnet` und `opus` zu Claude Codes integriertem Standard für Microsoft Foundry aufgelöst, der hinter der neuesten Version zurückbleiben kann und möglicherweise noch nicht in Ihrem Konto verfügbar ist. Microsoft Foundry hat keine Startup-Modellprüfung, daher schlagen Anfragen fehl, wenn der Standard nicht verfügbar ist. Wenn Sie Azure-Bereitstellungen erstellen, wählen Sie eine spezifische Modellversion anstelle von „automatisch auf die neueste Version aktualisieren".
</Warning>

Legen Sie die Modellvariablen so fest, dass sie den Bereitstellungsnamen entsprechen, die Sie in Schritt 1 erstellt haben.

Ohne `ANTHROPIC_DEFAULT_OPUS_MODEL` wird der `opus`-Alias auf Microsoft Foundry zu Opus 4.6 aufgelöst. Legen Sie ihn auf die Opus 4.8-ID fest, um das neueste Modell zu verwenden:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5'
```

Hintergrundaufgaben wie die Generierung von Sitzungstiteln verwenden das kleine/schnelle Modell, normalerweise ein Haiku-Klasse-Modell. Bei Microsoft Foundry setzt Claude Code dies standardmäßig auf das primäre Modell, da nicht jedes Konto eine Haiku-Bereitstellung hat. Um Haiku für Hintergrundaufgaben zu verwenden, legen Sie `ANTHROPIC_DEFAULT_HAIKU_MODEL` auf eine Haiku-Bereitstellung fest, die in Ihrem Konto verfügbar ist, wie oben gezeigt.

Aktuelle und ältere Modell-IDs finden Sie unter [Modellübersicht](https://platform.claude.com/docs/en/about-claude/models/overview). Siehe [Modellkonfiguration](/docs/de/model-config#pin-models-for-third-party-deployments) für die vollständige Liste der Umgebungsvariablen.

[Prompt Caching](/docs/de/prompt-caching) ist automatisch aktiviert. Um stattdessen eine 1-Stunden-Cache-TTL anstelle des 5-Minuten-Standards anzufordern, legen Sie die folgende Variable fest; Cache-Schreibvorgänge mit einer 1-Stunden-TTL werden mit einem höheren Satz abgerechnet:

```bash theme={null}
export ENABLE_PROMPT_CACHING_1H=1
```

<h3 id="5-run-claude-code">
  5. Claude Code ausführen
</h3>

Mit den gesetzten Umgebungsvariablen starten Sie Claude Code aus Ihrem Projektverzeichnis:

```bash theme={null}
claude
```

Claude Code liest `CLAUDE_CODE_USE_FOUNDRY` und die anderen Microsoft Foundry-Variablen aus der Umgebung und verbindet sich bei der ersten Eingabeaufforderung mit Ihrer Azure-Ressource. Im Gegensatz zu Amazon Bedrock und Google Cloud's Agent Platform hat Microsoft Foundry keinen interaktiven Setup-Assistenten, daher sind die Umgebungsvariablen in den Schritten 3 und 4 der einzige Konfigurationspfad.

Um Ihre Einrichtung zu überprüfen, führen Sie `/status` in Claude Code aus. Die API-Provider-Zeile zeigt `Microsoft Foundry` zusammen mit dem Ressourcennamen oder der Basis-URL, die Sie konfiguriert haben.

<h2 id="azure-rbac-configuration">
  Azure RBAC-Konfiguration
</h2>

Die Standardrollen `Azure AI User` und `Cognitive Services User` enthalten alle erforderlichen Berechtigungen zum Aufrufen von Claude-Modellen.

Für restriktivere Berechtigungen erstellen Sie eine benutzerdefinierte Rolle mit Folgendem:

```json theme={null}
{
  "permissions": [
    {
      "dataActions": [
        "Microsoft.CognitiveServices/accounts/providers/*"
      ]
    }
  ]
}
```

Weitere Informationen finden Sie in der [Microsoft Foundry RBAC-Dokumentation](https://learn.microsoft.com/en-us/azure/ai-foundry/concepts/rbac-azure-ai-foundry).

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Wenn Sie einen Fehler erhalten „Failed to get token from azureADTokenProvider: ChainedTokenCredential authentication failed":

* Konfigurieren Sie Entra ID in der Umgebung, oder legen Sie `ANTHROPIC_FOUNDRY_API_KEY` fest.

Wenn Anfragen mit wiederholten Verbindungsfehlern bei der ersten Eingabeaufforderung fehlschlagen:

* Überprüfen Sie, dass `ANTHROPIC_FOUNDRY_RESOURCE` auf Ihren tatsächlichen Ressourcennamen und nicht auf einen Platzhalter gesetzt ist. Claude Code erstellt die Endpunkt-URL aus diesem Wert, daher verweist ein falscher Name auf einen Host, der nicht vorhanden ist.

<h2 id="additional-resources">
  Zusätzliche Ressourcen
</h2>

* [Microsoft Foundry-Dokumentation](https://learn.microsoft.com/en-us/azure/ai-foundry/what-is-azure-ai-foundry)
* [Microsoft Foundry-Modelle](https://ai.azure.com/explore/models)
* [Microsoft Foundry-Preise](https://azure.microsoft.com/en-us/pricing/details/ai-foundry/)
