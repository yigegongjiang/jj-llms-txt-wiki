> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestionar aprobaciones e entrada de usuario

> Presente las solicitudes de aprobación y preguntas aclaratorias de Claude a los usuarios, luego devuelva sus decisiones al SDK.

Mientras trabaja en una tarea, Claude a veces necesita consultar con los usuarios. Podría necesitar permiso antes de eliminar archivos, o necesitar preguntar qué base de datos usar para un nuevo proyecto. Su aplicación necesita presentar estas solicitudes a los usuarios para que Claude pueda continuar con su entrada.

Claude solicita entrada del usuario en dos situaciones: cuando necesita **permiso para usar una herramienta** (como eliminar archivos o ejecutar comandos), y cuando tiene **preguntas aclaratorias** (a través de la herramienta `AskUserQuestion`). Ambas activan su callback `canUseTool`, que pausa la ejecución hasta que devuelva una respuesta. Esto es diferente de los turnos de conversación normales donde Claude termina y espera su próximo mensaje.

Para preguntas aclaratorias, Claude genera las preguntas y opciones. Su función es presentarlas a los usuarios y devolver sus selecciones. No puede agregar sus propias preguntas a este flujo; si necesita preguntarle algo a los usuarios usted mismo, hágalo por separado en la lógica de su aplicación.

