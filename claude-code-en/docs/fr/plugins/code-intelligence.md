> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins d'intelligence de code

> Installez un plugin de serveur de langage pour que Claude voie les erreurs de type après les modifications et navigue dans le code par symbole, et répondez à la boîte de dialogue de recommandation du plugin LSP.

Un plugin d'intelligence de code donne à Claude les diagnostics en direct et la navigation vers la définition que votre éditeur possède, de sorte que Claude détecte les erreurs de type et les imports manquants que ses propres modifications introduisent avant que vous n'exécutiez votre build, et trouve les définitions et les références par symbole au lieu de par recherche textuelle.

Chaque plugin connecte Claude Code à un serveur de langage pour un langage via le Language Server Protocol (LSP). Vous installez le plugin depuis la marketplace officielle d'Anthropic et le binaire du serveur de langage sur votre machine.

<Note>
  Les plugins d'intelligence de code fonctionnent dans les sessions de terminal. Dans les [sessions cloud](/docs/fr/claude-code-on-the-web), Claude Code ne démarre pas les serveurs de langage des plugins, donc Claude n'obtient pas de diagnostics ou de navigation de code là-bas. Pour écrire votre propre plugin de serveur de langage, ou pour connecter un serveur de langage qui n'a pas de plugin, voir [Serveurs LSP dans les composants de plugin](/docs/fr/plugins/components#lsp-servers).
</Note>

