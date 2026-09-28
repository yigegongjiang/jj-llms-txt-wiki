> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modifica dei system prompt

> Scegli tra il preset `claude_code` e un system prompt personalizzato, e personalizza il comportamento con CLAUDE.md, stili di output, append, o un prompt completamente personalizzato.

I system prompt definiscono il comportamento, le capacità e lo stile di risposta di Claude. Inizia dal preset `claude_code` per strumenti di codifica simili a CLI o IDE dove un utente osserva e guida il lavoro. Scrivi il tuo prompt per agenti con una superficie, identità o modello di autorizzazione diverso.

<h2 id="how-system-prompts-work">
  Come funzionano i system prompt
</h2>

Un system prompt è l'insieme iniziale di istruzioni che modella il comportamento di Claude durante una conversazione. Agent SDK ha tre punti di partenza per esso:

* **Default minimalista**: quando non impostate `systemPrompt` in TypeScript o `system_prompt` in Python, l'SDK utilizza un prompt minimalista che copre la chiamata degli strumenti ma omette il resto del contenuto del preset `claude_code`, incluse le istruzioni di sicurezza e protezione e il contesto sulla directory di lavoro e sull'ambiente. Questo differisce da `claude -p`, che utilizza il system prompt di Claude Code per impostazione predefinita. Se state migrando dalla CLI e desiderate un comportamento corrispondente, impostate il preset `claude_code`.
* **Preset `claude_code`**: il system prompt che utilizza la CLI di Claude Code, con istruzioni di utilizzo degli strumenti, istruzioni di sicurezza e protezione, e contesto sulla directory di lavoro e sull'ambiente. Impostate `systemPrompt: { type: "preset", preset: "claude_code" }` in TypeScript o `system_prompt={"type": "preset", "preset": "claude_code"}` in Python, facoltativamente con `append` per aggiungere le vostre istruzioni alla fine.
* **Stringa personalizzata**: un prompt che scrivete voi stessi. L'SDK invia solo ciò che fornite.

<h3 id="decide-on-a-starting-point">
  Decidere un punto di partenza
</h3>

Il fattore decisivo è quanto strettamente il vostro agente assomiglia a Claude Code: un agente di codifica che opera in un repository, con un umano che osserva l'output in streaming e guida il lavoro. Più il vostro prodotto si allontana da questo, più vorrete scrivere il vostro prompt.

| State costruendo                                                                                                                   | Utilizzate                        | Cosa ottenete                                                                                                                                                         |
| :--------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Uno strumento di codifica simile a CLI o IDE dove un umano osserva e guida, e i default di Claude Code sono quello che desiderate  | Preset `claude_code`              | Il prompt di Claude Code, inclusa la guida degli strumenti, le regole di sicurezza, e il contesto dell'ambiente                                                       |
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

`append` e una stringa di prompt personalizzata modificano direttamente il system prompt, e uno stile di output cambia le istruzioni che Claude Code fornisce a Claude per ogni risposta. CLAUDE.md segue un percorso diverso: l'SDK lo legge e inietta il suo contenuto nella conversazione come contesto del progetto, quindi modella il comportamento insieme a qualsiasi system prompt Lei scelga. [Skills](/docs/it/agent-sdk/skills), [hooks](/docs/it/agent-sdk/hooks), e [permissions](/docs/it/agent-sdk/permissions) modellano anche il comportamento al di fuori del system prompt e sono trattati in pagine separate.

<h3 id="claude-md-files-for-project-level-instructions">
  File CLAUDE.md per istruzioni a livello di progetto
</h3>

I file CLAUDE.md forniscono a Claude contesto e istruzioni persistenti a livello di progetto. L'SDK inietta il loro contenuto nella conversazione e lascia il system prompt intatto, quindi funzionano con qualsiasi configurazione di system prompt. Per sapere cosa mettere in CLAUDE.md, dove posizionarlo e come scrivere istruzioni efficaci, vedi [Quando aggiungere a CLAUDE.md](/docs/it/memory#when-to-add-to-claude-md) e il resto di [Come Claude ricorda il tuo progetto](/docs/it/memory). Questa sezione copre ciò che è specifico dell'SDK: come CLAUDE.md si carica.

