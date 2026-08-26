> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conectar con herramientas externas usando MCP

> Configure servidores MCP para extender su agente con herramientas externas. Cubre tipos de transporte, búsqueda de herramientas para conjuntos grandes de herramientas, autenticación y manejo de errores.

El [Protocolo de Contexto de Modelo (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) es un estándar abierto para conectar agentes de IA a herramientas externas y fuentes de datos. Con MCP, su agente puede consultar bases de datos, integrarse con APIs como Slack y GitHub, y conectarse a otros servicios sin escribir implementaciones de herramientas personalizadas.

Los servidores MCP pueden ejecutarse como procesos locales, conectarse a través de HTTP o ejecutarse directamente dentro de su aplicación SDK.

<Note>
  Esta página cubre la configuración de MCP para el Agent SDK. Para agregar servidores MCP a la CLI de Claude Code de modo que se carguen en cada proyecto, consulte [Alcances de instalación de MCP](/docs/es/mcp#mcp-installation-scopes).
</Note>

<h2 id="quickstart">
  Inicio rápido
</h2>

Este ejemplo se conecta al servidor MCP de [documentación de Claude Code](https://code.claude.com/docs) usando [transporte HTTP](#http%2Fsse-servers) y utiliza [`allowedTools`](#allow-mcp-tools) con un comodín para permitir todas las herramientas del servidor.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the docs MCP server to explain what hooks are in Claude Code",
    options: {
      mcpServers: {
        "claude-code-docs": {
          type: "http",
          url: "https://code.claude.com/docs/mcp"
        }
      },
      allowedTools: ["mcp__claude-code-docs__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "claude-code-docs": {
                  "type": "http",
                  "url": "https://code.claude.com/docs/mcp",
              }
          },
          allowed_tools=["mcp__claude-code-docs__*"],
      )

      async for message in query(
          prompt="Use the docs MCP server to explain what hooks are in Claude Code",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

El agente se conecta al servidor de documentación, busca información sobre hooks y devuelve los resultados.

<h2 id="add-an-mcp-server">
  Agregar un servidor MCP
</h2>

Puede configurar servidores MCP en código al llamar a `query()`, o en un archivo `.mcp.json` cargado a través de [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  En código
</h3>

Pase servidores MCP directamente en la opción `mcpServers`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List files in my project",
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "filesystem": {
                  "command": "npx",
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-filesystem",
                      "/Users/me/projects",
                  ],
              }
          },
          allowed_tools=["mcp__filesystem__*"],
      )

      async for message in query(prompt="List files in my project", options=options):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="from-a-config-file">
  Desde un archivo de configuración
</h3>

Cree un archivo `.mcp.json` en la raíz de su proyecto. El archivo se recoge cuando la fuente de configuración `project` está habilitada, que lo está para las opciones predeterminadas de `query()`. Si establece `settingSources` explícitamente, incluya `"project"` para que este archivo se cargue:

```json theme={null}
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
    }
  }
}
```

<h2 id="allow-mcp-tools">
  Permitir herramientas MCP
</h2>

Las herramientas MCP requieren permiso explícito antes de que Claude pueda usarlas. Sin permiso, Claude verá que las herramientas están disponibles pero no podrá llamarlas.

<h3 id="tool-naming-convention">
  Convención de nomenclatura de herramientas
</h3>

Las herramientas MCP siguen el patrón de nomenclatura `mcp__<server-name>__<tool-name>`. Por ejemplo, un servidor GitHub llamado `"github"` con una herramienta `list_issues` se convierte en `mcp__github__list_issues`.

<h3 id="auto-approve-with-allowedtools">
  Aprobación automática con allowedTools
</h3>

Use `allowedTools` para aprobar automáticamente herramientas MCP específicas para que Claude pueda usarlas sin un aviso de permiso:

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: [
      "mcp__github__*", // All tools from the github server
      "mcp__db__query", // Only the query tool from db server
      "mcp__slack__send_message" // Only send_message from slack server
    ]
  }
};
```

Los comodines (`*`) le permiten permitir todas las herramientas de un servidor sin enumerar cada una individualmente.

<Note>
  **Prefiera `allowedTools` sobre modos de permiso para acceso a MCP.** `permissionMode: "acceptEdits"` no aprueba automáticamente herramientas MCP (solo ediciones de archivos y comandos Bash del sistema de archivos). `permissionMode: "bypassPermissions"` sí aprueba automáticamente herramientas MCP pero también desactiva la mayoría de los otros avisos de seguridad, lo que es más amplio de lo necesario; consulte [Cómo se evalúan los permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) para los avisos que permanecen. Un comodín en `allowedTools` otorga exactamente el servidor MCP que desea y nada más. Consulte [Modos de permiso](/docs/es/agent-sdk/permissions#permission-modes) para una comparación completa.
</Note>

<h3 id="discover-available-tools">
  Descubrir herramientas disponibles
</h3>

Para ver qué herramientas proporciona un servidor MCP, consulte la documentación del servidor o conéctese al servidor e inspeccione el mensaje de inicialización `system`:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available MCP tools:", message.mcp_servers);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, SystemMessage


  async def main():
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available MCP tools:", message.data["mcp_servers"])


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="transport-types">
  Tipos de transporte
</h2>

Los servidores MCP se comunican con su agente utilizando diferentes protocolos de transporte. Consulte la documentación del servidor para ver qué transporte admite:

* Si los documentos le dan un **comando para ejecutar** (como `npx @modelcontextprotocol/server-github`), use stdio
* Si los documentos le dan una **URL**, use HTTP o SSE
* Si está construyendo sus propias herramientas en código, use un servidor MCP SDK

<h3 id="stdio-servers">
  Servidores stdio
</h3>

Procesos locales que se comunican a través de stdin/stdout. Use esto para servidores MCP que ejecuta en la misma máquina:

<Tabs>
  <Tab title="En código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues", "mcp__github__search_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues", "mcp__github__search_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

<h3 id="http/sse-servers">
  Servidores HTTP/SSE
</h3>

Use HTTP o SSE para servidores MCP alojados en la nube y APIs remotas:

<Tabs>
  <Tab title="En código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "remote-api": {
              type: "sse",
              url: "https://api.example.com/mcp/sse",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__remote-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "remote-api": {
                  "type": "sse",
                  "url": "https://api.example.com/mcp/sse",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__remote-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "remote-api": {
          "type": "sse",
          "url": "https://api.example.com/mcp/sse",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Para el transporte HTTP transmisible, use `"type": "http"` en su lugar. En `.mcp.json` y otros archivos de configuración JSON, `"streamable-http"` se acepta como un alias para `"http"`. La opción programática `mcpServers` acepta solo `"http"`.

<h3 id="sdk-mcp-servers">
  Servidores MCP SDK
</h3>

Defina herramientas personalizadas directamente en el código de su aplicación en lugar de ejecutar un proceso de servidor separado. Consulte la [guía de herramientas personalizadas](/docs/es/agent-sdk/custom-tools) para detalles de implementación.

<h2 id="mcp-tool-search">
  Búsqueda de herramientas MCP
</h2>

Cuando tiene muchas herramientas MCP configuradas, las definiciones de herramientas pueden consumir una parte significativa de su ventana de contexto. La búsqueda de herramientas resuelve esto al retener las definiciones de herramientas del contexto y cargar solo las que Claude necesita para cada turno.

La búsqueda de herramientas está habilitada de forma predeterminada. Consulte [Búsqueda de herramientas](/docs/es/agent-sdk/tool-search) para opciones de configuración y detalles.

Para más detalles, incluidas las mejores prácticas y el uso de búsqueda de herramientas con herramientas SDK personalizadas, consulte la [guía de búsqueda de herramientas](/docs/es/agent-sdk/tool-search).

<h2 id="authentication">
  Autenticación
</h2>

La mayoría de los servidores MCP requieren autenticación para acceder a servicios externos. Pase credenciales a través de variables de entorno en la configuración del servidor.

<h3 id="pass-credentials-via-environment-variables">
  Pasar credenciales a través de variables de entorno
</h3>

Use el campo `env` para pasar claves API, tokens y otras credenciales al servidor MCP:

<Tabs>
  <Tab title="En código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            github: {
              command: "npx",
              args: ["-y", "@modelcontextprotocol/server-github"],
              env: {
                GITHUB_TOKEN: process.env.GITHUB_TOKEN
              }
            }
          },
          allowedTools: ["mcp__github__list_issues"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "github": {
          "command": "npx",
          "args": ["-y", "@modelcontextprotocol/server-github"],
          "env": {
            "GITHUB_TOKEN": "${GITHUB_TOKEN}"
          }
        }
      }
    }
    ```

    La sintaxis `${GITHUB_TOKEN}` expande variables de entorno en tiempo de ejecución.
  </Tab>
