> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Meilleures pratiques pour Claude Code

> Conseils et modèles pour tirer le meilleur parti de Claude Code, de la configuration de votre environnement à la mise à l'échelle sur plusieurs sessions parallèles.

Claude Code est un environnement de codage agentique. Contrairement à un chatbot qui répond aux questions et attend, Claude Code peut lire vos fichiers, exécuter des commandes, apporter des modifications et travailler de manière autonome sur les problèmes pendant que vous regardez, redirigez ou vous éloignez complètement.

Cela change votre façon de travailler. Au lieu d'écrire du code vous-même et de demander à Claude de le réviser, vous décrivez ce que vous voulez et Claude détermine comment le construire. Claude explore, planifie et implémente.

Mais cette autonomie s'accompagne toujours d'une courbe d'apprentissage. Claude fonctionne dans certaines contraintes que vous devez comprendre.

Ce guide couvre les modèles qui se sont avérés efficaces dans les équipes internes d'Anthropic et pour les ingénieurs utilisant Claude Code sur diverses bases de code, langages et environnements. Pour comprendre comment la boucle agentique fonctionne sous le capot, consultez [Comment fonctionne Claude Code](/docs/fr/how-claude-code-works).

***

La plupart des meilleures pratiques sont basées sur une contrainte : la fenêtre de contexte de Claude se remplit rapidement et les performances se dégradent à mesure qu'elle se remplit.

La fenêtre de contexte de Claude contient l'intégralité de votre conversation, y compris chaque message, chaque fichier que Claude lit et chaque sortie de commande. Cependant, cela peut se remplir rapidement. Une seule session de débogage ou exploration de base de code peut générer et consommer des dizaines de milliers de tokens.

