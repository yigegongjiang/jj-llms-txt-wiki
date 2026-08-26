> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatiser le travail avec les routines

> Mettez Claude Code en pilotage automatique. Définissez des routines qui s'exécutent selon un calendrier, se déclenchent sur des appels API, ou réagissent aux événements GitHub à partir de l'infrastructure cloud gérée par Anthropic.

<Note>
  Les routines sont en aperçu de recherche. Le comportement, les limites et la surface de l'API peuvent changer.
</Note>

Une routine est une configuration Claude Code enregistrée : une invite, un ou plusieurs référentiels, et un ensemble de [connecteurs](/docs/fr/mcp), empaquetés une fois et exécutés automatiquement. Les routines s'exécutent sur l'infrastructure cloud gérée par Anthropic, de sorte qu'elles continuent de fonctionner lorsque votre ordinateur portable est fermé.

Chaque routine peut avoir un ou plusieurs déclencheurs attachés :

* **Planifiée** : s'exécute selon une cadence récurrente comme toutes les heures, chaque nuit ou chaque semaine, ou une seule fois à un moment futur spécifique
* **API** : se déclenche à la demande en envoyant une requête HTTP POST à un point de terminaison par routine avec un jeton porteur
* **GitHub** : s'exécute automatiquement en réponse aux événements du référentiel tels que les demandes de tirage ou les versions

Une seule routine peut combiner des déclencheurs. Par exemple, une routine d'examen des PR peut s'exécuter chaque nuit, se déclencher à partir d'un script de déploiement, et réagir également à chaque nouvelle PR.

