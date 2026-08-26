> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop in WSL

> Esegui sessioni Code all'interno di una distribuzione WSL 2 su Windows

Su Windows, la scheda Code può eseguire una sessione all'interno di una distribuzione WSL 2 invece che su Windows stesso. Il processo Claude Code della sessione, i suoi strumenti e git vengono tutti eseguiti all'interno della distribuzione, utilizzando la sua toolchain Linux e i percorsi Linux nativi, lo stesso ambiente che il tuo progetto utilizza.

Utilizza una sessione WSL quando il tuo repository si trova all'interno del filesystem della distribuzione. Lavorare su questi file da Windows passa attraverso un filesystem di rete, che è lento e interrompe il file watching; eseguire la sessione all'interno della distribuzione evita entrambi i problemi.

<h2 id="requirements">
  Requisiti
</h2>

* Windows 10 o 11 con [WSL 2](https://learn.microsoft.com/windows/wsl/install). WSL 1 non è supportato.
* Almeno una distribuzione installata (ad esempio, Ubuntu).
* `git` installato all'interno della distribuzione.

<h2 id="start-a-wsl-session">
  Avvia una sessione WSL
</h2>

<Steps>
  <Step title="Scegli una distribuzione">
    Avvia una nuova sessione nella scheda Code e apri il selezionatore di ambiente. Le tue distribuzioni WSL 2 installate appaiono in una sezione **WSL**. Scegline una.
  </Step>

  <Step title="Scegli una cartella">
    La sessione inizia nella directory home della distribuzione. Utilizza il selezionatore di cartelle per scegliere una cartella di progetto. La navigazione avviene all'interno della distribuzione, con percorsi Linux come `/home/you/project`.
  </Step>

  <Step title="Fidati della cartella">
    La prima sessione in una cartella mostra la finestra di dialogo di fiducia dell'area di lavoro. La fiducia viene concessa per distribuzione e cartella; fidarsi di una cartella in una distribuzione non si applica a un'altra distribuzione o allo stesso percorso su Windows.
  </Step>
</Steps>

La prima sessione in una distribuzione richiede un po' più di tempo mentre Claude si configura al suo interno. Puoi anche aprire una cartella `\\wsl.localhost\...` dal selezionatore di cartelle normale, e si riapre all'interno di quella distribuzione.

Le cartelle che hai utilizzato di recente appaiono nel selezionatore per distribuzione, quindi riconnettersi a un progetto è un solo clic.

<h2 id="what-works-in-a-wsl-session">
  Cosa funziona in una sessione WSL
</h2>

Le sessioni parallele, le chat laterali, la revisione visiva dei diff, lo stato dei branch e delle pull request, e i worktrees funzionano tutti, supportati da git e dalla toolchain all'interno della distribuzione. "Open in editor" apre VS Code connesso alla distribuzione tramite [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl).

Alcune funzionalità non sono ancora disponibili nelle sessioni WSL: il terminale integrato, i connettori e i plugin, il fork della sessione, il riquadro del browser dei file e i suggerimenti di file quando digiti `@` nel compositore.

<h2 id="managed-devices">
  Dispositivi gestiti
</h2>

Su dispositivi gestiti da un'organizzazione, le sessioni WSL potrebbero non essere disponibili. Se l'avvio della sessione non riesce con un messaggio che il dispositivo è gestito, ciò è controllato dal tuo amministratore. Amministratori: consulta [come le impostazioni raggiungono i dispositivi](/docs/it/admin-setup#decide-how-settings-reach-devices) nella guida di distribuzione.
