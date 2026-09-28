> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Choisir un environnement sandbox

> Comparez les options de sandbox Claude Code : l'outil Bash sandboxé intégré, le runtime sandbox, les dev containers, Docker et les machines virtuelles. Choisissez l'isolation appropriée pour votre modèle de menace.

L'isolation de Claude Code limite ce qu'une session peut lire, écrire et atteindre sur le réseau. Cela importe surtout lorsque vous laissez Claude travailler avec moins d'invites de permission, l'exécutez sans surveillance ou le pointez vers du code en lequel vous n'avez pas entièrement confiance.

Claude Code peut s'exécuter dans plusieurs types d'environnements isolés, allant d'un sandbox léger par commande à une machine virtuelle entièrement séparée. Cette page compare ces options selon ce qu'elles isolent et ce qu'elles nécessitent, vous aide à en choisir une pour votre modèle de menace et montre comment appliquer ce choix dans toute une organisation.

<Info>
  Pour le modèle de sécurité plus large, voir [Sécurité](/docs/fr/security). Pour les déploiements Agent SDK, voir [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment).
</Info>

<h2 id="compare-sandboxing-approaches">
  Comparer les approches de sandboxing
</h2>

Les deux premières approches du tableau ci-dessous s'exécutent sur le système d'exploitation hôte sans conteneurs. Les autres placent Claude Code à l'intérieur d'un conteneur ou d'une machine virtuelle.