Pour commencer, trouvez votre langage dans le tableau sous [Installer un plugin d'intelligence de code](#install-a-code-intelligence-plugin). Les plugins de ce tableau proviennent de la [marketplace officielle de plugins](/docs/fr/plugins/anthropic-marketplaces) d'Anthropic.

Si vous avez déjà vu une boîte de dialogue de **recommandation de plugin LSP**, voir [Accepter ou rejeter la boîte de dialogue de recommandation](#accept-or-dismiss-the-recommendation-dialog) pour savoir ce que chaque choix fait.

<h2 id="install-a-code-intelligence-plugin">
  Installer un plugin d'intelligence de code
</h2>

Un plugin d'intelligence de code indique à Claude Code quelle commande démarre le serveur de langage et quelles extensions de fichier il gère. Il n'inclut pas le serveur de langage. Installez d'abord le binaire du serveur de langage, puis le plugin, puis confirmez que le serveur démarre.

<Steps>
  <Step title="Installer le binaire du serveur de langage">
    Trouvez votre langage dans le tableau ci-dessous et installez le binaire dans sa ligne. Si votre langage n'est pas listé, voir [Ajouter un langage sans plugin officiel](#add-a-language-without-an-official-plugin).

    | Langage                  | Plugin                                                                                                           | Binaire                          |
    | :----------------------- | :--------------------------------------------------------------------------------------------------------------- | :------------------------------- |
    | C/C++                    | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                         |
    | C#                       | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                      |
    | Go                       | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                          |
    | Java                     | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                          |
    | Kotlin                   | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                     |
    | Liquid                   | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, depuis la CLI Shopify |
    | Lua                      | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`            |
    | PHP                      | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`                   |
    | Python                   | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`             |
    | Ruby                     | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                       |
    | Rust                     | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`                  |
    | Swift                    | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`                  |
    | TypeScript et JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server`     |

    Anthropic maintient tous les plugins du tableau sauf `liquid-lsp`, que Shopify maintient et que la marketplace officielle liste.

    Pour trouver la commande qui installe le binaire, suivez le lien du plugin dans le tableau vers son README. Pour TypeScript, cette commande est `npm install -g typescript-language-server typescript`.

    Après avoir installé le binaire, confirmez qu'il se trouve sur le `PATH` du shell à partir duquel vous démarrez `claude`, par exemple avec `which typescript-language-server`, ou `Get-Command typescript-language-server` dans PowerShell.
  </Step>

  <Step title="Installer le plugin">
    Pour installer le plugin listé pour votre langage dans le tableau de l'étape 1, exécutez `/plugin install` dans une session Claude Code, en remplaçant `typescript-lsp` par le nom de ce plugin :

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    Un message de confirmation indique si le plugin est actif maintenant ou s'il a besoin de `/reload-plugins`. Si l'installation échoue avec `Marketplace "claude-plugins-official" not found`, voir l'[entrée de dépannage pour cette erreur](/docs/fr/plugins/troubleshooting#marketplace-claude-plugins-official-not-found). Pour contrôler où le plugin est installé, ou pour exécuter l'installation depuis votre shell au lieu de dans Claude Code, voir [Installer les plugins](/docs/fr/plugins/install).
  </Step>

  <Step title="Confirmer que le serveur démarre">
    Le serveur de langage démarre la première fois que Claude modifie un fichier avec l'une des extensions du plugin. Pour le voir fonctionner, demandez à Claude d'introduire une erreur de type dans un fichier de ce langage, puis de la corriger. Ensuite, vérifiez la conversation pour une ligne de diagnostics :

    * **Une ligne de diagnostics apparaît** : `Found N new diagnostic issues in M files (ctrl+o to expand)` sous la modification qui a introduit l'erreur signifie que le serveur a démarré.
    * **Aucune ligne de diagnostics n'apparaît** : exécutez `/plugin` et ouvrez l'onglet **Errors**. Une ligne lisant `Executable not found in $PATH: "<binary>"` nomme le binaire à installer. Si l'onglet n'a pas de telle ligne, voir [Dépanner l'intelligence de code](#troubleshoot-code-intelligence).

    Après avoir installé un binaire manquant, Claude Code réessaie la prochaine fois que Claude modifie un fichier correspondant. Si vous avez installé le binaire dans un répertoire qui ne se trouve pas sur le `PATH` du shell à partir duquel vous avez démarré `claude`, démarrez une nouvelle session à partir d'un shell où il se trouve.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  Voir ce que Claude gagne
</h2>

Avec un serveur de langage en cours d'exécution, Claude gagne les diagnostics et la navigation de code :

* **Diagnostics après les modifications** : chaque fois que Claude modifie ou écrit un fichier que le serveur gère, Claude obtient les erreurs et les avertissements que le serveur signale. Il voit une erreur de type, un import manquant, ou une erreur de syntaxe qu'il a introduite sans exécuter un compilateur.
* **Navigation de code** : Claude obtient un outil `LSP` qui recherche les symboles via le serveur au lieu de les rechercher par texte. L'outil est en lecture seule. Pour savoir ce que Claude peut rechercher avec l'outil et comment les permissions s'appliquent à celui-ci, voir [Comportement de l'outil LSP](/docs/fr/tools-reference#lsp-tool-behavior).

<h3 id="read-the-diagnostics-yourself">
  Lire les diagnostics vous-même
</h3>

Après que Claude modifie un fichier que le serveur gère, la conversation affiche uniquement le résumé `Found N new diagnostic issues`. Pour lire les problèmes eux-mêmes, appuyez sur **Ctrl+O**.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  Accepter ou rejeter la boîte de dialogue de recommandation
</h2>

Si un binaire de serveur de langage se trouve déjà sur votre `PATH` et le plugin qui l'utilise n'est pas installé, Claude Code vous propose d'installer le plugin pour vous dans une boîte de dialogue intitulée **LSP plugin recommendation**.

<h3 id="when-the-recommendation-dialog-appears">
  Quand la boîte de dialogue de recommandation apparaît
</h3>

La boîte de dialogue **LSP plugin recommendation** peut apparaître après que Claude modifie un fichier. Ces conditions décident si elle apparaît et quel plugin elle offre :

* **Un plugin correspond au fichier** : l'une des marketplaces que vous avez ajoutées, ou la marketplace officielle que Claude Code a enregistrée pour vous, liste un plugin d'intelligence de code pour l'extension de ce fichier, et le binaire du plugin est installé.
* **Officiel en premier** : quand plus d'une marketplace offre un plugin pour l'extension, la boîte de dialogue offre le plugin de la marketplace officielle.
* **Une fois par session** : la boîte de dialogue apparaît au maximum une fois dans une session, pour le premier fichier correspondant que Claude modifie.
* **Pas pour les sessions cloud** : la boîte de dialogue n'apparaît jamais quand votre terminal est attaché à une session cloud, comme celle que vous avez démarrée avec [`claude --cloud`](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud).

<h3 id="respond-to-the-recommendation-dialog">
  Répondre à la boîte de dialogue de recommandation
</h3>

La boîte de dialogue **LSP plugin recommendation** nomme le plugin et offre ces choix :

* **Yes, install** : Claude Code installe le plugin pour votre compte utilisateur et affiche `<plugin> installed · restart to apply`. Démarrez une nouvelle session pour charger le serveur.
* **No, not now** : la boîte de dialogue se ferme, et une session ultérieure peut offrir le plugin à nouveau. Appuyer sur **Esc** fait la même chose.
* **Never for this plugin** : la boîte de dialogue cesse d'apparaître pour ce plugin et continue d'apparaître pour les autres.
* **Disable all LSP recommendations** : la boîte de dialogue cesse d'apparaître pour tous les langages.

Si vous ne choisissez pas une option, Claude Code la ferme après 30 secondes et compte cela comme ignoré. Le compte est conservé entre les sessions. Après cinq boîtes de dialogue ignorées, Claude Code cesse de recommander les plugins, comme si vous aviez choisi **Disable all LSP recommendations**.

<h3 id="turn-recommendations-back-on">
  Réactiver les recommandations
</h3>

La boîte de dialogue **LSP plugin recommendation** cesse d'apparaître après que vous ayez choisi **Disable all LSP recommendations** ou l'ayez ignorée cinq fois.

* **Désactivée ou ignorée cinq fois** : pour la réactiver dans l'un ou l'autre cas, supprimez les clés `lspRecommendationDisabled` et `lspRecommendationIgnoredCount` de `~/.claude.json`, le fichier de configuration propre à Claude Code.
* **Never for this plugin** : si vous avez choisi **Never for this plugin** et souhaitez que ce plugin soit offert à nouveau, supprimez son identifiant `name@marketplace` de la liste `lspRecommendationNeverPlugins` dans le même fichier.

<h2 id="troubleshoot-code-intelligence">
  Dépanner l'intelligence de code
</h2>

La page de dépannage des plugins couvre les symptômes spécifiques aux plugins d'intelligence de code sous [Language server doesn't start, uses too much memory, or reports wrong diagnostics](/docs/fr/plugins/troubleshooting#language-server-doesnt-start) :

* **Le serveur de langage ne démarre pas** : vous voyez `Executable not found in $PATH` dans l'onglet **Errors** de `/plugin`, ou Claude ne signale jamais de diagnostics pour le langage.
* **Utilisation élevée de la mémoire** : l'utilisation de la mémoire augmente pendant que le serveur indexe le projet.
* **Diagnostics faux positifs dans un monorepo** : les diagnostics signalent les imports comme non résolus quand ils ne le sont pas.

<h2 id="add-a-language-without-an-official-plugin">
  Ajouter un langage sans plugin officiel
</h2>

Si votre langage ne figure pas dans le [tableau des plugins officiels](#install-a-code-intelligence-plugin), vous pouvez toujours connecter un serveur de langage.

1. Écrivez un plugin avec un fichier `.lsp.json` qui nomme la commande du serveur et les extensions de fichier qu'il gère.
2. Ensuite, chargez le plugin avec [`--plugin-dir`](/docs/fr/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) ou publiez-le sur une marketplace.

Pour les champs du fichier et un exemple travaillé, voir [Serveurs LSP dans les composants de plugin](/docs/fr/plugins/components#lsp-servers).

<h2 id="next-steps">
  Prochaines étapes
</h2>

* [Serveurs LSP dans les composants de plugin](/docs/fr/plugins/components#lsp-servers) : écrivez le `.lsp.json` pour un serveur de langage qui n'a pas de plugin officiel
* [Installer et gérer les plugins](/docs/fr/plugins/install) : portées, mises à jour et désinstallation
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting) : charger les erreurs au-delà de celles spécifiques au serveur de langage sur cette page
* [Trouver les plugins dans la marketplace officielle](/docs/fr/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace) : où parcourir le reste de la marketplace officielle
