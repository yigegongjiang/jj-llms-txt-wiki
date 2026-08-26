> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Controllare l'accesso ai server MCP per la vostra organizzazione

> Limitare quali server MCP gli utenti possono aggiungere o connettere con file di configurazione gestiti, allowlist e denylist.

Per impostazione predefinita, chiunque esegua Claude Code può connettere qualsiasi [server MCP](/docs/it/mcp) desideri. Anthropic esamina i connettori rispetto ai suoi [criteri di elenco](https://claude.com/docs/connectors/building/review-criteria) prima di aggiungerli alla [Directory Anthropic](https://claude.ai/directory), ma non esegue audit di sicurezza né gestisce alcun server MCP. Come amministratore, potete limitare quali server vengono eseguiti nella vostra organizzazione, da un set fisso approvato alla disabilitazione completa di MCP.

Questa pagina spiega come:

* [Scegliere un modello](#choose-a-pattern) che corrisponda al livello di controllo di cui avete bisogno
* [Distribuire un set di server fisso con `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), incluso come [disabilitare completamente MCP](#disable-mcp-entirely)
* [Controllare i server con allowlist e denylist](#policy-based-control-with-allowlists-and-denylists)
* [Comunicare agli utenti cosa aspettarsi](#how-restrictions-appear-to-users) quando una restrizione blocca un server
* [Monitorare quali server la vostra organizzazione utilizza effettivamente](#monitor-mcp-usage)

<Note>
  La pagina [Security](/docs/it/security) copre il modello di minaccia MCP e come valutare un server prima di approvarlo. [Decidere cosa applicare](/docs/it/admin-setup#decide-what-to-enforce) copre le restrizioni MCP insieme agli altri controlli amministrativi.
</Note>

<h2 id="choose-a-pattern">
  Scegliere un modello
</h2>

Claude Code supporta una gamma di livelli di restrizione. Ogni modello utilizza uno o entrambi i meccanismi descritti di seguito: `managed-mcp.json` per distribuire un set fisso, e `allowedMcpServers`/`deniedMcpServers` per filtrare ciò che gli utenti configurano.

| Modello                 | Cosa fa                                                                                                              | Configurare                                                                                         |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| **Disabilitare MCP**    | Nessun server viene caricato da nessuna parte                                                                        | `managed-mcp.json` con una mappa di server vuota                                                    |
| **Distribuzione fissa** | Ogni utente ottiene gli stessi server e non può aggiungerne altri                                                    | `managed-mcp.json` con i server desiderati                                                          |
| **Catalogo approvato**  | Pubblicare un elenco di server approvati; gli utenti aggiungono quelli che desiderano, tutto il resto viene bloccato | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                            |
| **Solo server plugin**  | I server possono provenire solo da plugin; gli utenti non possono aggiungere i propri                                | [`strictPluginOnlyCustomization`](/docs/it/settings#strictpluginonlycustomization) con `mcp` nell'elenco |
| **Allowlist soft**      | Applicare un allowlist che gli utenti possono ampliare nelle loro impostazioni                                       | `allowedMcpServers` senza `allowManagedMcpServersOnly`                                              |
| **Solo denylist**       | Bloccare i server noti come cattivi, consentire tutto il resto                                                       | `deniedMcpServers`                                                                                  |
| **Nessuna restrizione** | Gli utenti aggiungono qualsiasi cosa                                                                                 | Non distribuire alcuna configurazione MCP gestita                                                   |

<Note>
  Claude Code non dispone di un registro di server MCP integrato che gli utenti possono sfogliare e installare. Per il modello di catalogo approvato, condividete l'elenco approvato e i suoi comandi `claude mcp add` in un luogo dove i vostri utenti li troveranno, come un wiki interno, oppure distribuite i server come plugin attraverso un [marketplace di plugin gestito](/docs/it/plugin-marketplaces#managed-marketplace-restrictions) in modo che gli utenti possano sfogliarli e installarli da `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Controllo esclusivo con managed-mcp.json
</h2>

Se distribuite un file `managed-mcp.json`, Claude Code carica solo i server che quel file definisce. Gli utenti non possono aggiungere, modificare o utilizzare alcun altro server MCP, inclusi i server forniti da plugin. Il file sopprime anche i connettori di claude.ai a meno che non li [consentiate insieme al set gestito](#allow-claude-ai-connectors-alongside-the-managed-set).

Due altre impostazioni possono filtrare ulteriormente il set gestito:

* `allowedMcpServers` e `deniedMcpServers` si applicano anche ai server gestiti, quindi un server gestito che non li supera non verrà caricato.
* Il proprio `deniedMcpServers` di un utente si unisce dalle sue impostazioni, quindi gli utenti possono bloccare un server gestito per se stessi.

Vedere [Come viene valutato un server](#how-a-server-is-evaluated) per l'ordine completo dei controlli.

`managed-mcp.json` è un file autonomo, quindi non può essere consegnato attraverso [impostazioni gestite dal server](/docs/it/server-managed-settings). Qualsiasi processo che possa scrivere in un percorso di sistema con privilegi di amministratore può distribuirlo. Su larga scala, di solito avviene attraverso strumenti di gestione dei dispositivi, come Jamf o un profilo di configurazione su macOS, Criteri di gruppo o Intune su Windows, o la gestione della flotta di vostra scelta su Linux. Claude Code cerca il file in uno di questi percorsi:

| Piattaforma | Percorso                                                   |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux e WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

Il file utilizza lo stesso formato di un file di progetto [`.mcp.json`](/docs/it/mcp#project-scope):

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

Qualsiasi utente sulla macchina può leggere questo file, quindi non archiviate chiavi API o altre credenziali nei blocchi `env`. Passate le credenziali per utente con uno di questi:

* [Espansione `${VAR}`](/docs/it/mcp#environment-variable-expansion-in-mcp-json) per leggere i segreti dall'ambiente di ogni utente.
* [OAuth o intestazioni per utente](/docs/it/mcp#authenticate-with-remote-mcp-servers) in modo che ogni utente si autentichi come se stesso.
* [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication) per generare credenziali al momento della connessione.

<h3 id="validate-the-configuration">
  Convalidare la configurazione
</h3>

Per confermare che il file è in vigore, eseguite due controlli su una macchina gestita:

1. `claude mcp list` mostra solo i server in `managed-mcp.json`. Se i server personali di un utente appaiono ancora, il file non viene letto; controllate il percorso e i permessi.
2. `claude mcp add --transport http test https://example.com/mcp` fallisce con `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. L'URL non deve essere un server reale, poiché il controllo della politica rifiuta il comando prima che qualsiasi cosa venga contattata.

<h3 id="disable-mcp-entirely">
  Disabilitare MCP interamente
</h3>

Distribuite un `managed-mcp.json` contenente una mappa di server vuota per bloccare ogni server MCP:

```json theme={null}
{
  "mcpServers": {}
}
```

Gli utenti non vedono alcun server MCP in `/mcp`, e `claude mcp add` fallisce con l'errore di politica aziendale di cui sopra. I server che gli utenti avevano precedentemente configurato smettono di caricarsi la prossima volta che avviano una sessione, senza alcun avviso che la politica è il motivo.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Consentire i connettori di claude.ai insieme al set gestito
</h3>

La distribuzione di `managed-mcp.json` sopprime i [connettori di claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) per impostazione predefinita, inclusi i connettori che un amministratore ha configurato per l'organizzazione nella console di amministrazione di claude.ai. Per caricare quei connettori insieme ai server in `managed-mcp.json`, impostate `"allowAllClaudeAiMcps": true` in una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices). Richiede Claude Code v2.1.149 o successivo.

Con l'impostazione abilitata, Claude Code carica gli stessi connettori di claude.ai che caricherrebbe se `managed-mcp.json` non fosse distribuito. [Elenchi consentiti e elenchi negati](#policy-based-control-with-allowlists-and-denylists) si applicano ancora a quei connettori, quindi potete bloccare quelli specifici con `deniedMcpServers`. L'impostazione influisce solo sui connettori di claude.ai; i server forniti da plugin rimangono soppressi.

Claude Code legge questa impostazione solo dai livelli di politica controllati dall'amministratore: impostazioni gestite dal server, una chiave di registro plist distribuita da MDM o HKLM, o un file `managed-settings.json` di sistema. Inserirla nelle impostazioni utente o di progetto non ha alcun effetto, quindi gli utenti non possono riabilitare i connettori che il controllo esclusivo ha soppresso.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Controllo basato su politica con allowlist e denylist
</h2>

Le allowlist e le denylist filtrano quali server configurati sono autorizzati a caricarsi. Non sono un registro: un server deve comunque essere aggiunto da un utente, un plugin o `managed-mcp.json` prima che l'allowlist o la denylist si applichino ad esso. Per distribuire server agli utenti, utilizzate [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json). Entrambi gli elenchi filtrano anche i server passati con il flag CLI [`--mcp-config`](/docs/it/cli-reference#cli-flags); `--strict-mcp-config` limita quali file di configurazione si caricano e non bypassa nessuno dei due elenchi.

Per rendere l'allowlist autorevole, impostate `allowedMcpServers` e `allowManagedMcpServersOnly: true` insieme in una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices), come impostazioni gestite dal server o un file `managed-settings.json` distribuito. [Limitare l'allowlist solo alle impostazioni gestite](#restrict-the-allowlist-to-managed-settings-only) mostra la configurazione. Senza `allowManagedMcpServersOnly`, le allowlist da ogni fonte di impostazioni si uniscono, incluso il `~/.claude/settings.json` personale di un utente, quindi un utente può ampliare ciò che la vostra allowlist consente. Le denylist si uniscono da ogni fonte indipendentemente.

<Note>
  `allowManagedMcpServersOnly` è separato da `allowManagedPermissionRulesOnly`, che blocca solo le [regole di autorizzazione](/docs/it/permissions#managed-settings). L'impostazione di quel flag non applica l'allowlist MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Abbinare i server per URL, comando o nome
</h3>

`allowedMcpServers` e `deniedMcpServers` sono elenchi di voci. Ogni voce è un oggetto con una singola chiave che identifica i server per il loro URL, il loro comando o il loro nome:

| Chiave          | Corrisponde a                                                                                        | Usare per                                          |
| :-------------- | :--------------------------------------------------------------------------------------------------- | :------------------------------------------------- |
| `serverUrl`     | Un URL di server remoto, esatto o con caratteri jolly `*`                                            | Server HTTP e SSE                                  |
| `serverCommand` | Il comando esatto e gli argomenti che avviano un server stdio                                        | Server stdio                                       |
| `serverName`    | L'etichetta assegnata dall'utente. Solo corrispondenza esatta; i caratteri jolly non vengono espansi | Entrambi i tipi, ma vedere l'Avvertenza di seguito |

Lasciare `allowedMcpServers` non impostato è diverso dall'impostarlo su un array vuoto:

| Impostazione        | Non impostato (predefinito) | Array vuoto `[]`         | Popolato                                |
| :------------------ | :-------------------------- | :----------------------- | :-------------------------------------- |
| `allowedMcpServers` | Tutti i server consentiti   | Nessun server consentito | Solo i server corrispondenti consentiti |
| `deniedMcpServers`  | Nessun server bloccato      | Nessun server bloccato   | Server corrispondenti bloccati          |

Vedere [Voci non valide nelle impostazioni gestite](/docs/it/settings#invalid-entries-in-managed-settings) per ciò che accade quando una voce non supera la convalida dello schema.

<Warning>
  Una voce `serverName`, in entrambi gli elenchi, non è un controllo di sicurezza. Il nome è l'etichetta che un utente assegna quando esegue `claude mcp add` o modifica un file di configurazione, non il server sottostante, quindi un utente può chiamare qualsiasi server `github`. Per i connettori claude.ai il nome è il nome visualizzato restituito da claude.ai, che può cambiare. Per applicare quali server vengono effettivamente eseguiti, aggiungete voci `serverCommand` o `serverUrl`.
</Warning>

La validazione di `serverName` differisce tra i due elenchi:

* In `deniedMcpServers`, `serverName` accetta qualsiasi stringa non vuota, quindi potete bloccare i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) per il loro nome visualizzato. Ad esempio, `{ "serverName": "claude.ai Slack" }` blocca il connettore Slack. Preferite una voce `serverUrl` quando avete bisogno che il blocco sia robusto rispetto ai cambi di nome, o quando un nome di connettore collide e guadagna un suffisso ` (N)`.
* In `allowedMcpServers`, `serverName` è limitato a lettere, numeri, trattini e sottolineature. Utilizzate `serverUrl` per aggiungere un connettore claude.ai all'allowlist.

Per disattivare tutti i connettori claude.ai, vedere [`disableClaudeAiConnectors`](/docs/it/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Come viene valutato un server
</h3>

Prima di caricare un server, incluso uno da `managed-mcp.json`, Claude Code esegue tre controlli in ordine:

1. **Unire gli elenchi.** Le voci di allowlist e denylist da ogni fonte di impostazioni si combinano in un allowlist e una denylist. Quando `allowManagedMcpServersOnly` è `true`, viene mantenuto solo l'allowlist gestito; la denylist si unisce sempre da ogni fonte.
2. **Controllare la denylist.** Un server che corrisponde a qualsiasi voce di denylist, per URL, comando o nome, viene bloccato. Nulla sostituisce una corrispondenza di denylist.
3. **Controllare l'allowlist.** Se `allowedMcpServers` non è impostato da nessuna parte, ogni server che ha superato la denylist viene caricato. Se è impostato, ciò a cui il server deve corrispondere dipende dal suo tipo, mostrato nella tabella di seguito.

| Tipo di server      | Consentito quando corrisponde a                                                                                           |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------ |
| Remoto (HTTP o SSE) | Una voce `serverUrl`. Una corrispondenza `serverName` conta solo quando l'allowlist non contiene voci `serverUrl`         |
| Stdio               | Una voce `serverCommand`. Una corrispondenza `serverName` conta solo quando l'allowlist non contiene voci `serverCommand` |

Tre regole di corrispondenza si applicano all'interno di questi controlli:

* **I comandi corrispondono esattamente.** Ogni argomento, in ordine. `["npx", "-y", "server"]` non corrisponde a `["npx", "server"]` o `["npx", "-y", "server", "--flag"]`.
* **I valori `serverCommand` e `serverUrl` si espandono prima della corrispondenza.** Sia la voce di politica che il valore configurato del server passano attraverso la stessa espansione [`${VAR}` e `${VAR:-default}`](/docs/it/mcp#environment-variable-expansion-in-mcp-json) di `.mcp.json`, quindi una voce scritta come `["${HOME}/bin/server"]` corrisponde a una configurazione di server che utilizza sia lo stesso riferimento che il percorso espanso. Su Windows, fate riferimento a una variabile di ambiente impostata lì, come `${USERPROFILE}` invece di `${HOME}`. I valori `serverName` corrispondono letteralmente e non si espandono mai.
* **Gli URL supportano caratteri jolly `*`** ovunque nel modello, incluso lo schema. La corrispondenza del nome host non distingue tra maiuscole e minuscole e ignora un punto FQDN finale, quindi `https://Mcp.Example.com/*` corrisponde a `https://mcp.example.com/api`. I percorsi rimangono sensibili alle maiuscole.

| Modello                     | Consente                                                                                           |
| :-------------------------- | :------------------------------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Tutti i percorsi su un dominio specifico                                                           |
| `https://mcp.example.com`   | Anche tutti i percorsi su quel dominio. Un modello senza percorso corrisponde a qualsiasi percorso |
| `https://*.example.com/*`   | Qualsiasi sottodominio di `example.com`                                                            |
| `http://localhost:*/*`      | Qualsiasi porta su localhost                                                                       |
| `*://mcp.example.com/*`     | Qualsiasi schema a un dominio specifico                                                            |

Poiché l'espansione `${VAR}` legge l'ambiente di processo di Claude Code stesso, una voce di politica `serverCommand` o `serverUrl` che fa riferimento a una variabile si espande a qualsiasi valore un utente imposta. Utilizzate URL e comandi letterali per le voci su cui fate affidamento per l'applicazione.

<h3 id="example-configuration">
  Configurazione di esempio
</h3>

La configurazione di seguito configura un allowlist rigido con una denylist. Le righe evidenziate cambiano il modo in cui il resto dell'elenco viene valutato, e i callout dopo il blocco spiegano ognuno:

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
* **Riga 11**: una voce `serverName` nella denylist. Le voci di denylist si applicano sempre, quindi qualsiasi server denominato `dangerous-server` viene bloccato indipendentemente dal suo URL o comando.

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
  | Server HTTP a `https://api.internal.example.com/mcp` | Consentito: corrisponde al sottodominio con carattere jolly  |
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

<Accordion title="Allowlist con override di denylist">
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

  | Server                                          | Risultato                                                                                    |
  | :---------------------------------------------- | :------------------------------------------------------------------------------------------- |
  | Server HTTP a `https://mcp.example.com/api`     | Consentito: corrisponde al modello di URL dell'allowlist, nessuna corrispondenza di denylist |
  | Server HTTP a `https://staging.example.com/api` | Bloccato: corrisponde a entrambi, ma la denylist ha la precedenza                            |
  | Server HTTP a `https://other.com/mcp`           | Bloccato: non corrisponde all'allowlist                                                      |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Limitare l'allowlist solo alle impostazioni gestite
</h3>

Per rendere l'allowlist gestito l'unico che si applica, impostate `allowManagedMcpServersOnly` nel file di impostazioni gestite:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Quando `allowManagedMcpServersOnly` è `true`, le allowlist dalle impostazioni di utente, progetto e locale vengono ignorate. La denylist si unisce comunque da tutte le fonti, quindi gli utenti possono sempre bloccare i server per se stessi.

<h2 id="how-restrictions-appear-to-users">
  Come le restrizioni appaiono agli utenti
</h2>

Quando una restrizione blocca un server, l'utente vede un errore da `claude mcp add` o il server smette silenziosamente di caricarsi. Utilizzate questa tabella per riconoscere questi rapporti e per comunicare agli utenti cosa aspettarsi prima di implementare una modifica:

| Restrizione                                                         | Cosa vede l'utente                                                                                         |
| :------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` è presente e l'utente esegue `claude mcp add`    | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| Il server è su una denylist e l'utente esegue `claude mcp add`      | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                        |
| Il server non è sull'allowlist e l'utente esegue `claude mcp add`   | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                         |
| Un server precedentemente configurato è ora bloccato dalla politica | Il server scompare silenziosamente da `/mcp` e `claude mcp list` senza avviso                              |

Nell'ultimo caso, l'utente non riceve alcun segnale che la politica è il motivo per cui il suo server è scomparso, quindi comunicate agli utenti interessati quali server sono bloccati quando implementate una nuova restrizione.

<h2 id="monitor-mcp-usage">
  Monitorare l'utilizzo di MCP
</h2>

Quando [l'esportazione OpenTelemetry](/docs/it/monitoring-usage) è configurata, Claude Code può registrare quali server MCP e strumenti gli utenti invocano. Impostate `OTEL_LOG_TOOL_DETAILS=1` per includere i nomi dei server MCP e degli strumenti negli eventi degli strumenti, quindi aggregateli nel vostro collector per vedere quali server i vostri utenti effettivamente connettono. Vedere [Monitoraggio](/docs/it/monitoring-usage) per configurare l'esportatore e per lo schema completo degli eventi.

<h2 id="configuration-summary">
  Riepilogo della configurazione
</h2>

Ogni file e impostazione che questa pagina copre, cosa controlla e come consegnarlo:

| Superficie                   | Cosa controlla                                                                   | Dove si trova                                                                                                                                              | Come consegnare                                                                                                                                                                               |
| :--------------------------- | :------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | Set di server fisso, controllo esclusivo                                         | Percorso di sistema: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, o `C:\Program Files\ClaudeCode\`                                     | MDM, GPO, gestione della flotta, o qualsiasi processo con privilegi di amministratore. Non può essere impostato attraverso impostazioni gestite dal server                                    |
| `allowedMcpServers`          | Allowlist di server consentiti                                                   | Qualsiasi [file di impostazioni](/docs/it/settings#settings-files); le voci da ogni fonte si uniscono a meno che `allowManagedMcpServersOnly` non sia impostato | Per l'applicazione, una [fonte di impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices): impostazioni gestite dal server, `managed-settings.json`, profilo MDM, o registro |
| `deniedMcpServers`           | Denylist di server bloccati                                                      | Qualsiasi file di impostazioni; le voci da ogni fonte si uniscono                                                                                          | Uguale a `allowedMcpServers`                                                                                                                                                                  |
| `allowManagedMcpServersOnly` | Blocca l'allowlist solo alle fonti gestite                                       | Solo fonti di impostazioni gestite; l'impostazione non ha effetto altrove                                                                                  | Uguale a `allowedMcpServers`                                                                                                                                                                  |
| `allowAllClaudeAiMcps`       | Carica i connettori claude.ai insieme a `managed-mcp.json` invece di sopprimerli | Solo fonti di impostazioni gestite; l'impostazione non ha effetto altrove                                                                                  | Uguale a `allowedMcpServers`                                                                                                                                                                  |

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Decidere cosa applicare](/docs/it/admin-setup#decide-what-to-enforce): restrizioni MCP insieme alle regole di autorizzazione, sandboxing e agli altri controlli di amministrazione
* [Connettere Claude Code agli strumenti tramite MCP](/docs/it/mcp): il riferimento MCP completo, inclusi trasporti, ambiti e autenticazione
* [Impostazioni](/docs/it/settings): la gerarchia delle impostazioni e come le impostazioni gestite hanno la precedenza
* [Impostazioni gestite dal server](/docs/it/server-managed-settings): consegnare `allowedMcpServers` e `deniedMcpServers` dalla console di amministrazione di Claude.ai
* [Sicurezza](/docs/it/security): il modello di minaccia che questi controlli difendono
* [Guida dell'amministratore Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, gestione dei posti e playbook di implementazione
