> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Étendre Claude avec des skills

> Créez, gérez et partagez des skills pour étendre les capacités de Claude dans Claude Code. Inclut les commandes personnalisées et les skills groupées.

Les skills étendent ce que Claude peut faire. Créez un fichier `SKILL.md` avec des instructions, et Claude l'ajoute à sa boîte à outils. Claude utilise les skills quand c'est pertinent, ou vous pouvez en invoquer une directement avec `/skill-name`.

Créez une skill quand vous collez constamment les mêmes instructions, liste de contrôle ou procédure multi-étapes dans le chat, ou quand une section de CLAUDE.md s'est transformée en procédure plutôt qu'en fait. Contrairement au contenu de CLAUDE.md, le corps d'une skill ne se charge que lorsqu'elle est utilisée, donc le matériel de référence long ne coûte presque rien jusqu'à ce que vous en ayez besoin.

<Note>
  Pour les commandes intégrées comme `/help` et `/compact`, et les skills groupées comme `/debug` et `/code-review`, consultez la [référence des commandes](/docs/fr/commands).

  **Les commandes personnalisées ont été fusionnées dans les skills.** Un fichier à `.claude/commands/deploy.md` et une skill à `.claude/skills/deploy/SKILL.md` créent tous deux `/deploy` et fonctionnent de la même manière. Vos fichiers `.claude/commands/` existants continuent de fonctionner. Les skills ajoutent des fonctionnalités optionnelles : un répertoire pour les fichiers de support, un frontmatter pour [contrôler si vous ou Claude les invoquez](#control-who-invokes-a-skill), et la capacité pour Claude de les charger automatiquement quand c'est pertinent.
</Note>

Les skills Claude Code suivent la norme ouverte [Agent Skills](https://agentskills.io), qui fonctionne sur plusieurs outils d'IA. Claude Code étend la norme avec des fonctionnalités supplémentaires comme [le contrôle d'invocation](#control-who-invokes-a-skill), [l'exécution de sous-agents](#run-skills-in-a-subagent), et [l'injection de contexte dynamique](#inject-dynamic-context). Consultez [Utiliser le frontmatter de skill en dehors de Claude Code](#using-skill-frontmatter-outside-claude-code) pour savoir quels champs frontmatter font partie de la norme et lesquels sont des extensions Claude Code.

<h2 id="bundled-skills">
  Compétences groupées
</h2>

Claude Code inclut un ensemble de compétences groupées, telles que `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` et `/claude-api`. Les compétences groupées sont basées sur des invites : elles donnent à Claude des instructions détaillées et lui permettent d'orchestrer le travail en utilisant ses outils. La plupart des commandes intégrées exécutent plutôt une logique fixe directement.

Vous invoquez une compétence groupée de la même manière que n'importe quelle autre compétence, en tapant `/` suivi du nom de la compétence. Claude invoque automatiquement certaines compétences groupées lorsqu'elles sont pertinentes ; d'autres, notamment `/verify`, ne s'exécutent que lorsque vous les invoquez, ce qui vous permet de contrôler quand ces vérifications plus longues consomment du temps et des tokens.

La plupart des compétences groupées sont disponibles dans chaque session. Quelques-unes dépendent d'une fonctionnalité spécifique : `/workflow-authoring`, par exemple, n'est disponible que lorsque les [flux de travail dynamiques](/docs/fr/workflows) sont activés.

Pour désactiver les compétences groupées, utilisez le paramètre [`disableBundledSkills`](/docs/fr/settings-reference#disablebundledskills).

<Note>
  La vérification de configuration [`/doctor`](/docs/fr/commands#all-commands) reste saisissable lorsque `disableBundledSkills` est activé, dans Claude Code v2.1.205 et versions ultérieures. Pour la masquer, définissez la variable d'environnement `DISABLE_DOCTOR_COMMAND` ou une entrée [`skillOverrides`](#override-skill-visibility-from-settings) de `"doctor": "off"`. Avant v2.1.205, `/doctor` était une commande intégrée plutôt qu'une compétence groupée.
</Note>

Les compétences groupées sont listées aux côtés des commandes intégrées dans la [référence des commandes](/docs/fr/commands), marquées **Skill** dans la colonne Purpose.

<h3 id="run-and-verify-your-app">
  Exécuter et vérifier votre application
</h3>

Trois compétences groupées travaillent ensemble pour lancer votre application et confirmer les modifications par rapport à l'application en cours d'exécution au lieu de simplement des tests :

| Skill                  | Purpose                                                                                                                                                           |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Lancez et pilotez votre application pour voir une modification fonctionner                                                                                        |
| `/verify`              | Construisez et exécutez votre application pour confirmer qu'une modification de code fait ce qu'elle devrait, sans revenir aux tests ou aux vérifications de type |
| `/run-skill-generator` | Enseignez à `/run` et `/verify` comment construire et lancer votre projet                                                                                         |

`/run` et `/verify` fonctionnent sans configuration. Ils déduisent le lancement de votre type de projet (CLI, serveur, TUI, piloté par navigateur) et de ce qui se trouve dans votre README, `package.json` ou `Makefile`. Cette déduction devient peu fiable pour les projets qui nécessitent quelque chose au-delà d'un lancement standard : une base de données, un fichier env, une session graphique, une construction multi-étapes.

`/run-skill-generator` enregistre plutôt la recette. Il fait fonctionner votre application à partir d'un environnement propre, capture ce qui a fonctionné (les commandes d'installation, les variables d'environnement, le script de lancement) et l'enregistre en tant que compétence par projet à `.claude/skills/run-<name>/`. Après cela, `/run`, `/verify` et tout autre agent du référentiel suivent la recette enregistrée au lieu de la redécouvrir. Exécutez `/run-skill-generator` une fois par projet, et à nouveau si le processus de construction ou de lancement change.

`/verify` peut également enregistrer sa propre recette. Lorsqu'elle doit construire et piloter votre application sans recette enregistrée, elle écrit ce qui a fonctionné dans `.claude/skills/verify/SKILL.md` à la racine du référentiel, ou dans le répertoire du package touché dans un monorepo, afin que les exécutions ultérieures et les autres agents suivent les mêmes étapes. À la racine du référentiel, la compétence enregistrée remplace la compétence groupée `/verify`. Cela nécessite Claude Code v2.1.200 ou version ultérieure.

Claude modifie le fichier enregistré uniquement lorsqu'il a mal dirigé une exécution, comme une commande qui a échoué ou une étape manquante, afin que vous puissiez valider le fichier sans diffs par session. Avant v2.1.205, la compétence groupée a dit à Claude de plier tout ce qu'une exécution a appris, ce qui a causé des conflits de fusion fréquents.

<h2 id="getting-started">
  Démarrage
</h2>

<h3 id="create-your-first-skill">
  Créer votre première compétence
</h3>

Cet exemple crée une compétence qui résume les modifications non validées dans votre référentiel git et signale tout ce qui est risqué. Il extrait le diff en direct dans l'invite avant que Claude ne le lise, de sorte que la réponse est ancrée dans votre arborescence de travail réelle plutôt que dans ce que Claude peut deviner à partir des fichiers ouverts. Claude charge la compétence automatiquement lorsque vous posez des questions sur vos modifications, ou vous pouvez l'invoquer directement avec `/summarize-changes`.

<Steps>
  <Step title="Créer le répertoire de la compétence">
    Créez un répertoire pour la compétence dans votre dossier de compétences personnelles. Les compétences personnelles sont disponibles dans tous vos projets.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Écrire SKILL.md">
    Chaque compétence a besoin d'un fichier `SKILL.md` avec deux parties : un frontmatter YAML entre les marqueurs `---` qui indique à Claude quand utiliser la compétence, et du contenu markdown avec les instructions que Claude suit lorsque la compétence s'exécute. Le nom du répertoire devient la commande que vous tapez, et la `description` aide Claude à décider quand charger la compétence automatiquement.

    Enregistrez ceci dans `~/.claude/skills/summarize-changes/SKILL.md` :

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    La ligne `` !`git diff HEAD` `` utilise l'[injection de contexte dynamique](#inject-dynamic-context) : Claude Code exécute la commande et remplace la ligne par sa sortie avant que Claude ne voie le contenu de la compétence, de sorte que les instructions arrivent avec le diff actuel déjà intégré.
  </Step>

  <Step title="Tester la compétence">
    Ouvrez un projet git, apportez une petite modification à n'importe quel fichier, et démarrez Claude Code en exécutant `claude`. Vous pouvez tester la compétence de deux façons.

    **Laissez Claude l'invoquer automatiquement** en posant une question qui correspond à la description :

    ```text theme={null}
    What did I change?
    ```

    **Ou invoquez-la directement** avec le nom de la compétence :

    ```text theme={null}
    /summarize-changes
    ```

    De l'une ou l'autre façon, Claude devrait répondre avec un court résumé de votre modification et une liste de risques.
  </Step>
</Steps>

<h2 id="where-skills-live">
  Choisir où les skills se chargent
</h2>

L'endroit où vous enregistrez une skill détermine quelles sessions la chargent. Enregistrez-la dans votre répertoire personnel pour l'obtenir dans chaque projet, validez-la dans un référentiel pour la partager avec tous ceux qui y travaillent, ou distribuez-la via un plugin ou des paramètres gérés pour atteindre toute une équipe.

| Emplacement               | Chemin                                                                                                                     | Se charge dans                                                                                                                                                                                                                  |
| :------------------------ | :------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Entreprise                | `.claude/skills/<skill-name>/SKILL.md` dans le [répertoire des paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms) | Tous les utilisateurs sur les machines où votre organisation la déploie                                                                                                                                                         |
| Personnel                 | `~/.claude/skills/<skill-name>/SKILL.md`                                                                                   | Tous vos projets sur cette machine, mais pas les [sessions Cowork ou cloud](#skills-in-cowork-and-cloud-sessions)                                                                                                               |
| Projet                    | `.claude/skills/<skill-name>/SKILL.md`                                                                                     | Sessions dans ce référentiel. Validez-la pour que votre équipe l'obtienne aussi                                                                                                                                                 |
| Imbriquée                 | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                            | Sessions démarrées dans ou sous `<subdir>`. Une session démarrée au-dessus la charge une fois que Claude travaille sur des fichiers là-bas. Voir [monorepos et sous-répertoires](#discovery-from-parent-and-nested-directories) |
| Répertoire supplémentaire | `.claude/skills/<skill-name>/SKILL.md` dans un répertoire que vous transmettez avec `--add-dir`                            | Cette session. Voir [répertoires en dehors du projet](#skills-from-additional-directories)                                                                                                                                      |
| Plugin                    | `<plugin>/skills/<skill-name>/SKILL.md`                                                                                    | Partout où le [plugin](/docs/fr/plugins/overview) est activé, en tant que `/plugin-name:skill-name`                                                                                                                                  |
| Compte claude.ai          | Skills activées pour votre compte claude.ai                                                                                | Sessions Cowork, sessions cloud et sessions de terminal où vous vous connectez avec ce compte. Voir [Skills synchronisées depuis claude.ai](#how-synced-skills-behave)                                                          |

Les dossiers de skills suivent également ces règles :

* **Dossiers avec lien symbolique** : une entrée `<skill-name>` à l'emplacement entreprise, personnel ou projet peut être un lien symbolique vers un répertoire ailleurs sur le disque. Claude Code lit `SKILL.md` depuis la cible et charge la skill une seule fois même si plusieurs emplacements pointent vers la même cible. Les skills de plugin [gèrent les liens symboliques différemment](/docs/fr/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks).
* **Nom réservé** : ne nommez pas un dossier de skill `synced`, quelle que soit la casse. Claude Code utilise `~/.claude/skills/synced/` pour les [skills téléchargées depuis claude.ai](#where-synced-skills-load) et ignore une skill que vous créez à ce nom aux emplacements entreprise, personnel et projet.
* **Fichiers de commande** : un fichier Markdown dans `.claude/commands/` est le format plus ancien et fonctionne toujours. Il supporte le même [frontmatter](#frontmatter-reference) sauf `name` et `paths`. Pour trouver le nom que vous tapez pour l'invoquer, voir [Comment une skill obtient son nom de commande](#how-a-skill-gets-its-command-name). Préférez une skill pour les nouveaux travaux, puisque les skills supportent aussi les [fichiers de support](#add-supporting-files).
* **Dossier de skill en tant que plugin** : ajoutez un `.claude-plugin/plugin.json` à un dossier de skill et il se charge en tant que [plugin](/docs/fr/plugins/loading#plugins-shared-through-a-repository) nommé `<name>@skills-dir`, pour qu'il puisse regrouper des agents, des hooks et des serveurs MCP. Dans un `.claude/skills/` de projet, cela nécessite d'accepter d'abord la boîte de dialogue de confiance de l'espace de travail.

<h3 id="discovery-from-parent-and-nested-directories">
  Charger les skills dans les monorepos et les sous-répertoires
</h3>

Claude Code charge les skills de projet depuis `.claude/skills/` dans le répertoire où vous le démarrez et dans chaque répertoire parent jusqu'à la racine du référentiel, donc démarrer dans `packages/frontend/` récupère toujours les skills définies à la racine. Quand vous [déplacez la session avec `/cd`](/docs/fr/permissions#move-the-session-to-another-directory) sur v2.1.246 ou ultérieur, Claude Code ajoute les skills de projet du nouveau répertoire.

Dans une session s'exécutant dans un [git worktree](/docs/fr/worktrees) lié, Claude Code recherche les répertoires parents uniquement jusqu'à la racine du worktree. Sur Claude Code v2.1.277 ou ultérieur, quand le checkout du worktree n'a pas de répertoire `.claude/skills` à sa racine, Claude Code charge à la place les skills de projet du checkout principal. Voir [Ce que les worktrees partagent avec le checkout principal](/docs/fr/worktrees#what-worktrees-share-with-the-main-checkout).

Les skills dans un répertoire `.claude/skills/` en dessous de celui où vous avez démarré ne se chargent pas au démarrage. Elles se chargent la première fois que Claude lit ou édite un fichier dans ce sous-répertoire et restent disponibles pour le reste de la session. Jusqu'à ce moment, elles n'apparaissent pas dans le menu `/` et vous ne pouvez pas les invoquer par nom. Pour les charger plus tôt, exécutez `/add-dir` avec le chemin du sous-répertoire, ce qui nécessite Claude Code v2.1.257 ou ultérieur.

Quand une skill imbriquée partage un nom avec une autre skill, les deux restent disponibles. Avec une skill `deploy` à la racine du référentiel et une autre dans `apps/web/.claude/skills/` :

* `/deploy` exécute la skill racine. Claude Code liste aussi les variantes qualifiées par répertoire pour Claude, avec une instruction pour invoquer celle dont le répertoire contient les fichiers sur lesquels il travaille, donc la skill imbriquée s'applique toujours au travail dans `apps/web/`.
* `/apps/web:deploy` exécute la skill imbriquée seule. Sa description nomme le répertoire auquel elle s'applique.

<h3 id="skills-from-additional-directories">
  Charger les skills depuis un répertoire en dehors du projet
</h3>

Quand vous ajoutez un répertoire avec `--add-dir` ou `/add-dir`, Claude Code charge les skills dans le `.claude/skills/` de ce répertoire, ainsi que son `.claude/commands/` et `.claude/agents/`. Les répertoires que le SDK Agent ajoute via [`additionalDirectories`](/docs/fr/agent-sdk/typescript#options) en TypeScript ou [`add_dirs`](/docs/fr/agent-sdk/python#claudeagentoptions) en Python se chargent de la même façon, car le SDK les transmet en tant que `--add-dir`. Le paramètre `permissions.additionalDirectories` dans `settings.json` accorde uniquement l'accès aux fichiers et ne charge aucun de ceux-ci.

Claude Code surveille `.claude/skills/` dans un répertoire que vous transmettez avec `--add-dir` au lancement, comme le décrit [Éditer une skill pendant une session](#live-change-detection). Il ne surveille pas le `.claude/commands/` ou `.claude/agents/` du répertoire ajouté, donc redémarrez la session après avoir modifié un fichier là-bas.

Ces chargements dépendent de la source de paramètre `project` [setting source](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources), qui est activée par défaut. Une politique [`strictPluginOnlyCustomization`](/docs/fr/settings-reference#strictpluginonlycustomization), le [mode bare](/docs/fr/headless#start-faster-with-bare-mode) et [`--safe-mode`](/docs/fr/cli-reference#cli-flags) les restreignent davantage, comme ces pages le décrivent. Voir [Les répertoires supplémentaires accordent l'accès aux fichiers, pas la configuration](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) pour le tableau complet de ce qu'un répertoire ajouté charge, y compris `CLAUDE.md` et les paramètres de plugin.

<h3 id="resolve-skills-that-share-a-name">
  Résoudre les skills qui partagent un nom
</h3>

Quand deux skills partagent un nom, l'endroit d'où chacune provient décide laquelle `/name` exécute. Le tableau couvre les emplacements entreprise, personnel, projet, imbriqué, plugin et claude.ai, les skills regroupées et les fichiers de commande :

| Même nom dans                                                                                                           | Laquelle s'exécute                                                                                                                                                                                                                       |
| :---------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Deux parmi entreprise, personnel et projet                                                                              | Entreprise sur personnel, et personnel sur projet. Avec `deploy` dans `~/.claude/skills/` et dans le `.claude/skills/` du projet, `/deploy` exécute la personnelle                                                                       |
| N'importe lequel de ces emplacements et une [skill regroupée](#bundled-skills)                                          | Votre skill remplace la commande regroupée, mais pas ses alias. Une skill `code-review` de projet remplace `/code-review`, et l'alias regroupé `/review` n'exécute jamais votre skill                                                    |
| Une skill et un fichier dans `.claude/commands/`                                                                        | La skill                                                                                                                                                                                                                                 |
| Une skill racine de projet et une skill imbriquée                                                                       | Les deux se chargent. Voir [monorepos et sous-répertoires](#discovery-from-parent-and-nested-directories)                                                                                                                                |
| Une skill de plugin et une skill à n'importe lequel des emplacements ci-dessus                                          | Les deux se chargent, car les skills de plugin sont espacées de noms en tant que `/plugin-name:skill-name`                                                                                                                               |
| N'importe lequel de ce qui précède et une skill [synchronisée depuis votre compte claude.ai](#how-synced-skills-behave) | L'autre skill ou commande. La skill synchronisée s'exécute toujours en tant que `/anthropic-skills:<name>`. Voir [Quand un nom de skill synchronisée correspond à une autre commande](#when-a-synced-skill-name-matches-another-command) |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Utiliser les skills dans les sessions Cowork et cloud
</h3>

Les sessions [Cowork](https://claude.com/product/cowork) et les [sessions cloud](/docs/fr/cloud-environments#what-carries-over-from-your-setup), y compris les [routines](/docs/fr/routines), ne lisent pas `~/.claude/skills/` sur votre machine. Les sessions Cowork interactives et planifiées chargent les skills activées pour votre compte claude.ai, synchronisées au démarrage de la session ; gérez-les depuis **Customize** dans la barre latérale de l'application Desktop ou depuis les paramètres de skills sur claude.ai. Les sessions cloud chargent en outre les skills de projet validées dans le `.claude/skills/` du référentiel cloné.

Si une skill existe uniquement dans `~/.claude/skills/` sur votre machine, Claude Code signale que la skill n'a pas été trouvée quand une [routine](/docs/fr/routines) l'invoque, car chaque exécution de routine démarre en tant que nouvelle session cloud. Pour rendre une skill personnelle disponible dans ces sessions :

* Pour les sessions Cowork et cloud, activez la skill pour votre compte claude.ai.
* Pour les sessions cloud, vous pouvez à la place valider la skill dans le `.claude/skills/` du référentiel. Les plugins déclarés dans le `.claude/settings.json` du référentiel et les plugins activés uniquement dans vos paramètres utilisateur [ne se chargent pas dans les sessions cloud](/docs/fr/cloud-environments#what-carries-over-from-your-setup).

Les [tâches planifiées Desktop](/docs/fr/desktop-scheduled-tasks) s'exécutent localement sur votre machine, donc elles chargent `~/.claude/skills/`.

<h3 id="how-synced-skills-behave">
  Skills synchronisées depuis claude.ai
</h3>

Cette section s'applique à vous si vous utilisez les sessions Cowork ou cloud, ou si vous vous connectez à Claude Code dans votre terminal avec un compte claude.ai. Dans ces sessions, Claude Code charge les skills activées pour votre compte claude.ai, sans configuration de votre part, comme le décrit [Où les skills synchronisées se chargent](#where-synced-skills-load). Ces skills incluent celles que vous créez ou activez dans vos paramètres claude.ai, les skills que votre organisation fournit là-bas, et les skills intégrées d'Anthropic telles que `pdf` et `xlsx`.

Claude Code télécharge une skill synchronisée depuis votre compte plutôt que de lire un fichier que vous avez écrit sur la machine où la session s'exécute, donc il applique des règles aux skills synchronisées qui ne s'appliquent pas aux skills que vous stockez dans les [emplacements de skills](#where-skills-live).

<h4 id="where-synced-skills-load">
  Où les skills synchronisées se chargent
</h4>

Dans une session Cowork ou cloud, Claude Code charge les skills activées pour votre compte claude.ai, et [Skills dans les sessions Cowork et cloud](#skills-in-cowork-and-cloud-sessions) dit comment choisir quelles skills ces sessions obtiennent.

Dans votre terminal, Claude Code synchronise ces skills dans les sessions où vous vous connectez avec votre compte claude.ai. Quand la session démarre, Claude Code télécharge les skills de votre compte dans `~/.claude/skills/synced/` en arrière-plan, puis vérifie claude.ai pour les modifications environ toutes les 10 minutes pendant que la session s'exécute. Quand une vérification découvre qu'une skill a été ajoutée, éditée ou désactivée sur claude.ai, Claude Code l'ajoute, la met à jour ou la supprime dans la session en cours sans redémarrage. La synchronisation dans les sessions de terminal nécessite Claude Code v2.1.273 ou ultérieur.

La synchronisation ne retarde jamais le démarrage, car Claude attend le téléchargement d'une skill uniquement quand il invoque cette skill. Une exécution courte [non-interactive](/docs/fr/headless) peut donc se terminer avant qu'une skill nouvellement ajoutée ne se télécharge, auquel cas une session ultérieure la télécharge. Pour faire en sorte qu'une exécution non-interactive télécharge vos skills et attende la liste avant de répondre à l'invite, définissez [`CLAUDE_CODE_SYNC_SKILLS`](/docs/fr/env-vars#variables) sur `1`.

Claude Code ne synchronise que dans une session qui se connecte avec votre compte claude.ai et [récupère les drapeaux de fonctionnalités depuis Anthropic](/docs/fr/env-vars#features-that-need-feature-flag-fetching). Il ne synchronise pas dans ces sessions :

* Une session qui n'utilise pas une connexion stockée par `/login`, comme une qui s'authentifie avec une clé API, ou une où `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN` ou un script `apiKeyHelper` fournit les identifiants
* Une session qui ne récupère pas les drapeaux de fonctionnalités, comme une sur Amazon Bedrock ou une où vous définissez `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* Une session en [mode bare](/docs/fr/headless#start-faster-with-bare-mode) ou une que vous démarrez avec `--safe-mode`
* Une session où les paramètres gérés de votre organisation [verrouillent les skills aux sources de plugin](/docs/fr/settings-reference#strictpluginonlycustomization-skills), ou une que vous démarrez avec une liste [`--setting-sources`](/docs/fr/cli-reference#cli-flags) qui omet `user`

Si vous vous connectez avec `/login` pendant une session, redémarrez Claude Code pour commencer la synchronisation.

Les skills qu'une session antérieure a synchronisées restent sur le disque. Claude Code les charge dans les sessions ultérieures connectées au même compte, même quand il ne peut pas atteindre claude.ai.

Claude Code télécharge les skills synchronisées et ne les télécharge jamais. Si vous ou Claude éditez un fichier sous `~/.claude/skills/synced/`, la modification n'est pas enregistrée sur votre compte claude.ai, et une synchronisation ultérieure peut la remplacer ou la supprimer. Pour modifier une skill synchronisée, mettez-la à jour sur claude.ai ; la prochaine synchronisation télécharge la nouvelle version.

Pour voir quelles skills se sont synchronisées, exécutez `/skills`. Le menu les liste sous `claude.ai sync`.

Certaines skills d'Anthropic, comme `pdf` et `xlsx`, se synchronisent toujours. Pour les autres, activez ou désactivez une skill dans vos paramètres de skills sur claude.ai pour changer si elle se synchronise.

Pour arrêter la synchronisation sur une machine, définissez [`syncClaudeAiSkills`](/docs/fr/settings-reference#syncclaudeaiskills) sur `false` dans vos paramètres utilisateur. Claude Code arrête le téléchargement, et la prochaine fois qu'il démarre, il déplace les skills qu'il a déjà synchronisées vers `~/.claude/skills/.trash/` et ne les charge plus. Votre organisation peut désactiver la synchronisation pour tout le monde en désactivant Skills sur claude.ai. Pour arrêter la synchronisation tout en laissant Skills activé, elle peut définir la même clé dans les [paramètres gérés](/docs/fr/managed-settings).

Si votre organisation désactive Skills sur claude.ai, Claude Code supprime les skills téléchargées et elles cessent de se charger. Les skills supprimées se déplacent vers `~/.claude/skills/.trash/`, où vous pouvez récupérer les fichiers jusqu'à ce que le [balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically) les supprime. Une fois que votre organisation réactive Skills, Claude Code télécharge les skills que vous avez activées à la prochaine synchronisation.

<h4 id="when-a-synced-skill-name-matches-another-command">
  Quand un nom de skill synchronisée correspond à une autre commande
</h4>

Vous pouvez invoquer une skill synchronisée par son nom complet, `/anthropic-skills:<name>`, ou par son nom court, `/<name>`. Quand une autre commande utilise ce nom court, `/<name>` exécute l'autre commande, et la skill synchronisée s'exécute uniquement en tant que `/anthropic-skills:<name>`. Avec une skill `deploy` locale et une `deploy` synchronisée, `/deploy` exécute la skill locale et `/anthropic-skills:deploy` exécute la synchronisée. Avant v2.1.269, une skill synchronisée n'avait que son nom court.

L'autre commande peut être n'importe laquelle de celles-ci :

* Une commande intégrée ou une [skill regroupée](#bundled-skills), y compris une qui n'est pas disponible dans votre session, par exemple après avoir désactivé les skills regroupées
* Une skill à n'importe quel [niveau local](#where-skills-live) ou un fichier dans `.claude/commands/`
* Une skill de plugin
* Une [invite MCP](/docs/fr/mcp#use-mcp-prompts-as-commands)

Claude Code étiquette les skills synchronisées pour que vous puissiez dire d'où elles proviennent. Le menu `/skills` et `/context` groupent les skills synchronisées sous `claude.ai sync`, et le menu de commande `/` les marque comme provenant de claude.ai.

Quand il compare les noms, Claude Code ignore la casse, l'espacement et les caractères invisibles, et traite les formes de compatibilité telles que les lettres pleine largeur et les variantes de tiret comme leurs équivalents simples. Par exemple, une skill synchronisée nommée `Commit` et une skill locale nommée `commit` comptent comme le même nom, donc `/commit` continue d'exécuter votre skill locale.

Un nom qui diffère uniquement par une lettre ressemblante d'un autre alphabet compte comme un nom différent, et l'étiquette `claude.ai sync` est comment vous distinguez les deux. Ces vérifications et étiquettes nécessitent Claude Code v2.1.228 ou ultérieur.

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Comment Claude Code gère le frontmatter d'une skill synchronisée
</h4>

Claude Code applique deux règles au frontmatter d'une skill synchronisée :

* Claude Code honore le frontmatter dans chaque type de session, donc une concession `allowed-tools` passe par le [flux de permission](/docs/fr/permissions) normal.
* Claude Code assainit le texte d'affichage que la skill fournit, comme sa description. Il supprime les caractères de contrôle, et dans le texte qui atteint Claude, comme la description, il échappe aussi les crochets pointus pour que le texte ne puisse pas imiter le formatage interne de Claude Code. Cet assainissement nécessite Claude Code v2.1.228 ou ultérieur.

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Comment Claude Code gère le corps d'une skill synchronisée
</h4>

Ce que Claude Code fait avec le corps d'une skill synchronisée dépend de l'endroit où la session s'exécute :

* Dans une session cloud, le corps conserve le comportement qu'une skill locale a, car la session s'exécute dans un conteneur isolé.
* Dans une session Cowork sur votre bureau, le corps conserve le comportement qu'une skill locale a, sauf que Claude Code remplace chaque ligne de commande `!` par l'espace réservé [`disableSkillShellExecution`](#inject-dynamic-context), comme il le fait pour chaque skill que vous fournissez là-bas.
* Dans toute autre session sur votre machine, Claude Code n'exécute pas les commandes [`!`](#inject-dynamic-context), n'attache pas les fichiers que les références `@` nomment de la façon qu'il le fait pour une skill locale, et ne substitue pas les espaces réservés `${CLAUDE_PROJECT_DIR}` et `${CLAUDE_SESSION_ID}`, donc les références `@` et les deux espaces réservés atteignent Claude en tant que texte littéral. Une ligne de commande `!` atteint aussi Claude en tant que texte littéral, ou en tant que cet espace réservé quand `disableSkillShellExecution` est activé. Cette gestion nécessite Claude Code v2.1.228 ou ultérieur.

<h3 id="live-change-detection">
  Éditer une skill pendant une session
</h3>

Claude Code surveille les répertoires de skills pour les modifications de fichiers, sauf en [mode bare](/docs/fr/headless#start-faster-with-bare-mode). Quand vous ajoutez, éditez ou supprimez une skill sous `~/.claude/skills/`, le `.claude/skills/` du projet, ou un `.claude/skills/` à l'intérieur d'un répertoire `--add-dir`, Claude Code récupère la modification dans la session actuelle, sans redémarrage. Si vous créez un répertoire de skills de niveau supérieur qui n'existait pas quand la session a démarré, redémarrez Claude Code pour qu'il puisse surveiller le nouveau répertoire.

La détection de changement en direct couvre uniquement le texte `SKILL.md`. Pour un dossier de skill qui est aussi un [plugin](/docs/fr/plugins/loading#plugins-shared-through-a-repository), les modifications apportées à `hooks/`, `.mcp.json`, `agents/` et `output-styles/` nécessitent `/reload-plugins` pour prendre effet.

<h3 id="remove-a-skill">
  Supprimer une skill
</h3>

La façon dont vous supprimez une skill dépend de l'endroit d'où elle provient :

* **Skill personnelle ou de projet** : supprimez le répertoire de la skill, `~/.claude/skills/<skill-name>/` ou `.claude/skills/<skill-name>/`. Claude Code la [supprime de `/skills` dans la session actuelle](#live-change-detection) ; le contenu que Claude Code a déjà chargé depuis elle suit le [cycle de vie du contenu de skill](#skill-content-lifecycle).
* **Skill d'entreprise** : un administrateur supprime le répertoire de la skill depuis `.claude/skills/` à l'intérieur du [répertoire des paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms), par exemple `/etc/claude-code/.claude/skills/<skill-name>/` sur Linux.
* **Skill de plugin** : désactivez ou désinstallez le plugin qui la fournit, depuis le menu `/plugin` ou avec `/plugin uninstall <plugin-name>@<marketplace-name>`. Claude Code décharge les skills du plugin quand [le changement s'applique](/docs/fr/plugins/cli-reference#reload-plugins) ou quand vous redémarrez.
* **Skill synchronisée depuis claude.ai** : désactivez la skill pour votre compte claude.ai, au même endroit où vous l'[avez activée](#skills-in-cowork-and-cloud-sessions). Claude Code la supprime de `~/.claude/skills/synced/` la prochaine fois qu'elle [synchronise vos skills](#where-synced-skills-load). Si vous supprimez le répertoire à la main à la place, la prochaine synchronisation le télécharge à nouveau pendant que la skill reste activée sur claude.ai.
* **Skill regroupée** : définissez [`disableBundledSkills`](#bundled-skills) sur `true` pour désactiver les skills regroupées, ou définissez une skill sur `"off"` dans [`skillOverrides`](#override-skill-visibility-from-settings) pour la masquer.

Pour conserver une skill personnelle ou de projet mais empêcher Claude de l'invoquer de lui-même, définissez [`disable-model-invocation: true`](#control-who-invokes-a-skill) dans son frontmatter, ou `"user-invocable-only"` dans [`skillOverrides`](#override-skill-visibility-from-settings) quand vous ne voulez pas éditer le fichier.

<h2 id="configure-skills">
  Configurer les compétences
</h2>

Les compétences sont configurées via le frontmatter YAML en haut de `SKILL.md` et le contenu markdown qui suit.

<h3 id="types-of-skill-content">
  Types de contenu de compétence
</h3>

Les fichiers de compétence peuvent contenir n'importe quelles instructions, mais réfléchir à la façon dont vous souhaitez les invoquer aide à guider ce qu'il faut inclure :

**Le contenu de référence** ajoute des connaissances que Claude applique à votre travail actuel. Conventions, modèles, guides de style, connaissances du domaine. Ce contenu s'exécute en ligne afin que Claude puisse l'utiliser aux côtés du contexte de votre conversation.

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

**Le contenu de tâche** donne à Claude des instructions étape par étape pour une action spécifique, comme les déploiements, les commits ou la génération de code. Ce sont souvent des actions que vous souhaitez invoquer directement avec `/skill-name` plutôt que de laisser Claude décider quand les exécuter. Ajoutez `disable-model-invocation: true` pour empêcher Claude de le déclencher automatiquement. L'exemple ci-dessous ajoute `context: fork`, qui exécute la compétence dans son propre contexte de sous-agent ; voir [Exécuter les compétences dans un sous-agent](#run-skills-in-a-subagent).

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

Gardez le corps lui-même concis. Une fois qu'une compétence se charge, son contenu [reste en contexte à travers les tours](#skill-content-lifecycle), donc chaque ligne a un coût de token récurrent. Indiquez ce qu'il faut faire plutôt que de raconter comment ou pourquoi, et appliquez le même test de concision que vous feriez pour [le contenu CLAUDE.md](/docs/fr/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Référence du frontmatter
</h3>

Configurez une compétence avec le [frontmatter](/docs/fr/glossary#frontmatter) YAML entre les marqueurs `---` en haut de `SKILL.md`, et écrivez les instructions de la compétence en Markdown après le `---` de fermeture. Les noms de champs utilisent des mots minuscules séparés par des tirets, sauf `when_to_use`. Un [fichier de commande](#where-skills-live) dans `.claude/commands/` accepte les mêmes champs sauf `name` et `paths`. Cet exemple définit quatre champs :

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Tous les champs sont optionnels. Seul `description` est recommandé afin que Claude sache quand utiliser la compétence. Un nom de champ doit correspondre exactement au tableau, tirets inclus : Claude Code ignore un champ qu'il ne reconnaît pas sans signaler une erreur.

Claude Code lit le frontmatter uniquement lorsque l'ouverture `---` est la première ligne du fichier. Sinon, il traite le fichier entier, marqueurs `---` inclus, comme contenu de compétence. Si le YAML entre les marqueurs ne s'analyse pas, la compétence se charge quand même sans champs définis ; voir [Compétence ne se déclenchant pas](#skill-not-triggering) pour trouver et corriger l'erreur.

Les champs booléens acceptent `yes`, `no`, `on`, `off`, `1` et `0` dans n'importe quelle casse de lettre, en plus de `true` et `false`. Avant v2.1.218, Claude Code ne reconnaissait que `true` et `false`.

| Champ                      | Requis     | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Non        | Nom d'affichage montré dans les listes de compétences. Par défaut, le nom du répertoire. Voir [Comment une compétence obtient son nom de commande](#how-a-skill-gets-its-command-name) pour savoir comment le champ interagit avec le nom que vous tapez pour invoquer la compétence.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `description`              | Recommandé | Ce que fait la compétence et quand l'utiliser. Claude utilise ceci pour décider quand appliquer la compétence. S'il est omis, utilise la première ligne non vide du contenu markdown. Mettez le cas d'usage clé en premier : le texte combiné `description` et `when_to_use` est tronqué à 1 536 caractères dans la liste des compétences pour réduire l'utilisation du contexte.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `when_to_use`              | Non        | Contexte supplémentaire pour savoir quand Claude devrait invoquer la compétence, comme les phrases déclencheurs ou les demandes d'exemple. Ajouté à `description` dans la liste des compétences et compte vers le plafond de 1 536 caractères.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `argument-hint`            | Non        | Indice affiché lors de l'autocomplétion pour indiquer les arguments attendus. Exemple : `[issue-number]` ou `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `arguments`                | Non        | Arguments positionnels nommés pour la [substitution `$name`](#available-string-substitutions) dans le contenu de la compétence. Accepte une chaîne séparée par des espaces ou une liste YAML. Les noms correspondent aux positions d'argument dans l'ordre.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disable-model-invocation` | Non        | Définissez sur `true` pour empêcher Claude de charger automatiquement cette compétence. Utilisez pour les flux de travail que vous souhaitez déclencher manuellement avec `/name`. Empêche également la compétence d'être [préchargée dans les sous-agents](/docs/fr/sub-agents#preload-skills-into-subagents). À partir de v2.1.196, empêche également la compétence de s'exécuter lorsqu'une [tâche programmée](/docs/fr/scheduled-tasks) se déclenche avec la compétence comme invite. Par défaut : `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `user-invocable`           | Non        | Définissez sur `false` lorsque seul Claude devrait invoquer la compétence : Claude Code la masque du menu `/` et ne l'exécute pas lorsque vous tapez `/name`. Utilisez pour les connaissances de base que les utilisateurs ne devraient pas invoquer directement. Par défaut : `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `allowed-tools`            | Non        | Outils que Claude peut utiliser sans demander la permission lors du tour qui invoque cette compétence. La subvention s'efface lorsque vous envoyez votre message suivant. Accepte une chaîne séparée par des espaces ou des virgules, ou une liste YAML. Voir [Pré-approuver les outils pour une compétence](#pre-approve-tools-for-a-skill).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disallowed-tools`         | Non        | Outils supprimés du pool disponible de Claude tandis que cette compétence est active. Utilisez pour les compétences autonomes qui ne devraient jamais appeler certains outils, comme `AskUserQuestion` pour une boucle de fond. Accepte une chaîne séparée par des espaces ou des virgules, ou une liste YAML. La restriction s'efface lorsque vous envoyez votre message suivant. Comme les règles de refus, le champ ne peut pas supprimer [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) tant que tout autre outil reste.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `model`                    | Non        | Modèle à utiliser lorsque cette compétence est active. Le remplacement s'applique pour le reste du tour actuel et n'est pas enregistré dans les paramètres. Le modèle de session reprend à votre prochaine invite. Accepte les mêmes valeurs que [`/model`](/docs/fr/model-config), ou `inherit` pour conserver le modèle actif. Une valeur exclue par la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation n'est pas utilisée et la session conserve son modèle actuel. En [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) et en [mode plan tandis que le classificateur examine les commandes](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode), un modèle que le mode auto ne supporte pas n'est pas utilisé non plus, et la session conserve son modèle actuel. Avec `context: fork`, la valeur définit le [modèle du sous-agent forké](#run-skills-in-a-subagent) à la place, et une valeur exclue suit les [mêmes règles qu'un remplacement de modèle de sous-agent](/docs/fr/model-config#restrict-model-selection). |
| `effort`                   | Non        | [Niveau d'effort](/docs/fr/model-config#adjust-effort-level) lorsque cette compétence est active. Remplace le niveau d'effort de la session. Par défaut : hérite de la session. Options : `low`, `medium`, `high`, `xhigh`, `max` ; les niveaux disponibles dépendent du modèle.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `context`                  | Non        | Définissez sur `fork` pour exécuter dans un contexte de sous-agent forké. Voir [Exécuter les compétences dans un sous-agent](#run-skills-in-a-subagent).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `agent`                    | Non        | Quel type de sous-agent utiliser lorsque `context: fork` est défini.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `background`               | Non        | S'applique uniquement avec `context: fork`. Définissez sur `false` pour attendre le résultat du sous-agent forké dans le tour qui a invoqué la compétence, au lieu de [l'exécuter en arrière-plan](#run-skills-in-a-subagent). Par défaut : `true`. Nécessite Claude Code v2.1.218 ou ultérieur.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `hooks`                    | Non        | Hooks que Claude Code enregistre lorsque la compétence est invoquée et continue à exécuter pour le reste de la session. Voir [Hooks dans les compétences et les agents](/docs/fr/hooks#hooks-in-skills-and-agents) pour le format de configuration et l'option `once`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `paths`                    | Non        | Modèles Glob qui limitent quand cette compétence est activée. Accepte une chaîne séparée par des virgules ou une liste YAML. Lorsqu'elle est définie, Claude charge la compétence automatiquement uniquement lorsqu'il travaille avec des fichiers correspondant aux modèles. Utilise le même format que [les règles spécifiques au chemin](/docs/fr/memory#path-specific-rules).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `shell`                    | Non        | Shell à utiliser pour `` !`command` `` et ` ```! ` blocs dans cette compétence. Accepte `bash` (par défaut) ou `powershell`. La définition de `powershell` exécute les commandes shell en ligne via PowerShell lorsque l'[outil PowerShell](/fr/tools-reference#powershell-tool) est activé : il est activé par défaut sur Windows sans Git Bash, activé par défaut avec Git Bash pour les comptes claude.ai et Console, et nécessite `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` dans les sessions Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry et sur macOS, Linux et WSL. Définissez-le sur `0` pour désactiver l'outil.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `metadata`                 | Non        | Carte YAML libre pour vos propres données clé-valeur, comme les champs d'habilitation ou de catalogue, lus par votre propre outillage à partir de `SKILL.md`. Claude Code n'agit pas sur son contenu et supprime une valeur qui n'est pas une carte. Ne réutilisez pas les noms de champs frontmatter tels que `paths` comme clés.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `license`                  | Non        | Licence couvrant la compétence. Fait partie de la spécification [Agent Skills](https://agentskills.io) ; voir [Utiliser le frontmatter de compétence en dehors de Claude Code](#using-skill-frontmatter-outside-claude-code). Claude Code accepte le champ mais n'agit pas sur lui.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `compatibility`            | Non        | Exigences d'environnement pour la compétence, comme les produits prévus ou les prérequis système, tels que définis par la spécification [Agent Skills](https://agentskills.io) ; voir [Utiliser le frontmatter de compétence en dehors de Claude Code](#using-skill-frontmatter-outside-claude-code). Accepte une chaîne de jusqu'à 500 caractères. Claude Code accepte le champ mais n'agit pas sur lui.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Utiliser le frontmatter de compétence en dehors de Claude Code
</h4>

Claude Code accepte tous les champs du tableau ci-dessus. En dehors de Claude Code, vous ne pouvez utiliser que les champs de la spécification [Agent Skills](https://agentskills.io) :

| Chemin de distribution                                                                                                                                       | Champs de frontmatter que vous pouvez utiliser                                 |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Compétences Claude Code à [n'importe quel niveau](#where-skills-live), y compris les compétences [plugin](/docs/fr/plugins/overview)                              | Tous les champs du tableau ci-dessus                                           |
| Téléchargements de compétences claude.ai, l'API Skills et l'empaquetage avec `package_skill.py` de [anthropics/skills](https://github.com/anthropics/skills) | `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools` |

Lorsque vous activez une compétence personnelle pour votre compte claude.ai, par exemple pour l'utiliser dans [les sessions Cowork et cloud](#skills-in-cowork-and-cloud-sessions) et les routines, vous la téléchargez sur claude.ai, donc les mêmes règles s'appliquent.

Si vous incluez un champ que la spécification ne permet pas, l'empaquetage ou le téléchargement échoue avec une erreur matérielle au lieu d'ignorer le champ :

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

Restreindre le frontmatter aux six champs de la spécification évite l'erreur de clé inattendue ci-dessus. La [spécification Agent Skills](https://agentskills.io) et les [exigences de l'API Skills](https://docs.claude.com/en/api/skills-guide) définissent tout le reste que ces chemins valident. Les fonctionnalités du corps spécifiques à Claude Code, telles que [l'injection de contexte dynamique](#inject-dynamic-context), ne fonctionnent pas dans le chat claude.ai ou via l'API. Claude Code accepte les six champs, donc le frontmatter qui suit la spécification se charge dans Claude Code sans modifications.

<h4 id="how-a-skill-gets-its-command-name">
  Comment une compétence obtient son nom de commande
</h4>

La commande que vous tapez pour invoquer une compétence provient de l'endroit où le fichier de compétence se trouve et, pour les compétences de plugin, également du champ frontmatter `name`. Dans une compétence personnelle ou de projet, `name` définit uniquement l'étiquette d'affichage affichée dans les listes de compétences, et la commande provient toujours du nom du répertoire. Dans une compétence de plugin, `name` définit le dernier segment de la commande et le préfixe du plugin reste en place.

Le tableau ci-dessous montre d'où provient le nom de la commande pour chaque disposition :

| Emplacement de la compétence                                                                                           | Source du nom de la commande                                                                                            | Exemple                                                                                                                                          |
| :--------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| Répertoire de compétence sous `~/.claude/skills/` ou `.claude/skills/`                                                 | Nom du répertoire                                                                                                       | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                                     |
| Répertoire `.claude/skills/` [imbriqué](#where-skills-live), lorsque le nom entre en conflit avec une autre compétence | Chemin du sous-répertoire relatif au répertoire de travail, puis le nom du répertoire de compétence                     | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                                   |
| Fichier sous `.claude/commands/`                                                                                       | Nom du fichier sans extension                                                                                           | `.claude/commands/deploy.md` → `/deploy`                                                                                                         |
| Fichier dans un sous-répertoire de `.claude/commands/`                                                                 | Chemin du sous-répertoire relatif à `commands/` avec chaque `/` remplacé par `:`, puis le nom du fichier sans extension | `.claude/commands/frontend/component.md` → `/frontend:component`                                                                                 |
| Sous-répertoire `skills/` du plugin                                                                                    | Frontmatter `name` ou le nom du répertoire, préfixé par le plugin                                                       | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`, ou `/my-plugin:fancy` avec `name: fancy`                                               |
| `SKILL.md` racine du plugin                                                                                            | Frontmatter `name`, avec le nom du répertoire du plugin comme secours                                                   | `my-plugin/SKILL.md` avec `name: review` → `/my-plugin:review`. Voir [une seule compétence à la racine du plugin](/docs/fr/plugins/components#skills) |
| Compétence [synchronisée depuis claude.ai](#how-synced-skills-behave)                                                  | Le nom de la compétence sur votre compte claude.ai, préfixé avec `anthropic-skills:`                                    | Compétence de compte `deploy` → `/anthropic-skills:deploy`, ou `/deploy` si aucune autre commande n'utilise ce nom                               |

Dans une compétence de plugin, le frontmatter `name` remplace le nom du répertoire dans le dernier segment de la commande, donc `my-plugin/skills/review/SKILL.md` avec `name: fancy` devient `/my-plugin:fancy`. Le `/fancy` nu invoque également la compétence à moins qu'une autre commande n'utilise déjà ce nom. Si le `name` que vous écrivez commence déjà par le propre préfixe du plugin, Claude Code n'ajoute pas le préfixe à nouveau sur v2.1.246 ou ultérieur. Par exemple, `name: my-plugin:fancy` devient toujours `/my-plugin:fancy`. De v2.1.216 à v2.1.245, Claude Code doublait le préfixe lorsque le `name` le portait déjà.

Dans [les sessions non interactives](/docs/fr/headless), les noms `help` et `feedback` ne sont pas réservés à leurs commandes intégrées terminales uniquement, donc une compétence de plugin avec l'un de ces noms conserve sa commande nue là. Tous les autres noms de commandes intégrées terminales, comme `/login`, restent réservés même si la commande ne peut pas s'exécuter dans ces sessions.

Pour un `SKILL.md` racine de plugin, il n'y a pas de répertoire de compétence d'où prendre le nom, donc `name` fournit le segment final entier. Sans un champ `name`, Claude Code revient au nom du répertoire du plugin.

<h4 id="available-string-substitutions">
  Substitutions de chaîne disponibles
</h4>

Les compétences prennent en charge la substitution de chaîne pour les valeurs dynamiques dans le contenu de la compétence :

| Variable                | Description                                                                                                                                                                                                                                                                                                                                                       |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | Tous les arguments passés lors de l'invocation de la compétence. Lorsqu'aucun espace réservé ne reçoit un argument, Claude Code ajoute `ARGUMENTS: <value>` à la fin. Voir [Passer des arguments aux compétences](#pass-arguments-to-skills).                                                                                                                     |
| `$ARGUMENTS[N]`         | Accédez à un argument spécifique par index basé sur 0, comme `$ARGUMENTS[0]` pour le premier argument.                                                                                                                                                                                                                                                            |
| `$N`                    | Raccourci pour `$ARGUMENTS[N]`, comme `$0` pour le premier argument ou `$1` pour le second.                                                                                                                                                                                                                                                                       |
| `$name`                 | Argument nommé déclaré dans la liste frontmatter [`arguments`](#frontmatter-reference). Les noms correspondent aux positions dans l'ordre, donc avec `arguments: [issue, branch]` l'espace réservé `$issue` se développe au premier argument et `$branch` au second.                                                                                              |
| `${CLAUDE_SESSION_ID}`  | L'ID de session actuel. Utile pour la journalisation, la création de fichiers spécifiques à la session ou la corrélation de la sortie de compétence avec les sessions.                                                                                                                                                                                            |
| `${CLAUDE_EFFORT}`      | Le niveau d'effort actuel : `low`, `medium`, `high`, `xhigh` ou `max`. Ultracode n'est pas un niveau distinct et est signalé comme `xhigh`. Utilisez ceci pour adapter les instructions de compétence au paramètre d'effort actif.                                                                                                                                |
| `${CLAUDE_SKILL_DIR}`   | Le répertoire contenant le fichier `SKILL.md` de la compétence. Pour les compétences de plugin, c'est le sous-répertoire de la compétence dans le plugin, pas la racine du plugin. Utilisez ceci dans les commandes d'injection bash pour référencer les scripts ou fichiers fournis avec la compétence, quel que soit le répertoire de travail actuel.           |
| `${CLAUDE_PROJECT_DIR}` | Le répertoire racine du projet. C'est le même chemin que [hooks](/docs/fr/hooks#reference-scripts-by-path) et les serveurs MCP reçoivent comme `CLAUDE_PROJECT_DIR`. Utilisez ceci pour référencer les scripts ou fichiers locaux du projet, comme `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, indépendamment de l'endroit où la compétence est installée.        |
| `${CLAUDE_PLUGIN_ROOT}` | Le répertoire d'installation du plugin. Substitué uniquement dans les compétences de plugin. Utilisez ceci pour référencer les scripts ou fichiers fournis n'importe où dans le plugin, y compris les ressources partagées entre les compétences du plugin. Voir [les variables d'environnement du plugin](/docs/fr/plugins/manifest-reference#environment-variables). |
| `${CLAUDE_PLUGIN_DATA}` | Le [répertoire de données persistantes](/docs/fr/plugins/components#path-variables-and-persistent-data) du plugin, qui survit aux mises à jour du plugin. Substitué uniquement dans les compétences de plugin. Utilisez ceci pour référencer les dépendances installées, les fichiers générés ou les caches qui doivent survivre à une mise à jour.                    |

Claude Code substitue `${CLAUDE_SKILL_DIR}` et `${CLAUDE_PROJECT_DIR}` à deux endroits : le contenu markdown de la compétence et les règles Bash dans le frontmatter [`allowed-tools`](#frontmatter-reference). Dans une compétence de plugin, Claude Code substitue `${CLAUDE_PLUGIN_ROOT}` et `${CLAUDE_PLUGIN_DATA}` aux mêmes deux endroits. L'utilisation de la même variable aux deux endroits permet à une compétence d'exécuter un script fourni sans invite de permission. La compétence suivante montre le modèle :

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

Si cette compétence est installée à `~/.claude/skills/render-chart/`, les deux occurrences de `${CLAUDE_SKILL_DIR}` se développent à ce répertoire. La règle `allowed-tools` correspond alors à la commande exacte que le corps de la compétence indique à Claude d'exécuter, donc le script s'exécute sans invite.

La substitution `${CLAUDE_PROJECT_DIR}` nécessite Claude Code v2.1.196 ou ultérieur.

Les arguments indexés utilisent les guillemets de style shell, donc enveloppez les valeurs multi-mots entre guillemets pour les passer comme un seul argument. Par exemple, `/my-skill "hello world" second` fait que `$0` se développe à `hello world` et `$1` à `second`. L'espace réservé `$ARGUMENTS` se développe toujours à la chaîne d'argument complète telle que tapée.

Un espace réservé indexé sans argument correspondant, comme `$2` lorsqu'un seul argument a été passé, reste dans le contenu inchangé. Un espace réservé nommé du frontmatter [`arguments`](#frontmatter-reference) sans argument correspondant se développe à une chaîne vide.

Si vous passez une valeur d'argument qui elle-même contient du texte comme `$1` ou `$ARGUMENTS`, Claude Code l'insère comme texte littéral et ne l'étend pas. Par exemple, si le corps d'une compétence contient `Summarize $0` et que vous exécutez `/summarize "$ARGUMENTS from yesterday"`, Claude reçoit `Summarize $ARGUMENTS from yesterday`. Claude Code remplace toujours les variables `${CLAUDE_*}` comme `${CLAUDE_SKILL_DIR}` après avoir inséré les arguments.

Pour inclure un `$` littéral avant un chiffre, `ARGUMENTS` ou un nom d'argument déclaré, comme `$1.00` en prose, échappez-le avec une barre oblique inverse : `\$1.00`. Une barre oblique inverse avant tout autre `$` est laissée inchangée. Seule une barre oblique inverse directement avant le jeton l'échappe. Une barre oblique inverse doublée comme `\\$1` laisse les deux barres obliques inverses en place, et `$1` se développe toujours à la valeur de l'argument. L'échappement par barre oblique inverse couvre uniquement ces espaces réservés d'argument. Une barre oblique inverse n'empêche pas la substitution d'une variable `${CLAUDE_*}` où la variable s'applique.

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

Les compétences peuvent inclure plusieurs fichiers dans leur répertoire. Cela garde `SKILL.md` concentré sur l'essentiel tout en permettant à Claude d'accéder à du matériel de référence détaillé uniquement si nécessaire. Les grandes docs de référence, les spécifications API ou les collections d'exemples n'ont pas besoin de se charger en contexte à chaque fois que la compétence s'exécute.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Référencez les fichiers de support à partir de `SKILL.md` afin que Claude sache ce que chaque fichier contient et quand le charger :

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Gardez `SKILL.md` sous 500 lignes. Déplacez le matériel de référence détaillé vers des fichiers séparés.</Tip>

<h3 id="control-who-invokes-a-skill">
  Contrôler qui invoque une compétence
</h3>

Par défaut, vous et Claude pouvez invoquer n'importe quelle compétence. Vous pouvez taper `/skill-name` pour l'invoquer directement, et Claude peut la charger automatiquement lorsqu'elle est pertinente pour votre conversation. Deux champs frontmatter vous permettent de restreindre ceci :

* **`disable-model-invocation: true`** : Seul vous pouvez invoquer la compétence. Utilisez ceci pour les flux de travail avec des effets secondaires ou que vous souhaitez contrôler le timing, comme `/commit`, `/deploy` ou `/send-slack-message`. Vous ne voulez pas que Claude décide de déployer parce que votre code semble prêt.

* **`user-invocable: false`** : Seul Claude peut invoquer la compétence. Utilisez ceci pour les connaissances de base qui ne sont pas actionnables en tant que commande. Une compétence `legacy-system-context` explique comment fonctionne un ancien système. Claude devrait le savoir lorsqu'il est pertinent, mais `/legacy-system-context` n'est pas une action significative pour les utilisateurs.

Cet exemple crée une compétence de déploiement que seul vous pouvez déclencher. Si vous définissez `disable-model-invocation: true`, Claude ne peut pas exécuter la compétence automatiquement :

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

Si Claude essaie quand même, Claude Code bloque l'appel et lui indique de ne pas reproduire les étapes de déploiement d'une autre manière, donc attendez-vous à ce que Claude vous suggère d'exécuter `/deploy` vous-même.

Voici comment les deux champs affectent l'invocation et le chargement du contexte :

| Frontmatter                      | Vous pouvez invoquer | Claude peut invoquer | Quand chargé en contexte                                                                    |
| :------------------------------- | :------------------- | :------------------- | :------------------------------------------------------------------------------------------ |
| (par défaut)                     | Oui                  | Oui                  | Description toujours en contexte, la compétence complète se charge lorsqu'elle est invoquée |
| `disable-model-invocation: true` | Oui                  | Non                  | Description pas en contexte, la compétence complète se charge lorsque vous l'invoquez       |
| `user-invocable: false`          | Non                  | Oui                  | Description toujours en contexte, la compétence complète se charge lorsqu'elle est invoquée |

<Note>
  Dans une session régulière, les descriptions de compétences sont chargées en contexte afin que Claude sache ce qui est disponible, mais le contenu complet de la compétence ne se charge que lorsqu'elle est invoquée. [Les sous-agents avec compétences préchargées](/docs/fr/sub-agents#preload-skills-into-subagents) fonctionnent différemment : le contenu complet de la compétence est injecté au démarrage.
</Note>

<h3 id="skill-content-lifecycle">
  Cycle de vie du contenu de la compétence
</h3>

Lorsque vous ou Claude invoquez une compétence, le contenu `SKILL.md` rendu entre dans la conversation en tant que message unique et y reste à travers les tours ultérieurs. Cette persistance s'applique aux instructions de la compétence, pas à ses permissions : une subvention [`allowed-tools`](#pre-approve-tools-for-a-skill) s'efface lorsque vous envoyez votre message suivant. Claude Code ne relit pas le fichier de compétence aux tours ultérieurs, donc écrivez les conseils qui devraient s'appliquer tout au long d'une tâche en tant qu'instructions permanentes plutôt que des étapes ponctuelles.

Lorsque Claude réinvoque une compétence dont le contenu rendu est identique à la copie déjà en contexte, Claude Code ajoute une courte note que la compétence est déjà chargée plutôt qu'une deuxième copie du contenu. Lorsque le contenu rendu diffère, parce que les arguments ont changé ou qu'une commande [contexte dynamique](#inject-dynamic-context) a produit une nouvelle sortie, Claude Code ajoute le contenu complet à nouveau.

[L'auto-compaction](/docs/fr/how-claude-code-works#when-context-fills-up) porte les compétences invoquées en avant dans un budget de tokens. Lorsque la conversation est résumée pour libérer du contexte, Claude Code réattache l'invocation la plus récente de chaque compétence après le résumé, en gardant les premiers 5 000 tokens de chacune. Les compétences réattachées partagent un budget combiné de 25 000 tokens. Claude Code remplit ce budget à partir de la compétence la plus récemment invoquée, donc les compétences plus anciennes peuvent être entièrement supprimées après la compaction si vous en avez invoqué beaucoup dans une session.

Si une compétence semble cesser d'influencer le comportement après la première réponse, le contenu est généralement toujours présent et le modèle choisit d'autres outils ou approches. Renforcez la `description` de la compétence et les instructions afin que le modèle continue à la préférer, ou utilisez [hooks](/docs/fr/hooks) pour appliquer le comportement de manière déterministe. Si la compétence est grande ou que vous en avez invoqué plusieurs après elle, réinvoquez-la après la compaction pour restaurer le contenu complet.

<h3 id="pre-approve-tools-for-a-skill">
  Pré-approuver les outils pour une compétence
</h3>

Le champ `allowed-tools` accorde la permission pour les outils listés lors du tour qui invoque la compétence, afin que Claude puisse les utiliser sans vous demander l'approbation. La subvention s'efface lorsque vous envoyez votre message suivant, même si le contenu de la compétence [reste en contexte](#skill-content-lifecycle) ; invoquer la compétence à nouveau réapplique pour ce tour. Il ne restreint pas les outils disponibles : chaque outil reste appelable, et vos [paramètres de permission](/docs/fr/permissions) gouvernent toujours les outils qui ne sont pas listés. Pour pré-approuver les outils pour la session entière plutôt qu'un seul tour, ajoutez plutôt des règles d'autorisation à ces paramètres de permission.

La confiance de l'espace de travail ne contrôle pas ce champ. Claude Code applique l'`allowed-tools` d'une compétence de projet chaque fois que vous ou Claude invoquez la compétence, y compris dans une exécution `-p` dans un dossier que vous n'avez jamais approuvé. Une compétence peut se donner un accès aux outils très large, donc examinez l'`allowed-tools` des compétences archivées dans un référentiel avant d'exécuter Claude Code là.

Cette compétence permet à Claude d'exécuter les commandes git sans approbation par utilisation chaque fois que vous l'invoquez :

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Pour supprimer les outils du pool disponible de Claude tandis qu'une compétence est active, listez-les dans `disallowed-tools` dans le frontmatter de la compétence. La restriction s'efface lorsque vous envoyez votre message suivant. Comme les règles de refus, le champ ne peut pas supprimer [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) tant que tout autre outil reste. Pour bloquer les outils à travers toutes les compétences et invites, ajoutez des règles de refus dans vos [paramètres de permission](/docs/fr/permissions).

<h3 id="pass-arguments-to-skills">
  Passer des arguments aux compétences
</h3>

Vous et Claude pouvez passer des arguments lors de l'invocation d'une compétence. Les arguments sont disponibles via l'espace réservé `$ARGUMENTS`.

Cette compétence corrige un problème GitHub par numéro. L'espace réservé `$ARGUMENTS` est remplacé par tout ce qui suit le nom de la compétence :

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

Lorsque vous exécutez `/fix-issue 123`, Claude reçoit « Fix GitHub issue 123 following our coding standards... »

Si vous invoquez une compétence avec des arguments mais qu'aucun espace réservé dans le contenu de la compétence n'en reçoit un, Claude Code ajoute `ARGUMENTS: <your input>` à la fin du contenu de la compétence afin que Claude voie toujours ce que vous avez tapé. Un espace réservé est `$ARGUMENTS`, une forme indexée comme `$1` ou un argument nommé. Un espace réservé indexé sans argument à sa position reste comme texte littéral et ne compte pas comme en recevant un. Un espace réservé nommé compte même lorsque sa position n'a pas d'argument, car il se développe à une chaîne vide.

Vous pouvez également empiler plusieurs compétences au début d'un message. Taper `/write-tests /fix-issue 123` charge les deux compétences et passe le texte final `123` comme `$ARGUMENTS` à chacune d'elles. Avant v2.1.199, seule la première compétence se chargeait et recevait `/fix-issue 123` comme texte d'argument littéral.

Claude Code étend la première compétence plus jusqu'à cinq autres empilées après elle. L'expansion s'arrête au premier jeton qui n'est pas une compétence invocable par l'utilisateur en ligne, donc une compétence qui s'exécute en tant que [sous-agent forké](#run-skills-in-a-subagent), comme [`/code-review`](/docs/fr/code-review#review-a-diff-locally), ou une dont les arguments peuvent eux-mêmes commencer par une commande slash, comme `/loop`, termine également la course là. Ce jeton et tout ce qui le suit deviennent le texte d'argument pour chaque compétence étendue. `/code-review` s'exécute en tant que sous-agent forké à partir de v2.1.218 ; sur les versions antérieures, il s'exécutait en ligne et s'empilait.

Pour accéder aux arguments individuels par position, utilisez `$ARGUMENTS[N]` ou le plus court `$N` :

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

L'exécution de `/migrate-component SearchBar JavaScript TypeScript` remplace `$ARGUMENTS[0]` par `SearchBar`, `$ARGUMENTS[1]` par `JavaScript` et `$ARGUMENTS[2]` par `TypeScript`. La même compétence utilisant le raccourci `$N` :

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
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

La syntaxe `` !`<command>` `` exécute des commandes shell avant que le contenu de la compétence soit envoyé à Claude. La sortie de la commande remplace l'espace réservé, de sorte que Claude reçoit des données réelles, pas la commande elle-même. Claude Code n'exécute pas ces commandes sur votre machine lorsque la compétence est [synchronisée depuis votre compte claude.ai](#how-claude-code-handles-the-body-of-a-synced-skill). Cette restriction nécessite Claude Code v2.1.228 ou version ultérieure.

Cette compétence résume une demande de tirage en récupérant les données de PR en direct avec GitHub CLI. Les commandes `` !`gh pr diff` `` et autres s'exécutent en premier, et leur sortie est insérée dans l'invite :

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

La substitution s'exécute une seule fois sur le fichier d'origine. La sortie de la commande est insérée en tant que texte brut et n'est pas réanalysée pour d'autres espaces réservés `` !`<command>` ``, de sorte qu'une commande ne peut pas émettre un espace réservé pour qu'une passe ultérieure l'étende.

La forme en ligne n'est reconnue que lorsque `!` apparaît au début d'une ligne ou immédiatement après un espace blanc. Si `!` suit un autre caractère, comme dans `` KEY=!`cmd` ``, l'espace réservé est laissé en tant que texte littéral et la commande ne s'exécute pas.

Pour les commandes multi-lignes, utilisez un bloc de code délimité ouvert avec ` ```! ` au lieu de la forme en ligne :

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

Pour désactiver ce comportement pour les compétences et les commandes personnalisées provenant de sources utilisateur, projet, plugin ou [répertoire supplémentaire](#skills-from-additional-directories), définissez `"disableSkillShellExecution": true` dans [settings](/docs/fr/settings). Chaque commande est remplacée par `[shell command execution disabled by policy]` au lieu d'être exécutée. Les compétences groupées et gérées ne sont pas affectées. Ce paramètre est très utile dans [managed settings](/docs/fr/managed-settings), où les utilisateurs ne peuvent pas le remplacer.

Claude Code n'exécute jamais ces commandes sur votre machine lorsqu'elles apparaissent dans les compétences [synchronisées depuis votre compte claude.ai](#how-synced-skills-behave), quel que soit ce paramètre. Cette restriction nécessite Claude Code v2.1.228 ou version ultérieure. [How Claude Code handles the body of a synced skill](#how-claude-code-handles-the-body-of-a-synced-skill) indique ce que Claude reçoit à la place de la commande dans chaque type de session.

<Tip>
  Pour demander un raisonnement plus approfondi lorsqu'une compétence s'exécute, incluez `ultrathink` n'importe où dans le contenu de la compétence. Voir [Use ultrathink for one-off deep reasoning](/docs/fr/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h4 id="how-injected-commands-run">
  Comment les commandes injectées s'exécutent
</h4>

Claude Code choisit l'outil qui exécute les commandes injectées d'une compétence à partir de la clé `shell` dans le frontmatter de la compétence et de votre environnement. Chaque combinaison exécute les commandes via l'outil Bash ou l'outil PowerShell, sauf une qui échoue l'invocation directement :

* `shell: powershell`, avec l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool) activé : les commandes s'exécutent via l'outil PowerShell.
* `shell: bash` lorsque bash n'est pas disponible : l'invocation échoue avant l'exécution de toute commande. Cela se produit sur Windows sans Git Bash. Claude Code affiche ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``.
* Toute autre combinaison : les commandes s'exécutent via l'outil Bash lorsque bash est disponible. Sinon, elles s'exécutent via l'outil PowerShell.

L'un ou l'autre outil exécute les commandes de la même manière qu'il exécute les propres commandes shell de Claude. Ils partagent le répertoire de travail, le délai d'expiration et la gestion de la sortie :

* **Répertoire de travail** : Claude Code exécute chaque commande dans le répertoire de travail actuel du shell de la session. Ce répertoire se déplace lorsque Claude exécute `cd`. Utilisez [`${CLAUDE_SKILL_DIR}` ou `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) dans les chemins qui doivent se résoudre de la même manière à chaque fois.
* **stderr** : avec le shell `bash` par défaut, Claude Code fusionne stderr dans stdout. Tout ce que la commande écrit dans stderr apparaît dans le texte injecté.
* **Délai d'expiration** : chaque commande s'exécute sous le [délai d'expiration](/docs/fr/tools-reference#timeout-and-output-limits) par défaut de 2 minutes de l'outil Bash. Lorsque l'outil Bash [déplace une commande expirée en arrière-plan](/docs/fr/tools-reference#background-commands), la compétence s'affiche toujours. Le texte injecté signale le déplacement et nomme la tâche en arrière-plan et le fichier collectant la sortie de la commande. Lorsque la commande est une que l'outil Bash ne met jamais automatiquement en arrière-plan, Claude Code la tue au délai d'expiration. Cet échec [abandonne l'invocation](#when-an-injected-command-fails).
* **Taille de la sortie** : la sortie au-delà du plafond en ligne de l'outil Bash arrive sous la forme d'un chemin de fichier plus un court aperçu, pas du texte tronqué. [Output limits](/docs/fr/tools-reference#output-limits) couvre le plafond et comment ajuster chaque limite.

L'outil PowerShell applique le même comportement de délai d'expiration, de mise en arrière-plan et de plafond de sortie aux commandes qu'il exécute. Voir la section [outil PowerShell](/docs/fr/tools-reference#powershell-tool) pour ses spécificités.

<h4 id="when-an-injected-command-fails">
  Quand une commande injectée échoue
</h4>

Une commande échouée abandonne l'invocation de compétence entière, pas seulement son propre espace réservé. Claude ne voit jamais le contenu de la compétence pour cette invocation. L'abandon affiche `Shell command failed for pattern "..."`. Le message d'erreur inclut la sortie de la commande sous `[stderr]`.

Avec le shell `bash` par défaut, tout code de sortie non nul compte comme un échec. Une exception s'applique : Claude Code traite le code de sortie 1 des [commandes de recherche et de comparaison](/docs/fr/tools-reference#output-limits) comme un résultat normal et injecte leur sortie. Les codes de sortie de 2 ou plus échouent même pour ces commandes.

Les commandes qui obtiennent l'exception dépendent du shell :

* Shell `bash` par défaut : les commandes listées sous [Output limits](/docs/fr/tools-reference#output-limits)
* `shell: powershell`, lorsque l'outil PowerShell est activé : un [ensemble différent](/docs/fr/tools-reference#shell-selection-in-settings-hooks-and-skills) qui inclut `grep` et `git diff` mais pas `find` ou `diff`

Avec le shell `bash` par défaut, ajoutez `|| true` à toute autre commande que vous vous attendez à quitter non-zéro. Un script de vérification qui quitte 1 lorsqu'il trouve des problèmes en est un exemple.

<h4 id="permission-checks-on-injected-commands">
  Vérifications des permissions sur les commandes injectées
</h4>

Les commandes injectées ne demandent jamais la permission pendant que la compétence s'affiche. Claude Code vérifie chacune d'elles par rapport à vos [règles de permission](/docs/fr/permissions) en premier. Une commande qu'une règle de refus correspond abandonne l'invocation avec `Shell command permission check failed for pattern "..."`.

En dehors du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), lorsque la vérification des permissions d'une commande retourne autre chose que l'autorisation, Claude Code abandonne l'invocation avec la même erreur. Cela inclut une règle qui vous demanderait normalement. Pour empêcher une commande non appariée d'abandonner ici, pré-approuvez-la avec [`allowed-tools`](#pre-approve-tools-for-a-skill). Les règles de refus et d'ask remplacent toujours `allowed-tools`. Voir [Manage permissions](/docs/fr/permissions#manage-permissions).

En mode auto, une commande qui aurait autrement besoin de votre approbation n'abandonne pas l'invocation. La compétence se charge avec une instruction indiquant à Claude d'exécuter d'abord la commande, et l'appel propre de Claude passe ensuite par les [vérifications habituelles du mode auto](/docs/fr/permission-modes#how-the-classifier-evaluates-actions). L'invocation abandonne toujours dans une [compétence forkée](#run-skills-in-a-subagent) qui définit `agent`, et dans une session où Claude n'a pas l'[outil shell qui exécute les commandes injectées](#how-injected-commands-run).

<h3 id="run-skills-in-a-subagent">
  Exécuter les compétences dans un sous-agent
</h3>

Ajoutez `context: fork` à votre frontmatter lorsque vous souhaitez qu'une compétence s'exécute en isolation. Claude Code démarre un nouveau sous-agent du type défini dans le champ `agent` et lui donne le contenu de la compétence comme invite. Le sous-agent ne voit pas votre historique de conversation, de sorte que les instructions de la compétence doivent être autonomes.

<Note>
  Malgré le nom, une compétence avec `context: fork` ne s'exécute pas dans un [fork de la conversation actuelle](/docs/fr/sub-agents#fork-the-current-conversation), ce qui remettrait au sous-agent tout ce que vous avez discuté jusqu'à présent. Lorsque la tâche dépend de cet historique, forkez la conversation au lieu d'utiliser `context: fork`.
</Note>

Le sous-agent forké s'exécute en [arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) : vous continuez à travailler pendant qu'il s'exécute, et son résultat arrive dans votre conversation lorsqu'il se termine. Définissez `background: false` dans le frontmatter pour attendre le résultat dans le tour qui a invoqué la compétence. Avant v2.1.218, les compétences forkées bloquaient toujours le tour jusqu'à ce qu'elles se terminent.

Claude Code attend également le résultat, même lorsque la compétence ne définit pas `background: false`, dans des cas comme ceux-ci :

* En mode non-interactif, avec le drapeau `-p` ou le SDK Agent
* Lorsque vous définissez [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/fr/env-vars) sur `1`, ce qui désactive également toutes les autres fonctionnalités de tâche en arrière-plan
* Lorsque vous invoquez une compétence forkée alors qu'une invocation antérieure de la même compétence s'exécute toujours
* Lorsqu'une [tâche planifiée](/docs/fr/scheduled-tasks) se déclenche avec la compétence comme invite

Un fork en arrière-plan s'exécute également avec l'[ensemble d'outils plus restreint qui s'applique aux sous-agents en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) : le sous-agent de la compétence est un type d'agent régulier, de sorte que l'exemption pour les sous-agents qui forkent la conversation ne le couvre pas. Si les étapes de votre compétence dépendent d'un outil en dehors de cet ensemble, définissez `background: false` pour conserver l'ensemble d'outils complet.

Une compétence forkée qui s'exécute en arrière-plan applique ses modifications en dehors des [points de contrôle](/docs/fr/checkpointing) de votre session, de sorte que `/rewind` ne les annule pas ; utilisez git pour les annuler.

<Warning>
  `context: fork` n'a de sens que pour les compétences avec des instructions explicites. Si votre compétence contient des directives comme « utiliser ces conventions API » sans tâche, le sous-agent reçoit les directives mais aucune invite exploitable, et retourne sans sortie significative.
</Warning>

Les compétences et les [sous-agents](/docs/fr/sub-agents) travaillent ensemble dans deux directions :

| Approche                        | Invite système               | Tâche                           | Charge également                                                                                                          |
| :------------------------------ | :--------------------------- | :------------------------------ | :------------------------------------------------------------------------------------------------------------------------ |
| Compétence avec `context: fork` | Du type d'agent              | Contenu SKILL.md                | CLAUDE.md, selon le [contexte de démarrage](/docs/fr/sub-agents#what-loads-at-startup) de l'agent                              |
| Sous-agent avec champ `skills`  | Corps markdown du sous-agent | Message de délégation de Claude | Compétences préchargées + CLAUDE.md, selon le [contexte de démarrage](/docs/fr/sub-agents#what-loads-at-startup) du sous-agent |

Avec `context: fork`, vous écrivez la tâche dans votre compétence et choisissez un type d'agent pour l'exécuter. Les agents Explore et Plan intégrés [ignorent CLAUDE.md et l'état git](/docs/fr/sub-agents#what-loads-at-startup) pour garder leur contexte petit, de sorte qu'une compétence forkée utilisant `agent: Explore` ne voit que le contenu SKILL.md et l'invite système propre de l'agent. Pour l'inverse, où vous définissez un sous-agent personnalisé qui utilise les compétences comme matériel de référence, voir [Subagents](/docs/fr/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Exemple : Compétence de recherche utilisant l'agent Explore
</h4>

Cette compétence exécute la recherche dans un agent Explore forké. Le contenu de la compétence devient la tâche, et l'agent fournit des outils en lecture seule optimisés pour l'exploration de la base de code :

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

Lorsque cette compétence s'exécute :

1. Un nouveau contexte isolé est créé
2. Le sous-agent reçoit le contenu de la compétence comme son invite (les instructions « Research \$ARGUMENTS thoroughly »)
3. Le champ `agent` détermine l'environnement d'exécution (modèle, outils et permissions)
4. Le sous-agent résume ses résultats et les retourne à votre conversation principale lorsqu'il se termine

Le champ `agent` spécifie quelle configuration de sous-agent utiliser. Les options incluent les agents intégrés (`Explore`, `Plan`, `general-purpose`) ou tout sous-agent personnalisé de `.claude/agents/`. S'il est omis, utilise `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Restreindre l'accès aux compétences de Claude
</h3>

Par défaut, Claude peut invoquer n'importe quelle compétence qui n'a pas `disable-model-invocation: true` défini. Les compétences qui définissent `allowed-tools` accordent à Claude l'accès à ces outils sans approbation par utilisation pendant le tour qui invoque la compétence ; l'octroi s'efface lorsque vous envoyez votre message suivant. Vos [paramètres de permission](/docs/fr/permissions) régissent toujours le comportement d'approbation de base pour tous les autres outils. Quelques commandes intégrées sont également disponibles via l'outil Skill, notamment `/init` et `/security-review`. D'autres commandes intégrées telles que `/compact` ne le sont pas.

Trois façons de contrôler les compétences que Claude peut invoquer :

**Désactiver toutes les compétences** en refusant l'outil Skill dans `/permissions` :

```text theme={null}
# Add to deny rules:
Skill
```

**Autoriser ou refuser des compétences spécifiques** en utilisant les [règles de permission](/docs/fr/permissions) :

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Syntaxe de permission : `Skill(name)` pour correspondance exacte, `Skill(name *)` pour correspondance de préfixe avec n'importe quels arguments.

Si votre règle `deny` nomme un alias ou un nom non qualifié plutôt que le nom propre de la compétence, Claude Code bloque toujours la compétence : avec `Skill(review)` il bloque la compétence groupée `/code-review` via son alias `/review`, et avec `Skill(deploy)` il bloque une [compétence imbriquée](#where-skills-live) listée comme `apps/web:deploy` via son nom non qualifié. Avant v2.1.260, Claude Code ne bloquait pas une compétence imbriquée listée sous son nom qualifié lorsque la règle deny nommait uniquement le nom non qualifié.

Claude Code correspond à une règle `allow` uniquement contre le nom propre de la compétence et le nom dans l'invocation de Claude.

**Masquer les compétences individuelles** en ajoutant `disable-model-invocation: true` à leur frontmatter. Cela supprime la compétence du contexte de Claude entièrement.

<Note>
  Avec `user-invocable: false`, vous ne pouvez pas invoquer la compétence, mais Claude le peut. Pour empêcher Claude de l'invoquer via l'outil Skill, définissez `disable-model-invocation: true`.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Remplacer la visibilité des compétences à partir des paramètres
</h3>

Le paramètre `skillOverrides` contrôle la visibilité des compétences à partir de vos [paramètres](/docs/fr/settings) au lieu du frontmatter propre de la compétence. Utilisez-le pour les compétences dont SKILL.md vous ne voulez pas éditer, comme celles archivées dans un référentiel de projet partagé. Le menu `/skills` l'écrit pour vous : mettez en surbrillance une compétence et appuyez sur `Space` pour parcourir les états, puis `Esc` pour enregistrer dans `.claude/settings.local.json`.

Chaque clé est un nom de compétence et chaque valeur est l'un des quatre états :

| Valeur                  | Listé à Claude     | Dans le menu `/` |
| :---------------------- | :----------------- | :--------------- |
| `"on"`                  | Nom et description | Oui              |
| `"name-only"`           | Nom uniquement     | Oui              |
| `"user-invocable-only"` | Masqué             | Oui              |
| `"off"`                 | Masqué             | Masqué           |

Le menu `/skills` étiquette l'état `"user-invocable-only"` `user-only`.

À partir de v2.1.199, `"off"` masque également la compétence des listes de commandes annoncées aux clients [Remote Control](/docs/fr/remote-control) et aux appelants [Agent SDK](/docs/fr/agent-sdk/skills#discover-available-commands), en plus du menu terminal `/`. Invoquer une compétence masquée par son nom complet retourne toujours l'erreur `skillOverrides` au lieu de l'exécuter.

Une compétence absente de `skillOverrides` est traitée comme `"on"`. L'exemple ci-dessous réduit une compétence à son nom et en désactive une autre entièrement :

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Certaines compétences groupées ont des alias, comme `checkup` pour `/doctor`. Si vous définissez une entrée `skillOverrides` sous un alias dans [managed settings](/docs/fr/managed-settings) ou dans un fichier que vous transmettez avec le drapeau `--settings`, Claude Code l'applique à la compétence derrière l'alias. Vous ne pouvez restreindre une compétence que davantage via un alias, jamais la rendre plus visible, et si vous définissez également une entrée sous le nom propre de la compétence dans managed settings, cette entrée prend la priorité. Avant v2.1.260, Claude Code n'appliquait pas une entrée sous un alias à la compétence dans aucune source de paramètres.

Dans les paramètres utilisateur, projet et local, Claude Code correspond aux entrées uniquement contre les noms de compétences. Si vous définissez une entrée pour `review` là, elle s'applique à une compétence nommée `review`, pas à la compétence groupée `/code-review` via son alias `/review`.

Les compétences de plugin ne sont pas affectées par `skillOverrides`. Gérez-les via `/plugin` à la place.

<h3 id="find-unused-skills">
  Trouver les compétences inutilisées
</h3>

Chaque compétence dans l'[énumération des compétences](#skill-descriptions-are-cut-short) ajoute à votre contexte à chaque tour, que Claude l'utilise ou non. Exécutez `/skill-doctor` pour voir ce que chacune de vos compétences coûte et à quelle fréquence elle est utilisée, afin que vous puissiez décider lesquelles désactiver. Dans une session interactive, le rapport s'ouvre dans l'onglet **Stats** du gestionnaire `/plugin`. En [mode non-interactif](/docs/fr/headless) avec `-p`, Claude Code l'imprime en tant que texte.

Le rapport couvre les compétences de votre session autres que les compétences groupées et les compétences d'entreprise. Il signale les compétences dans l'énumération qui n'ont jamais été invoquées et indique où les désactiver. Parmi les compétences qu'il vous dit où désactiver, commencez par celles qui ont le coût de contexte le plus élevé. Le rapport liste également les plugins que vous n'avez pas utilisés récemment.

`/skill-doctor` nécessite Claude Code v2.1.252 ou version ultérieure et n'est pas disponible dans les sessions qui ignorent [feature-flag fetching](/docs/fr/env-vars#features-that-need-feature-flag-fetching). Si vous exécutez `/skill-doctor` sur [Remote Control](/docs/fr/remote-control) depuis votre téléphone ou navigateur, Claude Code répond [`Skill usage reports are not available on this connection.`](/docs/fr/errors#skill-usage-reports-are-not-available-on-this-connection) à la place. Exécutez `/skill-doctor` dans le terminal sur la machine où la session s'exécute.

<h2 id="evaluate-and-iterate-on-a-skill">
  Évaluer et itérer sur une compétence
</h2>

Voir une compétence se déclencher vous indique que Claude l'a trouvée, pas qu'elle a fait ce que vous aviez l'intention. Pour savoir qu'une compétence fonctionne, mesurez séparément si Claude l'invoque sur les invites qu'elle devrait, et si la sortie correspond à ce que vous attendez quand elle le fait.

La vérification des deux est une comparaison de base. Collectez quelques invites réalistes, exécutez chacune dans une session nouvelle avec la compétence disponible et à nouveau avec elle [désactivée](#override-skill-visibility-from-settings), et comparez les résultats. Une session nouvelle est importante car le contexte restant de la création de la compétence masquera les lacunes dans les instructions écrites.

Deux outils automatisent cette comparaison. Pour une compétence qui est livrée dans un [plugin](/docs/fr/plugins/overview), [`claude plugin eval`](/docs/fr/plugin-evals) exécute chaque invite dans une session isolée avec et sans le plugin, la note avec des évaluateurs que vous définissez ou qu'il écrit pour vous, et quitte avec un code non-zéro en dessous d'un seuil afin que vous puissiez gater CI sur celui-ci. Pour itérer sur une seule compétence à l'intérieur d'une conversation Claude Code, le plugin skill-creator ci-dessous exécute une boucle similaire avec son propre format `evals/evals.json`. Les deux formats ne sont pas interchangeables.

<h3 id="run-evals-with-skill-creator">
  Exécuter des évaluations avec skill-creator
</h3>

Le [plugin `skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatise la boucle de comparaison à l'intérieur de Claude Code. Installez-le depuis la marketplace officielle :

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Si l'installation échoue, faites correspondre le message que Claude Code signale :

* `Marketplace "claude-plugins-official" not found` : ajoutez la marketplace avec `/plugin marketplace add anthropics/claude-plugins-official`, puis réessayez l'installation.
* Le plugin [n'est pas trouvé dans la marketplace](/docs/fr/plugins/install#install-a-plugin) : vérifiez le nom du plugin.

Si le résumé d'installation signale `Run /reload-plugins to activate.`, Claude Code exécute ensuite ce rechargement pour vous. Si le rechargement vous avertit que votre prochain message relierait la conversation, exécutez `/reload-plugins --force` pour rendre les compétences du plugin disponibles dans la session actuelle. Ensuite, demandez à Claude d'évaluer une compétence existante, par exemple `evaluate my summarize-changes skill with skill-creator`. Le plugin vous guide à travers l'écriture de cas de test et exécute la boucle :

* **Cas de test** : stocke les invites, les fichiers d'entrée et le comportement attendu dans `evals/evals.json` à l'intérieur du répertoire de compétence
* **Exécutions isolées** : génère un [sous-agent](/docs/fr/sub-agents) par cas de test afin que chaque exécution commence avec un contexte propre, et enregistre le nombre de tokens et la durée
* **Notation** : vérifie chaque assertion par rapport à la sortie et écrit réussi ou échoué avec des preuves dans `grading.json`
* **Benchmark** : agrège le taux de réussite, le temps et les tokens pour avec-compétence par rapport à sans-compétence dans `benchmark.json` afin que vous puissiez comparer l'amélioration du taux de réussite par rapport à la surcharge de tokens et de temps
* **Comparaison de versions** : exécute un A/B en aveugle entre deux versions de la compétence afin que vous puissiez confirmer qu'une modification est une amélioration avant de la valider
* **Ajustement de description** : génère des invites should-trigger et should-not-trigger, mesure le taux de réussite, et propose des modifications de description quand la compétence s'active sur les mauvaises demandes
* **Visionneuse d'examen** : ouvre un rapport HTML où vous inspectez chaque sortie et enregistrez les commentaires qualitatifs que l'itération suivante lit

Pour le format du fichier d'évaluation et le flux de travail d'itération complet, consultez [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) sur agentskills.io. Pour des informations générales sur les modes benchmark et comparaison, consultez l'[annonce skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Partager des compétences
</h2>

Les compétences peuvent être distribuées à différentes portées selon votre audience :

* **Compétences de projet** : Validez `.claude/skills/` dans le contrôle de version
* **Plugins** : Créez un répertoire `skills/` dans votre [plugin](/docs/fr/plugins/overview)
* **Gérées** : Déployez à l'échelle de l'organisation via les [paramètres gérés](/docs/fr/managed-settings)

<h3 id="generate-visual-output">
  Générer une sortie visuelle
</h3>

Les compétences peuvent regrouper et exécuter des scripts dans n'importe quel langage, donnant à Claude des capacités au-delà de ce qui est possible dans une seule invite. Un modèle courant est la génération de sortie visuelle : des fichiers HTML interactifs qui s'ouvrent dans votre navigateur pour explorer les données, déboguer ou créer des rapports.

Cet exemple crée un explorateur de base de code : une vue arborescente interactive où vous pouvez développer et réduire les répertoires, voir les tailles de fichiers en un coup d'œil et identifier les types de fichiers par couleur.

Créez le répertoire Skill :

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Enregistrez ceci dans `~/.claude/skills/codebase-visualizer/SKILL.md`. La description indique à Claude quand activer cette Skill, et les instructions indiquent à Claude d'exécuter le script fourni. Le chemin du script utilise [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) pour qu'il se résolve correctement que la compétence soit installée au niveau personnel, du projet ou du plugin :

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

* Une **barre latérale de résumé** affichant le nombre de fichiers, le nombre de répertoires, la taille totale et le nombre de types de fichiers
* Un **graphique en barres** ventilant la base de code par type de fichier (top 8 par taille)
* Un **arbre réductible** où vous pouvez développer et réduire les répertoires, avec des indicateurs de type de fichier codés par couleur

Le script nécessite Python 3 mais utilise uniquement des bibliothèques intégrées, il n'y a donc aucun paquet à installer :

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

Pour tester, ouvrez Claude Code dans n'importe quel projet et demandez « Visualize this codebase. » Claude exécute le script, qui affiche le chemin du fichier généré, tel que `Generated /path/to/codebase-map.html`, et l'ouvre dans votre navigateur. Si vous travaillez dans un environnement sans interface graphique où aucun navigateur ne s'ouvre, le chemin affiché confirme que le script a réussi.

Ce modèle fonctionne pour toute sortie visuelle : graphiques de dépendances, rapports de couverture de test, documentation API ou visualisations de schémas de base de données. Le script fourni fait le travail tandis que Claude gère l'orchestration.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="skill-not-triggering">
  La compétence ne se déclenche pas
</h3>

Si Claude n'utilise pas votre compétence quand prévu :

1. Vérifiez que la description inclut les mots-clés que les utilisateurs diraient naturellement
2. Vérifiez que la compétence apparaît dans `What skills are available?`
3. Essayez de reformuler votre demande pour correspondre plus étroitement à la description
4. Invoquez-la directement avec `/skill-name` si la compétence est invocable par l'utilisateur

Si le YAML du frontmatter est malformé, Claude Code charge le corps de la compétence avec des métadonnées vides, donc `/skill-name` fonctionne toujours mais Claude ne peut pas faire correspondre votre `description`. Exécutez avec `--debug` pour voir l'erreur d'analyse.

Si la compétence est fournie dans un plugin, vous pouvez mesurer la fréquence à laquelle elle se déclenche sur des invites réalistes plutôt que de vérifier une par une : écrivez un cas d'évaluation avec un [évaluateur `tool_used: Skill`](/docs/fr/plugin-evals#create-your-first-eval-suite) et exécutez-le avec `claude plugin eval` après chaque modification de description.

Pour trouver les fichiers `SKILL.md` dont le frontmatter ne s'analyse pas, exécutez [`claude plugin validate`](/docs/fr/plugins/cli-reference#validate-a-directory) sur le répertoire des compétences, par exemple `claude plugin validate .claude/skills` pour les compétences du projet ou `claude plugin validate ~/.claude/skills` pour les compétences personnelles. Nécessite Claude Code v2.1.233 ou ultérieur.

<h3 id="skill-triggers-too-often">
  La compétence se déclenche trop souvent
</h3>

Si Claude utilise votre compétence quand vous ne le souhaitez pas :

1. Rendez la description plus spécifique
2. Ajoutez `disable-model-invocation: true` si vous ne voulez que l'invocation manuelle

<h3 id="skill-descriptions-are-cut-short">
  Les descriptions de compétences sont tronquées
</h3>

Claude Code charge une liste de noms et descriptions de compétences dans le contexte pour que Claude sache ce qui est disponible. La liste contient toujours tous les noms de compétences, mais si vous avez de nombreuses compétences, Claude Code raccourcit les descriptions pour s'adapter au budget de caractères de la liste, ce qui peut supprimer les mots-clés dont Claude a besoin pour faire correspondre votre demande. Le budget s'adapte à 1 % de la fenêtre de contexte du modèle. Quand la liste dépasse le budget, Claude Code supprime les descriptions en commençant par les compétences que vous invoquez le moins, de sorte que les compétences que vous utilisez le plus conservent leur texte complet.

Exécutez `/doctor` pour une estimation du coût contextuel de la liste et de ses plus grands contributeurs. Pour trouver les compétences qui valent la peine d'être désactivées, exécutez [`/skill-doctor`](#find-unused-skills). Quand la liste dépasse son budget, Claude Code écrit également un avertissement dans le journal de débogage, visible avec [`--debug`](/docs/fr/cli-reference#cli-flags).

La ligne Skills dans `/context` rapporte la taille de la liste après l'application du budget, de sorte qu'elle correspond à ce que le modèle reçoit. Avant v2.1.196, la ligne comptait le texte complet de chaque description et pouvait afficher une valeur plusieurs fois plus grande que le budget configuré.

Pour augmenter le budget, définissez le paramètre [`skillListingBudgetFraction`](/docs/fr/settings-reference#skilllistingbudgetfraction) (par exemple `0.02` = 2 %) ou la variable d'environnement `SLASH_COMMAND_TOOL_CHAR_BUDGET` sur un nombre de caractères fixe. Pour libérer du budget pour d'autres compétences, définissez les entrées de faible priorité sur `"name-only"` dans [`skillOverrides`](#override-skill-visibility-from-settings) afin qu'elles s'affichent sans description. Vous pouvez également réduire le texte `description` et `when_to_use` à la source : mettez le cas d'utilisation clé en premier, car le texte combiné de chaque entrée est limité à 1 536 caractères quel que soit le budget. Le plafond est configurable avec [`skillListingMaxDescChars`](/docs/fr/settings-reference#skilllistingmaxdescchars).

<h3 id="personal-skills-disappeared">
  Les compétences personnelles ont disparu
</h3>

Si les dossiers de compétences que vous avez créés dans `~/.claude/skills/` ont disparu, regardez dans `~/.claude/skills/.trash/`. Quand Claude Code [synchronise les compétences depuis claude.ai](#how-synced-skills-behave), il les télécharge dans le sous-dossier `synced` séparé et ne déplace ni ne supprime les dossiers que vous créez.

Avant v2.1.280, un fichier nommé `manifest.json` dans `~/.claude/skills/` causait à Claude Code de déplacer les dossiers de compétences que ce fichier listait dans un dossier horodaté sous `~/.claude/skills/.trash/`, et ces compétences cessaient de se charger.

Pour restaurer une compétence, déplacez son dossier du dossier horodaté vers `~/.claude/skills/`. Faites cela avant que le [balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically) supprime les entrées de la corbeille, par défaut 30 jours après leur déplacement vers la corbeille.

<h2 id="related-resources">
  Ressources connexes
</h2>

* **[Déboguer votre configuration](/docs/fr/debug-your-config)** : diagnostiquer pourquoi une skill n'apparaît pas ou ne se déclenche pas
* **[Évaluer la qualité de la sortie de la skill](https://agentskills.io/skill-creation/evaluating-skills)** : le format du fichier eval et le flux de travail d'itération sur agentskills.io
* **[Meilleures pratiques de création de skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)** : conseils de rédaction qui s'appliquent à tous les produits Claude
* **[Subagents](/docs/fr/sub-agents)** : déléguer les tâches à des agents spécialisés
* **[Plugins](/docs/fr/plugins/overview)** : empaqueter et distribuer les skills avec d'autres extensions
* **[Hooks](/docs/fr/hooks)** : automatiser les workflows autour des événements d'outils
* **[Memory](/docs/fr/memory)** : gérer les fichiers CLAUDE.md pour le contexte persistant
* **[Commands](/docs/fr/commands)** : référence pour les commandes intégrées et les skills groupées
* **[Permissions](/docs/fr/permissions)** : contrôler l'accès aux outils et aux skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)** : les skills de projet validées dans un repo se chargent également lorsque ce repo est utilisé dans un canal Claude Tag
