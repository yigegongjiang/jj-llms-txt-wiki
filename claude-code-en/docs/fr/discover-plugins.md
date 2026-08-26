> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Découvrir et installer des plugins prédéfinis via les marketplaces

> Trouvez et installez des plugins depuis les marketplaces pour étendre Claude Code avec de nouvelles compétences, agents et capacités.

Les plugins étendent Claude Code avec des skills, des agents, des hooks et des serveurs MCP. Les marketplaces de plugins sont des catalogues qui vous aident à découvrir et installer ces extensions sans les construire vous-même.

Vous cherchez à créer et distribuer votre propre marketplace ? Consultez [Créer et distribuer une marketplace de plugins](/docs/fr/plugin-marketplaces).

<h2 id="how-marketplaces-work">
  Comment fonctionnent les marketplaces
</h2>

Une marketplace est un catalogue de plugins que quelqu'un d'autre a créé et partagé. L'utilisation d'une marketplace est un processus en deux étapes :

<Steps>
  <Step title="Ajouter la marketplace">
    Cela enregistre le catalogue avec Claude Code pour que vous puissiez parcourir ce qui est disponible. Aucun plugin n'est installé pour le moment.
  </Step>

  <Step title="Installer des plugins individuels">
    Parcourez le catalogue et installez les plugins que vous souhaitez.
  </Step>
</Steps>

Pensez-y comme ajouter un app store : ajouter le store vous donne accès pour parcourir sa collection, mais vous choisissez toujours quelles applications télécharger individuellement.

<h2 id="official-anthropic-marketplace">
  Marketplace officielle Anthropic
</h2>

La marketplace officielle Anthropic (`claude-plugins-official`) est automatiquement disponible quand vous démarrez Claude Code. Exécutez `/plugin` et allez à l'onglet **Discover** pour parcourir ce qui est disponible, ou consultez le catalogue sur [claude.com/plugins](https://claude.com/plugins).

Pour installer un plugin depuis la marketplace officielle, utilisez `/plugin install <name>@claude-plugins-official`. Par exemple, pour installer l'intégration GitHub :

```shell theme={null}
/plugin install github@claude-plugins-official
```

Si Claude Code signale que le plugin n'est pas trouvé dans une marketplace, votre marketplace est soit manquante soit obsolète. Exécutez `/plugin marketplace update claude-plugins-official` pour l'actualiser, ou `/plugin marketplace add anthropics/claude-plugins-official` si vous ne l'avez pas encore ajoutée. Ensuite, réessayez l'installation.

<Note>
  La marketplace officielle est maintenue par Anthropic, et l'inclusion est à la discrétion d'Anthropic. Les formulaires de soumission intégrés à l'application ajoutent des plugins à la [marketplace communautaire](#community-marketplace), pas à la marketplace officielle. Pour distribuer des plugins indépendamment, [créez votre propre marketplace](/docs/fr/plugin-marketplaces) et partagez-la avec les utilisateurs.
</Note>

La marketplace officielle inclut plusieurs catégories de plugins :

<h3 id="code-intelligence">
  Code intelligence
</h3>

Les plugins de code intelligence activent l'outil LSP intégré de Claude Code, donnant à Claude la capacité de sauter aux définitions, trouver les références et voir les erreurs de type immédiatement après les modifications. Ces plugins configurent les connexions [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), la même technologie qui alimente la code intelligence de VS Code.

Ces plugins nécessitent que le binaire du serveur de langage soit installé sur votre système. Si vous avez déjà un serveur de langage installé, Claude peut vous inviter à installer le plugin correspondant quand vous ouvrez un projet.

| Langage    | Plugin              | Binaire requis               |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