Cela importe car les performances des LLM se dégradent à mesure que le contexte se remplit. Lorsque la fenêtre de contexte est presque pleine, Claude peut commencer à « oublier » les instructions antérieures ou faire plus d'erreurs. La fenêtre de contexte est la ressource la plus importante à gérer. Pour voir comment une session se remplit en pratique, [regardez une démonstration interactive](/docs/fr/context-window) de ce qui se charge au démarrage et ce que chaque lecture de fichier coûte. Suivez l'utilisation du contexte en continu avec une [ligne d'état personnalisée](/docs/fr/statusline), et consultez [Réduire l'utilisation des tokens](/docs/fr/costs#reduce-token-usage) pour des stratégies de réduction de l'utilisation des tokens.

***

<h2 id="give-claude-a-way-to-verify-its-work">
  Donnez à Claude un moyen de vérifier son travail
</h2>

<Tip>
  Donnez à Claude une vérification qu'il peut exécuter : des tests, une compilation, une capture d'écran à comparer. C'est la différence entre une session que vous regardez et une session dont vous vous éloignez.
</Tip>

Claude s'arrête quand le travail semble terminé. Sans une vérification qu'il peut exécuter, « semble terminé » est le seul signal disponible, et vous devenez la boucle de vérification : chaque erreur attend que vous la remarquiez. Donnez à Claude quelque chose qui produit un résultat de réussite ou d'échec, et la boucle se ferme d'elle-même. Claude fait le travail, exécute la vérification, lit le résultat et itère jusqu'à ce que la vérification réussisse.

La vérification est tout ce qui retourne un signal que Claude peut lire dans la conversation : une suite de tests, un code de sortie de compilation, un linter, un script qui compare la sortie à une fixture, ou une [capture d'écran du navigateur](/docs/fr/chrome) comparée à une conception.

| Stratégie                                                              | Avant                                                         | Après                                                                                                                                                                                                                        |
| ---------------------------------------------------------------------- | ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Fournir des critères de vérification**                               | *« implémenter une fonction qui valide les adresses e-mail »* | *« écrire une fonction validateEmail. exemples de cas de test : [user@example.com](mailto:user@example.com) est vrai, invalid est faux, [user@.com](mailto:user@.com) est faux. exécuter les tests après l'implémentation »* |
| **Vérifier les modifications de l'interface utilisateur visuellement** | *« rendre le tableau de bord plus beau »*                     | *« \[coller la capture d'écran] implémenter cette conception. prendre une capture d'écran du résultat et la comparer à l'original. lister les différences et les corriger »*                                                 |
| **Traiter les causes profondes, pas les symptômes**                    | *« la compilation échoue »*                                   | *« la compilation échoue avec cette erreur : \[coller l'erreur]. la corriger et vérifier que la compilation réussit. traiter la cause profonde, ne pas supprimer l'erreur »*                                                 |

Une fois que la vérification existe, décidez à quel point elle contrôle l'arrêt :

* **En un seul message** : demandez à Claude d'exécuter la vérification et d'itérer dans le même message, comme dans le tableau ci-dessus.
* **Sur une session** : définissez la vérification comme une [condition `/goal`](/docs/fr/goal). Un évaluateur séparé la revérifie après chaque tour et Claude continue de travailler jusqu'à ce qu'elle soit satisfaite.
* **Comme une porte déterministe** : un [hook Stop](/docs/fr/hooks#stop) exécute votre vérification en tant que script et bloque la fin du tour jusqu'à ce qu'elle réussisse. Claude Code remplace le hook et termine le tour après 8 blocages consécutifs.
* **Par un deuxième avis** : un [sous-agent de vérification](/docs/fr/sub-agents) ou un [flux de travail dynamique](/docs/fr/workflows) qui vérifie ses propres conclusions a un modèle frais qui essaie de réfuter le résultat, de sorte que l'agent qui fait le travail n'est pas celui qui le note.

Chaque étape échange la configuration pour l'attention. La version avec message fonctionne sur n'importe quelle tâche aujourd'hui. Les versions `/goal` et Stop hook sont ce qui permet à une exécution sans surveillance de se terminer correctement sans vous.

Demandez à Claude de montrer des preuves plutôt que d'affirmer le succès : la sortie du test, la commande qu'il a exécutée et ce qu'elle a retourné, ou une capture d'écran du résultat. Examiner les preuves est plus rapide que de relancer la vérification vous-même, et cela fonctionne pour les sessions que vous n'aviez pas observées.

***

<h2 id="explore-first-then-plan-then-code">
  Explorez d'abord, puis planifiez, puis codez
</h2>

<Tip>
  Séparez la recherche et la planification de l'implémentation pour éviter de résoudre le mauvais problème.
</Tip>

Laisser Claude sauter directement au codage peut produire du code qui résout le mauvais problème. Utilisez [plan mode](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) pour séparer l'exploration de l'exécution.

Le flux de travail recommandé comporte quatre phases :

<Steps>
  <Step title="Explorez">
    Entrez en plan mode. Claude lit les fichiers et répond aux questions sans apporter de modifications.

    ```txt claude (plan mode) theme={null}
    read /src/auth and understand how we handle sessions and login.
    also look at how we manage environment variables for secrets.
    ```
  </Step>

  <Step title="Planifiez">
    Demandez à Claude de créer un plan d'implémentation détaillé.

    ```txt claude (plan mode) theme={null}
    I want to add Google OAuth. What files need to change?
    What's the session flow? Create a plan.
    ```

    Appuyez sur `Ctrl+G` pour ouvrir le plan dans votre éditeur de texte pour une édition directe avant que Claude ne procède.
  </Step>

  <Step title="Implémentez">
    Quittez le plan mode et laissez Claude coder, en vérifiant par rapport à son plan.

    ```txt claude (default mode) theme={null}
    implement the OAuth flow from your plan. write tests for the
    callback handler, run the test suite and fix any failures.
    ```
  </Step>

  <Step title="Validez">
    Demandez à Claude de valider avec un message descriptif et de créer une PR.

    ```txt claude (default mode) theme={null}
    commit with a descriptive message and open a PR
    ```
  </Step>
</Steps>

<Callout>
  Plan mode est utile, mais ajoute également des frais généraux.

  Pour les tâches où la portée est claire et la correction est petite (comme corriger une faute de frappe, ajouter une ligne de journal ou renommer une variable), demandez à Claude de le faire directement.

  La planification est plus utile lorsque vous êtes incertain de l'approche, lorsque la modification modifie plusieurs fichiers ou lorsque vous n'êtes pas familier avec le code en cours de modification. Si vous pouviez décrire le diff en une phrase, ignorez le plan.
</Callout>

***

<h2 id="provide-specific-context-in-your-prompts">
  Fournissez un contexte spécifique dans vos invites
</h2>

<Tip>
  Plus vos instructions sont précises, moins vous aurez besoin de corrections.
</Tip>

Claude peut déduire l'intention, mais il ne peut pas lire dans vos pensées. Référencez des fichiers spécifiques, mentionnez les contraintes et pointez vers des modèles d'exemple.

| Stratégie                                                                                                | Avant                                                          | Après                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Délimitez la tâche.** Spécifiez quel fichier, quel scénario et les préférences de test.                | *« ajouter des tests pour foo.py »*                            | *« écrire un test pour foo.py couvrant le cas limite où l'utilisateur est déconnecté. éviter les mocks. »*                                                                                                                                                                                                                                                                                                                                 |
| **Pointez vers les sources.** Dirigez Claude vers la source qui peut répondre à une question.            | *« pourquoi ExecutionFactory a-t-il une API aussi bizarre ? »* | *« parcourir l'historique git d'ExecutionFactory et résumer comment son API en est venue à être »*                                                                                                                                                                                                                                                                                                                                         |
| **Référencez les modèles existants.** Pointez Claude vers les modèles de votre base de code.             | *« ajouter un widget calendrier »*                             | *« regarder comment les widgets existants sont implémentés sur la page d'accueil pour comprendre les modèles. HotDogWidget.php est un bon exemple. suivre le modèle pour implémenter un nouveau widget calendrier qui permet à l'utilisateur de sélectionner un mois et de paginer vers l'avant/l'arrière pour choisir une année. construire à partir de zéro sans bibliothèques autres que celles déjà utilisées dans la base de code. »* |
| **Décrivez le symptôme.** Fournissez le symptôme, l'emplacement probable et ce que « corrigé » signifie. | *« corriger le bug de connexion »*                             | *« les utilisateurs signalent que la connexion échoue après l'expiration de la session. vérifier le flux d'authentification dans src/auth/, en particulier l'actualisation des tokens. écrire un test défaillant qui reproduit le problème, puis le corriger »*                                                                                                                                                                            |

Les invites vagues peuvent être utiles lorsque vous explorez et que vous pouvez vous permettre de corriger la trajectoire. Une invite comme `« qu'amélioreriez-vous dans ce fichier ? »` peut révéler des choses auxquelles vous n'auriez pas pensé à demander.

<h3 id="provide-rich-content">
  Fournissez du contenu riche
</h3>

<Tip>
  Utilisez `@` pour référencer des fichiers, coller des captures d'écran/images ou canaliser les données directement.
</Tip>

Vous pouvez fournir des données riches à Claude de plusieurs façons :

* **Référencez les fichiers avec `@`** au lieu de décrire où le code se trouve. Claude lit le fichier avant de répondre.
* **Collez les images directement**. Copiez/collez ou glissez-déposez les images dans l'invite.
* **Donnez des URL** pour la documentation et les références API. Utilisez `/permissions` pour autoriser les domaines fréquemment utilisés.
* **Canalisez les données** en exécutant `cat error.log | claude` pour envoyer le contenu du fichier directement.
* **Laissez Claude récupérer ce dont il a besoin**. Dites à Claude de tirer le contexte lui-même en utilisant des commandes Bash, des outils MCP ou en lisant des fichiers.

***

<h2 id="configure-your-environment">
  Configurez votre environnement
</h2>

Quelques étapes de configuration rendent Claude Code beaucoup plus efficace dans toutes vos sessions. Pour un aperçu complet des fonctionnalités d'extension et du moment d'utiliser chacune, consultez [Étendre Claude Code](/docs/fr/features-overview).

<h3 id="write-an-effective-claude-md">
  Écrivez un CLAUDE.md efficace
</h3>

<Tip>
  Exécutez `/init` pour générer un fichier CLAUDE.md de démarrage basé sur la structure actuelle de votre projet, puis affinez au fil du temps.
</Tip>

CLAUDE.md est un fichier spécial que Claude lit au début de chaque conversation. Incluez des commandes Bash, le style de code et les règles de flux de travail. Cela donne à Claude un contexte persistant qu'il ne peut pas déduire du code seul.

La commande `/init` analyse votre base de code pour détecter les systèmes de construction, les frameworks de test et les modèles de code, vous donnant une base solide à affiner.

Il n'y a pas de format requis pour les fichiers CLAUDE.md, mais gardez-le court et lisible par l'homme. Par exemple :

```markdown CLAUDE.md theme={null}
# Code style
- Use ES modules (import/export) syntax, not CommonJS (require)
- Destructure imports when possible (eg. import { foo } from 'bar')

# Workflow
- Be sure to typecheck when you're done making a series of code changes
- Prefer running single tests, and not the whole test suite, for performance
```

CLAUDE.md est chargé à chaque session, donc incluez uniquement les choses qui s'appliquent largement. Pour les connaissances de domaine ou les flux de travail qui ne sont pertinents que parfois, utilisez [skills](/docs/fr/skills) à la place. Claude les charge à la demande sans surcharger chaque conversation.

Gardez-le concis. Pour chaque ligne, demandez-vous : *« Supprimer cela causerait-il à Claude de faire des erreurs ? »* Si non, supprimez-le. Les fichiers CLAUDE.md gonflés font que Claude ignore vos instructions réelles !

| ✅ Inclure                                                                     | ❌ Exclure                                                   |
| ----------------------------------------------------------------------------- | ----------------------------------------------------------- |
| Commandes Bash que Claude ne peut pas deviner                                 | Tout ce que Claude peut déduire en lisant le code           |
| Règles de style de code qui diffèrent des valeurs par défaut                  | Conventions de langage standard que Claude connaît déjà     |
| Instructions de test et exécuteurs de test préférés                           | Documentation API détaillée (lien vers les docs à la place) |
| Étiquette du référentiel (nommage des branches, conventions PR)               | Informations qui changent fréquemment                       |
| Décisions architecturales spécifiques à votre projet                          | Explications longues ou tutoriels                           |
| Particularités de l'environnement de développement (variables d'env requises) | Pratiques évidentes comme « écrire du code propre »         |
| Pièges courants ou comportements non évidents                                 | Descriptions du code fichier par fichier                    |

Si Claude continue à faire quelque chose que vous ne voulez pas malgré une règle contre cela, le fichier est probablement trop long et la règle se perd. Si Claude vous pose des questions qui sont répondues dans CLAUDE.md, la formulation pourrait être ambiguë. Traitez CLAUDE.md comme du code : révisez-le lorsque les choses vont mal, élaguez-le régulièrement et testez les modifications en observant si le comportement de Claude change réellement.

Vous pouvez affiner les instructions en ajoutant de l'emphase (par exemple, « IMPORTANT » ou « VOUS DEVEZ ») pour améliorer l'adhérence. Vérifiez CLAUDE.md dans git pour que votre équipe puisse contribuer. Le fichier augmente en valeur au fil du temps.

Les fichiers CLAUDE.md peuvent importer des fichiers supplémentaires en utilisant la syntaxe `@path/to/import` :

```markdown CLAUDE.md theme={null}
See @README.md for project overview and @package.json for available npm commands.

# Additional Instructions
- Git workflow: @docs/git-instructions.md
- Personal overrides: @~/.claude/my-project-instructions.md
```

Vous pouvez placer les fichiers CLAUDE.md dans plusieurs emplacements :

* **Dossier personnel (`~/.claude/CLAUDE.md`)** : s'applique à toutes les sessions Claude
* **Racine du projet (`./CLAUDE.md`)** : vérifier dans git pour partager avec votre équipe
* **Racine du projet (`./CLAUDE.local.md`)** : notes personnelles spécifiques au projet ; ajoutez ce fichier à votre `.gitignore` pour qu'il ne soit pas partagé avec votre équipe
* **Répertoires parents** : utile pour les monorepos où `root/CLAUDE.md` et `root/foo/CLAUDE.md` sont extraits automatiquement
* **Répertoires enfants** : Claude extrait les fichiers CLAUDE.md enfants à la demande lorsqu'il travaille avec des fichiers dans ces répertoires

<h3 id="configure-permissions">
  Configurez les permissions
</h3>

<Tip>
  Utilisez [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) pour laisser un classificateur gérer les approbations, `/permissions` pour autoriser les commandes spécifiques, ou `/sandbox` pour l'isolation au niveau du système d'exploitation. Chacun réduit les interruptions tout en vous gardant en contrôle.
</Tip>

Par défaut, Claude Code demande une permission pour les actions qui pourraient modifier votre système : écritures de fichiers, commandes Bash, outils MCP, etc. C'est sûr mais fastidieux. Après la dixième approbation, vous ne révisez vraiment plus, vous cliquez simplement. Il y a trois façons de réduire ces interruptions :

* **Mode auto** : un modèle classificateur séparé examine les commandes et bloque uniquement ce qui semble risqué : escalade de portée, infrastructure inconnue ou actions motivées par du contenu hostile. Meilleur lorsque vous faites confiance à la direction générale d'une tâche mais que vous ne voulez pas cliquer à chaque étape
* **Listes blanches de permissions** : permettre des outils spécifiques que vous savez être sûrs, comme `npm run lint` ou `git commit`
* **Sandboxing** : activer l'isolation au niveau du système d'exploitation qui restreint l'accès au système de fichiers et au réseau, permettant à Claude de travailler plus librement dans des limites définies

Lisez plus sur [les modes de permission](/docs/fr/permission-modes), [les règles de permission](/docs/fr/permissions) et [le sandboxing](/docs/fr/sandboxing).

<h3 id="use-cli-tools">
  Utilisez les outils CLI
</h3>

<Tip>
  Dites à Claude Code d'utiliser les outils CLI comme `gh`, `aws`, `gcloud` et `sentry-cli` lors de l'interaction avec les services externes.
</Tip>

Les outils CLI sont le moyen le plus efficace en contexte d'interagir avec les services externes. Si vous utilisez GitHub, installez le CLI `gh`. Claude sait comment l'utiliser pour créer des problèmes, ouvrir des demandes de tirage et lire les commentaires. Sans `gh`, Claude peut toujours utiliser l'API GitHub, mais les demandes non authentifiées atteignent souvent les limites de débit.

Claude est également efficace pour apprendre les outils CLI qu'il ne connaît pas déjà. Essayez des invites comme `Utilisez 'foo-cli-tool --help' pour en savoir plus sur l'outil foo, puis utilisez-le pour résoudre A, B, C.`

<h3 id="connect-mcp-servers">
  Connectez les serveurs MCP
</h3>

<Tip>
  Exécutez `claude mcp add` pour connecter les outils externes comme Notion, Figma ou votre base de données.
</Tip>

Avec les [serveurs MCP](/docs/fr/mcp), vous pouvez demander à Claude d'implémenter des fonctionnalités à partir de suivi de problèmes, interroger des bases de données, analyser les données de surveillance, intégrer les conceptions de Figma et automatiser les flux de travail.

<h3 id="set-up-hooks">
  Configurez les hooks
</h3>

<Tip>
  Utilisez les hooks pour les actions qui doivent se produire à chaque fois sans exception.
</Tip>

Les [hooks](/docs/fr/hooks-guide) exécutent automatiquement les scripts à des points spécifiques du flux de travail de Claude. Contrairement aux instructions CLAUDE.md qui sont consultatives, les hooks sont déterministes et garantissent que l'action se produit.

Claude peut écrire des hooks pour vous. Essayez des invites comme *« Écrire un hook qui exécute eslint après chaque édition de fichier »* ou *« Écrire un hook qui bloque les écritures dans le dossier migrations. »* Modifiez `.claude/settings.json` directement pour configurer les hooks à la main, et exécutez `/hooks` pour parcourir ce qui est configuré.

<h3 id="create-skills">
  Créez des skills
</h3>

<Tip>
  Créez des fichiers `SKILL.md` dans `.claude/skills/` pour donner à Claude des connaissances de domaine et des flux de travail réutilisables.
</Tip>

Les [skills](/docs/fr/skills) étendent les connaissances de Claude avec des informations spécifiques à votre projet, équipe ou domaine. Claude les applique automatiquement lorsqu'elles sont pertinentes, ou vous pouvez les invoquer directement avec `/skill-name`.

Créez une skill en ajoutant un répertoire avec un `SKILL.md` à `.claude/skills/` :

```markdown .claude/skills/api-conventions/SKILL.md theme={null}
---
name: api-conventions
description: REST API design conventions for our services
---
# API Conventions
- Use kebab-case for URL paths
- Use camelCase for JSON properties
- Always include pagination for list endpoints
- Version APIs in the URL path (/v1/, /v2/)
```

Les skills peuvent également définir des flux de travail réutilisables que vous invoquez directement :

```markdown .claude/skills/fix-issue/SKILL.md theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---
Analyze and fix the GitHub issue: $ARGUMENTS.

1. Use `gh issue view` to get the issue details
2. Understand the problem described in the issue
3. Search the codebase for relevant files
4. Implement the necessary changes to fix the issue
5. Write and run tests to verify the fix
6. Ensure code passes linting and type checking
7. Create a descriptive commit message
8. Push and create a PR
```

Exécutez `/fix-issue 1234` pour l'invoquer. Utilisez `disable-model-invocation: true` pour les flux de travail avec des effets secondaires que vous souhaitez déclencher manuellement.

<h3 id="create-custom-subagents">
  Créez des subagents personnalisés
</h3>

<Tip>
  Définissez des assistants spécialisés dans `.claude/agents/` que Claude peut déléguer pour les tâches isolées.
</Tip>

Les [subagents](/docs/fr/sub-agents) s'exécutent dans leur propre contexte avec leur propre ensemble d'outils autorisés. Ils sont utiles pour les tâches qui lisent de nombreux fichiers ou qui ont besoin d'une attention spécialisée sans encombrer votre conversation principale.

```markdown .claude/agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code for security vulnerabilities
tools: Read, Grep, Glob, Bash
model: opus
---
You are a senior security engineer. Review code for:
- Injection vulnerabilities (SQL, XSS, command injection)
- Authentication and authorization flaws
- Secrets or credentials in code
- Insecure data handling

Provide specific line references and suggested fixes.
```

Dites à Claude d'utiliser les subagents explicitement : *« Utilisez un subagent pour réviser ce code pour les problèmes de sécurité. »*

<h3 id="install-plugins">
  Installez les plugins
</h3>

<Tip>
  Exécutez `/plugin` pour parcourir la marketplace. Les plugins ajoutent des skills, des outils et des intégrations sans configuration.
</Tip>

Les [plugins](/docs/fr/plugins) regroupent les skills, les hooks, les subagents et les serveurs MCP dans une seule unité installable de la communauté et d'Anthropic. Si vous travaillez avec un langage typé, installez un [plugin d'intelligence de code](/docs/fr/discover-plugins#code-intelligence) pour donner à Claude une navigation de symboles précise et une détection d'erreur automatique après les éditions.

Pour des conseils sur le choix entre les skills, les subagents, les hooks et MCP, consultez [Étendre Claude Code](/docs/fr/features-overview#match-features-to-your-goal).

***

<h2 id="communicate-effectively">
  Communiquez efficacement
</h2>

La façon dont vous communiquez avec Claude Code a un impact significatif sur la qualité des résultats.

<h3 id="ask-codebase-questions">
  Posez des questions sur la base de code
</h3>

<Tip>
  Posez à Claude les questions que vous poseriez à un ingénieur senior.
</Tip>

Lors de l'intégration à une nouvelle base de code, utilisez Claude Code pour l'apprentissage et l'exploration. Vous pouvez poser à Claude les mêmes types de questions que vous poseriez à un autre ingénieur :

* Comment fonctionne la journalisation ?
* Comment créer un nouveau point de terminaison API ?
* Que fait `async move { ... }` à la ligne 134 de `foo.rs` ?
* Quels cas limites `CustomerOnboardingFlowImpl` gère-t-il ?
* Pourquoi ce code appelle-t-il `foo()` au lieu de `bar()` à la ligne 333 ?

Utiliser Claude Code de cette façon est un flux de travail d'intégration efficace, améliorant le temps de montée en charge et réduisant la charge sur les autres ingénieurs. Aucune invite spéciale requise : posez les questions directement.

<h3 id="let-claude-interview-you">
  Laissez Claude vous interviewer
</h3>

<Tip>
  Pour les fonctionnalités plus grandes, laissez Claude vous interviewer d'abord. Commencez par une invite minimale et demandez à Claude de vous interviewer en utilisant l'outil `AskUserQuestion`.
</Tip>

Claude pose des questions sur les choses que vous n'auriez peut-être pas considérées, y compris l'implémentation technique, l'interface utilisateur/UX, les cas limites et les compromis.

```text theme={null}
I want to build [brief description]. Interview me in detail using the AskUserQuestion tool.

Ask about technical implementation, UI/UX, edge cases, concerns, and tradeoffs. Don't ask obvious questions, dig into the hard parts I might not have considered.

Keep interviewing until we've covered everything, then write a complete spec to SPEC.md.
```

Une fois la spécification complète, démarrez une nouvelle session pour l'exécuter. La nouvelle session a un contexte propre entièrement axé sur l'implémentation, et vous avez une spécification écrite à référencer.

Les spécifications les plus utiles sont autonomes : elles nomment les fichiers et les interfaces impliqués, énoncent ce qui est hors de portée, et se terminent par une étape de vérification de bout en bout qui prouve que la fonctionnalité fonctionne. Le temps consacré à rendre la spécification précise rapporte plus que le temps passé à regarder l'implémentation.

***

<h2 id="manage-your-session">
  Gérez votre session
</h2>

Les conversations sont persistantes et réversibles. Utilisez cela à votre avantage !

<h3 id="course-correct-early-and-often">
  Corrigez la trajectoire tôt et souvent
</h3>

<Tip>
  Corrigez Claude dès que vous remarquez qu'il s'écarte de la bonne voie.
</Tip>

Les meilleurs résultats proviennent de boucles de rétroaction serrées. Bien que Claude résolve occasionnellement les problèmes parfaitement à la première tentative, le corriger rapidement produit généralement de meilleures solutions plus rapidement.

* **`Esc`** : arrêtez Claude en pleine action avec la touche `Esc`. Le contexte est préservé, vous pouvez donc rediriger.
* **`Esc + Esc` ou `/rewind`** : appuyez sur `Esc` deux fois ou exécutez `/rewind` pour ouvrir le menu de rembobinage et restaurer la conversation et l'état du code précédents, ou résumer à partir d'un message sélectionné.
* **`« Annuler cela »`** : demandez à Claude d'annuler ses modifications.
* **`/clear`** : réinitialiser le contexte entre les tâches non liées. Les sessions longues avec un contexte non pertinent peuvent réduire les performances.

Si vous avez corrigé Claude plus de deux fois sur le même problème dans une session, le contexte est encombré d'approches échouées. Exécutez `/clear` et recommencez avec une invite plus spécifique qui incorpore ce que vous avez appris. Une session propre avec une meilleure invite surpasse presque toujours une session longue avec des corrections accumulées.

<h3 id="manage-context-aggressively">
  Gérez le contexte agressivement
</h3>

<Tip>
  Exécutez `/clear` entre les tâches non liées pour réinitialiser le contexte.
</Tip>

Claude Code compacte automatiquement l'historique de conversation lorsque vous approchez des limites de contexte, ce qui préserve le code et les décisions importants tout en libérant de l'espace.

Pendant les sessions longues, la fenêtre de contexte de Claude peut se remplir de conversations non pertinentes, de contenu de fichiers et de commandes. Cela peut réduire les performances et parfois distraire Claude.

* Utilisez `/clear` fréquemment entre les tâches pour réinitialiser complètement la fenêtre de contexte
* Lorsque le compactage automatique se déclenche, Claude résume ce qui importe le plus, y compris les modèles de code, les états de fichiers et les décisions clés
* Pour plus de contrôle, exécutez `/compact <instructions>`, comme `/compact Focus on the API changes`
* Pour compacter uniquement une partie de la conversation, utilisez `Esc + Esc` ou `/rewind`, sélectionnez un point de contrôle de message et choisissez **Summarize from here** ou **Summarize up to here**. Le premier condense les messages à partir de ce point tout en gardant le contexte antérieur intact ; le second condense les messages antérieurs tout en gardant les messages récents en intégralité. Consultez [Restore vs. summarize](/docs/fr/checkpointing#restore-vs-summarize).
* Personnalisez le comportement de compactage dans CLAUDE.md avec des instructions comme `« Lors du compactage, toujours préserver la liste complète des fichiers modifiés et toutes les commandes de test »` pour assurer que le contexte critique survit à la résumé
* Pour les questions rapides qui n'ont pas besoin de rester en contexte, utilisez [`/btw`](/docs/fr/interactive-mode#side-questions-with-%2Fbtw). La réponse apparaît dans une superposition rejetable et n'entre jamais dans l'historique de conversation, vous pouvez donc vérifier un détail sans augmenter le contexte.

<h3 id="use-subagents-for-investigation">
  Utilisez les subagents pour l'investigation
</h3>

<Tip>
  Déléguez la recherche avec `« utiliser les subagents pour enquêter sur X »`. Ils explorent dans un contexte séparé, gardant votre conversation principale propre pour l'implémentation.
</Tip>

Puisque le contexte est votre contrainte fondamentale, les subagents sont l'un des outils les plus puissants disponibles. Lorsque Claude enquête sur une base de code, il lit de nombreux fichiers, qui consomment tous votre contexte. Les subagents s'exécutent dans des fenêtres de contexte séparées et rapportent les résumés :

```text theme={null}
Use subagents to investigate how our authentication system handles token
refresh, and whether we have any existing OAuth utilities I should reuse.
```

Le subagent explore la base de code, lit les fichiers pertinents et rapporte les résultats, tout sans encombrer votre conversation principale.

Vous pouvez également utiliser les subagents pour la vérification après que Claude implémente quelque chose :

```text theme={null}
use a subagent to review this code for edge cases
```

<h3 id="rewind-with-checkpoints">
  Rembobinez avec des points de contrôle
</h3>

<Tip>
  Chaque invite que vous envoyez crée un point de contrôle. Vous pouvez restaurer la conversation, le code ou les deux à n'importe quel point de contrôle précédent.
</Tip>

Claude crée automatiquement des instantanés de fichiers avant chaque modification afin qu'un point de contrôle puisse les restaurer. Appuyez deux fois sur `Escape` ou exécutez `/rewind` pour ouvrir le menu de rembobinage. Vous pouvez restaurer la conversation uniquement, restaurer le code uniquement, restaurer les deux ou résumer à partir d'un message sélectionné. Consultez [Checkpointing](/docs/fr/checkpointing) pour plus de détails.

Au lieu de planifier soigneusement chaque mouvement, vous pouvez dire à Claude d'essayer quelque chose de risqué. Si cela ne fonctionne pas, rembobinez et essayez une approche différente. Les points de contrôle persistent entre les sessions, vous pouvez donc fermer votre terminal et toujours rembobiner plus tard.

<Warning>
  Les points de contrôle ne suivent que les modifications apportées par les outils d'édition de fichiers de Claude. Les modifications apportées par des commandes Bash ou des processus externes ne sont pas capturées. Ce n'est pas un remplacement pour git.
</Warning>

<h3 id="resume-conversations">
  Reprenez les conversations
</h3>

<Tip>
  Nommez les sessions avec `/rename` et traitez-les comme des branches : chaque flux de travail obtient son propre contexte persistant.
</Tip>

Claude Code enregistre les conversations localement, donc lorsqu'une tâche s'étend sur plusieurs sessions, vous n'avez pas à réexpliquer le contexte. Exécutez `claude --continue` pour reprendre la session la plus récente, ou `claude --resume` pour choisir parmi une liste. Donnez aux sessions des noms descriptifs comme `oauth-migration` afin de pouvoir les trouver plus tard. Consultez [Manage sessions](/docs/fr/sessions) pour l'ensemble complet des contrôles de reprise, de branchement et de nommage.

***

<h2 id="automate-and-scale">
  Automatisez et mettez à l'échelle
</h2>

Une fois que vous êtes efficace avec un Claude, multipliez votre production avec des sessions parallèles, le mode non interactif et les modèles de fan-out.

Tout ce qui précède suppose un humain, un Claude et une conversation. Mais Claude Code se met à l'échelle horizontalement. Les techniques de cette section montrent comment vous pouvez en faire plus.

<h3 id="run-non-interactive-mode">
  Exécutez le mode non interactif
</h3>

<Tip>
  Utilisez `claude -p "prompt"` dans CI, les hooks de pré-commit ou les scripts. Ajoutez `--output-format stream-json --verbose` pour la sortie JSON en streaming.
</Tip>

Avec `claude -p "your prompt"`, vous pouvez exécuter Claude de manière non interactive, sans une session interactive. L'exécution crée toujours une session reprise à moins que vous ne transmettiez `--no-session-persistence`. Le [mode non interactif](/docs/fr/headless) est la façon dont vous intégrez Claude dans les pipelines CI, les hooks de pré-commit ou tout flux de travail automatisé. Les formats de sortie vous permettent d'analyser les résultats par programmation : texte brut, JSON ou JSON en streaming.

```bash theme={null}
# One-off queries
claude -p "Explain what this project does"

# Structured output for scripts
claude -p "List all API endpoints" --output-format json

# Streaming for real-time processing
claude -p "Analyze this log file" --output-format stream-json --verbose
```

<h3 id="run-multiple-claude-sessions">
  Exécutez plusieurs sessions Claude
</h3>

<Tip>
  Exécutez plusieurs sessions Claude en parallèle pour accélérer le développement, exécuter des expériences isolées ou démarrer des flux de travail complexes.
</Tip>

Choisissez l'approche parallèle qui correspond au niveau de coordination que vous souhaitez faire vous-même :

* [Worktrees](/docs/fr/worktrees) : exécutez des sessions CLI séparées dans des checkouts git isolés afin que les modifications ne se heurtent pas
* [Application de bureau](/docs/fr/desktop#work-in-parallel-with-sessions) : gérez visuellement plusieurs sessions locales, chacune dans son propre worktree
* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : exécutez des sessions sur l'infrastructure cloud gérée par Anthropic dans des VM isolées
* [Équipes d'agents](/docs/fr/agent-teams) : coordination automatisée de plusieurs sessions avec des tâches partagées, la messagerie et un chef d'équipe

Au-delà de la parallélisation du travail, plusieurs sessions permettent des flux de travail axés sur la qualité. Un contexte frais améliore la révision de code puisque Claude ne sera pas biaisé vers le code qu'il vient d'écrire.

Par exemple, utilisez un modèle Writer/Reviewer :

| Session A (Writer)                                                      | Session B (Reviewer)                                                                                                                                                     |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Implement a rate limiter for our API endpoints`                        |                                                                                                                                                                          |
|                                                                         | `Review the rate limiter implementation in @src/middleware/rateLimiter.ts. Look for edge cases, race conditions, and consistency with our existing middleware patterns.` |
| `Here's the review feedback: [Session B output]. Address these issues.` |                                                                                                                                                                          |

Vous pouvez faire quelque chose de similaire avec les tests : avoir un Claude écrire des tests, puis un autre écrire du code pour les réussir.

<h3 id="fan-out-across-files">
  Fan out sur les fichiers
</h3>

<Tip>
  Bouclez à travers les tâches en appelant `claude -p` pour chacune. Utilisez `--allowedTools` pour délimiter les permissions pour les opérations par lot.
</Tip>

Pour les migrations ou analyses à grande échelle, vous pouvez distribuer le travail sur de nombreuses invocations Claude parallèles :

<Steps>
  <Step title="Générez une liste de tâches">
    Demandez à Claude de lister tous les fichiers qui doivent être migrés (par exemple, `list all 2,000 Python files that need migrating`)
  </Step>

  <Step title="Écrivez un script pour boucler à travers la liste">
    ```bash theme={null}
    for file in $(cat files.txt); do
      claude -p "Migrate $file from React to Vue. Return OK or FAIL." \
        --allowedTools "Edit,Bash(git commit *)"
    done
    ```
  </Step>

  <Step title="Testez sur quelques fichiers, puis exécutez à l'échelle">
    Affinez votre invite en fonction de ce qui se passe mal avec les 2-3 premiers fichiers, puis exécutez sur l'ensemble complet. L'indicateur `--allowedTools` restreint ce que Claude peut faire, ce qui importe lorsque vous exécutez sans surveillance.
  </Step>
</Steps>

Vous pouvez également intégrer Claude dans les pipelines de données/traitement existants :

```bash theme={null}
claude -p "<your prompt>" --output-format json | your_command
```

Utilisez `--verbose` pour le débogage pendant le développement, et désactivez-le en production.

<h3 id="run-autonomously-with-auto-mode">
  Exécutez de manière autonome avec le mode auto
</h3>

Pour une exécution ininterrompue avec des vérifications de sécurité en arrière-plan, utilisez le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode). Un modèle classificateur examine les commandes avant qu'elles ne s'exécutent, bloquant l'escalade de portée, l'infrastructure inconnue et les actions motivées par du contenu hostile tout en laissant le travail de routine se dérouler sans invites.

```bash theme={null}
claude --permission-mode auto -p "fix all lint errors"
```

Pour les exécutions non interactives avec l'indicateur `-p`, le mode auto abandonne si le classificateur bloque à plusieurs reprises les actions, puisqu'il n'y a pas d'utilisateur pour se replier. Consultez [quand le mode auto se replie](/docs/fr/permission-modes#when-auto-mode-falls-back) pour les seuils.

<h3 id="add-an-adversarial-review-step">
  Ajoutez une étape d'examen contradictoire
</h3>

<Tip>
  Avant de considérer une tâche comme terminée, demandez à un sous-agent d'examiner la diff dans un contexte frais et de signaler les lacunes.
</Tip>

Plus Claude travaille sans surveillance, plus une vérification indépendante importe avant de compter le travail comme terminé. Un examinateur exécutant dans un contexte [sous-agent](/docs/fr/sub-agents) frais ne voit que la diff et les critères que vous lui donnez, pas le raisonnement qui a produit le changement, donc il évalue le résultat selon ses propres termes.

Pour une vérification de la correction, exécutez la compétence [`/code-review`](/docs/fr/commands) fournie, qui examine la diff actuelle pour les bogues dans un sous-agent frais et retourne les résultats à la session. Pour vérifier la diff par rapport à votre plan à la place, écrivez vous-même l'invite d'examen. Nommez le travail à vérifier, le plan à vérifier et ce qui compte comme une constatation :

```text theme={null}
Use a subagent to review the rate limiter diff against PLAN.md. Check that
every requirement is implemented, the listed edge cases have tests, and
nothing outside the task's scope changed. Report gaps, not style preferences.
```

Parce que l'examinateur s'exécute en tant que sous-agent, la session d'implémentation reçoit les lacunes directement et peut les corriger et les réexaminer sans que vous copiiez les résultats entre les fenêtres. Pour les exécutions autonomes plus longues, une [équipe d'agents](/docs/fr/agent-teams) peut maintenir cette boucle en marche sur de nombreuses tâches pendant que vous vérifiez les résultats enregistrés.

<Callout>
  Un examinateur invité à trouver des lacunes signalera généralement certaines, même lorsque le travail est solide, car c'est ce qu'on lui a demandé de faire. Poursuivre chaque constatation mène à une sur-ingénierie : couches d'abstraction supplémentaires, code défensif et tests pour des cas qui ne peuvent pas se produire. Dites à l'examinateur de signaler uniquement les lacunes qui affectent la correction ou les exigences énoncées, et traitez le reste comme optionnel.
</Callout>

***

<h2 id="avoid-common-failure-patterns">
  Évitez les modèles d'échec courants
</h2>

Ce sont des erreurs courantes. Les reconnaître tôt économise du temps :

* **La session fourre-tout.** Vous commencez par une tâche, puis demandez à Claude quelque chose d'autre, puis revenez à la première tâche. Le contexte est plein d'informations non pertinentes.
  > **Correction** : `/clear` entre les tâches non liées.
* **Corriger encore et encore.** Claude fait quelque chose de mal, vous le corrigez, c'est toujours mal, vous corrigez à nouveau. Le contexte est pollué par des approches échouées.
  > **Correction** : Après deux corrections échouées, `/clear` et écrivez une meilleure invite initiale incorporant ce que vous avez appris.
* **Le CLAUDE.md sur-spécifié.** Si votre CLAUDE.md est trop long, Claude ignore la moitié car les règles importantes se perdent dans le bruit.
  > **Correction** : Élaguez impitoyablement. Si Claude fait déjà quelque chose correctement sans l'instruction, supprimez-le ou convertissez-le en hook.
* **L'écart de confiance-puis-vérification.** Claude produit une implémentation plausible qui ne gère pas les cas limites.
  > **Correction** : Fournissez toujours une vérification (tests, scripts, captures d'écran). Si vous ne pouvez pas la vérifier, ne la déployez pas.
* **L'exploration infinie.** Vous demandez à Claude d'« enquêter » sur quelque chose sans le délimiter. Claude lit des centaines de fichiers, remplissant le contexte.
  > **Correction** : Délimitez les enquêtes étroitement ou utilisez les subagents pour que l'exploration ne consomme pas votre contexte principal.

***

<h2 id="develop-your-intuition">
  Développez votre intuition
</h2>

Les modèles de ce guide ne sont pas gravés dans le marbre. Ce sont des points de départ qui fonctionnent bien en général, mais pourraient ne pas être optimaux pour chaque situation.

Parfois, vous *devriez* laisser le contexte s'accumuler parce que vous êtes profondément dans un problème complexe et l'historique est précieux. Parfois, vous devriez ignorer la planification et laisser Claude le découvrir parce que la tâche est exploratoire. Parfois, une invite vague est exactement ce qu'il faut parce que vous voulez voir comment Claude interprète le problème avant de le contraindre.

Faites attention à ce qui fonctionne. Lorsque Claude produit une excellente sortie, remarquez ce que vous avez fait : la structure de l'invite, le contexte que vous avez fourni, le mode dans lequel vous étiez. Lorsque Claude lutte, demandez-vous pourquoi. Le contexte était-il trop bruyant ? L'invite trop vague ? La tâche trop grande pour une seule passe ?

Au fil du temps, vous développerez une intuition qu'aucun guide ne peut capturer. Vous saurez quand être spécifique et quand être ouvert, quand planifier et quand explorer, quand effacer le contexte et quand le laisser s'accumuler.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Comment fonctionne Claude Code](/docs/fr/how-claude-code-works) : la boucle agentique, les outils et la gestion du contexte
* [Étendre Claude Code](/docs/fr/features-overview) : skills, hooks, MCP, subagents et plugins
* [Flux de travail courants](/docs/fr/common-workflows) : recettes étape par étape pour le débogage, les tests, les PR et plus
* [CLAUDE.md](/docs/fr/memory) : stocker les conventions de projet et le contexte persistant
