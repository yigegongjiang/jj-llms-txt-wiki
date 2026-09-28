> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Étendre les agents avec des skills

> Contrôlez les skills que Claude peut invoquer dans les sessions du Claude Agent SDK, distribuez les commandes par nom et créez des skills que vos sessions découvrent

Agent Skills étendent Claude avec des capacités spécialisées que Claude invoque lorsque c'est pertinent. Les Skills sont empaquetés sous forme de fichiers `SKILL.md` contenant des instructions, des descriptions et des ressources de support optionnelles. Cette page couvre également les [commandes dans les sessions du Agent SDK](#commands-in-agent-sdk-sessions).

Pour des informations complètes sur les skills, y compris les avantages, l'architecture et les directives de création, consultez l'[aperçu d'Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-agent-sdk">
  Comment les skills fonctionnent avec le Agent SDK
</h2>

Lors de l'utilisation du Claude Agent SDK, les skills sont :

* **Définis comme des artefacts du système de fichiers** : vous créez chaque skill sous forme de fichier `SKILL.md` dans son propre répertoire, tel que `.claude/skills/<name>/SKILL.md`
* **Chargés à partir du système de fichiers** : le SDK charge les skills à partir des emplacements du système de fichiers régis par `settingSources` (TypeScript) ou `setting_sources` (Python)
* **Découverts automatiquement** : une fois que les paramètres du système de fichiers sont chargés, le SDK découvre les métadonnées des skills au démarrage à partir des répertoires utilisateur et projet, et charge le contenu complet lorsque Claude invoque le skill
* **Invoqués par le modèle** : Claude choisit de manière autonome quand les utiliser en fonction du contexte
* **Invoqués par l'utilisateur** : vous distribuez un skill directement en envoyant `/<name>` dans une invite. Voir [Commandes dans les sessions du Agent SDK](#commands-in-agent-sdk-sessions)
* **Limités via l'option `skills`** : les skills découverts sont activés par défaut. Passez une liste de noms de skills, `"all"`, ou `[]` pour contrôler lesquels Claude peut invoquer

Contrairement aux sous-agents, que vous pouvez définir dans l'[option `agents`](/docs/fr/agent-sdk/subagents#programmatic-definition-recommended), vous créez les skills sous forme de fichiers sur le disque. Le SDK ne fournit pas d'API programmatique pour les enregistrer.

<Note>
  Les skills sont découverts via les sources de paramètres du système de fichiers. Avec les options `query()` par défaut, le SDK charge les sources utilisateur et projet, donc les skills dans `~/.claude/skills/`, `<cwd>/.claude/skills/`, et `.claude/skills/` dans n'importe quel répertoire parent de `<cwd>` jusqu'à la racine du référentiel sont disponibles. La source du projet couvre également `<dir>/.claude/skills/` dans chaque répertoire que vous transmettez via `additionalDirectories` (TypeScript) ou `add_dirs` (Python), car le SDK transmet ces répertoires à Claude Code en tant que [`--add-dir`](/docs/fr/skills#skills-from-additional-directories). Si vous définissez `settingSources` explicitement, incluez `'project'` pour conserver les skills du projet et des répertoires ajoutés et `'user'` pour conserver vos skills personnels, ou utilisez l'[option `plugins`](/docs/fr/agent-sdk/plugins) pour charger les skills à partir d'un chemin spécifique.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Utiliser les skills avec le Agent SDK
</h2>

Définissez l'option `skills` sur `query()` pour contrôler lesquels Claude peut invoquer dans la session. Lorsqu'elle est omise, les skills découverts sont activés et l'outil Skill est disponible, ce qui correspond au comportement de la CLI. Passez `"all"` pour laisser Claude invoquer chaque skill découvert, une liste de noms de skills pour autoriser uniquement ceux-ci, ou `[]` pour laisser Claude n'en invoquer aucun.

Par exemple, pour laisser Claude invoquer uniquement deux skills nommés :

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  Configurer les skills dans une session
</h3>

Lorsque vous définissez `skills`, le SDK ajoute automatiquement l'outil Skill à `allowedTools`. Si vous transmettez également une liste `tools` explicite, incluez `"Skill"` dans cette liste afin que Claude puisse invoquer les skills.

Une fois configuré, Claude découvre automatiquement les skills à partir du système de fichiers et les invoque lorsque c'est pertinent pour la demande de l'utilisateur.

L'exemple suivant active chaque skill découvert dans une session et pré-approuve les outils que les skills ont généralement besoin. L'exemple définit `cwd` sur le répertoire de travail actuel du processus, donc exécutez-le à partir d'un projet qui a un répertoire `.claude/skills/` dans le répertoire actuel ou n'importe quel parent jusqu'à la racine du référentiel :

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
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
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Confirmer que les skills sont chargés
</h3>

Près du début du flux, le SDK produit un message système avec le sous-type `init`. Vérifiez son tableau `skills` pour confirmer que vos skills sont chargés avant que Claude ne commence à travailler. Le tableau inclut les skills invocables par l'utilisateur que vous avez définis avec un champ frontmatter `description` ou `when_to_use`, ainsi que les [skills groupés inclus avec Claude Code](/docs/fr/skills#bundled-skills).

Le tableau liste uniquement les skills invocables par l'utilisateur. Un skill avec [`user-invocable: false`](/docs/fr/skills#control-who-invokes-a-skill) dans son frontmatter se charge et reste disponible pour Claude, mais n'apparaît pas dans le tableau. Le tableau liste les mêmes skills que vous soyez ou non dans votre liste `skills`.

<h3 id="allow-only-specific-skills">
  Autoriser uniquement des skills spécifiques
</h3>

Pour laisser Claude invoquer uniquement des skills spécifiques, passez leurs noms dans la liste `skills`. Les noms correspondent au champ `name` dans `SKILL.md` ou au nom du répertoire du skill. Utilisez `plugin:skill` pour les skills fournis par les plugins.

La liste ne prend que les noms de skills exacts. Si une entrée ne peut pas fonctionner comme un nom exact, `query()` rejette la liste avant le démarrage de la session. Voir [Erreur de nom de skill invalide](#invalid-skill-name-error) pour les règles de nommage et l'erreur que chaque SDK lève.

Le modèle ne voit pas les skills non listés et l'outil Skill les rejette, tandis que leurs fichiers restent sur le disque et restent accessibles via Read et Bash. Restreindre la liste ne restreint pas la [distribution par nom](#dispatch-commands-by-name).

Pour laisser Claude invoquer chaque skill découvert, passez `skills: "all"` plutôt qu'un caractère générique.

<h2 id="commands-in-agent-sdk-sessions">
  Commandes dans les sessions du Agent SDK
</h2>

Cette section est la documentation des commandes du SDK. Une commande est tout ce que vous exécutez en envoyant `/<name>` dans une invite. Les entrées sur la surface de commande diffèrent dans ce qui les soutient :

* **Commandes intégrées** : exécutent la logique codée dans le processus Claude Code que le SDK exécute, par exemple `/compact`
* **Skills groupés** : artefacts d'invite inclus avec Claude Code, par exemple `/code-review`
* **Vos skills** : artefacts d'invite que vous créez, chacun un répertoire contenant un fichier `SKILL.md`. Le nom d'un skill invocable par l'utilisateur rejoint automatiquement la surface, donc distribuer votre propre `/security-check` et exécuter un intégré fonctionnent de la même manière
* **Fichiers de commande personnalisés** : une forme d'artefact plus ancienne avec le même comportement, des fichiers Markdown plats dans `.claude/commands/` dont les noms de fichiers deviennent des noms de commande. Les skills sont leur successeur recommandé

Par défaut, vous et Claude pouvez invoquer n'importe quel skill. Vous pouvez restreindre l'un ou l'autre chemin via le [frontmatter](/docs/fr/skills#control-who-invokes-a-skill) du skill. Pour une définition des deux termes, voir les entrées [Commande](/docs/fr/glossary#command) et [Skill](/docs/fr/glossary#skill) du glossaire. Voir [Commandes dans Claude Code](/docs/fr/commands) pour chaque intégré et [Étendre Claude avec des skills](/docs/fr/skills) pour le guide complet des deux formes d'artefacts.

<h3 id="discover-available-commands">
  Découvrir les commandes disponibles
</h3>

Vous pouvez distribuer les commandes qui fonctionnent sans terminal interactif via le SDK. Le message `system/init` liste celles disponibles dans votre session dans son champ `slash_commands`. Les commandes qui ont besoin d'un terminal interactif, telles que `/theme` et `/terminal-setup`, n'apparaissent pas dans la liste. Accédez au champ au démarrage de votre session :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

La liste imprimée mélange les commandes intégrées, les skills groupés, vos skills invocables par l'utilisateur et les fichiers `.claude/commands/` :

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

Un skill avec [`user-invocable: false`](/docs/fr/skills#control-who-invokes-a-skill) dans son frontmatter n'apparaît pas dans cette liste ou dans le tableau `skills` de [Confirmer que les skills sont chargés](#confirm-skills-loaded). Les sessions qui configurent les [serveurs MCP](/docs/fr/agent-sdk/mcp) peuvent également exposer les [invites MCP en tant que commandes](/docs/fr/mcp#use-mcp-prompts-as-commands).

<h3 id="dispatch-commands-by-name">
  Distribuer les commandes par nom
</h3>

Envoyez une commande en l'incluant dans votre chaîne d'invite, de la même manière que vous envoyez du texte régulier. La distribution ne dépend pas de l'option `skills`. Envoyer `/<name>` exécute un skill invocable par l'utilisateur même lorsque votre liste `skills` l'omet. Les commandes qui agissent sur l'historique de la conversation, telles que `/compact`, ont besoin de messages antérieurs pour fonctionner.

Un `/<name>` qui ne correspond ni à une commande de la session ni à une commande Claude Code intégrée ne fait pas échouer la requête. Claude Code envoie l'invite à Claude en tant que message ordinaire, avec une note indiquant que la commande n'a pas été exécutée, donc la requête dépense un tour de modèle et retourne la réponse de Claude. Avant v2.1.274, un `/<name>` qui ne correspondait à rien retournait `Unknown command: /<name>` comme résultat sans tour de modèle.

Un `/<name>` qui correspond à une commande Claude Code intégrée qui n'est pas disponible dans la session, telle que `/theme`, retourne `/theme isn't available in this environment.` comme résultat sans tour de modèle.

<Note>
  Une commande peut atteindre la limite `maxTurns` / `max_turns` comme n'importe quelle autre invite, terminant la requête avec un résultat d'erreur au lieu de `success`. Pour le contrat de résultat d'erreur, voir [Gérer le résultat](/docs/fr/agent-sdk/agent-loop#handle-the-result). Si votre commande pourrait atteindre la limite, enveloppez la boucle dans un `try`/`catch` en TypeScript ou `try`/`except` en Python, comme indiqué dans [Entrée de message unique](/docs/fr/agent-sdk/streaming-vs-single-mode#single-message-input), ou définissez `maxTurns` assez haut pour que le travail se termine.
</Note>

<h3 id="compact-history-with-/compact">
  Compacter l'historique avec `/compact`
</h3>

La commande `/compact` réduit la taille de votre historique de conversation en résumant les messages plus anciens tout en préservant le contexte important. La compaction a besoin d'une conversation existante avec suffisamment de messages antérieurs à résumer. Cet exemple a d'abord une conversation, puis la compacte et lit le message système `compact_boundary` qui rapporte le résultat :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  Un message `compact_boundary` n'arrive que lorsque la compaction s'est exécutée. S'il n'y a rien à résumer, `/compact` rapporte la raison au lieu de lever. L'exécution se termine toujours avec un résultat `success` et aucun message `compact_boundary`, et le texte du résultat porte la raison, par exemple `Not enough messages to compact.` après un court échange unique. Un appel `query()` unique et nouveau commence avec un contexte vide, donc utilisez ce modèle dans une session avec des tours antérieurs, par exemple en [mode d'entrée en continu](/docs/fr/agent-sdk/streaming-vs-single-mode) ou lors de la reprise d'une session.
</Note>

<h3 id="reset-context-with-/clear">
  Réinitialiser le contexte avec `/clear`
</h3>

La commande `/clear` réinitialise la conversation à un contexte vide, donc les invites suivantes commencent sans historique de conversation antérieur. La conversation précédente reste sur le disque. Vous pouvez revenir à cette conversation en passant son ID de session à l'[option `resume`](/docs/fr/agent-sdk/sessions#resume-by-id).

`/clear` est utile en [mode d'entrée en continu](/docs/fr/agent-sdk/streaming-vs-single-mode), où vous envoyez plusieurs invites sur une seule connexion. Pour les appels `query()` uniques, chaque appel commence déjà avec un contexte vide, donc envoyer `/clear` n'a aucun effet pratique. Commencez plutôt un nouveau `query()`.

<h2 id="create-skills">
  Créer des skills
</h2>

Créez chaque skill en tant que répertoire contenant un fichier `SKILL.md` avec un frontmatter YAML et du contenu Markdown. Le champ `description` détermine quand Claude invoque votre skill.

**Exemple de structure de répertoire** :

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  Choisir un niveau de découverte
</h3>

Enregistrez les skills à l'un des deux [niveaux de découverte](/docs/fr/skills#where-skills-live) les plus courants :

* **Skills de projet** : `.claude/skills/`, disponibles uniquement dans le projet actuel
* **Skills personnels** : `~/.claude/skills/`, disponibles dans tous vos projets

Si vous avez des fichiers de commandes personnalisées existants dans `.claude/commands/`, ils continuent de fonctionner. Un fichier de commande à `.claude/commands/deploy.md` crée `/deploy` et fonctionne de la même manière qu'une skill à `.claude/skills/deploy/SKILL.md`. Si un fichier de commande et une skill partagent un nom, consultez [Résoudre les skills qui partagent un nom](/docs/fr/skills#resolve-skills-that-share-a-name) pour savoir lequel s'exécute. Le SDK charge les fichiers `.claude/commands/` et `~/.claude/commands/` à partir des deux mêmes portées que les skills. Consultez [Étendre Claude avec des skills](/docs/fr/skills) pour le guide complet des deux formes d'artefacts.

<h3 id="create-and-dispatch-your-first-skill">
  Créer et dispatcher votre première skill
</h3>

Pour voir le flux complet, créez `.claude/skills/security-check/SKILL.md` :

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

Une fois le fichier créé, la skill est disponible via le SDK. Claude l'invoque quand une demande correspond à sa description, et vous pouvez la dispatcher directement :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Une exécution réussie se termine par un résultat `success` dont le texte porte les conclusions de l'analyse. Sur une petite application Express avec des problèmes semés, le texte du résultat commence par :

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

Le nom de la skill apparaît également dans le tableau `slash_commands` du message d'initialisation.

<Note>
  Claude Code inclut les skills groupées `code-review` et `verify`. Si vous nommez un fichier `.claude/commands/` d'après l'une d'elles, par exemple `.claude/commands/code-review.md`, le fichier de commande masque la skill groupée et `slash_commands` liste le nom une seule fois.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Pré-approuver les outils pour les skills
</h2>

<Note>
  Pour les skills du projet et personnels, Claude Code applique le champ frontmatter [`allowed-tools`](/docs/fr/skills#pre-approve-tools-for-a-skill) dans les sessions du SDK. Vous pouvez également pré-approuver les outils pour ces skills via l'option `allowedTools` (`allowed_tools` en Python) dans votre configuration de requête. Les skills [synchronisés depuis claude.ai](/docs/fr/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) suivent leurs propres règles de frontmatter.
</Note>

Les skills s'exécutent avec les outils de la session. L'exemple ci-dessous pré-approuve `Read`, `Grep` et `Glob` avec `allowedTools` (`allowed_tools` en Python), afin que Claude puisse inspecter les fichiers lors de l'exécution du [skill security-check](#create-and-dispatch-your-first-skill) sans s'arrêter pour approbation :

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Dans le flux, l'invocation du skill apparaît comme une utilisation de l'outil Skill, suivie d'appels Read sur les fichiers du projet. L'exécution se termine avec un résultat `success` dont le texte porte les résultats.

La liste pré-approuve les outils nommés plutôt que de restreindre les autres. Pour le flux de permission complet, y compris les modes de permission et le rappel `canUseTool`, voir [Permissions](/docs/fr/agent-sdk/permissions).

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="skills-not-found">
  Skills non trouvés
</h3>

**Vérifiez la configuration settingSources** : le SDK découvre les skills via les sources de paramètres `user` et `project`. Si vous définissez `settingSources`/`setting_sources` explicitement et omettez ces sources, le SDK ne charge pas les skills :

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
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Pour savoir quels répertoires de skills chaque source charge, voir le [tableau des sources du système de fichiers](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources). Pour plus de détails sur `settingSources`/`setting_sources`, voir la [référence du SDK TypeScript](/docs/fr/agent-sdk/typescript#settingsource) ou la [référence du SDK Python](/docs/fr/agent-sdk/python#settingsource).

**Vérifiez le répertoire de travail** : le SDK charge les skills à partir de `.claude/skills/` dans l'option `cwd` et dans chaque répertoire parent jusqu'à la racine du référentiel. Assurez-vous que `cwd` pointe vers ou en dessous du répertoire contenant `.claude/skills/`, dans le même référentiel :

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

Voir [Utiliser les skills avec le Agent SDK](#use-skills-with-the-agent-sdk) pour le modèle complet.

**Vérifiez l'emplacement du système de fichiers** :

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill non utilisé
</h3>

**Vérifiez l'option `skills`** : si vous avez passé une liste `skills`, confirmez que le nom du skill est inclus. Lorsque Claude essaie d'invoquer un skill non listé, l'outil Skill retourne `Skill <name> is not in this session's skills allowlist`. Ajoutez le nom à votre liste, ou distribuez le skill directement en envoyant `/<name>` dans une invite, ce qui fonctionne sans listing.

**Vérifiez la description** : assurez-vous qu'elle est spécifique et inclut les mots-clés pertinents. Voir [Meilleures pratiques d'Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) pour des conseils sur la rédaction de descriptions efficaces.

<h3 id="invalid-skill-name-error">
  Erreur de nom de skill invalide
</h3>

Lorsqu'un nom dans votre liste `skills` ne peut pas fonctionner comme un nom de skill exact, `query()` rejette la liste avant de démarrer le processus Claude Code. Les noms qui déclenchent le rejet incluent :

* Un nom vide
* Un nom contenant des parenthèses, des virgules ou des caractères de contrôle
* Un nom rembourré avec des espaces
* Une forme de caractère générique telle qu'un `*` nu ou un suffixe `:*`

Chaque SDK expose le rejet différemment :

<Tabs>
  <Tab title="TypeScript">
    Le SDK TypeScript lève une `Error` indiquant la règle que l'entrée a enfreinte. Par exemple, `skills: ["docs:*"]` lève :

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Un nom vide rapporte `Skill names must be non-empty strings.`

    Avant le TypeScript Agent SDK 0.3.221, le SDK n'exécutait pas cette vérification.
  </Tab>

  <Tab title="Python">
    Le SDK Python lève `ValueError` indiquant la règle que l'entrée a enfreinte. Par exemple, `skills=["docs:*"]` lève :

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Un nom vide rapporte `Skill names must be non-empty strings`.

    Avant le Python Agent SDK 0.2.129, le SDK n'exécutait pas cette vérification.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  Dépannage supplémentaire
</h3>

Pour le dépannage général des skills, tel que les erreurs de syntaxe YAML et le débogage, voir la [section dépannage des skills de Claude Code](/docs/fr/skills#troubleshooting).

<h2 id="next-steps">
  Étapes suivantes
</h2>

Le [guide des skills de Claude Code](/docs/fr/skills) couvre la création en profondeur. Ses conseils s'appliquent aux sessions du SDK. Commencez par ces sections :

* [Référence du frontmatter](/docs/fr/skills#frontmatter-reference) : chaque champ pris en charge
* [Passer des arguments aux skills](/docs/fr/skills#pass-arguments-to-skills) : `$ARGUMENTS`, `$0`, `$1` et l'empilement de skills. Le [tableau de substitution complet](/docs/fr/skills#available-string-substitutions) ajoute les arguments nommés et les variables `${CLAUDE_*}`
* [Injecter du contexte dynamique](/docs/fr/skills#inject-dynamic-context) : les lignes `` !`command` `` qui s'exécutent avant que Claude ne voie le contenu du skill
* [Choisir où les skills se chargent](/docs/fr/skills#where-skills-live) : chaque emplacement de skill, l'espace de noms des plugins et quel skill s'exécute lorsque deux partagent un nom

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Commandes dans Claude Code](/docs/fr/commands) : la surface de commande complète, y compris chaque intégré
* [Aperçu d'Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview) : aperçu conceptuel, avantages et architecture
* [Meilleures pratiques d'Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices) : directives de création pour des skills efficaces
* [Livre de recettes d'Agent Skills](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction) : exemples de skills et modèles
* [Sous-agents dans le SDK](/docs/fr/agent-sdk/subagents) : agents similaires basés sur le système de fichiers avec options programmatiques
* [Aperçu du SDK](/docs/fr/agent-sdk/overview) : concepts généraux du SDK
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) : documentation complète de l'API
* [Référence du SDK Python](/docs/fr/agent-sdk/python) : documentation complète de l'API
