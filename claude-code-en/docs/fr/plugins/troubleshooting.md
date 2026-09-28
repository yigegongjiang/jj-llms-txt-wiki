> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dépanner les plugins

> Corrigez les erreurs de plugins dans Claude Code. Trouvez le message exact que vous avez vu, regroupé par étape depuis l'exécution de /plugin jusqu'à l'installation et la politique organisationnelle.

Cette page répertorie les messages d'erreur et les symptômes des plugins Claude Code et des marketplaces, les catalogues à partir desquels Claude Code installe les plugins. Chaque entrée indique la cause, une solution et ce que vous voyez une fois la correction appliquée.

Lorsqu'un message nomme un plugin ou une marketplace, l'entrée affiche un espace réservé tel que `<name>` à la place.

Utilisez cette page que vous installiez des plugins, les construisiez, hébergiez une marketplace ou administriez des plugins pour une organisation.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Pourquoi les portées, le cache et la précédence se comportent de cette façon** : lisez [Référence du chargement des plugins](/docs/fr/plugins/loading)
  * **Recherche d'un drapeau, d'un champ ou d'une commande** : utilisez la [référence des commandes de plugin](/docs/fr/plugins/cli-reference), la [référence du manifeste](/docs/fr/plugins/manifest-reference) ou la [référence de la marketplace](/docs/fr/plugins/marketplace-reference)
</Note>

Recherchez le message exact que vous avez vu. Chaque message est répertorié sous l'étape qui le produit, ce qui n'est pas toujours la commande que vous avez exécutée. Par exemple, une installation peut échouer parce qu'une marketplace est manquante, donc ce message se trouve sous [Ajouter une marketplace](#add-a-marketplace).

<h2 id="find-where-/plugin-runs">
  Trouvez où `/plugin` s'exécute
</h2>

`/plugin` est une commande que vous tapez dans une session de terminal Claude Code en cours d'exécution, et elle ouvre un panneau interactif. Les entrées de cette section couvrent les endroits où vous pouvez la taper mais elle ne peut pas s'exécuter, et les orthographes de commande qui n'existent pas.

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

Vous avez tapé `/plugin` quelque part en dehors d'une session de terminal Claude Code, et Claude a répondu avec cette ligne au lieu d'ouvrir quoi que ce soit.

Vous recevez cette réponse dans une session qui n'a pas de terminal pour dessiner le panneau `/plugin` : [mode non interactif](/docs/fr/headless) avec `claude -p`, le SDK Agent, l'onglet Code de l'application de bureau Claude, le panneau de l'extension VS Code et le navigateur à claude.ai/code.

Dans le panneau de l'extension VS Code, seule une ligne `/plugin` avec quelque chose après, comme `/plugin install <plugin>@<marketplace>`, reçoit cette réponse. `/plugin` ou `/plugins` tapé seul ouvre la boîte de dialogue **Gérer les plugins**.

Installez le plugin à partir de la surface sur laquelle vous vous trouvez à la place :

