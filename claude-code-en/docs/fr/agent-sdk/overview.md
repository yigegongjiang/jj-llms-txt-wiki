> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Présentation du SDK Agent

> Créez des agents IA de production avec Claude Code en tant que bibliothèque

Créez des agents IA qui lisent autonomement les fichiers, exécutent des commandes, recherchent sur le web, modifient le code, et bien plus. Le SDK Agent vous offre les mêmes outils, boucle d'agent et gestion du contexte qui alimentent Claude Code, programmables en Python et TypeScript. Pour en savoir plus sur la réflexion derrière la conception du harnais d'agent, consultez [Un harnais pour chaque tâche : flux de travail dynamiques dans Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) sur le blog.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

Le SDK Agent inclut des outils intégrés pour lire les fichiers, exécuter des commandes et modifier le code, afin que votre agent puisse commencer à travailler immédiatement sans que vous ayez besoin d'implémenter l'exécution des outils. Plongez dans le guide de démarrage rapide ou explorez des agents réels construits avec le SDK :

<CardGroup cols={2}>
  <Card title="Guide de démarrage rapide" icon="play" href="/docs/fr/agent-sdk/quickstart">
    Créez un agent de correction de bugs en quelques minutes
  </Card>

  <Card title="Agents d'exemple" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistant e-mail, agent de recherche, et bien plus
  </Card>
</CardGroup>

<h2 id="get-started">
  Commencer
</h2>

