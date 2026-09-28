> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gérer les plugins Claude Code pour votre organisation

> Contrôlez les plugins que Claude Code installe et autorise sur chaque machine de votre organisation via des paramètres gérés.

Les paramètres gérés vous permettent de décider quels plugins Claude Code installe et autorise sur chaque machine de votre organisation. Les utilisateurs ne peuvent pas les remplacer. Vous les livrez soit sous forme de [paramètres gérés par le serveur](/docs/fr/server-managed-settings) depuis la console d'administration claude.ai, soit sous forme de paramètres gérés par le point de terminaison via MDM ou un fichier `managed-settings.json`. La plupart des contrôles de cette page ne prennent effet que depuis les paramètres gérés.

Cette page est destinée aux administrateurs et les paramètres ici gouvernent Claude Code.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Installation de plugins pour vous-même** : commencez par [Installer des plugins](/docs/fr/plugins/install)
  * **Contrôler les plugins que les membres peuvent utiliser dans claude.ai et Cowork** : voir [Gérer les plugins pour votre organisation](https://support.claude.com/en/articles/13837433) dans le centre d'aide
  * **La page des plugins dans les paramètres d'administration de claude.ai** : [**Paramètres de l'organisation > Plugins et compétences**](https://claude.ai/admin-settings/skills?tab=inventory) active les plugins pour les comptes claude.ai des membres, et ceux-ci atteignent Claude Code sous forme de [plugins synchronisés](/docs/fr/plugins/loading#synced-plugins). Il ne définit aucune des clés de cette page
</Note>

Les sections suivent l'ordre que prennent la plupart des déploiements : [exiger des plugins](#pre-install-and-require-plugins) pour tout le monde ou par référentiel, [ensemencer les conteneurs et l'IC](#seed-containers-and-ci), [restreindre](#restrict-what-users-can-install) ce que les utilisateurs peuvent ajouter eux-mêmes, [définir la politique de mise à jour](#set-update-policy), puis [auditer](#audit-and-review) ce qui est installé. Pour examiner chaque clé de politique en un seul endroit, voir la [matrice de contrôle](#control-matrix).

<h2 id="pre-install-and-require-plugins">
  Pré-installer et exiger des plugins
</h2>

Un marketplace est un catalogue de plugins que Claude Code récupère à partir d'un référentiel git, d'une URL ou d'un chemin local. Une fois que vous enregistrez un marketplace sur une machine, Claude Code peut installer des plugins à partir de celui-ci.

Pour installer des plugins pour une flotte, définissez deux clés ensemble dans les [paramètres gérés](/docs/fr/managed-settings), le fichier de politique ou la politique livrée par le serveur que chaque machine de votre organisation lit : `extraKnownMarketplaces` enregistre un marketplace sur chaque machine, et `enabledPlugins` nomme les plugins à installer et activer à partir de celui-ci. [Choisir un mécanisme de livraison](#choose-a-delivery-mechanism) couvre comment les paramètres gérés atteignent chaque machine.

<h3 id="choose-a-delivery-mechanism">
  Choisir un mécanisme de livraison
</h3>

Les paramètres gérés atteignent une machine via l'un de trois mécanismes de livraison :

* **Paramètres gérés par le serveur** : définissez les clés de plugin en JSON à [**Paramètres de l'organisation > Claude Code > Paramètres gérés**](https://claude.ai/admin-settings/claude-code). Nécessite un [rôle Propriétaire](/docs/fr/server-managed-settings#access-control) dans votre organisation Claude. Une session cloud récupère ces paramètres avant d'installer les plugins.
* **Politiques MDM** : sur macOS, livrez un plist dont les clés de niveau supérieur sont les clés de paramètres. Sur Windows, stockez l'ensemble du document JSON en tant que chaîne dans une valeur de registre. Le domaine plist et la clé de registre se trouvent dans [Où chaque mécanisme stocke la politique](/docs/fr/managed-settings#where-each-mechanism-stores-the-policy).
* **Fichier de paramètres gérés** : placez un `managed-settings.json` au chemin système de la plateforme. Vous pouvez également ajouter des fichiers au répertoire drop-in `managed-settings.d/` à côté de celui-ci. Les chemins de fichier par plateforme se trouvent dans [Où chaque mécanisme stocke la politique](/docs/fr/managed-settings#where-each-mechanism-stores-the-policy), et les règles de fusion drop-in se trouvent dans [Diviser une politique basée sur fichier entre les équipes](/docs/fr/managed-settings#split-a-file-based-policy-across-teams).

Utilisez les paramètres gérés par le serveur si vous avez une organisation Claude for Teams ou Enterprise sur claude.ai et que vos appareils ne sont pas tous sous MDM. Sinon, utilisez une politique MDM ou le fichier de paramètres gérés. Pour le compromis, voir [Choisir entre les paramètres gérés par le serveur et gérés par le point de terminaison](/docs/fr/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings).

<h4 id="which-managed-source-applies-on-a-machine">
  Quelle source gérée s'applique sur une machine
</h4>

Par défaut, une seule de ces trois sources s'applique sur une machine. Claude Code utilise la première qui livre une clé de politique, en vérifiant d'abord les paramètres gérés par le serveur, puis les politiques MDM, puis le fichier de paramètres gérés. Si les paramètres gérés par le serveur livrent même une seule clé non liée, Claude Code ignore les clés de plugin dans une politique MDM ou un fichier de paramètres gérés sur cette machine, à l'exception des [clés qu'il lit à partir de chaque source](/docs/fr/managed-settings#keys-read-from-every-admin-source).

Pour appliquer chaque source à la place, définissez [`managedSourcesBehavior`](/docs/fr/managed-settings#compose-every-managed-source) sur `"merge"`.

[Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) liste également les clés que Claude Code lit à partir de chaque source dans les deux modes.

<h3 id="require-a-marketplace-and-its-plugins">
  Exiger un marketplace et ses plugins
</h3>

Ajoutez le marketplace sous `extraKnownMarketplaces`, indexé par le `name` propre du marketplace à partir de son `marketplace.json`. Ensuite, ajoutez chaque plugin sous `enabledPlugins` en tant que `plugin-name@marketplace-name`. Chaque entrée de marketplace porte un objet `source` avec un champ `source` nommant le type, tel que `github`. Cet exemple de paramètres gérés enregistre un marketplace d'organisation et force-active deux plugins à partir de celui-ci :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

Une fois que les paramètres atteignent une machine, Claude Code enregistre le marketplace et installe les deux plugins au début de la session suivante de l'utilisateur. Les utilisateurs les voient dans `/plugin`, et désactiver l'un à leur propre portée ne l'empêche pas de se charger, car les paramètres gérés ont la priorité sur chaque autre portée.

Pour bloquer un plugin à chaque portée et le masquer de la liste du marketplace, définissez-le sur `false` dans le `enabledPlugins` géré à la place.

Ajustez les champs `autoUpdate` et `source` pour votre marketplace :

* **`autoUpdate`** : `true` garde le marketplace et ses plugins en actualisation en arrière-plan, et `false` désactive cela. Voir [Définir la politique de mise à jour](#set-update-policy).
* **`source`** : `github` est l'un de plusieurs types de sources. Une source `git` prend une `url` pour GitLab ou un hôte interne, et une source `url` prend l'adresse d'un `marketplace.json` hébergé. Chaque forme de source se trouve dans la [référence du marketplace](/docs/fr/plugins/marketplace-reference).

Si le marketplace est un référentiel git privé, chaque utilisateur a besoin d'un accès en lecture à celui-ci. Le clone d'un marketplace basé sur git s'exécute avec git sur la machine de l'utilisateur, en utilisant les identifiants stockés et sans invites. Pour les utilisateurs sans comptes d'hôte git, utilisez un [seed](#seed-containers-and-ci) à la place.

Une entrée gérée remplace également une entrée de marketplace de même nom ou une copie `--plugin-dir` d'une autre source :

* **Marketplaces** : une entrée de marketplace gérée remplace une entrée de priorité inférieure du même nom, et les champs des deux entrées ne fusionnent pas.
* **Copies `--plugin-dir`** : `--plugin-dir` charge un plugin à partir d'un répertoire local pour une session. Pour ce qui se passe quand le nom de cette copie correspond à un plugin que votre `enabledPlugins` géré nomme, voir [Conflits de noms](/docs/fr/plugins/loading#name-conflicts).

Le marketplace officiel d'Anthropic `claude-plugins-official` n'a besoin d'aucune entrée `extraKnownMarketplaces` quand `enabledPlugins` définit l'un de ses plugins sur `true`. Cette entrée `name@claude-plugins-official` déclare le marketplace par elle-même, partout où ces clés s'appliquent. Si vous n'activez aucun de ses plugins et souhaitez toujours qu'il soit enregistré sur chaque machine, donnez-lui une entrée explicite, comme [Autoriser le marketplace officiel et le vôtre](#allow-the-official-marketplace-and-your-own) le fait.

<h3 id="require-plugins-per-repository">
  Exiger des plugins par référentiel
</h3>

Pour couvrir les contributeurs d'un seul référentiel au lieu de votre flotte entière, définissez `extraKnownMarketplaces` et `enabledPlugins` dans le `.claude/settings.json` de ce référentiel. Les entrées `extraKnownMarketplaces` s'appliquent uniquement dans un dossier que le contributeur a approuvé, et dans un dossier non approuvé Claude Code les ignore sans message :

* **Sessions interactives** : Claude Code enregistre le marketplace uniquement après que le contributeur accepte la [boîte de dialogue de confiance de l'espace de travail](/docs/fr/permissions#what-runs-before-you-trust-a-folder) pour ce dossier.
* **[Exécutions non interactives `-p`](/docs/fr/headless)** : les entrées s'appliquent uniquement dans un dossier dont la confiance que l'utilisateur a déjà acceptée de manière interactive, ou dont vous définissez l'indicateur `hasTrustDialogAccepted` dans `~/.claude.json`.

Un plugin que le marketplace liste par un chemin relatif se charge à partir de la copie du marketplace une fois que les entrées `extraKnownMarketplaces` du référentiel s'appliquent. Un plugin dont l'entrée de marketplace pointe vers une source externe à la place, comme le référentiel GitHub propre du plugin, ne s'installe pas à partir des paramètres du référentiel seuls. Chaque contributeur voit `Plugin "<name>" is enabled in project settings but isn't installed` jusqu'à ce qu'il exécute `claude plugin install <name>@<marketplace> --scope project`, comme [Installer des plugins](/docs/fr/plugins/install) le décrit.

Si vous utilisez une source `directory` ou `file` locale avec un chemin relatif, le chemin se résout par rapport au checkout principal de votre référentiel. Quand vous exécutez Claude Code à partir d'une git worktree, le chemin pointe toujours vers le checkout principal, donc tous les worktrees partagent le même emplacement de marketplace.

Pour déployer un ensemble de plugins avec des dépendances, mettez le plugin d'ensemble dans `enabledPlugins`, comme [Dépendances des plugins](/docs/fr/plugins/dependencies) le décrit.

<h3 id="when-each-surface-applies-the-plugin-keys">
  Quand chaque surface applique les clés de plugin
</h3>

Le tableau montre quand chaque type de session Claude Code applique `extraKnownMarketplaces` et `enabledPlugins`, à partir des paramètres gérés et du `.claude/settings.json` d'un référentiel. Pour l'application Desktop et les extensions IDE, voir [Installer un plugin](/docs/fr/plugins/install#install-a-plugin).

| Surface              | `extraKnownMarketplaces` et `enabledPlugins` gérés                                                                                                                                                                                                                                                                                                                                                | `.claude/settings.json` du référentiel                                                                      |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------- |
| Terminal, interactif | Appliqué au démarrage de la session sur chaque machine qui reçoit les paramètres                                                                                                                                                                                                                                                                                                                  | `extraKnownMarketplaces` appliqué après la confiance ; `enabledPlugins` appliqué au démarrage de la session |
| `-p` et IC           | Appliqué au démarrage de la session, avec les installations s'exécutant en arrière-plan                                                                                                                                                                                                                                                                                                           | `extraKnownMarketplaces` dans les dossiers approuvés uniquement ; `enabledPlugins` appliqué                 |
| Sessions cloud       | Dans un environnement hébergé par Anthropic, seuls les paramètres gérés par le serveur atteignent la session, qui les attend avant d'installer les plugins. Les politiques MDM et les fichiers de paramètres gérés restent sur la machine de l'utilisateur. Pour un environnement auto-hébergé, voir [Où et quand une politique s'applique](/docs/fr/managed-settings#where-and-when-a-policy-applies) | Voir l'onglet **Session cloud** sous [Installer un plugin](/docs/fr/plugins/install#install-a-plugin)            |

Dans une exécution `-p` ou IC, les marketplaces et les plugins s'installent en arrière-plan, donc un plugin peut manquer du premier tour. Définissez `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1` pour faire attendre l'exécution à l'installation avant sa première requête.

<h3 id="confirm-the-rollout">
  Confirmer le déploiement
</h3>

Vérifiez que le marketplace et les plugins sont arrivés sur une machine ou dans une exécution IC :

* **Sur une machine** : démarrez Claude Code et exécutez `/plugin`. Le marketplace et les plugins sont listés.
* **En IC** : exécutez `claude -p` avec `--output-format stream-json --verbose`. L'événement `init` liste les plugins chargés sous `plugins`.

<h2 id="seed-containers-and-ci">
  Ensemencer les conteneurs et l'IC
</h2>

Pour les images de conteneur et les exécuteurs IC qui ne peuvent pas cloner au moment de l'exécution, pré-remplissez un répertoire de plugins au moment de la construction et pointez `CLAUDE_CODE_PLUGIN_SEED_DIR` vers celui-ci. Claude Code enregistre les marketplaces du seed au démarrage et charge les caches de plugins à partir du seed en place, sans cloner.

Un seed sert également les utilisateurs qui n'ont pas de compte d'hôte git.

<Note>
  Dans les environnements IC/CD, configurez un assistant d'identifiants git avant d'installer des plugins à partir de référentiels privés. Sur GitHub Actions, exportez un jeton avec accès en lecture au référentiel du marketplace en tant que `GH_TOKEN`, puis exécutez `gh auth setup-git`. Le jeton de flux de travail par défaut ne peut accéder qu'au référentiel du flux de travail lui-même, donc un marketplace privé dans un autre référentiel a besoin d'un jeton d'accès personnel ou d'un jeton d'application.
</Note>

<Steps>
  <Step title="Installer dans le seed au moment de la construction">
    Définissez `CLAUDE_CODE_PLUGIN_CACHE_DIR` sur le chemin du seed pour que le marketplace et les plugins s'installent là à la place de `~/.claude/plugins` :

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    Le seed a la même disposition que `~/.claude/plugins` : `known_marketplaces.json`, `marketplaces/<name>/`, et `cache/<marketplace>/<plugin>/<version>/`. Vous pouvez monter le seed à un chemin différent de celui où vous l'avez construit.
  </Step>

  <Step title="Pointer l'exécution vers le seed">
    Définissez `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` dans l'environnement du conteneur. Pour utiliser plusieurs seeds, séparez leurs chemins avec `:` sur Unix ou `;` sur Windows. Claude Code utilise le premier seed qui contient un marketplace ou un cache de plugin donné.
  </Step>

  <Step title="Activer les plugins">
    Les plugins dans un seed ne sont pas activés d'eux-mêmes. Définissez `enabledPlugins` pour chaque plugin de seed que vous souhaitez charger, dans les paramètres gérés ou dans le `.claude/settings.json` du référentiel.
  </Step>
</Steps>

Pour vérifier un seed, exécutez `claude -p` avec `--output-format stream-json --verbose` dans l'image. Dans la liste `plugins` de l'événement `init`, le `path` de chaque plugin chargé se trouve sous le seed, tel que `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`.

Les marketplaces de seed suivent ces règles :

* **Lecture seule** : Claude Code n'écrit jamais dans le seed et force `autoUpdate` à off pour les marketplaces de seed.
* **Les entrées de seed ont la priorité** : à chaque démarrage, un marketplace déclaré dans le seed remplace l'entrée de l'utilisateur du même nom. Les utilisateurs se désabonnent d'un plugin de seed avec `claude plugin disable`, pas en supprimant le marketplace.
* **La mise à jour et la suppression échouent** : `claude plugin marketplace update <name>` et `remove` sans `--scope` sur un marketplace de seed échouent avec un message qui nomme le répertoire du seed.
* **La politique s'applique toujours** : la [liste blanche et la liste noire](#restrict-what-users-can-install) vérifient également la source enregistrée d'un marketplace de seed. Autorisez la source à partir de laquelle vous avez construit le seed.

Pour les flottes sans accès git sortant, combinez un seed avec des sources de marketplace `directory` ou `file` sur un montage partagé. Définissez également `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, qui désactive également la [mise à jour automatique des plugins](/docs/fr/plugins/loading#when-auto-update-runs). Si un proxy est disponible, voir [Configuration du proxy](/docs/fr/network-config#proxy-configuration) pour les variables à définir.

<h2 id="restrict-what-users-can-install">
  Restreindre ce que les utilisateurs peuvent installer
</h2>

La liste blanche gérée `strictKnownMarketplaces` et la liste noire `blockedMarketplaces` décident quelles sources de marketplace les plugins peuvent provenir. La source d'un marketplace est le référentiel git, l'URL ou le chemin local que Claude Code récupère. Les deux listes correspondent à la source du marketplace d'où provient un plugin, pas à l'entrée propre du plugin à l'intérieur de ce marketplace.

Pour le verrouillage courant, qui autorise le marketplace officiel et le vôtre, voir [Autoriser le marketplace officiel et le vôtre](#allow-the-official-marketplace-and-your-own). Associez-le à [`disableSideloadFlags`](#control-matrix) pour que les utilisateurs ne puissent pas charger les plugins à partir d'un répertoire local ou d'une URL non plus.

Les deux listes s'appliquent avant tout téléchargement et à nouveau au démarrage de la session :

* **Avant un téléchargement** : les listes s'appliquent quand un utilisateur ajoute un marketplace et à chaque installation, mise à jour, actualisation et mise à jour automatique.
* **Au démarrage de la session** : les listes s'appliquent à nouveau aux plugins déjà installés, donc un plugin installé dont la source du marketplace ne correspond plus ne se charge pas. `/plugin` le liste avec `Marketplace "<name>" is not in the allowed marketplace list` ou `Marketplace "<name>" is blocked by enterprise policy`.

L'endroit où les deux listes sont appliquées dépend de l'endroit où vous les définissez :

* **La console d'administration claude.ai** : Claude Code applique les deux listes dans les sessions qui [lisent les paramètres gérés par le serveur](/docs/fr/managed-settings#where-and-when-a-policy-applies). claude.ai les vérifie également quand quelqu'un dans votre organisation ajoute un nouveau marketplace à partir d'un référentiel git sur claude.ai, ou à partir de **Personnaliser** dans l'application Claude Desktop en dehors de son onglet Code. Cela couvre un marketplace qu'un membre ajoute pour son propre compte et un ajouté pour toute l'organisation sous [**Paramètres de l'organisation > Plugins**](https://claude.ai/admin-settings/plugins). claude.ai refuse un référentiel que la liste blanche n'admet pas ou que la liste noire nomme. Il ne re-vérifie pas un marketplace qui a été ajouté dans l'un ou l'autre endroit avant que vous définissiez les listes, et il ne vérifie pas les plugins téléchargés.
* **Un fichier de paramètres gérés, une politique au niveau du système d'exploitation ou une autre source gérée** : Claude Code applique les deux listes où il lit cette source. claude.ai ne la lit pas.

Tant qu'une liste blanche est définie, ou qu'une liste noire nomme une source autre que [`skills-dir`](#blocklist-with-blockedmarketplaces), un plugin dont Claude Code ne peut pas trouver le marketplace ne se charge pas. `/plugin` affiche l'erreur de politique pour celui-ci plutôt qu'une erreur de non-trouvé. Le cas courant est une entrée `enabledPlugins` obsolète pour un marketplace que personne n'a enregistré.

<h3 id="control-matrix">
  Matrice de contrôle
</h3>

Le tableau liste chaque clé de politique de plugin, ce qu'elle applique et ce qu'elle ne peut pas faire.

| Clé                                                                      | Ce qu'elle applique                                                                                                                                                                                                                                                                                                                          | Ce qu'elle ne peut pas faire                                                                                                                                                                                                                                  |
| :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `strictKnownMarketplaces`                                                | Liste blanche des sources de marketplace. `[]` bloque chaque source, y compris le marketplace officiel. Alias : `allowedMarketplaces`                                                                                                                                                                                                        | N'enregistre pas un marketplace, ne restreint pas les entrées à l'intérieur d'un marketplace autorisé, ou ne bloque pas `--plugin-dir`                                                                                                                        |
| `blockedMarketplaces`                                                    | Liste noire des sources de marketplace, vérifiée avant la liste blanche                                                                                                                                                                                                                                                                      | Ne bloque pas un marketplace déjà enregistré à partir d'une source qu'il ne correspond pas                                                                                                                                                                    |
| `syncClaudeAiPlugins`                                                    | Définissez `false` pour arrêter Claude Code de télécharger et charger les plugins [synchronisés à partir de claude.ai](/docs/fr/plugins/loading#synced-plugins) pour le compte de chaque utilisateur. Nécessite Claude Code v2.1.273 ou ultérieur                                                                                                 | N'éteint pas un plugin synchronisé. Pour cela, définissez `"<name>@synced": false` dans [`enabledPlugins`](/docs/fr/settings-reference#enabledplugins)                                                                                                             |
| `enabledPlugins`                                                         | `true` force-active, `false` bloque à chaque portée et masque le plugin                                                                                                                                                                                                                                                                      | N'installe pas un plugin dont le marketplace n'est pas enregistré ou autorisé                                                                                                                                                                                 |
| `disableSideloadFlags`                                                   | Rejette `--plugin-dir`, `--plugin-url`, `--agents`, l'option `plugins` du SDK Agent, et `--mcp-config` non-SDK au démarrage, et rejette les dossiers nommés dans la variable [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/fr/env-vars#variables) de la même manière                                                                                          | Ne restreint pas `.mcp.json`, `claude mcp add`, ou les serveurs fournis par SDK. Associez-le à [`allowedMcpServers`](/docs/fr/managed-mcp)                                                                                                                         |
| `disableCommandPluginSources`                                            | Bloque les plugins avec une source `command` de l'installation, de la mise à jour ou du chargement. Une source `command` est celle dont le répertoire de plugins est produit en exécutant une commande sur la machine. Quand non défini, il prend la valeur de `allowManagedHooksOnly`                                                       | N'affecte pas les autres types de sources                                                                                                                                                                                                                     |
| `allowManagedHooksOnly`                                                  | Restreint les hooks qui s'exécutent. Voir [`allowManagedHooksOnly`](/docs/fr/settings-reference#allowmanagedhooksonly)                                                                                                                                                                                                                            | Ne fait confiance pas aux hooks des plugins que les utilisateurs activent eux-mêmes                                                                                                                                                                           |
| `strictPluginOnlyCustomization`                                          | Bloque les compétences, les agents, les hooks et les serveurs MCP qui ne proviennent pas d'un plugin, des paramètres gérés ou des éléments intégrés de Claude Code. Définissez `true` pour couvrir les quatre types, ou un tableau de valeurs `skills`, `agents`, `hooks` et `mcp` telles que `["skills", "hooks"]` pour en couvrir certains | Ne restreint pas les plugins que les utilisateurs installent. Associez-le à `strictKnownMarketplaces`                                                                                                                                                         |
| `pluginSuggestionMarketplaces`                                           | Marketplaces dont les plugins peuvent apparaître comme suggestions d'installation. Voir [Recommander des plugins](#recommend-plugins)                                                                                                                                                                                                        | N'affecte pas les conseils intégrés                                                                                                                                                                                                                           |
| `pluginTrustMessage`                                                     | Ajoute votre texte à l'avertissement de confiance que `/plugin` affiche avant l'installation d'un plugin                                                                                                                                                                                                                                     | Ne change pas le texte de l'avertissement lui-même                                                                                                                                                                                                            |
| `allowedChannelPlugins`                                                  | Remplace la liste par défaut des plugins autorisés à envoyer des messages de canal. Nécessite `channelsEnabled: true`                                                                                                                                                                                                                        | Voir [Restreindre les plugins de canal qui peuvent s'exécuter](/docs/fr/channels#restrict-which-channel-plugins-can-run)                                                                                                                                           |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/fr/env-vars) | Arrête les sessions de terminal interactives de l'auto-enregistrement du marketplace officiel                                                                                                                                                                                                                                                | Ne supprime pas un marketplace déjà enregistré. La liste blanche et la liste noire contrôlent le même auto-enregistrement sans celui-ci. Une machine qui a démarré une fois avec celui-ci défini ne reprend pas l'auto-enregistrement après l'avoir désactivé |

Chaque clé du tableau est un paramètre géré, à l'exception de `enabledPlugins`, `syncClaudeAiPlugins` et `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` :

* **`enabledPlugins`** : vous pouvez le définir dans n'importe quelle portée, et les paramètres gérés le verrouillent.
* **`syncClaudeAiPlugins`** : chaque utilisateur peut également le définir dans ses propres paramètres utilisateur ou locaux. Voir sa [portée dans la référence des paramètres](/docs/fr/settings-reference#syncclaudeaiplugins).
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`** : c'est une variable d'environnement que vous livrez via le bloc `env` géré montré sous [Désactiver les mises à jour pour toute la flotte](#turn-updates-off-for-the-whole-fleet).

Chaque clé de paramètres ici a une entrée dans la [référence des paramètres](/docs/fr/settings-reference).

<h4 id="aliases-for-the-marketplace-keys">
  Alias pour les clés du marketplace
</h4>

`strictKnownMarketplaces` peut également être orthographié `allowedMarketplaces`, et `extraKnownMarketplaces` peut également être orthographié `additionalMarketplaces`.

* **Version** : les alias nécessitent Claude Code v2.1.232 ou ultérieur, et les clients plus anciens les ignorent. Dans un fichier qu'une flotte mixte lit, gardez les noms canoniques.
* **Les deux orthographes définies** : quand un fichier définit les deux orthographes, la valeur de la clé canonique s'applique.

<h3 id="allowlist-with-strictknownmarketplaces">
  Liste blanche avec `strictKnownMarketplaces`
</h3>

Définissez la liste blanche sur une liste de ces objets de source. La plupart des entrées correspondent exactement, les entrées `hostPattern` et `pathPattern` correspondent en tant qu'expressions régulières, et les caractères génériques de propriétaire `github` correspondent par propriétaire :

* **`github`** : `{ "source": "github", "repo": "your-org/approved-plugins" }`, avec `ref` et `path` optionnels.
* **Caractère générique de propriétaire `github`** : `{ "source": "github", "repo": "your-org/*" }` correspond à chaque référentiel sous ce propriétaire. Le `*` doit représenter le nom de référentiel entier. Claude Code ignore les entrées telles que `*/plugins` et `your-org/tools-*` comme invalides, donc elles ne correspondent à rien. Nécessite Claude Code v2.1.223 ou ultérieur.
* **`git`** : `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, avec `ref` et `path` optionnels.
* **`url`** : `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, avec `headers` optionnels.
* **`file` et `directory`** : `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` ou `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, avec des chemins absolus.
* **`hostPattern`** : `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, comparé à l'hôte des sources `github`, `git` et `url`. Le motif correspond n'importe où dans le nom d'hôte, donc ancrez-le avec `^` et `$` comme montré pour correspondre à l'hôte entier. Une source `github` compte toujours comme `github.com`. Utilisez une entrée `hostPattern` pour un serveur GitHub Enterprise Server ou un hôte GitLab où les développeurs créent leurs propres marketplaces. La [page GHES](/docs/fr/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) a l'exemple travaillé.
* **`pathPattern`** : `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, comparé au `path` des sources `file` et `directory`. Le motif correspond n'importe où dans le chemin, donc commencez-le par `^` pour épingler un préfixe de répertoire. `".*"` autorise chaque chemin local.
* **`skills-dir`** : `{ "source": "skills-dir" }` garde les [plugins du répertoire de compétences](#keep-skills-directory-plugins-loading) en chargement tandis qu'une liste blanche est définie, et ne correspond à aucun marketplace.

<h4 id="how-entries-match">
  Comment les entrées correspondent
</h4>

Une entrée `url` correspond sur sa valeur `url` ; `headers` ne sont pas comparés. Pour les entrées `github` et `git`, le `repo` ou `url`, le `ref` et le `path` doivent tous correspondre, ou être absents des deux côtés :

* Une entrée sans `ref` ne couvre pas une source avec `ref: "main"`.
* Une entrée pour `your-org/your-marketplace` ne couvre pas une URL `git` qui clone le même référentiel.
* Une barre oblique finale, un suffixe `.git` ou `ssh://` à la place de `https://` est une valeur différente. Quand un marketplace peut être cloné par plus d'une URL, préférez une entrée `hostPattern`.

Les entrées de caractère générique de propriétaire suivent les règles exactes pour `ref` et correspondent à n'importe quel `path` à l'intérieur du référentiel à moins que l'entrée n'en épingle un. La correspondance de caractère générique est sensible à la casse sur la liste blanche.

<h4 id="keep-skills-directory-plugins-loading">
  Garder les plugins du répertoire de compétences en chargement
</h4>

Les plugins du répertoire de compétences sont les plugins que les utilisateurs gardent sous `~/.claude/skills/` ou un `.claude/skills/` du projet dans des dossiers qui portent un `.claude-plugin/plugin.json`. Si vous définissez une liste blanche sans une entrée `{ "source": "skills-dir" }`, ils arrêtent de se charger. Les [compétences](/docs/fr/skills) simples, c'est-à-dire un `SKILL.md` sans ce manifeste, continuent de se charger.

<h4 id="marketplaces-hosted-on-claude-ai">
  Marketplaces hébergés sur claude.ai
</h4>

La liste blanche et la liste noire correspondent à un [marketplace hébergé sur claude.ai](/docs/fr/plugins/install#add-from-claude-ai) par son hôte. Pour en autoriser ou en bloquer un, ajoutez une entrée `hostPattern` qui correspond à `claude.ai` à `strictKnownMarketplaces` ou `blockedMarketplaces`. Sur la liste blanche, une telle entrée admet vos marketplaces claude.ai de l'organisation et les marketplaces par défaut de claude.ai, mais pas un marketplace composé des téléchargements claude.ai propres d'un membre ou dont la portée claude.ai n'a pas été déclarée. Nécessite Claude Code v2.1.273 ou ultérieur.

<h4 id="lock-every-source-out">
  Verrouiller chaque source
</h4>

Une liste blanche vide, `[]`, verrouille chaque source de marketplace, y compris le marketplace officiel.

Ce verrouillage ne couvre pas les plugins [synchronisés à partir de claude.ai](/docs/fr/plugins/loading#synced-plugins), que Claude Code télécharge à partir du compte de chaque utilisateur plutôt qu'à partir d'un marketplace. Pour arrêter ceux-ci aussi, définissez [`syncClaudeAiPlugins`](/docs/fr/settings-reference#syncclaudeaiplugins) sur `false` dans les paramètres gérés, ou désactivez les compétences pour votre organisation sur claude.ai.

<h3 id="blocklist-with-blockedmarketplaces">
  Liste noire avec `blockedMarketplaces`
</h3>

`blockedMarketplaces` prend les mêmes objets de source que [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces) et est vérifiée en premier, donc une source sur les deux listes est bloquée. La correspondance de liste noire est plus large que la correspondance de liste blanche :

* Les URL git sont canonicalisées, donc les formes `git@` et `https://`, les suffixes `.git` et les barres obliques finales d'un référentiel `github.com` correspondent tous à la même entrée.
* Une entrée `github` bloque également l'URL `git` équivalente, et vice versa.
* Pour une entrée `owner/*`, la comparaison du propriétaire est insensible à la casse.
* Une entrée sans `ref` ou `path` bloque chaque ref et chemin des référentiels qu'elle correspond.

Cette entrée bloque chaque référentiel sous un propriétaire GitHub :

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

Les entrées `url` dans `blockedMarketplaces` s'appliquent également quand un utilisateur ajoute une URL de référentiel `https://` que Claude Code [clone plutôt que récupère](/docs/fr/plugins/cli-reference#plugin-marketplace-add), comme une URL de référentiel `github.com` ou `gitlab.com` nue. L'utilisateur ne peut pas ajouter cette URL si une entrée la nomme. La correspondance ignore le suffixe `.git` et tout ref que l'utilisateur ajoute après `#`. Nécessite Claude Code v2.1.232 ou ultérieur.

Une entrée `{ "source": "skills-dir" }` ici arrête les [plugins du répertoire de compétences](#keep-skills-directory-plugins-loading) de se charger, à partir de `~/.claude/skills/` et du `.claude/skills/` d'un projet.

Une liste noire qui nomme uniquement cette entrée ne compte pas comme une restriction active, donc elle ne [arrête pas les plugins dont Claude Code ne peut pas trouver le marketplace](#restrict-what-users-can-install) de se charger.

<h3 id="allow-the-official-marketplace-and-your-own">
  Autoriser le marketplace officiel et le vôtre
</h3>

La plupart des organisations autorisent le marketplace officiel et le leur, et enregistrent les deux pour que chaque machine les ait. Cette politique de paramètres gérés autorise les deux marketplaces, enregistre les deux, force-active deux plugins et rejette `--plugin-dir` :

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

Sur une machine avec cette politique, ajouter une source en dehors de la liste, par exemple `/plugin marketplace add https://example.com/other-marketplace.git`, échoue avec un message contenant `is blocked by enterprise policy` suivi des sources autorisées. `claude --plugin-dir ./x` se termine avec un message nommant `disableSideloadFlags`.

L'entrée `{ "source": "skills-dir" }` garde les [plugins du répertoire de compétences](#keep-skills-directory-plugins-loading) en chargement sous cette liste blanche. Supprimez cette entrée et ils arrêtent de se charger.

Enregistrez les deux marketplaces avec des entrées `extraKnownMarketplaces` explicites, comme cette politique le fait, plutôt que de compter sur la liste blanche ou sur l'auto-enregistrement du marketplace officiel :

* **La liste blanche n'enregistre rien** : une entrée `extraKnownMarketplaces` le fait, et elle doit elle-même passer la liste blanche. Claude Code refuse d'enregistrer un marketplace géré dont la source ne correspond pas à la liste blanche.
* **Le marketplace officiel ne s'enregistre que dans une session de terminal interactif** : même là, il ne s'enregistre que quand la liste blanche le permet. Une exécution `-p` ou un terminal attaché à une session cloud ne l'enregistre jamais.
* **Une tentative bloquée est mémorisée** : si une machine a jamais fonctionné sous une politique qui bloquait le marketplace officiel, Claude Code enregistre la tentative bloquée et ne réessaie pas après le changement de politique. Un verrouillage `[]` est une telle politique. Cette machine l'enregistre à nouveau uniquement via une entrée `extraKnownMarketplaces` comme celle de cette politique, une entrée `enabledPlugins` pour l'un de ses plugins, ou un `/plugin marketplace add` manuel.

<h2 id="set-update-policy">
  Définir la politique de mise à jour
</h2>

Vous pouvez définir la politique de mise à jour par marketplace, pour toute la flotte, ou par groupe d'utilisateurs via les canaux de version.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  Activer ou désactiver la mise à jour automatique par marketplace
</h3>

La mise à jour automatique des plugins s'exécute en arrière-plan après le démarrage pour les marketplaces qui l'ont activée. Pour savoir quels marketplaces l'ont activée par défaut, voir [Quand la mise à jour automatique s'exécute](/docs/fr/plugins/loading#when-auto-update-runs). Pour décider pour la flotte, définissez `"autoUpdate": true` ou `false` sur une entrée `extraKnownMarketplaces` gérée :

* Si l'entrée gérée définit le champ, Claude Code rejette le basculement `/plugin` de l'utilisateur avec une erreur qui commence par `Auto-update for '<name>' is set by`.
* Si l'entrée gérée laisse le champ non défini, le basculement de l'utilisateur persiste.

<h3 id="turn-updates-off-for-the-whole-fleet">
  Désactiver les mises à jour pour toute la flotte
</h3>

Pour désactiver la mise à jour automatique des plugins pour chaque marketplace, définissez `DISABLE_AUTOUPDATER` dans le bloc `env` géré, comme cet exemple le fait. La même variable arrête également les mises à jour de Claude Code lui-même :

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Pour arrêter les mises à jour de Claude Code lui-même mais garder la mise à jour automatique des plugins, ajoutez `"FORCE_AUTOUPDATE_PLUGINS": "1"` au même bloc. Les autres [variables d'environnement qui arrêtent la mise à jour automatique des plugins](/docs/fr/plugins/loading#when-auto-update-runs) fonctionnent de la même manière.

`DISABLE_AUTOUPDATER` ne couvre pas les plugins avec une [source `command`](/docs/fr/plugins/marketplace-reference#command-plugin-source). Claude Code réexécute la commande de chaque plugin activé à chaque session et installe la sortie quand elle a changé. Pour ce qui arrête ces exécutions, voir [Quand une source de commande réexécute](/docs/fr/plugins/loading#when-a-command-source-re-runs).

<h3 id="assign-release-channels-to-user-groups">
  Assigner les canaux de version aux groupes d'utilisateurs
</h3>

Pour exécuter des canaux stables et d'accès anticipé, hébergez deux marketplaces qui pointent vers différents refs des mêmes plugins. Ensuite, donnez à chaque groupe d'utilisateurs son propre marketplace via soit des paramètres gérés par le point de terminaison séparés, soit une politique de passerelle. Les paramètres gérés par le serveur de la console d'administration [s'appliquent à chaque utilisateur de votre organisation](/docs/fr/server-managed-settings#current-limitations), donc ils ne peuvent pas assigner des paramètres différents à différents groupes.

* Déployez des [paramètres gérés par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) séparés, tels qu'un fichier de paramètres gérés ou un profil MDM, sur les appareils de chaque groupe. Pour vérifier si le fichier ou le profil par groupe s'applique sur un appareil qui a également une source au niveau de l'organisation, voir [Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#precedence-within-the-managed-tier).
* Définissez une [politique de passerelle d'applications Claude](/docs/fr/claude-apps-gateway-config#managed) par groupe. La passerelle applique la première politique dont la règle de correspondance correspond à un utilisateur, donc ordonnez les politiques pour que chaque utilisateur atteigne la politique de son groupe. La `extraKnownMarketplaces` de cette politique ne fusionne pas avec celle d'une autre politique, donc listez chaque marketplace dont le groupe a besoin, pas seulement son marketplace de canal.

Avec l'un ou l'autre mécanisme, le groupe stable reçoit cette configuration :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

Le groupe d'accès anticipé reçoit `latest-tools` à la place. Pour configurer les deux marketplaces, voir [Exécuter les canaux de version](/docs/fr/plugins/host-marketplace#run-release-channels).

<h2 id="recommend-plugins">
  Recommander des plugins
</h2>

Les propriétaires de marketplace peuvent joindre des signaux `relevance` aux entrées pour que Claude Code suggère le plugin quand un projet correspond.

Les suggestions d'un marketplace n'apparaissent que quand il est enregistré sur la machine de l'utilisateur, vous listez son nom dans `pluginSuggestionMarketplaces` dans les paramètres gérés, et vous déclarez sa source dans la même politique. Déclarez la source soit comme l'entrée `extraKnownMarketplaces` du marketplace, soit comme une entrée de liste blanche. Le marketplace officiel a besoin uniquement du nom. Voir [Activer les suggestions dans les paramètres gérés](/docs/fr/plugins/relevance#enable-suggestions-in-managed-settings).

<h2 id="audit-and-review">
  Auditer et examiner
</h2>

Les événements OpenTelemetry et l'API Analytics vous disent ce que votre flotte installe et exécute.

Pour ce qu'un plugin peut exécuter sur une machine et ce que chaque niveau de confiance permet, lisez [Sécurité des plugins](/docs/fr/plugins/security) avant d'approuver un marketplace.

<h3 id="opentelemetry-events">
  Événements OpenTelemetry
</h3>

`claude_code.plugin_installed` enregistre chaque installation, et `claude_code.plugin_loaded` enregistre chaque plugin activé au démarrage de la session. Les deux événements masquent ou omettent les noms de plugins et de marketplaces tiers à moins que vous définissiez `OTEL_LOG_TOOL_DETAILS=1`, comme [Noms de plugins masqués dans votre backend](/docs/fr/plugins/measure#redacted-plugin-names-in-your-backend) le montre. Les listes de champs se trouvent sous [Événement de plugin installé](/docs/fr/monitoring-usage#plugin-installed-event) et [Événement de plugin chargé](/docs/fr/monitoring-usage#plugin-loaded-event).

<h3 id="analytics-api">
  API Analytics
</h3>

Sur le plan Enterprise, `GET /v1/organizations/analytics/plugins` retourne les comptes d'installation et d'invocation par plugin, par jour, sur Claude Code et Cowork. Vous pouvez grouper les comptes par utilisateur ou groupe RBAC. L'activité de plugin qui atteint Anthropic sans un nom de plugin apparaît dans une ligne `third-party` agrégée. Voir la [référence du point de terminaison](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) et [Accéder aux données par programmation](/docs/fr/analytics#access-data-programmatically) pour la clé dont elle a besoin.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  Planifier ce que les paramètres gérés ne peuvent pas appliquer
</h2>

Ces demandes des examens de sécurité n'ont pas de clé dédiée dans le schéma de paramètres actuel. Les contrôles existants les plus proches sont :

* **Ciblage par utilisateur ou par groupe** : chaque clé de plugin s'applique à chaque utilisateur qui reçoit les paramètres. Les paramètres gérés par le serveur livrent une configuration par organisation. Pour une politique par groupe, utilisez des paramètres gérés par le point de terminaison séparés ou des politiques de passerelle, comme sous [Assigner les canaux de version aux groupes d'utilisateurs](#assign-release-channels-to-user-groups).
* **Restreindre les entrées à l'intérieur d'un marketplace autorisé** : la liste blanche correspond aux sources de marketplace. Pour bloquer un plugin d'un marketplace autorisé, définissez-le sur `false` dans `enabledPlugins` géré.
* **Masquer `/plugin`** : aucune clé ne désactive la commande. L'équivalent le plus proche combine une liste blanche nommant uniquement votre marketplace, des entrées `enabledPlugins` gérées pour les plugins que vous fournissez, et `disableSideloadFlags`.
* **Contrôler `--plugin-dir` via la liste blanche** : la liste blanche ne couvre pas `--plugin-dir`. `disableSideloadFlags` le fait.
* **Appliquer les bascules de plugin claude.ai via ces clés** : [**Paramètres de l'organisation > Plugins et compétences**](https://claude.ai/admin-settings/skills?tab=inventory) ne définit pas les clés de cette page. Ce que les membres et votre organisation activent là atteint l'interface de ligne de commande sous forme de [plugins synchronisés](/docs/fr/plugins/loading#synced-plugins), qui ont leurs propres contrôles.

<h2 id="troubleshoot-policy">
  Dépanner la politique
</h2>

Si la politique de plugin ne se comporte pas comme prévu sur une machine, vérifiez d'abord ces symptômes :

* **Le fichier géré n'a pas été analysé** : quand un `managed-settings.json` n'est pas un JSON valide, Claude Code refuse de démarrer et imprime [une erreur nommant le fichier](/docs/fr/errors#managed-settings-document-could-not-be-parsed). Un fichier qui s'analyse mais a une entrée invalide garde le reste de sa politique. Voir [Entrées invalides dans les paramètres gérés](/docs/fr/managed-settings#invalid-entries-in-managed-settings).
* **La source gérée n'a pas chargé** : exécutez `/status` et cherchez `Enterprise managed settings` dans la ligne `Setting sources`. S'il manque, la source n'a pas chargé.
* **Un utilisateur signale `blocked by enterprise policy`** : le message nomme le marketplace ou sa source. Pour une liste blanche, il liste également les sources autorisées. Les entrées visibles par l'utilisateur se trouvent sur [Dépanner les plugins](/docs/fr/plugins/troubleshooting).
* **Un plugin que l'utilisateur a désactivé dans `~/.claude/settings.json` se charge toujours** : une autre source de paramètres l'a réactivé, comme une entrée `enabledPlugins` gérée qui le force-active. `/plugin` et `claude plugin list` affichent `Disabled in ~/.claude/settings.json but still loads` avec cette source de paramètres.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Référence du marketplace](/docs/fr/plugins/marketplace-reference#marketplace-sources) : les valeurs `source` que `extraKnownMarketplaces`, `strictKnownMarketplaces` et `blockedMarketplaces` acceptent
* [Héberger et maintenir un marketplace](/docs/fr/plugins/host-marketplace) : exécutez le marketplace vers lequel votre politique pointe
* [Sécurité et confiance des plugins](/docs/fr/plugins/security) : ce qu'un plugin peut faire sur une machine et comment en examiner un avant l'installation
* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) : livrez ces clés à partir de la console d'administration claude.ai
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting#blocked-by-your-organization) : les messages que les utilisateurs voient quand la politique les bloque
