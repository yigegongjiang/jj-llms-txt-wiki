> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personnaliser les raccourcis clavier

> Personnalisez les raccourcis clavier dans Claude Code avec un fichier de configuration des liaisons de touches.

Claude Code prend en charge les raccourcis clavier personnalisables. Exécutez `/keybindings` pour créer ou ouvrir votre fichier de configuration à `~/.claude/keybindings.json`.

<h2 id="configuration-file">
  Fichier de configuration
</h2>

Le fichier de configuration des liaisons de touches est un objet avec un tableau `bindings`. Chaque bloc spécifie un contexte et une carte des séquences de touches aux actions.

<Note>Les modifications du fichier keybindings sont automatiquement détectées et appliquées sans redémarrer Claude Code.</Note>

| Champ      | Description                                                     |
| :--------- | :-------------------------------------------------------------- |
| `$schema`  | URL du schéma JSON optionnel pour l'autocomplétion de l'éditeur |
| `$docs`    | URL de documentation optionnelle                                |
| `bindings` | Tableau de blocs de liaison par contexte                        |

Cet exemple lie `Ctrl+E` pour ouvrir un éditeur externe dans le contexte de chat, et délié `Ctrl+U` :

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/fr/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  Contextes
</h2>

Chaque bloc de liaison spécifie un **contexte** où les liaisons s'appliquent :

