> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Controllare l'accesso ai server MCP per la vostra organizzazione

> Limitare quali server MCP gli utenti possono aggiungere o connettere, o fornire server a ogni utente, con file di configurazione gestiti, impostazioni gestite, allowlist e denylists.

Per impostazione predefinita, chiunque esegua Claude Code può connettere qualsiasi [server MCP](/docs/it/mcp) desideri. Anthropic esamina i connettori rispetto ai suoi [criteri di elenco](https://claude.com/docs/connectors/building/review-criteria) prima di aggiungerli alla [Directory Anthropic](https://claude.ai/directory), ma non esegue audit di sicurezza o gestisce alcun server MCP. Come amministratore, potete limitare quali server vengono eseguiti nella vostra organizzazione, da un set fisso approvato alla disabilitazione completa di MCP, e potete fornire server a ogni utente.

Queste restrizioni coprono i server che Claude Code carica da solo, inclusi i connettori che recupera da claude.ai. I connettori che l'app desktop fornisce alle sue sessioni locali e SSH arrivano in-process e sono governati dalle impostazioni dell'organizzazione claude.ai; [Come i connettori raggiungono Claude Code](/docs/it/mcp#how-connectors-reach-claude-code) mostra quali controlli si applicano ai connettori in ogni tipo di sessione, incluse le sessioni cloud.

Questa pagina copre come:

* [Scegliere un modello](#choose-a-pattern) che corrisponda a quanto controllo avete bisogno
* [Distribuire un set di server fisso con `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), incluso come [disabilitare MCP completamente](#disable-mcp-entirely)
* [Fornire server attraverso impostazioni gestite](#provide-servers-through-managed-settings) mentre gli utenti mantengono i propri
* [Controllare i server con allowlist e denylists](#policy-based-control-with-allowlists-and-denylists)
* [Comunicare agli utenti cosa aspettarsi](#how-restrictions-appear-to-users) quando una restrizione blocca un server
* [Monitorare quali server la vostra organizzazione utilizza effettivamente](#monitor-mcp-usage)

<Note>
  La pagina [Security](/docs/it/security) copre il modello di minaccia MCP e come valutare un server prima di approvarlo. [Decidere cosa applicare](/docs/it/admin-setup#decide-what-to-enforce) copre le restrizioni MCP insieme agli altri controlli amministrativi.
</Note>

<h2 id="choose-a-pattern">
  Scegliere un pattern
</h2>

Claude Code supporta una gamma di livelli di restrizione. Ogni pattern utilizza uno o più dei meccanismi trattati di seguito: `managed-mcp.json` per distribuire un set fisso, l'impostazione gestita `managedMcpServers` per fornire server insieme a quelli aggiunti dagli utenti, e `allowedMcpServers`/`deniedMcpServers` per filtrare ciò che gli utenti configurano.

| Pattern                 | Cosa fa                                                                                                                                                                                                                                       | Configura                                                                                                     |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------ |
| **Disabilita MCP**      | Nessun server si carica, a parte i [server in-process che l'app che ha avviato la sessione registra](#exclusive-control-with-managed-mcp-json) e quelli che [fornisci tramite `managedMcpServers`](#provide-servers-through-managed-settings) | `managed-mcp.json` con una mappa server vuota                                                                 |
| **Distribuzione fissa** | Ogni utente ottiene gli stessi server e non può aggiungerne altri                                                                                                                                                                             | `managed-mcp.json` con i server che desideri                                                                  |
| **Server forniti**      | Ogni utente ottiene i server remoti che elenchi e mantiene i propri                                                                                                                                                                           | `managedMcpServers` nelle impostazioni gestite                                                                |
| **Catalogo approvato**  | Pubblica un elenco di server approvati; gli utenti aggiungono quelli che desiderano, tutto il resto è bloccato                                                                                                                                | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                                      |
| **Solo server plugin**  | Gli utenti non possono aggiungere server tramite `~/.claude.json` o `.mcp.json`; i server plugin si caricano comunque                                                                                                                         | [`strictPluginOnlyCustomization`](/docs/it/settings-reference#strictpluginonlycustomization) con `mcp` nell'elenco |
| **Allowlist soft**      | Applica un allowlist che gli utenti possono ampliare nelle loro impostazioni                                                                                                                                                                  | `allowedMcpServers` senza `allowManagedMcpServersOnly`                                                        |
| **Solo denylist**       | Blocca i server noti come cattivi, consenti tutto il resto                                                                                                                                                                                    | `deniedMcpServers`                                                                                            |
| **Nessuna restrizione** | Gli utenti aggiungono qualsiasi cosa                                                                                                                                                                                                          | Non distribuire alcuna configurazione MCP gestita                                                             |

<Note>
  Claude Code non dispone di un registro MCP server integrato che gli utenti possono sfogliare e installare. Per il pattern catalogo approvato, condividi l'elenco approvato e i suoi comandi `claude mcp add` in un luogo dove i tuoi utenti li troveranno, come un wiki interno, oppure distribuisci i server come plugin tramite un [marketplace plugin gestito](/docs/it/plugins/org#restrict-what-users-can-install) in modo che gli utenti possano sfogliarli e installarli da `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Controllo esclusivo con managed-mcp.json
</h2>

Quando distribuisci un file `managed-mcp.json`, Claude Code carica solo questi server MCP:

* I server che il file definisce
* I server che [fornisci attraverso `managedMcpServers`](#provide-servers-through-managed-settings)
* I server in-process che l'app che ha avviato la sessione registra, come il server proprio dell'estensione VS Code o i [connettori che l'app desktop fornisce](/docs/it/mcp#how-connectors-reach-claude-code)

Gli utenti non possono aggiungere, modificare o utilizzare altri server MCP, inclusi i server forniti dai plugin e i server passati con il [flag CLI `--mcp-config`](/docs/it/cli-reference#cli-flags). Il file sopprime anche i connettori claude.ai che Claude Code recupera da solo, a meno che tu non [li consenta insieme al set gestito](#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="deploy-managed-mcp-json">
  Distribuire managed-mcp.json
</h3>

`managed-mcp.json` è un file autonomo, quindi non può essere fornito attraverso [impostazioni gestite dal server](/docs/it/server-managed-settings). Per fornire server attraverso impostazioni gestite, senza controllo esclusivo, utilizza [`managedMcpServers`](#provide-servers-through-managed-settings).

Qualsiasi processo che può scrivere in un percorso di sistema con privilegi di amministratore può distribuire il file. Su una flotta, di solito avviene attraverso strumenti di gestione dei dispositivi, come Jamf o un profilo di configurazione su macOS, Criteri di gruppo o Intune su Windows, o la gestione della flotta di tua scelta su Linux. Claude Code cerca il file in uno di questi percorsi:

| Piattaforma | Percorso                                                   |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux e WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

Il file utilizza lo stesso formato di un file [`.mcp.json`](/docs/it/mcp#project-scope) del progetto:

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  Autenticarsi con credenziali per utente
</h3>

Qualsiasi utente sulla macchina può leggere questo file, quindi non memorizzare chiavi API o altre credenziali nei blocchi `env`. Passa credenziali per utente con uno di questi:

* [Espansione `${VAR}`](/docs/it/mcp#environment-variable-expansion-in-mcp-json) per leggere i segreti dall'ambiente di ogni utente.
* [OAuth o intestazioni per utente](/docs/it/mcp#authenticate-with-remote-mcp-servers) in modo che ogni utente si autentichi come se stesso.
* [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication) per generare credenziali al momento della connessione.

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  Server passati con `--mcp-config` o `--strict-mcp-config`
</h3>

Quando una sessione riceve server attraverso `--mcp-config` mentre un `managed-mcp.json` che Claude Code può leggere e analizzare è distribuito, ciò che l'utente vede differisce tra una workstation e una sessione cloud:

* Su una workstation, Claude Code esce all'avvio con `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* Nelle [sessioni cloud](/docs/it/claude-code-on-the-web) su un host dove il file è distribuito, come un [runner self-hosted](/docs/it/self-hosted-environments-configuration#mcp-servers), Claude Code si avvia solo con i server gestiti e salta i connettori claude.ai e gli altri server che l'host cloud fornisce attraverso `--mcp-config`. Nulla nella sessione comunica all'utente quali server sono stati omessi. Claude Code li nomina in un avviso su stderr, che un runner self-hosted registra al livello di log `debug`.

Il flag `--strict-mcp-config` chiede di sostituire il set gestito. Se un utente lo passa mentre tale file è distribuito, Claude Code esce all'avvio sia su una workstation che in una sessione cloud.

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  Come allowlist e denylists si applicano al set gestito
</h3>

La denylist può filtrare ulteriormente i server in `managed-mcp.json`:

* `deniedMcpServers` si applica anche ai server gestiti, quindi un server gestito che corrisponde a una voce non verrà caricato.
* La propria `deniedMcpServers` di un utente si unisce dalle sue impostazioni, quindi gli utenti possono bloccare un server gestito per se stessi.

`allowedMcpServers` non si applica ai server in `managed-mcp.json`, con un'eccezione: Claude Code controlla comunque un server la cui definizione utilizza [espansione `${VAR}`](/docs/it/mcp#environment-variable-expansion-in-mcp-json) rispetto all'allowlist, perché la configurazione effettiva di quel server proviene dall'ambiente di ogni utente piuttosto che dal file solo. Prima della v2.1.259, ogni server gestito doveva passare l'allowlist ogni volta che uno era impostato. Vedi [Come viene valutato un server](#how-a-server-is-evaluated) per quali campi attivano il controllo `${VAR}` e l'ordine completo dei controlli.

Se hai utilizzato `allowedMcpServers` per impedire il caricamento di alcuni dei tuoi server `managed-mcp.json`, quei server inizieranno a caricarsi al primo avvio di v2.1.259 o successivo di ogni utente a meno che non utilizzino l'espansione `${VAR}`, senza prompt o avviso: solo `deniedMcpServers` continua a sottrarre da quei server. Aggiungi voci di denylist per loro, o distribuisci un `managed-mcp.json` separato per gruppo, prima che i tuoi utenti eseguano l'upgrade.

<h3 id="validate-the-configuration">
  Convalidare la configurazione
</h3>

Per confermare che il file è in vigore, esegui due controlli su una macchina gestita:

1. `claude mcp list` mostra solo i server in `managed-mcp.json`, più quelli che fornisci attraverso `managedMcpServers`. Due altri risultati significano che qualcosa non va:
   * Se i server propri di un utente appaiono ancora, Claude Code non sta leggendo il file, quindi controlla il percorso e le autorizzazioni sulle sue directory padre.
   * Se i server del file non appaiono e la sezione `MCP config diagnostics` contrassegna la configurazione aziendale come non riuscita a analizzare, Claude Code non può leggere o analizzare il file. Correggi l'errore che quella sezione nomina, quindi fai riavviare Claude Code all'utente.
2. `claude mcp add --transport http test https://example.com/mcp` fallisce con `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. L'URL non deve essere un server reale, poiché il controllo della policy rifiuta il comando prima che qualsiasi cosa venga contattata.

<h3 id="disable-mcp-entirely">
  Disabilitare MCP completamente
</h3>

Distribuisci un `managed-mcp.json` contenente una mappa di server vuota per bloccare ogni server MCP a parte i [server in-process che l'app che ha avviato la sessione registra](#exclusive-control-with-managed-mcp-json):

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` fallisce con l'errore di policy aziendale di cui sopra. I server che gli utenti avevano precedentemente configurato smettono di caricarsi la prossima volta che avviano una sessione, senza avviso che la policy è il motivo. I server che fornisci attraverso `managedMcpServers` si caricano comunque sotto una mappa vuota, quindi lascia quella chiave non impostata anche per disabilitare MCP completamente.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Consentire i connettori claude.ai insieme al set gestito
</h3>

Per impostazione predefinita, la distribuzione di `managed-mcp.json` sopprime i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) che Claude Code recupera da solo, inclusi i connettori che un amministratore ha configurato per l'organizzazione nella console di amministrazione claude.ai. Per caricare quei connettori insieme ai server in `managed-mcp.json`, imposta `"allowAllClaudeAiMcps": true` in una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices).

Con l'impostazione abilitata, Claude Code carica gli stessi connettori claude.ai che caricherrebbe se `managed-mcp.json` non fosse distribuito. [Allowlist e denylists](#policy-based-control-with-allowlists-and-denylists) si applicano comunque a quei connettori, quindi puoi bloccare quelli specifici con `deniedMcpServers`. L'impostazione influisce solo sui connettori claude.ai che Claude Code recupera da solo; i server forniti dai plugin rimangono soppressi.

Le sessioni cloud e le sessioni locali e SSH dell'app desktop ricevono i connettori in un altro modo, descritto in [Come i connettori raggiungono Claude Code](/docs/it/mcp#how-connectors-reach-claude-code). Un `managed-mcp.json` sull'host che esegue una sessione cloud, come un [host runner self-hosted](/docs/it/self-hosted-environments-configuration#mcp-servers), sopprime i connettori di quella sessione indipendentemente dal fatto che tu imposti `allowAllClaudeAiMcps`. Nessun `managed-mcp.json` raggiunge i connettori che l'app desktop fornisce alle sue sessioni locali e SSH.

Claude Code legge `allowAllClaudeAiMcps` solo dai livelli di policy controllati dall'amministratore: impostazioni gestite dal server, una chiave plist distribuita da MDM o una chiave di registro HKLM, o un file `managed-settings.json` di sistema. Posizionarlo nelle impostazioni utente o progetto non ha effetto, quindi gli utenti non possono riabilitare i connettori che il controllo esclusivo ha soppresso.

<h2 id="provide-servers-through-managed-settings">
  Fornire server tramite impostazioni gestite
</h2>

Per fornire a ogni utente un set di server MCP remoti senza assumere il controllo esclusivo di MCP, elencali sotto `managedMcpServers` in una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices): impostazioni gestite dal server, una [policy del gateway Claude apps](/docs/it/claude-apps-gateway-config#what-goes-in-cli), un profilo MDM o una policy del registro, oppure `managed-settings.json`. Gli utenti mantengono i server che aggiungono loro stessi e ricevono i tuoi in aggiunta. Richiede Claude Code v2.1.259 o successivo. I client precedenti ignorano la chiave.

Il valore è un oggetto con chiave per nome del server. Ogni voce ha la stessa forma di un server HTTP o SSE in un file di progetto [`.mcp.json`](/docs/it/mcp#project-scope), inclusi i membri facoltativi `headers` e `oauth` descritti in [Autenticazione con server MCP remoti](/docs/it/mcp#authenticate-with-remote-mcp-servers). Questo esempio fornisce un server di ricerca a cui ogni utente accede con OAuth e un server di record che invia un'intestazione emessa dalla tua organizzazione:

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

Chiunque possa leggere le impostazioni gestite su una macchina, incluso l'utente, può leggere un valore di intestazione che imposti qui. Utilizza una credenziale emessa per quel pubblico intero, oppure ometti `headers` e consenti a ogni utente di accedere con OAuth.

<h3 id="what-an-entry-can-contain">
  Cosa può contenere una voce
</h3>

Claude Code carica una voce solo quando supera ogni controllo di seguito. Elimina una voce che ne fallisce uno, registra un avviso che puoi leggere con `/status` e carica comunque le altre voci:

* `type` è `http` o `sse`. Come in `.mcp.json`, `streamable-http` è accettato come alias per `http`.
* `url` è un URL `https://`. Claude Code rifiuta un URL `http://` semplice, incluso uno che punta a `localhost`.
* La voce non ha un membro `command`, `args`, `env` o `headersHelper`, quindi un documento di impostazioni gestite non nomina mai un programma da eseguire sulla macchina di un utente.
* Nessun valore contiene un riferimento `${VAR}`. Claude Code non espande le variabili di ambiente in queste voci, quindi scrivi valori letterali.
* Il nome del server contiene solo lettere, numeri, trattini e sottolineature, e nessuna chiave o valore contiene caratteri di controllo o formattazione invisibile.

Claude Desktop ha un'impostazione gestita con lo stesso nome il cui valore è un array di una forma di voce diversa, quindi non copiare uno nell'altro. Claude Code non accetta la forma array e registra un avviso invece di caricarlo.

Un gateway Claude apps esegue gli stessi controlli all'avvio; vedi [Server MCP in una policy](/docs/it/claude-apps-gateway-config#mcp-servers-in-a-policy).

<h3 id="how-provided-servers-load">
  Come si caricano i server forniti
</h3>

Queste regole decidono cosa si carica quando un server fornito si sovrappone a un'altra definizione di server o a un'altra impostazione su questa pagina:

* Un server fornito ha la precedenza su un server con lo stesso nome nell'ambito locale, di progetto o utente, e su un server plugin o connettore claude.ai che punta allo stesso URL.
* Se distribuisci anche `managed-mcp.json`, Claude Code carica i suoi server e i server forniti insieme, e la voce del file ha la precedenza quando entrambi definiscono un nome.
* I server forniti continuano a caricarsi quando [`strictPluginOnlyCustomization`](/docs/it/settings-reference#strictpluginonlycustomization) blocca la superficie `mcp`.
* `deniedMcpServers` si applica ai server forniti, incluse le voci dalle impostazioni personali di un utente, quindi un utente può bloccarne uno per se stesso. I server forniti non necessitano di una voce `allowedMcpServers`.

Quando non hai anche distribuito `managed-mcp.json`, i flag per esecuzione mantengono il loro significato:

* Un server che un utente passa con `--mcp-config` con lo stesso nome sostituisce quello fornito per quella esecuzione ed è controllato rispetto a `allowedMcpServers`.
* `--strict-mcp-config` lascia fuori i server forniti insieme a ogni altro server configurato.

Con `managed-mcp.json` distribuito, entrambi i flag si comportano come [Controllo esclusivo con managed-mcp.json](#exclusive-control-with-managed-mcp-json) descrive.

<h3 id="what-users-can-see-and-change">
  Cosa gli utenti possono vedere e modificare
</h3>

Gli utenti non possono modificare o rimuovere un server fornito:

* `claude mcp remove` segnala che il server è fornito dall'organizzazione.
* Quando non hai anche distribuito `managed-mcp.json`, una voce che un utente aggiunge con lo stesso nome viene salvata ma non utilizzata mentre la tua è presente.
* Gli utenti possono comunque disattivare un server fornito per se stessi in [`/mcp`](/docs/it/mcp#disable-a-server-without-removing-it), che elenca i server forniti sotto **Managed MCPs**.

`claude mcp get` e `/mcp` mostrano l'URL di un server fornito solo come host, ad esempio `https://mcp.example.com/…`, e `claude mcp get` mostra i nomi delle intestazioni senza i loro valori.

<h3 id="where-managedmcpservers-applies">
  Dove si applica `managedMcpServers`
</h3>

Claude Code legge `managedMcpServers` dalla fonte gestita che seleziona in [Come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources). Quando quella fonte imposta [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior) su `"merge"`, Claude Code fornisce invece i server da ogni fonte admin, e quando due fonti definiscono lo stesso nome, la voce della fonte con ranking più alto si applica interamente. Non legge mai la chiave dal registro HKCU scrivibile dall'utente, dalle [impostazioni padre che un host di embedding fornisce](/docs/it/managed-settings#parent-settings-from-embedding-hosts), o da file di impostazioni utente, di progetto o locali, dove elimina la chiave con un avviso.

Claude Code non legge la chiave nella scheda Code dell'app Claude Desktop su una distribuzione di terze parti o nelle sessioni Cowork dell'app, perché Claude Desktop fornisce e blocca i server MCP di quelle sessioni stesso. `/status` e `claude doctor` lo dicono quando le tue impostazioni gestite portano la chiave lì.

<h3 id="when-provided-servers-connect">
  Quando i server forniti si connettono
</h3>

Quando `managedMcpServers` arriva tramite impostazioni gestite dal server, i suoi tempi seguono [Comportamento di recupero e caching](/docs/it/server-managed-settings#fetch-and-caching-behavior):

* Su una macchina con impostazioni memorizzate nella cache, Claude Code trattiene la copia memorizzata nella cache di questa chiave fino a quando il server non conferma le impostazioni per la sessione, e attende quella conferma prima di caricare i server MCP. Se la conferma fallisce, la sessione continua senza i server forniti e `/status` dice che sono trattenuti.
* Al primo avvio di una macchina, senza nulla memorizzato nella cache ancora, una sessione interattiva che inizia prima dell'arrivo delle impostazioni connette i server forniti non appena arrivano, e un'esecuzione `claude -p` che è già iniziata può terminare senza di loro.

Con [accesso al gateway](/docs/it/claude-apps-gateway-config#precedence-with-other-managed-sources), Claude Code carica la policy prima dell'inizio della sessione, quindi nessuno dei due casi ritarda o salta i server forniti.

Le sessioni interattive già in esecuzione applicano le tue modifiche alla chiave:

* **Aggiungere un server**: Claude Code lo connette quando arrivano le impostazioni aggiornate, senza un riavvio.
* **Modificare la voce di un server**: quelle sessioni si riconnettono ad esso con la nuova definizione.
* **Rimuovere un server**: una sessione interattiva in esecuzione lo disconnette una volta che legge le impostazioni modificate. Un'esecuzione non interattiva (`-p`) lo mantiene fino alla fine.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Controllo basato su policy con allowlist e denylist
</h2>

Gli allowlist e i denylist filtrano quali server configurati sono autorizzati a caricarsi. Non sono un registro: un server deve comunque essere aggiunto da un utente, un plugin o la tua organizzazione prima che uno dei due elenchi si applichi ad esso.

I server che la tua organizzazione fornisce tramite `managedMcpServers` si caricano senza una voce di allowlist, e [Come viene valutato un server](#how-a-server-is-evaluated) copre i server `managed-mcp.json`. Il denylist si applica a ogni server indipendentemente da dove proviene, ad eccezione delle voci in-process `type: "sdk"`.

Per distribuire server agli utenti, utilizza [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) o [`managedMcpServers`](#provide-servers-through-managed-settings). Entrambi gli elenchi filtrano anche i server passati con il flag CLI [`--mcp-config`](/docs/it/cli-reference#cli-flags), ad eccezione delle voci in-process `type: "sdk"`; `--strict-mcp-config` limita quali file di configurazione si caricano e non aggira nessuno dei due elenchi.

Per rendere l'allowlist autorevole, imposta `allowedMcpServers` e `allowManagedMcpServersOnly: true` insieme in una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices), come impostazioni gestite dal server o un file `managed-settings.json` distribuito.

Il blocco si applica da ogni fonte gestita controllata da admin, quindi un blocco in un file distribuito si applica comunque quando vengono utilizzate anche impostazioni gestite dal server che non menzionano MCP. Mentre il blocco è attivo, l'allowlist gestito proviene dalla fonte admin con il ranking più alto che ne imposta uno. La lettura del blocco e dell'allowlist tra le fonti richiede Claude Code v2.1.273 o successivo.

[Limitare l'allowlist solo alle impostazioni gestite](#restrict-the-allowlist-to-managed-settings-only) mostra la configurazione.

Senza `allowManagedMcpServersOnly`, gli allowlist da ogni ambito di impostazioni si uniscono, incluso il `~/.claude/settings.json` dell'utente, quindi un utente può ampliare ciò che il tuo allowlist consente. I denylist si uniscono da ogni ambito indipendentemente.

<Note>
  `allowManagedMcpServersOnly` è separato da `allowManagedPermissionRulesOnly`, che blocca solo le [regole di autorizzazione](/docs/it/permissions#managed-settings). L'impostazione di quel flag non applica l'allowlist MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Abbinare server per URL, comando o nome
</h3>

`allowedMcpServers` e `deniedMcpServers` sono elenchi di voci. Ogni voce è un oggetto con una singola chiave che identifica i server per il loro URL, il loro comando o il loro nome:

| Chiave          | Corrisponde a                                                                                 | Utilizzare per                                   |
| :-------------- | :-------------------------------------------------------------------------------------------- | :----------------------------------------------- |
| `serverUrl`     | Un URL di server remoto, esatto o con wildcard `*`                                            | Server HTTP e SSE                                |
| `serverCommand` | Il comando esatto e gli argomenti che avviano un server stdio                                 | Server stdio                                     |
| `serverName`    | L'etichetta assegnata dall'utente. Solo corrispondenza esatta; i wildcard non vengono espansi | Entrambi i tipi, ma vedi l'Avvertenza di seguito |

Lasciare `allowedMcpServers` non impostato è diverso dall'impostarlo su un array vuoto:

| Impostazione        | Non impostato (predefinito) | Array vuoto `[]`                                                                             | Popolato                                                                                                    |
| :------------------ | :-------------------------- | :------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers` | Tutti i server consentiti   | Nessun server consentito, a parte [i server dell'organizzazione](#how-a-server-is-evaluated) | Solo i server corrispondenti consentiti, a parte [i server dell'organizzazione](#how-a-server-is-evaluated) |
| `deniedMcpServers`  | Nessun server bloccato      | Nessun server bloccato                                                                       | Server corrispondenti bloccati                                                                              |

Vedi [Voci non valide nelle impostazioni gestite](/docs/it/managed-settings#invalid-entries-in-managed-settings) per sapere cosa accade quando una voce non supera la convalida dello schema.

<Warning>
  Una voce `serverName`, in uno dei due elenchi, non è un controllo di sicurezza. Il nome è l'etichetta che un utente assegna quando esegue `claude mcp add` o modifica un file di configurazione, non il server sottostante, quindi un utente può chiamare qualsiasi server `github`. Per i connettori claude.ai il nome è il nome visualizzato restituito da claude.ai, che può cambiare. Per applicare quali server effettivamente vengono eseguiti, aggiungi voci `serverCommand` o `serverUrl`.
</Warning>

La convalida di `serverName` differisce tra i due elenchi:

* In `deniedMcpServers`, `serverName` accetta qualsiasi stringa non vuota senza spazi iniziali o finali, quindi puoi bloccare i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) per il loro nome visualizzato. Ad esempio, `{ "serverName": "claude.ai Slack" }` blocca il connettore Slack. Preferisci una voce `serverUrl` quando hai bisogno che il deny sia robusto rispetto ai cambi di nome, o quando un nome di connettore collide e ottiene un suffisso ` (N)`.
* In `allowedMcpServers`, `serverName` è limitato a lettere, numeri, trattini e sottolineature. Utilizza `serverUrl` per aggiungere all'allowlist un connettore claude.ai che Claude Code recupera da solo; per i connettori che un host cloud fornisce alle sessioni self-hosted, utilizza invece le voci elencate in [Il traffico dei connettori esce dalla tua rete](/docs/it/self-hosted-environments-deploy#connector-traffic-leaves-your-network).

Per disattivare tutti i connettori claude.ai che Claude Code recupera da solo, vedi [`disableClaudeAiConnectors`](/docs/it/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Come viene valutato un server
</h3>

Prima di caricare un server, incluso uno da `managed-mcp.json`, Claude Code esegue i tre controlli di seguito in ordine. Li esegue di nuovo quando un utente ricollega un server o riattiva uno disabilitato in `/mcp`. I server in-process `type: "sdk"`, che l'[app che ha avviato la sessione registra](/docs/it/mcp#how-connectors-reach-claude-code), saltano tutti e tre.

1. **Unisci gli elenchi.** Le voci di allowlist e denylist da ogni ambito di impostazioni si combinano in un allowlist e un denylist. Quando `allowManagedMcpServersOnly` è `true`, viene mantenuto solo l'allowlist gestito; il denylist si unisce sempre da ogni ambito. Quando è presente più di una fonte gestita, [Le chiavi lette da ogni fonte admin](/docs/it/managed-settings#keys-read-from-every-admin-source) dice quali di esse forniscono gli elenchi dell'ambito gestito.
2. **Controlla il denylist.** Un server che corrisponde a qualsiasi voce del denylist, per URL, comando o nome, viene bloccato. Nulla sostituisce una corrispondenza del denylist.
3. **Controlla l'allowlist.** Se `allowedMcpServers` non è impostato da nessuna parte, ogni server che ha superato il denylist si carica. Se è impostato, ciò a cui il server deve corrispondere dipende dal suo tipo, mostrato nella tabella di seguito.

   I server dell'organizzazione saltano questo controllo: ogni voce `managedMcpServers` e qualsiasi voce `managed-mcp.json` i cui valori non utilizzano l'espansione `${VAR}`. Anche i server integrati lo saltano, come Claude in Chrome, il server `ide` a cui Claude Code si connette in un IDE VS Code o JetBrains in esecuzione, e i server che la CLI stessa configura.

   Un server `managed-mcp.json` che utilizza l'espansione `${VAR}` nel suo comando, argomenti, `env`, URL o intestazioni viene comunque controllato, così come ogni server che un utente, un plugin, `--mcp-config` o claude.ai aggiunge.

| Tipo di server      | Consentito quando corrisponde                                                                                             |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------ |
| Remoto (HTTP o SSE) | Una voce `serverUrl`. Una corrispondenza `serverName` conta solo quando l'allowlist non contiene voci `serverUrl`         |
| Stdio               | Una voce `serverCommand`. Una corrispondenza `serverName` conta solo quando l'allowlist non contiene voci `serverCommand` |

Tre regole di corrispondenza si applicano all'interno di questi controlli:

* **I comandi corrispondono esattamente.** Ogni argomento, in ordine. `["npx", "-y", "server"]` non corrisponde a `["npx", "server"]` o `["npx", "-y", "server", "--flag"]`.
* **I valori `serverCommand` e `serverUrl` si espandono prima della corrispondenza.** Sia la voce della policy che il valore configurato del server passano attraverso l'espansione [`${VAR}` e `${VAR:-default}`](/docs/it/mcp#environment-variable-expansion-in-mcp-json), quindi una voce scritta come `["${HOME}/bin/server"]` corrisponde a una configurazione del server che utilizza lo stesso riferimento o il percorso espanso. Su Windows, fai riferimento a una variabile di ambiente impostata lì, come `${USERPROFILE}` invece di `${HOME}`. I valori `serverName` corrispondono letteralmente e non si espandono mai. I due lati leggono ambienti diversi; [Come si espandono le voci della policy](#how-policy-entries-expand) copre quale e come differiscono le voci di allowlist e denylist.
* **Gli URL supportano wildcard `*`** ovunque nel modello, incluso lo schema. La corrispondenza del nome host non distingue tra maiuscole e minuscole e ignora un punto FQDN finale, quindi `https://Mcp.Example.com/*` corrisponde a `https://mcp.example.com/api`. I percorsi rimangono sensibili alle maiuscole e minuscole.

| Modello                     | Consente                                                                                           |
| :-------------------------- | :------------------------------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Tutti i percorsi su un dominio specifico                                                           |
| `https://mcp.example.com`   | Anche tutti i percorsi su quel dominio. Un modello senza percorso corrisponde a qualsiasi percorso |
| `https://*.example.com/*`   | Qualsiasi sottodominio di `example.com`                                                            |
| `http://localhost:*/*`      | Qualsiasi porta su localhost                                                                       |
| `*://mcp.example.com/*`     | Qualsiasi schema a un dominio specifico                                                            |

<h4 id="how-policy-entries-expand">
  Come si espandono le voci della policy
</h4>

Il valore configurato del server si espande dall'ambiente del processo live, come il resto di `.mcp.json`. Una voce della policy si espande da un ambiente bloccato invece, quindi una variabile impostata da un file di impostazioni di progetto o utente non può cambiare cosa significa una voce di allowlist. Poiché una voce della policy dipende comunque dal valore della shell di avvio per qualsiasi variabile a cui fa riferimento, utilizza URL e comandi letterali per le voci su cui conti per l'applicazione.

| Elenco di voci      | Si espande da                                                                                                                                                                                                           | Espansione che cambierebbe lo schema, l'host o l'ambito del percorso di una voce URL |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `allowedMcpServers` | L'ambiente da cui Claude Code è stato avviato, più i valori `env` dalle impostazioni gestite                                                                                                                            | Claude Code ignora la voce                                                           |
| `deniedMcpServers`  | Lo stesso, e una variabile senza valore di avvio e nessun `:-default` si riempie dalle impostazioni file al di fuori del repository, come impostazioni utente o gestite, che solo allargano ciò che la voce corrisponde | La voce corrisponde comunque                                                         |

Richiede Claude Code v2.1.219 o successivo.

<h3 id="example-configuration">
  Configurazione di esempio
</h3>

La configurazione di seguito configura un allowlist rigido con un denylist. Le righe evidenziate cambiano come viene valutato il resto dell'elenco, e i callout dopo il blocco spiegano ognuno:

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **Riga 3**: la prima voce `serverUrl`. Una volta che ne esiste una, ogni server remoto deve corrispondere a un modello di URL, quindi un utente non può ottenere un server remoto non elencato dandogli un nome consentito.
* **Riga 5**: la prima voce `serverCommand`. Lo stesso effetto per i server stdio, quindi ogni server locale deve corrispondere esattamente a un comando elencato.
* **Riga 11**: una voce `serverName` nel denylist. Le voci del denylist si applicano sempre, quindi qualsiasi server denominato `dangerous-server` viene bloccato indipendentemente dal suo URL o comando.

Una voce `serverName` in questo allowlist non corrisponderebbe mai a nulla, poiché entrambi i tipi di trasporto hanno già voci più rigorose.

Gli accordion di seguito illustrano come un server viene valutato rispetto ad altre combinazioni di allowlist e denylist.

<Accordion title="Allowlist solo URL">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | Server                                               | Risultato                                                    |
  | :--------------------------------------------------- | :----------------------------------------------------------- |
  | Server HTTP a `https://mcp.example.com/api`          | Consentito: corrisponde al modello di URL                    |
  | Server HTTP a `https://api.internal.example.com/mcp` | Consentito: corrisponde al sottodominio wildcard             |
  | Server HTTP a `https://external.example.com/mcp`     | Bloccato: non corrisponde a nessun modello di URL            |
  | Server stdio con qualsiasi comando                   | Bloccato: nessuna voce di nome o comando a cui corrispondere |
</Accordion>

<Accordion title="Allowlist solo comando">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                               | Risultato                                          |
  | :--------------------------------------------------- | :------------------------------------------------- |
  | Server stdio con `["npx", "-y", "approved-package"]` | Consentito: corrisponde al comando                 |
  | Server stdio con `["node", "server.js"]`             | Bloccato: non corrisponde al comando               |
  | Server HTTP denominato `my-api`                      | Bloccato: nessuna voce di nome a cui corrispondere |
</Accordion>

<Accordion title="Allowlist misto di nome e comando">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                                                       | Risultato                                                                                   |
  | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
  | Server stdio denominato `local-tool` con `["npx", "-y", "approved-package"]` | Consentito: corrisponde al comando                                                          |
  | Server stdio denominato `local-tool` con `["node", "server.js"]`             | Bloccato: le voci di comando esistono ma non corrisponde                                    |
  | Server stdio denominato `github` con `["node", "server.js"]`                 | Bloccato: i server stdio devono corrispondere ai comandi quando le voci di comando esistono |
  | Server HTTP denominato `github`                                              | Consentito: corrisponde al nome                                                             |
  | Server HTTP denominato `other-api`                                           | Bloccato: il nome non corrisponde                                                           |
</Accordion>

<Accordion title="Allowlist solo nome">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | Server                                                        | Risultato                                  |
  | :------------------------------------------------------------ | :----------------------------------------- |
  | Server stdio denominato `github` con qualsiasi comando        | Consentito: nessuna restrizione di comando |
  | Server stdio denominato `internal-tool` con qualsiasi comando | Consentito: nessuna restrizione di comando |
  | Server HTTP denominato `github`                               | Consentito: corrisponde al nome            |
  | Qualsiasi server denominato `other`                           | Bloccato: il nome non corrisponde          |
</Accordion>

<Accordion title="Allowlist con override del denylist">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | Server                                          | Risultato                                                                                     |
  | :---------------------------------------------- | :-------------------------------------------------------------------------------------------- |
  | Server HTTP a `https://mcp.example.com/api`     | Consentito: corrisponde al modello di URL dell'allowlist, nessuna corrispondenza del denylist |
  | Server HTTP a `https://staging.example.com/api` | Bloccato: corrisponde a entrambi, ma il denylist ha la precedenza                             |
  | Server HTTP a `https://other.com/mcp`           | Bloccato: non corrisponde all'allowlist                                                       |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Limitare l'allowlist solo alle impostazioni gestite
</h3>

Per rendere l'allowlist gestito l'unico che si applica, imposta `allowManagedMcpServersOnly` nel file di impostazioni gestite:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Quando `allowManagedMcpServersOnly` è `true`, gli allowlist dalle impostazioni utente, progetto e locali vengono ignorati. Il denylist si unisce comunque da ogni ambito di impostazioni, quindi gli utenti possono sempre bloccare i server per se stessi.

<h2 id="how-restrictions-appear-to-users">
  Come le restrizioni appaiono agli utenti
</h2>

Per vedere cosa gli utenti vedono all'avvio quando `managed-mcp.json` è distribuito e la sessione ha anche server `--mcp-config`, consultare [Controllo esclusivo con managed-mcp.json](#exclusive-control-with-managed-mcp-json). Utilizzare questa tabella per riconoscere gli altri rapporti e per comunicare agli utenti cosa aspettarsi prima di implementare una modifica:

| Restrizione                                                                                                             | Cosa vede l'utente                                                                                                           |
| :---------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` è presente e l'utente esegue `claude mcp add`                                                        | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| Il server è su una denylist e l'utente esegue `claude mcp add`                                                          | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| Il server non è su una allowlist e l'utente esegue `claude mcp add`                                                     | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| L'utente esegue `claude mcp remove` su un server da `managedMcpServers`                                                 | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| Un server precedentemente configurato è ora bloccato dalla policy                                                       | Il server scompare da `/mcp` e `claude mcp list`                                                                             |
| Un server viene bloccato mentre una sessione è in esecuzione e l'utente seleziona **Reconnect** o lo riattiva in `/mcp` | [`MCP server <name> is blocked by enterprise managed policy`](/docs/it/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

Quando un server scompare silenziosamente, l'utente non riceve alcun segnale che la policy sia il motivo, quindi comunicare agli utenti interessati quali server sono bloccati quando si implementa una nuova restrizione.

<h2 id="monitor-mcp-usage">
  Monitorare l'utilizzo di MCP
</h2>

Quando [l'esportazione OpenTelemetry](/docs/it/monitoring-usage) è configurata, Claude Code può registrare quali server MCP e strumenti gli utenti invocano. Impostate `OTEL_LOG_TOOL_DETAILS=1` per includere i nomi dei server MCP e degli strumenti negli eventi degli strumenti, quindi aggregateli nel vostro collector per vedere quali server i vostri utenti effettivamente connettono. Vedere [Monitoraggio](/docs/it/monitoring-usage) per configurare l'esportatore e per lo schema completo degli eventi.

<h2 id="configuration-summary">
  Riepilogo della configurazione
</h2>

Ogni file e impostazione che questa pagina copre, cosa controlla e come consegnarlo:

| Superficie                   | Cosa controlla                                                                                                                                                                                                                                                        | Dove si trova                                                                                                                                                                                                                     | Come consegnare                                                                                                                                                                                      |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | Set di server fisso, controllo esclusivo                                                                                                                                                                                                                              | Percorso di sistema: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, o `C:\Program Files\ClaudeCode\`                                                                                                            | MDM, GPO, gestione della flotta, o qualsiasi processo con privilegi di amministratore. Non può essere impostato attraverso impostazioni gestite dal server                                           |
| `managedMcpServers`          | Server remoti forniti a ogni utente insieme ai propri                                                                                                                                                                                                                 | Solo fonti di impostazioni gestite; l'impostazione non ha effetto altrove                                                                                                                                                         | Una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices): impostazioni gestite dal server, una politica gateway, `managed-settings.json`, profilo MDM, o registro HKLM |
| `allowedMcpServers`          | Allowlist di server consentiti                                                                                                                                                                                                                                        | Qualsiasi [ambito di impostazioni](/docs/it/settings#where-settings-live); [Come un server viene valutato](#how-a-server-is-evaluated) dice come gli elenchi da diversi ambiti e fonti gestite si combinano                            | Per l'applicazione, una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices): impostazioni gestite dal server, `managed-settings.json`, profilo MDM, o registro        |
| `deniedMcpServers`           | Denylist di server bloccati                                                                                                                                                                                                                                           | Qualsiasi ambito di impostazioni; [Come un server viene valutato](#how-a-server-is-evaluated) dice come gli elenchi da diversi ambiti e fonti gestite si combinano                                                                | Uguale a `allowedMcpServers`                                                                                                                                                                         |
| `allowManagedMcpServersOnly` | Blocca l'allowlist solo alle fonti gestite                                                                                                                                                                                                                            | Solo fonti di impostazioni gestite; [Chiavi lette da ogni fonte amministrativa](/docs/it/managed-settings#keys-read-from-every-admin-source) dice quali fonti gestite possono attivarla. L'impostazione non ha effetto in altri ambiti | Uguale a `allowedMcpServers`                                                                                                                                                                         |
| `allowAllClaudeAiMcps`       | Carica i connettori claude.ai che Claude Code recupera da solo insieme a `managed-mcp.json`. [Un `managed-mcp.json` sull'host che esegue una sessione cloud sopprime comunque i connettori di quella sessione](#allow-claude-ai-connectors-alongside-the-managed-set) | Solo fonti di impostazioni gestite; l'impostazione non ha effetto altrove                                                                                                                                                         | Uguale a `allowedMcpServers`                                                                                                                                                                         |

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Decidere cosa applicare](/docs/it/admin-setup#decide-what-to-enforce): restrizioni MCP insieme alle regole di autorizzazione, sandboxing e agli altri controlli di amministrazione
* [Connettere Claude Code agli strumenti tramite MCP](/docs/it/mcp): il riferimento MCP completo, inclusi trasporti, ambiti e autenticazione
* [Impostazioni](/docs/it/settings): la gerarchia delle impostazioni e come le impostazioni gestite hanno la precedenza
* [Impostazioni gestite dal server](/docs/it/server-managed-settings): consegnare `allowedMcpServers` e `deniedMcpServers` dalla console di amministrazione di Claude.ai
* [Sicurezza](/docs/it/security): il modello di minaccia che questi controlli difendono
* [Guida dell'amministratore Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, gestione dei posti e playbook di implementazione
