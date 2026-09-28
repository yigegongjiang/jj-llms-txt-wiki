> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Misurare il costo e l'utilizzo dei plugin

> Misurare il costo in token di un plugin Claude Code, scoprire se le persone lo usano ancora e scegliere gli eventi di telemetria per domande sui plugin a livello organizzativo.

Ogni sessione in cui un plugin è abilitato include i nomi e le descrizioni delle sue skills, agents e commands nel contesto di Claude, e questi token contano rispetto all'utilizzo dell'utente indipendentemente dal fatto che il plugin venga utilizzato o meno. Questa pagina mostra come visualizzare quel numero per un plugin, come ridurlo se mantieni il plugin, e dove viene visualizzato l'utilizzo in modo da poter determinare se un plugin è ancora utilizzato.

Questa pagina è per gli autori e i manutentori di plugin. Se amministri Claude Code per un'organizzazione, [Misurare su una flotta](#measure-across-a-fleet) copre le stesse domande su ogni macchina.

<Note>
  Questi casi sono coperti su altre pagine:

  * **Testare l'affidabilità con cui il plugin cambia il comportamento di Claude**: vedi [Test plugins with evals](/docs/it/plugin-evals)
  * **Ridurre il contesto della tua sessione**: vedi [Manage installed plugins](/docs/it/plugins/install#manage-installed-plugins) e la pagina [context window](/docs/it/context-window)
</Note>

Inizia con [Misurare il costo di un plugin](#measure-what-a-plugin-costs).

<h2 id="measure-what-a-plugin-costs">
  Misurare il costo di un plugin
</h2>

Per vedere cosa aggiunge un plugin al contesto di Claude, esegui [`claude plugin details`](/docs/it/plugins/cli-reference#plugin-details) con il nome del plugin. Lo esegui nella tua shell, non al prompt di una sessione Claude Code in esecuzione. Il plugin deve essere caricato: installato, in una directory di skills, o passato con `--plugin-dir` nello stesso comando, come in `claude --plugin-dir ./formatter plugin details formatter`.

Questo esempio legge un plugin installato denominato `formatter` che ha due skills, un command, un agent, un hook e un server MCP:

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Ogni parte dell'output risponde a una domanda diversa:

* **Component inventory**: cosa Claude Code ha trovato nel plugin. I commands vengono conteggiati con le skills, quindi `format-all` appare sotto `Skills`. Gli hooks e i server MCP non ottengono una stima dei costi e nessuna riga per componente; per vedere cosa aggiungono gli strumenti MCP di un plugin, esegui `/context` in una sessione con il plugin abilitato e leggi la categoria `MCP tools`.
* **Always-on**: i token che i nomi e le descrizioni delle skills, degli agents e dei commands del plugin aggiungono a ogni sessione in cui il plugin è abilitato, indipendentemente dal fatto che qualcosa venga eseguito. Questo è il numero che ogni utente porta con sé, e quello da ridurre.
* **Per-component**: ogni riga divide una skill, un agent o un command nella sua quota always-on e nel suo costo on-invoke, che è il corpo che si carica solo quando quel componente viene eseguito. Usa la colonna always-on per trovare quale componente contribuisce di più.

<h3 id="lower-the-always-on-figure">
  Ridurre il valore always-on
</h3>

Se mantieni il plugin, questi cambiamenti riducono quello che aggiunge a ogni sessione. Se lo usi solo, le tue opzioni sono disabilitarlo o disinstallarlo; vedi [Manage installed plugins](/docs/it/plugins/install#manage-installed-plugins).

Il valore always-on conta il nome di ogni componente più la sua `description` e il frontmatter `when_to_use`. Per ridurlo:

* Accorcia le descrizioni di skills e agents.
* Dividi un plugin grande in modo che gli utenti installino solo i componenti di cui hanno bisogno.

La descrizione di una skill è anche quella che Claude abbina a una richiesta, quindi una più breve può impedire che la skill si attivi. Dopo aver ridotto le descrizioni, controlla l'attivazione con un [grader `tool_used: Skill`](/docs/it/plugin-evals#create-your-first-eval-suite) nella tua suite di eval.

Per sapere cosa contribuisce ogni tipo di componente, vedi [plugin components](/docs/it/plugins/components).

<h3 id="cost-shown-to-users-before-install">
  Costo mostrato agli utenti prima dell'installazione
</h3>

I plugin nel marketplace ufficiale mostrano il loro costo agli utenti prima dell'installazione. In `/plugin`, quando un utente sfoglia l'elenco dei plugin di un marketplace e seleziona un plugin, il riquadro dei dettagli mostra una sezione **Context cost** con una riga `Every turn:` e una riga `When invoked:`. Quando il valore always-on è 2.000 token o più, la riga `Every turn:` appare evidenziata.

Un plugin nel tuo marketplace non ha una sezione **Context cost**.

<h2 id="check-whether-a-plugin-is-used">
  Verificare se un plugin è utilizzato
</h2>

Claude Code non segnala l'utilizzo di un plugin al suo autore. L'utilizzo viene registrato sulla macchina di ogni persona che ha installato il plugin, quindi quello che puoi imparare dipende dalla tua relazione con quelle persone:

* **Amministri Claude Code per la loro organizzazione**: gli eventi OpenTelemetry e l'Analytics API contano le installazioni e le attivazioni di skills su ogni macchina. Vedi [Misurare su una flotta](#measure-across-a-fleet).
* **Sono colleghi che puoi chiedere**: il Claude Code di ogni utente mostra loro se usano ancora il plugin, in quattro posti: il pannello [`/plugin`](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor), e [`/usage`](#usage-share-in-/usage). Tutti e quattro sono comandi che l'utente esegue al prompt di Claude Code in una sessione sulla propria macchina.
* **Nessuno dei due**: non hai alcun segnale di utilizzo da Claude Code per quel plugin.

<h3 id="not-used-recently-in-/plugin">
  Non utilizzato di recente in `/plugin`
</h3>

Nella scheda **Installed** di `/plugin`, un plugin che l'utente ha installato da un marketplace si sposta sotto un'intestazione **Not used recently** una volta che non è stato utilizzato per almeno 14 giorni e 10 sessioni. I dettagli del plugin mostrano anche una riga `Last used:`. Per sapere cosa fanno gli utenti con quell'intestazione e quella riga, vedi [Find plugins you no longer use](/docs/it/plugins/install#find-plugins-you-no-longer-use).

L'intestazione **Not used recently** non appare mai per:

* Plugin caricati con `--plugin-dir` o da una directory di skills
* Plugin abilitati tramite impostazioni gestite, o montati da una [directory seed](/docs/it/plugins/org#seed-containers-and-ci)
* Plugin che includono un tema, uno stile di output, un monitor o un workflow, perché questi sono in uso senza un'attivazione tracciata

Un [language server](/docs/it/plugins/components#lsp-servers) di un plugin conta come utilizzato quando fornisce diagnostica o risponde a una richiesta di navigazione del codice, quindi un plugin LSP il cui server è attivo nelle tue sessioni non è elencato come inutilizzato.

Quando l'organizzazione dell'utente imposta [`strictKnownMarketplaces`](/docs/it/plugins/org#restrict-what-users-can-install), né l'intestazione né la riga `Last used:` appare.

<h3 id="find-skills-that-never-run">
  Trovare skills che non vengono mai eseguite
</h3>

Esegui `/skill-doctor` per vedere il costo di ogni tua skill e quanto spesso viene utilizzata. Contrassegna le skills che sono nell'elenco di skills di Claude ma non sono mai state richiamate, incluse le skills dai plugin.

In una sessione interattiva, il report si apre nella scheda **Stats** del gestore `/plugin`. Vedi [Find unused skills](/docs/it/skills#find-unused-skills) per sapere cosa copre il report e dove è disponibile.

<h3 id="unused-plugins-in-/doctor">
  Plugin inutilizzati in `/doctor`
</h3>

Il checkup `/doctor` elenca ogni skill installata dall'utente, server MCP e plugin, e consiglia di disabilitare quelli che non sono stati utilizzati. Vedi [`/doctor` in the commands reference](/docs/it/commands#all-commands).

<h3 id="usage-share-in-/usage">
  Condivisione dell'utilizzo in `/usage`
</h3>

Su un piano Pro, Max, Team o Enterprise, la suddivisione `/usage` attribuisce l'utilizzo recente a skills, subagents, plugin e server MCP come quota del totale. Vedi [Using the `/usage` command](/docs/it/costs#using-the-/usage-command).

<h2 id="measure-across-a-fleet">
  Misurare su una flotta
</h2>

Se amministri Claude Code per un'organizzazione, puoi misurare il costo e l'utilizzo dei plugin su ogni macchina da una di queste fonti:

* **OpenTelemetry events**: Claude Code esporta questi al tuo backend una volta che [configuri un esportatore](/docs/it/monitoring-usage). Vedi [OpenTelemetry events for plugin installs and use](#pick-the-opentelemetry-event-for-each-question).
* **Analytics API**: servita dai record di Anthropic, senza necessità di esportatore. Vedi [Query the Analytics API](#query-the-analytics-api).

<h3 id="pick-the-opentelemetry-event-for-each-question">
  OpenTelemetry events for plugin installs and use
</h3>

Questi eventi e attributi OpenTelemetry rispondono a ogni domanda sui plugin dal tuo backend:

| Question                                          | OpenTelemetry event or attribute                                                                                                                         |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Which plugins get installed, and from where       | [`claude_code.plugin_installed`](/docs/it/monitoring-usage#plugin-installed-event), one per install                                                           |
| Which plugins are active in how many sessions     | [`claude_code.plugin_loaded`](/docs/it/monitoring-usage#plugin-loaded-event), one per enabled plugin at session start                                         |
| Which skills activate, and which plugin owns them | [`claude_code.skill_activated`](/docs/it/monitoring-usage#skill-activated-event), with `plugin.name` and `marketplace.name` for plugin skills                 |
| What a plugin's hooks report                      | [`claude_code.hook_plugin_metrics`](/docs/it/monitoring-usage#hook-plugin-metrics-event), emitted only for hooks in official-marketplace plugins              |
| What a plugin costs in API spend                  | `plugin.name` and `marketplace.name` on the [cost counter](/docs/it/monitoring-usage#cost-counter), set when the active skill or subagent belongs to a plugin |

<h3 id="redacted-plugin-names-in-your-backend">
  Nomi di plugin redatti nel tuo backend
</h3>

I plugin dal marketplace ufficiale segnalano il loro nome di plugin e il nome del marketplace al tuo backend letteralmente. Il nome di ogni altro plugin è redatto o omesso per impostazione predefinita, incluso un plugin dal marketplace della tua organizzazione. Il [trust tier](/docs/it/plugins/security#find-plugins-in-telemetry) del plugin decide quale.

Per ottenere nomi reali su alcuni eventi, imposta la variabile di ambiente [`OTEL_LOG_TOOL_DETAILS`](/docs/it/monitoring-usage#common-configuration-variables) su `1` sulle macchine che esportano telemetria, ad esempio nel blocco `env` delle stesse [impostazioni gestite](/docs/it/monitoring-usage#administrator-configuration) che configurano l'esportatore:

| Event                                 | Default                                                                                            | With `OTEL_LOG_TOOL_DETAILS=1`                      |
| :------------------------------------ | :------------------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| `plugin_loaded`                       | `plugin.name` and `marketplace.name` are the literal string `third-party`                          | Real names                                          |
| `plugin_installed`, `skill_activated` | `plugin.name` and `marketplace.name` omitted; on `skill_activated`, `skill.name` is `custom_skill` | Real names                                          |
| Cost counter                          | `plugin.name` is `third-party`; `marketplace.name` absent                                          | Real `plugin.name`; `marketplace.name` still absent |

Su `plugin_loaded`, `plugin_id_hash` identifica ancora ogni plugin per impostazione predefinita, quindi puoi contare i plugin di terze parti distinti.

<h3 id="query-the-analytics-api">
  Query the Analytics API
</h3>

Sul piano Enterprise, l'Analytics API risponde a "quali plugin installa e richiama la mia organizzazione" dai record di Anthropic, senza necessità di esportatore. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) restituisce conteggi di installazione e attivazione per plugin, per giorno, su Claude Code e Cowork, che puoi raggruppare per utente, gruppo RBAC o prodotto.

L'attività del plugin che raggiunge Anthropic senza un nome di plugin appare in una riga aggregata `third-party`. [Find plugins in telemetry](/docs/it/plugins/security#find-plugins-in-telemetry) dice quali plugin Claude Code segnala per nome.

Autentica la richiesta con una chiave API che ha lo scope `read:analytics`, che un Primary Owner crea come descritto in [Access data programmatically](/docs/it/analytics#access-data-programmatically).

Vedi il [riferimento dell'endpoint](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) per i parametri e i campi di risposta.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Test plugins with evals](/docs/it/plugin-evals): misurare l'affidabilità con cui il plugin guida Claude, non solo quello che costa
* [Ridurre il valore always-on](#lower-the-always-on-figure): cosa cambiare nel plugin per ridurre il suo costo per turno
* [Plugin security and trust](/docs/it/plugins/security#find-plugins-in-telemetry): quali campi di telemetria portano nomi di plugin e quando sono redatti
* [Monitoring usage](/docs/it/monitoring-usage): il riferimento completo degli eventi OpenTelemetry
