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
  El SDK incluye un binario nativo de Claude Code para su plataforma como una dependencia opcional como `@anthropic-ai/claude-agent-sdk-darwin-arm64`. La mayoría de las instalaciones no necesitan una instalación separada de Claude Code. La versión del SDK rastrea la versión del Claude Code incluida. SDK v0.3.191 incluye Claude Code v2.1.191, por lo que una característica en esta página que requiere una versión de Claude Code necesita la versión del SDK con el mismo número de parche o posterior. Si su gestor de paquetes omite las dependencias opcionales, el SDK lanza `Native CLI binary for <platform>-<arch> not found`; en su lugar, establezca [`pathToClaudeCodeExecutable`](#options) en un binario `claude` instalado por separado.

  Si su gestor de paquetes no aplica el campo `libc` de npm, como Yarn 1.x no lo hace, obtiene tanto los paquetes de plataforma glibc como musl en Linux, aproximadamente duplicando el tamaño de instalación. En Agent SDK v0.2.141 o posterior, el SDK aún lanza la variante correcta. Para recuperar el espacio en una imagen de contenedor, elimine el paquete de plataforma que no coincida con el libc donde se ejecuta su aplicación; para un tiempo de ejecución glibc en x64, eso es `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`. En una máquina de desarrollo la eliminación es temporal, ya que Yarn reinstala el paquete en el siguiente cambio de dependencia.
</Note>

<h3 id="compile-to-a-single-executable">
  Compilar a un ejecutable único
</h3>

Cuando compila su aplicación en un ejecutable de un solo archivo con `bun build --compile`, el SDK no puede resolver el binario CLI incluido en tiempo de ejecución. `require.resolve` no funciona dentro del sistema de archivos virtual `$bunfs` del ejecutable compilado, por lo que el SDK lanza `Native CLI binary for <platform>-<arch> not found`.

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
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Parámetros
</h4>

| Parámetro     | Tipo                                                                                                   | Descripción                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------ | :----------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | El nombre de la herramienta                                                                                                                                                                                                                                                                                                                                                                          |
| `description` | `string`                                                                                               | Una descripción de lo que hace la herramienta                                                                                                                                                                                                                                                                                                                                                        |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | Esquema Zod que define los parámetros de entrada de la herramienta (soporta tanto Zod 3 como Zod 4)                                                                                                                                                                                                                                                                                                  |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | Función asincrónica que ejecuta la lógica de la herramienta                                                                                                                                                                                                                                                                                                                                          |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | Extras opcionales. `annotations` proporciona sugerencias de comportamiento MCP a los clientes. `searchHint` es una frase de capacidad de una línea que se muestra en la lista de herramientas diferidas cuando la [búsqueda de herramientas](/docs/es/agent-sdk/tool-search) está activa. `alwaysLoad: true` mantiene el esquema completo de esta herramienta en el mensaje inicial en lugar de diferirlo |

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
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parámetros
</h4>

| Parámetro              | Tipo                          | Descripción                                                                                                                                                                                                                                                                                             |
| :--------------------- | :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.name`         | `string`                      | El nombre del servidor MCP                                                                                                                                                                                                                                                                              |
| `options.version`      | `string`                      | Cadena de versión opcional                                                                                                                                                                                                                                                                              |
| `options.instructions` | `string`                      | Instrucciones del servidor opcionales, devueltas desde `initialize` y expuestas al modelo como un bloque de instrucciones MCP                                                                                                                                                                           |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | Matriz de definiciones de herramientas creadas con [`tool()`](#tool)                                                                                                                                                                                                                                    |
| `options.alwaysLoad`   | `boolean`                     | Cuando es `true`, cada herramienta de este servidor permanece en el mensaje inicial y nunca se difiere detrás de la [búsqueda de herramientas](/docs/es/agent-sdk/tool-search). Se combina con `alwaysLoad` por herramienta en [`tool()`](#tool)                                                             |
| `options.timeout`      | `number`                      | Tiempo de espera en milisegundos para las llamadas de herramientas de este servidor. Claude Code lo aplica a este servidor en lugar de [`MCP_TOOL_TIMEOUT`](/docs/es/env-vars). Pase un número entero de al menos 1000. Claude Code ignora otros valores. Requiere TypeScript Agent SDK v0.3.248 o posterior |

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

| Propiedad            | Tipo                    | Descripción                                                                                                                                                                                                                                                                                      |
| :------------------- | :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Rol del mensaje                                                                                                                                                                                                                                                                                  |
| `uuid`               | `string`                | Identificador de mensaje único                                                                                                                                                                                                                                                                   |
| `session_id`         | `string`                | Sesión a la que pertenece este mensaje                                                                                                                                                                                                                                                           |
| `message`            | `unknown`               | Carga útil de mensaje sin procesar de la transcripción                                                                                                                                                                                                                                           |
| `parent_tool_use_id` | `string \| null`        | Para mensajes de subagente, el `tool_use_id` de la llamada de herramienta `Agent` o `Skill` que inició el subagente. `null` para mensajes de sesión principal y sesiones más antiguas                                                                                                            |
| `parent_agent_id`    | `string \| null`        | Para mensajes de un [subagente anidado](/docs/es/sub-agents#let-subagents-spawn-their-own-subagents), el `agentId` del subagente que lo generó. `null` para mensajes de sesión principal, mensajes de subagentes de nivel superior y sesiones más antiguas. Requiere Claude Code v2.1.202 o posterior |

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
  Esta función es alfa y su API puede cambiar antes de la estabilización.
</Note>

La instantánea difiere de lo que una sesión `query()` activa aplica:

* **`policyHelper`**: `resolveSettings()` lee fuentes MDM, incluidas plist de macOS y HKLM/HKCU de Windows, pero no ejecuta el subproceso `policyHelper` configurado por el administrador.
* **Configuración administrada por servidor**: `resolveSettings()` no obtiene [configuración administrada por servidor](/docs/es/managed-settings#delivery-mechanisms). Páselas como `options.serverManagedSettings` para incluirlas.
* **`defaultMode`**: la instantánea devuelve `permissions.defaultMode` tal como está de todos los niveles, por lo que puede incluir los valores `'auto'` y `'bypassPermissions'` de la configuración del proyecto y local, que [una sesión activa ignora](/docs/es/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parámetros
</h4>

`resolveSettings()` acepta un único objeto de opciones. Todos los campos son opcionales.

| Parámetro                       | Tipo                                  | Predeterminado    | Descripción                                                                                                                                                                                                                                                                                                                                                |
| :------------------------------ | :------------------------------------ | :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()`   | Directorio para resolver la configuración del proyecto y local relativa a                                                                                                                                                                                                                                                                                  |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Todas las fuentes | Qué fuentes del sistema de archivos cargar. Pase `[]` para omitir la configuración del usuario, proyecto y local. La [política administrada por punto final](/docs/es/managed-settings#delivery-mechanisms) se carga en todos los casos. `resolveSettings()` incluye configuración administrada por servidor solo cuando pasa `options.serverManagedSettings`   |
| `options.managedSettings`       | `Settings`                            | `undefined`       | Configuración de nivel de política suministrada por el host de incrustación. Sigue las mismas reglas que [`managedSettings` en `Options`](#options), excepto que `resolveSettings()` no ejecuta un [`policyHelper`](/docs/es/settings-reference#policyhelper) configurado, por lo que la instantánea puede incluir configuración que una sesión activa descarta |
| `options.serverManagedSettings` | `Settings`                            | `undefined`       | Carga útil de configuración administrada por servidor desde `/api/claude_code/settings`. Las claves no restrictivas pasan sin filtrar                                                                                                                                                                                                                      |

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

El ejemplo a continuación resuelve la configuración para un directorio de proyecto e imprime la fuente que controla el período de limpieza. En una máquina donde ningún archivo de configuración establece `cleanupPeriodDays`, ambas líneas impresas muestran `undefined` para el valor, que es la salida esperada en lugar de un error.

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

| Propiedad                         | Tipo                                                                                                                                                                                                           | Predeterminado                                     | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                            | Controlador para cancelar operaciones                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                               | Directorios adicionales a los que Claude puede acceder. El SDK pasa cada entrada a Claude Code como `--add-dir`, por lo que con la configuración `project` Claude Code también [carga los skills, comandos y subagentes del directorio](/docs/es/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                                        | Nombre del agente para el hilo principal. El agente debe estar definido en la opción `agents` o en la configuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                                        | Defina subagentes mediante programación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                            | Cuando es `true`, genere resúmenes de progreso de una línea para subagentes y reenvíelos en eventos [`task_progress`](#sdktaskprogressmessage) a través del campo `summary`. Se aplica a subagentes en primer plano y en segundo plano                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                            | Habilite omitir permisos. Requerido cuando se usa `permissionMode: 'bypassPermissions'`, al inicio o posteriormente a través de `setPermissionMode()`. Vea [Plan Mode](/docs/es/agent-sdk/permissions#plan-mode-plan) para cómo interactúa con `permissionMode: 'plan'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                               | Herramientas para aprobar automáticamente sin solicitar. Esto no restringe Claude a solo estas herramientas. Si nombra una de las [herramientas de seguimiento de tareas](/docs/es/agent-sdk/todo-tracking#model-availability) aquí, Claude Code también optar por la sesión. Las herramientas no listadas caen en `permissionMode` y `canUseTool`. Use `disallowedTools` para bloquear herramientas. Vea [Permissions](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                               | Habilite características beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                                        | Función de permiso personalizado, invocada solo cuando el [flujo de permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) cae en un mensaje. No se invoca para llamadas preaprobadas por `allowedTools`, reglas de permiso, o `permissionMode`. Una regla de permiso no preaprueba las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves). Vea [`CanUseTool`](#canusetool) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                            | Continúe la conversación más reciente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                                    | Directorio de trabajo actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                            | Habilite el modo de depuración para el proceso de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                                        | Escriba registros de depuración en una ruta de archivo específica. Habilita implícitamente el modo de depuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                               | Herramientas a negar. Un nombre simple como `"Bash"` elimina la herramienta del contexto de Claude. Una regla con alcance como `"Bash(rm *)"` deja la herramienta disponible y niega las llamadas coincidentes en cada modo de permiso, incluyendo `bypassPermissions`, para el comando [tal como está escrito](/docs/es/permissions#bash-rule-limits). Vea [Permissions](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                                        | Controla cuánto esfuerzo pone Claude en su respuesta. Funciona con el pensamiento adaptativo para guiar la profundidad del pensamiento. Vea [adjust the effort level](/docs/es/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                            | Habilite el seguimiento de cambios de archivo para rebobinar. Vea [File checkpointing](/docs/es/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                                      | Variables de entorno. Cuando se establece, esto reemplaza el entorno del subproceso en lugar de fusionarse con `process.env`, así que pase `{ ...process.env, YOUR_VAR: 'value' }` para mantener variables heredadas como `PATH`. Vea [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) para un ejemplo de este patrón, y [Environment variables](/docs/es/env-vars) para variables que la CLI subyacente lee. Establezca `CLAUDE_AGENT_SDK_CLIENT_APP` para identificar su aplicación en el encabezado User-Agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | Detectado automáticamente                          | Tiempo de ejecución de JavaScript a usar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                               | Argumentos a pasar al ejecutable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                               | Argumentos adicionales                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                                        | Modelo a usar si el principal falla. Acepta una lista separada por comas. Para el orden y el límite, vea [Fallback model chains](/docs/es/model-config#fallback-model-chains). Para orientación, vea [Choose a model](/docs/es/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                            | Cuando se reanuda con `resume`, bifurque a un nuevo ID de sesión en lugar de continuar la sesión original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                            | Reenvíe bloques de texto y pensamiento de subagentes como mensajes de asistente y usuario con `parent_tool_use_id` establecido, para que los consumidores puedan renderizar una transcripción anidada. Sin esta opción, Claude Code emite bloques `tool_use` y `tool_result` de subagentes pero no texto o pensamiento. Los mensajes de subagentes en cada profundidad de anidamiento se reenvían en Claude Code v2.1.219 y posterior; antes de v2.1.219, solo aparecían mensajes de subagentes de profundidad 1. Los mensajes de subagentes que un skill bifurcado genera, y de skills bifurcados anidados, requieren v2.1.275 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                               | Devoluciones de llamada de hooks para eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                            | Incluya eventos del ciclo de vida de hooks en la transmisión de mensajes como [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), y [`SDKHookResponseMessage`](#sdkhookresponsemessage). Los eventos del ciclo de vida para hooks `SessionStart` y `Setup` siempre se incluyen y no necesitan esta opción. Algunos eventos de hooks, como `Notification`, `SessionEnd`, `PreCompact`, y `PostCompact`, nunca producen un `SDKHookStartedMessage`, incluso con esta opción. Para esos eventos, Claude Code aún emite un `SDKHookProgressMessage` mientras un hook de comando que se ejecuta durante más de un segundo produce salida, y emite un `SDKHookResponseMessage` solo cuando un hook [que se ejecuta en segundo plano](/docs/es/hooks#run-hooks-in-the-background) finaliza                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                            | Incluya eventos de mensaje parcial                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                            | *Alpha.* Tiempo de espera en milisegundos para cada llamada `sessionStore.load()` y `sessionStore.listSubkeys()` durante la materialización de reanudación. Si el adaptador no se resuelve dentro de esta ventana, la consulta falla en lugar de colgarse. Se ignora cuando `sessionStore` no está establecido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                                        | Configuración de nivel de política suministrada por el proceso padre que genera la sesión. En máquinas con configuración administrada implementada por administrador, Claude Code ignora estas a menos que la fuente administrada de mayor prioridad del administrador establezca `parentSettingsBehavior: 'merge'`, y nunca las fusiona mientras un [`policyHelper`](/docs/es/settings-reference#policyhelper) suministra configuración administrada. Los valores fusionados pasan a través de un filtro de solo restrictivo; [Restrict parent settings](/docs/es/claude-apps-gateway#restrict-parent-settings) cubre lo que el filtro admite y los bloqueos `allowManaged*Only`. Un host que establece [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars) tiene tres claves leídas directamente de esta carga en su lugar: su [configuración de modelo](/docs/es/model-config#restrict-model-selection) en Claude Code v2.1.222 o posterior, [`modelPricing`](/docs/es/settings-reference#modelpricing) cuando ninguna fuente administrada la establece en v2.1.246 o posterior, y su entrada `ENABLE_TOOL_SEARCH` env en v2.1.247 o posterior                                                                                                                                                                              |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                                        | Detenga la consulta cuando la estimación de costo del lado del cliente alcance este valor en USD. Comparado con la misma estimación que `total_cost_usd`. Para advertencias de precisión y comportamiento de reinicio, vea [Track cost and usage](/docs/es/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                                        | *Deprecado:* Use `thinking` en su lugar. Tokens máximos para el proceso de pensamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                                        | Número máximo de turnos agentes (viajes de ronda de uso de herramientas)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                               | Configuraciones de servidor MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `model`                           | `string`                                                                                                                                                                                                       | Predeterminado de CLI                              | Alias de modelo Claude o nombre de modelo completo. Vea [accepted values and provider-specific IDs](/docs/es/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                                        | Devolución de llamada para manejar solicitudes de elicitación de MCP. Se llama cuando un servidor MCP solicita entrada del usuario y ningún hook la maneja primero. Cuando no se proporciona, las solicitudes de elicitación no manejadas se rechazan automáticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                                        | Defina el formato de salida para los resultados del agente. Vea [Structured outputs](/docs/es/agent-sdk/structured-outputs) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                                        | No es un campo `Options`. Establezca `outputStyle` en el objeto [`settings`](/docs/es/settings) en línea o en un archivo de configuración en su lugar. Vea [Activate an output style](/docs/es/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | Auto-resuelto desde el binario nativo incluido     | Ruta al ejecutable de Claude Code. Solo se necesita si las dependencias opcionales se omitieron durante la instalación o su plataforma no está en el conjunto compatible                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                                        | Modo de permiso para la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                                        | Nombre de herramienta MCP para solicitudes de permiso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                                           | Quién responde a los mensajes de permiso: `'host'` los enruta a su devolución de llamada [`canUseTool`](#canusetool) o a la herramienta `permissionPromptToolName`, y `'none'` [niega las llamadas que habrían solicitado](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated). Requiere Claude Code v2.1.259 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                             | Cuando es `false`, deshabilita la persistencia de sesión en disco. Las sesiones no se pueden reanudar más tarde                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                                        | Instrucciones de flujo de trabajo personalizado para Plan Mode. Cuando `permissionMode` es `'plan'`, esta cadena reemplaza el cuerpo de flujo de trabajo de Plan Mode predeterminado. La CLI aún lo envuelve con el preámbulo de cumplimiento de solo lectura y el pie de página del protocolo ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                               | Cargue plugins personalizados desde rutas locales. Vea [Plugins](/docs/es/agent-sdk/plugins) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                                        | Ruta absoluta del checkout de confianza del que `cwd` es un worktree. Claude Code lee la configuración del proyecto, `.mcp.json`, y los comandos, agentes, skills, flujos de trabajo, rutinas y estilos de salida del proyecto `.claude/` desde este directorio en lugar de `cwd`, y establece `CLAUDE_PROJECT_DIR` en él. Los hooks, scripts auxiliares como `apiKeyHelper`, y servidores MCP de stdio comienzan con este directorio como su directorio de trabajo. Los archivos `CLAUDE.md` y `.claude/rules/` aún se cargan desde `cwd`. Requiere Claude Code v2.1.275 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                            | Habilite sugerencias de mensaje. Después de un turno, Claude Code emite un mensaje `prompt_suggestion` que lleva un mensaje de usuario predicho siguiente. Claude Code no genera sugerencia para algunos turnos, como cuando su cuenta está cerca o en su límite de uso. Vea [When Claude Code skips suggestions](/docs/es/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                                        | ID de sesión a reanudar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                                        | Con `resumeSessionAt`: el UUID del mensaje del turno que la reanudación truncada intenta descartar. Claude Code rechaza la reanudación cuando el rango descartado contiene algo no atribuible a ese turno, como mensajes en cola absorbidos o notificaciones de tareas, y nombra el indicador `--resume-drops-turn` en el mensaje de rechazo. Solo el Agent SDK y las reanudaciones en modo de impresión leen el par. Requiere Claude Code v2.1.223 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                                        | Reanude la sesión en un UUID de mensaje específico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                                        | Configure el comportamiento de sandbox mediante programación. Vea [Sandbox settings](#sandboxsettings) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `sessionId`                       | `string`                                                                                                                                                                                                       | Auto-generado                                      | Use un UUID específico para la sesión en lugar de generar uno automáticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `sessionStore`                    | [`SessionStore`](/docs/es/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                                        | Refleje transcripciones de sesión en un backend externo para que otro host pueda reanudarlas. Vea [Persist sessions to external storage](/docs/es/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                                        | *Alpha.* Modo de vaciado para `sessionStore`. Se ignora cuando `sessionStore` no está establecido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                                        | Objeto de [settings](/docs/es/settings) en línea, ruta a un archivo de configuración, o cadena JSON en línea. Completa la capa de configuración de marca en el [orden de precedencia](/docs/es/settings#settings-precedence). Cambie en tiempo de ejecución con [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | Valores predeterminados de CLI (todas las fuentes) | Controle qué configuración del sistema de archivos cargar. Pase `[]` para deshabilitar la configuración de usuario, proyecto y local. La [política administrada por punto final](/docs/es/managed-settings#delivery-mechanisms) se carga independientemente; la configuración administrada por servidor se obtiene cuando la sesión se autentica con una credencial de organización en una [configuración elegible](/docs/es/server-managed-settings#platform-availability). Vea [Use Claude Code features](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                                        | Skills disponibles para la sesión. Pase `'all'` para habilitar cada skill descubierto, o una lista de nombres de skills. Pase solo nombres exactos. En Agent SDK v0.3.221 o posterior, el SDK rechaza nombres malformados y de forma comodín con un error antes de iniciar el proceso de Claude Code. Cuando se establece, el SDK agrega la herramienta Skill a `allowedTools` automáticamente. Si también pasa `tools`, incluya `'Skill'` en esa lista. Vea [Skills](/docs/es/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                                        | Función personalizada para generar el proceso de Claude Code. Use para ejecutar Claude Code en máquinas virtuales, contenedores o entornos remotos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                                        | Devolución de llamada para salida de stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                            | Use solo los servidores pasados en `mcpServers` e ignore el proyecto `.mcp.json`, la configuración del usuario, los servidores MCP proporcionados por plugins, y [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (mensaje mínimo)                       | Configuración de mensaje del sistema. Pase una cadena para un mensaje personalizado, o `{ type: 'preset', preset: 'claude_code' }` para usar el mensaje del sistema de Claude Code. Pase una matriz de cadenas con la constante exportada `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` entre las partes estática y por solicitud para [cachear la parte estática de un mensaje personalizado](/docs/es/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt). Cuando use la forma de objeto preestablecido, agregue `append` para extenderlo con instrucciones adicionales, y establezca `excludeDynamicSections: true` para mover el contexto por sesión al primer mensaje de usuario para [mejor reutilización de caché de mensaje en máquinas](/docs/es/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines). Establezca `snapshot: false` para reconstruir el mensaje en cada solicitud en lugar de [reutilizar el mensaje que la sesión registró en su primera solicitud](/docs/es/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Para establecer `snapshot` en un mensaje personalizado, pase la forma `{ type: 'custom', prompt }`. La forma `{ type: 'custom' }` y el campo `snapshot` requieren TypeScript Agent SDK v0.3.257 o posterior |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                                        | *Alpha.* Presupuesto de tarea del lado de la API en tokens. Cuando se establece, se le dice al modelo su presupuesto de token restante para que pueda controlar el uso de herramientas y terminar antes del límite                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | `{ type: 'adaptive' }` para modelos compatibles    | Controla el comportamiento de pensamiento/razonamiento de Claude. Vea [`ThinkingConfig`](#thinkingconfig) para opciones                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                                        | Título de visualización para la sesión. Cuando se reanuda a través de `resume` o `continue`, el título persistente de la sesión reanudada tiene precedencia; use [`renameSession()`](#renamesession) para cambiar el título de una sesión existente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                                        | Mapee nombres de herramientas integradas a nombres de herramientas MCP para que Claude llame a su implementación MCP en lugar de la integrada. Por ejemplo, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                                        | Configuración para el comportamiento de herramientas integradas. Vea [`ToolConfig`](#toolconfig) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                                        | Configuración de herramientas. Pase una matriz de nombres de herramientas o use el preestablecido para obtener las herramientas predeterminadas de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="handle-slow-or-stalled-api-responses">
  Manejo de respuestas de API lentas o estancadas
</h4>

El subproceso CLI lee varias variables de entorno que controlan los tiempos de espera de API y la detección de estancamiento. Páselas a través de la opción `env`:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

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
* `CLAUDE_CODE_MAX_RETRIES`: máximo de reintentos de API. Predeterminado `10`, limitado a `15`. Cada reintento obtiene su propia ventana `API_TIMEOUT_MS`, por lo que el tiempo de pared en el peor caso es aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` más retroceso. Para ejecuciones desatendidas que necesitan esperar a través de interrupciones más largas, establezca [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/es/errors#tune-retry-behavior): reintenta errores de capacidad transitorios indefinidamente y, en Claude Code v2.1.199 o posterior, eleva el predeterminado para otros errores transitorios a `300` y elimina el límite en esta variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: perro guardián de estancamiento para subagentes. Mientras el perro guardián de transmisión está activado, el predeterminado es `CLAUDE_STREAM_IDLE_TIMEOUT_MS` más 5 minutos, que suma `600000` a menos que eleve esa variable. Con el perro guardián de transmisión desactivado, el predeterminado es `600000`. Antes de v2.1.257, el predeterminado era siempre `600000`.

  El temporizador se reinicia en cada evento de transmisión. En caso de estancamiento, Claude Code aborta el subagente e informa el estancamiento al padre. Para un subagente de fondo, también marca la tarea como fallida y adjunta cualquier resultado parcial.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` con `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: perro guardián de transmisión que aborta la solicitud cuando los encabezados han llegado pero el cuerpo de respuesta deja de transmitirse. El perro guardián está activado de forma predeterminada para todos los proveedores; establezca `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desactivarlo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` tiene un valor predeterminado de `300000` y se fija a ese mínimo. Después de la anulación, [Automatic retries](/docs/es/errors#automatic-retries) cubre lo que Claude Code hace, basado en cuán lejos había progresado la respuesta.

  Mientras el perro guardián espera una respuesta que una puerta de enlace detrás de `ANTHROPIC_BASE_URL` mantiene abierta con pings de keep-alive, un host que establece `includePartialMessages` sigue recibiendo eventos de transmisión `ping` [stream events](#sdkpartialassistantmessage), así que lea esos marcos como vivacidad en lugar de agotar el tiempo de espera de la sesión en silencio. Antes de v2.1.257, los marcos se detenían 5 minutos después del último evento de transmisión real.

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
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Métodos
</h4>

| Método                                 | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrumpe la consulta. Solo disponible en modo de entrada de transmisión. Cuando la CLI anuncia la capacidad `interrupt_receipt_v1` en [`SDKSystemMessage.capabilities`](#sdksystemmessage), se resuelve con un [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) que enumera los mensajes que estaban pendientes cuando llegó la interrupción. Se resuelve a `undefined` en CLIs anteriores a v2.1.205                                                                                                                                           |
| `rewindFiles(userMessageId, options?)` | Restaura archivos a su estado en el mensaje de usuario especificado. Pase `{ dryRun: true }` para obtener una vista previa de los cambios. Requiere `enableFileCheckpointing: true`. Vea [File checkpointing](/docs/es/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                |
| `setPermissionMode()`                  | Cambia el modo de permiso (solo disponible en modo de entrada de transmisión)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `setModel()`                           | Cambia el modelo (solo disponible en modo de entrada de transmisión). Pasar `undefined` o la cadena `"default"` reinicia al [modelo predeterminado de Claude Code](/docs/es/model-config)                                                                                                                                                                                                                                                                                                                                                                           |
| `setMaxThinkingTokens()`               | *Deprecado:* Use la opción `thinking` en su lugar. Cambia los tokens de pensamiento máximos. Pasar `null` reinicia el pensamiento al valor predeterminado de la sesión: se borra una anulación a mitad de sesión, y el pensamiento permanece desactivado para sesiones que lo tienen deshabilitado                                                                                                                                                                                                                                                             |
| `applyFlagSettings(settings)`          | Fusiona la configuración en la capa de configuración de marca de la sesión en tiempo de ejecución (solo disponible en modo de entrada de transmisión). Vea [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                         |
| `updateSettings(source, settings)`     | Escribe una clave permitida en el archivo de configuración local del proyecto o en su archivo de configuración de usuario, para que el valor persista para sesiones posteriores. Vea [`updateSettings()`](#updatesettings). Requiere TypeScript SDK v0.3.257 o posterior, que incluye Claude Code v2.1.257                                                                                                                                                                                                                                                     |
| `initializationResult()`               | Devuelve el resultado de inicialización completo incluyendo comandos compatibles, modelos, información de cuenta y configuración de estilo de salida                                                                                                                                                                                                                                                                                                                                                                                                           |
| `reinitialize()`                       | Reenvía la solicitud de control `initialize` a la CLI en ejecución y devuelve un resultado nuevo en lugar del resultado de primera conexión en caché. Úselo después de una brecha de transporte, como reconectarse a una sesión después de una desconexión, para que las solicitudes de permiso pendientes lleguen a su devolución de llamada `canUseTool` nuevamente. Haga que la devolución de llamada sea idempotente por ID de solicitud, porque una solicitud cuya respuesta se perdió se envía nuevamente. Requiere Claude Code v2.1.195 o posterior     |
| `supportedCommands()`                  | Devuelve comandos disponibles. Desde Agent SDK v0.3.216 la lista refleja cambios de comandos a mitad de sesión; vea [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                                                                                                                                                                                                  |
| `supportedModels()`                    | Devuelve modelos disponibles con información de visualización                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `supportedAgents()`                    | Devuelve subagentes disponibles como [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `mcpServerStatus()`                    | Devuelve el estado de los servidores MCP conectados como [`McpServerStatus`](#mcpserverstatus)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `getContextUsage(opts?)`               | Devuelve un [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) desglosando el uso de la ventana de contexto de la sesión por categoría, skill y herramienta. Con el `detail` predeterminado, es el mismo dato que `/context` muestra en una sesión interactiva. La [opción `detail`](#sdkcontrolgetcontextusageresponse) requiere Agent SDK v0.3.257 o posterior                                                                                                                                                                        |
| `readFile(path, options?)`             | Lee un archivo del sistema de archivos de la sesión. Claude Code resuelve la ruta contra `cwd`; [What `readFile()` can read](#what-readfile-can-read) enumera los archivos que sirve. Pase `{ maxBytes }` para cambiar el límite de lectura (predeterminado 1 MB, techo 10 MB) y `{ encoding: 'base64' }` para archivos binarios como imágenes. Se resuelve con un [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse), o `null` en denegación de permiso, un archivo faltante, o un error de transporte. Requiere TypeScript SDK v0.2.121 o posterior |
| `reloadSkills()`                       | Recarga skills desde el disco, por lo que los skills que agrega o edita a mitad de sesión se ponen a disposición de la sesión en ejecución. Se resuelve con un [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) que enumera los skills disponibles después de la recarga. Requiere Agent SDK v0.3.163 o posterior                                                                                                                                                                                                                           |
| `accountInfo()`                        | Devuelve información de cuenta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `reconnectMcpServer(serverName)`       | Reconecte un servidor MCP por nombre. Si el nombre también coincide con una entrada en un archivo de configuración como `.mcp.json` o `~/.claude.json`, Claude Code reconecta el servidor que configuró a través de [`mcpServers`](#options) o `setMcpServers()`, no la entrada del archivo de configuración. Ese orden de resolución requiere Claude Code v2.1.257 o posterior                                                                                                                                                                                |
| `toggleMcpServer(serverName, enabled)` | Habilite o deshabilite un servidor MCP por nombre, con la misma resolución de nombre que `reconnectMcpServer()`. Deshabilitar desconecta el servidor                                                                                                                                                                                                                                                                                                                                                                                                           |
| `setMcpServers(servers)`               | Reemplace dinámicamente el conjunto de servidores MCP para esta sesión. Se resuelve con un [`McpSetServersResult`](#mcpsetserversresult) que nombra qué servidores se agregaron y eliminaron, y cualquier error                                                                                                                                                                                                                                                                                                                                                |
| `readMcpResource(serverName, uri)`     | *Alpha.* Lee un recurso MCP Apps `ui://` de un servidor MCP conectado para que su aplicación pueda renderizar el widget de una herramienta. Se resuelve con un [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse). Requiere TypeScript Agent SDK v0.3.280 o posterior                                                                                                                                                                                                                                                                   |
| `streamInput(stream)`                  | Transmita mensajes de entrada a la consulta para conversaciones de múltiples turnos                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `stopTask(taskId)`                     | Detenga una tarea de fondo en ejecución por ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `close()`                              | Cierre la consulta y termine el proceso subyacente. Finaliza forzadamente la consulta y limpia todos los recursos                                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Cambia [settings](/docs/es/settings) en una sesión en ejecución sin reiniciar la consulta. Úselo cuando una configuración que no tiene un setter dedicado necesite cambiar a mitad de sesión, como restringir `permissions` después de que el agente lea entrada no confiable. `setModel()` y `setPermissionMode()` son setters dedicados para esas dos claves; `applyFlagSettings()` es la forma general que acepta cualquier subconjunto de las claves de configuración, y pasar `model` aquí se comporta igual que `setModel()`.

Solo algunas claves tienen efecto a mitad de sesión:

* **Aplicadas en el siguiente turno**: `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Cambiar `agent` también aplica la anulación de modelo y hooks de ese agente en el siguiente turno. Su mensaje del sistema se aplica en el siguiente turno, o, en una sesión que [reutiliza un mensaje del sistema registrado](/docs/es/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session), una vez que la sesión se compacta.
* **Aplicadas durante el turno actual**: `model`. Si cambia `model` mientras Claude está trabajando en un turno, la respuesta que Claude ya está generando finaliza en el modelo anterior, y el resto del turno, comenzando con la siguiente llamada que Claude Code hace al modelo, usa el nuevo. Los subagentes mantienen su propio modelo. Antes de v2.1.212, un cambio a mitad de turno esperaba el siguiente turno.
* **Sin efecto a mitad de sesión**: las opciones de mensaje del sistema. Estos se resuelven una vez al inicio, por lo que la sesión en ejecución mantiene el valor original aunque la llamada tenga éxito. Para cambiarlos, inicie una nueva sesión.

`effortLevel` acepta un nombre de [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level). También acepta `"ultracode"`, que solicita esfuerzo `xhigh` con [ultracode](/docs/es/workflows#let-claude-decide-with-ultracode) activado. `applyFlagSettings()` declara `effortLevel` sin ese valor, así que pase el equivalente `{ ultracode: true }` en TypeScript. El valor `ultracode` requiere Claude Code v2.1.203 o posterior y solo es aceptado por `applyFlagSettings()`, no por la clave `effortLevel` en un archivo de configuración.

Los valores se escriben en la capa de configuración de marca, la misma capa que la opción `settings` en línea de `query()` completa al inicio. Esta es la misma capa que la [sección de precedencia en la página](#settings-precedence) llama opciones programáticas.

Las llamadas sucesivas fusionan superficialmente las claves de nivel superior. Una segunda llamada con `{ permissions: {...} }` reemplaza el objeto `permissions` completo de la llamada anterior en lugar de fusionarse profundamente en él.

Para borrar una clave que estableció con `applyFlagSettings()`, pase `null` para esa clave. La mayoría de las claves luego recurren primero a un valor que la opción `settings` de `query()` estableció al inicio, luego a fuentes de menor precedencia. Un `model` borrado se reinicia al [modelo predeterminado de Claude Code](/docs/es/model-config), incluso cuando un archivo de configuración establece `model`. Pasar `undefined` no tiene efecto porque la serialización JSON lo elimina.

Tres claves además de `model` reinician el estado de la sesión en lugar de recurrir:

* `effortLevel: null` devuelve la sesión al nivel de esfuerzo predeterminado del modelo, no a la opción `effort` de `query()` o un `effortLevel` de un archivo de configuración.
* `agent: null` ejecuta el hilo principal sin agente, comenzando con el siguiente turno, en lugar de restaurar la opción `agent` de `query()` o un `agent` de un archivo de configuración. Si el agente borrado había aplicado su propio modelo, la sesión vuelve al modelo que resolvió al inicio.
* `ultracode: null` desactiva ultracode, como lo hace `false`, en lugar de restaurar un valor `ultracode` de un archivo de configuración. La sesión mantiene su nivel de esfuerzo actual, así que pase `effortLevel` en la misma llamada para cambiarlo.

Solo disponible en modo de entrada de transmisión, la misma restricción que `setModel()` y `setPermissionMode()`.

El ejemplo a continuación cambia el modelo activo a mitad de sesión, luego borra la anulación para que el modelo se reinicie al [modelo predeterminado de Claude Code](/docs/es/model-config).

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Anule el modelo para el resto de la sesión
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Más tarde: borre la anulación; el modelo se reinicia al modelo predeterminado de Claude Code
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` es solo TypeScript. El SDK de Python no expone un método equivalente.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

Escribe una clave permitida en un archivo de configuración en disco, para que el valor persista para sesiones posteriores que carguen esa fuente. Cada fuente acepta una clave, con un valor de cadena:

* **`"localSettings"`**: acepta `outputStyle` y lo fusiona en el archivo de configuración local del proyecto, `.claude/settings.local.json`. El nuevo estilo entra en vigor en la siguiente solicitud de la sesión.
* **`"userSettings"`**: acepta `effortLevel` y lo guarda como el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) predeterminado para el modelo actual de la sesión, bajo [`modelSettings`](/docs/es/settings-reference#modelsettings) en su archivo de configuración de usuario. Pasar `max` no escribe nada, porque `max` es solo de sesión. La sesión en ejecución mantiene su nivel de esfuerzo actual de cualquier forma, así que llame a [`applyFlagSettings()`](#applyflagsettings) cuando también quiera cambiar eso. Esta fuente requiere TypeScript SDK v0.3.277 o posterior, que incluye Claude Code v2.1.277.

La llamada rechaza cuando la solicitud lleva cualquier otra clave, cuando la sesión se ejecuta sobre un transporte remoto, y cuando las [`settingSources`](#options) de la sesión excluyen la fuente que nombra. No se admite la eliminación de una clave.

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
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied` informa si Claude Code registró los `hooks` que la solicitud `initialize` llevaba. El SDK envía esa solicitud una vez cuando la sesión comienza y nuevamente en cada llamada [`reinitialize()`](#query-object). El campo requiere Agent SDK v0.3.238 o posterior.

Claude Code omite el campo cuando la solicitud no llevaba hooks. Cuando la solicitud llevaba hooks, el valor depende de si la solicitud es la primera inicialización de la sesión y, para una repetida, de cómo llegó a la sesión:

* `true`: Claude Code registró los hooks. Una primera inicialización de sesión devuelve este valor. También lo hace una inicialización repetida enviada sobre stdin de la CLI. En ese caso los hooks en la nueva solicitud reemplazan los hooks registrados anteriormente.
* `false`: Claude Code ignoró los hooks. Una inicialización repetida enviada a una sesión remota devuelve este valor, por lo que un segundo cliente que se une a una sesión no puede reemplazar los hooks que el primer cliente registró.

Antes de Agent SDK v0.3.238, la respuesta nunca llevaba el campo, y Claude Code ignoraba `hooks` en cada inicialización repetida.

La respuesta siempre informa `fast_mode_state`, y cuando algo bloquea [fast mode](/docs/es/fast-mode), `fast_mode_disabled_reason` lleva el código de razón junto con él, para que pueda explicar el estado bloqueado en lugar de re-derivar la disponibilidad. Ambos comportamientos requieren Claude Code v2.1.219 o posterior. Antes de v2.1.219, la respuesta omitía `fast_mode_state` cuando fast mode no estaba disponible y nunca llevaba una razón. Para los códigos de razón y sus significados, vea [`fast_mode_disabled_reason`](#sdkresultmessage) en el mensaje de resultado.

El contenedor de respuesta de control para una `initialize` exitosa también lleva una matriz `pending_permission_requests`. El campo está en el contenedor de respuesta en sí, no en la carga `SDKControlInitializeResponse` anterior. Cada entrada es un mensaje `control_request` completo con la misma forma `{ type: "control_request", request_id, request }` que la sesión transmite para solicitudes de permiso mientras se ejecuta.

La matriz enumera las solicitudes de permiso que este proceso de Claude Code ha emitido y aún no ha resuelto. El SDK lee la matriz para usted y envía cada entrada a su devolución de llamada [`canUseTool`](#canusetool), el mismo reenvío que [`reinitialize()`](#query-object) activa después de una brecha de transporte. Maneje IDs de solicitud repetidos de forma idempotente, porque una entrada puede repetir una solicitud que la devolución de llamada ya recibió antes de que se cayera la conexión.

La matriz siempre está presente en una respuesta `initialize` exitosa y está vacía cuando este proceso no tiene solicitud de permiso sin resolver. Requiere Claude Code v2.1.268 o posterior. Las versiones anteriores podrían omitir el campo, así que si analiza el protocolo de cable usted mismo, trate un campo faltante como una CLI más antigua en lugar de como prueba de que nada está pendiente.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

El recibo de interrupción: el valor que [`interrupt()`](#query-object) se resuelve con en una CLI que anuncia la capacidad `interrupt_receipt_v1` en [`SDKSystemMessage.capabilities`](#sdksystemmessage). Requiere Claude Code v2.1.205 o posterior. Las CLIs anteriores responden a la interrupción con una carga de éxito vacía, por lo que `interrupt()` se resuelve a `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` enumera los UUIDs de los mensajes de usuario que estaban pendientes cuando llegó la interrupción: mensajes aún en la cola, más cualquier mensaje que Claude Code ya había sacado de la cola para el siguiente turno. Una vez que el primer turno de la sesión ha comenzado, Claude Code procesa los mensajes listados después de la interrupción a menos que los cancele primero, y puede fusionar varios en un turno. Si interrumpe antes de que comience el primer turno, Claude Code aborta ese turno tan pronto como comienza, y los mensajes listados en ese turno no obtienen respuesta.

Use el recibo para decidir si debe reenviar algo. Un mensaje listado que no cancele entra en la conversación independientemente de si obtiene una respuesta, por lo que reenviarlo lo entrega a Claude dos veces.

Interprete la lista con estas advertencias:

* Solo los mensajes que fueron encolados con un UUID aparecen. Una matriz vacía no significa que nada más se ejecutará.
* Solo se enumeran los mensajes del hilo principal. Los mensajes dirigidos a un subagente están fuera del alcance.
* La lista puede incluir UUIDs que su cliente nunca envió, como [activadores de tareas programadas](/docs/es/scheduled-tasks). Ignore los UUIDs que no reconozca en lugar de tratarlos como un error.

Un cliente que controla el protocolo de control de la CLI directamente, en lugar de a través de `interrupt()`, puede establecer `cancel_queued: true` en la solicitud de control `interrupt`. Claude Code v2.1.219 y posterior anuncia soporte con la capacidad `interrupt_cancel_queued_v1` en [`SDKSystemMessage.capabilities`](#sdksystemmessage); las CLIs más antiguas ignoran el campo y dejan que los mensajes en cola se ejecuten como de costumbre. Tal interrupción también cancela cada mensaje que de otro modo sería listado bajo `still_queued`: el recibo los lista bajo `cancelled` en su lugar, `still_queued` está vacío, y ninguno de ellos se ejecuta.

La lista `cancelled` lleva las mismas advertencias que `still_queued`. El método `interrupt()` nunca envía `cancel_queued`, por lo que los recibos que se resuelve no llevan `cancelled`.

El recibo es una instantánea tomada en el momento en que se procesa la interrupción, y en una interrupción limpia llega antes del [`SDKResultMessage`](#sdkresultmessage) del turno interrumpido. Lea el recibo en lugar de inspeccionar la cola después de ese resultado: el bucle inicia el siguiente turno en cola inmediatamente, por lo que la cola que inspecciona después del resultado ya ha cambiado.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

Tipo de retorno de [`getContextUsage()`](#query-object). Con el `detail` predeterminado, esta es la misma carga que Claude Code renderiza para el comando `/context` en una sesión interactiva, por lo que junto con los conteos de tokens lleva campos de visualización como `color` y `gridRows` que Claude Code usa para dibujar la cuadrícula de uso de `/context`.

El argumento `detail` opcional del método elige cómo Claude Code cuenta cada categoría. Con el predeterminado, `'full'`, Claude Code cuenta cada categoría con solicitudes de API de conteo de tokens. Pase `{ detail: 'summary' }` para obtener una respuesta del uso de la última respuesta y estimaciones locales en su lugar. No se realizan solicitudes de conteo de tokens, y los números por categoría son aproximados. El argumento `detail` requiere Agent SDK v0.3.257 o posterior.

Cuando envía `/context` como un mensaje en lugar de llamar al método, Claude Code adjunta una carga [`SDKContextUsage`](#sdkcontextusage) al campo `context_usage` del mensaje del asistente que entrega el resultado. Ese campo requiere Agent SDK v0.3.232 o posterior.

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

Lea la atribución de tokens de los campos de colección:

* `categories` contiene los totales por categoría.
* `mcpTools` y `agents` atribuyen tokens a herramientas MCP individuales y subagentes.
* `memoryFiles` enumera cada archivo de memoria cargado con su costo.
* `skills.skillFrontmatter` atribuye los tokens de la lista de skills a cada skill incluido. Los conteos por skill miden la entrada de cada skill tal como Claude Code realmente la envía, que puede ser más corta que el frontmatter completo del skill. Compare `skills.totalSkills` con `skills.includedSkills` para ver si cada skill descubierto hizo en la lista.

`totalTokens` es el uso de contexto actual de la sesión, y `maxTokens` es la ventana contra la que se mide el uso. Esa ventana es la ventana de contexto del modelo, o la ventana de auto-compactación más baja cuando se aplica una. `rawMaxTokens` lleva el mismo valor que `maxTokens`, y `percentage` es `totalTokens` como un porcentaje redondeado de esa ventana.

Claude Code deja los diagnósticos opcionales `deferredBuiltinTools`, `systemTools`, y `systemPromptSections` sin establecer, así que espere que estén ausentes incluso aunque el tipo los declare.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

Tipo de retorno de [`readFile()`](#query-object).

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` contiene el texto del archivo, o datos base64 cuando solicitó `encoding: 'base64'`; el campo `encoding` de la respuesta se establece en `'base64'` en ese caso. `absPath` es la ruta absoluta resuelta. `truncated` se establece cuando el archivo era más largo que el límite `maxBytes` y el contenido fue cortado en ese límite.

<h4 id="what-readfile-can-read">
  Lo que `readFile()` puede leer
</h4>

`readFile()` sirve un conjunto más estrecho de archivos que la herramienta Read:

* Un archivo regular dentro de uno de los directorios de trabajo de la sesión, como `cwd` y `additionalDirectories`
* Algunos de los propios archivos de Claude Code para la sesión, como resultados de herramientas

Las reglas de negación y solicitud de Read aún bloquean una ruta coincidente, y una regla de permiso amplia de Read no abre el resto del sistema de archivos a `readFile()`. Para cualquier otra cosa la llamada se resuelve con `null`.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

Tipo de retorno de [`reloadSkills()`](#query-object).

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` enumera los skills disponibles después de la recarga, en la misma forma [`SlashCommand`](#slashcommand) que `supportedCommands()` devuelve.

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

Tipo de retorno de [`readMcpResource()`](#query-object), que lleva el resultado `resources/read` del servidor MCP. Requiere TypeScript Agent SDK v0.3.280 o posterior.

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

Pase a `readMcpResource()` el nombre del servidor tal como `mcpServerStatus()` lo reporta y un URI `ui://`, como el `ui.resourceUri` que una herramienta declara en su [`_meta`](#mcpserverstatus). La llamada rechaza para cualquier otro esquema de URI, para un [servidor MCP del SDK](#createsdkmcpserver) que su aplicación aloja a sí misma, y para un servidor que no está conectado. Está disponible cuando el mensaje de inicialización [`capabilities`](#sdksystemmessage) incluye `mcp_read_resource_v1`.

Cada entrada `contents` es un elemento de contenido tal como el servidor lo envió. `blob` contiene datos base64 para un elemento binario, y `_meta` es el `_meta` del elemento, donde un servidor MCP Apps pone el `ui.csp` y `ui.permissions` del recurso. El contenido es HTML de terceros no confiable, así que renderícelo en un sandbox.

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
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Campo                                 | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------------------ | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Sí        | Descripción en lenguaje natural de cuándo usar este agente                                                                                                                                                                                                                                                                                                                                                 |
| `tools`                               | No        | Matriz de nombres de herramientas permitidas. Si se omite, hereda cada [herramienta disponible para subagentes](/docs/es/sub-agents#available-tools). Para precargar Skills en el contexto del agente, use el campo `skills` en lugar de enumerar `'Skill'` aquí                                                                                                                                                |
| `disallowedTools`                     | No        | Matriz de nombres de herramientas a desautorizar explícitamente para este agente. Los patrones de nivel de servidor MCP también se aceptan: `mcp__server` o `mcp__server__*` elimina cada herramienta de ese servidor, y `mcp__*` elimina cada herramienta MCP de cualquier servidor                                                                                                                       |
| `prompt`                              | Sí        | El mensaje del sistema del agente                                                                                                                                                                                                                                                                                                                                                                          |
| `model`                               | No        | Anulación de modelo para este agente. Acepta un alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID de modelo completo. `'inherit'` usa el modelo principal. Cuando lo omite, Claude Code elige el modelo en el [orden de modelo de subagente](/docs/es/sub-agents#choose-a-model)                                                                                                      |
| `mcpServers`                          | No        | Especificaciones de servidor MCP para este agente                                                                                                                                                                                                                                                                                                                                                          |
| `skills`                              | No        | Matriz de nombres de skills a precargar en el contexto del agente                                                                                                                                                                                                                                                                                                                                          |
| `initialPrompt`                       | No        | Auto-enviado como el primer turno de usuario cuando este agente se ejecuta como el agente del hilo principal                                                                                                                                                                                                                                                                                               |
| `maxTurns`                            | No        | Número máximo de turnos agentes (viajes de ronda de API) antes de detener                                                                                                                                                                                                                                                                                                                                  |
| `background`                          | No        | Ejecute este agente como una tarea de fondo no bloqueante cuando se invoque                                                                                                                                                                                                                                                                                                                                |
| `omitClaudeMd`                        | No        | Ejecute este agente sin los archivos CLAUDE.md de usuario, proyecto y local cuando se ejecuta como un subagente; los archivos de política administrada aún se cargan. Úselo para agentes que toman todo lo que necesitan del mensaje de solicitud de herramienta del agente. Se ignora cuando este agente se ejecuta como el agente del hilo principal. Requiere TypeScript Agent SDK v0.3.271 o posterior |
| `memory`                              | No        | Fuente de memoria para este agente: `'user'`, `'project'`, o `'local'`                                                                                                                                                                                                                                                                                                                                     |
| `effort`                              | No        | Nivel de esfuerzo de razonamiento para este agente. Acepta un nivel nombrado o un entero                                                                                                                                                                                                                                                                                                                   |
| `permissionMode`                      | No        | Modo de permiso para la ejecución de herramientas dentro de este agente. Las [reglas de herencia de subagente](/docs/es/agent-sdk/permissions#available-modes) deciden cuándo se aplica. Vea [`PermissionMode`](#permissionmode)                                                                                                                                                                                |
| `criticalSystemReminder_EXPERIMENTAL` | No        | Experimental: Recordatorio crítico agregado al mensaje del sistema                                                                                                                                                                                                                                                                                                                                         |

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

| Valor       | Descripción                                                                                           | Ubicación                     |
| :---------- | :---------------------------------------------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Configuración global del usuario                                                                      | `~/.claude/settings.json`     |
| `'project'` | Configuración de proyecto compartida (controlada por versión)                                         | `.claude/settings.json`       |
| `'local'`   | Configuración de proyecto local, ignorada por git cuando Claude Code guarda una configuración en ella | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamiento predeterminado
</h4>

Cuando `settingSources` se omite o es `undefined`, `query()` carga la misma configuración del sistema de archivos que la CLI de Claude Code: usuario, proyecto y local. Vea [What settingSources does not control](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que se leen independientemente de esta opción, y cómo deshabilitarlas.

<h4 id="why-use-settingsources">
  Por qué usar settingSources
</h4>

**Deshabilitar configuración del sistema de archivos:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// No cargue la configuración de usuario, proyecto o local desde el disco
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Cargue solo fuentes de configuración específicas:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Cargue solo la configuración del proyecto, ignore usuario y local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Solo .claude/settings.json
  }
});
```

Para cargar instrucciones de proyecto CLAUDE.md, incluya `"project"` en `settingSources`. Vea [Modify system prompts](/docs/es/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) para cómo la carga de CLAUDE.md interactúa con las opciones de mensaje del sistema.

<h4 id="settings-precedence">
  Precedencia de configuración
</h4>

Cuando se cargan múltiples fuentes, la configuración se fusiona con esta precedencia (mayor a menor):

1. Configuración local (`.claude/settings.local.json`)
2. Configuración del proyecto (`.claude/settings.json`)
3. Configuración del usuario (`~/.claude/settings.json`)

Las opciones programáticas como `agents`, `allowedTools`, y `settings` anulan la configuración del sistema de archivos de usuario, proyecto y local. La configuración de política administrada tiene precedencia sobre las opciones programáticas.

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
  | "auto"; // Clasificador de modelo aprueba o niega mensajes de permiso
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Tipo de función de permiso personalizado para controlar el uso de herramientas.

La función es el reemplazo del SDK para el mensaje de permiso interactivo: se invoca solo cuando el [flujo de evaluación de permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) se resuelve en un mensaje. Las llamadas de herramientas ya aprobadas por una entrada `allowedTools`, una regla de configuración de permiso, o el modo de permiso, como `acceptEdits` o `bypassPermissions`, nunca la invocan. Para controlar cada llamada de herramienta, use un [hook `PreToolUse`](/docs/es/agent-sdk/hooks) en su lugar.

Una regla de permiso no preaprueba las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves); vea [How permissions are evaluated](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) para cuál de ellas llega a la devolución de llamada y qué sucede en modo `dontAsk` y `auto`.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
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
| `mcpServer`      | `{ name: string; source: string }`          | Para una herramienta `mcp__*`, el servidor MCP que la sirve y de dónde vino la definición de ese servidor, con los campos de [`McpServerProvenance`](#mcpserverprovenance). Ausente para otras herramientas. Requiere Agent SDK v0.3.274 o posterior                                                                                                             |
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
  timeout?: number;
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
  Tipos de Mensajes
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
  message: BetaMessage; // From Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

El campo `message` es un [`BetaMessage`](https://platform.claude.com/docs/en/api/messages/create) del SDK de Anthropic. Incluye campos como `id`, `content`, `model`, `stop_reason` y `usage`.

`SDKAssistantMessageError` es uno de: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'`, o `'unknown'`. Cuatro de estos valores significan más de lo que sus nombres dicen:

* `'model_not_found'`: el modelo seleccionado no existe o no está disponible para su cuenta o implementación
* `'overloaded'`: la API devolvió un 529 porque el servidor está a capacidad, a diferencia de `'rate_limit'`, que es un 429 contra su cuota
* `'account_on_hold'`: [su cuenta está en espera](/docs/es/errors#your-account-is-on-hold)
* `'cloud_credential_error'`: Claude Code no pudo obtener credenciales de AWS o Google Cloud utilizables en la máquina en la que se ejecuta, por lo que ninguna solicitud llegó al proveedor de nube. La causa habitual es un inicio de sesión en la nube que expiró o nunca se completó en esa máquina, aunque un servicio de credenciales brevemente inaccesible reporta el mismo valor. Consulte [No se pudieron cargar las credenciales de AWS o Google Cloud](/docs/es/errors#could-not-load-aws-or-google-cloud-credentials). Requiere TypeScript Agent SDK v0.3.267 o posterior, que incluye Claude Code v2.1.267

`aborted` es `true` cuando una interrupción o cancelación truncó el mensaje del asistente antes de que la transmisión se completara: el mensaje no tiene `stop_reason` y el contenido puede terminar a mitad de palabra. El campo está ausente en los mensajes completados normalmente. Requiere Agent SDK v0.3.214 o posterior.

Claude Code establece `user_message_uuid` y `user_message_uuids` en el primer mensaje del asistente del turno, bajo las condiciones en [`user_message_uuid`](#user_message_uuid).

`timestamp` es la hora ISO 8601 cuando el contenido del mensaje terminó de generarse en el proceso que lo produjo. El valor proviene del reloj de esa máquina, así que úselo solo para mostrar y no ordene los mensajes por él. Un turno de API puede producir varios mensajes del asistente que comparten un `message.id`, cada uno con su propio `timestamp`. Cuando el campo está ausente, recurra a la hora en que recibió el mensaje.

`context_usage` es una copia estructurada del informe `/context`, escrita como [`SDKContextUsage`](#sdkcontextusage), y requiere Agent SDK v0.3.232 o posterior. Cuando envía `/context` como un prompt, Claude Code entrega el informe como un mensaje del asistente cuyo `message.content` contiene la tabla de markdown, y adjunta `context_usage` a ese mismo mensaje. Claude Code no establece el campo en ningún otro mensaje del asistente, y las versiones anteriores entregan la tabla `/context` sin él, así que lea el desglose del campo cuando esté presente y recurra al texto de markdown cuando no lo esté.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Mensaje de entrada del usuario.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // From Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

Establezca `pasted_content` para enviar contenido que el usuario pegó en su interfaz de prompt en lugar de escribir, una entrada por pegado, cada una una cadena o una matriz de bloques de contenido. Claude Code añade el texto de cada entrada después del texto escrito, en orden, y puede envolver cada pegado en etiquetas `<pasted_content>`. Los bloques que no sean texto se ignoran, así que envíe imágenes y documentos en `message.content`. Requiere Agent SDK v0.3.277 o posterior.

Establezca `shouldQuery` en `false` para añadir el mensaje a la transcripción sin activar un turno del asistente. El mensaje se retiene y se fusiona en el siguiente mensaje del usuario que sí activa un turno. Úselo para inyectar contexto, como la salida de un comando que ejecutó fuera de banda, sin gastar una llamada de modelo en él.

En un mensaje que lleva un bloque `tool_result`, `tool_use_result` es el objeto de salida estructurado de la herramienta en lugar del texto enviado al modelo. Su forma depende de la herramienta nombrada por el bloque `tool_use` coincidente, así que el campo se escribe `unknown`; las formas integradas se enumeran en [Tipos de Salida de Herramientas](#tool-output-types).

Para la herramienta `Agent`, `tool_use_result` es [`AgentOutput`](#agent-2). En un resultado `completed`, `content` contiene el informe del subagente sin el ID del agente y el tráiler de uso que Claude Code añade al texto `tool_result`, así que renderice desde `tool_use_result` en lugar de analizar ese texto.

Para una herramienta MCP cuyo resultado contiene bloques `resource_link`, `tool_use_result` es un objeto con una matriz `resourceLinks` de entradas [`SDKMcpResourceLink`](#sdkmcpresourcelink). Claude recibe cada enlace como una línea de texto en el bloque `tool_result`, así que lea `resourceLinks` para renderizar los archivos que el servidor devolvió en lugar de analizar ese texto. Claude Code omite `resourceLinks` cuando el resultado no tiene enlaces y en resultados de subagentes, mantiene como máximo 50 enlaces por resultado, y deja de añadir enlaces una vez que la matriz alcanza 64 KiB de JSON serializado. `resourceLinks` requiere Agent SDK v0.3.257 o posterior.

Establezca `inline_pastes` para indicar a Claude Code qué partes de `message.content` el usuario pegó en lugar de escribir, una cadena por pegado. El texto del prompt permanece donde el usuario lo puso. Claude Code puede envolver cada pegado listado en etiquetas `<pasted_content>` donde se encuentra, para que Claude pueda distinguir el material pegado de las propias palabras del usuario. Solo los pegados en el último bloque de texto del prompt se envuelven. Requiere TypeScript Agent SDK v0.3.280 o posterior.

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

Un turno de usuario inyectado desde fuera de la sesión, uno cuyo [`origin`](#sdkmessageorigin) es `peer` o `channel`, llega a la transmisión como una reproducción ya sea que se entregara durante un turno activo o iniciara un nuevo turno mientras la sesión estaba inactiva. Antes de v2.1.207, un turno inyectado entregado mientras la sesión estaba inactiva no producía ningún mensaje en la transmisión y solo aparecía cuando volvía a leer la transcripción.

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
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
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
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

Varios campos en el resultado llevan detalles de diagnóstico más allá de `subtype`:

* `api_error_status`: el código de estado HTTP del error de API que terminó la conversación. Ausente o `null` cuando el turno terminó sin un error de API.
* `ttft_ms`: tiempo hasta el primer token en milisegundos, medido cuando llega el primer mensaje del asistente completo. Presente solo en el brazo de éxito.
* `ttft_stream_ms`: tiempo en milisegundos hasta el primer evento de transmisión `message_start`, cuando se abre la transmisión de respuesta. Menor que `ttft_ms`; la brecha entre los dos es el tiempo dedicado a transmitir el primer mensaje. Presente solo en el brazo de éxito.
* `user_message_uuid`: el `uuid` del mensaje que envió que este turno respondió. Consulte [`user_message_uuid`](#user_message_uuid) para saber qué resultados lo llevan.
* `user_message_uuids`: los `uuid`s de cada mensaje que envió que Claude Code respondió en este turno. Consulte [`user_message_uuids`](#user_message_uuids).
* `request_sent_wall_ms`: milisegundos de época en los que Claude Code envió la solicitud de API, para uniones contra marcas de tiempo del lado del servidor. Presente solo junto con [`user_message_uuid`](#user_message_uuid), en un resultado de éxito con `is_error` false cuyo turno envió una solicitud de API.
* `first_content_frame_ms`: tiempo en milisegundos hasta el primer evento de transmisión `content_block_start` o `content_block_delta`, contando bloques de pensamiento como contenido. Presente solo en el brazo de éxito, cuando `is_error` es false. Requiere Agent SDK v0.3.260 o posterior.
* `first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms`: tiempos para cargar el primer evento de transmisión del turno. Claude Code los registra solo en sesiones que transmite a claude.ai, como [sesiones en la nube](/docs/es/claude-code-on-the-web), y los resultados que `query()` produce no los llevan. Requiere Agent SDK v0.3.260 o posterior.
* `usage`: solo bucle del agente principal. Excluye llamadas de subagente y modelo auxiliar, y es por turno en sesiones de entrada de transmisión. Prefiera `modelUsage` para contabilidad de tokens/costos.
* `modelUsage`: totales por modelo para cada llamada de modelo realizada a través de la canalización de consultas durante esta llamada `query()`, incluido el bucle principal, subagentes y llamadas internas como compactación y agentes de Workflow. Las llamadas auxiliares fuera de esa canalización, como el clasificador de permisos y solicitudes de conteo de tokens, se excluyen. Una llamada que reanuda una sesión también cuenta los [totales por modelo restaurados de las llamadas anteriores de la sesión](/docs/es/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). En sesiones de entrada de transmisión los totales son acumulativos entre turnos, así que lea el resultado más reciente en lugar de sumar entre resultados. Consulte [Rastrear costos en modo de entrada de transmisión](/docs/es/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) para reinicializaciones y [Recuperar totales después de un bloqueo de sesión](/docs/es/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) para resultados puestos a cero.
* `total_cost_usd`: costo estimado acumulativo en USD, cubriendo las mismas llamadas que `modelUsage` y reiniciado en los mismos puntos. Una llamada que reanuda una sesión también cuenta los [totales restaurados de las llamadas anteriores de la sesión](/docs/es/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Es una estimación, no una declaración de facturación. Consulte [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking) para advertencias de precisión.
* `queued_turn_count`: el número de mensajes que envió con `origin: { kind: "human" }` que aún están esperando cuando Claude Code produjo el resultado. Consulte [`queued_turn_count`](#queued_turn_count) para saber qué significan `0` y un campo ausente.
* `startup_failure_reason`: por qué Claude Code se negó a iniciar, en el resultado `error_during_execution` que escribe antes de salir en un fallo de inicio conocido. Consulte [`startup_failure_reason`](#startup_failure_reason) para los valores y qué fallos lo llevan. Requiere Agent SDK v0.3.274 o posterior.
* `terminal_reason`: por qué terminó el bucle. Uno de `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, o `"turn_setup_failed"`.
* `fast_mode_state`: uno de `"on"`, `"off"`, o `"cooldown"`.
* `fast_mode_disabled_reason`: por qué [el modo rápido](/docs/es/fast-mode) no está disponible ahora. Ausente cuando nada bloquea el modo rápido, aunque una solicitud aún puede ejecutarse a velocidad estándar. Durante el enfriamiento después de un límite de velocidad de modo rápido, Claude Code reporta `fast_mode_state: "cooldown"` sin código de razón y vuelve a habilitar el modo rápido cuando expira el enfriamiento. Requiere Claude Code v2.1.219 o posterior.

Use el código de razón para explicar por qué el modo rápido está desactivado en su propia interfaz en lugar de volver a derivar la disponibilidad. Cada código nombra la verificación que bloqueó el modo rápido:

| Código de razón        | Significado                                                                                                                                                    |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | La cuenta no tiene la suscripción pagada o créditos de uso que requiere el modo rápido                                                                         |
| `preference`           | La organización ha deshabilitado el modo rápido                                                                                                                |
| `extra_usage_disabled` | Los créditos de uso están desactivados para la cuenta                                                                                                          |
| `network_error`        | La [verificación de disponibilidad](/docs/es/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) no pudo alcanzar `api.anthropic.com`                          |
| `unknown`              | Claude Code no pudo determinar la disponibilidad                                                                                                               |
| `not_first_party`      | La sesión utiliza un proveedor que no es la API de Anthropic                                                                                                   |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/es/env-vars) está establecido                                                                                               |
| `model_not_allowed`    | El modelo Opus de modo rápido no está en la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de la organización              |
| `sdk_opt_in_required`  | La sesión no ha optado por el modo rápido: pase `fastMode: true` en la opción [`settings`](#options) o a través de [`applyFlagSettings()`](#applyflagsettings) |
| `pending`              | La verificación de disponibilidad aún no se ha completado                                                                                                      |

El mismo par de campos aparece en [`SDKSystemMessage`](#sdksystemmessage) y en [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse), para que pueda leer el estado del modo rápido antes del primer turno.

El campo `origin` reenvía el [`SDKMessageOrigin`](#sdkmessageorigin) del mensaje del usuario que activó este resultado. Cuando el SDK inyecta un turno de seguimiento sintético, como para una tarea de fondo terminada, el `SDKResultMessage` resultante lleva `origin: { kind: "task-notification" }`. Las rutinas cuyo disparador se activó y los mensajes verificados por el servidor de sus otras sesiones llegan con este tipo también, cada uno con el `subkind` descrito en [Subtipos de notificación de tarea](#task-notification-subkinds). Verifique `kind` para distinguir los resultados que responden a su prompt de los seguimientos inyectados antes de enrutarlos o suprimirlos. Si su aplicación [declara ejecuciones programadas](#declare-a-scheduled-run), sus resultados también llevan `kind: "task-notification"`, así que no suprima solo en `kind`.

Cuando varias finalizaciones de tareas de fondo se ponen en cola juntas, Claude Code puede responderlas en un turno en lugar de uno cada una. Cada finalización aún produce su propio resultado con este origen. Todos excepto el último de los completados que Claude Code responde juntos producen resultados vacíos con `num_turns: 0`, en orden, y el resultado del último lleva el turno que responde a todos ellos.

El campo está ausente para resultados emitidos antes de cualquier turno de usuario, como errores de inicio.

Cuando un hook `PreToolUse` devuelve `permissionDecision: "defer"`, el resultado tiene `stop_reason: "tool_deferred"` y `deferred_tool_use` lleva el `id`, `name` e `input` de la herramienta pendiente. Lea este campo para mostrar la solicitud en su propia interfaz, luego reanude con el mismo `session_id` para continuar. Consulte [Diferir una llamada de herramienta para más tarde](/docs/es/hooks#defer-a-tool-call-for-later) para el viaje completo.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

El `uuid` del [`SDKUserMessage`](#sdkusermessage) que el turno está respondiendo, repetido para que pueda hacer coincidir la respuesta de Claude Code con el mensaje que envió. Claude Code repite un `uuid` solo si establece uno en el mensaje. El campo es opcional en `SDKUserMessage`, y un prompt de cadena pasado a `query()` no lleva ninguno.

Qué mensaje de los suyos responde un turno depende de cómo comenzó el turno:

* **Un mensaje regular que envió**, es decir, uno sin `isSynthetic: true`: el turno responde ese mensaje durante toda su ejecución. Cuando envía varios mensajes juntos, Claude Code puede fusionarlos en un turno, y el campo entonces lleva solo el `uuid` del último mensaje. Para hacer coincidir la respuesta con cualquiera de los mensajes fusionados, use [`user_message_uuids`](#user_message_uuids).
* **Un mensaje que envió con `isSynthetic: true`**: el turno responde ese mensaje al principio. Si Claude Code recoge un mensaje regular suyo entre llamadas de herramientas, el turno responde el mensaje recogido a partir de entonces. Repetir el `uuid` de un mensaje sintético requiere Agent SDK v0.3.265 o posterior; las versiones anteriores no repiten nada en turnos sintéticos.
* **Un prompt que Claude Code generó por sí mismo**, como el turno que continúa el trabajo interrumpido después de que se reinicia una sesión: el turno no responde ningún mensaje suyo al principio y sus marcos no llevan ningún eco. Si Claude Code recoge un mensaje regular suyo entre llamadas de herramientas, el turno responde ese mensaje a partir de entonces. El eco de recogida requiere Agent SDK v0.3.265 o posterior; las versiones anteriores no repiten nada en estos turnos.

Claude Code repite el `uuid` del mensaje respondido en tres tipos de marco:

* **El resultado**: cada resultado de un turno que respondió un mensaje que envió. Cada tal resultado lo lleva en Agent SDK v0.3.265 o posterior. Antes de v0.3.265, el resultado de éxito de un turno que un mensaje regular inició carecía de él cuando el turno no envió ninguna solicitud de API o terminó con una llamada de herramienta diferida. Antes de v0.3.246, los resultados de error también carecían de él, y antes de v0.3.216 cada resultado lo hacía.
* **La primera respuesta del turno**: el primer [mensaje del asistente](#sdkassistantmessage), o con `includePartialMessages` el primer [evento de transmisión](#sdkpartialassistantmessage) cuyo `event.type` no es `ping`, para que pueda vincular la respuesta antes de que llegue el resultado. Cuando un turno no transmite nada, Claude Code lo establece en el primer mensaje del asistente en su lugar. El eco de primera respuesta requiere Agent SDK v0.3.246 o posterior. Cuando el mensaje que el turno está respondiendo cambia a mitad del turno, la primera respuesta después del cambio también lleva el campo, en Agent SDK v0.3.265 o posterior; las versiones anteriores lo establecen en un marco de respuesta por turno.
* **Cada marco [`thinking_tokens`](#sdkthinkingtokensmessage) del turno**: para que pueda atribuir el progreso del pensamiento al mensaje que envió sin esperar la primera respuesta del turno. Requiere Agent SDK v0.3.260 o posterior.

Claude Code omite el campo en estos casos:

* Marcos de respuesta que no sean esas primeras respuestas
* Marcos de subagente
* Turnos que no responden ningún mensaje con un `uuid`: el turno respondió un mensaje que envió sin uno, o Claude Code inició el turno por sí mismo y no recogió ningún mensaje regular que tenga uno
* Resultados que no responden ningún mensaje que envió, como el resultado puesto a cero después de un bloqueo de proceso de trabajo

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

Los `uuid`s de cada mensaje que envió que Claude Code respondió en este turno. Cuando envía varios mensajes juntos, Claude Code puede fusionarlos en un turno, y `user_message_uuid` entonces nombra solo el último de ellos. Para hacer coincidir la respuesta con cualquiera de los mensajes fusionados, busque el `uuid` de ese mensaje en cualquier lugar de esta lista. Requiere Agent SDK v0.3.259 o posterior.

Claude Code establece la lista junto con `user_message_uuid` en cada marco de respuesta que lleva ese campo y en el resultado. Para el conjunto completo de marcos que llevan `user_message_uuid`, y la versión que cada uno requiere, consulte [`user_message_uuid`](#user_message_uuid). La lista siempre contiene `user_message_uuid` y contiene como máximo 64 entradas.

Cuando Claude Code recoge un mensaje regular que envió mientras se ejecutaba un turno, añade el `uuid` de ese mensaje a la lista del resultado.

Cuando una primera respuesta o resultado lleva `user_message_uuid` sin la lista, proviene de una versión anterior de Claude Code, así que recurra al campo único.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

El número de mensajes que envió con [`origin: { kind: "human" }`](#sdkmessageorigin) que aún están esperando en la cola de comandos cuando Claude Code produjo el resultado. Requiere Agent SDK v0.3.242 o posterior.

Qué significan `0` y un campo ausente:

* **`0`**: Claude Code no cuenta los mensajes que envió sin ese `origin`, y no cuenta notificaciones de tareas, así que un turno aún puede seguir.
* **Ausente**: el resultado final que Claude Code emite después de un bloqueo o error de inicio fatal omite el campo, y [puede llevar totales puestos a cero](/docs/es/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Por qué Claude Code se negó a iniciar, para que su aplicación pueda ofrecer la solución en lugar de un reintento. Claude Code lo establece en el resultado `error_during_execution` que escribe antes de salir en un fallo de inicio conocido. Ese resultado lleva totales puestos a cero, y su matriz `errors` lleva el mismo texto que stderr. El campo está ausente en cada otro resultado. Requiere Agent SDK v0.3.274 o posterior.

Establezca `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` en `1` en [`env`](#options) para recibir este resultado para cada valor `SDKStartupFailureReason`. Sin esa variable, Claude Code escribe el resultado solo para estos fallos, y el resto termina con salida stderr, una salida distinta de cero, y ningún mensaje de resultado:

* Una reanudación que Claude Code detiene porque no puede [devolver la sesión a su árbol de trabajo](/docs/es/worktrees#the-session-resumes-outside-its-worktree), con `worktree_unverified` o `worktree_resume_refused`. Esa sección dice qué error lleva qué valor.
* Una [`continue`](#options) rechazada de una conversación que una sesión de fondo mantiene, con `session_held_by_background`. Para una [`resume`](#options) rechazada de tal conversación, Claude Code escribe el resultado solo cuando la variable está establecida.

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

Cada valor nombra un rechazo:

| Valor                                  | Qué detuvo la sesión                                                                                                                                                                                                                                                      |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `org_pin_api_key_conflict`             | La configuración administrada [requiere un inicio de sesión de puerta de enlace de primera parte o en la nube](/docs/es/authentication#restrict-login-to-your-organization), y se configura una clave de API de Anthropic, token de autenticación o `apiKeyHelper` en su lugar |
| `org_verify_failed`                    | La organización del inicio de sesión no pudo verificarse contra el pin, por ejemplo debido a un fallo de red o un token revocado                                                                                                                                          |
| `org_pin_mismatch`                     | El inicio de sesión pertenece a una organización que el pin no permite                                                                                                                                                                                                    |
| `managed_settings_invalid`             | La configuración de política administrada no pudo leerse, o el pin no nombra ninguna organización                                                                                                                                                                         |
| `remote_settings_required_unavailable` | La configuración administrada que la organización requiere no pudo cargarse                                                                                                                                                                                               |
| `gateway_signin_required`              | La [puerta de enlace en la nube](/docs/es/claude-apps-gateway) terminó este inicio de sesión                                                                                                                                                                                   |
| `gateway_access_denied`                | La solicitud de configuración administrada a la puerta de enlace en la nube volvió con un 403, que la [tabla de solución de problemas](/docs/es/claude-apps-gateway-deploy#troubleshooting) de la puerta de enlace cubre                                                       |
| `proxy_invalid`                        | Una configuración de proxy no es una URL completa                                                                                                                                                                                                                         |
| `temp_dir_unusable`                    | El directorio temporal por usuario no es seguro o no pudo crearse                                                                                                                                                                                                         |
| `cwd_unavailable`                      | El directorio de trabajo fue eliminado, movido o no se puede leer                                                                                                                                                                                                         |
| `shell_tool_missing`                   | En Windows, no hay herramienta de shell disponible: Git Bash falta, y PowerShell falta o está desactivado con `CLAUDE_CODE_USE_POWERSHELL_TOOL`                                                                                                                           |
| `session_held_by_background`           | La conversación a reanudar o continuar se ejecuta como una [sesión de fondo](/docs/es/agent-view)                                                                                                                                                                              |
| `worktree_resume_refused`              | El árbol de trabajo de la sesión falló sus verificaciones de seguridad, o la reanudación se lanzó desde dentro de él. `errors` dice si ejecutar la misma reanudación nuevamente continúa sin el árbol de trabajo                                                          |
| `worktree_unverified`                  | El árbol de trabajo de la sesión no pudo verificarse ahora, y reintentar puede tener éxito                                                                                                                                                                                |
| `cli_version_too_old`                  | Esta versión de Claude Code está por debajo del mínimo que Anthropic requiere                                                                                                                                                                                             |
| `bypass_root`                          | Se solicitó el modo de permisos de derivación mientras se ejecutaba como root                                                                                                                                                                                             |

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
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` reporta el estado del [modo rápido](/docs/es/fast-mode) de la sesión. Cuando algo bloquea el modo rápido, `fast_mode_disabled_reason` nombra la verificación que lo bloqueó; el campo requiere Claude Code v2.1.219 o posterior. Para los códigos de razón y sus significados, consulte [`fast_mode_disabled_reason`](#sdkresultmessage) en el mensaje de resultado.

`terminal_slash_commands` nombra las entradas en `slash_commands` cuya interfaz está vinculada a la terminal local, como `exit`. Puede enviarlas como cualquier otra entrada en `slash_commands`; el campo existe para que un cliente remoto o móvil pueda ocultarlas de sus menús de comandos. El campo está presente solo cuando no está vacío, y requiere Agent SDK v0.3.229 o posterior.

*

`source` en cada entrada `mcp_servers`: de dónde proviene la definición del servidor, con los mismos valores que [`McpServerStatus`](#mcpserverstatus)'s `source`. Requiere Agent SDK v0.3.274 o posterior.

*

`effort`: el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) que Claude Code envía en la siguiente solicitud de la sesión, o `null` cuando no envía ninguno. Claude Code establece el campo solo en el mensaje de inicialización que envía a clientes de [Control Remoto](/docs/es/remote-control), y lo omite del mensaje de inicialización que su aplicación lee. Requiere Agent SDK v0.3.234 o posterior.

La matriz `capabilities` nombra los comportamientos del protocolo que implementa este CLI, para que pueda detectar características en lugar de comparar cadenas `claude_code_version`. Es un conjunto abierto: ignore los valores que no reconozca, y verifique la capacidad específica cuyo comportamiento depende. El campo requiere Claude Code v2.1.205 o posterior y está ausente en CLI anteriores.

| Capacidad                    | Significado                                                                                                                                                                                                                                                                                           |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) se resuelve con un recibo [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) que enumera los mensajes que estaban pendientes cuando llegó la interrupción                                                                                                   |
| `interrupt_cancel_queued_v1` | La solicitud de control `interrupt` honra `cancel_queued: true`, cancelando los mensajes que el recibo enumeraría bajo `still_queued` y enumerándolos bajo `cancelled` en su lugar. Consulte [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse). Requiere Claude Code v2.1.219 o posterior |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Mensaje parcial de transmisión (solo cuando `includePartialMessages` es true). El campo `parent_tool_use_id` siempre es `null`: los eventos de transmisión se emiten solo para la sesión principal. Para atribución de subagente, use mensajes completos, que llevan `parent_tool_use_id`, o habilite [`forwardSubagentText`](#options) para recibir texto y pensamiento de subagente como mensajes completos.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // From Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Time to first token in ms, present only on message_start events
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code establece `user_message_uuid` y `user_message_uuids` en el primer evento de transmisión que no es ping del turno, y nuevamente cuando el mensaje que el turno está respondiendo cambia, bajo las condiciones en [`user_message_uuid`](#user_message_uuid).

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

Banner de texto genérico emitido por el bucle. Lleva líneas de estado no erróneas, retroalimentación de hooks como la razón de bloqueo de un hook `UserPromptSubmit`, y salida de comandos. En Claude Code v2.1.227 o posterior, el [`systemMessage`](/docs/es/hooks#json-output) de un hook puede llegar como este mensaje, con cada línea prefijada por el nombre del hook, como `PostToolUse:Bash says:`. Si el `systemMessage` de un hook llega como este mensaje depende del evento. Cada [sección de evento](/docs/es/hooks#hook-events) en la página de hooks dice cómo se muestra la salida. Renderice `content` como texto sin formato en el `level` dado.

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

Emitido en el desmontaje gracioso del trabajador para que los clientes remotos puedan mostrar por qué el trabajador salió en lugar de esperar el tiempo de espera del latido. La `reason` es una cadena corta en snake\_case establecida por el CLI del host, como `"host_exit"` o `"remote_control_disabled"`. Actúe sobre esto solo cuando transmita en vivo. Una sesión reanudada reproduce instancias pasadas de este mensaje, así que ignórelas en ese caso.

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

Evento de progreso de instalación de plugin. Emitido cuando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/es/env-vars) está establecido, para que su aplicación Agent SDK pueda rastrear la instalación de plugin del mercado antes del primer turno. Los estados `started` y `completed` cierran la instalación general. Los estados `installed` y `failed` reportan mercados individuales e incluyen `name`.

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

Evento de transmisión emitido cuando el sistema de permisos deniega una llamada de herramienta sin un prompt interactivo. Úselo para renderizar la denegación en su interfaz a medida que sucede, en lugar de solo observar el resultado de herramienta `is_error` que sigue. Qué denegaciones reporta depende de cómo la ejecución maneja los prompts de permiso:

* **Con una devolución de llamada [`canUseTool`](#canusetool) y el [`permissionPrompts: 'host'`](#options) predeterminado**: los prompts de permiso van a su devolución de llamada, y este evento reporta las denegaciones que Claude Code decide por sí solo sin llamarla.
*

**Sin ninguno**: una ejecución `-p` desnuda, o `query()` que no establece ni `canUseTool` ni `permissionPromptToolName`, deniega cualquier llamada de herramienta que habría solicitado, y este evento reporta esas denegaciones así como las que Claude Code decide por sí solo. Antes de v2.1.223, Claude Code no emitía este evento en ejecuciones sin una devolución de llamada.

* **Con una herramienta de prompt MCP**, establecida con `permissionPromptToolName` o la bandera [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags), y el `permissionPrompts: 'host'` predeterminado: Claude Code no emite este evento en absoluto, ni siquiera para las denegaciones de regla que decide por sí solo.
*

**Con [`permissionPrompts: 'none'`](#options)**: Claude Code deniega las llamadas que habrían solicitado, incluso cuando `canUseTool` o una herramienta de prompt MCP también está establecida, y este evento reporta esas denegaciones así como las que Claude Code decide por sí solo. Requiere Claude Code v2.1.259 o posterior.

En cada configuración, este evento omite cualquier denegación decidida en la ruta del hook `PreToolUse`, ya sea que el hook denegara la llamada por sí solo o una regla de denegación anulara la decisión de permitir o preguntar del hook. El evento también es de mejor esfuerzo: ocasionalmente Claude Code registra una denegación sin emitir este evento, así que `permission_denials` en el [mensaje de resultado](#sdkresultmessage) es el registro autorizado.

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
| `decision_reason`      | `string` | Razón legible por humanos del componente que decidió, cuando está disponible                                                                          |
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

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

Forma estructurada del informe `/context`, llevada como `context_usage` en el [`SDKAssistantMessage`](#sdkassistantmessage) que entrega un resultado `/context`. Agent SDK v0.3.232 y posterior exportan el tipo. A diferencia de [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse), lleva solo los datos necesarios para renderizar el desglose de uso, sin campos de visualización como `color` y `gridRows`.

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

La tabla enumera lo que Claude Code pone en cada campo. Los campos de `model` a `over_limit` describen la sesión en su conjunto, y los campos de colección atribuyen tokens a elementos individuales.

| Campo            | Tipo                                                      | Descripción                                                                                                                                                                                                                                                                                                                         |
| ---------------- | --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | El modelo del bucle principal para el cual Claude Code calculó el uso, no el de un subagente                                                                                                                                                                                                                                        |
| `total_tokens`   | `number`                                                  | La estimación de Claude Code de los tokens en uso. No se fija a la ventana, por lo que puede exceder `raw_max_tokens` cuando la sesión está sobre el límite                                                                                                                                                                         |
| `raw_max_tokens` | `number`                                                  | La ventana de contexto del modelo, o la [ventana de auto-compactación](/docs/es/model-config#context-window-and-auto-compaction) más baja cuando se aplica una, como la que estableció o el límite de 200K que Claude Code aplica a algunos modelos con una ventana de 1M de tokens. Claude Code mide `total_tokens` contra esta ventana |
| `percentage`     | `number`                                                  | `total_tokens` como un porcentaje redondeado de `raw_max_tokens`, por lo que puede exceder 100 cuando la sesión está sobre el límite                                                                                                                                                                                                |
| `over_limit`     | `object`                                                  | Presente solo cuando `total_tokens` excede `raw_max_tokens`. `tokens_over` es la cantidad sobre, y `kind` dice cómo Claude Code resolvió la ventana                                                                                                                                                                                 |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | Una entrada por fila del desglose de uso por categoría                                                                                                                                                                                                                                                                              |
| `mcp_tools`      | `object[]`                                                | Tokens atribuidos a cada herramienta MCP, con su nombre de cable, como `mcp__linear__create_issue`, y su `server_name`                                                                                                                                                                                                              |
| `memory_files`   | `object[]`                                                | Tokens atribuidos a cada archivo de memoria cargado, con su `path` y una etiqueta de fuente como `Project` o `User` en `type`                                                                                                                                                                                                       |
| `agents`         | `object[]`                                                | Tokens atribuidos a cada definición de subagente personalizado, con un identificador de fuente como `projectSettings`, `userSettings`, o `plugin`. Los subagentes integrados no se enumeran                                                                                                                                         |
| `skills`         | `object[]`                                                | Tokens atribuidos a cada habilidad en la lista de habilidades, con un identificador de fuente y, para habilidades de plugin, el nombre del plugin en `plugin_name`. Ausente cuando ninguna habilidad contribuye tokens                                                                                                              |

`over_limit.kind` registra cómo Claude Code resolvió la ventana, no si la API acepta la siguiente solicitud:

* `hard_limit`: la ventana es lo que Claude Code cree que es el límite propio del modelo, más allá del cual la API rechaza solicitudes
* `compaction_window`: la ventana es una ventana de política de compactación, que puede o no coincidir con el límite del modelo

Claude Code evoluciona el tipo de manera aditiva, añadiendo nuevos datos como campos opcionales en lugar de remodelar los existentes. Lea los campos que conoce e ignore cualquiera que no reconozca.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

Una fila del desglose de uso por categoría `/context`.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

La tabla enumera lo que Claude Code pone en cada campo de una fila.

| Campo    | Tipo     | Descripción                                                                                                                       |
| -------- | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | El nombre de visualización de la fila como `/context` lo imprime, como `Messages`. Clasifique las filas por `kind`, no por nombre |
| `tokens` | `number` | El recuento de tokens de la fila. Las filas pueden llevar cero tokens                                                             |
| `kind`   | `string` | Lo que representa la fila: `used`, `free`, `buffer`, o `deferred`                                                                 |

Cada valor `kind` dice qué son los tokens de la fila:

* `used`: contenido que ocupa la ventana de contexto
* `free`: la ventana restante
* `buffer`: la reserva de compactación
* `deferred`: esquemas de herramientas que Claude Code mantiene fuera de la ventana y excluye del cálculo de uso, enumerados para conciencia

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Procedencia de un mensaje de rol de usuario. Esto aparece como `origin` en [`SDKUserMessage`](#sdkusermessage) y se reenvía al [`SDKResultMessage`](#sdkresultmessage) correspondiente para que pueda saber qué activó un turno dado.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | Significado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Entrada directa del usuario final. Si su aplicación reenvía lo que el usuario escribió como un mensaje de usuario, establezca su `origin` en `{ kind: "human" }` explícitamente: Claude Code trata un mensaje de usuario sin `origin` como no atribuido, y verifica que requieren un prompt escrito por humanos, como la [palabra clave de flujo de trabajo `ultracode`](/docs/es/workflows#ask-for-a-workflow-in-your-prompt), no lo aceptan. Antes de v2.1.210, Claude Code trataba un `origin` ausente en un mensaje de usuario como entrada humana. |
| `channel`           | Mensaje que llega en un [canal](/docs/es/channels). `server` es el nombre del servidor MCP de origen.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `peer`              | Mensaje de otro agente: un [compañero](/docs/es/agent-teams) en proceso o un [par entre sesiones](/docs/es/cross-session-messaging), otra de sus sesiones de Claude Code. Consulte [Campos de origen de par](#peer-origin-fields) para la semántica por campo y el modelo de confianza.                                                                                                                                                                                                                                                                      |
| `task-notification` | Turno sintético inyectado para una entrega que llega sin un prompt de usuario fresco, como una tarea de fondo terminada; consulte [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) para ese brazo. Un prompt que su aplicación [declara como una ejecución programada](#declare-a-scheduled-run) también lleva este tipo. El `subkind` opcional marca qué generó la notificación. Consulte [Subtipos de notificación de tarea](#task-notification-subkinds).                                                                            |
| `coordinator`       | Mensaje de un coordinador de equipo en un [equipo de agentes](/docs/es/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `auto-continuation` | Turno sintético inyectado cuando la sesión continúa sin entrada de usuario fresca, como un resultado de comando que activa un prompt de seguimiento.                                                                                                                                                                                                                                                                                                                                                                                               |
| `unclassified`      | Turno inyectado cuyo origen no pudo determinarse. Requiere Claude Code v2.1.223 o posterior. Cuando Claude Code recibe un [`SDKUserMessage`](#sdkusermessage) con `isSynthetic: true` y no puede clasificarlo como ningún otro `kind`, establece este tipo a medida que llega el mensaje y enmarca el turno al modelo como una fuente no usuario en lugar de tratarlo como entrada humana. Su aplicación no debe establecer este valor.                                                                                                            |

<h3 id="task-notification-subkinds">
  Subtipos de notificación de tarea
</h3>

Cuando Claude Code entrega una notificación de tarea en una sesión, establece `subkind` en el `origin` de la notificación si los servidores de Anthropic verificaron de dónde provino esa notificación. También establece `subkind` cuando su aplicación [declara el mensaje como una ejecución programada](#declare-a-scheduled-run) por sí misma, lo que requiere TypeScript Agent SDK v0.3.280 o posterior. `subkind` requiere Claude Code v2.1.213 o posterior, y toma uno de dos valores:

* `scheduled-trigger`: la notificación es un prompt almacenado de una [rutina](/docs/es/routines), entregado porque uno de los disparadores de la rutina se activó: su horario, su [disparador de API](/docs/es/routines#add-an-api-trigger), su [disparador de GitHub](/docs/es/routines#add-a-github-trigger), o **Ejecutar ahora**. Un prompt que su aplicación [declara como una ejecución programada](#declare-a-scheduled-run) también lleva este valor. Claude Code enmarca estos al modelo como la tarea asignada de la sesión, con un aviso diferente del [aviso que otras notificaciones de tarea llevan](#sdktasknotificationmessage).
*

`peer-send-message`: la notificación es un mensaje que otra de sus sesiones envió con la herramienta `send_message` del lado del servidor que [sesiones en la nube](/docs/es/claude-code-on-the-web) usan para mensajearse entre sí, no la [herramienta `SendMessage` entre sesiones](/docs/es/cross-session-messaging), y los servidores de Anthropic verificaron que ambas sesiones pertenecen al mismo grupo privado de sesiones. Requiere Claude Code v2.1.224 o posterior. Una entrega `send_message` que los servidores no verificaron de esa manera no obtiene ningún subkind.

Cada otra notificación de tarea no tiene `subkind`. Eso incluye [actividad de PR](/docs/es/claude-code-on-the-web#how-claude-responds-to-pr-activity) entregada en una sesión y eventos de fondo como una tarea terminada. Los mensajes de la [herramienta `SendMessage` entre sesiones](/docs/es/cross-session-messaging) no son notificaciones de tarea en absoluto: ya sea que provengan de una sesión en la misma máquina o a través de servidores de Anthropic desde otra máquina, Claude Code les da `kind: "peer"` y los [campos de origen de par](#peer-origin-fields).

`fireReason` dice por qué se activó una notificación `scheduled-trigger`, como un token en minúsculas corto como `scheduled`, `manual`, `retry`, `catch_up`, o `api`. Los servidores de Anthropic lo establecen en las entregas de una [rutina](/docs/es/routines), y su aplicación lo establece cuando declara una ejecución programada. Está ausente cuando ninguno envió uno. Requiere TypeScript Agent SDK v0.3.280 o posterior.

<h4 id="declare-a-scheduled-run">
  Declarar una ejecución programada
</h4>

Si su aplicación ejecuta prompts en su propio horario, declare cada ejecución para que Claude Code enmarque el turno al modelo como una tarea programada en lugar de como entrada en vivo del usuario. Inicie la sesión con `CLAUDE_CODE_HOST_SCHEDULED_RUN` establecido en `1` en [`env`](#options), luego envíe el [`SDKUserMessage`](#sdkusermessage) de la ejecución con `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` y sin `isSynthetic`. Claude Code ignora la declaración en un proceso iniciado sin esa variable. También la ignora en un proceso cuyo entorno lleva [`CLAUDECODE`](/docs/es/env-vars) o `CLAUDE_CODE_CHILD_SESSION`. Claude Code mantiene `fireReason` solo cuando el valor es 1 a 32 letras minúsculas o guiones bajos. Requiere TypeScript Agent SDK v0.3.280 o posterior.

<h3 id="peer-origin-fields">
  Campos de origen de par
</h3>

Un origen `peer` identifica qué agente envió el mensaje: un [compañero](/docs/es/agent-teams) en proceso enviando a `main` con `SendMessage`, o un [par entre sesiones](/docs/es/cross-session-messaging), otra de sus sesiones de Claude Code. Los pares entre sesiones requieren Claude Code v2.1.224 o posterior en macOS y Linux; consulte [disponibilidad de mensajería entre sesiones](/docs/es/cross-session-messaging#availability) para el requisito de Windows nativo. Un par entre sesiones puede ejecutarse en la misma máquina, o en [otra de sus máquinas](/docs/es/cross-session-messaging#message-sessions-on-other-machines) o [en la nube](/docs/es/claude-code-on-the-web) cuando su mensaje llega a través de Control Remoto. Los dos tipos de remitente llenan los campos de manera diferente:

* `from`: el nombre del compañero, o la dirección del remitente para un par entre sesiones. Para un [mensaje unidireccional entre máquinas](/docs/es/cross-session-messaging#message-sessions-on-other-machines), el remitente no tiene dirección de respuesta y `from` es `"unknown"`. El valor es creado por el remitente; `verifiedPeerPid` es la identidad verificada.
*

`fromMode`: la clase de permiso de la sesión de envío, `bypass` o `prompting`, declarada por un host que retransmite un mensaje de par entre sus sesiones, como la [aplicación de escritorio](/docs/es/desktop#work-across-sessions). Claude Code lo lee en la sesión receptora cuando aplica los [controles de entrada](/docs/es/cross-session-messaging#control-inbound-messages). Requiere Agent SDK v0.3.234 o posterior.

* `senderTaskId`: el ID de tarea del compañero. Ausente para un par entre sesiones.
*

`name`: el nombre de visualización del remitente, normalizado por Claude Code: elimina puntos de código de control, formato, sustituto, y separador de línea o párrafo Unicode, luego recorta el resultado y lo limita a 64 puntos de código con puntos suspensivos. Requiere Claude Code v2.1.205 o posterior.

*

`body`: el cuerpo del mensaje decodificado con la envoltura de par eliminada, byte exacto con lo que el modelo ve. Siempre presente para un mensaje de compañero; para un par entre sesiones, presente solo cuando el turno es exactamente una envoltura de par formada por Claude Code. Renderice `name` y `body` en lugar de volver a analizar el texto del mensaje. Requiere Claude Code v2.1.205 o posterior.

*

`fromSession`: el ID de sesión abierto por host del remitente, establecido por el host del remitente para que su interfaz pueda vincular de nuevo a la sesión de envío. Como `from`, es afirmado por el remitente: úselo como destino de navegación solo, y no lo trate como prueba de la identidad del remitente. Requiere Claude Code v2.1.216 o posterior.

*

`verifiedPeerPid`: el ID de proceso del proceso que se conectó al socket de mensajería entre sesiones de esta sesión, verificado por el kernel y leído de la conexión misma, nunca de la carga útil. Úselo, no `from`, para identificar al remitente: `from` es falsificable por cualquier proceso del mismo usuario. El campo está ausente cuando Claude Code no puede verificarlo, como en Windows o entrada que no es socket, así que un valor ausente significa que el remitente no está verificado. Para tráfico retransmitido identifica el relé en lugar del autor del mensaje, y los ID de proceso son reciclables, así que trátelo como procedencia en lugar de un token de autenticación. Requiere Claude Code v2.1.216 o posterior.

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
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
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
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
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
  mcp_server?: McpServerProvenance;
};
```

`mcp_server` está presente cuando la herramienta proviene de un servidor MCP; vea [`McpServerProvenance`](#mcpserverprovenance). Las entradas `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` y `PermissionDenied` llevan el mismo campo. El campo requiere Agent SDK v0.3.274 o posterior.

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
  mcp_server?: McpServerProvenance;
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
  mcp_server?: McpServerProvenance;
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

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
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
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
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

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
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

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

Se activa antes de que un cambio de modelo solicitado surta efecto. `context_tokens` y los campos posteriores estiman lo que cuesta reenviar la conversación al nuevo modelo. Para las descripciones completas de campos y semántica de bloqueo, vea [PreModelSwitch](/docs/es/hooks#premodelswitch).

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

Se activa después de que el modelo de la sesión cambia. Lleva los mismos campos que `PreModelSwitchHookInput`, con dos valores de `source` más. Vea [PostModelSwitch](/docs/es/hooks#postmodelswitch).

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
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
  mcp_server?: McpServerProvenance;
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

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated since v2.1.178. Carries the session-derived team name; will be removed. */
  team_name?: string;
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

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
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

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` es la ruta absoluta del directorio que se agregó. `source` es `"slash_command"` cuando `/add-dir` lo agregó y `"register_repo_root"` cuando la solicitud de control del SDK lo hizo.

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

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
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
  /**
   * Una secuencia de escape de terminal (p. ej. OSC 9 / OSC 777 notificación de escritorio)
   * para que Claude Code emita en su nombre. Solo se permiten OSCs de notificación/título
   * (0, 1, 2, 9, 99, 777) y BEL; un valor que contenga cualquier otra cosa se ignora en su totalidad. Solo la CLI interactiva lo emite;
   * el SDK ignora el campo.
   */
  terminalSequence?: string;
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
        sessionTitle?: string;
        /** Cuando la decisión es "block", omita el mensaje original del bloque. */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * Vuelva a escanear los directorios de skill y comando después de que se completen los hooks de SessionStart,
         * para que los skills instalados por el hook estén disponibles en la
         * misma sesión.
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * El mismo contrato que PreToolUse: "allow" procede, "deny" cancela
         * el cambio, "ask" le pide al usuario que confirme. Solo /model en una
         * sesión interactiva muestra ese mensaje; todas las otras superficies,
         * incluidas las solicitudes set_model, tratan "ask" como un rechazo.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** Llega al modelo con la siguiente solicitud que sirve el nuevo modelo. */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * Nota breve sobre el resultado de esta llamada de herramienta para el clasificador de permisos del modo automático. Limitado a 2000 caracteres, compartido entre
         * todos los hooks que responden a la misma llamada; honrado solo en respuestas de hooks síncronos. No copie salida de herramienta no confiable en él.
         */
        classifierContext?: string;
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
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
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
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** Texto mostrado en lugar del delta. Omita (o devuelva el delta sin cambios) para mostrar el original. */
        displayContent?: string;
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

Unión de tipos de entrada de herramienta exportados desde `@anthropic-ai/claude-agent-sdk`; los miembros incluyen:

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nombre de herramienta:** `Agent`. El nombre anterior `Task` aún se acepta como alias, y el array `tools` en el mensaje de inicialización [`SDKSystemMessage`](#sdksystemmessage) actualmente enumera esta herramienta como `Task` para compatibilidad hacia atrás.

<Note>
  El campo `mode` está deprecado e ignorado en Claude Code v2.1.212 o posterior. Un subagente se ejecuta en el modo de permiso de la sesión padre o en la [`permissionMode`](#agentdefinition) de su definición, y las [reglas de herencia de subagentes](/docs/es/agent-sdk/permissions#available-modes) deciden cuál.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Deprecated; ignored
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Deprecated; ignored. The subagent inheritance rules decide a subagent's permission mode
  isolation?: "worktree" | "remote";
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
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
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

Ejecuta comandos Bash con tiempo de espera opcional y ejecución en segundo plano. El directorio de trabajo persiste entre comandos, incluyendo comandos ejecutados en turnos posteriores de una sesión de múltiples turnos; el estado del shell como variables de entorno exportadas no. Para los límites sobre qué cambios de directorio se mantienen, vea [Lo que persiste entre comandos](/docs/es/tools-reference#what-persists-between-commands).

<h3 id="monitor">
  Monitor
</h3>

**Nombre de herramienta:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

Ejecuta una fuente de fondo y entrega cada evento a Claude para que pueda reaccionar sin sondeo: `command` ejecuta un script y emite un evento por línea de stdout, y `ws` abre un WebSocket y emite un evento por marco de texto. Proporcione exactamente uno de `command` o `ws`. La fuente `ws` requiere Claude Code v2.1.195 o posterior.

`timeout_ms` es el plazo de la vigilancia en milisegundos. El valor predeterminado es 300000 y acepta valores hasta 3600000. El plazo efectivo es como máximo 1800000, que son 30 minutos, por lo que un valor aceptado más grande se acorta a eso. En el plazo, la vigilancia termina y Claude recibe un aviso para que pueda iniciar una nueva vigilancia si aún la necesita.

El tipo exportado marca `timeout_ms` como requerido porque el esquema completa el valor predeterminado; una llamada que lo omite valida.

Cuando Monitor ejecuta un comando, sigue las mismas reglas de permiso que Bash; una vigilancia de WebSocket solicita aprobación por separado. Vea la [referencia de herramienta Monitor](/docs/es/tools-reference#monitor-tool) para comportamiento y disponibilidad de proveedor.

<h3 id="taskoutput">
  TaskOutput
</h3>

Eliminado en Claude Code v2.1.277, junto con su tipo `TaskOutputInput`. Anteriormente recuperaba salida de una tarea de fondo en ejecución o completada; Claude lee el archivo de salida de una tarea de fondo con `Read` en su lugar.

Una entrada `disallowedTools` o una regla de denegación que aún nombre `TaskOutput` se ignora sin una advertencia.

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

Para un PDF, Claude recibe el contenido del archivo dentro del `tool_result` de la llamada Read. Una lectura que devuelve la salida `pdf` [output](#tool-output-types) lleva un bloque `text` de resumen seguido de un bloque `document`. Una que devuelve la salida `parts` lleva el bloque `text` de resumen seguido de un bloque por página extraída: un bloque `image`, o un bloque `text` nombrando la página cuando Claude Code no pudo renderizarla como imagen. Antes de Agent SDK v0.3.242, Claude Code entregaba el contenido del archivo como un mensaje `user` separado después del resultado de la herramienta.

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
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
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
  shell_id?: string; // Deprecated: use task_id
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
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

Ejecuta un [flujo de trabajo dinámico](/docs/es/workflows): un script que orquesta muchos subagentes en segundo plano y devuelve un resultado consolidado. La herramienta `Workflow` está disponible en Agent SDK v0.3.149 y posterior. Se requiere al menos uno de `script`, `name` o `scriptPath`.

| Campo             | Tipo      | Descripción                                                                                                                                                                                                                                                                                                                             |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Script de flujo de trabajo en línea. Debe comenzar con `export const meta = { name, description }` como un literal, seguido del cuerpo del script usando `agent()`, `parallel()`, `pipeline()` y `phase()`. Una matriz `phases` opcional en `meta` agrupa agentes bajo etapas nombradas en la vista de progreso                         |
| `name`            | `string`  | Nombre de un flujo de trabajo integrado o uno guardado en `.claude/workflows/`. Se resuelve a un script                                                                                                                                                                                                                                 |
| `scriptPath`      | `string`  | Ruta a un archivo de script de flujo de trabajo en disco. Tiene precedencia sobre `script` y `name`. Claude Code persiste cada invocación de script y devuelve la ruta en el resultado, para que pueda editar ese archivo e invocar nuevamente con el mismo `scriptPath` para iterar                                                    |
| `args`            | `unknown` | Valor de entrada expuesto al script como el `args` global, para flujos de trabajo nombrados parametrizados como una pregunta de investigación o una lista de rutas de archivo. Pase matrices y objetos como valores JSON reales, no como una cadena codificada en JSON                                                                  |
| `resumeFromRunId` | `string`  | ID de ejecución de una invocación anterior de `Workflow` para reanudar. Las llamadas `agent()` completadas con entradas sin cambios devuelven resultados en caché; el resto se ejecuta en vivo. [Reanudar después de una pausa](/docs/es/workflows#resume-after-a-pause) cubre qué llamadas completadas se re-ejecutan. Solo la misma sesión |
| `title`           | `string`  | Ignorado; el bloque `meta` del script establece el título                                                                                                                                                                                                                                                                               |
| `description`     | `string`  | Ignorado; el bloque `meta` del script establece la descripción                                                                                                                                                                                                                                                                          |

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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Vea [Disponibilidad de modelo](/docs/es/agent-sdk/todo-tracking#model-availability) para optar por participar.
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
  [k: string]: unknown;
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

<h3 id="exitworktree">
  ExitWorktree
</h3>

**Nombre de herramienta:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

Sale del worktree git actual y regresa al directorio de trabajo original. La acción `keep` deja el worktree y la rama en disco, mientras que `remove` elimina ambos. `discard_changes` debe ser `true` cuando se elimina un worktree que tiene archivos sin confirmar o commits sin fusionar.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**Nombre de herramienta:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

Entra en modo de planificación, donde Claude investiga y presenta un plan antes de hacer cambios.

<h3 id="croncreate">
  CronCreate
</h3>

**Nombre de herramienta:** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

Programa un mensaje para ejecutarse en un cronograma cron de 5 campos en hora local. Establezca `recurring` a `false` para dispararse una vez en la siguiente coincidencia. Los trabajos tienen alcance de sesión de forma predeterminada: iniciar una conversación nueva los borra, y reanudar con `--resume` o `--continue` restaura trabajos que no han expirado. Vea [Tareas programadas](/docs/es/scheduled-tasks).

Establecer `durable` a `true` solicita persistencia a `.claude/scheduled_tasks.json` para que el trabajo sobreviva a los reinicios. La programación durable no está disponible en todas las sesiones: cuando no lo está, Claude Code acepta `durable: true` pero crea el trabajo solo de sesión. Lea el campo `durable` de la salida para ver si el trabajo persistió.

<h3 id="crondelete">
  CronDelete
</h3>

**Nombre de herramienta:** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

Elimina un trabajo cron programado por el ID devuelto desde `CronCreate`.

<h3 id="cronlist">
  CronList
</h3>

**Nombre de herramienta:** `CronList`

```typescript theme={null}
type CronListInput = {};
```

Enumera los trabajos cron programados: trabajos duraderos desde `.claude/scheduled_tasks.json` y trabajos solo de sesión desde la sesión actual.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**Nombre de herramienta:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

Programa un despertar de una sola vez que dispara el mensaje dado después de un retraso. Esta herramienta respalda el comando `/loop` de ritmo propio. El tiempo de ejecución fija `delaySeconds` entre 60 y 3600 segundos. Los campos `delaySeconds`, `reason`, `prompt` y `noop` son requeridos a menos que `stop` sea true. `noop: true` reporta un despertar donde nada cambió. Establecer `stop: true` cancela el despertar pendiente y finaliza el `/loop` de ritmo propio. El campo `stop` requiere Claude Code v2.1.202 o posterior. Vea la [fila ScheduleWakeup en la referencia de herramientas](/docs/es/tools-reference).

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**Nombre de herramienta:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

Gestiona [Rutinas](/docs/es/routines), las ejecuciones de Claude Code programadas y activadas alojadas en la nube. Esta herramienta respalda el comando `/schedule`. `trigger_id` es requerido para las acciones `get`, `update`, `run` y `list_runs`. `body` es requerido para `create`, `update` y `create_webhook_trigger`, y opcional para `run`.

`create_webhook_trigger` adjunta una fuente de evento a una rutina existente, como un [evento de GitHub](/docs/es/routines#add-a-github-trigger) que la dispara. El `body` nombra la fuente, los eventos y la rutina a disparar. Requiere Claude Code v2.1.225 o posterior.

`list_runs` enumera las ejecuciones recientes de una rutina, y `get_run_log` lee el registro de una ejecución. `session_id` nombra la ejecución a leer, desde un resultado de `list_runs`, y `cursor` pagina a través de los resultados de cualquiera de las acciones. Ambas acciones requieren Claude Code v2.1.227 o posterior.

Esta herramienta está disponible solo cuando la sesión se autentica con una cuenta claude.ai en un plan con Rutinas habilitadas, y está ausente cuando la política de su organización deshabilita [Claude Code en la web](/docs/es/claude-code-on-the-web). En Claude Code v2.1.227 o posterior, la herramienta también está ausente cuando un Propietario ha [desactivado las rutinas para la organización](/docs/es/routines#routines-are-disabled-by-your-organizations-policy). Antes de v2.1.227, una sesión con solo el interruptor de rutinas desactivado aún mostraba la herramienta, y el servidor denegaba sus llamadas.

<h3 id="pushnotification">
  PushNotification
</h3>

**Nombre de herramienta:** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

Envía una notificación push proactiva al usuario. Mantenga `message` bajo 200 caracteres porque los sistemas operativos móviles truncan texto más largo. Vea la [fila PushNotification en la referencia de herramientas](/docs/es/tools-reference) para disponibilidad de proveedor; la entrega push se ejecuta a través de infraestructura alojada por Anthropic que no es accesible desde Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud o Microsoft Foundry.

<h3 id="repl">
  REPL
</h3>

Eliminado en v2.1.275. A través de v2.1.274, una herramienta `REPL` experimental podría activarse con `CLAUDE_CODE_REPL=1` en la opción [`env`](#options).

<h3 id="reportfindings">
  ReportFindings
</h3>

**Nombre de herramienta:** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Reporta hallazgos de revisión de código como una lista estructurada para que Claude Code pueda renderizarlos en lugar de imprimirlos como texto. `level` es el nivel de esfuerzo en el que se ejecutó la revisión. Los hallazgos se ordenan de más grave a menos grave, con un máximo de 32 por llamada, y el array está vacío cuando ninguno sobrevivió. Requiere Claude Code v2.1.196 o posterior.

Cada hallazgo lleva estos campos:

* `file`: ruta relativa al repositorio en la que está el hallazgo. El `line` opcional es la línea indexada en 1 a la que se ancla.
* `summary`: declaración de una oración del defecto. `failure_scenario` describe las entradas concretas y el estado que conducen a la salida incorrecta o al bloqueo.
* `short_summary`: etiqueta comprimida opcional de como máximo 60 caracteres para visualización compacta. Requiere Claude Code v2.1.212 o posterior.
* `category`: slug kebab-case corto opcional del tipo de hallazgo, como `correctness` o `test-coverage`. Requiere Claude Code v2.1.199 o posterior.
* `verdict`: se establece cuando se ejecutó una pasada de verificación; ausente en revisiones solo en línea.
* `outcome`: se establece solo cuando se reporta nuevamente después de aplicar correcciones.

<h3 id="artifact">
  Artifact
</h3>

**Nombre de herramienta:** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

Publica un archivo `.html` o `.md` local como una página de artefacto alojada, o enumera los artefactos publicados del usuario. Omita `action` o pase `"publish"` para publicar `file_path`, que es requerido para la acción de publicación. Cada campo a continuación se aplica a una publicación:

* `icon`: una palabra genérica corta para el icono de pestaña del navegador del artefacto, como `chart` o `map`. Claude lo incluye en una primera publicación y lo omite en una actualización, lo que mantiene el icono almacenado del artefacto.
* `favicon`: deprecado, y Claude lo omite.
* `title`: nombra la página publicada en la pestaña del navegador y la galería cuando el archivo HTML no tiene etiqueta `<title>`.
* `url`: apunta a un artefacto existente para actualizar en su lugar en lugar de crear uno nuevo.

`force` es una sobrescritura de último recurso que descarta una versión más nueva que otra sesión publicó. En un conflicto, la publicación fallida devuelve el contenido más nuevo; Claude fusiona sus cambios en ese contenido, o vuelve a leer el artefacto, y publica nuevamente. Pase `force` solo cuando el usuario solicita explícitamente descartar esa versión.

Pase `"list"` para enumerar los artefactos publicados del usuario; solo `limit` y `scope` pueden acompañarlo. `scope` tiene como valor predeterminado `"mine"`, que enumera artefactos que el usuario posee; `"shared"` enumera artefactos que otras personas compartieron con el usuario, y `"all"` enumera ambos.

* `capabilities`: las capacidades de tiempo de ejecución que usa la página publicada, codificadas por nombre de capacidad, como los [conectores que la página puede llamar](/docs/es/artifacts#pull-live-data-with-mcp-connectors). El servicio de artefactos valida la declaración y rechaza una publicación que nombra una capacidad que la cuenta no puede usar o le da una configuración inválida. Pase `{}` para borrar una declaración almacenada, y omita el campo en un redeploy para mantenerlo. Requiere Agent SDK v0.3.235 o posterior.
* `contract`: la versión de tiempo de ejecución contra la que se ejecuta la página publicada. Omítalo para mantener la versión actual del artefacto, pase `"latest"` para actualizar, o pase una versión específica para fijar o revertir. Requiere Agent SDK v0.3.235 o posterior.

Los tipos se exportan, pero la herramienta está desactivada de forma predeterminada en sesiones de Agent SDK. La publicación también requiere todas las condiciones en la [tabla de disponibilidad de artefactos](/docs/es/artifacts#availability), que las sesiones autenticadas con una clave API no cumplen.

<h3 id="projects">
  Projects
</h3>

**Nombre de herramienta:** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

Lee y escribe el Proyecto de claude.ai adjunto a la sesión. Distribuye en `method`:

* `project_info`: devuelve metadatos del proyecto y la lista de documentos.
* `project_read`: lee un documento por `path`.
* `project_search`: consulta la base de conocimiento del proyecto con `query`. `n` limita los resultados y tiene como valor predeterminado 5.
* `project_write`: crea o reemplaza un documento en `path` desde exactamente uno de `content`, que lleva texto en línea, o `local_path`, que nombra un archivo dentro del directorio de trabajo. `present_to_user: true` marca el documento escrito como el entregable que el usuario necesita ver.
* `project_delete`: elimina un documento por `path`.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**Nombre de herramienta:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

Enumera los hijos directos de un recurso de directorio en un servidor MCP. Solo se puede usar contra un servidor que haya declarado soporte para listado de directorios; el listado no es recursivo. El listado de directorios no está habilitado en todas las sesiones: cuando está desactivado, la llamada devuelve una lista `resources` vacía y el campo `error` reporta que el listado de directorios no está habilitado.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**Nombre de herramienta:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

Vuelve a consultar la lista de herramientas de servidores MCP conectados y aplica cualquier cambio. Los tipos se exportan, pero Claude Code registra la herramienta solo cuando establece `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` en la opción [`env`](#options), y solo en sesiones con al menos un servidor MCP. Requiere Claude Code v2.1.211 o posterior.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**Nombre de herramienta:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Renderiza una fila de chip selector de rol clickeable durante la incorporación de Cowork para que el usuario pueda elegir su rol y obtener un plugin coincidente instalado. No toma argumentos; la lista de roles se define por el cliente. La llamada se bloquea hasta que el usuario responde.

<h3 id="mcpinput">
  McpInput
</h3>

**Nombre de herramienta:** nombres de herramientas MCP dinámicas de la forma `mcp__<server>__<tool>`

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

Los argumentos de herramientas MCP son un objeto abierto: cada servidor define sus propios parámetros, por lo que el tipo no coloca restricciones en nombres de campos o valores. Consulte el esquema de herramientas del servidor para los campos que acepta una herramienta específica.

<h2 id="tool-output-types">
  Tipos de Salida de Herramienta
</h2>

Documentación de esquemas de salida para todas las herramientas integradas de Claude Code. Estos tipos se exportan desde `@anthropic-ai/claude-agent-sdk` y representan los datos de respuesta reales devueltos por cada herramienta.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Unión de tipos de salida de herramienta exportados desde `@anthropic-ai/claude-agent-sdk`; los miembros incluyen:

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
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

**Nombre de herramienta:** `Agent`. El nombre anterior `Task` aún se acepta como alias, y el array `tools` en el mensaje de inicialización [`SDKSystemMessage`](#sdksystemmessage) actualmente lista esta herramienta como `Task` para compatibilidad hacia atrás.

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
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
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
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
      modelsUsed?: string[];
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

En la variante `completed`, `resolvedModel` nombra el modelo en el que el subagente comenzó, que puede diferir del `model` solicitado cuando [`availableModels`](/docs/es/model-config#restrict-model-selection) u otra anulación se aplica. Este campo requiere Claude Code v2.1.174 o posterior. En `async_launched`, nombra el modelo en uso cuando la tarea se movió al fondo.

`modelsUsed` lista los modelos que el subagente utilizó, en orden. El campo está presente solo cuando ocurrió un cambio a mitad de ejecución, y un modelo aparece nuevamente cuando la ejecución cambió de vuelta a él. En `async_launched`, la lista cubre los modelos utilizados antes de pasar al fondo. Tanto `modelsUsed` como el comportamiento de pasar al fondo de `resolvedModel` requieren Claude Code v2.1.212 o posterior.

Si Claude Code [mantuvo el worktree aislado del subagente](/docs/es/worktrees#isolate-subagents-with-worktrees), `worktreePath` en el resultado `completed` es donde encontrarlo. `worktreeBranch` es su rama, presente cuando Claude Code creó el worktree con git.

Claude Code completa `usage` y `totalTokens` desde la solicitud final de API del subagente, no desde toda la ejecución, por lo que `usage.service_tier` es la cadena de nivel de servicio que la API reportó en esa solicitud. Cuando está presente, `usage.output_tokens_details.thinking_tokens` es el número de tokens de salida de esa solicitud que fueron tokens de pensamiento. El campo `output_tokens_details` requiere TypeScript SDK v0.3.228 o posterior, que incluye Claude Code v2.1.228.

`usage.output_tokens_details` coincide con [`Usage.output_tokens_details`](#usage) en significado, limitado a esa solicitud final, pero cada nivel de él es opcional aquí. Proteja tanto el objeto como el campo, por ejemplo `usage.output_tokens_details?.thinking_tokens ?? 0`, en lugar de leerlo directamente.

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
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
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
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

Los campos `stdout`, `stderr`, y `backgroundTaskId` llevan:

| Campo              | Lo que lleva                                                                                                                  |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| `stdout`           | La salida estándar y error estándar del comando, fusionados en un flujo intercalado                                           |
| `stderr`           | Avisos que la herramienta misma añade, como un reinicio del directorio de trabajo del shell, no el error estándar del comando |
| `backgroundTaskId` | Presente para comandos de fondo                                                                                               |

`timedOutAfterMs` es el tiempo de espera en milisegundos, establecido cuando el comando alcanzó su tiempo de espera y se movió al fondo en lugar de comenzar allí explícitamente. `backgroundCwdHint` se establece cuando el comando en segundo plano contenía un builtin de cambio de directorio como `cd`, `pushd`, `popd`, o `chdir`, y nota que el directorio de trabajo de la sesión no cambió. Ambos campos requieren Claude Code v2.1.210 o posterior.

Cuando un subagente ejecutándose en primer plano posee un comando en segundo plano, Claude Code termina el comando cuando ese subagente da su respuesta final. Claude Code establece `backgroundEndsWithFinalResponse` a `true` en tales comandos, y omite el campo cuando el comando sobrevive al turno, como los comandos iniciados por la conversación principal o por subagentes de fondo. El campo requiere Claude Code v2.1.227 o posterior.

Claude Code establece `gitOperation.commit.branch` a la rama nombrada en la línea de resumen del commit de git, y la omite para un commit realizado en un HEAD desacoplado. El campo requiere Agent SDK v0.3.227 o posterior. Claude Code reporta un comando `gh pr reopen` como la acción PR `reopened`, que requiere Agent SDK v0.3.234 o posterior.

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
  originalFile: string | null;
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
    repository?: string | null;
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
        /** True cuando una lectura de archivo completo fue paginada automáticamente porque excedió el límite de tokens (el contenido es una primera página parcial). */
        truncatedByTokenCap?: boolean;
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
      /** Número de página del documento de la primera página extraída; etiqueta las imágenes de página en el contenido de tool_result. */
      firstPage?: number;
      /** Solo en proceso: los bytes de imagen de página se entregan como bloques de imagen en el contenido de tool_result y no se retienen en el tool_use_result emitido, por lo que esta clave está ausente allí. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Se establece cuando la deduplicación coincidió con una entrada sembrada al inicio (CLAUDE.md / memoria anidada) en lugar de un tool_result de Read anterior. */
      source?: "seeded";
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
    repository?: string | null;
  };
  userModified?: boolean;
};
```

Devuelve el resultado de escritura con información de diff estructurado. Lo que `originalFile` y `structuredPatch` contienen depende de la escritura:

* Para un archivo recién creado, `originalFile` es null y `structuredPatch` está vacío
* En una sobrescritura, `originalFile` lleva el contenido anterior, excepto cuando ese contenido es más grande que aproximadamente 10 MB: Claude Code entonces omite el diff y devuelve `originalFile` null y `structuredPatch` vacío
* `structuredPatch` también está vacío cuando la escritura no cambió nada o el diff agotó el tiempo de espera

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
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

Devuelve rutas de archivo que coinciden con el patrón glob, ordenadas por hora de modificación.

`totalMatches` y `countIsComplete` requieren Claude Code v2.1.191 o posterior. `totalMatches` reporta el número de archivos coincidentes antes del truncamiento. Cuando `countIsComplete` es false, `totalMatches` es un límite inferior porque la búsqueda subyacente truncó su propia salida.

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
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Devuelve resultados de búsqueda. La forma varía por `mode`: lista de archivos, contenido con coincidencias, o conteos de coincidencias. En modo `count`, `numFiles` y `numMatches` son totales sobre el conjunto de resultados completo, no la porción paginada. Antes de v2.1.208, un `head_limit` u `offset` que truncaba las entradas listadas también truncaba esos totales.

`totalFiles` requiere Claude Code v2.1.208 o posterior y reporta el número total de resultados antes de la paginación `head_limit` y `offset` en modo `files_with_matches`. `totalLines` requiere Claude Code v2.1.210 o posterior y reporta el número total de líneas antes de la paginación en modo `content`.

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
  old_source?: string;
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
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

Devuelve el contenido obtenido con estado HTTP y metadatos.

`artifactRead` es el registro propio de Claude Code de una lectura de artefacto, presente solo cuando Claude obtuvo un artefacto que la sesión puede publicar. Claude Code lo lee de vuelta cuando se reanuda una sesión para que una publicación posterior se base en la versión correcta; su código no necesita actuar sobre él. `slug` nombra el artefacto, `ver` es la versión que la lectura registró y está ausente cuando no registró ninguna, y `seeded: false` marca una lectura cuya fuente completa no llegó a Claude. El campo `seeded` requiere Agent SDK v0.3.239 o posterior.

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
  searchCount?: number;
};
```

Devuelve resultados de búsqueda de la web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nombre de herramienta:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // se establece cuando el flujo de trabajo se lanzó como una sesión remota
  warning?: string;
  error?: string;
};
```

Devuelve inmediatamente después de que la herramienta acepta la invocación. El resultado final llega más tarde como una finalización de tarea. Verifique `error` antes de tratar la ejecución como iniciada: un script que falla su verificación de sintaxis devuelve `status: "async_launched"` con `error` establecido, y nunca se ejecuta.

| Campo           | Tipo                                    | Descripción                                                                                                                                                                                                                    |
| --------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `status`        | `"async_launched" \| "remote_launched"` | La herramienta aceptó la invocación. `"async_launched"` para ejecuciones en proceso, `"remote_launched"` para ejecuciones enviadas a una sesión remota en lugar de ejecutarse en proceso                                       |
| `taskId`        | `string`                                | Identificador de tarea de fondo para la ejecución                                                                                                                                                                              |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | Tipo de tarea de la tarea de fondo registrada, coincidiendo con el brazo `status`                                                                                                                                              |
| `workflowName`  | `string`                                | El `meta.name` del script de flujo de trabajo                                                                                                                                                                                  |
| `runId`         | `string`                                | Identificador de ejecución de flujo de trabajo para pasar como `resumeFromRunId` en una invocación posterior. Ausente para ejecuciones `remote_launched`, donde la URL de sesión en la nube es el identificador de reanudación |
| `summary`       | `string`                                | Descripción de una línea de lo que hace el flujo de trabajo                                                                                                                                                                    |
| `transcriptDir` | `string`                                | Directorio donde se escriben las transcripciones de subagentes durante la ejecución                                                                                                                                            |
| `scriptPath`    | `string`                                | Ruta al script de flujo de trabajo persistido para esta ejecución. Edítelo y páselo como `scriptPath` para volver a ejecutar sin reenviar el script                                                                            |
| `sessionUrl`    | `string`                                | URL de sesión en la nube, se establece cuando `status` es `"remote_launched"`                                                                                                                                                  |
| `warning`       | `string`                                | Aviso sin bloqueo, como el estado local de git divergiendo de la rama enviada que una sesión en la nube clonará                                                                                                                |
| `error`         | `string`                                | Se establece cuando el script falla su verificación de sintaxis. Cuando está presente, la ejecución no se inició a pesar del estado lanzado                                                                                    |

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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Consulte [Disponibilidad de modelo](/docs/es/agent-sdk/todo-tracking#model-availability) para optar por participar.
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
  planWasEdited?: boolean;
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
    blobSavedTo?: string;
  }>;
  error?: string;
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

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**Nombre de herramienta:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

Devuelve la acción tomada y detalles sobre el worktree que se salió.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**Nombre de herramienta:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

Devuelve una confirmación de que se entró en modo de planificación.

<h3 id="croncreate-2">
  CronCreate
</h3>

**Nombre de herramienta:** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true cuando se persiste a .claude/scheduled_tasks.json; false cuando es solo de sesión
};
```

Devuelve el ID del trabajo y una descripción legible por humanos del cronograma.

<h3 id="crondelete-2">
  CronDelete
</h3>

**Nombre de herramienta:** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

Devuelve el ID del trabajo eliminado.

<h3 id="cronlist-2">
  CronList
</h3>

**Nombre de herramienta:** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

Devuelve los trabajos cron programados: trabajos duraderos de `.claude/scheduled_tasks.json` y trabajos solo de sesión de la sesión actual. Un trabajo solo de sesión lleva `durable: false`; los trabajos leídos del disco omiten el campo.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**Nombre de herramienta:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

Devuelve cuándo se disparará el despertar como una marca de tiempo de época en milisegundos, el retraso realmente utilizado, y si el retraso solicitado fue limitado. El campo `stopped` es `true` cuando la llamada terminó el bucle con `stop: true`. Requiere Claude Code v2.1.202 o posterior. El campo `cancelledWakeups` cuenta cuántos despertares pendientes una llamada `stop: true` canceló. Un valor de 0 significa que nada estaba pendiente, y un cron `/loop` recurrente no se cancela por `stop: true`. Requiere Claude Code v2.1.206 o posterior.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**Nombre de herramienta:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

Devuelve el estado de respuesta de API y el cuerpo para la operación de activación.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**Nombre de herramienta:** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

Devuelve detalles de entrega, incluyendo si se envió una notificación push o local y por qué se omitió la entrega.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**Nombre de herramienta:** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Devuelve el número de hallazgos reportados, el nivel de esfuerzo en el que se ejecutó la revisión, y los hallazgos repetidos para el cuerpo del resultado. Requiere Claude Code v2.1.196 o posterior. El campo `short_summary` repetido requiere Claude Code v2.1.212 o posterior.

<h3 id="artifact-2">
  Artifact
</h3>

**Nombre de herramienta:** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

Devuelve la `url` de la página publicada y la `path` local que se publicó para la acción de publicación, con `updated` establecido en true cuando la publicación reimplementó un artefacto existente, y `warnings` llevando cualquier aviso de tiempo de publicación. La acción de lista devuelve las filas de `artifacts` en su lugar, con `truncated` establecido cuando existen más artefactos que el límite solicitado. En listados cuyo alcance no es `"mine"`, cada fila lleva `rel` marcando si el usuario posee el artefacto o se compartió con ellos, y la `scope` del resultado registra qué alcance no predeterminado produjo el listado; ambos están ausentes en listados predeterminados.

<h3 id="projects-2">
  Projects
</h3>

**Nombre de herramienta:** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

Discriminado en el campo `method`, reflejando la entrada. `project_read` devuelve documentos de texto pequeños en línea en `content` y escribe documentos más grandes en una ruta `local_file` en su lugar; `project_search` devuelve `hits` RAG con `rag: true` cuando el índice del proyecto está disponible y retrocede a una lista de ruta `docs` en caso contrario.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**Nombre de herramienta:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

Devuelve los hijos directos del recurso de directorio. Los subdirectorios aparecen con mimeType `"inode/directory"`; `error` lleva un mensaje legible por humanos cuando el servidor no pudo listar el directorio.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**Nombre de herramienta:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // herramientas ahora disponibles desde este servidor
  added?: string[]; // nombres de herramientas que esta actualización añadió
  removed?: string[]; // nombres de herramientas que esta actualización eliminó
  error?: string; // por qué falló la actualización o el servidor no estaba disponible
}>;
```

Devuelve una entrada por servidor: `refreshed` significa que la lista de herramientas re-consultada se aplicó, `error` significa que la re-consulta falló y se mantuvo el conjunto de herramientas anterior, y `not_connected` significa que el servidor no tiene conexión activa para consultar.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**Nombre de herramienta:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

Devuelve la selección del usuario: `role` cuando eligieron un chip de rol o escribieron uno, y `dismissed: true` cuando cerraron el selector. Un objeto vacío significa que el usuario aprobó la llamada sin elegir un rol.

<h3 id="mcpoutput">
  McpOutput
</h3>

**Nombre de herramienta:** nombres de herramientas MCP dinámicos de la forma `mcp__<server>__<tool>`

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

Los resultados de herramientas MCP se devuelven como una cadena o una matriz de bloques de contenido, dependiendo del servidor. La rama de objeto simple al final del tipo exportado es un artefacto de generación de esquema: el SDK no devuelve un objeto simple, porque la salida estructurada de un servidor se serializa a una cadena JSON antes de ser devuelta. En tiempo de ejecución el valor también puede ser `undefined`, aunque el tipo exportado no modela esto.

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

De dónde provino la clave API para las solicitudes de la sesión, reportada como `apiKeySource` en el mensaje de inicialización [`SDKSystemMessage`](#sdksystemmessage).

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code reporta uno de cuatro valores:

| Valor                | Clave en uso                                                                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | La clave en la variable de entorno `ANTHROPIC_API_KEY`                                                                                      |
| `apiKeyHelper`       | La clave devuelta por su comando [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper)                                                      |
| `/login managed key` | La clave que Claude Code almacenó cuando inició sesión con una [cuenta de Claude Console](/docs/es/authentication#claude-console-authentication) |
| `none`               | Sin clave API. La sesión se autentica de otra manera, como un inicio de sesión en claude.ai, un token portador o un proveedor de nube       |

Agent SDK v0.3.234 y posterior enumeran estos cuatro valores en el tipo. El tipo también mantiene `user`, `project`, `org`, `temporary` y `oauth` para que el código más antiguo siga compilándose, y Claude Code no los reporta.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Características beta disponibles que se pueden habilitar a través de la opción `betas`. Vea [Encabezados Beta](https://platform.claude.com/docs/en/api/beta-headers) para más información.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  La beta `context-1m-2025-08-07` se retiró a partir del 30 de abril de 2026. Pasar este valor con Claude Sonnet 4.5 o Sonnet 4 no tiene efecto, y las solicitudes que excedan la ventana de contexto estándar de 200k tokens devuelven un error. Para usar una ventana de contexto de 1M tokens, migre a [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 u Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), que incluyen contexto de 1M a precios estándar sin encabezado beta requerido.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Información sobre un comando disponible.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` es `true` en una fila cuando el comando es propio de Claude Code y escribir `/name` lo ejecuta. Está ausente para un comando definido por un usuario, proyecto, plugin o servidor MCP, y para un comando agrupado que uno de esos [reemplaza por nombre](/docs/es/skills#resolve-skills-that-share-a-name). Requiere Agent SDK v0.3.277 o posterior.

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

| Campo         | Tipo                  | Descripción                                                                                                                                                                                                         |
| :------------ | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | `string`              | Identificador de tipo de agente (por ejemplo, `"Explore"`, `"general-purpose"`)                                                                                                                                     |
| `description` | `string`              | Descripción de cuándo usar este agente                                                                                                                                                                              |
| `model`       | `string \| undefined` | Modelo que usa este agente: un alias o ID de modelo, o `'inherit'` para el modelo del padre. Cuando es `undefined`, Claude Code elige el modelo en el [orden de modelo de subagente](/docs/es/sub-agents#choose-a-model) |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

El servidor MCP que sirve una herramienta `mcp__*`, y de dónde provino la definición de ese servidor. Las entradas de hook [`PreToolUse`](#pretoolusehookinput), `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` y `PermissionDenied` la llevan como `mcp_server`, y las opciones [`CanUseTool`](#canusetool) la llevan como `mcpServer`. Ambas la omiten para herramientas que no provienen de un servidor MCP.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| Campo    | Tipo     | Descripción                                                                                                                 |
| :------- | :------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | El nombre bajo el cual está registrado el servidor, el mismo valor que [`mcpServerStatus()`](#query-object) reporta para él |
| `source` | `string` | De dónde provino la definición del servidor: `sdk`, `plugin` o un alcance de configuración                                  |

`source` toma uno de los siguientes valores. El conjunto está abierto, así que trate un valor que no reconozca como una fuente configurada, nunca como `sdk`:

* **`sdk`**: un servidor en proceso que su aplicación registró. Solo la aplicación host del SDK puede registrar uno, así que un servidor configurado nunca reporta `sdk`, sea cual sea su nombre.
* **`plugin`**: un servidor que proporciona un [plugin](/docs/es/agent-sdk/plugins). Su `name` es la forma con alcance `plugin:<plugin-name>:<server-name>` descrita bajo [servidores MCP proporcionados por plugins](/docs/es/mcp#plugin-provided-mcp-servers).
* **Un alcance de configuración**: `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai` o `agent`. Un servidor `.mcp.json` reporta `project`, y [alcances de instalación MCP](/docs/es/mcp#mcp-installation-scopes) define `local`, `project` y `user`. Los servidores que su aplicación pasa en la opción [`mcpServers`](#options), que no sean servidores SDK en proceso, reportan `dynamic`.

Base las decisiones de confianza en `source`, no en `name` o el prefijo de nombre de herramienta `mcp__<server>__`. Para cualquier fuente que no sea `sdk`, `name` es texto no confiable: escápelo antes de mostrarlo.

`McpServerProvenance` y los campos que la llevan requieren Agent SDK v0.3.274 o posterior.

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
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

`source` dice de dónde provino la definición del servidor, con los mismos valores y regla de confianza que el `source` de [`McpServerProvenance`](#mcpserverprovenance). El campo requiere Agent SDK v0.3.274 o posterior y está ausente en versiones anteriores.

`_meta` en una entrada `tools` lleva los miembros de MCP Apps de `_meta` de esa herramienta, por lo que su aplicación puede encontrar el recurso `ui://` para renderizar con [`readMcpResource()`](#query-object). Claude Code pasa el objeto `ui` y la cadena `ui/resourceUri` plana deprecada, y retiene todas las otras claves. Dentro de `ui`, `resourceUri` es una cadena `ui://` y `visibility` es una matriz de `"model"` y `"app"` cuando el servidor los establece, y cualquier otro miembro pasa sin cambios. Claude Code descarta cualquier clave cuando el valor está mal formado, y omite `_meta` de una herramienta que no declara ninguno. El campo está presente solo cuando las [`capabilities`](#sdksystemmessage) del mensaje de inicialización incluyen `mcp_tool_ui_meta_v1`, y requiere TypeScript Agent SDK v0.3.280 o posterior.

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
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` cuenta los tokens de pensamiento que este modelo generó. `outputTokens` ya los incluye, así que no sume los dos juntos. El campo está ausente hasta que un turno se ejecuta en una versión de Claude Code que lo registra, por lo que una sesión reanudada que comenzó en una versión anterior reporta un recuento parcial. `thinkingTokens` requiere Agent SDK v0.3.257 o posterior.

Los campos `canonicalModel` y `provider` requieren Claude Code v2.1.218 o posterior. `canonicalModel` es el ID de modelo canónico que la búsqueda de precios utiliza; puede diferir de la cadena de modelo sin procesar que clave la entrada, por ejemplo cuando esa cadena es un ID específico del proveedor o un alias.

`provider` nombra el backend de API que sirvió el modelo, como `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` o `gateway`.

`costBasis` nombra la tabla de precios que fijó el precio de la solicitud más reciente del modelo: `list` para precio de lista, `managed` para una tabla [`modelPricing`](/docs/es/settings-reference#modelpricing), o `unknown` cuando ninguno coincidió con el ID del modelo. El campo requiere Claude Code v2.1.246 o posterior.

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
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`, `BetaIterationsUsage` y `BetaOutputTokensDetails` se definen en `@anthropic-ai/sdk`.

`output_tokens_details` desglosa la salida facturada por categoría. Actualmente lleva un campo, `thinking_tokens: number`, contando los tokens de salida que el modelo generó como razonamiento interno, incluyendo los delimitadores de bloque de pensamiento. El campo `output_tokens_details` requiere TypeScript SDK v0.3.228 o posterior, que agrupa Claude Code v2.1.228.

* **Facturación**: lea el desglose para observabilidad, no para facturación. `output_tokens` sigue siendo el total autorizado, y `output_tokens - thinking_tokens` aproxima la salida sin razonamiento.
* **Qué cubre el recuento**: el razonamiento sin procesar que el modelo produjo, que puede ser más largo que el texto de pensamiento devuelto en el cuerpo de la respuesta. La API lo calcula volviendo a tokenizar ese texto sin procesar, por lo que puede diferir del recuento de generación exacto del modelo en algunos tokens.
* **Streaming**: en mensajes de asistente transmitidos este desglose, como `output_tokens`, es un marcador de posición `message_start` y no lleva un recuento real, así que léalo del mensaje de resultado `usage` como [Leer tokens de salida del mensaje de resultado](/docs/es/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) describe. En el mensaje de resultado, `thinking_tokens` lee `0` cuando el modelo o proveedor no reporta desglose.
* **Casos `null`**: `output_tokens_details` en sí es `null` en mensajes de asistente que Claude Code sintetiza, como mensajes de error de API.

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

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

Un archivo que una herramienta MCP devolvió por referencia. Claude Code construye cada entrada desde un bloque `resource_link` en el resultado de la herramienta y entrega la lista como `resourceLinks` en [`SDKUserMessage.tool_use_result`](#sdkusermessage), o como `resource_links` en [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) cuando la llamada terminó en segundo plano. Requiere Agent SDK v0.3.257 o posterior.

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code descarta un bloque cuyo `uri` o `name` no sea una cadena, y omite un campo opcional cuyo valor no sea del tipo listado.

| Campo         | Tipo                                   | Descripción                                                                |
| :------------ | :------------------------------------- | :------------------------------------------------------------------------- |
| `uri`         | `string`                               | URI del recurso, como lo devolvió el servidor                              |
| `name`        | `string`                               | Nombre que el servidor le dio al recurso                                   |
| `title`       | `string \| undefined`                  | Título de visualización, cuando el servidor estableció uno                 |
| `description` | `string \| undefined`                  | Descripción, cuando el servidor estableció una                             |
| `mimeType`    | `string \| undefined`                  | Tipo MIME, cuando el servidor estableció uno                               |
| `size`        | `number \| undefined`                  | Tamaño en bytes, cuando el servidor estableció uno                         |
| `annotations` | `Record<string, unknown> \| undefined` | El objeto de anotaciones MCP del bloque, cuando el servidor estableció uno |

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

El campo `display` opcional controla si el texto de pensamiento se devuelve `"summarized"` u `"omitted"`. En Claude Opus 4.7 y posterior, el valor predeterminado de la API es `"omitted"`, así que establezca `"summarized"` para recibir contenido de pensamiento en bloques `thinking`. Claude Code no envía `display` a Amazon Bedrock o a la Plataforma de Agentes de Google Cloud, por lo que en esos proveedores Opus 4.7 y posterior devuelven bloques `thinking` vacíos incluso cuando establece `display` en `"summarized"`.

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

Cuando llama a `setMcpServers()`, Claude Code aplica estas reglas:

* **Servidores que la llamada no nombra**: Claude Code mantiene los servidores proporcionados por plugins en ejecución. Requiere Agent SDK v0.3.210 o posterior.
* **Servidores que la llamada nombra**: excepto para servidores integrados que la CLI inició al inicio, Claude Code reemplaza un servidor en ejecución solo cuando su configuración difiere de la que pasó.
* **Servidores integrados que la CLI inició al inicio**: si la llamada nombra uno, Claude Code descarta esa entrada y la reporta en `errors`.

La promesa se resuelve después de que los servidores stdio, HTTP y SSE recién agregados se conecten o fallen, por lo que las herramientas de servidores que se conectaron están disponibles en el siguiente turno.

`added` enumera los servidores que Claude Code agregó o reemplazó, independientemente de si se conectaron. Un servidor que no se conectó aparece en `added` y `errors`, con el texto de falla bajo `errors` y una fila `failed` en [`mcpServerStatus()`](#methods). Antes de Claude Code v2.1.257, un servidor cuyo intento de conexión lanzó una excepción se reportaba solo bajo `errors`.

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
  skippedLinks?: number;
};
```

`skippedLinks` cuenta las rutas rastreadas que el rebobinado se negó a restaurar o eliminar por seguridad de enlace: un enlace simbólico, enlace duro u otro archivo no regular en la ruta rastreada, un directorio padre que ya no se resuelve a donde apuntaba cuando se tomó el punto de control, o una copia de seguridad que no se pudo leer de forma segura. El campo requiere Claude Code v2.1.216 o posterior. Una llamada de vista previa con `rewindFiles(userMessageId, { dryRun: true })` nunca lo establece.

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

Notificación cuando una tarea de fondo se completa, falla o se detiene. Las tareas de fondo incluyen comandos Bash `run_in_background`, vigilancias [Monitor](#monitor) y subagentes de fondo. Para el campo `ambient`, vea [`SDKTaskStartedMessage`](#sdktaskstartedmessage), que lo define y su requisito de versión.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Cuando Claude Code [mueve una llamada de herramienta MCP larga al fondo](/docs/es/mcp#automatic-backgrounding-of-long-tool-calls), el bloque `tool_result` para esa llamada contiene solo un marcador de posición y el resultado real de la llamada llega en esta notificación. Haga coincidir la notificación con la llamada usando `tool_use_id`. En una notificación `completed`, `resource_links` enumera los archivos que la herramienta devolvió por referencia como entradas [`SDKMcpResourceLink`](#sdkmcpresourcelink), con los mismos límites de 50 enlaces y 64 KiB que [`tool_use_result.resourceLinks`](#sdkusermessage). Claude Code omite `resource_links` cuando el resultado no tenía enlaces y en notificaciones para tareas que no son llamadas de herramienta MCP. `resource_links` requiere Agent SDK v0.3.257 o posterior.

Claude Code antepone un aviso a cada notificación de tarea que envía al modelo, excepto entregas marcadas con la [subclase `scheduled-trigger`](#task-notification-subkinds), que llevan un marco de tarea asignada en su lugar. El aviso establece que no ha ocurrido entrada humana, por lo que el modelo no trata la notificación como una instrucción o aprobación del usuario.

Para detectar un turno de notificación de tarea, verifique `origin.kind === "task-notification"` en [`SDKUserMessage`](#sdkusermessage) o [`SDKResultMessage`](#sdkresultmessage) en lugar de hacer coincidir el texto del aviso. Lea `subkind` del mismo campo si necesita saber qué lo generó. Antes de v2.1.205, Claude Code dejaba el aviso fuera de las notificaciones que llegaban mientras la sesión estaba inactiva.

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
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

Mientras una llamada de herramienta se ejecuta en la conversación principal, Claude Code emite un mensaje `tool_progress` cada 30 segundos con `heartbeat: true`. Cada latido lleva el nombre de la herramienta y segundos transcurridos, por lo que puede distinguir una llamada de larga duración de una sesión estancada. Claude Code no emite latidos para llamadas de herramienta dentro de un subagente. El campo `heartbeat` requiere Agent SDK v0.3.214 o posterior. Antes de v2.1.257, Claude Code tampoco emitía latidos para una llamada de herramienta Agent en primer plano.

En mensajes `tool_progress` para la herramienta Agent que no sean latidos, `subagent_type` nombra el tipo de subagente en ejecución, como `general-purpose`. `subagent_retry` está presente mientras ese subagente espera un retroceso de error de API, como un límite de velocidad o sobrecarga, con un mensaje por intento de reintento. Ambos campos requieren Agent SDK v0.3.214 o posterior.

Para renderizar un indicador de reintento desde `subagent_retry`:

* Rastrear el indicador por `parent_tool_use_id`, que es único por subagente. `tool_use_id` se comparte entre subagentes paralelos de un turno de asistente, por lo que rastrear por él permitiría que la actualización de un subagente borre el indicador de otro.
* Borrar el indicador cuando un `tool_progress` posterior para el mismo `parent_tool_use_id` llega sin `subagent_retry` ni `heartbeat: true`, o cuando llega el mensaje de resultado de la herramienta. Los fotogramas con `heartbeat: true` reportan solo vivacidad, así que mantenga el indicador cuando uno llega. `attempt` puede exceder `max_retries` bajo reintento persistente, así que no derive el borrado de los contadores.
* Tratar `error_category` como un token para elegir su propio texto de mensaje, no como texto de visualización. Los valores son `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error` e `unknown`. Maneje un valor que no reconozca de la manera que maneja `unknown`, porque versiones posteriores pueden agregar valores.

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

Se emite cuando comienza una tarea. El campo `task_type` es `"local_bash"` para comandos Bash y vigilancias [Monitor](#monitor), `"local_agent"` para subagentes, o `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient` es `true` para tareas que no son parte del trabajo de la sesión, como tareas que Claude Code ejecuta para su propia operación. Los vigilantes de actualización en vivo también son ambientes, incluyendo vigilantes que el usuario pidió. Excluya tareas ambientes de indicadores de actividad. El campo requiere Agent SDK v0.3.247 o posterior.

`ambient` también aparece en [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) y en entradas [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage).

`is_backgrounded` y `spawn_depth` describen cómo Claude Code inició la tarea. Ambos campos requieren Agent SDK v0.3.238 o posterior.

* `is_backgrounded`: Claude Code lo establece en tareas `"local_agent"` y `"local_bash"`. `true` significa que la tarea se ejecuta en segundo plano. `false` significa que la tarea se ejecuta en primer plano, y la llamada de herramienta que la inició permanece bloqueada hasta que la tarea se complete o se mueva al fondo.
* `spawn_depth`: Claude Code lo establece solo en tareas `"local_agent"`. Un subagente que el hilo principal generó tiene profundidad `1`. Un subagente que un subagente de profundidad `1` generó tiene profundidad `2`, y así sucesivamente.

Un [subagente reanudado](/docs/es/agent-sdk/subagents#resume-subagents) siempre reporta `is_backgrounded: true`, porque Claude Code ejecuta cada subagente reanudado en segundo plano. Cuando una tarea en primer plano se mueve al fondo más tarde, Claude Code reporta el nuevo valor `is_backgrounded` en un mensaje [`task_updated`](#sdktaskupdatedmessage) en lugar de enviar un segundo `task_started`.

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

Se emite siempre que el conjunto de tareas de fondo activas cambia: una tarea comienza, se completa, se mata, un agente en primer plano se pone en segundo plano, o el campo `description` o `ambient` de una tarea cambia.

El array `tasks` es el conjunto completo activo. Reemplace cualquier conjunto en caché con cada carga útil en lugar de emparejar eventos `task_started` y `task_notification`, para que el siguiente cambio de membresía corrija cualquier evento que haya perdido.

El orden relativo a esos eventos por tarea no está especificado, así que no correlacione los dos flujos.

Nada se emite al inicio. Reinicie a un conjunto vacío siempre que el proceso CLI de la sesión comience o se reinicie y deje que el siguiente cambio de membresía lo repuele.

Cuando envía una solicitud de control `initialize` repetida a una sesión en ejecución, como con [`reinitialize()`](#query-object) después de una brecha de transporte, Claude Code sigue la respuesta con una instantánea del conjunto activo actual, incluso cuando está vacío. Un host que se reconecta por lo tanto aprende qué se está ejecutando sin esperar al siguiente cambio de membresía. Antes de Agent SDK v0.3.239, Claude Code no envió instantánea después de un `initialize` repetido.

Requiere Claude Code v2.1.203 o posterior.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Se emite mientras Claude está produciendo un bloque de pensamiento, incluyendo uno redactado. `estimated_tokens` es una estimación en ejecución de los tokens de pensamiento generados hasta ahora en el bloque actual, y `estimated_tokens_delta` es el incremento llevado por este fotograma. Use estas estimaciones para visualización de progreso.

Cuando el modelo o proveedor reporta un desglose, el recuento final para el bucle de agente de nivel superior es el [`usage.output_tokens_details.thinking_tokens`](#usage) del mensaje de resultado, que [no incluye tokens de subagente](/docs/es/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Requiere Claude Code v2.1.153 o posterior.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
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

Claude Code no emite este tipo de mensaje. Cuando envía un comando como `/context` o `/usage` como un mensaje, su salida llega como un [`SDKAssistantMessage`](#sdkassistantmessage).

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

Se emite cuando el conjunto de comandos disponibles cambia a mitad de sesión, como cuando Claude Code descubre skills al entrar en un subdirectorio. El array `commands` es la lista completa actualizada, así que reemplace cualquier lista de comandos en caché con esta carga útil. Llamar a [`supportedCommands()`](#query-object) después de este mensaje devuelve la misma lista actualizada, porque el método rastrea el último push; esto requiere Agent SDK v0.3.216 o posterior. En versiones anteriores del SDK, `supportedCommands()` devuelve la instantánea capturada en la inicialización y nunca refleja cambios a mitad de sesión.

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

Se emite después de un turno cuando [`promptSuggestions`](#options) está habilitado y Claude Code generó una sugerencia para ese turno. Contiene el siguiente mensaje de usuario predicho. Para los turnos que no obtienen ninguno, vea [Cuándo Claude Code omite sugerencias](/docs/es/interactive-mode#when-claude-code-skips-suggestions).

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

Se emite cuando la conversación de la sesión se reemplaza sin terminar la sesión. En una llamada `query()`, solo `/clear` y sus alias producen este mensaje. Monte una transcripción vacía bajo `new_conversation_id` y descarte cualquier título de sesión en caché.

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

`AbortError` es la única clase de error en la API tipificada del SDK. Otros fallos, como el proceso de Claude Code saliendo o no pudiendo iniciarse, rechazan la iteración de mensajes con errores que no llevan clase SDK para hacer coincidir. [Troubleshooting](/docs/es/agent-sdk/troubleshooting) clave esos errores por mensaje, con la causa y solución para cada uno.

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
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`         | Auto-apruebe comandos Bash cuando el sandbox está habilitado                                                                                                                                                                                                      |
| `excludedCommands`          | `string[]`                                            | `[]`           | Comandos que omiten restricciones de sandbox, como `['docker *']`. Estos se ejecutan sin sandbox automáticamente sin participación del modelo; [`sandbox.excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands) cubre cuándo se aplica una entrada   |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`         | Permita que el modelo solicite ejecutar comandos fuera del sandbox. Cuando es `true`, el modelo puede establecer `dangerouslyDisableSandbox` en la entrada de herramienta, que se vuelve al [sistema de permisos](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined`    | Configuración de sandbox específica de red                                                                                                                                                                                                                        |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined`    | Configuración de sandbox específica del sistema de archivos para restricciones de lectura/escritura                                                                                                                                                               |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined`    | Mapa de subcadenas de comando, o `*` para cada comando, a subcadenas del texto de violación a ignorar, como `{ "*": ['/etc/hosts'] }`; consulte [`sandbox.ignoreViolations`](/docs/es/settings-reference#sandbox-ignoreviolations)                                     |
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
  **Seguridad de socket Unix:** La opción `allowUnixSockets` puede otorgar acceso a servicios del sistema que alcanzan fuera del sandbox. Por ejemplo, permitir `/var/run/docker.sock` efectivamente otorga acceso completo al sistema host a través de la API de Docker, omitiendo el aislamiento de sandbox. Solo permita sockets Unix que sean estrictamente necesarios y comprenda las implicaciones de seguridad de cada uno.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuración específica de red para el modo sandbox. Estas configuraciones se aplican a comandos Bash en sandbox cuando `enabled` es `true` en la [`SandboxSettings`](#sandboxsettings) principal. No restringen la herramienta WebFetch, que utiliza [reglas de permisos](/docs/es/permissions#webfetch) en su lugar.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Propiedad                 | Tipo       | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------ | :--------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`           | Nombres de dominio a los que los procesos en sandbox pueden acceder                                                                                                                                                                                                                                                                                                                                                                        |
| `deniedDomains`           | `string[]` | `[]`           | Nombres de dominio a los que los procesos en sandbox no pueden acceder. Tiene prioridad sobre `allowedDomains`                                                                                                                                                                                                                                                                                                                             |
| `strictAllowlist`         | `boolean`  | `false`        | Niegue a los comandos en sandbox el acceso a hosts fuera de la [lista de permitidos de red](/docs/es/sandboxing#network-isolation) en lugar de solicitar. Se aplica solo a comandos en sandbox; las herramientas en proceso como WebFetch no están controladas por esto. Solo se respeta desde la configuración de usuario, administrada o CLI `--settings`; la configuración del proyecto se ignora. Requiere Claude Code v2.1.219 o posterior |
| `allowManagedDomainsOnly` | `boolean`  | `false`        | Solo configuración administrada. Cuando se establece en [configuración administrada](/docs/es/managed-settings), solo se respetan las entradas `allowedDomains` y las reglas de permitir `WebFetch(domain:...)` de la configuración administrada, y se ignoran las entradas de permitir de la configuración de usuario, proyecto o local. No tiene efecto cuando se establece a través de opciones de SDK                                       |
| `allowLocalBinding`       | `boolean`  | `false`        | Permita que los procesos se vinculen a puertos locales (por ejemplo, para servidores de desarrollo)                                                                                                                                                                                                                                                                                                                                        |
| `allowUnixSockets`        | `string[]` | `[]`           | Rutas de socket Unix a las que los procesos pueden acceder (por ejemplo, socket de Docker)                                                                                                                                                                                                                                                                                                                                                 |
| `allowAllUnixSockets`     | `boolean`  | `false`        | Permita el acceso a todos los sockets Unix                                                                                                                                                                                                                                                                                                                                                                                                 |
| `httpProxyPort`           | `number`   | `undefined`    | Puerto proxy HTTP para solicitudes de red                                                                                                                                                                                                                                                                                                                                                                                                  |
| `socksProxyPort`          | `number`   | `undefined`    | Puerto proxy SOCKS para solicitudes de red                                                                                                                                                                                                                                                                                                                                                                                                 |

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

Cuando `allowUnsandboxedCommands` está habilitado, el modelo puede solicitar ejecutar comandos fuera del sandbox estableciendo `dangerouslyDisableSandbox: true` en la entrada de herramienta. Estas solicitudes se vuelven al sistema de permisos existente, lo que significa que se invoca su controlador `canUseTool`, permitiéndole implementar lógica de autorización personalizada.

Sus entradas `excludedCommands` en su lugar omiten el sandbox sin participación del modelo; [`sandbox.excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands) cubre cuándo se aplica una entrada.

En el ejemplo siguiente, `isCommandAuthorized` representa una verificación de autorización que usted define.

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

<Warning>
  Los comandos que se ejecutan con `dangerouslyDisableSandbox: true` tienen acceso completo al sistema. Asegúrese de que su controlador `canUseTool` valide estas solicitudes cuidadosamente.

  Si `permissionMode` se establece en `bypassPermissions` y `allowUnsandboxedCommands` está habilitado, el modelo puede ejecutar autónomamente comandos fuera del sandbox sin solicitudes de aprobación, aparte de las [acciones que ningún modo auto-aprueba](/docs/es/permission-modes#actions-no-mode-auto-approves). Esta combinación efectivamente permite que el modelo escape del aislamiento de sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Ver también
</h2>

* [Descripción general del SDK](/docs/es/agent-sdk/overview) - Conceptos generales del SDK
* [Referencia del SDK de Python](/docs/es/agent-sdk/python) - Documentación del SDK de Python
* [Referencia de CLI](/docs/es/cli-reference) - Interfaz de línea de comandos
* [Flujos de trabajo comunes](/docs/es/common-workflows) - Guías paso a paso
