> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Output styles

> Adattare Claude Code per usi oltre l'ingegneria del software

Output styles cambiano il modo in cui Claude risponde, non quello che Claude sa. Modificano il prompt di sistema per impostare il ruolo, il tono e il formato di output. Utilizzatene uno quando continuate a ripetere la stessa voce o formato ad ogni turno, oppure quando desiderate che Claude agisca come qualcosa di diverso da un ingegnere del software.

Uno stile di output personalizzato aggiunge le vostre istruzioni al prompt di sistema e vi permette di scegliere se mantenere le istruzioni integrate di ingegneria del software di Claude Code. Mantenetele quando state cambiando il modo in cui Claude comunica ma state ancora codificando, come rispondere sempre con un diagramma. Omettele quando Claude non sta facendo ingegneria del software affatto, come un assistente di scrittura o un analista di dati.

Per le istruzioni relative al vostro progetto, alle convenzioni o al codebase, utilizzate [CLAUDE.md](/docs/it/memory) invece.

<h2 id="built-in-output-styles">
  Output styles integrati
</h2>

Lo **Default** output style di Claude Code è il prompt di sistema esistente, progettato per aiutarvi a completare i compiti di ingegneria del software in modo efficiente.

Ci sono tre output styles integrati aggiuntivi:

* **Proactive**: Claude esegue immediatamente, fa ipotesi ragionevoli invece di fermarsi per decisioni di routine, e preferisce l'azione alla pianificazione. Questo è una guida di esecuzione autonoma più forte di quella che [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) applica, e funziona senza cambiare la vostra modalità di permesso, quindi vedete comunque i prompt di permesso prima che gli strumenti vengano eseguiti.

* **Explanatory**: Fornisce "Insights" educativi tra l'aiuto nel completamento dei compiti di ingegneria del software. Aiuta a comprendere le scelte di implementazione e i pattern del codebase.

* **Learning**: Modalità collaborativa di apprendimento pratico in cui Claude non solo condividerà "Insights" durante la codifica, ma vi chiederà anche di contribuire con piccoli, strategici pezzi di codice voi stessi. Claude Code aggiungerà marcatori `TODO(human)` nel vostro codice per voi da implementare.

<h2 id="change-your-output-style">
  Cambiare il vostro output style
</h2>

Eseguite `/config` e selezionate **Output style** per scegliere uno stile da un menu. La vostra selezione viene salvata in `.claude/settings.local.json` al [livello del progetto locale](/docs/it/settings).

<Note>Il comando standalone `/output-style` è stato deprecato nella v2.1.73 e rimosso nella v2.1.91. Utilizzate `/config` o modificate direttamente l'impostazione `outputStyle`.</Note>

Per impostare uno stile senza il menu, modificate direttamente il campo `outputStyle` in un file di impostazioni:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

Output style è parte del prompt di sistema, che Claude Code legge una volta all'avvio della sessione. Le modifiche hanno effetto dopo `/clear` o una nuova sessione. Consultate [Come Claude Code utilizza il prompt caching](/docs/it/prompt-caching#changing-output-style) per sapere cosa fa un cambio di output style sulla cache.

<h2 id="create-a-custom-output-style">
  Creare uno stile di output personalizzato
</h2>

Uno stile di output personalizzato è un file Markdown: frontmatter per i metadati, quindi le istruzioni da aggiungere al prompt di sistema.

<Steps>
  <Step title="Creare un file Markdown">
    Salvarlo a uno di tre livelli. Il nome del file diventa il nome dello stile a meno che non impostiate `name` nel frontmatter.

    * Utente: `~/.claude/output-styles`
    * Progetto: `.claude/output-styles`
    * Politica gestita: `.claude/output-styles` all'interno della [directory delle impostazioni gestite](/docs/it/settings#settings-files)

    Gli output styles del progetto si caricano da ogni `.claude/output-styles/` tra la directory di lavoro e la radice del repository. A partire dalla v2.1.178, quando più di una di queste directory annidate definisce uno stile con lo stesso nome, Claude Code utilizza quello più vicino alla directory di lavoro.
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
    Eseguite `/config` e selezionate il vostro stile sotto **Output style**. Ha effetto dopo `/clear` o la prossima volta che avviate una sessione.
  </Step>
