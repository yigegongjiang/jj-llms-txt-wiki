> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panoramica della distribuzione aziendale

> Scopri come Claude Code può integrarsi con vari servizi di terze parti e infrastrutture per soddisfare i requisiti di distribuzione aziendale.

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

Le organizzazioni possono distribuire Claude Code direttamente tramite Anthropic o tramite un provider cloud. Questa pagina ti aiuta a scegliere la configurazione giusta.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  Confronta le opzioni di distribuzione
</h2>

Per la maggior parte delle organizzazioni, Claude for Teams o Claude for Enterprise offre la migliore esperienza. I membri del team ottengono accesso sia a Claude Code che a Claude sul web con un'unica sottoscrizione, fatturazione centralizzata e nessuna configurazione dell'infrastruttura richiesta.

**Claude for Teams** è self-service e include funzionalità di collaborazione, strumenti di amministrazione e gestione della fatturazione. Ideale per team più piccoli che hanno bisogno di iniziare rapidamente.

**Claude for Enterprise** aggiunge SSO e domain capture, autorizzazioni basate sui ruoli, accesso all'API di conformità e impostazioni di policy gestite per la distribuzione di configurazioni Claude Code a livello organizzativo. Ideale per organizzazioni più grandi con requisiti di sicurezza e conformità.

