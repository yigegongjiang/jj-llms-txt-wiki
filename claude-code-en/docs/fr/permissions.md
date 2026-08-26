> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer les autorisations

> Contrôlez ce que Claude Code peut accéder et faire avec des règles d'autorisation granulaires, des modes et des politiques gérées.

Claude Code prend en charge les autorisations granulaires afin que vous puissiez spécifier exactement ce que l'agent est autorisé à faire et ce qu'il ne peut pas faire. Les paramètres d'autorisation peuvent être archivés dans le contrôle de version et distribués à tous les développeurs de votre organisation, ainsi que personnalisés par les développeurs individuels.

<h2 id="permission-system">
  Système d'autorisation
</h2>

Claude Code utilise un système d'autorisation à plusieurs niveaux pour équilibrer la puissance et la sécurité :

| Type d'outil            | Exemple                      | Approbation requise                                                                       | Comportement « Oui, ne pas demander à nouveau » |
| :---------------------- | :--------------------------- | :---------------------------------------------------------------------------------------- | :---------------------------------------------- |
| Lecture seule           | Lectures de fichiers, Grep   | Non, dans le [répertoire de travail et répertoires supplémentaires](#working-directories) | S/O                                             |
| Commandes Bash          | Exécution shell              | Oui, sauf un ensemble intégré de [commandes en lecture seule](#read-only-commands)        | Permanent par répertoire de projet et commande  |
| Modification de fichier | Édition/écriture de fichiers | Oui                                                                                       | Jusqu'à la fin de la session                    |

Sur une invite d'autorisation Bash ou PowerShell, appuyez sur `Ctrl+E` pour afficher une explication de la commande : ce qu'elle fait, pourquoi Claude l'exécute, et ce qui pourrait mal se passer, étiqueté **Risque faible**, **Risque moyen**, ou **Risque élevé**. Claude Code envoie la commande et la propre description de Claude de l'appel au modèle pour générer l'explication uniquement lorsque vous appuyez sur `Ctrl+E`, pas à chaque invite. L'affichage de l'explication n'exécute pas la commande ; appuyez sur `Ctrl+E` à nouveau pour la masquer.

Pour désactiver le raccourci, définissez [`permissionExplainerEnabled`](/docs/fr/settings#global-config-settings) sur `false` dans `~/.claude.json`.

<h2 id="manage-permissions">
  Gérer les autorisations
</h2>

Vous pouvez afficher et gérer les autorisations d'outils de Claude Code avec `/permissions`. Cette interface utilisateur répertorie toutes les règles d'autorisation et le fichier `settings.json` dont elles proviennent.

* Les règles **Allow** permettent à Claude Code d'utiliser l'outil spécifié sans approbation manuelle.
* Les règles **Ask** demandent une confirmation chaque fois que Claude Code essaie d'utiliser l'outil spécifié.
* Les règles **Deny** empêchent Claude Code d'utiliser l'outil spécifié.

Les règles sont évaluées dans l'ordre : deny, puis ask, puis allow. La première correspondance dans cet ordre détermine le résultat, et la spécificité de la règle ne change pas l'ordre.

Une règle deny large comme `Bash(aws *)` bloque chaque appel correspondant, y compris les appels qui correspondent également à une règle allow plus étroite comme `Bash(aws s3 ls)`, donc une règle deny ne peut pas comporter d'exceptions de liste d'autorisation. La même priorité s'applique entre ask et allow : une règle ask correspondante demande une confirmation même lorsqu'une règle allow plus spécifique correspond également au même appel.

Les règles deny se comportent différemment selon qu'elles nomment un outil ou qu'elles délimitent un motif au sein de celui-ci. Un nom d'outil simple comme `Bash` supprime l'outil du contexte de Claude entièrement, donc Claude ne le voit jamais. Une règle délimitée comme `Bash(rm *)` laisse l'outil disponible et bloque les appels correspondants lorsque Claude essaie de les utiliser.

<Note>
  Les règles d'autorisation sont appliquées par Claude Code, et non par le modèle. Les instructions dans votre prompt ou `CLAUDE.md` façonnent ce que Claude essaie de faire, mais elles ne changent pas ce que Claude Code autorise. Pour accorder ou révoquer l'accès, utilisez `/permissions`, les règles décrites ici, un [mode d'autorisation](/docs/fr/permission-modes), ou un [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

<h2 id="permission-modes">
  Modes d'autorisation
</h2>

Claude Code prend en charge plusieurs modes d'autorisation qui contrôlent la façon dont il approuve les appels d'outils. Consultez [Modes d'autorisation](/docs/fr/permission-modes) pour savoir quand utiliser chacun. Définissez le `defaultMode` dans vos [fichiers de paramètres](/docs/fr/settings#settings-files) :

| Mode                | Description                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Comportement standard : demande une autorisation à la première utilisation de chaque outil. Étiqueté Manuel dans l'interface de ligne de commande, les extensions VS Code et JetBrains, et l'application de bureau, et Claude Code accepte `manual` comme alias. L'étiquette et l'alias nécessitent Claude Code v2.1.200 ou version ultérieure. L'étiquette de l'application de bureau ne dépend pas de votre version CLI                               |
| `acceptEdits`       | Accepte automatiquement les éditions de fichiers et les commandes courantes du système de fichiers telles que `mkdir`, `touch`, `mv` et `cp` pour les chemins du répertoire de travail ou `additionalDirectories`                                                                                                                                                                                                                                       |
| `plan`              | Claude lit les fichiers et exécute les commandes shell en lecture seule pour explorer mais n'édite pas vos fichiers source. Étiqueté Plan dans l'interface de ligne de commande et l'extension VS Code                                                                                                                                                                                                                                                  |
| `auto`              | Approuve automatiquement les appels d'outils avec des vérifications de sécurité en arrière-plan qui vérifient que les actions s'alignent avec votre demande                                                                                                                                                                                                                                                                                             |
| `dontAsk`           | Refuse automatiquement les outils sauf s'ils sont pré-approuvés via `/permissions` ou les règles `permissions.allow`. `AskUserQuestion`, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) sont refusés même si vous les avez autorisés                                         |
| `bypassPermissions` | Ignore les invites d'autorisation, sauf celles forcées par les règles `ask` explicites, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool). Les suppressions de répertoires racine et personnel telles que `rm -rf /` demandent également toujours comme disjoncteur de sécurité |

<Warning>
  Le mode `bypassPermissions` ignore les invites d'autorisation, y compris les écritures dans `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` et `.mvn`. Utilisez ce mode uniquement dans des environnements isolés comme les conteneurs ou les machines virtuelles où Claude Code ne peut pas causer de dommages.

  Quelques invites se déclenchent toujours dans ce mode. Les règles `ask` explicites, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) demandent toujours. Les suppressions ciblant la racine du système de fichiers ou le répertoire personnel, telles que `rm -rf /` et `rm -rf ~`, demandent également comme disjoncteur de sécurité contre l'erreur du modèle, y compris lorsque la commande contient une substitution de commande avec `$(...)` ou des backticks, ou une substitution de processus avec `<(...)`. Avant v2.1.208, seule la forme simple, telle que `rm -rf ~` tapée comme sa propre commande, demandait ; les commandes qui atteignaient la suppression par une substitution ne demandaient pas.
</Warning>

Pour empêcher le mode `bypassPermissions` ou `auto` d'être utilisé, définissez `permissions.disableBypassPermissionsMode` ou `permissions.disableAutoMode` sur `"disable"` dans n'importe quel [fichier de paramètres](/docs/fr/settings#settings-files). Ces paramètres sont particulièrement utiles dans les [paramètres gérés](#managed-settings) où ils ne peuvent pas être remplacés.

<h2 id="permission-rule-syntax">
  Syntaxe des règles d'autorisation
</h2>

Les règles d'autorisation suivent le format `Tool` ou `Tool(specifier)`.

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

Les règles de refus et de demande peuvent correspondre à un paramètre d'entrée de haut niveau sur n'importe quel outil avec `Tool(param:value)`. La règle correspond lorsque Claude appelle l'outil avec ce paramètre défini sur cette valeur exacte. Une règle d'autorisation pour une valeur de paramètre n'établirait pas que l'appel est sûr dans l'ensemble, donc les règles d'autorisation continuent à utiliser la syntaxe de spécificateur propre à chaque outil. Cela fonctionne pour n'importe quel paramètre scalaire que l'outil accepte :

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

Les champs qu'un outil correspond déjà avec ses propres règles de canonicalisation ne sont pas correspondables de cette façon : `command` pour Bash et PowerShell, `file_path` pour Read, Edit et Write, `path` pour Grep et Glob, `notebook_path` pour NotebookEdit, et `url` pour WebFetch. Une règle comme `Bash(command:rm *)` serait contournable par une commande composée, donc Claude Code l'ignore et émet un avertissement au démarrage. Utilisez plutôt `Bash(rm *)`, `Read(./path)`, ou `WebFetch(domain:host)`.

<h3 id="wildcard-patterns">
  Modèles de caractères génériques
</h3>

Les règles Bash prennent en charge les modèles glob avec `*`. Les caractères génériques peuvent apparaître à n'importe quelle position dans la commande. Cette configuration permet les commandes npm et git commit tout en bloquant git push :

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)",
      "Bash(git * main)",
      "Bash(* --version)",
      "Bash(* --help *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

L'espace avant `*` est important : `Bash(ls *)` correspond à `ls -la` mais pas à `lsof`, tandis que `Bash(ls*)` correspond aux deux. Le suffixe `:*` est une façon équivalente d'écrire un caractère générique de fin, donc `Bash(ls:*)` correspond aux mêmes commandes que `Bash(ls *)`.

La boîte de dialogue d'autorisation écrit la forme séparée par des espaces lorsque vous sélectionnez « Oui, ne pas demander à nouveau » pour un préfixe de commande. La forme `:*` n'est reconnue qu'à la fin d'un modèle. Dans un modèle comme `Bash(git:* push)`, le deux-points est traité comme un caractère littéral et ne correspondra pas aux commandes git.

<h3 id="tool-name-wildcards">
  Caractères génériques de nom d'outil
</h3>

Les règles de refus et de demande acceptent également les modèles glob dans la position du nom d'outil. Le modèle doit correspondre au nom d'outil complet : `"*"` correspond à chaque outil, et `"mcp__*"` correspond à chaque outil MCP sur tous les serveurs. Un outil correspondant à une règle de refus de nom nu est supprimé du contexte de Claude, de la même manière qu'un nom d'outil nu. Cette configuration refuse chaque outil MCP :

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

Une règle de refus ou de demande dont le nom d'outil ne correspond à aucun outil connu produit un avertissement au démarrage pour détecter les fautes de frappe. Les noms d'outils contenant `_` ou `*` sont exemptés de la vérification.

L'étiquette affichée pour un outil dans la transcription et la boîte de dialogue d'autorisation peut différer de son nom canonique. Par exemple, l'outil étiqueté `Stop Task` dans la transcription a le nom canonique `TaskStop`. Les règles d'autorisation et les [correspondances de hook](/docs/fr/hooks) correspondent uniquement au nom canonique, donc une règle écrite comme `Stop Task` ne correspond pas. Pour les règles de refus et de demande, l'avertissement au démarrage ci-dessus détecte l'inadéquation. Utilisez les noms canoniques listés dans la [référence des outils](/docs/fr/tools-reference).

<h2 id="tool-specific-permission-rules">
  Règles d'autorisation spécifiques aux outils
</h2>

<h3 id="bash">
  Bash
</h3>

Les règles d'autorisation Bash prennent en charge la correspondance de caractères génériques avec `*`. Les caractères génériques peuvent apparaître à n'importe quelle position dans la commande, y compris au début, au milieu ou à la fin :

* `Bash(npm run build)` correspond à la commande Bash exacte `npm run build`
* `Bash(npm run test *)` correspond aux commandes Bash commençant par `npm run test`
* `Bash(npm *)` correspond à toute commande commençant par `npm `
* `Bash(* install)` correspond à toute commande se terminant par ` install`
* `Bash(git * main)` correspond à des commandes comme `git checkout main` et `git log --oneline main`

Un seul `*` correspond à n'importe quelle séquence de caractères, y compris les espaces, donc un caractère générique peut s'étendre sur plusieurs arguments. `Bash(git *)` correspond à `git log --oneline --all`, et `Bash(git * main)` correspond à `git push origin main` ainsi qu'à `git merge main`.

Lorsque `*` apparaît à la fin avec un espace avant (comme `Bash(ls *)`), il applique une limite de mot, exigeant que le préfixe soit suivi d'un espace ou de la fin de la chaîne. Par exemple, `Bash(ls *)` correspond à `ls -la` mais pas à `lsof`. En contraste, `Bash(ls*)` sans espace correspond aux deux `ls -la` et `lsof` car il n'y a pas de contrainte de limite de mot.

<h4 id="compound-commands">
  Commandes composées
</h4>

<Tip>
  Claude Code est conscient des opérateurs shell, donc une règle comme `Bash(safe-cmd *)` ne lui donnera pas la permission d'exécuter la commande `safe-cmd && other-cmd`. Les séparateurs de commande reconnus sont `&&`, `||`, `;`, `|`, `|&`, `&` et les sauts de ligne. Une règle doit correspondre à chaque sous-commande indépendamment.
</Tip>

Lorsque vous approuvez une commande composée avec « Oui, ne pas demander à nouveau », Claude Code enregistre une règle séparée pour chaque sous-commande qui nécessite une approbation, plutôt qu'une seule règle pour la chaîne complète. Par exemple, approuver `git status && npm test` enregistre une règle pour `npm test`, donc les invocations futures de `npm test` sont reconnues indépendamment de ce qui précède le `&&`. Les sous-commandes comme `cd` dans un sous-répertoire génèrent leur propre règle Read pour ce chemin. Jusqu'à 5 règles peuvent être enregistrées pour une seule commande composée.

<h4 id="process-wrappers">
  Wrappers de processus
</h4>

Avant de correspondre aux règles Bash, Claude Code supprime un ensemble fixe de wrappers de processus afin qu'une règle comme `Bash(npm test *)` corresponde également à `timeout 30 npm test`. Les wrappers reconnus sont `timeout`, `time`, `nice`, `nohup` et `stdbuf`.

Le `xargs` nu est également supprimé, donc `Bash(grep *)` correspond à `xargs grep pattern`. La suppression s'applique uniquement lorsque `xargs` n'a pas de drapeaux : une invocation comme `xargs -n1 grep pattern` est mise en correspondance en tant que commande `xargs`, donc les règles écrites pour la commande interne ne la couvrent pas.

Cette liste de wrappers est intégrée et n'est pas configurable. Les exécuteurs d'environnement de développement tels que `direnv exec`, `devbox run`, `mise exec`, `npx` et `docker exec` ne figurent pas dans la liste. Parce que ces outils exécutent leurs arguments en tant que commande, une règle comme `Bash(devbox run *)` correspond à tout ce qui vient après `run`, y compris `devbox run rm -rf .`. Pour approuver le travail à l'intérieur d'un exécuteur d'environnement, écrivez une règle spécifique qui inclut à la fois l'exécuteur et la commande interne, comme `Bash(devbox run npm test)`. Ajoutez une règle par commande interne que vous souhaitez autoriser.

Les wrappers exec tels que `watch`, `setsid`, `ionice` et `flock` demandent toujours et ne peuvent pas être approuvés automatiquement par une règle de préfixe comme `Bash(watch *)`. Il en va de même pour `find` avec `-exec` ou `-delete` : une règle `Bash(find *)` ne couvre pas ces formes. Pour approuver une invocation spécifique, écrivez une règle de correspondance exacte pour la chaîne de commande complète.

<h4 id="read-only-commands">
  Commandes en lecture seule
</h4>

Claude Code reconnaît un ensemble intégré de commandes Bash comme étant en lecture seule et les exécute sans invite d'autorisation dans tous les modes. Ceux-ci incluent `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` et les formes en lecture seule de `git`. L'ensemble n'est pas configurable ; pour exiger une invite pour l'une de ces commandes, ajoutez une règle `ask` ou `deny` pour celle-ci.

Les modèles glob non cités sont autorisés pour les commandes dont chaque drapeau est en lecture seule, donc `ls *.ts` et `wc -l src/*.py` s'exécutent sans invite. Les commandes avec des drapeaux capables d'écriture ou d'exécution, tels que `find`, `sort`, `sed` et `git`, demandent toujours lorsqu'un glob non cité est présent car le glob pourrait s'étendre à un drapeau comme `-delete`.

Un `cd` dans un chemin à l'intérieur de votre répertoire de travail ou d'un [répertoire supplémentaire](#working-directories) est également en lecture seule. Une commande composée comme `cd packages/api && ls` s'exécute sans invite lorsque chaque partie se qualifie seule. La combinaison de `cd` avec `git` dans une seule commande composée demande toujours lorsque le `cd` change dans un répertoire différent, car exécuter `git` dans un nouveau répertoire peut exécuter les hooks de ce répertoire. Un `cd` dont la cible se résout au répertoire de travail courant est une non-opération et ne déclenche pas cette invite.

La combinaison de `cd` avec une redirection de sortie dans une seule commande composée demande également lorsque Claude Code ne peut pas déterminer le répertoire dans lequel la cible de redirection se résout après l'exécution de `cd`. Une commande dont la seule cible de redirection est `/dev/null`, comme `cd app; grep -r pattern . 2>/dev/null`, ne déclenche pas cette invite, car `/dev/null` ne dépend pas du répertoire de travail. Avant la v2.1.207, une commande composée contenant `cd` demandait pour toute redirection de sortie, y compris une dont la seule cible était `/dev/null`.

<Warning>
  Les modèles d'autorisation Bash qui tentent de contraindre les arguments de commande sont fragiles. Par exemple, `Bash(curl http://github.com/ *)` a l'intention de restreindre curl aux URL GitHub, mais ne correspondra pas aux variations comme :

  * Options avant l'URL : `curl -X GET http://github.com/...`
  * Protocole différent : `curl https://github.com/...`
  * Redirections : `curl -L http://bit.ly/xyz`, qui redirige vers GitHub
  * Variables : `URL=http://github.com && curl $URL`
  * Espaces supplémentaires : `curl  http://github.com`

  Pour un filtrage d'URL plus fiable, envisagez :

  * **Restreindre les outils réseau Bash** : utilisez les règles de refus pour bloquer `curl`, `wget` et les commandes similaires, puis utilisez l'outil WebFetch avec l'autorisation `WebFetch(domain:github.com)` pour les domaines autorisés
  * **Utiliser les hooks PreToolUse** : implémentez un hook qui valide les URL dans les commandes Bash et bloque les domaines non autorisés
  * **Ajouter des conseils CLAUDE.md** : décrivez vos modèles curl autorisés dans `CLAUDE.md`. Cela façonne ce que Claude essaie mais n'applique pas une limite, donc associez-le à l'une des options ci-dessus

  Notez que l'utilisation de WebFetch seul n'empêche pas l'accès au réseau. Si Bash est autorisé, Claude peut toujours utiliser `curl`, `wget` ou d'autres outils pour atteindre n'importe quelle URL.
</Warning>

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

Les règles `Edit` s'appliquent à tous les outils intégrés qui éditent les fichiers. Claude fait un effort raisonnable pour appliquer les règles `Read` à tous les outils intégrés qui lisent les fichiers comme Grep et Glob, aux mentions `@file` dans vos invites et à la sélection et au contexte de fichier ouvert qu'un [IDE](/docs/fr/vs-code#the-built-in-ide-mcp-server) connecté partage avec Claude.

Une règle de refus `Read` bloque également l'[outil Edit](/docs/fr/errors#file-is-covered-by-a-read-deny-rule) sur le même chemin, y compris la création d'un nouveau fichier là-bas. Write et NotebookEdit ne sont pas couverts, donc ajoutez une règle de refus `Edit` pour les chemins qu'aucun outil ne peut modifier. Nécessite Claude Code v2.1.208 ou ultérieur.

<Warning>
  Les règles de refus Read et Edit s'appliquent aux outils de fichiers intégrés de Claude et aux commandes de fichiers que Claude Code reconnaît dans Bash, tels que `cat`, `head`, `tail` et `sed`. Elles ne s'appliquent pas aux sous-processus arbitraires qui lisent ou écrivent des fichiers indirectement, comme un script Python ou Node qui ouvre des fichiers lui-même. Pour une application au niveau du système d'exploitation qui bloque tous les processus d'accéder à un chemin, [activez le sandbox](/docs/fr/sandboxing).
</Warning>

Les règles Read et Edit suivent toutes deux la spécification [gitignore](https://git-scm.com/docs/gitignore) avec quatre types de modèles distincts :

| Modèle             | Signification                                              | Exemple                          | Correspond                                                     |
| ------------------ | ---------------------------------------------------------- | -------------------------------- | -------------------------------------------------------------- |
| `//path`           | Chemin absolu à partir de la racine du système de fichiers | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                      |
| `~/path`           | Chemin à partir du répertoire home                         | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                                 |
| `/path`            | Chemin relatif à la source des paramètres                  | `Edit(/src/**/*.ts)`             | `<racine du projet>/src/**/*.ts` dans les paramètres du projet |
| `path` ou `./path` | Chemin relatif au répertoire courant                       | `Read(*.env)`                    | `<cwd>/*.env`                                                  |

<Warning>
  Un modèle comme `/Users/alice/file` n'est pas un chemin absolu. La barre oblique unique en début ancre à la source des paramètres, pas à la racine du système de fichiers. Utilisez `//Users/alice/file` pour les chemins absolus.
</Warning>

Un modèle `/path` s'ancre au répertoire associé au fichier de paramètres qui le définit, donc la même règle correspond à des emplacements différents selon l'endroit où vous la placez :

| Règle définie dans                                               | `/path` se résout à            |
| :--------------------------------------------------------------- | :----------------------------- |
| Paramètres du projet ou locaux, tels que `.claude/settings.json` | `<racine du projet>/path`      |
| Paramètres utilisateur à `~/.claude/settings.json`               | `~/.claude/path`               |
| Un fichier passé avec `--settings <file>`                        | `<répertoire du fichier>/path` |
| Drapeaux CLI, `/permissions` ou règles de session                | `<cwd original>/path`          |

Une règle de refus comme `Read(/secrets/**)` dans les paramètres utilisateur bloque `~/.claude/secrets/**`, pas un répertoire `secrets` dans votre projet. Pour écrire une règle dans les paramètres utilisateur qui s'applique à l'intérieur de chaque projet, utilisez plutôt un chemin absolu `//` ou un chemin relatif à home `~/`.

Sur Windows, les chemins sont normalisés en forme POSIX avant la correspondance. `C:\Users\alice` devient `/c/Users/alice`, donc utilisez `//c/**/.env` pour correspondre aux fichiers `.env` n'importe où sur ce lecteur. Pour correspondre sur tous les lecteurs, utilisez `//**/.env`.

Exemples :

* `Edit(/docs/**)` : édite dans `<projet>/docs/`, pas `/docs/` ou `<projet>/.claude/docs/`
* `Read(~/.zshrc)` : lit le `.zshrc` de votre répertoire home
* `Edit(//tmp/scratch.txt)` : édite le chemin absolu `/tmp/scratch.txt`
* `Read(src/**)` : lit à partir de `<répertoire courant>/src/`

Une règle ne correspond qu'aux fichiers sous son ancrage, donc l'ancrage détermine jusqu'où une règle de refus s'étend. Les noms de fichiers nus suivent la sémantique gitignore et correspondent à n'importe quelle profondeur, donc `Read(.env)` et `Read(**/.env)` sont équivalents :

| Règle de refus                  | Bloque                                              | Ne bloque pas                                                 |
| ------------------------------- | --------------------------------------------------- | ------------------------------------------------------------- |
| `Read(.env)` ou `Read(**/.env)` | tout `.env` au ou sous le répertoire courant        | `.env` dans un répertoire parent ou un autre projet           |
| `Read(//**/.env)`               | tout `.env` n'importe où sur le système de fichiers | rien ; la règle est ancrée à la racine du système de fichiers |

<Note>
  Dans les modèles gitignore, `*` correspond aux fichiers dans un seul segment de chemin et peut apparaître à n'importe quelle position du modèle, tandis que `**` correspond sur les répertoires. Pour autoriser tous les accès aux fichiers, utilisez simplement le nom de l'outil sans parenthèses : `Read`, `Edit` ou `Write`.
</Note>

Lorsque vous approuvez un chemin de fichier avec « Oui, ne pas demander à nouveau », Claude Code échappe les caractères de modèle gitignore dans ce chemin, tels que `[`, `]` et `*`, afin que la règle générée ne corresponde qu'au chemin littéral que vous avez approuvé. Les règles que vous écrivez vous-même ne sont pas échappées. Avant la v2.1.202, Claude Code enregistrait le chemin non échappé, donc une règle générée pour un répertoire nommé `[2024-06] Reports` pouvait échouer à correspondre à son propre chemin ou correspondre à des répertoires frères non intentionnels.

Lorsque Claude accède à un lien symbolique, les règles d'autorisation vérifient deux chemins : le lien symbolique lui-même et le fichier vers lequel il se résout. Les règles d'autorisation et de refus traitent cette paire différemment : les règles d'autorisation reviennent à vous inviter, tandis que les règles de refus bloquent carrément.

* **Règles d'autorisation** : s'appliquent uniquement lorsque le chemin du lien symbolique et sa cible correspondent tous les deux. Un lien symbolique à l'intérieur d'un répertoire autorisé qui pointe vers l'extérieur vous invite toujours.
* **Règles de refus** : s'appliquent lorsque le chemin du lien symbolique ou sa cible correspond. Un lien symbolique qui pointe vers un fichier refusé est lui-même refusé.

Par exemple, avec `Read(./project/**)` autorisé et `Read(~/.ssh/**)` refusé, un lien symbolique à `./project/key` pointant vers `~/.ssh/id_rsa` est bloqué : la cible échoue à la règle d'autorisation et correspond à la règle de refus.

<h3 id="webfetch">
  WebFetch
</h3>

Les règles WebFetch utilisent un préfixe `domain:` et correspondent au nom d'hôte de l'URL demandée. La correspondance est insensible à la casse, prend en charge les caractères génériques `*` et supprime un point final des deux côtés de la règle et du nom d'hôte afin que `example.com.` et `example.com` soient traités de la même manière.

* `WebFetch(domain:example.com)` correspond aux demandes vers `example.com`
* `WebFetch(domain:*.example.com)` correspond à tout sous-domaine à n'importe quelle profondeur, comme `api.example.com` ou `a.b.example.com`, mais pas à `example.com` lui-même
* `WebFetch(domain:*)` correspond à chaque domaine et est équivalent à une règle `WebFetch` nu

En toute position autre qu'un `*.` initial ou un `*` nu, le caractère générique correspond uniquement au texte entre deux points. `WebFetch(domain:example.*)` correspond à `example.org`, où `*` devient `org`, mais pas à `example.evil.com`, où `*` devrait devenir `evil.com` et traverser un point. Cela empêche un caractère générique de fin de correspondre à des domaines qu'un attaquant pourrait enregistrer.

<h3 id="mcp">
  MCP
</h3>

Les règles MCP utilisent le nom du serveur tel que configuré dans Claude Code, optionnellement suivi du nom d'un outil de ce serveur.

* `mcp__puppeteer` correspond à tout outil fourni par le serveur `puppeteer`
* `mcp__puppeteer__*` utilise la syntaxe de caractère générique et correspond également à tous les outils du serveur `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` correspond à l'outil `puppeteer_navigate` fourni par le serveur `puppeteer`

Si votre organisation a défini un outil de connecteur [claude.ai](/docs/fr/mcp#organization-controls-on-connector-tools) sur `ask`, les règles d'autorisation pour cet outil ne prennent pas effet : Claude Code demande à chaque appel, même en modes `auto` et `bypassPermissions`. En mode `dontAsk`, qui ne demande jamais, Claude Code refuse l'appel à la place. Les outils de connecteur apparaissent comme `mcp__claude_ai_<server>__<tool>`.

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

| Règle                 | Correspond                                                   | Ne correspond pas                 |
| --------------------- | ------------------------------------------------------------ | --------------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                                 | `~/code/app/src`, `~/code`        |
| `Cd(~/code/**)`       | `~/code` et tout répertoire sous celui-ci                    | répertoires en dehors de `~/code` |
| `Cd(**/node_modules)` | tout répertoire `node_modules` à n'importe quelle profondeur | `node_modules/pkg`                |

<h2 id="extend-permissions-with-hooks">
  Étendre les autorisations avec des hooks
</h2>

Les [hooks Claude Code](/docs/fr/hooks-guide) fournissent un moyen d'enregistrer des commandes shell personnalisées pour effectuer l'évaluation des autorisations à l'exécution. Lorsque Claude Code effectue un appel d'outil, les hooks PreToolUse s'exécutent avant l'invite d'autorisation. La sortie du hook peut refuser l'appel d'outil, forcer une invite ou ignorer l'invite pour laisser l'appel se poursuivre.

Les décisions du hook ne contournent pas les règles d'autorisation. Claude Code évalue les règles de refus et de demande indépendamment de ce qu'un hook PreToolUse retourne : une règle de refus correspondante bloque l'appel, et une règle de demande correspondante demande toujours même lorsque le hook a retourné `"allow"` ou `"ask"`. Cela préserve la précédence de refus en premier décrite dans [Gérer les autorisations](#manage-permissions), y compris les règles de refus définies dans les paramètres gérés.

Les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) demandent également toujours une invite lorsqu'un hook retourne `"allow"`.

Un hook de blocage prend également la priorité sur les règles d'autorisation. Un hook qui se termine avec le code 2 arrête l'appel d'outil avant que les règles d'autorisation ne soient évaluées, donc le blocage s'applique même lorsqu'une règle d'autorisation permettrait autrement l'appel. Pour exécuter toutes les commandes Bash sans invites sauf pour quelques-unes que vous voulez bloquer, ajoutez `"Bash"` à votre liste d'autorisation et enregistrez un hook PreToolUse qui rejette ces commandes spécifiques. Consultez [Bloquer les éditions des fichiers protégés](/docs/fr/hooks-guide#block-edits-to-protected-files) pour un script de hook que vous pouvez adapter.

<h2 id="working-directories">
  Répertoires de travail
</h2>

Par défaut, Claude a accès aux fichiers du répertoire où il a été lancé. Vous pouvez étendre cet accès :

* **Au démarrage** : utilisez l'argument CLI `--add-dir <path>`
* **Pendant la session** : utilisez la commande `/add-dir`
* **Configuration persistante** : ajoutez à `additionalDirectories` dans les [fichiers de paramètres](/docs/fr/settings#settings-files)

Les fichiers dans les répertoires supplémentaires suivent les mêmes règles d'autorisation que le répertoire de travail d'origine : ils deviennent lisibles sans invites, et les autorisations d'édition de fichiers suivent le mode d'autorisation actuel.

Dans les sessions en arrière-plan sur macOS, l'hôte de session demande l'accès aux dossiers protégés tels que `~/Desktop`, `~/Documents` et `~/Downloads` séparément de votre terminal lorsque Claude doit lire ou écrire des fichiers là-bas ; si les lectures échouent avec `Operation not permitted`, consultez [comment accorder l'accès aux dossiers aux sessions en arrière-plan](/docs/fr/agent-view#background-sessions-can't-read-desktop-documents-or-downloads-on-macos).

Pour modifier le répertoire de travail principal de la session au lieu d'en ajouter un autre, utilisez [`/cd`](/docs/fr/commands). La commande `/cd` nécessite Claude Code v2.1.169 ou version ultérieure. Contrairement à `/add-dir`, elle relocalise la session : le `CLAUDE.md` du nouveau répertoire est chargé et `--resume` trouve la session à partir de là.

<h3 id="additional-directories-grant-file-access-not-configuration">
  Les répertoires supplémentaires accordent l'accès aux fichiers, pas la configuration
</h3>

L'ajout d'un répertoire étend l'endroit où Claude peut lire et éditer les fichiers. Cela ne fait pas de ce répertoire une racine de configuration complète : la plupart de la configuration `.claude/` n'est pas découverte à partir de répertoires supplémentaires, bien que quelques types soient chargés comme exceptions.

Ces exceptions s'appliquent uniquement aux répertoires ajoutés avec l'indicateur `--add-dir` ou la commande `/add-dir`. Les répertoires listés dans `permissions.additionalDirectories` dans un fichier de paramètres accordent uniquement l'accès aux fichiers et ne chargent aucune des configurations ci-dessous.

Les types de configuration suivants sont chargés à partir des répertoires `--add-dir` :

| Configuration                                                                            | Chargé à partir de `--add-dir`                                                                                                                                        |
| :--------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/fr/skills) dans `.claude/skills/`                                              | Oui, avec rechargement en direct                                                                                                                                      |
| [Subagents](/docs/fr/sub-agents) dans `.claude/agents/`                                       | Oui                                                                                                                                                                   |
| [Paramètres](/docs/fr/settings) dans `.claude/settings.json` et `.claude/settings.local.json` | Clés `enabledPlugins` et `extraKnownMarketplaces` uniquement                                                                                                          |
| Fichiers [CLAUDE.md](/docs/fr/memory), `.claude/rules/` et `CLAUDE.local.md`                  | Uniquement lorsque `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` est défini. `CLAUDE.local.md` nécessite également le paramètre `local`, qui est activé par défaut |

Les commandes et les styles de sortie sont découverts à partir du répertoire de travail actuel et de ses parents, de votre répertoire utilisateur à `~/.claude/` et des paramètres gérés. Les hooks et d'autres clés `settings.json` se chargent à partir du dossier `.claude/` du répertoire de travail actuel sans secours au répertoire parent, aux côtés de votre `~/.claude/settings.json` utilisateur et des paramètres gérés. Pour partager cette configuration entre les projets, utilisez l'une de ces approches :

* **Configuration au niveau utilisateur** : placez les fichiers dans `~/.claude/agents/`, `~/.claude/output-styles/` ou `~/.claude/settings.json` pour les rendre disponibles dans chaque projet
* **Plugins** : empaquetez et distribuez la configuration en tant que [plugin](/docs/fr/plugins) que les équipes peuvent installer
* **Lancer à partir du répertoire de configuration** : exécutez Claude Code à partir du répertoire contenant la configuration `.claude/` que vous souhaitez

<h2 id="how-permissions-interact-with-sandboxing">
  Comment les autorisations interagissent avec le sandboxing
</h2>

Les autorisations et le [sandboxing](/docs/fr/sandboxing) sont des couches de sécurité complémentaires :

* **Les autorisations** contrôlent quels outils Claude Code peut utiliser et quels fichiers ou domaines il peut accéder. Elles s'appliquent à tous les outils, notamment Bash, Read, Edit, WebFetch et MCP.
* **Le sandboxing** fournit une application au niveau du système d'exploitation qui restreint l'accès du système de fichiers et du réseau de l'outil Bash. Il s'applique uniquement aux commandes Bash et à leurs processus enfants.

Utilisez les deux pour une défense en profondeur :

* Les règles de refus d'autorisation empêchent Claude d'essayer même d'accéder aux ressources restreintes
* Les restrictions de sandbox empêchent les commandes Bash d'atteindre les ressources en dehors des limites définies, même si une injection de prompt contourne la prise de décision de Claude
* Les restrictions du système de fichiers dans le sandbox combinent les paramètres [`sandbox.filesystem`](/docs/fr/sandboxing) avec les règles de refus Read et Edit ; les deux sont fusionnées dans la limite finale du sandbox
* Les restrictions réseau combinent les règles d'autorisation WebFetch avec les listes `allowedDomains` et `deniedDomains` du sandbox

Lorsque le sandboxing est activé avec `autoAllowBashIfSandboxed: true`, qui est la valeur par défaut, les commandes Bash en sandbox s'exécutent sans invite même si vos autorisations incluent une règle ask `Bash` simple, ou la [forme équivalente `Bash(*)`](#match-all-uses-of-a-tool) : la limite du sandbox remplace cette invite pour l'outil entier. Ces vérifications s'appliquent toujours :

* Les règles ask limitées au contenu comme `Bash(git push *)` forcent toujours une invite
* Les règles de refus explicites s'appliquent toujours
* Les commandes `rm` ou `rmdir` qui ciblent `/`, votre répertoire personnel ou d'autres chemins système critiques déclenchent toujours une invite

Les commandes qui ne s'exécutent pas en sandbox, comme les commandes exclues, respectent la règle ask `Bash` simple comme d'habitude. Consultez [modes sandbox](/docs/fr/sandboxing#sandbox-modes) pour modifier ce comportement.

<h2 id="managed-settings">
  Paramètres gérés
</h2>

Pour les organisations qui ont besoin d'un contrôle centralisé sur la configuration de Claude Code, les administrateurs peuvent déployer des paramètres gérés qui ne peuvent pas être remplacés par les paramètres utilisateur ou projet. Ces paramètres de politique suivent le même format que les fichiers de paramètres réguliers et peuvent être livrés via des politiques MDM/au niveau du système d'exploitation, des fichiers de paramètres gérés, des [paramètres gérés par le serveur](/docs/fr/server-managed-settings) ou une [passerelle Claude apps](/docs/fr/claude-apps-gateway) auto-hébergée. Consultez [fichiers de paramètres](/docs/fr/settings#settings-files) pour les mécanismes de livraison et les emplacements de fichiers.

<h3 id="managed-only-settings">
  Paramètres gérés uniquement
</h3>

Les paramètres suivants ne sont efficaces que dans les paramètres gérés. Les placer dans les fichiers de paramètres utilisateur ou projet n'a aucun effet.

| Paramètre                                      | Description                                                                                                                                                                                                                                                                                                                                                                      |
| :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowAllClaudeAiMcps`                         | Lorsque `true`, les connecteurs claude.ai se chargent aux côtés d'un `managed-mcp.json` déployé au lieu d'être supprimés par son contrôle exclusif. Consultez [Configuration MCP gérée](/docs/fr/managed-mcp)                                                                                                                                                                         |
| `allowedChannelPlugins`                        | Liste blanche des plugins de canal qui peuvent envoyer des messages. Remplace la liste blanche Anthropic par défaut lorsqu'elle est définie. Nécessite `channelsEnabled: true`. Consultez [Restreindre les plugins de canal qui peuvent s'exécuter](/docs/fr/channels#restrict-which-channel-plugins-can-run)                                                                         |
| `allowManagedHooksOnly`                        | Lorsque `true`, seuls les hooks gérés, les hooks SDK et les hooks des plugins activés de force dans les paramètres gérés `enabledPlugins` sont chargés. Les hooks utilisateur, projet et tous les autres hooks de plugin sont bloqués                                                                                                                                            |
| `allowManagedMcpServersOnly`                   | Lorsque `true`, seuls les `allowedMcpServers` des paramètres gérés sont respectés. `deniedMcpServers` fusionne toujours à partir de toutes les sources. Consultez [Configuration MCP gérée](/docs/fr/managed-mcp)                                                                                                                                                                     |
| `allowManagedPermissionRulesOnly`              | Lorsque `true`, empêche les paramètres utilisateur et projet de définir les règles d'autorisation `allow`, `ask` ou `deny`. Seules les règles dans les paramètres gérés s'appliquent. N'affecte pas la liste blanche du serveur MCP ; pour cela, définissez `allowManagedMcpServersOnly`                                                                                         |
| `blockedMarketplaces`                          | Liste noire des sources de marketplace. Les sources bloquées sont vérifiées avant le téléchargement, elles ne touchent donc jamais le système de fichiers. Consultez [restrictions de marketplace gérées](/docs/fr/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                              |
| `channelsEnabled`                              | Autoriser les [canaux](/docs/fr/channels) pour l'organisation. Consultez [contrôles d'entreprise](/docs/fr/channels#enterprise-controls) pour la valeur par défaut sur chaque plan                                                                                                                                                                                                         |
| `disableSideloadFlags`                         | Rejeter les drapeaux CLI `--plugin-dir`, `--plugin-url`, `--agents` et `--mcp-config` au démarrage. Sans cela, les utilisateurs peuvent contourner `strictKnownMarketplaces` pour une seule exécution en passant ces drapeaux. Consultez [`disableSideloadFlags`](/docs/fr/settings#available-settings). Nécessite Claude Code v2.1.193 ou version ultérieure                         |
| `forceRemoteSettingsRefresh`                   | Lorsque `true`, bloque le démarrage de la CLI jusqu'à ce que les paramètres gérés distants soient fraîchement récupérés et se termine si la récupération échoue. Consultez [application fail-closed](/docs/fr/server-managed-settings#enforce-fail-closed-startup)                                                                                                                    |
| `pluginTrustMessage`                           | Message personnalisé ajouté à l'avertissement de confiance du plugin affiché avant l'installation                                                                                                                                                                                                                                                                                |
| `sandbox.filesystem.allowManagedReadPathsOnly` | Lorsque `true`, seuls les chemins `filesystem.allowRead` des paramètres gérés sont respectés. `denyRead` fusionne toujours à partir de toutes les sources                                                                                                                                                                                                                        |
| `sandbox.network.allowManagedDomainsOnly`      | Lorsque `true`, seuls les `allowedDomains` et les règles d'autorisation `WebFetch(domain:...)` des paramètres gérés sont respectés. Les domaines non autorisés sont bloqués automatiquement sans inviter l'utilisateur. Les domaines refusés fusionnent toujours à partir de toutes les sources                                                                                  |
| `strictKnownMarketplaces`                      | Contrôle quels marketplaces de plugins les utilisateurs peuvent ajouter et installer des plugins à partir de. Consultez [restrictions de marketplace gérées](/docs/fr/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                                                                           |
| `strictPluginOnlyCustomization`                | Bloquer les skills, agents, hooks et serveurs MCP provenant de sources utilisateur et projet, afin qu'ils ne puissent provenir que de plugins ou de paramètres gérés. `true` verrouille les quatre surfaces ; un tableau tel que `["skills", "hooks"]` verrouille uniquement les nommés. Consultez [`strictPluginOnlyCustomization`](/docs/fr/settings#strictpluginonlycustomization) |
| `wslInheritsWindowsSettings`                   | Lorsque `true` dans la clé de registre Windows HKLM ou `C:\Program Files\ClaudeCode\managed-settings.json`, WSL lit les paramètres gérés à partir de la chaîne de politique Windows en plus de `/etc/claude-code`. Consultez [Fichiers de paramètres](/docs/fr/settings#settings-files)                                                                                               |

`disableBypassPermissionsMode` est généralement placé dans les paramètres gérés pour appliquer la politique organisationnelle, mais il fonctionne à partir de n'importe quelle portée. Un utilisateur peut le définir dans ses propres paramètres pour se verrouiller hors du mode de contournement.

<Note>
  Sur les plans Team et Enterprise, un propriétaire active ou désactive [Remote Control](/docs/fr/remote-control) et les [sessions web](/docs/fr/claude-code-on-the-web) à l'échelle de l'organisation dans les [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code). Remote Control peut également être désactivé par appareil avec le paramètre [`disableRemoteControl`](/docs/fr/settings#available-settings). Les sessions web n'ont pas de clé de paramètres gérés par appareil.
</Note>

<h2 id="settings-precedence">
  Précédence des paramètres
</h2>

Les règles d'autorisation suivent la même [précédence des paramètres](/docs/fr/settings#settings-precedence) que tous les autres paramètres de Claude Code :

1. **Paramètres gérés** : ne peuvent pas être remplacés par aucun autre niveau, y compris les arguments de ligne de commande
2. **Arguments de ligne de commande** : remplacements de session temporaires
3. **Paramètres de projet local** (`.claude/settings.local.json`)
4. **Paramètres de projet partagés** (`.claude/settings.json`)
5. **Paramètres utilisateur** (`~/.claude/settings.json`)

Si un outil est refusé à n'importe quel niveau, aucun autre niveau ne peut l'autoriser. Par exemple, un refus de paramètres gérés ne peut pas être remplacé par `--allowedTools`, et `--disallowedTools` peut ajouter des restrictions au-delà de ce que les paramètres gérés définissent.

Les mêmes règles s'appliquent dans les différentes portées de paramètres : si les paramètres utilisateur autorisent une autorisation et que les paramètres de projet la refusent, la règle de refus la bloque. L'inverse est également vrai : un refus au niveau utilisateur bloque une autorisation au niveau du projet, car les règles de refus de n'importe quelle portée sont évaluées avant les règles d'autorisation.

Les hôtes d'intégration peuvent fournir une politique gérée supplémentaire via l'option SDK `managedSettings` lorsque [`parentSettingsBehavior`](/docs/fr/settings#settings-precedence) est défini sur `"merge"` ; les valeurs de l'intégrateur peuvent renforcer la politique mais pas l'assouplir.

<h2 id="project-allow-rules-and-workspace-trust">
  Règles d'autorisation de projet et confiance de l'espace de travail
</h2>

Les règles `permissions.allow` et les entrées `permissions.additionalDirectories` dans le fichier `.claude/settings.json` d'un projet accordent des capacités, donc Claude Code les applique uniquement après que vous acceptiez la [boîte de dialogue de confiance de l'espace de travail](/docs/fr/security#additional-safeguards) pour cet espace de travail. Jusqu'à ce moment, Claude Code lit les règles mais ne les applique pas. La boîte de dialogue de confiance répertorie les règles d'autorisation et les répertoires supplémentaires que le dossier accorderait afin que vous puissiez les examiner avant d'accepter. Les règles `deny` et `ask` ne sont pas affectées, car elles ne font que restreindre.

Claude Code enregistre la confiance par espace de travail, en utilisant comme clé la racine du référentiel git ou, en dehors d'un référentiel, le répertoire à partir duquel vous avez démarré Claude Code. Lorsque vous démarrez dans votre répertoire personnel, la confiance est conservée pour la session actuelle uniquement et n'est pas écrite sur le disque ; consultez la note sur les [protections supplémentaires](/docs/fr/security#additional-safeguards). Faire confiance à un répertoire parent n'applique pas les règles d'autorisation d'un projet imbriqué.

`.claude/settings.local.json` est votre propre fichier, donc la vérification de confiance de l'espace de travail ne s'applique généralement pas à celui-ci. Lorsqu'un référentiel aurait pu fournir le fichier, par exemple lorsqu'il est validé dans git ou que `.claude` est un lien symbolique, ses règles d'autorisation et répertoires supplémentaires passent par la vérification de confiance comme les paramètres du projet.

Claude Code exécute git pour vérifier si le référentiel a fourni le fichier, et il n'exécute cette vérification que dans un dossier couvert par une boîte de dialogue de confiance acceptée, pour ce dossier ou pour l'un de ses répertoires parents. Dans une session interactive dans un dossier que vous n'avez pas encore approuvé, les règles d'autorisation et les répertoires supplémentaires dans `.claude/settings.local.json` passent par la vérification de confiance comme les paramètres du projet jusqu'à ce que vous acceptiez la boîte de dialogue, sauf si la session s'exécute dans votre répertoire de configuration personnel comme décrit ci-dessous. Des deux exceptions ci-dessous, seule l'exception du répertoire de configuration s'applique avant la boîte de dialogue, car elle n'a pas besoin d'exécuter git. Déterminer qu'un répertoire ne se trouve pas à l'intérieur d'un référentiel git utilise la même vérification git, donc l'exception de non-présence dans un référentiel prend effet une fois qu'une boîte de dialogue de confiance couvrant le dossier est acceptée. Avant la v2.1.207, un `.claude/settings.local.json` non suivi appliquait ses règles d'autorisation dans ce dossier avant que vous acceptiez la boîte de dialogue.

Les règles d'autorisation et les répertoires supplémentaires dans `.claude/settings.local.json` s'appliquent également sans confiance de l'espace de travail dans deux cas :

* Le répertoire à partir duquel vous avez démarré Claude Code ne se trouve pas à l'intérieur d'un référentiel git.
* La session s'exécute dans votre répertoire de configuration personnel : votre répertoire personnel ou tout répertoire dont vous avez défini le sous-répertoire `.claude` comme [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars).

Dans les deux cas, le fichier est un fichier que vous avez créé plutôt qu'un fichier qu'un référentiel aurait pu fournir, et un `.claude/settings.local.json` validé dans le référentiel nécessite toujours la confiance de l'espace de travail. Les versions 2.1.196 à 2.1.199 traitaient le fichier comme fourni par le référentiel dans ces espaces de travail, ignoraient ses règles d'autorisation et imprimaient un avertissement [`this workspace has not been trusted`](/docs/fr/errors#workspace-has-not-been-trusted) sur stderr. Les deux exceptions ci-dessus correspondent à la v2.1.195 et antérieures et ont été restaurées dans la v2.1.200.

Également à partir de la v2.1.200, un espace de travail dont les règles d'autorisation ou les répertoires supplémentaires ne sont toujours pas appliqués, mais qui n'a jamais montré la boîte de dialogue de confiance parce qu'un répertoire parent était déjà approuvé, affiche la boîte de dialogue la prochaine fois que vous démarrez Claude Code là-bas de manière interactive. La boîte de dialogue offre deux choix :

* **Oui, j'approuve ce dossier** : enregistre la confiance pour cet espace de travail et applique les règles dans la même session.
* **Non, continuer sans ces permissions** : continue de fonctionner avec ces règles ignorées. La boîte de dialogue réapparaît dans la session suivante.

En [mode non interactif](/docs/fr/headless) avec `-p`, aucune boîte de dialogue n'apparaît et les règles restent ignorées.

<h2 id="example-configurations">
  Exemples de configurations
</h2>

Ce [référentiel](https://github.com/anthropics/claude-code/tree/main/examples/settings) inclut des configurations de paramètres de démarrage pour les scénarios de déploiement courants. Utilisez-les comme points de départ et ajustez-les pour répondre à vos besoins.

<h2 id="see-also">
  Voir aussi
</h2>

* [Paramètres](/docs/fr/settings) : référence de configuration complète incluant le tableau des paramètres d'autorisation
* [Configurer le mode auto](/docs/fr/auto-mode-config) : dites au classificateur du mode auto quelle infrastructure votre organisation approuve
* [Sandboxing](/docs/fr/sandboxing) : isolation du système de fichiers et du réseau au niveau du système d'exploitation pour les commandes Bash
* [Authentification](/docs/fr/authentication) : configurer l'accès utilisateur à Claude Code
* [Sécurité](/docs/fr/security) : garanties de sécurité et meilleures pratiques
* [Hooks](/docs/fr/hooks-guide) : automatiser les flux de travail et étendre l'évaluation des autorisations
