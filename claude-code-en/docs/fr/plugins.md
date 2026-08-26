> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Créer des plugins

> Créez des plugins personnalisés pour étendre Claude Code avec des skills, des agents, des hooks et des serveurs MCP.

Les plugins vous permettent d'étendre Claude Code avec des fonctionnalités personnalisées qui peuvent être partagées entre les projets et les équipes. Ce guide couvre la création de vos propres plugins avec des skills, des agents, des hooks et des serveurs MCP.

Vous cherchez à installer des plugins existants ? Consultez [Découvrir et installer des plugins](/docs/fr/discover-plugins). Pour les spécifications techniques complètes, consultez [Référence des plugins](/docs/fr/plugins-reference).

<h2 id="when-to-use-plugins-vs-standalone-configuration">
  Quand utiliser les plugins par rapport à la configuration autonome
</h2>

Claude Code prend en charge deux façons d'ajouter des skills, des agents et des hooks personnalisés :

| Approche                                                                                           | Noms des skills      | Idéal pour                                                                                                    |
| :------------------------------------------------------------------------------------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------ |
| **Autonome** (répertoire `.claude/`)                                                               | `/hello`             | Flux de travail personnels, personnalisations spécifiques au projet, expériences rapides                      |
| **Plugins** (répertoires avec skills, agents, hooks, ou un manifeste `.claude-plugin/plugin.json`) | `/plugin-name:hello` | Partage avec les coéquipiers, distribution à la communauté, versions publiées, réutilisable entre les projets |

**Utilisez la configuration autonome quand** :

* Vous personnalisez Claude Code pour un seul projet
* La configuration est personnelle et n'a pas besoin d'être partagée
* Vous expérimentez avec des skills ou des hooks avant de les empaqueter
* Vous voulez des noms de skills courts comme `/hello` ou `/deploy`

**Utilisez les plugins quand** :

* Vous voulez partager des fonctionnalités avec votre équipe ou la communauté
* Vous avez besoin des mêmes skills/agents sur plusieurs projets
* Vous voulez le contrôle de version et les mises à jour faciles pour vos extensions
* Vous distribuez via une marketplace
* Vous êtes d'accord avec les skills avec espace de noms comme `/my-plugin:hello` (l'espace de noms prévient les conflits entre les plugins)

