> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mode interactif

> Référence complète des raccourcis clavier, modes d'entrée et fonctionnalités interactives dans les sessions Claude Code.

<h2 id="keyboard-shortcuts">
  Raccourcis clavier
</h2>

<Note>
  Les raccourcis clavier peuvent varier selon la plateforme et le terminal. En [rendu plein écran](/docs/fr/fullscreen), appuyez sur `?` dans la visionneuse de transcription pour voir les raccourcis disponibles.

  **Utilisateurs macOS** : Les raccourcis de la touche Option/Alt (`Alt+B`, `Alt+F`, `Alt+D`, `Alt+Y`, `Alt+P`) nécessitent de configurer Option en tant que Meta dans votre terminal. Consultez [Activer les raccourcis de la touche Option sur macOS](/docs/fr/terminal-config#enable-option-key-shortcuts-on-macos) pour le paramètre dans chaque terminal.
</Note>

<h3 id="general-controls">
  Contrôles généraux
</h3>

| Raccourci                                                                                           | Description                                                                                                                                                                                                                                                                                                                | Contexte                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                                                                            | Interrompre ou effacer l'entrée                                                                                                                                                                                                                                                                                            | Interrompt une opération en cours. Si rien ne s'exécute, la première pression efface l'entrée du prompt et une deuxième pression quitte Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `Ctrl+X Ctrl+K`                                                                                     | Arrêter tous les [sous-agents en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) dans cette session et désactiver les [réponses automatiques des artefacts](/docs/fr/artifacts#let-claude-reply-to-comments-on-its-own) pour le reste de celle-ci. Appuyez deux fois dans les 3 secondes pour confirmer | Contrôle des sous-agents                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Ctrl+D`                                                                                            | Quitter la session Claude Code                                                                                                                                                                                                                                                                                             | La première pression affiche un indice de confirmation et une deuxième pression dans les 800 ms quitte. Lorsque le prompt contient du texte, `Ctrl+D` supprime le caractère après le curseur à la place                                                                                                                                                                                                                                                                                                                                                                                                               |
| `Ctrl+G` ou `Ctrl+X Ctrl+E`                                                                         | Ouvrir dans l'éditeur de texte par défaut                                                                                                                                                                                                                                                                                  | Modifiez votre prompt ou votre réponse personnalisée dans votre éditeur de texte par défaut. `Ctrl+X Ctrl+E` est la liaison native readline. Activez **Afficher la dernière réponse dans l'éditeur externe** dans `/config` pour ajouter la réponse précédente de Claude en tant que contexte commenté avec `#` au-dessus de votre prompt ; Claude Code supprime le bloc de commentaire lorsque vous enregistrez                                                                                                                                                                                                      |
| `Ctrl+L`                                                                                            | Redessiner l'écran                                                                                                                                                                                                                                                                                                         | Force un redessinage complet du terminal, en conservant l'entrée et l'historique de la conversation. Utilisez ceci pour récupérer si l'affichage devient brouillé ou partiellement vide. Consultez [Effacer la conversation](/docs/fr/fullscreen#clear-the-conversation) pour le rendu plein écran                                                                                                                                                                                                                                                                                                                         |
| `Ctrl+O`                                                                                            | Basculer la visionneuse de transcription                                                                                                                                                                                                                                                                                   | Affiche l'utilisation détaillée des outils et l'exécution, avec un horodatage et le modèle utilisé sur chaque message d'assistant. Développe également les lignes qui s'effondrent par défaut, comme les appels MCP, affichés sous la forme d'une seule ligne `Called slack 3 times`, et les [messages de vos autres sessions](/docs/fr/cross-session-messaging#what-a-message-looks-like), affichés sous la forme d'un aperçu d'une ligne `Message from @<sender>`                                                                                                                                                        |
| `Ctrl+R`                                                                                            | Recherche inversée dans l'historique des commandes                                                                                                                                                                                                                                                                         | Recherchez les commandes précédentes de manière interactive                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `Ctrl+V` ou `Cmd+V` (iTerm2) ou `Alt+V` (Windows et WSL)                                            | Coller une image du presse-papiers                                                                                                                                                                                                                                                                                         | Insère une puce `[Image #N]` au curseur pour que vous puissiez la référencer positionnellement dans votre prompt. Sur WSL, `Ctrl+V` et `Alt+V` sont tous deux liés ; utilisez `Alt+V` si votre terminal intercepte `Ctrl+V`                                                                                                                                                                                                                                                                                                                                                                                           |
| `Ctrl+B`                                                                                            | Tâches en arrière-plan                                                                                                                                                                                                                                                                                                     | Met en arrière-plan les commandes Bash et les agents. Les utilisateurs de Tmux appuient deux fois                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `Ctrl+T`                                                                                            | Basculer la liste de contrôle des tâches de Claude                                                                                                                                                                                                                                                                         | Afficher ou masquer la [liste de contrôle à faire de Claude](#task-list) dans la zone d'état. Ce n'est pas la vue des tâches en arrière-plan ; utilisez [`/tasks`](/docs/fr/commands) pour voir les shells et sous-agents en cours d'exécution                                                                                                                                                                                                                                                                                                                                                                             |
| `Ctrl+S`                                                                                            | Ranger ou restaurer le prompt                                                                                                                                                                                                                                                                                              | Avec du texte dans l'entrée, le range et efface le prompt. Appuyé à nouveau sur un prompt vide, restaure le texte rangé, la position du curseur, le contenu collé et le mode d'entrée, donc une commande shell rangée `!` [shell](#shell-mode-with-prefix) revient en mode shell                                                                                                                                                                                                                                                                                                                                      |
| `Ctrl+Z`                                                                                            | Suspendre Claude Code                                                                                                                                                                                                                                                                                                      | Unix uniquement. Suspend le processus vers votre shell ; exécutez `fg` pour reprendre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `Flèches Gauche/Droite`                                                                             | Parcourir les onglets de dialogue                                                                                                                                                                                                                                                                                          | Naviguez entre les onglets dans les dialogues de permission et les menus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Tab`                                                                                               | Accepter une suggestion d'autocomplétion ou ajouter un commentaire à une réponse de permission                                                                                                                                                                                                                             | Pendant que les suggestions d'autocomplétion s'affichent dans l'entrée du prompt, accepte la suggestion sélectionnée. Sur la plupart des prompts de permission, avec **Oui** ou **Non** en focus, ouvre un champ de commentaire sur cette option, et l'appuyer à nouveau ferme le champ. Consultez [ajouter un commentaire lorsque vous répondez à un prompt de permission](/docs/fr/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                                                                                        |
| `Flèches Haut/Bas` ou `Ctrl+P`/`Ctrl+N`                                                             | Déplacer le curseur ou naviguer dans l'historique des commandes                                                                                                                                                                                                                                                            | Lorsque l'entrée s'étend sur plus d'une ligne visuelle, qu'elle soit enveloppée ou multiligne, déplace d'abord le curseur dans le prompt. Une fois que le curseur est sur la première ou la dernière ligne visuelle, appuyer à nouveau navigue dans l'historique des commandes. Pendant que vous avez des messages en attente, `Haut` à partir de la première ligne [les reprend à la place](#take-back-what-you-queued)                                                                                                                                                                                              |
| `Esc`                                                                                               | Interrompre Claude ou fermer un dialogue                                                                                                                                                                                                                                                                                   | Arrêtez la réponse actuelle ou l'appel d'outil au milieu du tour pour que vous puissiez rediriger. Claude conserve le travail effectué jusqu'à présent. Si vous avez des [messages en attente](#queue-messages-while-claude-works), Claude Code les envoie ensuite. Lorsqu'un dialogue est ouvert, `Esc` ferme le dialogue. Sur un prompt de permission, `Esc` refuse l'action, comme [**Non** sans commentaire](/docs/fr/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                                                   |
| `Esc` + `Esc`                                                                                       | Effacer le brouillon d'entrée ou rembobiner                                                                                                                                                                                                                                                                                | Lorsque l'entrée du prompt contient du texte, double `Esc` l'efface et enregistre le brouillon dans l'historique pour que `Haut` le rappelle. Lorsque l'entrée est vide, double `Esc` ouvre le [menu de rembobinage](/docs/fr/checkpointing) pour restaurer ou résumer le code et la conversation à partir d'un point antérieur                                                                                                                                                                                                                                                                                            |
| `Ctrl+Entrée` ou `Ctrl+X Ctrl+S`                                                                    | Envoyer les messages en attente maintenant                                                                                                                                                                                                                                                                                 | Envoie vos [messages en attente](#queue-messages-while-claude-works) et votre brouillon avec eux, immédiatement. [Quand Claude Code envoie ce que vous avez mis en attente](#when-claude-code-sends-what-you-queued) couvre ce qui se passe au tour sur lequel Claude travaille. En [mode shell](#shell-mode-with-prefix), la touche met en attente votre commande uniquement. Dans les terminaux qui ne signalent pas les touches étendues, `Ctrl+Entrée` arrive sous la forme d'une simple `Entrée` ; `Ctrl+X Ctrl+S` fonctionne dans n'importe quel terminal. Nécessite Claude Code v2.1.275 ou version ultérieure |
| `Shift+Tab`, ou `Alt+M` sur Windows lorsque le runtime Node ou Bun n'active pas le mode d'entrée VT | Parcourir les modes de permission                                                                                                                                                                                                                                                                                          | Parcourez `default` (étiqueté Manuel dans l'indicateur de mode), `acceptEdits`, `plan`, et, le cas échéant, `bypassPermissions` puis `auto`. À partir de `auto`, la première pression bascule vers `default`. Consultez [modes de permission](/docs/fr/permission-modes). Sur un prompt de permission de fichier, la même touche ferme un [champ de commentaire](/docs/fr/permissions#add-a-comment-when-you-answer-a-permission-prompt) ouvert. Sans champ ouvert, il sélectionne l'option qui autorise l'action pour le reste de la session, lorsque le prompt offre cette option                                             |
| `Option+P` (macOS) ou `Alt+P` (Windows/Linux)                                                       | Changer de modèle                                                                                                                                                                                                                                                                                                          | Changez de modèles sans effacer votre prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `Option+T` (macOS) ou `Alt+T` (Windows/Linux)                                                       | Basculer la réflexion étendue                                                                                                                                                                                                                                                                                              | Activez ou désactivez le mode de réflexion étendue. N'a aucun effet sur Opus 5.5 ou les modèles Fable, qui utilisent toujours la réflexion étendue. Fonctionne sur macOS sans configurer Option en tant que Meta                                                                                                                                                                                                                                                                                                                                                                                                      |
| `Option+O` (macOS) ou `Alt+O` (Windows/Linux)                                                       | Basculer le mode rapide                                                                                                                                                                                                                                                                                                    | Activez ou désactivez le [mode rapide](/docs/fr/fast-mode)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h3 id="text-editing">
  Édition de texte
</h3>

| Raccourci                  | Description                                       | Contexte                                                                                                                                                                                                                    |
| :------------------------- | :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                   | Déplacer le curseur au début de la ligne actuelle | En entrée multiligne, déplace au début de la ligne logique actuelle                                                                                                                                                         |
| `Ctrl+E`                   | Déplacer le curseur à la fin de la ligne actuelle | En entrée multiligne, déplace à la fin de la ligne logique actuelle                                                                                                                                                         |
| `Ctrl+K`                   | Supprimer jusqu'à la fin de la ligne              | Stocke le texte supprimé pour le collage                                                                                                                                                                                    |
| `Ctrl+U`                   | Supprimer du curseur au début de la ligne         | Stocke le texte supprimé pour le collage. Répétez pour effacer sur plusieurs lignes en entrée multiligne. Sur macOS, les émulateurs de terminal, y compris iTerm2 et Terminal.app, mappent `Cmd+Backspace` à ce raccourci   |
| `Ctrl+W`                   | Supprimer jusqu'à l'espace blanc précédent        | Stocke le texte supprimé pour le collage. Une pression supprime un chemin entier ou `--flag=value`. Pour supprimer uniquement le mot précédent, appuyez sur `Option+Delete` sur macOS ou `Ctrl+Backspace` sur Windows       |
| `Ctrl+Y`                   | Coller le texte supprimé                          | Colle le texte que vous avez supprimé en dernier avec l'un des raccourcis de suppression de mot ou de ligne, comme `Ctrl+K`, `Ctrl+U` ou `Ctrl+W`                                                                           |
| `Alt+Y` (après `Ctrl+Y`)   | Parcourir l'historique du collage                 | Après le collage, parcourez le texte supprimé précédemment. Nécessite [Option en tant que Meta](#keyboard-shortcuts) sur macOS                                                                                              |
| `Alt+B`                    | Déplacer le curseur d'un mot en arrière           | Navigation par mot. Nécessite [Option en tant que Meta](#keyboard-shortcuts) sur macOS                                                                                                                                      |
| `Alt+F`                    | Déplacer le curseur d'un mot en avant             | Se déplace à la fin du mot actuel, ou à la fin du mot suivant lorsque le curseur est entre les mots. Nécessite [Option en tant que Meta](#keyboard-shortcuts) sur macOS                                                     |
| `Alt+D`                    | Supprimer jusqu'à la fin du mot                   | Supprime jusqu'à la fin du mot actuel, ou jusqu'à la fin du mot suivant lorsque le curseur est entre les mots. Stocke le texte supprimé pour le collage. Nécessite [Option en tant que Meta](#keyboard-shortcuts) sur macOS |
| `Ctrl+_` ou `Ctrl+Shift+-` | Annuler la dernière modification d'entrée         | Restaure le texte d'entrée précédent et la position du curseur                                                                                                                                                              |

<h3 id="make-ctrl-w-delete-back-to-whitespace">
  Limites de mots dans les raccourcis d'édition
</h3>

Les raccourcis de mot `Alt+B`, `Alt+F`, `Alt+D`, `Option+Delete` et `Ctrl+Backspace` traitent un mot comme une suite de lettres et de chiffres, donc la ponctuation telle que `_`, `.` et `/` sépare les mots. Avec `src/utils/foo.ts` dans le prompt, les appuis répétés de `Alt+B` s'arrêtent au début de `ts`, `foo`, `utils` et `src`.

`Ctrl+W` est différent : il ignore la ponctuation et supprime jusqu'à l'espace blanc précédent, donc une pression supprime tout `src/utils/foo.ts`.

Dans le texte écrit sans espaces, comme le chinois ou le japonais, les raccourcis de mot se déplacent ou suppriment toujours un mot à la fois.

Ces conventions readline s'appliquent dans Claude Code v2.1.261 et versions ultérieures. Le paramètre [`keybindingFlavor`](/docs/fr/settings-reference#keybindingflavor) qui les activait dans les versions antérieures est obsolète et n'a aucun effet.

Vous ne pouvez pas remapper ces raccourcis dans le [fichier de configuration des liaisons de clavier](/docs/fr/keybindings), qui n'a pas d'actions pour eux.

<h3 id="theme-and-display">
  Thème et affichage
</h3>

| Raccourci | Description                                              | Contexte                                                                                                                                   |
| :-------- | :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+T`  | Basculer la coloration syntaxique pour les blocs de code | Fonctionne uniquement dans le menu du sélecteur `/theme`. Contrôle si le code dans les réponses de Claude utilise la coloration syntaxique |

<h3 id="multiline-input">
  Entrée multiligne
</h3>

| Méthode              | Raccourci          | Contexte                                                                                                                                                                                               |
| :------------------- | :----------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Échappement rapide   | `\` + `Entrée`     | Fonctionne dans tous les terminaux                                                                                                                                                                     |
| Touche Option        | `Option+Entrée`    | Après activation de [Option en tant que Meta](/docs/fr/terminal-config#enable-option-key-shortcuts-on-macos) sur macOS                                                                                      |
| Shift+Entrée         | `Shift+Entrée`     | Natif dans iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal. Pour les autres terminaux, consultez [Entrer des prompts multilignes](/docs/fr/terminal-config#enter-multiline-prompts) |
| Séquence de contrôle | `Ctrl+J`           | Fonctionne dans n'importe quel terminal sans configuration                                                                                                                                             |
| Mode collage         | Coller directement | Pour les blocs de code, les journaux                                                                                                                                                                   |

<h3 id="quick-commands">
  Commandes rapides
</h3>

| Raccourci           | Description                               | Notes                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------ | :---------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/` au début        | Commande ou compétence                    | Consultez [commandes](#commands) et [compétences](/docs/fr/skills)                                                                                                                                                                                                                                                                                                                                                                 |
| `!` au début        | Mode shell                                | Exécutez une commande directement, ajoutez sa sortie à la session et demandez à Claude d'y répondre                                                                                                                                                                                                                                                                                                                           |
| `@`                 | Mention de chemin de fichier              | Déclenchez l'autocomplétion du chemin de fichier. Dans les sessions avec [messagerie entre sessions](/docs/fr/cross-session-messaging#message-another-session), lorsque vous tapez au moins une lettre après le `@`, Claude Code suggère également vos autres sessions actives sur cette machine, pour que vous puissiez dire à Claude de messager celle que vous choisissez. Nécessite Claude Code v2.1.232 ou version ultérieure |
| `:`                 | Code d'emoji                              | Tapez un `:name:` complet pour insérer l'emoji, ou deux caractères ou plus pour les suggestions. Consultez [Codes d'emoji](#emoji-shortcodes). Nécessite Claude Code v2.1.217 ou version ultérieure                                                                                                                                                                                                                           |
| `?` sur entrée vide | Basculer le panneau d'aide des raccourcis | Taper `?` lorsque l'entrée contient déjà du texte insère le caractère                                                                                                                                                                                                                                                                                                                                                         |

<h3 id="transcript-viewer">
  Visionneuse de transcription
</h3>

Lorsque la visionneuse de transcription est ouverte (basculée avec `Ctrl+O`), ces raccourcis sont disponibles. Exécutez `/tui` sans argument pour vérifier quel rendu est actif. `Ctrl+E` peut être remappé via [`transcript:toggleShowAll`](/docs/fr/keybindings).

| Raccourci            | Description                                                                                                                                                                                                                                           |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Basculer le panneau d'aide des raccourcis clavier. Nécessite le [rendu plein écran](/docs/fr/fullscreen)                                                                                                                                                   |
| `{` / `}`            | Sauter au prompt utilisateur précédent ou suivant, comme le mouvement de paragraphe vim. Nécessite le [rendu plein écran](/docs/fr/fullscreen)                                                                                                             |
| `Ctrl+E`             | Basculer afficher tout le contenu. Disponible uniquement dans le rendu classique, pas dans le [rendu plein écran](/docs/fr/fullscreen)                                                                                                                     |
| `[`                  | Écrire la conversation complète dans le scrollback natif de votre terminal pour que `Cmd+F`, le mode copie tmux et d'autres outils natifs puissent la rechercher. Nécessite le [rendu plein écran](/docs/fr/fullscreen#search-and-review-the-conversation) |
| `v`                  | Écrire la conversation dans un fichier temporaire et l'ouvrir dans `$VISUAL` ou `$EDITOR`. Nécessite le [rendu plein écran](/docs/fr/fullscreen)                                                                                                           |
| `q`, `Ctrl+C`, `Esc` | Quitter la vue de transcription. Les trois peuvent être remappés via [`transcript:exit`](/docs/fr/keybindings)                                                                                                                                             |

<h3 id="voice-input">
  Entrée vocale
</h3>

| Raccourci                         | Description   | Notes                                                                                                                                                                                                              |
| :-------------------------------- | :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Maintenir ou appuyer sur `Espace` | Dictée vocale | Nécessite que la [dictée vocale](/docs/fr/voice-dictation) soit activée. Maintenez pour enregistrer, ou exécutez `/voice tap` pour le basculement par appui. [Remappable](/docs/fr/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Commandes
</h2>

Tapez `/` dans Claude Code pour voir les commandes disponibles, ou tapez `/` suivi de n'importe quelles lettres pour filtrer. Le menu `/` répertorie les commandes intégrées, les [compétences](/docs/fr/skills) créées par les utilisateurs et groupées, ainsi que les commandes contribuées par les [plugins](/docs/fr/plugins/overview) et les [serveurs MCP](/docs/fr/mcp#use-mcp-prompts-as-commands). Toutes les commandes intégrées ne sont pas visibles pour chaque utilisateur, car certaines dépendent de votre plateforme ou de votre plan, et [quelques commandes disponibles sont masquées du menu par conception](/docs/fr/commands#how-the-command-menu-matches-what-you-type) et s'exécutent lorsque vous tapez leur nom complet.

Dans le [rendu en plein écran](/docs/fr/fullscreen#use-the-mouse), la liste des commandes `/` et des suggestions de fichiers `@` répondent également à la souris : le survol met en évidence une ligne et le clic l'accepte.

Consultez la [référence des commandes](/docs/fr/commands) pour la liste complète des commandes incluses dans Claude Code.

<h3 id="complete-a-command-mid-prompt">
  Compléter une commande au milieu d'une invite
</h3>

La complétion de commande fonctionne également au milieu d'une invite : tapez `/` après un espace, puis les premières lettres d'un nom, comme dans `exécuter les tests, puis /com`. Seules les commandes dont les noms commencent par ces lettres correspondent, donc un chemin de fichier tel que `/tmp/notes.md` ne maintient pas une liste ouverte. Claude Code exécute une commande lui-même uniquement lorsque la commande [commence votre message](/docs/fr/commands).

* **Dans le [rendu en plein écran](/docs/fr/fullscreen)** : les correspondances s'ouvrent sous forme de liste pendant que vous tapez, sans ligne en surbrillance, donc `Entrée` envoie toujours votre invite telle que vous l'avez tapée. Appuyez sur `Tab` pour insérer la correspondance supérieure, ou choisissez une ligne avec les touches fléchées et `Entrée`.
* **En dehors du plein écran** : le reste de la correspondance supérieure apparaît sous forme de texte fantôme à votre curseur, avec un décompte tel que `+2` lorsque plusieurs commandes correspondent. Appuyez sur `Tab` pour insérer la seule correspondance, ou pour ouvrir la liste lorsque plusieurs correspondent, puis choisissez une ligne avec les touches fléchées et `Entrée`.

Dans les deux rendus, appuyez sur `Tab` sur un `/` nu au milieu d'une invite pour lister chaque commande.

Une compétence de plugin correspond également sur son nom nu, donc `/deploy` trouve une compétence nommée `myplugin:deploy-app`. Lorsque vous insérez la correspondance, Claude Code écrit le `/myplugin:deploy-app` complet.

<h2 id="vim-editor-mode">
  Mode d'édition Vim
</h2>

Activez l'édition de style vim via `/config` → Editor mode.

Claude Code conserve votre mode vim et la position du curseur lorsque vous basculez le [lecteur de transcription](#transcript-viewer) avec `Ctrl+O` ou ouvrez et fermez un panneau tel que `/config`. Si vous quittez l'invite en mode NORMAL, elle est toujours en mode NORMAL lorsque vous revenez, avec le curseur où vous l'aviez laissé.

<h3 id="mode-switching">
  Changement de mode
</h3>

| Commande          | Action                                                                                                                       | Du mode        |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `Esc` ou `Ctrl+[` | Entrer en mode NORMAL. Dans les terminaux qui utilisent le protocole clavier Kitty, `Ctrl+[` nécessite v2.1.242 ou ultérieur | INSERT, VISUAL |
| `i`               | Insérer avant le curseur                                                                                                     | NORMAL         |
| `I`               | Insérer au début de la ligne                                                                                                 | NORMAL         |
| `a`               | Insérer après le curseur                                                                                                     | NORMAL         |
| `A`               | Insérer à la fin de la ligne                                                                                                 | NORMAL         |
| `o`               | Ouvrir une ligne en dessous                                                                                                  | NORMAL         |
| `O`               | Ouvrir une ligne au-dessus                                                                                                   | NORMAL         |
| `v`               | Démarrer la sélection visuelle par caractère                                                                                 | NORMAL         |
| `V`               | Démarrer la sélection visuelle par ligne                                                                                     | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Remapper les séquences de touches en mode INSERT
</h3>

Le paramètre [`vimInsertModeRemaps`](/docs/fr/settings-reference#viminsertmoderemaps) mappe une séquence de deux touches en mode INSERT à Échap, donc un mappage comme `jj` vous ramène en mode NORMAL. Nécessite Claude Code v2.1.208 ou ultérieur.

L'exemple `~/.claude/settings.json` suivant active le mode vim et mappe `jj` à Échap :

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Chaque clé est exactement deux caractères imprimables tapés en séquence, et `"<Esc>"` est la seule cible prise en charge. Les entrées avec une longueur ou une cible différente sont ignorées.

Taper le premier caractère d'une séquence l'insère normalement. Appuyer sur le deuxième caractère dans une seconde supprime ce caractère en attente et bascule en mode NORMAL, ne laissant aucun caractère dans votre entrée. Après la fenêtre d'une seconde, ou si une touche différente suit, les deux caractères restent comme du texte littéral, vous pouvez donc toujours taper un mot contenant la séquence en faisant une pause entre les deux touches.

Claude Code lit ce paramètre à partir de votre fichier de paramètres utilisateur, du drapeau `--settings` et des [paramètres gérés](/docs/fr/managed-settings) uniquement. Les entrées dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet sont ignorées, donc un référentiel extrait ne peut pas remapper vos frappes.

<h3 id="navigation-normal-mode">
  Navigation (mode NORMAL)
</h3>

| Commande        | Action                                                                                                                                                                                         |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Déplacer à gauche/bas/haut/droite                                                                                                                                                              |
| `Space`         | Déplacer à droite                                                                                                                                                                              |
| `w`             | Mot suivant                                                                                                                                                                                    |
| `e`             | Fin du mot                                                                                                                                                                                     |
| `b`             | Mot précédent                                                                                                                                                                                  |
| `0`             | Début de la ligne                                                                                                                                                                              |
| `$`             | Fin de la ligne                                                                                                                                                                                |
| `^`             | Premier caractère non vide                                                                                                                                                                     |
| `gg`            | Début de l'entrée                                                                                                                                                                              |
| `G`             | Fin de l'entrée                                                                                                                                                                                |
| `f{char}`       | Sauter à la prochaine occurrence du caractère                                                                                                                                                  |
| `F{char}`       | Sauter à l'occurrence précédente du caractère                                                                                                                                                  |
| `t{char}`       | Sauter juste avant la prochaine occurrence du caractère                                                                                                                                        |
| `T{char}`       | Sauter juste après l'occurrence précédente du caractère                                                                                                                                        |
| `;`             | Répéter le dernier mouvement f/F/t/T                                                                                                                                                           |
| `,`             | Répéter le dernier mouvement f/F/t/T en sens inverse                                                                                                                                           |
| `/`             | Ouvrir la recherche d'historique inversée, identique à `Ctrl+R`. L'invite de recherche vide affiche un indice : appuyez sur `Esc` puis `i` puis `/` pour ouvrir le menu de commande à la place |

<Note>
  En mode NORMAL vim, si le curseur est au début ou à la fin de l'entrée et ne peut pas se déplacer davantage, `j`/`k` et `↑`/`↓` naviguent dans l'historique des commandes à la place. `←` sur une invite vide ouvre la [vue agent](/docs/fr/agent-view) à partir du mode NORMAL ainsi que INSERT ; avant v2.1.219, `←` sur une invite vide ne faisait rien en mode NORMAL.
</Note>

<h3 id="editing-normal-mode">
  Édition (mode NORMAL)
</h3>

| Commande              | Action                                                                                                                                |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `x`                   | Supprimer le caractère                                                                                                                |
| `dd`                  | Supprimer la ligne                                                                                                                    |
| `D`                   | Supprimer jusqu'à la fin de la ligne                                                                                                  |
| `dw`/`de`/`db`        | Supprimer le mot/jusqu'à la fin/en arrière                                                                                            |
| `df{char}`/`dt{char}` | Supprimer jusqu'à et y compris, ou jusqu'à, la prochaine occurrence d'un caractère                                                    |
| `cc`                  | Changer la ligne                                                                                                                      |
| `C`                   | Changer jusqu'à la fin de la ligne                                                                                                    |
| `cw`/`ce`/`cb`        | Changer le mot/jusqu'à la fin/en arrière                                                                                              |
| `s`                   | Remplacer le caractère : supprimer le caractère sous le curseur et entrer en mode INSERT. Nécessite Claude Code v2.1.211 ou ultérieur |
| `S`                   | Remplacer la ligne : effacer la ligne et entrer en mode INSERT. Nécessite Claude Code v2.1.211 ou ultérieur                           |
| `yy`/`Y`              | Copier la ligne                                                                                                                       |
| `yw`/`ye`/`yb`        | Copier le mot/jusqu'à la fin/en arrière                                                                                               |
| `p`                   | Coller après le curseur                                                                                                               |
| `P`                   | Coller avant le curseur                                                                                                               |
| `>>`                  | Indenter la ligne                                                                                                                     |
| `<<`                  | Dédenter la ligne                                                                                                                     |
| `J`                   | Joindre les lignes                                                                                                                    |
| `u`                   | Annuler                                                                                                                               |
| `.`                   | Répéter la dernière modification                                                                                                      |

<h3 id="text-objects-normal-mode">
  Objets texte (mode NORMAL)
</h3>

Les objets texte fonctionnent avec les opérateurs comme `d`, `c` et `y` :

| Commande  | Action                                          |
| :-------- | :---------------------------------------------- |
| `iw`/`aw` | Mot intérieur/autour                            |
| `iW`/`aW` | MOT intérieur/autour (délimité par des espaces) |
| `i"`/`a"` | Guillemets doubles intérieurs/autour            |
| `i'`/`a'` | Guillemets simples intérieurs/autour            |
| `i(`/`a(` | Parenthèses intérieures/autour                  |
| `i[`/`a[` | Crochets intérieurs/autour                      |
| `i{`/`a{` | Accolades intérieures/autour                    |

<h3 id="visual-mode">
  Mode visuel
</h3>

Appuyez sur `v` pour la sélection par caractère ou `V` pour la sélection par ligne. Les mouvements étendent la sélection, et les opérateurs agissent directement sur elle.

| Commande         | Action                                              |
| :--------------- | :-------------------------------------------------- |
| `d`/`x`          | Supprimer la sélection                              |
| `y`              | Copier la sélection                                 |
| `c`/`s`          | Changer la sélection                                |
| `p`              | Remplacer la sélection par le contenu du registre   |
| `r{char}`        | Remplacer chaque caractère sélectionné par `{char}` |
| `~`/`u`/`U`      | Basculer, minuscules ou majuscules la sélection     |
| `>`/`<`          | Indenter ou dédenter les lignes sélectionnées       |
| `J`              | Joindre les lignes sélectionnées                    |
| `o`              | Échanger le curseur et l'ancre                      |
| `iw`/`aw`/`i"`/… | Sélectionner un objet texte                         |
| `v`/`V`          | Basculer entre caractère et ligne, ou quitter       |

Le mode visuel par bloc avec `Ctrl+V` n'est pas pris en charge.

<h2 id="command-history">
  Historique des commandes
</h2>

Claude Code conserve un historique des invites que vous tapez, et le rappel avec la flèche vers le haut récupère les invites des sessions précédentes du même projet :

* L'historique des entrées est stocké par répertoire de travail
* L'exécution de `/clear` démarre une nouvelle session : le rappel liste ensuite les invites de la nouvelle session en premier, avec les invites des sessions antérieures après. La conversation de la session précédente est conservée et peut être reprise.
* Soumettre deux fois la même invite d'affilée enregistre une seule entrée d'historique, donc appuyer sur Haut accède à l'invite distincte précédente
* Lorsque vous rappelez une invite qui incluait du texte collé, Claude Code envoie à nouveau le contenu collé complet lors de la resoumission. Si le contenu a depuis été [nettoyé](/docs/fr/claude-directory#cleaned-up-automatically), Claude Code n'envoie pas la chaîne littérale `[Pasted text #N]` ; consultez [Coller du contenu volumineux](/docs/fr/terminal-config#paste-large-content) pour savoir ce qui se passe avec l'invite
* L'expansion d'historique avec `!` est désactivée par défaut

<h3 id="reverse-search-with-ctrl-r">
  Recherche inversée avec Ctrl+R
</h3>

Appuyez sur `Ctrl+R` pour rechercher interactivement dans votre historique de commandes. En [rendu plein écran](/docs/fr/fullscreen), `Ctrl+R` ouvre une boîte de dialogue de recherche à la place : tapez pour filtrer, appuyez sur `Haut` et `Bas` pour vous déplacer dans les correspondances, et appuyez sur `Ctrl+S` pour faire défiler l'étendue entre cette session, ce projet et tous les projets. Appuyez sur `Entrée` ou `Tab` pour placer une correspondance dans l'entrée d'invite, ou `Échap` pour annuler. Les étapes ci-dessous décrivent la recherche en ligne du moteur de rendu classique :

1. **Démarrer la recherche** : appuyez sur `Ctrl+R` pour activer la recherche d'historique inversée
2. **Tapez la requête** : entrez le texte à rechercher dans les commandes précédentes. Le terme de recherche est mis en évidence dans les résultats correspondants
3. **Naviguer dans les correspondances** : appuyez à nouveau sur `Ctrl+R` pour parcourir les correspondances plus anciennes
4. **Étendue de la recherche** : la recherche en ligne recherche toujours les invites de tous les projets
5. **Accepter une correspondance** :
   * Appuyez sur `Tab` ou `Échap` pour accepter la correspondance actuelle et continuer l'édition
   * Appuyez sur `Entrée` pour accepter et exécuter la commande immédiatement
6. **Annuler la recherche** :
   * Appuyez sur `Ctrl+C` pour annuler et restaurer votre entrée d'origine
   * Appuyez sur `Retour arrière` sur une recherche vide pour annuler

La recherche en ligne analyse votre historique d'invites complet, du plus récent au plus ancien, avec les doublons réduits à l'occurrence la plus récente. La boîte de dialogue plein écran recherche tout votre historique d'invites dans l'étendue sélectionnée, du plus récent au plus ancien, avec les doublons réduits à l'occurrence la plus récente : les invites les plus récentes apparaissent immédiatement, et les correspondances des invites plus anciennes se remplissent au fur et à mesure que Claude Code charge le reste. Les invites correspondantes s'affichent avec le terme de recherche mis en évidence, afin que vous puissiez trouver et réutiliser les entrées précédentes.

Accepter une correspondance ou annuler la recherche prend effet immédiatement, même si Claude Code charge toujours l'historique.

<h2 id="background-bash-commands">
  Commandes Bash en arrière-plan
</h2>

Claude Code prend en charge l'exécution de commandes Bash en arrière-plan, ce qui vous permet de continuer à travailler pendant que les processus de longue durée s'exécutent.

<h3 id="how-backgrounding-works">
  Fonctionnement de l'arrière-plan
</h3>

Lorsque Claude Code exécute une commande en arrière-plan, il exécute la commande de manière asynchrone et retourne immédiatement un ID de tâche en arrière-plan. Claude Code peut répondre à de nouvelles invites pendant que la commande continue à s'exécuter en arrière-plan.

Pour exécuter des commandes en arrière-plan, vous pouvez soit :

* Inviter Claude Code à exécuter une commande en arrière-plan
* Appuyer sur `Ctrl+B` pour déplacer une invocation d'outil Bash régulière vers l'arrière-plan. Les utilisateurs de Tmux doivent appuyer sur `Ctrl+B` deux fois en raison de la touche de préfixe de tmux.

**Fonctionnalités clés :**

* La sortie est écrite dans un fichier et Claude peut la récupérer à l'aide de l'outil Read
* Les tâches en arrière-plan ont des ID uniques pour le suivi et la récupération de la sortie
* Les tâches en arrière-plan sont automatiquement nettoyées lorsque Claude Code se ferme. Sur macOS et Linux, lorsque vous arrêtez une tâche en arrière-plan à partir de [`/tasks`](/docs/fr/commands) ou que Claude Code l'arrête à la fermeture, les processus qui se sont détachés du shell de la tâche, comme ceux démarrés sous `setsid` ou `timeout`, s'arrêtent également
* Si vous mettez la session en arrière-plan au lieu de la fermer, vos tâches en arrière-plan continuent à s'exécuter dans la session en arrière-plan. Voir [mettre une session en cours en arrière-plan](/docs/fr/agent-view#from-inside-a-session)
* Les tâches en arrière-plan sont automatiquement terminées si la sortie dépasse 5 Go, avec une note dans stderr expliquant pourquoi
* Sur macOS et Linux, Claude Code termine les tâches en arrière-plan en cours d'exécution lorsque le système d'exploitation signale une pression mémoire critique, à condition que la session soit inactive depuis au moins 30 minutes et qu'aucun tour ou sous-agent ne soit en cours d'exécution. Nécessite Claude Code v2.1.193 ou version ultérieure
  * Le [journal de débogage](/docs/fr/debug-your-config) indique pourquoi les tâches ont été arrêtées, ou pourquoi un événement de pression les a laissées en cours d'exécution
  * Définissez [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/fr/env-vars) sur `1` pour désactiver les arrêts dus à la pression mémoire
* Les commandes en arrière-plan appartenant à un [sous-agent](/docs/fr/sub-agents) n'ont pas de limite de temps, sauf qu'une commande appartenant à un sous-agent s'exécutant au premier plan se termine lorsque ce sous-agent donne sa réponse finale ; voir [Commandes en arrière-plan](/docs/fr/tools-reference#background-commands) dans la référence des outils. Avant v2.1.218, ni la récolte de pression mémoire ni la limite de 60 minutes ne couvraient les commandes déplacées vers l'arrière-plan avec `Ctrl+B`

Pour désactiver toutes les fonctionnalités de tâche en arrière-plan, définissez la variable d'environnement `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` sur `1`. Voir [Variables d'environnement](/docs/fr/env-vars) pour plus de détails.

**Commandes couramment mises en arrière-plan :**

* Outils de compilation (webpack, vite, make)
* Gestionnaires de paquets (npm, yarn, pnpm)
* Exécuteurs de tests (jest, pytest)
* Serveurs de développement
* Processus de longue durée (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Mode shell avec préfixe `!`
</h3>

Exécutez des commandes shell directement sans passer par Claude en préfixant votre entrée avec `!` :

```bash theme={null}
! npm test
! git status
! ls -la
```

Mode shell :

* Ajoute la commande et sa sortie au contexte de la conversation
* Affiche la progression et la sortie en temps réel
* Prend en charge le même backgrounding `Ctrl+B` pour les commandes de longue durée
* Ne nécessite pas que Claude interprète ou approuve la commande
* Prend en charge l'autocomplétion basée sur l'historique : tapez une commande partielle et appuyez sur `Tab` pour compléter à partir des commandes `!` précédentes du projet actuel
* Prend en charge l'autocomplétion du chemin de fichier en direct à partir de v2.1.193 sur toutes les plates-formes : tapez un jeton contenant une barre oblique, comme `./src/` ou `~/`, pour voir une liste déroulante des fichiers et répertoires correspondants, puis appuyez sur `Tab` pour accepter. Utilisez des barres obliques sur Windows également ; la liste déroulante est déclenchée par `/`, pas `\`
* Quittez avec `Escape`, `Backspace`, ou `Ctrl+U` sur une invite vide
* Coller du texte commençant par `!` dans une invite vide entre automatiquement en mode shell, correspondant au comportement du `!` tapé

À moins que votre session ne soit l'une de celles énumérées sous [mode sandbox strict](/docs/fr/sandboxing#the-unsandboxed-retry-escape-hatch), les commandes que vous tapez en mode shell s'exécutent en dehors du [sandbox](/docs/fr/sandboxing) même si vous avez activé le sandboxing, car le sandbox s'applique aux commandes que Claude exécute.

Claude répond automatiquement à la sortie de la commande une fois qu'elle arrive dans la transcription, vous pouvez donc exécuter `! npm test` et obtenir une explication des défaillances sans une deuxième invite. La réponse coûte la même chose que l'envoi d'une invite normale. Pour restaurer le comportement antérieur où la sortie est ajoutée au contexte sans réponse, définissez [`respondToBashCommands`](/docs/fr/settings-reference#respondtobashcommands) sur `false` dans `settings.json`. Avant v2.1.186, le mode shell ajoutait toujours la sortie au contexte sans réponse.

<h2 id="queue-messages-while-claude-works">
  Mettre en file d'attente les messages pendant que Claude travaille
</h2>

Tapez un message et appuyez sur `Entrée` pendant que Claude travaille. Claude Code met le message en file d'attente au lieu d'interrompre le tour, et affiche les entrées en attente au-dessus de la zone de saisie jusqu'à ce qu'il les envoie. Vous pouvez mettre en file d'attente les commandes `!` [shell](#shell-mode-with-prefix) et la plupart des [commandes](/docs/fr/commands) de la même manière, à l'exception des commandes, telles que `/status`, que Claude Code exécute dès que vous les envoyez.

Les messages envoyés et mis en file d'attente s'affichent en gris jusqu'à ce que Claude commence à y répondre, vous pouvez donc voir quels messages Claude n'a pas encore commencé à traiter.

<h3 id="when-claude-code-sends-what-you-queued">
  Quand Claude Code envoie ce que vous avez mis en file d'attente
</h3>

Le moment où une entrée en attente est envoyée par Claude Code dépend de ce que vous avez mis en file d'attente.

* Messages : si vous mettez un message en file d'attente pendant que Claude exécute des appels d'outils, Claude Code le transmet à Claude dès que ces appels d'outils se terminent, dans le même tour. Quand le tour se termine avec des messages toujours en attente, ils s'envoient sans une autre pression de touche, dans l'ordre où vous les avez tapés
* Commandes et commandes shell : Claude Code les conserve jusqu'à la fin du tour, puis les exécute une à une, en conservant l'ordre dans lequel vous les avez mises en file d'attente

Pour envoyer ce que vous avez mis en file d'attente sans attendre, appuyez sur `Ctrl+Entrée`. Vos messages en attente s'envoient immédiatement, avec votre brouillon mis en file d'attente derrière eux si vous en aviez tapé un. Nécessite Claude Code v2.1.275 ou ultérieur.

Si vous aviez mis en file d'attente une commande shell `!` avant vos messages, la touche interrompt le tour. Sinon, ce qui se passe au tour dépend de ce que Claude fait quand vous appuyez sur la touche :

* Exécution de commandes shell, de sous-agents ou d'autres travaux qui peuvent passer à l'[arrière-plan](#background-bash-commands) : ce travail passe à l'arrière-plan et continue de s'exécuter, et Claude lit vos messages dans le même tour
* Uniquement l'écriture d'une réponse, ou l'exécution de quelque chose qui ne peut pas passer à l'arrière-plan : Claude Code interrompt le tour et envoie vos messages ensuite. Avant v2.1.281, la touche interrompait le tour dans les deux cas

En [mode shell](#shell-mode-with-prefix), la touche met uniquement votre commande en file d'attente. Dans les terminaux qui ne signalent pas les touches étendues, `Ctrl+Entrée` arrive comme un simple `Entrée` et met le brouillon en file d'attente à la place ; `Ctrl+X Ctrl+S` fonctionne dans n'importe quel terminal. Les deux touches sont des liaisons de l'action [`chat:sendNow`](/docs/fr/keybindings#chat-actions).

Appuyez sur `Échap` pour interrompre le tour sans soumettre votre brouillon. Claude Code conserve ce que vous avez mis en file d'attente et l'envoie immédiatement.

Claude Code exécute certaines commandes dès que vous les envoyez au lieu de les mettre en file d'attente, notamment `/model`, `/effort` et `/fast`. Chacune de ces trois commandes modifie un paramètre : le modèle, le niveau d'effort ou le mode rapide. Le moment où Claude Code applique le nouveau paramètre au tour sur lequel Claude travaille déjà, ou uniquement à partir de votre tour suivant, diffère selon la commande :

* [`/model`](/docs/fr/model-config#setting-your-model) : une fois que vous confirmez l'[avertissement de cache](/docs/fr/prompt-caching#switching-models), si Claude Code en affiche un, Claude Code applique votre modification à la prochaine requête qu'il effectue dans ce tour
* [`/effort`](/docs/fr/model-config#adjust-effort-level) : une fois que vous confirmez l'[avertissement de cache](/docs/fr/prompt-caching#changing-effort-level), si Claude Code en affiche un, Claude Code applique votre modification à la prochaine requête qu'il effectue dans ce tour
* [`/fast`](/docs/fr/fast-mode#toggle-fast-mode) : Claude Code conserve le paramètre de mode rapide qui était actif au début du tour, donc votre changement de vitesse s'applique à partir de votre tour suivant. Si votre modèle actuel ne supporte pas le mode rapide, l'activer [bascule également votre modèle](/docs/fr/prompt-caching#turning-on-fast-mode), et Claude Code utilise le nouveau modèle à partir de sa prochaine requête dans ce tour

<h3 id="take-back-what-you-queued">
  Récupérer ce que vous avez mis en file d'attente
</h3>

Appuyez sur `Haut` à partir de la première ligne de la zone de saisie pour récupérer les messages et commandes en attente. Claude Code les supprime de la file d'attente et les place dans la zone de saisie, un par ligne, avant tout texte que vous aviez tapé. Modifiez le texte et appuyez sur `Entrée` pour le mettre à nouveau en file d'attente en tant qu'une seule entrée, ou videz la zone de saisie pour l'abandonner.

Claude Code récupère les commandes shell en attente uniquement quand la zone de saisie est vide et que vous n'avez rien d'autre en attente, et il bascule la zone de saisie en mode shell quand il le fait. Sinon, il les laisse dans la file d'attente, listées avec leur préfixe `!`, et les exécute après la fin du tour.

<h2 id="prompt-suggestions">
  Suggestions de prompt
</h2>

Lorsque vous ouvrez une session pour la première fois, Claude Code affiche une commande d'exemple grisée dans l'entrée de prompt pour vous aider à démarrer. Il la sélectionne à partir de l'historique git de votre projet, de sorte que l'exemple reflète les fichiers sur lesquels vous avez travaillé récemment.

Après que Claude réponde, Claude Code peut suggérer votre prochain prompt en fonction de votre historique de conversation, par exemple une étape de suivi d'une demande en plusieurs parties ou une continuation naturelle de votre flux de travail.

* Appuyez sur `Tab` ou `Flèche droite` pour placer la suggestion dans l'entrée de prompt, puis `Entrée` pour soumettre
* Commencez à taper pour la rejeter

Claude Code génère chacune de ces suggestions de prompt suivant avec une demande en arrière-plan auprès du même modèle que celui utilisé par votre session. La demande compte dans les limites d'utilisation de votre plan ou vos coûts d'API. Parce qu'elle réutilise le cache de prompt de la conversation, il s'agit principalement de lectures du cache plus quelques jetons de sortie, de sorte que le coût supplémentaire est minime.

<h3 id="when-claude-code-skips-suggestions">
  Quand Claude Code ignore les suggestions
</h3>

En mode interactif, Claude Code désactive les suggestions de prompt par défaut et masque le bouton bascule **Prompt suggestions** dans `/config` dans une [session qui ne récupère pas les drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching), par exemple une session chez un fournisseur tiers ou via une passerelle d'applications Claude, et dans une [première session après une installation ou une mise à jour](/docs/fr/env-vars#first-session-after-an-install-or-upgrade) dont les drapeaux ne sont pas encore arrivés.

Claude Code ignore également les suggestions individuelles dans plusieurs situations, notamment :

* Le cache de prompt est froid, pour éviter un coût inutile
* Après le premier tour d'une conversation, dans certaines sessions
* La réponse précédente s'est terminée par une erreur
* Pendant que vous êtes en plan mode
* Votre compte est proche ou à sa limite d'utilisation. Pour maintenir les suggestions activées jusqu'à ce que vous atteigniez la limite, définissez [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/fr/env-vars) sur `true`. Avant la v2.1.238, Claude Code les ignorait près de la limite même avec la variable définie sur `true`
* Dans une [équipe d'agents](/docs/fr/agent-teams), dans les sessions des coéquipiers par défaut. La session du responsable affiche les suggestions

En mode impression, Claude Code ne génère pas de suggestions par défaut. Passez [`--prompt-suggestions`](/docs/fr/cli-reference#cli-flags) avec `-p "<prompt>" --output-format stream-json --verbose` pour que Claude Code émette un message `prompt_suggestion` après chaque tour qui en génère un. Le générateur ignore également les conversations très courtes et les caches de prompt froids ici, donc une seule requête `-p` courte peut n'en émettre aucune.

<h3 id="turn-prompt-suggestions-off">
  Désactiver les suggestions de prompt
</h3>

Pour désactiver complètement les suggestions de prompt, utilisez l'une des options suivantes :

* Désactivez **Prompt suggestions** dans `/config`
* Définissez [`promptSuggestionEnabled`](/docs/fr/settings-reference#promptsuggestionenabled) sur `false` dans votre fichier de paramètres
* Définissez la variable d'environnement [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/fr/env-vars) sur `false`, qui a la priorité sur le paramètre :
  ```bash theme={null}
  export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
  ```

Pour désactiver les suggestions de prompt dans toute une organisation, définissez `promptSuggestionEnabled` sur `false` dans les [paramètres gérés](/docs/fr/managed-settings). Définissez également `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION` sur `false` sous la clé [`env`](/docs/fr/settings-reference#env) gérée afin que les utilisateurs ne puissent pas les réactiver avec leur propre variable d'environnement.

<h2 id="emoji-shortcodes">
  Codes courts d'emoji
</h2>

Tapez un `:` suivi d'un code court d'emoji dans l'entrée de l'invite pour insérer l'emoji. Nécessite Claude Code v2.1.217 ou version ultérieure.

* Tapez un code court complet tel que `:heart:` et Claude Code le remplace par ❤️ dès que vous tapez le `:` de fermeture
* Tapez `:` plus au moins deux caractères d'un nom, tel que `:hea`, pour ouvrir une fenêtre contextuelle de suggestion, puis appuyez sur `Tab` ou `Entrée` pour insérer l'emoji en surbrillance

Le code court doit commencer l'entrée ou suivre un espace, donc un `:` à l'intérieur d'un mot ou d'une URL n'ouvre pas de suggestions.

Pour désactiver la fonctionnalité, définissez [`emojiCompletionEnabled`](/docs/fr/settings-reference#emojicompletionenabled) sur `false` dans `settings.json`. Cela désactive à la fois la fenêtre contextuelle de suggestion et le remplacement en ligne.

<h2 id="check-spelling-as-you-type">
  Vérifier l'orthographe au fur et à mesure de la saisie
</h2>

Claude Code peut souligner les mots mal orthographiés dans l'entrée de l'invite pendant que vous tapez. Il vérifie uniquement le texte dans la zone de saisie, jamais les réponses de Claude ou vos fichiers. Il ne vérifie également rien lorsque la zone de saisie est en [mode shell](#shell-mode-with-prefix), recherche d'historique `Ctrl+R`, ou [dictée vocale](/docs/fr/voice-dictation).

La vérification orthographique est désactivée par défaut, et Claude Code ne vérifie rien en [mode lecteur d'écran](/docs/fr/accessibility). Nécessite Claude Code v2.1.235 ou version ultérieure.

<h3 id="prerequisites">
  Conditions préalables
</h3>

* Installez [aspell](https://github.com/GNUAspell/aspell), [hunspell](https://github.com/hunspell/hunspell), ou [ispell](https://en.wikipedia.org/wiki/Ispell) et assurez-vous qu'il se trouve dans votre `PATH`. Claude Code exécute le premier des trois qu'il trouve, dans cet ordre, sur toutes les plateformes, y compris un shim `.cmd` qu'un gestionnaire de paquets installe sur Windows.
* Pour vérifier que le programme se trouve dans votre `PATH`, exécutez `aspell --version`, `hunspell --version`, ou `ispell -v` dans votre terminal. Une erreur « command not found » signifie qu'il ne se trouve pas encore dans votre `PATH`.

<h3 id="turn-spell-checking-on-or-off">
  Activer ou désactiver la vérification orthographique
</h3>

Claude Code lit le paramètre [`spellcheck`](/docs/fr/settings-reference#spellcheck) à partir de trois emplacements, et l'ignore dans le `.claude/settings.json` et `.claude/settings.local.json` d'un projet. Activez-le à partir de celui que vous utilisez :

<Tabs>
  <Tab title="Paramètres utilisateur">
    Ajoutez `spellcheck` à `~/.claude/settings.json`. Il s'applique dans chaque projet que vous ouvrez, comme le reste de vos [paramètres utilisateur](/docs/fr/settings#where-settings-live) :

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>

  <Tab title="Ligne de commande">
    Enregistrez `spellcheck` dans un fichier JSON, tel que `spellcheck.json` :

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```

    Ensuite, transmettez le fichier à `--settings`. Il s'applique à cette session uniquement :

    ```bash theme={null}
    claude --settings spellcheck.json
    ```
  </Tab>

  <Tab title="Paramètres gérés">
    Ajoutez `spellcheck` à l'une des [sources de paramètres gérés](/docs/fr/permissions#managed-settings) de votre organisation. Il s'applique à chaque utilisateur qui reçoit ces paramètres, et ils ne peuvent pas le désactiver :

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>
</Tabs>

Pour vérifier que la vérification orthographique est activée, tapez un mot mal orthographié et un espace. Claude Code souligne le mot. Si ce n'est pas le cas, consultez [Quand Claude Code ne souligne rien](#when-claude-code-underlines-nothing). Pour désactiver à nouveau la vérification orthographique, définissez `enabled` sur `false` au même endroit, ou supprimez `spellcheck`.

Pour choisir lequel des trois programmes Claude Code exécute, quel dictionnaire il utilise, ou la couleur du soulignement, ajoutez l'un de ces champs à côté de `enabled`, au même endroit :

* `checker` : `aspell`, `hunspell`, ou `ispell`. Claude Code ne revient pas à un vérificateur que vous nommez, et traite toute autre valeur comme `auto`.
* `language` : un nom de dictionnaire dans la forme de votre vérificateur, tel que `en_GB`. Claude Code ignore toute valeur qui n'est pas un nom de dictionnaire simple, tel qu'un chemin ou un nom avec des espaces, et le vérificateur utilise son dictionnaire par défaut.
* `color` : un nom de couleur tel que `yellow`, ou une valeur `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, ou `ansi:<name>`. Claude Code utilise la couleur d'erreur de votre thème par défaut et pour toute valeur qu'il ne reconnaît pas.

Par exemple, ce paramètre `spellcheck` exécute hunspell avec son dictionnaire `en_GB` et souligne les mots en jaune. Il fonctionne de la même manière dans `~/.claude/settings.json`, dans le fichier que vous transmettez à `--settings`, et dans les paramètres gérés :

```json theme={null}
{
  "spellcheck": {
    "enabled": true,
    "checker": "hunspell",
    "language": "en_GB",
    "color": "yellow"
  }
}
```

Si plusieurs des trois emplacements ont un paramètre `spellcheck`, Claude Code n'en utilise qu'un seul : d'abord les paramètres gérés, puis `--settings`, puis les paramètres utilisateur. Il ne combine pas les champs de deux emplacements. Par exemple, lorsque `--settings` définit `spellcheck`, un `language` dans vos paramètres utilisateur n'a aucun effet.

<h3 id="what-claude-code-underlines">
  Ce que Claude Code souligne
</h3>

Peu de temps après que vous ayez arrêté de taper, Claude Code souligne les mots que le dictionnaire ne connaît pas. Il laisse le mot que vous tapez encore seul jusqu'à ce que vous le dépassiez, et il ne change jamais votre texte. Il ignore également le texte qui ressemble à du code :

* Les commandes telles que `/help`, les mentions `@`, les URL, les chemins de fichiers, et les drapeaux tels que `--verbose`
* Les mots avec des chiffres, des traits de soulignement, ou une lettre majuscule après la première, et le texte entre backticks

Claude Code ignore également le texte chinois, japonais, coréen, thaï, lao, khmer et birman.

Claude Code n'a pas sa propre liste de mots : un mot est mal orthographié lorsque votre vérificateur le dit. Pour empêcher Claude Code de souligner un mot, ajoutez le mot au dictionnaire personnel de votre vérificateur, en suivant la documentation du vérificateur. Claude Code récupère le nouveau mot après son redémarrage.

<h3 id="when-claude-code-underlines-nothing">
  Quand Claude Code ne souligne rien
</h3>

Claude Code ne souligne rien lorsqu'il ne peut pas maintenir un vérificateur en cours d'exécution :

* Aucun vérificateur n'est installé, ou celui que vous avez nommé dans `checker` est manquant
* Le vérificateur échoue deux fois de suite, au démarrage ou plus tard dans la session. Claude Code le redémarre après le premier échec et arrête la vérification après le second, jusqu'à ce que vous redémarriez Claude Code
* Le vérificateur prend plus de 15 secondes pour répondre, trois fois. À chaque fois, Claude Code laisse les mots en attente non marqués ; après le troisième, il arrête la vérification jusqu'à ce que vous redémarriez Claude Code

Pour savoir lequel de ces cas s'est produit, démarrez `claude --debug` avec la vérification orthographique activée et tapez un mot. Ensuite, recherchez les lignes `[spellcheck]` dans le journal de débogage à `~/.claude/debug/<session-id>.txt`. Une ligne nomme le programme que Claude Code a démarré, ou énumère ceux qu'il a recherchés et n'a pas trouvés. Les lignes ultérieures expliquent pourquoi il s'est arrêté. Une erreur de dictionnaire manquant là signifie que le vérificateur n'a pas de dictionnaire pour votre valeur `language`, ou pas de dictionnaire par défaut lorsque `language` n'est pas défini. Installez-en un, ou définissez `language` sur un dictionnaire que vous avez.

<h2 id="invisible-characters-in-prompts">
  Caractères invisibles dans les invites
</h2>

Le texte collé peut contenir des caractères Unicode qu'un terminal n'affiche pas du tout, comme les caractères de balise, les contrôles bidirectionnels et les espaces de largeur nulle, de sorte qu'une invite peut contenir du texte que vous ne voyez jamais. Pour éviter que le texte copié ne contienne des instructions que votre terminal n'affiche pas, Claude Code supprime ces caractères lorsque vous appuyez sur Entrée, avant d'envoyer quoi que ce soit. Il nettoie à la fois l'invite et le contenu de toute [référence de texte collé](/docs/fr/terminal-config#paste-large-content) que l'invite inclut. Claude Code conserve les joineurs que les scripts persan et indique écrivent et les sélecteurs à l'intérieur des séquences emoji.

Si Claude Code a supprimé quelque chose, cet Entrée n'envoie rien. L'invite nettoyée revient dans la zone de saisie avec un avis tel que `Removed 3 invisible characters · review and press Enter to send`, et appuyer à nouveau sur Entrée envoie le texte tel qu'affiché.

Lorsque vous transmettez une invite sur la ligne de commande, comme dans `claude "fix the login bug"`, ou que vous en canalisez une dans une session interactive, Claude Code n'attend pas un deuxième Entrée. Il supprime les caractères, affiche un avis et envoie l'invite nettoyée. Si l'invite nettoyée commençait par `/`, Claude Code la place dans la zone de saisie pour que vous la révisiez et l'envoyiez à la place.

<h2 id="review-changes-with-/diff">
  Examinez les modifications avec /diff
</h2>

Exécutez `/diff` pour examiner les modifications dans votre arborescence de travail sans quitter Claude Code. Vous voyez les modifications que Claude a apportées jusqu'à présent ainsi que tout ce que vous n'avez pas validé.

Dans les modifications que `/diff` lit à partir de git, un sous-module apparaît comme une seule entrée, et uniquement lorsque le commit vers lequel il pointe change ; les modifications apportées aux fichiers à l'intérieur du sous-module n'y apparaissent pas.

Dans le [rendu en plein écran](/docs/fr/fullscreen), `/diff` ouvre le [panneau de diff](#diff-panel) à côté de la conversation, qui reste ouvert et se met à jour pendant que vous continuez à travailler. Dans le rendu classique, `/diff` ouvre la [visionneuse de diff](#diff-viewer) à la place de l'invite, et vous la fermez une fois que vous avez fini de lire.

<h3 id="diff-panel">
  Panneau de diff
</h3>

Le panneau de diff répertorie les fichiers modifiés avec leurs nombres de lignes ajoutées et supprimées, et affiche le diff de chaque fichier sous la liste. Claude Code l'actualise chaque fois que Claude modifie un fichier ou exécute une commande shell. Pour le fermer, exécutez `/diff` à nouveau ou cliquez sur le `✕` dans son en-tête.

Pour utiliser le panneau, vous avez besoin de :

* [Rendu en plein écran](/docs/fr/fullscreen)
* Un référentiel git
* Un terminal d'au moins 110 colonnes de large
* Claude Code v2.1.260 ou version ultérieure

Lorsque le panneau ne peut pas s'ouvrir, `/diff` ouvre la visionneuse de diff à la place ou vous indique pourquoi.

Le panneau s'ouvre également automatiquement une fois que Claude commence à modifier des fichiers, si votre terminal fait au moins 144 colonnes de large. Après l'avoir ouvert vous-même avec `/diff`, les sessions ultérieures l'ouvrent dès que Claude modifie un fichier dans n'importe quel terminal suffisamment large pour le contenir. Fermez le panneau et il reste fermé, dans cette session et les sessions ultérieures, jusqu'à ce que vous exécutiez `/diff` à nouveau.

Pendant que le panneau est ouvert, vous pouvez :

* **Accéder à un fichier** : cliquez sur sa ligne dans la liste. Faites défiler le panneau avec la molette de la souris. Lorsque la liste des fichiers elle-même est trop longue pour tenir, faites-la défiler avec `Alt+Haut` et `Alt+Bas`, ou `Ctrl+Haut` et `Ctrl+Bas`.
* **Demander à Claude des lignes spécifiques** : sélectionnez-les dans le panneau avec la souris. Claude Code joint la sélection à votre prochaine invite et affiche un nombre de lignes à côté de l'entrée jusqu'à ce que vous l'envoyiez.
  * Pour envoyer l'invite sans la sélection, déplacez le curseur juste après l'indicateur de nombre de lignes et appuyez sur `Retour arrière` pour le supprimer. Nécessite Claude Code v2.1.271 ou version ultérieure.
* **Afficher les fichiers que le panneau omet** : la liste ignore les fichiers de test et les fichiers générés, et réduit les modifications antérieures à cette session en une seule ligne en bas. Cliquez sur l'une ou l'autre ligne de comptage pour l'étendre.
* **Modifier ce que le panneau compare** : appuyez sur `Ctrl+X B` pour basculer entre les modifications de cette session, vos modifications non validées sous forme de liste unique, et tout ce qui s'est passé depuis que votre branche s'est séparée de la branche par défaut. Claude Code mémorise le choix pour chaque projet.

Pour lier des touches à ces actions, consultez [Actions du panneau de diff](/docs/fr/keybindings#diff-panel-actions).

<h3 id="diff-viewer">
  Visionneuse de diff
</h3>

La visionneuse de diff prend la place de l'invite jusqu'à ce que vous la fermiez. Sa vue **Actuelle** affiche vos modifications non validées de git, ou, s'il n'y en a pas, ce que votre branche ajoute au-dessus de la branche par défaut. La visionneuse a également une vue de tour pour chaque invite après laquelle Claude a modifié des fichiers, affichant uniquement ces modifications. Claude Code construit les vues de tour à partir des modifications de fichiers de Claude plutôt qu'à partir de git, donc une modification que Claude apporte via une commande shell n'apparaît que sous Actuelle.

Utilisez ces touches dans la visionneuse :

* **Gauche et Droite** : se déplacer entre Actuelle et les vues de tour.
* **Haut et Bas** : sélectionner un fichier.
* **Entrée** : ouvrir le diff du fichier sélectionné. Faites-le défiler avec Haut et Bas, ou PageUp et PageDown.
* **Échap** : revenir du diff d'un fichier à la liste, ou fermer la visionneuse à partir de la liste.

Pour rebinder ces touches, consultez [Actions de diff](/docs/fr/keybindings#diff-actions).

<h2 id="side-questions-with-/btw">
  Questions annexes avec /btw
</h2>

Utilisez `/btw` pour poser une question sur votre travail actuel sans l'ajouter à l'historique de la conversation.

```
/btw what was the name of that config file again?
```

Claude répond à une question annexe à partir de ce qui se trouve déjà dans la conversation : vos messages, ses réponses et les résultats des outils qu'il a rassemblés. Vous pouvez poser des questions sur le code que Claude a déjà lu, les décisions qu'il a prises plus tôt, ou n'importe quoi d'autre de la session. Une question annexe ultérieure voit également vos questions annexes antérieures : Claude Code rejoue les 20 échanges les plus récents à chaque demande, jusqu'à ce que vous les effaciez. La question et la réponse n'entrent jamais dans l'historique de la conversation. Dans le terminal, elles apparaissent dans une superposition pouvant être fermée. Le terminal conserve le fil en mémoire : appuyez sur `x` pour effacer les échanges antérieurs, et il disparaît quand vous quittez Claude Code.

Dans le [panneau de chat de l'extension VS Code](/docs/fr/vs-code#use-the-prompt-box), `/btw` ouvre un panneau plutôt que la superposition décrite dans cette section, et vous posez des questions de suivi directement dans le panneau. Le fil du panneau survit aux rechargements de fenêtre, selon le calendrier de rétention que cette page décrit. Vous avez besoin de l'extension à la version 2.1.227 ou ultérieure. Les versions antérieures de l'extension n'offrent pas `/btw`.

* **Disponible pendant que Claude travaille** : vous pouvez exécuter `/btw` même pendant que Claude traite une réponse. La question annexe s'exécute indépendamment et n'interrompt pas le tour principal. Elle voit tout ce qui se trouve dans la conversation jusqu'à présent, sauf la réponse que Claude est encore en train d'écrire.
* **Pas d'accès aux outils** : les questions annexes répondent uniquement à partir de ce qui se trouve déjà en contexte. Claude ne peut pas lire de fichiers, exécuter de commandes ou effectuer de recherches lors de la réponse à une question annexe. Si Claude écrit des appels d'outils sous forme de texte de toute façon, la réponse se termine par une note indiquant que rien n'a été exécuté.
* **Réponse unique** : il n'y a pas de tours de suivi dans la superposition. Pour continuer le fil, posez une autre question `/btw`. Pour continuer avec un accès complet aux outils dans une session locale, appuyez sur `f` pour créer un [sous-agent en arrière-plan](/docs/fr/sub-agents#fork-the-current-conversation) à partir de cette question et réponse.
* **Coût faible** : tandis que le [cache de prompt](/docs/fr/prompt-caching) de la conversation est actif, une question annexe coûte peu au-delà de la réponse elle-même.

Vos cinq questions annexes antérieures les plus récentes apparaissent sous forme de liste estompée au-dessus de la réponse actuelle, avec un décompte de celles plus anciennes. Elles restent en dehors de l'historique de la conversation.

Pour revenir à la superposition après l'avoir fermée, exécutez `/btw` sans question. La superposition se rouvre sur votre échange le plus récent. Avant la version 2.1.212, `/btw` sans question affichait un message d'utilisation à la place.

Une fois que la réponse apparaît, la superposition accepte ces touches.

| Touche                       | Action                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space`, `Enter`, `Escape`   | Fermer la réponse et revenir à l'invite                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `Up` / `Down`                | Faire défiler la réponse                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `Shift+Left` / `Shift+Right` | Naviguer entre cette réponse et vos réponses `/btw` antérieures. `Shift+Left` se déplace vers les réponses plus anciennes et `Shift+Right` revient vers la réponse actuelle. `[` et `]` font la même chose, pour les terminaux qui ne signalent pas `Shift` avec les touches fléchées. `Tab` / `Shift+Tab` parcourent les mêmes réponses. Nécessite Claude Code v2.1.257 ou ultérieure. Entre v2.1.187 et v2.1.256, les touches étaient `Left` / `Right` simples |
| `c`                          | Copier la réponse dans votre presse-papiers en tant que Markdown brut. Utilisez ceci au lieu de la sélection à la souris, qui capture le rendu du terminal avec retour à la ligne plutôt que le texte source                                                                                                                                                                                                                                                     |
| `f`                          | Démarrer un [sous-agent créé](/docs/fr/sub-agents#fork-the-current-conversation) qui hérite de la conversation parent plus cette question et réponse, afin qu'il puisse continuer avec un accès complet aux outils. Vous restez dans la session actuelle et trouvez la création dans le [panneau sous votre invite](/docs/fr/sub-agents#observe-and-steer-running-forks). Disponible uniquement dans les sessions locales                                                  |
| `x`                          | Effacer la liste des échanges `/btw` antérieurs affichés au-dessus de la réponse actuelle                                                                                                                                                                                                                                                                                                                                                                        |

Dans une [session en arrière-plan](/docs/fr/agent-view#attach-to-a-session) attachée, `Left` se détache et vous ramène à la vue agent, même pendant que la réponse arrive encore. La question annexe continue de s'exécuter pendant que vous êtes absent. La prochaine fois que vous vous attachez à la session, la superposition se rouvre avec la question annexe, ou avec sa réponse. Avant v2.1.257, `Left` ne se détachait pas là.

`/btw` voit votre conversation complète mais n'a pas d'outils. Un [sous-agent](/docs/fr/sub-agents) a des outils et commence à partir de l'invite qu'il reçoit, ou, pour une [création](/docs/fr/sub-agents#fork-the-current-conversation), à partir d'une copie de cette conversation. Utilisez `/btw` pour poser des questions sur ce que Claude connaît déjà de cette session ; utilisez un sous-agent pour aller découvrir quelque chose de nouveau.

<h2 id="task-list">
  Liste des tâches
</h2>

La liste des tâches est la liste de contrôle de Claude : des éléments que Claude a créés pour planifier un travail multi-étapes, avec des indicateurs montrant ce qui est en attente, en cours ou terminé. Elle est distincte de la vue des tâches en arrière-plan. Pour voir les shells en cours d'exécution et les sous-agents, utilisez [`/tasks`](/docs/fr/commands) à la place.

La liste se remplit uniquement dans les sessions qui disposent des outils de suivi des tâches, que Claude Code fournit par défaut sur [les modèles Claude 3.x, Opus 4 à 4.7, Sonnet 4 à 4.6 et Haiku 4.5](/docs/fr/tools-reference#task-tool-availability). Sur tout autre modèle, y compris un identifiant de modèle que Claude Code ne reconnaît pas, la liste reste vide sauf si vous acceptez avec `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` ou l'une des autres méthodes sous [Disponibilité de l'outil de tâche](/docs/fr/tools-reference#task-tool-availability). Lorsque la session dispose des outils, la liste des tâches fonctionne comme suit :

* Appuyez sur `Ctrl+T` pour basculer l'affichage de la liste des tâches. L'affichage montre jusqu'à cinq tâches à la fois. Lorsque Claude n'a pas encore créé d'éléments de liste de contrôle, le basculement n'a aucun effet visible car il n'y a rien à afficher
* Si vous laissez la liste développée, Claude Code restaure la vue développée la prochaine fois que vous lancez une session qui contient toujours des tâches, par exemple avec `--resume` ou `--continue`. Lorsque la liste des tâches est vide, Claude Code la démarre en mode réduit
* Pour voir toutes les tâches ou les effacer, demandez directement à Claude : « affiche-moi toutes les tâches » ou « efface toutes les tâches »
* Les tâches persistent lors des compactions de contexte, aidant Claude à rester organisé sur les projets plus importants
* Pour partager une liste de tâches entre les sessions, définissez `CLAUDE_CODE_TASK_LIST_ID` pour utiliser un répertoire nommé dans `~/.claude/tasks/` : `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Récapitulatif de session
</h2>

Lorsque vous revenez au terminal après vous être éloigné, Claude Code affiche un récapitulatif d'une ligne de ce qui s'est passé dans la session jusqu'à présent. Le récapitulatif se génère en arrière-plan une fois qu'au moins trois minutes se sont écoulées depuis le dernier tour complété et que le terminal n'est pas actif, de sorte qu'il soit prêt lorsque vous revenez. Les récapitulatifs n'apparaissent qu'une fois que la session compte au moins trois tours, et jamais deux fois de suite.

Exécutez `/recap` pour générer un résumé à la demande. Claude Code limite à la fois les récapitulatifs automatiques et la sortie de `/recap` à 400 caractères. Pour désactiver les récapitulatifs automatiques, ouvrez `/config` et désactivez **Session recap**.

Le récapitulatif de session est activé par défaut pour tous les plans et fournisseurs. Le récapitulatif est toujours ignoré en mode non interactif.

<h2 id="wait-for-a-usage-limit-to-reset">
  Attendre la réinitialisation d'une limite d'utilisation
</h2>

Quand une [limite d'utilisation](/docs/fr/errors#youve-hit-your-session-limit) de claude.ai arrête Claude au milieu d'une tâche, Claude Code attend dans la session ouverte et continue la tâche automatiquement après la réinitialisation de la limite. La continuation automatique est activée par défaut dans les sessions interactives connectées avec un abonnement claude.ai. Nécessite Claude Code v2.1.234 ou version ultérieure.

Pendant que Claude Code attend, une ligne en bas de la session indique quand il continuera :

```text theme={null}
Usage limit reached · continuing automatically at 3:45pm · esc to cancel
```

Gardez la session ouverte. Ce qui se passe ensuite dépend de la façon dont l'attente se termine :

* **À la réinitialisation** : la ligne affiche `continuing shortly`, puis `Usage limit reset · continuing automatically`, et Claude Code envoie à Claude une invite fixe pour reprendre la tâche où elle s'est arrêtée. Il ne renvoie pas votre dernier message.
* **Après la mise en veille de votre ordinateur** : si elle a duré plus d'environ 30 minutes et que la limite s'est réinitialisée pendant la mise en veille, la ligne affiche `Your usage limit has reset · press enter to continue`. Appuyez sur `Entrée` pour continuer. Après une mise en veille plus courte, Claude Code continue automatiquement.
* **Tôt** : quand vous terminez l'ajout de [crédits d'utilisation](/docs/fr/costs#add-usage-credits-to-your-subscription) avec `/usage-credits`, vous reconnectez après `/upgrade`, ou changez de modèle avec `/model` pendant l'attente, Claude Code vérifie si l'utilisation est à nouveau disponible et continue immédiatement si c'est le cas. Il ne vérifie pas après une mise à niveau ou un achat que vous effectuez dans un navigateur de votre côté. Sous [`opusplan`](/docs/fr/model-config#opusplan-model-setting) et d'autres paramètres de modèle qui exécutent le mode plan sur un modèle différent, Claude Code attend la réinitialisation à la place.

La tâche continuée s'exécute comme n'importe quel autre tour. Claude Code demande toujours les [permissions](/docs/fr/permissions) comme d'habitude, de sorte que la tâche peut s'arrêter sur une invite pendant que vous êtes absent. S'il atteint à nouveau la limite, Claude Code réarme l'attente automatiquement au maximum deux fois de suite, puis s'arrête et affiche `Automatic continue stopped after repeated usage-limit hits · /rate-limit-options to try again`.

<h3 id="cancel-the-wait">
  Annuler l'attente
</h3>

Appuyez sur `Échap` à une invite vide, ou `Ctrl+C`, pendant que la ligne s'affiche, ou exécutez [`/rate-limit-options`](/docs/fr/commands#all-commands) et choisissez **Don't continue automatically**. Claude Code confirme avec une ligne qui commence par `Automatic continue cancelled`.

Après une annulation, rien ne continue jusqu'à ce que vous envoyiez une invite ou que vous choisissiez la ligne qui commence par **Wait here, then continue automatically** dans `/rate-limit-options` à nouveau. Claude Code ne démarre pas une attente de son propre chef pour cette fenêtre de réinitialisation ; la prochaine fenêtre de réinitialisation recommence à zéro.

L'attente se termine également sans continuer la tâche dans ces cas :

* **Vous envoyez une invite** : Claude Code exécute votre invite au lieu d'attendre.
* **Vous quittez Claude Code** : l'attente ne redémarre pas quand vous reprenez la session.
* **La conversation change de mains** : vous changez de compte avec `/login`, effacez ou rembobinez la conversation, `/resume` une autre session, en tirez une avec `/teleport`, relancez avec `/tui`, ou remettez la session à Claude Desktop, une session en arrière-plan, ou le cloud.
* **Le paramètre s'éteint, ou la réinitialisation dépasse 24 heures** : cela termine uniquement une attente que Claude Code a démarrée de son propre chef. Une attente que vous avez choisie dans `/rate-limit-options` continue à compter à rebours.
* **La continuation est bloquée** : un hook [`UserPromptSubmit`](/docs/fr/hooks#userpromptsubmit) qui bloque l'invite de continuation, ou une défaillance avant qu'elle n'atteigne le modèle, termine l'attente. Claude Code vous indique que la continuation n'a pas été exécutée. Envoyez une invite pour continuer.

<h3 id="start-a-wait-yourself">
  Démarrer une attente vous-même
</h3>

Claude Code ne démarre pas l'attente de son propre chef dans ces cas :

* **Sessions Remote Control et agent team teammate** : une personne à ce terminal peut toujours en démarrer une.
* **Une réinitialisation à plus de 24 heures** : une limite hebdomadaire peut se réinitialiser des jours à l'avance.
* **Une limite Opus ou Sonnet pendant que vous exécutez un modèle en dehors de cette famille** : votre prochain tour peut ne pas atteindre cette limite. [`opusplan`](/docs/fr/model-config#opusplan-model-setting) et d'autres paramètres de modèle qui exécutent le mode plan sur la famille limitée ne bénéficient pas de cette exception.

Dans ces cas, et chaque fois que la continuation automatique est désactivée, Claude Code ouvre le menu des options de limite d'utilisation une fois par fenêtre de réinitialisation quand vous atteignez une limite à votre propre terminal. Choisissez la ligne qui commence par **Wait here, then continue automatically** pour démarrer l'attente. Dans une session [Remote Control](/docs/fr/remote-control) ou [agent team](/docs/fr/agent-teams) teammate, exécutez `/rate-limit-options` vous-même pour ouvrir le menu.

Claude Code n'offre pas du tout l'attente dans ces cas :

* **Sessions en arrière-plan et exécutions `-p`** : la ligne du menu n'est pas disponible.
* **Clés API, fournisseurs cloud et facturation basée sur l'utilisation** : l'utilisation y est mesurée par demande, il n'y a donc pas de réinitialisation à attendre.
* **Une [passerelle LLM](/docs/fr/llm-gateway#subscriptions-and-gateways) sans connexion claude.ai enregistrée** : Claude Code offre l'attente uniquement quand une connexion claude.ai enregistrée est l'identifiant actif.

<h3 id="turn-automatic-continue-off">
  Désactiver la continuation automatique
</h3>

Dans `/config`, désactivez **Continue automatically at usage limit**, ou définissez [`autoContinueAtUsageLimit`](/docs/fr/settings-reference#autocontinueatusagelimit) sur `false` dans vos paramètres utilisateur. `/config autoContinueAtUsageLimit=false` fonctionne également, y compris avec `-p`, mais la forme `key=value` ne peut pas le réactiver, car le paramètre accorde l'exécution sans surveillance. Les fichiers de paramètres que Claude Code lit pour cette clé sont dans la [référence des paramètres](/docs/fr/settings-reference#autocontinueatusagelimit).

<h2 id="pr-review-status">
  Statut de la revue de PR
</h2>

Lorsque vous travaillez sur une branche avec une demande de fusion ouverte, Claude Code affiche un lien PR cliquable dans le pied de page, par exemple « PR #446 ». Le lien a un soulignement coloré indiquant l'état de la revue :

* Vert : approuvé
* Jaune : en attente de revue
* Rouge : modifications demandées
* Gris : brouillon

Le badge disparaît une fois que la demande de fusion est fusionnée ou fermée.

`Cmd+clic` (macOS) ou `Ctrl+clic` (Windows/Linux) sur le lien pour ouvrir la demande de fusion dans votre navigateur.

Le statut se rafraîchit dès qu'un `git push`, ou une commande `gh pr` qui modifie la demande de fusion, comme `gh pr create` ou `gh pr merge`, réussit dans la session.

Claude Code affiche le badge comme un lien hypertexte même lorsqu'il ne peut pas détecter la prise en charge des liens hypertextes dans votre terminal, ce qui se produit couramment via SSH ou dans tmux. Définissez [`FORCE_HYPERLINK=0`](/docs/fr/env-vars) pour afficher le badge en tant que texte brut.

Lorsque vous définissez [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars), Claude Code ne vérifie pas l'état de la demande de fusion ou de la demande de fusion.

<Note>
  Le statut de PR pour les référentiels GitHub nécessite un jeton GitHub. Claude Code en trouve un en fonction de l'hôte du distant :

  * **github.com** : `GH_TOKEN` ou `GITHUB_TOKEN`, ou le jeton enregistré par `gh auth login`. Sans celui-ci, le pied de page affiche `install gh for PR status` lorsque la CLI `gh` n'est pas installée, ou `gh auth login for PR status` lorsqu'elle l'est
  * **Un hôte GitHub Enterprise défini comme `GH_HOST`** : `GH_ENTERPRISE_TOKEN` ou `GITHUB_ENTERPRISE_TOKEN`, ou le jeton enregistré par `gh auth login --hostname <host>`. Sans celui-ci, le pied de page affiche les mêmes indices
  * **Tout autre hôte GitHub** : le jeton enregistré par `gh auth login --hostname <host>`. Sans celui-ci, Claude Code n'affiche aucun badge et aucun indice
</Note>

<h3 id="gitlab-merge-requests">
  Demandes de fusion GitLab
</h3>

Lorsque vous travaillez sur une branche avec une demande de fusion GitLab ouverte, Claude Code affiche un badge `MR !N` cliquable dans l'emplacement du pied de page qui contient autrement le lien GitHub PR. `!N` est la syntaxe de référence propre à GitLab pour le numéro de demande de fusion N. Le soulignement coloré affiche l'état de la demande de fusion :

* Vert : GitLab signale que la demande de fusion est fusionnable
* Jaune : tout autre état ouvert
* Gris : brouillon

Le badge disparaît une fois que la demande de fusion est fusionnée ou fermée.

Il se rafraîchit dès qu'un `git push`, ou une commande `glab mr` qui modifie la demande de fusion, comme `glab mr create` ou `glab mr merge`, réussit dans la session.

Pour obtenir le badge, vous avez besoin de :

* Claude Code v2.1.234 ou version ultérieure
* Un distant de référentiel qui pointe vers votre hôte GitLab, soit gitlab.com, soit une instance auto-gérée
* La [CLI `glab`](https://gitlab.com/gitlab-org/cli) sur votre `PATH`, authentifiée avec `glab auth login`

Claude Code ignore les variables d'environnement du jeton de `glab`, comme `GITLAB_TOKEN`, lorsqu'il vérifie le statut, donc vous n'obtenez aucun badge à partir d'un jeton exporté seul. Claude Code recherche également `glab` et sa connexion une fois par session, donc redémarrez Claude Code après avoir installé `glab` ou exécuté `glab auth login`.

<h2 id="issue-reference-links">
  Liens de référence aux problèmes
</h2>

Lorsque Claude mentionne un problème sous la forme `owner/repo#123`, vous pouvez cliquer sur la référence pour l'ouvrir, à condition que votre terminal supporte les hyperliens. Si Claude Code ne détecte pas la prise en charge des hyperliens dans votre terminal, définissez [`FORCE_HYPERLINK`](/docs/fr/env-vars) sur `1` pour activer les liens, ou sur `0` pour conserver les références sous forme de texte brut.

Vous n'obtenez un lien que pour la forme à deux parties `owner/repo#123`. Ceux-ci restent du texte brut :

* Un `#123` isolé
* Un chemin GitLab imbriqué tel que `group/subgroup/project#123`
* Toute référence à l'intérieur d'une portée de code ou d'un bloc de code

Claude Code construit le lien pour l'hôte du référentiel qu'il identifie à partir de votre git remote, et non pour le référentiel que la référence nomme :

| L'hôte de votre référentiel                                              | Où `owner/repo#123` crée des liens             |
| :----------------------------------------------------------------------- | :--------------------------------------------- |
| github.com, un hôte GitHub Enterprise, ou tout hôte non listé ci-dessous | `https://<host>/owner/repo/issues/123`         |
| gitlab.com                                                               | `https://gitlab.com/owner/repo/-/issues/123`   |
| bitbucket.org, codeberg.org, ou gitea.com                                | Pas de lien ; la référence reste du texte brut |

<h2 id="see-also">
  Voir aussi
</h2>

* [Skills](/docs/fr/skills) - Invites personnalisées et flux de travail
* [Checkpointing](/docs/fr/checkpointing) - Rembobiner les modifications de Claude et restaurer les états précédents
* [Référence CLI](/docs/fr/cli-reference) - Drapeaux et options de ligne de commande
* [Paramètres](/docs/fr/settings) - Options de configuration
* [Gestion de la mémoire](/docs/fr/memory) - Gestion des fichiers CLAUDE.md