</Tabs>

Consulte [Listar problemas de un repositorio](#list-issues-from-a-repository) para un ejemplo completo y funcional con registro de depuración.

<h3 id="http-headers-for-remote-servers">
  Encabezados HTTP para servidores remotos
</h3>

Para servidores HTTP y SSE, pase encabezados de autenticación directamente en la configuración del servidor:

<Tabs>
  <Tab title="En código">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "secure-api": {
              type: "http",
              url: "https://api.example.com/mcp",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__secure-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "secure-api": {
                  "type": "http",
                  "url": "https://api.example.com/mcp",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__secure-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "secure-api": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```

    La sintaxis `${API_TOKEN}` expande variables de entorno en tiempo de ejecución.
  </Tab>
</Tabs>

<h3 id="oauth2-authentication">
  Autenticación OAuth2
</h3>

La [especificación MCP admite OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) para autorización. El SDK no abre un navegador ni ejecuta un flujo OAuth interactivo. Cuando un servidor configurado devuelve un desafío de autorización y no hay ningún token almacenado disponible, la ejecución del agente continúa sin las herramientas de ese servidor, y el servidor se reporta con estado `needs-auth` en el array `mcp_servers` del [mensaje de inicialización del sistema](/docs/es/agent-sdk/typescript#sdksystemmessage). Verifique ese array al inicio si su agente depende de que un servidor específico esté conectado.

Para proporcionar credenciales, complete el flujo OAuth en su propia aplicación y pase el token de acceso resultante en los `headers` del servidor:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app
  const accessToken = await getAccessTokenFromOAuthFlow();

  const options = {
    mcpServers: {
      "oauth-api": {
        type: "http",
        url: "https://api.example.com/mcp",
        headers: {
          Authorization: `Bearer ${accessToken}`
        }
      }
    },
    allowedTools: ["mcp__oauth-api__*"]
  };
  ```

  ```python Python theme={null}
  # After completing OAuth flow in your app
  access_token = await get_access_token_from_oauth_flow()

  options = ClaudeAgentOptions(
      mcp_servers={
          "oauth-api": {
              "type": "http",
              "url": "https://api.example.com/mcp",
              "headers": {"Authorization": f"Bearer {access_token}"},
          }
      },
      allowed_tools=["mcp__oauth-api__*"],
  )
  ```
</CodeGroup>

<h2 id="examples">
  Ejemplos
</h2>

<h3 id="list-issues-from-a-repository">
  Listar problemas de un repositorio
</h3>

Este ejemplo se conecta al [servidor GitHub MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/github) para enumerar problemas recientes. El ejemplo incluye registro de depuración para verificar la conexión MCP y las llamadas de herramientas.

Antes de ejecutar, cree un [token de acceso personal de GitHub](https://github.com/settings/tokens) con alcance `repo` y establézcalo como variable de entorno:

```bash theme={null}
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-github"],
          env: {
            GITHUB_TOKEN: process.env.GITHUB_TOKEN
          }
        }
      },
      allowedTools: ["mcp__github__list_issues"]
    }
  })) {
    // Verify MCP server connected successfully
    if (message.type === "system" && message.subtype === "init") {
      console.log("MCP servers:", message.mcp_servers);
    }

    // Log when Claude calls an MCP tool
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "tool_use" && block.name.startsWith("mcp__")) {
          console.log("MCP tool called:", block.name);
        }
      }
    }

    // Print the final result
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      SystemMessage,
      AssistantMessage,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "command": "npx",
                  "args": ["-y", "@modelcontextprotocol/server-github"],
                  "env": {"GITHUB_TOKEN": os.environ["GITHUB_TOKEN"]},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )

      async for message in query(
          prompt="List the 3 most recent issues in anthropics/claude-code",
          options=options,
      ):
          # Verify MCP server connected successfully
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("MCP servers:", message.data.get("mcp_servers"))

          # Log when Claude calls an MCP tool
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "name") and block.name.startswith("mcp__"):
                      print("MCP tool called:", block.name)

          # Print the final result
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="query-a-database">
  Consultar una base de datos
</h3>

Este ejemplo utiliza el [servidor Postgres MCP](https://github.com/modelcontextprotocol/servers/tree/main/src/postgres) para consultar una base de datos. La cadena de conexión se pasa como argumento al servidor. El agente descubre automáticamente el esquema de la base de datos, escribe la consulta SQL y devuelve los resultados:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Connection string from environment variable
  const connectionString = process.env.DATABASE_URL;

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // Pass connection string as argument to the server
          args: ["-y", "@modelcontextprotocol/server-postgres", connectionString]
        }
      },
      // Allow only read queries, not writes
      allowedTools: ["mcp__postgres__query"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      # Connection string from environment variable
      connection_string = os.environ["DATABASE_URL"]

      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # Pass connection string as argument to the server
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-postgres",
                      connection_string,
                  ],
              }
          },
          # Allow only read queries, not writes
          allowed_tools=["mcp__postgres__query"],
      )

      # Natural language query - Claude writes the SQL
      async for message in query(
          prompt="How many users signed up last week? Break it down by day.",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Manejo de errores
</h2>

Los servidores MCP pueden fallar al conectarse por varias razones: el proceso del servidor podría no estar instalado, las credenciales podrían ser inválidas, o un servidor remoto podría ser inaccesible.

El SDK emite un mensaje `system` con subtipo `init` al inicio de cada consulta. Este mensaje incluye el estado de conexión para cada servidor MCP. Verifique el campo `status` para detectar fallos de conexión antes de que el agente comience a trabajar:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Process data",
    options: {
      mcpServers: {
        "data-processor": dataServer
      }
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      const failedServers = message.mcp_servers.filter((s) => s.status !== "connected");

      if (failedServers.length > 0) {
        console.warn("Failed to connect:", failedServers);
      }
    }

    if (message.type === "result" && message.subtype === "error_during_execution") {
      console.error("Execution failed");
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      async for message in query(prompt="Process data", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              failed_servers = [
                  s
                  for s in message.data.get("mcp_servers", [])
                  if s.get("status") != "connected"
              ]

              if failed_servers:
                  print(f"Failed to connect: {failed_servers}")

          if (
              isinstance(message, ResultMessage)
              and message.subtype == "error_during_execution"
          ):
              print("Execution failed")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="server-shows-failed-status">
  El servidor muestra estado "failed"
</h3>

Verifique el mensaje `init` para ver qué servidores no se conectaron:

```typescript theme={null}
if (message.type === "system" && message.subtype === "init") {
  for (const server of message.mcp_servers) {
    if (server.status === "failed") {
      console.error(`Server ${server.name} failed to connect`);
    }
  }
}
```

Causas comunes:

* **Variables de entorno faltantes**: Asegúrese de que los tokens y credenciales requeridos estén configurados. Para servidores stdio, verifique que el campo `env` coincida con lo que espera el servidor.
* **Servidor no instalado**: Para comandos `npx`, verifique que el paquete exista y que Node.js esté en su PATH.
* **Cadena de conexión inválida**: Para servidores de base de datos, verifique el formato de la cadena de conexión y que la base de datos sea accesible.
* **Problemas de red**: Para servidores HTTP/SSE remotos, verifique que la URL sea accesible y que los firewalls permitan la conexión.

<h3 id="tools-not-being-called">
  Las herramientas no se están llamando
</h3>

Si Claude ve herramientas pero no las usa, verifique que haya otorgado permiso con `allowedTools`:

```typescript hidelines={1,-1} theme={null}
const _ = {
  options: {
    mcpServers: {
      // your servers
    },
    allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
  }
};
```

<h3 id="connection-timeouts">
  Tiempos de espera de conexión
</h3>

Las conexiones del servidor MCP se agotarán después de 30 segundos por defecto. Si su servidor tarda más en iniciarse, la conexión fallará. Aumente el límite con la variable de entorno [`MCP_TIMEOUT`](/docs/es/env-vars), en milisegundos. Para servidores que necesitan más tiempo de inicio, considere también:

* Usar un servidor más ligero si está disponible
* Precalentar el servidor antes de iniciar su agente
* Verificar los registros del servidor para causas de inicialización lenta

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  La salida de la herramienta excede el máximo de tokens permitidos
</h3>

El SDK aplica el mismo límite de salida MCP que Claude Code. Cuando el resultado de una herramienta es mayor que 25,000 tokens, la salida completa se guarda en un archivo y el resultado de la herramienta se reemplaza con un mensaje de error que nombra la ruta del archivo, para que el agente pueda leer la salida en porciones. Aumente el límite con la variable de entorno [`MAX_MCP_OUTPUT_TOKENS`](/docs/es/env-vars). Consulte [Límites de salida MCP y advertencias](/docs/es/mcp#mcp-output-limits-and-warnings) para el comportamiento completo, incluida la forma en que un servidor puede declarar un límite más alto por herramienta.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* **[Guía de herramientas personalizadas](/docs/es/agent-sdk/custom-tools)**: Construya su propio servidor MCP que se ejecute en proceso con su aplicación SDK
* **[Permisos](/docs/es/agent-sdk/permissions)**: Controle qué herramientas MCP puede usar su agente con `allowedTools` y `disallowedTools`
* **[Límites de salida de MCP y advertencias](/docs/es/mcp#mcp-output-limits-and-warnings)**: Cómo el SDK maneja los resultados de herramientas que exceden `MAX_MCP_OUTPUT_TOKENS`, incluyendo la alternativa de persistencia en disco y la anotación `anthropic/maxResultSizeChars` por herramienta
* **[Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)**: Referencia completa de la API incluyendo opciones de configuración de MCP
* **[Referencia del SDK de Python](/docs/es/agent-sdk/python)**: Referencia completa de la API incluyendo opciones de configuración de MCP
* **[Directorio de servidores MCP](https://github.com/modelcontextprotocol/servers)**: Explore servidores MCP disponibles para bases de datos, APIs y más
