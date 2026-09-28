> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Trouver des bugs avec ultrareview

> Exécutez une révision de code approfondie et multi-agents dans le cloud avec /code-review ultra pour trouver et vérifier les bugs avant de fusionner.

<Note>
  Ultrareview est une fonctionnalité en aperçu de recherche. La fonctionnalité, la tarification et la disponibilité peuvent changer en fonction des commentaires. La commande est `/code-review ultra`. Lorsque ultrareview est disponible pour votre compte, `/ultrareview` est un alias.
</Note>

Ultrareview est une révision de code approfondie qui s'exécute en tant que [session cloud](/docs/fr/claude-code-on-the-web) sur l'infrastructure d'Anthropic. Lorsque vous exécutez `/code-review ultra`, Claude Code lance une flotte d'agents examinateurs dans un sandbox cloud pour trouver des bugs dans votre branche ou votre demande de fusion.

Comparé à un `/code-review` local, ultrareview offre :

* **Signal plus élevé** : chaque constatation signalée est indépendamment reproduite et vérifiée, de sorte que les résultats se concentrent sur les bugs réels plutôt que sur les suggestions de style
* **Couverture plus large** : une flotte plus importante d'agents examinateurs explore le changement en parallèle, ce qui met en évidence les problèmes qu'une révision locale pourrait manquer
* **Aucune utilisation de ressources locales** : la révision s'exécute entièrement dans un sandbox cloud, de sorte que votre terminal reste libre pour d'autres travaux pendant qu'elle s'exécute

Ultrareview nécessite une authentification avec un compte claude.ai car il s'exécute en tant que session cloud sur l'infrastructure d'Anthropic. Si vous êtes connecté avec une clé API uniquement, exécutez `/login` et authentifiez-vous d'abord avec claude.ai. Ultrareview n'est pas disponible lors de l'utilisation de Claude Code avec Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, et il n'est pas disponible pour les organisations qui ont activé la rétention zéro des données. Lorsque ultrareview n'est pas disponible, `/code-review ultra` exécute une révision locale dans votre session à la place.

<h2 id="run-ultrareview-from-the-cli">
  Exécuter ultrareview depuis la CLI
</h2>

Démarrez une revue depuis n'importe quel référentiel git :

```text theme={null}
/code-review ultra
```

