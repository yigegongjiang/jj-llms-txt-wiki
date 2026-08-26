> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Revertir cambios de archivos con checkpointing

> Rastrear cambios de archivos durante sesiones de agente y restaurar archivos a cualquier estado anterior

El checkpointing de archivos rastrea las modificaciones de archivos realizadas a través de las herramientas Write, Edit y NotebookEdit durante una sesión de agente, lo que le permite revertir archivos a cualquier estado anterior. ¿Desea probarlo? Salte al [ejemplo interactivo](#try-it-out).

Con checkpointing, puede:

* **Deshacer cambios no deseados** restaurando archivos a un estado conocido y bueno
* **Explorar alternativas** restaurando a un checkpoint e intentando un enfoque diferente
* **Recuperarse de errores** cuando el agente realiza modificaciones incorrectas

<Warning>
  Solo se rastrean los cambios realizados a través de las herramientas Write, Edit y NotebookEdit. Los cambios realizados a través de comandos Bash (como `echo > file.txt` o `sed -i`) no se capturan en el sistema de checkpoint.
</Warning>

<h2 id="how-checkpointing-works">
  Cómo funciona el checkpointing
</h2>

Cuando habilita el checkpointing de archivos, el SDK crea copias de seguridad de archivos antes de modificarlos a través de las herramientas Write, Edit o NotebookEdit. Los mensajes de usuario en el flujo de respuesta incluyen un UUID de checkpoint que puede usar como punto de restauración.

Checkpoint funciona con estas herramientas integradas que el agente usa para modificar archivos:

| Herramienta  | Descripción                                                                  |
| ------------ | ---------------------------------------------------------------------------- |
| Write        | Crea un archivo nuevo o sobrescribe un archivo existente con contenido nuevo |
| Edit         | Realiza ediciones dirigidas a partes específicas de un archivo existente     |
| NotebookEdit | Modifica celdas en cuadernos Jupyter (archivos `.ipynb`)                     |

<Note>
  La reversión de archivos restaura archivos en disco a un estado anterior. No revierte la conversación en sí. El historial de conversación y el contexto permanecen intactos después de llamar a `rewindFiles()` (TypeScript) o `rewind_files()` (Python).
</Note>

El sistema de checkpoint rastrea:

* Archivos creados durante la sesión
* Archivos modificados durante la sesión
* El contenido original de archivos modificados

Cuando revierte a un checkpoint, los archivos creados se eliminan y los archivos modificados se restauran a su contenido en ese punto.

<h2 id="implement-checkpointing">
  Implementar checkpointing
</h2>

Para usar el checkpointing de archivos, habilítelo en sus opciones, capture UUIDs de checkpoint del flujo de respuesta, luego llame a `rewindFiles()` (TypeScript) o `rewind_files()` (Python) cuando necesite restaurar.

El siguiente ejemplo muestra el flujo completo: habilitar checkpointing, capturar el UUID de checkpoint y el ID de sesión del flujo de respuesta, luego reanudar la sesión más tarde para revertir archivos. Cada paso se explica en detalle a continuación. Los ejemplos en esta sección utilizan el mensaje "Refactor the authentication module". Ejecútelos en un proyecto que contenga un módulo de autenticación, o cambie el mensaje para nombrar archivos que existan en su proyecto, para que pueda ver cambios de archivos y ver cómo la reversión los restaura.

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
  <Step title="Habilitar checkpointing">
    Configure sus opciones de SDK para habilitar checkpointing y recibir UUIDs de checkpoint:

    | Opción                      | Python                                      | TypeScript                                    | Descripción                                                     |
    | --------------------------- | ------------------------------------------- | --------------------------------------------- | --------------------------------------------------------------- |
    | Habilitar checkpointing     | `enable_file_checkpointing=True`            | `enableFileCheckpointing: true`               | Rastrea cambios de archivos para reversión                      |
    | Recibir UUIDs de checkpoint | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Requerido para obtener UUIDs de mensajes de usuario en el flujo |

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

  <Step title="Capturar UUID de checkpoint e ID de sesión">
    Con la opción `replay-user-messages` establecida (mostrada arriba), cada mensaje de usuario en el flujo de respuesta tiene un UUID que sirve como checkpoint.

    Para la mayoría de los casos de uso, capture el UUID del primer mensaje de usuario (`message.uuid`); revertir a él restaura todos los archivos a su estado original. Para almacenar múltiples checkpoints y revertir a estados intermedios, consulte [Múltiples puntos de restauración](#multiple-restore-points).

    Capturar el ID de sesión (`message.session_id`) es opcional; solo lo necesita si desea revertir más tarde, después de que se complete el flujo. Si está llamando a `rewindFiles()` inmediatamente mientras aún procesa mensajes (como lo hace el ejemplo en [Checkpoint antes de operaciones arriesgadas](#checkpoint-before-risky-operations)), puede omitir la captura del ID de sesión.

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

  <Step title="Revertir archivos">
    Para revertir después de que se complete el flujo, reanude la sesión con un mensaje vacío y llame a `rewind_files()` (Python) o `rewindFiles()` (TypeScript) con su UUID de checkpoint. También puede revertir durante el flujo; consulte [Checkpoint antes de operaciones arriesgadas](#checkpoint-before-risky-operations) para ese patrón.

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

    Si captura el ID de sesión y el ID de checkpoint, también puede revertir desde la CLI. Este comando requiere el ejecutable `claude`, que viene de [instalar Claude Code](/docs/es/setup) y no está instalado por el paquete SDK. El SDK habilita checkpointing para usted, pero cuando ejecuta `claude -p` directamente debe establecer la variable de entorno `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`:

    ```bash theme={null}
    CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
    ```

    La bandera `--rewind-files` no aparece en la salida de `claude --help`, pero la CLI la acepta como se muestra.
  </Step>
</Steps>

<h2 id="common-patterns">
  Patrones comunes
</h2>

Estos patrones muestran diferentes formas de capturar y usar UUIDs de checkpoint según su caso de uso.

<h3 id="checkpoint-before-risky-operations">
  Checkpoint antes de operaciones arriesgadas
</h3>

Este patrón mantiene solo el UUID de checkpoint más reciente, actualizándolo antes de cada turno del agente. Si algo sale mal durante el procesamiento, puede revertir inmediatamente al último estado seguro y salir del bucle.

Antes de ejecutar este ejemplo, reemplace `your_revert_condition` (Python) o `yourRevertCondition` (TypeScript) con su propia verificación, como detección de errores o un fallo de validación; el marcador de posición no está definido en el ejemplo.

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
  Múltiples puntos de restauración
</h3>

Si Claude realiza cambios en múltiples turnos, es posible que desee revertir a un punto específico en lugar de volver completamente. Por ejemplo, si Claude refactoriza un archivo en el turno uno y agrega pruebas en el turno dos, es posible que desee mantener la refactorización pero deshacer las pruebas.

Este patrón almacena todos los UUIDs de checkpoint en una matriz con metadatos. Después de que se complete la sesión, puede revertir a cualquier checkpoint anterior:

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
  Pruébelo
</h2>

Este ejemplo completo crea un pequeño archivo de utilidad, hace que el agente agregue comentarios de documentación, le muestra los cambios, luego pregunta si desea revertir.

Antes de comenzar, asegúrese de tener el [Claude Agent SDK instalado](/docs/es/agent-sdk/quickstart).

<Steps>
  <Step title="Crear un archivo de prueba">
    Cree un nuevo archivo llamado `utils.py` (Python) o `utils.ts` (TypeScript) y pegue el siguiente código:

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

  <Step title="Ejecutar el ejemplo interactivo">
    Cree un nuevo archivo llamado `try_checkpointing.py` (Python) o `try_checkpointing.ts` (TypeScript) en el mismo directorio que su archivo de utilidad, y pegue el siguiente código.

    Este script le pide a Claude que agregue comentarios de documentación a su archivo de utilidad, luego le da la opción de revertir y restaurar el original.

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

    Este ejemplo demuestra el flujo de trabajo completo de checkpointing:

    1. **Habilitar checkpointing**: configure el SDK con `enable_file_checkpointing=True` y `permission_mode="acceptEdits"` para aprobar automáticamente ediciones de archivos
    2. **Capturar datos de checkpoint**: mientras el agente se ejecuta, almacene el UUID del primer mensaje de usuario (su punto de restauración) y el ID de sesión
    3. **Solicitar reversión**: después de que el agente termine, verifique su archivo de utilidad para ver los comentarios de documentación, luego decida si desea deshacer los cambios
    4. **Reanudar y revertir**: si es así, reanude la sesión con un mensaje vacío y llame a `rewind_files()` para restaurar el archivo original
  </Step>

  <Step title="Ejecutar el ejemplo">
    Ejecute el script desde el mismo directorio que su archivo de utilidad.

    <Tip>
      Abra su archivo de utilidad (`utils.py` o `utils.ts`) en su IDE o editor antes de ejecutar el script. Verá que el archivo se actualiza en tiempo real mientras el agente agrega comentarios de documentación, luego revierte al original cuando elige revertir.
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

    Verá que el agente agrega comentarios de documentación, luego un mensaje preguntando si desea revertir. Si elige sí, el archivo se restaura a su estado original.
  </Step>
</Steps>

<h2 id="limitations">
  Limitaciones
</h2>

El checkpointing de archivos tiene las siguientes limitaciones:

| Limitación                                | Descripción                                                     |
| ----------------------------------------- | --------------------------------------------------------------- |
| Solo herramientas Write/Edit/NotebookEdit | Los cambios realizados a través de comandos Bash no se rastrean |
| Misma sesión                              | Los checkpoints están vinculados a la sesión que los creó       |
| Solo contenido de archivo                 | Crear, mover o eliminar directorios no se deshace al revertir   |
| Archivos locales                          | Los archivos remotos o de red no se rastrean                    |

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="checkpointing-options-not-recognized">
  Las opciones de checkpointing no se reconocen
</h3>

Si `enableFileCheckpointing` o `rewindFiles()` no está disponible, es posible que esté en una versión anterior del SDK.

**Solución**: Actualice a la última versión del SDK:

* **Python**: `pip install --upgrade claude-agent-sdk`
* **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

<h3 id="user-messages-don’t-have-uuids">
  Los mensajes de usuario no tienen UUIDs
</h3>

Si `message.uuid` es `undefined` o está faltando, no está recibiendo UUIDs de checkpoint.

**Causa**: La opción `replay-user-messages` no está establecida.

**Solución**: Agregue `extra_args={"replay-user-messages": None}` (Python) o `extraArgs: { 'replay-user-messages': null }` (TypeScript) a sus opciones.

<h3 id="no-file-checkpoint-found-for-message-error">
  Error "No file checkpoint found for message"
</h3>

Este error ocurre cuando los datos de checkpoint no existen para el UUID de mensaje de usuario especificado.

**Causas comunes**:

* El checkpointing de archivos no estaba habilitado en la sesión original (`enable_file_checkpointing` o `enableFileCheckpointing` no estaba establecido en `true`)
* La sesión no se completó correctamente antes de intentar reanudar y revertir

**Solución**: Asegúrese de que `enable_file_checkpointing=True` (Python) o `enableFileCheckpointing: true` (TypeScript) estuviera establecido en la sesión original, luego use el patrón mostrado en los ejemplos: capture el UUID del primer mensaje de usuario, complete la sesión completamente, luego reanude con un mensaje vacío y llame a `rewindFiles()` una sola vez.

<h3 id="file-rewinding-is-not-enabled-error">
  Error "File rewinding is not enabled"
</h3>

Este error ocurre cuando intenta una reversión no interactiva sin checkpointing habilitado: ejecutar `claude -p` simple con `--rewind-files`, o ejecutar una sesión del SDK, incluida una reanudada, cuyas opciones no habilitan checkpointing. El SDK establece la variable de entorno `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` internamente solo cuando `enable_file_checkpointing` (Python) o `enableFileCheckpointing` (TypeScript) está habilitado en la sesión que realiza la reversión; la CLI simple nunca la establece.

**Solución**: Para la CLI simple, establezca la variable de entorno al ejecutar el comando:

```bash theme={null}
CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true claude -p --resume <session-id> --rewind-files <checkpoint-uuid>
```

Para el SDK, establezca `enable_file_checkpointing=True` (Python) o `enableFileCheckpointing: true` (TypeScript) en la sesión reanudada, como lo hacen los ejemplos en esta página.

<h3 id="processtransport-is-not-ready-for-writing-error">
  Error "ProcessTransport is not ready for writing"
</h3>

Este error ocurre cuando llama a `rewindFiles()` o `rewind_files()` después de haber terminado de iterar a través de la respuesta. La conexión al proceso de CLI se cierra cuando se completa el bucle.

**Solución**: Reanude la sesión con un mensaje vacío, luego llame a rewind en la nueva consulta:

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
  Próximos pasos
</h2>

* **[Sessions](/docs/es/agent-sdk/sessions)**: aprenda cómo reanudar sesiones, que es necesario para revertir después de que se complete el flujo. Cubre IDs de sesión, reanudación de conversaciones y bifurcación de sesiones.
* **[Permissions](/docs/es/agent-sdk/permissions)**: configure qué herramientas puede usar Claude y cómo se aprueban las modificaciones de archivos. Útil si desea más control sobre cuándo ocurren las ediciones.
* **[TypeScript SDK reference](/docs/es/agent-sdk/typescript)**: referencia completa de API incluyendo todas las opciones para `query()` y el método `rewindFiles()`.
* **[Python SDK reference](/docs/es/agent-sdk/python)**: referencia completa de API incluyendo todas las opciones para `ClaudeAgentOptions` y el método `rewind_files()`.
