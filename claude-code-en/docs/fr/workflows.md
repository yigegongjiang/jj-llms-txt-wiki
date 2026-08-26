> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrer des sous-agents à grande échelle avec des workflows dynamiques

> Les workflows dynamiques orchestrent de nombreux sous-agents à partir d'un script que Claude écrit et que vous pouvez relancer. Utilisez-les pour les audits de base de code, les migrations importantes et la recherche avec vérification croisée.

<Note>
  Les workflows dynamiques nécessitent Claude Code v2.1.154 ou version ultérieure et sont disponibles sur tous les plans payants, avec accès à l'API Anthropic, et sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry. Sur Pro, activez-les à partir de la ligne Dynamic workflows dans `/config`.
</Note>

Un workflow dynamique est un script JavaScript qui orchestre des [sous-agents](/docs/fr/sub-agents) à grande échelle. Claude écrit le script pour la tâche que vous décrivez, et un runtime l'exécute en arrière-plan tandis que votre session reste réactive.

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
    Exécutez `/deep-research` avec une question que vous souhaitez enquêter. Il distribue les recherches web sur plusieurs angles, récupère et vérifie les sources qu'il trouve, et synthétise un rapport cité.

    ```text theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Autoriser les workflows">
    Claude Code demande s'il faut autoriser le workflow. Sélectionnez **Oui** pour continuer. L'invite exacte dépend de votre mode de permission. Voir [Approuver le plan avant qu'il s'exécute](#approve-the-plan-before-it-runs) pour les options par mode.
  </Step>

  <Step title="Regarder la progression">
    L'exécution commence en arrière-plan. Exécutez `/workflows`, utilisez les touches fléchées pour sélectionner l'exécution, et appuyez sur Entrée pour ouvrir sa vue de progression :

    ```text theme={null}
    /workflows
    ```

    La vue affiche chaque phase avec son nombre d'agents, le total des tokens et le temps écoulé. Explorez n'importe quelle phase pour voir ses agents et ce que chacun a trouvé. Voir [Regarder l'exécution](#watch-the-run) pour l'ensemble complet des contrôles.

    Vous pouvez également regarder à partir du panneau des tâches sous la zone de saisie : un résumé de progression d'une ligne apparaît là pendant que l'exécution se déroule. Appuyez sur la flèche vers le bas pour le mettre au point, puis Entrée pour l'agrandir.
  </Step>

  <Step title="Lire le rapport">
    Quand l'exécution se termine, le rapport atterrit dans votre session. Il cite les sources dont provient chaque affirmation, les affirmations qui n'ont pas survécu à la vérification croisée étant déjà filtrées.

    À partir de la v2.1.196, quand les agents vérificateurs ne peuvent pas vérifier une affirmation, par exemple après une limite de débit ou une erreur API, le rapport liste cette affirmation comme non vérifiée au lieu de la compter comme réfutée.
  </Step>
</Steps>

Pour exécuter un workflow pour votre propre tâche, [faites écrire un par Claude](#have-claude-write-a-workflow), et une fois qu'une exécution fait ce que vous vouliez, vous pouvez [l'enregistrer](#save-the-workflow-for-reuse) comme commande de votre propre.

<h3 id="bundled-workflows">
  Workflows groupés
</h3>

Claude Code inclut `/deep-research` comme workflow intégré :

| Commande                    | Ce qu'elle fait                                                                                                                                                                                                                                                                                                                                             |
| :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | Distribue les recherches web sur une question sur plusieurs angles, récupère et vérifie les sources qu'elle trouve, vote sur chaque affirmation, et retourne un rapport cité avec les affirmations qui n'ont pas survécu à la vérification croisée filtrées. Nécessite que l'outil [WebSearch](/docs/fr/tools-reference#websearch-tool-behavior) soit disponible |

Les [workflows que vous enregistrez](#save-the-workflow-for-reuse) vous-même deviennent des commandes de la même manière et apparaissent dans l'autocomplétion `/` aux côtés des workflows intégrés.

<h3 id="watch-the-run">
  Regarder l'exécution
</h3>

Les workflows s'exécutent en arrière-plan, donc la session reste réactive pendant que les agents travaillent. Exécutez `/workflows` à tout moment pour lister les workflows en cours d'exécution et terminés, puis sélectionnez-en un pour ouvrir sa vue de progression.

```text theme={null}
/workflows
```

La vue de progression affiche chaque phase avec ses nombres d'agents, ses totaux de tokens et son temps écoulé. Le pied de page liste la clé pour chaque action :

| Clé             | Action                                                                                                                                                             |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`       | Sélectionner une phase ou un agent                                                                                                                                 |
| `Entrée` ou `→` | Explorez la phase sélectionnée, puis un agent pour lire son prompt, ses appels d'outils récents et son résultat                                                    |
| `Échap` ou `←`  | Revenir un niveau en arrière. Dans les v2.1.203 à v2.1.205, `←` n'a pas permis de revenir en arrière d'une phase ou d'un agent ; utilisez `Échap` sur ces versions |
| `j` / `k`       | Faire défiler dans le détail de l'agent quand il déborde                                                                                                           |
| `f`             | Filtrer la liste des agents dans la phase sélectionnée par statut. Appuyez à nouveau pour parcourir                                                                |
| `p`             | Mettre en pause ou reprendre l'exécution                                                                                                                           |
| `x`             | Arrêter l'agent sélectionné, ou arrêter le workflow entier quand le focus est sur l'exécution                                                                      |
| `r`             | Redémarrer l'agent en cours d'exécution sélectionné                                                                                                                |
| `s`             | [Enregistrer](#save-the-workflow-for-reuse) le script de l'exécution comme commande                                                                                |

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

Pour exécuter une seule tâche en tant que workflow sans modifier le niveau d'effort de la session, incluez le mot clé `ultracode` dans votre prompt. Demander avec vos propres mots, par exemple « utiliser un workflow » ou « exécuter un workflow », fonctionne également : Claude traite une demande directe comme le même opt-in. Avant la v2.1.160, le mot clé déclencheur littéral était `workflow` ; les demandes en langage naturel fonctionnent dans les deux versions.

```text theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code met en évidence le mot clé dans votre saisie et Claude écrit un script de workflow pour la tâche au lieu de la traiter tour par tour. Si vous ne vouliez pas démarrer un workflow, appuyez sur `Option+W` sur macOS ou `Alt+W` sur Windows et Linux pour ignorer la mise en évidence pour ce prompt, ou appuyez sur retour arrière tandis que le curseur se trouve juste après le mot clé en évidence. Pour empêcher le mot clé de déclencher quoi que ce soit, désactivez le déclencheur de mot clé Ultracode dans `/config`.

Si l'exécution fait ce que vous vouliez, vous pouvez [l'enregistrer comme commande](#save-the-workflow-for-reuse) après.

Si vous avez déjà un orchestrateur construit d'une autre façon, comme un dossier de prompts de sous-agents ou une compétence qui distribue le travail, vous pouvez pointer Claude vers celui-ci et demander un workflow qui fait la même chose.

<h3 id="let-claude-decide-with-ultracode">
  Laisser Claude décider avec ultracode
</h3>

Ultracode est un paramètre Claude Code qui combine l'[effort de raisonnement](/docs/fr/model-config#adjust-effort-level) `xhigh` avec l'orchestration automatique des workflows. Avec lui activé, Claude planifie un workflow pour chaque tâche substantielle au lieu d'attendre que vous le demandiez.

```text theme={null}
/effort ultracode
```

Pour démarrer une session avec ultracode déjà activé, lancez avec `claude --effort ultracode`. Nécessite Claude Code v2.1.203 ou ultérieur.

Avec ultracode activé, Claude décide quand une tâche justifie un workflow. Une seule demande peut se transformer en plusieurs workflows d'affilée : un pour comprendre le code, un pour faire le changement, et un pour le vérifier. Cela s'applique à chaque tâche de la session, donc chaque demande utilise plus de tokens et prend plus de temps qu'aux niveaux d'effort inférieurs.

Ultracode dure pour la session actuelle et se réinitialise quand vous en commencez une nouvelle. Revenez avec `/effort high` quand vous retournez au travail de routine. Il est disponible sur les modèles qui supportent l'[effort](/docs/fr/model-config#adjust-effort-level) `xhigh` ; sur les autres modèles, le menu `/effort` ne l'offre pas.

<h3 id="approve-the-plan-before-it-runs">
  Approuver le plan avant qu'il s'exécute
</h3>

Dans le CLI, l'invite par exécution affiche les phases planifiées et ces options :

* **Oui, l'exécuter** : démarrer l'exécution
* **Oui, et ne pas demander à nouveau pour `<name>` dans `<path>`** : démarrer, et ignorer cette invite pour ce workflow dans ce projet à partir de maintenant
* **Afficher le script brut** : lire le script avant de décider
* **Non** : annuler

`Ctrl+G` ouvre le script dans votre éditeur. `Tab` vous permet d'ajuster le prompt avant le démarrage de l'exécution.

Que vous voyiez cette invite dépend de votre [mode de permission](/docs/fr/permission-modes) :

| Mode de permission                                 | Quand vous êtes invité                                                                                                                                                                                      |
| :------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Par défaut, accepter les modifications             | À chaque exécution, sauf si vous avez sélectionné **Oui, et ne pas demander à nouveau** pour ce workflow dans ce projet                                                                                     |
| Auto                                               | Première exécution uniquement. Tout **Oui** enregistre le consentement dans vos paramètres utilisateur, et les exécutions ultérieures commencent sans invite. Ignoré entièrement quand ultracode est activé |
| Contourner les permissions, `claude -p`, Agent SDK | Jamais. L'exécution commence immédiatement                                                                                                                                                                  |

Dans l'application Desktop, une carte d'approbation affiche le nom du workflow, la liste des phases et une mise en garde sur l'utilisation des tokens, avec les actions **Une fois**, **Toujours** et **Refuser**. La vue de progression apparaît dans le volet des tâches en arrière-plan.

Votre mode de permission contrôle uniquement l'invite de lancement ci-dessus. Les sous-agents que le workflow génère s'exécutent toujours en mode `acceptEdits` et héritent de votre [liste d'autorisation d'outils](/docs/fr/settings#permission-settings), quel que soit le mode de votre session. Les modifications de fichiers sont approuvées automatiquement.

Les commandes shell, les récupérations web et les outils MCP qui ne sont pas dans votre liste d'autorisation peuvent toujours vous inviter pendant l'exécution. Pour éviter cela lors d'une exécution longue, ajoutez les commandes dont les agents ont besoin à votre liste d'autorisation avant de commencer.

Dans `claude -p` et l'Agent SDK, il n'y a personne pour inviter, donc les appels d'outils suivent vos règles de permission configurées sans confirmation interactive.

<h3 id="save-the-workflow-for-reuse">
  Enregistrer le workflow pour réutilisation
</h3>

Quand Claude écrit un workflow pour une tâche que vous répéterez, vous pouvez enregistrer le script de cette exécution comme commande. Un processus comme une revue que vous exécutez sur chaque branche exécute ensuite la même orchestration à chaque fois.

Exécutez `/workflows`, sélectionnez l'exécution que vous voulez conserver, et appuyez sur `s`. Dans la boîte de dialogue d'enregistrement, Tab bascule entre les deux emplacements d'enregistrement :

* `.claude/workflows/` dans votre projet : partagé avec tous ceux qui clonent le repo
* `~/.claude/workflows/` dans votre répertoire personnel : disponible dans chaque projet, visible uniquement pour vous. Si vous définissez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars), cet emplacement est le répertoire `workflows/` sous ce chemin.

La boîte de dialogue d'enregistrement affiche le chemin résolu pour l'emplacement personnel. Avant la v2.1.208, elle affichait `~/.claude/workflows/` même quand `CLAUDE_CONFIG_DIR` était défini ; le fichier était toujours enregistré sous le répertoire configuré.

Appuyez sur Entrée pour enregistrer. Le workflow s'exécute comme `/<name>` dans les futures sessions à partir de l'un ou l'autre emplacement.

Dans un monorepo avec plusieurs répertoires `.claude/`, vous pouvez conserver les workflows aux côtés du package auquel ils s'appliquent. À partir de la v2.1.178, l'enregistrement à l'emplacement du projet écrit dans le répertoire `.claude/workflows/` le plus proche qui existe déjà entre votre répertoire de travail et la racine du référentiel, ou à la racine du référentiel s'il n'en existe pas encore. Les workflows de projet se chargent également à partir de chaque `.claude/workflows/` le long de ce chemin, et quand plus d'un définit le même nom, Claude Code exécute celui le plus proche du répertoire de travail.

Si un workflow de projet et un workflow personnel partagent un nom, celui du projet s'exécute.

<h3 id="pass-input-to-a-saved-workflow">
  Passer une entrée à un workflow enregistré
</h3>

Un workflow enregistré peut accepter une entrée via le paramètre `args`. Le script la lit comme une variable globale nommée `args`. Utilisez ceci pour fournir une question de recherche, une liste de chemins cibles, ou un objet de configuration au moment de l'invocation au lieu de modifier le script pour chaque exécution.

L'invite suivante exécute un workflow enregistré avec une liste de numéros de problème :

```text theme={null}
> Run /triage-issues on issues 1024, 1025, and 1030
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

```text theme={null}
> use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Continuer à corriger jusqu'à ce qu'une vérification réussisse
</h3>

Exécutez un vérificateur, corrigez ce qui a échoué, et répétez jusqu'à ce qu'il réussisse ou cesse de faire des progrès.

```text theme={null}
> use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrer de nombreux fichiers en parallèle
</h3>

Découvrez les fichiers à migrer, transformez chacun dans une copie isolée afin que les modifications ne se chevauchent pas, et vérifiez chaque résultat.

```text theme={null}
> use a workflow to migrate every component under src/components/ from styled-components to Tailwind, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Examiner chaque fichier modifié et écrire un résumé
</h3>

Exécutez un examinateur par fichier, puis remettez toutes les conclusions à un agent qui les classe et les déduplique.

```text theme={null}
> use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Rechercher un sujet à travers de nombreuses sources
</h3>

Distribuez les lecteurs sur les journaux des modifications, les problèmes et la documentation, puis synthétisez. Le workflow groupé `/deep-research` fait cela ; vous pouvez également décrire une version plus étroite.

```text theme={null}
> use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Trouver des problèmes jusqu'à ce que la liste cesse de croître
</h3>

Continuez à chercher par rounds et arrêtez-vous quand les nouveaux rounds ne trouvent rien de nouveau.

```text theme={null}
> use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
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

Le corps est du JavaScript simple avec `await` au niveau supérieur. `agent()` génère un sous-agent et `pipeline()` en exécute un par élément dans une liste. Si vous voulez éditer un script à la main, demandez à Claude de vous guider à travers le changement, ou consultez l'entrée de l'outil Workflow dans la [référence Agent SDK](/docs/fr/agent-sdk/typescript) pour l'ensemble complet des options.

<h2 id="how-a-workflow-runs">
  Comment un workflow s'exécute
</h2>

Le runtime du workflow exécute le script dans un environnement isolé, séparé de votre conversation. Les résultats intermédiaires restent dans les variables du script au lieu d'atterrir dans le contexte de Claude.

Chaque exécution écrit son script dans un fichier sous le répertoire de votre session dans `~/.claude/projects/`. Claude reçoit le chemin au démarrage de l'exécution, vous pouvez donc le demander. Vous pouvez ouvrir ce fichier pour lire l'orchestration que Claude a écrite, la comparer avec le script d'une exécution précédente, ou l'éditer et demander à Claude de relancer à partir de la version éditée.

Le runtime suit le résultat de chaque agent au fur et à mesure que l'exécution progresse, ce qui rend une exécution [reprendre](#resume-after-a-pause) possible dans la même session.

<h3 id="behavior-and-limits">
  Comportement et limites
</h3>

Le runtime applique les contraintes suivantes :

| Contrainte                                                                          | Pourquoi                                                                                                                                                             |
| :---------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pas d'entrée utilisateur en cours d'exécution                                       | Seules les invites de permission d'agent peuvent mettre en pause une exécution. Pour l'approbation entre les étapes, exécutez chaque étape comme son propre workflow |
| Pas d'accès direct au système de fichiers ou au shell à partir du workflow lui-même | Les agents lisent, écrivent et exécutent des commandes. Le script coordonne les agents                                                                               |
| Jusqu'à 16 agents concurrents, moins sur les machines avec des cœurs CPU limités    | Limite l'utilisation des ressources locales                                                                                                                          |
| 1 000 agents au total par exécution                                                 | Empêche les boucles incontrôlées                                                                                                                                     |

<h2 id="manage-runs">
  Gérer les exécutions
</h2>

Une fois qu'une exécution commence, vous la gérez à partir de la vue `/workflows`, ou en agrandissant sa ligne de progression dans le panneau des tâches sous la zone de saisie.

<h3 id="resume-after-a-pause">
  Reprendre après une pause
</h3>

Si vous arrêtez une exécution, vous pouvez la reprendre : les agents qui ont déjà terminé retournent leurs résultats en cache, et le reste s'exécute en direct. Un agent qui était encore en cours d'exécution quand vous avez arrêté n'est pas sauvegardé et recommence à la reprise, donc un workflow qui distribue le travail sur de nombreux petits agents préserve plus de progrès qu'un seul agent long. Reprenez une exécution en pause à partir de `/workflows` en la sélectionnant et en appuyant sur `p`, ou demandez à Claude de relancer le workflow avec le même script.

La reprise fonctionne dans la même session Claude Code. Si vous quittez Claude Code pendant qu'un workflow s'exécute, la session suivante démarre le workflow à nouveau.

<h3 id="cost">
  Coût
</h3>

Un workflow génère de nombreux agents, donc une seule exécution peut utiliser significativement plus de tokens que de travailler à travers la même tâche en conversation. Les exécutions comptent vers l'utilisation de votre plan et les limites de débit comme toute autre session.

Pour évaluer les dépenses avant de vous engager dans une tâche importante, exécutez d'abord le workflow sur un petit échantillon : un répertoire au lieu de l'ensemble du dépôt, ou une question étroite au lieu d'une question large. La vue `/workflows` affiche l'utilisation des tokens de chaque agent au fur et à mesure que l'exécution progresse, et vous pouvez arrêter l'exécution à tout moment sans perdre le travail terminé. Les [limites de comportement](#behavior-and-limits) du runtime limitent le nombre d'agents qu'une seule exécution peut générer, ce qui limite le coût d'un script qui s'échappe. Pour garder chaque exécution plus petite par défaut, [définissez une directive de taille](#set-a-size-guideline) dans `/config`.

Claude Code signale également une exécution qui devient anormalement grande. Quand un workflow planifie plus de 25 agents, ou que son total de tokens projeté dépasse 1,5 million, sa ligne de progression dans le panneau des tâches sous la zone de saisie affiche un avertissement `Large workflow`. L'avertissement vous dirige vers [`/workflows`](#watch-the-run), où vous pouvez arrêter l'exécution. Nécessite Claude Code v2.1.203 ou ultérieur.

L'avertissement est consultatif : il ne met pas en pause ou ne limite pas l'exécution. Deux paramètres changent quand vous le voyez :

* Si vous [définissez une directive de taille](#set-a-size-guideline), le nombre d'agents de la directive remplace le seuil de 25 agents.
* Les sessions avec [ultracode](#let-claude-decide-with-ultracode) activé n'affichent pas l'avertissement, car l'activation d'ultracode vous inscrit déjà aux exécutions importantes.

Chaque agent dans un workflow utilise le modèle de votre session sauf si le script achemine une étape vers un autre ou si la variable d'environnement [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/fr/model-config#environment-variables) est définie, ce qui remplace les deux. Pour contrôler le coût du modèle :

* Vérifiez `/model` avant une exécution importante si vous basculez généralement vers un modèle plus petit pour le travail de routine
* Demandez à Claude d'utiliser un modèle plus petit pour les étapes qui n'ont pas besoin du plus fort quand vous décrivez la tâche

<h3 id="set-a-size-guideline">
  Définir une directive de taille
</h3>

Le paramètre Dynamic workflow size dans `/config` garde les workflows que Claude écrit à une échelle plus petite par défaut. Claude Code envoie le paramètre à Claude comme conseil, donc une invite qui appelle une échelle différente le remplace toujours. Nécessite Claude Code v2.1.202 ou ultérieur.

Chaque valeur définit le nombre d'agents que Claude vise dans les scripts qu'il écrit.

| Valeur         | Conseil envoyé à Claude                       |
| :------------- | :-------------------------------------------- |
| `unrestricted` | Aucune directive. C'est la valeur par défaut. |
| `small`        | Viser moins de 5 agents.                      |
| `medium`       | Viser moins de 15 agents.                     |
| `large`        | Viser moins de 50 agents.                     |

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

Quand les workflows sont désactivés, les commandes de workflow groupées ne sont pas disponibles, le mot-clé `ultracode` ne déclenche plus une exécution, et `ultracode` est supprimé du menu `/effort`.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Exécuter les agents en parallèle](/docs/fr/agents) : comparer les sous-agents, la vue des agents, les équipes d'agents et les workflows
* [Créer des sous-agents personnalisés](/docs/fr/sub-agents) : la primitive worker que les workflows orchestrent
* [Gérer les coûts](/docs/fr/costs) : comment les exécutions multi-agents comptent vers les limites d'utilisation
