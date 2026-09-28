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

Gli amministratori possono distribuire l'indirizzo del gateway e la credenziale attraverso [impostazioni gestite](/docs/it/managed-settings), gestione dei dispositivi, o un [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper), in modo che Claude Code le raccolga all'avvio senza che Lei debba impostare nulla. Per verificare se la Sua organizzazione lo ha già fatto:

<Steps>
  <Step title="Avvia Claude Code">
    Esegui `claude`. Se si apre alla schermata di accesso invece di una sessione, nessuna credenziale del gateway è stata distribuita; [configurala da solo](#configure-claude-code-yourself) di seguito.
  </Step>

  <Step title="Controlla la scheda Status">
    Se Claude Code ha avviato una sessione senza mostrare la schermata di accesso, esegui `/status`, che si apre sulla scheda **Status**, e controlla due righe:

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

Le esportazioni di shell si applicano solo a quella sessione di terminale e ai programmi avviati da essa. Un editor lanciato dal dock o dal menu Start non le vedrà. Per farle persistere tra i nuovi terminali, aggiungi le stesse righe al tuo profilo di shell, come `~/.zshrc`, `~/.bashrc`, o il tuo `$PROFILE` di PowerShell.

Se esporti il gateway solo nella tua shell, non raggiunge in modo affidabile gli agenti di background ospitati dal [supervisor](/docs/it/agent-view#how-background-sessions-are-hosted); vedi [come ogni sessione di background recupera il suo gateway](/docs/it/agent-view#llm-gateway). Usa un file di impostazioni per qualsiasi gateway che gli agenti di background devono sempre instradare attraverso.

<h4 id="set-in-a-settings-file">
  Imposta in un file di impostazioni
</h4>

Per fare in modo che la configurazione si applichi ovunque Claude Code viene eseguito, inclusi gli [agenti di background](/docs/it/agent-view#how-background-sessions-are-hosted), imposta le variabili nel blocco `env` di un [file di impostazioni](/docs/it/settings) invece di dipendere dalla tua shell. I file di impostazioni hanno ambiti diversi:

* `~/.claude/settings.json` si applica a tutti i tuoi progetti. Su Windows il percorso è `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` si applica a un progetto. Claude Code lo aggiunge al tuo gitignore globale quando salva un'impostazione lì; se lo crei a mano o fai scrivere a Claude, aggiungilo al tuo gitignore tu stesso prima in modo da non committere accidentalmente la tua credenziale

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

<h3 id="slack-cloud-sessions-and-remote-control">
  Slack, sessioni cloud e Remote Control
</h3>

[Claude Code in Slack](/docs/it/slack) e [sessioni cloud](/docs/it/claude-code-on-the-web) usano sempre l'API di Anthropic; non fanno parte di una distribuzione del gateway. Le variabili del gateway impostate nella configurazione dell'ambiente di una sessione cloud non vengono applicate. Se il tuo traffico deve rimanere sul gateway, non abilitare queste superfici per quegli utenti.

[Remote Control](/docs/it/remote-control) e [dettatura vocale](/docs/it/voice-dictation) si basano entrambi su un'identità claude.ai: Remote Control per accoppiare una sessione live con il tuo account, e dettatura vocale per raggiungere l'endpoint di trascrizione claude.ai. Non sono disponibili mentre `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o un `apiKeyHelper` è attivo. Remote Control è anche disabilitato mentre `ANTHROPIC_BASE_URL` punta a un host non-Anthropic, quindi accedere con claude.ai non è sufficiente da solo. Prima della v2.1.196, un URL di base non-Anthropic non bloccava Remote Control.

Per ripristinare una delle due funzioni, accedi con claude.ai e annulla l'impostazione delle variabili del gateway che quella funzione controlla. La sezione Remote Control di `claude doctor` nomina ciò che sta attualmente bloccando Remote Control.

* Dettatura vocale: annulla l'impostazione della credenziale del gateway
* Remote Control: annulla l'impostazione della credenziale del gateway e `ANTHROPIC_BASE_URL`

<h2 id="additional-configuration">
  Configurazione aggiuntiva
</h2>

Queste impostazioni coprono i casi oltre l'URL di base e le credenziali. Impostarle solo se le istruzioni dell'amministratore, le regole di uscita della rete o la [tabella di risoluzione dei problemi](#troubleshoot-gateway-errors) lo richiedono.

<h3 id="send-additional-headers">
  Inviare intestazioni aggiuntive
</h3>

Alcuni gateway instradano o taggano le richieste utilizzando un'intestazione personalizzata oltre alle credenziali, ad esempio un identificatore di tenant o una chiave di instradamento. Per inviarne una, impostare [`ANTHROPIC_CUSTOM_HEADERS`](/docs/it/env-vars) con una coppia `Name: Value` per riga. L'esempio seguente aggiunge un'intestazione di instradamento denominata `X-Org-Route`:

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

È possibile impostare anche `ANTHROPIC_CUSTOM_HEADERS` nel blocco `env` di un file di impostazioni. Utilizzare `\n` tra le coppie lì, poiché le stringhe JSON non possono estendersi su più righe:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

I nomi delle intestazioni di instradamento e tenant come questi contano come [intestazioni che richiedono approvazione](/docs/it/server-managed-settings#environment-variables-and-the-approval-dialog). Quando le intestazioni provengono da un file di impostazioni del progetto, Claude Code le applica secondo le [regole per quando applica i valori `env`](/docs/it/settings-reference#when-claude-code-applies-env-values).

<h3 id="add-gateway-models-to-the-model-picker">
  Aggiungere modelli gateway al selettore di modelli
</h3>

Con il rilevamento dei modelli abilitato, Claude Code interroga il gateway per il suo elenco di modelli all'avvio e aggiunge questi nomi al selettore `/model` insieme alle voci integrate. Se l'utente o l'amministratore impostano `replaceBuiltInOptions` in una lineup [`modelPicker`](/docs/it/settings-reference#modelpicker), Claude Code nasconde anche i nomi rilevati. Mantiene una riga per il modello che la sessione sta già utilizzando.

Abilitarlo se il gateway fornisce nomi di modelli che non sono nell'elenco integrato di Claude Code e si desidera selezionarli dal selettore. Se i modelli integrati sono quelli utilizzati, non è necessario il rilevamento; l'amministratore potrebbe averlo già abilitato tramite impostazioni gestite.

Per abilitarlo, impostare `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` nella shell o nel blocco `env` di `~/.claude/settings.json`.

I modelli rilevati vengono visualizzati come voci `/model` aggiuntive. Ogni voce mostra la descrizione fornita dal gateway per il modello, o `From gateway` quando non ne fornisce una.

Per confermare che il rilevamento è stato eseguito, avviare `claude --debug` e cercare le righe `[gatewayDiscovery]` nel registro di debug in `~/.claude/debug/<session-id>.txt`. La prima volta che il rilevamento ha successo, Claude Code registra quanti modelli ha memorizzato nella cache, e registra di nuovo solo quando l'elenco del gateway cambia. Un `404`, timeout o reindirizzamento appare lì anche. Per quando viene eseguito il rilevamento, cosa filtra e il formato di risposta che i gateway forniscono, vedere il [riferimento del rilevamento dei modelli](/docs/it/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Ruotare le credenziali con apiKeyHelper
</h3>

Un `apiKeyHelper` è un comando che Claude Code esegue per recuperare la credenziale del gateway, invece di leggerla da una variabile di ambiente statica.

Utilizzare un helper quando la credenziale scade secondo una pianificazione, proviene da un comando vault o SSO, o l'amministratore ha detto di configurarne uno. Se la credenziale è una stringa fissa impostata una volta, la [variabile di credenziale](#set-the-credential-variable) è tutto ciò che serve e è possibile saltare questa sezione.

L'helper è qualsiasi comando shell che stampa la credenziale corrente su stdout. Claude Code lo esegue attraverso la shell di sistema, quindi su Windows può essere un eseguibile o una chiamata PowerShell. Fare in modo che il comando stampi solo la credenziale. Su Claude Code v2.1.227 o successivo, un banner o una riga di log stampata insieme alla chiave fa [fallire l'helper](/docs/it/errors#your-apikeyhelper-script-is-failing). Scrivere lo script, renderlo eseguibile e farvi riferimento da `apiKeyHelper` nel [file di impostazioni](/docs/it/settings):

<Tabs>
  <Tab title="Bash o Zsh">
    Ad esempio, uno script che legge da un vault:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Farvi riferimento nel percorso in `~/.claude/settings.json`:

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

    Farvi riferimento dall'invocazione PowerShell in `%USERPROFILE%\.claude\settings.json`, sfuggendo ai backslash nella stringa JSON:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code memorizza nella cache l'output dell'helper per cinque minuti per impostazione predefinita e riesegue l'helper dopo che la durata della cache scade. Per modificare la durata, impostare `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` in millisecondi, ad esempio `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` per 15 minuti.

Vedere [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) per gli altri casi in cui Claude Code riesegue l'helper.

Il valore dell'helper viene inviato sia nell'intestazione `Authorization` che in `x-api-key`, quindi funziona indipendentemente da quale intestazione il gateway legge.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Disattivare il traffico al di fuori del percorso del gateway
</h3>

Il gateway trasporta le richieste di modello, ma Claude Code invia anche traffico di background non essenziale al di fuori del percorso del gateway, ad Anthropic e a servizi di terze parti come GitHub: controlli di versione, telemetria, note di rilascio e richieste simili. Su una rete che consente solo l'uscita verso il gateway, queste richieste falliscono e possono apparire come connessioni bloccate nel monitoraggio dell'uscita.

Claude Code allega una credenziale a una richiesta di telemetria o metriche di utilizzo solo quando la richiesta va all'host a cui appartiene la credenziale. Mentre `ANTHROPIC_BASE_URL` punta al gateway, Claude Code invia i suoi eventi di telemetria ad Anthropic senza la credenziale del gateway. Con una [variabile di credenziale](#set-the-credential-variable) o `apiKeyHelper` anche attivi, Claude Code non segnala le metriche di utilizzo alla dashboard [analytics](/docs/it/analytics#access-analytics-for-api-customers) della Console. Prima della v2.1.246, Claude Code poteva allegare la credenziale del gateway alle richieste di telemetria e metriche di utilizzo destinate agli host Anthropic; le richieste di modello andavano sempre al gateway con la credenziale che il gateway si aspetta.

Per disattivare quel traffico, impostare `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` insieme alle variabili del gateway, nello stesso blocco di esportazioni shell o `env` del file di impostazioni:

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

L'impostazione della variabile ha questi effetti e limiti:

* Disabilita gli aggiornamenti automatici, quindi pianificare un altro percorso di aggiornamento, come il gestore di pacchetti o la distribuzione gestita.
* Sopprime il controllo di disponibilità della [modalità veloce](/docs/it/fast-mode). A meno che un controllo precedente non abbia già abilitato la modalità veloce sulla macchina, `/fast` segnala che la modalità veloce non è disponibile.
* Non influisce sul [rilevamento dei modelli gateway](#add-gateway-models-to-the-model-picker), che interroga solo il gateway. Prima della v2.1.257, la variabile fermava anche l'aggiornamento del rilevamento, quindi il selettore manteneva l'elenco precedentemente memorizzato nella cache.
* Il controllo di sicurezza del dominio dello strumento WebFetch]\(/it/data-usage#webfetch-domain-safety-check) non è interessato e chiama ancora `api.anthropic.com`. Disattivarlo separatamente con `skipWebFetchPreflight: true` nelle [impostazioni](/docs/it/settings) se la rete blocca quell'host.
* Per ogni flusso di telemetria e la variabile che lo controlla, vedere [servizi di telemetria](/docs/it/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Instradare a un provider cloud attraverso un gateway
</h3>

Queste configurazioni puntano Claude Code a un gateway attraverso una variabile di URL di base specifica del provider al posto di `ANTHROPIC_BASE_URL`. I gateway Amazon Bedrock e Google Cloud's Agent Platform accettano i formati di richiesta nativi di questi provider; i gateway Microsoft Foundry e Claude Platform on AWS accettano il formato Anthropic Messages. Sugli instradamenti Amazon Bedrock e Google Cloud's Agent Platform, Claude Code limita anche le intestazioni beta e i campi di richiesta che invia al set che il provider accetta. Per ciò che il gateway riceve su ogni instradamento, vedere la [guida di compatibilità del gateway](/docs/it/llm-gateway-protocol).

Utilizzarne uno solo se il team del gateway ha nominato specificamente Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o Claude Platform on AWS. Se la [richiesta di verifica](#verify-the-connection) sopra ha restituito JSON, è possibile saltare questa sezione.

Impostare il blocco per il provider che il team del gateway ha nominato. Le variabili skip-auth nei blocchi Amazon Bedrock, Google Cloud's Agent Platform e Claude Platform on AWS dicono a Claude Code di non firmare le richieste con le credenziali del provider cloud, poiché il gateway le contiene. Se il gateway ha anche bisogno del suo token, dove lo si mette dipende dal provider:

* **Amazon Bedrock, Google Cloud's Agent Platform o Claude Platform on AWS**: aggiungere `ANTHROPIC_AUTH_TOKEN` dopo il blocco. Claude Code lo invia al gateway come intestazione `Authorization: Bearer`. Per una credenziale in uno schema o intestazione diversa, utilizzare [`ANTHROPIC_CUSTOM_HEADERS`](#send-additional-headers) invece. Mantenere la variabile skip-auth impostata comunque, poiché senza di essa Claude Code rimuove qualsiasi intestazione `Authorization` che `ANTHROPIC_AUTH_TOKEN`, un [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) o `ANTHROPIC_CUSTOM_HEADERS` aggiungerebbero.
* **Microsoft Foundry**: utilizzare `ANTHROPIC_FOUNDRY_API_KEY` come mostra il [suo blocco](#microsoft-foundry)

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Lasciare `AWS_BEARER_TOKEN_BEDROCK` non impostato quando il gateway emette la sua credenziale. Se lo si imposta, Claude Code invia quella [chiave API Amazon Bedrock](/docs/it/amazon-bedrock#2-configure-aws-credentials) come intestazione `Authorization` invece del token del gateway, anche con `CLAUDE_CODE_SKIP_BEDROCK_AUTH` impostato.

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

Sostituire l'ID del progetto e la regione con i propri valori. Claude Code include entrambi nel percorso di ogni richiesta che invia al gateway:

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

Il blocco copre l'instradamento e l'autenticazione. Gli override di regione e i pin di modello dalla [configurazione di Agent Platform](/docs/it/google-vertex-ai#4-configure-claude-code) si applicano anche attraverso un gateway:

* **Regioni per modello**: se il gateway fornisce alcuni modelli da una regione diversa da `CLOUD_ML_REGION`, impostare la variabile `VERTEX_REGION_CLAUDE_*` corrispondente per ciascuno, ad esempio `VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1`. Il [riferimento delle variabili di ambiente](/docs/it/env-vars) elenca i nomi esatti.
* **Versioni dei modelli**: pin `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` e `ANTHROPIC_DEFAULT_HAIKU_MODEL` come in [Pin model versions](/docs/it/google-vertex-ai#5-pin-model-versions). L'impostazione di `ANTHROPIC_DEFAULT_HAIKU_MODEL` sposta anche attività di background come i titoli delle sessioni a quel modello, e quella sezione spiega quale modello le esegue altrimenti.
* **Capacità dei modelli**: se si pin un ID modello che la versione di Claude Code non riconosce, funzionalità come livelli di sforzo o pensiero esteso possono rimanere disabilitate su di esso. Dichiarare ciò che il modello supporta con [`ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES`](/docs/it/model-config#customize-pinned-model-display-and-capabilities) e i suoi equivalenti Sonnet e Haiku.

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Mettere la credenziale del gateway in `ANTHROPIC_FOUNDRY_API_KEY`; viene inviata al gateway come intestazione `x-api-key`. Un gateway che si aspetta un bearer token può prendere [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/it/env-vars) invece. Claude Code invia quel valore come intestazione `Authorization: Bearer`, e ha la precedenza su `ANTHROPIC_FOUNDRY_API_KEY` quando entrambi sono impostati. Richiede Claude Code v2.1.203 o successivo.

Per un gateway che inietta la sua intestazione `Authorization`, impostare `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` e lasciare entrambe le variabili di credenziale non impostate. Claude Code invia quindi richieste senza una credenziale Azure e preserva l'intestazione `Authorization` fornita, ad esempio attraverso `ANTHROPIC_CUSTOM_HEADERS`. Prima della v2.1.203, `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` senza una chiave API lasciava il client Microsoft Foundry incapace di inviare richieste.

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
  Claude Platform on AWS
</h4>

Vedere [Claude Platform on AWS](/docs/it/claude-platform-on-aws) per l'ID dell'area di lavoro.

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

<h4 id="confirm-the-provider-route">
  Confermare l'instradamento del provider
</h4>

Avviare `claude` dalla shell dove è stato impostato il blocco ed eseguire `/status`. Con il blocco Amazon Bedrock, la scheda **Status** mostra righe come queste:

```text theme={null}
API provider: Amazon Bedrock
Bedrock base URL: https://llm-gateway.example.com/bedrock
AWS auth skipped
```

Gli altri blocchi producono le stesse righe sotto i nomi del provider, ad esempio `Vertex base URL` e `GCP auth skipped` per Google Cloud's Agent Platform; il blocco Microsoft Foundry mostra una riga auth skipped solo se si imposta `CLAUDE_CODE_SKIP_FOUNDRY_AUTH`. Se si instrada anche attraverso un proxy aziendale, una riga `Proxy` mostra l'URL del proxy. Se la riga dell'URL di base è mancante, la variabile non ha raggiunto la sessione.

<h2 id="troubleshoot-gateway-errors">
  Risolvi gli errori del gateway
</h2>

Questi sono gli errori più comuni quando si esegue Claude Code attraverso un gateway, con la causa dal lato del gateway e la correzione:

| Errore                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Causa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Correzione                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Un avviso di avvio che nomina due fonti di credenziale e termina con `auth may not work as expected`. Le versioni più vecchie mostrano `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` invece.                                                                                                                                                                                                                                                                  | Una credenziale del gateway e un accesso salvato sono entrambi attivi; la variabile viene utilizzata per le richieste, ma l'accesso stantio può causare un comportamento di autenticazione inaspettato                                                                                                                                                                                                                                                                                       | Annulla l'impostazione della variabile per usare l'accesso salvato, o esegui `/logout` per usare la credenziale del gateway                                                                                                                                                                                                                                                                                                                   |
| Errori `401` che nominano un token non valido o non riconosciuto                                                                                                                                                                                                                                                                                                                                                                                                                       | La credenziale non è una che il gateway ha emesso, o è in un'intestazione che il gateway non legge                                                                                                                                                                                                                                                                                                                                                                                           | Conferma che la variabile corrisponde al tuo tipo di credenziale nella [tabella di credenziale](#set-the-credential-variable), e rigenera la chiave al gateway se è stata revocata                                                                                                                                                                                                                                                            |
| `Your apiKeyHelper script is failing`, o `apiKeyHelper failed:` su stderr in modalità non interattiva                                                                                                                                                                                                                                                                                                                                                                                  | Il comando nell'impostazione [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) non ha prodotto una chiave utilizzabile, quindi le richieste portano una chiave segnaposto                                                                                                                                                                                                                                                                                                                | Esegui il comando direttamente per vedere perché fallisce, e ri-autentica con il tuo provider di credenziali se segnala una sessione scaduta; vedi [il riferimento dell'errore](/docs/it/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                                                               |
| `Connection refused — a firewall or proxy may be blocking it (ConnectionRefused)` quando nulla risponde all'indirizzo, o `Can't reach the API server — check your internet or DNS (ENOTFOUND)` quando il nome host non si risolve, spesso dopo una pausa silenziosa mentre Claude Code [riprova con backoff](/docs/it/errors#automatic-retries). Il codice tra parentesi varia; [Unable to connect to API](/docs/it/errors#unable-to-connect-to-api) copre i codici e la formulazione precedente | Nulla ha risposto all'URL di base: l'indirizzo è sbagliato, o una VPN o firewall blocca il percorso al gateway                                                                                                                                                                                                                                                                                                                                                                               | Esegui il [test curl di cui sopra](#verify-the-connection), che fallisce immediatamente con la stessa causa, e conferma l'URL e il percorso di rete con il tuo team del gateway                                                                                                                                                                                                                                                               |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                                                                                                                                                                                                                                                                                               | Il gateway o un proxy intermedio ha restituito una risposta non-API, spesso una pagina di errore HTML o di accesso                                                                                                                                                                                                                                                                                                                                                                           | Testa con la [richiesta curl di cui sopra](#verify-the-connection); correggi il percorso del gateway che restituisce qualcosa di diverso da una risposta API Claude. [Il riferimento dell'errore](/docs/it/errors#api-returned-an-empty-or-malformed-response) spiega il dettaglio che il messaggio segnala                                                                                                                                        |
| Errori `400` che nominano `context_management`, `Extra inputs are not permitted`, o altri campi non riconosciuti                                                                                                                                                                                                                                                                                                                                                                       | Il gateway invia le richieste a un upstream che rifiuta i campi che Claude Code invia agli endpoint in formato Anthropic                                                                                                                                                                                                                                                                                                                                                                     | Imposta `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`, che sopprime la maggior parte dei campi pre-release; vedi [feature pass-through](/docs/it/llm-gateway-protocol#feature-pass-through). Alcuni beta non sono controllati da questo flag; per quelli, imposta la variabile provider `CLAUDE_CODE_USE_*` corrispondente in modo che Claude Code invii solo quello che quel provider accetta                                                        |
| Errori `400` che nominano `thinking` o `adaptive`, come `Input tag 'adaptive' found`                                                                                                                                                                                                                                                                                                                                                                                                   | La build del modello upstream non accetta il ragionamento adattivo, che Claude Code richiede per i modelli Claude 4.6 e successivi                                                                                                                                                                                                                                                                                                                                                           | Aggiorna l'upstream del gateway. Su Opus 4.6 e Sonnet 4.6, `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` funziona invece. Le variabili di capacità della [configurazione del modello](/docs/it/model-config) si applicano solo alle configurazioni del provider, come `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, non dietro un gateway `ANTHROPIC_BASE_URL`                                                                                 |
| Errori `400` che indicano un contesto o limite di token nelle parole del gateway stesso, come `ContextWindowExceededError` o `prompt token count of N exceeds the limit of M`                                                                                                                                                                                                                                                                                                          | Il gateway applica un contesto più piccolo della finestra nativa del modello e riscrive l'errore upstream, quindi Claude Code non lo riconosce come un [errore troppo lungo](/docs/it/errors#prompt-is-too-long) e non compatta e riprova automaticamente                                                                                                                                                                                                                                         | Esegui `/compact` per recuperare la sessione. Per prevenirlo, imposta `CLAUDE_CODE_AUTO_COMPACT_WINDOW` al limite del gateway; Claude Code blocca il valore ad almeno 100.000 token e al massimo la finestra di contesto del modello, quindi non puoi abbinare un limite del gateway inferiore a 100.000, e `/compact` rimane il recupero lì. Imposta anche `CLAUDE_CODE_MAX_OUTPUT_TOKENS` sotto il limite di output del modello del gateway |
| Errori `400` su ogni richiesta, nelle parole del gateway stesso che rifiutano lo schema di input di uno strumento o il suo `pattern`, su Claude Code v2.1.265 attraverso v2.1.267                                                                                                                                                                                                                                                                                                      | In un rollout graduale su quelle versioni, lo schema dello [strumento Artifact](/docs/it/artifacts#availability) contiene un'espressione regolare con classi di caratteri Unicode `\p{...}`. L'API Anthropic l'accetta, ma un gateway o upstream che controlla il `pattern` dello schema di ogni strumento con il proprio motore regex rifiuta l'intera richiesta                                                                                                                                 | Aggiorna a v2.1.268 o successivo, che non invia l'espressione regolare. Su una versione interessata, [disattiva gli artifact](/docs/it/artifacts#disable-artifacts), che rimuove lo strumento e il suo schema dalle richieste                                                                                                                                                                                                                      |
| Errori `400` su ogni richiesta, nelle parole del gateway stesso che rifiutano un tipo di strumento non riconosciuto, come `Input tag 'advisor_20260301'`, su Claude Code v2.1.275                                                                                                                                                                                                                                                                                                      | In un rollout graduale su quella versione, le richieste portano una voce dello [strumento advisor](/docs/it/advisor) anche con l'advisor disattivato. L'API Anthropic l'accetta, ma un gateway o upstream che convalida i tipi di strumento rifiuta l'intera richiesta; uno che [invia i campi del corpo della richiesta invariati](/docs/it/llm-gateway-protocol#forward-as-open-lists) lo passa attraverso senza effetti. La voce è una dichiarazione che non porta alcun contenuto di conversazione | Aggiorna a v2.1.276 o successivo, che non invia la voce dietro un gateway `ANTHROPIC_BASE_URL` a meno che tu non attivi l'advisor. Su v2.1.275, imposta [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`](/docs/it/env-vars), che rimuove la voce dalle richieste                                                                                                                                                                                             |
| Modelli mancanti dal selettore `/model`                                                                                                                                                                                                                                                                                                                                                                                                                                                | I nomi dei modelli del gateway non sono nell'elenco integrato di Claude Code, o Claude Code sta mostrando un [`modelPicker`](/docs/it/settings-reference#modelpicker) che sostituisce le opzioni integrate                                                                                                                                                                                                                                                                                        | Abilita la [scoperta di modelli del gateway](#add-gateway-models-to-the-model-picker) o aggiungi nomi con le variabili della [configurazione del modello](/docs/it/model-config). Se Claude Code mostra un `modelPicker` che sostituisce, aggiungi i modelli del gateway ad esso, o chiedi al tuo amministratore di aggiungerli quando le impostazioni gestite lo forniscono                                                                       |
| `/fast` segnala `Fast mode unavailable due to network connectivity issues` mentre le richieste di inferenza funzionano                                                                                                                                                                                                                                                                                                                                                                 | Il controllo di disponibilità della [modalità veloce](/docs/it/fast-mode) va direttamente a `api.anthropic.com` e non segue `ANTHROPIC_BASE_URL`, quindi l'uscita diretta bloccata fallisce il controllo. Lo stesso messaggio appare su una rete aperta quando il controllo presenta una chiave emessa dal gateway da `ANTHROPIC_API_KEY` o da un `apiKeyHelper` e Anthropic la rifiuta                                                                                                           | Consenti `api.anthropic.com` se l'uscita è bloccata, o imposta una variabile di salto; per una chiave del gateway rifiutata solo le variabili di salto aiutano. Vedi [usa la modalità veloce dietro proxy e gateway LLM](/docs/it/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                                         |
| `/fast` segnala `Fast mode has been disabled by your organization` in una sessione autenticata con `ANTHROPIC_AUTH_TOKEN`, anche se l'organizzazione ha la modalità veloce abilitata                                                                                                                                                                                                                                                                                                   | Il controllo di disponibilità richiede un accesso claude.ai o una chiave API Anthropic; con solo un token bearer, Claude Code tratta la modalità veloce come disabilitata senza inviare il controllo                                                                                                                                                                                                                                                                                         | Imposta `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`; vedi [usa la modalità veloce dietro proxy e gateway LLM](/docs/it/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)                                                                                                                                                                                                                                                                       |
| Claude Code ti chiede di accedere anche se il [test curl](#verify-the-connection) ha successo                                                                                                                                                                                                                                                                                                                                                                                          | La CLI non ha una credenziale propria: un URL di base raggiungibile non è uno, e in una sessione interattiva un blocco `env` nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto si applica solo dopo la procedura guidata di primo avvio e il [prompt di fiducia](/docs/it/permissions#what-runs-before-you-trust-a-folder)                                                                                                                                               | Imposta `ANTHROPIC_AUTH_TOKEN` da qualche parte Claude Code legge prima della configurazione di primo avvio: un'esportazione di shell, il blocco `env` in `~/.claude/settings.json`, o impostazioni gestite                                                                                                                                                                                                                                   |
| `ANTHROPIC_API_KEY` è impostato ma ignorato, senza prompt                                                                                                                                                                                                                                                                                                                                                                                                                              | La chiave ha bisogno di un'approvazione una tantum nelle sessioni interattive, e una chiave precedentemente rifiutata viene ignorata senza chiedere di nuovo                                                                                                                                                                                                                                                                                                                                 | Abilitala sotto `/config` con l'opzione `Use custom API key`                                                                                                                                                                                                                                                                                                                                                                                  |
| `This machine's managed settings require a first-party login`                                                                                                                                                                                                                                                                                                                                                                                                                          | Le impostazioni gestite includono `forceLoginMethod` o `forceLoginOrgUUID`, che non possono coesistere con `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, o `apiKeyHelper`                                                                                                                                                                                                                                                                                                                     | Il tuo amministratore deve rimuovere `forceLoginMethod` e `forceLoginOrgUUID` dalle impostazioni gestite per usare le credenziali del gateway, o rimuovere la credenziale del gateway per usare l'accesso di prima parte. I due non possono essere combinati                                                                                                                                                                                  |
| `403` con un corpo HTML come `403 Forbidden`, quando i log del gateway stesso non mostrano alcuna richiesta ricevuta                                                                                                                                                                                                                                                                                                                                                                   | Un web application firewall o reverse proxy davanti al gateway ha bloccato il corpo della richiesta prima che raggiungesse il gateway. I prompt di Claude Code includono tag di stile XML e codice sorgente che corrispondono alle regole del corpo dello scripting tra siti, quindi un breve test curl passa mentre una sessione reale no                                                                                                                                                   | Esenta il percorso `/v1/messages` del gateway dall'ispezione del corpo della richiesta. Su AWS WAF questa è la regola gestita `CrossSiteScripting_Body`; su nginx con ModSecurity è la regola del corpo OWASP CRS equivalente                                                                                                                                                                                                                 |
| Errori di certificato o TLS come `SSL certificate verification failed` o `Self-signed certificate detected`, quando il [test curl](#verify-the-connection) ha successo                                                                                                                                                                                                                                                                                                                 | Il runtime di Claude Code non sta fidando della stessa autorità di certificazione che `curl` usa. Comune dietro proxy di ispezione TLS aziendale                                                                                                                                                                                                                                                                                                                                             | Imposta `NODE_EXTRA_CA_CERTS` al percorso del bundle CA; vedi [archivio di certificati CA](/docs/it/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                           |

Se Claude Code ti chiede di accedere ripetutamente dopo aver rimosso la configurazione del gateway, la causa è solitamente l'archiviazione delle credenziali piuttosto che il gateway; vedi [errori di autenticazione](/docs/it/errors#authentication-errors).

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Panoramica dei gateway LLM](/docs/it/llm-gateway): cos'è un gateway e come interagisce con gli abbonamenti a claude.ai
* [Distribuisci un gateway LLM per la tua organizzazione](/docs/it/llm-gateway-rollout): la checklist rivolta agli amministratori per distribuire e distribuire la configurazione del gateway
* [Guida alla compatibilità del gateway](/docs/it/llm-gateway-protocol): cosa Claude Code invia a un gateway, incluse le intestazioni e i campi che il gateway deve inoltrar
* [Impostazioni](/docs/it/settings): dove si trovano i file di impostazioni e come viene letto il blocco `env`
* [Autenticazione](/docs/it/authentication): come le variabili di credenziale, `apiKeyHelper`, e l'accesso OAuth interagiscono
