> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer les permissions

> Contrôlez comment votre agent utilise les outils avec les modes de permission, les hooks et les règles déclaratives d'autorisation/refus.

Le Claude Agent SDK fournit des contrôles de permission pour gérer la façon dont Claude utilise les outils. Utilisez les modes de permission et les règles pour définir ce qui est autorisé automatiquement, et le callback [`canUseTool`](/docs/fr/agent-sdk/user-input) pour gérer tout le reste à l'exécution.

<h2 id="how-permissions-are-evaluated">
  Comment les permissions sont évaluées
</h2>

Lorsque Claude demande un outil, le SDK vérifie les permissions dans cet ordre :

<Steps>
  <Step title="Hooks">
    Exécutez d'abord les [hooks](/docs/fr/agent-sdk/hooks). Un hook peut refuser l'appel catégoriquement ou le transmettre. Un hook qui retourne `allow` ne contourne pas les règles de refus et de demande ci-dessous ; celles-ci sont évaluées indépendamment du résultat du hook. Un hook `PreToolUse` allow ne peut pas non plus approuver une suppression `rm` ou `rmdir` ciblant un [chemin critique](/docs/fr/permission-modes#critical-paths).
  </Step>

  <Step title="Règles de refus">
    Vérifiez les règles `deny` (à partir de `disallowed_tools` et [settings.json](/docs/fr/settings-reference#permission-settings)). Si une règle de refus correspond, l'outil est bloqué, même en mode `bypassPermissions`. Les règles de refus sans nom comme `Bash` suppriment l'outil du contexte de Claude avant que cette évaluation ne commence, donc seules les règles délimitées comme `Bash(rm *)` sont vérifiées à cette étape.
  </Step>

  <Step title="Règles de demande">
    Vérifiez les règles `ask` à partir de [settings.json](/docs/fr/settings-reference#permission-settings). Si une règle de demande correspond, l'appel passe à votre callback [`canUseTool`](/docs/fr/agent-sdk/user-input) pour confirmation, même en mode `bypassPermissions`.

    Les outils qui nécessitent une interaction utilisateur se comportent de la même manière : `AskUserQuestion` et les outils MCP dont le serveur définit [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool) passent toujours au callback, même lorsqu'une règle allow correspond. En mode `dontAsk`, les deux cas sont refusés à la place, car ce mode ne demande jamais. L'annotation MCP nécessite Claude Code v2.1.199 ou ultérieur.

    Les outils du connecteur [claude.ai](/docs/fr/mcp#organization-controls-on-connector-tools) que votre organisation a définis sur `ask` quittent également le flux à cette étape. Chaque appel passe au callback, même en mode `bypassPermissions` et même lorsqu'une règle allow correspond. Le callback reçoit la raison `Your organization requires approval for this tool`. En mode `dontAsk`, l'appel est refusé à la place, car ce mode ne demande jamais.
  </Step>

  <Step title="Mode de permission">
    Appliquez le [mode de permission](#permission-modes) actif :

    * En mode `bypassPermissions`, Claude Code approuve tout ce qui atteint cette étape sauf les suppressions `rm` et `rmdir` ciblant un [chemin critique](/docs/fr/permission-modes#critical-paths), qui passent à la place.
    * En mode `acceptEdits`, Claude Code approuve les opérations de fichier listées sous [Mode Accept edits](#accept-edits-mode-acceptedits).
    * En mode `plan`, Claude Code envoie les outils de modification de fichier et d'écriture shell à votre callback `canUseTool` indépendamment des règles allow, afin que les opérations d'écriture ne puissent pas être approuvées automatiquement lors de la planification.
    * Dans les autres modes, la demande passe.
  </Step>

  <Step title="Règles d'autorisation">
    Vérifiez les règles `allow` (à partir de `allowed_tools` et settings.json). Si une règle correspond, l'outil est approuvé. Un appel que l'outil approuve de lui-même est résolu à cette étape aussi, sans règle nécessaire : par exemple une lecture de fichier dans vos répertoires de travail ou une [commande Bash en lecture seule](/docs/fr/permissions#read-only-commands). Les suppressions `rm` et `rmdir` ciblant un [chemin critique](/docs/fr/permission-modes#critical-paths) ne sont jamais approuvées par une règle allow : elles atteignent votre callback dans les modes qui demandent, vont au [classificateur](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) en mode `auto` sur Claude Code v2.1.218 ou ultérieur, et sont refusées en mode `dontAsk`.
  </Step>

  <Step title="Callback canUseTool">
    Si aucune des étapes ci-dessus ne l'a résolu, appelez votre callback [`canUseTool`](/docs/fr/agent-sdk/user-input) pour une décision. En mode `dontAsk`, cette étape est ignorée et l'outil est refusé.

    Dans le SDK TypeScript, si vous définissez [`permissionPrompts: 'none'`](/docs/fr/agent-sdk/typescript#options), votre callback n'est pas appelé à cette étape. Un hook [`PermissionRequest`](/docs/fr/hooks#permissionrequest) a toujours une chance de décider, et s'il ne le fait pas, Claude Code refuse l'appel. L'option nécessite Claude Code v2.1.259 ou ultérieur.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="Diagramme du flux d'évaluation des permissions en six étapes correspondant aux étapes ci-dessus : une demande d'outil passe par les hooks, les règles de refus, les règles de demande, le mode de permission, les règles d'autorisation et canUseTool. Les hooks, les règles de refus et canUseTool peuvent router vers Blocked ; le contournement du mode de permission, les règles d'autorisation et canUseTool peuvent router vers Execute ; les règles de demande routent vers canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="Diagramme du flux d'évaluation des permissions en six étapes correspondant aux étapes ci-dessus : une demande d'outil passe par les hooks, les règles de refus, les règles de demande, le mode de permission, les règles d'autorisation et canUseTool. Les hooks, les règles de refus et canUseTool peuvent router vers Blocked ; le contournement du mode de permission, les règles d'autorisation et canUseTool peuvent router vers Execute ; les règles de demande routent vers canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

Si vous transmettez un callback `canUseTool` dans une configuration où le SDK TypeScript s'attend à ce que l'ordre d'évaluation approuve automatiquement les appels avant que le callback ne soit consulté, le SDK émet un avertissement de processus Node.js une fois lorsque la requête est construite. Le code de l'avertissement est `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Deux configurations le déclenchent :

* `permissionMode: 'bypassPermissions'`, qui approuve automatiquement chaque appel qui atteint l'étape du mode de permission à l'exception des [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves)
* Chaque entrée `allowedTools` sans nom comme `"Read"`, qui approuve automatiquement cet outil entier avant que le callback ne soit consulté, à l'exception des [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves)

Les entrées avec un spécificateur comme `Bash(ls *)` et le mode `acceptEdits` ne le déclenchent pas, et les règles d'autorisation provenant de fichiers de paramètres ne sont pas visibles pour la vérification.

Écoutez avec `process.on('warning', ...)` et faites correspondre le code pour le journaliser ou le supprimer. Pour contrôler chaque appel d'outil indépendamment du mode et des règles, utilisez plutôt un [hook `PreToolUse`](/docs/fr/agent-sdk/hooks).

Cette page se concentre sur les **règles d'autorisation et de refus** et les **modes de permission**. Pour les autres étapes :

* **Hooks :** exécutez du code personnalisé pour autoriser, refuser ou modifier les demandes d'outils. Voir [Contrôler l'exécution avec les hooks](/docs/fr/agent-sdk/hooks).
* **Callback canUseTool :** invitez les utilisateurs à approuver au moment de l'exécution, lorsqu'aucune étape antérieure ne résout l'appel. Voir [Gérer les approbations et l'entrée utilisateur](/docs/fr/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Règles d'autorisation et de refus
</h2>

`allowed_tools` et `disallowed_tools` (TypeScript : `allowedTools` / `disallowedTools`) ajoutent des entrées aux listes de règles d'autorisation et de refus dans le flux d'évaluation ci-dessus. Si vous nommez l'un des [outils de suivi des tâches](/docs/fr/agent-sdk/todo-tracking#model-availability) dans `allowed_tools`, Claude Code opte également pour la session. Tout autre outil non listé dans `allowed_tools` reste disponible pour Claude, et un appel à celui-ci qui nécessite une approbation tombe dans le mode de permission. Les règles de refus se comportent différemment selon qu'elles nomment un outil ou délimitent un motif au sein de celui-ci.

| Option                            | Effet                                                                                                                                                                                                                                                                         |
| :-------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` et `Grep` sont approuvés automatiquement. Les autres outils non listés ici existent toujours, et les appels à ceux-ci qui nécessitent une approbation tombent dans le mode de permission et `canUseTool`.                                                              |
| `disallowed_tools=["Bash"]`       | La définition de l'outil `Bash` est supprimée de la requête. Claude ne voit pas l'outil et ne peut pas le tenter.                                                                                                                                                             |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` reste disponible. Les appels correspondant à `rm *` [tel qu'écrit](/docs/fr/permissions#bash-rule-limits) sont refusés dans tous les modes de permission, y compris `bypassPermissions`. Les autres appels `Bash`, y compris `/bin/rm`, tombent dans le mode de permission. |
| `disallowed_tools=["*"]`          | Chaque définition d'outil est supprimée de la requête. Les globs de noms d'outils sont pris en charge dans les règles de refus : `"*"` correspond à chaque outil et `"mcp__*"` correspond à chaque outil MCP sur tous les serveurs.                                           |

Les règles d'autorisation acceptent les globs de noms d'outils uniquement après un préfixe littéral `mcp__<server>__`. Le segment serveur doit être sans glob afin que la règle nomme un serveur spécifique que vous avez configuré : `mcp__puppeteer__*` correspond à chaque outil du serveur `puppeteer`, et `mcp__github__get_*` correspond à ses outils `get_`. Une entrée non ancrée comme `allowed_tools=["*"]` ou `allowed_tools=["mcp__*"]` est ignorée avec un avertissement au démarrage et n'approuve automatiquement rien.

Les règles délimitées pour `Read` et `Edit` prennent un motif de chemin. Les règles `Edit(path)` régissent tous les outils intégrés qui écrivent des fichiers, y compris `Write` et `NotebookEdit` ; une règle `Write(path)` n'est jamais mise en correspondance par les vérifications de permission de fichier.

Utilisez `//path` pour un chemin du système de fichiers absolu : une règle de refus de `Edit(//secrets/**)` bloque les écritures n'importe où sous `/secrets` sur le disque. Avec une seule barre oblique, `Edit(/secrets/**)` s'ancre à la source de la règle à la place. Pour les règles transmises via `allowed_tools` ou `disallowed_tools`, cela signifie le répertoire de travail de la session, donc la règle ne bloque pas `/secrets` sur le disque. Voir [Règles Read et Edit](/docs/fr/permissions#read-and-edit) pour les quatre formes d'ancrage et comment les règles des fichiers de paramètres se résolvent.

<Warning>
  **Les outils approuvés automatiquement n'atteignent jamais `canUseTool`.** Un appel d'outil approuvé à n'importe quelle étape antérieure, par `acceptEdits` ou `bypassPermissions`, ou par une règle d'autorisation, ignore votre rappel `canUseTool`, donc les vérifications de permission que vous y mettez sont silencieusement contournées pour cet outil. `AskUserQuestion`, les outils MCP marqués [`_meta["anthropic/requiresUserInteraction"]`](/docs/fr/mcp#require-approval-for-a-specific-tool), les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les suppressions `rm` et `rmdir` ciblant un [chemin critique](/docs/fr/permission-modes#critical-paths) atteignent toujours le rappel, même lorsqu'une règle d'autorisation correspond. En mode `auto`, les suppressions de chemin critique vont au [classificateur](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) au lieu du rappel, tandis que les autres appels listés ici l'atteignent toujours ; le routage du classificateur nécessite Claude Code v2.1.218 ou ultérieur. En mode `dontAsk`, ces appels sont refusés à la place, sans invoquer le rappel.

  La couverture dépend de la forme de l'entrée : un nom nu comme `Read` ou `mcp__github__get_issue` approuve automatiquement chaque appel à cet outil en dehors des exceptions ci-dessus, tandis qu'une règle délimitée comme `Bash(npm test *)` approuve automatiquement uniquement les appels correspondants, et les autres appels `Bash` qui nécessitent une approbation tombent toujours dans le rappel. Pour les vérifications qui doivent s'exécuter sur chaque appel d'outil, utilisez un [hook `PreToolUse`](/docs/fr/agent-sdk/hooks) : les hooks s'exécutent avant chaque autre étape, et un refus de hook s'applique même en mode `bypassPermissions`.
</Warning>

Pour un agent verrouillé, associez `allowedTools` avec `permissionMode: "dontAsk"` :

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

Les outils listés sont approuvés, en dehors des [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves), et chaque autre appel qui inviterait est refusé à la place. Les appels qui ne nécessitent aucune approbation en mode `default` s'exécutent que vous les listiez ou non, comme les [commandes Bash en lecture seule](/docs/fr/permissions#read-only-commands), les outils comme `Agent` qui ne demandent pas avant d'exécuter, et les lectures de fichiers dans vos répertoires de travail. Pour mettre un outil hors de portée de Claude entièrement, ajoutez son nom nu à `disallowedTools`.

<Warning>
  **`allowed_tools` ne contraint pas `bypassPermissions`.** `allowed_tools` pré-approuve les outils que vous listez. Les autres outils non listés ne sont pas mis en correspondance par aucune règle d'autorisation et tombent dans le mode de permission, où `bypassPermissions` les approuve. Définir `allowed_tools=["Read"]` aux côtés de `permission_mode="bypassPermissions"` approuve toujours chaque outil, y compris `Bash`, `Write`, et `Edit`. Si vous avez besoin de `bypassPermissions` mais que vous voulez que des outils spécifiques soient bloqués, utilisez `disallowed_tools`.
</Warning>

Vous pouvez également configurer les règles d'autorisation, de refus et de demande de manière déclarative dans `.claude/settings.json`. Ces règles sont lues lorsque la source de paramètre `project` est activée, ce qu'elle est pour les options `query()` par défaut. Si vous définissez `setting_sources` (TypeScript : `settingSources`) explicitement, incluez `"project"` pour qu'elles s'appliquent. Voir [Paramètres de permission](/docs/fr/settings-reference#permission-settings) pour la syntaxe des règles.

<h2 id="permission-modes">
  Modes de permission
</h2>

Les modes de permission offrent un contrôle global sur la façon dont Claude utilise les outils. Vous pouvez définir le mode de permission lors de l'appel de `query()` ou le modifier dynamiquement pendant les sessions de streaming.

<h3 id="available-modes">
  Modes disponibles
</h3>

Le SDK prend en charge ces modes de permission :

| Mode                | Description                                            | Comportement des outils                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------ | :----------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportement de permission standard                    | Pas d'approbations automatiques basées sur le mode ; les appels qui nécessitent une approbation et ne correspondent à aucune règle d'autorisation déclenchent votre rappel `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `dontAsk`           | Refuser au lieu de demander                            | Tout appel qui demanderait autrement est refusé. Les appels approuvés par `allowed_tools` ou les règles s'exécutent, tout comme les appels qui ne nécessitent pas d'approbation en mode `default`, tels que les lectures de fichiers dans vos répertoires de travail et les appels à `Agent`. Les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils qui nécessitent une interaction utilisateur sont refusés même si vous les avez pré-approuvés, tout comme les suppressions `rm` et `rmdir` ciblant un [chemin critique](/docs/fr/permission-modes#critical-paths). `canUseTool` n'est jamais appelé |
| `acceptEdits`       | Accepter automatiquement les modifications de fichiers | Les modifications de fichiers et les [opérations du système de fichiers](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, etc.) sont automatiquement approuvées                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `bypassPermissions` | Contourner les vérifications de permission             | Les outils s'exécutent sans invites de permission, sauf pour les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves). À utiliser avec prudence                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `plan`              | Mode de planification                                  | Claude explore et planifie sans modifier vos fichiers source ; les modifications de fichiers ne sont jamais approuvées automatiquement et demandent via votre rappel `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `auto`              | Approbations classées par le modèle                    | Un classificateur de modèle approuve ou refuse les invites de permission. Voir [Mode Auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) pour la disponibilité                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

<Warning>
  **Héritage des sous-agents :** Un sous-agent s'exécute dans le mode de permission de la session parent sauf si vous définissez `permissionMode` sur son [`AgentDefinition`](/docs/fr/agent-sdk/typescript#agentdefinition) et que la session parent est en mode `default`, `dontAsk` ou `plan`. Même dans ce cas, Claude Code n'applique jamais une valeur `"bypassPermissions"`. Un sous-agent s'exécute en mode `bypassPermissions` uniquement lorsque la session parent elle-même le fait. L'exception `bypassPermissions` nécessite Claude Code v2.1.267 ou ultérieur.

  Les sous-agents peuvent avoir des invites système différentes et un comportement moins contraint que votre agent principal, donc hériter de `bypassPermissions` leur accorde un accès système complet et autonome. Les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves) s'appliquent toujours.
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

Approuve automatiquement les opérations de fichiers afin que Claude puisse modifier le code sans demander. Les autres outils (comme les commandes Bash qui ne sont pas des opérations du système de fichiers) nécessitent toujours des permissions normales.

**Opérations approuvées automatiquement :**

* Modifications de fichiers (outils Edit, Write)
* Commandes du système de fichiers : `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Les deux s'appliquent uniquement aux chemins à l'intérieur du répertoire de travail ou de `additionalDirectories`. En mode `acceptEdits`, Claude Code n'approuve pas automatiquement la demande lorsque Claude :

* Travaille sur un chemin en dehors de cette portée
* Écrit dans un chemin protégé
* Supprime un [chemin critique](/docs/fr/permission-modes#critical-paths) avec `rm` ou `rmdir`

**À utiliser quand :** vous faites confiance aux modifications de Claude et souhaitez une itération plus rapide, par exemple lors du prototypage ou lorsque vous travaillez dans un répertoire isolé.

<h4 id="don’t-ask-mode-dontask">
  Mode ne pas demander (`dontAsk`)
</h4>

Convertit toute invite de permission en refus, sans appeler `canUseTool`. Les outils pré-approuvés par `allowed_tools`, les règles d'autorisation `settings.json` ou un hook s'exécutent normalement, tout comme les appels qui ne nécessitent pas d'approbation en mode `default`, tels que les lectures de fichiers dans vos répertoires de travail et les appels à `Agent`. Les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), les outils qui nécessitent une interaction utilisateur, et les suppressions `rm` et `rmdir` ciblant un [chemin critique](/docs/fr/permission-modes#critical-paths) sont refusés même lorsqu'une règle d'autorisation correspond. Un refus de hook PreToolUse ne supprime pas non plus une suppression de chemin critique.

**À utiliser quand :** vous souhaitez une surface d'outils fixe et explicite pour un agent sans interface et préférez un refus catégorique à une dépendance silencieuse à l'absence de `canUseTool`.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Mode contournement des permissions (`bypassPermissions`)
</h4>

Approuve automatiquement les utilisations d'outils sans demander, sauf les cas énumérés dans l'avertissement ci-dessous. Les hooks s'exécutent toujours et peuvent bloquer les opérations si nécessaire. Sur Linux et macOS, Claude Code refuse de démarrer dans ce mode en tant que root ou sous `sudo` en dehors d'un [bac à sable reconnu](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode), et la requête échoue avant le premier tour.

<Warning>
  À utiliser avec une extrême prudence. Claude a un accès système complet dans ce mode. À utiliser uniquement dans des environnements contrôlés où vous faites confiance à toutes les opérations possibles.

  `allowed_tools` ne contraint pas ce mode. Chaque outil est approuvé, pas seulement ceux que vous avez énumérés. Ces contrôles s'appliquent toujours :

  * Les règles de refus, les règles explicites `ask` et les hooks sont évalués avant la vérification du mode et peuvent toujours bloquer un outil.
  * Les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), les outils qui nécessitent une interaction utilisateur, et les suppressions `rm` et `rmdir` ciblant un [chemin critique](/docs/fr/permission-modes#critical-paths) reviennent toujours à votre rappel `canUseTool`.
  * Les [protections de messagerie inter-sessions](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode) s'appliquent toujours.
</Warning>

<h4 id="plan-mode-plan">
  Mode de planification (`plan`)
</h4>

Claude explore la base de code et produit un plan sans modifier vos fichiers source. Les outils en lecture seule s'exécutent comme ils le font en mode de permission `default`.

Les modifications de fichiers ne sont jamais approuvées automatiquement en mode plan, même lorsqu'une règle d'autorisation correspond. Elles demandent via votre rappel `canUseTool` à la place. Sur Claude Code v2.1.212 ou ultérieur, les commandes shell qui modifient les fichiers, telles que `touch` et `rm`, atteignent votre rappel `canUseTool` de la même manière.

Si vous définissez `allowDangerouslySkipPermissions: true` aux côtés de `permissionMode: 'plan'`, les modifications de fichiers et les commandes shell qui modifient les fichiers atteignent toujours votre rappel `canUseTool`. L'option vous permet de passer à `bypassPermissions` plus tard avec `setPermissionMode()`.

Claude peut utiliser `AskUserQuestion` pour clarifier les exigences avant de finaliser le plan. Voir [Gérer les approbations et les entrées utilisateur](/docs/fr/agent-sdk/user-input#handle-clarifying-questions) pour gérer ces invites.

**À utiliser quand :** vous souhaitez que Claude propose des modifications sans les exécuter, par exemple lors d'une révision de code ou lorsque vous devez approuver les modifications avant qu'elles ne soient apportées.

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour les autres étapes du flux d'évaluation des permissions :

* [Gérer les approbations et les entrées utilisateur](/docs/fr/agent-sdk/user-input) : invites d'approbation interactives et questions de clarification
* [Guide des hooks](/docs/fr/agent-sdk/hooks) : exécuter du code personnalisé aux points clés du cycle de vie de l'agent
* [Règles de permission](/docs/fr/settings-reference#permission-settings) : règles déclaratives d'autorisation/refus dans `settings.json`
