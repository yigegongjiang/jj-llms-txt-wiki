> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gérer les approbations et les entrées utilisateur

> Présentez les demandes d'approbation et les questions de clarification de Claude aux utilisateurs, puis renvoyez leurs décisions au SDK.

Lors du travail sur une tâche, Claude a parfois besoin de vérifier auprès des utilisateurs. Il peut avoir besoin d'une permission avant de supprimer des fichiers, ou avoir besoin de demander quelle base de données utiliser pour un nouveau projet. Votre application doit présenter ces demandes aux utilisateurs afin que Claude puisse continuer avec leurs entrées.

Claude demande une entrée utilisateur dans deux situations : lorsqu'il a besoin d'une **permission pour utiliser un outil** (comme supprimer des fichiers ou exécuter des commandes), et lorsqu'il a des **questions de clarification** (via l'outil `AskUserQuestion`). Les deux déclenchent votre callback `canUseTool`, qui met en pause l'exécution jusqu'à ce que vous retourniez une réponse. C'est différent des tours de conversation normaux où Claude termine et attend votre prochain message.

Pour les questions de clarification, Claude génère les questions et les options. Votre rôle est de les présenter aux utilisateurs et de retourner leurs sélections. Vous ne pouvez pas ajouter vos propres questions à ce flux ; si vous avez besoin de poser une question aux utilisateurs vous-même, faites-le séparément dans votre logique d'application.

Le callback peut rester en attente indéfiniment. L'exécution reste en pause jusqu'à ce que votre callback retourne, et le SDK n'annule l'attente que lorsque la requête elle-même est annulée. Si un utilisateur pourrait prendre plus de temps pour répondre que votre processus ne peut raisonnablement rester en cours d'exécution, retournez la décision du [hook `defer`](/docs/fr/hooks#defer-a-tool-call-for-later), qui permet au processus de quitter et de reprendre plus tard à partir de la session persistante.

Ce guide vous montre comment détecter chaque type de demande et répondre de manière appropriée.

<h2 id="detect-when-claude-needs-input">
  Détecter quand Claude a besoin d'une entrée
</h2>

Passez un callback `canUseTool` dans vos options de requête. Le callback se déclenche chaque fois que Claude a besoin d'une entrée utilisateur, en recevant le nom de l'outil et l'entrée comme arguments :

<CodeGroup>
  ```python Python theme={null}
  async def handle_tool_request(tool_name, input_data, context):
      # Inviter l'utilisateur et retourner allow ou deny
      ...


  options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
  ```

  ```typescript TypeScript theme={null}
  async function handleToolRequest(toolName, input, options) {
    // options includes { signal: AbortSignal, suggestions?: PermissionUpdate[] }
    // Inviter l'utilisateur et retourner allow ou deny
  }

  const options = { canUseTool: handleToolRequest };
  ```
</CodeGroup>

Le callback se déclenche dans deux cas :

1. **L'outil a besoin d'approbation** : Claude veut utiliser un outil qui n'est pas auto-approuvé par une [règle de permission](/docs/fr/agent-sdk/permissions) ou un mode de permission. Vérifiez `tool_name` pour l'outil (par exemple, `"Bash"`, `"Write"`).
2. **Claude pose une question** : Claude appelle l'outil `AskUserQuestion`. Vérifiez si `tool_name == "AskUserQuestion"` pour le gérer différemment. Si vous spécifiez un tableau `tools`, incluez `AskUserQuestion` pour que cela fonctionne. Voir [Gérer les questions de clarification](#handle-clarifying-questions) pour plus de détails.

<Warning>
  **Le callback ne se déclenche jamais pour les outils auto-approuvés.** Toute approbation antérieure dans le [flux d'évaluation des permissions](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated), une règle d'autorisation ou un mode comme `acceptEdits` ou `bypassPermissions`, résout l'appel avant que `canUseTool` soit consulté. Si vous listez un outil seul dans `allowed_tools`, une vérification `canUseTool` pour cet outil ne s'exécute jamais sauf si une règle d'ask ou le mode `plan` redirige l'appel vers une invite. Pour une logique qui doit s'appliquer à chaque appel d'outil, utilisez un [hook `PreToolUse`](/docs/fr/agent-sdk/hooks), qui s'exécute avant le reste du flux et peut autoriser, refuser ou modifier les demandes.

  `AskUserQuestion`, les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool), et les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) atteignent le callback même quand une règle d'autorisation correspond. En mode `dontAsk`, ces appels sont refusés à la place, sans invoquer le callback.
