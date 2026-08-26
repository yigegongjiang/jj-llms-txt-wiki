> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code su Google Cloud's Agent Platform

> Scopri come configurare Claude Code tramite Google Cloud's Agent Platform, precedentemente Vertex AI, inclusa la configurazione, la configurazione IAM e la risoluzione dei problemi.

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
  Prerequisiti
</h2>

Prima di configurare Claude Code con Google Cloud's Agent Platform di Google Cloud, precedentemente noto come Vertex AI, assicurati di avere:

* Un account Google Cloud Platform (GCP) con fatturazione abilitata
* Un progetto GCP con Google Cloud's Agent Platform API abilitata
* Accesso ai modelli Claude desiderati (ad esempio, Claude Sonnet 4.6)
* Google Cloud SDK (`gcloud`) installato e configurato
* Quota allocata nella regione GCP desiderata

Per accedere con le tue credenziali Google Cloud's Agent Platform, segui [Accedi con Google Cloud's Agent Platform](#sign-in-with-agent-platform) di seguito. Per distribuire Claude Code in un team, utilizza i passaggi di [configurazione manuale](#set-up-manually) e [fissa le versioni del tuo modello](#5-pin-model-versions) prima del rollout.

<h2 id="sign-in-with-agent-platform">
  Accedi con Agent Platform
</h2>

Se hai credenziali Google Cloud e desideri iniziare a utilizzare Claude Code tramite Google Cloud's Agent Platform, la procedura guidata di accesso ti guida attraverso i passaggi. Completi i prerequisiti lato GCP una volta per progetto; la procedura guidata gestisce il lato Claude Code.

<Steps>
  <Step title="Abilita i modelli Claude nel tuo progetto GCP">
    [Abilita Google Cloud's Agent Platform API](#1-enable-agent-platform-api) per il tuo progetto, quindi richiedi accesso ai modelli Claude che desideri in [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden). Consulta [Configurazione IAM](#iam-configuration) per le autorizzazioni di cui il tuo account ha bisogno.
  </Step>

  <Step title="Avvia Claude Code e scegli Google Cloud's Agent Platform">
    Esegui `claude`. Al prompt di accesso, seleziona **3rd-party platform**, quindi **Google Vertex AI**, l'etichetta che il prompt di accesso utilizza ancora per Google Cloud's Agent Platform.
  </Step>

  <Step title="Segui i prompt della procedura guidata">
    Scegli come autenticarti a Google Cloud: Application Default Credentials da `gcloud`, un file di chiave dell'account di servizio, o credenziali già presenti nel tuo ambiente. La procedura guidata rileva il tuo progetto e la tua regione, verifica quali modelli Claude il tuo progetto può invocare, e ti consente di fissarli. Salva il risultato nel blocco `env` del tuo [file di impostazioni utente](/docs/it/settings), quindi non è necessario esportare variabili di ambiente da solo.
  </Step>
</Steps>

Dopo aver effettuato l'accesso, esegui `/setup-vertex` in qualsiasi momento per riaprire la procedura guidata e modificare le tue credenziali, progetto, regione o fissaggi di modello. Il passaggio di fissaggio del modello inizia dai tuoi modelli attualmente fissati. La procedura guidata scrive in `~/.claude/settings.json`, o in `$CLAUDE_CONFIG_DIR/settings.json` quando [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars#variables) è impostato.

<h2 id="region-configuration">
  Configurazione della regione
</h2>

Claude Code supporta endpoint di Google Cloud's Agent Platform [globali](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), multi-regione e regionali. Imposta `CLOUD_ML_REGION` su `global`, una posizione multi-regione come `eu` o `us`, o una regione specifica come `us-east5`. Claude Code seleziona il nome host corretto di Google Cloud's Agent Platform per ogni modulo, inclusi gli host `aiplatform.eu.rep.googleapis.com` e `aiplatform.us.rep.googleapis.com` per le posizioni multi-regione.

<Note>
  Google Cloud's Agent Platform potrebbe non supportare i modelli predefiniti di Claude Code su ogni tipo di endpoint. La disponibilità del modello varia tra [regioni specifiche](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), posizioni multi-regione e [endpoint globali](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models). Potrebbe essere necessario passare a una posizione supportata o specificare un modello supportato.
</Note>

<h2 id="set-up-manually">
  Configurazione manuale
</h2>

Per configurare Google Cloud's Agent Platform tramite variabili di ambiente invece della procedura guidata, ad esempio in CI o in un rollout aziendale con script, segui i passaggi di seguito.

<h3 id="1-enable-agent-platform-api">
  1. Abilita Agent Platform API
</h3>

Abilita Google Cloud's Agent Platform API nel tuo progetto GCP:

```bash theme={null}
# Imposta il tuo ID progetto
gcloud config set project YOUR-PROJECT-ID

# Abilita Agent Platform API
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. Richiedi accesso al modello
</h3>

Richiedi accesso ai modelli Claude in Google Cloud's Agent Platform:

1. Accedi a [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. Cerca i modelli "Claude"
3. Richiedi accesso ai modelli Claude desiderati (ad esempio, Claude Sonnet 4.6)
4. Attendi l'approvazione (potrebbe richiedere 24-48 ore)

<h3 id="3-configure-gcp-credentials">
  3) Configura le credenziali GCP
</h3>

Claude Code utilizza l'autenticazione standard di Google Cloud.

Per ulteriori informazioni, consulta la [documentazione di autenticazione di Google Cloud](https://cloud.google.com/docs/authentication).

Claude Code v2.1.121 o versioni successive supporta [X.509 certificate-based Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) attraverso la stessa catena Application Default Credentials. Imposta `GOOGLE_APPLICATION_CREDENTIALS` al percorso del tuo file di configurazione delle credenziali.

<Note>
  Claude Code utilizza `ANTHROPIC_VERTEX_PROJECT_ID` come ID progetto per le richieste Google Cloud's Agent Platform. Le variabili di ambiente `GCLOUD_PROJECT` e `GOOGLE_CLOUD_PROJECT` e il file di credenziali a cui fa riferimento `GOOGLE_APPLICATION_CREDENTIALS` hanno la precedenza su di esso. Se nessuno di questi è impostato, l'ID progetto viene risolto dalla tua configurazione `gcloud` o dall'account di servizio collegato.
</Note>

<h4 id="advanced-credential-configuration">
  Configurazione avanzata delle credenziali
</h4>

Claude Code supporta l'aggiornamento automatico delle credenziali GCP tramite l'impostazione `gcpAuthRefresh`. Quando Claude Code rileva che le tue credenziali GCP sono scadute o non possono essere caricate, esegue il comando configurato per ottenere nuove credenziali prima di riprovare la richiesta.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

L'output del comando viene visualizzato all'utente, ma l'input interattivo non è supportato. Questo funziona bene per i flussi di autenticazione basati su browser in cui la CLI mostra un URL e completi l'autenticazione nel browser. Il comando di aggiornamento scade dopo tre minuti se l'autenticazione non viene completata. Se imposti `gcpAuthRefresh` nelle impostazioni del progetto come `.claude/settings.json`, il comando viene eseguito solo dopo che accetti il prompt di fiducia dell'area di lavoro.

<h3 id="4-configure-claude-code">
  4. Configura Claude Code
</h3>

Imposta le seguenti variabili di ambiente:

```bash theme={null}
# Abilita integrazione Agent Platform
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# Facoltativo: Esegui l'override dell'URL dell'endpoint Agent Platform per endpoint personalizzati o gateway
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# Facoltativo: Disabilita prompt caching se necessario
export DISABLE_PROMPT_CACHING=1

# Facoltativo: Richiedi TTL cache prompt di 1 ora invece del valore predefinito di 5 minuti
export ENABLE_PROMPT_CACHING_1H=1

# Quando CLOUD_ML_REGION=global, esegui l'override della regione per i modelli che non supportano endpoint globali
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

La maggior parte delle versioni del modello ha una variabile `VERTEX_REGION_CLAUDE_*` corrispondente. Consulta il [riferimento delle variabili di ambiente](/docs/it/env-vars) per l'elenco completo. Controlla [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) per determinare quali modelli supportano endpoint globali rispetto a quelli solo regionali.

[Prompt caching](/docs/it/prompt-caching) è abilitato automaticamente. Per disabilitarlo, imposta `DISABLE_PROMPT_CACHING=1`. Per richiedere un TTL cache di 1 ora invece del valore predefinito di 5 minuti, imposta `ENABLE_PROMPT_CACHING_1H=1`; le scritture della cache con TTL di 1 ora vengono fatturate a una tariffa più elevata. Per limiti di velocità aumentati, contatta il supporto di Google Cloud. Quando utilizzi Google Cloud's Agent Platform, il comando `/logout` non è disponibile poiché l'autenticazione è gestita tramite le credenziali di Google Cloud.

Claude Code disabilita [MCP tool search](/docs/it/mcp#scale-with-mcp-tool-search) per impostazione predefinita su Google Cloud's Agent Platform, quindi le definizioni degli strumenti MCP vengono caricate in anticipo. Google Cloud's Agent Platform supporta la ricerca degli strumenti per Claude Sonnet 4.5 e versioni successive e Claude Opus 4.5 e versioni successive. Imposta `ENABLE_TOOL_SEARCH=true` per abilitarla su questi modelli. I modelli precedenti su Google Cloud's Agent Platform non accettano l'intestazione beta richiesta e le richieste non riescono se abiliti la ricerca degli strumenti con essi.

<h3 id="5-pin-model-versions">
  5. Fissa le versioni del modello
</h3>

<Warning>
  Fissa versioni specifiche del modello quando distribuisci a più utenti. Senza fissaggio, gli alias di modello come `sonnet` e `opus` si risolvono nel valore predefinito integrato di Claude Code per Google Cloud's Agent Platform, che può essere in ritardo rispetto alla versione più recente e potrebbe non essere ancora abilitato nel tuo progetto. Claude Code [ritorna](#startup-model-checks) a un modello precedente o di livello inferiore all'avvio quando il valore predefinito non è disponibile, ma il fissaggio ti consente di controllare quando i tuoi utenti passano a un nuovo modello.
</Warning>

Imposta queste variabili di ambiente su ID modello Google Cloud's Agent Platform specifici.

Senza `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` su Google Cloud's Agent Platform si risolve in Opus 4.8, e senza `ANTHROPIC_DEFAULT_SONNET_MODEL`, l'alias `sonnet` si risolve in Sonnet 4.5. Questo esempio fissa ogni alias a una versione specifica:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Per gli ID modello attuali e legacy, consulta [Panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview). Consulta [Configurazione del modello](/docs/it/model-config#pin-models-for-third-party-deployments) per l'elenco completo delle variabili di ambiente.

Claude Code utilizza questi modelli predefiniti quando nessuna variabile di fissaggio è impostata:

| Tipo di modello        | Valore predefinito           |
| :--------------------- | :--------------------------- |
| Modello primario       | `claude-opus-4-8`            |
| Modello piccolo/veloce | `claude-sonnet-4-5@20250929` |

Le attività in background come la generazione del titolo della sessione utilizzano il modello piccolo/veloce, normalmente un modello della classe Haiku. Su Google Cloud's Agent Platform, Claude Code utilizza il modello Sonnet predefinito per le attività in background perché Haiku potrebbe non essere abilitato in ogni progetto o regione. Due selezioni cambiano quale modello le esegue:

* Quando selezioni un modello primario con `--model`, `ANTHROPIC_MODEL`, o l'impostazione `model`, le attività in background utilizzano quel modello. Impostare `ANTHROPIC_DEFAULT_OPUS_MODEL` senza `ANTHROPIC_DEFAULT_SONNET_MODEL` conta come una selezione anche, perché il modello Sonnet integrato potrebbe non essere abilitato in un progetto che indirizza il suo Opus.
* Per utilizzare Haiku per le attività in background, imposta `ANTHROPIC_DEFAULT_HAIKU_MODEL` su un ID modello disponibile nel tuo progetto.

<Warning>
  I modelli Opus hanno un prezzo per token più elevato rispetto ai modelli Sonnet, quindi una distribuzione che non fissa un modello primario viene fatturata alla tariffa Opus una volta che si aggiorna a v2.1.207 o successiva. Per mantenere Sonnet 4.5 come modello primario, imposta `ANTHROPIC_MODEL` al suo ID modello completo. Una distribuzione che indirizza il valore predefinito con `ANTHROPIC_DEFAULT_SONNET_MODEL` e non imposta `ANTHROPIC_DEFAULT_OPUS_MODEL` mantiene il suo modello Sonnet indirizzato come predefinito.
</Warning>

Prima di v2.1.207, il modello primario su Google Cloud's Agent Platform era predefinito a Sonnet 4.5, l'alias `opus` si risolveva in Opus 4.6, e le attività in background utilizzavano sempre il modello primario.

Per personalizzare ulteriormente i modelli:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h2 id="startup-model-checks">
  Controlli del modello all'avvio
</h2>

Quando Claude Code si avvia con Google Cloud's Agent Platform configurato, verifica che i modelli che intende utilizzare siano accessibili nel tuo progetto.

Se hai fissato una versione del modello più vecchia del valore predefinito corrente di Claude Code, e il tuo progetto può invocare la versione più recente, Claude Code ti chiede di aggiornare il fissaggio. Accettare scrive il nuovo ID modello nel tuo [file di impostazioni utente](/docs/it/settings) e riavvia Claude Code. Rifiutare viene ricordato fino al prossimo cambio di versione predefinita.

Se non hai fissato un modello e il valore predefinito corrente non è disponibile nel tuo progetto, Claude Code ritorna alla versione precedente per la sessione corrente e mostra un avviso. Prova le versioni precedenti del modello predefinito per primo e, quando il valore predefinito è un modello Opus e nessuna versione Opus è disponibile, ritorna al modello Sonnet predefinito. Il ritorno non è persistente. Abilita il modello più recente in [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) o [fissa una versione](#5-pin-model-versions) per rendere la scelta permanente.

<h2 id="iam-configuration">
  Configurazione IAM
</h2>

Assegna le autorizzazioni IAM richieste:

Il ruolo `roles/aiplatform.user` include le autorizzazioni richieste:

* `aiplatform.endpoints.predict` - Richiesto per l'invocazione del modello e il conteggio dei token

Per autorizzazioni più restrittive, crea un ruolo personalizzato con solo le autorizzazioni di cui sopra.

Per i dettagli, consulta la [documentazione IAM di Google Cloud Vertex AI](https://cloud.google.com/vertex-ai/docs/general/access-control).

<Note>
  Crea un progetto GCP dedicato per Claude Code per semplificare il tracciamento dei costi e il controllo degli accessi.
</Note>

<h2 id="1m-token-context-window">
  Finestra di contesto da 1M token
</h2>

Claude Sonnet 5, Opus 4.6 e versioni successive, e Sonnet 4.6 supportano la [finestra di contesto da 1M token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) su Google Cloud's Agent Platform. Sonnet 5 funziona sempre con la finestra da 1M, senza alcuna variante `[1m]` da selezionare. Per gli altri modelli, Claude Code abilita automaticamente la finestra di contesto estesa quando selezioni una variante di modello 1M.

La [procedura guidata di configurazione](#sign-in-with-agent-platform) offre un'opzione di contesto 1M quando fissa i modelli. Per abilitarla per un modello fissato manualmente, aggiungi `[1m]` all'ID del modello. Consulta [Fissa i modelli per le distribuzioni di terze parti](/docs/it/model-config#pin-models-for-third-party-deployments) per i dettagli.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Se riscontri errori "Could not load the default credentials":

* Esegui `gcloud auth application-default login` per configurare le credenziali predefinite dell'applicazione
* Imposta `GOOGLE_APPLICATION_CREDENTIALS` su un percorso di file della chiave dell'account di servizio
* Vedi [Configure GCP credentials](#3-configure-gcp-credentials) per tutte le opzioni

Se riscontri problemi di quota:

* Controlla le quote attuali o richiedi un aumento della quota tramite [Cloud Console](https://cloud.google.com/docs/quotas/view-manage)

Se riscontri errori "model not found" 404:

* Conferma che il modello è abilitato in [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
* Verifica che il modello sia disponibile nella posizione che hai specificato. Alcuni modelli sono offerti solo su posizioni `global` o multi-regione come `eu` e `us`, non in regioni specifiche
* Se utilizzi `CLOUD_ML_REGION=global`, controlla che i tuoi modelli supportino endpoint globali in [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) in "Supported features". Per i modelli che non supportano endpoint globali, puoi:
  * Specificare un modello supportato tramite `ANTHROPIC_MODEL` o `ANTHROPIC_DEFAULT_HAIKU_MODEL`, oppure
  * Impostare una regione o una posizione multi-regione utilizzando le variabili di ambiente `VERTEX_REGION_<MODEL_NAME>`

Se riscontri errori 429:

* Per gli endpoint regionali, assicurati che il modello primario e il modello piccolo/veloce siano supportati nella tua regione selezionata
* Considera di passare a `CLOUD_ML_REGION=global` per una migliore disponibilità

<h2 id="additional-resources">
  Risorse aggiuntive
</h2>

* [Documentazione di Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs)
* [Prezzi di Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/pricing)
* [Quote e limiti di Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs/quotas)
