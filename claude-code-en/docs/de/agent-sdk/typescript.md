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
  Das SDK bündelt eine native Claude Code-Binärdatei für Ihre Plattform als optionale Abhängigkeit wie `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Sie müssen Claude Code nicht separat installieren. Wenn Ihr Paketmanager optionale Abhängigkeiten überspringt, wirft das SDK `Native CLI binary for <platform> not found`; setzen Sie stattdessen [`pathToClaudeCodeExecutable`](#options) auf eine separat installierte `claude`-Binärdatei.
</Note>

<h3 id="compile-to-a-single-executable">
  In eine einzelne ausführbare Datei kompilieren
</h3>

Wenn Sie Ihre Anwendung mit `bun build --compile` in eine einzelne ausführbare Datei kompilieren, kann das SDK die gebündelte CLI-Binärdatei zur Laufzeit nicht auflösen. `require.resolve` funktioniert nicht innerhalb des virtuellen Dateisystems `$bunfs` der kompilierten ausführbaren Datei, daher wirft das SDK `Native CLI binary for <platform> not found`.

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

Die primäre Funktion für die Interaktion mit Claude Code. Erstellt einen asynchronen Generator, der Nachrichten streamt, wenn sie ankommen.

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

Wärmt den CLI-Unterprozess vor, indem er ihn spawnt und den Initialize-Handshake abschließt, bevor eine Eingabeaufforderung verfügbar ist. Das zurückgegebene [`WarmQuery`](#warmquery)-Handle akzeptiert später eine Eingabeaufforderung und schreibt sie in einen bereits bereiten Prozess, sodass der erste `query()`-Aufruf ohne Kosten für das Spawnen und Initialisieren des Unterprozesses aufgelöst wird.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  Parameter
</h4>

| Parameter             | Typ                   | Beschreibung                                                                                                                                                                                                          |
| :-------------------- | :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Optionales Konfigurationsobjekt. Gleich wie der `options`-Parameter für `query()`                                                                                                                                     |
| `initializeTimeoutMs` | `number`              | Maximale Zeit in Millisekunden zum Warten auf die Unterprozessinitialisierung. Standardwert ist `60000`. Wenn die Initialisierung nicht rechtzeitig abgeschlossen wird, lehnt das Promise mit einem Timeout-Fehler ab |

<h4 id="returns-2">
  Rückgabewert
</h4>

Gibt ein `Promise<`[`WarmQuery`](#warmquery)`>` zurück, das aufgelöst wird, sobald der Unterprozess gespawnt wurde und seinen Initialize-Handshake abgeschlossen hat.

<h4 id="example">
  Beispiel
</h4>

Rufen Sie `startup()` früh auf, beispielsweise beim Anwendungsstart, und rufen Sie dann `.query()` auf dem zurückgegebenen Handle auf, sobald eine Eingabeaufforderung bereit ist. Dies verschiebt das Spawnen und die Initialisierung des Unterprozesses aus dem kritischen Pfad.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Bezahlen Sie die Startup-Kosten im Voraus
const warm = await startup({ options: { maxTurns: 3 } });

// Später, wenn eine Eingabeaufforderung bereit ist, ist dies sofort
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Erstellt eine typsichere MCP-Tool-Definition zur Verwendung mit SDK MCP-Servern.

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
  Parameter
</h4>

| Parameter     | Typ                                                               | Beschreibung                                                                                       |
| :------------ | :---------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                          | Der Name des Tools                                                                                 |
| `description` | `string`                                                          | Eine Beschreibung, was das Tool tut                                                                |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | Zod-Schema, das die Eingabeparameter des Tools definiert (unterstützt sowohl Zod 3 als auch Zod 4) |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Asynchrone Funktion, die die Tool-Logik ausführt                                                   |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | Optionale MCP-Tool-Anmerkungen, die Verhaltenshinweise für Clients bereitstellen                   |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Erneut exportiert aus `@modelcontextprotocol/sdk/types.js`. Alle Felder sind optionale Hinweise; Clients sollten sich nicht auf sie für Sicherheitsentscheidungen verlassen.

| Feld              | Typ       | Standard    | Beschreibung                                                                                                                                          |
| :---------------- | :-------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | Benutzerfreundlicher Titel für das Tool                                                                                                               |
| `readOnlyHint`    | `boolean` | `false`     | Wenn `true`, ändert das Tool seine Umgebung nicht                                                                                                     |
| `destructiveHint` | `boolean` | `true`      | Wenn `true`, kann das Tool destruktive Updates durchführen (nur sinnvoll, wenn `readOnlyHint` `false` ist)                                            |
| `idempotentHint`  | `boolean` | `false`     | Wenn `true`, haben wiederholte Aufrufe mit denselben Argumenten keine zusätzliche Auswirkung (nur sinnvoll, wenn `readOnlyHint` `false` ist)          |
| `openWorldHint`   | `boolean` | `true`      | Wenn `true`, interagiert das Tool mit externen Entitäten (z. B. Websuche). Wenn `false`, ist die Domäne des Tools geschlossen (z. B. ein Memory-Tool) |

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
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parameter
</h4>

| Parameter         | Typ                           | Beschreibung                                                           |
| :---------------- | :---------------------------- | :--------------------------------------------------------------------- |
| `options.name`    | `string`                      | Der Name des MCP-Servers                                               |
| `options.version` | `string`                      | Optionale Versionsnummer                                               |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | Array von Tool-Definitionen, die mit [`tool()`](#tool) erstellt wurden |

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
| `fileSize`     | `number \| undefined` | Sitzungsdateigröße in Bytes. Nur für lokale JSONL-Speicherung gefüllt                                          |
| `customTitle`  | `string \| undefined` | Vom Benutzer festgelegter Sitzungstitel (über `/rename`)                                                       |
| `firstPrompt`  | `string \| undefined` | Erste aussagekräftige Benutzer-Eingabeaufforderung in der Sitzung                                              |
| `gitBranch`    | `string \| undefined` | Git-Branch am Ende der Sitzung                                                                                 |
| `cwd`          | `string \| undefined` | Arbeitsverzeichnis für die Sitzung                                                                             |
| `tag`          | `string \| undefined` | Vom Benutzer festgelegtes Sitzungs-Tag (siehe [`tagSession()`](#tagsession))                                   |
| `createdAt`    | `number \| undefined` | Erstellungszeit in Millisekunden seit Epoch, vom Zeitstempel des ersten Eintrags                               |

<h4 id="example-2">
  Beispiel
</h4>

Geben Sie die 10 neuesten Sitzungen für ein Projekt aus. Ergebnisse werden nach `lastModified` absteigend sortiert, sodass das erste Element das neueste ist. Lassen Sie `dir` weg, um über alle Projekte zu suchen.

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

| Eigenschaft          | Typ                     | Beschreibung                                                                                                                                                                                                                                                                            |
| :------------------- | :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Nachrichtenrolle                                                                                                                                                                                                                                                                        |
| `uuid`               | `string`                | Eindeutige Nachrichten-ID                                                                                                                                                                                                                                                               |
| `session_id`         | `string`                | Sitzung, zu der diese Nachricht gehört                                                                                                                                                                                                                                                  |
| `message`            | `unknown`               | Rohe Nachricht-Payload aus dem Transkript                                                                                                                                                                                                                                               |
| `parent_tool_use_id` | `string \| null`        | Für Subagent-Nachrichten die `tool_use_id` des spawning `Agent`-Tool-Aufrufs. `null` für Hauptsitzungs-Nachrichten und ältere Sitzungen                                                                                                                                                 |
| `parent_agent_id`    | `string \| null`        | Für Nachrichten von einem [verschachtelten Subagent](/docs/de/sub-agents#spawn-nested-subagents) die `agentId` des Subagents, der sie spawnt hat. `null` für Hauptsitzungs-Nachrichten, Nachrichten von Top-Level-Subagents und ältere Sitzungen. Erfordert Claude Code v2.1.202 oder später |

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

Liest Metadaten für eine einzelne Sitzung nach ID, ohne das vollständige Projektverzeichnis zu scannen.

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

Taggt eine Sitzung. Übergeben Sie `null`, um das Tag zu löschen. Wiederholte Aufrufe sind sicher; das neueste Tag gewinnt.

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
| `sessionId`   | `string`         | erforderlich | UUID der zu taggenden Sitzung                                                         |
| `tag`         | `string \| null` | erforderlich | Tag-Zeichenkette oder `null` zum Löschen                                              |
| `options.dir` | `string`         | `undefined`  | Projektverzeichnispfad. Wenn weggelassen, werden alle Projektverzeichnisse durchsucht |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Löst die effektiven Claude Code-Einstellungen für ein bestimmtes Verzeichnis mithilfe der gleichen Merge-Engine wie die CLI auf, ohne die Claude CLI zu spawnen. Verwenden Sie es, um zu überprüfen, welche Konfiguration ein `query()`-Aufruf sehen würde, bevor Sie einen aufrufen.

<Note>
  Diese Funktion ist Alpha und ihre API kann sich vor der Stabilisierung ändern. Sie liest MDM-Quellen, einschließlich macOS plist und Windows HKLM/HKCU, für Parität mit CLI-Startup, führt aber nicht den vom Administrator konfigurierten `policyHelper`-Unterprozess aus. Das Feld `permissions.defaultMode` wird unverändert aus allen Ebenen einschließlich Projekteinstellungen zurückgegeben. Der Vertrauensfilter, den die CLI vor der Berücksichtigung eskalierender Berechtigungsmodi anwendet, wird nicht angewendet.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parameter
</h4>

`resolveSettings()` akzeptiert ein einzelnes Optionsobjekt. Alle Felder sind optional.

| Parameter                       | Typ                                   | Standard        | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------ | :------------------------------------ | :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | Verzeichnis zum Auflösen von Projekt- und lokalen Einstellungen relativ zu                                                                                                                                                                                                                                                                                                                                                                                               |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Alle Quellen    | Welche Dateisystemquellen geladen werden sollen. Übergeben Sie `[]`, um Benutzer-, Projekt- und lokale Einstellungen zu überspringen. Verwaltete Richtlinieneinstellungen werden in allen Fällen geladen. Server-verwaltete Einstellungen werden von `serverManagedSettings` übernommen, wenn der Host diese übergibt, oder aus dem On-Disk-Cache der CLI gelesen; der Snapshot ruft sie nicht aus dem Netzwerk ab                                                       |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Restriktive Richtlinien-Tier-Einstellungen, die vom Embedding-Host bereitgestellt werden. Gelöscht standardmäßig, wenn eine vom Administrator bereitgestellte verwaltete Tier vorhanden ist; zusammengeführt unter dieser Tier, wenn [`parentSettingsBehavior`](/docs/de/settings#available-settings) `"merge"` ist. Nicht-restriktive Schlüssel wie `model` werden stillschweigend gelöscht, sodass diese Option verwaltete Richtlinien verschärfen, aber nicht lockern kann |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Server-verwaltete Einstellungs-Payload von `/api/claude_code/settings`. Nicht-restriktive Schlüssel werden ungefiltert durchgelassen                                                                                                                                                                                                                                                                                                                                     |

<h4 id="return-type-resolvedsettings">
  Rückgabetyp: `ResolvedSettings`
</h4>

`resolveSettings()` gibt ein Objekt zurück, das die zusammengeführten Einstellungen und die Quelle beschreibt, die jeden Schlüssel beigetragen hat.

| Eigenschaft  | Typ                                                 | Beschreibung                                                                                     |
| :----------- | :-------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| `effective`  | `Settings`                                          | Zusammengeführte Einstellungen nach Anwendung aller aktivierten Quellen in Prioritätsreihenfolge |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Für jeden Top-Level-Schlüssel in `effective`, welche Quelle den Wert bereitgestellt hat          |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Pro-Quelle rohe Einstellungen, geordnet von niedrigster zu höchster Priorität                    |

<h4 id="example-4">
  Beispiel
</h4>

Das folgende Beispiel löst Einstellungen für ein Projektverzeichnis auf und gibt die Quelle aus, die die Bereinigungsperiode steuert.

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

| Eigenschaft                       | Typ                                                                                                      | Standard                                                 | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`                                  | Controller zum Abbrechen von Operationen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                                                     | Zusätzliche Verzeichnisse, auf die Claude zugreifen kann                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `agent`                           | `string`                                                                                                 | `undefined`                                              | Agent-Name für den Hauptthread. Der Agent muss in der `agents`-Option oder in den Einstellungen definiert sein                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                                              | Programmatische Definition von Subagenten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                                                  | Wenn `true`, generieren Sie einzeilige Fortschrittsübersichten für Subagenten und leiten Sie diese auf [`task_progress`](#sdktaskprogressmessage)-Ereignissen über das `summary`-Feld weiter. Gilt für Vordergrund- und Hintergrund-Subagenten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                                                  | Aktivieren Sie das Umgehen von Berechtigungen. Erforderlich bei Verwendung von `permissionMode: 'bypassPermissions'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                                                     | Tools, die automatisch genehmigt werden, ohne zu fragen. Dies beschränkt Claude nicht nur auf diese Tools; nicht aufgelistete Tools fallen durch `permissionMode` und `canUseTool`. Verwenden Sie `disallowedTools`, um Tools zu blockieren. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                                                     | Beta-Funktionen aktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                                              | Benutzerdefinierte Berechtigungsfunktion, die nur aufgerufen wird, wenn der [Berechtigungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Eingabeaufforderung führt. Nicht aufgerufen für Aufrufe, die von `allowedTools`, Allow-Regeln oder `permissionMode` automatisch genehmigt werden. `AskUserQuestion`, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, erreichen es auch, wenn Sie diese zugelassen haben; im `dontAsk`-Modus werden diese stattdessen verweigert. Siehe [`CanUseTool`](#canusetool) für Details |
| `continue`                        | `boolean`                                                                                                | `false`                                                  | Setzen Sie die neueste Konversation fort                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                                          | Aktuelles Arbeitsverzeichnis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `debug`                           | `boolean`                                                                                                | `false`                                                  | Aktivieren Sie den Debug-Modus für den Claude Code-Prozess                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `debugFile`                       | `string`                                                                                                 | `undefined`                                              | Schreiben Sie Debug-Protokolle in einen bestimmten Dateipfad. Aktiviert implizit den Debug-Modus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                                                     | Tools, die verweigert werden. Ein einfacher Name wie `"Bash"` entfernt das Tool aus Claudes Kontext. Eine scoped-Regel wie `"Bash(rm *)"` lässt das Tool verfügbar und verweigert übereinstimmende Aufrufe in jedem Berechtigungsmodus, einschließlich `bypassPermissions`. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                      |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | Modell-Standard                                          | Steuert, wie viel Aufwand Claude in seine Antwort investiert. Funktioniert mit adaptivem Denken, um die Denktiefe zu lenken. Siehe [Aufwandsstufe anpassen](/docs/de/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                                                  | Aktivieren Sie die Dateienänderungsverfolgung zum Zurückspulen. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                                            | Umgebungsvariablen. Wenn gesetzt, ersetzt dies die Subprocess-Umgebung, anstatt sie mit `process.env` zusammenzuführen. Übergeben Sie daher `{ ...process.env, YOUR_VAR: 'value' }`, um geerbte Variablen wie `PATH` beizubehalten. Siehe [Langsame oder steckengebliebene API-Antworten verarbeiten](#handle-slow-or-stalled-api-responses) für ein Beispiel dieses Musters und [Umgebungsvariablen](/docs/de/env-vars) für Variablen, die die zugrunde liegende CLI liest. Setzen Sie `CLAUDE_AGENT_SDK_CLIENT_APP`, um Ihre App im User-Agent-Header zu identifizieren                                                                                                                                                                    |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | Automatisch erkannt                                      | JavaScript-Laufzeit zum Verwenden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                                                     | Argumente, die an die ausführbare Datei übergeben werden sollen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                                                     | Zusätzliche Argumente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                                              | Modell, das verwendet werden soll, wenn das primäre fehlschlägt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `forkSession`                     | `boolean`                                                                                                | `false`                                                  | Beim Fortsetzen mit `resume` zu einer neuen Sitzungs-ID verzweigen, anstatt die ursprüngliche Sitzung fortzusetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                                                  | Leiten Sie Subagenten-Text und Denk-Blöcke als Assistenten- und Benutzer-Nachrichten mit `parent_tool_use_id` weiter, damit Consumer ein verschachteltes Transkript rendern können. Standardmäßig werden nur `tool_use`- und `tool_result`-Blöcke von Subagenten ausgegeben                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                                                     | Hook-Callbacks für Ereignisse                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                                                  | Schließen Sie Hook-Lebenszyklusereignisse für jedes Hook-Ereignis im Nachrichtenstrom als [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage) und [`SDKHookResponseMessage`](#sdkhookresponsemessage) ein. Lebenszyklusereignisse für `SessionStart`- und `Setup`-Hooks sind immer enthalten und benötigen diese Option nicht                                                                                                                                                                                                                                                                                                                                                        |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                                                  | Teilweise Nachrichtenereignisse einbeziehen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                                                  | *Alpha.* Timeout in Millisekunden für jeden `sessionStore.load()`- und `sessionStore.listSubkeys()`-Aufruf während der Resume-Materialisierung. Wenn sich der Adapter nicht innerhalb dieses Fensters einigt, schlägt die Abfrage fehl, anstatt zu hängen. Wird ignoriert, wenn `sessionStore` nicht gesetzt ist                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                                              | Richtlinien-Tier-Einstellungen, die vom spawning Parent-Prozess bereitgestellt werden. Werden gelöscht, wenn bereits ein IT-kontrollierter verwalteter Einstellungs-Tier auf der Maschine vorhanden ist, es sei denn, dieser Administrator entscheidet sich mit `parentSettingsBehavior: 'merge'` dafür. Gefiltert auf restriktive-only-Schlüssel unabhängig                                                                                                                                                                                                                                                                                                                                                                            |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                                              | Beenden Sie die Abfrage, wenn die clientseitige Kostenschätzung diesen USD-Wert erreicht. Verglichen mit derselben Schätzung wie `total_cost_usd`; siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking) für Genauigkeitsvorbehalt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                                              | *Veraltet:* Verwenden Sie stattdessen `thinking`. Maximale Token für den Denkprozess                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                                              | Maximale agentengesteuerte Turns (Tool-Use-Roundtrips)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                                                     | MCP-Server-Konfigurationen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `model`                           | `string`                                                                                                 | Standard aus CLI                                         | Claude-Modell-Alias oder vollständiger Modellname. Siehe [akzeptierte Werte und Provider-spezifische IDs](/docs/de/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                                              | Callback für die Verarbeitung von MCP-Elicitierungsanfragen. Wird aufgerufen, wenn ein MCP-Server Benutzereingaben anfordert und kein Hook dies zuerst verarbeitet. Wenn nicht bereitgestellt, werden unbehandelte Elicitierungsanfragen automatisch abgelehnt                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                                              | Definieren Sie das Ausgabeformat für Agent-Ergebnisse. Siehe [Strukturierte Ausgaben](/docs/de/agent-sdk/structured-outputs) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                                              | Nicht ein `Options`-Feld. Setzen Sie `outputStyle` im Inline-[`settings`](/docs/de/settings)-Objekt oder einer Einstellungsdatei. Siehe [Aktivieren Sie einen Ausgabestil](/docs/de/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | Automatisch aufgelöst aus gebündelter nativer Binärdatei | Pfad zur Claude Code-Ausführungsdatei. Nur erforderlich, wenn optionale Abhängigkeiten während der Installation übersprungen wurden oder Ihre Plattform nicht in der unterstützten Menge enthalten ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                                              | Berechtigungsmodus für die Sitzung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                                              | MCP-Tool-Name für Berechtigungsaufforderungen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `persistSession`                  | `boolean`                                                                                                | `true`                                                   | Wenn `false`, deaktiviert die Sitzungspersistenz auf der Festplatte. Sitzungen können später nicht fortgesetzt werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                                              | Benutzerdefinierte Workflow-Anweisungen für den Plan-Modus. Wenn `permissionMode` `'plan'` ist, ersetzt diese Zeichenkette den Standard-Plan-Modus-Workflow-Body. Die CLI umhüllt ihn immer noch mit der schreibgeschützten Durchsetzungspräambel und dem ExitPlanMode-Protokoll-Footer                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                                                     | Laden Sie benutzerdefinierte Plugins aus lokalen Pfaden. Siehe [Plugins](/docs/de/agent-sdk/plugins) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                                                  | Aktivieren Sie Eingabeaufforderungsvorschläge. Gibt nach jedem Turn eine `prompt_suggestion`-Nachricht mit einer vorhergesagten nächsten Benutzer-Eingabeaufforderung aus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `resume`                          | `string`                                                                                                 | `undefined`                                              | Sitzungs-ID zum Fortsetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                                              | Sitzung bei einer bestimmten Nachrichten-UUID fortsetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                                              | Konfigurieren Sie das Sandbox-Verhalten programmatisch. Siehe [Sandbox-Einstellungen](#sandboxsettings) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sessionId`                       | `string`                                                                                                 | Automatisch generiert                                    | Verwenden Sie eine bestimmte UUID für die Sitzung, anstatt eine zu generieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `sessionStore`                    | [`SessionStore`](/docs/de/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                                              | Spiegeln Sie Sitzungstranskripte auf einem externen Backend, damit jeder Host sie fortsetzen kann. Siehe [Sitzungen im externen Speicher persistieren](/docs/de/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                                              | *Alpha.* Flush-Modus für `sessionStore`. Wird ignoriert, wenn `sessionStore` nicht gesetzt ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                                              | Inline-[Einstellungen](/docs/de/settings)-Objekt oder Pfad zu einer Einstellungsdatei. Füllt die Flag-Einstellungsebene in der [Prioritätsreihenfolge](/docs/de/settings#settings-precedence) auf. Ändern Sie zur Laufzeit mit [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | CLI-Standards (alle Quellen)                             | Steuern Sie, welche Dateisystem-Einstellungen geladen werden. Übergeben Sie `[]`, um Benutzer-, Projekt- und lokale Einstellungen zu deaktivieren. Verwaltete Richtlinieneinstellungen werden unabhängig davon geladen; Server-verwaltete Einstellungen werden abgerufen, wenn sich die Sitzung mit einer Organisationsanmeldedaten auf einer [berechtigten Konfiguration](/docs/de/server-managed-settings#platform-availability) authentifiziert. Siehe [Claude Code-Funktionen verwenden](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                        |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                                              | Skills, die der Sitzung zur Verfügung stehen. Übergeben Sie `'all'`, um jeden entdeckten Skill zu aktivieren, oder eine Liste von Skill-Namen. Wenn gesetzt, aktiviert das SDK das Skill-Tool automatisch in `allowedTools`. Wenn Sie auch `tools` übergeben, beziehen Sie `'Skill'` in diese Liste ein. Siehe [Skills](/docs/de/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                           |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                                              | Benutzerdefinierte Funktion zum Spawnen des Claude Code-Prozesses. Verwenden Sie, um Claude Code in VMs, Containern oder Remote-Umgebungen auszuführen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                                              | Callback für Stderr-Ausgabe                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                                                  | Verwenden Sie nur die Server, die in `mcpServers` übergeben werden, und ignorieren Sie das Projekt `.mcp.json`, Benutzereinstellungen, von Plugins bereitgestellte MCP-Server und [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (minimale Eingabeaufforderung)               | Konfiguration der Systemeingabeaufforderung. Übergeben Sie eine Zeichenkette für eine benutzerdefinierte Eingabeaufforderung oder `{ type: 'preset', preset: 'claude_code' }`, um die Systemeingabeaufforderung von Claude Code zu verwenden. Bei Verwendung der Preset-Objektform fügen Sie `append` hinzu, um sie mit zusätzlichen Anweisungen zu erweitern, und setzen Sie `excludeDynamicSections: true`, um sitzungsspezifischen Kontext in die erste Benutzer-Nachricht zu verschieben, um [bessere Prompt-Cache-Wiederverwendung über Maschinen hinweg](/docs/de/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                 |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                                              | *Alpha.* API-seitiges Task-Budget in Token. Wenn gesetzt, wird dem Modell sein verbleibendes Token-Budget mitgeteilt, damit es die Tool-Nutzung pacing kann und vor dem Limit abwickelt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | `{ type: 'adaptive' }` für unterstützte Modelle          | Steuert das Denk-/Reasoning-Verhalten von Claude. Siehe [`ThinkingConfig`](#thinkingconfig) für Optionen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `title`                           | `string`                                                                                                 | `undefined`                                              | Anzeigetitel für die Sitzung. Beim Fortsetzen über `resume` oder `continue` hat der persistierte Titel der fortgesetzten Sitzung Vorrang; verwenden Sie [`renameSession()`](#renamesession), um eine vorhandene Sitzung umzubenennen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                                              | Ordnen Sie integrierte Tool-Namen MCP-Tool-Namen zu, damit Claude Ihre MCP-Implementierung anstelle der integrierten aufruft. Zum Beispiel `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                                              | Konfiguration für das Verhalten integrierter Tools. Siehe [`ToolConfig`](#toolconfig) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                                              | Tool-Konfiguration. Übergeben Sie ein Array von Tool-Namen oder verwenden Sie die Voreinstellung, um die Standard-Tools von Claude Code zu erhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h4 id="handle-slow-or-stalled-api-responses">
  Langsame oder steckengebliebene API-Antworten verarbeiten
</h4>

Der CLI-Unterprozess liest mehrere Umgebungsvariablen, die API-Timeouts und Stall-Erkennung steuern. Übergeben Sie sie über die `env`-Option:

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

* `API_TIMEOUT_MS`: Pro-Request-Timeout auf dem Anthropic-Client in Millisekunden. Standard `600000`. Gilt für die Hauptschleife und alle Subagenten.
* `CLAUDE_CODE_MAX_RETRIES`: Maximale API-Wiederholungen. Standard `10`, begrenzt auf `15`. Jede Wiederholung erhält sein eigenes `API_TIMEOUT_MS`-Fenster, sodass die schlimmste Wandzeit ungefähr `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus Backoff ist. Für unbeaufsichtigte Läufe, die längere Ausfallzeiten abwarten müssen, setzen Sie `CLAUDE_CODE_RETRY_WATCHDOG=1`: Es wiederholt Kapazitätsfehler unbegrenzt, und ab Claude Code v2.1.199 erhöht sich der Standard für andere vorübergehende Fehler auf `300` und entfernt die Obergrenze für diese Variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: Stall-Watchdog für Subagenten, die mit `run_in_background` gestartet werden. Standard `600000`. Setzt sich bei jedem Stream-Ereignis zurück; bei Stall bricht es den Subagenten ab, markiert die Aufgabe als fehlgeschlagen und zeigt den Fehler dem übergeordneten Element mit jedem Teilergebnis. Gilt nicht für synchrone Subagenten.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` mit `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: Bricht die Anfrage ab, wenn Header angekommen sind, aber der Antwortkörper nicht mehr streamt. Der Watchdog ist standardmäßig für alle Provider aktiviert; setzen Sie `CLAUDE_ENABLE_STREAM_WATCHDOG=0`, um ihn zu deaktivieren. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` hat einen Standard von `300000` und ist auf dieses Minimum begrenzt. Die abgebrochene Anfrage durchläuft den normalen Wiederholungspfad.

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
  Methoden
</h4>

| Methode                                | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Unterbricht die Abfrage. Nur im Streaming-Eingabemodus verfügbar. Wenn die CLI die `interrupt_receipt_v1`-Funktion in [`SDKSystemMessage.capabilities`](#sdksystemmessage) ankündigt, wird mit einem [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) aufgelöst, das die in der Warteschlange befindlichen Nachrichten auflistet, die den Interrupt überstehen. Wird auf CLIs vor v2.1.205 mit `undefined` aufgelöst                                                                                                                                |
| `rewindFiles(userMessageId, options?)` | Stellt Dateien in ihren Zustand bei der angegebenen Benutzer-Nachricht wieder her. Übergeben Sie `{ dryRun: true }`, um Änderungen in der Vorschau anzuzeigen. Erfordert `enableFileCheckpointing: true`. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                          |
| `setPermissionMode()`                  | Ändert den Berechtigungsmodus (nur im Streaming-Eingabemodus verfügbar)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `setModel()`                           | Ändert das Modell (nur im Streaming-Eingabemodus verfügbar)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `setMaxThinkingTokens()`               | *Veraltet:* Verwenden Sie stattdessen die `thinking`-Option. Ändert die maximalen Denk-Token                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `applyFlagSettings(settings)`          | Führt Einstellungen zur Laufzeit in die Flag-Einstellungsebene der Sitzung zusammen (nur im Streaming-Eingabemodus verfügbar). Siehe [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                 |
| `initializationResult()`               | Gibt das vollständige Initialisierungsergebnis zurück, einschließlich unterstützter Befehle, Modelle, Kontoinformationen und Ausgabestil-Konfiguration                                                                                                                                                                                                                                                                                                                                                                                                           |
| `reinitialize()`                       | Sendet die `initialize`-Steueranfrage erneut an die laufende CLI und gibt ein frisches Ergebnis anstelle des zwischengespeicherten First-Connect-Ergebnisses zurück. Verwenden Sie es nach einer Transportlücke, z. B. nach dem Wiederherstellen einer Verbindung zu einer Sitzung nach einer Trennung, damit ausstehende Berechtigungsanfragen Ihren `canUseTool`-Callback erneut erreichen. Machen Sie den Callback idempotent pro Request-ID, da eine Anfrage, deren Antwort verloren ging, erneut versendet wird. Erfordert Claude Code v2.1.195 oder später |
| `supportedCommands()`                  | Gibt verfügbare Slash-Befehle zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `supportedModels()`                    | Gibt verfügbare Modelle mit Anzeigeinformationen zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `supportedAgents()`                    | Gibt verfügbare Subagenten als [`AgentInfo`](#agentinfo)`[]` zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `mcpServerStatus()`                    | Gibt den Status verbundener MCP-Server zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `accountInfo()`                        | Gibt Kontoinformationen zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `reconnectMcpServer(serverName)`       | Verbinden Sie einen MCP-Server nach Name erneut                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `toggleMcpServer(serverName, enabled)` | Aktivieren oder deaktivieren Sie einen MCP-Server nach Name                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `setMcpServers(servers)`               | Ersetzen Sie dynamisch die Menge der MCP-Server für diese Sitzung. Gibt Informationen darüber zurück, welche Server hinzugefügt, entfernt und welche Fehler aufgetreten sind                                                                                                                                                                                                                                                                                                                                                                                     |
| `streamInput(stream)`                  | Streamen Sie Eingabenachrichten zur Abfrage für Multi-Turn-Konversationen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `stopTask(taskId)`                     | Beenden Sie eine laufende Hintergrund-Aufgabe nach ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `close()`                              | Schließen Sie die Abfrage und beenden Sie den zugrunde liegenden Prozess. Beendet die Abfrage erzwungen und bereinigt alle Ressourcen                                                                                                                                                                                                                                                                                                                                                                                                                            |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Ändert [Einstellungen](/docs/de/settings) auf einer laufenden Sitzung, ohne die Abfrage neu zu starten. Verwenden Sie es, wenn eine Einstellung, die keinen dedizierten Setter hat, sich mitten in der Sitzung ändern muss, z. B. um `permissions` zu verschärfen, nachdem der Agent nicht vertrauenswürdige Eingaben liest. `setModel()` und `setPermissionMode()` sind dedizierte Setter für diese beiden Schlüssel; `applyFlagSettings()` ist die allgemeine Form, die jede Teilmenge der Einstellungsschlüssel akzeptiert, und das Übergeben von `model` hier verhält sich genauso wie `setModel()`.

Nur einige Schlüssel wirken sich mitten in der Sitzung aus:

* **Angewendet beim nächsten Turn**: `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Das Wechseln von `agent` wendet auch die Modellüberschreibung, Hooks und das System-Prompt dieses Agenten beim nächsten Turn an.
* **Keine Auswirkung mitten in der Sitzung**: die Systemeingabeaufforderungsoptionen. Diese werden einmal beim Start aufgelöst, sodass die laufende Sitzung den ursprünglichen Wert behält, obwohl der Aufruf erfolgreich ist. Um sie zu ändern, starten Sie eine neue Sitzung.

`effortLevel` akzeptiert einen [Aufwandsstufen](/docs/de/model-config#adjust-effort-level)-Namen. Es akzeptiert auch `"ultracode"`, das die Sitzung mit `xhigh`-Aufwand ausführt und [ultracode](/docs/de/workflows#let-claude-decide-with-ultracode) aktiviert. Der `Settings`-Typ deklariert `effortLevel` ohne diesen Wert, daher übergeben Sie das Äquivalent `{ ultracode: true }` in TypeScript. Der `ultracode`-Wert erfordert Claude Code v2.1.203 oder später und wird nur von `applyFlagSettings()` akzeptiert, nicht vom `effortLevel`-Schlüssel in einer Einstellungsdatei.

Die Werte werden in die Flag-Einstellungsebene geschrieben, die gleiche Ebene, die die Inline-`settings`-Option von `query()` beim Start füllt. Flag-Einstellungen befinden sich in der Nähe der Oberseite der [Einstellungspriorität](/docs/de/settings#settings-precedence): Sie überschreiben Benutzer-, Projekt- und lokale Einstellungen, und nur verwaltete Richtlinieneinstellungen können sie überschreiben. Dies ist die gleiche Ebene, die der [Abschnitt zur Priorität auf der Seite](#settings-precedence) programmatische Optionen nennt.

Aufeinanderfolgende Aufrufe führen Top-Level-Schlüssel flach zusammen. Ein zweiter Aufruf mit `{ permissions: {...} }` ersetzt das gesamte `permissions`-Objekt aus dem vorherigen Aufruf, anstatt es tief zusammenzuführen. Um einen Schlüssel aus der Flag-Ebene zu löschen und auf niedrigere Prioritätsquellen zurückzugreifen, übergeben Sie `null` für diesen Schlüssel. Das Übergeben von `undefined` hat keine Auswirkung, da die JSON-Serialisierung es löscht.

Nur im Streaming-Eingabemodus verfügbar, die gleiche Einschränkung wie `setModel()` und `setPermissionMode()`.

Das folgende Beispiel wechselt das aktive Modell mitten in der Sitzung und löscht dann die Überschreibung, sodass das Modell auf das zurückfällt, was die Benutzer- oder Projekteinstellungen angeben.

```typescript theme={null}
const q = query({ prompt: messageStream });

// Überschreiben Sie das Modell für den Rest der Sitzung
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Später: Löschen Sie die Überschreibung und greifen Sie auf niedrigere Prioritätseinstellungen zurück
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` ist nur TypeScript. Das Python SDK stellt keine entsprechende Methode bereit.
</Note>

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle, das von [`startup()`](#startup) zurückgegeben wird. Der Unterprozess ist bereits gespawnt und initialisiert, sodass das Aufrufen von `query()` auf diesem Handle die Eingabeaufforderung direkt in einen bereiten Prozess ohne Startup-Latenz schreibt.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Methoden
</h4>

| Methode         | Beschreibung                                                                                                                                                              |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `query(prompt)` | Senden Sie eine Eingabeaufforderung an den vorgewärmten Unterprozess und geben Sie ein [`Query`](#query-object) zurück. Kann nur einmal pro `WarmQuery` aufgerufen werden |
| `close()`       | Schließen Sie den Unterprozess, ohne eine Eingabeaufforderung zu senden. Verwenden Sie dies, um eine warme Abfrage zu verwerfen, die nicht mehr benötigt wird             |

`WarmQuery` implementiert `AsyncDisposable`, sodass es mit `await using` für automatische Bereinigung verwendet werden kann.

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
};
```

Wenn ein Client `initialize` an eine bereits laufende Sitzung sendet, trägt der Control-Response-Wrapper auch ein optionales `pending_permission_requests`-Array. Das Feld befindet sich auf dem Response-Wrapper selbst, nicht in der oben beschriebenen `SDKControlInitializeResponse`-Nutzlast. Jeder Eintrag ist eine vollständige `control_request`-Nachricht mit der gleichen `{ type: "control_request", request_id, request }`-Form, die die Sitzung für Berechtigungsanfragen während der Ausführung streamt.

Dies sind Anfragen, die vor der Verbindung des Clients gestellt wurden und noch auf eine Antwort warten. Das SDK liest das Array für Sie und versendet jeden Eintrag an Ihren [`canUseTool`](#canusetool)-Callback, die gleiche Wiederversendung, die [`reinitialize()`](#query-object) nach einer Transportlücke auslöst. Behandeln Sie wiederholte Request-IDs idempotent, da ein Eintrag eine Anfrage wiederholen kann, die der Callback bereits erhalten hat, bevor die Verbindung unterbrochen wurde.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Der Interrupt-Beleg: der Wert, mit dem [`interrupt()`](#query-object) auf einer CLI aufgelöst wird, die die `interrupt_receipt_v1`-Funktion in [`SDKSystemMessage.capabilities`](#sdksystemmessage) ankündigt. Erfordert Claude Code v2.1.205 oder später. Frühere CLIs beantworten den Interrupt mit einer leeren Erfolgsnutzlast, daher wird `interrupt()` mit `undefined` aufgelöst.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` listet die UUIDs von Benutzernachrichten auf, die den Interrupt überstehen: Nachrichten, die sich noch in der Warteschlange befinden, plus jeden Batch, der bereits für den nächsten Turn aus der Warteschlange entfernt wurde, aber noch nicht vom Abort erreichbar ist. Jede wird als eigener Turn nach dem Interrupt ausgeführt, es sei denn, Sie brechen sie zuerst ab. Verwenden Sie den Beleg, um zu entscheiden, ob Sie etwas erneut senden möchten; das erneute Senden einer Nachricht, die bereits aufgelistet ist, erzeugt einen doppelten Turn.

Interpretieren Sie die Liste mit diesen Vorbehalten:

* Nur Nachrichten, die mit einer UUID in die Warteschlange eingereiht wurden, werden angezeigt. Ein leeres Array bedeutet nicht, dass nichts anderes ausgeführt wird.
* Nur Hauptthread-Nachrichten werden aufgelistet. Nachrichten, die an einen Subagenten adressiert sind, sind außerhalb des Geltungsbereichs.
* Die Liste kann UUIDs enthalten, die Ihr Client nie gesendet hat, z. B. [geplante Task](/docs/de/scheduled-tasks)-Trigger. Ignorieren Sie UUIDs, die Sie nicht erkennen, anstatt sie als Fehler zu behandeln.

Der Beleg ist eine Momentaufnahme, die zum Zeitpunkt der Verarbeitung des Interrupts erstellt wird, und bei einem sauberen Interrupt kommt er an, bevor das [`SDKResultMessage`](#sdkresultmessage) des unterbrochenen Turns. Lesen Sie den Beleg, anstatt die Warteschlange nach diesem Ergebnis zu überprüfen: Die Schleife startet den nächsten in der Warteschlange befindlichen Turn sofort, sodass sich die Warteschlange, die Sie nach dem Ergebnis überprüfen, bereits geändert hat.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Konfiguration für einen programmatisch definierten Subagenten.

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

| Feld                                  | Erforderlich | Beschreibung                                                                                                                                                                                                                                              |
| :------------------------------------ | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Ja           | Natürlichsprachige Beschreibung, wann dieser Agent verwendet werden soll                                                                                                                                                                                  |
| `tools`                               | Nein         | Array von zulässigen Tool-Namen. Wenn weggelassen, erbt alle Tools vom übergeordneten Element. Um Skills in den Agent-Kontext vorzuladen, verwenden Sie das `skills`-Feld, anstatt `'Skill'` hier aufzulisten                                             |
| `disallowedTools`                     | Nein         | Array von Tool-Namen, die für diesen Agent explizit nicht zulässig sind. MCP-Server-Level-Muster werden auch akzeptiert: `mcp__server` oder `mcp__server__*` entfernt jedes Tool von diesem Server, und `mcp__*` entfernt jedes MCP-Tool von jedem Server |
| `prompt`                              | Ja           | Die Systemeingabeaufforderung des Agenten                                                                                                                                                                                                                 |
| `model`                               | Nein         | Modellüberschreibung für diesen Agenten. Akzeptiert einen Alias wie `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, oder eine vollständige Modell-ID. Wenn weggelassen oder `'inherit'`, verwendet das Hauptmodell                               |
| `mcpServers`                          | Nein         | MCP-Server-Spezifikationen für diesen Agenten                                                                                                                                                                                                             |
| `skills`                              | Nein         | Array von Skill-Namen, die in den Agent-Kontext vorgeladen werden sollen                                                                                                                                                                                  |
| `initialPrompt`                       | Nein         | Wird automatisch als erster Benutzer-Turn eingereicht, wenn dieser Agent als Hauptthread-Agent ausgeführt wird                                                                                                                                            |
| `maxTurns`                            | Nein         | Maximale Anzahl agentengesteuerter Turns (API-Roundtrips) vor dem Stoppen                                                                                                                                                                                 |
| `background`                          | Nein         | Führen Sie diesen Agent als nicht-blockierende Hintergrund-Aufgabe aus, wenn er aufgerufen wird                                                                                                                                                           |
| `memory`                              | Nein         | Speicherquelle für diesen Agent: `'user'`, `'project'` oder `'local'`                                                                                                                                                                                     |
| `effort`                              | Nein         | Reasoning-Aufwandsstufe für diesen Agent. Akzeptiert eine benannte Stufe oder eine Ganzzahl                                                                                                                                                               |
| `permissionMode`                      | Nein         | Berechtigungsmodus für die Tool-Ausführung innerhalb dieses Agenten. Siehe [`PermissionMode`](#permissionmode)                                                                                                                                            |
| `criticalSystemReminder_EXPERIMENTAL` | Nein         | Experimentell: Kritische Erinnerung, die zur Systemeingabeaufforderung hinzugefügt wird                                                                                                                                                                   |

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

Steuert, welche dateisystembasierte Konfigurationsquellen das SDK Einstellungen aus lädt.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Wert        | Beschreibung                                             | Ort                           |
| :---------- | :------------------------------------------------------- | :---------------------------- |
| `'user'`    | Globale Benutzereinstellungen                            | `~/.claude/settings.json`     |
| `'project'` | Gemeinsame Projekteinstellungen (versionskontrolliert)   | `.claude/settings.json`       |
| `'local'`   | Lokale Projekteinstellungen (nicht versionskontrolliert) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Standardverhalten
</h4>

Wenn `settingSources` weggelassen oder `undefined` ist, lädt `query()` die gleichen Dateisystem-Einstellungen wie die Claude Code CLI: Benutzer, Projekt und lokal. Verwaltete Richtlinieneinstellungen werden in allen Fällen geladen; Server-verwaltete Einstellungen werden abgerufen, wenn sich die Sitzung mit einer Organisationsanmeldedaten auf einer [berechtigten Konfiguration](/docs/de/server-managed-settings#platform-availability) authentifiziert. Siehe [Was settingSources nicht steuert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control) für Eingaben, die unabhängig von dieser Option gelesen werden, und wie man sie deaktiviert.

<h4 id="why-use-settingsources">
  Warum settingSources verwenden
</h4>

**Dateisystem-Einstellungen deaktivieren:**

```typescript theme={null}
// Laden Sie keine Benutzer-, Projekt- oder lokalen Einstellungen von der Festplatte
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Alle Dateisystem-Einstellungen explizit laden:**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // Laden Sie alle Einstellungen
  }
});
```

**Nur bestimmte Einstellungsquellen laden:**

```typescript theme={null}
// Laden Sie nur Projekteinstellungen, ignorieren Sie Benutzer und lokal
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Nur .claude/settings.json
  }
});
```

**Test- und CI-Umgebungen:**

```typescript theme={null}
// Stellen Sie konsistentes Verhalten in CI sicher, indem Sie lokale Einstellungen ausschließen
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // Nur teamweit gemeinsame Einstellungen
    permissionMode: "bypassPermissions"
  }
});
```

**SDK-only-Anwendungen:**

```typescript theme={null}
// Definieren Sie alles programmatisch.
// Übergeben Sie [], um sich von Dateisystem-Einstellungsquellen abzumelden.
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

