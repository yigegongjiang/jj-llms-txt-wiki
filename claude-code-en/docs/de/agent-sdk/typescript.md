> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK Referenz - TypeScript

> Vollständige API-Referenz für das TypeScript Agent SDK, einschließlich aller Funktionen, Typen und Schnittstellen.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  Installation
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  Das SDK bündelt eine native Claude Code-Binärdatei für Ihre Plattform als optionale Abhängigkeit wie `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Die meisten Installationen benötigen keine separate Claude Code-Installation. Die SDK-Version verfolgt die gebündelte Claude Code-Version. SDK v0.3.191 bündelt Claude Code v2.1.191, daher benötigt eine Funktion auf dieser Seite, die eine Claude Code-Version erfordert, die SDK-Version mit der gleichen Patch-Nummer oder später. Wenn Ihr Paketmanager optionale Abhängigkeiten überspringt, wirft das SDK `Native CLI binary for <platform>-<arch> not found`; setzen Sie stattdessen [`pathToClaudeCodeExecutable`](#options) auf eine separat installierte `claude`-Binärdatei.

  Wenn Ihr Paketmanager das `libc`-Feld von npm nicht anwendet, wie Yarn 1.x nicht, erhalten Sie unter Linux sowohl die glibc- als auch die musl-Plattformpakete, was die Installationsgröße ungefähr verdoppelt. Bei Agent SDK v0.2.141 oder später startet das SDK immer noch die richtige Variante. Um den Speicherplatz in einem Container-Image freizugeben, löschen Sie das Plattformpaket, das nicht mit der libc übereinstimmt, wo Ihre App ausgeführt wird; für eine glibc-Laufzeit auf x64 ist das `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`. Auf einem Entwicklungscomputer ist das Löschen vorübergehend, da Yarn das Paket bei der nächsten Abhängigkeitsänderung neu installiert.
</Note>

<h3 id="compile-to-a-single-executable">
  In eine einzelne ausführbare Datei kompilieren
</h3>

Wenn Sie Ihre Anwendung mit `bun build --compile` in eine einzelne ausführbare Datei kompilieren, kann das SDK die gebündelte CLI-Binärdatei zur Laufzeit nicht auflösen. `require.resolve` funktioniert nicht innerhalb des virtuellen Dateisystems `$bunfs` der kompilierten ausführbaren Datei, daher wirft das SDK `Native CLI binary for <platform>-<arch> not found`.

Um dieses Problem zu umgehen, betten Sie die Plattform-Binärdatei als Datei-Asset ein, extrahieren Sie sie beim Start mit `extractFromBunfs()` in einen echten Pfad und übergeben Sie diesen Pfad an [`pathToClaudeCodeExecutable`](#options).

Der `extractFromBunfs()`-Helfer erfordert `@anthropic-ai/claude-agent-sdk` v0.3.144 oder später. Das folgende Beispiel erstellt für macOS auf Apple Silicon:

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

`extractFromBunfs()` kopiert die eingebettete Binärdatei aus dem virtuellen Dateisystem der kompilierten ausführbaren Datei in ein benutzerabhängiges temporäres Verzeichnis und gibt den echten Pfad zurück. Außerhalb einer kompilierten ausführbaren Datei gibt es den Eingabepfad unverändert zurück, sodass derselbe Code in der Entwicklung ohne Änderungen ausgeführt wird.

Jede kompilierte ausführbare Datei bettelt eine einzelne Plattform-Binärdatei ein. Stimmen Sie das Plattformpaket im Import mit Ihrem `--target` ab:

* Zum Cross-Kompilieren installieren Sie das nicht übereinstimmende Plattformpaket, beispielsweise `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`.
* Unter Windows ist der Binär-Unterpfad `claude.exe`, beispielsweise `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`.

<h2 id="functions">
  Funktionen
</h2>

<h3 id="query">
  `query()`
</h3>

Die primäre Funktion für die Interaktion mit Claude Code. Erstellt einen asynchronen Generator, der Nachrichten streamt, sobald sie ankommen.

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
  Parameter
</h4>

| Parameter | Typ                                                              | Beschreibung                                                                               |
| :-------- | :--------------------------------------------------------------- | :----------------------------------------------------------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | Die Eingabeaufforderung als Zeichenkette oder asynchrones Iterable für den Streaming-Modus |
| `options` | [`Options`](#options)                                            | Optionales Konfigurationsobjekt (siehe Options-Typ unten)                                  |

<h4 id="returns">
  Rückgabewert
</h4>

Gibt ein [`Query`](#query-object)-Objekt zurück, das `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` mit zusätzlichen Methoden erweitert.

<h3 id="startup">
  `startup()`
</h3>

Wärmt die CLI-Subprozess vor, indem sie ihn spawnt und den Initialize-Handshake abschließt, bevor eine Eingabeaufforderung verfügbar ist. Das zurückgegebene [`WarmQuery`](#warmquery)-Handle akzeptiert später eine Eingabeaufforderung und schreibt sie in einen bereits bereiten Prozess, sodass der erste `query()`-Aufruf aufgelöst wird, ohne die Kosten für Subprozess-Spawn und Initialisierung inline zu zahlen.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  Parameter
</h4>

| Parameter             | Typ                   | Beschreibung                                                                                                                                                                                                         |
| :-------------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Optionales Konfigurationsobjekt. Identisch mit dem `options`-Parameter für `query()`                                                                                                                                 |
| `initializeTimeoutMs` | `number`              | Maximale Zeit in Millisekunden zum Warten auf die Subprozess-Initialisierung. Standardwert ist `60000`. Wenn die Initialisierung nicht rechtzeitig abgeschlossen wird, lehnt das Promise mit einem Timeout-Fehler ab |

<h4 id="returns-2">
  Rückgabewert
</h4>

Gibt ein `Promise<`[`WarmQuery`](#warmquery)`>` zurück, das aufgelöst wird, sobald der Subprozess gespawnt wurde und seinen Initialize-Handshake abgeschlossen hat.

<h4 id="example">
  Beispiel
</h4>

Rufen Sie `startup()` früh auf, beispielsweise beim Anwendungsstart, und rufen Sie dann `.query()` auf dem zurückgegebenen Handle auf, sobald eine Eingabeaufforderung bereit ist. Dies verlagert Subprozess-Spawn und Initialisierung aus dem kritischen Pfad.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Startup-Kosten im Voraus zahlen
const warm = await startup({ options: { maxTurns: 3 } });

// Später, wenn eine Eingabeaufforderung bereit ist, ist dies sofort
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Erstellt eine typsichere MCP-Tool-Definition zur Verwendung mit SDK-MCP-Servern.

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
  Parameter
</h4>

| Parameter     | Typ                                                                                                    | Beschreibung                                                                                                                                                                                                                                                                                                                                                                         |
| :------------ | :----------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | Der Name des Tools                                                                                                                                                                                                                                                                                                                                                                   |
| `description` | `string`                                                                                               | Eine Beschreibung, was das Tool tut                                                                                                                                                                                                                                                                                                                                                  |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | Zod-Schema, das die Eingabeparameter des Tools definiert (unterstützt sowohl Zod 3 als auch Zod 4)                                                                                                                                                                                                                                                                                   |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | Asynchrone Funktion, die die Tool-Logik ausführt                                                                                                                                                                                                                                                                                                                                     |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | Optionale Extras. `annotations` bietet MCP-Verhaltenshinweise für Clients. `searchHint` ist eine einzeilige Funktionsbeschreibung, die in der aufgeschobenen Tool-Liste angezeigt wird, wenn [Tool-Suche](/docs/de/agent-sdk/tool-search) aktiv ist. `alwaysLoad: true` behält das vollständige Schema dieses Tools in der anfänglichen Eingabeaufforderung bei, anstatt es aufzuschieben |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Erneut exportiert aus `@modelcontextprotocol/sdk/types.js`. Alle Felder sind optionale Hinweise; Clients sollten sich nicht auf sie für Sicherheitsentscheidungen verlassen.

| Feld              | Typ       | Standard    | Beschreibung                                                                                                                                                            |
| :---------------- | :-------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | Benutzerfreundlicher Titel für das Tool                                                                                                                                 |
| `readOnlyHint`    | `boolean` | `false`     | Wenn `true`, ändert das Tool seine Umgebung nicht                                                                                                                       |
| `destructiveHint` | `boolean` | `true`      | Wenn `true`, kann das Tool destruktive Aktualisierungen durchführen (nur sinnvoll, wenn `readOnlyHint` `false` ist)                                                     |
| `idempotentHint`  | `boolean` | `false`     | Wenn `true`, haben wiederholte Aufrufe mit denselben Argumenten keine zusätzliche Auswirkung (nur sinnvoll, wenn `readOnlyHint` `false` ist)                            |
| `openWorldHint`   | `boolean` | `true`      | Wenn `true`, interagiert das Tool mit externen Entitäten (beispielsweise Websuche). Wenn `false`, ist die Domäne des Tools geschlossen (beispielsweise ein Memory-Tool) |

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

Erstellt eine MCP-Server-Instanz, die im selben Prozess wie Ihre Anwendung ausgeführt wird.

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
  Parameter
</h4>

| Parameter              | Typ                           | Beschreibung                                                                                                                                                                                                                                                                                         |
| :--------------------- | :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.name`         | `string`                      | Der Name des MCP-Servers                                                                                                                                                                                                                                                                             |
| `options.version`      | `string`                      | Optionale Versionsnummer                                                                                                                                                                                                                                                                             |
| `options.instructions` | `string`                      | Optionale Server-Anweisungen, die von `initialize` zurückgegeben und dem Modell als MCP-Anweisungsblock angezeigt werden                                                                                                                                                                             |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | Array von Tool-Definitionen, die mit [`tool()`](#tool) erstellt wurden                                                                                                                                                                                                                               |
| `options.alwaysLoad`   | `boolean`                     | Wenn `true`, bleibt jedes Tool von diesem Server in der anfänglichen Eingabeaufforderung und wird niemals hinter [Tool-Suche](/docs/de/agent-sdk/tool-search) aufgeschoben. Kombiniert mit pro-Tool `alwaysLoad` in [`tool()`](#tool)                                                                     |
| `options.timeout`      | `number`                      | Timeout in Millisekunden für die Tool-Aufrufe dieses Servers. Claude Code wendet es auf diesen Server anstelle von [`MCP_TOOL_TIMEOUT`](/docs/de/env-vars) an. Übergeben Sie eine ganze Zahl von mindestens 1000. Claude Code ignoriert andere Werte. Erfordert TypeScript Agent SDK v0.3.248 oder später |

<h3 id="listsessions">
  `listSessions()`
</h3>

Entdeckt und listet vergangene Sitzungen mit leichten Metadaten auf. Filtern Sie nach Projektverzeichnis oder listen Sie Sitzungen über alle Projekte auf.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  Parameter
</h4>

| Parameter                  | Typ       | Standard    | Beschreibung                                                                                                                  |
| :------------------------- | :-------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `options.dir`              | `string`  | `undefined` | Verzeichnis, für das Sitzungen aufgelistet werden sollen. Wenn weggelassen, werden Sitzungen über alle Projekte zurückgegeben |
| `options.limit`            | `number`  | `undefined` | Maximale Anzahl der zurückzugebenden Sitzungen                                                                                |
| `options.includeWorktrees` | `boolean` | `true`      | Wenn `dir` sich in einem Git-Repository befindet, Sitzungen aus allen Worktree-Pfaden einbeziehen                             |

<h4 id="return-type-sdksessioninfo">
  Rückgabetyp: `SDKSessionInfo`
</h4>

| Eigenschaft    | Typ                   | Beschreibung                                                                                                   |
| :------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------- |
| `sessionId`    | `string`              | Eindeutige Sitzungs-ID (UUID)                                                                                  |
| `summary`      | `string`              | Anzeigetitel: benutzerdefinierter Titel, automatisch generierte Zusammenfassung oder erste Eingabeaufforderung |
| `lastModified` | `number`              | Letzte Änderungszeit in Millisekunden seit Epoch                                                               |
| `fileSize`     | `number \| undefined` | Sitzungsdateigröße in Bytes. Nur für lokale JSONL-Speicherung ausgefüllt                                       |
| `customTitle`  | `string \| undefined` | Vom Benutzer festgelegter Sitzungstitel (über `/rename`)                                                       |
| `firstPrompt`  | `string \| undefined` | Erste aussagekräftige Benutzer-Eingabeaufforderung in der Sitzung                                              |
| `gitBranch`    | `string \| undefined` | Git-Branch am Ende der Sitzung                                                                                 |
| `cwd`          | `string \| undefined` | Arbeitsverzeichnis für die Sitzung                                                                             |
| `tag`          | `string \| undefined` | Vom Benutzer festgelegtes Sitzungs-Tag (siehe [`tagSession()`](#tagsession))                                   |
| `createdAt`    | `number \| undefined` | Erstellungszeit in Millisekunden seit Epoch, vom Zeitstempel des ersten Eintrags                               |

<h4 id="example-2">
  Beispiel
</h4>

Geben Sie die 10 neuesten Sitzungen für ein Projekt aus. Die Ergebnisse werden nach `lastModified` absteigend sortiert, sodass das erste Element das neueste ist. Lassen Sie `dir` weg, um über alle Projekte zu suchen.

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

Liest Benutzer- und Assistenten-Nachrichten aus einem vergangenen Sitzungstranskript.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  Parameter
</h4>

| Parameter        | Typ      | Standard     | Beschreibung                                                                                            |
| :--------------- | :------- | :----------- | :------------------------------------------------------------------------------------------------------ |
| `sessionId`      | `string` | erforderlich | Sitzungs-UUID zum Lesen (siehe `listSessions()`)                                                        |
| `options.dir`    | `string` | `undefined`  | Projektverzeichnis, in dem die Sitzung zu finden ist. Wenn weggelassen, werden alle Projekte durchsucht |
| `options.limit`  | `number` | `undefined`  | Maximale Anzahl der zurückzugebenden Nachrichten                                                        |
| `options.offset` | `number` | `undefined`  | Anzahl der Nachrichten, die vom Anfang übersprungen werden sollen                                       |

<h4 id="return-type-sessionmessage">
  Rückgabetyp: `SessionMessage`
</h4>

| Eigenschaft          | Typ                     | Beschreibung                                                                                                                                                                                                                                                                                                   |
| :------------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Nachrichtenrolle                                                                                                                                                                                                                                                                                               |
| `uuid`               | `string`                | Eindeutige Nachrichten-ID                                                                                                                                                                                                                                                                                      |
| `session_id`         | `string`                | Sitzung, zu der diese Nachricht gehört                                                                                                                                                                                                                                                                         |
| `message`            | `unknown`               | Rohe Nachrichtennutzlast aus dem Transkript                                                                                                                                                                                                                                                                    |
| `parent_tool_use_id` | `string \| null`        | Für Subagenten-Nachrichten die `tool_use_id` des `Agent`- oder `Skill`-Tool-Aufrufs, der den Subagenten gestartet hat. `null` für Hauptsitzungs-Nachrichten und ältere Sitzungen                                                                                                                               |
| `parent_agent_id`    | `string \| null`        | Für Nachrichten von einem [verschachtelten Subagenten](/docs/de/sub-agents#let-subagents-spawn-their-own-subagents) die `agentId` des Subagenten, der ihn gespawnt hat. `null` für Hauptsitzungs-Nachrichten, Nachrichten von Top-Level-Subagenten und ältere Sitzungen. Erfordert Claude Code v2.1.202 oder später |

<h4 id="example-3">
  Beispiel
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

Liest Metadaten für eine einzelne Sitzung nach ID, ohne das vollständige Projektverzeichnis zu durchsuchen.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  Parameter
</h4>

| Parameter     | Typ      | Standard     | Beschreibung                                                                          |
| :------------ | :------- | :----------- | :------------------------------------------------------------------------------------ |
| `sessionId`   | `string` | erforderlich | UUID der zu suchenden Sitzung                                                         |
| `options.dir` | `string` | `undefined`  | Projektverzeichnispfad. Wenn weggelassen, werden alle Projektverzeichnisse durchsucht |

Gibt [`SDKSessionInfo`](#return-type-sdksessioninfo) zurück, oder `undefined`, wenn die Sitzung nicht gefunden wird.

<h3 id="renamesession">
  `renameSession()`
</h3>

Benennt eine Sitzung um, indem ein benutzerdefinierter Titeleintrag angehängt wird. Wiederholte Aufrufe sind sicher; der neueste Titel gewinnt.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  Parameter
</h4>

| Parameter     | Typ      | Standard     | Beschreibung                                                                          |
| :------------ | :------- | :----------- | :------------------------------------------------------------------------------------ |
| `sessionId`   | `string` | erforderlich | UUID der umzubenennenden Sitzung                                                      |
| `title`       | `string` | erforderlich | Neuer Titel. Muss nach dem Trimmen von Leerzeichen nicht leer sein                    |
| `options.dir` | `string` | `undefined`  | Projektverzeichnispfad. Wenn weggelassen, werden alle Projektverzeichnisse durchsucht |

<h3 id="tagsession">
  `tagSession()`
</h3>

Markiert eine Sitzung mit einem Tag. Übergeben Sie `null`, um das Tag zu löschen. Wiederholte Aufrufe sind sicher; das neueste Tag gewinnt.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  Parameter
</h4>

| Parameter     | Typ              | Standard     | Beschreibung                                                                          |
| :------------ | :--------------- | :----------- | :------------------------------------------------------------------------------------ |
| `sessionId`   | `string`         | erforderlich | UUID der zu markierenden Sitzung                                                      |
| `tag`         | `string \| null` | erforderlich | Tag-Zeichenkette oder `null` zum Löschen                                              |
| `options.dir` | `string`         | `undefined`  | Projektverzeichnispfad. Wenn weggelassen, werden alle Projektverzeichnisse durchsucht |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Löst die effektiven Claude Code-Einstellungen für ein bestimmtes Verzeichnis mit derselben Merge-Engine wie die CLI auf, ohne die Claude CLI zu spawnen. Verwenden Sie es, um zu überprüfen, welche Konfiguration ein `query()`-Aufruf sehen würde, bevor Sie einen aufrufen.

<Note>
  Diese Funktion ist Alpha und ihre API kann sich vor der Stabilisierung ändern.
</Note>

Der Snapshot unterscheidet sich von dem, was eine Live-`query()`-Sitzung anwendet:

* **`policyHelper`**: `resolveSettings()` liest MDM-Quellen, einschließlich macOS plist und Windows HKLM/HKCU, führt aber nicht den vom Administrator konfigurierten `policyHelper`-Subprozess aus.
* **Server-verwaltete Einstellungen**: `resolveSettings()` ruft [server-verwaltete Einstellungen](/docs/de/server-managed-settings#fetch-and-caching-behavior) nicht ab. Übergeben Sie sie als `options.serverManagedSettings`, um sie einzubeziehen.
* **`defaultMode`**: Der Snapshot gibt `permissions.defaultMode` unverändert aus jeder Ebene zurück, sodass er die Werte `'auto'` und `'bypassPermissions'` aus Projekt- und lokalen Einstellungen enthalten kann, die [eine Live-Sitzung ignoriert](/docs/de/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parameter
</h4>

`resolveSettings()` akzeptiert ein einzelnes Optionsobjekt. Alle Felder sind optional.

| Parameter                       | Typ                                   | Standard        | Beschreibung                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------------ | :------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.cwd`                   | `string`                              | `process.cwd()` | Verzeichnis, relativ zu dem Projekt- und lokale Einstellungen aufgelöst werden                                                                                                                                                                                                                                                                                      |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Alle Quellen    | Welche Dateisystem-Quellen geladen werden sollen. Übergeben Sie `[]`, um Benutzer-, Projekt- und lokale Einstellungen zu überspringen. [Endpoint-verwaltete Richtlinie](/docs/de/managed-settings#delivery-mechanisms) wird in allen Fällen geladen. `resolveSettings()` enthält server-verwaltete Einstellungen nur, wenn Sie `options.serverManagedSettings` übergeben |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Richtlinien-Tier-Einstellungen, die vom Embedding-Host bereitgestellt werden. Folgt denselben Regeln wie [`managedSettings` in `Options`](#options), außer dass `resolveSettings()` einen konfigurierten [`policyHelper`](/docs/de/settings-reference#policyhelper) nicht ausführt, sodass der Snapshot Einstellungen enthalten kann, die eine Live-Sitzung verwirft     |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Server-verwaltete Einstellungen-Nutzlast von `/api/claude_code/settings`. Nicht-restriktive Schlüssel werden ungefiltert durchgeleitet                                                                                                                                                                                                                              |

<h4 id="return-type-resolvedsettings">
  Rückgabetyp: `ResolvedSettings`
</h4>

`resolveSettings()` gibt ein Objekt zurück, das die zusammengeführten Einstellungen und die Quelle beschreibt, die jeden Schlüssel beigetragen hat.

| Eigenschaft  | Typ                                                 | Beschreibung                                                                              |
| :----------- | :-------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| `effective`  | `Settings`                                          | Zusammengeführte Einstellungen nach Anwendung aller aktivierten Quellen in Vorrangordnung |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Für jeden Top-Level-Schlüssel in `effective`, welche Quelle den Wert bereitgestellt hat   |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Pro-Quelle rohe Einstellungen, geordnet von niedrigster zu höchster Vorrangordnung        |

<h4 id="example-4">
  Beispiel
</h4>

Das folgende Beispiel löst Einstellungen für ein Projektverzeichnis auf und gibt die Quelle aus, die den Cleanup-Zeitraum steuert. Auf einem Computer, auf dem keine Einstellungsdatei `cleanupPeriodDays` setzt, zeigen beide gedruckten Zeilen `undefined` für den Wert an, was die erwartete Ausgabe ist, anstatt ein Fehler.

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
  Typen
</h2>

<h3 id="options">
  `Options`
</h3>

Konfigurationsobjekt für die `query()`-Funktion.

| Eigenschaft                       | Typ                                                                                                                                                                                                            | Standard                                                 | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                                  | Controller zum Abbrechen von Operationen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                                     | Zusätzliche Verzeichnisse, auf die Claude zugreifen kann. Das SDK übergibt jeden Eintrag an Claude Code als `--add-dir`, sodass Claude Code mit der `project`-Einstellung auch [die Fähigkeiten, Befehle und Subagenten des Verzeichnisses lädt](/docs/de/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                                              | Agent-Name für den Hauptthread. Der Agent muss in der `agents`-Option oder in den Einstellungen definiert sein                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                                              | Subagenten programmgesteuert definieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                                  | Wenn `true`, werden einzeilige Fortschrittsübersichten für Subagenten generiert und auf [`task_progress`](#sdktaskprogressmessage)-Ereignissen über das `summary`-Feld weitergeleitet. Gilt für Vordergrund- und Hintergrund-Subagenten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                                  | Ermöglicht das Umgehen von Berechtigungen. Erforderlich bei Verwendung von `permissionMode: 'bypassPermissions'`, beim Start oder später über `setPermissionMode()`. Siehe [Plan-Modus](/docs/de/agent-sdk/permissions#plan-mode-plan), um zu sehen, wie er mit `permissionMode: 'plan'` interagiert                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                                     | Tools, die automatisch genehmigt werden, ohne zu fragen. Dies beschränkt Claude nicht nur auf diese Tools. Wenn Sie eines der [Task-Tracking-Tools](/docs/de/agent-sdk/todo-tracking#model-availability) hier nennen, aktiviert Claude Code auch die Sitzung. Andere nicht aufgelistete Tools fallen unter `permissionMode` und `canUseTool`. Verwenden Sie `disallowedTools`, um Tools zu blockieren. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                                     | Beta-Funktionen aktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                                              | Benutzerdefinierte Berechtigungsfunktion, die nur aufgerufen wird, wenn der [Berechtigungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Eingabeaufforderung führt. Wird nicht für Aufrufe aufgerufen, die von `allowedTools`, Allow-Regeln oder `permissionMode` automatisch genehmigt werden. Eine Allow-Regel genehmigt nicht vorab die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves). Siehe [`CanUseTool`](#canusetool) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                                  | Setzen Sie das neueste Gespräch fort                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                                          | Aktuelles Arbeitsverzeichnis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                                  | Debug-Modus für den Claude Code-Prozess aktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                                              | Debug-Protokolle in einen bestimmten Dateipfad schreiben. Aktiviert implizit den Debug-Modus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                                     | Tools zu verweigern. Ein einfacher Name wie `"Bash"` entfernt das Tool aus Claudes Kontext. Eine scoped-Regel wie `"Bash(rm *)"` lässt das Tool verfügbar und verweigert übereinstimmende Aufrufe in jedem Berechtigungsmodus, einschließlich `bypassPermissions`, für den Befehl [wie geschrieben](/docs/de/permissions#bash-rule-limits). Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                                              | Steuert, wie viel Aufwand Claude in seine Antwort investiert. Funktioniert mit adaptivem Denken, um die Denktiefe zu lenken. Siehe [Anstrengungsstufe anpassen](/docs/de/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                                  | Dateiänderungsverfolgung zum Zurückspulen aktivieren. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                                            | Umgebungsvariablen. Wenn gesetzt, ersetzt dies die Subprocess-Umgebung, anstatt sie mit `process.env` zu zusammenzuführen, also übergeben Sie `{ ...process.env, YOUR_VAR: 'value' }`, um vererbte Variablen wie `PATH` zu behalten. Siehe [Langsame oder steckengebliebene API-Antworten behandeln](#handle-slow-or-stalled-api-responses) für ein Beispiel dieses Musters und [Umgebungsvariablen](/docs/de/env-vars) für Variablen, die die zugrunde liegende CLI liest. Setzen Sie `CLAUDE_AGENT_SDK_CLIENT_APP`, um Ihre App im User-Agent-Header zu identifizieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | Automatisch erkannt                                      | Zu verwendende JavaScript-Laufzeit                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                                     | Argumente, die an die ausführbare Datei übergeben werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                                     | Zusätzliche Argumente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                                              | Modell, das verwendet wird, wenn das primäre Modell fehlschlägt. Akzeptiert eine kommagetrennte Liste. Für die Reihenfolge und die Obergrenze siehe [Fallback-Modellketten](/docs/de/model-config#fallback-model-chains). Für Anleitungen siehe [Modell wählen](/docs/de/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                                  | Beim Fortsetzen mit `resume` zu einer neuen Sitzungs-ID verzweigen, anstatt die ursprüngliche Sitzung fortzusetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                                  | Subagenten-Text und Denkblöcke als Assistent- und Benutzernachrichten mit `parent_tool_use_id` gesetzt weiterleiten, damit Consumer ein verschachteltes Transkript rendern können. Ohne diese Option gibt Claude Code Subagenten-`tool_use`- und `tool_result`-Blöcke aus, aber keinen Text oder Denken. Nachrichten von Subagenten in jeder Verschachtelungstiefe werden auf Claude Code v2.1.219 und später weitergeleitet; vor v2.1.219 erschienen nur Nachrichten von Tiefe-1-Subagenten. Nachrichten von Subagenten, die ein verzweigter Skill erzeugt, und von verschachtelten verzweigten Skills erfordern v2.1.275 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                                     | Hook-Callbacks für Ereignisse                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                                  | Hook-Lebenszyklusereignisse in den Nachrichtenstrom als [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage) und [`SDKHookResponseMessage`](#sdkhookresponsemessage) einbeziehen. Lebenszyklusereignisse für `SessionStart`- und `Setup`-Hooks sind immer enthalten und benötigen diese Option nicht. Einige Hook-Ereignisse wie `Notification`, `SessionEnd`, `PreCompact` und `PostCompact` erzeugen niemals eine `SDKHookStartedMessage`, auch nicht mit dieser Option. Für diese Ereignisse gibt Claude Code weiterhin eine `SDKHookProgressMessage` aus, während ein Befehl-Hook, der länger als eine Sekunde läuft, Ausgabe erzeugt, und gibt eine `SDKHookResponseMessage` nur aus, wenn ein Hook [der im Hintergrund läuft](/docs/de/hooks#run-hooks-in-the-background) beendet wird                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                                  | Teilweise Nachrichtenereignisse einbeziehen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                                  | *Alpha.* Timeout in Millisekunden für jeden `sessionStore.load()`- und `sessionStore.listSubkeys()`-Aufruf während der Materialisierung beim Fortsetzen. Wenn sich der Adapter nicht innerhalb dieses Fensters einigt, schlägt die Abfrage fehl, anstatt zu hängen. Wird ignoriert, wenn `sessionStore` nicht gesetzt ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                                              | Richtlinien-Tier-Einstellungen, die Ihr Host-Prozess der erzeugten Sitzung bereitstellt. Auf Maschinen mit von Administratoren bereitgestellten verwalteten Einstellungen ignoriert Claude Code diese, es sei denn, die höchste Priorität der verwalteten Quelle des Administrators setzt `parentSettingsBehavior: 'merge'`, und führt sie niemals zusammen, während ein [`policyHelper`](/docs/de/settings-reference#policyhelper) verwaltete Einstellungen bereitstellt. Zusammengeführte Werte durchlaufen einen restriktiven Filter; [Übergeordnete Einstellungen einschränken](/docs/de/claude-apps-gateway#restrict-parent-settings) behandelt, was der Filter zulässt und die `allowManaged*Only`-Sperren. Ein Host, der [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars) setzt, hat drei Schlüssel, die direkt aus dieser Nutzlast gelesen werden: seine [Modellkonfiguration](/docs/de/model-config#restrict-model-selection) auf Claude Code v2.1.222 oder später, [`modelPricing`](/docs/de/settings-reference#modelpricing), wenn keine verwaltete Quelle sie auf v2.1.246 oder später setzt, und seinen `ENABLE_TOOL_SEARCH`-Env-Eintrag auf v2.1.247 oder später                                                                                                                                                                                                                                                                                                                                                         |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                                              | Beenden Sie die Abfrage, wenn die clientseitige Kostenschätzung diesen USD-Wert erreicht. Zählt nur die Ausgaben des Aufrufs selbst; Gesamtwerte, die aus einer fortgesetzten Sitzung wiederhergestellt werden, zählen nicht. Für Genauigkeitsvorbehalt und Zurücksetzen-Verhalten siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                                              | *Veraltet:* Verwenden Sie stattdessen `thinking`. Maximale Token für den Denkprozess                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                                              | Maximale agentengesteuerte Umdrehungen (Tool-Use-Roundtrips)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                                     | MCP-Serverkonfigurationen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `model`                           | `string`                                                                                                                                                                                                       | Standard aus CLI                                         | Claude-Modellalias oder vollständiger Modellname. Siehe [akzeptierte Werte und anbieter-spezifische IDs](/docs/de/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                                              | Callback zur Behandlung von MCP-Elicitierungsanfragen. Wird aufgerufen, wenn ein MCP-Server Benutzereingaben anfordert und kein Hook es zuerst behandelt. Wenn nicht bereitgestellt, werden unbehandelte Elicitierungsanfragen automatisch abgelehnt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                                              | Definieren Sie das Ausgabeformat für Agentenergebnisse. Siehe [Strukturierte Ausgaben](/docs/de/agent-sdk/structured-outputs) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                                              | Keine `Options`-Feld. Setzen Sie `outputStyle` im Inline-[`settings`](/docs/de/settings)-Objekt oder einer Einstellungsdatei. Siehe [Ausgabestil aktivieren](/docs/de/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | Automatisch aufgelöst aus gebündelter nativer Binärdatei | Pfad zur Claude Code-Ausführungsdatei. Nur erforderlich, wenn optionale Abhängigkeiten während der Installation übersprungen wurden oder Ihre Plattform nicht in der unterstützten Menge enthalten ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                                              | Berechtigungsmodus für die Sitzung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                                              | MCP-Tool-Name für Berechtigungsaufforderungen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                                                 | Wer beantwortet Berechtigungsaufforderungen: `'host'` leitet sie an Ihren [`canUseTool`](#canusetool)-Callback oder das `permissionPromptToolName`-Tool weiter, und `'none'` [verweigert die Aufrufe, die sonst aufgefordert hätten](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated). Erfordert Claude Code v2.1.259 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                                   | Wenn `false`, wird die Sitzungspersistenz auf der Festplatte deaktiviert. Sitzungen können später nicht fortgesetzt werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                                              | Benutzerdefinierte Workflow-Anweisungen für den Plan-Modus. Wenn `permissionMode` `'plan'` ist, ersetzt dieser String den Standard-Plan-Modus-Workflow-Text. Die CLI umhüllt ihn immer noch mit der schreibgeschützten Durchsetzungspräambel und der ExitPlanMode-Protokoll-Fußzeile                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                                     | Laden Sie benutzerdefinierte Plugins aus lokalen Pfaden. Siehe [Plugins](/docs/de/agent-sdk/plugins) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                                              | Absoluter Pfad des vertrauenswürdigen Checkouts, von dem `cwd` ein Worktree ist. Claude Code liest Projekteinstellungen, `.mcp.json` und die Befehle, Agenten, Fähigkeiten, Workflows, Routinen und Ausgabestile des Projekts aus `.claude/` aus diesem Verzeichnis anstelle von `cwd`, und setzt `CLAUDE_PROJECT_DIR` darauf. Hooks, Hilfsskripte wie `apiKeyHelper` und stdio MCP-Server starten mit diesem Verzeichnis als Arbeitsverzeichnis. `CLAUDE.md`-Dateien und `.claude/rules/` werden immer noch von `cwd` geladen. Erfordert Claude Code v2.1.275 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                                  | Eingabeaufforderungsvorschläge aktivieren. Nach einer Umdrehung gibt Claude Code eine `prompt_suggestion`-Nachricht mit einer vorhergesagten nächsten Benutzereingabeaufforderung aus. Claude Code generiert für einige Umdrehungen keinen Vorschlag, z. B. während Ihr Konto sich dem Nutzungslimit nähert oder dieses erreicht hat. Siehe [Wenn Claude Code Vorschläge überspringt](/docs/de/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                                              | Sitzungs-ID zum Fortsetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                                              | Mit `resumeSessionAt`: die Eingabeaufforderungs-UUID der Umdrehung, die das Truncating-Resume verwerfen soll. Claude Code verweigert das Fortsetzen, wenn der verworfene Bereich etwas enthält, das nicht dieser Umdrehung zugeordnet werden kann, z. B. absorbierte Warteschlangen-Nachrichten oder Task-Benachrichtigungen, und nennt das `--resume-drops-turn`-Flag in der Ablehnungsmeldung. Nur das Agent SDK und Print-Modus-Resumes lesen das Paar. Erfordert Claude Code v2.1.223 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                                              | Sitzung bei einer bestimmten Nachrichten-UUID fortsetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                                              | Konfigurieren Sie das Sandbox-Verhalten programmgesteuert. Siehe [Sandbox-Einstellungen](#sandboxsettings) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `sessionId`                       | `string`                                                                                                                                                                                                       | Automatisch generiert                                    | Verwenden Sie eine bestimmte UUID für die Sitzung, anstatt eine automatisch zu generieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sessionStore`                    | [`SessionStore`](/docs/de/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                                              | Spiegeln Sie Sitzungstranskripte zu einem externen Backend, damit ein anderer Host sie fortsetzen kann. Siehe [Sitzungen in externem Speicher persistieren](/docs/de/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                                              | *Alpha.* Flush-Modus für `sessionStore`. Wird ignoriert, wenn `sessionStore` nicht gesetzt ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                                              | Inline-[settings](/docs/de/settings)-Objekt, ein Einstellungsdateipfad oder eine Inline-JSON-Zeichenkette. Füllt die Flag-Einstellungsebene in der [Rangfolge](/docs/de/settings#settings-precedence). Ändern Sie zur Laufzeit mit [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | CLI-Standards (alle Quellen)                             | Steuern Sie, welche Dateisystem-Einstellungen geladen werden. Übergeben Sie `[]`, um Benutzer-, Projekt- und lokale Einstellungen zu deaktivieren. [Endpoint-verwaltete Richtlinie](/docs/de/managed-settings#delivery-mechanisms) wird unabhängig geladen; server-verwaltete Einstellungen werden abgerufen, wenn sich die Sitzung mit einer Organisationsanmeldeinformation bei einer [berechtigten Konfiguration](/docs/de/server-managed-settings#platform-availability) authentifiziert. Siehe [Claude Code-Funktionen verwenden](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                                              | Fähigkeiten, die der Sitzung zur Verfügung stehen. Übergeben Sie `'all'`, um jede entdeckte Fähigkeit zu aktivieren, oder eine Liste von Fähigkeitsnamen. Übergeben Sie nur exakte Namen. Auf Agent SDK v0.3.221 oder später lehnt das SDK fehlerhafte und Wildcard-Form-Namen mit einem Fehler ab, bevor der Claude Code-Prozess gestartet wird. Wenn gesetzt, fügt das SDK das Skill-Tool automatisch zu `allowedTools` hinzu. Wenn Sie auch `tools` übergeben, beziehen Sie `'Skill'` in diese Liste ein. Siehe [Fähigkeiten](/docs/de/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                                              | Benutzerdefinierte Funktion zum Erzeugen des Claude Code-Prozesses. Verwenden Sie, um Claude Code in VMs, Containern oder Remote-Umgebungen auszuführen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                                              | Callback für stderr-Ausgabe                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                                  | Verwenden Sie nur die Server, die in `mcpServers` übergeben werden, und ignorieren Sie Projekt `.mcp.json`, Benutzereinstellungen, Plugin-bereitgestellte MCP-Server und [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (minimale Eingabeaufforderung)               | Systemanfrage-Konfiguration. Übergeben Sie eine Zeichenkette für eine benutzerdefinierte Eingabeaufforderung oder `{ type: 'preset', preset: 'claude_code' }`, um Claude Codes Systemeingabeaufforderung zu verwenden. Übergeben Sie ein Array von Zeichenketten mit der exportierten `SYSTEM_PROMPT_DYNAMIC_BOUNDARY`-Konstante zwischen den statischen und pro-Anfrage-Teilen, um [den statischen Teil einer benutzerdefinierten Eingabeaufforderung zu cachen](/docs/de/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt). Wenn Sie die Preset-Objektform verwenden, fügen Sie `append` hinzu, um sie mit zusätzlichen Anweisungen zu erweitern, und setzen Sie `excludeDynamicSections: true`, um sitzungsspezifischen Kontext in die erste Benutzernachricht zu verschieben, um [bessere Prompt-Cache-Wiederverwendung über Maschinen hinweg](/docs/de/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) zu erreichen. Setzen Sie `snapshot: false`, um die Eingabeaufforderung bei jeder Anfrage neu zu erstellen, anstatt [die Eingabeaufforderung wiederzuverwenden, die die Sitzung bei ihrer ersten Anfrage aufgezeichnet hat](/docs/de/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Um `snapshot` auf einer benutzerdefinierten Eingabeaufforderung zu setzen, übergeben Sie die `{ type: 'custom', prompt }`-Form. Die `{ type: 'custom' }` Form und das `snapshot`-Feld erfordern TypeScript Agent SDK v0.3.257 oder später |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                                              | *Alpha.* API-seitiges Task-Budget in Token. Wenn gesetzt, wird dem Modell sein verbleibendes Token-Budget mitgeteilt, damit es die Tool-Nutzung pacing kann und vor dem Limit abwickelt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | `{ type: 'adaptive' }` für unterstützte Modelle          | Steuert Claudes Denk-/Reasoning-Verhalten. Siehe [`ThinkingConfig`](#thinkingconfig) für Optionen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                                              | Anzeigetitel für die Sitzung. Beim Fortsetzen über `resume` oder `continue` hat der Titel der fortgesetzten Sitzung Vorrang; verwenden Sie [`renameSession()`](#renamesession), um eine vorhandene Sitzung umzubenennen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                                              | Ordnen Sie integrierte Tool-Namen MCP-Tool-Namen zu, damit Claude Ihre MCP-Implementierung anstelle der integrierten aufruft. Zum Beispiel `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                                              | Konfiguration für das Verhalten integrierter Tools. Siehe [`ToolConfig`](#toolconfig) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                                              | Tool-Konfiguration. Übergeben Sie ein Array von Tool-Namen oder verwenden Sie die Voreinstellung, um Claude Codes Standard-Tools zu erhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

<h4 id="handle-slow-or-stalled-api-responses">
  Langsame oder steckengebliebene API-Antworten behandeln
</h4>

Der CLI-Subprocess liest mehrere Umgebungsvariablen, die API-Timeouts und Stall-Erkennung steuern. Übergeben Sie sie über die `env`-Option:

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

* `API_TIMEOUT_MS`: Pro-Anfrage-Timeout auf dem Anthropic-Client in Millisekunden. Standard `600000`. Gilt für die Hauptschleife und alle Subagenten.
* `CLAUDE_CODE_MAX_RETRIES`: maximale API-Wiederholungen. Standard `10`, begrenzt auf `15`. Jede Wiederholung erhält sein eigenes `API_TIMEOUT_MS`-Fenster, daher ist die schlimmste Wandzeit ungefähr `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus Backoff. Für unbeaufsichtigte Läufe, die längere Ausfallzeiten abwarten müssen, setzen Sie [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/de/errors#tune-retry-behavior): Es versucht transiente Kapazitätsfehler unbegrenzt erneut und erhöht auf Claude Code v2.1.199 oder später den Standard für andere transiente Fehler auf `300` und entfernt die Obergrenze für diese Variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: Stall-Watchdog für Subagenten. Während der Stream-Watchdog eingeschaltet ist, ist der Standard `CLAUDE_STREAM_IDLE_TIMEOUT_MS` plus 5 Minuten, was `600000` ist, es sei denn, Sie erhöhen diese Variable. Mit dem Stream-Watchdog aus ist der Standard `600000`. Vor v2.1.257 war der Standard immer `600000`.

  Der Timer wird bei jedem Stream-Ereignis zurückgesetzt. Bei einem Stall bricht Claude Code den Subagenten ab und meldet den Stall dem übergeordneten Element. Für einen Hintergrund-Subagenten markiert es auch die Task als fehlgeschlagen und fügt alle Teilergebnisse an.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` mit `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: Stream-Watchdog, der die Anfrage abbricht, wenn Header angekommen sind, aber der Antwortkörper nicht mehr streamt. Der Watchdog ist standardmäßig für alle Anbieter aktiviert; setzen Sie `CLAUDE_ENABLE_STREAM_WATCHDOG=0`, um ihn zu deaktivieren. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` hat einen Standard von `300000` und ist auf dieses Minimum begrenzt. Nach dem Abbruch behandelt [Automatische Wiederholungen](/docs/de/errors#automatic-retries) das, was Claude Code tut, basierend darauf, wie weit die Antwort fortgeschritten war.

  Während der Watchdog auf eine Antwort wartet, die ein Gateway hinter `ANTHROPIC_BASE_URL` mit Keep-Alive-Pings offen hält, empfängt ein Host, der `includePartialMessages` setzt, weiterhin `ping`-[Stream-Ereignisse](#sdkpartialassistantmessage), daher lesen Sie diese Frames als Lebendigkeit, anstatt die Sitzung bei Stille zu zeitlich zu begrenzen. Vor v2.1.257 stoppten die Frames 5 Minuten nach dem letzten echten Stream-Ereignis.

<h3 id="query-object">
  `Query`-Objekt
</h3>

Schnittstelle, die von der `query()`-Funktion zurückgegeben wird.

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
  Methoden
</h4>

| Methode                                | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Unterbricht die Abfrage. Nur im Streaming-Eingabemodus verfügbar. Wenn die CLI die `interrupt_receipt_v1`-Fähigkeit in [`SDKSystemMessage.capabilities`](#sdksystemmessage) ankündigt, wird mit einer [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) aufgelöst, die die Nachrichten auflistet, die ausstanden, als die Unterbrechung ankam. Wird auf CLIs vor v2.1.205 mit `undefined` aufgelöst                                                                                                                                                                |
| `rewindFiles(userMessageId, options?)` | Stellt Dateien in ihren Zustand bei der angegebenen Benutzernachricht wieder her. Übergeben Sie `{ dryRun: true }`, um Änderungen in der Vorschau anzuzeigen. Erfordert `enableFileCheckpointing: true`. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                         |
| `setPermissionMode()`                  | Ändert den Berechtigungsmodus (nur im Streaming-Eingabemodus verfügbar)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `setModel()`                           | Ändert das Modell (nur im Streaming-Eingabemodus verfügbar). Übergeben von `undefined` oder der Zeichenkette `"default"` setzt auf [Claude Codes Standard-Modell](/docs/de/model-config) zurück                                                                                                                                                                                                                                                                                                                                                                                     |
| `setMaxThinkingTokens()`               | *Veraltet:* Verwenden Sie stattdessen die `thinking`-Option. Ändert die maximalen Denk-Token. Übergeben von `null` setzt das Denken auf den Sitzungsstandard zurück: eine Mid-Session-Überschreibung wird gelöscht, und das Denken bleibt für Sitzungen, die es deaktiviert haben, ausgeschaltet                                                                                                                                                                                                                                                                               |
| `applyFlagSettings(settings)`          | Führt Einstellungen zur Laufzeit in die Flag-Einstellungsebene der Sitzung zusammen (nur im Streaming-Eingabemodus verfügbar). Siehe [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                               |
| `updateSettings(source, settings)`     | Schreibt einen Allowlist-Schlüssel in die lokale Einstellungsdatei des Projekts oder Ihre Benutzereinstellungsdatei, damit der Wert für spätere Sitzungen persistiert. Siehe [`updateSettings()`](#updatesettings). Erfordert TypeScript SDK v0.3.257 oder später, das Claude Code v2.1.257 bündelt                                                                                                                                                                                                                                                                            |
| `initializationResult()`               | Gibt das vollständige Initialisierungsergebnis zurück, einschließlich unterstützter Befehle, Modelle, Kontoinformationen und Ausgabestil-Konfiguration                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `reinitialize()`                       | Sendet die `initialize`-Steueranfrage erneut an die laufende CLI und gibt ein frisches Ergebnis anstelle des zwischengespeicherten First-Connect-Ergebnisses zurück. Verwenden Sie es nach einer Transportlücke, z. B. nach dem Wiederherstellen einer Verbindung zu einer Sitzung nach einer Trennung, damit ausstehende Berechtigungsanfragen Ihren `canUseTool`-Callback erneut erreichen. Machen Sie den Callback idempotent pro Anfrage-ID, da eine Anfrage, deren Antwort verloren ging, erneut versendet wird. Erfordert Claude Code v2.1.195 oder später               |
| `supportedCommands()`                  | Gibt verfügbare Befehle zurück. Ab Agent SDK v0.3.216 spiegelt die Liste Mid-Session-Befehlsänderungen wider; siehe [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                                                                                                                                                                                                                  |
| `supportedModels()`                    | Gibt verfügbare Modelle mit Anzeigeinformationen zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `supportedAgents()`                    | Gibt verfügbare Subagenten als [`AgentInfo`](#agentinfo)`[]` zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `mcpServerStatus()`                    | Gibt Status verbundener MCP-Server als [`McpServerStatus`](#mcpserverstatus)`[]` zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `getContextUsage(opts?)`               | Gibt eine [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) zurück, die die Kontextfenster-Nutzung der Sitzung nach Kategorie, Fähigkeit und Tool aufschlüsselt. Mit dem Standard `detail` ist es die gleichen Daten, die `/context` in einer interaktiven Sitzung anzeigt. Die [`detail`-Option](#sdkcontrolgetcontextusageresponse) erfordert Agent SDK v0.3.257 oder später                                                                                                                                                                         |
| `readFile(path, options?)`             | Liest eine Datei aus dem Dateisystem der Sitzung. Claude Code löst den Pfad gegen `cwd` auf; [Was `readFile()` lesen kann](#what-readfile-can-read) listet die Dateien auf, die es bereitstellt. Übergeben Sie `{ maxBytes }`, um die Leseobergrenze zu ändern (Standard 1 MB, Obergrenze 10 MB) und `{ encoding: 'base64' }` für Binärdateien wie Bilder. Wird mit einer [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse) aufgelöst oder `null` bei Berechtigungsverweigerung, fehlender Datei oder Transportfehler. Erfordert TypeScript SDK v0.2.121 oder später |
| `reloadSkills()`                       | Lädt Fähigkeiten von der Festplatte neu, sodass Fähigkeiten, die Sie mid-session hinzufügen oder bearbeiten, der laufenden Sitzung zur Verfügung stehen. Wird mit einer [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) aufgelöst, die die nach dem Neuladen verfügbaren Fähigkeiten auflistet. Erfordert Agent SDK v0.3.163 oder später                                                                                                                                                                                                                   |
| `accountInfo()`                        | Gibt Kontoinformationen zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `reconnectMcpServer(serverName)`       | Verbinden Sie einen MCP-Server nach Name erneut. Wenn der Name auch einem Eintrag in einer Einstellungsdatei wie `.mcp.json` oder `~/.claude.json` entspricht, verbindet Claude Code den Server, den Sie über [`mcpServers`](#options) oder `setMcpServers()` konfiguriert haben, nicht den Einstellungsdatei-Eintrag. Diese Auflösungsreihenfolge erfordert Claude Code v2.1.257 oder später                                                                                                                                                                                  |
| `toggleMcpServer(serverName, enabled)` | Aktivieren oder deaktivieren Sie einen MCP-Server nach Name, mit der gleichen Namensauflösung wie `reconnectMcpServer()`. Das Deaktivieren trennt den Server                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `setMcpServers(servers)`               | Ersetzen Sie dynamisch die Menge der MCP-Server für diese Sitzung. Wird mit einem [`McpSetServersResult`](#mcpsetserversresult) aufgelöst, das benennt, welche Server hinzugefügt und entfernt wurden, und alle Fehler                                                                                                                                                                                                                                                                                                                                                         |
| `readMcpResource(serverName, uri)`     | *Alpha.* Liest eine MCP Apps `ui://`-Ressource von einem verbundenen MCP-Server, damit Ihre Anwendung das Widget eines Tools rendern kann. Wird mit einer [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse) aufgelöst. Erfordert TypeScript Agent SDK v0.3.280 oder später                                                                                                                                                                                                                                                                             |
| `streamInput(stream)`                  | Streamen Sie Eingabenachrichten zur Abfrage für Multi-Turn-Gespräche                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `stopTask(taskId)`                     | Beenden Sie eine laufende Hintergrund-Task nach ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `close()`                              | Schließen Sie die Abfrage und beenden Sie den zugrunde liegenden Prozess. Beendet die Abfrage erzwungen und bereinigt alle Ressourcen                                                                                                                                                                                                                                                                                                                                                                                                                                          |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Ändert [Einstellungen](/docs/de/settings) auf einer laufenden Sitzung, ohne die Abfrage neu zu starten. Verwenden Sie es, wenn eine Einstellung, die keinen dedizierten Setter hat, mid-session geändert werden muss, z. B. das Verschärfen von `permissions`, nachdem der Agent nicht vertrauenswürdige Eingaben liest. `setModel()` und `setPermissionMode()` sind dedizierte Setter für diese beiden Schlüssel; `applyFlagSettings()` ist die allgemeine Form, die jede Teilmenge der Einstellungsschlüssel akzeptiert, und das Übergeben von `model` hier verhält sich gleich wie `setModel()`.

Nur einige Schlüssel treten mid-session in Kraft:

* **Angewendet bei der nächsten Umdrehung**: `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Das Wechseln von `agent` wendet auch die Modellüberschreibung und Hooks dieses Agenten bei der nächsten Umdrehung an. Sein Systemprompt wird bei der nächsten Umdrehung angewendet oder, in einer Sitzung, die [einen aufgezeichneten Systemprompt wiederverwenden](/docs/de/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session), sobald die Sitzung komprimiert wird.
* **Während der aktuellen Umdrehung angewendet**: `model`. Wenn Sie `model` wechseln, während Claude an einer Umdrehung arbeitet, wird die Antwort, die Claude bereits generiert, auf dem alten Modell beendet, und der Rest der Umdrehung, beginnend mit dem nächsten Aufruf, den Claude Code an das Modell macht, verwendet das neue. Subagenten behalten ihr eigenes Modell. Vor v2.1.212 wartete ein Mid-Turn-Wechsel auf die nächste Umdrehung.
* **Keine Auswirkung mid-session**: die Systemprompt-Optionen. Diese werden einmal beim Start aufgelöst, daher behält die laufende Sitzung den ursprünglichen Wert, auch wenn der Aufruf erfolgreich ist. Um sie zu ändern, starten Sie eine neue Sitzung.

`effortLevel` akzeptiert einen [Anstrengungsstufen](/docs/de/model-config#adjust-effort-level)-Namen. Es akzeptiert auch `"ultracode"`, das `xhigh`-Anstrengung mit [ultracode](/docs/de/workflows#let-claude-decide-with-ultracode) anfordert. `applyFlagSettings()` deklariert `effortLevel` ohne diesen Wert, daher übergeben Sie das Äquivalent `{ ultracode: true }` in TypeScript. Der `ultracode`-Wert erfordert Claude Code v2.1.203 oder später und wird nur von `applyFlagSettings()` akzeptiert, nicht vom `effortLevel`-Schlüssel in einer Einstellungsdatei.

Die Werte werden in die Flag-Einstellungsebene geschrieben, die gleiche Ebene, die die Inline-`settings`-Option von `query()` beim Start füllt. Dies ist die gleiche Ebene, die der [On-Page-Rangfolge-Abschnitt](#settings-precedence) programmatische Optionen nennt.

Aufeinanderfolgende Aufrufe führen Top-Level-Schlüssel flach zusammen. Ein zweiter Aufruf mit `{ permissions: {...} }` ersetzt das gesamte `permissions`-Objekt aus dem vorherigen Aufruf, anstatt es tief zusammenzuführen.

Um einen Schlüssel zu löschen, den Sie mit `applyFlagSettings()` gesetzt haben, übergeben Sie `null` für diesen Schlüssel. Die meisten Schlüssel fallen dann auf Quellen mit niedrigerer Priorität zurück. Ein gelöschtes `model` wird auf [Claude Codes Standard-Modell](/docs/de/model-config) zurückgesetzt, auch wenn eine Einstellungsdatei `model` setzt. Übergeben von `undefined` hat keine Auswirkung, da JSON-Serialisierung es ablegt.

Drei Schlüssel neben `model` setzen Sitzungszustand zurück, anstatt auf Quellen mit niedrigerer Priorität zurückzufallen:

* `effortLevel: null` gibt die Sitzung zur Standard-Anstrengungsstufe des Modells zurück, nicht zur `effort`-Option von `query()` oder einem `effortLevel` aus einer Einstellungsdatei.
* `agent: null` führt den Hauptthread ohne Agenten aus, beginnend mit der nächsten Umdrehung, anstatt die `agent`-Option von `query()` oder einen `agent` aus einer Einstellungsdatei wiederherzustellen. Wenn der gelöschte Agent sein eigenes Modell angewendet hatte, kehrt die Sitzung zum Modell zurück, das sie beim Start aufgelöst hat.
* `ultracode: null` schaltet ultracode aus, wie `false` es tut, anstatt einen `ultracode`-Wert aus einer Einstellungsdatei wiederherzustellen. Die Sitzung behält ihre aktuelle Anstrengungsstufe, daher übergeben Sie `effortLevel` im gleichen Aufruf, um sie zu ändern.

Nur im Streaming-Eingabemodus verfügbar, die gleiche Einschränkung wie `setModel()` und `setPermissionMode()`.

Das Beispiel unten wechselt das aktive Modell mid-session und löscht dann die Überschreibung, damit das Modell auf [Claude Codes Standard-Modell](/docs/de/model-config) zurückgesetzt wird.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Override the model for the rest of the session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Later: clear the override; the model resets to Claude Code's default
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` ist nur TypeScript. Das Python SDK stellt keine äquivalente Methode bereit.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

Schreibt einen Allowlist-Schlüssel in eine Einstellungsdatei auf der Festplatte, damit der Wert für spätere Sitzungen persistiert, die diese Quelle laden. Jede Quelle akzeptiert einen Schlüssel, mit einem Zeichenkettenwert:

* **`"localSettings"`**: akzeptiert `outputStyle` und führt es in die lokale Einstellungsdatei des Projekts `.claude/settings.local.json` zusammen. Der neue Stil tritt bei der nächsten Anfrage der Sitzung in Kraft.
* **`"userSettings"`**: akzeptiert `effortLevel` und speichert es als Standard-[Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) für das aktuelle Modell der Sitzung, unter [`modelSettings`](/docs/de/settings-reference#modelsettings) in Ihrer Benutzereinstellungsdatei. Das Übergeben von `max` schreibt nichts, da `max` nur für Sitzungen ist. Die laufende Sitzung behält ihre aktuelle Anstrengungsstufe entweder so, daher rufen Sie [`applyFlagSettings()`](#applyflagsettings) auf, wenn Sie das auch ändern möchten. Diese Quelle erfordert TypeScript SDK v0.3.277 oder später, das Claude Code v2.1.277 bündelt.

Der Aufruf lehnt ab, wenn die Anfrage einen anderen Schlüssel trägt, wenn die Sitzung über einen Remote-Transport läuft, und wenn die [`settingSources`](#options) der Sitzung die Quelle ausschließen, die Sie nennen. Das Löschen eines Schlüssels wird nicht unterstützt.

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle, das von [`startup()`](#startup) zurückgegeben wird. Der Subprocess ist bereits erzeugt und initialisiert, daher schreibt das Aufrufen von `query()` auf diesem Handle die Eingabeaufforderung direkt in einen bereiten Prozess ohne Startup-Latenz.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Methoden
</h4>

| Methode         | Beschreibung                                                                                                                                                             |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Senden Sie eine Eingabeaufforderung an den vorgewärmten Subprocess und geben Sie eine [`Query`](#query-object) zurück. Kann nur einmal pro `WarmQuery` aufgerufen werden |
| `close()`       | Schließen Sie den Subprocess, ohne eine Eingabeaufforderung zu senden. Verwenden Sie dies, um eine warme Abfrage zu verwerfen, die nicht mehr benötigt wird              |

`WarmQuery` implementiert `AsyncDisposable`, daher kann es mit `await using` für automatische Bereinigung verwendet werden.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Rückgabetyp von `initializationResult()`. Enthält Sitzungsinitialisierungsdaten.

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

`hooks_applied` meldet, ob Claude Code die `hooks` registriert hat, die die `initialize`-Anfrage trug. Das SDK sendet diese Anfrage einmal, wenn die Sitzung startet, und erneut bei jedem [`reinitialize()`](#query-object)-Aufruf. Das Feld erfordert Agent SDK v0.3.238 oder später.

Claude Code lässt das Feld weg, wenn die Anfrage keine Hooks trug. Wenn die Anfrage Hooks trug, hängt der Wert davon ab, ob die Anfrage die erste Initialisierung der Sitzung ist und, für eine wiederholte, wie sie die Sitzung erreichte:

* `true`: Claude Code registrierte die Hooks. Eine erste Initialisierung einer Sitzung gibt diesen Wert zurück. Eine wiederholte Initialisierung, die über die CLI-Standardeingabe gesendet wird, gibt auch `true` zurück. In diesem Fall ersetzen die Hooks in der neuen Anfrage die Hooks, die früher registriert wurden.
* `false`: Claude Code ignorierte die Hooks. Eine wiederholte Initialisierung, die an eine Remote-Sitzung gesendet wird, gibt diesen Wert zurück, daher kann ein zweiter Client, der einer Sitzung beitritt, die Hooks, die der erste Client registrierte, nicht ersetzen.

Vor Agent SDK v0.3.238 trug die Antwort niemals das Feld, und Claude Code ignorierte `hooks` bei jeder wiederholten Initialisierung.

Die Antwort meldet immer `fast_mode_state`, und wenn etwas [Fast-Modus](/docs/de/fast-mode) blockiert, trägt `fast_mode_disabled_reason` den Grund-Code daneben, daher können Sie den blockierten Zustand erklären, anstatt die Verfügbarkeit neu abzuleiten. Beide Verhaltensweisen erfordern Claude Code v2.1.219 oder später. Vor v2.1.219 ließ die Antwort `fast_mode_state` weg, wenn Fast-Modus nicht verfügbar war, und trug niemals einen Grund. Für die Grund-Codes und ihre Bedeutungen siehe [`fast_mode_disabled_reason`](#sdkresultmessage) auf der Ergebnis-Nachricht.

Der Steuer-Antwort-Wrapper für eine erfolgreiche `initialize` trägt auch ein `pending_permission_requests`-Array. Das Feld ist auf dem Antwort-Wrapper selbst, nicht in der `SDKControlInitializeResponse`-Nutzlast oben. Jeder Eintrag ist eine vollständige `control_request`-Nachricht mit der gleichen `{ type: "control_request", request_id, request }`-Form, die die Sitzung für Berechtigungsanfragen während der Ausführung streamt.

Das Array listet die Berechtigungsanfragen auf, die dieser Claude Code-Prozess ausgegeben hat und noch nicht aufgelöst hat. Das SDK liest das Array für Sie und versendet jeden Eintrag an Ihren [`canUseTool`](#canusetool)-Callback, die gleiche Neulieferung, die [`reinitialize()`](#query-object) nach einer Transportlücke auslöst. Behandeln Sie wiederholte Anfrage-IDs idempotent, da ein Eintrag eine Anfrage wiederholen kann, die der Callback bereits erhalten hat, bevor die Verbindung unterbrochen wurde.

Das Array ist immer auf einer erfolgreichen `initialize`-Antwort vorhanden und ist leer, wenn dieser Prozess keine ungelöste Berechtigungsanfrage hat. Erfordert Claude Code v2.1.268 oder später. Frühere Versionen könnten das Feld auslassen, daher behandeln Sie ein fehlendes Feld als ältere CLI, wenn Sie das Drahtprotokoll selbst analysieren, anstatt als Beweis, dass nichts ausstehend ist.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Die Unterbrechnungsbestätigung: der Wert, mit dem [`interrupt()`](#query-object) auf einer CLI aufgelöst wird, die die `interrupt_receipt_v1`-Fähigkeit in [`SDKSystemMessage.capabilities`](#sdksystemmessage) ankündigt. Erfordert Claude Code v2.1.205 oder später. Frühere CLIs beantworten die Unterbrechung mit einer leeren Erfolgsnutzlast, daher wird `interrupt()` mit `undefined` aufgelöst.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` listet die UUIDs der Benutzernachrichten auf, die ausstanden, als die Unterbrechung ankam: Nachrichten, die noch in der Warteschlange sind, plus alle Nachrichten, die Claude Code bereits aus der Warteschlange für die nächste Umdrehung genommen hatte. Sobald die erste Umdrehung der Sitzung gestartet hat, verarbeitet Claude Code die aufgelisteten Nachrichten nach der Unterbrechung, es sei denn, Sie stornieren sie zuerst, und kann mehrere in eine Umdrehung zusammenführen. Wenn Sie vor dem Start der ersten Umdrehung unterbrechen, bricht Claude Code diese Umdrehung ab, sobald sie startet, und die aufgelisteten Nachrichten in dieser Umdrehung erhalten keine Antwort.

Verwenden Sie die Bestätigung, um zu entscheiden, ob Sie etwas erneut senden möchten. Eine aufgelistete Nachricht, die Sie nicht stornieren, tritt in das Gespräch ein, unabhängig davon, ob sie eine Antwort erhält, daher wird das erneute Senden an Claude zweimal gesendet.

Interpretieren Sie die Liste mit diesen Vorbehalten:

* Nur Nachrichten, die mit einer UUID in die Warteschlange eingereiht wurden, erscheinen. Ein leeres Array bedeutet nicht, dass nichts anderes läuft.
* Nur Hauptthread-Nachrichten werden aufgelistet. Nachrichten, die an einen Subagenten adressiert sind, sind außerhalb des Geltungsbereichs.
* Die Liste kann UUIDs enthalten, die Ihr Client nie gesendet hat, z. B. [geplante Task](/docs/de/scheduled-tasks)-Trigger. Ignorieren Sie UUIDs, die Sie nicht erkennen, anstatt sie als Fehler zu behandeln.

Ein Client, der das Steuerprotokoll der CLI direkt antreibt, anstatt über `interrupt()`, kann `cancel_queued: true` auf der `interrupt`-Steueranfrage setzen. Claude Code v2.1.219 und später kündigt Unterstützung mit der `interrupt_cancel_queued_v1`-Fähigkeit in [`SDKSystemMessage.capabilities`](#sdksystemmessage) an; ältere CLIs ignorieren das Feld und lassen Warteschlangen-Nachrichten wie gewohnt laufen. Eine solche Unterbrechung storniert auch jede Nachricht, die sonst unter `still_queued` aufgelistet würde: die Bestätigung listet sie stattdessen unter `cancelled` auf, `still_queued` ist leer, und keine von ihnen läuft.

Die `cancelled`-Liste trägt die gleichen Vorbehalte wie `still_queued`. Die `interrupt()`-Methode sendet niemals `cancel_queued`, daher tragen Bestätigungen, mit denen sie aufgelöst wird, nicht `cancelled`.

Die Bestätigung ist eine Momentaufnahme, die zum Zeitpunkt der Verarbeitung der Unterbrechung aufgenommen wird, und bei einer sauberen Unterbrechung kommt sie vor der [`SDKResultMessage`](#sdkresultmessage) der unterbrochenen Umdrehung an. Lesen Sie die Bestätigung, anstatt die Warteschlange nach diesem Ergebnis zu überprüfen: die Schleife startet die nächste Warteschlangen-Umdrehung sofort, daher hat sich die Warteschlange, die Sie nach dem Ergebnis überprüfen, bereits geändert.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

Rückgabetyp von [`getContextUsage()`](#query-object). Mit dem Standard `detail` ist dies die gleiche Nutzlast, die Claude Code für den `/context`-Befehl in einer interaktiven Sitzung rendert, daher trägt sie neben den Token-Zählungen Anzeigefelder wie `color` und `gridRows`, die Claude Code verwendet, um das `/context`-Nutzungsgitter zu zeichnen.

Das optionale `detail`-Argument der Methode wählt, wie Claude Code jede Kategorie zählt. Mit dem Standard `'full'` zählt Claude Code jede Kategorie mit Token-Zähl-API-Anfragen. Übergeben Sie `{ detail: 'summary' }`, um eine Antwort aus der letzten Antwort-Nutzung und lokalen Schätzungen zu erhalten. Keine Token-Zähl-Anfragen gehen raus, und die Pro-Kategorie-Zahlen sind ungefähr. Das `detail`-Argument erfordert Agent SDK v0.3.257 oder später.

Wenn Sie `/context` als Eingabeaufforderung anstelle des Aufrufs der Methode senden, fügt Claude Code eine [`SDKContextUsage`](#sdkcontextusage)-Nutzlast an das `context_usage`-Feld der Assistent-Nachricht an, die das Ergebnis liefert. Dieses Feld erfordert Agent SDK v0.3.232 oder später.

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

Lesen Sie Token-Zuordnung aus den Sammlungsfeldern:

* `categories` enthält die Pro-Kategorie-Summen.
* `mcpTools` und `agents` ordnen Token einzelnen MCP-Tools und Subagenten zu.
* `memoryFiles` listet jede geladene Speicherdatei mit ihren Kosten auf.
* `skills.skillFrontmatter` ordnet die Token der Fähigkeitsliste jeder eingeschlossenen Fähigkeit zu. Die Pro-Fähigkeit-Zählungen messen jeden Fähigkeitslisten-Eintrag, wie Claude Code ihn tatsächlich sendet, was kürzer als die vollständige Frontmatter der Fähigkeit sein kann. Vergleichen Sie `skills.totalSkills` mit `skills.includedSkills`, um zu sehen, ob jede entdeckte Fähigkeit in die Liste aufgenommen wurde.

`totalTokens` ist die aktuelle Kontextnutzung der Sitzung, und `maxTokens` ist das Fenster, gegen das die Nutzung gemessen wird. Dieses Fenster ist das Kontextfenster des Modells oder das niedrigere Auto-Komprimierungs-Fenster, wenn eines zutrifft. `rawMaxTokens` trägt den gleichen Wert wie `maxTokens`, und `percentage` ist `totalTokens` als gerundeter Prozentsatz dieses Fensters.

Claude Code lässt die optionalen `deferredBuiltinTools`-, `systemTools`- und `systemPromptSections`-Diagnosen ungesetzt, daher erwarten Sie, dass sie abwesend sind, auch wenn der Typ sie deklariert.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

Rückgabetyp von [`readFile()`](#query-object).

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` enthält den Dateitext oder Base64-Daten, wenn Sie `encoding: 'base64'` angefordert haben; das `encoding`-Feld der Antwort wird in diesem Fall auf `'base64'` gesetzt. `absPath` ist der aufgelöste absolute Pfad. `truncated` wird gesetzt, wenn die Datei länger als die `maxBytes`-Obergrenze war und der Inhalt bei diesem Limit gekürzt wurde.

<h4 id="what-readfile-can-read">
  Was `readFile()` lesen kann
</h4>

`readFile()` bedient einen engeren Satz von Dateien als das Read-Tool:

* Eine reguläre Datei in einem der Arbeitsverzeichnisse der Sitzung, z. B. `cwd` und `additionalDirectories`
* Ein paar von Claude Codes eigenen Dateien für die Sitzung, z. B. Tool-Ergebnisse

`Read`-Deny- und Ask-Regeln blockieren immer noch einen übereinstimmenden Pfad, und eine breite `Read`-Allow-Regel öffnet nicht den Rest des Dateisystems für `readFile()`. Für alles andere wird der Aufruf mit `null` aufgelöst.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

Rückgabetyp von [`reloadSkills()`](#query-object).

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` listet die nach dem Neuladen verfügbaren Fähigkeiten in der gleichen [`SlashCommand`](#slashcommand)-Form auf, die `supportedCommands()` zurückgibt.

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

Rückgabetyp von [`readMcpResource()`](#query-object), der das `resources/read`-Ergebnis des MCP-Servers trägt. Erfordert TypeScript Agent SDK v0.3.280 oder später.

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

Übergeben Sie `readMcpResource()` den Server-Namen, wie `mcpServerStatus()` ihn meldet, und eine `ui://`-URI, z. B. die `ui.resourceUri`, die ein Tool in seinen [`_meta`](#mcpserverstatus) deklariert. Der Aufruf lehnt für jedes andere URI-Schema ab, für einen [SDK MCP-Server](#createsdkmcpserver), den Ihre Anwendung selbst hostet, und für einen Server, der nicht verbunden ist. Es ist verfügbar, wenn die Init-Nachricht [`capabilities`](#sdksystemmessage) `mcp_read_resource_v1` enthalten.

Jeder `contents`-Eintrag ist ein Inhalts-Element, wie der Server es gesendet hat. `blob` enthält Base64-Daten für ein binäres Element, und `_meta` ist das `_meta` des Elements selbst, wo ein MCP Apps-Server die `ui.csp` und `ui.permissions` der Ressource ablegt. Der Inhalt ist nicht vertrauenswürdiges HTML von Drittanbietern, daher rendern Sie ihn in einer Sandbox.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Konfiguration für einen programmgesteuert definierten Subagenten.

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

| Feld                                  | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------ | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Ja           | Natürlichsprachige Beschreibung, wann dieser Agent verwendet werden soll                                                                                                                                                                                                                                                                                                                       |
| `tools`                               | Nein         | Array von zulässigen Tool-Namen. Wenn weggelassen, erbt jedes [Tool, das Subagenten zur Verfügung steht](/docs/de/sub-agents#available-tools). Um Fähigkeiten in den Kontext des Agenten vorzuladen, verwenden Sie das `skills`-Feld, anstatt `'Skill'` hier aufzulisten                                                                                                                            |
| `disallowedTools`                     | Nein         | Array von Tool-Namen, die für diesen Agenten explizit nicht zulässig sind. MCP-Server-Level-Muster werden auch akzeptiert: `mcp__server` oder `mcp__server__*` entfernt jedes Tool von diesem Server, und `mcp__*` entfernt jedes MCP-Tool von jedem Server                                                                                                                                    |
| `prompt`                              | Ja           | Der Systemprompt des Agenten                                                                                                                                                                                                                                                                                                                                                                   |
| `model`                               | Nein         | Modellüberschreibung für diesen Agenten. Akzeptiert einen Alias wie `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, oder eine vollständige Modell-ID. `'inherit'` verwendet das Hauptmodell. Wenn Sie es weglassen, wählt Claude Code das Modell in der [Subagenten-Modellreihenfolge](/docs/de/sub-agents#choose-a-model)                                                                 |
| `mcpServers`                          | Nein         | MCP-Server-Spezifikationen für diesen Agenten                                                                                                                                                                                                                                                                                                                                                  |
| `skills`                              | Nein         | Array von Fähigkeitsnamen, die in den Agenten-Kontext vorgeladen werden sollen                                                                                                                                                                                                                                                                                                                 |
| `initialPrompt`                       | Nein         | Automatisch eingereicht als die erste Benutzer-Umdrehung, wenn dieser Agent als Hauptthread-Agent läuft                                                                                                                                                                                                                                                                                        |
| `maxTurns`                            | Nein         | Maximale Anzahl von agentengesteuerten Umdrehungen (API-Roundtrips), bevor gestoppt wird                                                                                                                                                                                                                                                                                                       |
| `background`                          | Nein         | Führen Sie diesen Agenten als nicht-blockierende Hintergrund-Task aus, wenn aufgerufen                                                                                                                                                                                                                                                                                                         |
| `omitClaudeMd`                        | Nein         | Führen Sie diesen Agenten ohne die Benutzer-, Projekt- und lokalen CLAUDE.md-Dateien aus, wenn er als Subagent läuft; verwaltete Richtliniendateien werden immer noch geladen. Verwenden Sie es für Agenten, die alles, was sie brauchen, aus dem Agent-Tool-Prompt nehmen. Wird ignoriert, wenn dieser Agent als Hauptthread-Agent läuft. Erfordert TypeScript Agent SDK v0.3.271 oder später |
| `memory`                              | Nein         | Speicherquelle für diesen Agenten: `'user'`, `'project'` oder `'local'`                                                                                                                                                                                                                                                                                                                        |
| `effort`                              | Nein         | Reasoning-Anstrengungsstufe für diesen Agenten. Akzeptiert eine benannte Stufe oder eine Ganzzahl                                                                                                                                                                                                                                                                                              |
| `permissionMode`                      | Nein         | Berechtigungsmodus für die Tool-Ausführung innerhalb dieses Agenten. Die [Subagenten-Vererbungsregeln](/docs/de/agent-sdk/permissions#available-modes) entscheiden, wann er zutrifft. Siehe [`PermissionMode`](#permissionmode)                                                                                                                                                                     |
| `criticalSystemReminder_EXPERIMENTAL` | Nein         | Experimentell: Kritische Erinnerung, die zum Systemprompt hinzugefügt wird                                                                                                                                                                                                                                                                                                                     |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Gibt MCP-Server an, die einem Subagenten zur Verfügung stehen. Kann ein Server-Name (Zeichenkette, die auf einen Server aus der `mcpServers`-Konfiguration des übergeordneten Elements verweist) oder eine Inline-Server-Konfiguration sein, die Server-Namen auf Konfigurationen abbildet.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Wobei `McpServerConfigForProcessTransport` `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig` ist.

<h3 id="settingsource">
  `SettingSource`
</h3>

Steuert, welche dateisystem-basierte Konfigurationsquellen das SDK Einstellungen aus lädt.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Wert        | Beschreibung                                                                                 | Ort                           |
| :---------- | :------------------------------------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Globale Benutzereinstellungen                                                                | `~/.claude/settings.json`     |
| `'project'` | Gemeinsame Projekteinstellungen (versionskontrolliert)                                       | `.claude/settings.json`       |
| `'local'`   | Lokale Projekteinstellungen, gitignoriert, wenn Claude Code eine Einstellung darin speichert | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Standardverhalten
</h4>

Wenn `settingSources` weggelassen oder `undefined` ist, lädt `query()` die gleichen Dateisystem-Einstellungen wie die Claude Code CLI: Benutzer, Projekt und lokal. Siehe [Was settingSources nicht steuert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control) für Eingaben, die unabhängig von dieser Option gelesen werden, und wie man sie deaktiviert.

<h4 id="why-use-settingsources">
  Warum settingSources verwenden
</h4>

**Dateisystem-Einstellungen deaktivieren:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Do not load user, project, or local settings from disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Nur bestimmte Einstellungsquellen laden:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Load only project settings, ignore user and local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Only .claude/settings.json
  }
});
```

Um CLAUDE.md-Projektanweisungen zu laden, beziehen Sie `"project"` in `settingSources` ein. Siehe [Systemprompts ändern](/docs/de/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions), um zu sehen, wie CLAUDE.md-Laden mit den Systemprompt-Optionen interagiert.

<h4 id="settings-precedence">
  Einstellungs-Rangfolge
</h4>

Wenn mehrere Quellen geladen werden, werden Einstellungen mit dieser Rangfolge zusammengeführt (höchste zu niedrigste):

1. Lokale Einstellungen (`.claude/settings.local.json`)
2. Projekteinstellungen (`.claude/settings.json`)
3. Benutzereinstellungen (`~/.claude/settings.json`)

Programmatische Optionen wie `agents`, `allowedTools` und `settings` überschreiben Benutzer-, Projekt- und lokale Dateisystem-Einstellungen. Verwaltete Richtlinieneinstellungen haben Vorrang vor programmatischen Optionen.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Standard permission behavior
  | "acceptEdits" // Auto-accept file edits
  | "bypassPermissions" // Bypass permission checks; explicit ask rules still prompt
  | "plan" // Planning mode - explore without editing
  | "dontAsk" // Don't prompt for permissions, deny if not pre-approved
  | "auto"; // Model classifier approves or denies permission prompts
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Benutzerdefinierter Berechtigungsfunktionstyp zur Steuerung der Tool-Nutzung.

Die Funktion ist der SDK-Ersatz für die interaktive Berechtigungsaufforderung: Sie wird nur aufgerufen, wenn der [Berechtigungsbewertungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Eingabeaufforderung führt. Tool-Aufrufe, die bereits von einem `allowedTools`-Eintrag, einer Einstellungs-Allow-Regel oder dem Berechtigungsmodus wie `acceptEdits` oder `bypassPermissions` genehmigt wurden, rufen sie niemals auf. Um jeden Tool-Aufruf zu gating, verwenden Sie stattdessen einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks).

Eine Allow-Regel genehmigt nicht vorab die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves); siehe [Wie Berechtigungen bewertet werden](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated), um zu sehen, welche von ihnen den Callback erreichen und was in `dontAsk`- und `auto`-Modus passiert.

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

| Option           | Typ                                         | Beschreibung                                                                                                                                                                                                                                                                                                                                                                 |
| :--------------- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Signalisiert, wenn die Operation abgebrochen werden soll                                                                                                                                                                                                                                                                                                                     |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Vorgeschlagene Berechtigungsaktualisierungen, damit der Benutzer nicht erneut für dieses Tool aufgefordert wird. Bash-Eingabeaufforderungen enthalten einen Vorschlag mit dem `localSettings`-[Ziel](#permissionupdatedestination), daher schreibt das Zurückgeben in `updatedPermissions` die Regel in `.claude/settings.local.json` und persistiert über Sitzungen hinweg. |
| `blockedPath`    | `string`                                    | Der Dateipfad, der die Berechtigungsanfrage ausgelöst hat, falls zutreffend                                                                                                                                                                                                                                                                                                  |
| `mcpServer`      | `{ name: string; source: string }`          | Für ein `mcp__*`-Tool, der MCP-Server, der es bedient, und woher diese Server-Definition kam, mit den Feldern von [`McpServerProvenance`](#mcpserverprovenance). Abwesend für andere Tools. Erfordert Agent SDK v0.3.274 oder später                                                                                                                                         |
| `decisionReason` | `string`                                    | Erklärt, warum diese Berechtigungsanfrage ausgelöst wurde                                                                                                                                                                                                                                                                                                                    |
| `toolUseID`      | `string`                                    | Eindeutige Kennung für diesen spezifischen Tool-Aufruf innerhalb der Assistent-Nachricht                                                                                                                                                                                                                                                                                     |
| `agentID`        | `string`                                    | Wenn innerhalb eines Sub-Agenten läuft, die ID des Sub-Agenten                                                                                                                                                                                                                                                                                                               |
| `requestId`      | `string`                                    | Die `request_id` des `control_request`-Umschlags. Eine `control_response`, die Ihre Anwendung außerhalb des SDK sendet, z. B. ein signierter HTTP POST, muss diesen Wert widerspiegeln, damit der Claude Code-Prozess die Antwort mit der Anfrage abgleichen kann                                                                                                            |

Der Callback löst die Anfrage normalerweise auf, indem er ein [`PermissionResult`](#permissionresult) zurückgibt, das das SDK über seinen Transport als `control_response` zurückschreibt. Geben Sie `null` nur zurück, wenn Ihre Anwendung die `control_response` für diese Anfrage bereits über ihren eigenen Kanal gesendet hat, wobei `requestId` widergespiegelt wird; das SDK überspringt dann das Schreiben der Antwort zu seinem Transport. Das Zurückgeben von `null` in jedem anderen Fall lässt den Tool-Aufruf unbegrenzt blockiert, da keine `control_response` jemals gesendet wird und Berechtigungsaufforderungen nicht zeitlich begrenzt sind.

Die `requestId`-Option und der `null`-Rückgabewert erfordern Claude Code v2.1.199 oder später.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Ergebnis einer Berechtigungsprüfung.

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

Konfiguration für das Verhalten integrierter Tools.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Feld                            | Typ                    | Beschreibung                                                                                                                                                                             |
| :------------------------------ | :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Aktiviert das `preview`-Feld auf [`AskUserQuestion`](/docs/de/agent-sdk/user-input#question-format)-Optionen und setzt sein Inhaltsformat. Wenn nicht gesetzt, gibt Claude keine Vorschau aus |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Konfiguration für MCP-Server.

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

Konfiguration zum Laden von Plugins im SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Feld               | Typ       | Beschreibung                                                                                                                                                                                                                       |
| :----------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Muss `'local'` sein (derzeit werden nur lokale Plugins unterstützt)                                                                                                                                                                |
| `path`             | `string`  | Absoluter oder relativer Pfad zum Plugin-Verzeichnis                                                                                                                                                                               |
| `skipMcpDiscovery` | `boolean` | Wenn `true`, lädt das SDK Fähigkeiten, Hooks, Agenten und Befehle aus diesem Plugin, liest aber nicht seine `.mcp.json` oder Manifest `mcpServers`. Setzen Sie dies, wenn Ihre Anwendung die MCP-Verbindungen des Plugins besitzt. |

**Beispiel:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Für vollständige Informationen zum Erstellen und Verwenden von Plugins siehe [Plugins](/docs/de/agent-sdk/plugins).

<h2 id="message-types">
  Nachrichtentypen
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

Union-Typ aller möglichen Nachrichten, die von der Abfrage zurückgegeben werden.

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

Assistenten-Antwortnachricht.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Aus Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Das `message`-Feld ist eine [`BetaMessage`](https://platform.claude.com/docs/de/api/messages/create) aus dem Anthropic SDK. Es enthält Felder wie `id`, `content`, `model`, `stop_reason` und `usage`.

`SDKAssistantMessageError` ist einer von: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'` oder `'unknown'`. Vier dieser Werte bedeuten mehr als ihre Namen aussagen:

* `'model_not_found'`: Das ausgewählte Modell existiert nicht oder ist nicht für Ihr Konto oder Ihre Bereitstellung verfügbar
* `'overloaded'`: Die API hat einen 529-Fehler zurückgegeben, weil der Server ausgelastet ist, im Gegensatz zu `'rate_limit'`, das ein 429-Fehler gegen Ihr Kontingent ist
* `'account_on_hold'`: [Ihr Konto ist gesperrt](/docs/de/errors#your-account-is-on-hold)
* `'cloud_credential_error'`: Claude Code konnte keine verwendbaren AWS- oder Google Cloud-Anmeldedaten auf der Maschine, auf der es läuft, abrufen, daher erreichte keine Anfrage den Cloud-Anbieter. Die übliche Ursache ist eine Cloud-Anmeldung, die abgelaufen ist oder auf dieser Maschine nie abgeschlossen wurde, obwohl ein kurzzeitig unerreichbarer Anmeldedatendienst denselben Wert meldet. Siehe [AWS- oder Google Cloud-Anmeldedaten konnten nicht geladen werden](/docs/de/errors#could-not-load-aws-or-google-cloud-credentials). Erfordert TypeScript Agent SDK v0.3.267 oder später, das Claude Code v2.1.267 bündelt

`aborted` ist `true`, wenn ein Interrupt oder Abbruch die Assistenten-Nachricht vor Abschluss des Streams abgeschnitten hat: Die Nachricht hat keinen `stop_reason` und der Inhalt kann mitten im Wort enden. Das Feld fehlt bei normal abgeschlossenen Nachrichten. Es erfordert Agent SDK v0.3.214 oder später.

Claude Code setzt `user_message_uuid` und `user_message_uuids` auf die erste Assistenten-Nachricht des Turns unter den Bedingungen in [`user_message_uuid`](#user_message_uuid).

`timestamp` ist die ISO-8601-Zeit, wenn die Generierung des Nachrichteninhalts auf dem Prozess, der ihn erzeugt hat, beendet wurde. Der Wert stammt von der Uhr dieser Maschine, daher verwenden Sie ihn nur zur Anzeige und ordnen Sie Nachrichten nicht danach. Ein API-Turn kann mehrere Assistenten-Nachrichten erzeugen, die eine `message.id` teilen, jede mit ihrem eigenen `timestamp`. Wenn das Feld fehlt, greifen Sie auf die Zeit zurück, zu der Sie die Nachricht erhalten haben.

`context_usage` ist eine strukturierte Kopie des `/context`-Berichts, typisiert als [`SDKContextUsage`](#sdkcontextusage), und erfordert Agent SDK v0.3.232 oder später. Wenn Sie `/context` als Eingabeaufforderung senden, liefert Claude Code den Bericht als Assistenten-Nachricht, deren `message.content` die Markdown-Tabelle enthält, und fügt `context_usage` an dieselbe Nachricht an. Claude Code setzt das Feld nicht auf andere Assistenten-Nachrichten, und frühere Versionen liefern die `/context`-Tabelle ohne es, daher lesen Sie die Aufschlüsselung aus dem Feld, wenn es vorhanden ist, und greifen Sie auf den Markdown-Text zurück, wenn es nicht vorhanden ist.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Benutzer-Eingabenachricht.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Aus Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

Setzen Sie `pasted_content`, um Inhalte zu senden, die der Benutzer in Ihre Eingabeaufforderungs-Benutzeroberfläche eingefügt hat, anstatt sie einzugeben, einen Eintrag pro Einfügung, jeder ein String oder ein Array von Inhaltsblöcken. Claude Code fügt den Text jedes Eintrags nach dem eingegebenen Text an, in Reihenfolge, und kann jede Einfügung in `<pasted_content>`-Tags einwickeln. Blöcke außer Text werden ignoriert, daher senden Sie Bilder und Dokumente in `message.content`. Erfordert Agent SDK v0.3.277 oder später.

Setzen Sie `shouldQuery` auf `false`, um die Nachricht zum Transkript hinzuzufügen, ohne einen Assistenten-Turn auszulösen. Die Nachricht wird gehalten und in die nächste Benutzer-Nachricht zusammengeführt, die einen Turn auslöst. Verwenden Sie dies, um Kontext einzufügen, z. B. die Ausgabe eines Befehls, den Sie außerhalb des Bands ausgeführt haben, ohne einen Modell-Aufruf dafür auszugeben.

Auf einer Nachricht, die einen `tool_result`-Block trägt, ist `tool_use_result` das strukturierte Ausgabeobjekt des Tools und nicht der Text, der an das Modell gesendet wird. Seine Form hängt vom Tool ab, das durch den entsprechenden `tool_use`-Block benannt wird, daher ist das Feld als `unknown` typisiert; die integrierten Formen sind unter [Tool-Ausgabetypen](#tool-output-types) aufgelistet.

Für das `Agent`-Tool ist `tool_use_result` [`AgentOutput`](#agent-2). Bei einem `completed`-Ergebnis enthält `content` den Bericht des Subagenten ohne die Agent-ID und den Nutzungs-Trailer, den Claude Code an den `tool_result`-Text anhängt, daher rendern Sie stattdessen aus `tool_use_result`.

Für ein MCP-Tool, dessen Ergebnis `resource_link`-Blöcke enthält, ist `tool_use_result` ein Objekt mit einem `resourceLinks`-Array von [`SDKMcpResourceLink`](#sdkmcpresourcelink)-Einträgen. Claude empfängt jeden Link als Textzeile im `tool_result`-Block, daher lesen Sie `resourceLinks`, um die vom Server zurückgegebenen Dateien zu rendern, anstatt diesen Text zu analysieren. Claude Code lässt `resourceLinks` weg, wenn das Ergebnis keine Links hat und bei Ergebnissen von Subagenten, behält höchstens 50 Links pro Ergebnis und stoppt das Hinzufügen von Links, sobald das Array 64 KiB serialisiertes JSON erreicht. `resourceLinks` erfordert Agent SDK v0.3.257 oder später.

Setzen Sie `inline_pastes`, um Claude Code mitzuteilen, welche Teile von `message.content` der Benutzer eingefügt hat, anstatt sie einzugeben, einen String pro Einfügung. Der Eingabetext bleibt dort, wo der Benutzer ihn platziert hat. Claude Code kann jede aufgelistete Einfügung in `<pasted_content>`-Tags einwickeln, wo sie steht, damit Claude eingefügte Materialien von den eigenen Worten des Benutzers unterscheiden kann. Nur Einfügungen im letzten Textblock der Eingabeaufforderung werden eingewickelt. Erfordert TypeScript Agent SDK v0.3.280 oder später.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

Wiedergegebene Benutzer-Nachricht mit erforderlicher UUID.

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

Ein Benutzer-Turn, der von außerhalb der Sitzung eingefügt wird, dessen [`origin`](#sdkmessageorigin)-Art `peer` oder `channel` ist, erreicht den Stream als Wiedergabe, unabhängig davon, ob er während eines aktiven Turns geliefert wurde oder einen neuen Turn gestartet hat, während die Sitzung untätig war. Vor v2.1.207 erzeugte ein eingefügter Turn, der geliefert wurde, während die Sitzung untätig war, keine Nachricht im Stream und erschien nur, wenn Sie das Transkript erneut lasen.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

Endgültige Ergebnis-Nachricht.

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

Mehrere Felder im Ergebnis enthalten diagnostische Details über `subtype` hinaus:

* `api_error_status`: Der HTTP-Statuscode des API-Fehlers, der die Konversation beendet hat. Fehlt oder ist `null`, wenn der Turn ohne API-Fehler endete.
* `ttft_ms`: Zeit bis zum ersten Token in Millisekunden, gemessen, wenn die erste vollständige Assistenten-Nachricht ankommt. Nur auf dem Success-Arm vorhanden.
* `ttft_stream_ms`: Zeit in Millisekunden bis zum ersten `message_start`-Stream-Ereignis, wenn der Response-Stream öffnet. Niedriger als `ttft_ms`; die Lücke zwischen den beiden ist die Zeit, die zum Streamen der ersten Nachricht benötigt wird. Nur auf dem Success-Arm vorhanden.
* `user_message_uuid`: Die `uuid` der Nachricht, die Sie gesendet haben und diesen Turn beantwortet hat. Siehe [`user_message_uuid`](#user_message_uuid) für welche Ergebnisse sie tragen.
* `user_message_uuids`: Die `uuid`s jeder Nachricht, die Sie gesendet haben und die Claude Code in diesem Turn beantwortet hat. Siehe [`user_message_uuids`](#user_message_uuids).
* `request_sent_wall_ms`: Epoch-Millisekunden, zu denen Claude Code die API-Anfrage versendet hat, für Joins gegen Server-seitige Zeitstempel. Nur zusammen mit [`user_message_uuid`](#user_message_uuid) vorhanden, auf einem Success-Ergebnis mit `is_error` false, dessen Turn eine API-Anfrage versendet hat.
*

`first_content_frame_ms`: Zeit in Millisekunden bis zum ersten `content_block_start`- oder `content_block_delta`-Stream-Ereignis, wobei Thinking-Blöcke als Inhalt gezählt werden. Nur auf dem Success-Arm vorhanden, wenn `is_error` false ist. Erfordert Agent SDK v0.3.260 oder später.

*

`first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms`: Zeitangaben zum Hochladen des ersten Stream-Ereignisses des Turns. Claude Code zeichnet sie nur in Sitzungen auf, die es zu claude.ai streamt, wie [Cloud-Sitzungen](/docs/de/claude-code-on-the-web), und die Ergebnisse, die `query()` liefert, enthalten sie nicht. Erfordert Agent SDK v0.3.260 oder später.

* `usage`: Nur Hauptagenten-Schleife. Schließt Subagenten- und Hilfmodell-Aufrufe aus und ist pro Turn in Streaming-Input-Sitzungen. Bevorzugen Sie `modelUsage` für Token-/Kostenabrechnung.
* `modelUsage`: Pro-Modell-Summen für jeden Modell-Aufruf, der während dieses `query()`-Aufrufs durch die Abfrage-Pipeline gemacht wurde, einschließlich der Hauptschleife, Subagenten und interner Aufrufe wie Komprimierung und Workflow-Agenten. Hilfaufrufe außerhalb dieser Pipeline, wie der Berechtigungsklassifizierer und Token-Zählungsanfragen, sind ausgeschlossen. Ein Aufruf, der eine Sitzung fortgesetzt, zählt auch die [Pro-Modell-Summen, die aus den früheren Aufrufen der Sitzung wiederhergestellt wurden](/docs/de/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). In Streaming-Input-Sitzungen sind die Summen kumulativ über Turns, daher lesen Sie das neueste Ergebnis, anstatt über Ergebnisse zu summieren. Siehe [Kosten im Streaming-Input-Modus verfolgen](/docs/de/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) für Zurückstellungen und [Summen nach einem Sitzungsabsturz wiederherstellen](/docs/de/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) für zurückgesetzte Ergebnisse.
* `total_cost_usd`: Kumulativer geschätzter Kosten in USD, abdeckend die gleichen Aufrufe wie `modelUsage` und Zurückstellung an den gleichen Punkten. Ein Aufruf, der eine Sitzung fortgesetzt, zählt auch die [Summen, die aus den früheren Aufrufen der Sitzung wiederhergestellt wurden](/docs/de/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Es ist eine Schätzung, keine Abrechnungsaussage. Siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking) für Genauigkeitsvorbehalt.
* `queued_turn_count`: Die Anzahl der Nachrichten, die Sie mit `origin: { kind: "human" }` gesendet haben und die noch warten, wenn Claude Code das Ergebnis erzeugt hat. Siehe [`queued_turn_count`](#queued_turn_count) für was `0` und ein fehlendes Feld Ihnen sagen.
*

`startup_failure_reason`: Warum Claude Code sich weigerte zu starten, auf dem `error_during_execution`-Ergebnis, das es vor dem Beenden bei einem bekannten Startfehler schreibt. Siehe [`startup_failure_reason`](#startup_failure_reason) für die Werte und welche Fehler es tragen. Erfordert Agent SDK v0.3.274 oder später.

* `terminal_reason`: Warum die Schleife endete. Einer von `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"` oder `"turn_setup_failed"`.
* `fast_mode_state`: Einer von `"on"`, `"off"` oder `"cooldown"`.
* `fast_mode_disabled_reason`: Warum [Fast Mode](/docs/de/fast-mode) gerade nicht verfügbar ist. Fehlt, wenn nichts Fast Mode blockiert, obwohl eine Anfrage möglicherweise immer noch mit Standardgeschwindigkeit ausgeführt wird. Während der Abkühlung nach einem Fast Mode Rate Limit meldet Claude Code `fast_mode_state: "cooldown"` ohne Grund-Code und reaktiviert Fast Mode, wenn die Abkühlung abläuft. Erfordert Claude Code v2.1.219 oder später.

Verwenden Sie den Grund-Code, um zu erklären, warum Fast Mode in Ihrer eigenen Benutzeroberfläche aus ist, anstatt die Verfügbarkeit neu abzuleiten. Jeder Code benennt die Überprüfung, die Fast Mode blockiert hat:

| Grund-Code             | Bedeutung                                                                                                                                                                       |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | Das Konto hat nicht das bezahlte Abonnement oder die Nutzungsguthaben, die Fast Mode erfordert                                                                                  |
| `preference`           | Die Organisation hat Fast Mode deaktiviert                                                                                                                                      |
| `extra_usage_disabled` | Nutzungsguthaben sind für das Konto ausgeschaltet                                                                                                                               |
| `network_error`        | Die [Verfügbarkeitsprüfung](/docs/de/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) konnte `api.anthropic.com` nicht erreichen                                             |
| `unknown`              | Claude Code konnte die Verfügbarkeit nicht bestimmen                                                                                                                            |
| `not_first_party`      | Die Sitzung verwendet einen anderen Anbieter als die Anthropic API                                                                                                              |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/de/env-vars) ist gesetzt                                                                                                                     |
| `model_not_allowed`    | Das Fast Mode Opus-Modell ist nicht in der [`availableModels`](/docs/de/model-config#restrict-model-selection)-Zulassungsliste der Organisation                                      |
| `sdk_opt_in_required`  | Die Sitzung hat sich nicht für Fast Mode angemeldet: Übergeben Sie `fastMode: true` in der [`settings`](#options)-Option oder durch [`applyFlagSettings()`](#applyflagsettings) |
| `pending`              | Die Verfügbarkeitsprüfung ist noch nicht abgeschlossen                                                                                                                          |

Das gleiche Feldpaar erscheint auf [`SDKSystemMessage`](#sdksystemmessage) und auf [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse), daher können Sie den Fast Mode-Status vor dem ersten Turn lesen.

Das `origin`-Feld leitet die [`SDKMessageOrigin`](#sdkmessageorigin) der Benutzer-Nachricht weiter, die dieses Ergebnis ausgelöst hat. Wenn das SDK einen synthetischen Follow-up-Turn einfügt, wie für eine beendete Hintergrund-Aufgabe, trägt die resultierende `SDKResultMessage` `origin: { kind: "task-notification" }`. Routinen, deren Trigger ausgelöst wurde, und von Servern verifizierte Nachrichten von Ihren anderen Sitzungen kommen mit dieser Art an, jede mit dem `subkind`, der in [Task-Notification-Subkinds](#task-notification-subkinds) beschrieben ist. Überprüfen Sie `kind`, um Ergebnisse zu unterscheiden, die Ihre Eingabeaufforderung beantworten, von eingefügten Follow-ups, bevor Sie sie weiterleiten oder unterdrücken. Wenn Ihre Anwendung [geplante Ausführungen erklärt](#declare-a-scheduled-run), tragen ihre Ergebnisse auch `kind: "task-notification"`, daher unterdrücken Sie nicht nur auf `kind` allein.

Wenn mehrere Hintergrund-Task-Abschlüsse zusammen in die Warteschlange eingereiht werden, kann Claude Code sie in einem Turn beantworten, anstatt einen Turn pro Abschluss. Jeder Abschluss erzeugt immer noch sein eigenes Ergebnis mit dieser Herkunft. Alle außer dem letzten der Abschlüsse, die Claude Code zusammen beantwortet, erzeugen leere Ergebnisse mit `num_turns: 0`, in Reihenfolge, und das Ergebnis des letzten trägt den Turn, der sie alle beantwortet.

Das Feld fehlt bei Ergebnissen, die vor einem Benutzer-Turn ausgegeben werden, z. B. Startfehler.

Wenn ein `PreToolUse`-Hook `permissionDecision: "defer"` zurückgibt, hat das Ergebnis `stop_reason: "tool_deferred"` und `deferred_tool_use` enthält die `id`, den `name` und die `input` des ausstehenden Tools. Lesen Sie dieses Feld, um die Anfrage in Ihrer eigenen Benutzeroberfläche anzuzeigen, und setzen Sie dann mit derselben `session_id` fort, um fortzufahren. Siehe [Einen Tool-Aufruf für später aufschieben](/docs/de/hooks#defer-a-tool-call-for-later) für die vollständige Runde.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

Die `uuid` der [`SDKUserMessage`](#sdkusermessage), die den Turn beantwortet, wiedergegeben, damit Sie Claude Codes Antwort mit der Nachricht abgleichen können, die Sie gesendet haben. Claude Code wiederholt eine `uuid` nur, wenn Sie eine auf der Nachricht setzen. Das Feld ist optional auf `SDKUserMessage`, und eine String-Eingabeaufforderung, die an `query()` übergeben wird, trägt keine.

Welche Ihrer Nachrichten ein Turn beantwortet, hängt davon ab, wie der Turn gestartet wurde:

* **Eine reguläre Nachricht, die Sie gesendet haben**, d. h. eine ohne `isSynthetic: true`: Der Turn beantwortet diese Nachricht für seinen gesamten Lauf. Wenn Sie mehrere Nachrichten dicht beieinander senden, kann Claude Code sie in einen Turn zusammenführen, und das Feld trägt dann nur die `uuid` der letzten Nachricht. Um die Antwort mit einer der zusammengeführten Nachrichten abzugleichen, verwenden Sie [`user_message_uuids`](#user_message_uuids).
* **Eine Nachricht, die Sie mit `isSynthetic: true` gesendet haben**: Der Turn beantwortet diese Nachricht zunächst. Wenn Claude Code eine reguläre Nachricht von Ihnen zwischen Tool-Aufrufen aufgreift, beantwortet der Turn die aufgegriffene Nachricht von da an. Das Wiederholen einer synthetischen Nachricht `uuid` erfordert Agent SDK v0.3.265 oder später; frühere Versionen wiederholen nichts bei synthetischen Turns.
* **Eine Eingabeaufforderung, die Claude Code selbst generiert hat**, wie der Turn, der unterbrochene Arbeit nach einem Sitzungsneustart fortsetzt: Der Turn beantwortet zunächst keine Nachricht von Ihnen und seine Frames tragen keine Wiederholung. Wenn Claude Code eine reguläre Nachricht von Ihnen zwischen Tool-Aufrufen aufgreift, beantwortet der Turn diese Nachricht von da an. Die Aufgreif-Wiederholung erfordert Agent SDK v0.3.265 oder später; frühere Versionen wiederholen nichts bei diesen Turns.

Claude Code wiederholt die `uuid` der beantworteten Nachricht auf drei Arten von Frames:

* **Das Ergebnis**: Jedes Ergebnis eines Turns, der eine Nachricht beantwortet, die Sie gesendet haben. Jedes solche Ergebnis trägt es auf Agent SDK v0.3.265 oder später. Vor v0.3.265 fehlte das Success-Ergebnis eines Turns, den eine reguläre Nachricht gestartet hat, wenn der Turn keine API-Anfrage versendet hat oder mit einem aufgeschobenen Tool-Aufruf endete. Vor v0.3.246 fehlte es auch auf Fehler-Ergebnissen, und vor v0.3.216 auf jedem Ergebnis.
* **Die erste Antwort des Turns**: die erste [Assistenten-Nachricht](#sdkassistantmessage), oder mit `includePartialMessages` das erste [Stream-Ereignis](#sdkpartialassistantmessage), dessen `event.type` nicht `ping` ist, damit Sie die Antwort binden können, bevor das Ergebnis ankommt. Wenn ein Turn nichts streamt, setzt Claude Code es auf die erste Assistenten-Nachricht statt. Die erste-Antwort-Wiederholung erfordert Agent SDK v0.3.246 oder später. Wenn sich die Nachricht, die der Turn beantwortet, mitten im Turn ändert, trägt die erste Antwort nach der Änderung das Feld auch auf Agent SDK v0.3.265 oder später; frühere Versionen setzen es auf einen Antwort-Frame pro Turn.
* **Jeder [`thinking_tokens`](#sdkthinkingtokensmessage)-Frame des Turns**: damit Sie den Thinking-Fortschritt der Nachricht zuordnen können, die Sie gesendet haben, ohne auf die erste Antwort des Turns zu warten. Erfordert Agent SDK v0.3.260 oder später.

Claude Code lässt das Feld in diesen Fällen weg:

* Antwort-Frames außer diesen ersten Antworten
* Subagenten-Frames
* Turns, die keine Nachricht mit einer `uuid` beantworten: Der Turn beantwortete eine Nachricht, die Sie ohne eine gesendet haben, oder Claude Code startete den Turn selbst und griff keine reguläre Nachricht auf, die eine hat
* Ergebnisse, die keine Nachricht beantworten, die Sie gesendet haben, wie das zurückgesetzte Ergebnis nach einem abgestürzten Worker-Prozess

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

Die `uuid`s jeder Nachricht, die Sie gesendet haben und die Claude Code in diesem Turn beantwortet hat. Wenn Sie mehrere Nachrichten dicht beieinander senden, kann Claude Code sie in einen Turn zusammenführen, und `user_message_uuid` benennt dann nur die letzte davon. Um die Antwort mit einer der zusammengeführten Nachrichten abzugleichen, suchen Sie nach der `uuid` dieser Nachricht überall in dieser Liste. Erfordert Agent SDK v0.3.259 oder später.

Claude Code setzt die Liste zusammen mit `user_message_uuid` auf jeden Antwort-Frame, der dieses Feld trägt, und auf das Ergebnis. Für den vollständigen Satz von Frames, die `user_message_uuid` tragen, und die Version, die jeder erfordert, siehe [`user_message_uuid`](#user_message_uuid). Die Liste enthält immer `user_message_uuid` und hält höchstens 64 Einträge.

Wenn Claude Code eine reguläre Nachricht aufgreift, die Sie während eines Turns versendet haben, fügt es die `uuid` dieser Nachricht zur Ergebnis-Liste hinzu.

Wenn eine erste Antwort oder ein Ergebnis `user_message_uuid` ohne die Liste trägt, kam es von einer früheren Claude Code-Version, daher greifen Sie auf das einzelne Feld zurück.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

Die Anzahl der Nachrichten, die Sie mit [`origin: { kind: "human" }`](#sdkmessageorigin) gesendet haben und die noch in der Befehlswarteschlange warten, wenn Claude Code das Ergebnis erzeugt hat. Erfordert Agent SDK v0.3.242 oder später.

Was `0` und ein fehlendes Feld Ihnen sagen:

* **`0`**: Claude Code zählt keine Nachrichten, die Sie ohne diese `origin` gesendet haben, und zählt keine Task-Benachrichtigungen, daher kann ein Turn immer noch folgen.
* **Fehlt**: Das endgültige Ergebnis, das Claude Code nach einem Absturz oder fatalen Startfehler ausgibt, lässt das Feld weg, und [kann zurückgesetzte Summen tragen](/docs/de/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Warum Claude Code sich weigerte zu starten, damit Ihre Anwendung die Behebung anbieten kann, anstatt einen Wiederholungsversuch. Claude Code setzt es auf das `error_during_execution`-Ergebnis, das es vor dem Beenden bei einem bekannten Startfehler schreibt. Dieses Ergebnis trägt zurückgesetzte Summen, und sein `errors`-Array trägt den gleichen Text wie stderr. Das Feld fehlt auf jedem anderen Ergebnis. Erfordert Agent SDK v0.3.274 oder später.

Setzen Sie `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` auf `1` in [`env`](#options), um dieses Ergebnis für jeden `SDKStartupFailureReason`-Wert zu erhalten. Ohne diese Variable schreibt Claude Code das Ergebnis nur für diese Fehler, und der Rest endet mit stderr-Ausgabe, einem Nicht-Null-Exit und keiner Ergebnis-Nachricht:

* Ein Resume, das Claude Code stoppt, weil es [die Sitzung nicht zu ihrem Worktree zurückbringen kann](/docs/de/worktrees#the-session-resumes-outside-its-worktree), mit `worktree_unverified` oder `worktree_resume_refused`. Dieser Abschnitt sagt, welcher Fehler welchen Wert trägt.
* Ein abgelehntes [`continue`](#options) einer Konversation, die eine Hintergrund-Sitzung hält, mit `session_held_by_background`. Für ein abgelehntes [`resume`](#options) einer solchen Konversation schreibt Claude Code das Ergebnis nur, wenn die Variable gesetzt ist.

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

Jeder Wert benennt eine Ablehnung:

| Wert                                   | Was die Sitzung stoppte                                                                                                                                                                                                                   |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org_pin_api_key_conflict`             | Verwaltete Einstellungen [erfordern eine First-Party- oder Cloud-Gateway-Anmeldung](/docs/de/authentication#restrict-login-to-your-organization), und ein Anthropic API-Schlüssel, Auth-Token oder `apiKeyHelper` ist stattdessen konfiguriert |
| `org_verify_failed`                    | Die Anmeldungs-Organisation konnte nicht gegen die PIN verifiziert werden, z. B. wegen eines Netzwerkfehlers oder eines widerrufenen Tokens                                                                                               |
| `org_pin_mismatch`                     | Die Anmeldung gehört zu einer Organisation, die die PIN nicht erlaubt                                                                                                                                                                     |
| `managed_settings_invalid`             | Verwaltete Richtlinien-Einstellungen konnten nicht gelesen werden, oder die PIN benennt keine Organisation                                                                                                                                |
| `remote_settings_required_unavailable` | Verwaltete Einstellungen, die die Organisation erfordert, konnten nicht geladen werden                                                                                                                                                    |
| `gateway_signin_required`              | Das [Cloud-Gateway](/docs/de/claude-apps-gateway) beendete diese Anmeldung                                                                                                                                                                     |
| `gateway_access_denied`                | Die verwaltete Einstellungs-Anfrage an das Cloud-Gateway kam mit einem 403 zurück, das die [Fehlerbehebungs-Tabelle](/docs/de/claude-apps-gateway-deploy#troubleshooting) des Gateways abdeckt                                                 |
| `proxy_invalid`                        | Eine Proxy-Einstellung ist keine vollständige URL                                                                                                                                                                                         |
| `temp_dir_unusable`                    | Das benutzerspezifische temporäre Verzeichnis ist unsicher oder konnte nicht erstellt werden                                                                                                                                              |
| `cwd_unavailable`                      | Das Arbeitsverzeichnis wurde gelöscht, verschoben oder kann nicht gelesen werden                                                                                                                                                          |
| `shell_tool_missing`                   | Unter Windows ist kein Shell-Tool verfügbar: Git Bash fehlt, und PowerShell fehlt oder ist mit `CLAUDE_CODE_USE_POWERSHELL_TOOL` ausgeschaltet                                                                                            |
| `session_held_by_background`           | Die Konversation zum Fortsetzen oder Fortfahren läuft als [Hintergrund-Sitzung](/docs/de/agent-view)                                                                                                                                           |
| `worktree_resume_refused`              | Der Worktree der Sitzung ist bei seinen Sicherheitsprüfungen fehlgeschlagen, oder das Resume wurde von innen gestartet. `errors` sagt, ob das erneute Ausführen desselben Resumes ohne den Worktree fortgesetzt wird                      |
| `worktree_unverified`                  | Der Worktree der Sitzung konnte gerade nicht verifiziert werden, und ein Wiederholungsversuch kann erfolgreich sein                                                                                                                       |
| `cli_version_too_old`                  | Diese Claude Code-Version liegt unter dem Minimum, das Anthropic erfordert                                                                                                                                                                |
| `bypass_root`                          | Bypass-Berechtigungsmodus wurde angefordert, während als Root ausgeführt wird                                                                                                                                                             |

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

System-Initialisierungsnachricht.

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

`fast_mode_state` meldet den [Fast Mode](/docs/de/fast-mode)-Status der Sitzung. Wenn etwas Fast Mode blockiert, benennt `fast_mode_disabled_reason` die Überprüfung, die es blockiert hat; das Feld erfordert Claude Code v2.1.219 oder später. Für die Grund-Codes und ihre Bedeutungen siehe [`fast_mode_disabled_reason`](#sdkresultmessage) auf der Ergebnis-Nachricht.

`terminal_slash_commands` benennt die Einträge in `slash_commands`, deren Schnittstelle an das lokale Terminal gebunden ist, wie `exit`. Sie können sie wie jeden anderen Eintrag in `slash_commands` senden; das Feld existiert, damit ein Remote- oder Mobile-Client sie aus seinen Befehlsmenüs ausblenden kann. Das Feld ist nur vorhanden, wenn nicht leer, und erfordert Agent SDK v0.3.229 oder später.

*

`source` auf jedem `mcp_servers`-Eintrag: Woher die Definition des Servers kam, mit den gleichen Werten wie [`McpServerStatus`](#mcpserverstatus)s `source`. Erfordert Agent SDK v0.3.274 oder später.

*

`effort`: Die [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level), die Claude Code bei der nächsten Anfrage der Sitzung sendet, oder `null`, wenn es keine sendet. Claude Code setzt das Feld nur auf die Init-Nachricht, die es an [Remote Control](/docs/de/remote-control)-Clients sendet, und lässt es aus der Init-Nachricht weg, die Ihre Anwendung liest. Erfordert Agent SDK v0.3.234 oder später.

Das `capabilities`-Array benennt die Protokoll-Verhaltensweisen, die diese CLI implementiert, damit Sie Feature-Erkennung durchführen können, anstatt `claude_code_version`-Zeichenketten zu vergleichen. Es ist ein offenes Set: Ignorieren Sie Werte, die Sie nicht erkennen, und überprüfen Sie auf die spezifische Fähigkeit, deren Verhalten Sie benötigen. Das Feld erfordert Claude Code v2.1.205 oder später und fehlt auf früheren CLIs.

| Fähigkeit                    | Bedeutung                                                                                                                                                                                                                                                                                                           |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) wird mit einer [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse)-Quittung aufgelöst, die die Nachrichten benennt, die ausstanden, als der Interrupt ankam                                                                                                                |
| `interrupt_cancel_queued_v1` | Die `interrupt`-Steueranfrage ehrt `cancel_queued: true`, bricht die Nachrichten ab, die die Quittung sonst unter `still_queued` auflisten würde, und listet sie stattdessen unter `cancelled` auf. Siehe [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse). Erfordert Claude Code v2.1.219 oder später |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Streaming-Teilnachricht (nur wenn `includePartialMessages` true ist). Das `parent_tool_use_id`-Feld ist immer `null`: Stream-Ereignisse werden nur für die Hauptsitzung ausgegeben. Für die Zuordnung von Subagenten verwenden Sie vollständige Nachrichten, die `parent_tool_use_id` enthalten, oder aktivieren Sie [`forwardSubagentText`](#options), um Subagenten-Text und Thinking als vollständige Nachrichten zu erhalten.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Aus Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Zeit bis zum ersten Token in ms, nur bei message_start-Ereignissen vorhanden
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code setzt `user_message_uuid` und `user_message_uuids` auf das erste nicht-ping-Stream-Ereignis des Turns und erneut, wenn sich die Nachricht, die der Turn beantwortet, ändert, unter den Bedingungen in [`user_message_uuid`](#user_message_uuid).

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

Nachricht, die eine Konversations-Komprimierungsgrenze anzeigt.

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

Generisches Text-Banner, das von der Schleife ausgegeben wird. Enthält nicht-fehlerhafte Statuszeilen, Hook-Feedback wie ein Block-Grund eines `UserPromptSubmit`-Hooks und Befehlsausgabe. Auf Claude Code v2.1.227 oder später kann die [`systemMessage`](/docs/de/hooks#json-output) eines Hooks als diese Nachricht ankommen, mit jeder Zeile mit dem Namen des Hooks vorangestellt, wie `PostToolUse:Bash says:`. Ob die `systemMessage` eines Hooks als diese Nachricht ankommt, hängt vom Ereignis ab. Jeder [Ereignisabschnitt](/docs/de/hooks#hook-events) auf der Hooks-Seite sagt, wie die Ausgabe angezeigt wird. Rendern Sie `content` als Klartext auf der angegebenen `level`.

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

Wird bei ordnungsgemäßem Herunterfahren des Workers ausgegeben, damit Remote-Clients sehen können, warum der Worker verschwunden ist, anstatt auf Heartbeat-Timeout zu warten. Der `reason` ist eine kurze snake\_case-Zeichenkette, die von der Host-CLI gesetzt wird, z. B. `"host_exit"` oder `"remote_control_disabled"`. Handeln Sie nur dann, wenn Sie live streamen. Eine wiederaufgenommene Sitzung spielt vergangene Instanzen dieser Nachricht ab, also ignorieren Sie sie in diesem Fall.

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

Plugin-Installationsfortschritt-Ereignis. Wird ausgegeben, wenn [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/de/env-vars) gesetzt ist, damit Ihre Agent SDK-Anwendung die Marketplace-Plugin-Installation vor dem ersten Turn verfolgen kann. Die `started`- und `completed`-Status klammern die Gesamtinstallation. Die `installed`- und `failed`-Status melden einzelne Marketplaces und enthalten `name`.

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

Stream-Ereignis, das ausgegeben wird, wenn das Berechtigungssystem einen Tool-Aufruf ablehnt, ohne eine interaktive Eingabeaufforderung anzuzeigen. Verwenden Sie es, um die Ablehnung in Ihrer Benutzeroberfläche zu rendern, während sie geschieht, anstatt nur das `is_error`-Tool-Ergebnis zu beobachten, das folgt. Welche Ablehnungen es meldet, hängt davon ab, wie der Lauf Berechtigungsaufforderungen handhabt:

* **Mit einem [`canUseTool`](#canusetool)-Callback** und dem Standard [`permissionPrompts: 'host'`](#options): Berechtigungsaufforderungen gehen an Ihren Callback, und dieses Ereignis meldet die Ablehnungen, die Claude Code auf eigene Faust entscheidet, ohne es aufzurufen.
* **Mit keinem**: ein bloßer `-p`-Lauf, oder `query()`, das weder `canUseTool` noch `permissionPromptToolName` setzt, lehnt jeden Tool-Aufruf ab, der aufgefordert hätte, und dieses Ereignis meldet diese Ablehnungen sowie die, die Claude Code auf eigene Faust entscheidet. Vor v2.1.223 gab Claude Code dieses Ereignis nicht in Läufen ohne Callback aus.
* **Mit einem MCP-Aufforderungs-Tool**, gesetzt mit `permissionPromptToolName` oder dem [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags)-Flag, und dem Standard `permissionPrompts: 'host'`: Claude Code gibt dieses Ereignis überhaupt nicht aus, nicht einmal für die Regel-Ablehnungen, die es auf eigene Faust entscheidet.
* **Mit [`permissionPrompts: 'none'`](#options)**: Claude Code lehnt die Aufrufe ab, die aufgefordert hätten, auch wenn `canUseTool` oder ein MCP-Aufforderungs-Tool auch gesetzt ist, und dieses Ereignis meldet diese Ablehnungen sowie die, die Claude Code auf eigene Faust entscheidet. Erfordert Claude Code v2.1.259 oder später.

In jeder Konfiguration überspringt dieses Ereignis jede Ablehnung, die auf dem `PreToolUse`-Hook-Pfad entschieden wurde, ob der Hook den Aufruf selbst ablehnte oder eine Deny-Regel die Allow- oder Ask-Entscheidung des Hooks überschrieb. Das Ereignis ist auch Best-Effort: Gelegentlich zeichnet Claude Code eine Ablehnung auf, ohne dieses Ereignis auszugeben, daher ist `permission_denials` auf der [Ergebnis-Nachricht](#sdkresultmessage) der maßgebliche Datensatz.

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

| Feld                   | Typ      | Beschreibung                                                                                                                                       |
| ---------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Name des Tools, das abgelehnt wurde                                                                                                                |
| `tool_use_id`          | `string` | ID des `tool_use`-Blocks, auf den diese Ablehnung antwortet                                                                                        |
| `agent_id`             | `string` | Subagent-ID, wenn der abgelehnte Aufruf innerhalb eines Subagenten stammt. Spiegelt das Feld auf `can_use_tool` für das Routing auf der Host-Seite |
| `decision_reason_type` | `string` | Diskriminator für die Komponente, die entschieden hat, z. B. `"rule"`, `"mode"`, `"classifier"` oder `"asyncAgent"`                                |
| `decision_reason`      | `string` | Menschenlesbarer Grund von der entscheidenden Komponente, wenn verfügbar                                                                           |
| `message`              | `string` | Ablehnungsnachricht, die an das Modell im `tool_result` zurückgegeben wird                                                                         |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Informationen über einen verweigerten Tool-Einsatz.

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

Strukturierte Form des `/context`-Berichts, getragen als `context_usage` auf der [`SDKAssistantMessage`](#sdkassistantmessage), die ein `/context`-Ergebnis liefert. Agent SDK v0.3.232 und später exportieren den Typ. Im Gegensatz zu [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) trägt es nur die Daten, die benötigt werden, um die Nutzungsaufschlüsselung zu rendern, ohne Anzeigefelder wie `color` und `gridRows`.

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

Die Tabelle listet auf, was Claude Code in jedes Feld einfügt. Die Felder von `model` bis `over_limit` beschreiben die Sitzung als Ganzes, und die Sammlungsfelder ordnen Token einzelnen Elementen zu.

| Feld             | Typ                                                       | Beschreibung                                                                                                                                                                                                                                                                                                                             |
| ---------------- | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | Das Modell der Hauptschleife, für das Claude Code die Nutzung berechnet hat, nicht das eines Subagenten                                                                                                                                                                                                                                  |
| `total_tokens`   | `number`                                                  | Claude Codes Schätzung der verwendeten Token. Nicht auf das Fenster begrenzt, daher kann es `raw_max_tokens` überschreiten, wenn die Sitzung über dem Limit ist                                                                                                                                                                          |
| `raw_max_tokens` | `number`                                                  | Das Kontext-Fenster des Modells, oder das niedrigere [Auto-Komprimierungs-Fenster](/docs/de/model-config#context-window-and-auto-compaction), wenn eines gilt, wie eines, das Sie setzen, oder die 200K-Grenze, die Claude Code auf einige Modelle mit einem 1M-Token-Fenster anwendet. Claude Code misst `total_tokens` gegen dieses Fenster |
| `percentage`     | `number`                                                  | `total_tokens` als gerundeter Prozentsatz von `raw_max_tokens`, daher kann es 100 überschreiten, wenn die Sitzung über dem Limit ist                                                                                                                                                                                                     |
| `over_limit`     | `object`                                                  | Nur vorhanden, wenn `total_tokens` `raw_max_tokens` überschreitet. `tokens_over` ist der Betrag über, und `kind` sagt, wie Claude Code das Fenster aufgelöst hat                                                                                                                                                                         |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | Ein Eintrag pro Zeile der Nutzungs-nach-Kategorie-Aufschlüsselung                                                                                                                                                                                                                                                                        |
| `mcp_tools`      | `object[]`                                                | Token, die jedem MCP-Tool zugeordnet sind, mit seinem Draht-Namen, wie `mcp__linear__create_issue`, und seinem `server_name`                                                                                                                                                                                                             |
| `memory_files`   | `object[]`                                                | Token, die jeder geladenen Speicherdatei zugeordnet sind, mit ihrem `path` und einem Quellen-Label wie `Project` oder `User` in `type`                                                                                                                                                                                                   |
| `agents`         | `object[]`                                                | Token, die jeder benutzerdefinierten Subagenten-Definition zugeordnet sind, mit einem Quellen-Identifikator wie `projectSettings`, `userSettings` oder `plugin`. Integrierte Subagenten sind nicht aufgelistet                                                                                                                           |
| `skills`         | `object[]`                                                | Token, die jedem Skill in der Skill-Auflistung zugeordnet sind, mit einem Quellen-Identifikator und, für Plugin-Skills, dem Namen des Plugins in `plugin_name`. Fehlt, wenn keine Skills Token beitragen                                                                                                                                 |

`over_limit.kind` zeichnet auf, wie Claude Code das Fenster aufgelöst hat, nicht ob die API die nächste Anfrage akzeptiert:

* `hard_limit`: Das Fenster ist das, was Claude Code für das eigene Limit des Modells hält, über das die API Anfragen ablehnt
* `compaction_window`: Das Fenster ist ein Komprimierungs-Richtlinien-Fenster, das möglicherweise mit dem Limit des Modells übereinstimmt oder nicht

Claude Code entwickelt den Typ additiv, indem es neue Daten als optionale Felder hinzufügt, anstatt bestehende umzugestalten. Lesen Sie die Felder, die Sie kennen, und ignorieren Sie alle, die Sie nicht erkennen.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

Eine Zeile der `/context`-Nutzungs-nach-Kategorie-Aufschlüsselung.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

Die Tabelle listet auf, was Claude Code in jedes Feld einer Zeile einfügt.

| Feld     | Typ      | Beschreibung                                                                                                                   |
| -------- | -------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `name`   | `string` | Der Anzeigename der Zeile, wie `/context` ihn druckt, z. B. `Messages`. Klassifizieren Sie Zeilen nach `kind`, nicht nach Name |
| `tokens` | `number` | Die Token-Anzahl der Zeile. Zeilen können null Token tragen                                                                    |
| `kind`   | `string` | Was die Zeile darstellt: `used`, `free`, `buffer` oder `deferred`                                                              |

Jeder `kind`-Wert sagt, was die Token der Zeile sind:

* `used`: Inhalt, der das Kontext-Fenster besetzt
* `free`: Das verbleibende Fenster
* `buffer`: Die Komprimierungs-Reserve
* `deferred`: Tool-Schemas, die Claude Code aus dem Fenster hält und von der Nutzungsberechnung ausschließt, aufgelistet zur Kenntnis

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Herkunft einer Benutzer-Rolle-Nachricht. Dies erscheint als `origin` auf [`SDKUserMessage`](#sdkusermessage) und wird auf die entsprechende [`SDKResultMessage`](#sdkresultmessage) weitergeleitet, damit Sie erkennen können, was einen bestimmten Turn ausgelöst hat.

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

| `kind`              | Bedeutung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Direkte Eingabe vom Endbenutzer. Wenn Ihre Anwendung das, was der Benutzer eingegeben hat, als Benutzer-Nachricht weiterleitet, setzen Sie seine `origin` explizit auf `{ kind: "human" }`: Claude Code behandelt eine Benutzer-Nachricht ohne `origin` als nicht zugeordnet, und überprüft, die einen menschlich eingegebenen Prompt erfordern, wie das [`ultracode`-Workflow-Schlüsselwort](/docs/de/workflows#ask-for-a-workflow-in-your-prompt), akzeptieren es nicht. Vor v2.1.210 behandelte Claude Code ein fehlendes `origin` auf einer Benutzer-Nachricht als menschliche Eingabe. |
| `channel`           | Nachricht, die auf einem [Kanal](/docs/de/channels) ankommt. `server` ist der Name des Quell-MCP-Servers.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `peer`              | Nachricht von einem anderen Agent: ein In-Process-[Teamkollege](/docs/de/agent-teams) oder ein [Cross-Session-Peer](/docs/de/cross-session-messaging), eine andere Ihrer Claude Code-Sitzungen. Siehe [Peer-Herkunftsfelder](#peer-origin-fields) für die Pro-Feld-Semantik und das Vertrauensmodell.                                                                                                                                                                                                                                                                                            |
| `task-notification` | Synthetischer Turn, der für eine Lieferung eingefügt wird, die ohne einen frischen Benutzer-Prompt ankommt, wie eine beendete Hintergrund-Aufgabe; siehe [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) für diesen Arm. Das optionale `subkind` markiert, was die Benachrichtigung ausgelöst hat. Siehe [Task-Notification-Subkinds](#task-notification-subkinds).                                                                                                                                                                                                        |
| `coordinator`       | Nachricht von einem Team-Koordinator in einem [Agent-Team](/docs/de/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `auto-continuation` | Synthetischer Turn, der eingefügt wird, wenn die Sitzung ohne frische Benutzereingabe fortgesetzt wird, z. B. ein Befehlsergebnis, das eine Follow-up-Eingabeaufforderung auslöst.                                                                                                                                                                                                                                                                                                                                                                                                     |
| `unclassified`      | Eingefügter Turn, dessen Herkunft nicht bestimmt werden konnte. Erfordert Claude Code v2.1.223 oder später. Wenn Claude Code eine [`SDKUserMessage`](#sdkusermessage) mit `isSynthetic: true` empfängt und sie nicht als eine andere `kind` klassifizieren kann, setzt es diese Art, wenn die Nachricht ankommt, und rahmt den Turn zum Modell als eine Nicht-Benutzer-Quelle ein, anstatt ihn als menschliche Eingabe zu behandeln. Ihre Anwendung sollte diesen Wert nicht setzen.                                                                                                   |

<h3 id="task-notification-subkinds">
  Task-Notification-Subkinds
</h3>

Wenn Claude Code eine Task-Benachrichtigung in eine Sitzung liefert, setzt es `subkind` auf der Benachrichtigungs-`origin` nur, wenn Anthropic-Server verifizierten, woher diese Benachrichtigung kam. Es setzt auch `subkind`, wenn Ihre Anwendung [die Nachricht als geplante Ausführung erklärt](#declare-a-scheduled-run), was TypeScript Agent SDK v0.3.280 oder später erfordert. `subkind` erfordert Claude Code v2.1.213 oder später, und es nimmt einen von zwei Werten an:

* `scheduled-trigger`: Die Benachrichtigung ist ein [Routine](/docs/de/routines)-gespeicherter Prompt, geliefert, weil einer der Routine-Trigger ausgelöst wurde: sein Zeitplan, sein [API-Trigger](/docs/de/routines#add-an-api-trigger), sein [GitHub-Trigger](/docs/de/routines#add-a-github-trigger) oder **Jetzt ausführen**. Eine Eingabeaufforderung, die Ihre Anwendung [als geplante Ausführung erklärt](#declare-a-scheduled-run), trägt auch diesen Wert. Claude Code rahmt diese zum Modell als die zugewiesene Aufgabe der Sitzung ein, mit einer anderen Benachrichtigung als die [Benachrichtigung, die andere Task-Benachrichtigungen tragen](#sdktasknotificationmessage).
*

`peer-send-message`: Die Benachrichtigung ist eine Nachricht, die eine andere Ihrer Sitzungen mit dem Server-seitigen `send_message`-Tool gesendet hat, das [Claude Code im Web](/docs/de/claude-code-on-the-web)-Sitzungen verwenden, um sich gegenseitig zu schreiben, nicht das [Cross-Session-`SendMessage`-Tool](/docs/de/cross-session-messaging), und Anthropic-Server verifizierten, dass beide Sitzungen zur gleichen privaten Gruppe von Sitzungen gehören. Erfordert Claude Code v2.1.224 oder später. Eine `send_message`-Lieferung, die die Server nicht auf diese Weise verifizierten, bekommt keinen subkind.

Jede andere Task-Benachrichtigung hat keinen `subkind`. Das schließt [geplante Aufgaben](/docs/de/scheduled-tasks) ein, die auf Ihrer eigenen Maschine ausgelöst werden, [PR-Aktivität](/docs/de/claude-code-on-the-web#how-claude-responds-to-pr-activity), die in eine Sitzung geliefert wird, und Hintergrund-Ereignisse wie eine beendete Aufgabe. Nachrichten vom [Cross-Session-`SendMessage`-Tool](/docs/de/cross-session-messaging) sind überhaupt keine Task-Benachrichtigungen: Ob sie von einer Sitzung auf der gleichen Maschine oder durch Anthropic-Server von einer anderen Maschine kommen, gibt Claude Code ihnen `kind: "peer"` und die [Peer-Herkunftsfelder](#peer-origin-fields).

`fireReason` sagt, warum eine `scheduled-trigger`-Benachrichtigung ausgelöst wurde, als ein kurzes Kleinbuchstaben-Token wie `scheduled`, `manual`, `retry`, `catch_up` oder `api`. Anthropic-Server setzen es auf die Lieferungen einer [Routine](/docs/de/routines), und Ihre Anwendung setzt es, wenn sie eine geplante Ausführung erklärt. Es fehlt, wenn keiner einen gesendet hat. Erfordert TypeScript Agent SDK v0.3.280 oder später.

<h4 id="declare-a-scheduled-run">
  Geplante Ausführung erklären
</h4>

Wenn Ihre Anwendung Eingabeaufforderungen nach ihrem eigenen Zeitplan ausführt, erklären Sie jede Ausführung, damit Claude Code den Turn zum Modell als geplante Aufgabe rahmt, anstatt als Live-Eingabe vom Benutzer. Starten Sie die Sitzung mit `CLAUDE_CODE_HOST_SCHEDULED_RUN` auf `1` in [`env`](#options), dann senden Sie die [`SDKUserMessage`](#sdkusermessage) der Ausführung mit `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` und ohne `isSynthetic`. Claude Code ignoriert die Erklärung in einem Prozess, der ohne diese Variable gestartet wurde. Es ignoriert sie auch in einem Prozess, dessen Umgebung [`CLAUDECODE`](/docs/de/env-vars) oder `CLAUDE_CODE_CHILD_SESSION` trägt. Claude Code behält `fireReason` nur, wenn der Wert 1 bis 32 Kleinbuchstaben oder Unterstriche ist. Erfordert TypeScript Agent SDK v0.3.280 oder später.

<h3 id="peer-origin-fields">
  Peer-Herkunftsfelder
</h3>

Eine `peer`-Herkunft identifiziert, welcher Agent die Nachricht gesendet hat: ein In-Process-[Teamkollege](/docs/de/agent-teams), der an `main` mit `SendMessage` sendet, oder ein [Cross-Session-Peer](/docs/de/cross-session-messaging), eine andere Ihrer Claude Code-Sitzungen. Cross-Session-Peers erfordern Claude Code v2.1.224 oder später auf macOS und Linux; siehe [Cross-Session-Messaging-Verfügbarkeit](/docs/de/cross-session-messaging#availability) für die native Windows-Anforderung. Ein Cross-Session-Peer kann auf der gleichen Maschine laufen, oder auf [einer anderen Ihrer Maschinen](/docs/de/cross-session-messaging#message-sessions-on-other-machines) oder [Claude Code im Web](/docs/de/claude-code-on-the-web), wenn seine Nachricht durch Remote Control ankommt. Die zwei Arten von Sendern füllen die Felder unterschiedlich:

* `from`: Der Name des Teamkollegen, oder die Absenderadresse für einen Cross-Session-Peer. Für eine [Einweg-Cross-Machine-Nachricht](/docs/de/cross-session-messaging#message-sessions-on-other-machines) hat der Sender keine Antwortadresse und `from` ist `"unknown"`. Der Wert ist vom Sender verfasst; `verifiedPeerPid` ist die verifizierte Identität.
*

`fromMode`: Die Berechtigungsklasse der sendenden Sitzung, `bypass` oder `prompting`, erklärt von einem Host, der eine Peer-Nachricht zwischen Ihren Sitzungen weitergeleitet, wie die [Desktop-App](/docs/de/desktop#work-across-sessions). Claude Code liest es in der empfangenden Sitzung, wenn es die [Inbound-Steuerungen](/docs/de/cross-session-messaging#control-inbound-messages) anwendet. Erfordert Agent SDK v0.3.234 oder später.

* `senderTaskId`: Die Task-ID des Teamkollegen. Fehlt für einen Cross-Session-Peer.
*

`name`: Der Anzeigename des Absenders, normalisiert von Claude Code: Es entfernt Unicode-Steuer-, Format-, Surrogate- und Zeilen- oder Absatz-Trennzeichen-Codepunkte, schneidet dann das Ergebnis ab und begrenzt es auf 64 Codepunkte mit einer Ellipse. Erfordert Claude Code v2.1.205 oder später.

*

`body`: Der dekodierte Nachrichtentext mit der Peer-Hülle entfernt, byte-genau mit dem, was das Modell sieht. Immer vorhanden für eine Teamkollegen-Nachricht; für einen Cross-Session-Peer, nur vorhanden, wenn der Turn genau eine von Claude Code gebildete Peer-Hülle ist. Rendern Sie `name` und `body` anstatt die Nachricht erneut zu analysieren. Erfordert Claude Code v2.1.205 oder später.

*

`fromSession`: Die Host-öffnbare Session-ID des Absenders, gesetzt vom Host des Absenders, damit Ihre Benutzeroberfläche zurück zur sendenden Sitzung verlinken kann. Wie `from` ist es vom Sender behauptet: Verwenden Sie es nur als Navigationsziel, und behandeln Sie es nicht als Beweis der Identität des Absenders. Erfordert Claude Code v2.1.216 oder später.

*

`verifiedPeerPid`: Die Prozess-ID des Prozesses, der sich mit dem Cross-Session-Messaging-Socket dieser Sitzung verbunden hat, verifiziert vom Kernel und aus der Verbindung selbst gelesen, niemals aus der Nutzlast. Verwenden Sie es, nicht `from`, um den Absender zu identifizieren: `from` ist von jedem gleichen Benutzer-Prozess fälschbar. Das Feld fehlt, wenn Claude Code es nicht verifizieren kann, wie auf Windows oder Nicht-Socket-Eingang, daher bedeutet ein fehlendes Wert, dass der Absender nicht verifiziert ist. Für weitergeleitet Traffic identifiziert es das Relay anstelle des Autors der Nachricht, und Prozess-IDs sind wiederverwendbar, daher behandeln Sie es als Herkunft anstelle eines Authentifizierungs-Tokens. Erfordert Claude Code v2.1.216 oder später.

<h2 id="hook-types">
  Hook-Typen
</h2>

Einen umfassenden Leitfaden zur Verwendung von Hooks mit Beispielen und häufigen Mustern finden Sie im [Hooks-Leitfaden](/docs/de/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Verfügbare Hook-Ereignisse.

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

Hook-Callback-Funktionstyp.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // Union aller Hook-Eingabetypen
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

Hook-Konfiguration mit optionalem Matcher.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // Timeout in Sekunden für alle Hooks in diesem Matcher
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

Union-Typ aller Hook-Eingabetypen.

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

Basis-Schnittstelle, die alle Hook-Eingabetypen erweitern.

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

Das Feld `prompt_id` ist eine UUID, die die derzeit verarbeitete Benutzereingabe identifiziert. Sie entspricht dem [`prompt.id`-Attribut bei OpenTelemetry-Ereignissen](/docs/de/monitoring-usage#event-correlation-attributes) und ist bis zur ersten Benutzereingabe nicht vorhanden. Erfordert Claude Code v2.1.196 oder später.

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

`mcp_server` ist vorhanden, wenn das Tool von einem MCP-Server stammt; siehe [`McpServerProvenance`](#mcpserverprovenance). Die Eingaben `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` und `PermissionDenied` enthalten das gleiche Feld. Das Feld erfordert Agent SDK v0.3.274 oder später.

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

Wird einmal ausgelöst, nachdem jeder Werkzeugaufruf in einem Batch aufgelöst wurde, bevor die nächste Modellanfrage erfolgt. `tool_response` enthält den serialisierten `tool_result`-Inhalt, den das Modell sieht; die Form unterscheidet sich vom strukturierten `Output`-Objekt von `PostToolUseHookInput`.

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
  reason: ExitReason; // String aus EXIT_REASONS-Array
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

Wird ausgelöst, bevor ein angefordeter Modellwechsel wirksam wird. `context_tokens` und die Felder danach schätzen, was das erneute Senden des Gesprächs an das neue Modell kostet. Für die vollständigen Feldbeschreibungen und Blocking-Semantik siehe [PreModelSwitch](/docs/de/hooks#premodelswitch).

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

Wird ausgelöst, nachdem sich das Modell der Sitzung ändert. Es enthält die gleichen Felder wie `PreModelSwitchHookInput`, mit zwei weiteren `source`-Werten. Siehe [PostModelSwitch](/docs/de/hooks#postmodelswitch).

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
  /** @deprecated seit v2.1.178. Enthält den von der Sitzung abgeleiteten Teamnamen; wird entfernt. */
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
  /** @deprecated seit v2.1.178. Enthält den von der Sitzung abgeleiteten Teamnamen; wird entfernt. */
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
  /** @deprecated seit v2.1.178. Enthält den von der Sitzung abgeleiteten Teamnamen; wird entfernt. */
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

`directory` ist der absolute Pfad des hinzugefügten Verzeichnisses. `source` ist `"slash_command"`, wenn `/add-dir` es hinzugefügt hat, und `"register_repo_root"`, wenn die SDK-Steueranfrage dies tat.

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

Hook-Rückgabewert.

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
   * Eine Terminal-Escape-Sequenz (z. B. OSC 9 / OSC 777 Desktop-Benachrichtigung)
   * für Claude Code, um in Ihrem Namen auszugeben. Nur Benachrichtigungs-/Titel-OSCs
   * (0, 1, 2, 9, 99, 777) und BEL sind zulässig; ein Wert, der etwas anderes enthält,
   * wird als Ganzes ignoriert. Nur die interaktive CLI gibt es aus; das SDK ignoriert das Feld.
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
        /** Wenn decision "block" ist, lassen Sie die ursprüngliche Eingabeaufforderung aus der Blocknachricht weg. */
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
         * Scannen Sie Skill- und Befehlsverzeichnisse erneut, nachdem SessionStart-Hooks
         * abgeschlossen sind, damit im Hook installierte Skills in der gleichen Sitzung verfügbar sind.
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
         * Gleicher Vertrag wie PreToolUse: "allow" setzt fort, "deny" bricht
         * den Wechsel ab, "ask" fragt den Benutzer zur Bestätigung. Nur /model in einer
         * interaktiven Sitzung zeigt diese Eingabeaufforderung; jede andere Oberfläche,
         * einschließlich set_model-Anfragen, behandelt "ask" als Ablehnung.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** Erreicht das Modell mit der nächsten Anfrage, die das neue Modell bedient. */
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
         * Kurze Notiz über das Ergebnis dieses Werkzeugaufrufs für den automatischen Modus
         * Berechtigungsklassifizierer. Begrenzt auf 2000 Zeichen, geteilt über
         * alle Hooks, die auf denselben Aufruf reagieren; berücksichtigt bei synchronen
         * Hook-Antworten nur. Kopieren Sie nicht vertrauenswürdige Werkzeugausgabe hinein.
         */
        classifierContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Verwenden Sie `updatedToolOutput`, das für alle Tools funktioniert. */
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
        /** Text, der anstelle des Delta angezeigt wird. Weglassen (oder das Delta unverändert zurückgeben), um das Original anzuzeigen. */
        displayContent?: string;
      };
};
```

<h2 id="tool-input-types">
  Tool-Eingabetypen
</h2>

Dokumentation von Eingabeschemas für alle integrierten Claude Code-Tools. Diese Typen werden aus `@anthropic-ai/claude-agent-sdk` exportiert und können für typsichere Tool-Interaktionen verwendet werden.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Union von Tool-Eingabetypen, exportiert aus `@anthropic-ai/claude-agent-sdk`; Mitglieder umfassen:

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

**Tool-Name:** `Agent`. Der vorherige Name `Task` wird immer noch als Alias akzeptiert, und das `tools`-Array in der [`SDKSystemMessage`](#sdksystemmessage)-Init-Nachricht listet dieses Tool derzeit als `Task` für Rückwärtskompatibilität auf.

<Note>
  Das Feld `mode` ist veraltet und wird auf Claude Code v2.1.212 oder später ignoriert. Ein Subagent wird entweder im Berechtigungsmodus der übergeordneten Sitzung oder in der [`permissionMode`](#agentdefinition) seiner Definition ausgeführt, und die [Subagent-Vererbungsregeln](/docs/de/agent-sdk/permissions#available-modes) entscheiden, welche.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Veraltet; wird ignoriert
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Veraltet; wird ignoriert. Die Subagent-Vererbungsregeln entscheiden über den Berechtigungsmodus eines Subagenten
  isolation?: "worktree" | "remote";
};
```

Startet einen neuen Agenten, um komplexe, mehrstufige Aufgaben autonom zu bewältigen.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Tool-Name:** `AskUserQuestion`

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

Stellt dem Benutzer während der Ausführung Klärungsfragen. Siehe [Genehmigungen und Benutzereingaben verarbeiten](/docs/de/agent-sdk/user-input#handle-clarifying-questions) für Verwendungsdetails.

<h3 id="bash">
  Bash
</h3>

**Tool-Name:** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

Führt Bash-Befehle mit optionalem Timeout und Hintergrundausführung aus. Das Arbeitsverzeichnis bleibt zwischen Befehlen erhalten, einschließlich Befehlen, die in späteren Turns einer Multi-Turn-Sitzung ausgeführt werden; Shell-Status wie exportierte Umgebungsvariablen nicht. Für die Grenzen, welche Verzeichniswechsel übertragen werden, siehe [Was zwischen Befehlen erhalten bleibt](/docs/de/tools-reference#what-persists-between-commands).

<h3 id="monitor">
  Monitor
</h3>

**Tool-Name:** `Monitor`

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

Führt eine Hintergrundquelle aus und liefert jedes Ereignis an Claude, damit es reagieren kann, ohne zu pollen: `command` führt ein Skript aus und gibt ein Ereignis pro Stdout-Zeile aus, und `ws` öffnet einen WebSocket und gibt ein Ereignis pro Textframe aus. Geben Sie genau eines von `command` oder `ws` an. Die `ws`-Quelle erfordert Claude Code v2.1.195 oder später.

`timeout_ms` ist die Frist der Überwachung in Millisekunden. Sie ist standardmäßig 300000 und akzeptiert Werte bis zu 3600000. Die effektive Frist beträgt höchstens 1800000, was 30 Minuten entspricht, daher wird ein größerer akzeptierter Wert auf diesen gekürzt. Bei Erreichen der Frist endet die Überwachung und Claude erhält eine Benachrichtigung, damit es eine neue Überwachung starten kann, falls es noch eine benötigt.

Der exportierte Typ markiert `timeout_ms` als erforderlich, da das Schema den Standard ausfüllt; ein Aufruf, der es auslässt, validiert.

Wenn Monitor einen Befehl ausführt, folgt es den gleichen Berechtigungsregeln wie Bash; ein WebSocket-Watch fordert separat zur Genehmigung auf. Siehe die [Monitor-Tool-Referenz](/docs/de/tools-reference#monitor-tool) für Verhalten und Anbieter-Verfügbarkeit.

<h3 id="taskoutput">
  TaskOutput
</h3>

Entfernt in Claude Code v2.1.277, zusammen mit seinem `TaskOutputInput`-Typ. Zuvor abgerufene Ausgabe aus einer laufenden oder abgeschlossenen Hintergrund-Aufgabe; Claude liest die Ausgabedatei einer Hintergrund-Aufgabe stattdessen mit `Read`.

Ein `disallowedTools`-Eintrag oder eine Deny-Regel, die immer noch `TaskOutput` benennt, wird ohne Warnung ignoriert.

<h3 id="edit">
  Edit
</h3>

**Tool-Name:** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

Führt exakte String-Ersetzungen in Dateien durch.

<h3 id="read">
  Read
</h3>

**Tool-Name:** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

Liest Dateien aus dem lokalen Dateisystem, einschließlich Text, Bilder, PDFs und Jupyter-Notebooks. Verwenden Sie `pages` für PDF-Seitenbereiche (z. B. `"1-5"`).

Für ein PDF erhält Claude den Inhalt der Datei innerhalb des `tool_result`-Inhalts des Read-Aufrufs. Ein Read, das die `pdf`-[Ausgabe](#tool-output-types) zurückgibt, enthält einen zusammenfassenden `text`-Block gefolgt von einem `document`-Block. Ein Read, das die `parts`-Ausgabe zurückgibt, enthält den zusammenfassenden `text`-Block gefolgt von einem Block pro extrahierter Seite: ein `image`-Block oder ein `text`-Block, der die Seite benennt, wenn Claude Code sie nicht als Bild rendern konnte. Vor Agent SDK v0.3.242 lieferte Claude Code den Inhalt der Datei als separate `user`-Nachricht nach dem Tool-Ergebnis.

<h3 id="write">
  Write
</h3>

**Tool-Name:** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

Schreibt eine Datei in das lokale Dateisystem, überschreibt, falls vorhanden.

<h3 id="glob">
  Glob
</h3>

**Tool-Name:** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

Schnelle Datei-Musterabstimmung, die mit jeder Codebasis-Größe funktioniert.

<h3 id="grep">
  Grep
</h3>

**Tool-Name:** `Grep`

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

Leistungsstarkes Suchtool, das auf ripgrep mit Regex-Unterstützung basiert.

<h3 id="taskstop">
  TaskStop
</h3>

**Tool-Name:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Veraltet: Verwenden Sie task_id
};
```

Beendet eine laufende Hintergrund-Aufgabe oder Shell nach ID. Ab v2.1.198 akzeptiert `task_id` auch einen Agent-Team-Teamkollegen oder einen benannten Hintergrund-Agenten nach Agent-ID oder Name.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Tool-Name:** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Bearbeitet Zellen in Jupyter-Notebook-Dateien.

<h3 id="webfetch">
  WebFetch
</h3>

**Tool-Name:** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

Ruft Inhalte von einer URL ab und verarbeitet sie mit einem KI-Modell.

<h3 id="websearch">
  WebSearch
</h3>

**Tool-Name:** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

Durchsucht das Web und gibt formatierte Ergebnisse zurück.

<h3 id="workflow">
  Workflow
</h3>

**Tool-Name:** `Workflow`

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

Führt einen [dynamischen Workflow](/docs/de/workflows) aus: ein Skript, das viele Subagenten im Hintergrund orchestriert und ein konsolidiertes Ergebnis zurückgibt. Das `Workflow`-Tool ist in Agent SDK v0.3.149 und später verfügbar. Mindestens eines von `script`, `name` oder `scriptPath` ist erforderlich.

| Feld              | Typ       | Beschreibung                                                                                                                                                                                                                                                                                                                                                  |
| ----------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Inline-Workflow-Skript. Muss mit `export const meta = { name, description }` als Literal beginnen, gefolgt vom Skript-Body mit `agent()`, `parallel()`, `pipeline()` und `phase()`. Ein optionales `phases`-Array in `meta` gruppiert Agenten unter benannten Phasen in der Fortschrittsansicht                                                               |
| `name`            | `string`  | Name eines integrierten Workflows oder eines in `.claude/workflows/` gespeicherten. Wird zu einem Skript aufgelöst                                                                                                                                                                                                                                            |
| `scriptPath`      | `string`  | Pfad zu einer Workflow-Skriptdatei auf der Festplatte. Hat Vorrang vor `script` und `name`. Claude Code speichert jede Invokation und gibt den Pfad im Ergebnis zurück, sodass Sie diese Datei bearbeiten und erneut mit demselben `scriptPath` aufrufen können, um zu iterieren                                                                              |
| `args`            | `unknown` | Eingabewert, der dem Skript als globales `args` verfügbar gemacht wird, für parametrisierte benannte Workflows wie eine Forschungsfrage oder eine Liste von Dateipfaden. Übergeben Sie Arrays und Objekte als tatsächliche JSON-Werte, nicht als JSON-codierte Zeichenkette                                                                                   |
| `resumeFromRunId` | `string`  | Run-ID eines vorherigen `Workflow`-Aufrufs zum Fortsetzen. Abgeschlossene `agent()`-Aufrufe mit unveränderten Eingaben geben zwischengespeicherte Ergebnisse zurück; der Rest wird live ausgeführt. [Nach einer Pause fortsetzen](/docs/de/workflows#resume-after-a-pause) behandelt, welche abgeschlossenen Aufrufe erneut ausgeführt werden. Nur gleiche Sitzung |
| `title`           | `string`  | Ignoriert; der `meta`-Block des Skripts legt den Titel fest                                                                                                                                                                                                                                                                                                   |
| `description`     | `string`  | Ignoriert; der `meta`-Block des Skripts legt die Beschreibung fest                                                                                                                                                                                                                                                                                            |

<h3 id="todowrite">
  TodoWrite
</h3>

**Tool-Name:** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Erstellt und verwaltet eine strukturierte Aufgabenliste zum Verfolgen des Fortschritts.

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Siehe [Modellverfügbarkeit](/docs/de/agent-sdk/todo-tracking#model-availability), um sich anzumelden.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**Tool-Name:** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

Erstellt eine einzelne Aufgabe und gibt ihre zugewiesene ID zurück.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Tool-Name:** `TaskUpdate`

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

Patcht eine Aufgabe nach ID. Setzen Sie `status` auf `"deleted"`, um sie zu entfernen.

<h3 id="taskget">
  TaskGet
</h3>

**Tool-Name:** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

Gibt vollständige Details für eine Aufgabe zurück oder `null`, wenn die ID nicht gefunden wird.

<h3 id="tasklist">
  TaskList
</h3>

**Tool-Name:** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

Gibt einen Snapshot aller Aufgaben in der aktuellen Liste zurück.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Tool-Name:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Veraltet: wird nicht mehr verwendet. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
  [k: string]: unknown;
};
```

Beendet den Planungsmodus. Das Feld `allowedPrompts` ist veraltet und wird ignoriert; Claude Code akzeptiert es immer noch, damit vorhandene Aufrufer und Transkripte validiert werden. Vor v2.1.205 forderte es eingabeaufforderungsbasierte Bash-Berechtigungen zur Implementierung des Plans an.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Tool-Name:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Listet verfügbare MCP-Ressourcen von verbundenen Servern auf.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Tool-Name:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

Liest eine bestimmte MCP-Ressource von einem Server.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**Tool-Name:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

Erstellt und betritt einen temporären Git-Worktree für isolierte Arbeit. Übergeben Sie `path`, um stattdessen in einen vorhandenen Worktree zu wechseln. Beim ersten Eintritt muss das Ziel ein registrierter Worktree des aktuellen Repositorys sein oder, in einem Multi-Repo-Workspace, eines Repositorys, das darin verschachtelt ist; von innerhalb einer Worktree-Sitzung muss es unter `.claude/worktrees/` des Repositorys der Sitzung sein. `name` und `path` schließen sich gegenseitig aus.

<h3 id="exitworktree">
  ExitWorktree
</h3>

**Tool-Name:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

Beendet den aktuellen Git-Worktree und kehrt zum ursprünglichen Arbeitsverzeichnis zurück. Die `keep`-Aktion lässt den Worktree und den Branch auf der Festplatte, während `remove` beide löscht. `discard_changes` muss `true` sein, wenn ein Worktree mit nicht committeten Dateien oder nicht zusammengeführten Commits entfernt wird.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**Tool-Name:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

Betritt den Planungsmodus, in dem Claude einen Plan recherchiert und präsentiert, bevor Änderungen vorgenommen werden.

<h3 id="croncreate">
  CronCreate
</h3>

**Tool-Name:** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

Plant eine Eingabeaufforderung so ein, dass sie nach einem 5-Feld-Cron-Plan in Ortszeit ausgeführt wird. Setzen Sie `recurring` auf `false`, um einmalig beim nächsten Match zu starten. Jobs sind standardmäßig sitzungsbezogen: Das Starten eines neuen Gesprächs löscht sie, und das Fortsetzen mit `--resume` oder `--continue` stellt Jobs wieder her, die nicht abgelaufen sind. Siehe [Geplante Aufgaben](/docs/de/scheduled-tasks).

Das Setzen von `durable` auf `true` fordert Persistenz zu `.claude/scheduled_tasks.json` an, damit der Job Neustarts überlebt. Dauerhafte Planung ist nicht in jeder Sitzung verfügbar: Wenn nicht, akzeptiert Claude Code `durable: true`, erstellt aber den Job nur für die Sitzung. Lesen Sie das `durable`-Feld der Ausgabe, um zu sehen, ob der Job persistiert wurde.

<h3 id="crondelete">
  CronDelete
</h3>

**Tool-Name:** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

Löscht einen geplanten Cron-Job nach der ID, die von `CronCreate` zurückgegeben wurde.

<h3 id="cronlist">
  CronList
</h3>

**Tool-Name:** `CronList`

```typescript theme={null}
type CronListInput = {};
```

Listet die geplanten Cron-Jobs auf: dauerhafte Jobs aus `.claude/scheduled_tasks.json` und sitzungsbezogene Jobs aus der aktuellen Sitzung.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**Tool-Name:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

Plant ein einmaliges Aufwachen, das die angegebene Eingabeaufforderung nach einer Verzögerung auslöst. Dieses Tool unterstützt den selbstgesteuerten `/loop`-Befehl. Die Laufzeit begrenzt `delaySeconds` auf zwischen 60 und 3600 Sekunden. Die Felder `delaySeconds`, `reason`, `prompt` und `noop` sind erforderlich, es sei denn, `stop` ist true. `noop: true` meldet ein Aufwachen, bei dem sich nichts geändert hat. Das Setzen von `stop: true` bricht das ausstehende Aufwachen ab und beendet den selbstgesteuerten `/loop`. Das Feld `stop` erfordert Claude Code v2.1.202 oder später. Siehe die [ScheduleWakeup-Zeile in der Tools-Referenz](/docs/de/tools-reference).

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**Tool-Name:** `RemoteTrigger`

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

Verwaltet [Routinen](/docs/de/routines), die geplanten und ausgelösten Claude Code-Läufe, die in der Cloud gehostet werden. Dieses Tool unterstützt den `/schedule`-Befehl. `trigger_id` ist erforderlich für die Aktionen `get`, `update`, `run` und `list_runs`. `body` ist erforderlich für `create`, `update` und `create_webhook_trigger` und optional für `run`.

`create_webhook_trigger` fügt eine Ereignisquelle an eine vorhandene Routine an, wie z. B. ein [GitHub-Ereignis](/docs/de/routines#add-a-github-trigger), das sie auslöst. Der `body` benennt die Quelle, die Ereignisse und die auszulösende Routine. Erfordert Claude Code v2.1.225 oder später.

`list_runs` listet die letzten Läufe einer Routine auf, und `get_run_log` liest das Protokoll eines Laufs. `session_id` benennt den zu lesenden Lauf aus einem `list_runs`-Ergebnis, und `cursor` blättert durch die Ergebnisse beider Aktionen. Beide Aktionen erfordern Claude Code v2.1.227 oder später.

Dieses Tool ist nur verfügbar, wenn die Sitzung mit einem claude.ai-Konto auf einem Plan mit aktivierten Routinen authentifiziert ist, und fehlt, wenn die Richtlinie Ihrer Organisation [Claude Code im Web](/docs/de/claude-code-on-the-web) deaktiviert. Ab Claude Code v2.1.227 fehlt das Tool auch, wenn ein Besitzer [Routinen für die Organisation deaktiviert hat](/docs/de/routines#routines-are-disabled-by-your-organizations-policy). Vor v2.1.227 zeigte eine Sitzung mit nur dem Routinen-Toggle deaktiviert immer noch das Tool an, und der Server verweigerte seine Aufrufe.

<h3 id="pushnotification">
  PushNotification
</h3>

**Tool-Name:** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

Sendet eine proaktive Push-Benachrichtigung an den Benutzer. Halten Sie `message` unter 200 Zeichen, da mobile Betriebssysteme längeren Text abschneiden. Siehe die [PushNotification-Zeile in der Tools-Referenz](/docs/de/tools-reference) für Anbieter-Verfügbarkeit; Push-Zustellung läuft durch von Anthropic gehostete Infrastruktur, auf die nicht von Amazon Bedrock, Claude Platform auf AWS, Google Cloud's Agent Platform oder Microsoft Foundry zugegriffen werden kann.

<h3 id="repl">
  REPL
</h3>

Entfernt in v2.1.275. Bis v2.1.274 konnte ein experimentelles `REPL`-Tool mit `CLAUDE_CODE_REPL=1` in der [`env`-Option](#options) aktiviert werden.

<h3 id="reportfindings">
  ReportFindings
</h3>

**Tool-Name:** `ReportFindings`

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

Meldet Code-Review-Ergebnisse als strukturierte Liste, damit Claude Code sie rendern kann, anstatt sie als Text auszudrucken. `level` ist die Aufwandsstufe, auf der die Überprüfung ausgeführt wurde. Ergebnisse werden nach Schweregrad geordnet, mit höchstens 32 pro Aufruf, und das Array ist leer, wenn keine überlebt haben. Erfordert Claude Code v2.1.196 oder später.

Jedes Ergebnis enthält diese Felder:

* `file`: Repo-relativer Pfad, in dem sich das Ergebnis befindet. Die optionale `line` ist die 1-indizierte Zeile, auf die es verweist.
* `summary`: Ein-Satz-Aussage des Defekts. `failure_scenario` beschreibt die konkreten Eingaben und den Zustand, die zu der falschen Ausgabe oder dem Absturz führen.
* `short_summary`: optionale komprimierte Bezeichnung von höchstens 60 Zeichen für kompakte Anzeige. Erfordert Claude Code v2.1.212 oder später.
* `category`: optionaler kurzer Kebab-Case-Slug des Ergebnistyps, wie `correctness` oder `test-coverage`. Erfordert Claude Code v2.1.199 oder später.
* `verdict`: wird gesetzt, wenn ein Verify-Pass ausgeführt wurde; fehlt bei Inline-Only-Reviews.
* `outcome`: wird nur gesetzt, wenn nach dem Anwenden von Fixes erneut gemeldet wird.

<h3 id="artifact">
  Artifact
</h3>

**Tool-Name:** `Artifact`

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

Veröffentlicht eine lokale `.html`- oder `.md`-Datei als gehostete Artifact-Seite oder listet die veröffentlichten Artifacts des Benutzers auf. Lassen Sie `action` weg oder übergeben Sie `"publish"`, um `file_path` zu veröffentlichen, das für die Publish-Aktion erforderlich ist. Jedes Feld unten gilt für eine Veröffentlichung:

* `icon`: ein kurzes generisches Wort für das Browser-Tab-Symbol des Artifacts, wie `chart` oder `map`. Claude fügt es bei einer ersten Veröffentlichung ein und lässt es bei einer Aktualisierung weg, was das gespeicherte Symbol des Artifacts beibehält.
* `favicon`: veraltet, und Claude lässt es weg.
* `title`: benennt die veröffentlichte Seite im Browser-Tab und in der Galerie, wenn die HTML-Datei kein `<title>`-Tag hat.
* `url`: zielt auf ein vorhandenes Artifact ab, um es an Ort und Stelle zu aktualisieren, anstatt ein neues zu erstellen.

`force` ist ein letzter Ausweg zum Überschreiben, der eine neuere Version verwirft, die eine andere Sitzung veröffentlicht hat. Bei einem Konflikt gibt die fehlgeschlagene Veröffentlichung den neueren Inhalt zurück; Claude führt seine Änderungen in diesen Inhalt ein oder liest das Artifact erneut und veröffentlicht erneut. Übergeben Sie `force` nur, wenn der Benutzer explizit darum bittet, diese Version zu verwerfen.

Übergeben Sie `"list"`, um die veröffentlichten Artifacts des Benutzers aufzuzählen; nur `limit` und `scope` dürfen es begleiten. `scope` ist standardmäßig `"mine"`, das Artifacts auflistet, die der Benutzer besitzt; `"shared"` listet Artifacts auf, die andere Personen mit dem Benutzer geteilt haben, und `"all"` listet beide auf.

* `capabilities`: die Laufzeit-Funktionen, die die veröffentlichte Seite verwendet, nach Funktionsnamen verschlüsselt, wie die [Konnektoren, die die Seite aufrufen kann](/docs/de/artifacts#pull-live-data-with-mcp-connectors). Der Artifact-Service validiert die Deklaration und lehnt eine Veröffentlichung ab, die eine Funktionalität benennt, die das Konto nicht verwenden kann, oder eine ungültige Konfiguration gibt. Übergeben Sie `{}`, um eine gespeicherte Deklaration zu löschen, und lassen Sie das Feld bei einer erneuten Bereitstellung weg, um es zu behalten. Erfordert Agent SDK v0.3.235 oder später.
* `contract`: die Laufzeit-Version, gegen die die veröffentlichte Seite ausgeführt wird. Lassen Sie es weg, um die aktuelle Version des Artifacts zu behalten, übergeben Sie `"latest"`, um zu aktualisieren, oder übergeben Sie eine bestimmte Version, um zu fixieren oder zurückzurollen. Erfordert Agent SDK v0.3.235 oder später.

Die Typen werden exportiert, aber das Tool ist in Agent SDK-Sitzungen standardmäßig ausgeschaltet. Die Veröffentlichung erfordert auch jede Bedingung in der [Artifacts-Verfügbarkeitstabelle](/docs/de/artifacts#availability), die Sitzungen, die mit einem API-Schlüssel authentifiziert sind, nicht erfüllen.

<h3 id="projects">
  Projects
</h3>

**Tool-Name:** `Projects`

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

Liest und schreibt das claude.ai-Projekt, das an die Sitzung angehängt ist. Verteilt auf `method`:

* `project_info`: gibt Projekt-Metadaten und die Dokumentliste zurück.
* `project_read`: liest ein Dokument nach `path`.
* `project_search`: fragt die Wissensdatenbank des Projekts mit `query` ab. `n` begrenzt die Treffer und ist standardmäßig 5.
* `project_write`: erstellt oder ersetzt ein Dokument unter `path` aus genau einem von `content`, das Inline-Text enthält, oder `local_path`, das eine Datei im Arbeitsverzeichnis benennt. `present_to_user: true` markiert das geschriebene Dokument als das Ergebnis, das der Benutzer sehen muss.
* `project_delete`: löscht ein Dokument nach `path`.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**Tool-Name:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

Listet die direkten Kinder einer Verzeichnisressource auf einem MCP-Server auf. Nur gegen einen Server verwendbar, der Unterstützung für Verzeichnisauflistung deklariert hat; die Auflistung ist nicht rekursiv. Verzeichnisauflistung ist nicht in jeder Sitzung aktiviert: Wenn sie ausgeschaltet ist, gibt der Aufruf eine leere `resources`-Liste zurück und das `error`-Feld meldet, dass Verzeichnisauflistung nicht aktiviert ist.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**Tool-Name:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

Fragt die Tool-Liste verbundener MCP-Server erneut ab und wendet alle Änderungen an. Die Typen werden exportiert, aber Claude Code registriert das Tool nur, wenn Sie `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` in der [`env`-Option](#options) setzen, und nur in Sitzungen mit mindestens einem MCP-Server. Erfordert Claude Code v2.1.211 oder später.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**Tool-Name:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Rendert eine anklickbare Rollen-Picker-Chip-Reihe während des Cowork-Onboardings, damit der Benutzer seine Rolle auswählen und ein passendes Plugin installieren kann. Nimmt keine Argumente; die Rollenliste wird vom Client definiert. Der Aufruf blockiert, bis der Benutzer antwortet.

<h3 id="mcpinput">
  McpInput
</h3>

**Tool-Name:** dynamische MCP-Tool-Namen der Form `mcp__<server>__<tool>`

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

MCP-Tool-Argumente sind ein offenes Objekt: Jeder Server definiert seine eigenen Parameter, daher setzt der Typ keine Einschränkungen auf Feldnamen oder Werte. Konsultieren Sie das eigene Tool-Schema des Servers für die Felder, die ein bestimmtes Tool akzeptiert.

<h2 id="tool-output-types">
  Tool-Ausgabetypen
</h2>

Dokumentation von Ausgabeschemas für alle integrierten Claude Code-Tools. Diese Typen werden aus `@anthropic-ai/claude-agent-sdk` exportiert und stellen die tatsächlichen Antwortdaten dar, die von jedem Tool zurückgegeben werden.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Union von Tool-Ausgabetypen, die aus `@anthropic-ai/claude-agent-sdk` exportiert werden; Mitglieder umfassen:

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

**Tool-Name:** `Agent`. Der frühere Name `Task` wird immer noch als Alias akzeptiert, und das `tools`-Array in der [`SDKSystemMessage`](#sdksystemmessage)-Initialisierungsnachricht listet dieses Tool derzeit als `Task` für Rückwärtskompatibilität auf.

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

Gibt das Ergebnis vom Subagenten zurück. Diskriminiert nach dem `status`-Feld: `"completed"` für abgeschlossene Aufgaben, `"async_launched"` für Hintergrund-Aufgaben und `"remote_launched"` für Aufgaben, die Claude Code an eine Cloud-Sitzung versendet hat, wobei `sessionUrl` auf diese Sitzung verweist und `taskId` sie identifiziert.

Bei der `completed`-Variante benennt `resolvedModel` das Modell, auf dem der Subagent gestartet wurde, das sich vom angeforderten `model`-Input unterscheiden kann, wenn [`availableModels`](/docs/de/model-config#restrict-model-selection) oder eine andere Überschreibung gilt. Dieses Feld erfordert Claude Code v2.1.174 oder später. Bei `async_launched` benennt es das Modell, das verwendet wird, wenn die Aufgabe in den Hintergrund wechselt.

`modelsUsed` listet die Modelle auf, die der Subagent verwendet hat, in der Reihenfolge. Das Feld ist nur vorhanden, wenn ein Mid-Run-Wechsel stattgefunden hat, und ein Modell erscheint erneut, wenn der Lauf zu ihm zurückgewechselt hat. Bei `async_launched` deckt die Liste die Modelle ab, die vor dem Hintergrund-Wechsel verwendet wurden. Sowohl `modelsUsed` als auch das Hintergrund-Verhalten von `resolvedModel` erfordern Claude Code v2.1.212 oder später.

Wenn Claude Code [den isolierten Worktree des Subagenten beibehalten hat](/docs/de/worktrees#isolate-subagents-with-worktrees), ist `worktreePath` im `completed`-Ergebnis der Ort, wo er zu finden ist. `worktreeBranch` ist sein Branch, vorhanden, wenn Claude Code den Worktree mit Git erstellt hat.

Claude Code füllt `usage` und `totalTokens` aus der letzten API-Anfrage des Subagenten, nicht aus dem gesamten Lauf, daher ist `usage.service_tier` die Service-Tier-Zeichenkette, die die API bei dieser Anfrage gemeldet hat. Wenn vorhanden, ist `usage.output_tokens_details.thinking_tokens` die Anzahl der Ausgabe-Token dieser Anfrage, die Denk-Token waren. Das Feld `output_tokens_details` erfordert TypeScript SDK v0.3.228 oder später, das Claude Code v2.1.228 bündelt.

`usage.output_tokens_details` entspricht [`Usage.output_tokens_details`](#usage) in der Bedeutung, begrenzt auf diese letzte Anfrage, aber jede Ebene davon ist optional hier. Schützen Sie sowohl das Objekt als auch das Feld, zum Beispiel `usage.output_tokens_details?.thinking_tokens ?? 0`, anstatt es direkt zu lesen.

Vor v2.1.207 war der veröffentlichte Typ enger. Er ließ `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount` und die Nutzungsfelder `inference_geo`, `speed` und `iterations` weg, und er typisierte `service_tier` als `"standard" | "priority" | "batch"`. Felder, die der Typ als optional markiert, können bei Ergebnissen fehlen, die von früheren Versionen aufgezeichnet wurden.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**Tool-Name:** `AskUserQuestion`

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

Gibt die gestellten Fragen und die Antworten des Benutzers zurück. `response` wird gesetzt, wenn der Benutzer eine freie Antwort eingegeben hat, anstatt die strukturierten Fragen zu beantworten; wenn vorhanden, erhält Claude „Der Benutzer hat geantwortet: …" anstelle der Pro-Frage-Antworteliste.

<h3 id="bash-2">
  Bash
</h3>

**Tool-Name:** `Bash`

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

Die Felder `stdout`, `stderr` und `backgroundTaskId` enthalten:

| Feld               | Was es enthält                                                                                                      |
| ------------------ | ------------------------------------------------------------------------------------------------------------------- |
| `stdout`           | Die Stdout und Stderr des Befehls, zusammengeführt in einen verschachtelten Stream                                  |
| `stderr`           | Hinweise, die das Tool selbst hinzufügt, wie z. B. ein Shell-Arbeitsverzeichnis-Reset, nicht die Stderr des Befehls |
| `backgroundTaskId` | Vorhanden für Hintergrund-Befehle                                                                                   |

`timedOutAfterMs` ist das Timeout in Millisekunden, gesetzt, wenn der Befehl sein Timeout erreicht hat und in den Hintergrund wechselt, anstatt dort explizit zu starten. `backgroundCwdHint` wird gesetzt, wenn der Hintergrund-Befehl ein Verzeichniswechsel-Builtin wie `cd`, `pushd`, `popd` oder `chdir` enthielt, und vermerkt, dass sich das Sitzungsarbeitsverzeichnis nicht geändert hat. Beide Felder erfordern Claude Code v2.1.210 oder später.

Wenn ein Subagent, der im Vordergrund läuft, einen Hintergrund-Befehl besitzt, beendet Claude Code den Befehl, wenn dieser Subagent seine endgültige Antwort gibt. Claude Code setzt `backgroundEndsWithFinalResponse` auf `true` bei solchen Befehlen und lässt das Feld weg, wenn der Befehl den Zug überlebt, wie Befehle, die vom Hauptgespräch oder von Hintergrund-Subagenten gestartet werden. Das Feld erfordert Claude Code v2.1.227 oder später.

Claude Code setzt `gitOperation.commit.branch` auf den Branch, der in Gits Commit-Zusammenfassungszeile benannt ist, und lässt ihn für einen Commit auf einem detached HEAD weg. Das Feld erfordert Agent SDK v0.3.227 oder später. Claude Code meldet einen `gh pr reopen`-Befehl als die `reopened`-PR-Aktion, was Agent SDK v0.3.234 oder später erfordert.

<h3 id="monitor-2">
  Monitor
</h3>

**Tool-Name:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Gibt die Hintergrund-Aufgaben-ID für den laufenden Monitor zurück. Verwenden Sie diese ID mit `TaskStop`, um die Überwachung früh zu stornieren.

<h3 id="edit-2">
  Edit
</h3>

**Tool-Name:** `Edit`

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

Gibt den strukturierten Diff der Bearbeitungsoperation zurück.

<h3 id="read-2">
  Read
</h3>

**Tool-Name:** `Read`

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
        /** True when a whole-file read was auto-paginated because it exceeded the token cap (the content is a partial first page). */
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
      /** Document page number of the first extracted page; labels the page images in the tool_result content. */
      firstPage?: number;
      /** In-process only: the page-image bytes are delivered as image blocks in the tool_result content and aren't retained on the emitted tool_use_result, so this key is absent there. */
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
      /** Set when the dedup matched a startup-seeded entry (CLAUDE.md / nested memory) rather than a prior Read tool_result. */
      source?: "seeded";
    };
```

Gibt Dateiinhalte in einem Format zurück, das für den Dateityp geeignet ist. Diskriminiert nach dem `type`-Feld.

<h3 id="write-2">
  Write
</h3>

**Tool-Name:** `Write`

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

Gibt das Schreib-Ergebnis mit strukturierten Diff-Informationen zurück. Was `originalFile` und `structuredPatch` enthalten, hängt vom Schreib-Vorgang ab:

* Für eine neu erstellte Datei ist `originalFile` null und `structuredPatch` ist leer
* Bei einem Überschreiben enthält `originalFile` den vorherigen Inhalt, außer wenn dieser Inhalt größer als etwa 10 MB ist: Claude Code überspringt dann den Diff und gibt `originalFile` null und `structuredPatch` leer zurück
* `structuredPatch` ist auch leer, wenn der Schreib-Vorgang nichts geändert hat oder das Diff-Timeout abgelaufen ist

<h3 id="glob-2">
  Glob
</h3>

**Tool-Name:** `Glob`

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

Gibt Dateipfade zurück, die dem Glob-Muster entsprechen, sortiert nach Änderungszeit.

`totalMatches` und `countIsComplete` erfordern Claude Code v2.1.191 oder später. `totalMatches` meldet die Anzahl der übereinstimmenden Dateien vor der Kürzung. Wenn `countIsComplete` false ist, ist `totalMatches` eine untere Grenze, da die zugrunde liegende Suche ihre eigene Ausgabe gekürzt hat.

<h3 id="grep-2">
  Grep
</h3>

**Tool-Name:** `Grep`

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

Gibt Suchergebnisse zurück. Die Form variiert je nach `mode`: Dateiliste, Inhalt mit Übereinstimmungen oder Übereinstimmungszahlen. Im `count`-Modus sind `numFiles` und `numMatches` Gesamtwerte über den vollständigen Ergebnissatz, nicht den paginierten Slice. Vor v2.1.208 kürzten ein `head_limit` oder `offset`, das die aufgelisteten Einträge kürzten, auch diese Gesamtwerte.

`totalFiles` erfordert Claude Code v2.1.208 oder später und meldet die Gesamtzahl der Ergebnisse vor `head_limit` und `offset`-Paginierung im `files_with_matches`-Modus. `totalLines` erfordert Claude Code v2.1.210 oder später und meldet die Gesamtzahl der Zeilen vor der Paginierung im `content`-Modus.

<h3 id="taskstop-2">
  TaskStop
</h3>

**Tool-Name:** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

Gibt Bestätigung nach dem Stoppen der Hintergrund-Aufgabe zurück.

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**Tool-Name:** `NotebookEdit`

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

Gibt das Ergebnis der Notebook-Bearbeitung mit ursprünglichen und aktualisierten Dateiinhalten zurück.

<h3 id="webfetch-2">
  WebFetch
</h3>

**Tool-Name:** `WebFetch`

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

Gibt den abgerufenen Inhalt mit HTTP-Status und Metadaten zurück.

`artifactRead` ist Claude Codes eigener Datensatz eines Artefakt-Lesevorgangs, vorhanden nur, wenn Claude ein Artefakt abgerufen hat, das die Sitzung veröffentlichen kann. Claude Code liest es zurück, wenn eine Sitzung fortgesetzt wird, damit eine spätere Veröffentlichung auf der richtigen Version aufbaut; Ihr Code muss nicht darauf reagieren. `slug` benennt das Artefakt, `ver` ist die Version, die der Lesevorgang aufgezeichnet hat, und ist abwesend, wenn er keine aufgezeichnet hat, und `seeded: false` markiert einen Lesevorgang, dessen vollständige Quelle Claude nicht erreichte. Das Feld `seeded` erfordert Agent SDK v0.3.239 oder später.

<h3 id="websearch-2">
  WebSearch
</h3>

**Tool-Name:** `WebSearch`

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

Gibt Suchergebnisse aus dem Web zurück.

<h3 id="workflow-2">
  Workflow
</h3>

**Tool-Name:** `Workflow`

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
  sessionUrl?: string; // set when the workflow launched as a cloud session
  warning?: string;
  error?: string;
};
```

Gibt sofort nach dem Akzeptieren der Invokation durch das Tool zurück. Das endgültige Ergebnis kommt später als Aufgabenvollendung an. Überprüfen Sie `error`, bevor Sie den Lauf als gestartet behandeln: Ein Skript, das seine Syntaxprüfung nicht besteht, gibt `status: "async_launched"` mit gesetztem `error` zurück und wird nie ausgeführt.

| Feld            | Typ                                     | Beschreibung                                                                                                                                                                              |
| --------------- | --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | Das Tool hat die Invokation akzeptiert. `"async_launched"` für In-Process-Läufe, `"remote_launched"` für Läufe, die an eine Cloud-Sitzung versendet werden, anstatt In-Process zu laufen  |
| `taskId`        | `string`                                | Hintergrund-Aufgabenkennung für den Lauf                                                                                                                                                  |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | Aufgabentyp der registrierten Hintergrund-Aufgabe, entsprechend dem `status`-Arm                                                                                                          |
| `workflowName`  | `string`                                | Der `meta.name` aus dem Workflow-Skript                                                                                                                                                   |
| `runId`         | `string`                                | Workflow-Lauf-Kennung, die als `resumeFromRunId` bei einer späteren Invokation übergeben werden soll. Fehlt bei `remote_launched`-Läufen, wo die Cloud-Sitzungs-URL das Resume-Handle ist |
| `summary`       | `string`                                | Einzeilige Beschreibung, was der Workflow tut                                                                                                                                             |
| `transcriptDir` | `string`                                | Verzeichnis, in dem Subagenten-Transkripte während der Ausführung geschrieben werden                                                                                                      |
| `scriptPath`    | `string`                                | Pfad zum persistierten Workflow-Skript für diesen Lauf. Bearbeiten Sie es und übergeben Sie es als `scriptPath`, um es erneut auszuführen, ohne das Skript erneut zu senden               |
| `sessionUrl`    | `string`                                | Cloud-Sitzungs-URL, gesetzt, wenn `status` `"remote_launched"` ist                                                                                                                        |
| `warning`       | `string`                                | Nicht blockierender Hinweis, wie z. B. lokaler Git-Status, der vom gepushten Branch abweicht, den eine Cloud-Sitzung klonen wird                                                          |
| `error`         | `string`                                | Wird gesetzt, wenn das Skript seine Syntaxprüfung nicht besteht. Wenn vorhanden, wurde der Lauf trotz des gestarteten Status nicht gestartet                                              |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**Tool-Name:** `TodoWrite`

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

Gibt die vorherigen und aktualisierten Aufgabenlisten zurück.

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Siehe [Modellverfügbarkeit](/docs/de/agent-sdk/todo-tracking#model-availability), um sich anzumelden.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**Tool-Name:** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

Gibt die erstellte Aufgabe mit ihrer zugewiesenen ID zurück.

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**Tool-Name:** `TaskUpdate`

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

Gibt das Aktualisierungsergebnis zurück, einschließlich welche Felder sich geändert haben.

<h3 id="taskget-2">
  TaskGet
</h3>

**Tool-Name:** `TaskGet`

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

Gibt den vollständigen Aufgabendatensatz zurück oder `null`, wenn die ID nicht gefunden wird.

<h3 id="tasklist-2">
  TaskList
</h3>

**Tool-Name:** `TaskList`

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

Gibt einen Snapshot aller Aufgaben in der aktuellen Liste zurück.

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**Tool-Name:** `ExitPlanMode`

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

Gibt den Planzustand nach dem Beenden des Planungsmodus zurück.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**Tool-Name:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Gibt ein Array verfügbarer MCP-Ressourcen zurück.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Tool-Name:** `ReadMcpResourceTool`

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

Gibt die Inhalte der angeforderten MCP-Ressource zurück.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**Tool-Name:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Gibt Informationen über den Git-Worktree zurück.

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**Tool-Name:** `ExitWorktree`

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

Gibt die durchgeführte Aktion und Details über den Worktree zurück, der beendet wurde.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**Tool-Name:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

Gibt eine Bestätigung zurück, dass der Planungsmodus betreten wurde.

<h3 id="croncreate-2">
  CronCreate
</h3>

**Tool-Name:** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true when persisted to .claude/scheduled_tasks.json; false when session-only
};
```

Gibt die Job-ID und eine menschenlesbare Beschreibung des Zeitplans zurück.

<h3 id="crondelete-2">
  CronDelete
</h3>

**Tool-Name:** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

Gibt die ID des gelöschten Jobs zurück.

<h3 id="cronlist-2">
  CronList
</h3>

**Tool-Name:** `CronList`

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

Gibt die geplanten Cron-Jobs zurück: dauerhafte Jobs aus `.claude/scheduled_tasks.json` und Sitzungs-Only-Jobs aus der aktuellen Sitzung. Ein Sitzungs-Only-Job enthält `durable: false`; Jobs, die von der Festplatte gelesen werden, lassen das Feld weg.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**Tool-Name:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

Gibt zurück, wann das Aufwachen als Epoch-Millisekunden-Zeitstempel auslöst, die tatsächlich verwendete Verzögerung und ob die angeforderte Verzögerung begrenzt wurde. Das Feld `stopped` ist `true`, wenn der Aufruf die Schleife mit `stop: true` beendet hat. Es erfordert Claude Code v2.1.202 oder später. Das Feld `cancelledWakeups` zählt, wie viele ausstehende Aufweckungen ein `stop: true`-Aufruf storniert hat. Ein Wert von 0 bedeutet, dass nichts ausstehend war, und ein wiederkehrendes `/loop`-Cron wird nicht durch `stop: true` storniert. Es erfordert Claude Code v2.1.206 oder später.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**Tool-Name:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

Gibt den API-Antwortstatus und -Text für die Trigger-Operation zurück.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**Tool-Name:** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

Gibt Zustellungsdetails zurück, einschließlich ob eine Push- oder lokale Benachrichtigung gesendet wurde und warum die Zustellung übersprungen wurde.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**Tool-Name:** `ReportFindings`

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

Gibt die Anzahl der gemeldeten Erkenntnisse, die Aufwandsstufe, auf der die Überprüfung lief, und die Erkenntnisse zurück, die für den Ergebnis-Text wiedergegeben werden. Erfordert Claude Code v2.1.196 oder später. Das wiedergegebene Feld `short_summary` erfordert Claude Code v2.1.212 oder später.

<h3 id="artifact-2">
  Artifact
</h3>

**Tool-Name:** `Artifact`

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

Gibt die `url` der veröffentlichten Seite und den lokalen `path` zurück, der für die Veröffentlichungsaktion veröffentlicht wurde, mit `updated` auf true gesetzt, wenn die Veröffentlichung ein vorhandenes Artefakt erneut bereitgestellt hat, und `warnings` mit allen Veröffentlichungs-Zeit-Hinweisen. Die Listenaktion gibt stattdessen die `artifacts`-Zeilen zurück, mit `truncated` gesetzt, wenn mehr Artefakte vorhanden sind als das angeforderte Limit. Bei Auflistungen, deren Umfang nicht `"mine"` ist, enthält jede Zeile `rel`, das markiert, ob der Benutzer das Artefakt besitzt oder es mit ihm geteilt wurde, und die Ausgabe des `scope` zeichnet auf, welcher Nicht-Standard-Umfang die Auflistung erzeugt hat; beide fehlen bei Standard-Auflistungen.

<h3 id="projects-2">
  Projects
</h3>

**Tool-Name:** `Projects`

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

Diskriminiert nach dem `method`-Feld, spiegelt die Eingabe wider. `project_read` gibt kleine Text-Dokumente inline in `content` zurück und schreibt größere Dokumente stattdessen in einen `local_file`-Pfad; `project_search` gibt RAG-`hits` mit `rag: true` zurück, wenn der Projektindex verfügbar ist, und fällt ansonsten auf eine `docs`-Pfadliste zurück.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**Tool-Name:** `ReadMcpResourceDirTool`

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

Gibt die direkten Kinder der Verzeichnisressource zurück. Unterverzeichnisse erscheinen mit mimeType `"inode/directory"`; `error` enthält eine menschenlesbare Nachricht, wenn der Server das Verzeichnis nicht auflisten konnte.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**Tool-Name:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // tools now available from this server
  added?: string[]; // tool names this refresh added
  removed?: string[]; // tool names this refresh removed
  error?: string; // why the refresh failed or the server was unavailable
}>;
```

Gibt einen Eintrag pro Server zurück: `refreshed` bedeutet, dass die neu abgefragte Werkzeugliste angewendet wurde, `error` bedeutet, dass die Neuabfrage fehlgeschlagen ist und die vorherige Werkzeugliste beibehalten wurde, und `not_connected` bedeutet, dass der Server keine Live-Verbindung zum Abfragen hat.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**Tool-Name:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

Gibt die Auswahl des Benutzers zurück: `role`, wenn er einen Rollen-Chip ausgewählt oder eingegeben hat, und `dismissed: true`, wenn er die Auswahl geschlossen hat. Ein leeres Objekt bedeutet, dass der Benutzer den Aufruf genehmigt hat, ohne eine Rolle auszuwählen.

<h3 id="mcpoutput">
  McpOutput
</h3>

**Tool-Name:** dynamische MCP-Tool-Namen der Form `mcp__<server>__<tool>`

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

MCP-Tool-Ergebnisse werden je nach Server als Zeichenkette oder als Array von Inhaltsblöcken zurückgegeben. Der nachfolgende Plain-Object-Zweig im exportierten Typ ist ein Schema-Generierungs-Artefakt: Das SDK gibt kein bloßes Objekt zurück, da die strukturierte Ausgabe eines Servers vor der Rückgabe in einen JSON-String serialisiert wird. Zur Laufzeit kann der Wert auch `undefined` sein, obwohl der exportierte Typ dies nicht modelliert.

<h2 id="permission-types">
  Berechtigungstypen
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Operationen zum Aktualisieren von Berechtigungen.

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
  | "userSettings" // Globale Benutzereinstellungen
  | "projectSettings" // Pro-Verzeichnis-Projekteinstellungen
  | "localSettings" // Lokale Projekteinstellungen
  | "session" // Nur aktuelle Sitzung
  | "cliArg"; // CLI-Argument
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
  Andere Typen
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

Woher der API-Schlüssel für die Anfragen der Sitzung kam, gemeldet als `apiKeySource` in der [`SDKSystemMessage`](#sdksystemmessage)-Initialisierungsnachricht.

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

Claude Code meldet einen von vier Werten:

| Wert                 | Verwendeter Schlüssel                                                                                                                                  |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_API_KEY`  | Der Schlüssel in der `ANTHROPIC_API_KEY`-Umgebungsvariable                                                                                             |
| `apiKeyHelper`       | Der Schlüssel, der von Ihrem [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Befehl zurückgegeben wird                                           |
| `/login managed key` | Der Schlüssel, den Claude Code speicherte, als Sie sich mit einem [Claude-Konsolen-Konto](/docs/de/authentication#claude-console-authentication) anmeldeten |
| `none`               | Kein API-Schlüssel. Die Sitzung authentifiziert sich auf andere Weise, z. B. über eine claude.ai-Anmeldung, ein Bearer-Token oder einen Cloud-Provider |

Agent SDK v0.3.234 und später listen diese vier Werte im Typ auf. Der Typ behält auch `user`, `project`, `org`, `temporary` und `oauth`, damit älterer Code noch kompiliert wird, und Claude Code meldet sie nicht.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Verfügbare Beta-Funktionen, die über die `betas`-Option aktiviert werden können. Siehe [Beta-Header](https://platform.claude.com/docs/en/api/beta-headers) für weitere Informationen.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  Das `context-1m-2025-08-07`-Beta ist ab dem 30. April 2026 veraltet. Das Übergeben dieses Wertes mit Claude Sonnet 4.5 oder Sonnet 4 hat keine Auswirkung, und Anfragen, die das Standard-200k-Token-Kontextfenster überschreiten, geben einen Fehler zurück. Um ein 1M-Token-Kontextfenster zu verwenden, migrieren Sie zu [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 oder Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), die 1M-Kontext zu Standardpreisen ohne Beta-Header enthalten.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informationen über einen verfügbaren Befehl.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` ist `true` in einer Zeile, wenn der Befehl Claude Code's eigener ist und das Eingeben von `/name` ihn ausführt. Er fehlt für einen Befehl, der von einem Benutzer, Projekt, Plugin oder MCP-Server definiert wird, und für einen gebündelten Befehl, den einer dieser [nach Name ersetzt](/docs/de/skills#resolve-skills-that-share-a-name). Erfordert Agent SDK v0.3.277 oder später.

<h3 id="modelinfo">
  `ModelInfo`
</h3>

Informationen über ein verfügbares Modell.

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

| Feld                       | Typ                                                                | Beschreibung                                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | Modell-Identifikator, der in API-Aufrufen übergeben werden soll                                                                                                                                                                                                                                                                        |
| `resolvedModel`            | `string \| undefined`                                              | Kanonische Wire-Modell-ID, in die sich der `value` dieses Eintrags auflöst. Ein Alias-Eintrag wie `sonnet` wird in eine explizite Modell-ID wie `claude-sonnet-5` aufgelöst, sodass ein Host eine gespeicherte explizite Modell-ID mit dem Alias-Eintrag abgleichen kann, der sie abdeckt. Erfordert Claude Code v2.1.197 oder später. |
| `displayName`              | `string`                                                           | Benutzerfreundlicher Anzeigename                                                                                                                                                                                                                                                                                                       |
| `description`              | `string`                                                           | Beschreibung der Modell-Fähigkeiten                                                                                                                                                                                                                                                                                                    |
| `supportsEffort`           | `boolean \| undefined`                                             | Ob dieses Modell Anstrengungsstufen unterstützt                                                                                                                                                                                                                                                                                        |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Anstrengungsstufen, die dieses Modell akzeptiert                                                                                                                                                                                                                                                                                       |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Ob dieses Modell adaptives Denken unterstützt, bei dem Claude entscheidet, wann und wie viel zu denken ist                                                                                                                                                                                                                             |
| `supportsFastMode`         | `boolean \| undefined`                                             | Ob dieses Modell den Schnellmodus unterstützt                                                                                                                                                                                                                                                                                          |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Ob dieses Modell den Auto-Modus unterstützt                                                                                                                                                                                                                                                                                            |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Informationen über einen verfügbaren Subagenten, der über das Agent-Tool aufgerufen werden kann.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| Feld          | Typ                   | Beschreibung                                                                                                                                                                                                                          |
| :------------ | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | `string`              | Agent-Typ-Identifikator (z. B. `"Explore"`, `"general-purpose"`)                                                                                                                                                                      |
| `description` | `string`              | Beschreibung, wann dieser Agent verwendet werden soll                                                                                                                                                                                 |
| `model`       | `string \| undefined` | Modell, das dieser Agent verwendet: ein Alias oder eine Modell-ID, oder `'inherit'` für das übergeordnete Modell. Wenn `undefined`, wählt Claude Code das Modell in der [Subagenten-Modellreihenfolge](/docs/de/sub-agents#choose-a-model) |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

Der MCP-Server, der ein `mcp__*`-Tool bereitstellt, und woher die Definition dieses Servers kam. Die [`PreToolUse`](#pretoolusehookinput)-, `PostToolUse`-, `PostToolUseFailure`-, `PermissionRequest`- und `PermissionDenied`-Hook-Eingaben tragen ihn als `mcp_server`, und die [`CanUseTool`](#canusetool)-Optionen tragen ihn als `mcpServer`. Beide lassen ihn für Tools weg, die nicht von einem MCP-Server stammen.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| Feld     | Typ      | Beschreibung                                                                                                              |
| :------- | :------- | :------------------------------------------------------------------------------------------------------------------------ |
| `name`   | `string` | Der Name, unter dem der Server registriert ist, der gleiche Wert, den [`mcpServerStatus()`](#query-object) für ihn meldet |
| `source` | `string` | Woher die Definition des Servers kam: `sdk`, `plugin` oder ein Konfigurationsbereich                                      |

`source` nimmt einen der folgenden Werte an. Die Menge ist offen, daher behandeln Sie einen Wert, den Sie nicht erkennen, als konfigurierte Quelle, niemals als `sdk`:

* **`sdk`**: ein In-Process-Server, den Ihre Anwendung registriert hat. Nur die SDK-Host-Anwendung kann einen registrieren, daher meldet ein konfigurierter Server niemals `sdk`, unabhängig von seinem Namen.
* **`plugin`**: ein Server, den ein [Plugin](/docs/de/agent-sdk/plugins) bereitstellt. Sein `name` ist die scoped `plugin:<plugin-name>:<server-name>`-Form, die unter [Plugin-bereitgestellte MCP-Server](/docs/de/mcp#plugin-provided-mcp-servers) beschrieben wird.
* **Ein Konfigurationsbereich**: `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai` oder `agent`. Ein `.mcp.json`-Server meldet `project`, und [MCP-Installationsbereiche](/docs/de/mcp#mcp-installation-scopes) definiert `local`, `project` und `user`. Server, die Ihre Anwendung in der [`mcpServers`-Option](#options) übergibt, außer In-Process-SDK-Servern, melden `dynamic`.

Basieren Sie Vertrauensentscheidungen auf `source`, nicht auf `name` oder dem `mcp__<server>__`-Tool-Namen-Präfix. Für jede Quelle außer `sdk` ist `name` nicht vertrauenswürdiger Text: maskieren Sie ihn vor der Anzeige.

`McpServerProvenance` und die Felder, die ihn tragen, erfordern Agent SDK v0.3.274 oder später.

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Status eines verbundenen MCP-Servers.

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

`source` sagt, woher die Definition des Servers kam, mit den gleichen Werten und Vertrauensregel wie [`McpServerProvenance`](#mcpserverprovenance)'s `source`. Das Feld erfordert Agent SDK v0.3.274 oder später und fehlt in früheren Versionen.

`_meta` auf einem `tools`-Eintrag trägt die MCP-Apps-Mitglieder des `_meta` dieses Tools, sodass Ihre Anwendung die `ui://`-Ressource finden kann, um sie mit [`readMcpResource()`](#query-object) zu rendern. Claude Code leitet das `ui`-Objekt und die veraltete flache `ui/resourceUri`-Zeichenkette durch und behält jeden anderen Schlüssel zurück. Innerhalb von `ui` ist `resourceUri` eine `ui://`-Zeichenkette und `visibility` ein Array von `"model"` und `"app"`, wenn der Server sie setzt, und jedes andere Mitglied wird unverändert durchgeleitet. Claude Code verwirft jeden Schlüssel, wenn der Wert fehlerhaft ist, und lässt `_meta` von einem Tool weg, das weder deklariert. Das Feld ist nur vorhanden, wenn die [`capabilities`](#sdksystemmessage) der Init-Nachricht `mcp_tool_ui_meta_v1` enthalten, und erfordert TypeScript Agent SDK v0.3.280 oder später.

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

Die Konfiguration eines MCP-Servers, wie von `mcpServerStatus()` gemeldet. Dies ist die Union aller MCP-Server-Transporttypen.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

Siehe [`McpServerConfig`](#mcpserverconfig) für Details zu jedem Transporttyp.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

Kontoinformationen für den authentifizierten Benutzer.

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

Pro-Modell-Nutzungsstatistiken, die in Ergebnis-Nachrichten zurückgegeben werden. Der `costUSD`-Wert ist eine clientseitige Schätzung. Siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking) für Abrechnungsvorbehalt.

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

`thinkingTokens` zählt die Denk-Token, die dieses Modell generiert hat. `outputTokens` enthält sie bereits, daher addieren Sie die beiden nicht zusammen. Das Feld fehlt, bis ein Turn auf einer Claude-Code-Version läuft, die es aufzeichnet, daher meldet eine fortgesetzte Sitzung, die auf einer früheren Version begann, eine teilweise Anzahl. `thinkingTokens` erfordert Agent SDK v0.3.257 oder später.

Die Felder `canonicalModel` und `provider` erfordern Claude Code v2.1.218 oder später. `canonicalModel` ist die kanonische Modell-ID, die die Preissuche verwendet; sie kann sich von der rohen Modell-Zeichenkette unterscheiden, die den Eintrag indiziert, z. B. wenn diese Zeichenkette eine anbieter-spezifische ID oder ein Alias ist.

`provider` benennt das API-Backend, das das Modell bereitgestellt hat, z. B. `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` oder `gateway`.

`costBasis` benennt die Preistabelle, die das Modell der letzten Anfrage bepreist hat: `list` für Listenpreis, `managed` für eine [`modelPricing`](/docs/de/settings-reference#modelpricing)-Tabelle, oder `unknown`, wenn keine die Modell-ID abglich. Das Feld erfordert Claude Code v2.1.246 oder später.

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Eine Version von [`Usage`](#usage) mit allen nullable Feldern, die nicht nullable gemacht werden.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Token-Nutzungsstatistiken. Dies ist der `BetaUsage`-Typ aus `@anthropic-ai/sdk`.

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

`BetaServerToolUsage`, `BetaIterationsUsage` und `BetaOutputTokensDetails` sind in `@anthropic-ai/sdk` definiert.

`output_tokens_details` unterteilt die abgerechnete Ausgabe nach Kategorie. Sie trägt derzeit ein Feld, `thinking_tokens: number`, das die Ausgabe-Token zählt, die das Modell als interne Überlegung generiert hat, einschließlich der Denk-Block-Trennzeichen. Das Feld `output_tokens_details` erfordert TypeScript SDK v0.3.228 oder später, das Claude Code v2.1.228 bündelt.

* **Abrechnung**: Lesen Sie die Aufschlüsselung zur Beobachtung, nicht zur Abrechnung. `output_tokens` bleibt die autoritative Summe, und `output_tokens - thinking_tokens` approximiert die Nicht-Reasoning-Ausgabe.
* **Was die Anzahl abdeckt**: das rohe Reasoning, das das Modell produziert hat, das länger sein kann als der Denk-Text, der im Antwortkörper zurückgegeben wird. Die API berechnet es durch erneutes Tokenisieren dieses rohen Textes, daher kann es sich um einige Token von der genauen Generierungsanzahl des Modells unterscheiden.
* **Streaming**: Bei gestreamten Assistenten-Nachrichten ist diese Aufschlüsselung wie `output_tokens` ein `message_start`-Platzhalter und trägt keine echte Anzahl, daher lesen Sie sie aus der Ergebnis-Nachricht `usage`, wie [Ausgabe-Token aus der Ergebnis-Nachricht lesen](/docs/de/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) beschreibt. In der Ergebnis-Nachricht liest `thinking_tokens` `0`, wenn das Modell oder der Provider keine Aufschlüsselung meldet.
* **`null`-Fälle**: `output_tokens_details` selbst ist `null` bei Assistenten-Nachrichten, die Claude Code synthetisiert, z. B. API-Fehler-Nachrichten.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

MCP-Tool-Ergebnistyp (aus `@modelcontextprotocol/sdk/types.js`). `structuredContent` ist ein JSON-Objekt, das zusammen mit `content` zurückgegeben werden kann, einschließlich Bildblöcke. Siehe [Strukturierte Daten zurückgeben](/docs/de/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Zusätzliche Felder variieren je nach Typ
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

Eine Datei, die ein MCP-Tool per Referenz zurückgegeben hat. Claude Code erstellt jeden Eintrag aus einem `resource_link`-Block im Ergebnis des Tools und liefert die Liste als `resourceLinks` auf [`SDKUserMessage.tool_use_result`](#sdkusermessage) oder als `resource_links` auf [`SDKTaskNotificationMessage`](#sdktasknotificationmessage), wenn der Aufruf im Hintergrund beendet wurde. Erfordert Agent SDK v0.3.257 oder später.

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

Claude Code verwirft einen Block, dessen `uri` oder `name` keine Zeichenkette ist, und lässt ein optionales Feld weg, dessen Wert nicht vom aufgelisteten Typ ist.

| Feld          | Typ                                    | Beschreibung                                                             |
| :------------ | :------------------------------------- | :----------------------------------------------------------------------- |
| `uri`         | `string`                               | URI der Ressource, wie der Server sie zurückgegeben hat                  |
| `name`        | `string`                               | Name, den der Server der Ressource gab                                   |
| `title`       | `string \| undefined`                  | Anzeige-Titel, wenn der Server einen gesetzt hat                         |
| `description` | `string \| undefined`                  | Beschreibung, wenn der Server eine gesetzt hat                           |
| `mimeType`    | `string \| undefined`                  | MIME-Typ, wenn der Server einen gesetzt hat                              |
| `size`        | `number \| undefined`                  | Größe in Bytes, wenn der Server eine gesetzt hat                         |
| `annotations` | `Record<string, unknown> \| undefined` | Das MCP-Annotations-Objekt des Blocks, wenn der Server eines gesetzt hat |

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Steuert das Denk-/Reasoning-Verhalten von Claude. Hat Vorrang vor dem veralteten `maxThinkingTokens`.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // Das Modell bestimmt, wann und wie viel zu denken ist (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Festes Denk-Token-Budget
  | { type: "disabled" }; // Kein erweitertes Denken
```

Das optionale `display`-Feld steuert, ob Denk-Text `"summarized"` oder `"omitted"` zurückgegeben wird. Bei Claude Opus 4.7 und später ist der API-Standard `"omitted"`, daher setzen Sie `"summarized"`, um Denk-Inhalte in `thinking`-Blöcken zu erhalten. Claude Code sendet `display` nicht an Amazon Bedrock oder Google Cloud's Agent Platform, daher geben Opus 4.7 und später auf diesen Anbietern leere `thinking`-Blöcke zurück, auch wenn Sie `display` auf `"summarized"` setzen.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

Schnittstelle für benutzerdefiniertes Process-Spawning (verwendet mit `spawnClaudeCodeProcess`-Option). `ChildProcess` erfüllt bereits diese Schnittstelle.

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

Optionen, die an die benutzerdefinierte Spawn-Funktion übergeben werden.

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
  Das `signal`-Feld teilt Ihrer Spawn-Funktion mit, wann der Prozess abgebaut werden soll. Übergeben Sie es als `signal`-Option an Node's `spawn()`, oder übergeben Sie es an Ihren VM- oder Container-Abbau-Handler.

  Dieses Signal wird nicht in dem Moment ausgelöst, in dem [`Options.abortController`](#options) abbricht. Das SDK schließt zunächst die Standardeingabe des Prozesses und wartet etwa zwei Sekunden, damit die CLI sauber herunterfahren kann, dann bricht dieses Signal ab. Um in dem Moment zu reagieren, in dem der Aufrufer abbricht, hören Sie stattdessen auf Ihrem eigenen `Options.abortController.signal`, auf das Ihre Spawn-Funktion aus ihrem umschließenden Bereich verweisen kann.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

Ergebnis einer `setMcpServers()`-Operation.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

Wenn Sie `setMcpServers()` aufrufen, wendet Claude Code diese Regeln an:

* **Server, die der Aufruf nicht benennt**: Claude Code hält von Plugins bereitgestellte Server am Laufen. Erfordert Agent SDK v0.3.210 oder später.
* **Server, die der Aufruf benennt**: außer für integrierte Server, die die CLI beim Start gestartet hat, ersetzt Claude Code einen laufenden Server nur, wenn sich seine Konfiguration von der unterscheidet, die Sie übergeben haben.
* **Integrierte Server, die die CLI beim Start gestartet hat**: Wenn der Aufruf einen benennt, verwirft Claude Code diesen Eintrag und meldet ihn in `errors`.

Das Promise wird aufgelöst, nachdem neu hinzugefügte Stdio-, HTTP- und SSE-Server verbunden oder fehlgeschlagen sind, daher sind Tools von Servern, die verbunden sind, im nächsten Turn verfügbar.

`added` listet die Server auf, die Claude Code hinzugefügt oder ersetzt hat, unabhängig davon, ob sie verbunden sind. Ein Server, der keine Verbindung hergestellt hat, erscheint sowohl in `added` als auch in `errors`, mit dem Fehlertext unter `errors` und einer `failed`-Zeile in [`mcpServerStatus()`](#methods). Vor Claude Code v2.1.257 wurde ein Server, dessen Verbindungsversuch einen Fehler auslöste, nur unter `errors` gemeldet.

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

Ergebnis einer `rewindFiles()`-Operation.

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

`skippedLinks` zählt die verfolgten Pfade, die das Rewind aus Linksicherheitsgründen nicht wiederherstellen oder löschen konnte: ein Symlink, Hard Link oder andere Nicht-Regulär-Datei am verfolgten Pfad, ein übergeordnetes Verzeichnis, das nicht mehr zu dem Ort aufgelöst wird, auf den es zeigte, als der Checkpoint erstellt wurde, oder ein Backup, das nicht sicher gelesen werden konnte. Das Feld erfordert Claude Code v2.1.216 oder später. Ein Vorschau-Aufruf mit `rewindFiles(userMessageId, { dryRun: true })` setzt ihn nie.

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Status-Update-Nachricht (z. B. Komprimierung).

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

Benachrichtigung, wenn eine Hintergrund-Aufgabe abgeschlossen, fehlgeschlagen oder gestoppt wird. Hintergrund-Aufgaben umfassen `run_in_background` Bash-Befehle, [Monitor](#monitor)-Watches und Hintergrund-Subagenten. Für das `ambient`-Feld siehe [`SDKTaskStartedMessage`](#sdktaskstartedmessage), das es definiert und seine Versionsanforderung.

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

Wenn Claude Code [einen langen MCP-Tool-Aufruf in den Hintergrund verschiebt](/docs/de/mcp#automatic-backgrounding-of-long-tool-calls), enthält der `tool_result`-Block für diesen Aufruf nur einen Platzhalter und das echte Ergebnis des Aufrufs kommt in dieser Benachrichtigung an. Gleichen Sie die Benachrichtigung mit dem Aufruf mit `tool_use_id` ab. Bei einer `completed`-Benachrichtigung listet `resource_links` die Dateien auf, die das Tool per Referenz als [`SDKMcpResourceLink`](#sdkmcpresourcelink)-Einträge zurückgegeben hat, mit den gleichen 50-Link- und 64-KiB-Limits wie [`tool_use_result.resourceLinks`](#sdkusermessage). Claude Code lässt `resource_links` weg, wenn das Ergebnis keine Links hatte und bei Benachrichtigungen für Aufgaben, die keine MCP-Tool-Aufrufe sind. `resource_links` erfordert Agent SDK v0.3.257 oder später.

Claude Code stellt jeder Task-Benachrichtigung, die es an das Modell sendet, einen Hinweis voran, außer Lieferungen mit dem [`scheduled-trigger`-Subkind](#task-notification-subkinds), die stattdessen einen zugewiesenen Task-Rahmen tragen. Der Hinweis besagt, dass keine menschliche Eingabe stattgefunden hat, daher behandelt das Modell die Benachrichtigung nicht als Benutzer-Anweisung oder Genehmigung.

Um einen Task-Benachrichtigungs-Turn zu erkennen, überprüfen Sie `origin.kind === "task-notification"` auf der [`SDKUserMessage`](#sdkusermessage) oder [`SDKResultMessage`](#sdkresultmessage), anstatt auf den Hinweistext zu prüfen. Lesen Sie `subkind` aus dem gleichen Feld, wenn Sie wissen müssen, was es ausgelöst hat. Vor v2.1.205 ließ Claude Code den Hinweis bei Benachrichtigungen weg, die ankamen, während die Sitzung untätig war.

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

Zusammenfassung der Tool-Nutzung in einer Konversation.

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

Wird ausgegeben, wenn ein Hook mit der Ausführung beginnt.

Claude Code liefert diese Nachricht, [`SDKHookProgressMessage`](#sdkhookprogressmessage) und [`SDKHookResponseMessage`](#sdkhookresponsemessage) sofort an den Nachrichtenstrom, auch während ein `SessionStart`- oder `Setup`-Hook noch während des Sitzungsstarts läuft. Claude Code v2.1.169 bis v2.1.203 lieferte diese Nachrichten in einem Batch, nachdem ein `SessionStart`- oder `Setup`-Hook abgeschlossen war; v2.1.204 stellte die Live-Lieferung wieder her.

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

Wird ausgegeben, während ein Hook läuft, mit Stdout/Stderr-Ausgabe.

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

Wird ausgegeben, wenn ein Hook die Ausführung beendet.

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

Wird regelmäßig ausgegeben, während ein Tool ausgeführt wird, um Fortschritt anzuzeigen.

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

Während ein Tool-Aufruf in der Hauptkonversation läuft, gibt Claude Code alle 30 Sekunden eine `tool_progress`-Nachricht mit `heartbeat: true` aus. Jeder Heartbeat trägt den Tool-Namen und verstrichene Sekunden, daher können Sie einen lang laufenden Aufruf von einer steckengebliebenen Sitzung unterscheiden. Claude Code gibt keine Heartbeats für Tool-Aufrufe innerhalb eines Subagenten aus. Das `heartbeat`-Feld erfordert Agent SDK v0.3.214 oder später. Vor v2.1.257 gab Claude Code auch keine Heartbeats für einen Vordergrund-Agent-Tool-Aufruf aus.

Bei `tool_progress`-Nachrichten für das Agent-Tool außer Heartbeats benennt `subagent_type` den laufenden Subagenten-Typ, z. B. `general-purpose`. `subagent_retry` ist vorhanden, während dieser Subagent einen API-Fehler-Backoff wartet, z. B. ein Ratenlimit oder Überlastung, mit einer Nachricht pro Wiederholungsversuch. Beide Felder erfordern Agent SDK v0.3.214 oder später.

Um einen Wiederholungs-Indikator aus `subagent_retry` zu rendern:

* Verfolgen Sie den Indikator nach `parent_tool_use_id`, das pro Subagent eindeutig ist. `tool_use_id` wird von parallelen Subagenten aus einem Assistenten-Turn geteilt, daher würde die Verfolgung danach den Indikator eines Subagenten löschen. Löschen Sie den Indikator, wenn ein späterer `tool_progress` für den gleichen `parent_tool_use_id` ankommt, ohne `subagent_retry` noch `heartbeat: true`, oder wenn die Ergebnis-Nachricht des Tools ankommt. Frames mit `heartbeat: true` melden nur Lebendigkeit, daher behalten Sie den Indikator, wenn einer ankommt. `attempt` kann `max_retries` unter persistenter Wiederholung überschreiten, daher leiten Sie das Löschen nicht von den Zählern ab.
* Behandeln Sie `error_category` als Token zur Auswahl Ihres eigenen Nachrichtentextes, nicht als Anzeigetext. Die Werte sind `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error` und `unknown`. Behandeln Sie einen Wert, den Sie nicht erkennen, wie `unknown`, weil spätere Versionen Werte hinzufügen können.

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

Wird während Authentifizierungsflüssen ausgegeben.

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

Wird ausgegeben, wenn eine Aufgabe beginnt. Das `task_type`-Feld ist `"local_bash"` für Bash-Befehle und [Monitor](#monitor)-Watches, `"local_agent"` für Subagenten oder `"remote_agent"`.

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

`ambient` ist `true` für Aufgaben, die nicht Teil der Arbeit der Sitzung sind, z. B. Aufgaben, die Claude Code für seinen eigenen Betrieb ausführt. Live-Update-Watcher sind auch ambient, einschließlich Watcher, die der Benutzer angefordert hat. Schließen Sie ambient-Aufgaben aus Aktivitätsindikatoren aus. Das Feld erfordert Agent SDK v0.3.247 oder später.

`ambient` erscheint auch auf [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) und auf [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage)-Einträgen.

`is_backgrounded` und `spawn_depth` beschreiben, wie Claude Code die Aufgabe gestartet hat. Beide Felder erfordern Agent SDK v0.3.238 oder später.

* `is_backgrounded`: Claude Code setzt es auf `"local_agent"`- und `"local_bash"`-Aufgaben. `true` bedeutet, die Aufgabe läuft im Hintergrund. `false` bedeutet, die Aufgabe läuft im Vordergrund, und der Tool-Aufruf, der sie gestartet hat, bleibt blockiert, bis die Aufgabe beendet wird oder in den Hintergrund wechselt.
* `spawn_depth`: Claude Code setzt es nur auf `"local_agent"`-Aufgaben. Ein Subagent, den der Hauptthread gespawnt hat, hat Tiefe `1`. Ein Subagent, den ein Tiefe-`1`-Subagent gespawnt hat, hat Tiefe `2`, und so weiter.

Ein [fortgesetzter Subagent](/docs/de/agent-sdk/subagents#resume-subagents) meldet immer `is_backgrounded: true`, weil Claude Code jeden fortgesetzten Subagenten im Hintergrund ausführt. Wenn eine Vordergrund-Aufgabe später in den Hintergrund wechselt, meldet Claude Code den neuen `is_backgrounded`-Wert in einer [`task_updated`](#sdktaskupdatedmessage)-Nachricht, anstatt eine zweite `task_started` zu senden.

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Wird regelmäßig ausgegeben, während ein Subagent oder eine Hintergrund-Aufgabe läuft. Das `summary`-Feld wird nur ausgefüllt, wenn [`agentProgressSummaries`](#options) aktiviert ist.

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

Wird ausgegeben, wenn sich der Status einer Hintergrund-Aufgabe ändert, z. B. wenn sie von `running` zu `completed` übergeht. Führen Sie `patch` in Ihre lokale Aufgabenkarte zusammen, die nach `task_id` indiziert ist. Das `end_time`-Feld ist ein Unix-Epoch-Zeitstempel in Millisekunden, vergleichbar mit `Date.now()`.

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

Wird ausgegeben, wenn sich die Menge der aktiven Hintergrund-Aufgaben ändert: eine Aufgabe startet, wird abgeschlossen, wird beendet, ein Vordergrund-Agent wird in den Hintergrund verschoben, oder das `description`- oder `ambient`-Feld einer Aufgabe ändert sich.

Das `tasks`-Array ist die vollständige aktive Menge. Ersetzen Sie alle zwischengespeicherten Mengen mit jeder Nutzlast, anstatt `task_started`- und `task_notification`-Ereignisse zu koppeln, sodass die nächste Änderung der Mitgliedschaft alle verpassten Ereignisse korrigiert.

Die Reihenfolge relativ zu diesen Pro-Aufgaben-Ereignissen ist nicht spezifiziert, daher korrelieren Sie die beiden Streams nicht.

Beim Start wird nichts ausgegeben. Setzen Sie auf eine leere Menge zurück, wenn der CLI-Prozess der Sitzung startet oder neu startet, und lassen Sie die nächste Änderung der Mitgliedschaft ihn neu auffüllen.

Wenn Sie eine wiederholte `initialize`-Steueranfrage an eine laufende Sitzung senden, z. B. mit [`reinitialize()`](#query-object) nach einer Transportlücke, folgt Claude Code der Antwort mit einem Snapshot der aktuellen aktiven Menge, auch wenn sie leer ist. Ein wiederverbindender Host erfährt daher, was läuft, ohne auf die nächste Änderung der Mitgliedschaft zu warten. Vor Agent SDK v0.3.239 sendete Claude Code keinen Snapshot nach einer wiederholten `initialize`.

Erfordert Claude Code v2.1.203 oder später.

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

Wird ausgegeben, während Claude einen Denk-Block produziert, einschließlich eines redigierten. `estimated_tokens` ist eine laufende Schätzung der bisher im aktuellen Block generierten Denk-Token, und `estimated_tokens_delta` ist das Inkrement, das von diesem Frame getragen wird. Verwenden Sie diese Schätzungen für die Fortschrittsanzeige.

Wenn das Modell oder der Provider eine Aufschlüsselung meldet, ist die endgültige Anzahl für die Top-Level-Agent-Schleife die [`usage.output_tokens_details.thinking_tokens`](#usage) der Ergebnis-Nachricht, die [keine Subagenten-Token enthält](/docs/de/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Erfordert Claude Code v2.1.153 oder später.

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

Wird ausgegeben, wenn Datei-Checkpoints auf der Festplatte persistiert werden.

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

Wird ausgegeben, wenn die Sitzung auf ein Ratenlimit trifft.

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

Wenn `errorCode` `"credits_required"` ist, stammt die Ablehnung von einem claude.ai-Abonnement, dessen enthaltene Nutzung aufgebraucht ist, und die Sitzung kann nicht fortgesetzt werden, bis der Benutzer Nutzungsguthaben kauft. `canUserPurchaseCredits` gibt an, ob der authentifizierte Benutzer Guthaben für das Konto kaufen kann, und `hasChargeableSavedPaymentMethod` gibt an, ob eine gespeicherte Zahlungsmethode hinterlegt ist. Alle drei Felder fehlen bei Ratenlimit-Ereignissen, die keine Guthaben-erforderlich-Ablehnungen sind. Erfordert Claude Code v2.1.181 oder später.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Claude Code gibt diese Nachrichtentyp nicht aus. Wenn Sie einen Befehl wie `/context` oder `/usage` als Eingabeaufforderung senden, kommt seine Ausgabe als [`SDKAssistantMessage`](#sdkassistantmessage) an.

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

Wird ausgegeben, wenn sich die Menge der verfügbaren Befehle während einer Sitzung ändert, z. B. wenn Claude Code Skills entdeckt, wenn der Agent ein Unterverzeichnis betritt. Das `commands`-Array ist die vollständig aktualisierte Liste, daher ersetzen Sie alle zwischengespeicherten Befehlslisten durch diese Nutzlast. Das Aufrufen von [`supportedCommands()`](#query-object) nach dieser Nachricht gibt die gleiche aktualisierte Liste zurück, weil die Methode den neuesten Push verfolgt; dies erfordert Agent SDK v0.3.216 oder später. In früheren SDK-Versionen gibt `supportedCommands()` den bei der Initialisierung erfassten Snapshot zurück und spiegelt nie Änderungen während der Sitzung wider.

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

Wird nach einem Turn ausgegeben, wenn [`promptSuggestions`](#options) aktiviert ist und Claude Code eine Suggestion für diesen Turn generiert hat. Enthält die vorhergesagte nächste Benutzer-Eingabeaufforderung. Für die Turns, die keine erhalten, siehe [Wenn Claude Code Suggestions überspringt](/docs/de/interactive-mode#when-claude-code-skips-suggestions).

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

Wird ausgegeben, wenn die Konversation der Sitzung ersetzt wird, ohne die Sitzung zu beenden. In einem `query()`-Aufruf erzeugen nur `/clear` und seine Aliase diese Nachricht. Mounten Sie ein leeres Transkript unter `new_conversation_id` und verwerfen Sie alle zwischengespeicherten Sitzungstitel.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

Die veröffentlichten Typings des SDK deklarieren `SDKConversationResetMessage` in Claude Code v2.1.203 und später. Vor v2.1.203 referenzierte `SDKMessage` den Typ, ohne ihn zu deklarieren, daher schlug die Eingrenzung auf `type === "conversation_reset"` fehl, wenn `skipLibCheck` deaktiviert war.

<h3 id="aborterror">
  `AbortError`
</h3>

Benutzerdefinierte Fehlerklasse für Abbruchoperationen.

```typescript theme={null}
class AbortError extends Error {}
```

`AbortError` ist die einzige Fehlerklasse in der typisierten API des SDK. Andere Fehler, wie das Beenden oder Fehlschlag des Claude-Code-Prozesses, lehnen die Nachrichteniteration mit Fehlern ab, die keine SDK-Klasse zum Abgleichen tragen. [Troubleshooting](/docs/de/agent-sdk/troubleshooting) indiziert diese Fehler nach Nachricht, mit der Ursache und Behebung für jeden.

<h2 id="sandbox-configuration">
  Sandbox-Konfiguration
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Konfiguration für Sandbox-Verhalten. Verwenden Sie dies, um Command-Sandboxing zu aktivieren und Netzwerkbeschränkungen programmatisch zu konfigurieren.

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

| Eigenschaft                 | Typ                                                   | Standard    | Beschreibung                                                                                                                                                                                                                                                               |
| :-------------------------- | :---------------------------------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Aktivieren Sie den Sandbox-Modus für die Befehlsausführung                                                                                                                                                                                                                 |
| `failIfUnavailable`         | `boolean`                                             | `true`      | Stoppen Sie beim Start, wenn `enabled` auf `true` gesetzt ist, aber die Sandbox nicht gestartet werden kann. Setzen Sie `false`, um auf unsandboxed Ausführung mit einer Warnung auf stderr zurückzufallen                                                                 |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Bash-Befehle automatisch genehmigen, wenn Sandbox aktiviert ist                                                                                                                                                                                                            |
| `excludedCommands`          | `string[]`                                            | `[]`        | Befehle, die Sandbox-Beschränkungen umgehen, z. B. `['docker *']`. Diese werden automatisch ohne Modellbeteiligung unsandboxed ausgeführt; [`sandbox.excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) behandelt, wann ein Eintrag gilt                  |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Erlauben Sie dem Modell, die Ausführung von Befehlen außerhalb der Sandbox anzufordern. Wenn `true`, kann das Modell `dangerouslyDisableSandbox` in der Tool-Eingabe setzen, was auf das [Berechtigungssystem](#permissions-fallback-for-unsandboxed-commands) zurückfällt |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Netzwerkspezifische Sandbox-Konfiguration                                                                                                                                                                                                                                  |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Dateisystemspezifische Sandbox-Konfiguration für Lese-/Schreibbeschränkungen                                                                                                                                                                                               |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Zuordnung von Befehlssubstrings oder `*` für jeden Befehl zu Substrings des Verletzungstexts zum Ignorieren, z. B. `{ "*": ['/etc/hosts'] }`; siehe [`sandbox.ignoreViolations`](/docs/de/settings-reference#sandbox-ignoreviolations)                                          |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Aktivieren Sie eine schwächere verschachtelte Sandbox für Kompatibilität                                                                                                                                                                                                   |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Benutzerdefinierte ripgrep-Binärkonfiguration für Sandbox-Umgebungen                                                                                                                                                                                                       |

<Note>
  Die Sandbox hängt von der Plattformunterstützung ab und benötigt unter Linux Tools wie `bubblewrap` und `socat`. Wenn `enabled` auf `true` gesetzt ist und die Sandbox nicht gestartet werden kann, meldet `query()` eine `result`-Nachricht mit `subtype: "error_during_execution"` und den Grund in `errors`. Für einen einzelnen `query()`-Aufruf wirft das SDK nach dem Liefern dieses Fehler-Ergebnisses, daher wickeln Sie die Schleife in einen try-Block ein, um über ihn hinwegzugehen. Siehe [Handle the result](/docs/de/agent-sdk/agent-loop#handle-the-result) für den Fehlervertrag.

  Um stattdessen unsandboxed auszuführen, setzen Sie `failIfUnavailable: false`.
</Note>

<h4 id="example-usage">
  Beispielverwendung
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
  // Ein einzelner query()-Aufruf wirft nach dem Liefern eines Fehler-Ergebnisses,
  // z. B. wenn die Sandbox nicht gestartet werden kann (failIfUnavailable ist standardmäßig true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Unix-Socket-Sicherheit:** Die `allowUnixSockets`-Option kann Zugriff auf Systemdienste gewähren, die außerhalb der Sandbox reichen. Beispielsweise gewährt das Zulassen von `/var/run/docker.sock` effektiv vollständigen Host-Systemzugriff über die Docker-API und umgeht die Sandbox-Isolierung. Lassen Sie nur Unix-Sockets zu, die unbedingt erforderlich sind, und verstehen Sie die Sicherheitsauswirkungen jedes einzelnen.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Netzwerkspezifische Konfiguration für den Sandbox-Modus. Diese Einstellungen gelten für sandboxed Bash-Befehle, wenn `enabled` in den übergeordneten [`SandboxSettings`](#sandboxsettings) auf `true` gesetzt ist. Sie beschränken das WebFetch-Tool nicht, das stattdessen [Berechtigungsregeln](/docs/de/permissions#webfetch) verwendet.

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

| Eigenschaft               | Typ        | Standard    | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------ | :--------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Domänennamen, auf die Sandbox-Prozesse zugreifen können                                                                                                                                                                                                                                                                                                                                                                  |
| `deniedDomains`           | `string[]` | `[]`        | Domänennamen, auf die Sandbox-Prozesse nicht zugreifen können. Hat Vorrang vor `allowedDomains`                                                                                                                                                                                                                                                                                                                          |
| `strictAllowlist`         | `boolean`  | `false`     | Verweigern Sie sandboxed Befehlen den Zugriff auf Hosts außerhalb der [Netzwerk-Allowlist](/docs/de/sandboxing#network-isolation), anstatt zu fragen. Nur für sandboxed Befehle erzwungen; In-Process-Tools wie WebFetch werden nicht dadurch blockiert. Nur von Benutzer-, verwalteten oder CLI `--settings` Einstellungen berücksichtigt; Projekteinstellungen werden ignoriert. Erfordert Claude Code v2.1.219 oder später |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Nur verwaltete Einstellungen. Wenn in [verwalteten Einstellungen](/docs/de/managed-settings) gesetzt, werden nur `allowedDomains`-Einträge und `WebFetch(domain:...)`-Zulassungsregeln aus verwalteten Einstellungen berücksichtigt, und Zulassungseinträge aus Benutzer-, Projekt- oder lokalen Einstellungen werden ignoriert. Hat keine Auswirkung, wenn über SDK-Optionen gesetzt                                         |
| `allowLocalBinding`       | `boolean`  | `false`     | Erlauben Sie Prozessen, sich an lokale Ports zu binden (z. B. für Dev-Server)                                                                                                                                                                                                                                                                                                                                            |
| `allowUnixSockets`        | `string[]` | `[]`        | Unix-Socket-Pfade, auf die Prozesse zugreifen können (z. B. Docker-Socket)                                                                                                                                                                                                                                                                                                                                               |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Erlauben Sie Zugriff auf alle Unix-Sockets                                                                                                                                                                                                                                                                                                                                                                               |
| `httpProxyPort`           | `number`   | `undefined` | HTTP-Proxy-Port für Netzwerkanfragen                                                                                                                                                                                                                                                                                                                                                                                     |
| `socksProxyPort`          | `number`   | `undefined` | SOCKS-Proxy-Port für Netzwerkanfragen                                                                                                                                                                                                                                                                                                                                                                                    |

<Note>
  Der integrierte Sandbox-Proxy erzwingt `allowedDomains` basierend auf dem angeforderten Hostnamen und beendet oder inspiziert keinen TLS-Verkehr, daher können Techniken wie [Domain Fronting](https://en.wikipedia.org/wiki/Domain_fronting) ihn möglicherweise umgehen. Siehe [Sandboxing-Sicherheitsbeschränkungen](/docs/de/sandboxing#security-limitations) für Details und [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment#traffic-forwarding) für die Konfiguration eines TLS-terminierenden Proxys.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Dateisystemspezifische Konfiguration für den Sandbox-Modus.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| Eigenschaft  | Typ        | Standard | Beschreibung                                      |
| :----------- | :--------- | :------- | :------------------------------------------------ |
| `allowWrite` | `string[]` | `[]`     | Dateipfadmuster, um Schreibzugriff zu ermöglichen |
| `denyWrite`  | `string[]` | `[]`     | Dateipfadmuster, um Schreibzugriff zu verweigern  |
| `denyRead`   | `string[]` | `[]`     | Dateipfadmuster, um Lesezugriff zu verweigern     |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Berechtigungen-Fallback für Unsandboxed-Befehle
</h3>

Wenn `allowUnsandboxedCommands` aktiviert ist, kann das Modell anfordern, Befehle außerhalb der Sandbox auszuführen, indem es `dangerouslyDisableSandbox: true` in der Tool-Eingabe setzt. Diese Anfragen fallen auf das bestehende Berechtigungssystem zurück, was bedeutet, dass Ihr `canUseTool`-Handler aufgerufen wird, sodass Sie benutzerdefinierte Autorisierungslogik implementieren können.

Ihre `excludedCommands`-Einträge führen stattdessen einen Aufruf außerhalb der Sandbox mit keiner Modellbeteiligung durch; [`sandbox.excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) behandelt, wann ein Eintrag gilt.

Im folgenden Beispiel steht `isCommandAuthorized` für eine Autorisierungsprüfung, die Sie definieren.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Modell kann unsandboxed Ausführung anfordern
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Überprüfen Sie, ob das Modell die Sandbox umgehen möchte
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // Das Modell fordert an, diesen Befehl außerhalb der Sandbox auszuführen
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
  Befehle, die mit `dangerouslyDisableSandbox: true` ausgeführt werden, haben vollständigen Systemzugriff. Stellen Sie sicher, dass Ihr `canUseTool`-Handler diese Anfragen sorgfältig validiert.

  Wenn `permissionMode` auf `bypassPermissions` gesetzt ist und `allowUnsandboxedCommands` aktiviert ist, kann das Modell autonom Befehle außerhalb der Sandbox ausführen, ohne Genehmigungsaufforderungen, abgesehen von den [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves). Diese Kombination ermöglicht dem Modell effektiv, die Sandbox-Isolierung stillschweigend zu verlassen.
</Warning>

<h2 id="see-also">
  Siehe auch
</h2>

* [SDK-Übersicht](/docs/de/agent-sdk/overview) - Allgemeine SDK-Konzepte
* [Python SDK-Referenz](/docs/de/agent-sdk/python) - Python SDK-Dokumentation
* [CLI-Referenz](/docs/de/cli-reference) - Befehlszeilenschnittstelle
* [Häufige Workflows](/docs/de/common-workflows) - Schritt-für-Schritt-Anleitungen
