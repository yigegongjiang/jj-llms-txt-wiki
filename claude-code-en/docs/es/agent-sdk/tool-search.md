> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escala a muchas herramientas con búsqueda de herramientas

> Escala tu agente a miles de herramientas descubriendo y cargando solo lo que se necesita, bajo demanda.

La búsqueda de herramientas permite que tu agente trabaje con cientos o miles de herramientas descubriendo y cargándolas dinámicamente bajo demanda. En lugar de cargar todas las definiciones de herramientas en la ventana de contexto de antemano, el agente busca en tu catálogo de herramientas y carga solo las herramientas que necesita.

Este enfoque resuelve dos desafíos a medida que las bibliotecas de herramientas se escalan:

* **Eficiencia de contexto:** Las definiciones de herramientas pueden consumir grandes porciones de la ventana de contexto (50 herramientas pueden usar 10-20K tokens), dejando menos espacio para el trabajo real.
* **Precisión de selección de herramientas:** La precisión de selección de herramientas se degrada con más de 30-50 herramientas cargadas a la vez.

La búsqueda de herramientas está habilitada por defecto.

<h2 id="how-tool-search-works">
  Cómo funciona la búsqueda de herramientas
</h2>

Cuando la búsqueda de herramientas está activa, las definiciones de herramientas se retienen de la ventana de contexto. El agente recibe un resumen de las herramientas disponibles y busca las relevantes cuando la tarea requiere una capacidad que no está ya cargada. Hasta cinco de las herramientas más relevantes se cargan en contexto de forma predeterminada, donde permanecen disponibles para turnos posteriores. Si la conversación es lo suficientemente larga como para que el SDK compacte mensajes anteriores para liberar espacio, las herramientas descubiertas previamente pueden ser removidas, y el agente busca nuevamente según sea necesario.

La búsqueda de herramientas añade un viaje de ida y vuelta extra la primera vez que Claude descubre una herramienta (el paso de búsqueda), pero para grandes conjuntos de herramientas esto se compensa con un contexto más pequeño en cada turno. Con menos de \~10 herramientas, cargar todo de antemano es típicamente más rápido.

Para detalles sobre el mecanismo API subyacente, consulta [Búsqueda de herramientas en la API](https://platform.claude.com/docs/es/agents-and-tools/tool-use/tool-search-tool).

<Note>
  La búsqueda de herramientas es compatible con Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 y modelos posteriores; consulta [compatibilidad de modelos en la documentación de la API](https://platform.claude.com/docs/es/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para la lista actual. En la Plataforma de Agentes de Google Cloud, los modelos mínimos compatibles son Claude Sonnet 4.5 y Claude Opus 4.5.
</Note>

<h2 id="configure-tool-search">
  Configurar la búsqueda de herramientas
</h2>

La búsqueda de herramientas está activada por defecto. Está deshabilitada por defecto en Google Cloud's Agent Platform, donde es compatible con Claude Sonnet 4.5 y posterior y Claude Opus 4.5 y posterior. También está deshabilitada cuando `ANTHROPIC_BASE_URL` apunta a un host que no es de primera parte, ya que la mayoría de los proxies no reenvían bloques `tool_reference`. Puedes anular cualquiera de los valores por defecto con la variable de entorno `ENABLE_TOOL_SEARCH`:

| Valor            | Comportamiento                                                                                                                                                                                                                                                                                                   |
| :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (sin establecer) | La búsqueda de herramientas está activada. Las definiciones de herramientas se difieren y se descubren bajo demanda. Se retrocede a la carga de antemano en Google Cloud's Agent Platform o en un `ANTHROPIC_BASE_URL` que no es de primera parte.                                                               |
| `true`           | La búsqueda de herramientas siempre está activada. El SDK envía el encabezado beta incluso en Google Cloud's Agent Platform y a través de proxies. Las solicitudes fallan en modelos de Google Cloud's Agent Platform anteriores a Sonnet 4.5 u Opus 4.5, o en proxies que no soportan bloques `tool_reference`. |
| `auto`           | Verifica el recuento de tokens combinado de todas las definiciones de herramientas contra la ventana de contexto del modelo. Si exceden el 10%, la búsqueda de herramientas se activa. Si están por debajo del 10%, todas las herramientas se cargan en contexto normalmente.                                    |
| `auto:N`         | Igual que `auto` con un porcentaje personalizado. `auto:5` se activa cuando las definiciones de herramientas exceden el 5% de la ventana de contexto. Los valores más bajos se activan antes.                                                                                                                    |
| `false`          | La búsqueda de herramientas está desactivada. Todas las definiciones de herramientas se cargan en contexto en cada turno.                                                                                                                                                                                        |

La configuración [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/es/env-vars) mantiene la búsqueda de herramientas desactivada, y `ENABLE_TOOL_SEARCH` no puede anularla. La variable elimina el encabezado beta que requieren las definiciones de herramientas `defer_loading` y los bloques de contenido `tool_reference`.

La búsqueda de herramientas se aplica a todas las herramientas registradas, ya sea que provengan de servidores MCP remotos o [servidores MCP personalizados del SDK](/docs/es/agent-sdk/custom-tools). Cuando se usa `auto`, el umbral se basa en el tamaño combinado de todas las definiciones de herramientas en todos los servidores.

Establezca el valor en la opción `env` en `query()`. En TypeScript, `env` reemplaza el entorno del subproceso, por lo que debe expandir `...process.env` para mantener las variables heredadas. En Python, `env` se fusiona sobre el entorno heredado. Este ejemplo se conecta a un servidor MCP remoto que expone muchas herramientas, pre-aprueba todas ellas con un comodín, y usa `auto:5` para que la búsqueda de herramientas se active cuando sus definiciones excedan el 5% de la ventana de contexto:

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
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when tools exceed 5% of context
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
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when tools exceed 5% of context
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

Establecer `ENABLE_TOOL_SEARCH` en `"false"` desactiva la búsqueda de herramientas y carga todas las definiciones de herramientas en contexto en cada turno. Esto elimina el viaje de ida y vuelta de búsqueda, que puede ser más rápido cuando el conjunto de herramientas es pequeño (menos de \~10 herramientas) y las definiciones caben cómodamente en la ventana de contexto.

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
* **Soporte de modelo:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 y modelos posteriores; consulta [compatibilidad de modelos en la documentación de la API](https://platform.claude.com/docs/es/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para la lista actual. En la plataforma de agentes de Google Cloud, Claude Sonnet 4.5 y posteriores y Claude Opus 4.5 y posteriores.

<h2 id="related-documentation">
  Documentación relacionada
</h2>

* [Búsqueda de herramientas en la API](https://platform.claude.com/docs/es/agents-and-tools/tool-use/tool-search-tool): Documentación completa de la API para búsqueda de herramientas, incluyendo implementaciones personalizadas
* [Conectar servidores MCP](/docs/es/agent-sdk/mcp): Conecte a herramientas externas a través de servidores MCP
* [Herramientas personalizadas](/docs/es/agent-sdk/custom-tools): Construya sus propias herramientas con servidores MCP del SDK
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): Referencia completa de la API
* [Referencia del SDK de Python](/docs/es/agent-sdk/python): Referencia completa de la API
