> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser Claude Code avec un lecteur d'écran

> Configurez Claude Code pour les lecteurs d'écran tels que VoiceOver et NVDA, ainsi que les paramètres pour les loupes d'écran, le mouvement réduit et les thèmes adaptés aux daltoniens.

Claude Code dispose d'un mode lecteur d'écran qui remplace son interface de terminal visuelle par du texte simple et linéaire. Au lieu de boîtes, d'animations de progression et de redessinages sur place, le mode imprime des lignes étiquetées qu'un lecteur d'écran tel que VoiceOver ou NVDA lit dans l'ordre, ce qui vous permet de maintenir une conversation complète, d'approuver les autorisations d'outils et d'examiner la sortie de bout en bout.

Le mode lecteur d'écran est optionnel. Si vous utilisez une loupe d'écran, un mouvement réduit ou un thème adapté aux daltoniens au lieu d'un lecteur d'écran, consultez [Paramètres d'accessibilité au-delà du mode lecteur d'écran](#accessibility-settings-beyond-screen-reader-mode).

<Note>
  Le mode lecteur d'écran nécessite Claude Code v2.1.181 ou version ultérieure. Les versions antérieures rejettent l'indicateur `--ax-screen-reader` avec `error: unknown option '--ax-screen-reader'`.
</Note>

<h2 id="turn-on-screen-reader-mode">
  Activer le mode lecteur d'écran
</h2>

Choisissez la méthode qui correspond à la fréquence à laquelle vous utilisez un lecteur d'écran :

* Pour une session : exécutez `claude --ax-screen-reader`.
* Pour les sessions démarrées à partir d'un shell : définissez la variable d'environnement `CLAUDE_AX_SCREEN_READER` sur `1`. Dans Bash ou Zsh, exécutez `export CLAUDE_AX_SCREEN_READER=1` ; dans PowerShell, exécutez `$env:CLAUDE_AX_SCREEN_READER = "1"`. Ajoutez la ligne à votre profil shell pour couvrir chaque shell.
* Pour chaque session sur la machine : ajoutez `"axScreenReader": true` à votre [fichier de paramètres](/docs/fr/settings) utilisateur. Cela couvre n'importe quel terminal, y compris le terminal intégré VS Code.

<Note>
  Les méthodes sont listées dans l'ordre de priorité : l'indicateur [`--ax-screen-reader`](/docs/fr/cli-reference#cli-flags) remplace la variable d'environnement [`CLAUDE_AX_SCREEN_READER`](/docs/fr/env-vars), qui remplace le paramètre [`axScreenReader`](/docs/fr/settings#available-settings).
</Note>

Si vous utilisez Claude Code via SSH, définissez la variable d'environnement ou le paramètre sur la machine distante où Claude Code s'exécute.

Lorsque le mode est activé, la première chose que Claude Code imprime est une ligne de confirmation nommant la méthode qui l'a activé : `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, ou `[Screen Reader Mode: on via settings]`. Le format de dénomination de méthode nécessite Claude Code v2.1.206 ou version ultérieure. Lorsque Claude Code se relance, par exemple pour terminer l'installation d'une mise à jour, le nouveau processus hérite du mode via la variable d'environnement `CLAUDE_AX_SCREEN_READER`, de sorte que sa ligne de confirmation lit `[Screen Reader Mode: on via env]` indépendamment de la méthode que vous avez utilisée.
Les versions antérieures impriment `[Accessible screen reader mode: on]`.

<h2 id="turn-off-screen-reader-mode">
  Désactiver le mode lecteur d'écran
</h2>

Inversez la méthode qui a activé le mode : démarrez sans l'indicateur, désactivez la variable d'environnement ou définissez `axScreenReader` sur `false`. La définition de `CLAUDE_AX_SCREEN_READER=0` maintient le mode désactivé même lorsque le paramètre est `true`.

<h2 id="what-your-screen-reader-hears">
  Ce que votre lecteur d'écran entend
</h2>

En mode lecteur d'écran, Claude Code écrit du texte plat :

* pas de caractères de dessin de boîte pour le chrome de l'interface
* pas d'indices basés sur la couleur uniquement
* pas de redessinages du contenu qui n'a pas changé ; les barres de progression s'affichent sous forme de texte statique
* les tableaux dans les réponses de Claude se lisent comme des phrases `Header: value` au lieu d'une grille de caractères de boîte. Nécessite Claude Code v2.1.198 ou version ultérieure ; les versions antérieures dessinent les tableaux sous forme de grilles même en mode lecteur d'écran.

La sortie s'accumule dans le scrollback de votre terminal, ce qui vous permet de relire les tours précédents avec les commandes de révision de votre lecteur d'écran ou la recherche de votre terminal.

Le mode lecteur d'écran s'affiche sous forme de texte de défilement simple, même si vous avez activé le [rendu en plein écran](/docs/fr/fullscreen) avec le [paramètre `tui`](/docs/fr/settings#available-settings) ; le paramètre n'a aucun effet tant que le mode est actif. Les sessions d'arrière-plan attachées s'affichent toujours en plein écran ; consultez [Limitations connues](#known-limitations).

Chaque message de la transcription commence par une étiquette que votre lecteur d'écran annonce, nommant ce qu'il est : vos messages, les réponses de Claude, l'activité des outils, les erreurs et les invites. Les étiquettes sont également consultables, ce qui vous permet de sauter entre les sections de la transcription en recherchant le scrollback de votre terminal :

| Étiquette              | Signification                                                                                                |
| :--------------------- | :----------------------------------------------------------------------------------------------------------- |
| `you:`                 | Vos messages                                                                                                 |
| `claude:`              | Les réponses de Claude                                                                                       |
| `tool:`                | Activité des outils, comme une édition de fichier ou une commande exécutée                                   |
| `tool error:`          | Un outil qui a échoué                                                                                        |
| `error:`               | Une erreur dans la conversation, comme une demande API échouée                                               |
| `Permission Required:` | Une invite d'autorisation en attente de votre réponse                                                        |
| `Cost:`                | Le résumé du coût de la session lorsque Claude Code se ferme, si votre compte [affiche les coûts](/docs/fr/costs) |

Le curseur du terminal suit le curseur d'entrée, de sorte que la commande de lecture de la ligne actuelle d'un lecteur d'écran répond « où suis-je » avec l'invite que vous modifiez.

<h3 id="jump-between-turns">
  Sauter entre les tours
</h3>

Claude Code émet des marqueurs d'intégration shell OSC 133 aux limites des tours, de sorte que la touche de saut à l'invite précédente de votre terminal se déplace entre les tours sans lire la transcription entière :

* iTerm2 : Cmd+Maj+Haut
* Terminal VS Code : Ctrl+Haut sous Windows, Cmd+Haut sur macOS
* Windows Terminal : aucune touche par défaut ; liez l'action `scrollToMark` dans ses paramètres
* Kitty et Ghostty : consultez la documentation du terminal pour sa touche de saut à l'invite

Le terminal macOS ne tient pas compte des marqueurs, et Claude Code ne les émet pas dans WezTerm. Dans ces terminaux, recherchez plutôt l'étiquette `you:` dans le scrollback.

<h2 id="answer-menus-and-prompts">
  Répondre aux menus et aux invites
</h2>

En mode lecteur d'écran, les menus que vous navigueriez normalement avec les touches fléchées, y compris les invites d'autorisation, deviennent des listes numérotées. Chaque option est annoncée comme une ligne numérotée, suivie d'une invite `Enter selection` qui nomme la plage valide. Tapez le numéro de l'option que vous souhaitez et appuyez sur Entrée.

* Pour annuler un menu pouvant être fermé : appuyez sur Échap. Son invite se termine par `or Escape to cancel`.
* Si vous tapez un numéro qui ne figure pas sur la liste : Claude Code annonce la plage valide et vous permet de réessayer.

Les invites oui ou non demandent une réponse tapée au lieu d'un menu à deux options. Répondez `y` ou `n` et appuyez sur Entrée. `yes` et `no` fonctionnent également.

<h2 id="hear-when-claude-code-needs-you">
  Entendre quand Claude Code a besoin de vous
</h2>

En mode lecteur d'écran, Claude Code sonne la cloche du terminal lorsqu'il a besoin de votre attention, de sorte que vous n'ayez pas à vérifier constamment la transcription. La cloche sonne quand :

* Claude termine une réponse
* une invite d'autorisation apparaît
* un outil qui s'est exécuté plus longtemps que 5 secondes se termine

La cloche est l'alerte standard de votre terminal. Pour la désactiver, modifiez le paramètre de cloche dans votre application de terminal. La cloche ne nécessite pas le mode lecteur d'écran : en dehors du mode, définissez [`preferredNotifChannel`](/docs/fr/settings#available-settings) sur `"terminal_bell"` pour des alertes similaires lorsque Claude vous attend. Consultez [Obtenir une cloche de terminal ou une notification](/docs/fr/terminal-config#get-a-terminal-bell-or-notification).

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  Paramètres d'accessibilité au-delà du mode lecteur d'écran
</h2>

Ces options répondent aux besoins d'accessibilité en dehors du mode lecteur d'écran. Tous fonctionnent avec lui.

* La [variable d'environnement](/docs/fr/env-vars) `CLAUDE_CODE_ACCESSIBILITY` est destinée aux loupes d'écran. Définissez `CLAUDE_CODE_ACCESSIBILITY=1` pour garder le curseur de terminal natif visible afin que les loupes, telles que macOS Zoom, puissent suivre la position du curseur.
* Le [paramètre](/docs/fr/settings#available-settings) `prefersReducedMotion` réduit ou désactive les barres de progression, les scintillements et autres animations sans modifier le reste de l'interface.
* Le [paramètre](/docs/fr/settings#available-settings) `theme` sélectionne les couleurs de l'interface, y compris les thèmes adaptés aux daltoniens `dark-daltonized` et `light-daltonized`.

<h2 id="known-limitations">
  Limitations connues
</h2>

Certains comportements ne sont pas adaptés au mode lecteur d'écran :

* Le mode lecteur d'écran ne s'active pas automatiquement lorsqu'un lecteur d'écran est en cours d'exécution.
* Les changements de mode, tels que l'entrée en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode), ne sont pas encore annoncés.
* L'attachement à une [session d'arrière-plan](/docs/fr/agent-view) avec `claude attach` ou à partir de la vue agent entre dans l'écran alternatif du terminal, qui n'a pas de scrollback natif. C'est le [même comportement que les autres sessions attachées](/docs/fr/fullscreen). Pour en sortir, appuyez sur la flèche gauche sur une invite vide, ou Ctrl+Z si une boîte de dialogue a le focus.
* Claude Code annonce les coûts dans le résumé qu'il imprime à la sortie, pas par tour.
* Le mode lecteur d'écran ne change pas le [mode non interactif](/docs/fr/headless) avec l'indicateur `-p`. Le mode non interactif écrit déjà du texte simple et reste une alternative pour les scripts.

<h2 id="report-an-issue">
  Signaler un problème
</h2>

Si quelque chose ne fonctionne pas avec votre lecteur d'écran, votre loupe ou votre terminal, ouvrez un problème sur le [suivi des problèmes Claude Code](https://github.com/anthropics/claude-code/issues) et mentionnez votre technologie d'assistance dans le titre. Incluez votre système d'exploitation, votre application de terminal et le nom et la version de votre technologie d'assistance dans le rapport.

<h2 id="related-resources">
  Ressources connexes
</h2>

Ces pages contiennent les entrées de référence complètes et la configuration connexe pour ce que cette page couvre :

* [Paramètres](/docs/fr/settings#available-settings) : les entrées `axScreenReader`, `prefersReducedMotion`, `theme` et `preferredNotifChannel`
* [Variables d'environnement](/docs/fr/env-vars) : les entrées `CLAUDE_AX_SCREEN_READER` et `CLAUDE_CODE_ACCESSIBILITY`
* [Référence CLI](/docs/fr/cli-reference#cli-flags) : l'indicateur `--ax-screen-reader`
* [Configuration du terminal](/docs/fr/terminal-config) : cloches, notifications et thèmes en dehors du mode lecteur d'écran
* [Mode non interactif](/docs/fr/headless) : exécutions `claude -p` scriptées, qui écrivent du texte simple sans mode lecteur d'écran
