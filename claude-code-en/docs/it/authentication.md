> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Autenticazione

> Accedi a Claude Code e configura l'autenticazione per singoli utenti, team e organizzazioni.

Claude Code supporta molteplici metodi di autenticazione a seconda della Vostra configurazione. I singoli utenti possono accedere con un account Claude.ai, mentre i team possono utilizzare Claude for Teams o Enterprise, la Claude Console, o un provider cloud come Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Accedi a Claude Code
</h2>

Dopo aver [installato Claude Code](/docs/it/setup#install-claude-code), eseguite `claude` nel vostro terminale. Al primo avvio, Claude Code apre una finestra del browser per consentirvi di accedere. Se avete impostato la variabile di ambiente `ANTHROPIC_API_KEY`, Claude Code salta il prompt di accesso e vi chiede invece di approvare la chiave.

Se il browser non si apre automaticamente, premete `c` per copiare l'URL di accesso negli appunti, quindi incollatelo nel vostro browser.

Se il vostro browser mostra un codice di accesso invece di reindirizzarvi dopo aver effettuato l'accesso, incollatelo nel terminale al prompt `Paste code here if prompted`. Questo accade quando il browser non riesce a raggiungere il server di callback locale di Claude Code, il che è comune in WSL2, sessioni SSH e container.

Quando l'accesso è completato, il terminale mostra `Login successful` e vi chiede di premere `Enter` per continuare.

Potete autenticarvi con uno di questi tipi di account:

* **Sottoscrizione Claude Pro o Max**: accedete con il vostro account claude.ai. Sottoscrivete su [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams o Enterprise**: accedete con l'account claude.ai che l'amministratore del vostro team vi ha invitato a utilizzare.
* **Claude Console**: accedete con le vostre credenziali Console. L'amministratore deve avervi [invitato](#claude-console-authentication) prima. Potete accedere con o senza [creare una chiave API](#sign-in-without-an-api-key).
* **Provider cloud**: se la vostra organizzazione utilizza [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry), impostate le variabili di ambiente richieste prima di eseguire `claude`, oppure selezionate **3rd-party platform** al prompt di accesso, che avvia una procedura guidata di configurazione interattiva per Bedrock e Vertex AI. Non è necessario alcun accesso tramite browser.
* **Cloud gateway**: se la vostra organizzazione esegue un [gateway di app Claude](/docs/it/claude-apps-gateway) auto-ospitato, accedete con SSO aziendale tramite `/login`. Il token emesso dal gateway è l'unica credenziale della sessione.

Gli amministratori possono indirizzare quale metodo di accesso utilizzano gli sviluppatori e richiedere che gli accessi a claude.ai appartengano a un'organizzazione specifica; consultate [Limitare l'accesso alla vostra organizzazione](#restrict-login-to-your-organization).

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

* **Claude for Teams**: piano self-service con funzionalità di collaborazione, strumenti di amministrazione, SSO, gestione della fatturazione e [impostazioni gestite dal server](/docs/it/server-managed-settings) per la configurazione Claude Code a livello organizzativo. Ideale per team più piccoli.
* **Claude for Enterprise**: aggiunge domain capture, autorizzazioni basate su ruoli e API di conformità. Ideale per organizzazioni più grandi con requisiti di sicurezza e conformità.

<Steps>
  <Step title="Sottoscrivete">
    Sottoscrivete a [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) o contattate il team di vendita per [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Invitate i membri del team">
    Invitate i membri del team dalla dashboard di amministrazione.
  </Step>

  <Step title="Installate e accedete">
    I membri del team installano Claude Code e accedono con i loro account claude.ai.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Autenticazione Claude Console
</h3>

Per le organizzazioni che preferiscono la fatturazione basata su API, potete configurare l'accesso tramite la Claude Console.

<Steps>
  <Step title="Creare o utilizzare un account Console">
    Utilizzate il vostro account Claude Console esistente o createne uno nuovo.
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

<h4 id="sign-in-without-an-api-key">
  Accedere senza una chiave API
</h4>

Potete accedere al vostro account Console senza creare una chiave API, anche quando la vostra organizzazione non consente agli sviluppatori di crearle. Scegliete l'account Anthropic Console al prompt `/login` e Claude Code vi chiede come desiderate accedere. Richiede Claude Code v2.1.242 o successivo. Entrambi i percorsi vi accedono a Console nel browser e differiscono in ciò che Claude Code memorizza successivamente:

* **Accedete con il vostro account Console**, etichettato `(recommended)`: Claude Code mantiene il token OAuth da quell'accesso e lo memorizza come un [profilo Anthropic](#anthropic-profiles-and-federation-credentials). Non crea alcuna chiave API
* **Creare una chiave API**, etichettato `(legacy)`: Claude Code crea una chiave API Console per voi e la memorizza con le vostre altre credenziali

In pratica, il profilo memorizza un accesso OAuth mentre una chiave API è una credenziale statica: Claude Code aggiorna automaticamente l'accesso del profilo e, quando l'aggiornamento non riesce, le richieste non riescono con [Anthropic profile login expired](/docs/it/errors#anthropic-profile-login-expired) finché non accedete di nuovo.

Non avete la scelta su ogni macchina. Claude Code crea una chiave API senza chiedere in questi casi:

* Eseguite contro un provider cloud, come [Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry](/docs/it/third-party-integrations) o [Claude Platform on AWS](/docs/it/claude-platform-on-aws)
* Qualsiasi file di impostazioni imposta [`forceLoginOrgUUID`](#restrict-login-to-your-organization), o imposta `forceLoginMethod` su `"claudeai"` o `"console"`
* Una fonte di impostazioni gestite sulla vostra macchina, come il file di impostazioni gestite, un profilo MDM, o le impostazioni gestite dal server memorizzate nella cache, esiste ma Claude Code [non può leggerla](/docs/it/managed-settings#invalid-entries-in-managed-settings) e nessun'altra fonte gestita fornisce una policy

Annullate l'impostazione di `ANTHROPIC_API_KEY` prima di accedere senza una chiave. Un profilo scritto dall'accesso Console di Claude Code stesso, o dall'accesso `ant auth login` della CLI Claude Platform, è lo stesso tipo di credenziale, quindi accedere di nuovo lo sostituisce.

Dopo aver effettuato l'accesso senza una chiave, avete un profilo invece di una chiave API memorizzata:

* **Quale profilo scrive**: Claude Code scrive il profilo denominato da `ANTHROPIC_PROFILE`, o il vostro profilo attivo, o `default`. Se quel profilo è un profilo di federazione, Claude Code rifiuta l'accesso invece di sovrascriverlo
* **Da cosa vi disconnette**: Claude Code vi disconnette da qualsiasi accesso claude.ai memorizzato sulla macchina
* **Come annullarlo**: eseguite `/logout`, che rimuove e revoca la credenziale che questo accesso ha scritto

Se la vostra organizzazione utilizza [server-managed settings](/docs/it/server-managed-settings), si applicano a questo accesso su Claude Code v2.1.257 o successivo.

Tutto il resto sui profili si applica a questo accesso, incluso dove si classifica rispetto alle vostre altre credenziali, la riga `Profile` che ottenete in `/status`, e le funzionalità che necessitano di un accesso claude.ai. Vedete [Anthropic profiles and federation credentials](#anthropic-profiles-and-federation-credentials).

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

<h3 id="restrict-login-to-your-organization">
  Limitare l'accesso alla vostra organizzazione
</h3>

Per richiedere che gli accessi claude.ai degli sviluppatori appartengano a una specifica organizzazione Anthropic, impostate [`forceLoginMethod`](/docs/it/settings-reference#forceloginmethod) e [`forceLoginOrgUUID`](/docs/it/settings-reference#forceloginorguuid) nelle [impostazioni gestite](/docs/it/managed-settings). Impostate `forceLoginOrgUUID` al vostro ID organizzazione, mostrato nelle [impostazioni di amministrazione claude.ai](https://claude.ai/admin-settings/organization) per le organizzazioni Claude for Teams o Enterprise. Claude Code segnala un errore per un accesso claude.ai a qualsiasi altra organizzazione e esce all'avvio se la credenziale claude.ai in uso appartiene a un'organizzazione che non è elencata.

Per gli accessi Claude Console, Claude Code utilizza `forceLoginOrgUUID` per pre-selezionare l'organizzazione nella pagina di accesso Console quando lo impostate su un singolo ID organizzazione Console, mostrato su [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization). Non controlla a quale organizzazione appartiene la credenziale Console risultante, all'accesso o all'avvio, e uno sviluppatore che ha effettuato l'accesso con un account Console prima che distribuiste le chiavi rimane connesso.

Se impostate `forceLoginOrgUUID` in qualsiasi file di impostazioni, Claude Code smette di offrire l'[accesso Console senza chiave](#sign-in-without-an-api-key) nelle sessioni a cui si applica quel file e crea una chiave API. Per indirizzare gli sviluppatori all'accesso claude.ai, impostate `forceLoginMethod` su `"claudeai"`.

Gli sviluppatori possono accedere da diversi percorsi: il flusso terminale `/login`, l'[estensione VS Code](/docs/it/vs-code), l'Agent SDK, `claude setup-token`, `/install-github-app`, e l'accesso [gateway](/docs/it/claude-apps-gateway) per le organizzazioni che instradano attraverso un gateway cloud. Su Claude Code v2.1.212 o successivo, ogni percorso applica `forceLoginMethod`; prima di v2.1.212, solo gli accessi terminali applicavano entrambe le chiavi. Sulla schermata di accesso interattiva del terminale, raggiunta da `/login` o dall'onboarding al primo avvio, Claude Code pre-seleziona un metodo `claudeai` o `console` senza applicarlo, quindi anche con `forceLoginMethod` impostato su `"claudeai"`, uno sviluppatore può comunque completare un accesso Console lì. I percorsi differiscono su `forceLoginOrgUUID`:

* **Accessi terminale, estensione VS Code e Agent SDK**: verificano `forceLoginOrgUUID` per gli accessi dell'account claude.ai
* **`claude setup-token` e `/install-github-app`**: applicano solo `forceLoginMethod`, quindi possono coniare un token in un'organizzazione diversa
* **Accesso [gateway](/docs/it/claude-apps-gateway)**: selezionato da `forceLoginMethod: "gateway"` piuttosto che limitato da esso, e non autentica rispetto a un'organizzazione Anthropic, quindi `forceLoginOrgUUID` non si applica; utilizzate il vostro provider di identità gateway per limitare l'accesso

Distribuite le chiavi attraverso il vostro strumento di gestione dei dispositivi. Le [impostazioni gestite dal server](/docs/it/server-managed-settings) raggiungono solo gli account che sono già autenticati nella vostra organizzazione, quindi non possono reindirizzare il primo accesso di uno sviluppatore. Se la vostra organizzazione distribuisce anche impostazioni gestite dal server, impostate le chiavi in entrambi i luoghi: le fonti di impostazioni gestite [non si uniscono](/docs/it/server-managed-settings#settings-precedence), e le impostazioni gestite dal server memorizzate nella cache sostituiscono il file gestito dal dispositivo, a parte alcuni [eccezioni per chiave](/docs/it/server-managed-settings#per-key-exceptions-across-managed-sources). `forceLoginOrgUUID` e i valori `"claudeai"` e `"console"` di `forceLoginMethod` non sono tra quelle eccezioni, quindi manteneteli in entrambi i luoghi.

Le chiavi decidono anche se una sessione che non utilizza una credenziale di accesso può iniziare. Vedete [`forceLoginOrgUUID`](/docs/it/settings-reference#forceloginorguuid) nel riferimento delle impostazioni per il comportamento completo.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper`**: bloccati all'avvio, poiché l'appartenenza all'organizzazione non può essere verificata per una credenziale di ambiente
* **Sessioni del provider cloud come Amazon Bedrock**: non bloccate, perché si autenticano rispetto al vostro provider cloud. Limitatele attraverso le vostre policy IAM cloud
* **[Profilo Anthropic o credenziali di federazione](#anthropic-profiles-and-federation-credentials)**: non bloccate, e le chiavi non controllano a quale organizzazione appartiene il profilo

<h2 id="credential-management">
  Gestione delle credenziali
</h2>

Claude Code gestisce in modo sicuro le Vostre credenziali di autenticazione:

* **Posizione di archiviazione**:
  * Su macOS, le credenziali sono archiviate nel Keychain macOS crittografato. Quando il Keychain rifiuta la scrittura, ad esempio quando è bloccato in una sessione SSH, Claude Code archivia il Vostro accesso in `~/.claude/.credentials.json` con modalità file `0600` invece, lo stesso archiviazione che utilizza su Linux. Un accesso Console che crea una chiave API fallisce fino a quando il Keychain non è scrivibile. Per spostare il Vostro accesso di nuovo nel Keychain, seguite [i passaggi di recupero](/docs/it/troubleshoot-install#not-logged-in-or-token-expired).
  * Su Linux, le credenziali sono archiviate in `~/.claude/.credentials.json` con modalità file `0600`.
  * Su Windows, le credenziali sono archiviate in `%USERPROFILE%\.claude\.credentials.json` e ereditano i controlli di accesso della directory del profilo utente, che limita il file al Vostro account utente per impostazione predefinita.
  * Se avete impostato la variabile di ambiente `CLAUDE_CONFIG_DIR`, Claude Code mantiene il file `.credentials.json` in quella directory, incluso il file che il fallback macOS scrive, e chiave la voce macOS Keychain a quella directory anche, quindi una sessione con un `CLAUDE_CONFIG_DIR` diverso legge una voce diversa.
  * Claude Code gestisce `.credentials.json` attraverso `/login` e `/logout`. Per instradare le richieste attraverso un endpoint API personalizzato, impostate invece la variabile di ambiente [`ANTHROPIC_BASE_URL`](/docs/it/env-vars).
* **Tipi di autenticazione supportati**: credenziali claude.ai, credenziali API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, credenziali del profilo Anthropic e [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), e token di sessione del [gateway delle app Claude](/docs/it/claude-apps-gateway).
* **Script di credenziali personalizzati**: configurate l'impostazione [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) per eseguire uno script shell che restituisce una chiave API.
* **Intervalli di aggiornamento**: Claude Code esegue di nuovo `apiKeyHelper` dopo cinque minuti per impostazione predefinita. Impostate la variabile di ambiente `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` per intervalli di aggiornamento personalizzati. Consultate [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) per gli altri casi in cui Claude Code esegue di nuovo l'helper.
* **Avviso di helper lento**: se `apiKeyHelper` impiega più di 10 secondi per restituire una chiave, Claude Code visualizza un avviso nella barra del prompt mostrando il tempo trascorso. Se vedete questo avviso regolarmente, verificate se lo script di credenziali può essere ottimizzato.
* **Errori dell'helper**: quando lo script esce con un errore, scade il timeout, o non stampa nulla, le richieste falliscono con [`Your apiKeyHelper script is failing`](/docs/it/errors#your-apikeyhelper-script-is-failing) entro tre tentativi. Prima della v2.1.208, gli errori dell'helper emergevano come un generico 401 dopo circa dieci tentativi silenziosi.

`apiKeyHelper`, `ANTHROPIC_API_KEY`, e `ANTHROPIC_AUTH_TOKEN` si applicano alla CLI e alle superfici che la avvolgono, inclusa l'estensione VS Code, l'Agent SDK, e GitHub Actions. Claude Desktop e le sessioni cloud non chiamano `apiKeyHelper` né leggono queste variabili di ambiente: utilizzano OAuth, ad eccezione delle sessioni desktop che eseguono una [configurazione di inferenza di terze parti](/docs/it/llm-gateway-connect#desktop-app), che si autenticano con le credenziali di quella configurazione.

<h3 id="renew-an-expiring-login">
  Rinnovare un accesso in scadenza
</h3>

Quando l'accesso creato con `/login` è entro tre giorni dalla scadenza, Claude Code mostra un avviso all'avvio: `Your login expires in 3 days · run /login to renew`. Richiede Claude Code v2.1.203 o successivo. Prima della v2.1.217, l'avviso appariva cinque giorni prima.

Eseguite `/login` per rinnovare. L'avviso è informativo e non blocca mai una richiesta: l'autenticazione continua a funzionare fino a quando l'accesso non scade effettivamente. La durata dell'accesso stesso rimane invariata; l'avviso anticipato è ciò che v2.1.203 aggiunge.

Una volta che l'accesso archiviato scade e non può essere aggiornato, ogni richiesta del modello fallisce con [`Login expired · Please run /login`](/docs/it/errors#login-expired) fino a quando non accedete di nuovo. Prima della v2.1.206, Claude Code segnalava un accesso scaduto sulle richieste del modello come un errore del modello.

Potete verificare questo stato prima che una richiesta fallisca: [`/status`](/docs/it/commands) mostra una riga `Login` che legge `Expired — log in again`, più l'organizzazione e l'email che ha salvato per l'accesso scaduto. La riga appare solo quando l'accesso claude.ai o Claude Console salvato è la credenziale attiva. La riga richiede Claude Code v2.1.210 o successivo.

L'avviso appare solo quando un accesso claude.ai o Claude Console è la credenziale attiva, e non quando un provider cloud, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper` fornisce la credenziale.

Il rinnovo anticipato è più importante per le sessioni che vengono eseguite in modo automatico. Una [sessione in background nella vista agente](/docs/it/agent-view) o una sessione di [Remote Control](/docs/it/remote-control) che supera la durata dell'accesso smette di fare progressi una volta che la credenziale scade e non può recuperare fino a quando non accedete di nuovo.

<h3 id="authentication-precedence">
  Precedenza di autenticazione
</h3>

Quando sono presenti più credenziali, Claude Code ne sceglie una in questo ordine:

1. Credenziali del provider cloud, quando `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, o `CLAUDE_CODE_USE_FOUNDRY` è impostato. Consultate [integrazioni di terze parti](/docs/it/third-party-integrations) per la configurazione.
2. Variabile di ambiente `ANTHROPIC_AUTH_TOKEN`. Inviata come header `Authorization: Bearer`. Utilizzatela quando si instrada attraverso un [gateway LLM o proxy](/docs/it/llm-gateway) che si autentica con bearer token anziché chiavi API Anthropic.
3. Variabile di ambiente `ANTHROPIC_API_KEY`. Inviata come header `X-Api-Key`. Utilizzatela per l'accesso diretto all'API Anthropic con una chiave dalla [Claude Console](https://platform.claude.com). In modalità interattiva, vi viene chiesto una volta di approvare o rifiutare la chiave, e la Vostra scelta viene ricordata. Per cambiarla in seguito, utilizzate l'interruttore "Use custom API key" in `/config`. L'interruttore appare solo mentre `ANTHROPIC_API_KEY` è impostato nel Vostro ambiente. In modalità non interattiva (`-p`), la chiave viene sempre utilizzata quando presente.
4. Output dello script [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper). Utilizzatelo per credenziali dinamiche o rotanti, come token di breve durata recuperati da un vault.
5. Variabile di ambiente `CLAUDE_CODE_OAUTH_TOKEN`. Un token OAuth di lunga durata generato da [`claude setup-token`](#generate-a-long-lived-token). Utilizzatelo per pipeline CI e script dove l'accesso tramite browser non è disponibile. Se eseguite `/login` mentre la variabile è impostata, Claude Code passa la sessione corrente al nuovo accesso, ma legge di nuovo la variabile in ogni nuova sessione fino a quando non la rimuovete dal Vostro profilo shell o dal blocco `env` di un [file di impostazioni](/docs/it/settings).
6. Credenziali del profilo Anthropic e di federazione, le credenziali che la CLI `ant` e Workload Identity Federation utilizzano. Un profilo che `ant auth login` ha scritto si classifica qui solo quando lo nominate in `ANTHROPIC_PROFILE`; altrimenti si classifica al di sotto di `/login`. Consultate [Profili Anthropic e credenziali di federazione](#anthropic-profiles-and-federation-credentials).
7. Credenziali OAuth di sottoscrizione da `/login`. Questo è il valore predefinito per gli utenti Claude Pro, Max, Team, ed Enterprise.

Una sessione del [gateway delle app Claude](/docs/it/claude-apps-gateway) autenticata si trova al di fuori di questo elenco: è una selezione di provider come Amazon Bedrock o Google Cloud's Agent Platform, e ha la precedenza su di essi. Quando esiste una sessione gateway, la CLI si autentica con il token gateway anche se `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, o `CLAUDE_CODE_USE_FOUNDRY` è impostato, e le fonti di credenziali sopra come il bearer token, la chiave API, `apiKeyHelper`, e i profili non vengono utilizzati.

Se le [impostazioni gestite](/docs/it/managed-settings) della Vostra macchina impostano [`forceLoginMethod`](/docs/it/settings-reference#forceloginmethod) a `"gateway"` o impostano [`forceLoginGatewayUrl`](/docs/it/settings-reference#forcelogingatewayurl), e non selezionate un provider cloud attraverso una variabile come `CLAUDE_CODE_USE_BEDROCK` o `CLAUDE_CODE_USE_VERTEX`, la Vostra sessione utilizza solo l'accesso gateway. Claude Code salta le altre fonti di credenziali e vi chiede di accedere con `/login`. Consultate [Administrator policy requires a Cloud gateway sign-in](/docs/it/errors#administrator-policy-requires-a-cloud-gateway-sign-in) per quello che vedete con ogni credenziale residua. Prima della v2.1.261, o prima della v2.1.265 su una macchina che imposta solo `forceLoginGatewayUrl`, Claude Code utilizzava un accesso salvato residuo su queste macchine fino a quando non avete effettuato l'accesso al gateway.

Se avete una sottoscrizione Claude attiva ma avete anche `ANTHROPIC_API_KEY` impostato nel Vostro ambiente, Claude Code utilizza la chiave API una volta che l'approvate. Questo può causare errori di autenticazione se la chiave appartiene a un'organizzazione disabilitata o scaduta.

Eseguite `unset ANTHROPIC_API_KEY` per tornare alla Vostra sottoscrizione, e controllate `/status` per confermare quale metodo è attivo. Quando un accesso e una chiave API sono entrambi configurati, `/status` contrassegna la credenziale che non è in uso.

[Claude Code sul Web](/docs/it/claude-code-on-the-web) utilizza sempre le Vostre credenziali di sottoscrizione. Se impostate `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN` nell'ambiente cloud, non sovrascrivono le Vostre credenziali di sottoscrizione.

<h4 id="anthropic-profiles-and-federation-credentials">
  Profili Anthropic e credenziali di federazione
</h4>

Un profilo è un file di configurazione delle credenziali denominato nella Vostra [directory di configurazione Anthropic](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory), per impostazione predefinita `~/.config/anthropic` su macOS e Linux o `%APPDATA%\Anthropic` su Windows. La modalità di autenticazione di un profilo è `oidc_federation` quando lo configurate per [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) o `user_oauth` quando [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication) l'ha scritto o avete [effettuato l'accesso a un account Console senza una chiave API](#sign-in-without-an-api-key).

Claude Code non legge profili o variabili di federazione in [modalità bare](/docs/it/headless#start-faster-with-bare-mode), in Claude Desktop, o in sessioni cloud. In quelle sessioni, `/status` non mostra alcuna riga `Profile`.

Claude Code controlla tre fonti in questo ordine e si ferma alla prima che è impostata. La tabella mostra cosa imposta ogni fonte e dove si classifica rispetto alla Vostra credenziale `/login`.

| Fonte                    | Impostato da                                                                                                                                                                          | Classificazione rispetto a `/login`                                                                                                                                    |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Profilo denominato       | `ANTHROPIC_PROFILE`                                                                                                                                                                   | Sopra, qualunque sia la modalità di autenticazione del profilo                                                                                                         |
| Variabili di federazione | `ANTHROPIC_FEDERATION_RULE_ID` e `ANTHROPIC_ORGANIZATION_ID`, entrambe impostate                                                                                                      | Sopra                                                                                                                                                                  |
| Profilo attivo           | Il file [`active_config`](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) nella Vostra directory di configurazione, o un profilo denominato `default` | Sopra quando la sua modalità di autenticazione è `oidc_federation`; sotto una credenziale `/login` funzionante quando la sua modalità di autenticazione è `user_oauth` |

La regola `user_oauth` impedisce a un profilo `ant auth login` residuo di spostare le Vostre richieste fuori dall'account a cui avete effettuato l'accesso con `/login`. Per le variabili di federazione, Claude Code legge anche le altre variabili nel [riferimento WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables), come `ANTHROPIC_IDENTITY_TOKEN_FILE`, quando scambia il Vostro token di identità. Per il formato del file di profilo, consultate il [riferimento WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file).

Per confermare quale fonte Claude Code ha scelto, eseguite `/status`. Una riga `Profile` nomina la fonte al posto della riga `Login method`, e quando il profilo è la credenziale in uso, le righe `Organization` e `Email` mostrano il suo account.

Se avviate Claude Code con `--debug`, scrive anche una riga `Using Anthropic profile auth` con il nome della fonte nel log di debug in `~/.claude/debug/<session-id>.txt`. Quando Claude Code passa oltre un profilo attivo `user_oauth` perché avete una credenziale `/login` funzionante, scrive un avviso nel log di debug dicendo che sta utilizzando l'accesso claude.ai.

Quando l'accesso di un profilo `user_oauth` è scaduto e Claude Code non può rinnovarlo, le richieste falliscono con [Anthropic profile login expired](/docs/it/errors#anthropic-profile-login-expired).

Le funzionalità che necessitano del Vostro accesso claude.ai, come i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) e [`/schedule`](/docs/it/routines), non sono disponibili mentre una di queste fonti è selezionata. Per impedire a Claude Code di selezionare una fonte:

* **Profilo denominato o variabili di federazione**: annullate `ANTHROPIC_PROFILE`, o annullate una variabile di federazione
* **Profilo attivo**: eseguite `/logout` per un profilo `user_oauth` la cui credenziale corrente avete scritto [effettuando l'accesso a un account Console senza una chiave API](#sign-in-without-an-api-key), eseguite `ant auth logout` per uno la cui credenziale corrente `ant auth login` ha scritto, o eliminate il file del profilo da `configs/` nella Vostra directory di configurazione per entrambe le modalità di autenticazione

<h3 id="generate-a-long-lived-token">
  Generare un token di lunga durata
</h3>

Per pipeline CI, script, o altri ambienti dove l'accesso interattivo tramite browser non è disponibile, generate un token OAuth di un anno con `claude setup-token`:

```bash theme={null}
claude setup-token
```

Il comando apre lo stesso flusso di autorizzazione del browser di `/login`, e il token viene stampato nel terminale dopo che approvate l'accesso nel browser. Non salva il token da nessuna parte; copiatelo e impostatelo come variabile di ambiente `CLAUDE_CODE_OAUTH_TOKEN` ovunque vogliate autenticarvi:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Questo token si autentica con la Vostra sottoscrizione Claude e richiede un piano Pro, Max, Team, o Enterprise. Può solo fare richieste di modello, quindi non può stabilire sessioni di [Remote Control](/docs/it/remote-control) o recuperare [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai). I server MCP che configurate localmente funzionano ancora.

[Bare mode](/docs/it/headless#start-faster-with-bare-mode) non legge `CLAUDE_CODE_OAUTH_TOKEN`. Se il Vostro script passa `--bare`, autenticatevi con `ANTHROPIC_API_KEY` o un `apiKeyHelper` invece.