* **Application de bureau Claude, session locale ou SSH** : cliquez sur le bouton **+** à côté de l'invite, puis **Plugins**, puis **Ajouter un plugin** pour ouvrir le [navigateur de plugins](/docs/fr/desktop#install-plugins)
* **Extension VS Code** : utilisez l'onglet **VS Code** sous [Installer un plugin](/docs/fr/plugins/install#install-a-plugin)
* **Claude Code sur le web, ou une session cloud de bureau** : une session cloud n'a pas de navigateur de plugins. Consultez l'onglet **Session cloud** sous [Installer un plugin](/docs/fr/plugins/install#install-a-plugin) pour voir ce qu'une session cloud charge
* **Un terminal auquel vous avez accès** : exécutez `claude` et tapez `/plugin` là, ou exécutez `claude plugin install <plugin>@<marketplace>` dans votre shell sans démarrer une session

Lorsqu'une installation de terminal fonctionne, `/plugin` imprime un résumé d'installation qui commence par `✓ Installed <plugin>.` et `claude plugin install` imprime `Successfully installed plugin: <plugin>@<marketplace>`.

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

Vous avez tapé `/plugin ...` à une invite de shell, et le shell a signalé qu'aucun fichier nommé `/plugin` n'existe. Bash signale `bash: /plugin: No such file or directory`.

`/plugin` est une commande que vous tapez dans une session Claude Code, pas à l'invite du shell. Démarrez une session et tapez la même commande là :

```shell theme={null}
claude
```

Ensuite, à l'invite Claude Code :

```text theme={null}
/plugin install <plugin>@<marketplace>
```

Une installation réussie imprime un résumé qui commence par `✓ Installed <plugin>.` Si l'installation elle-même échoue ensuite, son message se trouve sous [Ajouter une marketplace](#add-a-marketplace) ou [Installer un plugin](#install-a-plugin).

Pour installer à partir du shell sans démarrer une session, exécutez `claude plugin install <plugin>@<marketplace>` à la place.

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

Vous avez tapé `/plugin ...` à une invite PowerShell, et `/plugin` est une commande Claude Code, pas un programme. Bash et Zsh signalent [leur propre forme de cette erreur](#zsh-no-such-file-or-directory-plugin).

Utilisez plutôt l'une de ces options :

* Exécutez `claude`, puis tapez `/plugin` à l'invite Claude Code
* Exécutez `claude plugin install <plugin>@<marketplace>` dans PowerShell sans démarrer une session

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` après `claude plugin ...`
</h3>

Vous avez exécuté `claude plugin install ...` dans votre shell, et le shell n'a pas pu trouver `claude` du tout. Sur Windows, le message est `'claude' is not recognized as the name of a cmdlet` ou `'claude' is not recognized as an internal or external command`.

La cause n'est pas la commande de plugin. Soit Claude Code n'est pas installé, soit son répertoire d'installation n'est pas sur votre `PATH` dans ce shell. Suivez [`command not found: claude` après l'installation](/docs/fr/troubleshoot-install#command-not-found-claude-after-installation), puis réessayez la commande de plugin.

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` et orthographes de commande qui n'existent pas
</h3>

Vous avez tapé une commande de plugin que vous avez vue quelque part et avez obtenu `Unknown command: /<name>` dans une session, ou `error: unknown command '<name>'` ou `error: unknown option '<flag>'` du binaire `claude` dans votre shell.

Plusieurs orthographes de commande sont en usage que Claude Code n'a pas. Le tableau ci-dessous mappe chacune à la vraie commande. La [référence des commandes de plugin](/docs/fr/plugins/cli-reference) répertorie chaque sous-commande et drapeau.

| Vous avez tapé                             | Ce que Claude Code dit                                                       | Utilisez plutôt                                                                                                                                   |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | `claude plugin marketplace add <source>` pour ajouter une marketplace, ou `claude plugin install <plugin>@<marketplace>` pour installer un plugin |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                                                    |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                                          |
| `/plugin add <source>`                     | Le panneau `/plugin` s'ouvre sur l'onglet **Discover**                       | `/plugin marketplace add <source>`                                                                                                                |
| `marketplace.anthropic.com` comme source   | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` pour la marketplace officielle                                                                               |

Ces orthographes semblent incorrectes mais fonctionnent :

* `claude plugins` est un alias de `claude plugin`
* `claude plugin remove` est un alias de `claude plugin uninstall`
* `/plugins` et `/marketplace` dans une session ouvrent le même panneau que `/plugin`

<h2 id="add-a-marketplace">
  Ajouter une marketplace
</h2>

Une marketplace est un catalogue que vous ajoutez à Claude Code à partir d'un référentiel git, d'une URL ou d'un chemin local. Ces entrées couvrent les messages que vous recevez lorsque l'ajout échoue ou qu'une actualisation ultérieure échoue.

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

Vous avez exécuté `/plugin install <plugin>@claude-plugins-official` dans une session, et Claude Code a signalé qu'il n'a pas de marketplace portant ce nom.

La marketplace officielle n'est pas encore enregistrée sur cette machine. Claude Code l'enregistre normalement de lui-même la première fois que vous démarrez une session de terminal interactif. Elle n'a pas encore fonctionné si vous n'avez utilisé Claude Code que via l'extension VS Code, et elle saute ou reporte cette étape :

* Lorsqu'une politique bloque la source
* Lorsque `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` est défini
* Après une tentative échouée qui attend une nouvelle tentative

Les commandes shell `claude plugin` ne l'enregistrent jamais pour vous.

Ajoutez-la, puis réessayez l'installation :

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: claude-plugins-official`, et `/plugin marketplace list` affiche la marketplace avec sa source.

Pour tout autre nom de marketplace dans ce message, consultez [`Marketplace "<name>" not found`](#marketplace-not-found).

La même chaîne apparaît également dans l'onglet **Erreurs** de `/plugin`, la liste des échecs de chargement du panneau, lorsqu'un plugin répertorié dans vos paramètres nomme une marketplace que vous n'avez pas ajoutée.

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

Vous avez exécuté `/plugin install <plugin>@<name>` dans une session, souvent à partir d'une ligne d'installation que quelqu'un vous a envoyée, et Claude Code a signalé qu'il n'a pas de marketplace portant ce nom.

Si le nom commence par `claudeai-`, la marketplace est hébergée sur claude.ai, et vous l'ajoutez par nom à partir de votre shell avec `claude plugin marketplace add --claudeai <name>`. Consultez [Ajouter une marketplace à partir de claude.ai](/docs/fr/plugins/install#add-from-claude-ai).

Pour tout autre nom, une ligne d'installation nomme une marketplace mais ne dit pas où la marketplace est hébergée, et Claude Code n'a pas d'index pour rechercher un nom de marketplace. Demandez à la personne qui a envoyé la ligne la source de la marketplace, qui est un `owner/repo` GitHub, une URL git ou un chemin. Ensuite, [ajoutez la marketplace](/docs/fr/plugins/install#add-a-marketplace) et exécutez à nouveau la ligne d'installation.

Une marketplace que quelqu'un vous envoie est tierce, donc [examinez le plugin avant de l'installer](/docs/fr/plugins/security#review-a-plugin-before-you-install).

Si vous avez déjà ajouté la marketplace, vérifiez l'orthographe par rapport à `/plugin marketplace list`.

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

Vous avez exécuté `/plugin marketplace add <source>` ou `claude plugin marketplace add <source>`, et Claude Code a répondu `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`.

Claude Code accepte une source dans l'une de ces formes :

* Un raccourci GitHub `owner/repo`
* Une URL `https://` ou `http://`
* Une URL SSH `user@host:path`
* Un chemin local commençant par `./`, `../`, `/` ou `~`

Un nom nu tel que `claude-plugins-official` ne correspond à aucun d'eux. Pas plus qu'un nom d'hôte nu tel que `marketplace.anthropic.com`.

Retapez la source dans l'une des formes acceptées :

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: <name>` lorsque l'ajout fonctionne.

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

Vous avez passé une source avec une barre oblique qui n'est pas `owner/repo`, comme `github.com/owner/repo` ou un chemin `gitlab.example.com/group/project`. Claude Code l'a refusée avec ce message et une liste de formes acceptées.

Le raccourci `owner/repo` est spécifique à GitHub et doit suivre les règles de nommage de GitHub, donc un nom d'hôte ou un segment de chemin supplémentaire échoue. Passez la source dans la forme qui correspond à l'endroit où la marketplace est hébergée :

* **Un référentiel sur n'importe quel hôte** : l'URL de clonage complète
* **Un `marketplace.json` hébergé** : son URL `https://`
* **Un checkout local** : `./path` ou un chemin absolu

Par exemple, pour ajouter la marketplace officielle par son URL de clonage, dans une session :

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

Un ajout réussi imprime `Successfully added marketplace: <name>`.

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

Vous avez passé un chemin local à `marketplace add`, et rien n'existe à ce chemin. Un chemin relatif se résout par rapport à votre répertoire courant.

Vérifiez le chemin résolu dans le message. Ensuite, exécutez la commande à partir du répertoire à partir duquel le chemin relatif commence, ou passez un chemin absolu au répertoire de la marketplace. Un ajout réussi imprime `Successfully added marketplace: <name>`.

Claude Code accepte un répertoire qui contient `.claude-plugin/marketplace.json`, ou un chemin vers un fichier `.json`. Un chemin vers tout autre fichier échoue avec `File path must point to a .json file (marketplace.json)`.

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code a cloné ou téléchargé la marketplace mais n'a trouvé aucun `marketplace.json` au chemin attendu à l'intérieur. La commande d'ajout la signale comme `Failed to add marketplace: Marketplace file not found at ...`.

L'emplacement par défaut est `.claude-plugin/marketplace.json` à la racine du référentiel, et la [référence de la marketplace](/docs/fr/plugins/marketplace-reference) répertorie les emplacements acceptés.

La correction diffère pour le propriétaire et pour tout le monde d'autre :

* **Vous possédez la marketplace** : mettez le fichier à cet emplacement et réajoutez la marketplace
* **Quelqu'un d'autre l'héberge** : demandez au propriétaire la source exacte qu'il publie

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` ou `HTTPS authentication failed`
</h3>

Vous avez ajouté ou mis à jour une marketplace à partir d'un référentiel git, et le clonage a échoué avec `Failed to clone marketplace repository:` suivi de l'une de ces lignes.

Vérifiez d'abord le référentiel lui-même : un `owner/repo` mal orthographié, un référentiel qui n'existe pas ou un référentiel privé que vous ne pouvez pas voir se termine également par ce message. Ouvrez l'URL du référentiel dans votre navigateur, ou exécutez `git ls-remote <url>` dans votre terminal, pour confirmer qu'il existe et que vous y avez accès.

Si le référentiel est correct, la cause est les identifiants. Claude Code exécute git avec les invites interactives désactivées, donc il ne peut pas vous demander un mot de passe, une phrase de passe de clé ou un identifiant de la façon dont votre terminal le ferait. Si git a besoin d'une invite, vous voyez `fatal: Cannot prompt because user interactivity has been disabled` ou `terminal prompts disabled` dans l'erreur d'origine. Seuls les identifiants qui fonctionnent déjà de manière non interactive réussissent :

* **SSH** : `ssh -T git@<host>` doit réussir sans demander de phrase de passe, et l'hôte doit déjà être dans `known_hosts`
* **HTTPS** : votre assistant d'identifiants doit contenir un jeton pour l'hôte. Pour GitHub, exécutez `gh auth login` et `gh auth setup-git`. Pour un autre hôte, stockez un jeton d'accès personnel dans votre assistant d'identifiants git. Testez avec `git ls-remote <url>`

Une fois que `git ls-remote` réussit dans votre terminal sans invite, exécutez à nouveau l'ajout ou la mise à jour. Un ajout réussi imprime `Successfully added marketplace: <name>`. Une mise à jour réussie imprime `Successfully updated marketplace: <name>` à partir de votre shell, ou `✔ Updated 1 marketplace` dans une session.

Pour faire en sorte que Claude Code ignore SSH pour les sources GitHub `owner/repo`, définissez `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`. Sans cela, Claude Code clone ces sources sur SSH lorsqu'une clé SSH pour `github.com` semble configurée, et revient à HTTPS lorsque le clonage SSH échoue.

Pour ce que la mise à jour automatique en arrière-plan peut et ne peut pas faire avec vos identifiants, consultez [Ce que la mise à jour automatique en arrière-plan fait avec les identifiants](/docs/fr/plugins/host-marketplace#what-background-auto-update-does-with-credentials).

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

Vous avez ajouté une marketplace sur SSH à partir d'un hôte auquel vous ne vous êtes jamais connecté, et le clonage a échoué avec cette ligne et un conseil `ssh -T git@<host>`. Pour un hôte dont la clé a changé, le message est `SSH host key has changed` avec un conseil `ssh-keygen -R <host>` à la place.

Claude Code clone avec `StrictHostKeyChecking=yes`, donc il refuse un hôte dont vous n'avez pas encore accepté la clé plutôt que d'accepter la clé automatiquement. Connectez-vous une fois à partir de votre terminal pour accepter l'empreinte digitale, puis réessayez :

```shell theme={null}
ssh -T git@github.com
```

Pour un référentiel public, ajoutez la marketplace par son URL `https://` à la place pour éviter complètement SSH.

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

Sur Windows, vous avez ajouté une marketplace et Claude Code a signalé `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)`.

Claude Code recherche `git` sur votre `PATH` et refuse d'en exécuter un trouvé uniquement dans le répertoire courant. Pour corriger cela, installez Git et réessayez :

<Steps>
  <Step title="Installer Git pour Windows">
    Installez Git pour Windows afin que `git` soit sur votre `PATH`.
  </Step>

  <Step title="Ouvrir un nouveau terminal">
    Ouvrez un nouveau terminal pour que le `PATH` mis à jour s'applique.
  </Step>

  <Step title="Confirmer que git s'exécute">
    Confirmez que `git --version` imprime une version.
  </Step>

  <Step title="Réessayer l'ajout">
    Exécutez à nouveau la commande `marketplace add`.
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

Vous avez ajouté ou mis à jour une marketplace, et elle a échoué avec `Git clone timed out after 120s`, suivi d'un conseil pour définir `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`.

Le clonage d'une marketplace et le re-clonage d'une pour la mettre à jour obtiennent 120 secondes par défaut. Pour un grand référentiel ou une connexion lente, augmentez la limite. La valeur est en millisecondes :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

Ensuite, réessayez dans le même shell.

Si le référentiel est un monorepo, limitez le checkout aux répertoires que vous nommez avec `claude plugin marketplace add <source> --sparse <paths>`.

<h3 id="marketplace-updates-keep-failing-offline">
  Les mises à jour de la marketplace continuent d'échouer hors ligne
</h3>

Vous travaillez dans un environnement où l'hôte git de la marketplace est inaccessible, et chaque session répète un échec d'actualisation en arrière-plan. Votre checkout existant de la marketplace reste en place et le démarrage n'est pas retardé.

Chaque session, pour une marketplace avec [mise à jour automatique activée](/docs/fr/plugins/loading#which-marketplaces-and-plugins-auto-update), Claude Code vérifie l'hôte git de la marketplace pour les nouveaux commits en arrière-plan. Lorsque cette vérification ne peut pas atteindre l'hôte, elle essaie de cloner à nouveau la marketplace, et hors ligne ce clonage échoue aussi.

Définissez cette variable pour ignorer la tentative de re-clonage et continuer à utiliser le checkout existant lorsque la vérification ne peut pas atteindre l'hôte :

<Tabs>
  <Tab title="Bash ou Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

Avec la variable définie, Claude Code ignore le re-clonage uniquement pour un checkout qui contient déjà `.claude-plugin/marketplace.json`. Une marketplace qui n'a jamais été clonée ou dont le clonage s'est arrêté à mi-chemin obtient toujours la tentative de clonage, donc ajoutez-la une fois en ligne.

Pour un déploiement entièrement hors ligne, pré-remplissez plutôt le répertoire des plugins au moment de la construction de l'image avec `CLAUDE_CODE_PLUGIN_SEED_DIR`, en suivant [Ensemencer les conteneurs et CI](/docs/fr/plugins/org#seed-containers-and-ci).

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  L'ajout de marketplace échoue sur un hôte GitHub Enterprise Server
</h3>

Vous avez ajouté une marketplace à partir d'une URL GitHub Enterprise Server (GHES) et avez obtenu une erreur de politique, ou vous l'avez ajoutée à partir de claude.ai et avez obtenu une erreur d'accès GitHub.

Les deux cas se trouvent sur la page GHES :

* [Une erreur de politique](/docs/fr/github-enterprise-server#marketplace-add-fails-with-a-policy-error) signifie que votre organisation a restreint les sources de marketplace et un administrateur doit ajouter un `hostPattern` pour l'hôte
* [Une erreur d'accès GitHub sur claude.ai](/docs/fr/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error) signifie que votre propre compte GitHub Enterprise n'est pas encore connecté

<h2 id="install-a-plugin">
  Installer un plugin
</h2>

Vous avez ajouté une marketplace et exécuté une installation, et l'installation s'est arrêtée avec un message au lieu d'installer quoi que ce soit. Ces entrées couvrent ces messages. Elles couvrent également les messages connexes qui apparaissent plus tard dans l'onglet **Erreurs** de `/plugin`, ou comme un onglet **Discover** vide, lorsqu'un plugin ou sa marketplace ne peut pas être trouvé, lu ou approuvé.

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

Vous avez exécuté `/plugin install <name>@<marketplace>` ou `claude plugin install <name>@<marketplace>`, et le nom du plugin ne figure pas dans la copie du catalogue de cette marketplace sur votre machine.

`claude plugin install` dans votre shell imprime le même message lorsque vous n'avez pas du tout ajouté la marketplace. Si `claude plugin marketplace update <marketplace>` répond ensuite `Marketplace '<marketplace>' not found`, [ajoutez d'abord la marketplace](#add-a-marketplace).

<h4 id="the-message-ends-with-a-refresh-hint">
  `not found in marketplace` avec un conseil d'actualisation
</h4>

Le conseil se lit comme suit : `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` ou `The marketplace couldn't be refreshed (...)`. Claude Code n'a pas actualisé la marketplace avant la recherche, par exemple lorsque vous êtes hors ligne, donc votre copie du catalogue peut être obsolète. Actualisez avec le nom de la marketplace, puis installez à nouveau :

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` imprime `Successfully updated marketplace: <name>`, et `/plugin marketplace update` affiche `✔ Updated 1 marketplace`. Si l'installation réessayée imprime le même message, vérifiez le nom comme [`not found in marketplace` sans conseil](#the-message-has-no-hint) le décrit. [Quand Claude Code actualise une marketplace avant une installation](/docs/fr/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install) répertorie les autres cas où l'actualisation ne s'exécute pas.

<h4 id="the-message-has-no-hint">
  `not found in marketplace` sans conseil
</h4>

Le nom est le problème le plus probable. Ouvrez `/plugin`, allez à **Discover**, et copiez le nom de la liste.

Avant v2.1.232, Claude Code actualisait la marketplace nommée uniquement après que la recherche ait échoué, et uniquement lorsque la mise à jour automatique était activée pour elle.

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

Vous avez exécuté `/plugin install <name>` sans `@marketplace`, et aucune marketplace enregistrée n'a ce plugin. `claude plugin install <name>` signale `Plugin "<name>" not found in any configured marketplace`.

Sans nom de marketplace, `claude plugin install` recherche les catalogues qu'il a déjà et ne les actualise pas d'abord, et `/plugin install` actualise uniquement les marketplaces qui ont la mise à jour automatique activée. Nommez la marketplace, et Claude Code l'actualise avant de rechercher le plugin :

```text theme={null}
/plugin install <name>@<marketplace>
```

Lorsque l'installation fonctionne, vous voyez `✓ Installed <plugin>.` dans une session, ou `Successfully installed plugin: <plugin>@<marketplace>` à partir de `claude plugin install`.

Si vous ne savez pas quelle marketplace répertorie le plugin, exécutez `/plugin marketplace list` pour les marketplaces que vous avez, et parcourez **Discover** dans `/plugin` pour le nom du plugin.

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

Vous avez exécuté `/plugin install` pour un plugin qui est déjà installé à la portée utilisateur ou par les paramètres gérés, et Claude Code a refusé avec `Use '/plugin' to manage existing plugins.` Si vous avez tapé le nom du plugin sans `@<marketplace>`, le message omet `globally`.

Le plugin est déjà disponible dans chaque projet, donc il n'y a rien à ajouter. Pour modifier sa [portée](/docs/fr/plugins/install), l'activer ou le désactiver, ou le configurer, ouvrez `/plugin` et allez à **Installed**.

Un plugin installé uniquement à la portée du projet ou locale ne déclenche pas ce message. Claude Code vous permet de l'installer à la portée utilisateur aussi, donc il est disponible dans d'autres projets.

`claude plugin install` dans votre shell imprime un message différent. Pour un plugin déjà installé à la portée cible, il imprime `Plugin "<name>@<marketplace>" is already installed (scope: user)` et quitte 0. Si son répertoire de cache est manquant, la même commande le re-télécharge.

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

Vous avez installé un plugin dont l'entrée de marketplace utilise un type de source que cette version de Claude Code ne peut pas récupérer, et Claude Code s'est arrêté avec ce message et `Update Claude Code and try again.`

Mettez à jour Claude Code, puis réessayez l'installation. Les types de source se trouvent sur la [référence de la marketplace](/docs/fr/plugins/marketplace-reference).

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

Vous avez installé un plugin qui est distribué sous forme d'archive zip, et Claude Code l'a refusé avec cette ligne et `The archive was not installed.` L'entrée de marketplace du plugin utilise une source [`archive`](/docs/fr/plugins/marketplace-reference) avec une épingle `sha256`, et le digest du fichier téléchargé ne correspond pas à l'épingle.

Le message complet ressemble à ceci :

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

La correction diffère pour l'éditeur et l'installateur :

* **Vous publiez le plugin** : recalculez le digest du fichier exact que l'URL sert et mettez à jour le `sha256` dans l'entrée de marketplace. Utilisez `shasum -a 256 my-plugin.zip`, ou `Get-FileHash -Algorithm SHA256 my-plugin.zip` dans PowerShell
* **Vous installez le plugin** : exécutez `/plugin marketplace update <name>` dans une session pour actualiser le catalogue au cas où l'entrée aurait été corrigée, puis réessayez l'installation. Si les digests ne correspondent toujours pas après l'actualisation, demandez au propriétaire de la marketplace quel fichier ils ont épinglé avant d'installer

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

Une marketplace que vous avez ajoutée plus tôt a cessé de charger, tout comme ses plugins. Cette ligne apparaît dans l'onglet **Erreurs** de `/plugin` ou lors de la prochaine actualisation.

La marketplace est enregistrée sous un nom qui est [réservé aux marketplaces officielles d'Anthropic](/docs/fr/plugins/marketplace-reference), mais sa source enregistrée n'est pas un référentiel GitHub `anthropics`. Les noms réservés sont re-vérifiés chaque fois qu'une marketplace charge ou s'actualise, donc la marketplace et les plugins installés à partir de celle-ci cessent de charger.

Le message complet nomme le nom réservé et la correction :

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

La correction diffère pour les utilisateurs et les éditeurs :

* **Vous utilisez la marketplace** : dans votre shell, exécutez `claude plugin marketplace remove <name>`, puis ajoutez à nouveau la marketplace à partir du référentiel officiel `github.com/anthropics`
* **Vous publiez une marketplace tierce qui a utilisé le nom avant qu'il ne devienne réservé** : renommez-la et demandez aux utilisateurs de la réajouter à partir de votre source

Avant v2.1.205, Claude Code ne vérifiait le nom que lorsque vous ajoutiez la marketplace, donc une entrée enregistrée avant que son nom ne devienne réservé continuait à charger.

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` ou `has an invalid manifest file`
</h3>

Claude Code a récupéré le plugin, puis n'a pas pu lire son `.claude-plugin/plugin.json`. Dans le shell, le `<name>` dans cette ligne peut être un nom de répertoire temporaire ; le préfixe `Failed to install plugin "<name>@<marketplace>"` porte le vrai nom du plugin. Le libellé indique quel contrôle a échoué :

* **`corrupt manifest file`, suivi de `JSON parse error:`** : le fichier n'est pas un JSON valide
* **`invalid manifest file`, suivi de `Validation errors:`** : le fichier analyse mais échoue le schéma, comme `name: Invalid input` pour un champ obligatoire manquant

`claude plugin install` signale l'un ou l'autre comme `Failed to install plugin "<name>@<marketplace>":` et quitte avec le code 1.

L'auteur du plugin doit corriger le fichier, et le plugin ne peut pas être installé jusqu'à ce que ce soit fait :

* **Si c'est vous** : exécutez `claude plugin validate <plugin-directory>` dans votre shell pour voir la même erreur avec le chemin offensant, puis corrigez le fichier
* **Si ce n'est pas vous** : signalez le message au propriétaire de la marketplace

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

L'onglet **Erreurs** dans `/plugin` affiche ceci pour un plugin activé que sa marketplace répertorie par un chemin relatif, comme `./plugins/my-plugin`, lorsqu'aucun répertoire n'existe à ce chemin à l'intérieur de la marketplace. Si vous maintenez la marketplace, corrigez le chemin `source` de l'entrée ou restaurez le dossier. Sinon, signalez le message au propriétaire de la marketplace.

`Marketplace directory not found at path: <path>` signifie que le propre répertoire de la marketplace est manquant à la place. Pour une marketplace que vous avez ajoutée à partir d'un chemin local, ce répertoire a été déplacé ou supprimé. Restaurez-le, ou supprimez la marketplace et ajoutez-la à nouveau à partir de son nouvel emplacement.

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` ou `No marketplaces configured`
</h3>

Vous avez ouvert `/plugin` et l'onglet **Discover** est vide, ou `claude plugin marketplace list` a imprimé `No marketplaces configured`.

Aucune marketplace n'est enregistrée, donc il n'y a pas de catalogue à afficher. Dans une session, ajoutez la marketplace officielle, `anthropics/claude-plugins-official` :

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code imprime `Successfully added marketplace: claude-plugins-official`, et **Discover** répertorie ses plugins. La page [Marketplaces Anthropic](/docs/fr/plugins/anthropic-marketplaces) répertorie les autres marketplaces que vous pouvez ajouter.

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

Vous avez confirmé l'ajout d'une marketplace via [`/plugin install <plugin> --marketplace <source>`](/docs/fr/plugins/install#add-a-marketplace-and-install-in-one-command), et le catalogue que Claude Code a récupéré à partir de cette source a le même nom qu'une marketplace que vous avez déjà ajoutée à partir d'une source différente. Claude Code conserve la marketplace existante au lieu de la remplacer, et le plugin n'est pas installé.

Le message complet ressemble à ceci :

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

Choisissez quelle source vous voulez :

* **La marketplace que vous avez déjà ajoutée** : installez à partir de celle-ci par nom avec `/plugin install <plugin>@<name>`
* **La nouvelle source** : exécutez `/plugin marketplace remove <name>`, puis réessayez l'installation

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

Vous avez exécuté `marketplace add`, et le catalogue à cette source a le même nom qu'une marketplace qu'un fichier de paramètres déclare déjà sous [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces) avec une source différente. Claude Code refuse l'ajout et n'enregistre rien.

Le message se termine par la correction : la source doit correspondre à celle déclarée pour ce nom dans les paramètres, ou vous modifiez la déclaration. Comparez la source que vous avez passée par rapport à l'entrée `extraKnownMarketplaces` pour ce nom, y compris son `ref`, `path` et `headers`, puis faites l'une de ces choses :

* **Utilisez la source déclarée** : ajoutez la marketplace à partir de la source que l'entrée de paramètres nomme
* **Utilisez la nouvelle source** : modifiez ou supprimez l'entrée `extraKnownMarketplaces`, puis ajoutez à nouveau la marketplace. Si les paramètres gérés la déclarent, demandez à votre administrateur

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

Vous avez sélectionné des plugins à installer dans le menu `/plugin`, aucun d'eux n'a été installé, et le menu s'est fermé avec ce résumé de ce qui a échoué.

Certaines raisons, comme la sortie de git après un clonage échoué, affichent uniquement leur première ligne. Lorsqu'une telle raison a été raccourcie, le résumé se termine par `Installing a plugin from its details (Enter) in /plugin shows its full error.`

Ce qu'il faut faire dépend de si le résumé a raccourci la raison :

* Corrigez ce que la raison entre parenthèses nomme
* Lorsque la raison a été raccourcie, exécutez `/plugin`, sélectionnez le plugin sur l'onglet **Discover**, et appuyez sur **Entrée** pour l'installer à partir de ses détails. Si l'installation échoue là, la vue des détails affiche l'erreur complète

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

Lorsque vous installez un plugin, Claude Code télécharge une copie fraîche de ses fichiers et la déplace dans le dossier de cette version dans le [cache des plugins](/docs/fr/plugins/loading#find-plugins-on-disk). Ce message signifie que le déplacement a échoué, généralement parce qu'un autre programme utilisait le dossier pendant que l'installation s'exécutait. Le code du système de fichiers apparaît entre parenthèses :

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

Le message indique ce qui s'est passé avec la copie qui a été installée avant, ce qui vous indique si le plugin fonctionne toujours :

* `The previously installed copy was moved back` : la version que vous aviez est toujours installée
* `had to be removed first`, `was not moved back` ou `could not be moved back` : cette version du plugin n'est pas installée jusqu'à ce qu'une installation réussisse
* Aucune telle phrase : il n'y avait pas de copie antérieure, donc la version n'est pas encore installée

Sur Windows, lorsqu'un autre programme détient la copie installée elle-même, le message dit plutôt que cette copie `could not be replaced` et que `It was not replaced and the new copy was discarded`, donc la version que vous aviez est toujours installée.

Une liste `Left on disk` nomme les dossiers mis de côté à l'intérieur du cache. Une installation ultérieure de cette version ou un nettoyage du cache des plugins les supprime, donc vous n'avez pas besoin de les supprimer.

Pour corriger l'installation :

* Fermez les autres sessions Claude Code, les éditeurs et les terminaux qui utilisent le dossier du plugin sous `~/.claude/plugins/cache`, puis exécutez à nouveau l'installation
* Lorsque le message dit de vérifier les permissions du dossier du cache des plugins, restaurez votre permission d'écriture sur le dossier qu'il nomme et libérez de l'espace disque, puis exécutez à nouveau l'installation

<h3 id="dependency-errors">
  Erreurs de dépendance
</h3>

Un plugin qui déclare des dépendances peut échouer à installer, ou installer et rester désactivé, lorsqu'une dépendance ne peut pas être satisfaite. Le message vous parvient au moment de l'installation ou au moment du chargement :

* **Pendant l'installation** : le refus revient comme le message d'erreur de l'installation
* **Lorsque le plugin charge** : le problème apparaît dans `claude plugin list` et l'onglet **Erreurs** de `/plugin`, et Claude Code garde le plugin affecté désactivé jusqu'à ce que vous le résolviez

Le tableau répertorie chaque message et sa correction. Pour déclarer des dépendances en tant qu'auteur, consultez [Dépendances des plugins](/docs/fr/plugins/dependencies).

| Message                                                                                         | Signification                                                                                                          | Comment résoudre                                                                                                                                                                                                                                                                                        |
| :---------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Dependency "<dep>" is not installed`                                                           | Une dépendance déclarée n'est pas installée.                                                                           | Installez-la dans votre shell avec `claude plugin install <dep>@<marketplace>`, ou désinstallez le plugin. Si la marketplace de la dépendance n'est pas encore enregistrée, ajoutez-la et exécutez `/reload-plugins` dans votre session, qui installe les dépendances manquantes qu'elle peut résoudre. |
| `Dependency "<dep>" is disabled`                                                                | La dépendance est installée mais désactivée.                                                                           | Activez la dépendance, ou désinstallez le plugin qui en a besoin.                                                                                                                                                                                                                                       |
| `Requires "<dep>" <range>, installed <version>`                                                 | La version de la dépendance installée est en dehors de la plage déclarée du plugin.                                    | Mettez à jour la dépendance vers une version dans la plage, ou désinstallez le plugin.                                                                                                                                                                                                                  |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                          | Aucune version ne satisfait chaque plage qui l'épingle. Le message répertorie les plages.                              | Désinstallez ou mettez à jour l'un des plugins en conflit, ou demandez à l'auteur en amont d'élargir sa contrainte.                                                                                                                                                                                     |
| `... has version requirements too complex to intersect` ou `has an invalid version requirement` | Une plage n'est pas un semver valide, ou les plages combinées ne peuvent pas être intersectées.                        | Corrigez la plage invalide ou simplifiez les longues chaînes `\|\|`.                                                                                                                                                                                                                                    |
| `... has no git tag satisfying <range>`                                                         | Le référentiel de la dépendance n'a pas de balise `<name>--v*` dans la plage.                                          | Vérifiez que les balises en amont libèrent avec cette convention, ou assouplissez la plage.                                                                                                                                                                                                             |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist`  | La dépendance se trouve dans une marketplace différente, et la résolution inter-marketplace est désactivée par défaut. | Installez la dépendance vous-même à la même portée, dans votre shell avec `claude plugin install <dep>@<marketplace>` plus le `--scope` auquel vous installez le plugin, puis réessayez.                                                                                                                |

Pour voir ces par programmation, exécutez `claude plugin list --json` dans votre shell. Les plugins avec des problèmes portent un champ `errors` avec les messages et un champ `errorDetails` avec un `type` pour chacun : les deux premières lignes sont `dependency-unsatisfied` et la troisième est `dependency-version-unsatisfied`.

<h2 id="plugin-installed-but-not-working">
  Plugin installé mais ne fonctionne pas
</h2>

L'installation a réussi, mais les skills, hooks ou serveurs du plugin ne font rien. Commencez par [Le plugin n'apparaît pas ou ses skills ne s'affichent pas](#plugin-doesnt-appear-or-its-skills-dont-show-up), qui vous indique où Claude Code signale ce qu'il a chargé, puis faites correspondre le message.

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  Le plugin n'apparaît pas ou ses skills ne s'affichent pas
</h3>

Vous avez installé un plugin et avez tapé `/` en vous attendant à ses skills, ou avez demandé à Claude de l'utiliser, et rien ne s'est passé.

Vérifiez l'état du plugin avant de changer quoi que ce soit :

<Steps>
  <Step title="Confirmez que le plugin est installé et activé">
    Exécutez `/plugin` et ouvrez **Installed**. Confirmez que le plugin est répertorié et activé. `claude plugin list` dans votre shell imprime la même liste avec la version, la portée et le `Status: ✔ enabled` de chaque plugin.
  </Step>

  <Step title="Lisez l'onglet Erreurs">
    Ouvrez l'onglet **Erreurs** dans le même panneau. Chaque entrée associe un message à une ligne de guidance. La plupart des messages du reste de cette section proviennent de cet onglet.
  </Step>

  <Step title="Rechargez si vous avez installé pendant cette session">
    Si le plugin est installé et sans erreur mais que vous l'avez installé pendant cette session, exécutez `/reload-plugins`. Il imprime `Reloaded:` avec des comptes de plugins, skills, agents, hooks et serveurs. Lorsque quelque chose a échoué, il ajoute `N errors during load. Run /plugin for details.`
  </Step>
</Steps>

Si le plugin charge sans erreur et ses skills n'apparaissent toujours pas, l'étape suivante diffère pour votre propre plugin et pour celui de quelqu'un d'autre :

* **Un plugin que vous construisez** : consultez [Le plugin charge mais ses skills sont manquants](#plugin-loads-but-its-skills-are-missing)
* **Un plugin que quelqu'un d'autre a publié** : ouvrez **Installed** dans `/plugin` et ouvrez le volet de détails du plugin, qui répertorie ce que le plugin contient. Un plugin qui ne répertorie aucun skill là n'en a aucun à offrir lorsque vous tapez `/`

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

Le résumé d'installation dans `/plugin` s'est terminé par `Run /reload-plugins to activate.` au lieu de `Plugin is now active.`

Claude Code n'a pas activé le plugin pendant l'installation, soit parce que l'activer [invaliderait le cache d'invite](/docs/fr/prompt-caching#enabling-or-disabling-a-plugin) soit parce que la tentative d'activation a échoué.

Vous n'avez pas besoin de taper la commande. Le panneau se ferme et Claude Code exécute `/reload-plugins` pour vous, ou le met en file d'attente jusqu'à ce que la réponse qui s'écoule se termine.

Lisez ce que ce rechargement imprime :

* **`Reloaded:` avec des comptes de plugins, skills, agents, hooks et serveurs** : le plugin est maintenant actif. Lorsque quelque chose n'a pas pu charger, la ligne ajoute `N errors during load. Run /plugin for details.`
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`** : le rechargement ajouterait ou supprimerait un serveur MCP de plugin, ou l'outil `LSP`, et invaliderait votre cache d'invite. Pour le cas LSP, la ligne commence par `This reload adds the LSP tool` ou `This reload removes the LSP tool`. Exécutez-le avec `--force` pour activer le plugin de toute façon, ou démarrez une nouvelle session

Avant v2.1.268, une installation qui n'a pas été activée pendant l'installation restait en attente jusqu'à ce que vous exécutiez vous-même `/reload-plugins`.

Avant v2.1.246, le compte des skills dans ce résumé incluait uniquement les entrées `commands/` d'un plugin, donc un rechargement pouvait charger les skills `SKILL.md` d'un plugin et signaler toujours `0 skills`.

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

L'onglet **Erreurs** affiche cette ligne avec la guidance `Run /plugin to refresh the plugin cache`. Claude Code a un enregistrement d'installation pour le plugin, mais le répertoire que l'enregistrement pointe est manquant, par exemple après que vous ayez vidé le cache.

Réinstallez le plugin à partir de votre shell. `claude plugin install <name>@<marketplace>` re-télécharge un plugin dont le répertoire d'installation est manquant même si son enregistrement existe :

```shell theme={null}
claude plugin install <name>@<marketplace>
```

Ensuite, exécutez `/reload-plugins` dans votre session. L'entrée de l'onglet **Erreurs** disparaît et le plugin est de retour sous **Installed**.

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

Vous avez défini un plugin à `false` dans `~/.claude/settings.json`, et sa ligne dans `claude plugin list` ou `/plugin` affiche ce message suivi de la source qui l'active, comme `— project settings enable it, which overrides your user setting`. Un `true` dans cette source de précédence plus élevée remplace votre paramètre utilisateur.

Pour refuser un plugin activé par le projet sur votre machine, définissez l'id à `false` dans `.claude/settings.local.json`, qui a une précédence plus élevée que le fichier du projet. Pour les autres sources que le message peut nommer, consultez [Désactivé dans les paramètres utilisateur mais charge toujours](/docs/fr/plugins/loading#disabled-in-user-settings-but-still-loads).

Si `claude plugin list` marque plutôt le plugin `required by your org`, aucun fichier de paramètres n'est impliqué : votre organisation marque ce plugin synchronisé comme requis sur claude.ai, et il charge même si vous l'aviez désactivé plus tôt. Consultez [Plugins synchronisés à partir de claude.ai](/docs/fr/plugins/loading#synced-plugins).

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

L'onglet **Erreurs** affiche cette ligne pour un plugin que le `.claude/settings.json` de votre projet active, avec la guidance `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`.

Les paramètres d'un référentiel peuvent activer un plugin pour tous ceux qui l'ouvrent, mais ils ne l'installent pas. Lorsque le plugin provient d'une source externe telle qu'un référentiel GitHub ou un package npm, Claude Code ne le télécharge pas jusqu'à ce que vous l'installiez vous-même. Exécutez la commande de la ligne de guidance dans votre shell, puis rechargez :

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

Après avoir exécuté `/reload-plugins` dans votre session, l'entrée de l'onglet **Erreurs** est partie et le plugin est répertorié sous **Installed**.

Si votre organisation pré-installe des plugins pour vous, elle le fait via les paramètres gérés à la place. Consultez [Pré-installer et exiger des plugins](/docs/fr/plugins/org#pre-install-and-require-plugins).

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` et hooks qui ne se déclenchent pas
</h3>

Les hooks d'un plugin ne s'exécutent pas. Soit l'onglet **Erreurs** affiche un échec de chargement pour eux, les hooks chargent et vous voyez des avis `<Event> hook error` dans la transcription, soit un hook charge sans erreur et ne se déclenche jamais.

<h4 id="hooks-fail-to-load">
  Les hooks échouent à charger
</h4>

L'onglet **Erreurs** affiche l'un de ces messages :

* **`Failed to load hooks from <path>: <reason>`** : `hooks/hooks.json` n'est pas un JSON valide ou échoue le schéma des hooks. La raison nomme l'erreur d'analyse ou de validation. Corrigez le fichier. Pour attraper un problème de syntaxe JSON dans `hooks/hooks.json` avant de publier le plugin, exécutez `claude plugin validate <plugin-directory>` dans votre shell
* **`hooks path not found: <path>`** : le champ `hooks` du manifeste nomme un fichier qui n'existe pas à ce chemin relatif à la racine du plugin. Corrigez le chemin ou ajoutez le fichier

<h4 id="hook-error-notices-in-the-transcript">
  Avis `hook error` dans la transcription
</h4>

Un avis de la forme `... hook error: Failed with non-blocking status code: <stderr>` signifie que le hook s'est exécuté et sa commande a échoué. Par exemple, `Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found` signifie que le shell que Claude Code a généré n'a pas pu trouver `node`. Installez-le, ou assurez-vous qu'il est sur le `PATH` du terminal à partir duquel vous démarrez `claude`.

Pour toute autre erreur, exécutez la commande du hook vous-même à partir du répertoire du plugin pour voir la sortie complète, ou capturez le stderr complet avec [journalisation de débogage](/docs/fr/hooks#debug-hooks).

<h4 id="hook-loads-but-never-fires">
  Le hook charge mais ne se déclenche jamais
</h4>

Si un hook charge sans erreur mais ne se déclenche jamais, vérifiez sa définition puis regardez-le s'exécuter :

<Steps>
  <Step title="Vérifiez le nom de l'événement">
    Les noms d'événements sont sensibles à la casse, donc confirmez que le vôtre correspond exactement, par exemple `PostToolUse`.
  </Step>

  <Step title="Vérifiez le matcher">
    Confirmez que le `matcher` du hook correspond au nom de l'outil.
  </Step>

  <Step title="Déclenchez l'événement exprès">
    Pour un hook `PostToolUse`, demandez à Claude d'éditer un fichier.
  </Step>

  <Step title="Lisez le journal de débogage">
    Ouvrez le [journal de débogage](/docs/fr/hooks#debug-hooks), qui enregistre quels hooks ont correspondu. Un hook qui s'est exécuté y apparaît avec son code de sortie.
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` et serveurs MCP qui ne démarrent pas
</h3>

Un plugin regroupe un serveur MCP, et l'onglet **Erreurs** affiche `Invalid MCP server config for "<server>": <error>`, ou le serveur est répertorié mais `/mcp` ne le montre jamais connecté.

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

La configuration du serveur passe la vérification du schéma, mais Claude Code ne peut pas la résoudre pour cette session. Le texte après les deux points nomme la cause et décide de la correction :

* **`Missing environment variables: <names>`** : définissez ces variables dans le shell à partir duquel vous démarrez Claude Code, puis démarrez une nouvelle session
* **`URL is unset or invalid`** : une option `${user_config.*}` que l'URL utilise n'est pas définie. Exécutez `/plugin configure <plugin>` pour la définir
* **`has an invalid MCP url`** ou **`headersHelper for MCP server '<server>' references ${user_config.*}`** : la configuration du plugin lui-même est en faute. Corrigez l'`url` ou `headersHelper` dans la configuration MCP de votre plugin, ou signalez-le à l'auteur du plugin si le plugin n'est pas le vôtre. Le cas `headersHelper` a sa propre entrée sous [la commande de plugin référence user\_config](/docs/fr/errors#plugin-command-references-user-config)

<h4 id="server-is-configured-but-never-connects">
  Le serveur est configuré mais ne se connecte jamais
</h4>

Exécutez `/mcp` pour voir l'état du serveur. Lorsque le serveur est sain, `/mcp` le répertorie comme connecté.

Pour lire l'erreur que le serveur a imprimée au démarrage, exécutez `claude --debug` et ouvrez le journal à `~/.claude/debug/<session-id>.txt`. Le drapeau `--debug` n'imprime pas au terminal.

Une entrée de serveur dans `.mcp.json` qui échoue le schéma n'apparaît pas dans l'onglet **Erreurs**. Claude Code supprime ce serveur et enregistre `Invalid MCP server config for <server> in <path>` uniquement dans ce journal de débogage. Pour trouver l'entrée sans charger le plugin, exécutez `claude plugin validate` dans votre shell sur le répertoire du plugin, qui la signale comme une erreur.

Avant v2.1.281, `claude plugin validate` ne vérifiait pas `.mcp.json`.

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  Le serveur fonctionne avec `--plugin-dir` mais échoue après l'installation
</h4>

Vous êtes l'auteur du plugin, et le serveur démarre lorsque vous chargez le plugin à partir de son répertoire source avec `--plugin-dir` mais échoue une fois que le plugin est installé.

Claude Code copie un plugin installé dans son cache, donc un chemin qui ne fonctionne que depuis le répertoire source se casse. Écrivez les chemins à l'intérieur du plugin avec `${CLAUDE_PLUGIN_ROOT}`.

Pour les chemins qui atteignent en dehors du répertoire du plugin, consultez [Les fichiers que le plugin référence en dehors de son répertoire ne sont pas trouvés](#files-the-plugin-references-outside-its-directory-arent-found).

<h3 id="language-server-doesnt-start">
  Le serveur de langage ne démarre pas, utilise trop de mémoire ou signale des diagnostics incorrects
</h3>

Vous avez installé un [plugin d'intelligence de code](/docs/fr/plugins/code-intelligence) et Claude ne voit pas de diagnostics, ou le serveur de langage utilise trop de mémoire ou signale des erreurs qui ne sont pas réelles.

<h4 id="language-server-doesn’t-start">
  Le serveur de langage ne démarre pas
</h4>

Le plugin se connecte à un binaire de serveur de langage que vous installez séparément, et Claude Code le génère par nom de commande à partir de votre `PATH`.

L'onglet **Erreurs** de `/plugin` affiche l'échec avec sa raison, comme `Executable not found in $PATH: "<binary>"`, et `claude --debug` le journalise comme `LSP server <name> failed to start: <reason>`.

Installez le binaire et confirmez qu'il est sur le `PATH` du terminal à partir duquel vous démarrez `claude`, par exemple avec `which typescript-language-server`. Ensuite, démarrez une nouvelle session.

<h4 id="language-server-uses-too-much-memory">
  Le serveur de langage utilise trop de mémoire
</h4>

Les serveurs de langage tels que `rust-analyzer` et `pyright` indexent le projet entier. Désactivez le plugin avec `/plugin disable <plugin>` dans une session et fiez-vous plutôt aux outils de recherche intégrés de Claude.

<h4 id="false-positive-diagnostics-in-a-monorepo">
  Faux diagnostics positifs dans un monorepo
</h4>

Un serveur de langage qui n'est pas configuré pour l'espace de travail peut signaler des importations non résolues pour les packages internes. Il n'y a rien à corriger du côté Claude Code, et les diagnostics n'empêchent pas Claude d'éditer le code.

<h2 id="build-a-plugin">
  Construire un plugin
</h2>

Vous développez un plugin et le chargez avec `--plugin-dir` ou l'installez à partir d'une marketplace locale. Ces entrées couvrent les échecs que vous rencontrez lors du développement d'un plugin. Pour que les vérifications s'exécutent après chaque modification, consultez [Tester et déboguer](/docs/fr/plugins/create#test-and-debug).

Deux échecs qui atteignent également les utilisateurs d'un plugin ont leurs entrées sous [Plugin installé mais ne fonctionne pas](#plugin-installed-but-not-working) :

* **Un hook qui ne se déclenche pas** : consultez [hooks qui ne se déclenchent pas](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **Un serveur MCP qui ne démarre pas** : consultez [Les serveurs MCP qui ne démarrent pas](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

L'onglet **Erreurs** affiche `commands path not found: <absolute path>` avec la guidance `Check that the path in your manifest or marketplace config is correct`. Le même message apparaît pour `skills`, `agents` et `hooks`.

Claude Code a résolu un chemin à partir de votre `plugin.json` ou entrée de marketplace par rapport à la racine du plugin et n'a rien trouvé là. Le chemin dans le message est le chemin absolu qu'il a vérifiée, donc comparez-le avec ce qui est sur le disque. Corrigez le chemin ou créez le répertoire, puis exécutez `/reload-plugins`.

Les chemins dans le manifeste sont relatifs à la racine du plugin et commencent par `./`. Un chemin qui se résout en dehors de la racine du plugin est signalé comme `<component> path escapes plugin directory` à la place et est supprimé.

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` à une racine de marketplace ne charge pas les plugins sous `plugins/`
</h3>

Vous avez démarré `claude --plugin-dir <path>` et ne voyez aucune erreur, mais les skills, agents et hooks du plugin ne sont pas là.

`--plugin-dir` prend le répertoire racine du plugin, celui qui contient `.claude-plugin/plugin.json` et les répertoires de composants tels que `skills/`. Si vous le pointez plutôt à une racine de marketplace, Claude Code ne lit pas `marketplace.json`, donc un plugin sous `plugins/` ne charge pas, et vous ne voyez aucune erreur. Avant v2.1.281, Claude Code chargeait une racine de marketplace comme un plugin vide nommé d'après ce répertoire. Pointez le drapeau au répertoire du plugin lui-même :

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

Ensuite, ouvrez **Installed** dans `/plugin`, où le volet de détails du plugin répertorie ses composants.

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  Les fichiers que le plugin référence en dehors de son répertoire ne sont pas trouvés
</h3>

Un plugin fonctionne à partir de son répertoire source avec `--plugin-dir` mais échoue après l'installation, avec des erreurs concernant un chemin comme `../shared-utils`.

Claude Code copie un plugin installé dans son cache et le charge à partir de là, donc un chemin qui atteint en dehors du propre répertoire du plugin ne pointe vers rien dans le cache. Déplacez les fichiers partagés à l'intérieur du répertoire du plugin, ou référencez-les via un lien symbolique à l'intérieur de celui-ci. Pour où se trouve le cache et comment les chemins se résolvent, consultez [Trouvez les plugins sur le disque](/docs/fr/plugins/loading#find-plugins-on-disk).

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` affiche des barres obliques avant sur Windows
</h3>

Sur Windows, un hook de plugin reçoit `${CLAUDE_PLUGIN_ROOT}` comme `C:/Users/you/...` plutôt que `C:\Users\you\...`, et un script qui s'attendait à des barres obliques inverses se casse.

Claude Code exécute les hooks de forme shell via Git Bash sur Windows et substitue la racine du plugin dans la forme Win32 avec barres obliques avant exprès. Les builtins Bash, les outils MSYS et les binaires Windows natifs acceptent tous cette forme.

Si votre script a besoin de barres obliques inverses, basculez le hook vers l'une des formes qui conservent les chemins natifs, décrites sous [forme exec et forme shell](/docs/fr/hooks#exec-form-and-shell-form) :

* Un hook de forme exec, qui génère le processus directement avec un tableau `args`
* Un hook avec `"shell": "powershell"`

<h3 id="plugin-loads-but-its-skills-are-missing">
  Le plugin charge mais ses skills sont manquants
</h3>

Votre plugin est répertorié sous **Installed** sans erreurs, mais ses skills ne sont pas offerts lorsque vous tapez `/`.

Les skills chargent à partir de `skills/` à la racine du plugin et les commandes à partir de `commands/` à la racine du plugin. Seul `plugin.json` appartient à l'intérieur de `.claude-plugin/`, et un répertoire `skills/` à l'intérieur de `.claude-plugin/` n'est pas scanné. Déplacez les répertoires à la racine du plugin et exécutez `/reload-plugins`. Après cela, le volet de détails du plugin dans `/plugin` répertorie les skills, et taper `/` les offre.

Chaque skill est un répertoire contenant `SKILL.md`. Une entrée `skills` dans le manifeste qui pointe vers un fichier `SKILL.md` plutôt que son répertoire est signalée comme `path is a file; skills entries must be directories containing SKILL.md`.

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  Le skill charge mais Claude n'invoque jamais le skill
</h3>

Le skill de votre plugin s'exécute lorsque vous tapez sa commande `/<plugin>:<skill>`, mais Claude ne l'invoque jamais en réponse à une demande simple.

Vérifiez ces causes dans l'ordre :

* **Le skill définit `disable-model-invocation: true`** : avec ce champ défini, seul vous pouvez invoquer le skill. Le modèle de skill dans [Créez votre premier plugin](/docs/fr/plugins/create#create-your-first-plugin) le définit. Supprimez la ligne d'un skill que vous voulez que Claude invoque de lui-même. [Contrôlez qui invoque un skill](/docs/fr/skills#control-who-invokes-a-skill) couvre le champ
* **La description ne correspond pas à la façon dont les gens demandent** : travaillez à travers les vérifications dans [Le skill ne se déclenche pas](/docs/fr/skills#skill-not-triggering)
* **La description est tronquée** : lorsque de nombreux skills sont installés, Claude Code raccourcit les descriptions pour s'adapter au budget de caractères de la liste, ce qui peut supprimer les mots-clés dont Claude a besoin pour correspondre à une demande. Consultez [Les descriptions de skill sont coupées court](/docs/fr/skills#skill-descriptions-are-cut-short)

Pour mesurer la fréquence à laquelle le skill se déclenche sur des invites réalistes plutôt que de vérifier une à la fois, écrivez un cas d'évaluation avec un [évaluateur `tool_used: Skill`](/docs/fr/plugin-evals#create-your-first-eval-suite) et exécutez-le avec `claude plugin eval` après chaque modification de description.

<h3 id="is-not-a-plugin-or-skill-folder">
  `<directory> is not a plugin or skill folder` à partir de `claude plugin eval init`
</h3>

Vous avez exécuté `claude plugin eval init` à partir d'un répertoire qui n'est pas la racine d'un plugin, comme votre répertoire personnel ou la racine d'un référentiel qui garde le plugin dans un sous-répertoire. `init` écrit la suite sous le répertoire de travail, donc il s'arrête au lieu de créer un répertoire `evals/` que le plugin ne verrait jamais.

Changez à la racine du plugin, le répertoire qui contient `.claude-plugin/plugin.json` ou le `SKILL.md` du skill, et exécutez à nouveau la commande. Pour échafauder la suite ailleurs exprès, passez `--eval-dir`. Consultez [Testez les plugins avec des evals](/docs/fr/plugin-evals).

<h3 id="the-userconfig-dialog-never-appears">
  La boîte de dialogue `userConfig` n'apparaît jamais
</h3>

Votre plugin déclare des options `userConfig`, mais aucune boîte de dialogue de configuration n'apparaît lorsque vous l'installez.

L'installation interactive affiche la boîte de dialogue, et la commande shell prend les valeurs comme drapeaux à la place :

* **`/plugin install` dans une session, ou l'onglet Discover dans `/plugin`** : la boîte de dialogue fait partie de cette installation interactive
* **`claude plugin install` dans votre shell** : ne demande jamais les valeurs `userConfig`. Il enregistre toutes les valeurs `--config KEY=VALUE` que vous passez, et lorsque les options restent non définies, il imprime `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` Lorsque l'une des options non définies est requise, `(M required)` suit `not yet set`.

Si vous avez installé à partir du shell, passez les valeurs avec `--config`, un drapeau par option :

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

Lorsque chaque option est définie, la sortie d'installation ne porte aucune ligne `not yet set`. Pour ouvrir la boîte de dialogue après coup à la place, exécutez `/plugin configure my-plugin@my-marketplace` dans une session.

Si vous passez une clé `--config` que le manifeste ne déclare pas, le plugin s'installe toujours, et la commande imprime `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` suivi des clés que le plugin déclare.

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` signale des erreurs
</h3>

Vous avez exécuté `claude plugin validate <path>`, ou `/plugin validate <path>` dans une session, et il a imprimé `Found N errors` et `Validation failed`, puis a quitté avec le code 1.

Le validateur lit le manifeste au chemin que vous lui donnez : `.claude-plugin/plugin.json` pour un répertoire de plugin, ou `.claude-plugin/marketplace.json` pour un répertoire de marketplace. Pour une marketplace, il préfixe les problèmes dans le propre manifeste d'une entrée avec l'index de l'entrée, comme `plugins[1] plugin.json → json: ...`.

Le tableau couvre les messages qui arrêtent la validation et deux avertissements, `No frontmatter block found` et `Unknown field '<key>'`, qui l'arrêtent uniquement lorsque vous passez `--strict`. Les autres avertissements, comme une description manquante, ne sont pas répertoriés.

| Message                                                                                                  | Cause                                                                              | Correction                                                                                                                       |
| :------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: <path>`                                                                                 | Le chemin n'a pas de manifeste, ou n'existe pas.                                   | Exécutez la commande par rapport à la racine du plugin ou de la marketplace, le répertoire qui contient `.claude-plugin/`.       |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | Le répertoire n'a pas de manifeste `.claude-plugin/`.                              | Créez le manifeste, ou pointez au bon répertoire.                                                                                |
| `Invalid JSON syntax: <parse error>`                                                                     | Le manifeste, ou `hooks/hooks.json`, n'est pas un JSON valide.                     | Corrigez le JSON. Jusqu'à ce que vous corrigiez `hooks/hooks.json`, une session charge le plugin sans les hooks dans ce fichier. |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | Un chemin de composant dans le manifeste n'existe pas.                             | Corrigez le chemin ou créez le répertoire.                                                                                       |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | Un chemin de composant échappe au répertoire du plugin.                            | Utilisez les chemins à l'intérieur de la racine du plugin.                                                                       |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | Une entrée `skills` pointe vers `SKILL.md` au lieu de son répertoire.              | Pointez au répertoire parent, ou `.` pour un `SKILL.md` au niveau racine.                                                        |
| `No frontmatter block found` ou `YAML frontmatter failed to parse: <error>`                              | Un fichier de skill, agent ou commande a un frontmatter YAML manquant ou invalide. | Ajoutez ou corrigez le frontmatter entre les délimiteurs `---`. Signalé lors de la validation d'un répertoire de plugin.         |
| `Unknown field '<key>'`                                                                                  | Le manifeste a un champ que le schéma ne définit pas.                              | Supprimez-le, ou utilisez le nom que le message suggère. Claude Code ignore les champs inconnus au moment du chargement.         |

Exécutez la commande à nouveau après chaque correction jusqu'à ce qu'elle n'imprime aucune erreur.

Les champs `plugin.json` se trouvent sur la [référence du manifeste](/docs/fr/plugins/manifest-reference), et les messages au niveau de la marketplace se trouvent sous [Erreurs de validation de la marketplace](#marketplace-validation-errors).

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

Le plugin échoue à charger avec `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

Le plugin a son propre `plugin.json`, et son entrée de marketplace définit `strict: false` tout en déclarant également l'un de `commands`, `agents`, `skills`, `hooks`, `outputStyles` ou `themes`. Supprimez ces champs de l'entrée, ou définissez `strict: true` dans l'entrée pour que Claude Code les ajoute à `plugin.json`. Consultez [Mode strict](/docs/fr/plugins/marketplace-reference#strict-mode).

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

Lorsque le plugin charge, le journal `claude --debug` à `~/.claude/debug/<session-id>.txt` enregistre `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` Rien n'apparaît dans la session ou l'onglet **Erreurs**.

Le chemin `commands` dans le manifeste existe mais ne contient aucun fichier `.md` et aucun `SKILL.md` dans un sous-répertoire. Ajoutez les fichiers de commande, ou supprimez le chemin du manifeste.

<h2 id="host-a-marketplace">
  Héberger une marketplace
</h2>

Vous publiez une marketplace et un utilisateur signale une erreur, ou votre propre validation échoue. Ces entrées sont pour le propriétaire de la marketplace.

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Les plugins avec des chemins relatifs échouent dans les marketplaces basées sur URL
</h3>

Les utilisateurs ont ajouté votre marketplace avec une URL `https://example.com/marketplace.json`. Les installations de plugins dont la `source` est un chemin relatif, comme `./plugins/my-plugin`, échouent avec `its marketplace entry path does not stay inside the marketplace directory`. Les plugins déjà installés échouent à charger avec `Plugin source path refused`. Les deux messages ont une [entrée de référence d'erreur](/docs/fr/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory).

Lorsqu'un utilisateur ajoute une marketplace basée sur URL, Claude Code télécharge uniquement le fichier `marketplace.json` lui-même. Il ne récupère pas les fichiers de plugin par chemin relatif à partir de ce serveur, donc un chemin relatif dans une entrée pointe vers un répertoire qui n'a jamais été récupéré. Donnez à chaque entrée une source que Claude Code peut récupérer de lui-même, comme un référentiel GitHub :

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

Alternativement, hébergez la marketplace dans un référentiel git et dites aux utilisateurs de l'ajouter avec l'URL du référentiel. Pour une source git, Claude Code clone le référentiel entier, donc les chemins relatifs se résolvent. Les types de source se trouvent sur la [référence de la marketplace](/docs/fr/plugins/marketplace-reference).

<h3 id="marketplace-validation-errors">
  Erreurs de validation de la marketplace
</h3>

Vous avez exécuté `claude plugin validate .` à partir de votre répertoire de marketplace et il a signalé des erreurs ou des avertissements sur le fichier de marketplace lui-même.

`claude plugin validate` valide également chaque entrée dont la `source` est un chemin local et avertit lorsque la `version` de l'entrée ne correspond pas au manifeste du plugin lui-même.

Le tableau répertorie les messages au niveau de la marketplace. Les messages au niveau de l'entrée sont les messages de plugin sous [`claude plugin validate` signale des erreurs](#claude-plugin-validate-reports-errors), préfixés avec `plugins[N] plugin.json →`.

| Message                                                                                                                   | Type          | Correction                                                                                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------ | :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Duplicate plugin name "<name>" found in marketplace`                                                                     | Erreur        | Donnez à chaque plugin un `name` unique.                                                                                                                         |
| `Path contains "..": <path>` sous `plugins[N].source`                                                                     | Erreur        | Utilisez les chemins relatifs à la racine de la marketplace sans segments `..`.                                                                                  |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                          | Erreur        | Supprimez le caractère du nom, comme une échappement ou une nouvelle ligne.                                                                                      |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                               | Erreur        | Supprimez le caractère du `name` du plugin.                                                                                                                      |
| `Marketplace has no plugins defined`                                                                                      | Avertissement | Ajoutez au moins une entrée à `plugins`.                                                                                                                         |
| `No marketplace description provided`                                                                                     | Avertissement | Ajoutez une `description` au niveau supérieur.                                                                                                                   |
| `Plugin name "<name>" is not kebab-case` sous `plugins[N] plugin.json → name`                                             | Avertissement | Renommez en lettres minuscules, chiffres et tirets. Claude Code accepte d'autres formes, mais la synchronisation de la marketplace claude.ai les rejette.        |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                          | Avertissement | Mettez à jour l'entrée pour correspondre à `plugin.json`, qui est autoritaire au moment de l'installation.                                                       |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                 | Avertissement | Renommez la marketplace. La synchronisation de la marketplace gérée de Claude Desktop rejette `org`, `org-provisioned` et `unknown` dans n'importe quelle casse. |
| `Marketplace name "<name>" is not accepted by Claude Desktop` ou `Plugin name "<name>" is not accepted by Claude Desktop` | Avertissement | Renommez en au maximum 128 caractères de lettres, chiffres, `.`, `_` et `-`, commençant par une lettre ou un chiffre.                                            |

Avant v2.1.247, un nom de marketplace contenant des caractères de contrôle ou de formatage bidirectionnel était signalé uniquement comme `Marketplace name impersonates an official Anthropic/Claude marketplace`.

<h2 id="blocked-by-your-organization">
  Bloqué par votre organisation
</h2>

Votre organisation déploie des paramètres gérés qui restreignent les plugins, et une commande a été refusée avec un message de politique. Ces entrées nomment le paramètre derrière chaque refus pour que vous sachiez ce qu'il faut demander à votre administrateur. Pour le côté administrateur, consultez [Gérez les plugins pour votre organisation](/docs/fr/plugins/org).

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

Vous avez exécuté `/plugin marketplace add`, `update` ou une installation, et Claude Code a refusé avec cette ligne. Pour une source GitHub ou git, l'hôte suit la source entre parenthèses, comme dans `'github:owner/repo' (github.com)`.

Votre administrateur a défini `blockedMarketplaces` ou `strictKnownMarketplaces` dans les paramètres gérés, et cette source n'est pas autorisée. Demandez à votre administrateur d'autoriser la source, ou ajoutez l'une des sources autorisées que le message répertorie.

Faites correspondre le reste du message pour voir quel type de politique a bloqué la source :

* **`Allowed sources: <list>`** : le bloc provient de la liste d'autorisation `strictKnownMarketplaces` plutôt que de la liste de blocage `blockedMarketplaces`
* **`No external marketplaces are allowed.`** : la liste d'autorisation `strictKnownMarketplaces` est vide
* **Un `Tip:` que le raccourci suppose github.com** : la liste d'autorisation autorise un hôte git par nom d'hôte, et le raccourci `owner/repo` que vous avez passé pointe vers github.com. Si le référentiel se trouve sur votre hôte interne, ajoutez-le à nouveau avec son URL complète, comme `git@your-git-host.com:owner/repo.git`

Une marketplace que vous avez ajoutée avant que la politique ne devienne plus restrictive cesse également de s'actualiser, car la politique s'applique à chaque actualisation.

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

L'onglet **Erreurs** affiche cette ligne, ou `Marketplace "<name>" is blocked by enterprise policy`, pour une marketplace que vous avez déjà enregistrée.

Les mêmes paramètres gérés qui bloquent une [source de marketplace](#marketplace-source-is-blocked-by-enterprise-policy) s'appliquent au moment du chargement. `strictKnownMarketplaces` n'inclut pas cette marketplace, ou `blockedMarketplaces` la nomme, donc Claude Code cesse de la charger et ses plugins. Pour la variante de liste d'autorisation, la ligne de guidance affiche les sources autorisées, ou `Contact your administrator to configure allowed marketplace sources`. Pour la variante de liste de blocage, elle se lit comme `This marketplace source is explicitly blocked by your administrator`.

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

Une installation a été refusée avec cette ligne, une activation avec la même ligne se terminant par `cannot be enabled`, ou une installation ou mise à jour avec une nommant la raison : `Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy`, ou `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`.

Les paramètres gérés bloquent ce plugin, sa marketplace ou une dépendance dont il a besoin. Demandez à votre administrateur quelle entrée s'applique. Une dépendance bloquée signifie que le plugin ne peut pas s'installer jusqu'à ce que la marketplace de la dépendance soit autorisée.

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

Vous avez démarré `claude` avec `--plugin-dir`, `--plugin-url`, `--agents` ou `--mcp-config`. Claude Code a quitté avec ce message et `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

Votre administrateur a défini `disableSideloadFlags` dans les paramètres gérés, ce qui désactive les drapeaux qui chargent les plugins, agents et serveurs à partir de chemins arbitraires. Chargez le plugin à partir d'une marketplace approuvée à la place, ou demandez à votre administrateur de supprimer le paramètre.

Un message connexe dans l'onglet **Erreurs** de `/plugin` est `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`. Les paramètres gérés activent ou désactivent ce plugin par nom, et Claude Code ignore votre copie `--plugin-dir` de celui-ci pour que le drapeau ne puisse pas remplacer la politique.

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

Vous avez exécuté `claude plugin init` ou `claude plugin enable`, et il s'est arrêté avec cette ligne. Le message nomme `strictKnownMarketplaces or blockedMarketplaces` et demande à votre administrateur d'ajouter `{"source":"skills-dir"}` à `strictKnownMarketplaces` ou de le supprimer de `blockedMarketplaces`.

La source `skills-dir` représente les plugins que Claude Code charge à partir de votre répertoire `~/.claude/skills/`. Demandez à votre administrateur de faire le changement que le message nomme.

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

Vous avez installé ou mis à jour un plugin avec une source `command`, et il s'est arrêté avec cette ligne et `The plugin was not installed or updated and its command was not run.`

Votre administrateur a défini `disableCommandPluginSources`, donc Claude Code refuse d'exécuter la commande déclarée par la marketplace qui produit le plugin. Définir `allowManagedHooksOnly` seul a le même effet lorsque `disableCommandPluginSources` n'est pas défini. Demandez à votre administrateur si le plugin peut être publié à partir d'un type de source que la politique autorise.

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

Vous avez exécuté `claude plugin marketplace update <name>`, et il a échoué avec `Marketplace '<name>' is seed-managed (<dir>)` et un conseil de demander à votre administrateur.

Un opérateur a pré-rempli cette marketplace via `CLAUDE_CODE_PLUGIN_SEED_DIR`, et Claude Code traite une marketplace gérée par seed comme en lecture seule. Une mise à jour en masse `marketplace update` la saute et met à jour les autres.

Pour modifier le contenu de la marketplace, demandez à la personne qui maintient l'image seed de la mettre à jour. Pour la procédure, consultez [Ensemencer les conteneurs et CI](/docs/fr/plugins/org#seed-containers-and-ci).

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Référence de chargement des plugins](/docs/fr/plugins/loading) : pourquoi les portées, le cache et la précédence se comportent de cette façon
* [Référence des commandes de plugin](/docs/fr/plugins/cli-reference) : drapeaux, valeurs par défaut, sortie et codes de sortie pour les commandes `claude plugin`
* [Installer et gérer les plugins](/docs/fr/plugins/install) : les étapes d'installation depuis le début
* [Gérez les plugins pour votre organisation](/docs/fr/plugins/org#troubleshoot-policy) : dépannage du côté politique pour les administrateurs
