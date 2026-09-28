> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tester les plugins avec des evals

> Écrivez des cas d'eval pour votre plugin Claude Code, exécutez-les avec claude plugin eval, notez les résultats, comparez-les avec une base de référence sans plugin et contrôlez CI sur le score.

La commande shell `claude plugin eval` exécute votre [plugin](/docs/fr/plugins/overview) par rapport à une suite de cas de test et note les résultats. Chaque cas est une invite réaliste plus un ou plusieurs évaluateurs. Un évaluateur est une vérification réussi/échoué sur ce que Claude a produit, comme une regex sur la réponse, si un outil particulier a été appelé, ou une rubrique qu'un deuxième modèle juge sur la réponse.

Vous n'avez pas à écrire la suite à la main ; `claude plugin eval init` vous pose des questions sur votre plugin, propose les cas et les évaluateurs, les essaie, et écrit les fichiers. Vous pouvez aussi demander à Claude de faire la même chose à partir d'une session que vous avez déjà ouverte.

Utilisez les evals pour :

* Mesurer la fiabilité avec laquelle votre plugin oriente Claude vers le bon résultat
* Détecter les régressions lorsque vous modifiez le plugin ou qu'un nouveau modèle est lancé
* Voir ce que le plugin contribue par rapport à aucun plugin

