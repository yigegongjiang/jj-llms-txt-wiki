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

| Approche                                          | Ce qui est isolé                                                                                     | Nécessite Docker | Effort de configuration                          |
| :------------------------------------------------ | :--------------------------------------------------------------------------------------------------- | :--------------- | :----------------------------------------------- |
| [Outil Bash sandboxé](#sandboxed-bash-tool)       | Commandes Bash et leurs processus enfants                                                            | Non              | Minimal sur macOS ; faible sur Linux et WSL2     |
| [Runtime sandbox](#sandbox-runtime)               | L'ensemble du processus Claude Code, y compris les outils de fichiers, les serveurs MCP et les hooks | Non              | Faible                                           |
| [Dev container](#dev-containers)                  | Environnement de développement complet                                                               | Oui              | Moyen                                            |
| [Conteneur personnalisé](#custom-container)       | Environnement de développement complet                                                               | Oui              | Moyen à élevé                                    |
| [Machine virtuelle](#virtual-machine)             | Système d'exploitation complet                                                                       | Non              | Élevé                                            |
| [Claude Code sur le web](#claude-code-on-the-web) | Système d'exploitation complet, hébergé par Anthropic                                                | Non              | Aucun ; nécessite un abonnement Claude et GitHub |

L'[outil Bash sandboxé](/docs/fr/sandboxing) est intégré à Claude Code et restreint uniquement les commandes Bash. Les outils de fichiers intégrés, les serveurs MCP et les hooks s'exécutent toujours directement sur votre hôte. Chaque autre approche du tableau place l'ensemble du processus Claude Code à l'intérieur de la limite d'isolation, donc les outils de fichiers, les serveurs MCP et les hooks sont également restreints.

<Warning>
  L'isolation sandbox réduit l'impact d'une violation, mais elle n'élimine pas le risque. Toute approche qui permet l'egress réseau peut toujours fuir les données que l'agent peut lire, et toute approche qui monte votre répertoire de projet en écriture peut toujours modifier ce code. Consultez les [limitations de sécurité](/docs/fr/sandboxing#security-limitations) avant de compter sur un sandbox comme contrôle strict.

  L'isolation ne change pas non plus ce qui est envoyé au modèle. Vos invites et les fichiers que Claude lit sont transmis à l'API Anthropic ou à votre fournisseur configuré avec ou sans sandbox. Voir [Utilisation des données](/docs/fr/data-usage) pour ce que Claude Code envoie et comment le réduire.
</Warning>

<h2 id="choose-an-approach">
  Choisir une approche
</h2>

Faites correspondre votre objectif à une ligne ci-dessous, puis lisez la section de détail qui suit.

| Vous voulez                                                                                       | Commencez par                                                                                                                                        |
| :------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| Réduire les invites de permission pendant le travail quotidien sur votre propre machine           | L'[outil Bash sandboxé](/docs/fr/sandboxing), activé avec `/sandbox`                                                                                      |
| Laisser Claude travailler sans surveillance avec `--dangerously-skip-permissions` ou en mode auto | Le [dev container](/docs/fr/devcontainer) préconfiguré, n'importe quel conteneur ou VM, ou le [runtime sandbox](#sandbox-runtime)                         |
| Isoler les serveurs MCP et les hooks ainsi que Bash, sans Docker                                  | Le runtime sandbox                                                                                                                                   |
| Travailler sur un référentiel non fiable                                                          | Une machine virtuelle dédiée, ou [Claude Code sur le web](/docs/fr/claude-code-on-the-web) si vous avez un abonnement Claude et un compte GitHub connecté |
| Standardiser un environnement sandboxé dans une équipe                                            | Le [dev container](/docs/fr/devcontainer) préconfiguré, copié dans votre référentiel                                                                      |
| Utiliser Claude Code à partir d'un appareil sans configuration locale                             | [Claude Code sur le web](/docs/fr/claude-code-on-the-web), qui nécessite un abonnement Claude et un compte GitHub connecté                                |
| Exiger l'isolation pour chaque développeur de votre organisation                                  | [Appliquer l'isolation dans une organisation](#enforce-isolation-across-an-organization)                                                             |
| Travailler sur un hôte Windows natif                                                              | Un conteneur ou une VM, ou exécutez le sandbox Bash à l'intérieur de WSL2                                                                            |

<h3 id="how-isolation-relates-to-permission-modes">
  Comment l'isolation se rapporte aux modes de permission
</h3>

Les [modes de permission](/docs/fr/permission-modes) décident si un appel d'outil s'exécute et si vous êtes invité en premier. L'isolation restreint ce qu'une commande peut accéder une fois qu'elle s'exécute. Les deux fonctionnent ensemble : lorsqu'un mode de permission laisse les actions s'exécuter sans vous demander, une limite d'isolation restreint ce que ces actions peuvent atteindre.

Lorsque vous passez `--dangerously-skip-permissions`, Claude agit sans vous demander d'abord ; vous êtes uniquement invité pour les [règles ask](/docs/fr/permissions#manage-permissions) explicites, les outils connecteurs [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool), et les suppressions ciblant `/` ou votre répertoire personnel. Sans invites pour attraper les erreurs, la limite d'isolation que vous choisissez est ce qui protège votre système. Exécutez toujours les sessions `--dangerously-skip-permissions` à l'intérieur d'un conteneur, d'une VM, ou du [runtime sandbox](#sandbox-runtime), afin que les outils de fichiers, les serveurs MCP et les hooks soient également à l'intérieur de la limite.

Le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) remplace l'invite par un classificateur qui examine les actions et bloque celles qui escaladent au-delà de la demande, ciblent une infrastructure non reconnue ou semblent motivées par du contenu hostile que Claude a lu. Le classificateur est un contrôle par action, pas une limite d'isolation, donc une limite d'isolation ajoute toujours une défense en profondeur pour les exécutions sans surveillance, et n'est pas requise comme elle l'est pour `--dangerously-skip-permissions`.

L'[outil Bash sandboxé](#sandboxed-bash-tool) seul contraint uniquement Bash, donc il n'est pas suffisant pour les exécutions entièrement sans surveillance dans l'un ou l'autre mode. Vous pouvez superposer les approches : exécuter l'outil Bash sandboxé à l'intérieur d'un conteneur ou d'une VM vous donne des restrictions de commande au niveau du système d'exploitation en plus de la limite d'environnement externe. Pour savoir comment le sandbox Bash lui-même interagit avec les règles de permission et les modes de permission, voir [Comment le sandboxing se rapporte aux permissions et aux modes de permission](/docs/fr/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Outil Bash sandboxé
</h2>

<Note>
  Cette option ne supporte pas Windows natif. Sur les hôtes Windows, utilisez WSL2 ou l'une des approches de conteneur ou VM ci-dessous.
</Note>

L'outil Bash sandboxé est intégré à Claude Code. Il utilise des primitives du système d'exploitation pour restreindre l'accès au système de fichiers et au réseau de chaque commande Bash que Claude exécute : Seatbelt, le sandbox macOS intégré, et [bubblewrap](https://github.com/containers/bubblewrap) sur Linux et WSL2. Par défaut, il autorise les écritures dans le répertoire de travail et invite la première fois qu'une commande a besoin d'un nouveau domaine réseau.

Activez-le avec la commande `/sandbox`. Le guide [Sandboxing](/docs/fr/sandboxing) couvre les modes d'approbation, la limite par défaut et comment l'élargir ou la réduire.

Le sandbox par commande ne couvre pas tout ce qui s'exécute dans une session :

* D'autres [outils intégrés](/docs/fr/tools-reference) tels que Read, Edit et WebFetch s'exécutent à l'intérieur du processus Claude Code et ne génèrent pas de code arbitraire. Les [règles de permission](/docs/fr/permissions) pour le chemin ou le domaine les contrôlent à la place.
* Les serveurs [MCP](/docs/fr/mcp) et les hooks sont des processus séparés qui s'exécutent sans contrainte sur l'hôte.

Pour mettre les outils intégrés, les serveurs MCP et les hooks tous derrière une limite du système d'exploitation, exécutez l'ensemble du processus Claude Code à l'intérieur du [runtime sandbox](#sandbox-runtime), du [dev container](#dev-containers) ou d'un [conteneur personnalisé](#custom-container).

<h2 id="sandbox-runtime">
  Runtime sandbox
</h2>

Le package [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) enveloppe un processus entier dans la même isolation Seatbelt ou bubblewrap que le sandbox Bash intégré utilise. Exécuter Claude Code à travers lui contraint chaque outil, hook et serveur MCP dans la session, pas seulement Bash. Le runtime est un aperçu de recherche bêta, et son format de configuration peut changer à mesure que le package évolue.

Le runtime refuse tout accès en écriture et réseau par défaut, donc configurez-le avant de lancer Claude Code à travers lui. Dans `~/.srt-settings.json`, ou un fichier que vous passez avec `--settings`, autorisez l'accès en écriture à au moins votre répertoire de projet et aux chemins de configuration de Claude Code `~/.claude` et `~/.claude.json`. Autorisez les domaines réseau dont votre session a besoin, y compris `api.anthropic.com` ou le point de terminaison de votre fournisseur configuré. Voir le [README](https://github.com/anthropic-experimental/sandbox-runtime) du package pour le schéma de configuration complet.

Une fois le fichier de paramètres en place, lancez Claude Code avec `npx` et passez `claude` comme commande à envelopper :

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code démarre à l'intérieur du sandbox avec les limites de système de fichiers et réseau que vous avez configurées. La même commande fonctionne pour sandboxer les serveurs MCP autonomes ou d'autres processus d'aide.

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

Une machine virtuelle dédiée fournit la séparation la plus forte, avec son propre noyau et, dans les déploiements cloud ou microVM, son propre matériel virtualisé. Les options incluent les instances cloud, les hyperviseurs locaux et les microVMs tels que Firecracker.

Utilisez cette approche lorsque vous évaluez du code non fiable, lorsque votre politique de sécurité nécessite une séparation au niveau du noyau entre l'agent et l'hôte, ou lorsqu'aucune approche au niveau de l'hôte ne répond à vos exigences de conformité. La fonctionnalité [sandboxes](https://docs.docker.com/ai/sandboxes/) de Docker Desktop fournit une microVM avec son propre daemon Docker et synchronisation d'espace de travail, qui peut exécuter Claude Code sur les hôtes qui ont déjà Docker Desktop.

<h2 id="claude-code-on-the-web">
  Claude Code sur le web
</h2>

[Claude Code sur le web](/docs/fr/claude-code-on-the-web) exécute chaque session dans une machine virtuelle isolée gérée par Anthropic. Un proxy réseau applique une liste blanche par défaut, et un proxy séparé détient votre token GitHub en dehors du sandbox tout en émettant des credentials limités pour l'accès au référentiel à l'intérieur.

Utilisez cette approche lorsque vous voulez une isolation VM complète sans provisionner l'infrastructure vous-même, ou lorsque vous déléguez des tâches à partir d'un appareil qui n'a pas d'environnement de développement local. Cela nécessite un abonnement Claude et un compte GitHub connecté, et les sessions clonent votre référentiel depuis GitHub. Voir [Claude Code sur le web](/docs/fr/claude-code-on-the-web) pour la disponibilité du plan et les options d'authentification GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Appliquer l'isolation dans une organisation
</h2>

Les développeurs individuels peuvent opter pour n'importe quelle approche ci-dessus. Ce qu'une organisation peut appliquer, et avec quels outils, dépend de l'approche :

* **Sandbox Bash intégré** : la seule approche que Claude Code applique lui-même. Livrez les clés de paramètres `sandbox` via les [paramètres gérés](/docs/fr/settings#settings-files), soit comme un fichier géré par votre MDM, soit via les [paramètres gérés par serveur](/docs/fr/server-managed-settings) sur Claude.ai. Voir [Appliquer le sandboxing avec les paramètres gérés](/docs/fr/sandboxing#enforce-sandboxing-with-managed-settings) pour les clés à déployer et comment empêcher les développeurs d'élargir la politique.
* **Dev containers** : validez l'[exemple de dev container](/docs/fr/devcontainer) dans vos référentiels pour standardiser l'environnement dans une équipe. C'est une convention plutôt qu'une limite d'application, car Claude Code ne nécessite pas de conteneur. Si les développeurs ne doivent pas pouvoir exécuter Claude Code en dehors, appliquez cela avec les outils de gestion d'appareils ou de liste blanche de logiciels de votre organisation.
* **Conteneurs personnalisés et VMs** : distribuez Claude Code via l'image approuvée et utilisez les outils de gestion d'appareils ou de liste blanche de logiciels de votre organisation pour empêcher l'installation en dehors.

<h2 id="see-also">
  Voir aussi
</h2>

Ces pages couvrent les détails de configuration et de politique pour les approches ci-dessus.

* [Sandboxing](/docs/fr/sandboxing) : configurez l'outil Bash sandboxé intégré
* [Dev container](/docs/fr/devcontainer) : le conteneur de développement Docker préconfiguré
* [Sécurité](/docs/fr/security) : le modèle de sécurité complet de Claude Code
* [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment) : conseils d'isolation pour les applications Agent SDK
* [Paramètres](/docs/fr/settings#sandbox-settings) : toutes les clés de configuration sandbox, y compris la livraison de paramètres gérés