Sans arguments, ultrareview examine la différence entre votre branche actuelle et la branche par défaut, y compris les modifications non validées et les modifications en attente. Pour les modifications non validées de fichiers nommés comme des identifiants ou des clés, tels que les fichiers `.env` et `*.tfvars`, Claude Code suit les règles pour [charger un référentiel local vers une session cloud](/docs/fr/claude-code-on-the-web#send-local-repositories-without-github).

Pour une revue de branche, Claude Code regroupe l'état du référentiel et le charge dans un sandbox cloud ; lorsque vous [examinez une demande de tirage](#review-a-pull-request), Claude Code ne charge rien de votre machine.

Avant le lancement, Claude Code affiche une boîte de dialogue de confirmation avec l'étendue de la revue, vos exécutions gratuites restantes et le coût estimé ; pour une revue de branche, l'étendue inclut le nombre de fichiers et de lignes. Après confirmation, la revue continue en arrière-plan pendant que vous continuez à utiliser votre session.

La commande s'exécute uniquement lorsque vous l'invoquez avec `/code-review ultra` ; Claude ne démarre pas une ultrareview de lui-même.

<h3 id="review-against-a-different-base">
  Examiner par rapport à une base différente
</h3>

Pour comparer par rapport à une base autre que la branche par défaut, transmettez le nom de la branche. Cet exemple examine votre branche actuelle par rapport à `develop` à la place :

```text theme={null}
/code-review ultra develop
```

La branche de base n'a pas besoin d'exister dans votre clone local ; Claude Code la récupère depuis `origin`. Si le nom contient une faute de frappe, Claude Code suggère le nom de branche le plus proche dans l'erreur.

Un ID de commit ou une étiquette fonctionne également comme base, et la revue couvre alors les modifications sur votre branche depuis ce commit.

<h3 id="review-a-pull-request">
  Examiner une demande de tirage
</h3>

Pour examiner une demande de tirage GitHub au lieu d'une branche locale, transmettez le numéro de PR :

```text theme={null}
/code-review ultra 1234
```

La commande accepte également `#1234`, `PR 1234` et les URL de PR collées ; une URL collée doit pointer vers le référentiel dans votre répertoire actuel.

En mode PR, le sandbox cloud clone la demande de tirage directement depuis l'hôte plutôt que de regrouper votre arborescence de travail locale. Le mode PR fonctionne avec les référentiels sur `github.com` et sur les instances [GitHub Enterprise Server](/docs/fr/github-enterprise-server) qu'un propriétaire a connectées à Claude Code.

Pour les référentiels sur `github.com`, le sandbox clone avec le compte GitHub connecté à votre compte Claude, donc le compte doit pouvoir lire le référentiel de la PR. Claude Code vérifie cela avant de créer la session cloud, sauf si vous avez défini [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars#variables), et refuse le lancement lorsque [aucun compte n'est connecté](/docs/fr/errors#no-github-account-is-connected-to-your-claude-account) ou [le compte ne peut pas voir le référentiel](/docs/fr/errors#your-connected-github-account-cant-see-the-repository) ; le refus nomme la correction. Avant v2.1.248, Claude Code ne vérifiait pas cela avant le lancement.

Exécutez [`/web-setup`](/docs/fr/web-quickstart#connect-from-your-terminal) pour connecter votre connexion GitHub CLI à votre compte Claude.

<h3 id="post-findings-to-the-pull-request">
  Publier les résultats sur la demande de tirage
</h3>

Sur Claude Code v2.1.227 ou version ultérieure, lorsque vous examinez une demande de tirage sur `github.com`, vous pouvez faire en sorte que Claude publie les résultats terminés sur la PR en tant que commentaire simple unique depuis votre propre compte GitHub. Le commentaire n'est pas une revue ou une approbation, et il se termine par une note « Généré par Claude Code ». Lorsque vous examinez une branche ou une demande de tirage GitHub Enterprise Server, Claude Code affiche les résultats dans votre session uniquement.

Claude Code ne publie jamais sauf si vous le choisissez sur cette exécution, et `--no-post` est la valeur par défaut. La publication est un choix que vous faites pour chaque exécution :

* **Interactif** : dans la boîte de dialogue de lancement, sélectionnez **Exécuter et publier les résultats sur la PR en tant que moi**. Si vous ajoutez `--post` à la commande, comme dans `/code-review ultra 1234 --post`, Claude Code présélectionne ce choix et demande toujours avant le lancement.
* **Non-interactif** : exécutez la [sous-commande `claude ultrareview`](#run-ultrareview-non-interactively) avec `--post`. Vous consentez à la publication en exécutant la sous-commande avec le drapeau, donc Claude Code publie sans demander. Dans une exécution `claude -p '/code-review ultra'`, Claude Code se ferme avant l'arrivée des résultats, donc il ne publie rien ; utilisez la sous-commande à la place.

Claude Code ne publie pas depuis votre machine. Il envoie l'ID de session de la revue à l'API Anthropic, qui publie les résultats stockés de la revue en tant que commentaire via le compte GitHub que vous avez connecté à Claude. La publication nécessite la même connexion claude.ai que la revue elle-même, et elle n'est pas disponible sur les fournisseurs tiers ou lorsque vous définissez [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars).

Dans une session interactive, Claude Code démarre la publication lorsque les résultats arrivent, donc gardez la session ouverte jusqu'à ce que la revue se termine. Claude Code conserve le choix de publication uniquement dans cette session. Si la session se termine avant la fin de la revue, Claude Code ne publie rien, même si vous reprenez la conversation plus tard.

Lorsque la publication se termine, Claude vous indique le résultat :

* **Publié** : Claude vous donne un lien vers le commentaire.
* **Déjà publié** : une publication antérieure de la même revue a déjà mis le commentaire sur la PR, donc Claude vous lie à la demande de tirage au lieu de publier à nouveau.
* **Échoué** : Claude vous dit pourquoi, et les résultats restent dans votre terminal pour que vous puissiez les publier à la main.

<h3 id="pass-a-request-in-plain-words">
  Transmettre une demande en langage simple
</h3>

Sur Claude Code v2.1.218 ou version ultérieure, vous pouvez également décrire ce sur quoi vous travaillez en langage simple :

```text theme={null}
/code-review ultra check my auth changes
```

La revue couvre toujours votre branche actuelle, la même étendue que l'exécution sans argument. Claude conserve votre texte comme une note, affichée dans la boîte de dialogue de lancement, et relie les résultats à celui-ci lorsqu'ils arrivent.

Claude Code traite votre texte comme une note uniquement lorsqu'il contient plus d'un mot et n'est pas un nom de branche ou une référence de PR. Il lit un seul mot comme un nom de branche ou une référence de PR, donc un nom de branche mal orthographié obtient l'erreur de branche la plus proche de [Examiner par rapport à une base différente](#review-against-a-different-base) au lieu de se lancer avec une note. Si votre texte combine une référence de PR avec d'autres mots, comme `check PR 123 again`, Claude Code ne se lance pas non plus ; il vous demande de relancer avec le numéro de PR seul pour examiner cette PR, ou sans la référence pour examiner votre branche actuelle.

<Tip>
  Si votre référentiel est trop volumineux pour être regroupé, Claude Code vous invite à utiliser le mode PR à la place. Poussez votre branche et ouvrez une PR brouillon, puis exécutez `/code-review ultra <PR-number>`.
</Tip>

<h3 id="diff-limits-and-fallbacks">
  Limites de diff et solutions de secours
</h3>

Ultrareview vérifie la différence avant tout travail de revue et vous indique quand il ne peut pas l'examiner tel quel :

* **Diff trop volumineux** : une revue de branche peut inclure jusqu'à 500 fichiers modifiés et 8 000 lignes modifiées par défaut. Les valeurs exactes peuvent changer, et le [refus](/docs/fr/errors#diff-is-too-large-for-ultrareview) nomme celles en vigueur, la taille de votre diff et les fichiers avec le plus de lignes modifiées. Claude Code refuse une demande de tirage trop volumineux de la même manière, en nommant ses nombres de fichiers et de lignes mais pas la ventilation par fichier
* **Rien à examiner** : lorsque la différence par rapport à la base est vide, ultrareview refuse et nomme la branche ou le commit auquel il a comparé et le cas dans lequel vous vous trouvez, comme être sur la branche de base elle-même sans rien non validé, ou une branche dont les commits font tous partie de la base déjà. Il suggère également la solution pour ce cas, comme basculer vers la branche avec votre travail, mettre en attente ou valider les modifications locales, ou transmettre une base différente
* **Premier commit** : le premier commit d'un référentiel n'a rien d'antérieur pour comparer, donc ultrareview examine chaque fichier après confirmation dans la boîte de dialogue de lancement. Si vous avez des fichiers non suivis, il refuse à la place et vous dit de `git add` ceux que vous voulez examiner. Les mêmes limites de taille s'appliquent.

  Un premier commit n'est examiné en entier qu'après cette confirmation, donc la sous-commande `claude ultrareview` et `claude -p` le refusent et vous pointent vers une session interactive à la place. Nécessite Claude Code v2.1.277 ou version ultérieure
* **Pas de base de fusion** : lorsque votre branche ne partage aucun historique avec la branche de base, ou que le référentiel n'a pas de branche de base pour comparer, ultrareview examine chaque fichier suivi du référentiel à la place. La solution de secours nécessite un clone complet et applique les mêmes limites de taille. Elle se lance uniquement lorsque vous confirmez dans la boîte de dialogue de lancement ou exécutez la sous-commande `claude ultrareview` vous-même. Dans `claude -p` et partout ailleurs où ni l'un ni l'autre ne se produit, ultrareview refuse, dit que la revue couvrirait chaque fichier, et vous pointe vers une session interactive.

  Sur un checkout sans branches ou autres refs, comme un HEAD détaché créé en vérifiant `FETCH_HEAD` après avoir récupéré une URL, Claude Code [refuse la revue](/docs/fr/errors#your-checkout-has-no-branches) et suggère de créer d'abord une branche

<h2 id="pricing-and-free-runs">
  Tarification et exécutions gratuites
</h2>

Ultrareview est une fonctionnalité premium qui facture l'utilisation supplémentaire plutôt que l'utilisation incluse dans votre plan.

| Plan               | Exécutions gratuites incluses | Après les exécutions gratuites                                                                                                  |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Pro                | 3 exécutions gratuites        | facturées comme [utilisation supplémentaire](https://support.claude.com/fr/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max                | 3 exécutions gratuites        | facturées comme [utilisation supplémentaire](https://support.claude.com/fr/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team et Enterprise | aucune                        | facturées comme [utilisation supplémentaire](https://support.claude.com/fr/articles/12429409-extra-usage-for-paid-claude-plans) |

* **Exécutions gratuites** : les trois exécutions Pro et Max sont une allocation unique par compte et ne se renouvellent pas.
* **Coût par révision** : après avoir utilisé les exécutions gratuites, généralement entre 5 et 25 dollars en utilisation supplémentaire selon la taille du changement, correspondant à l'estimation affichée par la boîte de dialogue de lancement avant chaque exécution.
* **Quand une exécution compte** : une fois que la session cloud démarre. Une révision que vous arrêtez tôt ou qui ne se termine pas utilise quand même une exécution gratuite ; une révision payante facture uniquement l'utilisation supplémentaire pour la portion qui a été exécutée.

Parce que ultrareview facture toujours l'utilisation supplémentaire en dehors des exécutions gratuites, votre compte ou organisation doit avoir l'utilisation supplémentaire activée avant de pouvoir lancer une révision payante. Si l'utilisation supplémentaire n'est pas activée, Claude Code bloque le lancement, et la façon dont vous l'activez dépend de votre accès à la facturation :

* Si vous pouvez gérer la facturation de votre compte, Claude Code vous renvoie aux paramètres de facturation où vous pouvez activer l'utilisation supplémentaire.
* Sur les plans Team et Enterprise, les membres sans accès à la facturation envoient une demande depuis la CLI demandant à leur administrateur d'activer l'utilisation supplémentaire.

Vous pouvez également exécuter `/usage-credits` pour vérifier ou modifier votre paramètre d'utilisation supplémentaire.

Claude Code vous demande de confirmer la facturation de l'utilisation supplémentaire une fois par conversation : quand vous démarrez une nouvelle conversation, par exemple avec `/clear`, Claude Code affiche à nouveau la confirmation pour la prochaine révision payante.

<h2 id="track-a-running-review">
  Suivre une révision en cours
</h2>

Une révision prend généralement 5 à 10 minutes. La révision s'exécute en tant que tâche en arrière-plan, de sorte que vous pouvez continuer à travailler dans votre session, démarrer d'autres commandes ou fermer complètement le terminal. Si vous avez choisi de [publier les constatations sur la demande de tirage](#post-findings-to-the-pull-request), gardez la session ouverte jusqu'à ce que la révision se termine ; si la session se termine en premier, Claude Code ne publie rien.

Utilisez `/tasks` pour voir les révisions en cours et terminées, ouvrir la vue détaillée d'une révision ou arrêter une révision en cours. Si vous arrêtez une révision, Claude Code archive la session cloud et ne retourne pas les constatations partielles.

Claude peut également vous indiquer qu'une révision a été arrêtée ou que sa session n'a pas été trouvée :

* Si la session cloud de la révision est arrêtée ou [archivée](/docs/fr/claude-code-on-the-web#archive-sessions) sur claude.ai avant la fin de la révision, Claude vous indique qu'elle a été arrêtée.
* Si la session cloud de la révision a été supprimée, ou si vous vous êtes connecté à un compte Claude ou une organisation différente depuis son lancement, Claude vous indique que la session n'a pas été trouvée.
* Si vous avez changé de compte, la révision peut toujours se terminer sous le compte qui l'a lancée. Si la révision est toujours en cours, reconnectez-vous avec ce compte et reprenez la conversation avec `claude --resume` pour la réattacher.

Lorsque la révision se termine, Claude Code affiche les constatations vérifiées sous forme de notification dans votre session. Chaque constatation inclut l'emplacement du fichier et une explication du problème afin que vous puissiez demander à Claude de le corriger directement.

<h2 id="run-ultrareview-non-interactively">
  Exécuter ultrareview de manière non-interactive
</h2>

Utilisez la sous-commande `claude ultrareview` pour démarrer une ultrareview à partir de CI ou d'un script sans session interactive. La sous-commande lance la même révision que `/code-review ultra`, bloque jusqu'à ce que la révision distante se termine, et imprime les constatations sur stdout.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Sans arguments, la sous-commande examine la différence entre votre branche actuelle et la branche par défaut, avec le même [repli sur l'ensemble du référentiel](#diff-limits-and-fallbacks) que `/code-review ultra` lorsqu'aucune base de fusion n'existe. Transmettez un numéro de PR pour examiner une demande de fusion, ou une branche de base pour examiner la différence par rapport à celle-ci ; la [gestion de la branche de base](#review-against-a-different-base) correspond à la commande interactive.

Vous consentez au repli sur l'ensemble du référentiel et à l'invite de facturation et de conditions lorsque vous exécutez la sous-commande, de sorte que l'exécution démarre sans attendre d'entrée. L'exécution vous-même est ce qui compte comme consentement. Lorsque Claude exécute la sous-commande pour vous à la place, par exemple via l'outil Bash, Claude Code refuse la révision de l'ensemble du référentiel.

Sur Claude Code v2.1.218 ou ultérieur, vous pouvez également démarrer la révision cloud en exécutant `/code-review ultra` dans une session non-interactive, par exemple `claude -p '/code-review ultra'`. Claude Code lance la révision et imprime un lien de suivi sans attendre les constatations, contrairement à `claude ultrareview`, qui bloque jusqu'à leur arrivée. Lorsque la révision facturerait des crédits d'utilisation, Claude Code s'arrête avant de lancer et vous oriente vers `claude ultrareview`, car la confirmation de facturation nécessite une session interactive. Avant v2.1.218, `/code-review ultra` dans une session non-interactive exécutait une révision locale.

Les messages de progression et l'URL de session en direct vont vers stderr afin que stdout reste analysable. Utilisez ces drapeaux pour contrôler la sortie, le délai d'expiration et si les constatations doivent être publiées :

| Drapeau               | Description                                                                                                                                                                                                                                                                                                                                   |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--json`              | Imprimez la charge utile `bugs.json` brute au lieu des constatations formatées                                                                                                                                                                                                                                                                |
| `--timeout <minutes>` | Nombre maximum de minutes à attendre pour que la révision se termine. Par défaut 45                                                                                                                                                                                                                                                           |
| `--post`              | [Publiez les constatations terminées](#post-findings-to-the-pull-request) sur la demande de fusion sous forme d'un commentaire simple depuis votre compte GitHub. Fonctionne sur les cibles de demande de fusion `github.com` ; sur d'autres cibles, Claude Code ignore le drapeau et le signale. Nécessite Claude Code v2.1.227 ou ultérieur |
| `--no-post`           | Ne publiez pas les constatations. C'est le comportement par défaut, et si vous transmettez les deux drapeaux, Claude Code ne publie pas. Nécessite Claude Code v2.1.227 ou ultérieur                                                                                                                                                          |

L'exécution de `claude ultrareview` nécessite la même authentification et configuration de crédits d'utilisation que `/code-review ultra`.

La sous-commande se termine avec l'un de ces trois codes :

* **0** : la révision s'est terminée, avec ou sans constatations
* **1** : la révision n'a pas pu se lancer ou a été arrêtée avant sa fin, la session cloud a généré une erreur, ou le délai d'expiration s'est écoulé
* **130** : vous avez interrompu la sous-commande avec Ctrl-C

Si vous interrompez la sous-commande, la révision distante continue de s'exécuter ; suivez l'URL de session imprimée sur stderr pour la regarder dans le navigateur.

Avec `--post`, la sous-commande démarre la publication immédiatement après avoir imprimé les constatations, et imprime le lien sur stderr.

* Si l'exécution échoue, est arrêtée ou expire, ou si vous l'interrompez, la sous-commande ne publie rien.
* Si la révision se termine mais que le commentaire n'est pas publié, Claude Code imprime la raison sur stderr, et les constatations restent sur stdout afin que vous puissiez les publier manuellement.

Pour les révisions automatiques sur les demandes de fusion GitHub, [Code Review](/docs/fr/code-review) s'intègre directement à votre référentiel et publie les constatations sous forme de commentaires PR en ligne sans étape CLI.

<h2 id="how-ultrareview-compares-to-/code-review">
  Comment ultrareview se compare à /code-review
</h2>

Les deux révisions examinent le code, mais vous les utilisez à différentes étapes de votre flux de travail.

|            | `/code-review`                                                    | `/code-review ultra`                                                                             |
| ---------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Cible      | votre diff de travail, une pull request, une branche ou un chemin | votre diff de travail ou une pull request                                                        |
| S'exécute  | localement dans votre session                                     | dans un sandbox cloud                                                                            |
| Profondeur | s'adapte à l'argument effort                                      | flotte multi-agents avec vérification indépendante                                               |
| Durée      | secondes à quelques minutes                                       | environ 5 à 10 minutes                                                                           |
| Coût       | compte vers l'utilisation normale                                 | exécutions gratuites, puis environ 5 à 25 dollars par révision en tant que crédits d'utilisation |
| Idéal pour | retours rapides lors de l'itération                               | confiance pré-fusion sur les changements substantiels                                            |

Utilisez `/code-review` pour des retours rapides pendant que vous travaillez, ou transmettez un numéro de PR pour examiner la pull request d'un coéquipier avant de l'approuver. Utilisez `/code-review ultra` avant de fusionner un changement substantiel lorsque vous souhaitez une passe plus approfondie qui détecte les problèmes qu'une révision locale pourrait manquer.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : découvrez comment fonctionnent les sessions distantes et les sandboxes cloud
* [Gérer les coûts efficacement](/docs/fr/costs) : suivre l'utilisation et définir les limites de dépenses
