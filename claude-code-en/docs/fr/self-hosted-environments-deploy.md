> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Déployer des environnements auto-hébergés en production

> Exécuter des runners auto-hébergés en production : durcissement de la sécurité, contrôle de la sortie réseau, identifiants git, recettes Kubernetes et Compose, et dépannage.

<Note>
  Les environnements auto-hébergés sont en bêta publique sur les plans Team et Enterprise ; [Disponibilité et limitations](/docs/fr/self-hosted-environments#availability-and-limitations) couvre le chemin d'activation. Cette page couvre l'exécution de la flotte en production ; consultez le [guide de démarrage rapide](/docs/fr/self-hosted-environments-quickstart) pour votre premier runner et session.
</Note>

Un [environnement auto-hébergé](/docs/fr/self-hosted-environments) exécute les [sessions cloud](/docs/fr/claude-code-on-the-web) de Claude Code sur des runners que vous déployez dans votre réseau, et en production ces sessions exécutent du code dirigé par le modèle au nom de tous ceux qui peuvent dispatcher une session vers l'environnement. Cette page s'adresse à l'opérateur qui amène un environnement fonctionnel en production. Elle parcourt le déploiement dans l'ordre : ce qu'il faut verrouiller avant de connecter des systèmes réels, la sortie réseau dont la flotte a besoin, comment les sessions s'authentifient auprès de votre hôte git, les recettes de déploiement elles-mêmes, et ce qu'il faut vérifier quand les sessions se comportent mal.

<h2 id="harden-your-deployment">
  Sécurisez votre déploiement
</h2>

Un runner auto-hébergé exécute du code arbitraire, dirigé par le modèle, sur votre infrastructure au nom de tous ceux qui peuvent dispatcher une session vers son environnement. Il s'agit de tout membre de votre organisation Anthropic, et de toute personne qui peut démarrer une session de canal [Claude Tag](https://claude.com/docs/claude-tag/overview) dans une portée qu'un propriétaire a routée vers l'environnement. Travaillez sur chaque élément avant de connecter un environnement à des systèmes de production :

* **Conteneurs éphémères, par session** : exécutez chaque processus runner dans un conteneur ou une VM fraîche qui est détruite lorsque le processus se termine, avec `--capacity 1` et la valeur par défaut `--drain-grace-sec 0` afin que chaque conteneur serve exactement une session. À une capacité plus élevée, ou avec une période de drainage positive, un conteneur sert plusieurs sessions du même [propriétaire verrouillé](/docs/fr/self-hosted-environments#key-concepts) ; voir [Cycle de vie du runner](/docs/fr/self-hosted-environments#runner-lifecycle). Ne réutilisez pas un système de fichiers entre les redémarrages du runner, sauf dans la configuration délibérée [checkout pré-chauffé](#reuse-a-pre-warmed-checkout), et jamais entre les propriétaires.
* **Pas de larges identifiants dans l'image** : n'incluez pas de clés SSH de longue durée, d'identifiants de fournisseur cloud, ou de jetons d'accès personnel qui accordent plus que ce qu'une session a besoin. Générez les identifiants utilisés pendant une session, tels que les jetons push ou API, par session à partir de votre [script wrapper](/docs/fr/self-hosted-environments-configuration#wrapper-scripts). Pour le clone initial, qui se produit avant l'exécution du wrapper, utilisez un [hook de cycle de vie `checkout`](/docs/fr/self-hosted-environments-configuration#checkout) ou [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) ; voir [Configurer git](#configure-git).
* **Gardez le secret de l'environnement loin des hôtes exécutant les sessions** : le secret de l'environnement peut enregistrer des runners et récupérer toute session mise en file d'attente sur l'environnement. Sur une flotte fixe, il réside sur chaque hôte runner, où le code de toute session peut lire le fichier secret. Préférez les [runners à la demande](/docs/fr/self-hosted-environments-configuration#on-demand-runners), où le secret reste sur l'hôte orchestrateur, qui n'exécute jamais de code utilisateur, et chaque runner reçoit un bon de travail à usage unique qui enregistre exactement un runner. Sur une flotte fixe, traitez le fichier secret-environnement comme lisible par toute session et faites tourner le secret après tout compromis de session suspecté.
* **Sortie réseau par défaut-refuser** : limitez le trafic sortant du conteneur runner et session à votre propre limite réseau sur chaque environnement ; [Sortie par défaut-refuser](#default-deny-egress) couvre ce qu'il faut autoriser et pourquoi.
* **IAM hôte avec privilèges minimaux** : l'identité de calcul attachée à l'hôte runner, telle qu'un profil d'instance ou un compte de service de nœud, ne devrait accorder que ce dont le runner lui-même a besoin. Les sessions devraient obtenir leurs propres identifiants via votre script wrapper plutôt que d'hériter de ceux de l'hôte.
* **Bloquez le point de terminaison des métadonnées cloud des sessions** : garder les sessions hors de l'identité hôte nécessite de bloquer leur accès au point de terminaison des métadonnées, et les politiques de sortie au niveau du sous-réseau n'interceptent pas le trafic de métadonnées link-local, donc bloquez-le dans le conteneur lui-même :

  * IMDSv2 avec une limite de saut d'un
  * GKE Workload Identity avec dissimulation des métadonnées
  * Un refus explicite pour `169.254.169.254` dans l'espace de noms réseau du conteneur de session

  Le bloc s'applique également à votre script wrapper et aux hooks de cycle de vie, puisqu'ils partagent le conteneur. Authentifiez tout échange de jeton avec le [JWT de session](/docs/fr/self-hosted-environments-identity) contre votre propre service de jeton sur une sortie autorisée, ou utilisez une identité web basée sur fichier telle que les rôles IAM pour les comptes de service (IRSA) sur Amazon EKS.
* **Isolation du système de fichiers par runner** : chaque processus runner obtient son propre répertoire de travail qu'aucun autre processus sur l'hôte ne peut lire ou écrire. Rendez `--hooks-dir`, le script wrapper, et le `~/.claude/` de l'hôte en lecture seule pour la session, soit intégrés dans l'image, soit montés en lecture seule.
* **Dispatch n'a pas de contrôle d'accès par environnement** : tout membre de votre organisation Anthropic peut dispatcher une session vers n'importe lequel de ses environnements. Si un propriétaire [route les canaux Claude Tag vers l'environnement](/docs/fr/cloud-environments#set-the-environment-a-claude-tag-channel-uses), toute personne que le [paramètre d'accès Claude Tag](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude) admet peut démarrer des sessions de canal qui s'y exécutent. Par défaut, il s'agit de toute personne dans l'espace de travail Slack connecté, avec ou sans compte Claude. Traitez chaque hôte runner comme accessible pour l'exécution de code par tous ceux qui peuvent dispatcher vers lui, et placez sur un hôte runner uniquement les données et identifiants que toutes ces personnes sont autorisées à lire. [`--lock-to-account`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) limite les sessions de quel compte un hôte donné exécute, mais cela ne réduit pas qui peut dispatcher dans l'environnement. Pour faire des environnements auto-hébergés la seule option de sélection, un [propriétaire](/docs/fr/cloud-environments#organization-shared-environments) peut masquer les environnements hébergés par Anthropic pour toute l'organisation à partir de la page [**Environnements cloud**](https://claude.ai/admin-settings/cloud-environments).
* **Appliquez la garde des paramètres de dépôt** : choisissez le mode de garde avec [`--confine-repo-settings`](/docs/fr/self-hosted-environments-reference#runner-cli-flags). La valeur par défaut `warn` enregistre une violation et lance quand même la session, `enforce` refuse la session, et `off` désactive l'analyse. Le runner analyse les paramètres validés de chaque dépôt pour :

  * Une autorisation qui se résout en dehors de l'espace de travail propre de cette session : une entrée `additionalDirectories`, une règle `Edit`, `Write`, ou `NotebookEdit` dans `permissions.allow`, ou une entrée `sandbox.filesystem.allowWrite` ou `allowRead`
  * Un bloc `env` non vide
  * Un remplacement de posture d'opérateur tel que `sandbox.enabled: false`

  La garde s'exécute indépendamment de [`--trust-workspace`](/docs/fr/self-hosted-environments-reference#runner-cli-flags), et ne couvre pas les hooks de dépôt, `.mcp.json`, ou les règles Bash ; voir [Permissions et approbation des outils](/docs/fr/self-hosted-environments-configuration#permissions-and-tool-approval) pour savoir où ces autorisations doivent se trouver.

<Note>
  La liste d'autorisation IP de votre organisation ne couvre pas le trafic du runner auto-hébergé par défaut. Ne vous fiez pas à elle comme contrôle réseau pour le trafic du runner ou de la session ; appliquez plutôt une sortie par défaut-refuser à votre propre limite réseau, et contactez votre équipe de compte Anthropic si vous souhaitez l'application de la liste d'autorisation IP pour votre organisation.
</Note>

<h2 id="network-requirements">
  Exigences réseau
</h2>

Le runner et les enfants de session qu'il génère établissent des connexions sortantes vers les hôtes ci-dessous. Limitez la sortie du conteneur de session à ces hôtes et aux services internes spécifiques que les sessions doivent atteindre ; [Sortie par défaut-refuser](#default-deny-egress) couvre comment et pourquoi.

Ces hôtes sont toujours requis :

| Hôte                                                                 | Port                                               | Utilisé pour                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------------------------------------------------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                                                  | 443, HTTPS ; WSS pour le connecteur SCM uniquement | Plan de contrôle du runner et streaming de session, inférence de modèle, drapeaux de fonctionnalités, analytique de produit, récupérations de clés [JWKS](/docs/fr/self-hosted-environments-identity), signature de commit, le proxy git quand `--use-anthropic-git-proxy` est défini, et le tunnel [connecteur SCM](/docs/fr/self-hosted-environments-reference#scm-connector-flags) de l'orchestrateur quand `--scm-connector-host` est défini |
| Votre hôte git, tel que `github.com` ou votre hôte GitHub Enterprise | 443 ou 22                                          | Clonage et push de référentiels. Non nécessaire si le runner utilise `--use-anthropic-git-proxy`, qui route le trafic git via `api.anthropic.com`.                                                                                                                                                                                                                                                                                     |

Que ces hôtes soient nécessaires dépend de votre configuration :

| Hôte                                 | Port | Quand requis                                                                                                                                                                                                                                                                                                                                           |
| :----------------------------------- | :--- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `downloads.claude.ai`                | 443  | Au moment de l'installation, quand vous installez ou mettez à jour Claude Code sur l'hôte avec l'installateur natif ; le script `install.sh` lui-même est servi depuis `claude.ai`. Au moment de l'exécution de la session, uniquement quand les sessions installent des plugins depuis la place de marché officielle Anthropic.                       |
| `storage.googleapis.com`             | 443  | Au moment de l'exécution de la session, pour les comptages d'installation de plugins et les métadonnées affichées dans `/plugin`.                                                                                                                                                                                                                      |
| `code.claude.com` et `claude.com`    | 443  | Recherches de documentation par l'agent claude-code-guide intégré et demandes WebFetch pré-approuvées pendant les sessions. Bloquer ces hôtes affecte uniquement les recherches de documentation.                                                                                                                                                      |
| `*.frame.claudeusercontent.com`      | 443  | Uniquement quand l'[outil Artifact](/docs/fr/artifacts#availability) est disponible pour les sessions dans votre organisation ; les valeurs par défaut varient selon le plan, selon le tableau de disponibilité là-bas. Définissez `CLAUDE_CODE_DISABLE_ARTIFACT=1` sur le runner pour garder l'outil désactivé indépendamment du paramètre d'organisation. |
| `registry.npmjs.org`                 | 443  | Quand une session installe un plugin, à la fois pour récupérer les packages de plugin source npm et pour installer les dépendances Node.js d'un plugin, ou quand un serveur MCP lancé par `npx` s'exécute                                                                                                                                              |
| `http-intake.logs.us5.datadoghq.com` | 443  | Métriques opérationnelles Anthropic. Uniquement quand `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` est défini ; désactivé par défaut dans les environnements auto-hébergés.                                                                                                                                                                                     |
| `browser-intake-us5-datadoghq.com`   | 443  | Téléchargements de rapports d'erreurs Anthropic, envoyés uniquement quand [le rapport d'erreurs](/docs/fr/data-usage#telemetry-services) est activé pour le compte de la session. Supprimé par `DISABLE_ERROR_REPORTING=1` ou `DISABLE_TELEMETRY=1`.                                                                                                        |

Le runner n'atteint pas `statsig.anthropic.com`, `*.sentry.io`, `claude.ai`, ou `platform.claude.com`. Ces hôtes apparaissent dans certaines listes de contrôle réseau d'entreprise plus anciennes, mais vous n'avez pas besoin de les autoriser pour le trafic du runner ou de la session : les récupérations de drapeaux de fonctionnalités vont à `api.anthropic.com`, et le runner s'authentifie avec le secret de l'environnement plutôt qu'avec OAuth interactif. Deux flux côté hôte atteignent `claude.ai`, donc exécutez-les à partir d'un hôte dont la sortie le permet plutôt que d'élargir la sortie du conteneur de session : l'installateur d'une ligne récupère `install.sh` depuis `claude.ai` au moment de l'installation, et `claude auth login` interactif, que le [guide de configuration](/docs/fr/self-hosted-environments-quickstart#set-up-an-environment-and-runner), le mode signé du `doctor`, et [la dispatch CI](/docs/fr/self-hosted-environments-testing#authenticate-from-ci) utilisent, se connecte via `claude.ai`, `claude.com`, et `platform.claude.com`. `mcp-proxy.anthropic.com` n'est pas requis non plus : les sessions auto-hébergées ne l'utilisent pas, et la livraison de vos connecteurs claude.ai d'organisation aux sessions, quand activée pour votre organisation, route via `api.anthropic.com`. Consultez [Serveurs MCP](/docs/fr/self-hosted-environments-configuration#mcp-servers).

<h3 id="default-deny-egress">
  Sortie par défaut-refuser
</h3>

Déployez les conteneurs runner et session dans un segment réseau ou un espace de noms dont le trafic sortant est limité aux hôtes du [tableau des exigences réseau](#network-requirements), votre hôte git, et les services internes spécifiques que les sessions doivent atteindre. Le produit ne peut pas vérifier ou appliquer cela, donc appliquez-le à votre propre limite réseau sur chaque environnement. Le code de session est dirigé par le modèle et peut tenter des connexions vers des hôtes arbitraires ; la sortie par défaut-refuser au niveau réseau limite où ces tentatives peuvent atterrir. Cela s'applique indépendamment du mode de permission : l'ensemble d'outils pré-approuvé par défaut inclut déjà `Bash`, donc la sortie shell s'exécute sans invite même sans [mode auto](/docs/fr/self-hosted-environments-configuration#permissions-and-tool-approval).

Pour plus de détails sur la télémétrie que chaque session émet et comment la désactiver, consultez [Télémétrie](/docs/fr/self-hosted-environments-reference#telemetry).

<h3 id="authenticate-to-an-egress-proxy">
  S'authentifier auprès d'un proxy de sortie
</h3>

Certains proxies de sortie d'entreprise nécessitent un en-tête `Proxy-Authorization` sur chaque connexion. Le jeton dans cet en-tête tourne souvent trop vite pour être écrit dans l'URL du proxy que vous définissez dans `HTTPS_PROXY`. Définissez `HTTPS_PROXY` ou `HTTP_PROXY` sur l'URL de votre proxy comme d'habitude, puis définissez `--proxy-authorization-command` ou `--proxy-authorization-file` pour dire au runner où lire la valeur de l'en-tête. Les deux drapeaux nécessitent Claude Code v2.1.238 ou plus récent.

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  Choisir d'où provient la valeur `Proxy-Authorization`
</h4>

Choisissez le drapeau qui correspond à la façon dont vous produisez le jeton `Proxy-Authorization` :

* **[`--proxy-authorization-command <command>`](/docs/fr/self-hosted-environments-reference#runner-cli-flags)** : choisissez ceci pour un jeton que vous générez à la demande. Le runner exécute la commande shell et utilise sa stdout rognée comme valeur d'en-tête, par exemple `Bearer <token>`.
* **[`--proxy-authorization-file <path>`](/docs/fr/self-hosted-environments-reference#runner-cli-flags)** : choisissez ceci pour un jeton qu'un autre processus fait tourner en place. Le runner lit le fichier et utilise son contenu rogné comme valeur d'en-tête.

<h4 id="configurations-the-runner-refuses-to-start-with">
  Configurations que le runner refuse de démarrer avec
</h4>

Chaque drapeau a également une forme de variable d'environnement, listée à côté dans la [référence des drapeaux CLI du runner](/docs/fr/self-hosted-environments-reference#runner-cli-flags). Avant que le runner ne contacte votre proxy ou le plan de contrôle, il vérifie les drapeaux et leurs variables, et refuse de démarrer dans trois cas :

* **Les deux drapeaux définis** : un drapeau plus la variable d'environnement de l'autre drapeau compte comme la définition des deux.
* **Pas d'URL de proxy** : ni `HTTPS_PROXY` ni `HTTP_PROXY` ne contient une URL `http://` ou `https://`. Le runner lit les deux variables en majuscules ou minuscules, et ne consulte pas `ALL_PROXY`.
* **L'un ou l'autre drapeau passé à la sous-commande orchestrateur** : `self-hosted-runner orchestrator` n'accepte pas les drapeaux ou leurs variables d'environnement. Passez le drapeau à chaque runner que l'orchestrateur démarre à la place.

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  Ce que le runner change quand un drapeau d'autorisation de proxy est défini
</h4>

Avec l'un ou l'autre drapeau défini, le runner démarre son propre écouteur et envoie le trafic proxy de lui-même, ses hooks de cycle de vie, et ses sessions via cet écouteur. L'écouteur ajoute l'en-tête `Proxy-Authorization` en route vers votre proxy.

* **Écouteur** : l'écouteur est un proxy avant sur `127.0.0.1`. Le runner démarre l'écouteur avant de s'enregistrer auprès du plan de contrôle, et quitte au démarrage si l'écouteur ne peut pas démarrer.
* **Variables de proxy** : le runner réécrit lequel de `HTTPS_PROXY` et `HTTP_PROXY` vous avez défini pour qu'il pointe vers l'écouteur. Cette valeur réécrite atteint le runner lui-même, ses hooks de cycle de vie, et chaque session qu'il exécute.
* **Rotation de jeton** : un jeton pivoté prend effet sans redémarrage. Pour chaque connexion que l'écouteur ouvre vers votre proxy, le runner exécute votre commande ou relit votre fichier et ajoute le résultat comme en-tête.
* **Environnement de session** : une session atteint votre proxy uniquement via l'écouteur. Dans l'environnement de chaque session, le runner supprime `ALL_PROXY`, supprime toute orthographe de `HTTPS_PROXY` ou `HTTP_PROXY` que vous n'avez pas définie, et épingle `NO_PROXY` à la valeur du runner.
* **Journaux** : le runner ne journalise jamais la valeur de l'en-tête.

<h2 id="configure-git">
  Configurer git
</h2>

Le runner gère les checkouts de référentiel mais ne configure pas l'identité git ou les identifiants par défaut. Vous contrôlez l'image et l'environnement de processus du runner, donc vous contrôlez la configuration git. Choisissez l'une de deux approches :

* **Laisser le runner configurer git** : démarrez le runner avec `--configure-git` pour qu'il écrive la même identité et configuration de signature de commit que les sessions hébergées par Anthropic utilisent
* **Livrer la configuration git dans votre image** : définissez l'identité et les identifiants push vous-même, par exemple pour committer sous votre propre identité de bot

Planchers de version Git sur l'hôte runner : [`--configure-git`](#let-the-runner-configure-git) la signature de commit SSH nécessite Git 2.34 ou plus récent, [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) nécessite 2.32 ou plus récent, et reprendre les sessions à partir de branches poussées par [`--push-outcome-on-release`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) nécessite 2.29 ou plus récent. Git 2.24 est suffisant si vous omettez les trois et gérez l'identité git vous-même.

<h3 id="let-the-runner-configure-git">
  Laisser le runner configurer git
</h3>

Démarrez le runner avec `--configure-git`, ou définissez `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`, pour qu'il écrive la configuration git globale au démarrage :

* `user.name = Claude` et `user.email = noreply@anthropic.com`, correspondant aux sessions hébergées par Anthropic
* Signature de commit et de tag au format SSH, routée via un shim géré par le runner qui signe chaque commit via le service de signature d'Anthropic en utilisant les identifiants de la session. Les signatures sont vérifiables sur GitHub par rapport à la clé de signature SSH publiée d'Anthropic.
* `push.negotiate = true`, donc git demande à votre hôte git quels commits il possède déjà avant de préparer un push. Nécessite Claude Code v2.1.257 ou plus récent.
* `core.hooksPath` pointant vers un répertoire de hooks géré par le runner. Ses hooks `commit-msg` et `prepare-commit-msg` ajoutent une remorque `Co-authored-by:` pour le créateur de la session à chaque commit, construite à partir de l'email dans [`CCR_SESSION_ACCOUNT_EMAIL`](/docs/fr/self-hosted-environments-configuration#wrapper-scripts) et omise quand cette variable n'est pas définie. Si votre image définit déjà `core.hooksPath`, le runner laisse votre paramètre en place, ignore l'installation de ces hooks, et affiche un avertissement `[runner:git]`.

La signature de commit nécessite git 2.34 ou plus récent ; le runner vérifie au démarrage et quitte avec une erreur si votre git est plus ancien. Ce drapeau ne configure pas les identifiants push, que vous fournissez toujours dans l'image.

<h3 id="ship-git-config-in-your-image">
  Livrer la configuration git dans votre image
</h3>

L'identité Git est requise pour tout commit. Définissez-la au niveau du système dans votre Dockerfile pour que la configuration s'applique indépendamment de l'utilisateur sous lequel le processus runner s'exécute :

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

Sans une identité, `git commit` échoue avec `Please tell me who you are` et les sessions ne peuvent pas progresser. Vous pouvez utiliser votre propre identité de bot à la place ; le runner ne remplace pas ces valeurs.

Ne cuisez pas les identifiants push longue durée ou largement scoped dans une image runner partagée : un identifiant dans l'image est disponible pour chaque session que l'image exécute, peu importe qui l'a démarrée. À la place, créez un jeton court-durée, least-scoped par session à partir de votre [script wrapper](/docs/fr/self-hosted-environments-configuration#wrapper-scripts), en utilisant l'identité du créateur de session décodée du JWT de session. Associez-le à un conteneur éphémère par session, qui nécessite `--capacity 1`, pour qu'aucun identifiant ne survive à la session qui l'a créé ; consultez la [section de durcissement](#harden-your-deployment).

Si vous devez configurer les identifiants push au niveau de l'image, par exemple pour une clé de déploiement en lecture seule, limitez-les aussi étroitement que votre hôte git le permet :

* Une clé de déploiement SSH limitée à un référentiel avec une réécriture `url.<base>.insteadOf`
* Un `credential.helper` qui retourne un jeton minimalement scoped
* `GIT_SSH_COMMAND` pointant vers une clé étroitement scoped

Quel que soit le mécanisme que vous configurez, il doit fonctionner sans invite, car le clone intégré du runner et la récupération désactivent les invites que git, SSH, et Git Credential Manager afficheraient autrement :

* Le runner définit `GIT_TERMINAL_PROMPT=0`, donc git ne demande pas de nom d'utilisateur ou de mot de passe.
* Le runner exécute SSH avec `BatchMode=yes`, ajouté à votre `GIT_SSH_COMMAND` si vous en définissez un, donc SSH ne demande pas de phrase de passe ou de confirmation d'hôte.
* Le runner définit `GCM_INTERACTIVE=never`, donc Git Credential Manager n'ouvre pas de dialogue de connexion.
* Le runner efface `core.askPass`, donc si vous utilisez un helper askpass, définissez-le via la variable d'environnement `GIT_ASKPASS` à la place.

Si votre hôte git rejette l'identifiant, ou que vous n'en avez pas configuré un, le runner réessaie quelques fois puis échoue la préparation du référentiel quand le référentiel est celui vers lequel la session pousse les résultats. Pour un référentiel que la session lit uniquement, [Troubleshooting](#troubleshooting) couvre quand le runner le saute à la place. Le runner ne transmet pas ces paramètres dans l'environnement de la session.

Si les répertoires de checkout sont possédés par un uid différent du processus runner, git refuse d'opérer sur eux ; ajoutez `safe.directory` :

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Utiliser le proxy git Anthropic
</h3>

Démarrez le runner avec `--use-anthropic-git-proxy`, ou définissez `CLAUDE_RUNNER_USE_GIT_PROXY=1`, pour qu'il clone via le proxy git d'Anthropic, authentifié avec le jeton court-durée de la session. Pour les sessions utilisateur ordinaires, le proxy utilise le jeton OAuth GitHub ou GitHub Enterprise stocké pour le créateur de session ; pour les sessions de bot et d'agent, il utilise le jeton d'installation GitHub App de votre organisation. De toute façon, l'image runner n'a besoin d'aucun identifiant git : pas de clés SSH, pas de credential helper, pas de `.netrc`. C'est le même chemin d'authentification que les environnements hébergés par Anthropic utilisent.

Le proxy nécessite `--capacity 1` car l'URL du proxy est par session, et git 2.32 ou plus récent car les anciennes versions de git ignorent le mécanisme de configuration que le proxy utilise pour isoler les sessions les unes des autres. Le runner refuse de démarrer si l'une ou l'autre exigence n'est pas satisfaite. Parce que le proxy récupère du côté d'Anthropic, votre hôte git doit être accessible depuis l'infrastructure Anthropic, la même exigence que les sessions hébergées par Anthropic ont ; pour un hôte git qui n'est routable que dans votre réseau, utilisez un [hook de cycle de vie `checkout`](/docs/fr/self-hosted-environments-configuration#checkout) à la place. Chaque processus runner gère une session à la fois, donc exécutez plus de répliques pour le parallélisme. Quand le proxy est activé, `--git-host-rewrite` et `--git-ssh-rewrite` n'ont aucun effet : l'URL du proxy pointe vers `api.anthropic.com`, pas votre hôte git.

Le runner signale également l'adhésion à Anthropic quand il s'enregistre, affichant `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)` au démarrage. Signaler l'adhésion nécessite Claude Code v2.1.267 ou plus récent, et les versions antérieures acceptent le drapeau sans le signaler ou afficher cette ligne. Chaque session sur un runner ayant adhéré utilise ensuite soit git géré par Anthropic, soit l'URL du proxy par session. Quand une session utilise l'URL du proxy par session, le runner enregistre une ligne `[runner:warn]` indiquant cela.

<h3 id="rewrite-git-urls-for-private-networks">
  Réécrire les URL git pour les réseaux privés
</h3>

Les URL de référentiel arrivent du plan de contrôle en HTTPS, avec le nom d'hôte de votre hôte git ; pour GitHub Enterprise, c'est le nom d'hôte que vous avez configuré pour l'[intégration GitHub Enterprise](/docs/fr/github-enterprise-server) dans les paramètres d'administration de Claude Code sur claude.ai. Deux drapeaux répétables réécrivent ces URL avant le clone :

* `--git-host-rewrite <from>=<to>` : pour le DNS à horizon divisé, où Anthropic atteint votre hôte git via un nom d'hôte externe mais les runners doivent utiliser un interne
* `--git-ssh-rewrite <host>` : pour les hôtes git qui n'acceptent que SSH, réécrivant `https://<host>/owner/repo` en `git@<host>:owner/repo`

La réécriture d'hôte s'exécute en premier, donc listez le nom d'hôte interne dans `--git-ssh-rewrite` si vous avez besoin des deux. Pour un contrôle complet du checkout, utilisez un [hook de cycle de vie `checkout`](/docs/fr/self-hosted-environments-configuration#checkout).

<h2 id="build-the-runner-image">
  Construire l'image du runner
</h2>

Anthropic ne publie pas d'image runner pré-construite. Construisez la vôtre autour du binaire `claude`, en superposant la chaîne d'outils que vos référentiels ont besoin : runtimes de langage, compilateurs, gestionnaires de paquets, et sidecars [MCP](/docs/fr/mcp).

Les recettes ci-dessous utilisent `--capacity 4`, donc un conteneur sert jusqu'à quatre sessions concurrentes du même propriétaire verrouillé. Cela ne fournit pas l'isolation du conteneur par session dans la [section de durcissement](#harden-your-deployment) : avant de connecter un environnement aux systèmes de production, soit exécutez les recettes à `--capacity 1` avec un conteneur par session, soit utilisez les [runners à la demande](/docs/fr/self-hosted-environments-configuration#on-demand-runners), qui gardent également le secret de l'environnement hors des hôtes exécutant les sessions.

Ce Dockerfile est un point de départ minimal :

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

Échangez `linux-x64` pour `linux-arm64` si vos nœuds sont ARM, ou pour `linux-x64-musl` ou `linux-arm64-musl` sur une image basée sur musl comme Alpine ; consultez [Configuration Alpine Linux](/docs/fr/setup#alpine-linux-and-musl-based-distributions) pour les paquets supplémentaires dont les images musl ont besoin. L'URL est l'emplacement de version standard de Claude Code, donc vous pouvez vérifier le binaire téléchargé par rapport au manifeste signé de la version comme décrit dans [Intégrité binaire et signature de code](/docs/fr/setup#binary-integrity-and-code-signing). Construisez l'image avec Claude Code version 2.1.224 ou plus récent, puis poussez-la vers votre registre et référencez-la dans les recettes ci-dessous :

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  Dimensionner le CPU et la mémoire pour les sessions
</h2>

Dimensionnez le conteneur ou l'hôte d'un runner pour les sessions qu'il exécute plutôt que pour le processus runner lui-même. Le runner lui-même interroge le système pour trouver du travail, prépare le checkout de chaque session, exécute vos [hooks de cycle de vie](/docs/fr/self-hosted-environments-configuration#lifecycle-hooks), et démarre et supervise les processus de session. La charge provient des sessions : chacune est un processus Claude Code plus tout ce qu'il démarre, comme les builds, les suites de tests, les installations de paquets, et les [serveurs MCP](/docs/fr/mcp).

Pour une session, commencez par les valeurs suivantes, exprimées comme des demandes et des limites Kubernetes ou l'équivalent de votre plateforme, et traitez-les comme un point de départ plutôt que comme une exigence :

* **Mémoire** : une demande et une limite de 4 Gio chacune, ce qui satisfait le minimum de 4 Go dans les [exigences système](/docs/fr/setup#system-requirements) de Claude Code. Gardez les deux égales afin que le planificateur tienne compte de la mémoire complète du conteneur. Lorsque le conteneur atteint sa limite de mémoire, le noyau tue les processus à l'intérieur, ce qui peut terminer une session en cours de tâche.
* **CPU** : une demande de 2 CPUs et une limite de 4 CPUs, afin qu'une session puisse dépasser la demande lors des builds. Le noyau limite un conteneur à sa limite de CPU plutôt que de tuer les processus à l'intérieur, donc les sessions à la limite s'exécutent plus lentement mais continuent de s'exécuter.

Dans une spécification de conteneur Kubernetes, définissez ces valeurs de départ avec le bloc `resources` suivant :

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

Les builds et les tests constituent généralement la plus grande et la plus variable partie de la charge d'une session, donc exécutez un build représentatif de votre référentiel, mesurez son pic de CPU et de mémoire, et augmentez toute valeur de départ qui ne laisse pas de place au processus Claude Code en plus de ce pic.

Le runner utilise `--capacity` pour limiter le nombre de sessions qu'il exécute à la fois. Il ne divise pas le CPU ou la mémoire entre elles, donc les sessions sur un runner partagent le CPU et la mémoire du conteneur. Pour limiter la part d'une session, appliquez les limites de votre [script wrapper](/docs/fr/self-hosted-environments-configuration#wrapper-scripts). Ce qu'il faut donner à un conteneur dépend donc du nombre de sessions qu'il dessert à la fois :

* **Une session par runner** : donnez à chaque conteneur les valeurs d'une session. Utilisez ce dimensionnement à `--capacity 1`, que la [section de durcissement](#harden-your-deployment) recommande, et pour les [runners à la demande](/docs/fr/self-hosted-environments-configuration#on-demand-runners), où vous définissez les valeurs sur la charge de travail que votre [hook `spawn-runner`](/docs/fr/self-hosted-environments-configuration#the-spawn-runner-hook) soumet, comme un modèle de pod de Job Kubernetes.
* **Plusieurs sessions par runner** : à un `--capacity` supérieur à un, multipliez les valeurs d'une session par la capacité, car jusqu'à ce nombre de sessions peuvent s'exécuter dans le conteneur en même temps. Les recettes [Kubernetes](#kubernetes) et [Docker Compose](#docker-compose) exécutent `--capacity 4` sans limites de CPU ou de mémoire, donc ajoutez des limites dimensionnées pour la capacité que vous exécutez.

<h2 id="kubernetes">
  Kubernetes
</h2>

Le runner sert `GET /healthz` sur le port 8080 par défaut, configurable avec `--health-port`, donc les sondes Kubernetes fonctionnent sans configuration supplémentaire. Le point de terminaison retourne `200` chaque fois que le processus est vivant, donc les sondes ci-dessous détectent un processus mort, pas un bloqué ; pour attraper un runner qui a arrêté d'interroger, alertez sur la série `last_poll_age_seconds` de [`/metrics`](/docs/fr/self-hosted-environments-reference#prometheus-metrics). Le Deployment ci-dessous monte le secret de l'environnement à partir d'un Secret Kubernetes, pointe les sondes de vivacité et de disponibilité vers `/healthz`, et définit une période de grâce de terminaison de 90 secondes. Consultez [Timing d'arrêt](#shutdown-timing) pour savoir pourquoi la période de grâce est importante.

Le manifeste ne définit pas de `resources` CPU ou mémoire sur le conteneur runner. Ajoutez un bloc dimensionné pour la capacité que vous exécutez, comme [Dimensionner le CPU et la mémoire pour les sessions](#size-cpu-and-memory-for-sessions) le décrit.

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

Le Deployment ci-dessus vit dans un espace de noms `claude-runners`. Créez d'abord l'espace de noms :

```bash theme={null}
kubectl create namespace claude-runners
```

Créez le Secret de sauvegarde à partir d'un fichier local contenant la valeur que vous avez copiée à l'étape [**Copier la clé d'environnement**](/docs/fr/self-hosted-environments-quickstart#set-up-an-environment-and-runner) de l'interface utilisateur d'administration, pour que le secret n'apparaisse jamais dans votre historique de shell. Exécutez `(umask 077 && cat > ./environment-secret)`, collez le secret, appuyez sur Entrée, puis Ctrl-D. Ensuite, créez le Secret et supprimez le fichier :

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

Le service Compose ci-dessous redémarre le runner chaque fois qu'il se termine, ce qui couvre à la fois les crashes et la sortie normale après le drainage. Une politique de redémarrage Docker redémarre le même conteneur avec sa couche inscriptible intacte, donc le runner revient sur un système de fichiers réutilisé plutôt que le frais que la [posture de durcissement](#harden-your-deployment) recommande ; utilisez cette recette pour l'évaluation, et pour la production soit recréez le conteneur par exécution, soit utilisez un orchestrateur qui le fait.

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  Délai d'arrêt
</h2>

À la réception de `SIGTERM`, le runner cesse de prendre de nouveaux travaux et, sauf si vous définissez [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), attend jusqu'à `--drain-wait-sec`, zéro par défaut, que les tours en cours se terminent, termine l'arborescence des processus de chaque session, et exécute le hook de cycle de vie [`post-session`](/docs/fr/self-hosted-environments-configuration#post-session). Cette arborescence de processus inclut les commandes que Claude exécutait encore dans la session.

Le chemin de drainage complet nécessite jusqu'à `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec`, plus 15 secondes de surcharge fixe pour le nettoyage des processus, plus 30 secondes supplémentaires lorsque [`--push-outcome-on-release`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) est défini. Cela représente 80 secondes par défaut, et le runner enregistre le total au démarrage. Les sessions se drainent en parallèle sous ce budget unique, donc le total ne croît pas avec `--capacity`.

À la valeur par défaut `--drain-wait-sec 0`, un redémarrage continu interrompt les tours en cours ; chaque session reprend sur un autre runner, perdant le travail non poussé comme décrit sous [Problèmes connus](#additional-limitations). Définissez `--drain-wait-sec` et augmentez la période de grâce pour correspondre, pour laisser les tours se terminer en premier.

Tout au long de ce chemin, le runner continue de faire un heartbeat vers le plan de contrôle à capacité zéro, de sorte que le bail de session n'expire pas et ne soit pas remis en file d'attente vers un autre runner tandis que le hook `post-session` écrit toujours le travail non validé. Le heartbeat s'arrête juste avant que le runner ne se désenregistre.

Donnez au runner au moins le total qu'il enregistre au démarrage avant que l'hôte ne l'arrête. L'endroit où vous définissez cela dépend de la façon dont vos hôtes s'arrêtent :

* **Avec une période de grâce `SIGTERM`** : définissez `terminationGracePeriodSeconds` sur Kubernetes, `stop_grace_period` sur Docker Compose, ou l'équivalent de votre orchestrateur à au moins ce total. La valeur par défaut de Kubernetes de 30 secondes est plus courte que le chemin de drainage du runner, donc Kubernetes arrête le pod avant que le runner ne termine le drainage.
* **Avec [`--retire-at`](/docs/fr/self-hosted-environments-reference#runner-cli-flags)** : dimensionnez la marge entre l'heure de retraite et l'heure d'arrêt de l'hôte pour couvrir les tours typiques, plus la rétention des tâches de fond que [Cycle de vie du runner](/docs/fr/self-hosted-environments#runner-lifecycle) décrit, plus ce même total. Calculez l'heure de retraite à chaque lancement, par exemple `date +%s` plus la durée de vie prévue du runner.
* **Avec [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)** : ajoutez deux parties supplémentaires au total du chemin de drainage. La première est les minutes que vous configurez. La seconde est la grâce post-libération que [Différer le drainage au-delà du premier signal](#defer-the-drain-past-the-first-signal) décrit, 75 secondes par défaut. Avec le flag défini, le runner imprime également la figure combinée au démarrage, après le total du chemin de drainage.

<h3 id="defer-the-drain-past-the-first-signal">
  Différer le drainage au-delà du premier signal
</h3>

Définissez [`--defer-shutdown-max-min <n>`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) si vous souhaitez qu'un runner que vous redémarrez continue de servir les sessions qu'il détient pendant jusqu'à `n` minutes, au lieu de les drainer au premier signal. À la première `SIGTERM` ou `SIGINT`, le runner cesse de prendre de nouveaux travaux et continue de servir les sessions qu'il détient. Il continue de sonder pour que le plan de contrôle ne remette pas ces sessions en file d'attente. Nécessite Claude Code v2.1.238 ou ultérieur.

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  Ce qui arrive aux sessions que le runner détient après le premier signal
</h4>

Dans les deux premiers stades qui suivent le signal, le runner libère les sessions, et une session libérée reprend sur un runner frais lorsque son utilisateur envoie son prochain message. En comptant à partir du premier signal, le runner passe par trois stades :

* **Pendant les premiers `n` minutes** : le runner sert ses sessions normalement et continue d'appliquer `--startup-timeout-min` et `--kill-session-after-min`. Si vous définissez également [`--release-idle-session-min`](/docs/fr/self-hosted-environments-reference#runner-cli-flags), le runner libère toute session dont l'utilisateur a été inactif aussi longtemps ; sans cela, les sessions inactives restent sur le runner.
* **Lorsque les `n` minutes s'écoulent** : le runner libère chaque session qu'il détient toujours, inactive ou non. Le runner attend la fin du tour d'une session en cours de tour, et jusqu'à 60 secondes supplémentaires pour les tâches de fond d'un tour, avant de libérer cette session.
* **Lorsque la grâce post-libération s'écoule** : le runner draine les sessions qu'il détient toujours, et le plan de contrôle remet chaque session drainée en file d'attente vers un autre runner immédiatement. La grâce post-libération commence lorsque les `n` minutes s'écoulent et est de 75 secondes par défaut. Si vous définissez `--drain-wait-sec` au-dessus de 60 secondes, la grâce post-libération est `--drain-wait-sec` plus 15 secondes à la place.

À tout stade, le runner quitte 0 dès qu'il ne détient aucune session. Un deuxième signal raccourcit les stades : le runner draine immédiatement, comme il le fait au premier signal sans `--defer-shutdown-max-min`. Une fois qu'un drainage est en cours, le signal suivant force la sortie du runner. Cela s'applique qu'un deuxième signal ou l'expiration de la grâce post-libération ait commencé le drainage.

<h4 id="size-the-stop-timeout">
  Dimensionner le délai d'arrêt
</h4>

Donnez à votre délai d'arrêt de l'hôte au moins la somme de trois parties : les `n` minutes que vous configurez, la grâce post-libération, et le chemin de drainage complet que [Délai d'arrêt](#shutdown-timing) décrit. Avec les paramètres par défaut, la grâce post-libération est de 75 secondes et le chemin de drainage est de 80 secondes, donc autorisez `n` minutes plus 155 secondes. Le runner imprime cette somme au démarrage chaque fois que `--defer-shutdown-max-min` est défini.

Si le délai d'arrêt s'écoule avant que le runner ne termine, l'hôte tue le runner. Les sessions qu'il détient toujours ne reçoivent aucun hook `post-session`. Le runner ne se désenregistre pas, et le plan de contrôle remet les sessions en file d'attente environ une minute plus tard. Si vous ne pouvez pas donner au délai d'arrêt cette somme, laissez `--defer-shutdown-max-min` non défini pour que le runner draine au premier signal à la place.

<h3 id="what-reaches-a-running-post-session-hook">
  Ce qui atteint un hook post-session en cours d'exécution
</h3>

Le hook `post-session` et l'enfant de session Claude s'exécutent chacun dans leur propre groupe de processus POSIX, séparé de celui du runner, donc les mécanismes d'arrêt les atteignent différemment :

* **Un `SIGTERM` tandis que le runner draine déjà** : force la sortie du runner immédiatement, en sautant tout ce qui reste du chemin de drainage. Sans [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), c'est le deuxième `SIGTERM` que le runner reçoit. Rien ne signale un hook `post-session` en cours d'exécution, donc sur un hôte nu où un processus init adopte les orphelins, il se termine de lui-même, mais sans supervision : son budget de délai d'expiration ne s'applique plus, et une écriture au tuyau de journal fermé peut le tuer avec `SIGPIPE`, donc un hook qui doit survivre à une sortie forcée là devrait rediriger sa propre sortie vers un fichier. Dans les recettes de conteneur sur cette page, le runner est le PID 1 du conteneur et sa sortie termine le conteneur, et sous le `KillMode=control-group` par défaut de systemd, la suppression au niveau du cgroup atteint également le hook, comme l'entrée **Suppressions au niveau du cgroup** le décrit ; dans les deux cas, traitez une sortie forcée comme fatale au hook et fiez-vous à la période de grâce à la place.
* **Signaux au niveau du groupe de processus**, tels que `kill -- -<pid>` dans un script wrapper, le contrôle des tâches du shell, ou un watchdog au niveau du groupe : atteignent le runner et un sous-processus de hook `checkout` en cours, qui reste intentionnellement attaché au groupe, mais pas un hook `post-session` en cours d'exécution ou l'enfant de session.
* **Suppressions au niveau du cgroup**, telles que le `KillMode=control-group` par défaut de systemd ou le `SIGKILL` que Kubernetes livre à tout le conteneur lorsque `terminationGracePeriodSeconds` expire : atteignent tout, y compris le hook. L'isolation du groupe de processus ne protège pas contre celles-ci, c'est pourquoi la période de grâce doit couvrir le chemin de drainage complet.
* **Le délai d'expiration du hook lui-même** : lorsqu'un hook dépasse `--post-session-hook-timeout-sec`, le runner envoie `SIGTERM` à tout le groupe de processus du hook, puis `SIGKILL` deux secondes plus tard, donc un worker que le hook a forké, tel que tar, rsync, ou git, se termine avec le shell wrapper au lieu de survivre en tant qu'orphelin. La supervision du runner se termine une fois que le stdio du hook se ferme : un worker qui a redirigé sa propre sortie vers un fichier et survit à l'étape `SIGTERM` est au-delà de la portée du runner.

Lorsque le drainage commence, et à nouveau lors d'une sortie forcée, le runner enregistre combien de hooks `post-session` s'exécutent toujours, afin que vous puissiez distinguer un drainage silencieux d'un qui est en cours de snapshot.

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  Garder le répertoire de base et la capacité identiques sur tous les runners
</h2>

Si un runner meurt en milieu de session, le serveur remet la session en file d'attente et un autre runner dans l'environnement la récupère. Ce runner dérive le chemin de checkout de ses propres `--base-dir` et `--capacity` : `--capacity 1` se vérifie directement sous `--base-dir`, et un `--capacity` au-dessus de `1` utilise des worktrees par session à la place. Quand les runners dans le même environnement utilisent des valeurs différentes pour l'un ou l'autre drapeau, le répertoire de travail de la session reprise change, et les chemins absolus que l'agent a enregistrés plus tôt, dans les éditions, les appels d'outils, ou ses propres notes, pointent vers un emplacement qui n'existe plus.

Utilisez le même `--base-dir` et `--capacity` sur chaque runner dans un environnement, et n'utilisez pas une valeur par hôte comme un ID d'instance ou un nom d'hôte.

Le répertoire de base par défaut est `/workspace`, avec l'exception que la ligne de référence [`--base-dir`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) enregistre. Le runner a besoin d'accès en écriture à celui-ci. Au démarrage, avant de s'enregistrer, le runner crée le répertoire et confirme qu'il peut y écrire, et quitte avec `cannot create or write to base directory` quand il ne peut pas. Un runner démarré en tant que root crée le `/workspace` par défaut lui-même. Pour un runner non-root, créez le répertoire et donnez la propriété à l'utilisateur du runner avant de démarrer le runner, ou pointez `--base-dir` vers un répertoire que cet utilisateur possède déjà.

<h2 id="reuse-a-pre-warmed-checkout">
  Réutiliser un checkout pré-chauffé
</h2>

Pour les grands référentiels, le clone peut dominer le démarrage de la session. À `--capacity 1` sans [hook `checkout`](/docs/fr/self-hosted-environments-configuration#checkout), le runner garde un clone canonique par référentiel à `<base-dir>/<repo-owner>/<repo>` et le réutilise entre les sessions : il récupère la ref demandée, détache `HEAD`, et réinitialise dur à celle-ci, ce qui est quasi-instantané quand peu a changé. Pour sauter le clone froid, fournissez le clone de l'une de deux façons :

* **Clone dans l'image** : construisez le clone dans votre image runner à ce chemin. Chaque conteneur frais démarre alors avec le clone chaud sans réutiliser un disque.
* **Clone sur un volume persistant** : sur les runners que vous pré-verrouillez au compte d'un utilisateur avec [`--lock-to-account`](/docs/fr/self-hosted-environments-reference#runner-cli-flags), pointez `--base-dir` vers un volume persistant, pour que le disque ne serve que ce compte. Un runner pré-verrouillé ne récupère jamais les sessions de canal Claude Tag, donc cette option ne s'applique pas aux runners qui les servent.

Ce que le chemin de réutilisation fait et ne garantit pas :

* **N'importe quelle forme de clone fonctionne** : un clone complet, peu profond, ou à branche unique au chemin est utilisé tel quel. Le runner ne passe jamais `--depth` lors de la récupération dans un clone existant, donc un pré-chauffage complet garde son historique complet et un peu profond reste peu profond. `CLAUDE_RUNNER_FETCH_DEPTH` (`full`, `0`, ou un nombre ; par défaut 50) contrôle uniquement le clone froid que le runner fait quand aucun clone n'existe encore.
* **Les changements suivis se réinitialisent, les fichiers non suivis persistent** : chaque session commence à partir d'une réinitialisation dur qui efface les modifications suivies de la session précédente, mais le runner ne lance jamais `git clean`, donc les fichiers non suivis des sessions antérieures du propriétaire verrouillé restent dans l'arbre.
* **Les répertoires par session persistent aussi** : à côté du checkout, le runner crée des entrées par session sous `<base-dir>/_sessions/` pour chaque session qu'il exécute. Le répertoire de configuration Claude de la session contient une copie locale de la transcription de la conversation. À côté se trouvent les fichiers téléchargés de la session, quand la session en a. Le répertoire de session s'y trouve aussi : il contient tous les worktrees par session et les checkouts du hook `checkout` pendant que la session s'exécute, et il conserve tout ce que Claude a écrit dedans.

  Par défaut, le runner les laisse en place quand la session se termine, donc sur un disque qui survit au processus runner, ils s'accumulent. Chaque session s'exécute en tant qu'utilisateur du runner, donc toute session ultérieure que ce disque sert peut les lire. Si vous conservez un `--base-dir` persistant, dimensionnez le volume pour cette croissance. La même chose s'applique à toute configuration qui redémarre le runner sur le même système de fichiers, y compris la [recette Docker Compose](#docker-compose).
* **Avec `--remove-session-state`, les répertoires par session ne persistent pas** : démarrez le runner avec [`--remove-session-state`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) pour qu'il supprime les répertoires par session de chaque session à la fin de la session. La suppression est au mieux : les répertoires restent quand le runner est tué avant l'exécution de son nettoyage. Le clone canonique et les fichiers qu'une session a écrits ailleurs sur l'hôte, comme le répertoire temporaire, restent indépendamment.
* **Avec le proxy git, la réinitialisation devient un checkout** : avec [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), le runner assainit le `.git/` du clone avant chaque session, gardant le magasin d'objets, les refs, et l'état peu profond mais supprimant l'index, donc chaque session paie un checkout complet de l'arbre de travail au lieu d'une réinitialisation quasi-instantanée ; il ne re-clone toujours pas. Les pré-chauffages de sous-module ne sont pas supportés sous le proxy.
* **Les longs clones n'ont besoin d'aucune solution de contournement** : le runner limite chaque opération git avec un watchdog sans progrès de 120 secondes et un plafond dur de 30 minutes, pas un délai d'expiration plat, donc un clone froid lent qui continue de signaler le progrès se termine.

<h2 id="pin-the-version">
  Épingler la version
</h2>

Le processus enfant Claude Code de chaque session exécute le binaire du runner lui-même, et le runner désactive la mise à jour automatique à l'intérieur des sessions qu'il génère, donc chaque session exécute la version que vous avez installée sur l'hôte ou construite dans l'image. Une mise à jour au niveau de l'hôte prend effet la prochaine fois que le runner démarre.

* **Pour garder une flotte sur une version** : construisez l'image avec une version épinglée, ou sur un hôte nu installez une version spécifique et [désactivez les mises à jour automatiques](/docs/fr/setup#disable-auto-updates)
* **Pour mettre à niveau** : installez la version plus récente ou reconstruisez l'image, puis redémarrez les runners
* **Plugins** : les places de marché de plugins ne se mettent pas à jour automatiquement non plus ; définissez `FORCE_AUTOUPDATE_PLUGINS=1` dans l'environnement du runner pour laisser les plugins se mettre à jour automatiquement pendant que le binaire reste épinglé

<h2 id="scale-the-fleet">
  Mettre à l'échelle la flotte
</h2>

Votre orchestrateur décide quand ajouter ou supprimer des runners. En raison du [verrou d'un propriétaire par runner](/docs/fr/self-hosted-environments#runner-lifecycle), le nombre minimum de répliques est le nombre d'utilisateurs et d'agents Claude Tag que vous vous attendez à être actifs simultanément ; `--capacity` contrôle le parallélisme au sein des sessions d'un propriétaire, pas entre les propriétaires.

Deux approches de mise à l'échelle sont disponibles :

* **Flotte fixe** : exécutez un ensemble statique de répliques runner et mettez à l'échelle sur les [métriques Prometheus](/docs/fr/self-hosted-environments-reference#prometheus-metrics) que chaque runner sert
* **Runners à la demande** : exécutez la sous-commande `claude self-hosted-runner orchestrator`, qui interroge Anthropic pour les sessions en attente sans runner disponible et invoque votre hook `spawn-runner` pour en démarrer un par session. Consultez [Runners à la demande](/docs/fr/self-hosted-environments-configuration#on-demand-runners).

<h2 id="known-issues-and-limitations">
  Problèmes connus et limitations
</h2>

Voici les limitations de cette version, avec des solutions de contournement où l'une existe.

<h3 id="connector-traffic-leaves-your-network">
  Le trafic du connecteur quitte votre réseau
</h3>

Anthropic appelle les outils connecteur à partir de sa propre infrastructure plutôt que de votre runner. Les outils connecteur sont les connecteurs claude.ai, tels que GitHub, Slack et Linear. Quand Claude utilise un connecteur dans une session auto-hébergée, ce trafic passe par `api.anthropic.com` plutôt que d'originer à l'intérieur de votre limite réseau.

Pour garder un connecteur hors des sessions auto-hébergées, filtrez-le avec les [paramètres de politique `allowedMcpServers` et `deniedMcpServers`](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists). Claude Code applique ces paramètres aux connecteurs qu'Anthropic livre ainsi qu'aux serveurs que vous configurez à partir de l'hôte runner et aux serveurs que les utilisateurs ajoutent, donc si vous déployez une liste d'autorisation pour d'autres serveurs, Claude Code bloque également les connecteurs livrés. Pour garder les connecteurs disponibles aux côtés d'une liste d'autorisation basée sur URL, ajoutez des entrées qui correspondent aux chemins de proxy Anthropic pour les connecteurs livrés :

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

Si le trafic d'outils doit rester à l'intérieur de votre réseau, exécutez les outils équivalents en tant que serveurs MCP locaux sur l'image runner à la place. Consultez [Serveurs MCP](/docs/fr/self-hosted-environments-configuration#mcp-servers).

<h3 id="some-sessions-don’t-count-as-idle">
  Certaines sessions ne comptent pas comme inactives
</h3>

Une session tenant une tâche de fond qui ne se termine jamais ne compte pas comme inactive, donc `--release-idle-session-min` ne libérera pas l'emplacement de cette session. Une session qui attend une approbation demandée de l'intérieur d'un appel d'outil en cours d'exécution ne compte pas non plus comme inactive. Définissez toujours `--kill-session-after-min` à côté comme un arrêt dur pour qu'aucune session ne puisse tenir un emplacement indéfiniment.

`--kill-session-after-min` est un arrêt dur pour les sessions qui s'échappent. Sur un runner en v2.1.260 ou ultérieur, une session qui atteint la limite n'est pas terminée immédiatement. Le runner lui donne une fenêtre de grâce, 15 minutes par défaut, que vous pouvez modifier avec [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/fr/self-hosted-environments-reference#environment-variable-only-settings) :

* Si la session attend son utilisateur, le runner la libère. Si son tour s'est terminé et elle ne contient que des tâches de fond, le runner attend jusqu'à 60 secondes pour que ces tâches se terminent, puis la libère. La session reprend quand son utilisateur envoie son prochain message.
* Si un tour est toujours en cours d'exécution, le runner attend que le tour se termine, ou que la session attende ensuite son utilisateur, puis la libère.
* Si la session est toujours sur le runner quand la fenêtre de grâce se termine, le runner la termine, et tout travail du tour en cours d'exécution est perdu. Un tour attendant une approbation demandée de l'intérieur d'un appel d'outil en cours d'exécution est une façon pour une session de dépasser la fenêtre.

Une session libérée reprend à partir d'un clone frais, donc le travail qu'elle n'avait pas poussé est parti de toute façon ; consultez [Les sessions reprises perdent le travail non poussé](#additional-limitations). Avant v2.1.260, le runner terminait chaque session à la limite, après avoir attendu au maximum la fenêtre de grâce pour qu'un tour en cours d'exécution se termine.

Définissez le drapeau au-dessus de votre session la plus longue attendue, comme `--kill-session-after-min 480` pour 8 heures. Pour libérer les emplacements des conversations qui deviennent inactives, utilisez `--release-idle-session-min` à la place.

<h3 id="additional-limitations">
  Limitations supplémentaires
</h3>

* **Les sessions reprises perdent le travail non poussé** : quand une session est libérée ou son runner est redémarré, et que l'utilisateur envoie un autre message, la session reprend sur un runner frais qui clone le référentiel à nouveau à partir de sa branche de démarrage, donc le travail que la session n'avait pas poussé est parti. Définissez [`--push-outcome-on-release`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) pour que le runner fasse un meilleur effort de push des branches de résultat de la session avant de la libérer, pour que la session reprise commence à partir de ces commits à la place ; cela préserve le travail engagé, pas un arbre de travail sale. Avant de l'activer, limitez qui peut pousser vers les refs `claude/*` sur la télécommande source, par exemple avec une règle de branche : à la reprise, le runner récupère la branche précédemment poussée sans vérifier qui l'a poussée, donc n'importe qui avec accès push à ces refs peut placer du contenu dans l'espace de travail repris. Le runner rejette également la configuration par session à la reprise, ce qui signifie le répertoire de configuration Claude de la session et tout état de shell que la session a écrit ; `--push-outcome-on-release` ne couvre pas ceux-ci.
* **Les référentiels privés ne peuvent pas être ajoutés en milieu de session** : un référentiel ajouté à une session après son démarrage n'est pas cloné avec des identifiants sur un runner auto-hébergé, donc l'ajout échoue. Sélectionnez chaque référentiel dont la session a besoin quand vous la créez.
* **Certains connecteurs n'apparaissent pas dans les sessions auto-hébergées** : un connecteur que vous n'avez pas encore connecté dans les paramètres claude.ai n'est pas listé dans une session auto-hébergée, et la session ne vous invitera pas à le connecter. Connectez-le d'abord dans les paramètres, puis démarrez une session fraîche. Ajouter un connecteur à une session déjà en cours d'exécution ne rend pas non plus ses outils disponibles à Claude ; démarrez une session fraîche pour récupérer un connecteur nouvellement ajouté.

<h3 id="report-an-issue">
  Signaler un problème
</h3>

Pour les problèmes avec les environnements auto-hébergés, contactez votre équipe de compte Anthropic.

<h2 id="troubleshooting">
  Dépannage
</h2>

Pour un diagnostic guidé, exécutez la sous-commande doctor sur l'hôte runner. La sous-commande doctor démarre une session Claude Code interactive avec les journaux et l'état du runner attachés. Connectez-vous d'abord avec `claude auth login` sur cet hôte pour que la session puisse interroger votre environnement, ses runners, et ses sessions en attente. Sans cette connexion, par exemple quand l'hôte s'authentifie avec une clé API, il est limité au point de terminaison de santé local, aux métriques, et au journal du runner, et il lit le journal uniquement si vous avez démarré le runner avec `--log-file`.

```bash theme={null}
claude self-hosted-runner doctor
```

Problèmes courants :

* **Le runner n'apparaît pas dans l'environnement** : confirmez que l'hôte peut atteindre `api.anthropic.com` sur HTTPS, que le secret de l'environnement est actuel, et que l'horloge de l'hôte est à moins de cinq minutes de l'heure réelle ; un décalage plus grand cause l'échec de l'authentification. Le runner enregistre `[runner:fatal]` avec la raison du rejet en cas d'échec d'authentification.
* **Le runner quitte au démarrage avec `cannot create or write to base directory`** : le runner ne peut pas créer ou écrire à `--base-dir`, qui par défaut est `/workspace`. Corrigez la propriété du répertoire ou pointez `--base-dir` vers un chemin inscriptible, comme décrit dans [Garder le répertoire de base et la capacité identiques sur tous les runners](#keep-the-base-directory-and-capacity-identical-across-runners). Si le runner enregistre à la place `[runner:fatal]` disant que la vérification du répertoire de base a expiré, le répertoire est sur un montage NFS ou CSI suspendu. Vérifiez la santé du montage plutôt que les permissions. Le runner imprime ces deux échecs de démarrage à stderr avant d'ouvrir `--log-file`, donc cherchez-les dans le terminal ou les journaux de conteneur de votre plateforme plutôt que dans le fichier journal. Avant v2.1.225, le runner ne vérifiait pas le répertoire de base au démarrage, et cette mauvaise configuration échouait les sessions après la récupération à la place.
* **Les sessions restent en attente** : chaque runner en ligne peut être verrouillé à un propriétaire différent. Vérifiez la métrique `claude_code_self_hosted_runner_locked_account` de chaque runner [métrique](/docs/fr/self-hosted-environments-reference#prometheus-metrics) ou le champ `locked_account` de sa ligne de journal `[runner:health]` pour voir qui la détient. Les deux affichent l'email du propriétaire uniquement après que le runner ait reçu un jeton de session portant une réclamation `act.email`, ce qu'une session d'agent Claude Tag ne fait jamais. Sans la réclamation, le runner n'émet aucune série `locked_account` et enregistre `locked_account=yes`, ce qui vous dit que le runner est verrouillé mais pas à quel propriétaire. Ajoutez des répliques, ou attendez qu'un runner existant se draine et redémarre. Si l'environnement utilise des runners à la demande, vérifiez l'orchestrateur à la place ; consultez [Runners à la demande](/docs/fr/self-hosted-environments-configuration#on-demand-runners).
* **Les sessions échouent immédiatement après la récupération** : ouvrez la session dans claude.ai/code pour voir l'erreur. Les causes les plus courantes sont les identifiants git manquants [identifiants git](#configure-git) dans l'image runner et les outils de build qui ne sont pas installés. Un répertoire de base non inscriptible arrête le runner au démarrage au lieu d'échouer les sessions. Consultez l'entrée **Le runner quitte au démarrage avec `cannot create or write to base directory`** dans cette liste.
* **Les sessions ne peuvent pas atteindre le réseau via un proxy de sortie authentifiant** : quand la source que vous avez définie avec [`--proxy-authorization-command` ou `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) échoue, expire après 30 secondes, ou produit une valeur vide, le runner répond à cette connexion `502 Bad Gateway` et enregistre pourquoi. Le runner rédige la stderr de la commande dans ce journal et ne journalise jamais la valeur de l'en-tête. Avec `--proxy-authorization-command`, exécutez la commande vous-même sur l'hôte pour confirmer qu'elle imprime la valeur d'en-tête entière sur stdout. Si le runner quitte à la place au démarrage avec `could not start the proxy-authorization listener`, il ne pouvait pas ouvrir son écouteur de boucle locale.
* **Le runner enregistre des lignes `Poll failed` contenant `rejecting the malformed poll response`** : le runner a reçu une réponse de sondage dont le corps n'est pas le JSON attendu de la file d'attente, le plus souvent parce que quelque chose entre le runner et `api.anthropic.com`, comme un proxy d'interception ou un portail captif, a répondu avec sa propre page. Le runner rejette la réponse, la compte sous le type `transport` de la [métrique](/docs/fr/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_poll_errors_total`, et réessaie selon le calendrier de sondage échoué décrit dans [Cycle de vie de la session](/docs/fr/self-hosted-environments#session-lifecycle). Le runner continue de servir ses sessions en direct. Configurez le proxy pour passer les réponses de `api.anthropic.com` inchangées. Avant v2.1.246, le runner lisait une telle réponse comme une file d'attente de travail vide, ce qui pouvait terminer ses sessions en direct ou le faire quitter.
* **La branche d'une session n'existe plus sur la télécommande** : pour une source git que la session ne lit que, le runner saute cette source et continue sur les autres. Pour la source vers laquelle la session pousse les résultats, une branche supprimée, généralement parce qu'elle a été fusionnée et supprimée automatiquement, échoue la session avec une erreur nommant le référentiel et la branche et vous demandant de restaurer la branche et de réessayer. Le runner échoue la session avec la même erreur quand sauter laisserait sans référentiel du tout. Avant v2.1.228, une telle session démarrait dans un répertoire vide.
* **Une session démarre sans l'un de ses référentiels** : sur un runner sans [hook `checkout`](/docs/fr/self-hosted-environments-configuration#checkout), l'hôte git peut refuser la vérification d'accès du runner pour un référentiel que la session ne lit que. Le runner saute alors ce référentiel, enregistre une ligne `[runner:warn] could not access context source` nommant le refus, et démarre la session sur les autres.

  Le runner saute uniquement un refus clair : l'hôte répond que le référentiel n'a pas été trouvé, git ne trouve aucune identifiants pour l'hôte, ou l'authentification échoue. Une défaillance réseau, un délai d'expiration, ou un HTTP `403` échoue toujours le démarrage de la session, tout comme un refus pour un référentiel vers lequel la session pousse les résultats. Le runner échoue toujours une session que sauter laisserait sans référentiel du tout. Avec [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), le runner saute uniquement un référentiel que le proxy git lui-même refuse.

  La vérification d'accès s'exécute à nouveau chaque fois que la session démarre sur un runner, donc une fois que l'identité git du runner a accès en lecture, le prochain démarrage clone le référentiel. Avant v2.1.274, chacun de ces refus échouait le démarrage de la session.
* **Les sessions prennent des minutes pour démarrer** : le clone initial domine généralement. Regardez la [métrique](/docs/fr/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_session_init_duration_seconds` pour confirmer, et coupez le clone avec un [checkout pré-chauffé](#reuse-a-pre-warmed-checkout) ou un `CLAUDE_RUNNER_FETCH_DEPTH` plus petit.
* **Les tours échouent avec un 401** : chaque session authentifie les appels de modèle avec le [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/fr/self-hosted-environments-configuration#wrapper-scripts) de courte durée que le runner récupère auprès d'Anthropic et fait tourner sur stdin de la session. Quand un tour se termine avec un 401 ou 403 de l'API du modèle, le runner récupère un jeton frais et le transmet à la session. Le tour échoué n'est pas réessayé.

  Quand une récupération échoue, le runner enregistre une ligne `inference_token refresh failed` qui dit quand il réessayera, et il continue à réessayer aussi longtemps que la session s'exécute.

  Si chaque appel commence à échouer environ 30 minutes dans une session, un script wrapper a probablement coupé stdin de la session, donc les rotations de jeton ne peuvent pas l'atteindre ; consultez [Garder stdin et le descripteur de fichier 3 attachés](/docs/fr/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached).

  Avant v2.1.274, le runner arrêtait de réessayer une récupération échouée après quelques tentatives et attendait la prochaine programmée. Un tour échoué n'a pas déclenché une récupération, donc chaque tour échouait avec un 401 jusqu'à la prochaine récupération programmée.
* **Le pod est tué en milieu de drainage** : augmentez `terminationGracePeriodSeconds` à au moins la valeur que le runner enregistre au démarrage. Consultez [Timing d'arrêt](#shutdown-timing).

Une fois que la journalisation est initialisée, le runner écrit son journal de cycle de vie, y compris les lignes `[runner:fatal]`, à stdout, et la sortie de débogage à stderr, tout comme des lignes en texte brut plutôt que JSON. Les échecs de démarrage décrits dans les entrées de dépannage ci-dessus s'impriment à stderr avant ce point. Capturez les deux flux avec `--log-file`, ce qui permet également à `self-hosted-runner doctor` de les suivre, ou avec la collecte de journaux de votre plateforme.

Le processus enfant de chaque session écrit un journal de débogage séparé. En cas d'échec, le runner affiche la queue du journal aux côtés de la session dans claude.ai/code. À moins que vous n'ayez démarré le runner avec [`--remove-session-state`](/docs/fr/self-hosted-environments-reference#runner-cli-flags), il conserve également le journal d'une session échouée sur le disque et imprime son chemin dans le journal du runner.

<h2 id="what’s-next">
  Prochaines étapes
</h2>

* [Personnaliser les sessions](/docs/fr/self-hosted-environments-configuration) : scripts wrapper, hooks de cycle de vie, runners à la demande, serveurs MCP, et permissions
* [Tester de bout en bout](/docs/fr/self-hosted-environments-testing) : vérifier une nouvelle image runner à partir de CI avant de la promouvoir
* [Référence](/docs/fr/self-hosted-environments-reference) : chaque drapeau CLI, variable d'environnement, et métrique
