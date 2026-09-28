> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer les autorisations

> Contrôlez ce que Claude Code peut accéder et faire avec des règles d'autorisation granulaires, des modes et des politiques gérées.

Claude Code prend en charge les autorisations granulaires afin que vous puissiez spécifier exactement ce que l'agent est autorisé à faire et ce qu'il ne peut pas faire. Vous pouvez archiver les paramètres d'autorisation dans le contrôle de version pour les partager avec tous les développeurs de votre organisation, et chaque développeur peut personnaliser les siens.

<h2 id="permission-system">
  Système d'autorisation
</h2>

Claude Code utilise un système d'autorisation à plusieurs niveaux pour équilibrer la puissance et la sécurité. Le tableau montre, pour chaque type d'outil, si le mode Manuel demande une approbation avant l'exécution de l'action. Les autres [modes d'autorisation](#permission-modes) changent lesquels vous demandent ; en mode auto, un classificateur examine les actions à votre place, et [comment le classificateur évalue les actions](/docs/fr/permission-modes#how-the-classifier-evaluates-actions) énumère celles qu'il voit.

| Type d'outil            | Exemple                      | Approbation requise                                                                                                   | Comportement « Oui, ne pas demander à nouveau » |
| :---------------------- | :--------------------------- | :-------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------- |
| Lecture seule           | Lectures de fichiers, Grep   | Non, dans le [répertoire de travail et répertoires supplémentaires](#working-directories)                             | S/O                                             |
| Commandes Bash          | Exécution shell              | Oui, sauf un ensemble intégré de [commandes en lecture seule](#read-only-commands)                                    | Permanent par répertoire de projet et commande  |
| Modification de fichier | Édition/écriture de fichiers | Oui                                                                                                                   | Jusqu'à la fin de la session                    |
| Récupération web        | WebFetch                     | Oui, sauf un ensemble intégré de [domaines de documentation préapprouvés](/docs/fr/tools-reference#webfetch-tool-behavior) | Permanent par répertoire de projet et domaine   |
| Recherche web           | WebSearch                    | Oui                                                                                                                   | Permanent par répertoire de projet              |

Lorsque vous choisissez « Oui, ne pas demander à nouveau » et que l'approbation s'enregistre de manière permanente, comme pour une commande Bash ou un domaine WebFetch, Claude Code enregistre la règle dans `.claude/settings.local.json` à la racine du référentiel git, résolu via [worktrees](/docs/fr/worktrees) vers le checkout principal. La règle s'applique aux sessions futures n'importe où dans ce référentiel, y compris les sessions démarrées dans des sous-répertoires et dans les worktrees. Une approbation de modification de fichier n'est pas enregistrée dans le fichier : comme le montre le tableau, elle dure jusqu'à la fin de la session. Dans certains cas, comme en dehors d'un référentiel git ou sur Windows, Claude Code n'utilise pas la racine du référentiel ; [Où Claude Code cherche chaque fichier](/docs/fr/settings#where-claude-code-looks-for-each-file) énumère ces cas et où il enregistre la règle à la place.

Avant la v2.1.211, Claude Code enregistrait toujours la règle dans le répertoire de démarrage, donc une approbation accordée dans un worktree ou un sous-répertoire ne s'appliquait pas au reste du référentiel. Les règles que les versions antérieures ont enregistrées dans un sous-répertoire ou un worktree s'appliquent toujours aux sessions démarrées là.

Parfois, une invite d'autorisation n'offre qu'une approbation unique, sans option « ne pas demander à nouveau » et sans option pour autoriser l'action pour le reste de la session. Claude Code n'offre ces options que lorsque l'invite peut vous montrer tout ce qu'elles autoriseraient, donc une règle que vous enregistrez à partir d'une invite couvre uniquement ce que son option nommée. Lorsqu'une invite n'offre que l'approbation unique, approuvez l'action une fois, ou ajoutez la règle vous-même dans [`/permissions`](#manage-permissions).

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  Ajouter un commentaire lorsque vous répondez à une invite d'autorisation
</h3>

Vous pouvez joindre une note à Claude lorsque vous approuvez ou refusez une action unique. Sur la plupart des invites d'autorisation, y compris les invites Bash, PowerShell, fichier et outil MCP, déplacez-vous vers **Oui** ou **Non** et appuyez sur `Tab` pour ouvrir un champ de commentaire sur cette option. Les invites WebFetch et navigateur n'offrent pas le champ. Les options qui autorisent l'action pour le reste de la session ou enregistrent une règle n'en prennent pas non plus.

Avec le champ ouvert, tapez le commentaire puis appuyez sur l'une de ces touches :

* `Entrée` : soumet votre réponse avec le commentaire joint. Si vous laissez le champ vide, Claude Code soumet la réponse sans commentaire.
* `Tab` : ferme le champ sans répondre. Claude Code conserve le texte que vous avez tapé et l'envoie toujours si vous répondez avec cette option.
* `Maj+Tab` : sur une invite de fichier, comme une invite Édition ou Écriture, ferme le champ de la même manière que `Tab`. Avant la v2.1.235, appuyer sur `Maj+Tab` à l'intérieur du champ sélectionnait plutôt l'option qui autorise l'action pour le reste de la session, donc Claude Code approuvait l'action pour le reste de la session et supprimait le commentaire.

Claude Code livre le commentaire différemment selon la façon dont vous avez répondu :

* **Oui** : Claude Code exécute l'action, puis envoie votre commentaire à Claude après le résultat.
* **Non** : Claude Code envoie votre commentaire à Claude comme raison du refus, et Claude continue de travailler. Si vous sélectionnez **Non** sans commentaire sur une invite de la conversation principale, Claude Code arrête le tour.

<h2 id="manage-permissions">
  Gérer les autorisations
</h2>

Vous pouvez afficher et gérer les autorisations d'outils de Claude Code avec `/permissions`. Cette interface utilisateur répertorie toutes les règles d'autorisation et le fichier `settings.json` dont elles proviennent. Vous pouvez ouvrir cette interface utilisateur pendant que Claude travaille : lorsque vous ajoutez ou supprimez une règle, Claude Code applique la modification à partir du prochain appel d'outil de Claude dans le même tour. Avant la v2.1.234, Claude Code mettait en file d'attente la commande jusqu'à la fin du tour.

* Les règles **Allow** permettent à Claude Code d'utiliser l'outil spécifié sans approbation manuelle.
* Les règles **Ask** demandent une confirmation chaque fois que Claude Code essaie d'utiliser l'outil spécifié.
* Les règles **Deny** empêchent Claude Code d'utiliser l'outil spécifié.

Les règles sont évaluées dans l'ordre : deny, puis ask, puis allow. La première correspondance dans cet ordre détermine le résultat, et la spécificité de la règle ne change pas l'ordre.

Une règle deny large comme `Bash(aws *)` bloque chaque appel correspondant, y compris les appels qui correspondent également à une règle allow plus étroite comme `Bash(aws s3 ls)`. Une règle allow ne peut pas créer une exception à partir d'une règle deny. La même priorité s'applique entre ask et allow : une règle ask correspondante demande une confirmation même lorsqu'une règle allow plus spécifique correspond également au même appel.

Les règles deny se comportent différemment selon qu'elles nomment un outil ou qu'elles délimitent un motif au sein de celui-ci. Un nom d'outil simple comme `Bash` supprime l'outil du contexte de Claude entièrement, donc Claude ne le voit jamais. Si vous ajoutez une telle règle en cours de session, Claude ne peut pas appeler l'outil à partir de son prochain appel d'outil ; [Denying an entire tool](/docs/fr/prompt-caching#denying-an-entire-tool) couvre ce qui se passe avec une définition que Claude a déjà vue. Une règle délimitée comme `Bash(rm *)` laisse l'outil disponible et bloque les appels correspondants lorsque Claude essaie de les utiliser.

La suppression par nom simple s'applique à tous les outils sauf [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) : une règle deny ne peut pas le supprimer tant que tout autre outil reste disponible, et une règle ask ne le demande jamais.

<Note>
  Les règles d'autorisation sont appliquées par Claude Code, et non par le modèle. Les instructions dans votre prompt ou `CLAUDE.md` façonnent ce que Claude essaie de faire, mais elles ne changent pas ce que Claude Code autorise. Pour accorder ou révoquer l'accès, utilisez `/permissions`, les règles décrites ici, un [mode d'autorisation](/docs/fr/permission-modes), ou un [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

Lorsque le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) est disponible pour votre session, cette interface utilisateur inclut également les [règles du classificateur du mode auto](/docs/fr/auto-mode-config#edit-rules-from-permissions). Sélectionnez l'onglet **Auto mode** pour les afficher.

<h2 id="permission-modes">
  Modes d'autorisation
</h2>

Claude Code prend en charge plusieurs modes d'autorisation qui contrôlent la façon dont il approuve les appels d'outils. Consultez [Modes d'autorisation](/docs/fr/permission-modes) pour savoir quand utiliser chacun. Pour modifier le mode dans lequel les sessions commencent, définissez `defaultMode` dans vos [fichiers de paramètres](/docs/fr/settings#where-settings-live). [Quel mode une session commence](/docs/fr/permission-modes#which-mode-a-session-starts-in) couvre la valeur par défaut intégrée pour chaque plan et ce que l'extension VS Code lit.

| Mode                | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Demande une autorisation à la première utilisation de chaque outil. Étiqueté Manuel dans l'interface de ligne de commande, les extensions VS Code et JetBrains, et l'application de bureau, et Claude Code accepte `manual` comme alias. L'étiquette et l'alias nécessitent Claude Code v2.1.200 ou version ultérieure. L'étiquette de l'application de bureau ne dépend pas de votre version CLI                                                                                                                                                                                                                                                                 |
| `acceptEdits`       | Accepte automatiquement les éditions de fichiers et les commandes courantes du système de fichiers telles que `mkdir`, `touch`, `mv` et `cp` pour les chemins du répertoire de travail ou `additionalDirectories`                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `plan`              | Claude lit les fichiers et exécute les commandes shell en lecture seule pour explorer mais n'édite pas vos fichiers source ; avec le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) disponible, les commandes approuvées par le classificateur s'exécutent également. Étiqueté Plan dans l'interface de ligne de commande et l'extension VS Code                                                                                                                                                                                                                                                                                              |
| `auto`              | Approuve automatiquement les appels d'outils avec des vérifications de sécurité en arrière-plan qui vérifient que les actions s'alignent avec votre demande                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `dontAsk`           | Refuse automatiquement chaque appel qui demanderait autrement une autorisation ; les lectures de fichiers dans vos répertoires de travail et autres actions qui ne nécessitent aucune approbation s'exécutent toujours, tout comme les outils pré-approuvés via `/permissions` ou les règles `permissions.allow`. `AskUserQuestion`, les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool), et les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) dans les sessions où ce paramètre atteint Claude Code sont refusés même si vous les avez autorisés |
| `bypassPermissions` | Ignore les invites d'autorisation, sauf pour les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

<Warning>
  En mode `bypassPermissions`, Claude Code ignore les invites d'autorisation, y compris pour les écritures dans les [chemins protégés](/docs/fr/permission-modes#protected-paths) tels que `.git` et `.claude`. Les [protections de messagerie inter-sessions](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode) s'appliquent toujours. Utilisez ce mode uniquement dans des environnements isolés comme les conteneurs ou les machines virtuelles où Claude Code ne peut pas causer de dommages.
</Warning>

Pour empêcher le mode `bypassPermissions` ou `auto` d'être utilisé, définissez `permissions.disableBypassPermissionsMode` ou `permissions.disableAutoMode` sur `"disable"` dans n'importe quel [fichier de paramètres](/docs/fr/settings#where-settings-live). Ces paramètres sont particulièrement utiles dans les [paramètres gérés](#managed-settings) où ils ne peuvent pas être remplacés.

<h2 id="permission-rule-syntax">
  Syntaxe des règles d'autorisation
</h2>

Les règles d'autorisation suivent le format `Tool` ou `Tool(specifier)`. Les parenthèses à l'intérieur du spécificateur sont littérales, donc une commande ou un chemin qui les contient n'a besoin d'aucun échappement.

<h3 id="match-all-uses-of-a-tool">
  Correspondre à tous les usages d'un outil
</h3>

Pour correspondre à tous les usages d'un outil, utilisez simplement le nom de l'outil sans parenthèses :

| Règle      | Effet                                                |
| :--------- | :--------------------------------------------------- |
| `Bash`     | Correspond à toutes les commandes Bash               |
| `WebFetch` | Correspond à toutes les demandes de récupération web |
| `Read`     | Correspond à toutes les lectures de fichiers         |

`Bash(*)` est équivalent à `Bash` et correspond à toutes les commandes Bash. En tant que règle de refus, les deux formes suppriment l'outil du contexte de Claude.

<h3 id="use-specifiers-for-fine-grained-control">
  Utiliser des spécificateurs pour un contrôle granulaire
</h3>

Ajoutez un spécificateur entre parenthèses pour correspondre à des usages d'outils spécifiques :

| Règle                          | Effet                                                                |
| :----------------------------- | :------------------------------------------------------------------- |
| `Bash(npm run build)`          | Correspond à la commande exacte `npm run build`                      |
| `Read(./.env)`                 | Correspond à la lecture du fichier `.env` dans le répertoire courant |
| `WebFetch(domain:example.com)` | Correspond aux demandes de récupération vers example.com             |

<h3 id="match-by-input-parameter">
  Correspondre par paramètre d'entrée
</h3>

Les règles de refus et de demande peuvent correspondre à un paramètre d'entrée de haut niveau sur n'importe quel outil intégré avec `Tool(param:value)`.

Pour correspondre à un paramètre sur un outil MCP, transmettez une règle de refus avec [`--disallowedTools`](/docs/fr/cli-reference#cli-flags). Lorsque Claude Code charge un fichier de paramètres, il ignore toute règle `mcp__` qui contient des parenthèses. Claude Code répertorie la règle ignorée dans la boîte de dialogue des paramètres invalides lorsqu'une session interactive démarre, et dans la sortie de [`claude doctor`](/docs/fr/debug-your-config#check-resolved-settings).

Une règle de paramètre correspond lorsque Claude appelle l'outil avec ce paramètre défini sur cette valeur exacte. Une règle d'autorisation pour une valeur de paramètre n'établirait pas que l'appel est sûr dans l'ensemble, donc les règles d'autorisation continuent à utiliser la syntaxe de spécificateur propre à chaque outil. Cela fonctionne pour n'importe quel paramètre scalaire que l'outil accepte :

| Règle                          | Correspond                                              |
| :----------------------------- | :------------------------------------------------------ |
| `Agent(model:opus)`            | Les appels Agent qui demandent le niveau de modèle Opus |
| `Agent(isolation:worktree)`    | Les appels Agent qui demandent une arborescence git     |
| `Bash(run_in_background:true)` | Les appels Bash qui s'exécutent en arrière-plan         |

La correspondance des paramètres suit ces règles :

* Le nom du paramètre doit être un champ direct de l'entrée de l'outil, tel que `model` sur l'outil Agent. Les champs imbriqués dans un objet ou un tableau ne sont pas correspondables
* Chaque règle nomme un paramètre. Pour contrôler à la fois `model` et `isolation`, écrivez deux règles, `Agent(model:opus)` et `Agent(isolation:worktree)`, plutôt que de les combiner dans une seule règle
* La valeur prend en charge `*` comme caractère générique qui correspond à n'importe quelle séquence de caractères, donc `Agent(isolation:*)` correspond à n'importe quelle valeur d'isolation explicite. Sans `*`, la correspondance est exacte
* Un paramètre que le modèle omet n'est jamais mis en correspondance, donc `Agent(model:*)` ne correspond pas à un appel qui laisse `model` non défini
* La valeur est comparée à l'entrée littérale que Claude envoie, avant toute normalisation. `Agent(model:opus)` correspond à l'alias `opus` mais pas à un ID de modèle complet. Exécutez avec [`--verbose`](/docs/fr/cli-reference) pour voir les noms et valeurs de paramètres exacts dans chaque appel d'outil
* L'espace blanc autour du deux-points est ignoré

Vous ne pouvez pas correspondre au champ de contenu principal d'un outil de cette façon : `command` pour Bash et PowerShell, `file_path` pour Read, Edit et Write, `path` pour Grep et Glob, `notebook_path` pour NotebookEdit, et `url` pour WebFetch. Une règle comme `Bash(command:rm *)` serait contournable par une commande composée, donc Claude Code l'ignore et émet un avertissement au démarrage. Utilisez plutôt `Bash(rm *)`, `Read(./path)`, ou `WebFetch(domain:host)`.

<h3 id="wildcard-patterns">
  Modèles de caractères génériques
</h3>

Un `*` dans une règle Bash correspond à n'importe quel texte, y compris les espaces, donc une règle couvre une famille de commandes. Une règle sans `*` correspond à une commande exacte.

<Warning>
  Mettez le `*` après la sous-commande. Dans `git log --oneline main`, `git` est le programme et `log` est la sous-commande, le mot qui détermine ce que le programme fait. Claude Code correspond à tout ce qui précède le premier `*` tel qu'écrit, donc ces mots sont ce qui limite la règle : `Bash(git log *)` permet uniquement les commandes `git log`, et `Bash(git *)` permet chaque commande git. Claude Code [avertit au démarrage](/docs/fr/errors#has-a-wildcard-before-the-rest-of-the-command) à propos d'une règle d'autorisation avec un `*` avant la sous-commande, comme `Bash(git * main)`.
</Warning>

Écrivez la commande que vous voulez que Claude exécute sans demander, et remplacez les parties qui varient par `*`. Avec cette configuration, Claude Code exécute les scripts npm et les commits git sans demander et refuse les commandes qui commencent par `git push`. Un push écrit d'une autre manière, comme `git -C . push`, n'est pas mis en correspondance ; voir [ce qu'une règle Bash ne correspond pas](#bash-rule-limits).

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Un `*` peut aller n'importe où dans la règle : au début, au milieu ou à la fin. Chaque ligne montre une règle, les commandes qu'elle correspond, et les commandes proches qu'elle ne correspond pas :

| Vous écrivez           | Correspond                                                                           | Ne correspond pas                      |
| :--------------------- | :----------------------------------------------------------------------------------- | :------------------------------------- |
| `Bash(npm run build)`  | `npm run build`                                                                      | `npm run build --watch`                |
| `Bash(npm run *)`      | `npm run build`, `npm run test --watch`, `npm run`                                   | `npm install`                          |
| `Bash(git log * main)` | `git log --oneline main`, `git log -5 main`, `git log --output=<file> main`          | `git log main`, `git push origin main` |
| `Bash(git * main)`     | `git merge main`, `git push origin main`, `git -c core.fsmonitor=<script> diff main` | `git log`                              |
| `Bash(* --version)`    | `node --version`, `bash -c 'echo hi' --version`                                      | `node -v`                              |
| `Bash(ls *)`           | `ls -la`, `ls`                                                                       | `lsof`                                 |
| `Bash(ls*)`            | `ls -la`, `lsof`                                                                     |                                        |
| `Bash(* --help *)`     | `npm --help x`                                                                       | `npm --help`                           |

Trois règles de correspondance produisent ces lignes :

* **Le `*` représente le texte qui se trouve à sa place.** Dans `Bash(git * main)`, il représente la sous-commande, donc Claude Code correspond à chaque sous-commande git et à chaque option avant elle. Cela inclut `-c`, qui fait exécuter à git un programme que vous nommez. Dans `Bash(* --version)`, le `*` représente le programme, donc n'importe quel programme correspond.
* **Un `*` à la fin, avec un espace avant lui, correspond également à la commande nue.** `Bash(ls *)` correspond à `ls`, et `Bash(git log *)` correspond à `git log`. Cela ne s'applique que lorsque le `*` de fin est le seul caractère générique de la règle : `Bash(* --help *)` correspond à `npm --help x` mais pas à `npm --help`.
* **L'espace avant un `*` de fin fait partie de la règle.** `Bash(ls *)` nécessite un espace après `ls`, donc `lsof` ne correspond pas. `Bash(ls*)` n'a pas d'espace, donc il correspond à `lsof` aussi.

Le suffixe `:*` est une façon équivalente d'écrire un caractère générique de fin, donc `Bash(ls:*)` correspond aux mêmes commandes que `Bash(ls *)`.

La boîte de dialogue d'autorisation écrit la forme séparée par des espaces lorsque vous sélectionnez « Oui, ne pas demander à nouveau » pour un préfixe de commande. La forme `:*` n'est reconnue qu'à la fin d'un modèle. Dans un modèle comme `Bash(git:* push)`, le deux-points est traité comme un caractère littéral et ne correspondra pas aux commandes git.

<h3 id="tool-name-wildcards">
  Caractères génériques de nom d'outil
</h3>

Les règles de refus et de demande acceptent également les modèles glob dans la position du nom d'outil. Le modèle doit correspondre au nom d'outil complet : `"*"` correspond à chaque outil, et `"mcp__*"` correspond à chaque outil MCP sur tous les serveurs. Un outil correspondant à une règle de refus de nom nu est supprimé du contexte de Claude, de la même manière qu'un nom d'outil nu, y compris l'exception [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) : un refus glob ne peut pas le supprimer tant que tout autre outil reste, et une demande glob ne le demande jamais. Cette configuration refuse chaque outil MCP :

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

Les règles d'autorisation acceptent les globs de nom d'outil uniquement après un préfixe littéral `mcp__<server>__`. Le segment serveur doit être exempt de glob afin que la règle nomme un serveur spécifique que vous avez configuré. `mcp__puppeteer__*` correspond à chaque outil du serveur `puppeteer`, et `mcp__github__get_*` correspond à ses outils `get_`. Un glob d'autorisation non ancré tel que `"*"`, `"B*"`, ou `"mcp__*"` est ignoré avec un avertissement et n'approuve automatiquement rien.

Une règle de refus ou de demande dont le nom d'outil ne correspond à aucun outil connu produit un avertissement au démarrage pour détecter les fautes de frappe. Les noms d'outils contenant `_` ou `*` sont exemptés de la vérification, et il en va de même pour les noms des outils que Claude Code a supprimés, comme `TaskOutput`.

L'étiquette affichée pour un outil dans la transcription et la boîte de dialogue d'autorisation peut différer de son nom canonique. Par exemple, l'outil étiqueté `Stop Task` dans la transcription a le nom canonique `TaskStop`. Les règles d'autorisation et les [correspondances de hook](/docs/fr/hooks) ne correspondent pas à l'étiquette, donc une règle écrite comme `Stop Task` ne correspond pas. Pour les règles de refus et de demande, l'avertissement au démarrage ci-dessus détecte l'inadéquation. Utilisez les noms canoniques listés dans la [référence des outils](/docs/fr/tools-reference).

<h2 id="tool-specific-permission-rules">
  Règles d'autorisation spécifiques aux outils
</h2>

<h3 id="bash">
  Bash
</h3>

Les règles Bash correspondent à l'ensemble du texte de la commande, avec `*` représentant n'importe quel texte. [Les modèles de caractères génériques](#wildcard-patterns) montrent quelles commandes chaque forme de règle correspond et où placer le `*`. Le reste de cette section couvre comment Claude Code correspond aux commandes composées et aux wrappers, ce qu'une règle ne correspond pas, les commandes en lecture seule et les redirections.

<h4 id="compound-commands">
  Commandes composées
</h4>

<Tip>
  Claude Code est conscient des opérateurs shell, donc une règle comme `Bash(safe-cmd *)` ne lui donnera pas la permission d'exécuter la commande `safe-cmd && other-cmd`. Les séparateurs de commande reconnus sont `&&`, `||`, `;`, `|`, `|&`, `&` et les sauts de ligne. Une règle doit correspondre à chaque sous-commande indépendamment.
</Tip>

Les règles de refus et de demande s'appliquent lorsqu'une sous-commande les correspond, y compris une commande imbriquée à l'intérieur d'une sous-coquille, une substitution de commande ou un corps de contrôle de flux tel qu'une boucle `for`. Une règle de demande comme `Bash(git clean *)` vous invite toujours pour `cd /tmp && git clean -f` ou `echo "$(git clean -f)"`, même en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode).

Lorsque `&&` ou `||` n'a rien après lui, comme dans `npm test &&`, Claude Code traite la commande comme non analysable et ne la divise pas en sous-commandes pour la correspondance des règles d'autorisation, donc une règle telle que `Bash(npm *)` ne l'approuve pas.

Lorsque vous approuvez une commande composée avec « Oui, ne pas demander à nouveau », Claude Code enregistre une règle séparée pour chaque sous-commande qui nécessite une approbation, plutôt qu'une seule règle pour la chaîne composée complète. Par exemple, approuver `git status && npm test` enregistre une règle pour `npm test`, donc les invocations futures de `npm test` sont reconnues indépendamment de ce qui précède le `&&`. Les sous-commandes comme `cd` dans un répertoire en dehors de vos répertoires de travail génèrent leur propre règle Read pour ce chemin. Jusqu'à 5 règles peuvent être enregistrées pour une seule commande composée.

<h4 id="process-wrappers">
  Wrappers
</h4>

Avant de correspondre aux règles Bash, Claude Code supprime un ensemble fixe de wrappers, donc une règle comme `Bash(npm test *)` correspond également à `timeout 30 npm test`. Les wrappers supprimés sont `timeout`, `time`, `nice`, `nohup` et `stdbuf`, plus les builtins shell `command` et `builtin`, et `noglob` de zsh. Chacun exécute son argument comme la commande réelle. Deux formes connexes ne sont pas supprimées : la forme de requête `command -v`, qui recherche une commande plutôt que de l'exécuter, et `nocorrect` de zsh.

Claude Code supprime également une affectation initiale de certaines variables d'environnement connues comme sûres, donc `Bash(npm test *)` correspond à `NODE_ENV=test npm test`. Une règle d'autorisation ne correspondra pas au-delà d'une affectation de toute autre variable. Une règle de refus ou de demande correspond au-delà de toute affectation initiale, donc `Bash(rm *)` en refus correspond toujours à `FOO=bar rm -rf tmp/`.

Le `xargs` nu est également supprimé, donc `Bash(grep *)` correspond à `xargs grep pattern`. La suppression s'applique uniquement lorsque `xargs` n'a pas de drapeaux : une invocation comme `xargs -n1 grep pattern` est mise en correspondance en tant que commande `xargs`, donc les règles écrites pour la commande interne ne la couvrent pas.

Cette liste de wrappers est intégrée et n'est pas configurable. Les exécuteurs d'environnement de développement tels que `direnv exec`, `devbox run`, `mise exec`, `npx` et `docker exec` ne figurent pas dans la liste. Parce que ces outils exécutent leurs arguments en tant que commande, une règle comme `Bash(devbox run *)` correspond à tout ce qui vient après `run`, y compris `devbox run rm -rf .`. Pour approuver le travail à l'intérieur d'un exécuteur d'environnement, écrivez une règle spécifique qui inclut à la fois l'exécuteur et la commande interne, comme `Bash(devbox run npm test)`. Ajoutez une règle par commande interne que vous souhaitez autoriser.

Les wrappers exec tels que `watch`, `setsid`, `ionice` et `flock` ne peuvent pas être approuvés automatiquement par une règle de préfixe comme `Bash(watch *)`, donc en mode Manuel ils demandent toujours. Il en va de même pour `find` avec `-exec` ou `-delete` : une règle `Bash(find *)` ne couvre pas ces formes. Pour approuver une invocation spécifique, écrivez une règle de correspondance exacte pour la chaîne de commande complète.

<h4 id="bash-rule-limits">
  Ce qu'une règle Bash ne correspond pas
</h4>

Une règle Bash correspond au texte de la commande que Claude écrit, après que Claude Code divise les [commandes composées](#compound-commands) et supprime les [wrappers](#process-wrappers). Elle ne correspond pas au même programme invoqué sous une forme différente, donc une règle de refus ou de demande couvre l'invocation que Claude produit généralement et n'est pas une limite de sécurité autour du programme. Ces règles en `deny` ou `ask` arrêtent la première forme et pas les autres :

| Règle              | Arrête                     | N'arrête pas                                                                                          |
| :----------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`, `sh -c 'curl https://example.com'`                               |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`, `bash -c 'rm -rf build/'`                                                       |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`, `git -c push.default=current push origin main`, `git 'push' origin main` |

Vos autres règles et le mode d'autorisation décident les commandes dans la dernière colonne.

Pour l'application du système de fichiers et du réseau qui ne dépend pas du texte de la commande, utilisez le [sandboxing](/docs/fr/sandboxing). Pour inspecter le texte de commande complet avec votre propre logique avant son exécution, utilisez un hook [PreToolUse](#extend-permissions-with-hooks).

<h4 id="read-only-commands">
  Commandes en lecture seule
</h4>

Claude Code reconnaît un ensemble intégré de commandes Bash comme étant en lecture seule et les exécute sans invite d'autorisation dans tous les modes, sauf pour un chemin que [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) protège. L'ensemble inclut `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` et les formes en lecture seule de `git`. L'ensemble n'est pas configurable ; pour exiger une invite pour l'une de ces commandes, ajoutez une règle `ask` ou `deny` pour celle-ci. En mode auto, ces commandes peuvent également attendre l'examen du classificateur ; voir [comment le classificateur évalue les actions](/docs/fr/permission-modes#how-the-classifier-evaluates-actions).

Une redirection telle que `ls > out.txt` ajoute une vérification sur la cible. Voir [Redirections](#redirections).

Les modèles glob non cités sont autorisés pour les commandes dont chaque drapeau est en lecture seule, donc `ls *.ts` et `wc -l src/*.py` s'exécutent sans invite.

En mode Manuel, les commandes de cet ensemble demandent toujours dans ces cas :

* **Globs non cités pour les commandes avec drapeaux capables d'écriture** : les commandes avec des drapeaux capables d'écriture ou d'exécution, tels que `find`, `sort`, `sed` et `git`, demandent lorsqu'un glob non cité est présent, car le glob pourrait s'étendre à un drapeau comme `-delete`.
* **`docker` pointant vers un autre daemon** : les formes en lecture seule de `docker` demandent lorsque la commande porte un drapeau qui sélectionne un daemon différent, tel que `-H`, `--context` ou `--url` et `--connection` de Podman.
* **`file` avec des drapeaux d'ouverture de chemin** : `file` demande lorsqu'il passe `-m`/`--magic-file` ou `-f`/`--files-from`, car ces drapeaux font que `file` ouvre les chemins nommés dans la valeur du drapeau.
* **Chemins réseau sur Windows** : une commande dont les arguments incluent un chemin réseau (UNC), tel que `\\server\share\file`, demande car l'accès à un chemin réseau peut envoyer vos identifiants Windows à l'hôte qu'il nomme. La même vérification s'applique aux commandes de l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool).
* **Commandes que l'analyse ne peut pas analyser** : lorsque Claude Code ne peut pas analyser complètement une commande, il demande une approbation au lieu de traiter la commande comme en lecture seule. Les commandes plus longues que 10 000 caractères demandent toujours car elles dépassent ce que l'analyse analyse.

Un `cd` dans un chemin à l'intérieur de votre répertoire de travail ou d'un [répertoire supplémentaire](#working-directories) est également en lecture seule, et une commande composée comme `cd packages/api && ls` s'exécute sans invite lorsque chaque partie se qualifie seule. Ces combinaisons demandent même lorsque chaque partie est en lecture seule :

* **`cd` avec `git`** : demande lorsque le `cd` change dans un répertoire différent, car exécuter `git` dans un nouveau répertoire peut exécuter les hooks de ce répertoire. Un `cd` dont la cible se résout au répertoire de travail courant est une non-opération et ne déclenche pas l'invite.
* **`cd` avec une redirection** : demande lorsque Claude Code ne peut pas déterminer le répertoire dans lequel la cible de redirection se résout après l'exécution de `cd`. Une commande dont la seule cible de redirection est `/dev/null`, telle que `cd app; grep -r pattern . 2>/dev/null`, ne demande pas, car `/dev/null` ne dépend pas du répertoire de travail.

<Warning>
  Les modèles d'autorisation Bash qui tentent de contraindre les arguments de commande sont fragiles. Par exemple, `Bash(curl http://github.com/ *)` a l'intention de restreindre curl aux URL GitHub, mais ne correspondra pas aux variations comme :

  * Options avant l'URL : `curl -X GET http://github.com/...`
  * Protocole différent : `curl https://github.com/...`
  * Redirections : `curl -L http://short.example.com/xyz`, qui redirige vers GitHub
  * Variables : `URL=http://github.com && curl $URL`

  Pour un filtrage d'URL plus fiable, envisagez :

  * **Restreindre les outils réseau Bash** : utilisez les règles de refus pour bloquer `curl`, `wget` et les commandes similaires, puis utilisez l'outil WebFetch avec l'autorisation `WebFetch(domain:github.com)` pour les domaines autorisés. Une règle de refus ne correspond pas au même programme par chemin ou à l'intérieur de `sh -c`, donc associez-la à la [liste d'autorisation du réseau sandbox](/docs/fr/sandboxing#network-isolation) lorsque la restriction doit tenir ; voir [ce qu'une règle Bash ne correspond pas](#bash-rule-limits)
  * **Utiliser les hooks PreToolUse** : implémentez un hook qui valide les URL dans les commandes Bash et bloque les domaines non autorisés
  * **Ajouter des conseils CLAUDE.md** : décrivez vos modèles curl autorisés dans `CLAUDE.md`. Cela façonne ce que Claude essaie mais n'applique pas une limite, donc associez-le à l'une des options ci-dessus

  Notez que l'utilisation de WebFetch seul n'empêche pas l'accès au réseau. Si Bash est autorisé, Claude peut toujours utiliser `curl`, `wget` ou d'autres outils pour atteindre n'importe quelle URL.
</Warning>

<h4 id="redirections">
  Redirections
</h4>

Lorsqu'une commande redirige la sortie ou l'entrée, Claude Code vérifie la cible de redirection par rapport à vos règles de fichier comme si Claude avait écrit ou lu ce fichier directement :

* **Redirections de sortie** : pour `> file`, `>> file` ou `2> file`, la vérification couvre vos règles d'autorisation et de refus `Edit`, les [chemins protégés](/docs/fr/permission-modes#protected-paths) et les [répertoires de travail](#working-directories). Une règle telle que `Bash(git commit *)` autorise la commande, pas la cible. Une cible qui commence par `~` ou contient un caractère glob nécessite votre approbation.
* **Redirections d'entrée** : pour `< file`, la vérification couvre vos règles d'autorisation et de refus `Read` et les répertoires de travail. Une cible en dehors des répertoires de travail nécessite votre approbation sauf si une règle d'autorisation la couvre. Une cible qui contient un modèle glob ou un chemin relatif qui suit un `cd` dans la même commande nécessite votre approbation même lorsqu'une règle d'autorisation la couvre. Claude Code vérifie les cibles d'entrée en v2.1.257 et ultérieur.

Les cibles sans fichier derrière elles ne sont pas vérifiées : `/dev/null`, les formes de descripteur de fichier telles que `2>&1` et `<&3`, et les here-docs et here-strings.

Claude Code vérifie également les fichiers qu'une commande `tee` écrit, y compris dans un pipeline tel que `make | tee build.log`. La vérification couvre vos règles d'autorisation et de refus `Edit`, les [chemins protégés](/docs/fr/permission-modes#protected-paths) et les [répertoires de travail](#working-directories). Une règle d'autorisation telle que `Bash(tee *)` ne couvre pas une destination en dehors des répertoires de travail. Claude Code vérifie les cibles `tee` en v2.1.269 et ultérieur.

<h3 id="powershell">
  PowerShell
</h3>

Les règles d'autorisation PowerShell utilisent la même forme que les règles Bash. Les caractères génériques avec `*` correspondent à n'importe quelle position, le suffixe `:*` est équivalent à un ` *` de fin, et un `PowerShell` nu ou `PowerShell(*)` correspond à chaque commande. Cette configuration permet les commandes `Get-ChildItem` et `git commit` tout en bloquant `Remove-Item` :

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

Les alias courants sont canonicalisés avant la correspondance. Une règle écrite pour le nom de la cmdlet correspond également à ses alias, donc `PowerShell(Get-ChildItem *)` correspond à `gci`, `ls` et `dir` aussi. La correspondance est insensible à la casse.

Claude Code analyse l'AST PowerShell et vérifie chaque commande dans une commande composée indépendamment. Les opérateurs de pipeline `|`, les séparateurs d'instruction `;` et sur PowerShell 7+ les opérateurs de chaîne `&&` et `||` divisent une commande composée en sous-commandes. Une règle doit correspondre à chaque sous-commande pour que la commande composée soit autorisée.

<h3 id="read-and-edit">
  Read et Edit
</h3>

Pour bloquer les outils de fichier de Claude de lire un fichier ou un répertoire, ajoutez une règle de refus `Read` pour son chemin, telle que `Read(./.env)` ou `Read(./secrets/**)` ; [Exclure les fichiers sensibles](/docs/fr/settings-reference#exclude-sensitive-files) a un exemple prêt à coller.

Les règles `Edit` s'appliquent à tous les outils intégrés qui éditent les fichiers. Claude fait un effort raisonnable pour appliquer les règles `Read` à tous les outils intégrés qui lisent les fichiers comme Grep et Glob, aux mentions `@file` dans vos invites et à la sélection et au contexte de fichier ouvert qu'un [IDE](/docs/fr/vs-code#the-built-in-ide-mcp-server) connecté partage avec Claude.

Une règle de refus `Read` bloque également les [outils Edit et Write](/docs/fr/errors#file-is-covered-by-a-read-deny-rule) sur le même chemin, y compris la création d'un nouveau fichier là-bas. NotebookEdit n'est pas couvert, donc ajoutez une règle de refus `Edit` pour les chemins qu'aucun outil ne peut modifier. La vérification nécessite Claude Code v2.1.208 ou ultérieur sur les éditions, et v2.1.228 ou ultérieur sur les écritures.

Claude Code vérifie les autorisations de fichier uniquement par rapport aux règles `Edit(path)` et `Read(path)`. Si vous écrivez une règle de chemin pour `Write`, `NotebookEdit`, `Glob` ou l'outil hérité `MultiEdit` à la place, Claude Code accepte la règle mais ne la consulte jamais, et [avertit au démarrage](/docs/fr/errors#is-not-matched-by-file-permission-checks), sauf pour une règle `Glob` passée dans `--allowedTools`. Utilisez `Edit(docs/**)` à la place de `Write(docs/**)`, `NotebookEdit(docs/**)` ou `MultiEdit(docs/**)`, et `Read(docs/**)` à la place de `Glob(docs/**)`. Claude Code n'avertit pas à propos d'une règle de nom d'outil sans chemin, telle qu'une règle de refus pour `Write` ; elle correspond à cette règle au niveau de l'outil partout. Nécessite Claude Code v2.1.210 ou ultérieur.

<Warning>
  Les règles de refus Read et Edit s'appliquent aux outils de fichier intégrés de Claude, aux commandes de fichier que Claude Code reconnaît dans Bash, telles que `cat`, `head`, `tail`, `sed` et `tee`, et aux cibles des [redirections](#redirections) Bash telles que `> file` et `< file`. Elles ne s'appliquent pas à une commande qui lit les fichiers sans les nommer, telle que `grep -r pattern .` exécutée à partir du répertoire qui contient le fichier, ou aux sous-processus arbitraires qui lisent ou écrivent des fichiers indirectement, comme un script Python ou Node qui ouvre des fichiers lui-même. Pour une application au niveau du système d'exploitation qui bloque tous les processus d'accéder à un chemin, [activez le sandbox](/docs/fr/sandboxing).
</Warning>

Les règles Read et Edit utilisent toutes deux la syntaxe de modèle [gitignore](https://git-scm.com/docs/gitignore) avec quatre types de modèles distincts ; pour les modèles de répertoire à segment unique, la profondeur de correspondance dépend également du type de règle, décrit plus loin dans cette section :

| Modèle             | Signification                                              | Exemple                          | Correspond                                                                    |
| ------------------ | ---------------------------------------------------------- | -------------------------------- | ----------------------------------------------------------------------------- |
| `//path`           | Chemin absolu à partir de la racine du système de fichiers | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                                     |
| `~/path`           | Chemin à partir du répertoire home                         | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                                                |
| `/path`            | Chemin relatif à la source des paramètres                  | `Edit(/src/**/*.ts)`             | `<répertoire de travail principal>/src/**/*.ts` dans les paramètres du projet |
| `path` ou `./path` | Chemin relatif au répertoire courant                       | `Read(*.env)`                    | `<cwd>/*.env`                                                                 |

<Warning>
  Un modèle comme `/Users/alice/file` n'est pas un chemin absolu. La barre oblique unique en début ancre à la source des paramètres, pas à la racine du système de fichiers. Utilisez `//Users/alice/file` pour les chemins absolus.
</Warning>

Un modèle `/path` s'ancre à un répertoire associé à la source des paramètres qui le définit, donc la même règle correspond à des emplacements différents selon l'endroit où vous la placez :

| Règle définie dans                                 | `/path` se résout à                      |
| :------------------------------------------------- | :--------------------------------------- |
| Paramètres du projet à `.claude/settings.json`     | `<répertoire de travail principal>/path` |
| Paramètres locaux à `.claude/settings.local.json`  | `<répertoire de travail principal>/path` |
| Paramètres utilisateur à `~/.claude/settings.json` | `~/.claude/path`                         |
| Un fichier passé avec `--settings <file>`          | `<répertoire du fichier>/path`           |
| Drapeaux CLI ou règles de session                  | `<répertoire de travail principal>/path` |

Une règle que vous ajoutez via `/permissions` suit la ligne pour le fichier de paramètres dans lequel vous l'enregistrez.

Les règles de paramètres locaux s'ancrent au [répertoire de travail principal](#working-directories) de la session, pas à la racine du référentiel où Claude Code [stocke le fichier](#permission-system) en v2.1.211 et ultérieur. Dans une session démarrée à la racine du référentiel, les deux répertoires sont les mêmes ; dans une session [worktree](/docs/fr/worktrees), une règle partagée telle que `Edit(/src/**)` correspond au répertoire `src/` propre de ce worktree.

Une règle de refus telle que `Read(/secrets/**)` dans les paramètres utilisateur bloque `~/.claude/secrets/**`, pas un répertoire `secrets` dans votre projet. Pour écrire une règle dans les paramètres utilisateur qui s'applique à l'intérieur de chaque projet, utilisez plutôt un chemin absolu `//` ou un chemin relatif à home `~/`.

Sur Windows, les chemins sont normalisés en forme POSIX avant la correspondance. `C:\Users\alice` devient `/c/Users/alice`, donc utilisez `//c/**/.env` pour correspondre aux fichiers `.env` n'importe où sur ce lecteur. Pour correspondre sur tous les lecteurs, utilisez `//**/.env`.

Exemples :

* `Edit(/docs/**)` : édite dans `<répertoire de travail principal>/docs/`, pas `/docs/` ou `<répertoire de travail principal>/.claude/docs/`
* `Read(~/.zshrc)` : lit le `.zshrc` de votre répertoire home
* `Edit(//tmp/scratch.txt)` : édite le chemin absolu `/tmp/scratch.txt`
* `Read(src/**)` : en tant que règle d'autorisation, lit à partir de `<répertoire courant>/src/` uniquement ; en tant que règle de refus ou de demande, correspond à un répertoire `src` à n'importe quelle profondeur sous le répertoire courant

Une règle ne correspond qu'aux fichiers sous son ancrage ; dans cette limite, la profondeur de correspondance dépend de la forme du modèle et, pour les modèles de répertoire à segment unique, du type de règle, décrit ci-dessous. Les noms de fichiers nus suivent la sémantique gitignore et correspondent à n'importe quelle profondeur, donc `Read(.env)` et `Read(**/.env)` sont équivalents :

| Règle de refus                  | Bloque                                              | Ne bloque pas                                                 |
| ------------------------------- | --------------------------------------------------- | ------------------------------------------------------------- |
| `Read(.env)` ou `Read(**/.env)` | tout `.env` au ou sous le répertoire courant        | `.env` dans un répertoire parent ou un autre projet           |
| `Read(//**/.env)`               | tout `.env` n'importe où sur le système de fichiers | rien ; la règle est ancrée à la racine du système de fichiers |

Un modèle relatif avec un segment de répertoire unique, tel que `src/**`, correspond à des profondeurs différentes selon le type de règle :

* **Règles d'autorisation** : `Edit(src/**)` correspond uniquement à `<cwd>/src` et aux fichiers sous celui-ci. Pour autoriser un nom de répertoire à n'importe quelle profondeur, écrivez `Edit(**/src/**)`.
* **Règles de refus et de demande** : `Read(secrets/**)` correspond à un répertoire nommé `secrets` à n'importe quelle profondeur sous le répertoire courant, donc la règle s'applique également aux copies imbriquées.

Chaque autre forme de modèle correspond à la même profondeur dans chaque type de règle : `Edit(/src/**)` et `Edit(src/components/**)` correspondent uniquement à leur emplacement ancré, tandis que `Edit(**/src/**)` correspond à n'importe quelle profondeur.

L'exemple suivant montre chaque forme de modèle par rapport à un projet avec un répertoire `src/` de niveau supérieur et une copie imbriquée sous `vendor/` :

```text theme={null}
<répertoire courant>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| Règle                                                   | Correspond à `src/app.ts` | Correspond à `vendor/pkg/src/lib.js` |
| :------------------------------------------------------ | :------------------------ | :----------------------------------- |
| `Edit(src/**)` en tant que règle d'autorisation         | Oui                       | Non                                  |
| `Edit(src/**)` en tant que règle de refus ou de demande | Oui                       | Oui                                  |
| `Edit(/src/**)` dans n'importe quel type de règle       | Oui                       | Non                                  |
| `Edit(**/src/**)` dans n'importe quel type de règle     | Oui                       | Oui                                  |

<Note>
  Dans les modèles gitignore, `*` correspond dans un seul segment de chemin et peut apparaître à n'importe quelle position du modèle, tandis que `**` correspond sur les répertoires.
</Note>

Lorsque vous approuvez un chemin de fichier avec « Oui, ne pas demander à nouveau », Claude Code échappe les caractères de modèle gitignore dans ce chemin, tels que `[`, `]` et `*`, afin que la règle générée ne corresponde qu'au chemin littéral que vous avez approuvé. Les règles que vous écrivez vous-même ne sont pas échappées. Avant la v2.1.202, Claude Code enregistrait le chemin non échappé, donc une règle générée pour un répertoire nommé `[2024-06] Reports` pouvait échouer à correspondre à son propre chemin ou correspondre à des répertoires frères non intentionnels.

Vous n'avez pas besoin d'échapper les parenthèses dans un chemin, donc `Edit(./Finance (2024)/**)` correspond au dossier `Finance (2024)` tel qu'épelé.

Une règle de refus ou de demande dont le chemin n'est pas utilisable comme modèle gitignore protège toujours ce chemin exact. Une règle d'autorisation avec un modèle inutilisable n'approuve rien.

Une règle de refus ou de demande dont le chemin commence par `!` est une négation gitignore. Elle découpe les chemins qu'elle correspond hors des règles `path` ou `./path` énumérées avant elle. Dans la liste `deny` d'un fichier de paramètres, `Read(*.env)` suivi de `Read(!sample.env)` bloque chaque fichier dont le nom se termine par `.env` à n'importe quelle profondeur, sauf les fichiers nommés `sample.env`. Une règle `!` énumérée en premier ne découpe rien.

La découpe ne s'étend qu'aux règles de la même source. Un `Read(!.env)` dans les paramètres du projet ou dans `--disallowedTools` n'annule pas un refus `Read(./.env)` des paramètres gérés ou de tout autre fichier de paramètres.

Deux limites réduisent ce qu'un modèle `!` peut découper :

* Claude Code lit un modèle `!` relatif au répertoire courant même lorsque `/`, `~/` ou `//` suit le `!`, donc le modèle ne peut pas atteindre une règle ancrée avec l'un de ces préfixes. `Read(!~/notes/public/**)` ne découpe rien hors de `Read(~/notes/**)`.
* Une découpe ne peut pas rouvrir un fichier à l'intérieur d'un répertoire qu'une règle bloque dans son ensemble. Avec `Read(secrets/**)` et `Read(!secrets/public/**)`, Claude Code bloque toujours `secrets/public` avec le reste de `secrets`.

Lorsque Claude accède à un lien symbolique, les règles d'autorisation vérifient deux chemins : le lien symbolique lui-même et le fichier vers lequel il se résout. Les règles d'autorisation et de refus traitent cette paire différemment : les règles d'autorisation reviennent à vous inviter, tandis que les règles de refus bloquent carrément.

* **Règles d'autorisation** : s'appliquent uniquement lorsque le chemin du lien symbolique et sa cible correspondent tous les deux. Un lien symbolique à l'intérieur d'un répertoire autorisé qui pointe vers l'extérieur vous invite toujours.
* **Règles de refus** : s'appliquent lorsque le chemin du lien symbolique ou sa cible correspond. Un lien symbolique qui pointe vers un fichier refusé est lui-même refusé. Par exemple, avec `Read(./project/**)` autorisé et `Read(~/.ssh/**)` refusé, un lien symbolique à `./project/key` pointant vers `~/.ssh/id_rsa` est bloqué : la cible échoue à la règle d'autorisation et correspond à la règle de refus.

Sur macOS et Linux, une règle de refus ou de demande écrite via un répertoire lié symboliquement avec un modèle `//`, `~/` ou `/` s'applique également à l'emplacement réel du répertoire. Par exemple, sur macOS, où `/etc` se résout à `/private/etc`, `Read(//etc/**)` bloque également `/private/etc/hosts`. Avant la v2.1.268, une règle de refus ou de demande écrite via un répertoire lié symboliquement ne s'appliquait pas à un chemin donné par son emplacement réel.

Lorsqu'un outil ouvre un fichier approuvé, Claude Code [confirme que le chemin se résout toujours à l'emplacement que la vérification d'autorisation a approuvé](/docs/fr/errors#refusing-after-a-symlink-changed).

Grep et Glob recherchent le répertoire auquel l'argument `path` se résout. Claude Code applique les règles de refus `Read` à ce répertoire.

<h3 id="webfetch">
  WebFetch
</h3>

Les règles WebFetch utilisent un préfixe `domain:` et correspondent au nom d'hôte de l'URL demandée. La correspondance est insensible à la casse, prend en charge les caractères génériques `*` et supprime un point final des deux côtés de la règle et du nom d'hôte afin que `example.com.` et `example.com` soient traités de la même manière.

* `WebFetch(domain:example.com)` correspond aux demandes vers `example.com`
* `WebFetch(domain:*.example.com)` correspond à tout sous-domaine à n'importe quelle profondeur, comme `api.example.com` ou `a.b.example.com`, mais pas à `example.com` lui-même
* `WebFetch(domain:*)` correspond à chaque domaine. Ce n'est pas la même chose qu'une règle `WebFetch` nu ; voir [Autoriser ou refuser chaque fetch](#allow-or-deny-every-fetch)

À n'importe quelle position autre qu'un `*.` initial ou un `*` nu, le caractère générique correspond uniquement au texte entre deux points. `WebFetch(domain:example.*)` correspond à `example.org`, où `*` devient `org`, mais pas à `example.evil.com`, où `*` devrait devenir `evil.com` et traverser un point. Cela empêche un caractère générique de fin de correspondre à des domaines qu'un attaquant pourrait enregistrer.

Les caractères génériques dans les règles `WebFetch` nécessitent Claude Code v2.1.172 ou ultérieur pour correspondre aux fetches.

<h4 id="allow-or-deny-every-fetch">
  Autoriser ou refuser chaque fetch
</h4>

Une règle `WebFetch` nu est le nom de l'outil sans partie `domain:`, telle que `"deny": ["WebFetch"]`. À la fois elle et `WebFetch(domain:*)` couvrent chaque URL, mais Claude Code les applique différemment, et seule la forme `domain:` ajoute également son domaine à la [liste de domaines autorisés ou refusés](/docs/fr/sandboxing#network-isolation) du sandbox. Cette section énumère les formes de caractères génériques que le sandbox honore et la version qui a ajouté le `*` nu.

Chaque ligne montre ce qu'une règle fait dans la liste `allow` et dans la liste `deny` :

| Règle                | Dans `allow`                                                                                          | Dans `deny`                                                                                                                                           |
| :------------------- | :---------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `WebFetch`           | Claude fetch sans vous inviter. Ne change pas quels hôtes les commandes en sandbox peuvent atteindre. | Claude Code supprime l'outil `WebFetch`, donc Claude ne peut pas fetch du tout. Ne change pas quels hôtes les commandes en sandbox peuvent atteindre. |
| `WebFetch(domain:*)` | Claude fetch sans vous inviter, et les commandes en sandbox peuvent atteindre n'importe quel hôte.    | Claude Code garde l'outil et refuse chaque fetch, et les commandes en sandbox ne peuvent atteindre aucun hôte.                                        |

Les deux formes diffèrent également sur les lectures des [artifacts](/docs/fr/artifacts), les pages que l'outil Artifact publie sur claude.ai. Une règle de refus ou de demande `WebFetch` nu ne s'applique pas à ces lectures. Une règle `domain:` couvrant `claude.ai` ou l'hôte de contenu `*.claudeusercontent.com`, telle que `WebFetch(domain:claude.ai)` ou `WebFetch(domain:*)`, refuse chaque lecture ou demande avant celle-ci. Une règle [`Artifact`](/docs/fr/artifacts#disable-artifacts) fait la même chose.

Lorsqu'une règle bloque une lecture, le refus nomme la règle. Avant la v2.1.268, une règle de refus `WebFetch` nu bloquait chaque lecture d'artifact, et une règle de demande nu demandait avant chacune.

Pour laisser Claude fetch librement tout en gardant la liste d'autorisation du sandbox telle qu'elle est, utilisez la forme nu. Ce `settings.json` fait cela :

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Lorsque vous demandez à Claude de fetch une page, il fetch sans invite. Lorsque vous lui demandez d'exécuter un `curl` [en sandbox](/docs/fr/sandboxing) contre un hôte en dehors de la liste d'autorisation du sandbox, Claude Code vous invite toujours pour cet hôte, car la règle nu n'a pas ajouté l'hôte à la liste d'autorisation.

En [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), Claude nomme plutôt l'hôte dans les [domaines autorisés par commande](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode) de la commande pour que le classificateur examine.

<h3 id="mcp">
  MCP
</h3>

Les règles MCP utilisent le nom du serveur tel que configuré dans Claude Code, optionnellement suivi du nom d'un outil de ce serveur.

* `mcp__puppeteer` correspond à tout outil fourni par le serveur `puppeteer`
* `mcp__puppeteer__*` utilise la syntaxe de caractère générique et correspond également à tous les outils du serveur `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` correspond à l'outil `puppeteer_navigate` fourni par le serveur `puppeteer`

Si votre organisation a défini un outil de connecteur [claude.ai](/docs/fr/mcp#organization-controls-on-connector-tools) sur `ask` et que ce paramètre atteint Claude Code dans votre session, les règles d'autorisation pour cet outil ne prennent pas effet : Claude Code demande à chaque appel, même en modes `auto` et `bypassPermissions`. En mode `dontAsk`, qui ne demande jamais, Claude Code refuse l'appel à la place. Les outils de connecteur que Claude Code récupère lui-même apparaissent comme `mcp__claude_ai_<server>__<tool>`.

Dans une session [Cowork](https://claude.com/docs/cowork/overview) dans l'application Claude Desktop, Claude exécute les commandes shell via l'outil `mcp__workspace__bash` de Cowork plutôt que l'outil `Bash` intégré, et Cowork fournit également `mcp__workspace__web_fetch` pour les web fetches. Claude Code applique également les règles de refus qui nomment l'outil entier `Bash` ou `WebFetch` à ces outils Cowork, donc une règle de refus `Bash` gérée empêche Claude d'exécuter des commandes shell dans Cowork. Lorsque Claude Code bloque un tel appel, le message nomme l'outil Cowork : `Permission to use mcp__workspace__bash has been denied.` Les règles d'autorisation ne se reportent pas : Claude Code n'applique jamais une règle d'autorisation `Bash` à `mcp__workspace__bash`.

<h3 id="agent-subagents">
  Agent (subagents)
</h3>

Utilisez les règles `Agent(AgentName)` pour contrôler quels [subagents](/docs/fr/sub-agents) Claude peut utiliser :

* `Agent(Explore)` correspond au subagent Explore
* `Agent(Plan)` correspond au subagent Plan
* `Agent(my-custom-agent)` correspond à un subagent personnalisé nommé `my-custom-agent`

Ajoutez ces règles au tableau `deny` dans vos paramètres ou utilisez l'indicateur CLI `--disallowedTools` pour désactiver des agents spécifiques. Pour désactiver l'agent Explore :

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

Les règles `Cd` contrôlent les répertoires vers lesquels la [commande `/cd`](/docs/fr/commands) peut déplacer la session. `Cd` n'est pas un outil invocable par le modèle : Claude ne peut pas l'appeler, et les règles s'appliquent uniquement lorsque vous exécutez `/cd` vous-même.

Une règle de refus `Cd` nu désactive `/cd` entièrement. Une règle de refus `Cd(<path-pattern>)` bloque les cibles correspondantes. Les règles de refus vérifient chaque orthographe de la cible, y compris chaque saut de lien symbolique qu'elle résout, donc une règle écrite pour un chemin bloque également les cibles qui s'y résolvent.

L'ajout de toute règle d'autorisation `Cd` bascule `/cd` en mode liste blanche : le répertoire cible résolu doit correspondre à l'une de vos règles d'autorisation, ou `/cd` refuse. Sans règles `Cd` configurées, `/cd` conserve son comportement par défaut et vous invite à faire confiance à un répertoire inconnu.

Les modèles de chemin partagent les ancrages `//`, `~/` et `/` des [règles Read et Edit](#read-and-edit), mais la correspondance est ancrée au chemin du répertoire entier plutôt qu'au style gitignore. `*` correspond à exactement un segment de chemin et `**` correspond sur les segments. Un `/**` de fin correspond également à sa racine nommée.

| Règle                 | Correspond                                                                             | Ne correspond pas                 |
| --------------------- | -------------------------------------------------------------------------------------- | --------------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                                                           | `~/code/app/src`, `~/code`        |
| `Cd(~/code/**)`       | `~/code` et tout répertoire sous celui-ci                                              | répertoires en dehors de `~/code` |
| `Cd(**/node_modules)` | tout répertoire `node_modules` à n'importe quelle profondeur sous le répertoire actuel | `node_modules/pkg`                |

<h2 id="extend-permissions-with-hooks">
  Étendre les autorisations avec des hooks
</h2>

Les [hooks Claude Code](/docs/fr/hooks-guide) vous permettent d'enregistrer des commandes shell personnalisées qui évaluent les autorisations à l'exécution. Lorsque Claude Code effectue un appel d'outil, les hooks PreToolUse s'exécutent avant l'invite d'autorisation, pour tous les outils sauf [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior). La sortie du hook peut refuser l'appel d'outil, forcer une invite ou ignorer l'invite pour laisser l'appel se poursuivre.

Les décisions du hook ne contournent pas les règles d'autorisation. Claude Code évalue les règles de refus et de demande indépendamment de ce qu'un hook PreToolUse retourne : une règle de refus correspondante bloque l'appel, et une règle de demande correspondante demande toujours même lorsque le hook a retourné `"allow"` ou `"ask"`. Cela préserve la précédence de refus en premier décrite dans [Gérer les autorisations](#manage-permissions), y compris les règles de refus définies dans les paramètres gérés.

Les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) demandent également toujours une invite lorsqu'un hook retourne `"allow"`, tout comme les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) dans les sessions où ce paramètre atteint Claude Code.

Un hook de blocage prend également la priorité sur les règles d'autorisation. Un hook qui se termine avec le code 2 arrête l'appel d'outil avant que les règles d'autorisation ne soient évaluées, donc le blocage s'applique même lorsqu'une règle d'autorisation permettrait autrement l'appel. Pour exécuter toutes les commandes Bash sans invites sauf pour quelques-unes que vous voulez bloquer, ajoutez `"Bash"` à votre liste d'autorisation et enregistrez un hook PreToolUse qui rejette ces commandes spécifiques. Consultez [Bloquer les éditions des fichiers protégés](/docs/fr/hooks-guide#block-edits-to-protected-files) pour un script de hook que vous pouvez adapter.

<h2 id="working-directories">
  Répertoires de travail
</h2>

Par défaut, Claude a accès aux fichiers du répertoire où vous l'avez lancé. Ce répertoire est le répertoire de travail principal de la session jusqu'à ce que vous [déplaciez la session avec `/cd`](#move-the-session-to-another-directory). Vous pouvez étendre cet accès :

* **Au démarrage** : utilisez l'argument CLI `--add-dir <path>`
* **Pendant la session** : utilisez la commande `/add-dir`
* **Configuration persistante** : ajoutez à `additionalDirectories` dans les [fichiers de paramètres](/docs/fr/settings#where-settings-live)

Les fichiers dans les répertoires supplémentaires suivent les mêmes règles d'autorisation que le répertoire de travail d'origine : ils deviennent lisibles sans invites, et les autorisations d'édition de fichiers suivent le mode d'autorisation actuel.

Vous ne pouvez pas ajouter la plupart des [chemins réseau](/docs/fr/errors#working-directory-is-a-network-path), tels que le partage UNC `\\server\share`, comme répertoires de travail, car leur recherche peut contacter l'hôte qu'ils désignent. Sur Windows, mappez plutôt le partage à une lettre de lecteur et transmettez le lecteur avec `--add-dir` au lancement.

Définissez [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) pour que les outils de fichiers refusent les chemins qu'il délimite dans chaque mode d'autorisation. En mode auto, Claude Code propose de l'activer la première fois que Claude [lit en dehors des répertoires de travail](/docs/fr/permission-modes#first-read-outside-the-working-directories).

Dans les sessions en arrière-plan sur macOS, l'hôte de session demande l'accès aux dossiers protégés tels que `~/Desktop`, `~/Documents` et `~/Downloads` séparément de votre terminal lorsque Claude doit lire ou écrire des fichiers là-bas ; si les lectures échouent avec `Operation not permitted`, consultez [comment accorder l'accès aux dossiers aux sessions en arrière-plan](/docs/fr/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos).

<h3 id="move-the-session-to-another-directory">
  Déplacer la session vers un autre répertoire
</h3>

Pour déplacer la session vers un répertoire de travail principal différent, plutôt que d'[ajouter un répertoire](#working-directories) à côté du répertoire actuel, exécutez `/cd <path>`. Claude Code conserve la conversation, charge le `CLAUDE.md` du nouveau répertoire, et vous demande de [faire confiance à l'espace de travail](#project-allow-rules-and-workspace-trust) si vous n'y avez pas travaillé auparavant. Ensuite, Claude Code [trouve la session déplacée](/docs/fr/sessions#resume-a-session) lorsque vous exécutez `--resume` à partir du nouveau répertoire.

Dès que vous vous déplacez, Claude Code applique la configuration de projet du nouveau répertoire :

* Ses paramètres de projet, y compris ses règles d'autorisation et ses [hooks](/docs/fr/hooks)
* Ses serveurs [`.mcp.json`](/docs/fr/mcp#project-scope), soumis à la même [approbation de serveur](/docs/fr/mcp#project-server-approvals-and-workspace-trust) qu'au démarrage, et les serveurs MCP [local-scope](/docs/fr/mcp#local-scope) que vous y avez enregistrés
* Les [plugins](/docs/fr/plugins/overview) que ses paramètres activent, ses [skills](/docs/fr/skills#discovery-from-parent-and-nested-directories), et ses [subagents](/docs/fr/sub-agents)
* Ses valeurs [`env`](/docs/fr/settings-reference#env), appliquées en plus des variables d'environnement des paramètres du répertoire précédent, qui restent en vigueur

Claude Code déconnecte également les serveurs MCP [local-scope](/docs/fr/mcp#local-scope) du répertoire précédent et du projet, ainsi que les serveurs des [plugins](/docs/fr/mcp#plugin-provided-mcp-servers) qui ne sont plus activés après le déplacement. Il prend les [répertoires supplémentaires](#working-directories) à partir des paramètres du nouveau répertoire au lieu de ceux du répertoire précédent, et conserve les répertoires que vous avez ajoutés avec `--add-dir` ou `/add-dir`. Les hooks que le déplacement active reçoivent toujours [`${CLAUDE_PROJECT_DIR}`](/docs/fr/hooks#reference-scripts-by-path) défini à la racine du projet où la session a commencé.

Lorsque le nouveau répertoire n'est pas encore approuvé, Claude Code énumère dans l'invite de confiance les règles d'autorisation, les répertoires supplémentaires, les hooks et les commandes d'assistance que les paramètres du répertoire activeraient, afin que vous puissiez les examiner avant d'accepter. Si vous refusez, la session reste où elle est. Avant la v2.1.246, `/cd` n'appliquait pas les paramètres, hooks, serveurs MCP ou skills du nouveau répertoire jusqu'à ce que vous repreniez la session, et son invite de confiance ne listait pas ce que les paramètres du répertoire activeraient.

Restreignez ou désactivez les cibles `/cd` avec les [règles d'autorisation `Cd`](#cd).

<h3 id="additional-directories-grant-file-access-not-configuration">
  Les répertoires supplémentaires accordent l'accès aux fichiers, pas la configuration
</h3>

L'ajout d'un répertoire étend l'endroit où Claude peut lire et éditer les fichiers. Cela ne fait pas de ce répertoire une racine de configuration complète : la plupart de la configuration `.claude/` n'est pas découverte à partir de répertoires supplémentaires, bien que quelques types soient chargés comme exceptions.

Ces exceptions s'appliquent uniquement aux répertoires ajoutés avec l'indicateur `--add-dir` ou la commande `/add-dir`, y compris les répertoires que le SDK Agent ajoute via l'indicateur. Les répertoires listés dans `permissions.additionalDirectories` dans un fichier de paramètres accordent uniquement l'accès aux fichiers et ne chargent aucune des configurations ci-dessous.

L'option [`additionalDirectories`](/docs/fr/agent-sdk/typescript#options) du SDK Agent en TypeScript et l'option [`add_dirs`](/docs/fr/agent-sdk/python#claudeagentoptions) en Python reçoivent également les exceptions, même si l'option TypeScript partage son nom avec la clé de paramètres. Le SDK transmet chaque entrée à Claude Code en tant que `--add-dir`, de sorte que ces répertoires se comportent comme des répertoires ajoutés par indicateur. Les skills, commandes et subagents de tout répertoire ajouté par indicateur se chargent via la [source de paramètres](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) `project`, ils ne se chargent donc pas lorsque vous excluez cette source avec [`--setting-sources`](/docs/fr/cli-reference) sur la CLI ou `settingSources` dans le SDK, et le [mode bare](/docs/fr/headless#start-faster-with-bare-mode) ignore les commandes et subagents parmi eux.

Les types de configuration suivants sont chargés à partir des répertoires `--add-dir` :

| Configuration                                                                            | Chargé à partir de `--add-dir`                                                                                                                                                         |
| :--------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/fr/skills) dans `.claude/skills/`                                              | Oui, avec rechargement en direct                                                                                                                                                       |
| [Fichiers de commande](/docs/fr/skills#where-skills-live) dans `.claude/commands/`            | Oui, sans rechargement en direct. Lorsque le répertoire ajouté et votre projet définissent tous deux une commande portant le même nom, Claude Code exécute la commande de votre projet |
| [Subagents](/docs/fr/sub-agents) dans `.claude/agents/`                                       | Oui, sans rechargement en direct                                                                                                                                                       |
| [Paramètres](/docs/fr/settings) dans `.claude/settings.json` et `.claude/settings.local.json` | Clés `enabledPlugins` et [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces) uniquement                                                                          |
| Fichiers [CLAUDE.md](/docs/fr/memory), `.claude/rules/` et `CLAUDE.local.md`                  | Uniquement lorsque `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` est défini. `CLAUDE.local.md` nécessite également la source de paramètres `local`, qui est activée par défaut      |

Pour charger les skills, commandes et subagents à partir d'un sous-répertoire de votre [répertoire de travail principal](#working-directories) en cours de session, exécutez `/add-dir` avec le chemin de ce sous-répertoire. Claude Code les charge pour le reste de la session sans vous demander confirmation ni ajouter de répertoire de travail, car le sous-répertoire est déjà lisible. Cela nécessite Claude Code v2.1.257 ou version ultérieure.

Claude Code découvre les styles de sortie à partir du répertoire de travail actuel et de ses parents, de votre répertoire utilisateur à `~/.claude/`, et des paramètres gérés. Les hooks et d'autres clés `.claude/settings.json` se chargent à partir du dossier `.claude/` du répertoire de travail actuel sans secours au répertoire parent, aux côtés de votre `~/.claude/settings.json` utilisateur et des paramètres gérés. `.claude/settings.local.json` se charge à partir de la racine du référentiel git à la place, même lorsque vous démarrez Claude Code dans un sous-répertoire, sauf dans les cas où Claude Code [n'utilise pas la racine du référentiel](/docs/fr/settings#where-claude-code-looks-for-each-file), comme sur Windows ; avant la v2.1.211, il se chargeait également uniquement à partir du répertoire de travail actuel. Les sessions du [SDK Agent](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) le chargent à partir du répertoire de travail dans toutes les versions.

Pour partager cette configuration entre les projets, utilisez l'une de ces approches :

* **Configuration au niveau utilisateur** : placez les fichiers dans `~/.claude/agents/`, `~/.claude/output-styles/` ou `~/.claude/settings.json` pour les rendre disponibles dans chaque projet
* **Plugins** : empaquetez et distribuez la configuration en tant que [plugin](/docs/fr/plugins/overview) que les équipes peuvent installer
* **Lancer à partir du répertoire de configuration** : exécutez Claude Code à partir du répertoire contenant la configuration `.claude/` que vous souhaitez

<h2 id="how-permissions-interact-with-sandboxing">
  Comment les autorisations interagissent avec le sandboxing
</h2>

Les autorisations et le [sandboxing](/docs/fr/sandboxing) sont des couches de sécurité complémentaires :

* **Les autorisations** contrôlent quels outils Claude Code peut utiliser et quels fichiers ou domaines il peut accéder. Elles s'appliquent à Bash, Read, Edit, WebFetch, MCP et tous les autres outils, sauf qu'une règle deny ou ask ne peut pas bloquer [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) tant que tout autre outil reste disponible.
* **Le sandboxing** fournit une application au niveau du système d'exploitation qui restreint l'accès du système de fichiers et du réseau des commandes shell. Il s'applique uniquement à Bash, PowerShell et aux commandes [Monitor](/docs/fr/tools-reference#monitor-tool) et à leurs processus enfants.

Utilisez les deux pour une défense en profondeur, car les restrictions de sandbox s'appliquent toujours même si une injection de prompt contourne la prise de décision de Claude. Les chemins et domaines des paramètres de sandbox et des règles d'autorisation sont [fusionnés dans la configuration finale du sandbox](/docs/fr/sandboxing#permission-rules).

Lorsque vous activez le sandboxing et laissez `autoAllowBashIfSandboxed` à sa valeur par défaut de `true`, les commandes Bash en sandbox s'exécutent sans invite même si vos autorisations incluent une règle ask `Bash` simple, ou la [forme équivalente `Bash(*)`](#match-all-uses-of-a-tool) : la limite du sandbox remplace cette invite pour l'outil entier.

En [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode), Claude Code ignore cette substitution. Sans règle ask, les [commandes en lecture seule intégrées](#read-only-commands) s'exécutent toujours sans invite, et toute autre commande shell passe par le flux d'autorisation régulier pendant que vous planifiez toujours ; consultez [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) pour savoir comment Claude Code contrôle les commandes là-bas. Avec une règle ask `Bash` simple, chaque commande Bash demande une invite, y compris les commandes en lecture seule en sandbox, de la même manière qu'en dehors du sandboxing. Avant la v2.1.212, la substitution s'appliquait également en mode plan.

Ces vérifications s'appliquent toujours :

* Les règles ask limitées au contenu comme `Bash(git push *)` forcent toujours une invite
* Les règles de refus explicites s'appliquent toujours
* Les commandes `rm` ou `rmdir` qui ciblent un [chemin critique](/docs/fr/permission-modes#critical-paths) passent toujours par le flux d'autorisation régulier

Les commandes qui ne s'exécutent pas en sandbox, comme les commandes exclues, respectent la règle ask `Bash` simple comme d'habitude. Consultez [modes sandbox](/docs/fr/sandboxing#sandbox-modes) pour modifier ce comportement.

<span id="managed-only-settings" />

<h2 id="managed-settings">
  Paramètres gérés
</h2>

Pour les organisations qui ont besoin d'un contrôle centralisé, les administrateurs déploient des paramètres gérés que les paramètres utilisateur et projet ne peuvent pas remplacer, à l'exception de quelques [clés sensibles à la sécurité](/docs/fr/settings#exceptions-to-managed-settings-precedence). [Déployer les paramètres gérés](/docs/fr/managed-settings) couvre les mécanismes de livraison, la précédence au sein du niveau géré, et les [clés que seuls les paramètres gérés peuvent définir](/docs/fr/managed-settings#managed-only-settings).

L'une de ces clés, [`allowManagedPermissionRulesOnly`](/docs/fr/settings-reference#allowmanagedpermissionrulesonly), fait des paramètres gérés la seule source de paramètres pour les règles d'autorisation. Son entrée énumère chaque source que Claude Code ignore ensuite.

`disableBypassPermissionsMode` est généralement placé dans les paramètres gérés pour appliquer la politique organisationnelle, mais il fonctionne à partir de n'importe quelle portée. Un utilisateur peut le définir dans ses propres paramètres pour se verrouiller hors du mode de contournement.

<h2 id="settings-precedence">
  Précédence des paramètres
</h2>

Les règles d'autorisation suivent la même [précédence des paramètres](/docs/fr/settings#settings-precedence) que tous les autres paramètres de Claude Code, avec les paramètres gérés au plus haut niveau : aucun autre niveau, y compris les arguments de ligne de commande, ne peut remplacer une règle d'autorisation gérée.

Si un outil est refusé à n'importe quel niveau, aucun autre niveau ne peut l'autoriser. Par exemple, un refus de paramètres gérés ne peut pas être remplacé par `--allowedTools`, et `--disallowedTools` peut ajouter des restrictions au-delà de ce que les paramètres gérés définissent.

Les mêmes règles s'appliquent dans les différentes portées de paramètres : si les paramètres utilisateur autorisent une autorisation et que les paramètres de projet la refusent, la règle de refus la bloque. L'inverse est également vrai : un refus au niveau utilisateur bloque une autorisation au niveau du projet, car les règles de refus de n'importe quelle portée sont évaluées avant les règles d'autorisation.

Les hôtes d'intégration peuvent fournir une politique gérée supplémentaire via l'option SDK `managedSettings`, y compris les règles d'autorisation d'autorisation, sauf si l'administrateur définit les verrous `allowManaged*Only` ; [Livrer une politique aux sessions Claude Desktop](/docs/fr/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) couvre le moment où la politique de l'intégrateur s'applique.

<h2 id="project-allow-rules-and-workspace-trust">
  Règles d'autorisation de projet et confiance de l'espace de travail
</h2>

Les règles `permissions.allow` et les entrées `permissions.additionalDirectories` dans le fichier `.claude/settings.json` d'un projet accordent des capacités, donc Claude Code les applique uniquement après que vous acceptiez la [boîte de dialogue de confiance de l'espace de travail](/docs/fr/security#additional-safeguards) pour ce dossier. La boîte de dialogue répertorie les règles et les répertoires que le dossier accorderait afin que vous puissiez les examiner en premier. Les règles `deny` et `ask` ne sont pas affectées, car elles ne font que restreindre.

Claude Code enregistre et stocke la confiance que vous acceptez selon l'endroit où vous le démarrez :

* Dans un référentiel, Claude Code enregistre la confiance sur la racine du référentiel git, donc la confiance couvre l'ensemble du référentiel à l'exception de tout référentiel git imbriqué à l'intérieur, comme un sous-module. Dans une [arborescence de travail](/docs/fr/worktrees), il utilise la racine du checkout principal, comme il le fait pour les [règles enregistrées](#permission-system).
* En dehors d'un référentiel, Claude Code enregistre la confiance sur le répertoire à partir duquel vous l'avez démarré, et la confiance couvre tout sous-répertoire de ce répertoire à l'exception d'un référentiel git imbriqué à l'intérieur, comme un clone. Chaque sous-répertoire couvert compte alors comme un dossier dont vous avez approuvé le parent.
* Lorsque vous démarrez dans votre répertoire personnel, Claude Code conserve la confiance pour la session actuelle uniquement et ne l'écrit pas sur le disque ; consultez la note sur les [protections supplémentaires](/docs/fr/security#additional-safeguards).

Claude Code affiche la boîte de dialogue de confiance dans les sessions interactives uniquement. Une exécution `claude -p` ou une session SDK ne l'affiche jamais, et faire confiance à un dossier parent ne compte pas pour ces règles, donc [Ce qui s'exécute avant que vous fassiez confiance à un dossier](#what-runs-before-you-trust-a-folder) indique quel contenu du référentiel Claude Code utilise toujours dans chacune de ces deux situations.

<h3 id="when-your-local-settings-file-needs-trust">
  Quand votre fichier de paramètres locaux a besoin de confiance
</h3>

`.claude/settings.local.json` est normalement votre propre fichier, donc Claude Code applique ses règles d'autorisation et ses répertoires supplémentaires sans l'étape de confiance. Lorsque le fichier est suivi dans git, ou que `.claude` est un lien symbolique, Claude Code le traite comme fourni par le référentiel à la place et retient ses règles sans les appliquer jusqu'à ce que vous fassiez confiance au dossier.

Claude Code exécute git pour distinguer les deux, et il n'exécute git qu'une fois que vous avez approuvé le dossier : vous avez accepté la boîte de dialogue de confiance pour celui-ci ou pour un répertoire parent dont la confiance s'étend à celui-ci, ou vous êtes dans une session `-p` ou SDK, ce qui compte comme accepté. Jusqu'à ce moment, l'endroit où vous avez démarré Claude Code décide ce qui se passe avec les règles du fichier :

* **Dans votre répertoire de configuration personnel :** Claude Code applique le `.claude/settings.local.json` de ce dossier immédiatement sans exécuter git. Votre répertoire de configuration personnel est votre répertoire personnel, ou un répertoire dont vous avez défini le sous-répertoire `.claude` comme [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars#variables). Si ce répertoire `CLAUDE_CONFIG_DIR` se trouve à l'intérieur d'un référentiel git et que Claude Code [conserve vos paramètres locaux à la racine du référentiel](/docs/fr/settings#where-claude-code-looks-for-each-file) à la place, il retient les règles sans les appliquer, comme partout ailleurs.
* **N'importe où ailleurs :** Claude Code retient les règles du fichier sans les appliquer, comme pour les paramètres du projet. Une fois la vérification exécutée, Claude Code applique les règles d'un fichier non suivi, ou d'un fichier dans un répertoire en dehors de tout référentiel git, même si vous n'avez pas approuvé ce dossier exact.

<Note>
  L'exception du répertoire de configuration ignore uniquement l'étape de confiance. `~/.claude/settings.local.json` est toujours une [portée locale](/docs/fr/settings#compare-the-scope-of-each-settings-file), donc Claude Code ne le lit que dans les sessions que vous démarrez dans votre répertoire personnel lui-même, pas dans chaque projet. Pour appliquer les règles de permission dans tous vos projets, ajoutez-les à vos paramètres utilisateur à la place : `~/.claude/settings.json`, ou `$CLAUDE_CONFIG_DIR/settings.json` lorsque `CLAUDE_CONFIG_DIR` est défini.
</Note>

Sur les versions 2.1.196 à 2.1.199, Claude Code conservait les règles du fichier dans votre répertoire de configuration personnel et en dehors des référentiels git aussi, et imprimait l'avertissement [`this workspace has not been trusted`](/docs/fr/errors#workspace-has-not-been-trusted) là-bas. Avant la v2.1.207, Claude Code appliquait les règles d'un fichier non suivi avant que vous acceptiez la boîte de dialogue.

<h3 id="what-runs-before-you-trust-a-folder">
  Ce qui s'exécute avant que vous fassiez confiance à un dossier
</h3>

Chaque ligne est un type de contenu qu'un référentiel peut fournir. Les colonnes sont les deux situations dans lesquelles vous n'avez pas approuvé le dossier lui-même : vous avez approuvé uniquement un dossier parent, ou vous avez exécuté `claude -p` ou le SDK là-bas, ce qui ne montre jamais la boîte de dialogue de confiance. La colonne du dossier parent ne s'applique pas à l'intérieur d'un [référentiel imbriqué](#project-allow-rules-and-workspace-trust) : dans une session interactive, Claude Code affiche la boîte de dialogue de confiance pour celui-ci, et une exécution `claude -p` ou SDK là-bas suit la colonne `claude -p`.

| Ce que le référentiel fournit                                                                                                                                                                                                                                                                                                                  | Vous avez approuvé uniquement un dossier parent                                                                                                                                                                                   | `claude -p` ou le SDK, dossier jamais approuvé                                                                                                                                                                      |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Hooks](/docs/fr/hooks) dans les fichiers de paramètres, le bloc [`env`](/docs/fr/settings-reference#env) et les commandes d'assistance telles que [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper), et les [hooks](/docs/fr/hooks#hooks-in-skills-and-agents) d'une compétence de projet et [`allowed-tools`](/docs/fr/skills#pre-approve-tools-for-a-skill) | Utilisé                                                                                                                                                                                                                           | Utilisé. La confiance de l'espace de travail ne bloque jamais les `allowed-tools` d'une compétence dans aucune session                                                                                              |
| Règles `permissions.allow` et `additionalDirectories` dans `.claude/settings.json`                                                                                                                                                                                                                                                             | Non utilisé jusqu'à ce que vous acceptiez la boîte de dialogue de confiance, qui réapparaît en les répertoriant                                                                                                                   | Non utilisé. Claude Code imprime un avertissement [`this workspace has not been trusted`](/docs/fr/errors#workspace-has-not-been-trusted) sur stderr                                                                     |
| Hooks de frontmatter dans un [sous-agent](/docs/fr/sub-agents#hooks-in-subagent-frontmatter) de projet, un plugin [`@skills-dir`](/docs/fr/plugins/loading#plugins-shared-through-a-repository) de projet, et les entrées [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces) du référentiel ou d'un répertoire `--add-dir`        | Non utilisé, et aucune boîte de dialogue n'est proposée                                                                                                                                                                           | Non utilisé                                                                                                                                                                                                         |
| [`mcpServers`](/docs/fr/sub-agents#scope-mcp-servers-to-a-subagent) en ligne dans le frontmatter d'un sous-agent du référentiel ou d'un répertoire `--add-dir`. Avant la v2.1.238, Claude Code chargeait ces serveurs dans les deux situations                                                                                                      | Non utilisé, et aucune boîte de dialogue n'est proposée                                                                                                                                                                           | Non utilisé                                                                                                                                                                                                         |
| Serveurs dans `.mcp.json`, y compris ceux que le référentiel [approuve dans ses propres paramètres](/docs/fr/mcp#project-server-approvals-and-workspace-trust)                                                                                                                                                                                      | Claude Code vous demande avant de vous y connecter. Les approbations du référentiel lui-même ne comptent pas                                                                                                                      | Connecté sans demander, approuvé ou non. Le SDK ne les charge que lorsque `settingSources` inclut les paramètres du projet. `claude mcp list` dans le même dossier signale toujours un tel serveur comme en attente |
| Un [`headersHelper`](/docs/fr/mcp#trust-a-folder-before-its-headershelper-runs) sur un serveur dans `.mcp.json`. Avant la v2.1.238, Claude Code exécutait l'assistant dans les deux situations                                                                                                                                                      | Non exécuté jusqu'à ce que vous acceptiez la boîte de dialogue de confiance, qui réapparaît en nommant l'endroit où l'assistant est déclaré. Claude Code connecte le serveur avec ses `headers` statiques seuls jusqu'à ce moment | Non exécuté. Claude Code connecte le serveur avec ses `headers` statiques seuls et imprime une ligne [`headersHelper not run`](/docs/fr/errors#headershelper-not-run) par serveur sur stderr                             |

Pour les lignes qui ont besoin de ce dossier exact approuvé, approuvez-le manuellement : définissez `projects["<path>"].hasTrustDialogAccepted` sur `true` dans `~/.claude.json`, où `<path>` est la racine du référentiel, ou le dossier lui-même en dehors d'un référentiel. Claude Code imprime la clé exacte dans la ligne du journal de débogage pour un hook de sous-agent ignoré ou un serveur MCP en ligne, dans l'avertissement stderr pour les règles d'autorisation ignorées, et dans la ligne `headersHelper not run` pour un assistant ignoré.

Avant d'exécuter `claude -p` dans un référentiel que vous n'avez pas écrit, décidez ce qu'il peut exécuter sur votre machine :

* Passez `--setting-sources user`, ou définissez le `settingSources` du SDK sans paramètres de projet, afin que Claude Code ne lise ni les fichiers de paramètres du projet ni son `.mcp.json`
* Commencez avec [`--bare`](/docs/fr/headless#start-faster-with-bare-mode) afin que Claude Code ne lise aucun hook, compétence, commande personnalisée, sous-agent, plugin ou serveur `.mcp.json` du projet. Le bloc `env` du projet et les assistants tels que `awsAuthRefresh` dans ses fichiers de paramètres s'appliquent toujours, et Claude Code ne lit `apiKeyHelper` que depuis `--settings`
* Passez `--settings '{"disableAllHooks": true}'` pour [désactiver les hooks](/docs/fr/hooks#disable-or-remove-hooks) pour cette exécution. Le définir uniquement dans vos paramètres utilisateur ne suffit pas, car les paramètres du projet du référentiel ont la priorité sur les vôtres et peuvent le redéfinir sur `false`
* Ajoutez une entrée [`disabledMcpjsonServers`](/docs/fr/settings-reference#disabledmcpjsonservers) pour rejeter un serveur `.mcp.json` par nom dans chaque type de session

<h2 id="example-configurations">
  Exemples de configurations
</h2>

Ce [référentiel](https://github.com/anthropics/claude-code/tree/main/examples/settings) inclut des configurations de paramètres de démarrage pour les scénarios de déploiement courants. Utilisez-les comme points de départ et ajustez-les pour répondre à vos besoins.

<h2 id="see-also">
  Voir aussi
</h2>

* [Tous les paramètres](/docs/fr/settings-reference#permission-settings) : chaque clé de paramètre, y compris les clés d'autorisation
* [Configurer le mode auto](/docs/fr/auto-mode-config) : dites au classificateur du mode auto quelle infrastructure votre organisation approuve
* [Sandboxing](/docs/fr/sandboxing) : isolation du système de fichiers et du réseau au niveau du système d'exploitation pour les commandes Bash
* [Authentification](/docs/fr/authentication) : configurer l'accès utilisateur à Claude Code
* [Sécurité](/docs/fr/security) : garanties de sécurité et meilleures pratiques
* [Hooks](/docs/fr/hooks-guide) : automatiser les flux de travail et étendre l'évaluation des autorisations
