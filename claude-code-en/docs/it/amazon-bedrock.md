> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code su Amazon Bedrock

> Scopri come configurare Claude Code tramite Amazon Bedrock, inclusa la configurazione, la configurazione IAM e la risoluzione dei problemi.

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

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  Prerequisiti
</h2>

Prima di configurare Claude Code con Amazon Bedrock, assicurati di avere:

* Un account AWS con accesso a Amazon Bedrock abilitato
* Accesso ai modelli Claude desiderati (ad esempio, Claude Sonnet 4.6) in Amazon Bedrock
* AWS CLI installato e configurato (facoltativo - necessario solo se non hai un altro meccanismo per ottenere le credenziali)
* Autorizzazioni IAM appropriate

Per accedere con le tue credenziali Amazon Bedrock, segui [Accedi con Amazon Bedrock](#sign-in-with-bedrock) di seguito. Per distribuire Claude Code in un team, utilizza i passaggi di [configurazione manuale](#set-up-manually) e [fissa le versioni del tuo modello](#4-pin-model-versions) prima del rollout.

<h2 id="sign-in-with-bedrock">
  Accedi con Bedrock
</h2>

Se disponi di credenziali AWS e desideri iniziare a utilizzare Claude Code tramite Amazon Bedrock, la procedura guidata di accesso ti guida attraverso i passaggi. Completi i prerequisiti lato AWS una volta per account; la procedura guidata gestisce il lato Claude Code.

<Steps>
  <Step title="Abilita i modelli Anthropic nel tuo account AWS">
    Nella [console di Amazon Bedrock](https://console.aws.amazon.com/bedrock/), apri il catalogo dei modelli, seleziona un modello Anthropic e invia il modulo del caso d'uso. L'accesso viene concesso immediatamente dopo l'invio. Consulta [Invia i dettagli del caso d'uso](#1-submit-use-case-details) per AWS Organizations e [Configurazione IAM](#iam-configuration) per le autorizzazioni di cui il tuo ruolo ha bisogno.
  </Step>

  <Step title="Avvia Claude Code e scegli Amazon Bedrock">
    Esegui `claude`. Al prompt di accesso, seleziona **piattaforma di terze parti**, quindi **Amazon Bedrock**. Se sei già connesso e visualizzi il prompt della chat, esegui `/setup-bedrock` per aprire la procedura guidata. Finché `CLAUDE_CODE_USE_BEDROCK=1` non è impostato, Claude Code [nasconde il comando dal menu dei comandi](/docs/it/commands#how-the-command-menu-matches-what-you-type); digitalo per intero.
  </Step>

  <Step title="Segui i prompt della procedura guidata">
    Scegli come autenticarti ad AWS: un profilo AWS rilevato dalla tua directory `~/.aws`, una chiave API di Amazon Bedrock, una chiave di accesso e un segreto, o credenziali già presenti nel tuo ambiente. La procedura guidata chiede la tua regione, verifica quali modelli Claude il tuo account può invocare e ti consente di fissarli. Salva il risultato nel blocco `env` del tuo [file di impostazioni utente](/docs/it/settings), quindi non è necessario esportare variabili di ambiente da solo.
  </Step>
</Steps>

Dopo aver effettuato l'accesso, esegui `/setup-bedrock` in qualsiasi momento per riaprire la procedura guidata e modificare le tue credenziali, regione o pin dei modelli. Il passaggio del pin del modello inizia dai tuoi modelli attualmente fissati. La procedura guidata scrive in `~/.claude/settings.json`, o in `$CLAUDE_CONFIG_DIR/settings.json` quando [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars#variables) è impostato.

<h2 id="set-up-manually">
  Configurazione manuale
</h2>

Per configurare Amazon Bedrock tramite variabili di ambiente invece della procedura guidata, ad esempio in CI o in un rollout aziendale con script, seguire i passaggi di seguito.

<h3 id="1-submit-use-case-details">
  1. Inviare i dettagli del caso d'uso
</h3>

Prima di invocare un modello Anthropic per la prima volta, inviare i dettagli del caso d'uso. Questa operazione viene eseguita una volta per account AWS.

1. Assicurarsi di disporre delle autorizzazioni IAM corrette descritte di seguito
2. Navigare alla [console di Amazon Bedrock](https://console.aws.amazon.com/bedrock/)
3. Selezionare un modello Anthropic dal **Catalogo modelli**
4. Completare il modulo del caso d'uso. L'accesso viene concesso immediatamente dopo l'invio.

Se si utilizza AWS Organizations, è possibile inviare il modulo una volta dall'account di gestione utilizzando l'API [`PutUseCaseForModelAccess`](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html). Questa chiamata richiede l'autorizzazione IAM `bedrock:PutUseCaseForModelAccess`. L'approvazione si estende automaticamente agli account figlio.

<h3 id="2-configure-aws-credentials">
  2. Configurare le credenziali AWS
</h3>

Claude Code utilizza la catena di credenziali predefinita dell'SDK AWS. Configurare le credenziali utilizzando uno di questi metodi:

**Opzione A: Configurazione AWS CLI**

```bash theme={null}
aws configure
```

**Opzione B: Variabili di ambiente (chiave di accesso)**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**Opzione C: Variabili di ambiente (profilo SSO)**

Sostituire `your-profile-name` con il nome del profilo AWS prima di eseguire questi comandi.

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code richiede credenziali di ruolo dalla regione IAM Identity Center denominata da `sso_region` del profilo, che non deve corrispondere alla regione in cui si esegue Amazon Bedrock. Nella versione 2.1.207, la regione di Amazon Bedrock ha sovrascritto `sso_region`, quindi un profilo la cui istanza di IAM Identity Center si trova in una regione diversa non è riuscito ad autenticarsi con un errore `Session token not found or invalid`.

**Opzione D: Credenziali della console di gestione AWS**

```bash theme={null}
aws login
```

[Ulteriori informazioni](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) su `aws login`.

**Opzione E: Chiavi API di Amazon Bedrock**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Le chiavi API di Amazon Bedrock forniscono un metodo di autenticazione più semplice senza la necessità di credenziali AWS complete. [Ulteriori informazioni sulle chiavi API di Amazon Bedrock](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/).

<h4 id="credential-caching-and-resolution-timeout">
  Caching delle credenziali e timeout di risoluzione
</h4>

Claude Code risolve la catena del provider di credenziali predefinito AWS una volta e mantiene le credenziali risolte in memoria. Le riutilizza fino a cinque minuti prima della scadenza, o per un'ora quando non hanno scadenza, quindi un profilo supportato da SSO richiede credenziali da IAM Identity Center circa una volta per durata della credenziale. Un errore di credenziale dall'API cancella la cache e il nuovo tentativo risolve credenziali aggiornate. Richiede Claude Code v2.1.207 o successiva.

La cache copre tutte le opzioni di credenziale sopra elencate tranne una chiave API di Amazon Bedrock, che non utilizza la catena del provider. Per risolvere la catena su ogni richiesta invece, impostare [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/it/env-vars).

Ogni risoluzione della catena scade dopo 60 secondi. Se un passaggio della catena si blocca, ad esempio un helper `credential_process` che attende un input che non può ricevere, la richiesta non riesce con [`AWS default-chain credential resolve timed out`](/docs/it/errors#aws-default-chain-credential-resolve-timed-out). Se la catena esegue un accesso interattivo che legittimamente ha bisogno di più tempo, come SSO basato su browser con MFA tramite un wrapper come `aws-vault`, aumentare il limite in millisecondi con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/it/env-vars). Prima della versione 2.1.207, una risoluzione di credenziale bloccata lasciava la richiesta in attesa indefinitamente.

Tranne quando si esegue l'autenticazione con una chiave API di Amazon Bedrock, la [procedura guidata di configurazione](#sign-in-with-bedrock) applica lo stesso limite a ogni chiamata AWS che effettua durante la verifica delle credenziali, e alla ricerca delle credenziali prima di ogni controllo del modello. Durante la verifica delle credenziali, un controllo che lo supera non riesce con [`Timed out after 60s waiting for AWS`](/docs/it/errors#bedrock-setup-verification-timed-out-waiting-for-aws).

<h4 id="advanced-credential-configuration">
  Configurazione avanzata delle credenziali
</h4>

Claude Code supporta l'aggiornamento automatico delle credenziali per AWS SSO e provider di identità aziendali. Aggiungere queste impostazioni al file di impostazioni di Claude Code (vedere [Settings](/docs/it/settings) per i percorsi dei file).

Queste due impostazioni hanno diverse condizioni di attivazione:

* **`awsAuthRefresh`**: viene eseguito solo quando Claude Code rileva che le credenziali AWS sono scadute, localmente in base al loro timestamp o quando l'API restituisce un errore di credenziale, quindi ritenta la richiesta con credenziali aggiornate.
* **`awsCredentialExport`**: viene eseguito all'avvio della sessione e su ogni ricaricamento delle credenziali, anche quando le credenziali nella catena del provider di credenziali predefinito AWS sono ancora valide. Utilizzare questa opzione quando l'account Amazon Bedrock richiede credenziali tra account che differiscono da quelle che la catena del provider predefinito risolverebbe.

Prima di eseguire il comando `awsAuthRefresh`, Claude Code effettua una chiamata STS `GetCallerIdentity` per confermare che le credenziali sono effettivamente scadute e ignora il comando quando funzionano ancora. Claude Code invia questo controllo attraverso la [configurazione del proxy](/docs/it/network-config#proxy-configuration), rispettando `HTTPS_PROXY` e `NO_PROXY`. Prima della versione 2.1.239, Claude Code inviava questo controllo direttamente e si bloccava all'avvio su reti che consentono solo l'uscita attraverso un proxy.

<h5 id="example-configuration">
  Configurazione di esempio
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  Impostazioni di configurazione spiegate
</h5>

**`awsAuthRefresh`**: Utilizzare questa opzione per i comandi che modificano la directory `.aws`, come l'aggiornamento delle credenziali, della cache SSO o dei file di configurazione. L'output del comando viene visualizzato all'utente, ma l'input interattivo non è supportato. Funziona bene per i flussi SSO basati su browser in cui la CLI visualizza un URL o un codice e si completa l'autenticazione nel browser.

**`awsCredentialExport`**: Utilizzare solo se non è possibile modificare `.aws` e si devono restituire direttamente le credenziali. L'output viene acquisito silenziosamente e non mostrato all'utente. Il comando deve restituire JSON in questo formato:

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

L'output flat da `aws configure export-credentials --format process` è accettato anche, con le stesse chiavi al livello superiore invece di annidate sotto `Credentials`.

`Expiration` è facoltativo. Quando il comando restituisce un `Expiration` ISO 8601 valido, Claude Code memorizza nella cache le credenziali fino a cinque minuti prima di quel momento. Senza di esso, le credenziali vengono memorizzate nella cache per un'ora.

Quando si configura `awsCredentialExport` senza `awsAuthRefresh`, Claude Code utilizza le credenziali esportate direttamente e non ri-risolve la catena del provider di credenziali predefinito AWS all'avvio. Richiede Claude Code v2.1.206 o successiva.

<h3 id="3-configure-claude-code">
  3. Configurare Claude Code
</h3>

Impostare le seguenti variabili di ambiente per abilitare Amazon Bedrock:

```bash theme={null}
# Enable Bedrock integration
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # optional if your AWS profile already sets a region

# Optional: Override the AWS region for the small/fast model (Bedrock and Mantle).
# On Bedrock, has no effect without ANTHROPIC_DEFAULT_HAIKU_MODEL
# or the deprecated ANTHROPIC_SMALL_FAST_MODEL set.
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# Optional: Override the Bedrock endpoint URL for custom endpoints or gateways
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

Quando si abilita Amazon Bedrock per Claude Code, tenere presente quanto segue:

* È necessario impostare `AWS_REGION` solo per sovrascrivere la regione del profilo AWS o quando il profilo non ha una regione. Claude Code risolve la regione in questo ordine:

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * la `region` impostata sul profilo AWS attivo, letta dal file delle credenziali condivise AWS per primo e poi dal file di configurazione condiviso, corrispondendo alla precedenza dell'SDK AWS
  * `us-east-1`

  Se un valore da una qualsiasi di queste fonti non ha la forma di un nome di regione, Claude Code lo tratta come non impostato e continua verso il basso nell'ordine. Ad esempio, Claude Code tratta un valore contenente una barra, un punto o uno spazio come non impostato.

  Il profilo attivo è `AWS_PROFILE` se impostato, altrimenti `default`. Impostare `AWS_SHARED_CREDENTIALS_FILE` o `AWS_CONFIG_FILE` per puntare a percorsi di file non predefiniti.

  Eseguire `/status` per visualizzare la regione risolta. Quando la regione proviene dai file di configurazione AWS o dal fallback predefinito, Claude Code nota anche la fonte nell'output `/status`.
* Quando si utilizza Amazon Bedrock, il comando `/logout` non è disponibile poiché l'autenticazione viene gestita tramite credenziali AWS.
* Lo strumento WebSearch non è disponibile su Amazon Bedrock. Vedere [Comportamento dello strumento WebSearch](/docs/it/tools-reference#websearch-tool-behavior).
* È possibile utilizzare file di impostazioni per variabili di ambiente come `AWS_PROFILE` che non si desidera perdere in altri processi. Vedere [Settings](/docs/it/settings) per ulteriori informazioni.

<h3 id="4-pin-model-versions">
  4. Fissare le versioni del modello
</h3>

<Warning>
  Fissare versioni specifiche del modello quando si distribuisce a più utenti. Senza fissaggio, gli alias del modello come `sonnet` e `opus` si risolvono nel valore predefinito integrato di Claude Code per Amazon Bedrock, che può rimanere indietro rispetto alla versione più recente e potrebbe non essere ancora disponibile nel proprio account. Claude Code [ritorna](#startup-model-checks) a un modello precedente o di livello inferiore all'avvio quando il valore predefinito non è disponibile, ma il fissaggio consente di controllare quando gli utenti passano a un nuovo modello.
</Warning>

Impostare queste variabili di ambiente su ID modello Amazon Bedrock specifici.

Senza `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` su Amazon Bedrock si risolve in Opus 5.5, e senza `ANTHROPIC_DEFAULT_SONNET_MODEL`, l'alias `sonnet` si risolve in Sonnet 4.5. Questo esempio fissa ogni alias a una versione specifica:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

Questi ID utilizzano il prefisso del profilo di inferenza tra regioni `us.`. Se si utilizza un prefisso di regione diverso o profili di inferenza dell'applicazione, regolare di conseguenza. Nelle regioni AWS GovCloud, utilizzare il prefisso `us-gov.`.

Per mantenere i modelli predefiniti integrati e modificare solo il loro prefisso preferito, impostare [`ANTHROPIC_BEDROCK_REGION_PREFIX`](#cross-region-inference-profile-prefixes) invece di fissare. La differenza si mostra in ciò a cui l'alias `opus` si risolve:

| Si imposta                                                    | L'alias `opus` si risolve in                                                              |
| :------------------------------------------------------------ | :---------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` | `us.anthropic.claude-opus-4-8`, l'ID esatto che hai fissato                               |
| `ANTHROPIC_BEDROCK_REGION_PREFIX=eu`                          | `eu.anthropic.claude-opus-5-5`, il valore predefinito integrato con il prefisso preferito |

Per gli ID modello attuali e legacy, vedere [Panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview). Per l'elenco completo delle variabili di ambiente di fissaggio, vedere [Configurazione del modello](/docs/it/model-config#pin-models-for-third-party-deployments).

Claude Code utilizza questi modelli predefiniti quando non sono impostate variabili di fissaggio:

| Tipo di modello        | Modello predefinito                                                                         |
| :--------------------- | :------------------------------------------------------------------------------------------ |
| Modello primario       | Opus 5.5, ad esempio `us.anthropic.claude-opus-5-5` in una regione `us-*`                   |
| Modello piccolo/veloce | Sonnet 4.5, ad esempio `us.anthropic.claude-sonnet-4-5-20250929-v1:0` in una regione `us-*` |

Le attività in background come la generazione del titolo della sessione utilizzano il modello piccolo/veloce, normalmente un modello di classe Haiku. Su Amazon Bedrock, Claude Code utilizza il modello Sonnet predefinito per le attività in background perché Haiku potrebbe non essere abilitato in ogni account o regione. Due selezioni cambiano quale modello li trasporta:

* Quando si seleziona un modello primario con `--model`, `ANTHROPIC_MODEL` o l'impostazione `model`, le attività in background utilizzano quel modello. Quando Claude Code avvia la sessione sul modello impostato con [`ANTHROPIC_DEFAULT_MODEL`](/docs/it/model-config#set-a-default-model-for-new-sessions), le attività in background utilizzano anche quel modello. L'impostazione di `ANTHROPIC_DEFAULT_OPUS_MODEL` senza `ANTHROPIC_DEFAULT_SONNET_MODEL` conta anche come una selezione, perché il modello Sonnet integrato potrebbe non essere abilitato in un account che indirizza il proprio Opus.
* Per utilizzare Haiku per le attività in background, impostare `ANTHROPIC_DEFAULT_HAIKU_MODEL` su un ID modello disponibile nel proprio account.

<Warning>
  I modelli Opus hanno un prezzo per token più elevato rispetto ai modelli Sonnet, quindi una distribuzione che non fissa un modello primario viene fatturata alla tariffa Opus una volta che si aggiorna alla versione 2.1.207 o successiva. Per mantenere Sonnet 4.5 come modello primario, impostare `ANTHROPIC_MODEL` al suo ID modello completo. Una distribuzione che indirizza il valore predefinito con `ANTHROPIC_DEFAULT_SONNET_MODEL` e non imposta `ANTHROPIC_DEFAULT_OPUS_MODEL` mantiene il modello Sonnet indirizzato come valore predefinito.
</Warning>

Prima della versione 2.1.280, il modello primario su Amazon Bedrock era predefinito su Opus 5 e l'alias `opus` si risolveva in Opus 5 dalla versione 2.1.219. Nella versione 2.1.207 fino a 2.1.218, il modello primario su Amazon Bedrock era predefinito su Opus 4.8 e l'alias `opus` si risolveva in Opus 4.8. Prima della versione 2.1.207, il modello primario era predefinito su Sonnet 4.5, l'alias `opus` si risolveva in Opus 4.6 e le attività in background utilizzavano sempre il modello primario.

Per personalizzare ulteriormente i modelli, utilizzare uno di questi metodi:

```bash theme={null}
# Using inference profile ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# Using application inference profile ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# Optional: Disable prompt caching if needed
# export DISABLE_PROMPT_CACHING=1

# Optional: Request 1-hour prompt cache TTL instead of the 5-minute default
# export ENABLE_PROMPT_CACHING_1H=1
```

Il TTL della cache di 1 ora viene fatturato a una tariffa più elevata rispetto al valore predefinito di 5 minuti. Vedere [durata della cache](/docs/it/prompt-caching#cache-lifetime). Per impostare TTL diversi per la conversazione principale e per le richieste che Claude Code effettua al di fuori di essa, [scegliere il TTL da soli](/docs/it/prompt-caching#choose-the-ttl-yourself).

<Note>La memorizzazione nella cache del prompt potrebbe non essere disponibile in tutte le regioni di Amazon Bedrock. Se i conteggi dei token della cache rimangono a zero, controllare [modelli supportati, regioni e limiti](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) nella documentazione di Amazon Bedrock.</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  Mappare ogni versione del modello a un profilo di inferenza
</h4>

Le variabili di ambiente `ANTHROPIC_DEFAULT_*_MODEL` configurano un profilo di inferenza per famiglia di modelli. Se l'organizzazione ha bisogno di esporre diverse versioni della stessa famiglia nel selettore `/model`, ciascuna indirizzata al proprio ARN del profilo di inferenza dell'applicazione, utilizzare l'impostazione `modelOverrides` nel [file di impostazioni](/docs/it/settings#where-settings-live).

Questo esempio mappa quattro versioni di Opus a ARN distinti in modo che gli utenti possano passare tra loro senza aggirare i profili di inferenza dell'organizzazione:

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

Quando un utente seleziona una di queste versioni in `/model`, Claude Code chiama Amazon Bedrock con l'ARN mappato. La stessa mappatura si applica quando si passa l'ID modello Anthropic direttamente tramite `--model` o `ANTHROPIC_MODEL`. Le versioni senza un override ritornano all'ID modello Amazon Bedrock integrato o a qualsiasi profilo di inferenza corrispondente scoperto all'avvio. Prima della versione 2.1.200, i valori `--model` e `ANTHROPIC_MODEL` raggiungevano Amazon Bedrock così come erano senza passare attraverso la mappa di override. Vedere [Override degli ID modello per versione](/docs/it/model-config#override-model-ids-per-version) per i dettagli su come gli override interagiscono con `availableModels` e altre impostazioni del modello.

<h2 id="startup-model-checks">
  Controlli del modello all'avvio
</h2>

Quando Claude Code si avvia con Amazon Bedrock configurato, verifica che i modelli che intende utilizzare siano accessibili nel vostro account.

Se avete fissato una versione del modello più vecchia rispetto al valore predefinito corrente di Claude Code, e il vostro account può richiamare la versione più recente, Claude Code vi chiede di aggiornare il pin. Accettando si scrive il nuovo ID del modello nel vostro [file di impostazioni utente](/docs/it/settings) e si riavvia Claude Code. Rifiutando viene ricordato fino al prossimo cambio della versione predefinita. I pin che puntano a un [ARN del profilo di inferenza dell'applicazione](#map-each-model-version-to-an-inference-profile) vengono saltati, poiché sono gestiti dall'amministratore.

Se non avete fissato un modello e il valore predefinito corrente non è disponibile nel vostro account, Claude Code esegue il fallback per la sessione corrente e mostra un avviso. Prova prima le versioni precedenti del modello predefinito e, quando il valore predefinito è un modello Opus e nessuna versione Opus è disponibile, esegue il fallback al modello Sonnet predefinito. Il fallback non viene mantenuto. Abilitate il modello più recente nel vostro account Amazon Bedrock o [fissate una versione](#4-pin-model-versions) per rendere permanente la scelta.

Quando avviate la sessione su una versione specifica di Sonnet oppure Opus, ad esempio con `--model`, `ANTHROPIC_MODEL`, o l'[impostazione `model`](/docs/it/settings-reference#model), quella versione agisce come valore predefinito fissato della sessione per l'alias `sonnet` oppure `opus` corrispondente. Claude Code salta il controllo di disponibilità per il valore predefinito integrato che il vostro modello sostituisce e si avvia sul modello che avete configurato, senza alcun avviso di fallback.

Gli alias dei modelli come `opus` non agiscono come pin, e nemmeno un ID del modello che Claude Code non riconosce, come un ARN del profilo di inferenza dell'applicazione.

<h2 id="cross-region-inference-profile-prefixes">
  Prefissi del profilo di inferenza tra regioni
</h2>

Sull'API Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html), Claude Code risolve i suoi modelli predefiniti incorporati agli ID del [profilo di inferenza tra regioni](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html); per instradare le versioni dei modelli attraverso i vostri profili di inferenza, consultate [Mappare ogni versione del modello a un profilo di inferenza](#map-each-model-version-to-an-inference-profile). Questa tabella mostra il prefisso che Claude Code preferisce per ogni regione AWS risolta:

| Regione AWS               | Prefisso  |
| :------------------------ | :-------- |
| `us-gov-*` (AWS GovCloud) | `us-gov.` |
| `us-*`                    | `us.`     |
| `eu-*`                    | `eu.`     |
| `ap-*`                    | `apac.`   |
| Tutte le altre regioni    | `global.` |

Impostare `ANTHROPIC_BEDROCK_REGION_PREFIX` per scegliere il prefisso che Claude Code prova per primo; quando Claude Code può verificare la disponibilità del profilo e non trova alcun profilo corrispondente per un modello, esegue il fallback come descritto nell'ordine di risoluzione di seguito. I valori validi sono `us`, `eu`, `apac`, `jp`, `au` e `global`. Ad esempio, impostatelo su `global` quando il vostro account ha profili `global.` abilitati ma Claude Code ne deriverebbe uno specifico della geografia dalla vostra regione AWS. Richiede Claude Code v2.1.224 o successivo.

Questo esempio instrada i modelli predefiniti attraverso profili `global.`:

```bash theme={null}
export ANTHROPIC_BEDROCK_REGION_PREFIX=global
# In a us-* region, the primary model now resolves to
# global.anthropic.claude-opus-5-5 instead of us.anthropic.claude-opus-5-5
```

Il prefisso preferito è una preferenza, non una garanzia, indipendentemente dal fatto che provenga dalla vostra regione o dalla variabile. Il modo in cui Claude Code lo applica dipende dal fatto che possa verificare la disponibilità del profilo nel vostro account:

* Quando Claude Code può [elencare i profili di inferenza](#iam-configuration) nel vostro account, risolve ogni modello in questo ordine:
  1. Il profilo con il vostro prefisso preferito.
  2. Qualsiasi profilo corrispondente, per un modello che non ha un profilo con quel prefisso.
  3. L'ID del modello incorporato con il vostro prefisso preferito, per un modello che non ha alcun profilo corrispondente. Claude Code applica questo ID senza verificare la disponibilità in questo passaggio; i [controlli del modello di avvio](#startup-model-checks) coprono comunque i modelli predefiniti della sessione.
* Quando l'individuazione del profilo non è disponibile, Claude Code applica il prefisso senza verificare la disponibilità. Se il vostro account non ha profili di inferenza con quel prefisso abilitati, le richieste non riescono con un errore 400.

Claude Code non riscrive gli ID dei profili di inferenza Amazon Bedrock o gli ARN che configurate voi stessi, o i valori [`modelOverrides`](#map-each-model-version-to-an-inference-profile); gli ID dei modelli in formato Anthropic si risolvono attraverso [la stessa mappatura del selettore `/model`](#map-each-model-version-to-an-inference-profile). Claude Code ignora inoltre la variabile in due casi:

* Nelle regioni AWS GovCloud, Claude Code utilizza sempre `us-gov.`, l'unico prefisso che instrada all'interno della partizione GovCloud.
* Quando impostate un valore che non è uno dei valori validi, Claude Code esegue il fallback al prefisso preferito derivato dalla regione.

<h2 id="iam-configuration">
  Configurazione IAM
</h2>

Crea una policy IAM con le autorizzazioni richieste per Claude Code:

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

Per autorizzazioni più restrittive, puoi limitare la Resource a ARN di profili di inferenza specifici.

`bedrock:GetInferenceProfile` consente a Claude Code di risolvere un [ARN del profilo di inferenza dell'applicazione](#map-each-model-version-to-an-inference-profile) al suo modello di fondazione di supporto, che viene utilizzato per selezionare la forma di richiesta corretta per quel modello.

Se il token non dispone di questa autorizzazione, Claude Code si recupera automaticamente ritentando una volta con la forma alternativa, quindi le richieste hanno comunque successo ma ogni nuovo modello aggiunge un round-trip aggiuntivo. Concedere l'autorizzazione evita il retry. Questo si applica più spesso alle distribuzioni `AWS_BEARER_TOKEN_BEDROCK`, dove la policy del token è tipicamente più ristretta di un ruolo IAM completo.

Per i dettagli, vedi [Documentazione IAM di Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html).

<Note>
  Crea un account AWS dedicato per Claude Code per semplificare il tracciamento dei costi e il controllo degli accessi.
</Note>

<h2 id="1m-token-context-window">
  Finestra di contesto da 1M token
</h2>

Claude Sonnet 5, Opus 4.6 e versioni successive, e Sonnet 4.6 supportano la [finestra di contesto da 1M token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) su Amazon Bedrock. Sonnet 5 funziona sempre con la finestra da 1M sia sull'API Invoke che sull'[endpoint Mantle](#use-the-mantle-endpoint), senza alcuna variante `[1m]` da selezionare. Per gli altri modelli sull'API Invoke, Claude Code abilita automaticamente la finestra di contesto estesa quando selezioni una variante di modello da 1M.

La [procedura guidata di configurazione](#sign-in-with-bedrock) offre un'opzione di contesto da 1M quando fissa i modelli. Per abilitarla per un modello fissato manualmente, aggiungi `[1m]` all'ID del modello. Vedi [Fissa i modelli per distribuzioni di terze parti](/docs/it/model-config#pin-models-for-third-party-deployments) per i dettagli.

<h2 id="service-tiers">
  Livelli di servizio
</h2>

[I livelli di servizio di Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) ti consentono di scambiare il costo rispetto alla latenza. Imposta `ANTHROPIC_BEDROCK_SERVICE_TIER` su `default`, `flex` o `priority`:

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code invia questo come intestazione `X-Amzn-Bedrock-Service-Tier` su ogni richiesta. La disponibilità del livello varia in base al modello e alla regione. La capacità riservata utilizza un [ARN di throughput fornito](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) come ID del modello invece di questa impostazione.

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) ti consente di implementare il filtro dei contenuti per Claude Code. Crea un Guardrail nella [console di Amazon Bedrock](https://console.aws.amazon.com/bedrock/), pubblica una versione, quindi aggiungi le intestazioni Guardrail al tuo [file di impostazioni](/docs/it/settings). Abilita l'inferenza tra regioni sul tuo Guardrail se stai utilizzando profili di inferenza tra regioni.

Configurazione di esempio:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

Se la tua organizzazione fornisce le intestazioni del guardrail attraverso una politica di [gateway delle app Claude](/docs/it/claude-apps-gateway), contano come [impostazioni che richiedono approvazione](/docs/it/server-managed-settings#environment-variables-and-the-approval-dialog).

<h2 id="use-the-mantle-endpoint">
  Utilizza l'endpoint Mantle
</h2>

Mantle è un endpoint di Amazon Bedrock che serve i modelli Claude attraverso la forma API nativa di Anthropic piuttosto che l'API Invoke di Amazon Bedrock. Utilizza le stesse [credenziali AWS](#2-configure-aws-credentials) e configurazione [`awsAuthRefresh`](#advanced-credential-configuration).

Mantle ha le sue proprie azioni IAM con il prefisso `bedrock-mantle:`, quindi le azioni `bedrock:` nella [configurazione IAM](#iam-configuration) non la coprono. Concedi alla tua identità IAM `bedrock-mantle:CreateInference` per l'inferenza e `bedrock-mantle:CountTokens` per il conteggio dei token. Vedi [Effettuare richieste di inferenza](https://docs.aws.amazon.com/bedrock/latest/userguide/inference.html) e [Conteggio dei token](https://docs.aws.amazon.com/bedrock/latest/userguide/count-tokens.html) nella documentazione AWS, e il [riferimento di autorizzazione del servizio](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrockpoweredbyawsmantle.html) per ogni azione Mantle.

<h3 id="enable-mantle">
  Abilita Mantle
</h3>

Con le credenziali AWS già configurate, imposta `CLAUDE_CODE_USE_MANTLE` per instradare le richieste all'endpoint Mantle:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code costruisce l'URL dell'endpoint dalla regione AWS, risolta con la stessa precedenza di [Amazon Bedrock sopra](#3-configure-claude-code). Per sovrascrivere l'URL per un endpoint personalizzato o gateway, imposta `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`.

Esegui `/status` all'interno di Claude Code per confermare. La riga del provider mostra `Amazon Bedrock (Mantle)` quando Mantle è attivo.

<h3 id="select-a-mantle-model">
  Seleziona un modello Mantle
</h3>

Mantle utilizza ID di modello con prefisso `anthropic.` e senza suffisso di versione, ad esempio `anthropic.claude-sonnet-5` o `anthropic.claude-haiku-4-5`. I modelli disponibili per il tuo account dipendono da ciò che la tua organizzazione ha ricevuto; gli ID di modello aggiuntivi sono elencati nei tuoi materiali di onboarding da AWS. Contatta il tuo team di account AWS per richiedere l'accesso ai modelli consentiti.

Imposta il modello con il flag `--model` o con `/model` all'interno di Claude Code:

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Esegui Mantle insieme all'API Invoke
</h3>

I modelli disponibili per te su Mantle potrebbero non includere ogni modello che utilizzi oggi. Impostare sia `CLAUDE_CODE_USE_BEDROCK` che `CLAUDE_CODE_USE_MANTLE` consente a Claude Code di chiamare entrambi gli endpoint dalla stessa sessione. Gli ID di modello che corrispondono al formato Mantle vengono instradati a Mantle, e tutti gli altri ID di modello vanno all'API Invoke di Amazon Bedrock.

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Per visualizzare un modello Mantle nel selettore `/model`, elenca il suo ID in `availableModels` nel tuo [file di impostazioni](/docs/it/settings). Questa impostazione limita anche il selettore alle voci elencate. L'elenco di `anthropic.claude-haiku-4-5` rimuove l'alias bare `haiku` dal selettore, quindi elenca anche i prefissi di versione o gli ID completi per le versioni che desideri mantenere selezionabili. L'ID Mantle e l'alias `haiku` si risolvono nella stessa famiglia di modelli, quindi l'unione mantiene solo la voce più specifica. Vedi [Comportamento di unione](/docs/it/model-config#merge-behavior):

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

Le voci con il prefisso `anthropic.` vengono aggiunte come opzioni del selettore personalizzato e instradate a Mantle. Sostituisci `anthropic.claude-haiku-4-5` con l'ID del modello che il tuo account ha ricevuto. Vedi [Limita la selezione del modello](/docs/it/model-config#restrict-model-selection) per come `availableModels` interagisce con altre impostazioni del modello.

Quando entrambi i provider sono attivi, `/status` mostra `Amazon Bedrock + Amazon Bedrock (Mantle)`.

<h3 id="route-mantle-through-a-gateway">
  Instrada Mantle attraverso un gateway
</h3>

Se la tua organizzazione instrada il traffico del modello attraverso un [gateway LLM](/docs/it/llm-gateway) centralizzato che inietta le credenziali AWS lato server, disabilita l'autenticazione lato client in modo che Claude Code invii richieste senza firme SigV4 o intestazioni `x-api-key`:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Variabili di ambiente Mantle
</h3>

Queste variabili sono specifiche dell'endpoint Mantle. Vedi [Variabili di ambiente](/docs/it/env-vars) per l'elenco completo.

| Variabile                               | Scopo                                                                                       |
| :-------------------------------------- | :------------------------------------------------------------------------------------------ |
| `CLAUDE_CODE_USE_MANTLE`                | Abilita l'endpoint Mantle. Imposta su `1` o `true`.                                         |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | Sovrascrivi l'URL dell'endpoint Mantle predefinito                                          |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | Salta l'autenticazione lato client per configurazioni proxy                                 |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Sovrascrivi la regione AWS per il modello della classe Haiku (condiviso con Amazon Bedrock) |

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  Loop di autenticazione con SSO e proxy aziendali
</h3>

Se le schede del browser si aprono ripetutamente quando si utilizza AWS SSO, rimuovi l'impostazione `awsAuthRefresh` dal tuo [file di impostazioni](/docs/it/settings). Questo può accadere quando le VPN aziendali o i proxy di ispezione TLS interrompono il flusso del browser SSO. Claude Code tratta la connessione interrotta come un errore di autenticazione, riesegue `awsAuthRefresh` e si ripete indefinitamente.

Se il tuo ambiente di rete interferisce con i flussi SSO automatici basati su browser, utilizza `aws sso login` manualmente prima di avviare Claude Code invece di affidarti a `awsAuthRefresh`.

<h3 id="certificate-errors-behind-a-tls-inspecting-proxy">
  Errori di certificato dietro un proxy che ispeziona TLS
</h3>

Claude Code applica la configurazione del tuo [archivio di certificati CA](/docs/it/network-config#ca-certificate-store) alle sue richieste ad AWS, incluse:

* Scoperta del modello
* Conteggio dei token
* Le chiamate del ruolo di credenziale STS e SSO che risolvono le tue credenziali AWS
* La verifica delle credenziali e i controlli del modello della [procedura guidata di configurazione](#sign-in-with-bedrock)

Per queste richieste, un certificato radice aziendale nel tuo archivio di attendibilità del sistema operativo o nel bundle `NODE_EXTRA_CA_CERTS` non necessita di alcuna configurazione specifica di Amazon Bedrock.

Prima della v2.1.260, Claude Code applicava la tua configurazione CA a queste richieste solo quando passavano attraverso un proxy configurato, e su una connessione diretta si fidavano solo dell'archivio di certificati predefinito del runtime.

Prima della v2.1.261, la ricerca delle credenziali dietro i controlli del modello della procedura guidata di configurazione con l'opzione **Usa credenziali già nel mio ambiente** si fidava ancora solo dell'archivio di certificati predefinito del runtime. Dietro un proxy che ispeziona TLS il cui certificato radice è solo nell'archivio del sistema operativo, le richieste interessate non riuscivano con `unable to get local issuer certificate`, oppure la procedura guidata mostrava i modelli come `unreachable`, mentre le richieste di inferenza avevano successo. Aggiorna a v2.1.261 o versione successiva.

<h3 id="region-issues">
  Problemi di regione
</h3>

Se riscontri problemi di regione:

* Controlla la disponibilità del modello: `aws bedrock list-inference-profiles --region your-region`
* Passa a una regione supportata: `export AWS_REGION=us-east-1`
* Considera l'utilizzo di profili di inferenza per l'accesso tra regioni

Se ricevi un errore "on-demand throughput isn't supported":

* Specifica il modello come ID di [profilo di inferenza](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)

Claude Code utilizza l'API Amazon Bedrock [Invoke](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) e non supporta l'API Converse.

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  Errori di streaming dietro un gateway o proxy
</h3>

Amazon Bedrock trasmette le risposte `InvokeModelWithResponseStream` in un formato binario event-stream con l'intestazione `Content-Type: application/vnd.amazon.eventstream`. Un gateway o proxy tra Claude Code e Amazon Bedrock deve inoltrare il corpo della risposta e le sue intestazioni, incluso `Content-Type`, così come Amazon Bedrock le ha inviate.

Se il gateway riscrive `Content-Type` in un altro valore, Claude Code rifiuta la risposta con un errore che inizia con `Bedrock streaming response has content-type`, indicando il valore che ha ricevuto. La riscrittura comune è `text/event-stream`, da un'integrazione che ri-emette il flusso come server-sent events.

Se il gateway elimina o cancella l'intestazione, Claude Code presume che il corpo sia il flusso di eventi di Amazon Bedrock e lo decodifica, quindi un corpo che il gateway ha fatto passare senza modifiche continua a trasmettere.

Se un gateway che elimina l'intestazione ri-emette anche il flusso come server-sent events, Claude Code non può decodificare il corpo e ricade su un percorso più lento senza streaming ad ogni turno: ogni risposta appare solo una volta completata invece di trasmettere. In quel caso, imposta [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_DEFAULT=1`](/docs/it/env-vars) in modo che Claude Code legga il corpo come server-sent events.

Per correggere l'errore o il fallback, configura il gateway per inoltrare il corpo della risposta `InvokeModelWithResponseStream` e la sua intestazione `Content-Type` senza modifiche.

Un gateway che converte il flusso in server-sent events non sta più servendo l'API Amazon Bedrock. Se accetta anche richieste dell'API Anthropic Messages, connettiti ad esso come [gateway LLM](/docs/it/llm-gateway-connect) con `ANTHROPIC_BASE_URL` invece di `CLAUDE_CODE_USE_BEDROCK`.

<h3 id="zero-token-counts-in-/context">
  Conteggi di token zero in /context
</h3>

Il comando `/context` conta i token per ogni gruppo di strumenti inviando gli schemi degli strumenti all'API count-tokens di Amazon Bedrock. Nelle versioni di Claude Code precedenti a v2.1.196, Amazon Bedrock ha rifiutato quella richiesta perché gli schemi contenevano campi che la sua API count-tokens non accetta, quindi ogni gruppo di strumenti mostrava 0 token. Altre righe nella suddivisione, come i messaggi e i file di memoria, non sono interessati.

Aggiorna a v2.1.196 o versione successiva.

<h3 id="mantle-endpoint-errors">
  Errori dell'endpoint Mantle
</h3>

Se `/status` non mostra `Amazon Bedrock (Mantle)` dopo aver impostato `CLAUDE_CODE_USE_MANTLE`, la variabile non sta raggiungendo il processo. Conferma che sia esportata nella shell in cui hai lanciato `claude`, o impostala nel blocco `env` del tuo [file di impostazioni](/docs/it/settings).

Cosa significa un `403` dall'endpoint Mantle dipende dal fatto che l'errore nomini un'azione IAM:

* Se l'errore nomina un'azione `bedrock-mantle:`, concedi alla tua identità IAM quell'azione.
* Se l'errore non nomina alcuna azione e le tue credenziali sono valide, il tuo account AWS non ha ricevuto l'accesso al modello che hai richiesto. Contatta il tuo team di account AWS per richiedere l'accesso.

Un `400` che nomina l'ID del modello significa che quel modello non è servito su Mantle. Mantle ha il suo proprio lineup di modelli separato dal catalogo Amazon Bedrock standard, quindi gli ID del profilo di inferenza come `us.anthropic.claude-sonnet-4-6` non funzioneranno. Utilizza un ID nel formato Mantle, o abilita [entrambi gli endpoint](#run-mantle-alongside-the-invoke-api) in modo che Claude Code instrada ogni richiesta all'endpoint in cui il modello è disponibile.

<h2 id="additional-resources">
  Risorse aggiuntive
</h2>

* [Documentazione di Amazon Bedrock](https://docs.aws.amazon.com/bedrock/)
* [Prezzi di Amazon Bedrock](https://aws.amazon.com/bedrock/pricing/)
* [Profili di inferenza di Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Burndown dei token di Amazon Bedrock e quote](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code su Amazon Bedrock: Guida di configurazione rapida](https://builder.aws.com/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Implementazione del monitoraggio di Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
