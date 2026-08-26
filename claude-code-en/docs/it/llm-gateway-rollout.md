> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Distribuire un gateway LLM per la vostra organizzazione

> Distribuire un prodotto gateway per Claude Code: configurarlo per inoltrare ciò che Claude Code invia, emettere credenziali per sviluppatori, distribuire la configurazione tramite impostazioni gestite e verificare la distribuzione.

Questa pagina guida un amministratore nella distribuzione di un gateway LLM per Claude Code. Presuppone che abbiate un prodotto gateway distribuito che soddisfi i [requisiti del gateway](#gateway-requirements). La distribuzione o l'operazione di qualsiasi prodotto specifico non è trattata qui; distribuite il vostro seguendo la documentazione del fornitore.

<Note>
  * Per connettere Claude Code sulla vostra macchina a un gateway esistente, vedere [Connettere Claude Code a un gateway LLM](/docs/it/llm-gateway-connect)
  * Per informazioni su ciò che Claude Code invia a un gateway e cosa inoltrare, vedere il [riferimento del protocollo gateway](/docs/it/llm-gateway-protocol)
</Note>

<h2 id="prerequisites">
  Prerequisiti
</h2>

Per completare la distribuzione, avrete bisogno di:

* Un gateway distribuito sulla vostra infrastruttura, che serve HTTPS all'indirizzo esatto che distribuirete agli sviluppatori, non a un indirizzo che vi reindirizza, e configurato per instradare i nomi dei modelli Claude al vostro provider
* Una credenziale del provider per il gateway da inoltrare con:
  * Per l'API Anthropic: una chiave API dalla [Claude Console](https://platform.claude.com/settings/keys)
  * Per un provider cloud: credenziali cloud con accesso ai modelli. Vedere i prerequisiti nella pagina [Amazon Bedrock](/docs/it/amazon-bedrock#prerequisites), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai#prerequisites) o [Microsoft Foundry](/docs/it/microsoft-foundry#prerequisites)
* Un modo per consegnare file di impostazioni alle macchine degli sviluppatori, come MDM o gestione della configurazione
  * Se non ne avete ancora uno, [come le impostazioni raggiungono i dispositivi](/docs/it/admin-setup#decide-how-settings-reach-devices) confronta le opzioni

<h3 id="gateway-requirements">
  Requisiti del gateway
</h3>

Qualunque prodotto fornisca il gateway, deve:

* **Accettare un formato API supportato**: uno dei formati nella [tabella dei formati API](/docs/it/llm-gateway-protocol#api-formats). I passaggi di distribuzione di seguito presuppongono l'API Anthropic Messages a `POST /v1/messages`, che la maggior parte dei gateway serve
* **Trasmettere le risposte in streaming**: passare gli eventi inviati dal server così come arrivano invece di memorizzare l'intera risposta nel buffer
* **Instradare i nomi dei modelli Claude**: mappare ogni nome che gli sviluppatori usano a un modello upstream. Claude Code invia un nome di modello come `claude-sonnet-4-6` in ogni richiesta; nella maggior parte dei prodotti gateway il mapping è un elenco di modelli o una tabella di instradamento nella configurazione del gateway stesso
* **Inoltrare intestazioni e corpo invariati**: passare `anthropic-beta`, `anthropic-version` e il corpo della richiesta in entrambe le direzioni; la [tabella di pass-through delle funzionalità](/docs/it/llm-gateway-protocol#feature-pass-through) mappa ciascuno alla funzionalità che si interrompe senza di esso
* **Restituire gli errori upstream non modificati**: il recupero automatico di Claude Code corrisponde alla formulazione dell'errore, quindi avvolgere gli errori nel proprio envelope del gateway lo interrompe
* **Esentare il percorso dall'ispezione WAF del corpo della richiesta**: i prompt di Claude Code contengono codice sorgente e tag in stile XML che corrispondono alle regole del corpo dello scripting tra siti; un WAF davanti al gateway restituisce `403` su sessioni reali mentre le brevi richieste di test passano

Facoltativamente, servire `GET /v1/models` in modo che Claude Code possa popolare il selettore di modelli dal vostro gateway con [model discovery](/docs/it/llm-gateway-protocol#model-discovery).&#x20;

<h2 id="rollout-steps">
  Passaggi di distribuzione
</h2>

La distribuzione richiede cinque passaggi, ciascuno con un checkpoint:

1. [Confermare che il gateway instrada i vostri modelli](#confirm-the-gateway-routes-your-models)
2. [Emettere a ogni sviluppatore una credenziale](#issue-developer-credentials)
3. [Testare Claude Code rispetto al gateway](#test-claude-code-against-the-gateway)
4. [Distribuire l'URL di base e le credenziali](#distribute-the-configuration)
5. [Verificare da una macchina sviluppatore](#verify-the-rollout)

I passaggi coinvolgono tre credenziali diverse e i checkpoint le nominano per segnaposto in modo che possiate dire quale sia responsabile quando qualcosa fallisce:

| Credenziale                            | Chi la detiene                                                                                                 | Segnaposto nei checkpoint                                  |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------- |
| Credenziale del provider               | Il gateway, che la inoltra al provider upstream                                                                | Configurata sul gateway; non appare mai nei comandi client |
| Credenziale amministrativa del gateway | Voi, se il vostro prodotto gateway ne emette una per la sua interfaccia amministrativa o di test               | `<gateway-key>`                                            |
| Chiave dello sviluppatore              | Ogni sviluppatore, emessa dal gateway in [Emettere credenziali per sviluppatori](#issue-developer-credentials) | `<developer-key>`                                          |

<h3 id="confirm-the-gateway-routes-your-models">
  Confermare che il gateway instrada i vostri modelli
</h3>

Il vostro gateway dovrebbe già essere configurato con la vostra credenziale del provider, in ascolto al suo URL di base e inoltro delle richieste all'API del vostro provider. Testate che il percorso funzioni da capo a fondo con una richiesta minima, sostituendo due valori dalla vostra distribuzione:

* `<gateway-key>` è qualunque credenziale vi consenta di chiamare il gateway in questo momento: una chiave amministrativa, una chiave di test o la vostra chiave sviluppatore personale se ne avete già emessa una. Non tutti i prodotti gateway hanno una credenziale amministrativa separata; se il vostro non ne ha una, emettete prima una chiave sviluppatore per voi stessi in [Emettere credenziali per sviluppatori](#issue-developer-credentials)
* `model` è un nome di modello Claude che il vostro gateway è configurato per instradare. L'esempio usa `claude-sonnet-4-6`; sostituite un nome che avete configurato

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <gateway-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Checkpoint**: un `200` con un campo `content` significa che il gateway ha raggiunto il provider con quel nome di modello. Un `404` significa che quel nome non è instradato al gateway; un `401` dal provider significa che la credenziale del provider del gateway è sbagliata.

Ripetete la richiesta una volta per ogni nome di modello Claude nella configurazione di instradamento del vostro gateway. Un nome che il gateway non instrada restituisce `404` a qualsiasi sviluppatore che lo seleziona, quindi testate ogni nome prima della distribuzione.

<Note>
  Evitate di servire il gateway dietro un reindirizzamento. Un reindirizzamento può eliminare il corpo della richiesta o rimuovere l'intestazione della credenziale sulle richieste di inferenza, e [model discovery](/docs/it/llm-gateway-protocol#model-discovery) tratta qualsiasi reindirizzamento come un fallimento in modo che la credenziale non possa trapelare a una destinazione di reindirizzamento.
</Note>

<h3 id="issue-developer-credentials">
  Emettere credenziali per sviluppatori
</h3>

Ogni sviluppatore ha bisogno della propria chiave gateway per autenticarsi. Create una credenziale per sviluppatore al gateway, seguendo la documentazione di gestione delle credenziali del vostro prodotto.

Confermate che una chiave appena emessa funzioni rispetto al gateway con la stessa richiesta di [Confermare che il gateway instrada i vostri modelli](#confirm-the-gateway-routes-your-models), sostituendo `<gateway-key>` con la nuova `<developer-key>`:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Checkpoint**: un `200` con un campo `content` significa che la chiave dello sviluppatore raggiunge il gateway e il gateway la inoltra. Un `401` qui, quando [il passaggio precedente](#confirm-the-gateway-routes-your-models) ha avuto successo, significa che la chiave dello sviluppatore è sbagliata o non ha ancora avuto effetto al gateway.

Emettere una chiave per sviluppatore piuttosto che una chiave condivisa è ciò che rende possibile l'attribuzione dell'utilizzo per sviluppatore e l'offboarding individuale. La variabile di ambiente che contiene la chiave dipende da quale intestazione il gateway legge. Per un gateway che controlla le credenziali nell'intestazione `Authorization: Bearer`, gli sviluppatori impostano la loro chiave in `ANTHROPIC_AUTH_TOKEN`. Per un gateway che legge le chiavi dall'intestazione `x-api-key`, gli sviluppatori impostano invece `ANTHROPIC_API_KEY`; la [tabella delle credenziali](/docs/it/llm-gateway-connect#set-the-credential-variable) copre il mapping.

<h3 id="test-claude-code-against-the-gateway">
  Testare Claude Code rispetto al gateway
</h3>

Eseguite Claude Code attraverso il gateway voi stessi prima di distribuire qualcosa, usando la stessa configurazione che la distribuzione consegnerà a livello di flotta. Digitate questi direttamente in un terminale, non in un file `.env` o di impostazioni; durano solo per questa sessione di terminale, quindi chiuderla restituisce la vostra macchina alla sua configurazione normale. Usate `ANTHROPIC_API_KEY` invece di `ANTHROPIC_AUTH_TOKEN` se il vostro gateway legge l'intestazione `x-api-key`:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN="<developer-key>"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
    ```
  </Tab>
</Tabs>

Quindi inviate un prompt una tantum attraverso il gateway:

```bash theme={null}
claude -p "Reply with one word: connected"
```

**Checkpoint**: il prompt restituisce una risposta e la richiesta appare nel log del gateway come un `POST` al percorso `/v1/messages` con stato `200`. Claude Code aggiunge una stringa di query come `?beta=true`, quindi abbinate il percorso, non l'URL completo. Due messaggi di errore puntano in direzioni diverse:

* `Not logged in`: controllate il log del gateway per distinguere le due cause. Se è vuoto, nessuna credenziale ha raggiunto la sessione e nessuna richiesta ha lasciato la macchina; rieseguite le esportazioni nella shell da cui state testando. Se mostra una richiesta rifiutata con `x-api-key` nel corpo `401`, il gateway si aspetta chiavi in quell'intestazione invece; passate a `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` significa che una credenziale è stata inviata e rifiutata, e il log del gateway dice dove: un `401` che nomina `api.anthropic.com` o l'endpoint del vostro provider significa che il gateway ha raggiunto l'upstream ma la sua credenziale del provider è stata rifiutata, quindi la chiave dello sviluppatore ha funzionato e la credenziale del provider che il gateway detiene è sbagliata o un segnaposto

Un URL di base sbagliato o irraggiungibile produce un sintomo diverso: Claude Code [ritenta la connessione con backoff](/docs/it/errors#automatic-retries) e può stare senza output per diversi minuti prima di segnalare un errore. Se il comando sembra bloccarsi, controllate il log del gateway invece di aspettare; nessuna richiesta in arrivo significa che `ANTHROPIC_BASE_URL` non punta al gateway.

<h3 id="distribute-the-configuration">
  Distribuire la configurazione
</h3>

Ogni macchina sviluppatore ha bisogno dell'indirizzo del gateway e di una credenziale. Potete distribuirli centralmente tramite [impostazioni gestite](/docs/it/settings#settings-files), in modo che gli sviluppatori non configurino nulla, o consegnate agli sviluppatori i valori da impostare loro stessi.

<h4 id="what-to-distribute">
  Cosa distribuire
</h4>

Lo stesso insieme di variabili si applica qualunque percorso scegliate. La maggior parte delle distribuzioni ha bisogno solo di `ANTHROPIC_BASE_URL` e di una credenziale; includete le righe condizionali quando la vostra configurazione del gateway lo richiede.

| Variabile o impostazione                                                                                                                                                                                                      | Cosa fa                                                                                                                                                                                                  | Includere quando                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                                          | Invia le richieste API di Claude Code al gateway invece di `api.anthropic.com`                                                                                                                           | Sempre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `apiKeyHelper`, o una credenziale in `ANTHROPIC_AUTH_TOKEN` o `ANTHROPIC_API_KEY`                                                                                                                                             | Autentica ogni richiesta al gateway. L'helper esegue un comando per recuperare la chiave; le variabili contengono una chiave statica, inviata come `Authorization: Bearer` e `x-api-key` rispettivamente | Sempre; uno dei tre                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                                                    | Aggiunge intestazioni HTTP extra a ogni richiesta API                                                                                                                                                    | Il vostro gateway richiede un'intestazione tenant o di instradamento su ogni richiesta                                                                                                                                                                                                                                                                                                                                                                                                           |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                                                  | Interroga il `/v1/models` del gateway all'avvio e aggiunge i nomi restituiti al selettore `/model`                                                                                                       | Il vostro gateway serve `/v1/models` e volete che i selettori degli sviluppatori siano popolati da esso                                                                                                                                                                                                                                                                                                                                                                                          |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                                                      | Impedisce a Claude Code di inviare intestazioni di capacità pre-release e campi del corpo                                                                                                                | Il vostro gateway inoltra a un upstream Bedrock o Vertex che rifiuta i campi beta; vedere [Requisiti del gateway](#gateway-requirements)                                                                                                                                                                                                                                                                                                                                                         |
| `ANTHROPIC_MODEL` o [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/it/model-config)                                                                                                                                                       | Impostare quale nome di modello Claude Code richiede per la sessione principale e per il traffico di background                                                                                          | Il vostro gateway instrada nomi di modelli che non corrispondono ai valori predefiniti di Claude Code, o instradare [funzionalità di background](/docs/it/costs#background-token-usage) a un modello diverso. Instradare sia i nomi di override che i nomi predefiniti di Claude Code al gateway, poiché alcune sub-call possono richiedere il nome predefinito indipendentemente dall'override; la [configurazione del modello](/docs/it/model-config) copre quale modello ogni parte di una sessione usa |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL`, o `ANTHROPIC_AWS_BASE_URL` con le [variabili per quel provider](/docs/it/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Puntare Claude Code al gateway attraverso un URL di base specifico del provider. Bedrock e Agent Platform passano anche al formato di richiesta nativo di quei provider                                  | Il vostro gateway fronteggia Bedrock, Agent Platform, Foundry o la Claude Platform su AWS; vedere [Formati API](/docs/it/llm-gateway-protocol#api-formats)                                                                                                                                                                                                                                                                                                                                            |

<h4 id="distribute-through-managed-settings">
  Distribuire tramite impostazioni gestite
</h4>

Consegnate le variabili attraverso il blocco `env` di un [file di impostazioni gestite](/docs/it/settings#settings-files), spinto da MDM, criterio di registro o gestione della configurazione:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

Aggiungete le variabili condizionali dalla tabella allo stesso blocco `env`. Un `ANTHROPIC_BASE_URL` gestito è applicato e non può essere sovrascritto da un'esportazione di shell di uno sviluppatore, poiché Claude Code lo applica sopra l'ambiente del processo e le impostazioni di priorità inferiore.

Non includete `forceLoginMethod` o `forceLoginOrgUUID` nelle impostazioni gestite insieme a una credenziale gateway. Su Claude Code v2.1.146 e successivi, una delle due chiavi, con qualsiasi valore, blocca `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` e `apiKeyHelper` all'avvio, in modo che gli sviluppatori vedano `This machine's managed settings require a first-party login` e non possano procedere.&#x20;

La consegna di [impostazioni gestite dal server](/docs/it/server-managed-settings#platform-availability) richiede una connessione diretta a `api.anthropic.com`, quindi non raggiunge le sessioni instradate dal gateway. Le distribuzioni gateway utilizzano questo percorso di impostazioni gestite basato su file, che applica le stesse chiavi.

Per la credenziale, distribuite un comando [`apiKeyHelper`](/docs/it/llm-gateway-connect#rotate-credentials-with-apikeyhelper) nel file di impostazioni gestite come mostrato sopra; il comando si autentica al vostro archivio di segreti come sviluppatore locale, quindi ogni macchina riceve la propria chiave. In alternativa, consegnate a ogni sviluppatore la sua chiave attraverso il vostro processo di segreti esistente e fate loro impostare `ANTHROPIC_AUTH_TOKEN` loro stessi.

Alcuni ambienti hanno bisogno di consegna separata:

* L'app desktop legge l'instradamento del gateway dalla sua configurazione di inferenza di terze parti, non dalle impostazioni gestite; distribuite quel file attraverso MDM insieme alle impostazioni gestite in modo che le sessioni desktop instradino anche attraverso il gateway. Vedere la [documentazione di configurazione di terze parti del desktop](https://claude.com/docs/third-party/claude-desktop/configuration) e la [documentazione del gateway del desktop](https://claude.com/docs/third-party/claude-desktop/gateway)
* I runner CI hanno bisogno di `ANTHROPIC_BASE_URL` e della credenziale impostati nell'[ambiente del runner](/docs/it/llm-gateway-connect#configure-each-surface)
* WSL su macchine Windows gestite legge le impostazioni gestite di Windows solo quando [`wslInheritsWindowsSettings`](/docs/it/settings#available-settings) è `true`

<h4 id="hand-developers-the-values-to-set-themselves">
  Consegnare agli sviluppatori i valori da impostare loro stessi
</h4>

Se non avete la distribuzione di impostazioni gestite in atto, inviate a ogni sviluppatore ciò di cui ha bisogno per seguire la [pagina di connessione](/docs/it/llm-gateway-connect#configure-claude-code-yourself):

* L'URL del gateway
* La loro credenziale personale
* **Quale variabile mettere la credenziale in**: `ANTHROPIC_AUTH_TOKEN` per un gateway bearer-token, o `ANTHROPIC_API_KEY` per un gateway `x-api-key`. Dire agli sviluppatori quale uno risparmia loro il trial-and-error descritto nella [pagina di connessione](/docs/it/llm-gateway-connect#set-the-credential-variable)
* Qualsiasi variabile condizionale dalla [tabella Cosa distribuire](#what-to-distribute), con i loro valori

La [pagina di connessione](/docs/it/llm-gateway-connect#configure-claude-code-yourself) guida gli sviluppatori attraverso l'impostazione di ciascuno.

**Checkpoint**: su una macchina sviluppatore, `claude` avvia una sessione senza mostrare la schermata di accesso, poiché la credenziale distribuita soddisfa l'autenticazione. Quindi eseguite `/status` e aprite la scheda **Status**: la riga `Anthropic base URL` mostra l'indirizzo del gateway, e per la distribuzione gestita la riga `Setting sources` include impostazioni gestite. Una schermata di accesso, o una riga `Anthropic base URL` mancante, significa che la configurazione non ha raggiunto la macchina.

<h3 id="verify-the-rollout">
  Verificare la distribuzione
</h3>

Confermate che tutto funzioni da una macchina sviluppatore, non dall'host del gateway, in modo che il test copra il percorso di rete che gli sviluppatori usano. Inviate una richiesta in streaming, che controlla l'endpoint, il pass-through dello streaming e l'instradamento del modello in una volta:

<Tabs>
  <Tab title="Bash o Zsh">
    ```bash theme={null}
    curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
      -H "Authorization: Bearer <developer-key>" `
      -H "anthropic-version: 2023-06-01" `
      -H "content-type: application/json" `
      --data-binary '@-'
    ```
  </Tab>
</Tabs>

Dovreste vedere le righe `data:` arrivare in modo incrementale. L'intera risposta che arriva in una volta dopo una pausa significa che il gateway sta memorizzando nel buffer, il che blocca Claude Code; un `404` significa che il nome del modello non è instradato. Ripetete per nome di modello.

Quindi avviate `claude` e inviate un messaggio. Ogni sintomo in questo passaggio ha una causa:

* Un prompt di accesso significa un divario di credenziale. Eseguite `/status` e aprite la scheda **Status**: quando la riga `Setting sources` non include impostazioni gestite, la distribuzione non ha raggiunto la macchina; quando lo fa, la credenziale dello sviluppatore non è stata consegnata, quindi impostate `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper`
* Gli errori `Failed to authenticate` significano che il gateway sta rifiutando le richieste; il suo log dice quale credenziale ha fallito. Un rifiuto che il gateway registra stesso nomina la chiave dello sviluppatore, mentre un `401` da `api.anthropic.com` o dall'endpoint del vostro provider significa che la credenziale del provider che il gateway detiene è stata rifiutata
* Un prompt di approvazione una tantum per la chiave è previsto al primo utilizzo quando il gateway si aspetta chiavi nell'intestazione `x-api-key`, impostata come `ANTHROPIC_API_KEY`. Con `ANTHROPIC_AUTH_TOKEN`, nessun prompt appare e la variabile prende il controllo silenziosamente; un accesso claude.ai precedentemente salvato è inattivo per quella sessione

Infine, controllate i log del gateway per il messaggio che avete inviato: la credenziale identifica lo sviluppatore, e l'[intestazione `x-claude-code-session-id`](/docs/it/llm-gateway-protocol#request-headers) raggruppa le richieste per sessione. Se le funzionalità falliscono con i [sintomi di risoluzione dei problemi](/docs/it/llm-gateway-connect#troubleshoot-gateway-errors), il gateway sta rimuovendo intestazioni o riscrivendo errori; vedere i [requisiti del gateway](#gateway-requirements) sopra.

<h2 id="maintain-the-gateway">
  Mantenere il gateway
</h2>

Dopo la distribuzione, tre tipi di cambiamento raggiungono il gateway nel tempo. Ciascuno ha un sintomo da osservare e un'azione da intraprendere.

| Cambiamento                                                                                           | Sintomo quando il gateway non è aggiornato                                                                                                                                         | Azione                                                                                                                                                                                                                                                                                       |
| :---------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Le nuove versioni di Claude Code aggiungono valori `anthropic-beta` e campi del corpo della richiesta | Gli sviluppatori segnalano errori `400` che nominano un nuovo campo dopo aver aggiornato Claude Code; vedere [feature pass-through](/docs/it/llm-gateway-protocol#feature-pass-through) | Inoltrare le intestazioni `anthropic-*` e i corpi delle richieste verbatim piuttosto che allowlisting; testare le nuove versioni di Claude Code rispetto al gateway prima che raggiungano gli sviluppatori                                                                                   |
| Nuovi modelli Claude diventano disponibili                                                            | Gli sviluppatori che selezionano un nuovo nome di modello ottengono `404`; il selettore `/model` non lo elenca                                                                     | Aggiungere il nome del modello alla configurazione di instradamento del gateway, quindi rieseguire il [controllo di instradamento](#confirm-the-gateway-routes-your-models). Se distribuite `ANTHROPIC_MODEL` o le variabili del modello predefinito, aggiornate le impostazioni gestite     |
| Le credenziali scadono o hanno bisogno di rotazione                                                   | Tutte le richieste degli sviluppatori iniziano a fallire con `401` dall'upstream                                                                                                   | Ruotate la credenziale del provider del gateway secondo il vostro programma; le chiavi dello sviluppatore ruotano al gateway, e un [`apiKeyHelper`](/docs/it/llm-gateway-connect#rotate-credentials-with-apikeyhelper) gestisce la rotazione per sviluppatore senza ridistribuire le impostazioni |

Quando dimensionate i limiti di velocità per chiave, tenete conto del client che [ritenta i fallimenti transitori](/docs/it/errors#automatic-retries), incluse le risposte `429`, fino a 10 volte con backoff, onorando `Retry-After`. Mantenete il [riferimento del protocollo](/docs/it/llm-gateway-protocol) come il contratto per ciò che ogni versione di Claude Code invia.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Connettere Claude Code a un gateway LLM](/docs/it/llm-gateway-connect): i passaggi di configurazione rivolti agli sviluppatori, con configurazione per superficie e una tabella di risoluzione dei problemi che potete consegnare agli sviluppatori
* [Riferimento del protocollo gateway](/docs/it/llm-gateway-protocol): il contratto di rete per gli operatori del gateway, che copre endpoint, intestazioni da inoltrare e la tabella di pass-through delle funzionalità
* [File di impostazioni e precedenza](/docs/it/settings#settings-files): come le impostazioni gestite, di progetto e utente si combinano, e dove va il file gestito su ogni piattaforma
* [Configurare Claude Code per la vostra organizzazione](/docs/it/admin-setup): la distribuzione più ampia di cui questo gateway è una parte, inclusa l'applicazione delle politiche, la visibilità dell'utilizzo e la gestione dei dati
