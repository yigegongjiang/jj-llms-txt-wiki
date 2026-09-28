> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Scansiona il tuo codebase per le vulnerabilità

> Installa il plugin Claude Security per scansionare il tuo codebase alla ricerca di vulnerabilità in una sessione Claude Code e trasforma i risultati in patch che esamini e applichi.

Il plugin Claude Security esegue una scansione multi-agente delle vulnerabilità del tuo codebase all'interno di una sessione Claude Code. Un team di agenti Claude mappa la tua architettura, costruisce un modello di minaccia, ricerca le vulnerabilità e esamina indipendentemente ogni risultato prima di scrivere il rapporto. Utilizza il plugin per scansionare un intero repository o [solo un set di modifiche](#scan-only-your-changes), come il diff di un branch, il diff di una pull request o un singolo commit, quindi trasforma i risultati che scegli in patch che esamini e applichi tu stesso.

Il plugin viene eseguito localmente nella tua sessione, utilizza i modelli a cui hai accesso in Claude Code, e ogni scansione conta rispetto ai limiti di utilizzo del tuo piano. Se desideri un servizio gestito che monitori i tuoi repository, o desideri eseguire scansioni su [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5), consulta il prodotto [Claude Security](https://claude.com/product/claude-security), disponibile nel piano Enterprise. Il plugin raggiunge il codice che il prodotto gestito non può raggiungere, come i repository ospitati su GitLab o Bitbucket, o su reti che non consentono connessioni in entrata.

Il plugin è anche distinto dagli strumenti di revisione già presenti in Claude Code: il [plugin security guidance](/docs/it/security-guidance) esamina il codice mentre Claude lo scrive, [`/security-review`](/docs/it/commands#all-commands) esegue un singolo passaggio sul tuo branch, e [Code Review](/docs/it/code-review) esamina le pull request. Per capire come i livelli si sovrappongono, consulta [Come il plugin si integra con altri strumenti di sicurezza](#how-the-plugin-fits-with-other-security-tools).

<h2 id="prerequisites">
  Prerequisiti
</h2>

Per eseguire il plugin, hai bisogno di:

* Un piano a pagamento, per i [dynamic workflows](/docs/it/workflows) che la scansione utilizza per orchestrare i suoi agenti. Su Pro, attivali dalla riga Dynamic workflows in `/config`.
* Python 3.9 o successivo disponibile nel tuo `PATH` come `python3`. Verifica con `python3 --version`. Gli strumenti del plugin utilizzano solo la libreria standard di Python, quindi non viene installato nulla.
* Linux, macOS o Windows.
* Git, per le scansioni di modifiche e per trasformare i risultati in patch; questi lavori non supportano altri sistemi di controllo versione. Una scansione completa funziona in qualsiasi directory, con o senza controllo versione.

<h2 id="install-the-plugin">
  Installa il plugin
</h2>

In una sessione Claude Code, installa dal [marketplace ufficiale Anthropic](/docs/it/plugins/anthropic-marketplaces):

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

Il comando apre i dettagli del plugin, dove scegli un [ambito di installazione](/docs/it/plugins/install#install-a-plugin) per avviare l'installazione.

Se l'installazione non riesce, la soluzione dipende dal messaggio che Claude Code segnala:

* Se segnala `Marketplace "claude-plugins-official" not found`, aggiungi il marketplace con `/plugin marketplace add anthropics/claude-plugins-official`, quindi riprova l'installazione.
* Se segnala che non riesce a [trovare il plugin nel marketplace](/docs/it/plugins/install#install-a-plugin), controlla il nome del plugin per eventuali errori di digitazione.

Controlla il riepilogo dell'installazione. Se segnala `Run /reload-plugins to activate.`, consulta [Applica le modifiche del plugin senza riavviare](/docs/it/plugins/cli-reference#reload-plugins) per attivare il plugin nella tua sessione corrente.

Una volta che il plugin è attivo, sei pronto a [scansionare e correggere il tuo codebase](#scan-and-fix-your-codebase).

<h3 id="uninstall-the-plugin">
  Disinstalla il plugin
</h3>

Per rimuovere il plugin, disinstallalo dal menu `/plugin`, oppure esegui `claude plugin uninstall claude-security` nel tuo terminale.

<h2 id="scan-and-fix-your-codebase">
  Scansiona e correggi il tuo codebase
</h2>

Il plugin aggiunge un comando, `/claude-security`, che apre un menu dei suoi tre lavori: scansionare il codebase, scansionare un set di modifiche e suggerire patch. Il percorso felice esegue una scansione completa, quindi trasforma i suoi risultati in patch:

<Steps>
  <Step title="Apri il menu Claude Security">
    Esegui `/claude-security` e scegli **Scan codebase**.
  </Step>

  <Step title="Scegli cosa scansionare">
    Il plugin legge prima il tuo repository, quindi offre l'intero repository o un'area focalizzata, con il conteggio dei file e il costo relativo di ogni opzione indicati. Scegli l'intero repository, oppure rispondi "I don't know" e il plugin sceglie un valore predefinito sensato per la dimensione del tuo repository.
  </Step>

  <Step title="Conferma l'esecuzione">
    Una scansione può richiedere un po' di tempo, può utilizzare un numero significativo di token e richiede che Claude Code rimanga aperto mentre si completa. Nulla viene eseguito finché non confermi.
  </Step>

  <Step title="Leggi il rapporto">
    Mentre la scansione viene eseguita, segnala ogni fase mentre inizia, con i dettagli disponibili in [`/workflows`](/docs/it/workflows). I risultati si trovano in una directory con timestamp nel tuo repository, descritta in [Leggi i risultati della scansione](#read-the-scan-results).
  </Step>

  <Step title="Trasforma i risultati in patch">
    Esegui `/claude-security` di nuovo e scegli **Suggest patches**, quindi scegli quali risultati affrontare. Le patch esaminate si trovano nella cartella `patches/` del rapporto; [Correggi i risultati](#fix-findings) spiega come ogni patch viene costruita ed esaminata.
  </Step>

  <Step title="Applica le patch che accetti">
    Applica ogni patch dalla tua shell con `git apply`, nella sua stessa pull request. Le patch non vengono mai applicate automaticamente.
  </Step>
</Steps>

Non devi iniziare dal menu: chiedi un lavoro direttamente, come argomenti del comando, come `/claude-security scan my branch`, o in linguaggio naturale, come "scan commit abc1234". Il plugin funziona meglio in [auto mode](/docs/it/permission-modes), che consente agli agenti della scansione di procedere senza un prompt di autorizzazione ad ogni passaggio.

<h3 id="scan-only-your-changes">
  Scansiona solo le tue modifiche
</h3>

Quando il tuo branch ha commit che la base non ha, il menu `/claude-security` offre di scansionare solo quel diff, così puoi controllare un branch prima di unirlo. Puoi anche scansionare una delle tue pull request aperte, o un singolo commit chiedendolo, come "scan commit abc1234". Vengono scansionate solo le modifiche committate: esegui prima il commit o lo stash delle modifiche in corso, oppure esegui una scansione completa, che legge l'albero di lavoro.

Le scansioni di modifiche richiedono un repository git; le scansioni complete di una directory senza versione funzionano comunque. Trovare le tue pull request aperte è l'unico passaggio che raggiunge la rete, ed è offerto solo quando la tua sessione ha già il permesso di eseguire la CLI di GitHub e `gh` è connesso.

<h3 id="scope-large-repositories">
  Delimita i repository di grandi dimensioni
</h3>

Su un repository di grandi dimensioni, scansiona un'area alla volta invece dell'intero albero. Scegli uno degli ambiti focalizzati che il plugin offre, come il tuo livello API o il tuo codice di autenticazione, e l'esecuzione si dimensiona in base a quello che scegli. La sezione di copertura del rapporto indica cosa è stato e non è stato esaminato. Esegui un'altra scansione su un'area diversa in qualsiasi momento.

<h3 id="read-the-scan-results">
  Leggi i risultati della scansione
</h3>

Ogni scansione scrive i suoi risultati in una directory `CLAUDE-SECURITY-<timestamp>/` con timestamp nel tuo repository:

* **`CLAUDE-SECURITY-RESULTS.md`**: il rapporto, con l'ID di ogni risultato, come `F1`, più il suo impatto, lo scenario di exploit, la gravità, la confidenza e la raccomandazione
* **`CLAUDE-SECURITY-RESULTS.jsonl`**: gli stessi risultati in forma leggibile da macchina, un oggetto JSON per riga
* **`CLAUDE-SECURITY-RESULTS.sarif`**: gli stessi risultati come un log [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) per la scansione del codice GitHub e qualsiasi altro strumento che legge lo standard. La scansione classifica i risultati secondo le loro categorie di debolezza [CWE](https://cwe.mitre.org/)
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**: il timbro di revisione, che registra quale commit è stato scansionato, con quale sforzo, se le modifiche non sottoposte a commit facevano parte dell'albero scansionato e quanto accuratamente l'esecuzione è stata verificata, così un rapporto è sempre legato al codice che descrive. Una scansione al di fuori del controllo versione timbra `UNVERSIONED` al posto del commit

Quella directory è l'unica modifica che una scansione apporta al tuo checkout, e ha il suo `.gitignore`, così un `git add` casuale non spazza mai un rapporto in un commit. Per mantenere un rapporto nella cronologia per un audit trail, elimina quel singolo file `.gitignore` e esegui il commit della directory come qualsiasi altro.

I risultati appaiono nel rapporto solo dopo che gli agenti verificatori indipendenti li analizzano, il che mantiene i rapporti brevi e degni di lettura. Le scansioni sono non deterministiche: due scansioni dello stesso codice possono far emergere risultati diversi. Esegui scansioni regolarmente e utilizza i timbri di revisione per attribuire ogni rapporto al codice esatto e alle impostazioni che ha coperto.

<h2 id="fix-findings">
  Correggi i risultati
</h2>

Avvia il flusso di correzione scegliendo **Suggest patches** dal menu `/claude-security`, o chiedi in linguaggio naturale, come "fix finding F3", quindi scegli quali risultati dal rapporto affrontare. Le patch vengono costruite rispetto al codice sottoposto a commit, e il rapporto deve ancora descrivere il codice che hai: i risultati il cui codice è cambiato nel frattempo vengono saltati con una nota, e il plugin offre una scansione fresca invece di applicare patch da un rapporto obsoleto. Ogni patch viene redatta in una copia scratch del tuo repository, così i tuoi file sorgente rimangono intatti finché non applichi una patch tu stesso.

Prima della consegna, ogni patch viene esaminata da un agente indipendente da quello che l'ha scritta, che esegue i test del tuo progetto rispetto alla modifica quando il codice li ha e legge il diff per conto suo per qualsiasi cosa nuova che potrebbe introdurre. Una patch viene scritta solo quando quella revisione può garantire che la modifica affronta il risultato, non introduce alcuna nuova vulnerabilità e lascia il comportamento altrimenti invariato. Quando non può garantire tutti e tre, ricevi una breve nota che spiega il motivo invece di una patch.

<h3 id="patches-are-never-applied-automatically">
  Le patch non vengono mai applicate automaticamente
</h3>

Applicare una patch è sempre una tua decisione. Le patch si trovano nella cartella `patches/` del rapporto, una `F<n>.patch` per risultato con una nota accanto che spiega la modifica. Applica una dalla tua shell, o chiedi a Claude di applicarla e aprire una pull request:

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

Quando il codice con patch non ha test, la nota della patch lo dice, così sai che la sua revisione è stata eseguita senza un passaggio di test. Applica ogni patch nella sua stessa pull request così può essere esaminata e testata per conto suo.

<h2 id="how-the-plugin-fits-with-other-security-tools">
  Come il plugin si integra con altri strumenti di sicurezza
</h2>

Il plugin Claude Security è il livello di scansione profonda su richiesta in uno stack di difesa in profondità, insieme al [plugin security guidance](/docs/it/security-guidance), [`/security-review`](/docs/it/commands#all-commands), [Code Review](/docs/it/code-review), al prodotto gestito [Claude Security](https://claude.com/product/claude-security) e ai tuoi scanner esistenti:

| Fase                             | Strumento                                                                       | Cosa copre                                                                                                    |
| :------------------------------- | :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------ |
| In sessione                      | [Plugin security guidance](/docs/it/security-guidance)                               | Vulnerabilità comuni nel codice che Claude scrive, corrette nella stessa sessione                             |
| Su richiesta, singolo passaggio  | [`/security-review`](/docs/it/commands#all-commands)                                 | Un passaggio di sicurezza una tantum sul branch corrente                                                      |
| Su richiesta, scansione profonda | Plugin Claude Security                                                          | Scansione multi-agente di un repository o diff, con risultati e patch esaminati indipendentemente             |
| Su pull request                  | [Code Review](/docs/it/code-review), piani Team e Enterprise                         | Revisione multi-agente di correttezza e sicurezza con contesto completo del codebase                          |
| Gestito                          | [Claude Security](https://claude.com/product/claude-security), piano Enterprise | Scansione ospitata che monitora i repository connessi                                                         |
| In CI                            | I tuoi scanner di analisi statica e dipendenze esistenti                        | Regole specifiche del linguaggio, controlli della catena di approvvigionamento e applicazione delle politiche |

Il plugin non sostituisce i tuoi strumenti di sicurezza del codice sorgente esistenti. Eseguilo insieme all'analisi statica, alla scansione delle dipendenze e alla revisione del codice: ragiona sul tuo codice nel modo in cui farebbe un ricercatore di sicurezza umano, il che completa i controlli deterministici che questi strumenti forniscono.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

**Il menu `/claude-security` si apre con un avviso Python.** Il plugin ha bisogno di `python3` 3.9 o successivo nel tuo `PATH`. Quando non riesce a trovare `python3` affatto, il menu avverte che Claude Security non funzionerà finché uno non viene installato; quando il primo `python3` nel tuo `PATH` è più vecchio, l'avviso nomina la versione che ha trovato. Installa Python 3, o metti un `python3` più recente per primo nel tuo `PATH`, quindi avvia una nuova sessione.

**Potresti vedere un avviso "safeguards flagged this message" quando esegui la scansione su un modello Fable.** Il messaggio nomina il modello, ad esempio "I safeguards di Fable 5.1 hanno contrassegnato questo messaggio". I classificatori di sicurezza della cibersicurezza di Fable contrassegnano determinate richieste, e Claude Code riesegue una richiesta contrassegnata su un modello Opus attraverso il [fallback automatico del modello](/docs/it/model-config#automatic-model-fallback). Questo è previsto, e la scansione dovrebbe comunque completarsi con successo.

<h2 id="related-resources">
  Risorse correlate
</h2>

Per approfondire i pezzi che questa pagina tocca:

* [Plugin security guidance](/docs/it/security-guidance): cattura i problemi nel codice mentre Claude lo scrive, nella stessa sessione
* [Code Review](/docs/it/code-review): configura la revisione multi-agente al momento della PR
* [Claude Security](https://claude.com/product/claude-security): il servizio gestito che monitora i repository connessi
* [Sicurezza di Claude Code](/docs/it/security): come Claude Code affronta la fiducia, i permessi e i salvaguardie
* [Install and manage plugins](/docs/it/plugins/install): trova e installa altri plugin dal marketplace ufficiale
