> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panoramica dell'Agent SDK

> Costruisci agenti AI di produzione con Claude Code come libreria

Un agente è un'applicazione che completa un'attività pianificando i propri passaggi e chiamando strumenti che leggono file, eseguono comandi o modificano codice. L'Agent SDK ti offre gli stessi strumenti, il [ciclo dell'agente](/docs/it/agent-sdk/agent-loop) e la gestione del contesto che alimentano Claude Code, programmabili in Python e TypeScript.

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Confronta l'Agent SDK con altri strumenti Claude
</h2>

L'Agent SDK, la CLI, il Client SDK e gli Managed Agents differiscono per chi esegue l'agente, cosa è incluso di default e come lo raggiungi. Trova la riga che corrisponde a come desideri costruire ed eseguire il tuo.

| Desideri                                                                                                    | Usa                                                                               | Cosa ottieni                                                                                                                                                                                                                                                                                                                                                                                                  |
| ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Incorporare l'agente di Claude Code nella tua applicazione Python o TypeScript, in un processo che gestisci | **Agent SDK**                                                                     | Una libreria che esegue il binario di Claude Code, con le [capacità](#capabilities) di Claude Code, come strumenti integrati, permessi, sessioni e hooks.                                                                                                                                                                                                                                                     |
| Fare sviluppo interattivo o eseguire attività una tantum da un terminale                                    | [**Claude Code CLI**](/docs/it/overview)                                               | L'interfaccia del terminale, costruita per l'uso interattivo quotidiano.                                                                                                                                                                                                                                                                                                                                      |
| Chiamare l'API Claude direttamente dal tuo codice                                                           | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | Accesso diretto all'API Claude da uno qualsiasi dei linguaggi del client SDK. Scrivi il tool loop da solo, oppure lascia che il [tool runner](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner) beta del client SDK lo gestisca.                                                                                                                                                     |
| Avere Anthropic ospitare l'agente, configurato tramite l'API Claude                                         | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | Un harness agente ospitato che esegue il ciclo dell'agente, con sessioni in una sandbox cloud gestita da Anthropic o una [sandbox auto-ospitata](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) sulla tua infrastruttura. Usalo dall'[SDK per il tuo linguaggio](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk), dalla CLI `ant`, o dall'API REST. |

Per guidare lo stesso ciclo dell'agente da un linguaggio diverso da Python o TypeScript, [esegui la CLI come sottoprocesso](/docs/it/headless) con il flag `-p` e `--output-format json`.

<h2 id="capabilities">
  Capacità
</h2>

Queste capacità di Claude Code sono disponibili nell'SDK:

| Capacità                     | Cosa fa                                                                                   | Scopri di più                                                                                                                                                                                                  |
| ---------------------------- | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Built-in tools               | Leggi, scrivi, modifica file, esegui comandi e cerca sul web                              | [Tools reference](/docs/it/tools-reference)                                                                                                                                                                         |
| Hooks                        | Esegui codice personalizzato in punti chiave del ciclo di vita dell'agente                | [Hooks](/docs/it/agent-sdk/hooks)                                                                                                                                                                                   |
| Subagents                    | Genera agenti specializzati per sottoattività mirate                                      | [Subagents](/docs/it/agent-sdk/subagents)                                                                                                                                                                           |
| MCP                          | Connetti strumenti e fonti di dati esterne tramite il Model Context Protocol              | [MCP](/docs/it/agent-sdk/mcp)                                                                                                                                                                                       |
| Permissions                  | Controlla quali strumenti vengono eseguiti automaticamente, quali richiedono approvazione | [Permissions](/docs/it/agent-sdk/permissions)                                                                                                                                                                       |
| Sessions                     | Mantieni il contesto tra gli scambi, riprendi o dividi in seguito                         | [Sessions](/docs/it/agent-sdk/sessions)                                                                                                                                                                             |
| Skills, commands, and memory | Carica automaticamente da `.claude/` del tuo progetto e da `~/.claude/`, come Claude Code | [Skills](/docs/it/agent-sdk/skills), [Commands](/docs/it/agent-sdk/skills#commands-in-agent-sdk-sessions), [Memory](/docs/it/agent-sdk/modifying-system-prompts), [Configuration loading](/docs/it/agent-sdk/claude-code-features) |
| Plugins                      | Pacchetto skills, agenti, hooks e server MCP, e caricali per percorso locale              | [Plugins](/docs/it/agent-sdk/plugins)                                                                                                                                                                               |

<h2 id="get-started">
  Iniziare
</h2>

Segui la [Guida rapida](/docs/it/agent-sdk/quickstart) per installare l'SDK, impostare la tua chiave API e costruire il tuo primo agente, uno che trova e corregge i bug nel codice esistente.

<Note>
  Se non precedentemente approvato, Anthropic non consente ai sviluppatori di terze parti di offrire l'accesso a claude.ai o limiti di velocità per i loro prodotti, inclusi gli agenti costruiti su Claude Agent SDK. Utilizza invece i metodi di autenticazione con chiave API descritti nella [Guida rapida](/docs/it/agent-sdk/quickstart).
</Note>

<h2 id="changelog">
  Changelog
</h2>

Visualizza il changelog completo per gli aggiornamenti dell'SDK, le correzioni di bug e le nuove funzionalità:

* **TypeScript SDK**: [visualizza CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [visualizza CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  Segnalazione di bug
</h2>

Se riscontri bug o problemi con l'Agent SDK:

* **TypeScript SDK**: [segnala i problemi su GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [segnala i problemi su GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Linee guida di branding
</h2>

Per i partner che integrano Claude Agent SDK, l'uso del branding Claude è facoltativo. Quando fai riferimento a Claude nel tuo prodotto:

**Consentito:**

* "Claude Agent", preferito per i menu a discesa
* "Claude", quando già all'interno di un menu etichettato "Agents"
* "\{YourAgentName} Powered by Claude", se hai un nome di agente esistente

**Non consentito:**

* "Claude Code" o "Claude Code Agent"
* Arte ASCII con branding Claude Code o elementi visivi che imitano Claude Code

Il tuo prodotto dovrebbe mantenere il suo proprio branding e non sembrare Claude Code o alcun prodotto Anthropic. Per domande sulla conformità del branding, contatta il [team di vendita](https://www.anthropic.com/contact-sales) di Anthropic.

<h2 id="license-and-terms">
  Licenza e termini
</h2>

L'uso di Claude Agent SDK è disciplinato dai [Termini di servizio commerciali di Anthropic](https://www.anthropic.com/legal/commercial-terms), incluso quando lo utilizzi per alimentare prodotti e servizi che metti a disposizione dei tuoi clienti e utenti finali, tranne nella misura in cui un componente o una dipendenza specifica è coperta da una licenza diversa come indicato nel file LICENSE di quel componente.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Queste risorse coprono dettagli tecnici più approfonditi e progetti di esempio per la creazione con l'Agent SDK.

* [Quickstart](/docs/it/agent-sdk/quickstart): costruisci il tuo primo agente che trova e corregge i bug
* [Guida alla migrazione](/docs/it/agent-sdk/migration-guide): migra dai pacchetti Claude Code SDK all'Agent SDK
* [Agent loop](/docs/it/agent-sdk/agent-loop): come Claude pianifica, chiama gli strumenti e decide quando un'attività è completata
* [Example agents](https://github.com/anthropics/claude-agent-sdk-demos): app demo per lo sviluppo locale
* [TypeScript SDK](/docs/it/agent-sdk/typescript): riferimento API TypeScript completo ed esempi
* [Python SDK](/docs/it/agent-sdk/python): riferimento API Python completo ed esempi
* [Agent harness design](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): come il team Claude Code utilizza i flussi di lavoro dinamici per orchestrare molti subagenti contemporaneamente
