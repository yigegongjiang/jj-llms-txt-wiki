> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Limiti di spesa del gateway delle app Claude

> Limita la spesa di ogni sviluppatore attraverso il gateway delle app Claude per giorno, settimana o mese. Imposta i limiti con un'API Admin e il gateway li applica in tempo reale su ogni richiesta.

I limiti di spesa limitano quanto ogni sviluppatore può spendere attraverso il tuo [gateway delle app Claude](/docs/it/claude-apps-gateway) in un determinato giorno, settimana o mese. Quando uno sviluppatore supera il suo limite, il gateway restituisce `429` alla sua prossima richiesta e lo blocca fino al ripristino del periodo o fino a quando un amministratore non aumenta il limite. Utilizza i limiti di spesa per dare a ogni sviluppatore, gruppo o all'intera organizzazione un tetto massimo su una credenziale che tutti condividono.

Un gateway delle app Claude inoltra tutta l'inferenza attraverso una credenziale upstream condivisa, quindi la fattura del tuo provider attribuisce tutto a quella credenziale, non ai singoli sviluppatori. Senza limiti per sviluppatore, una flotta di agenti incontrollata può spendere l'intero impegno dell'organizzazione. I limiti di spesa sono la vista per sviluppatore del gateway e il circuito di protezione su quella fattura condivisa.

<h2 id="set-a-cap">
  Imposta un limite
</h2>

