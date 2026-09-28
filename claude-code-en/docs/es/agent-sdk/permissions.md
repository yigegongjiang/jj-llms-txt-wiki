> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar permisos

> Controle cómo su agente utiliza herramientas con modos de permiso, hooks y reglas declarativas de permitir/denegar.

El SDK del Agente Claude proporciona controles de permisos para gestionar cómo Claude utiliza herramientas. Utilice modos de permiso y reglas para definir qué está permitido automáticamente, y la devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) para manejar todo lo demás en tiempo de ejecución.

<h2 id="how-permissions-are-evaluated">
  Cómo se evalúan los permisos
</h2>

Cuando Claude solicita una herramienta, el SDK comprueba los permisos en este orden:

<Steps>
  <Step title="Hooks">
    Ejecutar [hooks](/docs/es/agent-sdk/hooks) primero. Un hook puede denegar la llamada directamente o dejarla pasar. Un hook que devuelve `allow` no omite las reglas de denegación y pregunta a continuación; esas se evalúan independientemente del resultado del hook. Un hook `PreToolUse` allow tampoco puede aprobar una eliminación `rm` o `rmdir` dirigida a una [ruta crítica](/docs/es/permission-modes#critical-paths).
  </Step>

  <Step title="Reglas de denegación">
    Comprobar reglas `deny` (de `disallowed_tools` y [settings.json](/docs/es/settings-reference#permission-settings)). Si una regla de denegación coincide, la herramienta se bloquea, incluso en modo `bypassPermissions`. Las reglas de denegación de nombre simple como `Bash` eliminan la herramienta del contexto de Claude antes de que comience esta evaluación, por lo que solo se comprueban las reglas con alcance como `Bash(rm *)` en este paso.
  </Step>

  <Step title="Reglas de pregunta">
    Comprobar reglas `ask` de [settings.json](/docs/es/settings-reference#permission-settings). Si una regla de pregunta coincide, la llamada se pasa a su devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) para confirmación, incluso en modo `bypassPermissions`.

    Las herramientas que requieren interacción del usuario se comportan de la misma manera: `AskUserQuestion` y herramientas MCP cuyo servidor establece [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) siempre se pasan a la devolución de llamada, incluso cuando una regla de permiso coincide. En modo `dontAsk` ambos casos se deniegan en su lugar, porque ese modo nunca solicita. La anotación MCP requiere Claude Code v2.1.199 o posterior.

    Las herramientas del conector [claude.ai](/docs/es/mcp#organization-controls-on-connector-tools) que su organización ha establecido en `ask` también salen del flujo en este paso. Cada llamada se pasa a la devolución de llamada, incluso en modo `bypassPermissions` y incluso cuando una regla de permiso coincide. La devolución de llamada recibe la razón `Your organization requires approval for this tool`. En modo `dontAsk` la llamada se deniega en su lugar, porque ese modo nunca solicita.
  </Step>

  <Step title="Modo de permiso">
    Aplicar el [modo de permiso](#permission-modes) activo:

    * En modo `bypassPermissions`, Claude Code aprueba todo lo que llega a este paso excepto eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](/docs/es/permission-modes#critical-paths), que se pasan en su lugar.
    * En modo `acceptEdits`, Claude Code aprueba las operaciones de archivo enumeradas en [Modo de aceptación de ediciones](#accept-edits-mode-acceptedits).
    * En modo `plan`, Claude Code envía herramientas de edición de archivos y escritura de shell a su devolución de llamada `canUseTool` independientemente de las reglas de permiso, por lo que las operaciones de escritura no pueden aprobarse automáticamente mientras se planifica.
    * En otros modos, la solicitud se pasa.
  </Step>

  <Step title="Reglas de permiso">
    Comprobar reglas `allow` (de `allowed_tools` y settings.json). Si una regla coincide, la herramienta se aprueba. Una llamada que la herramienta aprueba por sí sola también se resuelve en este paso, sin necesidad de regla: por ejemplo una lectura de archivo dentro de sus directorios de trabajo o un [comando Bash de solo lectura](/docs/es/permissions#read-only-commands). Las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](/docs/es/permission-modes#critical-paths) nunca se aprueban por una regla de permiso: llegan a su devolución de llamada en los modos que solicitan, van al [clasificador](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) en modo `auto` en Claude Code v2.1.218 o posterior, y se deniegan en modo `dontAsk`.
  </Step>

  <Step title="Devolución de llamada canUseTool">
    Si no se resuelve por ninguno de los anteriores, llame a su devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) para una decisión. En modo `dontAsk`, este paso se omite y la herramienta se deniega.

    En el SDK de TypeScript, si establece [`permissionPrompts: 'none'`](/docs/es/agent-sdk/typescript#options), su devolución de llamada no se llama en este paso. Un hook [`PermissionRequest`](/docs/es/hooks#permissionrequest) aún tiene la oportunidad de decidir, y si no lo hace, Claude Code deniega la llamada. La opción requiere Claude Code v2.1.259 o posterior.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="Diagrama del flujo de evaluación de permisos de seis pasos que coincide con los pasos anteriores: una solicitud de herramienta pasa a través de hooks, reglas de denegación, reglas de pregunta, modo de permiso, reglas de permiso y canUseTool. Los hooks, reglas de denegación y canUseTool pueden enrutar hacia Bloqueado; el modo de permiso bypass, reglas de permiso y canUseTool pueden enrutar hacia Ejecutar; las reglas de pregunta se enrutan a canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="Diagrama del flujo de evaluación de permisos de seis pasos que coincide con los pasos anteriores: una solicitud de herramienta pasa a través de hooks, reglas de denegación, reglas de pregunta, modo de permiso, reglas de permiso y canUseTool. Los hooks, reglas de denegación y canUseTool pueden enrutar hacia Bloqueado; el modo de permiso bypass, reglas de permiso y canUseTool pueden enrutar hacia Ejecutar; las reglas de pregunta se enrutan a canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

Si pasa una devolución de llamada `canUseTool` en una configuración donde el SDK de TypeScript espera que el orden de evaluación apruebe automáticamente las llamadas antes de que se consulte la devolución de llamada, el SDK emite una advertencia de proceso de Node.js una vez cuando se construye la consulta. El código de la advertencia es `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Dos configuraciones la activan:

* `permissionMode: 'bypassPermissions'`, que aprueba automáticamente cada llamada que llega al paso del modo de permiso aparte de las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves)
* Cada entrada `allowedTools` simple como `"Read"`, que aprueba automáticamente esa herramienta completa antes de que se consulte la devolución de llamada, aparte de las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves)

Las entradas con un especificador como `Bash(ls *)` y el modo `acceptEdits` no la activan, y las reglas de permiso provenientes de archivos de configuración no son visibles para la comprobación.

Escuche con `process.on('warning', ...)` y haga coincidir el código para registrarlo o suprimirlo. Para controlar cada llamada de herramienta independientemente del modo y las reglas, use un [hook `PreToolUse`](/docs/es/agent-sdk/hooks) en su lugar.

Esta página se enfoca en **reglas de permiso y denegación** y **modos de permiso**. Para los otros pasos:

* **Hooks:** ejecutar código personalizado para permitir, denegar o modificar solicitudes de herramientas. Consulte [Controlar la ejecución con hooks](/docs/es/agent-sdk/hooks).
* **Devolución de llamada canUseTool:** solicitar aprobación a los usuarios en tiempo de ejecución, cuando ningún paso anterior resuelve la llamada. Consulte [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Reglas de permitir y denegar
</h2>

`allowed_tools` y `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) añaden entradas a las listas de reglas de permitir y denegar en el flujo de evaluación anterior. Si nombra una de las [herramientas de seguimiento de tareas](/docs/es/agent-sdk/todo-tracking#model-availability) en `allowed_tools`, Claude Code también opta por la sesión. Cualquier otra herramienta no listada en `allowed_tools` sigue estando disponible para Claude, y una llamada a ella que necesite aprobación cae en el modo de permisos. Las reglas de denegación se comportan de manera diferente dependiendo de si nombran una herramienta o delimitan un patrón dentro de una.

| Opción                            | Efecto                                                                                                                                                                                                                                                                       |
| :-------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` y `Grep` se aprueban automáticamente. Otras herramientas no listadas aquí siguen existiendo, y las llamadas a ellas que necesiten aprobación caen en el modo de permisos y `canUseTool`.                                                                              |
| `disallowed_tools=["Bash"]`       | La definición de la herramienta `Bash` se elimina de la solicitud. Claude no ve la herramienta y no puede intentarla.                                                                                                                                                        |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` sigue disponible. Las llamadas que coincidan con `rm *` [tal como está escrito](/docs/es/permissions#bash-rule-limits) se deniegan en todos los modos de permisos, incluido `bypassPermissions`. Otras llamadas a `Bash`, incluido `/bin/rm`, caen en el modo de permisos. |
| `disallowed_tools=["*"]`          | Cada definición de herramienta se elimina de la solicitud. Los patrones globales de nombres de herramientas se admiten en reglas de denegación: `"*"` coincide con cada herramienta y `"mcp__*"` coincide con cada herramienta MCP en todos los servidores.                  |

Las reglas de permitir aceptan patrones globales de nombres de herramientas solo después de un prefijo literal `mcp__<server>__`. El segmento del servidor debe estar libre de patrones globales para que la regla nombre un servidor específico que haya configurado: `mcp__puppeteer__*` coincide con cada herramienta del servidor `puppeteer`, y `mcp__github__get_*` coincide con sus herramientas `get_`. Una entrada sin ancla como `allowed_tools=["*"]` o `allowed_tools=["mcp__*"]` se ignora con una advertencia de inicio y no aprueba automáticamente nada.

Las reglas delimitadas para `Read` y `Edit` toman un patrón de ruta. Las reglas `Edit(path)` rigen todas las herramientas integradas que escriben archivos, incluidas `Write` y `NotebookEdit`; una regla `Write(path)` nunca es coincidida por las comprobaciones de permisos de archivo.

Use `//path` para una ruta del sistema de archivos absoluta: una regla de denegación de `Edit(//secrets/**)` bloquea escrituras en cualquier lugar bajo `/secrets` en el disco. Con una sola barra diagonal inicial, `Edit(/secrets/**)` se ancla en la fuente de la regla en su lugar. Para reglas pasadas a través de `allowed_tools` o `disallowed_tools`, eso significa el directorio de trabajo de la sesión, por lo que la regla no bloquea `/secrets` en el disco. Consulte [Reglas de Read y Edit](/docs/es/permissions#read-and-edit) para las cuatro formas de anclaje y cómo se resuelven las reglas de archivos de configuración.

<Warning>
  **Las herramientas aprobadas automáticamente nunca llegan a `canUseTool`.** Una llamada de herramienta aprobada en cualquier paso anterior, por `acceptEdits` o `bypassPermissions`, o por una regla de permitir, omite su devolución de llamada `canUseTool`, por lo que las comprobaciones de permisos que coloque allí se omiten silenciosamente para esa herramienta. `AskUserQuestion`, herramientas MCP marcadas [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool), herramientas de conector [que su organización configuró para `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y removals de `rm` y `rmdir` dirigidos a una [ruta crítica](/docs/es/permission-modes#critical-paths) aún llegan a la devolución de llamada, incluso cuando una regla de permitir coincide. En modo `auto`, los removals de ruta crítica van al [clasificador](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) en lugar de la devolución de llamada, mientras que las otras llamadas listadas aquí aún la alcanzan; el enrutamiento del clasificador requiere Claude Code v2.1.218 o posterior. En modo `dontAsk` estas llamadas se deniegan en su lugar, sin invocar la devolución de llamada.

  La cobertura depende de la forma de la entrada: un nombre simple como `Read` o `mcp__github__get_issue` aprueba automáticamente cada llamada a esa herramienta aparte de las excepciones anteriores, mientras que una regla delimitada como `Bash(npm test *)` aprueba automáticamente solo las llamadas coincidentes, y otras llamadas a `Bash` que necesiten aprobación aún caen en la devolución de llamada. Para comprobaciones que deben ejecutarse en cada llamada de herramienta, use un [hook `PreToolUse`](/docs/es/agent-sdk/hooks): los hooks se ejecutan antes de cada otro paso, y una denegación de hook se aplica incluso en modo `bypassPermissions`.
</Warning>

Para un agente bloqueado, empareje `allowedTools` con `permissionMode: "dontAsk"`:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

Las herramientas listadas se aprueban, aparte de las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves), y cada otra llamada que solicitaría se deniega en su lugar. Las llamadas que no necesitan aprobación en modo `default` se ejecutan independientemente de si las lista o no, como [comandos Bash de solo lectura](/docs/es/permissions#read-only-commands), herramientas como `Agent` que no preguntan antes de ejecutarse, y lecturas de archivos dentro de sus directorios de trabajo. Para poner una herramienta fuera del alcance de Claude por completo, añada su nombre simple a `disallowedTools`.

<Warning>
  **`allowed_tools` no restringe `bypassPermissions`.** `allowed_tools` aprueba previamente las herramientas que lista. Otras herramientas no listadas no son coincididas por ninguna regla de permitir y caen en el modo de permisos, donde `bypassPermissions` las aprueba. Configurar `allowed_tools=["Read"]` junto con `permission_mode="bypassPermissions"` aún aprueba cada herramienta, incluidas `Bash`, `Write` y `Edit`. Si necesita `bypassPermissions` pero desea que herramientas específicas se bloqueen, use `disallowed_tools`.
</Warning>

También puede configurar reglas de permitir, denegar y preguntar de manera declarativa en `.claude/settings.json`. Estas reglas se leen cuando la fuente de configuración `project` está habilitada, que lo está para las opciones predeterminadas de `query()`. Si establece `setting_sources` (TypeScript: `settingSources`) explícitamente, incluya `"project"` para que se apliquen. Consulte [Configuración de permisos](/docs/es/settings-reference#permission-settings) para la sintaxis de reglas.

<h2 id="permission-modes">
  Modos de permisos
</h2>

Los modos de permisos proporcionan control global sobre cómo Claude utiliza las herramientas. Puede establecer el modo de permisos al llamar a `query()` o cambiarlo dinámicamente durante sesiones de transmisión.

<h3 id="available-modes">
  Modos disponibles
</h3>

El SDK admite estos modos de permisos:

| Modo                | Descripción                                   | Comportamiento de herramientas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------ | :-------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportamiento de permisos estándar           | Sin aprobaciones automáticas basadas en modo; las llamadas que necesitan aprobación y no coinciden con ninguna regla de permiso activan su devolución de llamada `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `dontAsk`           | Denegar en lugar de preguntar                 | Cualquier llamada que de otro modo solicitaría confirmación es denegada. Las llamadas aprobadas por `allowed_tools` o reglas se ejecutan, al igual que las llamadas que no necesitan aprobación en modo `default`, como lecturas de archivos dentro de sus directorios de trabajo y llamadas a `Agent`. Las herramientas de conector [que su organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y las herramientas que requieren interacción del usuario son denegadas incluso si las ha preaprobado, al igual que las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](/docs/es/permission-modes#critical-paths). `canUseTool` nunca se llama |
| `acceptEdits`       | Aceptar automáticamente ediciones de archivos | Las ediciones de archivos y [operaciones del sistema de archivos](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, etc.) se aprueban automáticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `bypassPermissions` | Omitir verificaciones de permisos             | Las herramientas se ejecutan sin solicitudes de permisos, excepto para las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves). Úselo con precaución                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `plan`              | Modo de planificación                         | Claude explora y planifica sin editar sus archivos fuente; las ediciones de archivos nunca se aprueban automáticamente y se solicitan a través de su devolución de llamada `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `auto`              | Aprobaciones clasificadas por modelo          | Un clasificador de modelo aprueba o deniega solicitudes de permisos. Consulte [Modo Auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para disponibilidad                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

<Warning>
  **Herencia de subagentes:** Un subagente se ejecuta en el modo de permisos de la sesión principal a menos que establezca `permissionMode` en su [`AgentDefinition`](/docs/es/agent-sdk/typescript#agentdefinition) y la sesión principal esté en modo `default`, `dontAsk` o `plan`. Incluso entonces, Claude Code nunca aplica un valor `"bypassPermissions"`. Un subagente se ejecuta en modo `bypassPermissions` solo cuando la sesión principal lo hace. La excepción `bypassPermissions` requiere Claude Code v2.1.267 o posterior.

  Los subagentes pueden tener indicaciones del sistema diferentes y un comportamiento menos restringido que su agente principal, por lo que heredar `bypassPermissions` les otorga acceso completo y autónomo al sistema. Las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves) aún se aplican.
</Warning>

<h3 id="set-permission-mode">
  Establecer modo de permisos
</h3>

Puede establecer el modo de permisos una vez al iniciar una consulta, o cambiarlo dinámicamente mientras la sesión está activa.

<Tabs>
  <Tab title="En el momento de la consulta">
    Pase `permission_mode` (Python) o `permissionMode` (TypeScript) al crear una consulta. Este modo se aplica durante toda la sesión a menos que se cambie dinámicamente.

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
    Llame a `set_permission_mode()` (Python) o `setPermissionMode()` (TypeScript) para cambiar el modo durante la sesión. El nuevo modo entra en vigor inmediatamente para todas las solicitudes de herramientas posteriores. Esto le permite comenzar de forma restrictiva y flexibilizar los permisos a medida que aumenta la confianza, por ejemplo, cambiar a `acceptEdits` después de revisar el enfoque inicial de Claude.

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
  Modo aceptar ediciones (`acceptEdits`)
</h4>

Aprueba automáticamente operaciones de archivos para que Claude pueda editar código sin solicitar confirmación. Otras herramientas (como comandos Bash que no son operaciones del sistema de archivos) aún requieren permisos normales.

**Operaciones aprobadas automáticamente:**

* Ediciones de archivos (herramientas Edit, Write)
* Comandos del sistema de archivos: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Ambos se aplican solo a rutas dentro del directorio de trabajo o `additionalDirectories`. En modo `acceptEdits`, Claude Code no aprueba automáticamente la solicitud cuando Claude:

* Trabaja en una ruta fuera de ese alcance
* Escribe en una ruta protegida
* Elimina una [ruta crítica](/docs/es/permission-modes#critical-paths) con `rm` o `rmdir`

**Úselo cuando:** confíe en las ediciones de Claude y desee una iteración más rápida, como durante la creación de prototipos o cuando trabaja en un directorio aislado.

<h4 id="don’t-ask-mode-dontask">
  Modo no preguntar (`dontAsk`)
</h4>

Convierte cualquier solicitud de permisos en una denegación, sin llamar a `canUseTool`. Las herramientas preaprobadas por `allowed_tools`, reglas de permiso en `settings.json` o un hook se ejecutan normalmente, al igual que las llamadas que no necesitan aprobación en modo `default`, como lecturas de archivos dentro de sus directorios de trabajo y llamadas a `Agent`. Las herramientas de conector [que su organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools), las herramientas que requieren interacción del usuario y las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](/docs/es/permission-modes#critical-paths) son denegadas incluso cuando una regla de permiso coincide. Un permiso de hook `PreToolUse` tampoco borra una eliminación de ruta crítica.

**Úselo cuando:** desee una superficie de herramientas fija y explícita para un agente sin interfaz y prefiera una denegación definitiva sobre la dependencia silenciosa de que `canUseTool` esté ausente.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Modo omitir permisos (`bypassPermissions`)
</h4>

Aprueba automáticamente usos de herramientas sin solicitar confirmación, excepto en los casos enumerados en la advertencia a continuación. Los hooks aún se ejecutan y pueden bloquear operaciones si es necesario. En Linux y macOS, Claude Code se niega a iniciarse en este modo como root o bajo `sudo` fuera de una [zona de pruebas reconocida](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode), y la consulta falla antes del primer turno.

<Warning>
  Úselo con extrema precaución. Claude tiene acceso completo al sistema en este modo. Úselo solo en entornos controlados donde confíe en todas las operaciones posibles.

  `allowed_tools` no restringe este modo. Cada herramienta se aprueba, no solo las que enumeró. Estos controles aún se aplican:

  * Las reglas de denegación, las reglas explícitas de `ask` y los hooks se evalúan antes de la verificación del modo y aún pueden bloquear una herramienta.
  * Las herramientas de conector [que su organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools), las herramientas que requieren interacción del usuario y las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](/docs/es/permission-modes#critical-paths) aún se transfieren a su devolución de llamada `canUseTool`.
  * Las [medidas de seguridad de mensajería entre sesiones](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode) aún se aplican.
</Warning>

<h4 id="plan-mode-plan">
  Modo de planificación (`plan`)
</h4>

Claude explora la base de código y produce un plan sin editar sus archivos fuente. Las herramientas de solo lectura se ejecutan como lo hacen en el modo de permisos `default`.

Las ediciones de archivos nunca se aprueban automáticamente en modo de planificación, incluso cuando una regla de permiso coincide. En su lugar, se solicitan a través de su devolución de llamada `canUseTool`. En Claude Code v2.1.212 o posterior, los comandos de shell que modifican archivos, como `touch` y `rm`, llegan a su devolución de llamada `canUseTool` de la misma manera.

Si establece `allowDangerouslySkipPermissions: true` junto con `permissionMode: 'plan'`, las ediciones de archivos y los comandos de shell que modifican archivos aún llegan a su devolución de llamada `canUseTool`. La opción le permite cambiar a `bypassPermissions` más tarde con `setPermissionMode()`.

Claude puede usar `AskUserQuestion` para aclarar requisitos antes de finalizar el plan. Consulte [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input#handle-clarifying-questions) para manejar estas solicitudes.

**Úselo cuando:** desee que Claude proponga cambios sin ejecutarlos, como durante la revisión de código o cuando necesita aprobar cambios antes de que se realicen.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para los otros pasos en el flujo de evaluación de permisos:

* [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input): mensajes de aprobación interactivos y preguntas aclaratorias
* [Guía de hooks](/docs/es/agent-sdk/hooks): ejecutar código personalizado en puntos clave del ciclo de vida del agente
* [Reglas de permisos](/docs/es/settings-reference#permission-settings): reglas declarativas de permitir/denegar en `settings.json`
