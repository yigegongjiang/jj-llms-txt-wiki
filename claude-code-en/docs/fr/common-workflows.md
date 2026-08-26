> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Flux de travail courants

> Guides étape par étape pour explorer les bases de code, corriger les bogues, refactoriser, tester et autres tâches quotidiennes avec Claude Code.

Cette page rassemble de courtes recettes pour le développement quotidien. Pour des conseils de plus haut niveau sur les prompts et la gestion du contexte, consultez [Bonnes pratiques](/docs/fr/best-practices).

Cette page couvre :

* [Recettes de prompts](#prompt-recipes) pour explorer le code, corriger les bogues, refactoriser, tester, les PR et la documentation
* [Reprendre les conversations précédentes](#resume-previous-conversations) pour qu'une tâche puisse s'étendre sur plusieurs sessions
* [Exécuter des sessions parallèles avec worktrees](#run-parallel-sessions-with-worktrees) pour que les modifications concurrentes ne se heurtent pas
* [Planifier avant de modifier](#plan-before-editing) pour examiner les modifications avant qu'elles ne touchent le disque
* [Déléguer la recherche à des subagents](#delegate-research-to-subagents) pour garder votre contexte principal propre
* [Canaliser Claude dans des scripts](#pipe-claude-into-scripts) pour CI et le traitement par lot

<h2 id="prompt-recipes">
  Recettes de prompts
</h2>

Ce sont des modèles de prompts pour les tâches quotidiennes comme explorer du code inconnu, déboguer, refactoriser, écrire des tests et créer des PR. Chacun fonctionne sur n'importe quelle surface Claude Code ; adaptez la formulation à votre projet.

<h3 id="understand-new-codebases">
  Comprendre les nouvelles bases de code
</h3>

Pour configurer Claude Code dans un monorepo ou une grande base de code, consultez [Monorepos et grands référentiels](/docs/fr/large-codebases).

<h4 id="get-a-quick-codebase-overview">
  Obtenir un aperçu rapide de la base de code
</h4>

Supposons que vous venez de rejoindre un nouveau projet et que vous devez comprendre rapidement sa structure.

<Steps>
  <Step title="Accédez au répertoire racine du projet">
    ```bash theme={null}
    cd /path/to/project 
    ```
  </Step>

  <Step title="Démarrez Claude Code">
    ```bash theme={null}
    claude 
    ```
  </Step>

  <Step title="Demandez un aperçu de haut niveau">
    ```text theme={null}
    give me an overview of this codebase
    ```
  </Step>

  <Step title="Approfondissez les composants spécifiques">
    ```text theme={null}
    explain the main architecture patterns used here
    ```

    ```text theme={null}
    what are the key data models?
    ```

    ```text theme={null}
    how is authentication handled?
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Commencez par des questions larges, puis réduisez à des domaines spécifiques
  * Posez des questions sur les conventions de codage et les modèles utilisés dans le projet
  * Demandez un glossaire des termes spécifiques au projet
</Tip>

<h4 id="find-relevant-code">
  Trouver du code pertinent
</h4>

Supposons que vous ayez besoin de localiser du code lié à une fonctionnalité ou une capacité spécifique.

<Steps>
  <Step title="Demandez à Claude de trouver les fichiers pertinents">
    ```text theme={null}
    find the files that handle user authentication
    ```
  </Step>

  <Step title="Obtenez du contexte sur la façon dont les composants interagissent">
    ```text theme={null}
    how do these authentication files work together?
    ```
  </Step>

  <Step title="Comprenez le flux d'exécution">
    ```text theme={null}
    trace the login process from front-end to database
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Soyez spécifique sur ce que vous recherchez
  * Utilisez le langage du domaine du projet
  * Installez un [plugin d'intelligence de code](/docs/fr/discover-plugins#code-intelligence) pour votre langage afin de donner à Claude une navigation précise ' aller à la définition ' et ' trouver les références '
</Tip>

***

<h3 id="fix-bugs-efficiently">
  Corriger les bogues efficacement
</h3>

Supposons que vous ayez rencontré un message d'erreur et que vous ayez besoin de trouver et de corriger sa source.

<Steps>
  <Step title="Partagez l'erreur avec Claude">
    ```text theme={null}
    I'm seeing an error when I run npm test
    ```
  </Step>

  <Step title="Demandez des recommandations de correction">
    ```text theme={null}
    suggest a few ways to fix the @ts-ignore in user.ts
    ```
  </Step>

  <Step title="Appliquez la correction">
    ```text theme={null}
    update user.ts to add the null check you suggested
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Dites à Claude la commande pour reproduire le problème et obtenir une trace de pile
  * Mentionnez les étapes pour reproduire l'erreur
  * Faites savoir à Claude si l'erreur est intermittente ou cohérente
</Tip>

***

<h3 id="refactor-code">
  Refactoriser le code
</h3>

Supposons que vous ayez besoin de mettre à jour du code ancien pour utiliser des modèles et des pratiques modernes.

<Steps>
  <Step title="Identifiez le code hérité pour la refactorisation">
    ```text theme={null}
    find deprecated API usage in our codebase
    ```
  </Step>

  <Step title="Obtenez des recommandations de refactorisation">
    ```text theme={null}
    suggest how to refactor utils.js to use modern JavaScript features
    ```
  </Step>

  <Step title="Appliquez les modifications en toute sécurité">
    ```text theme={null}
    refactor utils.js to use ES2024 features while maintaining the same behavior
    ```
  </Step>

  <Step title="Vérifiez la refactorisation">
    ```text theme={null}
    run tests for the refactored code
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Demandez à Claude d'expliquer les avantages de l'approche moderne
  * Demandez que les modifications maintiennent la compatibilité rétroactive si nécessaire
  * Effectuez la refactorisation par petits incréments testables
</Tip>

***

<h3 id="work-with-tests">
  Travailler avec les tests
</h3>

Supposons que vous ayez besoin d'ajouter des tests pour du code non couvert.

<Steps>
  <Step title="Identifiez le code non testé">
    ```text theme={null}
    find functions in NotificationsService.swift that are not covered by tests
    ```
  </Step>

  <Step title="Générez l'échafaudage des tests">
    ```text theme={null}
    add tests for the notification service
    ```
  </Step>

  <Step title="Ajoutez des cas de test significatifs">
    ```text theme={null}
    add test cases for edge conditions in the notification service
    ```
  </Step>

  <Step title="Exécutez et vérifiez les tests">
    ```text theme={null}
    run the new tests and fix any failures
    ```
  </Step>
</Steps>

Claude peut générer des tests qui suivent les modèles et conventions existants de votre projet. Lorsque vous demandez des tests, soyez spécifique sur le comportement que vous souhaitez vérifier. Claude examine vos fichiers de test existants pour correspondre au style, aux frameworks et aux modèles d'assertion déjà en usage.

Pour une couverture complète, demandez à Claude d'identifier les cas limites que vous auriez pu manquer. Claude peut analyser vos chemins de code et suggérer des tests pour les conditions d'erreur, les valeurs limites et les entrées inattendues qui sont faciles à oublier.

***

<h3 id="create-pull-requests">
  Créer des demandes de tirage
</h3>

Vous pouvez créer des demandes de tirage en demandant directement à Claude (« créer une pr pour mes modifications »), ou guider Claude à travers cela étape par étape :

<Steps>
  <Step title="Résumez vos modifications">
    ```text theme={null}
    summarize the changes I've made to the authentication module
    ```
  </Step>

  <Step title="Générez une demande de tirage">
    ```text theme={null}
    create a pr
    ```
  </Step>

  <Step title="Examinez et affinez">
    ```text theme={null}
    enhance the PR description with more context about the security improvements
    ```
  </Step>
</Steps>

Lorsque vous créez une PR en utilisant `gh pr create`, la session est automatiquement liée à cette PR. Pour la reprendre plus tard, exécutez `claude --from-pr 123`, en remplaçant 123 par le numéro de la PR, ou collez l'URL de la PR dans le sélecteur [`/resume`](/docs/fr/sessions#use-the-session-picker).

<Tip>
  Examinez la PR générée par Claude avant de la soumettre et demandez à Claude de mettre en évidence les risques ou considérations potentiels.
</Tip>

<h3 id="handle-documentation">
  Gérer la documentation
</h3>

Supposons que vous ayez besoin d'ajouter ou de mettre à jour la documentation de votre code.

<Steps>
  <Step title="Identifiez le code non documenté">
    ```text theme={null}
    find functions without proper JSDoc comments in the auth module
    ```
  </Step>

  <Step title="Générez la documentation">
    ```text theme={null}
    add JSDoc comments to the undocumented functions in auth.js
    ```
  </Step>

  <Step title="Examinez et améliorez">
    ```text theme={null}
    improve the generated documentation with more context and examples
    ```
  </Step>

  <Step title="Vérifiez la documentation">
    ```text theme={null}
    check if the documentation follows our project standards
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Spécifiez le style de documentation que vous souhaitez (JSDoc, docstrings, etc.)
  * Demandez des exemples dans la documentation
  * Demandez la documentation pour les API publiques, les interfaces et la logique complexe
</Tip>

***

<h3 id="work-in-notes-and-non-code-folders">
  Travailler dans les notes et les dossiers non-code
</h3>

Claude Code fonctionne dans n'importe quel répertoire. Exécutez-le à l'intérieur d'un coffre-fort de notes, d'un dossier de documentation ou de toute collection de fichiers markdown pour rechercher, modifier et réorganiser le contenu de la même manière que vous le feriez pour du code.

Le répertoire `.claude/` et `CLAUDE.md` se trouvent aux côtés des répertoires de configuration d'autres outils sans conflit. Claude lit les fichiers à nouveau à chaque appel d'outil, il voit donc les modifications que vous apportez dans une autre application la prochaine fois qu'il lit ce fichier.

***

<h3 id="work-with-images">
  Travailler avec les images
</h3>

Supposons que vous ayez besoin de travailler avec des images dans votre base de code et que vous souhaitiez l'aide de Claude pour analyser le contenu des images.

<Steps>
  <Step title="Ajoutez une image à la conversation">
    Vous pouvez utiliser l'une de ces méthodes :

    1. Glissez-déposez une image dans la fenêtre Claude Code
    2. Copiez une image et collez-la dans l'interface CLI avec Ctrl+V. Sur macOS, Cmd+V fonctionne également dans iTerm2.
    3. Fournissez un chemin d'image à Claude. Par exemple, « Analyser cette image : /path/to/your/image.png »
  </Step>

  <Step title="Demandez à Claude d'analyser l'image">
    ```text theme={null}
    What does this image show?
    ```

    ```text theme={null}
    Describe the UI elements in this screenshot
    ```

    ```text theme={null}
    Are there any problematic elements in this diagram?
    ```
  </Step>

  <Step title="Utilisez les images pour le contexte">
    ```text theme={null}
    Here's a screenshot of the error. What's causing it?
    ```

    ```text theme={null}
    This is our current database schema. How should we modify it for the new feature?
    ```
  </Step>

  <Step title="Obtenez des suggestions de code à partir du contenu visuel">
    ```text theme={null}
    Generate CSS to match this design mockup
    ```

    ```text theme={null}
    What HTML structure would recreate this component?
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Utilisez les images quand les descriptions textuelles seraient peu claires ou fastidieuses
  * Incluez des captures d'écran d'erreurs, de conceptions d'interface utilisateur ou de diagrammes pour un meilleur contexte
  * Vous pouvez travailler avec plusieurs images dans une conversation
  * L'analyse d'images fonctionne avec les diagrammes, les captures d'écran, les maquettes et bien d'autres
  * Quand Claude référence des images (par exemple, `[Image #1]`), `Cmd+Click` (Mac) ou `Ctrl+Click` (Windows/Linux) le lien pour ouvrir l'image dans votre visionneuse par défaut
</Tip>

***

<h3 id="reference-files-and-directories">
  Référencer les fichiers et répertoires
</h3>

Utilisez @ pour inclure rapidement des fichiers ou des répertoires sans attendre que Claude les lise.

<Steps>
  <Step title="Référencez un seul fichier">
    ```text theme={null}
    Explain the logic in @src/utils/auth.js
    ```

    Cela inclut le contenu complet du fichier dans la conversation.
  </Step>

  <Step title="Référencez un répertoire">
    ```text theme={null}
    What's the structure of @src/components?
    ```

    Cela fournit une liste de répertoires avec les informations de fichier.
  </Step>

  <Step title="Référencez les ressources MCP">
    ```text theme={null}
    Show me the data from @github:repos/owner/repo/issues
    ```

    Cela récupère les données des serveurs MCP connectés en utilisant le format @server:resource. Consultez [Ressources MCP](/docs/fr/mcp#use-mcp-resources) pour plus de détails.
  </Step>
</Steps>

<Tip>
  Conseils :

  * Les chemins de fichiers peuvent être relatifs ou absolus
  * Les références de fichiers @ ajoutent `CLAUDE.md` dans le répertoire du fichier et les répertoires parents au contexte
  * Les références de répertoires affichent les listes de fichiers, pas les contenus
  * Vous pouvez référencer plusieurs fichiers dans un seul message (par exemple, « @file1.js et @file2.js »)
</Tip>

***

<h3 id="run-claude-on-a-schedule">
  Exécuter Claude selon un calendrier
</h3>

Supposons que vous souhaitiez que Claude gère une tâche automatiquement de manière récurrente, comme examiner les PR ouvertes chaque matin, auditer les dépendances chaque semaine ou vérifier les échecs CI pendant la nuit.

Choisissez une option de planification en fonction de l'endroit où vous souhaitez que la tâche s'exécute :

| Option                                                         | Où elle s'exécute                          | Idéale pour                                                                                                                                                                                                                                               |
| :------------------------------------------------------------- | :----------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Routines](/docs/fr/routines)                                       | Infrastructure gérée par Anthropic         | Les tâches qui doivent s'exécuter même quand votre ordinateur est éteint. Peuvent également se déclencher sur les appels API ou les événements GitHub en plus d'un calendrier. Configurez sur [claude.ai/code/routines](https://claude.ai/code/routines). |
| [Tâches planifiées sur le bureau](/docs/fr/desktop-scheduled-tasks) | Votre machine, via l'application de bureau | Les tâches qui ont besoin d'un accès direct aux fichiers locaux, aux outils ou aux modifications non validées.                                                                                                                                            |
| [GitHub Actions](/docs/fr/github-actions)                           | Votre pipeline CI                          | Les tâches liées aux événements du référentiel comme les PR ouvertes, ou les calendriers cron qui doivent vivre aux côtés de votre configuration de flux de travail.                                                                                      |
| [`/loop`](/docs/fr/scheduled-tasks)                                 | La session CLI actuelle                    | L'interrogation rapide pendant qu'une session est ouverte. Les tâches s'arrêtent quand vous commencez une nouvelle conversation ; `--resume` et `--continue` restaurent les tâches non expirées.                                                          |

<Tip>
  Lors de la rédaction de prompts pour les tâches planifiées, soyez explicite sur ce que signifie le succès et ce qu'il faut faire avec les résultats. La tâche s'exécute de manière autonome, elle ne peut donc pas poser de questions de clarification. Par exemple : « Examinez les PR ouvertes étiquetées `needs-review`, laissez des commentaires en ligne sur les problèmes et publiez un résumé dans le canal Slack `#eng-reviews`. »
</Tip>

***

<h3 id="ask-claude-about-its-capabilities">
  Demandez à Claude ses capacités
</h3>

Claude a un accès intégré à sa documentation et peut répondre à des questions sur ses propres fonctionnalités et limitations.

<h4 id="example-questions">
  Exemples de questions
</h4>

```text theme={null}
can Claude Code create pull requests?
```

```text theme={null}
how does Claude Code handle permissions?
```

```text theme={null}
what skills are available?
```

```text theme={null}
how do I use MCP with Claude Code?
```

```text theme={null}
how do I configure Claude Code for Amazon Bedrock?
```

```text theme={null}
what are the limitations of Claude Code?
```

<Note>
  Claude fournit des réponses basées sur la documentation à ces questions. Pour des démonstrations pratiques, exécutez `/powerup` pour des leçons interactives avec des démos animées, ou consultez les sections de flux de travail spécifiques ci-dessus.
</Note>

<Tip>
  Conseils :

  * Claude a toujours accès à la dernière documentation de Claude Code, quelle que soit la version que vous utilisez
  * Posez des questions spécifiques pour obtenir des réponses détaillées
  * Claude peut expliquer les fonctionnalités complexes comme l'intégration MCP, les configurations d'entreprise et les flux de travail avancés
</Tip>

***

<h2 id="resume-previous-conversations">
  Reprendre les conversations précédentes
</h2>

Quand une tâche s'étend sur plusieurs sessions, reprenez là où vous avez laissé au lieu de réexpliquer le contexte. Claude Code enregistre chaque conversation localement.

```bash theme={null}
claude --continue
```

Cela reprend la session la plus récente dans le répertoire actuel ; s'il n'y en a pas encore, il affiche `No conversation found to continue` et se termine. Utilisez `claude --resume` pour choisir dans une liste, ou `/resume` depuis une session en cours. Consultez [Gérer les sessions](/docs/fr/sessions) pour nommer, créer des branches et la référence complète du sélecteur.

<h2 id="run-parallel-sessions-with-worktrees">
  Exécuter des sessions parallèles avec worktrees
</h2>

Travaillez sur une fonctionnalité dans un terminal tandis que Claude corrige un bogue dans un autre, sans que les modifications ne se heurtent. Chaque worktree est un checkout séparé sur sa propre branche.

```bash theme={null}
claude --worktree feature-auth
```

Exécutez la même commande avec un nom différent dans un deuxième terminal pour démarrer une session parallèle isolée. Consultez [Worktrees](/docs/fr/worktrees) pour le nettoyage, `.worktreeinclude` et le support VCS non-git. Pour surveiller les sessions parallèles à partir d'un seul écran au lieu de terminaux séparés, consultez [agents en arrière-plan](/docs/fr/agent-view).

<h2 id="plan-before-editing">
  Planifier avant de modifier
</h2>

Pour les modifications que vous souhaitez examiner avant qu'elles ne touchent le disque, basculez en mode plan. Claude lit les fichiers et propose un plan mais ne fait aucune modification jusqu'à ce que vous approuviez.

```bash theme={null}
claude --permission-mode plan
```

Vous pouvez également appuyer sur `Shift+Tab` pendant une session pour basculer en mode plan. Consultez [Mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) pour le flux d'approbation et la modification du plan dans votre éditeur de texte.

<h2 id="delegate-research-to-subagents">
  Déléguer la recherche à des subagents
</h2>

Explorer une grande base de code remplit votre contexte avec des lectures de fichiers. Déléguez l'exploration pour que seules les conclusions reviennent.

```text theme={null}
use a subagent to investigate how our auth system handles token refresh
```

Le subagent lit les fichiers dans sa propre fenêtre de contexte et rapporte un résumé. Consultez [Subagents](/docs/fr/sub-agents) pour définir des agents personnalisés avec leurs propres outils et prompts.

<h2 id="pipe-claude-into-scripts">
  Canaliser Claude dans des scripts
</h2>

Exécutez Claude de manière non-interactive pour CI, les hooks de pré-commit ou le traitement par lot. Stdin et stdout fonctionnent comme n'importe quel outil Unix.

```bash theme={null}
git log --oneline -20 | claude -p "summarize these recent commits"
```

Consultez [Mode non-interactif](/docs/fr/headless) pour les formats de sortie, les drapeaux de permission et les modèles de fan-out.

<h2 id="next-steps">
  Étapes suivantes
</h2>

<CardGroup cols={2}>
  <Card title="Bonnes pratiques" icon="lightbulb" href="/docs/fr/best-practices">
    Modèles pour tirer le meilleur parti de Claude Code
  </Card>

  <Card title="Gérer les sessions" icon="rotate-left" href="/docs/fr/sessions">
    Reprendre, nommer et créer des branches de conversations
  </Card>

  <Card title="Worktrees" icon="code-branch" href="/docs/fr/worktrees">
    Exécuter des sessions parallèles isolées
  </Card>

  <Card title="Étendre Claude Code" icon="puzzle-piece" href="/docs/fr/features-overview">
    Ajouter des skills, des hooks, MCP, des subagents et des plugins
  </Card>
</CardGroup>
