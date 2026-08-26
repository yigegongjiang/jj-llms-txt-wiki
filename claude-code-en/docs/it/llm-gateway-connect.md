> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Connetti Claude Code a un gateway LLM

> Indirizza Claude Code al gateway LLM della tua organizzazione. Verifica se il tuo amministratore lo ha già configurato, oppure imposta l'URL di base e le credenziali da solo, quindi verifica la connessione e risolvi gli errori del gateway.

Un [gateway LLM](/docs/it/llm-gateway) è un proxy che la tua organizzazione esegue tra Claude Code e il provider del modello. Quando la tua organizzazione ne utilizza uno, Claude Code si autentica al gateway con una credenziale che la tua organizzazione emette invece del tuo accesso personale a claude.ai.

Questa pagina è per gli sviluppatori che eseguono Claude Code attraverso un gateway gestito dalla loro organizzazione. Copre due percorsi: [verificare se l'amministratore lo ha già configurato per te](#check-for-an-existing-configuration) e [configurarlo da solo](#configure-claude-code-yourself) quando non lo ha fatto.

<Note>
  * Per distribuire un gateway per la tua organizzazione, vedi [Distribuisci un gateway LLM](/docs/it/llm-gateway-rollout)
  * Per sapere cosa Claude Code invia a un gateway, vedi il [riferimento del protocollo gateway](/docs/it/llm-gateway-protocol)
</Note>

<h2 id="check-for-an-existing-configuration">
  Verifica di una configurazione esistente
</h2>

Gli amministratori possono distribuire l'indirizzo del gateway e la credenziale attraverso [impostazioni gestite](/docs/it/settings#settings-files), gestione dei dispositivi, o un [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), in modo che Claude Code le raccolga all'avvio senza che tu debba impostare nulla. Per verificare se la tua organizzazione lo ha già fatto:

<Steps>
  <Step title="Avvia Claude Code">
    Esegui `claude`. Se si apre alla schermata di accesso invece di una sessione, nessuna credenziale del gateway è stata distribuita; [configurala da solo](#configure-claude-code-yourself) di seguito.
  </Step>

  <Step title="Controlla la scheda Status">
    Se Claude Code ha avviato una sessione senza mostrare la schermata di accesso, esegui `/status`, apri la scheda **Status** e controlla due righe:

    * `Anthropic base URL`: questa riga appare solo quando è impostato un indirizzo del gateway. Se non è presente, Claude Code non è indirizzato al gateway; [configuralo da solo](#configure-claude-code-yourself) di seguito.
    * `Auth token` o `API key`: una riga che nomina `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY`, o un `apiKeyHelper` conferma che una credenziale del gateway è attiva. Una riga `Login method` che nomina un account claude.ai significa che la credenziale non è stata distribuita; [impostala da solo](#set-the-credential-variable).
  </Step>

  <Step title="Invia un messaggio di prova">
    Chiudi il menu `/status` e invia qualsiasi prompt in Claude Code. Una risposta normale da Claude, senza errori, conferma che la connessione al gateway funziona.
  </Step>
</Steps>

Se entrambe le righe nel menu `/status` sembrano corrette ma il messaggio a Claude fallisce, vedi la [tabella di risoluzione dei problemi](#troubleshoot-gateway-errors).

<h2 id="configure-claude-code-yourself">
  Configura Claude Code da solo
</h2>

Per configurare Claude Code per il gateway da solo, hai bisogno dal tuo team del gateway:

* L'URL di base del gateway
* Una credenziale: una stringa di chiave o token, o un comando che ne recupera una
  * Se il tuo team del gateway non ha detto quale tipo di credenziale è, la sezione [variabile di credenziale](#set-the-credential-variable) di seguito copre cosa provare

Le sezioni di seguito coprono la configurazione in ordine:

* [Imposta la variabile di credenziale](#set-the-credential-variable) e [imposta l'URL di base](#set-the-base-url-and-credential): le due variabili di cui ogni connessione gateway ha bisogno
* [Verifica la connessione](#verify-the-connection): conferma che funziona prima di persistere qualsiasi cosa
* [Configura ogni superficie](#configure-each-surface): se stai utilizzando una superficie diversa dalla CLI di Claude Code, come VS Code, vedi come configurarla con le tue credenziali del gateway
* [Configurazione aggiuntiva](#additional-configuration): variabili che alcuni gateway necessitano oltre all'URL di base e alla credenziale, come un'intestazione personalizzata, un helper di credenziale, scoperta di modelli, un URL di base in formato provider, o disattivare il traffico al di fuori del percorso del gateway. Imposta questi solo se il tuo amministratore li ha nominati o la tua rete limita l'uscita

<h3 id="set-the-credential-variable">
  Imposta la variabile di credenziale
</h3>

Per autenticare Claude Code al gateway, imposta la tua credenziale in una variabile di ambiente. Quale variabile dipende da cosa il tuo team del gateway ti ha detto:

| Imposta la credenziale in                               | Usa quando                                                               |
| :------------------------------------------------------ | :----------------------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | Il tuo team del gateway ha detto "bearer token" o "Authorization header" |
| `ANTHROPIC_API_KEY`                                     | Il tuo team del gateway ha detto "API key" o "x-api-key"                 |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | La credenziale ruota o proviene da un vault                              |

Se non ti è stato detto quale tipo, usa `ANTHROPIC_AUTH_TOKEN`; la [richiesta di verifica](#verify-the-connection) di seguito mostra come dire se hai bisogno di cambiare.

<h3 id="set-the-base-url-and-credential">
  Imposta l'URL di base e la credenziale
</h3>

Imposta l'URL di base del gateway e la variabile di credenziale che hai scelto sopra come variabili di ambiente. Gli esempi usano `ANTHROPIC_AUTH_TOKEN`; sostituiscilo con `ANTHROPIC_API_KEY` se quella è [la variabile che hai scelto](#set-the-credential-variable). Puoi impostarli [nella tua shell](#set-as-shell-environment-variables), che dura per una sessione di terminale, o [in un file di impostazioni di Claude Code](#set-in-a-settings-file), che persiste ovunque Claude Code viene eseguito.

Per la tua prima connessione, inizia con esportazioni di shell ed esegui la [richiesta di verifica](#verify-the-connection) prima di spostare i valori in un file di impostazioni.

<h4 id="set-as-shell-environment-variables">
  Imposta come variabili di ambiente della shell
</h4>

Sostituisci i valori con quelli che il tuo team del gateway ti ha dato:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

Le esportazioni di shell si applicano solo a quella sessione di terminale e ai programmi avviati da essa; un editor lanciato dal dock o dal menu Start non le vedrà. Per farle persistere tra i nuovi terminali, aggiungi le stesse righe al tuo profilo di shell, come `~/.zshrc`, `~/.bashrc`, o il tuo `$PROFILE` di PowerShell, o usa un file di impostazioni invece.

<h4 id="set-in-a-settings-file">
  Imposta in un file di impostazioni
</h4>

Per fare in modo che la configurazione si applichi ovunque Claude Code viene eseguito senza dipendere dalla tua shell, imposta le variabili nel blocco `env` di un [file di impostazioni](/docs/it/settings). I file di impostazioni hanno ambiti diversi:

* `~/.claude/settings.json` si applica a tutti i tuoi progetti. Su Windows il percorso è `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` si applica a un progetto. Claude Code lo aggiunge al tuo gitignore quando crea il file; se lo crei tu stesso, aggiungilo al tuo gitignore manualmente prima in modo da non committere accidentalmente la tua credenziale

<Warning>
  Non mettere la credenziale nel `.claude/settings.json` di un progetto. Quel file è committato e condiviso con tutti coloro che clonano il repository.
</Warning>

Il blocco `env` ha lo stesso aspetto in entrambi i file:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

Quando sia un'esportazione di shell che un blocco `env` di file di impostazioni impostano la stessa variabile, il valore del file di impostazioni si applica. Esegui `/status` per vedere quale URL di base e fonte di credenziale Claude Code sta utilizzando.

<h3 id="verify-the-connection">
  Verifica la connessione
</h3>

Con le variabili esportate nella tua shell, invia una richiesta di un token al gateway direttamente. Questo conferma che l'URL e la credenziale funzionano prima di aprire Claude Code, quindi un fallimento punta al gateway piuttosto che alla tua configurazione. I comandi di seguito leggono le variabili di shell, quindi hanno bisogno delle [esportazioni di shell](#set-as-shell-environment-variables) anche se metti anche i valori in un file di impostazioni.

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

Se il tuo gateway si aspetta chiavi nell'intestazione `x-api-key`, sostituisci l'intestazione `Authorization` con `x-api-key: $ANTHROPIC_API_KEY` nel comando Bash, o la voce della tabella hash `"Authorization"` con `"x-api-key" = "$env:ANTHROPIC_API_KEY"` nel comando PowerShell.

Una risposta JSON che inizia con `{"id":"msg_` e include un campo `"content":[...]` significa che il gateway è raggiungibile e la credenziale funziona. Un errore che nomina un modello sconosciuto prova comunque che l'URL e la credenziale funzionano, poiché il gateway ha autenticato la richiesta prima di rifiutare il nome del modello; non hai bisogno di trovare un modello che il tuo gateway serve per questo test. Un `401` significa che la credenziale è stata rifiutata: se hai indovinato la variabile, passa all'altra e ri-esporta.

<h4 id="confirm-in-claude-code">
  Conferma in Claude Code
</h4>

Avvia `claude` dalla stessa shell in modo che erediti le esportazioni, invia un messaggio, ed esegui `/status`.

Nella scheda **Status**, la riga `Anthropic base URL` dovrebbe mostrare il tuo indirizzo del gateway, che conferma che le richieste vengono instradate lì; se la riga non è presente, la variabile non ha raggiunto la sessione. Una riga `Auth token` o `API key` che nomina la variabile che hai impostato conferma che la credenziale del gateway è attiva piuttosto che un accesso claude.ai salvato.

Se il messaggio fallisce, o `/status` non mostra l'URL del gateway, vedi la [tabella di risoluzione dei problemi](#troubleshoot-gateway-errors) di seguito.

<h3 id="how-the-credential-variable-maps-to-a-header">
  Come la variabile di credenziale si mappa a un'intestazione
</h3>

Ogni variabile invia la credenziale in un'intestazione HTTP diversa: `ANTHROPIC_AUTH_TOKEN` in `Authorization: Bearer`, `ANTHROPIC_API_KEY` in `x-api-key`, e `apiKeyHelper` in entrambe. Una credenziale nella variabile sbagliata raggiunge il gateway in un'intestazione che non legge, e la richiesta fallisce con `401`. Se la richiesta di verifica ha restituito `401`, passa all'altra variabile e riprova.

<h3 id="conflicts-with-an-existing-login">
  Conflitti con un accesso esistente
</h3>

Una variabile di credenziale del gateway ha precedenza su un accesso claude.ai salvato o una chiave Console. Il tuo accesso claude.ai rimane salvato e inutilizzato mentre la variabile è impostata; annulla l'impostazione della variabile e Claude Code torna ad esso. Con `ANTHROPIC_AUTH_TOKEN`, la variabile ha precedenza immediatamente. Con `ANTHROPIC_API_KEY`, ti viene chiesto una volta in modalità interattiva di approvare la chiave prima che prenda il controllo.

Esegui `/status` per confermare quale fonte di credenziale è attiva. Se l'avvio mostra un avviso di conflitto di autenticazione che nomina due fonti, vedi la prima riga della [tabella di risoluzione dei problemi](#troubleshoot-gateway-errors) per quale eliminare. Per cancellare un accesso salvato in modo che rimanga solo la credenziale del gateway, esegui `/logout`.

<h2 id="configure-each-surface">
  Configura ogni superficie
</h2>

La CLI legge le variabili di ambiente e i file di impostazioni di cui sopra. Le altre superfici sono l'estensione VS Code, l'app desktop, GitHub Actions, Agent SDK, e le superfici cloud come Slack e il web; le sezioni di seguito coprono se quelle impostazioni raggiungono ognuna.

<h3 id="vs-code-extension">
  Estensione VS Code
</h3>

Imposta le variabili del gateway per l'[estensione VS Code](/docs/it/vs-code) in `claudeCode.environmentVariables`, nelle impostazioni utente di VS Code stesso aperte con il comando **Preferences: Open User Settings (JSON)**. L'estensione controlla le credenziali da questa impostazione prima di avviarsi, quindi è il posto affidabile per la credenziale del gateway; i valori in `~/.claude/settings.json` raggiungono il processo generato ma non il controllo di accesso dell'estensione stessa.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  App desktop
</h3>

L'app desktop legge il routing del gateway dalla sua [configurazione di inferenza di terze parti](https://claude.com/docs/third-party/claude-desktop/gateway), non da `ANTHROPIC_BASE_URL` o `settings.json`. Quella configurazione può provenire dalla tua organizzazione o da un modulo nell'app stessa:

* **Distribuita da un amministratore**: se la tua organizzazione ha [distribuito la configurazione](/docs/it/llm-gateway-rollout#distribute-through-managed-settings), l'app desktop instrada attraverso il gateway senza alcuna configurazione da parte tua
* **Configurata localmente**: per i dispositivi senza una configurazione distribuita da un amministratore, apri Help → Troubleshooting → Enable Developer Mode, che riavvia l'app con un menu Developer. Quindi apri Developer → Configure Third-Party Inference e inserisci l'URL di base del tuo gateway. Una configurazione distribuita da un amministratore ha la precedenza e rende questo modulo di sola lettura

Con la configurazione del gateway attiva, l'app desktop esegue sessioni solo sulla tua macchina locale: il selettore di ambiente non offre sessioni SSH o ambienti cloud ospitati da Anthropic, e [Remote Control](/docs/it/remote-control) non è disponibile. Per utilizzare Claude Code su un host remoto attraverso il gateway, esegui la CLI su quell'host con [`ANTHROPIC_BASE_URL` e la credenziale del gateway](#set-the-base-url-and-credential) impostati lì.

Se l'app desktop mostra `Gateway was unreachable`, l'app non ha potuto raggiungere l'URL di base configurato all'avvio; controlla l'URL e il percorso di rete con il [test curl di cui sopra](#verify-the-connection).

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/it/github-actions) legge `ANTHROPIC_BASE_URL` e `ANTHROPIC_CUSTOM_HEADERS` dal blocco `env` del workflow. Passa la credenziale come input `anthropic_api_key` dell'azione; l'azione la imposta come `ANTHROPIC_API_KEY`, quindi raggiunge il gateway nell'intestazione `x-api-key`.

Per un gateway `x-api-key`, imposta l'URL di base in `env` e passa la chiave del gateway come input:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Per un gateway bearer-token, passa lo stesso segreto sia come input `anthropic_api_key` che come `ANTHROPIC_AUTH_TOKEN` nel blocco `env` del workflow. L'azione richiede `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN`, o federazione dell'identità del carico di lavoro prima di avviare Claude Code, e non legge `ANTHROPIC_AUTH_TOKEN`, quindi l'input è lì solo per soddisfare quel controllo di avvio. La variabile env è ciò che mette la chiave nell'intestazione `Authorization` che il gateway legge; la copia in `x-api-key` viene ignorata:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Per le altre opzioni di autenticazione dell'azione, inclusi `CLAUDE_CODE_OAUTH_TOKEN` e federazione dell'identità del carico di lavoro, vedi [Claude Code GitHub Actions](/docs/it/github-actions) e il [README](https://github.com/anthropics/claude-code-action#readme) dell'azione.

<h3 id="agent-sdk">
  Agent SDK
</h3>

L'[Agent SDK](/docs/it/agent-sdk/overview) non ha opzioni specifiche del gateway; passa le variabili di ambiente al processo Claude Code che genera. Ogni SDK accetta un'opzione `env` che imposta l'ambiente del processo generato, e gli SDK TypeScript e Python lo trattano diversamente:

* TypeScript: il processo generato eredita l'ambiente padre per impostazione predefinita, ma impostare `options.env` sostituisce completamente l'ambiente. Distribuisci `process.env` in esso per mantenere le tue variabili del gateway.
* Python: `ClaudeAgentOptions(env=...)` si unisce sopra l'ambiente ereditato, quindi le variabili del gateway impostate nel processo padre si trasportano senza distribuire.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-web-and-remote-control">
  Slack, web e Remote Control
</h3>

[Claude Code in Slack](/docs/it/slack) e [Claude Code sul web](/docs/it/claude-code-on-the-web) sono prodotti ospitati da Anthropic che usano sempre l'API di Anthropic; non fanno parte di una distribuzione del gateway. Le variabili del gateway impostate nella configurazione dell'ambiente di una sessione cloud non vengono applicate. Se il tuo traffico deve rimanere sul gateway, non abilitare queste superfici per quegli utenti.

[Remote Control](/docs/it/remote-control) e [dettatura vocale](/docs/it/voice-dictation) si basano entrambi su un'identità claude.ai: Remote Control per accoppiare una sessione live con il tuo account, e dettatura vocale per raggiungere l'endpoint di trascrizione claude.ai. Non sono disponibili mentre `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o un `apiKeyHelper` è attivo. A partire da v2.1.196, Remote Control è anche disabilitato mentre `ANTHROPIC_BASE_URL` punta a un host non-Anthropic, quindi accedere con claude.ai non è sufficiente da solo.

Per ripristinare una delle due funzioni, accedi con claude.ai e annulla l'impostazione delle variabili del gateway che controlla. La sezione Remote Control di `claude doctor` nomina la variabile di credenziale da annullare.

* Dettatura vocale: annulla l'impostazione della credenziale del gateway
* Remote Control: annulla l'impostazione della credenziale del gateway e `ANTHROPIC_BASE_URL`

<h2 id="additional-configuration">
  Configurazione aggiuntiva
</h2>

Queste impostazioni coprono casi oltre l'URL di base e la credenziale. Impostale solo se le istruzioni del tuo amministratore, le regole di uscita della tua rete, o la [tabella di risoluzione dei problemi](#troubleshoot-gateway-errors) ne richiedono una.

<h3 id="send-additional-headers">
  Invia intestazioni aggiuntive
</h3>

Alcuni gateway instradano o taggiano le richieste usando un'intestazione personalizzata oltre alla credenziale, ad esempio un identificatore di tenant o una chiave di instradamento. Per inviarne una, imposta [`ANTHROPIC_CUSTOM_HEADERS`](/docs/it/env-vars) con una coppia `Name: Value` per riga. L'esempio di seguito aggiunge un'intestazione di instradamento denominata `X-Org-Route`:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

Puoi anche impostare `ANTHROPIC_CUSTOM_HEADERS` nel blocco `env` di un file di impostazioni. Usa `\n` tra le coppie lì, poiché le stringhe JSON non possono estendersi su più righe:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

<h3 id="add-gateway-models-to-the-model-picker">
  Aggiungi modelli del gateway al selettore di modelli
</h3>

La scoperta di modelli interroga il gateway per il suo elenco di modelli all'avvio e aggiunge quei nomi al selettore `/model` insieme alle voci integrate.

Abilitala se il tuo gateway serve nomi di modelli che non sono nell'elenco integrato di Claude Code e vuoi selezionarli dal selettore. Se i modelli integrati sono quelli che usi, non hai bisogno della scoperta; il tuo amministratore potrebbe anche averla già abilitata attraverso le impostazioni gestite.

Per abilitarla, imposta `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` nella tua shell o nel blocco `env` di `~/.claude/settings.json`. La scoperta richiede Claude Code v2.1.129 o successivo.&#x20;

I modelli scoperti appaiono come voci `/model` aggiuntive etichettate `From gateway`. Per confermare che la scoperta è stata eseguita, avvia `claude --debug` e cerca le righe `[gatewayDiscovery]`: un successo registra quanti modelli sono stati memorizzati nella cache, e un `404`, timeout, o reindirizzamento viene registrato lì anche. Per quando la scoperta viene eseguita, cosa filtra, e il formato di risposta che i gateway servono, vedi il [riferimento della scoperta di modelli](/docs/it/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Ruota le credenziali con apiKeyHelper
</h3>

Un `apiKeyHelper` è un comando che Claude Code esegue per recuperare la tua credenziale del gateway, invece di leggerla da una variabile di ambiente statica.

Usa un helper quando la credenziale scade secondo una pianificazione, proviene da un comando vault o SSO, o il tuo amministratore ti ha detto di configurarne uno. Se la tua credenziale è una stringa fissa che imposti una volta, la [variabile di credenziale](#set-the-credential-variable) è tutto ciò di cui hai bisogno e puoi saltare questa sezione.

L'helper è qualsiasi comando di shell che stampa la credenziale corrente su stdout. Claude Code lo esegue attraverso la tua shell di sistema, quindi su Windows può essere un eseguibile o un'invocazione di PowerShell. Scrivi lo script, rendilo eseguibile, e referenzialo da `apiKeyHelper` nel tuo [file di impostazioni](/docs/it/settings):

<Tabs>
  <Tab title="Bash o Zsh">
    Ad esempio, uno script che legge da un vault:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Referenzia il suo percorso in `~/.claude/settings.json`:

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    Ad esempio, uno script che legge da un vault:

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Referenzia l'invocazione di PowerShell in `%USERPROFILE%\.claude\settings.json`, sfuggendo i backslash nella stringa JSON:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code memorizza nella cache l'output dell'helper per cinque minuti per impostazione predefinita e lo ri-esegue quando una richiesta restituisce HTTP 401. Per cambiare la durata della cache, imposta `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` in millisecondi, ad esempio `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` per 15 minuti.

Il valore dell'helper viene inviato sia nell'intestazione `Authorization` che in `x-api-key`, quindi funziona qualunque intestazione il tuo gateway legga.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Spegni il traffico al di fuori del percorso del gateway
</h3>

Il gateway trasporta le richieste di modello, ma Claude Code invia anche traffico di background non essenziale al di fuori del percorso del gateway, ad Anthropic e a servizi di terze parti come GitHub: controlli di versione, telemetria, rapporti di errore, note di rilascio e richieste simili. Su una rete che consente solo l'uscita verso il gateway, queste richieste falliscono e possono apparire come connessioni bloccate nel tuo monitoraggio dell'uscita.

Per spegnere quel traffico, imposta `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` insieme alle variabili del gateway, nello stesso blocco di esportazioni di shell o `env` del file di impostazioni:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

L'impostazione della variabile ha questi effetti e limitazioni:

* Disabilita gli aggiornamenti automatici, quindi pianifica un altro percorso di aggiornamento, come il tuo gestore di pacchetti o la distribuzione gestita.
* Sopprime il controllo di disponibilità della [modalità veloce](/docs/it/fast-mode). A meno che un controllo precedente non abbia già abilitato la modalità veloce sulla macchina, `/fast` segnala che la modalità veloce non è disponibile.
* Spegne la [scoperta del modello del gateway](#add-gateway-models-to-the-model-picker), anche se la scoperta interroga il gateway stesso. I modelli precedentemente scoperti rimangono disponibili dalla cache locale, ma l'elenco non viene aggiornato.
* Il controllo di sicurezza del dominio dello strumento WebFetch non è interessato e continua a chiamare `api.anthropic.com`. Spegnilo separatamente con `skipWebFetchPreflight: true` nelle [impostazioni](/docs/it/settings) se la tua rete blocca quell'host.
* Per ogni flusso di telemetria e la variabile che lo controlla, vedi [servizi di telemetria](/docs/it/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Instrada a un provider cloud attraverso un gateway
</h3>

Queste configurazioni indirizzano Claude Code a un gateway attraverso una variabile di URL di base specifica del provider al posto di `ANTHROPIC_BASE_URL`. I gateway Amazon Bedrock e Google Cloud's Agent Platform accettano i formati di richiesta nativi di quei provider; i gateway Microsoft Foundry e Claude Platform su AWS accettano il formato Anthropic Messages e differiscono solo in quale variabile di URL di base li raggiunge.

Usane una solo se il tuo team del gateway ha specificamente nominato Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform su AWS. Se la [richiesta di verifica](#verify-the-connection) di cui sopra ha restituito JSON, puoi saltare questa sezione.

Imposta il blocco per il provider che il tuo team del gateway ha nominato. Le variabili skip-auth dicono a Claude Code di non firmare le richieste con le credenziali del provider, poiché il gateway le detiene. Se il gateway ha bisogno del suo proprio token, aggiungi `ANTHROPIC_AUTH_TOKEN` dopo il blocco, tranne per Microsoft Foundry, che usa `ANTHROPIC_FOUNDRY_API_KEY` come mostrato. Un gateway Microsoft Foundry che si aspetta un bearer token può usare [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/it/env-vars) invece; ha la precedenza su `ANTHROPIC_FOUNDRY_API_KEY` quando entrambi sono impostati. `ANTHROPIC_FOUNDRY_AUTH_TOKEN` richiede Claude Code v2.1.203 o successivo.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h4>

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Metti la credenziale del gateway in `ANTHROPIC_FOUNDRY_API_KEY`; viene inviata al gateway come intestazione `x-api-key`. Un gateway che si aspetta un bearer token può prendere [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/it/env-vars) invece. Claude Code invia quel valore come intestazione `Authorization: Bearer`, e ha la precedenza su `ANTHROPIC_FOUNDRY_API_KEY` quando entrambi sono impostati. Richiede Claude Code v2.1.203 o successivo.

Per un gateway che inietta il suo proprio intestazione `Authorization`, imposta `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` e lascia entrambe le variabili di credenziale non impostate. Claude Code quindi invia richieste senza una credenziale Azure e preserva l'intestazione `Authorization` che fornisci, ad esempio attraverso `ANTHROPIC_CUSTOM_HEADERS`. Prima di v2.1.203, `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` senza una chiave API ha lasciato il client Microsoft Foundry incapace di inviare richieste.

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  Claude Platform su AWS
</h4>

Vedi [Claude Platform su AWS](/docs/it/claude-platform-on-aws) per l'ID dell'area di lavoro.

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h2 id="troubleshoot-gateway-errors">
  Risolvi gli errori del gateway
</h2>

Questi sono gli errori più comuni quando si esegue Claude Code attraverso un gateway, con la causa dal lato del gateway e la correzione:

| Errore                                                                                                                                                                                                                | Causa                                                                                                                                                                                                                                                                                                                                      | Correzione                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Un avviso di avvio che nomina due fonti di credenziale e termina con `auth may not work as expected`. Le versioni più vecchie mostrano `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` invece. | Una credenziale del gateway e un accesso salvato sono entrambi attivi; la variabile viene utilizzata per le richieste, ma l'accesso stantio può causare un comportamento di autenticazione inaspettato                                                                                                                                     | Annulla l'impostazione della variabile per usare l'accesso salvato, o esegui `/logout` per usare la credenziale del gateway                                                                                                                                                                                                                                                                                                                |
| Errori `401` che nominano un token non valido o non riconosciuto                                                                                                                                                      | La credenziale non è una che il gateway ha emesso, o è in un'intestazione che il gateway non legge                                                                                                                                                                                                                                         | Conferma che la variabile corrisponde al tuo tipo di credenziale nella [tabella di credenziale](#set-the-credential-variable), e rigenera la chiave al gateway se è stata revocata                                                                                                                                                                                                                                                         |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                 | Il comando nell'impostazione [`apiKeyHelper`](/docs/it/settings#available-settings) è uscito con un errore, è scaduto, o non ha stampato nulla, quindi le richieste portano una chiave segnaposto                                                                                                                                               | Esegui il comando direttamente per vedere perché fallisce, e ri-autentica con il tuo provider di credenziali se segnala una sessione scaduta; vedi [il riferimento dell'errore](/docs/it/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                                                            |
| `Unable to connect to API (ConnectionRefused)`, o `(ECONNREFUSED)` da installazioni npm, spesso dopo una pausa silenziosa mentre Claude Code [riprova con backoff](/docs/it/errors#automatic-retries)                      | Nulla ha risposto all'URL di base: l'indirizzo è sbagliato, o una VPN o firewall blocca il percorso al gateway                                                                                                                                                                                                                             | Esegui il [test curl di cui sopra](#verify-the-connection), che fallisce immediatamente con la stessa causa, e conferma l'URL e il percorso di rete con il tuo team del gateway                                                                                                                                                                                                                                                            |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                              | Il gateway o un proxy intermedio ha restituito una risposta non-API, spesso una pagina di errore HTML o di accesso                                                                                                                                                                                                                         | Testa con la [richiesta curl di cui sopra](#verify-the-connection); correggi il percorso del gateway che restituisce non-JSON                                                                                                                                                                                                                                                                                                              |
| Errori `400` che nominano `context_management`, `Extra inputs are not permitted`, o altri campi non riconosciuti                                                                                                      | Il gateway invia le richieste a un upstream che rifiuta i campi che Claude Code invia agli endpoint in formato Anthropic                                                                                                                                                                                                                   | Imposta `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`, che sopprime la maggior parte dei campi pre-release; vedi [feature pass-through](/docs/it/llm-gateway-protocol#feature-pass-through). Alcuni beta non sono controllati da questo flag; per quelli, imposta la variabile provider `CLAUDE_CODE_USE_*` corrispondente in modo che Claude Code invii solo quello che quel provider accetta                                                     |
| Errori `400` che nominano `thinking` o `adaptive`, come `Input tag 'adaptive' found`                                                                                                                                  | La build del modello upstream non accetta il ragionamento adattivo, che Claude Code richiede per i modelli Claude 4.6 e successivi                                                                                                                                                                                                         | Aggiorna l'upstream del gateway. Su Opus 4.6 e Sonnet 4.6, `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` funziona invece. Le variabili di capacità della [configurazione del modello](/docs/it/model-config) si applicano solo alle configurazioni del provider, come `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, non dietro un gateway `ANTHROPIC_BASE_URL`                                                                              |
| Errori `400` che indicano un contesto o limite di token nelle parole del gateway stesso, come `ContextWindowExceededError` o `prompt token count of N exceeds the limit of M`                                         | Il gateway applica un contesto più piccolo della finestra nativa del modello e riscrive l'errore upstream, quindi il compact-and-retry automatico, che corrisponde alla dicitura `prompt is too long` di Anthropic, non si attiva                                                                                                          | Esegui `/compact` per recuperare la sessione. Per prevenirlo, imposta `CLAUDE_CODE_AUTO_COMPACT_WINDOW` al limite del gateway; il valore è bloccato ad almeno 100.000 token e al massimo la finestra di contesto del modello, quindi un limite del gateway inferiore a 100.000 non può essere abbinato e `/compact` rimane il recupero lì. Imposta anche `CLAUDE_CODE_MAX_OUTPUT_TOKENS` sotto il limite di output del modello del gateway |
| Modelli mancanti dal selettore `/model`                                                                                                                                                                               | I nomi dei modelli del gateway non sono nell'elenco integrato di Claude Code                                                                                                                                                                                                                                                               | Abilita la [scoperta di modelli del gateway](#add-gateway-models-to-the-model-picker) o aggiungi nomi con le variabili della [configurazione del modello](/docs/it/model-config)                                                                                                                                                                                                                                                                |
| Claude Code ti chiede di accedere anche se il [test curl](#verify-the-connection) ha successo                                                                                                                         | La CLI non ha una credenziale propria: un URL di base raggiungibile non è uno, e un blocco `env` nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto si applica solo dopo la procedura guidata di primo avvio e il prompt di fiducia                                                                                | Imposta `ANTHROPIC_AUTH_TOKEN` da qualche parte Claude Code legge prima della configurazione di primo avvio: un'esportazione di shell, il blocco `env` in `~/.claude/settings.json`, o impostazioni gestite                                                                                                                                                                                                                                |
| `ANTHROPIC_API_KEY` è impostato ma ignorato, senza prompt                                                                                                                                                             | La chiave ha bisogno di un'approvazione una tantum nelle sessioni interattive, e una chiave precedentemente rifiutata viene ignorata senza chiedere di nuovo                                                                                                                                                                               | Abilitala sotto `/config` con l'opzione `Use custom API key`                                                                                                                                                                                                                                                                                                                                                                               |
| `This machine's managed settings require a first-party login`                                                                                                                                                         | Le impostazioni gestite includono `forceLoginMethod` o `forceLoginOrgUUID`, che su Claude Code v2.1.146 e successivo non possono coesistere con `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper`                                                                                                                              | Il tuo amministratore deve rimuovere `forceLoginMethod` e `forceLoginOrgUUID` dalle impostazioni gestite per usare le credenziali del gateway, o rimuovere la credenziale del gateway per usare l'accesso di prima parte. I due non possono essere combinati                                                                                                                                                                               |
| `403` con un corpo HTML come `403 Forbidden`, quando i log del gateway stesso non mostrano alcuna richiesta ricevuta                                                                                                  | Un web application firewall o reverse proxy davanti al gateway ha bloccato il corpo della richiesta prima che raggiungesse il gateway. I prompt di Claude Code includono tag di stile XML e codice sorgente che corrispondono alle regole del corpo dello scripting tra siti, quindi un breve test curl passa mentre una sessione reale no | Esenta il percorso `/v1/messages` del gateway dall'ispezione del corpo della richiesta. Su AWS WAF questa è la regola gestita `CrossSiteScripting_Body`; su nginx con ModSecurity è la regola del corpo OWASP CRS equivalente                                                                                                                                                                                                              |
| Errori di certificato o TLS come `SSL certificate verification failed` o `Self-signed certificate detected`, quando il [test curl](#verify-the-connection) ha successo                                                | Il runtime di Claude Code non sta fidando della stessa autorità di certificazione che `curl` usa. Comune dietro proxy di ispezione TLS aziendale                                                                                                                                                                                           | Imposta `NODE_EXTRA_CA_CERTS` al percorso del bundle CA; vedi [archivio di certificati CA](/docs/it/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                        |

Se Claude Code ti chiede di accedere ripetutamente dopo aver rimosso la configurazione del gateway, la causa è solitamente l'archiviazione delle credenziali piuttosto che il gateway; vedi [errori di autenticazione](/docs/it/errors#authentication-errors).

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Panoramica dei gateway LLM](/docs/it/llm-gateway): cos'è un gateway e come interagisce con gli abbonamenti a claude.ai
* [Distribuisci un gateway LLM per la tua organizzazione](/docs/it/llm-gateway-rollout): la checklist rivolta agli amministratori per distribuire e distribuire la configurazione del gateway
* [Riferimento del protocollo gateway](/docs/it/llm-gateway-protocol): cosa Claude Code invia a un gateway, incluse le intestazioni e i campi che il gateway deve inoltrar
* [Impostazioni](/docs/it/settings): dove si trovano i file di impostazioni e come viene letto il blocco `env`
* [Autenticazione](/docs/it/authentication): come le variabili di credenziale, `apiKeyHelper`, e l'accesso OAuth interagiscono