El callback puede permanecer pendiente indefinidamente. La ejecución permanece pausada hasta que su callback regrese. Si un usuario podría tardar más en responder de lo que su proceso puede razonablemente mantenerse ejecutando, registre un [hook `PreToolUse`](/docs/es/agent-sdk/hooks) que devuelva la [decisión `defer`](/docs/es/hooks#defer-a-tool-call-for-later) en lugar de esperar en el callback, para que el proceso pueda salir y reanudarse más tarde desde la sesión persistida.

Esta guía le muestra cómo detectar cada tipo de solicitud y responder apropiadamente.

<h2 id="detect-when-claude-needs-input">
  Detectar cuándo Claude necesita entrada
</h2>

Pase un callback `canUseTool` en sus opciones de consulta. El callback se activa cada vez que Claude necesita entrada del usuario, recibiendo el nombre de la herramienta y la entrada como argumentos:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import ClaudeAgentOptions


  async def handle_tool_request(tool_name, input_data, context):
      # Solicitar al usuario y devolver permitir o denegar
      ...


  options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
  ```

  ```typescript TypeScript theme={null}
  async function handleToolRequest(toolName, input, options) {
    // options includes { signal: AbortSignal, suggestions?: PermissionUpdate[] }
    // Solicitar al usuario y devolver permitir o denegar
  }

  const options = { canUseTool: handleToolRequest };
  ```
</CodeGroup>

El callback se activa en dos casos:

1. **La herramienta necesita aprobación**: Claude quiere usar una herramienta que no está aprobada automáticamente por una [regla de permisos](/docs/es/agent-sdk/permissions) o modo de permisos. Verifique `tool_name` para la herramienta (por ejemplo, `"Bash"`, `"Write"`).
2. **Claude hace una pregunta**: Claude llama a la herramienta `AskUserQuestion`. Verifique si `tool_name == "AskUserQuestion"` para manejarlo de manera diferente. Si especifica un array `tools`, incluya `AskUserQuestion` para que esto funcione. Vea [Manejar preguntas aclaratorias](#handle-clarifying-questions) para más detalles.

<Warning>
  **El callback nunca se activa para herramientas aprobadas automáticamente.** Cualquier aprobación anterior en el [flujo de evaluación de permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated), una regla de permiso o un modo como `acceptEdits` o `bypassPermissions`, resuelve la llamada antes de que se consulte `canUseTool`. Si enumera una herramienta directamente en `allowed_tools`, una verificación `canUseTool` para esa herramienta se ejecuta solo cuando el [flujo de evaluación](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) redirija la llamada de vuelta a un prompt, como una regla de pregunta o modo `plan`. Para lógica que debe aplicarse a cada llamada de herramienta, use un [hook `PreToolUse`](/docs/es/agent-sdk/hooks), que se ejecuta antes del resto del flujo y puede permitir, denegar o modificar solicitudes.

  Una regla de permiso no aprueba previamente las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves); vea [Cómo se evalúan los permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) para saber cuáles de ellas llegan al callback y qué sucede en modo `dontAsk` y `auto`.
</Warning>

También puede usar el [hook `PermissionRequest`](/docs/es/agent-sdk/hooks#available-hooks) para enviar notificaciones externas (Slack, correo electrónico, push) cuando Claude está esperando aprobación.

<h2 id="handle-tool-approval-requests">
  Manejar solicitudes de aprobación de herramientas
</h2>

Una vez que haya pasado una devolución de llamada `canUseTool` en las opciones de su consulta, se activa cuando Claude quiere usar una herramienta que nada anterior en el flujo de permisos ha aprobado. En algunas configuraciones, como el modo `dontAsk`, Claude Code no la llama; el último paso de [Cómo se evalúan los permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) los enumera y dice qué sucede con la llamada en su lugar.

Su devolución de llamada recibe tres argumentos:

| Argumento                           | Descripción                                                                                                                                                                                                                                                                                                                                                 |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `toolName`                          | El nombre de la herramienta que Claude quiere usar (por ejemplo, `"Bash"`, `"Write"`, `"Edit"`)                                                                                                                                                                                                                                                             |
| `input`                             | Los parámetros que Claude está pasando a la herramienta. El contenido varía según la herramienta.                                                                                                                                                                                                                                                           |
| `options` (TS) / `context` (Python) | Contexto adicional que incluye `suggestions` opcionales (entradas `PermissionUpdate` propuestas para evitar re-solicitar) y una señal de cancelación. En TypeScript, `signal` es un `AbortSignal`; en Python, el campo de señal está reservado para uso futuro. Consulte [`ToolPermissionContext`](/docs/es/agent-sdk/python#toolpermissioncontext) para Python. |

El objeto `input` contiene parámetros específicos de la herramienta. Ejemplos comunes:

| Herramienta | Campos de entrada                       |
| ----------- | --------------------------------------- |
| `Bash`      | `command`, `description`, `timeout`     |
| `Write`     | `file_path`, `content`                  |
| `Edit`      | `file_path`, `old_string`, `new_string` |
| `Read`      | `file_path`, `offset`, `limit`          |

Consulte la referencia del SDK para esquemas de entrada completos: [Python](/docs/es/agent-sdk/python#tool-input%2Foutput-types) | [TypeScript](/docs/es/agent-sdk/typescript#tool-input-types).

Puede mostrar esta información al usuario para que pueda decidir si permitir o rechazar la acción, y luego devolver la respuesta apropiada.

El siguiente ejemplo le pide a Claude que cree y elimine un archivo de prueba. Cuando Claude intenta cada operación, la devolución de llamada imprime la solicitud de herramienta en la terminal y solicita aprobación s/n.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import (
      HookMatcher,
      PermissionResultAllow,
      PermissionResultDeny,
      ToolPermissionContext,
  )


  async def can_use_tool(
      tool_name: str, input_data: dict, context: ToolPermissionContext
  ) -> PermissionResultAllow | PermissionResultDeny:
      # Display the tool request
      print(f"\nTool: {tool_name}")
      if tool_name == "Bash":
          print(f"Command: {input_data.get('command')}")
          if input_data.get("description"):
              print(f"Description: {input_data.get('description')}")
      else:
          print(f"Input: {input_data}")

      # Get user approval
      response = input("Allow this action? (y/n): ")

      # Return allow or deny based on user's response
      if response.lower() == "y":
          # Allow: tool executes with the original (or modified) input
          return PermissionResultAllow(updated_input=input_data)
      else:
          # Deny: tool doesn't execute, Claude sees the message
          return PermissionResultDeny(message="User denied this action")


  # Required workaround: dummy hook keeps the stream open for can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Create a test file in /tmp and then delete it",
          },
      }


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline";

  // Helper to prompt user for input in the terminal
  function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    return new Promise((resolve) =>
      rl.question(question, (answer) => {
        rl.close();
        resolve(answer);
      })
    );
  }

  for await (const message of query({
    prompt: "Create a test file in /tmp and then delete it",
    options: {
      canUseTool: async (toolName, input) => {
        // Display the tool request
        console.log(`\nTool: ${toolName}`);
        if (toolName === "Bash") {
          console.log(`Command: ${input.command}`);
          if (input.description) console.log(`Description: ${input.description}`);
        } else {
          console.log(`Input: ${JSON.stringify(input, null, 2)}`);
        }

        // Get user approval
        const response = await prompt("Allow this action? (y/n): ");

        // Return allow or deny based on user's response
        if (response.toLowerCase() === "y") {
          // Allow: tool executes with the original (or modified) input
          return { behavior: "allow", updatedInput: input };
        } else {
          // Deny: tool doesn't execute, Claude sees the message
          return { behavior: "deny", message: "User denied this action" };
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

Este ejemplo utiliza un flujo s/n donde cualquier entrada que no sea `s` se trata como un rechazo. En la práctica, podría crear una interfaz de usuario más rica que permita a los usuarios modificar la solicitud, proporcionar comentarios o redirigir a Claude completamente. Consulte [Responder a solicitudes de herramientas](#respond-to-tool-requests) para ver todas las formas en que puede responder.

<h3 id="respond-to-tool-requests">
  Responder a solicitudes de herramientas
</h3>

Su devolución de llamada devuelve uno de dos tipos de respuesta:

| Respuesta    | Python                                     | TypeScript                            |
| ------------ | ------------------------------------------ | ------------------------------------- |
| **Permitir** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **Denegar**  | `PermissionResultDeny(message=...)`        | `{ behavior: "deny", message }`       |

Al permitir, la herramienta se ejecuta con la entrada que Claude solicitó a menos que devuelva una entrada modificada, `updatedInput` en TypeScript o `updated_input` en Python. Antes de v2.1.207, Claude Code rechazaba un resultado de permiso que omitía `updatedInput` y denegaba la llamada de herramienta con un error de validación.

Al denegar, proporcione un mensaje explicando por qué. Claude ve este mensaje y puede ajustar su enfoque.

Más allá de permitir o denegar, puede modificar la entrada de la herramienta o proporcionar contexto que ayude a Claude a ajustar su enfoque:

* **Aprobar**: permitir que la herramienta se ejecute como Claude solicitó
* **Aprobar con cambios**: modificar la entrada antes de la ejecución (por ejemplo, sanitizar rutas, agregar restricciones)
* **Aprobar y recordar**: devolver una regla de permiso sugerida para que las llamadas coincidentes omitan la solicitud la próxima vez
* **Rechazar**: bloquear la herramienta y decirle a Claude por qué
* **Sugerir alternativa**: bloquear pero guiar a Claude hacia lo que el usuario quiere en su lugar
* **Redirigir completamente**: usar [entrada de transmisión](/docs/es/agent-sdk/streaming-vs-single-mode) para enviar a Claude una instrucción completamente nueva

Los ayudantes `ask_user` y `askUser` en los siguientes fragmentos representan la interfaz de usuario de solicitud de su propia aplicación.

<Tabs>
  <Tab title="Aprobar">
    El usuario aprueba la acción tal como está. Pase la `input` de su devolución de llamada sin cambios y la herramienta se ejecuta exactamente como Claude solicitó.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          print(f"Claude wants to use {tool_name}")
          approved = await ask_user("Allow this action?")

          if approved:
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        console.log(`Claude wants to use ${toolName}`);
        const approved = await askUser("Allow this action?");

        if (approved) {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Aprobar con cambios">
    El usuario aprueba pero quiere modificar la solicitud primero. Puede cambiar la entrada antes de que se ejecute la herramienta. Claude ve el resultado pero no se le dice que haya cambiado nada. Útil para sanitizar parámetros, agregar restricciones o limitar el acceso.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash":
              # User approved, but scope all commands to sandbox
              sandboxed_input = {**input_data}
              sandboxed_input["command"] = input_data["command"].replace(
                  "/tmp", "/tmp/sandbox"
              )
              return PermissionResultAllow(updated_input=sandboxed_input)
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash") {
          // User approved, but scope all commands to sandbox
          const sandboxedInput = {
            ...input,
            command: input.command.replace("/tmp", "/tmp/sandbox")
          };
          return { behavior: "allow", updatedInput: sandboxedInput };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Aprobar y recordar">
    El usuario aprueba y no quiere que se le pregunte de nuevo para este tipo de llamada. El tercer argumento de devolución de llamada lleva `suggestions`, una matriz de entradas [`PermissionUpdate`](/docs/es/agent-sdk/typescript#permissionupdate) ya preparadas. Devuelva una en `updatedPermissions` para aplicarla. Una sugerencia con el destino `localSettings` escribe la regla en `.claude/settings.local.json` para que futuras sesiones omitan la solicitud para llamadas coincidentes.

    El ejemplo de Python requiere `claude-agent-sdk` 0.1.80 o posterior.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])

          if choice == "always":
              persist = [
                  s for s in context.suggestions if s.destination == "localSettings"
              ]
              return PermissionResultAllow(
                  updated_input=input_data, updated_permissions=persist
              )
          if choice == "once":
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input, { suggestions = [] }) => {
        const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);

        if (choice === "always") {
          const persist = suggestions.filter(
            (s) => s.destination === "localSettings"
          );
          return {
            behavior: "allow",
            updatedInput: input,
            updatedPermissions: persist
          };
        }
        if (choice === "once") {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Rechazar">
    El usuario no quiere que esta acción suceda. Bloquee la herramienta y proporcione un mensaje explicando por qué. Claude ve este mensaje y puede intentar un enfoque diferente.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          approved = await ask_user(f"Allow {tool_name}?")

          if not approved:
              return PermissionResultDeny(message="User rejected this action")
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        const approved = await askUser(`Allow ${toolName}?`);

        if (!approved) {
          return {
            behavior: "deny",
            message: "User rejected this action"
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Sugerir alternativa">
    El usuario no quiere esta acción específica, pero tiene una idea diferente. Bloquee la herramienta e incluya orientación en su mensaje. Claude leerá esto y decidirá cómo proceder según sus comentarios.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash" and "rm" in input_data.get("command", ""):
              # User doesn't want to delete, suggest archiving instead
              return PermissionResultDeny(
                  message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
              )
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash" && input.command.includes("rm")) {
          // User doesn't want to delete, suggest archiving instead
          return {
            behavior: "deny",
            message:
              "User doesn't want to delete files. They asked if you could compress them into an archive instead."
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Redirigir completamente">
    Para un cambio de dirección completo (no solo un empujón), use [entrada de transmisión](/docs/es/agent-sdk/streaming-vs-single-mode) para enviar a Claude una nueva instrucción directamente. Esto omite la solicitud de herramienta actual y le da a Claude instrucciones completamente nuevas para seguir.
  </Tab>
</Tabs>

<h2 id="handle-clarifying-questions">
  Manejar preguntas aclaratorias
</h2>

Cuando Claude necesita más dirección en una tarea con múltiples enfoques válidos, llama a la herramienta `AskUserQuestion`. Esto activa su callback `canUseTool` con `toolName` establecido en `AskUserQuestion`. La entrada contiene las preguntas de Claude como opciones de opción múltiple, que muestra al usuario y devuelve sus selecciones.

<Tip>
  Las preguntas aclaratorias son especialmente comunes en [`plan` mode](/docs/es/agent-sdk/permissions#plan-mode-plan), donde Claude explora la base de código y hace preguntas antes de proponer un plan. Esto hace que el modo plan sea ideal para flujos de trabajo interactivos donde desea que Claude recopile requisitos antes de hacer cambios.
</Tip>

Los siguientes pasos muestran cómo manejar preguntas aclaratorias:

<Steps>
  <Step title="Pasar un callback canUseTool">
    Pase un callback `canUseTool` en sus opciones de consulta. De forma predeterminada, `AskUserQuestion` está disponible. Si especifica un array `tools` para restringir las capacidades de Claude (por ejemplo, un agente de solo lectura con solo `Read`, `Glob` y `Grep`), incluya `AskUserQuestion` en ese array. De lo contrario, Claude no podrá hacer preguntas aclaratorias:

    <CodeGroup>
      ```python Python theme={null}
      async for message in query(
          prompt="Analyze this codebase",
          options=ClaudeAgentOptions(
              # Incluya AskUserQuestion en su lista de herramientas
              tools=["Read", "Glob", "Grep", "AskUserQuestion"],
              can_use_tool=can_use_tool,
          ),
      ):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      for await (const message of query({
        prompt: "Analyze this codebase",
        options: {
          // Incluya AskUserQuestion en su lista de herramientas
          tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
          canUseTool: async (toolName, input) => {
            // Manejar preguntas aclaratorias aquí
          }
        }
      })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Detectar AskUserQuestion">
    En su callback, verifique si `toolName` es igual a `AskUserQuestion` para manejarlo de manera diferente a otras herramientas:

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name: str, input_data: dict, context):
          if tool_name == "AskUserQuestion":
              # Su implementación para recopilar respuestas del usuario
              return await handle_clarifying_questions(input_data)
          # Manejar otras herramientas normalmente
          return await prompt_for_approval(tool_name, input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "AskUserQuestion") {
          // Su implementación para recopilar respuestas del usuario
          return handleClarifyingQuestions(input);
        }
        // Manejar otras herramientas normalmente
        return promptForApproval(toolName, input);
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Analizar la entrada de la pregunta">
    La entrada contiene las preguntas de Claude en un array `questions`. Cada pregunta tiene una `question` (el texto a mostrar), `options` (las opciones) y `multiSelect` (si se permiten múltiples selecciones):

    ```json theme={null}
    {
      "questions": [
        {
          "question": "How should I format the output?",
          "header": "Format",
          "options": [
            { "label": "Summary", "description": "Brief overview" },
            { "label": "Detailed", "description": "Full explanation" }
          ],
          "multiSelect": false
        },
        {
          "question": "Which sections should I include?",
          "header": "Sections",
          "options": [
            { "label": "Introduction", "description": "Opening context" },
            { "label": "Conclusion", "description": "Final summary" }
          ],
          "multiSelect": true
        }
      ]
    }
    ```

    Vea [Formato de pregunta](#question-format) para descripciones completas de campos.
  </Step>

  <Step title="Recopilar respuestas del usuario">
    Presente las preguntas al usuario y recopile sus selecciones. Cómo lo hace depende de su aplicación: un indicador de terminal, un formulario web, un diálogo móvil, etc.
  </Step>

  <Step title="Devolver respuestas a Claude">
    Construya el objeto `answers` como un registro donde cada clave es el texto `question` y cada valor es la `label` de la opción seleccionada:

    | Del objeto de pregunta                                              | Usar como |
    | ------------------------------------------------------------------- | --------- |
    | Campo `question` (por ejemplo, `"How should I format the output?"`) | Clave     |
    | Campo `label` de la opción seleccionada (por ejemplo, `"Summary"`)  | Valor     |

    Para preguntas de selección múltiple, pase un array de etiquetas o únalas con `", "`. Si [admite entrada de texto libre](#support-free-text-input), use el texto personalizado del usuario como valor.

    <CodeGroup>
      ```python Python theme={null}
      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": {
                  "How should I format the output?": "Summary",
                  "Which sections should I include?": ["Introduction", "Conclusion"],
              },
          }
      )
      ```

      ```typescript TypeScript theme={null}
      return {
        behavior: "allow",
        updatedInput: {
          questions: input.questions,
          answers: {
            "How should I format the output?": "Summary",
            "Which sections should I include?": "Introduction, Conclusion"
          }
        }
      };
      ```
    </CodeGroup>
  </Step>
</Steps>

<h3 id="question-format">
  Formato de pregunta
</h3>

La entrada contiene las preguntas generadas por Claude en un array `questions`. Cada pregunta tiene estos campos:

| Campo         | Descripción                                                                                                                                                      |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `question`    | El texto completo de la pregunta a mostrar                                                                                                                       |
| `header`      | Etiqueta corta para la pregunta (máximo 12 caracteres)                                                                                                           |
| `options`     | Array de 2-4 opciones, cada una con `label` y `description`. TypeScript: opcionalmente `preview`. Vea [Vistas previas de opciones](#option-previews-typescript). |
| `multiSelect` | Si es `true`, los usuarios pueden seleccionar múltiples opciones                                                                                                 |

La estructura que su callback recibe:

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

<h4 id="option-previews-typescript">
  Vistas previas de opciones (TypeScript)
</h4>

`toolConfig.askUserQuestion.previewFormat` agrega un campo `preview` a cada opción para que su aplicación pueda mostrar una maqueta visual junto a la etiqueta. Sin esta configuración, Claude no genera vistas previas y el campo está ausente.

| `previewFormat`                 | `preview` contiene                                                                                                        |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------ |
| sin establecer (predeterminado) | El campo está ausente. Claude no genera vistas previas.                                                                   |
| `"markdown"`                    | Arte ASCII y bloques de código cercados                                                                                   |
| `"html"`                        | Un fragmento `<div>` con estilo (el SDK rechaza `<script>`, `<style>` y `<!DOCTYPE>` antes de que su callback se ejecute) |

El formato se aplica a todas las preguntas en la sesión. Claude incluye `preview` en opciones donde una comparación visual ayuda (opciones de diseño, esquemas de color) y la omite donde no lo haría (confirmaciones sí/no, opciones de solo texto). Verifique `undefined` antes de renderizar.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview es una cadena HTML o undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

Una opción con una vista previa HTML:

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

<h3 id="response-format">
  Formato de respuesta
</h3>

Devuelva un objeto `answers` que asigne cada campo `question` de la pregunta a la `label` de la opción seleccionada:

| Campo       | Descripción                                                                                                 |
| ----------- | ----------------------------------------------------------------------------------------------------------- |
| `questions` | Pase el array de preguntas original (requerido para el procesamiento de herramientas)                       |
| `answers`   | Objeto donde las claves son texto de pregunta y los valores son etiquetas seleccionadas                     |
| `response`  | Respuesta de texto libre opcional que el usuario escribió en lugar de responder las preguntas estructuradas |

Para preguntas de selección múltiple, pase un array de etiquetas o únalas con `", "`. Para entrada de texto libre por pregunta, como una opción "Otro", coloque el texto del usuario en `answers[question]` como se muestra en [Admitir entrada de texto libre](#support-free-text-input). Establezca `response` solo cuando su interfaz de usuario permita al usuario descartar la tarjeta de pregunta y escribir una respuesta general que no sea una respuesta a ninguna pregunta específica. Cuando `response` está establecido, Claude recibe "El usuario respondió: …" en lugar de la lista de respuestas por pregunta.

```jsonc theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

<h4 id="support-free-text-input">
  Admitir entrada de texto libre
</h4>

Las opciones predefinidas de Claude no siempre cubrirán lo que los usuarios quieren. Para permitir que los usuarios escriban su propia respuesta:

* Muestre una opción "Otro" adicional después de las opciones de Claude que acepte entrada de texto
* Use el texto personalizado del usuario como valor de respuesta (no la palabra "Otro")

Vea el [ejemplo completo](#complete-example) a continuación para una implementación completa.

<h3 id="complete-example">
  Ejemplo completo
</h3>

Claude hace preguntas aclaratorias cuando necesita entrada del usuario para proceder. Por ejemplo, cuando se le pide que ayude a decidir sobre una pila de tecnología para una aplicación móvil, Claude podría preguntar sobre multiplataforma vs nativo, preferencias de backend o plataformas objetivo. Estas preguntas ayudan a Claude a tomar decisiones que coincidan con las preferencias del usuario en lugar de adivinar.

Este ejemplo maneja esas preguntas en una aplicación de terminal. Esto es lo que sucede en cada paso:

1. **Enrutar la solicitud**: El callback `canUseTool` verifica si el nombre de la herramienta es `"AskUserQuestion"` y enruta a un manejador dedicado
2. **Mostrar preguntas**: El manejador recorre el array `questions` e imprime cada pregunta con opciones numeradas
3. **Recopilar entrada**: El usuario puede ingresar un número para seleccionar una opción, o escribir texto libre directamente (por ejemplo, "jquery", "i don't know")
4. **Asignar respuestas**: El código verifica si la entrada es numérica (usa la etiqueta de la opción) o texto libre (usa el texto directamente)
5. **Devolver a Claude**: La respuesta incluye tanto el array `questions` original como el mapeo `answers`

Guarde la versión de TypeScript como `ask.ts` y ejecútela con `npx tsx ask.ts`, o guarde la versión de Python como `ask.py` y ejecútela con `python ask.py`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


  def parse_response(response: str, options: list) -> str:
      """Analizar la entrada del usuario como número(s) de opción o texto libre."""
      try:
          indices = [int(s.strip()) - 1 for s in response.split(",")]
          labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
          return ", ".join(labels) if labels else response
      except ValueError:
          return response


  async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
      """Mostrar las preguntas de Claude y recopilar respuestas del usuario."""
      answers = {}

      for q in input_data.get("questions", []):
          print(f"\n{q['header']}: {q['question']}")

          options = q["options"]
          for i, opt in enumerate(options):
              print(f"  {i + 1}. {opt['label']} - {opt['description']}")
          if q.get("multiSelect"):
              print("  (Enter numbers separated by commas, or type your own answer)")
          else:
              print("  (Enter a number, or type your own answer)")

          response = input("Your choice: ").strip()
          answers[q["question"]] = parse_response(response, options)

      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": answers,
          }
      )


  async def can_use_tool(
      tool_name: str, input_data: dict, context
  ) -> PermissionResultAllow:
      # Enrutar AskUserQuestion a nuestro manejador de preguntas
      if tool_name == "AskUserQuestion":
          return await handle_ask_user_question(input_data)
      # Auto-aprobar otras herramientas para este ejemplo
      return PermissionResultAllow(updated_input=input_data)


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Help me decide on the tech stack for a new mobile app",
          },
      }


  # Solución requerida: hook ficticio mantiene el flujo abierto para can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline/promises";

  // Helper para solicitar entrada del usuario en la terminal
  async function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
    const answer = await rl.question(question);
    rl.close();
    return answer;
  }

  // Analizar la entrada del usuario como número(s) de opción o texto libre
  function parseResponse(response: string, options: any[]): string {
    const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
    const labels = indices
      .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
      .map((i) => options[i].label);
    return labels.length > 0 ? labels.join(", ") : response;
  }

  // Mostrar las preguntas de Claude y recopilar respuestas del usuario
  async function handleAskUserQuestion(input: any) {
    const answers: Record<string, string> = {};

    for (const q of input.questions) {
      console.log(`\n${q.header}: ${q.question}`);

      const options = q.options;
      options.forEach((opt: any, i: number) => {
        console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
      });
      if (q.multiSelect) {
        console.log("  (Enter numbers separated by commas, or type your own answer)");
      } else {
        console.log("  (Enter a number, or type your own answer)");
      }

      const response = (await prompt("Your choice: ")).trim();
      answers[q.question] = parseResponse(response, options);
    }

    // Devolver las respuestas a Claude (debe incluir preguntas originales)
    return {
      behavior: "allow",
      updatedInput: { questions: input.questions, answers }
    };
  }

  async function main() {
    for await (const message of query({
      prompt: "Help me decide on the tech stack for a new mobile app",
      options: {
        canUseTool: async (toolName, input) => {
          // Enrutar AskUserQuestion a nuestro manejador de preguntas
          if (toolName === "AskUserQuestion") {
            return handleAskUserQuestion(input);
          }
          // Auto-aprobar otras herramientas para este ejemplo
          return { behavior: "allow", updatedInput: input };
        }
      }
    })) {
      if ("result" in message) console.log(message.result);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="limitations">
  Limitaciones
</h2>

* **Subagentes**: `AskUserQuestion` no está disponible actualmente en subagentes generados a través de la herramienta Agent
* **Límites de preguntas**: cada llamada `AskUserQuestion` admite 1-4 preguntas con 2-4 opciones cada una

<h2 id="other-ways-to-get-user-input">
  Otras formas de obtener entrada del usuario
</h2>

El callback `canUseTool` y la herramienta `AskUserQuestion` cubren la mayoría de escenarios de aprobación y aclaración, pero el SDK ofrece otras formas de obtener entrada de los usuarios:

<h3 id="streaming-input">
  Entrada de flujo
</h3>

Use [entrada de flujo](/docs/es/agent-sdk/streaming-vs-single-mode) cuando necesite:

* **Interrumpir el agente a mitad de tarea**: enviar una señal de cancelación o cambiar de dirección mientras Claude está trabajando
* **Proporcionar contexto adicional**: agregar información que Claude necesita sin esperar a que la solicite
* **Construir interfaces de chat**: permitir que los usuarios envíen mensajes de seguimiento durante operaciones de larga duración

La entrada de flujo es ideal para interfaces conversacionales donde los usuarios interactúan con el agente durante toda la ejecución, no solo en puntos de aprobación.

<h3 id="custom-tools">
  Herramientas personalizadas
</h3>

Use [herramientas personalizadas](/docs/es/agent-sdk/custom-tools) cuando necesite:

* **Recopilar entrada estructurada**: construir formularios, asistentes o flujos de trabajo de varios pasos que vayan más allá del formato de opción múltiple de `AskUserQuestion`
* **Integrar sistemas de aprobación externos**: conectarse a plataformas de tickets, flujo de trabajo o aprobación existentes
* **Implementar interacciones específicas del dominio**: crear herramientas adaptadas a las necesidades de su aplicación, como interfaces de revisión de código o listas de verificación de implementación

Las herramientas personalizadas le dan control total sobre la interacción, pero requieren más trabajo de implementación que usar el callback `canUseTool` integrado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Configurar permisos](/docs/es/agent-sdk/permissions): configurar modos y reglas de permisos
* [Controlar la ejecución con hooks](/docs/es/agent-sdk/hooks): ejecutar código personalizado en puntos clave del ciclo de vida del agente
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript#canusetool): documentación completa de la API canUseTool
