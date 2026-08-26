> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Trova bug con ultrareview

> Esegui una revisione del codice profonda e multi-agente nel cloud con /code-review ultra per trovare e verificare i bug prima di eseguire il merge.

<Note>
  Ultrareview è una funzione di anteprima di ricerca. La funzione, i prezzi e la disponibilità possono cambiare in base al feedback. Il comando viene ora richiamato come `/code-review ultra`, e `/ultrareview` rimane come alias.
</Note>

Ultrareview è una revisione del codice profonda che viene eseguita su Claude Code nell'infrastruttura web. Quando eseguite `/code-review ultra`, Claude Code avvia una flotta di agenti revisori in una sandbox remota per trovare i bug nel vostro ramo o nella vostra pull request.

Rispetto a una `/code-review` locale o `/review`, ultrareview offre:

* **Segnale più elevato**: ogni risultato segnalato viene riprodotto e verificato in modo indipendente, quindi i risultati si concentrano su bug reali piuttosto che su suggerimenti di stile
* **Copertura più ampia**: una flotta più grande di agenti revisori esplora il cambiamento in parallelo, il che evidenzia i problemi che una revisione a passaggio medio potrebbe perdere
* **Nessun utilizzo di risorse locali**: la revisione viene eseguita interamente in una sandbox remota, quindi il vostro terminale rimane libero per altri lavori mentre viene eseguita

Ultrareview richiede l'autenticazione con un account Claude.ai perché viene eseguito su Claude Code nell'infrastruttura web. Se siete connessi solo con una chiave API, eseguite `/login` e autenticatevi prima con Claude.ai. Ultrareview non è disponibile quando si utilizza Claude Code con Amazon Bedrock, Google Cloud Vertex AI o Microsoft Foundry, e non è disponibile per le organizzazioni che hanno abilitato Zero Data Retention.

<h2 id="run-ultrareview-from-the-cli">
  Esegui ultrareview dalla CLI
</h2>

Avviate una revisione da qualsiasi repository git nella CLI di Claude Code.

```text theme={null}
/code-review ultra
```

Senza argomenti, ultrareview esamina il diff tra il vostro ramo attuale e il ramo predefinito, inclusi eventuali cambiamenti non committati e in staging nel vostro albero di lavoro. Claude Code raggruppa lo stato del repository e lo carica in una sandbox remota per la revisione.

Per esaminare invece una pull request di GitHub, passate il numero della PR.

```text theme={null}
/code-review ultra 1234
```

In modalità PR, la sandbox remota clona la pull request direttamente dall'host piuttosto che raggruppare il vostro albero di lavoro locale. La modalità PR funziona con repository su `github.com` e su istanze di [GitHub Enterprise Server](/docs/it/github-enterprise-server) che un amministratore ha collegato a Claude Code.

<Tip>
  Se il vostro repository è troppo grande per essere raggruppato, Claude Code vi suggerisce di utilizzare la modalità PR. Eseguite il push del vostro ramo e aprite una PR in bozza, quindi eseguite `/code-review ultra <PR-number>`.

  Se il diff della pull request è troppo grande, Claude Code rifiuta la revisione con un suggerimento di scoping prima che inizi qualsiasi lavoro di revisione.
</Tip>

Prima di avviare, Claude Code mostra una finestra di dialogo di conferma con l'ambito della revisione (incluso il conteggio dei file e delle righe quando si esamina un ramo), i vostri run gratuiti rimanenti e il costo stimato. Dopo la conferma, la revisione continua in background e potete continuare a utilizzare la vostra sessione. Il comando viene eseguito solo quando lo richiamate con `/code-review ultra`; Claude non avvia un ultrareview da solo.

<h2 id="pricing-and-free-runs">
  Prezzi e run gratuiti
</h2>

Ultrareview è una funzione premium che viene fatturata rispetto all'utilizzo extra piuttosto che all'utilizzo incluso nel vostro piano.

