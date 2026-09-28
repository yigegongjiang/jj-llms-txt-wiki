> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aperçu des plugins

> Comprenez ce qu'est un plugin Claude Code, quand vous en avez besoin au lieu d'une compétence autonome ou d'un serveur MCP, et quelle page lire pour en installer ou en créer un.

Un plugin Claude Code est un répertoire de skills, d'agents, de hooks, de serveurs MCP ou d'autres composants que Claude Code installe et charge comme une seule unité. La plupart des plugins proviennent d'une marketplace, qui est un catalogue listant les plugins et indiquant où récupérer chacun. Vous pouvez également charger un plugin à partir d'un dossier que quelqu'un vous donne, ou [créer le vôtre](/docs/fr/plugins/create).

<Note>
  Si vous utilisez le chat claude.ai ou Cowork et non Claude Code, consultez [Plugins sur claude.ai et dans Cowork](https://claude.com/docs/plugins/overview).
</Note>

Pour essayer un plugin maintenant, exécutez `/plugin` dans une session de terminal Claude Code et installez-en un à partir de l'onglet **Discover**, qui liste les plugins de la marketplace officielle d'Anthropic et de toute marketplace que vous avez ajoutée. À partir de là :

* [Installer et gérer les plugins](/docs/fr/plugins/install) : les étapes d'installation complètes, les portées et autres surfaces
* [Créer un plugin](/docs/fr/plugins/create) : créez le vôtre
* [Décider si vous avez besoin d'un plugin](#decide-whether-you-need-a-plugin) : si un plugin est le bon outil pour ce que vous voulez

<h2 id="understand-what-a-plugin-is">
  Comprendre ce qu'est un plugin
</h2>

Un plugin est un répertoire de composants, généralement avec un manifeste. Le manifeste, un fichier JSON à `.claude-plugin/plugin.json`, donne son nom au plugin et peut ajouter une version, une description et d'autres [métadonnées](/docs/fr/plugins/manifest-reference). Les composants sont ce que le plugin ajoute à Claude Code, tels que :

* [**Skills**](/docs/fr/plugins/components#skills) : instructions `SKILL.md` que Claude charge quand c'est pertinent, et que vous pouvez également exécuter en tant que commande
* [**Agents**](/docs/fr/plugins/components#agents) : définitions de sous-agents que Claude peut déléguer
* [**Hooks**](/docs/fr/plugins/components#hooks) : commandes que Claude Code exécute à des points de son cycle de vie, comme après chaque modification
* [**Serveurs MCP**](/docs/fr/plugins/components#mcp-servers) : serveurs d'outils auxquels Claude Code se connecte pendant que le plugin est activé

Ce diagramme montre un plugin nommé `my-plugin` qui contient un de chacun de ces composants, et ce que vous obtenez de chaque fichier une fois que le plugin se charge.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagramme en deux colonnes jointes par cinq flèches droites. À gauche, le répertoire d'un plugin nommé my-plugin, contenant un manifeste à .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json et d'autres composants. À droite, ce que chaque fichier vous donne dans votre session : le manifeste définit le nom du plugin, my-plugin ; la skill s'exécute en tant que /my-plugin:review ; le fichier agent est un sous-agent que Claude peut déléguer ; le fichier hooks contient des hooks qui s'exécutent sur les événements du cycle de vie ; et .mcp.json ajoute un serveur MCP qui donne à Claude des outils." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagramme en deux colonnes jointes par cinq flèches droites. À gauche, le répertoire d'un plugin nommé my-plugin, contenant un manifeste à .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json et d'autres composants. À droite, ce que chaque fichier vous donne dans votre session : le manifeste définit le nom du plugin, my-plugin ; la skill s'exécute en tant que /my-plugin:review ; le fichier agent est un sous-agent que Claude peut déléguer ; le fichier hooks contient des hooks qui s'exécutent sur les événements du cycle de vie ; et .mcp.json ajoute un serveur MCP qui donne à Claude des outils." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

Pour chaque type de composant qu'un plugin peut contenir, avec un exemple de chacun, consultez [Composants de plugin](/docs/fr/plugins/components). Pour voir où chaque élément est situé dans le répertoire d'un plugin, utilisez l'[explorateur de plugin](/docs/fr/plugins/components#explore-the-plugin-directory) sur cette page.

<h3 id="decide-whether-you-need-a-plugin">
  Décider si vous avez besoin d'un plugin
</h3>

Les skills, sous-agents, hooks et serveurs MCP fonctionnent tous seuls, sans plugin. Une skill que vous enregistrez dans `~/.claude/skills/`, par exemple, est disponible dans chaque projet sur votre machine. Pour en configurer une seule, consultez [Skills](/docs/fr/skills), [Sous-agents](/docs/fr/sub-agents), [Hooks](/docs/fr/hooks-guide) ou [MCP](/docs/fr/mcp).

Utilisez un plugin quand vous voulez plusieurs skills, sous-agents, hooks ou serveurs MCP empaquetés comme une seule unité. Installez-en un pour obtenir une configuration que quelqu'un d'autre a construite, avec une seule commande et des mises à jour de sa marketplace. Créez-en un pour donner votre propre configuration à vos coéquipiers, l'installer dans de nombreux projets ou publier des versions avec numérotation.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  Ce qu'un plugin activé ajoute à vos sessions
</h3>

Un plugin activé fait partie de chaque session, pas seulement des sessions où vous l'utilisez. Cela a quelques conséquences qu'il vaut la peine de connaître avant d'en installer un :

* **Contexte et utilisation** : pour chaque skill, agent et commande que [Claude peut invoquer de lui-même](/docs/fr/skills#control-who-invokes-a-skill), le nom et la description sont dans le contexte de Claude à chaque tour pour que Claude sache qu'il existe. Ces jetons comptent vers votre utilisation et laissent moins de place dans la [fenêtre de contexte](/docs/fr/context-window) même dans les sessions où rien du plugin ne s'exécute. Le texte complet d'une skill ou d'un agent se charge uniquement quand il est utilisé. Ce que les serveurs MCP du plugin ajoutent par tour suit la [recherche d'outils MCP](/docs/fr/mcp#scale-with-mcp-tool-search).
* **Processus** : les serveurs MCP que le plugin définit s'exécutent aux côtés de chaque session où il est activé, et ses hooks se déclenchent à leurs événements.
* **Permissions** : ce que le plugin exécute, il l'exécute en tant que vous. Consultez [Sécurité et confiance des plugins](/docs/fr/plugins/security) pour savoir ce qu'il faut d'abord examiner.

Vous pouvez vérifier l'empreinte d'un plugin à chaque étape :

* **Avant d'installer** : ouvrez le plugin à partir de l'onglet **Marketplaces** dans `/plugin`. Les plugins de la marketplace officielle d'Anthropic affichent une estimation du **Context cost** là.
* **Après l'installation** : [Mesurer le coût d'un plugin](/docs/fr/plugins/measure#measure-what-a-plugin-costs) montre comment lire l'empreinte d'un plugin, et le groupe **Not used recently** de l'onglet **Installed** liste les plugins que vous pourriez désactiver.
* **Pour l'arrêter sans le désinstaller** : désactivez le plugin avec `/plugin` ou, dans votre shell, `claude plugin disable`. Consultez [Gérer les plugins installés](/docs/fr/plugins/install#manage-installed-plugins).

<h2 id="get-plugins-from-a-marketplace">
  Obtenir des plugins à partir d'une marketplace
</h2>

Une marketplace est un référentiel ou un répertoire avec un fichier `.claude-plugin/marketplace.json` qui liste les plugins et indique où récupérer chacun. C'est un catalogue, pas un magasin hébergé. Vous ajoutez une marketplace une fois, puis installez les plugins à partir de celle-ci par nom, comme `commit-commands@claude-plugins-official`.

<Note>
  Une marketplace de plugins n'est pas [Claude Marketplace](https://claude.com/marketplace). Claude Marketplace est le site web à claude.com/marketplace où vous parcourez les plugins, les connecteurs, les produits partenaires et les partenaires de service. Ce n'est pas une marketplace que vous ajoutez avec `/plugin marketplace add`.
</Note>

Claude Code ajoute la marketplace officielle d'Anthropic la première fois que vous démarrez une session de terminal interactive, sauf si une [politique gérée](/docs/fr/plugins/org#allow-the-official-marketplace-and-your-own) l'en empêche. Claude Code n'ajoute aucune autre marketplace de lui-même, y compris les marketplaces communautaires et de démonstration d'Anthropic. Pour distinguer les trois marketplaces d'Anthropic, lisez [Marketplaces d'Anthropic](/docs/fr/plugins/anthropic-marketplaces). Pour voir ce que celle officielle liste, ouvrez l'onglet **Discover** de `/plugin` dans une session ou parcourez [Claude Marketplace](https://claude.com/marketplace/plugins).

Ce diagramme montre le chemin d'une marketplace à votre session. Une marketplace liste un plugin, vous installez ce plugin, et Claude Code charge ses composants.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagramme du chemin de la marketplace en trois boîtes, de gauche à droite. Une marketplace, un catalogue de plugins, liste un plugin. Le plugin est un répertoire installé comme une unité, contenant des skills, des agents, des hooks, des serveurs MCP et d'autres composants. Vous installez le plugin dans Claude Code, qui charge ses composants." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagramme du chemin de la marketplace en trois boîtes, de gauche à droite. Une marketplace, un catalogue de plugins, liste un plugin. Le plugin est un répertoire installé comme une unité, contenant des skills, des agents, des hooks, des serveurs MCP et d'autres composants. Vous installez le plugin dans Claude Code, qui charge ses composants." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[Installer et gérer les plugins](/docs/fr/plugins/install#install-a-plugin) contient les étapes d'installation pour chaque endroit où vous exécutez Claude Code. Pendant que vous développez un plugin, vous n'avez pas besoin d'une marketplace : chargez-le directement à partir de son dossier avec `--plugin-dir`, comme le montre [Développer sans marketplace](/docs/fr/plugins/create#develop-without-a-marketplace).

<h3 id="make-an-installed-plugin-available-in-your-session">
  Rendre un plugin installé disponible dans votre session
</h3>

Avant qu'un plugin que vous avez installé vous donne une skill que vous pouvez exécuter, il doit être présent à chacune de ces couches :

* **Paramètres** : vos paramètres listent les marketplaces que vous avez ajoutées et les plugins qui sont activés.
* **Disque** : `~/.claude/plugins/` contient ce que Claude Code a récupéré et installé.
* **Session** : les plugins se chargent au démarrage, ou quand vous [rechargez les plugins](/docs/fr/plugins/loading#check-which-stage-a-plugin-reached).

Lisez [Référence de chargement des plugins](/docs/fr/plugins/loading) pour les règles à chaque couche, y compris quel fichier de paramètres a la priorité et où se trouvent les fichiers sur le disque.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Distinguer les marketplaces d'Anthropic des marketplaces tierces
</h2>

Le nom d'une marketplace la place dans l'un des trois niveaux. Claude Code accepte les noms officiels et communautaires uniquement pour les marketplaces provenant de référentiels `github.com/anthropics/` :

* **Officiel** : marketplaces avec l'un des [noms de marketplace officiels](/docs/fr/plugins/security#official-marketplace-names) d'Anthropic, y compris `claude-plugins-official` et la marketplace de démonstration `claude-code-plugins`.
* **Communauté** : marketplaces avec l'un des noms communautaires d'Anthropic, comme `claude-community`. [Identifier les marketplaces d'Anthropic par nom](/docs/fr/plugins/security#marketplace-tiers) les liste.
* **Tiers** : toute autre marketplace. Une marketplace que votre coéquipier ou votre organisation publie est tierce.

Quel que soit le niveau, un plugin que vous installez peut exécuter du code avec vos privilèges utilisateur. Lisez [Sécurité et confiance des plugins](/docs/fr/plugins/security) pour savoir comment examiner un plugin avant de l'installer.

Par le biais des [paramètres gérés](/docs/fr/settings#settings-files), une organisation peut autoriser ou bloquer les marketplaces, forcer l'installation de plugins et désactiver le chargement en session uniquement. Lisez [Gérer les plugins pour votre organisation](/docs/fr/plugins/org) pour ces contrôles.

<h2 id="understand-install-scopes">
  Comprendre les portées d'installation
</h2>

Quand vous installez un plugin, vous choisissez une portée, et la portée décide pour qui le plugin est activé :

* **Portée utilisateur** : activé pour vous dans chaque projet sur cet ordinateur
* **Portée du projet** : activé pour tous ceux qui travaillent dans ce référentiel, via le `.claude/settings.json` commité. Chaque collaborateur doit toujours [l'installer sur sa propre machine](/docs/fr/plugins/loading#enabled-in-project-settings-but-not-installed)
* **Portée locale** : activé pour vous dans ce référentiel uniquement

Un plugin que vous installez à portée utilisateur dans le terminal, les sessions locales de l'application de bureau ou l'extension VS Code est disponible dans les deux autres sur cet ordinateur, car tous les trois lisent les mêmes fichiers de paramètres. Consultez [Choisir une portée d'installation](/docs/fr/plugins/install#choose-an-install-scope) pour savoir comment en choisir une.

Une session cloud, y compris une dans le navigateur à claude.ai/code, ne charge pas les plugins dans vos paramètres locaux. Pour les étapes d'installation dans le terminal, VS Code et l'application de bureau, et pour ce qu'une session cloud charge, consultez [Installer un plugin](/docs/fr/plugins/install#install-a-plugin).

<Note>
  Le même format de plugin s'installe également sur claude.ai et dans Cowork, où un ensemble différent de composants se charge. Pour ces surfaces, consultez [Plugins sur claude.ai et dans Cowork](https://claude.com/docs/plugins/overview) sur claude.com.
</Note>

<h2 id="next-steps">
  Étapes suivantes
</h2>

La plupart des gens commencent par installer un plugin à partir de la marketplace officielle d'Anthropic, que Claude Code ajoute la première fois que vous démarrez une session de terminal interactive. Exécutez `/plugin` dans une session de terminal pour la parcourir, ou suivez [Installer et gérer les plugins](/docs/fr/plugins/install), qui couvre également l'application de bureau et VS Code. Pour voir ce qui se trouve dans cette marketplace avant d'ouvrir Claude Code, parcourez [Claude Marketplace](https://claude.com/marketplace/plugins) sur le web.

Pour créer le vôtre, [Créer un plugin](/docs/fr/plugins/create) commence par un répertoire vide et se termine par un plugin fonctionnel.

Une fois que vous avez installé ou créé un plugin, ces pages couvrent ce qui vient ensuite :

* **Partager ce que vous avez créé** : [Publier et distribuer un plugin](/docs/fr/plugins/publish)
* **Vérifier si cela fonctionne et est utilisé** : [Tester les plugins avec des evals](/docs/fr/plugin-evals) et [Mesurer le coût et l'utilisation des plugins](/docs/fr/plugins/measure)
* **Exécuter une marketplace pour votre équipe** : [Créer une marketplace](/docs/fr/plugins/create-marketplace), puis [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace)
* **Définir la politique des plugins pour une organisation** : [Gérer les plugins pour votre organisation](/docs/fr/plugins/org)
* **Corriger un problème** : [Dépanner les plugins](/docs/fr/plugins/troubleshooting)
