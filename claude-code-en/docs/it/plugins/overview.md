> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panoramica dei plugin

> Comprendi cos'è un plugin Claude Code, quando ne hai bisogno invece di una skill autonoma o di un server MCP, e quale pagina leggere per installarne uno o crearne uno.

Un plugin Claude Code è una directory di skills, agenti, hooks, server MCP o altri componenti che Claude Code installa e carica come un'unica unità. La maggior parte dei plugin proviene da un marketplace, che è un catalogo che elenca i plugin e da dove recuperare ciascuno. Puoi anche caricare un plugin da una cartella che qualcuno ti fornisce, oppure [creare il tuo](/docs/it/plugins/create).

<Note>
  Se utilizzi la chat claude.ai o Cowork e non Claude Code, consulta [Plugin su claude.ai e in Cowork](https://claude.com/docs/plugins/overview).
</Note>

Per provare un plugin ora, esegui `/plugin` in una sessione di terminale Claude Code e installa uno dalla scheda **Discover**, che elenca i plugin dal marketplace ufficiale di Anthropic e da qualsiasi marketplace che hai aggiunto. Da lì:

* [Installa e gestisci i plugin](/docs/it/plugins/install): i passaggi di installazione completi, gli ambiti e altre superfici
* [Crea un plugin](/docs/it/plugins/create): crea il tuo
* [Decidi se hai bisogno di un plugin](#decide-whether-you-need-a-plugin): se un plugin è lo strumento giusto per quello che vuoi

<h2 id="understand-what-a-plugin-is">
  Comprendi cos'è un plugin
</h2>

Un plugin è una directory di componenti, solitamente con un manifest. Il manifest, un file JSON in `.claude-plugin/plugin.json`, assegna al plugin il suo nome e può aggiungere una versione, una descrizione e altri [metadati](/docs/it/plugins/manifest-reference). I componenti sono ciò che il plugin aggiunge a Claude Code, come:

* [**Skills**](/docs/it/plugins/components#skills): istruzioni `SKILL.md` che Claude carica quando rilevante, e che puoi anche eseguire come comando
* [**Agenti**](/docs/it/plugins/components#agents): definizioni di subagenti a cui Claude può delegare
* [**Hooks**](/docs/it/plugins/components#hooks): comandi che Claude Code esegue in punti del suo ciclo di vita, come dopo ogni modifica
* [**Server MCP**](/docs/it/plugins/components#mcp-servers): server di strumenti a cui Claude Code si connette mentre il plugin è abilitato

Questo diagramma mostra un plugin denominato `my-plugin` che contiene uno di ciascuno di questi componenti, e cosa ottieni da ogni file una volta che il plugin si carica.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

Per ogni tipo di componente che un plugin può contenere, con un esempio di ciascuno, consulta [Componenti del plugin](/docs/it/plugins/components). Per vedere dove si trova ogni pezzo nella directory di un plugin, utilizza l'[esploratore di plugin](/docs/it/plugins/components#explore-the-plugin-directory) su quella pagina.

<h3 id="decide-whether-you-need-a-plugin">
  Decidi se hai bisogno di un plugin
</h3>

Skills, subagenti, hooks e server MCP funzionano tutti da soli, senza un plugin. Una skill che salvi in `~/.claude/skills/`, ad esempio, è disponibile in ogni progetto sul tuo computer. Per configurarne una da sola, consulta [Skills](/docs/it/skills), [Subagenti](/docs/it/sub-agents), [Hooks](/docs/it/hooks-guide) o [MCP](/docs/it/mcp).

Utilizza un plugin quando vuoi che diverse skills, subagenti, hooks o server MCP siano confezionati come un'unica unità. Installa uno per ottenere una configurazione che qualcun altro ha costruito, con un comando e aggiornamenti dal suo marketplace. Creane uno per fornire la tua configurazione ai tuoi colleghi, installarlo in molti progetti o pubblicare versioni rilasciate.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  Cosa un plugin abilitato aggiunge alle tue sessioni
</h3>

Un plugin abilitato fa parte di ogni sessione, non solo delle sessioni in cui lo utilizzi. Questo ha alcune conseguenze che vale la pena conoscere prima di installarne uno:

* **Contesto e utilizzo**: per ogni skill, agente e comando che [Claude può invocare autonomamente](/docs/it/skills#control-who-invokes-a-skill), il nome e la descrizione sono nel contesto di Claude ad ogni turno in modo che Claude sappia che esiste. Questi token contano verso il tuo utilizzo e lasciano meno spazio nella [finestra di contesto](/docs/it/context-window) anche nelle sessioni in cui nulla dal plugin viene eseguito. Il testo completo di una skill o di un agente si carica solo quando viene utilizzato. Ciò che i server MCP del plugin aggiungono per turno segue la [ricerca di strumenti MCP](/docs/it/mcp#scale-with-mcp-tool-search).
* **Processi**: i server MCP che il plugin definisce vengono eseguiti insieme a ogni sessione in cui è abilitato, e i suoi hook si attivano ai loro eventi.
* **Autorizzazioni**: ciò che il plugin esegue, lo esegue come te. Consulta [Sicurezza e affidabilità dei plugin](/docs/it/plugins/security) per sapere cosa rivedere per primo.

Puoi controllare l'impronta di un plugin ad ogni fase:

* **Prima di installare**: apri il plugin dalla scheda **Marketplaces** in `/plugin`. I plugin nel marketplace ufficiale di Anthropic mostrano una stima di **Context cost** lì.
* **Dopo l'installazione**: [Misura il costo di un plugin](/docs/it/plugins/measure#measure-what-a-plugin-costs) mostra come leggere l'impronta di un plugin, e il gruppo **Not used recently** della scheda **Installed** elenca i plugin che potresti disattivare.
* **Per fermarlo senza disinstallarlo**: disabilita il plugin con `/plugin` o, nella tua shell, `claude plugin disable`. Consulta [Gestisci i plugin installati](/docs/it/plugins/install#manage-installed-plugins).

<h2 id="get-plugins-from-a-marketplace">
  Ottieni plugin da un marketplace
</h2>

Un marketplace è un repository o una directory con un file `.claude-plugin/marketplace.json` che elenca i plugin e da dove recuperare ciascuno. È un catalogo, non un negozio ospitato. Aggiungi un marketplace una volta, quindi installa i plugin da esso per nome, come `commit-commands@claude-plugins-official`.

<Note>
  Un marketplace di plugin non è [Claude Marketplace](https://claude.com/marketplace). Claude Marketplace è il sito web su claude.com/marketplace dove puoi sfogliare plugin, connettori, prodotti partner e partner di servizi. Non è un marketplace che aggiungi con `/plugin marketplace add`.
</Note>

Claude Code aggiunge il marketplace ufficiale di Anthropic la prima volta che avvii una sessione di terminale interattiva, a meno che una [politica gestita](/docs/it/plugins/org#allow-the-official-marketplace-and-your-own) non lo blocchi. Claude Code non aggiunge nessun altro marketplace da solo, inclusi i marketplace community e demo di Anthropic. Per distinguere i tre marketplace di Anthropic, leggi [Marketplace di Anthropic](/docs/it/plugins/anthropic-marketplaces). Per vedere cosa elenca quello ufficiale, apri la scheda **Discover** di `/plugin` in una sessione o sfoglia [Claude Marketplace](https://claude.com/marketplace/plugins).

Questo diagramma mostra il percorso da un marketplace alla tua sessione. Un marketplace elenca un plugin, installi quel plugin e Claude Code carica i suoi componenti.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagram of the marketplace path in three boxes, left to right. A marketplace, a catalog of plugins, lists a plugin. The plugin is one directory installed as a unit, holding skills, agents, hooks, MCP servers, and other components. You install the plugin into Claude Code, which loads its components." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagram of the marketplace path in three boxes, left to right. A marketplace, a catalog of plugins, lists a plugin. The plugin is one directory installed as a unit, holding skills, agents, hooks, MCP servers, and other components. You install the plugin into Claude Code, which loads its components." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[Installa e gestisci i plugin](/docs/it/plugins/install#install-a-plugin) contiene i passaggi di installazione per ogni luogo in cui esegui Claude Code. Mentre stai sviluppando un plugin, non hai bisogno di un marketplace: caricalo direttamente dalla sua cartella con `--plugin-dir`, come mostra [Sviluppa senza un marketplace](/docs/it/plugins/create#develop-without-a-marketplace).

<h3 id="make-an-installed-plugin-available-in-your-session">
  Rendi disponibile un plugin installato nella tua sessione
</h3>

Prima che un plugin che hai installato ti dia una skill che puoi eseguire, deve essere presente ad ognuno di questi livelli:

* **Impostazioni**: le tue impostazioni elencano i marketplace che hai aggiunto e i plugin che sono abilitati.
* **Disco**: `~/.claude/plugins/` contiene ciò che Claude Code ha recuperato e installato.
* **Sessione**: i plugin si caricano all'avvio, o quando [ricarichi i plugin](/docs/it/plugins/loading#check-which-stage-a-plugin-reached).

Leggi [Riferimento al caricamento dei plugin](/docs/it/plugins/loading) per le regole ad ogni livello, incluso quale file di impostazioni ha la precedenza e dove si trovano i file su disco.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Distingui i marketplace di Anthropic da quelli di terze parti
</h2>

Il nome di un marketplace lo colloca in uno di tre livelli. Claude Code accetta i nomi ufficiali e community solo per i marketplace provenienti da repository `github.com/anthropics/`:

* **Ufficiale**: marketplace con uno dei [nomi ufficiali del marketplace](/docs/it/plugins/security#official-marketplace-names) di Anthropic, incluso `claude-plugins-official` e il marketplace demo `claude-code-plugins`.
* **Community**: marketplace con uno dei nomi community di Anthropic, come `claude-community`. [Identifica i marketplace di Anthropic per nome](/docs/it/plugins/security#marketplace-tiers) li elenca.
* **Terze parti**: ogni altro marketplace. Un marketplace che il tuo collega o la tua organizzazione pubblica è di terze parti.

Indipendentemente dal livello, un plugin che installi può eseguire codice con i tuoi privilegi utente. Leggi [Sicurezza e affidabilità dei plugin](/docs/it/plugins/security) per sapere come rivedere un plugin prima di installarlo.

Attraverso [impostazioni gestite](/docs/it/settings#settings-files), un'organizzazione può consentire o bloccare marketplace, forzare l'installazione di plugin e disattivare il caricamento solo per sessione. Leggi [Gestisci i plugin per la tua organizzazione](/docs/it/plugins/org) per questi controlli.

<h2 id="understand-install-scopes">
  Comprendi gli ambiti di installazione
</h2>

Quando installi un plugin, scegli un ambito, e l'ambito decide chi il plugin è abilitato per:

* **Ambito utente**: abilitato per te in ogni progetto su questo computer
* **Ambito progetto**: abilitato per tutti coloro che lavorano in questo repository, attraverso il `.claude/settings.json` committato. Ogni collaboratore deve comunque [installarlo sulla propria macchina](/docs/it/plugins/loading#enabled-in-project-settings-but-not-installed)
* **Ambito locale**: abilitato per te solo in questo repository

Un plugin che installi a livello di ambito utente nel terminale, nelle sessioni locali dell'app desktop o nell'estensione VS Code è disponibile negli altri due su quel computer, perché tutti e tre leggono gli stessi file di impostazioni. Consulta [Scegli un ambito di installazione](/docs/it/plugins/install#choose-an-install-scope) per sapere come sceglierne uno.

Una sessione cloud, inclusa una nel browser su claude.ai/code, non carica i plugin nelle tue impostazioni locali. Per i passaggi di installazione nel terminale, VS Code e l'app desktop, e per ciò che una sessione cloud carica, consulta [Installa un plugin](/docs/it/plugins/install#install-a-plugin).

<Note>
  Lo stesso formato di plugin si installa anche su claude.ai e in Cowork, dove viene caricato un diverso set di componenti. Per quelle superfici, consulta [Plugin su claude.ai e in Cowork](https://claude.com/docs/plugins/overview) su claude.com.
</Note>

<h2 id="next-steps">
  Passaggi successivi
</h2>

La maggior parte delle persone inizia installando un plugin dal marketplace ufficiale di Anthropic, che Claude Code aggiunge la prima volta che avvii una sessione di terminale interattiva. Esegui `/plugin` in una sessione di terminale per sfogliarlo, o segui [Installa e gestisci i plugin](/docs/it/plugins/install), che copre anche l'app desktop e VS Code. Per vedere cosa c'è in quel marketplace prima di aprire Claude Code, sfoglia [Claude Marketplace](https://claude.com/marketplace/plugins) sul web.

Per creare il tuo, [Crea un plugin](/docs/it/plugins/create) inizia con una directory vuota e termina con un plugin funzionante.

Una volta che hai installato o costruito un plugin, queste pagine coprono cosa viene dopo:

* **Condividi quello che hai costruito**: [Pubblica e distribuisci un plugin](/docs/it/plugins/publish)
* **Verifica se funziona ed è utilizzato**: [Testa i plugin con evals](/docs/it/plugin-evals) e [Misura il costo e l'utilizzo dei plugin](/docs/it/plugins/measure)
* **Esegui un marketplace per il tuo team**: [Crea un marketplace](/docs/it/plugins/create-marketplace), quindi [Ospita e mantieni un marketplace](/docs/it/plugins/host-marketplace)
* **Imposta la politica dei plugin per un'organizzazione**: [Gestisci i plugin per la tua organizzazione](/docs/it/plugins/org)
* **Risolvi un problema**: [Risolvi i problemi dei plugin](/docs/it/plugins/troubleshooting)
