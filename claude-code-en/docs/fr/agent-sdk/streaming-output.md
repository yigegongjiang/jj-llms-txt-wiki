> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Diffuser les réponses en temps réel

> Recevez les réponses en temps réel du SDK Agent à mesure que le texte et les appels d'outils sont diffusés

Par défaut, le SDK Agent produit des objets `AssistantMessage` complets après que Claude ait terminé de générer chaque réponse. Pour recevoir des mises à jour incrémentielles à mesure que le texte et les appels d'outils sont générés, activez la diffusion de messages partiels en définissant `include_partial_messages` (Python) ou `includePartialMessages` (TypeScript) sur `true` dans vos options.

<Tip>
  Cette page couvre la diffusion de sortie (réception des jetons en temps réel). Pour les modes d'entrée (comment vous envoyez les messages), consultez [Envoyer des messages aux agents](/docs/fr/agent-sdk/streaming-vs-single-mode). Vous pouvez également [diffuser les réponses en utilisant le SDK Agent via la CLI](/docs/fr/headless).
</Tip>

<h2 id="enable-streaming-output">
  Activer la diffusion de sortie
</h2>

Pour activer la diffusion, définissez `include_partial_messages` (Python) ou `includePartialMessages` (TypeScript) sur `true` dans vos options. Cela fait que le SDK produit des messages `StreamEvent` contenant les événements API bruts à mesure qu'ils arrivent, en plus des `AssistantMessage` et `ResultMessage` habituels.

Votre code doit alors :

1. Vérifier le type de chaque message pour distinguer `StreamEvent` des autres types de messages
2. Pour `StreamEvent`, extraire le champ `event` et vérifier son `type`
3. Rechercher les événements `content_block_delta` où `delta.type` est `text_delta`, qui contiennent les fragments de texte réels

