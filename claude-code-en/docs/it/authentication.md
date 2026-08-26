> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autenticazione

> Accedi a Claude Code e configura l'autenticazione per singoli utenti, team e organizzazioni.

Claude Code supporta molteplici metodi di autenticazione a seconda della Vostra configurazione. I singoli utenti possono accedere con un account Claude.ai, mentre i team possono utilizzare Claude for Teams o Enterprise, la Claude Console, o un provider cloud come Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Accedi a Claude Code
</h2>

Dopo aver [installato Claude Code](/docs/it/setup#install-claude-code), eseguite `claude` nel vostro terminale. Al primo avvio, Claude Code apre una finestra del browser per consentirvi di accedere.

Se il browser non si apre automaticamente, premete `c` per copiare l'URL di accesso negli appunti, quindi incollatelo nel vostro browser.

Se il vostro browser mostra un codice di accesso invece di reindirizzarvi dopo aver effettuato l'accesso, incollatelo nel terminale al prompt `Paste code here if prompted`. Questo accade quando il browser non riesce a raggiungere il server di callback locale di Claude Code, il che è comune in WSL2, sessioni SSH e container.

Quando l'accesso è completato, il terminale mostra `Login successful` e vi chiede di premere `Enter` per continuare.

Potete autenticarvi con uno di questi tipi di account:

* **Sottoscrizione Claude Pro o Max**: accedete con il vostro account Claude.ai. Sottoscrivete su [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams o Enterprise**: accedete con l'account Claude.ai che l'amministratore del vostro team vi ha invitato a utilizzare.
* **Claude Console**: accedete con le vostre credenziali Console. L'amministratore deve avervi [invitato](#claude-console-authentication) prima.
* **Provider cloud**: se la vostra organizzazione utilizza [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry), impostate le variabili di ambiente richieste prima di eseguire `claude`, oppure selezionate **3rd-party platform** al prompt di accesso, che avvia una procedura guidata di configurazione interattiva per Bedrock e Vertex AI. Non è necessario alcun accesso tramite browser.
* **Cloud gateway**: se la vostra organizzazione esegue un [gateway di app Claude](/docs/it/claude-apps-gateway) auto-ospitato, accedete con SSO aziendale tramite `/login`. Il token emesso dal gateway è l'unica credenziale della sessione.

Gli amministratori possono limitare l'accesso interattivo con le impostazioni gestite [`forceLoginMethod` e `forceLoginOrgUUID`](/docs/it/settings#available-settings). Quando una di queste è impostata, le sessioni autenticate da `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper` vengono bloccate all'avvio; le sessioni dei provider cloud non sono interessate.

Per disconnettervi e autenticarvi di nuovo, digitate `/logout` al prompt di Claude Code. La disconnessione ripristina anche lo stato di configurazione al primo avvio, quindi la prossima volta che eseguite `claude` vi guida attraverso l'accesso e la configurazione di nuovo.

Se avete difficoltà ad accedere, consultate la sezione [risoluzione dei problemi di autenticazione](/docs/it/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Configurare l'autenticazione del team
</h2>

Per team e organizzazioni, potete configurare l'accesso a Claude Code in uno di questi modi:

* [Claude for Teams o Enterprise](#claude-for-teams-or-enterprise), consigliato per la maggior parte dei team
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/it/claude-apps-gateway), un gateway auto-ospitato che consente ai sviluppatori di accedere con il vostro IdP e instrada l'inferenza al provider cloud che configurate
* [Amazon Bedrock](/docs/it/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/it/google-vertex-ai)
* [Microsoft Foundry](/docs/it/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams o Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) e [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) offrono la migliore esperienza per le organizzazioni che utilizzano Claude Code. I membri del team ottengono accesso sia a Claude Code che a Claude sul web con fatturazione centralizzata e gestione del team.

* **Claude for Teams**: piano self-service con funzionalità di collaborazione, strumenti di amministrazione e gestione della fatturazione. Ideale per team più piccoli.
* **Claude for Enterprise**: aggiunge SSO, domain capture, autorizzazioni basate su ruoli, API di conformità e impostazioni di policy gestite per configurazioni Claude Code a livello organizzativo. Ideale per organizzazioni più grandi con requisiti di sicurezza e conformità.

<Steps>
  <Step title="Sottoscrivete">
    Sottoscrivete a [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) o contattate il team di vendita per [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Invitate i membri del team">
    Invitate i membri del team dalla dashboard di amministrazione.
  </Step>

  <Step title="Installate e accedete">
    I membri del team installano Claude Code e accedono con i loro account Claude.ai.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Autenticazione Claude Console
</h3>

Per le organizzazioni che preferiscono la fatturazione basata su API, potete configurare l'accesso tramite la Claude Console.

<Steps>
  <Step title="Creare o utilizzare un account Console">
    Utilizzate il Vostro account Claude Console esistente o createne uno nuovo.
  </Step>

  <Step title="Aggiungere utenti">
    Potete aggiungere utenti tramite uno dei due metodi:

    * Invitate utenti in massa dalla Console: Settings -> Members -> Invite
    * [Configurate SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="Assegnare ruoli">
    Quando invitate utenti, assegnate uno dei seguenti:

    * **Ruolo Claude Code**: gli utenti possono solo creare chiavi API Claude Code
    * **Ruolo Developer**: gli utenti possono creare qualsiasi tipo di chiave API
  </Step>

  <Step title="Gli utenti completano la configurazione">
    Ogni utente invitato deve:

    * Accettare l'invito Console
    * [Controllare i requisiti di sistema](/docs/it/setup#system-requirements)
    * [Installare Claude Code](/docs/it/setup#install-claude-code)
    * Accedere con le credenziali dell'account Console
  </Step>
</Steps>

<h3 id="cloud-provider-authentication">
  Autenticazione del provider cloud
</h3>

Per i team che utilizzano Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry:

<Steps>
  <Step title="Seguire la configurazione del provider">
    Seguite la [documentazione Amazon Bedrock](/docs/it/amazon-bedrock), la [documentazione Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o la [documentazione Microsoft Foundry](/docs/it/microsoft-foundry).
  </Step>

  <Step title="Distribuire la configurazione">
    Distribuite le variabili di ambiente e le istruzioni per generare credenziali cloud ai vostri utenti. Leggete di più su come [gestire la configurazione qui](/docs/it/settings).
  </Step>

  <Step title="Installare Claude Code">
    Gli utenti possono [installare Claude Code](/docs/it/setup#install-claude-code).
  </Step>
</Steps>

<h2 id="credential-management">
  Gestione delle credenziali
</h2>

Claude Code gestisce in modo sicuro le Vostre credenziali di autenticazione:

* **Posizione di archiviazione**:
  * Su macOS, le credenziali sono archiviate nel Keychain macOS crittografato.
  * Su Linux, le credenziali sono archiviate in `~/.claude/.credentials.json` con modalità file `0600`.
  * Su Windows, le credenziali sono archiviate in `%USERPROFILE%\.claude\.credentials.json` e ereditano i controlli di accesso della directory del profilo utente, che limita il file al Vostro account utente per impostazione predefinita.
  * Se avete impostato la variabile di ambiente `CLAUDE_CONFIG_DIR` su Linux o Windows, il file `.credentials.json` si trova in quella directory.
  * Claude Code gestisce `.credentials.json` attraverso `/login` e `/logout`. Per instradare le richieste attraverso un endpoint API personalizzato, impostate invece la variabile di ambiente [`ANTHROPIC_BASE_URL`](/docs/it/env-vars).
* **Tipi di autenticazione supportati**: credenziali Claude.ai, credenziali API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, e token di sessione del [gateway delle app Claude](/docs/it/claude-apps-gateway).
* **Script di credenziali personalizzati**: l'impostazione [`apiKeyHelper`](/docs/it/settings#available-settings) può essere configurata per eseguire uno script shell che restituisce una chiave API.
* **Intervalli di aggiornamento**: per impostazione predefinita, `apiKeyHelper` viene chiamato dopo 5 minuti o in risposta a HTTP 401. Impostate la variabile di ambiente `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` per intervalli di aggiornamento personalizzati.
* **Avviso di helper lento**: se `apiKeyHelper` impiega più di 10 secondi per restituire una chiave, Claude Code visualizza un avviso nella barra del prompt mostrando il tempo trascorso. Se vedete questo avviso regolarmente, verificate se lo script di credenziali può essere ottimizzato.
* **Errori dell'helper**: quando lo script esce con un errore, scade il timeout, o non stampa nulla, le richieste falliscono con [`Your apiKeyHelper script is failing`](/docs/it/errors#your-apikeyhelper-script-is-failing) entro tre tentativi. Prima della v2.1.208, gli errori dell'helper emergevano come un generico 401 dopo circa dieci tentativi silenziosi.

`apiKeyHelper`, `ANTHROPIC_API_KEY`, e `ANTHROPIC_AUTH_TOKEN` si applicano alla CLI e alle superfici che la avvolgono, inclusa l'estensione VS Code, l'Agent SDK, e GitHub Actions. Claude Desktop e le sessioni cloud non chiamano `apiKeyHelper` né leggono queste variabili di ambiente: utilizzano OAuth, ad eccezione delle sessioni desktop che eseguono una [configurazione di inferenza di terze parti](/docs/it/llm-gateway-connect#desktop-app), che si autenticano con le credenziali di quella configurazione.

<h3 id="renew-an-expiring-login">
  Rinnovare un accesso in scadenza
</h3>

Quando l'accesso creato con `/login` è entro cinque giorni dalla scadenza, Claude Code mostra un avviso all'avvio: `Your login expires in 3 days · run /login to renew`. Richiede Claude Code v2.1.203 o successivo.

Eseguite `/login` per rinnovare. L'avviso è informativo e non blocca mai una richiesta: l'autenticazione continua a funzionare fino a quando l'accesso non scade effettivamente. La durata dell'accesso stesso rimane invariata; l'avviso anticipato è ciò che v2.1.203 aggiunge.

Una volta che l'accesso archiviato scade e non può essere aggiornato, ogni richiesta fallisce con [`Login expired · Please run /login`](/docs/it/errors#login-expired) fino a quando non accedete di nuovo. Prima della v2.1.206, un accesso scaduto emergeva come un errore del modello.

L'avviso appare solo quando un accesso claude.ai o Claude Console è la credenziale attiva, e non quando un provider cloud, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper` fornisce la credenziale.

Il rinnovo anticipato è più importante per le sessioni che vengono eseguite in modo automatico. Una [sessione in background nella vista agente](/docs/it/agent-view) o una sessione di [Remote Control](/docs/it/remote-control) che supera la durata dell'accesso smette di fare progressi una volta che la credenziale scade e non può recuperare fino a quando non accedete di nuovo.

<h3 id="authentication-precedence">
  Precedenza di autenticazione
</h3>

Quando sono presenti più credenziali, Claude Code ne sceglie una in questo ordine:

1. Credenziali del provider cloud, quando `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, o `CLAUDE_CODE_USE_FOUNDRY` è impostato. Consultate [integrazioni di terze parti](/docs/it/third-party-integrations) per la configurazione.
2. Variabile di ambiente `ANTHROPIC_AUTH_TOKEN`. Inviata come header `Authorization: Bearer`. Utilizzatela quando si instrada attraverso un [gateway LLM o proxy](/docs/it/llm-gateway) che si autentica con bearer token anziché chiavi API Anthropic.
3. Variabile di ambiente `ANTHROPIC_API_KEY`. Inviata come header `X-Api-Key`. Utilizzatela per l'accesso diretto all'API Anthropic con una chiave dalla [Claude Console](https://platform.claude.com). In modalità interattiva, vi viene chiesto una volta di approvare o rifiutare la chiave, e la Vostra scelta viene ricordata. Per cambiarla in seguito, utilizzate l'interruttore "Use custom API key" in `/config`. L'interruttore appare solo mentre `ANTHROPIC_API_KEY` è impostato nel Vostro ambiente. In modalità non interattiva (`-p`), la chiave viene sempre utilizzata quando presente.
4. Output dello script [`apiKeyHelper`](/docs/it/settings#available-settings). Utilizzatelo per credenziali dinamiche o rotanti, come token di breve durata recuperati da un vault.
5. Variabile di ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Un token OAuth di lunga durata generato da [`claude setup-token`](#generate-a-long-lived-token). Utilizzatelo per pipeline CI e script dove l'accesso tramite browser non è disponibile.
6. Credenziali OAuth di sottoscrizione da `/login`. Questo è il valore predefinito per gli utenti Claude Pro, Max, Team, ed Enterprise.

Una sessione del [gateway delle app Claude](/docs/it/claude-apps-gateway) autenticata si trova al di fuori di questo elenco: è una selezione di provider come Bedrock o Vertex, e ha la precedenza su di essi. Quando esiste una sessione gateway, la CLI si autentica con il token gateway anche se `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, o `CLAUDE_CODE_USE_FOUNDRY` è impostato, e le voci bearer token, chiave API e `apiKeyHelper` sopra non vengono utilizzate.

Se avete una sottoscrizione Claude attiva ma avete anche `ANTHROPIC_API_KEY` impostato nel Vostro ambiente, la chiave API ha la precedenza una volta approvata. Questo può causare errori di autenticazione se la chiave appartiene a un'organizzazione disabilitata o scaduta. Eseguite `unset ANTHROPIC_API_KEY` per tornare alla Vostra sottoscrizione, e controllate `/status` per confermare quale metodo è attivo. La riga `Login method` mostra il Vostro account di sottoscrizione, e una riga `API key` appare quando una chiave API è in uso.

[Claude Code sul Web](/docs/it/claude-code-on-the-web) utilizza sempre le Vostre credenziali di sottoscrizione. Se impostate `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN` nell'ambiente sandbox, non sovrascrivono le Vostre credenziali di sottoscrizione.

<h3 id="generate-a-long-lived-token">
  Generare un token di lunga durata
</h3>

Per pipeline CI, script, o altri ambienti dove l'accesso interattivo tramite browser non è disponibile, generate un token OAuth di un anno con `claude setup-token`:

```bash theme={null}
claude setup-token
```

Il comando vi guida attraverso l'autorizzazione OAuth e stampa un token nel terminale. Non salva il token da nessuna parte; copiatelo e impostatelo come variabile di ambiente `CLAUDE_CODE_OAUTH_TOKEN` ovunque vogliate autenticarvi:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Questo token si autentica con la Vostra sottoscrizione Claude e richiede un piano Pro, Max, Team, o Enterprise. È limitato solo all'inferenza e non può stabilire sessioni di [Remote Control](/docs/it/remote-control).

[Bare mode](/docs/it/headless#start-faster-with-bare-mode) non legge `CLAUDE_CODE_OAUTH_TOKEN`. Se il Vostro script passa `--bare`, autenticatevi con `ANTHROPIC_API_KEY` o un `apiKeyHelper` invece.
