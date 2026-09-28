> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Verificare l'identità della sessione negli ambienti self-hosted

> Verificare il JWT CLAUDE_CODE_SESSION_ACCESS_TOKEN in modo che i servizi sulla vostra rete possano fidarsi delle richieste provenienti da sessioni nel vostro ambiente self-hosted.

<Note>
  Gli ambienti self-hosted sono in beta pubblica sui piani Team ed Enterprise; un [Owner](/docs/it/cloud-environments#organization-shared-environments) li abilita attivando **Allow self-hosted environments** nella [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Questa pagina copre la verifica dell'identità della sessione; consultare la [guida rapida](/docs/it/self-hosted-environments-quickstart) per la configurazione e [Deploy to production](/docs/it/self-hosted-environments-deploy) per le ricette della flotta.
</Note>

Un [ambiente self-hosted](/docs/it/self-hosted-environments) consente alle sessioni di [Claude Code sul web](/docs/it/claude-code-on-the-web) di eseguirsi su infrastrutture che voi gestite invece che su quelle di Anthropic. Poiché la sessione viene eseguita all'interno della vostra rete, Claude può chiamare direttamente i vostri servizi interni. Questi servizi hanno bisogno di un modo per confermare che una richiesta proviene da una sessione Claude Code nel vostro ambiente e per identificare l'identità dell'utente o del servizio che ha creato quella sessione.

Ogni sessione in un ambiente self-hosted riceve un JSON Web Token (JWT) firmato nella variabile di ambiente `CLAUDE_CODE_SESSION_ACCESS_TOKEN`. Una sessione presenta il token come qualsiasi credenziale bearer; ad esempio, uno script che Claude esegue può chiamare il vostro servizio con `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"`. Anthropic firma il token e pubblica le chiavi di verifica in un endpoint JWKS pubblico. I vostri servizi recuperano quelle chiavi, verificano la firma e leggono i claim per decidere quale accesso concedere.

<h2 id="the-session-token">
  Il token della sessione
</h2>

Prima di scrivere il codice di verifica, sapete cosa stabilisce il token e la forma che la vostra libreria JWT vedrà.

<h3 id="what-the-token-proves">
  Cosa prova il token
</h3>

Un token valido stabilisce alcuni fatti e deliberatamente non altri:

* **Prova**: Anthropic ha emesso il token per una sessione specifica in un ambiente specifico e come è stata creata la sessione: da un utente nella vostra organizzazione, o dall'identità del servizio della vostra organizzazione, che è come iniziano le [sessioni del canale Claude Tag](https://claude.com/docs/claude-tag/concepts/agent-identity)
* **Non prova**: quale processo sull'host del runner lo presenta. Il token si trova in una variabile di ambiente all'interno della sessione, quindi qualsiasi codice che Claude esegue e qualsiasi tool o server MCP che la sessione avvia può leggerlo e presentarlo.

Due conseguenze per i vostri servizi:

* Verificate il claim `aud` rispetto all'ID del vostro ambiente, il valore `ccpool_...` mostrato con il vostro ambiente nella [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), per rifiutare i token emessi per l'ambiente di qualsiasi altra organizzazione.
* Limitate le credenziali che derivate dal token a quello che una singola sessione di codifica dovrebbe essere in grado di fare, non a tutto quello che il creatore della sessione può fare. Consultare [Scope derived credentials](#scope-derived-credentials).

<h3 id="token-format">
  Formato del token
</h3>

Il valore di `CLAUDE_CODE_SESSION_ACCESS_TOKEN` ha un prefisso `sk-ant-cc-` seguito da un JWT standard a tre parti:

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

Rimuovete il prefisso prima di passare il valore a una libreria JWT. I token emessi per le sessioni cloud ospitate da Anthropic portano invece un prefisso `sk-ant-si-` e sono firmati da un diverso set di chiavi, quindi rifiutate qualsiasi valore che non inizi con `sk-ant-cc-`.

L'algoritmo di firma è `ES256`, che è ECDSA sulla curva P-256 con SHA-256. L'intestazione del token porta un `kid` che identifica quale chiave nel JWKS lo ha firmato.

<h2 id="verify-the-token">
  Verificare il token
</h2>

La verifica viene eseguita in uno di due posti. I servizi sulla vostra rete verificano il token crittograficamente rispetto alle chiavi pubblicate da Anthropic, e gli script wrapper all'interno della sessione possono invece utilizzare il decoder integrato del binario del runner.

<h3 id="verify-the-token-from-your-service">
  Verificare il token dal vostro servizio
</h3>

Anthropic pubblica le chiavi di verifica in un endpoint pubblico e non autenticato:

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

La risposta è un [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517) standard. Anthropic ruota periodicamente le chiavi di firma e le chiavi precedenti a una rotazione rimangono nel set abbastanza a lungo affinché i token che hanno firmato continuino a verificarsi, quindi non fissate una singola chiave. L'endpoint imposta `Cache-Control: public, max-age=300`, quindi memorizzare nella cache il set di chiavi e recuperarlo ogni cinque minuti è sicuro.

Verificate ogni token in arrivo rispetto a questi controlli:

<Steps>
  <Step title="Controllare il prefisso">
    Rifiutate il valore se non inizia con `sk-ant-cc-`, quindi rimuovete quel prefisso. Il resto è un JWT compatto standard.
  </Step>

  <Step title="Verificare la firma">
    Recuperate il JWKS, selezionate la chiave il cui `kid` corrisponde all'intestazione del token e verificate la firma `ES256`. Rifiutate i token il cui header `alg` non è `ES256`. Se un token arriva con un `kid` che non è nel vostro set di chiavi memorizzato nella cache, recuperate il JWKS una volta prima di rifiutarlo: dopo una rotazione, i nuovi token sono firmati con una chiave che il vostro set memorizzato nella cache non ha ancora.
  </Step>

  <Step title="Verificare l'emittente">
    Rifiutate il token se `iss` non è esattamente `ccr`.
  </Step>

  <Step title="Verificare il pubblico rispetto al vostro ambiente">
    Il claim `aud` è un array. Rifiutate il token a meno che non contenga l'ID del vostro ambiente, che ha la forma `ccpool_...`. L'ID dell'ambiente è mostrato nella finestra di dialogo dei dettagli del vostro ambiente nella [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments) e appare come il claim `ccr:pool_id` in qualsiasi token di sessione dell'ambiente. Questo controllo è quello che limita il token al vostro ambiente e rifiuta i token emessi per altre organizzazioni.
  </Step>

  <Step title="Verificare il ruolo">
    Rifiutate il token se `ccr:role` non è esattamente `session_worker`. Altri token emessi per ambienti self-hosted, come i segreti dell'ambiente, i token del runner e gli ordini di lavoro, sono firmati dallo stesso set di chiavi ma portano ruoli diversi.
  </Step>

  <Step title="Verificare la scadenza">
    Rifiutate il token se `exp` è nel passato. Anthropic emette i token di sessione con una durata di vita di quattro ore per impostazione predefinita e un massimo di otto ore. Il runner aggiorna il token prima della scadenza e invia il nuovo valore alla sessione, quindi i sottoprocessi che Claude avvia dopo un aggiornamento lo ereditano. Una sessione può quindi presentare diversi token validi distinti al vostro servizio nel corso della sua durata.
  </Step>

  <Step title="Leggere l'identità">
    L'identità dell'utente che crea è nel claim `act`: `act.sub` è il suo ID utente Anthropic nella forma con prefisso `user:<id>` e `act.email`, quando la superficie di creazione ne ha registrato uno, è il suo indirizzo email. Le sessioni che l'identità del servizio della vostra organizzazione crea, incluse le sessioni del canale Claude Tag, portano invece un soggetto `agent:`, quindi trattate una sessione come creata dall'utente solo quando `act.sub` porta il prefisso `user:`, piuttosto che testare se i claim di identità sono assenti. Consultare il [riferimento dei claim](#claims-reference) per la struttura completa e i claim duplicati piatti.
  </Step>
</Steps>

I controlli si mappano direttamente sulle librerie JWT standard. Gli esempi seguenti implementano la sequenza completa in Node.js con [`jose`](https://www.npmjs.com/package/jose), che gestisce il recupero JWKS, la memorizzazione nella cache e la selezione di `kid`, e in Python con [`PyJWT`](https://pyjwt.readthedocs.io/) e il suo client JWKS integrato.

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  Verificare il token all'interno della sessione
</h3>

Gli [script wrapper](/docs/it/self-hosted-environments-configuration#wrapper-scripts) vengono eseguiti all'interno della sessione, prima che Claude inizi. Invece di chiamare una libreria JWT, possono eseguire il sottocomando `self-hosted-runner decode-token` del binario del runner. Il sottocomando legge il token da un argomento posizionale, da `CLAUDE_CODE_SESSION_ACCESS_TOKEN` o da stdin piped, in quell'ordine, quindi rimuove il prefisso, verifica la firma rispetto all'endpoint JWKS, controlla la scadenza e stampa i claim come JSON. Il sottocomando esegue solo i controlli di firma e scadenza; non controlla `iss`, `aud` o `ccr:role`. Quando la decisione di autenticazione del vostro wrapper dipende da questi claim, leggete i claim dal JSON stampato e confrontateli esplicitamente.

Questo comando estrae l'identità del creatore, preferendo il soggetto del provider SSO, quindi l'indirizzo email, quindi il soggetto `act.sub` del creatore, `user:<id>` o `agent:<id>`:

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

I wrapper ricevono il percorso assoluto al binario del runner stesso in `CLAUDE_RUNNER_CLAUDE_BIN`; utilizzate quel percorso piuttosto che un `claude` risolto da PATH in modo che la decodifica venga eseguita sullo stesso binario che il runner stesso utilizza.

Utilizzate `jq -re` piuttosto che `jq -r` in modo che un claim mancante causi un'uscita diversa da zero. Con solo `-r`, un claim mancante stampa la stringa letterale `null` e esce con zero, il che passa silenziosamente un valore errato a valle. Passate `--no-verify` a `decode-token` solo per l'ispezione offline dove l'endpoint JWKS è irraggiungibile.

<h2 id="claims-reference">
  Riferimento dei claim
</h2>

La tabella seguente elenca i claim del token di sessione rilevanti per la verifica. Leggete l'identità dallo spazio dei nomi `ccr:*` e dalla catena `act`; i claim piatti `account_email`, `organization_uuid` e `account_uuid` sono duplicati di compatibilità all'indietro che potrebbero essere rimossi. Le sessioni che l'identità del servizio della vostra organizzazione crea, incluse le sessioni del canale Claude Tag, portano un soggetto `agent:` in `act.sub` e omettono `act.email`, `ccr:account_id`, `account_email` e `account_uuid`. I due claim di email sono facoltativi anche per le sessioni create dall'utente: Anthropic li registra al momento della creazione della sessione solo quando le credenziali della richiesta di creazione portano un'email, e una sessione inviata dalla CLI può mancare di entrambi, quindi basate l'identità su `act.sub` o `ccr:account_id` piuttosto che su email. I token possono anche portare claim aggiuntivi oltre questa tabella; ignorate i claim che non riconoscete.

| Claim               | Tipo            | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | string          | Sempre `ccr`.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sub`               | string          | `ccr:session:<session_id>`.                                                                                                                                                                                                                                                                                                                                                                                                       |
| `aud`               | array di string | Contiene sempre `anthropic-api`. Per le sessioni negli ambienti self-hosted l'array contiene anche l'ID del vostro ambiente, come `ccpool_...`. Verificate l'ID dell'ambiente, non `anthropic-api`.                                                                                                                                                                                                                               |
| `exp`               | number          | Scadenza come timestamp Unix. Durata di vita predefinita di quattro ore, massimo di otto ore.                                                                                                                                                                                                                                                                                                                                     |
| `iat`               | number          | Emesso come timestamp Unix.                                                                                                                                                                                                                                                                                                                                                                                                       |
| `jti`               | string          | Identificatore univoco del token.                                                                                                                                                                                                                                                                                                                                                                                                 |
| `ccr:role`          | string          | Sempre `session_worker` per i token di sessione.                                                                                                                                                                                                                                                                                                                                                                                  |
| `ccr:session_id`    | string          | L'ID della sessione. Stesso valore del suffisso di `sub`.                                                                                                                                                                                                                                                                                                                                                                         |
| `ccr:pool_id`       | string          | L'ID del vostro ambiente. Stesso valore che appare in `aud`.                                                                                                                                                                                                                                                                                                                                                                      |
| `ccr:org_id`        | string          | L'ID della vostra organizzazione Anthropic.                                                                                                                                                                                                                                                                                                                                                                                       |
| `ccr:account_id`    | string          | L'ID dell'account Anthropic dell'utente che crea: il valore di `act.sub` senza il prefisso `user:`, un ID taggato `user_...`. Lo stesso valore che il `CLAUDE_RUNNER_ACCOUNT_ID` dell'[hook spawn-runner](/docs/it/self-hosted-environments-configuration#the-spawn-runner-hook) porta e che [`--lock-to-account`](/docs/it/self-hosted-environments-reference#runner-cli-flags) accetta, quindi i tre si confrontano come stringhe uguali. |
| `account_email`     | string          | Duplicato di `act.email`; assente ogni volta che `act.email` lo è.                                                                                                                                                                                                                                                                                                                                                                |
| `organization_uuid` | string          | L'UUID della vostra organizzazione Anthropic.                                                                                                                                                                                                                                                                                                                                                                                     |
| `account_uuid`      | string          | L'UUID dell'account Anthropic dell'utente che crea.                                                                                                                                                                                                                                                                                                                                                                               |
| `act`               | object          | Catena di delega [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693). Consultare [La catena `act`](#the-act-chain).                                                                                                                                                                                                                                                                                                                |

<h3 id="the-act-chain">
  La catena `act`
</h3>

Il claim `act` registra il percorso di delega completo dall'identità dell'utente o del servizio che ha creato la sessione fino all'[ambiente](/docs/it/self-hosted-environments#key-concepts) il cui segreto ha ammesso il runner e l'identità che ha creato quel segreto. Il creatore è l'attore più esterno, quindi `act.sub` li identifica direttamente.

| Percorso          | Descrizione                                                                                                                                                                                                                                                             |
| :---------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `act.sub`         | L'ID utente Anthropic dell'utente che crea, nella forma `user:<id>`, o `agent:<id>` quando l'identità del servizio della vostra organizzazione ha creato la sessione, come fa per le sessioni del canale Claude Tag.                                                    |
| `act.email`       | L'indirizzo email dell'utente che crea, quando uno è stato registrato al momento della creazione della sessione. Non lo richiedete; basate su `act.sub`.                                                                                                                |
| `act.attested_by` | L'attestazione del provider di identità upstream per l'utente che crea, quando disponibile. `act.attested_by.sub` è il soggetto che il vostro provider SSO, come Google o Okta, ha emesso. Preferite questo a `act.email` quando mappate a identità nei vostri sistemi. |
| `act.act`         | Il runner che ha generato la sessione. `act.act.sub` è `ccr:runner:<runner_id>`.                                                                                                                                                                                        |
| `act.act.act`     | L'ambiente. `act.act.act.sub` è `ccr:pool:<pool_id>`.                                                                                                                                                                                                                   |
| `act.act.act.act` | L'identità che ha creato il segreto dell'ambiente con cui il runner si è registrato. La catena termina qui.                                                                                                                                                             |

<h2 id="scope-derived-credentials">
  Scope derived credentials
</h2>

Il token di sessione identifica l'utente o l'identità del servizio che ha creato la sessione, ma non lo trattate come equivalente a quel creatore che accede direttamente. Il token si trova in una variabile di ambiente all'interno della sessione, quindi qualsiasi codice che Claude esegue e qualsiasi tool o server MCP che la sessione avvia può leggerlo e presentarlo.

La verifica è anche offline: un token che verifica rispetto al JWKS rimane valido fino al suo `exp`, qualunque cosa sia accaduta alla sessione da allora, e Anthropic non pubblica un feed di revoca per i token di sessione. Limitate qualsiasi cosa deriviate dal token di conseguenza.

Quando il vostro servizio scambia il token per credenziali interne, emettete credenziali limitate a quello che una sessione di codifica dovrebbe raggiungere:

* **Limitate le capacità**: concedete accesso in lettura e scrittura alle risorse di cui la sessione ha bisogno per i compiti di codifica, non alle capacità amministrative che il creatore detiene altrove.
* **Limitate la durata**: limitate le credenziali derivate al `exp` del token, o più breve.
* **Controllate come la sessione**: registrate `ccr:session_id` e `jti` insieme all'identità del creatore in modo da poter tracciare le azioni di nuovo a una sessione specifica.

<h2 id="related-environment-variables">
  Variabili di ambiente correlate
</h2>

L'identità del creatore appare anche in variabili di ambiente semplici su due superfici che non verificano mai il token:

* **L'[hook `spawn-runner`](/docs/it/self-hosted-environments-configuration#the-spawn-runner-hook), sull'orchestratore**: l'hook viene eseguito prima che esista un runner per una sessione in coda e riceve l'identità del creatore in variabili come `CLAUDE_RUNNER_ACCOUNT_EMAIL` e `CLAUDE_RUNNER_ACCOUNT_ID`. L'orchestratore le legge dall'ordine di lavoro, il token firmato monouso che autorizza l'avvio di un runner, senza verificare la firma dell'ordine di lavoro stesso; i claim sono attendibili perché l'ordine di lavoro arriva sulla connessione dell'orchestratore ad Anthropic, che il segreto dell'ambiente autentica.
* **[Script wrapper](/docs/it/self-hosted-environments-configuration#wrapper-scripts), all'interno della sessione**: i wrapper ricevono `CCR_SESSION_ACCOUNT_EMAIL`, l'email del creatore pre-estratta dal token senza verifica della firma. La variabile è adatta per l'etichettatura, come i trailer di commit, non per le decisioni di autenticazione.

Utilizzate le variabili semplici per le decisioni lato orchestratore come la selezione di un'immagine di macchina. Utilizzate `CLAUDE_CODE_SESSION_ACCESS_TOKEN` quando un servizio a valle ha bisogno di una prova crittografica indipendente piuttosto che fidarsi dell'ambiente del runner.

<h2 id="what’s-next">
  Cosa c'è dopo
</h2>

* [Self-hosted environments](/docs/it/self-hosted-environments): l'ambiente, il runner e il modello di sessione; la [guida rapida](/docs/it/self-hosted-environments-quickstart) e [Deploy to production](/docs/it/self-hosted-environments-deploy) contengono la configurazione e le operazioni
* [Customize sessions](/docs/it/self-hosted-environments-configuration): script wrapper che consumano il token e l'hook `spawn-runner`
* [Reference](/docs/it/self-hosted-environments-reference): flag CLI, variabili di ambiente e metriche