**Laden von CLAUDE.md-Projektanweisungen:**

```typescript theme={null}
// Laden Sie Projekteinstellungen, um CLAUDE.md-Dateien einzubeziehen
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Verwenden Sie die Systemeingabeaufforderung von Claude Code
    },
    settingSources: ["project"], // Lädt CLAUDE.md aus dem Projektverzeichnis
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  Einstellungspriorität
</h4>

Wenn mehrere Quellen geladen werden, werden Einstellungen mit dieser Priorität zusammengeführt (höchste zu niedrigste):

1. Lokale Einstellungen (`.claude/settings.local.json`)
2. Projekteinstellungen (`.claude/settings.json`)
3. Benutzereinstellungen (`~/.claude/settings.json`)

Programmatische Optionen wie `agents`, `allowedTools` und `settings` überschreiben Benutzer-, Projekt- und lokale Dateisystem-Einstellungen. Verwaltete Richtlinieneinstellungen haben Vorrang vor programmatischen Optionen.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Standardberechtigungsverhalten
  | "acceptEdits" // Dateibearbeitungen automatisch akzeptieren
  | "bypassPermissions" // Alle Berechtigungsprüfungen umgehen; explizite Ask-Regeln werden immer noch aufgefordert
  | "plan" // Planungsmodus - Erkunden ohne Bearbeitung
  | "dontAsk" // Fragen Sie nicht nach Berechtigungen, verweigern Sie, wenn nicht vorab genehmigt
  | "auto"; // Verwenden Sie einen Modell-Klassifizierer, um jeden Tool-Aufruf zu genehmigen oder zu verweigern
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Benutzerdefinierte Berechtigungsfunktionstyp zur Steuerung der Tool-Nutzung.

Die Funktion ist der SDK-Ersatz für die interaktive Berechtigungsaufforderung: Sie wird nur aufgerufen, wenn der [Berechtigungsbewertungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Eingabeaufforderung führt. Tool-Aufrufe, die bereits von einem `allowedTools`-Eintrag, einer Settings-Allow-Regel oder dem Berechtigungsmodus wie `acceptEdits` oder `bypassPermissions` genehmigt wurden, rufen ihn nie auf. `AskUserQuestion`, MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, und Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) erreichen die Funktion auch, wenn eine Allow-Regel passt. Im `dontAsk`-Modus werden diese Aufrufe stattdessen verweigert, ohne sie aufzurufen. Um jeden Tool-Aufruf zu gaten, verwenden Sie stattdessen einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks).

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

| Option           | Typ                                         | Beschreibung                                                                                                                                                                                                                                                                                                                                                               |
| :--------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Signalisiert, wenn die Operation abgebrochen werden soll                                                                                                                                                                                                                                                                                                                   |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Vorgeschlagene Berechtigungsaktualisierungen, damit der Benutzer nicht erneut für dieses Tool aufgefordert wird. Bash-Eingabeaufforderungen enthalten einen Vorschlag mit dem `localSettings` [Ziel](#permissionupdatedestination), sodass die Rückgabe in `updatedPermissions` die Regel in `.claude/settings.local.json` schreibt und über Sitzungen hinweg persistiert. |
| `blockedPath`    | `string`                                    | Der Dateipfad, der die Berechtigungsanfrage ausgelöst hat, falls zutreffend                                                                                                                                                                                                                                                                                                |
| `decisionReason` | `string`                                    | Erklärt, warum diese Berechtigungsanfrage ausgelöst wurde                                                                                                                                                                                                                                                                                                                  |
| `toolUseID`      | `string`                                    | Eindeutige ID für diesen spezifischen Tool-Aufruf innerhalb der Assistenten-Nachricht                                                                                                                                                                                                                                                                                      |
| `agentID`        | `string`                                    | Wenn innerhalb eines Sub-Agenten ausgeführt, die ID des Sub-Agenten                                                                                                                                                                                                                                                                                                        |
| `requestId`      | `string`                                    | Die `control_request`-Umschlag-`request_id`. Eine `control_response`, die Ihre Anwendung außerhalb des SDK sendet, z. B. ein signierter HTTP POST, muss diesen Wert widerspiegeln, damit der Claude Code-Prozess die Antwort mit der Anfrage abgleichen kann                                                                                                               |

Der Callback löst die Anfrage normalerweise durch Rückgabe eines [`PermissionResult`](#permissionresult) auf, das das SDK über seinen Transport als `control_response` zurückschreibt. Geben Sie `null` nur zurück, wenn Ihre Anwendung die `control_response` für diese Anfrage bereits über ihren eigenen Kanal gesendet hat, wobei `requestId` widergespiegelt wird; das SDK überspringt dann das Schreiben der Antwort auf seinen Transport. Das Zurückgeben von `null` in jedem anderen Fall lässt den Tool-Aufruf unbegrenzt blockiert, da keine `control_response` jemals gesendet wird und Berechtigungsaufforderungen nicht zeitlich begrenzt sind.

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

| Feld                            | Typ                    | Beschreibung                                                                                                                                                                                 |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Aktiviert das `preview`-Feld auf [`AskUserQuestion`](/docs/de/agent-sdk/user-input#question-format)-Optionen und legt sein Inhaltsformat fest. Wenn nicht gesetzt, gibt Claude keine Vorschau aus |

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

| Feld               | Typ       | Beschreibung                                                                                                                                                                                                                 |
| :----------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Muss `'local'` sein (derzeit nur lokale Plugins unterstützt)                                                                                                                                                                 |
| `path`             | `string`  | Absoluter oder relativer Pfad zum Plugin-Verzeichnis                                                                                                                                                                         |
| `skipMcpDiscovery` | `boolean` | Wenn `true`, lädt das SDK Skills, Hooks, Agents und Befehle aus diesem Plugin, liest aber nicht seine `.mcp.json` oder Manifest `mcpServers`. Setzen Sie dies, wenn Ihre Anwendung die MCP-Verbindungen des Plugins besitzt. |

**Beispiel:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Vollständige Informationen zum Erstellen und Verwenden von Plugins finden Sie unter [Plugins](/docs/de/agent-sdk/plugins).

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
};
```

