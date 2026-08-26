> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer les permissions

> Contrôlez comment votre agent utilise les outils avec les modes de permission, les hooks et les règles de permission/refus déclaratives.

Le Claude Agent SDK fournit des contrôles de permission pour gérer la façon dont Claude utilise les outils. Utilisez les modes de permission et les règles pour définir ce qui est autorisé automatiquement, et le callback [`canUseTool`](/docs/fr/agent-sdk/user-input) pour gérer tout le reste à l'exécution.

<Note>
  Cette page couvre les modes de permission et les règles. Pour créer des flux d'approbation interactifs où les utilisateurs approuvent ou refusent les demandes d'outils à l'exécution, consultez [Gérer les approbations et les entrées utilisateur](/docs/fr/agent-sdk/user-input).
</Note>

<h2 id="how-permissions-are-evaluated">
  Comment les permissions sont évaluées
</h2>

Lorsque Claude demande un outil, le SDK vérifie les permissions dans cet ordre :

<Steps>
  <Step title="Hooks">
    Exécutez d'abord les [hooks](/docs/fr/agent-sdk/hooks). Un hook peut refuser l'appel directement ou le transmettre. Un hook qui retourne `allow` ne saute pas les règles de refus et de demande ci-dessous ; celles-ci sont évaluées indépendamment du résultat du hook.
  </Step>

  <Step title="Règles de refus">
    Vérifiez les règles `deny` (à partir de `disallowed_tools` et [settings.json](/docs/fr/settings#permission-settings)). Si une règle de refus correspond, l'outil est bloqué, même en mode `bypassPermissions`. Les règles de nom simple comme `Bash` suppriment l'outil du contexte de Claude avant que cette évaluation ne commence, donc seules les règles délimitées comme `Bash(rm *)` sont vérifiées à cette étape.
  </Step>

  <Step title="Règles de demande">
    Vérifiez les règles `ask` à partir de [settings.json](/docs/fr/settings#permission-settings). Si une règle de demande correspond, l'appel passe à votre callback [`canUseTool`](/docs/fr/agent-sdk/user-input) pour confirmation, même en mode `bypassPermissions`.

    Les outils qui nécessitent une interaction utilisateur se comportent de la même manière : `AskUserQuestion` et les outils MCP dont le serveur définit [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) passent toujours au callback, même lorsqu'une règle d'autorisation correspond. En mode `dontAsk`, les deux cas sont refusés à la place, car ce mode ne demande jamais. L'annotation MCP nécessite Claude Code v2.1.199 ou ultérieur.

    Les outils du connecteur [claude.ai](/docs/fr/mcp#organization-controls-on-connector-tools) que votre organisation a définis sur `ask` quittent également le flux à cette étape. Chaque appel passe au callback, même en mode `bypassPermissions` et même lorsqu'une règle d'autorisation correspond. Le callback reçoit la raison `Your organization requires approval for this tool`. En mode `dontAsk`, l'appel est refusé à la place, car ce mode ne demande jamais.
  </Step>

  <Step title="Mode de permission">
    Appliquez le [mode de permission](#permission-modes) actif. `bypassPermissions` approuve tout ce qui atteint cette étape. `acceptEdits` approuve les opérations de fichiers. `plan` achemine les outils d'édition de fichiers et d'écriture shell vers votre callback `canUseTool` indépendamment des règles d'autorisation, donc les opérations d'écriture ne peuvent pas être approuvées automatiquement lors de la planification. Les autres modes passent au suivant.
  </Step>

  <Step title="Règles d'autorisation">
    Vérifiez les règles `allow` (à partir de `allowed_tools` et settings.json). Si une règle correspond, l'outil est approuvé.
  </Step>

  <Step title="Callback canUseTool">
    Si aucune des étapes ci-dessus ne résout le problème, appelez votre callback [`canUseTool`](/docs/fr/agent-sdk/user-input) pour une décision. En mode `dontAsk`, cette étape est ignorée et l'outil est refusé.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="Diagramme du flux d'évaluation des permissions en six étapes correspondant aux étapes ci-dessus : une demande d'outil passe par les hooks, les règles de refus, les règles de demande, le mode de permission, les règles d'autorisation et canUseTool. Les hooks, les règles de refus et canUseTool peuvent router vers Bloqué ; le contournement du mode de permission, les règles d'autorisation et canUseTool peuvent router vers Exécuter ; les règles de demande routent vers canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

À partir de la v2.1.198, si vous transmettez un callback `canUseTool` que cet ordre d'évaluation ne peut jamais atteindre, le SDK TypeScript émet un avertissement du processus Node.js une fois lorsque la requête est construite. Le code de l'avertissement est `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Deux configurations le déclenchent :

* `permissionMode: 'bypassPermissions'`, qui approuve automatiquement chaque appel qui atteint l'étape du mode de permission
* Chaque entrée `allowedTools` simple comme `"Read"`, qui approuve automatiquement cet outil entier avant que le callback ne soit consulté

Les entrées avec un spécificateur comme `Bash(ls *)` et le mode `acceptEdits` ne le déclenchent pas, et les règles d'autorisation provenant des fichiers de paramètres ne sont pas visibles pour la vérification.

Écoutez avec `process.on('warning', ...)` et faites correspondre le code pour le journaliser ou le supprimer. Pour contrôler chaque appel d'outil indépendamment du mode et des règles, utilisez plutôt un [hook `PreToolUse`](/docs/fr/agent-sdk/hooks).

Cette page se concentre sur les **règles d'autorisation et de refus** et les **modes de permission**. Pour les autres étapes :

* **Hooks :** exécutez du code personnalisé pour autoriser, refuser ou modifier les demandes d'outils. Consultez [Contrôler l'exécution avec les hooks](/docs/fr/agent-sdk/hooks).
* **Callback canUseTool :** invitez les utilisateurs à approuver à l'exécution, lorsqu'aucune étape antérieure ne résout l'appel. Consultez [Gérer les approbations et les entrées utilisateur](/docs/fr/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Règles d'autorisation et de refus
</h2>

`allowed_tools` et `disallowed_tools` (TypeScript : `allowedTools` / `disallowedTools`) ajoutent des entrées aux listes de règles d'autorisation et de refus dans le flux d'évaluation ci-dessus. Les règles d'autorisation affectent uniquement l'approbation : un outil non listé dans `allowed_tools` est toujours disponible pour Claude et passe au mode de permission. Les règles de refus se comportent différemment selon qu'elles nomment un outil ou délimitent un motif au sein de celui-ci.

| Option                            | Effet                                                                                                                                                                                                                               |
| :-------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` et `Grep` sont auto-approuvés. Les outils non listés ici existent toujours et passent au mode de permission et à `canUseTool`.                                                                                               |
| `disallowed_tools=["Bash"]`       | La définition de l'outil `Bash` est supprimée de la requête. Claude ne voit pas l'outil et ne peut pas le tenter.                                                                                                                   |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` reste disponible. Les appels correspondant à `rm *` sont refusés dans tous les modes de permission, y compris `bypassPermissions`. Les autres appels `Bash` passent au mode de permission.                                   |
| `disallowed_tools=["*"]`          | Chaque définition d'outil est supprimée de la requête. Les globs de noms d'outils sont pris en charge dans les règles de refus : `"*"` correspond à chaque outil et `"mcp__*"` correspond à chaque outil MCP sur tous les serveurs. |

Les règles d'autorisation acceptent les globs de noms d'outils uniquement après un préfixe littéral `mcp__<server>__`. Le segment serveur doit être sans glob afin que la règle nomme un serveur spécifique que vous avez configuré : `mcp__puppeteer__*` correspond à chaque outil du serveur `puppeteer`, et `mcp__github__get_*` correspond à ses outils `get_`. Une entrée non ancrée comme `allowed_tools=["*"]` ou `allowed_tools=["mcp__*"]` est ignorée avec un avertissement au démarrage et n'auto-approuve rien.

Les règles délimitées pour `Read` et `Edit` prennent un motif de chemin. Les règles `Edit(path)` régissent tous les outils intégrés qui écrivent des fichiers, y compris `Write` et `NotebookEdit` ; une règle `Write(path)` n'est jamais mise en correspondance par les vérifications de permission de fichier.

Utilisez `//path` pour un chemin de système de fichiers absolu : une règle de refus de `Edit(//secrets/**)` bloque les écritures n'importe où sous `/secrets` sur le disque. Avec une seule barre oblique, `Edit(/secrets/**)` s'ancre à la source de la règle à la place. Pour les règles transmises via `allowed_tools` ou `disallowed_tools`, cela signifie le répertoire de travail de la session, de sorte que la règle ne bloque pas `/secrets` sur le disque. Consultez [Règles Read et Edit](/docs/fr/permissions#read-and-edit) pour les quatre formes d'ancrage et la façon dont les règles des fichiers de paramètres se résolvent.

<Warning>
  **Les outils auto-approuvés ne parviennent jamais à `canUseTool`.** Un appel d'outil approuvé à n'importe quelle étape antérieure, par `acceptEdits` ou `bypassPermissions`, ou par une règle d'autorisation, ignore votre rappel `canUseTool`, de sorte que les vérifications de permission que vous y mettez sont silencieusement contournées pour cet outil. `AskUserQuestion`, les outils MCP marqués [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool), et les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) parviennent toujours au rappel, même lorsqu'une règle d'autorisation correspond.

  La couverture dépend de la forme de l'entrée : un nom nu comme `Read` ou `mcp__github__get_issue` auto-approuve chaque appel à cet outil, tandis qu'une règle délimitée comme `Bash(ls *)` auto-approuve uniquement les appels correspondants et les autres appels `Bash` passent toujours au rappel. Pour les vérifications qui doivent s'exécuter sur chaque appel d'outil, utilisez un hook [`PreToolUse`](/docs/fr/agent-sdk/hooks) : les hooks s'exécutent avant chaque autre étape, et un refus de hook s'applique même en mode `bypassPermissions`.
</Warning>

Pour un agent verrouillé, associez `allowedTools` avec `permissionMode: "dontAsk"`. Les outils listés sont approuvés, à l'exception des outils toujours-demandés mentionnés dans l'avertissement ci-dessus ; tout le reste est refusé directement au lieu de demander :

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools` ne contraint pas `bypassPermissions`.** `allowed_tools` pré-approuve uniquement les outils que vous listez. Les outils non listés ne correspondent à aucune règle d'autorisation et passent au mode de permission, où `bypassPermissions` les approuve. Définir `allowed_tools=["Read"]` avec `permission_mode="bypassPermissions"` approuve toujours tous les outils, y compris `Bash`, `Write` et `Edit`. Si vous avez besoin de `bypassPermissions` mais que vous voulez que certains outils soient bloqués, utilisez `disallowed_tools`.
</Warning>

Vous pouvez également configurer les règles d'autorisation, de refus et de demande de manière déclarative dans `.claude/settings.json`. Ces règles sont lues lorsque la source de paramètre `project` est activée, ce qui est le cas pour les options `query()` par défaut. Si vous définissez `setting_sources` (TypeScript : `settingSources`) explicitement, incluez `"project"` pour qu'elles s'appliquent. Consultez [Paramètres de permission](/docs/fr/settings#permission-settings) pour la syntaxe des règles.

<h2 id="permission-modes">
  Modes de permission
</h2>

Les modes de permission fournissent un contrôle global sur la façon dont Claude utilise les outils. Vous pouvez définir le mode de permission lors de l'appel de `query()` ou le modifier dynamiquement pendant les sessions de streaming.

<h3 id="available-modes">
  Modes disponibles
</h3>

Le SDK supporte ces modes de permission :

| Mode                | Description                                 | Comportement de l'outil                                                                                                                                                                                                                                                                                                                                   |
| :------------------ | :------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportement de permission standard         | Pas d'auto-approbations ; les outils non appariés déclenchent votre callback `canUseTool`                                                                                                                                                                                                                                                                 |
| `dontAsk`           | Refuser au lieu de demander                 | Tout ce qui n'est pas pré-approuvé par `allowed_tools` ou les règles est refusé ; les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils qui nécessitent une interaction utilisateur sont refusés même si vous les avez pré-approuvés. `canUseTool` n'est jamais appelé        |
| `acceptEdits`       | Auto-accepter les modifications de fichiers | Les modifications de fichiers et les [opérations du système de fichiers](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, etc.) sont automatiquement approuvées                                                                                                                                                                                      |
| `bypassPermissions` | Contourner les contrôles de permission      | Les outils s'exécutent sans invites de permission, sauf les outils correspondant à une [règle `ask`](#how-permissions-are-evaluated) explicite, les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils qui nécessitent une interaction utilisateur (à utiliser avec prudence) |
| `plan`              | Mode de planification                       | Claude explore et planifie sans modifier vos fichiers source ; les modifications de fichiers ne sont jamais auto-approuvées et demandent via votre callback `canUseTool`                                                                                                                                                                                  |
| `auto`              | Approbations classées par modèle            | Un classificateur de modèle approuve ou refuse chaque appel d'outil. Consultez [Mode Auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) pour la disponibilité                                                                                                                                                                                   |

<Warning>
  **Héritage des sous-agents :** Lorsque le parent utilise `bypassPermissions`, `acceptEdits` ou `auto`, tous les sous-agents héritent de ce mode et il ne peut pas être remplacé par sous-agent. Les sous-agents peuvent avoir des invites système différentes et un comportement moins contraint que votre agent principal, donc hériter de `bypassPermissions` leur accorde un accès système complet et autonome. Une [règle `ask`](#how-permissions-are-evaluated) explicite, les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils qui nécessitent une interaction utilisateur forcent toujours une invite.
</Warning>

<h3 id="set-permission-mode">
  Définir le mode de permission
</h3>

Vous pouvez définir le mode de permission une fois au démarrage d'une requête, ou le modifier dynamiquement pendant que la session est active.

<Tabs>
  <Tab title="Au moment de la requête">
    Passez `permission_mode` (Python) ou `permissionMode` (TypeScript) lors de la création d'une requête. Ce mode s'applique pour toute la session sauf s'il est modifié dynamiquement.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Pendant le streaming">
    Appelez `set_permission_mode()` (Python) ou `setPermissionMode()` (TypeScript) pour modifier le mode en cours de session. Le nouveau mode prend effet immédiatement pour toutes les demandes d'outils suivantes. Cela vous permet de commencer de manière restrictive et d'assouplir les permissions à mesure que la confiance augmente, par exemple en passant à `acceptEdits` après avoir examiné l'approche initiale de Claude.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  Détails des modes
</h3>

<h4 id="accept-edits-mode-acceptedits">
  Mode d'acceptation des modifications (`acceptEdits`)
</h4>

Auto-approuve les opérations de fichiers afin que Claude puisse modifier le code sans demander. Les autres outils (comme les commandes Bash qui ne sont pas des opérations du système de fichiers) nécessitent toujours des permissions normales.

**Opérations auto-approuvées :**

* Modifications de fichiers (outils Edit, Write)
* Commandes du système de fichiers : `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Les deux s'appliquent uniquement aux chemins à l'intérieur du répertoire de travail ou de `additionalDirectories`. Les chemins en dehors de cette portée et les écritures vers des chemins protégés demandent toujours.

**À utiliser quand :** vous faites confiance aux modifications de Claude et voulez une itération plus rapide, par exemple lors du prototypage ou lorsque vous travaillez dans un répertoire isolé.

<h4 id="don’t-ask-mode-dontask">
  Mode de non-demande (`dontAsk`)
</h4>

Convertit toute invite de permission en refus. Les outils pré-approuvés par `allowed_tools`, les règles d'autorisation de `settings.json` ou un hook s'exécutent normalement. Les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils qui nécessitent une interaction utilisateur sont refusés même lorsqu'une règle d'autorisation correspond. Tout le reste est refusé sans appeler `canUseTool`.

**À utiliser quand :** vous voulez une surface d'outil fixe et explicite pour un agent sans interface et préférez un refus catégorique à une dépendance silencieuse à l'absence de `canUseTool`.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Mode de contournement des permissions (`bypassPermissions`)
</h4>

Auto-approuve tous les usages d'outils sans invites. Les hooks s'exécutent toujours et peuvent bloquer les opérations si nécessaire.

<Warning>
  À utiliser avec une extrême prudence. Claude a un accès système complet dans ce mode. À utiliser uniquement dans des environnements contrôlés où vous faites confiance à toutes les opérations possibles.

  `allowed_tools` ne contraint pas ce mode. Tous les outils sont approuvés, pas seulement ceux que vous avez listés. Les règles de refus (`disallowed_tools`), les règles `ask` explicites et les hooks sont évalués avant la vérification du mode et peuvent toujours bloquer un outil. Les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils qui nécessitent une interaction utilisateur tombent toujours dans votre callback `canUseTool`.
</Warning>

<h4 id="plan-mode-plan">
  Mode de planification (`plan`)
</h4>

Claude explore la base de code et produit un plan sans modifier vos fichiers source. Les outils en lecture seule s'exécutent comme en mode par défaut. Les modifications de fichiers ne sont jamais auto-approuvées en mode plan, même lorsqu'une règle d'autorisation correspond. Elles demandent via votre callback `canUseTool` à la place. Claude peut utiliser `AskUserQuestion` pour clarifier les exigences avant de finaliser le plan. Consultez [Gérer les approbations et les entrées utilisateur](/docs/fr/agent-sdk/user-input#handle-clarifying-questions) pour gérer ces invites.

**À utiliser quand :** vous voulez que Claude propose des modifications sans les exécuter, par exemple lors d'une révision de code ou lorsque vous devez approuver les modifications avant qu'elles ne soient apportées.

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour les autres étapes du flux d'évaluation des permissions :

* [Gérer les approbations et les entrées utilisateur](/docs/fr/agent-sdk/user-input) : invites d'approbation interactives et questions de clarification
* [Guide des hooks](/docs/fr/agent-sdk/hooks) : exécutez du code personnalisé à des points clés du cycle de vie de l'agent
* [Règles de permission](/docs/fr/settings#permission-settings) : règles d'autorisation/refus déclaratives dans `settings.json`
