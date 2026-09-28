> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser Claude Code avec un lecteur d'écran

> Configurez Claude Code pour les lecteurs d'écran tels que VoiceOver et NVDA, ainsi que les paramètres pour les loupes d'écran, le mouvement réduit et les thèmes adaptés aux daltoniens.

Claude Code dispose d'un mode lecteur d'écran qui remplace son interface de terminal visuelle par du texte simple et linéaire. Au lieu de boîtes, d'animations de progression et de redessinages sur place, Claude Code imprime des lignes étiquetées qu'un lecteur d'écran tel que VoiceOver ou NVDA lit dans l'ordre. Vous pouvez maintenir une conversation complète, approuver les autorisations d'outils et examiner la sortie de bout en bout.

Le mode lecteur d'écran est optionnel. Si vous utilisez une loupe d'écran, un mouvement réduit ou un thème adapté aux daltoniens au lieu d'un lecteur d'écran, définissez `CLAUDE_CODE_ACCESSIBILITY`, `prefersReducedMotion` ou `theme` à partir du tableau [Paramètres d'accessibilité](#accessibility-settings). Le mode lecteur d'écran adapte uniquement l'interface du terminal, vous n'en avez donc pas besoin dans le panneau de chat de l'extension VS Code. Sur Claude Code v2.1.236 ou version ultérieure, l'extension [annonce l'activité de conversation à votre lecteur d'écran](/docs/fr/vs-code#use-a-screen-reader) sans aucun paramètre.

<h2 id="turn-on-screen-reader-mode">
  Activer le mode lecteur d'écran
</h2>

Choisissez la méthode qui correspond à la fréquence à laquelle vous utilisez un lecteur d'écran :

* Pour une session : exécutez `claude --ax-screen-reader`.
* Pour les sessions démarrées à partir d'un shell : définissez la variable d'environnement `CLAUDE_AX_SCREEN_READER` sur `1`. Dans Bash ou Zsh, exécutez `export CLAUDE_AX_SCREEN_READER=1`. Dans PowerShell, exécutez `$env:CLAUDE_AX_SCREEN_READER = "1"`. Ajoutez cette ligne à votre profil shell pour la conserver pour les shells futurs.
* Pour chaque session sur la machine : ajoutez `"axScreenReader": true` à votre [fichier de paramètres](/docs/fr/settings) utilisateur. Le paramètre s'applique dans n'importe quel terminal, y compris le terminal intégré VS Code.

Si vous combinez les méthodes, Claude Code applique l'indicateur [`--ax-screen-reader`](/docs/fr/cli-reference#cli-flags) sur la variable d'environnement [`CLAUDE_AX_SCREEN_READER`](/docs/fr/env-vars#variables), et la variable sur le paramètre [`axScreenReader`](/docs/fr/settings-reference#axscreenreader).

Si vous utilisez Claude Code via SSH, définissez la variable d'environnement ou le paramètre sur la machine distante où Claude Code s'exécute.

La première ligne que Claude Code imprime confirme le mode : `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, ou `[Screen Reader Mode: on via settings]`.

<h2 id="turn-off-screen-reader-mode">
  Désactiver le mode lecteur d'écran
</h2>

Inversez la méthode qui a activé le mode : démarrez sans l'indicateur, désactivez la variable d'environnement ou définissez `axScreenReader` sur `false`. Si vous définissez `CLAUDE_AX_SCREEN_READER` sur `0`, Claude Code maintient le mode désactivé même lorsque le paramètre est `true`.

<h2 id="accessibility-settings">
  Paramètres d'accessibilité
</h2>

Le tableau énumère chaque option d'accessibilité, que vous la définissiez comme un drapeau, une variable d'environnement ou un paramètre, et ce qu'elle change.

| Option                                                                  | Type                     | Ce qu'elle change                                                                                                                                                                                                                                                                                           |
| :---------------------------------------------------------------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`--ax-screen-reader`](/docs/fr/cli-reference#cli-flags)                     | Drapeau                  | Mode lecteur d'écran pour une session.                                                                                                                                                                                                                                                                      |
| [`CLAUDE_AX_SCREEN_READER`](/docs/fr/env-vars#variables)                     | Variable d'environnement | Mode lecteur d'écran pour les sessions démarrées à partir du shell où vous l'avez définie.                                                                                                                                                                                                                  |
| [`axScreenReader`](/docs/fr/settings-reference#axscreenreader)               | Paramètre                | Mode lecteur d'écran pour chaque session lorsque `true`.                                                                                                                                                                                                                                                    |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/fr/env-vars#variables)                  | Variable d'environnement | Durée pendant laquelle Claude Code attend après la ligne de confirmation avant de dessiner la première invite en mode lecteur d'écran. Nécessite Claude Code v2.1.217 ou version ultérieure.                                                                                                                |
| [`CLAUDE_AX_PREPARK_MS`](/docs/fr/env-vars#variables)                        | Variable d'environnement | Durée pendant laquelle Claude Code attend, avec le curseur au début de la ligne, avant d'écrire une ligne nouvelle ou modifiée en mode lecteur d'écran. Nécessite Claude Code v2.1.233 ou version ultérieure.                                                                                               |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/fr/env-vars#variables)                   | Variable d'environnement | Un curseur de terminal qui reste visible pour les loupes d'écran telles que macOS Zoom lorsque vous le définissez sur `1`. Le curseur suit le curseur d'entrée et, sur Claude Code v2.1.218 ou version ultérieure, la ligne en surbrillance dans les menus et les panneaux tels que `/config` et `/plugin`. |
| [`prefersReducedMotion`](/docs/fr/settings-reference#prefersreducedmotion)   | Paramètre                | Réduction ou absence de barres de chargement, scintillement et autres animations lorsque `true`.                                                                                                                                                                                                            |
| [`theme`](/docs/fr/settings-reference#theme)                                 | Paramètre                | Les couleurs de l'interface, y compris les thèmes `dark-daltonized` et `light-daltonized` adaptés aux daltoniens. Vous pouvez également en choisir un avec [`/theme`](/docs/fr/commands#all-commands).                                                                                                           |
| [`preferredNotifChannel`](/docs/fr/settings-reference#preferrednotifchannel) | Paramètre                | Avec la valeur `"terminal_bell"`, une cloche de terminal en dehors du mode lecteur d'écran lorsque Claude vous attend.                                                                                                                                                                                      |

<h2 id="what-your-screen-reader-hears">
  Ce que votre lecteur d'écran entend
</h2>

En mode lecteur d'écran, Claude Code écrit du texte plat :

* Pas de caractères de dessin de boîte pour le chrome de l'interface
* Pas d'indices basés sur la couleur uniquement
* Pas de redessins du contenu qui n'a pas changé. Les barres de progression s'affichent sous forme de texte statique
* Les tableaux dans les réponses de Claude se lisent comme des phrases `En-tête : valeur` au lieu d'une grille de caractères de boîte

Claude Code laisse tout ce qu'il imprime dans l'historique de votre terminal, afin que vous puissiez relire les tours précédents avec les commandes de révision de votre lecteur d'écran ou la recherche de votre terminal. Claude Code ignore le paramètre [`tui`](/docs/fr/settings-reference#tui) en mode lecteur d'écran. À part les sessions d'arrière-plan attachées listées sous [Limitations connues](#known-limitations), il imprime du texte défilant au lieu du [rendu en plein écran](/docs/fr/fullscreen).

Claude Code attend également à deux points pour que votre lecteur d'écran puisse suivre :

* Après que Claude Code imprime la ligne de confirmation, il attend 3 secondes avant de dessiner l'invite, afin que votre lecteur d'écran puisse terminer la ligne. Appuyez sur n'importe quelle touche pour terminer l'attente. Pour modifier la durée de l'attente, définissez [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/fr/env-vars#variables).
* Avant que Claude Code écrive une ligne nouvelle ou modifiée, comme un indice ou plus de la réponse de Claude, il déplace le curseur au début de la ligne et attend 50 millisecondes. Votre lecteur d'écran lit alors la ligne à partir de son premier caractère. Les caractères que vous tapez ou supprimez à la fin de la ligne d'entrée apparaissent immédiatement. Pour modifier la durée de l'attente, définissez [`CLAUDE_AX_PREPARK_MS`](/docs/fr/env-vars#variables).

Chaque message dans la transcription commence par une étiquette que votre lecteur d'écran annonce, nommant ce qu'il est : vos messages, les réponses et la réflexion de Claude, l'activité des outils, les erreurs et les avertissements, et les invites. Les étiquettes sont également consultables, afin que vous puissiez sauter entre les sections de la transcription en recherchant l'historique de votre terminal :

| Étiquette              | Signification                                                                                                  |
| :--------------------- | :------------------------------------------------------------------------------------------------------------- |
| `you:`                 | Vos messages                                                                                                   |
| `claude:`              | Les réponses de Claude                                                                                         |
| `thinking:`            | La réflexion de Claude                                                                                         |
| `tool:`                | L'activité des outils, comme une modification de fichier ou l'exécution d'une commande                         |
| `tool error:`          | Un outil qui a échoué                                                                                          |
| `error:`               | Une erreur dans la conversation, comme une demande d'API échouée                                               |
| `warning:`             | Un avertissement de Claude Code, comme un basculement vers un modèle de secours                                |
| `Permission Required:` | Une invite d'autorisation en attente de votre réponse                                                          |
| `Cost:`                | Le résumé du coût de la session lorsque Claude Code se termine, si votre compte [affiche les coûts](/docs/fr/costs) |

Claude Code garde le curseur du terminal sur le curseur d'entrée, afin que la commande de lecture de la ligne actuelle de votre lecteur d'écran lise l'invite que vous modifiez.

Au fur et à mesure que vous tapez à la fin de la ligne d'entrée, ou appuyez sur `Backspace` là, Claude Code écrit uniquement les caractères qui changent. Votre lecteur d'écran n'écho que ces caractères.

Lorsque vous supprimez un mot ou une ligne avec l'un des [raccourcis d'édition de texte](/docs/fr/interactive-mode#text-editing), Claude Code annonce le texte supprimé :

* Suppression de mots avec `Ctrl+W` ou `Alt+D`, ou avec `Option+Delete` sur macOS ou `Ctrl+Backspace` sur Windows
* Suppression jusqu'au début de la ligne avec `Ctrl+U` ou `Cmd+Backspace`
* Suppression jusqu'à la fin de la ligne avec `Ctrl+K`

Lorsque vous parcourez les [modes d'autorisation](/docs/fr/permission-modes) avec `Shift+Tab`, Claude Code annonce le mode d'autorisation sur lequel vous atterrissez, comme `[plan mode on]` ou `[accept edits on]`. Claude Code imprime l'annonce une fois et ne la répète pas lors des redessins ultérieurs.

<h3 id="jump-between-turns">
  Sauter entre les tours
</h3>

Claude Code émet des marqueurs d'intégration de shell OSC 133 aux limites des tours, afin que la touche de saut à l'invite précédente de votre terminal se déplace entre les tours sans lire toute la transcription :

* iTerm2 : Cmd+Shift+Up
* Terminal VS Code : Ctrl+Up sur Windows, Cmd+Up sur macOS
* Windows Terminal : pas de touche par défaut ; liez l'action `scrollToMark` dans ses paramètres
* Kitty et Ghostty : consultez la documentation du terminal pour sa touche de saut à l'invite

macOS Terminal n'agit pas sur les marqueurs, et Claude Code ne les émet pas dans WezTerm. Dans ces terminaux, recherchez plutôt l'étiquette `you:` dans l'historique.

<h2 id="answer-menus-and-prompts">
  Répondre aux menus et aux invites
</h2>

En mode lecteur d'écran, les menus que vous navigueriez normalement avec les touches fléchées, y compris les invites de permission, deviennent des listes numérotées. Claude Code annonce chaque option comme une ligne numérotée, puis une invite `Entrer la sélection` qui nomme la plage valide. Tapez le numéro de l'option que vous souhaitez et appuyez sur Entrée.

* Appuyez sur Échap pour annuler un menu dont l'invite se termine par `ou Échap pour annuler`.
* Si vous tapez un numéro qui ne figure pas sur la liste, Claude Code annonce la plage valide et vous permet de réessayer.

Le sélecteur [`/effort`](/docs/fr/model-config#adjust-effort-level), qui est un curseur en dehors du mode lecteur d'écran, devient le même type de liste numérotée.

Les invites oui-ou-non demandent une réponse tapée au lieu d'un menu à deux options. Répondez `y` ou `n` et appuyez sur Entrée. `yes` et `no` fonctionnent également.

<h2 id="hear-when-claude-code-needs-you">
  Entendre quand Claude Code a besoin de vous
</h2>

En mode lecteur d'écran, Claude Code sonne la cloche du terminal quand il a besoin de votre attention, afin que vous n'ayez pas à vérifier constamment la transcription. La cloche sonne quand :

* Claude termine une réponse
* Une invite ou une boîte de dialogue nécessite votre réponse, comme une invite de permission
* Un outil qui a fonctionné plus longtemps que 5 secondes se termine

La cloche est l'alerte standard de votre terminal. Pour la désactiver, modifiez le paramètre de cloche dans votre application de terminal. En dehors du mode lecteur d'écran, définissez [`preferredNotifChannel`](/docs/fr/settings-reference#preferrednotifchannel) sur `"terminal_bell"` pour obtenir une [cloche similaire](/docs/fr/terminal-config#get-a-terminal-bell-or-notification) quand Claude attend votre réponse.

<h2 id="known-limitations">
  Limitations connues
</h2>

Certains comportements ne sont pas adaptés au mode lecteur d'écran :

* Le mode lecteur d'écran ne s'active pas automatiquement lorsqu'un lecteur d'écran est en cours d'exécution.
* Claude Code n'annonce pas un changement de mode de permission effectué de toute autre manière que le cycle avec `Shift+Tab`, comme l'entrée en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) à partir d'une commande.
* L'attachement à une [session d'arrière-plan](/docs/fr/agent-view) avec `claude attach` ou à partir de la vue agent entre dans l'écran alternatif du terminal, qui n'a pas de scrollback natif. C'est le [même comportement que les autres sessions attachées](/docs/fr/fullscreen). Pour en sortir, appuyez sur la flèche gauche sur une invite vide, ou Ctrl+Z si une boîte de dialogue a le focus.
* Claude Code annonce les coûts dans le résumé qu'il imprime à la sortie, pas par tour.
* Le mode lecteur d'écran ne change pas le [mode non interactif](/docs/fr/headless) avec l'indicateur `-p`. Le mode non interactif écrit déjà du texte simple et reste une alternative pour les scripts.

<h2 id="report-an-issue">
  Signaler un problème
</h2>

Si quelque chose ne fonctionne pas avec votre lecteur d'écran, votre loupe ou votre terminal, ouvrez un problème sur le [suivi des problèmes Claude Code](https://github.com/anthropics/claude-code/issues) et mentionnez votre technologie d'assistance dans le titre. Incluez votre système d'exploitation, votre application de terminal et le nom et la version de votre technologie d'assistance dans le rapport.