L'SDK legge CLAUDE.md quando la corrispondente fonte di impostazione è abilitata: `'project'` carica `CLAUDE.md` o `.claude/CLAUDE.md` dalla directory di lavoro, e `'user'` carica `~/.claude/CLAUDE.md`. Le opzioni predefinite di `query()` abilitano entrambe le fonti, quindi CLAUDE.md si carica automaticamente. Se impostate `settingSources` in TypeScript o `setting_sources` in Python esplicitamente, includete le fonti di cui avete bisogno. Il caricamento di CLAUDE.md è controllato dalle fonti di impostazione, non dal preset `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Caricare CLAUDE.md con l'SDK
</h4>

Per caricare CLAUDE.md, impostate `settingSources` per includere il livello in cui vive il vostro CLAUDE.md. L'esempio seguente carica un CLAUDE.md a livello di progetto insieme al preset `claude_code`, quindi Claude ha sia il prompt dell'agente di codifica che le convenzioni del vostro progetto:

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
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


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

Quando eseguite uno dei due esempi, l'SDK trasmette i messaggi mentre Claude lavora: un messaggio di inizializzazione del sistema, messaggi dell'assistente, messaggi dell'utente che trasportano risultati degli strumenti, e un messaggio di risultato finale con l'esito della sessione.

CLAUDE.md è persistente in tutte le sessioni di un progetto, condiviso con il vostro team tramite git, e scoperto automaticamente senza modifiche al codice. Non viene caricato se passate un array `settingSources` vuoto.

<h3 id="output-styles-for-persistent-configurations">
  Stili di output per configurazioni persistenti
</h3>

Gli stili di output sono configurazioni salvate che modificano il ruolo, il tono e il formato di output di Claude. Vengono archiviati come file markdown e possono essere riutilizzati in sessioni e progetti diversi.

<h4 id="create-an-output-style">
  Creare uno stile di output
</h4>