L'exemple ci-dessous active la diffusion et affiche les fragments de texte à mesure qu'ils arrivent. Remarquez les vérifications de type imbriquées : d'abord pour `StreamEvent`, puis pour `content_block_delta`, puis pour `text_delta` :

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_response():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Bash", "Read"],
      )

      async for message in query(prompt="List the files in my project", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              if event.get("type") == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "text_delta":
                      print(delta.get("text", ""), end="", flush=True)


  asyncio.run(stream_response())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the files in my project",
    options: {
      includePartialMessages: true,
      allowedTools: ["Bash", "Read"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;
      if (event.type === "content_block_delta") {
        if (event.delta.type === "text_delta") {
          process.stdout.write(event.delta.text);
        }
      }
    }
  }
  ```
</CodeGroup>

<h2 id="streamevent-reference">
  Référence StreamEvent
</h2>

Lorsque les messages partiels sont activés, vous recevez les événements de diffusion bruts de l'API Claude enveloppés dans un objet. Le type a des noms différents dans chaque SDK :

* **Python** : `StreamEvent` (importer depuis `claude_agent_sdk.types`)
* **TypeScript** : `SDKPartialAssistantMessage` avec `type: 'stream_event'`

Les deux contiennent les événements bruts de l'API Claude, pas le texte accumulé. Vous devez extraire et accumuler les deltas de texte vous-même. Voici la structure de chaque type :

<CodeGroup>
  ```python Python theme={null}
  @dataclass
  class StreamEvent:
      uuid: str  # Unique identifier for this event
      session_id: str  # Session identifier
      event: dict[str, Any]  # The raw Claude API stream event
      parent_tool_use_id: str | None  # Always None
  ```

  ```typescript TypeScript theme={null}
  type SDKPartialAssistantMessage = {
    type: "stream_event";
    event: BetaRawMessageStreamEvent; // From Anthropic SDK
    parent_tool_use_id: string | null;
    uuid: UUID;
    session_id: string;
    ttft_ms?: number; // Time to first token in ms, present only on message_start events
  };
  ```
</CodeGroup>

Le champ `parent_tool_use_id` est toujours `None` en Python et `null` en TypeScript. Les événements de diffusion sont émis pour la session principale uniquement ; les deltas au niveau des tokens des sous-agents ne sont pas transmis. Pour attribuer la sortie à un sous-agent, utilisez les messages complets, qui portent `parent_tool_use_id`. Voir [Détecter l'invocation de sous-agent](/docs/fr/agent-sdk/subagents#detect-subagent-invocation).

Le champ `event` contient l'événement de diffusion brut de l'[API Claude](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). Les types d'événements courants incluent :

| Type d'événement      | Description                                                       |
| :-------------------- | :---------------------------------------------------------------- |
| `message_start`       | Début d'un nouveau message                                        |
| `content_block_start` | Début d'un nouveau bloc de contenu (texte ou utilisation d'outil) |
| `content_block_delta` | Mise à jour incrémentielles du contenu                            |
| `content_block_stop`  | Fin d'un bloc de contenu                                          |
| `message_delta`       | Mises à jour au niveau du message (raison d'arrêt, utilisation)   |
| `message_stop`        | Fin du message                                                    |

<h2 id="message-flow">
  Flux de messages
</h2>

Avec les messages partiels activés, vous recevez les messages dans cet ordre :

```text theme={null}
StreamEvent (message_start)
StreamEvent (content_block_start) - text block
StreamEvent (content_block_delta) - text chunks...
StreamEvent (content_block_stop)
StreamEvent (content_block_start) - tool_use block
StreamEvent (content_block_delta) - tool input chunks...
StreamEvent (content_block_stop)
StreamEvent (message_delta)
StreamEvent (message_stop)
AssistantMessage - complete message with all content
... tool executes ...
... more streaming events for next turn ...
ResultMessage - final result
```

Sans les messages partiels activés (`include_partial_messages` en Python, `includePartialMessages` en TypeScript), vous recevez tous les types de messages sauf `StreamEvent`. Les types courants incluent `SystemMessage` (initialisation de session), `AssistantMessage` (réponses complètes), `ResultMessage` (résultat final), et un message de limite compact indiquant quand l'historique de conversation a été compacté (`SDKCompactBoundaryMessage` en TypeScript ; `SystemMessage` avec le sous-type `"compact_boundary"` en Python).

<h2 id="stream-text-responses">
  Diffuser les réponses texte
</h2>

Pour afficher le texte à mesure qu'il est généré, recherchez les événements `content_block_delta` où `delta.type` est `text_delta`. Ceux-ci contiennent les fragments de texte incrémentiels. L'exemple ci-dessous affiche chaque fragment à mesure qu'il arrive :

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_text():
      options = ClaudeAgentOptions(include_partial_messages=True)

      async for message in query(prompt="Explain how databases work", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              if event.get("type") == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "text_delta":
                      # Print each text chunk as it arrives
                      print(delta.get("text", ""), end="", flush=True)

      print()  # Final newline


  asyncio.run(stream_text())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Explain how databases work",
    options: { includePartialMessages: true }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;
      if (event.type === "content_block_delta" && event.delta.type === "text_delta") {
        process.stdout.write(event.delta.text);
      }
    }
  }

  console.log(); // Final newline
  ```
</CodeGroup>

<h2 id="stream-tool-calls">
  Diffuser les appels d'outils
</h2>

Les appels d'outils sont également diffusés de manière incrémentielles. Vous pouvez suivre quand les outils commencent, recevoir leur entrée à mesure qu'elle est générée, et voir quand ils se terminent. L'exemple ci-dessous suit l'outil actuellement appelé et accumule l'entrée JSON à mesure qu'elle est diffusée. Il utilise trois types d'événements :

* `content_block_start` : l'outil commence
* `content_block_delta` avec `input_json_delta` : les fragments d'entrée arrivent
* `content_block_stop` : l'appel d'outil est complet

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  from claude_agent_sdk.types import StreamEvent
  import asyncio


  async def stream_tool_calls():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Read", "Bash"],
      )

      # Track the current tool and accumulate its input JSON
      current_tool = None
      tool_input = ""

      async for message in query(prompt="Read the README.md file", options=options):
          if isinstance(message, StreamEvent):
              event = message.event
              event_type = event.get("type")

              if event_type == "content_block_start":
                  # New tool call is starting
                  content_block = event.get("content_block", {})
                  if content_block.get("type") == "tool_use":
                      current_tool = content_block.get("name")
                      tool_input = ""
                      print(f"Starting tool: {current_tool}")

              elif event_type == "content_block_delta":
                  delta = event.get("delta", {})
                  if delta.get("type") == "input_json_delta":
                      # Accumulate JSON input as it streams in
                      chunk = delta.get("partial_json", "")
                      tool_input += chunk
                      print(f"  Input chunk: {chunk}")

              elif event_type == "content_block_stop":
                  # Tool call complete - show final input
                  if current_tool:
                      print(f"Tool {current_tool} called with: {tool_input}")
                      current_tool = None


  asyncio.run(stream_tool_calls())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track the current tool and accumulate its input JSON
  let currentTool: string | null = null;
  let toolInput = "";

  for await (const message of query({
    prompt: "Read the README.md file",
    options: {
      includePartialMessages: true,
      allowedTools: ["Read", "Bash"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;

      if (event.type === "content_block_start") {
        // New tool call is starting
        if (event.content_block.type === "tool_use") {
          currentTool = event.content_block.name;
          toolInput = "";
          console.log(`Starting tool: ${currentTool}`);
        }
      } else if (event.type === "content_block_delta") {
        if (event.delta.type === "input_json_delta") {
          // Accumulate JSON input as it streams in
          const chunk = event.delta.partial_json;
          toolInput += chunk;
          console.log(`  Input chunk: ${chunk}`);
        }
      } else if (event.type === "content_block_stop") {
        // Tool call complete - show final input
        if (currentTool) {
          console.log(`Tool ${currentTool} called with: ${toolInput}`);
          currentTool = null;
        }
      }
    }
  }
  ```
</CodeGroup>

<h2 id="build-a-streaming-ui">
  Construire une interface utilisateur de diffusion
</h2>

Cet exemple combine la diffusion de texte et d'outils dans une interface utilisateur cohésive. Il suit si l'agent exécute actuellement un outil (en utilisant un drapeau `in_tool`) pour afficher des indicateurs de statut comme `[Using Read...]` pendant que les outils s'exécutent. Le texte se diffuse normalement quand il n'y a pas d'outil, et la fin de l'outil déclenche un message « done ». Ce modèle est utile pour les interfaces de chat qui doivent afficher la progression pendant les tâches d'agent multi-étapes.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
  from claude_agent_sdk.types import StreamEvent
  import asyncio
  import sys


  async def streaming_ui():
      options = ClaudeAgentOptions(
          include_partial_messages=True,
          allowed_tools=["Read", "Bash", "Grep"],
      )

      # Track whether we're currently in a tool call
      in_tool = False

      async for message in query(
          prompt="Find all TODO comments in the codebase", options=options
      ):
          if isinstance(message, StreamEvent):
              event = message.event
              event_type = event.get("type")

              if event_type == "content_block_start":
                  content_block = event.get("content_block", {})
                  if content_block.get("type") == "tool_use":
                      # Tool call is starting - show status indicator
                      tool_name = content_block.get("name")
                      print(f"\n[Using {tool_name}...]", end="", flush=True)
                      in_tool = True

              elif event_type == "content_block_delta":
                  delta = event.get("delta", {})
                  # Only stream text when not executing a tool
                  if delta.get("type") == "text_delta" and not in_tool:
                      sys.stdout.write(delta.get("text", ""))
                      sys.stdout.flush()

              elif event_type == "content_block_stop":
                  if in_tool:
                      # Tool call finished
                      print(" done", flush=True)
                      in_tool = False

          elif isinstance(message, ResultMessage):
              # Agent finished all work
              print(f"\n\n--- Complete ---")


  asyncio.run(streaming_ui())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track whether we're currently in a tool call
  let inTool = false;

  for await (const message of query({
    prompt: "Find all TODO comments in the codebase",
    options: {
      includePartialMessages: true,
      allowedTools: ["Read", "Bash", "Grep"]
    }
  })) {
    if (message.type === "stream_event") {
      const event = message.event;

      if (event.type === "content_block_start") {
        if (event.content_block.type === "tool_use") {
          // Tool call is starting - show status indicator
          process.stdout.write(`\n[Using ${event.content_block.name}...]`);
          inTool = true;
        }
      } else if (event.type === "content_block_delta") {
        // Only stream text when not executing a tool
        if (event.delta.type === "text_delta" && !inTool) {
          process.stdout.write(event.delta.text);
        }
      } else if (event.type === "content_block_stop") {
        if (inTool) {
          // Tool call finished
          console.log(" done");
          inTool = false;
        }
      }
    } else if (message.type === "result") {
      // Agent finished all work
      console.log("\n\n--- Complete ---");
    }
  }
  ```
</CodeGroup>

<h2 id="known-limitations">
  Limitations connues
</h2>

* **Sortie structurée** : le résultat JSON n'apparaît que dans le `ResultMessage.structured_output` final, pas comme des deltas de diffusion. Consultez [les sorties structurées](/docs/fr/agent-sdk/structured-outputs) pour plus de détails.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous pouvez diffuser le texte et les appels d'outils en temps réel, explorez ces sujets connexes :

* [Requêtes interactives ou ponctuelles](/docs/fr/agent-sdk/streaming-vs-single-mode) : choisissez entre les modes d'entrée pour votre cas d'usage
* [Sorties structurées](/docs/fr/agent-sdk/structured-outputs) : obtenez des réponses JSON typées de l'agent
* [Permissions](/docs/fr/agent-sdk/permissions) : contrôlez les outils que l'agent peut utiliser
