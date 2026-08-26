> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar permisos

> Controle cómo su agente utiliza herramientas con modos de permiso, hooks y reglas declarativas de permitir/denegar.

El SDK del Agente Claude proporciona controles de permisos para gestionar cómo Claude utiliza las herramientas. Utilice modos de permiso y reglas para definir qué está permitido automáticamente, y la devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) para manejar todo lo demás en tiempo de ejecución.

<Note>
  Esta página cubre modos de permiso y reglas. Para crear flujos de aprobación interactivos donde los usuarios aprueban o deniegan solicitudes de herramientas en tiempo de ejecución, consulte [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input).
</Note>

<h2 id="how-permissions-are-evaluated">
  Cómo se evalúan los permisos
</h2>

Cuando Claude solicita una herramienta, el SDK verifica los permisos en este orden:

<Steps>
  <Step title="Hooks">
    Ejecute [hooks](/docs/es/agent-sdk/hooks) primero. Un hook puede denegar la llamada directamente o pasarla. Un hook que devuelve `allow` no omite las reglas de denegar y preguntar a continuación; esas se evalúan independientemente del resultado del hook.
  </Step>

  <Step title="Reglas de denegar">
    Verifique las reglas `deny` (de `disallowed_tools` y [settings.json](/docs/es/settings#permission-settings)). Si una regla de denegar coincide, la herramienta se bloquea, incluso en modo `bypassPermissions`. Las reglas de nombre simple como `Bash` eliminan la herramienta del contexto de Claude antes de que comience esta evaluación, por lo que solo se verifican las reglas con alcance como `Bash(rm *)` en este paso.
  </Step>

  <Step title="Reglas de preguntar">
    Verifique las reglas `ask` de [settings.json](/docs/es/settings#permission-settings). Si una regla de preguntar coincide, la llamada se pasa a su devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) para confirmación, incluso en modo `bypassPermissions`.

    Las herramientas que requieren interacción del usuario se comportan de la misma manera: `AskUserQuestion` y las herramientas MCP cuyo servidor establece [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) siempre se pasan a la devolución de llamada, incluso cuando una regla de permitir coincide. En modo `dontAsk` ambos casos se deniegan en su lugar, porque ese modo nunca solicita confirmación. La anotación MCP requiere Claude Code v2.1.199 o posterior.

    Las herramientas del conector [claude.ai](/docs/es/mcp#organization-controls-on-connector-tools) que su organización ha establecido en `ask` también salen del flujo en este paso. Cada llamada se pasa a la devolución de llamada, incluso en modo `bypassPermissions` y incluso cuando una regla de permitir coincide. La devolución de llamada recibe la razón `Your organization requires approval for this tool`. En modo `dontAsk` la llamada se deniega en su lugar, porque ese modo nunca solicita confirmación.
  </Step>

  <Step title="Modo de permiso">
    Aplique el [modo de permiso](#permission-modes) activo. `bypassPermissions` aprueba todo lo que llega a este paso. `acceptEdits` aprueba operaciones de archivo. `plan` enruta herramientas de edición de archivo y escritura de shell a su devolución de llamada `canUseTool` independientemente de las reglas de permitir, por lo que las operaciones de escritura no pueden ser aprobadas automáticamente mientras se planifica. Otros modos se descartan.
  </Step>

  <Step title="Reglas de permitir">
    Verifique las reglas `allow` (de `allowed_tools` y settings.json). Si una regla coincide, la herramienta se aprueba.
  </Step>

  <Step title="Devolución de llamada canUseTool">
    Si no se resuelve por ninguno de los anteriores, llame a su devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) para una decisión. En modo `dontAsk`, este paso se omite y la herramienta se deniega.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="Diagrama del flujo de evaluación de permisos de seis pasos que coincide con los pasos anteriores: una solicitud de herramienta pasa a través de hooks, reglas de denegar, reglas de preguntar, modo de permiso, reglas de permitir y canUseTool. Los hooks, reglas de denegar y canUseTool pueden enrutar hacia Bloqueado; el modo de permiso de omisión, reglas de permitir y canUseTool pueden enrutar hacia Ejecutar; las reglas de preguntar enrutan a canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

A partir de v2.1.198, si pasa una devolución de llamada `canUseTool` que este orden de evaluación nunca puede alcanzar, el SDK de TypeScript emite una advertencia de proceso de Node.js una vez cuando se construye la consulta. El código de la advertencia es `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Dos configuraciones lo desencadenan:

* `permissionMode: 'bypassPermissions'`, que aprueba automáticamente cada llamada que llega al paso del modo de permiso
* Cada entrada `allowedTools` simple como `"Read"`, que aprueba automáticamente esa herramienta completa antes de que se consulte la devolución de llamada

Las entradas con un especificador como `Bash(ls *)` y el modo `acceptEdits` no lo desencadenan, y las reglas de permitir provenientes de archivos de configuración no son visibles para la verificación.

Escuche con `process.on('warning', ...)` y haga coincidir el código para registrarlo o suprimirlo. Para controlar cada llamada de herramienta independientemente del modo y las reglas, use un [hook `PreToolUse`](/docs/es/agent-sdk/hooks) en su lugar.

Esta página se enfoca en **reglas de permitir y denegar** y **modos de permiso**. Para los otros pasos:

* **Hooks:** ejecute código personalizado para permitir, denegar o modificar solicitudes de herramientas. Consulte [Controlar la ejecución con hooks](/docs/es/agent-sdk/hooks).
* **Devolución de llamada canUseTool:** solicite aprobación a los usuarios en tiempo de ejecución, cuando ningún paso anterior resuelve la llamada. Consulte [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Reglas de permitir y denegar
</h2>

`allowed_tools` y `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) agregan entradas a las listas de reglas de permitir y denegar en el flujo de evaluación anterior. Las reglas de permitir solo afectan la aprobación: una herramienta no listada en `allowed_tools` sigue estando disponible para Claude y se descarta al modo de permiso. Las reglas de denegar se comportan de manera diferente dependiendo de si nombran una herramienta o delimitan un patrón dentro de una.

| Opción                            | Efecto                                                                                                                                                                                                                                                   |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` y `Grep` se aprueban automáticamente. Las herramientas no listadas aquí aún existen y se descartan al modo de permiso y `canUseTool`.                                                                                                             |
| `disallowed_tools=["Bash"]`       | La definición de la herramienta `Bash` se elimina de la solicitud. Claude no ve la herramienta y no puede intentarla.                                                                                                                                    |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` permanece disponible. Las llamadas que coinciden con `rm *` se deniegan en cada modo de permiso, incluido `bypassPermissions`. Otras llamadas de `Bash` se descartan al modo de permiso.                                                          |
| `disallowed_tools=["*"]`          | Cada definición de herramienta se elimina de la solicitud. Los patrones globales de nombres de herramientas se admiten en reglas de denegar: `"*"` coincide con cada herramienta y `"mcp__*"` coincide con cada herramienta MCP en todos los servidores. |

Las reglas de permitir aceptan patrones globales de nombres de herramientas solo después de un prefijo literal `mcp__<server>__`. El segmento del servidor debe estar libre de patrones globales para que la regla nombre un servidor específico que haya configurado: `mcp__puppeteer__*` coincide con cada herramienta del servidor `puppeteer`, y `mcp__github__get_*` coincide con sus herramientas `get_`. Una entrada sin ancla como `allowed_tools=["*"]` o `allowed_tools=["mcp__*"]` se ignora con una advertencia de inicio y no pre-aprueba nada.

Las reglas delimitadas para `Read` y `Edit` toman un patrón de ruta. Las reglas `Edit(path)` rigen todas las herramientas integradas que escriben archivos, incluidas `Write` y `NotebookEdit`; una regla `Write(path)` nunca es coincidida por las comprobaciones de permiso de archivo.

Use `//path` para una ruta del sistema de archivos absoluta: una regla de denegar de `Edit(//secrets/**)` bloquea escrituras en cualquier lugar bajo `/secrets` en el disco. Con una sola barra diagonal inicial, `Edit(/secrets/**)` se ancla en la fuente de la regla en su lugar. Para reglas pasadas a través de `allowed_tools` o `disallowed_tools`, eso significa el directorio de trabajo de la sesión, por lo que la regla no bloquea `/secrets` en el disco. Consulte [Reglas de Read y Edit](/docs/es/permissions#read-and-edit) para las cuatro formas de anclaje y cómo se resuelven las reglas de archivos de configuración.

<Warning>
  **Las herramientas pre-aprobadas nunca llegan a `canUseTool`.** Una llamada de herramienta aprobada en cualquier paso anterior, por `acceptEdits` o `bypassPermissions`, o por una regla de permitir, omite su devolución de llamada `canUseTool`, por lo que las comprobaciones de permiso que coloque allí se omiten silenciosamente para esa herramienta. `AskUserQuestion`, herramientas MCP marcadas [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool), y herramientas de conector [que su organización configuró para `ask`](/docs/es/mcp#organization-controls-on-connector-tools) aún llegan a la devolución de llamada, incluso cuando una regla de permitir coincide.

  La cobertura depende de la forma de la entrada: un nombre simple como `Read` o `mcp__github__get_issue` pre-aprueba cada llamada a esa herramienta, mientras que una regla delimitada como `Bash(ls *)` pre-aprueba solo las llamadas coincidentes y otras llamadas de `Bash` aún se descartan a la devolución de llamada. Para comprobaciones que deben ejecutarse en cada llamada de herramienta, use un hook [`PreToolUse`](/docs/es/agent-sdk/hooks): los hooks se ejecutan antes de cada otro paso, y una denegación de hook se aplica incluso en modo `bypassPermissions`.
</Warning>

Para un agente bloqueado, empareje `allowedTools` con `permissionMode: "dontAsk"`. Las herramientas listadas se aprueban, aparte de las herramientas que siempre solicitan en la Advertencia anterior; cualquier otra cosa se deniega directamente en lugar de solicitar:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools` no restringe `bypassPermissions`.** `allowed_tools` solo pre-aprueba las herramientas que lista. Las herramientas no listadas no coinciden con ninguna regla de permitir y se descartan al modo de permiso, donde `bypassPermissions` las aprueba. Establecer `allowed_tools=["Read"]` junto con `permission_mode="bypassPermissions"` aún aprueba cada herramienta, incluidas `Bash`, `Write` y `Edit`. Si necesita `bypassPermissions` pero desea que herramientas específicas se bloqueen, use `disallowed_tools`.
</Warning>

También puede configurar reglas de permitir, denegar y preguntar de forma declarativa en `.claude/settings.json`. Estas reglas se leen cuando la fuente de configuración `project` está habilitada, que lo está para las opciones predeterminadas de `query()`. Si establece `setting_sources` (TypeScript: `settingSources`) explícitamente, incluya `"project"` para que se apliquen. Consulte [Configuración de permisos](/docs/es/settings#permission-settings) para la sintaxis de reglas.

<h2 id="permission-modes">
  Modos de permiso
</h2>

Los modos de permiso proporcionan control global sobre cómo Claude utiliza las herramientas. Puede establecer el modo de permiso al llamar a `query()` o cambiarlo dinámicamente durante sesiones de transmisión.

<h3 id="available-modes">
  Modos disponibles
</h3>

El SDK admite estos modos de permiso:

| Modo                | Descripción                                | Comportamiento de herramientas                                                                                                                                                                                                                                                                                                                            |
| :------------------ | :----------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportamiento de permiso estándar         | Sin aprobaciones automáticas; las herramientas no coincidentes activan su devolución de llamada `canUseTool`                                                                                                                                                                                                                                              |
| `dontAsk`           | Denegar en lugar de solicitar              | Cualquier cosa no pre-aprobada por `allowed_tools` o reglas se deniega; herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas que requieren interacción del usuario se deniegan incluso si las ha pre-aprobado. `canUseTool` nunca se llama                                 |
| `acceptEdits`       | Auto-aceptar ediciones de archivo          | Las ediciones de archivo y [operaciones del sistema de archivos](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, etc.) se aprueban automáticamente                                                                                                                                                                                                  |
| `bypassPermissions` | Omitir todas las verificaciones de permiso | Las herramientas se ejecutan sin solicitudes de permiso, excepto herramientas coincidentes por una [regla `ask` explícita](#how-permissions-are-evaluated), herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas que requieren interacción del usuario (usar con cuidado) |
| `plan`              | Modo de planificación                      | Claude explora y planifica sin editar sus archivos fuente; las ediciones de archivo nunca se aprueban automáticamente y se solicitan a través de su devolución de llamada `canUseTool`                                                                                                                                                                    |
| `auto`              | Aprobaciones clasificadas por modelo       | Un clasificador de modelo aprueba o deniega cada llamada de herramienta. Consulte [Auto mode](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para disponibilidad                                                                                                                                                                                  |

<Warning>
  **Herencia de subagentos:** Cuando el padre usa `bypassPermissions`, `acceptEdits` o `auto`, todos los subagentos heredan ese modo y no se puede anular por subagentos. Los subagentos pueden tener diferentes indicaciones del sistema y comportamiento menos restringido que su agente principal, por lo que heredar `bypassPermissions` les otorga acceso completo y autónomo al sistema. Las [reglas `ask` explícitas](#how-permissions-are-evaluated), herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas que requieren interacción del usuario aún fuerzan una solicitud.
</Warning>

<h3 id="set-permission-mode">
  Establecer modo de permiso
</h3>

Puede establecer el modo de permiso una vez al iniciar una consulta, o cambiarlo dinámicamente mientras la sesión está activa.

<Tabs>
  <Tab title="En tiempo de consulta">
    Pase `permission_mode` (Python) o `permissionMode` (TypeScript) al crear una consulta. Este modo se aplica para toda la sesión a menos que se cambie dinámicamente.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Durante la transmisión">
    Llame a `set_permission_mode()` (Python) o `setPermissionMode()` (TypeScript) para cambiar el modo a mitad de sesión. El nuevo modo entra en vigor inmediatamente para todas las solicitudes de herramientas posteriores. Esto le permite comenzar restrictivo y flexibilizar los permisos a medida que aumenta la confianza, por ejemplo, cambiar a `acceptEdits` después de revisar el enfoque inicial de Claude.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  Detalles del modo
</h3>

<h4 id="accept-edits-mode-acceptedits">
  Modo de aceptar ediciones (`acceptEdits`)
</h4>

Auto-aprueba operaciones de archivo para que Claude pueda editar código sin solicitar. Otras herramientas (como comandos Bash que no son operaciones del sistema de archivos) aún requieren permisos normales.

**Operaciones auto-aprobadas:**

* Ediciones de archivo (herramientas Edit, Write)
* Comandos del sistema de archivos: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Ambos se aplican solo a rutas dentro del directorio de trabajo o `additionalDirectories`. Las rutas fuera de ese alcance y las escrituras en rutas protegidas aún solicitan.

**Usar cuando:** confía en las ediciones de Claude y desea una iteración más rápida, como durante la creación de prototipos o cuando trabaja en un directorio aislado.

<h4 id="don’t-ask-mode-dontask">
  Modo no preguntar (`dontAsk`)
</h4>

Convierte cualquier solicitud de permiso en una denegación. Las herramientas pre-aprobadas por `allowed_tools`, reglas de permitir de `settings.json` o un hook se ejecutan normalmente. Las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas que requieren interacción del usuario se deniegan incluso cuando una regla de permitir coincide. Todo lo demás se deniega sin llamar a `canUseTool`.

**Usar cuando:** desea una superficie de herramienta fija y explícita para un agente sin interfaz y prefiere una denegación dura sobre la dependencia silenciosa de que `canUseTool` esté ausente.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Modo de omitir permisos (`bypassPermissions`)
</h4>

Auto-aprueba todos los usos de herramientas sin solicitudes. Los hooks aún se ejecutan y pueden bloquear operaciones si es necesario.

<Warning>
  Usar con extrema precaución. Claude tiene acceso completo al sistema en este modo. Solo use en entornos controlados donde confía en todas las operaciones posibles.

  `allowed_tools` no restringe este modo. Cada herramienta se aprueba, no solo las que listó. Las reglas de denegar (`disallowed_tools`), reglas explícitas de `ask` y hooks se evalúan antes de la verificación del modo y aún pueden bloquear una herramienta. Las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas que requieren interacción del usuario aún se transfieren a su devolución de llamada `canUseTool`.
</Warning>

<h4 id="plan-mode-plan">
  Modo de planificación (`plan`)
</h4>

Claude explora la base de código y produce un plan sin editar sus archivos fuente. Las herramientas de solo lectura se ejecutan como en modo predeterminado. Las ediciones de archivo nunca se aprueban automáticamente en modo de planificación, incluso cuando una regla de permitir coincide. Se solicitan a través de su devolución de llamada `canUseTool` en su lugar. Claude puede usar `AskUserQuestion` para aclarar requisitos antes de finalizar el plan. Consulte [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input#handle-clarifying-questions) para manejar estas solicitudes.

**Usar cuando:** desea que Claude proponga cambios sin ejecutarlos, como durante la revisión de código o cuando necesita aprobar cambios antes de que se realicen.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para los otros pasos en el flujo de evaluación de permisos:

* [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input): solicitudes de aprobación interactivas y preguntas aclaratorias
* [Guía de hooks](/docs/es/agent-sdk/hooks): ejecute código personalizado en puntos clave del ciclo de vida del agente
* [Reglas de permisos](/docs/es/settings#permission-settings): reglas declarativas de permitir/denegar en `settings.json`
