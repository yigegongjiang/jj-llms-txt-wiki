> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare le impostazioni gestite dal server

> Configurare centralmente Claude Code per la vostra organizzazione tramite impostazioni consegnate dal server, senza richiedere infrastrutture di gestione dei dispositivi.

Le impostazioni gestite dal server consentono ai proprietari dell'organizzazione di configurare centralmente Claude Code da [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) nella console claude.ai. I client di Claude Code recuperano automaticamente queste impostazioni quando gli utenti si autenticano con un accesso idoneo su una piattaforma dove la consegna gestita dal server è supportata. Vedere [Disponibilità della piattaforma](#platform-availability) per le credenziali e le piattaforme che si qualificano.

<Note>
  Le impostazioni gestite dal server sono disponibili per i clienti di [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) e [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise).
</Note>

<h2 id="requirements">
  Requisiti
</h2>

Per utilizzare le impostazioni gestite dal server, è necessario:

* Piano Claude for Teams o Claude for Enterprise
* Il ruolo di Owner o Primary Owner nella vostra organizzazione Claude, per visualizzare e modificare la configurazione
* Accesso di rete a `api.anthropic.com`

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  Scegliere tra impostazioni gestite dal server e gestite dall'endpoint
</h2>

Claude Code supporta due approcci per la configurazione centralizzata. Le impostazioni gestite dal server forniscono la configurazione dai server di Anthropic. Le [impostazioni gestite dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) vengono distribuite direttamente ai dispositivi tramite criteri nativi del sistema operativo (preferenze gestite macOS, registro Windows) o file di impostazioni gestiti.

