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

Puede configurar servidores MCP en código al llamar a `query()`, o en un archivo `.mcp.json` cargado mediante [`settingSources`](#from-a-config-file).

<h3 id="in-code">
  En código
</h3>

Pase servidores MCP directamente en la opción `mcpServers`. Este ejemplo inicia un servidor MCP del sistema de archivos local para `/Users/me/projects`. Reemplace esa ruta con un directorio en su máquina:

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

Cree un archivo `.mcp.json` en la raíz de su proyecto. El archivo se carga cuando la fuente de configuración `project` está habilitada, que lo está para las opciones predeterminadas de `query()`. Si establece `settingSources` explícitamente, incluya `"project"` para que este archivo se cargue. Reemplace `/Users/me/projects` con un directorio en su máquina:

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

<h2 id="connection-timing">
  Temporización de la conexión
</h2>

Claude Code registra los servidores que usted pasa en `options.mcpServers` al inicio y emite el [mensaje init](#error-handling) una vez que se resuelve la espera de primer turno, si la hay. Si cada servidor de `options.mcpServers` retrasa el primer turno, y cuándo se conecta, depende de su tipo:

| Tipo de servidor                                                                                             | ¿Retrasa el primer turno?                                             | Tiempo de espera del primer turno                                                                                  |
| :----------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| Servidor stdio, o servidor HTTP/SSE sin una lista de herramientas en caché                                   | Sí, hasta que se conecte                                              | [`MCP_TIMEOUT`](/docs/es/env-vars), 30 segundos por defecto; la conexión falla en ese plazo                             |
| Servidor remoto con una lista de herramientas en caché, guardada por Claude Code desde una conexión anterior | No; las herramientas en caché están disponibles desde el primer turno | Ninguno; se conecta en su primera llamada de herramienta, y esa conexión diferida tiene su propio tiempo de espera |
| Servidor [SDK](#sdk-mcp-servers) en proceso                                                                  | Sí, hasta que se conecte y enumere sus herramientas                   | Ninguno; las solicitudes de conexión y enumeración de herramientas tienen cada una su propio tiempo de espera      |

Los servidores cargados desde [archivos de configuración](#from-a-config-file) como `.mcp.json` o desde plugins comúnmente muestran `pending` en el mensaje init. Cuando `options.mcpServers` contiene un servidor stdio, HTTP o SSE, el primer turno espera a que se conecten estos servidores pendientes también, hasta `MCP_TIMEOUT`. Cuando `options.mcpServers` está vacío o contiene solo servidores SDK, el primer turno espera hasta 2 segundos en su lugar:

* **Con [búsqueda de herramientas](/docs/es/agent-sdk/tool-search), el valor predeterminado**: la espera cubre servidores aún pendientes configurados con [`alwaysLoad: true`](/docs/es/mcp#exempt-a-server-from-deferral) y no el resto. El resto sigue conectándose en segundo plano. [Disponibilidad de herramientas](/docs/es/mcp#tool-availability) describe cómo Claude accede a sus herramientas una vez que se conectan.
* **Sin búsqueda de herramientas**: la espera cubre cada servidor pendiente. [Configurar búsqueda de herramientas](/docs/es/agent-sdk/tool-search#configure-tool-search) cubre qué desactiva la búsqueda de herramientas. Si usted excluye la herramienta `ToolSearch` de la sesión, por ejemplo a través de `disallowedTools`, la sesión también se ejecuta sin búsqueda de herramientas.

Si usted establece `permissionPromptToolName`, el primer turno también espera al servidor de esa herramienta en todos los casos, hasta `MCP_TIMEOUT`.

Para establecer la espera del primer turno usted mismo, agregue `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` a la [opción `env`](/docs/es/agent-sdk/configuration#set-environment-variables), por ejemplo `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`. El primer turno entonces espera hasta esa cantidad de milisegundos para cada servidor pendiente, independientemente de si la búsqueda de herramientas está disponible. Este plazo también reemplaza la espera del primer turno `MCP_TIMEOUT` para servidores stdio, HTTP y SSE en `options.mcpServers`. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` requiere Claude Code v2.1.274 o posterior.

Los servidores aún pendientes cuando termina la espera siguen conectándose en segundo plano. Establezca la variable en `0` para omitir la espera. Un servidor `permissionPromptToolName` mantiene su propia espera `MCP_TIMEOUT` independientemente del valor.

Para bloquear el inicio mismo en una fase separada y anterior a la espera del primer turno, antes de que se envíe el mensaje init:

* Establezca [`MCP_CONNECTION_NONBLOCKING`](/docs/es/env-vars) en `0` para bloquear todo el lote de conexiones. Claude Code limita esa espera a 5 segundos por defecto. Ajuste el límite con la variable de entorno [`MCP_CONNECT_TIMEOUT_MS`](/docs/es/env-vars), en milisegundos. Los servidores aún pendientes en ese plazo siguen conectándose en segundo plano.
* Establezca `alwaysLoad: true` en la configuración de un servidor para que sus herramientas estén disponibles en sus esquemas completos en el primer turno, [exentas de aplazamiento de búsqueda de herramientas](/docs/es/mcp#exempt-a-server-from-deferral). Claude Code espera al inicio a que se conecten las herramientas de ese servidor, limitado al mismo plazo, mientras que otros servidores siguen conectándose en segundo plano; un servidor remoto con una lista de herramientas en caché las proporciona sin conectarse, según la tabla anterior.

El mensaje `system` con subtipo `init` reporta el estado de cada servidor en el momento en que se emite; consulte [Manejo de errores](#error-handling) para leer esos estados.

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

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
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

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=[
          "mcp__github__*",  # All tools from the github server
          "mcp__db__query",  # Only the query tool from db server
          "mcp__slack__send_message",  # Only send_message from slack server
      ],
  )
  ```
</CodeGroup>

Los caracteres comodín (`*`) le permiten permitir todas las herramientas de un servidor sin enumerar cada una individualmente.

<Note>
  **Prefiera `allowedTools` sobre modos de permiso para acceso MCP.** `permissionMode: "acceptEdits"` no aprueba automáticamente herramientas MCP (solo ediciones de archivos y comandos Bash del sistema de archivos). `permissionMode: "bypassPermissions"` sí aprueba automáticamente herramientas MCP pero también desactiva la mayoría de otros avisos de seguridad, lo que es más amplio de lo necesario; consulte [Cómo se evalúan los permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) para los avisos que permanecen. Un carácter comodín en `allowedTools` otorga exactamente el servidor MCP que desea y nada más. Consulte [Modos de permiso](/docs/es/agent-sdk/permissions#permission-modes) para una comparación completa.
</Note>

<h3 id="discover-available-tools">
  Descubrir herramientas disponibles
</h3>

Para ver qué herramientas proporciona un servidor MCP, consulte la documentación del servidor o inspeccione el array `tools` en el mensaje init `system`. Los nombres de herramientas MCP comienzan con `mcp__`.

Claude Code emite el mensaje init después de la [espera de conexión de primer turno](#connection-timing) para servidores pasados en `options.mcpServers`, por lo que el array `tools` enumera las herramientas `mcp__` de cada servidor que se ha conectado para entonces, más las de servidores con una [lista de herramientas en caché](#connection-timing), que se conectan en el primer uso. Las herramientas de cualquier otro servidor que no se haya conectado están ausentes; consulte [Manejo de errores](#error-handling) para leer el estado de cada servidor.

Este filtro imprime los nombres de herramientas MCP:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    mcpServers: {
      // your servers
    },
  };

  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      const mcpTools = message.tools.filter((name) => name.startsWith("mcp__"));
      console.log("Available MCP tools:", mcpTools);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              # your servers
          },
      )
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              mcp_tools = [t for t in message.data.get("tools", []) if t.startswith("mcp__")]
              print("Available MCP tools:", mcp_tools)


  asyncio.run(main())
  ```
</CodeGroup>

También puede pedirle a Claude que enumere las herramientas disponibles de un servidor.

<h2 id="transport-types">
  Tipos de transporte
</h2>

Los servidores MCP se comunican con su agente utilizando diferentes protocolos de transporte. Consulte la documentación del servidor para ver qué transporte admite:

* Si la documentación le proporciona un **comando para ejecutar** (como `npx @modelcontextprotocol/server-filesystem`), use stdio
* Si la documentación le proporciona una **URL**, use HTTP o SSE
* Si está creando sus propias herramientas en código, use un servidor MCP SDK

<h3 id="stdio-servers">
  Servidores stdio
</h3>

Procesos locales que se comunican a través de stdin/stdout. Utilice esto para servidores MCP que ejecuta en la misma máquina. Para el formulario `.mcp.json`, use los mismos campos que se muestran en [Desde un archivo de configuración](#from-a-config-file). En código, pase el comando y sus argumentos. Reemplace `/Users/me/projects` con un directorio en su máquina:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__read_file", "mcp__filesystem__list_directory"]
    }
  };
  ```

  ```python Python theme={null}
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
      allowed_tools=["mcp__filesystem__read_file", "mcp__filesystem__list_directory"],
  )
  ```
</CodeGroup>

<h3 id="http/sse-servers">
  Servidores HTTP/SSE
</h3>

Use HTTP o SSE para servidores MCP alojados en la nube y API remotas. Para el formulario `.mcp.json`, use los mismos campos que en el ejemplo en [Encabezados HTTP para servidores remotos](#http-headers-for-remote-servers), con `"type": "sse"` para un servidor SSE. En código, pase la URL del servidor:

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

Para el transporte HTTP transmisible, use `"type": "http"` en su lugar. En archivos de configuración `.mcp.json` y otros JSON, `"streamable-http"` se acepta como un alias para `"http"`. El tipo `McpHttpServerConfig` de los SDK declara solo `"http"`, así que use `"http"` para servidores que pase en código.

<h3 id="sdk-mcp-servers">
  Servidores MCP SDK
</h3>

Defina herramientas personalizadas directamente en el código de su aplicación en lugar de ejecutar un proceso de servidor separado. Consulte la [guía de herramientas personalizadas](/docs/es/agent-sdk/custom-tools) para obtener detalles de implementación.

Un servidor MCP SDK registrado por una [solicitud de control `initialize`](/docs/es/agent-sdk/typescript#sdkcontrolinitializeresponse) comienza a conectarse tan pronto como Claude Code procesa la solicitud.

<h2 id="mcp-tool-search">
  Búsqueda de herramientas MCP
</h2>

Cuando tiene muchas herramientas MCP configuradas, las definiciones de herramientas pueden consumir una porción significativa de su ventana de contexto. La búsqueda de herramientas resuelve esto al retener las definiciones de herramientas del contexto y cargar solo las que Claude necesita para cada turno.

La búsqueda de herramientas está habilitada de forma predeterminada. Consulte [Búsqueda de herramientas](/docs/es/agent-sdk/tool-search) para opciones de configuración, mejores prácticas y uso de búsqueda de herramientas con herramientas SDK personalizadas.

<h2 id="authentication">
  Autenticación
</h2>

La mayoría de los servidores MCP requieren autenticación para acceder a servicios externos. Pase las credenciales a través de variables de entorno en la configuración del servidor.

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
            "api-server": {
              command: "npx",
              args: ["-y", "@your-org/api-mcp-server"],
              env: {
                API_KEY: process.env.API_KEY
              }
            }
          },
          allowedTools: ["mcp__api-server__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "api-server": {
                  "command": "npx",
                  "args": ["-y", "@your-org/api-mcp-server"],
                  "env": {"API_KEY": os.environ["API_KEY"]},
              }
          },
          allowed_tools=["mcp__api-server__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "api-server": {
          "command": "npx",
          "args": ["-y", "@your-org/api-mcp-server"],
          "env": {
            "API_KEY": "${API_KEY}"
          }
        }
      }
    }
    ```

    La sintaxis `${API_KEY}` expande variables de entorno en tiempo de ejecución.
  </Tab>
</Tabs>

<h3 id="http-headers-for-remote-servers">
  Encabezados HTTP para servidores remotos
</h3>

Para servidores HTTP y SSE, pase los encabezados de autenticación directamente en la configuración del servidor:

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

Para un ejemplo completo y funcional de un servidor remoto autenticado con encabezados, consulte [Listar problemas de un repositorio](#list-issues-from-a-repository).

<h3 id="oauth2-authentication">
  Autenticación OAuth2
</h3>

La [especificación MCP admite OAuth 2.1](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) para autorización. El SDK no abre un navegador ni ejecuta un flujo OAuth interactivo. Cuando un servidor configurado devuelve un desafío de autorización y no hay ningún token almacenado disponible, la ejecución del agente continúa sin las herramientas de ese servidor, y el servidor reporta el estado `needs-auth`. La matriz `mcp_servers` del [mensaje de inicialización del sistema](/docs/es/agent-sdk/typescript#sdksystemmessage) aún puede mostrar `pending` para ese servidor cuando se emite. Para confirmar si un servidor necesita credenciales, consulte `mcpServerStatus()` en el SDK de TypeScript o [`get_mcp_status()`](/docs/es/agent-sdk/python#methods) en Python.

Para proporcionar credenciales, complete el flujo OAuth en su propia aplicación y pase el token de acceso resultante en los `headers` del servidor:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Después de completar el flujo OAuth en su aplicación.
  // Implemente getAccessTokenFromOAuthFlow para su proveedor OAuth.
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
  # Después de completar el flujo OAuth en su aplicación.
  # Implemente get_access_token_from_oauth_flow para su proveedor OAuth.
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

Este ejemplo se conecta al [servidor MCP de GitHub](https://github.com/github/github-mcp-server) remoto para listar problemas recientes. El ejemplo incluye registro de depuración para verificar la conexión MCP y las llamadas a herramientas.

Antes de ejecutar, cree un [token de acceso personal de GitHub](https://github.com/settings/personal-access-tokens) con acceso de lectura a los repositorios que desea consultar y establézcalo como variable de entorno:

```bash theme={null}
export GITHUB_TOKEN=YOUR_GITHUB_PAT
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          type: "http",
          url: "https://api.githubcopilot.com/mcp/",
          headers: {
            Authorization: `Bearer ${process.env.GITHUB_TOKEN}`
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
                  "type": "http",
                  "url": "https://api.githubcopilot.com/mcp/",
                  "headers": {"Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}"},
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

En la línea `MCP servers:`, un `status` de `connected` para `github` confirma que el token funciona. Si Claude Code tiene una [lista de herramientas en caché](#connection-timing) para el servidor, el estado puede mostrar `pending` en su lugar y el servidor se conecta en su primera llamada a herramienta. Si el estado es `failed` o `needs-auth`, consulte [Manejo de errores](#error-handling) antes de confiar en el resultado, ya que Claude puede recurrir a herramientas integradas cuando el servidor no está disponible.

<h3 id="query-a-database">
  Consultar una base de datos
</h3>

Este ejemplo utiliza [DBHub](https://github.com/bytebase/dbhub) para consultar una base de datos Postgres. El agente descubre automáticamente el esquema de la base de datos, escribe la consulta SQL y devuelve los resultados.

La herramienta `execute_sql` de DBHub ejecuta cualquier SQL que emita el agente, incluidas escrituras, a menos que lo restrinja. Establecer `readonly = true` en el [archivo de configuración de DBHub](https://dbhub.ai/config/toml) hace que DBHub rechace las declaraciones `INSERT`, `UPDATE`, `DELETE` y DDL, por lo que el ejemplo no puede modificar sus datos incluso si el agente emite una escritura. DBHub resuelve `${DATABASE_URL}` desde el entorno del proceso cuando carga la configuración, por lo que la cadena de conexión se mantiene fuera del archivo. Cree este `dbhub.toml` junto a su script:

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

El script luego apunta DBHub al archivo de configuración en lugar de pasar una cadena de conexión directamente. Antes de ejecutar, establezca la variable de entorno `DATABASE_URL` en su cadena de conexión. Reemplace los valores de marcador de posición con los detalles de su propia base de datos:

```bash theme={null}
export DATABASE_URL=postgresql://user:password@localhost:5432/mydb
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // dbhub.toml sets readonly = true, so execute_sql rejects writes
          args: ["-y", "@bytebase/dbhub", "--config", "dbhub.toml"]
        }
      },
      allowedTools: ["mcp__postgres__execute_sql"]
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
              "postgres": {
                  "command": "npx",
                  # dbhub.toml sets readonly = true, so execute_sql rejects writes
                  "args": [
                      "-y",
                      "@bytebase/dbhub",
                      "--config",
                      "dbhub.toml",
                  ],
              }
          },
          allowed_tools=["mcp__postgres__execute_sql"],
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

Claude Code emite un mensaje `system` con subtipo `init` al inicio de cada consulta. Este mensaje incluye el estado de conexión para cada servidor MCP. El campo `status` puede ser `"pending"`, `"connected"`, `"failed"`, `"needs-auth"` o `"disabled"`. Claude Code emite el mensaje init después del [tiempo de espera de conexión de primer turno](#connection-timing) para servidores pasados en `options.mcpServers`, por lo que un servidor de este tipo que se conectó dentro del tiempo de espera muestra `"connected"`.

En el mensaje init, no trate `"pending"` como un fallo por sí solo. Puede significar cualquiera de estos:

* El servidor aún no se ha conectado. Vea [cuánto tiempo espera Claude Code antes del primer turno](#connection-timing)
* La lista de herramientas del servidor fue [servida desde la caché](#connection-timing), con una conexión realizada en el primer uso
* El plazo de conexión expiró. Un servidor de este tipo reporta `"pending"` o `"failed"` dependiendo del tiempo

Verifique `"failed"` o `"needs-auth"` para detectar servidores que no serán utilizables:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Process data",
      options: {
        mcpServers: {
          // Replace dataServer with your server configuration
          "data-processor": dataServer
        }
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        const unavailableServers = message.mcp_servers.filter(
          (s) => s.status === "failed" || s.status === "needs-auth"
        );

        if (unavailableServers.length > 0) {
          console.warn("Unavailable MCP servers:", unavailableServers);
        }
      }

      if (message.type === "result" && message.subtype === "error_during_execution") {
        console.error("Execution failed");
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branch above has
    // already run; a failure to start or reach the Claude Code process
    // yields no result message. MCP servers that fail to connect don't
    // throw: use the status check above, and note that servers still
    // "pending" at init need a later status check.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      # Replace data_server with your server configuration
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      try:
          async for message in query(prompt="Process data", options=options):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  unavailable_servers = [
                      s
                      for s in message.data.get("mcp_servers", [])
                      if s.get("status") in ("failed", "needs-auth")
                  ]

                  if unavailable_servers:
                      print(f"Unavailable MCP servers: {unavailable_servers}")

              if (
                  isinstance(message, ResultMessage)
                  and message.subtype == "error_during_execution"
              ):
                  print("Execution failed")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branch above has
          # already run; a failure to start or reach the Claude Code process
          # yields no result message. MCP servers that fail to connect don't
          # raise: use the status check above, and note that servers still
          # "pending" at init need a later status check.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

El estado de un servidor remoto también puede cambiar después de que reporta `"connected"`. Cuando la conexión a él se cae a mitad de sesión, Claude Code mueve el servidor de vuelta a `"pending"` mientras [se reconecta](/docs/es/mcp#automatic-reconnection). Una llamada posterior a `mcpServerStatus()` en TypeScript, o [`ClaudeSDKClient.get_mcp_status()`](/docs/es/agent-sdk/python#methods) en Python, puede entonces reportar `"pending"` para un servidor que vio conectado anteriormente, sin cambio de configuración de su parte.

Después de que cinco intentos de reconexión fallen, el servidor reporta `"failed"`, o `"needs-auth"` cuando necesita ser autorizado nuevamente. Para reintentar manualmente, llame a [`reconnectMcpServer()`](/docs/es/agent-sdk/typescript#methods) en TypeScript o [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/es/agent-sdk/python#methods) en Python.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="server-shows-failed-status">
  El servidor muestra estado "failed"
</h3>

Verifique el mensaje `init` para ver qué servidores no pudieron conectarse:

<CodeGroup>
  ```typescript TypeScript theme={null}
  if (message.type === "system" && message.subtype === "init") {
    for (const server of message.mcp_servers) {
      if (server.status === "failed") {
        console.error(`Server ${server.name} failed to connect`);
      }
    }
  }
  ```

  ```python Python theme={null}
  if isinstance(message, SystemMessage) and message.subtype == "init":
      for server in message.data.get("mcp_servers", []):
          if server.get("status") == "failed":
              print(f"Server {server['name']} failed to connect")
  ```
</CodeGroup>

Un estado `"pending"` no significa que el servidor haya fallado. Consulte [Manejo de errores](#error-handling) para ver los casos que cubre en la inicialización. Para obtener estados actualizados más adelante en la sesión, llame al método `mcpServerStatus()` de la consulta en el SDK de TypeScript, o [`ClaudeSDKClient.get_mcp_status()`](/docs/es/agent-sdk/python#methods) en Python.

Causas comunes:

* **Variables de entorno faltantes**: Asegúrese de que los tokens y credenciales requeridos estén configurados. Para servidores stdio, verifique que el campo `env` coincida con lo que el servidor espera.
* **Servidor no instalado**: Para comandos `npx`, verifique que el paquete exista y que Node.js esté en su PATH.
* **Cadena de conexión inválida**: Para servidores de base de datos, verifique el formato de la cadena de conexión y que la base de datos sea accesible.
* **Problemas de red**: Para servidores HTTP/SSE remotos, verifique que la URL sea accesible y que los firewalls permitan la conexión.

<h3 id="tools-not-being-called">
  Las herramientas no se están llamando
</h3>

Si Claude ve herramientas pero no las utiliza, verifique que haya otorgado permiso con `allowedTools`:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=["mcp__servername__*"],  # Auto-approve calls from this server
  )
  ```
</CodeGroup>

<h3 id="connection-timeouts">
  Tiempos de espera de conexión
</h3>

Las conexiones del servidor MCP se agotan después de 30 segundos de forma predeterminada. Para cambiar cuánto tiempo puede tomar una llamada de herramienta en ejecución, establezca [`MCP_TOOL_TIMEOUT`](/docs/es/env-vars). Si su servidor tarda más en iniciarse, la conexión falla. Aumente el límite de conexión con la variable de entorno [`MCP_TIMEOUT`](/docs/es/env-vars), en milisegundos. Para servidores que necesitan más tiempo de inicio, también considere:

* Usar un servidor más ligero si está disponible
* Precalentar el servidor antes de iniciar su agente
* Verificar los registros del servidor para detectar causas de inicialización lenta

En TypeScript, puede establecer el límite de llamadas de herramientas para un único [servidor MCP del SDK](#sdk-mcp-servers) pasando [`timeout` a `createSdkMcpServer()`](/docs/es/agent-sdk/typescript#createsdkmcpserver).

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  La salida de la herramienta excede el máximo de tokens permitidos
</h3>

El SDK aplica el mismo límite de salida MCP que Claude Code. Cuando el resultado de una herramienta sin contenido de imagen es mayor que 25.000 tokens, Claude Code guarda la salida en un archivo y reemplaza el resultado de la herramienta con un mensaje de error que nombra la ruta del archivo, para que el agente pueda leer la salida en porciones.

Aumente el límite con la variable de entorno [`MAX_MCP_OUTPUT_TOKENS`](/docs/es/env-vars). Consulte [Límites de salida MCP y advertencias](/docs/es/mcp#mcp-output-limits-and-warnings) para el comportamiento completo, incluida la forma en que un servidor puede declarar un límite más alto por herramienta con la anotación `anthropic/maxResultSizeChars`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* **[Guía de herramientas personalizadas](/docs/es/agent-sdk/custom-tools)**: Cree su propio servidor MCP que se ejecute en proceso con su aplicación SDK
* **[Permisos](/docs/es/agent-sdk/permissions)**: Controle qué herramientas MCP puede usar su agente con `allowedTools` y `disallowedTools`
* **[Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)**: Referencia completa de la API incluyendo opciones de configuración de MCP
* **[Referencia del SDK de Python](/docs/es/agent-sdk/python)**: Referencia completa de la API incluyendo opciones de configuración de MCP
* **[Directorio de servidores MCP](https://github.com/modelcontextprotocol/servers)**: Explore los servidores MCP disponibles para bases de datos, API y más
