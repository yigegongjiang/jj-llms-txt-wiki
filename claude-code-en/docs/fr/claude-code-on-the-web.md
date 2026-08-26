> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser Claude Code sur le web

> Configurez les environnements cloud, les scripts de configuration, l'accès réseau et Docker dans le sandbox d'Anthropic. Déplacez les sessions entre le web et le terminal avec `--cloud` et `--teleport`.

<Note>
  Claude Code sur le web est en aperçu de recherche pour les utilisateurs Pro, Max et Team, ainsi que pour les utilisateurs Enterprise disposant de sièges premium ou de sièges Chat + Claude Code.
</Note>

Claude Code sur le web exécute les tâches sur l'infrastructure cloud gérée par Anthropic à [claude.ai/code](https://claude.ai/code). Les sessions persistent même si vous fermez votre navigateur, et vous pouvez les surveiller depuis l'application mobile Claude.

<Tip>
  Nouveau sur Claude Code sur le web ? Commencez par [Démarrer](/docs/fr/web-quickstart) pour connecter votre compte GitHub et soumettre votre première tâche.
</Tip>

Cette page couvre :

* [Options d'authentification GitHub](#github-authentication-options) : deux façons de connecter GitHub
* [L'environnement cloud](#the-cloud-environment) : quelle configuration est transférée, quels outils sont installés et comment configurer les environnements
* [Scripts de configuration](#setup-scripts) et gestion des dépendances
* [Accès réseau](#network-access) : niveaux, proxies et liste d'autorisation par défaut
* [Déplacer les tâches entre le web et le terminal](#move-tasks-between-web-and-terminal) avec `--cloud` et `--teleport`
* [Travailler avec les sessions](#work-with-sessions) : examiner, partager, archiver, supprimer
* [Correction automatique des demandes de tirage](#auto-fix-pull-requests) : répondre automatiquement aux défaillances CI et aux commentaires d'examen
* [Sécurité et isolation](#security-and-isolation) : comment les sessions sont isolées
* [Limitations](#limitations) : limites de débit et restrictions de plateforme

<h2 id="github-authentication-options">
  Options d'authentification GitHub
</h2>

Les sessions cloud ont besoin d'accès à vos référentiels GitHub pour cloner le code et pousser les branches. Vous pouvez accorder l'accès de deux façons :

| Méthode                | Comment ça marche                                                                                                | Idéal pour                                                                |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| **Application GitHub** | Autorisez l'application Claude GitHub lors de [l'intégration web](/docs/fr/web-quickstart).                           | Intégration web ; équipes qui veulent [Auto-fix](#auto-fix-pull-requests) |
| **`/web-setup`**       | Exécutez `/web-setup` dans votre terminal pour synchroniser votre jeton CLI `gh` local vers votre compte Claude. | Développeurs individuels qui utilisent déjà `gh`                          |

<Note>
  Avec l'une ou l'autre méthode, une session cloud peut accéder à n'importe quel référentiel que le compte GitHub connecté peut voir, pas seulement les référentiels sur lesquels l'application Claude GitHub est installée. L'installation de l'application active les webhooks PR pour [Auto-fix](#auto-fix-pull-requests) ; ce n'est pas un contrôle d'accès au niveau de la session. Pour restreindre les référentiels que votre équipe peut atteindre à partir des sessions cloud, restreignez l'accès sur GitHub lui-même, par exemple en limitant l'appartenance à l'équipe ou au référentiel pour les comptes GitHub connectés.
</Note>

L'une ou l'autre méthode fonctionne. [`/schedule`](/docs/fr/routines) vérifie l'une ou l'autre forme d'accès et vous invite à exécuter `/web-setup` si aucune n'est configurée. Consultez [Connecter depuis votre terminal](/docs/fr/web-quickstart#connect-from-your-terminal) pour la procédure pas à pas de `/web-setup`.

L'application GitHub est requise pour [Auto-fix](#auto-fix-pull-requests), qui utilise l'application pour recevoir les webhooks PR. Si vous vous connectez avec `/web-setup` et souhaitez ultérieurement Auto-fix, installez l'application sur ces référentiels.

Les administrateurs Team et Enterprise peuvent désactiver `/web-setup` avec le bouton bascule Quick web setup sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

<Note>
  Les organisations avec [Zéro rétention de données](/docs/fr/zero-data-retention) activée ne peuvent pas utiliser `/web-setup` ou d'autres fonctionnalités de session cloud.
</Note>

<h2 id="the-cloud-environment">
  L'environnement cloud
</h2>

Chaque session s'exécute dans une VM fraîche gérée par Anthropic avec votre référentiel cloné. Cette section couvre ce qui est disponible au démarrage d'une session et comment la personnaliser.

<h3 id="what’s-available-in-cloud-sessions">
  Ce qui est disponible dans les sessions cloud
</h3>

Les sessions cloud commencent par un clone frais de votre référentiel. Tout ce qui est validé dans le référentiel est disponible. Tout ce que vous avez installé ou configuré uniquement sur votre propre machine ne l'est pas dans la session. La politique de votre organisation arrive séparément via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings).

|                                                                                        | Disponible dans les sessions cloud | Pourquoi                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------------------------------------------------------- | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Votre `CLAUDE.md` du référentiel                                                       | Oui                                | Fait partie du clone                                                                                                                                                                                                                                                                                                                                                                       |
| Vos hooks `.claude/settings.json` du référentiel                                       | Oui                                | Fait partie du clone                                                                                                                                                                                                                                                                                                                                                                       |
| Vos serveurs MCP `.mcp.json` du référentiel                                            | Oui                                | Fait partie du clone                                                                                                                                                                                                                                                                                                                                                                       |
| Votre `.claude/rules/` du référentiel                                                  | Oui                                | Fait partie du clone                                                                                                                                                                                                                                                                                                                                                                       |
| Votre `.claude/skills/`, `.claude/agents/`, `.claude/commands/` du référentiel         | Oui                                | Fait partie du clone                                                                                                                                                                                                                                                                                                                                                                       |
| Plugins déclarés dans `.claude/settings.json`                                          | Oui                                | Installés au démarrage de la session à partir de la [marketplace](/docs/fr/plugin-marketplaces) que vous avez déclarée. Nécessite un accès réseau pour atteindre la source de la marketplace                                                                                                                                                                                                    |
| Votre [paramètres gérés par le serveur](/docs/fr/server-managed-settings) de l'organisation | Oui                                | Récupérés depuis les serveurs d'Anthropic au démarrage de la session. Consultez [Couverture de surface](/docs/fr/model-config#surface-coverage) pour savoir comment `availableModels` est appliqué dans les sessions cloud. Les paramètres déployés sur votre appareil via MDM ou des fichiers de paramètres gérés ne s'appliquent pas, car la session s'exécute sur une VM gérée par Anthropic |
| Votre `~/.claude/CLAUDE.md` utilisateur                                                | Non                                | Vit sur votre machine, pas dans le référentiel                                                                                                                                                                                                                                                                                                                                             |
| Votre `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` utilisateur      | Non                                | Vivent sur votre machine, pas dans le référentiel. Validez-les plutôt dans le répertoire `.claude/` du référentiel. Les compétences que vous activez sur claude.ai sont chargées automatiquement dans les sessions cloud                                                                                                                                                                   |
| Plugins activés uniquement dans vos paramètres utilisateur                             | Non                                | Les `enabledPlugins` limités à l'utilisateur vivent dans `~/.claude/settings.json`. Déclarez-les plutôt dans le `.claude/settings.json` du référentiel                                                                                                                                                                                                                                     |
| Serveurs MCP que vous avez ajoutés avec `claude mcp add`                               | Non                                | Ceux-ci écrivent dans votre configuration utilisateur locale, pas dans le référentiel. Déclarez le serveur dans [`.mcp.json`](/docs/fr/mcp#project-scope) à la place                                                                                                                                                                                                                            |
| Jetons API statiques et identifiants                                                   | Non                                | Aucun magasin de secrets dédié n'existe encore. Voir ci-dessous                                                                                                                                                                                                                                                                                                                            |
| Authentification interactive comme AWS SSO                                             | Non                                | Non pris en charge. SSO nécessite une connexion basée sur le navigateur qui ne peut pas s'exécuter dans une session cloud                                                                                                                                                                                                                                                                  |

Pour rendre votre propre configuration disponible dans les sessions cloud, validez-la dans le référentiel ; la politique de l'organisation arrive séparément via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings).

Un magasin de secrets dédié n'est pas encore disponible. Les variables d'environnement et les scripts de configuration sont stockés dans la configuration de l'environnement, visibles à quiconque peut modifier cet environnement. Si vous avez besoin de secrets dans une session cloud, ajoutez-les comme variables d'environnement en gardant cette visibilité à l'esprit.

<h3 id="installed-tools">
  Outils installés
</h3>

Les sessions cloud sont livrées avec des runtimes de langage courants, des outils de construction et des bases de données pré-installés. Le tableau ci-dessous résume ce qui est inclus par catégorie.

| Catégorie            | Inclus                                                                           |
| :------------------- | :------------------------------------------------------------------------------- |
| **Python**           | Python 3.x avec pip, poetry, uv, black, mypy, pytest, ruff                       |
| **Node.js**          | 20, 21 et 22 via nvm, avec npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**             | 3.1, 3.2, 3.3 avec gem, bundler, rbenv                                           |
| **PHP**              | 8.4 avec Composer                                                                |
| **Java**             | OpenJDK 21 avec Maven et Gradle                                                  |
| **Go**               | dernière version stable avec support des modules                                 |
| **Rust**             | rustc et cargo                                                                   |
| **C/C++**            | GCC, Clang, cmake, ninja, conan                                                  |
| **Docker**           | docker, dockerd, docker compose                                                  |
| **Bases de données** | PostgreSQL 16, Redis 7.0                                                         |
| **Utilitaires**      | git, jq, yq, ripgrep, tmux, vim, nano                                            |

¹ Bun est installé mais a des [problèmes de compatibilité proxy](#install-dependencies-with-a-sessionstart-hook) connus pour la récupération de paquets.

Pour les versions exactes, demandez à Claude d'exécuter `check-tools` dans une session cloud. Cette commande n'existe que dans les sessions cloud.

<h3 id="work-with-github-issues-and-pull-requests">
  Travailler avec les problèmes et demandes de tirage GitHub
</h3>

Les sessions cloud incluent des outils GitHub intégrés qui permettent à Claude de lire les problèmes, de lister les demandes de tirage, de récupérer les diffs et de publier des commentaires sans aucune configuration. Ces outils s'authentifient via le [proxy GitHub](#github-proxy) en utilisant la méthode que vous avez configurée sous [Options d'authentification GitHub](#github-authentication-options), donc votre jeton n'entre jamais dans le conteneur.

Vous pouvez définir `GH_TOKEN` ou `GITHUB_TOKEN` vous-même dans les [paramètres d'environnement](#configure-your-environment), ou laisser les deux non définis et laisser le [proxy GitHub](#github-proxy) vous authentifier :

* Si vous définissez un jeton, il passe au conteneur inchangé, donc `gh` et vos scripts l'utilisent directement.
* Si vous ne définissez aucun des deux, le conteneur définit les deux variables sur la chaîne d'espace réservé `proxy-injected` et le proxy substitue vos vraies identifiants sur les demandes GitHub sortantes. `gh` fonctionne sans jeton personnel, mais un script qui lit `GITHUB_TOKEN` directement obtient l'espace réservé, pas un jeton utilisable.

Pour vérifier quel cas s'applique à votre session, demandez à Claude d'exécuter `echo $GH_TOKEN`.

Le CLI `gh` n'est pas pré-installé. Si vous avez besoin d'une commande `gh` que les outils intégrés ne couvrent pas, comme `gh release` ou `gh workflow run`, installez et authentifiez-la vous-même :

<Steps>
  <Step title="Installer gh dans votre script de configuration">
    Ajoutez `apt update && apt install -y gh` à votre [script de configuration](#setup-scripts).
  </Step>

  <Step title="Fournir un jeton si le proxy ne gère pas l'authentification">
    Si `echo $GH_TOKEN` affiche `proxy-injected`, le [proxy GitHub](#github-proxy) authentifie `gh` pour vous et cette étape est inutile. Sinon, ajoutez une variable d'environnement `GH_TOKEN` à vos [paramètres d'environnement](#configure-your-environment) avec un jeton d'accès personnel GitHub. `gh` lit `GH_TOKEN` automatiquement, donc aucune étape `gh auth login` n'est nécessaire.
  </Step>
</Steps>

<h3 id="link-output-back-to-the-session">
  Lier la sortie à la session
</h3>

Chaque session cloud a une URL de transcription sur claude.ai, et la session peut lire son propre ID à partir de la variable d'environnement `CLAUDE_CODE_REMOTE_SESSION_ID`. Utilisez ceci pour mettre un lien traçable dans les corps PR, les messages de commit, les publications Slack ou les rapports générés afin qu'un examinateur puisse ouvrir l'exécution qui les a produits.

À partir de la v2.1.179, les commits que Claude crée dans une session web incluent une remorque git `Claude-Session: <url>`, et les corps PR incluent l'URL de la session sur sa propre ligne. À partir de la v2.1.182, définissez [`attribution.sessionUrl`](/docs/fr/settings#attribution-settings) sur `false` pour omettre la remorque et le lien du corps PR.

Pour inclure le lien de session dans quelque chose d'autre qu'un commit ou une PR, comme un message Slack que Claude publie ou un fichier de rapport qu'il écrit, demandez à Claude d'exécuter la commande suivante et d'utiliser sa sortie. La commande convertit le préfixe `cse_` dans la valeur de la variable d'environnement au préfixe `session_` que l'URL de transcription attend :

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Exécuter les tests, démarrer les services et ajouter des paquets
</h3>

Claude exécute les tests dans le cadre du travail sur une tâche. Demandez-le dans votre invite, comme « corriger les tests échoués dans `tests/` » ou « exécuter pytest après chaque modification ». Les exécuteurs de tests comme pytest, jest et cargo test sont pré-installés et fonctionnent sans configuration supplémentaire.

PostgreSQL et Redis sont pré-installés mais ne s'exécutent pas par défaut. Demandez à Claude de démarrer chacun pendant la session :

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker est disponible pour exécuter les services conteneurisés. Demandez à Claude d'exécuter `docker compose up` pour démarrer les services de votre projet. L'accès réseau pour extraire les images suit le [niveau d'accès](#access-levels) de votre environnement, et les [valeurs par défaut de confiance](#default-allowed-domains) incluent Docker Hub et d'autres registres courants.

Si vos images sont volumineuses ou lentes à extraire, ajoutez `docker compose pull` ou `docker compose build` à votre [script de configuration](#setup-scripts). Les images extraites sont sauvegardées dans l'[environnement en cache](#environment-caching), donc chaque nouvelle session les a sur le disque. Le cache stocke uniquement les fichiers, pas les processus en cours d'exécution, donc Claude démarre toujours les conteneurs à chaque session.

Pour ajouter des paquets qui ne sont pas pré-installés, utilisez un [script de configuration](#setup-scripts). La sortie du script est [mise en cache](#environment-caching), donc les paquets que vous installez là sont disponibles au démarrage de chaque session sans réinstallation à chaque fois. Vous pouvez également demander à Claude d'installer des paquets pendant la session, mais ces installations ne persistent pas entre les sessions.

<h3 id="resource-limits">
  Limites de ressources
</h3>

Les sessions cloud s'exécutent avec des plafonds de ressources approximatifs qui peuvent changer au fil du temps :

* 4 vCPU
* 16 Go de RAM
* 30 Go de disque

Les tâches nécessitant beaucoup plus de mémoire, comme les gros travaux de construction ou les tests gourmands en mémoire, peuvent échouer ou être terminées. Pour les charges de travail au-delà de ces limites, utilisez [Contrôle à distance](/docs/fr/remote-control) pour exécuter Claude Code sur votre propre matériel.

<h3 id="configure-your-environment">
  Configurer votre environnement
</h3>

Les environnements contrôlent [l'accès réseau](#network-access), les variables d'environnement et le [script de configuration](#setup-scripts) qui s'exécute avant le démarrage d'une session. Consultez [Outils installés](#installed-tools) pour ce qui est disponible sans aucune configuration. Vous pouvez gérer les environnements à partir de l'interface web ou du terminal :

| Action                                                         | Comment                                                                                                                                                                                                                                                                        |
| :------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ajouter un environnement                                       | Sélectionnez l'environnement actuel pour ouvrir le sélecteur, puis sélectionnez **Ajouter un environnement**. La boîte de dialogue inclut le nom, le niveau d'accès réseau, les variables d'environnement et le script de configuration.                                       |
| Modifier un environnement                                      | Sélectionnez l'icône cloud affichant le nom de l'environnement actuel pour ouvrir le sélecteur, survolez un environnement et cliquez sur l'icône des paramètres qui apparaît à droite.                                                                                         |
| Archiver un environnement                                      | Ouvrez l'environnement pour le modifier et sélectionnez **Archiver**. Les environnements archivés sont masqués du sélecteur mais les sessions existantes continuent de s'exécuter.                                                                                             |
| Définir l'environnement par défaut pour les sessions cloud CLI | Exécutez `/remote-env` dans votre terminal. Si vous avez un seul environnement, cette commande affiche votre configuration actuelle. `/remote-env` sélectionne uniquement la valeur par défaut ; ajoutez, modifiez et archivez les environnements à partir de l'interface web. |

Les variables d'environnement utilisent le format `.env` avec une paire `KEY=value` par ligne. N'enveloppez pas les valeurs entre guillemets, car les guillemets sont stockés comme faisant partie de la valeur. Cet exemple définit trois variables :

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

<h3 id="organization-shared-environments">
  Environnements partagés par l'organisation
</h3>

Les propriétaires et administrateurs des plans Team et Enterprise peuvent créer des environnements cloud qui sont partagés avec chaque membre de l'organisation. Les environnements partagés apparaissent dans le sélecteur d'environnement de chaque membre aux côtés de leurs environnements personnels, afin qu'une équipe puisse standardiser une configuration au lieu que chaque membre la recréé.

Gérez les environnements partagés à partir de la page **Environnements cloud** dans les [paramètres d'administration](https://claude.ai/admin-settings). À partir de là, vous pouvez :

* Créer, modifier et archiver les environnements partagés. Chacun a les mêmes champs qu'un environnement personnel : un nom, un [niveau d'accès réseau](#access-levels), des [variables d'environnement](#configure-your-environment) au format `.env` et un [script de configuration](#setup-scripts).
* Définir l'environnement par défaut pour l'organisation.

Les valeurs dans un environnement partagé atteignent les sessions de chaque membre dans cet environnement. Comme les environnements personnels, les environnements partagés n'ont pas de magasin de secrets dédié, donc n'incluez pas de secrets.

<h2 id="setup-scripts">
  Scripts de configuration
</h2>

Un script de configuration est un script Bash qui s'exécute au démarrage d'une nouvelle session cloud, avant le lancement de Claude Code. Utilisez les scripts de configuration pour installer les dépendances, configurer les outils ou récupérer tout ce dont la session a besoin et qui n'est pas pré-installé.

Les scripts s'exécutent en tant que root sur Ubuntu 24.04, donc `apt install` et la plupart des gestionnaires de paquets de langage fonctionnent.

Pour ajouter un script de configuration, ouvrez la boîte de dialogue des paramètres d'environnement et entrez votre script dans le champ **Script de configuration**.

Cet exemple installe le CLI `gh`, qui n'est pas pré-installé :

```bash theme={null}
#!/bin/bash
apt update && apt install -y gh
```

Si le script se termine avec un code non nul, la session ne démarre pas. Ajoutez `|| true` aux commandes non critiques pour éviter de bloquer la session sur une défaillance d'installation intermittente.

Gardez le temps d'exécution total du script en dessous d'environ cinq minutes afin que le [cache d'environnement](#environment-caching) puisse être construit. Exécutez les installations indépendantes en parallèle avec `&` et `wait`. Si un seul téléchargement ne rentre pas dans la limite de cinq minutes, déplacez-le vers un [hook SessionStart](#setup-scripts-vs-sessionstart-hooks) qui le lance en arrière-plan.

<Note>
  Les scripts de configuration qui installent des paquets ont besoin d'un accès réseau pour atteindre les registres. L'accès réseau **Trusted** par défaut permet les connexions aux [domaines de paquets courants](#default-allowed-domains) y compris npm, PyPI, RubyGems et crates.io. Les scripts échoueront à installer les paquets si votre environnement utilise l'accès réseau **None**.
</Note>

<h3 id="environment-caching">
  Mise en cache de l'environnement
</h3>

Le script de configuration s'exécute la première fois que vous démarrez une session dans un environnement. Après son achèvement, Anthropic crée un snapshot du système de fichiers et réutilise ce snapshot comme point de départ pour les sessions ultérieures. Les nouvelles sessions commencent avec vos dépendances, outils et images Docker déjà sur le disque, et l'étape du script de configuration est ignorée. Cela maintient le démarrage rapide même lorsque le script installe de grandes chaînes d'outils ou extrait des images de conteneur.

Le cache capture les fichiers, pas les processus en cours d'exécution. Tout ce que le script de configuration écrit sur le disque est transféré. Les services ou conteneurs qu'il démarre ne le sont pas, donc démarrez-les par session en demandant à Claude ou avec un [hook SessionStart](#setup-scripts-vs-sessionstart-hooks).

Le script de configuration s'exécute à nouveau pour reconstruire le cache lorsque vous modifiez le script de configuration de l'environnement ou les hôtes réseau autorisés, et lorsque le cache atteint son expiration après environ sept jours. La reprise d'une session existante ne réexécute jamais le script de configuration.

Vous n'avez pas besoin d'activer la mise en cache ou de gérer les snapshots vous-même.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Scripts de configuration vs. hooks SessionStart
</h3>

Utilisez un script de configuration pour installer les choses dont le cloud a besoin mais que votre ordinateur portable a déjà, comme un runtime de langage ou un outil CLI. Utilisez un hook [SessionStart](/docs/fr/hooks#sessionstart) pour la configuration du projet qui devrait s'exécuter partout, cloud et local, comme `npm install`.

Les deux s'exécutent au démarrage d'une session, mais ils appartiennent à des endroits différents :

|                | Scripts de configuration                                                                                        | Hooks SessionStart                                                                    |
| -------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Attaché à      | L'environnement cloud                                                                                           | Votre référentiel                                                                     |
| Configuré dans | Interface utilisateur de l'environnement cloud                                                                  | `.claude/settings.json` dans votre référentiel                                        |
| S'exécute      | Avant le lancement de Claude Code, lorsqu'aucun [environnement en cache](#environment-caching) n'est disponible | Après le lancement de Claude Code, sur chaque session y compris les sessions reprises |
| Portée         | Environnements cloud uniquement                                                                                 | Local et cloud                                                                        |

Les hooks SessionStart peuvent également être définis dans votre `~/.claude/settings.json` au niveau de l'utilisateur localement, mais les paramètres au niveau de l'utilisateur ne sont pas transférés aux sessions cloud. Dans le cloud, les hooks proviennent du référentiel et des [paramètres gérés par le serveur](/docs/fr/server-managed-settings) de votre organisation.

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Installer les dépendances avec un hook SessionStart
</h3>

Pour installer les dépendances uniquement dans les sessions cloud, ajoutez un hook SessionStart au `.claude/settings.json` de votre référentiel :

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Créez le script à `scripts/install_pkgs.sh` et rendez-le exécutable avec `chmod +x`. La variable d'environnement `CLAUDE_CODE_REMOTE` est définie sur `true` dans les sessions cloud, vous pouvez donc l'utiliser pour ignorer l'exécution locale :

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

Les hooks SessionStart ont certaines limitations dans les sessions cloud :

* **Pas de portée cloud uniquement** : les hooks s'exécutent dans les sessions locales et cloud. Pour ignorer l'exécution locale, vérifiez la variable d'environnement `CLAUDE_CODE_REMOTE` comme indiqué ci-dessus.
* **Nécessite un accès réseau** : les commandes d'installation ont besoin d'atteindre les registres de paquets. Si votre environnement utilise l'accès réseau **None**, ces hooks échouent. La [liste d'autorisation par défaut](#default-allowed-domains) sous **Trusted** couvre npm, PyPI, RubyGems et crates.io.
* **Compatibilité du proxy** : tout le trafic sortant passe par un [proxy de sécurité](#security-proxy). Certains gestionnaires de paquets ne fonctionnent pas correctement avec ce proxy. Bun est un exemple connu.
* **Ajoute une latence de démarrage** : les hooks s'exécutent chaque fois qu'une session démarre ou reprend, contrairement aux scripts de configuration qui bénéficient de la [mise en cache de l'environnement](#environment-caching). Gardez les scripts d'installation rapides en vérifiant si les dépendances sont déjà présentes avant de les réinstaller.

Pour persister les variables d'environnement pour les commandes Bash suivantes, écrivez dans le fichier à `$CLAUDE_ENV_FILE`. Consultez [Hooks SessionStart](/docs/fr/hooks#sessionstart) pour plus de détails.

Remplacer l'image de base par votre propre image Docker n'est pas encore pris en charge. Utilisez un script de configuration pour installer ce dont vous avez besoin en haut de l'[image fournie](#installed-tools), ou exécutez votre image en tant que conteneur aux côtés de Claude avec `docker compose`.

<h2 id="network-access">
  Accès réseau
</h2>

L'accès réseau contrôle les connexions sortantes de l'environnement cloud. Chaque environnement spécifie un niveau d'accès, et vous pouvez l'étendre avec des domaines autorisés personnalisés. La valeur par défaut est **Trusted**, qui permet les registres de paquets et autres [domaines autorisés](#default-allowed-domains).

Pour modifier l'accès réseau d'un environnement, [ouvrez-le pour le modifier](#configure-your-environment) et utilisez le sélecteur **Accès réseau** dans la boîte de dialogue. Il n'y a pas de page Environnements séparée. L'icône cloud apparaît partout où vous démarrez une session cloud ou configurez une [routine](/docs/fr/routines#environments-and-network-access).

<Note>
  Le trafic du connecteur MCP est acheminé via les serveurs d'Anthropic, donc les connecteurs que vous activez sur une session ou une routine fonctionnent sans ajouter leurs hôtes aux **Domaines autorisés**. Les connecteurs sont configurés par session ou par routine ; supprimez ceux que vous n'avez pas besoin pour limiter les outils que Claude peut atteindre. Cela repose sur le même canal lié à Anthropic noté sous [Sécurité et isolation](#security-and-isolation).
</Note>

<h3 id="access-levels">
  Niveaux d'accès
</h3>

Choisissez un niveau d'accès lorsque vous créez ou modifiez un environnement :

| Niveau      | Connexions sortantes                                                                                |
| :---------- | :-------------------------------------------------------------------------------------------------- |
| **None**    | Aucun accès réseau sortant                                                                          |
| **Trusted** | [Domaines autorisés](#default-allowed-domains) uniquement : registres de paquets, GitHub, SDK cloud |
| **Full**    | N'importe quel domaine                                                                              |
| **Custom**  | Votre propre liste d'autorisation, incluant optionnellement les valeurs par défaut                  |

Les opérations GitHub utilisent un [proxy séparé](#github-proxy) qui est indépendant de ce paramètre.

<h3 id="allow-specific-domains">
  Autoriser des domaines spécifiques
</h3>

Pour autoriser les domaines qui ne figurent pas dans la liste Trusted, sélectionnez **Custom** dans les paramètres d'accès réseau de l'environnement. Un champ **Domaines autorisés** apparaît. Entrez un domaine par ligne :

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Utilisez `*.` pour la correspondance de sous-domaine générique. Cochez **Inclure également la liste par défaut des gestionnaires de paquets courants** pour conserver les [domaines Trusted](#default-allowed-domains) aux côtés de vos entrées personnalisées, ou laissez-le décoché pour autoriser uniquement ce que vous listez.

Les domaines autorisés sont configurés par environnement. Il n'y a pas de liste d'autorisation au niveau de l'organisation que les propriétaires peuvent appliquer aux environnements de tous les utilisateurs ; les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) peuvent restreindre les sessions cloud mais ne peuvent pas ajouter de domaines autorisés.

<h3 id="github-proxy">
  Proxy GitHub
</h3>

Pour la sécurité, toutes les opérations GitHub passent par un service proxy dédié qui garde vos véritables identifiants GitHub en dehors du sandbox. Le proxy authentifie deux types de trafic :

* Interactions Git : le client git à l'intérieur du sandbox utilise une identité personnalisée limitée, que le proxy vérifie et traduit en votre jeton d'authentification GitHub réel
* Demandes d'API GitHub : le proxy remplace vos véritables identifiants sur les demandes des outils GitHub intégrés, et de `gh` lorsque votre session définit l'espace réservé `proxy-injected` décrit dans [Travailler avec les problèmes et les demandes de tirage GitHub](#work-with-github-issues-and-pull-requests)

Le proxy restreint également les opérations de poussée git à la branche de travail actuelle pour la sécurité, et permet le clonage, la récupération et les opérations PR tout en maintenant les limites de sécurité.

Le proxy limite les demandes d'API GitHub et de ressources de version aux référentiels attachés à la session, indépendamment du [niveau d'accès réseau](#access-levels) de l'environnement. Les scripts de configuration qui téléchargent des ressources de version à partir de référentiels non attachés retournent un 403. Les fichiers validés à partir de référentiels publics sont récupérés via `raw.githubusercontent.com`, que le [proxy de sécurité](#security-proxy) gère à la place. Ce domaine figure dans la [liste Trusted](#default-allowed-domains) par défaut, donc les fichiers restent accessibles sauf si le [niveau d'accès](#access-levels) de l'environnement l'exclut.

<h3 id="security-proxy">
  Proxy de sécurité
</h3>

Les environnements s'exécutent derrière un proxy réseau HTTP/HTTPS pour la sécurité et la prévention des abus. Tout le trafic Internet sortant passe par ce proxy, qui fournit :

* Protection contre les demandes malveillantes
* Limitation de débit et prévention des abus
* Filtrage de contenu pour une sécurité renforcée
* Un journal d'audit au niveau DNS des noms d'hôtes demandés

<h3 id="default-allowed-domains">
  Domaines autorisés par défaut
</h3>

Lors de l'utilisation de l'accès réseau **Trusted**, les domaines suivants sont autorisés par défaut. Les domaines marqués avec `*` indiquent une correspondance de sous-domaine générique, donc `*.gcr.io` autorise n'importe quel sous-domaine de `gcr.io`.

<AccordionGroup>
  <Accordion title="Services Anthropic">
    * api.anthropic.com
    * statsig.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Contrôle de version">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Registres de conteneurs">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Plateformes cloud">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
  </Accordion>

  <Accordion title="Gestionnaires de paquets JavaScript et Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
  </Accordion>

  <Accordion title="Gestionnaires de paquets Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Gestionnaires de paquets Ruby">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Gestionnaires de paquets Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Gestionnaires de paquets Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Gestionnaires de paquets JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Autres gestionnaires de paquets">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Distributions Linux">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Outils de développement et plateformes">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Services cloud et surveillance">
    * statsig.com
    * [www.statsig.com](http://www.statsig.com)
    * api.statsig.com
    * sentry.io
    * \*.sentry.io
    * downloads.sentry-cdn.com
    * http-intake.logs.datadoghq.com
    * browser-intake-us5-datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Livraison de contenu et miroirs">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Schéma et configuration">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="move-tasks-between-web-and-terminal">
  Déplacer les tâches entre le web et le terminal
</h2>

Ces flux de travail nécessitent le [CLI Claude Code](/docs/fr/quickstart) connecté au même compte claude.ai. Vous pouvez démarrer de nouvelles sessions cloud à partir de votre terminal, ou extraire les sessions cloud dans votre terminal pour continuer localement. Les sessions cloud persistent même si vous fermez votre ordinateur portable, et vous pouvez les surveiller de n'importe où, y compris depuis l'application mobile Claude.

<Note>
  À partir du CLI, le transfert de session est unidirectionnel : vous pouvez extraire les sessions cloud dans votre terminal avec `--teleport`, mais vous ne pouvez pas pousser une session de terminal existante vers le web. L'indicateur `--cloud` crée une nouvelle session cloud pour votre référentiel actuel. L'[application Desktop](/docs/fr/desktop#continue-in-another-surface) fournit un menu Continue in qui peut envoyer une session locale vers le web.
</Note>

<h3 id="from-terminal-to-web">
  Du terminal au web
</h3>

Démarrez une session cloud à partir de la ligne de commande avec l'indicateur `--cloud` :

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Cela crée une nouvelle session cloud sur claude.ai. La session clone votre répertoire courant du serveur distant GitHub à votre branche actuelle, donc poussez d'abord si vous avez des commits locaux, puisque la VM clone depuis GitHub plutôt que depuis votre machine. `--cloud` fonctionne avec un seul référentiel à la fois. La tâche s'exécute dans le cloud tandis que vous continuez à travailler localement. L'ancienne orthographe `--remote` fonctionne toujours comme alias déprécié pour `--cloud`.

À partir de la v2.1.195, le CLI affiche une liste de contrôle en direct des étapes de configuration, telles que le clonage du référentiel et l'exécution de votre [script de configuration](#setup-scripts), tandis que le conteneur cloud démarre. Les messages que vous tapez pendant que le conteneur est en cours de provisionnement sont mis en file d'attente et envoyés une fois que la session est prête.

<Note>
  `--cloud` crée des sessions cloud. `--remote-control` n'est pas lié : il expose une session CLI locale pour la surveillance depuis le web. Consultez [Contrôle à distance](/docs/fr/remote-control).
</Note>

Utilisez `/tasks` dans le CLI Claude Code pour vérifier la progression, ou ouvrez la session sur claude.ai ou l'application mobile Claude pour interagir directement. De là, vous pouvez diriger Claude, fournir des commentaires ou répondre à des questions comme dans n'importe quelle autre conversation.

<h4 id="tips-for-cloud-tasks">
  Conseils pour les tâches cloud
</h4>

**Planifiez localement, exécutez à distance** : pour les tâches complexes, démarrez Claude en mode plan pour collaborer sur l'approche, puis envoyez le travail vers le cloud :

```bash theme={null}
claude --permission-mode plan
```

En mode plan, Claude lit les fichiers, exécute les commandes pour explorer et propose un plan sans modifier le code source. Une fois que vous êtes satisfait, enregistrez le plan dans le référentiel, validez et poussez afin que la VM cloud puisse le cloner. Ensuite, démarrez une session cloud pour l'exécution autonome :

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

Ce modèle vous donne le contrôle sur la stratégie tout en permettant à Claude d'exécuter de manière autonome dans le cloud.

**Planifiez dans le cloud avec ultraplan** : pour rédiger et examiner le plan lui-même dans une session web, utilisez [ultraplan](/docs/fr/ultraplan). Claude génère le plan sur Claude Code sur le web tandis que vous continuez à travailler, puis vous commentez les sections dans votre navigateur et choisissez d'exécuter à distance ou d'envoyer le plan vers votre terminal.

**Exécutez les tâches en parallèle** : chaque commande `--cloud` crée sa propre session cloud qui s'exécute indépendamment. Vous pouvez lancer plusieurs tâches et elles s'exécuteront toutes simultanément dans des sessions séparées :

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Surveillez toutes les sessions avec `/tasks` dans le CLI Claude Code. Lorsqu'une session se termine, vous pouvez créer une PR à partir de l'interface web ou [téléporter](#from-web-to-terminal) la session vers votre terminal pour continuer à travailler.

<h4 id="send-local-repositories-without-github">
  Envoyer les référentiels locaux sans GitHub
</h4>

Lorsque vous exécutez `claude --cloud` à partir d'un référentiel qui n'est pas connecté à GitHub, Claude Code regroupe votre référentiel local et le télécharge directement vers la session cloud. Le paquet inclut votre historique de référentiel complet sur toutes les branches, plus toute modification non validée des fichiers suivis.

Ce repli s'active automatiquement lorsque l'accès à GitHub n'est pas disponible. Pour le forcer même lorsque GitHub est connecté, définissez `CCR_FORCE_BUNDLE=1` :

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Les référentiels regroupés doivent respecter ces limites :

* Le répertoire doit être un référentiel git avec au moins un commit
* Le référentiel regroupé doit être inférieur à 100 Mo. Les référentiels plus grands reviennent à regrouper uniquement la branche actuelle, puis à un snapshot unique aplati de l'arborescence de travail, et échouent uniquement si le snapshot est toujours trop volumineux
* Les fichiers non suivis ne sont pas inclus ; exécutez `git add` sur les fichiers que vous souhaitez que la session cloud voie
* Les sessions créées à partir d'un paquet ne peuvent pas pousser vers un serveur distant à moins que vous ayez également [authentification GitHub](#github-authentication-options) configurée

<h3 id="from-web-to-terminal">
  Du web au terminal
</h3>

Extrayez une session cloud dans votre terminal en utilisant l'une de ces options :

* **Utilisation de `--teleport`** : à partir de la ligne de commande, exécutez `claude --teleport` pour un sélecteur de session interactif, ou `claude --teleport <session-id>` pour reprendre une session spécifique directement. Si vous avez des modifications non validées, vous serez invité à les ranger d'abord.
* **Utilisation de `/teleport`** : à l'intérieur d'une session CLI existante, exécutez `/teleport` ou `/tp` pour ouvrir le même sélecteur de session sans redémarrer Claude Code.
* **À partir de `/tasks`** : exécutez `/tasks` pour voir vos sessions en arrière-plan, puis appuyez sur `t` pour vous téléporter dans l'une d'elles.
* **À partir de l'interface web** : sélectionnez **Ouvrir dans CLI** pour copier une commande que vous pouvez coller dans votre terminal.

Lorsque vous téléportez une session, Claude vérifie que vous êtes dans le bon référentiel, récupère et extrait la branche de la session cloud, et charge l'historique complet de la conversation dans votre terminal.

`--teleport` est distinct de `--resume`. `--resume` rouvre une conversation à partir de l'historique local de cette machine et ne liste pas les sessions cloud ; `--teleport` extrait une session cloud et sa branche.

<h4 id="teleport-requirements">
  Exigences de téléportation
</h4>

La téléportation vérifie ces exigences avant de reprendre une session. Si une exigence n'est pas satisfaite, vous verrez une erreur ou vous serez invité à résoudre le problème.

| Exigence            | Détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| État git propre     | Votre répertoire de travail ne doit avoir aucune modification non validée. La téléportation vous invite à ranger les modifications si nécessaire.                                                                                                                                                                                                                                                                                                                                                                         |
| Référentiel correct | Vous devez exécuter `--teleport` à partir d'une extraction du même référentiel, pas d'une fourche. À partir de la v2.1.199, Claude Code accepte une extraction même lorsqu'il ne peut pas analyser le serveur distant en nom d'hôte, comme un alias d'hôte SSH comme `git@work:owner/repo.git` ou une forme courte réécrite par `insteadOf`. Il affiche d'abord une invite de confirmation, et uniquement lorsque le propriétaire du serveur distant et le nom du référentiel correspondent au référentiel de la session. |
| Branche disponible  | La branche de la session cloud doit avoir été poussée vers le serveur distant. La téléportation la récupère et l'extrait automatiquement.                                                                                                                                                                                                                                                                                                                                                                                 |
| Même compte         | Vous devez être authentifié au même compte claude.ai utilisé dans la session cloud.                                                                                                                                                                                                                                                                                                                                                                                                                                       |

<h4 id="teleport-is-unavailable">
  `--teleport` n'est pas disponible
</h4>

La téléportation nécessite l'authentification par abonnement claude.ai. Si vous êtes authentifié via clé API, Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, exécutez `/login` pour vous connecter avec votre compte claude.ai à la place. Si vous êtes déjà connecté via claude.ai et `--teleport` n'est toujours pas disponible, votre organisation a peut-être désactivé les sessions cloud.

<h2 id="work-with-sessions">
  Travailler avec les sessions
</h2>

Les sessions apparaissent dans la barre latérale à claude.ai/code. De là, vous pouvez examiner les modifications, partager avec les coéquipiers, archiver le travail terminé ou supprimer les sessions définitivement.

<h3 id="manage-context">
  Gérer le contexte
</h3>

Les sessions cloud prennent en charge les [commandes intégrées](/docs/fr/commands) qui produisent une sortie textuelle. Les commandes qui s'exécutent uniquement dans l'interface du terminal, telles que `/plugin` ou `/resume`, ne sont pas disponibles. Les commandes qui ouvrent un sélecteur ou un panneau dans le terminal se comportent différemment dans les sessions cloud :

* **`/model`, `/effort`, `/fast`, `/color` et `/rename`** : passez la valeur comme argument, par exemple `/model sonnet`, au lieu d'ouvrir le sélecteur de terminal ou le curseur. Les formes d'argument nécessitent Claude Code v2.1.205 ou ultérieur dans l'environnement de la session et suivent les [notes de disponibilité](/docs/fr/commands#all-commands) de chaque commande : `/effort` signale `Not applied` tandis qu'un [lancement par défaut du niveau d'effort](/docs/fr/model-config#adjust-effort-level) du modèle est en vigueur, et `/fast` fonctionne uniquement dans une session qui a démarré avec le mode rapide activé.
* **`/config`** : sur le web, ouvre la section Claude Code de vos paramètres au lieu de définir une valeur, et le texte après la commande, y compris `key=value`, est ignoré. Pour modifier les paramètres d'une session cloud, utilisez les [variables d'environnement](#configure-your-environment) ou validez les [fichiers de paramètres](/docs/fr/settings) dans le référentiel.

Pour la gestion du contexte spécifiquement :

| Commande   | Fonctionne dans les sessions cloud | Notes                                                                                                                                 |
| :--------- | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `/compact` | Oui                                | Résume la conversation pour libérer du contexte. Accepte les instructions de focus optionnelles comme `/compact keep the test output` |
| `/context` | Oui                                | Affiche ce qui est actuellement dans la fenêtre de contexte                                                                           |
| `/clear`   | Non                                | Démarrez une nouvelle session à partir de la barre latérale à la place                                                                |

La compaction automatique s'exécute automatiquement lorsque la fenêtre de contexte approche de la capacité. Pour la déclencher plus tôt, définissez [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/fr/env-vars) dans vos [variables d'environnement](#configure-your-environment). Par exemple, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` compacte à 70 % de capacité au lieu d'attendre que la fenêtre soit presque pleine. Pour modifier la taille de fenêtre effective pour les calculs de compaction, utilisez [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/fr/env-vars).

Les [sous-agents](/docs/fr/sub-agents) fonctionnent de la même manière qu'en local. Claude peut les générer avec l'outil Task pour décharger la recherche ou le travail parallèle dans une fenêtre de contexte séparée, gardant la conversation principale plus légère. Les sous-agents définis dans votre `.claude/agents/` du référentiel sont récupérés automatiquement.

Les [équipes d'agents](/docs/fr/agent-teams) sont désactivées par défaut mais peuvent être activées en ajoutant `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` à vos [variables d'environnement](#configure-your-environment).

<h3 id="review-changes">
  Examiner les modifications
</h3>

Chaque session affiche un indicateur de diff avec les lignes ajoutées et supprimées, comme `+42 -18`. Sélectionnez-le pour ouvrir la vue diff, laissez des commentaires en ligne sur des lignes spécifiques et envoyez-les à Claude avec votre message suivant. Consultez [Examiner et itérer](/docs/fr/web-quickstart#review-and-iterate) pour la procédure pas à pas complète incluant la création de PR. Pour que Claude surveille la PR pour les défaillances CI et les commentaires d'examen automatiquement, consultez [Correction automatique des demandes de tirage](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Partager les sessions
</h3>

Pour partager une session, basculez sa visibilité selon les types de compte ci-dessous. Après cela, partagez le lien de session tel quel. Les destinataires voient l'état le plus récent lorsqu'ils ouvrent le lien, mais leur vue ne se met pas à jour en temps réel.

<h4 id="share-from-an-enterprise-or-team-account">
  Partage à partir d'un compte Enterprise ou Team
</h4>

Pour les comptes Enterprise et Team, les deux options de visibilité sont **Private** et **Team**. La visibilité Team rend la session visible aux autres membres de votre organisation claude.ai. Les sessions [Claude dans Slack](/docs/fr/slack) sont automatiquement partagées avec la visibilité Team.

La vérification de l'accès au référentiel est activée par défaut, en fonction du compte GitHub connecté au compte du destinataire. Le nom d'affichage de votre compte est visible à tous les destinataires ayant accès.

<h4 id="share-from-a-max-or-pro-account">
  Partage à partir d'un compte Max ou Pro
</h4>

Pour les comptes Max et Pro, les deux options de visibilité sont **Private** et **Public**. La visibilité Public rend la session visible à tout utilisateur connecté à claude.ai.

Vérifiez votre session pour le contenu sensible avant de la partager. Les sessions peuvent contenir du code et des identifiants provenant de référentiels GitHub privés. La vérification de l'accès au référentiel n'est pas activée par défaut.

Pour exiger que les destinataires aient accès au référentiel, ou pour masquer votre nom des sessions partagées, allez à Paramètres > Claude Code > Paramètres de partage.

<h3 id="archive-sessions">
  Archiver les sessions
</h3>

Vous pouvez archiver les sessions pour garder votre liste de sessions organisée. Les sessions archivées sont masquées de la liste de sessions par défaut mais peuvent être affichées en filtrant les sessions archivées.

Pour archiver une session, survolez la session dans la barre latérale et sélectionnez l'icône d'archivage.

<h3 id="delete-sessions">
  Supprimer les sessions
</h3>

La suppression d'une session supprime définitivement la session et ses données. Cette action ne peut pas être annulée. Vous pouvez supprimer une session de deux façons :

* **À partir de la barre latérale** : filtrez les sessions archivées, puis survolez la session que vous souhaitez supprimer et sélectionnez l'icône de suppression
* **À partir du menu de session** : ouvrez une session, sélectionnez la liste déroulante à côté du titre de la session et sélectionnez **Supprimer**

Vous serez invité à confirmer avant la suppression d'une session.

<h2 id="auto-fix-pull-requests">
  Correction automatique des demandes de tirage
</h2>

Claude peut surveiller une demande de tirage et répondre automatiquement aux défaillances CI et aux commentaires d'examen. Claude s'abonne aux événements GitHub sur la PR, et lorsqu'une vérification échoue ou qu'un examinateur laisse un commentaire, Claude enquête et pousse une correction si elle est claire.

<Note>
  Auto-fix nécessite que l'application Claude GitHub soit installée sur votre référentiel. Si vous ne l'avez pas déjà fait, installez-la à partir de la [page de l'application GitHub](https://github.com/apps/claude) ou lorsque vous y êtes invité lors de la [configuration](/docs/fr/web-quickstart#connect-github-and-create-an-environment).
</Note>

Il existe plusieurs façons d'activer auto-fix selon d'où provient la PR et quel appareil vous utilisez :

* **PR créées dans Claude Code sur le web** : ouvrez la barre d'état CI et sélectionnez **Auto-fix**
* **À partir de votre terminal** : exécutez [`/autofix-pr`](/docs/fr/commands) sur la branche de la PR. Claude Code détecte la PR ouverte avec `gh`, génère une session web et active auto-fix en une seule étape
* **À partir de l'application mobile** : dites à Claude de corriger automatiquement la PR, par exemple « regardez cette PR et corrigez les défaillances CI ou les commentaires d'examen »
* **N'importe quelle PR existante** : collez l'URL de la PR dans une session et dites à Claude de la corriger automatiquement

Auto-fix est un bouton bascule par PR. Pour arrêter la surveillance, ouvrez la barre d'état CI dans la session web et désactivez le bouton bascule **Auto-fix**, ou dites à Claude d'arrêter de surveiller la PR.

<h3 id="how-claude-responds-to-pr-activity">
  Comment Claude répond à l'activité PR
</h3>

Lorsque auto-fix est actif, Claude reçoit les événements GitHub pour la PR, y compris les nouveaux commentaires d'examen et les défaillances de vérification CI. Pour chaque événement, Claude enquête et décide comment procéder :

* **Corrections claires** : si Claude est confiant dans une correction et qu'elle n'entre pas en conflit avec les instructions antérieures, Claude apporte la modification, la pousse et explique ce qui a été fait dans la session
* **Demandes ambiguës** : si le commentaire d'un examinateur peut être interprété de plusieurs façons ou implique quelque chose d'architecturalement significatif, Claude vous demande avant d'agir
* **Événements en double ou sans action** : si un événement est un doublon ou ne nécessite aucune modification, Claude le note dans la session et continue

GitHub n'émet pas de webhook lorsque la branche de base avance et crée un conflit de fusion, donc auto-fix ne peut pas réagir aux conflits de son propre chef. Pour résoudre un conflit, ouvrez la session et demandez à Claude de rebaser.

Claude peut répondre aux fils de commentaires d'examen sur GitHub dans le cadre de leur résolution. Ces réponses sont publiées en utilisant votre compte GitHub, elles apparaissent donc sous votre nom d'utilisateur, mais chaque réponse est étiquetée comme provenant de Claude Code pour que les examinateurs sachent qu'elle a été écrite par l'agent et non par vous directement.

<Warning>
  Si votre référentiel utilise une automatisation déclenchée par commentaire comme Atlantis, Terraform Cloud ou des GitHub Actions personnalisées qui s'exécutent sur les événements `issue_comment`, sachez que Claude peut répondre en votre nom, ce qui peut déclencher ces flux de travail. Examinez l'automatisation de votre référentiel avant d'activer auto-fix et envisagez de désactiver auto-fix pour les référentiels où un commentaire PR peut déployer une infrastructure ou exécuter des opérations privilégiées.
</Warning>

<h2 id="security-and-isolation">
  Sécurité et isolation
</h2>

Chaque session cloud est séparée de votre machine et des autres sessions par plusieurs couches :

* **Machines virtuelles isolées** : chaque session s'exécute dans une VM isolée gérée par Anthropic
* **Contrôles d'accès réseau** : l'accès réseau est limité par défaut et peut être désactivé. Lors de l'exécution avec l'accès réseau désactivé, Claude Code peut toujours communiquer avec l'API Anthropic, ce qui peut permettre aux données de quitter la VM.
* **Protection des identifiants** : les identifiants sensibles tels que les identifiants git ou les clés de signature ne sont jamais à l'intérieur du sandbox avec Claude Code. L'authentification est gérée via un proxy sécurisé utilisant des identifiants limités.
* **Analyse sécurisée** : le code est analysé et modifié dans des VM isolées avant la création de PR

<h2 id="troubleshooting">
  Dépannage
</h2>

Pour les erreurs d'API d'exécution qui apparaissent dans la conversation comme `API Error: 500`, `529 Overloaded`, `429` ou `Prompt is too long`, consultez la [référence des erreurs](/docs/fr/errors). Ces erreurs et leurs corrections sont partagées avec le CLI et l'application Desktop. Les sections ci-dessous couvrent les problèmes spécifiques aux sessions cloud.

<h3 id="session-creation-failed">
  Échec de la création de session
</h3>

Si une nouvelle session ne démarre pas avec `Session creation failed` ou stagne à la mise en service, Claude Code n'a pas pu allouer un environnement cloud.

* Vérifiez [status.claude.com](https://status.claude.com) pour les incidents de session cloud
* Réessayez après une minute, car la capacité est mise en service à la demande
* Confirmez que votre référentiel est accessible. Le compte GitHub qui se connecte doit avoir accès au référentiel sur GitHub, soit par l'autorisation de l'application Claude GitHub, soit par un jeton `gh` synchronisé via `/web-setup`. L'installation de l'application sur le référentiel n'est pas requise. Consultez [Options d'authentification GitHub](#github-authentication-options).

<h3 id="remote-control-session-expired-or-access-denied">
  Session Remote Control expirée ou accès refusé
</h3>

`--teleport` se connecte via la même infrastructure de session Remote Control que les sessions cloud, donc les erreurs d'authentification et d'expiration de session apparaissent avec la terminologie Remote Control. Vous pouvez voir `Remote Control session expired` ou `Access denied`. Le jeton de connexion est de courte durée et limité à votre compte.

* Exécutez `/login` localement pour actualiser vos identifiants, puis reconnectez-vous
* Confirmez que vous êtes connecté au même compte qui possède la session
* Si vous voyez `Remote Control may not be available for this organization`, un propriétaire n'a pas activé les sessions cloud pour votre organisation

<h3 id="environment-expired">
  Environnement expiré
</h3>

Les sessions cloud s'arrêtent après une période d'inactivité et l'environnement sous-jacent est réclamé. À partir d'un terminal local, cela apparaît comme `Could not resume session ... its environment has expired. Creating a fresh session instead.` Sur le web, la session est marquée comme expirée dans la liste des sessions.

Rouvrez la session à partir de [claude.ai/code](https://claude.ai/code) pour mettre en service un environnement frais avec votre historique de conversation restauré.

<h2 id="limitations">
  Limitations
</h2>

Avant de compter sur les sessions cloud pour un flux de travail, tenez compte de ces contraintes :

* **Limites de débit** : Claude Code sur le web partage les limites de débit avec tous les autres usages de Claude et Claude Code au sein de votre compte. L'exécution de plusieurs tâches en parallèle consomme proportionnellement plus de limites de débit. Il n'y a pas de frais de calcul séparé pour la VM cloud.
* **Authentification du référentiel** : vous ne pouvez déplacer les sessions du web vers le local que lorsque vous êtes authentifié au même compte
* **Restrictions de plateforme** : le clonage du référentiel et la création de demandes de tirage nécessitent GitHub. Les instances [GitHub Enterprise Server](/docs/fr/github-enterprise-server) auto-hébergées sont prises en charge pour les plans Team et Enterprise. GitLab, Bitbucket et les autres référentiels non-GitHub peuvent être envoyés aux sessions cloud en tant que [paquet local](#send-local-repositories-without-github), mais la session ne peut pas pousser les résultats vers le serveur distant
* **Liste d'autorisation IP de l'organisation** : les sessions cloud appellent l'API Anthropic à partir de l'infrastructure gérée par Anthropic, pas de votre réseau. Si votre organisation a [l'autorisation IP](https://support.claude.com/fr/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) activée, chaque session cloud échoue avec une erreur d'authentification. Il en va de même pour [Code Review](/docs/fr/code-review) et [Routines](/docs/fr/routines). Contactez [le support Anthropic](https://support.claude.com/) pour exempter les services hébergés par Anthropic de la liste d'autorisation IP de votre organisation.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Ultraplan](/docs/fr/ultraplan) : rédigez un plan dans une session cloud et examinez-le dans votre navigateur
* [Ultrareview](/docs/fr/ultrareview) : exécutez un examen de code multi-agent approfondi dans un sandbox cloud
* [Routines](/docs/fr/routines) : automatisez le travail selon un calendrier, via un appel API ou en réponse aux événements GitHub
* [Configuration des hooks](/docs/fr/hooks) : exécutez les scripts aux événements du cycle de vie de la session
* [Référence des paramètres](/docs/fr/settings) : toutes les options de configuration
* [Sécurité](/docs/fr/security) : garanties d'isolation et gestion des données
* [Utilisation des données](/docs/fr/data-usage) : ce qu'Anthropic conserve des sessions cloud
* [Claude Tag](https://claude.com/docs/claude-tag/overview) : un @Claude géré par l'organisation dans Slack qui s'exécute sur le même environnement cloud