<Steps>
  <Step title="Installer le SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) est un gestionnaire de paquets Python rapide qui gère automatiquement les environnements virtuels :

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Créez et activez un environnement virtuel, puis installez le paquet. L'installation dans un environnement virtuel évite l'erreur `error: externally-managed-environment` que Python système sur les installations récentes de Debian, Ubuntu et Homebrew retourne pour `pip install` en dehors d'un venv.

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

        Le package Python nécessite Python 3.10 ou une version ultérieure. Si pip signale `No matching distribution found for claude-agent-sdk`, votre interpréteur est plus ancien que 3.10. Exécutez `python3 --version` sur macOS ou Linux, ou `py --version` sur Windows, pour vérifier.
      </Tab>
    </Tabs>

    <Note>
      Le SDK TypeScript regroupe un binaire Claude Code natif pour votre plateforme en tant que dépendance optionnelle, vous n'avez donc pas besoin d'installer Claude Code séparément.
    </Note>
  </Step>

  <Step title="Définir votre clé API">
    Obtenez une clé API à partir de la [Console](https://platform.claude.com/), puis définissez-la comme variable d'environnement.

    Sur macOS ou Linux :

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    Sur Windows PowerShell :

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    Le SDK prend également en charge l'authentification via des fournisseurs d'API tiers :

    * **Amazon Bedrock** : définissez la variable d'environnement `CLAUDE_CODE_USE_BEDROCK=1` et configurez les identifiants AWS
    * **Claude Platform on AWS** : définissez `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` et `ANTHROPIC_AWS_WORKSPACE_ID`, puis configurez les identifiants AWS
    * **Google Cloud's Agent Platform** : définissez la variable d'environnement `CLAUDE_CODE_USE_VERTEX=1` et configurez les identifiants Google Cloud
    * **Microsoft Azure** : définissez la variable d'environnement `CLAUDE_CODE_USE_FOUNDRY=1` et configurez les identifiants Azure

    Consultez les guides de configuration pour [Amazon Bedrock](/docs/fr/amazon-bedrock), [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou [Microsoft Foundry](/docs/fr/microsoft-foundry) pour plus de détails.

    <Note>
      Sauf approbation préalable, Anthropic n'autorise pas les développeurs tiers à proposer la connexion claude.ai ou les limites de débit pour leurs produits, y compris les agents construits sur le SDK Claude Agent. Veuillez utiliser les méthodes d'authentification par clé API décrites dans ce document à la place.
    </Note>
  </Step>

  <Step title="Exécuter votre premier agent">
    Cet exemple crée un agent qui liste les fichiers de votre répertoire courant en utilisant les outils intégrés.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**Prêt à construire ?** Suivez le [Guide de démarrage rapide](/docs/fr/agent-sdk/quickstart) pour créer un agent qui trouve et corrige les bugs en quelques minutes.

<h2 id="capabilities">
  Capacités
</h2>

Tout ce qui rend Claude Code puissant est disponible dans le SDK :

<Tabs>
  <Tab title="Outils intégrés">
    Votre agent peut lire des fichiers, exécuter des commandes et rechercher dans les bases de code dès le départ. Les outils clés incluent :

    | Outil                                                                       | Ce qu'il fait                                                                                |
    | --------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
    | **Read**                                                                    | Lire n'importe quel fichier du répertoire de travail                                         |
    | **Write**                                                                   | Créer de nouveaux fichiers                                                                   |
    | **Edit**                                                                    | Effectuer des modifications précises aux fichiers existants                                  |
    | **Bash**                                                                    | Exécuter des commandes de terminal, des scripts, des opérations git                          |
    | **Monitor**                                                                 | Surveiller un script en arrière-plan et réagir à chaque ligne de sortie en tant qu'événement |
    | **Glob**                                                                    | Trouver des fichiers par motif (`**/*.ts`, `src/**/*.py`)                                    |
    | **Grep**                                                                    | Rechercher le contenu des fichiers avec regex                                                |
    | **WebSearch**                                                               | Rechercher sur le web pour obtenir des informations actuelles                                |
    | **WebFetch**                                                                | Récupérer et analyser le contenu des pages web                                               |
    | **[AskUserQuestion](/docs/fr/agent-sdk/user-input#handle-clarifying-questions)** | Poser à l'utilisateur des questions de clarification avec des options à choix multiples      |

    Cet exemple crée un agent qui recherche les commentaires TODO dans votre base de code :

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Hooks">
    Exécutez du code personnalisé à des points clés du cycle de vie de l'agent. Les hooks du SDK utilisent des fonctions de rappel pour valider, enregistrer, bloquer ou transformer le comportement de l'agent.

    **Hooks disponibles :** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, et bien d'autres.

    Cet exemple enregistre tous les changements de fichiers dans un fichier d'audit :

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [En savoir plus sur les hooks →](/docs/fr/agent-sdk/hooks)
  </Tab>

  <Tab title="Sous-agents">
    Générez des agents spécialisés pour gérer des sous-tâches ciblées. Votre agent principal délègue le travail, et les sous-agents rapportent les résultats.

    Définissez des agents personnalisés avec des instructions spécialisées. Les sous-agents sont invoqués via l'outil Agent, donc incluez `Agent` dans `allowedTools` pour approuver automatiquement ces invocations :

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
                      )
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    Les messages provenant du contexte d'un sous-agent incluent un champ `parent_tool_use_id`, ce qui vous permet de suivre les messages appartenant à l'exécution de quel sous-agent.

    [En savoir plus sur les sous-agents →](/docs/fr/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    Connectez-vous à des systèmes externes via le Model Context Protocol : bases de données, navigateurs, API, et [des centaines d'autres](https://github.com/modelcontextprotocol/servers).

    Cet exemple connecte le [serveur Playwright MCP](https://github.com/microsoft/playwright-mcp) pour donner à votre agent des capacités d'automatisation de navigateur :

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [En savoir plus sur MCP →](/docs/fr/agent-sdk/mcp)
  </Tab>

  <Tab title="Permissions">
    Contrôlez exactement quels outils votre agent peut utiliser. Autorisez les opérations sûres, bloquez les opérations dangereuses, ou exigez une approbation pour les actions sensibles.

    <Note>
      Pour les invites d'approbation interactives et l'outil `AskUserQuestion`, consultez [Gérer les approbations et l'entrée utilisateur](/docs/fr/agent-sdk/user-input).
    </Note>

    Cet exemple crée un agent en lecture seule qui peut analyser mais pas modifier le code. `allowed_tools` pré-approuve `Read`, `Glob`, et `Grep`.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [En savoir plus sur les permissions →](/docs/fr/agent-sdk/permissions)
  </Tab>

  <Tab title="Sessions">
    Maintenez le contexte sur plusieurs échanges. Claude se souvient des fichiers lus, de l'analyse effectuée et de l'historique de la conversation. Reprenez les sessions plus tard, ou divisez-les pour explorer différentes approches.

    Cet exemple capture l'ID de session de la première requête, puis reprend pour continuer avec le contexte complet :

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [En savoir plus sur les sessions →](/docs/fr/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Fonctionnalités de Claude Code
</h3>

Le SDK prend également en charge la configuration basée sur le système de fichiers de Claude Code. Avec les options par défaut, le SDK les charge à partir de `.claude/` dans votre répertoire de travail et `~/.claude/`. Pour restreindre les sources qui se chargent, définissez `setting_sources` (Python) ou `settingSources` (TypeScript) dans vos options.

| Fonctionnalité                                   | Description                                                                                                | Emplacement                           |
| ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| [Skills](/docs/fr/agent-sdk/skills)                   | Capacités spécialisées que Claude utilise automatiquement ou que vous invoquez avec `/name`                | `.claude/skills/*/SKILL.md`           |
| [Commands](/docs/fr/agent-sdk/slash-commands)         | Commandes personnalisées au format hérité. Utilisez les skills pour les nouvelles commandes personnalisées | `.claude/commands/*.md`               |
| [Memory](/docs/fr/agent-sdk/modifying-system-prompts) | Contexte du projet et instructions                                                                         | `CLAUDE.md` ou `.claude/CLAUDE.md`    |
| [Plugins](/docs/fr/agent-sdk/plugins)                 | Étendre avec des skills, des agents, des hooks et des serveurs MCP                                         | Programmatique via l'option `plugins` |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Comparer le SDK Agent à d'autres outils Claude
</h2>

La plateforme Claude offre plusieurs façons de construire avec Claude. Voici comment le SDK Agent s'intègre :

<Tabs>
  <Tab title="SDK Agent vs SDK Client">
    Le [SDK Client Anthropic](https://platform.claude.com/docs/fr/api/client-sdks) vous donne un accès direct à l'API : vous envoyez des invites et implémentez vous-même l'exécution des outils. Le **SDK Agent** vous donne Claude avec l'exécution des outils intégrée.

    Avec le SDK Client, vous implémentez une boucle d'outils. Avec le SDK Agent, Claude la gère :

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="SDK Agent vs CLI Claude Code">
    Mêmes capacités, interface différente :

    | Cas d'usage                  | Meilleur choix |
    | ---------------------------- | -------------- |
    | Développement interactif     | CLI            |
    | Pipelines CI/CD              | SDK            |
    | Applications personnalisées  | SDK            |
    | Tâches ponctuelles           | CLI            |
    | Automatisation de production | SDK            |

    De nombreuses équipes utilisent les deux : CLI pour le développement quotidien, SDK pour la production. Les flux de travail se traduisent directement entre eux.
  </Tab>

  <Tab title="SDK Agent vs Agents gérés">
    [Agents gérés](https://platform.claude.com/docs/fr/managed-agents/overview) est une API REST hébergée : Anthropic exécute l'agent et le sandbox, et votre application envoie des événements et reçoit les résultats en streaming. Le **SDK Agent** est une bibliothèque qui exécute la boucle d'agent à l'intérieur de votre propre processus.

    |                           | SDK Agent                                                                                           | Agents gérés                                                                                                  |
    | ------------------------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
    | **S'exécute dans**        | Votre processus, votre infrastructure                                                               | Infrastructure gérée par Anthropic                                                                            |
    | **Interface**             | Bibliothèque Python ou TypeScript                                                                   | API REST                                                                                                      |
    | **L'agent travaille sur** | Fichiers sur votre infrastructure                                                                   | Un sandbox géré par session                                                                                   |
    | **État de la session**    | JSONL sur votre système de fichiers                                                                 | Journal des événements hébergé par Anthropic                                                                  |
    | **Outils personnalisés**  | Fonctions Python ou TypeScript en processus                                                         | Claude déclenche l'outil ; vous exécutez et retournez les résultats                                           |
    | **Idéal pour**            | Prototypage local, agents qui travaillent directement sur votre système de fichiers et vos services | Agents de production sans gérer l'infrastructure du sandbox ou de la session, sessions longues et asynchrones |

    Un chemin courant est de prototyper avec le SDK Agent localement, puis de passer aux Agents gérés pour la production.
  </Tab>
</Tabs>

<h2 id="changelog">
  Journal des modifications
</h2>

Consultez le journal des modifications complet pour les mises à jour du SDK, les corrections de bugs et les nouvelles fonctionnalités :

* **SDK TypeScript** : [voir CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **SDK Python** : [voir CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  Signaler les bugs
</h2>

Si vous rencontrez des bugs ou des problèmes avec le SDK Agent :

* **SDK TypeScript** : [signaler les problèmes sur GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **SDK Python** : [signaler les problèmes sur GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Directives de marque
</h2>

Pour les partenaires intégrant le SDK Claude Agent, l'utilisation de la marque Claude est facultative. Lorsque vous référencez Claude dans votre produit :

**Autorisé :**

* « Claude Agent » (préféré pour les menus déroulants)
* « Claude » (lorsque vous êtes déjà dans un menu étiqueté « Agents »)
* « {YourAgentName} Powered by Claude » (si vous avez un nom d'agent existant)

**Non autorisé :**

* « Claude Code » ou « Claude Code Agent »
* Art ASCII ou éléments visuels de marque Claude Code qui imitent Claude Code

Votre produit doit conserver sa propre marque et ne pas sembler être Claude Code ou un produit Anthropic. Pour des questions sur la conformité de la marque, contactez l'équipe [ventes](https://www.anthropic.com/contact-sales) d'Anthropic.

<h2 id="license-and-terms">
  Licence et conditions
</h2>

L'utilisation du SDK Claude Agent est régie par les [Conditions commerciales d'Anthropic](https://www.anthropic.com/legal/commercial-terms), y compris lorsque vous l'utilisez pour alimenter des produits et services que vous mettez à disposition de vos propres clients et utilisateurs finaux, sauf dans la mesure où un composant ou une dépendance spécifique est couvert par une licence différente comme indiqué dans le fichier LICENSE de ce composant.

<h2 id="next-steps">
  Prochaines étapes
</h2>

<CardGroup cols={2}>
  <Card title="Guide de démarrage rapide" icon="play" href="/docs/fr/agent-sdk/quickstart">
    Créez un agent qui trouve et corrige les bugs en quelques minutes
  </Card>

  <Card title="Agents d'exemple" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistant e-mail, agent de recherche, et bien plus
  </Card>

  <Card title="SDK TypeScript" icon="code" href="/docs/fr/agent-sdk/typescript">
    Référence API TypeScript complète et exemples
  </Card>

  <Card title="SDK Python" icon="code" href="/docs/fr/agent-sdk/python">
    Référence API Python complète et exemples
  </Card>
</CardGroup>
