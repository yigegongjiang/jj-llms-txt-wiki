> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Trova bug con ultrareview

> Esegui una revisione del codice profonda e multi-agente nel cloud con /code-review ultra per trovare e verificare i bug prima di eseguire il merge.

<Note>
  Ultrareview è una funzionalità in anteprima di ricerca. La funzionalità, i prezzi e la disponibilità possono cambiare in base al feedback. Il comando è `/code-review ultra`. Quando ultrareview è disponibile per il tuo account, `/ultrareview` è un alias.
</Note>

Ultrareview è una revisione del codice profonda che viene eseguita come una [sessione cloud](/docs/it/claude-code-on-the-web) sull'infrastruttura di Anthropic. Quando esegui `/code-review ultra`, Claude Code avvia una flotta di agenti revisori in una sandbox cloud per trovare bug nel tuo branch o pull request.

Rispetto a una `/code-review` locale, ultrareview offre:

* **Segnale più elevato**: ogni risultato segnalato viene riprodotto e verificato in modo indipendente, quindi i risultati si concentrano su bug reali piuttosto che su suggerimenti di stile
* **Copertura più ampia**: una flotta più grande di agenti revisori esplora il cambiamento in parallelo, il che fa emergere i problemi che una revisione locale può perdere
* **Nessun utilizzo di risorse locali**: la revisione viene eseguita interamente in una sandbox cloud, quindi il tuo terminale rimane libero per altri lavori mentre viene eseguita

Ultrareview richiede l'autenticazione con un account claude.ai perché viene eseguito come una sessione cloud sull'infrastruttura di Anthropic. Se sei connesso solo con una chiave API, esegui `/login` e autentica prima con claude.ai. Ultrareview non è disponibile quando si utilizza Claude Code con Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, e non è disponibile per le organizzazioni che hanno abilitato Zero Data Retention. Quando ultrareview non è disponibile, `/code-review ultra` esegue invece una revisione locale nella tua sessione.

<h2 id="run-ultrareview-from-the-cli">
  Esegui ultrareview dalla CLI
</h2>

Avviate una revisione da qualsiasi repository git:

```text theme={null}
/code-review ultra
```

