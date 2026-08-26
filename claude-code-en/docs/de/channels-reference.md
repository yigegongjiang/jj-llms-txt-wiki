> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Channels-Referenz

> Erstellen Sie einen MCP-Server, der Webhooks, Benachrichtigungen und Chat-Nachrichten in eine Claude Code-Sitzung pusht. Referenz für den Channel-Vertrag: Funktionsdeklaration, Benachrichtigungsereignisse, Antwort-Tools, Sender-Gating und Berechtigungsweitergabe.

<Note>
  Channels befinden sich in [Research Preview](/docs/de/channels#research-preview). Team- und Enterprise-Organisationen müssen [diese explizit aktivieren](/docs/de/channels#enterprise-controls).
</Note>

Ein Channel ist ein MCP-Server, der Ereignisse in eine Claude Code-Sitzung pusht, damit Claude auf Dinge reagieren kann, die außerhalb des Terminals geschehen.

Sie können einen unidirektionalen oder bidirektionalen Channel erstellen. Unidirektionale Channels leiten Benachrichtigungen, Webhooks oder Überwachungsereignisse weiter, auf die Claude reagieren kann. Bidirektionale Channels wie Chat-Brücken [stellen auch ein Antwort-Tool zur Verfügung](#expose-a-reply-tool), damit Claude Nachrichten zurücksendet. Ein Channel mit einem vertrauenswürdigen Sender-Pfad kann sich auch für [Berechtigungsprompts weitergeben](#relay-permission-prompts) entscheiden, damit Sie die Tool-Nutzung remote genehmigen oder ablehnen können.

Diese Seite behandelt:

* [Übersicht](#overview): wie Channels funktionieren
* [Was Sie benötigen](#what-you-need): Anforderungen und allgemeine Schritte
* [Beispiel: Webhook-Empfänger erstellen](#example-build-a-webhook-receiver): eine minimale unidirektionale Anleitung
* [Server-Optionen](#server-options): die Constructor-Felder
* [Benachrichtigungsformat](#notification-format): die Event-Payload und das Lieferverhalten
* [Antwort-Tool bereitstellen](#expose-a-reply-tool): Claude Nachrichten zurücksendet
* [Eingehende Nachrichten gaten](#gate-inbound-messages): Sender-Überprüfungen zur Verhinderung von Prompt-Injection
* [Berechtigungsprompts weitergeben](#relay-permission-prompts): Tool-Genehmigungsprompts an Remote-Channels weiterleiten

Um einen vorhandenen Channel zu verwenden, anstatt einen zu erstellen, siehe [Channels](/docs/de/channels). Telegram, Discord, iMessage und fakechat sind in der Research Preview enthalten.

<h2 id="overview">
  Übersicht
</h2>

Ein Channel ist ein [MCP](https://modelcontextprotocol.io)-Server, der auf demselben Computer wie Claude Code ausgeführt wird. Claude Code startet ihn als Unterprozess und kommuniziert über stdio. Ihr Channel-Server ist die Brücke zwischen externen Systemen und der Claude Code-Sitzung:

* **Chat-Plattformen** (Telegram, Discord): Ihr Plugin läuft lokal und fragt die API der Plattform nach neuen Nachrichten ab. Wenn jemand Ihrem Bot eine Direktnachricht sendet, empfängt das Plugin die Nachricht und leitet sie an Claude weiter. Keine URL zum Bereitstellen erforderlich.
* **Webhooks** (CI, Überwachung): Ihr Server lauscht auf einem lokalen HTTP-Port. Externe Systeme POSTen an diesen Port, und Ihr Server pusht die Payload an Claude.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" alt="Architekturdiagramm, das externe Systeme zeigt, die sich mit Ihrem lokalen Channel-Server verbinden, der über stdio mit Claude Code kommuniziert" width="600" height="220" data-path="images/channel-architecture.svg" />

<h2 id="what-you-need">
  Was Sie benötigen
</h2>

Die einzige harte Anforderung ist das [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk)-Paket und eine Node.js-kompatible Laufzeit. [Bun](https://bun.sh), [Node](https://nodejs.org) und [Deno](https://deno.com) funktionieren alle. Die vorgefertigten Plugins in der Research Preview verwenden Bun, aber Ihr Channel muss das nicht.

Ihr Server muss:

1. Die `claude/channel`-Funktionalität deklarieren, damit Claude Code einen Benachrichtigungslistener registriert
2. `notifications/claude/channel`-Ereignisse emittieren, wenn etwas geschieht
3. Sich über [stdio-Transport](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) verbinden (Claude Code startet Ihren Server als Unterprozess)

Die Abschnitte [Server-Optionen](#server-options) und [Benachrichtigungsformat](#notification-format) behandeln jede dieser Punkte im Detail. Siehe [Beispiel: Webhook-Empfänger erstellen](#example-build-a-webhook-receiver) für eine vollständige Anleitung.

Während der Research Preview befinden sich benutzerdefinierte Channels nicht auf der [genehmigten Allowlist](/docs/de/channels#supported-channels). Verwenden Sie `--dangerously-load-development-channels` zum lokalen Testen. Siehe [Testen während der Research Preview](#test-during-the-research-preview) für Details.

<h2 id="example-build-a-webhook-receiver">
  Beispiel: Webhook-Empfänger erstellen
</h2>

Diese Anleitung erstellt einen Single-File-Server, der auf HTTP-Anfragen lauscht und diese in Ihre Claude Code-Sitzung weiterleitet. Am Ende kann alles, das einen HTTP POST senden kann, wie eine CI-Pipeline, eine Überwachungsbenachrichtigung oder ein `curl`-Befehl, Ereignisse an Claude pushen.

Dieses Beispiel verwendet [Bun](https://bun.sh) als Laufzeit für seinen integrierten HTTP-Server und TypeScript-Unterstützung. Sie können stattdessen [Node](https://nodejs.org) oder [Deno](https://deno.com) verwenden; die einzige Anforderung ist das [MCP SDK](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Erstellen Sie das Projekt">
    Erstellen Sie ein neues Verzeichnis und installieren Sie das MCP SDK:

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk
    ```
  </Step>

  <Step title="Schreiben Sie den Channel-Server">
    Erstellen Sie eine Datei namens `webhook.ts`. Dies ist Ihr gesamter Channel-Server: Er verbindet sich mit Claude Code über stdio und lauscht auf HTTP POSTs auf Port 8788. Wenn eine Anfrage ankommt, pusht er den Body als Channel-Ereignis an Claude.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Erstellen Sie den MCP-Server und deklarieren Sie ihn als Channel
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // dieser Schlüssel macht ihn zu einem Channel — Claude Code registriert einen Listener dafür
        capabilities: { experimental: { 'claude/channel': {} } },
        // hinzugefügt zu Claudes System-Prompt, damit es weiß, wie diese Ereignisse zu behandeln sind
        instructions: 'Events from the webhook channel arrive as <channel source="webhook" ...>. They are one-way: read them and act, no reply expected.',
      },
    )

    // Verbinden Sie sich mit Claude Code über stdio (Claude Code startet diesen Prozess)
    await mcp.connect(new StdioServerTransport())

    // Starten Sie einen HTTP-Server, der jeden POST an Claude weiterleitet
    Bun.serve({
      port: 8788,  // jeder offene Port funktioniert
      // nur localhost: nichts außerhalb dieser Maschine kann POSTen
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // wird zum Body des <channel>-Tags
            // jeder Schlüssel wird zu einem Tag-Attribut, z.B. <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    Die Datei macht drei Dinge in Reihenfolge:

    * **Server-Konfiguration**: erstellt den MCP-Server mit `claude/channel` in seinen Funktionalitäten, was Claude Code mitteilt, dass dies ein Channel ist. Die [`instructions`](#server-options)-Zeichenkette geht in Claudes System-Prompt: teilen Sie Claude mit, welche Ereignisse zu erwarten sind, ob es antworten soll, und wie Antworten weitergeleitet werden sollen, falls ja.
    * **Stdio-Verbindung**: verbindet sich mit Claude Code über stdin/stdout. Dies ist Standard für jeden [MCP-Server](https://modelcontextprotocol.io/docs/concepts/transports#standard-io): Claude Code startet ihn als Unterprozess.
    * **HTTP-Listener**: startet einen lokalen Webserver auf Port 8788. Jeder POST-Body wird über `mcp.notification()` als Channel-Ereignis an Claude weitergeleitet. Der `content` wird zum Event-Body, und jeder `meta`-Eintrag wird zu einem Attribut auf dem `<channel>`-Tag. Der Listener benötigt Zugriff auf die `mcp`-Instanz, daher läuft er im selben Prozess. Sie könnten ihn für ein größeres Projekt in separate Module aufteilen.
  </Step>

  <Step title="Registrieren Sie Ihren Server bei Claude Code">
    Fügen Sie den Server zu Ihrer MCP-Konfiguration hinzu, damit Claude Code weiß, wie er zu starten ist. Für eine Projekt-Level `.mcp.json` im selben Verzeichnis verwenden Sie einen relativen Pfad. Für Benutzer-Level-Konfiguration in `~/.claude.json` verwenden Sie den vollständigen absoluten Pfad, damit der Server von jedem Projekt aus gefunden werden kann:

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code liest Ihre MCP-Konfiguration beim Start und startet jeden Server als Unterprozess.
  </Step>

  <Step title="Testen Sie es">
    Während der Research Preview befinden sich benutzerdefinierte Channels nicht auf der Allowlist, daher starten Sie Claude Code mit dem Development-Flag:

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    Beim ersten Mal, wenn Sie eine Sitzung in diesem Projekt starten, fragt Claude Code um Zustimmung, bevor der neue Server aus `.mcp.json` verwendet wird. Der Dialog meldet "New MCP server found in this project: webhook". Wählen Sie **Use this MCP server** aus, um fortzufahren.

    Wenn Claude Code startet, liest es Ihre MCP-Konfiguration, startet Ihre `webhook.ts` als Unterprozess, und der HTTP-Listener startet automatisch auf dem konfigurierten Port (8788 in diesem Beispiel). Sie müssen den Server nicht selbst ausführen.

    Eine schwache Benachrichtigung unter dem Startup-Banner bestätigt, dass der Channel registriert ist: `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Wenn Sie "blocked by org policy" sehen, muss Ihr Organisations-Admin [Channels aktivieren](/docs/de/channels#enterprise-controls) zuerst.

    Simulieren Sie in einem separaten Terminal einen Webhook, indem Sie einen HTTP POST mit einer Nachricht an Ihren Server senden. Dieses Beispiel sendet eine CI-Fehlerbenachrichtigung an Port 8788 (oder welchen Port Sie konfiguriert haben):

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    Die Payload kommt in Ihrer Claude Code-Sitzung als `<channel>`-Tag an:

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    In Ihrem Claude Code-Terminal sehen Sie, dass Claude die Nachricht empfängt und anfängt zu antworten: Dateien lesen, Befehle ausführen oder was auch immer die Nachricht erfordert. Dies ist ein unidirektionaler Channel, daher handelt Claude in Ihrer Sitzung, sendet aber nichts über den Webhook zurück. Um Antworten hinzuzufügen, siehe [Antwort-Tool bereitstellen](#expose-a-reply-tool).

    Wenn das Ereignis nicht ankommt, hängt die Diagnose davon ab, was `curl` zurückgegeben hat:

    * **`curl` erfolgreich, aber nichts erreicht Claude**: führen Sie `/mcp` in Ihrer Sitzung aus, um den Status des Servers zu überprüfen. "Failed to connect" bedeutet normalerweise einen Abhängigkeits- oder Importfehler in Ihrer Serverdatei; überprüfen Sie das Debug-Log unter `~/.claude/debug/<session-id>.txt` für die stderr-Spur.
    * **`curl` schlägt mit "connection refused" fehl**: der Port ist entweder noch nicht gebunden oder ein veralteter Prozess aus einem früheren Lauf hält ihn. `lsof -i :<port>` zeigt, was lauscht; `kill` den veralteten Prozess, bevor Sie Ihre Sitzung neu starten.
  </Step>
</Steps>

Der [fakechat-Server](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) erweitert dieses Muster mit einer Web-UI, Dateianhängen und einem Antwort-Tool für bidirektionalen Chat.

<h2 id="test-during-the-research-preview">
  Testen während der Research Preview
</h2>

Während der Research Preview muss sich jeder Channel auf der [genehmigten Allowlist](/docs/de/channels#research-preview) befinden, um sich zu registrieren. Das Development-Flag umgeht die Allowlist für spezifische Einträge nach einer Bestätigungsaufforderung. Dieses Beispiel zeigt beide Eintragstypen:

```bash theme={null}
# Testen eines Plugins, das Sie entwickeln
claude --dangerously-load-development-channels plugin:yourplugin@yourmarketplace

# Testen eines bloßen .mcp.json-Servers (noch kein Plugin-Wrapper)
claude --dangerously-load-development-channels server:webhook
```

Der Bypass ist pro Eintrag. Das Kombinieren dieses Flags mit `--channels` erweitert den Bypass nicht auf die `--channels`-Einträge. Während der Research Preview ist die genehmigte Allowlist von Anthropic kuratiert, daher bleibt Ihr Channel auf dem Development-Flag, während Sie ihn erstellen und testen.

<Note>
  Dieses Flag überspringt nur die Allowlist. Die `channelsEnabled`-Organisationsrichtlinie gilt weiterhin. Verwenden Sie es nicht, um Channels aus nicht vertrauenswürdigen Quellen auszuführen.
</Note>

<h2 id="server-options">
  Server-Optionen
</h2>

Ein Channel setzt diese Optionen im [`Server`](https://modelcontextprotocol.io/docs/concepts/servers)-Constructor. Die Felder `instructions` und `capabilities.tools` sind [Standard-MCP](https://modelcontextprotocol.io/docs/concepts/servers); `capabilities.experimental['claude/channel']` und `capabilities.experimental['claude/channel/permission']` sind die Channel-spezifischen Ergänzungen:

| Feld                                                     | Typ      | Beschreibung                                                                                                                                                                                                                                                                                                             |
| :------------------------------------------------------- | :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object` | Erforderlich. Immer `{}`. Das Vorhandensein registriert den Benachrichtigungslistener.                                                                                                                                                                                                                                   |
| `capabilities.experimental['claude/channel/permission']` | `object` | Optional. Immer `{}`. Deklariert, dass dieser Channel Berechtigungsweitergabeanfragen empfangen kann. Wenn deklariert, leitet Claude Code Tool-Genehmigungsprompts an Ihren Channel weiter, damit Sie diese remote genehmigen oder ablehnen können. Siehe [Berechtigungsprompts weitergeben](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object` | Nur bidirektional. Immer `{}`. Standard-MCP-Tool-Funktionalität. Siehe [Antwort-Tool bereitstellen](#expose-a-reply-tool).                                                                                                                                                                                               |
| `instructions`                                           | `string` | Empfohlen. Hinzugefügt zu Claudes System-Prompt. Teilen Sie Claude mit, welche Ereignisse zu erwarten sind, was die `<channel>`-Tag-Attribute bedeuten, ob es antworten soll, und wenn ja, welches Tool zu verwenden ist und welches Attribut zurückzugeben ist (wie `chat_id`).                                         |

Um einen unidirektionalen Channel zu erstellen, lassen Sie `capabilities.tools` weg. Dieses Beispiel zeigt ein bidirektionales Setup mit der Channel-Funktionalität, Tools und Anweisungen:

```ts theme={null}
import { Server } from '@modelcontextprotocol/sdk/server/index.js'

const mcp = new Server(
  { name: 'your-channel', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },  // registriert den Channel-Listener
      tools: {},  // weglassen für unidirektionale Channels
    },
    // hinzugefügt zu Claudes System-Prompt, damit es weiß, wie Ihre Ereignisse zu behandeln sind
    instructions: 'Messages arrive as <channel source="your-channel" ...>. Reply with the reply tool.',
  },
)
```

Um ein Ereignis zu pushen, rufen Sie `mcp.notification()` mit der Methode `notifications/claude/channel` auf. Die Parameter sind im nächsten Abschnitt.

<h2 id="notification-format">
  Benachrichtigungsformat
</h2>

Ihr Server emittiert `notifications/claude/channel` mit zwei Parametern:

| Feld      | Typ                      | Beschreibung                                                                                                                                                                                                                                                                                                        |
| :-------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `content` | `string`                 | Der Event-Body. Wird als Body des `<channel>`-Tags bereitgestellt.                                                                                                                                                                                                                                                  |
| `meta`    | `Record<string, string>` | Optional. Jeder Eintrag wird zu einem Attribut auf dem `<channel>`-Tag für Routing-Kontext wie Chat-ID, Sendername oder Benachrichtigungsschweregrad. Schlüssel müssen Bezeichner sein: nur Buchstaben, Ziffern und Unterstriche. Schlüssel mit Bindestrichen oder anderen Zeichen werden stillschweigend gelöscht. |

Ihr Server pusht Ereignisse durch Aufrufen von `mcp.notification()` auf der `Server`-Instanz. Dieses Beispiel pusht eine CI-Fehlerbenachrichtigung mit zwei Meta-Schlüsseln:

```ts theme={null}
await mcp.notification({
  method: 'notifications/claude/channel',
  params: {
    content: 'build failed on main: https://ci.example.com/run/1234',
    meta: { severity: 'high', run_id: '1234' },
  },
})
```

Das Ereignis kommt in Claudes Kontext in einem `<channel>`-Tag an. Das `source`-Attribut wird automatisch aus dem konfigurierten Namen Ihres Servers gesetzt:

```text theme={null}
<channel source="your-channel" severity="high" run_id="1234">
build failed on main: https://ci.example.com/run/1234
</channel>
```

Benachrichtigungen werden nicht bestätigt. Das `await` auf `mcp.notification()` wird aufgelöst, wenn die Nachricht in den Transport geschrieben wird, nicht wenn Claude sie verarbeitet hat. Wenn die Sitzung Ihren Server nicht als Channel geladen hat oder die Organisationsrichtlinie ihn blockiert, werden Ereignisse stillschweigend ohne Fehler an Ihren Server gelöscht.

Wenn Sie eine Lieferbestätigung benötigen, verfolgen Sie den Event-Status in Ihrem Server und stellen Sie ein [Antwort-Tool](#expose-a-reply-tool) zur Verfügung, das Claude aufrufen kann, um den Status zurück zu melden.

Ereignisse werden in die Sitzung eingereiht und in Reihenfolge verarbeitet. Wenn mehrere Benachrichtigungen ankommen, während Claude beschäftigt ist, werden sie beim nächsten Zug zusammen bereitgestellt und Claude behandelt sie als Gruppe. Um unabhängige Event-Streams gleichzeitig zu verarbeiten, führen Sie separate Sitzungen aus.

<h2 id="expose-a-reply-tool">
  Antwort-Tool bereitstellen
</h2>

Wenn Ihr Channel bidirektional ist, wie eine Chat-Brücke statt eines Alert-Forwarders, stellen Sie ein Standard-[MCP-Tool](https://modelcontextprotocol.io/docs/concepts/tools) zur Verfügung, das Claude aufrufen kann, um Nachrichten zurückzusenden. Nichts an der Tool-Registrierung ist Channel-spezifisch. Ein Antwort-Tool hat drei Komponenten:

1. Ein `tools: {}`-Eintrag in Ihren `Server`-Constructor-Funktionalitäten, damit Claude Code das Tool entdeckt
2. Tool-Handler, die das Tool-Schema definieren und die Versendungslogik implementieren
3. Eine `instructions`-Zeichenkette in Ihrem `Server`-Constructor, die Claude mitteilt, wann und wie das Tool aufgerufen wird

Um diese zum [Webhook-Empfänger oben](#example-build-a-webhook-receiver) hinzuzufügen:

<Steps>
  <Step title="Aktivieren Sie die Tool-Entdeckung">
    In Ihrem `Server`-Constructor in `webhook.ts` fügen Sie `tools: {}` zu den Funktionalitäten hinzu, damit Claude Code weiß, dass Ihr Server Tools anbietet:

    ```ts theme={null}
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},  // aktiviert die Tool-Entdeckung
    },
    ```
  </Step>

  <Step title="Registrieren Sie das Antwort-Tool">
    Fügen Sie Folgendes zu `webhook.ts` hinzu. Der `import` geht oben in der Datei mit Ihren anderen Importen; die zwei Handler gehen zwischen dem `Server`-Constructor und `mcp.connect()`. Dies registriert ein `reply`-Tool, das Claude mit einer `chat_id` und `text` aufrufen kann:

    ```ts theme={null}
    // Fügen Sie diesen Import oben in webhook.ts hinzu
    import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

    // Claude fragt dies beim Start ab, um zu entdecken, welche Tools Ihr Server anbietet
    mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [{
        name: 'reply',
        description: 'Send a message back over this channel',
        // inputSchema teilt Claude mit, welche Argumente zu übergeben sind
        inputSchema: {
          type: 'object',
          properties: {
            chat_id: { type: 'string', description: 'The conversation to reply in' },
            text: { type: 'string', description: 'The message to send' },
          },
          required: ['chat_id', 'text'],
        },
      }],
    }))

    // Claude ruft dies auf, wenn es ein Tool aufrufen möchte
    mcp.setRequestHandler(CallToolRequestSchema, async req => {
      if (req.params.name === 'reply') {
        const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
        // send() ist Ihre Ausgangsrichtung: POST an Ihre Chat-Plattform, oder für lokales
        // Testen die SSE-Übertragung, die im vollständigen Beispiel unten gezeigt wird.
        send(`Reply to ${chat_id}: ${text}`)
        return { content: [{ type: 'text', text: 'sent' }] }
      }
      throw new Error(`unknown tool: ${req.params.name}`)
    })
    ```
  </Step>

  <Step title="Aktualisieren Sie die Anweisungen">
    Aktualisieren Sie die `instructions`-Zeichenkette in Ihrem `Server`-Constructor, damit Claude weiß, dass Antworten über das Tool zurückgeleitet werden. Dieses Beispiel teilt Claude mit, `chat_id` aus dem eingehenden Tag zu übergeben:

    ```ts theme={null}
    instructions: 'Messages arrive as <channel source="webhook" chat_id="...">. Reply with the reply tool, passing the chat_id from the tag.'
    ```
  </Step>
</Steps>

Hier ist die vollständige `webhook.ts` mit bidirektionaler Unterstützung. Ausgehende Antworten streamen über `GET /events` mit [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE), daher kann `curl -N localhost:8788/events` sie live beobachten; eingehender Chat kommt auf `POST /` an:

```ts title="Full webhook.ts with reply tool' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

// --- Ausgangsrichtung: schreiben Sie an alle curl -N-Listener auf /events ---
// Eine echte Brücke würde stattdessen an Ihre Chat-Plattform POSTen.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},
    },
    instructions: 'Messages arrive as <channel source="webhook" chat_id="...">. Reply with the reply tool, passing the chat_id from the tag.',
  },
)

mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Send a message back over this channel',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'The conversation to reply in' },
        text: { type: 'string', description: 'The message to send' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

await mcp.connect(new StdioServerTransport())

let nextId = 1
Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // don't close idle SSE streams
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: SSE stream so curl -N can watch Claude's replies live
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // so curl shows something immediately
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // POST: forward to Claude as a channel event
    const body = await req.text()
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: {
        content: body,
        meta: { chat_id, path: url.pathname, method: req.method },
      },
    })
    return new Response('ok')
  },
})
```

Der [fakechat-Server](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) zeigt ein vollständigeres Beispiel mit Dateianhängen und Nachrichtenbearbeitung.

<h2 id="gate-inbound-messages">
  Eingehende Nachrichten gaten
</h2>

Ein ungegatterter Channel ist ein Prompt-Injection-Vektor. Jeder, der Ihren Endpunkt erreichen kann, kann Text vor Claude platzieren. Ein Channel, der auf einer Chat-Plattform oder einem öffentlichen Endpunkt lauscht, benötigt eine echte Sender-Überprüfung, bevor er etwas emittiert.

Überprüfen Sie den Sender gegen eine Allowlist, bevor Sie `mcp.notification()` aufrufen. Dieses Beispiel löscht jede Nachricht von einem Sender, der nicht in der Menge ist:

```ts theme={null}
const allowed = new Set(loadAllowlist())  // from your access.json or equivalent

// inside your message handler, before emitting:
if (!allowed.has(message.from.id)) {  // sender, not room
  return  // drop silently
}
await mcp.notification({ ... })
```

Gaten Sie auf der Identität des Senders, nicht auf der Chat- oder Raumidentität: `message.from.id` im Beispiel, nicht `message.chat.id`. In Gruppenchats unterscheiden sich diese, und das Gaten auf dem Raum würde jedem in einer genehmigten Gruppe erlauben, Nachrichten in die Sitzung einzuspritzen.

Die [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram)- und [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord)-Channels gaten auf die gleiche Weise auf einer Sender-Allowlist. Sie bootstrappen die Liste durch Pairing: Der Benutzer sendet dem Bot eine Direktnachricht, der Bot antwortet mit einem Pairing-Code, der Benutzer genehmigt ihn in seiner Claude Code-Sitzung, und seine Plattform-ID wird hinzugefügt. Siehe eine der Implementierungen für den vollständigen Pairing-Flow. Der [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage)-Channel verfolgt einen anderen Ansatz: Er erkennt die eigenen Adressen des Benutzers aus der Messages-Datenbank beim Start und lässt sie automatisch durch, wobei andere Sender nach Handle hinzugefügt werden.

<h2 id="relay-permission-prompts">
  Berechtigungsprompts weitergeben
</h2>

Wenn Claude ein Tool aufruft, das Genehmigung benötigt, öffnet sich der lokale Terminal-Dialog und die Sitzung wartet. Ein bidirektionaler Channel kann sich dafür entscheiden, denselben Prompt parallel zu empfangen und ihn an Sie auf einem anderen Gerät weiterzuleiten. Beide bleiben aktiv: Sie können im Terminal oder auf Ihrem Telefon antworten, und Claude Code wendet die Antwort an, die zuerst ankommt, und schließt die andere.

Die Weitergabe deckt Tool-Nutzungsgenehmigungen wie `Bash`, `Write` und `Edit` ab. Projekt-Vertrauen und MCP-Server-Zustimmungsdialoge werden nicht weitergeleitet; diese erscheinen nur im lokalen Terminal.

<h3 id="how-relay-works">
  Wie die Weitergabe funktioniert
</h3>

Wenn ein Berechtigungsprompt öffnet, hat die Weitergabeschleife vier Schritte:

1. Claude Code generiert eine kurze Request-ID und benachrichtigt Ihren Server
2. Ihr Server leitet den Prompt und die ID an Ihre Chat-App weiter
3. Der Remote-Benutzer antwortet mit ja oder nein und dieser ID
4. Ihr eingehender Handler analysiert die Antwort in ein Urteil, und Claude Code wendet es nur an, wenn die ID einer offenen Anfrage entspricht

Der lokale Terminal-Dialog bleibt während all dessen offen. Wenn jemand am Terminal antwortet, bevor das Remote-Urteil ankommt, wird diese Antwort stattdessen angewendet und die ausstehende Remote-Anfrage wird gelöscht.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" alt="Sequenzdiagramm: Claude Code sendet eine permission_request-Benachrichtigung an den Channel-Server, der Server formatiert und sendet den Prompt an die Chat-App, der Mensch antwortet mit einem Urteil, und der Server analysiert diese Antwort in eine Berechtigungsbenachrichtigung zurück an Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<h3 id="permission-request-fields">
  Berechtigungsanfrage-Felder
</h3>

Die ausgehende Benachrichtigung von Claude Code ist `notifications/claude/channel/permission_request`. Wie die [Channel-Benachrichtigung](#notification-format) ist der Transport Standard-MCP, aber die Methode und das Schema sind Claude Code-Erweiterungen. Das `params`-Objekt hat vier String-Felder, die Ihr Server in den ausgehenden Prompt formatiert:

| Feld            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `request_id`    | Fünf Kleinbuchstaben aus `a`-`z` ohne `l`, damit es nie als `1` oder `I` gelesen wird, wenn es auf einem Telefon eingegeben wird. Fügen Sie es in Ihren ausgehenden Prompt ein, damit es in der Antwort wiederholt werden kann. Claude Code akzeptiert nur ein Urteil, das eine ID trägt, die es ausgestellt hat. Der lokale Terminal-Dialog zeigt diese ID nicht an, daher ist Ihr ausgehender Handler die einzige Möglichkeit, sie zu erfahren. |
| `tool_name`     | Name des Tools, das Claude verwenden möchte, zum Beispiel `Bash` oder `Write`.                                                                                                                                                                                                                                                                                                                                                                    |
| `description`   | Menschenlesbarer Zusammenfassung dessen, was dieser spezifische Tool-Aufruf tut, derselbe Text, den der lokale Terminal-Dialog zeigt. Für einen Bash-Aufruf ist dies Claudes Beschreibung des Befehls oder der Befehl selbst, wenn keine gegeben wurde.                                                                                                                                                                                           |
| `input_preview` | Die Argumente des Tools als JSON-Zeichenkette, gekürzt auf 200 Zeichen. Für Bash ist dies der Befehl; für Write ist es der Dateipfad und ein Präfix des Inhalts. Lassen Sie es aus Ihrem Prompt weg, wenn Sie nur Platz für eine einzeilige Nachricht haben. Ihr Server entscheidet, was angezeigt wird.                                                                                                                                          |

Das Urteil, das Ihr Server zurücksendet, ist `notifications/claude/channel/permission` mit zwei Feldern: `request_id`, das die obige ID wiederholt, und `behavior`, das auf `'allow'` oder `'deny'` gesetzt ist. Allow lässt den Tool-Aufruf fortfahren; deny lehnt ihn ab, dasselbe wie das Antworten mit Nein im lokalen Dialog. Weder das Urteil beeinflusst zukünftige Aufrufe.

<h3 id="add-relay-to-a-chat-bridge">
  Weitergabe zu einer Chat-Brücke hinzufügen
</h3>

Das Hinzufügen von Berechtigungsweitergabe zu einem bidirektionalen Channel erfordert drei Komponenten:

1. Ein `claude/channel/permission: {}`-Eintrag unter `experimental`-Funktionalitäten in Ihrem `Server`-Constructor, damit Claude Code weiß, dass Prompts weitergeleitet werden sollen
2. Ein Benachrichtigungshandler für `notifications/claude/channel/permission_request`, der den Prompt formatiert und ihn über Ihre Plattform-API sendet
3. Eine Überprüfung in Ihrem eingehenden Nachrichtenhandler, die `yes <id>` oder `no <id>` erkennt und stattdessen eine `notifications/claude/channel/permission`-Urteilsbenachrichtigung emittiert, anstatt den Text an Claude weiterzuleiten

Deklarieren Sie die Funktionalität nur, wenn Ihr Channel [den Sender authentifiziert](#gate-inbound-messages), da jeder, der über Ihren Channel antworten kann, Tool-Nutzung in Ihrer Sitzung genehmigen oder ablehnen kann.

Um diese zu einer bidirektionalen Chat-Brücke wie der in [Antwort-Tool bereitstellen](#expose-a-reply-tool) zusammengestellten hinzuzufügen:

<Steps>
  <Step title="Deklarieren Sie die Berechtigungsfunktionalität">
    In Ihrem `Server`-Constructor fügen Sie `claude/channel/permission: {}` neben `claude/channel` unter `experimental` hinzu:

    ```ts theme={null}
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opt in to permission relay
      },
      tools: {},
    },
    ```
  </Step>

  <Step title="Behandeln Sie die eingehende Anfrage">
    Registrieren Sie einen Benachrichtigungshandler zwischen Ihrem `Server`-Constructor und `mcp.connect()`. Claude Code ruft ihn mit den [vier Anfrage-Feldern](#permission-request-fields) auf, wenn ein Berechtigungsdialog öffnet. Ihr Handler formatiert den Prompt für Ihre Plattform und enthält Anweisungen zum Antworten mit der ID:

    ```ts theme={null}
    import { z } from 'zod'

    // setNotificationHandler leitet nach z.literal auf dem method-Feld weiter,
    // daher ist dieses Schema sowohl der Validator als auch der Dispatch-Schlüssel
    const PermissionRequestSchema = z.object({
      method: z.literal('notifications/claude/channel/permission_request'),
      params: z.object({
        request_id: z.string(),     // five lowercase letters, include verbatim in your prompt
        tool_name: z.string(),      // e.g. "Bash", "Write"
        description: z.string(),    // human-readable summary of this call
        input_preview: z.string(),  // tool args as JSON, truncated to ~200 chars
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() ist Ihre Ausgangsrichtung: POST an Ihre Chat-Plattform, oder für lokales
      // Testen die SSE-Übertragung, die im vollständigen Beispiel unten gezeigt wird.
      send(
        `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
        // die ID in der Anweisung ist das, was Ihr eingehender Handler in Schritt 3 analysiert
        `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
      )
    })
    ```
  </Step>

  <Step title="Fangen Sie das Urteil in Ihrem eingehenden Handler ab">
    Ihr eingehender Handler ist die Schleife oder der Callback, der Nachrichten von Ihrer Plattform empfängt: derselbe Ort, an dem Sie [auf Sender gaten](#gate-inbound-messages) und `notifications/claude/channel` emittieren, um Chat an Claude weiterzuleiten. Fügen Sie eine Überprüfung vor dem Chat-Weiterleitungsaufruf hinzu, die das Urteilsformat erkennt und stattdessen die Berechtigungsbenachrichtigung emittiert.

    Der Regex entspricht dem ID-Format, das Claude Code generiert: fünf Buchstaben, nie `l`. Das `/i`-Flag toleriert Telefon-Autokorrektur, die die Antwort großschreibt; kleinschreiben Sie die erfasste ID, bevor Sie sie zurücksendet.

    ```ts theme={null}
    // matches "y abcde", "yes abcde", "n abcde", "no abcde"
    // [a-km-z] is the ID alphabet Claude Code uses (lowercase, skips 'l')
    // /i tolerates phone autocorrect; lowercase the capture before sending
    const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i

    async function onInbound(message: PlatformMessage) {
      if (!allowed.has(message.from.id)) return  // gate on sender first

      const m = PERMISSION_REPLY_RE.exec(message.text)
      if (m) {
        // m[1] is the verdict word, m[2] is the request ID
        // emit the verdict notification back to Claude Code instead of chat
        await mcp.notification({
          method: 'notifications/claude/channel/permission',
          params: {
            request_id: m[2].toLowerCase(),  // normalize in case of autocorrect caps
            behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
          },
        })
        return  // handled as verdict, don't also forward as chat
      }

      // didn't match verdict format: fall through to the normal chat path
      await mcp.notification({
        method: 'notifications/claude/channel',
        params: { content: message.text, meta: { chat_id: String(message.chat.id) } },
      })
    }
    ```
  </Step>
</Steps>

Claude Code hält auch den lokalen Terminal-Dialog offen, daher können Sie an beiden Orten antworten, und die erste Antwort, die ankommt, wird angewendet. Eine Remote-Antwort, die nicht genau dem erwarteten Format entspricht, schlägt auf eine von zwei Arten fehl, und in beiden Fällen bleibt der Dialog offen:

* **Anderes Format**: der Regex Ihres eingehenden Handlers schlägt fehl zu entsprechen, daher fällt Text wie `approve it` oder `yes` ohne ID als normale Nachricht an Claude durch.
* **Richtiges Format, falsche ID**: Ihr Server emittiert ein Urteil, aber Claude Code findet keine offene Anfrage mit dieser ID und löscht es stillschweigend.

<h3 id="full-example">
  Vollständiges Beispiel
</h3>

Die zusammengestellte `webhook.ts` unten kombiniert alle drei Erweiterungen von dieser Seite: das Antwort-Tool, Sender-Gating und Berechtigungsweitergabe. Wenn Sie hier anfangen, benötigen Sie auch die [Projekt-Setup und `.mcp.json`-Eintrag](#example-build-a-webhook-receiver) aus der anfänglichen Anleitung.

Um beide Richtungen von curl aus testbar zu machen, dient der HTTP-Listener zwei Pfaden:

* **`GET /events`**: hält einen SSE-Stream offen und pusht jede ausgehende Nachricht als `data:`-Zeile, daher kann `curl -N` Claudes Antworten und Berechtigungsprompts live beobachten, wenn sie ankommen.
* **`POST /`**: die eingehende Seite, derselbe Handler wie zuvor, jetzt mit der Urteilsformat-Überprüfung vor dem Chat-Weiterleitungszweig eingefügt.

```ts title="Full webhook.ts with permission relay' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'
import { z } from 'zod'

// --- Ausgangsrichtung: schreiben Sie an alle curl -N-Listener auf /events ---
// Eine echte Brücke würde stattdessen an Ihre Chat-Plattform POSTen.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

// Sender-Allowlist. Für die lokale Anleitung vertrauen wir dem einzelnen X-Sender
// Header-Wert "dev"; eine echte Brücke würde die Plattform-Benutzer-ID überprüfen.
const allowed = new Set(['dev'])

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opt in to permission relay
      },
      tools: {},
    },
    instructions:
      'Messages arrive as <channel source="webhook" chat_id="...">. ' +
      'Reply with the reply tool, passing the chat_id from the tag.',
  },
)

// --- reply tool: Claude calls this to send a message back ---
mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Send a message back over this channel',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'The conversation to reply in' },
        text: { type: 'string', description: 'The message to send' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

// --- permission relay: Claude Code (not Claude) calls this when a dialog opens
const PermissionRequestSchema = z.object({
  method: z.literal('notifications/claude/channel/permission_request'),
  params: z.object({
    request_id: z.string(),
    tool_name: z.string(),
    description: z.string(),
    input_preview: z.string(),
  }),
})

mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
  send(
    `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
    `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
  )
})

await mcp.connect(new StdioServerTransport())

// --- HTTP on :8788: GET /events streams outbound, POST routes inbound ---
const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i
let nextId = 1

Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // don't close idle SSE streams
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events: SSE stream so curl -N can watch replies and prompts live
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // so curl shows something immediately
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // everything else is inbound: gate on sender first
    const body = await req.text()
    const sender = req.headers.get('X-Sender') ?? ''
    if (!allowed.has(sender)) return new Response('forbidden', { status: 403 })

    // check for verdict format before treating as chat
    const m = PERMISSION_REPLY_RE.exec(body)
    if (m) {
      await mcp.notification({
        method: 'notifications/claude/channel/permission',
        params: {
          request_id: m[2].toLowerCase(),
          behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
        },
      })
      return new Response('verdict recorded')
    }

    // normal chat: forward to Claude as a channel event
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: { content: body, meta: { chat_id, path: url.pathname } },
    })
    return new Response('ok')
  },
})
```

Testen Sie den Urteilspfad in drei Terminals. Das erste ist Ihre Claude Code-Sitzung, gestartet mit dem [Development-Flag](#test-during-the-research-preview), damit es `webhook.ts` startet:

```bash theme={null}
claude --dangerously-load-development-channels server:webhook
```

Im zweiten streamen Sie die ausgehende Seite, damit Sie Claudes Antworten und alle Berechtigungsprompts live sehen können, wenn sie ankommen:

```bash theme={null}
curl -N localhost:8788/events
```

Im dritten senden Sie eine Nachricht, die Claude veranlasst, einen Befehl auszuführen:

```bash theme={null}
curl -d "list the files in this directory" -H "X-Sender: dev" localhost:8788
```

Das Auflisten von Dateien ist schreibgeschützt, daher führt Claude es ohne Genehmigung aus. Der Berechtigungsdialog öffnet sich, wenn Claude das `reply`-Tool aufruft, um seine Antwort zurückzusenden. Der lokale Dialog öffnet sich in Ihrem Claude Code-Terminal, und einen Moment später erscheint der Prompt für `mcp__webhook__reply` im `/events`-Stream, einschließlich der fünf-buchstabigen ID. Genehmigen Sie ihn von der Remote-Seite:

```bash theme={null}
curl -d "yes <id>" -H "X-Sender: dev" localhost:8788
```

Der lokale Dialog schließt sich, das `reply`-Tool läuft, und Claudes Antwort landet im Stream.

Die drei Channel-spezifischen Teile in dieser Datei:

* **Funktionalitäten** im `Server`-Constructor: `claude/channel` registriert den Benachrichtigungslistener, `claude/channel/permission` entscheidet sich für Berechtigungsweitergabe, `tools` lässt Claude das Antwort-Tool entdecken.
* **Ausgehende Pfade**: der `reply`-Tool-Handler ist das, was Claude für Gesprächsantworten aufruft; der `PermissionRequestSchema`-Benachrichtigungshandler ist das, was Claude Code aufruft, wenn ein Berechtigungsdialog öffnet. Beide rufen `send()` auf, um über `/events` zu übertragen, aber sie werden von verschiedenen Teilen des Systems ausgelöst.
* **HTTP-Handler**: `GET /events` hält einen SSE-Stream offen, damit curl Ausgangsrichtung live beobachten kann; `POST` ist eingehend, gatet auf dem `X-Sender`-Header. Ein `yes <id>`- oder `no <id>`-Body geht an Claude Code als Urteilsbenachrichtigung und erreicht nie Claude; alles andere wird an Claude als Channel-Ereignis weitergeleitet.

<h2 id="package-as-a-plugin">
  Als Plugin verpacken
</h2>

Um Ihren Channel installierbar und teilbar zu machen, wickeln Sie ihn in ein [Plugin](/docs/de/plugins) ein und veröffentlichen Sie ihn auf einem [Marketplace](/docs/de/plugin-marketplaces). Benutzer installieren ihn mit `/plugin install`, dann aktivieren ihn pro Sitzung mit `--channels plugin:<name>@<marketplace>`.

Ein Channel, der auf Ihrem eigenen Marketplace veröffentlicht wird, benötigt immer noch `--dangerously-load-development-channels` zum Ausführen, da er nicht auf der [genehmigten Allowlist](/docs/de/channels#supported-channels) ist. Die Standard-Allowlist ist die Channel-Plugins in `claude-plugins-official`, die Anthropic nach eigenem Ermessen kuratiert. Die [In-App-Einreichungsformulare](/docs/de/plugins#submit-your-plugin-to-the-community-marketplace) fügen Plugins zum Community-Marketplace hinzu, der nicht auf der Channel-Allowlist ist.

Wenn Sie mit einem Anthropic-Partner-Kontakt arbeiten, wenden Sie sich an diese Person, um eine offizielle Marketplace-Auflistung zu koordinieren. Bei Team- und Enterprise-Plänen kann ein Admin stattdessen Ihr Plugin in die [`allowedChannelPlugins`](/docs/de/channels#restrict-which-channel-plugins-can-run)-Liste der Organisation aufnehmen, die die Standard-Anthropic-Allowlist ersetzt.

<h2 id="see-also">
  Siehe auch
</h2>

* [Channels](/docs/de/channels) zum Installieren und Verwenden von Telegram, Discord, iMessage oder der fakechat-Demo und zum Aktivieren von Channels für eine Team- oder Enterprise-Organisation
* [Arbeitende Channel-Implementierungen](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) für vollständigen Server-Code mit Pairing-Flows, Antwort-Tools und Dateianhängen
* [MCP](/docs/de/mcp) für das zugrunde liegende Protokoll, das Channel-Server implementieren
* [Plugins](/docs/de/plugins) zum Verpacken Ihres Channels, damit Benutzer ihn mit `/plugin install` installieren können