<Tip>
  Commencez par la configuration autonome dans `.claude/` pour une itération rapide, puis [convertissez en plugin](#convert-existing-configurations-to-plugins) quand vous êtes prêt à partager.
</Tip>

<h2 id="quickstart">
  Démarrage rapide
</h2>

Ce démarrage rapide vous guide dans la création d'un plugin avec un skill personnalisé. Vous allez créer un manifeste (le fichier de configuration qui définit votre plugin), ajouter un skill et le tester localement en utilisant le drapeau `--plugin-dir`.

<h3 id="prerequisites">
  Prérequis
</h3>

* Claude Code [installé et authentifié](/docs/fr/quickstart#step-1-install-claude-code)

<Note>
  Si vous ne voyez pas la commande `/plugin`, mettez à jour Claude Code vers la dernière version. Consultez [Dépannage](/docs/fr/troubleshooting) pour les instructions de mise à niveau.
</Note>

<h3 id="create-your-first-plugin">
  Créez votre premier plugin
</h3>

<Steps>
  <Step title="Créez le répertoire du plugin">
    Chaque plugin se trouve dans son propre répertoire contenant vos skills, agents ou hooks, optionnellement aux côtés d'un manifeste `.claude-plugin/plugin.json`. L'emplacement n'a pas d'importance pour ce démarrage rapide car vous pointerez Claude Code vers le répertoire avec `--plugin-dir` à l'étape de test. Créez-le n'importe où, par exemple dans un dossier de travail ou un répertoire de projets :

    ```bash theme={null}
    mkdir my-first-plugin
    ```

    Les étapes restantes s'exécutent à partir du répertoire parent et font référence à des chemins comme `my-first-plugin/...` relatifs à celui-ci.
  </Step>

  <Step title="Créez le manifeste du plugin">
    Le fichier manifeste à `.claude-plugin/plugin.json` définit l'identité de votre plugin : son nom, sa description et sa version. Claude Code utilise ces métadonnées pour afficher votre plugin dans le gestionnaire de plugins.

    Créez le répertoire `.claude-plugin` à l'intérieur de votre dossier de plugin :

    ```bash theme={null}
    mkdir my-first-plugin/.claude-plugin
    ```

    Ensuite, créez `my-first-plugin/.claude-plugin/plugin.json` avec ce contenu :

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    | Champ         | Objectif                                                                                                                                                                                                                                                                                                                              |
    | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    | `name`        | Identifiant unique et espace de noms du skill. Les skills sont préfixés avec ceci (par exemple, `/my-first-plugin:hello`).                                                                                                                                                                                                            |
    | `description` | Affiché dans le gestionnaire de plugins lors de la navigation ou de l'installation de plugins.                                                                                                                                                                                                                                        |
    | `version`     | Optionnel. S'il est défini, les utilisateurs ne reçoivent les mises à jour que lorsque vous augmentez ce champ. S'il est omis et que votre plugin est distribué via git, le SHA du commit est utilisé et chaque commit compte comme une nouvelle version. Consultez [gestion des versions](/docs/fr/plugins-reference#version-management). |
    | `author`      | Optionnel. Utile pour l'attribution.                                                                                                                                                                                                                                                                                                  |

    Pour les champs supplémentaires comme `homepage`, `repository` et `license`, consultez le [schéma manifeste complet](/docs/fr/plugins-reference#plugin-manifest-schema).
  </Step>

  <Step title="Ajoutez un skill">
    Les skills se trouvent dans le répertoire `skills/`. Chaque skill est un dossier contenant un fichier `SKILL.md`. Le nom du dossier devient le nom du skill, préfixé par l'espace de noms du plugin (`hello/` dans un plugin nommé `my-first-plugin` crée `/my-first-plugin:hello`).

    Créez un répertoire de skill dans votre dossier de plugin :

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Ensuite, créez `my-first-plugin/skills/hello/SKILL.md` avec ce contenu :

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```
  </Step>

  <Step title="Testez votre plugin">
    Exécutez Claude Code avec le drapeau `--plugin-dir` pour charger votre plugin :

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Une fois Claude Code démarré, essayez votre nouveau skill :

    ```shell theme={null}
    /my-first-plugin:hello
    ```

    Vous verrez Claude répondre avec un salut. Exécutez `/help` pour voir votre skill listé sous l'espace de noms du plugin.

    <Note>
      **Pourquoi l'espace de noms ?** Les skills des plugins sont toujours avec espace de noms (comme `/my-first-plugin:hello`) pour prévenir les conflits quand plusieurs plugins ont des skills avec le même nom.

      Pour changer le préfixe d'espace de noms, mettez à jour le champ `name` dans `plugin.json`.
    </Note>
  </Step>

  <Step title="Ajoutez des arguments au skill">
    Rendez votre skill dynamique en acceptant l'entrée de l'utilisateur. L'espace réservé `$ARGUMENTS` capture tout texte que l'utilisateur fournit après le nom du skill.

    Mettez à jour votre fichier `SKILL.md` :

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a personalized message
    ---

    # Hello Skill

    Greet the user named "$ARGUMENTS" warmly and ask how you can help them today. Make the greeting personal and encouraging.
    ```

    Exécutez `/reload-plugins` pour récupérer les modifications, puis essayez le skill avec votre nom :

    ```shell theme={null}
    /my-first-plugin:hello Alex
    ```

    Claude vous saluera par votre nom. Pour plus d'informations sur la transmission d'arguments aux skills, consultez [Skills](/docs/fr/skills#pass-arguments-to-skills).
  </Step>
</Steps>

Vous avez créé et testé avec succès un plugin avec ces composants clés :

* **Manifeste du plugin** (`.claude-plugin/plugin.json`) : décrit les métadonnées de votre plugin
* **Répertoire des skills** (`skills/`) : contient vos skills personnalisés
* **Arguments du skill** (`$ARGUMENTS`) : capture l'entrée de l'utilisateur pour un comportement dynamique

<Tip>
  Le drapeau `--plugin-dir` est utile pour le développement et les tests. Quand vous êtes prêt à partager votre plugin avec d'autres, consultez [Créer et distribuer une marketplace de plugins](/docs/fr/plugin-marketplaces).
</Tip>

<h2 id="develop-a-plugin-in-your-skills-directory">
  Développez un plugin dans votre répertoire de skills
</h2>

Au lieu de passer `--plugin-dir` à chaque lancement, vous pouvez garder un plugin dans votre répertoire de skills et faire en sorte que Claude Code le charge automatiquement. `claude plugin init` en crée un :

```bash theme={null}
claude plugin init my-tool
```

Cela crée `~/.claude/skills/my-tool/` avec un manifeste `.claude-plugin/plugin.json` et un `SKILL.md` de démarrage. À la session suivante, il se charge en tant que `my-tool@skills-dir` sans étape de marketplace ou d'installation.

Pour les règles de chargement automatique, la portée personnelle par rapport à la portée du projet, l'exigence de confiance de l'espace de travail et comment mettre à jour ou supprimer un, consultez [Plugins du répertoire de skills](/docs/fr/plugins-reference#skills-directory-plugins).

<h2 id="plugin-structure-overview">
  Aperçu de la structure du plugin
</h2>

Vous avez créé un plugin avec un skill, mais les plugins peuvent inclure beaucoup plus : des agents personnalisés, des hooks, des serveurs MCP, des serveurs LSP et des moniteurs en arrière-plan.

<Warning>
  **Erreur courante** : Ne mettez pas `commands/`, `agents/`, `skills/` ou `hooks/` à l'intérieur du répertoire `.claude-plugin/`. Seul `plugin.json` va à l'intérieur de `.claude-plugin/`. Tous les autres répertoires doivent être au niveau racine du plugin.

  La racine du plugin est le répertoire propre du plugin individuel : celui qui contient `.claude-plugin/plugin.json`. Ce n'est jamais `~/.claude/`. Par exemple, Claude Code ne lit pas un `.mcp.json` placé à `~/.claude/.mcp.json`.
</Warning>

| Répertoire        | Emplacement      | Objectif                                                                                                |
| :---------------- | :--------------- | :------------------------------------------------------------------------------------------------------ |
| `.claude-plugin/` | Racine du plugin | Contient le manifeste `plugin.json` (optionnel si les composants utilisent les emplacements par défaut) |
| `skills/`         | Racine du plugin | Skills en tant que répertoires `<name>/SKILL.md`                                                        |
| `commands/`       | Racine du plugin | Skills en tant que fichiers Markdown plats. Utilisez `skills/` pour les nouveaux plugins                |
| `agents/`         | Racine du plugin | Définitions d'agents personnalisés                                                                      |
| `hooks/`          | Racine du plugin | Gestionnaires d'événements dans `hooks.json`                                                            |
| `.mcp.json`       | Racine du plugin | Configurations du serveur MCP                                                                           |
| `.lsp.json`       | Racine du plugin | Configurations du serveur LSP pour l'intelligence du code                                               |
| `monitors/`       | Racine du plugin | Configurations du moniteur en arrière-plan dans `monitors.json`                                         |
| `bin/`            | Racine du plugin | Exécutables ajoutés au `PATH` de l'outil Bash tandis que le plugin est activé                           |
| `settings.json`   | Racine du plugin | [Paramètres](/docs/fr/settings) par défaut appliqués quand le plugin est activé                              |

Un plugin qui fournit exactement un skill peut placer `SKILL.md` directement à la racine du plugin au lieu de créer un répertoire `skills/`. Claude Code le charge en tant que skill unique et utilise le champ `name` du frontmatter pour le nom d'invocation. Utilisez la disposition `skills/` pour les plugins qui pourraient croître pour avoir plus d'un skill.

<Note>
  **Prochaines étapes** : Prêt à ajouter plus de fonctionnalités ? Allez à [Développer des plugins plus complexes](#develop-more-complex-plugins) pour ajouter des agents, des hooks, des serveurs MCP et des serveurs LSP. Pour les spécifications techniques complètes de tous les composants du plugin, consultez [Référence des plugins](/docs/fr/plugins-reference).
</Note>

<h2 id="develop-more-complex-plugins">
  Développer des plugins plus complexes
</h2>

Une fois que vous êtes à l'aise avec les plugins de base, vous pouvez créer des extensions plus sophistiquées.

<h3 id="add-skills-to-your-plugin">
  Ajoutez des Skills à votre plugin
</h3>

Les plugins peuvent inclure des [Agent Skills](/docs/fr/skills) pour étendre les capacités de Claude. Les skills sont invoqués par le modèle : Claude les utilise automatiquement en fonction du contexte de la tâche.

Ajoutez un répertoire `skills/` à la racine de votre plugin avec des dossiers de Skill contenant des fichiers `SKILL.md` :

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── code-review/
        └── SKILL.md
```

Chaque `SKILL.md` contient un frontmatter YAML et des instructions. Incluez une `description` pour que Claude sache quand utiliser le skill :

```yaml theme={null}
---
description: Reviews code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
---

When reviewing code, check for:
1. Code organization and structure
2. Error handling
3. Security concerns
4. Test coverage
```

Après l'installation du plugin, exécutez `/reload-plugins` pour charger les Skills. Pour des conseils complets sur la création de Skills incluant la divulgation progressive et les restrictions d'outils, consultez [Agent Skills](/docs/fr/skills).

<h3 id="add-lsp-servers-to-your-plugin">
  Ajoutez des serveurs LSP à votre plugin
</h3>

<Tip>
  Pour les langages courants comme TypeScript, Python et Rust, installez les plugins LSP pré-construits à partir de la marketplace officielle. Créez des plugins LSP personnalisés uniquement quand vous avez besoin de support pour des langages non encore couverts.
</Tip>

Les plugins LSP (Language Server Protocol) donnent à Claude l'intelligence du code en temps réel. Si vous avez besoin de supporter un langage qui n'a pas de plugin LSP officiel, vous pouvez en créer un en ajoutant un fichier `.lsp.json` à votre plugin :

```json .lsp.json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

Les utilisateurs qui installent votre plugin doivent avoir le binaire du serveur de langage installé sur leur machine.

Pour les options de configuration LSP complètes, consultez [Serveurs LSP](/docs/fr/plugins-reference#lsp-servers).

<h3 id="add-background-monitors-to-your-plugin">
  Ajoutez des moniteurs en arrière-plan à votre plugin
</h3>

Les moniteurs en arrière-plan permettent à votre plugin de surveiller les journaux, les fichiers ou l'état externe en arrière-plan et de notifier Claude à mesure que les événements arrivent. Claude Code démarre automatiquement chaque moniteur quand le plugin est actif, donc vous n'avez pas besoin d'instruire Claude pour démarrer la surveillance.

Ajoutez un fichier `monitors/monitors.json` à la racine du plugin avec un tableau d'entrées de moniteur :

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Chaque ligne stdout de `command` est livrée à Claude en tant que notification pendant la session. Pour le schéma complet, incluant le déclencheur `when` et la substitution de variables, consultez [Moniteurs](/docs/fr/plugins-reference#monitors).

<h3 id="ship-default-settings-with-your-plugin">
  Livrez les paramètres par défaut avec votre plugin
</h3>

Les plugins peuvent inclure un fichier `settings.json` à la racine du plugin pour appliquer la configuration par défaut quand le plugin est activé. Actuellement, seules les clés `agent` et `subagentStatusLine` sont supportées.

Définir `agent` active l'un des [agents personnalisés](/docs/fr/sub-agents) du plugin en tant que thread principal, en appliquant son invite système, ses restrictions d'outils et son modèle. Cela permet à un plugin de changer le comportement par défaut de Claude Code quand il est activé.

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Cet exemple active l'agent `security-reviewer` défini dans le répertoire `agents/` du plugin. Les paramètres de `settings.json` ont priorité sur les `settings` déclarés dans `plugin.json`. Les clés inconnues sont silencieusement ignorées.

<h3 id="organize-complex-plugins">
  Organisez les plugins complexes
</h3>

Pour les plugins avec de nombreux composants, organisez votre structure de répertoires par fonctionnalité. Pour les dispositions de répertoires complètes et les modèles d'organisation, consultez [Structure du répertoire du plugin](/docs/fr/plugins-reference#plugin-directory-structure).

<h3 id="test-your-plugins-locally">
  Testez vos plugins localement
</h3>

Utilisez le drapeau `--plugin-dir` pour tester les plugins pendant le développement. Cela charge votre plugin directement sans nécessiter d'installation.

```bash theme={null}
claude --plugin-dir ./my-plugin
```

Le drapeau accepte également une archive `.zip` du répertoire du plugin, qui nécessite Claude Code v2.1.128 ou ultérieur.

```bash theme={null}
claude --plugin-dir ./my-plugin.zip
```

Quand un plugin `--plugin-dir` a le même nom qu'un plugin marketplace installé, la copie locale prend la priorité pour cette session. Cela vous permet de tester les modifications d'un plugin que vous avez déjà installé sans le désinstaller d'abord. L'exception concerne les plugins dont les paramètres gérés forcent l'activation ou la désactivation : `--plugin-dir` ne peut pas les remplacer.

À mesure que vous apportez des modifications à votre plugin, exécutez `/reload-plugins` pour récupérer les mises à jour sans redémarrer. Cela recharge les plugins, les skills, les agents, les hooks, les serveurs MCP du plugin et les serveurs LSP du plugin. Testez vos composants de plugin :

* Essayez vos skills avec `/plugin-name:skill-name`
* Vérifiez que les agents apparaissent dans `/context` sous Agents personnalisés, ou mentionnez-en un avec le symbole @ par son nom délimité
* Vérifiez que les hooks fonctionnent comme prévu

<Tip>
  Vous pouvez charger plusieurs plugins à la fois en spécifiant le drapeau plusieurs fois :

  ```bash theme={null}
  claude --plugin-dir ./plugin-one --plugin-dir ./plugin-two
  ```
</Tip>

Pour tester un plugin qui est déjà empaqueté en tant qu'archive `.zip` et hébergé à une URL, comme un artefact de build CI, utilisez `--plugin-url` à la place. Claude Code récupère l'archive au démarrage et la charge pour cette session uniquement. Si la récupération échoue ou que l'archive est invalide, Claude Code signale une erreur de chargement de plugin et démarre sans elle. Les mêmes [considérations de confiance](/docs/fr/discover-plugins#security) s'appliquent que pour toute source de plugin : pointez uniquement ce drapeau vers des archives que vous contrôlez ou en lesquelles vous avez confiance.

Pour charger plusieurs plugins, répétez le drapeau pour chaque URL :

```bash theme={null}
claude --plugin-url https://example.com/my-plugin.zip --plugin-url https://example.com/other.zip
```

Ou passez des URL séparées par des espaces en tant qu'un seul argument entre guillemets :

```bash theme={null}
claude --plugin-url "https://example.com/my-plugin.zip https://example.com/other.zip"
```

<h3 id="debug-plugin-issues">
  Déboguez les problèmes de plugin
</h3>

Si votre plugin ne fonctionne pas comme prévu :

1. **Vérifiez la structure** : Assurez-vous que vos répertoires sont à la racine du plugin, pas à l'intérieur de `.claude-plugin/`
2. **Testez les composants individuellement** : Vérifiez chaque skill, agent et hook séparément
3. **Utilisez les outils de validation et de débogage** : Consultez [Outils de débogage et de développement](/docs/fr/plugins-reference#debugging-and-development-tools) pour les commandes CLI et les techniques de dépannage

<h3 id="share-your-plugins">
  Partagez vos plugins
</h3>

Quand votre plugin est prêt à être partagé :

1. **Ajoutez de la documentation** : Incluez un `README.md` avec les instructions d'installation et d'utilisation
2. **Choisissez une stratégie de versioning** : Décidez si vous allez définir une `version` explicite ou vous fier au SHA du commit git. Consultez [gestion des versions](/docs/fr/plugins-reference#version-management)
3. **Créez ou utilisez une marketplace** : Distribuez via des [marketplaces de plugins](/docs/fr/plugin-marketplaces) pour l'installation
4. **Testez avec d'autres** : Faites tester le plugin par les membres de l'équipe avant une distribution plus large

Une fois que votre plugin est dans une marketplace, d'autres peuvent l'installer en utilisant les instructions dans [Découvrir et installer des plugins](/docs/fr/discover-plugins). Pour garder un plugin interne à votre équipe, hébergez la marketplace dans un [référentiel privé](/docs/fr/plugin-marketplaces#private-repositories).

<h3 id="submit-your-plugin-to-the-community-marketplace">
  Soumettez votre plugin à la marketplace communautaire
</h3>

Anthropic maintient deux marketplaces publiques pour les plugins Claude Code :

* **`claude-plugins-official`** : un ensemble organisé de plugins maintenus par Anthropic. Enregistré automatiquement la première fois que vous démarrez Claude Code de manière interactive. Un script non-interactif qui s'exécute avant ce premier lancement doit l'ajouter explicitement avec `claude plugin marketplace add anthropics/claude-plugins-official`.
* **`claude-community`** : la marketplace communautaire publique où les soumissions tierces arrivent après examen. Les utilisateurs l'ajoutent avec `/plugin marketplace add anthropics/claude-plugins-community` et l'installent en tant que `@claude-community`.

Pour soumettre votre plugin pour examen de la marketplace communautaire, utilisez l'un des formulaires dans l'application :

* **claude.ai** : [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console** : [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Le formulaire claude.ai nécessite une organisation Team ou Enterprise et un accès à la gestion du répertoire ; les propriétaires d'organisation ont cet accès par défaut. Les auteurs individuels qui ne font pas partie d'une organisation Team ou Enterprise peuvent utiliser le formulaire Console à la place.

Exécutez `claude plugin validate` localement avant de soumettre. Le pipeline d'examen exécute la même vérification sur chaque soumission, ainsi qu'un dépistage de sécurité automatisé.

Les plugins approuvés sont épinglés à un SHA de commit spécifique dans le catalogue [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), et CI augmente automatiquement l'épingle à mesure que vous poussez de nouveaux commits vers votre référentiel. Le catalogue public se synchronise chaque nuit à partir du pipeline d'examen, il peut donc y avoir un délai entre l'approbation et l'apparition de votre plugin dans `marketplace.json`. Pour vérifier si votre plugin est installable, recherchez son nom dans le [catalogue communautaire](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

La marketplace officielle, `claude-plugins-official`, est organisée séparément. Anthropic décide quels plugins inclure à sa discrétion. Il n'y a pas de processus de candidature, et le formulaire de soumission n'ajoute pas de plugins à la marketplace officielle.

Si Anthropic liste votre plugin dans la marketplace officielle, votre CLI peut inviter les utilisateurs de Claude Code à l'installer. Consultez [Recommander votre plugin à partir de votre CLI](/docs/fr/plugin-hints).

<Note>
  Pour les spécifications techniques complètes, les techniques de débogage et les stratégies de distribution, consultez [Référence des plugins](/docs/fr/plugins-reference).
</Note>

<h2 id="convert-existing-configurations-to-plugins">
  Convertir les configurations existantes en plugins
</h2>

Si vous avez déjà des skills ou des hooks dans votre répertoire `.claude/`, vous pouvez les convertir en plugin pour un partage et une distribution plus faciles.

<h3 id="migration-steps">
  Étapes de migration
</h3>

<Steps>
  <Step title="Créez la structure du plugin">
    Créez un nouveau répertoire de plugin dans la racine de votre projet, à côté du dossier `.claude/` existant, afin que les chemins `cp` relatifs à l'étape suivante se résolvent :

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Créez le fichier manifeste à `my-plugin/.claude-plugin/plugin.json` :

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Copiez vos fichiers existants">
    Copiez vos configurations existantes dans le répertoire du plugin :

    ```bash theme={null}
    # Copy commands
    cp -r .claude/commands my-plugin/

    # Copy agents (if any)
    cp -r .claude/agents my-plugin/

    # Copy skills (if any)
    cp -r .claude/skills my-plugin/
    ```
  </Step>

  <Step title="Migrez les hooks">
    Si vous avez des hooks dans vos paramètres, créez un répertoire de hooks :

    ```bash theme={null}
    mkdir my-plugin/hooks
    ```

    Créez `my-plugin/hooks/hooks.json` avec votre configuration de hooks. Copiez l'objet `hooks` de votre `.claude/settings.json` ou `settings.local.json`, car le format est le même. La commande reçoit l'entrée du hook en tant que JSON sur stdin, donc utilisez `jq` pour extraire le chemin du fichier :

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Testez votre plugin migré">
    Chargez votre plugin pour vérifier que tout fonctionne :

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Testez chaque composant : exécutez vos commandes, vérifiez que les agents apparaissent dans `/context`, et vérifiez que les hooks se déclenchent correctement.
  </Step>
</Steps>

<h3 id="what-changes-when-migrating">
  Ce qui change lors de la migration
</h3>

| Autonome (`.claude/`)                      | Plugin                                 |
| :----------------------------------------- | :------------------------------------- |
| Disponible uniquement dans un projet       | Peut être partagé via des marketplaces |
| Fichiers dans `.claude/commands/`          | Fichiers dans `plugin-name/commands/`  |
| Hooks dans `settings.json`                 | Hooks dans `hooks/hooks.json`          |
| Doit être copié manuellement pour partager | Installer avec `/plugin install`       |

<Note>
  Après la migration, supprimez les fichiers originaux de `.claude/` pour éviter les doublons. Les définitions d'agents du projet et de l'utilisateur dans `.claude/agents/` remplacent les agents du plugin portant le même nom, donc la version du plugin ne prend effet qu'une fois que les originaux sont supprimés. Les skills du plugin sont espacés de noms sous la forme `/plugin-name:skill-name`, donc l'original `/skill-name` et la copie du plugin restent tous deux disponibles plutôt que l'un remplaçant l'autre.
</Note>

<h2 id="next-steps">
  Prochaines étapes
</h2>

Maintenant que vous comprenez le système de plugins de Claude Code, voici les chemins suggérés pour différents objectifs :

<h3 id="for-plugin-users">
  Pour les utilisateurs de plugins
</h3>

* [Découvrir et installer des plugins](/docs/fr/discover-plugins) : parcourir les marketplaces et installer des plugins
* [Configurer les marketplaces d'équipe](/docs/fr/discover-plugins#configure-team-marketplaces) : configurer les plugins au niveau du référentiel pour votre équipe

<h3 id="for-plugin-developers">
  Pour les développeurs de plugins
</h3>

* [Créer et distribuer une marketplace](/docs/fr/plugin-marketplaces) : empaqueter et partager vos plugins
* [Référence des plugins](/docs/fr/plugins-reference) : spécifications techniques complètes
* Approfondissez les composants spécifiques du plugin :
  * [Skills](/docs/fr/skills) : détails du développement des skills
  * [Subagents](/docs/fr/sub-agents) : configuration et capacités des agents
  * [Hooks](/docs/fr/hooks) : gestion des événements et automatisation
  * [MCP](/docs/fr/mcp) : intégration d'outils externes