| Approche                                      | Ce qui est isolé                                                                                     | Nécessite Docker | Effort de configuration                                                                                       |
| :-------------------------------------------- | :--------------------------------------------------------------------------------------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------ |
| [Outil Bash en sandbox](#sandboxed-bash-tool) | Commandes Bash, PowerShell et Monitor et leurs processus enfants                                     | Non              | Minimal sur macOS ; faible sur Linux et WSL2                                                                  |
| [Runtime sandbox](#sandbox-runtime)           | L'ensemble du processus Claude Code, y compris les outils de fichiers, les serveurs MCP et les hooks | Non              | Faible                                                                                                        |
| [Conteneur de développement](#dev-containers) | Environnement de développement complet                                                               | Oui              | Moyen                                                                                                         |
| [Conteneur personnalisé](#custom-container)   | Environnement de développement complet                                                               | Oui              | Moyen à élevé                                                                                                 |
| [Machine virtuelle](#virtual-machine)         | Système d'exploitation complet                                                                       | Non              | Élevé                                                                                                         |
| [Sessions cloud](#cloud-sessions)             | Système d'exploitation complet, hébergé par Anthropic                                                | Non              | Aucun ; nécessite un abonnement Claude et un compte GitHub connecté sauf si vous lancez avec `claude --cloud` |

L'[outil Bash en sandbox](/docs/fr/sandboxing) est intégré à Claude Code et restreint les commandes Bash. Les outils de fichiers intégrés, les serveurs MCP et les hooks s'exécutent toujours directement sur votre hôte. Toutes les autres approches du tableau placent l'ensemble du processus Claude Code à l'intérieur de la limite d'isolation, de sorte que les outils de fichiers, les serveurs MCP et les hooks sont également restreints.

<Warning>
  L'isolation du sandbox réduit l'impact d'une violation, mais elle n'élimine pas le risque. Toute approche qui permet la sortie réseau peut toujours divulguer les données que l'agent peut lire, et toute approche qui monte votre répertoire de projet en écriture peut toujours modifier ce code. Consultez les [limitations de sécurité](/docs/fr/sandboxing#security-limitations) avant de vous fier à un sandbox comme contrôle strict.

  L'isolation ne change pas non plus ce qui est envoyé au modèle. Vos invites et les fichiers que Claude lit sont transmis à l'API Anthropic ou à votre fournisseur configuré avec ou sans sandbox. Consultez [Utilisation des données](/docs/fr/data-usage) pour savoir ce que Claude Code envoie et comment le réduire.
</Warning>

<h2 id="choose-an-approach">
  Choisir une approche
</h2>

Faites correspondre votre objectif à une ligne ci-dessous, puis lisez la section de détail qui suit.

| Vous voulez                                                                                       | Commencez par                                                                                                                                                                          |
| :------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Réduire les invites de permission pendant le travail quotidien sur votre propre machine           | L'[outil Bash sandboxé](/docs/fr/sandboxing), configuré avec `/sandbox`                                                                                                                     |
| Laisser Claude travailler sans surveillance avec `--dangerously-skip-permissions` ou en mode auto | Le [dev container](/docs/fr/devcontainer) préconfiguré, n'importe quel conteneur ou VM, ou le [runtime sandbox](#sandbox-runtime)                                                           |
| Isoler les serveurs MCP et les hooks ainsi que Bash, sans Docker                                  | Le runtime sandbox                                                                                                                                                                     |
| Travailler sur un référentiel non fiable                                                          | Une machine virtuelle dédiée, ou une [session cloud](/docs/fr/claude-code-on-the-web) si vous avez un abonnement Claude ; GitHub n'est pas requis lorsque vous lancez avec `claude --cloud` |
| Standardiser un environnement sandboxé dans une équipe                                            | Le [dev container](/docs/fr/devcontainer) préconfiguré, copié dans votre référentiel                                                                                                        |
| Utiliser Claude Code à partir d'un appareil sans configuration locale                             | Une [session cloud](/docs/fr/claude-code-on-the-web), qui nécessite un abonnement Claude et un compte GitHub connecté                                                                       |
| Exiger l'isolation pour chaque développeur de votre organisation                                  | [Appliquer l'isolation dans une organisation](#enforce-isolation-across-an-organization)                                                                                               |
| Travailler sur un hôte Windows natif                                                              | Un conteneur ou une VM, ou exécutez le sandbox Bash à l'intérieur de WSL2                                                                                                              |

<h3 id="how-isolation-relates-to-permission-modes">
  Comment l'isolation se rapporte aux modes de permission
</h3>

Les [modes de permission](/docs/fr/permission-modes) décident si un appel d'outil s'exécute et si vous êtes invité en premier. L'isolation restreint ce qu'une commande peut accéder une fois qu'elle s'exécute. Les deux fonctionnent ensemble : lorsqu'un mode de permission laisse les actions s'exécuter sans vous demander, une limite d'isolation restreint ce que ces actions peuvent atteindre.

Lorsque vous passez `--dangerously-skip-permissions`, Claude agit sans vous demander d'abord. Les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves) s'appliquent toujours.

Sans invites pour attraper les erreurs, la limite d'isolation que vous choisissez est ce qui protège votre système. Exécutez toujours les sessions `--dangerously-skip-permissions` à l'intérieur d'un conteneur, d'une VM, ou du [runtime sandbox](#sandbox-runtime), afin que les outils de fichiers, les serveurs MCP et les hooks soient également à l'intérieur de la limite. Sur Linux et macOS, Claude Code refuse de démarrer avec ce drapeau lorsqu'il s'exécute en tant que root, donc exécutez le conteneur, la VM, ou le runtime sandbox en tant qu'utilisateur non-root.

Le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) remplace l'invite par un classificateur qui examine les actions. Le classificateur est un contrôle par action, pas une limite d'isolation, donc une limite d'isolation ajoute toujours une défense en profondeur pour les exécutions sans surveillance, et n'est pas requise comme elle l'est pour `--dangerously-skip-permissions`.

L'[outil Bash sandboxé](#sandboxed-bash-tool) seul contraint uniquement les commandes shell, donc il n'est pas suffisant pour les exécutions entièrement sans surveillance dans l'un ou l'autre mode. Vous pouvez superposer les approches : exécuter l'outil Bash sandboxé à l'intérieur d'un conteneur ou d'une VM vous donne des restrictions de commande au niveau du système d'exploitation en plus de la limite d'environnement externe. Pour savoir comment le sandbox Bash lui-même interagit avec les règles de permission et les modes de permission, voir [Comment le sandboxing se rapporte aux permissions et aux modes de permission](/docs/fr/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Outil Bash sandboxé
</h2>

<Note>
  Cette option ne supporte pas Windows natif. Sur les hôtes Windows, utilisez WSL2 ou l'une des approches de conteneur ou VM ci-dessous.
</Note>

L'outil Bash sandboxé est intégré à Claude Code. Il utilise des primitives du système d'exploitation pour restreindre l'accès au système de fichiers et au réseau de chaque commande Bash, PowerShell ou Monitor que Claude exécute.

Exécutez la commande `/sandbox` pour ouvrir le panneau sandbox et choisir un mode. Le guide [Sandboxing](/docs/fr/sandboxing) couvre les modes d'approbation, la limite par défaut et comment l'élargir ou la réduire.

Le sandbox par commande ne couvre pas tout ce qui s'exécute dans une session :

* D'autres [outils intégrés](/docs/fr/tools-reference) tels que Read, Edit et WebFetch s'exécutent à l'intérieur du processus Claude Code et ne génèrent pas de code arbitraire. Les [règles de permission](/docs/fr/permissions) pour le chemin ou le domaine les contrôlent à la place.
* Les serveurs [MCP](/docs/fr/mcp) et les [hooks de commande](/docs/fr/hooks#command-hook-fields) sont des processus séparés qui s'exécutent sans contrainte sur l'hôte.

Pour mettre les outils intégrés, les serveurs MCP et les hooks tous derrière une limite du système d'exploitation, exécutez l'ensemble du processus Claude Code à l'intérieur du [runtime sandbox](#sandbox-runtime), du [dev container](#dev-containers) ou d'un [conteneur personnalisé](#custom-container).

<h2 id="sandbox-runtime">
  Runtime sandbox
</h2>

Le package [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) enveloppe un processus entier dans la même isolation Seatbelt ou bubblewrap que le sandbox Bash intégré utilise. Exécuter Claude Code à travers le runtime contraint chaque outil, hook et serveur MCP dans la session, pas seulement les commandes shell. Le runtime est un aperçu de recherche bêta, et son format de configuration peut changer à mesure que le package évolue.

Cette section couvre ce que vous configurez et ce que le runtime applique de lui-même. Pour déployer le runtime dans les applications Agent SDK, consultez le [guide de déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment#sandbox-runtime).

<h3 id="set-up-and-launch-the-runtime">
  Configurer et lancer le runtime
</h3>

Sur Linux et WSL2, le runtime dépend des mêmes packages `bubblewrap` et `socat` que le sandbox intégré, plus `ripgrep`, que Claude Code regroupe mais que le runtime autonome résout à partir de votre PATH. Installez `bubblewrap` et `socat` comme décrit dans [Configurer Linux et WSL2](/docs/fr/sandboxing#set-up-linux-and-wsl2), et `ripgrep` à partir du gestionnaire de packages de votre distribution. Sur macOS, vous n'avez besoin d'aucun package supplémentaire. Le runtime utilise le sandbox Seatbelt intégré là-bas.

Par défaut, le runtime refuse l'accès réseau et confine les écritures à un petit ensemble de chemins runtime intégrés, donc configurez-le avant de lancer Claude Code à travers lui. Mettez votre configuration dans `~/.srt-settings.json`, ou dans un fichier que vous passez avec `--settings`. Le [README](https://github.com/anthropic-experimental/sandbox-runtime) du package documente le schéma de configuration complet.

Autorisez l'accès en écriture à au moins :

* Votre répertoire de projet.
* Les chemins de configuration de Claude Code `~/.claude` et `~/.claude.json`.
* `/tmp`, où Claude Code écrit les fichiers runtime.

Autorisez les domaines réseau dont votre session a besoin :

* `api.anthropic.com`, ou le point de terminaison de votre fournisseur configuré. Sur un fournisseur tiers, conservez également `api.anthropic.com` : la vérification de sécurité du domaine WebFetch l'appelle toujours par défaut sauf si vous définissez `skipWebFetchPreflight: true`.
* `claude.ai` et `platform.claude.com`, que [la connexion OAuth et l'actualisation des tokens](/docs/fr/network-config#network-access-requirements) nécessitent. Les exécutions authentifiées avec une clé API peuvent abandonner ces deux.

Sur Linux et WSL2, le runtime applique les autorisations d'écriture uniquement aux chemins qui existent déjà. Dans un environnement vierge, créez les chemins de configuration de Claude Code avant le premier lancement :

```bash theme={null}
mkdir -p ~/.claude && echo '{}' > ~/.claude.json
```

Une fois le fichier de paramètres en place, lancez Claude Code avec `npx` et passez `claude` comme commande à envelopper :

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code démarre à l'intérieur du sandbox avec les limites de système de fichiers et réseau que vous avez configurées. La même commande fonctionne pour sandboxer les serveurs MCP autonomes ou d'autres processus d'aide.

<h3 id="what-the-runtime-blocks-on-its-own">
  Ce que le runtime bloque de lui-même
</h3>

Le runtime bloque les écritures à plus haut risque sans aucune configuration de votre part :

* `denyWrite` prend précédence sur `allowWrite`.
* À la racine du projet, le runtime refuse `.git/hooks`, refuse `.git/config` sauf si vous définissez `filesystem.allowGitConfig: true`, et refuse `.mcp.json`, `.claude/commands`, `.claude/agents`, et les fichiers de démarrage du shell.
* Sur macOS, ces refus sont vérifiés quand une écriture se produit, donc ils couvrent également les fichiers imbriqués et les référentiels créés pendant la session.
* Sur Linux et WSL2, le runtime construit la liste de refus une fois au lancement. Il couvre de manière fiable la racine du projet, effectue une analyse superficielle de meilleur effort pour les copies imbriquées qui existent à ce moment, et ne couvre rien que la session crée plus tard, comme `git init`, `git clone`, ou l'échafaudage. La section `mandatoryDenySearchDepth` du README décrit la sémantique exacte de l'analyse.
* Sans un `~/.srt-settings.json` valide, le runtime démarre quand même, bloque l'accès réseau, et confine les écritures aux chemins runtime intégrés tels que `/tmp/claude`, `~/.npm/_logs`, et `~/.claude/debug`. Ne prenez pas un démarrage propre comme preuve que vos paramètres ont été chargés.
* Quand vous passez `--settings`, le runtime refuse de démarrer si le fichier échoue à charger.

Vos autorisations d'écriture incluent toujours d'autres chemins à partir desquels Claude Code charge la configuration, donc refusez-les avec `denyWrite`. Une session sandboxée qui peut les écrire peut persister des hooks, des règles de permission, ou des serveurs MCP qui s'exécutent sans sandbox la prochaine fois que vous lancez Claude Code.

<h3 id="after-unattended-runs">
  Après les exécutions sans surveillance
</h3>

Examinez les chemins que vous avez gardés accessibles en écriture. Sur Linux et WSL2, examinez également tout ce que la session a créé.

<h2 id="dev-containers">
  Dev containers
</h2>

Un dev container exécute Claude Code à l'intérieur d'un conteneur Docker que VS Code ou un éditeur compatible gère, avec votre projet monté dedans. Vous pouvez en définir un avec un répertoire `.devcontainer/` dans votre référentiel.

Le référentiel claude-code publie un [exemple de dev container](/docs/fr/devcontainer) avec un pare-feu iptables par défaut-refus comme point de départ. Copiez-le dans votre référentiel et ajustez la liste blanche du pare-feu, l'image de base et la version épinglée de Claude Code pour correspondre à votre environnement. Parce que le pare-feu bloque l'egress non approuvé, une configuration comme celle-ci supporte l'exécution de Claude Code avec `--dangerously-skip-permissions` pour le travail sans surveillance.

<h2 id="custom-container">
  Conteneur personnalisé
</h2>

Vous pouvez exécuter Claude Code dans n'importe quelle image de conteneur Docker ou OCI avec vos propres politiques réseau, volumes montés et profils seccomp. C'est le chemin le plus courant pour les organisations avec une infrastructure de conteneur existante ou des exécuteurs CI.

Plusieurs services de sandbox gérés et d'exécution à distance peuvent héberger le conteneur pour vous. La même liste de contrôle s'applique que pour n'importe quel conteneur que vous exploitez : examinez ce qui est monté en écriture, quelles credentials et tokens sont accessibles à l'intérieur, et ce que la politique d'egress réseau permet.

Vous pouvez superposer le sandbox Bash intégré à l'intérieur du conteneur pour les restrictions par commande. Les conteneurs non privilégiés ont besoin du paramètre nested-sandbox décrit dans [Dépannage du sandboxing](/docs/fr/sandboxing#troubleshooting).

<h2 id="virtual-machine">
  Machine virtuelle
</h2>

Une machine virtuelle dédiée fournit la séparation la plus forte, avec son propre noyau et, dans les déploiements cloud ou microVM, son propre matériel virtualisé. Les options incluent les instances cloud, les hyperviseurs locaux et les microVMs tels que Firecracker. Utilisez cette approche lorsque vous évaluez du code non fiable, lorsque votre politique de sécurité nécessite une séparation au niveau du noyau entre l'agent et l'hôte, ou lorsqu'aucune approche au niveau de l'hôte ne répond à vos exigences de conformité.

[Docker Sandboxes](https://docs.docker.com/ai/sandboxes/) fournit une microVM avec son propre daemon Docker et synchronisation d'espace de travail, qui peut exécuter Claude Code sur n'importe quel hôte avec Docker Sandboxes installé. C'est un produit gratuit et autonome de Docker qui ne nécessite pas Docker Desktop.

<h2 id="cloud-sessions">
  Sessions cloud
</h2>

Une [session cloud](/docs/fr/claude-code-on-the-web) s'exécute dans une machine virtuelle isolée gérée par Anthropic. Un proxy réseau applique une liste blanche par défaut, et un proxy séparé détient votre token GitHub en dehors du sandbox tout en émettant des credentials limités pour l'accès au référentiel à l'intérieur. Les sessions que votre organisation achemine vers un [environnement auto-hébergé](/docs/fr/self-hosted-environments) s'exécutent sur l'infrastructure que vous provisionnez à la place, où l'isolation, le contrôle de sortie et les credentials git relèvent de la responsabilité de votre déploiement.

Utilisez cette approche lorsque vous voulez une isolation VM complète sans provisionner l'infrastructure vous-même, ou lorsque vous déléguez des tâches à partir d'un appareil qui n'a pas d'environnement de développement local. Cela nécessite un abonnement Claude. Sauf si vous lancez à partir de la CLI, vous avez également besoin d'un compte GitHub connecté pour que le sandbox puisse cloner votre référentiel. Lorsque vous lancez à partir de la CLI avec `--cloud`, Claude Code peut [regrouper et télécharger votre référentiel local](/docs/fr/claude-code-on-the-web#send-local-repositories-without-github) à la place. Voir [Utiliser Claude Code dans le cloud](/docs/fr/claude-code-on-the-web) pour la disponibilité du plan et les options d'authentification GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Appliquer l'isolation dans une organisation
</h2>

Les développeurs individuels peuvent opter pour n'importe quelle approche de sandboxing sur cette page. Ce qu'une organisation peut appliquer, et avec quels outils, dépend de l'approche :

* **Sandbox Bash intégré** : la seule approche que Claude Code applique lui-même. Livrez les clés de paramètres `sandbox` via les [paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms), soit comme un fichier géré par votre MDM, soit via les [paramètres gérés par serveur](/docs/fr/server-managed-settings) sur Claude.ai. Voir [Appliquer le sandboxing avec les paramètres gérés](/docs/fr/sandboxing#enforce-sandboxing-with-managed-settings) pour les clés à déployer et comment empêcher les développeurs d'élargir la politique.
* **Dev containers** : validez l'[exemple de dev container](/docs/fr/devcontainer) dans vos référentiels pour standardiser l'environnement dans une équipe. C'est une convention plutôt qu'une limite d'application, car Claude Code ne nécessite pas de conteneur. Si les développeurs ne doivent pas pouvoir exécuter Claude Code en dehors, appliquez cela avec les outils de gestion d'appareils ou de liste blanche de logiciels de votre organisation.
* **Conteneurs personnalisés et VMs** : distribuez Claude Code via l'image approuvée et utilisez les outils de gestion d'appareils ou de liste blanche de logiciels de votre organisation pour empêcher l'installation en dehors.

<h2 id="see-also">
  Voir aussi
</h2>

Ces pages couvrent les détails de configuration et de politique pour les approches de sandboxing sur cette page.

* [Sandboxing](/docs/fr/sandboxing) : configurez l'outil Bash sandboxé intégré
* [Dev container](/docs/fr/devcontainer) : le conteneur de développement Docker préconfiguré
* [Sécurité](/docs/fr/security) : le modèle de sécurité complet de Claude Code
* [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment) : conseils d'isolation pour les applications Agent SDK
* [Paramètres](/docs/fr/settings-reference#sandbox-settings) : toutes les clés de configuration sandbox, y compris la livraison de paramètres gérés
