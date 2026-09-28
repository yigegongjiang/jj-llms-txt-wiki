> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Environnements auto-hébergés

> Exécutez les sessions cloud Claude Code sur l'infrastructure que vous contrôlez : configurez un environnement auto-hébergé, déployez des runners, et routez les sessions vers votre propre calcul.

<Note>
  Les environnements auto-hébergés sont en bêta publique sur les plans Team et Enterprise et sont désactivés par défaut. Consultez [Disponibilité et limitations](#availability-and-limitations) pour le chemin d'activation et ce qui est exclu.
</Note>

Un environnement auto-hébergé exécute les sessions cloud Claude Code sur l'infrastructure que votre organisation exploite. Une [session cloud](/docs/fr/claude-code-on-the-web) est toute session qui s'exécute ailleurs que sur la machine du développeur : les développeurs les démarrent à partir de claude.ai, des applications mobiles et de bureau, du terminal avec [`claude --cloud`](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud), et des [routines planifiées](/docs/fr/routines), et par défaut elles s'exécutent sur l'infrastructure d'Anthropic. Dans un environnement auto-hébergé, ces mêmes sessions s'exécutent à l'intérieur de votre réseau, et l'expérience développeur est par ailleurs la même à part les différences dans [Disponibilité et limitations](#availability-and-limitations) et les [problèmes connus](/docs/fr/self-hosted-environments-deploy#known-issues-and-limitations) de la page de déploiement.

Si votre équipe n'utilise pas les sessions cloud, il n'y a rien à configurer ici : les sessions dans un terminal ou un IDE s'exécutent toujours sur la machine du développeur. Si vous voulez exécuter Claude Code sur votre propre machine toujours active et la piloter à partir d'autres appareils, utilisez [Contrôle à distance](/docs/fr/remote-control), qui est également disponible sur les plans Pro et Max. Quand vous êtes prêt à configurer, allez directement au [démarrage rapide](/docs/fr/self-hosted-environments-quickstart) ; pour examiner d'abord la posture de sécurité, commencez par [Déployer en production](/docs/fr/self-hosted-environments-deploy). Le reste de cette page explique comment fonctionne l'auto-hébergement et quand le choisir.

<h2 id="how-self-hosted-environments-work">
  Comment fonctionnent les environnements auto-hébergés
</h2>

L'auto-hébergement comporte trois parties :

* **Environnement** : une destination nommée vers laquelle les sessions cloud peuvent être envoyées. Votre organisation crée des environnements dans les paramètres d'administration de claude.ai, et chacun regroupe un ensemble de runners.
* **Runner** : un programme s'exécutant sur des hôtes à l'intérieur de votre réseau. Les runners exécutent les sessions ; l'idée est la même qu'un runner CI auto-hébergé.
* **Session** : une tâche Claude Code qu'un développeur a démarrée.

Quand un développeur démarre une session cloud, l'interface de démarrage de session affiche un sélecteur d'environnement listant les environnements hébergés par Anthropic aux côtés de ceux que votre organisation a créés. S'ils choisissent le vôtre, le plan de contrôle d'Anthropic place la session dans la file d'attente de votre environnement, où un runner la réclame, clone le référentiel que le développeur a choisi, et démarre un processus Claude Code sur votre hôte pour l'exécuter. Le runner s'authentifie auprès de votre hôte git avec les identifiants que vous configurez ; [Configurer git](/docs/fr/self-hosted-environments-deploy#configure-git) couvre les options. Les sessions atteignent vos services internes de l'intérieur de votre réseau, et votre hôte git de la même manière quand il est interne ; le trafic vers Anthropic, l'interrogation de la file d'attente, le flux d'événements de la session, et l'inférence du modèle, est HTTPS sortant vers `api.anthropic.com`, avec la courte liste des hôtes supplémentaires que les sessions peuvent atteindre dans [Exigences réseau](/docs/fr/self-hosted-environments-deploy#network-requirements). Anthropic ne se connecte jamais à votre réseau.

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="Diagramme d'architecture d'un environnement auto-hébergé : la limite de votre réseau contient un runner, deux processus de session Claude Code à l'intérieur, et votre hôte git, avec api.anthropic.com à l'extérieur contenant la file d'attente, le flux de session, et l'inférence. Le runner interroge la file d'attente et atteint l'hôte git, chaque processus de session ouvre ses propres connexions de flux, d'inférence et de git, et chaque connexion est sortante de votre réseau, sans aucune entrante." width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="Diagramme d'architecture d'un environnement auto-hébergé : la limite de votre réseau contient un runner, deux processus de session Claude Code à l'intérieur, et votre hôte git, avec api.anthropic.com à l'extérieur contenant la file d'attente, le flux de session, et l'inférence. Le runner interroge la file d'attente et atteint l'hôte git, chaque processus de session ouvre ses propres connexions de flux, d'inférence et de git, et chaque connexion est sortante de votre réseau, sans aucune entrante." width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

Les deux boîtes Claude Code dans le diagramme sont des processus de session : un runner exécutant deux sessions à la fois, jusqu'à sa capacité configurée. Un runner sert un [propriétaire](#key-concepts) à la fois et se verrouille à ce propriétaire quand il réclame sa première session, donc le code extrait ne se mélange jamais entre les propriétaires ; [Cycle de vie du runner](#runner-lifecycle) couvre la règle.

Vous pouvez démarrer les runners vous-même et les maintenir en fonctionnement, ou exécuter l'[orchestrateur de mise à l'échelle automatique](/docs/fr/self-hosted-environments-configuration#on-demand-runners), un deuxième processus que vous hébergez, qui démarre les runners à mesure que les sessions s'accumulent ; chaque runner se termine de lui-même quand son travail est terminé. De toute façon, vous configurez l'environnement une fois, et il apparaît dans le sélecteur sur chaque surface prise en charge.

<h2 id="availability-and-limitations">
  Disponibilité et limitations
</h2>

Vérifiez ceci avant de planifier un déploiement :

* **Plans** : bêta publique pour les organisations Team et Enterprise. Les environnements auto-hébergés sont désactivés par défaut ; un [Propriétaire](/docs/fr/cloud-environments#organization-shared-environments) active **Autoriser les environnements auto-hébergés** sur la [page d'administration **Environnements cloud**](https://claude.ai/admin-settings/cloud-environments), ce qui nécessite que [les sessions cloud](/docs/fr/claude-code-on-the-web) soient activées pour l'organisation.
* **Zéro rétention de données** : indisponible pour les organisations avec [Zéro rétention de données](/docs/fr/zero-data-retention) activée.
* **Inférence du modèle** : les sessions utilisent l'API Anthropic, et l'inférence ne peut pas être routée via [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/fr/third-party-integrations), ou une [passerelle LLM](/docs/fr/llm-gateway).
* **Surfaces** : les sessions démarrées à partir de [claude.ai/code](https://claude.ai/code), les applications mobiles et de bureau, les [routines planifiées](/docs/fr/routines), et le terminal, avec [`claude --cloud`](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud) ou une [dispatch `--environment`](/docs/fr/self-hosted-environments-testing#run-the-test-loop), peuvent s'exécuter dans des environnements auto-hébergés. Les sessions [Claude Tag](https://claude.com/docs/claude-tag/overview) peuvent aussi s'y exécuter, mais Claude ne peut pas encore utiliser les [Bundles d'accès](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle) dans ces sessions. Les sessions [Claude Security](/docs/fr/claude-security) et [Code Review](/docs/fr/code-review) ne les routent pas encore. Le support de ces deux surfaces suit séparément.
* **Référentiels** : les sessions extraient les référentiels de GitHub ; consultez [Options d'authentification GitHub](/docs/fr/claude-code-on-the-web#github-authentication-options).
* **Facturation** : les sessions dans un environnement auto-hébergé consomment l'utilisation Claude Code de votre organisation de la même manière que les sessions dans les environnements hébergés par Anthropic.

<h2 id="why-self-host">
  Pourquoi auto-héberger
</h2>

La plupart des équipes sont mieux servies par les environnements hébergés par Anthropic, qui ne nécessitent aucune infrastructure pour fonctionner ou maintenir. L'auto-hébergement est pour les équipes dont les exigences réseau, outillage ou conformité exigent de maintenir l'exécution des sessions sur l'infrastructure qu'elles contrôlent. Si c'est votre cas, planifiez la propriété opérationnelle qu'il entraîne : vous construisez et maintenez l'image du runner, exploitez la flotte, et contrôlez son réseau.

En échange, l'auto-hébergement vous donne l'accès réseau, l'outillage personnalisé, et le contrôle de conformité :

* **Accès réseau** : les sessions s'exécutent à l'intérieur de votre réseau et peuvent atteindre les services internes, les bases de données, et les registres sans les exposer à l'internet public
* **Outillage personnalisé** : pré-installez les compilateurs, les SDK, et les CLI internes dans votre image de runner pour que chaque session démarre prête à construire
* **Conformité** : les extractions de référentiels et les artefacts de construction restent sur l'infrastructure que vous contrôlez. Le contenu de la session va toujours à `api.anthropic.com` pour l'inférence du modèle.

<h2 id="environments-runners-and-sessions">
  Environnements, runners, et sessions
</h2>

Les environnements sont gérés sur la page **Environnements cloud** dans les paramètres d'administration de claude.ai ; les runners sont des processus que vous démarrez et gérez sur votre propre infrastructure.

<h3 id="key-concepts">
  Concepts clés
</h3>

Ces termes apparaissent tout au long des pages auto-hébergées :

| Terme                  | Ce que c'est                                                                                                                                                                                                                                        |
| :--------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Environnement          | Un groupe nommé de vos runners, créé dans les paramètres de claude.ai. Les sessions sont routées vers un environnement, pas vers un runner individuel.                                                                                              |
| Secret d'environnement | L'identifiant partagé unique que les runners utilisent pour s'authentifier et s'enregistrer auprès de l'environnement. Affiché une seule fois à la création de l'environnement, étiqueté **clé d'environnement** dans l'interface d'administration. |
| Runner                 | Le processus de longue durée que vous déployez. Un runner s'enregistre auprès de l'environnement, reçoit un jeton de runner, et interroge les sessions.                                                                                             |
| Session                | Une tâche Claude Code, démarrée à partir de claude.ai, l'application mobile, ou une autre surface Anthropic telle qu'une routine planifiée ou un agent. Chaque session s'exécute en tant que processus Claude Code enfant que le runner génère.     |

Dans les champs API, les revendications de jeton, et les noms de métriques, l'environnement apparaît comme `pool`, et l'ID d'environnement est le `pool_id`. La [référence](/docs/fr/self-hosted-environments-reference) mappe les deux orthographes, y compris les noms d'indicateurs `pool` dépréciés.

Un runner sert un propriétaire à la fois. La première session qu'un runner récupère verrouille le runner à ce propriétaire de session, et le runner exécute ensuite les sessions uniquement pour ce propriétaire, jusqu'à une capacité configurée. Qui est le propriétaire dépend de la façon dont la session a démarré :

* **Sessions qu'un utilisateur démarre** : le propriétaire est le compte de cet utilisateur.
* **Sessions de canal Claude Tag** : Claude les exécute sans compte utilisateur attaché, donc le propriétaire est l'[agent Claude Tag](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity) qui a démarré la session. Chaque session de canal que cet agent démarre a le même propriétaire, peu importe qui a envoyé le message Slack, donc un runner verrouillé à celui-ci sert les sessions que différentes personnes ont démarrées quand vous l'exécutez à une `--capacity` supérieure à un ou avec un `--drain-grace-sec` positif. Un runner verrouillé à un utilisateur ne récupère jamais ceux-ci, et un runner verrouillé à un agent Claude Tag ne récupère jamais les sessions d'un utilisateur.

La taille minimale de la flotte est donc le nombre de propriétaires que vous vous attendez à être actifs à la fois, en comptant les utilisateurs et les agents Claude Tag.

<h3 id="session-lifecycle">
  Cycle de vie de la session
</h3>

Quand un développeur démarre une session et sélectionne votre environnement, le plan de contrôle d'Anthropic place la session dans la file d'attente de l'environnement. À partir de là :

1. Un runner avec une capacité libre réclame la session et maintient un bail sur celle-ci.
2. Le runner clone le référentiel dans son répertoire de travail et génère un processus Claude Code enfant.
3. L'enfant diffuse les événements en continu sur HTTPS tandis que le runner continue d'interroger ; chaque interrogation actualise le bail et sert également de battement cardiaque.
4. Si le runner cesse d'interroger pendant environ 60 secondes, le serveur remet la session en file d'attente pour un autre runner.

Le runner donne à chaque demande d'interrogation 10 secondes. Quand une demande expire, est perdue, ou reçoit une réponse que le runner ne peut pas analyser, le runner continue de servir ses sessions actives et réessaie après une seconde ou deux au lieu d'attendre l'interrogation suivante programmée. Par exemple, un proxy d'interception qui répond à l'interrogation avec sa propre page produit une réponse que le runner ne peut pas analyser. Chaque fois qu'une autre demande échoue de l'une de ces manières, le runner double l'écart avant la prochaine tentative, jusqu'à 20 secondes, et raccourcit l'écart chaque fois que le bail est proche de l'expiration.

<h3 id="runner-lifecycle">
  Cycle de vie du runner
</h3>

La première session qu'un runner récupère verrouille le runner à ce propriétaire de session, et le runner exécute jusqu'à `--capacity` sessions concurrentes pour ce propriétaire. Tant que le runner a des sessions actives et n'a pas reçu de signal d'arrêt ou atteint son heure de retraite, le runner continue de réclamer le travail en file d'attente du propriétaire verrouillé. Ce qui se passe une fois qu'ils se terminent dépend de [`--drain-grace-sec`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) :

* **À la valeur par défaut de `0`** : le runner se termine dès que ses sessions actives se terminent, sans interroger pour plus, donc l'orchestrateur sous lequel vous le déployez, tel que Kubernetes, peut le redémarrer avec un disque frais, prêt à servir n'importe quel propriétaire.
* **À une valeur positive** : le runner continue d'interroger la file d'attente du propriétaire verrouillé pendant ce nombre de secondes avant de se terminer.

Ce cycle de vie isole le code extrait de chaque propriétaire sans exiger que le runner supprime l'état du disque entre les propriétaires.

La façon dont votre infrastructure arrête un runner décide si vous avez besoin de `--retire-at`. Un arrêt qui livre `SIGTERM` n'a besoin d'aucun indicateur : le runner se vide comme [Timing d'arrêt](/docs/fr/self-hosted-environments-deploy#shutdown-timing) le décrit, ou continue de servir les sessions qu'il détient déjà quand vous définissez [`--defer-shutdown-max-min`](/docs/fr/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal). Si votre infrastructure détruit plutôt les hôtes à une heure murale connue sans signal, ou avec une période de grâce trop courte pour se vider, comme une limite de durée de vie du bac à sable ou une réclamation d'instance spot, passez `--retire-at <epoch-seconds>` défini à quelques minutes avant cette heure. À l'heure de retraite :

1. Le runner cesse de prendre du nouveau travail.
2. Le runner libère chaque session active via le même chemin de libération que l'indicateur [`--release-idle-session-min`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) utilise, donc la session reprend sur un runner frais quand l'utilisateur envoie son prochain message. Quand le runner libère chaque session dépend de son état :
   * Le runner libère une session qui est en plein tour dès que ce tour se termine.
   * Quand un tour se termine et laisse des tâches de fond en cours d'exécution, le runner attend jusqu'à 60 secondes pour elles, puis libère la session même si elles s'exécutent toujours. Si les tâches se sont terminées mais le tour de suivi qui lit leurs résultats ne s'est pas encore exécuté, le runner garde la session jusqu'à ce que ce tour se termine, et attend pas plus longtemps que [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/fr/self-hosted-environments-reference#environment-variable-only-settings) pour que ce tour démarre.
3. Le runner se termine 0 une fois que toutes ses sessions sont libérées.

Un tour qui survit à l'arrêt est toujours perdu ; [Timing d'arrêt](/docs/fr/self-hosted-environments-deploy#shutdown-timing) couvre le dimensionnement de la marge. Sans `--retire-at`, un arrêt d'hôte sans signal est indistinguible d'un crash : le plan de contrôle enregistre un worker perdu plutôt qu'une libération propre, et la session se remet en file d'attente pour un autre runner.

<h3 id="network-paths">
  Chemins réseau
</h3>

Le runner et ses sessions établissent plusieurs types de connexion sortante, et aucune connectivité entrante d'Anthropic n'est requise :

* **Plan de contrôle** : le runner interroge `api.anthropic.com` pour le travail et publie les événements de progression de configuration et d'échec, tous HTTPS sortants. L'interrogation sert également de battement cardiaque du runner.
* **Connecteur SCM** : l'orchestrateur optionnel [connecteur SCM](/docs/fr/self-hosted-environments-reference#scm-connector-flags) tunnel est la seule connexion WebSocket.
* **Git** : le runner clone à partir de et pousse vers votre hôte git sur HTTPS ou SSH, authentifié avec les identifiants que votre déploiement fournit ; [Configurer git](/docs/fr/self-hosted-environments-deploy#configure-git) couvre les options, y compris les identifiants frappés par session et la [passerelle git Anthropic](/docs/fr/self-hosted-environments-deploy#use-the-anthropic-git-proxy), qui route git via `api.anthropic.com` à la place.
* **Enfant de session** : le processus Claude Code enfant maintient le flux d'événements de la session à `api.anthropic.com`, et fait ses propres appels sortants pour l'inférence du modèle et pour les commandes git exécutées pendant la session. Consultez [Exigences réseau](/docs/fr/self-hosted-environments-deploy#network-requirements) pour la liste complète des sorties. Le [diagramme ci-dessus](#how-self-hosted-environments-work) montre ces chemins, à part le connecteur SCM optionnel.

L'inférence du modèle utilise l'API Anthropic. Le plan de contrôle livre le point de terminaison API à chaque session, et la session s'authentifie avec un jeton OAuth émis par Anthropic, limité à la session, donc l'inférence ne peut pas être routée via [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/fr/third-party-integrations), ou une [passerelle LLM](/docs/fr/llm-gateway) dans les environnements auto-hébergés.

Les proxies de sortie d'entreprise sont pris en charge. Le runner et l'[orchestrateur de mise à l'échelle automatique](/docs/fr/self-hosted-environments-configuration#on-demand-runners) optionnel honorent le proxy et les variables d'environnement mTLS décrites dans [Configuration réseau](/docs/fr/network-config), telles que `HTTPS_PROXY` et `NO_PROXY` ; définissez-les dans l'environnement de chaque processus. Les variables couvrent les appels du plan de contrôle, le WebSocket du [connecteur SCM](/docs/fr/self-hosted-environments-reference#scm-connector-flags) de l'orchestrateur, et le clone intégré pour les télécommandes HTTPS, et les sessions les héritent du runner. Le streaming de session utilise les événements envoyés par le serveur sur HTTPS, donc un proxy dans le chemin ne doit pas mettre en mémoire tampon les réponses.

Si votre proxy nécessite également un en-tête `Proxy-Authorization`, le runner peut l'ajouter à chaque connexion qu'il ouvre au proxy ; consultez [S'authentifier auprès d'un proxy de sortie](/docs/fr/self-hosted-environments-deploy#authenticate-to-an-egress-proxy).

<h2 id="what-stays-on-your-infrastructure">
  Ce qui reste sur votre infrastructure
</h2>

Les extractions de référentiels, les artefacts de construction, les secrets, et tous les fichiers qu'une session crée ou modifie restent sur les machines que vous approvisionnez. La conversation elle-même, y compris les invites, les réponses, et les résultats des outils, va à `api.anthropic.com` pour l'inférence du modèle, et Anthropic stocke la transcription de la session pour que vous puissiez reprendre la session à partir d'une autre [surface prise en charge](#availability-and-limitations).

Un environnement auto-hébergé déplace l'exécution de la session dans votre réseau. Le plan de contrôle reste hébergé par Anthropic : l'orchestration de session, la mise en file d'attente, et l'interface claude.ai continuent de s'exécuter sur l'infrastructure d'Anthropic.

<h2 id="get-started">
  Commencer
</h2>

Les pages des environnements auto-hébergés sont organisées par ce que vous faites :

* [Démarrage rapide](/docs/fr/self-hosted-environments-quickstart) : installez Claude Code, créez un environnement, démarrez un runner, et routez votre première session
* [Déployer en production](/docs/fr/self-hosted-environments-deploy) : durcissement de la sécurité, sortie réseau, identifiants git, recettes Kubernetes et Compose, problèmes connus, et dépannage
* [Personnaliser les sessions](/docs/fr/self-hosted-environments-configuration) : scripts wrapper pour les identifiants par session, hooks de cycle de vie, runners à la demande, serveurs MCP, et permissions
* [Tester de bout en bout](/docs/fr/self-hosted-environments-testing) : un test de fumée CI qui vérifie une image de runner avant de la promouvoir
* [Référence](/docs/fr/self-hosted-environments-reference) : chaque indicateur CLI, variable d'environnement, métrique, et le point de terminaison de santé
* [Vérifier l'identité de la session](/docs/fr/self-hosted-environments-identity) : validez le jeton de session à partir de vos propres services avant d'accorder l'accès
