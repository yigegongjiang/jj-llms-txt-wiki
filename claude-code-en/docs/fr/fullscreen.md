> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rendu en plein écran

> Activez un mode de rendu plus fluide et sans scintillement avec support de la souris et une utilisation mémoire stable dans les longues conversations.

<Note>
  Le rendu en plein écran est un [aperçu de recherche](#research-preview) optionnel. Exécutez `/tui fullscreen` pour basculer dans votre conversation actuelle. Le comportement peut changer en fonction des retours.
</Note>

Le rendu en plein écran est un chemin de rendu alternatif pour le CLI Claude Code qui élimine le scintillement, maintient l'utilisation mémoire plate dans les longues conversations et ajoute le support de la souris. Il dessine l'interface sur le tampon d'écran alternatif du terminal, comme `vim` ou `htop`, et ne rend que les messages actuellement visibles. Cela réduit la quantité de données envoyées à votre terminal à chaque mise à jour.

La différence est plus notable dans les émulateurs de terminal où le débit de rendu est le goulot d'étranglement, comme le terminal intégré VS Code, tmux et iTerm2. Si votre position de défilement du terminal saute vers le haut pendant que Claude travaille, ou si l'écran clignote à mesure que la sortie de l'outil s'affiche, ce mode résout ces problèmes.

<Note>
  Le terme plein écran décrit comment Claude Code prend le contrôle de la surface de dessin du terminal, de la même manière que `vim`. Cela n'a rien à voir avec la maximisation de votre fenêtre de terminal, et fonctionne à n'importe quelle taille de fenêtre.
</Note>

<h2 id="enable-fullscreen-rendering">
  Activer le rendu en plein écran
</h2>

Exécutez `/tui fullscreen` dans n'importe quelle conversation Claude Code. Le CLI enregistre le [paramètre `tui`](/docs/fr/settings#available-settings) et redémarre en plein écran avec votre conversation intacte, afin que vous puissiez basculer en cours de session sans perdre le contexte. Exécutez `/tui default` pour revenir au moteur de rendu classique, ou `/tui` sans argument pour afficher quel moteur de rendu est actif.

La session relancée conserve la conversation telle qu'elle apparaît à l'écran. Si vous aviez exécuté [`/rewind`](/docs/fr/checkpointing#rewind-and-summarize) plus tôt dans la session, le redémarrage reprend à partir du point rembobiné plutôt que de la transcription plus longue enregistrée sur le disque. Avant la v2.1.207, le changement de moteurs de rendu après un rembobinage restaurait la conversation que le rembobinage avait supprimée.

Vous pouvez également définir la variable d'environnement `CLAUDE_CODE_NO_FLICKER` avant de démarrer Claude Code :

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Le paramètre `tui` et la variable d'environnement sont équivalents. La commande `/tui` efface `CLAUDE_CODE_NO_FLICKER` du processus relancé afin que le paramètre qu'elle écrit prenne effet.

<h2 id="what-changes">
  Ce qui change
</h2>

Le rendu en plein écran change la façon dont le CLI dessine sur votre terminal. La boîte d'entrée reste fixe en bas de l'écran au lieu de se déplacer à mesure que la sortie s'affiche. Si l'entrée reste en place pendant que Claude travaille, le rendu en plein écran est actif. Seuls les messages visibles sont conservés dans l'arborescence de rendu, donc la mémoire reste constante quel que soit la longueur de la conversation.

Parce que la conversation vit dans le tampon d'écran alternatif au lieu du scrollback de votre terminal, quelques choses fonctionnent différemment :

| Avant                                                            | Maintenant                                                                                          | Détails                                                                       |
| :--------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- |
| `Cmd+f` ou recherche tmux pour trouver du texte                  | `Ctrl+o` pour le mode transcription, puis `/` pour rechercher ou `[` pour écrire dans le scrollback | [Rechercher et examiner la conversation](#search-and-review-the-conversation) |
| Clic et glissement natif du terminal pour sélectionner et copier | Sélection dans l'application, copie automatique au relâchement de la souris                         | [Utiliser la souris](#use-the-mouse)                                          |
| `Cmd`-clic pour ouvrir une URL                                   | `Cmd`-clic sur macOS, `Ctrl`-clic ailleurs                                                          | [Utiliser la souris](#use-the-mouse)                                          |

Si la capture de souris interfère avec votre flux de travail, vous pouvez [la désactiver](#keep-native-text-selection) tout en conservant le rendu sans scintillement.

<h2 id="use-the-mouse">
  Utiliser la souris
</h2>

Le rendu en plein écran capture les événements de souris et les traite dans Claude Code :

* **Cliquez dans l'entrée d'invite** pour positionner votre curseur n'importe où dans le texte que vous tapez.
* **Cliquez sur une suggestion dans la liste de commandes `/` ou de fichiers `@`** pour l'accepter. Le survol met en évidence la ligne sous votre curseur.
* **Cliquez sur une option dans un menu de sélection** pour la choisir. Cela couvre les invites de permission, `/model`, `/config` et autres dialogues qui affichent une liste d'options. Le survol affiche un pointeur sur la ligne sous votre curseur. Nécessite Claude Code v2.1.187 ou ultérieur.
* **Cliquez sur une option dans un menu de sélection multiple** pour la basculer, et cliquez sur le bouton de soumission pour confirmer vos choix. Cliquer sur une ligne de texte libre, comme la ligne `Other` dans une question à choix multiples, met le focus sur son champ de saisie afin que vous puissiez taper une réponse. Nécessite Claude Code v2.1.208 ou ultérieur.
* **Cliquez sur un résultat d'outil réduit** pour le développer et voir la sortie complète. Cliquez à nouveau pour le réduire. L'appel d'outil et son résultat se développent ensemble. Seuls les messages qui ont plus à afficher sont cliquables.
* **Maintenez `Cmd` sur macOS, ou `Ctrl` sur Linux et Windows, et cliquez sur une URL ou un chemin de fichier** pour l'ouvrir. Les chemins de fichiers dans la sortie de l'outil, comme ceux imprimés après une modification ou une écriture, s'ouvrent dans votre application par défaut. Les URLs `http://` et `https://` simples s'ouvrent dans votre navigateur. À partir de la v2.1.181, un simple clic sans maintenir `Cmd` ou `Ctrl` n'ouvre plus les liens, ce qui correspond au comportement du terminal natif. Certains terminaux macOS transfèrent `Cmd`+clic à l'application en cours d'exécution au lieu d'ouvrir le lien eux-mêmes, et le protocole de souris du terminal n'a aucun moyen d'encoder la clé `Cmd`, donc Claude Code le reçoit comme un simple clic. Dans Ghostty, et à partir de la v2.1.198 dans Warp sur macOS, Claude Code détecte cela et permet à un simple clic sur un lien de l'ouvrir, et maintenir `Cmd` fonctionne toujours. Dans le terminal intégré VS Code et les terminaux basés sur xterm.js similaires, Claude Code se défère au gestionnaire de lien du terminal, qui utilise le même geste.
* **Cliquez et glissez** pour sélectionner du texte n'importe où dans la conversation. Double-cliquez pour sélectionner un mot, en correspondant avec les limites de mots d'iTerm2 afin qu'un chemin de fichier se sélectionne comme une unité. À partir de la v2.1.198, double-cliquer sur une URL sélectionne l'URL entière, y compris le schéma. Triple-cliquez pour sélectionner la ligne.
* **Faites défiler avec la molette de la souris** pour vous déplacer dans la conversation.

Le texte sélectionné est copié automatiquement dans votre presse-papiers au relâchement de la souris. Pour désactiver cela, basculez Copier à la sélection dans `/config`.

Avec Copier à la sélection désactivé, appuyez sur `Ctrl+Shift+c` pour copier manuellement. Sur les terminaux qui supportent le protocole clavier kitty, comme kitty, WezTerm, Ghostty et iTerm2, `Cmd+c` fonctionne également. Si vous avez une sélection active, `Ctrl+c` copie au lieu d'annuler.

Avec une sélection active, maintenez `Shift` et appuyez sur les touches fléchées pour l'étendre à partir du clavier. `Shift+↑` et `Shift+↓` font défiler la fenêtre d'affichage lorsque la sélection atteint le bord supérieur ou inférieur. `Shift+Home` et `Shift+End` étendent jusqu'au début ou à la fin de la ligne actuelle.

<h2 id="scroll-the-conversation">
  Faire défiler la conversation
</h2>

Le rendu en plein écran gère le défilement dans l'application. Utilisez ces raccourcis pour naviguer :

| Raccourci            | Action                                                     |
| :------------------- | :--------------------------------------------------------- |
| `PgUp` / `PgDn`      | Faites défiler vers le haut ou vers le bas de moitié écran |
| `Ctrl+Home`          | Allez au début de la conversation                          |
| `Ctrl+End`           | Allez au dernier message et réactivez le suivi automatique |
| Molette de la souris | Faites défiler quelques lignes à la fois                   |

Sur les claviers sans touches `PgUp`, `PgDn`, `Home` ou `End` dédiées, comme les claviers MacBook, maintenez `Fn` avec les touches fléchées : `Fn+↑` envoie `PgUp`, `Fn+↓` envoie `PgDn`, `Fn+←` envoie `Home`, et `Fn+→` envoie `End`. `Ctrl+Fn+→` n'atteint pas Claude Code sur macOS, donc un clavier MacBook n'a pas de raccourci de saut vers le bas fonctionnant par défaut. À la place, utilisez l'une de ces options :

* Cliquez sur le [bouton de saut vers le bas](#auto-follow).
* Faites défiler vers le bas avec la molette de la souris pour reprendre le suivi.
* Reliez `scroll:bottom` à un raccourci que votre clavier peut envoyer.

Ces actions sont reliables. Consultez [Actions de défilement](/docs/fr/keybindings#scroll-actions) pour la liste complète des noms d'actions, y compris les variantes de demi-page et de page complète qui n'ont pas de liaison par défaut.

<h3 id="auto-follow">
  Suivi automatique
</h3>

Le défilement vers le haut met en pause le suivi automatique afin que la nouvelle sortie ne vous ramène pas vers le bas. Un bouton `Aller au bas` flotte sur le bord inférieur de la transcription pendant que vous êtes défilé vers le haut, et affiche un décompte tel que `3 nouveaux messages` lorsque la nouvelle sortie arrive. Cliquez dessus, appuyez sur `Ctrl+End`, ou faites défiler vers le bas pour reprendre le suivi.

Pendant que le suivi automatique est en pause, la vue reste également où vous l'avez défilée lorsqu'une réponse termine le streaming. Avant la v2.1.207, la vue pouvait sauter au-dessus du début de la réponse lorsqu'une longue réponse terminait le streaming.

L'indice clavier du bouton reflète ce que votre clavier peut envoyer. Sur macOS, il suggère de cliquer, ou `Fn+↓` pour faire défiler, car `Ctrl+End` n'atteint pas Claude Code depuis un clavier Mac. Reliez [`scroll:bottom`](/docs/fr/keybindings#scroll-actions) et le bouton affiche votre raccourci sur chaque plateforme. Avant la v2.1.206, le bouton suggérait `Ctrl+End` sur macOS.

Sur un terminal trop étroit pour l'étiquette complète, le bouton raccourcit l'indice au lieu de l'enrouler sur la ligne de transcription en dessous. Avant la v2.1.206, une longue étiquette pouvait s'enrouler sur la transcription.

Pour désactiver entièrement le suivi automatique afin que la vue reste où vous la laissez, ouvrez `/config` et définissez Défilement automatique sur désactivé. Avec le défilement automatique désactivé, la vue ne saute jamais vers le bas d'elle-même. Les invites de permission et autres dialogues qui nécessitent une réponse font toujours défiler dans la vue indépendamment de ce paramètre.

<h3 id="mouse-wheel-scrolling">
  Défilement à la molette de la souris
</h3>

Le défilement à la molette de la souris nécessite que votre terminal transmette les événements de souris à Claude Code. La plupart des terminaux le font chaque fois qu'une application le demande. iTerm2 en fait un paramètre par profil : si la molette ne fait rien mais que `PgUp` et `PgDn` fonctionnent, ouvrez Paramètres → Profils → Terminal et activez Activer le rapport de souris. Le même paramètre est également requis pour que le clic pour développer et la sélection de texte fonctionnent.

Si le défilement à la molette de la souris semble lent, votre terminal envoie peut-être un événement de défilement par cran physique sans multiplicateur. Certains terminaux, comme Ghostty et iTerm2 avec défilement plus rapide activé, amplifient déjà les événements de molette. D'autres, y compris le terminal intégré VS Code, envoient exactement un événement par cran. Claude Code ne peut pas détecter lequel.

Définissez `CLAUDE_CODE_SCROLL_SPEED` pour multiplier la distance de défilement de base :

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Une valeur de `3` correspond à la valeur par défaut dans `vim` et les applications similaires. Le paramètre accepte les valeurs de 1 à 20, et les valeurs fractionnaires inférieures à 1 telles que `0,5` pour ralentir le défilement accéléré du trackpad et de la molette dans les terminaux qui amplifient déjà les événements de molette.

Pour ajuster la vitesse de défilement de manière interactive, exécutez `/scroll-speed`. La boîte de dialogue affiche une règle que vous pouvez faire défiler pendant qu'elle est ouverte afin que vous puissiez ressentir le changement immédiatement. Appuyez sur `←` et `→` pour ajuster, `r` pour réinitialiser à la valeur par défaut détectée automatiquement, et `Entrée` pour enregistrer.

La commande écrit la même valeur que la variable d'environnement `CLAUDE_CODE_SCROLL_SPEED` définit, persistée à `~/.claude/settings.json`. La commande n'est pas disponible dans le terminal de l'IDE JetBrains.

Indépendamment de la vitesse de base, Claude Code accélère la vitesse de défilement lorsque vous tournez la molette rapidement, donc une rotation rapide couvre plus de distance que le même nombre de crans lents. Pour désactiver l'accélération et maintenir une vitesse constante par cran, définissez `wheelScrollAccelerationEnabled` sur `false` dans [`settings.json`](/docs/fr/settings#available-settings). Ce paramètre nécessite Claude Code v2.1.174 ou une version ultérieure.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Défilement dans le terminal de l'IDE JetBrains
</h3>

Dans le terminal de l'IDE JetBrains, Claude Code applique sa propre gestion du défilement et ignore `CLAUDE_CODE_SCROLL_SPEED`. Le terminal envoie les événements de défilement à un taux beaucoup plus élevé que les autres émulateurs, donc un multiplicateur accordé ailleurs dépasse la limite ici.

En 2025.2, le terminal a également des bogues de défilement à la molette qui produisent des touches fléchées parasites et des événements de mauvaise direction. Claude Code détecte ces bogues à l'exécution et les atténue automatiquement, donc le défilement au trackpad et à la molette de la souris fonctionnent sans configuration. Pour la meilleure expérience de défilement, mettez à niveau vers 2025.3 ou une version ultérieure. Claude Code affiche un indice la première fois que vous faites défiler si le bogue est détecté.

<h2 id="search-and-review-the-conversation">
  Rechercher et examiner la conversation
</h2>

`Ctrl+o` bascule entre l'invite normale et le mode transcription.

Pour une vue plus calme qui affiche uniquement votre dernière invite, un résumé d'une ligne des appels d'outil avec les statistiques de diff de modification, et la réponse finale, exécutez `/focus`. Le paramètre persiste entre les sessions. Exécutez `/focus` à nouveau pour le désactiver.

Le mode transcription gagne la navigation et la recherche de style `less` :

| Touche                                | Action                                                                                                                                              |
| :------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                   | Ouvrir la recherche. Tapez pour trouver des correspondances, `Entrée` pour accepter, `Échap` pour annuler et restaurer votre position de défilement |
| `n` / `N`                             | Allez à la correspondance suivante ou précédente. Fonctionne après avoir fermé la barre de recherche                                                |
| `j` / `k` ou `↑` / `↓`                | Faites défiler une ligne                                                                                                                            |
| `g` / `G` ou `Home` / `End`           | Allez au début ou à la fin                                                                                                                          |
| `Ctrl+u` / `Ctrl+d`                   | Faites défiler une demi-page                                                                                                                        |
| `Ctrl+b` / `Ctrl+f` ou `Espace` / `b` | Faites défiler une page complète                                                                                                                    |
| `Ctrl+o`, `Échap`, ou `q`             | Quitter le mode transcription et revenir à l'invite                                                                                                 |

La `Cmd+f` de votre terminal et la recherche tmux ne voient pas la conversation car elle vit dans le tampon d'écran alternatif, pas le scrollback natif. Pour rendre le contenu à votre terminal, appuyez sur `Ctrl+o` pour entrer d'abord en mode transcription, puis :

* **`[`** : écrit la conversation complète dans le tampon de scrollback natif de votre terminal, avec toute la sortie de l'outil développée. La conversation est maintenant du texte ordinaire dans votre terminal, donc `Cmd+f`, le mode copie tmux et tout autre outil natif peuvent la rechercher ou la sélectionner. Les longues sessions peuvent faire une pause pendant un moment pendant que cela se produit. Cela dure jusqu'à ce que vous quittiez le mode transcription avec `Échap` ou `q`, ce qui vous ramène au rendu en plein écran. Le prochain `Ctrl+o` recommence à zéro.
* **`v`** : écrit la conversation dans un fichier temporaire et l'ouvre dans `$VISUAL` ou `$EDITOR`.

Appuyez sur `Échap` ou `q` pour revenir à l'invite.

<h2 id="clear-the-conversation">
  Effacer la conversation
</h2>

Appuyez deux fois sur `Ctrl+L` dans les deux secondes pour exécuter `/clear` et démarrer une nouvelle conversation. Le premier appui redessine l'écran et affiche un indice ; le deuxième appui efface la conversation. Sur macOS, appuyer deux fois sur `Cmd+K` exécute également `/clear`.

<h2 id="use-with-tmux">
  Utiliser avec tmux
</h2>

Le rendu en plein écran fonctionne à l'intérieur de tmux, avec trois mises en garde.

Le défilement à la molette de la souris nécessite le mode souris de tmux. Si votre `~/.tmux.conf` ne l'active pas déjà, ajoutez cette ligne et rechargez votre configuration :

```bash theme={null}
set -g mouse on
```

Sans le mode souris, les événements de molette vont à tmux au lieu de Claude Code. Le défilement au clavier avec `PgUp` et `PgDn` fonctionne de toute façon. Claude Code imprime un indice unique au démarrage s'il détecte tmux avec le mode souris désactivé.

Le rendu en plein écran est incompatible avec le mode d'intégration tmux d'iTerm2, qui est le mode dans lequel vous entrez avec `tmux -CC`. En mode intégration, iTerm2 rend chaque volet tmux comme une division native plutôt que de laisser tmux dessiner sur le terminal. Le tampon d'écran alternatif et le suivi de la souris ne fonctionnent pas correctement là : la molette de la souris ne fait rien, et le double-clic peut corrompre l'état du terminal. N'activez pas le rendu en plein écran dans les sessions `tmux -CC`. Le tmux régulier à l'intérieur d'iTerm2, sans `-CC`, fonctionne bien.

Toutes les versions de tmux n'appliquent pas la sortie synchronisée des applications, donc vous pouvez voir plus de scintillement lors des redessins sous tmux que lors de l'exécution de Claude Code directement dans votre terminal. Si le scintillement est notable, en particulier sur SSH, mettez à niveau vers la dernière version de tmux ou exécutez Claude Code dans son propre onglet de terminal en dehors de tmux. Vérifiez votre version de tmux avec `tmux -V`.

Claude Code active la sortie synchronisée automatiquement lorsqu'il détecte tmux 3.4 ou version ultérieure à partir de la variable `TERM_PROGRAM_VERSION`, et revient à l'interrogation directe du terminal pour la prise en charge de la sortie synchronisée lorsque la version ne peut pas être déterminée. Le fait que les redessins deviennent réellement atomiques dépend de votre version de tmux respectant la sortie synchronisée ; si vous voyez toujours du scintillement sous tmux 3.4 ou version ultérieure, mettez à niveau vers la dernière version de tmux. Cette détection nécessite Claude Code v2.1.200 ou version ultérieure.

<h2 id="keep-native-text-selection">
  Conserver la sélection de texte natif
</h2>

La capture de souris est le point de friction le plus courant, surtout sur SSH ou à l'intérieur de tmux. Lorsque Claude Code capture les événements de souris, la copie à la sélection natif de votre terminal cesse de fonctionner. La sélection que vous faites avec clic et glissement existe à l'intérieur de Claude Code, pas dans le tampon de sélection de votre terminal, donc le mode copie tmux, les indices Kitty et les outils similaires ne la voient pas.

Claude Code écrit la sélection dans votre presse-papiers système, et le chemin qu'il utilise dépend de votre configuration. Sur une session locale, il exécute un outil de presse-papiers natif :

* **macOS** : `pbcopy`
* **Linux** : `wl-copy` sur Wayland, ou `xclip` ou `xsel` sur X11, selon ce qui est installé. Claude Code écrit à la fois le presse-papiers et la sélection PRIMARY, donc le collage au clic du milieu fonctionne.
* **Windows et WSL** : PowerShell `Set-Clipboard`

À l'intérieur de tmux, il écrit également dans le tampon de collage tmux. Sur SSH, il revient aux séquences d'échappement OSC 52. Claude Code imprime un toast après chaque copie vous indiquant quel chemin il a utilisé.

Certains terminaux bloquent OSC 52 par défaut. iTerm2 le bloque jusqu'à ce que vous activiez Paramètres → Général → Sélection → Les applications du terminal peuvent accéder au presse-papiers ; exécuter [`/terminal-setup`](/docs/fr/terminal-config) dans iTerm2 active cela pour vous.

Pour une sélection natif ponctuelle, la touche à utiliser dépend de votre terminal :

* **Terminal.app** : `Fn`
* **iTerm2** : `Option`
* **VS Code, Cursor et Devin Desktop** : `Shift`, ou `Option` sur macOS avec le paramètre `terminal.integrated.macOptionClickForcesSelection` activé
* **La plupart des autres terminaux** : `Shift`

Maintenez cette touche enfoncée pendant que vous cliquez et glissez. Votre terminal gère la sélection lui-même au lieu de la transférer à Claude Code, donc les raccourcis de copie comme `Cmd+C` fonctionnent sur ce que vous sélectionnez. Claude Code affiche également la touche correcte dans son indice à l'écran.

Sur SSH ou à l'intérieur de tmux, Claude Code ne peut pas toujours détecter le terminal auquel vous vous connectez, donc l'indice liste les touches candidates à la place.

Si vous comptez sur la sélection natif tout le temps, définissez `CLAUDE_CODE_DISABLE_MOUSE=1` pour refuser la capture de souris tout en conservant le rendu sans scintillement et la mémoire plate :

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Avec la capture de souris désactivée, le défilement au clavier avec `PgUp`, `PgDn`, `Ctrl+Home` et `Ctrl+End` fonctionne toujours, et votre terminal gère la sélection nativement. Vous perdez le clic pour positionner le curseur, le clic pour développer la sortie de l'outil, le clic sur URL et le défilement à la molette à l'intérieur de Claude Code.

Pour conserver le défilement à la molette mais désactiver le clic, le glissement et la gestion du survol, définissez `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1` à la place. Nécessite Claude Code v2.1.195 ou version ultérieure. `CLAUDE_CODE_DISABLE_MOUSE` a la priorité lorsque les deux variables sont définies.

Avec les clics désactivés, Claude Code capture toujours la souris, donc la molette et le pavé tactile font défiler la conversation mais les clics gauches ne font rien à l'intérieur de Claude Code. Vous devez toujours maintenir la touche de votre terminal enfoncée pour la sélection natif par clic et glissement. Le clic droit et le collage au clic du milieu continuent de fonctionner sur les terminaux qui les supportent.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Texte obsolète ou mal placé à l'écran
</h3>

Le rendu en plein écran envoie uniquement les cellules qui ont changé entre les images. Certains terminaux, notamment Windows Terminal et autres hôtes basés sur ConPTY, fusionnent ces écritures positionnées de manière incorrecte et laissent des fragments de la sortie antérieure à l'écran jusqu'à ce que vous redimensionniez la fenêtre.

Définissez [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/fr/env-vars) pour repeindre chaque cellule à chaque image au lieu d'envoyer des mises à jour incrémentielles.

Sur Windows PowerShell :

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

Sur macOS ou Linux :

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

Sur Windows, Claude Code active déjà le repeint complet automatiquement pour les sessions en arrière-plan et la [vue agent](/docs/fr/agent-view), vous devez donc uniquement définir la variable pour une session interactive en plein écran que vous avez lancée directement.

<h2 id="research-preview">
  Aperçu de recherche
</h2>

Le rendu en plein écran est une fonctionnalité d'aperçu de recherche. Il a été testé sur les émulateurs de terminal courants, mais vous pouvez rencontrer des problèmes de rendu sur les terminaux moins courants ou les configurations inhabituelles.

Si vous rencontrez un problème, exécutez `/feedback` à l'intérieur de Claude Code pour le signaler, ou ouvrez un problème sur le [référentiel GitHub claude-code](https://github.com/anthropics/claude-code/issues). Incluez le nom et la version de votre émulateur de terminal.

Pour désactiver le rendu en plein écran, exécutez `/tui default`, ou déconfigurez `CLAUDE_CODE_NO_FLICKER` si vous l'avez activée de cette façon. Pour forcer le moteur de rendu classique indépendamment du paramètre `tui` enregistré, définissez `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Le moteur de rendu classique conserve la conversation dans le défilement natif de votre terminal afin que `Cmd+f` et le mode copie tmux fonctionnent comme d'habitude.

Les sessions en arrière-plan ouvertes à partir de la [vue agent](/docs/fr/agent-view) ou `claude attach` utilisent toujours le rendu en plein écran. Le terminal d'attachement entre dans le tampon d'écran alternatif pour afficher la session, et le moteur de rendu classique n'a pas de défilement ou de gestion de la souris là-bas, donc le paramètre `tui` et `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` ne s'appliquent pas à eux.
