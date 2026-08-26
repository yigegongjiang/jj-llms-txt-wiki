> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Contenitori di sviluppo

> Esegui Claude Code all'interno di un contenitore di sviluppo per ambienti coerenti e isolati in tutto il tuo team.

Un [contenitore di sviluppo](https://containers.dev/), o dev container, ti consente di definire un ambiente identico e isolato che ogni ingegnere del tuo team può eseguire. Con Claude Code installato in quel contenitore, i comandi che Claude esegue vengono eseguiti al suo interno piuttosto che sulla macchina host, mentre le modifiche ai file del tuo progetto vengono visualizzate nel tuo repository locale mentre lavori.

Questa pagina copre [l'installazione di Claude Code in un dev container](#add-claude-code-to-your-dev-container), quindi una serie di argomenti di configurazione autonomi: persistenza dell'autenticazione tra i rebuild, applicazione della politica organizzativa, restrizione dell'uscita di rete ed esecuzione senza prompt di autorizzazione. Leggi quelli che corrispondono alla tua configurazione.

<Warning>
  Sebbene il dev container fornisca protezioni sostanziali, nessun sistema è completamente immune da tutti gli attacchi.
  Quando eseguito con `--dangerously-skip-permissions`, i dev container non impediscono a un progetto dannoso di estrarre qualsiasi cosa accessibile all'interno del contenitore, incluse le credenziali di Claude Code archiviate in [`~/.claude`](/docs/it/claude-directory).
  Utilizza i dev container solo quando sviluppi con repository affidabili e monitora le attività di Claude.
  Evita di montare segreti host come `~/.ssh` o file di credenziali cloud nel contenitore; preferisci token con ambito repository o di breve durata.
</Warning>

<Accordion title="Come i dev container funzionano con il tuo editor">
  <img src="https://mintcdn.com/claude-code/YvJyjZfd9yMihr0i/images/devcontainer-architecture.svg?fit=max&auto=format&n=YvJyjZfd9yMihr0i&q=85&s=9017b1d16a446c6cc37ba562f35b9aae" className="dark:hidden" alt="Diagramma che mostra un editor sull'host che si connette a un dev container Docker. Claude Code, il terminale e gli strumenti di build vengono eseguiti all'interno del contenitore. Il repository host è bind-montato nel contenitore come workspace." width="640" height="300" data-path="images/devcontainer-architecture.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/devcontainer-architecture-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=a0a340b1f2afc6a590696102c8acaaca" className="hidden dark:block" alt="Diagramma che mostra un editor sull'host che si connette a un dev container Docker. Claude Code, il terminale e gli strumenti di build vengono eseguiti all'interno del contenitore. Il repository host è bind-montato nel contenitore come workspace." width="640" height="300" data-path="images/devcontainer-architecture-dark.svg" />

  Un dev container viene eseguito come contenitore Docker, sia sulla tua macchina che su un host cloud come GitHub Codespaces. Un editor che supporta la specifica Dev Containers, come VS Code, GitHub Codespaces, un IDE JetBrains o Cursor, si connette a quel contenitore: navighi e modifichi i file nell'editor come al solito, ma il terminale integrato, i language server e gli strumenti di build vengono tutti eseguiti all'interno del contenitore piuttosto che sul tuo host. Gli editor senza supporto per dev container, come Vim semplice, non fanno parte di questo flusso di lavoro.

  Claude Code viene eseguito all'interno del contenitore, quindi vede gli stessi file, dipendenze e strumenti del resto della toolchain del tuo progetto. In VS Code puoi utilizzare il [pannello dell'estensione Claude Code](/docs/it/vs-code) o eseguire `claude` nel terminale integrato; entrambi vengono eseguiti all'interno del contenitore e condividono la stessa configurazione `~/.claude`.
</Accordion>

<h2 id="add-claude-code-to-your-dev-container">
  Aggiungi Claude Code al tuo dev container
</h2>

Claude Code si installa in qualsiasi dev container tramite la [Claude Code Dev Container Feature](https://github.com/anthropics/devcontainer-features/tree/main/src/claude-code).

Le impostazioni funzionano con qualsiasi strumento che supporti la specifica Dev Containers, come VS Code, GitHub Codespaces o IDE JetBrains. I passaggi seguenti utilizzano VS Code come esempio.

Quando apri il contenitore in VS Code o Codespaces, la feature aggiunge anche l'estensione Claude Code VS Code; altri editor ignorano quella parte.

<Tip>
  Nuovo ai dev container? Il [tutorial VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/tutorial) ti guida attraverso l'installazione di Docker, l'estensione e l'apertura del tuo primo contenitore. Per un esempio più completo e hardened con un firewall e volumi persistenti, vedi [Prova il contenitore di riferimento](#try-the-reference-container).
</Tip>

<Steps>
  <Step title="Crea o aggiorna devcontainer.json">
    Salva quanto segue come `.devcontainer/devcontainer.json` nel tuo repository, o aggiungi il blocco `features` al tuo file esistente.

    Il tag della versione alla fine, come `:1.0`, fissa lo script di installazione della feature, non la versione di Claude Code. La feature installa l'ultimo Claude Code, e Claude Code si auto-aggiorna automaticamente all'interno del contenitore per impostazione predefinita.

    Per fissare la versione CLI o disabilitare l'auto-aggiornamento, vedi [Applica la politica organizzativa](#enforce-organization-policy).

    ```json .devcontainer/devcontainer.json theme={null}
    {
      "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
      "features": {
        "ghcr.io/anthropics/devcontainer-features/claude-code:1.0": {}
      }
    }
    ```

    Sostituisci la riga `image` con l'immagine base del tuo progetto o rimuovila se il tuo file esistente utilizza un Dockerfile.
  </Step>

  <Step title="Ricostruisci il contenitore">
    Apri il Command Palette di VS Code con `Cmd+Shift+P` su Mac o `Ctrl+Shift+P` su Windows e Linux, ed esegui **Dev Containers: Rebuild Container**.

    Per altri strumenti, segui l'azione di rebuild di quello strumento: vedi [ricostruzione in GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-a-codespace/rebuilding-the-container-in-a-codespace), la [CLI Dev Containers](https://github.com/devcontainers/cli), o la documentazione dev container del tuo IDE.
  </Step>

  <Step title="Accedi a Claude Code">
    Apri un terminale nel contenitore ricostruito ed esegui `claude`, quindi segui il prompt di autenticazione.
  </Step>
</Steps>

Quello che vedi al prompt di autenticazione dipende dal tuo provider:

* **Anthropic**: accedi tramite browser con il tuo account Claude o Anthropic Console
* **[Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry](/docs/it/third-party-integrations)**: Claude Code utilizza le tue credenziali del provider cloud, senza prompt del browser

Per i provider cloud, passa le credenziali nel contenitore come variabili di ambiente tramite `containerEnv`, un segreto di Codespaces, o l'identità del carico di lavoro del tuo cloud piuttosto che montare file di credenziali dall'host. Vedi [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry) per la catena di credenziali che Claude Code legge.

Vedi [Scegli il tuo provider API](/docs/it/admin-setup#choose-your-api-provider) per decidere quale percorso si adatta alla tua organizzazione.

<Note>
  Se l'accesso al browser si completa ma il callback non raggiunge mai il contenitore, copia il codice mostrato nel browser e incollalo al prompt `Paste code here if prompted` nel terminale. Questo può accadere quando l'inoltro delle porte dell'editor non instrada il callback localhost.
</Note>

<h2 id="persist-authentication-and-settings-across-rebuilds">
  Mantieni l'autenticazione e le impostazioni tra i rebuild
</h2>

Per impostazione predefinita, la directory home del contenitore viene scartata al rebuild, quindi gli ingegneri devono accedere di nuovo ogni volta. Claude Code archivia il suo token di autenticazione, le impostazioni utente e la cronologia della sessione in [`~/.claude`](/docs/it/claude-directory). Monta un volume denominato in quel percorso per mantenere questo stato tra i rebuild.

L'esempio seguente monta un volume nella directory home dell'utente `node`:

```json devcontainer.json theme={null}
"mounts": [
  "source=claude-code-config,target=/home/node/.claude,type=volume"
]
```

Sostituisci `/home/node` con la directory home del `remoteUser` del tuo contenitore. Se monti il volume in un luogo diverso da `~/.claude`, imposta [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars) al percorso di montaggio in modo che Claude Code legga e scriva lì.

Per isolare lo stato per progetto piuttosto che condividere un volume su tutti i repository, includi la variabile `${devcontainerId}` nel nome della sorgente. La [configurazione di riferimento](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) utilizza `source=claude-code-config-${devcontainerId}` per questo scopo.

In GitHub Codespaces, `~/.claude` persiste tra l'arresto e l'avvio di un codespace, ma viene comunque cancellato quando ricostruisci il contenitore, quindi il mount del volume sopra si applica anche lì. Per portare l'autenticazione tra i codespace, archivia `ANTHROPIC_API_KEY` o un `CLAUDE_CODE_OAUTH_TOKEN` da [`claude setup-token`](/docs/it/authentication#generate-a-long-lived-token) come [segreto di Codespaces](https://docs.github.com/en/codespaces/managing-your-codespaces/managing-your-account-specific-secrets-for-github-codespaces); Codespaces rende i segreti disponibili come variabili di ambiente all'interno del contenitore automaticamente.

<h2 id="enforce-organization-policy">
  Applica la politica organizzativa
</h2>

Un dev container è un luogo conveniente per applicare la politica organizzativa, perché la stessa immagine e configurazione vengono eseguite sulla macchina di ogni ingegnere.

Claude Code legge `/etc/claude-code/managed-settings.json` su Linux e lo applica con la massima precedenza nella [gerarchia delle impostazioni](/docs/it/settings#how-scopes-interact), quindi i valori lì sovrascrivono qualsiasi cosa un ingegnere imposti in `~/.claude` o nella directory `.claude/` del progetto. Copia il file in posizione dal tuo Dockerfile:

```dockerfile Dockerfile theme={null}
RUN mkdir -p /etc/claude-code
COPY managed-settings.json /etc/claude-code/managed-settings.json
```

Poiché il Dockerfile risiede nel repository, chiunque abbia accesso in scrittura può modificare o rimuovere questo passaggio. Per la politica che gli ingegneri non possono aggirare modificando i file del repository, fornisci le impostazioni gestite tramite [impostazioni gestite dal server](/docs/it/server-managed-settings) o il tuo MDM. Vedi [file di impostazioni gestite](/docs/it/settings#settings-files) per le chiavi disponibili e gli altri percorsi di consegna.

Per impostare [variabili di ambiente](/docs/it/env-vars) che si applicano a ogni sessione di Claude Code nel contenitore, aggiungile a `containerEnv` nel tuo `devcontainer.json`. L'esempio seguente disattiva la telemetria e la segnalazione degli errori e impedisce a Claude Code di auto-aggiornarsi dopo l'installazione:

```json devcontainer.json theme={null}
"containerEnv": {
  "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
  "DISABLE_AUTOUPDATER": "1"
}
```

La Dev Container Feature installa sempre l'ultima versione di Claude Code. Per fissare una versione specifica di Claude Code per build riproducibili, installala dal tuo Dockerfile con `npm install -g @anthropic-ai/claude-code@X.Y.Z` invece di utilizzare la feature, e imposta `DISABLE_AUTOUPDATER` come mostrato sopra.

Per l'elenco completo dei controlli di politica incluse le regole di autorizzazione, le restrizioni degli strumenti e gli allowlist dei server MCP, vedi [Configura Claude Code per la tua organizzazione](/docs/it/admin-setup).

Per rendere disponibili i [server MCP](/docs/it/mcp) all'interno del contenitore, definiscili a [ambito di progetto](/docs/it/mcp#mcp-installation-scopes) in un file `.mcp.json` alla radice del repository in modo che siano archiviati insieme alla configurazione del tuo dev container. Installa tutti i binari su cui i server stdio locali dipendono nel tuo Dockerfile, e aggiungi i domini dei server remoti al tuo allowlist di rete.

<h2 id="restrict-network-egress">
  Limita l'uscita di rete
</h2>

Puoi limitare il traffico in uscita del contenitore solo ai domini di cui Claude Code ha bisogno. Vedi [Requisiti di accesso alla rete](/docs/it/network-config#network-access-requirements) per i domini di inferenza e autenticazione, e [Servizi di telemetria](/docs/it/data-usage#telemetry-services) per le connessioni opzionali di telemetria e segnalazione degli errori e come disabilitarle.

Il contenitore di riferimento include uno script [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh) che blocca tutto il traffico in uscita tranne i domini di cui Claude Code e i tuoi strumenti di sviluppo hanno bisogno. L'esecuzione di un firewall all'interno di un contenitore richiede autorizzazioni extra, quindi il riferimento aggiunge le capacità `NET_ADMIN` e `NET_RAW` tramite `runArgs`. Lo script del firewall e queste capacità non sono richiesti per Claude Code stesso: puoi lasciarli fuori e affidarti ai tuoi controlli di rete.

<h2 id="run-without-permission-prompts">
  Esegui senza prompt di autorizzazione
</h2>

Poiché il contenitore esegue Claude Code come utente non-root e confina l'esecuzione dei comandi al contenitore, puoi passare `--dangerously-skip-permissions` per l'operazione automatica. La CLI rifiuta questo flag quando lanciato come root, quindi conferma che `remoteUser` è impostato su un account non-root.

Saltare i prompt di autorizzazione rimuove la tua opportunità di rivedere le chiamate degli strumenti prima che vengano eseguite. Claude può comunque modificare qualsiasi file nel workspace bind-montato, che appare direttamente sul tuo host, e raggiungere qualsiasi cosa la politica di rete del contenitore consente. Abbina questo flag alle [restrizioni di uscita di rete](#restrict-network-egress) sopra per limitare ciò che una sessione bypassata può raggiungere.

Se desideri meno prompt senza disabilitare i controlli di sicurezza, considera invece la [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), che ha un classificatore che rivede le azioni prima che vengano eseguite. Per impedire agli ingegneri di utilizzare `--dangerously-skip-permissions` del tutto, imposta `permissions.disableBypassPermissionsMode` su `"disable"` nelle [impostazioni gestite](/docs/it/settings#permission-settings).

<h2 id="try-the-reference-container">
  Prova il contenitore di riferimento
</h2>

Il repository [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/.devcontainer) include un esempio di dev container che combina la CLI, il firewall di uscita, i volumi persistenti e una shell basata su Zsh. È fornito come esempio funzionante piuttosto che come immagine base mantenuta; usalo per vedere come i pezzi si incastrano insieme prima di applicarli alla tua configurazione.

<Steps>
  <Step title="Installa i prerequisiti">
    Installa VS Code e l'[estensione Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).
  </Step>

  <Step title="Clona il riferimento">
    Clona il [repository Claude Code](https://github.com/anthropics/claude-code) e aprilo in VS Code.
  </Step>

  <Step title="Riapri nel contenitore">
    Quando richiesto, fai clic su **Reopen in Container**, o esegui **Dev Containers: Reopen in Container** dal Command Palette.
  </Step>

  <Step title="Avvia Claude Code">
    Una volta che il contenitore finisce di costruire, apri un terminale con `` Ctrl+` `` ed esegui `claude` per accedere e avviare la tua prima sessione.
  </Step>
</Steps>

Per utilizzare questa configurazione con il tuo progetto, copia la directory `.devcontainer/` nel tuo repository e adatta il Dockerfile per la tua toolchain, o torna a [Aggiungi Claude Code al tuo dev container](#add-claude-code-to-your-dev-container) per aggiungere solo la feature a una configurazione che hai già.

La configurazione di riferimento è composta da tre file. Nessuno di loro è richiesto quando aggiungi Claude Code al tuo dev container tramite la feature, ma mostrano un modo per combinare i pezzi.

| File                                                                                                       | Scopo                                                                     |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| [`devcontainer.json`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) | Mount dei volumi, capacità `runArgs`, estensioni VS Code e `containerEnv` |
| [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/Dockerfile)               | Immagine base, strumenti di sviluppo e l'installazione di Claude Code     |
| [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh)   | Blocca tutto il traffico di rete in uscita tranne i domini consentiti     |

<h2 id="next-steps">
  Passaggi successivi
</h2>

Una volta che Claude Code è in esecuzione nel tuo dev container, le pagine seguenti coprono il resto di un rollout organizzativo: scegliere un percorso di autenticazione, fornire politica gestita al di fuori del repository, monitorare l'utilizzo e comprendere cosa Claude Code archivia e invia.

* [Configura Claude Code per la tua organizzazione](/docs/it/admin-setup): scegli un provider di autenticazione, decidi come la politica raggiunge i dispositivi e pianifica il rollout
* [Impostazioni gestite dal server](/docs/it/server-managed-settings): fornisci politica gestita dalla console admin di Claude.ai in modo che gli ingegneri non possano aggirarla modificando i file del repository
* [Monitora l'utilizzo e l'attività di audit](/docs/it/monitoring-usage): esporta metriche OpenTelemetry e rivedi cosa il tuo team sta eseguendo
* [Requisiti di accesso alla rete](/docs/it/network-config#network-access-requirements): l'elenco completo dei domini per proxy e firewall
* [Servizi di telemetria e opt-out](/docs/it/data-usage#telemetry-services): cosa Claude Code invia per impostazione predefinita e le variabili di ambiente che lo disabilitano
* [Esplora la directory `.claude`](/docs/it/claude-directory): cosa contiene il mount del volume, incluse credenziali, impostazioni e cronologia della sessione
* [Ambienti sandbox](/docs/it/sandbox-environments): confronta i dev container con la sandbox Bash integrata, i container personalizzati e le VM
* [Modello di sicurezza](/docs/it/security): come il sistema di autorizzazione di Claude Code, il sandboxing e le protezioni dall'iniezione di prompt si incastrano insieme
* [Modalità di autorizzazione](/docs/it/permission-modes): l'intera gamma dalla modalità piano alla modalità auto al bypass, e quando utilizzare ciascuna