Les routines sont disponibles sur les plans Pro, Max, Team et Enterprise avec [Claude Code sur le web](/docs/fr/claude-code-on-the-web) activé. Créez et gérez-les sur [claude.ai/code/routines](https://claude.ai/code/routines), ou à partir de la CLI avec `/schedule`.

Les administrateurs Team et Enterprise peuvent désactiver les routines pour tous les membres avec le bouton bascule Routines sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Lorsqu'elles sont désactivées, les routines existantes cessent de s'exécuter et les membres ne peuvent pas en créer de nouvelles.

Cette page couvre la création d'une routine, la configuration de chaque type de déclencheur, la gestion des exécutions et la façon dont les limites d'utilisation s'appliquent.

<h2 id="example-use-cases">
  Exemples de cas d'usage
</h2>

Chaque exemple associe un type de déclencheur au type de travail pour lequel les routines sont adaptées : sans surveillance, répétable et lié à un résultat clair.

**Maintenance du carnet de commandes.** Un déclencheur de calendrier s'exécute chaque soir en semaine sur votre suivi des problèmes via un connecteur. La routine lit les problèmes ouverts depuis la dernière exécution, applique des étiquettes, assigne les propriétaires en fonction de la zone de code référencée, et publie un résumé sur Slack afin que l'équipe commence la journée avec une file d'attente organisée.

**Triage des alertes.** Votre outil de surveillance appelle le point de terminaison API de la routine lorsqu'un seuil d'erreur est dépassé, en transmettant le corps de l'alerte en tant que `text`. La routine extrait la trace de pile, la met en corrélation avec les commits récents du référentiel, et ouvre une demande de tirage brouillon avec un correctif proposé et un lien vers l'alerte. L'équipe d'astreinte examine la PR au lieu de commencer à partir d'un terminal vierge.

**Examen du code sur mesure.** Un déclencheur GitHub s'exécute sur `pull_request.opened`. La routine applique la liste de contrôle d'examen de votre équipe, laisse des commentaires en ligne pour les problèmes de sécurité, de performance et de style, et ajoute un commentaire récapitulatif afin que les examinateurs humains puissent se concentrer sur la conception plutôt que sur les vérifications mécaniques.

**Vérification du déploiement.** Votre pipeline CD appelle le point de terminaison API de la routine après chaque déploiement en production. La routine exécute des vérifications de fumée sur la nouvelle version, analyse les journaux d'erreurs pour détecter les régressions, et publie un feu vert ou un feu rouge sur le canal de version avant la fermeture de la fenêtre de déploiement.

**Dérive de la documentation.** Un déclencheur de calendrier s'exécute chaque semaine. La routine analyse les PR fusionnées depuis la dernière exécution, signale la documentation qui référence les API modifiées, et ouvre des PR de mise à jour par rapport au référentiel de documentation pour qu'un éditeur les examine.

**Portage de bibliothèque.** Un déclencheur GitHub s'exécute sur `pull_request.closed` filtré pour les PR fusionnées dans un référentiel SDK. La routine porte la modification vers un SDK parallèle dans une autre langue et ouvre une PR correspondante, en gardant les deux bibliothèques synchronisées sans qu'un humain ne réimplémente chaque modification.

Les sections ci-dessous expliquent comment créer une routine et configurer chacun de ces types de déclencheurs.

<h2 id="create-a-routine">
  Créer une routine
</h2>

Créez une routine à partir du web sur [claude.ai/code/routines](https://claude.ai/code/routines), à partir de l'application de bureau, ou à partir de la CLI. Les trois surfaces écrivent dans le même compte cloud, de sorte qu'une routine que vous créez dans l'une d'elles apparaît immédiatement dans les autres. Dans l'application de bureau, cliquez sur **Routines** dans la barre latérale, puis sur **New routine**, et choisissez **Remote** ; choisir **Local** à la place crée une [tâche planifiée de bureau](/docs/fr/desktop-scheduled-tasks), qui s'exécute sur votre machine plutôt que dans le cloud.

Le formulaire de création configure l'invite de la routine, les référentiels, l'environnement, les connecteurs et les déclencheurs.

Les routines s'exécutent de manière autonome en tant que sessions cloud Claude Code complètes : il n'y a pas de sélecteur de mode de permission et pas d'invites d'approbation pendant une exécution. La session peut exécuter des commandes shell, utiliser des [skills](/docs/fr/skills) validées dans le référentiel cloné, et appeler tous les connecteurs que vous incluez. Ce qu'une routine peut atteindre est déterminé par les référentiels que vous sélectionnez et leur paramètre de branche-push, l'accès réseau et les variables de l'[environnement](/docs/fr/claude-code-on-the-web#the-cloud-environment), et les connecteurs que vous incluez. Limitez chacun de ces éléments à ce dont la routine a réellement besoin.

Les routines appartiennent à votre compte claude.ai individuel. Elles ne sont pas partagées avec les coéquipiers, et elles comptent dans votre allocation quotidienne d'exécutions. Tout ce qu'une routine fait via votre identité GitHub connectée ou les connecteurs apparaît comme vous : les commits et les demandes de tirage portent votre utilisateur GitHub, et les messages Slack, les tickets Linear ou d'autres actions de connecteur utilisent vos comptes liés pour ces services.

<h3 id="create-from-the-web">
  Créer à partir du web
</h3>

<Steps>
  <Step title="Ouvrir le formulaire de création">
    Visitez [claude.ai/code/routines](https://claude.ai/code/routines) et cliquez sur **New routine**.
  </Step>

  <Step title="Nommer la routine et écrire l'invite">
    Donnez à la routine un nom descriptif et écrivez l'invite que Claude exécute à chaque fois. L'invite est la partie la plus importante : la routine s'exécute de manière autonome, donc l'invite doit être autonome et explicite sur ce qu'il faut faire et à quoi ressemble le succès.

    L'entrée d'invite inclut un sélecteur de modèle. Claude utilise le modèle sélectionné à chaque exécution.
  </Step>

  <Step title="Sélectionner les référentiels">
    Ajoutez un ou plusieurs référentiels GitHub pour que Claude y travaille. Chaque référentiel est cloné au début d'une exécution, en commençant par la branche par défaut. Claude crée des branches préfixées par `claude/` pour ses modifications.
  </Step>

  <Step title="Sélectionner un environnement">
    Choisissez un [environnement cloud](/docs/fr/claude-code-on-the-web#the-cloud-environment) pour la routine. Les environnements contrôlent ce à quoi la session cloud a accès :

    * **Network access** : définissez le niveau d'accès à Internet disponible pendant chaque exécution
    * **Environment variables** : fournissez des clés API, des jetons ou d'autres secrets que Claude peut utiliser
    * **Setup script** : installez les dépendances et les outils dont la routine a besoin. Le résultat est [mis en cache](/docs/fr/claude-code-on-the-web#environment-caching), de sorte que le script ne se réexécute pas à chaque session

    Un environnement **Default** est fourni avec un accès réseau **Trusted**, qui permet l'[ensemble par défaut](/docs/fr/claude-code-on-the-web#default-allowed-domains) des registres de paquets, des API de fournisseurs cloud, des registres de conteneurs et des domaines de développement courants, mais bloque tout le reste. Si votre routine doit atteindre vos propres services ou un domaine en dehors de cette liste, modifiez l'[accès réseau](/docs/fr/claude-code-on-the-web#network-access) de l'environnement avant d'exécuter. Pour utiliser un environnement séparé, [créez-en un](/docs/fr/claude-code-on-the-web#configure-your-environment) d'abord.
  </Step>

  <Step title="Sélectionner un déclencheur">
    Sous **Select a trigger**, choisissez comment la routine démarre. Vous pouvez choisir un type de déclencheur ou en combiner plusieurs.

    <Tabs>
      <Tab title="Schedule">
        Choisissez une fréquence prédéfinie pour une exécution récurrente, ou planifiez une exécution unique à un moment précis. Consultez [Add a schedule trigger](#add-a-schedule-trigger) pour la gestion des fuseaux horaires, l'échelonnement, les intervalles cron personnalisés et les exécutions uniques.
      </Tab>

      <Tab title="GitHub event">
        Sélectionnez le référentiel, l'événement auquel réagir et les filtres optionnels. Consultez [Add a GitHub trigger](#add-a-github-trigger) pour la liste complète des événements pris en charge et des champs de filtre.
      </Tab>

      <Tab title="API">
        Sélectionnez **API** ici, puis enregistrez la routine. L'URL et le jeton sont générés après l'enregistrement de la routine, car ils dépendent de l'ID de la routine. Consultez [Add an API trigger](#add-an-api-trigger) pour copier l'URL et générer un jeton.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Examiner les connecteurs et les permissions">
    Les onglets **Connectors** et **Permissions** en bas du formulaire contrôlent ce que la routine peut atteindre.

    Sous Connectors, tous vos [connecteurs MCP](/docs/fr/mcp) connectés sont inclus par défaut. Supprimez tous ceux dont la routine n'a pas besoin. Claude peut utiliser tous les outils d'un connecteur inclus, y compris les écritures, sans demander de permission pendant une exécution.

    Sous Permissions, activez **Allow unrestricted branch pushes** pour tout référentiel où Claude devrait pouvoir pousser vers des branches existantes au lieu de seulement des branches préfixées par `claude/`.
  </Step>

  <Step title="Créer la routine">
    Cliquez sur **Create**. La routine apparaît dans la liste et s'exécute la prochaine fois que l'un de ses déclencheurs correspond. Pour démarrer une exécution immédiatement, cliquez sur **Run now** sur la page de détail de la routine.

    Chaque exécution crée une nouvelle session aux côtés de vos autres sessions, où vous pouvez voir ce que Claude a fait, examiner les modifications et créer une demande de tirage.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Créer à partir de la CLI
</h3>

Exécutez `/schedule` dans n'importe quelle session pour créer une routine planifiée de manière conversationnelle. Vous pouvez également transmettre une description directement, pour une routine récurrente comme `/schedule daily PR review at 9am` ou une exécution unique comme `/schedule clean up feature flag in one week`. Claude parcourt les mêmes informations que le formulaire web collecte, puis enregistre la routine sur votre compte.

Une exécution réussie ressemble à une conversation : Claude pose des questions de suivi sur l'horaire, les référentiels et l'invite avant d'enregistrer. Si Claude répond plutôt que vous devez vous authentifier ou qu'il ne peut pas se connecter à votre compte claude.ai distant, aucune routine n'a été créée ; consultez [Troubleshooting](#troubleshooting).

`/schedule` dans la CLI crée uniquement des routines planifiées. Pour ajouter un déclencheur API ou GitHub, modifiez la routine sur le web sur [claude.ai/code/routines](https://claude.ai/code/routines).

La CLI prend également en charge la gestion des routines existantes. Exécutez `/schedule list` pour voir toutes les routines, `/schedule update` pour en modifier une, ou `/schedule run` pour la déclencher immédiatement.

<h2 id="configure-triggers">
  Configurer les déclencheurs
</h2>

Une routine démarre lorsque l'un de ses déclencheurs correspond. Vous pouvez attacher n'importe quelle combinaison de déclencheurs de calendrier, API et GitHub à la même routine, et les ajouter ou les supprimer à tout moment à partir de la section **Select a trigger** du formulaire d'édition de la routine.

<h3 id="add-a-schedule-trigger">
  Ajouter un déclencheur de calendrier
</h3>

Un déclencheur de calendrier exécute la routine selon une cadence récurrente, ou une seule fois à un moment futur spécifique. Choisissez une fréquence prédéfinie dans la section **Select a trigger** : toutes les heures, quotidienne, les jours de semaine ou hebdomadaire. Les heures sont entrées dans votre fuseau horaire local et converties automatiquement, de sorte que la routine s'exécute à cette heure murale indépendamment de l'endroit où se trouve l'infrastructure cloud.

Les exécutions peuvent démarrer quelques minutes après l'heure planifiée en raison de l'échelonnement. Le décalage est cohérent pour chaque routine.

Pour un intervalle personnalisé tel que toutes les deux heures ou le premier de chaque mois, choisissez la prédéfinie la plus proche dans le formulaire, puis exécutez `/schedule update` dans la CLI pour définir une expression cron spécifique. L'intervalle minimum est d'une heure ; les expressions qui s'exécutent plus fréquemment sont rejetées.

<h4 id="schedule-a-one-off-run">
  Planifier une exécution unique
</h4>

Une exécution unique planifiée déclenche la routine une seule fois à un horodatage spécifique. Utilisez-la pour vous rappeler plus tard dans la semaine, pour ouvrir une PR de nettoyage après la fin d'un déploiement, ou pour lancer une tâche de suivi lorsqu'une modification en amont arrive. Après le déclenchement de la routine, elle se désactive automatiquement et l'interface utilisateur web la marque comme **Ran**. Pour l'exécuter à nouveau, modifiez la routine et définissez une nouvelle heure unique.

<Note>
  La planification unique à partir de la CLI est déployée progressivement et peut ne pas être disponible sur votre compte pour le moment. Si `/schedule` n'offre que des calendriers récurrents, créez l'exécution unique à partir du web sur [claude.ai/code/routines](https://claude.ai/code/routines) à la place.
</Note>

Créez une exécution unique à partir de la CLI en décrivant l'heure en langage naturel. Claude résout la phrase par rapport à l'heure actuelle et confirme l'horodatage absolu avant d'enregistrer.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

La même conversion locale-UTC que pour les calendriers récurrents s'applique aux horodatages uniques.

Les exécutions uniques ne comptent pas par rapport au plafond quotidien d'exécution de routine. Elles consomment l'utilisation d'abonnement régulière de votre plan comme n'importe quelle autre session. Consultez [Usage and limits](#usage-and-limits) pour plus de détails.

<h3 id="add-an-api-trigger">
  Ajouter un déclencheur API
</h3>

Un déclencheur API donne à une routine un point de terminaison HTTP dédié. POSTer sur le point de terminaison avec le jeton porteur de la routine démarre une nouvelle session et retourne une URL de session. Utilisez ceci pour intégrer Claude Code dans les systèmes d'alerte, les pipelines de déploiement, les outils internes ou n'importe où vous pouvez faire une requête HTTP authentifiée.

Les déclencheurs API sont ajoutés à une routine existante à partir du web. La CLI ne peut actuellement pas créer ou révoquer des jetons.

<Steps>
  <Step title="Ouvrir la routine pour l'édition">
    Allez sur [claude.ai/code/routines](https://claude.ai/code/routines), cliquez sur la routine que vous souhaitez déclencher via API, puis cliquez sur l'icône de crayon pour ouvrir **Edit routine**.
  </Step>

  <Step title="Ajouter un déclencheur API">
    Faites défiler jusqu'à la section **Select a trigger** sous la boîte **Instructions**, cliquez sur **Add another trigger**, et choisissez **API**.
  </Step>

  <Step title="Copier l'URL et générer un jeton">
    La fenêtre modale affiche l'URL de cette routine ainsi qu'un exemple de commande curl. Copiez l'URL, puis cliquez sur **Generate token** et copiez le jeton immédiatement. Le jeton est affiché une seule fois et ne peut pas être récupéré ultérieurement, alors stockez-le quelque part de sécurisé comme le magasin de secrets de votre outil d'alerte.
  </Step>

  <Step title="Appeler le point de terminaison">
    Envoyez le jeton dans l'en-tête `Authorization: Bearer` lorsque vous POSTez sur l'URL. La section [Trigger a routine](#trigger-a-routine) ci-dessous montre un exemple complet.
  </Step>
</Steps>

Chaque routine a son propre jeton, limité au déclenchement de cette routine uniquement. Pour le faire tourner ou le révoquer, retournez à la même fenêtre modale et cliquez sur **Regenerate** ou **Revoke**.

<h4 id="trigger-a-routine">
  Déclencher une routine
</h4>

Envoyez une requête POST au point de terminaison `/fire` avec le jeton porteur dans l'en-tête `Authorization`. Le corps de la requête accepte un champ `text` optionnel pour le contexte spécifique à l'exécution tel qu'un corps d'alerte ou un journal défaillant, transmis à la routine aux côtés de son invite enregistrée. La valeur est du texte libre et n'est pas analysée : si vous envoyez JSON ou une autre charge utile structurée, la routine la reçoit comme une chaîne littérale.

L'exemple ci-dessous déclenche une routine à partir d'un shell. L'ID de routine et le jeton affichés sont des espaces réservés : remplacez-les par l'URL et le jeton que vous avez copiés lors de l'[ajout du déclencheur API](#add-an-api-trigger), sinon la requête échoue avec une erreur d'authentification `401` :

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

Une requête réussie retourne un corps JSON avec le nouvel ID de session et l'URL :

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

Ouvrez l'URL de session dans un navigateur pour regarder l'exécution en temps réel, examiner les modifications ou continuer la conversation manuellement.

<Warning>
  Le point de terminaison `/fire` est livré sous l'en-tête bêta `experimental-cc-routine-2026-04-01`. Les formes de requête et de réponse, les limites de débit et la sémantique des jetons peuvent changer pendant que la fonctionnalité est en aperçu de recherche. Les modifications de rupture sont livrées derrière les nouvelles versions d'en-tête bêta datées, et les deux versions d'en-tête précédentes les plus récentes continuent de fonctionner afin que les appelants aient le temps de migrer.
</Warning>

<h4 id="api-reference">
  Référence API
</h4>

Pour la référence API complète, y compris toutes les réponses d'erreur, les règles de validation et les limites de champs, consultez [Trigger a routine via API](https://platform.claude.com/docs/fr/api/claude-code/routines-fire) dans la documentation de la plateforme Claude.

Le point de terminaison `/fire` est disponible pour les utilisateurs de claude.ai uniquement et ne fait pas partie de la surface de l'API Claude Platform.

<h3 id="add-a-github-trigger">
  Ajouter un déclencheur GitHub
</h3>

Un déclencheur GitHub démarre une nouvelle session automatiquement lorsqu'un événement correspondant se produit sur un référentiel connecté. Chaque événement correspondant démarre sa propre session.

<Note>
  Pendant l'aperçu de recherche, les événements webhook GitHub sont soumis à des limites horaires par routine et par compte. Les événements au-delà de la limite sont supprimés jusqu'à la réinitialisation de la fenêtre. Consultez vos limites actuelles sur [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

Les déclencheurs GitHub sont configurés uniquement à partir de l'interface utilisateur web.

<Steps>
  <Step title="Ouvrir la routine pour l'édition">
    Allez sur [claude.ai/code/routines](https://claude.ai/code/routines), cliquez sur la routine, puis cliquez sur l'icône de crayon pour ouvrir **Edit routine**.
  </Step>

  <Step title="Ajouter un déclencheur d'événement GitHub">
    Faites défiler jusqu'à la section **Select a trigger**, cliquez sur **Add another trigger**, et choisissez **GitHub event**.
  </Step>

  <Step title="Installer l'application Claude GitHub">
    L'application Claude GitHub doit être installée sur le référentiel auquel vous souhaitez vous abonner. La configuration du déclencheur vous invite à l'installer si ce n'est pas déjà fait.

    <Note>
      L'exécution de `/web-setup` dans la CLI accorde l'accès au référentiel pour le clonage, mais elle n'installe pas l'application Claude GitHub et n'active pas la livraison des webhooks. Les déclencheurs GitHub nécessitent l'installation de l'application Claude GitHub, que la configuration du déclencheur vous invite à faire.
    </Note>
  </Step>

  <Step title="Configurer le déclencheur">
    Sélectionnez le référentiel, choisissez un événement dans la liste des [événements pris en charge](#supported-events), et ajoutez éventuellement des filtres. Enregistrez le déclencheur.
  </Step>
</Steps>

<h4 id="supported-events">
  Événements pris en charge
</h4>

Les déclencheurs GitHub peuvent s'abonner à l'une des catégories d'événements suivantes. Dans chaque catégorie, vous pouvez choisir une action spécifique, comme `pull_request.opened`, ou réagir à toutes les actions de la catégorie.

| Événement    | Se déclenche quand                                                                     |
| :----------- | :------------------------------------------------------------------------------------- |
| Pull request | Une PR est ouverte, fermée, assignée, étiquetée, synchronisée ou autrement mise à jour |
| Release      | Une version est créée, publiée, modifiée ou supprimée                                  |

<h4 id="filter-pull-requests">
  Filtrer les demandes de tirage
</h4>

Utilisez les filtres pour affiner les demandes de tirage qui démarrent une nouvelle session. Toutes les conditions de filtre doivent correspondre pour que la routine se déclenche. Les champs de filtre disponibles sont :

| Filtre      | Correspond à                                  |
| :---------- | :-------------------------------------------- |
| Author      | Nom d'utilisateur GitHub de l'auteur de la PR |
| Title       | Texte du titre de la PR                       |
| Body        | Texte de la description de la PR              |
| Base branch | Branche ciblée par la PR                      |
| Head branch | Branche d'où provient la PR                   |
| Labels      | Étiquettes appliquées à la PR                 |
| Is draft    | Si la PR est à l'état brouillon               |
| Is merged   | Si la PR a été fusionnée                      |

Chaque filtre associe un champ à un opérateur : égal à, contient, commence par, est l'un de, n'est pas l'un de, ou correspond à regex.

L'opérateur `matches regex` teste la valeur de champ entière, pas une sous-chaîne à l'intérieur. Pour correspondre à n'importe quel titre contenant `hotfix`, écrivez `.*hotfix.*`. Sans le `.*` environnant, le filtre correspond uniquement à un titre qui est exactement `hotfix` sans rien avant ou après. Pour la correspondance de sous-chaîne littérale sans syntaxe regex, utilisez l'opérateur `contains` à la place.

Quelques exemples de combinaisons de filtres :

* **Examen du module d'authentification** : branche de base `main`, branche de tête contient `auth-provider`. Envoie toute PR qui touche l'authentification à un examinateur ciblé.
* **Prêt pour l'examen uniquement** : est brouillon est `false`. Ignore les brouillons afin que la routine s'exécute uniquement lorsque la PR est prête pour l'examen.
* **Portage contrôlé par étiquette** : les étiquettes incluent `needs-backport`. Déclenche une routine de portage vers une autre branche uniquement lorsqu'un responsable marque la PR.

<h4 id="how-sessions-map-to-events">
  Comment les sessions correspondent aux événements
</h4>

Chaque événement GitHub correspondant démarre une nouvelle session. La réutilisation de session entre les événements n'est pas disponible pour les routines déclenchées par GitHub, de sorte que deux mises à jour de PR produisent deux sessions indépendantes.

<h2 id="manage-routines">
  Gérer les routines
</h2>

Cliquez sur une routine dans la liste pour ouvrir sa page de détail. La page de détail affiche les référentiels de la routine, les connecteurs, l'invite, le calendrier, les jetons API, les déclencheurs GitHub et une liste des exécutions passées.

<h3 id="view-and-interact-with-runs">
  Afficher et interagir avec les exécutions
</h3>

Cliquez sur n'importe quelle exécution pour l'ouvrir en tant que session complète. À partir de là, vous pouvez voir ce que Claude a fait, examiner les modifications, créer une demande de tirage ou continuer la conversation. Chaque session d'exécution fonctionne comme n'importe quelle autre session : utilisez le menu déroulant à côté du titre de la session pour la renommer, l'archiver ou la supprimer.

<Note>
  Un statut vert dans la liste des exécutions signifie que la session a démarré et s'est terminée sans erreur d'infrastructure. Cela ne signifie pas que la tâche dans votre invite a réussi. Ouvrez l'exécution pour lire la transcription et confirmer ce que Claude a réellement fait. Les demandes réseau bloquées, les outils de connecteur manquants et les défaillances au niveau des tâches s'affichent là plutôt que dans l'indicateur de statut.
</Note>

<h3 id="edit-and-control-routines">
  Éditer et contrôler les routines
</h3>

À partir de la page de détail de la routine, vous pouvez :

* Cliquer sur **Run now** pour démarrer une exécution immédiatement sans attendre l'heure planifiée suivante.
* Utiliser le bouton bascule dans la section **Repeats** pour mettre en pause ou reprendre le calendrier. Les routines en pause conservent leur configuration mais ne s'exécutent pas jusqu'à ce que vous les réactiviez.
* Cliquer sur l'icône de crayon pour ouvrir **Edit routine** et modifier le nom, l'invite, les référentiels, l'environnement, les connecteurs ou l'un des déclencheurs de la routine. La section **Select a trigger** est l'endroit où vous ajoutez ou supprimez les calendriers, les jetons API et les déclencheurs d'événements GitHub.
* Cliquer sur l'icône de suppression pour supprimer la routine. Les sessions passées créées par la routine restent dans votre liste de sessions.

<h3 id="repositories-and-branch-permissions">
  Référentiels et permissions de branche
</h3>

Les routines ont besoin d'un accès GitHub pour cloner les référentiels. Lorsque vous créez une routine à partir de la CLI avec `/schedule`, Claude vérifie si votre compte a GitHub connecté et vous invite à exécuter `/web-setup` si ce n'est pas le cas. Consultez [Options d'authentification GitHub](/docs/fr/claude-code-on-the-web#github-authentication-options) pour les deux façons d'accorder l'accès.

Chaque référentiel que vous ajoutez est cloné à chaque exécution. Claude commence à partir de la branche par défaut du référentiel sauf si votre invite spécifie le contraire.

Par défaut, Claude ne peut pousser que vers les branches préfixées par `claude/`. Cela empêche les routines de modifier accidentellement les branches protégées ou longue durée. Pour supprimer cette restriction pour un référentiel spécifique, activez **Allow unrestricted branch pushes** pour ce référentiel lors de la création ou de l'édition de la routine.

<h3 id="connectors">
  Connecteurs
</h3>

Les routines peuvent utiliser vos connecteurs MCP connectés pour lire et écrire dans les services externes pendant chaque exécution. Par exemple, une routine qui trie les demandes d'assistance peut lire à partir d'un canal Slack et créer des problèmes dans Linear.

Les connecteurs sont les [intégrations claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) sur votre compte. Les serveurs MCP que vous avez ajoutés localement dans la CLI avec `claude mcp add` sont stockés sur votre machine plutôt que sur votre compte claude.ai, donc ils n'apparaissent pas dans la liste des connecteurs. Pour utiliser l'un de ces serveurs dans une routine, ajoutez-le en tant que connecteur sur [claude.ai/customize/connectors](https://claude.ai/customize/connectors), ou déclarez-le dans un [`.mcp.json`](/docs/fr/mcp#project-scope) engagé afin qu'il fasse partie du référentiel cloné.

Lorsque vous créez une routine, tous vos connecteurs actuellement connectés sont inclus par défaut. Supprimez tous ceux qui ne sont pas nécessaires pour limiter les outils auxquels Claude a accès pendant l'exécution. Vous pouvez également ajouter des connecteurs directement à partir du formulaire de routine.

Pour gérer ou ajouter des connecteurs en dehors du formulaire de routine, visitez **Settings > Connectors** sur claude.ai ou utilisez `/schedule update` dans la CLI.

<h3 id="environments-and-network-access">
  Environnements et accès réseau
</h3>

Chaque routine s'exécute dans un [environnement cloud](/docs/fr/claude-code-on-the-web#the-cloud-environment) qui contrôle l'accès réseau, les variables d'environnement et les scripts de configuration. La routine hérite de la politique réseau de l'environnement à chaque exécution.

L'environnement **Default** utilise l'accès réseau **Trusted** : la [liste d'autorisation par défaut](/docs/fr/claude-code-on-the-web#default-allowed-domains) des registres de paquets, des API des fournisseurs de cloud, des registres de conteneurs et des domaines de développement courants est accessible, mais les domaines arbitraires ne le sont pas. Les demandes sortantes vers d'autres hôtes échouent avec `403` et `x-deny-reason: host_not_allowed`. Le trafic des connecteurs MCP est acheminé via les serveurs d'Anthropic, donc les connecteurs que vous ajoutez à la routine fonctionnent sans ajouter leurs hôtes aux **Allowed domains**. Supprimez tous les connecteurs dont vous n'avez pas besoin sous [Connecteurs](#connectors).

Pour autoriser des domaines supplémentaires :

<Steps>
  <Step title="Ouvrir la routine pour l'édition">
    Sur la page de détail de la routine, cliquez sur l'icône de crayon pour ouvrir **Edit routine**.
  </Step>

  <Step title="Ouvrir le sélecteur d'environnement">
    Sous la zone **Instructions**, sélectionnez l'icône cloud affichant le nom de votre environnement, tel que **Default**.
  </Step>

  <Step title="Ouvrir les paramètres d'environnement">
    Survolez l'environnement dans la liste et cliquez sur l'icône de paramètres qui apparaît à droite.
  </Step>

  <Step title="Modifier le niveau d'accès réseau">
    Dans la boîte de dialogue **Update cloud environment**, modifiez **Network access** en **Custom** et entrez vos domaines dans **Allowed domains**. Cochez **Also include default list of common package managers** pour conserver la [liste d'autorisation par défaut](/docs/fr/claude-code-on-the-web#default-allowed-domains) aux côtés de vos domaines personnalisés. Sélectionnez **Full** à la place pour un accès sans restriction.
  </Step>

  <Step title="Enregistrer">
    Cliquez sur **Save changes**. La nouvelle politique s'applique à partir de la prochaine exécution.
  </Step>
</Steps>

Consultez [Accès réseau](/docs/fr/claude-code-on-the-web#network-access) pour plus de détails sur les niveaux d'accès et la liste d'autorisation par défaut.

<h2 id="usage-and-limits">
  Utilisation et limites
</h2>

Les routines réduisent l'utilisation de l'abonnement de la même manière que les sessions interactives. En plus des limites d'abonnement standard, les routines ont un plafond quotidien sur le nombre d'exécutions qui peuvent démarrer par compte. Consultez votre consommation actuelle et vos exécutions de routine quotidiennes restantes sur [claude.ai/code/routines](https://claude.ai/code/routines) ou [claude.ai/settings/usage](https://claude.ai/settings/usage).

Lorsqu'une routine atteint le plafond quotidien ou votre limite d'utilisation d'abonnement, les organisations avec crédits d'utilisation activés peuvent continuer à exécuter les routines sur dépassement mesuré. Sans crédits d'utilisation, les exécutions supplémentaires sont rejetées jusqu'à la réinitialisation de la fenêtre. Activez les crédits d'utilisation à partir de **Settings > Billing** sur claude.ai.

Les exécutions ponctuelles ne comptent pas par rapport au plafond quotidien des routines. Elles réduisent votre utilisation d'abonnement régulière comme toute autre session, mais elles sont exemptes de l'allocation quotidienne d'exécutions de routine par compte.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="/schedule-returns-unknown-command">
  `/schedule` affiche « Commande inconnue »
</h3>

L'interface de ligne de commande masque `/schedule` lorsque l'une de ses exigences n'est pas satisfaite : le menu de commande affiche `Aucune commande ne correspond à "/schedule"` pendant que vous tapez, et la soumettre retourne `Commande inconnue : /schedule`. La cause est généralement l'une des suivantes :

* Vous êtes authentifié avec une clé API Console ou un fournisseur cloud tel qu'Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. `/schedule` nécessite une connexion par abonnement claude.ai. Si `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN` est défini dans votre shell, ou si `apiKeyHelper` est défini dans `settings.json`, supprimez-le d'abord, car ces paramètres ont la priorité sur une connexion claude.ai
* `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` ou `DISABLE_GROWTHBOOK` est défini dans votre environnement shell ou dans le bloc `env` d'un [fichier `settings.json`](/docs/fr/settings#available-settings). Ces paramètres désactivent la récupération des drapeaux de fonctionnalités, dont `/schedule` dépend
* Vous êtes dans une session Claude Code sur le web. Gérez les routines à partir de l'[interface web](https://claude.ai/code/routines) à la place

Vous pouvez toujours créer et gérer les routines sur [claude.ai/code/routines](https://claude.ai/code/routines) indépendamment de la façon dont l'interface de ligne de commande est configurée.

<h3 id="/schedule-asks-you-to-authenticate">
  `/schedule` vous demande de vous authentifier
</h3>

Si `/schedule` s'exécute mais que Claude répond que vous devez d'abord vous authentifier avec un compte claude.ai, l'interface de ligne de commande n'a pas de connexion claude.ai stockée. Les comptes API ne sont pas pris en charge pour les routines. Exécutez `/login`, connectez-vous avec votre compte claude.ai, puis exécutez `/schedule` à nouveau.

<h3 id="routines-are-disabled-by-your-organization’s-policy">
  « Les routines sont désactivées par la politique de votre organisation »
</h3>

Un propriétaire de votre organisation Team ou Enterprise a probablement désactivé le bouton bascule **Routines** sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Il s'agit d'un paramètre d'organisation côté serveur, il ne peut donc pas être remplacé par votre configuration locale. Demandez à un propriétaire d'activer les routines pour votre organisation.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [`/loop` et planification en session](/docs/fr/scheduled-tasks) : planifiez les tâches locales dans une session CLI ouverte
* [Tâches planifiées de bureau](/docs/fr/desktop-scheduled-tasks) : tâches planifiées locales qui s'exécutent sur votre machine avec accès aux fichiers locaux
* [Environnement cloud](/docs/fr/claude-code-on-the-web#the-cloud-environment) : configurez l'environnement d'exécution pour les sessions cloud
* [Connecteurs MCP](/docs/fr/mcp) : connectez les services externes comme Slack, Linear et Google Drive
* [GitHub Actions](/docs/fr/github-actions) : exécutez Claude dans votre pipeline CI sur les événements du référentiel