</Warning>

Vous pouvez également utiliser le [hook `PermissionRequest`](/docs/fr/agent-sdk/hooks#available-hooks) pour envoyer des notifications externes (Slack, email, push) quand Claude attend une approbation.

<h2 id="handle-tool-approval-requests">
  Gérer les demandes d'approbation d'outil
</h2>

Une fois que vous avez passé un callback `canUseTool` dans vos options de requête, il se déclenche lorsque Claude veut utiliser un outil qu'aucun élément antérieur du flux de permission n'a approuvé. Votre callback reçoit trois arguments :

| Argument                            | Description                                                                                                                                                                                                                                                                                                                                                            |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `toolName`                          | Le nom de l'outil que Claude veut utiliser (par exemple, `"Bash"`, `"Write"`, `"Edit"`)                                                                                                                                                                                                                                                                                |
| `input`                             | Les paramètres que Claude passe à l'outil. Le contenu varie selon l'outil.                                                                                                                                                                                                                                                                                             |
| `options` (TS) / `context` (Python) | Contexte supplémentaire incluant des `suggestions` optionnelles (entrées `PermissionUpdate` proposées pour éviter de re-inviter) et un signal d'annulation. En TypeScript, `signal` est un `AbortSignal` ; en Python, le champ signal est réservé pour une utilisation future. Voir [`ToolPermissionContext`](/docs/fr/agent-sdk/python#toolpermissioncontext) pour Python. |

L'objet `input` contient des paramètres spécifiques à l'outil. Exemples courants :

| Outil   | Champs d'entrée                         |
| ------- | --------------------------------------- |
| `Bash`  | `command`, `description`, `timeout`     |
| `Write` | `file_path`, `content`                  |
| `Edit`  | `file_path`, `old_string`, `new_string` |
| `Read`  | `file_path`, `offset`, `limit`          |

Voir la référence du SDK pour les schémas d'entrée complets : [Python](/docs/fr/agent-sdk/python#tool-input%2Foutput-types) | [TypeScript](/docs/fr/agent-sdk/typescript#tool-input-types).

Vous pouvez afficher ces informations à l'utilisateur afin qu'il puisse décider d'autoriser ou de rejeter l'action, puis retourner la réponse appropriée.

L'exemple suivant demande à Claude de créer et de supprimer un fichier de test. Lorsque Claude tente chaque opération, le callback imprime la demande d'outil au terminal et invite à une approbation y/n.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import (
      HookMatcher,
      PermissionResultAllow,
      PermissionResultDeny,
      ToolPermissionContext,
  )


  async def can_use_tool(
      tool_name: str, input_data: dict, context: ToolPermissionContext
  ) -> PermissionResultAllow | PermissionResultDeny:
      # Afficher la demande d'outil
      print(f"\nTool: {tool_name}")
      if tool_name == "Bash":
          print(f"Command: {input_data.get('command')}")
          if input_data.get("description"):
              print(f"Description: {input_data.get('description')}")
      else:
          print(f"Input: {input_data}")

      # Obtenir l'approbation de l'utilisateur
      response = input("Allow this action? (y/n): ")

      # Retourner allow ou deny en fonction de la réponse de l'utilisateur
      if response.lower() == "y":
          # Allow: l'outil s'exécute avec l'entrée originale (ou modifiée)
          return PermissionResultAllow(updated_input=input_data)
      else:
          # Deny: l'outil ne s'exécute pas, Claude voit le message
          return PermissionResultDeny(message="User denied this action")


  # Contournement requis : un hook factice garde le flux ouvert pour can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Create a test file in /tmp and then delete it",
          },
      }


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline";

  // Helper to prompt user for input in the terminal
  function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    return new Promise((resolve) =>
      rl.question(question, (answer) => {
        rl.close();
        resolve(answer);
      })
    );
  }

  for await (const message of query({
    prompt: "Create a test file in /tmp and then delete it",
    options: {
      canUseTool: async (toolName, input) => {
        // Afficher la demande d'outil
        console.log(`\nTool: ${toolName}`);
        if (toolName === "Bash") {
          console.log(`Command: ${input.command}`);
          if (input.description) console.log(`Description: ${input.description}`);
        } else {
          console.log(`Input: ${JSON.stringify(input, null, 2)}`);
        }

        // Obtenir l'approbation de l'utilisateur
        const response = await prompt("Allow this action? (y/n): ");

        // Retourner allow ou deny en fonction de la réponse de l'utilisateur
        if (response.toLowerCase() === "y") {
          // Allow: l'outil s'exécute avec l'entrée originale (ou modifiée)
          return { behavior: "allow", updatedInput: input };
        } else {
          // Deny: l'outil ne s'exécute pas, Claude voit le message
          return { behavior: "deny", message: "User denied this action" };
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<Note>
  En Python, `can_use_tool` nécessite le [mode streaming](/docs/fr/agent-sdk/streaming-vs-single-mode). Lorsque vous passez un flux de messages fini via `query(prompt=generator)` ou `ClaudeSDKClient.connect(prompt=async_iterable)`, le SDK ferme le flux d'entrée après le dernier message, avant que le callback de permission puisse être invoqué, sauf si un hook enregistré ou un serveur MCP en cours de processus le maintient ouvert. L'exemple ci-dessus le maintient ouvert avec un hook `PreToolUse` qui retourne `{"continue_": True}`. La connexion sans prompt et l'envoi de messages via `ClaudeSDKClient.query()` maintiennent le flux ouvert d'eux-mêmes et ne nécessitent aucun hook.
</Note>

Cet exemple utilise un flux y/n où toute entrée autre que `y` est traitée comme un refus. En pratique, vous pourriez construire une interface utilisateur plus riche qui permet aux utilisateurs de modifier la demande, de fournir des commentaires, ou de rediriger Claude entièrement. Voir [Répondre aux demandes d'outil](#respond-to-tool-requests) pour tous les moyens de répondre.

<h3 id="respond-to-tool-requests">
  Répondre aux demandes d'outil
</h3>

Votre callback retourne l'un de deux types de réponse :

| Réponse   | Python                                     | TypeScript                            |
| --------- | ------------------------------------------ | ------------------------------------- |
| **Allow** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **Deny**  | `PermissionResultDeny(message=...)`        | `{ behavior: "deny", message }`       |

Lors de l'autorisation, l'outil s'exécute avec l'entrée que Claude a demandée, sauf si vous retournez une entrée modifiée, `updatedInput` en TypeScript ou `updated_input` en Python. Avant la v2.1.207, Claude Code rejetait un résultat allow qui omettait `updatedInput` et refusait l'appel d'outil avec une erreur de validation.

Lors du refus, fournissez un message expliquant pourquoi. Claude voit ce message et peut ajuster son approche.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk.types import PermissionResultAllow, PermissionResultDeny

  # Autoriser l'outil à s'exécuter
  return PermissionResultAllow(updated_input=input_data)

  # Bloquer l'outil
  return PermissionResultDeny(message="User rejected this action")
  ```

  ```typescript TypeScript theme={null}
  // Autoriser l'outil à s'exécuter
  return { behavior: "allow", updatedInput: input };

  // Bloquer l'outil
  return { behavior: "deny", message: "User rejected this action" };
  ```
</CodeGroup>

Au-delà de l'autorisation ou du refus, vous pouvez modifier l'entrée de l'outil ou fournir un contexte qui aide Claude à ajuster son approche :

* **Approuver** : laisser l'outil s'exécuter comme Claude l'a demandé
* **Approuver avec des modifications** : modifier l'entrée avant l'exécution (par exemple, nettoyer les chemins, ajouter des contraintes)
* **Approuver et mémoriser** : renvoyer une règle de permission suggérée pour que les appels correspondants ignorent l'invite la prochaine fois
* **Rejeter** : bloquer l'outil et dire à Claude pourquoi
* **Suggérer une alternative** : bloquer mais guider Claude vers ce que l'utilisateur veut à la place
* **Rediriger entièrement** : utiliser [streaming input](/docs/fr/agent-sdk/streaming-vs-single-mode) pour envoyer à Claude une instruction complètement nouvelle

<Tabs>
  <Tab title="Approuver">
    L'utilisateur approuve l'action telle quelle. Passez l'`input` de votre callback inchangée et l'outil s'exécute exactement comme Claude l'a demandé.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          print(f"Claude wants to use {tool_name}")
          approved = await ask_user("Allow this action?")

          if approved:
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        console.log(`Claude wants to use ${toolName}`);
        const approved = await askUser("Allow this action?");

        if (approved) {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Approuver avec des modifications">
    L'utilisateur approuve mais veut d'abord modifier la demande. Vous pouvez modifier l'entrée avant l'exécution de l'outil. Claude voit le résultat mais n'est pas informé que vous avez changé quelque chose. Utile pour nettoyer les paramètres, ajouter des contraintes, ou limiter l'accès.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash":
              # L'utilisateur a approuvé, mais limiter toutes les commandes au sandbox
              sandboxed_input = {**input_data}
              sandboxed_input["command"] = input_data["command"].replace(
                  "/tmp", "/tmp/sandbox"
              )
              return PermissionResultAllow(updated_input=sandboxed_input)
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash") {
          // L'utilisateur a approuvé, mais limiter toutes les commandes au sandbox
          const sandboxedInput = {
            ...input,
            command: input.command.replace("/tmp", "/tmp/sandbox")
          };
          return { behavior: "allow", updatedInput: sandboxedInput };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Approuver et mémoriser">
    L'utilisateur approuve et ne veut pas être invité à nouveau pour ce type d'appel. Le troisième argument du callback porte `suggestions`, un tableau d'entrées [`PermissionUpdate`](/docs/fr/agent-sdk/typescript#permissionupdate) prêtes à l'emploi. Renvoyez-en une dans `updatedPermissions` pour l'appliquer. Une suggestion avec la destination `localSettings` écrit la règle dans `.claude/settings.local.json` afin que les futures sessions ignorent l'invite pour les appels correspondants.

    L'exemple Python nécessite `claude-agent-sdk` 0.1.80 ou ultérieur.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])

          if choice == "always":
              persist = [
                  s for s in context.suggestions if s.destination == "localSettings"
              ]
              return PermissionResultAllow(
                  updated_input=input_data, updated_permissions=persist
              )
          if choice == "once":
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input, { suggestions = [] }) => {
        const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);

        if (choice === "always") {
          const persist = suggestions.filter(
            (s) => s.destination === "localSettings"
          );
          return {
            behavior: "allow",
            updatedInput: input,
            updatedPermissions: persist
          };
        }
        if (choice === "once") {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Rejeter">
    L'utilisateur ne veut pas que cette action se produise. Bloquez l'outil et fournissez un message expliquant pourquoi. Claude voit ce message et peut essayer une approche différente.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          approved = await ask_user(f"Allow {tool_name}?")

          if not approved:
              return PermissionResultDeny(message="User rejected this action")
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        const approved = await askUser(`Allow ${toolName}?`);

        if (!approved) {
          return {
            behavior: "deny",
            message: "User rejected this action"
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Suggérer une alternative">
    L'utilisateur ne veut pas cette action spécifique, mais a une idée différente. Bloquez l'outil et incluez des conseils dans votre message. Claude lira ceci et décidera comment procéder en fonction de vos commentaires.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash" and "rm" in input_data.get("command", ""):
              # L'utilisateur ne veut pas supprimer, suggérer d'archiver à la place
              return PermissionResultDeny(
                  message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
              )
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash" && input.command.includes("rm")) {
          // L'utilisateur ne veut pas supprimer, suggérer d'archiver à la place
          return {
            behavior: "deny",
            message:
              "User doesn't want to delete files. They asked if you could compress them into an archive instead."
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Rediriger entièrement">
    Pour un changement de direction complet (pas seulement une nudge), utilisez [streaming input](/docs/fr/agent-sdk/streaming-vs-single-mode) pour envoyer à Claude une nouvelle instruction directement. Cela contourne la demande d'outil actuelle et donne à Claude des instructions entièrement nouvelles à suivre.
  </Tab>
</Tabs>

<h2 id="handle-clarifying-questions">
  Gérer les questions de clarification
</h2>

Lorsque Claude a besoin de plus de direction sur une tâche avec plusieurs approches valides, il appelle l'outil `AskUserQuestion`. Cela déclenche votre callback `canUseTool` avec `toolName` défini sur `AskUserQuestion`. L'entrée contient les questions de Claude sous forme d'options à choix multiples, que vous affichez à l'utilisateur et retournez ses sélections.

<Tip>
  Les questions de clarification sont particulièrement courantes en [mode `plan`](/docs/fr/agent-sdk/permissions#plan-mode-plan), où Claude explore la base de code et pose des questions avant de proposer un plan. Cela rend le mode plan idéal pour les flux de travail interactifs où vous voulez que Claude rassemble les exigences avant de faire des modifications.
</Tip>

Les étapes suivantes montrent comment gérer les questions de clarification :

<Steps>
  <Step title="Passer un callback canUseTool">
    Passez un callback `canUseTool` dans vos options de requête. Par défaut, `AskUserQuestion` est disponible. Si vous spécifiez un tableau `tools` pour restreindre les capacités de Claude (par exemple, un agent en lecture seule avec seulement `Read`, `Glob`, et `Grep`), incluez `AskUserQuestion` dans ce tableau. Sinon, Claude ne pourra pas poser de questions de clarification :

    <CodeGroup>
      ```python Python theme={null}
      async for message in query(
          prompt="Analyze this codebase",
          options=ClaudeAgentOptions(
              # Inclure AskUserQuestion dans votre liste d'outils
              tools=["Read", "Glob", "Grep", "AskUserQuestion"],
              can_use_tool=can_use_tool,
          ),
      ):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      for await (const message of query({
        prompt: "Analyze this codebase",
        options: {
          // Inclure AskUserQuestion dans votre liste d'outils
          tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
          canUseTool: async (toolName, input) => {
            // Gérer les questions de clarification ici
          }
        }
      })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Détecter AskUserQuestion">
    Dans votre callback, vérifiez si `toolName` est égal à `AskUserQuestion` pour le gérer différemment des autres outils :

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name: str, input_data: dict, context):
          if tool_name == "AskUserQuestion":
              # Votre implémentation pour collecter les réponses de l'utilisateur
              return await handle_clarifying_questions(input_data)
          # Gérer les autres outils normalement
          return await prompt_for_approval(tool_name, input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "AskUserQuestion") {
          // Votre implémentation pour collecter les réponses de l'utilisateur
          return handleClarifyingQuestions(input);
        }
        // Gérer les autres outils normalement
        return promptForApproval(toolName, input);
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Analyser l'entrée de la question">
    L'entrée contient les questions de Claude dans un tableau `questions`. Chaque question a une `question` (le texte à afficher), des `options` (les choix), et `multiSelect` (si plusieurs sélections sont autorisées) :

    ```json theme={null}
    {
      "questions": [
        {
          "question": "How should I format the output?",
          "header": "Format",
          "options": [
            { "label": "Summary", "description": "Brief overview" },
            { "label": "Detailed", "description": "Full explanation" }
          ],
          "multiSelect": false
        },
        {
          "question": "Which sections should I include?",
          "header": "Sections",
          "options": [
            { "label": "Introduction", "description": "Opening context" },
            { "label": "Conclusion", "description": "Final summary" }
          ],
          "multiSelect": true
        }
      ]
    }
    ```

    Voir [Format de question](#question-format) pour les descriptions complètes des champs.
  </Step>

  <Step title="Collecter les réponses de l'utilisateur">
    Présentez les questions à l'utilisateur et collectez ses sélections. La façon dont vous le faites dépend de votre application : une invite de terminal, un formulaire web, un dialogue mobile, etc.
  </Step>

  <Step title="Retourner les réponses à Claude">
    Construisez l'objet `answers` comme un enregistrement où chaque clé est le texte de `question` et chaque valeur est le `label` de l'option sélectionnée :

    | De l'objet question                                                 | Utiliser comme |
    | ------------------------------------------------------------------- | -------------- |
    | Champ `question` (par exemple, `"How should I format the output?"`) | Clé            |
    | Champ `label` de l'option sélectionnée (par exemple, `"Summary"`)   | Valeur         |

    Pour les questions multi-sélection, passez un tableau de labels ou joignez-les avec `", "`. Si vous [supportez l'entrée de texte libre](#support-free-text-input), utilisez le texte personnalisé de l'utilisateur comme valeur.

    <CodeGroup>
      ```python Python theme={null}
      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": {
                  "How should I format the output?": "Summary",
                  "Which sections should I include?": ["Introduction", "Conclusion"],
              },
          }
      )
      ```

      ```typescript TypeScript theme={null}
      return {
        behavior: "allow",
        updatedInput: {
          questions: input.questions,
          answers: {
            "How should I format the output?": "Summary",
            "Which sections should I include?": "Introduction, Conclusion"
          }
        }
      };
      ```
    </CodeGroup>
  </Step>
</Steps>

<h3 id="question-format">
  Format de question
</h3>

L'entrée contient les questions générées par Claude dans un tableau `questions`. Chaque question a ces champs :

| Champ         | Description                                                                                                                                         |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `question`    | Le texte complet de la question à afficher                                                                                                          |
| `header`      | Étiquette courte pour la question (max 12 caractères)                                                                                               |
| `options`     | Tableau de 2-4 choix, chacun avec `label` et `description`. TypeScript : optionnellement `preview` (voir [ci-dessous](#option-previews-typescript)) |
| `multiSelect` | Si `true`, les utilisateurs peuvent sélectionner plusieurs options                                                                                  |

La structure que votre callback reçoit :

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

<h4 id="option-previews-typescript">
  Aperçus d'options (TypeScript)
</h4>

`toolConfig.askUserQuestion.previewFormat` ajoute un champ `preview` à chaque option afin que votre application puisse afficher une maquette visuelle à côté du label. Sans ce paramètre, Claude ne génère pas d'aperçus et le champ est absent.

| `previewFormat`         | `preview` contient                                                                                                        |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| non défini (par défaut) | Le champ est absent. Claude ne génère pas d'aperçus.                                                                      |
| `"markdown"`            | Art ASCII et blocs de code clôturés                                                                                       |
| `"html"`                | Un fragment `<div>` stylisé (le SDK rejette `<script>`, `<style>`, et `<!DOCTYPE>` avant que votre callback ne s'exécute) |

Le format s'applique à toutes les questions de la session. Claude inclut `preview` sur les options où une comparaison visuelle aide (choix de mise en page, schémas de couleurs) et l'omet où ce ne serait pas le cas (confirmations oui/non, choix texte uniquement). Vérifiez `undefined` avant de rendre.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview is an HTML string or undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

Une option avec un aperçu HTML :

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

<h3 id="response-format">
  Format de réponse
</h3>

Retournez un objet `answers` mappant le champ `question` de chaque question au `label` de l'option sélectionnée :

| Champ       | Description                                                                                          |
| ----------- | ---------------------------------------------------------------------------------------------------- |
| `questions` | Passez le tableau de questions original (requis pour le traitement de l'outil)                       |
| `answers`   | Objet où les clés sont le texte de la question et les valeurs sont les labels sélectionnés           |
| `response`  | Réponse freeform optionnelle que l'utilisateur a tapée au lieu de répondre aux questions structurées |

Pour les questions multi-sélection, passez un tableau de labels ou joignez-les avec `", "`. Pour l'entrée de texte libre par question, comme une option « Autre », mettez le texte de l'utilisateur dans `answers[question]` comme indiqué dans [Supporter l'entrée de texte libre](#support-free-text-input). Définissez `response` uniquement lorsque votre interface utilisateur permet à l'utilisateur de rejeter la carte de question et de taper une réponse générale qui n'est pas une réponse à une question spécifique. Lorsque `response` est défini, Claude reçoit « L'utilisateur a répondu : … » au lieu de la liste de réponses par question.

```json theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

<h4 id="support-free-text-input">
  Supporter l'entrée de texte libre
</h4>

Les options prédéfinies de Claude ne couvriront pas toujours ce que les utilisateurs veulent. Pour permettre aux utilisateurs de taper leur propre réponse :

* Affichez un choix supplémentaire « Autre » après les options de Claude qui accepte l'entrée de texte
* Utilisez le texte personnalisé de l'utilisateur comme valeur de réponse (pas le mot « Autre »)

Voir l'[exemple complet](#complete-example) ci-dessous pour une implémentation complète.

<h3 id="complete-example">
  Exemple complet
</h3>

Claude pose des questions de clarification lorsqu'il a besoin d'une entrée utilisateur pour continuer. Par exemple, lorsqu'on lui demande d'aider à décider d'une pile technologique pour une application mobile, Claude pourrait poser des questions sur cross-platform vs native, les préférences de backend, ou les plates-formes cibles. Ces questions aident Claude à prendre des décisions qui correspondent aux préférences de l'utilisateur plutôt que de deviner.

Cet exemple gère ces questions dans une application de terminal. Voici ce qui se passe à chaque étape :

1. **Router la demande** : Le callback `canUseTool` vérifie si le nom de l'outil est `"AskUserQuestion"` et route vers un gestionnaire dédié
2. **Afficher les questions** : Le gestionnaire boucle à travers le tableau `questions` et imprime chaque question avec des options numérotées
3. **Collecter l'entrée** : L'utilisateur peut entrer un numéro pour sélectionner une option, ou taper du texte libre directement (par exemple, « jquery », « je ne sais pas »)
4. **Mapper les réponses** : Le code vérifie si l'entrée est numérique (utilise le label de l'option) ou du texte libre (utilise le texte directement)
5. **Retourner à Claude** : La réponse inclut à la fois le tableau `questions` original et le mapping `answers`

Enregistrez la version TypeScript sous `ask.ts` et exécutez-la avec `npx tsx ask.ts`, ou enregistrez la version Python sous `ask.py` et exécutez-la avec `python ask.py`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


  def parse_response(response: str, options: list) -> str:
      """Analyser l'entrée utilisateur comme numéro(s) d'option ou texte libre."""
      try:
          indices = [int(s.strip()) - 1 for s in response.split(",")]
          labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
          return ", ".join(labels) if labels else response
      except ValueError:
          return response


  async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
      """Afficher les questions de Claude et collecter les réponses de l'utilisateur."""
      answers = {}

      for q in input_data.get("questions", []):
          print(f"\n{q['header']}: {q['question']}")

          options = q["options"]
          for i, opt in enumerate(options):
              print(f"  {i + 1}. {opt['label']} - {opt['description']}")
          if q.get("multiSelect"):
              print("  (Enter numbers separated by commas, or type your own answer)")
          else:
              print("  (Enter a number, or type your own answer)")

          response = input("Your choice: ").strip()
          answers[q["question"]] = parse_response(response, options)

      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": answers,
          }
      )


  async def can_use_tool(
      tool_name: str, input_data: dict, context
  ) -> PermissionResultAllow:
      # Router AskUserQuestion vers notre gestionnaire de questions
      if tool_name == "AskUserQuestion":
          return await handle_ask_user_question(input_data)
      # Auto-approuver les autres outils pour cet exemple
      return PermissionResultAllow(updated_input=input_data)


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Help me decide on the tech stack for a new mobile app",
          },
      }


  # Contournement requis : un hook factice garde le flux ouvert pour can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline/promises";

  // Helper to prompt user for input in the terminal
  async function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
    const answer = await rl.question(question);
    rl.close();
    return answer;
  }

  // Parse user input as option number(s) or free text
  function parseResponse(response: string, options: any[]): string {
    const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
    const labels = indices
      .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
      .map((i) => options[i].label);
    return labels.length > 0 ? labels.join(", ") : response;
  }

  // Display Claude's questions and collect user answers
  async function handleAskUserQuestion(input: any) {
    const answers: Record<string, string> = {};

    for (const q of input.questions) {
      console.log(`\n${q.header}: ${q.question}`);

      const options = q.options;
      options.forEach((opt: any, i: number) => {
        console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
      });
      if (q.multiSelect) {
        console.log("  (Enter numbers separated by commas, or type your own answer)");
      } else {
        console.log("  (Enter a number, or type your own answer)");
      }

      const response = (await prompt("Your choice: ")).trim();
      answers[q.question] = parseResponse(response, options);
    }

    // Return the answers to Claude (must include original questions)
    return {
      behavior: "allow",
      updatedInput: { questions: input.questions, answers }
    };
  }

  async function main() {
    for await (const message of query({
      prompt: "Help me decide on the tech stack for a new mobile app",
      options: {
        canUseTool: async (toolName, input) => {
          // Router AskUserQuestion vers notre gestionnaire de questions
          if (toolName === "AskUserQuestion") {
            return handleAskUserQuestion(input);
          }
          // Auto-approuver les autres outils pour cet exemple
          return { behavior: "allow", updatedInput: input };
        }
      }
    })) {
      if ("result" in message) console.log(message.result);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="limitations">
  Limitations
</h2>

* **Subagents** : `AskUserQuestion` n'est actuellement pas disponible dans les subagents générés via l'outil Agent
* **Limites de questions** : chaque appel `AskUserQuestion` supporte 1-4 questions avec 2-4 options chacune

<h2 id="other-ways-to-get-user-input">
  Autres façons d'obtenir une entrée utilisateur
</h2>

Le callback `canUseTool` et l'outil `AskUserQuestion` couvrent la plupart des scénarios d'approbation et de clarification, mais le SDK offre d'autres façons d'obtenir une entrée des utilisateurs :

<h3 id="streaming-input">
  Streaming input
</h3>

Utilisez [streaming input](/docs/fr/agent-sdk/streaming-vs-single-mode) lorsque vous avez besoin de :

* **Interrompre l'agent en cours de tâche** : envoyer un signal d'annulation ou changer de direction pendant que Claude travaille
* **Fournir un contexte supplémentaire** : ajouter des informations dont Claude a besoin sans attendre qu'il les demande
* **Construire des interfaces de chat** : permettre aux utilisateurs d'envoyer des messages de suivi pendant les opérations longues

Streaming input est idéal pour les interfaces conversationnelles où les utilisateurs interagissent avec l'agent tout au long de l'exécution, pas seulement aux points d'approbation.

<h3 id="custom-tools">
  Outils personnalisés
</h3>

Utilisez [outils personnalisés](/docs/fr/agent-sdk/custom-tools) lorsque vous avez besoin de :

* **Collecter une entrée structurée** : construire des formulaires, des assistants, ou des flux de travail multi-étapes qui vont au-delà du format à choix multiples de `AskUserQuestion`
* **Intégrer des systèmes d'approbation externes** : se connecter à des plates-formes de ticketing, de flux de travail, ou d'approbation existantes
* **Implémenter des interactions spécifiques au domaine** : créer des outils adaptés aux besoins de votre application, comme des interfaces d'examen de code ou des listes de contrôle de déploiement

Les outils personnalisés vous donnent un contrôle total sur l'interaction, mais nécessitent plus de travail d'implémentation que d'utiliser le callback `canUseTool` intégré.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Configurer les permissions](/docs/fr/agent-sdk/permissions) : configurer les modes et règles de permission
* [Contrôler l'exécution avec les hooks](/docs/fr/agent-sdk/hooks) : exécuter du code personnalisé à des points clés du cycle de vie de l'agent
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript#canusetool) : documentation complète de l'API canUseTool
