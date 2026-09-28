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

| Approccio                                   | Cosa è isolato                                                                | Richiede Docker | Sforzo di setup                                                                                                |
| :------------------------------------------ | :---------------------------------------------------------------------------- | :-------------- | :------------------------------------------------------------------------------------------------------------- |
| [Sandboxed Bash tool](#sandboxed-bash-tool) | Comandi Bash, PowerShell e Monitor e i loro processi figli                    | No              | Minimo su macOS; basso su Linux e WSL2                                                                         |
| [Sandbox runtime](#sandbox-runtime)         | L'intero processo Claude Code, inclusi i file tools, i server MCP e gli hooks | No              | Basso                                                                                                          |
| [Dev container](#dev-containers)            | Ambiente di sviluppo completo                                                 | Sì              | Medio                                                                                                          |
| [Custom container](#custom-container)       | Ambiente di sviluppo completo                                                 | Sì              | Medio-alto                                                                                                     |
| [Virtual machine](#virtual-machine)         | Sistema operativo completo                                                    | No              | Alto                                                                                                           |
| [Cloud sessions](#cloud-sessions)           | Sistema operativo completo, ospitato da Anthropic                             | No              | Nessuno; richiede un abbonamento Claude e un account GitHub connesso a meno che non avvii con `claude --cloud` |

Lo [sandboxed Bash tool](/docs/it/sandboxing) è integrato in Claude Code e limita i comandi Bash. I file tools integrati, i server MCP e gli hooks vengono comunque eseguiti direttamente sul tuo host. Ogni altro approccio nella tabella posiziona l'intero processo Claude Code all'interno del confine di isolamento, quindi i file tools, i server MCP e gli hooks sono anch'essi limitati.

<Warning>
  L'isolamento sandbox riduce l'impatto di una violazione, ma non elimina il rischio. Qualsiasi approccio che consente l'uscita di rete può comunque perdere dati che l'agente può leggere, e qualsiasi approccio che monta la tua directory di progetto in scrittura può comunque modificare quel codice. Rivedi le [limitazioni di sicurezza](/docs/it/sandboxing#security-limitations) prima di fare affidamento su una sandbox come controllo rigido.

  L'isolamento inoltre non cambia ciò che viene inviato al modello. I tuoi prompt e i file che Claude legge vengono trasmessi all'API Anthropic o al tuo provider configurato con o senza una sandbox. Vedi [Data usage](/docs/it/data-usage) per ciò che Claude Code invia e come ridurlo.
</Warning>

<h2 id="choose-an-approach">
  Scegliere un approccio
</h2>

Abbina il tuo obiettivo a una riga sottostante, quindi leggi la sezione di dettaglio che segue.

| Vuoi                                                                                                  | Inizia con                                                                                                                                                                |
| :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Ridurre i prompt di autorizzazione durante il lavoro quotidiano sulla tua macchina                    | Lo [sandboxed Bash tool](/docs/it/sandboxing), configurato con `/sandbox`                                                                                                      |
| Lasciare che Claude lavori in modo automatico con `--dangerously-skip-permissions` o in modalità auto | Il [dev container](/docs/it/devcontainer) preconfigurato, qualsiasi container o VM, o il [sandbox runtime](#sandbox-runtime)                                                   |
| Isolare i server MCP e gli hooks così come Bash, senza Docker                                         | Il sandbox runtime                                                                                                                                                        |
| Lavorare su un repository non attendibile                                                             | Una macchina virtuale dedicata, o una [sessione cloud](/docs/it/claude-code-on-the-web) se hai un abbonamento Claude; GitHub non è richiesto quando avvii con `claude --cloud` |
| Standardizzare un ambiente sandboxed in un team                                                       | Il [dev container](/docs/it/devcontainer) preconfigurato, copiato nel tuo repository                                                                                           |
| Usare Claude Code da un dispositivo senza setup locale                                                | Una [sessione cloud](/docs/it/claude-code-on-the-web), che richiede un abbonamento Claude e un account GitHub connesso                                                         |
| Richiedere l'isolamento per ogni sviluppatore nella tua organizzazione                                | [Applicare l'isolamento in un'organizzazione](#enforce-isolation-across-an-organization)                                                                                  |
| Lavorare su un host Windows nativo                                                                    | Un container o VM, o eseguire la sandbox Bash all'interno di WSL2                                                                                                         |

<h3 id="how-isolation-relates-to-permission-modes">
  Come l'isolamento si relaziona alle modalità di autorizzazione
</h3>

Le [modalità di autorizzazione](/docs/it/permission-modes) decidono se una chiamata di strumento viene eseguita e se sei richiesto per primo. L'isolamento limita ciò che un comando può accedere una volta eseguito. I due lavorano insieme: quando una modalità di autorizzazione consente l'esecuzione di azioni senza chiederti, un confine di isolamento limita ciò che quelle azioni possono raggiungere.

Quando passi `--dangerously-skip-permissions`, Claude agisce senza chiederti per primo. Le [azioni che nessuna modalità auto-approva](/docs/it/permission-modes#actions-no-mode-auto-approves) si applicano ancora.

Senza prompt per catturare gli errori, il confine di isolamento che scegli è ciò che protegge il tuo sistema. Esegui sempre le sessioni `--dangerously-skip-permissions` all'interno di un container, una VM, o il [sandbox runtime](#sandbox-runtime), in modo che i file tools, i server MCP e gli hooks siano anch'essi all'interno del confine. Su Linux e macOS, Claude Code rifiuta di avviarsi con questo flag quando viene eseguito come root, quindi esegui il container, la VM, o il sandbox runtime come utente non-root.

La [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) sostituisce il prompt con un classificatore che esamina le azioni. Il classificatore è un controllo per azione, non un confine di isolamento, quindi un confine di isolamento aggiunge comunque difesa in profondità per esecuzioni automatiche, e non è richiesto come lo è per `--dangerously-skip-permissions`.

Lo [sandboxed Bash tool](#sandboxed-bash-tool) da solo vincola solo i comandi shell, quindi non è sufficiente per esecuzioni completamente automatiche in nessuna delle due modalità. Puoi stratificare gli approcci: eseguire lo sandboxed Bash tool all'interno di un container o VM ti dà restrizioni di comando a livello di SO in cima al confine dell'ambiente esterno. Per come la sandbox Bash stessa interagisce con le regole di autorizzazione e le modalità di autorizzazione, vedi [Come il sandboxing si relaziona alle autorizzazioni e alle modalità di autorizzazione](/docs/it/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Strumento Bash in sandbox
</h2>

<Note>
  Questa opzione non supporta Windows nativo. Su host Windows, utilizzare WSL2 o uno degli approcci con container o VM di seguito.
</Note>

Lo strumento Bash in sandbox è integrato in Claude Code. Utilizza primitive del sistema operativo per limitare l'accesso al filesystem e alla rete di ogni comando Bash, PowerShell o Monitor che Claude esegue.

Eseguire il comando `/sandbox` per aprire il pannello sandbox e scegliere una modalità. La guida [Sandboxing](/docs/it/sandboxing) copre le modalità di approvazione, il limite predefinito e come ampliarlo o restringerlo.

La sandbox per comando non copre tutto ciò che viene eseguito in una sessione:

* Altri [strumenti integrati](/docs/it/tools-reference) come Read, Edit e WebFetch vengono eseguiti all'interno del processo Claude Code e non generano codice arbitrario. Le [regole di autorizzazione](/docs/it/permissions) per il percorso o il dominio li controllano invece.
* I server [MCP](/docs/it/mcp) e gli [hook di comando](/docs/it/hooks#command-hook-fields) sono processi separati che vengono eseguiti senza vincoli sull'host.

Per mettere gli strumenti integrati, i server MCP e gli hook tutti dietro un unico limite del sistema operativo, eseguire l'intero processo Claude Code all'interno del [runtime sandbox](#sandbox-runtime), del [dev container](#dev-containers) o di un [container personalizzato](#custom-container).

<h2 id="sandbox-runtime">
  Sandbox runtime
</h2>

Il pacchetto [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) avvolge un intero processo nello stesso isolamento Seatbelt o bubblewrap che la sandbox Bash integrata utilizza. Eseguire Claude Code attraverso il runtime vincola ogni strumento, hook e server MCP nella sessione, non solo i comandi shell. Il runtime è un'anteprima di ricerca beta, e il suo formato di configurazione potrebbe cambiare man mano che il pacchetto evolve.

Questa sezione copre ciò che configuri e ciò che il runtime applica da solo. Per distribuire il runtime nelle applicazioni Agent SDK, consulta la [guida alla distribuzione sicura](/docs/it/agent-sdk/secure-deployment#sandbox-runtime).

<h3 id="set-up-and-launch-the-runtime">
  Set up and launch the runtime
</h3>

Su Linux e WSL2, il runtime si basa sugli stessi pacchetti `bubblewrap` e `socat` della sandbox integrata, più `ripgrep`, che Claude Code raggruppa ma il runtime autonomo risolve dal tuo PATH. Installa `bubblewrap` e `socat` come descritto in [Set up Linux and WSL2](/docs/it/sandboxing#set-up-linux-and-wsl2), e `ripgrep` dal gestore di pacchetti della tua distribuzione. Su macOS non hai bisogno di pacchetti aggiuntivi. Il runtime utilizza la sandbox Seatbelt integrata lì.

Per impostazione predefinita il runtime nega l'accesso di rete e limita le scritture a un piccolo insieme di percorsi runtime integrati, quindi configuralo prima di lanciare Claude Code attraverso di esso. Metti la tua configurazione in `~/.srt-settings.json`, o in un file che passi con `--settings`. Il [README](https://github.com/anthropic-experimental/sandbox-runtime) del pacchetto documenta lo schema di configurazione completo.

Consenti l'accesso in scrittura ad almeno:

* La tua directory di progetto.
* I percorsi di configurazione di Claude Code `~/.claude` e `~/.claude.json`.
* `/tmp`, dove Claude Code scrive i file runtime.

Consenti i domini di rete di cui la tua sessione ha bisogno:

* `api.anthropic.com`, o l'endpoint del tuo provider configurato. Su un provider di terze parti, mantieni anche `api.anthropic.com`: il controllo di sicurezza del dominio WebFetch lo chiama comunque per impostazione predefinita a meno che tu non imposti `skipWebFetchPreflight: true`.
* `claude.ai` e `platform.claude.com`, che [OAuth sign-in e token refresh](/docs/it/network-config#network-access-requirements) richiedono. Le esecuzioni autenticate con una chiave API possono eliminare questi due.

Su Linux e WSL2, il runtime applica i permessi di scrittura solo ai percorsi che già esistono. In un ambiente nuovo, crea i percorsi di configurazione di Claude Code prima del primo avvio:

```bash theme={null}
mkdir -p ~/.claude && echo '{}' > ~/.claude.json
```

Una volta che il file di impostazioni è in posizione, avvia Claude Code con `npx` e passa `claude` come comando da avvolgere:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code si avvia all'interno della sandbox con i confini di filesystem e di rete che hai configurato. Lo stesso comando funziona per il sandboxing di server MCP autonomi o altri processi di supporto.

<h3 id="what-the-runtime-blocks-on-its-own">
  What the runtime blocks on its own
</h3>

Il runtime blocca le scritture a rischio più elevato senza alcuna configurazione da parte tua:

* `denyWrite` ha la precedenza su `allowWrite`.
* Alla radice del progetto, il runtime nega `.git/hooks`, nega `.git/config` a meno che tu non imposti `filesystem.allowGitConfig: true`, e nega `.mcp.json`, `.claude/commands`, `.claude/agents`, e i file di avvio della shell.
* Su macOS, questi dinieghi vengono controllati quando avviene una scrittura, quindi coprono anche i file annidati e i repository creati durante la sessione.
* Su Linux e WSL2, il runtime costruisce l'elenco di diniego una volta all'avvio. Copre in modo affidabile la radice del progetto, esegue una scansione superficiale nel migliore dei casi per le copie annidate che esistono in quel momento, e non copre nulla che la sessione crea in seguito, come `git init`, `git clone`, o scaffolding. La sezione `mandatoryDenySearchDepth` del README descrive la semantica esatta della scansione.
* Senza un `~/.srt-settings.json` valido, il runtime si avvia comunque, blocca l'accesso di rete, e limita le scritture ai percorsi runtime integrati come `/tmp/claude`, `~/.npm/_logs`, e `~/.claude/debug`. Non prendere un avvio pulito come prova che le tue impostazioni sono state caricate.
* Quando passi `--settings`, il runtime rifiuta di avviarsi se il file non riesce a caricarsi.

I tuoi permessi di scrittura includono ancora altri percorsi da cui Claude Code carica la configurazione, quindi nega quelli con `denyWrite`. Una sessione in sandbox che può scriverli può persistere hook, regole di permesso, o server MCP che vengono eseguiti senza sandbox la prossima volta che avvii Claude Code.

<h3 id="after-unattended-runs">
  After unattended runs
</h3>

Rivedi i percorsi che hai mantenuto scrivibili. Su Linux e WSL2, rivedi anche tutto ciò che la sessione ha creato.

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

Una macchina virtuale dedicata fornisce la separazione più forte, con il suo kernel e, nei deployment cloud o microVM, il suo hardware virtualizzato. Le opzioni includono istanze cloud, hypervisor locali e microVM come Firecracker. Usa questo approccio quando stai valutando codice non attendibile, quando la tua politica di sicurezza richiede separazione a livello di kernel tra l'agente e l'host, o quando nessun approccio a livello di host soddisfa i tuoi requisiti di conformità.

[Docker Sandboxes](https://docs.docker.com/ai/sandboxes/) fornisce una microVM con il suo daemon Docker e sincronizzazione dell'area di lavoro, che può eseguire Claude Code su qualsiasi host con Docker Sandboxes installato. È un prodotto gratuito e autonomo di Docker che non richiede Docker Desktop.

<h2 id="cloud-sessions">
  Cloud sessions
</h2>

Una [sessione cloud](/docs/it/claude-code-on-the-web) viene eseguita in una macchina virtuale isolata gestita da Anthropic. Un proxy di rete applica una whitelist predefinita, e un proxy separato tiene il vostro token GitHub al di fuori della sandbox mentre emette credenziali scoped per l'accesso al repository all'interno di essa. Le sessioni che la vostra organizzazione instrada a un [ambiente self-hosted](/docs/it/self-hosted-environments) vengono eseguite su infrastruttura che voi stessi provisionate, dove l'isolamento, il controllo dell'egress e le credenziali git sono responsabilità della vostra distribuzione.

Utilizzate questo approccio quando desiderate l'isolamento completo della VM senza provisioning dell'infrastruttura da soli, o quando state delegando attività da un dispositivo che non dispone di un ambiente di sviluppo locale. Richiede un abbonamento Claude. A meno che non avviate dalla CLI, avete anche bisogno di un account GitHub connesso affinché la sandbox possa clonare il vostro repository. Quando avviate dalla CLI con `--cloud`, Claude Code può [raggruppare e caricare il vostro repository locale](/docs/it/claude-code-on-the-web#send-local-repositories-without-github) invece. Consultate [Utilizzare Claude Code nel cloud](/docs/it/claude-code-on-the-web) per la disponibilità del piano e le opzioni di autenticazione GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Applicare l'isolamento in un'organizzazione
</h2>

I singoli sviluppatori possono optare per qualsiasi approccio di sandboxing su questa pagina. Ciò che un'organizzazione può applicare, e con quali strumenti, dipende dall'approccio:

* **Built-in Bash sandbox**: l'unico approccio che Claude Code applica da solo. Fornisci le chiavi di impostazioni `sandbox` attraverso [managed settings](/docs/it/managed-settings#delivery-mechanisms), sia come file gestito dal tuo MDM che attraverso [server-managed settings](/docs/it/server-managed-settings) su Claude.ai. Vedi [Enforce sandboxing with managed settings](/docs/it/sandboxing#enforce-sandboxing-with-managed-settings) per le chiavi da distribuire e come impedire agli sviluppatori di ampliare la politica.
* **Dev containers**: esegui il commit dell'[esempio di dev container](/docs/it/devcontainer) nei tuoi repository per standardizzare l'ambiente in un team. Questa è una convenzione piuttosto che un confine di applicazione, perché Claude Code non richiede un container. Se gli sviluppatori non dovrebbero essere in grado di eseguire Claude Code al di fuori di esso, applica ciò con gli strumenti di gestione dei dispositivi della tua organizzazione o di allowlisting del software.
* **Custom containers e VMs**: distribuisci Claude Code attraverso l'immagine approvata e usa gli strumenti di gestione dei dispositivi della tua organizzazione o di allowlisting del software per prevenire l'installazione al di fuori di essa.

<h2 id="see-also">
  Vedi anche
</h2>

Queste pagine coprono i dettagli di configurazione e politica per gli approcci di sandboxing su questa pagina.

* [Sandboxing](/docs/it/sandboxing): configura lo sandboxed Bash tool integrato
* [Dev container](/docs/it/devcontainer): il container di sviluppo Docker preconfigurato
* [Security](/docs/it/security): il modello di sicurezza completo di Claude Code
* [Secure deployment](/docs/it/agent-sdk/secure-deployment): guida all'isolamento per le applicazioni Agent SDK
* [Settings](/docs/it/settings-reference#sandbox-settings): tutte le chiavi di configurazione sandbox, inclusa la consegna di managed settings