| Piano             | Run gratuiti inclusi | Dopo i run gratuiti                                                                                                |
| ----------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Pro               | 3 run gratuiti       | fatturato come [utilizzo extra](https://support.claude.com/it/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max               | 3 run gratuiti       | fatturato come [utilizzo extra](https://support.claude.com/it/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team e Enterprise | nessuno              | fatturato come [utilizzo extra](https://support.claude.com/it/articles/12429409-extra-usage-for-paid-claude-plans) |

I sottoscrittori Pro e Max ricevono tre run ultrareview gratuiti per provare la funzione. Questi tre run sono un'allocazione una tantum per account e non si rinnovano. Dopo averli utilizzati tutti, o dopo la scadenza del periodo di run gratuiti, ogni revisione viene fatturata all'utilizzo extra e in genere costa da \$5 a \$20 a seconda della dimensione della modifica. Un run viene conteggiato una volta che la sessione remota inizia, quindi una revisione che interrompete anticipatamente o che non riesce a completarsi utilizza comunque un run gratuito. Per una revisione a pagamento, l'utilizzo extra viene fatturato solo per la parte che è stata eseguita.

Poiché ultrareview viene sempre fatturato come utilizzo extra al di fuori dei run gratuiti, il vostro account o la vostra organizzazione deve avere l'utilizzo extra abilitato prima di poter avviare una revisione a pagamento. Se l'utilizzo extra non è abilitato, Claude Code blocca l'avvio e vi collega alle impostazioni di fatturazione dove potete attivarlo. Potete anche eseguire `/usage-credits` per controllare o modificare l'impostazione attuale.

<h2 id="track-a-running-review">
  Traccia una revisione in esecuzione
</h2>

Una revisione in genere richiede da 5 a 10 minuti. La revisione viene eseguita come attività in background, quindi potete continuare a lavorare nella vostra sessione, avviare altri comandi o chiudere completamente il terminale.

Utilizzate `/tasks` per visualizzare le revisioni in esecuzione e completate, aprire la vista dettagli per una revisione o interrompere una revisione in corso. L'interruzione di una revisione archivia la sessione cloud e i risultati parziali non vengono restituiti. Quando la revisione termina, i risultati verificati vengono visualizzati come notifica nella vostra sessione. Ogni risultato include la posizione del file e una spiegazione del problema in modo da poter chiedere a Claude di risolverlo direttamente.

<h2 id="run-ultrareview-non-interactively">
  Esegui ultrareview in modo non interattivo
</h2>

Utilizzate il sottocomando `claude ultrareview` per avviare un ultrareview da CI o da uno script senza una sessione interattiva. Il sottocomando avvia la stessa revisione di `/code-review ultra`, si blocca fino al completamento della revisione remota, stampa i risultati su stdout e esce con codice 0 in caso di successo o 1 in caso di errore.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Senza argomenti, il sottocomando esamina il diff tra il vostro ramo attuale e il ramo predefinito. Passate un numero di PR per esaminare una pull request, o passate un ramo di base per esaminare il diff rispetto a quel ramo. L'invocazione del sottocomando conta come consenso per il prompt di fatturazione e termini che il comando interattivo mostra.

I messaggi di progresso e l'URL della sessione live vanno a stderr in modo che stdout rimanga analizzabile. Utilizzate questi flag per controllare l'output e il timeout:

| Flag                  | Descrizione                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------------- |
| `--json`              | Stampa il payload `bugs.json` grezzo invece dei risultati formattati                         |
| `--timeout <minutes>` | Numero massimo di minuti da attendere per il completamento della revisione. Predefinito a 30 |

L'esecuzione di `claude ultrareview` richiede la stessa autenticazione e configurazione di utilizzo extra di `/code-review ultra`. Il sottocomando esce con codice 0 quando la revisione si completa con o senza risultati, codice 1 quando la revisione non riesce ad avviarsi, la sessione remota genera errori o il timeout scade, e codice 130 quando viene interrotto con Ctrl-C. La revisione remota continua a essere eseguita se interrompete il sottocomando; seguite l'URL della sessione stampato su stderr per guardarla nel browser.

Per le revisioni automatiche sulle pull request di GitHub, [Code Review](/docs/it/code-review) si integra direttamente con il vostro repository e pubblica i risultati come commenti PR inline senza un passaggio CLI.

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  Come ultrareview si confronta con /code-review e /review
</h2>

Tutti e tre i comandi esaminano il codice, ma si rivolgono a diverse fasi del vostro flusso di lavoro.

|                | `/code-review`                       | `/review <pr>`                                                | `/code-review ultra`                                                       |
| -------------- | ------------------------------------ | ------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Destinazione   | il vostro diff di lavoro             | una pull request di GitHub                                    | il vostro diff di lavoro o una pull request                                |
| Viene eseguito | localmente nella vostra sessione     | localmente nella vostra sessione                              | in remoto in una sandbox cloud                                             |
| Profondità     | scala con l'argomento effort         | una revisione a passaggio singolo con l'effort della sessione | flotta multi-agente con verifica indipendente                              |
| Durata         | da secondi a pochi minuti            | da secondi a pochi minuti                                     | circa 5-10 minuti                                                          |
| Costo          | conta verso l'utilizzo normale       | conta verso l'utilizzo normale                                | run gratuiti, quindi circa $5 a $20 per revisione come crediti di utilizzo |
| Migliore per   | feedback rapido durante l'iterazione | revisione della PR di un collega prima di approvarla          | fiducia pre-merge su cambiamenti sostanziali                               |

Utilizzate `/code-review` per un feedback rapido mentre lavorate. Utilizzate `/review <pr>` per esaminare una pull request nello stesso modo in cui lo fareste prima di approvarla. Utilizzate `/code-review ultra` prima di eseguire il merge di un cambiamento sostanziale quando desiderate un passaggio più profondo che catturi i problemi che una revisione locale potrebbe perdere.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Claude Code sul web](/docs/it/claude-code-on-the-web): scopri come funzionano le sessioni remote e le sandbox cloud
* [Pianifica cambiamenti complessi con ultraplan](/docs/it/ultraplan): la controparte di pianificazione di ultrareview per il lavoro di progettazione iniziale
* [Gestisci i costi in modo efficace](/docs/it/costs): traccia l'utilizzo e imposta i limiti di spesa
