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

| Contexte          | Description                                                             |
| :---------------- | :---------------------------------------------------------------------- |
| `Global`          | S'applique partout dans l'application                                   |
| `Chat`            | Zone de saisie de chat principale                                       |
| `Autocomplete`    | Le menu d'autocomplétion est ouvert                                     |
| `Settings`        | Menu des paramètres                                                     |
| `Confirmation`    | Dialogues de permission et de confirmation                              |
| `Tabs`            | Composants de navigation par onglets                                    |
| `Help`            | Le menu d'aide est visible                                              |
| `Transcript`      | Visionneuse de transcription                                            |
| `HistorySearch`   | Mode de recherche d'historique (Ctrl+R)                                 |
| `Task`            | Une tâche de fond est en cours d'exécution                              |
| `ThemePicker`     | Dialogue du sélecteur de thème                                          |
| `Attachments`     | Navigation de la pièce jointe d'image dans les dialogues de sélection   |
| `Footer`          | Navigation de l'indicateur de pied de page (tâches, équipes, diff)      |
| `MessageSelector` | Sélection de message du dialogue de rembobinage et de résumé            |
| `DiffDialog`      | Navigation de la visionneuse de diff                                    |
| `ModelPicker`     | Niveau d'effort du sélecteur de modèle                                  |
| `Select`          | Composants génériques de sélection/liste                                |
| `Plugin`          | Dialogue du plugin (parcourir, découvrir, gérer)                        |
| `Scroll`          | Défilement de la conversation et sélection de texte en mode plein écran |

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
| `app:exit`             | Ctrl+D     | Quitter Claude Code                                                                                                              |
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