Uno stile di output è un file markdown con [frontmatter](/docs/it/output-styles#frontmatter) per i metadati, seguito dal contenuto del prompt. Salvatelo in `~/.claude/output-styles/` per uno stile a livello di utente disponibile in ogni progetto, o `.claude/output-styles/` nel vostro repository per uno stile a livello di progetto che potete committare e condividere con il vostro team.

Uno stile di output personalizzato lascia fuori le istruzioni di ingegneria del software del preset `claude_code` e usa le vostre. Per mantenerle e stratificare le vostre istruzioni sopra, impostate `keep-coding-instructions: true` nel frontmatter. Queste istruzioni sono solo nel system prompt completo di Claude Code, quindi l'impostazione non ha effetto in una sessione sul system prompt più breve, che attivate o disattivate con [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/it/env-vars#variables). Mantenetele quando il vostro agente sta ancora facendo lavoro di ingegneria del software. Omettete quando state sostituendo completamente il ruolo.

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

* **CLI**: eseguite `/output-style <style>`, ad esempio `/output-style concise`, oppure eseguite `/config` e selezionate uno. Il comando `/output-style` richiede Claude Code v2.1.269 o successivo.
* **Impostazioni**: impostate `outputStyle` in `.claude/settings.local.json`
* **TypeScript SDK**: impostate `outputStyle` all'interno dell'oggetto `settings` inline passato a `query()`, oppure puntate `settings` a un file di impostazioni che lo imposta. `outputStyle` non è un campo `Options` di livello superiore:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Nell'SDK Python, impostate `outputStyle` tramite l'opzione `settings`, che accetta una stringa JSON come `'{"outputStyle": "Explanatory"}'` o un percorso a un file di impostazioni che lo imposta.

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
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


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Migliorare il prompt caching tra utenti e macchine
</h4>

Per impostazione predefinita, due sessioni che utilizzano lo stesso preset `claude_code` e lo stesso testo `append` non possono comunque condividere una voce della cache del prompt se vengono eseguite da directory di lavoro diverse. Questo perché il preset incorpora il contesto per sessione nel system prompt prima del vostro testo `append`: la directory di lavoro, se è un repository git, la piattaforma, la shell attiva, la versione del sistema operativo, e i percorsi della memoria automatica. Qualsiasi differenza in quel contesto produce un system prompt diverso e un cache miss. Il contenuto di CLAUDE.md non influisce sulla cache del system prompt perché l'SDK lo inietta nella conversazione, non nel system prompt.

Per rendere il system prompt identico tra le sessioni, impostate `excludeDynamicSections: true` in TypeScript o `"exclude_dynamic_sections": True` in Python. Il contesto per sessione si sposta nel primo messaggio dell'utente, lasciando solo il preset statico e il vostro testo `append` nel system prompt in modo che le configurazioni identiche condividano una voce della cache tra utenti e macchine.

<Note>
  `excludeDynamicSections` richiede `@anthropic-ai/claude-agent-sdk` v0.2.98 o successivo, o `claude-agent-sdk` v0.1.58 o successivo per Python. Impostatelo solo sulla forma dell'oggetto preset. L'SDK lo ignora quando passate un prompt personalizzato invece del preset; per mantenere le istruzioni di un prompt personalizzato memorizzate nella cache nell'SDK TypeScript, vedi [Memorizzare nella cache la parte statica di un prompt personalizzato](#cache-the-static-part-of-a-custom-prompt).
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
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


  asyncio.run(main())
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

In Python, caricate un prompt personalizzato di grandi dimensioni da un file con `system_prompt={"type": "file", "path": "..."}` invece di passarlo come stringa. L'SDK Python passa un prompt stringa come un argomento della riga di comando al subprocess CLI, quindi un prompt che supera il limite di lunghezza dell'argomento del sistema operativo fallisce al spawn del processo prima che venga inviata qualsiasi richiesta API. Su Linux l'errore è `Argument list too long`. Vedi [`SystemPromptFile`](/docs/it/agent-sdk/python#systempromptfile) per le soglie della piattaforma e il comportamento di Windows.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  Memorizzare nella cache la parte statica di un prompt personalizzato
</h4>

Nell'SDK TypeScript, potete passare un prompt personalizzato come un array di stringhe invece di una stringa, con il marcatore `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` tra la parte statica e il resto. Usate questo quando il vostro prompt combina istruzioni che sono uguali su ogni richiesta con contesto che cambia per richiesta, come il cliente o il ticket che l'agente sta gestendo. Quando passate entrambe le parti come una stringa, un cambiamento alla parte per richiesta cambia l'intero system prompt, quindi le istruzioni statiche perdono la cache anche. La forma array non è disponibile nell'SDK Python; [`ClaudeAgentOptions`](/docs/it/agent-sdk/python#claudeagentoptions) elenca le forme che `system_prompt` accetta.

<Note>
  L'SDK divide il prompt solo quando chiama direttamente l'API Claude o viene eseguito su [Claude Platform on AWS](/docs/it/claude-platform-on-aws). In ogni altra configurazione, come Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o un [LLM gateway](/docs/it/llm-gateway-connect), e ogni volta che impostate [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/it/llm-gateway-protocol#disable-pre-release-capabilities), l'SDK invia l'intero prompt come un blocco, lo stesso che passare una stringa.
</Note>

Per dividere il prompt, importate `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` da `@anthropic-ai/claude-agent-sdk` e passatelo come elemento array proprio tra le due parti. L'SDK invia le stringhe prima del marcatore come un blocco di testo e le stringhe dopo come un secondo blocco, ognuno con il suo punto di interruzione della cache. Nell'esempio seguente, un agente di supporto carica le sue istruzioni di triage da un file e riceve dettagli su un ticket su ogni richiesta, quindi le istruzioni rimangono memorizzate nella cache mentre i dettagli del ticket cambiano:

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Tracciare i token della cache](/docs/it/agent-sdk/cost-tracking#track-cache-tokens) descrive i campi `cache_creation_input_tokens` e `cache_read_input_tokens` su ogni messaggio di risultato.

L'SDK assembla i blocchi dall'array come segue:

* L'SDK unisce le stringhe su ogni lato del marcatore con una riga vuota tra di loro e rimuove il marcatore stesso, quindi il testo del marcatore non raggiunge Claude.
* Se includete il marcatore più di una volta, il primo è la divisione e l'SDK rimuove gli altri.
* Se lasciate fuori il marcatore, l'SDK unisce tutte le stringhe in un blocco, lo stesso che passare una stringa.

Con i flag [`--system-prompt` o `--system-prompt-file`](/docs/it/cli-reference#system-prompt-flags) della CLI, il prompt è una stringa, quindi non c'è un array per portare il marcatore. Includete una riga contenente solo `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` tra le parti statiche e per richiesta. Claude Code divide il prompt alla prima riga di questo tipo nei due blocchi e rimuove quella riga. Richiede Claude Code v2.1.275 o successivo.

Nell'SDK, preferite la forma array, che porta il confine senza una riga marcatore.

<h3 id="change-the-prompt-of-an-existing-session">
  Cambiare il prompt di una sessione esistente
</h3>

Per impostazione predefinita, se passate un `append` o prompt personalizzato diverso quando tornate a una sessione con `resume` o `continue`, Claude non lo vede al turno successivo. Claude Code registra il system prompt alla prima richiesta di una sessione e riutilizza quel record fino a quando la sessione non viene compattata. Il nuovo testo ha effetto dopo quella compattazione, o in una nuova sessione.

<h4 id="update-claude’s-instructions-mid-session">
  Aggiornare le istruzioni di Claude a metà sessione
</h4>

Se le istruzioni che mettete nel system prompt devono cambiare mentre una sessione è in esecuzione, ad esempio perché il vostro utente ha cambiato l'agente in una modalità di sola lettura o ha modificato la sua configurazione nella vostra app, inviate le nuove istruzioni nella conversazione invece di cambiare `systemPrompt`:

* **Nel vostro prossimo messaggio**: includete le nuove istruzioni nel prossimo messaggio dell'utente che inviate.
* **Da un hook**: restituite [`additionalContext`](/docs/it/hooks#add-context-for-claude) da un callback di hook `UserPromptSubmit` o `PostToolUse` [hook callback](/docs/it/agent-sdk/hooks#outputs), scritto come un'affermazione fattuale come "L'area di lavoro è ora di sola lettura". L'SDK inserisce il testo nella conversazione nel punto in cui l'hook si è attivato, quindi il prompt registrato rimane invariato.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  Disattivare la registrazione mentre iterate sulla formulazione
</h4>

Mentre iterate sulla formulazione del prompt e volete che ogni modifica raggiunga una sessione che riprendete, impostate `snapshot` a false sulla forma dell'oggetto del system prompt. Claude Code quindi ricostruisce il prompt su ogni richiesta. Il campo è disponibile sulla forma preset e personalizzata di [`systemPrompt`](/docs/it/agent-sdk/typescript#options) in TypeScript e di [`system_prompt`](/docs/it/agent-sdk/python#systempromptpreset) in Python, e richiede `@anthropic-ai/claude-agent-sdk` v0.3.257 o successivo, o `claude-agent-sdk` v0.2.153 o successivo.

Mantenete la registrazione attiva in produzione. Con la registrazione disattivata, un `append` o prompt personalizzato diverso su una sessione ripresa raggiunge Claude al turno successivo, e quella richiesta non può riutilizzare la [prompt cache](/docs/it/prompt-caching#how-the-cache-is-organized) della sessione. Dove l'API applica il [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking), Claude perde anche il suo thinking dai turni precedenti.

Al di fuori delle [cloud sessions](/docs/it/cloud-environments), se avviate Claude Code in [bare mode](/docs/it/headless#start-faster-with-bare-mode) passando `--bare` tramite `extraArgs` o impostando `CLAUDE_CODE_SIMPLE=1`, la registrazione rimane disattivata a meno che non impostiate `snapshot: true`.

La registrazione di un `append` o prompt personalizzato per impostazione predefinita richiede Claude Code v2.1.265 o successivo, che l'Agent SDK TypeScript raggruppa dalla v0.3.265 e l'Agent SDK Python dalla v0.2.153. Prima di Claude Code v2.1.268, le sessioni che non [recuperano i flag delle funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching), incluse le sessioni su Amazon Bedrock, Google Cloud's Agent Platform, e Microsoft Foundry, ricostruivano il prompt su ogni richiesta e `snapshot` non aveva effetto.

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

<h2 id="combine-approaches">
  Combinare gli approcci
</h2>

Gli approcci si compongono. Uno stile di output persistente o CLAUDE.md imposta il comportamento a lungo termine, e `append` sovrappone le istruzioni specifiche della sessione senza toccare la configurazione salvata.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combinare uno stile di output con aggiunte specifiche della sessione
</h3>

L'esempio seguente presuppone che uno stile di output Code Reviewer sia già attivo. Il blocco `append` sovrappone le aree di focus specifiche della sessione sulla persona, in modo che una singola sessione di revisione possa dare priorità a OAuth e archiviazione dei token senza modificare lo stile di output salvato:

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []


  async def main():
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


  asyncio.run(main())
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