Das `message`-Feld ist eine [`BetaMessage`](https://platform.claude.com/docs/de/api/messages/create) aus dem Anthropic SDK. Es enthält Felder wie `id`, `content`, `model`, `stop_reason` und `usage`.

`SDKAssistantMessageError` ist einer von: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'` oder `'unknown'`. `'model_not_found'` bedeutet, dass das ausgewählte Modell nicht existiert oder nicht für Ihr Konto oder Ihre Bereitstellung verfügbar ist. `'overloaded'` bedeutet, dass die API einen 529-Fehler zurückgegeben hat, weil der Server ausgelastet ist, im Gegensatz zu `'rate_limit'`, das ein 429-Fehler gegen Ihr Kontingent ist.

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
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Setzen Sie `shouldQuery` auf `false`, um die Nachricht zum Transkript hinzuzufügen, ohne einen Assistenten-Turn auszulösen. Die Nachricht wird gehalten und in die nächste Benutzer-Nachricht zusammengeführt, die einen Turn auslöst. Verwenden Sie dies, um Kontext einzufügen, z. B. die Ausgabe eines Befehls, den Sie außerhalb des Bands ausgeführt haben, ohne einen Modell-Aufruf dafür auszugeben.

Auf einer Nachricht, die einen `tool_result`-Block trägt, ist `tool_use_result` das strukturierte Ausgabeobjekt des Tools und nicht der Text, der an das Modell gesendet wird. Seine Form hängt vom Tool ab, das durch den entsprechenden `tool_use`-Block benannt wird, daher ist das Feld als `unknown` typisiert; die integrierten Formen sind unter [Tool-Ausgabetypen](#tool-output-types) aufgelistet.

Für das `Agent`-Tool ist `tool_use_result` [`AgentOutput`](#agent-2). Bei einem `completed`-Ergebnis enthält `content` den Bericht des Subagenten ohne die Agent-ID und den Nutzungs-Trailer, den Claude Code an den `tool_result`-Text anhängt, daher rendern Sie stattdessen aus `tool_use_result`.

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

Mehrere Felder im Ergebnis enthalten diagnostische Details über `subtype` hinaus:

* `api_error_status`: Der HTTP-Statuscode des API-Fehlers, der die Konversation beendet hat. Fehlt oder ist `null`, wenn der Turn ohne API-Fehler endete.
* `ttft_ms`: Zeit bis zum ersten Token in Millisekunden, gemessen, wenn die erste vollständige Assistenten-Nachricht ankommt. Nur auf dem Success-Arm vorhanden.
* `ttft_stream_ms`: Zeit in Millisekunden bis zum ersten `message_start`-Stream-Ereignis, wenn der Response-Stream öffnet. Niedriger als `ttft_ms`; die Lücke zwischen den beiden ist die Zeit, die zum Streamen der ersten Nachricht benötigt wird. Nur auf dem Success-Arm vorhanden.
* `terminal_reason`: Warum die Schleife endete. Einer von `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"` oder `"turn_setup_failed"`.
* `fast_mode_state`: Einer von `"on"`, `"off"` oder `"cooldown"`.

Das `origin`-Feld leitet die [`SDKMessageOrigin`](#sdkmessageorigin) der Benutzer-Nachricht weiter, die dieses Ergebnis ausgelöst hat. Wenn eine Hintergrund-Aufgabe beendet wird und das SDK einen synthetischen Follow-up-Turn einfügt, trägt die resultierende `SDKResultMessage` `origin: { kind: "task-notification" }`. Überprüfen Sie dieses Feld, um Ergebnisse zu unterscheiden, die Ihre Eingabeaufforderung beantworten, von Ergebnissen, die für Hintergrund-Aufgaben-Follow-ups ausgegeben werden, damit Sie letztere weiterleiten oder unterdrücken können. Das Feld fehlt bei Ergebnissen, die vor einem Benutzer-Turn ausgegeben werden, z. B. Startfehler.

Wenn ein `PreToolUse`-Hook `permissionDecision: "defer"` zurückgibt, hat das Ergebnis `stop_reason: "tool_deferred"` und `deferred_tool_use` enthält die `id`, den `name` und die `input` des ausstehenden Tools. Lesen Sie dieses Feld, um die Anfrage in Ihrer eigenen Benutzeroberfläche anzuzeigen, und setzen Sie dann mit derselben `session_id` fort, um fortzufahren. Siehe [Einen Tool-Aufruf für später aufschieben](/docs/de/hooks#defer-a-tool-call-for-later) für die vollständige Runde.

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

Das `capabilities`-Array benennt die Protokoll-Verhaltensweisen, die diese CLI implementiert, damit Sie Feature-Erkennung durchführen können, anstatt `claude_code_version`-Zeichenketten zu vergleichen. Es ist ein offenes Set: Ignorieren Sie Werte, die Sie nicht erkennen, und überprüfen Sie auf die spezifische Fähigkeit, deren Verhalten Sie benötigen. Das Feld erfordert Claude Code v2.1.205 oder später und fehlt auf früheren CLIs.

| Fähigkeit              | Bedeutung                                                                                                                                                                                                                   |
| ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) wird mit einer [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) Quittung aufgelöst, die die in die Warteschlange eingereihten Nachrichten benennt, die den Interrupt überstehen |

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
};
```

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

Generisches Text-Banner, das von der Schleife ausgegeben wird. Enthält nicht-fehlerhafte Statuszeilen, Hook-Feedback wie ein Block-Grund eines `UserPromptSubmit`-Hooks und Befehlsausgabe. Rendern Sie `content` als Klartext auf der angegebenen `level`.

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

Stream-Ereignis, das ausgegeben wird, wenn das Berechtigungssystem einen Tool-Aufruf automatisch ablehnt, ohne eine interaktive Eingabeaufforderung anzuzeigen. Verwenden Sie es, um die Ablehnung in Ihrer Benutzeroberfläche zu rendern, während sie geschieht, anstatt nur das `is_error`-Tool-Ergebnis zu beobachten, das folgt. Der interaktive Anfragepfad erreicht Ihre Anwendung separat über den [`canUseTool`](#canusetool)-Callback. Ablehnungen, die von einem `PreToolUse`-Hook ausgegeben werden, werden nicht über dieses Ereignis gemeldet.

Dieses Ereignis erfordert Claude Code v2.1.136 oder später.

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

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Herkunft einer Benutzer-Rolle-Nachricht. Dies erscheint als `origin` auf [`SDKUserMessage`](#sdkusermessage) und wird an die entsprechende [`SDKResultMessage`](#sdkresultmessage) weitergeleitet, damit Sie erkennen können, was einen bestimmten Turn ausgelöst hat.

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

| `kind`              | Bedeutung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Direkte Eingabe vom Endbenutzer. Bei Benutzer-Nachrichten bedeutet auch eine fehlende `origin` menschliche Eingabe.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `channel`           | Nachricht, die auf einem [Kanal](/docs/de/channels) ankommt. `server` ist der Name des Quell-MCP-Servers.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `peer`              | Nachricht von einem anderen Agent. Für einen In-Process-[Teamkollegen](/docs/de/agent-teams), der an `main` über `SendMessage` sendet, ist `from` der Name des Teamkollegen und `senderTaskId` ist seine Task-ID. Für einen Cross-Session-Peer wie einen anderen lokalen Claude Code-Prozess ist `from` die Absenderadresse und `senderTaskId` fehlt. `name` und `body` erfordern Claude Code v2.1.205 oder später. `name` ist der Anzeigename des Absenders, normalisiert von Claude Code: Es entfernt Unicode-Steuer-, Format-, Surrogate- und Zeilen- oder Absatz-Trennzeichen-Codepunkte, schneidet dann das Ergebnis ab und begrenzt es auf 64 Codepunkte mit einer Ellipse. `body` ist der dekodierte Nachrichtentext mit der Peer-Hülle entfernt, byte-genau mit dem, was das Modell sieht. Für eine Teamkollegen-Nachricht ist `body` immer vorhanden; für einen Cross-Session-Peer ist es nur vorhanden, wenn der Turn genau eine von Claude Code gebildete Peer-Hülle ist. Rendern Sie `name` und `body` anstatt die Nachricht erneut zu analysieren. |
| `task-notification` | Synthetischer Turn, der nach Abschluss einer Hintergrund-Aufgabe eingefügt wird. Siehe [`SDKTaskNotificationMessage`](#sdktasknotificationmessage).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `coordinator`       | Nachricht von einem Team-Koordinator in einem [Agent-Team](/docs/de/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `auto-continuation` | Synthetischer Turn, der eingefügt wird, wenn die Sitzung ohne neue Benutzereingabe fortgesetzt wird, z. B. ein Befehlsergebnis, das eine Follow-up-Eingabeaufforderung auslöst.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

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
  /** @deprecated seit v2.1.178. Enthält den von der Sitzung abgeleiteten Teamnamen; wird entfernt. */
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
  /** @deprecated seit v2.1.178. Enthält den von der Sitzung abgeleiteten Teamnamen; wird entfernt. */
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
  Tool-Eingabetypen
</h2>

Dokumentation von Eingabeschemas für alle integrierten Claude Code-Tools. Diese Typen werden aus `@anthropic-ai/claude-agent-sdk` exportiert und können für typsichere Tool-Interaktionen verwendet werden.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Union aller Tool-Eingabetypen, exportiert aus `@anthropic-ai/claude-agent-sdk`.

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

**Tool-Name:** `Agent` (zuvor `Task`, das immer noch als Alias akzeptiert wird)

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

Führt Bash-Befehle in einer persistenten Shell-Sitzung mit optionalem Timeout und Hintergrundausführung aus.

<h3 id="monitor">
  Monitor
</h3>

**Tool-Name:** `Monitor`

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

Führt eine Hintergrundquelle aus und liefert jedes Ereignis an Claude, damit es reagieren kann, ohne zu pollen: `command` führt ein Skript aus und gibt ein Ereignis pro Stdout-Zeile aus, und `ws` öffnet einen WebSocket und gibt ein Ereignis pro Textframe aus. Geben Sie genau eines von `command` oder `ws` an. Die `ws`-Quelle erfordert Claude Code v2.1.195 oder später.

Setzen Sie `persistent: true` für Sitzungslängen-Watches wie Log-Tails. Wenn Monitor einen Befehl ausführt, folgt es den gleichen Berechtigungsregeln wie Bash; ein WebSocket-Watch fordert separat zur Genehmigung auf. Siehe die [Monitor-Tool-Referenz](/docs/de/tools-reference#monitor-tool) für Verhalten und Anbieter-Verfügbarkeit.

<h3 id="taskoutput">
  TaskOutput
</h3>

**Tool-Name:** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

Ruft die Ausgabe einer laufenden oder abgeschlossenen Hintergrund-Aufgabe ab.

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
  args?: unknown;
  resumeFromRunId?: string;
};
```

Führt einen [dynamischen Workflow](/docs/de/workflows) aus: ein Skript, das viele Subagenten im Hintergrund orchestriert und ein konsolidiertes Ergebnis zurückgibt. Das `Workflow`-Tool ist in Agent SDK v0.3.149 und später verfügbar. Mindestens eines von `script`, `name` oder `scriptPath` ist erforderlich.

| Feld              | Typ       | Beschreibung                                                                                                                                                                                                                                                                                    |
| ----------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Inline-Workflow-Skript. Muss mit `export const meta = { name, description }` als Literal beginnen, gefolgt vom Skript-Body mit `agent()`, `parallel()`, `pipeline()` und `phase()`. Ein optionales `phases`-Array in `meta` gruppiert Agenten unter benannten Phasen in der Fortschrittsansicht |
| `name`            | `string`  | Name eines integrierten Workflows oder eines in `.claude/workflows/` gespeicherten. Wird zu einem Skript aufgelöst                                                                                                                                                                              |
| `scriptPath`      | `string`  | Pfad zu einer Workflow-Skriptdatei auf der Festplatte. Hat Vorrang vor `script` und `name`. Jeder Aufruf speichert sein Skript und gibt den Pfad im Ergebnis zurück, sodass Sie diese Datei bearbeiten und erneut mit demselben `scriptPath` aufrufen können, um zu iterieren                   |
| `args`            | `unknown` | Eingabewert, der dem Skript als globales `args` verfügbar gemacht wird, für parametrisierte benannte Workflows wie eine Forschungsfrage oder eine Liste von Dateipfaden. Übergeben Sie Arrays und Objekte als tatsächliche JSON-Werte, nicht als JSON-codierte Zeichenkette                     |
| `resumeFromRunId` | `string`  | Run-ID eines vorherigen `Workflow`-Aufrufs zum Fortsetzen. Abgeschlossene `agent()`-Aufrufe mit unveränderten Eingaben geben zwischengespeicherte Ergebnisse zurück; nur geänderte oder neue Aufrufe werden live ausgeführt. Nur gleiche Sitzung                                                |

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
  Ab TypeScript Agent SDK 0.3.142 ist `TodoWrite` standardmäßig deaktiviert. Verwenden Sie stattdessen `TaskCreate`, `TaskGet`, `TaskUpdate` und `TaskList`. Siehe [Zu Task-Tools migrieren](/docs/de/agent-sdk/todo-tracking#migrate-to-task-tools), um Ihren Überwachungscode zu aktualisieren, oder setzen Sie `CLAUDE_CODE_ENABLE_TASKS=0`, um zu `TodoWrite` zurückzukehren.
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

<h2 id="tool-output-types">
  Tool-Ausgabetypen
</h2>

Dokumentation von Ausgabeschemas für alle integrierten Claude Code-Tools. Diese Typen werden aus `@anthropic-ai/claude-agent-sdk` exportiert und stellen die tatsächlichen Antwortdaten dar, die von jedem Tool zurückgegeben werden.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Union aller Tool-Ausgabetypen.

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

**Tool-Name:** `Agent` (zuvor `Task`, das immer noch als Alias akzeptiert wird)

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

Gibt das Ergebnis vom Subagenten zurück. Diskriminiert nach dem `status`-Feld: `"completed"` für abgeschlossene Aufgaben, `"async_launched"` für Hintergrund-Aufgaben und `"remote_launched"` für Aufgaben, die Claude Code an eine Remote-Cloud-Sitzung versendet hat, wobei `sessionUrl` auf diese Sitzung verweist und `taskId` sie identifiziert.

Das Feld `resolvedModel` in den Varianten `completed` und `async_launched` benennt das Modell, auf dem der Subagent tatsächlich ausgeführt wurde, das sich vom angeforderten `model`-Input unterscheiden kann, wenn [`availableModels`](/docs/de/model-config#restrict-model-selection) oder eine andere Überschreibung gilt. Dieses Feld erfordert Claude Code v2.1.174 oder später.

In der Variante `completed` wird `worktreePath` gesetzt, wenn der Subagent in einem isolierten Git-Worktree ausgeführt wurde, und `worktreeBranch` benennt den Branch dieses Worktrees, wenn Claude Code ihn erstellt hat. `usage.service_tier` enthält die Service-Tier-Zeichenkette, die die API für die Anfragen des Subagenten gemeldet hat.

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
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

Gibt Befehlsausgabe mit aufgeteiltem Stdout/Stderr zurück. Hintergrund-Befehle enthalten eine `backgroundTaskId`.

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

Gibt die Hintergrund-Aufgaben-ID für den laufenden Monitor zurück. Verwenden Sie diese ID mit `TaskStop`, um die Watch früh zu stornieren.

<h3 id="edit-2">
  Edit
</h3>

**Tool-Name:** `Edit`

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
  };
};
```

Gibt das Schreib-Ergebnis mit strukturierten Diff-Informationen zurück.

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
};
```

Gibt Dateipfade zurück, die dem Glob-Muster entsprechen, sortiert nach Änderungszeit.

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
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Gibt Suchergebnisse zurück. Die Form variiert je nach `mode`: Dateiliste, Inhalt mit Übereinstimmungen oder Übereinstimmungszahlen.

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
};
```

Gibt den abgerufenen Inhalt mit HTTP-Status und Metadaten zurück.

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
};
```

Gibt Suchergebnisse aus dem Web zurück.

<h3 id="workflow-2">
  Workflow
</h3>

**Tool-Name:** `Workflow`

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

Gibt sofort nach dem Akzeptieren des Tools die Invokation zurück. Das endgültige Ergebnis kommt später als Aufgabenvollendung an. Überprüfen Sie `error`, bevor Sie den Lauf als gestartet behandeln: Ein Skript, das seine Syntaxprüfung nicht besteht, gibt `status: "async_launched"` mit gesetztem `error` zurück und wird nie ausgeführt.

| Feld            | Typ                | Beschreibung                                                                                                                                                                |
| --------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched"` | Das Tool hat die Invokation akzeptiert. Dies ist der einzige Wert, den das Feld annimmt                                                                                     |
| `taskId`        | `string`           | Hintergrund-Aufgabenkennung für den Lauf                                                                                                                                    |
| `runId`         | `string`           | Workflow-Lauf-Kennung, die als `resumeFromRunId` bei einer späteren Invokation übergeben werden soll                                                                        |
| `summary`       | `string`           | Einzeilige Beschreibung, was der Workflow tut                                                                                                                               |
| `transcriptDir` | `string`           | Verzeichnis, in dem Subagenten-Transkripte während der Ausführung geschrieben werden                                                                                        |
| `scriptPath`    | `string`           | Pfad zum persistierten Workflow-Skript für diesen Lauf. Bearbeiten Sie es und übergeben Sie es als `scriptPath`, um es erneut auszuführen, ohne das Skript erneut zu senden |
| `error`         | `string`           | Wird gesetzt, wenn das Skript seine Syntaxprüfung nicht besteht. Wenn vorhanden, wurde der Lauf trotz des `async_launched`-Status nicht gestartet                           |

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
  Ab TypeScript Agent SDK 0.3.142 ist `TodoWrite` standardmäßig deaktiviert. Verwenden Sie stattdessen `TaskCreate`, `TaskGet`, `TaskUpdate` und `TaskList`. Siehe [Zu Task-Tools migrieren](/docs/de/agent-sdk/todo-tracking#migrate-to-task-tools), um Ihren Überwachungscode zu aktualisieren, oder setzen Sie `CLAUDE_CODE_ENABLE_TASKS=0`, um zu `TodoWrite` zurückzukehren.
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
  }>;
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

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Verfügbare Beta-Funktionen, die über die `betas`-Option aktiviert werden können. Siehe [Beta-Header](https://platform.claude.com/docs/de/api/beta-headers) für weitere Informationen.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  Das `context-1m-2025-08-07`-Beta ist ab dem 30. April 2026 veraltet. Das Übergeben dieses Wertes mit Claude Sonnet 4.5 oder Sonnet 4 hat keine Auswirkung, und Anfragen, die das Standard-200k-Token-Kontextfenster überschreiten, geben einen Fehler zurück. Um ein 1M-Token-Kontextfenster zu verwenden, migrieren Sie zu [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 oder Claude Opus 4.8](https://platform.claude.com/docs/de/about-claude/models/overview), die 1M-Kontext zu Standardpreisen ohne Beta-Header enthalten.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informationen über einen verfügbaren Slash-Befehl.

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

| Feld          | Typ                   | Beschreibung                                                                              |
| :------------ | :-------------------- | :---------------------------------------------------------------------------------------- |
| `name`        | `string`              | Agent-Typ-Identifikator (z. B. `"Explore"`, `"general-purpose"`)                          |
| `description` | `string`              | Beschreibung, wann dieser Agent verwendet werden soll                                     |
| `model`       | `string \| undefined` | Modell-Alias, den dieser Agent verwendet. Wenn weggelassen, erbt das übergeordnete Modell |

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
};
```

`BetaServerToolUsage` und `BetaIterationsUsage` sind in `@anthropic-ai/sdk` definiert.

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

Das optionale `display`-Feld steuert, ob Denk-Text `"summarized"` oder `"omitted"` zurückgegeben wird. Bei Claude Opus 4.7 und später ist der API-Standard `"omitted"`, daher setzen Sie `"summarized"`, um Denk-Inhalte in `thinking`-Blöcken zu erhalten.

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
};
```

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

Benachrichtigung, wenn eine Hintergrund-Aufgabe abgeschlossen, fehlgeschlagen oder gestoppt wird. Hintergrund-Aufgaben umfassen `run_in_background` Bash-Befehle, [Monitor](#monitor)-Watches und Hintergrund-Subagenten.

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
  uuid: UUID;
  session_id: string;
};
```

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

Wird ausgegeben, wenn eine Hintergrund-Aufgabe beginnt. Das `task_type`-Feld ist `"local_bash"` für Hintergrund-Bash-Befehle und [Monitor](#monitor)-Watches, `"local_agent"` für Subagenten oder `"remote_agent"`.

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

Wird ausgegeben, wenn sich die Menge der aktiven Hintergrund-Aufgaben ändert: eine Aufgabe startet, wird abgeschlossen, wird beendet oder ein Vordergrund-Agent wird in den Hintergrund verschoben. Das `tasks`-Array ist die vollständige aktive Menge. Ersetzen Sie alle zwischengespeicherten Mengen mit jeder Nutzlast, anstatt `task_started`- und `task_notification`-Ereignisse zu koppeln, sodass die nächste Änderung der Mitgliedschaft alle verpassten Ereignisse korrigiert.

Die Reihenfolge relativ zu diesen Pro-Aufgaben-Ereignissen ist nicht spezifiziert, daher korrelieren Sie die beiden Streams nicht.

Beim Start wird nichts ausgegeben. Setzen Sie auf eine leere Menge zurück, wenn der CLI-Prozess der Sitzung startet oder neu startet, und lassen Sie die nächste Änderung der Mitgliedschaft ihn neu auffüllen.

Erfordert Claude Code v2.1.203 oder später.

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

Wird ausgegeben, während Claude einen Denk-Block produziert, einschließlich eines redigierten, mit einer laufenden Schätzung der bisher generierten Denk-Token. `estimated_tokens` ist die laufende Summe für den aktuellen Denk-Block und `estimated_tokens_delta` ist das Inkrement, das von diesem Frame getragen wird. Verwenden Sie es für die Fortschrittsanzeige. Die endgültige Anzahl für die Top-Level-Agent-Schleife ist die `usage.output_tokens` der Ergebnis-Nachricht, die [keine Subagenten-Token enthält](/docs/de/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); verwenden Sie [`modelUsage`](#modelusage) für die Gesamtbaum-Buchhaltung.

Erfordert Claude Code v2.1.153 oder später.

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

Ausgabe aus einem lokalen Slash-Befehl (z. B. `/voice` oder `/usage`). Wird als Assistenten-ähnlicher Text im Transkript angezeigt.

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

Wird ausgegeben, wenn sich die Menge der verfügbaren Befehle während einer Sitzung ändert, z. B. wenn Skills entdeckt werden, wenn der Agent ein Unterverzeichnis betritt. Das `commands`-Array ist die vollständig aktualisierte Liste, daher ersetzen Sie alle zwischengespeicherten Befehlslisten durch diese Nutzlast. Das erneute Aufrufen von `supportedCommands()` ist nicht gleichwertig: Diese Methode gibt den bei der Initialisierung erfassten Snapshot zurück und spiegelt keine Änderungen während der Sitzung wider.

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

Wird nach jedem Turn ausgegeben, wenn `promptSuggestions` aktiviert ist. Enthält eine vorhergesagte nächste Benutzer-Eingabeaufforderung.

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

Wird ausgegeben, wenn die Konversation der Sitzung ersetzt wird, ohne die Sitzung zu beenden, z. B. nach `/clear`, beim Beenden des Plan-Modus oder wenn eine neue Konversation startet. Mounten Sie ein leeres Transkript unter `new_conversation_id` und verwerfen Sie alle zwischengespeicherten Sitzungstitel.

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
| `excludedCommands`          | `string[]`                                            | `[]`        | Befehle, die immer Sandbox-Beschränkungen umgehen (z. B. `['docker']`). Diese werden automatisch ohne Modellbeteiligung unsandboxed ausgeführt                                                                                                                             |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Erlauben Sie dem Modell, die Ausführung von Befehlen außerhalb der Sandbox anzufordern. Wenn `true`, kann das Modell `dangerouslyDisableSandbox` in der Tool-Eingabe setzen, was auf das [Berechtigungssystem](#permissions-fallback-for-unsandboxed-commands) zurückfällt |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Netzwerkspezifische Sandbox-Konfiguration                                                                                                                                                                                                                                  |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Dateisystemspezifische Sandbox-Konfiguration für Lese-/Schreibbeschränkungen                                                                                                                                                                                               |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Zuordnung von Verletzungskategorien zu Mustern zum Ignorieren (z. B. `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                                                                                       |
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
  **Unix-Socket-Sicherheit:** Die `allowUnixSockets`-Option kann Zugriff auf leistungsstarke Systemdienste gewähren. Beispielsweise gewährt das Zulassen von `/var/run/docker.sock` effektiv vollständigen Host-Systemzugriff über die Docker-API und umgeht die Sandbox-Isolierung. Lassen Sie nur Unix-Sockets zu, die unbedingt erforderlich sind, und verstehen Sie die Sicherheitsauswirkungen jedes einzelnen.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Netzwerkspezifische Konfiguration für den Sandbox-Modus. Diese Einstellungen gelten für sandboxed Bash-Befehle, wenn `enabled` in den übergeordneten [`SandboxSettings`](#sandboxsettings) auf `true` gesetzt ist. Sie beschränken das WebFetch-Tool nicht, das stattdessen [Berechtigungsregeln](/docs/de/permissions#webfetch) verwendet.

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

| Eigenschaft               | Typ        | Standard    | Beschreibung                                                                                                                                                                                                                                                                                                                          |
| :------------------------ | :--------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowedDomains`          | `string[]` | `[]`        | Domänennamen, auf die Sandbox-Prozesse zugreifen können                                                                                                                                                                                                                                                                               |
| `deniedDomains`           | `string[]` | `[]`        | Domänennamen, auf die Sandbox-Prozesse nicht zugreifen können. Hat Vorrang vor `allowedDomains`                                                                                                                                                                                                                                       |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Nur verwaltete Einstellungen. Wenn in [verwalteten Einstellungen](/docs/de/permissions#managed-settings) gesetzt, werden nur `allowedDomains`-Einträge aus verwalteten Einstellungen berücksichtigt und Einträge aus Benutzer-, Projekt- oder lokalen Einstellungen werden ignoriert. Hat keine Auswirkung, wenn über SDK-Optionen gesetzt |
| `allowLocalBinding`       | `boolean`  | `false`     | Erlauben Sie Prozessen, sich an lokale Ports zu binden (z. B. für Dev-Server)                                                                                                                                                                                                                                                         |
| `allowUnixSockets`        | `string[]` | `[]`        | Unix-Socket-Pfade, auf die Prozesse zugreifen können (z. B. Docker-Socket)                                                                                                                                                                                                                                                            |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Erlauben Sie Zugriff auf alle Unix-Sockets                                                                                                                                                                                                                                                                                            |
| `httpProxyPort`           | `number`   | `undefined` | HTTP-Proxy-Port für Netzwerkanfragen                                                                                                                                                                                                                                                                                                  |
| `socksProxyPort`          | `number`   | `undefined` | SOCKS-Proxy-Port für Netzwerkanfragen                                                                                                                                                                                                                                                                                                 |

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

Wenn `allowUnsandboxedCommands` aktiviert ist, kann das Modell anfordern, Befehle außerhalb der Sandbox auszuführen, indem es `dangerouslyDisableSandbox: true` in der Tool-Eingabe setzt. Diese Anfragen fallen auf das bestehende Berechtigungssystem zurück, was bedeutet, dass Ihr `canUseTool`-Handler aufgerufen wird, sodass Sie benutzerdefinierte Autorisierungslogik implementieren können. Im folgenden Beispiel steht `isCommandAuthorized` für eine Autorisierungsprüfung, die Sie definieren.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Eine statische Liste von Befehlen, die immer automatisch die Sandbox umgehen (z. B. `['docker']`). Das Modell hat keine Kontrolle darüber.
  * `allowUnsandboxedCommands`: Lässt das Modell zur Laufzeit entscheiden, ob es die Ausführung außerhalb der Sandbox anfordert, indem es `dangerouslyDisableSandbox: true` in der Tool-Eingabe setzt.
</Note>

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

Dieses Muster ermöglicht es Ihnen:

* **Modell-Anfragen prüfen:** Protokollieren Sie, wenn das Modell unsandboxed Ausführung anfordert
* **Allowlists implementieren:** Nur bestimmte Befehle dürfen unsandboxed ausgeführt werden
* **Genehmigungsworkflows hinzufügen:** Erfordern Sie explizite Autorisierung für privilegierte Operationen

<Warning>
  Befehle, die mit `dangerouslyDisableSandbox: true` ausgeführt werden, haben vollständigen Systemzugriff. Stellen Sie sicher, dass Ihr `canUseTool`-Handler diese Anfragen sorgfältig validiert.

  Wenn `permissionMode` auf `bypassPermissions` gesetzt ist und `allowUnsandboxedCommands` aktiviert ist, kann das Modell autonom Befehle außerhalb der Sandbox ausführen, ohne dass Genehmigungsaufforderungen erforderlich sind (eine explizite [`ask`-Regel](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) erzwingt immer noch eine). Diese Kombination ermöglicht dem Modell effektiv, die Sandbox-Isolierung stillschweigend zu verlassen.
</Warning>

<h2 id="see-also">
  Siehe auch
</h2>

* [SDK-Übersicht](/docs/de/agent-sdk/overview) - Allgemeine SDK-Konzepte
* [Python SDK-Referenz](/docs/de/agent-sdk/python) - Python SDK-Dokumentation
* [CLI-Referenz](/docs/de/cli-reference) - Befehlszeilenschnittstelle
* [Häufige Workflows](/docs/de/common-workflows) - Schritt-für-Schritt-Anleitungen
