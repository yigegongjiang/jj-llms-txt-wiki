> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dale a Claude herramientas personalizadas

> Define herramientas personalizadas con el servidor MCP en proceso del SDK del Agente Claude para que Claude pueda llamar a sus funciones, acceder a sus APIs y realizar operaciones específicas del dominio.

Las herramientas personalizadas extienden el SDK del Agente permitiéndole definir sus propias funciones que Claude puede llamar durante una conversación. Usando el servidor MCP en proceso del SDK, puede dar a Claude acceso a bases de datos, APIs externas, lógica específica del dominio u cualquier otra capacidad que su aplicación necesite.

<h2 id="quick-reference">
  Referencia rápida
</h2>

| Si desea...                                               | Haga esto                                                                                                                                                                                                                       |
| :-------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Definir una herramienta                                   | Use [`@tool`](/docs/es/agent-sdk/python#tool) (Python) o [`tool()`](/docs/es/agent-sdk/typescript#tool) (TypeScript) con un nombre, descripción, esquema y controlador. Vea [Crear una herramienta personalizada](#create-a-custom-tool). |
| Registrar una herramienta con Claude                      | Envuelva en `create_sdk_mcp_server` / `createSdkMcpServer` y pase a `mcpServers` en `query()`. Vea [Llamar a una herramienta personalizada](#call-a-custom-tool).                                                               |
| Preaprobación de una herramienta                          | Agregue a sus herramientas permitidas. Vea [Configurar herramientas permitidas](#configure-allowed-tools).                                                                                                                      |
| Eliminar una herramienta integrada del contexto de Claude | Pase un array `tools` listando solo los integrados que desea. Vea [Configurar herramientas permitidas](#configure-allowed-tools).                                                                                               |
| Permitir que Claude llame herramientas en paralelo        | Establezca `readOnlyHint: true` en herramientas sin efectos secundarios. Vea [Agregar anotaciones de herramientas](#add-tool-annotations).                                                                                      |
| Controlar el mensaje de error que Claude lee              | Devuelva `isError: true` para componer el mensaje en lugar de exponer la excepción sin procesar. Vea [Manejar errores](#handle-errors).                                                                                         |
| Devolver imágenes o archivos                              | Use bloques `image` o `resource` en el array de contenido. Vea [Devolver imágenes y recursos](#return-images-and-resources).                                                                                                    |
| Devolver un resultado JSON legible por máquina            | Establezca `structuredContent` en el resultado. Vea [Devolver datos estructurados](#return-structured-data).                                                                                                                    |
| Escalar a muchas herramientas                             | Use [búsqueda de herramientas](/docs/es/agent-sdk/tool-search) para cargar herramientas bajo demanda.                                                                                                                                |

<h2 id="create-a-custom-tool">
  Crear una herramienta personalizada
</h2>

Una herramienta se define por cuatro partes, pasadas como argumentos al helper [`tool()`](/docs/es/agent-sdk/typescript#tool) en TypeScript o al decorador [`@tool`](/docs/es/agent-sdk/python#tool) en Python:

* **Nombre:** un identificador único que Claude utiliza para llamar a la herramienta.
* **Descripción:** qué hace la herramienta. Claude lee esto para decidir cuándo llamarla.
* **Esquema de entrada:** los argumentos que Claude debe proporcionar. En TypeScript esto es siempre un [esquema Zod](https://zod.dev/), y los `args` del manejador se tipan automáticamente a partir de él. En Python esto es un diccionario que asigna nombres a tipos, como `{"latitude": float}`, que el SDK convierte a JSON Schema para usted. El decorador de Python también acepta un diccionario completo de [JSON Schema](https://json-schema.org/understanding-json-schema/about) directamente cuando necesita enumeraciones, rangos, campos opcionales u objetos anidados.
* **Manejador:** la función asincrónica que se ejecuta cuando Claude llama a la herramienta. Recibe los argumentos validados y debe devolver un objeto con:
  * `content` (requerido): una matriz de bloques de resultado, cada uno con un `type` de `"text"`, `"image"`, `"audio"`, `"resource"` o `"resource_link"`. Consulte [Devolver imágenes y recursos](#return-images-and-resources) para bloques que no sean texto.
  * `structuredContent` (opcional): un objeto JSON que contiene el resultado como datos legibles por máquina, devuelto junto con `content`. Consulte [Devolver datos estructurados](#return-structured-data).
  * `isError` (opcional): establézcalo en `true` para señalar un fallo de herramienta para que Claude pueda reaccionar. Consulte [Manejar errores](#handle-errors).

Después de definir una herramienta, envuélvala en un servidor con [`createSdkMcpServer`](/docs/es/agent-sdk/typescript#createsdkmcpserver) (TypeScript) o [`create_sdk_mcp_server`](/docs/es/agent-sdk/python#create_sdk_mcp_server) (Python). El servidor se ejecuta en el proceso dentro de su aplicación, no como un proceso separado.

<h3 id="weather-tool-example">
  Ejemplo de herramienta meteorológica
</h3>

Este ejemplo define una herramienta `get_temperature` y la envuelve en un servidor MCP. Solo configura la herramienta; para pasarla a `query` y ejecutarla, consulte [Llamar a una herramienta personalizada](#call-a-custom-tool) a continuación.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  import httpx
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # Define a tool: name, description, input schema, handler
  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
  )
  async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "current": "temperature_2m",
                  "temperature_unit": "fahrenheit",
              },
          )
          data = response.json()

      # Return a content array - Claude sees this as the tool result
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Temperature: {data['current']['temperature_2m']}°F",
              }
          ]
      }


  # Wrap the tool in an in-process MCP server
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  // Define a tool: name, description, input schema, handler
  const getTemperature = tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
      latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
      longitude: z.number().describe("Longitude coordinate")
    },
    async (args) => {
      // args is typed from the schema: { latitude: number; longitude: number }
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
      );
      const data: any = await response.json();

      // Return a content array - Claude sees this as the tool result
      return {
        content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
      };
    }
  );

  // Wrap the tool in an in-process MCP server
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature]
  });
  ```
</CodeGroup>

Consulte la referencia de TypeScript [`tool()`](/docs/es/agent-sdk/typescript#tool) o la referencia de Python [`@tool`](/docs/es/agent-sdk/python#tool) para obtener detalles completos de parámetros, incluidos formatos de entrada de JSON Schema y estructura de valores de retorno.

<Tip>
  Para hacer un parámetro opcional: en TypeScript, agregue `.default()` al campo Zod. En Python, el esquema de diccionario trata cada clave como requerida, así que deje el parámetro fuera del esquema, menciónelo en la cadena de descripción y léalo con `args.get()` en el manejador. La herramienta [`get_precipitation_chance` a continuación](#add-more-tools) muestra ambos patrones.
</Tip>

<h3 id="call-a-custom-tool">
  Llamar a una herramienta personalizada
</h3>

Pase el servidor MCP que creó a `query` a través de la opción `mcpServers`. La clave en `mcpServers` se convierte en el segmento `{server_name}` en el nombre completamente calificado de cada herramienta: `mcp__{server_name}__{tool_name}`. Liste ese nombre en `allowedTools` para que la herramienta se ejecute sin un aviso de permiso.

Estos fragmentos reutilizan el `weatherServer` del [ejemplo de herramienta meteorológica](#weather-tool-example) para preguntarle a Claude cuál es el clima en una ubicación específica.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"weather": weather_server},
          allowed_tools=["mcp__weather__get_temperature"],
      )

      async for message in query(
          prompt="What's the temperature in San Francisco?",
          options=options,
      ):
          # ResultMessage is the final message after all tool calls complete
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "What's the temperature in San Francisco?",
    options: {
      mcpServers: { weather: weatherServer },
      allowedTools: ["mcp__weather__get_temperature"]
    }
  })) {
    // "result" is the final message after all tool calls complete
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Combine este fragmento con las definiciones de herramienta y servidor del [ejemplo de herramienta meteorológica](#weather-tool-example) en un archivo, luego ejecútelo con `python weather.py` para Python o `npx tsx weather.ts` para TypeScript. Claude llama a `get_temperature` y el script imprime una respuesta de una línea con la temperatura actual en San Francisco.

<h3 id="add-more-tools">
  Agregar más herramientas
</h3>

Un servidor contiene tantas herramientas como liste en su matriz `tools`. Con más de una herramienta en un servidor, puede listar cada una en `allowedTools` individualmente o usar el comodín `mcp__weather__*` para cubrir cada herramienta que el servidor expone.

El ejemplo a continuación define una segunda herramienta, `get_precipitation_chance`, y reemplaza la definición de `weatherServer` del [ejemplo de herramienta meteorológica](#weather-tool-example) con una que lista ambas herramientas en la matriz.

<CodeGroup>
  ```python Python theme={null}
  # Define a second tool for the same server
  @tool(
      "get_precipitation_chance",
      "Get the hourly precipitation probability for a location. "
      "Optionally pass 'hours' (1-24) to control how many hours to return.",
      {"latitude": float, "longitude": float},
  )
  async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
      # 'hours' isn't in the schema - read it with .get() to make it optional
      hours = args.get("hours", 12)
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "hourly": "precipitation_probability",
                  "forecast_days": 1,
              },
          )
          data = response.json()
      chances = data["hourly"]["precipitation_probability"][:hours]

      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
              }
          ]
      }


  # Rebuild the server with both tools in the array
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature, get_precipitation_chance],
  )
  ```

  ```typescript TypeScript theme={null}
  // Define a second tool for the same server
  const getPrecipitationChance = tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
      latitude: z.number(),
      longitude: z.number(),
      hours: z
        .number()
        .int()
        .min(1)
        .max(24)
        .default(12) // .default() makes the parameter optional
        .describe("How many hours of forecast to return")
    },
    async (args) => {
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
      );
      const data: any = await response.json();
      const chances = data.hourly.precipitation_probability.slice(0, args.hours);

      return {
        content: [{ type: "text", text: `Next ${args.hours} hours: ${chances.join("%, ")}%` }]
      };
    }
  );

  // Rebuild the server with both tools in the array
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature, getPrecipitationChance]
  });
  ```
</CodeGroup>

[Búsqueda de herramientas](/docs/es/agent-sdk/tool-search) está habilitada de forma predeterminada y difiere las herramientas MCP del SDK: Claude ve el nombre de cada herramienta en una lista compacta y carga su esquema completo bajo demanda. Con la búsqueda de herramientas deshabilitada, cada herramienta en esta matriz consume espacio de ventana de contexto en cada turno. En TypeScript, pase `alwaysLoad: true` en el argumento `extras` de [`tool()`](/docs/es/agent-sdk/typescript#tool) o en las opciones de [`createSdkMcpServer()`](/docs/es/agent-sdk/typescript#createsdkmcpserver) para mantener el esquema completo de una herramienta en el aviso inicial.

<h3 id="add-tool-annotations">
  Agregar anotaciones de herramientas
</h3>

[Las anotaciones de herramientas](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) son metadatos opcionales que describen cómo se comporta una herramienta. Páselas como el quinto argumento al helper `tool()` en TypeScript o a través del argumento de palabra clave `annotations` para el decorador `@tool` en Python. Todos los campos de sugerencia son booleanos.

| Campo             | Predeterminado | Significado                                                                                                                          |
| :---------------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `readOnlyHint`    | `false`        | La herramienta no modifica su entorno. Controla si la herramienta puede llamarse en paralelo con otras herramientas de solo lectura. |
| `destructiveHint` | `true`         | La herramienta puede realizar actualizaciones destructivas. Solo informativo.                                                        |
| `idempotentHint`  | `false`        | Las llamadas repetidas con los mismos argumentos no tienen efecto adicional. Solo informativo.                                       |
| `openWorldHint`   | `true`         | La herramienta alcanza sistemas fuera de su proceso. Solo informativo.                                                               |

Las anotaciones son metadatos, no cumplimiento. Una herramienta marcada como `readOnlyHint: true` aún puede escribir en el disco si eso es lo que hace el manejador. Mantenga la anotación precisa con respecto al manejador.

Este ejemplo agrega `readOnlyHint` a la herramienta `get_temperature` del [ejemplo de herramienta meteorológica](#weather-tool-example).

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import tool, ToolAnnotations


  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
      annotations=ToolAnnotations(
          readOnlyHint=True
      ),  # Lets Claude batch this with other read-only calls
  )
  async def get_temperature(args):
      return {"content": [{"type": "text", "text": "..."}]}
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

Consulte `ToolAnnotations` en la referencia de [TypeScript](/docs/es/agent-sdk/typescript#toolannotations) o [Python](/docs/es/agent-sdk/python#toolannotations).

<h2 id="control-tool-access">
  Controlar el acceso a herramientas
</h2>

El [ejemplo de herramienta meteorológica](#weather-tool-example) registró un servidor y enumeró herramientas en `allowedTools`. Esta sección cubre cómo delimitar el acceso cuando tiene varias herramientas o desea restringir las integradas. Para saber cómo se construyen los nombres de herramientas, consulte [Llamar a una herramienta personalizada](#call-a-custom-tool).

<h3 id="configure-allowed-tools">
  Configurar herramientas permitidas
</h3>

La opción `tools` y las listas permitidas/no permitidas afectan dos capas: disponibilidad, que controla si una herramienta aparece en el contexto de Claude, y permiso, que controla si una llamada se aprueba una vez que Claude intenta realizarla. `tools` y las entradas de `disallowedTools` con nombre simple cambian la disponibilidad. `allowedTools` y las reglas de `disallowedTools` con alcance cambian el permiso. Si nombra una de las [herramientas de seguimiento de tareas](/docs/es/agent-sdk/todo-tracking#model-availability) en `allowedTools`, Claude Code también opta por la sesión.

| Opción                     | Capa           | Efecto                                                                                                                                                                                                                                                                                                  |
| :------------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `tools: ["Read", "Grep"]`  | Disponibilidad | Solo las herramientas integradas enumeradas están en el contexto de Claude. Las herramientas integradas no enumeradas se eliminan. Las herramientas MCP no se ven afectadas.                                                                                                                            |
| `tools: []`                | Disponibilidad | Todas las herramientas integradas se eliminan. Claude solo puede usar sus herramientas MCP.                                                                                                                                                                                                             |
| herramientas permitidas    | Permiso        | Las herramientas enumeradas se ejecutan sin un aviso de permiso. Otras herramientas no enumeradas permanecen disponibles; las llamadas pasan por el [flujo de permiso](/docs/es/agent-sdk/permissions).                                                                                                      |
| herramientas no permitidas | Ambas          | Un nombre de herramienta simple como `"Bash"` elimina la herramienta del contexto de Claude, lo mismo que omitirla de `tools`. Una regla con alcance como `"Bash(rm *)"` deja la herramienta en contexto y deniega solo las llamadas coincidentes [como se escriben](/docs/es/permissions#bash-rule-limits). |

Para eliminar una herramienta integrada por completo, omítala de `tools` o enumere su nombre simple en `disallowedTools` (Python: `disallowed_tools`); ambas mantienen la herramienta fuera del contexto para que Claude nunca intente usarla. Una regla de `disallowedTools` con alcance bloquea las llamadas coincidentes pero deja la herramienta visible, por lo que Claude puede desperdiciar un turno intentándolo. Consulte [Configurar permisos](/docs/es/agent-sdk/permissions) para el orden de evaluación completo.

<h2 id="handle-errors">
  Manejar errores
</h2>

Un error del controlador no detiene el bucle del agente. El servidor MCP en proceso del SDK detecta excepciones no capturadas y las devuelve como resultados de error, por lo que la forma en que informa un error determina lo que Claude lee, no si la consulta falla:

| Qué sucede                                                                                    | Resultado                                                                                                                                                   |
| :-------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| El controlador lanza una excepción no capturada                                               | El servidor MCP la convierte en un resultado de error que lleva el mensaje de excepción sin procesar. Claude ve ese mensaje y el bucle del agente continúa. |
| El controlador captura el error y devuelve `isError: true` (TS) / `"is_error": True` (Python) | Claude ve el mensaje que compone. Puede agregar contexto que la excepción sin procesar no tiene, como qué solicitud falló o qué intentar en su lugar.       |

En ambos casos Claude puede reintentar, probar una herramienta diferente o explicar el fallo. Capture errores usted mismo cuando el mensaje de excepción sin procesar no sea suficiente para que Claude actúe.

El ejemplo a continuación captura dos tipos de fallos dentro del controlador y compone el mensaje de error que Claude lee. Un estado HTTP que no es 200 se captura de la respuesta y se devuelve como un resultado de error. Un error de red o JSON inválido se captura por el `try/except` (Python) o `try/catch` (TypeScript) circundante y también se devuelve como un resultado de error. En ambos casos Claude recibe un mensaje que describe el fallo en lugar de una cadena de excepción sin procesar.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any
  from claude_agent_sdk import tool


  @tool(
      "fetch_data",
      "Fetch data from an API",
      {"endpoint": str},  # Simple schema
  )
  async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
      try:
          async with httpx.AsyncClient() as client:
              response = await client.get(args["endpoint"])
              if response.status_code != 200:
                  # Return the failure as a tool result so Claude can react to it.
                  # is_error marks this as a failed call rather than odd-looking data.
                  return {
                      "content": [
                          {
                              "type": "text",
                              "text": f"API error: {response.status_code} {response.reason_phrase}",
                          }
                      ],
                      "is_error": True,
                  }

              data = response.json()
              return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
      except Exception as e:
          # Composes the message Claude reads. An uncaught exception would
          # reach Claude as the raw str(e) with no context.
          return {
              "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
              "is_error": True,
          }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_data",
    "Fetch data from an API",
    {
      endpoint: z.string().url().describe("API endpoint URL")
    },
    async (args) => {
      try {
        const response = await fetch(args.endpoint);

        if (!response.ok) {
          // Return the failure as a tool result so Claude can react to it.
          // isError marks this as a failed call rather than odd-looking data.
          return {
            content: [
              {
                type: "text",
                text: `API error: ${response.status} ${response.statusText}`
              }
            ],
            isError: true
          };
        }

        const data = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(data, null, 2)
            }
          ]
        };
      } catch (error) {
        // Composes the message Claude reads. An uncaught throw would
        // reach Claude as the raw error message with no context.
        return {
          content: [
            {
              type: "text",
              text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
            }
          ],
          isError: true
        };
      }
    }
  );
  ```
