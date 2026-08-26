> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conteneurs de développement

> Exécutez Claude Code dans un conteneur de développement pour des environnements cohérents et isolés dans toute votre équipe.

Un [conteneur de développement](https://containers.dev/), ou dev container, vous permet de définir un environnement identique et isolé que chaque ingénieur de votre équipe peut exécuter. Avec Claude Code installé dans ce conteneur, les commandes que Claude exécute s'exécutent à l'intérieur plutôt que sur la machine hôte, tandis que les modifications apportées à vos fichiers de projet apparaissent dans votre référentiel local au fur et à mesure que vous travaillez.

Cette page couvre [l'installation de Claude Code dans un dev container](#add-claude-code-to-your-dev-container), puis un ensemble de rubriques de configuration autonomes : persister l'authentification entre les reconstructions, appliquer la politique organisationnelle, restreindre la sortie réseau et exécuter sans invites de permission. Lisez celles qui correspondent à votre configuration.

<Warning>
  Bien que le dev container offre des protections substantielles, aucun système n'est complètement immunisé contre toutes les attaques.
  Lorsqu'il est exécuté avec `--dangerously-skip-permissions`, les dev containers n'empêchent pas un projet malveillant d'exfiltrer quoi que ce soit d'accessible à l'intérieur du conteneur, y compris les identifiants Claude Code stockés dans [`~/.claude`](/docs/fr/claude-directory).
  Utilisez les dev containers uniquement lors du développement avec des référentiels de confiance, et surveillez les activités de Claude.
  Évitez de monter les secrets de l'hôte tels que `~/.ssh` ou les fichiers d'identifiants cloud dans le conteneur ; préférez les jetons limités au référentiel ou à courte durée de vie.
</Warning>

<Accordion title="Comment les dev containers fonctionnent avec votre éditeur">
  <img src="https://mintcdn.com/claude-code/YvJyjZfd9yMihr0i/images/devcontainer-architecture.svg?fit=max&auto=format&n=YvJyjZfd9yMihr0i&q=85&s=9017b1d16a446c6cc37ba562f35b9aae" className="dark:hidden" alt="Diagramme montrant un éditeur sur l'hôte se connectant à un dev container Docker. Claude Code, le terminal et les outils de compilation s'exécutent à l'intérieur du conteneur. Le référentiel hôte est monté en bind dans le conteneur en tant qu'espace de travail." width="640" height="300" data-path="images/devcontainer-architecture.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/devcontainer-architecture-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=a0a340b1f2afc6a590696102c8acaaca" className="hidden dark:block" alt="Diagramme montrant un éditeur sur l'hôte se connectant à un dev container Docker. Claude Code, le terminal et les outils de compilation s'exécutent à l'intérieur du conteneur. Le référentiel hôte est monté en bind dans le conteneur en tant qu'espace de travail." width="640" height="300" data-path="images/devcontainer-architecture-dark.svg" />

  Un dev container s'exécute en tant que conteneur Docker, soit sur votre machine, soit sur un hôte cloud tel que GitHub Codespaces. Un éditeur qui prend en charge la spécification Dev Containers, tel que VS Code, GitHub Codespaces, un IDE JetBrains ou Cursor, se connecte à ce conteneur : vous parcourez et modifiez les fichiers dans l'éditeur comme d'habitude, mais le terminal intégré, les serveurs de langage et les outils de compilation s'exécutent tous à l'intérieur du conteneur plutôt que sur votre hôte. Les éditeurs sans support de dev container, tels que Vim simple, ne font pas partie de ce flux de travail.

  Claude Code s'exécute à l'intérieur du conteneur, il voit donc les mêmes fichiers, dépendances et outils que le reste de la chaîne d'outils de votre projet. Dans VS Code, vous pouvez utiliser soit le [panneau d'extension Claude Code](/docs/fr/vs-code), soit exécuter `claude` dans le terminal intégré ; les deux s'exécutent à l'intérieur du conteneur et partagent la même configuration `~/.claude`.
</Accordion>

<h2 id="add-claude-code-to-your-dev-container">
  Ajouter Claude Code à votre dev container
</h2>

Claude Code s'installe dans n'importe quel dev container via la [Fonctionnalité Claude Code Dev Container](https://github.com/anthropics/devcontainer-features/tree/main/src/claude-code).

Les paramètres fonctionnent avec n'importe quel outil qui prend en charge la spécification Dev Containers, tel que VS Code, GitHub Codespaces ou les IDE JetBrains. Les étapes ci-dessous utilisent VS Code comme exemple.

Lorsque vous ouvrez le conteneur dans VS Code ou Codespaces, la fonctionnalité ajoute également l'extension VS Code Claude Code ; les autres éditeurs ignorent cette partie.

<Tip>
  Nouveau dans les dev containers ? Le [tutoriel VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/tutorial) vous guide à travers l'installation de Docker, l'extension et l'ouverture de votre premier conteneur. Pour un exemple plus complet et renforcé avec un pare-feu et des volumes persistants, voir [Essayer le conteneur de référence](#try-the-reference-container).
</Tip>

<Steps>
  <Step title="Créer ou mettre à jour devcontainer.json">
    Enregistrez ce qui suit en tant que `.devcontainer/devcontainer.json` dans votre référentiel, ou ajoutez le bloc `features` à votre fichier existant.

    La balise de version à la fin, telle que `:1.0`, épingle le script d'installation de la fonctionnalité, pas la version de Claude Code. La fonctionnalité installe la dernière version de Claude Code, et Claude Code se met à jour automatiquement à l'intérieur du conteneur par défaut.

    Pour épingler la version CLI ou désactiver la mise à jour automatique, voir [Appliquer la politique organisationnelle](#enforce-organization-policy).

    ```json .devcontainer/devcontainer.json theme={null}
    {
      "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
      "features": {
        "ghcr.io/anthropics/devcontainer-features/claude-code:1.0": {}
      }
    }
    ```

    Remplacez la ligne `image` par l'image de base de votre projet ou supprimez-la si votre fichier existant utilise un Dockerfile.
  </Step>

  <Step title="Reconstruire le conteneur">
    Ouvrez la Palette de commandes VS Code avec `Cmd+Shift+P` sur Mac ou `Ctrl+Shift+P` sur Windows et Linux, et exécutez **Dev Containers: Rebuild Container**.

    Pour les autres outils, suivez l'action de reconstruction de cet outil : voir [reconstruction dans GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-a-codespace/rebuilding-the-container-in-a-codespace), la [CLI Dev Containers](https://github.com/devcontainers/cli), ou la documentation de dev container de votre IDE.
  </Step>

  <Step title="Se connecter à Claude Code">
    Ouvrez un terminal dans le conteneur reconstruit et exécutez `claude`, puis suivez l'invite d'authentification.
  </Step>
</Steps>

Ce que vous voyez à l'invite d'authentification dépend de votre fournisseur :

* **Anthropic** : connectez-vous via un navigateur avec votre compte Claude ou Anthropic Console
* **[Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](/docs/fr/third-party-integrations)** : Claude Code utilise vos identifiants de fournisseur cloud, sans invite de navigateur

Pour les fournisseurs cloud, transmettez les identifiants dans le conteneur en tant que variables d'environnement via `containerEnv`, un secret Codespaces ou l'identité de charge de travail de votre cloud plutôt que de monter les fichiers d'identifiants depuis l'hôte. Voir [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou [Microsoft Foundry](/docs/fr/microsoft-foundry) pour la chaîne d'identifiants que Claude Code lit.

Voir [Choisir votre fournisseur d'API](/docs/fr/admin-setup#choose-your-api-provider) pour décider quel chemin convient à votre organisation.

<Note>
  Si la connexion au navigateur se termine mais le rappel n'atteint jamais le conteneur, copiez le code affiché dans le navigateur et collez-le à l'invite `Paste code here if prompted` dans le terminal. Cela peut se produire lorsque la redirection de port de l'éditeur n'achemine pas le rappel localhost.
</Note>

<h2 id="persist-authentication-and-settings-across-rebuilds">
  Persister l'authentification et les paramètres entre les reconstructions
</h2>

Par défaut, le répertoire personnel du conteneur est supprimé lors de la reconstruction, les ingénieurs doivent donc se reconnecter à chaque fois. Claude Code stocke son jeton d'authentification, les paramètres utilisateur et l'historique de session sous [`~/.claude`](/docs/fr/claude-directory). Montez un volume nommé à ce chemin pour conserver cet état entre les reconstructions.

L'exemple suivant monte un volume au répertoire personnel de l'utilisateur `node` :

```json devcontainer.json theme={null}
"mounts": [
  "source=claude-code-config,target=/home/node/.claude,type=volume"
]
```

Remplacez `/home/node` par le répertoire personnel de l'utilisateur `remoteUser` de votre conteneur. Si vous montez le volume ailleurs que `~/.claude`, définissez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars) sur le chemin de montage afin que Claude Code y lise et écrive.

Pour isoler l'état par projet plutôt que de partager un volume sur tous les référentiels, incluez la variable `${devcontainerId}` dans le nom de la source. La [configuration de référence](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) utilise `source=claude-code-config-${devcontainerId}` à cette fin.

Dans GitHub Codespaces, `~/.claude` persiste lors de l'arrêt et du redémarrage d'un codespace, mais est toujours effacé lorsque vous reconstruisez le conteneur, donc le montage de volume ci-dessus s'applique également là. Pour conserver l'authentification entre les codespaces, stockez `ANTHROPIC_API_KEY` ou un `CLAUDE_CODE_OAUTH_TOKEN` de [`claude setup-token`](/docs/fr/authentication#generate-a-long-lived-token) en tant que [secret Codespaces](https://docs.github.com/en/codespaces/managing-your-codespaces/managing-your-account-specific-secrets-for-github-codespaces) ; Codespaces rend les secrets disponibles en tant que variables d'environnement à l'intérieur du conteneur automatiquement.

<h2 id="enforce-organization-policy">
  Appliquer la politique organisationnelle
</h2>

Un dev container est un endroit pratique pour appliquer la politique organisationnelle, car la même image et configuration s'exécutent sur la machine de chaque ingénieur.

Claude Code lit `/etc/claude-code/managed-settings.json` sur Linux et l'applique avec la plus haute priorité dans la [hiérarchie des paramètres](/docs/fr/settings#how-scopes-interact), donc les valeurs là-bas remplacent tout ce qu'un ingénieur définit dans `~/.claude` ou le répertoire `.claude/` du projet. Copiez le fichier en place depuis votre Dockerfile :

```dockerfile Dockerfile theme={null}
RUN mkdir -p /etc/claude-code
COPY managed-settings.json /etc/claude-code/managed-settings.json
```

Parce que le Dockerfile vit dans le référentiel, n'importe qui ayant accès en écriture peut modifier ou supprimer cette étape. Pour une politique que les ingénieurs ne peuvent pas contourner en modifiant les fichiers du référentiel, livrez les paramètres gérés via [paramètres gérés par le serveur](/docs/fr/server-managed-settings) ou votre MDM à la place. Voir [fichiers de paramètres gérés](/docs/fr/settings#settings-files) pour les clés disponibles et les autres chemins de livraison.

Pour définir les [variables d'environnement](/docs/fr/env-vars) qui s'appliquent à chaque session Claude Code dans le conteneur, ajoutez-les à `containerEnv` dans votre `devcontainer.json`. L'exemple suivant désactive la télémétrie et les rapports d'erreur et empêche Claude Code de se mettre à jour automatiquement après l'installation :

```json devcontainer.json theme={null}
"containerEnv": {
  "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
  "DISABLE_AUTOUPDATER": "1"
}
```

La Fonctionnalité Dev Container installe toujours la dernière version de Claude Code. Pour épingler une version spécifique de Claude Code pour des builds reproductibles, installez-la depuis votre Dockerfile avec `npm install -g @anthropic-ai/claude-code@X.Y.Z` au lieu d'utiliser la fonctionnalité, et définissez `DISABLE_AUTOUPDATER` comme indiqué ci-dessus.

Pour la liste complète des contrôles de politique incluant les règles de permission, les restrictions d'outils et les listes blanches de serveurs MCP, voir [Configurer Claude Code pour votre organisation](/docs/fr/admin-setup).

Pour rendre les [serveurs MCP](/docs/fr/mcp) disponibles à l'intérieur du conteneur, définissez-les à [portée du projet](/docs/fr/mcp#mcp-installation-scopes) dans un fichier `.mcp.json` à la racine du référentiel afin qu'ils soient vérifiés aux côtés de votre configuration de dev container. Installez tous les binaires sur lesquels les serveurs stdio locaux dépendent dans votre Dockerfile, et ajoutez les domaines de serveur distant à votre liste blanche réseau.

<h2 id="restrict-network-egress">
  Restreindre la sortie réseau
</h2>

Vous pouvez limiter le trafic sortant du conteneur aux seuls domaines dont Claude Code a besoin. Voir [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements) pour les domaines d'inférence et d'authentification, et [Services de télémétrie](/docs/fr/data-usage#telemetry-services) pour les connexions de télémétrie et de rapport d'erreur optionnelles et comment les désactiver.

Le conteneur de référence inclut un script [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh) qui bloque tout le trafic sortant sauf les domaines dont Claude Code et vos outils de développement ont besoin. L'exécution d'un pare-feu à l'intérieur d'un conteneur nécessite des permissions supplémentaires, donc la référence ajoute les capacités `NET_ADMIN` et `NET_RAW` via `runArgs`. Le script de pare-feu et ces capacités ne sont pas requis pour Claude Code lui-même : vous pouvez les laisser de côté et vous fier à vos propres contrôles réseau à la place.

<h2 id="run-without-permission-prompts">
  Exécuter sans invites de permission
</h2>

Parce que le conteneur exécute Claude Code en tant qu'utilisateur non-root et confine l'exécution des commandes au conteneur, vous pouvez passer `--dangerously-skip-permissions` pour un fonctionnement sans surveillance. La CLI rejette cet indicateur lorsqu'elle est lancée en tant que root, donc confirmez que `remoteUser` est défini sur un compte non-root.

Ignorer les invites de permission supprime votre opportunité d'examiner les appels d'outils avant qu'ils ne s'exécutent. Claude peut toujours modifier n'importe quel fichier dans l'espace de travail monté en bind, qui apparaît directement sur votre hôte, et atteindre tout ce que la politique réseau du conteneur permet. Associez cet indicateur aux [restrictions de sortie réseau](#restrict-network-egress) ci-dessus pour limiter ce qu'une session contournée peut atteindre.

Si vous voulez moins d'invites sans désactiver les contrôles de sécurité, envisagez plutôt le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), qui a un classificateur examinant les actions avant qu'elles ne s'exécutent. Pour empêcher les ingénieurs d'utiliser `--dangerously-skip-permissions` du tout, définissez `permissions.disableBypassPermissionsMode` sur `"disable"` dans les [paramètres gérés](/docs/fr/settings#permission-settings).

<h2 id="try-the-reference-container">
  Essayer le conteneur de référence
</h2>

Le référentiel [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/.devcontainer) inclut un exemple de dev container qui combine la CLI, le pare-feu de sortie, les volumes persistants et un shell basé sur Zsh. Il est fourni comme exemple fonctionnant plutôt que comme image de base maintenue ; utilisez-le pour voir comment les pièces s'assemblent avant de les appliquer à votre propre configuration.

<Steps>
  <Step title="Installer les prérequis">
    Installez VS Code et l'[extension Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).
  </Step>

  <Step title="Cloner la référence">
    Clonez le [référentiel Claude Code](https://github.com/anthropics/claude-code) et ouvrez-le dans VS Code.
  </Step>

  <Step title="Rouvrir dans le conteneur">
    Lorsque vous y êtes invité, cliquez sur **Rouvrir dans le conteneur**, ou exécutez **Dev Containers: Reopen in Container** depuis la Palette de commandes.
  </Step>

  <Step title="Démarrer Claude Code">
    Une fois la construction du conteneur terminée, ouvrez un terminal avec `` Ctrl+` `` et exécutez `claude` pour vous connecter et démarrer votre première session.
  </Step>
</Steps>

Pour utiliser cette configuration avec votre propre projet, copiez le répertoire `.devcontainer/` dans votre référentiel et ajustez le Dockerfile pour votre chaîne d'outils, ou retournez à [Ajouter Claude Code à votre dev container](#add-claude-code-to-your-dev-container) pour ajouter uniquement la fonctionnalité à une configuration que vous avez déjà.

La configuration de référence se compose de trois fichiers. Aucun d'eux n'est requis lorsque vous ajoutez Claude Code à votre propre dev container via la fonctionnalité, mais ils montrent une façon de combiner les pièces.

| Fichier                                                                                                    | Objectif                                                                      |
| ---------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| [`devcontainer.json`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) | Montages de volume, capacités `runArgs`, extensions VS Code et `containerEnv` |
| [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/Dockerfile)               | Image de base, outils de développement et l'installation de Claude Code       |
| [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh)   | Bloque tout le trafic réseau sortant sauf les domaines autorisés              |

<h2 id="next-steps">
  Étapes suivantes
</h2>

Une fois Claude Code en cours d'exécution dans votre dev container, les pages ci-dessous couvrent le reste d'un déploiement organisationnel : choisir un chemin d'authentification, livrer une politique gérée en dehors du référentiel, surveiller l'utilisation et comprendre ce que Claude Code stocke et envoie.

* [Configurer Claude Code pour votre organisation](/docs/fr/admin-setup) : choisir un fournisseur d'authentification, décider comment la politique atteint les appareils et planifier le déploiement
* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) : livrer une politique gérée depuis la console d'administration Claude.ai afin que les ingénieurs ne puissent pas la contourner en modifiant les fichiers du référentiel
* [Surveiller l'utilisation et l'activité d'audit](/docs/fr/monitoring-usage) : exporter les métriques OpenTelemetry et examiner ce que votre équipe exécute
* [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements) : la liste complète des domaines pour les proxies et les pare-feu
* [Services de télémétrie et désactivation](/docs/fr/data-usage#telemetry-services) : ce que Claude Code envoie par défaut et les variables d'environnement qui le désactivent
* [Explorer le répertoire `.claude`](/docs/fr/claude-directory) : ce que le montage de volume contient, y compris les identifiants, les paramètres et l'historique de session
* [Environnements sandbox](/docs/fr/sandbox-environments) : comparer les dev containers avec le sandbox Bash intégré, les conteneurs personnalisés et les machines virtuelles
* [Modèle de sécurité](/docs/fr/security) : comment le système de permission de Claude Code, le sandboxing et les protections contre l'injection de prompt s'assemblent
* [Modes de permission](/docs/fr/permission-modes) : la gamme complète du mode plan au mode auto au contournement, et quand utiliser chacun
