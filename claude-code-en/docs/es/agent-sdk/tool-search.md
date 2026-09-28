> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escala a muchas herramientas con búsqueda de herramientas

> Escala tu agente a miles de herramientas descubriendo y cargando solo lo que se necesita, bajo demanda.

La búsqueda de herramientas permite que tu agente trabaje con cientos o miles de herramientas descubriendo y cargándolas dinámicamente bajo demanda. En lugar de cargar todas las definiciones de herramientas en la ventana de contexto de antemano, el agente busca en tu catálogo de herramientas y carga solo las herramientas que necesita.

Este enfoque resuelve dos desafíos a medida que las bibliotecas de herramientas se escalan:

* **Eficiencia de contexto:** Las definiciones de herramientas pueden consumir grandes porciones de la ventana de contexto (50 herramientas pueden usar 10-20K tokens), dejando menos espacio para el trabajo real.
* **Precisión de selección de herramientas:** La precisión de selección de herramientas se degrada con más de 30-50 herramientas cargadas a la vez.

<h2 id="how-tool-search-works">
  Cómo funciona la búsqueda de herramientas
</h2>

La búsqueda de herramientas está activada de forma predeterminada, con las excepciones enumeradas en [Configurar la búsqueda de herramientas](#configure-tool-search).

Cuando está activa, las definiciones de herramientas se retienen de la ventana de contexto. El agente recibe un resumen de las herramientas disponibles y busca las relevantes cuando la tarea requiere una capacidad que no está ya cargada. Hasta cinco de las herramientas más relevantes se cargan en contexto de forma predeterminada, donde permanecen disponibles para turnos posteriores hasta que el SDK compacta los mensajes donde el agente las descubrió. Después de esa compactación, el agente busca esas herramientas nuevamente cuando las necesita.

La búsqueda de herramientas añade un viaje de ida y vuelta extra cada vez que Claude busca herramientas, pero para grandes conjuntos de herramientas esto se compensa con un contexto más pequeño en cada turno. Con menos de \~10 herramientas cuyas definiciones caben cómodamente en la ventana de contexto, cargar todo de antemano es típicamente más rápido.

Para detalles sobre el mecanismo API subyacente, consulta [Búsqueda de herramientas en la API](https://platform.claude.com/docs/es/agents-and-tools/tool-use/tool-search-tool).

<Note>
  La búsqueda de herramientas no es compatible con implementaciones de Microsoft Foundry [alojadas en Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), que la rechazan del lado del servidor: el SDK detecta el rechazo y carga las definiciones de herramientas de antemano para esa implementación en su lugar. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) no puede anular esto, ya que el rechazo proviene de la implementación misma.
</Note>

<h2 id="configure-tool-search">
  Configurar la búsqueda de herramientas
</h2>

La búsqueda de herramientas está activada por defecto. Para los modelos en la lista de modelos no compatibles del SDK, el SDK carga las definiciones de herramientas de antemano en su lugar, y ningún valor `ENABLE_TOOL_SEARCH` anula eso. En Google Cloud's Agent Platform, el SDK decide por generación de modelo:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 y posterior**: la búsqueda de herramientas está activada por defecto.
* **Modelos anteriores de Agent Platform**: el SDK carga las definiciones de herramientas de antemano, porque sus pilas de servicio rechazan el encabezado beta requerido. `ENABLE_TOOL_SEARCH` no puede anular esto.

Antes de Claude Code v2.1.221, el SDK deshabilitaba la búsqueda de herramientas para todos los modelos en Google Cloud's Agent Platform a menos que estableciera `ENABLE_TOOL_SEARCH`.

El SDK también desactiva la búsqueda de herramientas cuando `ANTHROPIC_BASE_URL` apunta a un host que no es de primera parte, ya que la mayoría de los proxies no reenvían bloques `tool_reference`. Puede anular ese valor por defecto con la variable de entorno `ENABLE_TOOL_SEARCH`:

| Valor            | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (sin establecer) | La búsqueda de herramientas está activada. Las definiciones de herramientas se difieren y se descubren bajo demanda. Se retrocede a la carga de antemano en los modelos de Google Cloud's Agent Platform anteriores a la generación Claude 4.5, un `ANTHROPIC_BASE_URL` que no es de primera parte, o una implementación de Microsoft Foundry alojada en Azure.                                                                                                                                       |
| `true`           | La búsqueda de herramientas siempre está activada, excepto en una implementación de Microsoft Foundry alojada en Azure, donde el rechazo del lado del servidor aún fuerza la carga de antemano, y en los modelos de Google Cloud's Agent Platform anteriores a la generación Claude 4.5, donde el SDK sigue cargando las definiciones de herramientas de antemano. El SDK envía el encabezado beta a través de proxies, y las solicitudes fallan en proxies que no soportan bloques `tool_reference`. |
| `auto`           | Cuenta los tokens en las definiciones de herramientas que la búsqueda de herramientas puede diferir y compara el total contra la ventana de contexto del modelo. Cuando el total alcanza el 10% de la ventana, la búsqueda de herramientas se activa. Por debajo de eso, el SDK carga todas las definiciones de herramientas en contexto de antemano.                                                                                                                                                 |
| `auto:N`         | Igual que `auto` con un porcentaje personalizado. `auto:5` se activa cuando esas definiciones alcanzan el 5% de la ventana de contexto. Los valores más bajos se activan antes.                                                                                                                                                                                                                                                                                                                       |
| `false`          | La búsqueda de herramientas está desactivada. Todas las definiciones de herramientas se cargan en contexto en cada turno.                                                                                                                                                                                                                                                                                                                                                                             |

Establecer [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/es/env-vars) mantiene la búsqueda de herramientas desactivada. No puede anularla estableciendo `ENABLE_TOOL_SEARCH` usted mismo. Su organización puede mantener la búsqueda de herramientas activada a través de [configuración administrada](/docs/es/managed-settings), en Claude Code v2.1.227 o posterior. [Deshabilitar capacidades de pre-lanzamiento](/docs/es/llm-gateway-protocol#disable-pre-release-capabilities) cubre dónde se aplica la anulación y qué elimina la variable.

La búsqueda de herramientas se aplica a todas las herramientas registradas, ya sea que provengan de servidores MCP remotos o [servidores MCP personalizados del SDK](/docs/es/agent-sdk/custom-tools). Cuando utiliza `auto`, el SDK cuenta todas las definiciones que la búsqueda de herramientas puede diferir hacia un umbral combinado: cada herramienta MCP que no esté marcada como [`alwaysLoad`](/docs/es/mcp#exempt-a-server-from-deferral), de cualquier servidor, más las herramientas integradas que se cargan bajo demanda. El SDK siempre carga las herramientas integradas principales como Bash, Read y Edit de antemano y no las cuenta hacia el umbral.

Establezca el valor en la opción `env` en `query()`. En TypeScript, `env` reemplaza el entorno del subproceso, por lo que debe expandir `...process.env` para mantener las variables heredadas. En Python, `env` se fusiona sobre el entorno heredado. Este ejemplo se conecta a un servidor MCP remoto que expone muchas herramientas, pre-aprueba todas ellas con un comodín, y utiliza `auto:5` para que la búsqueda de herramientas se active cuando las definiciones que puede diferir alcanzan el 5% de la ventana de contexto:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Find and run the appropriate database query",
      options: {
        mcpServers: {
          "enterprise-tools": {
            // Connect to a remote MCP server
            type: "http",
            url: "https://tools.example.com/mcp"
          }
        },
        allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
        env: {
          ...process.env, // env replaces the subprocess environment, so keep inherited variables
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when deferrable definitions reach 5% of context
        }
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "enterprise-tools": {
                  "type": "http",
                  "url": "https://tools.example.com/mcp",
              }
          },
          allowed_tools=[
              "mcp__enterprise-tools__*"
          ],  # Wildcard pre-approves all tools from this server
          env={
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when deferrable definitions reach 5% of context
          },
      )

      try:
          async for message in query(
              prompt="Find and run the appropriate database query",
              options=options,
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Para ejecutar este ejemplo, reemplace `https://tools.example.com/mcp` con la URL de su propio servidor MCP. Si tiene éxito, el texto del resultado se imprime en la consola.

Debido a que se trata de una llamada `query()` de un solo disparo, el SDK genera una excepción después de producir un resultado de error, por lo que el ejemplo envuelve el bucle en un bloque try. Para ver por qué falló una ejecución, verifique el `subtype` del mensaje de resultado, como `error_during_execution`, dentro del bucle. Para obtener más información sobre los mensajes de resultado, consulte [Manejar el resultado](/docs/es/agent-sdk/agent-loop#handle-the-result).

<h2 id="optimize-tool-discovery">
  Optimizar el descubrimiento de herramientas
</h2>

El mecanismo de búsqueda coincide consultas contra nombres y descripciones de herramientas. Nombres como `search_slack_messages` aparecen para un rango más amplio de solicitudes que `query_slack`. Las descripciones con palabras clave específicas ("Buscar mensajes de Slack por palabra clave, canal o rango de fechas") coinciden con más consultas que las genéricas ("Consultar Slack").

También puede añadir una sección de indicación del sistema listando categorías de herramientas disponibles. Esto le da al agente contexto sobre qué tipos de herramientas están disponibles para buscar. Pase el texto a través de la opción `systemPrompt` en TypeScript o `system_prompt` en Python, utilizando el preset `claude_code` con `append`, que añade su texto al prompt del preset en lugar de reemplazarlo:

<CodeGroup>
  ```typescript TypeScript theme={null}
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: "You can search for tools to interact with Slack, GitHub, and Jira."
    }
  }
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      system_prompt={
          "type": "preset",
          "preset": "claude_code",
          "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
      }
  )
  ```
</CodeGroup>

Para el conjunto completo de opciones de indicación del sistema, consulte [Modificación de indicaciones del sistema](/docs/es/agent-sdk/modifying-system-prompts).

<h2 id="limits">
  Límites
</h2>

* **Herramientas máximas:** 10,000 herramientas en tu catálogo
* **Resultados de búsqueda:** devuelve hasta cinco herramientas más relevantes por búsqueda de forma predeterminada
* **Soporte de modelo:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 y modelos posteriores; consulta [compatibilidad de modelos en la documentación de la API](https://platform.claude.com/docs/es/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para la lista actual. Lo mismo se aplica en la plataforma de agentes de Google Cloud.

<h2 id="related-documentation">
  Documentación relacionada
</h2>

* [Búsqueda de herramientas en la API](https://platform.claude.com/docs/es/agents-and-tools/tool-use/tool-search-tool): Documentación completa de la API para búsqueda de herramientas, incluyendo implementaciones personalizadas
* [Conectar servidores MCP](/docs/es/agent-sdk/mcp): Conecte a herramientas externas a través de servidores MCP
* [Herramientas personalizadas](/docs/es/agent-sdk/custom-tools): Construya sus propias herramientas con servidores MCP del SDK
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): Referencia completa de la API
* [Referencia del SDK de Python](/docs/es/agent-sdk/python): Referencia completa de la API