Vous pouvez également [créer votre propre plugin LSP](/docs/fr/plugins-reference#lsp-servers) pour d'autres langages.

<Note>
  Si vous voyez `Executable not found in $PATH` dans l'onglet Errors de `/plugin` après avoir installé un plugin, installez le binaire requis du tableau ci-dessus.
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  Ce que Claude gagne des plugins de code intelligence
</h4>

Une fois qu'un plugin de code intelligence est installé et que son binaire de serveur de langage est disponible, Claude gagne deux capacités :

* **Diagnostics automatiques** : après chaque modification de fichier que Claude effectue, le serveur de langage analyse les modifications et signale les erreurs et avertissements automatiquement. Claude voit les erreurs de type, les imports manquants et les problèmes de syntaxe sans avoir besoin d'exécuter un compilateur ou un linter. Si Claude introduit une erreur, il la remarque et la corrige dans le même tour. Cela ne nécessite aucune configuration au-delà de l'installation du plugin. Vous pouvez voir les diagnostics en ligne en appuyant sur **Ctrl+O** quand l'indicateur « diagnostics found » apparaît.
* **Navigation de code** : Claude peut utiliser le serveur de langage pour sauter aux définitions, trouver les références, obtenir les informations de type au survol, lister les symboles, trouver les implémentations et tracer les hiérarchies d'appels. Ces opérations donnent à Claude une navigation plus précise que la recherche basée sur grep, bien que la disponibilité puisse varier selon le langage et l'environnement.

Si vous rencontrez des problèmes, consultez [Dépannage de la code intelligence](#code-intelligence-issues).

<h3 id="external-integrations">
  Intégrations externes
</h3>

Ces plugins regroupent des [serveurs MCP](/docs/fr/mcp) préconfigurés pour que vous puissiez connecter Claude à des services externes sans configuration manuelle :

* **Contrôle de source** : `github`, `gitlab`
* **Gestion de projet** : `atlassian` (Jira/Confluence), `asana`, `linear`, `notion`
* **Design** : `figma`
* **Infrastructure** : `vercel`, `firebase`, `supabase`
* **Communication** : `slack`
* **Monitoring** : `sentry`

<h3 id="automatic-security-review">
  Examen automatique de la sécurité
</h3>

Le plugin `security-guidance` examine chaque modification que Claude effectue pour détecter les vulnérabilités courantes et instruit Claude de corriger ce qu'il trouve dans la même session. Consultez [Détecter les problèmes de sécurité pendant que Claude écrit du code](/docs/fr/security-guidance) pour voir ce qu'il vérifie et comment ajouter des règles spécifiques au projet.

<h3 id="development-workflows">
  Workflows de développement
</h3>

Plugins qui ajoutent des skills et des agents pour les tâches de développement courantes :

* **commit-commands** : Workflows de commit Git incluant commit, push et création de PR
* **pr-review-toolkit** : Agents spécialisés pour examiner les pull requests
* **agent-sdk-dev** : Outils pour construire avec le Claude Agent SDK
* **plugin-dev** : Toolkit pour créer vos propres plugins

<h3 id="output-styles">
  Styles de sortie
</h3>

Personnalisez comment Claude répond :

* **explanatory-output-style** : Insights éducatifs sur les choix d'implémentation
* **learning-output-style** : Mode d'apprentissage interactif pour la construction de compétences

<h2 id="community-marketplace">
  Marketplace communautaire
</h2>

La marketplace communautaire sur [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) héberge des plugins tiers qui ont réussi la validation automatisée d'Anthropic et le contrôle de sécurité. Chaque plugin est épinglé à un SHA de commit spécifique dans le catalogue. Contrairement à la marketplace officielle, vous l'ajoutez manuellement :

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

Ensuite, installez les plugins à partir de celle-ci en utilisant le nom de marketplace `claude-community` :

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

Pour soumettre votre propre plugin à la marketplace communautaire, consultez [Soumettre votre plugin à la marketplace communautaire](/docs/fr/plugins#submit-your-plugin-to-the-community-marketplace) dans le guide de création de plugins.

<h2 id="try-it-add-the-demo-marketplace">
  Essayez : ajouter la marketplace de démonstration
</h2>

Anthropic maintient également une [marketplace de plugins de démonstration](https://github.com/anthropics/claude-code/tree/main/plugins) (`claude-code-plugins`) avec des plugins d'exemple qui montrent ce qui est possible avec le système de plugins. Contrairement à la marketplace officielle, vous devez ajouter celle-ci manuellement.

<Steps>
  <Step title="Ajouter la marketplace">
    Depuis Claude Code, exécutez la commande `plugin marketplace add` pour la marketplace `anthropics/claude-code` :

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    Cela télécharge le catalogue de la marketplace et rend ses plugins disponibles pour vous.
  </Step>

  <Step title="Parcourir les plugins disponibles">
    Exécutez `/plugin` pour ouvrir le gestionnaire de plugins. Cela ouvre une interface à onglets avec quatre onglets que vous pouvez parcourir en utilisant **Tab**, ou **Shift+Tab** pour aller en arrière :

    * **Discover** : parcourez les plugins disponibles de toutes vos marketplaces
    * **Installed** : visualisez et gérez vos plugins installés
    * **Marketplaces** : ajoutez, supprimez ou mettez à jour vos marketplaces ajoutées
    * **Errors** : visualisez les erreurs de chargement de plugins

    Allez à l'onglet **Discover** pour voir les plugins de la marketplace que vous venez d'ajouter. Lorsque votre administrateur a autorisé la marketplace via le paramètre géré [`pluginSuggestionMarketplaces`](/docs/fr/settings#available-settings), les plugins marqués comme pertinents pour votre répertoire de travail actuel sont épinglés en haut avec une étiquette **suggested for this directory**.
  </Step>

  <Step title="Installer un plugin">
    Sélectionnez un plugin pour voir ses détails. Le volet de détails affiche ce que le plugin contient et ce qu'il coûte :

    * Une estimation du **Context cost** afin que vous puissiez voir combien de tokens le plugin ajoutera à votre [fenêtre de contexte](/docs/fr/features-overview#understand-context-costs) à chaque tour (Claude Code v2.1.143 et versions ultérieures)
    * La date de **Last updated** du plugin (v2.1.144 et versions ultérieures)
    * Une section **Will install** listant les commandes, agents, skills, hooks et serveurs MCP et LSP du plugin, afin que vous puissiez examiner exactement ce qu'il ajoute avant l'installation (v2.1.145 et versions ultérieures)

    Choisissez une portée d'installation :

    * **User scope** : installez pour vous-même dans tous les projets
    * **Project scope** : installez pour tous les collaborateurs sur ce référentiel
    * **Local scope** : installez pour vous-même dans ce référentiel uniquement

    Par exemple, sélectionnez **commit-commands**, un plugin qui ajoute des skills de workflow git, et installez-le à votre portée utilisateur.

    Vous pouvez également installer directement depuis la ligne de commande :

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    Consultez [Configuration scopes](/docs/fr/settings#configuration-scopes) pour en savoir plus sur les portées.
  </Step>

  <Step title="Utiliser votre nouveau plugin">
    Après l'installation, exécutez `/reload-plugins` pour activer le plugin. Les skills de plugin sont espacés par le nom du plugin, donc **commit-commands** fournit des skills comme `/commit-commands:commit`.

    Essayez en effectuant une modification à un fichier et en exécutant :

    ```shell theme={null}
    /commit-commands:commit
    ```

    Cela prépare vos modifications, génère un message de commit et crée le commit.

    Chaque plugin fonctionne différemment. Consultez les détails du plugin dans l'onglet **Discover** pour voir les commandes et skills qu'il fournit, ou visitez sa page d'accueil pour obtenir des conseils d'utilisation.
  </Step>
</Steps>

Le reste de ce guide couvre tous les moyens d'ajouter des marketplaces, installer des plugins et gérer votre configuration.

<h2 id="add-marketplaces">
  Ajouter des marketplaces
</h2>

Utilisez la commande `/plugin marketplace add` pour ajouter des marketplaces de différentes sources.

<Tip>
  **Raccourcis** : Vous pouvez utiliser `/plugin market` au lieu de `/plugin marketplace`, et `rm` au lieu de `remove`.
</Tip>

* **Référentiels GitHub** : format `owner/repo` (par exemple, `anthropics/claude-code`)
* **URLs Git** : n'importe quelle URL de référentiel git, y compris GitLab, Bitbucket et les serveurs auto-hébergés
* **Chemins locaux** : répertoires ou chemins directs vers les fichiers `marketplace.json`
* **URLs distantes** : URLs directs vers les fichiers `marketplace.json` hébergés

<h3 id="add-from-github">
  Ajouter depuis GitHub
</h3>

Ajoutez un référentiel GitHub qui contient un fichier `.claude-plugin/marketplace.json` en utilisant le format `owner/repo`, où `owner` est le nom d'utilisateur ou l'organisation GitHub et `repo` est le nom du référentiel.

Par exemple, `anthropics/claude-code` fait référence au référentiel `claude-code` appartenant à `anthropics` :

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  Ajouter depuis d'autres hôtes Git
</h3>

Ajoutez n'importe quel référentiel git en fournissant l'URL complète. Cela fonctionne avec n'importe quel hôte Git, y compris GitLab, Bitbucket et les serveurs auto-hébergés. Incluez le suffixe `.git` pour que Claude Code clone le référentiel plutôt que de traiter l'URL comme un lien direct vers un fichier `marketplace.json` hébergé.

Incluez le préfixe `https://` également. Claude Code v2.1.196 et versions ultérieures rejettent un hôte saisi sans celui-ci, tel que `gitlab.com/company/plugins.git`, comme un raccourci `owner/repo` GitHub invalide, et l'erreur vous indique d'ajouter le préfixe. Les versions antérieures l'ont mal interprété comme un chemin de référentiel GitHub et échouent au moment du clonage.

Utilisation de HTTPS :

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

Utilisation de SSH :

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

Pour ajouter une branche ou un tag spécifique, ajoutez `#` suivi de la ref :

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  Ajouter depuis des chemins locaux
</h3>

Ajoutez un répertoire local qui contient un fichier `.claude-plugin/marketplace.json` :

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

Vous pouvez également ajouter un chemin direct vers un fichier `marketplace.json` :

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  Ajouter depuis des URLs distantes
</h3>

Ajoutez un fichier `marketplace.json` distant via URL :

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  Les marketplaces basées sur URL ont certaines limitations par rapport aux marketplaces basées sur Git. Si vous rencontrez des erreurs « path not found » lors de l'installation de plugins, consultez [Dépannage](/docs/fr/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces).
</Note>

<h2 id="install-plugins">
  Installer des plugins
</h2>

Une fois que vous avez ajouté des marketplaces, vous pouvez installer des plugins directement :

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

La commande ouvre les détails de ce plugin, où vous choisissez une [portée d'installation](/docs/fr/settings#configuration-scopes). Vous voyez les mêmes choix lorsque vous exécutez `/plugin`, allez à l'onglet **Discover** et appuyez sur **Enter** sur un plugin :

* **User scope** (par défaut) : installez pour vous-même dans tous les projets
* **Project scope** : installez pour tous les collaborateurs sur ce référentiel, ce qui ajoute le plugin à `.claude/settings.json`
* **Local scope** : installez pour vous-même dans ce référentiel uniquement, non partagé avec les collaborateurs

Pour installer sans étape interactive, utilisez la commande shell [`claude plugin install`](/docs/fr/plugins-reference#plugin-install), qui s'installe à la portée utilisateur sauf si vous passez `--scope`.

Vous pouvez également voir des plugins avec la portée **managed**. Ceux-ci sont installés par les administrateurs via [managed settings](/docs/fr/settings#settings-files) et ne peuvent pas être modifiés.

<Warning>
  Assurez-vous de faire confiance à un plugin avant de l'installer. Anthropic ne contrôle pas quels serveurs MCP, fichiers ou autres logiciels sont inclus dans les plugins et ne peut pas vérifier qu'ils fonctionnent comme prévu. Consultez la page d'accueil de chaque plugin pour plus d'informations.
</Warning>

<h2 id="manage-installed-plugins">
  Gérer les plugins installés
</h2>

Exécutez `/plugin` et allez à l'onglet **Installed** pour visualiser, activer, désactiver ou désinstaller vos plugins. La liste est groupée par portée et triée pour que vous voyiez d'abord les problèmes : les plugins avec des erreurs de chargement ou des dépendances non résolues apparaissent en haut, suivis de vos favoris, avec les plugins désactivés repliés derrière un en-tête réduit en bas.

Depuis la liste, vous pouvez :

* appuyer sur `f` pour ajouter ou retirer le plugin sélectionné de vos favoris
* taper pour filtrer par nom ou description du plugin
* appuyer sur Enter pour ouvrir la vue détaillée d'un plugin et l'activer, le désactiver ou le désinstaller

Désinstaller un plugin qu'un `.claude/settings.json` de projet active demande quelle portée vous visez : le désactiver pour vous seul, ce qui écrit une substitution dans votre `.claude/settings.local.json` et laisse le plugin installé pour le projet, ou le désinstaller pour tout le monde, ce qui le supprime du `.claude/settings.json` partagé. Nécessite Claude Code v2.1.203 ou ultérieur. Avant v2.1.203, la boîte de dialogue proposait uniquement la désactivation locale.

La vue détaillée affiche les composants que le plugin contribue : commandes, skills, agents, hooks, serveurs MCP et serveurs LSP. Le même inventaire est disponible depuis la ligne de commande avec `claude plugin details`.

L'onglet **Installed** collecte également les plugins de marketplace que vous avez installés vous-même mais que vous n'avez pas utilisés depuis au moins deux semaines, sur une période d'au moins 10 sessions, sous un en-tête **Not used recently**. La vue détaillée affiche une ligne **Last used** pour chaque plugin. Utilisez ces informations pour trouver les plugins qui ajoutent toujours un coût de démarrage et de contexte même si vous ne les utilisez plus, puis désactivez-les ou désinstallez-les. Nécessite Claude Code v2.1.187 ou ultérieur.

Deux types de plugins ne sont jamais listés comme inutilisés :

* les plugins que votre organisation gère ou que vous chargez avec `--plugin-dir`
* les plugins qui contribuent un thème, un style de sortie, un moniteur ou un workflow, car ils fournissent de la valeur sans une invocation à suivre

L'en-tête **Not used recently** et la ligne **Last used** sont tous deux masqués quand votre organisation restreint les marketplaces avec [`strictKnownMarketplaces`](/docs/fr/settings#strictknownmarketplaces).

Un [serveur de langage](/docs/fr/plugins#add-lsp-servers-to-your-plugin) d'un plugin compte comme utilisé quand il fournit des diagnostics ou répond à une demande de navigation de code, donc un plugin LSP dont le serveur est actif dans vos sessions n'est pas listé comme inutilisé. Avant v2.1.203, l'activité du serveur de langage ne pouvait pas être comptée comme une utilisation, donc les plugins qui contribuent un serveur LSP étaient exemptés du groupe entièrement, de la même manière que les plugins de thème et de style de sortie le sont toujours.

La première session sur une version qui compte l'activité du serveur de langage réinitialise également l'enregistrement d'utilisation de chaque plugin LSP qui n'avait pas encore enregistré d'utilisation, donc Claude Code ne juge pas un plugin que vous avez installé plus tôt comme inutilisé en fonction des données enregistrées avant que l'activité de son serveur soit suivie. Avant v2.1.206, cette première session pouvait lister un plugin LSP activement utilisé sous **Not used recently** et suggérer de l'examiner.

Quand vous installez un plugin qui déclare des dépendances, la sortie d'installation liste quelles dépendances ont été auto-installées avec lui.

Vous pouvez également gérer les plugins avec des commandes directes.

Lister les plugins installés sans ouvrir le menu :

```shell theme={null}
/plugin list
```

Passez `--enabled` ou `--disabled` pour afficher uniquement les plugins dans cet état.

Désactiver un plugin sans le désinstaller :

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

Réactiver un plugin désactivé :

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

Dans ces identifiants, `plugin-name` est le `name` du plugin dans l'[entrée de marketplace](/docs/fr/plugin-marketplaces#plugin-entries), qui peut différer du `name` dans le propre `plugin.json` du plugin.

À partir de Claude Code v2.1.195, **Enable** et **Disable** dans l'interface `/plugin` fonctionnent pour les plugins dont les deux noms diffèrent, et `/plugin enable` et `/plugin disable` acceptent l'un ou l'autre nom. Quand vous désactivez un tel plugin dans une version antérieure, Claude Code signale `already disabled` et le laisse activé.

Supprimer complètement un plugin :

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

L'option `--scope` vous permet de cibler une portée spécifique avec les commandes CLI :

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  Appliquer les modifications de plugin sans redémarrer
</h3>

Quand vous installez, activez ou désactivez des plugins pendant une session, exécutez `/reload-plugins` pour récupérer toutes les modifications sans redémarrer :

```shell theme={null}
/reload-plugins
```

Claude Code recharge tous les plugins actifs et affiche les comptages pour les plugins, les skills, les agents, les hooks, les serveurs MCP de plugin et les serveurs LSP de plugin.

Le rechargement a un coût en jetons sur la demande suivante : les composants nouvellement chargés s'annoncent dans le contenu ajouté à la conversation, tandis que l'historique existant lit toujours à partir du cache de prompt. Un plugin qui fournit des serveurs MCP coûte plus cher quand ses outils ne sont pas différés par [recherche d'outils](/docs/fr/mcp#scale-with-mcp-tool-search) : le changement invalide le cache et la demande suivante relit l'intégralité de la conversation. Dans ce cas, `/reload-plugins` affiche un avertissement et n'applique pas le rechargement ; passez `--force` pour appliquer quand même. Consultez [activation ou désactivation d'un plugin](/docs/fr/prompt-caching#enabling-or-disabling-a-plugin) pour plus de détails.

<h2 id="manage-marketplaces">
  Gérer les marketplaces
</h2>

Vous pouvez gérer les marketplaces via l'interface interactive `/plugin` ou avec des commandes CLI.

<h3 id="use-the-interactive-interface">
  Utiliser l'interface interactive
</h3>

Exécutez `/plugin` et allez à l'onglet **Marketplaces** pour :

* Visualiser toutes vos marketplaces ajoutées avec leurs sources et statut
* Ajouter de nouvelles marketplaces
* Mettre à jour les listes de marketplace pour récupérer les derniers plugins
* Supprimer les marketplaces dont vous n'avez plus besoin

<h3 id="use-cli-commands">
  Utiliser les commandes CLI
</h3>

Vous pouvez également gérer les marketplaces avec des commandes directes.

Lister toutes les marketplaces configurées :

```shell theme={null}
/plugin marketplace list
```

Actualiser les listes de plugins d'une marketplace :

```shell theme={null}
/plugin marketplace update marketplace-name
```

Supprimer une marketplace :

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  La suppression d'une marketplace désinstallera tous les plugins que vous avez installés à partir de celle-ci.
</Warning>

<h3 id="configure-auto-updates">
  Configurer les mises à jour automatiques
</h3>

Claude Code peut automatiquement mettre à jour les marketplaces et leurs plugins installés en arrière-plan après le démarrage. Quand la mise à jour automatique est activée pour une marketplace, Claude Code actualise les données de la marketplace et met à jour les plugins installés vers leurs dernières versions sur le disque.

Claude Code vérifie les mises à jour de marketplace et de plugins après le démarrage de votre session, avec un délai aléatoire pouvant aller jusqu'à dix minutes, de sorte que la session en cours continue d'utiliser les versions qu'elle a chargées au lancement. Si des plugins ont été mis à jour, vous verrez une notification vous invitant à exécuter `/reload-plugins`, ou les nouvelles versions se chargeront au prochain lancement.

Basculez la mise à jour automatique pour les marketplaces individuelles via l'interface utilisateur :

1. Exécutez `/plugin` pour ouvrir le gestionnaire de plugins
2. Sélectionnez **Marketplaces**
3. Choisissez une marketplace dans la liste
4. Sélectionnez **Enable auto-update** ou **Disable auto-update**

Les marketplaces officielles Anthropic ont la mise à jour automatique activée par défaut. Les marketplaces tierces et de développement local ont la mise à jour automatique désactivée par défaut.

Les administrateurs peuvent également définir `"autoUpdate": true` sur chaque entrée [`extraKnownMarketplaces`](/docs/fr/settings#extraknownmarketplaces) dans les paramètres gérés pour activer la mise à jour automatique pour une marketplace d'organisation sans exiger que chaque utilisateur la bascule.

Pour désactiver complètement toutes les mises à jour automatiques pour Claude Code et tous les plugins, définissez la variable d'environnement `DISABLE_AUTOUPDATER`. Consultez [Auto updates](/docs/fr/setup#auto-updates) pour plus de détails.

Pour garder les mises à jour automatiques des plugins activées tout en désactivant les mises à jour automatiques de Claude Code, définissez `FORCE_AUTOUPDATE_PLUGINS=1` avec `DISABLE_AUTOUPDATER` :

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

Cela est utile quand vous voulez gérer les mises à jour de Claude Code manuellement mais recevoir toujours les mises à jour automatiques des plugins.

<h2 id="configure-team-marketplaces">
  Configurer les marketplaces d'équipe
</h2>

Les administrateurs d'équipe peuvent configurer l'installation automatique de marketplace pour les projets en ajoutant la configuration de marketplace à `.claude/settings.json`. Quand les membres de l'équipe font confiance au dossier du référentiel, Claude Code les invite à installer ces marketplaces et plugins.

À partir de Claude Code v2.1.195, cette étape d'installation s'applique sur chaque chemin qui charge des plugins. Un plugin que seul le `.claude/settings.json` du projet active, et qui provient d'une source externe telle qu'un référentiel GitHub ou un package npm, ne se charge pas tant que le membre de l'équipe ne l'installe pas. Jusqu'à ce moment, Claude Code signale le plugin comme non installé et affiche la commande `claude plugin install` à exécuter.

Ajoutez `extraKnownMarketplaces` au `.claude/settings.json` de votre projet :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Pour les options de configuration complètes incluant `extraKnownMarketplaces` et `enabledPlugins`, consultez [Plugin settings](/docs/fr/settings#plugin-settings).

<h2 id="security">
  Sécurité
</h2>

Les plugins et les marketplaces sont des composants hautement fiables qui peuvent exécuter du code arbitraire sur votre machine avec vos privilèges utilisateur. Installez uniquement les plugins et ajoutez les marketplaces à partir de sources auxquelles vous faites confiance. Les organisations peuvent restreindre quelles marketplaces les utilisateurs sont autorisés à ajouter en utilisant [managed marketplace restrictions](/docs/fr/plugin-marketplaces#managed-marketplace-restrictions).

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="/plugin-command-not-recognized">
  Commande /plugin non reconnue
</h3>

Si vous voyez « unknown command » ou la commande `/plugin` n'apparaît pas :

1. **Vérifiez votre version** : Exécutez `claude --version` pour voir ce qui est installé.
2. **Mettez à jour Claude Code** :
   * **Homebrew** : `brew upgrade claude-code`, ou `brew upgrade claude-code@latest` si vous avez installé ce cask
   * **npm** : `npm install -g @anthropic-ai/claude-code@latest`
   * **Native installer** : Réexécutez la commande d'installation depuis [Setup](/docs/fr/setup)
3. **Redémarrez Claude Code** : Après la mise à jour, redémarrez votre terminal et exécutez `claude` à nouveau.

<h3 id="common-issues">
  Problèmes courants
</h3>

* **Marketplace ne se charge pas** : Vérifiez que l'URL est accessible et que `.claude-plugin/marketplace.json` existe au chemin
* **Échecs d'installation de plugin** : Vérifiez que les URLs sources du plugin sont accessibles et que les référentiels sont publics, ou que vous y avez accès
* **Fichiers non trouvés après l'installation** : Les plugins sont copiés dans un cache, donc les chemins référençant des fichiers en dehors du répertoire du plugin ne fonctionneront pas
* **Les skills du plugin n'apparaissent pas** : Effacez le cache avec `rm -rf ~/.claude/plugins/cache`, redémarrez Claude Code et réinstallez le plugin.

Pour un dépannage détaillé avec des solutions, consultez [Dépannage](/docs/fr/plugin-marketplaces#troubleshooting) dans le guide de la marketplace. Pour les outils de débogage, consultez [Debugging and development tools](/docs/fr/plugins-reference#debugging-and-development-tools).

<h3 id="code-intelligence-issues">
  Problèmes de code intelligence
</h3>

* **Le serveur de langage ne démarre pas** : Vérifiez que le binaire est installé et disponible dans votre `$PATH`. Consultez l'onglet Errors de `/plugin` pour plus de détails.
* **Utilisation élevée de la mémoire** : Les serveurs de langage comme `rust-analyzer` et `pyright` peuvent consommer une mémoire importante sur les grands projets. Si vous rencontrez des problèmes de mémoire, désactivez le plugin avec `/plugin disable <plugin-name>` et fiez-vous aux outils de recherche intégrés de Claude à la place.
* **Diagnostics faux positifs dans les monorepos** : Les serveurs de langage peuvent signaler des erreurs d'import non résolues pour les packages internes si l'espace de travail n'est pas configuré correctement. Ceux-ci n'affectent pas la capacité de Claude à modifier le code.

<h2 id="next-steps">
  Prochaines étapes
</h2>

* **Construisez vos propres plugins** : Consultez [Plugins](/docs/fr/plugins) pour créer des skills, des agents et des hooks
* **Créez une marketplace** : Consultez [Créer une marketplace de plugins](/docs/fr/plugin-marketplaces) pour distribuer des plugins à votre équipe ou communauté
* **Référence technique** : Consultez [Plugins reference](/docs/fr/plugins-reference) pour les spécifications complètes
