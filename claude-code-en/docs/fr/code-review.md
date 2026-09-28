> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Révision de code

> Configurez des révisions de PR automatisées qui détectent les erreurs logiques, les vulnérabilités de sécurité et les régressions en utilisant l'analyse multi-agents de votre base de code complète

<Note>
  Code Review est en aperçu de recherche, disponible pour les abonnements [Team et Enterprise](https://claude.ai/admin-settings/claude-code). Il n'est pas disponible pour les organisations avec [Zero Data Retention](/docs/fr/zero-data-retention) activé. Sur les autres plans, vous pouvez toujours [examiner une diff localement](#review-a-diff-locally) avec la commande `/code-review`.
</Note>

Code Review analyse vos pull requests GitHub et publie les résultats sous forme de commentaires en ligne sur les lignes de code où il a trouvé des problèmes. Une flotte d'agents spécialisés examine les modifications de code dans le contexte de votre base de code complète, en recherchant les erreurs logiques, les vulnérabilités de sécurité, les cas limites cassés et les régressions subtiles.

Les résultats sont étiquetés par gravité et n'approuvent ni ne bloquent votre PR, de sorte que les flux de travail d'examen existants restent intacts. Vous pouvez affiner ce que Claude signale en ajoutant un fichier `CLAUDE.md` ou `REVIEW.md` à votre référentiel.

Pour exécuter Claude dans votre propre infrastructure CI au lieu de ce service géré, consultez [GitHub Actions](/docs/fr/github-actions) ou [GitLab CI/CD](/docs/fr/gitlab-ci-cd). Pour les référentiels sur une instance GitHub auto-hébergée, consultez [GitHub Enterprise Server](/docs/fr/github-enterprise-server).

Cette page couvre :

* [Comment fonctionnent les révisions](#how-reviews-work)
* [Configuration](#set-up-code-review)
* [Déclenchement manuel des révisions](#manually-trigger-reviews) avec `@claude review` et `@claude review always`
* [Personnalisation des révisions](#customize-reviews) avec `CLAUDE.md` et `REVIEW.md`
* [Tarification](#pricing)
* [Dépannage](#troubleshooting) des exécutions échouées et des commentaires manquants
* [Révision d'une diff localement](#review-a-diff-locally) avec la commande `/code-review`

<h2 id="how-reviews-work">
  Comment fonctionnent les révisions
</h2>

Une fois qu'un administrateur [active Code Review](#set-up-code-review) pour votre organisation, les révisions se déclenchent à l'ouverture d'une PR, à chaque push, ou sur demande manuelle, selon le comportement configuré du référentiel. Commenter `@claude review` [démarre une révision sur une PR](#manually-trigger-reviews) dans n'importe quel mode.

Lorsqu'une révision s'exécute, plusieurs agents analysent le diff et le code environnant en parallèle sur l'infrastructure Anthropic. Chaque agent recherche une classe de problème différente, puis une étape de vérification vérifie les candidats par rapport au comportement réel du code pour filtrer les faux positifs. Les résultats sont dédupliqués, classés par gravité et publiés sous forme de commentaires en ligne sur les lignes spécifiques où les problèmes ont été trouvés, avec un résumé dans le corps de la révision. Si aucun problème n'est trouvé, Code Review met à jour la vérification GitHub pour montrer qu'aucun problème n'a été détecté. Claude peut également publier un court commentaire de confirmation sur la PR.

Les révisions s'adaptent en coût à la taille et à la complexité de la PR, se complétant en moyenne en 20 minutes. Les administrateurs peuvent surveiller l'activité de révision et les dépenses via le [tableau de bord analytique](#view-usage).

<h3 id="severity-levels">
  Niveaux de gravité
</h3>

Chaque résultat est étiqueté avec un niveau de gravité :

| Marqueur | Gravité     | Signification                                                                  |
| :------- | :---------- | :----------------------------------------------------------------------------- |
| 🔴       | Important   | Un bug qui devrait être corrigé avant la fusion                                |
| 🟡       | Nit         | Un problème mineur, utile à corriger mais non bloquant                         |
| 🟣       | Préexistant | Un bug qui existe dans la base de code mais n'a pas été introduit par cette PR |

Les résultats incluent une section de raisonnement étendu réductible que vous pouvez développer pour comprendre pourquoi Claude a signalé le problème et comment il a vérifié le problème.

<h3 id="rate-and-reply-to-findings">
  Évaluer et répondre aux résultats
</h3>

Chaque commentaire de révision de Claude arrive avec 👍 et 👎 déjà attachés de sorte que les deux boutons apparaissent dans l'interface utilisateur GitHub pour un classement en un clic. Cliquez sur 👍 si le résultat était utile ou 👎 s'il était incorrect ou bruyant. Anthropic collecte les comptages de réactions après la fusion de la PR et les utilise pour affiner le réviseur. Les réactions ne déclenchent pas une re-révision ou ne changent rien sur la PR.

Répondre à un commentaire en ligne ne pousse pas Claude à répondre ou à mettre à jour la PR. Pour agir sur un résultat, corrigez le code et poussez. Si la PR est abonnée aux révisions déclenchées par push, la prochaine exécution résout le thread quand le problème est corrigé. Pour demander une révision fraîche sans pousser, commentez `@claude review` comme un [commentaire PR de haut niveau](#manually-trigger-reviews).

Pour rejeter un résultat sans modification du code, résolvez son thread ; répondre ne le rejette pas.

<h3 id="check-run-output">
  Sortie de l'exécution de vérification
</h3>

Au-delà des commentaires de révision en ligne, chaque révision remplit l'exécution de vérification **Claude Code Review** qui apparaît aux côtés de vos vérifications CI. Développez son lien **Details** pour voir un résumé de chaque résultat en un seul endroit, trié par gravité :

| Gravité      | Fichier:Ligne             | Problème                                                                                                   |
| ------------ | ------------------------- | ---------------------------------------------------------------------------------------------------------- |
| 🔴 Important | `src/auth/session.ts:142` | L'actualisation du token entre en concurrence avec la déconnexion, laissant les sessions obsolètes actives |
| 🟡 Nit       | `src/auth/session.ts:88`  | `parseExpiry` retourne silencieusement 0 sur une entrée malformée                                          |

Chaque résultat apparaît également comme une annotation dans l'onglet **Files changed**, marqué directement sur les lignes de diff pertinentes. Les résultats importants s'affichent avec un marqueur rouge, les nits avec un avertissement jaune, et les bugs préexistants avec un avis gris. Les annotations et le tableau de gravité sont écrits dans l'exécution de vérification indépendamment des commentaires de révision en ligne, de sorte qu'ils restent disponibles même si GitHub rejette un commentaire en ligne sur une ligne qui a bougé.

L'exécution de vérification se termine toujours avec une conclusion neutre, de sorte qu'elle ne bloque jamais la fusion via les règles de protection de branche. Si vous souhaitez conditionner les fusions aux résultats de Code Review, lisez la répartition de gravité à partir de la sortie de l'exécution de vérification dans votre propre CI. La dernière ligne du texte Details est un commentaire lisible par machine que votre flux de travail peut analyser avec `gh` et jq. Pour trouver l'ID de l'exécution de vérification, listez les exécutions de vérification du commit avec `gh api repos/OWNER/REPO/commits/<commit-sha>/check-runs --jq '.check_runs[] | {id, name}'` et prenez l'`id` de l'exécution `Claude Code Review`. Remplacez `OWNER`, `REPO` et `CHECK_RUN_ID` par le propriétaire de votre référentiel, le nom du référentiel et cet ID :

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID \
  --jq '.output.text | split("bughunter-severity: ")[1] | split(" -->")[0] | fromjson'
```

Cela retourne un objet JSON avec des comptages par gravité, par exemple `{"normal": 2, "nit": 1, "pre_existing": 0}`. La clé `normal` contient le nombre de résultats importants ; une valeur non nulle signifie que Claude a trouvé au least un bug à corriger avant la fusion.

<h3 id="what-code-review-checks">
  Ce que Code Review vérifie
</h3>

Par défaut, Code Review se concentre sur la correction : les bugs qui cassent la production, pas les préférences de formatage ou la couverture de test manquante. Vous pouvez élargir ce qu'il vérifie en [ajoutant des fichiers de guidance](#customize-reviews) à votre référentiel.

<h2 id="set-up-code-review">
  Configurer Code Review
</h2>

Un propriétaire active Code Review une fois pour l'organisation et sélectionne les référentiels à inclure.

<Steps>
  <Step title="Ouvrir les paramètres d'administration Claude Code">
    Allez à [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) et trouvez la section Code Review. Vous avez besoin du rôle Propriétaire ou Propriétaire principal dans votre organisation Claude et de la permission d'installer des GitHub Apps dans votre organisation GitHub.
  </Step>

  <Step title="Démarrer la configuration">
    Cliquez sur **Setup**. Cela commence le flux d'installation de GitHub App.
  </Step>

  <Step title="Installer la GitHub App Claude">
    Suivez les invites pour installer la GitHub App Claude : choisissez l'organisation GitHub qui possède les référentiels que vous souhaitez examiner, sélectionnez les référentiels auxquels l'application peut accéder, et approuvez les permissions demandées.

    Pour examiner une demande de tirage, Claude lit le contenu de votre référentiel via l'accès en lecture de l'application, et publie des commentaires et l'[exécution de vérification](#check-run-output) via son accès en écriture aux demandes de tirage et aux vérifications. Lors de l'installation, vous accordez un ensemble de permissions plus large partagé par d'autres fonctionnalités Claude, telles que [GitHub Actions](/docs/fr/github-actions) ; consultez [Permissions de GitHub App](/docs/fr/github-actions#github-app-permissions) pour la liste complète.
  </Step>

  <Step title="Sélectionner les référentiels">
    Choisissez les référentiels à activer pour Code Review. Si vous ne voyez pas un référentiel, assurez-vous d'avoir donné à la GitHub App Claude l'accès pendant l'installation. Vous pouvez ajouter plus de référentiels plus tard.
  </Step>

  <Step title="Définir les déclencheurs de révision par référentiel">
    Une fois la configuration terminée, la section Code Review affiche vos référentiels dans un tableau. Pour chaque référentiel, utilisez la liste déroulante **Review Behavior** pour choisir quand les révisions s'exécutent :

    * **Once after PR creation** : la révision s'exécute une fois à l'ouverture d'une PR ou marquée comme prête pour révision
    * **After every push** : la révision s'exécute à chaque push vers la branche PR, détectant les nouveaux problèmes à mesure que la PR évolue et résolvant automatiquement les threads lorsque vous corrigez les problèmes signalés
    * **Manual** : l'ouverture ou le push vers une PR ne démarre pas une révision ; commentez [`@claude review`](#manually-trigger-reviews) pour en demander une, ou `@claude review always` pour également abonner la PR aux révisions lors des pushes ultérieurs

    Quelle que soit l'option que vous choisissez, Claude examine une [demande de tirage à partir d'une fourche](#review-pull-requests-from-forks) uniquement quand quelqu'un commente `@claude review` sur celle-ci.

    Réviser à chaque push exécute le plus de révisions et coûte le plus cher. Le mode manuel est utile pour les référentiels à fort trafic où vous souhaitez opter pour des PR spécifiques dans la révision, ou pour commencer à réviser vos PR uniquement une fois qu'elles sont prêtes.
  </Step>
</Steps>

Le tableau des référentiels affiche également le coût moyen par révision pour chaque référentiel en fonction de l'activité récente. Utilisez le menu d'actions de ligne pour activer ou désactiver Code Review par référentiel, ou pour supprimer complètement un référentiel.

Pour vérifier la configuration, ouvrez une PR de test. Si vous avez choisi un déclencheur automatique, une exécution de vérification nommée **Claude Code Review** apparaît dans quelques minutes. Si vous avez choisi Manual, commentez `@claude review` sur la PR pour démarrer la première révision. Si aucune exécution de vérification n'apparaît, confirmez que le référentiel est listé dans vos paramètres d'administration et que la GitHub App Claude y a accès.

<h2 id="manually-trigger-reviews">
  Déclencher manuellement les révisions
</h2>

Les commandes de commentaire démarrent une révision à la demande. Elles fonctionnent quel que soit le déclencheur configuré du référentiel, de sorte que vous pouvez les utiliser pour opter pour des PR spécifiques dans la révision en mode Manual ou pour obtenir une re-révision immédiate dans d'autres modes.

| Commande                | Ce qu'elle fait                                                                                |
| :---------------------- | :--------------------------------------------------------------------------------------------- |
| `@claude review`        | Démarre une seule révision sans abonner la PR aux pushes futurs                                |
| `@claude review always` | Démarre une révision et abonne la PR aux révisions déclenchées par push à partir de maintenant |
| `@claude review once`   | Identique à `@claude review` : démarre une seule révision sans abonner                         |

Utilisez `@claude review always` quand vous souhaitez que chaque push ultérieur vers la PR démarre une révision complète, par exemple sur une PR hautement prioritaire dans un référentiel défini en mode Manual. Comme la commande simple n'abonne pas la PR, vous pouvez demander un deuxième avis ponctuel sans modifier si les pushes ultérieurs déclenchent des révisions.

<Note>
  Avant une mise à jour de juillet 2026, `@claude review` abonnait la PR aux révisions déclenchées par push. Si vous aviez compté sur ce comportement, commentez `@claude review always` à la place. `@claude review once` fonctionne toujours et se comporte de la même manière que la commande simple.
</Note>

Pour que l'une de ces commandes déclenche une révision :

* Publiez-la comme un commentaire PR de haut niveau, pas un commentaire en ligne sur une ligne de diff
* Mettez la commande au début du commentaire, avec `once` ou `always` sur la même ligne que le reste de la commande
* Vous devez avoir un accès en écriture, maintenance ou administrateur au référentiel
* La PR doit être ouverte

Si le référentiel appartient à une organisation et votre adhésion à cette organisation est privée, ce qui est la valeur par défaut de GitHub, GitHub ne vous identifie pas à Claude en tant que membre. Claude peut toujours réagir à votre commentaire avec 👀, mais il ne démarre pas une révision à moins que vous ayez été ajouté au référentiel directement en tant que collaborateur, même quand une équipe ou les permissions de base de l'organisation vous donnent un accès en écriture. Pour corriger cela, [rendez votre adhésion à l'organisation publique](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-your-membership-in-organizations/publicizing-or-hiding-organization-membership) ou demandez à un administrateur du référentiel de vous ajouter au référentiel en tant que collaborateur.

Contrairement aux déclencheurs automatiques, les déclencheurs manuels s'exécutent sur les PR brouillon, car une demande explicite signale que vous souhaitez la révision maintenant quel que soit le statut de brouillon.

Si une révision s'exécute déjà sur cette PR, la demande est mise en file d'attente jusqu'à ce que la révision en cours se termine. Vous pouvez surveiller la progression via l'exécution de vérification sur la PR.

<h3 id="review-pull-requests-from-forks">
  Réviser les demandes de tirage à partir de forks
</h3>

Claude ne révise pas une demande de tirage à partir d'un fork automatiquement, quel que soit le paramètre **Review Behavior** du référentiel. Pour en démarrer une, commentez `@claude review` sur la demande de tirage. Les [exigences pour les commandes de commentaire](#manually-trigger-reviews) s'appliquent toujours, et l'accès en écriture dont vous avez besoin est au référentiel de base, pas au fork.

Pour obtenir une autre révision d'une demande de tirage fork, publiez un nouveau commentaire `@claude review`. `@claude review always` fonctionne aussi, mais n'abonne pas la demande de tirage aux révisions sur les pushes ultérieurs. Rien d'autre qu'une commande de commentaire ne démarre une révision sur une demande de tirage fork :

* Cliquer sur **Re-run** sur l'exécution de vérification ne démarre pas une révision
* Pousser de nouveaux commits ne démarre pas une révision, même dans un référentiel défini sur **After every push**

<h2 id="customize-reviews">
  Personnaliser les révisions
</h2>

Code Review lit deux fichiers de votre référentiel pour guider ce qu'il signale. Ils diffèrent dans la force avec laquelle ils influencent la révision :

* **`CLAUDE.md`** : instructions de projet partagées que Claude Code utilise pour toutes les tâches, pas seulement les révisions. Code Review le lit comme contexte de projet et signale les violations nouvellement introduites comme des nits.
* **`REVIEW.md`** : instructions de révision uniquement, données aux agents qui trouvent et vérifient les résultats et consultées par les agents qui classent et rapportent les résultats. Utilisez-le pour dire ce que votre équipe veut signaler, à quelle gravité, et comment les résultats sont rapportés.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review lit vos fichiers `CLAUDE.md` du référentiel et traite les violations nouvellement introduites comme des [résultats au niveau nit](#severity-levels). Cela fonctionne bidirectionnellement : si votre PR modifie le code d'une manière qui rend une déclaration `CLAUDE.md` obsolète, Claude signale que les docs doivent être mises à jour aussi.

Claude lit les fichiers `CLAUDE.md` à chaque niveau de votre hiérarchie de répertoires, donc les règles dans le `CLAUDE.md` d'un sous-répertoire s'appliquent uniquement aux fichiers sous ce chemin. Consultez la [documentation de mémoire](/docs/fr/memory) pour plus d'informations sur le fonctionnement de `CLAUDE.md`.

Pour la guidance spécifique à la révision que vous ne souhaitez pas appliquer aux sessions Claude Code générales, utilisez [`REVIEW.md`](#review-md) à la place.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` est un fichier à la racine de votre référentiel qui adapte Code Review à votre référentiel. Les agents du pipeline de révision qui trouvent et vérifient les résultats reçoivent son contenu comme les instructions de révision de votre référentiel, aux côtés de la guidance de révision par défaut de Code Review, et les agents qui classent et rapportent les résultats le consultent avant de fixer la gravité et d'écrire la révision.

Mettez les règles que vous souhaitez appliquer directement dans `REVIEW.md`.

<h4 id="what-you-can-tune">
  Ce que vous pouvez affiner
</h4>

`REVIEW.md` est du markdown libre, donc tout ce que vous pouvez exprimer comme une instruction de révision est dans le champ d'application. Les modèles ci-dessous ont le plus d'impact en pratique.

**Gravité** : redéfinissez ce que 🔴 Important signifie pour votre référentiel. L'étalonnage par défaut cible le code de production ; un référentiel de docs, un référentiel de config, ou un prototype pourrait vouloir une définition beaucoup plus étroite. Énoncez explicitement quelles classes de résultats sont Important et lesquelles sont Nit au maximum. Vous pouvez également escalader dans l'autre direction, par exemple en traitant toute violation `CLAUDE.md` comme Important plutôt que le nit par défaut.

**Volume de nit** : limitez le nombre de commentaires 🟡 Nit qu'une seule révision publie. La prose et les fichiers de config peuvent être polis à jamais. Un plafond comme « signaler au maximum cinq nits, mentionner le reste comme un comptage dans le résumé » garde les révisions actionnables.

**Règles de saut** : listez les chemins, les modèles de branche et les catégories de résultats où Claude ne devrait publier aucun résultat. Les candidats courants sont le code généré, les lockfiles, les dépendances vendues, et les branches créées par machine, ainsi que tout ce que votre CI applique déjà comme le linting ou la vérification orthographique. Pour les chemins qui méritent une certaine révision mais pas un examen complet, définissez une barre plus élevée au lieu de sauter entièrement : « dans `scripts/`, signaler uniquement si proche de certain et grave. »

**Vérifications spécifiques au référentiel** : ajoutez des règles que vous souhaitez signaler sur chaque PR, comme « les nouveaux itinéraires API doivent avoir un test d'intégration. » Parce que `REVIEW.md` atteint chaque agent de résultat et de vérification directement, ceux-ci atterrissent plus fiablement que les mêmes règles dans un long `CLAUDE.md`.

**Barre de vérification** : exigez des preuves avant qu'une classe de résultat soit publiée. Par exemple, « les affirmations de comportement ont besoin d'une citation `file:line` dans la source, pas une inférence à partir de la dénomination » réduit les faux positifs qui coûteraient autrement à l'auteur un aller-retour.

**Convergence de re-révision** : dites à Claude comment se comporter quand une PR a déjà été révisée. Une règle comme « après la première révision, supprimez les nouveaux nits et publiez les résultats Important uniquement » empêche un correctif d'une ligne d'atteindre la septième manche sur le style seul.

**Forme du résumé** : demandez au corps de la révision de s'ouvrir avec un comptage d'une ligne comme `2 factual, 4 style`, et de commencer par « aucun problème factuel » quand c'est le cas. L'auteur veut connaître la forme du travail avant les détails.

<h4 id="example">
  Exemple
</h4>

Ce `REVIEW.md` recalibre la gravité pour un service backend, limite les nits, saute les fichiers générés, et ajoute des vérifications spécifiques au référentiel.

```markdown theme={null}
# Instructions de révision

## Ce que Important signifie ici

Réservez Important aux résultats qui cassent le comportement, fuient les données,
ou bloquent un rollback : logique incorrecte, requêtes de base de données non scoped, PII
dans les logs ou les messages d'erreur, et les migrations qui ne sont pas backward
compatible. Le style, la dénomination, et les suggestions de refactorisation sont Nit au
maximum.

## Limiter les nits

Signaler au maximum cinq Nits par révision. Si vous en avez trouvé plus, dites « plus N
éléments similaires » dans le résumé au lieu de les publier en ligne. Si
tout ce que vous avez trouvé est un Nit, commencez le résumé par « Aucun problème bloquant. »

## Ne pas signaler

- Tout ce que CI applique déjà : lint, formatage, erreurs de type
- Fichiers générés sous `src/gen/` et tout fichier `*.lock`
- Code de test uniquement qui viole intentionnellement les règles de production

## Toujours vérifier

- Les nouveaux itinéraires API ont un test d'intégration
- Les lignes de log n'incluent pas les adresses e-mail, les ID utilisateur, ou les corps de requête
- Les requêtes de base de données sont scoped au tenant de l'appelant
```

<h4 id="keep-it-focused">
  Gardez-le concentré
</h4>

La longueur a un coût : un long `REVIEW.md` dilue les règles qui importent le plus. Gardez-le aux instructions qui changent le comportement de révision, et laissez le contexte de projet général dans `CLAUDE.md`.

<h2 id="view-usage">
  Afficher l'utilisation
</h2>

Allez à [claude.ai/analytics/code-review](https://claude.ai/analytics/code-review) pour voir l'activité Code Review dans votre organisation. Le tableau de bord affiche :

| Section              | Ce qu'il affiche                                                                                         |
| :------------------- | :------------------------------------------------------------------------------------------------------- |
| PRs reviewed         | Nombre quotidien de pull requests examinées sur la plage de temps sélectionnée                           |
| Cost weekly          | Dépenses hebdomadaires sur Code Review                                                                   |
| Feedback             | Nombre de commentaires de révision qui ont été auto-résolus parce qu'un développeur a résolu le problème |
| Repository breakdown | Comptages par référentiel des PR examinées et des commentaires résolus                                   |

Les chiffres de coût du tableau de bord sont des estimations pour surveiller l'activité. Pour les dépenses exactes de facture, consultez votre facture Anthropic.

<h2 id="pricing">
  Tarification
</h2>

Code Review est facturé en fonction de l'utilisation des tokens. Chaque révision coûte en moyenne 15 à 25 dollars, s'adaptant à la taille de la PR, à la complexité de la base de code, et au nombre de problèmes nécessitant une vérification. L'utilisation de Code Review est facturée séparément via [crédits d'utilisation](https://support.claude.com/fr/articles/12429409-extra-usage-for-paid-claude-plans) et ne compte pas par rapport à l'utilisation incluse de votre plan.

Le déclencheur de révision que vous choisissez affecte le coût total :

* **Une fois après la création de la PR** : s'exécute une fois par PR
* **Après chaque push** : s'exécute à chaque push, multipliant le coût par le nombre de pushes
* **Manuel** : aucune révision sur les PR ouvertes ou les pushes, donc le coût s'accumule uniquement à partir des révisions que quelqu'un demande

En mode Une fois après la création de la PR ou Manuel, commenter `@claude review always` [opte la PR dans les révisions déclenchées par push](#manually-trigger-reviews), de sorte que des coûts supplémentaires s'accumulent par push après ce commentaire. En mode Après chaque push, les pushes déclenchent déjà des révisions, donc l'abonnement ne change pas le coût par push. Commenter `@claude review` exécute une seule révision sans vous abonner à des pushes futurs. Claude révise une [pull request à partir d'une fork](#review-pull-requests-from-forks) uniquement lorsque quelqu'un commente `@claude review`, donc une pull request à partir d'une fork n'accumule jamais de coût par push dans aucun mode.

Les coûts apparaissent sur votre facture Anthropic quel que soit le fait que votre organisation utilise Amazon Bedrock ou Google Cloud's Agent Platform pour d'autres fonctionnalités Claude Code. Pour définir un plafond de dépenses mensuelles pour Code Review, allez à [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) et configurez la limite pour le service Claude Code Review.

Surveillez les dépenses via le graphique de coût hebdomadaire dans [analytics](#view-usage) ou la colonne de coût moyen par référentiel dans les paramètres d'administration.

<h2 id="troubleshooting">
  Dépannage
</h2>

Les exécutions de révision sont au mieux. Une exécution échouée ne bloque jamais votre PR, mais elle ne se réessaye pas non plus d'elle-même. Cette section couvre comment récupérer d'une exécution échouée et où chercher quand l'exécution de vérification signale des problèmes que vous ne pouvez pas trouver.

<h3 id="retrigger-a-failed-or-timed-out-review">
  Redéclencher une révision échouée ou expirée
</h3>

Quand l'infrastructure de révision rencontre une erreur interne ou dépasse sa limite de temps, l'exécution de vérification se termine avec un titre de **Code review encountered an error** ou **Code review timed out**. La conclusion est toujours neutre, de sorte que rien ne bloque votre fusion, mais aucun résultat n'est publié.

Pour exécuter la révision à nouveau, commentez `@claude review` sur la PR. Cela démarre une révision fraîche sans abonner la PR aux pushes futurs. Si la PR n'est pas [issue d'une fork](#review-pull-requests-from-forks), vous pouvez plutôt cliquer sur **Re-run** sur la vérification **Claude Code Review** dans l'onglet Checks de GitHub. Une réexécution démarre également une révision fraîche sans abonner la PR.

<h3 id="review-didn’t-run-and-the-pr-shows-a-spend-cap-message">
  Révision n'a pas s'exécuté et la PR affiche un message de plafond de dépenses
</h3>

Quand le plafond de dépenses mensuelles de votre organisation est atteint, Code Review publie un seul commentaire sur la PR expliquant que la révision a été ignorée. Les révisions reprennent automatiquement au début de la prochaine période de facturation, ou immédiatement quand un administrateur augmente le plafond à [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

<h3 id="find-issues-that-aren’t-showing-as-inline-comments">
  Trouver les problèmes qui ne s'affichent pas comme des commentaires en ligne
</h3>

Si le titre de l'exécution de vérification dit que des problèmes ont été trouvés mais que vous ne voyez pas de commentaires de révision en ligne sur le diff, cherchez dans ces autres emplacements où les résultats sont surfacés :

* **Check run Details** : cliquez sur **Details** à côté de la vérification Claude Code Review dans l'onglet Checks. Le tableau de gravité liste chaque résultat avec son fichier, sa ligne, et son résumé quel que soit le fait que le commentaire en ligne ait été accepté.
* **Files changed annotations** : ouvrez l'onglet **Files changed** sur la PR. Les résultats s'affichent comme des annotations attachées directement aux lignes de diff, séparées des commentaires de révision.
* **Review body** : si vous avez poussé vers la PR pendant qu'une révision s'exécutait, certains résultats peuvent référencer des lignes qui n'existent plus dans le diff actuel. Ceux-ci apparaissent sous un titre **Additional findings** dans le texte du corps de révision plutôt que comme des commentaires en ligne.

<h2 id="review-a-diff-locally">
  Révision d'une diff localement
</h2>

La commande [`/code-review`](/docs/fr/commands) examine une diff dans votre terminal sans installer l'application GitHub. Elle signale les bugs de correction et la réutilisation, la simplification, et les nettoyages d'efficacité.

`/review` est un alias de `/code-review` ; avant v2.1.223, c'était une commande séparée qui exécutait une révision en une seule passe, en lecture seule, d'une demande de tirage GitHub.

<Steps>
  <Step title="Exécuter /code-review">
    À partir de la session où vous travaillez, exécutez la commande :

    ```text theme={null}
    /code-review
    ```

    Elle examine les commits de votre branche en avance sur son amont plus les modifications non validées, elle a donc besoin de travail sur la branche ou dans l'arborescence de travail pour avoir quelque chose à signaler. Pour examiner quelque chose d'autre, passez une cible : un chemin de fichier, un numéro de PR, un nom de branche, ou une plage de références telle que `main...my-feature`.

    Vous pouvez également ajouter des drapeaux :

    * `--fix` : applique les résultats à votre arborescence de travail après la révision
    * `--comment` : publie les résultats sous forme de commentaires en ligne sur une demande de tirage GitHub, ou sur une demande de fusion GitLab sous forme de note unique
    * `--post` : sur une révision `ultra` en cloud d'une demande de tirage `github.com`, présélectionne la publication des résultats terminés à la PR dans la boîte de dialogue de lancement ; voir [Publier les résultats à la demande de tirage](/docs/fr/ultrareview#post-findings-to-the-pull-request). Nécessite Claude Code v2.1.227 ou ultérieur

    Quand vous passez `--comment` pour une demande de fusion GitLab, Claude Code publie les résultats via l'interface de ligne de commande `glab` de GitLab. Nécessite Claude Code v2.1.257 ou ultérieur. Quand `glab` n'est pas installé, Claude imprime les résultats dans le terminal à la place.

    Passez la demande de fusion comme son URL ou une référence `!123`. Claude Code traite un nombre nu ou un nom de branche comme une demande de fusion uniquement quand l'extraction d'origine est sur `gitlab.com`. Sur une instance GitLab auto-gérée, passez l'URL ou la forme `!123`.
  </Step>

  <Step title="Continuer à travailler">
    La révision s'exécute en tant que [sous-agent](/docs/fr/sub-agents) en arrière-plan avec sa propre fenêtre de contexte, elle ne remplit donc pas votre conversation. Les résultats arrivent dans votre conversation quand la révision se termine.
  </Step>

  <Step title="Agir sur les résultats">
    Demandez à Claude de corriger ce que la révision a trouvé. Si vous avez passé `--fix` ou `--comment`, la révision a déjà appliqué ou publié ses résultats.
  </Step>
</Steps>

Claude signale les résultats sous forme de texte dans la réponse dans ces deux exécutions, même quand une application hôte demande une liste des résultats :

* Dans une session de terminal, où `/code-review` exécute la révision en tant que [sous-agent forké](/docs/fr/skills#run-skills-in-a-subagent)
* Dans une exécution `-p` avec sortie texte ou JSON

Dans une application hôte qui demande la liste des résultats, comme l'[application de bureau](/docs/fr/desktop), Claude signale les résultats de la révision via l'[outil `ReportFindings`](/docs/fr/tools-reference). Claude Code affiche le rapport sous forme de liste des résultats, et chaque entrée affiche l'emplacement du fichier, un résumé d'une phrase, et une balise de catégorie telle que `correctness` quand le résultat en porte une. Une demande hôte s'applique à chaque niveau d'effort et nécessite Claude Code v2.1.218 ou ultérieur.

Quand Claude corrige les résultats signalés plus tard dans la session, il les signale à nouveau, et Claude Code marque chaque résultat dans la liste des résultats mise à jour comme corrigé, ignoré, ou aucun changement nécessaire.

<h3 id="what-the-review-reads-and-edits">
  Ce que la révision lit et édite
</h3>

La révision suit votre `CLAUDE.md` comme n'importe quelle session Claude Code, mais elle ne lit pas [`REVIEW.md`](#review-md). Une révision en arrière-plan applique ses édits `--fix` en dehors des [points de contrôle](/docs/fr/checkpointing#subagent-edits-not-restored) de votre session, donc `/rewind` ne les annule pas ; utilisez git pour les annuler. Quand la révision [s'exécute au premier plan](#run-in-the-foreground), elle édite votre arborescence de travail pendant votre propre tour, donc `/rewind` restaure ses édits comme d'habitude.

<h3 id="tune-effort-and-arguments">
  Ajuster l'effort et les arguments
</h3>

Passez un [niveau d'effort](/docs/fr/model-config#adjust-effort-level) pour échanger la couverture contre la confiance. À `low` et `medium`, la révision signale uniquement les résultats dont elle est la plus confiante, vous voyez donc moins de faux positifs ; `high` à `max` élargissent la couverture et peuvent inclure des résultats dont la révision est moins sûre.

Quand vous ne tapez pas de niveau, la révision réutilise le dernier niveau de `low` à `max` que vous avez tapé, même dans une session antérieure, et Claude Code affiche un avis tel que `Reusing high effort, the level you typed last time`. Tapez un niveau, comme `/code-review high`, pour changer ce que les exécutions ultérieures réutilisent ; un niveau que vous passez dans une exécution `-p` non interactive ne le met pas à jour. `ultra` ne met à jour ni n'utilise le niveau mémorisé. Si vous n'avez jamais tapé de niveau, la révision utilise l'effort actuel de la session. Avant v2.1.223, un `/code-review` sans niveau utilisait toujours l'effort actuel de la session.

Après le niveau d'effort et les drapeaux, Claude Code lit le reste de la ligne de l'une de deux façons :

* **Sans `ultra`** : tout ce qui reste est la cible de révision, même quand cela commence par un autre nom de commande. `/code-review /fix-issue 123` examine avec `/fix-issue 123` comme texte cible au lieu de charger `/fix-issue` comme une deuxième [compétence empilée](/docs/fr/skills#pass-arguments-to-skills). Avant v2.1.218, une commande empilée après `/code-review` s'étendait comme sa propre compétence.
* **Avec `ultra`** : Claude Code lit un seul mot comme une branche de base ou un numéro de PR, et transforme le texte plus long qui ne nomme pas une branche ou une PR en [une note attachée à la révision](/docs/fr/ultrareview#pass-a-request-in-plain-words). `/code-review ultra check my auth changes` examine votre branche actuelle, et Claude relie les résultats à votre note.

<h3 id="run-in-the-foreground">
  Exécuter au premier plan
</h3>

La révision s'exécute en arrière-plan par défaut ; avant v2.1.218, elle s'exécutait dans votre conversation. Elle s'exécute au premier plan à la place dans des cas comme ceux-ci :

* Vous exécutez `/code-review` à nouveau alors qu'une révision antérieure est toujours en cours
* Vous l'exécutez en mode non interactif, avec le drapeau `-p` ou le SDK Agent ; Claude Code attend la révision et inclut les résultats dans la réponse, sauf pour `ultra`, qui [lance la révision en cloud sans attendre](#escalate-to-ultrareview)
* Vous définissez [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/fr/env-vars) à `1`, ce qui désactive également toute autre fonctionnalité de tâche en arrière-plan

<h3 id="let-claude-start-the-review">
  Laisser Claude démarrer la révision
</h3>

Claude peut démarrer `/code-review` de lui-même. Demandez-lui d'examiner vos modifications en langage naturel et il peut exécuter la compétence sans que vous tapiez la commande, et une [tâche programmée](/docs/fr/scheduled-tasks) avec `/code-review` comme invite exécute la révision.

Une tâche programmée ne lance jamais la [révision en cloud](#escalate-to-ultrareview), donc programmez `/code-review` sans l'argument `ultra`.

Pour empêcher Claude et les tâches programmées de démarrer la révision tout en gardant `/code-review` disponible pour que vous la tapiez, ajoutez une entrée [`skillOverrides`](/docs/fr/skills#override-skill-visibility-from-settings) à un [fichier de paramètres](/docs/fr/settings#where-settings-live) tel que `~/.claude/settings.json` :

```json theme={null}
{
  "skillOverrides": {
    "code-review": "user-invocable-only"
  }
}
```

Avant v2.1.246, Claude démarrait `/code-review` de lui-même uniquement où un drapeau de fonctionnalité récupéré d'Anthropic l'activait. Dans les [sessions qui ne récupèrent pas les drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching), `/code-review` s'exécutait uniquement quand vous le tapiez, et une `/code-review` programmée arrivait à Claude sous forme de texte brut.

<h3 id="escalate-to-ultrareview">
  Escalader vers ultrareview
</h3>

`/code-review ultra --fix` exécute la [ultrareview](/docs/fr/ultrareview) plus profonde dans le cloud, puis applique ses résultats à votre arborescence de travail quand ils reviennent dans votre session.

Ultrareview utilise sa propre portée : votre branche actuelle par rapport à la branche par défaut du référentiel, plus les modifications non validées et mises en scène dans l'arborescence de travail. Pour les modifications non validées des fichiers nommés comme des identifiants ou des clés, tels que les fichiers `.env` et `*.tfvars`, Claude Code suit les règles pour [télécharger un référentiel local vers une session en cloud](/docs/fr/claude-code-on-the-web#send-local-repositories-without-github). Passez un nom de branche, tel que `/code-review ultra develop`, pour comparer par rapport à une base différente.

Quand la cible est une demande de tirage `github.com`, vous pouvez faire en sorte que Claude [publie les résultats terminés à la PR](/docs/fr/ultrareview#post-findings-to-the-pull-request) en tant que commentaire depuis votre compte GitHub. Nécessite Claude Code v2.1.227 ou ultérieur.

<Note>
  Ultrareview nécessite l'authentification avec un compte claude.ai et n'est pas disponible sur Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry, ou pour les organisations avec Zero Data Retention activé. Quand ultrareview n'est pas disponible, `/code-review ultra` exécute une révision locale dans votre session à la place.
</Note>

Pour démarrer une révision en cloud à partir d'un script ou d'une CI, exécutez `claude -p '/code-review ultra'`. Claude Code lance la révision et imprime un lien pour la suivre. Nécessite Claude Code v2.1.218 ou ultérieur.

Quand la révision facturerait les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), Claude Code s'arrête avant de lancer, car la confirmation de facturation a besoin d'une session interactive. Exécutez la [sous-commande `claude ultrareview`](/docs/fr/ultrareview#run-ultrareview-non-interactively) à la place ; en l'exécutant, vous consentez aux frais.

La commande s'appelait `/simplify` avant v2.1.147, quand elle appliquait les correctifs par défaut. `/simplify` exécute une révision de nettoyage séparé qui applique les correctifs sans rechercher les bugs. Si vous avez écrit un script `/simplify` pour la recherche de bugs, passez à `/code-review --fix`.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Commandes](/docs/fr/commands) : exécutez `/code-review` dans une session Claude Code locale pour vérifier une diff avant de pousser
* [GitHub Actions](/docs/fr/github-actions) : exécutez Claude dans vos propres flux de travail GitHub Actions pour une automatisation personnalisée au-delà de la révision de code
* [GitLab CI/CD](/docs/fr/gitlab-ci-cd) : intégration Claude auto-hébergée pour les pipelines GitLab
* [Memory](/docs/fr/memory) : comment les fichiers `CLAUDE.md` fonctionnent dans Claude Code
* [Analytics](/docs/fr/analytics) : suivez l'utilisation de Claude Code au-delà de la révision de code
* [How Anthropic secures its AI-native software development lifecycle](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle) : comment la révision automatisée s'inscrit comme une couche du processus de développement sécurisé d'Anthropic
