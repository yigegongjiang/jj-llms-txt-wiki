> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurez votre terminal pour Claude Code

> Corrigez Maj+Entrée pour les sauts de ligne, recevez une alerte sonore du terminal quand Claude a terminé, configurez tmux, adaptez le thème de couleur et activez le mode Vim dans l'interface de ligne de commande Claude Code.

Claude Code fonctionne dans n'importe quel terminal sans configuration. Cette page est destinée aux cas où quelque chose ne se comporte pas comme prévu. Trouvez votre symptôme ci-dessous. Si tout vous semble déjà correct, vous n'avez pas besoin de cette page.

* [Maj+Entrée soumet au lieu d'insérer un saut de ligne](#enter-multiline-prompts)
* [Les raccourcis avec la touche Option ne font rien sur macOS](#enable-option-key-shortcuts-on-macos)
* [Aucun son ou alerte quand Claude a terminé](#get-a-terminal-bell-or-notification)
* [Vous exécutez Claude Code dans tmux](#configure-tmux)
* [Retour arrière supprime un mot entier sur Windows](#fix-backspace-deleting-a-whole-word-on-windows)
* [L'affichage scintille ou le défilement revient en arrière](#switch-to-fullscreen-rendering)
* [Vous voulez les touches Vim dans l'invite](#edit-prompts-with-vim-keybindings)

Cette page concerne la configuration de votre terminal pour envoyer les bons signaux à Claude Code. Pour modifier les touches auxquelles Claude Code lui-même répond, consultez plutôt [les raccourcis clavier](/docs/fr/keybindings).

<h2 id="enter-multiline-prompts">
  Entrer des invites multilignes
</h2>

Appuyer sur Entrée soumet votre message. Pour ajouter un saut de ligne sans soumettre, appuyez sur Ctrl+J, ou tapez `\` puis appuyez sur Entrée. Les deux fonctionnent dans chaque terminal sans configuration.

Dans la plupart des terminaux, vous pouvez également appuyer sur Maj+Entrée, mais le support varie selon l'émulateur de terminal :

| Terminal                                                                                                 | Maj+Entrée pour nouvelle ligne                                             |
| :------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal                                  | Fonctionne sans configuration                                              |
| Autres terminaux qui supportent le protocole clavier kitty, tels que foot et Alacritty 0.16 ou ultérieur | Fonctionne sans configuration. Nécessite Claude Code v2.1.269 ou ultérieur |
| VS Code, Cursor, Devin Desktop, Alacritty avant 0.16, Zed                                                | Exécutez `/terminal-setup` une fois                                        |
| gnome-terminal, JetBrains IDEs tels que PyCharm et Android Studio                                        | Non disponible ; utilisez Ctrl+J ou `\` puis Entrée                        |

Pour VS Code, Cursor, Devin Desktop, Alacritty avant 0.16 et Zed, `/terminal-setup` écrit une liaison de touche Maj+Entrée dans le fichier de configuration du terminal. À la première exécution, vous voyez une confirmation telle que `Installed VSCode terminal Shift+Enter key binding`. Les liaisons existantes sont conservées ; si vous voyez un message tel que `VSCode terminal Shift+Enter key binding already configured`, aucune modification n'a été apportée. Exécutez `/terminal-setup` directement dans le terminal hôte plutôt qu'à l'intérieur de tmux ou screen, car il doit écrire dans la configuration du terminal hôte.

Dans VS Code, Cursor et Devin Desktop, `/terminal-setup` met également à jour deux paramètres d'éditeur : il définit `terminal.integrated.gpuAcceleration` sur `"off"` pour éviter le texte brouillé dans le terminal intégré, et il définit `terminal.integrated.mouseWheelScrollSensitivity` pour un défilement plus fluide en [mode plein écran](/docs/fr/fullscreen). Pour annuler la modification de l'accélération GPU, redéfinissez-la sur `"auto"` et rechargez la fenêtre de l'éditeur.

Dans Zed, `/terminal-setup` met à jour votre `keymap.json` sur place :

* Si le keymap a déjà des liaisons et qu'aucune d'elles n'est un Terminal `shift-enter`, Claude Code crée d'abord une sauvegarde dans le même répertoire, telle que `keymap.json.1a2b3c4d.bak`, puis fusionne la liaison Maj+Entrée dans votre keymap, en conservant vos autres liaisons de touches et commentaires
* Si Claude Code ne peut pas lire ou analyser le keymap, ne peut pas le sauvegarder, ou ne peut pas vérifier le résultat fusionné, il [laisse le fichier inchangé et imprime le bloc de liaison de touche à ajouter vous-même](/docs/fr/errors#terminal-setup-left-your-zed-keymap-unchanged)

Si vous exécutez à l'intérieur de tmux, Maj+Entrée nécessite également la [configuration tmux ci-dessous](#configure-tmux) même lorsque le terminal externe la supporte.

Pour lier la nouvelle ligne à une touche différente, ou pour inverser le comportement afin que Entrée insère une nouvelle ligne et Maj+Entrée soumet, mappez les actions `chat:newline` et `chat:submit` dans votre [fichier de liaisons de touches](/docs/fr/keybindings).

<h2 id="enable-option-key-shortcuts-on-macos">
  Activer les raccourcis de la touche Option sur macOS
</h2>

Certains raccourcis Claude Code utilisent la touche Option, comme Option+Entrée pour une nouvelle ligne ou Option+P pour changer de modèle. Sur macOS, la plupart des terminaux n'envoient pas Option comme modificateur par défaut, donc ces raccourcis ne font rien jusqu'à ce que vous l'activiez. Le paramètre du terminal pour cela est généralement étiqueté « Use Option as Meta Key » ; Meta est le nom historique Unix pour la touche maintenant étiquetée Option ou Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Ouvrez Paramètres → Profils → Clavier et cochez « Use Option as Meta Key ».

    Si vous avez accepté l'invite de configuration du terminal au premier lancement de Claude Code, c'est déjà fait. Cette invite exécute `/terminal-setup` pour vous, ce qui active Option comme Meta et désactive la cloche audible dans votre profil Apple Terminal.

    En [mode lecteur d'écran](/docs/fr/accessibility), `/terminal-setup` laisse le paramètre de cloche inchangé afin que la cloche du terminal reste audible. Avant la v2.1.211, `/terminal-setup` désactivait la cloche même en mode lecteur d'écran. Si une exécution antérieure a désactivé la cloche, réactivez-la sous Paramètres → Profils → Avancé → « Audible bell ».
  </Tab>

  <Tab title="iTerm2">
    Ouvrez Paramètres → Profils → Touches → Général et définissez la touche Option gauche et la touche Option droite sur « Esc+ ».

    L'exécution de `/terminal-setup` dans iTerm2 active « Applications in terminal may access clipboard » sous Paramètres → Général → Sélection afin que la commande `/copy` puisse écrire dans votre presse-papiers système. La commande détecte iTerm2 même lorsqu'elle est exécutée depuis tmux. Redémarrez iTerm2 pour que la modification prenne effet.
  </Tab>

  <Tab title="VS Code">
    Ajoutez `"terminal.integrated.macOptionIsMeta": true` à vos paramètres VS Code.
  </Tab>
</Tabs>

Pour Ghostty, Kitty et autres terminaux, recherchez un paramètre Option-as-Alt ou Option-as-Meta dans le fichier de configuration du terminal.

<h2 id="get-a-terminal-bell-or-notification">
  Obtenir une cloche de terminal ou une notification
</h2>

Lorsque Claude termine une tâche ou s'arrête pour une demande de permission, et que vous semblez être absent du terminal, il déclenche un événement de notification. Consultez [quand chaque type de notification se déclenche](/docs/fr/hooks#notification) pour connaître le moment exact. Afficher cela comme une cloche de terminal ou une notification de bureau vous permet de passer à d'autres tâches pendant qu'une longue tâche s'exécute.

Par défaut, Claude Code envoie une notification de bureau uniquement dans Ghostty, Kitty et iTerm2. Dans d'autres terminaux, définissez [`preferredNotifChannel`](/docs/fr/settings-reference#preferrednotifchannel) sur `"terminal_bell"` pour sonner la cloche du terminal à la place, ou configurez un [hook Notification](#play-a-sound-with-a-notification-hook) pour un son ou une commande personnalisée. L'entrée de paramètres suivante active la cloche du terminal :

```json ~/.claude/settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

La notification de bureau atteint votre machine locale via SSH, de sorte qu'une session distante peut toujours vous alerter. Ghostty et Kitty la transmettent à votre centre de notifications du système d'exploitation sans configuration supplémentaire. iTerm2 vous demande d'activer la transmission :

<Steps>
  <Step title="Ouvrir les paramètres de notification d'iTerm2">
    Allez à Paramètres → Profils → Terminal.
  </Step>

  <Step title="Activer les alertes">
    Cochez « Notification Center Alerts », puis cliquez sur « Filter Alerts » et activez « Send escape sequence-generated alerts ».
  </Step>
</Steps>

Si les notifications n'apparaissent toujours pas, confirmez que votre application de terminal dispose de la permission de notification dans les paramètres de votre système d'exploitation, et si vous exécutez tmux, [activez la transmission](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Jouer un son avec un hook Notification
</h3>

Dans n'importe quel terminal, vous pouvez configurer un [hook Notification](/docs/fr/hooks-guide#get-notified-when-claude-needs-input) pour jouer un son ou exécuter une commande personnalisée lorsque Claude a besoin de votre attention. Les hooks s'exécutent aux côtés de la notification intégrée plutôt que de la remplacer, de sorte que les terminaux qui ne reçoivent pas de notification de bureau, comme Warp ou le terminal intégré de VS Code, peuvent utiliser un hook ou définir `preferredNotifChannel` sur `"terminal_bell"` à la place.

L'exemple ci-dessous joue un son système sur macOS. Le guide lié contient des commandes de notification de bureau pour macOS, Linux et Windows.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  Configurer tmux
</h2>

Quand Claude Code s'exécute dans tmux, par défaut Shift+Entrée soumet au lieu d'insérer une nouvelle ligne, et les notifications de bureau et la [barre de progression](/docs/fr/settings-reference#terminalprogressbarenabled) n'atteignent jamais le terminal externe. Ajoutez ces lignes à `~/.tmux.conf`, puis exécutez `tmux source-file ~/.tmux.conf` pour les appliquer au serveur en cours d'exécution :

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

La ligne `allow-passthrough` permet aux notifications et aux mises à jour de progression d'atteindre le terminal externe au lieu d'être absorbées par tmux. Les lignes `extended-keys` permettent à tmux de distinguer Shift+Entrée d'une simple Entrée afin que le raccourci de nouvelle ligne fonctionne.

<h2 id="fix-backspace-deleting-a-whole-word-on-windows">
  Corriger la suppression d'un mot entier avec Retour arrière sur Windows
</h2>

Sur Windows, Claude Code lit un Retour arrière qui arrive sous la forme `^H` comme Ctrl+Retour arrière, ce qui [supprime le mot précédent](/docs/fr/interactive-mode#text-editing), sauf quand `TERM_PROGRAM` est `mintty` ou `TERM` est `cygwin`. Sur macOS et Linux, Claude Code le lit comme un simple Retour arrière.

Si chaque appui sur Retour arrière supprime un mot entier, votre terminal envoie `^H` pour un simple Retour arrière. Définissez [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE=0`](/docs/fr/env-vars). Retour arrière et Ctrl+H effacent alors chacun un caractère. Si Ctrl+Retour arrière efface seulement un caractère sur macOS ou Linux parce que votre terminal envoie `^H` pour cela, définissez plutôt la variable à `1`.

<h2 id="match-the-color-theme">
  Adapter le thème de couleur
</h2>

Utilisez la commande `/theme`, ou le sélecteur de thème dans `/config`, pour choisir un thème Claude Code qui correspond à votre terminal. La sélection de l'option auto détecte le fond clair ou sombre de votre terminal, de sorte que le thème suit les changements d'apparence du système d'exploitation chaque fois que votre terminal le fait. Claude Code ne contrôle pas le schéma de couleurs du terminal lui-même, qui est défini par l'application terminal.

Pour personnaliser ce qui apparaît en bas de l'interface, configurez une [barre d'état personnalisée](/docs/fr/statusline) qui affiche le modèle actuel, le répertoire de travail, la branche git, ou d'autres contextes.

<h3 id="create-a-custom-theme">
  Créer un thème personnalisé
</h3>

En plus des présets intégrés, `/theme` répertorie tous les thèmes personnalisés que vous avez définis et tous les thèmes contribués par les [plugins](/docs/fr/plugins/components#themes-and-output-styles) installés. Sélectionnez **Nouveau thème personnalisé…** à la fin de la liste pour en créer un de manière interactive : vous nommez le thème, puis choisissez les jetons de couleur individuels à remplacer. Appuyez sur `Ctrl+E` tandis qu'un thème personnalisé est en surbrillance pour le modifier.

Chaque thème personnalisé est un fichier JSON dans `~/.claude/themes/`. Le nom de fichier sans l'extension `.json` est le slug du thème, et la sélection du thème stocke `custom:<slug>` comme votre préférence de thème. Le fichier a trois champs optionnels :

| Champ       | Type   | Description                                                                                                                                                |
| :---------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | Étiquette d'affichage affichée dans `/theme`. Par défaut, le slug du nom de fichier                                                                        |
| `base`      | string | Preset intégré à partir duquel le thème commence : `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, ou `light-ansi`. Par défaut `dark` |
| `overrides` | object | Mappage des noms de jetons de couleur aux valeurs de couleur. Les jetons non listés ici passent au preset de base                                          |

Les valeurs de couleur acceptent `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, ou `ansi:<name>` où `<name>` est l'un des 16 noms de couleur ANSI standard tels que `red` ou `cyanBright`. Les jetons inconnus et les valeurs de couleur invalides sont ignorés, donc une faute de frappe ne peut pas casser le rendu.

L'exemple suivant définit un thème qui conserve le preset sombre mais recolore l'accent du prompt, le texte d'erreur et le texte de succès :

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code surveille `~/.claude/themes/` et recharge lorsqu'un fichier est ajouté ou modifié, de sorte que les modifications apportées dans votre éditeur s'appliquent à une session en cours sans redémarrage. Si le dossier `~/.claude/themes/` lui-même n'existait pas au démarrage de Claude Code, redémarrez une fois après avoir créé votre premier fichier de thème. Après cela, les modifications s'appliquent sans redémarrage.

La référence ci-dessous couvre les jetons que vous pouvez définir dans `overrides`. L'éditeur interactif dans `/theme` affiche les mêmes jetons avec un aperçu en direct, plus quelques accents à usage unique tels que les couleurs de l'écran d'intégration qui sont omis ici.

<Accordion title="Référence des jetons de couleur">
  L'exemple suivant combine les jetons de plusieurs groupes ci-dessous : l'accent de marque, la bordure du mode plan, les arrière-plans de diff, et l'arrière-plan du message.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    Couleurs de texte et d'accent
  </h4>

  Contrôlez l'accent de marque principal et les nuances de texte de premier plan utilisées dans toute l'interface.

  | Jeton         | Contrôle                                                                         |
  | :------------ | :------------------------------------------------------------------------------- |
  | `claude`      | Accent de marque principal, utilisé pour le spinner et l'étiquette d'assistant   |
  | `text`        | Texte de premier plan par défaut                                                 |
  | `inverseText` | Texte dessiné sur un fond coloré, comme les badges de statut                     |
  | `inactive`    | Texte secondaire tel que les indices, les horodatages et les éléments désactivés |
  | `subtle`      | Bordures faibles et texte secondaire désaccentué                                 |
  | `suggestion`  | Suggestions d'autocomplétion et surbrillance de sélection dans les sélecteurs    |
  | `permission`  | Bordures de dialogue, y compris les invites de permission et les sélecteurs      |
  | `remember`    | Indicateurs de mémoire et `CLAUDE.md`                                            |

  <h4 id="status-colors">
    Couleurs de statut
  </h4>

  Signalez les états de succès, d'échec et d'avertissement dans les messages et les indicateurs.

  | Jeton     | Contrôle                                                          |
  | :-------- | :---------------------------------------------------------------- |
  | `success` | Messages de succès et vérifications réussies                      |
  | `error`   | Messages d'erreur et échecs                                       |
  | `warning` | Avertissements, messages de prudence et l'indicateur du mode auto |
  | `merged`  | Statut de demande de tirage fusionnée                             |

  <h4 id="input-box-and-mode-indicators">
    Boîte d'entrée et indicateurs de mode
  </h4>

  Définissez la couleur de bordure de la boîte d'entrée et l'accent affiché tandis qu'un mode de permission ou un indicateur est actif.

  | Jeton          | Contrôle                                                                                                                                                                                                                   |
  | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | `promptBorder` | Bordure de la boîte d'entrée                                                                                                                                                                                               |
  | `planMode`     | Accent du mode plan, messages du mode plan et dialogues du mode plan                                                                                                                                                       |
  | `autoAccept`   | Accent du mode accepter les modifications                                                                                                                                                                                  |
  | `bashBorder`   | Bordure de la boîte d'entrée lors de la saisie d'une commande shell `!`                                                                                                                                                    |
  | `ide`          | Indicateur de connexion IDE                                                                                                                                                                                                |
  | `fastMode`     | Indicateur du mode rapide                                                                                                                                                                                                  |
  | `effortUltra`  | L'étiquette `ultracode` sur la bordure de la boîte d'entrée tandis que [ultracode](/docs/fr/model-config#adjust-effort-level) est activé. Votre remplacement de cette couleur prend effet sur Claude Code v2.1.239 ou ultérieur |

  <h4 id="diff-rendering">
    Rendu des diffs
  </h4>

  Coloriez le code ajouté et supprimé dans les modifications et révisions de fichiers.

  | Jeton               | Contrôle                                                                                                 |
  | :------------------ | :------------------------------------------------------------------------------------------------------- |
  | `diffAdded`         | Arrière-plan des lignes ajoutées                                                                         |
  | `diffRemoved`       | Arrière-plan des lignes supprimées                                                                       |
  | `diffAddedDimmed`   | Arrière-plan des lignes ajoutées dans le diff estompé affiché après que vous rejetiez une modification   |
  | `diffRemovedDimmed` | Arrière-plan des lignes supprimées dans le diff estompé affiché après que vous rejetiez une modification |
  | `diffAddedWord`     | Surbrillance au niveau des mots dans une ligne ajoutée                                                   |
  | `diffRemovedWord`   | Surbrillance au niveau des mots dans une ligne supprimée                                                 |

  <h4 id="fullscreen-mode">
    Mode plein écran
  </h4>

  Claude Code peint `userMessageBackground`, `bashMessageBackgroundColor`, et `memoryBackgroundColor` dans les rendus par défaut et plein écran. Il utilise `userMessageBackgroundHover` et `selectionBg` uniquement dans le [mode de rendu plein écran](/docs/fr/fullscreen).

  | Jeton                        | Contrôle                                                                      |
  | :--------------------------- | :---------------------------------------------------------------------------- |
  | `userMessageBackground`      | Arrière-plan derrière vos messages dans la transcription                      |
  | `userMessageBackgroundHover` | Arrière-plan derrière un message lors du survol ou de l'expansion             |
  | `bashMessageBackgroundColor` | Arrière-plan derrière les entrées de commande shell `!` dans la transcription |
  | `memoryBackgroundColor`      | Arrière-plan derrière les entrées de mémoire `#` dans la transcription        |
  | `selectionBg`                | Arrière-plan du texte sélectionné à la souris                                 |

  <h4 id="usage-meter-and-speaker-labels">
    Compteur d'utilisation et étiquettes de haut-parleur
  </h4>

  Ajustez la barre affichée dans la vue `/usage` et les étiquettes qui distinguent vos messages de ceux de Claude.

  | Jeton              | Contrôle                                                     |
  | :----------------- | :----------------------------------------------------------- |
  | `rate_limit_fill`  | Portion remplie du compteur d'utilisation                    |
  | `rate_limit_empty` | Portion non remplie du compteur d'utilisation                |
  | `briefLabelYou`    | Couleur de l'étiquette `You` sur vos messages                |
  | `briefLabelClaude` | Couleur de l'étiquette `Claude` sur les messages d'assistant |

  <h4 id="shimmer-variants-and-subagent-colors">
    Variantes de scintillement et couleurs des sous-agents
  </h4>

  Plusieurs jetons ont une variante de scintillement appariée qui fournit la couleur plus claire utilisée dans le dégradé animé du spinner. Remplacez le scintillement aux côtés de son jeton de base si l'animation semble mal assortie.

  * `claude` et `claudeShimmer`
  * `warning` et `warningShimmer`
  * `permission` et `permissionShimmer`
  * `promptBorder` et `promptBorderShimmer`
  * `inactive` et `inactiveShimmer`
  * `fastMode` et `fastModeShimmer`

  Chaque [sous-agent](/docs/fr/sub-agents) et tâche parallèle est affiché dans l'une des huit couleurs nommées pour que vous puissiez les distinguer dans la transcription. Les noms de jetons suivent le modèle `<color>_FOR_SUBAGENTS_ONLY`, où `<color>` est `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, ou `cyan`. Remplacez-les pour modifier l'apparence de chaque couleur nommée. Par exemple, un sous-agent avec `color: blue` dans sa définition est dessiné en utilisant la valeur `blue_FOR_SUBAGENTS_ONLY`.

  Claude Code rend le mot-clé [`ultrathink`](/docs/fr/model-config#use-ultrathink-for-one-off-deep-reasoning) dans l'entrée du prompt avec un dégradé arc-en-ciel à sept couleurs. Les noms de jetons suivent le modèle `rainbow_<color>` et `rainbow_<color>_shimmer`, où `<color>` est `red`, `orange`, `yellow`, `green`, `blue`, `indigo`, ou `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Basculer vers le rendu en plein écran
</h2>

En [mode lecteur d'écran](/docs/fr/accessibility), cette section ne s'applique pas. Claude Code s'affiche toujours sous forme de texte défilant simple sauf dans les [sessions en arrière-plan](/docs/fr/agent-view) attachées, et si vous exécutez `/tui fullscreen` dans toute autre session, Claude Code affiche une explication au lieu de basculer.

Si l'affichage scintille ou la position de défilement saute pendant que Claude travaille, basculez vers le [mode de rendu en plein écran](/docs/fr/fullscreen). Dans ce mode, vous faites défiler avec la souris ou PageUp à l'intérieur de Claude Code plutôt qu'avec le défilement natif de votre terminal ; consultez la [page plein écran](/docs/fr/fullscreen#search-and-review-the-conversation) pour savoir comment rechercher et copier.

Si le scintillement est le seul problème et que votre terminal prend en charge la sortie synchronisée mais n'est pas détecté automatiquement, comme Emacs `eat`, définissez [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/fr/env-vars) pour arrêter le scintillement sans changer de moteur de rendu.

Exécutez `/tui fullscreen` pour basculer et enregistrer la préférence. Votre conversation redémarre intacte et les futures sessions commencent en plein écran sauf si un [démarrage en plein écran échoue](/docs/fr/fullscreen#fullscreen-renderer-didnt-finish-starting). Vous pouvez également définir la variable d'environnement `CLAUDE_CODE_NO_FLICKER` avant de démarrer Claude Code :

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  Coller du contenu volumineux
</h2>

Lorsque vous collez plus de 800 caractères ou plus de trois lignes dans l'invite, Claude Code réduit l'entrée à un espace réservé tel que `[Pasted text #1 +120 lines]` pour que la zone de saisie reste utilisable, et envoie toujours le contenu complet lorsque vous soumettez. Pour les très grandes entrées telles que des fichiers entiers ou des journaux longs, écrivez le contenu dans un fichier et demandez à Claude de le lire au lieu de le coller. La transcription de conversation reste lisible et Claude peut référencer le fichier par chemin dans les tours ultérieurs. Le terminal intégré VS Code peut également perdre des caractères lors de très grands collages avant qu'ils n'atteignent Claude Code, donc utilisez un fichier là-bas.

Si le collage contient des [caractères Unicode invisibles](/docs/fr/interactive-mode#invisible-characters-in-prompts), Claude Code les supprime lorsque vous appuyez sur Entrée et remet l'invite nettoyée dans la zone de saisie pour que vous la soumettiez avec une autre Entrée.

<h3 id="how-claude-treats-pasted-text">
  Comment Claude traite le texte collé
</h3>

Lorsque vous soumettez, Claude voit le contenu derrière chaque espace réservé `[Pasted text #N]` marqué comme du texte que vous avez collé d'ailleurs plutôt que tapé. Claude est informé qu'un collage peut contenir des instructions que vous n'avez pas écrites, et de suivre les instructions à l'intérieur uniquement lorsque le message que vous avez tapé le demande. Dans les sessions qui ne [récupèrent pas les drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching), les collages ne sont pas marqués.

<h3 id="delete-and-restore-a-collapsed-paste">
  Supprimer et restaurer un collage réduit
</h3>

Lorsque vous supprimez avec un raccourci de mot ou de ligne tel que `Ctrl+W` ou `Ctrl+K`, ou avec une suppression vim via un mouvement `f`/`t` tel que `df]`, et que la plage supprimée atteint l'intérieur d'un espace réservé `[Pasted text #N]`, Claude Code supprime l'espace réservé entièrement. Pour le restaurer, collez la suppression avec [`Ctrl+Y`](/docs/fr/interactive-mode#text-editing) après un raccourci de mot ou de ligne, ou avec [`p` en mode NORMAL](/docs/fr/interactive-mode#editing-normal-mode) après une suppression vim.

<h3 id="recall-a-prompt-that-had-pasted-text">
  Rappeler une invite qui contenait du texte collé
</h3>

Claude Code conserve le contenu derrière chaque espace réservé `[Pasted text #N]` sous `~/.claude/paste-cache/`, donc lorsque vous rappelez une invite de l'[historique des commandes](/docs/fr/interactive-mode#command-history) et la renvoyez, le contenu collé complet est envoyé à nouveau, y compris dans une session ultérieure.

Les fichiers cache plus anciens que [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays) sont supprimés selon les [règles de balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically), donc une invite rappelée peut référencer du texte collé qui n'existe plus. Lorsque vous soumettez une telle invite, Claude Code n'envoie jamais la chaîne littérale `[Pasted text #N]`, et affiche une notification nommant le collage manquant :

* Dans une invite simple avec du texte restant, Claude Code supprime l'espace réservé et envoie le texte restant.
* Dans une commande [mode shell](/docs/fr/interactive-mode#shell-mode-with-prefix) ou une commande `/`, où la suppression changerait ce qui s'exécute, et dans toute invite où la suppression laisse vide, Claude Code annule la soumission et conserve le texte original dans l'entrée, avec l'espace réservé toujours dedans. Supprimez l'espace réservé ou modifiez la commande, puis renvoyez.

<h2 id="edit-prompts-with-vim-keybindings">
  Modifier les prompts avec les raccourcis clavier Vim
</h2>

Claude Code inclut un mode d'édition de style Vim pour l'entrée de prompt. Activez-le via `/config` → Editor mode, ou en définissant [`editorMode`](/docs/fr/settings-reference#editormode) sur `"vim"` dans `~/.claude/settings.json`. Réglez le mode Editor sur `normal` pour le désactiver.

Le mode Vim prend en charge un sous-ensemble des motions et opérateurs des modes NORMAL et VISUAL, tels que la navigation `hjkl`, la sélection `v`/`V`, et `d`/`c`/`y` avec des objets texte. Consultez la [référence du mode éditeur Vim](/docs/fr/interactive-mode#vim-editor-mode) pour le tableau complet des touches.

Les motions Vim ne sont pas remappables via le fichier keybindings. Pour mapper une séquence INSERT-mode à deux touches comme `jj` sur Échap, définissez [`vimInsertModeRemaps`](/docs/fr/interactive-mode#remap-insert-mode-key-sequences) dans vos paramètres utilisateur.

Appuyer sur Entrée soumet toujours votre prompt en mode INSERT, contrairement au Vim standard. Utilisez `o` ou `O` en mode NORMAL, ou Ctrl+J, pour insérer une nouvelle ligne à la place.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Mode interactif](/docs/fr/interactive-mode) : référence complète des raccourcis clavier et table de clés Vim
* [Keybindings](/docs/fr/keybindings) : remappez n'importe quel raccourci Claude Code, y compris Entrée et Maj+Entrée
* [Rendu en plein écran](/docs/fr/fullscreen) : détails sur le défilement, la recherche et la copie en mode plein écran
* [Guide des hooks](/docs/fr/hooks-guide) : plus d'exemples de hooks Notification pour Linux et Windows
* [Dépannage](/docs/fr/troubleshooting) : corrections pour les problèmes en dehors de la configuration du terminal
