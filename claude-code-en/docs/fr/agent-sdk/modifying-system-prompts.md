> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modification des invites système

> Choisissez entre le préréglage `claude_code` et une invite système personnalisée, et personnalisez le comportement avec CLAUDE.md, les styles de sortie, append, ou une invite entièrement personnalisée.

Les invites système définissent le comportement, les capacités et le style de réponse de Claude. Commencez par le préréglage `claude_code` pour les outils de codage de type CLI ou IDE où un humain observe et dirige le travail. Écrivez votre propre invite pour les agents ayant une surface, une identité ou un modèle de permissions différents.

Cette page couvre :

* [Fonctionnement des invites système](#how-system-prompts-work), avec un tableau de décision pour choisir entre le préréglage, le préréglage avec `append`, et une invite personnalisée
* [Personnaliser le comportement de l'agent](#customize-agent-behavior) avec des fichiers CLAUDE.md, des styles de sortie, `append`, ou une chaîne personnalisée
* [Comparer les quatre approches](#compare-the-four-approaches) par persistance, portée, et ce qu'elles préservent
* [Combiner les approches](#combine-approaches) pour superposer les méthodes de personnalisation

<h2 id="how-system-prompts-work">
  Fonctionnement des invites système
</h2>

Une invite système est l'ensemble initial d'instructions qui façonne le comportement de Claude tout au long d'une conversation. Le SDK Agent dispose de trois points de départ pour celle-ci :

* **Défaut minimal** : lorsque vous ne définissez pas `systemPrompt` en TypeScript ou `system_prompt` en Python, le SDK utilise une invite minimale qui couvre l'appel d'outils mais omet les directives de codage de Claude Code, le style de réponse et le contexte du projet. Cela diffère de `claude -p`, qui utilise l'invite complète de Claude Code par défaut. Si vous migrez depuis la CLI et souhaitez un comportement correspondant, définissez le préréglage `claude_code`.
* **Préréglage `claude_code`** : l'invite système complète que la CLI Claude Code utilise, avec les instructions d'utilisation des outils, les directives de style et de formatage du code, les règles de ton et de verbosité des réponses, les instructions de sécurité et de sûreté, et le contexte du répertoire de travail et de l'environnement. Définissez `systemPrompt: { type: "preset", preset: "claude_code" }` en TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code"}` en Python, éventuellement avec `append` pour ajouter vos propres instructions à la fin.
* **Chaîne personnalisée** : une invite que vous écrivez vous-même. Le SDK envoie uniquement ce que vous fournissez.

<h3 id="decide-on-a-starting-point">
  Décider d'un point de départ
</h3>

Le facteur décisif est la proximité de votre agent avec Claude Code : un agent de codage opérant dans un référentiel, avec un humain regardant la sortie en continu et dirigeant le travail. Plus votre produit s'éloigne de cela, plus vous voudrez écrire votre propre invite.

| Vous construisez                                                                                                                       | Utiliser                               | Ce que vous obtenez                                                                                                                                                |
| :------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Un outil de codage de type CLI ou IDE où un humain regarde et dirige, et les valeurs par défaut de Claude Code sont ce que vous voulez | Préréglage `claude_code`               | L'invite complète de Claude Code : guidance des outils, règles de sécurité, réponses adaptées au terminal, sensibilisation aux conventions du référentiel          |
| Le même type d'outil, plus des règles spécifiques au produit comme les normes de codage, le format de sortie ou le contexte du domaine | Préréglage `claude_code` avec `append` | Tout ce qui précède, avec vos instructions ajoutées après le préréglage. Rien n'est supprimé, donc c'est la personnalisation à risque le plus faible               |
| Un agent avec une surface, une identité ou un modèle de permission différent, ou un agent non-codage                                   | Chaîne d'invite personnalisée          | Uniquement ce que vous écrivez. Vous êtes responsable du remplacement de la guidance des outils et des instructions de sécurité dont votre agent a toujours besoin |
| Une boucle d'appel d'outils mince sans persona d'agent, où vous fournissez tout le comportement dans l'invite utilisateur              | Pas d'option `systemPrompt`            | Le défaut minimal : support d'appel d'outils et rien d'autre                                                                                                       |

« Différent de Claude Code » signifie généralement l'un des éléments suivants :

* **Surface différente** : la sortie n'est pas lue dans un terminal par la personne qui l'a déclenchée. Les interfaces de chat, les consommateurs de sortie structurée et l'automatisation non-codage ont chacun besoin d'une invite qui correspond à la façon dont leur sortie est rendue et examinée. L'automatisation de codage sans surveillance, comme un travail CI qui corrige les erreurs de lint ou examine les diffs, s'adapte toujours au préréglage car le travail lui-même est ce pour lequel le préréglage est écrit.
* **Identité différente** : l'agent ne devrait pas se présenter comme Claude Code. Un bot d'assistance, un assistant d'analyse de données ou tout agent spécifique à un domaine a besoin de son propre nom, portée et persona.
* **Modèle de permission différent** : l'agent s'exécute de manière autonome sans qu'un humain n'approuve chaque étape, ou opère sur un ensemble étroit de ressources. L'invite de Claude Code suppose qu'un humain est dans la boucle avec accès à un ensemble complet d'outils.
* **Tâches non-codage** : la plupart de l'invite de Claude Code est une guidance de codage. Pour les agents de recherche, de contenu ou d'opérations, cette guidance entre en concurrence avec les instructions dont vous avez réellement besoin.

Le [tableau de comparaison](#compare-the-four-approaches) montre ce que chaque méthode de personnalisation préserve.

<h2 id="customize-agent-behavior">
  Personnaliser le comportement de l'agent
</h2>

Les styles de sortie, `append`, et une chaîne d'invite personnalisée modifient chacun directement l'invite système. CLAUDE.md emprunte un chemin différent : le SDK le lit et injecte son contenu dans la conversation en tant que contexte du projet, non dans l'invite système, donc il façonne le comportement aux côtés de n'importe quelle configuration d'invite système que vous choisissez. Les [Skills](/docs/fr/agent-sdk/skills), les [hooks](/docs/fr/agent-sdk/hooks), et les [permissions](/docs/fr/agent-sdk/permissions) façonnent également le comportement en dehors de l'invite système et sont couverts sur leurs propres pages.

<h3 id="claude-md-files-for-project-level-instructions">
  Fichiers CLAUDE.md pour les instructions au niveau du projet
</h3>

Les fichiers CLAUDE.md donnent à Claude un contexte et des instructions persistants au niveau du projet. Le SDK injecte leur contenu dans la conversation, non dans l'invite système, donc ils fonctionnent avec n'importe quelle configuration d'invite système. Pour savoir quoi mettre dans CLAUDE.md, où le placer, et comment écrire des instructions efficaces, consultez [Comment Claude se souvient de votre projet](/docs/fr/memory). Cette section couvre ce qui est spécifique au SDK : comment CLAUDE.md se charge.

Le SDK lit CLAUDE.md lorsque la source de paramètre correspondante est activée : `'project'` charge `CLAUDE.md` ou `.claude/CLAUDE.md` à partir du répertoire de travail, et `'user'` charge `~/.claude/CLAUDE.md`. Les options `query()` par défaut activent les deux sources, donc CLAUDE.md se charge automatiquement. Si vous définissez `settingSources` en TypeScript ou `setting_sources` en Python explicitement, incluez les sources dont vous avez besoin. Le chargement de CLAUDE.md est contrôlé par les sources de paramètres, non par le préréglage `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Charger CLAUDE.md avec le SDK
</h4>

Pour charger CLAUDE.md, définissez `settingSources` pour inclure le niveau où votre CLAUDE.md se trouve. L'exemple ci-dessous charge un CLAUDE.md au niveau du projet aux côtés du préréglage `claude_code`, donc Claude a à la fois l'invite complète de l'agent de codage et les conventions de votre projet :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []

  async for message in query(
      prompt="Add a new React component for user profiles",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",  # Use Claude Code's system prompt
          },
          setting_sources=["project"],  # Loads CLAUDE.md from project
      ),
  ):
      messages.append(message)

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

CLAUDE.md est persistant dans toutes les sessions d'un projet, partagé avec votre équipe via git, et découvert automatiquement sans modifications de code. Il n'est pas chargé si vous passez un tableau `settingSources` vide.

<h3 id="output-styles-for-persistent-configurations">
  Styles de sortie pour les configurations persistantes
</h3>

Les styles de sortie sont des configurations enregistrées qui modifient l'invite système de Claude. Ils sont stockés sous forme de fichiers markdown et peuvent être réutilisés dans les sessions et les projets.

<h4 id="create-an-output-style">
  Créer un style de sortie
</h4>

Un style de sortie est un fichier markdown avec un [frontmatter](/docs/fr/output-styles#frontmatter) pour les métadonnées, suivi du contenu de l'invite. Enregistrez-le dans `~/.claude/output-styles/` pour un style au niveau de l'utilisateur disponible dans chaque projet, ou `.claude/output-styles/` dans votre référentiel pour un style au niveau du projet que vous pouvez valider et partager avec votre équipe.

Par défaut, un style de sortie personnalisé remplace les instructions d'ingénierie logicielle du préréglage `claude_code` par les vôtres. Pour les conserver et superposer vos instructions par-dessus, définissez `keep-coding-instructions: true` dans le frontmatter. Conservez-les lorsque votre agent effectue toujours du travail d'ingénierie logicielle. Omettez-les lorsque vous remplacez entièrement le rôle.

L'exemple ci-dessous définit une persona d'examen de code qui conserve les instructions de codage, car l'examen du code bénéficie toujours des conseils en matière de sécurité et de qualité du code de Claude Code. Enregistrez-le sous `~/.claude/output-styles/code-reviewer.md` pour le rendre disponible dans tous les projets :

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  Activer un style de sortie
</h4>

Une fois créés, activez les styles de sortie via :

* **CLI** : exécutez `/config` et sélectionnez un style de sortie
* **Paramètres** : définissez `outputStyle` dans `.claude/settings.local.json`
* **SDK TypeScript** : définissez `outputStyle` dans l'objet `settings` en ligne passé à `query()`, ou pointez `settings` vers un fichier de paramètres qui le définit. `outputStyle` n'est pas un champ `Options` de niveau supérieur :

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Le SDK Python n'a pas d'option pour sélectionner un style de sortie par programmation. Pour les déploiements basés sur le code où vous ne pouvez pas écrire dans `.claude/settings.local.json`, utilisez `append` ou une chaîne d'invite personnalisée à la place.

**Remarque pour les utilisateurs du SDK :** Les styles de sortie sont chargés lorsque vous incluez `settingSources: ['user']` ou `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` ou `setting_sources=["project"]` (Python) dans vos options.

<h3 id="append-to-the-claude_code-preset">
  Ajouter au préréglage `claude_code`
</h3>

Vous pouvez utiliser le préréglage Claude Code avec une propriété `append` pour ajouter vos instructions personnalisées tout en préservant toutes les fonctionnalités intégrées.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []

  async for message in query(
      prompt="Help me write a Python function to calculate fibonacci numbers",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "Always include detailed docstrings and type hints in Python code.",
          }
      ),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Améliorer la mise en cache des invites entre les utilisateurs et les machines
</h4>

Par défaut, deux sessions qui utilisent le même préréglage `claude_code` et le même texte `append` ne peuvent toujours pas partager une entrée de cache d'invite si elles s'exécutent à partir de répertoires de travail différents. C'est parce que le préréglage intègre le contexte par session dans l'invite système avant votre texte `append` : le répertoire de travail, qu'il s'agisse d'un référentiel git, la plateforme, le shell actif, la version du système d'exploitation, et les chemins de mémoire automatique. Toute différence dans ce contexte produit une invite système différente et un cache miss. Le contenu de CLAUDE.md n'affecte pas le cache d'invite système car le SDK l'injecte dans la conversation, non dans l'invite système.

Pour rendre l'invite système identique dans les sessions, définissez `excludeDynamicSections: true` en TypeScript ou `"exclude_dynamic_sections": True` en Python. Le contexte par session se déplace dans le premier message utilisateur, laissant uniquement le préréglage statique et votre texte `append` dans l'invite système afin que les configurations identiques partagent une entrée de cache dans les utilisateurs et les machines.

<Note>
  `excludeDynamicSections` nécessite `@anthropic-ai/claude-agent-sdk` v0.2.98 ou ultérieur, ou `claude-agent-sdk` v0.1.58 ou ultérieur pour Python. Il s'applique uniquement à la forme d'objet préréglé et n'a aucun effet lorsque `systemPrompt` est une chaîne.
</Note>

L'exemple suivant associe un bloc `append` partagé avec `excludeDynamicSections` afin qu'une flotte d'agents s'exécutant à partir de répertoires différents puisse réutiliser la même invite système mise en cache :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Triage the open issues in this repo",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
              "exclude_dynamic_sections": True,
          },
      ),
  ):
      ...
  ```
</CodeGroup>

**Compromis :** le répertoire de travail, l'indicateur de référentiel git, la plateforme, le shell actif, la version du système d'exploitation, et les chemins de mémoire automatique atteignent toujours Claude, mais comme faisant partie du premier message utilisateur plutôt que de l'invite système. Les instructions dans le message utilisateur ont un poids légèrement inférieur au même texte dans l'invite système, donc Claude peut s'y fier moins fortement lorsqu'il raisonne sur le répertoire actuel ou les chemins de mémoire automatique. Activez cette option lorsque la réutilisation du cache entre sessions est plus importante que le contexte d'environnement maximalement autoritaire.

Pour l'indicateur équivalent en mode CLI non interactif, consultez [`--exclude-dynamic-system-prompt-sections`](/docs/fr/cli-reference).

<h3 id="custom-system-prompts">
  Invites système personnalisées
</h3>

Vous pouvez fournir une chaîne personnalisée comme `systemPrompt` pour remplacer entièrement la valeur par défaut par vos propres instructions.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []

  async for message in query(
      prompt="Create a data processing pipeline",
      options=ClaudeAgentOptions(system_prompt=custom_prompt),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h2 id="compare-the-four-approaches">
  Comparaison des quatre approches
</h2>

Les quatre méthodes de personnalisation diffèrent par leur emplacement, la façon dont elles sont partagées et ce qu'elles préservent de la présélection `claude_code`.

| Fonctionnalité                 | CLAUDE.md                  | Styles de sortie                            | `systemPrompt` avec append | `systemPrompt` personnalisé     |
| ------------------------------ | -------------------------- | ------------------------------------------- | -------------------------- | ------------------------------- |
| **Persistance**                | Fichier par projet         | Enregistré sous forme de fichiers           | Session uniquement         | Session uniquement              |
| **Réutilisabilité**            | Par projet                 | Entre les projets                           | Duplication de code        | Duplication de code             |
| **Gestion**                    | Sur le système de fichiers | CLI + fichiers                              | Dans le code               | Dans le code                    |
| **Outils par défaut**          | Préservés                  | Préservés                                   | Préservés                  | Perdus (sauf s'ils sont inclus) |
| **Sécurité intégrée**          | Maintenue                  | Maintenue                                   | Maintenue                  | Doit être ajoutée               |
| **Contexte d'environnement**   | Automatique                | Automatique                                 | Automatique                | Doit être fourni                |
| **Niveau de personnalisation** | Ajouts uniquement          | Remplacer la valeur par défaut ou l'étendre | Ajouts uniquement          | Contrôle complet                |
| **Contrôle de version**        | Avec le projet             | Oui                                         | Avec le code               | Avec le code                    |
| **Portée**                     | Spécifique au projet       | Utilisateur ou projet                       | Session de code            | Session de code                 |

« Avec append » signifie utiliser `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` en TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` en Python. CLAUDE.md ne modifie pas le message système lui-même : le SDK injecte son contenu dans la conversation en tant que contexte du projet.

<h2 id="use-cases-and-best-practices">
  Cas d'utilisation et meilleures pratiques
</h2>

<h3 id="when-to-use-claude-md">
  Quand utiliser CLAUDE.md
</h3>

Utilisez CLAUDE.md pour les instructions qui doivent s'appliquer à chaque session dans un projet, indépendamment du système prompt utilisé par la session : normes de codage, commandes courantes, contexte d'architecture et conventions d'équipe. CLAUDE.md est validé dans votre référentiel, il reste donc synchronisé avec le code qu'il décrit. Consultez [Quand ajouter à CLAUDE.md](/docs/fr/memory#when-to-add-to-claude-md) pour des conseils complets.

Les fichiers CLAUDE.md se chargent lorsque la source de paramètre `project` est activée, ce qui est le cas pour les options `query()` par défaut. Si vous définissez explicitement `settingSources` en TypeScript ou `setting_sources` en Python, incluez `'project'` pour continuer à charger CLAUDE.md au niveau du projet.

<h3 id="when-to-use-output-styles">
  Quand utiliser les styles de sortie
</h3>

Les styles de sortie sont destinés aux personas que vous souhaitez réutiliser dans l'interface de ligne de commande et le SDK sans modifier le code de l'application. Comme ils résident sous forme de fichiers dans `.claude/output-styles`, le même persona est disponible à partir de `/config` dans l'interface de ligne de commande et à partir de toute session SDK qui charge la source de paramètre correspondante.

**Idéal pour :**

* Les modifications de comportement persistantes dans les sessions
* Les configurations partagées par l'équipe
* Les assistants spécialisés comme un examinateur de code, un data scientist ou un assistant DevOps
* Les modifications d'invite complexes qui nécessitent une gestion de version

**Exemples :**

* Créer un assistant dédié d'optimisation SQL
* Construire un examinateur de code axé sur la sécurité
* Développer un assistant pédagogique avec une pédagogie spécifique

<h3 id="when-to-use-systemprompt-with-append">
  Quand utiliser `systemPrompt` avec append
</h3>

Utilisez `append` lorsque le préréglage `claude_code` convient déjà à votre produit et que vous avez seulement besoin d'ajouter des instructions supplémentaires. Vous conservez les conseils d'outils du préréglage, les règles de sécurité et les conventions de codage sans les réimplémenter.

**Idéal pour :**

* Ajouter des normes ou des préférences de codage spécifiques
* Personnaliser le formatage de la sortie
* Ajouter des connaissances spécifiques au domaine
* Modifier la verbosité des réponses
* Améliorer le comportement par défaut de Claude Code sans perdre les instructions des outils

<h3 id="when-to-use-custom-systemprompt">
  Quand utiliser `systemPrompt` personnalisé
</h3>

Utilisez une invite personnalisée lorsque la surface, l'identité ou le modèle de permission de votre agent diffère de celui de Claude Code, comme décrit dans [Décider d'un point de départ](#decide-on-a-starting-point). Vous définissez l'ensemble complet des instructions, y compris tout conseil d'outils et toute règle de sécurité dont votre agent a besoin.

**Idéal pour :**

* Contrôle complet du comportement de Claude
* Les tâches spécialisées d'une seule session
* Tester de nouvelles stratégies d'invite
* Les situations où les outils par défaut ne sont pas nécessaires
* Construire des agents spécialisés avec un comportement unique

<h2 id="combine-approaches">
  Combiner les approches
</h2>

Ces méthodes se composent. Un style de sortie persistant ou CLAUDE.md définit le comportement à long terme, et `append` superpose les instructions spécifiques à la session sans modifier la configuration enregistrée.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combiner un style de sortie avec des ajouts spécifiques à la session
</h3>

L'exemple ci-dessous suppose qu'un style de sortie Code Reviewer est déjà actif. Le bloc `append` superpose les domaines de focus spécifiques à la session sur la persona, de sorte qu'une seule session de révision peut prioriser OAuth et le stockage des tokens sans modifier le style de sortie enregistré :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []

  async for message in query(
      prompt="Review this authentication module",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": """
              For this review, prioritize:
              - OAuth 2.0 compliance
              - Token storage security
              - Session management
              """,
          }
      ),
  ):
      messages.append(message)
  ```
</CodeGroup>

<h2 id="see-also">
  Voir aussi
</h2>

* [Styles de sortie](/docs/fr/output-styles) : créer, gérer et partager les styles de sortie pour la CLI, y compris le format de fichier et les emplacements de stockage
* [Comment Claude se souvient de votre projet](/docs/fr/memory) : ce qu'il faut mettre dans CLAUDE.md, où le placer et comment rédiger des instructions de projet efficaces
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) : le type `Options` complet, y compris `systemPrompt`, `settingSources` et `settings`
* [Référence du SDK Python](/docs/fr/agent-sdk/python) : le type `ClaudeAgentOptions` complet, y compris `system_prompt` et `setting_sources`
* [Paramètres](/docs/fr/settings) : la référence `settings.json`, y compris où les styles de sortie et autres configurations sont stockés