Senza argomenti, ultrareview esamina il diff tra il vostro ramo attuale e il ramo predefinito, inclusi eventuali cambiamenti non committati e in staging. Per i cambiamenti non committati nei file denominati come credenziali o chiavi, come i file `.env` e `*.tfvars`, Claude Code segue le regole per [caricare un repository locale in una sessione cloud](/docs/it/claude-code-on-the-web#send-local-repositories-without-github).

Per una revisione di ramo, Claude Code raggruppa lo stato del repository e lo carica in una sandbox remota; quando [esaminate una pull request](#review-a-pull-request), Claude Code non carica nulla dalla vostra macchina.

Prima di avviare, Claude Code mostra una finestra di dialogo di conferma con l'ambito della revisione, i vostri run gratuiti rimanenti e il costo stimato; per una revisione di ramo, l'ambito include il conteggio dei file e delle righe. Dopo la conferma, la revisione continua in background mentre continuate a utilizzare la vostra sessione.

Il comando viene eseguito solo quando lo richiamate con `/code-review ultra`; Claude non avvia un ultrareview da solo.

<h3 id="review-against-a-different-base">
  Revisione rispetto a una base diversa
</h3>

Per confrontare rispetto a una base diversa dal ramo predefinito, passate il nome del ramo. Questo esempio esamina il vostro ramo attuale rispetto a `develop` invece:

```text theme={null}
/code-review ultra develop
```

Il ramo base non deve esistere nel vostro clone locale; Claude Code lo recupera da `origin`. Se il nome contiene un errore di battitura, Claude Code suggerisce il nome del ramo più simile nell'errore.

Un ID commit o un tag funziona anche come base, e la revisione copre quindi i cambiamenti sul vostro ramo da quel commit.

<h3 id="review-a-pull-request">
  Esamina una pull request
</h3>

Per esaminare una pull request di GitHub invece di un ramo locale, passate il numero della PR:

```text theme={null}
/code-review ultra 1234
```

Il comando accetta anche `#1234`, `PR 1234` e URL di PR incollati; un URL incollato deve puntare al repository nella vostra directory attuale.

In modalità PR, la sandbox remota clona la pull request direttamente dall'host piuttosto che raggruppare il vostro albero di lavoro locale. La modalità PR funziona con repository su `github.com` e su istanze di [GitHub Enterprise Server](/docs/it/github-enterprise-server) che un proprietario ha collegato a Claude Code.

Per i repository su `github.com`, la sandbox clona con l'account GitHub collegato al vostro account Claude, quindi l'account deve essere in grado di leggere il repository della PR. Claude Code verifica questo prima di creare la sessione cloud, a meno che non abbiate impostato [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars#variables), e rifiuta l'avvio quando [nessun account è collegato](/docs/it/errors#no-github-account-is-connected-to-your-claude-account) o [l'account non può vedere il repository](/docs/it/errors#your-connected-github-account-cant-see-the-repository); il rifiuto nomina la soluzione. Prima della v2.1.248, Claude Code non verificava questo prima dell'avvio.

Eseguite [`/web-setup`](/docs/it/web-quickstart#connect-from-your-terminal) per collegare il vostro login GitHub CLI al vostro account Claude.

<h3 id="post-findings-to-the-pull-request">
  Pubblica i risultati sulla pull request
</h3>

Su Claude Code v2.1.227 o successivo, quando esaminate una pull request su `github.com`, potete fare in modo che Claude pubblichi i risultati finiti sulla PR come un singolo commento semplice dal vostro account GitHub. Il commento non è una revisione o un'approvazione, e termina con una nota "Generated by Claude Code". Quando esaminate un ramo o una pull request di GitHub Enterprise Server, Claude Code mostra i risultati nella vostra sessione solo.

Claude Code non pubblica mai a meno che non lo scegliate in quella esecuzione, e `--no-post` è l'impostazione predefinita. La pubblicazione è una scelta che fate per ogni esecuzione:

* **Interattivo**: nella finestra di dialogo di avvio, selezionate **Run and post the findings to the PR as me**. Se aggiungete `--post` al comando, come in `/code-review ultra 1234 --post`, Claude Code preseleziona quella scelta e chiede comunque prima di avviare.
* **Non interattivo**: eseguite il [subcommand `claude ultrareview`](#run-ultrareview-non-interactively) con `--post`. Acconsentite alla pubblicazione eseguendo il subcommand con il flag, quindi Claude Code pubblica senza chiedere. In un'esecuzione `claude -p '/code-review ultra'`, Claude Code esce prima che i risultati arrivino, quindi non pubblica nulla; utilizzate il subcommand invece.

Claude Code non pubblica dalla vostra macchina. Invia l'ID della sessione della revisione all'API Anthropic, che pubblica i risultati archiviati della revisione come commento attraverso l'account GitHub che avete collegato a Claude. La pubblicazione richiede lo stesso accesso a claude.ai della revisione stessa, e non è disponibile su provider di terze parti o quando impostate [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars).

In una sessione interattiva, Claude Code avvia la pubblicazione quando i risultati arrivano, quindi mantenete la sessione aperta fino al completamento della revisione. Claude Code mantiene la scelta di pubblicazione solo in quella sessione. Se la sessione termina prima che la revisione finisca, Claude Code non pubblica nulla, anche se riprendete la conversazione in seguito.

Quando la pubblicazione finisce, Claude vi dice il risultato:

* **Pubblicato**: Claude vi fornisce un collegamento al commento.
* **Già pubblicato**: una pubblicazione precedente della stessa revisione ha già messo il commento sulla PR, quindi Claude vi collega alla pull request invece di pubblicare di nuovo.
* **Non riuscito**: Claude vi dice perché, e i risultati rimangono nel vostro terminale in modo che possiate pubblicarli manualmente.

<h3 id="pass-a-request-in-plain-words">
  Passa una richiesta in parole semplici
</h3>

Su Claude Code v2.1.218 o successivo, potete anche descrivere su cosa state lavorando in parole semplici:

```text theme={null}
/code-review ultra check my auth changes
```

La revisione copre comunque il vostro ramo attuale, lo stesso ambito di esecuzione senza argomenti. Claude mantiene il vostro testo come una nota, mostrata nella finestra di dialogo di avvio, e mette in relazione i risultati con essa quando arrivano.

Claude Code tratta il vostro testo come una nota solo quando ha più di una parola e non è un nome di ramo o un riferimento a PR. Legge una singola parola come un nome di ramo o un riferimento a PR, quindi un nome di ramo errato ottiene l'errore del ramo più simile da [Revisione rispetto a una base diversa](#review-against-a-different-base) invece di avviare con una nota. Se il vostro testo combina un riferimento a PR con altre parole, come `check PR 123 again`, Claude Code non avvia nemmeno; vi chiede di rieseguire con il numero della PR da solo per esaminare quella PR, o senza il riferimento per esaminare il vostro ramo attuale.

<Tip>
  Se il vostro repository è troppo grande per essere raggruppato, Claude Code vi suggerisce di utilizzare la modalità PR. Eseguite il push del vostro ramo e aprite una PR in bozza, quindi eseguite `/code-review ultra <PR-number>`.
</Tip>

<h3 id="diff-limits-and-fallbacks">
  Limiti di diff e fallback
</h3>

Ultrareview controlla il diff prima che qualsiasi lavoro di revisione venga eseguito e vi dice quando non può esaminarla così com'è:

* **Diff troppo grande**: una revisione di ramo può includere fino a 500 file modificati e 8.000 righe modificate per impostazione predefinita. I valori esatti possono cambiare, e il [rifiuto](/docs/it/errors#diff-is-too-large-for-ultrareview) nomina quelli in vigore, la dimensione del vostro diff e i file con il maggior numero di righe modificate. Claude Code rifiuta una pull request troppo grande allo stesso modo, nominando i conteggi di file e righe ma non la suddivisione per file
* **Nulla da esaminare**: quando il diff rispetto alla base è vuoto, ultrareview rifiuta e nomina il ramo o il commit rispetto al quale ha confrontato e il caso in cui vi trovate, come trovarsi sul ramo base stesso senza nulla di non committato, o un ramo i cui commit fanno già parte della base. Suggerisce anche il modo per uscire da quel caso, come passare al ramo con il vostro lavoro, mettere in staging o committare le modifiche locali, o passare una base diversa.

  Un primo commit viene esaminato completamente solo dopo quella conferma, quindi il subcommand `claude ultrareview` e `claude -p` lo rifiutano e vi indirizzano a una sessione interattiva invece. Richiede Claude Code v2.1.277 o successivo
* **Nessuna base di merge**: quando il vostro ramo non condivide alcuna cronologia con il ramo base, o il repository non ha un ramo base con cui confrontare, ultrareview esamina ogni file tracciato nel repository invece. Il fallback richiede un clone completo e applica gli stessi limiti di dimensione. Si avvia solo quando confermate nella finestra di dialogo di avvio o eseguite il subcommand `claude ultrareview` voi stessi. In `claude -p` e ovunque nessuno dei due accada, ultrareview rifiuta, dice che la revisione coprirebbe ogni file, e vi indirizza a una sessione interattiva.

  Su un checkout senza rami o altri ref, come un HEAD staccato creato controllando `FETCH_HEAD` dopo il recupero di un URL, Claude Code [rifiuta la revisione](/docs/it/errors#your-checkout-has-no-branches) e suggerisce di creare prima un ramo

<h2 id="pricing-and-free-runs">
  Prezzi e run gratuiti
</h2>

Ultrareview è una funzione premium che viene fatturata rispetto all'utilizzo extra piuttosto che all'utilizzo incluso nel vostro piano.

| Piano             | Run gratuiti inclusi | Dopo i run gratuiti                                                                                                |
| ----------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Pro               | 3 run gratuiti       | fatturato come [utilizzo extra](https://support.claude.com/it/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max               | 3 run gratuiti       | fatturato come [utilizzo extra](https://support.claude.com/it/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team e Enterprise | nessuno              | fatturato come [utilizzo extra](https://support.claude.com/it/articles/12429409-extra-usage-for-paid-claude-plans) |

* **Run gratuiti**: i tre run Pro e Max sono un'allocazione una tantum per account e non si rinnovano.
* **Costo per revisione**: dopo aver utilizzato i run gratuiti, in genere da \$5 a \$25 in crediti di utilizzo a seconda della dimensione della modifica, corrispondente alla stima che la finestra di dialogo di avvio mostra prima di ogni run.
* **Quando un run viene conteggiato**: una volta che la sessione cloud inizia. Una revisione che interrompete anticipatamente o che non riesce a completarsi utilizza comunque un run gratuito; una revisione a pagamento viene fatturata solo per la parte che è stata eseguita.

Poiché ultrareview viene sempre fatturato come crediti di utilizzo al di fuori dei run gratuiti, il vostro account o la vostra organizzazione deve avere i crediti di utilizzo abilitati prima di poter avviare una revisione a pagamento. Se i crediti di utilizzo non sono abilitati, Claude Code blocca l'avvio e come attivarli dipende dal vostro accesso alla fatturazione:

* Se potete gestire la fatturazione per il vostro account, Claude Code vi collega alle impostazioni di fatturazione dove potete attivare i crediti di utilizzo.
* Nei piani Team e Enterprise, i membri senza accesso alla fatturazione inviano una richiesta dalla CLI chiedendo al loro amministratore di attivare i crediti di utilizzo.

Potete anche eseguire `/usage-credits` per controllare o modificare l'impostazione dei crediti di utilizzo.

Claude Code vi chiede di confermare la fatturazione dei crediti di utilizzo una volta per conversazione: quando avviate una nuova conversazione, ad esempio con `/clear`, Claude Code mostra di nuovo la conferma per la prossima revisione a pagamento.

<h2 id="track-a-running-review">
  Traccia una revisione in esecuzione
</h2>

Una revisione in genere richiede da 5 a 10 minuti. La revisione viene eseguita come attività in background, quindi potete continuare a lavorare nella vostra sessione, avviare altri comandi o chiudere completamente il terminale. Se avete scelto di [pubblicare i risultati nella pull request](#post-findings-to-the-pull-request), mantenete la sessione aperta fino al termine della revisione; se la sessione termina prima, Claude Code non pubblica nulla.

Utilizzate `/tasks` per visualizzare le revisioni in esecuzione e completate, aprire la vista dettagli per una revisione o interrompere una revisione in corso. Se interrompete una revisione, Claude Code archivia la sessione cloud e non restituisce risultati parziali.

Claude può anche comunicarvi che una revisione è stata interrotta o che la sua sessione non è stata trovata:

* Se la sessione cloud della revisione viene interrotta o [archiviata](/docs/it/claude-code-on-the-web#archive-sessions) su claude.ai prima del termine della revisione, Claude vi comunica che è stata interrotta.
* Se la sessione cloud della revisione è stata eliminata, oppure avete effettuato l'accesso a un account Claude o a un'organizzazione diversa da quando l'avete avviata, Claude vi comunica che la sessione non è stata trovata.
* Se avete cambiato account, la revisione potrebbe comunque terminare con l'account che l'ha avviata. Se la revisione è ancora in esecuzione, effettuate di nuovo l'accesso come quell'account e riprendete la conversazione con `claude --resume` per riallegare la revisione.

Quando la revisione termina, Claude Code mostra i risultati verificati come notifica nella vostra sessione. Ogni risultato include la posizione del file e una spiegazione del problema in modo da poter chiedere a Claude di risolverlo direttamente.

<h2 id="run-ultrareview-non-interactively">
  Esegui ultrareview in modo non interattivo
</h2>

Utilizzate il sottocomando `claude ultrareview` per avviare un ultrareview da CI o da uno script senza una sessione interattiva. Il sottocomando avvia la stessa revisione di `/code-review ultra`, si blocca fino al completamento della revisione remota, e stampa i risultati su stdout.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Senza argomenti, il sottocomando esamina il diff tra il vostro ramo attuale e il ramo predefinito, con lo stesso [fallback dell'intero repository](#diff-limits-and-fallbacks) di `/code-review ultra` quando non esiste una base di merge. Passate un numero di PR per esaminare una pull request, o un ramo di base per esaminare il diff rispetto a esso; la [gestione del ramo di base](#review-against-a-different-base) corrisponde al comando interattivo.

Voi acconsentite al fallback dell'intero repository e al prompt di fatturazione e termini quando eseguite il sottocomando, quindi l'esecuzione inizia senza attendere l'input. L'esecuzione da parte vostra è ciò che conta come consenso. Quando Claude esegue il sottocomando per voi, ad esempio tramite lo strumento Bash, Claude Code rifiuta la revisione dell'intero repository.

Su Claude Code v2.1.218 o successivo, potete anche avviare la revisione cloud eseguendo `/code-review ultra` in una sessione non interattiva, ad esempio `claude -p '/code-review ultra'`. Claude Code avvia la revisione e stampa un link di tracciamento senza attendere i risultati, a differenza di `claude ultrareview`, che si blocca fino al loro arrivo. Quando la revisione comporterebbe la fatturazione dei crediti di utilizzo, Claude Code si ferma prima di avviare e vi indirizza a `claude ultrareview`, perché la conferma della fatturazione richiede una sessione interattiva. Prima della v2.1.218, `/code-review ultra` in una sessione non interattiva eseguiva una revisione locale.

I messaggi di progresso e l'URL della sessione live vanno a stderr in modo che stdout rimanga analizzabile. Utilizzate questi flag per controllare l'output, il timeout e se pubblicare i risultati:

| Flag                  | Descrizione                                                                                                                                                                                                                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--json`              | Stampa il payload `bugs.json` grezzo invece dei risultati formattati                                                                                                                                                                                                                                   |
| `--timeout <minutes>` | Numero massimo di minuti da attendere per il completamento della revisione. Predefinito a 45                                                                                                                                                                                                           |
| `--post`              | [Pubblica i risultati finiti](#post-findings-to-the-pull-request) sulla pull request come un unico commento semplice dal vostro account GitHub. Funziona su target di pull request `github.com`; su altri target, Claude Code ignora il flag e lo comunica. Richiede Claude Code v2.1.227 o successivo |
| `--no-post`           | Non pubblica i risultati. Questo è il predefinito, e se passate entrambi i flag, Claude Code non pubblica. Richiede Claude Code v2.1.227 o successivo                                                                                                                                                  |

L'esecuzione di `claude ultrareview` richiede la stessa autenticazione e configurazione di crediti di utilizzo di `/code-review ultra`.

Il sottocomando esce con uno di tre codici:

* **0**: la revisione si è completata, con o senza risultati
* **1**: la revisione non è riuscita ad avviarsi, la sessione cloud ha generato un errore, o il timeout è scaduto
* **130**: avete interrotto il sottocomando con Ctrl-C

Se interrompete il sottocomando, la revisione remota continua a essere eseguita; seguite l'URL della sessione stampato su stderr per guardarla nel browser.

Con `--post`, il sottocomando avvia la pubblicazione subito dopo aver stampato i risultati, e stampa il link su stderr.

* Se l'esecuzione fallisce, scade il timeout, o lo interrompete, il sottocomando non pubblica nulla.
* Se la revisione si completa ma il commento non viene pubblicato, Claude Code stampa il motivo su stderr, e i risultati rimangono su stdout in modo che possiate pubblicarli manualmente.

Per le revisioni automatiche sulle pull request di GitHub, [Code Review](/docs/it/code-review) si integra direttamente con il vostro repository e pubblica i risultati come commenti PR inline senza un passaggio CLI.

<h2 id="how-ultrareview-compares-to-/code-review">
  Come ultrareview si confronta con /code-review
</h2>

Entrambe le revisioni esaminano il codice, ma le utilizzate in diverse fasi del vostro flusso di lavoro.

|                | `/code-review`                                                      | `/code-review ultra`                                                       |
| -------------- | ------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Destinazione   | il vostro diff di lavoro, una pull request, un branch o un percorso | il vostro diff di lavoro o una pull request                                |
| Viene eseguito | localmente nella vostra sessione                                    | in una sandbox cloud                                                       |
| Profondità     | scala con l'argomento effort                                        | flotta multi-agente con verifica indipendente                              |
| Durata         | da secondi a pochi minuti                                           | circa 5-10 minuti                                                          |
| Costo          | conta verso l'utilizzo normale                                      | run gratuiti, quindi circa $5 a $25 per revisione come crediti di utilizzo |
| Migliore per   | feedback rapido durante l'iterazione                                | fiducia pre-merge su cambiamenti sostanziali                               |

Utilizzate `/code-review` per un feedback rapido mentre lavorate, oppure passate un numero di PR per esaminare una pull request di un collega prima di approvarla. Utilizzate `/code-review ultra` prima di eseguire il merge di un cambiamento sostanziale quando desiderate un passaggio più profondo che catturi i problemi che una revisione locale potrebbe perdere.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Claude Code sul web](/docs/it/claude-code-on-the-web): scopri come funzionano le sessioni cloud e le sandbox cloud
* [Gestisci i costi in modo efficace](/docs/it/costs): traccia l'utilizzo e imposta i limiti di spesa
