> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Intercepter et contrôler le comportement des agents avec des hooks

> Interceptez et personnalisez le comportement des agents aux points d'exécution clés avec des hooks

Les hooks sont des fonctions de rappel qui exécutent votre code en réponse à des événements d'agent, comme un outil appelé, une session démarrée ou l'exécution arrêtée. Avec les hooks, vous pouvez :

* **Bloquer les opérations dangereuses** avant leur exécution, comme les commandes shell destructrices ou l'accès non autorisé aux fichiers
* **Enregistrer et auditer** chaque appel d'outil pour la conformité, le débogage ou l'analyse
* **Transformer les entrées et les sorties** pour nettoyer les données, injecter des identifiants ou rediriger les chemins de fichiers
* **Exiger une approbation humaine** pour les actions sensibles comme les écritures de base de données ou les appels API
* **Suivre le cycle de vie de la session** pour gérer l'état, nettoyer les ressources ou envoyer des notifications

Ce guide couvre le fonctionnement des hooks, comment les configurer et fournit des exemples pour les modèles courants comme bloquer les outils, modifier les entrées et transférer les notifications.

<h2 id="how-hooks-work">
  Fonctionnement des hooks
</h2>

<Steps>
  <Step title="Un événement se déclenche">
    Quelque chose se produit lors de l'exécution de l'agent et le SDK déclenche un événement : un outil est sur le point d'être appelé (`PreToolUse`), un outil a renvoyé un résultat (`PostToolUse`), un sous-agent a démarré ou s'est arrêté, l'agent est inactif ou l'exécution s'est terminée. Consultez la [liste complète des événements](#available-hooks).
  </Step>

  <Step title="Le SDK collecte les hooks enregistrés">
    Le SDK vérifie les hooks enregistrés pour ce type d'événement. Cela inclut les hooks de rappel que vous transmettez dans `options.hooks` et les hooks de commande shell à partir des fichiers de paramètres lorsque l'entrée [`settingSources`](/docs/fr/agent-sdk/typescript#settingsource) ou [`setting_sources`](/docs/fr/agent-sdk/python#settingsource) correspondante est activée, ce qui est le cas pour les options `query()` par défaut.
  </Step>

  <Step title="Les matchers filtrent les hooks qui s'exécutent">
    Si un hook a un motif [`matcher`](#matchers) (comme `"Write|Edit"`), le SDK le teste par rapport à la cible de l'événement (par exemple, le nom de l'outil). Les hooks sans matcher s'exécutent pour chaque événement de ce type.
  </Step>

  <Step title="Les fonctions de rappel s'exécutent">
    Chaque [fonction de rappel](#callback-functions) du hook correspondant reçoit des informations sur ce qui se passe : le nom de l'outil, ses arguments, l'ID de session et d'autres détails spécifiques à l'événement.
  </Step>

  <Step title="Votre rappel retourne une décision">
    Après avoir effectué toute opération (enregistrement, appels API, validation), votre rappel retourne un [objet de sortie](#outputs) qui indique à l'agent quoi faire : autoriser l'opération, la bloquer, modifier l'entrée ou injecter du contexte dans la conversation.
  </Step>
</Steps>

L'exemple suivant réunit ces étapes. Il enregistre un hook `PreToolUse` (étape 1) avec un matcher `"Write|Edit"` (étape 3) afin que le rappel ne se déclenche que pour les outils d'écriture de fichiers. Lorsqu'il est déclenché, le rappel reçoit l'entrée de l'outil (étape 4), vérifie si le chemin du fichier cible un fichier `.env` et retourne `permissionDecision: "deny"` pour bloquer l'opération (étape 5) :

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # Define a hook callback that receives tool call details
  async def protect_env_files(input_data, tool_use_id, context):
      # Extract the file path from the tool's input arguments
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # Block the operation if targeting a .env file
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # Return empty object to allow the operation
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for PreToolUse events
              # The matcher filters to only Write and Edit tool calls
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Update the database configuration")
          async for message in client.receive_response():
              # Filter for assistant and result messages
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback with the HookCallback type
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast input to the specific hook type for type safety
    const preInput = input as PreToolUseHookInput;

    // Cast tool_input to access its properties (typed as unknown in the SDK)
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // Block the operation if targeting a .env file
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // Return empty object to allow the operation
    return {};
  };

  for await (const message of query({
    prompt: "Update the database configuration",
    options: {
      hooks: {
        // Register the hook for PreToolUse events
        // The matcher filters to only Write and Edit tool calls
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // Filter for assistant and result messages
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

<h2 id="available-hooks">
  Hooks disponibles
</h2>

Le SDK fournit des hooks pour différentes étapes de l'exécution de l'agent. Certains hooks sont disponibles dans les deux SDK, tandis que d'autres sont réservés à TypeScript.

| Événement Hook                                         | SDK Python | SDK TypeScript | Ce qui le déclenche                                                                                    | Cas d'usage exemple                                                                             |
| ------------------------------------------------------ | ---------- | -------------- | ------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------- |
| `PreToolUse`                                           | Oui        | Oui            | Demande d'appel d'outil (peut bloquer ou modifier)                                                     | Bloquer les commandes shell dangereuses                                                         |
| `PostToolUse`                                          | Oui        | Oui            | Résultat de l'exécution de l'outil                                                                     | Enregistrer tous les changements de fichiers dans la piste d'audit                              |
| `PostToolUseFailure`                                   | Oui        | Oui            | Échec de l'exécution de l'outil                                                                        | Gérer ou enregistrer les erreurs d'outil                                                        |
| `PostToolBatch`                                        | Non        | Oui            | Un lot complet d'appels d'outil se résout, une fois par lot avant l'appel du modèle suivant            | Injecter des conventions une fois pour tout le lot                                              |
| `UserPromptSubmit`                                     | Oui        | Oui            | Soumission d'invite utilisateur                                                                        | Injecter du contexte supplémentaire dans les invites                                            |
| [`UserPromptExpansion`](/docs/fr/hooks#userpromptexpansion) | Non        | Oui            | Une commande tapée par l'utilisateur se développe en une invite avant d'atteindre Claude               | Bloquer une commande d'invocation directe ou ajouter du contexte quand une compétence est tapée |
| `MessageDisplay`                                       | Non        | Oui            | Un message d'assistant avec du texte se termine, une fois par message avec le texte complet du message | Masquer ou reformater le texte affiché sans modifier la transcription                           |
| `Stop`                                                 | Oui        | Oui            | Arrêt de l'exécution de l'agent                                                                        | Enregistrer l'état de la session avant la sortie                                                |
| `SubagentStart`                                        | Oui        | Oui            | Initialisation du sous-agent                                                                           | Suivre le lancement des tâches parallèles                                                       |
| `SubagentStop`                                         | Oui        | Oui            | Achèvement du sous-agent                                                                               | Agréger les résultats des tâches parallèles                                                     |
| `PreCompact`                                           | Oui        | Oui            | Demande de compaction de conversation                                                                  | Archiver la transcription complète avant le résumé                                              |
| `PermissionRequest`                                    | Oui        | Oui            | La boîte de dialogue de permission s'afficherait                                                       | Gestion des permissions personnalisée                                                           |
| `SessionStart`                                         | Non        | Oui            | Initialisation de la session                                                                           | Initialiser la journalisation et la télémétrie                                                  |
| `SessionEnd`                                           | Non        | Oui            | Arrêt de la session                                                                                    | Nettoyer les ressources temporaires                                                             |
| `Notification`                                         | Oui        | Oui            | Messages d'état de l'agent                                                                             | Envoyer les mises à jour d'état de l'agent à Slack ou PagerDuty                                 |
| `Setup`                                                | Non        | Oui            | Configuration/maintenance de la session                                                                | Exécuter les tâches d'initialisation                                                            |
| `TeammateIdle`                                         | Non        | Oui            | Le coéquipier devient inactif                                                                          | Réassigner le travail ou notifier                                                               |
| `TaskCompleted`                                        | Non        | Oui            | La tâche de fond se termine                                                                            | Agréger les résultats des tâches parallèles                                                     |
| `ConfigChange`                                         | Non        | Oui            | Le fichier de configuration change                                                                     | Recharger les paramètres dynamiquement                                                          |
| `WorktreeCreate`                                       | Non        | Oui            | Git worktree créé                                                                                      | Suivre les espaces de travail isolés                                                            |
| `WorktreeRemove`                                       | Non        | Oui            | Git worktree supprimé                                                                                  | Nettoyer les ressources de l'espace de travail                                                  |

<h2 id="configure-hooks">
  Configurer les hooks
</h2>

Pour configurer un hook, transmettez-le dans le champ `hooks` de vos options d'agent (`ClaudeAgentOptions` en Python, l'objet `options` en TypeScript) :

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

L'option `hooks` est un dictionnaire en Python ou un objet en TypeScript, où :

* **Les clés** : [les noms d'événements hook](#available-hooks) tels que `'PreToolUse'`, `'PostToolUse'` et `'Stop'`
* **Les valeurs** : des tableaux de [matchers](#matchers), chacun contenant un motif de filtre optionnel et vos [fonctions de rappel](#callback-functions)

<h3 id="matchers">
  Matchers
</h3>

Utilisez les matchers pour filtrer quand vos rappels se déclenchent. Le champ `matcher` correspond à une valeur différente selon le type d'événement hook. Par exemple, les hooks basés sur les outils correspondent au nom de l'outil, tandis que les hooks `Notification` correspondent au type de notification. Consultez la [référence des hooks Claude Code](/docs/fr/hooks#matcher-patterns) pour la liste complète des valeurs de matcher pour chaque type d'événement.

Les matchers du SDK suivent les mêmes règles que les [matchers dans les fichiers de paramètres](/docs/fr/hooks#matcher-patterns). Un matcher contenant uniquement des lettres, des chiffres, `_`, `-`, des espaces, `,` et `|` est comparé comme une chaîne exacte, avec des alternatives séparées par `|` ou `,` et des espaces blancs optionnels autour, donc `Write|Edit` et `Write, Edit` correspondent exactement à ces deux outils et `code-reviewer` correspond uniquement à ce type d'agent. Un matcher de `*`, une chaîne vide, ou l'omission du matcher entièrement correspond à chaque occurrence de l'événement.

Un matcher contenant tout autre caractère est évalué comme une expression régulière non ancrée, donc `^mcp__` correspond à chaque outil MCP et `Edit.*` correspond à la fois à `Edit` et à `NotebookEdit`. Enveloppez une expression régulière dans `^` et `$` lorsque vous avez besoin d'une correspondance de chaîne entière.

Un matcher comme `mcp__memory` ou `mcp__brave-search` contient uniquement des caractères de correspondance exacte, donc il est comparé comme une chaîne exacte et ne correspond à aucun outil ; utilisez `mcp__memory__.*` pour correspondre à chaque outil de ce serveur.

Les traits d'union dans l'ensemble de correspondance exacte nécessitent un runtime Claude Code v2.1.195 ou ultérieur. Sur les versions antérieures, un nom avec trait d'union comme `code-reviewer` est évalué comme une expression régulière non ancrée et doit être ancré comme `^code-reviewer$` pour correspondre exactement.

| Option    | Type             | Par défaut  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| --------- | ---------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `matcher` | `string`         | `undefined` | Motif mis en correspondance avec le champ de filtre de l'événement, en suivant les règles de comparaison ci-dessus. Pour les hooks d'outils, c'est le nom de l'outil. Les outils intégrés incluent `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent` et d'autres (consultez [Types d'entrée d'outil](/docs/fr/agent-sdk/typescript#tool-input-types) pour la liste complète). Les outils MCP utilisent le motif `mcp__<server>__<action>`. |
| `hooks`   | `HookCallback[]` | -           | Requis. Tableau de fonctions de rappel à exécuter lorsque le motif correspond                                                                                                                                                                                                                                                                                                                                                                            |
| `timeout` | `number`         | `60`        | Délai d'expiration en secondes                                                                                                                                                                                                                                                                                                                                                                                                                           |

Utilisez le motif `matcher` pour cibler des outils spécifiques chaque fois que possible. Un matcher avec `'Bash'` s'exécute uniquement pour les commandes Bash, tandis que l'omission du motif exécute vos rappels pour chaque occurrence de l'événement.

Pour les hooks basés sur les outils, les matchers ne filtrent que par nom d'outil, pas par chemins de fichiers ou d'autres arguments. Pour filtrer par chemin de fichier, vérifiez `tool_input.file_path` à l'intérieur de votre rappel.

<Tip>
  **Découvrir les noms d'outils :** Consultez [Types d'entrée d'outil](/docs/fr/agent-sdk/typescript#tool-input-types) pour la liste complète des noms d'outils intégrés, ou ajoutez un hook sans matcher pour enregistrer tous les appels d'outils que votre session effectue.

  **Nommage des outils MCP :** Les outils MCP commencent toujours par `mcp__` suivi du nom du serveur et de l'action : `mcp__<server>__<action>`. Par exemple, si vous configurez un serveur nommé `playwright`, ses outils seront nommés `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click`, et ainsi de suite. Le nom du serveur provient de la clé que vous utilisez dans la configuration `mcpServers`.
</Tip>

<h3 id="callback-functions">
  Fonctions de rappel
</h3>

<h4 id="inputs">
  Entrées
</h4>

Chaque rappel hook reçoit trois arguments :

* **Données d'entrée :** un objet typé contenant les détails de l'événement. Chaque type de hook a sa propre forme d'entrée. Par exemple, `PreToolUseHookInput` inclut `tool_name` et `tool_input`, tandis que `NotificationHookInput` inclut `message`. Consultez les définitions de type complètes dans les références du SDK [TypeScript](/docs/fr/agent-sdk/typescript#hookinput) et [Python](/docs/fr/agent-sdk/python#hookinput).
  * Toutes les entrées de hook partagent `session_id`, `cwd` et `hook_event_name`.
  * `agent_id` et `agent_type` sont remplis lorsque le hook se déclenche à l'intérieur d'un sous-agent. En TypeScript, ceux-ci se trouvent sur l'entrée de hook de base et sont disponibles pour tous les types de hook. En Python, ils sont des champs optionnels sur `PreToolUse`, `PostToolUse`, `PostToolUseFailure` et `PermissionRequest`, et des champs requis sur `SubagentStart` et `SubagentStop`.
* **ID d'utilisation d'outil** (`str | None` / `string | undefined`) : met en corrélation les événements `PreToolUse` et `PostToolUse` pour le même appel d'outil.
* **Contexte :** en TypeScript, contient une propriété `signal` (`AbortSignal`) pour l'annulation. En Python, cet argument est réservé pour une utilisation future.

<h4 id="outputs">
  Sorties
</h4>

Votre rappel retourne un objet avec deux catégories de champs :

* **Champs de niveau supérieur** fonctionnent de la même manière sur chaque événement : `systemMessage` affiche un message à l'utilisateur, et `continue` (`continue_` en Python) détermine si l'agent continue à s'exécuter après ce hook.
* **`hookSpecificOutput`** contrôle l'opération actuelle. Les champs à l'intérieur dépendent du type d'événement hook. Pour les hooks `PreToolUse`, c'est là que vous définissez `permissionDecision` (`"allow"`, `"deny"`, `"ask"` ou `"defer"`), `permissionDecisionReason` et `updatedInput`. Retourner `"defer"` termine la requête pour que vous puissiez [la reprendre plus tard](/docs/fr/hooks#defer-a-tool-call-for-later). Pour les hooks `PostToolUse`, vous pouvez définir `additionalContext` pour ajouter des informations au résultat de l'outil. Pour remplacer la sortie de l'outil avant que Claude ne la voie, définissez `updatedToolOutput`, qui fonctionne pour n'importe quel outil dans les deux SDK. Le champ plus ancien `updatedMCPToolOutput` remplace uniquement la sortie de l'outil MCP et est déprécié.

Retournez `{}` pour autoriser l'opération sans modifications. Les hooks de rappel du SDK utilisent le même format de sortie JSON que les [hooks de commande shell Claude Code](/docs/fr/hooks#json-output), qui documente chaque champ et option spécifique à l'événement. Pour les définitions de type du SDK, consultez les références du SDK [TypeScript](/docs/fr/agent-sdk/typescript#synchookjsonoutput) et [Python](/docs/fr/agent-sdk/python#synchookjsonoutput).

<Note>
  Lorsque plusieurs hooks ou règles de permission s'appliquent, `deny` a priorité sur `defer`, qui a priorité sur `ask`, qui a priorité sur `allow`. Si un hook retourne `deny`, l'opération est bloquée indépendamment des autres hooks.
</Note>

<h4 id="asynchronous-output">
  Sortie asynchrone
</h4>

Par défaut, l'agent attend que votre hook retourne avant de continuer. Si votre hook effectue un effet secondaire, tel que la journalisation ou l'envoi d'un webhook, et n'a pas besoin d'influencer le comportement de l'agent, vous pouvez retourner une sortie asynchrone à la place. Cela indique à l'agent de continuer immédiatement sans attendre la fin du hook :

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # Start a background task, then return immediately
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // Start a background task, then return immediately
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| Champ          | Type     | Description                                                                                                              |
| -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------ |
| `async`        | `true`   | Signale le mode asynchrone. L'agent continue sans attendre. En Python, utilisez `async_` pour éviter le mot-clé réservé. |
| `asyncTimeout` | `number` | Délai d'expiration optionnel en millisecondes pour l'opération de fond                                                   |

<Note>
  Les sorties asynchrones ne peuvent pas bloquer, modifier ou injecter du contexte dans l'opération puisque l'agent a déjà avancé. Utilisez-les uniquement pour les effets secondaires comme la journalisation, les métriques ou les notifications.
</Note>

<h2 id="examples">
  Exemples
</h2>

<h3 id="modify-tool-input">
  Modifier l'entrée de l'outil
</h3>

Cet exemple intercepte les appels d'outil Write et réécrit l'argument `file_path` pour ajouter `/sandbox`, redirigeant toutes les écritures de fichiers vers un répertoire en sandbox. Le rappel retourne `updatedInput` avec le chemin modifié et `permissionDecision: 'allow'` pour approuver automatiquement l'opération réécrite :

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  Lors de l'utilisation de `updatedInput`, vous devez également inclure `permissionDecision: 'allow'` pour approuver automatiquement l'entrée modifiée ou `permissionDecision: 'ask'` pour la montrer à l'utilisateur. Avec `'defer'`, `updatedInput` est ignoré. Retournez toujours un nouvel objet plutôt que de muter le `tool_input` original.
</Note>

<h3 id="add-context-and-block-a-tool">
  Ajouter du contexte et bloquer un outil
</h3>

Cet exemple bloque les écritures dans le répertoire `/etc` et explique pourquoi au modèle et à l'utilisateur :

* `permissionDecision: 'deny'` arrête l'appel d'outil.
* `permissionDecisionReason` indique au modèle pourquoi, afin qu'il évite de réessayer.
* `systemMessage` montre à l'utilisateur ce qui s'est passé.

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  Approuver automatiquement des outils spécifiques
</h3>

Par défaut, l'agent peut demander une permission avant d'utiliser certains outils. Cet exemple approuve automatiquement les outils de système de fichiers en lecture seule (Read, Glob, Grep) en retournant `permissionDecision: 'allow'`, les laissant s'exécuter sans confirmation de l'utilisateur tout en laissant tous les autres outils soumis aux vérifications de permission normales :

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  Enregistrer plusieurs hooks
</h3>

Quand un événement se déclenche, tous les hooks correspondants s'exécutent en parallèle. Pour les décisions de permission, le résultat le plus restrictif gagne : un seul `deny` bloque l'appel d'outil indépendamment de ce que les autres hooks retournent. Parce que l'ordre d'exécution est non-déterministe, écrivez chaque hook pour agir indépendamment plutôt que de compter sur l'exécution préalable d'un autre hook.

L'exemple ci-dessous enregistre trois vérifications indépendantes pour chaque appel d'outil :

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  Filtrer avec des matchers multi-outils
</h3>

Utilisez des matchers multi-outils pour partager un rappel entre outils connexes. Cet exemple enregistre trois matchers avec des portées différentes :

* Une liste exacte séparée par des barres (`Write|Edit|Delete`) déclenche `file_security_hook` uniquement pour les outils de modification de fichiers.
* Une regex (`^mcp__`) déclenche `mcp_audit_hook` pour tout outil MCP dont le nom commence par `mcp__`.
* Un matcher omis déclenche `global_logger` pour chaque appel d'outil indépendamment du nom.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|Delete", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|Delete", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  Suivre l'activité des sous-agents
</h3>

Utilisez les hooks `SubagentStop` pour surveiller quand les sous-agents terminent leur travail. Consultez le type d'entrée complet dans les références du SDK [TypeScript](/docs/fr/agent-sdk/typescript#hookinput) et [Python](/docs/fr/agent-sdk/python#hookinput). Cet exemple enregistre un résumé chaque fois qu'un sous-agent se termine :

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  Effectuer des requêtes HTTP à partir des hooks
</h3>

Les hooks peuvent effectuer des opérations asynchrones comme les requêtes HTTP. Capturez les erreurs à l'intérieur de votre hook au lieu de les laisser se propager, car une exception non gérée peut interrompre l'agent.

Cet exemple envoie un webhook après chaque exécution d'outil, enregistrant quel outil a été exécuté et quand. Le hook capture les erreurs afin qu'un webhook échoué n'interrompe pas l'agent :

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise. A failed webhook shouldn't stop the agent
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw. A failed webhook shouldn't stop the agent
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="forward-notifications-to-slack">
  Transférer les notifications à Slack
</h3>

Utilisez les hooks `Notification` pour recevoir les notifications système de l'agent et les transférer vers des services externes. Les notifications se déclenchent pour des types d'événements tels que :

* `permission_prompt` quand Claude a besoin d'une permission
* `idle_prompt` quand Claude attend une entrée
* `auth_success` quand l'authentification se termine
* `elicitation_dialog`, `elicitation_complete`, et `elicitation_response` pour les flux d'élicitation d'entrée utilisateur

Chaque notification inclut un champ `message` avec une description lisible par l'homme et optionnellement un `title`.

Cet exemple transfère chaque notification à un canal Slack. Il nécessite une [URL de webhook entrant Slack](https://api.slack.com/messaging/webhooks), que vous créez en ajoutant une application à votre espace de travail Slack et en activant les webhooks entrants :

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="fix-common-issues">
  Corriger les problèmes courants
</h2>

<h3 id="hook-not-firing">
  Hook ne se déclenche pas
</h3>

* Vérifiez que le nom de l'événement hook est correct et sensible à la casse (`PreToolUse`, pas `preToolUse`)
* Vérifiez que votre motif matcher correspond exactement au nom de l'outil
* Assurez-vous que le hook se trouve sous le type d'événement correct dans `options.hooks`
* Pour les hooks non basés sur les outils qui prennent en charge les matchers, comme `Notification` et `SubagentStop`, les matchers correspondent à des champs différents, et `Stop` ignore complètement les matchers (consultez [motifs matcher](/docs/fr/hooks#matcher-patterns))
* Les hooks peuvent ne pas se déclencher lorsque l'agent atteint la limite [`max_turns`](/docs/fr/agent-sdk/python#claudeagentoptions) car la session se termine avant que les hooks puissent s'exécuter

<h3 id="matcher-not-filtering-as-expected">
  Matcher ne filtre pas comme prévu
</h3>

Les matchers ne correspondent qu'aux noms d'outils, pas aux chemins de fichiers ou à d'autres arguments. Pour filtrer par chemin de fichier, vérifiez `tool_input.file_path` à l'intérieur de votre hook :

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  Délai d'expiration du hook
</h3>

* Augmentez la valeur `timeout` dans la configuration `HookMatcher`
* Utilisez le `AbortSignal` du troisième argument de rappel pour gérer l'annulation correctement en TypeScript

Un rappel `UserPromptSubmit` ou [`UserPromptExpansion`](/docs/fr/hooks#userpromptexpansion) qui dépasse son délai d'expiration bloque cette invite avec un message de délai d'expiration et la session continue. L'interruption de la requête pendant qu'un rappel est en attente annule l'appel d'outil en attente. Avant la v2.1.208, un délai d'expiration de rappel sur ces événements terminait la requête avec `error_during_execution`, et une interruption pendant un rappel `PreToolUse` en attente pouvait laisser l'appel d'outil se poursuivre.

<h3 id="tool-blocked-unexpectedly">
  Outil bloqué de manière inattendue
</h3>

* Vérifiez tous les hooks `PreToolUse` pour les retours `permissionDecision: 'deny'`
* Ajoutez la journalisation à vos hooks pour voir quel `permissionDecisionReason` ils retournent
* Vérifiez que les motifs matcher ne sont pas trop larges : un matcher vide correspond à tous les outils

<h3 id="modified-input-not-applied">
  Entrée modifiée non appliquée
</h3>

* Assurez-vous que `updatedInput` se trouve à l'intérieur de `hookSpecificOutput`, pas au niveau supérieur :

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* Retournez `permissionDecision: 'allow'` pour approuver automatiquement l'entrée modifiée, ou `'ask'` pour la montrer à l'utilisateur pour approbation

* Incluez `hookEventName` dans `hookSpecificOutput` pour identifier le type de hook pour lequel la sortie est destinée

<h3 id="session-hooks-not-available-in-python">
  Hooks de session non disponibles en Python
</h3>

`SessionStart` et `SessionEnd` peuvent être enregistrés en tant que hooks de rappel du SDK en TypeScript, mais ne sont pas disponibles dans le SDK Python car son type `HookEvent` les omet. En Python, ils ne sont disponibles que comme [hooks de commande shell](/docs/fr/hooks#hook-events) définis dans les fichiers de paramètres tels que `.claude/settings.json`. Pour charger les hooks de commande shell à partir de votre application SDK, incluez la source de paramètre appropriée avec [`setting_sources`](/docs/fr/agent-sdk/python#settingsource) ou [`settingSources`](/docs/fr/agent-sdk/typescript#settingsource) :

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Pour exécuter la logique d'initialisation en tant que rappel du SDK Python à la place, utilisez le premier message de `client.receive_response()` comme déclencheur.

<h3 id="subagent-permission-prompts-multiplying">
  Les invites de permission des sous-agents se multiplient
</h3>

Lors du lancement de plusieurs sous-agents, chacun peut demander des permissions séparément. Les sous-agents n'héritent pas automatiquement des permissions de l'agent parent. Pour éviter les invites répétées, utilisez les hooks `PreToolUse` pour approuver automatiquement des outils spécifiques, ou configurez des règles de permission qui s'appliquent aux sessions de sous-agent.

<h3 id="recursive-hook-loops-with-subagents">
  Boucles de hook récursives avec des sous-agents
</h3>

Un hook `UserPromptSubmit` qui lance des sous-agents peut créer des boucles infinies si ces sous-agents déclenchent le même hook. Pour éviter cela :

* Vérifiez un indicateur de sous-agent dans l'entrée du hook avant de lancer
* Utilisez une variable partagée ou un état de session pour suivre si vous êtes déjà à l'intérieur d'un sous-agent
* Limitez les hooks pour qu'ils s'exécutent uniquement pour la session d'agent de niveau supérieur

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage n'apparaît pas dans la sortie
</h3>

Le champ `systemMessage` affiche un message à l'utilisateur, pas au modèle. Par défaut, le SDK ne fait pas apparaître la sortie du hook dans le flux de messages que pour les hooks `SessionStart` et `Setup`, donc un message provenant de tout autre événement hook n'apparaît pas à moins que vous définissiez `includeHookEvents` (`include_hook_events` en Python). Pour transmettre du contexte au modèle à la place, retournez [`additionalContext`](/docs/fr/hooks#add-context-for-claude).

Si vous avez besoin de faire apparaître les décisions de hook à votre application de manière fiable, enregistrez-les séparément ou utilisez un canal de sortie dédié.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Référence des hooks Claude Code](/docs/fr/hooks) : schémas JSON d'entrée/sortie complets, documentation des événements et motifs matcher
* [Guide des hooks Claude Code](/docs/fr/hooks-guide) : exemples de hooks de commande shell et procédures pas à pas
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) : types de hook, définitions d'entrée/sortie et options de configuration
* [Référence du SDK Python](/docs/fr/agent-sdk/python) : types de hook, définitions d'entrée/sortie et options de configuration
* [Permissions](/docs/fr/agent-sdk/permissions) : contrôlez ce que votre agent peut faire
* [Outils personnalisés](/docs/fr/agent-sdk/custom-tools) : créez des outils pour étendre les capacités de l'agent
