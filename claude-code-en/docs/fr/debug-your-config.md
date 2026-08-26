> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Déboguer votre configuration

> Diagnostiquez pourquoi CLAUDE.md, les paramètres, les hooks, les serveurs MCP ou les skills ne prennent pas effet. Utilisez /context, /doctor, /hooks et /mcp pour voir ce qui a réellement été chargé.

Quand Claude ignore une instruction ou qu'une fonctionnalité que vous avez configurée n'apparaît pas, la cause est généralement que le fichier n'a pas été chargé, qu'il a été chargé depuis un emplacement différent de celui que vous attendiez, ou qu'un autre fichier l'a remplacé. Ce guide montre comment inspecter ce que Claude Code a réellement chargé afin que vous puissiez déterminer lequel de ces cas s'applique.

Pour les problèmes d'installation, d'authentification et de connectivité, consultez plutôt [Troubleshooting installation and login](/docs/fr/troubleshoot-install) à la place.

<h2 id="see-what-loaded-into-context">
  Voir ce qui a été chargé dans le contexte
</h2>

La commande `/context` affiche tout ce qui occupe la fenêtre de contexte pour la session actuelle, ventilé par catégorie : invite système, fichiers de mémoire, skills, sous-agents personnalisés avec la source de chacun, outils MCP et messages de conversation. Exécutez-la d'abord pour confirmer si vos fichiers `CLAUDE.md`, règles ou descriptions de skills sont présents.

Pour plus de détails sur une catégorie spécifique, suivez avec la commande dédiée :