</CodeGroup>

<h2 id="return-images-and-resources">
  Devolver imágenes y recursos
</h2>

El array `content` en un resultado de herramienta acepta bloques `text`, `image`, `audio`, `resource` y `resource_link`. Puede mezclarlos en la misma respuesta. En TypeScript, el SDK guarda bloques de audio en disco y Claude recibe un bloque de texto con la ruta del archivo guardado; en Python, el SDK elimina bloques de audio del resultado de la herramienta y registra una advertencia.

Claude recibe cada bloque de enlace de recurso como un bloque de texto que contiene el nombre, URI y descripción del enlace. En TypeScript, su aplicación también recibe los enlaces mismos como [`resourceLinks`](/docs/es/agent-sdk/typescript#sdkmcpresourcelink) en el `tool_use_result` del mensaje del usuario; en Python, el SDK los aplana a texto antes de que la CLI vea el resultado, por lo que la clave [`resourceLinks`](/docs/es/agent-sdk/python#usermessage) de Python nunca se produce para herramientas en proceso.

<h3 id="images">
  Imágenes
</h3>

Un bloque de imagen lleva los bytes de la imagen en línea, codificados como base64. No hay campo de URL. Para devolver una imagen que vive en una URL, búsquela en el controlador, lea los bytes de respuesta y codifíquelos en base64 antes de devolverlos. El resultado se procesa como entrada visual.

| Campo      | Tipo      | Notas                                                                                       |
| :--------- | :-------- | :------------------------------------------------------------------------------------------ |
| `type`     | `"image"` |                                                                                             |
| `data`     | `string`  | Bytes codificados en base64. Solo base64 sin procesar, sin prefijo `data:image/...;base64,` |
| `mimeType` | `string`  | Requerido. Por ejemplo `image/png`, `image/jpeg`, `image/webp`, `image/gif`                 |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx
  from claude_agent_sdk import tool


  # Define a tool that fetches an image from a URL and returns it to Claude
  @tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
  async def fetch_image(args):
      async with httpx.AsyncClient() as client:  # Fetch the image bytes
          response = await client.get(args["url"])

      return {
          "content": [
              {
                  "type": "image",
                  "data": base64.b64encode(response.content).decode(
                      "ascii"
                  ),  # Base64-encode the raw bytes
                  "mimeType": response.headers.get(
                      "content-type", "image/png"
                  ),  # Read MIME type from the response
              }
          ]
      }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_image",
    "Fetch an image from a URL and return it to Claude",
    {
      url: z.string().url()
    },
    async (args) => {
      const response = await fetch(args.url); // Fetch the image bytes
      const buffer = Buffer.from(await response.arrayBuffer()); // Read into a Buffer for base64 encoding
      const mimeType = response.headers.get("content-type") ?? "image/png";

      return {
        content: [
          {
            type: "image",
            data: buffer.toString("base64"), // Base64-encode the raw bytes
            mimeType
          }
        ]
      };
    }
  );
  ```
</CodeGroup>

<h3 id="resources">
  Recursos
</h3>

Un bloque de recurso incrusta un contenido identificado por un URI. El URI es una etiqueta para que Claude la referencie; el contenido real se encuentra en el campo `text` o `blob` del bloque. Úselo cuando su herramienta produce algo que tiene sentido direccionar por nombre más adelante, como un archivo generado o un registro de un sistema externo.

| Campo               | Tipo         | Notas                                                                                                                                                                    |
| :------------------ | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                                                                                                          |
| `resource.uri`      | `string`     | Identificador del contenido. Cualquier esquema de URI                                                                                                                    |
| `resource.text`     | `string`     | El contenido, si es texto. Proporcione este o `blob`, no ambos                                                                                                           |
| `resource.blob`     | `string`     | El contenido codificado en base64, si es binario. Solo TypeScript: el SDK de Python elimina recursos binarios del resultado de la herramienta y registra una advertencia |
| `resource.mimeType` | `string`     | Opcional                                                                                                                                                                 |

Este ejemplo muestra un bloque de recurso devuelto desde dentro de un controlador de herramienta. El URI `file:///tmp/report.md` es una etiqueta que Claude puede referenciar más adelante; el SDK no lee desde esa ruta.

