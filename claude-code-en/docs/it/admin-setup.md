> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare Claude Code per la tua organizzazione

> Una mappa decisionale per gli amministratori che distribuiscono Claude Code, che copre i provider API, le impostazioni gestite, l'applicazione delle policy, il monitoraggio dell'utilizzo e la gestione dei dati.

Claude Code applica la policy dell'organizzazione attraverso impostazioni gestite che hanno la precedenza sulla configurazione locale dello sviluppatore. Fornisci queste impostazioni dalla console di amministrazione Claude, dal tuo sistema di gestione dei dispositivi mobili (MDM) o da un file su disco. Le impostazioni controllano quali strumenti, comandi, server e destinazioni di rete Claude può raggiungere.

Questa pagina illustra le decisioni di distribuzione in ordine. Ogni riga si collega alla sezione sottostante e alla pagina di riferimento per quell'area.

<Note>
  SSO, il provisioning SCIM e l'assegnazione dei posti sono configurati a livello di account Claude. Consulta la [Guida dell'amministratore aziendale Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) e l'[assegnazione dei posti](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) per questi passaggi.
</Note>

| Decisione                                                                                   | Cosa stai scegliendo                                            | Riferimento                                                                                                                                                                   |
| :------------------------------------------------------------------------------------------ | :-------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Scegli il tuo provider API](#choose-your-api-provider)                                     | Dove Claude Code si autentica e come viene fatturato            | [Authentication](/docs/it/authentication), [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry) |
| [Decidi come le impostazioni raggiungono i dispositivi](#decide-how-settings-reach-devices) | Come la policy gestita raggiunge le macchine degli sviluppatori | [Server-managed settings](/docs/it/server-managed-settings), [Settings files](/docs/it/settings#settings-files)                                                                         |
| [Decidi cosa applicare](#decide-what-to-enforce)                                            | Quali strumenti, comandi e integrazioni sono consentiti         | [Permissions](/docs/it/permissions), [Sandboxing](/docs/it/sandboxing)                                                                                                                  |
| [Configura la visibilità dell'utilizzo](#set-up-usage-visibility)                           | Come tracciare la spesa e l'adozione                            | [Analytics](/docs/it/analytics), [Monitoring](/docs/it/monitoring-usage), [Costs](/docs/it/costs)                                                                                            |
| [Rivedi la gestione dei dati](#review-data-handling)                                        | Conservazione dei dati e postura di conformità                  | [Data usage](/docs/it/data-usage), [Security](/docs/it/security)                                                                                                                        |

<h2 id="choose-your-api-provider">
  Scegli il tuo provider API
</h2>

Claude Code si connette a Claude attraverso uno dei diversi provider API. La tua scelta influisce sulla fatturazione, l'autenticazione, sulla postura di conformità che erediti e su quali funzionalità di Claude Code i tuoi sviluppatori possono utilizzare.

| Provider                      | Scegli questo quando                                                                                                                         |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude for Teams / Enterprise | Vuoi Claude Code e claude.ai in un'unica sottoscrizione per posto senza infrastruttura da eseguire. Questa è la raccomandazione predefinita. |
| Claude Console                | Sei API-first o vuoi una fatturazione pay-as-you-go                                                                                          |
| Amazon Bedrock                | Vuoi ereditare i controlli di conformità e la fatturazione AWS esistenti                                                                     |
| Google Cloud's Agent Platform | Vuoi ereditare i controlli di conformità e la fatturazione GCP esistenti                                                                     |
| Microsoft Foundry             | Vuoi ereditare i controlli di conformità e la fatturazione Azure esistenti                                                                   |

Alcune funzionalità di Claude Code richiedono un account claude.ai. [Claude Code sul web](/docs/it/claude-code-on-the-web), [Routines](/docs/it/routines), [Code Review](/docs/it/code-review), [Remote Control](/docs/it/remote-control) e l'[estensione Chrome](/docs/it/chrome) non sono disponibili tramite chiavi API Console o credenziali di provider cloud da sole. Se distribuisci tramite Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, pianifica se gli sviluppatori hanno anche bisogno di posti Claude for Teams o Enterprise. Ogni pagina di funzionalità elenca i suoi requisiti di piano.

Per il confronto completo dei provider che copre l'autenticazione, le regioni e la parità delle funzionalità, consulta la [panoramica della distribuzione aziendale](/docs/it/third-party-integrations). La configurazione dell'autenticazione di ogni provider è in [Authentication](/docs/it/authentication).

I requisiti di proxy e firewall in [Network configuration](/docs/it/network-config) si applicano indipendentemente dal provider. Se vuoi un singolo endpoint davanti a più provider o un logging centralizzato delle richieste, consulta [LLM gateway](/docs/it/llm-gateway).

<h2 id="decide-how-settings-reach-devices">
  Decidi come le impostazioni raggiungono i dispositivi
</h2>

Le impostazioni gestite definiscono una policy che ha la precedenza sulla configurazione locale dello sviluppatore. Claude Code controlla le quattro fonti sottostanti in ordine di priorità e applica la prima che restituisce una configurazione non vuota, con un'eccezione: un piccolo insieme di [chiavi di blocco tra fonti](/docs/it/settings#settings-precedence), come i blocchi della lista di autorizzazione della sandbox, viene rispettato quando qualsiasi fonte controllata dall'amministratore li imposta.

| Meccanismo              | Consegna                                                                                                                                                                                            | Priorità | Piattaforme    |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------- | :------------- |
| Server-managed          | Console di amministrazione claude.ai, o un [gateway di app Claude](/docs/it/claude-apps-gateway) self-hosted per accessi tramite gateway                                                                 | Massima  | Tutte          |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                           | Alta     | macOS, Windows |
| File-based managed      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux e WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | Media    | Tutte          |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                 | Minima   | Solo Windows   |

Un [`policyHelper`](/docs/it/settings#compute-managed-settings-with-a-policy-helper) configurato ha la precedenza su tutte e quattro le fonti: il suo output diventa l'unica configurazione gestita per l'esecuzione. Vedi [Settings precedence](/docs/it/settings#settings-precedence).

Le impostazioni gestite dal server raggiungono i dispositivi al momento dell'autenticazione e si aggiornano ogni ora durante le sessioni attive, senza infrastruttura di endpoint. La consegna attraverso la console di amministrazione claude.ai richiede un piano Claude for Teams o Enterprise. Le distribuzioni su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry possono ottenere la stessa consegna remota eseguendo un [gateway di app Claude](/docs/it/claude-apps-gateway), oppure utilizzare uno dei meccanismi basati su file o a livello di sistema operativo.

Se la tua organizzazione mescola provider, configura le [impostazioni gestite dal server](/docs/it/server-managed-settings) per gli utenti di claude.ai più un [fallback basato su file o plist/registry](/docs/it/settings#settings-files) in modo che gli altri utenti ricevano comunque la policy gestita.

Le posizioni del registro plist e HKLM funzionano con qualsiasi provider e resistono alle manomissioni perché richiedono privilegi di amministratore per la scrittura. Il registro utente di Windows in HKCU è scrivibile senza elevazione, quindi trattalo come un default di convenienza piuttosto che come un canale di applicazione.

Per impostazione predefinita, WSL legge solo il percorso del file Linux in `/etc/claude-code`. Per estendere la tua policy del registro Windows e `C:\Program Files\ClaudeCode` a WSL sulla stessa macchina, imposta [`wslInheritsWindowsSettings: true`](/docs/it/settings#available-settings) in una di quelle fonti solo amministratore di Windows.

Qualunque meccanismo tu scelga, i valori gestiti hanno la precedenza sulle impostazioni dell'utente e del progetto. Le impostazioni di array come `permissions.allow` e `permissions.deny` uniscono le voci da tutte le fonti, quindi gli sviluppatori possono estendere gli elenchi gestiti ma non rimuovere da essi. Per [due eccezioni](/docs/it/settings#settings-precedence), `fallbackModel` e `availableModels`, il valore gestito sostituisce i livelli inferiori piuttosto che unirsi.

Consulta [Server-managed settings](/docs/it/server-managed-settings) e [Settings files and precedence](/docs/it/settings#settings-files).

<h3 id="wsl-sessions-in-claude-code-desktop">
  Sessioni WSL in Claude Code Desktop
</h3>

Su Windows, [Claude Code Desktop può eseguire sessioni di Code all'interno di una distribuzione WSL 2](/docs/it/desktop-wsl). Il processo Claude Code della sessione viene eseguito all'interno della distribuzione, quindi risolve le impostazioni gestite attraverso il percorso di discovery WSL sopra: le fonti solo Windows non lo raggiungono a meno che `wslInheritsWindowsSettings: true` non sia distribuito.

Sui dispositivi in cui sono presenti impostazioni gestite, le sessioni WSL Desktop sono disabilitate per impostazione predefinita. Se la tua organizzazione vuole abilitarle, contatta il tuo team di account Anthropic. Quando sono abilitate:

* Distribuisci `wslInheritsWindowsSettings: true` attraverso il registro HKLM o il file `C:\Program Files\ClaudeCode` in modo che le sessioni WSL ereditino la stessa policy delle sessioni host.
* Verifica eseguendo `/status` all'interno di una sessione WSL: la riga `Setting sources` dovrebbe mostrare `Enterprise managed settings` con la fonte Windows che hai distribuito, `(HKLM)` o `(file)`.

I processi all'interno della VM utility WSL 2 non sono visibili ai sensori di rilevamento endpoint lato Windows. Se utilizzi CrowdStrike Falcon, abilita il sensore Falcon per Linux su WSL 2 con le due esclusioni che la documentazione WSL di CrowdStrike richiede, per il processo della macchina virtuale WSL e l'immagine del disco della VM, in modo che l'attività dei processi e dei file all'interno della distribuzione sia osservabile. La [telemetria di esecuzione dello strumento OpenTelemetry](/docs/it/monitoring-usage) di Claude Code viene emessa in modo identico per le sessioni WSL e native.

<h2 id="decide-what-to-enforce">
  Decidi cosa applicare
</h2>

Le impostazioni gestite possono bloccare gli strumenti, l'esecuzione sandbox, limitare i server MCP e le fonti di plugin e controllare quali hook vengono eseguiti. Ogni riga è una superficie di controllo con le chiavi di impostazione che la guidano.

| Controllo                                                                              | Cosa fa                                                                                                                                                                                                                                                                                              | Impostazioni chiave                                                                                             |
| :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| [Permission rules](/docs/it/permissions)                                                    | Consenti, chiedi o nega strumenti e comandi specifici                                                                                                                                                                                                                                                | `permissions.allow`, `permissions.deny`                                                                         |
| [Permission lockdown](/docs/it/permissions#managed-only-settings)                           | Solo le regole di autorizzazione gestite si applicano; disabilita `--dangerously-skip-permissions`                                                                                                                                                                                                   | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                   |
| [Sandboxing](/docs/it/sandboxing)                                                           | Isolamento del filesystem e della rete a livello di sistema operativo con allowlist di domini                                                                                                                                                                                                        | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                             |
| [Managed policy CLAUDE.md](/docs/it/memory#deploy-organization-wide-claude-md)              | Istruzioni a livello di organizzazione caricate in ogni sessione, non possono essere escluse                                                                                                                                                                                                         | File nel percorso della policy gestita                                                                          |
| [MCP server control](/docs/it/managed-mcp)                                                  | Limitare quali server MCP gli utenti possono aggiungere o connettere, o distribuire un set fisso                                                                                                                                                                                                     | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly`, o un file `managed-mcp.json` distribuito |
| [Plugin marketplace control](/docs/it/plugin-marketplaces#managed-marketplace-restrictions) | Limitare quali fonti di marketplace gli utenti possono aggiungere e installare, rifiutare i flag CLI che caricano plugin, agent e server MCP per una singola esecuzione e allowlist quali plugin dei marketplace possono essere suggeriti                                                            | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`, `pluginSuggestionMarketplaces`        |
| [Customization lockdown](/docs/it/settings#strictpluginonlycustomization)                   | Bloccare skills, agents, hooks e server MCP da fonti utente e progetto, in modo che possano provenire solo da plugin o impostazioni gestite                                                                                                                                                          | `strictPluginOnlyCustomization`                                                                                 |
| [Hook restrictions](/docs/it/settings#hook-configuration)                                   | Solo gli hook gestiti vengono caricati; limitare gli URL degli hook HTTP                                                                                                                                                                                                                             | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                  |
| [Login enforcement](/docs/it/settings#available-settings)                                   | Limitare l'accesso interattivo a un metodo specifico o a un'organizzazione Anthropic. Quando impostato, le sessioni autenticate da `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` vengono bloccate all'avvio; le sessioni dei provider cloud non sono interessate                      | `forceLoginMethod`, `forceLoginOrgUUID`                                                                         |
| [Disable agent view](/docs/it/agent-view#how-background-sessions-are-hosted)                | Disattivare `claude agents`, `--bg`, `/background` e il supervisore on-demand                                                                                                                                                                                                                        | `disableAgentView`                                                                                              |
| [Model restrictions](/docs/it/model-config#restrict-model-selection)                        | `availableModels` filtra quali modelli appaiono nel selettore. L'aggiunta di `enforceAvailableModels` vincola anche il modello predefinito selezionato automaticamente. Consulta [surface coverage](/docs/it/model-config#surface-coverage) per come questa impostazione raggiunge la CLI, il web e l'IDE | `availableModels`, `enforceAvailableModels`                                                                     |
| [Version floor](/docs/it/settings)                                                          | Impedire all'aggiornamento automatico di installare al di sotto di un minimo a livello di organizzazione                                                                                                                                                                                             | `minimumVersion`                                                                                                |
| [Required version range](/docs/it/settings)                                                 | Rifiutare di avviarsi completamente quando la versione in esecuzione è al di fuori di un intervallo approvato dall'organizzazione. Più forte di `minimumVersion`, che blocca solo i downgrade                                                                                                        | `requiredMinimumVersion`, `requiredMaximumVersion`                                                              |

Le organizzazioni i cui membri si autenticano tramite claude.ai o l'API Anthropic possono anche governare i modelli senza distribuire impostazioni: le [restrizioni del modello dell'organizzazione](/docs/it/model-config#organization-model-restrictions) disabilitano i singoli modelli, un [modello predefinito dell'organizzazione](/docs/it/model-config#organization-default-model) imposta quale modello le nuove sessioni iniziano, e i [limiti di sforzo dell'organizzazione](/docs/it/model-config#organization-effort-limits) limitano i livelli di sforzo per ruolo. Tutti e tre i controlli richiedono un piano Claude Enterprise. Le restrizioni dei modelli e i limiti di sforzo sono applicati lato server; il modello predefinito è un punto di partenza che gli utenti possono modificare, a meno che l'organizzazione non lo applichi. L'applicazione è disponibile per un set limitato di organizzazioni; chiedi al tuo team di account Anthropic sulla disponibilità. Nessuno di questi controlli raggiunge le sessioni su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o [Claude Platform on AWS](/docs/it/claude-platform-on-aws); su questi provider, utilizza `availableModels` sopra per le restrizioni e la chiave `model` nelle impostazioni gestite per un valore predefinito.

[Claude Code on the web](/docs/it/claude-code-on-the-web) ha la sua propria superficie di amministrazione: nella pagina degli ambienti cloud nelle impostazioni di amministrazione, i proprietari e gli amministratori creano [ambienti condivisi dall'organizzazione](/docs/it/claude-code-on-the-web#organization-shared-environments) che impostano il [livello di accesso alla rete](/docs/it/claude-code-on-the-web#network-access), le variabili di ambiente e lo script di configurazione per le sessioni cloud dei membri, e scelgono l'ambiente predefinito dell'organizzazione.

Le regole di autorizzazione e il sandboxing coprono livelli diversi. Negare WebFetch blocca lo strumento fetch di Claude, ma se Bash è consentito, `curl` e `wget` possono comunque raggiungere qualsiasi URL. Il sandboxing chiude questo divario con un allowlist di domini di rete applicato a livello di sistema operativo.

Per il modello di minaccia che questi controlli difendono, consulta [Security](/docs/it/security).

<h2 id="set-up-usage-visibility">
  Configura la visibilità dell'utilizzo
</h2>

Scegli il monitoraggio in base a ciò che devi segnalare. Le dashboard, le API e i controlli di spesa differiscono tra i piani Claude for Teams o Enterprise e le organizzazioni Claude Console, quindi controlla la colonna Disponibilità prima di pianificare la tua reportistica attorno a una capacità.

| Capacità               | Cosa ottieni                                                                                                                    | Disponibilità                                                                                                                                                                                                                                                               | Da dove iniziare                                      |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| Usage monitoring       | Esportazione OpenTelemetry di sessioni, strumenti e token                                                                       | Tutti i provider                                                                                                                                                                                                                                                            | [Monitoring usage](/docs/it/monitoring-usage)              |
| Analytics dashboard    | Metriche di adozione e contributo con una leaderboard su Teams / Enterprise; metriche di utilizzo e spesa per utente su Console | Teams / Enterprise su [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console su [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                                                                 | [Analytics](/docs/it/analytics)                            |
| Programmatic reporting | Dati di utilizzo e costo per utente tramite un'API                                                                              | [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) per Enterprise, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) per Console                                                | [Costs](/docs/it/costs#manage-costs-for-your-organization) |
| Spend controls         | Limiti di spesa e limiti di velocità                                                                                            | Impostazioni amministratore per Teams / Enterprise, limiti dell'area di lavoro per Console; sui cloud di terze parti, controlli del budget cloud o un [Claude apps gateway](/docs/it/claude-apps-gateway) con [limiti di spesa](/docs/it/claude-apps-gateway-spend-limits) per utente | [Costs](/docs/it/costs#manage-costs-for-your-organization) |

Su Teams e Enterprise, i numeri di utilizzo e spesa per utente provengono dal [rapporto di spesa](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) nelle impostazioni di analisi della tua organizzazione, non dalla dashboard di analisi. I provider cloud espongono la spesa attraverso AWS Cost Explorer, GCP Billing o Azure Cost Management. Per pianificare i budget aziendali su Claude chat, Claude Code e Cowork, consulta la [guida al consumo Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide).

<h2 id="review-data-handling">
  Rivedi la gestione dei dati
</h2>

Sui piani Team, Enterprise, Claude API e provider cloud, Anthropic non addestra i modelli sul tuo codice o sui tuoi prompt. Il tuo provider API determina la conservazione e la postura di conformità.

| Argomento                 | Cosa sapere                                                                                                  | Da dove iniziare                               |
| :------------------------ | :----------------------------------------------------------------------------------------------------------- | :--------------------------------------------- |
| Data usage policy         | Cosa raccoglie Anthropic, quanto a lungo viene conservato, cosa non viene mai utilizzato per l'addestramento | [Data usage](/docs/it/data-usage)                   |
| Zero Data Retention (ZDR) | Nulla archiviato dopo il completamento della richiesta. Disponibile su Claude for Enterprise                 | [Zero data retention](/docs/it/zero-data-retention) |
| Security architecture     | Modello di rete, crittografia, autenticazione, audit trail                                                   | [Security](/docs/it/security)                       |

Se hai bisogno di logging di audit a livello di richiesta o di instradare il traffico in base alla sensibilità dei dati, posiziona un gateway tra gli sviluppatori e il tuo provider: un [Claude apps gateway](/docs/it/claude-apps-gateway) auto-ospitato registra un log di audit per richiesta con identità IdP, oppure utilizza un altro [LLM gateway](/docs/it/llm-gateway). Per i requisiti normativi e le certificazioni, consulta [Legal and compliance](/docs/it/legal-and-compliance).

<h2 id="verify-and-onboard">
  Verifica e onboard
</h2>

Dopo aver configurato le impostazioni gestite, fai eseguire a uno sviluppatore `/status` all'interno di Claude Code. Nella scheda **Status**, la riga `Setting sources` mostra `Enterprise managed settings` seguita dalla fonte tra parentesi, una di `(remote)`, `(plist)`, `(HKLM)`, `(HKCU)` o `(file)`. Consulta [Verify active settings](/docs/it/settings#verify-active-settings).

Condividi queste risorse per aiutare gli sviluppatori a iniziare:

* [Quickstart](/docs/it/quickstart): procedura dettagliata della prima sessione dall'installazione al lavoro con un progetto
* [Common workflows](/docs/it/common-workflows): modelli per attività quotidiane come revisione del codice, refactoring e debug
* [Claude 101](https://anthropic.skilljar.com/claude-101) e [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action): corsi di Anthropic Academy a ritmo autonomo

Per i problemi di accesso, indirizza gli sviluppatori alla [risoluzione dei problemi di autenticazione](/docs/it/troubleshoot-install#login-and-authentication). Le correzioni più comuni sono:

* Esegui `/logout` quindi `/login` per cambiare account
* Esegui `claude update` se l'opzione di autenticazione aziendale è mancante
* Riavvia il terminale dopo l'aggiornamento

Se uno sviluppatore vede "You haven't been added to your organization yet," il suo posto non include l'accesso a Claude Code e deve essere aggiornato nella console di amministrazione.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Con il provider e il meccanismo di consegna scelti, passa alla configurazione dettagliata:

* [Server-managed settings](/docs/it/server-managed-settings): fornire la policy gestita dalla console di amministrazione Claude
* [Settings reference](/docs/it/settings): ogni chiave di impostazione, posizione del file e regola di precedenza
* [Monorepos and large repos](/docs/it/large-codebases): modelli di configurazione per directory per le organizzazioni che distribuiscono in un monorepo
* [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry): distribuzione specifica del provider
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, gestione dei posti e playbook di rollout
