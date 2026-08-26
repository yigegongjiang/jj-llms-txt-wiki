> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modifica dei system prompt

> Scegli tra il preset `claude_code` e un system prompt personalizzato, e personalizza il comportamento con CLAUDE.md, stili di output, append, o un prompt completamente personalizzato.

I system prompt definiscono il comportamento, le capacità e lo stile di risposta di Claude. Inizia dal preset `claude_code` per strumenti di codifica simili a CLI o IDE dove un utente osserva e guida il lavoro. Scrivi il tuo prompt per agenti con una superficie, identità o modello di autorizzazione diverso.

Questa pagina copre:

* [Come funzionano i system prompt](#how-system-prompts-work), con una tabella decisionale per scegliere tra il preset, il preset con `append`, e un prompt personalizzato
* [Personalizza il comportamento dell'agente](#customize-agent-behavior) con file CLAUDE.md, stili di output, `append`, o una stringa personalizzata
* [Confronta i quattro approcci](#compare-the-four-approaches) per persistenza, ambito, e cosa preservano
* [Combina gli approcci](#combine-approaches) per stratificare i metodi di personalizzazione insieme

<h2 id="how-system-prompts-work">
  Come funzionano i system prompt
</h2>

Un system prompt è l'insieme iniziale di istruzioni che modella il comportamento di Claude durante una conversazione. Agent SDK ha tre punti di partenza per esso:

* **Default minimalista**: quando non impostate `systemPrompt` in TypeScript o `system_prompt` in Python, l'SDK utilizza un prompt minimalista che copre la chiamata degli strumenti ma omette le linee guida di codifica di Claude Code, lo stile di risposta e il contesto del progetto. Questo differisce da `claude -p`, che utilizza il prompt completo di Claude Code per impostazione predefinita. Se state migrando dalla CLI e desiderate un comportamento corrispondente, impostate il preset `claude_code`.
* **Preset `claude_code`**: il system prompt completo che utilizza la CLI di Claude Code, con istruzioni di utilizzo degli strumenti, linee guida di stile e formattazione del codice, regole di tono e verbosità della risposta, istruzioni di sicurezza e protezione, e contesto sulla directory di lavoro e sull'ambiente. Impostate `systemPrompt: { type: "preset", preset: "claude_code" }` in TypeScript o `system_prompt={"type": "preset", "preset": "claude_code"}` in Python, facoltativamente con `append` per aggiungere le vostre istruzioni alla fine.
* **Stringa personalizzata**: un prompt che scrivete voi stessi. L'SDK invia solo ciò che fornite.

<h3 id="decide-on-a-starting-point">
  Decidere un punto di partenza
</h3>

Il fattore decisivo è quanto strettamente il vostro agente assomiglia a Claude Code: un agente di codifica che opera in un repository, con un umano che osserva l'output in streaming e guida il lavoro. Più il vostro prodotto si allontana da questo, più vorrete scrivere il vostro prompt.

| State costruendo                                                                                                                   | Utilizzate                        | Cosa ottenete                                                                                                                                                         |
| :--------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Uno strumento di codifica simile a CLI o IDE dove un umano osserva e guida, e i default di Claude Code sono quello che desiderate  | Preset `claude_code`              | Il prompt completo di Claude Code: guida degli strumenti, regole di sicurezza, risposte compatibili con il terminale, consapevolezza delle convenzioni del repository |
| Lo stesso tipo di strumento, più regole specifiche del prodotto come standard di codifica, formato di output o contesto di dominio | Preset `claude_code` con `append` | Tutto quanto sopra, con le vostre istruzioni aggiunte dopo il preset. Nulla viene rimosso, quindi questa è la personalizzazione a rischio più basso                   |
| Un agente con una superficie diversa, un'identità diversa o un modello di permessi diverso, o un agente non di codifica            | Stringa di prompt personalizzata  | Solo quello che scrivete. Siete responsabili della sostituzione della guida degli strumenti e delle istruzioni di sicurezza di cui il vostro agente ha ancora bisogno |
| Un ciclo di chiamata degli strumenti sottile senza persona dell'agente, dove fornite tutto il comportamento nel prompt dell'utente | Nessuna opzione `systemPrompt`    | Il default minimalista: supporto per la chiamata degli strumenti e nient'altro                                                                                        |

"Diverso da Claude Code" di solito significa uno dei seguenti:

* **Superficie diversa**: l'output non viene letto in un terminale dalla persona che lo ha attivato. Le interfacce chat, i consumatori di output strutturato e l'automazione non di codifica hanno ciascuno bisogno di un prompt che corrisponda a come il loro output viene renderizzato e revisionato. L'automazione di codifica incustodita, come un lavoro CI che corregge gli errori di lint o esamina i diff, si adatta comunque al preset perché il lavoro stesso è quello per cui il preset è scritto.
* **Identità diversa**: l'agente non dovrebbe presentarsi come Claude Code. Un bot di supporto, un assistente di analisi dei dati, o qualsiasi agente specifico del dominio ha bisogno del suo proprio nome, ambito e persona.
* **Modello di permessi diverso**: l'agente viene eseguito autonomamente senza che un umano approvi ogni passaggio, o opera su un insieme ristretto di risorse. Il prompt di Claude Code presuppone che un umano sia nel ciclo con accesso a un set di strumenti completo.
* **Attività non di codifica**: la maggior parte del prompt di Claude Code è guida di codifica. Per agenti di ricerca, contenuto o operazioni, quella guida compete con le istruzioni di cui avete effettivamente bisogno.

La [tabella di confronto](#compare-the-four-approaches) mostra cosa preserva ogni metodo di personalizzazione.

<h2 id="customize-agent-behavior">
  Personalizzare il comportamento dell'agente
</h2>

Gli stili di output, `append`, e una stringa di prompt personalizzata modificano direttamente il system prompt. CLAUDE.md segue un percorso diverso: l'SDK lo legge e inietta il suo contenuto nella conversazione come contesto del progetto, non nel system prompt, quindi modella il comportamento insieme a qualsiasi system prompt Lei scelga. [Skills](/docs/it/agent-sdk/skills), [hooks](/docs/it/agent-sdk/hooks), e [permissions](/docs/it/agent-sdk/permissions) modellano anche il comportamento al di fuori del system prompt e sono trattati in pagine separate.

<h3 id="claude-md-files-for-project-level-instructions">
  File CLAUDE.md per istruzioni a livello di progetto
</h3>

I file CLAUDE.md forniscono a Claude contesto e istruzioni persistenti a livello di progetto. L'SDK inietta il loro contenuto nella conversazione, non nel system prompt, quindi funzionano con qualsiasi configurazione di system prompt. Per sapere cosa mettere in CLAUDE.md, dove posizionarlo e come scrivere istruzioni efficaci, vedi [Come Claude ricorda il tuo progetto](/docs/it/memory). Questa sezione copre ciò che è specifico dell'SDK: come CLAUDE.md si carica.

L'SDK legge CLAUDE.md quando la corrispondente fonte di impostazione è abilitata: `'project'` carica `CLAUDE.md` o `.claude/CLAUDE.md` dalla directory di lavoro, e `'user'` carica `~/.claude/CLAUDE.md`. Le opzioni predefinite di `query()` abilitano entrambe le fonti, quindi CLAUDE.md si carica automaticamente. Se impostate `settingSources` in TypeScript o `setting_sources` in Python esplicitamente, includete le fonti di cui avete bisogno. Il caricamento di CLAUDE.md è controllato dalle fonti di impostazione, non dal preset `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Caricare CLAUDE.md con l'SDK
</h4>

Per caricare CLAUDE.md, impostate `settingSources` per includere il livello in cui vive il vostro CLAUDE.md. L'esempio seguente carica un CLAUDE.md a livello di progetto insieme al preset `claude_code`, quindi Claude ha sia il prompt completo dell'agente di codifica che le convenzioni del vostro progetto:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []

  async for message in query(
      prompt="Add a new React component for user profiles",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",  # Use Claude Code's system prompt
          },
          setting_sources=["project"],  # Loads CLAUDE.md from project
      ),
  ):
      messages.append(message)

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

CLAUDE.md è persistente in tutte le sessioni di un progetto, condiviso con il vostro team tramite git, e scoperto automaticamente senza modifiche al codice. Non viene caricato se passate un array `settingSources` vuoto.

<h3 id="output-styles-for-persistent-configurations">
  Stili di output per configurazioni persistenti
</h3>

Gli stili di output sono configurazioni salvate che modificano il system prompt di Claude. Vengono archiviati come file markdown e possono essere riutilizzati in sessioni e progetti diversi.

<h4 id="create-an-output-style">
  Creare uno stile di output
</h4>

Uno stile di output è un file markdown con [frontmatter](/docs/it/output-styles#frontmatter) per i metadati, seguito dal contenuto del prompt. Salvatelo in `~/.claude/output-styles/` per uno stile a livello di utente disponibile in ogni progetto, o `.claude/output-styles/` nel vostro repository per uno stile a livello di progetto che potete committare e condividere con il vostro team.

Per impostazione predefinita, uno stile di output personalizzato sostituisce le istruzioni di ingegneria del software del preset `claude_code` con le vostre. Per mantenerle e stratificare le vostre istruzioni sopra, impostate `keep-coding-instructions: true` nel frontmatter. Mantenetele quando il vostro agente sta ancora facendo lavoro di ingegneria del software. Omettete quando state sostituendo completamente il ruolo.

L'esempio seguente definisce una persona di revisione del codice che mantiene le istruzioni di codifica, poiché la revisione del codice beneficia ancora della guida sulla sicurezza e la qualità del codice di Claude Code. Salvatelo come `~/.claude/output-styles/code-reviewer.md` per renderlo disponibile in tutti i progetti:

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  Attivare uno stile di output
</h4>

Una volta creato, attivate gli stili di output tramite:

* **CLI**: eseguite `/config` e selezionate uno stile di output
* **Impostazioni**: impostate `outputStyle` in `.claude/settings.local.json`
* **TypeScript SDK**: impostate `outputStyle` all'interno dell'oggetto `settings` inline passato a `query()`, oppure puntate `settings` a un file di impostazioni che lo imposta. `outputStyle` non è un campo `Options` di livello superiore:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

L'SDK Python non ha un'opzione per selezionare uno stile di output a livello di programmazione. Per distribuzioni solo codice dove non potete scrivere in `.claude/settings.local.json`, usate `append` o una stringa di prompt personalizzata invece.

**Nota per gli utenti dell'SDK:** Gli stili di output vengono caricati quando includete `settingSources: ['user']` o `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` o `setting_sources=["project"]` (Python) nelle vostre opzioni.

<h3 id="append-to-the-claude_code-preset">
  Aggiungere al preset `claude_code`
</h3>

Potete usare il preset Claude Code con una proprietà `append` per aggiungere le vostre istruzioni personalizzate preservando tutte le funzionalità integrate.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []

  async for message in query(
      prompt="Help me write a Python function to calculate fibonacci numbers",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "Always include detailed docstrings and type hints in Python code.",
          }
      ),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Migliorare il prompt caching tra utenti e macchine
</h4>

Per impostazione predefinita, due sessioni che utilizzano lo stesso preset `claude_code` e lo stesso testo `append` non possono comunque condividere una voce della cache del prompt se vengono eseguite da directory di lavoro diverse. Questo perché il preset incorpora il contesto per sessione nel system prompt prima del vostro testo `append`: la directory di lavoro, se è un repository git, la piattaforma, la shell attiva, la versione del sistema operativo, e i percorsi della memoria automatica. Qualsiasi differenza in quel contesto produce un system prompt diverso e un cache miss. Il contenuto di CLAUDE.md non influisce sulla cache del system prompt perché l'SDK lo inietta nella conversazione, non nel system prompt.

Per rendere il system prompt identico tra le sessioni, impostate `excludeDynamicSections: true` in TypeScript o `"exclude_dynamic_sections": True` in Python. Il contesto per sessione si sposta nel primo messaggio dell'utente, lasciando solo il preset statico e il vostro testo `append` nel system prompt in modo che le configurazioni identiche condividano una voce della cache tra utenti e macchine.

<Note>
  `excludeDynamicSections` richiede `@anthropic-ai/claude-agent-sdk` v0.2.98 o successivo, o `claude-agent-sdk` v0.1.58 o successivo per Python. Si applica solo alla forma dell'oggetto preset e non ha effetto quando `systemPrompt` è una stringa.
</Note>

L'esempio seguente abbina un blocco `append` condiviso con `excludeDynamicSections` in modo che una flotta di agenti in esecuzione da directory diverse possa riutilizzare lo stesso system prompt memorizzato nella cache:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Triage the open issues in this repo",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
              "exclude_dynamic_sections": True,
          },
      ),
  ):
      ...
  ```
</CodeGroup>

**Compromessi:** la directory di lavoro, il flag del repository git, la piattaforma, la shell attiva, la versione del sistema operativo, e i percorsi della memoria automatica raggiungono comunque Claude, ma come parte del primo messaggio dell'utente piuttosto che del system prompt. Le istruzioni nel messaggio dell'utente hanno un peso leggermente inferiore rispetto allo stesso testo nel system prompt, quindi Claude potrebbe fare affidamento su di esse meno fortemente quando ragiona sulla directory corrente o sui percorsi della memoria automatica. Abilitate questa opzione quando il riutilizzo della cache tra sessioni è più importante che il contesto dell'ambiente massimamente autorevole.

Per il flag equivalente in modalità CLI non interattiva, vedi [`--exclude-dynamic-system-prompt-sections`](/docs/it/cli-reference).

<h3 id="custom-system-prompts">
  System prompt personalizzati
</h3>

Potete fornire una stringa personalizzata come `systemPrompt` per sostituire completamente il valore predefinito con le vostre istruzioni.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []

  async for message in query(
      prompt="Create a data processing pipeline",
      options=ClaudeAgentOptions(system_prompt=custom_prompt),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h2 id="compare-the-four-approaches">
  Confronto dei quattro approcci
</h2>

I quattro metodi di personalizzazione differiscono per dove risiedono, come vengono condivisi e cosa preservano dal preset `claude_code`.

| Funzionalità                     | CLAUDE.md              | Stili di output                   | `systemPrompt` con append | `systemPrompt` personalizzato  |
| -------------------------------- | ---------------------- | --------------------------------- | ------------------------- | ------------------------------ |
| **Persistenza**                  | File per progetto      | Salvati come file                 | Solo sessione             | Solo sessione                  |
| **Riutilizzabilità**             | Per progetto           | Tra progetti                      | Duplicazione del codice   | Duplicazione del codice        |
| **Gestione**                     | Nel filesystem         | CLI + file                        | Nel codice                | Nel codice                     |
| **Strumenti predefiniti**        | Preservati             | Preservati                        | Preservati                | Persi (a meno che non inclusi) |
| **Sicurezza integrata**          | Mantenuta              | Mantenuta                         | Mantenuta                 | Deve essere aggiunta           |
| **Contesto dell'ambiente**       | Automatico             | Automatico                        | Automatico                | Deve essere fornito            |
| **Livello di personalizzazione** | Solo aggiunte          | Sostituisci o estendi predefinito | Solo aggiunte             | Controllo completo             |
| **Controllo versione**           | Con progetto           | Sì                                | Con codice                | Con codice                     |
| **Ambito**                       | Specifico del progetto | Utente o progetto                 | Sessione di codice        | Sessione di codice             |

"Con append" significa utilizzare `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` in TypeScript o `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` in Python. CLAUDE.md non modifica il prompt di sistema stesso: l'SDK ne inietta il contenuto nella conversazione come contesto del progetto.

<h2 id="use-cases-and-best-practices">
  Casi d'uso e best practice
</h2>

<h3 id="when-to-use-claude-md">
  Quando utilizzare CLAUDE.md
</h3>

Utilizzare CLAUDE.md per le istruzioni che dovrebbero applicarsi a ogni sessione in un progetto, indipendentemente dal prompt di sistema utilizzato dalla sessione: standard di codifica, comandi comuni, contesto dell'architettura e convenzioni del team. CLAUDE.md è sottoposto a commit nel vostro repository, quindi rimane sincronizzato con il codice che descrive. Consultare [When to add to CLAUDE.md](/docs/it/memory#when-to-add-to-claude-md) per una guida completa.

I file CLAUDE.md vengono caricati quando la fonte di impostazione `project` è abilitata, il che avviene per le opzioni predefinite di `query()`. Se impostate esplicitamente `settingSources` in TypeScript o `setting_sources` in Python, includete `'project'` per continuare a caricare CLAUDE.md a livello di progetto.

<h3 id="when-to-use-output-styles">
  Quando utilizzare gli stili di output
</h3>

Gli stili di output sono per le persone che desiderate riutilizzare tra la CLI e l'SDK senza modificare il codice dell'applicazione. Poiché risiedono come file in `.claude/output-styles`, la stessa persona è disponibile da `/config` nella CLI e da qualsiasi sessione SDK che carica la fonte di impostazione corrispondente.

**Ideale per:**

* Modifiche di comportamento persistenti tra sessioni
* Configurazioni condivise dal team
* Assistenti specializzati come un revisore del codice, un data scientist o un assistente DevOps
* Modifiche di prompt complesse che necessitano di versionamento

**Esempi:**

* Creazione di un assistente dedicato per l'ottimizzazione SQL
* Creazione di un revisore del codice incentrato sulla sicurezza
* Sviluppo di un assistente didattico con una pedagogia specifica

<h3 id="when-to-use-systemprompt-with-append">
  Quando utilizzare `systemPrompt` con append
</h3>

Utilizzare `append` quando il preset `claude_code` si adatta già al vostro prodotto e avete solo bisogno di aggiungere istruzioni extra. Mantenete la guida degli strumenti del preset, le regole di sicurezza e le convenzioni di codifica senza reimplementarle.

**Ideale per:**

* Aggiunta di standard di codifica o preferenze specifiche
* Personalizzazione della formattazione dell'output
* Aggiunta di conoscenze specifiche del dominio
* Modifica della verbosità della risposta
* Miglioramento del comportamento predefinito di Claude Code senza perdere le istruzioni degli strumenti

<h3 id="when-to-use-custom-systemprompt">
  Quando utilizzare `systemPrompt` personalizzato
</h3>

Utilizzare un prompt personalizzato quando la superficie, l'identità o il modello di autorizzazione dell'agente differisce da quello di Claude Code, come descritto in [Decide on a starting point](#decide-on-a-starting-point). Definite l'intero set di istruzioni, inclusa qualsiasi guida degli strumenti e regole di sicurezza di cui l'agente ha bisogno.

**Ideale per:**

* Controllo completo sul comportamento di Claude
* Attività specializzate a sessione singola
* Test di nuove strategie di prompt
* Situazioni in cui gli strumenti predefiniti non sono necessari
* Creazione di agenti specializzati con comportamento unico

<h2 id="combine-approaches">
  Combinazione di approcci
</h2>

Questi metodi si compongono insieme. Uno stile di output persistente o CLAUDE.md imposta il comportamento a lungo termine, e `append` sovrappone le istruzioni specifiche della sessione senza toccare la configurazione salvata.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combinare uno stile di output con aggiunte specifiche della sessione
</h3>

L'esempio seguente presuppone che uno stile di output Code Reviewer sia già attivo. Il blocco `append` sovrappone le aree di focus specifiche della sessione sulla persona, in modo che una singola sessione di revisione possa dare priorità a OAuth e all'archiviazione dei token senza modificare lo stile di output salvato:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []

  async for message in query(
      prompt="Review this authentication module",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": """
              For this review, prioritize:
              - OAuth 2.0 compliance
              - Token storage security
              - Session management
              """,
          }
      ),
  ):
      messages.append(message)
  ```
</CodeGroup>

<h2 id="see-also">
  Vedi anche
</h2>

* [Stili di output](/docs/it/output-styles): crea, gestisci e condividi stili di output per la CLI, inclusi il formato del file e i percorsi di archiviazione
* [Come Claude ricorda il tuo progetto](/docs/it/memory): cosa inserire in CLAUDE.md, dove posizionarlo e come scrivere istruzioni di progetto efficaci
* [Riferimento TypeScript SDK](/docs/it/agent-sdk/typescript): il tipo `Options` completo, inclusi `systemPrompt`, `settingSources` e `settings`
* [Riferimento Python SDK](/docs/it/agent-sdk/python): il tipo `ClaudeAgentOptions` completo, inclusi `system_prompt` e `setting_sources`
* [Impostazioni](/docs/it/settings): il riferimento `settings.json`, incluso dove sono archiviati gli stili di output e altre configurazioni
