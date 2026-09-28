> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer votre agent

> Configurez les sessions du SDK Agent : composez l'objet options, définissez le modèle, l'environnement et les limites, et trouvez la page de chaque option de fonctionnalité.

Une session du SDK Agent lit la configuration à partir des fichiers de paramètres, des variables d'environnement et de l'objet `options` que vous transmettez au démarrage. Cette page montre comment composer l'objet `options` et quels fichiers de paramètres et variables d'environnement le contrôlent.

Pour chaque type d'option et sa valeur par défaut, consultez les références [`Options`](/docs/fr/agent-sdk/typescript#options) (TypeScript) et [`ClaudeAgentOptions`](/docs/fr/agent-sdk/python#claudeagentoptions) (Python).

<h2 id="pass-options-to-a-session">
  Transmettre les options à une session
</h2>

Chaque appel `query()` accepte un objet options : `Options` en TypeScript, `ClaudeAgentOptions` en Python. Chaque champ est facultatif, et une session démarrée sans options s'exécute avec les valeurs par défaut du SDK. L'exemple ci-dessous configure une session en lecture seule qui résume les TODOs ouverts d'un projet. Les paires se lisent comme TypeScript / Python où les orthographes diffèrent :

* **`model`** : choisit le modèle
* **`allowedTools` / `allowed_tools`** : pré-approuve une liste d'outils en lecture seule
* **`maxTurns` / `max_turns`** : limite le nombre de tours
* **`cwd`** : définit le répertoire de travail

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

Pointez `cwd` vers l'un de vos propres projets et exécutez l'exemple. Le résumé des TODOs ouverts de ce projet s'affiche à l'arrivée du message de résultat.

