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
| Session démarrée en mode auto, ou Claude modifie les fichiers et exécute les commandes sans demander                                                               | [Mode de démarrage d'une session](/docs/fr/permission-modes#which-mode-a-session-starts-in)       |
| `API Error: 5xx`, `529 Overloaded`, `429`, erreurs de validation de requête                                                                                        | [Référence des erreurs](/docs/fr/errors)                                                          |
| `model not found` ou `you may not have access to it`                                                                                                               | [Référence des erreurs](/docs/fr/errors#theres-an-issue-with-the-selected-model)                  |
| L'extension VS Code ne se connecte pas ou ne détecte pas Claude                                                                                                    | [Intégration VS Code](/docs/fr/vs-code#fix-common-issues)                                         |
| `Claude Code process exited with code 1` dans VS Code ou une application SDK                                                                                       | [Référence des erreurs](/docs/fr/errors#claude-code-process-exited-with-code-n)                   |
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

1. Utilisez `/compact` régulièrement pour réduire la taille du contexte. S'il retourne `Not enough messages to compact.`, la conversation a trop peu de tours à résumer ; cela peut se produire même avec un contexte complet quand un seul grand collage l'a rempli
2. Fermez et redémarrez Claude Code entre les tâches majeures
3. Envisagez d'ajouter les grands répertoires de construction à votre fichier `.gitignore`
4. Redémarrez avec [`claude --safe-mode`](/docs/fr/cli-reference#cli-flags) pour vérifier si un plugin, un serveur MCP ou un hook est la source. Cela désactive toutes les personnalisations pour la session ; si l'utilisation diminue, consultez [Déboguer votre configuration](/docs/fr/debug-your-config#test-against-a-clean-configuration) pour trouver lequel

Si la mémoire heap d'une session dépasse 2,5 Go, un avertissement critique d'utilisation de la mémoire apparaît. Pour libérer la mémoire, redémarrez Claude Code et exécutez [`claude --continue`](/docs/fr/cli-reference#cli-flags) pour reprendre la conversation dans un nouveau processus.

En dehors du [rendu en plein écran](/docs/fr/fullscreen), l'exécution de `/compact` libère également la mémoire. L'avertissement disparaît une fois que l'utilisation de la mémoire redescend en dessous de 2,5 Go.

Si l'utilisation de la mémoire reste élevée après ces étapes, exécutez `/heapdump` pour écrire deux fichiers sur `~/Desktop` : un snapshot de tas JavaScript nommé `<session-id>.heapsnapshot` et une ventilation de la mémoire nommée `<session-id>-diagnostics.json`. Claude Code [masque la commande du menu de commandes](/docs/fr/commands#how-the-command-menu-matches-what-you-type) ; tapez-la en entier. Sur Linux sans dossier Desktop, les fichiers sont écrits dans votre répertoire personnel.

<Warning>
  Le fichier `.heapsnapshot` contient chaque chaîne du processus, y compris votre conversation complète et vos identifiants. Ne l'attachez pas à un problème public ou ne le partagez pas.
</Warning>

La commande imprime également un résumé dans la conversation, affichant la taille de l'ensemble résidant, le tas JS, les tampons de tableau et la mémoire native non comptabilisée, plus tous les indicateurs de fuite qu'elle a détectés, comme un taux de croissance de la mémoire élevé ou un nombre inhabituellement élevé de handles ouverts. Le résumé indique si la plupart de la mémoire se trouve dans le tas JS, que le snapshot capture, ou dans la mémoire native, qu'il ne capture pas.

Faites l'une de ces deux choses avec la sortie :

* **Signalez-le** : ouvrez un [problème GitHub](https://github.com/anthropics/claude-code/issues) et attachez uniquement le fichier `-diagnostics.json`, qui contient les statistiques derrière le résumé imprimé et aucun contenu de conversation ou identifiants
* **Enquêtez vous-même** : si le résumé indique que la plupart de la mémoire se trouve dans le tas JS, ouvrez le fichier `.heapsnapshot` dans Chrome DevTools sous Memory → Load et triez par taille retenue pour voir ce qui retient la mémoire

Si le résumé indique que la plupart de la mémoire est native, le snapshot ne peut pas le montrer ; incluez plutôt les indicateurs de fuite du résumé dans votre rapport.

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

<h3 id="mouse-wheel-scrolls-one-line-at-a-time-in-fullscreen-rendering">
  La molette de la souris fait défiler une ligne à la fois dans le rendu en plein écran
</h3>

Dans le [rendu en plein écran](/docs/fr/fullscreen), Claude Code fait défiler la conversation elle-même plutôt que de la laisser à votre terminal. Si chaque cran de molette déplace moins de lignes que vous le souhaitez, exécutez `/scroll-speed` pour augmenter le nombre de lignes par cran et l'enregistrer, ou définissez la variable d'environnement `CLAUDE_CODE_SCROLL_SPEED`, sauf dans le terminal de l'IDE JetBrains, où Claude Code applique sa propre gestion du défilement et ni l'un ni l'autre ne prend effet. Consultez [Défilement à la molette de la souris](/docs/fr/fullscreen#mouse-wheel-scrolling) pour les valeurs que chacun accepte.

Pour vous déplacer plus rapidement sans changer la vitesse, appuyez sur `PgUp` et `PgDn` pour faire défiler demi-écran à la fois. Pour rendre le défilement à votre scrollback natif du terminal à la place, exécutez `/tui default` pour basculer vers le rendu classique.

<h3 id="clipboard-commands-such-as-pbcopy-fail-inside-the-sandbox">
  Les commandes du presse-papiers telles que `pbcopy` échouent à l'intérieur du sandbox
</h3>

Quand le [sandboxing](/docs/fr/sandboxing) est activé, les utilitaires du presse-papiers tels que `pbcopy`, `xclip` et `wl-copy` peuvent échouer à atteindre le presse-papiers système depuis une commande Bash en sandbox, laissant votre presse-papiers inchangé après que Claude ait canalisé du texte vers eux.

Pour mettre la sortie de Claude sur votre presse-papiers, demandez à Claude d'imprimer le contenu dans sa réponse, puis exécutez [`/copy`](/docs/fr/commands). `/copy` écrit dans le presse-papiers depuis le processus Claude Code lui-même plutôt que depuis une commande en sandbox, donc le sandboxing ne le bloque pas. Il peut copier un seul bloc de code au lieu de la réponse entière, et il écrit également ce qu'il a copié dans un fichier et imprime le chemin, ce qui vous donne une solution de secours quand l'écriture du presse-papiers n'atteint pas votre terminal, par exemple sur SSH.

Quand Claude canalise du texte vers l'un de ces outils, ajouter `pbcopy *`, `wl-copy *` ou `xclip *` à [`excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands) ne retire pas cet appel du sandbox en soi.

<h3 id="copied-text-doesn’t-reach-your-local-clipboard-over-ssh">
  Texte copié n'atteint pas votre presse-papiers local sur SSH
</h3>

Quand Claude Code s'exécute sur une machine distante via SSH, il ne peut pas exécuter un outil de presse-papiers sur votre machine locale. En dehors de tmux, quand vous sélectionnez du texte dans le [rendu en plein écran](/docs/fr/fullscreen) ou exécutez `/copy`, Claude Code envoie le texte à votre terminal en tant que séquence d'échappement OSC 52 à la place. Votre terminal décide s'il faut le mettre sur votre presse-papiers. `/copy` signale `Copied to clipboard` que le texte soit arrivé ou non, et en dehors de tmux l'avis de sélection lit `sent N chars via OSC 52`.

Certains terminaux n'agissent pas sur OSC 52. iTerm2 l'ignore jusqu'à ce que vous activiez **Settings > General > Selection > Applications in terminal may access clipboard**, et macOS Terminal.app ne le supporte pas.

Pour obtenir le texte sans OSC 52 :

* Maintenez la touche de sélection native de votre terminal pendant que vous faites glisser, puis copiez avec le raccourci habituel de votre terminal, tel que `Cmd+C`. La touche est `Fn` dans Terminal.app et `Option` dans iTerm2. [Garder la sélection de texte native](/docs/fr/fullscreen#keep-native-text-selection) la liste pour les autres terminaux.
* Définissez [`CLAUDE_CODE_DISABLE_MOUSE=1`](/docs/fr/env-vars) sur la machine distante pour que votre terminal gère la sélection pour la session entière.

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

    `ripgrep` se trouve dans le référentiel communautaire d'Alpine. Si `apk` signale que le paquet est manquant, consultez [Configuration d'Alpine Linux](/docs/fr/setup#alpine-linux-and-musl-based-distributions).
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

Ensuite, définissez `USE_BUILTIN_RIPGREP` sur `0`, soit dans votre [environnement](/docs/fr/env-vars) shell, soit dans le bloc `env` de votre [`settings.json`](/docs/fr/settings-reference#all-settings) :

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

Pour confirmer que le changement a pris effet, exécutez `claude doctor` dans votre terminal et vérifiez que la ligne Search affiche le chemin de votre ripgrep système au lieu de `OK (bundled)`.

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

Pour les problèmes de compte, de facturation ou d'abonnement, contactez le support Anthropic à la place : connectez-vous à [claude.ai](https://claude.ai) (Utilisateurs Console : [platform.claude.com](https://platform.claude.com)), cliquez sur vos initiales en bas à gauche, et sélectionnez **Obtenir de l'aide**. Consultez [Comment obtenir du support](https://support.claude.com/en/articles/9015913-how-to-get-support) pour le flux complet, y compris qui peut vous mettre en contact avec un agent humain selon votre plan.
