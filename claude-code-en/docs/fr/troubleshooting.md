> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dépannage

> Corrigez l'utilisation élevée du CPU ou de la mémoire, les blocages, le thrashing de l'auto-compaction et les problèmes de recherche dans Claude Code, et trouvez la bonne page pour d'autres problèmes.

Cette page couvre les problèmes de performance, de stabilité et de recherche une fois que Claude Code est en cours d'exécution. Pour d'autres problèmes, commencez par la page qui correspond à votre situation :

| Symptôme                                                                                                                                                           | Aller à                                                                                      |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| `command not found`, l'installation échoue, problèmes de PATH, `EACCES`, erreurs TLS                                                                               | [Dépanner l'installation et la connexion](/docs/fr/troubleshoot-install)                          |
| Mise à jour ou l'installation du téléchargement échoue avec `The connection dropped while downloading the update` ou `aborted`                                     | [Référence des erreurs](/docs/fr/errors#the-connection-dropped-while-downloading-the-update)      |
| Boucles de connexion, erreurs OAuth, `403 Forbidden`, « organisation désactivée », identifiants Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry | [Dépanner l'installation et la connexion](/docs/fr/troubleshoot-install#login-and-authentication) |
| Les paramètres ne s'appliquent pas, les hooks ne se déclenchent pas, les serveurs MCP ne se chargent pas                                                           | [Déboguer votre configuration](/docs/fr/debug-your-config)                                        |
| `API Error: 5xx`, `529 Overloaded`, `429`, erreurs de validation de requête                                                                                        | [Référence des erreurs](/docs/fr/errors)                                                          |
| `model not found` ou `you may not have access to it`                                                                                                               | [Référence des erreurs](/docs/fr/errors#theres-an-issue-with-the-selected-model)                  |
| L'extension VS Code ne se connecte pas ou ne détecte pas Claude                                                                                                    | [Intégration VS Code](/docs/fr/vs-code#fix-common-issues)                                         |
| Le plugin JetBrains ou l'IDE n'est pas détecté                                                                                                                     | [Intégration JetBrains](/docs/fr/jetbrains#troubleshooting)                                       |
| Utilisation élevée du CPU ou de la mémoire, réponses lentes, blocages, la recherche ne trouve pas les fichiers                                                     | [Performance et stabilité](#performance-and-stability) ci-dessous                            |

Si vous n'êtes pas sûr de ce qui s'applique, exécutez `/doctor` dans Claude Code pour une vérification automatisée de votre installation, vos paramètres, vos extensions et votre utilisation du contexte ; il propose des corrections qu'il peut appliquer après votre confirmation. Si `claude` ne démarre pas du tout, exécutez `claude doctor` depuis votre shell à la place. Exécutez `/mcp` pour vérifier l'état du serveur MCP.

<h2 id="performance-and-stability">
  Performance et stabilité
</h2>

Ces sections couvrent les problèmes liés à l'utilisation des ressources, la réactivité et le comportement de recherche.

<h3 id="high-cpu-or-memory-usage">
  Utilisation élevée du CPU ou de la mémoire
</h3>

Claude Code est conçu pour fonctionner avec la plupart des environnements de développement, mais peut consommer des ressources importantes lors du traitement de grandes bases de code. Si vous rencontrez des problèmes de performance :

1. Utilisez `/compact` régulièrement pour réduire la taille du contexte
2. Fermez et redémarrez Claude Code entre les tâches majeures
3. Envisagez d'ajouter les grands répertoires de construction à votre fichier `.gitignore`
4. Redémarrez avec [`claude --safe-mode`](/docs/fr/cli-reference#cli-flags) pour vérifier si un plugin, un serveur MCP ou un hook est la source. Cela désactive toutes les personnalisations pour la session ; si l'utilisation diminue, consultez [Déboguer votre configuration](/docs/fr/debug-your-config#test-against-a-clean-configuration) pour trouver lequel

Si l'utilisation de la mémoire reste élevée après ces étapes, exécutez `/heapdump` pour écrire un snapshot de tas JavaScript et une ventilation de la mémoire sur `~/Desktop`. Sur Linux sans dossier Desktop, les fichiers sont écrits dans votre répertoire personnel.

La ventilation affiche la taille de l'ensemble résidant, le tas JS, les tampons de tableau et la mémoire native non comptabilisée, ce qui aide à identifier si la croissance se trouve dans les objets JavaScript ou dans le code natif. Pour inspecter les rétenteurs, ouvrez le fichier `.heapsnapshot` dans Chrome DevTools sous Memory → Load ; la ventilation est le fichier se terminant par `-diagnostics.json`.

<Warning>
  Le fichier `.heapsnapshot` contient chaque chaîne du processus. Ne l'attachez pas à un problème public ou ne le partagez pas. Attachez uniquement le fichier `-diagnostics.json` lors de la signalisation d'un problème de mémoire sur [GitHub](https://github.com/anthropics/claude-code/issues). Ce fichier contient les statistiques de mémoire et aucun contenu de conversation ou identifiants.
</Warning>

<h3 id="large-tables-are-cut-off-in-the-terminal">
  Les grandes tables sont coupées dans le terminal
</h3>

Un tableau Markdown avec plus de 200 lignes affiche ses 200 premières lignes suivies d'une ligne `… N more rows not shown`. Seul l'affichage est limité : le tableau complet reste dans la conversation, et [`/copy`](/docs/fr/commands) copie chaque ligne. Pour un tableau trop volumineux pour être lu dans le terminal, demandez à Claude de l'écrire dans un fichier à la place. Avant la v2.1.208, Claude Code affichait chaque ligne, donc reprendre une session qui contenait un très grand tableau pouvait se bloquer lors du re-rendu.

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  L'auto-compaction s'arrête avec une erreur de thrashing
</h3>

Si vous voyez `Autocompact is thrashing: the context refilled to the limit...`, la compaction automatique a réussi mais un fichier ou une sortie d'outil a immédiatement rempli la fenêtre de contexte plusieurs fois de suite. Claude Code arrête les tentatives pour éviter de gaspiller les appels API sur une boucle qui ne progresse pas.

Pour récupérer :

1. Demandez à Claude de lire le fichier surdimensionné en petits morceaux, comme une plage de lignes spécifique ou une fonction, au lieu du fichier entier
2. Exécutez `/compact` avec un focus qui supprime la sortie volumineuse, par exemple `/compact keep only the plan and the diff`
3. Déplacez le travail sur fichier volumineux vers un [sous-agent](/docs/fr/sub-agents) pour qu'il s'exécute dans une fenêtre de contexte séparée
4. Exécutez `/clear` si la conversation antérieure n'est plus nécessaire

<h3 id="command-hangs-or-freezes">
  Les commandes se figent ou se gèlent
</h3>

Si Claude Code semble ne pas répondre :

1. Appuyez sur Ctrl+C pour tenter d'annuler l'opération actuelle
2. Si ne répond pas, vous devrez peut-être fermer le terminal et redémarrer

Le redémarrage ne perd pas votre conversation. Exécutez `claude --resume` dans le même répertoire pour reprendre la session.

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  Texte garbled ou corrompu dans le terminal intégré d'un éditeur
</h3>

Si les caractères s'affichent sous forme de boîtes, de traînées ou de glyphes incorrects lors de l'exécution de Claude Code dans le terminal intégré de VS Code, Cursor ou Devin Desktop, le rendu GPU du terminal en est probablement la cause. Exécutez `/terminal-setup` dans Claude Code pour définir `terminal.integrated.gpuAcceleration` sur `"off"`, ou définissez-le manuellement dans les paramètres de votre éditeur et rechargez la fenêtre. Consultez [Configuration du terminal](/docs/fr/terminal-config) pour les autres paramètres que `/terminal-setup` écrit.

<h3 id="search-and-discovery-issues">
  Problèmes de recherche et de découverte
</h3>

Si l'outil Search, les mentions `@file`, les agents personnalisés ou les compétences personnalisées ne trouvent pas les fichiers, le binaire `ripgrep` fourni peut ne pas s'exécuter sur votre système. Installez le paquet `ripgrep` de votre plateforme et dites à Claude Code de l'utiliser à la place :

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

Ensuite, définissez `USE_BUILTIN_RIPGREP=0` dans votre [environnement](/docs/fr/env-vars).

<h3 id="slow-or-incomplete-search-results-on-wsl">
  Résultats de recherche lents ou incomplets sur WSL
</h3>

Les pénalités de performance de lecture de disque lors du [travail sur les systèmes de fichiers sur WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems) peuvent entraîner moins de correspondances que prévu lors de l'utilisation de Claude Code sur WSL. La recherche fonctionne toujours, mais retourne moins de résultats que sur un système de fichiers natif.

<Note>
  `claude doctor` affiche Search comme OK dans ce cas.
</Note>

**Solutions :**

1. **Soumettre des recherches plus spécifiques** : réduisez le nombre de fichiers recherchés en spécifiant des répertoires ou des types de fichiers : « Search for JWT validation logic in the auth-service package » ou « Find use of md5 hash in JS files ».

2. **Déplacer le projet vers le système de fichiers Linux** : si possible, assurez-vous que votre projet est situé sur le système de fichiers Linux (`/home/`) plutôt que sur le système de fichiers Windows (`/mnt/c/`).

3. **Utiliser Windows natif à la place** : envisagez d'exécuter Claude Code nativement sur Windows au lieu de via WSL, pour une meilleure performance du système de fichiers.

<h2 id="get-more-help">
  Obtenir plus d'aide
</h2>

Si vous rencontrez des problèmes non couverts ici :

1. Exécutez `/doctor` pour une vérification de la configuration et `/mcp` pour vérifier l'état du serveur MCP
2. Utilisez la commande `/feedback` dans Claude Code pour signaler les problèmes directement à Anthropic
3. Vérifiez le [référentiel GitHub](https://github.com/anthropics/claude-code) pour les problèmes connus
4. Demandez directement à Claude ses capacités et fonctionnalités. Claude a un accès intégré à sa documentation.
