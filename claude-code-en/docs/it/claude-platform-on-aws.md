> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code su Claude Platform on AWS

> Configura Claude Code per utilizzare l'API Claude gestita da Anthropic con autenticazione AWS, controllo dell'accesso IAM e fatturazione tramite AWS Marketplace.

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

Claude Platform on AWS è l'API Claude gestita da Anthropic con autenticazione AWS, controllo dell'accesso IAM e fatturazione tramite AWS Marketplace. Le richieste raggiungono direttamente l'API di Anthropic, quindi si ottengono gli stessi modelli e funzionalità dell'[API Claude](https://platform.claude.com/docs) con la stessa pianificazione dei rilasci. Le funzionalità lato client che Claude Code attiva tramite il servizio di feature flag di Anthropic, come [`/loop` auto-pacing](/docs/it/scheduled-tasks#let-claude-choose-the-interval), sono disattivate per impostazione predefinita, e lo [strumento advisor](/docs/it/advisor) non è disponibile. Consultare la [matrice di disponibilità delle funzionalità](/docs/it/feature-availability#summary-by-provider) per l'elenco completo. L'autenticazione avviene con le credenziali AWS o una chiave API dell'area di lavoro, e il pagamento avviene tramite AWS Marketplace.

Utilizzare questa guida per indirizzare Claude Code a un'area di lavoro già fornita tramite Claude Platform on AWS. Per la sottoscrizione AWS e la configurazione dell'area di lavoro che precede questo, consultare la [documentazione di Claude Platform on AWS](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws).

<Note>
  L'iscrizione tramite AWS Marketplace fornisce una nuova organizzazione Anthropic collegata al proprio account AWS. Questa organizzazione è separata da qualsiasi organizzazione già presente con Anthropic e le credenziali non si trasferiscono tra loro. Utilizzare l'ID dell'area di lavoro e le chiavi API dall'organizzazione collegata ad AWS, non da un account Claude Console preesistente.
</Note>

<h2 id="prerequisites">
  Prerequisiti
</h2>

Prima di configurare Claude Code, è necessario disporre di:

* Una sottoscrizione attiva a Claude Platform on AWS tramite AWS Marketplace
* Un'area di lavoro nella propria organizzazione Anthropic collegata ad AWS, con il relativo ID dell'area di lavoro
* Un principale IAM con autorizzazione per invocare il servizio Anthropic, oppure una chiave API limitata all'area di lavoro
* Credenziali AWS nel proprio ambiente, in `~/.aws/credentials`, o da un ruolo IAM collegato se si desidera l'autenticazione SigV4. L'AWS CLI è richiesta solo per il flusso di accesso SSO.

<h2 id="setup">
  Configurazione
</h2>

<h3 id="1-configure-aws-credentials">
  1. Configurare le credenziali AWS
</h3>

Claude Code supporta due metodi di autenticazione per Claude Platform on AWS. Scegliere il metodo che si adatta al modo in cui il team gestisce l'accesso.

**Opzione A: Credenziali AWS con SigV4**

Claude Code firma le richieste con SigV4 utilizzando la catena di credenziali AWS standard: variabili di ambiente, credenziali condivise in `~/.aws/credentials`, ruoli IAM, sessioni AWS SSO e qualsiasi altra fonte supportata dall'SDK AWS.

Per l'uso locale, accedere con l'AWS CLI prima di avviare Claude Code. L'esempio seguente utilizza un profilo SSO, ma qualsiasi metodo che produce credenziali nelle posizioni standard funziona.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

Per CI e automazione, fornire al runner un ruolo IAM con autorizzazione per invocare il servizio Anthropic e impostare `AWS_REGION`. La catena di credenziali raccoglie il ruolo automaticamente.

Se le credenziali SSO scadono durante la sessione, configurare [`awsAuthRefresh`](/docs/it/amazon-bedrock#advanced-credential-configuration) in modo che Claude Code riesegua il comando di accesso e riprovi invece di fallire. L'aggiornamento automatico su Claude Platform on AWS richiede Claude Code v2.1.198 o successivo; le versioni precedenti si fermano con un prompt per eseguire `/login`, che non può aggiornare le credenziali AWS. Aggiungere il comando al file `settings.json`:

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

Con `awsAuthRefresh` configurato, `/login` mostra un'opzione **Claude Platform on AWS · refresh credentials** in **Using 3rd-party platforms**. Selezionandola si esegue il comando configurato e si rileggono le credenziali AWS senza riavviare Claude Code.

**Opzione B: Chiave API dell'area di lavoro**

Una chiave API dell'area di lavoro è un segreto di lunga durata, utile quando non si desidera gestire credenziali AWS federate. Generarne una nella console AWS in **Claude Platform on AWS → API keys** e impostarla come `ANTHROPIC_AWS_API_KEY`:

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

La chiave viene inviata come `x-api-key` e ha la precedenza su SigV4, quindi qualsiasi credenziale AWS nel proprio ambiente viene ignorata. Le chiavi API da un'organizzazione Claude Console separata non funzioneranno qui.

Trattare le chiavi API dell'area di lavoro come qualsiasi altra credenziale di produzione. Il blocco `env` del [file delle impostazioni utente](/docs/it/settings) è un modo conveniente per limitare la chiave alla propria macchina senza esportarla globalmente.

<Note>
  I comandi `/login` e `/logout` non consentono di accedere a un abbonamento Claude.ai per Claude Platform on AWS. L'autenticazione viene eseguita tramite le credenziali AWS o la chiave API dell'area di lavoro. L'eccezione è l'opzione **refresh credentials** che `/login` mostra quando `awsAuthRefresh` è configurato, che rilegge le credenziali AWS come descritto sopra.
</Note>

<h3 id="2-configure-claude-code">
  2. Configurare Claude Code
</h3>

Impostare le variabili di ambiente che indirizzano Claude Code attraverso Claude Platform on AWS invece dell'API Anthropic predefinita.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` è obbligatorio e viene inviato su ogni richiesta come intestazione `anthropic-workspace-id`. L'URL di base viene calcolato da `AWS_REGION` come `https://aws-external-anthropic.{region}.api.aws`. Per sovrascrivere l'URL direttamente, impostare `ANTHROPIC_AWS_BASE_URL`.

Claude Platform on AWS è facoltativo anche quando le credenziali AWS sono presenti nel proprio ambiente. Amazon Bedrock e Microsoft Foundry hanno la precedenza nel routing dei provider, quindi annullare l'impostazione di `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_FOUNDRY` se sono impostati.

<h3 id="3-pin-model-versions">
  3. Fissare le versioni dei modelli
</h3>

Claude Platform on AWS utilizza gli stessi ID modello dell'API Claude diretta.

Gli alias predefiniti `fable`, `opus`, `sonnet` e `haiku` si risolvono alle impostazioni predefinite integrate di Claude Code per Claude Platform on AWS, che possono essere in ritardo rispetto alla versione più recente. Senza `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` si risolve in Opus 4.8. Prima della v2.1.207, si risolveva in Opus 4.7.

Se si distribuisce Claude Code a un team, fissare esplicitamente gli ID modello in modo che un nuovo rilascio non sposti tutti contemporaneamente:

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

Per l'elenco completo degli ID modello e degli alias, consultare [Panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview). Per altre variabili relative ai modelli, consultare [Configurazione del modello](/docs/it/model-config).

[Prompt caching](/docs/it/prompt-caching) è abilitato automaticamente. Per richiedere un TTL della cache di 1 ora invece del valore predefinito di 5 minuti, impostare `ENABLE_PROMPT_CACHING_1H=1`. L'API fattura le scritture della cache di 1 ora a una tariffa più alta. Consultare [prezzi del prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) per le tariffe.

<h2 id="use-the-agent-sdk">
  Utilizzare l'Agent SDK
</h2>

L'[Agent SDK](/docs/it/agent-sdk/overview) legge le stesse variabili di ambiente della CLI, quindi qualsiasi programma che genera il sottoprocesso Claude Code può indirizzare Claude Platform on AWS esportando `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID` e `ANTHROPIC_AWS_API_KEY` o credenziali AWS prima della chiamata.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

Questo esempio si basa sulla catena di credenziali AWS ambientale per SigV4. Per autenticarsi con una chiave API dell'area di lavoro, impostare `ANTHROPIC_AWS_API_KEY` allo stesso modo. Per la superficie più ampia dell'Agent SDK, consultare [Panoramica dell'Agent SDK](/docs/it/agent-sdk/overview).

<h2 id="route-through-a-corporate-proxy">
  Instradare attraverso un proxy aziendale
</h2>

Per instradare il traffico attraverso un proxy o un [gateway LLM](/docs/it/llm-gateway), impostare `ANTHROPIC_AWS_BASE_URL` all'indirizzo del proxy. Claude Code invia richieste a quell'URL con le stesse intestazioni di area di lavoro e autenticazione, quindi qualsiasi gateway che le inoltri invariate funziona.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

Se il gateway firma le richieste stesso, impostare `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1` in modo che Claude Code invii richieste non firmate e lasci che il gateway aggiunga intestazioni SigV4 prima di inoltrarle ad AWS. Se il gateway richiede il proprio token, impostarlo in `ANTHROPIC_AUTH_TOKEN`.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Eseguire `/status` per visualizzare il provider risolto e qualsiasi ID dell'area di lavoro configurato esplicitamente, override della regione, override dell'URL di base e impostazione di skip dell'autenticazione. Questo è il modo più veloce per confermare che Claude Code sta indirizzando Claude Platform on AWS.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` o `AccessDenied` su ogni richiesta
</h3>

Il principale IAM che Claude Code ha risolto probabilmente manca dell'autorizzazione per invocare il servizio Anthropic nell'area di lavoro. Controllare il ruolo collegato al profilo AWS o al runner che ha avviato Claude Code e verificare che disponga delle azioni `aws-external-anthropic` documentate nel [riferimento delle azioni IAM](https://platform.claude.com/docs/it/api/claude-platform-on-aws-iam-actions).

Se è stato impostato `ANTHROPIC_AWS_API_KEY`, la chiave ha la precedenza su SigV4 e una chiave obsoleta produce lo stesso errore. Rigenerare la chiave nella console AWS in **Claude Platform on AWS → API keys** o annullare l'impostazione della variabile per tornare alle credenziali AWS.

<h3 id="requests-fail-with-a-missing-workspace-error">
  Le richieste non riescono con un errore di area di lavoro mancante
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` è probabilmente non impostato o vuoto. Ogni richiesta di Claude Platform on AWS deve includere l'ID dell'area di lavoro. Non è implicito dalle credenziali AWS. Trovare l'ID in **Workspaces** nella pagina del servizio della console AWS ed esportarlo prima di avviare Claude Code.

<h3 id="requests-still-go-to-api-anthropic-com">
  Le richieste vanno ancora a `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` è probabilmente non impostato o impostato su un valore che non viene analizzato come veritiero. Impostarlo su `1` ed eseguire `/status` per confermare il provider risolto. Se è impostato anche `CLAUDE_CODE_USE_BEDROCK` o `CLAUDE_CODE_USE_FOUNDRY`, questi hanno la precedenza su Claude Platform on AWS.

<h2 id="additional-resources">
  Risorse aggiuntive
</h2>

La sottoscrizione a Claude Platform on AWS, la configurazione dell'area di lavoro e IAM che precede la configurazione di Claude Code è coperta nella documentazione della piattaforma:

* [Panoramica di Claude Platform on AWS](https://platform.claude.com/docs/it/build-with-claude/claude-platform-on-aws): sottoscrizione, configurazione dell'area di lavoro e riferimento del prodotto
* [Riferimento delle azioni IAM](https://platform.claude.com/docs/it/api/claude-platform-on-aws-iam-actions): autorizzazioni e criteri gestiti
