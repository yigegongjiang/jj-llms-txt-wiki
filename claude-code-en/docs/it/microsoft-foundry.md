> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code su Microsoft Foundry

> Scopri come configurare Claude Code tramite Microsoft Foundry, inclusi setup, configurazione e risoluzione dei problemi.

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
  Prerequisiti
</h2>

Prima di configurare Claude Code con Microsoft Foundry, assicurati di avere:

* Un abbonamento Azure con accesso a Microsoft Foundry
* Autorizzazioni RBAC per creare risorse e distribuzioni di Microsoft Foundry
* Azure CLI installato e configurato (facoltativo - necessario solo se non hai un altro meccanismo per ottenere le credenziali)

<Note>
  Se stai distribuendo Claude Code a più utenti, [fissa le versioni del tuo modello](#4-pin-model-versions) prima di implementare.
</Note>

<h2 id="setup">
  Setup
</h2>

<h3 id="1-provision-microsoft-foundry-resource">
  1. Provision Microsoft Foundry resource
</h3>

Per prima cosa, crea una risorsa Claude in Azure:

1. Accedi al [portale Microsoft Foundry](https://ai.azure.com/)
2. Crea una nuova risorsa, annotando il nome della risorsa
3. Crea distribuzioni per i modelli Claude, annotando il nome di distribuzione che assegni a ciascuno; imposterai questi nomi come variabili del modello nel passaggio 4:
   * Claude Opus
   * Claude Sonnet
   * Claude Haiku

<h3 id="2-configure-azure-credentials">
  2) Configure Azure credentials
</h3>

Claude Code supporta tre metodi di autenticazione per Microsoft Foundry. Scegli il metodo che meglio si adatta ai tuoi requisiti di sicurezza.

**Option A: API key authentication**

1. Accedi alla tua risorsa nel portale Microsoft Foundry
2. Vai alla sezione **Endpoints and keys**
3. Copia **API Key**
4. Imposta la variabile di ambiente, sostituendo `your-azure-api-key` con la chiave che hai copiato:

```bash theme={null}
export ANTHROPIC_FOUNDRY_API_KEY=your-azure-api-key
```

**Option B: Microsoft Entra ID authentication**

Quando né `ANTHROPIC_FOUNDRY_API_KEY` né `ANTHROPIC_FOUNDRY_AUTH_TOKEN` sono impostati, Claude Code utilizza automaticamente la [catena di credenziali predefinita](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) di Azure SDK.
Questo supporta una varietà di metodi per autenticare carichi di lavoro locali e remoti.

Negli ambienti locali, puoi comunemente utilizzare Azure CLI:

```bash theme={null}
az login
```

**Option C: Bearer token authentication**

Claude Code invia il valore di `ANTHROPIC_FOUNDRY_AUTH_TOKEN` su ogni richiesta come intestazione `Authorization: Bearer`. Utilizza questa opzione quando un altro processo, come un'applicazione host o uno script di accesso, ha già ottenuto un token di accesso per te. Richiede Claude Code v2.1.203 o successivo.

Imposta la variabile su un token bearer emesso da Microsoft Entra ID per la tua risorsa:

```bash theme={null}
export ANTHROPIC_FOUNDRY_AUTH_TOKEN=your-entra-access-token
```

`ANTHROPIC_FOUNDRY_AUTH_TOKEN` ha la precedenza su `ANTHROPIC_FOUNDRY_API_KEY` e sulla catena di credenziali predefinita.

<Note>
  Quando si utilizza Microsoft Foundry, il comando `/logout` non è disponibile poiché l'autenticazione viene gestita tramite le credenziali Azure.
</Note>

<h3 id="3-configure-claude-code">
  3. Configure Claude Code
</h3>

Imposta le seguenti variabili di ambiente per abilitare Microsoft Foundry:

```bash theme={null}
# Enable Microsoft Foundry integration
export CLAUDE_CODE_USE_FOUNDRY=1

# Azure resource name (replace {resource} with your resource name)
export ANTHROPIC_FOUNDRY_RESOURCE={resource}
# Or provide the full base URL:
# export ANTHROPIC_FOUNDRY_BASE_URL=https://{resource}.services.ai.azure.com/anthropic
```

<h3 id="4-pin-model-versions">
  4. Pin model versions
</h3>

<Warning>
  Fissa versioni specifiche del modello per ogni distribuzione. Senza fissare, gli alias di modello come `sonnet` e `opus` si risolvono nel valore predefinito integrato di Claude Code per Microsoft Foundry, che può essere in ritardo rispetto alla versione più recente e potrebbe non essere ancora disponibile nel tuo account. Microsoft Foundry non ha un controllo del modello all'avvio, quindi le richieste non riescono quando il valore predefinito non è disponibile. Quando crei distribuzioni Azure, seleziona una versione di modello specifica piuttosto che "aggiornamento automatico alla versione più recente".
</Warning>

Imposta le variabili del modello in modo che corrispondano ai nomi di distribuzione che hai creato nel passaggio 1.

Senza `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` su Microsoft Foundry si risolve in Opus 4.6. Impostalo sull'ID di Opus 4.8 per utilizzare il modello più recente:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5'
```

I compiti in background come la generazione del titolo della sessione utilizzano il modello piccolo/veloce, normalmente un modello della classe Haiku. Su Microsoft Foundry, Claude Code utilizza per impostazione predefinita il modello primario perché non tutti gli account hanno una distribuzione Haiku. Per utilizzare Haiku per i compiti in background, imposta `ANTHROPIC_DEFAULT_HAIKU_MODEL` su una distribuzione Haiku disponibile nel tuo account, come mostrato sopra.

Per gli ID dei modelli attuali e legacy, vedi [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview). Vedi [Model configuration](/docs/it/model-config#pin-models-for-third-party-deployments) per l'elenco completo delle variabili di ambiente.

[Prompt caching](/docs/it/prompt-caching) è abilitato automaticamente. Per richiedere un TTL della cache di 1 ora invece del valore predefinito di 5 minuti, imposta la seguente variabile; le scritture della cache con un TTL di 1 ora vengono fatturate a una tariffa più elevata:

```bash theme={null}
export ENABLE_PROMPT_CACHING_1H=1
```

<h3 id="5-run-claude-code">
  5. Run Claude Code
</h3>

Con le variabili di ambiente impostate, avvia Claude Code dalla directory del tuo progetto:

```bash theme={null}
claude
```

Claude Code legge `CLAUDE_CODE_USE_FOUNDRY` e le altre variabili Microsoft Foundry dall'ambiente e si connette alla tua risorsa Azure al primo prompt. A differenza di Amazon Bedrock e Google Cloud's Agent Platform, Microsoft Foundry non ha una procedura guidata di configurazione interattiva, quindi le variabili di ambiente nei passaggi 3 e 4 sono l'unico percorso di configurazione.

Per verificare la tua configurazione, esegui `/status` all'interno di Claude Code. La riga del provider API mostra `Microsoft Foundry`, insieme al nome della risorsa o all'URL di base che hai configurato.

<h2 id="azure-rbac-configuration">
  Configurazione RBAC di Azure
</h2>

I ruoli predefiniti `Azure AI User` e `Cognitive Services User` includono tutte le autorizzazioni necessarie per invocare i modelli Claude.

Per autorizzazioni più restrittive, crea un ruolo personalizzato con quanto segue:

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

Per i dettagli, vedi [Documentazione RBAC di Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/concepts/rbac-azure-ai-foundry).

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Se ricevi un errore "Failed to get token from azureADTokenProvider: ChainedTokenCredential authentication failed":

* Configura Entra ID nell'ambiente, oppure imposta `ANTHROPIC_FOUNDRY_API_KEY`.

Se le richieste non riescono con errori di connessione ripetuti al primo prompt:

* Verifica che `ANTHROPIC_FOUNDRY_RESOURCE` sia impostato sul nome della risorsa effettivo anziché su un segnaposto. Claude Code costruisce l'URL dell'endpoint da questo valore, quindi un nome non corretto punta a un host che non esiste.

<h2 id="additional-resources">
  Risorse aggiuntive
</h2>

* [Documentazione di Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/what-is-azure-ai-foundry)
* [Modelli di Microsoft Foundry](https://ai.azure.com/explore/models)
* [Prezzi di Microsoft Foundry](https://azure.microsoft.com/en-us/pricing/details/ai-foundry/)