Con il blocco [`admin:`](/docs/it/claude-apps-gateway-config#admin) configurato in `gateway.yaml`, il gateway serve un Admin API su `/v1/organizations/spend_limits` e applica i limiti in tempo reale su ogni richiesta di inferenza. I limiti stessi vengono impostati attraverso quell'API, non in `gateway.yaml`; ogni richiesta `POST /v1/organizations/spend_limits` crea o sostituisce un limite da `{scope, amount, period}`. L'API rispecchia le forme dei dati degli endpoint dei limiti di spesa dell'[Admin API](https://platform.claude.com/docs/en/manage-claude/admin-api) pubblico di Anthropic, quindi un client HTTP scritto per quel contratto può indirizzarsi al gateway modificando il suo URL di base.

Questa richiesta imposta un valore predefinito a livello di organizzazione di \$500 al mese per ogni sviluppatore:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

Questa richiesta aggiunge un limite più restrittivo di \$100 al giorno per ogni membro del gruppo `contractors`:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| Campo        | Valori                                              | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------ | --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope.type` | `user`, `rbac_group`, `organization`                | `user` indirizza uno sviluppatore tramite il suo OpenID Connect (OIDC) `sub`, l'ID utente stabile assegnato dal tuo provider di identità; passalo come `scope.user_id`. `rbac_group` indirizza un [gruppo IdP](/docs/it/claude-apps-gateway-config#managed) per nome; passalo come `scope.rbac_group_id`. `organization` è il valore predefinito a livello di organizzazione. Il gateway accetta tutti e tre; l'`POST` pubblico di Anthropic è solo per utenti oggi. |
| `amount`     | Stringa di numero intero di centesimi USD, o `null` | `null` è illimitato. `"0"` è un limite zero, che blocca ogni richiesta.                                                                                                                                                                                                                                                                                                                                                                                         |
| `period`     | `daily`, `weekly`, `monthly`                        | Un ambito può contenere un limite per periodo, e ognuno si applica indipendentemente: uno sviluppatore è bloccato se supera uno qualsiasi di essi.                                                                                                                                                                                                                                                                                                              |

Un limite di gruppo o organizzazione è un valore predefinito per posto che ogni membro eredita, non un pool condiviso. Per periodo, il limite effettivo di uno sviluppatore si risolve in questo ordine: un override per utente, quindi il più restrittivo dei suoi limiti di gruppo, quindi il valore predefinito dell'organizzazione, quindi illimitato. [`admin.group_limit_mode: max`](/docs/it/claude-apps-gateway-config#admin) capovolge il tie-break multi-gruppo al meno restrittivo invece.

<h3 id="authenticate-to-the-admin-api">
  Autentica all'Admin API
</h3>

Invia uno dei seguenti:

* Un header `x-api-key` che corrisponde a una chiave in [`admin.write_keys`](/docs/it/claude-apps-gateway-config#admin) per accesso completo, o `admin.read_keys` per accesso di sola lettura con `GET`. Ogni chiave porta un `id` che appare nel registro di audit come `admin-key:<id>`, quindi assegna a Terraform, CI e a ogni automazione la propria.
* Un token bearer del gateway il cui claim `groups` include uno dei [`admin.admin_groups`](/docs/it/claude-apps-gateway-config#admin). Questo è accesso completo e viene registrato come `oidc:<sub>`, quindi preferiscilo per gli amministratori umani.

<h2 id="how-enforcement-works">
  Come funziona l'applicazione
</h2>

Su ogni richiesta `/v1/messages`, il gateway risolve i limiti dello sviluppatore e la spesa da inizio periodo in una query Postgres. Se superano uno qualsiasi dei limiti, la richiesta restituisce `429` con `error.type: billing_error` e l'header `x-should-retry: false`. Il messaggio è `spend limit reached`, seguito dal tuo [`admin.blocked_message`](/docs/it/claude-apps-gateway-config#admin) se impostato.

`/v1/messages/count_tokens` è esente. Il conteggio dei token è gratuito, quindi viene eseguito indipendentemente dallo stato del limite.

Dopo ogni risposta, un misuratore di utilizzo legge i conteggi dei token dalla risposta mentre viene trasmessa al client, li prezza al prezzo di listino USD e incrementa i contatori Postgres per tutti e tre i bucket di periodo. Il misuratore è un singolo lettore sul flusso, quindi i byte del client rimangono intatti e un errore di misurazione non interrompe la risposta.

I limiti di spesa stimano la spesa dai conteggi dei token al prezzo di listino USD; sono un circuito di protezione, non una fattura. Per la fatturazione autorevole, riconcilia con il rapporto di utilizzo del tuo provider, come l'Admin API di Utilizzo e Costi di Anthropic, i log di invocazione su Bedrock, o Cloud Monitoring su Google Cloud.

I prezzi utilizzano la stessa tabella che Claude Code CLI utilizza per il proprio display dei costi, con la stessa canonicalizzazione dell'ID modello tra Anthropic, Bedrock (`us.anthropic.…-v1:0`), Agent Platform (`claude-…@date`), e forme di Foundry ID. Un ID modello che la tabella non riesce a posizionare, come un nome di distribuzione Foundry o un ARN del profilo di inferenza, viene prezzato al livello predefinito del modello sconosciuto di \$5/\$25 per milione di token di input/output piuttosto che zero, quindi un ID non riconosciuto non può aggirare un limite andando non misurato. Il gateway avvisa all'avvio e una volta per ID al runtime quando un modello viene prezzato attraverso il fallback.

Gli abort dei client vengono fatturati anche. L'upstream segnala i token di output solo nel frame terminale del flusso, quindi un flusso interrotto non li contiene. Il misuratore mantiene una stima conservativa del pavimento dal contenuto trasmesso, circa quattro caratteri per token, e la fattura quando e solo quando il frame di utilizzo terminale è mancante. Un flusso completo fattura sempre il conteggio segnalato dall'upstream. Senza questo, uno sviluppatore limitato potrebbe trasmettere l'output e interrompere ogni richiesta immediatamente prima della fine, spendendo senza mai essere conteggiato.

<h3 id="postgres-availability">
  Disponibilità di Postgres
</h3>

La pre-verifica interroga Postgres con un timeout di due secondi. Se l'archivio è irraggiungibile o scade, l'applicazione fallisce aperta per impostazione predefinita: la richiesta procede e il gateway registra un avviso. Imposta [`enforcement.fail_closed_on_error: true`](/docs/it/claude-apps-gateway-config#enforcement) per fallire chiuso invece, che restituisce lo stesso `429 billing_error` con il messaggio `spend limit unavailable`. Fail-open impedisce a un'interruzione dell'archivio di diventare un'interruzione dell'inferenza; fail-closed garantisce nessuna spesa non misurata.

<h2 id="admin-api-reference">
  Riferimento Admin API
</h2>

Gli endpoint di seguito vengono serviti sotto `/v1/organizations/spend_limits`.

| Metodo e percorso                              | Descrizione                                                                   |
| ---------------------------------------------- | ----------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Elenca i limiti configurati. Query: `?limit=&after_id=&before_id=`.           |
| `POST /v1/organizations/spend_limits`          | Crea o sostituisce un limite per `{scope, period}`.                           |
| `GET /v1/organizations/spend_limits/{id}`      | Recupera un limite per il suo ID con prefisso `spl_`.                         |
| `DELETE /v1/organizations/spend_limits/{id}`   | Elimina un limite. Restituisce `{type: "spend_limit_deleted", id}`.           |
| `GET /v1/organizations/spend_limits/effective` | Limite risolto e spesa da inizio periodo per principale per periodo.          |
| `GET /v1/organizations/spend_limits/audit`     | Traccia di mutazione amministrativa, più recente per primo. Query: `?limit=`. |

Le convenzioni rispecchiano l'Admin API di Anthropic:

* Un `type` su ogni oggetto
* ID con prefisso `spl_`
* Importi come stringhe di numero intero di centesimi USD; `POST` rifiuta qualsiasi altra `currency` con `400`
* L'envelope di errore `{type: "error", error: {type, message}, request_id}`
* Un header di risposta `request-id` su ogni risposta amministrativa, successo o errore, che corrisponde al `request_id` del corpo

Ogni mutazione scrive una riga prima/dopo a `admin_audit` nella stessa transazione, attribuita a `admin-key:<id>` o `oidc:<sub>`.

Il gateway serve gli endpoint dei limiti di spesa solo. Altre superfici Admin API, come la coda `spend_limit_increase_requests`, non fanno parte dell'Admin API del gateway.

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective` restituisce lo schema `SpendSummary` di Anthropic: ogni riga è un principale per un periodo, con il limite risolto, la spesa da inizio periodo e un oggetto `actor`. Differenze specifiche del gateway:

* `user_id` è l'OIDC `sub`.
* `actor.name` e `actor.email_address` sono `null` fino alla prima richiesta di inferenza del principale attraverso il gateway. Il gateway non ha una directory utenti; registra i valori ultimi visti dal JWT di sessione di ogni utente.
* Ogni riga contiene anche un array `groups`, gli ultimi gruppi IdP visti dal principale. Questa è un'estensione del gateway in modo che un'interfaccia utente amministrativa possa mostrare ogni livello di limite che si applica; i client a forma di Anthropic lo ignorano.
* Senza un filtro `user_ids[]`, elenca i principali con spesa registrata, perché il gateway non può enumerare tutti i membri dell'organizzazione.

I limiti di origine del gruppo si risolvono rispetto a quei gruppi ultimi visti con lo stesso tie-break `group_limit_mode` che l'applicazione utilizza, quindi il visualizzatore mostra il limite che effettivamente si applica.

| Parametro di query | Descrizione                                                                                                                              |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `user_ids[]`       | Ripetibile. Filtra a principali specifici per OIDC `sub`.                                                                                |
| `period[]`         | Ripetibile. Filtra a righe `daily`, `weekly`, o `monthly`.                                                                               |
| `sort`             | `spend_desc` elenca i maggiori spenditori per primi. Richiede esattamente uno `period[]`.                                                |
| `q`                | Filtro di sottostringa senza distinzione tra maiuscole e minuscole sull'OIDC `sub`, email ultimi visti e nome visualizzato ultimi visti. |
| `limit` / `page`   | Dimensione della pagina (1–1000, predefinito 20) e il cursore opaco dalla risposta precedente `next_page`.                               |

<Warning>
  `q=` e `user_ids[]=` vanno nelle stringhe di query GET, quindi qualsiasi proxy di fronting o load balancer li cattura nei suoi log di accesso. Se la tua politica di log PII è ristretta, pulisci questi parametri lì.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

Restituisce la traccia di mutazione del limite di spesa: chi ha modificato quale limite, snapshot prima/dopo e il motivo facoltativo, più recente per primo. `has_more` è esatto. Questo endpoint segue le convenzioni dell'Admin API locale piuttosto che una forma di dati di prima parte.

<h3 id="pagination">
  Paginazione
</h3>

L'elenco grezzo pagina per `after_id` e `before_id`, che sono ID `spl_…` mutuamente esclusivi; i risultati sono ordinati per creazione e `has_more` riflette la direzione di attraversamento. `/effective` pagina per il token opaco `next_page` passato indietro come `?page=`, con principali ordinati in modo ascendente in modo che le pagine rimangono stabili mentre la spesa viene registrata. `limit` è 1–1000, predefinito 20, su entrambi.

<h2 id="data-lifecycle">
  Ciclo di vita dei dati
</h2>

Il gateway contiene quattro tabelle relative alla spesa; una pulizia oraria applica le finestre di conservazione:

| Tabella            | Contenuti                                                                            | Conservazione                                                                                                |
| ------------------ | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ |
| `spend`            | Contatori da inizio periodo per principale in centesimi                              | [`admin.spend_retention_months`](/docs/it/claude-apps-gateway-config#admin), predefinito 13                       |
| `spend_limits`     | I limiti configurati                                                                 | Fino all'eliminazione tramite l'API                                                                          |
| `admin_audit`      | La traccia di mutazione                                                              | [`admin.audit_retention_days`](/docs/it/claude-apps-gateway-config#admin), predefinito 365                        |
| `principal_emails` | Email ultimi visti di ogni principale, nome visualizzato e gruppi IdP. Contiene PII. | [`admin.identity_retention_days`](/docs/it/claude-apps-gateway-config#admin) dall'ultima attività, predefinito 90 |

`identity_retention_days` è deliberatamente più breve di `spend_retention_months`: un'identità deprovisioning smette di aggiornarsi e invecchia, mentre i suoi contatori di spesa anonimi rimangono per il rapporto anno su anno.

Quando uno sviluppatore se ne va, elimina qualsiasi limite per utente tramite `DELETE /v1/organizations/spend_limits/{id}`; la loro spesa e le righe di identità invecchiano sulle finestre di conservazione di cui sopra. Per cancellare una persona immediatamente, per offboarding o una richiesta di accesso ai dati del soggetto (DSAR), esegui `DELETE FROM principal_emails WHERE principal = '<sub>'` direttamente contro il database del gateway. Questo rimuove l'unica tabella che contiene la loro email, nome e gruppi. Le righe `spend` e `admin_audit` fanno riferimento solo all'OIDC `sub` pseudonimo e invecchiano sulle loro finestre.

<h2 id="related">
  Correlati
</h2>

* [Configurazione `admin` e `enforcement`](/docs/it/claude-apps-gateway-config#admin): abilitazione dell'Admin API e ottimizzazione della conservazione
* [Guida di distribuzione](/docs/it/claude-apps-gateway-deploy#postgres): schema Postgres e guida al backup