| Action                | Par défaut                         | Description                                                                                                                                                                                     |
| :-------------------- | :--------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Échappement                        | Annuler l'entrée actuelle                                                                                                                                                                       |
| `chat:clearInput`     | Ctrl+L                             | Forcer un redessinage complet de l'écran, en préservant l'entrée. Dans le [rendu plein écran](/docs/fr/fullscreen#clear-the-conversation), appuyez deux fois en deux secondes pour exécuter `/clear` |
| `chat:clearScreen`    | Cmd+K                              | Dans le [rendu plein écran](/docs/fr/fullscreen#clear-the-conversation), appuyez deux fois en deux secondes pour exécuter `/clear`                                                                   |
| `chat:killAgents`     | Ctrl+X Ctrl+K                      | Arrêter tous les [sous-agents en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) en cours d'exécution dans cette session                                                |
| `chat:cycleMode`      | Maj+Tab\*                          | Cycler les modes de permission                                                                                                                                                                  |
| `chat:modelPicker`    | Meta+P                             | Ouvrir le sélecteur de modèle                                                                                                                                                                   |
| `chat:fastMode`       | Meta+O                             | Basculer le mode rapide                                                                                                                                                                         |
| `chat:thinkingToggle` | Meta+T                             | Basculer la réflexion étendue                                                                                                                                                                   |
| `chat:submit`         | Entrée                             | Soumettre le message                                                                                                                                                                            |
| `chat:newline`        | Ctrl+J                             | Insérer une nouvelle ligne sans soumettre                                                                                                                                                       |
| `chat:undo`           | Ctrl+\_, Ctrl+Maj+-                | Annuler la dernière action                                                                                                                                                                      |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E              | Ouvrir dans un éditeur externe                                                                                                                                                                  |
| `chat:stash`          | Ctrl+S                             | Mettre en cache l'invite actuelle                                                                                                                                                               |
| `chat:imagePaste`     | Ctrl+V (Alt+V sous Windows et WSL) | Coller une image depuis le presse-papiers. Sur WSL, les deux raccourcis sont liés par défaut                                                                                                    |

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

| Action                      | Par défaut     | Description                                                                                                  |
| :-------------------------- | :------------- | :----------------------------------------------------------------------------------------------------------- |
| `confirm:yes`               | Y, Entrée      | Confirmer l'action                                                                                           |
| `confirm:no`                | N, Échappement | Refuser l'action                                                                                             |
| `confirm:previous`          | Haut           | Option précédente                                                                                            |
| `confirm:next`              | Bas            | Option suivante                                                                                              |
| `confirm:nextField`         | Tab            | Champ suivant                                                                                                |
| `confirm:previousField`     | (non lié)      | Champ précédent                                                                                              |
| `confirm:toggle`            | Espace         | Basculer la sélection                                                                                        |
| `confirm:cycleMode`         | Maj+Tab        | Cycler les modes de permission                                                                               |
| `confirm:toggleExplanation` | Ctrl+E         | Basculer l'explication de la commande générée par le modèle sur les invites de permission Bash et PowerShell |

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

<h3 id="task-actions">
  Actions de tâche
</h3>

Actions disponibles dans le contexte `Task` :

| Action            | Par défaut            | Description                                                                                                                              |
| :---------------- | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Mettre la tâche actuelle en arrière-plan. L'accord Ctrl+X Ctrl+B nécessite la v2.1.169 ou ultérieure et évite le conflit de préfixe tmux |

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

| Action                  | Par défaut  | Description                                                        |
| :---------------------- | :---------- | :----------------------------------------------------------------- |
| `footer:next`           | Droite      | Élément de pied de page suivant                                    |
| `footer:previous`       | Gauche      | Élément de pied de page précédent                                  |
| `footer:up`             | Haut        | Naviguer vers le haut dans le pied de page (désélectionne en haut) |
| `footer:down`           | Bas         | Naviguer vers le bas dans le pied de page                          |
| `footer:openSelected`   | Entrée      | Ouvrir l'élément de pied de page sélectionné                       |
| `footer:clearSelection` | Échappement | Effacer la sélection du pied de page                               |

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

<h3 id="model-picker-actions">
  Actions du sélecteur de modèle
</h3>

Actions disponibles dans le contexte `ModelPicker` :

| Action                        | Par défaut | Description                                                    |
| :---------------------------- | :--------- | :------------------------------------------------------------- |
| `modelPicker:decreaseEffort`  | Gauche     | Diminuer le niveau d'effort                                    |
| `modelPicker:increaseEffort`  | Droite     | Augmenter le niveau d'effort                                   |
| `modelPicker:thisSessionOnly` | s          | Appliquer le modèle en surbrillance à cette session uniquement |

<h3 id="select-actions">
  Actions de sélection
</h3>

Actions disponibles dans le contexte `Select` :

| Action            | Par défaut      | Description           |
| :---------------- | :-------------- | :-------------------- |
| `select:next`     | Bas, J, Ctrl+N  | Option suivante       |
| `select:previous` | Haut, K, Ctrl+P | Option précédente     |
| `select:accept`   | Entrée          | Accepter la sélection |
| `select:cancel`   | Échappement     | Annuler la sélection  |

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
| `scroll:lineUp`             | (non lié)          | Faire défiler vers le haut d'une ligne. Le défilement à la souris déclenche cette action                                                                      |
| `scroll:lineDown`           | (non lié)          | Faire défiler vers le bas d'une ligne. Le défilement à la souris déclenche cette action                                                                       |
| `scroll:pageUp`             | PageUp             | Faire défiler vers le haut de la moitié de la hauteur de la fenêtre d'affichage                                                                               |
| `scroll:pageDown`           | PageDown           | Faire défiler vers le bas de la moitié de la hauteur de la fenêtre d'affichage                                                                                |
| `scroll:top`                | Ctrl+Home          | Sauter au début de la conversation                                                                                                                            |
| `scroll:bottom`             | Ctrl+End           | Sauter au dernier message et réactiver le suivi automatique                                                                                                   |
| `scroll:halfPageUp`         | (non lié)          | Faire défiler vers le haut de la moitié de la hauteur de la fenêtre d'affichage. Même comportement que `scroll:pageUp`, fourni pour les reliures de style vi  |
| `scroll:halfPageDown`       | (non lié)          | Faire défiler vers le bas de la moitié de la hauteur de la fenêtre d'affichage. Même comportement que `scroll:pageDown`, fourni pour les reliures de style vi |
| `scroll:fullPageUp`         | (non lié)          | Faire défiler vers le haut de la hauteur complète de la fenêtre d'affichage                                                                                   |
| `scroll:fullPageDown`       | (non lié)          | Faire défiler vers le bas de la hauteur complète de la fenêtre d'affichage                                                                                    |
| `selection:copy`            | Ctrl+Maj+C / Cmd+C | Copier le texte sélectionné dans le presse-papiers                                                                                                            |
| `selection:clear`           | (non lié)          | Effacer la sélection de texte active                                                                                                                          |
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

Une lettre majuscule autonome implique Maj. Par exemple, `K` est équivalent à `shift+k`. Ceci est utile pour les liaisons de style vim où les touches majuscules et minuscules ont des significations différentes.

Les lettres majuscules avec des modificateurs (par exemple, `ctrl+K`) sont traitées comme stylistiques et n'impliquent **pas** Maj : `ctrl+K` est identique à `ctrl+k`.

<h3 id="chords">
  Accords
</h3>

Les accords sont des séquences de touches séparées par des espaces :

```text theme={null}
ctrl+k ctrl+s   Appuyez sur Ctrl+K, relâchez, puis Ctrl+S
```

<h3 id="special-keys">
  Touches spéciales
</h3>

* `escape` ou `esc` - Touche Échappement
* `enter` ou `return` - Touche Entrée
* `tab` - Touche Tab
* `space` - Barre d'espace
* `up`, `down`, `left`, `right` - Touches fléchées
* `backspace`, `delete` - Touches de suppression

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

La famille `Ctrl+X` par défaut s'étend sur deux contextes : `ctrl+x ctrl+k` et `ctrl+x ctrl+e` dans `Chat`, et `ctrl+x ctrl+b` dans `Task`. Pour récupérer `ctrl+x` lui-même comme liaison à touche unique, déliez tous les éléments suivants :

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
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
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

| Raccourci | Raison                                                       |
| :-------- | :----------------------------------------------------------- |
| Ctrl+C    | Interruption/annulation codée en dur                         |
| Ctrl+D    | Sortie codée en dur                                          |
| Ctrl+M    | Identique à Entrée dans les terminaux (les deux envoient CR) |
| Caps Lock | Non livré aux applications de terminal                       |

<h2 id="terminal-conflicts">
  Conflits de terminal
</h2>

Certains raccourcis peuvent entrer en conflit avec les multiplexeurs de terminal :

| Raccourci | Conflit                                       |
| :-------- | :-------------------------------------------- |
| Ctrl+B    | Préfixe tmux (appuyez deux fois pour envoyer) |
| Ctrl+A    | Préfixe GNU screen                            |
| Ctrl+Z    | Suspension de processus Unix (SIGTSTP)        |

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
* Conflits de raccourcis réservés
* Conflits de multiplexeur de terminal
* Liaisons en double dans le même contexte

Claude Code signale les avertissements lors du chargement du fichier et écrit chacun d'eux dans le journal de débogage. Démarrez Claude Code avec [`--debug`](/docs/fr/cli-reference#cli-flags) pour voir les détails.
