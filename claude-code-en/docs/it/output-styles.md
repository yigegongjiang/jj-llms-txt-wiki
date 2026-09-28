> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Output styles

> Cambia il ruolo, il tono e il formato di risposta di Claude Code con uno stile di output integrato come Concise o Explanatory, oppure scrivi uno stile personalizzato.

Uno stile di output è un insieme di istruzioni che imposta il ruolo, il tono e il formato di risposta di Claude per ogni risposta in una sessione. Claude Code include quattro stili integrati oltre al suo predefinito, e puoi scrivere il tuo.

Utilizza uno stile di output per cambiare il modo in cui Claude risponde e lavora con te per un'intera sessione, così non devi ripetere la richiesta in ogni prompt. Ad esempio, uno stile integrato può rendere le risposte più brevi, aggiungere una spiegazione di ogni modifica, oppure fare in modo che Claude inizi il lavoro senza fare domande di routine. Uno stile personalizzato può anche trasformare Claude in qualcosa di diverso da un ingegnere del software, come un assistente di scrittura o un analista di dati.

* Per utilizzare uno stile integrato, scegline uno dagli [stili di output integrati](#built-in-output-styles) e [passa ad esso](#change-your-output-style).
* Per scrivere le tue istruzioni, [crea uno stile di output personalizzato](#create-a-custom-output-style).

<Note>
  Uno stile di output fornisce a Claude istruzioni da seguire. Non garantisce che qualcosa accada sempre o non accada mai. Alcuni bisogni si adattano a una funzione diversa:

  * Per quello che Claude dovrebbe sapere del tuo progetto, utilizza [CLAUDE.md](/docs/it/memory).
  * Per qualcosa che deve accadere ogni volta, come la formattazione dopo ogni modifica o il blocco di un comando, utilizza un [hook](/docs/it/hooks-guide).
  * Per skills, subagents e le altre opzioni, vedi [Scegli tra uno stile di output e altre funzioni](#choose-between-an-output-style-and-other-features).
</Note>

<h2 id="built-in-output-styles">
  Output styles integrati
</h2>

Claude Code inizia nello stile [**Default**](#default), le sue istruzioni standard per completare i compiti di ingegneria del software. Ognuno degli altri quattro output styles integrati mantiene quelle istruzioni e aggiunge le proprie.

Questa tabella mostra cosa cambia ogni stile in una sessione e quando è appropriato:

| Style                       | Cosa cambia                                                                                                     | Usalo quando                                                                                                          |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| [Proactive](#proactive)     | Claude inizia il lavoro subito e fa ipotesi ragionevoli invece di chiedere informazioni su decisioni di routine | Vuoi che Claude continui a lavorare attraverso decisioni di routine, e correggerai il corso se un'ipotesi è sbagliata |
| [Concise](#concise)         | Le risposte iniziano con il risultato e omettono il preambolo, la narrazione e i riepiloghi                     | Le risposte predefinite sono più lunghe di quanto desideri                                                            |
| [Explanatory](#explanatory) | Claude aggiunge brevi blocchi `Insight` che spiegano le scelte dietro il codice che scrive                      | Stai imparando a conoscere una codebase o vuoi il ragionamento insieme alla modifica                                  |
| [Learning](#learning)       | Claude spiega le sue scelte e lascia piccoli pezzi di codice da scrivere tu stesso                              | Vuoi pratica di codifica pratica mentre il compito viene comunque completato                                          |

<h3 id="default">
  Default
</h3>

Default significa che nessuno output style è selezionato. Claude Code non aggiunge istruzioni di stile e Claude lavora dal prompt di sistema standard di Claude Code, che è scritto per i compiti di ingegneria del software.

`default` appare nell'elenco `/output-style` insieme agli altri stili, quindi lo [selezioni allo stesso modo](#change-your-output-style).

<h3 id="proactive">
  Proactive
</h3>

Nello stile Proactive, Claude inizia l'implementazione non appena invii un compito. Fa ipotesi ragionevoli su decisioni di routine piuttosto che fermarsi per chiedere, e non passa alla plan mode a meno che non chiedi un piano. Puoi reindirizzarlo in qualsiasi momento.

Le istruzioni dello stile dicono anche a Claude di verificare con te nella conversazione prima di un'azione che elimina dati o modifica un sistema condiviso o di produzione. Questo controllo è un'istruzione che Claude segue ed è separato dai prompt di permesso.

Il passaggio allo stile Proactive non cambia la tua [modalità di permesso](/docs/it/permission-modes). La tua modalità di permesso decide comunque quali chiamate di strumento vengono eseguite senza chiederti, quindi i prompt di permesso appaiono allo stesso modo di prima del passaggio.

<h3 id="concise">
  Concise
</h3>

Nello stile Concise, la prima frase di una risposta afferma cosa è successo o qual è la risposta. Claude omette l'introduzione, la narrazione passo dopo passo e il riepilogo di chiusura, e risponde a una domanda semplice in una o tre frasi. Svolge il lavoro di ingegneria in modo altrettanto approfondito dello stile Default. Richiede Claude Code v2.1.237 o successivo.

Claude scrive comunque a piena lunghezza in questi casi:

* **Qualsiasi cosa tu chieda**: quando chiedi una spiegazione o più dettagli, Claude risponde completamente.
* **Qualsiasi cosa tu abbia bisogno per agire in sicurezza**: rapporti di errore, output di test falliti, avvisi di sicurezza e conferme per azioni distruttive mantengono il loro contenuto completo.

<h3 id="explanatory">
  Explanatory
</h3>

Nello stile Explanatory, Claude svolge il compito come fa nello stile Default e aggiunge brevi spiegazioni del perché ha fatto le scelte che ha fatto. Ogni spiegazione appare nella conversazione, prima o dopo il codice di cui parla, in un blocco etichettato `Insight`. Le spiegazioni non sono scritte nei tuoi file come commenti.

Un blocco `Insight` contiene due o tre punti sulla tua codebase o sul codice che Claude ha scritto, come questo dopo l'aggiunta di un endpoint API:

```text theme={null}
★ Insight ─────────────────────────────────────
- Ogni route in questo repo passa attraverso il wrapper withAuth, quindi il nuovo endpoint ottiene i controlli di sessione senza il suo middleware.
- I limiti di velocità sono impostati per route in limits.ts, ecco perché questo cambiamento aggiunge una voce lì piuttosto che un default globale.
─────────────────────────────────────────────────
```

<h3 id="learning">
  Learning
</h3>

Nello stile Learning, Claude aggiunge gli stessi blocchi `Insight` dello [stile Explanatory](#explanatory) e ti chiede anche di scrivere parte del codice. Claude gestisce l'implementazione di routine da solo. Quando raggiunge un pezzo con una vera decisione di design, come la gestione degli errori, una struttura dati o la logica di business con più di un approccio valido, lascia alcune righe per te.

Claude contrassegna il punto con un commento `TODO(human)` nel file, quindi invia una richiesta che dice cosa è già costruito, cosa scrivere e cosa considerare:

```text theme={null}
● Learn by Doing

Context: Il modulo di caricamento è in posizione e chiama validateFile() prima di accettare un file. I controlli di dimensione e tipo funzionano per le immagini, ma l'istruzione switch non ha ancora la gestione per i documenti.

Your Task: In upload.js, implementa il ramo del caso "document" all'interno di validateFile(). Cerca TODO(human).

Guidance: Decidi un limite di dimensione per i documenti e se l'estensione del file deve corrispondere al tipo MIME. Restituisci {valid: boolean, error?: string}.
```

Claude quindi si ferma e aspetta. Scrivi il tuo codice al commento `TODO(human)` e dì a Claude quando hai finito. Claude risponde con un `Insight` sul tuo codice e continua il compito.

<h2 id="change-your-output-style">
  Cambiare il vostro output style
</h2>

Scegliete uno stile con il comando, un menu o un file di impostazioni. Il comando e entrambi i menu salvano la vostra scelta in `.claude/settings.local.json` al [livello del progetto locale](/docs/it/settings).

* **Comando `/output-style`**: eseguite `/output-style <style>` per passare a uno stile, ad esempio `/output-style concise`. Senza argomenti, il comando elenca gli stili che potete scegliere e contrassegna quello attuale.

  Il comando funziona anche in [modalità non interattiva](/docs/it/headless) e nelle sessioni Agent SDK, e dall'app mobile o dal web tramite [Remote Control](/docs/it/remote-control#limitations), dove potete elencare e selezionare solo gli [stili incorporati](#built-in-output-styles). Richiede Claude Code v2.1.269 o successivo.
* **Menu Terminal**: eseguite `/config` e selezionate **Output style** per scegliere uno stile da un menu.
* **Estensione VS Code**: aprite il [menu dei comandi](/docs/it/vs-code#use-the-prompt-box) con `/` e selezionate **Output styles** per scegliere uno stile, inclusi i vostri stili personalizzati. Richiede Claude Code v2.1.257 o successivo.
* **App Desktop**: impostate il campo `outputStyle` in un file di impostazioni, ad esempio `.claude/settings.local.json`, il file che scrive il menu del terminal. Quando eseguite `/config` lì, Claude Code [apre **Settings > Claude Code**](/docs/it/desktop#what%E2%80%99s-not-available-in-desktop) piuttosto che un menu.

Per impostare uno stile senza il menu, modificate direttamente il campo `outputStyle` in un file di impostazioni:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

Il valore è sensibile alle maiuscole, quindi scrivete i nomi incorporati come `Proactive`, `Concise`, `Explanatory` e `Learning`. Un valore che non corrisponde esattamente a un nome di stile, come `explanatory`, vi fornisce lo stile Default. Il comando `/output-style` ignora le maiuscole.

Per impostare uno stile come predefinito in tutti i progetti, impostate `outputStyle` in `~/.claude/settings.json`. I file di impostazioni di un progetto [hanno la precedenza](/docs/it/settings#settings-precedence) su quel valore.

Quando cambiate stile a metà sessione, Claude utilizza il nuovo stile a partire dal vostro messaggio successivo. Per il costo di quel primo messaggio in prompt caching, consultate [Changing output style](/docs/it/prompt-caching#changing-output-style). Prima della v2.1.251, il nuovo stile si applicava solo dopo aver eseguito `/clear` o avviato una nuova sessione.

<h2 id="create-a-custom-output-style">
  Creare uno stile di output personalizzato
</h2>

Uno stile di output personalizzato è un file Markdown: frontmatter per i metadati, quindi le istruzioni per Claude.

Nell'estensione VS Code, potete anche creare il file dal [menu **Output styles**](/docs/it/vs-code#use-the-prompt-box) piuttosto che scriverlo manualmente. Questo richiede Claude Code v2.1.261 o successivo.

<Steps>
  <Step title="Creare un file Markdown">
    Salvarlo a uno di tre livelli. Il nome del file diventa il nome dello stile a meno che non impostiate `name` nel frontmatter.

    * Utente: `~/.claude/output-styles`
    * Progetto: `.claude/output-styles`
    * Politica gestita: `.claude/output-styles` all'interno della [directory delle impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms)

    Gli output styles del progetto si caricano da ogni `.claude/output-styles/` tra la directory di lavoro e la radice del repository. Quando più di una di queste directory annidate definisce uno stile con lo stesso nome, Claude Code utilizza quello più vicino alla directory di lavoro.
  </Step>

  <Step title="Aggiungere frontmatter e istruzioni">
    Decidete se mantenere le istruzioni di ingegneria del software di Claude Code. Impostate `keep-coding-instructions: true` se state cambiando il modo in cui Claude comunica ma volete comunque che codifichi allo stesso modo. Omettete se Claude non farà ingegneria del software.

    Questo esempio introduce ogni spiegazione con un diagramma mantenendo il comportamento di codifica di Claude:

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="Passare al vostro stile">
    Eseguite `/output-style <style>` nel terminale, oppure eseguite `/config` e selezionate il vostro stile sotto **Output style**. Claude utilizza il nuovo stile a partire dal vostro messaggio successivo. Nel terminale, Claude Code legge i file di stile quando si avvia, quindi se create o modificate uno durante una sessione in esecuzione, riavviate Claude Code per acquisire la modifica.
  </Step>
</Steps>

I [Plugins](/docs/it/plugins/manifest-reference) possono anche fornire output styles in una directory `output-styles/`.

<h3 id="frontmatter">
  Riferimento frontmatter
</h3>

Configurate uno stile di output con [frontmatter](/docs/it/glossary#frontmatter) YAML tra i marcatori `---` all'inizio del file. Tutti i campi sono facoltativi e i nomi dei campi utilizzano parole minuscole separate da trattini. Un campo scritto male viene ignorato senza errore. Se lo YAML non viene analizzato, lo stile si carica comunque con il suo nome file senza campi impostati; eseguite `claude --debug` per visualizzare l'errore di analisi.

| Campo                      | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                    |
| :------------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | No           | Nome dello stile di output, mostrato nel picker `/config`. Predefinito: il nome del file                                                                                                                                                                                                                                                       |
| `description`              | No           | Descrizione dello stile di output, mostrata nel picker `/config`                                                                                                                                                                                                                                                                               |
| `keep-coding-instructions` | No           | Impostate su `true` per mantenere le istruzioni integrate di ingegneria del software di Claude Code insieme al vostro stile. Predefinito: `false`                                                                                                                                                                                              |
| `force-for-plugin`         | No           | Solo output styles dei plugin. Impostate su `true` per applicare questo stile automaticamente ogni volta che il plugin è abilitato, senza richiedere agli utenti di selezionarlo. Sostituisce l'impostazione `outputStyle` dell'utente. Se più plugin abilitati impostano questo, Claude Code utilizza il primo caricato. Predefinito: `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  Scegliere tra uno stile di output e altre funzionalità
</h2>

Uno stile di output si applica a ogni risposta in una sessione. È un'istruzione che Claude segue, quindi nulla la impone. Quando quello che desiderate è più ristretto di ogni risposta, o deve accadere senza fallo, un'altra funzionalità si adatta meglio.

Questa tabella abbina quello che desiderate alla funzionalità che lo fa:

| Desiderate                                                                                                         | Utilizzate                                                        | Perché si adatta                                                                                                             |
| :----------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| Ogni risposta in una certa voce, lunghezza o formato, o Claude in un ruolo diverso                                 | Uno stile di output                                               | Si applica all'intera sessione, e cambiate gli stili con un comando                                                          |
| Claude conosce le convenzioni, i comandi e la struttura del vostro progetto                                        | [CLAUDE.md](/docs/it/memory)                                           | Contiene quello che Claude dovrebbe sapere sulla codebase, e rimane caricato qualunque stile scegliate                       |
| Istruzioni per un tipo di attività, come una checklist di rilascio o una procedura di revisione                    | Una [skill](/docs/it/skills)                                           | Claude la carica solo quando la richiamate o l'attività corrisponde, quindi non modifica le risposte non correlate           |
| Qualcosa che accada ogni volta senza eccezione, come la formattazione dopo ogni modifica o il blocco di un comando | Un [hook](/docs/it/hooks-guide)                                        | Claude Code esegue un hook stesso a un evento del ciclo di vita, quindi non dipende da Claude che segua un'istruzione        |
| Un assistente con le proprie istruzioni, modello e strumenti per un'attività focalizzata                           | Un [subagent](/docs/it/sub-agents)                                     | Viene eseguito in un contesto separato con il proprio prompt di sistema e restituisce un riepilogo alla vostra conversazione |
| Un'aggiunta alle istruzioni di Claude che passate quando avviate Claude Code                                       | [`--append-system-prompt`](/docs/it/cli-reference#system-prompt-flags) | Si aggiunge al prompt di sistema senza rimuovere nulla                                                                       |

Queste funzionalità si combinano. Ad esempio, potete utilizzare CLAUDE.md per quello che Claude dovrebbe sapere, uno stile di output per come risponde, e un hook per qualsiasi cosa che debba essere garantita. [Estendere Claude Code](/docs/it/features-overview) confronta il resto delle funzionalità dell'estensione.

<h2 id="how-output-styles-work">
  Come funzionano gli output styles
</h2>

Un output style modifica le istruzioni che Claude Code fornisce a Claude.

* Claude Code invia le istruzioni dello style attivo con ogni richiesta.
* Gli output styles personalizzati omettono le istruzioni di ingegneria del software integrate di Claude Code, come il modo di limitare le modifiche, scrivere commenti e verificare il lavoro, a meno che `keep-coding-instructions` non sia impostato su `true`.

Gli output styles si applicano alla conversazione principale e a un [fork](/docs/it/sub-agents#fork-the-current-conversation), che eredita la conversazione completa e il prompt di sistema del genitore. Altri [subagents eseguono il loro prompt di sistema](/docs/it/sub-agents#what-loads-at-startup), quindi gli styles non cambiano il modo in cui rispondono.

L'utilizzo dei token dipende dallo style. Le istruzioni di uno style aggiungono token di input, anche se il prompt caching riduce questo costo dopo la prima richiesta in una sessione.

Gli output styles Explanatory e Learning integrati producono risposte più lunghe rispetto a Default per design, il che aumenta i token di output. Lo style Concise fa il contrario istruendo Claude a mantenere le risposte brevi per impostazione predefinita. Per gli styles personalizzati, l'utilizzo dei token di output dipende da ciò che le vostre istruzioni dicono a Claude di produrre.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Settings](/docs/it/settings): dove risiede il campo `outputStyle` e come funziona la precedenza delle impostazioni
* [Permission modes](/docs/it/permission-modes): come lo stile Proactive si confronta con la modalità auto
* [Plugins](/docs/it/plugins/overview): pacchetto e distribuzione degli output styles insieme a skills, hooks e agents
* [Debug your configuration](/docs/it/debug-your-config): diagnosticare perché uno output style non ha effetto
