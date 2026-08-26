> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Obtener salida estructurada de agentes

> Devuelve JSON validado desde flujos de trabajo de agentes usando JSON Schema, Zod o Pydantic. Obtén datos estructurados seguros en tipos después del uso de herramientas de múltiples turnos.

Las salidas estructuradas le permiten definir la forma exacta de los datos que desea recuperar de un agente. El agente puede usar cualquier herramienta que necesite para completar la tarea, y aún así obtiene JSON validado que coincide con su esquema al final. Defina un [JSON Schema](https://json-schema.org/understanding-json-schema/about) para la estructura que necesita, y el SDK valida la salida en su contra, re-solicitando en caso de discrepancia. Si la validación no tiene éxito dentro del límite de reintentos, el resultado es un error en lugar de datos estructurados; consulte [Manejo de errores](#error-handling).

Para una seguridad de tipos completa, use [Zod](#type-safe-schemas-with-zod-and-pydantic) (TypeScript) o [Pydantic](#type-safe-schemas-with-zod-and-pydantic) (Python) para definir su esquema y obtener objetos fuertemente tipados.

<h2 id="why-structured-outputs">
  ¿Por qué salidas estructuradas?
</h2>

Los agentes devuelven texto de forma libre de forma predeterminada, lo que funciona para chat pero no cuando necesita usar la salida mediante programación. Las salidas estructuradas le proporcionan datos tipados que puede pasar directamente a su lógica de aplicación, base de datos o componentes de interfaz de usuario.

Considere una aplicación de recetas donde un agente busca en la web y trae recetas. Sin salidas estructuradas, obtiene texto de forma libre que tendría que analizar usted mismo. Con salidas estructuradas, define la forma que desea y obtiene datos tipados que puede usar directamente en su aplicación.

<AccordionGroup>
  <Accordion title="Sin salidas estructuradas">
    ```text theme={null}
    ¡Aquí hay una receta clásica de galletas con chispas de chocolate!

    **Galletas con Chispas de Chocolate**
    Tiempo de preparación: 15 minutos | Tiempo de cocción: 10 minutos

    Ingredientes:
    - 2 1/4 tazas de harina para todo uso
    - 1 taza de mantequilla, ablandada
    ...
    ```

    Para usar esto en su aplicación, tendría que analizar el título, convertir "15 minutos" a un número, separar ingredientes de instrucciones y manejar el formato inconsistente en todas las respuestas.
  </Accordion>

  <Accordion title="Con salidas estructuradas">
    ```json theme={null}
    {
      "name": "Galletas con Chispas de Chocolate",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "harina para todo uso", "amount": 2.25, "unit": "tazas" },
        { "item": "mantequilla, ablandada", "amount": 1, "unit": "taza" }
        // ...
      ],
      "steps": ["Precalentar horno a 375°F", "Mezclar mantequilla y azúcar" /* ... */]
    }
    ```

    Datos tipados que puede usar directamente en su interfaz de usuario.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  Inicio rápido
</h2>

Para usar salidas estructuradas, defina un [JSON Schema](https://json-schema.org/understanding-json-schema/about) que describa la forma de los datos que desea, luego páselo a `query()` a través de la opción `outputFormat` (TypeScript) u opción `output_format` (Python). Cuando el agente termina, el mensaje de resultado incluye un campo `structured_output` con datos validados que coinciden con su esquema.

El ejemplo a continuación le pide al agente que investigue Anthropic y devuelva el nombre de la empresa, año de fundación y sede como salida estructurada.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Define la forma de los datos que desea recuperar
  const schema = {
    type: "object",
    properties: {
      company_name: { type: "string" },
      founded_year: { type: "number" },
      headquarters: { type: "string" }
    },
    required: ["company_name"]
  };

  for await (const message of query({
    prompt: "Investigue Anthropic y proporcione información clave de la empresa",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // El mensaje de resultado contiene structured_output con datos validados
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Define la forma de los datos que desea recuperar
  schema = {
      "type": "object",
      "properties": {
          "company_name": {"type": "string"},
          "founded_year": {"type": "number"},
          "headquarters": {"type": "string"},
      },
      "required": ["company_name"],
  }


  async def main():
      async for message in query(
          prompt="Investigue Anthropic y proporcione información clave de la empresa",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": schema}
          ),
      ):
          # El mensaje de resultado contiene structured_output con datos validados
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Esquemas seguros en tipos con Zod y Pydantic
</h2>

En lugar de escribir JSON Schema a mano, puede usar [Zod](https://zod.dev/) (TypeScript) o [Pydantic](https://docs.pydantic.dev/latest/) (Python) para definir su esquema. Estas bibliotecas generan el JSON Schema para usted y le permiten analizar la respuesta en un objeto completamente tipado que puede usar en todo su código con autocompletado y verificación de tipos.

El ejemplo a continuación define un esquema para un plan de implementación de características con un resumen, lista de pasos (cada uno con nivel de complejidad) y riesgos potenciales. El agente planifica la característica y devuelve un objeto `FeaturePlan` tipado. Luego puede acceder a propiedades como `plan.summary` e iterar sobre `plan.steps` con seguridad de tipos completa.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Define esquema con Zod
  const FeaturePlan = z.object({
    feature_name: z.string(),
    summary: z.string(),
    steps: z.array(
      z.object({
        step_number: z.number(),
        description: z.string(),
        estimated_complexity: z.enum(["low", "medium", "high"])
      })
    ),
    risks: z.array(z.string())
  });

  type FeaturePlan = z.infer<typeof FeaturePlan>;

  // Convertir a JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // Usar en consulta
  for await (const message of query({
    prompt:
      "Planifique cómo agregar soporte de modo oscuro a una aplicación React. Divídalo en pasos de implementación.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // Validar y obtener resultado completamente tipado
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`Característica: ${plan.feature_name}`);
        console.log(`Resumen: ${plan.summary}`);
        plan.steps.forEach((step) => {
          console.log(`${step.step_number}. [${step.estimated_complexity}] ${step.description}`);
        });
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from pydantic import BaseModel
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  class Step(BaseModel):
      step_number: int
      description: str
      estimated_complexity: str  # 'low', 'medium', 'high'


  class FeaturePlan(BaseModel):
      feature_name: str
      summary: str
      steps: list[Step]
      risks: list[str]


  async def main():
      async for message in query(
          prompt="Planifique cómo agregar soporte de modo oscuro a una aplicación React. Divídalo en pasos de implementación.",
          options=ClaudeAgentOptions(
              output_format={
                  "type": "json_schema",
                  "schema": FeaturePlan.model_json_schema(),
              }
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              # Validar y obtener resultado completamente tipado
              plan = FeaturePlan.model_validate(message.structured_output)
              print(f"Característica: {plan.feature_name}")
              print(f"Resumen: {plan.summary}")
              for step in plan.steps:
                  print(
                      f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                  )


  asyncio.run(main())
  ```
</CodeGroup>

**Beneficios:**

* Inferencia de tipos completa (TypeScript) e indicaciones de tipos (Python)
* Validación en tiempo de ejecución con `safeParse()` o `model_validate()`
* Mejores mensajes de error
* Esquemas componibles y reutilizables

<h2 id="output-format-configuration">
  Configuración del formato de salida
</h2>

La opción `outputFormat` (TypeScript) u `output_format` (Python) acepta un objeto con:

* `type`: Establezca en `"json_schema"` para salidas estructuradas
* `schema`: Un objeto [JSON Schema](https://json-schema.org/understanding-json-schema/about) que define su estructura de salida. Puede generar esto a partir de un esquema Zod con `z.toJSONSchema()` o un modelo Pydantic con `.model_json_schema()`

El SDK admite características estándar de JSON Schema, incluidos todos los tipos básicos (objeto, matriz, cadena, número, booleano, nulo), `enum`, `const`, `required`, objetos anidados y definiciones `$ref`. Para la lista completa de características admitidas y limitaciones, consulte [Limitaciones de JSON Schema](https://platform.claude.com/docs/es/build-with-claude/structured-outputs#json-schema-limitations).

Un esquema que no es un JSON Schema válido falla la ejecución al inicio con un error que nombra el problema. Antes de v2.1.205, un esquema inválido se ignoraba silenciosamente y el agente devolvía texto no estructurado.

La palabra clave `format`, como `"format": "email"`, se acepta como una anotación y no se aplica mediante el validador del SDK. Antes de v2.1.205, cualquier esquema que contenga `format` se trataba como inválido.

<h2 id="example-todo-tracking-agent">
  Ejemplo: Agente de seguimiento de TODO
</h2>

Este ejemplo demuestra cómo funcionan las salidas estructuradas con el uso de herramientas de múltiples pasos. El agente necesita encontrar comentarios TODO en la base de código, luego buscar información de git blame para cada uno. Decide autónomamente qué herramientas usar (Grep para buscar, Bash para ejecutar comandos git) y combina los resultados en una única respuesta estructurada.

El esquema incluye campos opcionales (`author` y `date`) ya que la información de git blame podría no estar disponible para todos los archivos. El agente completa lo que puede encontrar y omite el resto.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Define estructura para extracción de TODO
  const todoSchema = {
    type: "object",
    properties: {
      todos: {
        type: "array",
        items: {
          type: "object",
          properties: {
            text: { type: "string" },
            file: { type: "string" },
            line: { type: "number" },
            author: { type: "string" },
            date: { type: "string" }
          },
          required: ["text", "file", "line"]
        }
      },
      total_count: { type: "number" }
    },
    required: ["todos", "total_count"]
  };

  // El agente usa Grep para encontrar TODOs, Bash para obtener información de git blame
  for await (const message of query({
    prompt: "Encuentre todos los comentarios TODO en esta base de código e identifique quién los agregó",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`Se encontraron ${data.total_count} TODOs`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  Agregado por ${todo.author} el ${todo.date}`);
        }
      });
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Define estructura para extracción de TODO
  todo_schema = {
      "type": "object",
      "properties": {
          "todos": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                      "text": {"type": "string"},
                      "file": {"type": "string"},
                      "line": {"type": "number"},
                      "author": {"type": "string"},
                      "date": {"type": "string"},
                  },
                  "required": ["text", "file", "line"],
              },
          },
          "total_count": {"type": "number"},
      },
      "required": ["todos", "total_count"],
  }


  async def main():
      # El agente usa Grep para encontrar TODOs, Bash para obtener información de git blame
      async for message in query(
          prompt="Encuentre todos los comentarios TODO en esta base de código e identifique quién los agregó",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": todo_schema}
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              data = message.structured_output
              print(f"Se encontraron {data['total_count']} TODOs")
              for todo in data["todos"]:
                  print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                  if "author" in todo:
                      print(f"  Agregado por {todo['author']} el {todo['date']}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Manejo de errores
</h2>

La generación de salida estructurada puede fallar cuando el agente no puede producir JSON válido que coincida con su esquema. Esto típicamente sucede cuando el esquema es demasiado complejo para la tarea, la tarea en sí es ambigua, o el agente alcanza su límite de reintentos intentando corregir errores de validación. También puede suceder sin ninguna falla de validación: una [alternativa de modelo](/docs/es/model-config#automatic-model-fallback) puede retraer una salida ya completada a mitad de la transmisión, y si ningún reintento la reemplaza, la ejecución termina con el mismo error. Verifique el campo `errors` en el mensaje de resultado para distinguir las dos causas antes de depurar su esquema.

Cuando ocurre un error, el mensaje de resultado tiene un `subtype` que indica qué salió mal:

| Subtype                               | Significado                                                                                                                                            |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `success`                             | La salida fue generada y validada exitosamente                                                                                                         |
| `error_max_structured_output_retries` | Ninguna salida válida sobrevivió después de múltiples intentos (fallos de validación, o una retracción de alternativa de modelo sin reintento exitoso) |

El ejemplo a continuación verifica el campo `subtype` para determinar si la salida fue generada exitosamente o si necesita manejar una falla:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const msg of query({
    prompt: "Extract contact info from the document",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: contactSchema
      }
    }
  })) {
    if (msg.type === "result") {
      if (msg.subtype === "success" && msg.structured_output) {
        // Use the validated output
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // Handle the failure - retry with simpler prompt, fall back to unstructured, etc.
        console.error("Could not produce valid output");
      }
    }
  }
  ```

  ```python Python theme={null}
  async for message in query(
      prompt="Extract contact info from the document",
      options=ClaudeAgentOptions(
          output_format={"type": "json_schema", "schema": contact_schema}
      ),
  ):
      if isinstance(message, ResultMessage):
          if message.subtype == "success" and message.structured_output:
              # Use the validated output
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # Handle the failure
              print("Could not produce valid output")
  ```
</CodeGroup>

**Consejos para evitar errores:**

* **Mantenga esquemas enfocados.** Los esquemas profundamente anidados con muchos campos requeridos son más difíciles de satisfacer. Comience simple y agregue complejidad según sea necesario.
* **Haga coincidir el esquema con la tarea.** Si la tarea podría no tener toda la información que su esquema requiere, haga esos campos opcionales.
* **Use solicitudes claras.** Las solicitudes ambiguas hacen que sea más difícil para el agente saber qué salida producir.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Documentación de JSON Schema](https://json-schema.org/): aprenda la sintaxis de JSON Schema para definir esquemas complejos con objetos anidados, matrices, enumeraciones y restricciones de validación
* [API Structured Outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs): use salidas estructuradas con la API de Claude directamente para solicitudes de un solo turno sin uso de herramientas
* [Custom tools](/docs/es/agent-sdk/custom-tools): proporcione a su agente herramientas personalizadas para llamar durante la ejecución antes de devolver salida estructurada