<CodeGroup>
  ```typescript TypeScript theme={null}
  return {
    content: [
      {
        type: "resource",
        resource: {
          uri: "file:///tmp/report.md", // Label for Claude to reference, not a path the SDK reads
          mimeType: "text/markdown",
          text: "# Report\n..." // The actual content, inline
        }
      }
    ]
  };
  ```

  ```python Python theme={null}
  return {
      "content": [
          {
              "type": "resource",
              "resource": {
                  "uri": "file:///tmp/report.md",  # Label for Claude to reference, not a path the SDK reads
                  "mimeType": "text/markdown",
                  "text": "# Report\n...",  # The actual content, inline
              },
          }
      ]
  }
  ```
</CodeGroup>

Estas formas de bloque provienen del tipo MCP `CallToolResult`. Consulte la [especificación de MCP](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result) para la definición completa.

<h2 id="return-structured-data">
  Devolver datos estructurados
</h2>

`structuredContent` es un objeto JSON opcional en el resultado, separado del array `content`. Úselo para devolver valores sin procesar que Claude pueda leer como campos exactos en lugar de analizarlos de una cadena de texto o imagen.

Cuando `structuredContent` está configurado, Claude recibe el JSON más cualquier bloque de imagen o recurso de `content`. Los bloques de texto en `content` no se reenvían, ya que se asume que duplican los datos estructurados. El ejemplo a continuación representa un gráfico como un bloque de imagen y devuelve los puntos de datos detrás de él en `structuredContent` desde el mismo controlador. En el fragmento, `chartPngBuffer` es un `Buffer` que contiene los bytes PNG renderizados.

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

