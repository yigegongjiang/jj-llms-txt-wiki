> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code auf Google Clouds Agent Platform

> Erfahren Sie, wie Sie Claude Code über Google Clouds Agent Platform konfigurieren, ehemals Vertex AI, einschließlich Setup, IAM-Konfiguration und Fehlerbehebung.

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

<ContactSalesCard surface="vertex" />

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Bevor Sie Claude Code mit Google Cloud's Agent Platform, ehemals Vertex AI, konfigurieren, stellen Sie sicher, dass Sie über Folgendes verfügen:

* Ein Google Cloud Platform (GCP)-Konto mit aktivierter Abrechnung
* Ein GCP-Projekt mit aktivierter Google Cloud's Agent Platform API
* Zugriff auf gewünschte Claude-Modelle (z. B. Claude Sonnet 4.6)
* Google Cloud SDK (`gcloud`) installiert und konfiguriert
* Kontingent im gewünschten GCP-Bereich zugewiesen

Um sich mit Ihren eigenen Google Cloud's Agent Platform-Anmeldedaten anzumelden, folgen Sie [Anmelden mit Google Cloud's Agent Platform](#sign-in-with-agent-platform) unten. Um Claude Code in einem Team bereitzustellen, verwenden Sie die Schritte zum [manuellen Setup](#set-up-manually) und [fixieren Sie Ihre Modellversionen](#5-pin-model-versions), bevor Sie ausrollen.

<h2 id="sign-in-with-agent-platform">
  Anmelden mit Agent Platform
</h2>

Wenn Sie Google Cloud-Anmeldedaten haben und Claude Code über Google Cloud's Agent Platform verwenden möchten, führt Sie der Anmeldeasistent durch den Prozess. Sie führen die GCP-seitigen Voraussetzungen einmal pro Projekt durch; der Assistent kümmert sich um die Claude Code-Seite.

<Steps>
  <Step title="Aktivieren Sie Claude-Modelle in Ihrem GCP-Projekt">
    [Aktivieren Sie Google Cloud's Agent Platform API](#1-enable-agent-platform-api) für Ihr Projekt, und fordern Sie dann Zugriff auf die Claude-Modelle an, die Sie im [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) möchten. Siehe [IAM-Konfiguration](#iam-configuration) für die Berechtigungen, die Ihr Konto benötigt.
  </Step>

  <Step title="Starten Sie Claude Code und wählen Sie Google Cloud's Agent Platform">
    Führen Sie `claude` aus. Wählen Sie bei der Anmeldeeingabeaufforderung **3rd-party platform** und dann **Google Vertex AI**, das Label, das die Anmeldeeingabeaufforderung immer noch für Google Cloud's Agent Platform verwendet. Wenn Sie bereits angemeldet sind, führen Sie `/login` aus, um dasselbe Menü zu öffnen.
  </Step>

  <Step title="Folgen Sie den Eingabeaufforderungen des Assistenten">
    Wählen Sie, wie Sie sich bei Google Cloud authentifizieren: Application Default Credentials von `gcloud`, eine Service-Account-Schlüsseldatei oder Anmeldedaten, die bereits in Ihrer Umgebung vorhanden sind. Der Assistent erkennt Ihr Projekt und Ihre Region, überprüft, welche Claude-Modelle Ihr Projekt aufrufen kann, und ermöglicht es Ihnen, diese zu fixieren. Das Ergebnis wird im `env`-Block Ihrer [Benutzereinstellungsdatei](/docs/de/settings) gespeichert, sodass Sie Umgebungsvariablen nicht selbst exportieren müssen.
  </Step>
</Steps>

Nachdem Sie sich angemeldet haben, führen Sie `/setup-vertex` jederzeit aus, um den Assistenten erneut zu öffnen und Ihre Anmeldedaten, Ihr Projekt, Ihre Region oder Ihre Modellpins zu ändern. Der Modellpin-Schritt beginnt mit Ihren aktuell fixierten Modellen. Der Assistent schreibt in `~/.claude/settings.json` oder in `$CLAUDE_CONFIG_DIR/settings.json`, wenn [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars#variables) gesetzt ist.

<h2 id="region-configuration">
  Regionskonfiguration
</h2>

Claude Code unterstützt Google Cloud's Agent Platform [globale](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), Multi-Region- und regionale Endpunkte. Legen Sie `CLOUD_ML_REGION` auf `global`, einen Multi-Region-Standort wie `eu` oder `us` oder eine bestimmte Region wie `us-east5` fest. Claude Code wählt den korrekten Google Cloud's Agent Platform-Hostnamen für jedes Formular aus, einschließlich der Hosts `aiplatform.eu.rep.googleapis.com` und `aiplatform.us.rep.googleapis.com` für Multi-Region-Standorte.

<Note>
  Google Cloud's Agent Platform unterstützt möglicherweise die Claude Code-Standardmodelle nicht auf jedem Endpunkttyp. Die Modellverfügbarkeit variiert je nach [spezifischen Regionen](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), Multi-Region-Standorten und [globalen Endpunkten](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models). Möglicherweise müssen Sie zu einem unterstützten Standort wechseln oder ein unterstütztes Modell angeben.
</Note>

<h2 id="set-up-manually">
  Manuelles Setup
</h2>

Um Google Cloud's Agent Platform über Umgebungsvariablen statt über den Assistenten zu konfigurieren, z. B. in CI oder einem skriptgesteuerten Enterprise-Rollout, folgen Sie den folgenden Schritten.

<h3 id="1-enable-agent-platform-api">
  1. Aktivieren Sie die Agent Platform API
</h3>

Aktivieren Sie die Agent Platform API von Google Cloud in Ihrem GCP-Projekt. Ersetzen Sie `YOUR-PROJECT-ID` hier und im Konfigurationsschritt unten durch Ihre GCP-Projekt-ID:

```bash theme={null}
# Legen Sie Ihre Projekt-ID fest
gcloud config set project YOUR-PROJECT-ID

# Aktivieren Sie die Agent Platform API
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. Fordern Sie Modellzugriff an
</h3>

Fordern Sie Zugriff auf Claude-Modelle in Google Cloud's Agent Platform an:

1. Navigieren Sie zum [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. Suchen Sie nach „Claude"-Modellen
3. Fordern Sie Zugriff auf gewünschte Claude-Modelle an (z. B. Claude Sonnet 4.6)
4. Warten Sie auf Genehmigung (kann 24–48 Stunden dauern)

<h3 id="3-configure-gcp-credentials">
  3) Konfigurieren Sie GCP-Anmeldedaten
</h3>

Claude Code verwendet die standardmäßige Google Cloud-Authentifizierung.

Weitere Informationen finden Sie in der [Google Cloud-Authentifizierungsdokumentation](https://cloud.google.com/docs/authentication).

Claude Code unterstützt [X.509-zertifikatbasierte Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) über die gleiche Application Default Credentials-Kette. Legen Sie `GOOGLE_APPLICATION_CREDENTIALS` auf den Pfad Ihrer Anmeldedaten-Konfigurationsdatei fest.

<Note>
  Claude Code adressiert Google Cloud's Agent Platform-Anfragen an das Projekt in `ANTHROPIC_VERTEX_PROJECT_ID`, auch wenn `GCLOUD_PROJECT`, `GOOGLE_CLOUD_PROJECT` oder die Anmeldedatei, auf die `GOOGLE_APPLICATION_CREDENTIALS` verweist, ein anderes Projekt trägt.
</Note>

<h4 id="advanced-credential-configuration">
  Erweiterte Anmeldedaten-Konfiguration
</h4>

Claude Code unterstützt die automatische Aktualisierung von Anmeldedaten für GCP über die Einstellung `gcpAuthRefresh`. Fügen Sie sie zu Ihrer Claude Code [Einstellungsdatei](/docs/de/settings) hinzu, z. B. `~/.claude/settings.json`. Wenn Claude Code erkennt, dass Ihre GCP-Anmeldedaten abgelaufen sind oder nicht geladen werden können, führt es den konfigurierten Befehl aus, um neue Anmeldedaten zu erhalten, bevor die Anfrage erneut versucht wird.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

Vor dem Ausführen des Befehls fordert Claude Code ein Zugriffstoken mit Ihren aktuellen Anmeldedaten an, um zu bestätigen, dass sie tatsächlich abgelaufen sind, und überspringt den Befehl, wenn sie noch funktionieren.

Wenn die Überprüfung nicht innerhalb von fünf Sekunden abgeschlossen wird, überspringt Claude Code auch den Befehl und führt ihn nur aus, nachdem eine Anfrage mit einem Anmeldedatenfehler fehlschlägt. Vor v2.1.261 zählte eine Überprüfung, die abgelaufen war, als abgelaufene Anmeldedaten, sodass der Befehl Ihren Browser beim Start öffnen konnte, obwohl Ihre Anmeldedaten noch gültig waren.

Claude Code zeigt Ihnen die Ausgabe des Befehls an, kann aber keine interaktive Eingabe an den Befehl senden. Dies funktioniert gut für browserbasierte Authentifizierungsabläufe, bei denen die CLI eine URL anzeigt und Sie die Authentifizierung im Browser abschließen. Der Aktualisierungsbefehl läuft nach drei Minuten ab, wenn die Authentifizierung nicht abgeschlossen ist. Wenn Sie `gcpAuthRefresh` in Projekteinstellungen wie `.claude/settings.json` festlegen, führt Claude Code ihn unter der gleichen [Workspace-Vertrauensregel wie Hooks in Einstellungsdateien](/docs/de/permissions#what-runs-before-you-trust-a-folder) aus, die `-p`-Sitzungen in Ordnern einschließt, denen Sie noch nie vertraut haben.

<h3 id="4-configure-claude-code">
  4. Konfigurieren Sie Claude Code
</h3>

Legen Sie die folgenden Umgebungsvariablen fest:

```bash theme={null}
# Aktivieren Sie die Agent Platform-Integration
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# Optional: Überschreiben Sie die Agent Platform-Endpunkt-URL für benutzerdefinierte Endpunkte oder Gateways
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# Wenn CLOUD_ML_REGION=global, überschreiben Sie die Region für Modelle, die keine globalen Endpunkte unterstützen
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

Die meisten Modellversionen haben eine entsprechende `VERTEX_REGION_CLAUDE_*`-Variable. Siehe die [Referenz für Umgebungsvariablen](/docs/de/env-vars) für die vollständige Liste. Überprüfen Sie [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden), um zu bestimmen, welche Modelle globale Endpunkte versus nur regionale Endpunkte unterstützen.

Wenn ein Regionswert nicht wie ein Regions- oder Standortname aussieht, behandelt Claude Code ihn als nicht gesetzt. Beispielsweise behandelt Claude Code einen Wert, der einen Schrägstrich, Punkt oder Leerzeichen enthält, als nicht gesetzt. Claude Code fällt für jede Variable auf eine andere Quelle zurück:

* `VERTEX_REGION_CLAUDE_*`: Claude Code fällt auf `CLOUD_ML_REGION` zurück.
* `CLOUD_ML_REGION`: Claude Code fällt auf `us-east5` zurück.

[Prompt Caching](/docs/de/prompt-caching) wird automatisch aktiviert. Um es zu deaktivieren, legen Sie `DISABLE_PROMPT_CACHING=1` fest. Um eine 1-Stunden-Cache-TTL statt des 5-Minuten-Standards anzufordern, legen Sie `ENABLE_PROMPT_CACHING_1H=1` fest; Cache-Schreibvorgänge mit einer 1-Stunden-TTL werden mit einem höheren Satz abgerechnet. Um unterschiedliche TTLs für Ihre Hauptkonversation und für die Anfragen festzulegen, die Claude Code außerhalb davon stellt, [wählen Sie die TTL selbst](/docs/de/prompt-caching#choose-the-ttl-yourself).

Um Ihre Ratenlimits zu erhöhen, wenden Sie sich an den Google Cloud-Support. Bei Verwendung von Google Cloud's Agent Platform ist der Befehl `/logout` nicht verfügbar, da die Authentifizierung über Google Cloud-Anmeldedaten erfolgt.

Claude Code entscheidet zwischen [MCP-Toolsuche](/docs/de/mcp#scale-with-mcp-tool-search) und vorausgehendem Laden nach Modellgeneration:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 und später**: Claude Code aktiviert die Toolsuche standardmäßig.
* **Frühere Modelle, einschließlich aller Claude 3.x-Modelle**: Claude Code lädt MCP-Tool-Definitionen voraus, da ihre Agent Platform-Serving-Stacks den erforderlichen Beta-Header ablehnen. Das Festlegen von `ENABLE_TOOL_SEARCH=true` überschreibt dies nicht.

Legen Sie `ENABLE_TOOL_SEARCH=false` fest, um die Toolsuche auf jedem Modell zu deaktivieren. Vor v2.1.221 deaktivierte Claude Code die Toolsuche für alle Modelle auf Google Cloud's Agent Platform, es sei denn, Sie legen `ENABLE_TOOL_SEARCH=true` fest.

<h3 id="5-pin-model-versions">
  5. Fixieren Sie Modellversionen
</h3>

<Warning>
  Fixieren Sie spezifische Modellversionen bei der Bereitstellung für mehrere Benutzer. Ohne Fixierung werden Modellaliase wie `sonnet` und `opus` zu Claude Codes integriertem Standard für Google Cloud's Agent Platform aufgelöst, der hinter der neuesten Version zurückbleiben kann und möglicherweise noch nicht in Ihrem Projekt aktiviert ist. Claude Code [fällt zurück](#startup-model-checks) beim Start zu einem früheren oder niedrigeren Modell zurück, wenn der Standard nicht verfügbar ist, aber das Fixieren ermöglicht es Ihnen, zu kontrollieren, wann Ihre Benutzer zu einem neuen Modell wechseln.
</Warning>

Legen Sie diese Umgebungsvariablen auf spezifische Google Cloud's Agent Platform-Modell-IDs fest.

Ohne `ANTHROPIC_DEFAULT_OPUS_MODEL` wird der `opus`-Alias auf Google Cloud's Agent Platform zu Opus 5.5 aufgelöst, und ohne `ANTHROPIC_DEFAULT_SONNET_MODEL` wird der `sonnet`-Alias zu Sonnet 4.5 aufgelöst. Dieses Beispiel fixiert jeden Alias auf eine spezifische Version:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Aktuelle und ältere Modell-IDs finden Sie unter [Modellübersicht](https://platform.claude.com/docs/en/about-claude/models/overview). Siehe [Modellkonfiguration](/docs/de/model-config#pin-models-for-third-party-deployments) für die vollständige Liste der Umgebungsvariablen.

Claude Code verwendet diese Standardmodelle, wenn keine Fixierungsvariablen gesetzt sind:

| Modelltyp                | Standardwert                 |
| :----------------------- | :--------------------------- |
| Primäres Modell          | `claude-opus-5-5`            |
| Kleines/schnelles Modell | `claude-sonnet-4-5@20250929` |

Hintergrundaufgaben wie die Generierung von Sitzungstiteln verwenden das kleine/schnelle Modell, normalerweise ein Haiku-Klasse-Modell. Auf Google Cloud's Agent Platform verwendet Claude Code das Standard-Sonnet-Modell für Hintergrundaufgaben, da Haiku möglicherweise nicht in jedem Projekt oder jeder Region aktiviert ist. Zwei Auswahlmöglichkeiten ändern, welches Modell sie trägt:

* Wenn Sie ein primäres Modell mit `--model`, `ANTHROPIC_MODEL` oder der Einstellung `model` auswählen, verwenden Hintergrundaufgaben dieses Modell. Wenn Claude Code die Sitzung auf dem Modell startet, das Sie mit [`ANTHROPIC_DEFAULT_MODEL`](/docs/de/model-config#set-a-default-model-for-new-sessions) festlegen, verwenden Hintergrundaufgaben auch dieses Modell. Das Festlegen von `ANTHROPIC_DEFAULT_OPUS_MODEL` ohne `ANTHROPIC_DEFAULT_SONNET_MODEL` zählt auch als Auswahl, da das integrierte Sonnet-Modell möglicherweise nicht in einem Projekt aktiviert ist, das sein eigenes Opus steuert.
* Um Haiku für Hintergrundaufgaben zu verwenden, legen Sie `ANTHROPIC_DEFAULT_HAIKU_MODEL` auf eine Modell-ID fest, die in Ihrem Projekt verfügbar ist.

<Warning>
  Opus-Modelle haben einen höheren Pro-Token-Preis als Sonnet-Modelle, daher wird eine Bereitstellung, die kein primäres Modell fixiert, mit dem Opus-Satz abgerechnet, sobald sie auf v2.1.207 oder später aktualisiert wird. Um Sonnet 4.5 als primäres Modell beizubehalten, legen Sie `ANTHROPIC_MODEL` auf seine vollständige Modell-ID fest. Eine Bereitstellung, die den Standard mit `ANTHROPIC_DEFAULT_SONNET_MODEL` steuert und `ANTHROPIC_DEFAULT_OPUS_MODEL` nicht setzt, behält ihr gesteuertes Sonnet-Modell als Standard.
</Warning>

Vor v2.1.280 war das primäre Modell auf Google Cloud's Agent Platform standardmäßig Opus 5 und der `opus`-Alias wurde zu Opus 5 von v2.1.219 aufgelöst. Auf v2.1.207 bis v2.1.218 war das primäre Modell auf Google Cloud's Agent Platform standardmäßig Opus 4.8 und der `opus`-Alias wurde zu Opus 4.8 aufgelöst. Vor v2.1.207 war das primäre Modell standardmäßig Sonnet 4.5, der `opus`-Alias wurde zu Opus 4.6 aufgelöst, und Hintergrundaufgaben verwendeten immer das primäre Modell.

Um Modelle weiter anzupassen:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h3 id="6-verify-your-configuration">
  6. Überprüfen Sie Ihre Konfiguration
</h3>

Starten Sie Claude Code und führen Sie `/status` aus, um das Setup zu bestätigen. Die Zeile `API provider` zeigt `Google Vertex AI` an, und die Zeilen `GCP project`, `Default region` und `Model` zeigen Ihre Projekt-ID, Region und aufgelöstes Modell an. Wenn die Provider-Zeile fehlt, erreichen die Umgebungsvariablen den Prozess nicht. Bestätigen Sie, dass sie in der Shell exportiert werden, in der Sie `claude` gestartet haben, oder legen Sie sie im `env`-Block Ihrer [Einstellungsdatei](/docs/de/settings) fest.

<h2 id="startup-model-checks">
  Startmodellprüfungen
</h2>

Wenn Claude Code mit Google Cloud's Agent Platform konfiguriert startet, überprüft es, dass die Modelle, die es verwenden möchte, in Ihrem Projekt zugänglich sind.

Wenn Sie eine Modellversion fixiert haben, die älter als der aktuelle Claude Code-Standard ist, und Ihr Projekt die neuere Version aufrufen kann, fordert Claude Code Sie auf, die Fixierung zu aktualisieren. Das Akzeptieren schreibt die neue Modell-ID in Ihre [Benutzereinstellungsdatei](/docs/de/settings) und startet Claude Code neu. Das Ablehnen wird bis zur nächsten Standardversionänderung beibehalten.

Wenn Sie ein Modell nicht fixiert haben und der aktuelle Standard in Ihrem Projekt nicht verfügbar ist, fällt Claude Code für die aktuelle Sitzung zur vorherigen Version zurück und zeigt einen Hinweis an. Es versucht zuerst frühere Versionen des Standardmodells und fällt, wenn der Standard ein Opus-Modell ist und keine Opus-Version verfügbar ist, auf das Standard-Sonnet-Modell zurück. Das Fallback wird nicht beibehalten. Aktivieren Sie das neuere Modell im [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) oder [fixieren Sie eine Version](#5-pin-model-versions), um die Auswahl dauerhaft zu machen.

Wenn Sie die Sitzung auf einer bestimmten Sonnet- oder Opus-Version starten, beispielsweise mit `--model`, `ANTHROPIC_MODEL` oder der [`model`-Einstellung](/docs/de/settings-reference#model), fungiert diese Version als Standard-Pin der Sitzung für den entsprechenden `sonnet`- oder `opus`-Alias. Claude Code überspringt die Verfügbarkeitsprüfung für den integrierten Standard, den Ihr Modell ersetzt, und startet mit dem von Ihnen konfigurierten Modell, ohne Fallback-Hinweis.

Modellaliase wie `opus` fungieren nicht als Pins, und auch nicht eine Modell-ID, die Claude Code nicht erkennt.

<h2 id="iam-configuration">
  IAM-Konfiguration
</h2>

Weisen Sie die Rolle `roles/aiplatform.user` zu, die die erforderlichen Berechtigungen umfasst:

* `aiplatform.endpoints.predict` - Erforderlich für Modellaufrufe und Token-Zählung

Für restriktivere Berechtigungen erstellen Sie eine benutzerdefinierte Rolle nur mit den oben genannten Berechtigungen.

Weitere Informationen finden Sie in der [Google Cloud Agent Platform IAM-Dokumentation](https://cloud.google.com/vertex-ai/docs/general/access-control).

<Note>
  Erstellen Sie ein dediziertes GCP-Projekt für Claude Code, um die Kostenverfolgung und Zugriffskontrolle zu vereinfachen.
</Note>

<h2 id="1m-token-context-window">
  1M-Token-Kontextfenster
</h2>

Claude Sonnet 5, Opus 4.6 und später sowie Sonnet 4.6 unterstützen das [1M-Token-Kontextfenster](https://platform.claude.com/docs/de/build-with-claude/context-windows#context-window-sizes-by-model) auf Google Cloud's Agent Platform. Sonnet 5 läuft immer mit dem 1M-Fenster, ohne dass eine `[1m]`-Variante zum Auswählen vorhanden ist. Bei den anderen Modellen aktiviert Claude Code automatisch das erweiterte Kontextfenster, wenn Sie eine 1M-Modellvariante auswählen.

Der [Setup-Assistent](#sign-in-with-agent-platform) bietet eine 1M-Kontextoption an, wenn er Modelle fixiert. Um es stattdessen für ein manuell fixiertes Modell zu aktivieren, hängen Sie `[1m]` an die Modell-ID an. Siehe [Modelle für Drittanbieter-Bereitstellungen fixieren](/docs/de/model-config#pin-models-for-third-party-deployments) für Details.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Wenn Sie auf Fehler „Could not load the default credentials" stoßen:

* Führen Sie `gcloud auth application-default login` aus, um Application Default Credentials einzurichten
* Setzen Sie `GOOGLE_APPLICATION_CREDENTIALS` auf einen Pfad zu einer Service-Account-Schlüsseldatei
* Siehe [GCP-Anmeldedaten konfigurieren](#3-configure-gcp-credentials) für alle Optionen

Wenn Sie auf Kontingentprobleme stoßen:

* Überprüfen Sie aktuelle Kontingente oder fordern Sie eine Kontingenterhöhung über die [Cloud Console](https://cloud.google.com/docs/quotas/view-manage) an

Wenn Sie auf Fehler „Modell nicht gefunden" 404 stoßen:

* Bestätigen Sie, dass das Modell im [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) aktiviert ist
* Überprüfen Sie, dass das Modell am angegebenen Standort verfügbar ist. Einige Modelle werden nur auf `global` oder Multi-Region-Standorten wie `eu` und `us` angeboten, nicht in spezifischen Regionen
* Wenn Sie `CLOUD_ML_REGION=global` verwenden, überprüfen Sie, dass Ihre Modelle globale Endpunkte im [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) unter „Unterstützte Funktionen" unterstützen. Für Modelle, die globale Endpunkte nicht unterstützen, können Sie entweder:
  * Ein unterstütztes Modell über `ANTHROPIC_MODEL` oder `ANTHROPIC_DEFAULT_HAIKU_MODEL` angeben, oder
  * Einen regionalen oder Multi-Region-Standort mit `VERTEX_REGION_<MODEL_NAME>`-Umgebungsvariablen festlegen

Wenn Sie auf 429-Fehler stoßen:

* Stellen Sie für regionale Endpunkte sicher, dass das primäre Modell und das kleine/schnelle Modell in Ihrer ausgewählten Region unterstützt werden
* Erwägen Sie, zu `CLOUD_ML_REGION=global` zu wechseln, um bessere Verfügbarkeit zu erreichen

<h2 id="additional-resources">
  Zusätzliche Ressourcen
</h2>

* [Google Cloud's Agent Platform-Dokumentation](https://cloud.google.com/vertex-ai/docs)
* [Google Cloud's Agent Platform-Preisgestaltung](https://cloud.google.com/vertex-ai/pricing)
* [Google Cloud's Agent Platform-Kontingente und -Limits](https://cloud.google.com/vertex-ai/docs/quotas)
