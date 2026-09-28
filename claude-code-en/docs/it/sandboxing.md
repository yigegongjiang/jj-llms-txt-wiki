> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura lo strumento Bash in sandbox

> Scopri come lo strumento Bash in sandbox di Claude Code fornisce isolamento del filesystem e della rete per un'esecuzione dell'agente più sicura e autonoma.

La sandbox Bash consente a Claude di eseguire la maggior parte dei comandi shell senza fermarsi per chiedere autorizzazione. Invece di approvare ogni comando, definisci quali file e domini di rete i comandi possono toccare, e il sistema operativo applica quel confine per ogni comando Bash, PowerShell o Monitor e i suoi processi figlio.

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

    Questo apre il pannello sandbox con tre schede, più una scheda Dependencies su Linux quando il filtro seccomp facoltativo è mancante:

    * **Mode**: scegli come i comandi sandboxati vengono approvati, trattato nel passaggio successivo
    * **Overrides**: scegli se i comandi che falliscono sotto la sandbox possono ricadere nell'esecuzione non sandboxata. Questa è l'impostazione [`allowUnsandboxedCommands`](/docs/it/settings-reference#sandbox-allowunsandboxedcommands)
    * **Config**: visualizza le impostazioni sandbox risolte

    Se il pannello mostra solo una scheda Dependencies, manca un pacchetto richiesto. Installalo come descritto in [Set up Linux and WSL2](#set-up-linux-and-wsl2), riavvia Claude Code ed esegui `/sandbox` di nuovo.
  </Step>

  <Step title="Scegli una modalità">
    Nella scheda Mode, seleziona auto-allow o autorizzazioni regolari. Auto-allow esegue i comandi sandboxati senza richiedere, e le autorizzazioni regolari mantengono i prompt di autorizzazione regolari anche quando i comandi sono sandboxati. Vedi [Sandbox modes](#sandbox-modes) per quali comandi richiedono comunque prompt in modalità auto-allow.
  </Step>

  <Step title="Esegui un comando Bash">
    Chiedi a Claude di eseguire un comando, come una build o una suite di test. Per impostazione predefinita, i comandi all'interno della sandbox possono scrivere nella directory di lavoro, nella directory temporanea della sessione e in qualsiasi [directory che hai aggiunto](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`.

    La prima volta che un comando ha bisogno di un nuovo dominio di rete, Claude Code richiede l'approvazione; in [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), Claude invece nomina gli host di cui un comando ha bisogno [sul comando stesso](#per-command-allowed-domains-in-auto-mode) affinché il classificatore li esamini insieme ad esso.

    I comandi che non possono essere eseguiti sandboxati ricadono nel flusso di autorizzazione regolare. Claude Code intitola il loro prompt di autorizzazione "Bash command (unsandboxed)" invece di "Bash command", così puoi dire quali comandi sono stati eseguiti al di fuori della sandbox. Per ampliare o restringere ciò che la sandbox consente, vedi [Configure sandboxing](#configure-sandboxing).

    Se i comandi sandboxati falliscono con `Operation not permitted` all'interno di un contenitore, vedi la voce Bubblewrap sotto [Troubleshooting](#troubleshooting).
  </Step>
</Steps>

Quando selezioni una modalità nel pannello, Claude Code la salva nelle impostazioni locali del tuo progetto in `.claude/settings.local.json`, che si applicano al progetto corrente. Claude Code aggiunge quel file al tuo gitignore globale quando salva un'impostazione lì. Per abilitare la sandbox in tutti i tuoi progetti, imposta [`sandbox.enabled`](/docs/it/settings-reference#sandbox-enabled) su `true` nelle impostazioni utente in `~/.claude/settings.json`. Per applicare il sandboxing per ogni sviluppatore in un'organizzazione, utilizza [impostazioni gestite](#enforce-sandboxing-with-managed-settings).

Per modificare la sandbox per una sessione senza scrivere in un file di impostazioni, avvia Claude Code con [`--settings`](/docs/it/settings#change-a-setting-for-one-session). Ad esempio, questo comando avvia una sessione sandboxata in cui Claude non può riprovare un comando bloccato al di fuori della sandbox:

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  Per impostazione predefinita, se la sandbox non può avviarsi perché mancano dipendenze o la piattaforma non è supportata, Claude Code mostra un avviso ed esegue i comandi senza sandboxing. Per rendere questo un errore grave, imposta [`sandbox.failIfUnavailable`](/docs/it/settings-reference#sandbox-failifunavailable) su `true`. Questo è destinato a distribuzioni gestite che richiedono il sandboxing come gate di sicurezza.
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

Quando una dipendenza è mancante, la scheda Dependencies in `/sandbox` elenca quale tra `ripgrep`, `bubblewrap`, `socat` e il filtro seccomp la tua piattaforma manca. Se non vedi la scheda dopo l'installazione e il riavvio di Claude Code, tutte le dipendenze sono presenti.

Ripgrep è incluso nel binario nativo di Claude Code. Il filtro seccomp è facoltativo e aggiunge il blocco del socket di dominio Unix. Installalo con `npm install -g @anthropic-ai/sandbox-runtime` se è mancante.

Quando una dipendenza richiesta è mancante, la scheda Dependencies è l'unica scheda mostrata fino a quando non la installi. Quando solo il filtro seccomp facoltativo è mancante, la scheda Dependencies appare insieme alle altre schede. Il controllo delle dipendenze viene eseguito all'avvio, quindi riavvia Claude Code dopo l'installazione dei pacchetti affinché `/sandbox` li rilevi.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 e versioni successive: consenti a bubblewrap di creare spazi dei nomi utente">
    Su Ubuntu 24.04 e versioni successive, la politica AppArmor predefinita impedisce a bubblewrap di creare gli spazi dei nomi utente di cui ha bisogno per l'isolamento.

    Per verificare se il tuo ambiente applica questa restrizione, incluso all'interno di WSL2, esegui `sysctl kernel.apparmor_restrict_unprivileged_userns`. Se il comando restituisce `0`, salta questo passaggio. Se stampa un errore `No such file or directory`, la chiave non esiste e puoi saltare questo passaggio. Se restituisce `1`, aggiungi un profilo AppArmor che conceda a `bwrap` questa capacità:

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

    Su WSL2, WSL passa il lancio di un binario Windows come `cmd.exe`, `powershell.exe` o qualsiasi cosa sotto `/mnt/c/` all'host Windows su un socket Unix, quindi se un comando sandboxato può lanciarne uno segue le [impostazioni Unix-socket](/docs/it/settings-reference#sandbox-network-allowunixsockets) della sandbox: il filtro seccomp facoltativo deve essere installato per bloccare il socket in primo luogo. Per consentire questi lanci, imposta `allowAllUnixSockets`; per tenerli completamente fuori dalla sandbox, aggiungi il comando a [`excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands).
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Sandbox modes
</h3>

Claude Code offre due modalità sandbox. In entrambe, la sandbox applica le stesse restrizioni di filesystem e rete; la differenza è solo se i comandi sandboxati sono auto-approvati o richiedono autorizzazione esplicita.

<h4 id="auto-allow-mode">
  Auto-allow mode
</h4>

Quando un comando può essere sandboxato, Claude Code lo esegue all'interno della sandbox e lo approva automaticamente, senza chiedere il tuo permesso. I comandi che non possono essere sandboxati, come quelli che necessitano di accesso alla rete a host non consentiti, ricadono nel flusso di autorizzazione regolare, dove Claude Code controlla le tue [regole di autorizzazione](/docs/it/permissions) e blocca qualsiasi comando che quelle regole non consentono già, con un prompt in modalità Manual.

Anche in modalità auto-allow, si applicano i seguenti:

* Le [regole di negazione](/docs/it/permissions) esplicite sono sempre rispettate
* I comandi `rm` o `rmdir` che puntano a un [percorso critico](/docs/it/permission-modes#critical-paths) passano comunque attraverso il flusso di autorizzazione regolare
* Le [regole ask](/docs/it/permissions) con ambito di contenuto come `Bash(git push *)` forzano comunque un prompt anche per i comandi sandboxati
* Una regola ask `Bash` semplice, o la forma equivalente `Bash(*)`, viene saltata per i comandi che vengono eseguiti sandboxati; si applica comunque ai comandi che ricadono nel flusso di autorizzazione regolare. In [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode), la regola non viene saltata: richiede per i comandi sandboxati anche, inclusi quelli di sola lettura. Prima della v2.1.212, il salto si applicava anche in plan mode

<Info>
  La modalità auto-allow funziona indipendentemente dall'impostazione della modalità di autorizzazione, con tre eccezioni: [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode), un comando in modalità auto che porta [domini consentiti per comando](#per-command-allowed-domains-in-auto-mode), e [revisione del classificatore lato server](/docs/it/permission-modes#how-the-classifier-evaluates-actions) dei comandi sandboxati in modalità auto. Anche se non sei in modalità "accetta modifiche", i comandi Bash sandboxati vengono eseguiti automaticamente quando auto-allow è abilitato. Ciò significa che i comandi Bash che modificano file entro i confini della sandbox vengono eseguiti senza richiedere, anche in modalità Manual, dove gli strumenti di modifica dei file richiederebbero.

  In plan mode, auto-allow non amplia le approvazioni; vedi [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) per come Claude Code blocca i comandi mentre pianifichi. Prima della v2.1.212, auto-allow eseguiva i comandi sandboxati senza un prompt in plan mode anche.
</Info>

<h4 id="regular-permissions-mode">
  Regular permissions mode
</h4>

Tutti i comandi Bash passano attraverso il flusso di autorizzazione regolare, anche quando sandboxati. Questo fornisce più controllo ma richiede più approvazioni.

<h4 id="the-unsandboxed-retry-escape-hatch">
  The unsandboxed retry escape hatch
</h4>

Alcuni comandi non possono essere eseguiti all'interno della sandbox, come strumenti incompatibili con essa o che necessitano di un host che non hai consentito. Claude Code segnala le violazioni della sandbox nel risultato del comando bloccato, nominando il percorso o l'host che la sandbox ha negato, così Claude vede cosa la sandbox ha bloccato. Piuttosto che fallire il compito o richiedere di disattivare il sandboxing, Claude Code include un escape hatch: Claude analizza la violazione e potrebbe riprovare il comando con il parametro `dangerouslyDisableSandbox`.

Il comando riprovato viene eseguito al di fuori della sandbox, quindi passa attraverso il flusso di autorizzazione regolare. In modalità Manual ottieni un prompt di conferma. In [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), il classificatore valuta il comando sottostante. Mentre [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) è attivo, una riprovazione che necessita di approvazione per essere eseguita al di fuori della sandbox ti richiede invece. Per essere richiesto su ogni riprovazione non sandboxata anche in modalità auto, aggiungi una [regola ask](/docs/it/permissions#match-by-input-parameter) per `Bash(dangerouslyDisableSandbox:true)`.

Puoi disabilitare questo escape hatch impostando `"allowUnsandboxedCommands": false` nelle tue [impostazioni sandbox](/docs/it/settings-reference#sandbox-settings). Con l'escape hatch disabilitato, Claude Code ignora il parametro `dangerouslyDisableSandbox`, e ogni comando che Claude esegue deve essere eseguito sandboxato a meno che non lo abbia elencato in `excludedCommands`. La scheda **Overrides** di `/sandbox` mostra questa impostazione come **Strict sandbox mode**.

La modalità strict sandbox si applica ai comandi che Claude esegue. I comandi che digiti tu stesso al prompt [shell-mode](/docs/it/interactive-mode#shell-mode-with-prefix) con il prefisso `!` vengono eseguiti al di fuori della sandbox a meno che la sessione non sia una di queste:

* **Una [sessione in background](/docs/it/agent-view)**: la modalità strict sandbox copre anche i comandi shell-mode
* **Una sessione Linux con [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/it/env-vars#variables) impostato**: ogni comando viene eseguito sandboxato, inclusi i comandi shell-mode

Prima della v2.1.260, la modalità strict sandbox sandboxava i comandi shell-mode in ogni sessione.

<h4 id="temporary-directories">
  Temporary directories
</h4>

La directory temporanea della sessione è scrivibile all'interno della sandbox per impostazione predefinita, insieme alla directory di lavoro. A meno che tu non [disabiliti l'isolamento del filesystem](#disable-filesystem-isolation), Claude Code imposta `$TMPDIR` su questa directory per i comandi sandboxati, così gli strumenti che scrivono file temporanei funzionano senza configurazione aggiuntiva.

I comandi non sandboxati ereditano il tuo `$TMPDIR` della shell quando è impostato, quindi mentre l'isolamento del filesystem è attivo, i comandi sandboxati e non sandboxati risolvono `$TMPDIR` in directory diverse. Se la tua shell lascia `$TMPDIR` non impostato o vuoto, un comando non sandboxato che fa riferimento a `$TMPDIR` riceve il tuo override [`CLAUDE_CODE_TMPDIR`](/docs/it/env-vars), o la directory temporanea del sistema operativo quando non ne hai impostato uno o l'override è un percorso lungo, così la variabile non si espande a una stringa vuota. Per passare file temporanei tra i due, scrivili nella directory di lavoro invece.

<h2 id="configure-sandboxing">
  Configura il sandboxing
</h2>

Personalizza il comportamento della sandbox attraverso il tuo file `settings.json`. Vedi [Settings](/docs/it/settings-reference#sandbox-settings) per il riferimento di configurazione completo.

Per impostazione predefinita, i comandi in sandbox possono scrivere nella directory di lavoro corrente, nella directory temporanea della sessione e in qualsiasi [directory che hai aggiunto](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`. Se i comandi dei sottoprocessi come `kubectl`, `terraform` o `npm` devono scrivere al di fuori di quelle directory, usa `sandbox.filesystem.allowWrite` per concedere l'accesso a percorsi specifici:

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

Questi percorsi sono applicati a livello del sistema operativo, quindi tutti i comandi in esecuzione all'interno della sandbox, inclusi i loro processi figlio, li rispettano. Questo è l'approccio consigliato quando uno strumento ha bisogno dell'accesso in scrittura a una posizione specifica, piuttosto che escludere completamente lo strumento dalla sandbox con `excludedCommands`.

Quando definisci lo stesso array del filesystem in più [ambiti di impostazioni](/docs/it/settings#settings-precedence), Claude Code li unisce, combinando i percorsi da ogni ambito piuttosto che sostituire l'array di un ambito con quello di un altro.

Se escludi un'origine con [`--setting-sources`](/docs/it/cli-reference) sulla CLI o [`settingSources`](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) in Agent SDK, Claude Code ignora le sue voci `sandbox.filesystem`, le sue regole di permesso `Edit` e le sue regole di negazione `Read` quando costruisce la configurazione della sandbox. Richiede Claude Code v2.1.246 o successivo.

Quando modifichi questi elenchi del filesystem durante una sessione, Claude Code [applica la modifica alla sessione in esecuzione](/docs/it/settings#when-edits-take-effect), quindi il prossimo comando in sandbox viene eseguito con i nuovi percorsi.

I prefissi dei percorsi controllano come vengono risolti i percorsi:

| Prefisso               | Significato                                                                                                    | Esempio                                                                     |
| :--------------------- | :------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `/`                    | Percorso assoluto dalla radice del filesystem                                                                  | `/tmp/build` rimane `/tmp/build`                                            |
| `~/`                   | Relativo alla directory home                                                                                   | `~/.kube` diventa `$HOME/.kube`                                             |
| `./` o nessun prefisso | Relativo alla radice del progetto per le impostazioni del progetto, o a `~/.claude` per le impostazioni utente | `./output` in `.claude/settings.json` si risolve in `<project-root>/output` |

Questa sintassi differisce dalle [regole di permesso Read e Edit](/docs/it/permissions#read-and-edit), che usano `//path` per assoluto e `/path` per relativo al progetto. I percorsi del filesystem della sandbox usano convenzioni standard: `/tmp/build` è assoluto. Per come Claude Code tratta una barra finale o un wildcard in questi percorsi, vedi [Prefissi dei percorsi della sandbox](/docs/it/settings-reference#sandbox-path-prefixes).

Puoi anche negare l'accesso in scrittura o lettura usando `sandbox.filesystem.denyWrite` e `sandbox.filesystem.denyRead`, e ri-consentire percorsi specifici all'interno di una regione negata usando `sandbox.filesystem.allowRead`. Quando le regole di lettura si sovrappongono, il percorso più specifico vince:

| Regole di esempio                                      | Risultato                                                                                                                                                                                                                                    |
| :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` con `"allowRead": ["~/projects"]` | `~/projects` è leggibile e il resto della directory home rimane bloccato. L'allow più ristretto riapre quella parte della regione negata                                                                                                     |
| `"allowRead": ["~/"]` con `"denyRead": ["~/.env"]`     | `~/.env` rimane bloccato e il resto della directory home è leggibile. Il deny si mantiene all'interno di un allow più ampio, quindi un allow ampio non può silenziosamente ri-esporre un segreto                                             |
| `"allowRead": ["~/"]` con `"denyRead": ["~/**/.env"]`  | Ogni `.env` sotto la directory home rimane bloccato e il resto è leggibile. Un [wildcard deny](/docs/it/settings-reference#sandbox-path-prefixes) si mantiene all'interno di un allow più ampio nello stesso modo in cui lo fa un percorso esatto |

L'esempio seguente blocca la lettura dall'intera directory home mantenendo comunque la lettura dal progetto corrente. Posizionalo nel `.claude/settings.json` del tuo progetto, perché il percorso relativo `.` si risolve alla radice del progetto solo quando la configurazione si trova nelle impostazioni del progetto:

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

Se avessi posizionato la stessa configurazione in `~/.claude/settings.json`, `.` si risolverebbe in `~/.claude` invece, e i file del progetto rimarrebbero bloccati dalla regola `denyRead`.

Per negare ai comandi in sandbox l'accesso in lettura alle directory home e ai volumi montati mantenendo le directory di lavoro leggibili, imposta [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) invece di scrivere regole di percorso.

<h3 id="disable-filesystem-isolation">
  Disabilita l'isolamento del filesystem
</h3>

Imposta `sandbox.filesystem.disabled` su `true` per saltare l'isolamento del filesystem mantenendo l'isolamento della rete. L'esempio seguente disattiva l'isolamento del filesystem mantenendo una lista di permessi di domini di rete:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

La sandbox ha due livelli indipendenti: [l'isolamento del filesystem](#filesystem-isolation) controlla quali percorsi i comandi in sandbox possono leggere e scrivere, e [l'isolamento della rete](#network-isolation) controlla quali domini possono raggiungere. Con il livello del filesystem disattivato, i comandi in sandbox ottengono accesso in lettura e scrittura senza restrizioni al filesystem host, mentre il loro egresso di rete rimane confinato ai tuoi domini consentiti. Disattiva il livello quando esegui il sandbox per controllare dove i comandi si connettono piuttosto che cosa scrivono.

L'impostazione è disattivata per impostazione predefinita e si applica sulle piattaforme dove la sandbox viene eseguita: macOS, Linux e WSL2. Richiede Claude Code v2.1.216 o successivo.

<Warning>
  Con l'isolamento del filesystem disattivato e i comandi auto-consentiti, un comando in sandbox può scrivere file che i comandi successivi eseguono o leggono, come file di avvio della shell, eseguibili su `$PATH` o `~/.claude/settings.json`, e usarli per ampliare il proprio accesso alla prossima esecuzione. Imposta `filesystem.disabled` su `true` solo per carichi di lavoro di cui ti fidi che non escalino il proprio accesso. Bloccare i domini di rete con [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) riduce il rischio ma non lo elimina, poiché quel blocco si applica solo ai comandi in esecuzione all'interno della sandbox.
</Warning>

<h4 id="which-settings-can-disable-it">
  Quali impostazioni possono disabilitarlo
</h4>

Poiché disattivare l'isolamento del filesystem amplia ciò che i comandi in sandbox possono fare, Claude Code onora `filesystem.disabled` solo da queste fonti di impostazioni:

* Le impostazioni utente, le impostazioni gestite e il flag CLI `--settings` possono impostarlo. Le impostazioni del progetto in `.claude/settings.json` e `.claude/settings.local.json` non possono, quindi un progetto estratto non può disattivare l'isolamento del filesystem.
* Quando le impostazioni gestite configurano `sandbox.filesystem` in generale, o elencano qualsiasi voce `sandbox.credentials.files` con `"mode": "deny"`, solo le impostazioni gestite possono impostare la chiave. Questo mantiene in vigore le restrizioni del filesystem distribuite dall'amministratore; per rilassare tale distribuzione, imposta `"disabled": true` nelle impostazioni gestite.
* Quando [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/it/env-vars) è impostato, Claude Code ignora `filesystem.disabled` da ogni fonte, incluse le impostazioni gestite, e mantiene l'isolamento del filesystem attivo.

Se una voce gestita `credentials.files` fissa `filesystem.disabled`, bloccando la chiave alle impostazioni gestite in modo che gli sviluppatori non possano disattivare l'isolamento del filesystem, dipende dalla `mode` della voce e da cosa accade alla voce quando la sandbox si avvia:

| Voce gestita                                                                                                       | Fissa `filesystem.disabled`        | Cosa protegge il file quando l'isolamento è disattivato                                                                                               |
| ------------------------------------------------------------------------------------------------------------------ | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"mode": "deny"`                                                                                                   | Sì                                 | Niente: il blocco di lettura fa parte del livello del filesystem                                                                                      |
| `"mode": "mask"`, applicato come maschera                                                                          | No                                 | La mascheratura stessa: la [copia sentinella e il proxy](#mask-credential-files) su Linux e WSL2, le proprie regole di lettura della sandbox su macOS |
| `"mode": "mask"`, [fallback a `deny`](#mask-credential-files) al setup                                             | No                                 | Niente, come `deny`. Elenca un percorso che non può essere mascherato, come una directory, come voce esplicita `deny`, che fissa la chiave            |
| `"mode": "mask"`, [degradato a `deny` dalla validazione](/docs/it/managed-settings#invalid-entries-in-managed-settings) | Sì, come una voce esplicita `deny` | Niente, come `deny`                                                                                                                                   |

Un fallback accade quando la sandbox si avvia, dopo che Claude Code ha già letto le impostazioni su cui viene eseguito il controllo del pin, quindi una voce con fallback non fissa mai. La validazione riscrive una voce non valida a `deny` mentre le impostazioni si caricano, quindi una voce degradata fissa come una che hai scritto come `deny`.

<h4 id="what-changes-when-filesystem-isolation-is-off">
  Cosa cambia quando l'isolamento del filesystem è disattivato
</h4>

Impostare `filesystem.disabled` solleva le protezioni che il livello del filesystem stesso applica. Le protezioni che altri livelli applicano continuano a funzionare:

| Protezione                                                                                    | Con l'isolamento del filesystem disattivato                                                                                                                                   |
| --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `filesystem.denyRead` e [`credentials.files`](#protect-credentials) blocchi di lettura `deny` | Non applicati. Il livello del filesystem applica entrambi                                                                                                                     |
| `credentials.envVars` voci `deny` e `mask`                                                    | Applicati. Lo scrubbing delle variabili di ambiente è indipendente dal livello del filesystem                                                                                 |
| [`credentials.files` voci `mask`](#mask-credential-files) applicate come maschere             | Applicati: la mascheratura è indipendente dal livello del filesystem. Una voce che ha [fallback a `deny`](#mask-credential-files) non è applicata, come qualsiasi voce `deny` |

Due altre cose cambiano:

* I comandi in sandbox ereditano il `$TMPDIR` della tua shell invece della directory temporanea della sessione, perché ogni directory temporanea è scrivibile e Claude Code non reindirizza più i comandi a quella della sessione.

  Su Linux la variabile è spesso non impostata nella shell padre. La guida dello strumento Bash dice a Claude di creare directory di lavoro con `mktemp -d` invece di fare affidamento su `$TMPDIR`.
* [`autoAllowBashIfSandboxed`](/docs/it/settings-reference#sandbox-autoallowbashifsandboxed) continua a impostare il valore predefinito su `true`, quindi i comandi in sandbox continuano a essere eseguiti senza prompt. Impostalo su `false` per richiedere prompt per i comandi in sandbox.

<h3 id="protect-credentials">
  Proteggi le credenziali
</h3>

L'impostazione `sandbox.credentials` dichiara file di credenziali e variabili di ambiente da proteggere dai comandi in sandbox. Ogni voce nomina un percorso di file o una variabile di ambiente e una `mode`. Il blocco dedicato `credentials` mantiene le regole delle credenziali raggruppate insieme e separate dalle regole generali del filesystem.

Per le voci con `"mode": "deny"`, i percorsi dei file vengono negati per le letture all'interno della sandbox, la stessa restrizione che `filesystem.denyRead` applica, e le variabili di ambiente vengono non impostate prima di ogni comando in sandbox. La protezione del file fa parte del livello del filesystem, quindi non si applica se [disabiliti l'isolamento del filesystem](#disable-filesystem-isolation); la protezione della variabile di ambiente continua comunque.

L'esempio seguente blocca le letture del file delle credenziali AWS e della directory SSH e rimuove `GITHUB_TOKEN` e `NPM_TOKEN` dall'ambiente dei comandi in sandbox:

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

Le voci delle variabili di ambiente e le voci dei file accettano anche `"mode": "mask"`, descritte sotto [Maschera le credenziali](#mask-credentials).

I percorsi dei file seguono le stesse [regole di prefisso](/docs/it/settings-reference#sandbox-path-prefixes) delle impostazioni `sandbox.filesystem.*`.

Claude Code unisce le voci `deny` da ogni [ambito di impostazioni](/docs/it/settings#settings-precedence) che la sessione carica. Una voce `deny` restringe solo l'accesso, quindi qualsiasi ambito può aggiungerne una, ma nessun ambito può rimuoverne una che un altro ambito ha aggiunto.

Quando [escludi una fonte di impostazioni](#configure-sandboxing):

* **Impostazioni del progetto o locali**: Claude Code non applica nessuna delle loro voci `credentials`. Richiede Claude Code v2.1.246 o successivo.
* **Impostazioni utente**: Claude Code applica comunque le voci `deny` in `~/.claude/settings.json` e mantiene le sue [voci `mask` del file](#mask-credential-files) come restrizioni, ma elimina le sue [voci `mask` delle variabili di ambiente](#mask-environment-variables).

Non esiste un elenco di negazione delle credenziali integrato, quindi solo i file e le variabili che elenchi sono limitati.

`sandbox.credentials` influisce solo sui comandi Bash in sandbox. Per rimuovere le credenziali da tutti i sottoprocessi indipendentemente dal sandboxing, imposta [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/it/env-vars).

<h3 id="mask-credentials">
  Maschera le credenziali
</h3>

La mascheratura va oltre una voce `deny` sotto [Proteggi le credenziali](#protect-credentials). Invece di bloccare una credenziale, Claude Code mostra ai comandi in sandbox un segnaposto, la sentinella, e il [proxy della sandbox](#network-isolation) scambia il valore reale sulle richieste in uscita agli host che consenti. Per i file, la sostituzione è il comportamento di Linux e WSL2; [macOS blocca il file invece](#mask-credential-files).

<h4 id="mask-environment-variables">
  Maschera le variabili di ambiente
</h4>

`"mode": "mask"` protegge una credenziale mantenendo gli strumenti che si autenticano con essa funzionanti. `deny` rimuove completamente la variabile, il che rompe anche gli strumenti che ne hanno bisogno, come `gh` o `npm`. Richiede Claude Code v2.1.199 o successivo.

Con `mask`, il comando in sandbox vede un valore sentinella per sessione invece di quello reale. Ogni voce `mask` può elencare `injectHosts`, gli host a cui il valore reale è consentito raggiungere. Quando una richiesta esce dalla sandbox per uno di loro, il [proxy della sandbox](#network-isolation) sostituisce la sentinella con il valore reale. Il comando e tutto ciò che registra non contengono mai la credenziale reale, ma le sue richieste si autenticano comunque.

Il proxy sostituisce la credenziale all'interno dei contenuti della richiesta, quindi deve vederli. Imposta [`network.tlsTerminate`](/docs/it/settings-reference#sandbox-network-tlsterminate) in modo che il proxy termini TLS stesso.

Senza di esso, la mascheratura fallisce senza esporre nulla: il comando continua a vedere solo la sentinella, ma la sentinella raggiunge il server invariata e l'autenticazione fallisce. Claude Code segnala questa configurazione errata all'avvio.

La sostituzione copre intestazioni e corpi di richiesta. Le richieste che si autenticano con una firma derivata dalla credenziale, piuttosto che dalla credenziale stessa, hanno bisogno di una nuova firma al proxy; [Firma di nuovo le richieste AWS](#re-sign-aws-requests) copre come funziona per AWS.

Il proxy inietta solo sulle connessioni che la [lista di permessi del dominio](#network-isolation) ammette, quindi ogni destinazione `injectHosts` deve anche essere raggiungibile attraverso `network.allowedDomains`.

L'esempio seguente maschera due token. `GH_TOKEN` viene sostituito solo sulle richieste a `api.github.com`, mentre `NPM_TOKEN` non ha `injectHosts` e viene sostituito sulle richieste a ogni host in `network.allowedDomains`.

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

<span id="ipv6-destinations-in-injecthosts" />Scrivi una destinazione IPv6 diversamente nei due elenchi, perché ogni elenco ha il suo matcher:

* **`network.allowedDomains`**: la [forma tra parentesi che gli elenchi di domini usano](#ipv6-addresses-in-domain-lists), come `"[::1]"`. Il proxy controlla questo elenco per ammettere la connessione.
* **`injectHosts`**: l'indirizzo nudo nella sua forma canonica compressa, come `"::1"` o `"2001:db8::1"`. Il proxy confronta ogni voce con l'indirizzo di destinazione nudo della connessione, ignorando le porte, quindi una forma tra parentesi, con ID zona o diversamente compressa non corrisponde mai e il proxy non inietta mai la credenziale lì.

`claude doctor` contrassegna le voci `injectHosts` che non possono mai corrispondere con l'avviso `Sandbox credential injectHosts entries can never match their destination`. Questo controllo richiede Claude Code v2.1.229 o successivo.

A differenza di `deny`, la mascheratura autorizza il proxy a inviare la tua credenziale reale agli host elencati, quindi Claude Code la onora solo dalle impostazioni che tu o il tuo amministratore controllate: impostazioni utente, impostazioni gestite e il flag CLI `--settings`. Claude Code ignora le voci `mask` nel `.claude/settings.json` o `.claude/settings.local.json` di un repository. In quei file ignora anche `network.tlsTerminate` e [`credentials.allowPlaintextInject`](/docs/it/settings-reference#sandbox-credentials-allowplaintextinject), l'impostazione che consente al proxy di iniettare credenziali in richieste non crittografate. Se [escludi le impostazioni utente](#configure-sandboxing), Claude Code elimina anche le voci `mask` delle variabili di ambiente in `~/.claude/settings.json`.

Quando il tuo amministratore fornisce voci `mask`, `network.tlsTerminate` o `credentials.allowPlaintextInject` attraverso le impostazioni gestite dal server, contano come [impostazioni che richiedono approvazione](/docs/it/server-managed-settings#security-approval-dialogs).

Quando la stessa variabile è elencata con `deny` in qualsiasi ambito, `deny` ha la precedenza.

La mascheratura sostituisce l'intero valore della variabile per impostazione predefinita, il che si adatta a un token nudo. I campi di voce opzionali, che richiedono Claude Code v2.1.224 o successivo, gestiscono valori con struttura:

* `extract`: un'espressione regolare che Claude Code applica su tutto il valore, sostituendo solo il testo catturato dal gruppo 1 di ogni corrispondenza, quindi uno strumento che analizza il valore, come una stringa di connessione `DATABASE_URL`, continua a funzionare all'interno della sandbox. Il pattern deve contenere almeno un gruppo di cattura.
* `onExtractNoMatch` controlla cosa accade quando il pattern non corrisponde a nulla:
  * `warn`, il valore predefinito, avverte e passa la variabile attraverso senza mascheratura
  * `deny` non imposta la variabile all'interno della sandbox
  * `error` interrompe il setup della sandbox finché non fissi la configurazione
* `decode: "jwt"`: per una variabile che contiene un JSON Web Token (JWT). Claude Code verifica che il valore sia un JWT e lo sostituisce con un token falso strutturalmente valido, quindi il codice all'interno della sandbox che decodifica il token continua a funzionare. Aggiungi `maskClaims` per elencare i claim del payload di livello superiore da mascherare individualmente invece di sostituire l'intero token; gli altri claim rimangono leggibili. Quando il valore non si verifica come JWT, o nessun claim elencato corrisponde, Claude Code passa la variabile attraverso senza mascheratura con un avviso. `decode` non può essere combinato con `extract`.

Vedi le [righe `credentials.envVars[]` nel riferimento delle impostazioni](/docs/it/settings-reference#sandbox-settings) per l'elenco completo dei campi.

<h4 id="re-sign-aws-requests">
  Firma di nuovo le richieste AWS
</h4>

Le richieste AWS portano firme SigV4 sui contenuti della richiesta, quindi maschera `AWS_ACCESS_KEY_ID` e `AWS_SECRET_ACCESS_KEY` insieme. Il proxy rileva una richiesta SigV4 dalla sentinella della chiave di accesso e la firma di nuovo dopo aver sostituito i valori reali. Mascherare solo il segreto lascia le richieste firmate con il segnaposto, che il proxy non può rilevare, quindi falliscono su AWS; Claude Code avverte di questo caso all'avvio, ma non quando solo l'ID della chiave di accesso è mascherato. Una richiesta rilevata che il proxy non può firmare di nuovo, come una senza il suo header `x-amz-date`, fallisce con un errore del proxy invece di raggiungere il server con una firma rotta.

Claude Code collega le variabili convenzionali `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN` in una credenziale automaticamente quando mascheri i loro interi valori. Se la tua credenziale AWS si trova in variabili con altri nomi, raggruppale tu stesso con [`credentials.awsPairs`](/docs/it/settings-reference#sandbox-credentials-awspairs), che richiede Claude Code v2.1.224 o successivo. Questo esempio aggiunge l'accoppiamento a una configurazione che già maschera `MY_KEY_ID`, `MY_SECRET_KEY` e `MY_SESSION_TOKEN` a valore intero, come nella [configurazione di mascheratura sopra](#mask-environment-variables):

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Ogni voce segue queste regole:

* `accessKeyIdVar` e `secretAccessKeyVar` nominano le voci `envVars` mascherate che contengono l'ID della chiave di accesso e la chiave segreta. L'opzionale `sessionTokenVar` nomina la voce che contiene il token di sessione per le credenziali temporanee; quando impostato, il proxy invia il token reale come `x-amz-security-token` sulle richieste firmate di nuovo.
* Ogni variabile nominata deve essere una voce `mask` che maschera il suo intero valore, senza `extract` o `decode`.
* Il proxy firma di nuovo le richieste sugli host elencati nella voce `injectHosts` dell'ID della chiave di accesso.
* Nominare una qualsiasi delle variabili convenzionali in una coppia sostituisce l'accoppiamento automatico.

Come le voci `mask`, `awsPairs` è onorato solo dalle impostazioni utente, dalle impostazioni gestite e dal flag CLI `--settings`.

Tre forme di richiesta AWS portano firme che il proxy non può ricalcolare. Quando tale richiesta è firmata con il segnaposto di una coppia mascherata, il proxy la fallisce piuttosto che inoltrarla con una firma rotta; le richieste firmate con credenziali non mascherate non sono mai interessate. L'impostazione [`credentials.sigv4`](/docs/it/settings-reference#sandbox-credentials-sigv4), che richiede Claude Code v2.1.224 o successivo, rilassa questo per forma: impostare la chiave di una forma su `passthrough` inoltra la richiesta con la sua firma derivata dal segnaposto, quindi lo strumento che chiama riceve la propria risposta di rifiuto di AWS invece di un errore del proxy. Come `awsPairs`, `sigv4` è onorato solo dalle impostazioni utente, dalle impostazioni gestite e dal flag CLI `--settings`.

| Forma di richiesta                   | Chiave `sigv4` | Perché il proxy non può firmarla di nuovo                                                                               |
| :----------------------------------- | :------------- | :---------------------------------------------------------------------------------------------------------------------- |
| caricamenti di streaming aws-chunked | `streaming`    | Le firme per chunk si concatenano dalla firma del seed, quindi la firma di nuovo richiederebbe la riscrittura del corpo |
| URL prescritti                       | `presigned`    | La firma si trova nell'URL stesso, senza header `Authorization`                                                         |
| Firme asimmetriche SigV4A            | `sigv4a`       | Non esiste un HMAC a chiave condivisa da ricalcolare                                                                    |

<h4 id="mask-credential-files">
  Maschera i file di credenziali
</h4>

Le voci dei file accettano anche `"mode": "mask"`, che richiede Claude Code v2.1.221 o successivo. Ciò che un comando in sandbox vede dipende dalla piattaforma:

* **Linux e WSL2**: i comandi in sandbox leggono una copia sentinella del file, un sostituto il cui segreto è sostituito con un valore segnaposto, e il [proxy della sandbox](#network-isolation) sostituisce il valore reale all'uscita.
* **macOS**: i comandi in sandbox non possono leggere il file elencato. Claude Code non costruisce nessuna copia sentinella e non sostituisce nulla all'uscita, quindi gli strumenti che si autenticano con il file non funzionano all'interno della sandbox, lo stesso effetto di `deny`. A differenza di una voce `deny`, il blocco di lettura si mantiene anche quando [disabiliti l'isolamento del filesystem](#disable-filesystem-isolation).

Su ogni piattaforma, Claude Code applica il requisito [`network.tlsTerminate`](/docs/it/settings-reference#sandbox-network-tlsterminate) e `injectHosts` nello stesso modo che per le [variabili di ambiente mascherate](#mask-environment-variables), e ignora le impostazioni del repository nello stesso modo. Se [escludi le impostazioni utente](#configure-sandboxing), Claude Code mantiene le voci `mask` del file in `~/.claude/settings.json` come restrizioni, ma le voci non autorizzano più il proxy a sostituire il valore reale.

L'esempio seguente maschera un token GitHub memorizzato in `~/.config/gh/hosts.yml`; il pattern `extract`, coperto di seguito, dice a Claude Code quale parte del file è il segreto. Su Linux e WSL2, i comandi in sandbox che leggono il file ottengono una sentinella al posto del token, e il proxy sostituisce il token reale sulle richieste a `api.github.com`:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

Per confermare che la maschera è attiva, chiedi a Claude di eseguire `cat ~/.config/gh/hosts.yml` in un comando in sandbox: su Linux e WSL2 l'output mostra un valore sentinella al posto del token, e su macOS la lettura fallisce invece.

Su Linux e WSL2, il pattern `extract` è ciò che mantiene il resto di `hosts.yml` leggibile. Claude Code applica l'espressione regolare su tutto il file e sostituisce solo il testo catturato dal gruppo 1 di ogni corrispondenza, quindi `gh` continua a analizzare la sua configurazione e solo il token è un segnaposto. Usa `extract` per qualsiasi file strutturato che gli strumenti analizzano, come `.netrc`, JSON o YAML; il pattern deve contenere almeno un gruppo di cattura. Senza `extract`, Claude Code sostituisce l'intero contenuto del file con un valore sentinella, il che si adatta a un file che contiene un singolo segreto nudo e nient'altro.

Per un file che contiene un JSON Web Token (JWT), imposta `decode: "jwt"` invece di, o insieme a, `extract`. `decode` richiede Claude Code v2.1.224 o successivo. Claude Code trova candidati JWT con un pattern integrato, o con il tuo pattern `extract` quando impostato, verifica che ogni candidato sia un JWT, e lo sostituisce con un token falso strutturalmente valido, quindi il codice che decodifica il token all'interno della sandbox continua a funzionare. Aggiungi `maskClaims` per mascherare solo i claim del payload di livello superiore nominati all'interno di ogni token verificato e lascia gli altri claim leggibili. Quando nessun candidato si verifica, o nessun claim nominato corrisponde, il campo `onExtractNoMatch` di seguito governa il risultato, come fa per un pattern che non corrisponde a nulla.

Due campi opzionali affinano come si comporta la corrispondenza. Entrambi si applicano solo quando `mode` è `mask` e `extract` o `decode` è impostato. Su macOS, Claude Code applica le voci `mask` come `deny` prima che il pattern venga eseguito ogni volta che l'isolamento del filesystem è attivo, quindi questi campi, e i risultati di non corrispondenza di seguito, hanno effetto lì solo quando [l'isolamento del filesystem è disattivato](#disable-filesystem-isolation):

* `onExtractNoMatch` controlla cosa accade quando la corrispondenza non trova nulla da mascherare nel file:

  * `warn`, il valore predefinito, avverte e salta la voce, quindi i comandi in sandbox possono leggere il file reale senza mascheratura. Il valore predefinito si adatta alle credenziali che possono essere legittimamente assenti; se il segreto potrebbe essere presente ma il pattern potrebbe perderlo, usa `deny`
  * `deny` rende il file illeggibile invece
  * `error` interrompe il setup della sandbox finché non fissi la configurazione

  Claude Code tratta `deny` come `error` ogni volta che il blocco di lettura non sarebbe applicato: quando [disabiliti l'isolamento del filesystem](#disable-filesystem-isolation), e quando una voce `filesystem.allowRead` da qualsiasi fonte di impostazioni riapre il percorso del file.
* `maskDuplicates` sostituisce anche copie verbatim di ogni valore di credenziale mascherato, un'acquisizione `extract` o un token verificato `decode`, trovato al di fuori degli intervalli corrispondenti, per un segreto ripetuto dove la corrispondenza non raggiunge. Corrisponde a sottostringhe grezze, quindi un valore breve o comune sarebbe sostituito ovunque appaia; riservalo per segreti lunghi e ad alta entropia. Valore predefinito: false.

`mask` si applica a un singolo file, quindi elenca ogni file di credenziale individualmente. Claude Code fallback a `deny` per una voce `mask` che non può mascherare in sicurezza: un percorso di directory, un pattern glob, un file più grande di 8 MiB, o un file che non è testo UTF-8. Scrivi le directory come voci esplicite `deny` invece; la tabella sotto [Quali impostazioni possono disabilitarlo](#which-settings-can-disable-it) copre se ogni forma fissa `filesystem.disabled` e come si comporta con l'isolamento del filesystem disattivato.

<h2 id="how-sandboxing-works">
  Come funziona il sandboxing
</h2>

<h3 id="filesystem-isolation">
  Isolamento del file system
</h3>

Lo strumento Bash in sandbox limita l'accesso al file system a directory specifiche:

* **Comportamento di scrittura predefinito**: accesso in lettura e scrittura alla directory di lavoro corrente e alle sue sottodirectory, qualsiasi directory aggiunta con `--add-dir`, `/add-dir`, o [`permissions.additionalDirectories`](/docs/it/settings-reference#permissions-additionaldirectories), più la directory temporanea della sessione a cui `$TMPDIR` punta
* **Comportamento di lettura predefinito**: accesso in lettura all'intero computer, ad eccezione di determinate directory negate. Nota che questo predefinito consente comunque la lettura di file di credenziali come `~/.aws/credentials` e `~/.ssh/`. Utilizza [`sandbox.credentials`](#protect-credentials) per bloccare le letture di questi file e annullare le variabili di ambiente segrete, oppure aggiungi i percorsi a `denyRead`.
* **Accesso bloccato**: non è possibile modificare file al di fuori della directory di lavoro, delle directory aggiunte e della directory temporanea della sessione senza autorizzazione esplicita, inclusi file di configurazione shell come `~/.bashrc` e binari di sistema in `/bin/`
* **Git worktrees**: quando la directory di lavoro è un [git worktree collegato](/docs/it/worktrees), la sandbox consente anche scritture nella directory `.git` condivisa del repository principale in modo che comandi come `git commit` possano aggiornare i ref e l'indice. Le scritture in `hooks/` e `config` all'interno di quella directory rimangono negate.
* **Configurabile**: definisci percorsi consentiti e negati personalizzati tramite le impostazioni

Per saltare completamente l'isolamento del file system mantenendo l'isolamento della rete, imposta [`sandbox.filesystem.disabled`](#disable-filesystem-isolation).

<h3 id="protected-paths">
  Percorsi protetti
</h3>

All'interno delle directory in cui i comandi in sandbox possono scrivere, la sandbox nega comunque le scritture ai file da cui Claude Code carica la configurazione e il codice. Un comando che potrebbe modificare quei file potrebbe concedere a se stesso le autorizzazioni, oppure aggiungere un hook o un server MCP che Claude Code esegue al di fuori della sandbox. Il sistema di autorizzazione ha i suoi [percorsi protetti](/docs/it/permission-modes#protected-paths), che controllano cosa Claude Code approva prima che uno strumento venga eseguito; l'elenco della sandbox si applica a un comando che è già in esecuzione. Copre quattro gruppi di percorsi:

* **Nella directory di lavoro e nelle directory sopra di essa**: i file di impostazioni `.claude`, le directory `.claude/skills`, `.claude/agents`, `.claude/commands` e `.claude/hooks`, `.mcp.json`, e i file che Claude Code esegue autonomamente, come `.claude/workflows` e `.claude/scheduled_tasks.json`
* **Solo nella directory di lavoro**: file di avvio shell come `.bashrc` e `.zshrc`, `.gitconfig`, le directory `.vscode` e `.idea`, e `hooks` e `config` all'interno di `.git`
* **File che trasformerebbero la directory di lavoro in un repository git bare**: `HEAD`, `objects` e `refs` al livello superiore, più `config` e `hooks` lì quando un `HEAD` si trova accanto a loro. Un file denominato `config` è negato anche senza `HEAD`. Su Linux e WSL2, la sandbox elimina un file `HEAD` di livello superiore o una directory `objects` o `refs` che appare mentre un comando in sandbox è in esecuzione
* **In `~/.claude`, o nella directory a cui `CLAUDE_CONFIG_DIR` punta**: la maggior parte dei suoi contenuti, più `~/.claude.json` e l'archivio credenziali `.credentials.json`

Se un symlink appare al percorso di un file di impostazioni protetto durante la sessione, la sandbox nega anche le scritture al file a cui punta, a partire dal comando successivo.

Non c'è modo di esentare uno di questi percorsi: una voce `allowWrite` o una regola di autorizzazione `Edit` che copre il percorso non solleva la protezione. L'unico modo per disattivare la protezione è [`filesystem.disabled`](#disable-filesystem-isolation), che disattiva l'isolamento del file system per ogni percorso. Per vedere la maggior parte di questi percorsi risolti per la tua macchina, esegui `/sandbox` e apri la scheda **Config**, che li elenca sotto **Denied within allowed**, mescolati con le tue voci `denyWrite` personali.

Se `git merge` o `git checkout` fallisce con `unable to unlink old` su uno di questi percorsi, vedi [Troubleshooting](#troubleshooting).

<h3 id="network-isolation">
  Isolamento della rete
</h3>

L'accesso alla rete è controllato tramite un server proxy in esecuzione al di fuori della sandbox:

* **Restrizioni di dominio**: Claude Code non pre-consente alcun dominio per impostazione predefinita. La prima volta che un comando ha bisogno di un nuovo dominio, Claude Code richiede l'approvazione; in [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), Claude Code invece nomina gli host di cui un comando ha bisogno sul comando stesso, per [Domini consentiti per comando](#per-command-allowed-domains-in-auto-mode).
* **Scelte di approvazione**: se scegli Sì quando richiesto, Claude Code consente l'host per il resto della sessione corrente e non richiede di nuovo il prompt per le connessioni successive allo stesso host. Se scegli "Sì, e non chiedere di nuovo", Claude Code salva una regola di autorizzazione `WebFetch(domain:...)` alle tue [impostazioni locali](/docs/it/permissions#permission-system), in modo che l'host rimanga consentito nelle sessioni future.
* **Domini pre-consentiti**: pre-consenti i domini con [`allowedDomains`](/docs/it/settings-reference#sandbox-network-alloweddomains) per evitare completamente il prompt. Claude Code pre-consente anche i domini dalle regole di autorizzazione `WebFetch(domain:...)`, come descritto in [Regole di autorizzazione](#permission-rules).
* **Allowlist rigoroso**: se imposti [`strictAllowlist`](/docs/it/settings-reference#sandbox-network-strictallowlist) a `true` nelle impostazioni utente, gestite o CLI `--settings`, Claude Code nega ai comandi in sandbox l'accesso a qualsiasi host al di fuori dell'allowlist invece di richiedere. L'allowlist è lo stesso contro cui la sandbox altrimenti richiede: `allowedDomains` più domini dalle regole di autorizzazione `WebFetch(domain:...)`, oppure solo le voci di impostazioni gestite quando `allowManagedDomainsOnly` è impostato. Claude Code applica questo solo ai comandi in sandbox; gli strumenti in-process come `WebFetch` seguono comunque le loro [regole di autorizzazione](#permission-rules). Impostarlo nel `.claude/settings.json` o `.claude/settings.local.json` di un repository non ha effetto. Richiede Claude Code v2.1.219 o successivo.
* **Blocco gestito**: se [`allowManagedDomainsOnly`](/docs/it/settings-reference#sandbox-network-allowmanageddomainsonly) è impostato nelle impostazioni gestite, i domini non consentiti vengono bloccati automaticamente invece di richiedere, e solo `allowedDomains` e le regole di autorizzazione `WebFetch(domain:...)` dalle impostazioni gestite vengono rispettati.
* **Proxy aziendale**: quando la tua rete richiede che il traffico in uscita passi attraverso un proxy aziendale, imposta `HTTPS_PROXY`, `HTTP_PROXY` e `NO_PROXY` come [configurazione proxy](/docs/it/network-config#proxy-configuration) descrive, nel blocco `env` delle tue impostazioni in modo che gli [agenti in background](/docs/it/network-config#set-network-variables-in-settings-not-the-shell) li ottengano anche, oppure nell'ambiente da cui avvii Claude Code. Claude Code applica l'allowlist di dominio e quindi esegue il tunneling delle connessioni consentite attraverso quel proxy upstream.
* **Supporto proxy personalizzato**: gli utenti avanzati possono implementare regole personalizzate sul traffico in uscita
* **Copertura completa**: le restrizioni si applicano a tutti gli script, programmi e sottoprocessi generati dai comandi

In una regola `WebFetch(domain:...)`, la sandbox rispetta due forme di wildcard: un `*.` iniziale, come `*.example.com`, e un `*` nudo. La forma `*` nuda richiede Claude Code v2.1.186 o successivo. Un wildcard in qualsiasi altra posizione, come `WebFetch(domain:example.*)`, corrisponde comunque ai fetch ma non ha effetto sui comandi in sandbox.

<Note>
  Il proxy integrato applica l'allowlist in base al nome host richiesto e, per impostazione predefinita, non termina o ispeziona il traffico TLS. L'impostazione sperimentale [`network.tlsTerminate`](/docs/it/settings-reference#sandbox-network-tlsterminate), disponibile in Claude Code v2.1.199 e successivo, fa sì che il proxy integrato termini TLS stesso, che le voci di credenziali [`mask`](#mask-credentials) richiedono. Vedi [Limitazioni di sicurezza](#security-limitations) per le implicazioni dell'impostazione predefinita, e [Configurazione proxy personalizzata](#custom-proxy-configuration) se il tuo modello di minaccia richiede l'ispezione TLS.
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  Domini consentiti per comando in modalità automatica
</h4>

In [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) con sandboxing attivo, Claude nomina gli host di cui un comando ha bisogno sul comando stesso invece di attivare un'approvazione di rete per ogni connessione. Ogni comando Bash, PowerShell o [Monitor](/docs/it/tools-reference#monitor-tool) che viene eseguito nella sandbox può portare un elenco di host oltre l'allowlist della sandbox: un dominio come `registry.npmjs.org`, un wildcard come `*.pythonhosted.org`, o un indirizzo IP, ciascuno con una porta opzionale `:port`. Il classificatore esamina gli host insieme al comando. Richiede Claude Code v2.1.271 o successivo.

Un elenco approvato apre quegli host solo per quel comando, finché è in esecuzione. Nulla viene aggiunto agli host consentiti della sessione o alle tue impostazioni; il comando successivo nomina i suoi host.

Un comando che porta host va al classificatore invece di essere approvato da una regola di autorizzazione o dalla [modalità auto-allow](#sandbox-modes) della sandbox. Se una [regola ask](/docs/it/permissions#manage-permissions) forza un prompt per il comando, la finestra di dialogo di autorizzazione nel tuo terminale elenca gli host accanto ad esso, e approvarvi copre entrambi.

Un elenco per comando amplia solo ciò che la sandbox nega per impostazione predefinita. Le voci [`deniedDomains`](/docs/it/settings-reference#sandbox-network-denieddomains) bloccano comunque. Quando [`strictAllowlist`](/docs/it/settings-reference#sandbox-network-strictallowlist) o [`allowManagedDomainsOnly`](/docs/it/settings-reference#sandbox-network-allowmanageddomainsonly) blocca l'allowlist, Claude Code rifiuta gli elenchi per comando.

Mentre gli elenchi per comando si applicano, Claude Code rifiuta una connessione a un host che nessun comando approvato ha elencato, senza un prompt o un controllo del classificatore. Il rifiuto nomina l'host nel risultato del comando, e Claude esegue di nuovo il comando con l'host aggiunto.

<h4 id="ipv6-addresses-in-domain-lists">
  Indirizzi IPv6 negli elenchi di dominio
</h4>

Gli elenchi di dominio della sandbox sono `allowedDomains`, `deniedDomains` e le regole `WebFetch(domain:...)` che li alimentano. Per corrispondere a un indirizzo IPv6 in uno qualsiasi di essi, scrivi il letterale tra parentesi: `"[::1]"` corrisponde a quell'indirizzo su ogni porta, e `"[::1]:443"` lo corrisponde sulla porta 443 solo. Scrivi la porta come numero da 1 a 65535 senza zeri iniziali. La forma tra parentesi richiede Claude Code v2.1.229 o successivo. Prima di v2.1.229, quando il testo dopo l'ultimo due punti di una voce non tra parentesi era un numero di porta, Claude Code lo leggeva come uno, quindi `::1:443` denominava l'indirizzo `::1` sulla porta 443.

Quando scegli "Sì, e non chiedere di nuovo" al prompt di approvazione della rete per un indirizzo IPv6, Claude Code salva la regola `WebFetch(domain:...)` con l'indirizzo tra parentesi, in modo che la regola continui a corrispondere all'indirizzo nelle sessioni future.

Una voce non tra parentesi con due o più due punti è ambigua: `::1:443` è sia un indirizzo IPv6 completo che un indirizzo seguito da una porta. Claude Code applica le ortografie ambigue in modo conservativo invece di indovinare quale lettura intendevi:

* **Elenchi di negazione**: Claude Code nega ogni lettura che la voce analizza come, quindi qualunque lettura intendevi è bloccata. Per una voce senza lettura analizzabile, Claude Code non blocca nulla.
* **Elenchi di consentimento**: Claude Code non consente mai più di quello che hai scritto. Riscrive una voce ambigua alla sua lettura host-e-porta quando quella lettura analizza in modo pulito, e può eliminare completamente la voce piuttosto che ampliare l'allowlist.

Esegui `claude doctor` nel tuo terminale per trovare le voci interessate: l'avviso `Sandbox network domain entries have unreliable spellings` nomina fino a tre di esse e conta il resto. Riscrivi ognuna nella forma tra parentesi per cancellare l'avviso. L'avviso nomina anche voci la cui ortografia è inaffidabile per altri motivi, come `@`, caratteri di percorso o query, o wildcard all'interno di parentesi.

<h3 id="os-level-enforcement">
  Applicazione a livello del sistema operativo
</h3>

Lo strumento Bash in sandbox utilizza primitive di sicurezza del sistema operativo:

* **macOS**: utilizza Seatbelt per l'applicazione della sandbox
* **Linux**: utilizza [bubblewrap](https://github.com/containers/bubblewrap) per l'isolamento
* **WSL2**: utilizza bubblewrap, come Linux

WSL1 non è supportato perché bubblewrap richiede funzionalità del kernel disponibili solo in WSL2.

Questi stessi primitivi sono disponibili come pacchetto standalone [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime), che la pagina [Sandbox environments](/docs/it/sandbox-environments#sandbox-runtime) copre come approccio separato per avvolgere l'intero processo di Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Come il sandboxing si relaziona alle autorizzazioni e alle modalità di autorizzazione
</h2>

Il sandboxing, le [regole di autorizzazione](/docs/it/permissions), e le [modalità di autorizzazione](/docs/it/permission-modes) sono livelli complementari. Le sezioni seguenti spiegano come la sandbox interagisce con ciascuno.

<h3 id="permission-rules">
  Regole di autorizzazione
</h3>

Le regole di autorizzazione e il sandboxing controllano cose diverse:

* **Le regole di autorizzazione** controllano quali strumenti Claude Code può utilizzare e vengono valutate prima che qualsiasi strumento venga eseguito. Si applicano a ogni strumento: Bash, Read, Edit, WebFetch, MCP e altri, tranne per il fatto che una regola di negazione o richiesta non può bloccare [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior) mentre rimane qualsiasi altro strumento.
* **Il sandboxing** fornisce l'applicazione a livello del sistema operativo che limita ciò a cui i comandi della shell possono accedere a livello di filesystem e di rete. Si applica solo ai comandi Bash, PowerShell e [Monitor](/docs/it/tools-reference#monitor-tool) e ai loro processi figlio.

I due livelli differiscono anche nel modo in cui vengono applicati. Claude Code valuta le decisioni di autorizzazione prima che un comando venga eseguito, in base alla stringa di comando e, in modalità automatica, al giudizio di un classificatore separato su se il comando è sicuro. Il sistema operativo applica il limite della sandbox al processo in esecuzione, quindi rimane indipendentemente da ciò che il modello ha scelto di eseguire e anche se un comando consentito fa più di quanto il suo nome suggerisca.

Le restrizioni del filesystem e della rete sono configurate sia attraverso le impostazioni della sandbox che attraverso le regole di autorizzazione:

| Impostazione o regola                                          | Cosa fa                                                                                                            |
| :------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                | Concede l'accesso in scrittura del sottoprocesso ai percorsi al di fuori della directory di lavoro                 |
| `sandbox.filesystem.denyWrite` e `sandbox.filesystem.denyRead` | Bloccano l'accesso del sottoprocesso a percorsi specifici                                                          |
| `sandbox.filesystem.allowRead`                                 | Consente nuovamente la lettura di percorsi specifici all'interno di una regione `denyRead`                         |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation) | Disattiva completamente il livello del filesystem mantenendo l'isolamento della rete                               |
| Regole di autorizzazione `Edit`                                | Concedono l'accesso in scrittura a percorsi specifici, nello stesso modo in cui `sandbox.filesystem.allowWrite` fa |
| Regole di negazione `Read` e `Edit`                            | Bloccano l'accesso a file o directory specifici                                                                    |
| Regole di autorizzazione e negazione `WebFetch(domain:...)`    | Controllano l'accesso al dominio                                                                                   |
| `allowedDomains` della sandbox                                 | Controlla quali domini i comandi Bash possono raggiungere                                                          |
| `deniedDomains` della sandbox                                  | Blocca domini specifici anche quando un wildcard `allowedDomains` più ampio altrimenti li permetterebbe            |

I percorsi e i domini sia dalle impostazioni della sandbox che dalle regole di autorizzazione vengono uniti nella configurazione finale della sandbox.

La [directory degli esempi del repository claude-code](https://github.com/anthropics/claude-code/tree/main/examples/settings) include configurazioni di impostazioni iniziali per scenari di distribuzione comuni, inclusi esempi specifici della sandbox. Utilizzate questi come punti di partenza e adattateli alle vostre esigenze.

<h3 id="permission-modes">
  Modalità di autorizzazione
</h3>

`/sandbox` non è una [modalità di autorizzazione](/docs/it/permission-modes). Le modalità di autorizzazione decidono se una chiamata di strumento viene eseguita e se siete richiesti per primo, mentre la sandbox limita ciò a cui un comando Bash può accedere una volta eseguito. Differiscono in ciò che controllano e cosa sostituisce il prompt per azione:

|                                                                              | Cosa controlla                                            | Cosa sostituisce il prompt                                                                                                                                                                                                     |
| :--------------------------------------------------------------------------- | :-------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                                   | Ciò a cui un comando Bash può accedere una volta eseguito | Il limite della sandbox stesso, in [modalità auto-allow](#sandbox-modes)                                                                                                                                                       |
| [Modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) | Se ogni chiamata di strumento viene eseguita              | Un classificatore che esamina le azioni                                                                                                                                                                                        |
| `--dangerously-skip-permissions`                                             | Se ogni chiamata di strumento viene eseguita              | Niente. I controlli del [percorso protetto](/docs/it/permission-modes#protected-paths) vengono anche saltati; le [azioni che nessuna modalità auto-approva](/docs/it/permission-modes#actions-no-mode-auto-approves) si applicano ancora |

La [modalità auto-allow](#sandbox-modes) della sandbox è separata dalla [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode): auto-allow approva i comandi Bash perché il limite della sandbox li contiene, mentre la modalità automatica utilizza un classificatore per esaminare le azioni. I due funzionano indipendentemente e possono essere combinati, con le eccezioni elencate in [Modalità sandbox](#sandbox-modes). Per scegliere un limite di isolamento per esecuzioni incustodite, vedere [Ambienti sandbox](/docs/it/sandbox-environments#how-isolation-relates-to-permission-modes). Per una tabella degli accoppiamenti comuni di modalità di autorizzazione e sandbox con i flag che avviano ciascuno, vedere [Configurazioni comuni](/docs/it/permission-modes#common-setups).

<h2 id="configure-the-sandbox-for-your-organization">
  Configura la sandbox per la tua organizzazione
</h2>

Gli amministratori possono richiedere il sandboxing per ogni utente, impedire agli sviluppatori di ampliare la politica e instradare il traffico sandbox attraverso un proxy aziendale.

<h3 id="enforce-sandboxing-with-managed-settings">
  Enforce sandboxing with managed settings
</h3>

Per richiedere la sandbox per ogni sviluppatore, fornisci le chiavi `sandbox` tramite [impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms), sia come file gestito dal tuo MDM che tramite [impostazioni gestite dal server](/docs/it/server-managed-settings) su claude.ai.

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
* **`allowUnsandboxedCommands: false`**: Claude Code ignora l'escape hatch `dangerouslyDisableSandbox`, quindi quando un comando fallisce sotto la sandbox, Claude non può riprovarlo senza sandbox

Due aggiunte meritano considerazione insieme a loro. Aggiungi `excludedCommands` per qualsiasi strumento approvato dall'organizzazione che deve essere eseguito senza isolamento. Aggiungi voci [`sandbox.credentials`](#protect-credentials) per directory di credenziali come `~/.aws` e `~/.ssh` e per variabili di ambiente segrete, poiché la politica di lettura predefinita le consente comunque.

Questa configurazione sandboxa i comandi che Claude esegue. Uno sviluppatore può comunque digitare un comando al [prompt della modalità shell con `!`](/docs/it/interactive-mode#shell-mode-with-prefix) ed eseguirlo al di fuori della sandbox, con lo stesso accesso che ha già in qualsiasi terminale al di fuori di Claude Code. Vedi [L'escape hatch di riprovazione senza sandbox](#the-unsandboxed-retry-escape-hatch) per le sessioni in cui i comandi digitati vengono eseguiti in sandbox.

La sandbox non viene eseguita su Windows nativo, quindi se la tua flotta include host Windows, limita questa configurazione a macOS e Linux o fai in modo che quegli utenti eseguano Claude Code all'interno di WSL2 o di un container.

<h3 id="keep-developers-from-widening-the-policy">
  Keep developers from widening the policy
</h3>

Per chiavi booleane come `enabled` e `failIfUnavailable`, Claude Code utilizza il valore gestito e ignora qualsiasi cosa uno sviluppatore imposti localmente. Per chiavi array come `excludedCommands` e `allowRead`, Claude Code unisce le voci da ogni ambito che la sessione carica, quindi uno sviluppatore può aggiungere voci che ampliano la politica.

Imposta `allowManagedReadPathsOnly` su `true` nelle impostazioni gestite in modo che solo le voci `allowRead` dalle impostazioni gestite vengono rispettate. Questo impedisce agli sviluppatori di ampliare l'accesso in lettura oltre i percorsi approvati dall'organizzazione. Per bloccare i domini di rete ai valori gestiti allo stesso modo, imposta [`allowManagedDomainsOnly`](/docs/it/settings-reference#sandbox-network-allowmanageddomainsonly).

Quando le impostazioni gestite configurano `sandbox.filesystem` o elencano qualsiasi voce `sandbox.credentials.files` con `"mode": "deny"`, solo le impostazioni gestite possono impostare [`filesystem.disabled`](#disable-filesystem-isolation), quindi gli sviluppatori non possono disattivare le restrizioni del filesystem distribuite dall'amministratore. Se una voce `mask` fissa la chiave dipende da come si risolve; la tabella sotto [Which settings can disable it](#which-settings-can-disable-it) copre i quattro casi.

`excludedCommands` non ha un equivalente blocco solo gestito, quindi uno sviluppatore può sempre aggiungere voci che eseguono comandi aggiuntivi al di fuori della sandbox. Mantieni l'elenco gestito ristretto.

<h3 id="custom-proxy-configuration">
  Custom proxy configuration
</h3>

Per le organizzazioni che richiedono una sicurezza di rete avanzata, puoi implementare un proxy personalizzato per:

* Decrittare e ispezionare il traffico HTTPS
* Applicare regole di filtraggio personalizzate
* Registrare tutte le richieste di rete
* Integrarsi con l'infrastruttura di sicurezza esistente

Per puntare Claude Code al tuo proxy, imposta le porte proxy nelle [impostazioni sandbox](/docs/it/settings-reference#sandbox-settings):

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

* **I comandi falliscono con un errore host-not-allowed**: molti strumenti CLI devono raggiungere host specifici. Concedere l'autorizzazione quando richiesto aggiunge l'host al vostro elenco consentito in modo che lo strumento venga eseguito all'interno della sandbox in futuro.
* **`jest` si blocca o fallisce**: `watchman` è incompatibile con la sandbox. Eseguite `jest --no-watchman` invece.
* **I CLI basati su Go falliscono la verifica TLS su macOS**: strumenti come `gh`, `gcloud` e `terraform` potrebbero fallire la verifica TLS sotto Seatbelt. Elencate questi strumenti in [`excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands). Se state utilizzando `httpProxyPort` con un proxy MITM e CA personalizzato, impostate [`enableWeakerNetworkIsolation`](/docs/it/settings-reference#sandbox-enableweakernetworkisolation) su `true` invece.
* **`open`, `osascript`, o i flussi di autenticazione basati su browser falliscono con errore `-600` su macOS**: la sandbox blocca gli Apple Events per impostazione predefinita. Impostate [`allowAppleEvents`](/docs/it/settings-reference#sandbox-allowappleevents) su `true` nelle impostazioni utente, gestite o CLI per consentirli. Le impostazioni del progetto vengono ignorate per questa chiave. L'abilitazione rimuove l'isolamento dell'esecuzione del codice, poiché i comandi sandboxati possono quindi avviare altre applicazioni non sandboxate senza prompt dell'utente e inviare comandi AppleScript alle applicazioni in esecuzione, soggetti al prompt di consenso dell'automazione macOS (TCC). In alternativa, aggiungete il comando a [`excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands).
* **I comandi `docker` falliscono**: `docker` è incompatibile con la sandbox. Aggiungete `docker *` a [`excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands).
* **`pbcopy`, `xclip`, o `wl-copy` non aggiorna gli appunti**: queste utilità degli appunti possono non riuscire a raggiungere gli appunti di sistema dall'interno della sandbox, nel qual caso il testo inviato tramite pipe a loro non arriva.

  Per mettere l'output di Claude negli appunti, chiedete a Claude di stamparlo nella sua risposta, quindi eseguite [`/copy`](/docs/it/commands). `/copy` scrive negli appunti dal processo Claude Code piuttosto che da un comando sandboxato.

  Quando Claude invia testo tramite pipe a uno di questi strumenti, aggiungere lo strumento a [`excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands) non toglie quella chiamata dalla sandbox di per sé.
* **Un comando git fallisce con `unable to unlink old`**: `git merge`, `git checkout` e comandi simili falliscono in questo modo quando devono sostituire un file a cui la sandbox nega le scritture, sia che il file sia sotto un [percorso protetto](#protected-paths) come `.claude/skills`, sotto una delle vostre voci `denyWrite`, o al di fuori delle directory in cui la sandbox consente ai comandi di scrivere. Su Linux e WSL2 l'errore termina con `Read-only file system`.

  Dopo il fallimento, Claude potrebbe [offrire di rieseguire il comando al di fuori della sandbox](#the-unsandboxed-retry-escape-hatch); approvate quel nuovo tentativo, o eseguite il comando git voi stessi in un altro terminale. Se avete impostato `allowUnsandboxedCommands` su `false`, Claude non può offrire il nuovo tentativo, quindi eseguite il comando voi stessi. Se lo stesso comando git fallisce spesso, aggiungetelo a [`excludedCommands`](/docs/it/settings-reference#sandbox-excludedcommands).
* **Bubblewrap non riesce ad avviarsi all'interno di un container**: in un container senza privilegi, bubblewrap non può montare un filesystem `/proc` fresco, quindi i comandi sandboxati falliscono con un errore `bwrap` come `Can't mount proc on /newroot/proc: Operation not permitted`. Impostate [`enableWeakerNestedSandbox`](/docs/it/settings-reference#sandbox-enableweakernestedsandbox) su `true` in modo che la sandbox interna bind-monti il `/proc` esistente del container invece. Utilizzate questa impostazione solo quando il container esterno fornisce già il confine di isolamento di cui avete bisogno, poiché espone le informazioni del processo ai comandi sandboxati che un mount `/proc` fresco nasconderebbe.
* **File di sola lettura a 0 byte appaiono nei percorsi delle impostazioni `.claude`, e "Sì, e non chiedere più" non salva**: su Linux e WSL2, la sandbox mantiene un diniego di scrittura su un file che non esiste ancora creando un placeholder di sola lettura a 0 byte lì mentre un comando sandboxato viene eseguito. La sandbox rimuove il placeholder in seguito. Se una sessione viene terminata prima che quella pulizia venga eseguita, ad esempio da SIGKILL, i placeholder rimangono. Le sessioni successive li bind-montano di sola lettura di nuovo ad ogni avvio, quindi una scrittura delle impostazioni come il salvataggio di una scelta di autorizzazione fallisce dove uno si trova.

  Eseguite `claude doctor` per elencare i file placeholder rimasti. L'avviso [`Stale sandbox mask files left by a killed session`](/docs/it/errors#stale-sandbox-mask-files-left-by-a-killed-session) nomina fino a tre di essi e conta il resto. Eliminate ogni file con `rm` mentre nessun'altra sessione Claude Code è in esecuzione in quel progetto. Prima della v2.1.257, Claude Code lasciava gli stessi placeholder dietro senza segnalarli.
* **`--dangerously-skip-permissions` fallisce come root**: questo flag viene bloccato quando viene eseguito come root o tramite sudo su Linux e macOS, perché l'accesso root combinato con nessun prompt di autorizzazione può modificare qualsiasi file o servizio sul sistema. Il controllo viene saltato automaticamente all'interno di una sandbox riconosciuta. Per eseguire autonomamente in un container, utilizzate la configurazione [dev container](/docs/it/devcontainer), che esegue Claude Code come utente non root.

<h2 id="limitations">
  Limitazioni
</h2>

Il sandboxing riduce il rischio ma non è un confine di isolamento completo. Rivedi le limitazioni seguenti prima di fare affidamento su di esso come controllo di sicurezza rigido.

<h3 id="security-limitations">
  Limitazioni di sicurezza
</h3>

* **Filtraggio della rete**: il sandbox limita i domini a cui i processi possono connettersi. Per impostazione predefinita il proxy integrato non termina o ispeziona TLS sul traffico in uscita, quindi i contenuti delle connessioni crittografate non vengono esaminati. L'impostazione sperimentale [`network.tlsTerminate`](/docs/it/settings-reference#sandbox-network-tlsterminate) termina TLS al proxy per la [sostituzione delle credenziali `mask`](#mask-credentials) ma non aggiunge filtraggio dei contenuti. Sei responsabile di assicurarti che solo i domini affidabili siano consentiti nella tua politica.

<Warning>
  Consentire domini ampi come `github.com` può creare percorsi per l'esfiltrazione di dati. Poiché il proxy prende la sua decisione di consentimento dal nome host fornito dal client senza ispezionare TLS, il codice in esecuzione all'interno della sandbox potrebbe potenzialmente utilizzare [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) o tecniche simili per raggiungere host al di fuori dell'allowlist. Se il tuo modello di minaccia richiede garanzie più forti, configura un [proxy personalizzato](#custom-proxy-configuration) che termina TLS e ispeziona il traffico, e installa il suo certificato CA all'interno della sandbox. L'isolamento della rete più consapevole di TLS è un'area di sviluppo attiva.
</Warning>

* **Escalation dei privilegi tramite socket Unix**: la configurazione `allowUnixSockets` può inavvertitamente concedere l'accesso a servizi di sistema che potrebbero portare a bypass della sandbox. Ad esempio, consentire l'accesso a `/var/run/docker.sock` concede effettivamente l'accesso al sistema host attraverso il socket Docker. Considera attentamente qualsiasi socket Unix che consenti attraverso la sandbox.
* **Escalation dei permessi del filesystem**: i permessi di scrittura del filesystem eccessivamente ampi possono abilitare attacchi di escalation dei privilegi. Consentire scritture a directory contenenti eseguibili in `$PATH`, directory di configurazione di sistema o file di configurazione della shell dell'utente come `.bashrc` o `.zshrc` può portare all'esecuzione di codice in diversi contesti di sicurezza quando altri utenti o processi di sistema accedono a questi file.
* **Forza della sandbox Linux**: l'implementazione Linux fornisce un forte isolamento del filesystem e della rete ma include una modalità `enableWeakerNestedSandbox` che le consente di funzionare all'interno di ambienti Docker senza namespace privilegiati, o su host Linux dove gli spazi dei nomi utente senza privilegi sono disabilitati da sysctl. Questa opzione indebolisce considerevolmente la sicurezza e dovrebbe essere utilizzata solo quando l'isolamento aggiuntivo è altrimenti applicato.
* **Apple Events su macOS**: la sandbox macOS blocca gli Apple Events per impostazione predefinita. L'impostazione `allowAppleEvents` rimuove questa restrizione in modo che strumenti come `open` e `osascript` funzionino, ma rimuove l'isolamento dell'esecuzione del codice: i comandi sandboxati possono avviare altre applicazioni senza sandbox senza alcun prompt dell'utente, e possono inviare comandi AppleScript alle applicazioni in esecuzione, soggetti al prompt di consenso per l'automazione macOS per app (TCC). È onorato solo dalle impostazioni utente, gestite o CLI. Le impostazioni del progetto non possono abilitarlo.

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
* **Variabili di ambiente**: i comandi Bash sandboxati ereditano l'ambiente del processo padre per impostazione predefinita, incluse le credenziali impostate lì. Usa [`sandbox.credentials`](#protect-credentials) per annullare o mascherare variabili specifiche per i comandi sandboxati, o imposta [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/it/env-vars) per rimuovere le credenziali dai sottoprocessi.
* **Subagenti**: i [subagenti](/docs/it/sub-agents) vengono eseguiti nello stesso processo della sessione padre e utilizzano la stessa configurazione sandbox. I comandi Bash all'interno di un subagente vengono sandboxati quando il sandboxing è abilitato nella sessione padre.

<Warning>
  Il sandboxing efficace richiede sia l'isolamento del filesystem che della rete. Senza isolamento della rete, un agente compromesso potrebbe esfiltare file sensibili come chiavi SSH. Senza isolamento del filesystem, sia da una politica permissiva che da [disabilitazione del livello filesystem](#disable-filesystem-isolation), un agente compromesso potrebbe backdoor le risorse di sistema per ottenere accesso alla rete. Quando ampli i predefiniti, verifica che un percorso `allowWrite`, una voce `allowedDomains` ampia o un'eccezione `excludedCommands` non annulli una restrizione dall'altro lato.
</Warning>

<h2 id="see-also">
  See also
</h2>

* [Sandbox environments](/docs/it/sandbox-environments): confronta la sandbox integrata con dev container, container e VM
* [Security](/docs/it/security): funzionalità di sicurezza complete e best practice
* [Permissions](/docs/it/permissions): configurazione delle autorizzazioni e controllo dell'accesso
* [All settings](/docs/it/settings-reference): ogni chiave di configurazione
* [CLI reference](/docs/it/cli-reference): opzioni della riga di comando
