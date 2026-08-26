> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia del SDK de Agent - TypeScript

> Referencia completa de la API del SDK de Agent de TypeScript, incluyendo todas las funciones, tipos e interfaces.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  Instalación
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  El SDK incluye un binario nativo de Claude Code para su plataforma como una dependencia opcional como `@anthropic-ai/claude-agent-sdk-darwin-arm64`. No necesita instalar Claude Code por separado. Si su gestor de paquetes omite las dependencias opcionales, el SDK lanza `Native CLI binary for <platform> not found`; en su lugar, establezca [`pathToClaudeCodeExecutable`](#options) en un binario `claude` instalado por separado.
</Note>

<h3 id="compile-to-a-single-executable">
  Compilar a un ejecutable único
</h3>

Cuando compila su aplicación en un ejecutable de un solo archivo con `bun build --compile`, el SDK no puede resolver el binario CLI incluido en tiempo de ejecución. `require.resolve` no funciona dentro del sistema de archivos virtual `$bunfs` del ejecutable compilado, por lo que el SDK lanza `Native CLI binary for <platform> not found`.

Para solucionar esto, incruste el binario de plataforma como un activo de archivo, extráigalo a una ruta real al inicio con `extractFromBunfs()`, y pase esa ruta a [`pathToClaudeCodeExecutable`](#options).

El asistente `extractFromBunfs()` requiere `@anthropic-ai/claude-agent-sdk` v0.3.144 o posterior. El ejemplo a continuación se compila para macOS en Apple Silicon:

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` copia el binario incrustado fuera del sistema de archivos virtual del ejecutable compilado a un directorio temporal por usuario y devuelve la ruta real. Fuera de un ejecutable compilado, devuelve la ruta de entrada sin cambios, por lo que el mismo código se ejecuta en desarrollo sin modificación.

Cada ejecutable compilado incrusta el binario de una única plataforma. Haga coincidir el paquete de plataforma en la importación con su `--target`:

* Para compilación cruzada, instale el paquete de plataforma que no coincida, por ejemplo `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`.
* En Windows, la subruta del binario es `claude.exe`, por ejemplo `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`.

<h2 id="functions">
  Funciones
</h2>

<h3 id="query">
  `query()`
</h3>

La función principal para interactuar con Claude Code. Crea un generador asincrónico que transmite mensajes a medida que llegan.

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  Parámetros
</h4>

| Parámetro | Tipo                                                             | Descripción                                                                              |
| :-------- | :--------------------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | El mensaje de entrada como una cadena o iterable asincrónico para el modo de transmisión |
| `options` | [`Options`](#options)                                            | Objeto de configuración opcional (vea el tipo Options a continuación)                    |

<h4 id="returns">
  Devuelve
</h4>

Devuelve un objeto [`Query`](#query-object) que extiende `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` con métodos adicionales.

<h3 id="startup">
  `startup()`
</h3>

Precalienta el subproceso CLI iniciándolo y completando el protocolo de inicialización antes de que un mensaje esté disponible. El identificador [`WarmQuery`](#warmquery) devuelto acepta un mensaje más tarde y lo escribe en un proceso ya listo, por lo que la primera llamada a `query()` se resuelve sin pagar el costo de generación e inicialización del subproceso en línea.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  Parámetros
</h4>

| Parámetro             | Tipo                  | Descripción                                                                                                                                                                                               |
| :-------------------- | :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Objeto de configuración opcional. Igual que el parámetro `options` para `query()`                                                                                                                         |
| `initializeTimeoutMs` | `number`              | Tiempo máximo en milisegundos para esperar la inicialización del subproceso. Por defecto es `60000`. Si la inicialización no se completa a tiempo, la promesa se rechaza con un error de tiempo de espera |

<h4 id="returns-2">
  Devuelve
</h4>

Devuelve una `Promise<`[`WarmQuery`](#warmquery)`>` que se resuelve una vez que el subproceso se ha generado y ha completado su protocolo de inicialización.

<h4 id="example">
  Ejemplo
</h4>

Llame a `startup()` temprano, por ejemplo al inicio de la aplicación, luego llame a `.query()` en el identificador devuelto una vez que un mensaje esté listo. Esto mueve la generación del subproceso e inicialización fuera de la ruta crítica.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Pague el costo de inicio por adelantado
const warm = await startup({ options: { maxTurns: 3 } });

// Más tarde, cuando un mensaje esté listo, esto es inmediato
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Crea una definición de herramienta MCP segura de tipos para usar con servidores MCP del SDK.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Parámetros
</h4>

| Parámetro     | Tipo                                                              | Descripción                                                                                             |
| :------------ | :---------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| `name`        | `string`                                                          | El nombre de la herramienta                                                                             |
| `description` | `string`                                                          | Una descripción de lo que hace la herramienta                                                           |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | Esquema Zod que define los parámetros de entrada de la herramienta (soporta tanto Zod 3 como Zod 4)     |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Función asincrónica que ejecuta la lógica de la herramienta                                             |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | Anotaciones opcionales de herramienta MCP que proporcionan sugerencias de comportamiento a los clientes |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Re-exportado desde `@modelcontextprotocol/sdk/types.js`. Todos los campos son sugerencias opcionales; los clientes no deben confiar en ellos para decisiones de seguridad.

| Campo             | Tipo      | Predeterminado | Descripción                                                                                                                                                                                  |
| :---------------- | :-------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined`    | Título legible por humanos para la herramienta                                                                                                                                               |
| `readOnlyHint`    | `boolean` | `false`        | Si es `true`, la herramienta no modifica su entorno                                                                                                                                          |
| `destructiveHint` | `boolean` | `true`         | Si es `true`, la herramienta puede realizar actualizaciones destructivas (solo significativo cuando `readOnlyHint` es `false`)                                                               |
| `idempotentHint`  | `boolean` | `false`        | Si es `true`, las llamadas repetidas con los mismos argumentos no tienen efecto adicional (solo significativo cuando `readOnlyHint` es `false`)                                              |
| `openWorldHint`   | `boolean` | `true`         | Si es `true`, la herramienta interactúa con entidades externas (por ejemplo, búsqueda web). Si es `false`, el dominio de la herramienta es cerrado (por ejemplo, una herramienta de memoria) |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

Crea una instancia de servidor MCP que se ejecuta en el mismo proceso que su aplicación.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parámetros
</h4>

| Parámetro         | Tipo                          | Descripción                                                          |
| :---------------- | :---------------------------- | :------------------------------------------------------------------- |
| `options.name`    | `string`                      | El nombre del servidor MCP                                           |
| `options.version` | `string`                      | Cadena de versión opcional                                           |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | Matriz de definiciones de herramientas creadas con [`tool()`](#tool) |

<h3 id="listsessions">
  `listSessions()`
</h3>

Descubre y enumera sesiones pasadas con metadatos ligeros. Filtre por directorio de proyecto o enumere sesiones en todos los proyectos.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  Parámetros
</h4>

| Parámetro                  | Tipo      | Predeterminado | Descripción                                                                                     |
| :------------------------- | :-------- | :------------- | :---------------------------------------------------------------------------------------------- |
| `options.dir`              | `string`  | `undefined`    | Directorio para enumerar sesiones. Cuando se omite, devuelve sesiones en todos los proyectos    |
| `options.limit`            | `number`  | `undefined`    | Número máximo de sesiones a devolver                                                            |
| `options.includeWorktrees` | `boolean` | `true`         | Cuando `dir` está dentro de un repositorio git, incluya sesiones de todas las rutas de worktree |

<h4 id="return-type-sdksessioninfo">
  Tipo de retorno: `SDKSessionInfo`
</h4>

| Propiedad      | Tipo                  | Descripción                                                                                      |
| :------------- | :-------------------- | :----------------------------------------------------------------------------------------------- |
| `sessionId`    | `string`              | Identificador de sesión único (UUID)                                                             |
| `summary`      | `string`              | Título de visualización: título personalizado, resumen generado automáticamente o primer mensaje |
| `lastModified` | `number`              | Última hora de modificación en milisegundos desde la época                                       |
| `fileSize`     | `number \| undefined` | Tamaño del archivo de sesión en bytes. Solo se completa para almacenamiento JSONL local          |
| `customTitle`  | `string \| undefined` | Título de sesión establecido por el usuario (a través de `/rename`)                              |
| `firstPrompt`  | `string \| undefined` | Primer mensaje de usuario significativo en la sesión                                             |
| `gitBranch`    | `string \| undefined` | Rama Git al final de la sesión                                                                   |
| `cwd`          | `string \| undefined` | Directorio de trabajo para la sesión                                                             |
| `tag`          | `string \| undefined` | Etiqueta de sesión establecida por el usuario (vea [`tagSession()`](#tagsession))                |
| `createdAt`    | `number \| undefined` | Hora de creación en milisegundos desde la época, de la marca de tiempo de la primera entrada     |

<h4 id="example-2">
  Ejemplo
</h4>

Imprima las 10 sesiones más recientes para un proyecto. Los resultados se ordenan por `lastModified` descendente, por lo que el primer elemento es el más nuevo. Omita `dir` para buscar en todos los proyectos.

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

Lee mensajes de usuario y asistente de una transcripción de sesión pasada.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  Parámetros
</h4>

| Parámetro        | Tipo     | Predeterminado | Descripción                                                                                    |
| :--------------- | :------- | :------------- | :--------------------------------------------------------------------------------------------- |
| `sessionId`      | `string` | requerido      | UUID de sesión a leer (vea `listSessions()`)                                                   |
| `options.dir`    | `string` | `undefined`    | Directorio de proyecto para encontrar la sesión. Cuando se omite, busca en todos los proyectos |
| `options.limit`  | `number` | `undefined`    | Número máximo de mensajes a devolver                                                           |
| `options.offset` | `number` | `undefined`    | Número de mensajes a omitir desde el inicio                                                    |

<h4 id="return-type-sessionmessage">
  Tipo de retorno: `SessionMessage`
</h4>

| Propiedad            | Tipo                    | Descripción                                                                                                                                                                                                                                                                     |
| :------------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`               | `"user" \| "assistant"` | Rol del mensaje                                                                                                                                                                                                                                                                 |
| `uuid`               | `string`                | Identificador de mensaje único                                                                                                                                                                                                                                                  |
| `session_id`         | `string`                | Sesión a la que pertenece este mensaje                                                                                                                                                                                                                                          |
| `message`            | `unknown`               | Carga útil de mensaje sin procesar de la transcripción                                                                                                                                                                                                                          |
| `parent_tool_use_id` | `string \| null`        | Para mensajes de subagente, el `tool_use_id` de la llamada de herramienta `Agent` que lo generó. `null` para mensajes de sesión principal y sesiones más antiguas                                                                                                               |
| `parent_agent_id`    | `string \| null`        | Para mensajes de un [subagente anidado](/docs/es/sub-agents#spawn-nested-subagents), el `agentId` del subagente que lo generó. `null` para mensajes de sesión principal, mensajes de subagentes de nivel superior y sesiones más antiguas. Requiere Claude Code v2.1.202 o posterior |

<h4 id="example-3">
  Ejemplo
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

Lee metadatos para una única sesión por ID sin escanear el directorio de proyecto completo.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  Parámetros
</h4>

| Parámetro     | Tipo     | Predeterminado | Descripción                                                                                   |
| :------------ | :------- | :------------- | :-------------------------------------------------------------------------------------------- |
| `sessionId`   | `string` | requerido      | UUID de la sesión a buscar                                                                    |
| `options.dir` | `string` | `undefined`    | Ruta del directorio del proyecto. Cuando se omite, busca en todos los directorios de proyecto |

Devuelve [`SDKSessionInfo`](#return-type-sdksessioninfo), o `undefined` si la sesión no se encuentra.

<h3 id="renamesession">
  `renameSession()`
</h3>

Cambia el nombre de una sesión añadiendo una entrada de título personalizado. Las llamadas repetidas son seguras; el título más reciente gana.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  Parámetros
</h4>

| Parámetro     | Tipo     | Predeterminado | Descripción                                                                                   |
| :------------ | :------- | :------------- | :-------------------------------------------------------------------------------------------- |
| `sessionId`   | `string` | requerido      | UUID de la sesión a renombrar                                                                 |
| `title`       | `string` | requerido      | Nuevo título. Debe ser no vacío después de recortar espacios en blanco                        |
| `options.dir` | `string` | `undefined`    | Ruta del directorio del proyecto. Cuando se omite, busca en todos los directorios de proyecto |

<h3 id="tagsession">
  `tagSession()`
</h3>

Etiqueta una sesión. Pase `null` para borrar la etiqueta. Las llamadas repetidas son seguras; la etiqueta más reciente gana.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  Parámetros
</h4>

| Parámetro     | Tipo             | Predeterminado | Descripción                                                                                   |
| :------------ | :--------------- | :------------- | :-------------------------------------------------------------------------------------------- |
| `sessionId`   | `string`         | requerido      | UUID de la sesión a etiquetar                                                                 |
| `tag`         | `string \| null` | requerido      | Cadena de etiqueta, o `null` para borrar                                                      |
| `options.dir` | `string`         | `undefined`    | Ruta del directorio del proyecto. Cuando se omite, busca en todos los directorios de proyecto |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Resuelve la configuración efectiva de Claude Code para un directorio determinado utilizando el mismo motor de fusión que la CLI, sin generar la CLI de Claude. Úselo para inspeccionar qué configuración vería una llamada a `query()` antes de invocar una.

<Note>
  Esta función es alfa y su API puede cambiar antes de la estabilización. Lee fuentes MDM, incluidas plist de macOS y HKLM/HKCU de Windows, para paridad con el inicio de la CLI, pero no ejecuta el subproceso `policyHelper` configurado por el administrador. El campo `permissions.defaultMode` se devuelve tal como está de todos los niveles, incluida la configuración del proyecto. El filtro de confianza que la CLI aplica antes de honrar los modos de permiso escalonados no se aplica.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parámetros
</h4>

`resolveSettings()` acepta un único objeto de opciones. Todos los campos son opcionales.

| Parámetro                       | Tipo                                  | Predeterminado    | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------------ | :------------------------------------ | :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()`   | Directorio para resolver la configuración del proyecto y local relativa a                                                                                                                                                                                                                                                                                                                                                                                       |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Todas las fuentes | Qué fuentes del sistema de archivos cargar. Pase `[]` para omitir la configuración del usuario, proyecto y local. La configuración de políticas administradas se carga en todos los casos. La configuración administrada por servidor se toma de `serverManagedSettings` cuando el host la pasa, o se lee de la caché en disco de la CLI en caso contrario; la instantánea no las obtiene de la red                                                             |
| `options.managedSettings`       | `Settings`                            | `undefined`       | Configuración de política restrictiva suministrada por el host de incrustación. Se descarta por defecto cuando una política administrada implementada por el administrador está presente; se fusiona bajo ese nivel cuando [`parentSettingsBehavior`](/docs/es/settings#available-settings) es `"merge"`. Las claves no restrictivas como `model` se descartan silenciosamente para que esta opción pueda restringir la política administrada pero no flexibilizarla |
| `options.serverManagedSettings` | `Settings`                            | `undefined`       | Carga útil de configuración administrada por servidor desde `/api/claude_code/settings`. Las claves no restrictivas pasan sin filtrar                                                                                                                                                                                                                                                                                                                           |

<h4 id="return-type-resolvedsettings">
  Tipo de retorno: `ResolvedSettings`
</h4>

`resolveSettings()` devuelve un objeto que describe la configuración fusionada y la fuente que contribuyó a cada clave.

| Propiedad    | Tipo                                                | Descripción                                                                                      |
| :----------- | :-------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| `effective`  | `Settings`                                          | Configuración fusionada después de aplicar todas las fuentes habilitadas en orden de precedencia |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Para cada clave de nivel superior en `effective`, qué fuente suministró el valor                 |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Configuración sin procesar por fuente, ordenada de precedencia más baja a más alta               |

<h4 id="example-4">
  Ejemplo
</h4>

El ejemplo a continuación resuelve la configuración para un directorio de proyecto e imprime la fuente que controla el período de limpieza.

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  Tipos
</h2>

<h3 id="options">
  `Options`
</h3>

Objeto de configuración para la función `query()`.

| Propiedad                         | Tipo                                                                                                     | Predeterminado                                     | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`                            | Controlador para cancelar operaciones                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                                               | Directorios adicionales a los que Claude puede acceder                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agent`                           | `string`                                                                                                 | `undefined`                                        | Nombre del agente para el hilo principal. El agente debe estar definido en la opción `agents` o en la configuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                                        | Defina subagentes mediante programación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                                            | Cuando es `true`, genere resúmenes de progreso de una línea para subagentes y reenvíelos en eventos [`task_progress`](#sdktaskprogressmessage) a través del campo `summary`. Se aplica a subagentes en primer plano y en segundo plano                                                                                                                                                                                                                                                                                                                                                                                                      |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                                            | Habilite omitir permisos. Requerido cuando se usa `permissionMode: 'bypassPermissions'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                                               | Herramientas para aprobar automáticamente sin solicitar. Esto no restringe Claude a solo estas herramientas; las herramientas no listadas caen en `permissionMode` y `canUseTool`. Use `disallowedTools` para bloquear herramientas. Vea [Permissions](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                      |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                                               | Habilite características beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                                        | Función de permiso personalizado, invocada solo cuando el [flujo de permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) cae en un mensaje. No se invoca para llamadas preaprobadas por `allowedTools`, reglas de permiso, o `permissionMode`. `AskUserQuestion`, herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) la alcanzan incluso si las ha permitido; en modo `dontAsk` se niegan en su lugar. Vea [`CanUseTool`](#canusetool) para detalles |
| `continue`                        | `boolean`                                                                                                | `false`                                            | Continúe la conversación más reciente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                                    | Directorio de trabajo actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `debug`                           | `boolean`                                                                                                | `false`                                            | Habilite el modo de depuración para el proceso de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `debugFile`                       | `string`                                                                                                 | `undefined`                                        | Escriba registros de depuración en una ruta de archivo específica. Habilita implícitamente el modo de depuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                                               | Herramientas a negar. Un nombre simple como `"Bash"` elimina la herramienta del contexto de Claude. Una regla con alcance como `"Bash(rm *)"` deja la herramienta disponible y niega las llamadas coincidentes en cada modo de permiso, incluyendo `bypassPermissions`. Vea [Permissions](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                   |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | Predeterminado del modelo                          | Controla cuánto esfuerzo pone Claude en su respuesta. Funciona con el pensamiento adaptativo para guiar la profundidad del pensamiento. Vea [adjust the effort level](/docs/es/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                                            | Habilite el seguimiento de cambios de archivo para rebobinar. Vea [File checkpointing](/docs/es/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                                      | Variables de entorno. Cuando se establece, esto reemplaza el entorno del subproceso en lugar de fusionarse con `process.env`, así que pase `{ ...process.env, YOUR_VAR: 'value' }` para mantener variables heredadas como `PATH`. Vea [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) para un ejemplo de este patrón, y [Environment variables](/docs/es/env-vars) para variables que la CLI subyacente lee. Establezca `CLAUDE_AGENT_SDK_CLIENT_APP` para identificar su aplicación en el encabezado User-Agent                                                                                                   |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | Detectado automáticamente                          | Tiempo de ejecución de JavaScript a usar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                                               | Argumentos a pasar al ejecutable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                                               | Argumentos adicionales                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                                        | Modelo a usar si el principal falla                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `forkSession`                     | `boolean`                                                                                                | `false`                                            | Cuando se reanuda con `resume`, bifurque a un nuevo ID de sesión en lugar de continuar la sesión original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                                            | Reenvíe bloques de texto y pensamiento de subagentes como mensajes de asistente y usuario con `parent_tool_use_id` establecido, para que los consumidores puedan renderizar una transcripción anidada. Por defecto, solo se emiten bloques `tool_use` y `tool_result` de subagentes                                                                                                                                                                                                                                                                                                                                                         |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                                               | Devoluciones de llamada de hooks para eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                                            | Incluya eventos del ciclo de vida de hooks en la transmisión de mensajes como [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), y [`SDKHookResponseMessage`](#sdkhookresponsemessage). Los eventos del ciclo de vida para hooks `SessionStart` y `Setup` siempre se incluyen y no necesitan esta opción                                                                                                                                                                                                                                                                               |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                                            | Incluya eventos de mensaje parcial                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                                            | *Alpha.* Tiempo de espera en milisegundos para cada llamada `sessionStore.load()` y `sessionStore.listSubkeys()` durante la materialización de reanudación. Si el adaptador no se resuelve dentro de esta ventana, la consulta falla en lugar de colgarse. Se ignora cuando `sessionStore` no está establecido                                                                                                                                                                                                                                                                                                                              |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                                        | Configuración de nivel de política suministrada por el proceso padre que genera. Se descarta cuando ya existe una capa de configuración administrada controlada por TI en la máquina, a menos que ese administrador opte por `parentSettingsBehavior: 'merge'`. Se filtra a solo claves restrictivas independientemente                                                                                                                                                                                                                                                                                                                     |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                                        | Detenga la consulta cuando la estimación de costo del lado del cliente alcance este valor en USD. Comparado con la misma estimación que `total_cost_usd`; vea [Track cost and usage](/docs/es/agent-sdk/cost-tracking) para advertencias de precisión                                                                                                                                                                                                                                                                                                                                                                                            |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                                        | *Deprecado:* Use `thinking` en su lugar. Tokens máximos para el proceso de pensamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                                        | Número máximo de turnos agentes (viajes de ronda de uso de herramientas)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                                               | Configuraciones de servidor MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `model`                           | `string`                                                                                                 | Predeterminado de CLI                              | Alias de modelo Claude o nombre de modelo completo. Vea [accepted values and provider-specific IDs](/docs/es/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                                        | Devolución de llamada para manejar solicitudes de elicitación de MCP. Se llama cuando un servidor MCP solicita entrada del usuario y ningún hook la maneja primero. Cuando no se proporciona, las solicitudes de elicitación no manejadas se rechazan automáticamente                                                                                                                                                                                                                                                                                                                                                                       |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                                        | Defina el formato de salida para los resultados del agente. Vea [Structured outputs](/docs/es/agent-sdk/structured-outputs) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                                        | No es un campo `Options`. Establezca `outputStyle` en el objeto [`settings`](/docs/es/settings) en línea o en un archivo de configuración en su lugar. Vea [Activate an output style](/docs/es/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                           |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | Auto-resuelto desde el binario nativo incluido     | Ruta al ejecutable de Claude Code. Solo se necesita si las dependencias opcionales se omitieron durante la instalación o su plataforma no está en el conjunto compatible                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                                        | Modo de permiso para la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                                        | Nombre de herramienta MCP para solicitudes de permiso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `persistSession`                  | `boolean`                                                                                                | `true`                                             | Cuando es `false`, deshabilita la persistencia de sesión en disco. Las sesiones no se pueden reanudar más tarde                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                                        | Instrucciones de flujo de trabajo personalizado para Plan Mode. Cuando `permissionMode` es `'plan'`, esta cadena reemplaza el cuerpo de flujo de trabajo de Plan Mode predeterminado. La CLI aún lo envuelve con el preámbulo de cumplimiento de solo lectura y el pie de página del protocolo ExitPlanMode                                                                                                                                                                                                                                                                                                                                 |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                                               | Cargue plugins personalizados desde rutas locales. Vea [Plugins](/docs/es/agent-sdk/plugins) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                                            | Habilite sugerencias de mensaje. Emite un mensaje `prompt_suggestion` después de cada turno con un mensaje de usuario predicho siguiente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resume`                          | `string`                                                                                                 | `undefined`                                        | ID de sesión a reanudar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                                        | Reanude la sesión en un UUID de mensaje específico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                                        | Configure el comportamiento de sandbox mediante programación. Vea [Sandbox settings](#sandboxsettings) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `sessionId`                       | `string`                                                                                                 | Auto-generado                                      | Use un UUID específico para la sesión en lugar de generar uno automáticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `sessionStore`                    | [`SessionStore`](/docs/es/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                                        | Refleje transcripciones de sesión en un backend externo para que cualquier host pueda reanudarlas. Vea [Persist sessions to external storage](/docs/es/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                                        | *Alpha.* Modo de vaciado para `sessionStore`. Se ignora cuando `sessionStore` no está establecido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                                        | Objeto de [settings](/docs/es/settings) en línea o ruta a un archivo de configuración. Completa la capa de configuración de marca en el [orden de precedencia](/docs/es/settings#settings-precedence). Cambie en tiempo de ejecución con [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                  |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | Valores predeterminados de CLI (todas las fuentes) | Controle qué configuración del sistema de archivos cargar. Pase `[]` para deshabilitar la configuración de usuario, proyecto y local. La configuración de política administrada se carga independientemente; la configuración administrada por servidor se obtiene cuando la sesión se autentica con una credencial de organización en una [configuración elegible](/docs/es/server-managed-settings#platform-availability). Vea [Use Claude Code features](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                             |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                                        | Skills disponibles para la sesión. Pase `'all'` para habilitar cada skill descubierto, o una lista de nombres de skills. Cuando se establece, el SDK agrega la herramienta Skill a `allowedTools` automáticamente. Si también pasa `tools`, incluya `'Skill'` en esa lista. Vea [Skills](/docs/es/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                              |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                                        | Función personalizada para generar el proceso de Claude Code. Use para ejecutar Claude Code en máquinas virtuales, contenedores o entornos remotos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                                        | Devolución de llamada para salida de stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                                            | Use solo los servidores pasados en `mcpServers` e ignore el proyecto `.mcp.json`, la configuración del usuario, los servidores MCP proporcionados por plugins, y [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                          |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (mensaje mínimo)                       | Configuración de mensaje del sistema. Pase una cadena para un mensaje personalizado, o `{ type: 'preset', preset: 'claude_code' }` para usar el mensaje del sistema de Claude Code. Cuando use la forma de objeto preestablecido, agregue `append` para extenderlo con instrucciones adicionales, y establezca `excludeDynamicSections: true` para mover el contexto por sesión al primer mensaje de usuario para [mejor reutilización de caché de mensaje en máquinas](/docs/es/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                            |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                                        | *Alpha.* Presupuesto de tarea del lado de la API en tokens. Cuando se establece, se le dice al modelo su presupuesto de token restante para que pueda controlar el uso de herramientas y terminar antes del límite                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | `{ type: 'adaptive' }` para modelos compatibles    | Controla el comportamiento de pensamiento/razonamiento de Claude. Vea [`ThinkingConfig`](#thinkingconfig) para opciones                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `title`                           | `string`                                                                                                 | `undefined`                                        | Título de visualización para la sesión. Cuando se reanuda a través de `resume` o `continue`, el título persistente de la sesión reanudada tiene precedencia; use [`renameSession()`](#renamesession) para cambiar el título de una sesión existente                                                                                                                                                                                                                                                                                                                                                                                         |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                                        | Mapee nombres de herramientas integradas a nombres de herramientas MCP para que Claude llame a su implementación MCP en lugar de la integrada. Por ejemplo, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                                        | Configuración para el comportamiento de herramientas integradas. Vea [`ToolConfig`](#toolconfig) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                                        | Configuración de herramientas. Pase una matriz de nombres de herramientas o use el preestablecido para obtener las herramientas predeterminadas de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="handle-slow-or-stalled-api-responses">
  Manejo de respuestas de API lentas o estancadas
</h4>

El subproceso CLI lee varias variables de entorno que controlan los tiempos de espera de API y la detección de estancamiento. Páselas a través de la opción `env`:

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS`: tiempo de espera por solicitud en el cliente de Anthropic, en milisegundos. Predeterminado `600000`. Se aplica al bucle principal y a todos los subagentes.
* `CLAUDE_CODE_MAX_RETRIES`: máximo de reintentos de API. Predeterminado `10`, limitado a `15`. Cada reintento obtiene su propia ventana `API_TIMEOUT_MS`, por lo que el tiempo de pared en el peor caso es aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` más retroceso. Para ejecuciones desatendidas que necesitan esperar a través de interrupciones más largas, establezca `CLAUDE_CODE_RETRY_WATCHDOG=1`: reintenta errores de capacidad indefinidamente, y a partir de Claude Code v2.1.199 eleva el predeterminado para otros errores transitorios a `300` y elimina el límite en esta variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: perro guardián de estancamiento para subagentes lanzados con `run_in_background`. Predeterminado `600000`. Se reinicia en cada evento de transmisión; en caso de estancamiento, aborta el subagente, marca la tarea como fallida y expone el error al padre con cualquier resultado parcial. No se aplica a subagentes síncronos.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` con `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: aborta la solicitud cuando los encabezados han llegado pero el cuerpo de respuesta deja de transmitirse. El perro guardián está activado de forma predeterminada para todos los proveedores; establezca `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desactivarlo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` tiene un valor predeterminado de `300000` y se fija a ese mínimo. La solicitud abortada pasa por la ruta de reintento normal.

<h3 id="query-object">
  Objeto `Query`
</h3>

Interfaz devuelta por la función `query()`.

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Métodos
</h4>

| Método                                 | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrumpe la consulta. Solo disponible en modo de entrada de transmisión. Cuando la CLI anuncia la capacidad `interrupt_receipt_v1` en [`SDKSystemMessage.capabilities`](#sdksystemmessage), se resuelve con un [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) que enumera los mensajes en cola que sobreviven a la interrupción. Se resuelve a `undefined` en CLIs anteriores a v2.1.205                                                                                                                                                  |
| `rewindFiles(userMessageId, options?)` | Restaura archivos a su estado en el mensaje de usuario especificado. Pase `{ dryRun: true }` para obtener una vista previa de los cambios. Requiere `enableFileCheckpointing: true`. Vea [File checkpointing](/docs/es/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                            |
| `setPermissionMode()`                  | Cambia el modo de permiso (solo disponible en modo de entrada de transmisión)                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `setModel()`                           | Cambia el modelo (solo disponible en modo de entrada de transmisión)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `setMaxThinkingTokens()`               | *Deprecado:* Use la opción `thinking` en su lugar. Cambia los tokens de pensamiento máximos. Pasar `null` reinicia el pensamiento al valor predeterminado de la sesión: se borra una anulación a mitad de sesión, y el pensamiento permanece desactivado para sesiones que lo tienen deshabilitado                                                                                                                                                                                                                                                         |
| `applyFlagSettings(settings)`          | Fusiona la configuración en la capa de configuración de marca de la sesión en tiempo de ejecución (solo disponible en modo de entrada de transmisión). Vea [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                     |
| `initializationResult()`               | Devuelve el resultado de inicialización completo incluyendo comandos compatibles, modelos, información de cuenta y configuración de estilo de salida                                                                                                                                                                                                                                                                                                                                                                                                       |
| `reinitialize()`                       | Reenvía la solicitud de control `initialize` a la CLI en ejecución y devuelve un resultado nuevo en lugar del resultado de primera conexión en caché. Úselo después de una brecha de transporte, como reconectarse a una sesión después de una desconexión, para que las solicitudes de permiso pendientes lleguen a su devolución de llamada `canUseTool` nuevamente. Haga que la devolución de llamada sea idempotente por ID de solicitud, porque una solicitud cuya respuesta se perdió se envía nuevamente. Requiere Claude Code v2.1.195 o posterior |
| `supportedCommands()`                  | Devuelve comandos slash disponibles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `supportedModels()`                    | Devuelve modelos disponibles con información de visualización                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `supportedAgents()`                    | Devuelve subagentes disponibles como [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `mcpServerStatus()`                    | Devuelve el estado de los servidores MCP conectados                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `accountInfo()`                        | Devuelve información de cuenta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `reconnectMcpServer(serverName)`       | Reconecte un servidor MCP por nombre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `toggleMcpServer(serverName, enabled)` | Habilite o deshabilite un servidor MCP por nombre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `setMcpServers(servers)`               | Reemplace dinámicamente el conjunto de servidores MCP para esta sesión. Devuelve información sobre qué servidores se agregaron, eliminaron y cualquier error                                                                                                                                                                                                                                                                                                                                                                                               |
| `streamInput(stream)`                  | Transmita mensajes de entrada a la consulta para conversaciones de múltiples turnos                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `stopTask(taskId)`                     | Detenga una tarea de fondo en ejecución por ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `close()`                              | Cierre la consulta y termine el proceso subyacente. Finaliza forzadamente la consulta y limpia todos los recursos                                                                                                                                                                                                                                                                                                                                                                                                                                          |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Cambia [settings](/docs/es/settings) en una sesión en ejecución sin reiniciar la consulta. Úselo cuando una configuración que no tiene un setter dedicado necesite cambiar a mitad de sesión, como restringir `permissions` después de que el agente lea entrada no confiable. `setModel()` y `setPermissionMode()` son setters dedicados para esas dos claves; `applyFlagSettings()` es la forma general que acepta cualquier subconjunto de las claves de configuración, y pasar `model` aquí se comporta igual que `setModel()`.

Solo algunas claves tienen efecto a mitad de sesión:

* **Aplicadas en el siguiente turno**: `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Cambiar `agent` también aplica la anulación de modelo, hooks y mensaje del sistema de ese agente en el siguiente turno.
* **Sin efecto a mitad de sesión**: las opciones de mensaje del sistema. Estos se resuelven una vez al inicio, por lo que la sesión en ejecución mantiene el valor original aunque la llamada tenga éxito. Para cambiarlos, inicie una nueva sesión.

`effortLevel` acepta un nombre de [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level). También acepta `"ultracode"`, que ejecuta la sesión a esfuerzo `xhigh` y activa [ultracode](/docs/es/workflows#let-claude-decide-with-ultracode). El tipo `Settings` declara `effortLevel` sin ese valor, así que pase el equivalente `{ ultracode: true }` en TypeScript. El valor `ultracode` requiere Claude Code v2.1.203 o posterior y solo es aceptado por `applyFlagSettings()`, no por la clave `effortLevel` en un archivo de configuración.

Los valores se escriben en la capa de configuración de marca, la misma capa que la opción `settings` en línea de `query()` completa al inicio. La configuración de marca se encuentra cerca de la parte superior del [orden de precedencia de configuración](/docs/es/settings#settings-precedence): anulan la configuración de usuario, proyecto y local, y solo la configuración de política administrada puede anularlas. Esta es la misma capa que la [sección de precedencia en la página](#settings-precedence) llama opciones programáticas.

Las llamadas sucesivas fusionan superficialmente las claves de nivel superior. Una segunda llamada con `{ permissions: {...} }` reemplaza el objeto `permissions` completo de la llamada anterior en lugar de fusionarse profundamente en él. Para borrar una clave de la capa de marca y recurrir a fuentes de menor precedencia, pase `null` para esa clave. Pasar `undefined` no tiene efecto porque la serialización JSON lo elimina.

Solo disponible en modo de entrada de transmisión, la misma restricción que `setModel()` y `setPermissionMode()`.

El ejemplo a continuación cambia el modelo activo a mitad de sesión, luego borra la anulación para que el modelo recurra a lo que especifique la configuración del usuario o proyecto.

```typescript theme={null}
const q = query({ prompt: messageStream });

// Anule el modelo para el resto de la sesión
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Más tarde: borre la anulación y recurra a la configuración de menor precedencia
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` es solo TypeScript. El SDK de Python no expone un método equivalente.
</Note>

<h3 id="warmquery">
  `WarmQuery`
</h3>

Identificador devuelto por [`startup()`](#startup). El subproceso ya está generado e inicializado, por lo que llamar a `query()` en este identificador escribe el mensaje directamente en un proceso listo sin latencia de inicio.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Métodos
</h4>

| Método          | Descripción                                                                                                                      |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Envíe un mensaje al subproceso precalentado y devuelva un [`Query`](#query-object). Solo se puede llamar una vez por `WarmQuery` |
| `close()`       | Cierre el subproceso sin enviar un mensaje. Use esto para descartar una consulta cálida que ya no es necesaria                   |

`WarmQuery` implementa `AsyncDisposable`, por lo que se puede usar con `await using` para limpieza automática.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Tipo de retorno de `initializationResult()`. Contiene datos de inicialización de sesión.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
};
```

Cuando un cliente envía `initialize` a una sesión que ya se está ejecutando, el contenedor de respuesta de control también lleva una matriz `pending_permission_requests` opcional. El campo está en el contenedor de respuesta en sí, no en la carga `SDKControlInitializeResponse` anterior. Cada entrada es un mensaje `control_request` completo con la misma forma `{ type: "control_request", request_id, request }` que la sesión transmite para solicitudes de permiso mientras se ejecuta.

Estas son solicitudes que se emitieron antes de que el cliente se conectara y aún están esperando una respuesta. El SDK lee la matriz para usted y envía cada entrada a su devolución de llamada [`canUseTool`](#canusetool), el mismo reenvío que [`reinitialize()`](#query-object) activa después de una brecha de transporte. Maneje IDs de solicitud repetidos de forma idempotente, porque una entrada puede repetir una solicitud que la devolución de llamada ya recibió antes de que se cayera la conexión.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

El recibo de interrupción: el valor que [`interrupt()`](#query-object) se resuelve con en una CLI que anuncia la capacidad `interrupt_receipt_v1` en [`SDKSystemMessage.capabilities`](#sdksystemmessage). Requiere Claude Code v2.1.205 o posterior. Las CLIs anteriores responden a la interrupción con una carga de éxito vacía, por lo que `interrupt()` se resuelve a `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` enumera los UUIDs de los mensajes de usuario que sobreviven a la interrupción: mensajes aún en la cola, más cualquier lote ya dequeued para el siguiente turno pero aún no alcanzable por la anulación. Cada uno se ejecuta como su propio turno después de la interrupción a menos que lo cancele primero. Use el recibo para decidir si debe reenviar algo; reenviar un mensaje que ya está listado produce un turno duplicado.

Interprete la lista con estas advertencias:

* Solo los mensajes que fueron encolados con un UUID aparecen. Una matriz vacía no significa que nada más se ejecutará.
* Solo se enumeran los mensajes del hilo principal. Los mensajes dirigidos a un subagente están fuera del alcance.
* La lista puede incluir UUIDs que su cliente nunca envió, como [activadores de tareas programadas](/docs/es/scheduled-tasks). Ignore los UUIDs que no reconozca en lugar de tratarlos como un error.

El recibo es una instantánea tomada en el momento en que se procesa la interrupción, y en una interrupción limpia llega antes del [`SDKResultMessage`](#sdkresultmessage) del turno interrumpido. Lea el recibo en lugar de inspeccionar la cola después de ese resultado: el bucle inicia el siguiente turno en cola inmediatamente, por lo que la cola que inspecciona después del resultado ya ha cambiado.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configuración para un subagente definido mediante programación.

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Campo                                 | Requerido | Descripción                                                                                                                                                                                                                                                                          |
| :------------------------------------ | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Sí        | Descripción en lenguaje natural de cuándo usar este agente                                                                                                                                                                                                                           |
| `tools`                               | No        | Matriz de nombres de herramientas permitidas. Si se omite, hereda todas las herramientas del padre. Para precargar Skills en el contexto del agente, use el campo `skills` en lugar de enumerar `'Skill'` aquí                                                                       |
| `disallowedTools`                     | No        | Matriz de nombres de herramientas a desautorizar explícitamente para este agente. Los patrones de nivel de servidor MCP también se aceptan: `mcp__server` o `mcp__server__*` elimina cada herramienta de ese servidor, y `mcp__*` elimina cada herramienta MCP de cualquier servidor |
| `prompt`                              | Sí        | El mensaje del sistema del agente                                                                                                                                                                                                                                                    |
| `model`                               | No        | Anulación de modelo para este agente. Acepta un alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID de modelo completo. Si se omite o es `'inherit'`, usa el modelo principal                                                                                |
| `mcpServers`                          | No        | Especificaciones de servidor MCP para este agente                                                                                                                                                                                                                                    |
| `skills`                              | No        | Matriz de nombres de skills a precargar en el contexto del agente                                                                                                                                                                                                                    |
| `initialPrompt`                       | No        | Auto-enviado como el primer turno de usuario cuando este agente se ejecuta como el agente del hilo principal                                                                                                                                                                         |
| `maxTurns`                            | No        | Número máximo de turnos agentes (viajes de ronda de API) antes de detener                                                                                                                                                                                                            |
| `background`                          | No        | Ejecute este agente como una tarea de fondo no bloqueante cuando se invoque                                                                                                                                                                                                          |
| `memory`                              | No        | Fuente de memoria para este agente: `'user'`, `'project'`, o `'local'`                                                                                                                                                                                                               |
| `effort`                              | No        | Nivel de esfuerzo de razonamiento para este agente. Acepta un nivel nombrado o un entero                                                                                                                                                                                             |
| `permissionMode`                      | No        | Modo de permiso para la ejecución de herramientas dentro de este agente. Vea [`PermissionMode`](#permissionmode)                                                                                                                                                                     |
| `criticalSystemReminder_EXPERIMENTAL` | No        | Experimental: Recordatorio crítico agregado al mensaje del sistema                                                                                                                                                                                                                   |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Especifica servidores MCP disponibles para un subagente. Puede ser un nombre de servidor (cadena que hace referencia a un servidor de la configuración `mcpServers` del padre) o una configuración de servidor en línea que mapea nombres de servidor a configuraciones.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Donde `McpServerConfigForProcessTransport` es `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Controla qué fuentes de configuración basadas en el sistema de archivos carga el SDK.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Valor       | Descripción                                                   | Ubicación                     |
| :---------- | :------------------------------------------------------------ | :---------------------------- |
| `'user'`    | Configuración global del usuario                              | `~/.claude/settings.json`     |
| `'project'` | Configuración de proyecto compartida (controlada por versión) | `.claude/settings.json`       |
| `'local'`   | Configuración de proyecto local (no controlada por versión)   | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamiento predeterminado
</h4>

Cuando `settingSources` se omite o es `undefined`, `query()` carga la misma configuración del sistema de archivos que la CLI de Claude Code: usuario, proyecto y local. La configuración de política administrada se carga en todos los casos; la configuración administrada por servidor se obtiene cuando la sesión se autentica con una credencial de organización en una [configuración elegible](/docs/es/server-managed-settings#platform-availability). Vea [What settingSources does not control](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que se leen independientemente de esta opción, y cómo deshabilitarlas.

<h4 id="why-use-settingsources">
  Por qué usar settingSources
</h4>

**Deshabilitar configuración del sistema de archivos:**

```typescript theme={null}
// No cargue la configuración de usuario, proyecto o local desde el disco
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Cargue toda la configuración del sistema de archivos explícitamente:**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // Cargue toda la configuración
  }
});
```

**Cargue solo fuentes de configuración específicas:**

```typescript theme={null}
// Cargue solo la configuración del proyecto, ignore usuario y local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Solo .claude/settings.json
  }
});
```

**Entornos de prueba e IC:**

```typescript theme={null}
// Asegure un comportamiento consistente en IC excluyendo la configuración local
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // Solo configuración compartida del equipo
    permissionMode: "bypassPermissions"
  }
});
```

**Aplicaciones solo SDK:**

```typescript theme={null}
// Defina todo mediante programación.
// Pase [] para optar por no usar fuentes de configuración del sistema de archivos.
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**Cargando instrucciones de proyecto CLAUDE.md:**

```typescript theme={null}
// Cargue la configuración del proyecto para incluir archivos CLAUDE.md
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Use el mensaje del sistema de Claude Code
    },
    settingSources: ["project"], // Carga CLAUDE.md del directorio del proyecto
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  Precedencia de configuración
</h4>

Cuando se cargan múltiples fuentes, la configuración se fusiona con esta precedencia (mayor a menor):

1. Configuración local (`.claude/settings.local.json`)
2. Configuración del proyecto (`.claude/settings.json`)
3. Configuración del usuario (`~/.claude/settings.json`)

Las opciones programáticas como `agents`, `allowedTools` y `settings` anulan la configuración del sistema de archivos de usuario, proyecto y local. La configuración de política administrada tiene precedencia sobre las opciones programáticas.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Comportamiento de permiso estándar
  | "acceptEdits" // Auto-aceptar ediciones de archivo
  | "bypassPermissions" // Omitir todas las verificaciones de permiso; las reglas de solicitud explícita aún solicitan
  | "plan" // Modo de planificación - explorar sin editar
  | "dontAsk" // No solicitar permisos, negar si no está preaprobado
  | "auto"; // Usar un clasificador de modelo para aprobar o negar cada llamada de herramienta
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Tipo de función de permiso personalizado para controlar el uso de herramientas.

La función es el reemplazo del SDK para el mensaje de permiso interactivo: se invoca solo cuando el [flujo de evaluación de permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) se resuelve en un mensaje. Las llamadas de herramientas ya aprobadas por una entrada `allowedTools`, una regla de permiso de configuración, o el modo de permiso, como `acceptEdits` o `bypassPermissions`, nunca la invocan. Para controlar cada llamada de herramienta, use un [hook `PreToolUse`](/docs/es/agent-sdk/hooks) en su lugar.

`AskUserQuestion`, herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool), y herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) la alcanzan incluso cuando una regla de permiso coincide. En modo `dontAsk` estas llamadas se niegan en su lugar, sin invocarla.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Opción           | Tipo                                        | Descripción                                                                                                                                                                                                                                                                                                                                                      |
| :--------------- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Señalizado si la operación debe abortarse                                                                                                                                                                                                                                                                                                                        |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Actualizaciones de permiso sugeridas para que el usuario no sea solicitado nuevamente para esta herramienta. Los mensajes de Bash incluyen una sugerencia con el destino `localSettings` [destination](#permissionupdatedestination), por lo que devolverlo en `updatedPermissions` escribe la regla en `.claude/settings.local.json` y persiste entre sesiones. |
| `blockedPath`    | `string`                                    | La ruta de archivo que activó la solicitud de permiso, si corresponde                                                                                                                                                                                                                                                                                            |
| `decisionReason` | `string`                                    | Explica por qué se activó esta solicitud de permiso                                                                                                                                                                                                                                                                                                              |
| `toolUseID`      | `string`                                    | Identificador único para esta llamada de herramienta específica dentro del mensaje del asistente                                                                                                                                                                                                                                                                 |
| `agentID`        | `string`                                    | Si se ejecuta dentro de un sub-agente, el ID del sub-agente                                                                                                                                                                                                                                                                                                      |
| `requestId`      | `string`                                    | El `request_id` del sobre `control_request`. Una `control_response` que su aplicación envía fuera del SDK, como un POST HTTP firmado, debe repetir este valor para que el proceso de Claude Code pueda coincidir la respuesta con la solicitud                                                                                                                   |

La devolución de llamada normalmente resuelve la solicitud devolviendo un [`PermissionResult`](#permissionresult), que el SDK escribe de vuelta sobre su transporte como `control_response`. Devuelva `null` solo cuando su aplicación ya haya enviado `control_response` para esta solicitud sobre su propio canal, repitiendo `requestId`; el SDK luego omite escribir la respuesta a su transporte. Devolver `null` en cualquier otro caso deja la llamada de herramienta bloqueada indefinidamente, porque nunca se envía `control_response` y los mensajes de permiso no tienen tiempo de espera.

La opción `requestId` y el valor de retorno `null` requieren Claude Code v2.1.199 o posterior.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Resultado de una verificación de permiso.

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

Configuración para el comportamiento de herramientas integradas.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Campo                           | Tipo                   | Descripción                                                                                                                                                                                                   |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Opte por el campo `preview` en las opciones de [`AskUserQuestion`](/docs/es/agent-sdk/user-input#question-format) y establezca su formato de contenido. Cuando no está establecido, Claude no emite vistas previas |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Configuración para servidores MCP.

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configuración para cargar plugins en el SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Campo              | Tipo      | Descripción                                                                                                                                                                                                                |
| :----------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Debe ser `'local'` (actualmente solo se soportan plugins locales)                                                                                                                                                          |
| `path`             | `string`  | Ruta absoluta o relativa al directorio del plugin                                                                                                                                                                          |
| `skipMcpDiscovery` | `boolean` | Cuando es `true`, el SDK carga skills, hooks, agentes y comandos de este plugin pero no lee su `.mcp.json` o manifest `mcpServers`. Establezca esto cuando su aplicación sea propietaria de las conexiones MCP del plugin. |

**Ejemplo:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Para información completa sobre la creación y uso de plugins, vea [Plugins](/docs/es/agent-sdk/plugins).

<h2 id="message-types">
  Tipos de Mensaje
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

Tipo de unión de todos los mensajes posibles devueltos por la consulta.

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

Mensaje de respuesta del asistente.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Del SDK de Anthropic
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

El campo `message` es un [`BetaMessage`](https://platform.claude.com/docs/es/api/messages/create) del SDK de Anthropic. Incluye campos como `id`, `content`, `model`, `stop_reason` y `usage`.

`SDKAssistantMessageError` es uno de: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, u `'unknown'`. `'model_not_found'` significa que el modelo seleccionado no existe o no está disponible para su cuenta o implementación. `'overloaded'` significa que la API devolvió un 529 porque el servidor está a capacidad, a diferencia de `'rate_limit'`, que es un 429 contra su cuota.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Mensaje de entrada del usuario.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Del SDK de Anthropic
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Establezca `shouldQuery` en `false` para añadir el mensaje a la transcripción sin activar un turno del asistente. El mensaje se mantiene y se fusiona en el siguiente mensaje de usuario que sí activa un turno. Use esto para inyectar contexto, como la salida de un comando que ejecutó fuera de banda, sin gastar una llamada de modelo en él.

En un mensaje que lleva un bloque `tool_result`, `tool_use_result` es el objeto de salida estructurada de la herramienta en lugar del texto enviado al modelo. Su forma depende de la herramienta nombrada por el bloque `tool_use` coincidente, por lo que el campo se escribe como `unknown`; las formas integradas se enumeran en [Tipos de Salida de Herramienta](#tool-output-types).

Para la herramienta `Agent`, `tool_use_result` es [`AgentOutput`](#agent-2). En un resultado `completed`, `content` contiene el informe del subagente sin el ID del agente y el remolque de uso que Claude Code añade al texto `tool_result`, así que renderice desde `tool_use_result` en lugar de analizar ese texto.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

Mensaje de usuario reproducido con UUID requerido.

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

Un turno de usuario inyectado desde fuera de la sesión, uno cuyo [`origin`](#sdkmessageorigin) es `peer` o `channel`, llega a la transmisión como una reproducción ya sea que se entregó durante un turno activo o inició un nuevo turno mientras la sesión estaba inactiva. Antes de v2.1.207, un turno inyectado entregado mientras la sesión estaba inactiva no producía ningún mensaje en la transmisión y solo aparecía cuando volvía a leer la transcripción.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

Mensaje de resultado final.

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

Varios campos en el resultado llevan detalles de diagnóstico más allá de `subtype`:

* `api_error_status`: el código de estado HTTP del error de API que terminó la conversación. Ausente o `null` cuando el turno terminó sin un error de API.
* `ttft_ms`: tiempo hasta el primer token en milisegundos, medido cuando llega el primer mensaje completo del asistente. Presente solo en el brazo de éxito.
* `ttft_stream_ms`: tiempo en milisegundos hasta el primer evento de transmisión `message_start`, cuando se abre la transmisión de respuesta. Menor que `ttft_ms`; la brecha entre los dos es el tiempo dedicado a transmitir el primer mensaje. Presente solo en el brazo de éxito.
* `terminal_reason`: por qué terminó el bucle. Uno de `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, o `"turn_setup_failed"`.
* `fast_mode_state`: uno de `"on"`, `"off"`, o `"cooldown"`.

El campo `origin` reenvía el [`SDKMessageOrigin`](#sdkmessageorigin) del mensaje de usuario que activó este resultado. Cuando una tarea de fondo finaliza y el SDK inyecta un turno de seguimiento sintético, el `SDKResultMessage` resultante lleva `origin: { kind: "task-notification" }`. Verifique este campo para distinguir los resultados que responden a su solicitud de los resultados emitidos para seguimientos de tareas de fondo, para que pueda enrutar o suprimir estos últimos. El campo está ausente para los resultados emitidos antes de cualquier turno de usuario, como errores de inicio.

Cuando un hook `PreToolUse` devuelve `permissionDecision: "defer"`, el resultado tiene `stop_reason: "tool_deferred"` y `deferred_tool_use` lleva el `id`, `name` e `input` de la herramienta pendiente. Lea este campo para mostrar la solicitud en su propia interfaz de usuario, luego reanude con el mismo `session_id` para continuar. Consulte [Diferir una llamada de herramienta para más tarde](/docs/es/hooks#defer-a-tool-call-for-later) para el viaje completo.

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

Mensaje de inicialización del sistema.

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

El array `capabilities` nombra los comportamientos de protocolo que esta CLI implementa, para que pueda detectar características en lugar de comparar cadenas `claude_code_version`. Es un conjunto abierto: ignore los valores que no reconozca y verifique la capacidad específica cuyo comportamiento depende. El campo requiere Claude Code v2.1.205 o posterior y está ausente en CLI anteriores.

| Capacidad              | Significado                                                                                                                                                                                 |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) se resuelve con una recepción [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) que nombra los mensajes en cola que sobreviven a la interrupción |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Mensaje parcial de transmisión (solo cuando `includePartialMessages` es true). El campo `parent_tool_use_id` siempre es `null`: los eventos de transmisión se emiten solo para la sesión principal. Para la atribución de subagentes, use mensajes completos, que llevan `parent_tool_use_id`, o habilite [`forwardSubagentText`](#options) para recibir texto y pensamiento de subagentes como mensajes completos.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Del SDK de Anthropic
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Tiempo hasta el primer token en ms, presente solo en eventos message_start
};
```

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

Mensaje que indica un límite de compactación de conversación.

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

Pancarta de texto genérica emitida por el bucle. Lleva líneas de estado sin error, retroalimentación de hooks como la razón de bloqueo de un hook `UserPromptSubmit`, y salida de comandos. Renderice `content` como texto sin formato en el `level` dado.

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

Se emite en el desmontaje elegante del worker para que los clientes remotos puedan mostrar por qué el worker desapareció en lugar de esperar el tiempo de espera del latido. El `reason` es una cadena corta en snake\_case establecida por la CLI del host, como `"host_exit"` o `"remote_control_disabled"`. Actúe sobre esto solo cuando transmita en vivo. Una sesión reanudada reproduce instancias pasadas de este mensaje, así que ignórelas en ese caso.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

Evento de progreso de instalación de plugin. Se emite cuando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/es/env-vars) está establecido, para que su aplicación Agent SDK pueda rastrear la instalación de plugins del mercado antes del primer turno. Los estados `started` y `completed` cierran la instalación general. Los estados `installed` y `failed` reportan mercados individuales e incluyen `name`.

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

Evento de transmisión emitido cuando el sistema de permisos deniega automáticamente una llamada de herramienta sin un aviso interactivo. Úselo para renderizar la denegación en su interfaz de usuario a medida que sucede, en lugar de solo observar el resultado de la herramienta `is_error` que sigue. La ruta de solicitud interactiva llega a su aplicación por separado a través de la devolución de llamada [`canUseTool`](#canusetool). Las denegaciones emitidas por un hook `PreToolUse` no se reportan a través de este evento.

Este evento requiere Claude Code v2.1.136 o posterior.

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| Campo                  | Tipo     | Descripción                                                                                                                                           |
| ---------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Nombre de la herramienta que fue denegada                                                                                                             |
| `tool_use_id`          | `string` | ID del bloque `tool_use` que esta denegación responde                                                                                                 |
| `agent_id`             | `string` | ID del subagente cuando la llamada denegada se originó dentro de un subagente. Refleja el campo en `can_use_tool` para enrutamiento del lado del host |
| `decision_reason_type` | `string` | Discriminador para el componente que decidió, como `"rule"`, `"mode"`, `"classifier"`, o `"asyncAgent"`                                               |
| `decision_reason`      | `string` | Razón legible por humanos del componente que decide, cuando está disponible                                                                           |
| `message`              | `string` | Mensaje de rechazo devuelto al modelo en el `tool_result`                                                                                             |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Información sobre un uso de herramienta denegado.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Procedencia de un mensaje con rol de usuario. Esto aparece como `origin` en [`SDKUserMessage`](#sdkusermessage) y se reenvía al [`SDKResultMessage`](#sdkresultmessage) correspondiente para que pueda saber qué activó un turno determinado.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | Significado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Entrada directa del usuario final. En mensajes de usuario, una `origin` ausente también significa entrada humana.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `channel`           | Mensaje que llega en un [canal](/docs/es/channels). `server` es el nombre del servidor MCP de origen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `peer`              | Mensaje de otro agente. Para un [compañero](/docs/es/agent-teams) en proceso enviando a `main` a través de `SendMessage`, `from` es el nombre del compañero y `senderTaskId` es su ID de tarea. Para un par entre sesiones como otro proceso local de Claude Code, `from` es la dirección del remitente y `senderTaskId` está ausente. `name` y `body` requieren Claude Code v2.1.205 o posterior. `name` es el nombre para mostrar del remitente, normalizado por Claude Code: elimina puntos de código de control, formato, sustituto, y separador de línea o párrafo Unicode, luego recorta el resultado y lo limita a 64 puntos de código con puntos suspensivos. `body` es el cuerpo del mensaje decodificado con la envoltura de par eliminada, byte exacto con lo que el modelo ve. Para un mensaje de compañero `body` siempre está presente; para un par entre sesiones está presente solo cuando el turno es exactamente una envoltura de par formada por Claude Code. Renderice `name` y `body` en lugar de volver a analizar el texto del mensaje. |
| `task-notification` | Turno sintético inyectado después de que finalizó una tarea de fondo. Consulte [`SDKTaskNotificationMessage`](#sdktasknotificationmessage).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `coordinator`       | Mensaje de un coordinador de equipo en un [equipo de agentes](/docs/es/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `auto-continuation` | Turno sintético inyectado cuando la sesión continúa sin entrada de usuario nueva, como un resultado de comando que activa un aviso de seguimiento.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

<h2 id="hook-types">
  Tipos de Hook
</h2>

Para una guía completa sobre el uso de hooks con ejemplos y patrones comunes, vea la [guía de Hooks](/docs/es/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Eventos de hook disponibles.

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

Tipo de función de devolución de llamada de hook.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // Unión de todos los tipos de entrada de hook
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

Configuración de hook con coincidencia opcional.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // Tiempo de espera en segundos para todos los hooks en este coincididor
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

Tipo de unión de todos los tipos de entrada de hook.

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Interfaz base que todos los tipos de entrada de hook extienden.

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

El campo `prompt_id` es un UUID que identifica el mensaje del usuario que se está procesando actualmente. Coincide con el [atributo `prompt.id` en eventos de OpenTelemetry](/docs/es/monitoring-usage#event-correlation-attributes) y está ausente hasta la primera entrada del usuario. Requiere Claude Code v2.1.196 o posterior.

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
};
```

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

Se activa una vez después de que cada llamada de herramienta en un lote se haya resuelto, antes de la siguiente solicitud del modelo. `tool_response` lleva el contenido serializado de `tool_result` que el modelo ve; la forma difiere del objeto `Output` estructurado de `PostToolUseHookInput`.

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // Cadena de matriz EXIT_REASONS
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated since v2.1.178. Carries the session-derived team name; will be removed. */
  team_name: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated since v2.1.178. Carries the session-derived team name; will be removed. */
  team_name?: string;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Valor de retorno de hook.

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Use `updatedToolOutput`, which works for all tools. */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      };
};
```

<h2 id="tool-input-types">
  Tipos de Entrada de Herramienta
</h2>

Documentación de esquemas de entrada para todas las herramientas integradas de Claude Code. Estos tipos se exportan desde `@anthropic-ai/claude-agent-sdk` y se pueden usar para interacciones de herramientas seguras de tipos.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Unión de todos los tipos de entrada de herramienta, exportados desde `@anthropic-ai/claude-agent-sdk`.

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nombre de herramienta:** `Agent` (anteriormente `Task`, que aún se acepta como alias)

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
};
```

Lanza un nuevo agente para manejar tareas complejas de múltiples pasos de forma autónoma.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nombre de herramienta:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
};
```

Hace preguntas aclaratorias al usuario durante la ejecución. Vea [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input#handle-clarifying-questions) para detalles de uso.

<h3 id="bash">
  Bash
</h3>

**Nombre de herramienta:** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

Ejecuta comandos bash en una sesión de shell persistente con tiempo de espera opcional y ejecución en segundo plano.

<h3 id="monitor">
  Monitor
</h3>

**Nombre de herramienta:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

Ejecuta una fuente de fondo y entrega cada evento a Claude para que pueda reaccionar sin sondeo: `command` ejecuta un script y emite un evento por línea de stdout, y `ws` abre un WebSocket y emite un evento por marco de texto. Proporcione exactamente uno de `command` o `ws`. La fuente `ws` requiere Claude Code v2.1.195 o posterior.

Establezca `persistent: true` para vigilancias de duración de sesión como colas de registro. Cuando Monitor ejecuta un comando, sigue las mismas reglas de permiso que Bash; una vigilancia de WebSocket solicita aprobación por separado. Vea la [referencia de herramienta Monitor](/docs/es/tools-reference#monitor-tool) para comportamiento y disponibilidad de proveedor.

<h3 id="taskoutput">
  TaskOutput
</h3>

**Nombre de herramienta:** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

Recupera salida de una tarea de fondo en ejecución o completada.

<h3 id="edit">
  Edit
</h3>

**Nombre de herramienta:** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

Realiza reemplazos de cadena exactos en archivos.

<h3 id="read">
  Read
</h3>

**Nombre de herramienta:** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

Lee archivos del sistema de archivos local, incluyendo texto, imágenes, PDFs y cuadernos Jupyter. Use `pages` para rangos de páginas PDF (por ejemplo, `"1-5"`).

<h3 id="write">
  Write
</h3>

**Nombre de herramienta:** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

Escribe un archivo en el sistema de archivos local, sobrescribiendo si existe.

<h3 id="glob">
  Glob
</h3>

**Nombre de herramienta:** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

Coincidencia de patrón de archivo rápida que funciona con cualquier tamaño de base de código.

<h3 id="grep">
  Grep
</h3>

**Nombre de herramienta:** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

Herramienta de búsqueda poderosa construida en ripgrep con soporte de expresiones regulares.

<h3 id="taskstop">
  TaskStop
</h3>

**Nombre de herramienta:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Deprecado: use task_id
};
```

Detiene una tarea de fondo en ejecución o shell por ID. A partir de v2.1.198, `task_id` también acepta un compañero de equipo de agentes o un agente de fondo nombrado por ID de agente o nombre.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nombre de herramienta:** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Edita celdas en archivos de cuaderno Jupyter.

<h3 id="webfetch">
  WebFetch
</h3>

**Nombre de herramienta:** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

Obtiene contenido de una URL y lo procesa con un modelo de IA.

<h3 id="websearch">
  WebSearch
</h3>

**Nombre de herramienta:** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

Busca en la web y devuelve resultados formateados.

<h3 id="workflow">
  Workflow
</h3>

**Nombre de herramienta:** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown;
  resumeFromRunId?: string;
};
```

Ejecuta un [flujo de trabajo dinámico](/docs/es/workflows): un script que orquesta muchos subagentes en segundo plano y devuelve un resultado consolidado. La herramienta `Workflow` está disponible en Agent SDK v0.3.149 y posterior. Se requiere al menos uno de `script`, `name` o `scriptPath`.

| Campo             | Tipo      | Descripción                                                                                                                                                                                                                                                                                                     |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Script de flujo de trabajo en línea. Debe comenzar con `export const meta = { name, description }` como un literal, seguido del cuerpo del script usando `agent()`, `parallel()`, `pipeline()` y `phase()`. Una matriz `phases` opcional en `meta` agrupa agentes bajo etapas nombradas en la vista de progreso |
| `name`            | `string`  | Nombre de un flujo de trabajo integrado o uno guardado en `.claude/workflows/`. Se resuelve a un script                                                                                                                                                                                                         |
| `scriptPath`      | `string`  | Ruta a un archivo de script de flujo de trabajo en disco. Tiene precedencia sobre `script` y `name`. Cada invocación persiste su script y devuelve la ruta en el resultado, para que pueda editar ese archivo e invocar nuevamente con el mismo `scriptPath` para iterar                                        |
| `args`            | `unknown` | Valor de entrada expuesto al script como el `args` global, para flujos de trabajo nombrados parametrizados como una pregunta de investigación o una lista de rutas de archivo. Pase matrices y objetos como valores JSON reales, no como una cadena codificada en JSON                                          |
| `resumeFromRunId` | `string`  | ID de ejecución de una invocación anterior de `Workflow` para reanudar. Las llamadas `agent()` completadas con entradas sin cambios devuelven resultados en caché; solo las llamadas cambiadas o nuevas se ejecutan en vivo. Solo la misma sesión                                                               |

<h3 id="todowrite">
  TodoWrite
</h3>

**Nombre de herramienta:** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Crea y gestiona una lista de tareas estructurada para rastrear el progreso.

<Note>
  A partir de TypeScript Agent SDK 0.3.142, `TodoWrite` está deshabilitado de forma predeterminada. Use `TaskCreate`, `TaskGet`, `TaskUpdate` y `TaskList` en su lugar. Vea [Migrar a herramientas Task](/docs/es/agent-sdk/todo-tracking#migrate-to-task-tools) para actualizar su código de monitoreo, o establezca `CLAUDE_CODE_ENABLE_TASKS=0` para revertir a `TodoWrite`.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nombre de herramienta:** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

Crea una única tarea y devuelve su ID asignado.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nombre de herramienta:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

Parcha una tarea por ID. Establezca `status` a `"deleted"` para eliminarla.

<h3 id="taskget">
  TaskGet
</h3>

**Nombre de herramienta:** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

Devuelve detalles completos para una tarea, o `null` cuando el ID no se encuentra.

<h3 id="tasklist">
  TaskList
</h3>

**Nombre de herramienta:** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

Devuelve una instantánea de todas las tareas en la lista actual.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nombre de herramienta:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Deprecated: no longer used. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
};
```

Sale del modo de planificación. El campo `allowedPrompts` está deprecado e ignorado; Claude Code aún lo acepta para que los llamadores existentes y las transcripciones se validen. Antes de v2.1.205, solicitaba permisos de Bash basados en mensajes para implementar el plan.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nombre de herramienta:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Enumera recursos MCP disponibles de servidores conectados.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nombre de herramienta:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

Lee un recurso MCP específico de un servidor.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**Nombre de herramienta:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

Crea e ingresa a un worktree git temporal para trabajo aislado. Pase `path` para cambiar a un worktree existente en lugar de crear uno nuevo. En la primera entrada, el destino debe ser un worktree registrado del repositorio actual o, en un espacio de trabajo de múltiples repositorios, de un repositorio anidado dentro de él; desde dentro de una sesión de worktree debe estar bajo `.claude/worktrees/` del repositorio de la sesión. `name` y `path` son mutuamente excluyentes.

<h2 id="tool-output-types">
  Tipos de Salida de Herramienta
</h2>

Documentación de esquemas de salida para todas las herramientas integradas de Claude Code. Estos tipos se exportan desde `@anthropic-ai/claude-agent-sdk` y representan los datos de respuesta reales devueltos por cada herramienta.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Unión de todos los tipos de salida de herramienta.

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-2">
  Agent
</h3>

**Nombre de herramienta:** `Agent` (anteriormente `Task`, que aún se acepta como alias)

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

Devuelve el resultado del subagente. Discriminado en el campo `status`: `"completed"` para tareas terminadas, `"async_launched"` para tareas de fondo, y `"remote_launched"` para tareas que Claude Code envió a una sesión en la nube remota, donde `sessionUrl` vincula a esa sesión e `taskId` la identifica.

El campo `resolvedModel` en las variantes `completed` y `async_launched` nombra el modelo en el que el subagente realmente se ejecutó, que puede diferir del modelo `model` solicitado cuando [`availableModels`](/docs/es/model-config#restrict-model-selection) u otra anulación se aplica. Este campo requiere Claude Code v2.1.174 o posterior.

En la variante `completed`, `worktreePath` se establece cuando el subagente se ejecutó en un worktree git aislado, y `worktreeBranch` nombra la rama de ese worktree cuando Claude Code la creó. `usage.service_tier` lleva la cadena de nivel de servicio que la API reportó para las solicitudes del subagente.

Antes de v2.1.207, el tipo publicado era más estrecho. Omitía `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount`, y los campos de uso `inference_geo`, `speed`, e `iterations`, y escribía `service_tier` como `"standard" | "priority" | "batch"`. Los campos que el tipo marca como opcionales pueden estar ausentes en los resultados registrados por versiones anteriores.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**Nombre de herramienta:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
};
```

Devuelve las preguntas hechas y las respuestas del usuario. `response` se establece cuando el usuario escribió una respuesta de forma libre en lugar de responder las preguntas estructuradas; cuando está presente, Claude recibe "El usuario respondió: …" en lugar de la lista de respuestas por pregunta.

<h3 id="bash-2">
  Bash
</h3>

**Nombre de herramienta:** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

Devuelve la salida del comando con stdout/stderr divididos. Los comandos de fondo incluyen un `backgroundTaskId`.

<h3 id="monitor-2">
  Monitor
</h3>

**Nombre de herramienta:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Devuelve el ID de tarea de fondo para el monitor en ejecución. Use este ID con `TaskStop` para cancelar la vigilancia temprano.

<h3 id="edit-2">
  Edit
</h3>

**Nombre de herramienta:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Devuelve el diff estructurado de la operación de edición.

<h3 id="read-2">
  Read
</h3>

**Nombre de herramienta:** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
    };
```

Devuelve el contenido del archivo en un formato apropiado para el tipo de archivo. Discriminado en el campo `type`.

<h3 id="write-2">
  Write
</h3>

**Nombre de herramienta:** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Devuelve el resultado de escritura con información de diff estructurado.

<h3 id="glob-2">
  Glob
</h3>

**Nombre de herramienta:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
};
```

Devuelve rutas de archivo que coinciden con el patrón glob, ordenadas por hora de modificación.

<h3 id="grep-2">
  Grep
</h3>

**Nombre de herramienta:** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Devuelve resultados de búsqueda. La forma varía por `mode`: lista de archivos, contenido con coincidencias o conteos de coincidencias.

<h3 id="taskstop-2">
  TaskStop
</h3>

**Nombre de herramienta:** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

Devuelve confirmación después de detener la tarea de fondo.

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**Nombre de herramienta:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

Devuelve el resultado de la edición del cuaderno con contenido de archivo original y actualizado.

<h3 id="webfetch-2">
  WebFetch
</h3>

**Nombre de herramienta:** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
};
```

Devuelve el contenido obtenido con estado HTTP y metadatos.

<h3 id="websearch-2">
  WebSearch
</h3>

**Nombre de herramienta:** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
};
```

Devuelve resultados de búsqueda de la web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nombre de herramienta:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

Devuelve inmediatamente después de que la herramienta acepta la invocación. El resultado final llega más tarde como una finalización de tarea. Verifique `error` antes de tratar la ejecución como iniciada: un script que falla su verificación de sintaxis devuelve `status: "async_launched"` con `error` establecido, y nunca se ejecuta.

| Campo           | Tipo               | Descripción                                                                                                                                          |
| --------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched"` | La herramienta aceptó la invocación. Este es el único valor que toma el campo                                                                        |
| `taskId`        | `string`           | Identificador de tarea de fondo para la ejecución                                                                                                    |
| `runId`         | `string`           | Identificador de ejecución de flujo de trabajo para pasar como `resumeFromRunId` en una invocación posterior                                         |
| `summary`       | `string`           | Descripción de una línea de lo que hace el flujo de trabajo                                                                                          |
| `transcriptDir` | `string`           | Directorio donde se escriben las transcripciones de subagentes durante la ejecución                                                                  |
| `scriptPath`    | `string`           | Ruta al script de flujo de trabajo persistido para esta ejecución. Edítelo y páselo como `scriptPath` para volver a ejecutar sin reenviar el script  |
| `error`         | `string`           | Se establece cuando el script falla su verificación de sintaxis. Cuando está presente, la ejecución no se inició a pesar del estado `async_launched` |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**Nombre de herramienta:** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Devuelve las listas de tareas anteriores y actualizadas.

<Note>
  A partir de TypeScript Agent SDK 0.3.142, `TodoWrite` está deshabilitado de forma predeterminada. Use `TaskCreate`, `TaskGet`, `TaskUpdate`, y `TaskList` en su lugar. Consulte [Migrar a herramientas de tareas](/docs/es/agent-sdk/todo-tracking#migrate-to-task-tools) para actualizar su código de monitoreo, o establezca `CLAUDE_CODE_ENABLE_TASKS=0` para revertir a `TodoWrite`.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**Nombre de herramienta:** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

Devuelve la tarea creada con su ID asignado.

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**Nombre de herramienta:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

Devuelve el resultado de la actualización, incluyendo qué campos cambiaron.

<h3 id="taskget-2">
  TaskGet
</h3>

**Nombre de herramienta:** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

Devuelve el registro de tarea completo, o `null` cuando el ID no se encuentra.

<h3 id="tasklist-2">
  TaskList
</h3>

**Nombre de herramienta:** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

Devuelve una instantánea de todas las tareas en la lista actual.

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**Nombre de herramienta:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Devuelve el estado del plan después de salir del modo de planificación.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**Nombre de herramienta:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Devuelve una matriz de recursos MCP disponibles.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Nombre de herramienta:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
  }>;
};
```

Devuelve el contenido del recurso MCP solicitado.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**Nombre de herramienta:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Devuelve información sobre el worktree git.

<h2 id="permission-types">
  Tipos de Permiso
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Operaciones para actualizar permisos.

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // Configuración global del usuario
  | "projectSettings" // Configuración del proyecto por directorio
  | "localSettings" // Configuración local del proyecto
  | "session" // Solo sesión actual
  | "cliArg"; // Argumento CLI
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  Otros Tipos
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Características beta disponibles que se pueden habilitar a través de la opción `betas`. Vea [Encabezados Beta](https://platform.claude.com/docs/es/api/beta-headers) para más información.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  La beta `context-1m-2025-08-07` se retiró a partir del 30 de abril de 2026. Pasar este valor con Claude Sonnet 4.5 o Sonnet 4 no tiene efecto, y las solicitudes que excedan la ventana de contexto estándar de 200k tokens devuelven un error. Para usar una ventana de contexto de 1M tokens, migre a [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 o Claude Opus 4.8](https://platform.claude.com/docs/es/about-claude/models/overview), que incluyen contexto de 1M a precios estándar sin encabezado beta requerido.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Información sobre un comando slash disponible.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

<h3 id="modelinfo">
  `ModelInfo`
</h3>

Información sobre un modelo disponible.

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| Campo                      | Tipo                                                               | Descripción                                                                                                                                                                                                                                                                                                                        |
| :------------------------- | :----------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | Identificador de modelo para pasar en llamadas API                                                                                                                                                                                                                                                                                 |
| `resolvedModel`            | `string \| undefined`                                              | ID de modelo canónico que el `value` de esta entrada se resuelve a. Una entrada de alias como `sonnet` se resuelve a un ID de modelo explícito como `claude-sonnet-5`, por lo que un host puede coincidir un ID de modelo explícito almacenado contra la entrada de alias que lo cubre. Requiere Claude Code v2.1.197 o posterior. |
| `displayName`              | `string`                                                           | Nombre de visualización legible para humanos                                                                                                                                                                                                                                                                                       |
| `description`              | `string`                                                           | Descripción de las capacidades del modelo                                                                                                                                                                                                                                                                                          |
| `supportsEffort`           | `boolean \| undefined`                                             | Si este modelo admite niveles de esfuerzo                                                                                                                                                                                                                                                                                          |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Niveles de esfuerzo que este modelo acepta                                                                                                                                                                                                                                                                                         |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Si este modelo admite pensamiento adaptativo, donde Claude decide cuándo y cuánto pensar                                                                                                                                                                                                                                           |
| `supportsFastMode`         | `boolean \| undefined`                                             | Si este modelo admite modo rápido                                                                                                                                                                                                                                                                                                  |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Si este modelo admite modo automático                                                                                                                                                                                                                                                                                              |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Información sobre un subagente disponible que se puede invocar a través de la herramienta Agent.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| Campo         | Tipo                  | Descripción                                                                     |
| :------------ | :-------------------- | :------------------------------------------------------------------------------ |
| `name`        | `string`              | Identificador de tipo de agente (por ejemplo, `"Explore"`, `"general-purpose"`) |
| `description` | `string`              | Descripción de cuándo usar este agente                                          |
| `model`       | `string \| undefined` | Alias de modelo que usa este agente. Si se omite, hereda el modelo del padre    |

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Estado de un servidor MCP conectado.

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

La configuración de un servidor MCP como se reporta por `mcpServerStatus()`. Esta es la unión de todos los tipos de transporte de servidor MCP.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

Vea [`McpServerConfig`](#mcpserverconfig) para detalles sobre cada tipo de transporte.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

Información de cuenta para el usuario autenticado.

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

Estadísticas de uso por modelo devueltas en mensajes de resultado. El valor `costUSD` es una estimación del lado del cliente. Vea [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking) para advertencias de facturación.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Una versión de [`Usage`](#usage) con todos los campos anulables hechos no anulables.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Estadísticas de uso de tokens. Este es el tipo `BetaUsage` de `@anthropic-ai/sdk`.

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
};
```

`BetaServerToolUsage` y `BetaIterationsUsage` se definen en `@anthropic-ai/sdk`.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

Tipo de resultado de herramienta MCP (desde `@modelcontextprotocol/sdk/types.js`). `structuredContent` es un objeto JSON que se puede devolver junto con `content`, incluyendo bloques de imagen. Vea [Devolver datos estructurados](/docs/es/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Los campos adicionales varían por tipo
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controla el comportamiento de pensamiento/razonamiento de Claude. Tiene precedencia sobre el `maxThinkingTokens` deprecado.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // El modelo determina cuándo y cuánto razonar (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Presupuesto de token de pensamiento fijo
  | { type: "disabled" }; // Sin pensamiento extendido
```

El campo `display` opcional controla si el texto de pensamiento se devuelve `"summarized"` u `"omitted"`. En Claude Opus 4.7 y posterior, el valor predeterminado de la API es `"omitted"`, así que establezca `"summarized"` para recibir contenido de pensamiento en bloques `thinking`.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

Interfaz para generación de proceso personalizado (usada con la opción `spawnClaudeCodeProcess`). `ChildProcess` ya satisface esta interfaz.

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

Opciones pasadas a la función de generación personalizada.

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  El campo `signal` le indica a su función de generación cuándo desmantelar el proceso. Páselo como la opción `signal` al `spawn()` de Node, o páselo a su controlador de desmontaje de VM o contenedor.

  Esta señal no se activa en el instante en que [`Options.abortController`](#options) se aborta. El SDK primero cierra la entrada estándar del proceso y espera aproximadamente dos segundos para que la CLI se apague limpiamente, luego aborta esta señal. Para reaccionar en el momento en que la persona que llama aborta, en su lugar escuche en su propio `Options.abortController.signal`, que su función de generación puede referenciar desde su alcance envolvente.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

Resultado de una operación `setMcpServers()`.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

Resultado de una operación `rewindFiles()`.

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
};
```

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Mensaje de actualización de estado (por ejemplo, compactación).

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

Notificación cuando una tarea de fondo se completa, falla o se detiene. Las tareas de fondo incluyen comandos Bash `run_in_background`, vigilancias [Monitor](#monitor) y subagentes de fondo.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

Resumen del uso de herramientas en una conversación.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

Se emite cuando un hook comienza a ejecutarse.

Claude Code entrega este mensaje, [`SDKHookProgressMessage`](#sdkhookprogressmessage) y [`SDKHookResponseMessage`](#sdkhookresponsemessage) al flujo de mensajes inmediatamente, incluso mientras un hook `SessionStart` o `Setup` aún se está ejecutando durante el inicio de sesión. Claude Code v2.1.169 a v2.1.203 entregó estos mensajes en un lote después de que un hook `SessionStart` o `Setup` se completó; v2.1.204 restauró la entrega en vivo.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

Se emite mientras un hook se está ejecutando, con salida de stdout/stderr.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

Se emite cuando un hook termina de ejecutarse.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

Se emite periódicamente mientras se ejecuta una herramienta para indicar progreso.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

Se emite durante flujos de autenticación.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

Se emite cuando comienza una tarea de fondo. El campo `task_type` es `"local_bash"` para comandos Bash de fondo y vigilancias [Monitor](#monitor), `"local_agent"` para subagentes, o `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Se emite periódicamente mientras se ejecuta un subagente o tarea de fondo. El campo `summary` se completa solo cuando [`agentProgressSummaries`](#options) está habilitado.

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

Se emite cuando el estado de una tarea de fondo cambia, como cuando transiciona de `running` a `completed`. Combine `patch` en su mapa de tareas local con clave `task_id`. El campo `end_time` es una marca de tiempo de época Unix en milisegundos, comparable con `Date.now()`.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

Se emite siempre que el conjunto de tareas de fondo activas cambia: una tarea comienza, se completa, se mata, o un agente en primer plano se pone en segundo plano. El array `tasks` es el conjunto completo activo. Reemplace cualquier conjunto en caché con cada carga útil en lugar de emparejar eventos `task_started` y `task_notification`, para que el siguiente cambio de membresía corrija cualquier evento que haya perdido.

El orden relativo a esos eventos por tarea no está especificado, así que no correlacione los dos flujos.

Nada se emite al inicio. Reinicie a un conjunto vacío siempre que el proceso CLI de la sesión comience o se reinicie y deje que el siguiente cambio de membresía lo repuele.

Requiere Claude Code v2.1.203 o posterior.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Se emite mientras Claude está produciendo un bloque de pensamiento, incluyendo uno redactado, llevando una estimación en ejecución de los tokens de pensamiento generados hasta ahora. `estimated_tokens` es el total en ejecución para el bloque de pensamiento actual y `estimated_tokens_delta` es el incremento llevado por este fotograma. Úselo para visualización de progreso. El recuento final para el bucle de agente de nivel superior es el `usage.output_tokens` del mensaje de resultado, que [no incluye tokens de subagente](/docs/es/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); use [`modelUsage`](#modelusage) para contabilidad de árbol completo.

Requiere Claude Code v2.1.153 o posterior.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

Se emite cuando los puntos de control de archivo se persisten en el disco.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

Se emite cuando la sesión encuentra un límite de velocidad.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

Cuando `errorCode` es `"credits_required"`, el rechazo proviene de una suscripción de claude.ai cuyo uso incluido se ha agotado, y la sesión no puede continuar hasta que el usuario compre créditos de uso. `canUserPurchaseCredits` indica si el usuario autenticado puede comprar créditos para la cuenta, y `hasChargeableSavedPaymentMethod` indica si hay un método de pago guardado en el archivo. Los tres campos están ausentes en eventos de límite de velocidad que no son rechazos de créditos requeridos. Requiere Claude Code v2.1.181 o posterior.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Salida de un comando slash local (por ejemplo, `/voice` o `/usage`). Se muestra como texto de estilo asistente en la transcripción.

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

Se emite cuando el conjunto de comandos disponibles cambia a mitad de sesión, como cuando se descubren skills al entrar en un subdirectorio. El array `commands` es la lista completa actualizada, así que reemplace cualquier lista de comandos en caché con esta carga útil. Llamar a `supportedCommands()` nuevamente no es equivalente: ese método devuelve la instantánea capturada en la inicialización y no refleja cambios a mitad de sesión.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

Se emite después de cada turno cuando `promptSuggestions` está habilitado. Contiene un mensaje de usuario predicho siguiente.

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

Se emite cuando la conversación de la sesión se reemplaza sin terminar la sesión, como después de `/clear`, al salir del modo plan, o cuando comienza una conversación nueva. Monte una transcripción vacía bajo `new_conversation_id` y descarte cualquier título de sesión en caché.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

Las tipificaciones publicadas del SDK declaran `SDKConversationResetMessage` en Claude Code v2.1.203 y posterior. Antes de v2.1.203, `SDKMessage` hacía referencia al tipo sin declararlo, por lo que el estrechamiento en `type === "conversation_reset"` no pasaba la verificación de tipos cuando `skipLibCheck` estaba deshabilitado.

<h3 id="aborterror">
  `AbortError`
</h3>

Clase de error personalizado para operaciones de aborto.

```typescript theme={null}
class AbortError extends Error {}
```

<h2 id="sandbox-configuration">
  Configuración de Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuración para el comportamiento de sandbox. Use esto para habilitar el sandboxing de comandos y configurar restricciones de red mediante programación.

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| Propiedad                   | Tipo                                                  | Predeterminado | Descripción                                                                                                                                                                                                                                                       |
| :-------------------------- | :---------------------------------------------------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`        | Habilite el modo sandbox para la ejecución de comandos                                                                                                                                                                                                            |
| `failIfUnavailable`         | `boolean`                                             | `true`         | Deténgase al inicio si `enabled` es `true` pero el sandbox no puede iniciarse. Establezca `false` para recurrir a la ejecución sin sandbox con una advertencia en stderr                                                                                          |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`         | Auto-apruebe comandos bash cuando el sandbox está habilitado                                                                                                                                                                                                      |
| `excludedCommands`          | `string[]`                                            | `[]`           | Comandos que siempre omiten restricciones de sandbox (por ejemplo, `['docker']`). Estos se ejecutan sin sandbox automáticamente sin participación del modelo                                                                                                      |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`         | Permita que el modelo solicite ejecutar comandos fuera del sandbox. Cuando es `true`, el modelo puede establecer `dangerouslyDisableSandbox` en la entrada de herramienta, que se vuelve al [sistema de permisos](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined`    | Configuración de sandbox específica de red                                                                                                                                                                                                                        |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined`    | Configuración de sandbox específica del sistema de archivos para restricciones de lectura/escritura                                                                                                                                                               |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined`    | Mapa de categorías de violación a patrones a ignorar (por ejemplo, `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                                                                                |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`        | Habilite un sandbox anidado más débil para compatibilidad                                                                                                                                                                                                         |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined`    | Configuración de binario ripgrep personalizado para entornos sandbox                                                                                                                                                                                              |

<Note>
  El sandbox depende de la compatibilidad de la plataforma y, en Linux, de herramientas como `bubblewrap` y `socat`. Cuando `enabled` es `true` y el sandbox no puede iniciarse, `query()` reporta un mensaje `result` con `subtype: "error_during_execution"` y la razón en `errors`. Para una única llamada a `query()`, el SDK lanza después de ceder ese resultado de error, así que envuelva el bucle en un bloque try para continuar más allá. Consulte [Manejar el resultado](/docs/es/agent-sdk/agent-loop#handle-the-result) para el contrato de error.

  Para ejecutar sin sandbox en su lugar, establezca `failIfUnavailable: false`.
</Note>

<h4 id="example-usage">
  Ejemplo de uso
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Seguridad de socket Unix:** La opción `allowUnixSockets` puede otorgar acceso a servicios del sistema poderosos. Por ejemplo, permitir `/var/run/docker.sock` efectivamente otorga acceso completo al sistema host a través de la API de Docker, omitiendo el aislamiento de sandbox. Solo permita sockets Unix que sean estrictamente necesarios y comprenda las implicaciones de seguridad de cada uno.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuración específica de red para el modo sandbox. Estas configuraciones se aplican a comandos Bash en sandbox cuando `enabled` es `true` en la [`SandboxSettings`](#sandboxsettings) principal. No restringen la herramienta WebFetch, que utiliza [reglas de permisos](/docs/es/permissions#webfetch) en su lugar.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Propiedad                 | Tipo       | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                         |
| :------------------------ | :--------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`           | Nombres de dominio a los que los procesos en sandbox pueden acceder                                                                                                                                                                                                                                                                                 |
| `deniedDomains`           | `string[]` | `[]`           | Nombres de dominio a los que los procesos en sandbox no pueden acceder. Tiene prioridad sobre `allowedDomains`                                                                                                                                                                                                                                      |
| `allowManagedDomainsOnly` | `boolean`  | `false`        | Solo configuración administrada. Cuando se establece en [configuración administrada](/docs/es/permissions#managed-settings), solo se respetan las entradas `allowedDomains` de la configuración administrada y se ignoran las entradas de la configuración de usuario, proyecto o local. No tiene efecto cuando se establece a través de opciones de SDK |
| `allowLocalBinding`       | `boolean`  | `false`        | Permita que los procesos se vinculen a puertos locales (por ejemplo, para servidores de desarrollo)                                                                                                                                                                                                                                                 |
| `allowUnixSockets`        | `string[]` | `[]`           | Rutas de socket Unix a las que los procesos pueden acceder (por ejemplo, socket de Docker)                                                                                                                                                                                                                                                          |
| `allowAllUnixSockets`     | `boolean`  | `false`        | Permita el acceso a todos los sockets Unix                                                                                                                                                                                                                                                                                                          |
| `httpProxyPort`           | `number`   | `undefined`    | Puerto proxy HTTP para solicitudes de red                                                                                                                                                                                                                                                                                                           |
| `socksProxyPort`          | `number`   | `undefined`    | Puerto proxy SOCKS para solicitudes de red                                                                                                                                                                                                                                                                                                          |

<Note>
  El proxy de sandbox integrado aplica `allowedDomains` basándose en el nombre de host solicitado y no termina ni inspecciona el tráfico TLS, por lo que técnicas como [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) potencialmente pueden omitirlo. Consulte [Limitaciones de seguridad de sandboxing](/docs/es/sandboxing#security-limitations) para obtener detalles y [Implementación segura](/docs/es/agent-sdk/secure-deployment#traffic-forwarding) para configurar un proxy que termine TLS.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Configuración específica del sistema de archivos para el modo sandbox.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| Propiedad    | Tipo       | Predeterminado | Descripción                                                     |
| :----------- | :--------- | :------------- | :-------------------------------------------------------------- |
| `allowWrite` | `string[]` | `[]`           | Patrones de ruta de archivo para permitir acceso de escritura a |
| `denyWrite`  | `string[]` | `[]`           | Patrones de ruta de archivo para negar acceso de escritura a    |
| `denyRead`   | `string[]` | `[]`           | Patrones de ruta de archivo para negar acceso de lectura a      |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback de Permisos para Comandos Sin Sandbox
</h3>

Cuando `allowUnsandboxedCommands` está habilitado, el modelo puede solicitar ejecutar comandos fuera del sandbox estableciendo `dangerouslyDisableSandbox: true` en la entrada de herramienta. Estas solicitudes se vuelven al sistema de permisos existente, lo que significa que se invoca su controlador `canUseTool`, permitiéndole implementar lógica de autorización personalizada. En el ejemplo siguiente, `isCommandAuthorized` representa una verificación de autorización que usted define.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Una lista estática de comandos que siempre omiten el sandbox automáticamente (por ejemplo, `['docker']`). El modelo no tiene control sobre esto.
  * `allowUnsandboxedCommands`: Permite que el modelo decida en tiempo de ejecución si solicitar ejecución sin sandbox estableciendo `dangerouslyDisableSandbox: true` en la entrada de herramienta.
</Note>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // El modelo puede solicitar ejecución sin sandbox
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Verifique si el modelo está solicitando omitir el sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // El modelo está solicitando ejecutar este comando fuera del sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

Este patrón le permite:

* **Auditar solicitudes del modelo:** Registre cuándo el modelo solicita ejecución sin sandbox
* **Implementar listas de permitidos:** Solo permita comandos específicos para ejecutarse sin sandbox
* **Agregar flujos de trabajo de aprobación:** Requiera autorización explícita para operaciones privilegiadas

<Warning>
  Los comandos que se ejecutan con `dangerouslyDisableSandbox: true` tienen acceso completo al sistema. Asegúrese de que su controlador `canUseTool` valide estas solicitudes cuidadosamente.

  Si `permissionMode` se establece en `bypassPermissions` y `allowUnsandboxedCommands` está habilitado, el modelo puede ejecutar autónomamente comandos fuera del sandbox sin solicitudes de aprobación (una [regla `ask`](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) explícita aún fuerza una). Esta combinación efectivamente permite que el modelo escape del aislamiento de sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Ver también
</h2>

* [Descripción general del SDK](/docs/es/agent-sdk/overview) - Conceptos generales del SDK
* [Referencia del SDK de Python](/docs/es/agent-sdk/python) - Documentación del SDK de Python
* [Referencia de CLI](/docs/es/cli-reference) - Interfaz de línea de comandos
* [Flujos de trabajo comunes](/docs/es/common-workflows) - Guías paso a paso
