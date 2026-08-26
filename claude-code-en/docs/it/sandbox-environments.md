> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Scegliere un ambiente sandbox

> Confronta le opzioni di sandbox di Claude Code: lo strumento Bash sandboxed integrato, il runtime sandbox, i dev container, Docker e le VM. Scegli l'isolamento giusto per il tuo modello di minaccia.

L'isolamento di Claude Code limita ciò che una sessione può leggere, scrivere e raggiungere sulla rete. Questo è particolarmente importante quando consenti a Claude di lavorare con meno prompt di autorizzazione, lo esegui in modo automatico o lo punti verso codice di cui non sei completamente sicuro.

Claude Code può essere eseguito in diversi tipi di ambienti isolati, che vanno da una sandbox leggera per comando a una macchina virtuale completamente separata. Questa pagina confronta gli ambienti in base a ciò che isolano e a cosa richiedono, ti aiuta a sceglierne uno per il tuo modello di minaccia e mostra come applicare tale scelta in tutta l'organizzazione.

<Info>
  Per il modello di sicurezza più ampio, vedi [Security](/docs/it/security). Per i deployment di Agent SDK, vedi [Secure deployment](/docs/it/agent-sdk/secure-deployment).
</Info>

<h2 id="compare-sandboxing-approaches">
  Confrontare gli approcci di sandboxing
</h2>

I primi due approcci nella tabella sottostante vengono eseguiti sul sistema operativo host senza container. Gli altri posizionano Claude Code all'interno di un container o di una macchina virtuale.