Cette page est destinée aux auteurs de plugins et de skills qui ont un plugin fonctionnel et qui veulent tester son comportement, et aux équipes qui contrôlent les modifications de plugins dans CI. Son format de cas est séparé du fichier `evals/evals.json` que le [plugin skill-creator](/docs/fr/skills#run-evals-with-skill-creator) utilise. Pour créer un plugin, voir [Créer un plugin](/docs/fr/plugins/create) ; pour vérifier les fichiers d'un plugin pour les erreurs de syntaxe et de schéma plutôt que son comportement, utilisez [`claude plugin validate`](/docs/fr/plugins/cli-reference#plugin-validate).

<Note>
  Chaque exécution d'eval et chaque évaluateur de juge est un vrai appel de modèle sur votre compte, compté par rapport à l'utilisation de votre plan ou votre facture API, alors vérifiez d'abord les [exigences](#requirements). Ensuite [créez votre première suite d'eval](#create-your-first-eval-suite), ou allez à [Exécuter les evals dans CI](#run-evals-in-ci) si vous en avez déjà une.
</Note>

<h2 id="requirements">
  Exigences
</h2>

Pour exécuter les evals de plugin, vous avez besoin de :

* Claude Code v2.1.269 ou ultérieur. Exécutez `claude --version` pour vérifier et `claude update` pour mettre à jour.
* Un répertoire de plugin avec un manifeste `plugin.json` ou `.claude-plugin/plugin.json`, ou un [plugin de répertoire de skills](/docs/fr/plugins/loading#plugins-shared-through-a-repository).
* La même authentification et le même fournisseur de modèle que vos sessions Claude Code normales. Les exécutions d'eval, les évaluateurs notés par le juge, et `claude plugin eval init` appellent le modèle avec vos identifiants, donc ils comptent par rapport à vos limites d'utilisation du plan ou votre facture API. Lorsque la commande rapporte un coût, le chiffre est une [estimation du prix catalogue](/docs/fr/costs) de ces appels.

<h2 id="how-an-eval-run-works">
  Comment fonctionne une exécution d'eval
</h2>

Une suite d'eval vit dans un répertoire appelé `evals/` à l'intérieur de votre plugin, disposé comme [Écrire et affiner les cas](#write-and-refine-cases) le montre. Chaque cas est son propre sous-répertoire avec une [invite](#set-run-limits-and-tools-in-prompt-md) et un ou plusieurs [évaluateurs](#grade-the-result). L'invite est quelque chose qu'une personne utilisant votre plugin pourrait taper, comme une demande que l'une de ses skills devrait gérer.

<h3 id="what-happens-in-a-run">
  Ce qui se passe dans une exécution
</h3>

Pour chaque exécution d'un cas, Claude Code démarre une nouvelle session [isolée](#how-runs-are-isolated) [non-interactive](/docs/fr/headless) avec seulement votre plugin chargé, envoie l'invite, et laisse Claude travailler jusqu'à ce qu'il finisse ou atteigne la limite de tour ou de temps du cas. Chaque évaluateur vérifie ensuite la réponse finale, la transcription, ou un fichier que Claude a créé, et réussit ou échoue.

<h3 id="how-a-case-is-scored">
  Comment un cas est noté
</h3>

Une exécution d'un agent non-déterministe vous dit peu de choses, donc chaque cas s'exécute trois fois par défaut. Le score d'une exécution est la fraction de ses évaluateurs qui ont réussi, pondérée si vous définissez des poids, et le score du cas est la moyenne sur ses exécutions. Un cas réussit lorsque son score atteint le [`--threshold`](#command-options), `1.0` par défaut.

Dans les appels de modèle, une suite fait environ cas × exécutions appels d'agent avec le plugin et autant à nouveau pour la [base de référence sans plugin](#the-no-plugin-baseline), plus trois appels de juge courts par évaluateur `llm` ou `baseline` par exécution.

<h3 id="the-no-plugin-baseline">
  La base de référence sans plugin
</h3>

Un score élevé en soi ne vous dit pas si le plugin a aidé, car Claude pourrait faire aussi bien sans lui. Pour séparer les deux, les exécutions de chaque cas sont répétées sans plugin chargé par défaut, et vous obtenez deux scores, `WITH` et `W/OUT`. Leur différence, `Δ`, est ce que le plugin a contribué. Si un cas marque 1.0 à la fois avec et sans le plugin, le plugin n'est pas ce qui l'a fait réussir.

Les deux ensembles d'exécutions sont appelés le bras with et le bras without ; [Comparer avec une base de référence sans plugin](#compare-against-a-no-plugin-baseline) couvre comment les évaluateurs sont notés sur les deux bras et comment désactiver la base de référence.

<h2 id="create-your-first-eval-suite">
  Créez votre première suite d'eval
</h2>

Cette procédure écrit un cas pour votre propre plugin, l'exécute, et lit le résultat. Avant de commencer, assurez-vous que vous avez :

* Claude Code v2.1.269 ou ultérieur et les autres [exigences](#requirements)
* Un terminal ouvert au répertoire racine de votre plugin, celui contenant `plugin.json` ou `.claude-plugin/plugin.json`
* Une skill dans le plugin que vous voulez tester, et une demande qu'un utilisateur taperait qui devrait la déclencher

<Steps>
  <Step title="Créer les cas">
    À partir de la racine du plugin, exécutez :

    ```bash theme={null}
    claude plugin eval init
    ```

    Si Claude Code ne fait pas déjà confiance à ce répertoire, il demande d'abord `Trust this plugin directory?` ; répondez `y`.

    Une session Claude Code interactive s'ouvre ensuite. Claude lit votre plugin et vous demande à quoi ressemble un bon résultat, propose des invites qui devraient et ne devraient pas déclencher le plugin, conçoit des évaluateurs pour chacun, les teste une fois pour vérifier qu'ils se comportent, et écrit un répertoire de cas par invite sous `evals/`, chacun nommé d'après son invite.

    Lorsque Claude vous dit que la suite est prête, quittez cette session avec `/exit` ou Ctrl+D pour revenir à votre shell.

    Si vous avez déjà une session Claude Code ouverte à la racine du plugin, vous pouvez plutôt demander à Claude d'exécuter `claude plugin eval init`. Claude exécute la commande et vous pose ensuite les mêmes questions dans cette conversation.

    Si vous préférez écrire un cas vous-même pour voir exactement ce que contiennent les fichiers, suivez [Écrire un cas à la main](#write-a-case-manually) et revenez ici pour l'exécuter.
  </Step>

  <Step title="Exécuter la suite">
    De retour à votre shell à la racine du plugin, exécutez chaque cas sous `evals/` :

    ```bash theme={null}
    claude plugin eval .
    ```

    Vous avez déjà fait confiance à ce répertoire lors de l'étape 1, donc l'exécution commence immédiatement. Si vous avez écrit le cas à la main à la place, l'exécution demande d'abord `Trust this plugin directory? [y/N]` ; répondez `y`. [Ce qu'une exécution peut accéder](#security) explique à quoi vous acceptez.

    Chaque cas s'exécute trois fois avec votre plugin et trois fois sans lui, donc un cas est six exécutions. Une ligne de progression s'affiche à la fin de chaque exécution, avec le score de cette exécution et le verdict de chaque évaluateur.
  </Step>

  <Step title="Lire le résumé">
    Lorsque la suite se termine, vous voyez un tableau récapitulatif, suivi de l'endroit où le rapport est allé :

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH` est le score du cas avec votre plugin chargé, `W/OUT` est le score sans lui, et un `Δ` positif signifie que le plugin a augmenté le score. `COST` est une estimation du prix catalogue des appels de modèle, et `NOTES` affiche l'explication de l'évaluateur défaillant de poids le plus élevé, ou l'erreur de l'exécution, du bras with.
  </Step>

  <Step title="Ouvrir le rapport et itérer">
    Ouvrez l'URL `Published:`, ou le chemin `Report:` lorsqu'aucune ligne `Published:` n'apparaît, pour voir le verdict de chaque évaluateur et l'explication pour chaque exécution, et pour les évaluateurs `llm` les votes du juge et l'extrait qu'il a jugé. La ligne `Published:` n'apparaît que lorsque votre compte peut [publier des rapports](#html-report).

    La découverte la plus courante au premier abord est un `Δ` proche de zéro avec l'évaluateur `tool_used: Skill` du cas échouant, ce qui signifie que Claude ne choisit pas votre skill sur une formulation naturelle. Ajustez la [`description`](/docs/fr/skills#frontmatter-reference) de la skill, exécutez `claude plugin eval .` à nouveau, et comparez.

    Pour itérer sur un cas à moindre coût, exécutez un seul bras une fois. Une seule exécution est bruyante, donc confirmez tout changement aux trois exécutions par défaut avant de lui faire confiance. Avec un seul bras, le tableau affiche les colonnes `SCORE` et `PASS%` au lieu de `WITH`, `W/OUT`, et `Δ` :

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    Remplacez `<case-name>` par l'un des noms de répertoire sous `evals/`.
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  Écrire et affiner les cas
</h2>

Les cas que `claude plugin eval init` écrit sont des fichiers simples que vous pouvez ouvrir, modifier et ajouter. Un cas est un répertoire sous le répertoire eval du plugin qui contient un `prompt.md`, un `case.yaml`, ou les deux. Pour regrouper les cas, imbriquez-les sous un répertoire qui n'est pas lui-même un cas ; tout ce qui se trouve à l'intérieur d'un répertoire de cas, comme `graders/` et les fichiers de fixture, appartient à ce cas.

Ceci est la disposition que `claude plugin eval init` écrit et celle à utiliser pour les nouvelles suites. La [référence de la suite d'eval](#eval-suite-reference) a l'arborescence complète, y compris les mocks et les résultats :

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  Écrire un cas à la main
</h3>

Faire écrire les cas par Claude avec `claude plugin eval init` est le chemin recommandé. Pour en écrire un vous-même à la place, commencez par un modèle vierge. La commande suivante écrit un cas nommé `first-case` avec un `prompt.md` d'espace réservé et un évaluateur d'espace réservé, et n'exécute rien :

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

Dans `prompt.md`, vous écrivez le message que Claude reçoit dans chaque exécution, et définissez les limites de l'exécution et les outils que le cas peut utiliser dans son frontmatter. Ouvrez `evals/first-case/prompt.md` et remplacez le corps d'espace réservé par une demande que l'une de vos skills devrait traiter, formulée de la manière qu'un utilisateur la taperait plutôt que de nommer la skill. Cet exemple concerne une skill qui rédige des messages de commit ; utilisez votre propre demande :

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

Chaque exécution commence dans un répertoire de travail vide, donc mettez tout ce dont la tâche a besoin dans l'invite elle-même, ou [configurez l'espace de travail](#add-setup-or-history-with-case-yaml) d'abord.

Le [liste complète des champs frontmatter](#prompt-md-fields) couvre le modèle, le délai d'expiration, les balises et les variables d'environnement.

Chaque fichier sous `graders/` est une vérification appliquée après l'exécution. Ouvrez `evals/first-case/graders/criteria.md` et remplacez l'espace réservé par une rubrique pour le modèle juge, écrite comme des conditions PASS et FAIL concrètes :

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

Ensuite, ajoutez un deuxième évaluateur qui vérifie si votre skill est ce qui a produit la réponse. Créez `evals/first-case/graders/skill-fired.md`, en remplaçant `your-skill-name` par le nom du répertoire de votre skill sous `skills/`, qui est le nom que Claude invoque :

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

Cela réussit lorsque Claude a invoqué cette skill au moins une fois pendant l'exécution, y compris par sa forme `plugin-name:skill-name` avec espace de noms.

[Types d'évaluateurs](#grader-types) énumère les autres vérifications disponibles, comme la correspondance d'une regex ou la confirmation qu'un fichier a été créé.

Avec les deux fichiers enregistrés, exécutez le cas de la manière que le [démarrage rapide](#create-your-first-eval-suite) le fait, avec `claude plugin eval .` à partir de la racine du plugin.

<h3 id="set-run-limits-and-tools-in-prompt-md">
  Définir les limites d'exécution et les outils dans prompt.md
</h3>

Définissez `max_turns`, `timeout_seconds`, `model`, `tags` d'un cas, et les `allowed_tools` qu'il peut utiliser dans le frontmatter de `prompt.md` ; la référence [frontmatter de prompt.md](#prompt-md-fields) énumère chaque champ et sa valeur par défaut.

Claude reçoit le corps exactement comme vous l'avez écrit. Les mentions `@path` dedans ne sont pas développées en pièces jointes de fichier, donc si Claude a besoin de lire un fichier, accordez un outil pour cela dans `allowed_tools`.

<h3 id="grade-the-result">
  Choisir et pondérer les évaluateurs
</h3>

Le frontmatter d'un évaluateur définit son `type`, et optionnellement un `weight` qui le fait compter pour plus du score de l'exécution et un [`arm`](#compare-against-a-no-plugin-baseline) qui contrôle comment il est noté par rapport à la base de référence. Des six types, `regex`, `tool_used`, `tool_order`, et `file_exists` sont calculés à partir de la transcription et des fichiers et ne coûtent rien, tandis que `llm` et `baseline` appellent un modèle juge et s'ajoutent au coût de l'exécution.

Il n'y a pas d'évaluateurs de code personnalisé.

[Types d'évaluateurs](#grader-types) énumère les options de chaque type et la condition de réussite, et [ce qu'un évaluateur peut regarder](#what-a-grader-can-look-at) énumère les valeurs que `target` et `focus` acceptent.

Le juge pour les évaluateurs `llm` et `baseline` est un petit modèle rapide par défaut. Passez `--judge-model sonnet` ou un ID de modèle complet pour en utiliser un plus fort pour les rubriques nuancées.

<h4 id="choose-graders-that-give-a-stable-signal">
  Choisir des évaluateurs qui donnent un signal stable
</h4>

Un évaluateur `llm` demande à un modèle un verdict, donc sa réponse peut différer entre les exécutions, et elle diffère plus le plus long le texte qu'il doit lire. Ces habitudes gardent les scores d'une suite assez stables pour être dignes de confiance :

* Pour une sortie longue comme un fichier généré, notez-la avec un évaluateur `regex` sur le contenu du fichier, qui vérifie le fichier entier de la même manière à chaque fois. Gardez les évaluateurs `llm` pour les sorties courtes, avec des rubriques écrites comme des conditions PASS et FAIL concrètes.
* Donnez à chaque cas un évaluateur sur le résultat, comme le message final ou un fichier produit, et un sur la façon dont Claude y est arrivé, comme `tool_used` ou `tool_order`. Ensemble, ils vous disent à la fois si la réponse était correcte et si votre plugin l'a produite.
* Si l'évaluateur `tool_used: Skill` d'un cas réussit mais `Δ` est négatif, soupçonnez le juge avant le plugin. Un petit modèle juge peut marquer une réponse correcte comme fausse parce qu'elle est formatée différemment de ce que la rubrique décrit. Réexécutez avec `--judge-model sonnet`, et resserrez la rubrique pour que le formatage ne décide pas du verdict.
* Pour vérifier qu'une construction ou un test a réussi à l'intérieur de l'exécution, demandez à Claude de l'exécuter et d'écrire le résultat dans un fichier, notez ce fichier, et affirmez que la commande a été exécutée avec un évaluateur `tool_used` dont `input_match` nomme la commande.

<h3 id="compare-against-a-no-plugin-baseline">
  Noter par rapport à la base de référence sans plugin
</h3>

Lorsqu'un plugin est en test, chaque cas s'exécute dans deux bras par défaut. Le bras with est ses exécutions avec le plugin chargé, et le bras without est le même nombre d'exécutions sans aucun plugin. Le résumé et le rapport affichent les deux scores et `Δ`, le score du bras with moins le score du bras without.

Passez `--ablation none` pour exécuter seulement le bras with, ce qui réduit de moitié le coût lorsque vous n'avez pas besoin de la comparaison, comme lors de l'itération sur les évaluateurs.

Dans une exécution à deux bras, certains évaluateurs sont rapportés avec `scored: false`. Une vérification comme « la skill a été invoquée » ne peut jamais réussir sans le plugin, donc la compter pousserait le bras without vers zéro et gonflerait `Δ`. Pour garder les deux bras comparables, Claude Code exclut ces évaluateurs du score dans les deux bras et les rapporte dans le bras with comme des indicateurs réussi/échoué uniquement. Cela inclut :

* Chaque évaluateur `tool_used` dont `tool` est `Skill`
* Chaque évaluateur `regex` avec `target: mock_calls` et chaque évaluateur `llm` avec `focus: mock_calls`, lorsque chaque [serveur mocké](#mock-mcp-servers) dans le cas en est un que votre plugin déclare
* Tout évaluateur que vous marquez `arm: with-only`

Trois paramètres changent cette exclusion :

* **Chaque évaluateur exclu** : si chaque évaluateur dans un cas en est un, ils sont notés normalement à la place, puisqu'il n'y aurait rien d'autre à noter.
* **`arm: both`** : définissez `arm: both` sur un évaluateur pour le noter dans les deux bras indépendamment, ce que vous voulez pour une vérification « ne doit pas invoquer la skill » avec `min: 0` et `max: 0`.
* **`--ablation none`** : sous `--ablation none`, rien n'est exclu, donc la même suite peut produire un score absolu différent dans les deux modes.

<h3 id="use-a-different-eval-directory">
  Utiliser un répertoire d'eval différent
</h3>

Si `evals/` est déjà pris par un autre outil, gardez la suite dans un répertoire différent. Vous pouvez enregistrer ce répertoire dans le `plugin.json` du plugin pour que chaque exécution et chaque collaborateur l'utilise, ou le passer sur la ligne de commande pour une seule exécution :

* **Dans `plugin.json`** : ajoutez `"experimental": { "evals": "quality/evals" }`.
* **Sur la ligne de commande** : passez `--eval-dir quality/evals` à la fois à `claude plugin eval` et `claude plugin eval init`.

Si vous définissez les deux, le répertoire du drapeau est utilisé. Donnez un chemin relatif de noms de répertoire simples comme `qa` ou `quality/evals`. Un chemin absolu ou contenant `..` n'est pas accepté : comme valeur de drapeau c'est une erreur, tandis qu'une valeur de manifeste inutilisable imprime une ligne `Warning:` et l'exécution utilise `evals/` à la place. Les cas, les résultats, et la sortie `init` se déplacent tous vers ce répertoire.

<h2 id="set-up-fixtures-and-mocks">
  Configurer les fixtures et les mocks
</h2>

Un cas peut avoir besoin de plus qu'une invite : des fichiers ou un référentiel git dans l'espace de travail, une conversation antérieure à continuer, ou des réponses des serveurs MCP avec lesquels votre plugin communique. Chacun d'eux est configuré à côté du cas pour que les exécutions restent reproductibles.

<h3 id="add-setup-or-history-with-case-yaml">
  Ensemencer l'espace de travail ou la conversation
</h3>

Chaque exécution commence dans un espace de travail vide. Lorsqu'un cas a besoin de plus que l'invite, ajoutez un `case.yaml` à côté de `prompt.md` avec un bloc `context` :

* **Fichiers de fixture ou un référentiel git** : écrivez un script Bash dans le répertoire de cas et nommez-le dans `context.scaffold_script`. Le script s'exécute en tant que vous, en dehors du sandbox de l'agent, et seulement lorsque vous passez `--scaffold`, donc passez ce drapeau seulement pour les suites que vous ou votre organisation avez écrites.
* **Une conversation antérieure à continuer** : enregistrez la transcription en tant que fichier `.jsonl` et nommez-la dans `context.history_file`, et l'invite du cas devient le prochain tour utilisateur.
* **Répertoires de fixture que Claude peut lire pendant l'exécution** : énumérez-les dans `context.add_dirs`.

Un `case.yaml` a également besoin de `schema_version: "1.1"` et `name` ; la référence [champs case.yaml](#case-yaml-fields) a la liste complète.

Ce `case.yaml` ensemence un espace de travail à partir d'un script et laisse Claude lire les fixtures à partir d'un répertoire `resources/` :

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  Serveurs MCP fictifs
</h3>

Vous pouvez évaluer un plugin dont les skills appellent des outils MCP sans le vrai service derrière eux. Mettez un fichier Markdown par outil sous `evals/mocks/<server>/<tool>.md` pour la suite entière, ou sous le répertoire `mocks/` propre d'un cas pour un cas, où `<server>` est le nom du serveur dans la [configuration MCP](/docs/fr/plugins/components#mcp-servers) de votre plugin.

Une exécution ne démarre jamais les vrais serveurs MCP de votre plugin à moins que vous le demandiez. Claude Code enregistre un serveur de substitution sous le nom propre de chaque serveur. Les outils avec un fichier mock répondent à partir de celui-ci et sont autorisés sans une concession `--allow-tools`, et un outil sans fichier mock n'est pas disponible pour Claude. Un serveur sans aucun mock du tout apparaît dans la ligne de progression `mocked:` du cas comme `plugin_<plugin>_<server>[not started: no mock]`.

Le corps du fichier est ce que l'outil retourne à Claude. Ce mock se substitue à un outil `create_issue` sur un serveur nommé `tracker`, vérifie l'entrée que Claude envoie, et renvoie le titre. Enregistrez-le en tant que `evals/mocks/tracker/create_issue.md` :

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

Le corps et le frontmatter d'un fichier mock acceptent ces options :

* **Substitutions** : insérez les champs de l'entrée de l'appel avec `{{input.<field>}}`, et le contenu d'un fichier de fixture à côté du mock avec `{{file:fixtures/{input.<field>}.json}}`.
* **`expect:`** : le bloc `expect:` protège l'entrée. Si un appel le viole, l'exécution s'arrête avec un score de 0 et enregistre pourquoi, donc un cas peut affirmer ce que votre plugin a demandé au serveur de faire.
* **`error: true`** : définissez `error: true` pour retourner le corps comme une erreur d'outil à la place.
* **`type: agent`** : définissez `type: agent` pour avoir un petit modèle répondre en tant que serveur à partir d'instructions dans le corps.

La [référence du fichier mock](#mock-files) énumère chaque clé et les fichiers `_server.md` et `_tools.json`.

Pour noter les appels eux-mêmes, pointez un évaluateur vers `target: mock_calls`.

Pour exécuter contre les vrais serveurs MCP du plugin à la place, passez l'un de ces drapeaux. De toute façon, ces processus s'exécutent en tant que vous, en dehors du sandbox de l'exécution, et leurs outils ont besoin d'une concession [`--allow-tools`](#grant-tools) :

* **`--allow-real-servers`** : démarrez le processus réel pour chaque serveur que vous n'avez pas mocké, et continuez à répondre aux outils mockés à partir de leurs fichiers
* **`--mocks off`** : ignorez `mocks/` entièrement et démarrez chaque serveur que le plugin déclare

<h4 id="replay-agent-mock-answers">
  Rejouer les réponses mock d'agent
</h4>

Un mock `type: agent` répond avec un appel au [`--judge-model`](#command-options), donc sa sortie varie entre les exécutions et change si vous changez le juge. Lorsqu'une exécution se termine sans erreur ou abandon, Claude Code enregistre chaque réponse qu'un mock d'agent a donnée sous le répertoire des résultats dans `mock-recordings/`.

Ouvrez `ADOPT.txt` là pour voir chaque enregistrement et le répertoire `.replay/<server>/` pour le copier dedans, à côté du mock qui l'a produit. Après avoir copié un enregistrement là, les exécutions ultérieures répondent à l'appel identique à partir de celui-ci sans appel de modèle. Validez `mocks/.replay/` avec le reste de `mocks/` pour que les exécutions CI soient reproductibles.

<h2 id="run-evals">
  Exécuter les evals
</h2>

Une fois qu'une suite existe, `claude plugin eval` l'exécute. Vous choisissez quel plugin et quels cas exécuter avec l'argument cible, accordez tous les outils que les cas ont besoin au-delà de l'ensemble en lecture seule avec `--allow-tools`, et contrôlez le nombre d'exécutions, les modèles, le coût et la sortie avec les autres options.

<h3 id="choose-what-to-evaluate">
  Choisir ce qu'il faut évaluer
</h3>

La plupart du temps, vous exécutez `claude plugin eval .` à partir de la racine du plugin, ce qui exécute chaque cas de la suite avec le plugin dans lequel vous vous tenez chargé. Pour exécuter un fichier de cas unique, ou pour évaluer un plugin que vous avez installé plutôt qu'un que vous développez, passez une cible différente :

| Cible                                                    | Ce qui s'exécute                                                                                                                                                                                                   |
| :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Le répertoire racine d'un plugin, comme `.`              | Chaque cas sous son répertoire d'eval, avec ce plugin chargé                                                                                                                                                       |
| Un fichier `prompt.md` ou `case.yaml` unique             | Ce cas, avec son plugin englobant chargé                                                                                                                                                                           |
| Un plugin installé par nom, `name` ou `name@marketplace` | Les cas dans le répertoire d'eval de la copie installée, avec la copie installée chargée. Les résultats sont écrits sous `./evals/results/` dans votre répertoire courant, ou `./<dir>/results/` avec `--eval-dir` |
| `name@skills-dir`                                        | La même chose, pour un [plugin de répertoire de skills](/docs/fr/plugins/loading#plugins-shared-through-a-repository)                                                                                                   |
| Omis                                                     | Le répertoire courant en tant que chemin                                                                                                                                                                           |

Ajoutez `--case <glob>` pour filtrer par nom de cas et `--tag <tag>` pour garder les cas avec l'une des balises données.

Mettez la cible avant `--tag`, `--allow-tools`, et `--json`. Les deux premiers prennent une liste et `--json` prend un chemin optionnel, donc chacun d'eux lit une cible qui suit comme sa propre valeur.

<h3 id="grant-tools">
  Accorder les outils
</h3>

Les exécutions ne s'arrêtent jamais pour demander la permission. Les outils intégrés qui ont besoin d'une concession que vous n'avez pas donnée, comme `Bash`, `Write`, `Edit`, `WebFetch`, et `WebSearch`, sont supprimés de la session, donc Claude ne peut pas les appeler du tout.

Une exécution permet seulement les outils en lecture seule que le cas énumère dans `allowed_tools`, à partir de `Read`, `Glob`, `Grep`, `NotebookRead`, `Skill`, `AskUserQuestion`, `Agent`, `TodoWrite`, et les outils de tâche `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, et `TaskStop`, plus tout ce que vous accordez avec `--allow-tools`. Cette concession s'applique à chaque cas de l'exécution. Pour laisser les cas utiliser `Bash`, `Write`, `Edit`, `WebFetch`, ou `WebSearch`, accordez-les vous-même :

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

Lorsqu'un cas a demandé un outil que vous n'avez pas accordé, la sortie de progression le énumère comme `not granted`. Les outils sur un serveur MCP [mocké](#mock-mcp-servers) n'ont besoin d'aucune concession. Les outils sur un vrai serveur MCP de plugin ont besoin à la fois du serveur démarré, avec `--allow-real-servers` ou `--mocks off`, et d'une concession par nom, comme `--allow-tools "mcp__plugin_my-plugin_github__*"` ; les outils MCP d'un plugin sont nommés `mcp__plugin_<plugin>_<server>__<tool>`.

Lorsque vous accordez `Bash` sous n'importe quelle forme, chaque commande s'exécute sous le [sandbox au niveau du système d'exploitation](/docs/fr/sandboxing) de Claude Code. Les écritures sont confinées à l'espace de travail de l'exécution, votre répertoire personnel et la configuration de Claude Code sont illisibles, et l'accès réseau est limité aux domaines que vous accordez avec `--allow-tools "WebFetch(domain:example.com)"`. Si vous accordez Bash ou PowerShell sur une machine sans backend de sandbox, Claude Code refuse chaque exécution plutôt que de l'exécuter sans confinement, et le cas affiche une erreur d'exécution et marque généralement 0. Windows natif n'a pas de backend, donc exécutez les suites accordant le shell sous WSL2 ; sur Linux, installez d'abord `bubblewrap` et `socat`. Voir les [prérequis du sandboxing](/docs/fr/sandboxing).

<h3 id="command-options">
  Options de commande
</h3>

Ce tableau couvre les options pour le nombre d'exécutions, les modèles, la notation, le coût, les concessions d'outils, les mocks et la sortie. Exécutez `claude plugin eval --help` pour la liste complète, qui inclut également `--case`, `--tag`, `--eval-dir`, `--no-scaffold`, `--report`, et `--verbose`.

| Option                     | Par défaut                                                                                                | Effet                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :-------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | `runs` de chaque cas, sinon 3                                                                             | Exécutions par cas par bras                                                                                                                                                                                                                                                                                                                                                        |
| `-j`, `--concurrency <n>`  | `1`                                                                                                       | Exécutez jusqu'à ce nombre d'exécutions d'agent à la fois, de 1 à 8. Elles partagent la limite de débit de votre compte, donc cela raccourcit le temps mural plutôt que d'augmenter le débit au-delà de cette limite. Les résultats gardent l'ordre des cas                                                                                                                        |
| `--model <model>`          | `model` de chaque cas, sinon `ANTHROPIC_MODEL` s'il est défini, sinon la valeur par défaut de Claude Code | Modèle pour l'agent en test. Épinglez-le dans CI pour qu'un déploiement de modèle ne soit pas confondu avec une régression de plugin                                                                                                                                                                                                                                               |
| `--judge-model <model>`    | Un petit modèle rapide                                                                                    | Modèle pour les évaluateurs `llm` et `baseline`                                                                                                                                                                                                                                                                                                                                    |
| `--ablation <mode>`        | `with-without` lorsqu'un plugin se résout, sinon `none`                                                   | Qu'il faut aussi exécuter chaque cas sans le plugin pour mesurer ce qu'il ajoute. `none` exécute un bras ; `with-without` ajoute la base de référence sans plugin                                                                                                                                                                                                                  |
| `--threshold <0..1>`       | `1.0`                                                                                                     | Un cas réussit lorsque son score du bras with est au moins cela. Tout cas en dessous le fait quitter la commande 1                                                                                                                                                                                                                                                                 |
| `--max-cost-usd <usd>`     | Pas de plafond                                                                                            | Un plafond sur l'estimation du coût du prix catalogue de l'exécution, pas sur l'utilisation du plan. Vérifié avant chaque exécution. Une fois dépensé, rien d'autre ne démarre ; les exécutions déjà en vol se terminent, donc la dépense peut dépasser le plafond par ces exécutions. Si une exécution est laissée non démarrée, la commande quitte 2 avec des résultats partiels |
| `--allow-tools <tools...>` | Aucun                                                                                                     | Accordez les outils au-delà de l'ensemble en lecture seule. Voir [Accorder les outils](#grant-tools)                                                                                                                                                                                                                                                                               |
| `--scaffold`               | Désactivé                                                                                                 | Exécutez le [`scaffold_script`](#add-setup-or-history-with-case-yaml) de chaque cas                                                                                                                                                                                                                                                                                                |
| `--trust-plugin`           | Désactivé                                                                                                 | Ignorez l'invite de confiance à la première exécution pour un plugin dont vous exécuteriez le code et la suite vous-même. Passez-le dans CI pour que le travail ne soit jamais refusé par ou laissé en attente à l'invite. Voir [Ce qu'une exécution peut accéder](#security)                                                                                                      |
| `--mocks <mode>`           | `record`                                                                                                  | `record` répond aux appels d'outils MCP à partir de [mocks](#mock-mcp-servers), ne démarre pas les vrais serveurs du plugin, et enregistre les réponses des mocks d'agent pour la relecture. `off` ignore les mocks et démarre les vrais serveurs MCP du plugin                                                                                                                    |
| `--allow-real-servers`     | Désactivé                                                                                                 | Avec `--mocks record`, démarrez aussi les vrais serveurs MCP du plugin pour les serveurs qui n'ont pas de mock                                                                                                                                                                                                                                                                     |
| `--json [path]`            | Désactivé                                                                                                 | Imprimez le [document de résultat](#json-result) sur stdout, ou écrivez-le dans un chemin se terminant par `.json`. L'exécution est silencieuse : pas de lignes de progression ou de tableau récapitulatif                                                                                                                                                                         |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                                                         | Où vont `aggregate-result.json` et `report.html`                                                                                                                                                                                                                                                                                                                                   |
| `--no-publish`             |                                                                                                           | Gardez le rapport HTML local. Voir [Rapport HTML](#html-report)                                                                                                                                                                                                                                                                                                                    |
| `--publish-report`         |                                                                                                           | Publiez le rapport même où il resterait local par défaut, comme une exécution qu'une session Claude Code a démarrée                                                                                                                                                                                                                                                                |
| `--keep-temp`              | Désactivé                                                                                                 | Gardez le répertoire sandbox de chaque exécution et imprimez son chemin, pour déboguer ce que Claude a produit                                                                                                                                                                                                                                                                     |

<h3 id="run-evals-in-ci">
  Exécuter les evals dans CI
</h3>

Dans votre travail CI, exécutez la suite avec `--json` pour écrire le résultat pour l'archivage, et échouez la construction sur le code de sortie. Passez `--trust-plugin` pour que le travail n'attende jamais à l'[invite de confiance à la première exécution](#security), épinglez les deux modèles pour que les scores soient comparables au fil du temps, gardez le rapport local, et définissez un plafond de coût comme limite supérieure :

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

Le code de sortie du travail vous dit ce qui s'est passé :

| Code de sortie | Signification                                                                                                                                                                                                                                               |
| :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0              | Chaque cas a marqué au ou au-dessus de `--threshold` et chaque fichier de cas a chargé                                                                                                                                                                      |
| 1              | Un cas a marqué en dessous du seuil, un fichier de cas n'a pas pu charger, aucun cas n'a été trouvé, une exécution n'a pas pu être démarrée, le répertoire du plugin n'est pas approuvé et `--trust-plugin` n'a pas été passé, ou une option était invalide |
| 2              | Exécution partielle : le plafond `--max-cost-usd` a été atteint, ou votre identifiant a été rejeté avant ou à la première exécution. `results.json` est toujours écrit avec `partial: true` et la raison                                                    |
| 130            | Interrompu. Les résultats partiels sont écrits                                                                                                                                                                                                              |
| 143            | Terminé, comme par un délai d'expiration CI                                                                                                                                                                                                                 |

Les problèmes d'écriture ou de publication du rapport HTML ne changent jamais le code de sortie.

Pour voir pourquoi un cas a marqué bas, exécutez-le localement sans `--json` pour que les lignes de progression par exécution et d'évaluateur s'impriment.

Un exécuteur CI a aussi besoin de ceux-ci en place :

* **Installation et identifiants** : un exécuteur CI a besoin d'une installation Claude Code et d'[identifiants dans l'environnement](/docs/fr/authentication) comme `ANTHROPIC_API_KEY`.
* **Confiance** : sans `--trust-plugin`, un travail dont le répertoire de checkout Claude Code ne fait pas déjà confiance a besoin de l'[invite de confiance à la première exécution](#trust-the-plugin-directory), et une exécution qui ne peut pas demander est refusée avec la sortie 1.
* **`init` dans CI** : `claude plugin eval init` a besoin d'un terminal pour vous poser ses questions ; dans CI, exécutez `claude plugin eval init --bare <name>` pour obtenir le modèle vierge.

Pour garder les coûts prévisibles, donnez aux suites rapides à chaque changement seulement des évaluateurs qui n'appellent pas un juge, utilisez `--ablation none` où vous n'avez pas besoin de `Δ`, et laissez les documents `partial: true` et les exécutions avec `skippedPaidGraders` en dehors de toute tendance que vous tracez.

<h2 id="read-the-results">
  Lire les résultats
</h2>

Chaque exécution avec au moins un cas écrit un répertoire `results/<timestamp>/` à l'intérieur du répertoire d'eval, contenant `aggregate-result.json` et `report.html`. Pour une cible de chemin qui est sous le plugin ; pour un plugin que vous avez nommé, c'est sous votre répertoire courant, comme le [tableau cible](#choose-what-to-evaluate) le montre. Le tableau récapitulatif, le JSON, et le rapport rendent tous les mêmes données de résultat.

<h3 id="html-report">
  Rapport HTML
</h3>

`report.html` est un fichier unique autonome qui ne fait aucune demande externe, donc vous pouvez le joindre à un travail CI ou l'ouvrir à partir du disque. Cet exemple est le haut d'un rapport pour une exécution de suite à trois cas avec `--threshold 0.8` ; le coût affiché est une estimation au prix catalogue et varie selon le modèle et le nombre de cas :

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="Haut d'un rapport d'eval : une ligne de verdict lisant « Plugin effect: +33.3 pts vs baseline, improved 2, flat 1, regressed 0 of 3 cases », cinq tuiles récapitulatives pour le score de la suite, le delta d'ablation, le score de base, les cas passant le seuil, et les exécutions parfaites, puis le premier cas avec son delta, sa barre de score, et une exécution dont les deux évaluateurs affichent tous deux un passage" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

Lisez-le de haut en bas :

* **La ligne de verdict et les tuiles** répondent à la question de savoir si le plugin a aidé sur l'ensemble de la suite. Le score de la suite est la moyenne des scores avec-plugin par cas, Ablation Δ est la distance à laquelle cela se situe au-dessus ou au-dessous du score de base, et Cases compte combien ont atteint le seuil. Perfect runs est la part des exécutions avec-plugin où chaque évaluateur a réussi.
* **Chaque carte de cas** affiche le propre `Δ` du cas et le score avec-plugin, avec une coche sur la barre au seuil. Un cas dont le `Δ` est négatif obtient un bord gauche rouge, donc les régressions se démarquent lorsque vous faites défiler.
* **À l'intérieur d'un cas**, les exécutions avec-plugin viennent en premier et les exécutions de base après. Chaque exécution énumère ses évaluateurs avec une puce de passage ou d'échec. Un évaluateur échoué est déjà développé avec son explication, et un évaluateur `llm` affiche également les votes du juge et la preuve qui lui a été montrée, ce qui est l'endroit où vous découvrez pourquoi une exécution a obtenu un score faible. Les évaluateurs qui ne comptent pas vers le score, comme `tool_used: Skill`, portent un badge `plugin-fired indicator`.
* **Prompt et Graders**, sous les exécutions, affichent l'invite du cas et la rubrique ou le motif de chaque évaluateur, afin que quelqu'un lisant le rapport sans la suite puisse voir ce qui a été demandé et ce qui comptait comme bon.

Si vous êtes connecté avec un abonnement claude.ai et que les [artifacts](/docs/fr/artifacts) sont disponibles pour votre compte, Claude Code publie également le rapport en tant qu'artifact privé et imprime `Published: <url>`. Passez `--no-publish` pour le garder local. Si aucune ligne `Published:` n'apparaît, comme avec l'authentification par clé API, le fichier local est le rapport.

Une exécution qu'une session Claude Code a démarrée, comme lorsque vous demandez à Claude d'exécuter la suite pour vous, reste également locale, et sa ligne `Report:` dit `kept local`. Ajoutez `--publish-report` à cette commande pour la publier.

<h3 id="json-result">
  Résultat JSON
</h3>

`aggregate-result.json`, et la sortie `--json`, est un document versionné avec `schemaVersion: 1` pour que les scripts CI l'analysent. Les noms de champs sont camelCase et les nouveaux champs sont ajoutés sans renommer les existants, donc écrivez votre script pour ignorer les champs qu'il ne reconnaît pas.

Ce sont les champs qu'un script de gating lit généralement. Le document porte également la configuration de la suite, chaque définition d'évaluateur, et les résultats d'évaluateur par exécution avec explications et preuves :

| Champ                                             | Signification                                                                                                                                                                                                                                       |
| :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `partial`, `partialReason`                        | `true` avec `cost_ceiling`, `interrupted`, ou `auth_failed` lorsque la suite n'a pas terminé. Laissez les résultats partiels en dehors des graphiques de tendance                                                                                   |
| `aggregates.overallScore`                         | Score de cas moyen sur la suite                                                                                                                                                                                                                     |
| `aggregates.casesPassed`, `aggregates.casesTotal` | Cas au ou au-dessus de `--threshold`, et le total                                                                                                                                                                                                   |
| `aggregates.meanDelta`                            | `Δ` moyen sur les cas, en mode à deux bras                                                                                                                                                                                                          |
| `cases[].name`                                    | Nom du cas                                                                                                                                                                                                                                          |
| `cases[].aggregates.score`                        | Score d'exécution du bras with moyen pour le cas                                                                                                                                                                                                    |
| `cases[].aggregates.delta`                        | Score du bras with moins score du bras without. Omis lorsque les bras ne sont pas comparables                                                                                                                                                       |
| `cases[].arms.with[].error`                       | `null`, ou pourquoi une exécution s'est terminée anormalement, comme `timed out after 300s`. Une exécution qui a démarré mais s'est terminée mal est toujours notée sur ce qu'elle a produit, donc une erreur non-null n'implique pas un score de 0 |
| `cases[].arms.with[].aborted`                     | Présent lorsqu'un [mock](#mock-mcp-servers) `expect:` ou `abort_when` a arrêté l'exécution, avec `server`, `tool`, et `reason`. L'exécution marque 0 et `error` reste `null`                                                                        |
| `cases[].arms.with[].skippedPaidGraders`          | `true` lorsque le plafond de coût a ignoré les évaluateurs de juge de cette exécution, donc son score n'est pas comparable                                                                                                                          |
| `costUsd`, `durationSeconds`, `claudeVersion`     | Coût estimé au prix catalogue incluant les appels de juge, secondes de temps mural, et la version de Claude Code qui a exécuté la suite                                                                                                             |

<h2 id="security">
  Ce qu'une exécution peut accéder
</h2>

`claude plugin eval` charge les skills, hooks et agents du plugin cible et exécute sa suite d'eval sur votre machine, en tant que vous. Le pointer vers un plugin est la même décision de confiance que `claude --plugin-dir`, donc n'évaluez que les plugins en lesquels vous avez confiance.

L'isolation décrite dans cette section limite ce que l'agent en test peut atteindre ; ce n'est pas une limite contre le code du plugin lui-même, et une suite qui réussit ne dit rien sur la sécurité du plugin.

<h3 id="trust-the-plugin-directory">
  Faire confiance au répertoire du plugin
</h3>

La première fois que vous exécutez `claude plugin eval` contre un répertoire, Claude Code demande « Trust this plugin directory? » avant de charger quoi que ce soit à partir de celui-ci, à moins que vous ayez déjà accepté l'invite de confiance là dans une session `claude` interactive. À l'intérieur d'un référentiel git, répondre oui fait confiance au référentiel entier, pour les sessions interactives aussi. Lorsque stdin ou stdout n'est pas un terminal, sous `--json`, ou lorsque la variable d'environnement `CI` est définie à une valeur vraie comme `true`, l'exécution ne peut pas demander et est refusée avec la sortie 1 ; passez `--trust-plugin` pour affirmer la confiance vous-même, seulement pour un plugin que vous exécuteriez sur votre propre machine. Une cible que vous nommez plutôt que de donner en tant que chemin, signifiant un plugin installé ou un plugin de répertoire de skills, ignore l'invite.

Certaines parties du plugin et de la suite s'exécutent seulement lorsque vous passez leur drapeau pour cette exécution :

* Un [`scaffold_script`](#add-setup-or-history-with-case-yaml) de cas avec `--scaffold`
* [Outils au-delà de l'ensemble en lecture seule](#grant-tools) avec `--allow-tools`
* Les [vrais serveurs MCP](#mock-mcp-servers) du plugin avec `--allow-real-servers` ou `--mocks off`

Un `allowed_tools` de cas et un frontmatter `allowed-tools` propre d'une skill ne peuvent pas élargir aucun d'eux.

Lorsque le plugin inclut des hooks que vous n'avez pas écrits, ou que vous démarrez ses vrais serveurs MCP, traitez ses scores comme consultatifs à moins que vous ne l'ayez exécuté dans un environnement isolé comme un conteneur ou un coureur CI, puisque les hooks et les serveurs s'exécutent en dehors du sandbox de l'agent et pourraient modifier les fichiers que les évaluateurs lisent.

<h3 id="how-runs-are-isolated">
  Comment les exécutions sont isolées
</h3>

Chaque exécution obtient un répertoire personnel jetable, un répertoire de travail, et une configuration Claude Code, et l'agent en test s'exécute là en tant que processus enfant `claude -p` avec seulement votre plugin chargé. Gardez ces conséquences à l'esprit lorsque vous écrivez des cas :

* **Rien de personnel ou au niveau du projet ne charge.** Vos paramètres utilisateur, hooks, fichiers `CLAUDE.md`, serveurs MCP, autres plugins installés, mémoire, et skills sont absents, et aucun `.claude/` ou `.mcp.json` au niveau du projet au-dessus du sandbox n'est lu. La plupart de votre environnement shell est également retenu ; seulement une [liste d'autorisation](#prompt-md-fields) et les variables `EVAL_*` atteignent l'exécution. Si le plugin a besoin de configuration, livrez-la dans le plugin, créez-la dans un `scaffold_script`, ou passez les variables `EVAL_*`.
* **La politique gérée peut toujours restreindre une exécution.** Les restrictions dans les [paramètres gérés](/docs/fr/managed-settings) qu'un administrateur a déployés sur la machine s'appliquent à l'intérieur d'une exécution, donc les résultats sur une machine gérée peuvent différer d'une machine non gérée par cette politique.
* **L'outil Artifact est désactivé.** Une skill qui publie un [artifact](/docs/fr/artifacts) ne peut être notée que sur ce qu'elle produit avant cette étape.
* **Les définitions de cas sont cachées à l'agent.** Une exécution ne peut pas lire le répertoire d'eval, donc Claude ne peut pas voir l'invite du cas, ses évaluateurs, ou les cas frères.
* **Pas de sandbox réseau en dehors des commandes shell.** Les commandes shell que vous accordez s'exécutent sous les règles de sandbox du réseau. Une concession `WebFetch(domain:…)` atteint ce domaine directement, et les hooks propres du plugin et tous les vrais serveurs MCP que vous démarrez peuvent atteindre n'importe quel hôte.

<h2 id="eval-suite-reference">
  Référence de la suite d'eval
</h2>

Tout ce qu'une suite d'eval peut contenir vit sous le répertoire d'eval du plugin, `evals/` à moins que vous [ayez configuré un autre](#use-a-different-eval-directory). Cet arborescence montre chaque fichier que `claude plugin eval` lit ou écrit là ; seulement `prompt.md` ou `case.yaml` est requis pour qu'un cas existe :

```text theme={null}
evals/
├── <case>/                        # one directory per case; nest under a non-case directory to group
│   ├── prompt.md                  # frontmatter: case and run fields; body: the prompt
│   ├── case.yaml                  # optional: context.* fields, or the whole case in one file
│   ├── graders/
│   │   └── <name>.md              # one grader per file; frontmatter: type and options; body: rubric
│   ├── mocks/                     # optional: mocks for this case only, same layout as below
│   └── <fixtures, scripts, transcripts referenced by case.yaml>
├── mocks/                         # optional: suite-wide MCP mocks
│   ├── <server>/
│   │   ├── <tool>.md              # one mocked tool; body: the tool result
│   │   ├── _server.md             # optional: one agent that answers several tools
│   │   ├── _tools.json            # optional: saved tools/list response for real descriptions and schemas
│   │   └── fixtures/              # files inserted with {{file:fixtures/...}}
│   └── .replay/<server>/          # adopted agent-mock recordings, answered without a model call
└── results/<timestamp>/           # written by each run; add results/ to .gitignore
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # agent-mock answers from clean runs, with ADOPT.txt
```

<h3 id="prompt-md-fields">
  Frontmatter de prompt.md
</h3>

Le frontmatter de `prompt.md` accepte ces champs. Une clé inconnue est une erreur :

| Champ                  | Par défaut                                | Objectif                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :--------------------- | :---------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`, défini pour vous                 | Version du format de cas. Les cas écrits en tant que `prompt.md` l'obtiennent automatiquement, donc vous le définissez rarement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `name`                 | Le nom du répertoire                      | Nom du cas. Les globs `--case` le correspondent et le rapport le clé                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `description`          |                                           | Pour les humains. Non utilisé au moment de l'exécution                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `tags`                 | `[]`                                      | Étiquettes pour le filtrage `--tag`. Un cas s'exécute si l'une de ses balises correspond                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `plugins`              | Le plugin englobant le plus proche        | Répertoires de plugins en test, relatifs au répertoire de cas. Définissez `plugins: ["../.."]` lorsque la détection automatique ne trouve pas votre plugin ; voir [le plugin n'a pas chargé](#the-baseline-arm-shows-no-plugin-or-delta-is-zero)                                                                                                                                                                                                                                                                                                                                                                |
| `runs`                 | `3`                                       | Exécutions par bras, 1 à 50. `--runs` le remplace                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `expected_outcome`     |                                           | Pour les humains. Non utilisé au moment de l'exécution                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `model`                | La valeur par défaut de la session enfant | Modèle pour l'agent en test. `--model` le remplace                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `max_turns`            | `10`                                      | Limite de tour, jusqu'à 200. L'atteindre est enregistré comme une erreur d'exécution et abaisse généralement le score, donc définissez-le généreusement                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `timeout_seconds`      | `300`                                     | Limite de temps mural par exécution, jusqu'à 3600                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowed_tools`        | `[]`                                      | Outils que le cas veut, comme `[Read, Glob, Grep, Skill]`. Les outils en lecture seule sont accordés lorsqu'ils sont énumérés ici ; pour tout le reste, voir [Accorder les outils](#grant-tools)                                                                                                                                                                                                                                                                                                                                                                                                                |
| `append_system_prompt` |                                           | Texte ajouté à l'invite système de la session enfant                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `env`                  | `{}`                                      | Variables d'environnement supplémentaires pour la session enfant. Les clés doivent correspondre à `EVAL_[A-Z0-9_]*` ; toute autre clé échoue l'exécution. L'exécution hérite seulement d'une liste d'autorisation de votre shell : les bases comme `PATH` et les paramètres régionaux, les paramètres de proxy et de certificat, les variables qui sélectionnent et authentifient votre fournisseur de modèle, la plupart des configurations `ANTHROPIC_*` et `CLAUDE_CODE_*`, et `EVAL_*`. Pour donner au plugin autre chose, comme un paramètre de chaîne d'outils, exportez-le en tant que variable `EVAL_*` |

<h3 id="case-yaml-fields">
  Champs case.yaml
</h3>

`case.yaml` est une alternative ou un complément à `prompt.md` : il décrit un cas en YAML et ajoute les champs qui pointent vers d'autres fichiers. Il nécessite `schema_version: "1.1"` et `name`. Les champs `prompt.md` `description`, `tags`, `plugins`, `runs`, et `expected_outcome` vont au niveau supérieur ; `model`, `max_turns`, `timeout_seconds`, `allowed_tools`, `append_system_prompt`, et `env` vont sous `execution:`. Lorsque les deux fichiers existent, le frontmatter de `prompt.md` remplace les champs `case.yaml` correspondants, le corps de `prompt.md` est l'invite, et `graders/*.md` sont ajoutés après tous les évaluateurs énumérés dans `case.yaml`.

Ces champs existent seulement dans `case.yaml` :

| Champ                     | Objectif                                                                                                                                                                                                                                                                    |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | Un script Bash dans le répertoire de cas qui s'exécute dans l'espace de travail vide avant que Claude ne démarre, pour créer des fichiers de fixture ou un référentiel git. Il s'exécute seulement lorsque vous passez [`--scaffold`](#add-setup-or-history-with-case-yaml) |
| `context.history_file`    | Une transcription `.jsonl` dans le répertoire de cas à reprendre. L'invite du cas devient le prochain tour utilisateur                                                                                                                                                      |
| `context.add_dirs`        | Répertoires à l'intérieur du répertoire de cas que Claude peut lire pendant l'exécution, accordés en lecture seule                                                                                                                                                          |
| `execution.prompt`        | L'invite, lorsque vous gardez le cas entier dans `case.yaml` et omettez `prompt.md`                                                                                                                                                                                         |
| `graders`                 | Une liste d'évaluateurs, chacun avec un `name` plus les mêmes clés qu'un fichier `graders/*.md` prend en frontmatter. Pour les évaluateurs `llm`, mettez la rubrique dans `criteria`                                                                                        |

<h3 id="grader-frontmatter">
  Frontmatter d'évaluateur
</h3>

Chaque fichier d'évaluateur sous `graders/` prend ces clés en frontmatter, plus les options pour son type. Le nom de l'évaluateur est le nom du fichier sans `.md` :

| Clé      | Par défaut | Objectif                                                                                                                                                                                                               |
| :------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`   | requis     | L'un des [types d'évaluateurs](#grader-types)                                                                                                                                                                          |
| `weight` | `1`        | Poids relatif dans le score de l'exécution. N'importe quel nombre positif                                                                                                                                              |
| `arm`    | non défini | `with-only` exclut l'évaluateur de la notation dans une [exécution à deux bras](#compare-against-a-no-plugin-baseline) ; `both` force un évaluateur que Claude Code exclurait autrement à être noté dans les deux bras |

<h4 id="what-a-grader-can-look-at">
  Ce qu'un évaluateur peut regarder
</h4>

Les évaluateurs `regex` prennent un `target` et les évaluateurs `llm` prennent un `focus`. Les deux acceptent les mêmes valeurs :

| Valeur                           | Ce que l'évaluateur voit                                                                                                                                                                                                                                                                                                                             |
| :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `last_message`                   | Le texte de la réponse finale de Claude. C'est la valeur par défaut                                                                                                                                                                                                                                                                                  |
| `trace`                          | La session entière en JSON, un message par ligne. Un évaluateur `regex` voit chaque message ; un juge `llm` voit les 12 premiers et les 12 derniers. Les guillemets et les sauts de ligne dedans sont échappés en JSON, donc une regex correspond à `\"` plutôt qu'à `"`                                                                             |
| `files`                          | La liste des chemins que Claude a créés pendant l'exécution, un par ligne. Pas leurs contenus, et pas les fichiers qu'un scaffold a créés ou que Claude a seulement modifiés                                                                                                                                                                         |
| `{ source: file, path: <path> }` | Le contenu d'un fichier dans l'espace de travail après l'exécution. Utilisez ceci pour noter ce que le plugin a produit. Un fichier PNG, JPEG, GIF, ou WebP est montré à un juge `llm` en tant qu'image. Un juge `llm` refuse les autres fichiers binaires comme `.pptx` ou PDF ; rendez-les en image ou écrivez-les en tant que texte et notez cela |
| `mock_calls`                     | Chaque appel que Claude a fait à un [outil MCP mocké](#mock-mcp-servers), avec son entrée et la réponse du mock                                                                                                                                                                                                                                      |

<h4 id="grader-types">
  Types d'évaluateurs
</h4>

Chaque type d'évaluateur ci-dessous énumère ses options et quand il réussit :

| Type          | Options                               | Réussit quand                                                                                                                                                                                                                                                          |
| :------------ | :------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`       | `pattern`, `flags`, `match`, `target` | La regex JavaScript `pattern` est trouvée dans la cible. Définissez `match: not_contains` pour exiger l'absence ou `match: "count:N"` pour exiger exactement N correspondances. Mettez l'insensibilité à la casse dans `flags: i` ; l'inline `(?i)` n'est pas supporté |
| `tool_used`   | `tool`, `input_match`, `min`, `max`   | Le nombre d'appels à `tool` dont l'entrée JSON-encodée correspond à la regex optionnelle `input_match` est entre `min`, par défaut 1, et `max`, par défaut illimité. Pour affirmer qu'un outil n'a jamais été appelé, définissez à la fois `min: 0` et `max: 0`        |
| `tool_order`  | `before`, `after`                     | Les deux outils ont été appelés et le premier appel `before` correspondant précède le premier appel `after` correspondant. Chacun est un nom d'outil ou `{ tool, input_match }`                                                                                        |
| `file_exists` | `path`, `exists`                      | Un fichier que Claude a créé correspond au glob `path`, ou aucun ne le fait avec `exists: false`. Seulement les fichiers créés pendant l'exécution comptent                                                                                                            |
| `llm`         | `criteria`, `focus`                   | Un modèle juge vote PASS sur la rubrique dans au moins deux des trois votes. Dans la disposition `.md`, le corps du fichier est les critères                                                                                                                           |
| `baseline`    | `baseline_file`, `criteria`           | Un juge trouve que l'exécution satisfait les critères au moins aussi bien que la transcription de référence à `baseline_file`, un `.jsonl` dans le répertoire de cas                                                                                                   |

<h3 id="mock-files">
  Fichiers mock
</h3>

Un fichier `<tool>.md` sous `mocks/<server>/` répond à un outil. Son corps est le résultat de l'outil, avec les substitutions `{{input.<field>}}` et `{{file:fixtures/<name>}}`. Son frontmatter accepte ces clés :

| Clé          | Par défaut | Objectif                                                                                                                                                                                                                                                                                                                     |
| :----------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`       | `fixed`    | `fixed` retourne le corps tel qu'écrit. `agent` traite le corps comme des instructions pour un petit modèle qui joue le serveur pour l'exécution et voit les appels antérieurs comme l'historique                                                                                                                            |
| `expect`     | non défini | Une carte des chemins d'entrée pointillés vers un nom de type comme `string`, `number`, `boolean`, `array`, ou `object`, une `/regex/`, un littéral, ou une liste de littéraux autorisés. Un appel qui le viole arrête l'exécution avec un score de 0 et est rapporté comme `aborted` avec le serveur, l'outil, et la raison |
| `error`      | `false`    | `fixed` seulement. Retournez le corps comme une erreur d'outil                                                                                                                                                                                                                                                               |
| `abort_when` | non défini | `agent` seulement. Prose énumérant les seules conditions sous lesquelles l'agent peut arrêter l'exécution                                                                                                                                                                                                                    |

Deux fichiers optionnels se trouvent à côté des fichiers d'outil dans le répertoire d'un serveur :

* **`_server.md`** : un seul mock `type: agent` qui répond à plusieurs outils, énumérés dans sa clé frontmatter `tools:`. Un `<tool>.md` pour le même outil a la priorité. Mettez une garde `expect:` sur le `<tool>.md` individuel, pas ici
* **`_tools.json`** : une réponse `tools/list` enregistrée du vrai serveur, pour que les outils mockés portent leurs vraies descriptions et schémas d'entrée au lieu d'un espace réservé permissif

Le répertoire `mocks/` propre d'un cas utilise la même disposition et remplace les fichiers mocks de la suite fichier par fichier.

<h2 id="troubleshooting">
  Dépannage
</h2>

Ce sont les problèmes que les auteurs rencontrent le plus souvent, indexés sur ce que vous voyez.

<h3 id="plugin-eval-is-currently-in-early-access">
  « plugin eval is currently in early access »
</h3>

Votre construction précède la disponibilité générale de la commande. Exécutez `claude update`, puis exécutez la commande à nouveau dans une session fraîche.

<h3 id="plugin-eval-is-currently-unavailable">
  « plugin eval is currently unavailable »
</h3>

Anthropic a désactivé la commande côté serveur. Rien sur votre machine ne la réactive ; exécutez `claude update` et réessayez dans une session fraîche plus tard.

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  « is not a trusted plugin directory, and this run cannot stop to ask you about it »
</h3>

C'est la première exécution contre un répertoire que Claude Code ne fait pas confiance encore, et il ne peut pas demander parce que stdin ou stdout n'est pas un terminal, vous avez passé `--json`, ou la variable d'environnement `CI` est définie sur une valeur vraie comme `true`. Exécutez `claude plugin eval <dir>` une fois dans un terminal et répondez à l'invite, ou passez `--trust-plugin` si vous faites confiance au code et à la suite du plugin. Voir [Ce qu'une exécution peut accéder](#security).

<h3 id="no-eval-cases-found">
  « No eval cases found »
</h3>

Aucun `<case>/prompt.md` ou `<case>/case.yaml` n'existe sous le répertoire d'eval en vigueur, ou vos filtres `--case` et `--tag` n'ont pas correspondu à aucun cas. Exécutez à partir de la racine du plugin, ou exécutez `claude plugin eval init` pour créer une suite.

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  Le bras de base de référence n'affiche aucun plugin, ou delta est zéro
</h3>

Si le résumé n'a pas de colonne `W/OUT`, ou le cas échoue avec « ablation requested but no plugin resolved », aucun plugin n'a été trouvé pour le cas. Ajoutez `plugins: ["../.."]` au cas, donnant le chemin du répertoire de cas au répertoire du plugin.

Si le plugin a chargé et `Δ` est toujours proche de zéro avec votre évaluateur `tool_used: Skill` échouant, c'est généralement une vraie découverte, signifiant que la `description` de la skill ne déclenche pas sur la formulation de l'invite. Ajustez la description et réexécutez la même suite.

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  « Agent type '...' not found » pour l'un des agents de votre plugin
</h3>

Par défaut, chaque cas s'exécute à la fois avec votre plugin et sans lui, et les exécutions sans lui sont la [base de référence sans plugin](#the-no-plugin-baseline). Lorsque Claude distribue l'un des agents de votre plugin dans une exécution de base de référence, l'appel de l'outil Agent échoue avec `Agent type '<plugin>:<agent-name>' not found. Available agents: ...`. La liste nomme seulement les agents qui existent sans le plugin, comme les [sous-agents intégrés](/docs/fr/sub-agents#built-in-subagents).

L'erreur est attendue, puisque `Δ` compare vos exécutions de plugin par rapport à la base de référence. Dans le résultat JSON, les exécutions de base de référence sont sous `cases[].arms.without`.

Dans les exécutions avec votre plugin chargé, un cas qui liste `Agent` dans `allowed_tools` peut distribuer l'un des agents de votre plugin par son nom avec espace de noms, comme `my-plugin:code-reviewer` pour l'agent `code-reviewer` dans un plugin nommé `my-plugin`. Pour ignorer les exécutions de base de référence, passez `--ablation none`.

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  Tout marque zéro bien que les bons fichiers aient été produits
</h3>

Vos évaluateurs ciblent `files`, la liste des chemins créés, lorsque vous aviez l'intention du contenu du fichier. Utilisez `{ source: file, path: <path> }` comme `target` ou `focus`.

Séparément, `file_exists` compte seulement les fichiers créés pendant l'exécution, donc un fichier que le scaffold a créé ou que Claude a seulement modifié est invisible pour lui ; notez son contenu, ou utilisez `tool_used` sur `Edit`.

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  Une regex sur la trace ne correspond pas au texte que je peux voir
</h3>

* **Mauvaise cible** : la `target` par défaut est `last_message`, pas la trace.
* **Échappement JSON** : lorsque vous ciblez `trace`, c'est du JSON par ligne, donc les guillemets apparaissent comme `\"`.
* **Syntaxe regex** : les regexes utilisent la syntaxe JavaScript, donc mettez `i` dans `flags` plutôt que d'écrire `(?i)`.

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  Les outils sont refusés, les outils MCP manquent, ou Bash ne s'exécutera pas
</h3>

Tout au-delà de l'ensemble en lecture seule a besoin de votre concession, comme `--allow-tools Bash Write`. Vos serveurs MCP personnels ne chargent jamais dans une exécution. Les serveurs propres du plugin ne démarrent pas à moins que vous [optiez pour](#mock-mcp-servers), et leurs outils ont alors aussi besoin d'une concession `--allow-tools "mcp__plugin_<plugin>_<server>__*"` ; un outil mocké n'en a besoin d'aucune.

<h3 id="the-run-exits-1-but-the-results-look-fine">
  L'exécution quitte 1 mais les résultats semblent bien
</h3>

La `--threshold` par défaut est 1.0, donc la commande quitte 1 lorsqu'un cas marque en dessous de parfait. Définissez un seuil qui correspond à votre barre. La sortie 1 couvre également un fichier de cas qui n'a pas pu charger, qui est rapporté sur stderr au-dessus du tableau.

<h3 id="json-output-path-must-end-in-json">
  « --json output path must end in .json »
</h3>

Vous avez mis la cible après `--json`, donc elle a été lue comme le chemin de sortie. Mettez la cible en premier, comme dans `claude plugin eval . --json`, ou donnez à `--json` un chemin `.json` explicite.

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  Un évaluateur affiche passed: false sous une exécution qui a marqué 1.0
</h3>

Cet évaluateur est exclu du score par conception dans une exécution à deux bras, et son champ `scored` est `false`. Voir [Comparer avec une base de référence sans plugin](#compare-against-a-no-plugin-baseline).

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  Les exécutions échouent avec une erreur de limite d'utilisation ou de limite de débit à mi-chemin
</h3>

Si votre compte atteint la limite d'utilisation de son plan ou une limite de débit API pendant qu'une suite s'exécute, chaque exécution ultérieure se termine avec cette erreur, est notée sur ce qu'elle a produit, et marque généralement 0. La suite se termine toujours et n'est pas marquée `partial`, donc le résultat peut ressembler à une régression. Vérifiez la colonne `NOTES` ou `cases[].arms.with[].error` dans le JSON pour le message de limite avant de faire confiance aux scores, puis réexécutez après que la limite se réinitialise, avec `--runs 1` ou un filtre `--case` si vous avez besoin de rester en dessous.

<h3 id="runs-time-out-or-hit-the-turn-cap">
  Les exécutions expirent ou atteignent la limite de tour
</h3>

Les valeurs par défaut sont 10 tours et 300 secondes. Augmentez `max_turns` et `timeout_seconds` dans le cas pour les tâches qui en ont besoin, et utilisez `--max-cost-usd` comme plafond de coût plutôt que des limites serrées par exécution.

<h2 id="see-also">
  Voir aussi
</h2>

* [Créer un plugin](/docs/fr/plugins/create) : construisez le plugin que vous testez, et chargez-le avec `--plugin-dir` pendant le développement
* [Référence des commandes de plugin](/docs/fr/plugins/cli-reference#plugin-eval) : les entrées de commande `plugin eval` et `plugin eval init`. La clé [`experimental.evals`](/docs/fr/plugins/manifest-reference#fields) du manifeste se trouve dans la référence du manifeste
* [Skills](/docs/fr/skills) : comment la description d'une skill décide quand Claude l'invoque, ce qu'un cas qui vérifie si la skill se déclenche mesure
* [Sandboxing](/docs/fr/sandboxing) : le sandbox au niveau du système d'exploitation qui s'applique lorsque vous accordez Bash à une exécution
* [Publier un plugin](/docs/fr/plugins/publish) : publiez le plugin une fois que sa suite réussit
* [Mesurer le coût et l'utilisation du plugin](/docs/fr/plugins/measure) : ce que le plugin ajoute au contexte de chaque session et si les gens l'utilisent toujours
