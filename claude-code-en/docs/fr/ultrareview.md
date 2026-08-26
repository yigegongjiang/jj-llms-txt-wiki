> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Trouver des bugs avec ultrareview

> Exécutez une révision de code approfondie et multi-agents dans le cloud avec /code-review ultra pour trouver et vérifier les bugs avant de fusionner.

<Note>
  Ultrareview est une fonctionnalité en aperçu de recherche. La fonctionnalité, la tarification et la disponibilité peuvent changer en fonction des commentaires. La commande est maintenant invoquée en tant que `/code-review ultra`, et `/ultrareview` reste un alias.
</Note>

Ultrareview est une révision de code approfondie qui s'exécute sur Claude Code sur l'infrastructure web. Lorsque vous exécutez `/code-review ultra`, Claude Code lance une flotte d'agents examinateurs dans un sandbox distant pour trouver des bugs dans votre branche ou votre demande de fusion.

Comparé à un `/code-review` local ou `/review`, ultrareview offre :

* **Signal plus élevé** : chaque constatation signalée est indépendamment reproduite et vérifiée, de sorte que les résultats se concentrent sur les bugs réels plutôt que sur les suggestions de style
* **Couverture plus large** : une flotte plus importante d'agents examinateurs explore le changement en parallèle, ce qui met en évidence les problèmes qu'une révision locale pourrait manquer
* **Aucune utilisation de ressources locales** : la révision s'exécute entièrement dans un sandbox distant, de sorte que votre terminal reste libre pour d'autres travaux pendant qu'elle s'exécute

Ultrareview nécessite une authentification avec un compte Claude.ai car il s'exécute sur Claude Code sur l'infrastructure web. Si vous êtes connecté avec une clé API uniquement, exécutez `/login` et authentifiez-vous d'abord avec Claude.ai. Ultrareview n'est pas disponible lors de l'utilisation de Claude Code avec Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, et il n'est pas disponible pour les organisations qui ont activé la rétention zéro des données.

<h2 id="run-ultrareview-from-the-cli">
  Exécuter ultrareview à partir de la CLI
</h2>

Démarrez une révision à partir de n'importe quel référentiel git dans la CLI Claude Code.

```text theme={null}
/code-review ultra
```

Sans arguments, ultrareview examine la différence entre votre branche actuelle et la branche par défaut, y compris les modifications non validées et mises en scène dans votre arborescence de travail. Claude Code regroupe l'état du référentiel et le télécharge vers un sandbox distant pour la révision.

Pour examiner une demande de fusion GitHub à la place, transmettez le numéro de PR.

```text theme={null}
/code-review ultra 1234
```

En mode PR, le sandbox distant clone la demande de fusion directement depuis l'hôte plutôt que de regrouper votre arborescence de travail locale. Le mode PR fonctionne avec les référentiels sur `github.com` et sur les instances [GitHub Enterprise Server](/docs/fr/github-enterprise-server) qu'un administrateur a connectées à Claude Code.

<Tip>
  Si votre référentiel est trop volumineux pour être regroupé, Claude Code vous invite à utiliser le mode PR à la place. Poussez votre branche et ouvrez une PR brouillon, puis exécutez `/code-review ultra <PR-number>`.

  Si la différence de la demande de fusion est trop importante, Claude Code refuse la révision avec un indice de portée avant que tout travail de révision ne s'exécute.
</Tip>

Avant de lancer, Claude Code affiche une boîte de dialogue de confirmation avec l'étendue de la révision (y compris le nombre de fichiers et de lignes lors de la révision d'une branche), vos exécutions gratuites restantes et le coût estimé. Après confirmation, la révision continue en arrière-plan et vous pouvez continuer à utiliser votre session. La commande s'exécute uniquement lorsque vous l'invoquez avec `/code-review ultra` ; Claude ne démarre pas une ultrareview de lui-même.

<h2 id="pricing-and-free-runs">
  Tarification et exécutions gratuites
</h2>

Ultrareview est une fonctionnalité premium qui facture l'utilisation supplémentaire plutôt que l'utilisation incluse dans votre plan.

