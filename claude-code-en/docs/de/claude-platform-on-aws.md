> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code auf Claude Platform on AWS

> Konfigurieren Sie Claude Code für die Verwendung der von Anthropic betriebenen Claude API mit AWS-Authentifizierung, IAM-Zugriffskontrolle und AWS Marketplace-Abrechnung.

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

export const Experiment = ({flag, treatment, children}) => {
  const VID_KEY = 'exp_vid';
  const CONSENT_COUNTRIES = new Set(['AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR', 'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL', 'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', 'RE', 'GP', 'MQ', 'GF', 'YT', 'BL', 'MF', 'PM', 'WF', 'PF', 'NC', 'AW', 'CW', 'SX', 'FO', 'GL', 'AX', 'GB', 'UK', 'AI', 'BM', 'IO', 'VG', 'KY', 'FK', 'GI', 'MS', 'PN', 'SH', 'TC', 'GG', 'JE', 'IM', 'CA', 'BR', 'IN']);
  const fnv1a = s => {
    let h = 0x811c9dc5;
    for (let i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
      h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    }
    return h >>> 0;
  };
  const bucket = (seed, vid) => fnv1a(fnv1a(seed + vid) + '') % 10000 < 5000 ? 'control' : 'treatment';
  const [decision] = useState(() => {
    const params = new URLSearchParams(location.search);
    const preBucketed = document.documentElement.dataset['gb_' + flag.replace(/-/g, '_')];
    const force = params.get('gb-force');
    if (force) {
      for (const p of force.split(',')) {
        const [k, v] = p.split(':');
        if (k === flag) return {
          variant: v || 'treatment',
          track: false
        };
      }
    }
    if (navigator.globalPrivacyControl) {
      return {
        variant: 'control',
        track: false
      };
    }
    const prefsMatch = document.cookie.match(/(?:^|; )anthropic-consent-preferences=([^;]+)/);
    if (prefsMatch) {
      try {
        if (JSON.parse(decodeURIComponent(prefsMatch[1])).analytics !== true) {
          return {
            variant: 'control',
            track: false
          };
        }
      } catch {
        return {
          variant: 'control',
          track: false
        };
      }
    } else {
      const country = params.get('country')?.toUpperCase() || (document.cookie.match(/(?:^|; )cf_geo=([A-Z]{2})/) || [])[1];
      if (!country || CONSENT_COUNTRIES.has(country)) {
        return {
          variant: 'control',
          track: false
        };
      }
    }
    let vid;
    try {
      const ajsMatch = document.cookie.match(/(?:^|; )ajs_anonymous_id=([^;]+)/);
      if (ajsMatch) {
        vid = decodeURIComponent(ajsMatch[1]).replace(/^"|"$/g, '');
      } else {
        vid = localStorage.getItem(VID_KEY);
        if (!vid) {
          vid = crypto.randomUUID();
        }
        document.cookie = `ajs_anonymous_id=${vid}; domain=.claude.com; path=/; Secure; SameSite=Lax; max-age=31536000`;
      }
      try {
        localStorage.setItem(VID_KEY, vid);
      } catch {}
    } catch {
      return {
        variant: 'control',
        track: false
      };
    }
    const variant = preBucketed === '1' ? 'treatment' : preBucketed === '0' ? 'control' : bucket(flag, vid);
    return {
      variant,
      track: true,
      vid
    };
  });
  useEffect(() => {
    if (!decision.track) return;
    fetch('https://api.anthropic.com/api/event_logging/v2/batch', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-service-name': 'claude_code_docs'
      },
      body: JSON.stringify({
        events: [{
          event_type: 'GrowthbookExperimentEvent',
          event_data: {
            device_id: decision.vid,
            anonymous_id: decision.vid,
            timestamp: new Date().toISOString(),
            experiment_id: flag,
            variation_id: decision.variant === 'treatment' ? 1 : 0,
            environment: 'production'
          }
        }]
      }),
      keepalive: true
    }).catch(() => {});
  }, []);
  return decision.variant === 'treatment' ? treatment : children;
};

<Experiment flag="docs-contact-sales-cta" treatment={<ContactSalesCard surface="claude_platform_on_aws" />} />

