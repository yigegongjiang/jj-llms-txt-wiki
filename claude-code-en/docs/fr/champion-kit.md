> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit du champion

> Un guide pratique pour les ingénieurs qui défendent Claude Code en interne : quoi partager, comment répondre aux questions et comment augmenter l'adoption dans votre équipe.

Cette page s'adresse aux ingénieurs individuels qui utilisent déjà Claude Code et qui souhaitent aider leur équipe à l'adopter. Elle couvre quoi partager, comment répondre aux questions que vous recevrez, un guide de trente jours et des réponses aux préoccupations courantes.

L'adoption d'un outil de développement se fait rarement à cause d'une annonce de déploiement. Elle se produit parce que quelqu'un dans l'équipe commence à utiliser l'outil correctement, en parle ouvertement et facilite l'adoption pour les autres. Le travail que vous faites en tant que champion a un effet disproportionné : chaque exemple que vous partagez raccourcit la courbe d'apprentissage pour les ingénieurs qui vous suivent, et chaque question que vous répondez publiquement transforme l'expérience d'une personne en quelque chose que toute l'équipe peut exploiter. Vous agissez comme un multiplicateur pour votre équipe, pas comme un support technique, et ce guide est structuré pour maintenir le rôle durable selon ces termes.

<h2 id="the-champion-role">
  Le rôle de champion
</h2>

Le rôle consiste en trois comportements qui se renforcent mutuellement.

| Comportement                             | À quoi cela ressemble en pratique                                                                                                                                                                                          | Pourquoi c'est important                                                                                                                                                                                                      |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Partagez ce que vous découvrez           | Publiez les prompts, les captures d'écran et les petites victoires de votre propre travail dans les endroits que votre équipe lit déjà, comme un canal d'ingénierie, un fil de standup ou une description de pull request. | Les exemples tirés de votre propre codebase sont plus convaincants que n'importe quelle documentation externe, car les collègues peuvent voir exactement comment l'outil s'applique aux problèmes qu'ils partagent avec vous. |
| Soyez la personne que les gens demandent | Quand un collègue vous demande comment vous avez réalisé quelque chose, répondez avec le prompt réel que vous avez utilisé pour qu'il puisse l'appliquer directement à sa propre tâche.                                    | Un exemple concret et exécutable supprime l'écart entre la curiosité et une première utilisation réussie, ce qui est l'endroit où la plupart des efforts d'adoption s'arrêtent.                                               |
| Agrandissez le cercle                    | Établissez un petit nombre d'habitudes légères et récurrentes, comme un canal dédié ou un fil hebdomadaire, afin que l'élan continue même quand votre attention est ailleurs.                                              | L'adoption qui dépend d'une seule personne est fragile. L'adoption qui est portée par des habitudes partagées continue à se composer d'elle-même.                                                                             |

La plupart de cela s'inscrit naturellement dans le travail que vous faites déjà. La différence est une petite quantité d'intention supplémentaire sur l'endroit où vos découvertes sont publiées et comment vos réponses se propagent.

<h3 id="what-this-should-cost-you">
  Ce que cela devrait vous coûter
</h3>

Fixez des attentes avec vous-même et avec votre responsable. Les activités ci-dessous sont destinées à s'adapter à une semaine de travail normale, et le rôle devrait rester un multiplicateur de votre travail existant plutôt qu'une responsabilité de support supplémentaire.

| Activité                                     | Temps par semaine  | Conseils                                                                                                                           |
| -------------------------------------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------- |
| Publier des victoires et des prompts         | Environ 15 minutes | Capturez-les au moment avec une capture d'écran et une ou deux phrases ; évitez de les transformer en rédactions formelles.        |
| Répondre aux questions dans un canal partagé | Environ 20 minutes | Répondez publiquement une fois, puis créez un lien vers cette réponse quand la question revient.                                   |
| Animer un fil de démonstration hebdomadaire  | Environ 5 minutes  | Vous publiez le prompt d'ouverture ; l'équipe fournit le contenu.                                                                  |
| Appairage ou démonstrations optionnels       | 0 à 30 minutes     | Réservez cela aux collègues qui sont vraiment bloqués, et offrez le lien [Quickstart](/docs/fr/quickstart) avant de planifier du temps. |

<h2 id="share-what-you-discover">
  Partagez ce que vous découvrez