</Steps>

I [Plugins](/docs/it/plugins-reference) possono anche fornire output styles in una directory `output-styles/`.

<h3 id="frontmatter">
  Frontmatter
</h3>

I file di output style supportano questi campi frontmatter:

| Frontmatter                | Scopo                                                                                                                                                                                                                                                                                           | Predefinito               |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------ |
| `name`                     | Nome dell'output style, se non il nome del file                                                                                                                                                                                                                                                 | Eredita dal nome del file |
| `description`              | Descrizione dell'output style, mostrata nel picker `/config`                                                                                                                                                                                                                                    | Nessuno                   |
| `keep-coding-instructions` | Mantenere le istruzioni integrate di ingegneria del software di Claude Code                                                                                                                                                                                                                     | `false`                   |
| `force-for-plugin`         | Solo output styles dei plugin: applica questo stile automaticamente ogni volta che il plugin è abilitato, senza richiedere agli utenti di selezionarlo. Sostituisce l'impostazione `outputStyle` dell'utente. Se più plugin abilitati impostano questo, Claude Code utilizza il primo caricato. | `false`                   |

<h2 id="how-output-styles-work">
  Come funzionano gli output styles
</h2>

Gli output styles modificano direttamente il prompt di sistema di Claude Code.

* Tutti gli output styles hanno le loro istruzioni personalizzate aggiunte alla fine del prompt di sistema.
* Tutti gli output styles attivano promemoria affinché Claude aderisca alle istruzioni dell'output style durante la conversazione.
* Gli output styles personalizzati omettono le istruzioni integrate di ingegneria del software di Claude Code, come come definire l'ambito dei cambiamenti, scrivere commenti e verificare il lavoro, a meno che `keep-coding-instructions` non sia impostato a `true`.

L'utilizzo dei token dipende dallo stile. L'aggiunta di istruzioni al prompt di sistema aumenta i token di input, anche se il prompt caching riduce questo costo dopo la prima richiesta in una sessione. Gli output styles integrati Explanatory e Learning producono risposte più lunghe rispetto a Default per progettazione, il che aumenta i token di output. Per gli stili personalizzati, l'utilizzo dei token di output dipende da ciò che le vostre istruzioni dicono a Claude di produrre.

<h2 id="comparisons-to-related-features">
  Confronti con funzionalità correlate
</h2>

Diverse funzionalità personalizzano il comportamento di Claude Code. Gli output styles modificano il prompt di sistema direttamente e si applicano a ogni risposta. Gli altri aggiungono istruzioni senza cambiare il prompt di sistema predefinito, o le limitano a un compito specifico.

| Funzionalità             | Come funziona                                                        | Utilizzatela quando                                                                            |
| :----------------------- | :------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- |
| Output styles            | Modifica il prompt di sistema                                        | Desiderate un ruolo, tono o formato di risposta predefinito diverso ad ogni turno              |
| [CLAUDE.md](/docs/it/memory)  | Aggiunge un messaggio utente dopo il prompt di sistema               | Claude dovrebbe sempre conoscere le convenzioni del vostro progetto e il contesto del codebase |
| `--append-system-prompt` | Aggiunge al prompt di sistema senza rimuovere nulla                  | Desiderate un'aggiunta una tantum per una singola invocazione                                  |
| [Agents](/docs/it/sub-agents) | Esegue un subagent con il suo prompt di sistema, modello e strumenti | Desiderate un helper con ambito separato per un compito focalizzato                            |
| [Skills](/docs/it/skills)     | Carica istruzioni specifiche per compiti quando invocate o rilevanti | Avete un flusso di lavoro riutilizzabile                                                       |

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Settings](/docs/it/settings): dove risiede il campo `outputStyle` e come funziona la precedenza delle impostazioni
* [Permission modes](/docs/it/permission-modes): come lo stile Proactive si confronta con la modalità auto
* [Plugins](/docs/it/plugins): pacchetto e distribuzione degli output styles insieme a skills, hooks e agents
* [Debug your configuration](/docs/it/debug-your-config): diagnosticare perché uno output style non ha effetto
