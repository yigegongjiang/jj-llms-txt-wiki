> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rendu en plein écran

> Activez un mode de rendu plus fluide et sans scintillement avec support de la souris et une utilisation mémoire stable dans les longues conversations.

<Note>
  Le rendu en plein écran est un [aperçu de recherche](#research-preview). Que vous [commenciez en plein écran ou dans le rendu classique](#fullscreen-by-default) dépend de votre configuration. Exécutez `/tui fullscreen` ou `/tui default` pour basculer dans votre conversation actuelle. Le comportement peut changer en fonction des retours.
</Note>

Le rendu en plein écran est un chemin de rendu alternatif pour le CLI Claude Code qui élimine le scintillement, maintient l'utilisation mémoire plate dans les longues conversations et ajoute le support de la souris. Il dessine l'interface sur le tampon d'écran alternatif du terminal, comme `vim` ou `htop`, et ne rend que les messages actuellement visibles. Cela réduit la quantité de données envoyées à votre terminal à chaque mise à jour.

La différence est plus notable dans les émulateurs de terminal où le débit de rendu est le goulot d'étranglement, comme le terminal intégré VS Code, tmux et iTerm2. Si votre position de défilement du terminal saute vers le haut pendant que Claude travaille, ou si l'écran clignote à mesure que la sortie de l'outil s'affiche, ce mode résout ces problèmes.

<Note>
  Le terme plein écran décrit comment Claude Code prend le contrôle de la surface de dessin du terminal, de la même manière que `vim`. Cela n'a rien à voir avec la maximisation de votre fenêtre de terminal, et fonctionne à n'importe quelle taille de fenêtre.
</Note>

<h2 id="enable-fullscreen-rendering">
  Activer le rendu en plein écran
</h2>

Exécutez `/tui fullscreen` dans n'importe quelle conversation Claude Code. Le CLI enregistre le [paramètre `tui`](/docs/fr/settings-reference#tui) et redémarre en plein écran avec votre conversation intacte, afin que vous puissiez basculer en cours de session sans perdre le contexte. Exécutez `/tui default` pour revenir au moteur de rendu classique, ou `/tui` sans argument pour afficher quel moteur de rendu est actif.

En [mode lecteur d'écran](/docs/fr/accessibility), Claude Code utilise toujours le moteur de rendu classique sauf dans les [sessions d'arrière-plan](/docs/fr/agent-view) attachées, qui restent en rendu plein écran. Si vous exécutez `/tui fullscreen` dans toute autre session, Claude Code affiche une explication au lieu de basculer et ne modifie pas le paramètre `tui` enregistré.

Claude Code conserve ceux-ci dans la session relancée :

* La conversation telle qu'elle apparaît à l'écran. Après un [`/rewind`](/docs/fr/checkpointing#rewind-and-summarize), cela signifie :
  * Si vous avez rembobiné plus tôt dans la session, Claude Code redémarre à partir du point rembobiné, et non à partir de la transcription plus longue enregistrée sur le disque. Par exemple, si vous avez rembobiné au-delà de vos trois derniers messages, la session relancée s'ouvre sans eux
  * Si vous avez rembobiné avant votre premier message, Claude Code redémarre avec une conversation vide
* Votre [mode de permission](/docs/fr/permission-modes) et [niveau d'effort](/docs/fr/model-config#adjust-effort-level)
* Le modèle que vous avez sélectionné en dernier avec [`/model`](/docs/fr/model-config#setting-your-model)
* Les règles que vous avez transmises avec [`--allowed-tools` ou `--disallowed-tools`](/docs/fr/cli-reference#cli-flags), et vos drapeaux `--agent`, `--agents`, `--append-system-prompt`, et `--system-prompt-snapshot`

Claude Code refuse de redémarrer si la session a une restriction qu'il ne peut pas transmettre au processus redémarré. Les restrictions qu'il ne peut pas transmettre incluent :

* Les drapeaux de lancement tels qu'un remplacement [`--system-prompt`](/docs/fr/cli-reference#cli-flags), une liste d'autorisation [`--tools`](/docs/fr/cli-reference#cli-flags), ou [`--setting-sources`](/docs/fr/cli-reference#cli-flags)
* Les règles de refus ou de demande qu'une [mise à jour de permission de hook ou SDK](/docs/fr/hooks#permission-update-entries) a ajoutées pour cette session uniquement

Dans ce cas, Claude Code affiche [`Cannot switch renderers in this session`](/docs/fr/errors#cannot-switch-renderers-in-this-session) avec les raisons. Il ne bascule pas et n'enregistre rien.

Vous pouvez également définir la variable d'environnement `CLAUDE_CODE_NO_FLICKER` avant de démarrer Claude Code :

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Pour savoir comment le [paramètre `tui`](/docs/fr/settings-reference#tui) et la variable se combinent lorsque les deux sont définis, consultez l'entrée du paramètre. Après un [démarrage en plein écran échoué](#fullscreen-renderer-didnt-finish-starting), Claude Code honore toujours la variable mais pas le paramètre. La commande `/tui` efface `CLAUDE_CODE_NO_FLICKER` du processus relancé afin que le paramètre qu'elle écrit prenne effet.

<h3 id="fullscreen-by-default">
  Plein écran par défaut
</h3>

Les [sessions d'arrière-plan](/docs/fr/agent-view) attachées se rendent en plein écran, et les autres sessions en [mode lecteur d'écran](/docs/fr/accessibility) utilisent le moteur de rendu classique. Sinon, Claude Code vous démarre dans le moteur de rendu de la première ligne de ce tableau qui correspond à votre configuration :

| Votre situation                                                                                                                                                                                                                | Moteur de rendu dans lequel vous démarrez |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------- |
| Vous avez défini [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](/docs/fr/env-vars) ou `CLAUDE_CODE_NO_FLICKER=0`                                                                                                                        | Classique                                 |
| Vous avez défini `CLAUDE_CODE_NO_FLICKER=1`                                                                                                                                                                                    | Plein écran                               |
| Claude Code [a désactivé le plein écran après un démarrage en plein écran échoué](#fullscreen-renderer-didnt-finish-starting) sur cette machine                                                                                | Classique                                 |
| Vous êtes en mode d'intégration [`tmux -CC`](#use-with-tmux) d'iTerm2, ou vous êtes connecté via SSH à Claude Code s'exécutant sur Windows                                                                                     | Classique                                 |
| Vous avez enregistré un [paramètre `tui`](/docs/fr/settings-reference#tui)                                                                                                                                                          | Le moteur de rendu que le paramètre nomme |
| Votre session ne [récupère pas les drapeaux de fonctionnalités auprès d'Anthropic](/docs/fr/env-vars#features-that-need-feature-flag-fetching), et Claude Code a cessé d'offrir la boîte de dialogue de démarrage sur cette machine | Classique                                 |
| Votre session ne récupère pas les drapeaux de fonctionnalités auprès d'Anthropic, et le premier lancement de Claude Code de cette machine a exécuté v2.1.239 ou ultérieur                                                      | Plein écran                               |
| Votre session récupère les drapeaux de fonctionnalités auprès d'Anthropic, et vous avez utilisé Claude Code pour la première fois le 6 mai 2026 ou après                                                                       | Plein écran                               |
| N'importe quoi d'autre                                                                                                                                                                                                         | Classique                                 |

Les sessions qui ne récupèrent pas les drapeaux de fonctionnalités incluent celles via [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou [Microsoft Foundry](/docs/fr/microsoft-foundry), et celles avec la télémétrie désactivée.

Si vous démarrez dans le moteur de rendu classique et n'avez pas enregistré de paramètre `tui`, Claude Code peut ouvrir une boîte de dialogue au démarrage offrant le basculement :

* Si vous acceptez, Claude Code redémarre de la même manière que `/tui fullscreen` le fait, en conservant le même état de session, et enregistre le paramètre une fois que la session relancée a [démarré avec succès](#fullscreen-renderer-didnt-finish-starting).
* Si vous choisissez **Pas maintenant**, Claude Code n'offre plus sur cette machine.
* Claude Code cesse d'offrir après avoir affiché la boîte de dialogue sur trois lancements, répondus ou non.

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

Le rendu en plein écran capture les événements de la souris et les traite dans Claude Code :

* **Cliquez dans le champ de saisie du message** pour positionner votre curseur n'importe où dans le texte que vous tapez.
* **Cliquez sur une suggestion dans la liste de commandes `/` ou de fichiers `@`** pour l'accepter. Le survol met en évidence la ligne sous votre curseur.
* **Cliquez sur une option dans un menu de sélection** pour la choisir. Cela couvre les invites de permission, `/model`, `/config` et autres dialogues qui affichent une liste d'options. Le survol affiche un pointeur sur la ligne sous votre curseur.
* **Cliquez sur une option dans un menu de sélection multiple** pour la basculer, et cliquez sur le bouton de soumission pour confirmer vos choix. Cliquer sur une ligne de texte libre, comme la ligne `Other` dans une question à choix multiples, met le focus sur son champ de saisie pour que vous puissiez taper une réponse. Nécessite Claude Code v2.1.208 ou version ultérieure.
* **Cliquez sur une valeur de paramètre dans le panneau `/config`** pour la modifier, et faites défiler la liste des paramètres avec la molette de la souris. Nécessite Claude Code v2.1.271 ou version ultérieure.
* **Faites défiler un menu de sélection ou de sélection multiple avec la molette de la souris** lorsqu'il a plus d'options qu'il n'en affiche à la fois, comme la liste `/model` dans une fenêtre de terminal courte. La molette fait défiler la liste tandis que le pointeur se trouve sur ses options. Nécessite Claude Code v2.1.280 ou version ultérieure.
* **Cliquez sur un résultat d'outil réduit** pour le développer et voir la sortie complète. Cliquez à nouveau pour le réduire. L'appel d'outil et son résultat se développent ensemble. Seuls les messages qui ont plus à afficher sont cliquables.
  * Cliquer développe également la sortie d'une commande shell `!`, qu'il s'agisse d'un résultat tronqué plus ancien ou de la ligne de progression en direct pendant l'exécution de la commande. Nécessite Claude Code v2.1.257 ou version ultérieure.
* **Maintenez `Cmd` sur macOS, ou `Ctrl` sur Linux et Windows, et cliquez sur une URL ou un chemin de fichier** pour l'ouvrir. Les URLs simples `http://` et `https://` s'ouvrent dans votre navigateur, et les chemins de fichiers dans la sortie d'outil, comme ceux imprimés après une opération Edit ou Write, s'ouvrent dans votre application par défaut. Un simple clic sans le modificateur n'ouvre pas les liens, ce qui correspond au comportement du terminal natif.
  * Claude Code affiche un chemin réseau (UNC), tel que `\\server\share\file.ts`, en tant que texte brut sans lien, car l'ouverture d'un chemin réseau peut envoyer vos identifiants Windows à l'hôte qu'il désigne.
  * Certains terminaux macOS transmettent `Cmd`+clic à l'application en cours d'exécution au lieu d'ouvrir le lien eux-mêmes, et le protocole de souris du terminal n'a aucun moyen d'encoder la clé `Cmd`, donc Claude Code reçoit un simple clic. Dans Ghostty, et dans Warp sur macOS, Claude Code détecte cela et permet à un simple clic sur un lien de l'ouvrir, et maintenir `Cmd` fonctionne toujours.
  * Dans le terminal intégré VS Code et les terminaux basés sur xterm.js similaires, Claude Code s'en remet au gestionnaire de liens du terminal, qui utilise le même geste.
* **Cliquez et glissez** pour sélectionner du texte n'importe où dans la conversation. Un double-clic sélectionne un mot, en respectant les limites de mots d'iTerm2 pour qu'un chemin de fichier soit sélectionné comme une unité. Un double-clic sur une URL sélectionne l'URL entière, y compris le schéma. Un triple-clic sélectionne la ligne.
* **Faites défiler avec la molette de la souris** pour vous déplacer dans la conversation.

Le texte sélectionné est copié automatiquement dans votre presse-papiers au relâchement de la souris. Pour désactiver cela, basculez Copier à la sélection dans `/config`.

Avec Copier à la sélection désactivé, appuyez sur `Ctrl+Maj+c` pour copier manuellement. Sur les terminaux qui prennent en charge le protocole clavier kitty, tels que kitty, WezTerm, Ghostty et iTerm2, `Cmd+c` fonctionne également. Si vous avez une sélection active, `Ctrl+c` copie au lieu d'annuler.

Avec une sélection active, maintenez `Maj` et appuyez sur les touches fléchées pour l'étendre à partir du clavier. `Maj+↑` et `Maj+↓` font défiler la fenêtre d'affichage lorsque la sélection atteint le bord supérieur ou inférieur. `Maj+Début` et `Maj+Fin` étendent jusqu'au début ou à la fin de la ligne actuelle.

Dans la vue de message normale, ce qui se passe avec une sélection active dépend de la touche que vous appuyez :

* **`Échap`** : Claude Code effectue l'action habituelle de la touche, comme l'interruption de la réponse en cours d'exécution ou la fermeture d'un dialogue ouvert, et la sélection reste mise en évidence.
* **`PgPrécédente`, `PgSuivante`, `Ctrl+Début`, `Ctrl+Fin`, ou `Maj`, `Alt` ou `Option`, ou `Cmd`, `Win`, ou `Super` avec une touche fléchée, `Début`, ou `Fin`** : la sélection reste.
* **N'importe quelle autre touche, y compris les touches fléchées simples, `Entrée` et les caractères tapés** : Claude Code efface la sélection.
* **Une touche liée à [`selection:clear`](/docs/fr/keybindings#scroll-actions)** : Claude Code efface la sélection, même lorsque la touche est `Échap` ou une autre touche qui la conserve autrement. L'action n'a pas de liaison par défaut.

En [mode transcription](#search-and-review-the-conversation), les touches de navigation et de recherche listées là conservent également la sélection.

<h2 id="scroll-the-conversation">
  Faire défiler la conversation
</h2>

Le rendu en plein écran gère le défilement dans l'application. Utilisez ces raccourcis pour naviguer :

| Raccourci            | Action                                                     |
| :------------------- | :--------------------------------------------------------- |
| `PgUp` / `PgDn`      | Faire défiler vers le haut ou vers le bas d'une demi-écran |
| `Ctrl+Home`          | Aller au début de la conversation                          |
| `Ctrl+End`           | Aller au dernier message et réactiver le suivi automatique |
| Molette de la souris | Faire défiler quelques lignes à la fois                    |

Vous pouvez faire défiler jusqu'au début de la session même après [compaction](/docs/fr/context-window#what-survives-compaction). Claude continue de fonctionner à partir du résumé de compaction, mais Claude Code conserve tous les messages antérieurs dans le défilement en plein écran à travers les compactions répétées.

Sur les claviers sans touches dédiées `PgUp`, `PgDn`, `Home` ou `End`, comme les claviers MacBook, maintenez `Fn` avec les touches fléchées : `Fn+↑` envoie `PgUp`, `Fn+↓` envoie `PgDn`, `Fn+←` envoie `Home`, et `Fn+→` envoie `End`. `Ctrl+Fn+→` n'atteint pas Claude Code sur macOS, donc un clavier MacBook n'a pas de combinaison de touches fonctionnelle pour sauter vers le bas par défaut. À la place, utilisez l'une de ces options :

* Cliquez sur le [bouton sauter vers le bas](#auto-follow).
* Faites défiler vers le bas avec la molette de la souris pour reprendre le suivi.
* Réaffectez `scroll:bottom` à une combinaison de touches que votre clavier peut envoyer.

Ces actions sont réaffectables. Consultez [Actions de défilement](/docs/fr/keybindings#scroll-actions) pour la liste complète des noms d'actions, y compris les variantes de demi-page et de page complète qui n'ont pas de liaison par défaut.

Pendant que vous êtes défilé vers le haut, une ligne d'en-tête atténuée en haut de la conversation affiche l'invite la plus récente qui a défilé au-dessus de la vue. Cliquez sur la ligne pour sauter à cette invite.

<h3 id="auto-follow">
  Suivi automatique
</h3>

Le défilement vers le haut met en pause le suivi automatique afin que la nouvelle sortie ne vous ramène pas vers le bas. Un bouton `Sauter vers le bas` flotte sur le bord inférieur de la transcription lorsque vous êtes défilé vers le haut, et affiche un décompte tel que `3 nouveaux messages` lorsqu'une nouvelle sortie arrive. Cliquez dessus, appuyez sur `Ctrl+End`, ou faites défiler vers le bas pour reprendre le suivi.

Lorsque le suivi automatique est en pause, la vue reste également où vous l'avez défilée lorsqu'une réponse termine le streaming.

L'indice clavier du bouton reflète ce que votre clavier peut envoyer. Sur macOS, il suggère de cliquer, ou `Fn+↓` pour faire défiler, car `Ctrl+End` n'atteint pas Claude Code depuis un clavier Mac. Réaffectez [`scroll:bottom`](/docs/fr/keybindings#scroll-actions) et le bouton affiche votre combinaison de touches sur chaque plateforme.

Sur un terminal trop étroit pour l'étiquette complète, le bouton raccourcit l'indice au lieu de l'enrouler sur la ligne de transcription en dessous.

Pour désactiver entièrement le suivi automatique afin que la vue reste où vous la laissez, ouvrez `/config` et définissez Auto-scroll sur off. Avec le défilement automatique désactivé, la vue ne saute jamais vers le bas d'elle-même. Les invites de permission et autres dialogues qui nécessitent une réponse défilent toujours dans la vue indépendamment de ce paramètre.

<h3 id="mouse-wheel-scrolling">
  Défilement à la molette de la souris
</h3>

Le défilement à la molette de la souris nécessite que votre terminal transfère les événements de la souris à Claude Code. La plupart des terminaux le font chaque fois qu'une application le demande. iTerm2 en fait un paramètre par profil : si la molette ne fait rien mais que `PgUp` et `PgDn` fonctionnent, ouvrez Paramètres → Profils → Terminal et activez Enable mouse reporting. Le même paramètre est également requis pour que le clic pour développer et la sélection de texte fonctionnent.

Si le défilement à la molette de la souris semble lent, votre terminal peut envoyer un événement de défilement par cran physique sans multiplicateur. Certains terminaux, comme Ghostty et iTerm2 avec défilement plus rapide activé, amplifient déjà les événements de molette. D'autres, y compris le terminal intégré VS Code, envoient exactement un événement par cran. Claude Code ne peut pas détecter lequel.

Définissez `CLAUDE_CODE_SCROLL_SPEED` pour multiplier la distance de défilement de base :

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Une valeur de `3` correspond à la valeur par défaut dans `vim` et les applications similaires. Le paramètre accepte toute valeur positive jusqu'à 20, y compris les valeurs fractionnaires inférieures à 1 telles que `0.25` pour ralentir le défilement du trackpad et de la molette accélérés dans les terminaux qui amplifient déjà les événements de molette.

Pour ajuster la vitesse de défilement de manière interactive, exécutez `/scroll-speed`. Le dialogue affiche une règle que vous pouvez faire défiler pendant qu'il est ouvert afin que vous puissiez sentir le changement immédiatement. Appuyez sur `←` et `→` pour ajuster la vitesse, `r` pour réinitialiser à la valeur par défaut détectée automatiquement, et `Entrée` pour enregistrer. Le dialogue augmente par nombres entiers jusqu'à 10, et sur les terminaux qui supportent un contrôle plus fin, il offre également des quarts de pas jusqu'à 0,25.

La commande écrit la même valeur que la variable d'environnement `CLAUDE_CODE_SCROLL_SPEED` définit, persistée dans `~/.claude/settings.json`. Le maximum du dialogue est 10 : si vous définissez une valeur plus élevée via la variable d'environnement, le dialogue affiche 10, et l'enregistrement à partir du dialogue persiste 10. La commande n'est pas disponible dans le terminal IDE JetBrains.

Séparément de la vitesse de base, Claude Code accélère la vitesse de défilement lorsque vous tournez la molette rapidement, donc une rotation rapide couvre plus de distance que le même nombre de crans lents. Pour désactiver l'accélération et maintenir une vitesse constante par cran, définissez `wheelScrollAccelerationEnabled` sur `false` dans [`settings.json`](/docs/fr/settings-reference#all-settings). Ce paramètre nécessite Claude Code v2.1.174 ou version ultérieure.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Défilement dans le terminal IDE JetBrains
</h3>

Dans le terminal IDE JetBrains, Claude Code applique sa propre gestion du défilement et ignore `CLAUDE_CODE_SCROLL_SPEED`. Le terminal envoie des événements de défilement à un taux beaucoup plus élevé que les autres émulateurs, donc un multiplicateur accordé ailleurs dépasse ici.

En 2025.2, le terminal a également des bogues de défilement à la molette qui produisent des touches fléchées parasites et des événements de mauvaise direction. Claude Code les détecte à l'exécution et les atténue automatiquement, donc le défilement du trackpad et de la molette de la souris fonctionnent sans configuration. Pour la meilleure expérience de défilement, mettez à niveau vers 2025.3 ou version ultérieure. Claude Code affiche un indice la première fois que vous faites défiler si le bogue est détecté.

<h2 id="search-and-review-the-conversation">
  Rechercher et examiner la conversation
</h2>

`Ctrl+o` bascule entre le mode invite normal et le mode transcription.

Pour une vue plus épurée qui affiche uniquement votre dernier invite, un résumé d'une ligne des appels d'outils avec les statistiques de modification, et la réponse finale, exécutez `/focus`. Le paramètre persiste entre les sessions. Exécutez `/focus` à nouveau pour le désactiver.

Le mode transcription gagne la navigation et la recherche de style `less` :

| Touche                                | Action                                                                                                                                              |
| :------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                   | Ouvrir la recherche. Tapez pour trouver des correspondances, `Entrée` pour accepter, `Échap` pour annuler et restaurer votre position de défilement |
| `n` / `N`                             | Accéder à la correspondance suivante ou précédente. Fonctionne après avoir fermé la barre de recherche                                              |
| `j` / `k` ou `↑` / `↓`                | Faire défiler une ligne                                                                                                                             |
| `g` / `G` ou `Accueil` / `Fin`        | Accéder au début ou à la fin                                                                                                                        |
| `{` / `}`                             | Accéder à l'invite précédente ou suivante                                                                                                           |
| `Ctrl+u` / `Ctrl+d`                   | Faire défiler une demi-page                                                                                                                         |
| `Ctrl+b` / `Ctrl+f` ou `Espace` / `b` | Faire défiler une page complète                                                                                                                     |
| `Ctrl+o`, `Échap`, ou `q`             | Quitter le mode transcription et revenir à l'invite                                                                                                 |

La fonction `Cmd+f` de votre terminal et la recherche tmux ne voient pas la conversation car elle se trouve dans le tampon d'écran alternatif, pas dans le défilement natif. Pour renvoyer le contenu à votre terminal, appuyez sur `Ctrl+o` pour entrer d'abord en mode transcription, puis :

* **`[`** : écrit la conversation complète dans le tampon de défilement natif de votre terminal, avec tous les résultats d'outils développés. La conversation est maintenant du texte ordinaire dans votre terminal, donc `Cmd+f`, le mode copie tmux et tout autre outil natif peuvent la rechercher ou la sélectionner. Les sessions longues peuvent faire une pause un moment pendant que cela se produit. Cela dure jusqu'à ce que vous quittiez le mode transcription avec `Échap` ou `q`, ce qui vous ramène au rendu plein écran. Le prochain `Ctrl+o` recommence à zéro.
* **`v`** : écrit la conversation dans un fichier temporaire et l'ouvre dans `$VISUAL` ou `$EDITOR`.

<h2 id="watch-your-changes-in-the-diff-panel">
  Regardez vos modifications dans le panneau de diff
</h2>

En rendu plein écran, [`/diff`](/docs/fr/interactive-mode#review-changes-with-%2Fdiff) ouvre un panneau à côté de la conversation plutôt qu'un visualiseur que vous devez fermer, afin que vous puissiez regarder les modifications s'accumuler pendant que Claude travaille. Dans un terminal large, le panneau peut également s'ouvrir automatiquement une fois que Claude commence à modifier des fichiers. [Panneau de diff](/docs/fr/interactive-mode#diff-panel) couvre ce qu'il affiche, comment le garder fermé et comment modifier ce qu'il compare.

<h2 id="clear-the-conversation">
  Effacer la conversation
</h2>

Exécutez `/clear` pour démarrer une nouvelle conversation.

Si l'affichage semble garni ou partiellement vide, appuyez sur `Ctrl+L` pour redessiner l'écran. Le redessinage conserve la conversation et votre saisie en place.

`Cmd+K` fait la même chose que `Ctrl+L` lorsque votre terminal le transmet à Claude Code. iTerm2 et Terminal.app gèrent `Cmd+K` eux-mêmes et effacent leur propre écran, et Claude Code détecte l'écran effacé et repeint la conversation. Avant v2.1.280, à partir de v2.1.260, appuyer sur `Ctrl+L` ou `Cmd+K` où il atteint Claude Code effaçait l'écran dans le rendu en plein écran. Avant v2.1.238, appuyer sur `Ctrl+L` deux fois en deux secondes exécutait `/clear`.

<h2 id="use-with-tmux">
  Utilisation avec tmux
</h2>

Le rendu en plein écran fonctionne dans tmux, avec trois réserves.

Le défilement à la molette de la souris nécessite le mode souris de tmux. Si votre `~/.tmux.conf` ne l'active pas déjà, ajoutez cette ligne et rechargez votre configuration :

```bash theme={null}
set -g mouse on
```

Sans le mode souris, les événements de molette vont à tmux au lieu de Claude Code. Le défilement au clavier avec `PgUp` et `PgDn` fonctionne de toute façon. Claude Code affiche un indice unique au démarrage s'il détecte tmux avec le mode souris désactivé.

Le rendu en plein écran est incompatible avec le mode d'intégration tmux d'iTerm2, qui est le mode dans lequel vous entrez avec `tmux -CC`. En mode intégration, iTerm2 rend chaque volet tmux comme une division native plutôt que de laisser tmux dessiner sur le terminal. Le tampon d'écran alternatif et le suivi de la souris ne fonctionnent pas correctement là : la molette de la souris ne fait rien, et le double-clic peut corrompre l'état du terminal. N'activez pas le rendu en plein écran dans les sessions `tmux -CC`. Le tmux régulier dans iTerm2, sans `-CC`, fonctionne bien.

Les versions de tmux jusqu'à la série 3.6 n'implémentent pas la sortie synchronisée, donc sous ces versions vous pouvez voir plus de scintillement lors des redessins que lors de l'exécution de Claude Code directement dans votre terminal. Claude Code sonde le terminal pour la prise en charge de la sortie synchronisée au démarrage et l'utilise quand le terminal le signale. Si vous voyez du scintillement sous tmux, mettez à jour vers le dernier tmux ou exécutez Claude Code dans son propre onglet de terminal en dehors de tmux.

<h2 id="keep-native-text-selection">
  Conserver la sélection de texte native
</h2>

La capture de souris est le point de friction le plus courant, en particulier sur SSH ou à l'intérieur de tmux. Lorsque Claude Code capture les événements de souris, la copie native au survol de votre terminal cesse de fonctionner. La sélection que vous effectuez avec un clic et un glissement existe à l'intérieur de Claude Code, pas dans le tampon de sélection de votre terminal, donc le mode copie de tmux, les indices de Kitty et les outils similaires ne la voient pas.

Claude Code écrit la sélection dans le presse-papiers de votre système, et le chemin qu'il utilise dépend de votre configuration. Sur une session locale, il exécute un outil de presse-papiers natif :

* **macOS** : `pbcopy`
* **Linux** : `wl-copy` sur Wayland, ou `xclip` ou `xsel` sur X11, selon celui qui est installé. Claude Code écrit à la fois le presse-papiers et la sélection PRIMARY, donc le collage au clic du milieu fonctionne.
* **Windows et WSL** : PowerShell `Set-Clipboard`

À l'intérieur de tmux, il écrit également dans le tampon de collage de tmux. Sur SSH, il revient aux séquences d'échappement OSC 52. À l'intérieur de GNU screen, Claude Code copie également les longues sélections dans le presse-papiers. Avant la v2.1.219, si vous copiiez une sélection plus longue qu'environ 570 caractères, GNU screen imprimait du texte en base64 dans la fenêtre à la place. Claude Code affiche un toast après chaque copie vous indiquant quel chemin il a utilisé.

Certains terminaux bloquent OSC 52 par défaut. iTerm2 le bloque jusqu'à ce que vous activiez Paramètres → Général → Sélection → Les applications du terminal peuvent accéder au presse-papiers ; l'exécution de [`/terminal-setup`](/docs/fr/terminal-config) dans iTerm2 active cela pour vous.

Pour une sélection native ponctuelle, la touche à utiliser dépend de votre terminal :

* **Terminal.app** : `Fn`
* **iTerm2** : `Option`
* **VS Code, Cursor et Devin Desktop** : `Shift`, ou `Option` sur macOS avec le paramètre `terminal.integrated.macOptionClickForcesSelection` activé
* **La plupart des autres terminaux** : `Shift`

Maintenez cette touche enfoncée pendant que vous cliquez et glissez. Votre terminal gère la sélection lui-même au lieu de la transmettre à Claude Code, donc les raccourcis de copie comme `Cmd+C` fonctionnent sur ce que vous sélectionnez. Claude Code affiche également la touche correcte dans son indice à l'écran.

Sur SSH ou à l'intérieur de tmux, Claude Code ne peut pas toujours détecter le terminal auquel vous vous connectez, donc l'indice répertorie les touches candidates à la place.

Si vous comptez sur la sélection native tout le temps, définissez `CLAUDE_CODE_DISABLE_MOUSE=1` pour refuser la capture de souris tout en conservant le rendu sans scintillement et la mémoire plate :

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Avec la capture de souris désactivée, le défilement au clavier avec `PgUp`, `PgDn`, `Ctrl+Home` et `Ctrl+End` fonctionne toujours, et votre terminal gère la sélection nativement. Vous perdez le clic pour positionner le curseur, le clic pour développer la sortie de l'outil, le clic sur les URL et le défilement à la roulette à l'intérieur de Claude Code.

Pour conserver le défilement à la roulette mais désactiver la gestion des clics, des glissements et des survols, définissez plutôt `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`. Nécessite Claude Code v2.1.195 ou version ultérieure. `CLAUDE_CODE_DISABLE_MOUSE` a la priorité lorsque les deux variables sont définies.

Avec les clics désactivés, Claude Code capture toujours la souris, donc la roulette et le pavé tactile font défiler la conversation mais les clics gauches ne font rien à l'intérieur de Claude Code. Vous devez toujours maintenir la touche de votre terminal enfoncée pour la sélection native au clic et au glissement. Le clic droit et le collage au clic du milieu continuent de fonctionner sur les terminaux qui les supportent.

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

Sur Windows, Claude Code active déjà automatiquement le repeint complet pour les sessions en arrière-plan et la [vue agent](/docs/fr/agent-view), vous n'avez donc besoin de définir la variable que pour une session interactive en plein écran que vous avez lancée directement.

<h3 id="fullscreen-renderer-didnt-finish-starting">
  `Claude Code's fullscreen renderer didn't finish starting last time` apparaît au démarrage
</h3>

Si une session en plein écran sur cette machine s'arrête avant d'avoir démarré avec succès, Claude Code démarre votre session suivante dans le rendu classique et affiche l'une de deux lignes. Une session a démarré avec succès une fois qu'elle a dessiné sa première image et qu'elle est restée active pendant 10 secondes ou que vous l'avez terminée avec `/exit`, Ctrl+C ou Ctrl+D. La ligne que vous voyez vous indique ce que Claude Code fait après cette session :

* Après un premier démarrage échoué, vous voyez `Claude Code's fullscreen renderer didn't finish starting last time on this machine`. Claude Code essaie à nouveau le rendu en plein écran dans la session suivante que vous démarrez
* Après deux démarrages échoués, vous voyez `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`. Claude Code continue à utiliser le rendu classique jusqu'à ce que vous mettiez à jour Claude Code ou exécutiez `/tui fullscreen`, et n'affiche rien dans ces sessions ultérieures

Pour confirmer qu'un démarrage échoué est la raison pour laquelle vous êtes dans le rendu classique, exécutez `/tui` sans argument. Tant qu'un démarrage échoué est la raison, la ligne `Current renderer` l'indique.

Pour conserver le rendu classique, exécutez `/tui default`, qui enregistre le paramètre `tui` sans relancer. Pour essayer à nouveau le rendu en plein écran, exécutez `/tui fullscreen`. Si cette session ne démarre pas non plus, [signalez le problème](#research-preview).

Avant la v2.1.236, Claude Code continuait à démarrer les sessions en rendu plein écran après un démarrage échoué.

<h4 id="how-claude-code-counts-failed-starts">
  Comment Claude Code compte les démarrages échoués
</h4>

* Sessions qui comptent : uniquement les sessions qui ont démarré en rendu plein écran parce que votre paramètre `tui` le dit, parce que vous avez accepté la [boîte de dialogue de démarrage](#fullscreen-by-default), ou parce que Claude Code vous démarre en plein écran par défaut
* `CLAUDE_CODE_NO_FLICKER=1` : si vous le définissez, Claude Code rend cette session en plein écran même après un démarrage échoué, et ne le compte pas
* Réinitialisation du comptage : Claude Code compte les démarrages échoués par version de Claude Code, et un démarrage plein écran réussi réinitialise le comptage
* Boîte de dialogue de démarrage : si vous avez accepté la boîte de dialogue et que la session relancée s'est arrêtée, Claude Code n'affiche aucune ligne et n'affiche plus la boîte de dialogue sur cette version de Claude Code

<h2 id="research-preview">
  Aperçu de recherche
</h2>

Le rendu en plein écran est une fonctionnalité en aperçu de recherche. Il a été testé sur les émulateurs de terminal courants, mais vous pouvez rencontrer des problèmes de rendu sur les terminaux moins courants ou les configurations inhabituelles.

Si vous rencontrez un problème, exécutez `/feedback` dans Claude Code pour le signaler, ou ouvrez un problème sur le [référentiel GitHub claude-code](https://github.com/anthropics/claude-code/issues). Incluez le nom et la version de votre émulateur de terminal.

Pour désactiver le rendu en plein écran, exécutez `/tui default`, ou désactivez `CLAUDE_CODE_NO_FLICKER` si vous l'avez activé de cette façon. Lorsque vous revenez avec `/tui default`, Claude Code peut d'abord afficher une invite de rétroaction facultative vous demandant ce qui vous a fait changer. Tapez une raison et appuyez sur `Entrée` pour l'envoyer, ou appuyez sur `Échap` pour ignorer. L'interface de ligne de commande relance le rendu classique de toute façon. Pour forcer le rendu classique indépendamment du paramètre `tui` enregistré, définissez `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Le rendu classique conserve la conversation dans le défilement natif de votre terminal afin que `Cmd+f` et le mode copie tmux fonctionnent comme d'habitude.

Les sessions en arrière-plan ouvertes à partir de la [vue agent](/docs/fr/agent-view) ou `claude attach` utilisent toujours le rendu en plein écran. Le terminal d'attachement entre dans le tampon d'écran alternatif pour afficher la session, et le rendu classique n'a pas de défilement ou de gestion de la souris là-bas, donc le paramètre `tui` et `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` ne s'y appliquent pas.
