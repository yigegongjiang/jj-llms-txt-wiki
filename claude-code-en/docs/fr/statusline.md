> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personnalisez votre barre de statut

> Configurez une barre de statut personnalisée pour surveiller l'utilisation de la fenêtre de contexte, les coûts et l'état git dans Claude Code

La barre de statut est une barre personnalisable en bas de Claude Code qui exécute n'importe quel script shell que vous configurez. Elle reçoit les données de session JSON sur stdin et affiche tout ce que votre script imprime, vous donnant une vue persistante et en un coup d'œil de l'utilisation du contexte, des coûts, de l'état git, ou de tout ce que vous voulez suivre.

Les barres de statut sont utiles quand vous :

* Voulez surveiller l'utilisation de la fenêtre de contexte pendant que vous travaillez
* Avez besoin de suivre les coûts de session
* Travaillez sur plusieurs sessions et avez besoin de les distinguer
* Voulez que la branche git et l'état soient toujours visibles

La barre de statut s'affiche dans sa propre ligne au-dessus des badges de pied de page intégrés et ne les remplace pas. Pour ajouter des badges de lien cliquables au pied de page quand un ID apparaît dans la conversation, sans écrire de script, configurez [`footerLinksRegexes`](/docs/fr/settings#footer-link-badges) à la place.

Voici un exemple d'une [barre de statut multi-lignes](#display-multiple-lines) qui affiche les informations git sur la première ligne et une barre de contexte codée par couleur sur la deuxième.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Une barre de statut multi-lignes affichant le nom du modèle, le répertoire, la branche git sur la première ligne, et une barre de progression d'utilisation du contexte avec le coût et la durée sur la deuxième ligne" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Cette page vous guide à travers [la configuration d'une barre de statut basique](#set-up-a-status-line), explique [comment les données circulent](#how-status-lines-work) de Claude Code à votre script, liste [tous les champs que vous pouvez afficher](#available-data), et fournit [des exemples prêts à l'emploi](#examples) pour les modèles courants comme l'état git, le suivi des coûts et les barres de progression.

<h2 id="set-up-a-status-line">
  Configurer une barre de statut
</h2>

Utilisez la [commande `/statusline`](#use-the-%2Fstatusline-command) pour que Claude Code génère un script pour vous, ou [créez manuellement un script](#manually-configure-a-status-line) et ajoutez-le à vos paramètres.

<h3 id="use-the-/statusline-command">
  Utiliser la commande /statusline
</h3>

La commande `/statusline` accepte des instructions en langage naturel décrivant ce que vous voulez afficher. Claude Code génère un fichier script dans `~/.claude/` et met à jour vos paramètres automatiquement :

```text theme={null}
/statusline show model name and context percentage with a progress bar
```

<h3 id="manually-configure-a-status-line">
  Configurer manuellement une barre de statut
</h3>

Ajoutez un champ `statusLine` à vos paramètres utilisateur (`~/.claude/settings.json`, où `~` est votre répertoire personnel) ou [paramètres de projet](/docs/fr/settings#settings-files). Définissez `type` sur `"command"` et pointez `command` vers un chemin de script ou une commande shell en ligne. Pour une procédure complète de création d'un script, voir [Construire une barre de statut étape par étape](#build-a-status-line-step-by-step).

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline.sh",
    "padding": 2
  }
}
```

Le champ `command` s'exécute dans un shell, vous pouvez donc aussi utiliser des commandes en ligne au lieu d'un fichier script. Cet exemple utilise `jq` pour analyser l'entrée JSON et afficher le nom du modèle et le pourcentage de contexte :

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'"
  }
}
```

Le champ optionnel `padding` ajoute un espacement horizontal supplémentaire (en caractères) au contenu de la barre de statut. Par défaut `0`. Cet espacement s'ajoute à l'espacement intégré de l'interface, il contrôle donc l'indentation relative plutôt que la distance absolue du bord du terminal.