</h2>

Votre propre expérience est le matériel le plus convaincant que vos collègues rencontreront, car il est spécifique à la codebase, aux flux de travail et aux problèmes que vous partagez tous. La documentation dit aux gens ce qui est possible ; vos publications leur montrent ce qui fonctionne réellement dans votre environnement.

<h3 id="what-is-worth-sharing">
  Ce qui vaut la peine d'être partagé
</h3>

Les publications les plus utiles décrivent une technique qu'un collègue peut réutiliser demain plutôt qu'un résultat qui est déjà complet. Les techniques se composent à mesure qu'elles se propagent dans une équipe ; les mises à jour de statut ne le font pas.

Exemples de techniques réutilisables :

* « J'ai appris que @-mentionner un répertoire fonctionne. En le pointant vers `@src/components/` et en demandant lesquels manquaient de tests, j'ai découvert deux que j'avais oubliés. »
* « Le mode Plan (`Maj+Tab`) montre exactement quels fichiers seront touchés avant toute modification, c'est pourquoi je suis à l'aise de l'utiliser sur du code partagé. »
* « J'ai configuré un hook Stop pour recevoir une notification de bureau quand une tâche longue se termine. La configuration est dans le fil. »
* « L'exécution de `/init` génère un `CLAUDE.md` à partir du référentiel afin que l'assistant arrête de redemander nos conventions. »

<h3 id="where-to-share-it">
  Où le partager
</h3>

Publiez partout où votre équipe lit déjà. L'objectif est de placer les exemples dans le chemin du travail normal plutôt que de créer une destination.

| Emplacement                                      | Mieux adapté pour                                                                  | Format recommandé                                                                                   |
| ------------------------------------------------ | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Un canal `#claude-code` ou d'ingénierie générale | Découvertes, prompts et moments « aujourd'hui j'ai appris »                        | Une capture d'écran accompagnée d'une ou deux phrases de contexte                                   |
| Descriptions de pull request                     | Démontrer l'approche sur du code réel que les relecteurs lisent déjà               | Une seule ligne comme « Claude et moi avons fait cette refonte ; heureux de parcourir l'approche. » |
| Standups ou mises à jour écrites hebdomadaires   | Normaliser l'utilisation avec les responsables et les managers de niveau supérieur | Une phrase décrivant un résultat concret                                                            |
| Wiki d'équipe ou documentation interne           | Modèles durables, compétences personnalisées et exemples `CLAUDE.md`               | Une courte page, liée depuis le sujet du canal pour qu'elle reste découvrable                       |

<h3 id="the-format-that-works">
  Le format qui fonctionne
</h3>

Une capture d'écran accompagnée d'une seule ligne de contexte, ou une brève description avant-après, est généralement le bon niveau de détail. Gardez chaque publication assez courte pour que quelqu'un qui la parcourt absorbe toujours le point. Une longue rédaction tend à être sauvegardée pour plus tard et oubliée, tandis qu'une courte publication avec une capture d'écran tend à être copiée et essayée.

Les exemples de publications ci-dessous illustrent le ton et la longueur ; adaptez-les plutôt que de les copier textuellement.

```text theme={null}
Appris aujourd'hui que @-mentionner un répertoire fonctionne. Je l'ai pointé vers
@src/components/ et j'ai demandé quels composants manquaient de tests, et cela
a découvert deux que j'avais oubliés.
```

```text theme={null}
J'ai configuré un hook Stop pour recevoir une notification de bureau quand une tâche longue
se termine. J'ai commencé une refonte, je me suis éloigné, et j'ai été notifié quand
c'était terminé. La configuration est dans le fil.
```

```text theme={null}
Le mode Plan est la raison pour laquelle je suis à l'aise d'utiliser ceci sur du code qui compte.
Appuyez sur Maj+Tab jusqu'à ce que vous voyiez « plan » ; cela énumère exactement quels fichiers
il a l'intention de toucher avant de changer quoi que ce soit.
```

<h2 id="be-the-person-people-ask">
  Soyez la personne que les gens demandent
</h2>

Une fois que vous avez partagé quelques exemples, les questions suivront. C'est là que le rôle de champion a le plus grand effet de levier, car une bonne réponse à une personne déverrouille souvent plusieurs autres qui regardent le même canal.

