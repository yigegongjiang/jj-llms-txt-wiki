> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrer des sous-agents à grande échelle avec des workflows dynamiques

> Les workflows dynamiques orchestrent de nombreux sous-agents à partir d'un script que Claude écrit et que vous pouvez relancer. Utilisez-les pour les audits de base de code, les migrations importantes et la recherche avec vérification croisée.

<Note>
  Les workflows dynamiques sont disponibles sur tous les plans payants, avec accès à l'API Anthropic, et sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry. Sur Pro, activez-les à partir de la ligne Dynamic workflows dans `/config`.
</Note>

Un workflow dynamique est un script JavaScript qui orchestre de nombreux [sous-agents](/docs/fr/sub-agents) à la fois. Claude écrit le script pour la tâche que vous décrivez, et un runtime l'exécute en arrière-plan tandis que votre session reste réactive.

Utilisez un workflow quand une tâche nécessite plus d'agents qu'une seule conversation ne peut en coordonner, ou quand vous voulez que l'orchestration soit codifiée sous forme de script que vous pouvez lire et relancer. Les exemples incluent un balayage de bugs à l'échelle de la base de code, une migration de 500 fichiers, une question de recherche qui nécessite une vérification croisée des sources les unes par rapport aux autres, et un plan difficile qui vaut la peine d'être rédigé sous plusieurs angles indépendants avant de vous engager sur l'un d'eux.

<h2 id="when-to-use-a-workflow">
  Quand utiliser un workflow
</h2>

