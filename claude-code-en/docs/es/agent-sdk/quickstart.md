> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Inicio rápido

> Comience con el SDK de Agent de Python o TypeScript para crear agentes de IA que funcionen de forma autónoma

Utilice el SDK de Agent para crear un agente de IA que lea su código, encuentre errores y los corrija, todo sin intervención manual.

**Lo que hará:**

1. Configurar un proyecto con el SDK de Agent
2. Crear un archivo con código con errores
3. Ejecutar un agente que encuentre y corrija los errores automáticamente

<h2 id="prerequisites">
  Requisitos previos
</h2>

* **Node.js 18+** o **Python 3.10+**
* Una **cuenta de Anthropic** ([regístrese aquí](https://platform.claude.com/))

<h2 id="setup">
  Configuración
</h2>

<Steps>
  <Step title="Crear una carpeta de proyecto">
    Cree un nuevo directorio para este inicio rápido:

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    Para sus propios proyectos, puede ejecutar el SDK desde cualquier carpeta; tendrá acceso a los archivos en ese directorio y sus subdirectorios de forma predeterminada.
  </Step>

  <Step title="Instalar el SDK">
    Instale el paquete del SDK de Agent para su idioma:

    <Tabs>
      <Tab title="TypeScript (nuevo proyecto)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        Establecer `"type": "module"` en `package.json` permite que su script de agente use `await` de nivel superior, y [tsx](https://tsx.is) ejecuta archivos TypeScript directamente.
      </Tab>

      <Tab title="TypeScript (proyecto existente)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is) ejecuta archivos TypeScript directamente. Si su proyecto usa CommonJS, nombre su script de agente `agent.mts` en lugar de `agent.ts`. La extensión `.mts` hace que tsx trate el archivo como un módulo ES, por lo que `await` de nivel superior funciona sin convertir todo su proyecto a módulos ES. Use `agent.mts` en lugar de `agent.ts` en los pasos de creación y ejecución posteriores en este inicio rápido.
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) es un gestor de paquetes de Python rápido que maneja automáticamente los entornos virtuales:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Cree y active un entorno virtual, luego instale el paquete.

        En macOS o Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        En Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Si PowerShell bloquea `Activate.ps1` con un error de política de ejecución, ejecute primero `Set-ExecutionPolicy -Scope Process RemoteSigned`.
      </Tab>
    </Tabs>

    <Note>
      El SDK de TypeScript incluye un binario nativo de Claude Code para su plataforma como una dependencia opcional, por lo que no necesita instalar Claude Code por separado.
    </Note>
  </Step>

  <Step title="Establecer su clave de API">
    Obtenga una clave de API de la [Consola de Claude](https://platform.claude.com/), luego establézcala como una variable de entorno en el shell donde ejecutará su agente:

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    El SDK lee la clave del entorno del proceso que ejecuta su agente; no carga archivos `.env` automáticamente. Si mantiene la clave en un archivo `.env`, cárguela usted mismo, por ejemplo con el paquete `dotenv`, antes de llamar al SDK.

    El SDK también admite autenticación a través de proveedores de API de terceros:

    * **Amazon Bedrock**: establezca la variable de entorno `CLAUDE_CODE_USE_BEDROCK=1` y configure las credenciales de AWS
    * **Claude Platform on AWS**: establezca `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` y `ANTHROPIC_AWS_WORKSPACE_ID`, luego configure las credenciales de AWS
    * **Google Cloud's Agent Platform**: establezca la variable de entorno `CLAUDE_CODE_USE_VERTEX=1` y configure las credenciales de Google Cloud
    * **Microsoft Azure**: establezca la variable de entorno `CLAUDE_CODE_USE_FOUNDRY=1` y configure las credenciales de Azure

    Consulte las guías de configuración para [Amazon Bedrock](/docs/es/amazon-bedrock), [Claude Platform on AWS](/docs/es/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), o [Microsoft Foundry](/docs/es/microsoft-foundry) para obtener más detalles.

    <Note>
      A menos que haya sido aprobado previamente, Anthropic no permite que desarrolladores de terceros ofrezcan inicio de sesión en claude.ai o límites de velocidad para sus productos, incluidos los agentes construidos en el SDK de Agent de Claude. Por favor, utilice los métodos de autenticación de clave de API descritos en este documento en su lugar.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  Crear un archivo con errores
</h2>

Este inicio rápido lo guía a través de la construcción de un agente que puede encontrar y corregir errores en el código. Primero, necesita un archivo con algunos errores intencionales para que el agente corrija. Cree `utils.py` en el directorio `my-agent` y pegue el siguiente código:

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

Este código tiene dos errores:

1. `calculate_average([])` se bloquea con una división por cero
2. `get_user_name(None)` se bloquea con un TypeError

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  Construir un agente que encuentre y corrija errores
</h2>

Cree `agent.py` si está utilizando el SDK de Python, o `agent.ts` para TypeScript. Use `agent.mts` en su lugar si su proyecto existente usa CommonJS:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

Este código tiene tres partes principales:

1. **`query`**: el punto de entrada principal que crea el bucle agentic. Devuelve un iterador asincrónico, por lo que utiliza `async for` para transmitir mensajes mientras Claude trabaja. Consulte la API completa en la referencia del SDK de [Python](/docs/es/agent-sdk/python#query) o [TypeScript](/docs/es/agent-sdk/typescript#query).

2. **`prompt`**: lo que desea que haga Claude. Claude determina qué herramientas usar en función de la tarea.

3. **`options`**: configuración para el agente. Este ejemplo utiliza `allowedTools` para preautorizar `Read`, `Edit` y `Glob`, y `permissionMode: "acceptEdits"` para aprobar automáticamente los cambios de archivo. Otras opciones incluyen `systemPrompt`, `mcpServers` y más. Consulte todas las opciones para [Python](/docs/es/agent-sdk/python#claudeagentoptions) o [TypeScript](/docs/es/agent-sdk/typescript#options).

El bucle `async for` continúa ejecutándose mientras Claude piensa, llama a herramientas, observa resultados y decide qué hacer a continuación. Cada iteración produce un mensaje: el razonamiento de Claude, una llamada a herramienta, un resultado de herramienta o el resultado final. El SDK maneja la orquestación (ejecución de herramientas, gestión de contexto, reintentos) para que solo consuma el flujo. El bucle termina cuando Claude completa la tarea o encuentra un error.

El manejo de mensajes dentro del bucle filtra la salida legible por humanos. Sin filtrado, vería objetos de mensaje sin procesar, incluida la inicialización del sistema y el estado interno, lo que es útil para depuración pero ruidoso de otra manera.

<Note>
  Este ejemplo utiliza transmisión para mostrar el progreso en tiempo real. Si no necesita salida en vivo (por ejemplo, para trabajos en segundo plano o canalizaciones de CI), puede recopilar todos los mensajes a la vez. Consulte [Transmisión frente a modo de un solo turno](/docs/es/agent-sdk/streaming-vs-single-mode) para obtener más detalles.
</Note>

<h3 id="run-your-agent">
  Ejecutar su agente
</h3>

Su agente está listo. Ejecútelo con el siguiente comando:

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    Si nombró su script `agent.mts`, ejecute `npx tsx agent.mts` en su lugar.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    Con su entorno virtual aún activado:

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

Mientras trabaja, el agente imprime su razonamiento y cada herramienta que llama, terminando con `Done: success`. Después de ejecutar, verifique `utils.py`. Verá código defensivo que maneja listas vacías y usuarios nulos. Su agente de forma autónoma:

1. **Leyó** `utils.py` para entender el código
2. **Analizó** la lógica e identificó casos extremos que causarían bloqueos
3. **Editó** el archivo para agregar manejo de errores adecuado

Esto es lo que hace diferente al SDK de Agent: Claude ejecuta herramientas directamente en lugar de pedirle que las implemente.

<Note>
  Si ve "API key not found", asegúrese de haber establecido la variable de entorno `ANTHROPIC_API_KEY` en el shell donde ejecuta su agente. El SDK no carga archivos `.env` automáticamente. Consulte la [guía completa de solución de problemas](/docs/es/troubleshooting) para obtener más ayuda.
</Note>

<h3 id="try-other-prompts">
  Probar otros prompts
</h3>

Ahora que su agente está configurado, pruebe algunos prompts diferentes:

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  Personalizar su agente
</h3>

Puede modificar el comportamiento de su agente cambiando las opciones. Aquí hay algunos ejemplos:

**Agregar capacidad de búsqueda web:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Dar a Claude un prompt de sistema personalizado:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**Ejecutar comandos en la terminal:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

Con `Bash` habilitado, intente: `"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  Conceptos clave
</h2>

**Tools** controlan lo que su agente puede hacer:

| Herramientas                           | Lo que el agente puede hacer |
| -------------------------------------- | ---------------------------- |
| `Read`, `Glob`, `Grep`                 | Análisis de solo lectura     |
| `Read`, `Edit`, `Glob`                 | Analizar y modificar código  |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Automatización completa      |

**Modos de permiso** controlan cuánta supervisión humana desea:

| Modo                | Comportamiento                                                                                                                                                                                                                                                                                                                                  | Caso de uso                                                       |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `acceptEdits`       | Aprueba automáticamente ediciones de archivo y comandos comunes del sistema de archivos, pregunta por otras acciones                                                                                                                                                                                                                            | Flujos de trabajo de desarrollo confiables                        |
| `plan`              | Ejecuta herramientas de solo lectura; las ediciones de archivo nunca se aprueban automáticamente y llegan a su devolución de llamada `canUseTool`                                                                                                                                                                                               | Definición del alcance de una tarea antes de aprobar la ejecución |
| `dontAsk`           | Deniega cualquier cosa que no esté en `allowedTools`; herramientas de conector [su organización configurada para `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas que requieren interacción del usuario se deniegan incluso si las ha incluido en la lista                                                               | Agentes sin cabeza bloqueados                                     |
| `auto`              | Un clasificador de modelo aprueba o deniega cada llamada de herramienta                                                                                                                                                                                                                                                                         | Agentes autónomos con protecciones de seguridad                   |
| `bypassPermissions` | Ejecuta cada herramienta sin indicadores, excepto herramientas coincidentes por una [regla `ask` explícita](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated), herramientas de conector [su organización configurada para `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas que requieren interacción del usuario | CI en sandbox, entornos completamente confiables                  |
| `default`           | Requiere una devolución de llamada `canUseTool` para manejar la aprobación                                                                                                                                                                                                                                                                      | Flujos de aprobación personalizados                               |

El ejemplo anterior utiliza el modo `acceptEdits`, que aprueba automáticamente las operaciones de archivo para que el agente pueda ejecutarse sin indicadores interactivos. Si desea solicitar a los usuarios la aprobación, utilice el modo `default` y proporcione una devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) que recopile la entrada del usuario. Para más control, consulte [Permisos](/docs/es/agent-sdk/permissions).

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que ha creado su primer agente, aprenda cómo extender sus capacidades y adaptarlo a su caso de uso:

* **[Permisos](/docs/es/agent-sdk/permissions)**: controle lo que su agente puede hacer y cuándo necesita aprobación
* **[Hooks](/docs/es/agent-sdk/hooks)**: ejecute código personalizado antes o después de llamadas de herramientas
* **[Sesiones](/docs/es/agent-sdk/sessions)**: construya agentes de múltiples turnos que mantengan contexto
* **[Servidores MCP](/docs/es/agent-sdk/mcp)**: conéctese a bases de datos, navegadores, API y otros sistemas externos
* **[Hosting](/docs/es/agent-sdk/hosting)**: implemente agentes en Docker, nube e CI/CD
* **[Agentes de ejemplo](https://github.com/anthropics/claude-agent-sdk-demos)**: vea ejemplos completos: asistente de correo electrónico, agente de investigación y más