| Plan               | Exécutions gratuites incluses | Après les exécutions gratuites                                                                                                  |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Pro                | 3 exécutions gratuites        | facturées comme [utilisation supplémentaire](https://support.claude.com/fr/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max                | 3 exécutions gratuites        | facturées comme [utilisation supplémentaire](https://support.claude.com/fr/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team et Enterprise | aucune                        | facturées comme [utilisation supplémentaire](https://support.claude.com/fr/articles/12429409-extra-usage-for-paid-claude-plans) |

Les abonnés Pro et Max reçoivent trois exécutions ultrareview gratuites pour essayer la fonctionnalité. Ces trois exécutions sont une allocation unique par compte et ne se renouvellent pas. Après les avoir utilisées ou après la fin de la période d'exécutions gratuites, chaque révision est facturée à l'utilisation supplémentaire et coûte généralement entre 5 et 20 dollars selon la taille du changement. Une exécution compte une fois que la session à distance démarre, donc une révision que vous arrêtez tôt ou qui ne se termine pas utilise quand même une exécution gratuite. Pour une révision payante, l'utilisation supplémentaire est facturée uniquement pour la portion qui a été exécutée.

Parce que ultrareview facture toujours l'utilisation supplémentaire en dehors des exécutions gratuites, votre compte ou organisation doit avoir l'utilisation supplémentaire activée avant de pouvoir lancer une révision payante. Si l'utilisation supplémentaire n'est pas activée, Claude Code bloque le lancement et vous renvoie aux paramètres de facturation où vous pouvez l'activer. Vous pouvez également exécuter `/usage-credits` pour vérifier ou modifier votre paramètre actuel.

<h2 id="track-a-running-review">
  Suivre une révision en cours
</h2>

Une révision prend généralement 5 à 10 minutes. La révision s'exécute en tant que tâche en arrière-plan, de sorte que vous pouvez continuer à travailler dans votre session, démarrer d'autres commandes ou fermer complètement le terminal.

Utilisez `/tasks` pour voir les révisions en cours et terminées, ouvrir la vue détaillée d'une révision ou arrêter une révision en cours. L'arrêt d'une révision archive la session cloud, et les constatations partielles ne sont pas renvoyées. Lorsque la révision se termine, les constatations vérifiées apparaissent sous forme de notification dans votre session. Chaque constatation inclut l'emplacement du fichier et une explication du problème afin que vous puissiez demander à Claude de le corriger directement.

<h2 id="run-ultrareview-non-interactively">
  Exécuter ultrareview de manière non-interactive
</h2>

Utilisez la sous-commande `claude ultrareview` pour démarrer une ultrareview à partir de CI ou d'un script sans session interactive. La sous-commande lance la même révision que `/code-review ultra`, bloque jusqu'à ce que la révision distante se termine, imprime les constatations sur stdout et se termine avec le code 0 en cas de succès ou 1 en cas d'échec.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Sans arguments, la sous-commande examine la différence entre votre branche actuelle et la branche par défaut. Transmettez un numéro de PR pour examiner une demande de fusion, ou transmettez une branche de base pour examiner la différence par rapport à cette branche à la place. L'invocation de la sous-commande compte comme consentement pour la facturation et l'invite de conditions que la commande interactive affiche.

Les messages de progression et l'URL de session en direct vont vers stderr afin que stdout reste analysable. Utilisez ces drapeaux pour contrôler la sortie et le délai d'expiration :

| Drapeau               | Description                                                                         |
| --------------------- | ----------------------------------------------------------------------------------- |
| `--json`              | Imprimez la charge utile `bugs.json` brute au lieu des constatations formatées      |
| `--timeout <minutes>` | Nombre maximum de minutes à attendre pour que la révision se termine. Par défaut 30 |

L'exécution de `claude ultrareview` nécessite la même authentification et configuration d'utilisation supplémentaire que `/code-review ultra`. La sous-commande se termine avec le code 0 lorsque la révision se termine avec ou sans constatations, le code 1 lorsque la révision ne parvient pas à se lancer, la session distante génère une erreur ou le délai d'expiration s'écoule, et le code 130 lorsqu'elle est interrompue avec Ctrl-C. La révision distante continue de s'exécuter si vous interrompez la sous-commande ; suivez l'URL de session imprimée sur stderr pour la regarder dans le navigateur.

Pour les révisions automatiques sur les demandes de fusion GitHub, [Code Review](/docs/fr/code-review) s'intègre directement à votre référentiel et publie les constatations sous forme de commentaires PR en ligne sans étape CLI.

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  Comment ultrareview se compare à /code-review et /review
</h2>

Les trois commandes examinent le code, mais elles ciblent différentes étapes de votre flux de travail.

|            | `/code-review`                      | `/review <pr>`                                                   | `/code-review ultra`                                                                             |
| ---------- | ----------------------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Cible      | votre diff de travail               | une pull request GitHub                                          | votre diff de travail ou une pull request                                                        |
| S'exécute  | localement dans votre session       | localement dans votre session                                    | à distance dans un sandbox cloud                                                                 |
| Profondeur | s'adapte à l'argument effort        | une révision en un seul passage au niveau d'effort de la session | flotte multi-agents avec vérification indépendante                                               |
| Durée      | secondes à quelques minutes         | secondes à quelques minutes                                      | environ 5 à 10 minutes                                                                           |
| Coût       | compte vers l'utilisation normale   | compte vers l'utilisation normale                                | exécutions gratuites, puis environ 5 à 20 dollars par révision en tant que crédits d'utilisation |
| Idéal pour | retours rapides lors de l'itération | examen de la PR d'un coéquipier avant approbation                | confiance pré-fusion sur les changements substantiels                                            |

Utilisez `/code-review` pour des retours rapides pendant que vous travaillez. Utilisez `/review <pr>` pour examiner une pull request de la même manière que vous le feriez avant de l'approuver. Utilisez `/code-review ultra` avant de fusionner un changement substantiel lorsque vous souhaitez une passe plus approfondie qui détecte les problèmes qu'une révision locale pourrait manquer.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : découvrez comment fonctionnent les sessions distantes et les sandboxes cloud
* [Planifier les changements complexes avec ultraplan](/docs/fr/ultraplan) : l'équivalent de planification pour ultrareview pour le travail de conception préalable
* [Gérer les coûts efficacement](/docs/fr/costs) : suivre l'utilisation et définir les limites de dépenses