| Approccio                                         | Cosa è isolato                                                                | Richiede Docker | Sforzo di setup                                  |
| :------------------------------------------------ | :---------------------------------------------------------------------------- | :-------------- | :----------------------------------------------- |
| [Sandboxed Bash tool](#sandboxed-bash-tool)       | Comandi Bash e i loro processi figli                                          | No              | Minimo su macOS; basso su Linux e WSL2           |
| [Sandbox runtime](#sandbox-runtime)               | L'intero processo Claude Code, inclusi i file tools, i server MCP e gli hooks | No              | Basso                                            |
| [Dev container](#dev-containers)                  | Ambiente di sviluppo completo                                                 | Sì              | Medio                                            |
| [Custom container](#custom-container)             | Ambiente di sviluppo completo                                                 | Sì              | Medio-alto                                       |
| [Virtual machine](#virtual-machine)               | Sistema operativo completo                                                    | No              | Alto                                             |
| [Claude Code on the web](#claude-code-on-the-web) | Sistema operativo completo, ospitato da Anthropic                             | No              | Nessuno; richiede un abbonamento Claude e GitHub |

Lo [sandboxed Bash tool](/docs/it/sandboxing) è integrato in Claude Code e limita solo i comandi Bash. I file tools integrati, i server MCP e gli hooks vengono comunque eseguiti direttamente sul tuo host. Ogni altro approccio nella tabella posiziona l'intero processo Claude Code all'interno del confine di isolamento, quindi i file tools, i server MCP e gli hooks sono anch'essi limitati.

<Warning>
  L'isolamento sandbox riduce l'impatto di una violazione, ma non elimina il rischio. Qualsiasi approccio che consente l'uscita di rete può comunque perdere dati che l'agente può leggere, e qualsiasi approccio che monta la tua directory di progetto in scrittura può comunque modificare quel codice. Rivedi le [limitazioni di sicurezza](/docs/it/sandboxing#security-limitations) prima di fare affidamento su una sandbox come controllo rigido.

  L'isolamento inoltre non cambia ciò che viene inviato al modello. I tuoi prompt e i file che Claude legge vengono trasmessi all'API Anthropic o al tuo provider configurato con o senza una sandbox. Vedi [Data usage](/docs/it/data-usage) per ciò che Claude Code invia e come ridurlo.
</Warning>

<h2 id="choose-an-approach">
  Scegliere un approccio
</h2>

Abbina il tuo obiettivo a una riga sottostante, quindi leggi la sezione di dettaglio che segue.

| Vuoi                                                                                                  | Inizia con                                                                                                                                       |
| :---------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| Ridurre i prompt di autorizzazione durante il lavoro quotidiano sulla tua macchina                    | Lo [sandboxed Bash tool](/docs/it/sandboxing), abilitato con `/sandbox`                                                                               |
| Lasciare che Claude lavori in modo automatico con `--dangerously-skip-permissions` o in modalità auto | Il [dev container](/docs/it/devcontainer) preconfigurato, qualsiasi container o VM, o il [sandbox runtime](#sandbox-runtime)                          |
| Isolare i server MCP e gli hooks così come Bash, senza Docker                                         | Il sandbox runtime                                                                                                                               |
| Lavorare su un repository non attendibile                                                             | Una macchina virtuale dedicata, o [Claude Code on the web](/docs/it/claude-code-on-the-web) se hai un abbonamento Claude e un account GitHub connesso |
| Standardizzare un ambiente sandboxed in un team                                                       | Il [dev container](/docs/it/devcontainer) preconfigurato, copiato nel tuo repository                                                                  |
| Usare Claude Code da un dispositivo senza setup locale                                                | [Claude Code on the web](/docs/it/claude-code-on-the-web), che richiede un abbonamento Claude e un account GitHub connesso                            |
| Richiedere l'isolamento per ogni sviluppatore nella tua organizzazione                                | [Applicare l'isolamento in un'organizzazione](#enforce-isolation-across-an-organization)                                                         |
| Lavorare su un host Windows nativo                                                                    | Un container o VM, o eseguire la sandbox Bash all'interno di WSL2                                                                                |

<h3 id="how-isolation-relates-to-permission-modes">
  Come l'isolamento si relaziona alle modalità di autorizzazione
</h3>

Le [modalità di autorizzazione](/docs/it/permission-modes) decidono se una chiamata di strumento viene eseguita e se sei richiesto per primo. L'isolamento limita ciò che un comando può accedere una volta eseguito. I due lavorano insieme: quando una modalità di autorizzazione consente l'esecuzione di azioni senza chiederti, un confine di isolamento limita ciò che quelle azioni possono raggiungere.

Quando passi `--dangerously-skip-permissions`, Claude agisce senza chiederti per primo; sei richiesto solo per le [regole ask](/docs/it/permissions#manage-permissions) esplicite, i connector tools [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), i tool MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool), e le rimozioni che puntano a `/` o alla tua home directory. Senza prompt per catturare gli errori, il confine di isolamento che scegli è ciò che protegge il tuo sistema. Esegui sempre le sessioni `--dangerously-skip-permissions` all'interno di un container, una VM, o il [sandbox runtime](#sandbox-runtime), in modo che i file tools, i server MCP e gli hooks siano anch'essi all'interno del confine.

La [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) sostituisce il prompt con un classificatore che esamina le azioni e blocca quelle che vanno oltre la richiesta, puntano a infrastrutture non riconosciute, o sembrano guidate da contenuti ostili che Claude ha letto. Il classificatore è un controllo per azione, non un confine di isolamento, quindi un confine di isolamento aggiunge comunque difesa in profondità per esecuzioni automatiche, e non è richiesto come lo è per `--dangerously-skip-permissions`.

Lo [sandboxed Bash tool](#sandboxed-bash-tool) da solo vincola solo Bash, quindi non è sufficiente per esecuzioni completamente automatiche in nessuna delle due modalità. Puoi stratificare gli approcci: eseguire lo sandboxed Bash tool all'interno di un container o VM ti dà restrizioni di comando a livello di SO in cima al confine dell'ambiente esterno. Per come la sandbox Bash stessa interagisce con le regole di autorizzazione e le modalità di autorizzazione, vedi [Come il sandboxing si relaziona alle autorizzazioni e alle modalità di autorizzazione](/docs/it/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Sandboxed Bash tool
</h2>

<Note>
  Questa opzione non supporta Windows nativo. Su host Windows, usa WSL2 o uno degli approcci container o VM sottostanti.
</Note>

Lo sandboxed Bash tool è integrato in Claude Code. Utilizza primitive del sistema operativo per limitare l'accesso al filesystem e alla rete di ogni comando Bash che Claude esegue: Seatbelt, la sandbox macOS integrata, e [bubblewrap](https://github.com/containers/bubblewrap) su Linux e WSL2. Per impostazione predefinita consente scritture nella directory di lavoro e richiede la prima volta che un comando ha bisogno di un nuovo dominio di rete.

Abilitalo con il comando `/sandbox`. La guida [Sandboxing](/docs/it/sandboxing) copre le modalità di approvazione, il confine predefinito, e come ampliarlo o restringerlo.

La sandbox per comando non copre tutto ciò che viene eseguito in una sessione:

* Altri [strumenti integrati](/docs/it/tools-reference) come Read, Edit e WebFetch vengono eseguiti all'interno del processo Claude Code e non generano codice arbitrario. Le [regole di autorizzazione](/docs/it/permissions) per percorso o dominio le controllano invece.
* I server [MCP](/docs/it/mcp) e gli hooks sono processi separati che vengono eseguiti senza vincoli sull'host.

Per mettere i file tools, i server MCP e gli hooks tutti dietro un confine di SO, esegui l'intero processo Claude Code all'interno del [sandbox runtime](#sandbox-runtime), del [dev container](#dev-containers), o di un [custom container](#custom-container).

<h2 id="sandbox-runtime">
  Sandbox runtime
</h2>

Il pacchetto [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) avvolge un intero processo nello stesso isolamento Seatbelt o bubblewrap che la sandbox Bash integrata utilizza. Eseguire Claude Code attraverso di esso vincola ogni strumento, hook e server MCP nella sessione, non solo Bash. Il runtime è un'anteprima di ricerca beta, e il suo formato di configurazione potrebbe cambiare man mano che il pacchetto evolve.

Il runtime nega tutto l'accesso in scrittura e di rete per impostazione predefinita, quindi configuralo prima di lanciare Claude Code attraverso di esso. In `~/.srt-settings.json`, o un file che passi con `--settings`, consenti l'accesso in scrittura ad almeno la tua directory di progetto e i percorsi di configurazione di Claude Code `~/.claude` e `~/.claude.json`. Consenti i domini di rete di cui la tua sessione ha bisogno, incluso `api.anthropic.com` o l'endpoint del tuo provider configurato. Vedi il [README](https://github.com/anthropic-experimental/sandbox-runtime) del pacchetto per lo schema di configurazione completo.

Una volta che il file di impostazioni è in posizione, avvia Claude Code con `npx` e passa `claude` come comando da avvolgere:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code si avvia all'interno della sandbox con i confini di filesystem e di rete che hai configurato. Lo stesso comando funziona per il sandboxing di server MCP autonomi o altri processi di supporto.

<h2 id="dev-containers">
  Dev containers
</h2>

Un dev container esegue Claude Code all'interno di un container Docker che VS Code o un editor compatibile gestisce, con il tuo progetto montato. Puoi definire il tuo con una directory `.devcontainer/` nel tuo repository.

Il repository claude-code pubblica un [esempio di dev container](/docs/it/devcontainer) con un firewall iptables default-deny come punto di partenza. Copialo nel tuo repository e regola la whitelist del firewall, l'immagine di base e la versione di Claude Code fissata per adattarsi al tuo ambiente. Poiché il firewall blocca l'uscita non approvata, una configurazione come questa supporta l'esecuzione di Claude Code con `--dangerously-skip-permissions` per il lavoro automatico.

<h2 id="custom-container">
  Custom container
</h2>

Puoi eseguire Claude Code in qualsiasi immagine container Docker o OCI con le tue politiche di rete, volumi montati e profili seccomp. Questo è il percorso più comune per le organizzazioni con infrastruttura container esistente o runner CI.

Diversi servizi di sandbox gestiti e di esecuzione remota possono ospitare il container per te. La stessa checklist si applica come per qualsiasi container che gestisci: rivedi cosa è montato in scrittura, quali credenziali e token sono raggiungibili all'interno, e cosa consente la politica di uscita di rete.

Puoi stratificare la sandbox Bash integrata all'interno del container per restrizioni per comando. I container senza privilegi hanno bisogno dell'impostazione nested-sandbox descritta in [Sandboxing troubleshooting](/docs/it/sandboxing#troubleshooting).

<h2 id="virtual-machine">
  Virtual machine
</h2>

Una macchina virtuale dedicata fornisce la separazione più forte, con il suo kernel e, nei deployment cloud o microVM, il suo hardware virtualizzato. Le opzioni includono istanze cloud, hypervisor locali e microVM come Firecracker.

Usa questo approccio quando stai valutando codice non attendibile, quando la tua politica di sicurezza richiede separazione a livello di kernel tra l'agente e l'host, o quando nessun approccio a livello di host soddisfa i tuoi requisiti di conformità. La funzione [sandboxes](https://docs.docker.com/ai/sandboxes/) di Docker Desktop fornisce una microVM con il suo daemon Docker e sincronizzazione dell'area di lavoro, che può eseguire Claude Code su host che hanno già Docker Desktop.

<h2 id="claude-code-on-the-web">
  Claude Code on the web
</h2>

[Claude Code on the web](/docs/it/claude-code-on-the-web) esegue ogni sessione in una macchina virtuale isolata gestita da Anthropic. Un proxy di rete applica una whitelist predefinita, e un proxy separato tiene il tuo token GitHub al di fuori della sandbox mentre emette credenziali scoped per l'accesso al repository all'interno di essa.

Usa questo approccio quando vuoi l'isolamento completo della VM senza provisioning dell'infrastruttura da solo, o quando stai delegando attività da un dispositivo che non ha un ambiente di sviluppo locale. Richiede un abbonamento Claude e un account GitHub connesso, e le sessioni clonano il tuo repository da GitHub. Vedi [Claude Code on the web](/docs/it/claude-code-on-the-web) per la disponibilità del piano e le opzioni di autenticazione GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Applicare l'isolamento in un'organizzazione
</h2>

I singoli sviluppatori possono optare per qualsiasi approccio sopra. Ciò che un'organizzazione può applicare, e con quali strumenti, dipende dall'approccio:

* **Built-in Bash sandbox**: l'unico approccio che Claude Code applica da solo. Fornisci le chiavi di impostazioni `sandbox` attraverso [managed settings](/docs/it/settings#settings-files), sia come file gestito dal tuo MDM che attraverso [server-managed settings](/docs/it/server-managed-settings) su Claude.ai. Vedi [Enforce sandboxing with managed settings](/docs/it/sandboxing#enforce-sandboxing-with-managed-settings) per le chiavi da distribuire e come impedire agli sviluppatori di ampliare la politica.
* **Dev containers**: esegui il commit dell'[esempio di dev container](/docs/it/devcontainer) nei tuoi repository per standardizzare l'ambiente in un team. Questa è una convenzione piuttosto che un confine di applicazione, perché Claude Code non richiede un container. Se gli sviluppatori non dovrebbero essere in grado di eseguire Claude Code al di fuori di esso, applica ciò con gli strumenti di gestione dei dispositivi della tua organizzazione o di allowlisting del software.
* **Custom containers e VMs**: distribuisci Claude Code attraverso l'immagine approvata e usa gli strumenti di gestione dei dispositivi della tua organizzazione o di allowlisting del software per prevenire l'installazione al di fuori di essa.

<h2 id="see-also">
  Vedi anche
</h2>

Queste pagine coprono i dettagli di configurazione e politica per gli approcci sopra.

* [Sandboxing](/docs/it/sandboxing): configura lo sandboxed Bash tool integrato
* [Dev container](/docs/it/devcontainer): il container di sviluppo Docker preconfigurato
* [Security](/docs/it/security): il modello di sicurezza completo di Claude Code
* [Secure deployment](/docs/it/agent-sdk/secure-deployment): guida all'isolamento per le applicazioni Agent SDK
* [Settings](/docs/it/settings#sandbox-settings): tutte le chiavi di configurazione sandbox, inclusa la consegna di managed settings