Claude Platform on AWS ist die von Anthropic betriebene Claude API mit AWS-Authentifizierung, IAM-Zugriffskontrolle und AWS Marketplace-Abrechnung. Anfragen erreichen die API von Anthropic direkt, sodass Sie die gleichen Modelle und API-Funktionen wie die [Claude API](https://platform.claude.com/docs) nach dem gleichen Veröffentlichungsplan erhalten. Client-seitige Funktionen, die Claude Code durch den Feature-Flag-Service von Anthropic aktiviert, wie z. B. [`/loop` Selbststeuerung](/docs/de/scheduled-tasks#let-claude-choose-the-interval), sind standardmäßig deaktiviert, und das [Advisor-Tool](/docs/de/advisor) ist nicht verfügbar. Siehe die [Feature-Verfügbarkeitsmatrix](/docs/de/feature-availability#summary-by-provider) für die vollständige Liste. Sie authentifizieren sich mit AWS-Anmeldedaten oder einem Workspace-API-Schlüssel und zahlen über AWS Marketplace.

Verwenden Sie diese Anleitung, um Claude Code auf einen Workspace zu verweisen, den Sie bereits über Claude Platform on AWS bereitgestellt haben. Für das AWS-Abonnement und die Workspace-Einrichtung, die davor kommt, siehe die [Claude Platform on AWS-Dokumentation](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws).

<Note>
  Das Abonnement über AWS Marketplace stellt eine neue Anthropic-Organisation bereit, die mit Ihrem AWS-Konto verknüpft ist. Diese Organisation ist separat von jeder Organisation, die Sie bereits bei Anthropic haben, und Anmeldedaten werden nicht zwischen ihnen übertragen. Verwenden Sie die Workspace-ID und API-Schlüssel aus der AWS-verknüpften Organisation, nicht aus einem bereits vorhandenen Claude Console-Konto.
</Note>

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Vor der Konfiguration von Claude Code benötigen Sie:

* Ein aktives Claude Platform on AWS-Abonnement über AWS Marketplace
* Einen Workspace in Ihrer AWS-verknüpften Anthropic-Organisation mit seiner Workspace-ID
* Einen IAM-Principal mit Berechtigung zum Aufrufen des Anthropic-Dienstes oder einen API-Schlüssel, der auf den Workspace beschränkt ist
* AWS-Anmeldedaten in Ihrer Umgebung, in `~/.aws/credentials` oder von einer angehängten IAM-Rolle, wenn Sie SigV4-Authentifizierung möchten. Die AWS CLI ist nur für den SSO-Anmeldungsfluss erforderlich.

<h2 id="setup">
  Einrichtung
</h2>

<h3 id="1-configure-aws-credentials">
  1. AWS-Anmeldedaten konfigurieren
</h3>

Claude Code unterstützt zwei Authentifizierungsmethoden für Claude Platform on AWS. Wählen Sie die Methode, die zu Ihrer Zugriffsverwaltung passt.

**Option A: AWS-Anmeldedaten mit SigV4**

Claude Code signiert Anfragen mit SigV4 unter Verwendung der Standard-AWS-Anmeldekette: Umgebungsvariablen, gemeinsame Anmeldedaten in `~/.aws/credentials`, IAM-Rollen, AWS SSO-Sitzungen und alle anderen Quellen, die das AWS SDK unterstützt.

Für die lokale Verwendung melden Sie sich mit der AWS CLI an, bevor Sie Claude Code starten. Das folgende Beispiel verwendet ein SSO-Profil, aber jede Methode, die Anmeldedaten an den Standardorten erzeugt, funktioniert.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

Für CI und Automatisierung geben Sie dem Runner eine IAM-Rolle mit Berechtigung zum Aufrufen des Anthropic-Dienstes und setzen Sie `AWS_REGION`. Die Anmeldekette nimmt die Rolle automatisch auf.

Wenn Ihre SSO-Anmeldedaten während einer Sitzung ablaufen, konfigurieren Sie [`awsAuthRefresh`](/docs/de/amazon-bedrock#advanced-credential-configuration), damit Claude Code Ihren Anmeldungsbefehl erneut ausführt und erneut versucht, anstatt fehlzuschlagen. Automatische Aktualisierung auf Claude Platform on AWS erfordert Claude Code v2.1.198 oder später; frühere Versionen stoppen mit einer Aufforderung, `/login` auszuführen, was AWS-Anmeldedaten nicht aktualisieren kann. Fügen Sie den Befehl zu Ihrer `settings.json` hinzu:

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

Mit konfiguriertem `awsAuthRefresh` zeigt `/login` eine Option **Claude Platform on AWS · Anmeldedaten aktualisieren** unter **Verwendung von Plattformen von Drittanbietern** an. Wenn Sie diese Option auswählen, wird der konfigurierte Befehl ausgeführt und Ihre AWS-Anmeldedaten werden erneut gelesen, ohne Claude Code neu zu starten.

**Option B: Workspace-API-Schlüssel**

Ein Workspace-API-Schlüssel ist ein langlebiges Geheimnis, das nützlich ist, wenn Sie keine verbundenen AWS-Anmeldedaten verwalten möchten. Generieren Sie einen in der AWS Console unter **Claude Platform on AWS → API keys** und setzen Sie ihn als `ANTHROPIC_AWS_API_KEY`:

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

Der Schlüssel wird als `x-api-key` gesendet und hat Vorrang vor SigV4, sodass alle AWS-Anmeldedaten in Ihrer Umgebung ignoriert werden. API-Schlüssel aus einer separaten Claude Console-Organisation funktionieren hier nicht.

Behandeln Sie Workspace-API-Schlüssel wie jede andere Produktionsanmeldedaten. Der [Benutzereinstellungsdatei](/docs/de/settings) `env`-Block ist eine praktische Möglichkeit, den Schlüssel auf Ihrem Computer zu beschränken, ohne ihn global zu exportieren.

<Note>
  Die Befehle `/login` und `/logout` melden Sie nicht bei einem Claude.ai-Abonnement für Claude Platform on AWS an. Die Authentifizierung erfolgt über Ihre AWS-Anmeldedaten oder Ihren Workspace-API-Schlüssel. Die Ausnahme ist die Option **Anmeldedaten aktualisieren**, die `/login` anzeigt, wenn `awsAuthRefresh` konfiguriert ist. Diese liest Ihre AWS-Anmeldedaten wie oben beschrieben erneut.
</Note>

<h3 id="2-configure-claude-code">
  2. Claude Code konfigurieren
</h3>

Setzen Sie die Umgebungsvariablen, die Claude Code durch Claude Platform on AWS anstelle der Standard-Anthropic API leiten.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` ist erforderlich und wird bei jeder Anfrage als `anthropic-workspace-id`-Header gesendet. Die Basis-URL wird aus `AWS_REGION` als `https://aws-external-anthropic.{region}.api.aws` berechnet. Um die URL direkt zu überschreiben, setzen Sie `ANTHROPIC_AWS_BASE_URL`.

Claude Platform on AWS ist optional, auch wenn AWS-Anmeldedaten in Ihrer Umgebung vorhanden sind. Amazon Bedrock und Microsoft Foundry haben Vorrang beim Provider-Routing, daher heben Sie `CLAUDE_CODE_USE_BEDROCK` und `CLAUDE_CODE_USE_FOUNDRY` auf, wenn sie gesetzt sind.

<h3 id="3-pin-model-versions">
  3. Modellversionen anheften
</h3>

Claude Platform on AWS verwendet die gleichen Modell-IDs wie die direkte Claude API.

Die Standard-Aliase `fable`, `opus`, `sonnet` und `haiku` werden zu Claude Codes integrierten Standardwerten für Claude Platform on AWS aufgelöst, die hinter der neuesten Version zurückbleiben können. Ohne `ANTHROPIC_DEFAULT_OPUS_MODEL` wird der `opus`-Alias zu Opus 4.8 aufgelöst. Vor v2.1.207 wurde er zu Opus 4.7 aufgelöst.

Wenn Sie Claude Code für ein Team bereitstellen, heften Sie die Modell-IDs explizit an, damit eine neue Version nicht alle auf einmal verschiebt:

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

Die vollständige Liste der Modell-IDs und Aliase finden Sie unter [Modellübersicht](https://platform.claude.com/docs/en/about-claude/models/overview). Für andere modellbezogene Variablen siehe [Modellkonfiguration](/docs/de/model-config).

[Prompt Caching](/docs/de/prompt-caching) ist automatisch aktiviert. Um statt des 5-Minuten-Standards eine 1-Stunden-Cache-TTL anzufordern, setzen Sie `ENABLE_PROMPT_CACHING_1H=1`. Die API berechnet 1-Stunden-Cache-Schreibvorgänge mit einer höheren Rate ab. Siehe [Prompt Caching-Preisgestaltung](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) für die Tarife.

<h2 id="use-the-agent-sdk">
  Agent SDK verwenden
</h2>

Das [Agent SDK](/docs/de/agent-sdk/overview) liest die gleichen Umgebungsvariablen wie die CLI, sodass jedes Programm, das den Claude Code-Unterprozess erzeugt, Claude Platform on AWS anvisieren kann, indem es `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID` und entweder `ANTHROPIC_AWS_API_KEY` oder AWS-Anmeldedaten vor dem Aufruf exportiert.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

Dieses Beispiel basiert auf der Umgebungs-AWS-Anmeldekette für SigV4. Um sich stattdessen mit einem Workspace-API-Schlüssel zu authentifizieren, setzen Sie `ANTHROPIC_AWS_API_KEY` auf die gleiche Weise. Für die breitere Agent SDK-Oberfläche siehe [Agent SDK-Übersicht](/docs/de/agent-sdk/overview).

<h2 id="route-through-a-corporate-proxy">
  Durch einen Unternehmens-Proxy leiten
</h2>

Um Datenverkehr durch einen Proxy oder [LLM-Gateway](/docs/de/llm-gateway) zu leiten, setzen Sie `ANTHROPIC_AWS_BASE_URL` auf die Adresse des Proxys. Claude Code sendet Anfragen an diese URL mit den gleichen Workspace- und Authentifizierungsheadern, sodass jedes Gateway, das sie unverändert weiterleitet, funktioniert.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

Wenn Ihr Gateway Anfragen selbst signiert, setzen Sie `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1`, damit Claude Code unsignierte Anfragen sendet und das Gateway SigV4-Header hinzufügt, bevor es an AWS weitergeleitet wird. Wenn das Gateway sein eigenes Token benötigt, setzen Sie es in `ANTHROPIC_AUTH_TOKEN`.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Führen Sie `/status` aus, um den aufgelösten Provider und alle explizit konfigurierten Workspace-ID, Region, Basis-URL-Überschreibung und Auth-Skip-Einstellung zu sehen. Dies ist der schnellste Weg, um zu bestätigen, dass Claude Code überhaupt auf Claude Platform on AWS abzielt.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` oder `AccessDenied` bei jeder Anfrage
</h3>

Der IAM-Principal, den Claude Code aufgelöst hat, hat wahrscheinlich keine Berechtigung zum Aufrufen des Anthropic-Dienstes in Ihrem Workspace. Überprüfen Sie die Rolle, die an Ihr AWS-Profil oder den Runner angehängt ist, der Claude Code gestartet hat, und überprüfen Sie, ob sie die `aws-external-anthropic`-Aktionen hat, die in der [IAM-Aktionsreferenz](https://platform.claude.com/docs/de/api/claude-platform-on-aws-iam-actions) dokumentiert sind.

Wenn Sie `ANTHROPIC_AWS_API_KEY` setzen, hat der Schlüssel Vorrang vor SigV4 und ein veralteter Schlüssel erzeugt den gleichen Fehler. Generieren Sie den Schlüssel in der AWS Console unter **Claude Platform on AWS → API keys** neu oder heben Sie die Variable auf, um auf Ihre AWS-Anmeldedaten zurückzugreifen.

<h3 id="requests-fail-with-a-missing-workspace-error">
  Anfragen schlagen mit einem fehlenden Workspace-Fehler fehl
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` ist wahrscheinlich nicht gesetzt oder leer. Jede Claude Platform on AWS-Anfrage muss die Workspace-ID enthalten. Sie wird nicht durch Ihre AWS-Anmeldedaten impliziert. Finden Sie die ID unter **Workspaces** auf der AWS Console-Serviceseite und exportieren Sie sie, bevor Sie Claude Code starten.

<h3 id="requests-still-go-to-api-anthropic-com">
  Anfragen gehen immer noch an `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` ist wahrscheinlich nicht gesetzt oder auf einen Wert gesetzt, der nicht als wahr analysiert wird. Setzen Sie ihn auf `1` und führen Sie `/status` aus, um den aufgelösten Provider zu bestätigen. Wenn `CLAUDE_CODE_USE_BEDROCK` oder `CLAUDE_CODE_USE_FOUNDRY` auch gesetzt ist, haben diese Vorrang vor Claude Platform on AWS.

<h2 id="additional-resources">
  Zusätzliche Ressourcen
</h2>

Das Claude Platform on AWS-Abonnement, der Workspace und die IAM-Einrichtung, die vor der Konfiguration von Claude Code kommt, werden in der Plattformdokumentation behandelt:

* [Claude Platform on AWS-Übersicht](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws): Abonnement, Workspace-Einrichtung und Produktreferenz
* [IAM-Aktionsreferenz](https://platform.claude.com/docs/en/api/claude-platform-on-aws-iam-actions): Berechtigungen und verwaltete Richtlinien