Le champ optionnel `refreshInterval` réexécute votre commande toutes les N secondes en plus des [mises à jour basées sur les événements](#how-status-lines-work). Le minimum est `1`. Définissez ceci quand votre barre de statut affiche des données basées sur le temps comme une horloge, ou quand les sous-agents en arrière-plan modifient l'état git pendant que la session principale est inactive. Laissez-le non défini pour s'exécuter uniquement sur les événements.

Le champ optionnel `hideVimModeIndicator` supprime le texte intégré `-- INSERT --` sous l'invite. Définissez ceci sur `true` quand votre script affiche [`vim.mode`](#available-data) lui-même, afin que le mode ne soit pas affiché deux fois.

<h3 id="disable-the-status-line">
  Désactiver la barre de statut
</h3>

Exécutez `/statusline` et demandez-lui de supprimer ou d'effacer votre barre de statut (par exemple, `/statusline delete`, `/statusline clear`, `/statusline remove it`). Vous pouvez aussi supprimer manuellement le champ `statusLine` de votre settings.json.

<h2 id="build-a-status-line-step-by-step">
  Construire une barre de statut étape par étape
</h2>

Cette procédure montre ce qui se passe sous le capot en créant manuellement une barre de statut qui affiche le modèle actuel, le répertoire de travail et le pourcentage d'utilisation de la fenêtre de contexte.

<Note>L'exécution de [`/statusline`](#use-the-%2Fstatusline-command) avec une description de ce que vous voulez configure tout cela automatiquement pour vous.</Note>

Ces exemples utilisent des scripts Bash, qui fonctionnent sur macOS et Linux. Sur Windows, voir [Configuration Windows](#windows-configuration) pour des exemples PowerShell et Git Bash.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-quickstart.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=696445e59ca0059213250651ad23db6b" alt="Une barre de statut affichant le nom du modèle, le répertoire et le pourcentage de contexte" width="726" height="164" data-path="images/statusline-quickstart.png" />
</Frame>

<Steps>
  <Step title="Créer un script qui lit JSON et imprime la sortie">
    Claude Code envoie les données JSON à votre script via stdin. Ce script utilise [`jq`](https://jqlang.github.io/jq/), un analyseur JSON en ligne de commande que vous devrez peut-être installer, pour extraire le nom du modèle, le répertoire et le pourcentage de contexte, puis imprime une ligne formatée.

    Enregistrez ceci dans `~/.claude/statusline.sh` (où `~` est votre répertoire personnel, tel que `/Users/username` sur macOS ou `/home/username` sur Linux) :

    ```bash theme={null}
    #!/bin/bash
    # Read JSON data that Claude Code sends to stdin
    input=$(cat)

    # Extract fields using jq
    MODEL=$(echo "$input" | jq -r '.model.display_name')
    DIR=$(echo "$input" | jq -r '.workspace.current_dir')
    # The "// 0" provides a fallback if the field is null
    PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

    # Output the status line - ${DIR##*/} extracts just the folder name
    echo "[$MODEL] 📁 ${DIR##*/} | ${PCT}% context"
    ```
  </Step>

  <Step title="Le rendre exécutable">
    Marquez le script comme exécutable pour que votre shell puisse l'exécuter :

    ```bash theme={null}
    chmod +x ~/.claude/statusline.sh
    ```
  </Step>

  <Step title="Ajouter aux paramètres">
    Dites à Claude Code d'exécuter votre script comme barre de statut. Ajoutez cette configuration à `~/.claude/settings.json`, qui définit `type` sur `"command"` (ce qui signifie « exécuter cette commande shell ») et pointe `command` vers votre script :

    ```json theme={null}
    {
      "statusLine": {
        "type": "command",
        "command": "~/.claude/statusline.sh"
      }
    }
    ```

    Votre barre de statut apparaît en bas de l'interface. Les paramètres se rechargent automatiquement, mais les modifications n'apparaîtront pas avant votre prochaine interaction avec Claude Code.
  </Step>
</Steps>

<h2 id="how-status-lines-work">
  Comment fonctionnent les barres de statut
</h2>

Claude Code exécute votre script et envoie les [données de session JSON](#available-data) via stdin. Votre script lit le JSON, extrait ce dont il a besoin et imprime du texte sur stdout. Claude Code affiche tout ce que votre script imprime.

**Quand elle se met à jour**

Votre script s'exécute après chaque nouveau message d'assistant, après que `/compact` se termine, quand le mode de permission change, ou quand le mode vim bascule. Les mises à jour sont débogées à 300 ms, ce qui signifie que les changements rapides se regroupent et votre script s'exécute une fois que les choses se stabilisent. Si une nouvelle mise à jour se déclenche pendant que votre script s'exécute encore, l'exécution en cours est annulée. Si vous modifiez votre script, les modifications n'apparaîtront pas avant que votre prochaine interaction avec Claude Code ne déclenche une mise à jour.

Ces déclencheurs peuvent devenir silencieux quand la session principale est inactive, par exemple pendant qu'un coordinateur attend les sous-agents en arrière-plan. Pour garder les segments basés sur le temps ou provenant de sources externes à jour pendant les périodes inactives, définissez [`refreshInterval`](#manually-configure-a-status-line) pour aussi réexécuter la commande sur un minuteur fixe.

**Ce que votre script peut afficher**

* **Plusieurs lignes** : chaque instruction `echo` ou `print` s'affiche comme une ligne séparée. Voir l'[exemple multi-lignes](#display-multiple-lines).
* **Couleurs** : utilisez les [codes d'échappement ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) comme `\033[32m` pour le vert (le terminal doit les supporter). Voir l'[exemple d'état git](#git-status-with-colors).
* **Liens** : utilisez les [séquences d'échappement OSC 8](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC) pour rendre le texte cliquable (Cmd+clic sur macOS, Ctrl+clic sur Windows/Linux). Nécessite un terminal qui supporte les hyperliens comme iTerm2, Kitty ou WezTerm. Voir l'[exemple de liens cliquables](#clickable-links).

**Dimensionner la sortie au terminal**

Claude Code capture la sortie de votre script au lieu de la connecter directement au terminal, donc `tput cols` et la détection de largeur au niveau du langage ne peuvent pas lire la taille du terminal depuis l'intérieur du script. Lisez plutôt les variables d'environnement `COLUMNS` et `LINES`. Claude Code définit ces variables aux dimensions actuelles du terminal avant d'exécuter votre script. Nécessite Claude Code v2.1.153 ou ultérieur.

<Note>La barre de statut s'exécute localement et ne consomme pas de jetons API. Elle se cache temporairement pendant certaines interactions UI, y compris les suggestions d'autocomplétion, le menu d'aide et les invites de permission.</Note>

<h2 id="available-data">
  Données disponibles
</h2>

Claude Code envoie les champs JSON suivants à votre script via stdin :

| Champ                                                                            | Description                                                                                                                                                                                                                                                                                                  |
| -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `model.id`, `model.display_name`                                                 | Identifiant du modèle actuel et nom d'affichage                                                                                                                                                                                                                                                              |
| `cwd`, `workspace.current_dir`                                                   | Répertoire de travail actuel. Les deux champs contiennent la même valeur ; `workspace.current_dir` est préféré pour la cohérence avec `workspace.project_dir`.                                                                                                                                               |
| `workspace.project_dir`                                                          | Répertoire où Claude Code a été lancé, qui peut différer de `cwd` si le répertoire de travail change pendant une session                                                                                                                                                                                     |
| `workspace.added_dirs`                                                           | Répertoires supplémentaires ajoutés via `/add-dir` ou `--add-dir`. Tableau vide si aucun n'a été ajouté                                                                                                                                                                                                      |
| `workspace.git_worktree`                                                         | Nom du git worktree quand le répertoire actuel se trouve à l'intérieur d'un worktree lié créé avec `git worktree add`. Absent dans le worktree principal. Rempli pour n'importe quel git worktree, contrairement à `worktree.*` qui s'applique uniquement aux sessions `--worktree`                          |
| `workspace.repo.host`, `workspace.repo.owner`, `workspace.repo.name`             | Identité du référentiel analysée à partir de la télécommande `origin`, par exemple `"github.com"`, `"anthropics"`, `"claude-code"`. Absent en dehors d'un référentiel git ou quand aucune télécommande `origin` n'est configurée                                                                             |
| `cost.total_cost_usd`                                                            | Coût total estimé de la session en USD, calculé côté client. Peut différer de votre facture réelle                                                                                                                                                                                                           |
| `cost.total_duration_ms`                                                         | Temps écoulé total depuis le début de la session, en millisecondes                                                                                                                                                                                                                                           |
| `cost.total_api_duration_ms`                                                     | Temps total passé à attendre les réponses API en millisecondes                                                                                                                                                                                                                                               |
| `cost.total_lines_added`, `cost.total_lines_removed`                             | Lignes de code modifiées                                                                                                                                                                                                                                                                                     |
| `context_window.total_input_tokens`, `context_window.total_output_tokens`        | Comptages de jetons actuellement dans la fenêtre de contexte, à partir de la réponse API la plus récente. L'entrée inclut les lectures et écritures du cache. Avant v2.1.132, il s'agissait de totaux cumulatifs de session                                                                                  |
| `context_window.context_window_size`                                             | Taille maximale de la fenêtre de contexte en jetons. 200 000 par défaut, ou 1 000 000 pour les modèles avec contexte étendu.                                                                                                                                                                                 |
| `context_window.used_percentage`                                                 | Pourcentage pré-calculé de fenêtre de contexte utilisée                                                                                                                                                                                                                                                      |
| `context_window.remaining_percentage`                                            | Pourcentage pré-calculé de fenêtre de contexte restante                                                                                                                                                                                                                                                      |
| `context_window.current_usage`                                                   | Comptages de jetons du dernier appel API, décrits dans [champs de fenêtre de contexte](#context-window-fields)                                                                                                                                                                                               |
| `exceeds_200k_tokens`                                                            | Si le comptage total de jetons (jetons d'entrée, de cache et de sortie combinés) de la réponse API la plus récente dépasse 200 k. C'est un seuil fixe indépendamment de la taille réelle de la fenêtre de contexte.                                                                                          |
| `effort.level`                                                                   | Effort de raisonnement actuel (`low`, `medium`, `high`, `xhigh`, ou `max`). Reflète la valeur de session en direct, y compris les changements `/effort` en cours de session. Ultracode n'est pas un niveau distinct et rapporte `xhigh`. Absent quand le modèle actuel ne supporte pas le paramètre d'effort |
| `thinking.enabled`                                                               | Si la réflexion étendue est activée pour la session                                                                                                                                                                                                                                                          |
| `rate_limits.five_hour.used_percentage`, `rate_limits.seven_day.used_percentage` | Pourcentage de la limite de débit de 5 heures ou 7 jours consommée, de 0 à 100                                                                                                                                                                                                                               |
| `rate_limits.five_hour.resets_at`, `rate_limits.seven_day.resets_at`             | Secondes d'époque Unix quand la fenêtre de limite de débit de 5 heures ou 7 jours se réinitialise                                                                                                                                                                                                            |
| `session_id`                                                                     | Identifiant de session unique                                                                                                                                                                                                                                                                                |
| `session_name`                                                                   | Nom de session personnalisé défini avec l'indicateur `--name` ou `/rename`. Absent si aucun nom personnalisé n'a été défini                                                                                                                                                                                  |
| `prompt_id`                                                                      | UUID identifiant l'invite utilisateur actuellement traitée. Correspond à l'attribut [`prompt.id` sur les événements OpenTelemetry](/docs/fr/monitoring-usage#event-correlation-attributes). Absent jusqu'à la première entrée utilisateur. Nécessite Claude Code v2.1.196 ou ultérieur                            |
| `transcript_path`                                                                | Chemin vers le fichier de transcription de conversation                                                                                                                                                                                                                                                      |
| `version`                                                                        | Version de Claude Code                                                                                                                                                                                                                                                                                       |
| `output_style.name`                                                              | Nom du style de sortie actuel                                                                                                                                                                                                                                                                                |
| `vim.mode`                                                                       | Mode vim actuel (`NORMAL`, `INSERT`, `VISUAL`, ou `VISUAL LINE`) quand le [mode vim](/docs/fr/interactive-mode#vim-editor-mode) est activé                                                                                                                                                                        |
| `agent.name`                                                                     | Nom de l'agent lors de l'exécution avec l'indicateur `--agent` ou les paramètres d'agent configurés                                                                                                                                                                                                          |
| `pr.number`, `pr.url`                                                            | Demande de tirage ouverte pour la branche actuelle. Reflète le badge PR dans la barre de statut inférieure. Absent jusqu'à ce qu'une PR soit trouvée, quand ce n'est pas dans un référentiel git, ou une fois que la PR fusionne ou se ferme                                                                 |
| `pr.review_state`                                                                | État d'examen de la PR ouverte : `approved`, `pending`, `changes_requested`, ou `draft`. Peut être indépendamment absent même quand `pr` est présent                                                                                                                                                         |
| `worktree.name`                                                                  | Nom du worktree actif. Présent uniquement pendant les sessions `--worktree`                                                                                                                                                                                                                                  |
| `worktree.path`                                                                  | Chemin absolu vers le répertoire du worktree                                                                                                                                                                                                                                                                 |
| `worktree.branch`                                                                | Nom de la branche git pour le worktree (par exemple, `"worktree-my-feature"`). Absent pour les worktrees basés sur des hooks                                                                                                                                                                                 |
| `worktree.original_cwd`                                                          | Le répertoire dans lequel Claude se trouvait avant d'entrer dans le worktree                                                                                                                                                                                                                                 |
| `worktree.original_branch`                                                       | Branche git extraite avant d'entrer dans le worktree. Absent pour les worktrees basés sur des hooks                                                                                                                                                                                                          |

<Accordion title="Schéma JSON complet">
  Votre commande de barre de statut reçoit cette structure JSON via stdin :

  ```json theme={null}
  {
    "cwd": "/current/working/directory",
    "session_id": "abc123...",
    "session_name": "my-session",
    "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
    "transcript_path": "/path/to/transcript.jsonl",
    "model": {
      "id": "claude-opus-4-8",
      "display_name": "Opus"
    },
    "workspace": {
      "current_dir": "/current/working/directory",
      "project_dir": "/original/project/directory",
      "added_dirs": [],
      "git_worktree": "feature-xyz",
      "repo": {
        "host": "github.com",
        "owner": "anthropics",
        "name": "claude-code"
      }
    },
    "version": "2.1.90",
    "output_style": {
      "name": "default"
    },
    "cost": {
      "total_cost_usd": 0.01234,
      "total_duration_ms": 45000,
      "total_api_duration_ms": 2300,
      "total_lines_added": 156,
      "total_lines_removed": 23
    },
    "context_window": {
      "total_input_tokens": 15500,
      "total_output_tokens": 1200,
      "context_window_size": 200000,
      "used_percentage": 8,
      "remaining_percentage": 92,
      "current_usage": {
        "input_tokens": 8500,
        "output_tokens": 1200,
        "cache_creation_input_tokens": 5000,
        "cache_read_input_tokens": 2000
      }
    },
    "exceeds_200k_tokens": false,
    "effort": {
      "level": "high"
    },
    "thinking": {
      "enabled": true
    },
    "rate_limits": {
      "five_hour": {
        "used_percentage": 23.5,
        "resets_at": 1738425600
      },
      "seven_day": {
        "used_percentage": 41.2,
        "resets_at": 1738857600
      }
    },
    "vim": {
      "mode": "NORMAL"
    },
    "agent": {
      "name": "security-reviewer"
    },
    "pr": {
      "number": 1234,
      "url": "https://github.com/anthropics/claude-code/pull/1234",
      "review_state": "pending"
    },
    "worktree": {
      "name": "my-feature",
      "path": "/path/to/.claude/worktrees/my-feature",
      "branch": "worktree-my-feature",
      "original_cwd": "/path/to/project",
      "original_branch": "main"
    }
  }
  ```

  **Champs qui peuvent être absents** (non présents dans JSON) :

  * `session_name` : apparaît uniquement quand un nom personnalisé a été défini avec `--name` ou `/rename`
  * `prompt_id` : apparaît uniquement après la première entrée utilisateur
  * `workspace.git_worktree` : apparaît uniquement quand le répertoire actuel se trouve à l'intérieur d'un git worktree lié
  * `workspace.repo` : apparaît uniquement à l'intérieur d'un référentiel git avec une télécommande `origin` configurée
  * `effort` : apparaît uniquement quand le modèle actuel supporte le paramètre d'effort de raisonnement
  * `vim` : apparaît uniquement quand le mode vim est activé
  * `agent` : apparaît uniquement lors de l'exécution avec l'indicateur `--agent` ou les paramètres d'agent configurés
  * `pr` : apparaît uniquement tant qu'une PR ouverte est trouvée pour la branche actuelle, et est supprimée une fois que la PR fusionne ou se ferme. `pr.review_state` peut être indépendamment absent
  * `worktree` : apparaît uniquement pendant les sessions `--worktree`. Quand présent, `branch` et `original_branch` peuvent aussi être absents pour les worktrees basés sur des hooks
  * `rate_limits` : apparaît uniquement pour les abonnés Claude.ai (Pro/Max) après la première réponse API dans la session. Chaque fenêtre (`five_hour`, `seven_day`) peut être indépendamment absente. Utilisez `jq -r '.rate_limits.five_hour.used_percentage // empty'` pour gérer l'absence avec élégance.

  **Champs qui peuvent être `null`** :

  * `context_window.current_usage` : `null` avant le premier appel API dans une session, et à nouveau après `/compact` jusqu'à ce que le prochain appel API le remplisse à nouveau
  * `context_window.used_percentage`, `context_window.remaining_percentage` : peuvent être `null` au début de la session

  Gérez les champs manquants avec un accès conditionnel et les valeurs null avec des valeurs par défaut de secours dans vos scripts.
</Accordion>

<h3 id="context-window-fields">
  Champs de fenêtre de contexte
</h3>

L'objet `context_window` décrit la fenêtre de contexte en direct à partir de la réponse API la plus récente. À partir de v2.1.132, `total_input_tokens` et `total_output_tokens` reflètent l'utilisation du contexte actuel, et non les totaux cumulatifs de session.

* **Totaux combinés** (`total_input_tokens`, `total_output_tokens`) : jetons actuellement dans la fenêtre de contexte. `total_input_tokens` est la somme de `input_tokens`, `cache_creation_input_tokens`, et `cache_read_input_tokens` ; `total_output_tokens` est les jetons de sortie de la réponse la plus récente. Les deux sont `0` avant la première réponse API.
* **Utilisation par composant** (`current_usage`) : les mêmes comptages de jetons ventilés par catégorie. Utilisez ceci quand vous avez besoin des accès au cache séparés de l'entrée fraîche.

L'objet `current_usage` contient :

* `input_tokens` : jetons d'entrée dans le contexte actuel
* `output_tokens` : jetons de sortie générés
* `cache_creation_input_tokens` : jetons écrits dans le cache
* `cache_read_input_tokens` : jetons lus du cache

Pour comprendre ce que signifient les champs de cache et comment ils sont facturés, consultez [vérifier les performances du cache](/docs/fr/prompt-caching#check-cache-performance).

Le champ `used_percentage` est calculé à partir des jetons d'entrée uniquement : `input_tokens + cache_creation_input_tokens + cache_read_input_tokens`. Il n'inclut pas `output_tokens`.

Si vous calculez le pourcentage de contexte manuellement à partir de `current_usage`, utilisez la même formule d'entrée uniquement pour correspondre à `used_percentage`.

L'objet `current_usage` est `null` avant le premier appel API dans une session, et à nouveau immédiatement après `/compact` jusqu'à ce que le prochain appel API le remplisse à nouveau.

<h2 id="examples">
  Exemples
</h2>

Ces exemples montrent les modèles courants de barre de statut. Pour utiliser n'importe quel exemple :

1. Enregistrez le script dans un fichier comme `~/.claude/statusline.sh` (ou `.py`/`.js`)
2. Le rendre exécutable : `chmod +x ~/.claude/statusline.sh`
3. Ajouter le chemin à vos [paramètres](#manually-configure-a-status-line)

Les exemples Bash utilisent [`jq`](https://jqlang.github.io/jq/) pour analyser JSON. Python et Node.js ont l'analyse JSON intégrée.

<h3 id="context-window-usage">
  Utilisation de la fenêtre de contexte
</h3>

Affiche le modèle actuel et l'utilisation de la fenêtre de contexte avec une barre de progression visuelle. Chaque script lit JSON depuis stdin, extrait le champ `used_percentage` et construit une barre de 10 caractères où les blocs remplis (▓) représentent l'utilisation :

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-context-window-usage.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=15b58ab3602f036939145dde3165c6f7" alt="Une barre de statut affichant le nom du modèle et une barre de progression avec pourcentage" width="448" height="152" data-path="images/statusline-context-window-usage.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  # Read all of stdin into a variable
  input=$(cat)

  # Extract fields with jq, "// 0" provides fallback for null
  MODEL=$(echo "$input" | jq -r '.model.display_name')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

  # Build progress bar: printf -v creates a run of spaces, then
  # ${var// /▓} replaces each space with a block character
  BAR_WIDTH=10
  FILLED=$((PCT * BAR_WIDTH / 100))
  EMPTY=$((BAR_WIDTH - FILLED))
  BAR=""
  [ "$FILLED" -gt 0 ] && printf -v FILL "%${FILLED}s" && BAR="${FILL// /▓}"
  [ "$EMPTY" -gt 0 ] && printf -v PAD "%${EMPTY}s" && BAR="${BAR}${PAD// /░}"

  echo "[$MODEL] $BAR $PCT%"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  # json.load reads and parses stdin in one step
  data = json.load(sys.stdin)
  model = data['model']['display_name']
  # "or 0" handles null values
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)

  # String multiplication builds the bar
  filled = pct * 10 // 100
  bar = '▓' * filled + '░' * (10 - filled)

  print(f"[{model}] {bar} {pct}%")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  // Node.js reads stdin asynchronously with events
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      // Optional chaining (?.) safely handles null fields
      const pct = Math.floor(data.context_window?.used_percentage || 0);

      // String.repeat() builds the bar
      const filled = Math.floor(pct * 10 / 100);
      const bar = '▓'.repeat(filled) + '░'.repeat(10 - filled);

      console.log(`[${model}] ${bar} ${pct}%`);
  });
  ```
</CodeGroup>

<h3 id="git-status-with-colors">
  État git avec couleurs
</h3>

Affiche la branche git avec des indicateurs codés par couleur pour les fichiers en attente et modifiés. Ce script utilise les [codes d'échappement ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) pour les couleurs de terminal : `\033[32m` est vert, `\033[33m` est jaune, et `\033[0m` réinitialise à la valeur par défaut.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-git-context.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e656f34f90d1d9a1d0e220988914345f" alt="Une barre de statut affichant le modèle, le répertoire, la branche git et des indicateurs colorés pour les fichiers en attente et modifiés" width="742" height="178" data-path="images/statusline-git-context.png" />
</Frame>

Chaque script vérifie si le répertoire actuel est un dépôt git, compte les fichiers en attente et modifiés, et affiche des indicateurs codés par couleur :

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')

  GREEN='\033[32m'
  YELLOW='\033[33m'
  RESET='\033[0m'

  if git rev-parse --git-dir > /dev/null 2>&1; then
      BRANCH=$(git branch --show-current 2>/dev/null)
      STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
      MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')

      GIT_STATUS=""
      [ "$STAGED" -gt 0 ] && GIT_STATUS="${GREEN}+${STAGED}${RESET}"
      [ "$MODIFIED" -gt 0 ] && GIT_STATUS="${GIT_STATUS}${YELLOW}~${MODIFIED}${RESET}"

      echo -e "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH $GIT_STATUS"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])

  GREEN, YELLOW, RESET = '\033[32m', '\033[33m', '\033[0m'

  try:
      subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
      staged_output = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
      modified_output = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
      staged = len(staged_output.split('\n')) if staged_output else 0
      modified = len(modified_output.split('\n')) if modified_output else 0

      git_status = f"{GREEN}+{staged}{RESET}" if staged else ""
      git_status += f"{YELLOW}~{modified}{RESET}" if modified else ""

      print(f"[{model}] 📁 {directory} | 🌿 {branch} {git_status}")
  except:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);

      const GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RESET = '\x1b[0m';

      try {
          execSync('git rev-parse --git-dir', { stdio: 'ignore' });
          const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
          const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
          const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;

          let gitStatus = staged ? `${GREEN}+${staged}${RESET}` : '';
          gitStatus += modified ? `${YELLOW}~${modified}${RESET}` : '';

          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} ${gitStatus}`);
      } catch {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="cost-and-duration-tracking">
  Suivi des coûts et de la durée
</h3>

Suivez les coûts API de votre session et le temps écoulé. Le champ `cost.total_cost_usd` accumule le coût estimé de tous les appels API dans la session actuelle. Le champ `cost.total_duration_ms` mesure le temps écoulé total depuis le début de la session, tandis que `cost.total_api_duration_ms` suit uniquement le temps passé à attendre les réponses API.

Chaque script formate le coût en devise et convertit les millisecondes en minutes et secondes :

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-cost-tracking.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e3444a51fe6f3440c134bd5f1f08ad29" alt="Une barre de statut affichant le nom du modèle, le coût de session et la durée" width="588" height="180" data-path="images/statusline-cost-tracking.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  COST_FMT=$(printf '$%.2f' "$COST")
  DURATION_SEC=$((DURATION_MS / 1000))
  MINS=$((DURATION_SEC / 60))
  SECS=$((DURATION_SEC % 60))

  echo "[$MODEL] 💰 $COST_FMT | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  duration_sec = duration_ms // 1000
  mins, secs = duration_sec // 60, duration_sec % 60

  print(f"[{model}] 💰 ${cost:.2f} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const cost = data.cost?.total_cost_usd || 0;
      const durationMs = data.cost?.total_duration_ms || 0;

      const durationSec = Math.floor(durationMs / 1000);
      const mins = Math.floor(durationSec / 60);
      const secs = durationSec % 60;

      console.log(`[${model}] 💰 $${cost.toFixed(2)} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="display-multiple-lines">
  Afficher plusieurs lignes
</h3>

Votre script peut afficher plusieurs lignes pour créer un affichage plus riche. Chaque instruction `echo` produit une ligne séparée dans la zone de statut.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Une barre de statut multi-lignes affichant le nom du modèle, le répertoire, la branche git sur la première ligne, et une barre de progression d'utilisation du contexte avec le coût et la durée sur la deuxième ligne" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Cet exemple combine plusieurs techniques : couleurs basées sur des seuils (vert sous 70 %, jaune 70-89 %, rouge 90 %+), une barre de progression et des informations de branche git. Chaque instruction `print` ou `echo` crée une ligne séparée :

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; RESET='\033[0m'

  # Pick bar color based on context usage
  if [ "$PCT" -ge 90 ]; then BAR_COLOR="$RED"
  elif [ "$PCT" -ge 70 ]; then BAR_COLOR="$YELLOW"
  else BAR_COLOR="$GREEN"; fi

  FILLED=$((PCT / 10)); EMPTY=$((10 - FILLED))
  printf -v FILL "%${FILLED}s"; printf -v PAD "%${EMPTY}s"
  BAR="${FILL// /█}${PAD// /░}"

  MINS=$((DURATION_MS / 60000)); SECS=$(((DURATION_MS % 60000) / 1000))

  BRANCH=""
  git rev-parse --git-dir > /dev/null 2>&1 && BRANCH=" | 🌿 $(git branch --show-current 2>/dev/null)"

  echo -e "${CYAN}[$MODEL]${RESET} 📁 ${DIR##*/}$BRANCH"
  COST_FMT=$(printf '$%.2f' "$COST")
  echo -e "${BAR_COLOR}${BAR}${RESET} ${PCT}% | ${YELLOW}${COST_FMT}${RESET} | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  CYAN, GREEN, YELLOW, RED, RESET = '\033[36m', '\033[32m', '\033[33m', '\033[31m', '\033[0m'

  bar_color = RED if pct >= 90 else YELLOW if pct >= 70 else GREEN
  filled = pct // 10
  bar = '█' * filled + '░' * (10 - filled)

  mins, secs = duration_ms // 60000, (duration_ms % 60000) // 1000

  try:
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True, stderr=subprocess.DEVNULL).strip()
      branch = f" | 🌿 {branch}" if branch else ""
  except:
      branch = ""

  print(f"{CYAN}[{model}]{RESET} 📁 {directory}{branch}")
  print(f"{bar_color}{bar}{RESET} {pct}% | {YELLOW}${cost:.2f}{RESET} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const cost = data.cost?.total_cost_usd || 0;
      const pct = Math.floor(data.context_window?.used_percentage || 0);
      const durationMs = data.cost?.total_duration_ms || 0;

      const CYAN = '\x1b[36m', GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RED = '\x1b[31m', RESET = '\x1b[0m';

      const barColor = pct >= 90 ? RED : pct >= 70 ? YELLOW : GREEN;
      const filled = Math.floor(pct / 10);
      const bar = '█'.repeat(filled) + '░'.repeat(10 - filled);

      const mins = Math.floor(durationMs / 60000);
      const secs = Math.floor((durationMs % 60000) / 1000);

      let branch = '';
      try {
          branch = execSync('git branch --show-current', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          branch = branch ? ` | 🌿 ${branch}` : '';
      } catch {}

      console.log(`${CYAN}[${model}]${RESET} 📁 ${dir}${branch}`);
      console.log(`${barColor}${bar}${RESET} ${pct}% | ${YELLOW}$${cost.toFixed(2)}${RESET} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="clickable-links">
  Liens cliquables
</h3>

Cet exemple crée un lien cliquable vers votre dépôt GitHub. Il lit l'URL du dépôt distant, convertit le format SSH en HTTPS avec `sed` et enveloppe le nom du dépôt dans les codes d'échappement OSC 8. Maintenez Cmd (macOS) ou Ctrl (Windows/Linux) et cliquez pour ouvrir le lien dans votre navigateur.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-links.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=4bcc6e7deb7cf52f41ab85a219b52661" alt="Une barre de statut affichant un lien cliquable vers un dépôt GitHub" width="726" height="198" data-path="images/statusline-links.png" />
</Frame>

Chaque script obtient l'URL du dépôt distant, convertit le format SSH en HTTPS et enveloppe le nom du dépôt dans les codes d'échappement OSC 8. La version Bash utilise `printf '%b'` qui interprète les échappements de barre oblique inverse de manière plus fiable que `echo -e` sur différents shells :

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')

  # Convert git SSH URL to HTTPS
  REMOTE=$(git remote get-url origin 2>/dev/null | sed 's/git@github.com:/https:\/\/github.com\//' | sed 's/\.git$//')

  if [ -n "$REMOTE" ]; then
      REPO_NAME=$(basename "$REMOTE")
      # OSC 8 format: \e]8;;URL\a then TEXT then \e]8;;\a
      # printf %b interprets escape sequences reliably across shells
      printf '%b' "[$MODEL] 🔗 \e]8;;${REMOTE}\a${REPO_NAME}\e]8;;\a\n"
  else
      echo "[$MODEL]"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, re, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  # Get git remote URL
  try:
      remote = subprocess.check_output(
          ['git', 'remote', 'get-url', 'origin'],
          stderr=subprocess.DEVNULL, text=True
      ).strip()
      # Convert SSH to HTTPS format
      remote = re.sub(r'^git@github\.com:', 'https://github.com/', remote)
      remote = re.sub(r'\.git$', '', remote)
      repo_name = os.path.basename(remote)
      # OSC 8 escape sequences
      link = f"\033]8;;{remote}\a{repo_name}\033]8;;\a"
      print(f"[{model}] 🔗 {link}")
  except:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      try {
          let remote = execSync('git remote get-url origin', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          // Convert SSH to HTTPS format
          remote = remote.replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '');
          const repoName = path.basename(remote);
          // OSC 8 escape sequences
          const link = `\x1b]8;;${remote}\x07${repoName}\x1b]8;;\x07`;
          console.log(`[${model}] 🔗 ${link}`);
      } catch {
          console.log(`[${model}]`);
      }
  });
  ```
</CodeGroup>

<h3 id="rate-limit-usage">
  Utilisation des limites de débit
</h3>

Affiche l'utilisation des limites de débit d'abonnement Claude.ai dans la barre de statut. L'objet `rate_limits` contient `five_hour` (fenêtre glissante de 5 heures) et `seven_day` (fenêtres hebdomadaires). Chaque fenêtre fournit `used_percentage` (0-100) et `resets_at` (secondes d'époque Unix quand la fenêtre se réinitialise).

Ce champ n'est présent que pour les abonnés Claude.ai (Pro/Max) après la première réponse API. Chaque script gère le champ absent avec élégance :

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  # "// empty" produces no output when rate_limits is absent
  FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
  WEEK=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')

  LIMITS=""
  [ -n "$FIVE_H" ] && LIMITS="5h: $(printf '%.0f' "$FIVE_H")%"
  [ -n "$WEEK" ] && LIMITS="${LIMITS:+$LIMITS }7d: $(printf '%.0f' "$WEEK")%"

  [ -n "$LIMITS" ] && echo "[$MODEL] | $LIMITS" || echo "[$MODEL]"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  parts = []
  rate = data.get('rate_limits', {})
  five_h = rate.get('five_hour', {}).get('used_percentage')
  week = rate.get('seven_day', {}).get('used_percentage')

  if five_h is not None:
      parts.append(f"5h: {five_h:.0f}%")
  if week is not None:
      parts.append(f"7d: {week:.0f}%")

  if parts:
      print(f"[{model}] | {' '.join(parts)}")
  else:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      const parts = [];
      const fiveH = data.rate_limits?.five_hour?.used_percentage;
      const week = data.rate_limits?.seven_day?.used_percentage;

      if (fiveH != null) parts.push(`5h: ${Math.round(fiveH)}%`);
      if (week != null) parts.push(`7d: ${Math.round(week)}%`);

      console.log(parts.length ? `[${model}] | ${parts.join(' ')}` : `[${model}]`);
  });
  ```
</CodeGroup>

<h3 id="cache-expensive-operations">
  Mettre en cache les opérations coûteuses
</h3>

Votre script de barre de statut s'exécute fréquemment pendant les sessions actives. Les commandes comme `git status` ou `git diff` peuvent être lentes, surtout dans les grands dépôts. Cet exemple met en cache les informations git dans un fichier temporaire et ne les actualise que toutes les 5 secondes.

Le nom du fichier de cache doit être stable dans les invocations de barre de statut au sein d'une session, mais unique dans les sessions afin que les sessions concurrentes dans différents dépôts ne lisent pas l'état git en cache les unes des autres. Les identifiants basés sur les processus comme `$$`, `os.getpid()` ou `process.pid` changent à chaque invocation et annulent le cache. Utilisez plutôt le `session_id` de l'entrée JSON : il est stable pour la durée de vie d'une session et unique par session.

Chaque script vérifie si le fichier de cache est manquant ou plus ancien que 5 secondes avant d'exécuter les commandes git :

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  SESSION_ID=$(echo "$input" | jq -r '.session_id')

  CACHE_FILE="/tmp/statusline-git-cache-$SESSION_ID"
  CACHE_MAX_AGE=5  # seconds

  cache_is_stale() {
      [ ! -f "$CACHE_FILE" ] || \
      # stat -f %m is macOS, stat -c %Y is Linux
      [ $(($(date +%s) - $(stat -f %m "$CACHE_FILE" 2>/dev/null || stat -c %Y "$CACHE_FILE" 2>/dev/null || echo 0))) -gt $CACHE_MAX_AGE ]
  }

  if cache_is_stale; then
      if git rev-parse --git-dir > /dev/null 2>&1; then
          BRANCH=$(git branch --show-current 2>/dev/null)
          STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
          MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')
          echo "$BRANCH|$STAGED|$MODIFIED" > "$CACHE_FILE"
      else
          echo "||" > "$CACHE_FILE"
      fi
  fi

  IFS='|' read -r BRANCH STAGED MODIFIED < "$CACHE_FILE"

  if [ -n "$BRANCH" ]; then
      echo "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH +$STAGED ~$MODIFIED"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os, time

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  session_id = data['session_id']

  CACHE_FILE = f"/tmp/statusline-git-cache-{session_id}"
  CACHE_MAX_AGE = 5  # seconds

  def cache_is_stale():
      if not os.path.exists(CACHE_FILE):
          return True
      return time.time() - os.path.getmtime(CACHE_FILE) > CACHE_MAX_AGE

  if cache_is_stale():
      try:
          subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
          branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
          staged = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
          modified = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
          staged_count = len(staged.split('\n')) if staged else 0
          modified_count = len(modified.split('\n')) if modified else 0
          with open(CACHE_FILE, 'w') as f:
              f.write(f"{branch}|{staged_count}|{modified_count}")
      except:
          with open(CACHE_FILE, 'w') as f:
              f.write("||")

  with open(CACHE_FILE) as f:
      branch, staged, modified = f.read().strip().split('|')

  if branch:
      print(f"[{model}] 📁 {directory} | 🌿 {branch} +{staged} ~{modified}")
  else:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const fs = require('fs');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const sessionId = data.session_id;

      const CACHE_FILE = `/tmp/statusline-git-cache-${sessionId}`;
      const CACHE_MAX_AGE = 5; // seconds

      const cacheIsStale = () => {
          if (!fs.existsSync(CACHE_FILE)) return true;
          return (Date.now() / 1000) - fs.statSync(CACHE_FILE).mtimeMs / 1000 > CACHE_MAX_AGE;
      };

      if (cacheIsStale()) {
          try {
              execSync('git rev-parse --git-dir', { stdio: 'ignore' });
              const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
              const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              fs.writeFileSync(CACHE_FILE, `${branch}|${staged}|${modified}`);
          } catch {
              fs.writeFileSync(CACHE_FILE, '||');
          }
      }

      const [branch, staged, modified] = fs.readFileSync(CACHE_FILE, 'utf8').trim().split('|');

      if (branch) {
          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} +${staged} ~${modified}`);
      } else {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="windows-configuration">
  Configuration Windows
</h3>

Sur Windows, Claude Code exécute les commandes de barre de statut via Git Bash quand Git Bash est installé, ou via PowerShell quand Git Bash est absent.

Git Bash traite les barres obliques inverses non échappées comme des caractères d'échappement, donc un chemin de style Windows comme `C:\Users\username\script.mjs` atteint le script runner avec ses séparateurs supprimés et la commande échoue sans erreur visible. Écrivez les chemins de fichiers dans la chaîne `command` avec des barres obliques avant, comme indiqué dans les exemples ci-dessous. Le raccourci `~` fonctionne également et se développe dans votre répertoire personnel Windows.

Pour exécuter un script PowerShell comme votre barre de statut, invoquez-le via `powershell`. Cela fonctionne que Claude Code achemine la commande via Git Bash ou PowerShell :

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "powershell -NoProfile -File C:/Users/username/.claude/statusline.ps1"
    }
  }
  ```

  ```powershell statusline.ps1 theme={null}
  $input_json = $input | Out-String | ConvertFrom-Json
  $cwd = $input_json.cwd
  $model = $input_json.model.display_name
  $used = $input_json.context_window.used_percentage
  $dirname = Split-Path $cwd -Leaf

  if ($used) {
      Write-Host "$dirname [$model] ctx: $used%"
  } else {
      Write-Host "$dirname [$model]"
  }
  ```
</CodeGroup>

Ou, quand Git Bash est installé, exécutez un script Bash directement :

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "~/.claude/statusline.sh"
    }
  }
  ```

  ```bash statusline.sh theme={null}
  #!/usr/bin/env bash
  input=$(cat)
  cwd=$(echo "$input" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
  model=$(echo "$input" | grep -o '"display_name":"[^"]*"' | cut -d'"' -f4)
  dirname="${cwd##*[/\\]}"
  echo "$dirname [$model]"
  ```
</CodeGroup>

<h2 id="subagent-status-lines">
  Barres de statut des sous-agents
</h2>

Le paramètre `subagentStatusLine` rend un corps de ligne personnalisé pour chaque [sous-agent](/docs/fr/sub-agents) affiché dans le panneau d'agent sous l'invite. Utilisez-le pour remplacer la ligne par défaut `name · description · token count` par votre propre formatage.

```json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "~/.claude/subagent-statusline.sh"
  }
}
```

La commande s'exécute une fois par cycle d'actualisation avec toutes les lignes de sous-agent visibles passées en tant qu'objet JSON unique sur stdin. L'entrée inclut les [champs d'entrée de hook de base](/docs/fr/hooks#common-input-fields), un champ `columns` avec la largeur de ligne utilisable, et un tableau `tasks`. Chaque tâche a `id`, `name`, `type`, `status`, `description`, `label`, `startTime`, `model`, `contextWindowSize`, `tokenCount`, `tokenSamples` et `cwd`.

Le champ `model` par tâche est l'ID de modèle résolu sur lequel la tâche s'exécute. `contextWindowSize` est la fenêtre de contexte de ce modèle en tokens, calculée de la même manière que `context_window.context_window_size` de la barre de statut principale, vous pouvez donc rendre un pourcentage par ligne à partir de `tokenCount`. Les deux champs nécessitent Claude Code v2.1.205 ou une version ultérieure et sont omis pour une tâche dont le modèle n'est pas encore résolu.

Écrivez une ligne JSON sur stdout par ligne que vous voulez remplacer, sous la forme `{"id": "<task id>", "content": "<row body>"}`. La chaîne `content` est rendue telle quelle, y compris les couleurs ANSI et les hyperliens OSC 8. Omettez le `id` d'une tâche pour conserver le rendu par défaut pour cette ligne ; émettez une chaîne `content` vide pour la masquer.

Les mêmes portes de confiance et `disableAllHooks` qui s'appliquent à `statusLine` s'appliquent ici. Les plugins peuvent expédier un `subagentStatusLine` par défaut dans leur [`settings.json`](/docs/fr/plugins-reference#standard-plugin-layout).

<h2 id="tips">
  Conseils
</h2>

* **Tester avec une entrée fictive** : `echo '{"model":{"display_name":"Opus"},"workspace":{"current_dir":"/home/user/project"},"context_window":{"used_percentage":25},"session_id":"test-session-abc"}' | ./statusline.sh`
* **Garder la sortie courte** : la barre de statut a une largeur limitée, donc une sortie longue peut être tronquée ou s'enrouler maladroitement
* **Mettre en cache les opérations lentes** : votre script s'exécute fréquemment pendant les sessions actives, donc les commandes comme `git status` peuvent causer des ralentissements. Voir l'[exemple de mise en cache](#cache-expensive-operations) pour savoir comment gérer cela.

Les projets communautaires comme [ccstatusline](https://github.com/sirmalloc/ccstatusline) et [starship-claude](https://github.com/martinemde/starship-claude) fournissent des configurations pré-construites avec des thèmes et des fonctionnalités supplémentaires.

<h2 id="troubleshooting">
  Dépannage
</h2>

**La barre de statut n'apparaît pas**

* Vérifiez que votre script est exécutable : `chmod +x ~/.claude/statusline.sh`
* Vérifiez que votre script affiche sur stdout, pas stderr
* Exécutez votre script manuellement pour vérifier qu'il produit une sortie
* Sur Windows avec Git Bash installé, les barres obliques inverses dans le chemin `command` sont probablement consommées comme caractères d'échappement avant l'exécution du script. Utilisez des barres obliques avant dans le chemin. Voir [Configuration Windows](#windows-configuration).
* Si `disableAllHooks` est défini sur `true` dans vos paramètres, la barre de statut est également désactivée. Supprimez ce paramètre ou définissez-le sur `false` pour le réactiver.
* Exécutez `claude --debug` pour enregistrer le code de sortie et stderr de la première invocation de barre de statut dans une session
* Demandez à Claude de lire votre fichier de paramètres et d'exécuter la commande `statusLine` directement pour afficher les erreurs

**La barre de statut affiche `--` ou des valeurs vides**

* Les champs peuvent être `null` avant la fin de la première réponse API
* Gérez les valeurs null dans votre script avec des valeurs par défaut de secours telles que `// 0` dans jq
* Redémarrez Claude Code si les valeurs restent vides après plusieurs messages

**Le pourcentage de contexte affiche des valeurs inattendues**

* Utilisez `used_percentage` pour l'état de contexte le plus simple et précis
* Le pourcentage de contexte peut différer de la sortie `/context` en raison du moment où chacun est calculé

**Les liens OSC 8 ne sont pas cliquables**

* Vérifiez que votre terminal supporte les hyperliens OSC 8 (iTerm2, Kitty, WezTerm)

* Terminal.app ne supporte pas les liens cliquables

* Si le texte du lien apparaît mais n'est pas cliquable, Claude Code peut ne pas avoir détecté le support des hyperliens dans votre terminal. Cela affecte couramment Windows Terminal et d'autres émulateurs ne figurant pas dans la liste de détection automatique. Définissez la variable d'environnement `FORCE_HYPERLINK` pour remplacer la détection avant de lancer Claude Code :

  ```bash theme={null}
  FORCE_HYPERLINK=1 claude
  ```

  Dans PowerShell, définissez d'abord la variable dans la session actuelle :

  ```powershell theme={null}
  $env:FORCE_HYPERLINK = "1"; claude
  ```

* Les sessions SSH et tmux peuvent supprimer les séquences OSC selon la configuration

* Si les séquences d'échappement apparaissent comme du texte littéral comme `\e]8;;`, utilisez `printf '%b'` au lieu de `echo -e` pour une gestion plus fiable des échappements

**Problèmes d'affichage avec les séquences d'échappement**

* Les séquences d'échappement complexes (couleurs ANSI, liens OSC 8) peuvent occasionnellement causer une sortie brouillée si elles chevauchent d'autres mises à jour UI
* Si vous voyez du texte corrompu, essayez de simplifier votre script en sortie en texte brut
* Les barres de statut multi-lignes avec codes d'échappement sont plus sujettes aux problèmes de rendu que le texte brut sur une seule ligne

**Confiance de l'espace de travail requise**

* La commande de barre de statut s'exécute uniquement si vous avez accepté la boîte de dialogue de confiance de l'espace de travail pour le répertoire actuel. Parce que `statusLine` exécute une commande shell, elle nécessite la même acceptation de confiance que les hooks et autres paramètres exécutant des shells.
* Si la confiance n'est pas acceptée, vous verrez la notification `statusline skipped · restart to fix` au lieu de votre sortie de barre de statut. Redémarrez Claude Code et acceptez l'invite de confiance pour l'activer.

**Erreurs de script ou blocages**

* Les scripts qui se terminent avec des codes non nuls ou ne produisent aucune sortie font que la barre de statut devient vide
* Les scripts lents bloquent la barre de statut de se mettre à jour jusqu'à ce qu'ils se terminent. Gardez les scripts rapides pour éviter une sortie obsolète.
* Si une nouvelle mise à jour se déclenche pendant qu'un script lent s'exécute, le script en cours est annulé
* Testez votre script indépendamment avec une entrée fictive avant de le configurer

**Les notifications partagent la ligne de la barre de statut**

* Les notifications système comme les erreurs de serveur MCP et les mises à jour automatiques s'affichent sur le côté droit de la même ligne que votre barre de statut. Les notifications transitoires telles que l'avertissement de contexte faible circulent également dans cette zone.
* L'activation du mode verbeux ajoute un compteur de jetons à cette zone
* Sur les terminaux étroits, ces notifications peuvent tronquer votre sortie de barre de statut