`allowedTools` (TypeScript) ou `allowed_tools` (Python) pré-approuve les outils listés, de sorte que les appels à ces outils s'exécutent sans attendre d'approbation. Les outils en dehors de la liste restent disponibles. Lorsque Claude appelle un outil non listé, le mode de permission décide si l'appel s'exécute. Pour plus d'informations, consultez [Règles d'autorisation et de refus](/docs/fr/agent-sdk/permissions#allow-and-deny-rules).

<h2 id="load-settings-files">
  Charger les fichiers de paramètres
</h2>

Les fichiers de paramètres fournissent une configuration au-delà de l'objet options. Deux options contrôlent la façon dont ils se chargent :

* **`settingSources` / `setting_sources`** : contrôle quelles sources du système de fichiers se chargent : utilisateur, projet et local. Les fichiers de paramètres et les fichiers CLAUDE.md arrivent par ces sources.
* **`settings`** : charge un chemin de fichier de paramètres ou une chaîne JSON en ligne dans l'une ou l'autre langue, et TypeScript accepte également un objet de paramètres. Quelle que soit la forme que vous transmettez, elle remplace les paramètres du système de fichiers utilisateur, projet et local ; seuls les paramètres de politique gérée ont un rang plus élevé. Les références documentent l'ordre de précédence complet sous [Précédence des paramètres](/docs/fr/agent-sdk/typescript#settings-precedence) pour TypeScript et [Précédence des paramètres](/docs/fr/agent-sdk/python#settings-precedence) pour Python.

Passez `[]` pour désactiver les paramètres utilisateur, projet et local. Pour plus d'informations, consultez [Utiliser les fonctionnalités de Claude Code dans le SDK](/docs/fr/agent-sdk/claude-code-features).

<h2 id="choose-a-model">
  Choisir un modèle
</h2>

À moins que l'option `model`, vos paramètres ou votre environnement ne sélectionnent un modèle, une nouvelle session démarre sur [le modèle par défaut de Claude Code](/docs/fr/model-config#default-model-setting). Pour l'ordre de ces sources, consultez [Définir votre modèle](/docs/fr/model-config#setting-your-model). Définissez `model` pour épingler un modèle spécifique, ou pour en choisir un plus petit pour des agents plus rapides et moins chers. La valeur prend un alias de modèle ou un nom de modèle complet ; les alias et les versions qu'ils résolvent sont listés sous [Alias de modèles](/docs/fr/model-config#model-aliases).

Définissez `fallbackModel` (TypeScript) ou `fallback_model` (Python) pour nommer un modèle de secours. Lorsque le modèle principal est surchargé ou indisponible, la session bascule vers le modèle de secours. Le modèle principal est réessayé au début de chaque tour utilisateur, de sorte que la session y revient une fois la panne résolue.

Dans l'une ou l'autre langue, l'option accepte un seul modèle ou une liste de secours séparée par des virgules. Pour l'ordre et la limite de chaîne, consultez [Chaînes de modèles de secours](/docs/fr/model-config#fallback-model-chains). En TypeScript, un modèle de secours égal à `model` lève une erreur au démarrage.

Les exemples ci-dessous montrent une liste de secours en TypeScript et un seul modèle de secours en Python :

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  Les paramètres de requête de l'[API Messages](https://platform.claude.com/docs/en/api/messages) `temperature`, `top_p` et `max_tokens` n'ont pas de champs sur l'objet options dans l'une ou l'autre langue. Définissez plutôt le [niveau d'effort](/docs/fr/agent-sdk/agent-loop#effort-level) ou un [plafond de dépenses](#limit-turns-and-spend), ou appelez l'API Messages lorsque vous avez besoin de ces paramètres directement.
</Note>

<h2 id="set-environment-variables">
  Définir les variables d'environnement
</h2>

L'option `env` définit les variables d'environnement pour le processus Claude Code qui exécute votre session. Le fait que vos valeurs remplacent l'environnement hérité ou le fusionnent diffère selon la langue :

* **TypeScript** : `env` remplace l'environnement du sous-processus
* **Python** : le SDK fusionne vos valeurs sur l'environnement hérité, et vos valeurs remplacent les valeurs héritées

En TypeScript, propagez `process.env` dans `env` pour conserver les variables héritées telles que `PATH`, `HOME` et `ANTHROPIC_API_KEY`. Lorsque vous laissez `env` non défini, le sous-processus hérite de votre environnement dans les deux langues.

L'exemple achemine le trafic API via une passerelle en définissant `ANTHROPIC_BASE_URL`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

Les variables que vous transmettez peuvent également configurer Claude Code lui-même. Pour les variables que le processus Claude Code lit, consultez [Variables d'environnement](/docs/fr/env-vars). Pour régler les délais d'expiration de l'API et la détection de blocage de cette façon, suivez la section Gérer les réponses API lentes ou bloquées dans la [référence TypeScript](/docs/fr/agent-sdk/typescript#handle-slow-or-stalled-api-responses) ou la [référence Python](/docs/fr/agent-sdk/python#handle-slow-or-stalled-api-responses).

<h2 id="set-the-working-directory">
  Définir le répertoire de travail
</h2>

Définissez `cwd` pour exécuter la session dans un répertoire spécifique. Lorsque vous laissez `cwd` non défini, la session s'exécute dans le répertoire de travail de votre processus. Aucun SDK n'a de setter pour `cwd`. Pour exécuter dans un répertoire différent, démarrez une autre session avec ce `cwd`.

Claude Code lit le répertoire de travail pour déterminer :

* **Paramètres et hooks du projet** : quels [paramètres et hooks du projet se chargent](/docs/fr/agent-sdk/claude-code-features)
* **Compétences** : où [les compétences de session sont découvertes](/docs/fr/agent-sdk/skills)
* **Stockage de session** : à quel projet [une session stockée appartient](/docs/fr/agent-sdk/session-storage)

Pour permettre aux outils d'accéder aux fichiers en dehors du répertoire de travail, ajoutez des chemins avec `additionalDirectories` (TypeScript) ou `add_dirs` (Python). Pour la portée de cette autorisation, consultez [Les répertoires supplémentaires accordent l'accès aux fichiers, pas la configuration](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration).

<h2 id="limit-turns-and-spend">
  Limiter les tours et les dépenses
</h2>

Limitez les tours et les dépenses avec `maxTurns` / `max_turns` et `maxBudgetUsd` / `max_budget_usd`. Les deux limites sont désactivées lorsqu'elles ne sont pas définies. Lorsqu'une session atteint une limite, l'exécution se termine par un message de résultat dont le sous-type nomme la limite, `error_max_turns` ou `error_max_budget_usd`. Ce qui se passe ensuite diffère selon le mode d'entrée :

* **`query()` en un seul coup** : le SDK produit le résultat de la limite, puis lève une exception, donc enveloppez la boucle dans un bloc try pour continuer au-delà de l'erreur
* **Entrée en streaming** : la session reste active au-delà d'un résultat de limite, et le nombre de tours maximum recommence pour chaque message en file d'attente. Le total du budget s'accumule sur les messages, et une fois que les dépenses atteignent la limite, les messages ultérieurs dans la même conversation se terminent par le même résultat de budget. Un [`/clear`](/docs/fr/agent-sdk/cost-tracking) recommence le budget

Les deux limites traitent `0` différemment :

* **`maxTurns` / `max_turns`** : `0` exécute la session sans limite de tours, comme laisser l'option non définie
* **`maxBudgetUsd` / `max_budget_usd`** : l'interface de ligne de commande rejette `0` comme un montant invalide au démarrage, et la session ne s'exécute jamais

Pour plus d'informations sur les deux limites, y compris les dépenses des sous-agents, consultez [Tours et budget](/docs/fr/agent-sdk/agent-loop#turns-and-budget).

<h2 id="change-configuration-mid-session">
  Modifier la configuration en cours de session
</h2>

Lorsque vous démarrez une session avec [entrée en streaming](/docs/fr/agent-sdk/streaming-vs-single-mode), vous pouvez basculer son modèle et son mode de permission pendant qu'elle s'exécute. L'endroit où vous appelez les setters diffère selon la langue :

* **TypeScript** : méthodes sur l'objet que `query()` retourne
* **Python** : méthodes sur [`ClaudeSDKClient`](/docs/fr/agent-sdk/python#claudesdkclient), puisque `query()` retourne un itérateur simple sans méthodes de contrôle

Les deux langues ont les mêmes setters :

* **`setModel()` / `set_model()`** : bascule le modèle. Appelez-le sans modèle pour basculer vers [le modèle par défaut de Claude Code](/docs/fr/model-config#default-model-setting) plutôt que le `model` que vous avez transmis dans les options.
* **`setPermissionMode()` / `set_permission_mode()`** : bascule le mode de permission

TypeScript a également `applyFlagSettings()` et `updateSettings()` :

* **`applyFlagSettings()`** : applique les paramètres à l'exécution, comme dans `await session.applyFlagSettings({ effortLevel: "high" })`. La méthode prend les clés du fichier de paramètres plutôt que les champs d'options, donc consultez la [référence `applyFlagSettings()`](/docs/fr/agent-sdk/typescript#applyflagsettings) pour le schéma et pour savoir quelles clés prennent effet en cours de session.
* **`updateSettings()`** : écrit une clé autorisée dans un fichier de paramètres. La [référence `updateSettings()`](/docs/fr/agent-sdk/typescript#updatesettings) nomme la clé que chaque source accepte et le plancher de version.
  * Passez `"localSettings"` pour écrire le fichier de paramètres locaux du projet, comme dans `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`. La clé écrite prend effet à la prochaine requête de la session et persiste pour les sessions ultérieures qui chargent les paramètres `local`.
  * Passez `"userSettings"` pour écrire `effortLevel`, la seule clé que cette source accepte. Claude Code l'enregistre comme le niveau d'effort par défaut pour le modèle actuel de la session, et l'effort de la session en cours ne change pas.

L'exemple ci-dessous exécute une session de deux tours, modifie la configuration entre les tours et affiche le modèle qui a répondu à chaque tour. En TypeScript, le flux de prompt maintient le deuxième message jusqu'à ce que les setters aient été exécutés, et le deuxième tour s'exécute sur le nouveau modèle.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

Sur l'API Claude, le programme affiche `First turn model: claude-sonnet-5`, puis `Second turn model: claude-opus-5` après le basculement.

<Note>
  Chaque modèle a son propre cache de prompt, donc après un basculement en cours de session, la prochaine requête recalcule la conversation complète sans cache aux tarifs du nouveau modèle. Pour plus d'informations, consultez [Basculer les modèles](/docs/fr/prompt-caching#switching-models).
</Note>

<h2 id="configure-specific-features">
  Configurer des fonctionnalités spécifiques
</h2>

Le tableau ci-dessous mappe chaque option à la fonctionnalité qu'elle configure. Pour les options que cette page ne couvre pas, consultez les références [TypeScript](/docs/fr/agent-sdk/typescript#options) et [Python](/docs/fr/agent-sdk/python#claudeagentoptions). Si vous connaissez votre objectif mais pas quelle option le sert, commencez par [Choisir la bonne fonctionnalité](/docs/fr/agent-sdk/claude-code-features#choose-the-right-feature).

| TypeScript                | Python                      | Contrôle                                            | Couvert dans                                                                                                                                                                                                               |
| ------------------------- | --------------------------- | --------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionMode`          | `permission_mode`           | Ce que l'agent peut faire sans approbation          | [Configurer les permissions](/docs/fr/agent-sdk/permissions)                                                                                                                                                                    |
| `allowedTools`            | `allowed_tools`             | Quels appels d'outils sont pré-approuvés            | [Configurer les permissions](/docs/fr/agent-sdk/permissions)                                                                                                                                                                    |
| `canUseTool`              | `can_use_tool`              | Votre rappel d'approbation pour les appels d'outils | [Gérer les demandes d'approbation d'outils](/docs/fr/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                                        |
| `systemPrompt`            | `system_prompt`             | Les instructions de l'agent                         | [Modification des invites système](/docs/fr/agent-sdk/modifying-system-prompts)                                                                                                                                                 |
| `settingSources`          | `setting_sources`           | Quels paramètres du système de fichiers se chargent | [Utiliser les fonctionnalités de Claude Code dans le SDK](/docs/fr/agent-sdk/claude-code-features)                                                                                                                              |
| `mcpServers`              | `mcp_servers`               | Serveurs d'outils externes                          | [Connecter à des outils externes avec MCP](/docs/fr/agent-sdk/mcp)                                                                                                                                                              |
| `agents`                  | `agents`                    | Définitions des sous-agents                         | [Sous-agents](/docs/fr/agent-sdk/subagents)                                                                                                                                                                                     |
| `hooks`                   | `hooks`                     | Rappels aux points du cycle de vie                  | [Hooks](/docs/fr/agent-sdk/hooks)                                                                                                                                                                                               |
| `skills`                  | `skills`                    | Quelles compétences se chargent                     | [Étendre les agents avec des compétences](/docs/fr/agent-sdk/skills)                                                                                                                                                            |
| `plugins`                 | `plugins`                   | Quels plugins se chargent                           | [Plugins](/docs/fr/agent-sdk/plugins)                                                                                                                                                                                           |
| `outputFormat`            | `output_format`             | Schémas de sortie structurée                        | [Sorties structurées](/docs/fr/agent-sdk/structured-outputs)                                                                                                                                                                    |
| `resume`                  | `resume`                    | Continuation d'une session stockée                  | [Sessions](/docs/fr/agent-sdk/sessions)                                                                                                                                                                                         |
| `forkSession`             | `fork_session`              | Branchement d'une session                           | [Sessions](/docs/fr/agent-sdk/sessions)                                                                                                                                                                                         |
| `sessionStore`            | `session_store`             | Persistance de session externe                      | [Stockage de session](/docs/fr/agent-sdk/session-storage)                                                                                                                                                                       |
| `enableFileCheckpointing` | `enable_file_checkpointing` | Éditions de fichiers rembobinables                  | [Checkpointing de fichiers](/docs/fr/agent-sdk/file-checkpointing)                                                                                                                                                              |
| `effort`                  | `effort`                    | Combien de travail Claude met dans les réponses     | [Niveau d'effort](/docs/fr/agent-sdk/agent-loop#effort-level)                                                                                                                                                                   |
| `sandbox`                 | `sandbox`                   | Comportement du sandbox pour l'exécution des outils | [Références TypeScript](/docs/fr/agent-sdk/typescript#sandbox-configuration) et [Python](/docs/fr/agent-sdk/python#sandbox-configuration), avec contexte de déploiement dans [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment) |

<h2 id="next-steps">
  Étapes suivantes
</h2>

Pour voir la configuration composée dans des agents fonctionnels :

* **[Démarrage rapide](/docs/fr/agent-sdk/quickstart)** : construisez et exécutez un premier agent de bout en bout
* **[Exemples](/docs/fr/agent-sdk/examples)** : trouvez un projet complet et exécutable ou une recette guidée Claude Cookbook qui correspond à ce que vous voulez construire
* **[Isolation multi-locataire](/docs/fr/agent-sdk/hosting#multi-tenant-isolation)** : isolez les paramètres et la mémoire de chaque locataire avec `settingSources` / `setting_sources`, `env` et `cwd`
