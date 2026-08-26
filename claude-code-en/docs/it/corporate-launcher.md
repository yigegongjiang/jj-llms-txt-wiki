> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire Claude Code dietro un launcher aziendale

> Instradare i processi che Claude Code avvia dal suo binario, incluso il servizio in background e ogni sessione di agent view, attraverso un launcher obbligatorio con CLAUDE_CODE_PROCESS_WRAPPER.

Alcune organizzazioni richiedono che ogni processo su una workstation si avvii attraverso un launcher obbligatorio. Il launcher applica la sandbox, i controlli di rete o l'iniezione di credenziali da cui dipende la postura di sicurezza dell'azienda, e un binario che si avvia senza di esso è una violazione della policy.

`CLAUDE_CODE_PROCESS_WRAPPER` avvia ogni processo che Claude Code lancia dal suo binario attraverso il vostro launcher: il servizio in background, ogni sessione che ospita in [agent view](/docs/it/agent-view), e i riavvii di Claude Code dopo un aggiornamento. Impostatelo al percorso assoluto del vostro launcher, e Claude Code eseguirà il launcher con il comando di Claude Code come suoi argomenti.

Un launcher che avvolge il comando `claude` nel vostro `PATH` non può raggiungere questi processi, perché si avviano dal percorso diretto del binario senza consultare `claude`.

<Note>
  `CLAUDE_CODE_PROCESS_WRAPPER` richiede Claude Code v2.1.208 o successivo. Le versioni precedenti ignorano la variabile e avviano ogni processo senza wrapper.
</Note>

<h2 id="what-the-launcher-covers">
  Cosa copre il launcher
</h2>

Con `CLAUDE_CODE_PROCESS_WRAPPER` impostato, Claude Code avvia ognuno dei seguenti processi attraverso il vostro launcher:

* Il servizio in background che `claude agents` e le sessioni in background avviano su richiesta.
* L'host del terminale e la sessione di Claude Code all'interno di ogni riga di agent view, incluse le sessioni di standby caldo che il servizio mantiene pronte.
* Le sessioni che il servizio riavvia dopo un aggiornamento o un crash.
* Il riavvio che Claude Code esegue di se stesso per completare l'installazione di un aggiornamento, inclusa l'azione restart-for-update di agent view.

Su Windows, la variabile viene ignorata: il contratto del launcher dipende da `exec`, che Windows non supporta. Una macchina Windows con la variabile impostata esegue ogni processo senza wrapper e continua a funzionare, e l'unico segnale è un avviso nel [debug log](/docs/it/troubleshooting). Se la vostra policy del launcher copre Windows, la variabile non la soddisfa lì: contate le macchine Windows come senza wrapper quando pianificate il rollout.

<h3 id="processes-that-start-outside-the-launcher">
  Processi che si avviano al di fuori del launcher
</h3>

Tre processi non si avviano mai attraverso il launcher:

