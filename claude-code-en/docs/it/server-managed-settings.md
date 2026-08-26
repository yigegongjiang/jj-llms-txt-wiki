> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare le impostazioni gestite dal server

> Configurare centralmente Claude Code per la vostra organizzazione tramite impostazioni consegnate dal server, senza richiedere infrastrutture di gestione dei dispositivi.

Le impostazioni gestite dal server consentono ai proprietari dell'organizzazione di configurare centralmente Claude Code da [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) nella console claude.ai. I client di Claude Code recuperano automaticamente queste impostazioni quando gli utenti si autenticano con un accesso OAuth dell'organizzazione o una chiave API configurata direttamente, su piattaforme dove la consegna gestita dal server è supportata. Vedere [Disponibilità della piattaforma](#platform-availability).

Questo approccio è progettato per le organizzazioni che non dispongono di infrastrutture di gestione dei dispositivi, o che hanno la necessità di gestire le impostazioni per gli utenti su dispositivi non gestiti.

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

Claude Code supporta due approcci per la configurazione centralizzata. Le impostazioni gestite dal server forniscono la configurazione dai server di Anthropic. Le [impostazioni gestite dall'endpoint](/docs/it/settings#settings-files) vengono distribuite direttamente ai dispositivi tramite criteri nativi del sistema operativo (preferenze gestite macOS, registro Windows) o file di impostazioni gestiti.

| Approccio                                                             | Ideale per                                                    | Modello di sicurezza                                                                                                                |
| :-------------------------------------------------------------------- | :------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| **Impostazioni gestite dal server**                                   | Organizzazioni senza MDM, o utenti su dispositivi non gestiti | Impostazioni consegnate dai server di Anthropic al momento dell'autenticazione                                                      |
| **[Impostazioni gestite dall'endpoint](/docs/it/settings#settings-files)** | Organizzazioni con MDM o gestione degli endpoint              | Impostazioni distribuite ai dispositivi tramite profili di configurazione MDM, criteri del registro, o file di impostazioni gestiti |

Se i vostri dispositivi sono registrati in una soluzione MDM o di gestione degli endpoint, le impostazioni gestite dall'endpoint forniscono garanzie di sicurezza più forti perché il file di impostazioni può essere protetto dalla modifica dell'utente a livello del sistema operativo. Le impostazioni gestite dall'endpoint non raggiungono le [sessioni cloud](/docs/it/model-config#surface-coverage), quindi le organizzazioni che utilizzano Claude Code sul web dovrebbero configurare anche le impostazioni gestite dal server.

<h2 id="configure-server-managed-settings">
  Configurare le impostazioni gestite dal server
</h2>

<Steps>
  <Step title="Aprire la console di amministrazione">
    Nella console claude.ai, andare a [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Se il collegamento vi reindirizza a una pagina Admin Settings diversa invece della pagina Claude Code, il vostro account non dispone del ruolo richiesto. I ruoli Admin e altri ruoli non-Owner non possono visualizzare o modificare le impostazioni gestite, quindi chiedete a un Owner o Primary Owner nella vostra organizzazione di apportare la modifica. Vedere [Controllo di accesso](#access-control).
  </Step>

  <Step title="Definire le impostazioni">
    Aggiungere la configurazione come JSON. Tutte le [impostazioni disponibili in `settings.json`](/docs/it/settings#available-settings) sono supportate eccetto quelle limitate alla distribuzione delle politiche a livello del sistema operativo; vedere [Limitazioni attuali](#current-limitations) per questo breve elenco. Questo include [hooks](/docs/it/hooks), [variabili di ambiente](/docs/it/env-vars), e [impostazioni solo gestite](/docs/it/permissions#managed-only-settings) come `allowManagedPermissionRulesOnly`.

    Questo esempio applica un elenco di negazione delle autorizzazioni, impedisce agli utenti di ignorare le autorizzazioni e limita le regole di autorizzazione a quelle definite nelle impostazioni gestite:

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

    Per configurare il classificatore della [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) in modo che conosca quali repository, bucket e domini la vostra organizzazione ritiene affidabili:

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    Poiché gli hook eseguono comandi shell, gli utenti vedono una [finestra di dialogo di approvazione della sicurezza](#security-approval-dialogs) prima che vengano applicati. Vedere [Configurare la modalità auto](/docs/it/auto-mode-config) per come le voci `autoMode` influenzano ciò che il classificatore blocca e avvertimenti importanti sui campi `environment`, `allow`, `soft_deny` e `hard_deny`.
  </Step>

  <Step title="Salvare e distribuire">
    Salvare le modifiche. I client di Claude Code ricevono le impostazioni aggiornate al prossimo avvio o ciclo di polling orario.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verificare la consegna delle impostazioni
</h3>

Per confermare che le impostazioni vengono applicate, chiedere a un utente di riavviare Claude Code. Se la configurazione include impostazioni che attivano la [finestra di dialogo di approvazione della sicurezza](#security-approval-dialogs), l'utente vede un prompt che descrive le impostazioni gestite all'avvio. È inoltre possibile verificare che le regole di autorizzazione gestite siano attive facendo eseguire a un utente `/permissions` per visualizzare le regole di autorizzazione effettive.

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

La maggior parte delle [chiavi di impostazioni](/docs/it/settings#available-settings) funzionano in qualsiasi ambito. Un numero limitato di chiavi viene letto solo dalle impostazioni gestite e non ha alcun effetto quando posizionato nei file di impostazioni dell'utente o del progetto. Vedere [impostazioni solo gestite](/docs/it/permissions#managed-only-settings) per l'elenco completo. Qualsiasi impostazione non in tale elenco può comunque essere posizionata nelle impostazioni gestite e ha la precedenza più alta.

<h3 id="current-limitations">
  Limitazioni attuali
</h3>

Le impostazioni gestite dal server hanno le seguenti limitazioni:

* Le impostazioni si applicano uniformemente a tutti gli utenti dell'organizzazione. Le configurazioni per gruppo non sono ancora supportate.
* Un file [`managed-mcp.json`](/docs/it/managed-mcp) non può essere distribuito tramite impostazioni gestite dal server. Distribuire invece le chiavi di politica `allowedMcpServers` e `deniedMcpServers` lì.
* Le impostazioni limitate alle fonti delle politiche a livello del sistema operativo, come `policyHelper` e `wslInheritsWindowsSettings`, non vengono rispettate. Distribuirle invece tramite MDM o un file `managed-settings.json` di sistema.

<h2 id="settings-delivery">
  Consegna delle impostazioni
</h2>

<h3 id="settings-precedence">
  Precedenza delle impostazioni
</h3>

Le impostazioni gestite dal server e le [impostazioni gestite dall'endpoint](/docs/it/settings#settings-files) occupano entrambe il livello più alto nella [gerarchia delle impostazioni](/docs/it/settings#settings-precedence) di Claude Code. Nessun altro livello di impostazioni può sostituirle, inclusi gli argomenti della riga di comando.

All'interno del livello gestito, un [`policyHelper`](/docs/it/settings#compute-managed-settings-with-a-policy-helper) configurato ha la precedenza su ogni altra fonte gestita, incluse le impostazioni gestite dal server: il suo output diventa l'unica configurazione gestita per l'esecuzione.

Altrimenti, Claude Code utilizza la prima fonte che fornisce una configurazione non vuota. Le impostazioni gestite dal server vengono controllate per prime, quindi le impostazioni gestite dall'endpoint. Le fonti non si uniscono: se le impostazioni gestite dal server forniscono qualsiasi chiave, le altre impostazioni gestite dall'endpoint vengono ignorate. Se le impostazioni gestite dal server non forniscono nulla, le impostazioni gestite dall'endpoint si applicano.

Si applica un'eccezione: un piccolo insieme di [chiavi di blocco tra fonti](/docs/it/settings#settings-precedence), come i blocchi della lista di autorizzazione della sandbox, viene rispettato quando qualsiasi fonte gestita controllata dall'amministratore li imposta; il livello del registro HKCU scrivibile dall'utente è escluso.

Se cancellate la configurazione gestita dal server nella console di amministrazione con l'intenzione di tornare a una plist gestita dall'endpoint o a una politica del registro, tenete presente che le [impostazioni memorizzate nella cache](#fetch-and-caching-behavior) persistono sulle macchine client fino al prossimo recupero riuscito. Eseguite `/status` per vedere quale fonte gestita è attiva.

<h3 id="fetch-and-caching-behavior">
  Comportamento di recupero e caching
</h3>

Claude Code recupera le impostazioni dai server di Anthropic all'avvio e esegue il polling per gli aggiornamenti ogni ora durante le sessioni attive.

**Primo avvio senza impostazioni memorizzate nella cache:**

* Claude Code recupera le impostazioni in modo asincrono
* Se il recupero non riesce, Claude Code continua senza impostazioni gestite
* C'è una breve finestra prima che le impostazioni si carichino in cui le restrizioni non sono ancora applicate

**Avvii successivi con impostazioni memorizzate nella cache:**

* Le impostazioni memorizzate nella cache si applicano immediatamente all'avvio, ad eccezione delle variabili di ambiente di trasporto, routing e autenticazione descritte di seguito
* Claude Code recupera le impostazioni aggiornate in background
* Le impostazioni memorizzate nella cache persistono attraverso i guasti di rete. Le variabili di ambiente trattenute rimangono trattenute fino a quando un recupero non riesce

A partire dalla v2.1.198, Claude Code trattiene tre categorie di variabili nel blocco `env` memorizzato nella cache fino a quando il server non conferma il payload per la sessione. Questo impedisce a un valore proxy, autorità di certificazione, endpoint o credenziale memorizzato nella cache di reindirizzare, intercettare o autenticare nuovamente il recupero delle impostazioni che conferma il payload. L'indurimento si applica solo alla cache delle impostazioni recuperate dal server: le [impostazioni gestite dall'endpoint](/docs/it/settings#settings-files) distribuite tramite MDM o `managed-settings.json` non sono interessate. Le categorie trattenute sono:

* Configurazione proxy e TLS, come `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` e le variabili del certificato client mTLS `CLAUDE_CODE_CLIENT_CERT` e `CLAUDE_CODE_CLIENT_KEY`
* Routing API e selezione del provider, inclusi `ANTHROPIC_BASE_URL`, le variabili di selezione del provider come `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, e gli URL dell'endpoint del provider come `ANTHROPIC_BEDROCK_BASE_URL`
* Credenziali di autenticazione, come `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` e `CLAUDE_CODE_OAUTH_TOKEN`

Ogni altra chiave nel blocco `env` memorizzato nella cache, come la telemetria e la configurazione di OpenTelemetry, si applica all'avvio come prima. Una volta che il recupero riesce, le variabili trattenute si applicano per il resto della sessione.

Se la vostra organizzazione ha bisogno di un proxy per raggiungere `api.anthropic.com`, impostatelo nell'ambiente della shell o nelle [impostazioni utente](/docs/it/settings#settings-files) piuttosto che solo nel blocco `env` gestito. Il primo avvio non ha cache, quindi quelle fonti erano già necessarie per il recupero iniziale.

Claude Code applica gli aggiornamenti delle impostazioni automaticamente senza un riavvio, ad eccezione delle impostazioni avanzate come la configurazione di OpenTelemetry, che richiedono un riavvio completo per avere effetto.

<h3 id="invalid-entries-in-delivered-settings">
  Voci non valide nelle impostazioni consegnate
</h3>

I payload consegnati vengono analizzati in modo tollerante con le stesse regole delle altre fonti gestite. Quando un payload contiene una voce che non supera la convalida dello schema, Claude Code rimuove quella voce, visualizza un errore di convalida e applica ogni impostazione valida rimanente. Vedere [Voci non valide nelle impostazioni gestite](/docs/it/settings#invalid-entries-in-managed-settings) per il comportamento a livello di campo, incluso il modo in cui vengono gestiti i campi di applicazione della sicurezza. Richiede Claude Code v2.1.169 o successivo.

La consegna gestita dal server aggiunge questi comportamenti:

* La cache in `~/.claude/remote-settings.json` memorizza il payload salvato con le voci non valide rimosse. Il payload non valido grezzo non viene mai persistito.
* Quando nessun campo nel payload può essere salvato, Claude Code mantiene le ultime impostazioni memorizzate nella cache accettate e registra un errore fatale.
* La [finestra di dialogo di approvazione della sicurezza](#security-approval-dialogs) valuta il payload salvato, quindi una voce non valida rimossa non viene mai presentata per l'approvazione e non viene mai eseguita.

Per eseguire il debug dei problemi di consegna, eseguite `claude --debug-file <path>` e cercate nel log `Remote settings`. Convalidate una modifica del payload con `claude doctor` su una macchina di test prima di distribuirla all'organizzazione.

<h3 id="enforce-fail-closed-startup">
  Applicare l'avvio fail-closed
</h3>

Per impostazione predefinita, se il recupero delle impostazioni remote non riesce all'avvio, la CLI continua senza impostazioni gestite. Per gli ambienti in cui questa breve finestra non applicata è inaccettabile, impostare `forceRemoteSettingsRefresh: true` nelle impostazioni gestite.

Quando questa impostazione è attiva, la CLI si blocca all'avvio fino a quando le impostazioni remote non vengono recuperate di recente. Se il recupero non riesce, la CLI esce piuttosto che procedere senza la politica. Questa impostazione si auto-perpetua: una volta consegnata dal server, viene anche memorizzata nella cache localmente in modo che gli avvii successivi applichino lo stesso comportamento anche prima del primo recupero riuscito di una nuova sessione.

Per abilitare questa funzione, aggiungere la chiave alla configurazione delle impostazioni gestite:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Potete anche impostare questa chiave in un [profilo MDM gestito dall'endpoint](/docs/it/settings#settings-files) o in un file `managed-settings.json` di sistema per applicare il comportamento fail-closed al primo avvio, prima che qualsiasi payload del server sia stato consegnato. A partire dalla v2.1.191, questo flag è un'eccezione alla [regola di precedenza](#settings-precedence) sopra: viene rispettato quando impostato in qualsiasi fonte gestita anche se è presente anche un payload memorizzato nella cache gestito dal server, quindi un valore consegnato da MDM non viene ignorato quando esistono impostazioni gestite dal server.

Le impostazioni fetch invia anche un'intestazione `Cache-Control: no-cache` in modo che i proxy HTTP intermedi non servano una risposta non aggiornata.

Prima di abilitare questa impostazione, assicurarsi che le politiche di rete consentano la connettività a `api.anthropic.com`. Se tale endpoint non è raggiungibile, la CLI esce all'avvio e gli utenti non possono avviare Claude Code.

A partire dalla v2.1.139, i sottocomandi `claude auth` come `claude auth login` sono esenti da questo controllo, in modo che gli utenti possano autenticarsi nuovamente quando le credenziali scadute sono il motivo per cui il recupero delle impostazioni non riesce.

<h3 id="security-approval-dialogs">
  Finestre di dialogo di approvazione della sicurezza
</h3>

Determinate impostazioni che potrebbero comportare rischi di sicurezza richiedono l'approvazione esplicita dell'utente prima che Claude Code le applichi:

* **Impostazioni dei comandi shell**: impostazioni che eseguono comandi shell
* **Variabili di ambiente personalizzate**: variabili non nell'elenco di sicurezza noto
* **Configurazioni di hook**: qualsiasi definizione di hook
* **Contenuto CLAUDE.md gestito**: un valore `claudeMd` consegnato attraverso impostazioni gestite

Quando queste impostazioni sono presenti, gli utenti vedono una finestra di dialogo di sicurezza che spiega cosa viene configurato. Gli utenti devono approvare per procedere. Se un utente rifiuta le impostazioni, Claude Code esce.

<Note>
  Un'esecuzione non interattiva, come `claude -p` o una sessione Agent SDK, non può mostrare la finestra di dialogo. Quando le impostazioni consegnate richiederebbero approvazione, Claude Code le applica solo per quella esecuzione: non le registra come approvate e non le scrive nella [cache locale](#fetch-and-caching-behavior), e la prossima sessione interattiva mostra la finestra di dialogo. Fino a quando un utente non approva in una sessione interattiva, ogni esecuzione non interattiva recupera nuovamente le impostazioni all'avvio. Prima della v2.1.207, un'esecuzione non interattiva salvava le impostazioni come approvate, quindi le sessioni interattive successive non mostravano mai la finestra di dialogo per esse.
</Note>

<h2 id="platform-availability">
  Disponibilità della piattaforma
</h2>

Le impostazioni gestite dal server richiedono una connessione diretta a `api.anthropic.com`, e la consegna richiede che la sessione si autentichi con un accesso OAuth dell'organizzazione o una chiave API configurata direttamente. Le chiavi restituite da uno script [`apiKeyHelper`](/docs/it/settings#available-settings) non attivano il recupero delle impostazioni.

Le impostazioni gestite dal server non sono disponibili quando si utilizzano provider di modelli di terze parti:

* Amazon Bedrock
* Google Cloud's Agent Platform
* Microsoft Foundry
* [Claude Platform on AWS](/docs/it/claude-platform-on-aws)
* Endpoint API personalizzati tramite `ANTHROPIC_BASE_URL` o gateway [LLM](/docs/it/llm-gateway) di terze parti

Se esportate una variabile provider `CLAUDE_CODE_USE_*` o un `ANTHROPIC_BASE_URL` non predefinito nella vostra shell, Claude Code salta il recupero delle impostazioni per le vostre sessioni. Non potete cancellare l'esportazione con un blocco `env` gestito dal server, perché il blocco arriva attraverso il recupero che l'esportazione impedisce. Un blocco `env` con impostazioni gestite dall'endpoint non ripristina il recupero neanche: Claude Code verifica l'idoneità prima di applicare i blocchi `env` gestiti, quindi l'override cambia la selezione del provider della sessione ma il recupero rimane saltato.

Per ripristinare la consegna gestita dal server, rimuovete l'esportazione dalla vostra shell, oppure impostate la variabile su `""` nel blocco `env` delle impostazioni utente, che si applica prima della verifica dell'idoneità. Per applicare la policy senza fare affidamento sugli utenti per modificare le loro shell, consegnate le impostazioni attraverso il canale gestito dall'endpoint.

Per le distribuzioni Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, un [gateway di app Claude](/docs/it/claude-apps-gateway) auto-ospitato fornisce la consegna equivalente di impostazioni gestite da remoto: i client firmati dal gateway recuperano le impostazioni gestite dal gateway invece che da `api.anthropic.com`. La semantica degli errori differisce all'avvio: un client gateway che non riesce a raggiungere il gateway esce con un errore invece di ricadere sulle impostazioni memorizzate nella cache, mentre l'aggiornamento in background orario è fail-open su entrambi i canali.

<h2 id="audit-logging">
  Registrazione di audit
</h2>

Gli eventi del registro di audit per le modifiche alle impostazioni sono disponibili tramite l'API di conformità o l'esportazione del registro di audit. Contattare il vostro team di account Anthropic per l'accesso.

Gli eventi di audit includono il tipo di azione eseguita, l'account e il dispositivo che ha eseguito l'azione, e riferimenti ai valori precedenti e nuovi.

<h2 id="security-considerations">
  Considerazioni sulla sicurezza
</h2>

Le impostazioni gestite dal server forniscono l'applicazione centralizzata dei criteri, ma operano come un controllo lato client, non come un limite di sicurezza. Su dispositivi non gestiti, un utente non ha bisogno dell'accesso amministratore o sudo per aggirarli.

| Scenario                                                                           | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :--------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| L'utente modifica il file di impostazioni memorizzato nella cache                  | Il file manomesso si applica all'avvio, ma le impostazioni corrette si ripristinano al prossimo recupero dal server. A partire dalla v2.1.198, le variabili di ambiente di trasporto, routing API e autenticazione nel blocco `env` sono [trattenute fino a quando il server non conferma il payload](#fetch-and-caching-behavior)                                                                                                                                                                                                                                                   |
| L'utente elimina il file di impostazioni memorizzato nella cache                   | Si verifica il comportamento del primo avvio: le impostazioni vengono recuperate in modo asincrono con una breve finestra non applicata                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| L'utente esegue un binario Claude Code modificato                                  | Un utente che può eseguire un client modificato può aggirare qualsiasi controllo lato client                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| L'utente esegue una versione precedente di Claude Code                             | Le versioni precedenti alle impostazioni gestite dal server non le recuperano o non le applicano                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| L'API non è disponibile                                                            | Le impostazioni memorizzate nella cache si applicano se disponibili, altrimenti le impostazioni gestite non vengono applicate fino al prossimo recupero riuscito. A partire dalla v2.1.198, le variabili di ambiente di trasporto, routing API e autenticazione nel blocco `env` memorizzato nella cache sono [trattenute in caso di errore di recupero](#fetch-and-caching-behavior); il resto della cache si applica comunque. Con `forceRemoteSettingsRefresh: true`, la CLI esce invece di continuare, ad eccezione dei [`claude auth` subcomandi](#enforce-fail-closed-startup) |
| L'utente si autentica con un'organizzazione diversa                                | Le impostazioni non vengono consegnate per gli account al di fuori dell'organizzazione gestita                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| L'utente configura un [provider di modelli di terze parti](#platform-availability) | Le impostazioni gestite dal server vengono ignorate. Questo include l'impostazione di `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, o un `ANTHROPIC_BASE_URL` non predefinito                                                                                                                                                                                                                                                                                                           |
| Il traffico di rete viene intercettato o reindirizzato                             | La convalida TLS disabilitata o il traffico intercettato può alterare le impostazioni che il client riceve                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

Per rilevare le modifiche della configurazione in fase di esecuzione, utilizzare gli [hook `ConfigChange`](/docs/it/hooks#configchange) per registrare le modifiche o bloccare le modifiche non autorizzate prima che abbiano effetto.

Per limitare quali organizzazioni i vostri utenti possono accedere con le credenziali fornite dal client, consultare [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) nel Centro assistenza Claude. Per garanzie di applicazione più forti, utilizzare le [impostazioni gestite dall'endpoint](/docs/it/settings#settings-files) su dispositivi registrati in una soluzione MDM.

<h2 id="see-also">
  Vedere anche
</h2>

Pagine correlate per la gestione della configurazione di Claude Code:

* [Settings](/docs/it/settings): riferimento di configurazione completo incluse tutte le impostazioni disponibili
* [Endpoint-managed settings](/docs/it/settings#settings-files): impostazioni gestite distribuite ai dispositivi dal reparto IT
* [Authentication](/docs/it/authentication): configurare l'accesso degli utenti a Claude Code
* [Security](/docs/it/security): salvaguardie di sicurezza e best practice
