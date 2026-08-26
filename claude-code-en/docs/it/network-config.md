> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurazione di rete aziendale

> Configurare Claude Code per ambienti aziendali con server proxy, Autorità di Certificazione (CA) personalizzate e autenticazione Transport Layer Security (mTLS) reciproca.

Claude Code supporta varie configurazioni di rete e sicurezza aziendali attraverso variabili di ambiente. Ciò include l'instradamento del traffico attraverso server proxy aziendali, la fiducia in Autorità di Certificazione (CA) personalizzate e l'autenticazione con certificati Transport Layer Security (mTLS) reciproco per una sicurezza migliorata.

<Note>
  Tutte le variabili di ambiente mostrate in questa pagina possono essere configurate anche in [`settings.json`](/docs/it/settings).
</Note>

<h2 id="proxy-configuration">
  Configurazione del proxy
</h2>

<h3 id="environment-variables">
  Variabili di ambiente
</h3>

Claude Code rispetta le variabili di ambiente proxy standard:

```bash theme={null}
# Proxy HTTPS (consigliato)
export HTTPS_PROXY=https://proxy.example.com:8080

# Proxy HTTP (se HTTPS non disponibile)
export HTTP_PROXY=http://proxy.example.com:8080

# Ignora il proxy per richieste specifiche - formato separato da spazi
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Ignora il proxy per richieste specifiche - formato separato da virgole
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Ignora il proxy per tutte le richieste
export NO_PROXY="*"
```

<Note>
  Claude Code non supporta proxy SOCKS.
</Note>

<h3 id="basic-authentication">
  Autenticazione di base
</h3>

Se il proxy richiede l'autenticazione di base, includere le credenziali nell'URL del proxy:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Evitare di codificare le password negli script. Utilizzare variabili di ambiente o archiviazione sicura delle credenziali.
</Warning>

<Tip>
  Per proxy che richiedono autenticazione avanzata (NTLM, Kerberos, ecc.), considerare l'utilizzo di un servizio LLM Gateway che supporti il metodo di autenticazione.
</Tip>

<h2 id="ca-certificate-store">
  Archivio certificati CA
</h2>

Per impostazione predefinita, Claude Code si fida sia dei certificati CA Mozilla in bundle che dell'archivio certificati del sistema operativo. La lettura dell'archivio del sistema operativo richiede un runtime con `tls.getCACertificates`: il programma di installazione nativo lo ha sempre, e gli install npm necessitano di Node 22.15 o versioni successive. Su versioni di Node più vecchie, si applicano solo il set in bundle e `NODE_EXTRA_CA_CERTS`. I proxy di ispezione TLS aziendali come CrowdStrike Falcon e Zscaler funzionano senza configurazione aggiuntiva quando il loro certificato radice è installato nell'archivio di fiducia del sistema operativo e il runtime può leggerlo.

`CLAUDE_CODE_CERT_STORE` accetta un elenco separato da virgole di fonti. I valori riconosciuti sono `bundled` per il set CA Mozilla fornito con Claude Code e `system` per l'archivio di fiducia del sistema operativo. L'impostazione predefinita è `bundled,system`.

Per fidarsi solo del set CA Mozilla in bundle:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Per fidarsi solo dell'archivio certificati del sistema operativo:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` non ha una chiave dello schema `settings.json` dedicata. Impostarla tramite il blocco `env` in `~/.claude/settings.json` o direttamente nell'ambiente del processo.
</Note>

<h2 id="custom-ca-certificates">
  Certificati CA personalizzati
</h2>

Se l'ambiente aziendale utilizza una CA personalizzata, configurare Claude Code per fidarsi di essa direttamente:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Autenticazione mTLS
</h2>

Per ambienti aziendali che richiedono l'autenticazione del certificato client:

```bash theme={null}
# Certificato client per l'autenticazione
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Chiave privata del client
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Facoltativo: Passphrase per la chiave privata crittografata
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code legge i file del certificato e della chiave all'avvio e li rilegge ogni volta che applica le impostazioni, incluso quando le impostazioni cambiano durante una sessione. Per ruotare il certificato e la chiave, sostituire i file negli stessi percorsi.

<h2 id="network-access-requirements">
  Requisiti di accesso alla rete
</h2>

Claude Code richiede accesso ai seguenti URL. Inserire questi nella whitelist nella configurazione del proxy e nelle regole del firewall, soprattutto in ambienti di rete containerizzati o limitati.

| URL                            | Richiesto per                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`            | Richieste API Claude                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `claude.ai`                    | Autenticazione account claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `platform.claude.com`          | Autenticazione account Anthropic Console                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `mcp-proxy.anthropic.com`      | [Connettori MCP da claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai), inclusi i connettori che un amministratore dell'organizzazione configura. Il traffico dei connettori viene instradato attraverso questo proxy; i connettori sono abilitati per impostazione predefinita per gli utenti autenticati da claude.ai. Per disabilitare, impostare [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/it/env-vars) o l'impostazione [`disableClaudeAiConnectors`](/docs/it/settings#available-settings) |
| `downloads.claude.ai`          | Download eseguibili plugin; installer nativo e auto-updater nativo                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `storage.googleapis.com`       | Conteggi delle installazioni e metadati dei plugin mostrati in `/plugin`. I caricamenti di [artifact](/docs/it/artifacts) firmati provano prima questo host; la pubblicazione ricade su `api.anthropic.com` quando è bloccato                                                                                                                                                                                                                                                                |
| `storage.googleapis.com`       | Installer nativo e auto-updater nativo nelle versioni precedenti a 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                              |
| `bridge.claudeusercontent.com` | Bridge WebSocket estensione [Claude in Chrome](/docs/it/chrome)                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `*.claudeusercontent.com`      | Visualizzazione di [artifact](/docs/it/artifacts) su claude.ai. Il visualizzatore carica il contenuto di ogni artifact da un sottodominio sandbox di questa origine. Richiesto nel browser del visualizzatore, non dalla CLI stessa                                                                                                                                                                                                                                                          |
| `raw.githubusercontent.com`    | Feed changelog per [`/release-notes`](/docs/it/commands) e le note di rilascio mostrate dopo l'aggiornamento                                                                                                                                                                                                                                                                                                                                                                                 |

Se si installa Claude Code tramite npm o si gestisce la propria distribuzione binaria, gli utenti finali non hanno bisogno dell'installer nativo e l'auto-updater non utilizza `downloads.claude.ai`. Gli altri usi nella tabella si applicano indipendentemente dal metodo di installazione.

Claude Code invia anche telemetria operativa facoltativa per impostazione predefinita, che è possibile disabilitare con variabili di ambiente. Consultare [Servizi di telemetria](/docs/it/data-usage#telemetry-services) per informazioni su come disabilitarla prima di finalizzare la whitelist.

Quando si utilizza [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry) o una sessione [gateway app Claude](/docs/it/claude-apps-gateway) con accesso effettuato, il traffico del modello e l'autenticazione vanno al provider o al gateway invece di `api.anthropic.com`, `claude.ai` o `platform.claude.com`. Lo strumento WebFetch chiama comunque `api.anthropic.com` per il suo [controllo di sicurezza del dominio](/docs/it/data-usage#webfetch-domain-safety-check) a meno che non si imposti `skipWebFetchPreflight: true` nelle [impostazioni](/docs/it/settings).

[Claude Code sul web](/docs/it/claude-code-on-the-web) e [Code Review](/docs/it/code-review) si connettono ai repository dall'infrastruttura gestita da Anthropic. Se l'organizzazione GitHub Enterprise Cloud limita l'accesso per indirizzo IP, abilitare [l'ereditarietà della lista di indirizzi IP consentiti per le app GitHub installate](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps). L'app GitHub di Claude registra i suoi intervalli di indirizzi IP, quindi l'abilitazione di questa impostazione consente l'accesso senza configurazione manuale. Per [aggiungere gli intervalli alla lista di indirizzi consentiti manualmente](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) invece, o per configurare altri firewall, consultare gli [indirizzi IP dell'API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Per istanze [GitHub Enterprise Server](/docs/it/github-enterprise-server) auto-ospitate dietro un firewall, inserire nella whitelist gli stessi [indirizzi IP dell'API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses) in modo che l'infrastruttura Anthropic possa raggiungere l'host GHES per clonare i repository e pubblicare i commenti di revisione.

<h3 id="desktop-and-claude-ai">
  Desktop e claude.ai
</h3>

La tabella precedente copre principalmente la CLI standalone. L'app Claude Desktop e claude.ai in un browser caricano il codice dell'applicazione da host CDN Anthropic aggiuntivi, incluso `assets-proxy.anthropic.com`. Consentire `claude.ai` mentre si bloccano questi host produce una pagina vuota piuttosto che un errore. Consultare [requisiti di accesso alla rete](/docs/it/desktop#network-access-requirements) nella pagina Desktop.

<h2 id="additional-resources">
  Risorse aggiuntive
</h2>

* [Impostazioni di Claude Code](/docs/it/settings)
* [Riferimento delle variabili di ambiente](/docs/it/env-vars)
* [Guida alla risoluzione dei problemi](/docs/it/troubleshooting)
