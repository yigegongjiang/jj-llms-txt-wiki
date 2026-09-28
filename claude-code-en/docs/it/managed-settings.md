> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Distribuire impostazioni gestite

> Distribuire impostazioni gestite su ogni macchina dello sviluppatore: meccanismi di consegna per sistema operativo, come Claude Code combina le fonti gestite e come verificare l'applicazione.

Le impostazioni gestite sono le impostazioni che la tua organizzazione distribuisce su ogni macchina dello sviluppatore. Claude Code le applica al di sopra di ogni altro livello, quindi nessun valore utente, progetto, locale o `--settings` le sostituisce, ad eccezione di poche [eccezioni sensibili alla sicurezza](/docs/it/settings#exceptions-to-managed-settings-precedence) dove un valore più restrittivo da un livello inferiore conta ancora.

Questa pagina è per l'amministratore che distribuisce impostazioni gestite o esegue il debug del motivo per cui una non si applica. Per decidere cosa applicare, inizia con la tabella [Decide what to enforce](/docs/it/admin-setup#decide-what-to-enforce). Per il percorso della console claude.ai, vedi [Server-managed settings](/docs/it/server-managed-settings). Per sapere in quale file vanno i valori propri dello sviluppatore, vedi [Settings](/docs/it/settings).

<h2 id="deploy-a-managed-settings-file">
  Distribuire un file di impostazioni gestite
</h2>

Questo è il modo più veloce per mettere una policy su ogni macchina: un file `managed-settings.json`. Se non hai ancora scelto come distribuire le impostazioni gestite, o i tuoi dispositivi sono sotto MDM o gli sviluppatori eseguono sessioni cloud, leggi prima [Choose a delivery mechanism](#choose-a-delivery-mechanism).

<Steps>
  <Step title="Scrivi managed-settings.json">
    Scrivi un `managed-settings.json` che contenga le chiavi che hai deciso di applicare, nella stessa forma JSON di `settings.json`. La tabella [Decide what to enforce](/docs/it/admin-setup#decide-what-to-enforce) elenca le chiavi dietro ogni controllo, e ogni voce nel [settings reference](/docs/it/settings-reference) dice se una fonte gestita può impostarla. Questo file blocca due letture di file, disattiva la modalità bypass e fa sì che Claude Code ignori le regole di autorizzazione da file utente, progetto e locale e da `--allowedTools`:

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Per un esempio più completo che mostra la forma di più chiavi gestite, incluso il metodo di accesso, i modelli, i server MCP e i marketplace, vedi [An organization's managed settings](/docs/it/settings-example#an-organizations-managed-settings).
  </Step>

  <Step title="Posiziona il file su ogni macchina">
    Salva il file come `managed-settings.json` nella directory di sistema per il sistema operativo, utilizzando qualsiasi strumento già posiziona file sulla tua flotta:

    * **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux e WSL**: `/etc/claude-code/managed-settings.json`
    * **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="Conferma che la policy è stata applicata">
    Su una macchina, esegui `/status` all'interno di Claude Code. La riga `Setting sources` mostra `Enterprise managed settings (file)`. Distribuisci al resto della flotta dopo; [Check that a policy is in force](#check-that-a-policy-is-in-force) copre cosa guardare quando la riga manca.
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  Scegli un meccanismo di consegna
</h2>

Il file nei passaggi precedenti è uno dei quattro modi per ottenere impostazioni gestite su una macchina. Ogni meccanismo porta le stesse chiavi di policy di un file `settings.json`, quindi il [settings reference](/docs/it/settings-reference) si applica a tutti loro. Poche chiavi sono legate a fonti particolari, e la riga Scope di ogni voce dice quale:

* **Delivery controls**: [`policyHelper`](/docs/it/settings-reference#policyhelper), [`wslInheritsWindowsSettings`](/docs/it/settings-reference#wslinheritswindowssettings), e [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior)
* **Gateway login keys**: [`forceLoginGatewayUrl`](/docs/it/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/it/settings-reference#gatewayinternalnetworks), e il valore `"gateway"` di [`forceLoginMethod`](/docs/it/settings-reference#forceloginmethod)

Un file di impostazioni gestite, un profilo MDM, o la console claude.ai applica una policy a tutti coloro che raggiunge. Per dare a un gruppo di sviluppatori una policy diversa, distribuisci un file o profilo diverso a quel gruppo; la console claude.ai [non può ancora indirizzare un gruppo](/docs/it/server-managed-settings#current-limitations), mentre un [Claude apps gateway](/docs/it/claude-apps-gateway) auto-ospitato distribuisce impostazioni gestite per gruppo IdP.

Quando più di un meccanismo distribuisce una policy alla stessa macchina, Claude Code per impostazione predefinita ne usa uno e ignora gli altri. [How Claude Code combines managed sources](#how-claude-code-combines-managed-sources) fornisce l'ordine e l'opt-in che applica ogni fonte.

Le righe MDM e file sono insieme chiamate impostazioni gestite da endpoint, perché la policy è archiviata sul dispositivo dello sviluppatore, al contrario della riga server-managed, dove Claude Code la recupera.

Scegli un meccanismo in base a come già gestisci i dispositivi, utilizzando la tabella sottostante.

| Meccanismo                                             | Come lo distribuisci                                                                                                                                                                                                         | Quando Claude Code lo legge                                                                                                                                                                                                                                          | Usalo quando                                                                                     |
| :----------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| [Server-managed settings](/docs/it/server-managed-settings) | Nella console admin claude.ai, o su un [Claude apps gateway](/docs/it/claude-apps-gateway) auto-ospitato                                                                                                                          | Recuperato all'avvio e sottoposto a polling ogni ora; vedi [changes that need approval](#where-and-when-a-policy-applies)                                                                                                                                            | Vuoi un posto per cambiare la policy per un'organizzazione claude.ai senza toccare ogni macchina |
| MDM o policy a livello di sistema operativo            | Come profilo di configurazione macOS o valore di registro Windows `HKLM`, tramite Jamf, Intune, Group Policy, o uno strumento simile; vedi [where each mechanism stores the policy](#where-each-mechanism-stores-the-policy) | Letto all'avvio e controllato per modifiche ogni 30 minuti                                                                                                                                                                                                           | Gestisci già i dispositivi con MDM o Group Policy                                                |
| Basato su file                                         | Come `managed-settings.json` in una directory di sistema su ogni macchina; vedi [where each mechanism stores the policy](#where-each-mechanism-stores-the-policy)                                                            | Letto all'avvio e ricaricato quando un file cambia                                                                                                                                                                                                                   | Macchine senza MDM, host Linux, o immagini che costruisci tu stesso                              |
| HKCU registry, Windows e WSL                           | Come valore di registro Windows `HKCU`; vedi [where each mechanism stores the policy](#where-each-mechanism-stores-the-policy)                                                                                               | Letto all'avvio e controllato per modifiche ogni 30 minuti; Claude Code lo usa solo quando nessun'altra fonte gestita distribuisce una chiave di policy e nessuna [host-supplied parent settings](#let-an-embedding-host-add-policy) fornisce una chiave restrittiva | Non puoi scrivere la chiave a livello di macchina `HKLM`                                         |

I modelli di avvio per Jamf, Iru, Intune e Group Policy si trovano nel [MDM examples repository](https://github.com/anthropics/claude-code/tree/main/examples/mdm).

Per i server MCP gestiti, che distribuisci insieme a uno qualsiasi di questi tramite `managed-mcp.json` o fornisci tramite la chiave [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers), vedi [Managed MCP configuration](/docs/it/managed-mcp).

<h3 id="where-and-when-a-policy-applies">
  Dove e quando una policy si applica
</h3>

Una policy distribuita raggiunge le sessioni dello sviluppatore come segue:

* **Surfaces**: sulla macchina dello sviluppatore, il terminale, le estensioni VS Code e JetBrains, la scheda Code dell'app desktop, e le sessioni [Agent SDK](/docs/it/agent-sdk/typescript) leggono tutte queste fonti. Le sessioni Agent SDK caricano impostazioni gestite anche quando `settingSources` esclude i file utente, progetto e locale.
* **Cloud sessions**: una sessione in un ambiente ospitato da Anthropic non legge un profilo MDM o file del dispositivo, quindi la policy per essa deve provenire da impostazioni server-managed. Una sessione in un [ambiente auto-ospitato](/docs/it/self-hosted-environments) legge anche il file di impostazioni gestite nella sua immagine runner, per impostazione predefinita solo quando le impostazioni server-managed non distribuiscono una chiave di policy, a parte le [chiavi che Claude Code legge da ogni fonte admin](#keys-read-from-every-admin-source). [How Claude Code combines managed sources](#how-claude-code-combines-managed-sources) copre l'opt-in che applica entrambi.
* **Cowork sessions**: [Cowork](https://claude.com/docs/cowork/overview) nell'app Claude Desktop esegue le sue sessioni su Claude Code. In una sessione Cowork, Claude Code non recupera mai impostazioni server-managed dalla console admin claude.ai, anche quando l'utente accede con un account Team o Enterprise, quindi quale policy si applica dipende da dove viene eseguita la sessione:

  * **Sulla macchina dell'utente**: per impostazione predefinita, Claude Code in una sessione Cowork legge la policy MDM o a livello di sistema operativo e il file di impostazioni gestite su quel dispositivo, quindi distribuisci la policy lì.
  * **In una sandbox VM completa**: quando la tua configurazione gestita di Claude Desktop imposta [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox), Claude Code viene eseguito all'interno di una macchina virtuale dove la policy MDM del dispositivo e il file di impostazioni gestite non sono presenti.
  * **Remote Cowork sessions**: queste vengono eseguite su VM gestite da Anthropic, dove Claude Code non ha policy del dispositivo da leggere.

  Ovunque la sessione venga eseguita, claude.ai applica gli elenchi [`strictKnownMarketplaces`](/docs/it/settings-reference#strictknownmarketplaces) e [`blockedMarketplaces`](/docs/it/settings-reference#blockedmarketplaces) della console admin stessa quando chiunque aggiunge un marketplace da un repository git su claude.ai o da **Customize** nella scheda Cowork. [How restrictions work](/docs/it/plugins/org#restrict-what-users-can-install) descrive quel controllo. La tabella [surface coverage](/docs/it/model-config#surface-coverage) confronta Cowork con le altre surface.
* **Running sessions**: la maggior parte dei cambiamenti raggiunge una sessione in esecuzione secondo la pianificazione nella [tabella del meccanismo di consegna](#choose-a-delivery-mechanism), senza un riavvio.
  * I cambiamenti a [`forceRemoteSettingsRefresh`](/docs/it/settings-reference#forceremotesettingsrefresh), [`requiredMinimumVersion`](/docs/it/settings-reference#requiredminimumversion), e [alcune chiavi modificabili dall'utente](/docs/it/settings#when-edits-take-effect) hanno effetto al prossimo avvio della sessione.
  * Una voce [`policyHelper`](/docs/it/settings-reference#policyhelper) nuova o modificata ha effetto al prossimo avvio. Se le impostazioni server-managed oscurano l'helper a quell'avvio, l'helper viene eseguito non appena un fetch segnala che quelle impostazioni sono state rimosse.
* **Changes that need approval**: a parte gli [aggiornamenti che aspettano il prossimo avvio](/docs/it/server-managed-settings#fetch-and-caching-behavior), un cambiamento server-managed a un'impostazione che [ha bisogno di approvazione](/docs/it/server-managed-settings#security-approval-dialogs), come un hook o una variabile `env`, aspetta che lo sviluppatore accetti la finestra di dialogo in una sessione interattiva, e si applica per l'esecuzione corrente in una sessione che un'estensione IDE o l'Agent SDK ospita. Gli altri cambiamenti server-managed si applicano al prossimo polling.
* **Long-lived sessions**: una sessione lasciata aperta per settimane può ancora rimanere indietro rispetto a un rollout. [`requiredMinimumVersion`](/docs/it/settings-reference#requiredminimumversion) blocca un binario obsoleto dall'avvio e non termina una sessione già in esecuzione.

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  Dove ogni meccanismo archivia la policy
</h3>

Le chiavi sono le stesse ovunque, ma ogni meccanismo le archivia in un posto e forma diversi:

* **Server-managed**: i server di Anthropic, o il tuo gateway, contengono la policy. Claude Code mantiene una cache locale che applica all'avvio e [sostituisce ad ogni fetch riuscito](/docs/it/server-managed-settings#security-considerations).
* **macOS configuration profile**: il dominio delle preferenze gestite `com.anthropic.claudecode`. Usa le stesse chiavi di livello superiore di `managed-settings.json`, con impostazioni nidificate come dizionari e liste come array plist.
* **Windows HKLM registry**: il JSON come valore `REG_SZ` o `REG_EXPAND_SZ` denominato `Settings` sotto `HKLM\SOFTWARE\Policies\ClaudeCode`.
* **File-based**: `managed-settings.json`, una directory opzionale `managed-settings.d/`, e `managed-mcp.json` nella directory di sistema: `/Library/Application Support/ClaudeCode/` su macOS, `/etc/claude-code/` su Linux e WSL, e `C:\Program Files\ClaudeCode\` su Windows. Claude Code non legge il percorso Windows legacy `C:\ProgramData\ClaudeCode\managed-settings.json`.
* **Windows HKCU registry**: lo stesso valore `Settings` sotto `HKCU\SOFTWARE\Policies\ClaudeCode`.

<h3 id="split-a-file-based-policy-across-teams">
  Dividi una policy basata su file tra i team
</h3>

Se diversi team possiedono parti di una policy, metti ogni parte nel suo file in `managed-settings.d/`, accanto a `managed-settings.json` nella stessa directory di sistema, invece di modificare un file condiviso.

Claude Code unisce `managed-settings.json` per primo, poi ogni file `*.json` nella directory in ordine alfabetico. Nomina i file con prefissi numerici per controllare l'ordine, come `10-telemetry.json` e `20-security.json`. Claude Code ignora i file nascosti e i file che non terminano in `.json`.

Quando due file impostano la stessa chiave, Claude Code li combina secondo queste regole:

* **Single values**, come `"model": "opus"` o `"cleanupPeriodDays": 7`: il valore del file successivo sostituisce quello precedente
* **Lists**, come `permissions.deny` o `sandbox.network.allowedDomains`: le due liste si combinano, con i duplicati rimossi
* **Nested blocks**, come `env` o `sandbox`: i due blocchi si uniscono chiave per chiave, e ogni chiave all'interno segue queste stesse regole
* **`fallbackModel`**: la catena successiva sostituisce quella precedente completamente
* **[`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces) e [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers)**: una voce successiva con lo stesso nome sostituisce quella precedente completamente
* **[`modelPicker`](/docs/it/settings-reference#modelpicker)**: la lineup successiva sostituisce quella precedente completamente

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Come Claude Code combina le fonti gestite
</h2>

Quando la vostra organizzazione fornisce più di una fonte gestita alla stessa macchina, la chiave [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior) decide cosa Claude Code fa con le altre:

* **`"first-wins"`, l'impostazione predefinita**: Claude Code utilizza la fonte con il ranking più alto che fornisce almeno una chiave di policy e ignora il resto piuttosto che unirle, a parte le chiavi in [Chiavi lette da ogni fonte admin](#keys-read-from-every-admin-source). Claude Code non mostra alcun avviso per le fonti che salta; `/status` [nomina la fonte che ha utilizzato e quelle che ha saltato](#read-the-source-in-/status).
* **`"merge"`**: Claude Code applica ogni fonte admin che fornisce una chiave di policy e le combina per tipo di chiave: sulla maggior parte delle chiavi il valore della fonte con ranking più alto si applica, gli elenchi si uniscono e i blocchi assumono il valore più restrittivo. [Componi ogni fonte gestita](#compose-every-managed-source) dice dove impostare la chiave e come ogni tipo di chiave si combina. Richiede Claude Code v2.1.242 o successivo.

Entrambe le impostazioni classificano le fonti nello stesso modo. Questi termini ricorrono in questa sezione:

* **Chiave di policy**: qualsiasi chiave di impostazioni diversa dalle due chiavi di controllo, [`wslInheritsWindowsSettings`](/docs/it/settings-reference#wslinheritswindowssettings) e [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior). Un file di impostazioni gestite o una policy MDM che contiene solo quelle non conta, e Claude Code passa alla fonte successiva.
* **Fonte admin**: una delle prime tre fonti di seguito. Il registro HKCU è scrivibile dall'utente e non è una.

Claude Code controlla le fonti in questo ordine, dalla priorità più alta alla più bassa:

1. Impostazioni remote, fornite da claude.ai come [impostazioni gestite dal server](/docs/it/server-managed-settings) o da un [gateway di app Claude](/docs/it/claude-apps-gateway). Claude Code recupera questa fonte solo quando la sessione si autentica direttamente all'API di Anthropic con un [accesso o una chiave idonei](/docs/it/server-managed-settings#platform-availability), o accede a un gateway con `/login`. Su altri provider, o quando `ANTHROPIC_BASE_URL` punta da qualche parte diversa dall'API di Anthropic, inizia dalla fonte successiva
2. Policy MDM o a livello di sistema operativo: la plist di macOS o la chiave del registro HKLM
3. File di impostazioni gestite, `managed-settings.d/*.json` e `managed-settings.json` uniti insieme
4. Il registro HKCU, su Windows, e su WSL una volta che il registro HKLM o il file di impostazioni gestite di Windows attiva [`wslInheritsWindowsSettings`](/docs/it/settings-reference#wslinheritswindowssettings) e il valore HKCU lo imposta anche. Claude Code lo legge solo quando nessuna fonte sopra di esso fornisce una chiave di policy e nessuna [impostazione padre fornita dall'host](#let-an-embedding-host-add-policy) fornisce una chiave restrittiva

Questo diagramma mostra la classificazione, con esempi delle chiavi cross-source che Claude Code legge dalle prime tre fonti con entrambe le impostazioni:

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="Diagram showing the four managed settings sources ranked from remote settings at the top through MDM, managed settings files, and the HKCU registry at the bottom. By default the first source with a policy key supplies the policy and the rest are skipped; with managedSourcesBehavior set to merge, every admin source with a policy key contributes, combined by kind of key, and the HKCU registry stays out. A side panel shows that cross-source keys such as the sandbox locks, forceRemoteSettingsRefresh, and the per-variable env merge are read from every admin source, which excludes the HKCU registry." width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="Diagram showing the four managed settings sources ranked from remote settings at the top through MDM, managed settings files, and the HKCU registry at the bottom. By default the first source with a policy key supplies the policy and the rest are skipped; with managedSourcesBehavior set to merge, every admin source with a policy key contributes, combined by kind of key, and the HKCU registry stays out. A side panel shows that cross-source keys such as the sandbox locks, forceRemoteSettingsRefresh, and the per-variable env merge are read from every admin source, which excludes the HKCU registry." width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  Chiavi lette da ogni fonte admin
</h3>

Con l'impostazione predefinita `"first-wins"`, Claude Code legge la maggior parte delle chiavi solo dalla [fonte che ha selezionato](#how-claude-code-combines-managed-sources), e ignora un valore in una fonte con ranking inferiore anche quando la fonte selezionata lascia quella chiave non impostata.

Alcune chiavi funzionano diversamente. Claude Code le legge da ogni fonte admin, quindi una policy MDM con ranking inferiore o un file di impostazioni gestite può comunque impostarle quando la fonte selezionata non lo fa. Claude Code esclude il registro HKCU scrivibile dall'utente da quella scansione; quando HKCU è l'unica fonte e nessun host fornisce impostazioni padre, HKCU si applica come qualsiasi fonte selezionata.

Le chiavi cross-source includono:

* `sandbox.network.allowManagedDomainsOnly` e `sandbox.filesystem.allowManagedReadPathsOnly`: un `true` in qualsiasi fonte admin attiva il blocco. Mentre un blocco è attivo, Claude Code unisce l'elenco di autorizzazione che blocca, `sandbox.network.allowedDomains` insieme alle regole di autorizzazione `WebFetch(domain:...)`, o `sandbox.filesystem.allowRead`, su ogni fonte admin. Senza il blocco, Claude Code tratta l'elenco di autorizzazione come qualsiasi altra chiave, quindi sotto `"first-wins"` l'elenco di autorizzazione di una fonte admin non selezionata viene ignorato
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly`: un `true` in qualsiasi fonte admin attiva il blocco dell'elenco di autorizzazione MCP. Mentre il blocco è attivo, l'elenco `allowedMcpServers` gestito proviene dalla fonte admin con ranking più alto che ne imposta uno. Un elenco gestito dal server sostituisce l'elenco di una fonte inferiore piuttosto che combinarsi con esso.

  Se nessuna fonte admin imposta un elenco, ogni server che supera l'elenco di negazione si carica, a meno che [le impostazioni padre](#let-an-embedding-host-add-policy) forniscano un elenco.

  Senza il blocco, Claude Code legge `allowedMcpServers` dalla fonte gestita che applica, quindi sotto `"first-wins"` l'elenco di una fonte admin non selezionata viene ignorato. Richiede Claude Code v2.1.273 o successivo
* `deniedMcpServers` e [`disableClaudeAiConnectors`](/docs/it/settings-reference#disableclaudeaiconnectors): una voce o un `true` in qualsiasi fonte admin si applica. Richiede Claude Code v2.1.273 o successivo
* I percorsi binari sandbox `sandbox.bwrapPath` e `sandbox.socatPath`
* Il binario sandbox `ripgrep`, [`sandbox.ripgrep`](/docs/it/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` e `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/it/settings-reference#useautomodeduringplan), [`syncClaudeAiSkills`](/docs/it/settings-reference#syncclaudeaiskills), e [`syncClaudeAiPlugins`](/docs/it/settings-reference#syncclaudeaiplugins), dove un `false` da qualsiasi fonte admin disattiva il comportamento. Un `false` nelle impostazioni utente o locali dello sviluppatore lo disattiva anche; ogni chiave può solo negare
* [`enableArtifact`](/docs/it/settings-reference#enableartifact), dove un `false` da qualsiasi fonte admin disattiva lo [strumento Artifact](/docs/it/artifacts). Un `false` nelle impostazioni utente, progetto o locali dello sviluppatore lo disattiva anche, e nessuna fonte lo riattiva; vedere [quali valori di livello inferiore contano ancora](/docs/it/settings#exceptions-to-managed-settings-precedence). Richiede Claude Code v2.1.242 o successivo
* [`maxEffortLevel`](/docs/it/settings-reference#maxeffortlevel), dove il limite più basso in qualsiasi fonte admin si applica. Se uno sviluppatore imposta un limite più basso nelle proprie impostazioni o con `--settings`, Claude Code applica quello; nessuna fonte può aumentare il limite. Richiede Claude Code v2.1.267 o successivo
* Un opt-out del trailer di commit in `attribution`, o nel deprecato `includeCoAuthoredBy`, da qualsiasi livello
* [`forceRemoteSettingsRefresh`](/docs/it/server-managed-settings)
* `env`, unito per variabile su tutte le fonti admin: ogni variabile proviene dalla fonte con priorità più alta che la definisce, quindi le fonti inferiori riempiono le variabili che quelle superiori lasciano non impostate. Alcune variabili seguono le proprie regole; [Eccezioni per chiave su fonti gestite](/docs/it/server-managed-settings#per-key-exceptions-across-managed-sources) nomina ognuna. Richiede Claude Code v2.1.223 o successivo. Prima della v2.1.223, Claude Code applicava solo l'intero blocco `env` della fonte selezionata

Le [chiavi di accesso al gateway](#choose-a-delivery-mechanism) seguono una regola separata. Claude Code non le legge mai dalle impostazioni gestite dal server, quindi mentre le impostazioni gestite dal server sono la fonte selezionata, la fonte admin con ranking più alto sulla macchina che porta una chiave di policy le fornisce comunque. Un valore in una fonte admin classificata al di sotto di quella, o nel registro HKCU, viene ignorato.

Quando una fonte admin imposta `allowManagedMcpServersOnly` o un elenco `allowedMcpServers` e quel valore non è quello in vigore, `/status` e `claude doctor` nominano quella fonte e chiave.

<h3 id="compose-every-managed-source">
  Componi ogni fonte gestita
</h3>

Per fare in modo che Claude Code applichi ogni fonte admin che la vostra organizzazione fornisce, impostare [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior) su `"merge"` nella fonte con ranking più alto che distribuite. Claude Code legge la chiave solo dalla fonte con ranking più alto che porta la chiave o una chiave di policy, quindi una fonte inferiore non può optare per l'unione con la fonte sopra di essa, e una macchina che non riceve mai impostazioni gestite dal server ha bisogno della chiave anche nel suo profilo MDM. Il registro HKCU scrivibile dall'utente non si unisce mai con un'altra fonte. Richiede Claude Code v2.1.242 o successivo.

Sotto `"merge"`, Claude Code aggiunge le voci dell'elenco di una fonte inferiore, come le regole `permissions.allow` e gli hooks, alla policy, quindi attivarlo solo quando ogni fonte classificata al di sotto di quella più alta è sotto il controllo di un amministratore.

Questa tabella mostra come Claude Code combina ogni tipo di chiave sotto `"merge"`. La [voce `managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior) nomina ogni chiave in tre delle righe: elenchi di autorizzazione restrittivi, valori presi nel complesso e chiavi lette solo dalla fonte con ranking più alto.

| Tipo di chiave                                     | Come Claude Code la combina                                                                                                                               | Esempi                                                                                                                       |
| :------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| Elenchi                                            | Combina le voci da ogni fonte                                                                                                                             | `permissions.allow`, `hooks`, `sandbox.network.allowedDomains`, `deniedMcpServers`                                           |
| Blocchi                                            | Applica il valore più restrittivo che qualsiasi fonte imposta; un valore meno restrittivo si applica solo dalla fonte con ranking più alto                | `allowManagedHooksOnly`, `permissions.disableBypassPermissionsMode`, `crossSessionInbound`                                   |
| Elenchi di autorizzazione restrittivi              | Prende l'elenco nel complesso dalla fonte con ranking più alto che lo imposta, senza aggiungere voci da fonti inferiori                                   | `availableModels`, `allowedMcpServers`, `strictKnownMarketplaces`, `allowedChannelPlugins`, e la catena `fallbackModel`      |
| Valori presi nel complesso                         | Prende il valore nel complesso dalla fonte con ranking più alto che lo imposta, senza combinare voci o campi da fonti inferiori                           | `sandbox.credentials.awsPairs`, `sandbox.ripgrep`                                                                            |
| Server MCP forniti                                 | Combina i nomi dei server da ogni fonte; quando due fonti impostano lo stesso nome, applica l'intera voce della fonte con ranking più alto                | `managedMcpServers`                                                                                                          |
| Chiavi lette solo dalla fonte con ranking più alto | Ignora la chiave in ogni fonte inferiore, anche quando la fonte con ranking più alto la lascia non impostata                                              | Helper di credenziali come `apiKeyHelper`, pin di accesso come `forceLoginOrgUUID`, `modelPicker`, `permissions.defaultMode` |
| `env`                                              | Si unisce per variabile su fonti admin con entrambe le impostazioni, come [Chiavi lette da ogni fonte admin](#keys-read-from-every-admin-source) descrive |                                                                                                                              |
| Ogni altra chiave                                  | Prende il valore dalla fonte con ranking più alto che lo imposta                                                                                          | `model`, `cleanupPeriodDays`                                                                                                 |

Per confermare quali fonti si sono combinate su una macchina, [leggere la riga `Setting sources` in `/status`](#read-the-source-in-/status); quella sezione dice cosa significa ogni etichetta.

<h3 id="compute-the-policy-with-a-helper-program">
  Calcola la policy con un programma helper
</h3>

Un [`policyHelper`](/docs/it/settings-reference#policyhelper) è un eseguibile che la vostra policy MDM o il file di impostazioni gestite nomina, e Claude Code lo esegue per calcolare le impostazioni gestite all'avvio. Quando la fonte selezionata ne configura uno e l'helper emette un oggetto `managedSettings`, quell'output cambia cosa Claude Code legge:

* **L'oggetto `managedSettings` emesso è l'unica impostazione gestita per la sessione**, incluso per le [chiavi che altrimenti legge da ogni fonte admin](#keys-read-from-every-admin-source), a parte [`forceRemoteSettingsRefresh`, che ha la sua propria regola di avvio](/docs/it/settings-reference#forceremotesettingsrefresh)

Per quali esecuzioni di helper falliscono, e cosa Claude Code fa quando una lo fa, vedere [Errori dell'helper](/docs/it/settings-reference#helper-failures).

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  Lasciare che un host di embedding aggiunga policy
</h3>

Quando un'altra applicazione avvia Claude Code, come Claude Desktop, un'estensione IDE o un'app Agent SDK, quell'host può passare le proprie impostazioni gestite attraverso l'opzione SDK `managedSettings`. Claude Code chiama queste impostazioni padre.

Per impostazione predefinita, Claude Code ignora le impostazioni padre ogni volta che una fonte admin è presente: impostazioni gestite dal server, una policy MDM o a livello di sistema operativo, o un file di impostazioni gestite.

Per fare in modo che Claude Code unisca le impostazioni padre insieme a una fonte admin, impostare [`parentSettingsBehavior`](/docs/it/settings-reference#parentsettingsbehavior) su `"merge"` nella fonte gestita con priorità più alta; Claude Code legge la chiave solo da quella fonte.

Claude Code quindi mantiene solo i valori dell'host che limitano quello che Claude può fare, con un gap da conoscere: a meno che non impostiate anche i blocchi `allowManaged*Only`, le regole di autorizzazione di permesso dell'host e gli elenchi di autorizzazione sandbox si applicano comunque. Vedere [Limitare le impostazioni padre](/docs/it/claude-apps-gateway#restrict-parent-settings) per i blocchi.

Un [`policyHelper`](/docs/it/settings-reference#policyhelper) può disattivare l'unione padre indipendentemente da questa chiave; la sua voce dice quando.

Claude Code applica anche questi controlli ai valori forniti dal padre da soli:

* Quando qualsiasi fonte admin imposta `allowManagedPermissionRulesOnly`, Claude Code elimina le [regole di autorizzazione di permesso fornite dal padre](/docs/it/claude-apps-gateway#restrict-parent-settings) e `additionalDirectories` mentre le legge, anche quando una fonte con priorità più alta lascia la chiave non impostata. L'effetto della chiave sulle vostre proprie regole di permesso proviene dalle impostazioni gestite che Claude Code applica, o dalle impostazioni padre che avete scelto di unire
* Claude Code applica il valore `forceLoginOrgUUID` o `allowedMcpServers` nelle impostazioni gestite che applica e blocca uno fornito dal padre. Al di fuori del blocco dell'elenco di autorizzazione MCP, un valore in una fonte admin inferiore che Claude Code non applica né si applica né blocca quello del padre. Fuori dal blocco dell'elenco di autorizzazione MCP, un valore in una fonte admin inferiore che Claude Code non applica né si applica né blocca quello del padre.

  Su Claude Code v2.1.273 o successivo, mentre `allowManagedMcpServersOnly` è attivo, l'elenco `allowedMcpServers` dalla fonte admin con ranking più alto che ne imposta uno si applica e blocca quello del padre, come una [chiave cross-source](#keys-read-from-every-admin-source). L'elenco del padre si applica solo quando nessuna fonte admin ne imposta uno. La voce [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior) dice quale fonte fornisce ogni chiave sotto `"merge"`. Prima della v2.1.223, un valore in qualsiasi fonte admin bloccava quello del padre
* Per `availableModels`, Claude Code applica il valore nelle impostazioni gestite che applica e blocca un elenco fornito dal padre
* Per `strictKnownMarketplaces`, Claude Code applica allo stesso modo l'elenco nelle impostazioni gestite che applica e blocca uno fornito dal padre. L'elenco del padre si applica solo quando nessuna fonte gestita applicata ne imposta uno. Richiede Claude Code v2.1.282 o successivo
* Un `blockedMarketplaces` fornito dal padre si applica in aggiunta a qualsiasi blocklist che una fonte gestita imposta. Richiede Claude Code v2.1.282 o successivo

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  Mantenere l'accesso alla cartella Cowork quando si applicano solo regole gestite
</h4>

[Cowork](https://claude.com/docs/cowork/overview) nell'app Claude Desktop esegue le sue sessioni su Claude Code e concede a ogni sessione l'accesso alle sue cartelle di lavoro, come la cartella che l'utente connette, attraverso regole di autorizzazione che fornisce quando avvia la sessione. Quando la vostra policy gestita imposta [`allowManagedPermissionRulesOnly`](/docs/it/settings-reference#allowmanagedpermissionrulesonly), Claude Code mantiene solo le regole di autorizzazione nella policy gestita: elimina le regole di autorizzazione che un host fornisce come impostazioni padre, come `--allowedTools`, o in un file di impostazioni, quindi le scritture in quelle cartelle perdono la loro pre-approvazione. In una sessione Cowork che chiede prima delle modifiche, Cowork non può mostrare il prompt, e Claude segnala ogni scrittura come bloccata perché il percorso si risolve in una posizione protetta o in un percorso al di fuori della cartella connessa.

Per ripristinare le scritture, aggiungere regole di autorizzazione per quelle cartelle alla fonte gestita che Claude Code [seleziona](#precedence-within-the-managed-tier) su quelle macchine: su una flotta gestita da MDM, è la policy MDM piuttosto che un file di impostazioni gestite separato. Questo esempio utilizza il modulo file, e una policy MDM prende le stesse chiavi. Mantiene `allowManagedPermissionRulesOnly` impostato e consente modifiche sotto una cartella `CoworkProjects` nella directory home di ogni utente; sostituire il percorso con le cartelle che i vostri utenti connettono:

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

Dopo aver distribuito la policy, Claude può salvare file sotto quella cartella in una nuova sessione Cowork. [Leggere e modificare le regole](/docs/it/permissions#read-and-edit) coprono la sintassi del percorso, incluso il modulo `//` per i percorsi assoluti.

<h3 id="what-a-developer-can-change">
  Cosa uno sviluppatore può cambiare
</h3>

I file di impostazioni propri di uno sviluppatore, i valori `--settings` e i file di progetto non sostituiscono mai un valore gestito; le [eccezioni](/docs/it/settings#exceptions-to-managed-settings-precedence) permettono solo a un valore di livello inferiore più restrittivo di contare. Questi casi si trovano al di fuori di quella regola:

* **Il modello per una sessione**: un `model` gestito è un predefinito, non un blocco. `--model` e `ANTHROPIC_MODEL` scelgono comunque il modello per quella sessione, quindi distribuire [`availableModels`](/docs/it/settings-reference#availablemodels) per limitare la scelta.
* **Diritti admin locali**: uno sviluppatore che è un amministratore sulla macchina può modificare la fonte gestita stessa, motivo per cui gli strumenti MDM possono ridistribuire il profilo o il file su una pianificazione e motivo per cui il registro HKLM e il dominio delle preferenze gestite di macOS esistono.
* **La cache gestita dal server**: le impostazioni gestite dal server provengono dai server di Anthropic, e una modifica alla cache locale [dura solo fino al prossimo recupero riuscito](/docs/it/server-managed-settings#security-considerations).
* **Altri strumenti**: le impostazioni gestite vincolano solo Claude Code. Uno sviluppatore che chiama l'API da un altro strumento non è sotto di esse.

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  Verifica che una policy sia in vigore
</h2>

Uno sviluppatore segnala che una policy non si applica, o vuoi confermare che un rollout è arrivato prima di spingerlo alla flotta. Due comandi su quella macchina lo rispondono: `/status` mostra quale fonte gestita Claude Code ha selezionato, e `claude doctor` elenca cosa ha eliminato.

<h3 id="read-the-source-in-/status">
  Leggi la fonte in /status
</h3>

Sulla macchina dello sviluppatore, esegui `/status` all'interno di Claude Code e leggi la riga `Setting sources`. Quando una fonte gestita è in vigore, la riga elenca `Enterprise managed settings` con la fonte che Claude Code ha selezionato tra parentesi:

* `(remote)`: impostazioni server-managed da claude.ai o un gateway
* `(plist)` o `(HKLM)`: una policy MDM o del sistema operativo
* `(file)`, `(drop-ins)`, o `(file + drop-ins)`: `managed-settings.json`, la directory drop-in, o entrambi
* `(remote + file, merged)`, o un altro elenco che termina in `, merged`: la tua organizzazione [compone ogni fonte gestita](#compose-every-managed-source), e Claude Code ha unito le fonti elencate nella policy. Una fonte inferiore può ancora fornire variabili `env` senza apparire nell'elenco. Richiede Claude Code v2.1.242 o successivo
* `(HKCU)`: il fallback del registro scrivibile dall'utente
* `(parent process)`: un [host di embedding](#let-an-embedding-host-add-policy) ha fornito impostazioni restrittive
* `(helper)`: un [`policyHelper`](/docs/it/settings-reference#policyhelper) configurato dalla fonte MDM o file selezionata

Quando Claude Code ha trovato una fonte gestita sulla macchina e non l'ha selezionata, una seconda riga, `Skipped sources`, nomina ogni tale fonte. Leggila per distinguere una policy che non ha mai raggiunto la macchina da una che l'ha raggiunta e che una fonte con priorità più alta ha ignorato. Richiede Claude Code v2.1.242 o successivo.

Quando la policy non si applica, la riga `Setting sources` ti dice quale dei due problemi hai:

* **La riga manca**: Claude Code non ha trovato alcuna fonte gestita che distribuisca una chiave di policy.

  Se hai distribuito un file di impostazioni gestite, controlla che si trovi al percorso per il sistema operativo e che contenga una [chiave di policy](#how-claude-code-combines-managed-sources) piuttosto che solo le chiavi di controllo. Un file che non è JSON valido non produce questo stato; Claude Code [rifiuta di avviarsi](#find-entries-claude-code-dropped) invece.

  Quando hai distribuito tramite impostazioni server-managed, esegui `claude doctor`, che segnala l'[esito del fetch](/docs/it/server-managed-settings#verify-settings-delivery).
* **La riga nomina una fonte diversa da quella che hai distribuito**: una fonte con priorità più alta è presente e Claude Code ha ignorato la tua, e `Skipped sources` l'elenca. [How Claude Code combines managed sources](#how-claude-code-combines-managed-sources) fornisce l'ordine.

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Trova voci che Claude Code ha eliminato
</h3>

Quando un file di impostazioni gestite, un profilo MDM, un valore di registro, o un payload server-managed non supera la convalida dello schema, Claude Code prima salta le singole voci che può riparare, come una regola di autorizzazione non valida, con un avviso per ognuna, quindi elimina qualsiasi chiave di livello superiore il cui valore ancora non supera e continua a applicare ogni chiave valida rimanente.

Claude Code è più rigoroso con il `managedSettings` che un [`policyHelper`](/docs/it/settings-reference#policyhelper) emette: fa le stesse riparazioni di voce, ma qualsiasi violazione dello schema che sopravvive fa fallire l'intera esecuzione dell'helper, e all'avvio Claude Code rifiuta di avviarsi, lo stesso che per un helper che esce con non-zero.

Quando un file di impostazioni gestite, un file drop-in, un plist MDM, o un valore di registro HKLM è presente ma non può essere analizzato come un oggetto JSON, Claude Code rifiuta di avviarsi e stampa [un errore che nomina la fonte](/docs/it/errors#managed-settings-document-could-not-be-parsed), anche quando un'altra fonte admin distribuisce una policy valida. Ogni fonte fallisce in questo modo quando:

* **File di impostazioni gestite o file drop-in**: il file non è JSON valido, o il suo livello superiore non è un oggetto
* **MDM plist**: il `plutil` di macOS segnala il plist malformato, o il suo contenuto convertito non è un oggetto JSON
* **Valore di registro HKLM**: il valore `Settings` non è una stringa, è vuoto, o non contiene un oggetto JSON

Tre stati di fonte non causano questo rifiuto:

* Un file, profilo, o valore di registro assente non è un fallimento; Claude Code viene eseguito senza quella fonte.
* Un file di impostazioni gestite vuoto conta come `{}`.
* Un valore malformato nella chiave di registro HKCU scrivibile dall'utente non blocca mai l'avvio. Claude Code lo segnala come un avviso in `/status` e `claude doctor` invece.

Se un file di impostazioni gestite, un file drop-in, o una directory `managed-settings.d/` non può essere letto e nessuna fonte admin fornisce una policy, le sessioni firmate con credenziali claude.ai o Claude Console escono all'avvio con un messaggio per contattare un amministratore.

Per trovare una voce eliminata, guarda in uno di tre posti:

* Le sessioni interattive mostrano una finestra di dialogo all'avvio che elenca le voci non valide.
* Le esecuzioni non interattive con `-p` stampano un riepilogo su stderr.
* [`claude doctor`](/docs/it/debug-your-config) elenca ogni voce non valida con la sua fonte e campo.

<h4 id="keys-that-fail-closed">
  Chiavi che falliscono chiuse
</h4>

Poche chiavi di applicazione non vengono eliminate quando non valide. Claude Code applica un fallback più restrittivo fino a quando il valore non viene corretto; la tabella mostra cosa applica per ogni chiave:

| Campo                         | Comportamento quando presente ma non valido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | Applicato come un allowlist vuoto fino a quando il valore non viene corretto, quindi nessun server MCP che gli utenti aggiungono viene ammesso. I server che la tua organizzazione distribuisce tramite [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers) si caricano ancora, e i server `managed-mcp.json` si caricano per [How a server is evaluated](/docs/it/managed-mcp#how-a-server-is-evaluated). Una voce non valida individuale viene rimossa e il sottoinsieme valido viene applicato.                                        |
| `allowedHttpHookUrls`         | Claude Code applica un [allowlist](/docs/it/settings-reference#allowedhttphookurls) gestito vuoto fino a quando non correggi il valore, quindi un hook HTTP viene eseguito solo se un altro file di impostazioni elenca il suo URL. Se solo una voce individuale non è valida, Claude Code rimuove quella voce e applica il resto.                                                                                                                                                                                                                    |
| `httpHookAllowedEnvVars`      | Claude Code applica un [allowlist](/docs/it/settings-reference#httphookallowedenvvars) gestito vuoto fino a quando non correggi il valore, quindi una variabile di intestazione viene interpolata solo se un altro file di impostazioni la nomina. Se solo una voce individuale non è valida, Claude Code rimuove quella voce e applica il resto.                                                                                                                                                                                                     |
| `allowedChannelPlugins`       | Claude Code applica un allowlist vuoto fino a quando non correggi il valore, quindi nessun plugin di canale passato a `--channels` viene ammesso. Se solo una voce individuale non è valida, rimuove quella voce e applica il resto.                                                                                                                                                                                                                                                                                                             |
| `strictKnownMarketplaces`     | Applicato come un allowlist vuoto fino a quando il valore non viene corretto, quindi nessuna [fonte marketplace](/docs/it/plugins/org#restrict-what-users-can-install) viene ammessa. Una voce individuale che non è valida o non può essere applicata, come una regex `hostPattern` che non si compila, viene rimossa e il sottoinsieme valido viene applicato.                                                                                                                                                                                      |
| `allowManagedHooksOnly`       | Trattato come `true` fino a quando non viene corretto: le [restrizioni hook](/docs/it/settings-reference#allowmanagedhooksonly) si applicano e, a meno che `disableCommandPluginSources` non sia esplicitamente `false`, i plugin sourced da comando sono disabilitati.                                                                                                                                                                                                                                                                               |
| `allowManagedMcpServersOnly`  | Trattato come `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disableCommandPluginSources` | Trattato come `true`, quindi i plugin sourced da comando rimangono disabilitati fino a quando il valore non viene corretto.                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `disableSideloadFlags`        | Trattato come `true` fino a quando il valore non viene corretto, con gli effetti elencati per [`disableSideloadFlags`](/docs/it/settings-reference#disablesideloadflags).                                                                                                                                                                                                                                                                                                                                                                             |
| `availableModels`             | Applicato come un allowlist vuoto fino a quando non viene corretto, quindi solo il modello Default è disponibile; una voce non stringa viene rimossa e il sottoinsieme valido viene applicato.                                                                                                                                                                                                                                                                                                                                                   |
| `enforceAvailableModels`      | Trattato come `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `syncClaudeAiPlugins`         | Trattato come `false`, quindi la sincronizzazione dei [plugin claude.ai](/docs/it/settings-reference#syncclaudeaiplugins) è disattivata fino a quando il valore non viene corretto.                                                                                                                                                                                                                                                                                                                                                                   |
| `forceLoginOrgUUID`           | Nessuna organizzazione è autorizzata ad accedere fino a quando il valore non viene corretto.                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `gatewayInternalNetworks`     | Quando il valore non valido proviene dalla fonte gestita più alta sulla macchina, `/login` rifiuta ogni nuovo accesso [cloud gateway](/docs/it/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) su quella macchina fino a quando il valore non viene corretto.                                                                                                                                                                                                                                                                    |
| `crossSessionInbound`         | Trattato come `refuse`, il valore più restrittivo, quindi i [messaggi cross-session](/docs/it/cross-session-messaging#control-inbound-messages) in entrata vengono rifiutati fino a quando il valore non viene corretto. Lo sviluppatore vede [un avviso](/docs/it/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse).                                                                                                                                                                                                                          |
| `deniedMcpServers`            | Una voce non valida individuale viene rimossa e il sottoinsieme valido viene applicato. Un valore completamente non valido viene eliminato con un avviso, poiché negare ogni server bloccherebbe i server che la policy non ha mai nominato.                                                                                                                                                                                                                                                                                                     |
| `blockedMarketplaces`         | Una voce non valida individuale viene rimossa e il sottoinsieme valido viene applicato. Una voce che si analizza ma non può mai corrispondere, come una regex `hostPattern` che non si compila, viene mantenuta con un avviso. Non blocca nulla fino a quando non viene corretto, ma le [restrizioni marketplace](/docs/it/plugins/org#restrict-what-users-can-install) rimangono attive. Un valore completamente non valido viene eliminato con un avviso, poiché bloccare ogni marketplace bloccherebbe le fonti che la policy non ha mai nominato. |
| `sandbox.credentials`         | Una voce non valida recuperabile viene degradata a `mode: "deny"` con un avviso; una non recuperabile viene rimossa; le voci valide rimangono applicate. Vedi [invalid credential entries](/docs/it/settings-reference#invalid-credential-entries-in-managed-settings)                                                                                                                                                                                                                                                                                |

`allowedHttpHookUrls` e `httpHookAllowedEnvVars` si uniscono tra i file di impostazioni, quindi le voci nelle tue impostazioni utente, progetto o locale si applicano ancora mentre l'elenco gestito è vuoto.

I fallback per queste due chiavi e per `allowedChannelPlugins` richiedono Claude Code v2.1.267 o successivo; le versioni precedenti eliminano l'intera chiave quando il suo valore o qualsiasi voce non è valida. I fallback per `strictKnownMarketplaces`, `blockedMarketplaces`, e `disableSideloadFlags` richiedono Claude Code v2.1.277 o successivo; le versioni precedenti eliminano l'intera chiave quando il suo valore o qualsiasi voce non è valida.

`requiredMinimumVersion` e `requiredMaximumVersion` falliscono aperti per design: un valore non valido viene eliminato piuttosto che applicato.

Questa tolleranza si applica solo alle impostazioni gestite. I file di impostazioni utente, progetto e locale rimangono rigorosi: un file il cui JSON o forma di livello superiore non supera la convalida viene rifiutato completamente e segnalato, e una voce individuale che fallisce, come una regola di autorizzazione malformata, viene saltata con un avviso mentre il resto del file si applica.

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  Chiavi che solo una fonte gestita può impostare
</h2>

Claude Code legge le seguenti chiavi solo da una fonte gestita; posizionarle nei file di impostazioni utente o progetto non ha effetto.

La maggior parte di loro sono lock: il valore che un lock governa, come regole di autorizzazione o `sandbox.network.allowedDomains`, è una chiave ordinaria che qualsiasi livello può impostare, e il lock dice a Claude Code di onorare solo il valore gestito.

La tabella copre i controlli di autorizzazione, plugin e consegna. Per qualsiasi chiave non elencata qui, la colonna Scope del [settings reference](/docs/it/settings-reference#all-settings) index dice se è managed-only; le chiavi managed-only rimanenti lì includono l'URL di accesso al gateway, versione, browser, mobile-simulator, host SSH, Desktop local-session, percorso binario sandbox, prezzo del modello, e controlli CLAUDE.md.

| Impostazione                                                                                                          | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/it/settings-reference#allowallclaudeaimcps)                                                 | Carica i connettori claude.ai che Claude Code recupera da solo insieme a un `managed-mcp.json` distribuito invece di sopprimerli                                                                                                                                                                                                                                                                                                                                                                                                                           |
| [`allowedChannelPlugins`](/docs/it/settings-reference#allowedchannelplugins)                                               | Allowlist di plugin di canale che possono inviare messaggi. Sostituisce l'allowlist Anthropic predefinito quando impostato. Richiede `channelsEnabled: true`. Vedi [Restrict which channel plugins can run](/docs/it/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                                                                                                                                                           |
| [`allowManagedHooksOnly`](/docs/it/settings-reference#allowmanagedhooksonly)                                               | Quando `true`, limita quali hook vengono eseguiti; vedi [what runs under `allowManagedHooksOnly`](/docs/it/settings-reference#what-runs-under-allowmanagedhooksonly) per l'elenco completo degli effetti                                                                                                                                                                                                                                                                                                                                                        |
| [`allowManagedMcpServersOnly`](/docs/it/settings-reference#allowmanagedmcpserversonly)                                     | Quando `true`, solo `allowedMcpServers` dalle impostazioni gestite sono rispettati. `deniedMcpServers` si unisce ancora da tutte le fonti. Vedi [Keys read from every admin source](#keys-read-from-every-admin-source) per quali fonti gestite possono impostarla, e [Managed MCP configuration](/docs/it/managed-mcp)                                                                                                                                                                                                                                         |
| [`allowManagedPermissionRulesOnly`](/docs/it/settings-reference#allowmanagedpermissionrulesonly)                           | Rende le impostazioni gestite l'unica fonte di impostazioni delle regole di autorizzazione. La voce elenca ogni fonte che ignora                                                                                                                                                                                                                                                                                                                                                                                                                           |
| [`blockedMarketplaces`](/docs/it/settings-reference#blockedmarketplaces)                                                   | Blocklist di fonti di marketplace. Le fonti bloccate vengono controllate prima del download, quindi non toccano mai il filesystem. Vedi [managed marketplace restrictions](/docs/it/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                |
| [`channelsEnabled`](/docs/it/settings-reference#channelsenabled)                                                           | Consenti [channels](/docs/it/channels) per l'organizzazione. Vedi [enterprise controls](/docs/it/channels#enterprise-controls) per il default su ogni piano                                                                                                                                                                                                                                                                                                                                                                                                          |
| [`disableCommandPluginSources`](/docs/it/settings-reference#disablecommandpluginsources)                                   | Quando `true`, blocca completamente le [fonti plugin `command`](/docs/it/plugins/marketplace-reference#command-plugin-source), quindi il comando dichiarato dal marketplace non viene mai eseguito. Blocca anche i comandi [`headersHelper`](/docs/it/plugins/host-marketplace#authenticate-archive-downloads) del marketplace, tranne per un marketplace che le impostazioni gestite stesse dichiarano. Quando non impostato, segue `allowManagedHooksOnly`. Richiede Claude Code v2.1.229 o successivo, e il blocco `headersHelper` richiede v2.1.238 o successivo |
| [`disableSideloadFlags`](/docs/it/settings-reference#disablesideloadflags)                                                 | Rifiuta i flag `--plugin-dir`, `--plugin-url`, `--agents`, e `--mcp-config` all'avvio. Nelle sessioni cloud, Claude Code elimina i server MCP che il server ha consegnato tramite `--mcp-config`, diversi dalle voci in-process `type: "sdk"`, e avvia la sessione. Richiede Claude Code v2.1.193 o successivo                                                                                                                                                                                                                                             |
| [`forceRemoteSettingsRefresh`](/docs/it/settings-reference#forceremotesettingsrefresh)                                     | Quando `true`, blocca l'avvio CLI fino a quando le impostazioni gestite remote non vengono recuperate di fresco e esce se il fetch fallisce. Vedi [fail-closed enforcement](/docs/it/server-managed-settings#enforce-fail-closed-startup)                                                                                                                                                                                                                                                                                                                       |
| [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers)                                                       | Server MCP remoti forniti a ogni utente insieme ai loro. Fornisce server piuttosto che bloccare qualcosa. Vedi [Provide servers through managed settings](/docs/it/managed-mcp#provide-servers-through-managed-settings). Richiede Claude Code v2.1.259 o successivo                                                                                                                                                                                                                                                                                            |
| [`managedSourcesBehavior`](/docs/it/settings-reference#managedsourcesbehavior)                                             | Se Claude Code applica solo la fonte gestita con priorità più alta o [compone ogni una di loro](#compose-every-managed-source)                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`parentSettingsBehavior`](/docs/it/settings-reference#parentsettingsbehavior)                                             | Se le impostazioni parent fornite dall'host si uniscono sotto la policy gestita                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| [`pluginSuggestionMarketplaces`](/docs/it/settings-reference#pluginsuggestionmarketplaces)                                 | Marketplace i cui plugin Claude Code può suggerire agli utenti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`pluginTrustMessage`](/docs/it/settings-reference#plugintrustmessage)                                                     | Messaggio personalizzato aggiunto all'avviso di fiducia del plugin mostrato prima dell'installazione                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| [`policyHelper`](/docs/it/settings-reference#policyhelper)                                                                 | Eseguibile che calcola le impostazioni gestite all'avvio; vedi [Compute managed settings with a policy helper](/docs/it/settings-reference#policyhelper)                                                                                                                                                                                                                                                                                                                                                                                                        |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/it/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | Quando `true`, solo i percorsi `filesystem.allowRead` dalle impostazioni gestite sono rispettati. `denyRead` si unisce ancora da tutte le fonti                                                                                                                                                                                                                                                                                                                                                                                                            |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/it/settings-reference#sandbox-network-allowmanageddomainsonly)           | Onora solo le regole di autorizzazione `allowedDomains` e `WebFetch(domain:...)` gestite; blocca altri domini senza chiedere                                                                                                                                                                                                                                                                                                                                                                                                                               |
| [`strictKnownMarketplaces`](/docs/it/settings-reference#strictknownmarketplaces)                                           | Controlla quali fonti di marketplace di plugin gli utenti possono aggiungere e installare plugin da. Vedi [managed marketplace restrictions](/docs/it/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                                              |
| [`strictPluginOnlyCustomization`](/docs/it/settings-reference#strictpluginonlycustomization)                               | Blocca skills, agents, hooks e server MCP da fonti utente e progetto; `true` blocca tutti e quattro, un array nomina quale                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| [`wslInheritsWindowsSettings`](/docs/it/settings-reference#wslinheritswindowssettings)                                     | Quando impostato nel registro HKLM o in un file sotto `C:\Program Files\ClaudeCode`, fai leggere a WSL la catena di policy Windows, e leggi `/etc/claude-code` solo quando nessun file di impostazioni gestite o drop-in sotto quella directory distribuisce una [chiave di policy](#how-claude-code-combines-managed-sources); la voce fornisce l'ordine                                                                                                                                                                                                  |

<Note>
  Sui piani Team e Enterprise, un Owner abilita o disabilita [Remote Control](/docs/it/remote-control) e [cloud sessions](/docs/it/claude-code-on-the-web) a livello di organizzazione nelle [Claude Code admin settings](https://claude.ai/admin-settings/claude-code). Remote Control può inoltre essere disabilitato per dispositivo con l'impostazione [`disableRemoteControl`](/docs/it/settings-reference#disableremotecontrol). Le sessioni cloud non hanno chiave di impostazioni gestite per dispositivo.

  Per verificare se queste impostazioni dell'organizzazione hanno raggiunto una determinata macchina, esegui `claude doctor` lì e leggi la riga `Organization policy`, che dice dove Claude Code ha caricato la policy o perché non l'ha caricata. Richiede Claude Code v2.1.261 o successivo. In una sessione in esecuzione, `/status` mostra la stessa riga quando la policy non è stata caricata.
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  Disattivare la telemetria per la vostra organizzazione
</h2>

Claude Code invia [telemetria](/docs/it/data-usage#telemetry-services) operazionale ad Anthropic per impostazione predefinita nelle sessioni che utilizzano l'API Anthropic, sia direttamente, tramite un gateway LLM, o tramite un `ANTHROPIC_BASE_URL` personalizzato; [Default behaviors by API provider](/docs/it/data-usage#default-behaviors-by-api-provider) indica quali provider la inviano. Per disattivarla per ogni sviluppatore senza affidarsi alla shell di ogni persona, fornite `DISABLE_TELEMETRY` tramite il blocco `env` delle vostre impostazioni gestite. Questo esempio imposta `DISABLE_TELEMETRY` per tutti gli sviluppatori raggiunti dalla policy:

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code applica un valore di `1` senza mostrare all'utente la [finestra di dialogo di approvazione](/docs/it/server-managed-settings#environment-variables-and-the-approval-dialog).

Se disattivate la telemetria, Claude Code smette di inviare i dati di utilizzo che alimentano la [dashboard di analitiche](/docs/it/analytics) della vostra organizzazione per gli sviluppatori raggiunti dalla policy. La variabile disattiva anche il recupero dei feature flag, il che rende Remote Control, la modalità auto predefinita e le altre [funzionalità che richiedono il recupero dei feature flag](/docs/it/env-vars#features-that-need-feature-flag-fetching) non disponibili per quegli sviluppatori.

[Where and when a policy applies](#where-and-when-a-policy-applies) indica quale meccanismo di distribuzione raggiunge ogni superficie, e [Platform availability](/docs/it/server-managed-settings#platform-availability) indica quali sessioni saltano il recupero delle impostazioni gestite dal server.

Se la vostra organizzazione utilizza chiavi di crittografia gestite dal cliente e instrada Claude Code tramite un gateway, [Configure proxies and gateways](/docs/it/third-party-integrations#configure-proxies-and-gateways) spiega perché quelle sessioni hanno bisogno di questa variabile.

<h2 id="see-also">
  Vedi anche
</h2>

* [Set up Claude Code for your organization](/docs/it/admin-setup): decidi cosa applicare e come
* [Server-managed settings](/docs/it/server-managed-settings): distribuisci policy dalla console claude.ai o da un gateway
* [Managed MCP configuration](/docs/it/managed-mcp): controlla quali server MCP gli sviluppatori possono usare
* [All settings](/docs/it/settings-reference): ogni chiave, con se una fonte gestita può impostarla
* [Example settings files](/docs/it/settings-example#an-organizations-managed-settings): un `managed-settings.json` completo che mostra la forma delle chiavi gestite
