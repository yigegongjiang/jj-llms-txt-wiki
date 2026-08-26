> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills dans le SDK

> Étendez Claude avec des capacités spécialisées en utilisant Agent Skills dans le Claude Agent SDK

<h2 id="overview">
  Aperçu
</h2>

Agent Skills étendent Claude avec des capacités spécialisées que Claude invoque de manière autonome lorsque c'est pertinent. Les Skills sont empaquetés sous forme de fichiers `SKILL.md` contenant des instructions, des descriptions et des ressources de support optionnelles.

Pour des informations complètes sur les Skills, y compris les avantages, l'architecture et les directives de création, consultez l'[aperçu d'Agent Skills](https://platform.claude.com/docs/fr/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-sdk">
  Comment les Skills fonctionnent avec le SDK
</h2>

Lors de l'utilisation du Claude Agent SDK, les Skills sont :

1. **Définis comme des artefacts du système de fichiers** : Créés sous forme de fichiers `SKILL.md` dans des répertoires spécifiques (`.claude/skills/`)
2. **Chargés à partir du système de fichiers** : Les Skills sont chargés à partir des emplacements du système de fichiers régis par `settingSources` (TypeScript) ou `setting_sources` (Python)
3. **Découverts automatiquement** : Une fois que les paramètres du système de fichiers sont chargés, les métadonnées des Skills sont découvertes au démarrage à partir des répertoires utilisateur et projet ; le contenu complet est chargé lorsqu'il est déclenché
4. **Invoqués par le modèle** : Claude choisit de manière autonome quand les utiliser en fonction du contexte
5. **Filtrés via l'option `skills`** : Les Skills découverts sont activés par défaut. Passez une liste de noms de Skills, `"all"`, ou `[]` pour contrôler lesquels sont disponibles dans la session

Contrairement aux sous-agents (qui peuvent être définis par programmation), les Skills doivent être créés comme des artefacts du système de fichiers. Le SDK ne fournit pas d'API programmatique pour enregistrer les Skills.

<Note>
  Les Skills sont découverts via les sources de paramètres du système de fichiers. Avec les options `query()` par défaut, le SDK charge les sources utilisateur et projet, donc les skills dans `~/.claude/skills/`, `<cwd>/.claude/skills/`, et `.claude/skills/` dans n'importe quel répertoire parent de `<cwd>` jusqu'à la racine du référentiel sont disponibles. Si vous définissez `settingSources` explicitement, incluez `'user'` ou `'project'` pour maintenir la découverte des skills, ou utilisez l'[option `plugins`](/docs/fr/agent-sdk/plugins) pour charger les skills à partir d'un chemin spécifique.
</Note>

<h2 id="using-skills-with-the-sdk">
  Utilisation des Skills avec le SDK
</h2>

Définissez l'option `skills` sur `query()` pour contrôler quels Skills sont disponibles pour la session. Lorsqu'elle est omise, les Skills découverts sont activés et l'outil Skill est disponible, ce qui correspond au comportement de la CLI. Passez `"all"` pour activer chaque Skill découvert, une liste de noms de Skills pour activer uniquement ceux-ci, ou `[]` pour désactiver tous les Skills. Lorsque vous définissez `skills`, le SDK ajoute automatiquement l'outil Skill à `allowedTools`. Si vous transmettez également une liste `tools` explicite, incluez `"Skill"` dans cette liste afin que Claude puisse invoquer les skills.

Une fois configuré, Claude découvre automatiquement les Skills à partir du système de fichiers et les invoque lorsque c'est pertinent pour la demande de l'utilisateur.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd="/path/to/project",  # Project with .claude/skills/
          setting_sources=["user", "project"],  # Load Skills from filesystem
          skills="all",  # Enable every discovered Skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: "/path/to/project", // Project with .claude/skills/
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all", // Enable every discovered Skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Pour activer uniquement des Skills spécifiques, passez leurs noms. Les noms correspondent au champ `name` dans `SKILL.md` ou au nom du répertoire du Skill. Utilisez `plugin:skill` pour les Skills fournis par les plugins.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

L'option `skills` est un filtre de contexte, pas un bac à sable. Les Skills non listés sont masqués au modèle et rejetés par l'outil Skill, mais leurs fichiers restent sur le disque et sont accessibles via Read et Bash.

<h2 id="skill-locations">
  Emplacements des Skills
</h2>

Les Skills sont chargés à partir des répertoires du système de fichiers en fonction de votre configuration `settingSources`/`setting_sources` :

* **Project Skills** (`.claude/skills/`) : Partagés avec votre équipe via git - chargés lorsque `setting_sources` inclut `"project"`
* **User Skills** (`~/.claude/skills/`) : Skills personnels dans tous les projets - chargés lorsque `setting_sources` inclut `"user"`
* **Plugin Skills** : Fournis avec les plugins Claude Code installés

<h2 id="creating-skills">
  Création de Skills
</h2>

Les Skills sont définis comme des répertoires contenant un fichier `SKILL.md` avec un frontmatter YAML et du contenu Markdown. Le champ `description` détermine quand Claude invoque votre Skill.

**Exemple de structure de répertoire** :

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

Pour des conseils complets sur la création de Skills, y compris la structure SKILL.md, les Skills multi-fichiers et les exemples, consultez :

* [Agent Skills dans Claude Code](/docs/fr/skills) : Guide complet avec des exemples
* [Agent Skills Best Practices](https://platform.claude.com/docs/fr/agents-and-tools/agent-skills/best-practices) : Directives de création et conventions de nommage

<h2 id="tool-restrictions">
  Restrictions d'outils
</h2>

<Note>
  Le champ frontmatter `allowed-tools` dans SKILL.md n'est pris en charge que lors de l'utilisation directe de la CLI Claude Code. **Il ne s'applique pas lors de l'utilisation de Skills via le SDK**.

  Lors de l'utilisation du SDK, contrôlez l'accès aux outils via l'option principale `allowedTools` dans votre configuration de requête.
</Note>

Pour contrôler l'accès aux outils pour les Skills dans les applications SDK, utilisez `allowedTools` pour pré-approuver des outils spécifiques. Sans un rappel `canUseTool`, tout ce qui ne figure pas dans la liste est refusé :

<Note>
  Les déclarations d'importation du premier exemple sont supposées dans les extraits de code suivants.
</Note>

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )

  async for message in query(prompt="Analyze the codebase structure", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Analyze the codebase structure",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"],
      permissionMode: "dontAsk" // Deny anything not in allowedTools
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="discovering-available-skills">
  Découverte des Skills disponibles
</h2>

Pour voir quels Skills sont disponibles dans votre application SDK, demandez simplement à Claude :

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
  )

  async for message in query(prompt="What Skills are available?", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "What Skills are available?",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all"
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude listera les Skills disponibles en fonction de votre répertoire de travail actuel et des plugins installés.

<h2 id="testing-skills">
  Test des Skills
</h2>

Testez les Skills en posant des questions qui correspondent à leurs descriptions :

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      cwd="/path/to/project",
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Bash"],
  )

  async for message in query(prompt="Extract text from invoice.pdf", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Extract text from invoice.pdf",
    options: {
      cwd: "/path/to/project",
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude invoque automatiquement le Skill pertinent si la description correspond à votre demande.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="skills-not-found">
  Skills non trouvés
</h3>

**Vérifiez la configuration settingSources** : Les Skills sont découverts via les sources de paramètres `user` et `project`. Si vous définissez `settingSources`/`setting_sources` explicitement et omettez ces sources, les skills ne sont pas chargés :

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const options = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const options = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Pour plus de détails sur `settingSources`/`setting_sources`, consultez la [référence du SDK TypeScript](/docs/fr/agent-sdk/typescript#settingsource) ou la [référence du SDK Python](/docs/fr/agent-sdk/python#settingsource).

**Vérifiez le répertoire de travail** : Le SDK charge les Skills à partir de `.claude/skills/` dans l'option `cwd` et dans chaque répertoire parent jusqu'à la racine du référentiel. Assurez-vous que `cwd` pointe vers ou en dessous du répertoire contenant `.claude/skills/`, dans le même référentiel :

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

Consultez la section « Utilisation des Skills avec le SDK » ci-dessus pour le modèle complet.

**Vérifiez l'emplacement du système de fichiers** :

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill non utilisé
</h3>

**Vérifiez l'option `skills`** : Si vous avez passé une liste `skills`, confirmez que le nom du skill est inclus. Passer `[]` désactive tous les skills.

**Vérifiez la description** : Assurez-vous qu'elle est spécifique et inclut les mots-clés pertinents. Consultez [Agent Skills Best Practices](https://platform.claude.com/docs/fr/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) pour des conseils sur la rédaction de descriptions efficaces.

<h3 id="additional-troubleshooting">
  Dépannage supplémentaire
</h3>

Pour le dépannage général des Skills (syntaxe YAML, débogage, etc.), consultez la [section dépannage des Skills de Claude Code](/docs/fr/skills#troubleshooting).

<h2 id="related-documentation">
  Documentation connexe
</h2>

<h3 id="skills-guides">
  Guides des Skills
</h3>

* [Agent Skills dans Claude Code](/docs/fr/skills) : Guide complet des Skills avec création, exemples et dépannage
* [Agent Skills Overview](https://platform.claude.com/docs/fr/agents-and-tools/agent-skills/overview) : Aperçu conceptuel, avantages et architecture
* [Agent Skills Best Practices](https://platform.claude.com/docs/fr/agents-and-tools/agent-skills/best-practices) : Directives de création pour des Skills efficaces
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction) : Exemples de Skills et modèles

<h3 id="sdk-resources">
  Ressources du SDK
</h3>

* [Subagents dans le SDK](/docs/fr/agent-sdk/subagents) : Agents similaires basés sur le système de fichiers avec options programmatiques
* [Slash Commands dans le SDK](/docs/fr/agent-sdk/slash-commands) : Commandes invoquées par l'utilisateur
* [Aperçu du SDK](/docs/fr/agent-sdk/overview) : Concepts généraux du SDK
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) : Documentation complète de l'API
* [Référence du SDK Python](/docs/fr/agent-sdk/python) : Documentation complète de l'API