Scopri di più su [Team plans](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) e [Enterprise plans](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

Se la tua organizzazione ha requisiti infrastrutturali specifici, confronta le opzioni di seguito:

<table>
  <thead>
    <tr>
      <th>Funzionalità</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform, formerly Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>Ideale per</td>
      <td>La maggior parte delle organizzazioni (consigliato)</td>
      <td>Sviluppatori individuali</td>
      <td>Distribuzioni native AWS</td>
      <td>Fatturazione AWS Marketplace con funzionalità Claude API</td>
      <td>Distribuzioni native GCP</td>
      <td>Distribuzioni native Azure</td>
    </tr>

    <tr>
      <td>Fatturazione</td>
      <td><strong>Teams:</strong> \$150/seat (Premium) con PAYG disponibile<br /><strong>Enterprise:</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">Contatta il team di vendita</a></td>
      <td>PAYG</td>
      <td>PAYG tramite AWS</td>
      <td>PAYG tramite AWS Marketplace</td>
      <td>PAYG tramite GCP</td>
      <td>PAYG tramite Azure</td>
    </tr>

    <tr>
      <td>Regioni</td>
      <td>Paesi supportati [countries](https://www.anthropic.com/supported-countries)</td>
      <td>Paesi supportati [countries](https://www.anthropic.com/supported-countries)</td>
      <td>Più [regions](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html) AWS</td>
      <td>Più regioni AWS</td>
      <td>Più [regions](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) GCP</td>
      <td>Più [regions](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/) Azure</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>Abilitato per impostazione predefinita</td>
      <td>Abilitato per impostazione predefinita</td>
      <td>Abilitato per impostazione predefinita</td>
      <td>Abilitato per impostazione predefinita</td>
      <td>Abilitato per impostazione predefinita</td>
      <td>Abilitato per impostazione predefinita</td>
    </tr>

    <tr>
      <td>Autenticazione</td>
      <td>Claude.ai SSO o email</td>
      <td>API key</td>
      <td>API key o credenziali AWS</td>
      <td>API key o credenziali AWS</td>
      <td>Credenziali GCP</td>
      <td>API key o Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>Tracciamento dei costi</td>
      <td>Dashboard di utilizzo</td>
      <td>Dashboard di utilizzo</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>GCP Billing</td>
      <td>Azure Cost Management</td>
    </tr>

    <tr>
      <td>Include Claude sul web</td>
      <td>Sì</td>
      <td>No</td>
      <td>No</td>
      <td>No</td>
      <td>No</td>
      <td>No</td>
    </tr>

    <tr>
      <td>Funzionalità Enterprise</td>
      <td>Gestione del team, SSO, monitoraggio dell'utilizzo</td>
      <td>Nessuna</td>
      <td>Policy IAM, CloudTrail</td>
      <td>Policy IAM, CloudTrail</td>
      <td>Ruoli IAM, Cloud Audit Logs</td>
      <td>Policy RBAC, Azure Monitor</td>
    </tr>
  </tbody>
</table>

Per una suddivisione funzionalità per funzionalità di ciò che è disponibile su ogni opzione, vedi [Feature availability](/docs/it/feature-availability).

Seleziona un'opzione di distribuzione per visualizzare le istruzioni di configurazione:

* [Claude for Teams o Enterprise](/docs/it/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/it/authentication#claude-console-authentication)
* [Claude apps gateway](/docs/it/claude-apps-gateway), un gateway self-hosted che aggiunge l'accesso IdP davanti ad Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry o l'API Anthropic
* [Amazon Bedrock](/docs/it/amazon-bedrock)
* [Claude Platform on AWS](/docs/it/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/it/google-vertex-ai)
* [Microsoft Foundry](/docs/it/microsoft-foundry)

Per Amazon Bedrock e Google Vertex AI, puoi anche eseguire `claude` e selezionare **3rd-party platform** al prompt di accesso per avviare una procedura guidata di configurazione interattiva.

<h2 id="configure-proxies-and-gateways">
  Configura proxy e gateway
</h2>

La maggior parte delle organizzazioni può utilizzare un provider cloud direttamente senza configurazione aggiuntiva. Tuttavia, potrebbe essere necessario configurare un proxy aziendale o un gateway LLM se la tua organizzazione ha requisiti di rete o gestione specifici. Queste sono configurazioni diverse che possono essere utilizzate insieme:

* **Corporate proxy**: Instrada il traffico attraverso un proxy HTTP/HTTPS. Utilizzalo se la tua organizzazione richiede che tutto il traffico in uscita passi attraverso un server proxy per il monitoraggio della sicurezza, la conformità o l'applicazione della policy di rete. Configura con le variabili di ambiente `HTTPS_PROXY` o `HTTP_PROXY`. Scopri di più in [Enterprise network configuration](/docs/it/network-config).
* **LLM Gateway**: Un servizio che si trova tra Claude Code e il provider cloud per gestire l'autenticazione e il routing. Utilizzalo se hai bisogno di tracciamento centralizzato dell'utilizzo tra i team, rate limiting personalizzato o budget, o gestione centralizzata dell'autenticazione. Configura con le variabili di ambiente `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, o `ANTHROPIC_FOUNDRY_BASE_URL`. Scopri di più in [LLM gateways](/docs/it/llm-gateway).

I seguenti esempi mostrano le variabili di ambiente da impostare nella tua shell o nel profilo shell (`.bashrc`, `.zshrc`). Vedi [Settings](/docs/it/settings) per altri metodi di configurazione.

<h3 id="amazon-bedrock">
  Amazon Bedrock
</h3>

<Tabs>
  <Tab title="Corporate proxy">
    Instrada il traffico Amazon Bedrock attraverso il tuo proxy aziendale impostando le seguenti [variabili di ambiente](/docs/it/env-vars):

    ```bash theme={null}
    # Enable Bedrock
    export CLAUDE_CODE_USE_BEDROCK=1
    export AWS_REGION=us-east-1

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM Gateway">
    Instrada il traffico Amazon Bedrock attraverso il tuo gateway LLM impostando le seguenti [variabili di ambiente](/docs/it/env-vars):

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
  <Tab title="Corporate proxy">
    Instrada il traffico Microsoft Foundry attraverso il tuo proxy aziendale impostando le seguenti [variabili di ambiente](/docs/it/env-vars):

    ```bash theme={null}
    # Enable Microsoft Foundry
    export CLAUDE_CODE_USE_FOUNDRY=1
    export ANTHROPIC_FOUNDRY_RESOURCE=your-resource
    export ANTHROPIC_FOUNDRY_API_KEY=your-api-key  # Or omit for Entra ID auth

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM Gateway">
    Instrada il traffico Microsoft Foundry attraverso il tuo gateway LLM impostando le seguenti [variabili di ambiente](/docs/it/env-vars):

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
  <Tab title="Corporate proxy">
    Instrada il traffico di Google Cloud's Agent Platform attraverso il tuo proxy aziendale impostando le seguenti [variabili di ambiente](/docs/it/env-vars):

    ```bash theme={null}
    # Enable Agent Platform
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id

    # Configure corporate proxy
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM Gateway">
    Instrada il traffico di Google Cloud's Agent Platform attraverso il tuo gateway LLM impostando le seguenti [variabili di ambiente](/docs/it/env-vars):

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
  Usa `/status` in Claude Code per verificare che la configurazione del proxy e del gateway sia applicata correttamente. Ad esempio, con la configurazione del gateway Bedrock sopra, l'output include righe come:

  ```
  API provider: Amazon Bedrock
  Bedrock base URL: https://your-llm-gateway.com/bedrock
  AWS region: us-east-1
  AWS auth skipped
  ```

  Se hai configurato un proxy aziendale, `/status` mostra anche una riga `Proxy` con l'URL del tuo proxy.
</Tip>

<h2 id="best-practices-for-organizations">
  Best practices for organizations
</h2>

<h3 id="invest-in-documentation-and-memory">
  Invest in documentation and memory
</h3>

Ti consigliamo vivamente di investire nella documentazione in modo che Claude Code comprenda il tuo codebase. Le organizzazioni possono distribuire file CLAUDE.md a più livelli:

* **A livello organizzativo**: Distribuisci a directory di sistema come `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS), `/etc/claude-code/CLAUDE.md` (Linux e WSL), o `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) per gli standard a livello aziendale
* **A livello di repository**: Crea file `CLAUDE.md` nelle radici dei repository contenenti l'architettura del progetto, i comandi di build e le linee guida per i contributi. Archivialo nel controllo del codice sorgente in modo che tutti gli utenti ne traggano beneficio

Scopri di più in [Memory and CLAUDE.md files](/docs/it/memory).

<h3 id="simplify-deployment">
  Simplify deployment
</h3>

Se hai un ambiente di sviluppo personalizzato, riteniamo che creare un modo "one click" per installare Claude Code sia fondamentale per aumentare l'adozione in tutta l'organizzazione.

<h3 id="start-with-guided-usage">
  Start with guided usage
</h3>

Incoraggia i nuovi utenti a provare Claude Code per domande e risposte sul codebase, o su correzioni di bug più piccole o richieste di funzionalità. Chiedi a Claude Code di fare un piano. Controlla i suggerimenti di Claude e fornisci feedback se è fuori strada. Nel tempo, man mano che gli utenti comprendono meglio questo nuovo paradigma, saranno più efficaci nel permettere a Claude Code di funzionare in modo più agentico.

<h3 id="pin-model-versions-for-cloud-providers">
  Pin model versions for cloud providers
</h3>

Se distribuisci tramite [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry), o [Claude Platform on AWS](/docs/it/claude-platform-on-aws), fissa versioni specifiche dei modelli utilizzando `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, e `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Senza fissare, gli alias dei modelli si risolvono nel valore predefinito integrato di Claude Code per quel provider, che può essere in ritardo rispetto alla versione più recente e potrebbe non essere ancora abilitato nel tuo account. Fissare le versioni ti consente di controllare quando i tuoi utenti passano a un nuovo modello. Vedi [Model configuration](/docs/it/model-config#pin-models-for-third-party-deployments) per ciò che ogni provider fa quando il valore predefinito non è disponibile.

<h3 id="configure-security-policies">
  Configure security policies
</h3>

I team di sicurezza possono configurare autorizzazioni gestite per ciò che Claude Code è e non è autorizzato a fare, che non può essere sovrascritto dalla configurazione locale. [Scopri di più](/docs/it/security).

<h3 id="leverage-mcp-for-integrations">
  Leverage MCP for integrations
</h3>

MCP è un ottimo modo per fornire a Claude Code più informazioni, come la connessione a sistemi di gestione dei ticket o log degli errori. Ti consigliamo che un team centrale configuri i server MCP e archivi una configurazione `.mcp.json` nel codebase in modo che tutti gli utenti ne traggano beneficio. [Scopri di più](/docs/it/mcp).

In Anthropic, confidiamo in Claude Code per alimentare lo sviluppo in ogni codebase Anthropic. Speriamo che tu apprezzi l'utilizzo di Claude Code tanto quanto lo facciamo noi.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Una volta scelto un'opzione di distribuzione e configurato l'accesso per il tuo team:

1. **Distribuisci al tuo team**: Condividi le istruzioni di installazione e fai in modo che i membri del team [installino Claude Code](/docs/it/setup) e si autentichino con le loro credenziali.
2. **Configura la configurazione condivisa**: Crea un [file CLAUDE.md](/docs/it/memory) nei tuoi repository per aiutare Claude Code a comprendere il tuo codebase e gli standard di codifica.
3. **Configura le autorizzazioni**: Rivedi le [impostazioni di sicurezza](/docs/it/security) per definire cosa Claude Code può e non può fare nel tuo ambiente.
