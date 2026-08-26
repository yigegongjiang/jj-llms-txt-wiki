> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mode interactif

> Référence complète des raccourcis clavier, modes d'entrée et fonctionnalités interactives dans les sessions Claude Code.

<h2 id="keyboard-shortcuts">
  Raccourcis clavier
</h2>

<Note>
  Les raccourcis clavier peuvent varier selon la plateforme et le terminal. En [rendu plein écran](/docs/fr/fullscreen), appuyez sur `?` dans la visionneuse de transcription pour voir les raccourcis disponibles là-bas.

  **Utilisateurs macOS** : Les raccourcis de la touche Option/Alt (`Alt+B`, `Alt+F`, `Alt+Y`, `Alt+M`, `Alt+P`) nécessitent de configurer Option en tant que Meta dans votre terminal :

  * **iTerm2** : Paramètres → Profils → Touches → Général → définir la touche Option gauche/droite sur « Esc+ »
  * **Terminal Apple** : Paramètres → Profils → Clavier → cocher « Utiliser Option comme touche Meta »
  * **VS Code** : définir `"terminal.integrated.macOptionIsMeta": true` dans les paramètres VS Code

  Consultez [Configuration du terminal](/docs/fr/terminal-config) pour plus de détails.
</Note>

<h3 id="general-controls">
  Contrôles généraux
</h3>

