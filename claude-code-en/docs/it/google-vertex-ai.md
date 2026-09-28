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
    Esegui `claude`. Al prompt di accesso, seleziona **3rd-party platform**, quindi **Google Vertex AI**, l'etichetta che il prompt di accesso utilizza ancora per Google Cloud's Agent Platform. Se sei già connesso, esegui `/login` per aprire lo stesso menu.
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

Abilita Google Cloud's Agent Platform API nel tuo progetto GCP. Sostituisci `YOUR-PROJECT-ID` con il tuo ID progetto GCP qui e nel passaggio di configurazione di seguito:

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

Claude Code supporta [X.509 certificate-based Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) attraverso la stessa catena Application Default Credentials. Imposta `GOOGLE_APPLICATION_CREDENTIALS` al percorso del tuo file di configurazione delle credenziali.

<Note>
  Claude Code indirizza le richieste Google Cloud's Agent Platform al progetto in `ANTHROPIC_VERTEX_PROJECT_ID`, anche quando `GCLOUD_PROJECT`, `GOOGLE_CLOUD_PROJECT`, o il file di credenziali a cui fa riferimento `GOOGLE_APPLICATION_CREDENTIALS` contiene un progetto diverso.
</Note>

<h4 id="advanced-credential-configuration">
  Configurazione avanzata delle credenziali
</h4>

Claude Code supporta l'aggiornamento automatico delle credenziali GCP tramite l'impostazione `gcpAuthRefresh`. Aggiungilo al tuo file di [impostazioni](/docs/it/settings) di Claude Code, ad esempio `~/.claude/settings.json`. Quando Claude Code rileva che le tue credenziali GCP sono scadute o non possono essere caricate, esegue il comando configurato per ottenere nuove credenziali prima di riprovare la richiesta.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

Prima di eseguire il comando, Claude Code richiede un token di accesso con le tue credenziali attuali per confermare che sono effettivamente scadute, e salta il comando quando funzionano ancora.

Se il controllo non termina entro cinque secondi, Claude Code salta anche il comando e lo esegue solo dopo che una richiesta non riesce con un errore di credenziale. Prima della v2.1.261, un controllo che scadeva contava come una credenziale scaduta, quindi il comando potrebbe aprire il tuo browser all'avvio anche se le tue credenziali erano ancora valide.