| Approccio                                                                          | Ideale per                                                    | Modello di sicurezza                                                                                                                |
| :--------------------------------------------------------------------------------- | :------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| **Impostazioni gestite dal server**                                                | Organizzazioni senza MDM, o utenti su dispositivi non gestiti | Impostazioni che Claude Code recupera dai server di Anthropic all'avvio e aggiorna ogni ora durante la sessione                     |
| **[Impostazioni gestite dall'endpoint](/docs/it/managed-settings#delivery-mechanisms)** | Organizzazioni con MDM o gestione degli endpoint              | Impostazioni distribuite ai dispositivi tramite profili di configurazione MDM, criteri del registro, o file di impostazioni gestiti |

Se i vostri dispositivi sono registrati in una soluzione MDM o di gestione degli endpoint, le impostazioni gestite dall'endpoint forniscono garanzie di sicurezza più forti perché il file di impostazioni può essere protetto dalla modifica dell'utente a livello del sistema operativo. Le impostazioni gestite dall'endpoint non raggiungono le [sessioni cloud](/docs/it/model-config#surface-coverage) negli ambienti ospitati da Anthropic, quindi le organizzazioni i cui sviluppatori eseguono sessioni cloud dovrebbero configurare anche le impostazioni gestite dal server. Le sessioni in un [ambiente self-hosted](/docs/it/self-hosted-environments) leggono anche il file di impostazioni gestite nell'immagine del runner. La [precedenza delle impostazioni](#settings-precedence) di seguito indica quando quel file si applica.

<h2 id="configure-server-managed-settings">
  Configurare le impostazioni gestite dal server
</h2>

<Steps>
  <Step title="Aprire la console di amministrazione">
    Nella console claude.ai, andare a [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Se il collegamento vi reindirizza a una pagina Admin Settings diversa invece della pagina Claude Code, il vostro account non dispone del ruolo richiesto. I ruoli Admin e altri ruoli non-Owner non possono visualizzare o modificare le impostazioni gestite, quindi chiedete a un Owner o Primary Owner nella vostra organizzazione di apportare la modifica. Vedere [Controllo di accesso](#access-control).
  </Step>

  <Step title="Definire le impostazioni">
    Aggiungere la configurazione come JSON. Tutte le [impostazioni disponibili in `settings.json`](/docs/it/settings-reference#all-settings) sono supportate eccetto quelle limitate alla distribuzione delle politiche a livello del sistema operativo; vedere [Limitazioni attuali](#current-limitations) per questo breve elenco. Questo include [hooks](/docs/it/hooks), [variabili di ambiente](/docs/it/env-vars), e [impostazioni solo gestite](/docs/it/managed-settings#managed-only-settings) come `allowManagedPermissionRulesOnly`.

    Questo esempio applica un elenco di negazione delle autorizzazioni, impedisce agli utenti di ignorare le autorizzazioni e limita le regole di autorizzazione a quelle definite nelle impostazioni gestite. La regola `Bash(curl *)` corrisponde a `curl` [come Claude lo scrive](/docs/it/permissions#bash-rule-limits), non a `/usr/bin/curl` o `sh -c 'curl …'`; per l'applicazione della rete che non dipende dal testo del comando, aggiungere un [blocco `sandbox` con `allowManagedDomainsOnly`](/docs/it/sandboxing#configure-the-sandbox-for-your-organization).

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Gli hook utilizzano lo stesso formato di `settings.json`.

    Questo esempio esegue uno script di audit dopo ogni modifica di file in tutta l'organizzazione:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    Poiché gli hook eseguono comandi shell, gli utenti in sessioni interattive vedono una [finestra di dialogo di approvazione della sicurezza](#security-approval-dialogs) prima che Claude Code li applichi.

    Per configurare il classificatore della [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) in modo che conosca quali repository, bucket e domini la vostra organizzazione ritiene affidabili, fornire un blocco `autoMode` nello stesso modo; vedere [Configurare la modalità auto](/docs/it/auto-mode-config) per come le voci `autoMode` influenzano ciò che il classificatore blocca e avvertimenti importanti sui campi `environment`, `allow`, `soft_deny` e `hard_deny`.
  </Step>

  <Step title="Salvare e distribuire">
    Salvare le modifiche. I client di Claude Code ricevono le impostazioni aggiornate al prossimo avvio o ciclo di polling orario.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verificare la consegna delle impostazioni
</h3>

Per confermare che le impostazioni vengono applicate, chiedere a un utente di riavviare Claude Code. Se la configurazione include impostazioni che attivano la [finestra di dialogo di approvazione della sicurezza](#security-approval-dialogs), l'utente vede un prompt che descrive le impostazioni gestite la prossima volta che Claude Code le recupera: al prossimo avvio, o entro un'ora in una sessione interattiva in esecuzione. È inoltre possibile verificare che le regole di autorizzazione gestite siano attive facendo eseguire a un utente `/permissions` per visualizzare le regole di autorizzazione effettive.

Per verificare il risultato del recupero su una macchina specifica, fare in modo che l'utente esegua `claude doctor` e legga la riga `Managed settings (remote)`. Richiede Claude Code v2.1.248 o successivo. La riga segnala uno di quattro risultati:

* Le impostazioni consegnate sono state caricate
* La vostra organizzazione non ha impostazioni gestite dal server configurate
* Il recupero non è riuscito, con la causa e se una politica memorizzata nella cache si applica ancora
* Claude Code ha saltato il recupero, con il motivo. Vedere [Disponibilità della piattaforma](#platform-availability) per i provider e le configurazioni che lo saltano

Mentre il recupero è ancora in corso, la riga segnala questo invece.

In una sessione in esecuzione, `/status` mostra la stessa riga dopo un recupero non riuscito, e per alcune cause di recupero saltato, come una variabile del provider di terze parti o un `ANTHROPIC_BASE_URL` personalizzato esportato nella shell dell'utente.

<h3 id="access-control">
  Controllo di accesso
</h3>

I seguenti ruoli possono gestire le impostazioni gestite dal server:

* **Primary Owner**
* **Owner**

Limitare l'accesso al personale di fiducia, poiché le modifiche alle impostazioni si applicano a tutti gli utenti dell'organizzazione.

<h3 id="managed-only-settings">
  Impostazioni solo gestite
</h3>

La maggior parte delle [chiavi di impostazioni](/docs/it/settings-reference#all-settings) funzionano in qualsiasi ambito. Un numero limitato di chiavi viene letto solo dalle impostazioni gestite e non ha alcun effetto quando posizionato nei file di impostazioni dell'utente o del progetto. Vedere [impostazioni solo gestite](/docs/it/managed-settings#managed-only-settings) per i controlli di autorizzazione e plugin, o leggere la colonna Scope dell'indice [Tutte le impostazioni](/docs/it/settings-reference#all-settings) per l'insieme completo.

<h3 id="current-limitations">
  Limitazioni attuali
</h3>

Le impostazioni gestite dal server hanno le seguenti limitazioni:

* Le impostazioni si applicano uniformemente a tutti gli utenti dell'organizzazione. Le configurazioni per gruppo non sono ancora supportate.
* Non è possibile distribuire un file [`managed-mcp.json`](/docs/it/managed-mcp) tramite impostazioni gestite dal server. Distribuire invece le chiavi di politica `allowedMcpServers` e `deniedMcpServers` lì. Su Claude Code v2.1.259 o successivo, è inoltre possibile fornire server remoti con [`managedMcpServers`](/docs/it/managed-mcp#provide-servers-through-managed-settings), che accetta solo server `http` e `sse` e non assume il controllo esclusivo come fa il file.

  Claude Code legge un file `managed-mcp.json` distribuito nel suo [percorso di sistema](/docs/it/managed-mcp#exclusive-control-with-managed-mcp-json) separatamente dal livello delle impostazioni gestite, quindi il file si applica ancora quando le impostazioni gestite dal server sono in vigore.
* Le impostazioni limitate alle fonti delle politiche a livello del sistema operativo, come `policyHelper` e `wslInheritsWindowsSettings`, non vengono rispettate. Distribuirle invece tramite MDM o un file `managed-settings.json` di sistema. Un `policyHelper` distribuito in questo modo viene eseguito solo quando la sua fonte è quella selezionata in [precedenza all'interno del livello gestito](/docs/it/managed-settings#precedence-within-the-managed-tier).

<h2 id="settings-delivery">
  Consegna delle impostazioni
</h2>

<h3 id="settings-precedence">
  Precedenza delle impostazioni
</h3>

Le impostazioni gestite dal server e le [impostazioni gestite dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) occupano entrambe il livello più alto nella [gerarchia delle impostazioni](/docs/it/settings#settings-precedence) di Claude Code. Nessun altro livello di impostazioni può sostituirle, inclusi gli argomenti della riga di comando, ad eccezione delle [eccezioni alla precedenza delle impostazioni gestite](/docs/it/settings#exceptions-to-managed-settings-precedence).

All'interno del livello gestito, Claude Code per impostazione predefinita utilizza la prima fonte che fornisce almeno una chiave di politica, controllando prima le impostazioni gestite dal server e poi le impostazioni gestite dall'endpoint, ad eccezione delle [chiavi di eccezione coperte di seguito](#per-key-exceptions-across-managed-sources). [Come Claude Code combina le fonti gestite](/docs/it/managed-settings#precedence-within-the-managed-tier) contiene la classificazione completa, l'eccezione per le chiavi di controllo e l'opt-in che si applica a ogni fonte.

Se la fonte selezionata è una politica MDM o un file di impostazioni gestite il cui [`policyHelper`](/docs/it/settings-reference#policyhelper) fornisce impostazioni gestite, l'output dell'helper sostituisce quella fonte come l'unica configurazione gestita per l'esecuzione. Claude Code non consulta un `policyHelper` configurato in MDM o impostazioni basate su file mentre le impostazioni gestite dal server forniscono una chiave di politica.

Se un recupero successivo trova le impostazioni gestite dal server rimosse, Claude Code esegue immediatamente quell'helper invece che al prossimo avvio. La voce [`policyHelper`](/docs/it/settings-reference#policyhelper) copre cosa succede quando quell'esecuzione non riesce.

Se cancellate la configurazione gestita dal server nella console di amministrazione con l'intenzione di tornare a una plist gestita dall'endpoint o a una politica del registro, tenete presente che le [impostazioni memorizzate nella cache](#fetch-and-caching-behavior) persistono sulle macchine client fino al prossimo recupero riuscito, e le chiavi che [si applicano solo al prossimo avvio](#fetch-and-caching-behavior), come `model`, rimangono in vigore fino a quando ogni client non si riavvia. Eseguite `/status` per vedere quale fonte gestita è attiva.

<h3 id="per-key-exceptions-across-managed-sources">
  Eccezioni per chiave tra fonti gestite
</h3>

Tre tipi di chiavi sono eccezioni alla regola di non-unione:

* **Chiavi di blocco tra fonti**: un piccolo insieme di chiavi, come i blocchi della lista di autorizzazione della sandbox, [elencate nella pagina delle impostazioni gestite](/docs/it/managed-settings#precedence-within-the-managed-tier). Claude Code le rispetta quando qualsiasi fonte gestita controllata dall'amministratore le imposta; il livello del registro HKCU scrivibile dall'utente è escluso.

  Quando un [`policyHelper`](/docs/it/settings-reference#policyhelper) fornisce impostazioni gestite, il suo output è l'unica fonte che questi controlli leggono, ad eccezione di [`forceRemoteSettingsRefresh`](/docs/it/settings-reference#forceremotesettingsrefresh), che Claude Code legge direttamente dalle fonti amministrative all'avvio.
* **Il blocco `env`**: a parte l'unità di telemetria e le variabili di routing associate a una chiave di credenziale, entrambe coperte di seguito, si unisce per chiave tra le fonti controllate dall'amministratore. Per ogni variabile di ambiente, la fonte con la priorità più alta che la definisce vince, e le fonti amministrative inferiori riempiono le variabili che le fonti superiori lasciano non impostate. Una voce `env` gestita dall'endpoint si applica quindi ogni volta che la configurazione gestita dal server lascia quella variabile non impostata, o mentre un valore server memorizzato nella cache per essa è [trattenuto in sospeso della conferma del server](#fetch-and-caching-behavior). Richiede Claude Code v2.1.223 o successivo. Prima della v2.1.223, Claude Code applica solo il blocco `env` della fonte selezionata.
  * **Unità di telemetria**: le chiavi dell'esportatore `OTEL_EXPORTER_OTLP_*`, gli interruttori di acquisizione del contenuto `OTEL_LOG_*`, `OTEL_LOGS_EXPORTER` e le variabili di tracciamento beta `ENABLE_BETA_TRACING_DETAILED` e `BETA_TRACING_ENDPOINT` seguono la fonte più alta che imposta una qualsiasi di esse come unità. Una fonte che fornisce la chiave di credenziale `otelHeadersHelper` rivendica l'unità anche, ma fornisce queste variabili solo quando è la fonte selezionata: una fonte che non è selezionata ma fornisce la chiave non contribuisce a nessuna di esse e blocca comunque le fonti inferiori dal riempirle. In ogni caso, un endpoint dell'esportatore da una fonte non può mai essere associato a credenziali da un'altra.
  * **Routing associato a credenziale**: una fonte che associa variabili di routing a una chiave di credenziale solo per la fonte selezionata, come `apiKeyHelper` o `otelHeadersHelper`, contribuisce a quelle variabili di routing solo quando vince lo slot.
* **Chiavi di accesso al gateway**: Claude Code non legge mai [`forceLoginGatewayUrl`](/docs/it/settings-reference#forcelogingatewayurl) o il valore `"gateway"` di [`forceLoginMethod`](/docs/it/settings-reference#forceloginmethod) dalle impostazioni gestite dal server, quindi selezionare le impostazioni gestite dal server non fornisce un accesso al gateway né nasconde uno impostato in una politica MDM o file di impostazioni gestite. La voce [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior) dice quale fonte amministrativa sulla macchina le fornisce.

<h3 id="fetch-and-caching-behavior">
  Comportamento di recupero e caching
</h3>

Claude Code recupera le impostazioni dai server di Anthropic all'avvio e esegue il polling per gli aggiornamenti ogni ora durante le sessioni attive.

Un client che ha effettuato l'accesso tramite un [gateway di app Claude](#platform-availability) recupera le impostazioni dal gateway e attende quel recupero prima che la sessione inizi, quindi il recupero negli elenchi di seguito non si applica ad esso. [Applicare l'avvio fail-closed](#enforce-fail-closed-startup) copre cosa succede quando quel recupero non riesce.

**Primo avvio senza impostazioni memorizzate nella cache:**

* Quando uno sviluppatore effettua l'accesso all'avvio, ad esempio su una prima esecuzione o dopo `/logout`, Claude Code attende fino a cinque secondi per il recupero prima di aprire la sessione. Quando la politica arriva in tempo, Claude Code la applica dalla prima schermata e mostra i vostri [`companyAnnouncements`](/docs/it/settings-reference#companyannouncements) su di essa. Quando il payload necessita di [approvazione della sicurezza](#security-approval-dialogs), Claude Code termina l'attesa e applica il payload una volta che lo sviluppatore approva
* In qualsiasi altro avvio, e quando quell'attesa di cinque secondi scade, Claude Code apre la sessione mentre il recupero continua, quindi passa una breve finestra prima che le impostazioni si carichino e le restrizioni abbiano effetto
* Se il recupero non riesce, Claude Code continua senza impostazioni gestite dal server e avverte nelle sessioni interattive che nessuna politica remota si applica; le impostazioni gestite dall'endpoint si applicano comunque. Se una fonte gestita imposta [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup), Claude Code esce invece

**Avvii successivi con impostazioni memorizzate nella cache:**

* Le impostazioni memorizzate nella cache si applicano immediatamente all'avvio, ad eccezione dei valori memorizzati nella cache `modelPricing` e `managedMcpServers` e delle variabili di ambiente che Claude Code trattiene fino a quando il server non conferma il payload
* Un [`modelPricing`](/docs/it/settings-reference#modelpricing) memorizzato nella cache non si applica fino a quando il recupero della sessione non conferma il payload. Fino ad allora, le cifre di costo che gli sviluppatori vedono in `/usage` e nella riga di stato sono al prezzo di listino
* Un blocco [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers) memorizzato nella cache non si applica fino a quando il recupero della sessione non conferma il payload. Claude Code attende fino a 30 secondi per quel recupero prima di connettere i server MCP. Se il recupero non riesce o scade, la sessione inizia senza i server dell'organizzazione, `/status` lo dice, e si connettono una volta che un recupero successivo li conferma. Vedere [Quando i server forniti si connettono](/docs/it/managed-mcp#when-provided-servers-connect) per il comportamento completo, incluso il primo avvio. Richiede Claude Code v2.1.259 o successivo
* Claude Code recupera le impostazioni aggiornate in background
* Le impostazioni memorizzate nella cache persistono attraverso i guasti di rete. Se il recupero all'avvio non riesce, Claude Code avverte nelle sessioni interattive che la politica memorizzata nella cache è in vigore
* Fino a quando un recupero non riesce, i valori trattenuti all'avvio rimangono trattenuti

Claude Code trattiene diverse categorie di variabili nel blocco `env` memorizzato nella cache fino a quando il server non conferma il payload per la sessione. Questo impedisce a un valore proxy, autorità di certificazione, endpoint o credenziale memorizzato nella cache di reindirizzare, intercettare o autenticare nuovamente il recupero delle impostazioni che conferma il payload. L'indurimento si applica solo alla cache delle impostazioni recuperate dal server: le [impostazioni gestite dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) distribuite tramite MDM o `managed-settings.json` non sono interessate. Il trattenimento richiede Claude Code v2.1.198 o successivo; prima della v2.1.198, l'intero blocco `env` memorizzato nella cache si applica all'avvio. Le categorie trattenute includono:

* Configurazione proxy e TLS, come `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` e le variabili del certificato client mTLS `CLAUDE_CODE_CLIENT_CERT` e `CLAUDE_CODE_CLIENT_KEY`
* Routing API e selezione del provider, inclusi `ANTHROPIC_BASE_URL`, le variabili di selezione del provider come `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, e gli URL dell'endpoint del provider come `ANTHROPIC_BEDROCK_BASE_URL`
* Credenziali di autenticazione, come `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` e `CLAUDE_CODE_OAUTH_TOKEN`
* Il selettore della directory di configurazione `CLAUDE_CONFIG_DIR`
* Selettori di fonte di credenziale e directory di configurazione, in Claude Code v2.1.223 o successivo: le variabili di Workload Identity Federation come `ANTHROPIC_FEDERATION_RULE_ID` e `ANTHROPIC_IDENTITY_TOKEN`, i selettori di profilo e directory di configurazione `ANTHROPIC_PROFILE` e `ANTHROPIC_CONFIG_DIR`, e le variabili della directory del sistema operativo `HOME`, `XDG_CONFIG_HOME`, `APPDATA` e `USERPROFILE`

Claude Code legge le variabili di Workload Identity Federation e i selettori `ANTHROPIC_PROFILE` e `ANTHROPIC_CONFIG_DIR` solo all'avvio, quindi un valore consegnato dal server per essi non cambia la fonte di credenziale della sessione nemmeno dopo che il recupero riesce. Per fornire quei selettori su Claude Code v2.1.223 o successivo, utilizzate [impostazioni gestite dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) come MDM o `managed-settings.json`. Per `CLAUDE_CONFIG_DIR` e le variabili della directory del sistema operativo, il trattenimento stesso è la protezione: il valore memorizzato nella cache rimane fuori dall'ambiente fino a quando il server non conferma il payload.

Ogni altra chiave nel blocco `env` memorizzato nella cache si applica all'avvio. Una volta che il server conferma il payload, e voi lo approvate se necessita di [approvazione della sicurezza](#security-approval-dialogs), le variabili trattenute si applicano per il resto della sessione.

Se la vostra organizzazione ha bisogno di un proxy per raggiungere `api.anthropic.com`, il trattenimento influisce solo sul blocco `env` consegnato dal server stesso: un proxy impostato in un blocco `env` [gestito dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) tramite MDM o `managed-settings.json`, nell'ambiente della shell, o nelle [impostazioni utente](/docs/it/settings#where-settings-live) raggiunge il recupero delle impostazioni. La fonte gestita dall'endpoint richiede Claude Code v2.1.223 o successivo: il valore proxy gestito dal server memorizzato nella cache è trattenuto fino a quando il recupero non lo conferma, quindi il valore gestito dall'endpoint si riempie per chiave e raggiunge il recupero stesso. Prima della v2.1.223, utilizzate l'ambiente della shell o le impostazioni utente in modo che il proxy si applichi insieme a un payload server memorizzato nella cache. Il primo avvio non ha cache, quindi una fonte gestita dall'endpoint, l'ambiente della shell o le impostazioni utente è ancora necessario per il recupero iniziale.

Claude Code applica la maggior parte degli aggiornamenti delle impostazioni alle sessioni in esecuzione senza un riavvio. Alcuni aggiornamenti si applicano solo al prossimo avvio, inclusa la configurazione dell'esportatore OpenTelemetry, la chiave `model` e la rimozione di una variabile dal blocco `env`.

<h3 id="invalid-entries-in-delivered-settings">
  Voci non valide nelle impostazioni consegnate
</h3>

Quando parte di un payload non supera la convalida dello schema, Claude Code visualizza un errore di convalida e applica ogni impostazione valida rimanente; [Voci non valide nelle impostazioni gestite](/docs/it/managed-settings#invalid-entries-in-managed-settings) dice cosa elimina e quali chiavi ricadono su un valore più rigoroso. Richiede Claude Code v2.1.169 o successivo.

La consegna gestita dal server aggiunge questi comportamenti:

* La cache in `~/.claude/remote-settings.json` memorizza il payload salvato con le voci non valide rimosse, ad eccezione dei valori `cleanupPeriodDays` e `desktopSessionCleanupPeriodDays` non validi, che rimangono nella copia memorizzata nella cache e non vengono mai applicati.
* Quando nessun campo nel payload può essere salvato e il payload non è solo quelle chiavi di conservazione, Claude Code rifiuta il payload, mantiene le ultime impostazioni memorizzate nella cache accettate e scrive `Remote settings: Settings validation failed - no fields could be salvaged` nel log di debug. Con `forceRemoteSettingsRefresh` impostato, la CLI esce invece.
* La [finestra di dialogo di approvazione della sicurezza](#security-approval-dialogs) valuta il payload salvato, quindi una voce non valida rimossa non viene mai presentata per l'approvazione e non viene mai eseguita.

Per eseguire il debug dei problemi di consegna, eseguite `claude --debug-file <path>` e cercate nel log `Remote settings`. Convalidate una modifica del payload con `claude doctor` su una macchina di test prima di distribuirla all'organizzazione.

<h3 id="enforce-fail-closed-startup">
  Applicare l'avvio fail-closed
</h3>

Per impostazione predefinita, se il recupero delle impostazioni remote non riesce all'avvio, la CLI continua con le impostazioni memorizzate nella cache dal recupero riuscito precedente, ad eccezione dei [valori che Claude Code trattiene](#fetch-and-caching-behavior) fino a quando un recupero riesce. Su una macchina che non le ha mai recuperate, la CLI continua senza impostazioni gestite dal server e applica comunque qualsiasi [impostazione gestita dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) sul dispositivo.

Per impedire ai client di avviarsi su impostazioni gestite dal server memorizzate nella cache o assenti, impostare `forceRemoteSettingsRefresh: true` nelle impostazioni gestite.

I client che hanno effettuato l'accesso tramite un [gateway di app Claude](#platform-availability) attendono il recupero all'avvio indipendentemente dal fatto che impostiate questo, e gestiscono un recupero non riuscito come segue:

* Se il gateway risponde a un avvio interattivo presidiato con un `401` e questa impostazione è disattivata, il gateway ha terminato quell'accesso. Claude Code stampa [`Cloud gateway session expired — run /login to reconnect.`](/docs/it/errors#cloud-gateway-session-expired) e apre la sessione disconnessa dal gateway fino a quando l'utente non esegue `/login`.
* Quando il recupero non riesce in qualsiasi altro modo, o in qualsiasi altro tipo di avvio ad eccezione di un sottocomando `claude auth`, il client esce con un errore.

Quando questa impostazione è attiva in una sessione che recupera impostazioni gestite dal server, la CLI si blocca all'avvio fino a quando le impostazioni remote non vengono recuperate di recente. Se il recupero non riesce, la CLI esce piuttosto che procedere senza la politica. Questa impostazione si auto-perpetua: una volta consegnata dal server, viene anche memorizzata nella cache localmente in modo che gli avvii successivi applichino lo stesso comportamento anche prima del primo recupero riuscito di una nuova sessione. Una sessione che [non recupera impostazioni gestite dal server](#platform-availability) inizia senza attendere.

Per abilitare questa funzione, aggiungere la chiave alla configurazione delle impostazioni gestite:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Potete anche impostare questa chiave in un [profilo MDM](/docs/it/managed-settings#delivery-mechanisms) gestito dall'endpoint o in un file `managed-settings.json` di sistema per applicare il comportamento fail-closed al primo avvio, prima che qualsiasi payload del server sia stato consegnato. In Claude Code v2.1.191 o successivo, questo flag è un'eccezione alla [regola di precedenza](#settings-precedence) sopra: Claude Code lo rispetta quando qualsiasi fonte gestita controllata dall'amministratore lo imposta, anche se è presente anche un payload memorizzato nella cache gestito dal server, quindi non ignora un valore consegnato da MDM quando esistono impostazioni gestite dal server.

Quando un [`policyHelper`](/docs/it/settings-reference#policyhelper) fornisce impostazioni gestite, il suo output sostituisce ogni altra fonte gestita per le chiavi che Claude Code legge dopo l'avvio. Per le fonti da cui Claude Code legge questa chiave, vedere [la sua voce di impostazioni](/docs/it/settings-reference#forceremotesettingsrefresh). La voce `policyHelper` dice da quali fonti Claude Code legge l'helper e quando viene eseguito.

Il recupero delle impostazioni invia anche un'intestazione `Cache-Control: no-cache` in modo che i proxy HTTP intermedi non servano una risposta non aggiornata.

Prima di abilitare questa impostazione, assicurarsi che le politiche di rete consentano la connettività a `api.anthropic.com`. Se tale endpoint non è raggiungibile, la CLI esce all'avvio e gli utenti non possono avviare Claude Code.

I sottocomandi `claude auth` come `claude auth login` sono esenti da questo controllo e dall'uscita all'avvio del gateway, in modo che gli utenti possano autenticarsi nuovamente quando le credenziali scadute sono il motivo per cui il recupero delle impostazioni non riesce.

<h3 id="security-approval-dialogs">
  Finestre di dialogo di approvazione della sicurezza
</h3>

Determinate impostazioni che potrebbero comportare rischi di sicurezza richiedono l'approvazione esplicita dell'utente prima che Claude Code le applichi in una sessione interattiva:

* **Impostazioni dei comandi shell**: impostazioni che eseguono comandi shell, come `apiKeyHelper`, `statusLine` e `otelHeadersHelper`
* **Impostazioni dei binari della sandbox**: `sandbox.bwrapPath`, `sandbox.socatPath` e `sandbox.ripgrep`. Ognuna di queste impostazioni punta a un eseguibile e Claude Code esegue quell'eseguibile
* **Impostazioni di rete e isolamento della sandbox**: impostazioni di [sandbox](/docs/it/sandboxing) che consentono al proxy della sandbox di leggere, reindirizzare o autenticare il traffico, o che indeboliscono l'isolamento della sandbox: `sandbox.network.tlsTerminate`, `sandbox.network.httpProxyPort`, `sandbox.network.socksProxyPort`, `sandbox.credentials`, `sandbox.allowAppleEvents`, `sandbox.enableWeakerNestedSandbox`, `sandbox.enableWeakerNetworkIsolation`, `sandbox.filesystem.disabled`, `sandbox.network.allowAllUnixSockets`, `sandbox.network.allowUnixSockets` e `sandbox.network.allowMachLookup`. Un blocco `sandbox.credentials` che contiene solo regole `deny` non necessita di approvazione, poiché limita la sandbox senza dare al proxy una credenziale. Prima della v2.1.251, Claude Code applicava queste impostazioni senza approvazione
* **Variabili di ambiente personalizzate**: variabili `env` consegnate che richiedono l'approvazione dell'utente, come le variabili proxy e base-URL; vedere [Variabili di ambiente e la finestra di dialogo di approvazione](#environment-variables-and-the-approval-dialog)
* **Configurazioni di hook**: qualsiasi definizione di hook

Quando queste impostazioni sono presenti, gli utenti vedono una finestra di dialogo di sicurezza che spiega cosa viene configurato. Gli utenti devono approvare per procedere. Se un utente rifiuta le impostazioni, Claude Code esce.

Un CLAUDE.md gestito consegnato tramite la chiave [`claudeMd`](/docs/it/settings-reference#claudemd) non richiede approvazione, perché è testo di istruzione per Claude piuttosto che un comando che Claude Code esegue. Claude Code controlla comunque i [permessi](/docs/it/permissions) per gli strumenti che Claude utilizza mentre segue quelle istruzioni. Prima della v2.1.260, un valore `claudeMd` richiedeva approvazione anche.

<h4 id="approval-memory">
  Memoria di approvazione
</h4>

Claude Code registra la vostra approvazione nella vostra directory di configurazione, `~/.claude` a meno che non impostiate [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars). Quello che registra dipende dalla credenziale che il recupero delle impostazioni utilizza:

* **Un accesso claude.ai salvato da `/login` o `claude auth login`, o l'[accesso alla Console senza chiave](/docs/it/authentication#sign-in-without-an-api-key)**: un'approvazione per organizzazione, tenuta dall'account che ha approvato più di recente.
* **Un accesso al [gateway di app Claude](/docs/it/claude-apps-gateway)**: un'approvazione per gateway.

  Se vi disconnettete e vi riconnettete allo stesso gateway, Claude Code non mostra la finestra di dialogo di nuovo mentre le impostazioni che richiedono approvazione rimangono invariate. Claude Code la mostra di nuovo quando quelle impostazioni cambiano, quando vi connettete a un gateway diverso e quando accettate un nuovo certificato per lo stesso gateway.

  Claude Code non salva alcuna approvazione per un gateway di sviluppo loopback raggiunto su HTTP semplice, quindi la finestra di dialogo appare di nuovo dopo ogni accesso.
* **Qualsiasi altra credenziale**, come una chiave API o `CLAUDE_CODE_OAUTH_TOKEN`: un'approvazione per le impostazioni consegnate, mantenuta con la copia memorizzata nella cache delle impostazioni in quella directory di configurazione. Claude Code mostra la finestra di dialogo di nuovo quando le impostazioni che richiedono approvazione cambiano, e dopo che eseguite `/logout` o `claude auth logout`, uno dei quali elimina la copia memorizzata nella cache.

Un'approvazione per `sandbox.credentials` o `sandbox.network.tlsTerminate` copre anche le voci [`sandbox.network.allowedDomains`](/docs/it/settings-reference#sandbox-network-alloweddomains) nelle stesse impostazioni consegnate, perché entrambe le impostazioni agiscono su quella lista di autorizzazione. La finestra di dialogo appare di nuovo quando l'amministratore aggiunge o rimuove una di quelle voci, anche se `sandbox.network.allowedDomains` non richiede approvazione da sola.

Con un accesso claude.ai salvato:

* Se vi disconnettete e vi riconnettete, o passate a un'altra organizzazione e successivamente tornate, Claude Code non mostra la finestra di dialogo di nuovo mentre quelle impostazioni rimangono invariate, a meno che un altro account non le abbia approvate per quell'organizzazione nella stessa directory di configurazione nel frattempo.
* Se vi connettete alla stessa organizzazione con un account diverso, Claude Code mostra la finestra di dialogo di nuovo anche quando le impostazioni rimangono invariate. L'approvazione di quell'account sostituisce la precedente, quindi quando tornate, Claude Code mostra la finestra di dialogo ancora una volta.

Claude Code non può sempre mostrare la finestra di dialogo. Ogni caso di seguito dice quali impostazioni si applicano quando non può e quando vedete di nuovo la finestra di dialogo:

* **Una sessione interattiva che non può mostrare la finestra di dialogo**: Claude Code non applica le impostazioni consegnate e mantiene le ultime impostazioni approvate. La finestra di dialogo appare nella prossima sessione che può mostrarla. Richiede Claude Code v2.1.211 o successivo.
* **`claude install` o `claude update`**: Claude Code non mostra la finestra di dialogo durante nessuno dei due comandi. Il comando viene eseguito con le ultime impostazioni approvate e la finestra di dialogo appare nella vostra prossima sessione interattiva. Se Claude Code attende il recupero delle impostazioni all'avvio, come con [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) impostato o su una distribuzione [gateway di app Claude](/docs/it/claude-apps-gateway), mostra la finestra di dialogo durante il comando invece, e un'esecuzione di installazione da una pipe non riesce; vedere [`Raw mode is not supported` durante l'installazione](/docs/it/troubleshoot-install#raw-mode-is-not-supported-during-install). Prima della v2.1.246, Claude Code cercava di mostrare la finestra di dialogo durante questi comandi anche.
* **Un errore chiude la finestra di dialogo prima che rispondiate**: Claude Code non applica le impostazioni consegnate e mantiene le ultime impostazioni approvate. La mostra di nuovo nella prossima sessione che può mostrarla.
* **Un'esecuzione non interattiva**, come `claude -p` o una sessione Agent SDK: Claude Code non può mostrare la finestra di dialogo, quindi quando le impostazioni consegnate richiederebbero approvazione, le applica solo per quella esecuzione. Non le registra come approvate o le scrive nella [cache locale](#fetch-and-caching-behavior), e la prossima sessione interattiva mostra la finestra di dialogo. Fino a quando un utente non approva in una sessione interattiva, ogni esecuzione non interattiva recupera le impostazioni di nuovo all'avvio. Prima della v2.1.207, un'esecuzione non interattiva salvava le impostazioni come approvate, quindi le sessioni interattive successive non mostravano mai la finestra di dialogo per esse.

<h4 id="environment-variables-and-the-approval-dialog">
  Variabili di ambiente e la finestra di dialogo di approvazione
</h4>

Claude Code applica alcune variabili `env` consegnate senza mostrare all'utente la finestra di dialogo di approvazione, incluse:

* Interruttori di funzionalità e comandi
* Impostazioni di selezione e comportamento del modello, come `ANTHROPIC_MODEL`, `DISABLE_PROMPT_CACHING` e `CLAUDE_CODE_EFFORT_LEVEL`
* Impostazioni della finestra di contesto e compattazione, come `DISABLE_AUTO_COMPACT`
* Opzioni di accessibilità e interfaccia utente del terminale
* Limiti numerici, budget e timeout

Altre variabili consegnate possono richiedere l'approvazione dell'utente prima che abbiano effetto; un valore proxy, base-URL o `OTEL_EXPORTER_OTLP_ENDPOINT` non vuoto lo fa sempre. Quando una variabile consegnata necessita di approvazione, la finestra di dialogo la nomina, in modo che l'utente veda esattamente cosa la politica chiede di impostare. Prima della v2.1.218, Claude Code applicava meno variabili senza chiedere all'utente, quindi impostazioni come `DISABLE_AUTO_COMPACT` attivavano la finestra di dialogo a qualsiasi valore non vuoto.

Claude Code decide se quattro interruttori di privacy necessitano di approvazione dal valore consegnato piuttosto che dal nome della variabile: `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, `DISABLE_ERROR_REPORTING`, `DISABLE_TELEMETRY` e `DO_NOT_TRACK`. Un valore veritiero come `1` o `true` spegne solo il tracciamento, la segnalazione o altro traffico non essenziale, quindi Claude Code lo applica senza chiedere all'utente. Per qualsiasi altro valore non vuoto, Claude Code mostra la finestra di dialogo. Prima della v2.1.218, tutti tranne `DO_NOT_TRACK` si applicavano senza approvazione a qualsiasi valore, e `DO_NOT_TRACK` attivava la finestra di dialogo a qualsiasi valore non vuoto.

Claude Code decide anche se [`API_FORCE_IDLE_TIMEOUT`](/docs/it/env-vars) necessita di approvazione dal valore consegnato: un valore veritiero attiva solo il [timeout di inattività del corpo](/docs/it/network-config#streaming-idle-watchdogs), quindi Claude Code lo applica senza chiedere all'utente. Per qualsiasi altro valore non vuoto, Claude Code mostra la finestra di dialogo. Prima della v2.1.248, qualsiasi valore non vuoto attivava la finestra di dialogo.

Se [`ANTHROPIC_CUSTOM_HEADERS`](/docs/it/env-vars#variables) necessita di approvazione dipende anche dal valore consegnato. Le intestazioni che solo taggano le richieste, come `Accept-Language`, si applicano senza la finestra di dialogo. Una riga che nomina una credenziale, un selettore di org o tenant, un override di routing o host, o un'intestazione di comportamento API, come `Authorization`, `X-Api-Key`, `Host`, `anthropic-beta` o le intestazioni `X-Amzn-Bedrock-*`, richiede approvazione. Lo fa anche una riga il cui nome non è un token di intestazione HTTP valido, o il cui valore contiene un carattere che un'intestazione HTTP non può portare. Il controllo corrisponde alle parole all'interno del nome dell'intestazione, quindi `X-Client-Version`, che contiene `client` e `version`, richiede approvazione anche. Prima della v2.1.251, qualsiasi valore `ANTHROPIC_CUSTOM_HEADERS` si applicava senza di essa.

Un valore falso come `0` o `false` per [`ENABLE_BETA_TRACING_DETAILED`](/docs/it/env-vars#variables) o [`OTEL_LOG_RAW_API_BODIES`](/docs/it/env-vars#variables) si applica senza la finestra di dialogo, perché spegne solo la tracciatura dettagliata o l'acquisizione del corpo API grezzo. Qualsiasi altro valore non vuoto per una delle due variabili richiede approvazione.

<h2 id="platform-availability">
  Disponibilità della piattaforma
</h2>

Le impostazioni gestite dal server richiedono una connessione diretta a `api.anthropic.com`. La consegna richiede inoltre che la sessione si autentichi con una di queste credenziali:

* Un accesso OAuth di Team o Enterprise
* Un token OAuth fornito tramite `CLAUDE_CODE_OAUTH_TOKEN`
* Una chiave API configurata direttamente
* Un [profilo Anthropic](/docs/it/authentication#anthropic-profiles-and-federation-credentials) `user_oauth`, a meno che il profilo non imposti un `base_url` diverso dall'API Anthropic. Richiede Claude Code v2.1.257 o successivo.

Né le chiavi restituite da uno script [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) né le credenziali di [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) attivano il recupero delle impostazioni.

In una sessione [Cowork](https://claude.com/docs/cowork/overview) nell'app Claude Desktop, Claude Code non recupera le impostazioni gestite dal server dalla console di amministrazione claude.ai, anche quando l'utente accede con un account Team o Enterprise. [Dove e quando si applica una policy](/docs/it/managed-settings#where-and-when-a-policy-applies) copre quale policy raggiunge le sessioni Cowork sulla macchina dell'utente e le sessioni Cowork remote.

Se esportate una variabile provider `CLAUDE_CODE_USE_*` o un `ANTHROPIC_BASE_URL` non predefinito nella vostra shell, Claude Code salta il recupero delle impostazioni per le vostre sessioni. [`claude doctor` e `/status` segnalano il recupero saltato e la sua causa](#verify-settings-delivery).

Non potete cancellare l'esportazione con un blocco `env` gestito dal server, perché il blocco arriva attraverso il recupero che l'esportazione impedisce. Un blocco `env` con [impostazioni gestite dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) non ripristina il recupero neanche: Claude Code verifica l'idoneità prima di applicare i blocchi `env` gestiti, quindi il valore gestito dall'endpoint cambia la selezione del provider della sessione ma il recupero rimane saltato.

Per ripristinare la consegna gestita dal server, rimuovete l'esportazione dalla vostra shell, oppure impostate la variabile su `""` nel blocco `env` delle impostazioni utente, che si applica prima della verifica dell'idoneità. Per applicare la policy senza fare affidamento sugli utenti per modificare le loro shell, consegnate le impostazioni attraverso il canale gestito dall'endpoint.

Per le distribuzioni Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e [Claude Platform on AWS](/docs/it/claude-platform-on-aws), un [gateway di app Claude](/docs/it/claude-apps-gateway) auto-ospitato fornisce la consegna equivalente di impostazioni gestite da remoto: i client firmati dal gateway recuperano le impostazioni gestite dal gateway invece che da `api.anthropic.com`. La semantica degli errori differisce all'avvio: un client gateway che non riesce a raggiungere il gateway esce con un errore invece di ricadere sulle impostazioni memorizzate nella cache, mentre l'aggiornamento in background orario è fail-open su entrambi i canali.

<h2 id="audit-logging">
  Registrazione di audit
</h2>

Gli eventi del registro di audit per le modifiche alle impostazioni sono disponibili tramite l'API di conformità o l'esportazione del registro di audit. Contattare il vostro team di account Anthropic per l'accesso.

Gli eventi di audit includono il tipo di azione eseguita, l'account e il dispositivo che ha eseguito l'azione, e riferimenti ai valori precedenti e nuovi.

<h2 id="security-considerations">
  Considerazioni sulla sicurezza
</h2>

Le impostazioni gestite dal server forniscono l'applicazione centralizzata dei criteri, ma operano come un controllo lato client, non come un limite di sicurezza. Su dispositivi non gestiti, un utente non ha bisogno dell'accesso amministratore o sudo per aggirarli.

| Scenario                                                                           | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| L'utente modifica il file di impostazioni memorizzato nella cache                  | Il file manomesso si applica all'avvio, ad eccezione dei [valori che Claude Code trattiene](#fetch-and-caching-behavior) fino a quando il server non conferma il payload. Il prossimo recupero dal server ripristina le impostazioni corrette, ad eccezione dei [valori che si applicano solo al prossimo avvio](#fetch-and-caching-behavior), come `model` o una variabile aggiunta al blocco `env`, che rimangono in vigore fino al riavvio                                                                                                                                                                                                                                                                                                                                          |
| L'utente elimina il file di impostazioni memorizzato nella cache                   | Si verifica il comportamento del [primo avvio](#fetch-and-caching-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| L'utente esegue un binario Claude Code modificato                                  | Un utente che può eseguire un client modificato può aggirare qualsiasi controllo lato client                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| L'utente esegue una versione precedente di Claude Code                             | Le versioni precedenti alle impostazioni gestite dal server non le recuperano o non le applicano                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| L'API non è disponibile                                                            | Le impostazioni memorizzate nella cache si applicano se disponibili, ad eccezione dei [valori che Claude Code trattiene](#fetch-and-caching-behavior) fino a quando un recupero ha successo. Senza una cache, Claude Code non applica alcuna impostazione gestita dal server fino al prossimo recupero riuscito e applica comunque qualsiasi [impostazione gestita dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) sul dispositivo. Con `forceRemoteSettingsRefresh: true`, la CLI esce invece di continuare, ad eccezione dei [`claude auth` subcomandi](#enforce-fail-closed-startup). I client che hanno effettuato l'accesso tramite un [gateway di app Claude](#platform-availability) escono all'avvio senza questa impostazione, con la stessa eccezione `claude auth` |
| L'utente si autentica con un'organizzazione diversa                                | Le impostazioni non vengono consegnate per gli account al di fuori dell'organizzazione gestita                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| L'utente configura un [provider di modelli di terze parti](#platform-availability) | Le impostazioni gestite dal server vengono ignorate. Questo include l'impostazione di `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, o un `ANTHROPIC_BASE_URL` non predefinito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Il traffico di rete viene intercettato o reindirizzato                             | La convalida TLS disabilitata o il traffico intercettato può alterare le impostazioni che il client riceve                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

Per registrare le modifiche ai file di impostazioni locali, incluso `managed-settings.json`, utilizzare gli [hook `ConfigChange`](/docs/it/hooks#configchange). Claude Code non li esegue quando arrivano le impostazioni gestite dal server o si aggiornano, o quando un profilo MDM o una politica del registro cambia, e un hook non può bloccare una modifica `policy_settings`.

Per limitare quali organizzazioni i vostri utenti possono accedere con le credenziali fornite dal client, consultare [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) nel Centro assistenza Claude. Per garanzie di applicazione più forti, utilizzare le [impostazioni gestite dall'endpoint](/docs/it/managed-settings#delivery-mechanisms) su dispositivi registrati in una soluzione MDM.

<h2 id="see-also">
  Vedere anche
</h2>

Pagine correlate per la gestione della configurazione di Claude Code:

* [Tutte le impostazioni](/docs/it/settings-reference): ogni chiave di impostazione
* [Endpoint-managed settings](/docs/it/managed-settings#delivery-mechanisms): impostazioni gestite distribuite ai dispositivi dal reparto IT
* [Authentication](/docs/it/authentication): configurare l'accesso degli utenti a Claude Code
* [Security](/docs/it/security): salvaguardie di sicurezza e best practice