| Contexte          | Description                                                                   |
| :---------------- | :---------------------------------------------------------------------------- |
| `Global`          | S'applique partout dans l'application                                         |
| `Chat`            | Zone de saisie de chat principale                                             |
| `Autocomplete`    | Le menu d'autocomplétion est ouvert                                           |
| `Settings`        | Menu des paramètres                                                           |
| `Confirmation`    | Dialogues de permission et de confirmation                                    |
| `Tabs`            | Composants de navigation par onglets                                          |
| `Help`            | Le menu d'aide est visible                                                    |
| `Transcript`      | Visionneuse de transcription                                                  |
| `HistorySearch`   | Mode de recherche d'historique (Ctrl+R)                                       |
| `Task`            | Une tâche de fond est en cours d'exécution                                    |
| `ThemePicker`     | Dialogue du sélecteur de thème                                                |
| `Attachments`     | Navigation de la pièce jointe d'image dans les dialogues de sélection         |
| `Footer`          | Navigation de l'indicateur de pied de page (tâches, équipes, diff, artefacts) |
| `MessageSelector` | Sélection de message du dialogue de rembobinage et de résumé                  |
| `DiffDialog`      | Navigation de la visionneuse de diff                                          |
| `DiffPanel`       | Le [panneau de diff](/docs/fr/interactive-mode#diff-panel) est ouvert              |
| `ModelPicker`     | Niveau d'effort du sélecteur de modèle                                        |
| `EffortSlider`    | Curseur d'effort ouvert par `/effort`                                         |
| `Select`          | Composants génériques de sélection/liste                                      |
| `Plugin`          | Dialogue du plugin (parcourir, découvrir, gérer)                              |
| `Agents`          | [Vue Agent](/docs/fr/agent-view) (`claude agents`)                                 |
| `Scroll`          | Défilement de la conversation et sélection de texte en mode plein écran       |

Avant v2.1.205, un contexte `Doctor` et une action `doctor:fix` existaient pour l'écran de diagnostics `/doctor`.

<h2 id="available-actions">
  Actions disponibles
</h2>

Les actions suivent un format `namespace:action`, tel que `chat:submit` pour envoyer un message ou `app:toggleTodos` pour afficher la liste des tâches. Chaque contexte a des actions spécifiques disponibles.

<h3 id="app-actions">
  Actions d'application
</h3>

Actions disponibles dans le contexte `Global` :

| Action                 | Par défaut | Description                                                                                                                      |
| :--------------------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C     | Annuler l'opération en cours                                                                                                     |
| `app:exit`             | Ctrl+D     | Quitter Claude Code. Appuyez deux fois en 800 ms pour confirmer                                                                  |
| `app:redraw`           | (non lié)  | Forcer le redessinage du terminal                                                                                                |
| `app:toggleTodos`      | Ctrl+T     | Basculer la visibilité de la liste des tâches de Claude. Ce n'est pas la vue des tâches en arrière-plan [`/tasks`](/docs/fr/commands) |
| `app:toggleTranscript` | Ctrl+O     | Basculer la transcription détaillée                                                                                              |

<h3 id="history-actions">
  Actions d'historique
</h3>

Actions pour naviguer dans l'historique des commandes :

| Action             | Par défaut | Description                      |
| :----------------- | :--------- | :------------------------------- |
| `history:search`   | Ctrl+R     | Ouvrir la recherche d'historique |
| `history:previous` | Haut       | Élément d'historique précédent   |
| `history:next`     | Bas        | Élément d'historique suivant     |

<h3 id="chat-actions">
  Actions de chat
</h3>

Actions disponibles dans le contexte `Chat` :

| Action                | Par défaut                         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------------- | :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Échappement                        | Annuler l'entrée actuelle                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `chat:clearInput`     | Ctrl+L                             | Forcer un redessinage complet de l'écran, en préservant l'entrée et la conversation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `chat:clearScreen`    | Cmd+K                              | Identique à `chat:clearInput`. Voir [Effacer la conversation](/docs/fr/fullscreen#clear-the-conversation) pour savoir comment Cmd+K se comporte sur iTerm2 et Terminal.app                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `chat:killAgents`     | Ctrl+X Ctrl+K                      | Arrêter tous les [sous-agents en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) en cours d'exécution dans cette session et désactiver les [réponses automatiques aux artefacts](/docs/fr/artifacts#let-claude-reply-to-comments-on-its-own) pour le reste de celle-ci                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `chat:cycleMode`      | Maj+Tab\*                          | Cycler les modes de permission                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:modelPicker`    | Meta+P                             | Ouvrir le sélecteur de modèle                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `chat:fastMode`       | Meta+O                             | Basculer le mode rapide                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:thinkingToggle` | Meta+T                             | Basculer la réflexion étendue                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `chat:submit`         | Entrée                             | Soumettre le message                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:queueSubmit`    | Ctrl+X Entrée                      | Soumettre le message, marqué pour attendre son tour : pendant que Claude travaille, Claude Code [le met en file d'attente](/docs/fr/interactive-mode#queue-messages-while-claude-works) et n'interrompt jamais le tour. Contrairement à `chat:submit`, il soumet le brouillon même si une suggestion d'autocomplétion est en surbrillance. Nécessite v2.1.247 ou ultérieure                                                                                                                                                                                                                                                                                                                                                                   |
| `chat:sendNow`        | Ctrl+Entrée, Ctrl+X Ctrl+S         | Envoyer vos [messages en file d'attente](/docs/fr/interactive-mode#queue-messages-while-claude-works) et votre brouillon avec eux immédiatement. [Quand Claude Code envoie ce que vous avez mis en file d'attente](/docs/fr/interactive-mode#when-claude-code-sends-what-you-queued) couvre ce qui se passe au tour sur lequel Claude travaille. Quand rien ne s'exécute, la touche soumet le brouillon, et en [mode shell](/docs/fr/interactive-mode#shell-mode-with-prefix) elle met uniquement la commande en file d'attente. Les terminaux qui ne signalent pas les touches étendues livrent `Ctrl+Entrée` comme simple `Entrée`, donc `Ctrl+X Ctrl+S` est la liaison qui fonctionne dans n'importe quel terminal. Nécessite v2.1.275 ou ultérieure |
| `chat:newline`        | Ctrl+J                             | Insérer une nouvelle ligne sans soumettre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `chat:undo`           | Ctrl+\_, Ctrl+Maj+-                | Annuler la dernière action                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E              | Ouvrir dans un éditeur externe. La [saisie de dispatch de la vue agent](/docs/fr/agent-view#keyboard-shortcuts) suit également les liaisons à un seul trait de ce raccourci                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `chat:stash`          | Ctrl+S                             | Mettre en cache l'invite actuelle                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `chat:imagePaste`     | Ctrl+V (Alt+V sous Windows et WSL) | Coller une image depuis le presse-papiers. Sur WSL, les deux raccourcis sont liés par défaut                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

\*Sous Windows sans mode VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), la valeur par défaut est Meta+M.

<h3 id="autocomplete-actions">
  Actions d'autocomplétion
</h3>

Actions disponibles dans le contexte `Autocomplete` :

| Action                  | Par défaut  | Description            |
| :---------------------- | :---------- | :--------------------- |
| `autocomplete:accept`   | Tab         | Accepter la suggestion |
| `autocomplete:dismiss`  | Échappement | Fermer le menu         |
| `autocomplete:previous` | Haut        | Suggestion précédente  |
| `autocomplete:next`     | Bas         | Suggestion suivante    |

<h3 id="confirmation-actions">
  Actions de confirmation
</h3>

Actions disponibles dans le contexte `Confirmation` :

| Action                  | Par défaut  | Description                                                                                                                                                                                                                                                                                                           |
| :---------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`           | Entrée      | Confirmer l'action                                                                                                                                                                                                                                                                                                    |
| `confirm:no`            | Échappement | Refuser l'action                                                                                                                                                                                                                                                                                                      |
| `confirm:previous`      | Haut        | Option précédente                                                                                                                                                                                                                                                                                                     |
| `confirm:next`          | Bas         | Option suivante                                                                                                                                                                                                                                                                                                       |
| `confirm:nextField`     | Tab         | Champ suivant                                                                                                                                                                                                                                                                                                         |
| `confirm:previousField` | (non lié)   | Champ précédent                                                                                                                                                                                                                                                                                                       |
| `confirm:toggle`        | Espace      | Basculer la sélection                                                                                                                                                                                                                                                                                                 |
| `confirm:cycleMode`     | Maj+Tab\*   | Cycler les modes de permission. Sur une invite de permission de fichier, ferme un [champ de commentaire](/docs/fr/permissions#add-a-comment-when-you-answer-a-permission-prompt) ouvert ; sans champ ouvert, sélectionne l'option qui autorise l'action pour le reste de la session, lorsque l'invite propose cette option |

\*Sous Windows sans mode VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), la valeur par défaut est Meta+M.

Avant v2.1.257, une action `confirm:toggleExplanation`, liée à `Ctrl+E` par défaut, affichait une explication générée par le modèle de la commande sur les invites de permission Bash et PowerShell.

Les dialogues utilisent `confirm:yes` et `confirm:no` pour accepter et annuler même lorsqu'ils ne posent pas une question oui ou non. Si vous liez une simple lettre telle que `y` ou `n` dans ce contexte, la lettre agit également sur les dialogues qui ne la montrent jamais comme clé. Un dialogue qui affiche `y` et `n` comme ses clés les lit lui-même et n'a besoin d'aucune liaison.

Cet exemple lie `y` à `confirm:yes` et `n` à `confirm:no` :

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

Avec ces liaisons, `y` et `n` tapent toujours comme des lettres pendant qu'un [champ de texte](#text-fields) a le focus.

Avant v2.1.280, `y` était également lié à `confirm:yes` et `n` à `confirm:no` par défaut. Si vous avez créé votre `keybindings.json` avec `/keybindings` avant v2.1.280, le fichier liste les deux liaisons et elles restent en vigueur jusqu'à ce que vous supprimiez ces deux lignes.

<h3 id="permission-actions">
  Actions de permission
</h3>

Actions disponibles dans le contexte `Confirmation` pour les dialogues de permission :

| Action                   | Par défaut | Description                                                                                                                                                  |
| :----------------------- | :--------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission:toggleDebug` | (non lié)  | Basculer les informations de débogage de permission. La valeur par défaut précédente de Ctrl+D a été supprimée dans la v2.1.146 car elle masquait `app:exit` |

<h3 id="transcript-actions">
  Actions de transcription
</h3>

Actions disponibles dans le contexte `Transcript` :

| Action                     | Par défaut             | Description                             |
| :------------------------- | :--------------------- | :-------------------------------------- |
| `transcript:toggleShowAll` | Ctrl+E                 | Basculer l'affichage de tout le contenu |
| `transcript:exit`          | q, Ctrl+C, Échappement | Quitter la vue de transcription         |

`transcript:toggleShowAll` s'applique uniquement dans le rendu classique ; dans le [rendu plein écran](/docs/fr/fullscreen), la visionneuse de transcription n'offre pas de basculement d'affichage complet.

<h3 id="history-search-actions">
  Actions de recherche d'historique
</h3>

Actions disponibles dans le contexte `HistorySearch` :

| Action                     | Par défaut       | Description                                 |
| :------------------------- | :--------------- | :------------------------------------------ |
| `historySearch:next`       | Ctrl+R           | Correspondance suivante                     |
| `historySearch:accept`     | Échappement, Tab | Accepter la sélection                       |
| `historySearch:cancel`     | Ctrl+C           | Annuler la recherche                        |
| `historySearch:execute`    | Entrée           | Exécuter la commande sélectionnée           |
| `historySearch:cycleScope` | Ctrl+S           | Cycler la portée : session, projet, partout |

Les valeurs par défaut de `historySearch:next`, `historySearch:accept`, `historySearch:cancel` et `historySearch:execute` s'appliquent à la recherche d'historique en ligne dans le rendu classique, qui recherche toujours les invites de tous les projets. `historySearch:cycleScope` prend effet uniquement dans le [rendu plein écran](/docs/fr/fullscreen), où `Ctrl+R` ouvre un dialogue de recherche à la place et `Ctrl+S` bascule sa portée. Les autres touches du dialogue sont fixes et ne peuvent pas être reliées : `Entrée` ou `Tab` place la correspondance en surbrillance dans l'entrée d'invite et `Échap` annule.

<h3 id="task-actions">
  Actions de tâche
</h3>

Actions disponibles dans le contexte `Task` :

| Action            | Par défaut            | Description                                                                                       |
| :---------------- | :-------------------- | :------------------------------------------------------------------------------------------------ |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Mettre la tâche actuelle en arrière-plan. L'accord Ctrl+X Ctrl+B évite le conflit de préfixe tmux |

<h3 id="theme-actions">
  Actions de thème
</h3>

Actions disponibles dans le contexte `ThemePicker` :

| Action                           | Par défaut | Description                       |
| :------------------------------- | :--------- | :-------------------------------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T     | Basculer la coloration syntaxique |

<h3 id="help-actions">
  Actions d'aide
</h3>

Actions disponibles dans le contexte `Help` :

| Action         | Par défaut  | Description           |
| :------------- | :---------- | :-------------------- |
| `help:dismiss` | Échappement | Fermer le menu d'aide |

<h3 id="tabs-actions">
  Actions d'onglets
</h3>

Actions disponibles dans le contexte `Tabs` :

| Action          | Par défaut      | Description      |
| :-------------- | :-------------- | :--------------- |
| `tabs:next`     | Tab, Droite     | Onglet suivant   |
| `tabs:previous` | Maj+Tab, Gauche | Onglet précédent |

<h3 id="attachments-actions">
  Actions de pièces jointes
</h3>

Actions disponibles dans le contexte `Attachments` :

| Action                 | Par défaut                | Description                              |
| :--------------------- | :------------------------ | :--------------------------------------- |
| `attachments:next`     | Droite                    | Pièce jointe suivante                    |
| `attachments:previous` | Gauche                    | Pièce jointe précédente                  |
| `attachments:remove`   | Retour arrière, Supprimer | Supprimer la pièce jointe sélectionnée   |
| `attachments:exit`     | Bas, Échappement          | Quitter la navigation des pièces jointes |

<h3 id="footer-actions">
  Actions de pied de page
</h3>

Actions disponibles dans le contexte `Footer` :

| Action                  | Par défaut                | Description                                                                                                                                                                                                                |
| :---------------------- | :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `footer:next`           | Droite                    | Élément de pied de page suivant                                                                                                                                                                                            |
| `footer:previous`       | Gauche                    | Élément de pied de page précédent                                                                                                                                                                                          |
| `footer:up`             | Haut                      | Naviguer vers le haut dans le pied de page (désélectionne en haut)                                                                                                                                                         |
| `footer:down`           | Bas                       | Naviguer vers le bas dans le pied de page                                                                                                                                                                                  |
| `footer:openSelected`   | Entrée                    | Ouvrir l'élément de pied de page sélectionné                                                                                                                                                                               |
| `footer:clearSelection` | Échappement               | Effacer la sélection du pied de page                                                                                                                                                                                       |
| `footer:dismiss`        | Retour arrière, Supprimer | Rejeter le lien [artefact](/docs/fr/artifacts) sélectionné du pied de page ; l'artefact publié lui-même n'est pas affecté. Sur d'autres lignes du pied de page, ces touches n'ont aucun effet. Nécessite v2.1.217 ou ultérieure |

Pendant qu'un élément de pied de page est sélectionné, tel qu'une ligne dans le panneau agent sous l'invite, `Entrée` l'ouvre même lorsque vous reliez `Entrée` dans le contexte `Chat` à `chat:queueSubmit` ou `chat:newline`.

Les liaisons `Chat` sur les touches que le contexte `Footer` ne lie pas, telles que `Maj+Tab` pour `chat:cycleMode`, continuent de fonctionner pendant qu'un élément est sélectionné.

<h3 id="message-selector-actions">
  Actions du sélecteur de message
</h3>

Actions disponibles dans le contexte `MessageSelector` :

| Action                   | Par défaut                            | Description                         |
| :----------------------- | :------------------------------------ | :---------------------------------- |
| `messageSelector:up`     | Haut, K, Ctrl+P                       | Déplacer vers le haut dans la liste |
| `messageSelector:down`   | Bas, J, Ctrl+N                        | Déplacer vers le bas dans la liste  |
| `messageSelector:top`    | Ctrl+Haut, Maj+Haut, Meta+Haut, Maj+K | Sauter au début                     |
| `messageSelector:bottom` | Ctrl+Bas, Maj+Bas, Meta+Bas, Maj+J    | Sauter à la fin                     |
| `messageSelector:select` | Entrée                                | Sélectionner le message             |

<h3 id="diff-actions">
  Actions de diff
</h3>

Actions disponibles dans le contexte `DiffDialog` :

| Action                | Par défaut  | Description                                                                                                                                                                                                  |
| :-------------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Échappement | Fermer la visionneuse de diff ; à partir de la vue détaillée, revient à la liste des fichiers à la place                                                                                                     |
| `diff:previousSource` | Gauche      | Source de diff précédente                                                                                                                                                                                    |
| `diff:nextSource`     | Droite      | Source de diff suivante                                                                                                                                                                                      |
| `diff:previousFile`   | Haut, K     | Fichier précédent dans la liste des fichiers ; faire défiler vers le haut d'une ligne dans la vue détaillée                                                                                                  |
| `diff:nextFile`       | Bas, J      | Fichier suivant dans la liste des fichiers ; faire défiler vers le bas d'une ligne dans la vue détaillée                                                                                                     |
| `diff:viewDetails`    | Entrée      | Afficher les détails du diff                                                                                                                                                                                 |
| `diff:back`           | (non lié)   | Revenir en arrière dans la visionneuse de diff. Échappement effectue l'action de retour via `diff:dismiss`. La valeur par défaut précédente de Gauche dans la vue détaillée a été supprimée dans la v2.1.203 |

La vue détaillée du diff lie également les touches de style paginateur aux [actions de défilement](#scroll-actions) standard. Ces liaisons font partie du contexte `DiffDialog` et s'appliquent uniquement dans la vue détaillée ; les valeurs par défaut du contexte `Scroll` listées sous [Actions de défilement](#scroll-actions) sont inchangées.

| Action                | Par défaut    | Description                                                       |
| :-------------------- | :------------ | :---------------------------------------------------------------- |
| `scroll:pageUp`       | PageUp        | Faire défiler vers le haut de la moitié de la fenêtre d'affichage |
| `scroll:pageDown`     | PageDown      | Faire défiler vers le bas de la moitié de la fenêtre d'affichage  |
| `scroll:fullPageUp`   | Maj+Espace, B | Faire défiler vers le haut d'une fenêtre d'affichage complète     |
| `scroll:fullPageDown` | Espace        | Faire défiler vers le bas d'une fenêtre d'affichage complète      |
| `scroll:top`          | G, Home       | Sauter au début                                                   |
| `scroll:bottom`       | Maj+G, End    | Sauter à la fin                                                   |

<h3 id="diff-panel-actions">
  Actions du panneau de diff
</h3>

Actions pour le [panneau de diff](/docs/fr/interactive-mode#diff-panel) que `/diff` ouvre dans le rendu plein écran. `app:cycleDiffBase` est dans le contexte `DiffPanel`, qui est actif pendant que le panneau est ouvert ; les autres sont `Global`. Le panneau nécessite Claude Code v2.1.260 ou ultérieure.

| Action                      | Par défaut           | Description                                                                         |
| :-------------------------- | :------------------- | :---------------------------------------------------------------------------------- |
| `app:toggleReplTab`         | (non lié)            | Ouvrir ou fermer le panneau de diff, identique à l'exécution de `/diff`             |
| `app:cycleDiffBase`         | Ctrl+X B             | Cycler la base de comparaison du panneau : cette session, non validée, puis branche |
| `app:diffFileListUp`        | Ctrl+Haut, Meta+Haut | Faire défiler la liste des fichiers du panneau vers le haut lorsqu'elle déborde     |
| `app:diffFileListDown`      | Ctrl+Bas, Meta+Bas   | Faire défiler la liste des fichiers du panneau vers le bas lorsqu'elle déborde      |
| `app:toggleDiffNoiseFilter` | (non lié)            | Afficher ou masquer les fichiers de test et générés dans le panneau                 |
| `app:toggleDiffPreSession`  | (non lié)            | Développer ou réduire les modifications antérieures à cette session                 |

<h3 id="model-picker-actions">
  Actions du sélecteur de modèle
</h3>

Actions disponibles dans le contexte `ModelPicker` :

| Action                        | Par défaut | Description                                                    |
| :---------------------------- | :--------- | :------------------------------------------------------------- |
| `modelPicker:decreaseEffort`  | Gauche     | Diminuer le niveau d'effort                                    |
| `modelPicker:increaseEffort`  | Droite     | Augmenter le niveau d'effort                                   |
| `modelPicker:thisSessionOnly` | s          | Appliquer le modèle en surbrillance à cette session uniquement |

<h3 id="effort-slider-actions">
  Actions du curseur d'effort
</h3>

Actions disponibles dans le contexte `EffortSlider`, le curseur qui s'ouvre lorsque vous exécutez `/effort` sans arguments. Les touches Gauche, Droite, Entrée et Échappement du curseur ne peuvent pas être reliées.

| Action                         | Par défaut | Description                                                                                                                             |
| :----------------------------- | :--------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| `effortSlider:thisSessionOnly` | s          | Appliquer le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) ciblé à cette session uniquement. Nécessite v2.1.257 ou ultérieure |

<h3 id="select-actions">
  Actions de sélection
</h3>

Actions disponibles dans le contexte `Select` :

| Action            | Par défaut      | Description                                |
| :---------------- | :-------------- | :----------------------------------------- |
| `select:next`     | Bas, J, Ctrl+N  | Option suivante                            |
| `select:previous` | Haut, K, Ctrl+P | Option précédente                          |
| `select:pageUp`   | PageUp          | Déplacer vers le haut d'une page d'options |
| `select:pageDown` | PageDown        | Déplacer vers le bas d'une page d'options  |
| `select:first`    | Home            | Première option                            |
| `select:last`     | End             | Dernière option                            |
| `select:accept`   | Entrée          | Accepter la sélection                      |
| `select:cancel`   | Échappement     | Annuler la sélection                       |

Claude Code applique vos liaisons `select:pageUp`, `select:pageDown`, `select:first` et `select:last` dans le menu `/skills`. Dans la plupart des autres listes, telles que le sélecteur `/model`, vos liaisons `select:first` et `select:last` s'appliquent. PageUp et PageDown parcourent les options dans ces listes indépendamment de vos liaisons.

Avant v2.1.280, ces autres listes ignoraient Home, End, et vos liaisons `select:first` et `select:last`.

<h3 id="plugin-actions">
  Actions de plugin
</h3>

Actions disponibles dans le contexte `Plugin` :

| Action            | Par défaut | Description                                                                                       |
| :---------------- | :--------- | :------------------------------------------------------------------------------------------------ |
| `plugin:toggle`   | Espace     | Basculer la sélection du plugin                                                                   |
| `plugin:install`  | I          | Installer les plugins sélectionnés                                                                |
| `plugin:favorite` | F          | Marquer le plugin sélectionné comme favori pour qu'il soit trié près du haut de l'onglet Installé |

<h3 id="settings-actions">
  Actions des paramètres
</h3>

Actions disponibles dans le contexte `Settings`. Les actions `select:accept` et `confirm:no` sont réutilisées à partir des contextes [Sélection](#select-actions) et [Confirmation](#confirmation-actions) avec un comportement spécifique aux paramètres : les modifications s'appliquent à chaque paramètre dès que vous le modifiez, donc Échappement ferme le panneau avec vos modifications enregistrées plutôt que de refuser.

| Action            | Par défaut     | Description                                                    |
| :---------------- | :------------- | :------------------------------------------------------------- |
| `settings:search` | /              | Entrer en mode de recherche                                    |
| `settings:retry`  | R              | Réessayer de charger les données d'utilisation en cas d'erreur |
| `select:accept`   | Entrée, Espace | Modifier le paramètre sélectionné ou ouvrir son sous-menu      |
| `confirm:no`      | Échappement    | Fermer le panneau. Les modifications sont déjà enregistrées    |

<h3 id="agents-actions">
  Actions des agents
</h3>

Actions disponibles dans le contexte `Agents`, qui s'applique dans la [vue agent](/docs/fr/agent-view), ouverte avec `claude agents`. Nécessite v2.1.257 ou ultérieure.

| Action              | Par défaut | Description                                                                                           |
| :------------------ | :--------- | :---------------------------------------------------------------------------------------------------- |
| `agents:switchView` | Ctrl+S     | Basculer le [regroupement de session](/docs/fr/agent-view#organize-the-list) entre l'état et le répertoire |
| `agents:togglePin`  | Ctrl+T     | [Épingler ou dépingler](/docs/fr/agent-view#organize-the-list) la session sélectionnée                     |

Pendant que la vue agent est ouverte, Claude Code utilise la liaison `Agents` pour toute touche que le contexte `Agents` lie, et il ignore une liaison `Chat` ou `Global` sur la même touche. Par exemple, appuyer sur Ctrl+S dans la vue agent bascule le regroupement de session plutôt que de déclencher la valeur par défaut `chat:stash`.

Le raccourci d'éditeur externe de la saisie de dispatch n'est pas une action `Agents`. La vue agent suit la liaison `chat:externalEditor` du contexte `Chat`, Ctrl+G par défaut.

Les liaisons se déclenchent sur des traits simples dans la vue agent, donc l'accord Ctrl+X Ctrl+E lié à `chat:externalEditor` n'ouvre pas l'éditeur là.

<h3 id="voice-actions">
  Actions vocales
</h3>

Actions disponibles dans le contexte `Chat` lorsque la [dictée vocale](/docs/fr/voice-dictation) est activée :

| Action             | Par défaut | Description                                                    |
| :----------------- | :--------- | :------------------------------------------------------------- |
| `voice:pushToTalk` | Espace     | Dicter une invite. Maintenez ou appuyez selon le mode `/voice` |

<h3 id="scroll-actions">
  Actions de défilement
</h3>

Actions disponibles dans le contexte `Scroll` lorsque le [rendu plein écran](/docs/fr/fullscreen) est activé :

| Action                      | Par défaut         | Description                                                                                                                                                   |
| :-------------------------- | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `scroll:lineUp`             | `wheelup`          | Faire défiler vers le haut d'une ligne. Le défilement à la souris déclenche cette action                                                                      |
| `scroll:lineDown`           | `wheeldown`        | Faire défiler vers le bas d'une ligne. Le défilement à la souris déclenche cette action                                                                       |
| `scroll:pageUp`             | PageUp             | Faire défiler vers le haut de la moitié de la hauteur de la fenêtre d'affichage                                                                               |
| `scroll:pageDown`           | PageDown           | Faire défiler vers le bas de la moitié de la hauteur de la fenêtre d'affichage                                                                                |
| `scroll:top`                | Ctrl+Home          | Sauter au début de la conversation                                                                                                                            |
| `scroll:bottom`             | Ctrl+End           | Sauter au dernier message et réactiver le suivi automatique                                                                                                   |
| `scroll:halfPageUp`         | (non lié)          | Faire défiler vers le haut de la moitié de la hauteur de la fenêtre d'affichage. Même comportement que `scroll:pageUp`, fourni pour les reliures de style vi  |
| `scroll:halfPageDown`       | (non lié)          | Faire défiler vers le bas de la moitié de la hauteur de la fenêtre d'affichage. Même comportement que `scroll:pageDown`, fourni pour les reliures de style vi |
| `scroll:fullPageUp`         | (non lié)          | Faire défiler vers le haut de la hauteur complète de la fenêtre d'affichage                                                                                   |
| `scroll:fullPageDown`       | (non lié)          | Faire défiler vers le bas de la hauteur complète de la fenêtre d'affichage                                                                                    |
| `selection:copy`            | Ctrl+Maj+C / Cmd+C | Copier le texte sélectionné dans le presse-papiers                                                                                                            |
| `selection:clear`           | (non lié)          | Effacer la sélection de texte active. Nécessite v2.1.234 ou ultérieure                                                                                        |
| `selection:extendLeft`      | Maj+Gauche         | Étendre la sélection active d'une colonne vers la gauche                                                                                                      |
| `selection:extendRight`     | Maj+Droite         | Étendre la sélection active d'une colonne vers la droite                                                                                                      |
| `selection:extendUp`        | Maj+Haut           | Étendre la sélection active d'une ligne vers le haut. Fait défiler la fenêtre d'affichage lorsque la sélection atteint le bord supérieur                      |
| `selection:extendDown`      | Maj+Bas            | Étendre la sélection active d'une ligne vers le bas. Fait défiler la fenêtre d'affichage lorsque la sélection atteint le bord inférieur                       |
| `selection:extendLineStart` | Maj+Home           | Étendre la sélection active au début de la ligne                                                                                                              |
| `selection:extendLineEnd`   | Maj+End            | Étendre la sélection active à la fin de la ligne                                                                                                              |

<h2 id="keystroke-syntax">
  Syntaxe des séquences de touches
</h2>

<h3 id="modifiers">
  Modificateurs
</h3>

Utilisez les touches de modification avec le séparateur `+` :

* `ctrl` ou `control` - Touche Contrôle
* `shift` - Touche Maj
* `alt`, `opt`, `option`, ou `meta` - Touche Alt sur Windows et Linux, touche Option sur macOS
* `cmd`, `command`, `super`, ou `win` - Touche Commande sur macOS, touche Windows sur Windows, touche Super sur Linux

Le groupe `cmd` n'est détecté que dans les terminaux qui signalent le modificateur Super, comme ceux prenant en charge le protocole clavier Kitty ou le mode `modifyOtherKeys` de xterm. La plupart des terminaux ne l'envoient pas, donc utilisez `ctrl` ou `meta` pour les liaisons que vous voulez que fonctionnent partout.

Par exemple :

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Maj + Tab
meta+p          Option + P sur macOS, Alt + P ailleurs
ctrl+shift+c    Plusieurs modificateurs
```

<h3 id="uppercase-letters">
  Lettres majuscules
</h3>

Claude Code analyse les noms de touches de manière insensible à la casse, donc `K` est la même liaison que `k` et `ctrl+K` est la même que `ctrl+k`. Pour lier Maj et une lettre, écrivez `shift+k`.

<h3 id="non-us-keyboard-layouts">
  Dispositions de clavier non-US
</h3>

Écrivez les noms de touches des raccourcis Ctrl en tant que caractères latins même lorsque votre disposition de clavier active tape d'autres caractères.

La façon dont Claude Code fait correspondre la touche que vous appuyez à une liaison dépend du type de disposition :

* Sous une disposition non-latine telle que le cyrillique, Claude Code fait correspondre les raccourcis Ctrl par la position de la touche en disposition US lorsque le terminal utilise le protocole clavier Kitty et signale cette position. Dans un tel terminal, avec une disposition russe active, appuyer sur Ctrl et la touche W physique déclenche `ctrl+w`. Dans un terminal qui ne signale pas la position, Claude Code fait correspondre ce que le terminal envoie pour la frappe : un code de contrôle ASCII déclenche le raccourci latin, et une frappe qui arrive en tant que caractère cyrillique ne correspond à aucune liaison
* Sous les dispositions qui réorganisent les lettres latines, telles que AZERTY, Claude Code fait correspondre la lettre que la touche tape, donc appuyer sur Ctrl et la touche étiquetée A déclenche `ctrl+a`

Avant v2.1.247, appuyer sur un raccourci Ctrl sous une disposition non-latine ne déclenchait pas sa liaison dans les terminaux qui utilisent le protocole clavier Kitty, tels que Ghostty, Kitty, WezTerm et iTerm2.

<h3 id="chords">
  Accords
</h3>

Les accords sont des séquences de touches séparées par des espaces :

```text theme={null}
ctrl+k ctrl+s   Appuyez sur Ctrl+K, relâchez, puis Ctrl+S
```

Appuyez sur chaque séquence de touches dans les 3 secondes suivant la précédente. Si vous attendez plus longtemps, Claude Code annule l'accord et affiche un bref message indiquant que c'est le cas.

<h3 id="special-keys">
  Touches spéciales
</h3>

* `escape` ou `esc` - Touche Échappement
* `enter` ou `return` - Touche Entrée
* `tab` - Touche Tab
* `space` - Barre d'espace
* `up`, `down`, `left`, `right` - Touches fléchées
* `pageup`, `pagedown` - Touches Page Précédente et Page Suivante
* `home`, `end` - Touches Accueil et Fin
* `backspace`, `delete` - Touches de suppression
* `wheelup`, `wheeldown` - Événements de défilement de la molette de la souris

<h2 id="unbind-default-shortcuts">
  Délier les raccourcis par défaut
</h2>

Définissez une action sur `null` pour délier un raccourci par défaut :

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

Cela fonctionne également pour les liaisons d'accords. Délier tous les accords qui partagent un préfixe libère ce préfixe pour une utilisation comme liaison à touche unique. Un accord dans n'importe quel contexte actif conserve son préfixe réservé, vous devez donc délier chaque accord dans le contexte qui le définit.

Claude Code lie ces accords par défaut sur le préfixe `ctrl+x` : `ctrl+x ctrl+k`, `ctrl+x ctrl+e`, `ctrl+x enter`, `ctrl+x ctrl+a`, `ctrl+x ctrl+s`, et `ctrl+x tab` dans `Chat`, `ctrl+x ctrl+b` dans `Task`, et `ctrl+x b` dans `DiffPanel`. L'accord `ctrl+x enter` nécessite la v2.1.247 ou une version ultérieure, `ctrl+x b`, `ctrl+x ctrl+a`, et `ctrl+x tab` nécessitent la v2.1.260 ou une version ultérieure, et `ctrl+x ctrl+s` nécessite la v2.1.275 ou une version ultérieure.

Pour récupérer `ctrl+x` lui-même comme liaison à touche unique, déliez tous les éléments suivants :

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

Si vous déliez certains accords mais pas tous sur un préfixe, appuyer sur le préfixe entre toujours en mode d'attente d'accord pour les liaisons restantes.

<h2 id="reserved-shortcuts">
  Raccourcis réservés
</h2>

Ces raccourcis ne peuvent pas être reliés :

| Raccourci | Raison                                                                                                                                                                                                                                                                   |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ctrl+C    | Interruption/annulation codée en dur                                                                                                                                                                                                                                     |
| Ctrl+D    | Sortie codée en dur                                                                                                                                                                                                                                                      |
| Ctrl+M    | Claude Code le reçoit toujours comme Entrée                                                                                                                                                                                                                              |
| Ctrl+\[   | Claude Code le reçoit toujours comme Échap. Dans les terminaux qui utilisent le protocole clavier Kitty, cela nécessite la version 2.1.242 ou ultérieure                                                                                                                 |
| Ctrl+I    | Claude Code le reçoit toujours comme Tab                                                                                                                                                                                                                                 |
| Ctrl+H    | Envoie l'octet ASCII de retour arrière. [La façon dont Claude Code le lit sur Windows](/docs/fr/terminal-config#fix-backspace-deleting-a-whole-word-on-windows) dépend de votre terminal et de la variable d'environnement [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](/docs/fr/env-vars) |
| Caps Lock | Non livré aux applications de terminal                                                                                                                                                                                                                                   |

<h2 id="terminal-conflicts">
  Conflits de terminal
</h2>

Certains raccourcis peuvent entrer en conflit avec les multiplexeurs de terminal :

| Raccourci | Conflit                                       |
| :-------- | :-------------------------------------------- |
| Ctrl+B    | Préfixe tmux (appuyez deux fois pour envoyer) |
| Ctrl+A    | Préfixe GNU screen                            |
| Ctrl+Z    | Suspension de processus Unix (SIGTSTP)        |

<h2 id="text-fields">
  Champs de texte
</h2>

Si vous liez une lettre, un chiffre ou une Espace nue, vous pouvez toujours taper ce caractère dans un champ de texte à l'intérieur d'une boîte de dialogue ou d'un panneau. L'un de ces champs est la réponse « Autre » à une question que Claude pose. Tant que le champ a le focus, une touche imprimable que vous appuyez sans Ctrl, Alt ou Cmd va au champ, et Claude Code ne la fait pas correspondre à vos liaisons.

Ces touches exécutent toujours leurs liaisons tant que le champ a le focus :

* Les touches qui ne tapent pas un caractère, telles que Entrée, Échap, Tab et les touches fléchées
* N'importe quelle touche appuyée avec Ctrl, Alt ou Cmd
* La deuxième frappe d'un [accord](#chords) déjà en cours

À l'invite principale, Claude Code fait correspondre chaque touche aux contextes actifs, tels que `Chat`, et tape la touche uniquement quand aucune liaison ne la prend.

<h2 id="vim-mode-interaction">
  Interaction du mode Vim
</h2>

Lorsque le mode vim est activé via `/config` → Mode Éditeur, les liaisons de touches et le mode vim fonctionnent indépendamment :

* **Mode Vim** gère l'entrée au niveau de la saisie de texte (mouvement du curseur, modes, motions)
* **Liaisons de touches** gèrent les actions au niveau du composant (basculer les tâches, soumettre, etc.)
* La touche Échappement en mode vim bascule INSERT en mode NORMAL ; elle ne déclenche pas `chat:cancel`
* La plupart des raccourcis Ctrl+touche passent par le mode vim au système de liaison de touches
* Les touches Vim ne sont pas remappables via le fichier de liaisons de touches. Pour mapper une séquence en mode INSERT à deux touches comme `jj` à Échappement, utilisez le paramètre [`vimInsertModeRemaps`](/docs/fr/interactive-mode#remap-insert-mode-key-sequences)
* En mode NORMAL vim, `?` affiche le menu d'aide (comportement vim)
* En mode NORMAL vim, `/` ouvre la recherche dans l'historique, identique à Ctrl+R en mode standard

<h2 id="validation">
  Validation
</h2>

Claude Code valide vos liaisons de touches et affiche des avertissements pour :

* Erreurs d'analyse (JSON invalide ou structure)
* Noms de contexte invalides
* Valeurs d'action invalides, telles qu'une action qui n'est pas une chaîne ou `null`
* Noms d'action inconnus, tels qu'une faute de frappe d'une action enregistrée. Claude Code ignore la liaison et conserve toute liaison par défaut pour cette touche en vigueur. Avant v2.1.246, une liaison avec un nom d'action inconnu désactivait silencieusement cette touche
* Conflits de raccourcis réservés
* Liaisons en double dans le même contexte

Claude Code signale les avertissements lors du chargement du fichier et écrit chacun d'eux dans le journal de débogage. Démarrez Claude Code avec [`--debug`](/docs/fr/cli-reference#cli-flags) pour voir les détails.