<h3 id="answer-with-a-prompt-rather-than-an-explanation">
  Répondez avec un prompt plutôt qu'une explication
</h3>

Quand un collègue vous demande comment vous avez réalisé quelque chose, la réponse la plus utile est le prompt que vous avez réellement utilisé. Ils apprendront plus en exécutant ce prompt contre leur propre problème que de n'importe quelle description que vous pourriez écrire, et cela leur donne quelque chose sur lequel ils peuvent agir immédiatement.

```text theme={null}
Collègue : Comment avez-vous réussi à trouver cette condition de course ?

Champion : J'ai demandé, « Le test dans @tests/scheduler.test.ts est instable, découvrez
pourquoi, » et il a tracé deux promesses non jointes dans le planificateur. Essayez la
même formulation sur votre test.
```

<h3 id="point-at-the-feature-rather-than-the-documentation">
  Pointez vers la fonctionnalité plutôt que vers la documentation
</h3>

Une réponse comme « Essayez le mode plan, appuyez sur `Maj+Tab` jusqu'à ce que vous le voyiez » est plus utile au moment qu'un lien vers la documentation. Si la personne a besoin de plus de profondeur plus tard, elle la trouvera d'elle-même ; en ce moment, elle a besoin de la seule chose qui la déverrouille.

<h3 id="questions-you-are-likely-to-hear">
  Questions que vous êtes susceptible d'entendre
</h3>

| Question                                                       | Réponse suggérée                                                                                                                                                                                                                                     | Ressource de suivi                                      |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| « Sur quoi devrais-je d'abord l'essayer ? »                    | Recommandez une tâche réelle mais contenue, idéalement un bug ou une tâche que la personne a reportée parce qu'elle est fastidieuse plutôt que difficile.                                                                                            | [Flux de travail courants](/docs/fr/common-workflows)        |
| « Comment puis-je lui faire confiance avec mon code ? »        | Présentez le mode plan : appuyer sur `Maj+Tab` le bascule, Claude propose exactement ce qu'il a l'intention de changer, et rien n'est modifié jusqu'à ce que l'utilisateur approuve.                                                                 | [Permissions](/docs/fr/permissions)                          |
| « L'installation en vaut-elle la peine ? »                     | L'installation prend environ deux minutes, s'exécute dans le terminal et ne nécessite aucune extension IDE. L'exécution de `/init` une fois suffit pour commencer à travailler.                                                                      | [Quickstart](/docs/fr/quickstart)                            |
| « Il a produit un résultat incorrect. »                        | Encouragez-les à fournir l'échec à Claude. Coller le message d'erreur ou le test échoué est beaucoup plus efficace que de reformuler la demande originale.                                                                                           | [Flux de travail courants](/docs/fr/common-workflows)        |
| « Il ne comprend pas les conventions de notre codebase. »      | Suggérez d'exécuter `/init` pour générer un fichier `CLAUDE.md`, puis d'ajouter les conventions de l'équipe, les commandes de test et tous les répertoires qui doivent être évités.                                                                  | [Memory](/docs/fr/memory)                                    |
| « N'est-ce que de l'autocomplétion ? »                         | Offrez une brève démonstration dans laquelle Claude explique un fichier inconnu, trace un bug entre les services ou rédige un plan de migration. Ces tâches nécessitent un raisonnement dans le référentiel plutôt que de compléter une seule ligne. | Une démonstration en direct de deux minutes             |
| « Qu'en est-il de la sécurité et de la gestion des données ? » | Référez cette question à votre administrateur. La politique de déploiement et de gestion des données de votre organisation est déjà configurée, et les champions ne doivent pas improviser cette réponse.                                            | [Security](/docs/fr/security) · [Data usage](/docs/fr/data-usage) |

<h2 id="grow-the-circle">
  Agrandissez le cercle
</h2>

L'objectif n'est pas de construire un programme ou de posséder un déploiement. C'est d'établir un petit nombre d'habitudes légères qui permettent à l'élan de continuer après que vous ayez arrêté de le conduire activement. Quand les questions dans le canal sont répondues par des personnes autres que vous, le rôle a fait son travail.

<h3 id="patterns-that-tend-to-work">
  Modèles qui tendent à fonctionner
</h3>

