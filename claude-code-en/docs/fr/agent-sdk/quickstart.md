> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Démarrage rapide

> Commencez avec le SDK Agent Python ou TypeScript pour créer des agents IA qui fonctionnent de manière autonome

Utilisez le SDK Agent pour créer un agent IA qui lit votre code, trouve les bugs et les corrige, tout sans intervention manuelle.

**Ce que vous allez faire :**

1. Configurer un projet avec le SDK Agent
2. Créer un fichier avec du code contenant des bugs
3. Exécuter un agent qui trouve et corrige les bugs automatiquement

<h2 id="prerequisites">
  Prérequis
</h2>

* **Node.js 18+** ou **Python 3.10+**
* Un **compte Anthropic** ([s'inscrire ici](https://platform.claude.com/))

<h2 id="setup">
  Configuration
</h2>

<Steps>
  <Step title="Créer un dossier de projet">
    Créez un nouveau répertoire pour ce démarrage rapide :

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    Pour vos propres projets, vous pouvez exécuter le SDK à partir de n'importe quel dossier ; il aura accès aux fichiers de ce répertoire et de ses sous-répertoires par défaut.
  </Step>

  <Step title="Installer le SDK">
    Installez le package du SDK Agent pour votre langage :

    <Tabs>
      <Tab title="TypeScript (nouveau projet)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        Définir `"type": "module"` dans `package.json` permet à votre script d'agent d'utiliser `await` au niveau supérieur, et [tsx](https://tsx.is) exécute les fichiers TypeScript directement.
      </Tab>

      <Tab title="TypeScript (projet existant)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is) exécute les fichiers TypeScript directement. Si votre projet utilise CommonJS, nommez votre script d'agent `agent.mts` au lieu de `agent.ts`. L'extension `.mts` fait que tsx traite le fichier comme un module ES, donc `await` au niveau supérieur fonctionne sans convertir tout votre projet en modules ES. Utilisez `agent.mts` à la place de `agent.ts` dans les étapes de création et d'exécution plus tard dans ce démarrage rapide.
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) est un gestionnaire de paquets Python rapide qui gère automatiquement les environnements virtuels :

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Créez et activez un environnement virtuel, puis installez le package.

        Sur macOS ou Linux :

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Sur Windows :

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Si PowerShell bloque `Activate.ps1` avec une erreur de politique d'exécution, exécutez d'abord `Set-ExecutionPolicy -Scope Process RemoteSigned`.
      </Tab>
    </Tabs>

    <Note>
      Le SDK TypeScript regroupe un binaire Claude Code natif pour votre plateforme en tant que dépendance optionnelle, vous n'avez donc pas besoin d'installer Claude Code séparément.
    </Note>
  </Step>

  <Step title="Définir votre clé API">
    Obtenez une clé API à partir de la [Console Claude](https://platform.claude.com/), puis définissez-la comme variable d'environnement dans le shell où vous exécuterez votre agent :

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    Le SDK lit la clé à partir de l'environnement du processus qui exécute votre agent ; il ne charge pas les fichiers `.env` automatiquement. Si vous conservez la clé dans un fichier `.env`, chargez-la vous-même, par exemple avec le package `dotenv`, avant d'appeler le SDK.

    Le SDK prend également en charge l'authentification via des fournisseurs d'API tiers :

    * **Amazon Bedrock** : définissez la variable d'environnement `CLAUDE_CODE_USE_BEDROCK=1` et configurez les identifiants AWS
    * **Claude Platform on AWS** : définissez `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` et `ANTHROPIC_AWS_WORKSPACE_ID`, puis configurez les identifiants AWS
    * **Google Cloud's Agent Platform** : définissez la variable d'environnement `CLAUDE_CODE_USE_VERTEX=1` et configurez les identifiants Google Cloud
    * **Microsoft Azure** : définissez la variable d'environnement `CLAUDE_CODE_USE_FOUNDRY=1` et configurez les identifiants Azure

    Consultez les guides de configuration pour [Amazon Bedrock](/docs/fr/amazon-bedrock), [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou [Microsoft Foundry](/docs/fr/microsoft-foundry) pour plus de détails.

    <Note>
      Sauf approbation préalable, Anthropic n'autorise pas les développeurs tiers à proposer la connexion claude.ai ou les limites de débit pour leurs produits, y compris les agents construits sur le SDK Agent Claude. Veuillez utiliser les méthodes d'authentification par clé API décrites dans ce document à la place.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  Créer un fichier avec des bugs
</h2>

Ce démarrage rapide vous guide dans la création d'un agent capable de trouver et corriger les bugs dans le code. D'abord, vous avez besoin d'un fichier avec quelques bugs intentionnels pour que l'agent les corrige. Créez `utils.py` dans le répertoire `my-agent` et collez le code suivant :

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

Ce code a deux bugs :

1. `calculate_average([])` plante avec une division par zéro
2. `get_user_name(None)` plante avec une TypeError

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  Créer un agent qui trouve et corrige les bugs
</h2>

Créez `agent.py` si vous utilisez le SDK Python, ou `agent.ts` pour TypeScript. Utilisez `agent.mts` à la place si votre projet existant utilise CommonJS :

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

Ce code a trois parties principales :

1. **`query`** : le point d'entrée principal qui crée la boucle agentique. Il retourne un itérateur asynchrone, vous utilisez donc `async for` pour diffuser les messages au fur et à mesure que Claude travaille. Consultez l'API complète dans la référence du SDK [Python](/docs/fr/agent-sdk/python#query) ou [TypeScript](/docs/fr/agent-sdk/typescript#query).

2. **`prompt`** : ce que vous voulez que Claude fasse. Claude détermine les outils à utiliser en fonction de la tâche.

3. **`options`** : configuration de l'agent. Cet exemple utilise `allowedTools` pour pré-approuver `Read`, `Edit` et `Glob`, et `permissionMode: "acceptEdits"` pour approuver automatiquement les modifications de fichiers. Les autres options incluent `systemPrompt`, `mcpServers` et bien d'autres. Consultez toutes les options pour [Python](/docs/fr/agent-sdk/python#claudeagentoptions) ou [TypeScript](/docs/fr/agent-sdk/typescript#options).

La boucle `async for` continue de s'exécuter tandis que Claude réfléchit, appelle des outils, observe les résultats et décide de la prochaine étape. Chaque itération produit un message : le raisonnement de Claude, un appel d'outil, un résultat d'outil ou le résultat final. Le SDK gère l'orchestration (exécution des outils, gestion du contexte, tentatives) afin que vous consommiez simplement le flux. La boucle se termine lorsque Claude termine la tâche ou rencontre une erreur.

La gestion des messages à l'intérieur de la boucle filtre la sortie lisible par l'homme. Sans filtrage, vous verriez des objets de message bruts incluant l'initialisation du système et l'état interne, ce qui est utile pour le débogage mais bruyant autrement.

<Note>
  Cet exemple utilise la diffusion en continu pour afficher la progression en temps réel. Si vous n'avez pas besoin de sortie en direct (par exemple, pour les tâches en arrière-plan ou les pipelines CI), vous pouvez collecter tous les messages à la fois. Consultez [Mode diffusion en continu vs. mode à tour unique](/docs/fr/agent-sdk/streaming-vs-single-mode) pour plus de détails.
</Note>

<h3 id="run-your-agent">
  Exécuter votre agent
</h3>

Votre agent est prêt. Exécutez-le avec la commande suivante :

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    Si vous avez nommé votre script `agent.mts`, exécutez `npx tsx agent.mts` à la place.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    Avec votre environnement virtuel toujours activé :

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

Au fur et à mesure qu'il travaille, l'agent imprime son raisonnement et chaque outil qu'il appelle, se terminant par `Done: success`. Après l'exécution, vérifiez `utils.py`. Vous verrez du code défensif gérant les listes vides et les utilisateurs nuls. Votre agent a autonomement :

1. **Lu** `utils.py` pour comprendre le code
2. **Analysé** la logique et identifié les cas limites qui causeraient un plantage
3. **Modifié** le fichier pour ajouter une gestion d'erreur appropriée

C'est ce qui rend le SDK Agent différent : Claude exécute les outils directement au lieu de vous demander de les implémenter.

<Note>
  Si vous voyez ' API key not found ', assurez-vous d'avoir défini la variable d'environnement `ANTHROPIC_API_KEY` dans le shell où vous exécutez votre agent. Le SDK ne charge pas les fichiers `.env` automatiquement. Consultez le [guide de dépannage complet](/docs/fr/troubleshooting) pour plus d'aide.
</Note>

<h3 id="try-other-prompts">
  Essayer d'autres invites
</h3>

Maintenant que votre agent est configuré, essayez quelques invites différentes :

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  Personnaliser votre agent
</h3>

Vous pouvez modifier le comportement de votre agent en changeant les options. Voici quelques exemples :

**Ajouter la capacité de recherche web :**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Donner à Claude une invite système personnalisée :**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**Exécuter des commandes dans le terminal :**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

Avec `Bash` activé, essayez : `"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  Concepts clés
</h2>

**Les outils** contrôlent ce que votre agent peut faire :

| Outils                                 | Ce que l'agent peut faire    |
| -------------------------------------- | ---------------------------- |
| `Read`, `Glob`, `Grep`                 | Analyse en lecture seule     |
| `Read`, `Edit`, `Glob`                 | Analyser et modifier le code |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Automatisation complète      |

**Les modes de permission** contrôlent le niveau de surveillance humaine que vous souhaitez :

| Mode                | Comportement                                                                                                                                                                                                                                                                                                                               | Cas d'usage                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| `acceptEdits`       | Approuve automatiquement les modifications de fichiers et les commandes courantes du système de fichiers, demande les autres actions                                                                                                                                                                                                       | Flux de travail de développement de confiance              |
| `plan`              | Exécute les outils en lecture seule ; les modifications de fichiers ne sont jamais approuvées automatiquement et atteignent votre rappel `canUseTool`                                                                                                                                                                                      | Délimitation d'une tâche avant d'approuver l'exécution     |
| `dontAsk`           | Refuse tout ce qui n'est pas dans `allowedTools` ; les outils de connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils qui nécessitent une interaction utilisateur sont refusés même si vous les avez listés                                                              | Agents sans tête verrouillés                               |
| `auto`              | Un classificateur de modèle approuve ou refuse chaque appel d'outil                                                                                                                                                                                                                                                                        | Agents autonomes avec garde-fous de sécurité               |
| `bypassPermissions` | Exécute chaque outil sans invites, sauf les outils correspondant à une règle [`ask`](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) explicite, les outils de connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils qui nécessitent une interaction utilisateur | CI en bac à sable, environnements entièrement de confiance |
| `default`           | Nécessite un rappel `canUseTool` pour gérer l'approbation                                                                                                                                                                                                                                                                                  | Flux d'approbation personnalisés                           |

L'exemple ci-dessus utilise le mode `acceptEdits`, qui approuve automatiquement les opérations de fichiers afin que l'agent puisse s'exécuter sans invites interactives. Si vous souhaitez inviter les utilisateurs à approuver, utilisez le mode `default` et fournissez un rappel [`canUseTool`](/docs/fr/agent-sdk/user-input) qui collecte l'entrée de l'utilisateur. Pour plus de contrôle, consultez [Permissions](/docs/fr/agent-sdk/permissions).

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous avez créé votre premier agent, apprenez à étendre ses capacités et à l'adapter à votre cas d'usage :

* **[Permissions](/docs/fr/agent-sdk/permissions)** : contrôlez ce que votre agent peut faire et quand il a besoin d'approbation
* **[Hooks](/docs/fr/agent-sdk/hooks)** : exécutez du code personnalisé avant ou après les appels d'outils
* **[Sessions](/docs/fr/agent-sdk/sessions)** : créez des agents multi-tours qui maintiennent le contexte
* **[Serveurs MCP](/docs/fr/agent-sdk/mcp)** : connectez-vous à des bases de données, des navigateurs, des API et d'autres systèmes externes
* **[Hébergement](/docs/fr/agent-sdk/hosting)** : déployez des agents sur Docker, le cloud et CI/CD
* **[Agents d'exemple](https://github.com/anthropics/claude-agent-sdk-demos)** : consultez des exemples complets : assistant e-mail, agent de recherche et bien d'autres