Claude Code ti mostra l'output del comando, ma non può inviare input interattivo al comando. Questo funziona bene per i flussi di autenticazione basati su browser in cui la CLI mostra un URL e completi l'autenticazione nel browser. Il comando di aggiornamento scade dopo tre minuti se l'autenticazione non viene completata. Se imposti `gcpAuthRefresh` nelle impostazioni del progetto come `.claude/settings.json`, Claude Code lo esegue secondo la stessa [regola di fiducia dell'area di lavoro degli hook nei file di impostazioni](/docs/it/permissions#what-runs-before-you-trust-a-folder), che include le sessioni `-p` nelle cartelle che non hai mai considerato attendibili.

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

# Quando CLOUD_ML_REGION=global, esegui l'override della regione per i modelli che non supportano endpoint globali
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

La maggior parte delle versioni del modello ha una variabile `VERTEX_REGION_CLAUDE_*` corrispondente. Consulta il [riferimento delle variabili di ambiente](/docs/it/env-vars) per l'elenco completo. Controlla [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) per determinare quali modelli supportano endpoint globali rispetto a quelli solo regionali.

Se un valore di regione non ha la forma di un nome di regione o posizione, Claude Code lo tratta come non impostato. Ad esempio, Claude Code tratta un valore contenente una barra, un punto o uno spazio come non impostato. Claude Code ritorna a una fonte diversa per ogni variabile:

* `VERTEX_REGION_CLAUDE_*`: Claude Code ritorna a `CLOUD_ML_REGION`.
* `CLOUD_ML_REGION`: Claude Code ritorna a `us-east5`.

[Prompt caching](/docs/it/prompt-caching) è abilitato automaticamente. Per disabilitarlo, imposta `DISABLE_PROMPT_CACHING=1`. Per richiedere un TTL cache di 1 ora invece del valore predefinito di 5 minuti, imposta `ENABLE_PROMPT_CACHING_1H=1`; le scritture della cache con TTL di 1 ora vengono fatturate a una tariffa più elevata. Per impostare TTL diversi per la tua conversazione principale e per le richieste che Claude Code effettua al di fuori di essa, [scegli il TTL tu stesso](/docs/it/prompt-caching#choose-the-ttl-yourself).

Per aumentare i tuoi limiti di velocità, contatta il supporto di Google Cloud. Quando utilizzi Google Cloud's Agent Platform, il comando `/logout` non è disponibile poiché l'autenticazione è gestita tramite le credenziali di Google Cloud.

Claude Code decide tra [MCP tool search](/docs/it/mcp#scale-with-mcp-tool-search) e caricamento anticipato per generazione del modello:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 e versioni successive**: Claude Code abilita la ricerca degli strumenti per impostazione predefinita.
* **Modelli precedenti, inclusi tutti i modelli Claude 3.x**: Claude Code carica le definizioni degli strumenti MCP in anticipo, perché i loro stack di servizio Agent Platform rifiutano l'intestazione beta richiesta. L'impostazione di `ENABLE_TOOL_SEARCH=true` non esegue l'override di questo.

Imposta `ENABLE_TOOL_SEARCH=false` per disabilitare la ricerca degli strumenti su ogni modello. Prima della v2.1.221, Claude Code disabilitava la ricerca degli strumenti per tutti i modelli su Google Cloud's Agent Platform a meno che non impostassi `ENABLE_TOOL_SEARCH=true`.

<h3 id="5-pin-model-versions">
  5. Fissa le versioni del modello
</h3>

<Warning>
  Fissa versioni specifiche del modello quando distribuisci a più utenti. Senza fissaggio, gli alias di modello come `sonnet` e `opus` si risolvono nel valore predefinito integrato di Claude Code per Google Cloud's Agent Platform, che può essere in ritardo rispetto alla versione più recente e potrebbe non essere ancora abilitato nel tuo progetto. Claude Code [ritorna](#startup-model-checks) a un modello precedente o di livello inferiore all'avvio quando il valore predefinito non è disponibile, ma il fissaggio ti consente di controllare quando i tuoi utenti passano a un nuovo modello.
</Warning>

Imposta queste variabili di ambiente su ID modello Google Cloud's Agent Platform specifici.

Senza `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` su Google Cloud's Agent Platform si risolve in Opus 5.5, e senza `ANTHROPIC_DEFAULT_SONNET_MODEL`, l'alias `sonnet` si risolve in Sonnet 4.5. Questo esempio fissa ogni alias a una versione specifica:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Per gli ID modello attuali e legacy, consulta [Panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview). Consulta [Configurazione del modello](/docs/it/model-config#pin-models-for-third-party-deployments) per l'elenco completo delle variabili di ambiente.

Claude Code utilizza questi modelli predefiniti quando nessuna variabile di fissaggio è impostata:

| Tipo di modello        | Valore predefinito           |
| :--------------------- | :--------------------------- |
| Modello primario       | `claude-opus-5-5`            |
| Modello piccolo/veloce | `claude-sonnet-4-5@20250929` |

Le attività in background come la generazione del titolo della sessione utilizzano il modello piccolo/veloce, normalmente un modello della classe Haiku. Su Google Cloud's Agent Platform, Claude Code utilizza il modello Sonnet predefinito per le attività in background perché Haiku potrebbe non essere abilitato in ogni progetto o regione. Due selezioni cambiano quale modello le esegue:

* Quando selezioni un modello primario con `--model`, `ANTHROPIC_MODEL`, o l'impostazione `model`, le attività in background utilizzano quel modello. Quando Claude Code avvia la sessione sul modello che hai impostato con [`ANTHROPIC_DEFAULT_MODEL`](/docs/it/model-config#set-a-default-model-for-new-sessions), le attività in background utilizzano quel modello anche. L'impostazione di `ANTHROPIC_DEFAULT_OPUS_MODEL` senza `ANTHROPIC_DEFAULT_SONNET_MODEL` conta anche come una selezione, perché il modello Sonnet integrato potrebbe non essere abilitato in un progetto che indirizza il suo Opus.
* Per utilizzare Haiku per le attività in background, imposta `ANTHROPIC_DEFAULT_HAIKU_MODEL` su un ID modello disponibile nel tuo progetto.

<Warning>
  I modelli Opus hanno un prezzo per token più elevato rispetto ai modelli Sonnet, quindi una distribuzione che non fissa un modello primario viene fatturata alla tariffa Opus una volta che si aggiorna a v2.1.207 o successiva. Per mantenere Sonnet 4.5 come modello primario, imposta `ANTHROPIC_MODEL` al suo ID modello completo. Una distribuzione che indirizza il valore predefinito con `ANTHROPIC_DEFAULT_SONNET_MODEL` e non imposta `ANTHROPIC_DEFAULT_OPUS_MODEL` mantiene il suo modello Sonnet indirizzato come predefinito.
</Warning>

Prima della v2.1.280, il modello primario su Google Cloud's Agent Platform era predefinito a Opus 5 e l'alias `opus` si risolveva in Opus 5 da v2.1.219. Su v2.1.207 attraverso v2.1.218, il modello primario su Google Cloud's Agent Platform era predefinito a Opus 4.8 e l'alias `opus` si risolveva in Opus 4.8. Prima della v2.1.207, il modello primario era predefinito a Sonnet 4.5, l'alias `opus` si risolveva in Opus 4.6, e le attività in background utilizzavano sempre il modello primario.

Per personalizzare ulteriormente i modelli:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h3 id="6-verify-your-configuration">
  6. Verifica la tua configurazione
</h3>

Avvia Claude Code ed esegui `/status` per confermare la configurazione. La riga `API provider` mostra `Google Vertex AI`, e le righe `GCP project`, `Default region`, e `Model` mostrano il tuo ID progetto, regione e modello risolto. Se la riga del provider è mancante, le variabili di ambiente non stanno raggiungendo il processo. Conferma che siano esportate nella shell in cui hai lanciato `claude`, o impostale nel blocco `env` del tuo [file di impostazioni](/docs/it/settings).

<h2 id="startup-model-checks">
  Controlli del modello all'avvio
</h2>

Quando Claude Code si avvia con Google Cloud's Agent Platform configurato, verifica che i modelli che intende utilizzare siano accessibili nel tuo progetto.

Se hai fissato una versione del modello più vecchia del valore predefinito corrente di Claude Code, e il tuo progetto può invocare la versione più recente, Claude Code ti chiede di aggiornare il fissaggio. Accettare scrive il nuovo ID modello nel tuo [file di impostazioni utente](/docs/it/settings) e riavvia Claude Code. Rifiutare viene ricordato fino al prossimo cambio di versione predefinita.

Se non hai fissato un modello e il valore predefinito corrente non è disponibile nel tuo progetto, Claude Code ritorna alla versione precedente per la sessione corrente e mostra un avviso. Prova le versioni precedenti del modello predefinito per primo e, quando il valore predefinito è un modello Opus e nessuna versione Opus è disponibile, ritorna al modello Sonnet predefinito. Il ritorno non è persistente. Abilita il modello più recente in [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) o [fissa una versione](#5-pin-model-versions) per rendere la scelta permanente.

Quando avvii la sessione su una versione specifica di Sonnet oppure Opus, ad esempio con `--model`, `ANTHROPIC_MODEL`, o l'[impostazione `model`](/docs/it/settings-reference#model), quella versione agisce come valore predefinito fissato della sessione per l'alias `sonnet` oppure `opus` corrispondente. Claude Code salta il controllo di disponibilità per il valore predefinito integrato che il tuo modello sostituisce e si avvia sul modello che hai configurato, senza alcun avviso di fallback.

Gli alias dei modelli come `opus` non agiscono come fissaggi, e nemmeno un ID modello che Claude Code non riconosce.

<h2 id="iam-configuration">
  Configurazione IAM
</h2>

Assegna il ruolo `roles/aiplatform.user`, che include le autorizzazioni richieste:

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