<Note>
  El decorador `@tool` de Python reenvía solo `content` e `is_error` del diccionario de retorno del controlador. Para devolver `structuredContent` desde Python, ejecute un [servidor MCP independiente](/docs/es/agent-sdk/mcp) en lugar de un servidor SDK en proceso.
</Note>

<h2 id="example-unit-converter">
  Ejemplo: convertidor de unidades
</h2>

Esta herramienta convierte valores entre unidades de longitud, temperatura y peso. Un usuario puede preguntar "convertir 100 kilómetros a millas" o "¿cuántos grados Celsius son 72°F?", y Claude elige el tipo de unidad correcto y las unidades de la solicitud.

Demuestra dos patrones:

* **Esquemas de enumeración:** `unit_type` está restringido a un conjunto fijo de valores. En TypeScript, use `z.enum()`. En Python, el esquema dict no admite enumeraciones, por lo que se requiere el esquema JSON Schema completo.
* **Manejo de entrada no compatible:** cuando no se encuentra un par de conversión, el controlador devuelve `isError: true` para que Claude pueda indicar al usuario qué salió mal en lugar de tratar un fallo como un resultado normal.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
  # The dict schema has no equivalent, so full JSON Schema is required.
  @tool(
      "convert_units",
      "Convert a value from one unit to another",
      {
          "type": "object",
          "properties": {
              "unit_type": {
                  "type": "string",
                  "enum": ["length", "temperature", "weight"],
                  "description": "Category of unit",
              },
              "from_unit": {
                  "type": "string",
                  "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
              },
              "to_unit": {"type": "string", "description": "Unit to convert to"},
              "value": {"type": "number", "description": "Value to convert"},
          },
          "required": ["unit_type", "from_unit", "to_unit", "value"],
      },
  )
  async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
      conversions = {
          "length": {
              "kilometers_to_miles": lambda v: v * 0.621371,
              "miles_to_kilometers": lambda v: v * 1.60934,
              "meters_to_feet": lambda v: v * 3.28084,
              "feet_to_meters": lambda v: v * 0.3048,
          },
          "temperature": {
              "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
              "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
              "celsius_to_kelvin": lambda v: v + 273.15,
              "kelvin_to_celsius": lambda v: v - 273.15,
          },
          "weight": {
              "kilograms_to_pounds": lambda v: v * 2.20462,
              "pounds_to_kilograms": lambda v: v * 0.453592,
              "grams_to_ounces": lambda v: v * 0.035274,
              "ounces_to_grams": lambda v: v * 28.3495,
          },
      }

      key = f"{args['from_unit']}_to_{args['to_unit']}"
      fn = conversions.get(args["unit_type"], {}).get(key)

      if not fn:
          return {
              "content": [
                  {
                      "type": "text",
                      "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                  }
              ],
              "is_error": True,
          }

      result = fn(args["value"])
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
              }
          ]
      }


  converter_server = create_sdk_mcp_server(
      name="converter",
      version="1.0.0",
      tools=[convert_units],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  const convert = tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
      unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
      from_unit: z
        .string()
        .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
      to_unit: z.string().describe("Unit to convert to"),
      value: z.number().describe("Value to convert")
    },
    async (args) => {
      type Conversions = Record<string, Record<string, (v: number) => number>>;

      const conversions: Conversions = {
        length: {
          kilometers_to_miles: (v) => v * 0.621371,
          miles_to_kilometers: (v) => v * 1.60934,
          meters_to_feet: (v) => v * 3.28084,
          feet_to_meters: (v) => v * 0.3048
        },
        temperature: {
          celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
          fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
          celsius_to_kelvin: (v) => v + 273.15,
          kelvin_to_celsius: (v) => v - 273.15
        },
        weight: {
          kilograms_to_pounds: (v) => v * 2.20462,
          pounds_to_kilograms: (v) => v * 0.453592,
          grams_to_ounces: (v) => v * 0.035274,
          ounces_to_grams: (v) => v * 28.3495
        }
      };

      const key = `${args.from_unit}_to_${args.to_unit}`;
      const fn = conversions[args.unit_type]?.[key];

      if (!fn) {
        return {
          content: [
            {
              type: "text",
              text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
            }
          ],
          isError: true
        };
      }

      const result = fn(args.value);
      return {
        content: [
          {
            type: "text",
            text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
          }
        ]
      };
    }
  );

  const converterServer = createSdkMcpServer({
    name: "converter",
    version: "1.0.0",
    tools: [convert]
  });
  ```
</CodeGroup>

Una vez que el servidor está definido, páselo a `query` de la misma manera que el ejemplo del clima. Este ejemplo envía tres solicitudes diferentes en un bucle para mostrar la misma herramienta manejando diferentes tipos de unidades. Para cada respuesta, inspecciona objetos `AssistantMessage` (que contienen las llamadas a herramientas que Claude realizó durante ese turno) e imprime cada `ToolUseBlock` antes de imprimir el texto final de `ResultMessage`. Esto le permite ver cuándo Claude está usando la herramienta frente a responder desde su propio conocimiento.

Debido a que [tool search](/docs/es/agent-sdk/tool-search) está activado de forma predeterminada, la salida también puede incluir una llamada a `ToolSearch` cuando Claude carga el esquema de herramienta diferido.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      AssistantMessage,
      ToolUseBlock,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"converter": converter_server},
          allowed_tools=["mcp__converter__convert_units"],
      )

      prompts = [
          "Convert 100 kilometers to miles.",
          "What is 72°F in Celsius?",
          "How many pounds is 5 kilograms?",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt, options=options):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              print(f"[tool call] {block.name}({block.input})")
                  elif isinstance(message, ResultMessage) and message.subtype == "success":
                      print(f"Q: {prompt}\nA: {message.result}\n")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. Only success
              # results are printed above, so handle the failure here and continue with
              # the next prompt.
              print(f"Call failed: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompts = [
    "Convert 100 kilometers to miles.",
    "What is 72°F in Celsius?",
    "How many pounds is 5 kilograms?"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({
        prompt,
        options: {
          mcpServers: { converter: converterServer },
          allowedTools: ["mcp__converter__convert_units"]
        }
      })) {
        if (message.type === "assistant") {
          for (const block of message.message.content) {
            if (block.type === "tool_use") {
              console.log(`[tool call] ${block.name}`, block.input);
            }
          }
        } else if (message.type === "result" && message.subtype === "success") {
          console.log(`Q: ${prompt}\nA: ${message.result}\n`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. Only success
      // results are logged above, so handle the failure here and continue with
      // the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Próximos pasos
</h2>

Puede mezclar los patrones en esta página en el mismo servidor: un único servidor puede contener una herramienta de base de datos, una herramienta de puerta de enlace API y un renderizador de imágenes uno al lado del otro.

Desde aquí:

* Si su servidor crece a docenas de herramientas, consulte [búsqueda de herramientas](/docs/es/agent-sdk/tool-search) para diferir su carga hasta que Claude las necesite.
* Para conectarse a servidores MCP externos (sistema de archivos, GitHub, Slack) en lugar de construir los suyos propios, consulte [Conectar servidores MCP](/docs/es/agent-sdk/mcp).
* Para controlar qué herramientas se ejecutan automáticamente frente a las que requieren aprobación, consulte [Configurar permisos](/docs/es/agent-sdk/permissions).
