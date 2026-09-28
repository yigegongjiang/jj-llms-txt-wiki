> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guida rapida agli ambienti self-hosted

> Configura il tuo primo ambiente self-hosted: installa Claude Code, crea l'ambiente, avvia un runner e indirizza una sessione ad esso.

<Note>
  Gli ambienti self-hosted sono in beta pubblica sui piani Team ed Enterprise; [Disponibilità e limitazioni](/docs/it/self-hosted-environments#availability-and-limitations) copre il percorso di abilitazione. Questa pagina avvia la tua prima sessione; vedi [Ambienti self-hosted](/docs/it/self-hosted-environments) per sapere cosa sono e [Distribuisci in produzione](/docs/it/self-hosted-environments-deploy) per l'hardening e le ricette di fleet.
</Note>

Un [ambiente self-hosted](/docs/it/self-hosted-environments) esegue [sessioni cloud](/docs/it/claude-code-on-the-web) di Claude Code su infrastrutture che la tua organizzazione gestisce, eseguite da processi runner che distribuisci. Questa guida rapida configura il tuo primo, il più piccolo che funziona: un runner su un singolo host, che esegue una sessione di test. Ci sono due passaggi: [crea l'ambiente, avvia un runner e indirizza una sessione ad esso](#set-up-an-environment-and-runner), quindi [invia un messaggio a quella sessione dal tuo terminale](#send-a-follow-up-message-to-a-running-session). Ti sposterai tra due superfici: claude.ai per creare l'ambiente, controllarne lo stato e indirizzare una sessione, e un terminale sull'host per tutto ciò che il runner fa.

Alla fine avrai un ambiente nella [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), un runner che esegue il polling per il lavoro, e una sessione in esecuzione sul tuo host. Prima di connettere repository reali o sistemi interni, lavora attraverso [Distribuisci in produzione](/docs/it/self-hosted-environments-deploy), che copre il profilo di sicurezza, il controllo dell'egress, le credenziali git e l'orchestrazione.

<h2 id="prerequisites">
  Prerequisiti
</h2>

<h3 id="organization-and-roles">
  Organizzazione e ruoli
</h3>

Il lato claude.ai ha bisogno di:

* **Allow self-hosted environments** attivato da un [Owner](/docs/it/cloud-environments#organization-shared-environments) nella [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments); il pulsante **New** non appare finché non è attivato. Se non hai il ruolo, qualcuno che lo ha può creare l'ambiente e passarti il suo secret; i passaggi del runner e del terminale su questa pagina non richiedono alcun ruolo claude.ai, e dove un passaggio controlla lo stato nell'interfaccia di amministrazione, le proprie righe di log del runner ti danno lo stesso segnale.
* Una [connessione GitHub](/docs/it/claude-code-on-the-web#github-authentication-options) per la tua organizzazione, in modo che gli sviluppatori possano selezionare repository quando avviano sessioni.

<h3 id="host-and-network">
  Host e rete
</h3>

L'host del runner ha bisogno di:

* Un host o container Linux o macOS con HTTPS in uscita verso `api.anthropic.com`, verso `claude.ai` e gli host di download a cui reindirizza per il passaggio di installazione sottostante, e verso il tuo host git per il clone; la [tabella dei requisiti di rete](/docs/it/self-hosted-environments-deploy#network-requirements) ha l'elenco completo. Windows non è supportato come host runner; esegui il runner in un container Linux invece. Le workstation degli sviluppatori non sono interessate, poiché le sessioni iniziano da claude.ai in un browser.
* Un orologio sincronizzato all'ora reale, ad esempio con NTP. L'autenticazione fallisce quando l'orologio è più di cinque minuti indietro; vedi [Troubleshooting](/docs/it/self-hosted-environments-deploy#troubleshooting).

<h3 id="software-on-the-runner-host">
  Software sull'host del runner
</h3>

Installa sull'host prima di iniziare:

* **Claude Code v2.1.224 o successivo**, con uno qualsiasi dei [metodi di installazione standard](/docs/it/setup). Il runner fa parte del binario `claude` standard, e le versioni precedenti non riconoscono il subcommand `self-hosted-runner`. Il canale `latest` dell'installer nativo porta ogni release non appena viene pubblicata; il canale `stable`, il cask Homebrew `claude-code`, e i repository apt, dnf e apk stabili rimangono indietro di circa una settimana. Per fissare la versione esatta che la tua fleet esegue, vedi [Installa una versione specifica](/docs/it/setup#install-a-specific-version). Per le immagini container, vedi il Dockerfile in [Distribuisci in produzione](/docs/it/self-hosted-environments-deploy#build-the-runner-image).
* **Git 2.24 o più recente**. Alcune opzioni git nella pagina di distribuzione richiedono versioni più recenti; [Configura git](/docs/it/self-hosted-environments-deploy#configure-git) indica ogni limite.

Conferma che l'host è pronto:

```bash theme={null}
claude self-hosted-runner --help
```

Un host pronto stampa il testo di utilizzo del runner, elencando flag come `--environment-secret-file`. Sulle versioni precedenti a 2.1.224, il comando stampa l'output `claude --help` generale invece; aggiorna con `claude update` o reinstalla dal canale `latest`.

<h2 id="set-up-an-environment-and-runner">
  Configura un ambiente e un runner
</h2>

Claude Code include una configurazione guidata: una sessione Claude Code interattiva che ti guida attraverso la creazione dell'ambiente nell'interfaccia di amministrazione, avvia un runner locale con il file secret che salvi, conferma che il runner si registra, e scrive un foglio di aiuto in `./runner-setup/CHEAT-SHEET.md`. Eseguilo su una macchina dove hai effettuato l'accesso con `claude auth login` usando un account che detiene un ruolo Owner; non è disponibile con chiavi API o provider di modelli di terze parti. Su host dove una sessione interattiva non è possibile, usa i passaggi manuali sottostanti invece. Conferma che il [controllo della versione](#software-on-the-runner-host) è passato per primo: sulle versioni precedenti a 2.1.224, questo comando avvia una sessione Claude ordinaria con le parole come prompt invece della configurazione guidata. Per avviare la configurazione guidata, esegui il subcommand setup e segui i prompt:

```bash theme={null}
claude self-hosted-runner setup
```

Per configurare manualmente invece:

<Steps>
  <Step title="Crea un ambiente">
    Vai alla [pagina **Cloud environments**](https://claude.ai/admin-settings/cloud-environments) nelle impostazioni di amministrazione. Sotto **Self-hosted environments**, seleziona **New**, nomina l'ambiente, e seleziona **Create**. Nel secondo passaggio della procedura guidata, seleziona **Copy environment key** per copiare il secret dell'ambiente, che l'interfaccia di amministrazione etichetta come environment key. claude.ai mostra il secret una volta, e non puoi recuperarlo in seguito; scade 365 giorni dopo la creazione. L'ID `ccpool_...` dell'ambiente rimane visibile nella sua finestra di dialogo dei dettagli; ne avrai bisogno per il controllo `aud` nella [verifica del token](/docs/it/self-hosted-environments-identity) e per l'invio di [sessioni di test da CI](/docs/it/self-hosted-environments-testing#run-the-test-loop).

    Se perdi il secret o hai bisogno di ruotarlo, crea un nuovo secret dalla scheda **Configuration** dell'ambiente, distribuisci il nuovo secret ai tuoi runner, quindi revoca quello vecchio. I runner che detengono un secret revocato falliscono il loro prossimo poll autenticato ed escono, registrando `poll auth failed`, e il tuo orchestrator li riavvia con il nuovo secret.
  </Step>

  <Step title="Avvia un runner">
    Crea la directory del secret. Questo passaggio e il successivo richiedono root per il percorso `/etc/claude`; qualsiasi percorso che il processo runner può leggere funziona, quindi regola entrambi i comandi e il valore `--environment-secret-file` insieme se ne usi uno diverso.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    Scrivi il secret dell'ambiente in un file. Il comando sottostante legge dal tuo terminale in modo che il secret rimanga fuori dalla cronologia della shell: incolla il valore che hai copiato, premi Invio, quindi Ctrl-D, e l'`umask` della subshell rende il file leggibile solo dal suo proprietario.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    Scegli una directory di base, sostituendo `<writable-dir>` nel comando del runner sottostante con un percorso assoluto che il runner può scrivere o creare. Il runner crea la directory all'avvio, quindi controlla i repository e crea directory per sessione sotto di essa. Senza `--base-dir` usa `/workspace`, che funziona solo se quella directory esiste già ed è scrivibile o avvii il runner come root.

    Se il runner non può creare o scrivere nel percorso, esce all'avvio con un errore che nomina la directory invece di registrarsi. Vedi [Troubleshooting](/docs/it/self-hosted-environments-deploy#troubleshooting).

    Quindi avvia il runner con `--environment-secret-file` e `--base-dir`. Il runner si registra con il tuo ambiente e inizia il polling per il lavoro. Se il runner esce, riavvialo manualmente. Le distribuzioni di produzione eseguono il runner sotto un orchestrator che riavvia i runner usciti, normalmente con un filesystem fresco per ogni riavvio; [Riutilizza un checkout pre-riscaldato](/docs/it/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) copre la configurazione del disco persistente supportata.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="Verifica che il runner appaia">
    Ritorna alla [pagina **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Lo stato del tuo ambiente cambia da **No runners deployed** a **Healthy** entro pochi secondi dall'avvio del runner; apri l'ambiente e seleziona **Activity** per vedere il runner stesso.
  </Step>

  <Step title="Indirizza una sessione all'ambiente">
    Avvia una sessione su claude.ai/code e seleziona il tuo ambiente dal selettore di ambiente, dove gli ambienti self-hosted appaiono insieme a quelli ospitati da Anthropic. Il runner clona con qualsiasi credenziale git che l'host ha già, quindi scegli un repository che questo host può già clonare, o uno pubblico; le opzioni di credenziale per i repository privati in produzione sono su [Configura git](/docs/it/self-hosted-environments-deploy#configure-git). Il prossimo runner disponibile raccoglie la sessione in coda e registra `Picked up session <session-id>` insieme al suo conteggio attivo e alla capacità, in modo che tu possa confermare dall'output del runner stesso quale host ha preso la sessione. Guarda la sessione lavorare e leggi le risposte di Claude su [claude.ai/code](https://claude.ai/code). Se la sessione rimane in coda invece, vedi [Troubleshooting](/docs/it/self-hosted-environments-deploy#troubleshooting).
  </Step>
</Steps>

Il runner esce per progettazione una volta che le sue sessioni attive finiscono; vedi [Runner lifecycle](/docs/it/self-hosted-environments#runner-lifecycle). Per la produzione, distribuiscilo sotto un orchestrator che lo riavvia all'uscita. Vedi [Distribuisci in produzione](/docs/it/self-hosted-environments-deploy).

<h2 id="send-a-follow-up-message-to-a-running-session">
  Inviare un messaggio di follow-up a una sessione in esecuzione
</h2>

Una volta che una sessione è in esecuzione nel vostro ambiente, inviatele un follow-up dalla CLI `claude` su qualsiasi macchina dove siete collegati con `claude auth login`; il comando non ha bisogno di essere eseguito dalla macchina che ha avviato la sessione. Il comando invia un messaggio:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Per `<session-id>`, passate l'ID bare `session_...` o `cse_...` oppure l'URL claude.ai/code della sessione. Un invio riuscito stampa `Sent to cloud session.` con l'ID della sessione e un link di visualizzazione. I moduli di ID accettati, l'output JSON, i requisiti dell'account e della policy, e il riferimento agli errori si trovano su [Send follow-ups from the CLI](/docs/it/claude-code-on-the-web#send-follow-ups-from-the-cli), poiché il comando funziona allo stesso modo contro le sessioni ospitate da Anthropic.

<h2 id="what’s-next">
  Cosa c'è dopo
</h2>

* [Distribuisci in produzione](/docs/it/self-hosted-environments-deploy): indurire la distribuzione, controllare l'egress, configurare le credenziali git, ed eseguire la fleet sotto Kubernetes o Compose
* [Personalizza sessioni](/docs/it/self-hosted-environments-configuration): script wrapper, hook del ciclo di vita, runner on-demand, server MCP, e permessi
* [Testa end to end](/docs/it/self-hosted-environments-testing): un test di fumo CI che invia una sessione e legge le risposte di Claude
