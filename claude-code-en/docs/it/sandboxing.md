> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura lo strumento Bash in sandbox

> Scopri come lo strumento Bash in sandbox di Claude Code fornisce isolamento del filesystem e della rete per un'esecuzione dell'agente più sicura e autonoma.

La sandbox Bash consente a Claude di eseguire la maggior parte dei comandi shell senza fermarsi per chiedere autorizzazione. Invece di approvare ogni comando, definisci quali file e domini di rete i comandi possono toccare, e il sistema operativo applica quel confine per ogni comando Bash e i suoi processi figlio.

<Note>
  Per confrontare altri approcci di isolamento come dev container, container personalizzati e macchine virtuali, vedi [Ambienti sandbox](/docs/it/sandbox-environments). Per ridurre i prompt di autorizzazione per strumenti diversi da Bash, vedi [modalità di autorizzazione](/docs/it/permission-modes).
</Note>

<h2 id="get-started">
  Get started
</h2>

La sandbox è integrata in Claude Code e viene eseguita su macOS, Linux e WSL2. Windows nativo non è supportato. Su Windows, esegui Claude Code all'interno di una distribuzione WSL2.

Su macOS, non c'è nulla da installare: il sandboxing utilizza il framework Seatbelt integrato. Su Linux e WSL2, la sandbox si basa su due pacchetti, trattati in [Set up Linux and WSL2](#set-up-linux-and-wsl2). Anche se non li hai ancora installati, puoi iniziare con `/sandbox`, perché il suo pannello mostra se manca qualcosa.