* Un [servizio in background installato](/docs/it/agent-view#the-supervisor-process): `launchd` o `systemd` avvia quel processo dal suo file di unità. `/status` e `claude daemon status` avvertono quando questo si applica, e le sessioni che il servizio genera si avviano comunque attraverso il launcher una volta che il servizio si riavvia con la variabile nelle sue impostazioni.
* Una sessione che avviate voi stessi in un terminale, che viene eseguita come l'avete invocata. Per coprire queste sessioni, mettete uno script denominato `claude` in una directory precedente su `PATH` che esegue il vostro launcher con il binario reale; non sostituite il symlink gestito. Gli auto-spawn non consultano `PATH`, quindi i due launcher non si impilano mai.
* Il primo processo di un deep link `claude-cli://`, che il gestore del protocollo del sistema operativo avvia direttamente. Tutto ciò che quella sessione avvia in background in seguito viene eseguito attraverso il launcher. Per chiudere completamente questo percorso, [impedite la registrazione del gestore](/docs/it/deep-links#registration-and-supported-platforms) con l'impostazione `disableDeepLinkRegistration`.

<h3 id="helper-process-names-in-process-monitors">
  Nomi dei processi helper nei monitor dei processi
</h3>

Con un launcher configurato, `ps` e Activity Monitor mostrano il nome del binario con versione per i processi helper in background invece delle etichette `claude bg-pty-host` e `claude bg-spare` di Claude Code, perché l'`exec` del launcher ricostruisce l'elenco degli argomenti. La ridenominazione è un effetto collaterale, non un occultamento: i processi sono altrimenti invariati, e Claude Code identifica i suoi processi dal percorso del binario, mai dal nome visualizzato.

<h2 id="set-up-the-launcher">
  Configurare il launcher
</h2>

<Steps>
  <Step title="Scrivere lo script del launcher">
    Create uno script eseguibile in un percorso assoluto, come `/opt/corp/launcher`. Claude Code lo esegue con il comando completo di Claude Code come suoi argomenti, e lo script deve terminare chiamando `exec "$@"` in modo che si sostituisca con Claude Code:

    ```bash theme={null}
    #!/bin/sh
    # La configurazione della vostra organizzazione: entrare nella sandbox, applicare
    # controlli di rete o iniettare credenziali.
    exec "$@"
    ```

    Rendetelo eseguibile con `chmod +x`. La parte di configurazione è tutto ciò che il vostro launcher deve fare prima che Claude Code venga eseguito; [il contratto del launcher](#the-launcher-contract) di seguito elenca le regole che lo script deve seguire.

    <Note>
      Se in precedenza avete sostituito il symlink `~/.local/bin/claude` con il vostro launcher, ripristinate il symlink originale nello stesso cambio. Un symlink sostituito fa sì che la prima sessione avvolta avvii il servizio in background attraverso entrambi i launcher contemporaneamente, e mette l'installazione in uno stato gestito esternamente: `/doctor` lo segnala, l'auto-update lascia il file in posizione, e la pulizia delle versioni precedenti rimane disabilitata finché l'installer non gestisce di nuovo quel percorso.
    </Note>
  </Step>

  <Step title="Impostare CLAUDE_CODE_PROCESS_WRAPPER nelle impostazioni">
    Impostate la variabile nel blocco `env` di un file di impostazioni in modo che il servizio in background staccato la erediti. Un `export` di shell non è sufficiente: il servizio in background si avvia su richiesta, sopravvive alla vostra shell e non rilegge mai i profili di shell.

    Per una macchina, aggiungetela a `~/.claude/settings.json`. Per distribuirla a ogni macchina della vostra organizzazione, mettete lo stesso blocco in [managed settings](/docs/it/permissions#managed-settings):

    ```json theme={null}
    {
      "env": {
        "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
      }
    }
    ```

    Quando più di una fonte imposta la variabile, il valore delle managed settings sovrascrive sia `~/.claude/settings.json` che un valore esportato nella shell, quindi gli utenti non possono puntare gli auto-spawn a un launcher diverso.

    Le impostazioni di progetto e locali non possono impostare questa variabile. Un file sottoposto a commit in un repository non deve essere in grado di mettere un binario davanti a ogni processo di Claude Code sulla macchina, quindi `CLAUDE_CODE_PROCESS_WRAPPER` in `.claude/settings.json` o `.claude/settings.local.json` viene ignorato, con un avviso nel [debug log](/docs/it/troubleshooting).
  </Step>

  <Step title="Riavviare il servizio in background e le vostre sessioni">
    Un servizio in background in esecuzione e qualsiasi sessione `claude` aperta leggono la variabile una volta all'avvio, quindi continuano ad avviare processi senza wrapper finché non vengono riavviati. Eseguite `claude daemon stop --any` per fermare il servizio su richiesta; il comando successivo che ne ha bisogno, come `claude agents`, avvia uno avvolto. Un [servizio installato](/docs/it/agent-view#the-supervisor-process) accetta `claude daemon stop` senza `--any`. Quindi riavviate le vostre sessioni `claude` aperte.

    Su macchine che non potete riavviare manualmente, la prima sessione avviata dopo il push delle impostazioni ritira automaticamente un servizio su richiesta senza wrapper rimasto. Una macchina dove non si avvia nessuna nuova sessione mantiene il suo servizio senza wrapper finché non lo fa, e un servizio installato ha sempre bisogno del riavvio in questo passaggio.
  </Step>

  <Step title="Verificare">
    Eseguite `/status` in una sessione: la voce Self-exec mostra il comando di avvio risolto e avverte quando il servizio in background in esecuzione non corrisponde. `claude daemon status` stampa le stesse informazioni dalla shell, incluso dopo che avete annullato l'impostazione della variabile, quando `/status` non mostra più la voce.
  </Step>
</Steps>

<h2 id="the-launcher-contract">
  Il contratto del launcher
</h2>

Quando il launcher non può essere eseguito, Claude Code rifiuta di avviare il processo invece di avviarlo senza wrapper. Su Windows, [la variabile viene ignorata](#what-the-launcher-covers) e i processi si avviano senza wrapper. Claude Code tiene lo script a queste regole:

* **Terminare con `exec "$@"`**. Un launcher che crea un fork di un figlio e esce lascia un processo Claude Code orfano che il servizio in background non può tracciare. Agent view contrassegna tale sessione come non riuscita con un messaggio che nomina il launcher, e il servizio raccoglie ciò che il launcher ha lasciato indietro.
* **Non riordinare, assorbire o anteporre argomenti.** Il primo argomento è il binario di Claude Code e tutto ciò che segue è il suo argv.
* **Passare ogni variabile di ambiente ereditata attraverso a `exec`.** Aggiungere variabili, come credenziali iniettate, va bene; eliminare quelle ereditate no.
  * I token di autenticazione per sessione, la selezione del modello e del provider, e `CLAUDE_CODE_PROCESS_WRAPPER` stesso viaggiano tutti sull'ambiente ereditato, quindi un launcher che lo ricostruisce da un elenco di autorizzazione interrompe le sessioni che avvia, e `/status` segnala una mancata corrispondenza del launcher.
  * Se il launcher deve entrare in uno spazio dei nomi o in una sandbox che ripristina l'ambiente, ri-esportate l'ambiente ereditato all'interno di esso verbatim.
* **Raggiungere `exec` entro circa tre secondi ogni volta che il launcher viene eseguito.** Un dispatch in background a freddo esegue il launcher due volte in serie prima del primo byte di output, quindi fate il lavoro lento come uno scambio di single sign-on pigrizia o da una cache.
  * Un launcher che funziona molto oltre il budget viene trattato come un avvio bloccato e riavviato.
* **Tollerare di essere invocato da dentro se stesso.** Claude Code applica il launcher a ogni auto-spawn annidato, quindi un launcher che acquisisce una risorsa esclusiva deve rilevare che la detiene già.
* **Non scrivere nel terminale prima che Claude Code si avvii.** Qualsiasi cosa stampata prima dell'`exec` viene segnalata come causa del crash se la sessione muore prima dell'inizializzazione.

<h3 id="format-of-the-claude_code_process_wrapper-value">
  Formato del valore `CLAUDE_CODE_PROCESS_WRAPPER`
</h3>

Per la maggior parte dei launcher, il valore è solo il percorso assoluto dello script, come `/opt/corp/launcher`.

Per passare al vostro launcher argomenti propri, scrivete dopo il percorso. Claude Code analizza il valore come un elenco di argomenti, non un comando di shell:

* Lo spazio bianco separa i token, e le virgolette doppie raggruppano un token che contiene spazi.
* Un valore che inizia con `[` viene letto come un array di stringhe JSON, come `["/opt/corp/launcher", "--profile", "cc"]`.
* La sintassi di shell non funziona: non c'è espansione di variabili o globbing, e un operatore non quotato come `;`, `|`, `&`, o `$(` viene rifiutato come errore di configurazione piuttosto che reinterpretato.

Quando il valore non può essere utilizzato, Claude Code rifiuta di avviare il processo interessato e [segnala il motivo](/docs/it/errors#claude_code_process_wrapper-launcher-errors).

<h2 id="relationship-to-claude_code_shell_prefix">
  Relazione con `CLAUDE_CODE_SHELL_PREFIX`
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER` avvolge i processi propri di Claude Code e passa il comando attraverso come token argv separati per il launcher a cui eseguire `exec`. [`CLAUDE_CODE_SHELL_PREFIX`](/docs/it/env-vars) avvolge i comandi di shell che Claude Code esegue per vostro conto, come le chiamate dello strumento Bash, gli hook e i comandi che avviano i server MCP stdio, e passa ognuno come una singola stringa quotata di shell in `$1` per il wrapper da rivalutare. Un launcher scritto per uno non funziona come l'altro.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Agent view](/docs/it/agent-view): le sessioni in background e il processo supervisore che il launcher copre
* [Environment variables](/docs/it/env-vars): la voce di riferimento `CLAUDE_CODE_PROCESS_WRAPPER`
* [Managed settings](/docs/it/permissions#managed-settings): consegnare il blocco `env` in tutta una flotta
* [Launcher error reference](/docs/it/errors#claude_code_process_wrapper-launcher-errors): i messaggi di rifiuto e come recuperare