| Commande         | Affiche                                                                                                                                                                                                                                                                                                  |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/memory`        | Quels fichiers `CLAUDE.md` et règles ont été chargés, plus les entrées de mémoire automatique                                                                                                                                                                                                            |
| `/skills`        | Les skills disponibles provenant des sources de projet, utilisateur et plugin                                                                                                                                                                                                                            |
| `/hooks`         | Les configurations de hooks actives                                                                                                                                                                                                                                                                      |
| `/mcp`           | Les serveurs MCP connectés et leur statut                                                                                                                                                                                                                                                                |
| `/permissions`   | Les règles d'autorisation et de refus résolues actuellement en vigueur                                                                                                                                                                                                                                   |
| `/doctor`        | Diagnostics de configuration : santé de l'installation, fichiers de paramètres invalides, extensions inutilisées, noms de [sous-agents](/docs/fr/sub-agents) en doublon dans le même répertoire, et contenu `CLAUDE.md` enregistré que Claude peut dériver de la base de code, avec des corrections proposées |
| `/debug [issue]` | Active la journalisation de débogage pour la session et invite Claude à diagnostiquer en utilisant la sortie du journal et les chemins de paramètres                                                                                                                                                     |
| `/status`        | Les sources de paramètres actives, y compris si les paramètres gérés sont en vigueur                                                                                                                                                                                                                     |

Si un fichier de mémoire est absent de `/memory`, vérifiez son emplacement par rapport à [comment les fichiers CLAUDE.md se chargent](/docs/fr/memory#how-claude-md-files-load). Les fichiers `CLAUDE.md` des sous-répertoires se chargent à la demande quand Claude lit un fichier dans ce répertoire avec l'outil Read, pas au démarrage de la session.

Si `/memory` confirme que le fichier a été chargé mais que Claude ne suit toujours pas une instruction particulière, le problème est probablement la façon dont l'instruction est écrite plutôt que si elle a été chargée. CLAUDE.md fonctionne bien pour le type de conseils que vous donneriez à un nouveau coéquipier, comme les conventions de projet, les commandes de compilation et l'emplacement des fichiers.

L'adhérence diminue quand une instruction est assez vague pour être interprétée de plusieurs façons, quand deux fichiers donnent des directives conflictuelles, ou quand le fichier est devenu assez long pour que les règles individuelles reçoivent moins d'attention. [Écrire des instructions efficaces](/docs/fr/memory#write-effective-instructions) couvre les modèles de spécificité, de taille et de structure qui maintiennent l'adhérence élevée.

<Note>
  CLAUDE.md et les permissions résolvent des problèmes différents. CLAUDE.md dit à Claude comment fonctionne votre projet afin qu'il prenne de bonnes décisions. [Permissions](/docs/fr/permissions) et [hooks](/docs/fr/hooks) appliquent des limites indépendamment de ce que Claude décide. Utilisez CLAUDE.md pour « nous le faisons de cette façon ici ». Utilisez les permissions ou les hooks pour les limites de sécurité et tout ce qui ne doit jamais se produire, où vous avez besoin d'une garantie plutôt que de conseils.
</Note>

<h2 id="check-resolved-settings">
  Vérifier les paramètres résolus
</h2>

Les paramètres fusionnent entre les portées gérées, utilisateur, projet et locale. Les paramètres gérés gagnent toujours quand ils sont présents. Parmi le reste, la portée plus proche remplace la plus large dans l'ordre local, puis projet, puis utilisateur. Certains paramètres peuvent également être définis par des drapeaux de ligne de commande ou des [variables d'environnement](/docs/fr/env-vars), qui agissent comme une autre couche de remplacement. Quand un paramètre ne semble pas s'appliquer, la valeur que vous avez définie est généralement remplacée par une autre portée ou une variable d'environnement.

Exécutez `/doctor` pour vérifier votre configuration et installation. Il signale ce qu'il trouve, y compris les fichiers de paramètres invalides, les installations en double, les extensions inutilisées et le contenu `CLAUDE.md` archivé que Claude peut dériver de la base de code, puis propose des corrections qu'il applique uniquement après votre confirmation. La vérification de suppression `CLAUDE.md` nécessite Claude Code v2.1.206 ou ultérieure. Avant la v2.1.205, `/doctor` ouvrait un écran de diagnostics en lecture seule et appuyer sur `f` envoyait le rapport à Claude pour le corriger.

Depuis le terminal, `claude doctor` affiche les diagnostics d'installation et de paramètres en lecture seule sans démarrer une session.

Exécutez `/status` pour voir quelles sources de paramètres sont actives, y compris si les paramètres gérés sont en vigueur. Pour comprendre quelle portée gagne pour une clé donnée, consultez [Comment les portées interagissent](/docs/fr/settings#how-scopes-interact).

<h2 id="check-mcp-servers">
  Vérifier les serveurs MCP
</h2>

Exécutez `/mcp` pour voir chaque serveur configuré, son statut de connexion et si vous l'avez approuvé pour le projet actuel. Un serveur peut être défini correctement mais ne pas fournir d'outils pour quelques raisons courantes :

* Les serveurs à portée de projet dans `.mcp.json` nécessitent une approbation unique. Si l'invite a été rejetée, le serveur reste désactivé jusqu'à ce que vous l'approuviez depuis `/mcp`.
* Un serveur qui échoue au démarrage s'affiche comme échoué dans `/mcp`. Les chemins de fichiers relatifs dans `command` ou `args` sont une cause fréquente, car ils se résolvent par rapport au répertoire à partir duquel vous avez lancé Claude Code plutôt qu'à l'emplacement de `.mcp.json`.
* Un serveur qui s'affiche comme connecté mais qui liste zéro outils a démarré avec succès mais ne retourne pas de liste d'outils. Sélectionnez **Reconnect** depuis `/mcp`. Si le nombre reste à zéro, exécutez `claude --debug mcp` pour voir la sortie stderr du serveur.

Pour les emplacements de configuration et les règles de portée, consultez [MCP](/docs/fr/mcp).

<h2 id="check-hooks">
  Vérifier les hooks
</h2>

Exécutez `/hooks` pour lister chaque hook enregistré pour la session actuelle, groupé par événement. Si un hook que vous avez défini n'apparaît pas, il n'est pas en cours de lecture : les hooks vont sous la clé `"hooks"` dans un fichier de paramètres, pas dans un fichier autonome.

Si le hook apparaît mais ne se déclenche pas, le matcher est la cause habituelle. Vérifiez-le pour ces erreurs :

* Le champ `matcher` est une chaîne unique qui utilise `|` pour correspondre à plusieurs noms d'outils, par exemple `"Edit|Write"`. Un séparateur `,` est équivalent, donc `"Edit,Write"` correspond aux mêmes outils. Avant v2.1.191, une virgule passait à l'évaluation regex et le matcher ne correspondait jamais, donc utilisez `|` si vous n'êtes pas sur v2.1.191 encore.
* Un nom d'outil mal orthographié produit un matcher qui ne correspond à rien, donc le hook échoue silencieusement.
* Une valeur de tableau est une erreur de schéma : Claude Code affiche un avis d'erreur de paramètres et rejette l'intégralité du fichier de paramètres utilisateur, projet ou local, `claude doctor` signale l'échec de validation, et aucun hook de ce fichier n'apparaît dans `/hooks`. Dans les [paramètres gérés](/docs/fr/settings#settings-files), seule l'entrée invalide est supprimée et les autres hooks du fichier s'appliquent toujours.

Les modifications apportées à `settings.json` prennent effet dans la session en cours après un bref délai de stabilité du fichier. Vous n'avez pas besoin de redémarrer. Si `/hooks` affiche toujours l'ancienne définition quelques secondes après l'enregistrement, exécutez `/hooks` à nouveau pour actualiser la vue.

Si `/hooks` affiche le hook mais qu'il ne se déclenche toujours pas, l'étape suivante consiste à regarder l'évaluation du hook en direct. Démarrez une session avec `claude --debug hooks` et déclenchez l'appel d'outil. Le journal de débogage enregistre chaque événement, quels matchers ont été vérifiés, et le code de sortie et la sortie du hook. Consultez [Debug hooks](/docs/fr/hooks#debug-hooks) pour le format du journal et [hooks troubleshooting](/docs/fr/hooks-guide#limitations-and-troubleshooting) pour les modèles d'échec courants.

<h2 id="test-against-a-clean-configuration">
  Tester avec une configuration propre
</h2>

Commencez par [`claude --safe-mode`](/docs/fr/cli-reference#cli-flags), qui lance une session avec toutes les personnalisations désactivées, y compris `CLAUDE.md`, les skills, les plugins, les hooks, les serveurs MCP, et les commandes et agents personnalisés. L'authentification, la sélection du modèle, les outils intégrés et les permissions fonctionnent normalement. Si le problème disparaît en mode sécurisé, l'une de ces surfaces en est la cause ; utilisez les vérifications ciblées ci-dessus pour trouver laquelle. Le mode sécurisé applique toujours les hooks gérés et la politique de paramètres de votre organisation. Les plugins gérés, les skills, `CLAUDE.md` et les serveurs MCP sont désactivés.

Si le problème persiste en mode sécurisé, ou si vos paramètres eux-mêmes sont suspects, comparez avec une session qui ne charge rien de votre configuration habituelle. Pointez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars) vers un répertoire vide pour contourner tout ce qui se trouve sous `~/.claude`, et lancez depuis un répertoire qui n'a pas de dossier `.claude`, `.mcp.json` ou `CLAUDE.md` afin que la configuration du projet soit également ignorée.

```bash theme={null}
cd /tmp && CLAUDE_CONFIG_DIR=/tmp/claude-clean claude
```

La session propre n'a aucun paramètre utilisateur ou projet, hooks, serveurs MCP, plugins ou mémoire.

* Les paramètres gérés s'appliquent toujours si votre organisation les déploie, car ils se trouvent à un chemin système en dehors de `~/.claude`
* Sur Linux et Windows, vous serez invité à vous connecter à nouveau car les identifiants sont stockés sous le répertoire de configuration
* Sur macOS, les identifiants se trouvent dans le Keychain et sont reportés à la session propre

Si le problème disparaît ici, la cause se trouve quelque part dans vos fichiers réels `~/.claude` ou `.claude` du projet. Réintroduisez-les un à la fois, en copiant les fichiers dans le répertoire temporaire ou en lançant depuis votre projet, pour trouver lequel. S'il persiste dans la session propre, la cause se trouve en dehors de votre configuration utilisateur et projet. Exécutez `/status` pour vérifier si les paramètres gérés sont en vigueur, recherchez les [variables d'environnement](/docs/fr/env-vars) qui affectent Claude Code, puis consultez [Troubleshooting](/docs/fr/troubleshooting).

<h2 id="check-common-causes">
  Vérifier les causes courantes
</h2>

La plupart des surprises de configuration remontent à un petit ensemble de règles d'emplacement et de syntaxe. Vérifiez-les avant de supposer un bogue :

| Symptôme                                                                              | Cause                                                                                                                                                 | Correction                                                                                                                                                                                                                                                                                              |
| :------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Le hook ne se déclenche jamais                                                        | `matcher` est un tableau JSON au lieu d'une chaîne                                                                                                    | Utilisez une chaîne unique avec `\|` pour correspondre à plusieurs outils, par exemple `"Edit\|Write"`. Consultez [matcher patterns](/docs/fr/hooks#matcher-patterns).                                                                                                                                       |
| Le hook ne se déclenche jamais                                                        | `matcher` utilise `,` comme séparateur sur une version antérieure à v2.1.191                                                                          | Claude Code v2.1.191 ou version ultérieure traite `,` comme un séparateur de liste comme `\|`. Les versions antérieures évaluent une virgule comme un caractère littéral, donc `"Edit,Write"` ne correspond à rien. Utilisez `\|` à la place, ou mettez à jour Claude Code.                             |
| Le hook ne se déclenche jamais                                                        | La valeur `matcher` est en minuscules, par exemple `"bash"`                                                                                           | La correspondance est sensible à la casse. Les noms d'outils sont en majuscules : `Bash`, `Edit`, `Write`, `Read`.                                                                                                                                                                                      |
| Le hook ne se déclenche jamais                                                        | Les hooks sont définis dans un fichier autonome au lieu de `settings.json`                                                                            | Il n'y a pas de fichier hooks autonome pour la configuration du projet ou de l'utilisateur. Définissez les hooks sous la clé `"hooks"` dans `settings.json`. Seuls les [plugins](/docs/fr/plugins-reference#hooks) chargent un fichier `hooks/hooks.json` séparé. Consultez [hook configuration](/docs/fr/hooks). |
| Les permissions, hooks ou env définis globalement sont ignorés                        | La configuration a été ajoutée à `~/.claude.json`                                                                                                     | `~/.claude.json` contient l'état de l'application et les bascules d'interface utilisateur. `permissions`, `hooks` et `env` appartiennent à `~/.claude/settings.json`. Ce sont deux fichiers différents.                                                                                                 |
| Une valeur `settings.json` semble ignorée                                             | La même clé est définie dans `settings.local.json`                                                                                                    | `settings.local.json` remplace `settings.json`, et les deux remplacent `~/.claude/settings.json`. Consultez [settings precedence](/docs/fr/settings#how-scopes-interact).                                                                                                                                    |
| Le skill n'apparaît pas dans `/skills`                                                | Le fichier skill est à `.claude/skills/name.md` au lieu d'être dans un dossier                                                                        | Utilisez un dossier avec `SKILL.md` à l'intérieur : `.claude/skills/name/SKILL.md`.                                                                                                                                                                                                                     |
| Le skill apparaît dans `/skills` mais Claude ne l'invoque jamais                      | Le skill a `disable-model-invocation: true` dans son frontmatter, ou sa description ne correspond pas à la façon dont vous formulez la demande        | Vérifiez le badge dans `/skills` : un label « user-only » signifie que Claude ne le déclenchera pas de lui-même. Consultez [skill invocation](/docs/fr/skills).                                                                                                                                              |
| Les instructions du sous-répertoire `CLAUDE.md` semblent ignorées                     | Les fichiers du sous-répertoire se chargent à la demande, pas au démarrage de la session                                                              | Ils se chargent quand Claude lit un fichier dans ce répertoire avec l'outil Read, pas au lancement et pas lors de l'écriture ou de la création de fichiers là-bas. Consultez [how CLAUDE.md files load](/docs/fr/memory#how-claude-md-files-load).                                                           |
| Le sous-agent ignore les instructions `CLAUDE.md`                                     | Les agents Explore et Plan intégrés ignorent `CLAUDE.md`. Les sous-agents personnalisés le chargent de la même manière que la conversation principale | Pour Explore ou Plan, reformulez l'instruction dans votre invite de délégation. Pour un sous-agent personnalisé, mettez les instructions critiques dans le corps du fichier agent, qui devient l'invite système du sous-agent. Consultez [what loads at startup](/docs/fr/sub-agents#what-loads-at-startup). |
| La logique de nettoyage ne s'exécute jamais à la fin de la session                    | Aucun hook `SessionEnd` configuré                                                                                                                     | Ajoutez un hook `SessionEnd` dans `settings.json`. Consultez la [hook events list](/docs/fr/hooks#hook-events).                                                                                                                                                                                              |
| Les serveurs MCP dans `.mcp.json` ne se chargent jamais                               | Le fichier est sous `.claude/` ou utilise le format de configuration de Claude Desktop                                                                | La configuration MCP du projet va à la racine du référentiel sous `.mcp.json`, pas à l'intérieur de `.claude/`. Consultez [MCP configuration](/docs/fr/mcp).                                                                                                                                                 |
| Les serveurs MCP ajoutés sous `mcpServers` dans `settings.json` n'apparaissent jamais | `settings.json` ne lit pas une clé `mcpServers`                                                                                                       | Définissez les serveurs du projet dans `.mcp.json` à la racine du référentiel, ou exécutez `claude mcp add --scope user` pour les serveurs à portée utilisateur. Consultez [MCP configuration](/docs/fr/mcp).                                                                                                |
| Le serveur MCP du projet ajouté n'apparaît pas                                        | L'invite d'approbation unique a été rejetée                                                                                                           | Les serveurs à portée de projet nécessitent une approbation. Exécutez `/mcp` pour voir le statut et approuver.                                                                                                                                                                                          |
| Le serveur MCP échoue au démarrage depuis certains répertoires                        | `command` ou `args` utilise un chemin de fichier relatif                                                                                              | Utilisez des chemins absolus pour les scripts locaux. Les exécutables sur votre `PATH` comme `npx` ou `uvx` fonctionnent tels quels.                                                                                                                                                                    |
| Le serveur MCP démarre sans les variables d'environnement attendues                   | Les variables sont dans `settings.json` `env`, qui ne se propage pas aux processus enfants MCP                                                        | Définissez plutôt `env` par serveur à l'intérieur de `.mcp.json`.                                                                                                                                                                                                                                       |
| La règle de refus `Bash(rm *)` ne bloque pas `/bin/rm` ou `find -delete`              | Les règles de préfixe correspondent à la chaîne de commande littérale, pas à l'exécutable sous-jacent                                                 | Ajoutez des modèles explicites pour chaque variante, ou utilisez un [PreToolUse hook](/docs/fr/hooks-guide) ou le [sandbox](/docs/fr/sandboxing) pour une garantie difficile.                                                                                                                                     |

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour la référence complète sur chaque surface de configuration, consultez la page dédiée :

* **[Référence du répertoire `.claude`](/docs/fr/claude-directory)** : chaque emplacement de fichier de configuration et ce qui le lit
* **[Paramètres](/docs/fr/settings)** : ordre de précédence et liste complète des clés
* **[Référence des hooks](/docs/fr/hooks)** : noms d'événements, charges utiles et format de sortie `--debug hooks`
* **[MCP](/docs/fr/mcp)** : configuration du serveur, approbation et sortie `/mcp`
* **[Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install)** : `command not found`, PATH et problèmes d'authentification
* **[Dépannage](/docs/fr/troubleshooting)** : performance, blocages et problèmes de recherche
