> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Descripción general del Agent SDK

> Construya agentes de IA en producción con Claude Code como una biblioteca

Construya agentes de IA que lean archivos de forma autónoma, ejecuten comandos, busquen en la web, editen código y mucho más. El Agent SDK le proporciona las mismas herramientas, bucle de agente y gestión de contexto que potencian Claude Code, programable en Python y TypeScript. Para conocer el razonamiento detrás del diseño del harness de agentes, consulte [A harness for every task: dynamic workflows in Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) en el blog.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

El Agent SDK incluye herramientas integradas para leer archivos, ejecutar comandos y editar código, por lo que su agente puede comenzar a trabajar inmediatamente sin que usted implemente la ejecución de herramientas. Sumérjase en el inicio rápido o explore agentes reales construidos con el SDK:

<CardGroup cols={2}>
  <Card title="Inicio rápido" icon="play" href="/docs/es/agent-sdk/quickstart">
    Construya un agente corrector de errores en minutos
  </Card>

  <Card title="Agentes de ejemplo" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Asistente de correo electrónico, agente de investigación y más
  </Card>
</CardGroup>

<h2 id="get-started">
  Comenzar
</h2>

<Steps>
  <Step title="Instale el SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) es un gestor de paquetes de Python rápido que maneja entornos virtuales automáticamente:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Cree y active un entorno virtual, luego instale el paquete. Instalar en un entorno virtual evita el error `error: externally-managed-environment` que devuelve Python del sistema en instalaciones recientes de Debian, Ubuntu y Homebrew para `pip install` fuera de un venv.

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

        El paquete de Python requiere Python 3.10 o posterior. Si pip reporta `No matching distribution found for claude-agent-sdk`, su intérprete es anterior a 3.10. Ejecute `python3 --version` en macOS o Linux, o `py --version` en Windows, para verificar.
      </Tab>
    </Tabs>

    <Note>
      El SDK de TypeScript agrupa un binario nativo de Claude Code para su plataforma como una dependencia opcional, por lo que no necesita instalar Claude Code por separado.
    </Note>
  </Step>

  <Step title="Configure su clave de API">
    Obtenga una clave de API de la [Consola](https://platform.claude.com/), luego configúrela como una variable de entorno.

    En macOS o Linux:

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    En Windows PowerShell:

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    El SDK también admite autenticación a través de proveedores de API de terceros:

    * **Amazon Bedrock**: configure la variable de entorno `CLAUDE_CODE_USE_BEDROCK=1` y configure las credenciales de AWS
    * **Claude Platform on AWS**: configure `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` y `ANTHROPIC_AWS_WORKSPACE_ID`, luego configure las credenciales de AWS
    * **Google Cloud's Agent Platform**: configure la variable de entorno `CLAUDE_CODE_USE_VERTEX=1` y configure las credenciales de Google Cloud
    * **Microsoft Azure**: configure la variable de entorno `CLAUDE_CODE_USE_FOUNDRY=1` y configure las credenciales de Azure

    Consulte las guías de configuración para [Amazon Bedrock](/docs/es/amazon-bedrock), [Claude Platform on AWS](/docs/es/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) o [Microsoft Foundry](/docs/es/microsoft-foundry) para obtener más detalles.

    <Note>
      A menos que haya sido aprobado previamente, Anthropic no permite que desarrolladores de terceros ofrezcan inicio de sesión en claude.ai o límites de velocidad para sus productos, incluidos los agentes construidos en el Claude Agent SDK. Por favor, utilice los métodos de autenticación de clave de API descritos en este documento en su lugar.
    </Note>
  </Step>

  <Step title="Ejecute su primer agente">
    Este ejemplo crea un agente que enumera archivos en su directorio actual utilizando herramientas integradas.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**¿Listo para construir?** Siga el [Inicio rápido](/docs/es/agent-sdk/quickstart) para crear un agente que encuentre y corrija errores en minutos.

<h2 id="capabilities">
  Capacidades
</h2>

Todo lo que hace que Claude Code sea poderoso está disponible en el SDK:

<Tabs>
  <Tab title="Herramientas integradas">
    Su agente puede leer archivos, ejecutar comandos y buscar en bases de código de forma inmediata. Las herramientas clave incluyen:

    | Herramienta                                                                 | Qué hace                                                                       |
    | --------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
    | **Read**                                                                    | Leer cualquier archivo en el directorio de trabajo                             |
    | **Write**                                                                   | Crear nuevos archivos                                                          |
    | **Edit**                                                                    | Realizar ediciones precisas en archivos existentes                             |
    | **Bash**                                                                    | Ejecutar comandos de terminal, scripts, operaciones de git                     |
    | **Monitor**                                                                 | Observar un script de fondo y reaccionar a cada línea de salida como un evento |
    | **Glob**                                                                    | Encontrar archivos por patrón (`**/*.ts`, `src/**/*.py`)                       |
    | **Grep**                                                                    | Buscar contenido de archivos con expresiones regulares                         |
    | **WebSearch**                                                               | Buscar en la web información actual                                            |
    | **WebFetch**                                                                | Obtener y analizar contenido de páginas web                                    |
    | **[AskUserQuestion](/docs/es/agent-sdk/user-input#handle-clarifying-questions)** | Hacer preguntas aclaratorias al usuario con opciones de opción múltiple        |

    Este ejemplo crea un agente que busca comentarios TODO en su base de código:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Hooks">
    Ejecute código personalizado en puntos clave del ciclo de vida del agente. Los hooks del SDK utilizan funciones de devolución de llamada para validar, registrar, bloquear o transformar el comportamiento del agente.

    **Hooks disponibles:** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit` y más.

    Este ejemplo registra todos los cambios de archivo en un archivo de auditoría:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Obtenga más información sobre hooks →](/docs/es/agent-sdk/hooks)
  </Tab>

  <Tab title="Subagentes">
    Genere agentes especializados para manejar subtareas enfocadas. Su agente principal delega trabajo y los subagentes informan con resultados.

    Defina agentes personalizados con instrucciones especializadas. Los subagentes se invocan a través de la herramienta Agent, así que incluya `Agent` en `allowedTools` para aprobar automáticamente esas invocaciones:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
                      )
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    Los mensajes dentro del contexto de un subagente incluyen un campo `parent_tool_use_id`, lo que le permite rastrear qué mensajes pertenecen a qué ejecución de subagente.

    [Obtenga más información sobre subagentes →](/docs/es/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    Conéctese a sistemas externos a través del Protocolo de Contexto del Modelo: bases de datos, navegadores, API y [cientos más](https://github.com/modelcontextprotocol/servers).

    Este ejemplo conecta el [servidor Playwright MCP](https://github.com/microsoft/playwright-mcp) para dar a su agente capacidades de automatización del navegador:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Obtenga más información sobre MCP →](/docs/es/agent-sdk/mcp)
  </Tab>

  <Tab title="Permisos">
    Controle exactamente qué herramientas puede usar su agente. Permita operaciones seguras, bloquee las peligrosas o requiera aprobación para acciones sensibles.

    <Note>
      Para solicitudes de aprobación interactivas y la herramienta `AskUserQuestion`, consulte [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input).
    </Note>

    Este ejemplo crea un agente de solo lectura que puede analizar pero no modificar código. `allowed_tools` aprueba previamente `Read`, `Glob` y `Grep`.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Obtenga más información sobre permisos →](/docs/es/agent-sdk/permissions)
  </Tab>

  <Tab title="Sesiones">
    Mantenga el contexto en múltiples intercambios. Claude recuerda archivos leídos, análisis realizados e historial de conversación. Reanude sesiones más tarde o divídalas para explorar diferentes enfoques.

    Este ejemplo captura el ID de sesión de la primera consulta, luego reanuda para continuar con contexto completo:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Obtenga más información sobre sesiones →](/docs/es/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Características de Claude Code
</h3>

El SDK también admite la configuración basada en el sistema de archivos de Claude Code. Con opciones predeterminadas, el SDK carga estas desde `.claude/` en su directorio de trabajo y `~/.claude/`. Para restringir qué fuentes se cargan, configure `setting_sources` (Python) o `settingSources` (TypeScript) en sus opciones.

| Característica                                   | Descripción                                                                                        | Ubicación                                    |
| ------------------------------------------------ | -------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| [Skills](/docs/es/agent-sdk/skills)                   | Capacidades especializadas que Claude utiliza automáticamente o que usted invoca con `/name`       | `.claude/skills/*/SKILL.md`                  |
| [Commands](/docs/es/agent-sdk/slash-commands)         | Comandos personalizados en el formato heredado. Utilice skills para nuevos comandos personalizados | `.claude/commands/*.md`                      |
| [Memory](/docs/es/agent-sdk/modifying-system-prompts) | Contexto e instrucciones del proyecto                                                              | `CLAUDE.md` o `.claude/CLAUDE.md`            |
| [Plugins](/docs/es/agent-sdk/plugins)                 | Extienda con skills, agentes, hooks y servidores MCP                                               | Programático a través de la opción `plugins` |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Compare el Agent SDK con otras herramientas de Claude
</h2>

La Plataforma Claude ofrece múltiples formas de construir con Claude. Así es como se ajusta el Agent SDK:

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    El [Anthropic Client SDK](https://platform.claude.com/docs/es/api/client-sdks) le proporciona acceso directo a la API: usted envía solicitudes e implementa la ejecución de herramientas usted mismo. El **Agent SDK** le proporciona Claude con ejecución de herramientas integrada.

    Con el Client SDK, implementa un bucle de herramientas. Con el Agent SDK, Claude lo maneja:

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Agent SDK vs Claude Code CLI">
    Mismas capacidades, interfaz diferente:

    | Caso de uso                  | Mejor opción |
    | ---------------------------- | ------------ |
    | Desarrollo interactivo       | CLI          |
    | Canalizaciones CI/CD         | SDK          |
    | Aplicaciones personalizadas  | SDK          |
    | Tareas puntuales             | CLI          |
    | Automatización en producción | SDK          |

    Muchos equipos usan ambos: CLI para desarrollo diario, SDK para producción. Los flujos de trabajo se traducen directamente entre ellos.
  </Tab>

  <Tab title="Agent SDK vs Managed Agents">
    [Managed Agents](https://platform.claude.com/docs/es/managed-agents/overview) es una API REST alojada: Anthropic ejecuta el agente y el sandbox, y su aplicación envía eventos y transmite resultados. El **Agent SDK** es una biblioteca que ejecuta el bucle del agente dentro de su propio proceso.

    |                                 | Agent SDK                                                                                  | Managed Agents                                                                                                  |
    | ------------------------------- | ------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- |
    | **Se ejecuta en**               | Su proceso, su infraestructura                                                             | Infraestructura administrada por Anthropic                                                                      |
    | **Interfaz**                    | Biblioteca de Python o TypeScript                                                          | API REST                                                                                                        |
    | **El agente trabaja en**        | Archivos en su infraestructura                                                             | Un sandbox administrado por sesión                                                                              |
    | **Estado de la sesión**         | JSONL en su sistema de archivos                                                            | Registro de eventos alojado por Anthropic                                                                       |
    | **Herramientas personalizadas** | Funciones de Python o TypeScript en proceso                                                | Claude activa la herramienta; usted ejecuta y devuelve resultados                                               |
    | **Mejor para**                  | Prototipado local, agentes que trabajan directamente en su sistema de archivos y servicios | Agentes de producción sin operar infraestructura de sandbox o sesión, sesiones de larga duración y asincrónicas |

    Una ruta común es crear prototipos con el Agent SDK localmente, luego pasar a Managed Agents para producción.
  </Tab>
</Tabs>

<h2 id="changelog">
  Registro de cambios
</h2>

Vea el registro de cambios completo para actualizaciones del SDK, correcciones de errores y nuevas características:

* **TypeScript SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  Reportar errores
</h2>

Si encuentra errores o problemas con el Agent SDK:

* **TypeScript SDK**: [reportar problemas en GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [reportar problemas en GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Directrices de marca
</h2>

Para socios que integran el Claude Agent SDK, el uso de la marca Claude es opcional. Al hacer referencia a Claude en su producto:

**Permitido:**

* "Claude Agent" (preferido para menús desplegables)
* "Claude" (cuando ya está dentro de un menú etiquetado como "Agents")
* "{YourAgentName} Powered by Claude" (si tiene un nombre de agente existente)

**No permitido:**

* "Claude Code" o "Claude Code Agent"
* Arte ASCII de marca Claude Code o elementos visuales que imiten Claude Code

Su producto debe mantener su propia marca y no parecer ser Claude Code o ningún producto de Anthropic. Para preguntas sobre cumplimiento de marca, póngase en contacto con el [equipo de ventas](https://www.anthropic.com/contact-sales) de Anthropic.

<h2 id="license-and-terms">
  Licencia y términos
</h2>

El uso del Claude Agent SDK se rige por los [Términos de Servicio Comerciales de Anthropic](https://www.anthropic.com/legal/commercial-terms), incluso cuando lo utiliza para potenciar productos y servicios que pone a disposición de sus propios clientes y usuarios finales, excepto en la medida en que un componente específico o dependencia esté cubierto por una licencia diferente como se indica en el archivo LICENSE de ese componente.

<h2 id="next-steps">
  Próximos pasos
</h2>

<CardGroup cols={2}>
  <Card title="Inicio rápido" icon="play" href="/docs/es/agent-sdk/quickstart">
    Construya un agente que encuentre y corrija errores en minutos
  </Card>

  <Card title="Agentes de ejemplo" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Asistente de correo electrónico, agente de investigación y más
  </Card>

  <Card title="TypeScript SDK" icon="code" href="/docs/es/agent-sdk/typescript">
    Referencia completa de API de TypeScript y ejemplos
  </Card>

  <Card title="Python SDK" icon="code" href="/docs/es/agent-sdk/python">
    Referencia completa de API de Python y ejemplos
  </Card>
</CardGroup>