| Modèle                                                                 | Comment l'exécuter                                                                                                                                                                                                                                                                                       | Effort requis                                      |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| Un canal dédié                                                         | Créez un canal `#claude-code` (ou un fil récurrent dans un existant), épinglez le lien [Quickstart](/docs/fr/quickstart) et un exemple fort, et répondez aux questions publiquement pour que chaque réponse bénéficie à tous ceux qui regardent.                                                              | Environ cinq minutes pour configurer, puis ambiant |
| Un fil de démonstration hebdomadaire                                   | Chaque vendredi, publiez « Avec quoi Claude vous a-t-il aidé cette semaine ? » Aucune préparation, diapositives ou réunion ne sont requises ; les captures d'écran et les brèves descriptions suffisent.                                                                                                 | Environ deux minutes par semaine                   |
| Partagez une compétence personnalisée                                  | Publiez votre fichier `.claude/skills/<name>/SKILL.md` le plus utile, par exemple une compétence `/ship` qui exécute les tests et lint avant de valider, avec une description d'une ligne. Parce que les compétences sont du Markdown simple, les collègues peuvent les adopter immédiatement.           | Environ cinq minutes par compétence                |
| Générez un guide de configuration à partir de votre propre utilisation | Exécutez `/team-onboarding` dans un projet dans lequel vous avez passé du temps réel. Claude analyse vos sessions récentes, commandes et serveurs MCP, puis produit un guide qu'un nouveau coéquipier peut coller comme son premier message pour rejouer votre configuration. Épinglez-le dans le canal. | Environ deux minutes                               |
| Appairez-vous sur une première tâche                                   | Offrez une seule session d'appairage de quinze minutes à quiconque commence. Un résultat réussi sur leur propre code est plus convaincant que n'importe quelle présentation.                                                                                                                             | Environ quinze minutes par personne                |
| Identifiez le prochain champion                                        | Le collègue qui vous pose le plus de questions est généralement prêt à assumer ce rôle. Transmettez-lui cette page et divisez les responsabilités du canal entre vous.                                                                                                                                   | Négligeable                                        |

<h3 id="thirty-day-playbook">
  Guide de trente jours
</h3>

Si un plan vague est utile, la séquence ci-dessous reflète ce qui tend à fonctionner dans la plupart des équipes. Ajustez librement pour adapter à votre contexte.

<Steps>
  <Step title="Semaine 1 : Ensemencez le canal">
    Créez le canal, épinglez le [Quickstart](/docs/fr/quickstart) et publiez deux ou trois de vos propres exemples avec les prompts inclus.

    **Signal que cela fonctionne :** quelques collègues réagissent ou répondent, et au moins une question est posée dans le canal.
  </Step>

  <Step title="Semaine 2 : Commencez le rythme">
    Commencez le fil de démonstration hebdomadaire, répondez à chaque question publiquement et partagez une compétence personnalisée ou un extrait `CLAUDE.md`.

    **Signal que cela fonctionne :** quelqu'un d'autre que vous publie un exemple du sien.
  </Step>

  <Step title="Semaine 3 : Appairez et consolidez">
    Offrez deux ou trois courtes sessions d'appairage et consolidez les questions et réponses les plus courantes dans un message FAQ épinglé.

    **Signal que cela fonctionne :** vous voyez une utilisation répétée, avec les mêmes collègues revenant plutôt que d'essayer une fois et d'arrêter.
  </Step>

  <Step title="Semaine 4 : Transmettez">
    Identifiez un deuxième champion et partagez un bref résumé de ce qui fonctionne et ce qui ne fonctionne pas avec votre responsable ou administrateur.

    **Signal que cela fonctionne :** les questions dans le canal sont répondues par des personnes autres que vous.
  </Step>
</Steps>

<h3 id="when-someone-wants-to-go-deeper">
  Quand quelqu'un veut aller plus loin
</h3>

Vous êtes l'introduction chaleureuse plutôt que le programme d'intégration. Quand un collègue passe de « devrais-je essayer ceci » à « comment devenir efficace avec ceci », pointez-le vers les pages [Quickstart](/docs/fr/quickstart) et [Flux de travail courants](/docs/fr/common-workflows). Elles contiennent de courtes sections couvrant les fonctionnalités qui sont véritablement utiles mais difficiles à découvrir par soi-même.

<h2 id="respond-to-common-concerns">
  Répondez aux préoccupations courantes
</h2>

