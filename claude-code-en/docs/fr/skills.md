> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Étendre Claude avec des skills

> Créez, gérez et partagez des skills pour étendre les capacités de Claude dans Claude Code. Inclut les commandes personnalisées et les skills groupées.

Les skills étendent ce que Claude peut faire. Créez un fichier `SKILL.md` avec des instructions, et Claude l'ajoute à sa boîte à outils. Claude utilise les skills quand c'est pertinent, ou vous pouvez en invoquer une directement avec `/skill-name`.

Créez une skill quand vous continuez à coller les mêmes instructions, checklist ou procédure multi-étapes dans le chat, ou quand une section de CLAUDE.md s'est transformée en procédure plutôt qu'en fait. Contrairement au contenu de CLAUDE.md, le corps d'une skill ne se charge que quand elle est utilisée, donc le matériel de référence long coûte presque rien jusqu'à ce que vous en ayez besoin.

<Note>
  Pour les commandes intégrées comme `/help` et `/compact`, et les skills groupées comme `/debug` et `/code-review`, consultez la [référence des commandes](/docs/fr/commands).

  **Les commandes personnalisées ont été fusionnées dans les skills.** Un fichier à `.claude/commands/deploy.md` et une skill à `.claude/skills/deploy/SKILL.md` créent tous les deux `/deploy` et fonctionnent de la même manière. Vos fichiers `.claude/commands/` existants continuent de fonctionner. Les skills ajoutent des fonctionnalités optionnelles : un répertoire pour les fichiers de support, un frontmatter pour [contrôler si vous ou Claude invoquez la skill](#control-who-invokes-a-skill), et la capacité pour Claude de les charger automatiquement quand c'est pertinent.
</Note>

Les skills Claude Code suivent la norme ouverte [Agent Skills](https://agentskills.io), qui fonctionne sur plusieurs outils d'IA. Claude Code étend la norme avec des fonctionnalités supplémentaires comme le [contrôle d'invocation](#control-who-invokes-a-skill), l'[exécution de subagent](#run-skills-in-a-subagent), et l'[injection de contexte dynamique](#inject-dynamic-context).

<h2 id="bundled-skills">
  Skills groupées
</h2>

Claude Code inclut un ensemble de skills groupées qui sont disponibles dans chaque session sauf si elles sont désactivées avec le paramètre [`disableBundledSkills`](/docs/fr/settings#available-settings), notamment `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` et `/claude-api`. Contrairement à la plupart des commandes intégrées, qui exécutent une logique fixe directement, les skills groupées sont basées sur des prompts : elles donnent à Claude des instructions détaillées et le laissent orchestrer le travail en utilisant ses outils. Vous les invoquez de la même manière que n'importe quelle autre skill, en tapant `/` suivi du nom de la skill.

La vérification de configuration [`/doctor`](/docs/fr/commands#all-commands) est l'une des exceptions à `disableBundledSkills` dans Claude Code v2.1.205 et versions ultérieures : elle reste tapable quand le paramètre est activé. Pour la masquer, définissez la variable d'environnement `DISABLE_DOCTOR_COMMAND` ou une entrée [`skillOverrides`](#override-skill-visibility-from-settings) de `"doctor": "off"`. Avant v2.1.205, `/doctor` était une commande intégrée plutôt qu'une skill groupée.

Les skills groupées sont listées aux côtés des commandes intégrées dans la [référence des commandes](/docs/fr/commands), marquées **Skill** dans la colonne Objectif.

<h3 id="run-and-verify-your-app">
  Exécuter et vérifier votre application
</h3>

Trois skills groupées travaillent ensemble pour lancer votre application et confirmer les modifications par rapport à l'application en cours d'exécution au lieu de simplement des tests :

| Skill                  | Objectif                                                                                                                                                         |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Lancer et piloter votre application pour voir un changement fonctionner                                                                                          |
| `/verify`              | Construire et exécuter votre application pour confirmer qu'une modification de code fait ce qu'elle devrait, sans revenir aux tests ou aux vérifications de type |
| `/run-skill-generator` | Enseigner à `/run` et `/verify` comment construire et lancer votre projet                                                                                        |

Les trois skills nécessitent Claude Code v2.1.145 ou version ultérieure.

`/run` et `/verify` fonctionnent sans configuration. Ils déduisent le lancement de votre type de projet (CLI, serveur, TUI, piloté par navigateur) et de ce qui se trouve dans votre README, `package.json` ou `Makefile`. Cette déduction devient peu fiable pour les projets qui nécessitent quelque chose au-delà d'un lancement standard : une base de données, un fichier env, une session graphique, une construction multi-étapes.

`/run-skill-generator` enregistre la recette à la place. Il fait fonctionner votre application à partir d'un environnement propre, capture ce qui a fonctionné (les commandes d'installation, les variables d'environnement, le script de lancement) et l'engage en tant que skill par projet à `.claude/skills/run-<name>/`. Après cela, `/run`, `/verify` et tout autre agent du référentiel suivent la recette enregistrée au lieu de la redécouvrir. Exécutez `/run-skill-generator` une fois par projet, et à nouveau si le processus de construction ou de lancement change.

<h2 id="getting-started">
  Démarrage
</h2>

<h3 id="create-your-first-skill">
  Créer votre première skill
</h3>

Cet exemple crée une skill qui résume les modifications non validées dans votre référentiel git et signale tout ce qui est risqué. Elle extrait le diff en direct dans l'invite avant que Claude ne le lise, de sorte que la réponse est ancrée dans votre arborescence de travail réelle plutôt que dans ce que Claude peut deviner à partir des fichiers ouverts. Claude charge la skill automatiquement quand vous demandez des informations sur vos modifications, ou vous pouvez l'invoquer directement avec `/summarize-changes`.

<Steps>
  <Step title="Créer le répertoire de la skill">
    Créez un répertoire pour la skill dans votre dossier de skills personnelles. Les skills personnelles sont disponibles dans tous vos projets.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Écrire SKILL.md">
    Chaque skill a besoin d'un fichier `SKILL.md` avec deux parties : un frontmatter YAML entre les marqueurs `---` qui dit à Claude quand utiliser la skill, et du contenu markdown avec les instructions que Claude suit quand la skill s'exécute. Le nom du répertoire devient la commande que vous tapez, et la `description` aide Claude à décider quand charger la skill automatiquement.

    Enregistrez ceci dans `~/.claude/skills/summarize-changes/SKILL.md` :

    ```yaml theme={null}
    ---
    description: Résume les modifications non validées et signale tout ce qui est risqué. À utiliser quand l'utilisateur demande ce qui a changé, veut un message de commit, ou demande d'examiner son diff.
    ---

    ## Modifications actuelles

    !`git diff HEAD`

    ## Instructions

    Résumez les modifications ci-dessus en deux ou trois points, puis listez tous les risques que vous remarquez tels que la gestion des erreurs manquante, les valeurs codées en dur, ou les tests qui doivent être mis à jour. Si le diff est vide, dites qu'il n'y a pas de modifications non validées.
    ```

    La ligne `` !`git diff HEAD` `` utilise [l'injection de contexte dynamique](#inject-dynamic-context) : Claude Code exécute la commande et remplace la ligne par sa sortie avant que Claude ne voie le contenu de la skill, de sorte que les instructions arrivent avec le diff actuel déjà intégré.
  </Step>

  <Step title="Tester la skill">
    Ouvrez un projet git, apportez une petite modification à n'importe quel fichier, et démarrez Claude Code en exécutant `claude`. Vous pouvez tester la skill de deux façons.

    **Laisser Claude l'invoquer automatiquement** en posant une question qui correspond à la description :

    ```text theme={null}
    What did I change?
    ```

    **Ou l'invoquer directement** avec le nom de la skill :

    ```text theme={null}
    /summarize-changes
    ```

    De l'une ou l'autre façon, Claude devrait répondre avec un court résumé de votre modification et une liste de risques.
  </Step>
</Steps>

<h3 id="where-skills-live">
  Où vivent les skills
</h3>

L'endroit où vous stockez une skill détermine qui peut l'utiliser :

| Localisation | Chemin                                               | S'applique à                                |
| :----------- | :--------------------------------------------------- | :------------------------------------------ |
| Entreprise   | Voir [paramètres gérés](/docs/fr/settings#settings-files) | Tous les utilisateurs de votre organisation |
| Personnel    | `~/.claude/skills/<skill-name>/SKILL.md`             | Tous vos projets                            |
| Projet       | `.claude/skills/<skill-name>/SKILL.md`               | Ce projet uniquement                        |
| Plugin       | `<plugin>/skills/<skill-name>/SKILL.md`              | Où le plugin est activé                     |

Quand les skills partagent le même nom à différents niveaux, l'entreprise remplace le personnel, et le personnel remplace le projet. Une skill à n'importe quel niveau remplace également une skill groupée portant le même nom. Par exemple, une skill `code-review` dans le `.claude/skills/` de votre projet remplace la skill groupée `/code-review`. Les skills de plugin utilisent un espace de noms `plugin-name:skill-name`, donc elles ne peuvent pas entrer en conflit avec d'autres niveaux. Si vous avez des fichiers dans `.claude/commands/`, ils fonctionnent de la même manière, mais si une skill et une commande partagent le même nom, la skill a la priorité.

Les skills se chargent également à partir de répertoires `.claude/skills/` imbriqués en dessous de votre répertoire de travail. Quand Claude lit ou modifie un fichier dans un sous-répertoire, les skills du `.claude/skills/` de ce sous-répertoire deviennent disponibles. Cela permet à un package monorepo de fournir ses propres skills qui s'appliquent quand vous travaillez sur ce package, même si la session a commencé à la racine du référentiel.

Si une skill imbriquée partage un nom avec une autre skill, les deux restent disponibles. Par exemple, avec une skill `deploy` à la racine du projet et une autre dans `apps/web/.claude/skills/` :

* La skill imbriquée apparaît sous un nom qualifié par répertoire, `apps/web:deploy`.
* Sa description indique quel répertoire elle s'applique à.
* Claude choisit la variante qui correspond aux fichiers sur lesquels il travaille.

Taper `/deploy` exécute la skill de la racine du projet. Tapez le nom qualifié `/apps/web:deploy` pour exécuter la variante imbriquée explicitement.

Quand vous ou Claude invoquez le nom non qualifié, la skill de la racine du projet se charge, et Claude Code ajoute une liste des variantes qualifiées par répertoire à son contenu avec une instruction pour invoquer également toute variante dont le répertoire contient les fichiers sur lesquels Claude travaille. Une skill imbriquée s'applique donc toujours au travail dans son répertoire quand seul le nom non qualifié est invoqué. Nécessite Claude Code v2.1.203 ou ultérieur.

Une entrée `<skill-name>` dans les emplacements d'entreprise, personnels ou de projet peut être un lien symbolique vers un répertoire ailleurs sur le disque. Claude Code suit le lien symbolique et lit `SKILL.md` à partir du répertoire cible, et si la même cible est accessible à partir de plusieurs emplacements, Claude Code charge la skill une seule fois. Les skills de plugin gèrent les liens symboliques différemment ; voir [Partager des fichiers dans une marketplace avec des liens symboliques](/docs/fr/plugins-reference#share-files-within-a-marketplace-with-symlinks).

<Note>
  Ajoutez un `.claude-plugin/plugin.json` à un dossier de skill et il se charge comme un [plugin](/docs/fr/plugins-reference#skills-directory-plugins) nommé `<name>@skills-dir`, de sorte qu'il peut regrouper des agents, des hooks et des serveurs MCP. Dans un `.claude/skills/` de projet, cela nécessite d'accepter d'abord la boîte de dialogue de confiance de l'espace de travail.
</Note>

<h4 id="live-change-detection">
  Détection de changement en direct
</h4>

Claude Code surveille les répertoires de skills pour les changements de fichiers. Ajouter, modifier ou supprimer une skill sous `~/.claude/skills/`, le `.claude/skills/` du projet, ou un `.claude/skills/` à l'intérieur d'un répertoire `--add-dir` prend effet dans la session actuelle sans redémarrage. Créer un répertoire de skills de haut niveau qui n'existait pas quand la session a commencé nécessite de redémarrer Claude Code pour que le nouveau répertoire puisse être surveillé.

<Note>
  La détection de changement en direct couvre le texte `SKILL.md` uniquement. Pour un dossier de skill qui est également un [plugin](/docs/fr/plugins-reference#skills-directory-plugins), les modifications apportées à `hooks/`, `.mcp.json`, `agents/` et `output-styles/` nécessitent `/reload-plugins` pour prendre effet.
</Note>

<h4 id="automatic-discovery-from-parent-and-nested-directories">
  Découverte automatique à partir de répertoires parents et imbriqués
</h4>

Les skills de projet se chargent à partir de `.claude/skills/` dans votre répertoire de démarrage et dans chaque répertoire parent jusqu'à la racine du référentiel, de sorte que démarrer Claude dans un sous-répertoire récupère toujours les skills définies à la racine. Quand vous travaillez avec des fichiers dans des sous-répertoires en dessous de votre répertoire de démarrage, Claude Code découvre également les skills à partir des répertoires `.claude/skills/` imbriqués à la demande. Par exemple, si vous modifiez un fichier dans `packages/frontend/`, Claude Code recherche également les skills dans `packages/frontend/.claude/skills/`. Cela supporte les configurations monorepo où les packages ont leurs propres skills.

Chaque skill est un répertoire avec `SKILL.md` comme point d'entrée :

```text theme={null}
my-skill/
├── SKILL.md           # Instructions principales (obligatoire)
├── template.md        # Modèle pour que Claude remplisse
├── examples/
│   └── sample.md      # Exemple de sortie montrant le format attendu
└── scripts/
    └── validate.sh    # Script que Claude peut exécuter
```

Le `SKILL.md` contient les instructions principales et est obligatoire. Les autres fichiers sont optionnels et vous permettent de créer des skills plus puissantes : des modèles pour que Claude remplisse, des exemples de sortie montrant le format attendu, des scripts que Claude peut exécuter, ou une documentation de référence détaillée. Référencez ces fichiers à partir de votre `SKILL.md` pour que Claude sache ce qu'ils contiennent et quand les charger. Voir [Ajouter des fichiers de support](#add-supporting-files) pour plus de détails.

<Note>
  Les fichiers dans `.claude/commands/` fonctionnent toujours et supportent le même [frontmatter](#frontmatter-reference). Les skills sont recommandées puisqu'elles supportent des fonctionnalités supplémentaires comme les fichiers de support.
</Note>

<h4 id="skills-from-additional-directories">
  Skills à partir de répertoires supplémentaires
</h4>

Le drapeau `--add-dir` et la commande `/add-dir` [accordent l'accès aux fichiers](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) plutôt que la découverte de configuration, mais les skills sont une exception : `.claude/skills/` dans un répertoire ajouté est chargé automatiquement. Cette exception s'applique uniquement à `--add-dir` et `/add-dir`. Le paramètre `permissions.additionalDirectories` dans `settings.json` accorde l'accès aux fichiers uniquement et ne charge pas les skills. Voir [Détection de changement en direct](#live-change-detection) pour savoir comment les modifications sont détectées pendant une session.

Les autres configurations `.claude/` comme les commandes et les styles de sortie ne sont pas chargées à partir de répertoires supplémentaires. Voir le [tableau des exceptions](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) pour la liste complète de ce qui est et n'est pas chargé, et les façons recommandées de partager la configuration entre les projets.

<Note>
  Les fichiers CLAUDE.md des répertoires `--add-dir` ne sont pas chargés par défaut. Pour les charger, définissez `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`. Voir [Charger à partir de répertoires supplémentaires](/docs/fr/memory#load-from-additional-directories).
</Note>

<h2 id="configure-skills">
  Configurer les skills
</h2>

Les skills sont configurées via le frontmatter YAML en haut de `SKILL.md` et le contenu markdown qui suit.

<h3 id="types-of-skill-content">
  Types de contenu de skill
</h3>

Les fichiers de skill peuvent contenir n'importe quelles instructions, mais réfléchir à la façon dont vous voulez les invoquer aide à guider ce qu'il faut inclure :

**Le contenu de référence** ajoute des connaissances que Claude applique à votre travail actuel. Conventions, modèles, guides de style, connaissances du domaine. Ce contenu s'exécute en ligne pour que Claude puisse l'utiliser aux côtés du contexte de votre conversation.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Le contenu de tâche** donne à Claude des instructions étape par étape pour une action spécifique, comme les déploiements, les commits ou la génération de code. Ce sont souvent des actions que vous voulez invoquer directement avec `/skill-name` plutôt que de laisser Claude décider quand les exécuter. Ajoutez `disable-model-invocation: true` pour empêcher Claude de la déclencher automatiquement.

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

Votre `SKILL.md` peut contenir n'importe quoi, mais réfléchir à la façon dont vous voulez que la skill soit invoquée (par vous, par Claude, ou les deux) et où vous voulez qu'elle s'exécute (en ligne ou dans un subagent) aide à guider ce qu'il faut inclure. Pour les skills complexes, vous pouvez également [ajouter des fichiers de support](#add-supporting-files) pour garder la skill principale concentrée.

Gardez le corps lui-même concis. Une fois qu'une skill se charge, son contenu [reste dans le contexte d'un tour à l'autre](#skill-content-lifecycle), donc chaque ligne a un coût de token récurrent. Énoncez ce qu'il faut faire plutôt que de narrer comment ou pourquoi, et appliquez le même test de concision que vous feriez pour le [contenu de CLAUDE.md](/docs/fr/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Référence du frontmatter
</h3>

Au-delà du contenu markdown, vous pouvez configurer le comportement de la skill en utilisant les champs du frontmatter YAML entre les marqueurs `---` en haut de votre fichier `SKILL.md` :

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Tous les champs sont optionnels. Seul `description` est recommandé pour que Claude sache quand utiliser la skill.

| Champ                      | Obligatoire | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Non         | Nom d'affichage montré dans les listes de skills. S'il est omis, utilise le nom du répertoire. Voir [Comment une skill obtient son nom de commande](#how-a-skill-gets-its-command-name) pour comprendre comment cela diffère du nom que vous tapez pour invoquer la skill.                                                                                                                                                                                                                                     |
| `description`              | Recommandé  | Ce que fait la skill et quand l'utiliser. Claude utilise ceci pour décider quand appliquer la skill. S'il est omis, utilise le premier paragraphe du contenu markdown. Mettez en avant le cas d'utilisation clé : le texte combiné `description` et `when_to_use` est tronqué à 1 536 caractères dans la liste des skills pour réduire l'utilisation du contexte.                                                                                                                                              |
| `when_to_use`              | Non         | Contexte supplémentaire pour quand Claude devrait invoquer la skill, comme les phrases déclencheurs ou les demandes d'exemple. Ajouté à `description` dans la liste des skills et compte vers le plafond de 1 536 caractères.                                                                                                                                                                                                                                                                                  |
| `argument-hint`            | Non         | Indice affiché lors de l'autocomplétion pour indiquer les arguments attendus. Exemple : `[issue-number]` ou `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                             |
| `arguments`                | Non         | Arguments positionnels nommés pour la [substitution `$name`](#available-string-substitutions) dans le contenu de la skill. Accepte une chaîne séparée par des espaces ou une liste YAML. Les noms correspondent aux positions d'argument dans l'ordre.                                                                                                                                                                                                                                                         |
| `disable-model-invocation` | Non         | Définissez à `true` pour empêcher Claude de charger automatiquement cette skill. Utilisez pour les workflows que vous voulez déclencher manuellement avec `/name`. Empêche également la skill d'être [préchargée dans les subagents](/docs/fr/sub-agents#preload-skills-into-subagents). À partir de v2.1.196, empêche également la skill de s'exécuter quand une [tâche planifiée](/docs/fr/scheduled-tasks) se déclenche avec la skill comme prompt. Par défaut : `false`.                                             |
| `user-invocable`           | Non         | Définissez à `false` pour masquer du menu `/`. Utilisez pour les connaissances de base que les utilisateurs ne devraient pas invoquer directement. Par défaut : `true`.                                                                                                                                                                                                                                                                                                                                        |
| `allowed-tools`            | Non         | Outils que Claude peut utiliser sans demander la permission quand cette skill est active. Accepte une chaîne séparée par des espaces ou une liste YAML.                                                                                                                                                                                                                                                                                                                                                        |
| `disallowed-tools`         | Non         | Outils supprimés du pool d'outils disponibles de Claude tandis que cette skill est active. Utilisez pour les skills autonomes qui ne devraient jamais appeler certains outils, comme `AskUserQuestion` pour une boucle de fond. Accepte une chaîne séparée par des espaces ou une liste YAML. La restriction s'efface quand vous envoyez votre prochain message.                                                                                                                                               |
| `model`                    | Non         | Modèle à utiliser quand cette skill est active. Le remplacement s'applique pour le reste du tour actuel et n'est pas sauvegardé dans les paramètres ; le modèle de session reprend à votre prochain prompt. Accepte les mêmes valeurs que [`/model`](/docs/fr/model-config), ou `inherit` pour garder le modèle actif. Une valeur exclue par la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation n'est pas utilisée et la session garde son modèle actuel. |
| `effort`                   | Non         | [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) quand cette skill est active. Remplace le niveau d'effort de la session. Par défaut : hérite de la session. Options : `low`, `medium`, `high`, `xhigh`, `max` ; les niveaux disponibles dépendent du modèle.                                                                                                                                                                                                                                           |
| `context`                  | Non         | Définissez à `fork` pour exécuter dans un contexte de subagent forké.                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `agent`                    | Non         | Quel type de subagent utiliser quand `context: fork` est défini.                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `hooks`                    | Non         | Hooks limités au cycle de vie de cette skill. Voir [Hooks dans les skills et les agents](/docs/fr/hooks#hooks-in-skills-and-agents) pour le format de configuration.                                                                                                                                                                                                                                                                                                                                                |
| `paths`                    | Non         | Modèles Glob qui limitent quand cette skill est activée. Accepte une chaîne séparée par des virgules ou une liste YAML. Quand défini, Claude charge la skill automatiquement uniquement quand vous travaillez avec des fichiers correspondant aux modèles. Utilise le même format que les [règles spécifiques au chemin](/docs/fr/memory#path-specific-rules).                                                                                                                                                      |
| `shell`                    | Non         | Shell à utiliser pour `` !`command` `` et ` ```! ` blocs dans cette skill. Accepte `bash` (par défaut) ou `powershell`. Définir `powershell` exécute les commandes shell en ligne via PowerShell sur Windows. Nécessite `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`.                                                                                                                                                                                                                                                   |

<h4 id="how-a-skill-gets-its-command-name">
  Comment une skill obtient son nom de commande
</h4>

La commande que vous tapez pour invoquer une skill provient de l'endroit où le fichier de skill se trouve. Le champ frontmatter `name` définit l'étiquette d'affichage montrée dans les listes de skills et, sauf pour un `SKILL.md` à la racine du plugin, ne change pas ce que vous tapez après `/`.

Le tableau ci-dessous montre d'où provient le nom de commande pour chaque disposition :

| Emplacement de la skill                                                                                         | Source du nom de commande                                                                      | Exemple                                                                                                                                            |
| :-------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| Répertoire de skill sous `~/.claude/skills/` ou `.claude/skills/`                                               | Nom du répertoire                                                                              | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                                       |
| [Répertoire `.claude/skills/` imbriqué](#where-skills-live), quand le nom entre en conflit avec une autre skill | Chemin du sous-répertoire relatif au répertoire de travail, puis le nom du répertoire de skill | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                                     |
| Fichier sous `.claude/commands/`                                                                                | Nom du fichier sans extension                                                                  | `.claude/commands/deploy.md` → `/deploy`                                                                                                           |
| Sous-répertoire `skills/` du plugin                                                                             | Nom du répertoire, préfixé par le plugin                                                       | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`                                                                                           |
| `SKILL.md` à la racine du plugin                                                                                | Frontmatter `name`, avec le nom du répertoire du plugin comme secours                          | `my-plugin/SKILL.md` avec `name: review` → `/my-plugin:review`. Voir [Règles de comportement du chemin](/docs/fr/plugins-reference#path-behavior-rules) |

Le cas de la racine du plugin est le seul endroit où `name` définit le nom de commande, car il n'y a pas de répertoire de skill pour le prendre. Si `name` n'est pas défini dans le frontmatter, le nom du répertoire du plugin est utilisé à la place.

<h4 id="available-string-substitutions">
  Substitutions de chaîne disponibles
</h4>

Les skills supportent la substitution de chaîne pour les valeurs dynamiques dans le contenu de la skill :

| Variable                | Description                                                                                                                                                                                                                                                                                                                                                   |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$ARGUMENTS`            | Tous les arguments passés lors de l'invocation de la skill. Si `$ARGUMENTS` n'est pas présent dans le contenu, les arguments sont ajoutés comme `ARGUMENTS: <value>`.                                                                                                                                                                                         |
| `$ARGUMENTS[N]`         | Accédez à un argument spécifique par index basé sur 0, comme `$ARGUMENTS[0]` pour le premier argument.                                                                                                                                                                                                                                                        |
| `$N`                    | Raccourci pour `$ARGUMENTS[N]`, comme `$0` pour le premier argument ou `$1` pour le deuxième.                                                                                                                                                                                                                                                                 |
| `$name`                 | Argument nommé déclaré dans la liste du frontmatter [`arguments`](#frontmatter-reference). Les noms correspondent aux positions dans l'ordre, donc avec `arguments: [issue, branch]` l'espace réservé `$issue` se développe en le premier argument et `$branch` en le deuxième.                                                                               |
| `${CLAUDE_SESSION_ID}`  | L'ID de session actuel. Utile pour la journalisation, la création de fichiers spécifiques à la session, ou la corrélation de la sortie de la skill avec les sessions.                                                                                                                                                                                         |
| `${CLAUDE_EFFORT}`      | Le niveau d'effort actuel : `low`, `medium`, `high`, `xhigh`, ou `max`. Ultracode n'est pas un niveau distinct et est signalé comme `xhigh`. Utilisez ceci pour adapter les instructions de la skill au paramètre d'effort actif.                                                                                                                             |
| `${CLAUDE_SKILL_DIR}`   | Le répertoire contenant le fichier `SKILL.md` de la skill. Pour les skills de plugin, c'est le sous-répertoire de la skill dans le plugin, pas la racine du plugin. Utilisez ceci dans les commandes d'injection bash pour référencer les scripts ou les fichiers groupés avec la skill, indépendamment du répertoire de travail actuel.                      |
| `${CLAUDE_PROJECT_DIR}` | Le répertoire racine du projet. C'est le même chemin que les [hooks](/docs/fr/hooks#reference-scripts-by-path) et les serveurs MCP reçoivent comme `CLAUDE_PROJECT_DIR`. Utilisez ceci pour référencer les scripts ou les fichiers locaux du projet, comme `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, indépendamment de l'endroit où la skill est installée. |

La substitution `${CLAUDE_PROJECT_DIR}` nécessite Claude Code v2.1.196 ou ultérieur. Elle s'applique à la fois au corps de la skill et au frontmatter [`allowed-tools`](#frontmatter-reference), donc une règle de permission comme `Bash(${CLAUDE_PROJECT_DIR}/scripts/lint.sh *)` se résout au même chemin que le corps de la skill utilise.

Les arguments indexés utilisent le guillemettage de style shell, donc enveloppez les valeurs multi-mots entre guillemets pour les passer comme un seul argument. Par exemple, `/my-skill "hello world" second` fait que `$0` se développe en `hello world` et `$1` en `second`. L'espace réservé `$ARGUMENTS` se développe toujours en la chaîne d'argument complète telle que tapée.

Pour inclure un `$` littéral avant un chiffre, `ARGUMENTS`, ou un nom d'argument déclaré, comme `$1.00` en prose, échappez-le avec une barre oblique inverse : `\$1.00`. Une barre oblique inverse avant tout autre `$` est laissée inchangée. Seule une barre oblique inverse directement avant le token l'échappe. Une barre oblique inverse doublée comme `\\$1` laisse les deux barres obliques inverses en place, et `$1` se développe toujours en la valeur de l'argument.

**Exemple utilisant les substitutions :**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Ajouter des fichiers de support
</h3>

Les skills peuvent inclure plusieurs fichiers dans leur répertoire. Cela garde `SKILL.md` concentré sur l'essentiel tout en permettant à Claude d'accéder au matériel de référence détaillé uniquement quand c'est nécessaire. Les grandes docs de référence, les spécifications d'API, ou les collections d'exemples n'ont pas besoin de se charger dans le contexte à chaque fois que la skill s'exécute.

```text theme={null}
my-skill/
├── SKILL.md (obligatoire - aperçu et navigation)
├── reference.md (docs API détaillées - chargées quand nécessaire)
├── examples.md (exemples d'utilisation - chargés quand nécessaire)
└── scripts/
    └── helper.py (script utilitaire - exécuté, pas chargé)
```

Référencez les fichiers de support à partir de `SKILL.md` pour que Claude sache ce que chaque fichier contient et quand le charger :

```markdown theme={null}
## Ressources supplémentaires

- Pour les détails complets de l'API, voir [reference.md](reference.md)
- Pour les exemples d'utilisation, voir [examples.md](examples.md)
```

<Tip>Gardez `SKILL.md` sous 500 lignes. Déplacez le matériel de référence détaillé vers des fichiers séparés.</Tip>

<h3 id="control-who-invokes-a-skill">
  Contrôler qui invoque une skill
</h3>

Par défaut, vous et Claude pouvez tous les deux invoquer n'importe quelle skill. Vous pouvez taper `/skill-name` pour l'invoquer directement, et Claude peut la charger automatiquement quand c'est pertinent pour votre conversation. Deux champs du frontmatter vous permettent de restreindre ceci :

* **`disable-model-invocation: true`** : Seul vous pouvez invoquer la skill. Utilisez ceci pour les workflows avec des effets secondaires ou que vous voulez contrôler le timing, comme `/commit`, `/deploy`, ou `/send-slack-message`. Vous ne voulez pas que Claude décide de déployer parce que votre code semble prêt.

* **`user-invocable: false`** : Seul Claude peut invoquer la skill. Utilisez ceci pour les connaissances de base qui ne sont pas actionnables comme une commande. Une skill `legacy-system-context` explique comment fonctionne un ancien système. Claude devrait le savoir quand c'est pertinent, mais `/legacy-system-context` n'est pas une action significative pour les utilisateurs.

Cet exemple crée une skill de déploiement que seul vous pouvez déclencher. Si vous définissez `disable-model-invocation: true`, Claude ne peut pas exécuter la skill automatiquement :

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Voici comment les deux champs affectent l'invocation et le chargement du contexte :

| Frontmatter                      | Vous pouvez invoquer | Claude peut invoquer | Quand chargé dans le contexte                                                       |
| :------------------------------- | :------------------- | :------------------- | :---------------------------------------------------------------------------------- |
| (par défaut)                     | Oui                  | Oui                  | Description toujours dans le contexte, la skill complète se charge quand invoquée   |
| `disable-model-invocation: true` | Oui                  | Non                  | Description pas dans le contexte, la skill complète se charge quand vous l'invoquez |
| `user-invocable: false`          | Non                  | Oui                  | Description toujours dans le contexte, la skill complète se charge quand invoquée   |

<Note>
  Dans une session régulière, les descriptions de skills sont chargées dans le contexte pour que Claude sache ce qui est disponible, mais le contenu complet de la skill ne se charge que quand elle est invoquée. [Les subagents avec des skills préchargées](/docs/fr/sub-agents#preload-skills-into-subagents) fonctionnent différemment : le contenu complet de la skill est injecté au démarrage.
</Note>

<h3 id="skill-content-lifecycle">
  Cycle de vie du contenu de la skill
</h3>

Quand vous ou Claude invoquez une skill, le contenu `SKILL.md` rendu entre dans la conversation comme un seul message et y reste pour le reste de la session. Claude Code ne relit pas le fichier de skill aux tours suivants, donc écrivez les directives qui devraient s'appliquer tout au long d'une tâche comme des instructions permanentes plutôt que des étapes ponctuelles.

Quand Claude réinvoque une skill dont le contenu rendu est identique à la copie déjà dans le contexte, Claude Code ajoute une courte note que la skill est déjà chargée plutôt qu'une deuxième copie du contenu. Quand le contenu rendu diffère, parce que les arguments ont changé ou qu'une commande de [contexte dynamique](#inject-dynamic-context) a produit une nouvelle sortie, Claude Code ajoute le contenu complet à nouveau. Avant v2.1.202, chaque réinvocation ajoutait une autre copie complète des instructions de la skill.

[L'auto-compaction](/docs/fr/how-claude-code-works#when-context-fills-up) porte les skills invoquées en avant dans un budget de tokens. Quand la conversation est résumée pour libérer du contexte, Claude Code réattache l'invocation la plus récente de chaque skill après le résumé, en gardant les premiers 5 000 tokens de chacune. Les skills réattachées partagent un budget combiné de 25 000 tokens. Claude Code remplit ce budget en commençant par la skill la plus récemment invoquée, donc les skills plus anciennes peuvent être entièrement supprimées après la compaction si vous en avez invoqué beaucoup dans une session.

Si une skill semble cesser d'influencer le comportement après la première réponse, le contenu est généralement toujours présent et le modèle choisit d'autres outils ou approches. Renforcez la `description` et les instructions de la skill pour que le modèle continue à la préférer, ou utilisez [hooks](/docs/fr/hooks) pour appliquer le comportement de manière déterministe. Si la skill est grande ou vous avez invoqué plusieurs autres après elle, réinvoquez-la après la compaction pour restaurer le contenu complet.

<h3 id="pre-approve-tools-for-a-skill">
  Pré-approuver les outils pour une skill
</h3>

Le champ `allowed-tools` accorde la permission pour les outils listés tandis que la skill est active, pour que Claude puisse les utiliser sans vous demander l'approbation par utilisation. Il ne restreint pas quels outils sont disponibles : chaque outil reste appelable, et vos [paramètres de permission](/docs/fr/permissions) gouvernent toujours les outils qui ne sont pas listés.

Pour les skills vérifiées dans le répertoire `.claude/skills/` d'un projet, `allowed-tools` prend effet après que vous acceptiez la boîte de dialogue de confiance de l'espace de travail pour ce dossier, de la même manière que les règles de permission dans `.claude/settings.json`. Examinez les skills du projet avant de faire confiance à un référentiel, car une skill peut s'accorder un accès large aux outils.

Cette skill permet à Claude d'exécuter les commandes git sans approbation par utilisation chaque fois que vous l'invoquez :

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Pour supprimer les outils du pool d'outils disponibles de Claude tandis qu'une skill est active, listez-les dans `disallowed-tools` dans le frontmatter de la skill. La restriction s'efface quand vous envoyez votre prochain message. Pour bloquer les outils dans toutes les skills et tous les prompts, ajoutez des règles de refus dans vos [paramètres de permission](/docs/fr/permissions).

<h3 id="pass-arguments-to-skills">
  Passer des arguments aux skills
</h3>

Vous et Claude pouvez tous les deux passer des arguments lors de l'invocation d'une skill. Les arguments sont disponibles via l'espace réservé `$ARGUMENTS`.

Cette skill corrige un problème GitHub par numéro. L'espace réservé `$ARGUMENTS` est remplacé par tout ce qui suit le nom de la skill :

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Quand vous exécutez `/fix-issue 123`, Claude reçoit « Fix GitHub issue 123 following our coding standards... »

Si vous invoquez une skill avec des arguments mais que la skill n'inclut pas `$ARGUMENTS`, Claude Code ajoute `ARGUMENTS: <your input>` à la fin du contenu de la skill pour que Claude voie toujours ce que vous avez tapé.

Vous pouvez également empiler plusieurs skills au début d'un message. À partir de v2.1.199, taper `/code-review /fix-issue 123` charge les deux skills et passe le texte final `123` comme `$ARGUMENTS` à chacune d'elles. Dans les versions antérieures, seule la première skill se chargeait et recevait `/fix-issue 123` comme texte d'argument littéral.

Claude Code développe la première skill plus jusqu'à cinq autres empilées après elle. L'expansion s'arrête au premier token qui n'est pas une skill invocable en ligne par l'utilisateur, donc une skill qui s'exécute comme un [subagent forké](#run-skills-in-a-subagent) ou une dont les arguments peuvent eux-mêmes commencer par une commande slash, comme `/loop`, arrête également l'exécution là ; ce token et tout ce qui suit deviennent le texte d'argument pour chaque skill développée.

Pour accéder aux arguments individuels par position, utilisez `$ARGUMENTS[N]` ou le raccourci plus court `$N` :

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Exécuter `/migrate-component SearchBar React Vue` remplace `$ARGUMENTS[0]` par `SearchBar`, `$ARGUMENTS[1]` par `React`, et `$ARGUMENTS[2]` par `Vue`. La même skill utilisant le raccourci `$N` :

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Modèles avancés
</h2>

<h3 id="inject-dynamic-context">
  Injecter du contexte dynamique
</h3>

La syntaxe `` !`<command>` `` exécute les commandes shell avant que le contenu de la skill soit envoyé à Claude. La sortie de la commande remplace l'espace réservé, donc Claude reçoit les données réelles, pas la commande elle-même.

Cette skill résume une pull request en récupérant les données de PR en direct avec le CLI GitHub. Les commandes `` !`gh pr diff` `` et autres s'exécutent d'abord, et leur sortie est insérée dans le prompt :

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

Quand cette skill s'exécute :

1. Chaque `` !`<command>` `` s'exécute immédiatement (avant que Claude ne voie quoi que ce soit)
2. La sortie remplace l'espace réservé dans le contenu de la skill
3. Claude reçoit le prompt complètement rendu avec les données réelles de PR

C'est du prétraitement, pas quelque chose que Claude exécute. Claude ne voit que le résultat final.

La substitution s'exécute une seule fois sur le fichier original. La sortie de la commande est insérée en tant que texte brut et n'est pas réanalysée pour d'autres espaces réservés `` !`<command>` ``, donc une commande ne peut pas émettre un espace réservé pour qu'une passe ultérieure l'étende.

La forme en ligne n'est reconnue que quand `!` apparaît au début d'une ligne ou immédiatement après un espace blanc. Si `!` suit un autre caractère, comme dans `` KEY=!`cmd` ``, l'espace réservé est laissé en tant que texte littéral et la commande ne s'exécute pas.

Pour les commandes multi-lignes, utilisez un bloc de code clôturé ouvert avec ` ```! ` au lieu de la forme en ligne :

````markdown theme={null}
## Environment
```!
node --version
npm --version
git status --short
```
````

Pour désactiver ce comportement pour les skills et les commandes personnalisées des sources utilisateur, projet, plugin ou [répertoire supplémentaire](#skills-from-additional-directories), définissez `"disableSkillShellExecution": true` dans [paramètres](/docs/fr/settings). Chaque commande est remplacée par `[shell command execution disabled by policy]` au lieu d'être exécutée. Les skills groupées et gérées ne sont pas affectées. Ce paramètre est très utile dans les [paramètres gérés](/docs/fr/permissions#managed-settings), où les utilisateurs ne peuvent pas le remplacer.

<Tip>
  Pour demander un raisonnement plus approfondi quand une skill s'exécute, incluez `ultrathink` n'importe où dans le contenu de la skill. Voir [Utiliser ultrathink pour un raisonnement approfondi ponctuel](/docs/fr/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h3 id="run-skills-in-a-subagent">
  Exécuter les skills dans un subagent
</h3>

Ajoutez `context: fork` à votre frontmatter quand vous voulez qu'une skill s'exécute en isolation. Le contenu de la skill devient le prompt qui pilote le subagent. Il n'aura pas accès à votre historique de conversation.

<Warning>
  `context: fork` n'a de sens que pour les skills avec des instructions explicites. Si votre skill contient des directives comme « utiliser ces conventions d'API » sans une tâche, le subagent reçoit les directives mais pas de prompt actionnable, et retourne sans sortie significative.
</Warning>

Les skills et les [subagents](/docs/fr/sub-agents) fonctionnent ensemble dans deux directions :

| Approche                     | Prompt système             | Tâche                           | Charge également                                  |
| :--------------------------- | :------------------------- | :------------------------------ | :------------------------------------------------ |
| Skill avec `context: fork`   | Du type d'agent            | Contenu de SKILL.md             | CLAUDE.md, sauf quand l'agent est Explore ou Plan |
| Subagent avec champ `skills` | Corps markdown du subagent | Message de délégation de Claude | Skills préchargées + CLAUDE.md                    |

Avec `context: fork`, vous écrivez la tâche dans votre skill et choisissez un type d'agent pour l'exécuter. Les agents intégrés Explore et Plan [ignorent CLAUDE.md et git status](/docs/fr/sub-agents#what-loads-at-startup) pour garder leur contexte petit, donc une skill forquée utilisant `agent: Explore` ne voit que le contenu de SKILL.md et le prompt système propre de l'agent. Pour l'inverse, où vous définissez un subagent personnalisé qui utilise les skills comme matériel de référence, voir [Subagents](/docs/fr/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Exemple : Skill de recherche utilisant l'agent Explore
</h4>

Cette skill exécute la recherche dans un agent Explore forké. Le contenu de la skill devient la tâche, et l'agent fournit des outils en lecture seule optimisés pour l'exploration de la base de code :

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Quand cette skill s'exécute :

1. Un nouveau contexte isolé est créé
2. Le subagent reçoit le contenu de la skill comme son prompt (« Research \$ARGUMENTS thoroughly... »)
3. Le champ `agent` détermine l'environnement d'exécution (modèle, outils et permissions)
4. Les résultats sont résumés et retournés à votre conversation principale

Le champ `agent` spécifie quelle configuration de subagent utiliser. Les options incluent les agents intégrés (`Explore`, `Plan`, `general-purpose`) ou n'importe quel subagent personnalisé de `.claude/agents/`. S'il est omis, utilise `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Restreindre l'accès aux skills de Claude
</h3>

Par défaut, Claude peut invoquer n'importe quelle skill qui n'a pas `disable-model-invocation: true` défini. Les skills qui définissent `allowed-tools` accordent à Claude l'accès à ces outils sans approbation par utilisation quand la skill est active. Vos [paramètres de permission](/docs/fr/permissions) gouvernent toujours le comportement d'approbation de base pour tous les autres outils. Quelques commandes intégrées sont également disponibles via l'outil Skill, notamment `/init`, `/review` et `/security-review`. Les autres commandes intégrées comme `/compact` ne le sont pas.

Trois façons de contrôler quelles skills Claude peut invoquer :

**Désactiver toutes les skills** en refusant l'outil Skill dans `/permissions` :

```text theme={null}
# Add to deny rules:
Skill
```

**Autoriser ou refuser des skills spécifiques** en utilisant les [règles de permission](/docs/fr/permissions) :

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Syntaxe de permission : `Skill(name)` pour une correspondance exacte, `Skill(name *)` pour une correspondance de préfixe avec n'importe quels arguments.

**Masquer les skills individuelles** en ajoutant `disable-model-invocation: true` à leur frontmatter. Cela supprime la skill du contexte de Claude entièrement.

<Note>
  Le champ `user-invocable` contrôle uniquement la visibilité du menu, pas l'accès à l'outil Skill. Utilisez `disable-model-invocation: true` pour bloquer l'invocation programmatique.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Remplacer la visibilité des skills à partir des paramètres
</h3>

Le paramètre `skillOverrides` contrôle la visibilité des skills à partir de vos [paramètres](/docs/fr/settings) au lieu du frontmatter de la skill elle-même. Utilisez-le pour les skills dont le SKILL.md vous ne voulez pas modifier, comme celles archivées dans un référentiel de projet partagé ou fournies par un serveur MCP. Le menu `/skills` l'écrit pour vous : mettez en surbrillance une skill et appuyez sur `Space` pour parcourir les états, puis `Enter` pour enregistrer dans `.claude/settings.local.json`.

Chaque clé est un nom de skill et chaque valeur est l'un des quatre états :

| Valeur                  | Listée à Claude    | Dans le menu `/` |
| :---------------------- | :----------------- | :--------------- |
| `"on"`                  | Nom et description | Oui              |
| `"name-only"`           | Nom uniquement     | Oui              |
| `"user-invocable-only"` | Masqué             | Oui              |
| `"off"`                 | Masqué             | Masqué           |

À partir de la v2.1.199, `"off"` masque également la skill des listes de commandes annoncées aux clients [Remote Control](/docs/fr/remote-control) et aux appelants [Agent SDK](/docs/fr/agent-sdk/slash-commands), pas seulement le menu terminal `/`. Invoquer une skill masquée par son nom complet retourne toujours l'erreur `skillOverrides` au lieu de l'exécuter.

Une skill absente de `skillOverrides` est traitée comme `"on"`. L'exemple ci-dessous réduit une skill à son nom et désactive une autre entièrement :

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Les skills de plugin ne sont pas affectées par `skillOverrides`. Gérez-les via `/plugin` à la place.

<h2 id="evaluate-and-iterate-on-a-skill">
  Évaluer et itérer sur une skill
</h2>

Voir une skill se déclencher vous indique que Claude l'a trouvée, pas qu'elle a fait ce que vous aviez l'intention. Pour savoir qu'une skill fonctionne, mesurez deux choses séparément : si Claude l'invoque sur les prompts qu'elle devrait, et si la sortie correspond à ce que vous attendez quand elle le fait.

La vérification des deux est une comparaison de base. Collectez quelques prompts réalistes, exécutez chacun dans une session fraîche avec la skill disponible et à nouveau avec elle [désactivée](#override-skill-visibility-from-settings), et comparez les résultats. Une session fraîche est importante car le contexte restant de la création de la skill masquera les lacunes dans les instructions écrites.

<h3 id="run-evals-with-skill-creator">
  Exécuter les evals avec skill-creator
</h3>

Le [plugin `skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatise la boucle de comparaison à l'intérieur de Claude Code. Installez-le à partir de la place de marché officielle :

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Si Claude Code signale que le plugin n'est pas trouvé dans aucune place de marché, votre place de marché est soit manquante soit obsolète. Exécutez `/plugin marketplace update claude-plugins-official` pour l'actualiser, ou `/plugin marketplace add anthropics/claude-plugins-official` si vous ne l'avez pas ajoutée avant. Puis réessayez l'installation.

Après l'installation, exécutez `/reload-plugins` pour rendre les skills du plugin disponibles dans la session actuelle. Ensuite, demandez à Claude d'évaluer une skill existante, par exemple `evaluate my summarize-changes skill with skill-creator`. Le plugin vous guide à travers l'écriture de cas de test et exécute la boucle :

* **Cas de test** : stocke les prompts, les fichiers d'entrée et le comportement attendu dans `evals/evals.json` à l'intérieur du répertoire de la skill
* **Exécutions isolées** : génère un [subagent](/docs/fr/sub-agents) par cas de test pour que chaque exécution commence avec un contexte propre, et enregistre le nombre de tokens et la durée
* **Notation** : vérifie chaque assertion par rapport à la sortie et écrit réussi ou échoué avec des preuves dans `grading.json`
* **Benchmark** : agrège le taux de réussite, le temps et les tokens pour avec-skill par rapport à sans-skill dans `benchmark.json` pour que vous puissiez comparer l'amélioration du taux de réussite par rapport à la surcharge de tokens et de temps
* **Comparaison de version** : exécute un A/B en aveugle entre deux versions de la skill pour que vous puissiez confirmer qu'une modification est une amélioration avant de la valider
* **Ajustement de description** : génère les prompts should-trigger et should-not-trigger, mesure le taux de réussite, et propose des modifications de description quand la skill s'active sur les mauvaises demandes
* **Visionneuse d'examen** : ouvre un rapport HTML où vous inspectez chaque sortie et enregistrez les commentaires qualitatifs que l'itération suivante lit

Pour le format du fichier eval et le flux de travail d'itération complet, voir [Évaluer la qualité de la sortie de la skill](https://agentskills.io/skill-creation/evaluating-skills) sur agentskills.io. Pour le contexte sur le benchmark et les modes de comparaison, voir l'[annonce de skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Partager les skills
</h2>

Les skills peuvent être distribuées à différentes portées selon votre audience :

* **Skills de projet** : Validez `.claude/skills/` dans le contrôle de version
* **Plugins** : Créez un répertoire `skills/` dans votre [plugin](/docs/fr/plugins)
* **Gérées** : Déployez à l'échelle de l'organisation via les [paramètres gérés](/docs/fr/settings#settings-files)

<h3 id="generate-visual-output">
  Générer une sortie visuelle
</h3>

Les skills peuvent grouper et exécuter des scripts dans n'importe quel langage, donnant à Claude des capacités au-delà de ce qui est possible dans un seul prompt. Un modèle puissant est la génération de sortie visuelle : des fichiers HTML interactifs qui s'ouvrent dans votre navigateur pour explorer les données, déboguer ou créer des rapports.

Cet exemple crée un explorateur de base de code : une vue d'arbre interactive où vous pouvez développer et réduire les répertoires, voir les tailles de fichiers en un coup d'œil, et identifier les types de fichiers par couleur.

Créez le répertoire Skill :

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Enregistrez ceci dans `~/.claude/skills/codebase-visualizer/SKILL.md`. La description indique à Claude quand activer cette Skill, et les instructions indiquent à Claude d'exécuter le script groupé. Le chemin du script utilise [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) pour qu'il se résolve correctement que la skill soit installée au niveau personnel, projet ou plugin :

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Enregistrez ceci dans `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Ce script analyse une arborescence de répertoires et génère un fichier HTML autonome avec :

* Une **barre latérale de résumé** montrant le nombre de fichiers, le nombre de répertoires, la taille totale et le nombre de types de fichiers
* Un **graphique en barres** décomposant la base de code par type de fichier (top 8 par taille)
* Un **arbre réductible** où vous pouvez développer et réduire les répertoires, avec des indicateurs de type de fichier codés par couleur

Le script nécessite Python 3 mais utilise uniquement les bibliothèques intégrées, donc il n'y a pas de packages à installer :

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Pour tester, ouvrez Claude Code dans n'importe quel projet et demandez « Visualize this codebase. » Claude exécute le script, génère `codebase-map.html`, et l'ouvre dans votre navigateur.

Ce modèle fonctionne pour n'importe quelle sortie visuelle : graphiques de dépendances, rapports de couverture de test, documentation d'API, ou visualisations de schéma de base de données. Le script groupé fait le gros du travail tandis que Claude gère l'orchestration.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="skill-not-triggering">
  Skill ne se déclenche pas
</h3>

Si Claude n'utilise pas votre skill quand attendu :

1. Vérifiez que la description inclut les mots-clés que les utilisateurs diraient naturellement
2. Vérifiez que la skill apparaît dans « What skills are available? »
3. Essayez de reformuler votre demande pour correspondre plus étroitement à la description
4. Invoquez-la directement avec `/skill-name` si la skill est invocable par l'utilisateur

Si le YAML du frontmatter est malformé, Claude Code charge le corps de la skill avec des métadonnées vides, donc `/skill-name` fonctionne toujours mais Claude n'a pas de `description` pour correspondre. Exécutez avec `--debug` pour voir l'erreur d'analyse.

<h3 id="skill-triggers-too-often">
  Skill se déclenche trop souvent
</h3>

Si Claude utilise votre skill quand vous ne le voulez pas :

1. Rendez la description plus spécifique
2. Ajoutez `disable-model-invocation: true` si vous voulez uniquement l'invocation manuelle

<h3 id="skill-descriptions-are-cut-short">
  Les descriptions de skills sont coupées court
</h3>

Claude Code charge une liste de noms de skills et de descriptions dans le contexte pour que Claude sache ce qui est disponible. La liste contient toujours tous les noms de skills, mais si vous avez beaucoup de skills, Claude Code raccourcit les descriptions pour tenir dans le budget de caractères de la liste, ce qui peut supprimer les mots-clés dont Claude a besoin pour correspondre à votre demande. Le budget s'adapte à 1 % de la fenêtre de contexte du modèle. Quand la liste déborde, Claude Code supprime les descriptions en commençant par les skills que vous invoquez le moins, de sorte que les skills que vous utilisez le plus conservent leur texte complet.

Exécutez `/doctor` pour une estimation du coût contextuel de la liste et de ses plus grands contributeurs. Quand la liste dépasse son budget, Claude Code écrit également un avertissement dans le journal de débogage, visible avec [`--debug`](/docs/fr/cli-reference#cli-flags).

La ligne Skills dans `/context` rapporte la taille de la liste après l'application du budget, de sorte qu'elle correspond à ce que le modèle reçoit. Avant la v2.1.196, la ligne comptait le texte complet de chaque description et pouvait afficher une valeur plusieurs fois plus grande que le budget configuré.

Pour augmenter le budget, définissez le paramètre [`skillListingBudgetFraction`](/docs/fr/settings#available-settings) (par exemple `0.02` = 2 %) ou la variable d'environnement `SLASH_COMMAND_TOOL_CHAR_BUDGET` à un nombre de caractères fixe. Pour libérer du budget pour d'autres skills, définissez les entrées de faible priorité sur `"name-only"` dans [`skillOverrides`](#override-skill-visibility-from-settings) afin qu'elles s'affichent sans description. Vous pouvez également réduire le texte `description` et `when_to_use` à la source : mettez en avant le cas d'utilisation clé, puisque le texte combiné de chaque entrée est limité à 1 536 caractères indépendamment du budget. Le plafond est configurable avec [`skillListingMaxDescChars`](/docs/fr/settings#available-settings).

<h2 id="related-resources">
  Ressources connexes
</h2>

* **[Déboguer votre configuration](/docs/fr/debug-your-config)** : diagnostiquer pourquoi une skill n'apparaît pas ou ne se déclenche pas
* **[Évaluer la qualité de la sortie de la skill](https://agentskills.io/skill-creation/evaluating-skills)** : le format du fichier eval et le flux de travail d'itération sur agentskills.io
* **[Meilleures pratiques de création de skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)** : conseils de rédaction qui s'appliquent à tous les produits Claude
* **[Subagents](/docs/fr/sub-agents)** : déléguer les tâches à des agents spécialisés
* **[Plugins](/docs/fr/plugins)** : empaqueter et distribuer les skills avec d'autres extensions
* **[Hooks](/docs/fr/hooks)** : automatiser les workflows autour des événements d'outils
* **[Memory](/docs/fr/memory)** : gérer les fichiers CLAUDE.md pour le contexte persistant
* **[Commands](/docs/fr/commands)** : référence pour les commandes intégrées et les skills groupées
* **[Permissions](/docs/fr/permissions)** : contrôler l'accès aux outils et aux skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)** : les skills de projet validées dans un repo se chargent également lorsque ce repo est utilisé dans un canal Claude Tag