Les [sous-agents](/docs/fr/sub-agents), les [skills](/docs/fr/skills), les [équipes d'agents](/docs/fr/agent-teams) et les workflows peuvent tous exécuter une tâche multi-étapes. La différence réside dans qui détient le plan :

|                                        | Sous-agents                        | Skills                           | Équipes d'agents                                        | Workflows                                           |
| :------------------------------------- | :--------------------------------- | :------------------------------- | :------------------------------------------------------ | :-------------------------------------------------- |
| Ce que c'est                           | Un worker Claude génère            | Des instructions que Claude suit | Un agent principal supervisant des sessions entre pairs | Un script que le runtime exécute                    |
| Qui décide ce qui s'exécute ensuite    | Claude, tour par tour              | Claude, en suivant le prompt     | L'agent principal, tour par tour                        | Le script                                           |
| Où vivent les résultats intermédiaires | La fenêtre de contexte de Claude   | La fenêtre de contexte de Claude | Une liste de tâches partagée                            | Les variables du script                             |
| Ce qui est répétable                   | La définition du worker            | Les instructions                 | La définition de l'équipe                               | L'orchestration elle-même                           |
| Échelle                                | Quelques tâches déléguées par tour | Identique aux sous-agents        | Une poignée de pairs s'exécutant longtemps              | Des dizaines à des centaines d'agents par exécution |
| Interruption                           | Redémarre le tour                  | Redémarre le tour                | Les coéquipiers continuent de s'exécuter                | Reprendre dans la même session                      |

Un workflow déplace le plan dans le code. Avec les sous-agents, les skills et les équipes d'agents, Claude est l'orchestrateur : il décide tour par tour ce qu'il faut générer ou assigner ensuite, et chaque résultat atterrit dans une fenêtre de contexte. Un script de workflow détient la boucle, la ramification et les résultats intermédiaires eux-mêmes, donc le contexte de Claude ne contient que la réponse finale.

Déplacer le plan dans le code permet également à un workflow d'appliquer un modèle de qualité répétable, pas seulement d'exécuter plus d'agents : il peut avoir des agents indépendants qui examinent adversarialement les conclusions les uns des autres avant qu'elles ne soient rapportées, ou rédiger un plan sous plusieurs angles et les peser les uns par rapport aux autres, afin que vous obteniez un résultat plus fiable qu'une seule passe.

<h2 id="run-a-bundled-workflow">
  Exécuter un workflow groupé
</h2>

Le moyen le plus rapide de voir un workflow en action est d'exécuter `/deep-research`, le [workflow intégré](#bundled-workflows) que Claude Code inclut pour enquêter sur une question à travers de nombreuses sources. Vous verrez les agents travailler à travers un ensemble de phases en arrière-plan tandis que votre session reste libre, et vous obtiendrez un rapport à la fin au lieu d'une transcription tour par tour.

<Steps>
  <Step title="Exécuter le workflow">
    Exécutez `/deep-research` avec une question que vous souhaitez enquêter. Il déploie des recherches web selon plusieurs angles, récupère et vérifie les sources qu'il trouve, et synthétise un rapport cité.

    ```text wrap theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Autoriser les workflows">
    Claude Code demande s'il faut autoriser le workflow. Sélectionnez **Oui** pour continuer. L'invite exacte dépend de votre mode de permission. Voir [Approuver le plan avant son exécution](#approve-the-plan-before-it-runs) pour les options par mode.
  </Step>

  <Step title="Surveiller la progression">
    L'exécution démarre en arrière-plan. Exécutez `/workflows`, utilisez les touches fléchées pour sélectionner l'exécution, et appuyez sur Entrée pour ouvrir sa vue de progression :

    ```text wrap theme={null}
    /workflows
    ```

    La vue affiche chaque phase avec son nombre d'agents, le total des tokens et le temps écoulé. Explorez n'importe quelle phase pour voir ses agents et ce que chacun a trouvé. Voir [Surveiller l'exécution](#watch-the-run) pour l'ensemble complet des contrôles.

    Vous pouvez également surveiller à partir du panneau des tâches sous la zone de saisie : un résumé de progression d'une ligne apparaît là pendant que l'exécution est en cours. Appuyez sur la flèche vers le bas pour le mettre au point, puis sur Entrée pour le développer.
  </Step>

  <Step title="Lire le rapport">
    Lorsque l'exécution se termine, le rapport arrive dans votre session. Il cite les sources dont provient chaque affirmation, les affirmations qui n'ont pas survécu à la vérification croisée étant déjà filtrées.

    Lorsque les agents vérificateurs ne peuvent pas vérifier une affirmation, par exemple après une limite de débit ou une erreur API, le rapport liste cette affirmation comme non vérifiée au lieu de la compter comme réfutée.
  </Step>
</Steps>

Pour exécuter un workflow pour votre propre tâche, [demandez à Claude d'en écrire un](#have-claude-write-a-workflow), et une fois qu'une exécution fait ce que vous vouliez, vous pouvez [l'enregistrer](#save-the-workflow-for-reuse) comme commande de votre choix.

<h3 id="bundled-workflows">
  Workflows groupés
</h3>

Claude Code inclut `/deep-research` comme workflow intégré :

| Commande                    | Ce qu'elle fait                                                                                                                                                                                                                                                                                                                                           |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | Déploie des recherches web sur une question selon plusieurs angles, récupère et vérifie les sources qu'il trouve, vote sur chaque affirmation, et retourne un rapport cité avec les affirmations qui n'ont pas survécu à la vérification croisée filtrées. Nécessite que l'outil [WebSearch](/docs/fr/tools-reference#websearch-tool-behavior) soit disponible |

`/deep-research` s'exécute uniquement lorsque vous l'invoquez.

[Les workflows que vous enregistrez](#save-the-workflow-for-reuse) vous-même deviennent des commandes de la même manière et apparaissent dans l'autocomplétion `/` aux côtés des workflows groupés.

<h3 id="watch-the-run">
  Surveiller l'exécution
</h3>

Les workflows s'exécutent en arrière-plan, de sorte que la session reste réactive pendant que les agents travaillent. Exécutez `/workflows` à tout moment pour lister les workflows en cours d'exécution et terminés, puis sélectionnez-en un pour ouvrir sa vue de progression.

La vue de progression affiche chaque phase avec ses nombres d'agents, ses totaux de tokens et son temps écoulé. Le pied de page liste la clé pour chaque action :

| Clé             | Action                                                                                                                                                  |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓`       | Sélectionner une phase ou un agent                                                                                                                      |
| `Entrée` ou `→` | Explorez la phase sélectionnée, puis le détail d'un agent. Dans le détail, `Entrée` le développe ou le réduit                                           |
| `Échap` ou `←`  | Reculer d'un niveau. Dans les versions v2.1.203 à v2.1.205, `←` n'a pas permis de reculer d'une phase ou d'un agent ; utilisez `Échap` sur ces versions |
| `j` / `k`       | Faire défiler dans le détail de l'agent lorsqu'il déborde                                                                                               |
| `f`             | Filtrer la liste des agents dans la phase sélectionnée par statut. Appuyez à nouveau pour parcourir                                                     |
| `p`             | Mettre en pause ou reprendre l'exécution                                                                                                                |
| `x`             | Arrêter l'agent sélectionné, ou arrêter l'ensemble du workflow lorsque le focus est sur l'exécution                                                     |
| `r`             | Redémarrer l'agent en cours d'exécution sélectionné                                                                                                     |
| `s`             | [Enregistrer](#save-the-workflow-for-reuse) le script de l'exécution comme commande                                                                     |

Le détail de l'agent liste l'invite de l'agent, ses appels d'outils récents et son résultat. Chaque appel affiche son état, par exemple toujours en cours d'exécution ou échoué. Lorsque l'agent maintient sa propre liste de tâches, le détail l'affiche également, avec le statut de chaque tâche.

Appuyez sur `Entrée` pour développer le détail. L'invite et le résultat s'affichent alors en intégralité, et chaque appel listé affiche son entrée et le début de son résultat.

<h2 id="have-claude-write-a-workflow">
  Faire écrire un workflow par Claude
</h2>

Vous pouvez faire écrire un workflow par Claude pour votre tâche de deux façons :

* [Demander un workflow dans votre prompt](#ask-for-a-workflow-in-your-prompt) avec vos propres mots ou en incluant le mot clé `ultracode`, et Claude en écrit un pour la tâche.
* [Laisser Claude décider avec ultracode](#let-claude-decide-with-ultracode) : définissez `/effort ultracode` et Claude planifie un workflow pour chaque tâche substantielle de la session.

Vous pouvez également exécuter une commande de workflow qui existe déjà : un [workflow groupé](#bundled-workflows) comme `/deep-research`, ou un que vous avez [enregistré](#save-the-workflow-for-reuse).

<h3 id="ask-for-a-workflow-in-your-prompt">
  Demander un workflow dans votre prompt
</h3>

Pour exécuter une seule tâche en tant que workflow sans modifier le niveau d'effort de la session, incluez le mot clé `ultracode` dans votre prompt. Demander avec vos propres mots, par exemple « utiliser un workflow » ou « exécuter un workflow », fonctionne également : Claude traite une demande directe comme le même opt-in.

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code met en évidence le mot clé dans votre saisie et Claude écrit un script de workflow pour la tâche au lieu de la traiter tour par tour. Le mot clé choisit uniquement la façon dont Claude structure le travail : les appels d'outils des agents reçoivent les mêmes vérifications de permission et [sandboxing](/docs/fr/sandboxing) que tout autre appel d'outil de la session.

Si l'exécution fait ce que vous vouliez, vous pouvez [l'enregistrer comme commande](#save-the-workflow-for-reuse) après. Si vous avez déjà un orchestrateur construit d'une autre façon, comme un dossier de prompts de sous-agents ou une compétence qui distribue le travail, vous pouvez pointer Claude vers celui-ci et demander un workflow qui fait la même chose.

<h4 id="dismiss-or-turn-off-the-keyword">
  Ignorer ou désactiver le mot clé
</h4>

Si vous ne vouliez pas démarrer un workflow, appuyez sur `Option+W` sur macOS ou `Alt+W` sur Windows et Linux pour ignorer la mise en évidence pour ce prompt, ou appuyez sur retour arrière tandis que le curseur se trouve juste après le mot clé en évidence. Pour empêcher le mot clé de déclencher quoi que ce soit, désactivez le déclencheur de mot clé Ultracode dans `/config`.

<h4 id="where-the-keyword-works">
  Où le mot clé fonctionne
</h4>

Le mot clé est un opt-in uniquement dans un prompt que vous tapez vous-même : à l'invite interactive, dans un panneau d'extension IDE, dans un client [Remote Control](/docs/fr/remote-control), ou dans une application Agent SDK qui marque l'[`origin`](/docs/fr/agent-sdk/typescript#sdkmessageorigin) de votre saisie clavier comme `{ kind: "human" }`. Il ne démarre pas un workflow quand il atteint la session d'une autre façon :

* un prompt passé avec `-p`
* un prompt qu'une application Agent SDK envoie sans le marquer comme saisie humaine
* un prompt de tâche planifiée
* une charge utile webhook ou un commentaire de pull request relayé dans la conversation

<Note>
  Avant la v2.1.210, le mot clé démarrait un workflow à partir de n'importe lequel de ces itinéraires également, y compris une charge utile webhook ou un commentaire de pull request relayé dans la conversation.
</Note>

<h3 id="let-claude-decide-with-ultracode">
  Laisser Claude décider avec ultracode
</h3>

Ultracode est un paramètre Claude Code qui combine l'[effort de raisonnement](/docs/fr/model-config#adjust-effort-level) `xhigh` avec l'orchestration automatique des workflows. Avec lui activé, Claude planifie un workflow pour chaque tâche substantielle au lieu d'attendre que vous le demandiez.

```text wrap theme={null}
/effort ultracode
```

Pour démarrer une session avec ultracode déjà activé, lancez avec `claude --effort ultracode`. Nécessite Claude Code v2.1.203 ou ultérieur.

Pour l'activer pendant que vous choisissez un modèle, déplacez le curseur d'effort du sélecteur `/model` vers `ultracode` avec les touches fléchées. [Ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level) énumère les itinéraires qui activent ultracode.

Avec ultracode activé, Claude décide quand une tâche justifie un workflow. Une seule demande peut se transformer en plusieurs workflows d'affilée : un pour comprendre le code, un pour faire le changement, et un pour le vérifier. Cela s'applique à chaque tâche de la session, donc chaque demande utilise plus de tokens et prend plus de temps qu'aux niveaux d'effort inférieurs.

`/effort ultracode` dure pour la session actuelle ; pour que chaque session commence avec lui, définissez le paramètre [`ultracode`](/docs/fr/settings-reference#ultracode). Revenez avec `/effort high` quand vous retournez au travail de routine. Le menu `/effort` l'offre uniquement [quand ultracode est disponible](/docs/fr/model-config#when-ultracode-is-available).

<h3 id="approve-the-plan-before-it-runs">
  Approuver le plan avant qu'il s'exécute
</h3>

Dans le CLI, l'invite par exécution affiche les phases planifiées et ces options :

* **Oui, l'exécuter** : démarrer l'exécution
* **Oui, et ne pas demander à nouveau pour `<name>` dans `<path>`** : démarrer, et ignorer cette invite pour ce workflow dans ce projet à partir de maintenant. Claude Code offre cette option quand vous exécutez un workflow groupé, enregistré, ou plugin par nom, pas pour un script que Claude a écrit pour la tâche actuelle.
* **Afficher le script brut** : lire le script avant de décider
* **Non** : annuler

`Ctrl+G` ouvre le script dans votre éditeur. `Tab` vous permet d'ajuster le prompt avant le démarrage de l'exécution.

Que vous voyiez cette invite dépend de votre [mode de permission](/docs/fr/permission-modes) :

| Mode de permission                 | Quand vous êtes invité                                                                                                                                                                                      |
| :--------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Auto                               | Première exécution uniquement. Tout **Oui** enregistre le consentement dans vos paramètres utilisateur, et les exécutions ultérieures commencent sans invite. Ignoré entièrement quand ultracode est activé |
| Manuel, accepter les modifications | À chaque exécution, sauf si vous avez sélectionné **Oui, et ne pas demander à nouveau** pour ce workflow dans ce projet                                                                                     |
| Contourner les permissions         | Claude Code ne vous invite pas. L'exécution commence immédiatement                                                                                                                                          |
| `claude -p`, Agent SDK             | Claude Code ne vous invite pas                                                                                                                                                                              |

Dans `claude -p` et l'Agent SDK, Claude Code ne montre jamais cette invite. Il exécute l'appel d'outil Workflow à travers la même [évaluation de permission](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) que le reste de la session, donc les règles de refus, les règles de demande, et le mode `dontAsk` s'appliquent au lancement comme ils s'appliquent à chaque appel d'outil. Pour laisser le workflow démarrer dans ces exécutions, utilisez l'un de ceux-ci :

* **Règle de permission** : `Workflow` dans vos règles d'autorisation approuve chaque workflow, et `Workflow(<name>)` approuve un workflow enregistré par nom.
* **Mode de permission auto** : le [classificateur](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) examine l'appel et peut l'approuver.
* **Mode de permission contourner les permissions** : Claude Code approuve l'appel.
* **Un hook `PreToolUse`** : un [hook](/docs/fr/hooks#pretooluse) qui retourne `allow` pour l'appel l'approuve.
* **Votre hôte** : un [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags) l'approuve, ou, avec l'Agent SDK, un callback [`canUseTool`](/docs/fr/agent-sdk/permissions) ou un hook [`PermissionRequest`](/docs/fr/hooks#permissionrequest) l'approuve.

Dans l'application Desktop, une carte d'approbation affiche le nom du workflow, la liste des phases, et une mise en garde sur l'utilisation des tokens, avec les actions **Une fois**, **Toujours**, et **Refuser**. La vue de progression apparaît dans le volet des tâches en arrière-plan.

Les sous-agents que le workflow génère utilisent vos [règles de permission](/docs/fr/settings-reference#permission-settings), et Claude Code choisit leur mode de permission selon les règles sous [quel mode de permission un sous-agent s'exécute dans](/docs/fr/sub-agents#permission-modes). Pour éviter les invites lors d'une exécution longue, ajoutez les outils dont les agents ont besoin à vos règles d'autorisation avant de commencer.

<h3 id="save-the-workflow-for-reuse">
  Enregistrer le workflow pour réutilisation
</h3>

Quand Claude écrit un workflow pour une tâche que vous répéterez, vous pouvez enregistrer le script de cette exécution comme commande. Un processus comme une revue que vous exécutez sur chaque branche exécute ensuite la même orchestration à chaque fois.

Exécutez `/workflows`, sélectionnez l'exécution que vous voulez conserver, et appuyez sur `s`. Dans la boîte de dialogue d'enregistrement, Tab bascule entre les deux emplacements d'enregistrement :

* `.claude/workflows/` dans votre projet : partagé avec tous ceux qui clonent le repo
* `~/.claude/workflows/` dans votre répertoire personnel : disponible dans chaque projet, visible uniquement pour vous. Si vous définissez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars), cet emplacement est le répertoire `workflows/` sous ce chemin.

La boîte de dialogue d'enregistrement affiche le chemin résolu pour l'emplacement personnel.

Appuyez sur Entrée pour enregistrer. Le workflow s'exécute comme `/<name>` dans les futures sessions à partir de l'un ou l'autre emplacement.

Claude Code vérifie l'emplacement d'enregistrement pour les liens symboliques avant d'écrire, et affiche une erreur au lieu d'écrire à travers un. Ce qu'il vérifie dépend de l'endroit où vous enregistrez :

* Emplacement du projet : Claude Code refuse si `.claude`, `.claude/workflows`, ou le fichier cible est un lien symbolique.
* Emplacement personnel : Claude Code refuse uniquement si le fichier cible lui-même est un lien symbolique, donc un répertoire `~/.claude` géré par un outil dotfiles fonctionne toujours.

Avant la v2.1.216, Claude Code suivait le lien, ce qui pouvait placer le fichier en dehors de l'emplacement que vous aviez choisi.

Dans un monorepo avec plusieurs répertoires `.claude/`, vous pouvez conserver les workflows aux côtés du package auquel ils s'appliquent. L'enregistrement à l'emplacement du projet écrit dans le répertoire `.claude/workflows/` le plus proche qui existe déjà entre votre répertoire de travail et la racine du référentiel, ou à la racine du référentiel s'il n'en existe pas encore. Les workflows de projet se chargent également à partir de chaque `.claude/workflows/` le long de ce chemin, et quand plus d'un définit le même nom, Claude Code exécute celui le plus proche du répertoire de travail.

Si un workflow de projet et un workflow personnel partagent un nom, celui du projet s'exécute.

<h3 id="distribute-a-workflow-in-a-plugin">
  Distribuer un workflow dans un plugin
</h3>

Pour partager un workflow entre les équipes ou les référentiels, incluez-le dans un [plugin](/docs/fr/plugins/overview). Placez le script dans un répertoire `workflows/` à la racine du plugin, ou pointez vers un emplacement différent avec le [champ de manifeste `workflows`](/docs/fr/plugins/manifest-reference#fields).

Les workflows de plugin sont espacés de noms par le nom du plugin. Un plugin appelé `acme-tools` contenant un script dont `meta.name` est `release-audit` s'exécute comme `/acme-tools:release-audit`.

<h3 id="pass-input-to-a-saved-workflow">
  Passer une entrée à un workflow enregistré
</h3>

Un workflow enregistré peut accepter une entrée via le paramètre `args`. Le script la lit comme une variable globale nommée `args`. Utilisez ceci pour fournir une question de recherche, une liste de chemins cibles, ou un objet de configuration au moment de l'invocation au lieu de modifier le script pour chaque exécution.

L'invite suivante exécute un workflow enregistré avec une liste de numéros de problème :

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
```

Claude passe la liste en tant que données structurées, donc le script peut appeler les méthodes de tableau et d'objet sur `args` directement sans l'analyser d'abord. Si `args` est omis, la variable globale est `undefined` à l'intérieur du script.

<h2 id="example-workflow-prompts">
  Exemples de prompts de workflow
</h2>

Un workflow convient mieux quand la tâche est plus grande qu'un agent ne peut la tenir en contexte, ou quand la même étape doit s'exécuter sur de nombreux éléments. Les prompts ci-dessous montrent des formes courantes. Chacun demande à Claude d'écrire et d'exécuter un workflow pour cette tâche ; vous n'écrivez pas le script vous-même.

<h3 id="audit-many-files-for-the-same-issue">
  Auditer de nombreux fichiers pour le même problème
</h3>

Distribuez un agent par fichier, puis collectez et vérifiez les conclusions.

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Continuer à corriger jusqu'à ce qu'une vérification réussisse
</h3>

Exécutez un vérificateur, corrigez ce qui a échoué, et répétez jusqu'à ce qu'il réussisse ou cesse de faire des progrès.

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrer de nombreux fichiers en parallèle
</h3>

Découvrez les fichiers à migrer, transformez chacun dans une copie isolée afin que les modifications ne se chevauchent pas, et vérifiez chaque résultat.

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Examiner chaque fichier modifié et écrire un résumé
</h3>

Exécutez un examinateur par fichier, puis remettez toutes les conclusions à un agent qui les classe et les déduplique.

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Rechercher un sujet à travers de nombreuses sources
</h3>

Distribuez les lecteurs sur les journaux des modifications, les problèmes et la documentation, puis synthétisez. Le workflow groupé `/deep-research` fait cela ; vous pouvez également décrire une version plus étroite.

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Trouver des problèmes jusqu'à ce que la liste cesse de croître
</h3>

Continuez à chercher par rounds et arrêtez-vous quand les nouveaux rounds ne trouvent rien de nouveau.

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  À quoi ressemble le script enregistré
</h3>

Quand vous [enregistrez un workflow](#save-the-workflow-for-reuse), le fichier dans `.claude/workflows/` contient un bloc `meta` suivi d'un corps de script qui orchestre les sous-agents. Vous n'avez généralement pas besoin de l'éditer, mais voici la forme d'un petit pour que vous puissiez reconnaître ce que Claude a généré :

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

Le corps est du JavaScript simple avec `await` au niveau supérieur. `agent()` génère un sous-agent, `pipeline()` en exécute un par élément dans une liste, et `parallel()` exécute un ensemble de tâches d'agent en même temps et attend que toutes se terminent.

Un appel `agent()` se résout en `null` si vous l'arrêtez en cours d'exécution ou s'il rencontre une erreur API irrécupérable. `pipeline()` conserve chaque `null` dans le tableau des résultats, c'est pourquoi l'exemple se termine par `.filter(Boolean)` pour supprimer ces entrées.

En [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), le prompt que votre script transmet à `agent()` ne compte pas comme une demande de votre part quand le classificateur examine les actions de ce sous-agent, car Claude Code le marque comme du texte que le script a calculé.

Si vous passez un `schema` sur un appel `agent()`, ce sous-agent retourne du JSON correspondant à la forme au lieu de prose. Claude Code vérifie le schéma avant de démarrer le sous-agent : quand il peut prouver que le schéma se contredit lui-même, l'appel échoue avec une erreur nommant la contradiction, et le sous-agent ne démarre jamais. Une contradiction qu'il peut prouver est une clé `required` que `additionalProperties: false` exclut.

Si la sortie du sous-agent échoue toujours la validation après cinq tentatives, l'appel échoue avec une erreur qui inclut l'échec de validation le plus récent. Pour modifier le nombre de tentatives, définissez [`MAX_STRUCTURED_OUTPUT_RETRIES`](/docs/fr/env-vars).

<h3 id="edit-a-saved-script">
  Éditer un script enregistré
</h3>

Pour modifier un [workflow que vous avez enregistré](#save-the-workflow-for-reuse), éditez son fichier `.js` ou demandez à Claude de faire le changement. Avant d'éditer ou de demander, exécutez la [compétence groupée](/docs/fr/skills#bundled-skills) `/workflow-authoring` pour charger la référence de rédaction de script sur laquelle Claude travaille. La compétence nécessite Claude Code v2.1.248 ou ultérieur.

Pour exécuter la version éditée dans la session actuelle, exécutez [`/reload-skills`](/docs/fr/commands#all-commands) pour relire les répertoires de workflow, puis exécutez `/<name>` à nouveau.

Claude Code applique ces règles à chaque partie du fichier quand il charge et exécute le script :

* **Bloc `meta`** : gardez `export const meta` comme première instruction, et gardez-le un objet littéral simple avec un `name` et une `description`. S'il contient autre chose que des valeurs littérales, comme une variable, un appel de fonction ou une propagation, Claude Code supprime `/<name>` de l'autocomplétion `/`.
* **Corps** : en plus de `agent()`, `pipeline()` et `parallel()`, vous pouvez appeler `phase()` pour regrouper les agents qui suivent sous un titre dans la vue de progression, appeler `log()` pour afficher un message au-dessus des phases, et lire le global [`args`](#pass-input-to-a-saved-workflow). Si le corps a une erreur de syntaxe, Claude Code la signale quand vous exécutez le workflow.
* **`phases`** : si vous les listez dans `meta`, donnez à chaque entrée exactement le titre que vous passez à `phase()`. Un titre `phase()` sans entrée obtient son propre groupe de progression.
* **Horodatages et aléatoire** : Claude Code fait en sorte que `Date.now()`, `Math.random()` et un `new Date()` sans argument lèvent une exception à l'intérieur du script, afin qu'une [exécution relancée](#resume-after-a-pause) répète les mêmes appels `agent()`. Passez plutôt un horodatage via `args`.

Vous pouvez également éditer [le script d'une seule exécution](#how-a-workflow-runs) plutôt que la copie enregistrée. [Reprendre après une pause](#resume-after-a-pause) couvre les agents qui s'exécutent à nouveau quand vous relancez un script édité. Pour les entrées de l'outil Workflow, consultez son entrée dans la [référence Agent SDK](/docs/fr/agent-sdk/typescript#workflow).

<h2 id="how-a-workflow-runs">
  Comment un workflow s'exécute
</h2>

Le runtime du workflow exécute le script dans un environnement isolé, séparé de votre conversation. Les résultats intermédiaires restent dans les variables du script au lieu d'atterrir dans le contexte de Claude.

Chaque exécution écrit son script dans un fichier sous le répertoire de votre session dans `~/.claude/projects/`. Claude reçoit le chemin au démarrage de l'exécution, vous pouvez donc le demander. Vous pouvez ouvrir ce fichier pour lire l'orchestration que Claude a écrite, la comparer avec le script d'une exécution précédente, ou l'éditer et demander à Claude de relancer à partir de la version éditée.

Claude ne peut démarrer un workflow que à partir d'un fichier de script que la session est déjà autorisée à lire. Pour exécuter un script conservé en dehors de votre répertoire de travail, ajoutez d'abord son répertoire avec [`/add-dir`](/docs/fr/permissions#working-directories) ou une [règle d'autorisation Read](/docs/fr/permissions#read-and-edit).

Le runtime suit le résultat de chaque agent au fur et à mesure que l'exécution progresse, ce qui rend une exécution [reprendre](#resume-after-a-pause) possible dans la même session.

<h3 id="prompt-caching-in-a-fan-out">
  Prompt caching dans un fan-out
</h3>

Les agents dans la même exécution peuvent lire le [prompt cache](/docs/fr/prompt-caching#subagents-and-the-cache) les uns des autres. Deux agents qui s'exécutent avec le même modèle, le même niveau d'effort, le même type d'agent, les mêmes outils, le même schéma de sortie et le même répertoire de travail construisent le même préfixe d'outils et de système-prompt, donc un agent qui démarre après que la réponse d'un frère correspondant a commencé lit le cache de ce frère à sa première requête.

Les requêtes d'un agent de workflow se situent en dehors du [bucket TTL du cache](/docs/fr/prompt-caching#which-ttl-each-request-gets) de la conversation principale, donc son cache se maintient pendant cinq minutes par défaut, y compris sur un abonnement Claude. Pour le conserver pendant une heure, définissez [`subagentPromptCacheTtl`](/docs/fr/settings-reference#subagentpromptcachettl) sur `1h`. L'API facture les écritures de cache d'une heure à un taux plus élevé.

Lorsqu'un fan-out démarre plusieurs agents correspondants à la fois, Claude Code maintient tous les agents sauf le premier jusqu'à ce que la réponse du premier agent commence, puis libère les agents maintenus ensemble afin que leurs premières requêtes lisent le préfixe partagé au lieu que chacun le traite sans cache. Claude Code plafonne la retenue à [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](/docs/fr/env-vars) millisecondes, `5000` par défaut. Définissez-le sur `0` pour désactiver la retenue.

<h3 id="behavior-and-limits">
  Comportement et limites
</h3>

Le runtime applique les contraintes suivantes :

| Contrainte                                                                                                                                                                                                                                                                                                                                                       | Pourquoi                                                                                                                                                                                                                                          |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Pas d'entrée utilisateur en cours d'exécution                                                                                                                                                                                                                                                                                                                    | Une exécution se met en pause uniquement pour les invites de permission d'agent et une [attente de limite d'utilisation](#when-a-run-hits-your-usage-limit). Pour l'approbation entre les étapes, exécutez chaque étape comme son propre workflow |
| Pas d'accès direct au système de fichiers ou au shell à partir du workflow lui-même                                                                                                                                                                                                                                                                              | Les agents lisent, écrivent et exécutent des commandes. Le script coordonne les agents                                                                                                                                                            |
| Pas de chargement de module : un script qui contient `import()` échoue avant le démarrage de l'exécution                                                                                                                                                                                                                                                         | Le corps du script est du JavaScript pur. Mettez le travail qui nécessite une bibliothèque dans la tâche d'un agent                                                                                                                               |
| Jusqu'à 16 agents concurrents par défaut, moins lorsque Claude Code dispose de moins de CPU disponibles, y compris à l'intérieur d'un conteneur limité en CPU. Pour modifier la limite, définissez [`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](/docs/fr/env-vars#variables) sur une valeur de 1 à 256, ce qui nécessite Claude Code v2.1.269 ou version ultérieure | Limite l'utilisation des ressources locales                                                                                                                                                                                                       |
| Dans un fan-out, les agents qui partagent le préfixe du prompt-cache du premier agent démarrent jusqu'à 5 secondes après par défaut                                                                                                                                                                                                                              | Tous sauf le premier lisent le [préfixe que le premier agent a mis en cache](#prompt-caching-in-a-fan-out) au lieu que chacun le traite sans cache                                                                                                |
| Jusqu'à 4 096 éléments dans un seul appel `parallel()` ou `pipeline()` : le runtime rejette une liste plus longue avec une erreur                                                                                                                                                                                                                                | Un plafond silencieux supprimerait une partie de la charge de travail sans le dire au script                                                                                                                                                      |
| 1 000 agents au total par exécution                                                                                                                                                                                                                                                                                                                              | Empêche les boucles incontrôlées                                                                                                                                                                                                                  |

<h2 id="manage-runs">
  Gérer les exécutions
</h2>

Une fois qu'une exécution commence, vous la gérez à partir de la vue `/workflows`, ou en agrandissant sa ligne de progression dans le panneau des tâches sous la zone de saisie.

Quand vous arrêtez une exécution, elle reste dans le panneau des tâches tant que l'un des processus de ses agents est toujours en cours d'exécution. Si vous l'arrêtez à nouveau, Claude Code renvoie à nouveau un signal à ces processus.

<h3 id="resume-after-a-pause">
  Reprendre après une pause
</h3>

Reprenez une exécution en pause à partir de `/workflows` en la sélectionnant et en appuyant sur `p`. Pour une exécution que vous avez arrêtée, demandez à Claude de relancer le workflow avec le même script. Si les agents de l'exécution arrêtée n'ont pas encore quitté, Claude Code refuse le relancement jusqu'à ce qu'ils le fassent, afin qu'une deuxième copie de ces agents ne puisse pas s'exécuter à côté d'eux.

Claude Code rejoue l'exécution dans l'ordre où les agents ont commencé, et chaque agent retourne soit son résultat sauvegardé, soit s'exécute à nouveau :

* **Terminé** : retourne son résultat sauvegardé. Le premier agent dont l'invite diffère de l'exécution précédente, parce que vous avez modifié le script ou qu'un agent antérieur a retourné quelque chose de différent, s'exécute à nouveau, tout comme tous les agents après lui, même ceux qui ont terminé.
* **Toujours en cours d'exécution quand vous avez arrêté** : recommence. L'arrêt de l'ensemble de l'exécution ne compte aucun agent comme ayant échoué.
* **Échoué** : s'exécute à nouveau, tout comme tous les agents qui ont commencé après lui, même ceux qui ont terminé. L'arrêt d'un seul agent, en le sélectionnant dans [`/workflows`](#watch-the-run) et en appuyant sur `x`, compte comme un échec.

Ce dernier cas signifie qu'un échec au milieu d'une distribution réexécute le travail qui a déjà terminé. Si un script démarre A, B, C et D dans cet ordre et que B échoue, relancer retourne A du cache et exécute B, C et D à nouveau.

Vous pouvez reprendre une exécution dans la même session Claude Code. Ce qui arrive à un workflow en cours d'exécution quand vous quittez la session dépend de la façon dont vous quittez :

* Si vous [mettez la session en arrière-plan](/docs/fr/agent-view#what-carries-over-when-you-background), Claude Code rejoue l'exécution de la même manière dans la session en arrière-plan et la continue.
* Si vous quittez Claude Code pendant qu'un workflow s'exécute et que la [vue agent est activée](/docs/fr/agent-view#from-inside-a-session), la boîte de dialogue de sortie propose `Move to background and exit`, qui transfère l'exécution de la même manière. Si vous choisissez `Exit and stop tasks` à la place, ou si l'option n'est pas proposée, l'exécution s'arrête avec la session. Claude Code conserve les résultats sauvegardés de l'exécution dans le répertoire de cette session dans `~/.claude/projects/`, donc une session que vous reprenez avec `claude --resume` peut les rejouer quand vous demandez à Claude de relancer le workflow. Dans une session que vous démarrez à nouveau, Claude n'a aucune exécution antérieure à rejouer et démarre le workflow à nouveau en tant que nouvelle exécution.

Dans une [session cloud](/docs/fr/claude-code-on-the-web), Claude Code sauvegarde également les résultats de l'exécution avec l'historique de conversation de la session, qui persiste quand la VM de la session est récupérée. Quand vous [rouvrez une telle session](/docs/fr/claude-code-on-the-web#environment-expired) et demandez à Claude de relancer le workflow, les agents terminés retournent toujours leurs résultats sauvegardés.

Dans les sessions locales et cloud, quand Claude relance une exécution antérieure et que Claude Code ne peut pas trouver du tout les résultats sauvegardés de cette exécution, le relancement échoue avec une erreur `nothing to resume` au lieu de démarrer l'exécution à nouveau de son propre chef. Demandez à Claude de démarrer le workflow à nouveau en tant que nouvelle exécution.

<h3 id="when-a-run-hits-your-usage-limit">
  Quand une exécution atteint votre limite d'utilisation
</h3>

Quand un agent atteint votre limite d'utilisation [claude.ai](/docs/fr/interactive-mode#wait-for-a-usage-limit-to-reset), l'exécution se met en pause plutôt que de faire échouer cet agent : les agents qui ont atteint la limite attendent la réinitialisation, et aucun nouvel agent ne démarre. Peu de temps après la réinitialisation de la limite, les agents en attente s'exécutent à nouveau et l'exécution continue d'elle-même. Nécessite Claude Code v2.1.271 ou ultérieur ; sur les versions antérieures, les agents affectés échouent.

Pendant que l'exécution attend, sa ligne de progression dans le panneau des tâches et l'en-tête [`/workflows`](#watch-the-run) affichent quand la limite se réinitialise.

L'exécution se met en pause uniquement quand tous ces éléments sont vrais ; quand l'un d'eux ne l'est pas, l'agent affecté échoue à la place :

* La session est interactive et connectée avec un abonnement claude.ai. Une exécution ne se met pas en pause en [mode non-interactif](/docs/fr/headless) avec `claude -p` ou l'[Agent SDK](/docs/fr/agent-sdk/overview), dans une [session en arrière-plan](/docs/fr/agent-view), ou dans une session coéquipier [Remote Control](/docs/fr/remote-control) ou [équipe d'agents](/docs/fr/agent-teams).
* [`autoContinueAtUsageLimit`](/docs/fr/settings-reference#autocontinueatusagelimit) est activé, le même paramètre qui permet à la session elle-même d'[attendre la réinitialisation d'une limite d'utilisation](/docs/fr/interactive-mode#wait-for-a-usage-limit-to-reset). Si vous le désactivez pendant une attente, l'attente se termine et les agents en attente échouent.
* La limite se réinitialise dans les 24 heures. Une limite hebdomadaire peut se réinitialiser plus loin.
* L'exécution n'a pas déjà attendu deux fois. Quand elle atteint la limite une troisième fois, l'agent échoue.

<h3 id="cost">
  Coût
</h3>

Un workflow génère de nombreux agents, donc une seule exécution peut utiliser significativement plus de tokens que de travailler à travers la même tâche en conversation. Les exécutions comptent vers l'utilisation de votre plan et les limites de débit.

Pour évaluer les dépenses avant de vous engager dans une tâche importante, exécutez d'abord le workflow sur un petit échantillon : un répertoire au lieu de l'ensemble du dépôt, ou une question étroite au lieu d'une question large. La vue `/workflows` affiche l'utilisation des tokens de chaque agent au fur et à mesure que l'exécution progresse, et vous pouvez arrêter l'exécution à tout moment, généralement sans perdre le travail terminé. [Reprendre après une pause](#resume-after-a-pause) couvre ce qu'une exécution arrêtée conserve. Les [limites d'agents](#behavior-and-limits) du runtime limitent le nombre d'agents qu'une seule exécution peut générer, ce qui limite le coût d'un script qui s'échappe. Pour garder les exécutions à moins d'agents, choisissez la directive de taille `small` [](#set-a-size-guideline).

Claude Code signale également une exécution qui devient anormalement grande. Quand un workflow planifie plus de 25 agents, ou que son total de tokens projeté dépasse 1,5 million, sa ligne de progression dans le panneau des tâches sous la zone de saisie affiche un avertissement `Large workflow`. L'avertissement vous dirige vers [`/workflows`](#watch-the-run), où vous pouvez arrêter l'exécution.

L'avertissement est consultatif : il ne met pas en pause ou ne limite pas l'exécution. Deux paramètres changent quand vous le voyez :

* Si vous choisissez une [directive de taille](#set-a-size-guideline) vous-même, le nombre d'agents de la directive remplace le seuil de 25 agents. La directive par défaut intégrée laisse le seuil à 25.
* Les sessions avec [ultracode](#let-claude-decide-with-ultracode) activé n'affichent pas l'avertissement, car l'activation d'ultracode vous inscrit déjà aux exécutions importantes.

Claude Code choisit le modèle de chaque agent de workflow dans le même [ordre qu'il utilise pour les sous-agents](/docs/fr/sub-agents#choose-a-model). Un modèle que le script nomme pour une étape compte comme le modèle par invocation dans cet ordre. Quand rien d'autre n'en assigne un, l'agent s'exécute sur le modèle de votre session.

Pour contrôler le coût du modèle :

* Vérifiez `/model` avant une exécution importante si vous basculez généralement vers un modèle plus petit pour le travail de routine
* Demandez à Claude d'utiliser un modèle plus petit pour les étapes qui n'ont pas besoin du plus fort quand vous décrivez la tâche

Quand la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation bloque un modèle que le script demande pour un agent, cet agent s'exécute sur un modèle substitué à la place, en suivant les mêmes [règles de substitution que les sous-agents](/docs/fr/sub-agents#choose-a-model). La vue de progression de l'exécution dans [`/workflows`](#watch-the-run) affiche un avertissement nommant à la fois les modèles demandés et substitués.

<h3 id="set-a-size-guideline">
  Définir une directive de taille
</h3>

Une directive de taille indique à Claude combien d'agents viser quand il écrit un workflow dynamique. Claude Code envoie la directive à Claude comme conseil, pas comme une limite, donc une invite qui appelle une échelle différente la remplace toujours. Nécessite Claude Code v2.1.202 ou ultérieur.

Chaque valeur correspond à un nombre d'agents :

| Valeur         | Nombre d'agents que Claude vise                              |
| :------------- | :----------------------------------------------------------- |
| `unrestricted` | Aucune directive : Claude dimensionne le workflow à la tâche |
| `small`        | Moins de 5 agents                                            |
| `medium`       | Moins de 10 agents                                           |
| `large`        | Moins de 50 agents                                           |

La valeur par défaut est `medium`, ou `small` quand vous êtes connecté sur un plan Pro avec Claude Code v2.1.271 ou ultérieur. Jusqu'à ce que vous choisissiez une valeur, la ligne `/config` affiche la valeur comme par défaut, et la ligne `Running in background` du workflow affiche la taille en vigueur. Nécessite Claude Code v2.1.219 ou ultérieur ; les versions antérieures utilisent par défaut `unrestricted`.

Pour modifier la directive, choisissez une valeur pour le paramètre Dynamic workflow size dans `/config`, ou exécutez `/config workflowSizeGuideline=small`. Sur v2.1.219 et ultérieur, vous pouvez également définir la clé [`workflowSizeGuideline`](/docs/fr/settings-reference#workflowsizeguideline) dans n'importe quel fichier de paramètres ; cette valeur a la priorité sur `/config`, et Claude Code masque la ligne `/config` tandis qu'un fichier de paramètres en fournit une.

Les modifications prennent effet à l'invite suivante. Les [limites d'agents du runtime](#behavior-and-limits) s'appliquent toujours indépendamment du paramètre.

<h3 id="turn-workflows-off">
  Désactiver les workflows
</h3>

Les workflows sont disponibles dans le CLI, l'application Desktop, les extensions IDE, le [mode non-interactif](/docs/fr/headless) avec `claude -p`, et l'[Agent SDK](/docs/fr/agent-sdk/overview). Les mêmes paramètres de désactivation s'appliquent sur chaque surface.

Pour désactiver les workflows pour vous-même :

* Basculez Dynamic workflows off dans `/config`. Persiste entre les sessions.
* Définissez `"disableWorkflows": true` dans `~/.claude/settings.json`. Persiste entre les sessions.
* Définissez `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Lire au démarrage, donc cela s'applique partout où vous le définissez.

Pour désactiver les workflows pour toute votre organisation, définissez `"disableWorkflows": true` dans les [paramètres gérés](/docs/fr/server-managed-settings), ou utilisez le bouton bascule sur la page des [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code).

Quand les workflows sont désactivés, les commandes de workflow groupées et la compétence `/workflow-authoring` ne sont pas disponibles, le mot-clé `ultracode` ne déclenche plus une exécution, et `ultracode` est supprimé du menu `/effort`.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Exécuter les agents en parallèle](/docs/fr/agents) : comparer les sous-agents, la vue des agents, les équipes d'agents et les workflows
* [Créer des sous-agents personnalisés](/docs/fr/sub-agents) : la primitive worker que les workflows orchestrent
* [Gérer les coûts](/docs/fr/costs) : comment les exécutions multi-agents comptent vers les limites d'utilisation
