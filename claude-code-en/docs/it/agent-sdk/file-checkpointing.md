> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ripristina le modifiche ai file con checkpointing

> Traccia le modifiche ai file durante le sessioni dell'agente e ripristina i file a qualsiasi stato precedente

Il checkpointing dei file traccia le modifiche ai file effettuate tramite gli strumenti Write, Edit e NotebookEdit durante una sessione dell'agente, consentendoti di ripristinare i file a qualsiasi stato precedente. Vuoi provarlo? Vai all'[esempio interattivo](#try-it-out).

Con il checkpointing, puoi:

* **Annullare le modifiche indesiderate** ripristinando i file a uno stato noto e funzionante
* **Esplorare alternative** ripristinando a un checkpoint e provando un approccio diverso
* **Recuperare da errori** quando l'agente effettua modifiche non corrette

<Warning>
  Solo le modifiche effettuate tramite gli strumenti Write, Edit e NotebookEdit vengono tracciate. Le modifiche effettuate tramite comandi Bash (come `echo > file.txt` o `sed -i`) non vengono acquisite dal sistema di checkpoint.
</Warning>

<h2 id="how-checkpointing-works">
  Come funziona il checkpointing
</h2>

Quando abiliti il checkpointing dei file, l'SDK crea backup dei file prima di modificarli tramite gli strumenti Write, Edit o NotebookEdit. I messaggi dell'utente nel flusso di risposta includono un UUID di checkpoint che puoi utilizzare come punto di ripristino.

Checkpoint funziona con questi strumenti integrati che l'agente utilizza per modificare i file:

| Tool         | Description                                                            |
| ------------ | ---------------------------------------------------------------------- |
| Write        | Crea un nuovo file o sovrascrive un file esistente con nuovo contenuto |
| Edit         | Effettua modifiche mirate a parti specifiche di un file esistente      |
| NotebookEdit | Modifica le celle nei notebook Jupyter (file `.ipynb`)                 |

<Note>
  Il ripristino dei file ripristina i file su disco a uno stato precedente. Non ripristina la conversazione stessa. La cronologia della conversazione e il contesto rimangono intatti dopo la chiamata a `rewindFiles()` (TypeScript) o `rewind_files()` (Python).
</Note>

Il sistema di checkpoint traccia:

* File creati durante la sessione
* File modificati durante la sessione
* Il contenuto originale dei file modificati

Quando ripristini a un checkpoint, i file creati vengono eliminati e i file modificati vengono ripristinati al loro contenuto in quel momento.

<h2 id="implement-checkpointing">
  Implementare il checkpointing
</h2>

Per utilizzare il checkpointing dei file, abilitalo nelle tue opzioni, acquisisci gli UUID di checkpoint dal flusso di risposta, quindi chiama `rewindFiles()` (TypeScript) o `rewind_files()` (Python) quando hai bisogno di ripristinare.

L'esempio seguente mostra il flusso completo: abilita il checkpointing, acquisisci l'UUID di checkpoint e l'ID di sessione dal flusso di risposta, quindi riprendi la sessione in seguito per ripristinare i file. Ogni passaggio è spiegato in dettaglio di seguito. Gli esempi in questa sezione utilizzano il prompt "Refactor the authentication module". Eseguili in un progetto che contiene un modulo di autenticazione, oppure modifica il prompt per nominare file che esistono nel tuo progetto, in modo da poter osservare i file cambiare e vedere il ripristino ripristinarli.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeSDKClient,
      ClaudeAgentOptions,
      UserMessage,
      ResultMessage,
  )


  async def main():
      # Step 1: Enable checkpointing
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",  # Auto-accept file edits without prompting
          extra_args={
              "replay-user-messages": None
          },  # Required to receive checkpoint UUIDs in the response stream
      )

      checkpoint_id = None
      session_id = None

      # Run the query and capture checkpoint UUID and session ID
      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")

          # Step 2: Capture checkpoint UUID from the first user message
          async for message in client.receive_response():
              if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                  checkpoint_id = message.uuid
              if isinstance(message, ResultMessage) and not session_id:
                  session_id = message.session_id

      # Step 3: Later, rewind by resuming the session with an empty prompt
      if checkpoint_id and session_id:
          async with ClaudeSDKClient(
              ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
          ) as client:
              await client.query("")  # Empty prompt to open the connection
              async for message in client.receive_response():
                  await client.rewind_files(checkpoint_id)
                  break
          print(f"Rewound to checkpoint: {checkpoint_id}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  async function main() {
    // Step 1: Enable checkpointing
    const opts = {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const, // Auto-accept file edits without prompting
      extraArgs: { "replay-user-messages": null } // Required to receive checkpoint UUIDs in the response stream
    };

    const response = query({
      prompt: "Refactor the authentication module",
      options: opts
    });

    let checkpointId: string | undefined;
    let sessionId: string | undefined;

    // Step 2: Capture checkpoint UUID from the first user message
    for await (const message of response) {
      if (message.type === "user" && message.uuid && !checkpointId) {
        checkpointId = message.uuid;
      }
      if ("session_id" in message && !sessionId) {
        sessionId = message.session_id;
      }
    }

    // Step 3: Later, rewind by resuming the session with an empty prompt
    if (checkpointId && sessionId) {
      const rewindQuery = query({
        prompt: "", // Empty prompt to open the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);
        break;
      }
      console.log(`Rewound to checkpoint: ${checkpointId}`);
    }
  }

  main();
  ```
</CodeGroup>

<Steps>
  <Step title="Abilita il checkpointing">
    Configura le opzioni dell'SDK per abilitare il checkpointing e ricevere gli UUID di checkpoint:

    | Option                   | Python                                      | TypeScript                                    | Description                                                            |
    | ------------------------ | ------------------------------------------- | --------------------------------------------- | ---------------------------------------------------------------------- |
    | Enable checkpointing     | `enable_file_checkpointing=True`            | `enableFileCheckpointing: true`               | Traccia le modifiche ai file per il ripristino                         |
    | Receive checkpoint UUIDs | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Obbligatorio per ottenere gli UUID dei messaggi dell'utente nel flusso |

    <CodeGroup>
      ```python Python theme={null}
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",
          extra_args={"replay-user-messages": None},
      )

      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")
      ```

      ```typescript TypeScript theme={null}
      const response = query({
        prompt: "Refactor the authentication module",
        options: {
          enableFileCheckpointing: true,
          permissionMode: "acceptEdits" as const,
          extraArgs: { "replay-user-messages": null }
        }
      });
      ```
    </CodeGroup>
  </Step>

  <Step title="Acquisisci l'UUID di checkpoint e l'ID di sessione">
    Con l'opzione `replay-user-messages` impostata (mostrata sopra), ogni messaggio dell'utente nel flusso di risposta ha un UUID che funge da checkpoint.

    Per la maggior parte dei casi d'uso, acquisisci il primo UUID del messaggio dell'utente (`message.uuid`); il ripristino ad esso ripristina tutti i file al loro stato originale. Per archiviare più checkpoint e ripristinare a stati intermedi, vedi [Più punti di ripristino](#multiple-restore-points).

    L'acquisizione dell'ID di sessione (`message.session_id`) è facoltativa; ne hai bisogno solo se vuoi ripristinare in seguito, dopo il completamento del flusso. Se stai chiamando `rewindFiles()` immediatamente mentre stai ancora elaborando i messaggi (come fa l'esempio in [Checkpoint prima di operazioni rischiose](#checkpoint-before-risky-operations)), puoi saltare l'acquisizione dell'ID di sessione.

    <CodeGroup>
      ```python Python theme={null}
      checkpoint_id = None
      session_id = None

      async for message in client.receive_response():
          # Capture the first user message UUID as the checkpoint
          if isinstance(message, UserMessage) and message.uuid and checkpoint_id is None:
              checkpoint_id = message.uuid
          # Capture session ID from the result message
          if isinstance(message, ResultMessage):
              session_id = message.session_id
      ```

      ```typescript TypeScript theme={null}
      let checkpointId: string | undefined;
      let sessionId: string | undefined;

      for await (const message of response) {
        // Capture the first user message UUID as the checkpoint
        if (message.type === "user" && message.uuid && !checkpointId) {
          checkpointId = message.uuid;
        }
        // Capture session ID from any message that has it
        if ("session_id" in message) {
          sessionId = message.session_id;
        }
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Ripristina i file">
    Per ripristinare dopo il completamento del flusso, riprendi la sessione con un prompt vuoto e chiama `rewind_files()` (Python) o `rewindFiles()` (TypeScript) con il tuo UUID di checkpoint. Puoi anche ripristinare durante il flusso; vedi [Checkpoint prima di operazioni rischiose](#checkpoint-before-risky-operations) per quel modello.

    <CodeGroup>
      ```python Python theme={null}
      async with ClaudeSDKClient(
          ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
      ) as client:
          await client.query("")  # Empty prompt to open the connection
          async for message in client.receive_response():
              await client.rewind_files(checkpoint_id)
              break
      ```

      ```typescript TypeScript theme={null}
      const rewindQuery = query({
        prompt: "", // Empty prompt to open the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);
        break;
      }
      ```
    </CodeGroup>

    Se acquisisci l'ID di sessione e l'ID di checkpoint, puoi anche ripristinare dalla CLI. Questo comando richiede l'eseguibile `claude`, che proviene da [installazione di Claude Code](/docs/it/setup) e non è installato dal pacchetto SDK. L'SDK abilita il checkpointing per te, ma quando esegui `claude -p` direttamente devi impostare la variabile di ambiente `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`:

    ```bash theme={null}
    CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
    ```

    Il flag `--rewind-files` non appare nell'output di `claude --help`, ma la CLI lo accetta come mostrato.
  </Step>
</Steps>

<h2 id="common-patterns">
  Modelli comuni
</h2>

Questi modelli mostrano diversi modi per acquisire e utilizzare gli UUID di checkpoint a seconda del tuo caso d'uso.

<h3 id="checkpoint-before-risky-operations">
  Checkpoint prima di operazioni rischiose
</h3>

Questo modello mantiene solo l'UUID di checkpoint più recente, aggiornandolo prima di ogni turno dell'agente. Se qualcosa va storto durante l'elaborazione, puoi immediatamente ripristinare all'ultimo stato sicuro e uscire dal ciclo.

Prima di eseguire questo esempio, sostituisci `your_revert_condition` (Python) o `yourRevertCondition` (TypeScript) con il tuo controllo personale, come il rilevamento di errori o un errore di convalida; il segnaposto non è definito nell'esempio.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage


  async def main():
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",
          extra_args={"replay-user-messages": None},
      )

      safe_checkpoint = None

      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")

          async for message in client.receive_response():
              # Update checkpoint before each agent turn starts
              # This overwrites the previous checkpoint. Only keep the latest
              if isinstance(message, UserMessage) and message.uuid:
                  safe_checkpoint = message.uuid

              # Decide when to revert based on your own logic
              # For example: error detection, validation failure, or user input
              if your_revert_condition and safe_checkpoint:
                  await client.rewind_files(safe_checkpoint)
                  # Exit the loop after rewinding, files are restored
                  break


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  async function main() {
    const response = query({
      prompt: "Refactor the authentication module",
      options: {
        enableFileCheckpointing: true,
        permissionMode: "acceptEdits" as const,
        extraArgs: { "replay-user-messages": null }
      }
    });

    let safeCheckpoint: string | undefined;

    for await (const message of response) {
      // Update checkpoint before each agent turn starts
      // This overwrites the previous checkpoint. Only keep the latest
      if (message.type === "user" && message.uuid) {
        safeCheckpoint = message.uuid;
      }

      // Decide when to revert based on your own logic
      // For example: error detection, validation failure, or user input
      if (yourRevertCondition && safeCheckpoint) {
        await response.rewindFiles(safeCheckpoint);
        // Exit the loop after rewinding, files are restored
        break;
      }
    }
  }

  main();
  ```
</CodeGroup>

<h3 id="multiple-restore-points">
  Più punti di ripristino
</h3>

Se Claude effettua modifiche su più turni, potresti voler ripristinare a un punto specifico piuttosto che all'inizio. Ad esempio, se Claude effettua il refactoring di un file nel turno uno e aggiunge test nel turno due, potresti voler mantenere il refactoring ma annullare i test.

Questo modello archivia tutti gli UUID di checkpoint in un array con metadati. Dopo il completamento della sessione, puoi ripristinare a qualsiasi checkpoint precedente:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from dataclasses import dataclass
  from datetime import datetime
  from claude_agent_sdk import (
      ClaudeSDKClient,
      ClaudeAgentOptions,
      UserMessage,
      ResultMessage,
  )


  # Store checkpoint metadata for better tracking
  @dataclass
  class Checkpoint:
      id: str
      description: str
      timestamp: datetime


  async def main():
      options = ClaudeAgentOptions(
          enable_file_checkpointing=True,
          permission_mode="acceptEdits",
          extra_args={"replay-user-messages": None},
      )

      checkpoints = []
      session_id = None

      async with ClaudeSDKClient(options) as client:
          await client.query("Refactor the authentication module")

          async for message in client.receive_response():
              if isinstance(message, UserMessage) and message.uuid:
                  checkpoints.append(
                      Checkpoint(
                          id=message.uuid,
                          description=f"After turn {len(checkpoints) + 1}",
                          timestamp=datetime.now(),
                      )
                  )
              if isinstance(message, ResultMessage) and not session_id:
                  session_id = message.session_id

      # Later: rewind to any checkpoint by resuming the session
      if checkpoints and session_id:
          target = checkpoints[0]  # Pick any checkpoint
          async with ClaudeSDKClient(
              ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
          ) as client:
              await client.query("")  # Empty prompt to open the connection
              async for message in client.receive_response():
                  await client.rewind_files(target.id)
                  break
          print(f"Rewound to: {target.description}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Store checkpoint metadata for better tracking
  interface Checkpoint {
    id: string;
    description: string;
    timestamp: Date;
  }

  async function main() {
    const opts = {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const,
      extraArgs: { "replay-user-messages": null }
    };

    const response = query({
      prompt: "Refactor the authentication module",
      options: opts
    });

    const checkpoints: Checkpoint[] = [];
    let sessionId: string | undefined;

    for await (const message of response) {
      if (message.type === "user" && message.uuid) {
        checkpoints.push({
          id: message.uuid,
          description: `After turn ${checkpoints.length + 1}`,
          timestamp: new Date()
        });
      }
      if ("session_id" in message && !sessionId) {
        sessionId = message.session_id;
      }
    }

    // Later: rewind to any checkpoint by resuming the session
    if (checkpoints.length > 0 && sessionId) {
      const target = checkpoints[0]; // Pick any checkpoint
      const rewindQuery = query({
        prompt: "", // Empty prompt to open the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(target.id);
        break;
      }
      console.log(`Rewound to: ${target.description}`);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="try-it-out">
  Prova
</h2>

Questo esempio completo crea un piccolo file di utilità, fa aggiungere all'agente commenti di documentazione, ti mostra le modifiche, quindi ti chiede se vuoi ripristinare.

Prima di iniziare, assicurati di avere [Claude Agent SDK installato](/docs/it/agent-sdk/quickstart).

<Steps>
  <Step title="Crea un file di test">
    Crea un nuovo file chiamato `utils.py` (Python) o `utils.ts` (TypeScript) e incolla il seguente codice:

    <CodeGroup>
      ```python utils.py theme={null}
      def add(a, b):
          return a + b


      def subtract(a, b):
          return a - b


      def multiply(a, b):
          return a * b


      def divide(a, b):
          if b == 0:
              raise ValueError("Cannot divide by zero")
          return a / b
      ```

      ```typescript utils.ts theme={null}
      export function add(a: number, b: number): number {
        return a + b;
      }

      export function subtract(a: number, b: number): number {
        return a - b;
      }

      export function multiply(a: number, b: number): number {
        return a * b;
      }

      export function divide(a: number, b: number): number {
        if (b === 0) {
          throw new Error("Cannot divide by zero");
        }
        return a / b;
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Esegui l'esempio interattivo">
    Crea un nuovo file chiamato `try_checkpointing.py` (Python) o `try_checkpointing.ts` (TypeScript) nella stessa directory del tuo file di utilità e incolla il seguente codice.

    Questo script chiede a Claude di aggiungere commenti doc al tuo file di utilità, quindi ti dà la possibilità di ripristinare e ripristinare l'originale.

    <CodeGroup>
      ```python try_checkpointing.py theme={null}
      import asyncio
      from claude_agent_sdk import (
          ClaudeSDKClient,
          ClaudeAgentOptions,
          UserMessage,
          ResultMessage,
      )


      async def main():
          # Configure the SDK with checkpointing enabled
          # - enable_file_checkpointing: Track file changes for rewinding
          # - permission_mode: Auto-accept file edits without prompting
          # - extra_args: Required to receive user message UUIDs in the stream
          options = ClaudeAgentOptions(
              enable_file_checkpointing=True,
              permission_mode="acceptEdits",
              extra_args={"replay-user-messages": None},
          )

          checkpoint_id = None  # Store the user message UUID for rewinding
          session_id = None  # Store the session ID for resuming

          print("Running agent to add doc comments to utils.py...\n")

          # Run the agent and capture checkpoint data from the response stream
          async with ClaudeSDKClient(options) as client:
              await client.query("Add doc comments to utils.py")

              async for message in client.receive_response():
                  # Capture the first user message UUID - this is our restore point
                  if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                      checkpoint_id = message.uuid
                  # Capture the session ID so we can resume later
                  if isinstance(message, ResultMessage):
                      session_id = message.session_id

          print("Done! Open utils.py to see the added doc comments.\n")

          # Ask the user if they want to rewind the changes
          if checkpoint_id and session_id:
              response = input("Rewind to remove the doc comments? (y/n): ")

              if response.lower() == "y":
                  # Resume the session with an empty prompt, then rewind
                  async with ClaudeSDKClient(
                      ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
                  ) as client:
                      await client.query("")  # Empty prompt opens the connection
                      async for message in client.receive_response():
                          await client.rewind_files(checkpoint_id)  # Restore files
                          break

                  print(
                      "\n✓ File restored! Open utils.py to verify the doc comments are gone."
                  )
              else:
                  print("\nKept the modified file.")


      asyncio.run(main())
      ```

      ```typescript try_checkpointing.ts theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";
      import * as readline from "readline";

      async function main() {
        // Configure the SDK with checkpointing enabled
        // - enableFileCheckpointing: Track file changes for rewinding
        // - permissionMode: Auto-accept file edits without prompting
        // - extraArgs: Required to receive user message UUIDs in the stream
        const opts = {
          enableFileCheckpointing: true,
          permissionMode: "acceptEdits" as const,
          extraArgs: { "replay-user-messages": null }
        };

        let sessionId: string | undefined; // Store the session ID for resuming
        let checkpointId: string | undefined; // Store the user message UUID for rewinding

        console.log("Running agent to add doc comments to utils.ts...\n");

        // Run the agent and capture checkpoint data from the response stream
        const response = query({
          prompt: "Add doc comments to utils.ts",
          options: opts
        });

        for await (const message of response) {
          // Capture the first user message UUID - this is our restore point
          if (message.type === "user" && message.uuid && !checkpointId) {
            checkpointId = message.uuid;
          }
          // Capture the session ID so we can resume later
          if ("session_id" in message) {
            sessionId = message.session_id;
          }
        }

        console.log("Done! Open utils.ts to see the added doc comments.\n");

        // Ask the user if they want to rewind the changes
        if (checkpointId && sessionId) {
          const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
          });

          const answer = await new Promise<string>((resolve) => {
            rl.question("Rewind to remove the doc comments? (y/n): ", resolve);
          });
          rl.close();

          if (answer.toLowerCase() === "y") {
            // Resume the session with an empty prompt, then rewind
            const rewindQuery = query({
              prompt: "", // Empty prompt opens the connection
              options: { ...opts, resume: sessionId }
            });

            for await (const msg of rewindQuery) {
              await rewindQuery.rewindFiles(checkpointId); // Restore files
              break;
            }

            console.log("\n✓ File restored! Open utils.ts to verify the doc comments are gone.");
          } else {
            console.log("\nKept the modified file.");
          }
        }
      }

      main();
      ```
    </CodeGroup>

    Questo esempio dimostra il flusso di lavoro completo del checkpointing:

    1. **Abilita il checkpointing**: configura l'SDK con `enable_file_checkpointing=True` e `permission_mode="acceptEdits"` per approvare automaticamente le modifiche ai file
    2. **Acquisisci i dati di checkpoint**: mentre l'agente è in esecuzione, archivia il primo UUID del messaggio dell'utente (il tuo punto di ripristino) e l'ID di sessione
    3. **Richiedi il ripristino**: dopo che l'agente ha terminato, controlla il tuo file di utilità per vedere i commenti doc, quindi decidi se vuoi annullare le modifiche
    4. **Riprendi e ripristina**: se sì, riprendi la sessione con un prompt vuoto e chiama `rewind_files()` per ripristinare il file originale
  </Step>

  <Step title="Esegui l'esempio">
    Esegui lo script dalla stessa directory del tuo file di utilità.

    <Tip>
      Apri il tuo file di utilità (`utils.py` o `utils.ts`) nel tuo IDE o editor prima di eseguire lo script. Vedrai il file aggiornarsi in tempo reale mentre l'agente aggiunge commenti doc, quindi tornare all'originale quando scegli di ripristinare.
    </Tip>

    <Tabs>
      <Tab title="Python">
        ```bash theme={null}
        python try_checkpointing.py
        ```
      </Tab>

      <Tab title="TypeScript">
        ```bash theme={null}
        npx tsx try_checkpointing.ts
        ```
      </Tab>
    </Tabs>

    Vedrai l'agente aggiungere commenti doc, quindi un prompt che ti chiede se vuoi ripristinare. Se scegli sì, il file viene ripristinato al suo stato originale.
  </Step>
</Steps>

<h2 id="limitations">
  Limitazioni
</h2>

Il checkpointing dei file ha le seguenti limitazioni:

| Limitation                             | Description                                                                                     |
| -------------------------------------- | ----------------------------------------------------------------------------------------------- |
| Solo strumenti Write/Edit/NotebookEdit | Le modifiche effettuate tramite comandi Bash non vengono tracciate                              |
| Stessa sessione                        | I checkpoint sono legati alla sessione che li ha creati                                         |
| Solo contenuto del file                | La creazione, lo spostamento o l'eliminazione di directory non vengono annullati dal ripristino |
| File locali                            | I file remoti o di rete non vengono tracciati                                                   |

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="checkpointing-options-not-recognized">
  Le opzioni di checkpointing non vengono riconosciute
</h3>

Se `enableFileCheckpointing` o `rewindFiles()` non sono disponibili, potresti essere su una versione SDK più vecchia.

**Soluzione**: Aggiorna alla versione SDK più recente:

* **Python**: `pip install --upgrade claude-agent-sdk`
* **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

<h3 id="user-messages-don’t-have-uuids">
  I messaggi dell'utente non hanno UUID
</h3>

Se `message.uuid` è `undefined` o mancante, non stai ricevendo gli UUID di checkpoint.

**Causa**: L'opzione `replay-user-messages` non è impostata.

**Soluzione**: Aggiungi `extra_args={"replay-user-messages": None}` (Python) o `extraArgs: { 'replay-user-messages': null }` (TypeScript) alle tue opzioni.

<h3 id="no-file-checkpoint-found-for-message-error">
  Errore "No file checkpoint found for message"
</h3>

Questo errore si verifica quando i dati di checkpoint non esistono per l'UUID del messaggio dell'utente specificato.

**Cause comuni**:

* Il checkpointing dei file non era abilitato sulla sessione originale (`enable_file_checkpointing` o `enableFileCheckpointing` non era impostato su `true`)
* La sessione non è stata completata correttamente prima di tentare di riprendere e ripristinare

**Soluzione**: Assicurati che `enable_file_checkpointing=True` (Python) o `enableFileCheckpointing: true` (TypeScript) fosse impostato sulla sessione originale, quindi utilizza il modello mostrato negli esempi: acquisisci il primo UUID del messaggio dell'utente, completa la sessione completamente, quindi riprendi con un prompt vuoto e chiama `rewindFiles()` una volta.

<h3 id="file-rewinding-is-not-enabled-error">
  Errore "File rewinding is not enabled"
</h3>

Questo errore si verifica quando tenti un rewind non interattivo senza checkpointing abilitato: eseguire il bare `claude -p` con `--rewind-files`, o eseguire una sessione SDK, inclusa una ripresa, le cui opzioni non abilitano il checkpointing. L'SDK imposta la variabile di ambiente `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` internamente solo quando `enable_file_checkpointing` (Python) o `enableFileCheckpointing` (TypeScript) è abilitato sulla sessione che esegue il rewind; il bare CLI non la imposta mai.

**Soluzione**: Per il bare CLI, imposta la variabile di ambiente quando esegui il comando:

```bash theme={null}
CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
```

Per l'SDK, imposta `enable_file_checkpointing=True` (Python) o `enableFileCheckpointing: true` (TypeScript) sulla sessione ripresa, come fanno gli esempi su questa pagina.

<h3 id="processtransport-is-not-ready-for-writing-error">
  Errore "ProcessTransport is not ready for writing"
</h3>

Questo errore si verifica quando chiami `rewindFiles()` o `rewind_files()` dopo aver terminato l'iterazione attraverso la risposta. La connessione al processo CLI si chiude quando il ciclo si completa.

**Soluzione**: Riprendi la sessione con un prompt vuoto, quindi chiama rewind sulla nuova query:

<CodeGroup>
  ```python Python theme={null}
  # Resume session with empty prompt, then rewind
  async with ClaudeSDKClient(
      ClaudeAgentOptions(enable_file_checkpointing=True, resume=session_id)
  ) as client:
      await client.query("")
      async for message in client.receive_response():
          await client.rewind_files(checkpoint_id)
          break
  ```

  ```typescript TypeScript theme={null}
  // Resume session with empty prompt, then rewind
  const rewindQuery = query({
    prompt: "",
    options: { ...opts, resume: sessionId }
  });

  for await (const msg of rewindQuery) {
    await rewindQuery.rewindFiles(checkpointId);
    break;
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Passaggi successivi
</h2>

* **[Sessions](/docs/it/agent-sdk/sessions)**: scopri come riprendere le sessioni, che è obbligatorio per il ripristino dopo il completamento del flusso. Copre gli ID di sessione, la ripresa delle conversazioni e il forking delle sessioni.
* **[Permissions](/docs/it/agent-sdk/permissions)**: configura quali strumenti Claude può utilizzare e come vengono approvate le modifiche ai file. Utile se vuoi più controllo su quando avvengono le modifiche.
* **[TypeScript SDK reference](/docs/it/agent-sdk/typescript)**: riferimento API completo incluse tutte le opzioni per `query()` e il metodo `rewindFiles()`.
* **[Python SDK reference](/docs/it/agent-sdk/python)**: riferimento API completo incluse tutte le opzioni per `ClaudeAgentOptions` e il metodo `rewind_files()`.
