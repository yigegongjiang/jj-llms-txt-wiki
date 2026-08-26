> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento del protocollo del gateway

> Il contratto API tra Claude Code e un gateway LLM: endpoint, intestazioni e campi del corpo da inoltrare, degradazione delle funzionalità quando i campi vengono rimossi, intestazioni di attribuzione per il tracciamento dei costi e scoperta dei modelli.

Questa pagina documenta le richieste che Claude Code invia a un gateway, inclusi gli endpoint che chiama, le intestazioni e i campi del corpo che il gateway deve inoltrare, e quali funzionalità smettono di funzionare quando non lo fa. È scritta per gli operatori che configurano un prodotto gateway per funzionare con Claude Code.

Un [gateway delle app Claude](/docs/it/claude-apps-gateway) in esecuzione serve una versione leggibile da macchina di questo contratto su `GET /protocol`, coprendo gli stessi requisiti di inoltro più gli endpoint specifici del gateway delle app Claude per l'accesso SSO, la consegna delle impostazioni gestite e la telemetria. Il gateway delle app Claude viene eseguito dallo stesso binario `claude` della CLI, quindi la [guida rapida del gateway delle app Claude](/docs/it/claude-apps-gateway#quickstart) è il percorso più breve verso un'istanza in esecuzione da cui è possibile recuperare la specifica.

<Note>
  * Per distribuire un gateway esistente o di terze parti per la vostra organizzazione, vedere [Distribuire un gateway LLM](/docs/it/llm-gateway-rollout)
  * Se siete uno sviluppatore individuale che autentica Claude Code a un gateway con una credenziale che vi è stata fornita, vedere [Connettere Claude Code a un gateway LLM](/docs/it/llm-gateway-connect)
</Note>

Questa pagina copre:

* [Formati API](#api-formats) e gli endpoint da servire per ciascuno
* [Intestazioni delle richieste](#request-headers): quali devono raggiungere l'upstream e quali il vostro gateway può consumare
* Il [blocco di attribuzione del prompt di sistema](#system-prompt-attribution-block) e come interagisce con il prompt caching
* [Passaggio delle funzionalità](#feature-pass-through): cosa si interrompe quando le intestazioni o i campi del corpo vengono rimossi
* [Scoperta dei modelli](#model-discovery)

Questa pagina utilizza due termini per quello che il vostro gateway fa con ogni intestazione e campo del corpo:

* **Inoltrare invariato**: passarlo all'upstream byte per byte
* **Consumare**: il gateway può leggerlo per il routing, l'attribuzione o il tracciamento e non è necessario inoltrarlo

Qualsiasi cosa non contrassegnata come inoltrare invariato è vostro da consumare o ignorare.

<h2 id="api-formats">
  Formati API
</h2>

Un gateway deve esporre almeno uno dei seguenti formati API ai client Claude Code. Quale formato Claude Code parla è determinato dalla configurazione del client: la variabile nella colonna Selezionato da della tabella sottostante punta Claude Code al vostro gateway in quel formato. Google Cloud's Agent Platform è l'endpoint Claude di Google Cloud, precedentemente Vertex AI; i nomi delle sue variabili mantengono l'ortografia `VERTEX`.

| Formato                                  | Selezionato da                                               | Endpoint                                                                  | Inoltrare invariato                                                                                                        |
| :--------------------------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (opzionale)                   | intestazioni di richiesta `anthropic-beta` e `anthropic-version`                                                           |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` con `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`     | campi del corpo della richiesta `anthropic_beta` e `anthropic_version`                                                     |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` con `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opzionale) | intestazioni di richiesta `anthropic-beta` e `anthropic-version`, e il campo del corpo della richiesta `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry e Claude Platform on AWS
</h3>

Microsoft Foundry e [Claude Platform on AWS](/docs/it/claude-platform-on-aws) implementano il formato Anthropic Messages. Claude Code instrada verso di loro attraverso le loro variabili, `ANTHROPIC_FOUNDRY_BASE_URL` e `ANTHROPIC_AWS_BASE_URL`, ma un gateway che le fronteggia implementa la riga Anthropic Messages sopra. Un gateway che fronteggia Claude Platform on AWS deve anche inoltrare l'intestazione `anthropic-workspace-id`, che [quella piattaforma richiede su ogni richiesta](/docs/it/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Endpoint opzionali e traffico di avvio
</h3>

Gli endpoint di conteggio dei token sono gli unici opzionali: quando sono assenti, Claude Code stima l'utilizzo del contesto localmente. Le richieste di inferenza vengono inviate a `/v1/messages?beta=true`, quindi abbinate sul percorso, non sull'URL completo. Il metodo Google Cloud's Agent Platform allega i suffissi al percorso del modello dell'editore, come in `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`.

Un gateway vede anche il traffico di avvio best-effort che può rifiutare senza rompere nulla: una sonda di connettività `HEAD /`, e su gateway in formato Amazon Bedrock una richiesta `GET /inference-profiles?type=SYSTEM_DEFINED`.

<h3 id="streaming">
  Streaming
</h3>

Le risposte di inferenza devono essere in streaming. Claude Code consuma gli eventi inviati dal server mentre arrivano, quindi un gateway che memorizza nel buffer le risposte complete prima di inoltrarle blocca il client.

<h3 id="format-mismatch-with-the-upstream">
  Mancata corrispondenza del formato con l'upstream
</h3>

Quale formato il client parla determina cosa riceve il vostro gateway. La modalità di errore comune è una mancata corrispondenza tra il formato che il client invia al vostro gateway e il formato che il provider upstream dietro di esso accetta.

* Quando il client parla il formato Amazon Bedrock o Google Cloud's Agent Platform, Claude Code invia solo il sottoinsieme del suo set di capacità completo che quei provider accettano
* Quando il client parla il formato Anthropic Messages, Claude Code invia il set completo, anche se il vostro gateway inoltra a un upstream Amazon Bedrock o Google Cloud's Agent Platform

Colmare quella differenza è il compito del vostro gateway. [Passaggio delle funzionalità](#feature-pass-through) descrive cosa si interrompe quando non lo fa.

<h2 id="request-headers">
  Intestazioni delle richieste
</h2>

Claude Code include queste intestazioni sulle richieste API. I nomi delle intestazioni non fanno distinzione tra maiuscole e minuscole sul filo. Inoltrare `anthropic-version` e `anthropic-beta` invariate, più `anthropic-workspace-id` quando l'upstream è [Claude Platform on AWS](/docs/it/claude-platform-on-aws); il resto il gateway può consumare per il routing, l'attribuzione e il tracciamento, e non è necessario inoltrare.

| Intestazione                    | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Authorization`, `x-api-key`    | La credenziale del gateway dello sviluppatore, in una o entrambe le intestazioni a seconda di quale [variabile di credenziale](/docs/it/llm-gateway-connect#set-the-credential-variable) impostano                                                                                                                                                                                                                                                                                                                 |
| `anthropic-version`             | Versione API, attualmente `2023-06-01`. Le richieste in formato Amazon Bedrock e Agent Platform di Google Cloud portano anche il campo del corpo `anthropic_version`, il cui valore è la stringa del dialetto del provider, non il valore di questa intestazione                                                                                                                                                                                                                                              |
| `anthropic-beta`                | Valori di capacità separati da virgole per la richiesta. Inoltrare l'intestazione verbatim; non creare un elenco di consentiti dei singoli valori, perché l'insieme cambia con i rilasci di Claude Code. Quando lo sviluppatore si autentica con un accesso claude.ai, che è possibile quando `ANTHROPIC_BASE_URL` è impostato senza una variabile di credenziale del gateway, questa intestazione porta anche una capacità OAuth che l'upstream richiede, e rimuoverla fa fallire quelle richieste con `401` |
| `x-claude-code-session-id`      | Un identificatore univoco per la sessione Claude Code corrente. Usarlo per aggregare tutte le richieste da una sessione senza analizzare i corpi delle richieste                                                                                                                                                                                                                                                                                                                                              |
| `x-claude-code-agent-id`        | Identificatore del [subagent](/docs/it/sub-agents) che ha emesso la richiesta, presente solo sulle richieste da un agente che Claude Code ha generato all'interno della sessione. Usarlo con l'ID della sessione per attribuire il costo agli agenti paralleli                                                                                                                                                                                                                                                     |
| `x-claude-code-parent-agent-id` | Identificatore dell'agente che ha generato l'agente richiedente, presente solo per gli agenti annidati                                                                                                                                                                                                                                                                                                                                                                                                        |

Gli ID dei subagent vengono generati freschi per ogni spawn. Gli agenti compagni, i membri denominati di un [team di agenti](/docs/it/agent-teams), riutilizzano un ID stabile basato sul nome tra le riconnessioni. In entrambi i casi l'ID identifica un agente, non una persona o un dispositivo, quindi non trattate l'intestazione dell'ID dell'agente come un identificatore utente.

Se i vostri sviluppatori impostano `ANTHROPIC_CUSTOM_HEADERS`, quelle intestazioni appaiono anche sulle richieste.

<h3 id="forward-as-open-lists">
  Inoltrare come elenchi aperti
</h3>

Trattate le intestazioni e i campi del corpo come elenchi aperti, non chiusi. Claude Code guadagna capacità nei rilasci, e arrivano come nuovi valori `anthropic-beta`, nuovi campi del corpo della richiesta, e occasionalmente nuove intestazioni `anthropic-*` o `x-claude-code-*`.

Quando inoltrate a un upstream in formato Anthropic, passate le intestazioni di richiesta `anthropic-*` e i campi del corpo della richiesta invariati piuttosto che creare un elenco di consentiti di quelli che vedete oggi. Un gateway bloccato a un elenco osservato rimuove l'intestazione o il campo della prossima capacità e lo interrompe nel rilascio che lo introduce.

L'eccezione è un upstream non-Anthropic come Amazon Bedrock o Agent Platform di Google Cloud, dove colmare la differenza dello schema è il compito del gateway; vedere [passaggio delle funzionalità](#feature-pass-through).

<h2 id="system-prompt-attribution-block">
  Blocco di attribuzione del prompt di sistema
</h2>

Claude Code antepone un breve blocco di attribuzione al prompt di sistema contenente la versione del client e un'impronta digitale derivata dalla conversazione. L'endpoint `api.anthropic.com` rimuove il blocco prima dell'elaborazione quando arriva invariato come primo blocco di sistema, quindi non influisce sul prompt caching di prima parte. Qualsiasi altro upstream lo riceve come parte del prompt.

La rimozione è posizionale, quindi funziona solo quando il gateway inoltra l'array `system` invariato. Per mantenere il blocco fuori dal prompt senza perdere altri contenuti di sistema:

* Inoltrare l'array `system` esattamente come ricevuto, mantenendo il blocco per primo: anteporre un altro blocco di sistema, riordinare l'array o convertirlo in una singola stringa annulla la rimozione, e il blocco raggiunge quindi il modello e la chiave della cache del prompt.
* Mantenere il blocco nella sua voce di array separata: l'endpoint tratta un blocco unito che inizia con l'intestazione di attribuzione come attribuzione nella sua interezza e scarta tutto ciò che vi è stato unito, incluso il resto del prompt di sistema.
* Se il vostro gateway deve rimodellare il contenuto di sistema, impostare [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/it/env-vars) in modo che Claude Code ometta il blocco. Anthropic e gli endpoint Claude dei provider cloud leggono il blocco per l'attribuzione, quindi ometterlo nel client piuttosto che rimuoverlo o spostarlo nel gateway.

Le richieste che raggiungono l'endpoint invariate non sono interessate.

Da Claude Code v2.1.181, il blocco è stabile per la durata di una conversazione quando le richieste instradano attraverso un URL di base personalizzato, quindi una cache del prompt lato gateway basata sul corpo della richiesta completa funziona senza disabilitarla. Prima di v2.1.181 il blocco includeva un token per richiesta; su quelle versioni, impostare `CLAUDE_CODE_ATTRIBUTION_HEADER=0` se il vostro gateway implementa una tale cache.

<h2 id="feature-pass-through">
  Passaggio delle funzionalità
</h2>

Claude Code tratta un gateway `ANTHROPIC_BASE_URL` come un endpoint in formato Anthropic e gli invia le intestazioni beta e i campi del corpo della richiesta che invia a `api.anthropic.com`, tranne un piccolo insieme di diagnostica e impostazioni predefinite riservate alle connessioni dirette, come l'impostazione predefinita di streaming fine degli strumenti coperta di seguito. Questo insieme varia per rilascio, quindi non dipendete dal suo contenuto.

Le capacità che aggiungono campi del corpo li associano a un'intestazione beta, e la coppia viaggia insieme. Un gateway che rimuove l'intestazione mentre passa il corpo, o inoltra un corpo in formato Anthropic a un upstream con uno schema diverso, produce errori `400` difficili; solo quando entrambe le metà sono assenti insieme la funzionalità si disattiva silenziosamente. Un gateway che riscrive o redige i corpi delle richieste per l'ispezione del contenuto interrompe l'associazione allo stesso modo della rimozione, quindi ispezionare senza modificare. La tabella nota dove una funzionalità si discosta dall'associazione.

Lo streaming fine degli strumenti è una delle impostazioni predefinite della connessione diretta: è disattivato per impostazione predefinita ogni volta che le richieste instradano attraverso un URL di base personalizzato, e un gateway lo riceve quando gli sviluppatori impostano [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/it/env-vars).

| Funzionalità                                                                                                                                                                                                                                  | Intestazione e coppia del corpo                                                                                                                                                                                                  | Sintomo quando interrotto                                                                                                                  | Rimedio                                                                                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| [Ragionamento adattivo](/docs/it/model-config#adjust-effort-level)                                                                                                                                                                                 | Nessuna intestazione beta. Claude Code invia `thinking: {"type": "adaptive"}` per Claude 4.6 e successivi, e tratta i nomi dei modelli che non riconosce, come gli alias del gateway, come modelli attuali che ricevono il campo | `400` che nomina il campo `thinking` o il tag `adaptive` quando la build del modello upstream non lo accetta                               | Aggiornare l'upstream. Su Opus 4.6 e Sonnet 4.6, gli sviluppatori possono invece impostare `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` |
| [Gestione del contesto](https://platform.claude.com/docs/en/build-with-claude/context-management)                                                                                                                                             | L'intestazione beta di gestione del contesto si associa al campo del corpo `context_management`                                                                                                                                  | `400` con `Extra inputs are not permitted`. Comune quando un gateway accetta richieste in formato Anthropic ma le inoltra a Amazon Bedrock | Inoltrare entrambi, o [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/it/env-vars)                                                     |
| [Contesto esteso](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) e [pensiero interleaved](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Solo intestazioni beta, nessun campo del corpo                                                                                                                                                                                   | Silenziosamente non disponibile quando l'intestazione viene rimossa; l'upstream non vede mai la richiesta di capacità                      | Inoltrare `anthropic-beta` verbatim                                                                                                  |
| Beta [tool fields](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                                    | Le intestazioni beta relative agli strumenti si associano ai campi dello schema dello strumento come `strict` e `defer_loading`                                                                                                  | `400` che nomina il campo dello schema dello strumento non riconosciuto quando il corpo passa attraverso senza la sua intestazione         | Inoltrare entrambi, o `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                                                     |
| [Sforzo](https://platform.claude.com/docs/en/build-with-claude/effort) e [output strutturati](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                       | Il campo del corpo `output_config` contiene impostazioni di sforzo, formato di output strutturato e budget dei compiti; ciascuno si associa con la sua intestazione beta                                                         | `400` che nomina `output_config`, spesso `Extra inputs are not permitted`, su upstream Amazon Bedrock e Google Cloud's Agent Platform      | Inoltrare il campo e le sue intestazioni insieme                                                                                     |
| [Conteggio dei token](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                   | Nessun accoppiamento beta; utilizza l'endpoint `count_tokens`                                                                                                                                                                    | Claude Code ricade alla stima dell'utilizzo del contesto localmente                                                                        | Esporre l'endpoint se desiderate conteggi esatti                                                                                     |

Le variabili `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [variables](/docs/it/model-config) dichiarano le capacità del modello solo nelle configurazioni del provider: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, e [`CLAUDE_CODE_USE_MANTLE`](/docs/it/amazon-bedrock#use-the-mantle-endpoint). Non hanno effetto dietro un gateway `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Ritentativo automatico e inoltro degli errori
</h3>

Claude Code ritenta automaticamente dopo alcuni rifiuti upstream e disabilita la capacità rifiutata per il resto della conversazione. I rifiuti del campo `thinking`, delle [thinking signatures](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), e dei messaggi di sistema a metà conversazione si riprendono in questo modo. I rifiuti di gestione del contesto e di campi dello schema dello strumento non ritentano; quegli errori `400` raggiungono lo sviluppatore.

La logica di ritentativo corrisponde alla formulazione dell'errore dell'upstream, quindi inoltrare i corpi della risposta di errore invariati. Un gateway che avvolge gli errori upstream nel suo involucro interrompe il percorso di recupero anche quando preserva il codice di stato.

<h3 id="disable-pre-release-capabilities">
  Disabilitare le capacità pre-release
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` impedisce a Claude Code di inviare capacità pre-release e i loro campi del corpo su ogni provider, inclusa la gestione del contesto e i campi dello strumento beta. Non influisce sul ragionamento adattivo, che è selezionato dal modello piuttosto che da beta, e non sopprime mai la capacità OAuth che l'autenticazione della sottoscrizione richiede.

L'insieme delle capacità che Claude Code invia cresce nei rilasci. Per le stringhe di intestazione beta attuali, vedere il [riferimento delle intestazioni beta](https://platform.claude.com/docs/en/api/beta-headers); testare il vostro gateway contro i nuovi rilasci di Claude Code piuttosto che bloccare a un elenco osservato.

<h2 id="model-discovery">
  Scoperta dei modelli
</h2>

Quando `ANTHROPIC_BASE_URL` punta a un gateway che espone il formato Anthropic Messages, Claude Code può interrogare l'endpoint `/v1/models` del gateway all'avvio e aggiungere i modelli restituiti al selettore `/model`.

Gli sviluppatori lo abilitano impostando [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/it/env-vars), nel loro ambiente o attraverso le impostazioni gestite. La scoperta è disattivata per impostazione predefinita in modo che i gateway supportati da una chiave API condivisa non espongano ogni modello a cui la chiave può accedere a ogni utente. Questo richiede Claude Code v2.1.129 o successivo.

<h3 id="when-discovery-runs">
  Quando viene eseguita la scoperta
</h3>

La scoperta si applica solo al formato Anthropic Messages. Non viene eseguita quando:

* Qualsiasi variabile del provider `CLAUDE_CODE_USE_*` è impostata, anche se `ANTHROPIC_BASE_URL` è anche impostato
* `ANTHROPIC_BASE_URL` non è impostato o punta a `api.anthropic.com`
* Il traffico non essenziale è disabilitato, attraverso [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars) o la politica dell'organizzazione

<h3 id="request-and-response">
  Richiesta e risposta
</h3>

La richiesta è `GET /v1/models?limit=1000` con un timeout di 3 secondi, e qualsiasi reindirizzamento è trattato come fallimento in modo che la credenziale non possa trapelare a una destinazione di reindirizzamento. Un gateway che risponde lentamente o reindirizza `/v1/models`, anche da `http` a `https`, fallisce la scoperta silenziosamente; servire l'endpoint direttamente all'URL di base configurato.

La richiesta di scoperta invia esattamente un'intestazione di credenziale:

* `ANTHROPIC_AUTH_TOKEN` come token bearer, quando impostato
* Altrimenti la chiave API risolta, incluso un valore [`apiKeyHelper`](/docs/it/llm-gateway-connect#rotate-credentials-with-apikeyhelper), nell'intestazione `x-api-key`

Questo differisce dalle richieste di inferenza, che inviano un valore helper in entrambe le intestazioni. Un gateway che autentica `/v1/models` deve accettare `x-api-key` per le distribuzioni helper. Qualsiasi intestazione da `ANTHROPIC_CUSTOM_HEADERS` è inclusa anche.

Claude Code legge `id` e il `display_name` opzionale da ogni voce nell'array `data` della risposta, e ignora le voci il cui `id` non inizia con `claude` o `anthropic`:

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  Voci del selettore e caching
</h3>

Il selettore è l'elenco interattivo dei modelli che si apre quando uno sviluppatore esegue `/model` in Claude Code. Ogni voce scoperta è etichettata "From gateway" e utilizza `display_name` quando fornito. L'[impostazione gestita `availableModels`](/docs/it/settings#available-settings) limita quello che la scoperta può aggiungere.

Un ID scoperto viene saltato quando corrisponde esattamente a una riga già nel selettore, o quando sia l'ID scoperto che quello esistente si risolvono in [Fable](/docs/it/model-config#work-with-fable-5). A partire da Claude Code v2.1.197, un ID esplicito scoperto viene anche ripiegato in una voce integrata quando entrambi si risolvono nello stesso modello. Le righe integrate sono codificate su alias come `sonnet`, quindi un ID scoperto esplicito del modello a cui l'alias attualmente si risolve, come `claude-sonnet-5`, si comprime nella riga `sonnet`, mentre un ID a cui l'alias non si risolve, come `claude-sonnet-4-6`, aggiunge comunque la sua propria riga "From gateway" accanto alla voce integrata.

I risultati vengono memorizzati nella cache in `~/.claude/cache/gateway-models.json`, o `%USERPROFILE%\.claude\cache\gateway-models.json` su Windows, e aggiornati ad ogni avvio. Se la richiesta fallisce o il gateway non implementa `/v1/models`, il selettore ricade all'elenco memorizzato nella cache dall'avvio precedente o all'elenco dei modelli integrati. Se il vostro gateway serve modelli Claude con alias che non corrispondono al filtro di scoperta, gli sviluppatori possono aggiungere quegli alias manualmente con le [variabili di configurazione del modello](/docs/it/model-config).

<h2 id="related-resources">
  Risorse correlate
</h2>

Per il resto della serie di documentazione del gateway e i riferimenti API sottostanti:

* [Panoramica dei gateway](/docs/it/gateways): cos'è un gateway e come scegliere tra l'app gateway Claude e un altro prodotto
* [Altri gateway LLM](/docs/it/llm-gateway): come implementare un gateway che la vostra organizzazione gestisce e come interagisce con le sottoscrizioni claude.ai
* [Distribuire un gateway LLM per la vostra organizzazione](/docs/it/llm-gateway-rollout): la checklist dell'amministratore che utilizza questo contratto
* [Connettere Claude Code a un gateway LLM](/docs/it/llm-gateway-connect): configurazione per sviluppatore e la tabella di risoluzione dei problemi
* [Riferimento delle intestazioni beta](https://platform.claude.com/docs/en/api/beta-headers): l'insieme attuale dei valori `anthropic-beta`
* [API Messages](https://platform.claude.com/docs/en/api/messages): il formato API che un gateway in formato Anthropic implementa
