> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Suivre l'utilisation de l'équipe avec l'analytique

> Consultez les métriques d'utilisation de Claude Code, suivez l'adoption et mesurez la vélocité d'ingénierie dans le tableau de bord analytique.

Claude Code fournit des tableaux de bord analytiques pour aider les organisations à comprendre les modèles d'utilisation des développeurs, suivre les métriques de contribution et mesurer l'impact de Claude Code sur la vélocité d'ingénierie. Accédez au tableau de bord pour votre plan :

| Plan                          | URL du tableau de bord                                                     | Inclut                                                                                                    | En savoir plus                                       |
| ----------------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | Métriques d'utilisation, métriques de contribution avec intégration GitHub, classement, export de données | [Détails](#access-analytics-for-team-and-enterprise) |
| API (Claude Console)          | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | Métriques d'utilisation, suivi des dépenses, insights d'équipe                                            | [Détails](#access-analytics-for-api-customers)       |

<h2 id="access-analytics-for-team-and-enterprise">
  Accéder à l'analytique pour Teams et Enterprise
</h2>

Accédez à [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code). Les administrateurs et propriétaires peuvent consulter le tableau de bord.

Le tableau de bord Teams et Enterprise inclut :

* **Métriques d'utilisation** : lignes de code acceptées, taux d'acceptation des suggestions, utilisateurs actifs quotidiens et sessions
* **Métriques de contribution** : PRs et lignes de code livrées avec l'assistance de Claude Code, avec [intégration GitHub](#enable-contribution-metrics)
* **Classement** : principaux contributeurs classés par utilisation de Claude Code
* **Export de données** : téléchargez les données de contribution au format CSV pour des rapports personnalisés

Pour les comptages de tokens par utilisateur et les estimations de coûts, configurez [l'export OpenTelemetry](/docs/fr/monitoring-usage), ou exportez le [rapport de dépenses](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) à partir des paramètres analytiques de votre organisation, qui répertorie l'utilisation des tokens et les dépenses estimées en crédits d'utilisation par utilisateur et par modèle.

<h3 id="enable-contribution-metrics">
  Activer les métriques de contribution
</h3>

<Note>
  Les métriques de contribution sont en bêta publique et disponibles sur les plans Claude for Teams et Claude for Enterprise. Ces métriques couvrent uniquement les utilisateurs au sein de votre organisation claude.ai. L'utilisation via l'API Claude Console ou les intégrations tierces n'est pas incluse.
</Note>

Les données d'utilisation et d'adoption sont disponibles pour tous les comptes Claude for Teams et Claude for Enterprise. Les métriques de contribution nécessitent une configuration supplémentaire pour connecter votre organisation GitHub.

Vous devez avoir le rôle Propriétaire pour configurer les paramètres analytiques. Un administrateur GitHub doit installer l'application GitHub.

<Warning>
  Les métriques de contribution ne sont pas disponibles pour les organisations avec [Zero Data Retention](/docs/fr/zero-data-retention) activé. Le tableau de bord analytique affichera uniquement les métriques d'utilisation.
</Warning>

<Steps>
  <Step title="Installer l'application GitHub">
    Un administrateur GitHub installe l'application Claude GitHub sur le compte GitHub de votre organisation à [github.com/apps/claude](https://github.com/apps/claude).
  </Step>

  <Step title="Activer l'analytique Claude Code">
    Un propriétaire Claude accède à [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) et active la fonctionnalité d'analytique Claude Code.
  </Step>

  <Step title="Activer l'analytique GitHub">
    Sur la même page, activez le bouton bascule ' Analytique GitHub '.
  </Step>

  <Step title="S'authentifier avec GitHub">
    Complétez le flux d'authentification GitHub et sélectionnez les organisations GitHub à inclure dans l'analyse.
  </Step>
</Steps>

Les données apparaissent généralement dans les 24 heures suivant l'activation, avec des mises à jour quotidiennes. Si aucune donnée n'apparaît, vous pouvez voir l'un de ces messages :

* **« Application GitHub requise »** : installez l'application GitHub pour consulter les métriques de contribution
* **« Traitement des données en cours »** : revenez dans quelques jours et confirmez que l'application GitHub est installée si les données n'apparaissent pas

Les métriques de contribution prennent en charge GitHub Cloud et GitHub Enterprise Server.

<h3 id="review-summary-metrics">
  Examiner les métriques récapitulatives
</h3>

<Note>
  Ces métriques sont délibérément conservatrices et représentent une sous-estimation de l'impact réel de Claude Code. Seules les lignes et les PRs où il y a une grande confiance dans l'implication de Claude Code sont comptabilisées.
</Note>

Le tableau de bord affiche ces métriques récapitulatives en haut :

* **PRs avec CC** : nombre total de demandes de fusion qui contiennent au moins une ligne de code écrite avec Claude Code
* **Lignes de code avec CC** : nombre total de lignes de code dans toutes les PRs fusionnées qui ont été écrites avec l'assistance de Claude Code. Seules les « lignes effectives » sont comptabilisées : lignes avec plus de 3 caractères après normalisation, excluant les lignes vides et les lignes contenant uniquement des crochets ou une ponctuation triviale.
* **PRs avec Claude Code (%)** : pourcentage de toutes les PRs fusionnées qui contiennent du code assisté par Claude Code
* **Taux d'acceptation des suggestions** : pourcentage de fois où les utilisateurs acceptent les suggestions d'édition de code de Claude Code, y compris l'utilisation des outils Edit, Write et NotebookEdit
* **Lignes de code acceptées** : nombre total de lignes de code écrites par Claude Code que les utilisateurs ont acceptées dans leurs sessions. Cela exclut les suggestions rejetées et ne suit pas les suppressions ultérieures.

<h3 id="explore-the-charts">
  Explorer les graphiques
</h3>

Le tableau de bord inclut plusieurs graphiques pour visualiser les tendances au fil du temps.

<h4 id="track-adoption">
  Suivre l'adoption
</h4>

Le graphique Adoption affiche les tendances d'utilisation quotidiennes :

* **utilisateurs** : utilisateurs actifs quotidiens
* **sessions** : nombre de sessions Claude Code actives par jour

<h4 id="measure-prs-per-user">
  Mesurer les PRs par utilisateur
</h4>

Ce graphique affiche l'activité des développeurs individuels au fil du temps :

* **PRs par utilisateur** : nombre total de PRs fusionnées par jour divisé par les utilisateurs actifs quotidiens
* **utilisateurs** : utilisateurs actifs quotidiens

Utilisez ceci pour comprendre comment la productivité individuelle change à mesure que l'adoption de Claude Code augmente.

<h4 id="view-pull-requests-breakdown">
  Afficher la répartition des demandes de fusion
</h4>

Le graphique Demandes de fusion affiche une répartition quotidienne des PRs fusionnées :

* **PRs avec CC** : demandes de fusion contenant du code assisté par Claude Code
* **PRs sans CC** : demandes de fusion sans code assisté par Claude Code

Basculez vers la vue **Lignes de code** pour voir la même répartition par lignes de code plutôt que par nombre de PR.

<h4 id="find-top-contributors">
  Trouver les principaux contributeurs
</h4>

Le classement affiche les 10 meilleurs utilisateurs classés par volume de contribution. Basculez entre :

* **Demandes de fusion** : affiche les PRs avec Claude Code par rapport à toutes les PRs pour chaque utilisateur
* **Lignes de code** : affiche les lignes avec Claude Code par rapport à toutes les lignes pour chaque utilisateur

Cliquez sur **Exporter tous les utilisateurs** pour télécharger les données de contribution complètes pour tous les utilisateurs au format CSV. L'export inclut tous les utilisateurs, pas seulement les 10 premiers affichés.

<h3 id="pr-attribution">
  Attribution des PR
</h3>

Lorsque les métriques de contribution sont activées, Claude Code analyse les demandes de fusion fusionnées pour déterminer quel code a été écrit avec l'assistance de Claude Code. Ceci est fait en mettant en correspondance l'activité de session Claude Code par rapport au code dans chaque PR.

<h4 id="tagging-criteria">
  Critères de balisage
</h4>

Les PRs sont balisées comme « avec Claude Code » si elles contiennent au moins une ligne de code écrite lors d'une session Claude Code. Le système utilise une correspondance conservatrice : seul le code où il y a une grande confiance dans l'implication de Claude Code est compté comme assisté.

<h4 id="attribution-process">
  Processus d'attribution
</h4>

Lorsqu'une demande de fusion est fusionnée :

1. Les lignes ajoutées sont extraites du diff de la PR
2. Les sessions Claude Code qui ont modifié les fichiers correspondants dans une fenêtre de temps sont identifiées
3. Les lignes de PR sont mises en correspondance avec la sortie de Claude Code en utilisant plusieurs stratégies
4. Les métriques sont calculées pour les lignes assistées par l'IA et le nombre total de lignes

Avant la comparaison, les lignes sont normalisées : les espaces blancs sont supprimés, les espaces multiples sont réduits, les guillemets sont standardisés et le texte est converti en minuscules.

Les demandes de fusion fusionnées contenant des lignes assistées par Claude Code sont étiquetées comme `claude-code-assisted` dans GitHub.

<h4 id="time-window">
  Fenêtre de temps
</h4>

Les sessions de 21 jours avant à 2 jours après la date de fusion de la PR sont considérées pour la correspondance d'attribution.

<h4 id="excluded-files">
  Fichiers exclus
</h4>

Certains fichiers sont automatiquement exclus de l'analyse car ils sont générés automatiquement :

* Fichiers de verrouillage : package-lock.json, yarn.lock, Cargo.lock et similaires
* Code généré : sorties Protobuf, artefacts de construction, fichiers minifiés
* Répertoires de construction : dist/, build/, node\_modules/, target/
* Fixtures de test : snapshots, cassettes, données fictives
* Lignes de plus de 1 000 caractères, qui sont probablement minifiées ou générées

<h4 id="attribution-notes">
  Notes d'attribution
</h4>

Gardez ces détails supplémentaires à l'esprit lors de l'interprétation des données d'attribution :

* Le code considérablement réécrit par les développeurs, avec une différence de plus de 20 %, n'est pas attribué à Claude Code
* Les sessions en dehors de la fenêtre de 21 jours ne sont pas considérées
* L'algorithme ne considère pas la branche source ou destination de la PR lors de l'attribution

<h3 id="get-the-most-from-analytics">
  Tirer le meilleur parti de l'analytique
</h3>

Utilisez les métriques de contribution pour démontrer le ROI, identifier les modèles d'adoption et trouver les membres de l'équipe qui peuvent aider les autres à démarrer.

<h4 id="monitor-adoption">
  Surveiller l'adoption
</h4>

Suivez le graphique Adoption et les nombres d'utilisateurs pour identifier :

* Les utilisateurs actifs qui peuvent partager les meilleures pratiques
* Les tendances globales d'adoption dans votre organisation
* Les baisses d'utilisation qui peuvent indiquer des frictions ou des problèmes

<h4 id="measure-roi">
  Mesurer le ROI
</h4>

Les métriques de contribution aident à répondre à « Cet outil vaut-il l'investissement ? » avec des données de votre propre base de code :

* Suivez les changements dans les PRs par utilisateur au fil du temps à mesure que l'adoption augmente
* Comparez les PRs et les lignes de code livrées avec et sans Claude Code
* Utilisez aux côtés des [métriques DORA](https://dora.dev/), de la vélocité des sprints ou d'autres KPIs d'ingénierie pour comprendre les changements résultant de l'adoption de Claude Code

<h4 id="identify-power-users">
  Identifier les utilisateurs avancés
</h4>

Le classement vous aide à trouver les membres de l'équipe avec une adoption élevée de Claude Code qui peuvent :

* Partager les techniques de prompting et les flux de travail avec l'équipe
* Fournir des commentaires sur ce qui fonctionne bien
* Aider à intégrer les nouveaux utilisateurs

<h4 id="access-data-programmatically">
  Accéder aux données par programmation
</h4>

Sur le plan Enterprise, l'[API Claude Enterprise Analytics](https://platform.claude.com/docs/en/api/admin/analytics) retourne les rapports d'engagement, d'utilisation et de coûts par utilisateur pour votre organisation sur toutes les surfaces Claude, y compris Claude Code. Un propriétaire principal crée une clé avec la portée `read:analytics` à [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). L'API n'est pas disponible sur le plan Teams.

Pour interroger les données de contribution via GitHub à la place, recherchez les PRs étiquetées avec `claude-code-assisted`.

<h2 id="access-analytics-for-api-customers">
  Accéder à l'analytique pour les clients API
</h2>

Les clients API utilisant Claude Console peuvent accéder à l'analytique à [platform.claude.com/claude-code](https://platform.claude.com/claude-code). Vous devez avoir la permission UsageView pour accéder au tableau de bord, qui est accordée aux rôles Developer, Billing, Admin, Owner et Primary Owner. Pour extraire les mêmes métriques quotidiennes par utilisateur par programmation, utilisez l'[API Claude Code Analytics](https://platform.claude.com/docs/fr/build-with-claude/claude-code-analytics-api) avec une clé API Admin.

<Note>
  Les métriques de contribution avec intégration GitHub ne sont actuellement pas disponibles pour les clients API. Le tableau de bord Console affiche uniquement les métriques d'utilisation et de dépenses.
</Note>

Le tableau de bord Console affiche :

* **Lignes de code acceptées** : nombre total de lignes de code écrites par Claude Code que les utilisateurs ont acceptées dans leurs sessions. Cela exclut les suggestions rejetées et ne suit pas les suppressions ultérieures.
* **Taux d'acceptation des suggestions** : pourcentage de fois où les utilisateurs acceptent l'utilisation de l'outil d'édition de code, y compris les outils Edit, Write et NotebookEdit.
* **Activité** : utilisateurs actifs quotidiens et sessions affichés sur un graphique.
* **Dépenses** : coûts quotidiens de l'API en dollars aux côtés du nombre d'utilisateurs.

<h3 id="view-team-insights">
  Afficher les insights d'équipe
</h3>

Le tableau des insights d'équipe affiche les métriques par utilisateur :

* **Membres** : tous les utilisateurs qui se sont authentifiés à Claude Code. Les utilisateurs de clé API s'affichent par identifiant de clé, les utilisateurs OAuth s'affichent par adresse e-mail.
* **Dépenses ce mois** : coûts totaux de l'API par utilisateur pour le mois en cours.
* **Lignes ce mois** : total par utilisateur des lignes de code acceptées pour le mois en cours.

<Note>
  Les chiffres de dépenses dans le tableau de bord Console sont des estimations à des fins analytiques. Pour les coûts réels, consultez votre page de facturation.
</Note>

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Surveillance avec OpenTelemetry](/docs/fr/monitoring-usage) : exportez les métriques et événements en temps réel vers votre pile d'observabilité
* [Gérer les coûts efficacement](/docs/fr/costs) : définissez les limites de dépenses et optimisez l'utilisation des tokens
* [Permissions](/docs/fr/permissions) : configurez les rôles et les permissions