| Raccourci                                                | Description                                                                                                                                                                       | Contexte                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                                 | Interrompre, ou effacer l'entrée                                                                                                                                                  | Interrompt une opération en cours. Si rien ne s'exécute, la première pression efface l'entrée d'invite et une deuxième pression quitte Claude Code                                                                                                                                                                                                                                             |
| `Ctrl+X Ctrl+K`                                          | Arrêter tous les [sous-agents en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) dans cette session. Appuyez deux fois dans les 3 secondes pour confirmer | Contrôle des sous-agents                                                                                                                                                                                                                                                                                                                                                                       |
| `Ctrl+D`                                                 | Quitter la session Claude Code                                                                                                                                                    | Signal EOF                                                                                                                                                                                                                                                                                                                                                                                     |
| `Ctrl+G` ou `Ctrl+X Ctrl+E`                              | Ouvrir dans l'éditeur de texte par défaut                                                                                                                                         | Modifiez votre invite ou réponse personnalisée dans votre éditeur de texte par défaut. `Ctrl+X Ctrl+E` est la liaison readline native. Activez Afficher la dernière réponse dans l'éditeur externe dans `/config` pour ajouter la réponse précédente de Claude en tant que contexte commenté avec `#` au-dessus de votre invite ; le bloc de commentaire est supprimé lorsque vous enregistrez |
| `Ctrl+L`                                                 | Redessiner l'écran                                                                                                                                                                | Force un redessinage complet du terminal. L'entrée et l'historique de la conversation sont conservés. Utilisez ceci pour récupérer si l'affichage devient brouillé ou partiellement vide                                                                                                                                                                                                       |
| `Ctrl+O`                                                 | Basculer la visionneuse de transcription                                                                                                                                          | Affiche l'utilisation détaillée des outils et l'exécution, avec un horodatage et le modèle utilisé sur chaque message d'assistant. Développe également les appels MCP, qui se réduisent à une seule ligne comme « Called slack 3 times » par défaut                                                                                                                                            |
| `Ctrl+R`                                                 | Recherche inversée dans l'historique des commandes                                                                                                                                | Recherchez les commandes précédentes de manière interactive                                                                                                                                                                                                                                                                                                                                    |
| `Ctrl+V` ou `Cmd+V` (iTerm2) ou `Alt+V` (Windows et WSL) | Coller une image du presse-papiers                                                                                                                                                | Insère une puce `[Image #N]` au curseur afin que vous puissiez la référencer positionnellement dans votre invite. Sur WSL, `Ctrl+V` et `Alt+V` sont tous deux liés ; utilisez `Alt+V` si votre terminal intercepte `Ctrl+V`                                                                                                                                                                    |
| `Ctrl+B`                                                 | Tâches en arrière-plan                                                                                                                                                            | Met en arrière-plan les commandes bash et les agents. Les utilisateurs Tmux appuyez deux fois                                                                                                                                                                                                                                                                                                  |
| `Ctrl+T`                                                 | Basculer la liste des tâches de Claude                                                                                                                                            | Afficher ou masquer [la liste des tâches de Claude](#task-list) dans la zone d'état. Ceci n'est pas la vue des tâches en arrière-plan ; utilisez [`/tasks`](/docs/fr/commands) pour voir les shells et sous-agents en cours d'exécution                                                                                                                                                             |
| `Flèches gauche/droite`                                  | Parcourir les onglets de dialogue                                                                                                                                                 | Naviguez entre les onglets dans les dialogues de permission et les menus                                                                                                                                                                                                                                                                                                                       |
| `Flèches haut/bas` ou `Ctrl+P`/`Ctrl+N`                  | Déplacer le curseur ou naviguer dans l'historique des commandes                                                                                                                   | Lorsque l'entrée s'étend sur plus d'une ligne visuelle, qu'elle soit enveloppée ou multiligne, déplace d'abord le curseur dans l'invite. Une fois que le curseur est sur la première ou la dernière ligne visuelle, appuyer à nouveau navigue dans l'historique des commandes. À partir de la v2.1.169, l'entrée monolignes enveloppée se comporte de la même manière que l'entrée multiligne  |
| `Esc`                                                    | Interrompre Claude, ou fermer un dialogue                                                                                                                                         | Arrêtez la réponse actuelle ou l'appel d'outil en cours de tour afin que vous puissiez rediriger. Claude conserve le travail effectué jusqu'à présent. Lorsqu'un dialogue tel qu'une invite de permission est ouvert, `Esc` ferme le dialogue plutôt que d'interrompre Claude. Avant la v2.1.202, `Esc` sur certains dialogues interrompait Claude et laissait le dialogue ouvert              |
| `Esc` + `Esc`                                            | Effacer le brouillon d'entrée, ou rembobiner                                                                                                                                      | Lorsque l'entrée d'invite contient du texte, double `Esc` l'efface et enregistre le brouillon dans l'historique afin que `Haut` le rappelle. Lorsque l'entrée est vide, double `Esc` ouvre le [menu de rembobinage](/docs/fr/checkpointing) pour restaurer ou résumer le code et la conversation à partir d'un point antérieur                                                                      |
| `Shift+Tab` ou `Alt+M` (certaines configurations)        | Basculer les modes de permission                                                                                                                                                  | Basculer entre `default` (étiqueté Manual dans l'indicateur de mode), `acceptEdits`, `plan` et tous les modes que vous avez activés, comme `auto` ou `bypassPermissions`. Consultez [modes de permission](/docs/fr/permission-modes).                                                                                                                                                               |
| `Option+P` (macOS) ou `Alt+P` (Windows/Linux)            | Changer de modèle                                                                                                                                                                 | Changez de modèles sans effacer votre invite                                                                                                                                                                                                                                                                                                                                                   |
| `Option+T` (macOS) ou `Alt+T` (Windows/Linux)            | Basculer la réflexion étendue                                                                                                                                                     | Activez ou désactivez le mode de réflexion étendue. N'a aucun effet sur Fable 5, qui utilise toujours la réflexion étendue. À partir de la v2.1.132, ce raccourci fonctionne sur macOS sans configurer Option en tant que Meta                                                                                                                                                                 |
| `Option+O` (macOS) ou `Alt+O` (Windows/Linux)            | Basculer le mode rapide                                                                                                                                                           | Activez ou désactivez le [mode rapide](/docs/fr/fast-mode)                                                                                                                                                                                                                                                                                                                                          |

<h3 id="text-editing">
  Édition de texte
</h3>

| Raccourci                | Description                                       | Contexte                                                                                                                                                                                                                           |
| :----------------------- | :------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                 | Déplacer le curseur au début de la ligne actuelle | Dans une entrée multiligne, déplace au début de la ligne logique actuelle                                                                                                                                                          |
| `Ctrl+E`                 | Déplacer le curseur à la fin de la ligne actuelle | Dans une entrée multiligne, déplace à la fin de la ligne logique actuelle                                                                                                                                                          |
| `Ctrl+K`                 | Supprimer jusqu'à la fin de la ligne              | Stocke le texte supprimé pour le collage                                                                                                                                                                                           |
| `Ctrl+U`                 | Supprimer du curseur au début de la ligne         | Stocke le texte supprimé pour le collage. Répétez pour effacer sur plusieurs lignes dans une entrée multiligne. Sur macOS, les émulateurs de terminal y compris iTerm2 et Terminal.app mappent `Cmd+Retour arrière` à ce raccourci |
| `Ctrl+W`                 | Supprimer le mot précédent                        | Stocke le texte supprimé pour le collage. Sur Windows, `Ctrl+Retour arrière` supprime également le mot précédent                                                                                                                   |
| `Ctrl+Y`                 | Coller le texte supprimé                          | Collez le texte supprimé avec `Ctrl+K`, `Ctrl+U` ou `Ctrl+W`                                                                                                                                                                       |
| `Alt+Y` (après `Ctrl+Y`) | Parcourir l'historique du collage                 | Après le collage, parcourez le texte précédemment supprimé. Nécessite [Option comme Meta](#keyboard-shortcuts) sur macOS                                                                                                           |
| `Alt+B`                  | Déplacer le curseur d'un mot en arrière           | Navigation par mot. Nécessite [Option comme Meta](#keyboard-shortcuts) sur macOS                                                                                                                                                   |
| `Alt+F`                  | Déplacer le curseur d'un mot en avant             | Navigation par mot. Nécessite [Option comme Meta](#keyboard-shortcuts) sur macOS                                                                                                                                                   |

<h3 id="theme-and-display">
  Thème et affichage
</h3>

| Raccourci | Description                                              | Contexte                                                                                                                                   |
| :-------- | :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+T`  | Basculer la coloration syntaxique pour les blocs de code | Fonctionne uniquement dans le menu du sélecteur `/theme`. Contrôle si le code dans les réponses de Claude utilise la coloration syntaxique |

<h3 id="multiline-input">
  Entrée multiligne
</h3>

| Méthode              | Raccourci          | Contexte                                                                                                    |
| :------------------- | :----------------- | :---------------------------------------------------------------------------------------------------------- |
| Échappement rapide   | `\` + `Entrée`     | Fonctionne dans tous les terminaux                                                                          |
| Touche Option        | `Option+Entrée`    | Après activation de [Option comme Meta](/docs/fr/terminal-config#enable-option-key-shortcuts-on-macos) sur macOS |
| Shift+Entrée         | `Shift+Entrée`     | Natif dans iTerm2, WezTerm, Ghostty, Kitty, Warp, Terminal Apple, Windows Terminal                          |
| Séquence de contrôle | `Ctrl+J`           | Fonctionne dans n'importe quel terminal sans configuration                                                  |
| Mode collage         | Coller directement | Pour les blocs de code, les journaux                                                                        |

<Tip>
  Shift+Entrée fonctionne sans configuration dans iTerm2, WezTerm, Ghostty, Kitty, Warp, Terminal Apple et Windows Terminal. Pour VS Code, Cursor, Devin Desktop, Alacritty et Zed, exécutez `/terminal-setup` pour installer la liaison.
</Tip>

<h3 id="quick-commands">
  Commandes rapides
</h3>

| Raccourci    | Description                  | Notes                                                                            |
| :----------- | :--------------------------- | :------------------------------------------------------------------------------- |
| `/` au début | Commande ou skill            | Consultez les [commandes](#commands) et les [skills](/docs/fr/skills)                 |
| `!` au début | Mode shell                   | Exécutez les commandes directement et ajoutez la sortie d'exécution à la session |
| `@`          | Mention de chemin de fichier | Déclencher l'autocomplétion du chemin de fichier                                 |

<h3 id="transcript-viewer">
  Visionneuse de transcription
</h3>

Lorsque la visionneuse de transcription est ouverte (basculée avec `Ctrl+O`), ces raccourcis sont disponibles. En [rendu plein écran](/docs/fr/fullscreen), appuyez sur `?` pour afficher le panneau de référence complet des raccourcis clavier à l'intérieur de la visionneuse. `Ctrl+E` peut être réaffecté via [`transcript:toggleShowAll`](/docs/fr/keybindings).

| Raccourci            | Description                                                                                                                                                                                                                                           |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Basculer le panneau d'aide des raccourcis clavier. Nécessite le [rendu plein écran](/docs/fr/fullscreen)                                                                                                                                                   |
| `{` / `}`            | Accéder à l'invite utilisateur précédente ou suivante, comme le mouvement de paragraphe vim. Nécessite le [rendu plein écran](/docs/fr/fullscreen)                                                                                                         |
| `Ctrl+E`             | Basculer afficher tout le contenu                                                                                                                                                                                                                     |
| `[`                  | Écrire la conversation complète dans le scrollback natif de votre terminal afin que `Cmd+F`, le mode copie tmux et d'autres outils natifs puissent la rechercher. Nécessite le [rendu plein écran](/docs/fr/fullscreen#search-and-review-the-conversation) |
| `v`                  | Écrire la conversation dans un fichier temporaire et l'ouvrir dans `$VISUAL` ou `$EDITOR`. Nécessite le [rendu plein écran](/docs/fr/fullscreen)                                                                                                           |
| `q`, `Ctrl+C`, `Esc` | Quitter la vue de transcription. Les trois peuvent être réaffectés via [`transcript:exit`](/docs/fr/keybindings)                                                                                                                                           |

<h3 id="voice-input">
  Entrée vocale
</h3>

| Raccourci                         | Description   | Notes                                                                                                                                                                                                                |
| :-------------------------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Maintenir ou appuyer sur `Espace` | Dictée vocale | Nécessite que la [dictée vocale](/docs/fr/voice-dictation) soit activée. Maintenez pour enregistrer, ou exécutez `/voice tap` pour le basculement par appui. [Réaffectable](/docs/fr/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Commandes
</h2>

Tapez `/` dans Claude Code pour voir toutes les commandes disponibles, ou tapez `/` suivi de n'importe quelles lettres pour filtrer. Le menu `/` affiche tout ce que vous pouvez invoquer : les commandes intégrées, les [skills](/docs/fr/skills) groupés et créés par l'utilisateur, et les commandes contribuées par les [plugins](/docs/fr/plugins) et les [serveurs MCP](/docs/fr/mcp#use-mcp-prompts-as-commands). Toutes les commandes intégrées ne sont pas visibles pour tous les utilisateurs car certaines dépendent de votre plateforme ou de votre plan.

Dans le [rendu en plein écran](/docs/fr/fullscreen#use-the-mouse), le menu de commande `/` et les listes de suggestions de fichiers `@` répondent également à la souris : le survol met en évidence une ligne et le clic l'accepte.

Consultez la [référence des commandes](/docs/fr/commands) pour la liste complète des commandes incluses dans Claude Code.

<h2 id="vim-editor-mode">
  Mode éditeur Vim
</h2>

Activez l'édition de style vim via `/config` → Mode éditeur.

<h3 id="mode-switching">
  Changement de mode
</h3>

| Commande | Action                                                   | Du mode        |
| :------- | :------------------------------------------------------- | :------------- |
| `Esc`    | Entrer en mode NORMAL                                    | INSERT, VISUAL |
| `i`      | Insérer avant le curseur                                 | NORMAL         |
| `I`      | Insérer au début de la ligne                             | NORMAL         |
| `a`      | Insérer après le curseur                                 | NORMAL         |
| `A`      | Insérer à la fin de la ligne                             | NORMAL         |
| `o`      | Ouvrir une ligne en dessous                              | NORMAL         |
| `O`      | Ouvrir une ligne au-dessus                               | NORMAL         |
| `v`      | Commencer une sélection visuelle caractère par caractère | NORMAL         |
| `V`      | Commencer une sélection visuelle ligne par ligne         | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Remapper les séquences de touches du mode INSERT
</h3>

Le paramètre [`vimInsertModeRemaps`](/docs/fr/settings#available-settings) mappe une séquence de deux touches en mode INSERT à Échap, donc un mappage comme `jj` vous ramène en mode NORMAL. Nécessite Claude Code v2.1.208 ou ultérieur.

L'exemple `~/.claude/settings.json` suivant active le mode vim et mappe `jj` à Échap :

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Chaque clé est exactement deux caractères imprimables tapés en séquence, et `"<Esc>"` est la seule cible prise en charge. Les entrées avec une longueur ou une cible différente sont ignorées.

Taper le premier caractère d'une séquence l'insère normalement. Appuyer sur le deuxième caractère dans une seconde supprime ce caractère en attente et bascule en mode NORMAL, ne laissant aucun caractère dans votre entrée. Après la fenêtre d'une seconde, ou si une touche différente suit, les deux caractères restent comme du texte littéral, vous pouvez donc toujours taper un mot contenant la séquence en faisant une pause entre les deux touches.

Claude Code lit ce paramètre à partir de votre fichier de paramètres utilisateur, du drapeau `--settings` et des [paramètres gérés](/docs/fr/permissions#managed-settings) uniquement. Les entrées dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet sont ignorées, donc un référentiel extrait ne peut pas remapper vos touches.

<h3 id="navigation-normal-mode">
  Navigation (mode NORMAL)
</h3>

| Commande        | Action                                                                                                                                                                                                               |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Déplacer gauche/bas/haut/droite                                                                                                                                                                                      |
| `Space`         | Déplacer à droite                                                                                                                                                                                                    |
| `w`             | Mot suivant                                                                                                                                                                                                          |
| `e`             | Fin du mot                                                                                                                                                                                                           |
| `b`             | Mot précédent                                                                                                                                                                                                        |
| `0`             | Début de la ligne                                                                                                                                                                                                    |
| `$`             | Fin de la ligne                                                                                                                                                                                                      |
| `^`             | Premier caractère non vide                                                                                                                                                                                           |
| `gg`            | Début de l'entrée                                                                                                                                                                                                    |
| `G`             | Fin de l'entrée                                                                                                                                                                                                      |
| `f{char}`       | Sauter à la prochaine occurrence du caractère                                                                                                                                                                        |
| `F{char}`       | Sauter à l'occurrence précédente du caractère                                                                                                                                                                        |
| `t{char}`       | Sauter juste avant la prochaine occurrence du caractère                                                                                                                                                              |
| `T{char}`       | Sauter juste après l'occurrence précédente du caractère                                                                                                                                                              |
| `;`             | Répéter le dernier mouvement f/F/t/T                                                                                                                                                                                 |
| `,`             | Répéter le dernier mouvement f/F/t/T en sens inverse                                                                                                                                                                 |
| `/`             | Ouvrir la recherche d'historique inversée, identique à `Ctrl+R`. À partir de v2.1.191, l'invite de recherche vide affiche un indice : appuyez sur `Esc` puis `i` puis `/` pour ouvrir le menu de commande à la place |

<Note>
  En mode normal vim, si le curseur est au début ou à la fin de l'entrée et ne peut pas se déplacer davantage, `j`/`k` et les flèches de direction naviguent dans l'historique des commandes à la place.
</Note>

<h3 id="editing-normal-mode">
  Édition (mode NORMAL)
</h3>

| Commande       | Action                                  |
| :------------- | :-------------------------------------- |
| `x`            | Supprimer le caractère                  |
| `dd`           | Supprimer la ligne                      |
| `D`            | Supprimer jusqu'à la fin de la ligne    |
| `dw`/`de`/`db` | Supprimer mot/jusqu'à la fin/en arrière |
| `cc`           | Changer la ligne                        |
| `C`            | Changer jusqu'à la fin de la ligne      |
| `cw`/`ce`/`cb` | Changer mot/jusqu'à la fin/en arrière   |
| `yy`/`Y`       | Copier la ligne                         |
| `yw`/`ye`/`yb` | Copier mot/jusqu'à la fin/en arrière    |
| `p`            | Coller après le curseur                 |
| `P`            | Coller avant le curseur                 |
| `>>`           | Indenter la ligne                       |
| `<<`           | Dédenter la ligne                       |
| `J`            | Joindre les lignes                      |
| `u`            | Annuler                                 |
| `.`            | Répéter la dernière modification        |

<h3 id="text-objects-normal-mode">
  Objets texte (mode NORMAL)
</h3>

Les objets texte fonctionnent avec les opérateurs comme `d`, `c` et `y` :

| Commande  | Action                                             |
| :-------- | :------------------------------------------------- |
| `iw`/`aw` | Mot intérieur/autour                               |
| `iW`/`aW` | MOT intérieur/autour (délimité par l'espace blanc) |
| `i"`/`a"` | Guillemets doubles intérieurs/autour               |
| `i'`/`a'` | Guillemets simples intérieurs/autour               |
| `i(`/`a(` | Parenthèses intérieures/autour                     |
| `i[`/`a[` | Crochets intérieurs/autour                         |
| `i{`/`a{` | Accolades intérieures/autour                       |

<h3 id="visual-mode">
  Mode visuel
</h3>

Appuyez sur `v` pour une sélection caractère par caractère ou `V` pour une sélection ligne par ligne. Les mouvements étendent la sélection, et les opérateurs agissent directement sur elle.

| Commande         | Action                                                                |
| :--------------- | :-------------------------------------------------------------------- |
| `d`/`x`          | Supprimer la sélection                                                |
| `y`              | Copier la sélection                                                   |
| `c`/`s`          | Changer la sélection                                                  |
| `p`              | Remplacer la sélection par le contenu du registre                     |
| `r{char}`        | Remplacer chaque caractère sélectionné par `{char}`                   |
| `~`/`u`/`U`      | Basculer, minuscules ou majuscules la sélection                       |
| `>`/`<`          | Indenter ou dédenter les lignes sélectionnées                         |
| `J`              | Joindre les lignes sélectionnées                                      |
| `o`              | Échanger le curseur et l'ancre                                        |
| `iw`/`aw`/`i"`/… | Sélectionner un objet texte                                           |
| `v`/`V`          | Basculer entre caractère par caractère et ligne par ligne, ou quitter |

Le mode visuel par bloc avec `Ctrl+V` n'est pas pris en charge.

<h2 id="command-history">
  Historique des commandes
</h2>

Claude Code maintient l'historique des commandes pour la session actuelle :

* L'historique des entrées est stocké par répertoire de travail
* L'historique des entrées se réinitialise lorsque vous exécutez `/clear` pour démarrer une nouvelle session. La conversation de la session précédente est conservée et peut être reprise.
* Soumettre la même invite deux fois de suite enregistre une seule entrée d'historique, donc appuyer sur Haut accède à l'invite distincte précédente
* Utilisez les flèches Haut/Bas pour naviguer (voir les raccourcis clavier ci-dessus)
* L'expansion de l'historique avec `!` est désactivée par défaut

<h3 id="reverse-search-with-ctrl-r">
  Recherche inversée avec Ctrl+R
</h3>

Appuyez sur `Ctrl+R` pour rechercher de manière interactive dans votre historique de commandes :

1. **Démarrer la recherche** : appuyez sur `Ctrl+R` pour activer la recherche d'historique inversée
2. **Tapez la requête** : entrez le texte à rechercher dans les commandes précédentes. Le terme de recherche est mis en évidence dans les résultats correspondants
3. **Naviguer dans les correspondances** : appuyez à nouveau sur `Ctrl+R` pour parcourir les correspondances plus anciennes
4. **Changer la portée** : la recherche s'applique par défaut aux invites de tous les projets. Appuyez sur `Ctrl+S` pour basculer la portée entre cette session, ce projet et tous les projets
5. **Accepter la correspondance** :
   * Appuyez sur `Tab` ou `Esc` pour accepter la correspondance actuelle et continuer l'édition
   * Appuyez sur `Entrée` pour accepter et exécuter la commande immédiatement
6. **Annuler la recherche** :
   * Appuyez sur `Ctrl+C` pour annuler et restaurer votre entrée d'origine
   * Appuyez sur `Retour arrière` sur une recherche vide pour annuler

La recherche charge les 100 invites uniques les plus récentes dans la portée sélectionnée, avec les doublons réduits à l'occurrence la plus récente. Les invites correspondantes s'affichent avec le terme de recherche mis en évidence, afin que vous puissiez trouver et réutiliser les entrées précédentes.

Accepter une correspondance ou annuler la recherche prend effet immédiatement, même si Claude Code charge toujours l'historique. Avant la v2.1.202, accepter ou annuler pendant ce chargement pouvait signaler une erreur interne.

<h2 id="background-bash-commands">
  Commandes bash en arrière-plan
</h2>

Claude Code prend en charge l'exécution de commandes bash en arrière-plan, ce qui vous permet de continuer à travailler pendant que les processus de longue durée s'exécutent.

<h3 id="how-backgrounding-works">
  Fonctionnement de la mise en arrière-plan
</h3>

Lorsque Claude Code exécute une commande en arrière-plan, il exécute la commande de manière asynchrone et retourne immédiatement un ID de tâche en arrière-plan. Claude Code peut répondre à de nouvelles invites pendant que la commande continue à s'exécuter en arrière-plan.

Pour exécuter les commandes en arrière-plan, vous pouvez soit :

* Inviter Claude Code à exécuter une commande en arrière-plan
* Appuyez sur `Ctrl+B` pour déplacer une invocation d'outil Bash régulière vers l'arrière-plan. Les utilisateurs Tmux doivent appuyer sur `Ctrl+B` deux fois en raison de la touche de préfixe de tmux.

**Caractéristiques clés :**

* La sortie est écrite dans un fichier et Claude peut la récupérer à l'aide de l'outil Read
* Les tâches en arrière-plan ont des ID uniques pour le suivi et la récupération de la sortie
* Les tâches en arrière-plan sont automatiquement nettoyées lorsque Claude Code se ferme. Mettre en arrière-plan la session au lieu de la fermer les confie à la session en arrière-plan, où elles continuent à s'exécuter. Consultez [mettre en arrière-plan une session en cours d'exécution](/docs/fr/agent-view#from-inside-a-session)
* Les tâches en arrière-plan sont automatiquement terminées si la sortie dépasse 5 Go, avec une note dans stderr expliquant pourquoi
* À partir de la v2.1.193, sur macOS et Linux, les tâches en arrière-plan en cours d'exécution sont terminées lorsque le système d'exploitation signale une pression mémoire, à condition que la session soit restée inactive pendant au moins 30 minutes sans aucun tour ou sous-agent en cours d'exécution. Définissez [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/fr/env-vars) sur `1` pour désactiver cette fonctionnalité

Pour désactiver toutes les fonctionnalités de tâche en arrière-plan, définissez la variable d'environnement `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` sur `1`. Consultez [Variables d'environnement](/docs/fr/env-vars) pour plus de détails.

**Commandes couramment mises en arrière-plan :**

* Outils de construction (webpack, vite, make)
* Gestionnaires de paquets (npm, yarn, pnpm)
* Exécuteurs de tests (jest, pytest)
* Serveurs de développement
* Processus de longue durée (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Mode shell avec le préfixe `!`
</h3>

Exécutez les commandes shell directement sans passer par Claude en préfixant votre entrée avec `!` :

```bash theme={null}
! npm test
! git status
! ls -la
```

Mode shell :

* Ajoute la commande et sa sortie au contexte de la conversation
* Affiche la progression et la sortie en temps réel
* Prend en charge la même mise en arrière-plan `Ctrl+B` pour les commandes de longue durée
* Ne nécessite pas que Claude interprète ou approuve la commande
* Prend en charge l'autocomplétion basée sur l'historique : tapez une commande partielle et appuyez sur `Tab` pour compléter à partir des commandes `!` précédentes du projet actuel
* À partir de la v2.1.193 sur toutes les plateformes, prend en charge l'autocomplétion de chemin de fichier en direct : tapez un jeton contenant une barre oblique, tel que `./src/` ou `~/`, pour voir une liste déroulante des fichiers et répertoires correspondants, puis appuyez sur `Tab` pour accepter. Utilisez les barres obliques sur Windows également ; la liste déroulante est déclenchée par `/`, pas `\`
* Quittez avec `Échap`, `Retour arrière` ou `Ctrl+U` sur une invite vide
* Coller du texte commençant par `!` dans une invite vide entre en mode shell automatiquement, correspondant au comportement du texte tapé `!`

À partir de la v2.1.186, Claude répond automatiquement à la sortie de la commande une fois qu'elle arrive dans la transcription, vous pouvez donc exécuter `! npm test` et obtenir une explication des défaillances sans une deuxième invite. La réponse coûte la même chose que l'envoi d'une invite normale. Pour restaurer le comportement antérieur où la sortie est ajoutée au contexte sans réponse, définissez [`respondToBashCommands`](/docs/fr/settings#available-settings) sur `false` dans `settings.json`. Avant la v2.1.186, le mode shell ajoutait toujours la sortie au contexte sans réponse.

Ceci est utile pour les opérations shell rapides tout en maintenant le contexte de la conversation.

<h2 id="prompt-suggestions">
  Suggestions d'invite
</h2>

Lorsque vous ouvrez une session pour la première fois, une commande d'exemple grisée apparaît dans l'entrée d'invite pour vous aider à démarrer. Claude Code la choisit à partir de l'historique git de votre projet, elle reflète donc les fichiers sur lesquels vous avez travaillé récemment.

Après la réponse de Claude, les suggestions continuent à apparaître en fonction de votre historique de conversation, comme une étape de suivi d'une demande en plusieurs parties ou une continuation naturelle de votre flux de travail.

* Appuyez sur `Tab` ou `Flèche droite` pour placer la suggestion dans l'entrée d'invite, puis `Entrée` pour soumettre
* Commencez à taper pour la rejeter

La suggestion s'exécute en tant que demande en arrière-plan qui réutilise le cache d'invite de la conversation parent, le coût supplémentaire est donc minimal. Claude Code ignore la génération de suggestions lorsque le cache est froid pour éviter les coûts inutiles.

Les suggestions sont automatiquement ignorées après le premier tour d'une conversation et en Plan Mode. En print mode, elles sont désactivées par défaut. Passez [`--prompt-suggestions`](/docs/fr/cli-reference#cli-flags) avec `--output-format stream-json --verbose` pour émettre un message `prompt_suggestion` après chaque tour à la place.

Pour désactiver complètement les suggestions d'invite, définissez la variable d'environnement ou basculez le paramètre dans `/config` :

```bash theme={null}
export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
```

<h2 id="side-questions-with-/btw">
  Questions latérales avec /btw
</h2>

Utilisez `/btw` pour poser une question rapide sur votre travail actuel sans l'ajouter à l'historique de la conversation. Ceci est utile lorsque vous voulez une réponse rapide mais que vous ne voulez pas encombrer le contexte principal ou détourner Claude d'une tâche de longue durée.

```
/btw what was the name of that config file again?
```

Les questions latérales ont une visibilité complète sur la conversation actuelle, vous pouvez donc poser des questions sur le code que Claude a déjà lu, les décisions qu'il a prises plus tôt, ou n'importe quoi d'autre de la session. La question et la réponse sont éphémères : elles apparaissent dans une superposition rejetable et n'entrent jamais dans l'historique de la conversation.

* **Disponible pendant que Claude travaille** : vous pouvez exécuter `/btw` même pendant que Claude traite une réponse. La question latérale s'exécute indépendamment et n'interrompt pas le tour principal.
* **Pas d'accès aux outils** : les questions latérales répondent uniquement à partir de ce qui est déjà en contexte. Claude ne peut pas lire les fichiers, exécuter les commandes ou effectuer de recherches lorsqu'il répond à une question latérale.
* **Réponse unique** : il n'y a pas de tours de suivi dans la superposition. Pour continuer le fil, divisez-le en sa propre session avec `f`.
* **Coût faible** : la question latérale réutilise le cache d'invite de la conversation parent, le coût supplémentaire est donc minimal.

Les questions latérales antérieures de la même session apparaissent sous forme de liste estompée au-dessus de la réponse actuelle. Elles restent en dehors de l'historique de la conversation mais restent visibles dans la superposition jusqu'à ce que vous les effaciez.

Une fois que la réponse apparaît, la superposition accepte ces touches.

| Touche                      | Action                                                                                                                                                                                                                                                                                                                                 |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Espace`, `Entrée`, `Échap` | Rejeter la réponse et revenir à l'invite                                                                                                                                                                                                                                                                                               |
| `Haut` / `Bas`              | Faire défiler la réponse                                                                                                                                                                                                                                                                                                               |
| `Gauche` / `Droite`         | Naviguer entre cette réponse et vos réponses `/btw` antérieures de la session. `Gauche` se déplace vers les réponses plus anciennes et `Droite` revient vers la réponse actuelle. Nécessite Claude Code v2.1.187 ou version ultérieure                                                                                                 |
| `c`                         | Copier la réponse dans votre presse-papiers en tant que Markdown brut. Utilisez ceci au lieu de la sélection à la souris, qui capture le rendu du terminal avec retour à la ligne plutôt que le texte source                                                                                                                           |
| `f`                         | Diviser en une nouvelle session. La division hérite de la conversation parent plus cette question et réponse en tant que tours de transcription réels, vous pouvez donc continuer avec un accès complet aux outils. La session d'origine est conservée sous [`/resume`](/docs/fr/commands). Disponible uniquement dans les sessions locales |
| `x`                         | Effacer la liste des échanges `/btw` antérieurs affichés au-dessus de la réponse actuelle                                                                                                                                                                                                                                              |

`/btw` est l'inverse d'un [subagent](/docs/fr/sub-agents) : il voit votre conversation complète mais n'a pas d'outils, tandis qu'un subagent a tous les outils mais commence avec un contexte vide. Utilisez `/btw` pour poser des questions sur ce que Claude sait déjà de cette session ; utilisez un subagent pour aller découvrir quelque chose de nouveau.

<h2 id="task-list">
  Liste des tâches
</h2>

La liste des tâches est la liste de contrôle de Claude : des éléments que Claude a créés pour planifier un travail en plusieurs étapes, avec des indicateurs montrant ce qui est en attente, en cours ou terminé. Elle est distincte de la vue des tâches en arrière-plan. Pour voir les shells en cours d'exécution et les sous-agents, utilisez [`/tasks`](/docs/fr/commands) à la place.

* Appuyez sur `Ctrl+T` pour basculer l'affichage de la liste des tâches. L'affichage montre jusqu'à cinq tâches à la fois. Lorsque Claude n'a pas encore créé d'éléments de liste de contrôle, le basculement n'a aucun effet visible car il n'y a rien à afficher
* Pour voir toutes les tâches ou les effacer, demandez directement à Claude : « show me all tasks » ou « clear all tasks »
* Les tâches persistent lors des compactions de contexte, aidant Claude à rester organisé sur les projets plus importants
* Pour partager une liste de tâches entre les sessions, définissez `CLAUDE_CODE_TASK_LIST_ID` pour utiliser un répertoire nommé dans `~/.claude/tasks/` : `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Récapitulatif de session
</h2>

Lorsque vous revenez au terminal après vous être éloigné, Claude Code affiche un récapitulatif d'une ligne de ce qui s'est passé dans la session jusqu'à présent. Le récapitulatif se génère en arrière-plan une fois qu'au moins trois minutes se sont écoulées depuis le dernier tour complété et que le terminal n'est pas en focus, afin qu'il soit prêt lorsque vous revenez. Les récapitulatifs n'apparaissent qu'une fois que la session a au moins trois tours, et jamais deux fois de suite.

Exécutez `/recap` pour générer un résumé à la demande. Pour désactiver les récapitulatifs automatiques, ouvrez `/config` et désactivez **Récapitulatif de session**.

Le récapitulatif de session est activé par défaut pour tous les plans et fournisseurs. Le récapitulatif est toujours ignoré en mode non interactif.

<h2 id="pr-review-status">
  Statut de révision PR
</h2>

Lorsque vous travaillez sur une branche avec une demande de tirage ouverte, Claude Code affiche un lien PR cliquable dans le pied de page, par exemple « PR #446 ». Le lien a un soulignement coloré indiquant l'état de la révision :

* Vert : approuvé
* Jaune : en attente de révision
* Rouge : modifications demandées
* Gris : brouillon

Le badge disparaît une fois que la demande de tirage est fusionnée ou fermée. `Cmd+clic` (macOS) ou `Ctrl+clic` (Windows/Linux) sur le lien pour ouvrir la demande de tirage dans votre navigateur. Le statut se met à jour toutes les 60 secondes, et immédiatement après l'exécution d'une commande `gh pr` ou `git push` dans la session.

<Note>
  Le statut PR nécessite que le CLI `gh` soit installé et authentifié (`gh auth login`).
</Note>

<h2 id="see-also">
  Voir aussi
</h2>

* [Skills](/docs/fr/skills) - Invites personnalisées et flux de travail
* [Checkpointing](/docs/fr/checkpointing) - Rembobiner les modifications de Claude et restaurer les états précédents
* [Référence CLI](/docs/fr/cli-reference) - Drapeaux et options de ligne de commande
* [Paramètres](/docs/fr/settings) - Options de configuration
* [Gestion de la mémoire](/docs/fr/memory) - Gestion des fichiers CLAUDE.md
