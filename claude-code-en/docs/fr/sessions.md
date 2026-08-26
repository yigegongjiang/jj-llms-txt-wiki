> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gérer les sessions

> Nommez, reprenez, créez des branches et basculez entre les conversations Claude Code. Couvre `--continue`, `--resume`, `--from-pr`, le sélecteur `/resume`, la dénomination des sessions, l'export des transcriptions et l'emplacement des transcriptions.

Une session est une conversation enregistrée liée à un répertoire de projet. Claude Code la stocke localement au fur et à mesure que vous travaillez, ce qui vous permet de reprendre là où vous vous êtes arrêté, de créer une branche pour essayer une approche différente ou de basculer entre les tâches.

L'[application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions), [Claude Code sur le web](/docs/fr/claude-code-on-the-web) et l'[extension VS Code](/docs/fr/vs-code#resume-past-conversations) maintiennent chacun leur propre historique de sessions. Cette page couvre l'interface CLI.

<h2 id="resume-a-session">
  Reprendre une session
</h2>

Les sessions sont enregistrées en continu dans les [fichiers de transcription locaux](#export-and-locate-session-data) au fur et à mesure que vous travaillez, ce qui vous permet de revenir à l'une d'elles après avoir quitté ou exécuté `/clear`. Utilisez ces points d'entrée :

| Commande                    | Ce qu'elle fait                                                    |
| :-------------------------- | :----------------------------------------------------------------- |
| `claude --continue`         | Reprend la session la plus récente dans le répertoire courant      |
| `claude --resume`           | Ouvre le [sélecteur de sessions](#use-the-session-picker)          |
| `claude --resume <name>`    | Reprend directement la session nommée                              |
| `claude --from-pr <number>` | Reprend la session liée à cette demande de tirage                  |
| `/resume`                   | Bascule vers une conversation différente depuis une session active |

Les sessions créées avec [`claude -p`](/docs/fr/headless) ou le [SDK Agent](/docs/fr/agent-sdk/overview) n'apparaissent pas dans le sélecteur de sessions, mais vous pouvez toujours en reprendre une en passant son ID de session à `claude --resume <session-id>`. Exécutez ceci depuis le répertoire dans lequel la session a été démarrée : la recherche d'ID de session est limitée au répertoire de projet courant et à ses git worktrees, donc une session créée ailleurs signale `No conversation found with session ID: <session-id>`.

<h3 id="where-the-session-picker-looks">
  Où le sélecteur de sessions cherche
</h3>

Les sessions sont stockées par répertoire de projet. Par défaut, le sélecteur de sessions affiche les sessions interactives du worktree courant, ainsi que les sessions démarrées ailleurs qui ont ajouté le répertoire courant avec `/add-dir`. Utilisez `Ctrl+W` pour élargir à tous les worktrees du référentiel ou `Ctrl+A` pour élargir à chaque projet sur cette machine.

À partir de la v2.1.169, le déplacement d'une session avec [`/cd`](/docs/fr/commands) la réinstalle dans le stockage de projet du nouveau répertoire, ce qui la fait apparaître dans le sélecteur de ce répertoire par la suite. À partir de la v2.1.196, une session déplacée reste absente du sélecteur de l'ancien répertoire même après un plantage ou une fermeture forcée. Sur les versions antérieures, elle pouvait aussi réapparaître dans la liste de l'ancien répertoire après une fermeture qui n'était pas propre lorsque l'ancien chemin contenait des caractères spéciaux tels que des traits de soulignement.

La sélection d'une session d'un autre worktree du même référentiel la reprend sur place. La sélection d'une session d'un projet non lié copie une commande `cd` et de reprise dans votre presse-papiers à la place.

La reprise par nom se résout dans le référentiel courant et ses worktrees. Les deux formes recherchent une correspondance exacte et la reprennent directement même si elle se trouve dans un worktree différent :

| Commande                 | Correspondance exacte | Nom ambigu                                                                                 |
| :----------------------- | :-------------------- | :----------------------------------------------------------------------------------------- |
| `claude --resume <name>` | Reprend directement   | Ouvre le sélecteur de sessions avec le nom pré-rempli comme terme de recherche             |
| `/resume <name>`         | Reprend directement   | Signale une erreur ; exécutez `/resume` sans argument pour ouvrir le sélecteur de sessions |

<h2 id="name-your-sessions">
  Nommer vos sessions
</h2>

Donnez aux sessions des noms descriptifs pour qu'elles soient trouvables dans le sélecteur de sessions et reprises par nom. Cela est particulièrement important lorsque vous travaillez sur plusieurs tâches en parallèle.

| Quand                             | Comment définir le nom                                                                                                                                                               |
| :-------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Au démarrage                      | `claude -n auth-refactor`                                                                                                                                                            |
| Pendant une session               | `/rename auth-refactor`. Le nom apparaît également dans la barre d'invite                                                                                                            |
| À partir du sélecteur de sessions | Mettez en surbrillance une session et appuyez sur `Ctrl+R`                                                                                                                           |
| À l'acceptation du plan           | L'acceptation d'un plan en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) nomme la session à partir du contenu du plan sauf si vous en avez déjà défini un |

Une fois qu'une session est nommée, revenez-y avec `claude --resume <name>` ou `/resume <name>`. Voir [Reprendre une session](#resume-a-session) pour savoir comment la résolution des noms se comporte entre les worktrees.

Les sessions interactives que vous ne nommez jamais reçoivent quand même un nom d'affichage par défaut au démarrage. Nécessite Claude Code v2.1.196 ou version ultérieure. Le nom par défaut combine le nom du répertoire de travail avec un suffixe de deux caractères, par exemple `my-app-3f`, et identifie la session dans les listes de sessions en cours d'exécution, telles que la [vue agent](/docs/fr/agent-view) et la sortie `claude agents --json`.

Le nom par défaut n'est pas un handle de reprise : `claude --resume <name>`, `/resume <name>` et le sélecteur de sessions ne correspondent qu'aux noms que vous avez définis. Nommer la session remplace le nom par défaut.

<h2 id="use-the-session-picker">
  Utiliser le sélecteur de sessions
</h2>

Exécutez `/resume` à l'intérieur d'une session, ou `claude --resume` sans arguments, pour ouvrir le sélecteur de sessions interactif. Utilisez ces raccourcis clavier pour naviguer, rechercher et élargir la liste :

| Raccourci                                          | Action                                                                                                                                                                                  |
| :------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                          | Naviguer entre les sessions                                                                                                                                                             |
| `→` / `←`                                          | Développer ou réduire les sessions groupées                                                                                                                                             |
| `Enter`                                            | Reprendre la session mise en surbrillance                                                                                                                                               |
| `Space`                                            | Prévisualiser le contenu de la session. `Ctrl+V` fonctionne également sur les terminaux qui ne le capturent pas comme collage                                                           |
| `Ctrl+R`                                           | Renommer la session mise en surbrillance                                                                                                                                                |
| `/` ou tout caractère imprimable autre que `Space` | Entrer en mode recherche et filtrer les sessions. Collez une URL de demande de tirage ou de fusion GitHub, GitHub Enterprise, GitLab ou Bitbucket pour trouver la session qui l'a créée |
| `Ctrl+A`                                           | Afficher les sessions de tous les projets sur cette machine. Appuyez à nouveau pour revenir au référentiel courant                                                                      |
| `Ctrl+W`                                           | Afficher les sessions de tous les worktrees du référentiel courant. Appuyez à nouveau pour revenir au worktree courant. Affiché uniquement dans les référentiels multi-worktrees        |
| `Ctrl+B`                                           | Filtrer les sessions de la branche git courante. Appuyez à nouveau pour afficher toutes les branches                                                                                    |
| `Esc`                                              | Quitter le sélecteur de sessions ou le mode recherche                                                                                                                                   |

Chaque ligne affiche le nom de la session s'il est défini, sinon le résumé de la conversation ou la première invite, ainsi que le temps écoulé depuis la dernière activité, le nombre de messages et la branche git. Le chemin du projet apparaît après avoir élargi à tous les projets avec `Ctrl+A`.

Les sessions créées avec `/branch`, `/rewind` ou `--fork-session` sont groupées sous leur session racine. Appuyez sur `→` pour développer un groupe.

<h2 id="branch-a-session">
  Créer une branche d'une session
</h2>

La création d'une branche crée une copie de la conversation jusqu'à présent et vous y bascule, laissant l'original intact. Utilisez-la pour essayer une approche différente sans perdre le chemin sur lequel vous étiez.

À partir d'une session, exécutez `/branch` avec un nom optionnel :

```text theme={null}
/branch try-streaming-approach
```

Si vous omettez le nom, Claude Code nomme la nouvelle branche d'après la première invite de la conversation. À partir de la v2.1.198, cela s'applique également après [compaction](/docs/fr/how-claude-code-works#when-context-fills-up) ; les versions antérieures revenaient au nom littéral `Branched conversation` au lieu de regarder au-delà du résumé de compaction jusqu'à la première invite originale.

À partir de la ligne de commande, combinez `--continue` ou `--resume` avec `--fork-session` :

```bash theme={null}
claude --continue --fork-session
```

La session originale reste inchangée et reste disponible dans le sélecteur de sessions. La confirmation `/branch` imprime deux ID de session : la nouvelle branche dans laquelle vous êtes maintenant et l'original. Pour revenir à l'original, passez son ID à `/resume`, utilisez le sélecteur de sessions ou exécutez `/resume <original-name>`. Les autorisations que vous avez approuvées avec « autoriser pour cette session » ne sont pas reportées à la nouvelle branche. Si vous reprenez la même session dans deux terminaux sans créer de branche, les messages des deux s'entrelacent dans une seule transcription.

Pour la rembobinage basé sur les points de contrôle au sein d'une seule session, voir [Points de contrôle](/docs/fr/checkpointing).

<h2 id="manage-context-within-a-session">
  Gérer le contexte au sein d'une session
</h2>

Ces commandes contrôlent ce qui se trouve dans la fenêtre de contexte sans quitter la session :

* **`/clear`** : recommencer avec un contexte vide. La conversation précédente est enregistrée et reprendre avec `/resume`, ou, dans le même processus Claude Code, depuis [l'entrée de session précédente du menu de rembobinage](/docs/fr/checkpointing#rewind-past-a-cleared-conversation)
* **`/compact [instructions]`** : remplacer l'historique par un résumé, optionnellement axé sur ce que vous spécifiez
* **`/context`** : afficher ce qui consomme actuellement le contexte

Pour savoir comment la compaction interagit avec CLAUDE.md, les compétences et les règles, voir le [guide de la fenêtre de contexte](/docs/fr/context-window). Pour les stratégies sur quand effacer par rapport à compacter, voir [Meilleures pratiques](/docs/fr/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Exporter et localiser les données de session
</h2>

Exécutez `/export` pour ouvrir un menu qui vous permet de copier la conversation courante dans votre presse-papiers ou de l'enregistrer en tant que fichier texte brut, avec les messages et les sorties d'outils rendus sous forme de texte lisible. Passez un nom de fichier pour ignorer le menu et écrire directement dans ce fichier.

<h3 id="access-conversations-from-scripts">
  Accéder aux conversations à partir de scripts
</h3>

`/export` produit une transcription rendue pour qu'une personne la lise. Les interfaces ci-dessous produisent des données structurées pour qu'un script les analyse : un résultat JSON d'une exécution, le chemin vers le fichier de transcription d'une session, ou un flux en direct d'événements. Choisissez en fonction de ce qui déclenche le script :

* **Exécuter Claude une fois et capturer le résultat** : invoquez `claude -p` avec [`--output-format json` ou `stream-json`](/docs/fr/headless#get-structured-output) pour capturer le résultat, l'ID de session, l'utilisation et le coût d'une exécution non interactive sous forme de JSON structuré.
* **Poser une question à une session existante** : passez un ID de session à [`claude -p --resume`](/docs/fr/headless#continue-conversations) pour envoyer une invite de suivi, comme une demande de résumé, et capturer la réponse structurée.
* **Réagir aux événements de session** : lisez le champ `transcript_path` que les [hooks](/docs/fr/hooks#common-input-fields) et les [commandes de ligne d'état](/docs/fr/statusline#available-data) reçoivent en entrée. Un hook `SessionEnd` peut archiver la transcription lorsqu'une session se termine.
* **Intégrer Claude dans une application TypeScript ou Python** : utilisez le [Agent SDK](/docs/fr/agent-sdk/overview) pour recevoir chaque message par programmation.

L'exemple ci-dessous utilise la deuxième interface. Il envoie une invite de suivi à une session existante et lit la réponse avec `jq` :

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Où les transcriptions sont stockées
</h3>

Par défaut, les transcriptions sont stockées en JSONL à `~/.claude/projects/<project>/<session-id>.jsonl`, où `<project>` est votre chemin de répertoire de travail avec les caractères non alphanumériques remplacés par `-`. Chaque ligne est un objet JSON pour un message, une utilisation d'outil ou une entrée de métadonnées. Le format d'entrée est interne à Claude Code et change entre les versions, donc les scripts qui analysent directement ces fichiers peuvent se casser à chaque version. Pour construire sur les données de session, utilisez `/export` ou les [interfaces de script](#access-conversations-from-scripts) à la place.

L'emplacement, la rétention et le comportement d'écriture sont configurables :

| Pour                                                         | Définir                                                | Où                           |
| ------------------------------------------------------------ | ------------------------------------------------------ | ---------------------------- |
| Déplacer le stockage hors de `~/.claude`                     | [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars)                    | Variable d'environnement     |
| Modifier la rétention de 30 jours                            | [`cleanupPeriodDays`](/docs/fr/settings#available-settings) | `settings.json`              |
| Supprimer les écritures de transcription dans tous les modes | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/fr/env-vars)      | Variable d'environnement     |
| Supprimer les écritures pour une exécution non interactive   | [`--no-session-persistence`](/docs/fr/cli-reference)        | Drapeau CLI avec `claude -p` |

<h2 id="see-also">
  Voir aussi
</h2>

Ces pages couvrent les mécaniques de session et de parallélisme connexes :

* [Worktrees](/docs/fr/worktrees) : exécuter des sessions parallèles isolées sur des branches séparées
* [Points de contrôle](/docs/fr/checkpointing) : rembobiner le code et la conversation à un point antérieur
* [Fenêtre de contexte](/docs/fr/context-window) : ce qui remplit le contexte et ce qui survit à la compaction
* [Mode non interactif](/docs/fr/headless) : comportement de la session sous `claude -p`