Le scepticisme sain est attendu ; les ingénieurs doivent être prudents face aux outils qui touchent leur code. La réponse la plus efficace est rarement d'argumenter le cas général. Au lieu de cela, reconnaissez la préoccupation, offrez un bref reframe et proposez une démonstration concrète sur le propre code de la personne. La plupart des préoccupations sont résolues par une seule expérience réussie.

| Préoccupation                                                           | Réponse suggérée                                                                                                                                                                                                                                  | Preuve à offrir                                                   |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| « Je suis plus rapide sans cela. »                                      | C'est probablement vrai pour le code que la personne écrit régulièrement. Suggérez de l'essayer sur le travail qu'ils ont tendance à éviter : fichiers hérités, services inconnus ou échafaudage de test, où l'effet de levier est le plus élevé. | Chronométrez une tâche fastidieuse des deux façons et comparez.   |
| « Je ne fais pas confiance à l'IA pour toucher le code de production. » | Acceptez qu'aucune modification ne devrait être fusionnée sans être lue. Le mode plan combiné à l'examen diff normal signifie que rien n'est appliqué que l'ingénieur n'a pas inspecté, la même norme que n'importe quelle pull request.          | Démontrez le mode plan sur un fichier réel.                       |
| « Cela rendra les ingénieurs juniors plus faibles. »                    | Utilisé correctement, c'est un expliquant efficace. Encouragez les ingénieurs juniors à demander à Claude d'expliquer un fichier et ses sites d'appel avant de lui demander de changer quoi que ce soit.                                          | Exécutez « Expliquez @file et où il est appelé depuis » ensemble. |
| « J'ai essayé une fois et cela a hallucié. »                            | C'est généralement un problème de contexte plutôt qu'un problème de modèle. @-mentionner les fichiers pertinents, exécuter `/init` et fournir la sortie d'erreur réelle résout généralement cela.                                                 | Réexécutez leur prompt original avec le contexte `@` approprié.   |
| « Nous n'avons pas le temps d'apprendre un autre outil. »               | Claude Code est une commande de terminal plutôt qu'une plateforme. S'il ne retourne pas de valeur dans la première session, il est raisonnable de le mettre de côté.                                                                              | Une installation de deux minutes suivie d'un bug réel.            |

<h2 id="quick-reference-sheet">
  Feuille de référence rapide
</h2>

Les techniques ci-dessous sont celles qui déplacent le plus fiablement quelqu'un d'un premier essai à une utilisation quotidienne. Épinglez ce tableau dans un canal ou partagez-le seul.

| Technique                                 | Comment l'appliquer                                                                                                                                                                        |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Fournissez le bon contexte                | Utilisez les références `@file` ou `@directory/`, ou collez directement la sortie d'erreur ou de journal. Fournir un contexte pertinent est plus efficace qu'une formulation élaborée.     |
| Examinez le plan avant la modification    | Appuyez sur `Maj+Tab` pour entrer en mode plan. Claude décrira les modifications prévues pour votre approbation avant de les exécuter.                                                     |
| Enseignez-lui votre référentiel           | Exécutez `/init` pour générer un fichier `CLAUDE.md`, puis ajoutez vos conventions, commandes de test et tous les répertoires qui ne doivent pas être modifiés. Voir [Memory](/docs/fr/memory). |
| Réutilisez un flux de travail             | Enregistrez un fichier `SKILL.md` dans `.claude/skills/<name>/` pour créer une compétence `/name` que toute l'équipe peut utiliser. Voir [Skills](/docs/fr/skills).                             |
| Restez informé pendant les tâches longues | Configurez un hook Stop pour recevoir une notification de bureau quand une tâche longue se termine. Voir [Hooks](/docs/fr/hooks-guide).                                                         |
| Récupérez d'un résultat incorrect         | Plutôt que de reformuler la demande, collez le test échoué ou la trace de pile à Claude et demandez-lui de traiter cet échec spécifique.                                                   |
| Gardez les modifications chirurgicales    | Demandez un diff, ou spécifiez « ne changez que X ». Claude respecte la portée quand la portée est énoncée.                                                                                |

<Tip>
  Claude Code est mis à jour fréquemment. Vérifiez les détails spécifiques à la version par rapport à la [page d'accueil de la documentation](/docs/fr/overview) avant de distribuer ce matériel en interne.
</Tip>