<Steps>
  <Step title="Esegui /sandbox">
    Avvia una sessione di Claude Code ed esegui il comando `/sandbox`:

    ```text theme={null}
    /sandbox
    ```

    Questo apre il pannello sandbox con tre schede:

    * **Mode**: scegli come i comandi sandboxati vengono approvati, trattato nel passaggio successivo
    * **Overrides**: scegli se i comandi che falliscono sotto la sandbox possono ricadere nell'esecuzione non sandboxata. Questa è l'impostazione [`allowUnsandboxedCommands`](/docs/it/settings#sandbox-settings)
    * **Config**: visualizza le impostazioni sandbox risolte

    Se il pannello mostra solo una scheda Dependencies, manca un pacchetto richiesto. Installalo come descritto in [Set up Linux and WSL2](#set-up-linux-and-wsl2), riavvia Claude Code ed esegui `/sandbox` di nuovo.
  </Step>

  <Step title="Scegli una modalità">
    Nella scheda Mode, seleziona auto-allow o autorizzazioni regolari. Auto-allow esegue i comandi sandboxati senza richiedere, e le autorizzazioni regolari mantengono i prompt di autorizzazione regolari anche quando i comandi sono sandboxati. Vedi [Modalità sandbox](#sandbox-modes) per quali comandi richiedono comunque prompt in modalità auto-allow.
  </Step>

  <Step title="Esegui un comando Bash">
    Chiedi a Claude di eseguire un comando, come una build o una suite di test. Per impostazione predefinita, i comandi all'interno della sandbox possono scrivere solo nella directory di lavoro e nella directory temporanea della sessione. La prima volta che un comando ha bisogno di un nuovo dominio di rete, Claude Code richiede l'approvazione.

    I comandi che non possono essere eseguiti sandboxati ricadono nel flusso di autorizzazione regolare. Per ampliare o restringere questi confini, vedi [Configure sandboxing](#configure-sandboxing).
  </Step>
</Steps>

Selezionare una modalità nel pannello scrive nelle impostazioni locali del tuo progetto in `.claude/settings.local.json`, che si applicano al progetto corrente e non vengono controllate in git. Per abilitare la sandbox in tutti i tuoi progetti, imposta [`sandbox.enabled`](/docs/it/settings#sandbox-settings) su `true` nelle impostazioni utente in `~/.claude/settings.json`. Per applicare il sandboxing per ogni sviluppatore in un'organizzazione, utilizza [impostazioni gestite](#enforce-sandboxing-with-managed-settings).

<Warning>
  Per impostazione predefinita, se la sandbox non può avviarsi perché mancano dipendenze o la piattaforma non è supportata, Claude Code mostra un avviso ed esegue i comandi senza sandboxing. Per rendere questo un errore grave, imposta [`sandbox.failIfUnavailable`](/docs/it/settings#sandbox-settings) su `true`. Questo è destinato a distribuzioni gestite che richiedono il sandboxing come gate di sicurezza.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Set up Linux and WSL2
</h3>

Su Linux e WSL2, la sandbox si basa su due pacchetti:

* [`bubblewrap`](https://github.com/containers/bubblewrap): lo strumento di sandboxing senza privilegi che applica l'isolamento del filesystem
* [`socat`](http://www.dest-unreach.org/socat/): il relay utilizzato per instradare il traffico di rete attraverso il proxy sandbox

Installali con il gestore di pacchetti della tua distribuzione:

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

Dopo l'installazione, la scheda Dependencies in `/sandbox` mostra se `ripgrep`, `bubblewrap`, `socat` e il filtro seccomp sono disponibili sulla tua piattaforma. Ripgrep è incluso nel binario nativo di Claude Code. Il filtro seccomp è facoltativo e aggiunge il blocco del socket di dominio Unix. Installalo con `npm install -g @anthropic-ai/sandbox-runtime` se manca.

Quando manca una dipendenza richiesta, la scheda Dependencies è l'unica scheda mostrata fino a quando non la installi. Il controllo delle dipendenze viene eseguito all'avvio, quindi riavvia Claude Code dopo l'installazione dei pacchetti affinché `/sandbox` li rilevi.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 e versioni successive: consenti a bubblewrap di creare spazi dei nomi utente">
    Su Ubuntu 24.04 e versioni successive, la politica AppArmor predefinita impedisce a bubblewrap di creare gli spazi dei nomi utente di cui ha bisogno per l'isolamento.

    Per verificare se il tuo ambiente applica questa restrizione, incluso all'interno di WSL2, esegui `sysctl kernel.apparmor_restrict_unprivileged_userns`. Se la chiave non esiste o restituisce `0`, salta questo passaggio. Se restituisce `1`, aggiungi un profilo AppArmor che conceda a `bwrap` questa capacità:

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    Il profilo si applica solo a `bwrap` stesso, non ai comandi che esegue all'interno della sandbox. Ricarica AppArmor per applicarlo:

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="Note su WSL2">
    Controlla la tua versione WSL con `wsl -l -v` da PowerShell. Se vedi `Sandboxing requires WSL2`, la tua distribuzione sta eseguendo WSL1. Aggiornala a WSL2 o esegui Claude Code senza sandboxing.

    Su WSL2, i comandi sandboxati non possono avviare binari Windows come `cmd.exe`, `powershell.exe`, o qualsiasi cosa sotto `/mnt/c/`. WSL li passa all'host Windows su un socket Unix, che la sandbox blocca. Se un comando ha bisogno di invocare un binario Windows, aggiungilo a [`excludedCommands`](/docs/it/settings#sandbox-settings) in modo che venga eseguito al di fuori della sandbox.
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Sandbox modes
</h3>

Claude Code offre due modalità sandbox:

**Modalità auto-allow**: I comandi Bash tenteranno di eseguire all'interno della sandbox e sono automaticamente consentiti senza richiedere autorizzazione. I comandi che non possono essere sandboxati, come quelli che necessitano di accesso alla rete a host non consentiti, ricadono nel flusso di autorizzazione regolare, dove Claude Code controlla le tue [regole di autorizzazione](/docs/it/permissions) e ti richiede per qualsiasi comando che quelle regole non consentono già, con un prompt in modalità predefinita o il classificatore in [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode).

Anche in modalità auto-allow, si applicano i seguenti:

* Le [regole di negazione](/docs/it/permissions) esplicite sono sempre rispettate
* I comandi `rm` o `rmdir` che puntano a `/`, alla tua directory home o ad altri percorsi critici del sistema attivano comunque un prompt di autorizzazione
* Le [regole ask](/docs/it/permissions) con ambito di contenuto come `Bash(git push *)` forzano comunque un prompt anche per i comandi sandboxati
* Una regola ask `Bash` semplice, o la forma equivalente `Bash(*)`, viene saltata per i comandi che vengono eseguiti sandboxati; si applica comunque ai comandi che ricadono nel flusso di autorizzazione regolare

**Modalità autorizzazioni regolari**: Tutti i comandi Bash passano attraverso il flusso di autorizzazione regolare, anche quando sandboxati. Questo fornisce più controllo ma richiede più approvazioni.

In entrambe le modalità, la sandbox applica le stesse restrizioni di filesystem e rete. La differenza è solo se i comandi sandboxati sono auto-approvati o richiedono autorizzazione esplicita.

La directory temporanea della sessione è scrivibile all'interno della sandbox per impostazione predefinita, insieme alla directory di lavoro. Claude Code imposta `$TMPDIR` su questa directory per i comandi sandboxati, quindi gli strumenti che scrivono file temporanei funzionano senza configurazione aggiuntiva. I comandi non sandboxati ereditano il tuo `$TMPDIR` della shell invariato, il che significa che i comandi sandboxati e non sandboxati risolvono `$TMPDIR` in directory diverse. Per passare file temporanei tra i due, scrivili nella directory di lavoro.

Alcuni comandi non possono essere eseguiti all'interno della sandbox, come strumenti incompatibili con essa o che necessitano di un host che non hai consentito. Piuttosto che fallire il compito o richiedere di disattivare il sandboxing, Claude Code include un escape hatch: quando un comando fallisce a causa di restrizioni della sandbox, Claude analizza il fallimento e potrebbe riprovare il comando con il parametro `dangerouslyDisableSandbox`. Il comando riprovato viene eseguito al di fuori della sandbox, quindi passa attraverso il flusso di autorizzazione regolare: in modalità predefinita ottieni un prompt di conferma; in [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) il classificatore valuta il comando sottostante invece di richiedere la tua approvazione. Per essere richiesto su ogni riprovazione non sandboxata anche in modalità auto, aggiungi una [regola ask](/docs/it/permissions#match-by-input-parameter) per `Bash(dangerouslyDisableSandbox:true)`.

Puoi disabilitare questo escape hatch impostando `"allowUnsandboxedCommands": false` nelle tue [impostazioni sandbox](/docs/it/settings#sandbox-settings). Quando disabilitato, che la scheda Overrides di `/sandbox` mostra come **Modalità sandbox rigorosa**, il parametro `dangerouslyDisableSandbox` viene completamente ignorato e tutti i comandi devono essere eseguiti sandboxati o essere esplicitamente elencati in `excludedCommands`.

<Info>
  La modalità auto-allow funziona indipendentemente dall'impostazione della modalità di autorizzazione. Anche se non sei in modalità "accetta modifiche", i comandi Bash sandboxati verranno eseguiti automaticamente quando auto-allow è abilitato. Ciò significa che i comandi Bash che modificano file entro i confini della sandbox verranno eseguiti senza richiedere, anche quando gli strumenti di modifica dei file normalmente richiederebbero approvazione.
</Info>

<h2 id="configure-sandboxing">
  Configure sandboxing
</h2>

Personalizza il comportamento della sandbox tramite il file `settings.json`. Vedi [Settings](/docs/it/settings#sandbox-settings) per il riferimento di configurazione completo.

Per impostazione predefinita, i comandi sandboxati possono scrivere solo nella directory di lavoro corrente e nella directory temporanea della sessione. Se i comandi dei sottoprocessi come `kubectl`, `terraform` o `npm` devono scrivere al di fuori di quelle directory, utilizza `sandbox.filesystem.allowWrite` per concedere l'accesso a percorsi specifici:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

Questi percorsi sono applicati a livello del sistema operativo, quindi tutti i comandi in esecuzione all'interno della sandbox, inclusi i loro processi figlio, li rispettano. Questo è l'approccio consigliato quando uno strumento ha bisogno di accesso in scrittura a una posizione specifica, piuttosto che escludere completamente lo strumento dalla sandbox con `excludedCommands`.

Quando lo stesso array di filesystem è definito in più [ambiti di impostazioni](/docs/it/settings#settings-precedence), gli array vengono uniti: i percorsi da ogni ambito vengono combinati, non sostituiti.

I prefissi di percorso controllano come i percorsi vengono risolti:

| Prefisso               | Significato                                                                                                    | Esempio                                                                     |
| :--------------------- | :------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `/`                    | Percorso assoluto dalla radice del filesystem                                                                  | `/tmp/build` rimane `/tmp/build`                                            |
| `~/`                   | Relativo alla directory home                                                                                   | `~/.kube` diventa `$HOME/.kube`                                             |
| `./` o nessun prefisso | Relativo alla radice del progetto per le impostazioni del progetto, o a `~/.claude` per le impostazioni utente | `./output` in `.claude/settings.json` si risolve in `<project-root>/output` |

Questa sintassi differisce dalle [regole di autorizzazione Read e Edit](/docs/it/permissions#read-and-edit), che utilizzano `//path` per assoluto e `/path` per relativo al progetto. I percorsi del filesystem della sandbox utilizzano convenzioni standard: `/tmp/build` è assoluto.

Puoi anche negare l'accesso in scrittura o lettura utilizzando `sandbox.filesystem.denyWrite` e `sandbox.filesystem.denyRead`, e ri-consentire percorsi specifici all'interno di una regione negata utilizzando `sandbox.filesystem.allowRead`. Quando le regole di lettura si sovrappongono, il percorso più specifico vince:

| Regole di esempio                                      | Risultato                                                                                                                                                                                         |
| :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `"denyRead": ["~/"]` con `"allowRead": ["~/projects"]` | `~/projects` è leggibile e il resto della directory home rimane bloccato. L'allow più stretto riaprire quella parte della regione negata                                                          |
| `"allowRead": ["~/"]` con `"denyRead": ["~/.env"]`     | `~/.env` rimane bloccato e il resto della directory home è leggibile. Un deny esatto rimane all'interno di un allow più ampio, quindi un allow ampio non può silenziosamente riesporsi un segreto |

L'esempio seguente blocca la lettura dall'intera directory home consentendo comunque letture dal progetto corrente. Posizionalo nel `.claude/settings.json` del tuo progetto, perché il percorso relativo `.` si risolve nella radice del progetto solo quando la configurazione si trova nelle impostazioni del progetto:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Il `.` in `allowRead` si risolve nella radice del progetto perché questa configurazione si trova nelle impostazioni del progetto. Se hai posizionato la stessa configurazione in `~/.claude/settings.json`, `.` si risolverebbe in `~/.claude` invece, e i file del progetto rimarrebbero bloccati dalla regola `denyRead`.

<h3 id="protect-credentials">
  Protect credentials
</h3>

L'impostazione `sandbox.credentials` dichiara file di credenziali e variabili di ambiente da proteggere dai comandi sandboxati. Ogni voce nomina un percorso di file o una variabile di ambiente e una `mode`. Il blocco dedicato `credentials` mantiene le regole delle credenziali raggruppate insieme e separate dalle regole generali del filesystem. Richiede Claude Code v2.1.187 o successivo.

Per le voci con `"mode": "deny"`, i percorsi dei file vengono negati per le letture all'interno della sandbox, la stessa restrizione che applica `filesystem.denyRead`, e le variabili di ambiente vengono rimosse prima di ogni esecuzione di comando sandboxato.

L'esempio seguente blocca le letture del file delle credenziali AWS e della directory SSH e rimuove `GITHUB_TOKEN` e `NPM_TOKEN` dall'ambiente dei comandi sandboxati:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

Le voci di file supportano solo `"mode": "deny"`. Le voci di variabili di ambiente accettano anche `"mode": "mask"`, descritto di seguito.

I percorsi dei file seguono le stesse [regole di prefisso](/docs/it/settings#sandbox-path-prefixes) delle impostazioni `sandbox.filesystem.*`, e le voci `deny` da ogni [ambito di impostazioni](/docs/it/settings#settings-precedence) vengono unite. Una voce `deny` restringe solo l'accesso, quindi qualsiasi ambito può aggiungerne una, ma nessun ambito può rimuoverne una che un altro ambito ha aggiunto.

Non esiste un elenco di negazione delle credenziali integrato, quindi solo i file e le variabili che elenchi sono limitati. L'impostazione influisce solo sui comandi Bash sandboxati. Per rimuovere le credenziali di Anthropic e dei provider cloud da tutti i sottoprocessi indipendentemente dal sandboxing, imposta [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/it/env-vars).

<h4 id="mask-environment-variables">
  Mask environment variables
</h4>

`"mode": "mask"` protegge una credenziale mantenendo il funzionamento degli strumenti che si autenticano con essa. `deny` rimuove completamente la variabile, il che interrompe anche gli strumenti che ne hanno bisogno, come `gh` o `npm`. Richiede Claude Code v2.1.199 o successivo.

Con `mask`, il comando sandboxato vede un valore sentinella per sessione invece di quello reale. Quando una richiesta esce dalla sandbox per uno degli `injectHosts` della credenziale, il [proxy della sandbox](#network-isolation) sostituisce il sentinella con il valore reale. Il comando e tutto ciò che registra non contengono mai la credenziale reale, ma le sue richieste si autenticano comunque.

Il proxy sostituisce la credenziale all'interno dei contenuti della richiesta, quindi deve vederli. Imposta [`network.tlsTerminate`](/docs/it/settings#sandbox-settings) in modo che il proxy termini TLS stesso. Senza di esso, il mascheramento fallisce in modo chiuso: il comando vede ancora solo il sentinella, ma il sentinella raggiunge il server invariato e l'autenticazione fallisce. Claude Code segnala questa configurazione errata all'avvio.

L'esempio seguente maschera due token. `GH_TOKEN` viene sostituito solo sulle richieste a `api.github.com`, mentre `NPM_TOKEN` non ha `injectHosts` e viene sostituito sulle richieste a ogni host in `network.allowedDomains`. Ogni voce `injectHosts` deve essere coperta da `network.allowedDomains`.

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

A differenza di `deny`, il mascheramento autorizza il proxy a inviare la tua credenziale reale agli host elencati, quindi viene rispettato solo dalle impostazioni che tu o il tuo amministratore controllate: impostazioni utente, impostazioni gestite e il flag CLI `--settings`. Le voci `mask`, `network.tlsTerminate` e [`credentials.allowPlaintextInject`](/docs/it/settings#sandbox-settings) nel `.claude/settings.json` o `.claude/settings.local.json` di un repository vengono ignorate.

Quando la stessa variabile è elencata con `deny` in qualsiasi ambito, `deny` ha la precedenza.

<h2 id="how-sandboxing-works">
  Come funziona il sandboxing
</h2>

<h3 id="filesystem-isolation">
  Isolamento del file system
</h3>

Lo strumento Bash in sandbox limita l'accesso al file system a directory specifiche:

* **Comportamento di scrittura predefinito**: accesso in lettura e scrittura alla directory di lavoro corrente e alle sue sottodirectory, più la directory temporanea della sessione a cui `$TMPDIR` punta
* **Comportamento di lettura predefinito**: accesso in lettura all'intero computer, ad eccezione di determinate directory negate. Nota che questo predefinito consente comunque la lettura di file di credenziali come `~/.aws/credentials` e `~/.ssh/`. Utilizza [`sandbox.credentials`](#protect-credentials) per bloccare le letture di questi file e annullare le variabili di ambiente segrete, oppure aggiungi i percorsi a `denyRead`.
* **Accesso bloccato**: non è possibile modificare file al di fuori della directory di lavoro corrente e della directory temporanea della sessione senza autorizzazione esplicita, inclusi file di configurazione shell come `~/.bashrc` e binari di sistema in `/bin/`
* **Git worktrees**: quando la directory di lavoro è un [git worktree collegato](/docs/it/worktrees), la sandbox consente anche scritture nella directory `.git` condivisa del repository principale in modo che comandi come `git commit` possano aggiornare i ref e l'indice. Le scritture in `hooks/` e `config` all'interno di quella directory rimangono negate.
* **Configurabile**: definisci percorsi consentiti e negati personalizzati tramite le impostazioni

Puoi concedere l'accesso in scrittura a percorsi aggiuntivi utilizzando `sandbox.filesystem.allowWrite` nelle impostazioni. Queste restrizioni sono applicate a livello del sistema operativo, quindi si applicano a tutti i comandi dei sottoprocessi, inclusi strumenti come `kubectl`, `terraform` e `npm`, non solo agli strumenti di file di Claude.

<h3 id="network-isolation">
  Isolamento della rete
</h3>

L'accesso alla rete è controllato tramite un server proxy in esecuzione al di fuori della sandbox:

* **Restrizioni di dominio**: nessun dominio è pre-consentito. La prima volta che un comando ha bisogno di un nuovo dominio, Claude Code richiede l'approvazione. A partire dalla v2.1.191, scegliere Sì consente l'host per il resto della sessione corrente, quindi le connessioni successive allo stesso host non richiedono di nuovo il prompt. Pre-consenti i domini con [`allowedDomains`](/docs/it/settings#sandbox-settings) per evitare completamente il prompt.
* **Blocco gestito**: se [`allowManagedDomainsOnly`](/docs/it/settings#sandbox-settings) è impostato nelle impostazioni gestite, i domini non consentiti vengono bloccati automaticamente invece di richiedere, e solo `allowedDomains` dalle impostazioni gestite vengono rispettati.
* **Supporto proxy personalizzato**: gli utenti avanzati possono implementare regole personalizzate sul traffico in uscita
* **Copertura completa**: le restrizioni si applicano a tutti gli script, programmi e sottoprocessi generati dai comandi

<Note>
  Il proxy integrato applica l'allowlist in base al nome host richiesto e, per impostazione predefinita, non termina o ispeziona il traffico TLS. L'impostazione sperimentale [`network.tlsTerminate`](/docs/it/settings#sandbox-settings), disponibile in Claude Code v2.1.199 e versioni successive, fa sì che il proxy integrato termini TLS stesso, che le voci di credenziali [`mask`](#protect-credentials) richiedono. Vedi [Limitazioni di sicurezza](#security-limitations) per le implicazioni dell'impostazione predefinita, e [Configurazione proxy personalizzata](#custom-proxy-configuration) se il tuo modello di minaccia richiede l'ispezione TLS.
</Note>

<h3 id="os-level-enforcement">
  Applicazione a livello del sistema operativo
</h3>

Lo strumento Bash in sandbox sfrutta le primitive di sicurezza del sistema operativo:

* **macOS**: utilizza Seatbelt per l'applicazione della sandbox
* **Linux**: utilizza [bubblewrap](https://github.com/containers/bubblewrap) per l'isolamento
* **WSL2**: utilizza bubblewrap, come Linux

WSL1 non è supportato perché bubblewrap richiede funzionalità del kernel disponibili solo in WSL2. Queste restrizioni a livello del sistema operativo assicurano che tutti i processi figlio generati dai comandi di Claude Code ereditino gli stessi confini di sicurezza.

Questi stessi primitivi sono disponibili come pacchetto standalone [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime), che la pagina [Sandbox environments](/docs/it/sandbox-environments#sandbox-runtime) copre come approccio separato per avvolgere l'intero processo di Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Come il sandboxing si relaziona alle autorizzazioni e alle modalità di autorizzazione
</h2>

Il sandboxing, le [regole di autorizzazione](/docs/it/permissions) e le [modalità di autorizzazione](/docs/it/permission-modes) sono livelli complementari. Le sezioni seguenti coprono come la sandbox interagisce con ciascuno.

<h3 id="permission-rules">
  Permission rules
</h3>

Le regole di autorizzazione e il sandboxing controllano cose diverse:

* **Le regole di autorizzazione** controllano quali strumenti Claude Code può utilizzare e vengono valutate prima che qualsiasi strumento venga eseguito. Si applicano a tutti gli strumenti: Bash, Read, Edit, WebFetch, MCP e altri.
* **Il sandboxing** fornisce l'applicazione a livello del sistema operativo che limita ciò che i comandi Bash possono accedere a livello di filesystem e rete. Si applica solo ai comandi Bash e ai loro processi figlio.

I due livelli differiscono anche nel modo in cui vengono applicati. Claude Code valuta le decisioni di autorizzazione prima che un comando venga eseguito, in base alla stringa di comando e, in modalità auto, il giudizio di un classificatore separato su se il comando è sicuro. Il sistema operativo applica il confine della sandbox al processo in esecuzione, quindi vale indipendentemente da ciò che il modello ha scelto di eseguire e anche se un comando consentito fa più di quanto il suo nome suggerisca.

Le restrizioni di filesystem e rete sono configurate sia tramite le impostazioni della sandbox che le regole di autorizzazione:

| Impostazione o regola                                          | Cosa fa                                                                                                    |
| :------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                | Concede l'accesso in scrittura dei sottoprocessi a percorsi al di fuori della directory di lavoro          |
| `sandbox.filesystem.denyWrite` e `sandbox.filesystem.denyRead` | Blocca l'accesso dei sottoprocessi a percorsi specifici                                                    |
| `sandbox.filesystem.allowRead`                                 | Ri-consente la lettura di percorsi specifici all'interno di una regione `denyRead`                         |
| Regole di consentimento `Edit`                                 | Concedono l'accesso in scrittura a percorsi specifici, allo stesso modo di `sandbox.filesystem.allowWrite` |
| Regole di negazione `Read` e `Edit`                            | Bloccano l'accesso a file o directory specifici                                                            |
| Regole di consentimento e negazione `WebFetch`                 | Controllano l'accesso al dominio                                                                           |
| Sandbox `allowedDomains`                                       | Controlla quali domini i comandi Bash possono raggiungere                                                  |
| Sandbox `deniedDomains`                                        | Blocca domini specifici anche quando un wildcard `allowedDomains` più ampio altrimenti li permetterebbe    |

I percorsi dalle impostazioni `sandbox.filesystem` e dalle regole di autorizzazione vengono uniti insieme nella configurazione finale della sandbox.

Il [repository claude-code nella directory examples](https://github.com/anthropics/claude-code/tree/main/examples/settings) include configurazioni di impostazioni iniziali per scenari di distribuzione comuni, inclusi esempi specifici della sandbox. Utilizzali come punti di partenza e adattali alle tue esigenze.

<h3 id="permission-modes">
  Permission modes
</h3>

`/sandbox` non è una [modalità di autorizzazione](/docs/it/permission-modes). Le modalità di autorizzazione decidono se una chiamata di strumento viene eseguita e se vieni richiesto prima, mentre la sandbox limita ciò che un comando Bash può accedere una volta che viene eseguito. Differiscono in ciò che controllano e cosa sostituisce il prompt per azione:

|                                                                        | Cosa controlla                                                 | Cosa sostituisce il prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :--------------------------------------------------------------------- | :------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                             | Cosa un comando Bash può accedere una volta che viene eseguito | Il confine della sandbox stesso, in [modalità auto-allow](#sandbox-modes)                                                                                                                                                                                                                                                                                                                                                                                                                          |
| [Modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) | Se ogni chiamata di strumento viene eseguita                   | Un classificatore che esamina le azioni                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `--dangerously-skip-permissions`                                       | Se ogni chiamata di strumento viene eseguita                   | Niente. I controlli di [percorso protetto](/docs/it/permission-modes#protected-paths) vengono anche saltati; solo le [regole ask](/docs/it/permissions#manage-permissions) esplicite, gli strumenti connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) e la rimozione di `/` o della tua directory home richiedono comunque un prompt |

La [modalità auto-allow](#sandbox-modes) della sandbox è separata dalla [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode): auto-allow approva i comandi Bash perché il confine della sandbox li contiene, mentre la modalità auto utilizza un classificatore per esaminare le azioni. I due funzionano indipendentemente e possono essere combinati. Per scegliere un confine di isolamento per esecuzioni incustodite, vedi [Sandbox environments](/docs/it/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="configure-the-sandbox-for-your-organization">
  Configura la sandbox per la tua organizzazione
</h2>

Gli amministratori possono richiedere il sandboxing per ogni utente, impedire agli sviluppatori di ampliare la politica e instradare il traffico sandbox attraverso un proxy aziendale.

<h3 id="enforce-sandboxing-with-managed-settings">
  Enforce sandboxing with managed settings
</h3>

Per richiedere la sandbox per ogni sviluppatore, fornisci le chiavi `sandbox` tramite [impostazioni gestite](/docs/it/settings#settings-files), sia come file gestito dal tuo MDM che tramite [impostazioni gestite dal server](/docs/it/server-managed-settings) su Claude.ai.

La seguente configurazione di impostazioni gestite abilita la sandbox, rifiuta di avviare Claude Code se la sandbox non può inizializzarsi e impedisce al modello di riprovare i comandi al di fuori della sandbox:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

Le due chiavi oltre `enabled` controllano cosa succede quando la sandbox non può eseguire un comando:

* **`failIfUnavailable`**: una dipendenza mancante come bubblewrap su Linux blocca l'avvio di Claude Code piuttosto che mostrare un avviso e ricadere nell'esecuzione non sandboxata
* **`allowUnsandboxedCommands: false`**: l'escape hatch `dangerouslyDisableSandbox` viene ignorato, quindi i comandi che falliscono sotto la sandbox non possono essere riprovati al di fuori di essa

Due aggiunte meritano considerazione insieme a loro. Aggiungi `excludedCommands` per qualsiasi strumento approvato dall'organizzazione che deve essere eseguito senza isolamento. Aggiungi voci [`sandbox.credentials`](#protect-credentials) per directory di credenziali come `~/.aws` e `~/.ssh` e per variabili di ambiente segrete, poiché la politica di lettura predefinita le consente comunque.

La sandbox non viene eseguita su Windows nativo, quindi se la tua flotta include host Windows, limita questa configurazione a macOS e Linux o fai in modo che quegli utenti eseguano Claude Code all'interno di WSL2 o di un container.

<h3 id="keep-developers-from-widening-the-policy">
  Keep developers from widening the policy
</h3>

Per chiavi booleane come `enabled` e `failIfUnavailable`, Claude Code utilizza il valore gestito e ignora qualsiasi cosa uno sviluppatore imposti localmente. Per chiavi array come `excludedCommands` e `allowRead`, Claude Code unisce le voci da ogni ambito, quindi uno sviluppatore può aggiungere voci che ampliano la politica.

Imposta `allowManagedReadPathsOnly` su `true` nelle impostazioni gestite in modo che solo le voci `allowRead` dalle impostazioni gestite vengono rispettate. Le voci `allowRead` dell'utente, del progetto e locali vengono ignorate. Questo impedisce agli sviluppatori di ampliare l'accesso in lettura oltre i percorsi approvati dall'organizzazione. Per bloccare i domini di rete ai valori gestiti allo stesso modo, imposta [`allowManagedDomainsOnly`](/docs/it/settings#sandbox-settings).

`excludedCommands` non ha un equivalente blocco solo gestito, quindi uno sviluppatore può sempre aggiungere voci che eseguono comandi aggiuntivi al di fuori della sandbox. Mantieni l'elenco gestito ristretto.

<h3 id="custom-proxy-configuration">
  Custom proxy configuration
</h3>

Per le organizzazioni che richiedono una sicurezza di rete avanzata, puoi implementare un proxy personalizzato per:

* Decrittare e ispezionare il traffico HTTPS
* Applicare regole di filtraggio personalizzate
* Registrare tutte le richieste di rete
* Integrarsi con l'infrastruttura di sicurezza esistente

Per puntare Claude Code al tuo proxy, imposta le porte proxy nelle [impostazioni sandbox](/docs/it/settings#sandbox-settings):

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Alcuni comandi falliscono all'interno della sandbox anche se funzionano al di fuori di essa. Le correzioni seguenti coprono i casi più comuni.

* **I comandi falliscono con un errore host-not-allowed**: molti strumenti CLI devono raggiungere host specifici. Concedere l'autorizzazione quando richiesto aggiunge l'host al tuo elenco consentito in modo che lo strumento venga eseguito all'interno della sandbox in futuro.
* **`jest` si blocca o fallisce**: `watchman` è incompatibile con la sandbox. Esegui `jest --no-watchman` invece.
* **I CLI basati su Go falliscono la verifica TLS su macOS**: strumenti come `gh`, `gcloud` e `terraform` potrebbero fallire la verifica TLS sotto Seatbelt. Elenca questi strumenti in `excludedCommands` per eseguirli al di fuori della sandbox. Se stai utilizzando `httpProxyPort` con un proxy MITM e CA personalizzato, imposta [`enableWeakerNetworkIsolation`](/docs/it/settings#sandbox-settings) su `true` invece.
* **`open`, `osascript`, o i flussi di autenticazione basati su browser falliscono con errore `-600` su macOS**: la sandbox blocca gli Apple Events per impostazione predefinita. Imposta [`allowAppleEvents`](/docs/it/settings#sandbox-settings) su `true` nelle impostazioni utente, gestite o CLI per consentirli. Le impostazioni del progetto vengono ignorate per questa chiave. L'abilitazione rimuove l'isolamento dell'esecuzione del codice, poiché i comandi sandboxati possono quindi avviare altre applicazioni non sandboxate senza prompt dell'utente e inviare comandi AppleScript alle applicazioni in esecuzione, soggetti al prompt di consenso dell'automazione macOS (TCC). In alternativa, aggiungi il comando a `excludedCommands` per eseguirlo al di fuori della sandbox.
* **I comandi `docker` falliscono**: `docker` è incompatibile con la sandbox. Aggiungi `docker *` a `excludedCommands` per eseguirlo al di fuori della sandbox.
* **Bubblewrap non riesce ad avviarsi all'interno di un container**: in un container senza privilegi, bubblewrap non può montare un filesystem `/proc` fresco. Imposta [`enableWeakerNestedSandbox`](/docs/it/settings#sandbox-settings) su `true` in modo che la sandbox interna bind-monti il `/proc` esistente del container invece. Utilizza questa impostazione solo quando il container esterno fornisce già il confine di isolamento di cui hai bisogno, poiché espone le informazioni del processo ai comandi sandboxati che un mount `/proc` fresco nasconderebbe.
* **Filtro seccomp su Linux**: il filtro seccomp è richiesto per bloccare i socket di dominio Unix. La scheda Dependencies in `/sandbox` mostra se è disponibile. Se manca, esegui `npm install -g @anthropic-ai/sandbox-runtime` per installare l'helper.
* **`--dangerously-skip-permissions` fallisce come root**: questo flag viene bloccato quando viene eseguito come root o tramite sudo su Linux e macOS, perché l'accesso root combinato con nessun prompt di autorizzazione può modificare qualsiasi file o servizio sul sistema. Il controllo viene saltato automaticamente all'interno di una sandbox riconosciuta. Per eseguire autonomamente in un container, utilizza la configurazione [dev container](/docs/it/devcontainer), che esegue Claude Code come utente non root.

<h2 id="limitations">
  Limitazioni
</h2>

Il sandboxing riduce il rischio ma non è un confine di isolamento completo. Rivedi le limitazioni seguenti prima di fare affidamento su di esso come controllo di sicurezza rigido.

<h3 id="security-limitations">
  Limitazioni di sicurezza
</h3>

* **Filtraggio della rete**: il sandbox limita i domini a cui i processi possono connettersi. Per impostazione predefinita il proxy integrato non termina o ispeziona TLS sul traffico in uscita, quindi i contenuti delle connessioni crittografate non vengono esaminati. L'impostazione sperimentale [`network.tlsTerminate`](/docs/it/settings#sandbox-settings) termina TLS al proxy per la [sostituzione delle credenziali `mask`](#protect-credentials) ma non aggiunge filtraggio dei contenuti. Sei responsabile di assicurarti che solo i domini affidabili siano consentiti nella tua politica.

<Warning>
  Consentire domini ampi come `github.com` può creare percorsi per l'esfiltrazione di dati. Poiché il proxy prende la sua decisione di consentimento dal nome host fornito dal client senza ispezionare TLS, il codice in esecuzione all'interno della sandbox potrebbe potenzialmente utilizzare [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) o tecniche simili per raggiungere host al di fuori dell'allowlist. Se il tuo modello di minaccia richiede garanzie più forti, configura un [proxy personalizzato](#custom-proxy-configuration) che termina TLS e ispeziona il traffico, e installa il suo certificato CA all'interno della sandbox. L'isolamento della rete più consapevole di TLS è un'area di sviluppo attiva.
</Warning>

* **Escalation dei privilegi tramite socket Unix**: la configurazione `allowUnixSockets` può inavvertitamente concedere l'accesso a potenti servizi di sistema che potrebbero portare a bypass della sandbox. Ad esempio, consentire l'accesso a `/var/run/docker.sock` concede effettivamente l'accesso al sistema host attraverso il socket Docker. Considera attentamente qualsiasi socket Unix che consenti attraverso la sandbox.
* **Escalation dei permessi del filesystem**: i permessi di scrittura del filesystem eccessivamente ampi possono abilitare attacchi di escalation dei privilegi. Consentire scritture a directory contenenti eseguibili in `$PATH`, directory di configurazione di sistema o file di configurazione della shell dell'utente come `.bashrc` o `.zshrc` può portare all'esecuzione di codice in diversi contesti di sicurezza quando altri utenti o processi di sistema accedono a questi file.
* **Forza della sandbox Linux**: l'implementazione Linux fornisce un forte isolamento del filesystem e della rete ma include una modalità `enableWeakerNestedSandbox` che le consente di funzionare all'interno di ambienti Docker senza namespace privilegiati, o su host Linux dove gli spazi dei nomi utente senza privilegi sono disabilitati da sysctl. Questa opzione indebolisce considerevolmente la sicurezza e dovrebbe essere utilizzata solo quando l'isolamento aggiuntivo è altrimenti applicato.
* **Apple Events su macOS**: la sandbox macOS blocca gli Apple Events per impostazione predefinita. L'impostazione `allowAppleEvents` rimuove questa restrizione in modo che strumenti come `open` e `osascript` funzionino, ma rimuove l'isolamento dell'esecuzione del codice: i comandi sandboxati possono avviare altre applicazioni senza sandbox senza alcun prompt dell'utente, e possono inviare comandi AppleScript alle applicazioni in esecuzione, soggetti al prompt di consenso per l'automazione macOS per app (TCC). È onorato solo dalle impostazioni utente, gestite o CLI. Le impostazioni del progetto non possono abilitarlo.
* **File di impostazioni protetti**: la sandbox nega automaticamente l'accesso in scrittura ai file `settings.json` di Claude Code a ogni ambito e alla directory delle impostazioni gestite, quindi un comando sandboxato non può modificare la sua stessa politica.

<h3 id="platform-and-tool-compatibility">
  Compatibilità della piattaforma e degli strumenti
</h3>

* **Supporto della piattaforma**: supporta macOS, Linux e WSL2. WSL1 e Windows nativo non sono supportati.
* **Overhead di prestazioni**: minimo, ma alcune operazioni del filesystem potrebbero essere leggermente più lente.
* **Compatibilità degli strumenti**: alcuni strumenti che richiedono modelli di accesso al sistema specifici potrebbero necessitare di regolazioni di configurazione, o potrebbero anche dover essere eseguiti al di fuori della sandbox.

<h3 id="scope">
  Ambito
</h3>

La sandbox isola i sottoprocessi Bash. Altri strumenti operano sotto confini diversi:

* **Strumenti di file integrati**: Read, Edit e Write utilizzano il sistema di autorizzazione direttamente piuttosto che eseguire attraverso la sandbox. Vedi [permissions](/docs/it/permissions).
* **Utilizzo del computer**: quando Claude apre app e controlla lo schermo, viene eseguito sul tuo desktop effettivo piuttosto che in un ambiente isolato. I prompt di autorizzazione per app gating ogni applicazione. Vedi [computer use nella CLI](/docs/it/computer-use) o [computer use in Desktop](/docs/it/desktop#let-claude-use-your-computer).
* **Variabili di ambiente**: i comandi Bash sandboxati ereditano l'ambiente del processo padre per impostazione predefinita, incluse le credenziali impostate lì. Usa [`sandbox.credentials`](#protect-credentials) per annullare o mascherare variabili specifiche per i comandi sandboxati, o imposta [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/it/env-vars) per rimuovere le credenziali di Anthropic e del provider cloud dai sottoprocessi.
* **Subagenti**: i [subagenti](/docs/it/sub-agents) vengono eseguiti nello stesso processo della sessione padre e utilizzano la stessa configurazione sandbox. I comandi Bash all'interno di un subagente vengono sandboxati quando il sandboxing è abilitato nella sessione padre.

<Warning>
  Il sandboxing efficace richiede sia l'isolamento del filesystem che della rete. Senza isolamento della rete, un agente compromesso potrebbe esfiltare file sensibili come chiavi SSH. Senza isolamento del filesystem, un agente compromesso potrebbe backdoor le risorse di sistema per ottenere accesso alla rete. Quando ampli i predefiniti, verifica che un percorso `allowWrite`, una voce `allowedDomains` ampia o un'eccezione `excludedCommands` non annulli una restrizione dall'altro lato.
</Warning>

<h2 id="see-also">
  See also
</h2>

* [Sandbox environments](/docs/it/sandbox-environments): confronta la sandbox integrata con dev container, container e VM
* [Security](/docs/it/security): funzionalità di sicurezza complete e best practice
* [Permissions](/docs/it/permissions): configurazione delle autorizzazioni e controllo dell'accesso
* [Settings](/docs/it/settings): riferimento di configurazione completo
* [CLI reference](/docs/it/cli-reference): opzioni della riga di comando
