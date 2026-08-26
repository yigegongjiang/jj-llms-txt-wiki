> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Démarrer avec Claude Code sur le web

> Exécutez Claude Code dans le cloud depuis votre navigateur ou téléphone. Connectez un référentiel GitHub, soumettez une tâche et examinez la PR sans configuration locale.

<Note>
  Claude Code sur le web est en aperçu de recherche pour les utilisateurs Pro, Max et Team, ainsi que pour les utilisateurs Enterprise disposant de sièges premium ou de sièges Chat + Claude Code.
</Note>

Claude Code sur le web s'exécute sur l'infrastructure cloud gérée par Anthropic au lieu de votre machine. Soumettez des tâches depuis [claude.ai/code](https://claude.ai/code) dans votre navigateur ou l'application mobile Claude.

Vous aurez besoin d'un référentiel GitHub pour [démarrer](#connect-github-and-create-an-environment). Claude le clone dans une machine virtuelle isolée, effectue des modifications et pousse une branche pour que vous la révisiez. Les sessions persistent sur les appareils, donc une tâche que vous commencez sur votre ordinateur portable est prête à être examinée depuis votre téléphone plus tard.

Claude Code sur le web fonctionne bien pour :

* **Tâches parallèles** : exécutez plusieurs tâches indépendantes à la fois, chacune dans sa propre session et branche, sans gérer plusieurs worktrees
* **Référentiels que vous n'avez pas localement** : Claude clone le référentiel à nouveau à chaque session, vous n'avez donc pas besoin de l'avoir extrait
* **Tâches qui ne nécessitent pas de direction fréquente** : soumettez une tâche bien définie, faites autre chose et examinez le résultat quand Claude a terminé
* **Questions de code et exploration** : comprenez une base de code ou tracez comment une fonctionnalité est implémentée sans extraction locale

Pour les travaux qui nécessitent votre configuration locale, vos outils ou votre environnement, l'exécution de Claude Code localement ou l'utilisation de [Remote Control](/docs/fr/remote-control) est plus appropriée.

<h2 id="how-sessions-run">
  Comment les sessions s'exécutent
</h2>

Quand vous soumettez une tâche :

1. **Clone et préparation** : votre référentiel est cloné sur une VM gérée par Anthropic, et votre [script de configuration](/docs/fr/claude-code-on-the-web#setup-scripts) s'exécute s'il est configuré.
2. **Configurer le réseau** : l'accès à Internet est défini en fonction du [niveau d'accès](/docs/fr/claude-code-on-the-web#access-levels) de votre environnement.
3. **Travail** : Claude analyse le code, effectue des modifications, exécute des tests et vérifie son travail. Vous pouvez regarder et diriger tout au long du processus, ou vous éloigner et revenir quand c'est fait.
4. **Pousser la branche** : quand Claude atteint un point d'arrêt, il pousse sa branche vers GitHub. Vous examinez le diff, laissez des commentaires en ligne, créez une PR ou envoyez un autre message pour continuer.

La session ne se ferme pas quand la branche est poussée. La création de PR et les modifications supplémentaires se font toutes dans la même conversation.

<h2 id="compare-ways-to-run-claude-code">
  Comparer les façons d'exécuter Claude Code
</h2>

Claude Code se comporte de la même manière partout. Ce qui change, c'est où le code s'exécute et si votre configuration locale est disponible. L'application Desktop offre à la fois des sessions locales et cloud, donc ses réponses ci-dessous dépendent de celle que vous choisissez :

|                                                     | Sur le web                                                                                                                | Remote Control                                             | Terminal CLI            | Application Desktop                |
| :-------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------- | :---------------------- | :--------------------------------- |
| **Le code s'exécute sur**                           | VM cloud Anthropic                                                                                                        | Votre machine                                              | Votre machine           | Votre machine ou VM cloud          |
| **Vous discutez depuis**                            | claude.ai ou application mobile                                                                                           | claude.ai ou application mobile                            | Votre terminal          | L'interface utilisateur Desktop    |
| **Utilise votre configuration locale**              | Non, référentiel uniquement                                                                                               | Oui                                                        | Oui                     | Oui pour local, non pour cloud     |
| **Nécessite GitHub**                                | Oui, ou [regroupez un référentiel local](/docs/fr/claude-code-on-the-web#send-local-repositories-without-github) via `--cloud` | Non                                                        | Non                     | Uniquement pour les sessions cloud |
| **Continue de s'exécuter si vous vous déconnectez** | Oui                                                                                                                       | Tant que le terminal reste ouvert                          | Non                     | Dépend du type de session          |
| **[Modes de permission](/docs/fr/permission-modes)**     | Accepter les modifications, Plan, Auto                                                                                    | Demander, Accepter automatiquement les modifications, Plan | Tous les modes          | Dépend du type de session          |
| **Accès réseau**                                    | Configurable par environnement                                                                                            | Réseau de votre machine                                    | Réseau de votre machine | Dépend du type de session          |

Consultez la [documentation du démarrage rapide du terminal](/docs/fr/quickstart), [Application Desktop](/docs/fr/desktop) ou [Remote Control](/docs/fr/remote-control) pour les configurer.

<h2 id="connect-github-and-create-an-environment">
  Connecter GitHub et créer un environnement
</h2>

La configuration est un processus unique. Si vous utilisez déjà la CLI GitHub, vous pouvez [le faire depuis votre terminal](#connect-from-your-terminal) au lieu du navigateur.

<Steps>
  <Step title="Visitez claude.ai/code">
    Allez à [claude.ai/code](https://claude.ai/code) et connectez-vous avec votre compte Anthropic.
  </Step>

  <Step title="Installez l'application Claude GitHub">
    Après vous être connecté, claude.ai/code vous invite à connecter GitHub. Suivez l'invite pour installer l'application Claude GitHub et lui accorder l'accès à vos référentiels. Les sessions cloud fonctionnent avec les référentiels GitHub existants, donc pour démarrer un nouveau projet, [créez d'abord un référentiel vide sur GitHub](https://github.com/new).
  </Step>

  <Step title="Créez votre environnement">
    Après avoir connecté GitHub, vous serez invité à créer un environnement cloud. L'environnement contrôle l'accès réseau que Claude a pendant les sessions et ce qui s'exécute quand une nouvelle session est créée. Consultez [Outils installés](/docs/fr/claude-code-on-the-web#installed-tools) pour voir ce qui est disponible sans aucune configuration.

    Le formulaire a ces champs :

    * **Nom** : une étiquette d'affichage. Utile quand vous avez plusieurs environnements pour différents projets ou niveaux d'accès.
    * **Accès réseau** : contrôle ce que la session peut atteindre sur Internet. Par défaut, `Trusted`, permet les connexions aux [registres de paquets courants](/docs/fr/claude-code-on-the-web#default-allowed-domains) comme npm, PyPI et RubyGems tout en bloquant l'accès général à Internet.
    * **Variables d'environnement** : variables optionnelles disponibles dans chaque session, au format `.env`. N'enveloppez pas les valeurs entre guillemets, car les guillemets sont stockés comme faisant partie de la valeur. Celles-ci sont visibles par quiconque peut modifier cet environnement.
    * **Script de configuration** : un script Bash optionnel qui s'exécute avant le lancement de Claude Code. Utilisez-le pour installer les outils système que la VM cloud n'inclut pas, comme `apt install -y gh`. Le résultat est [mis en cache](/docs/fr/claude-code-on-the-web#environment-caching), donc le script ne se réexécute pas à chaque session. Consultez [Scripts de configuration](/docs/fr/claude-code-on-the-web#setup-scripts) pour des exemples et des conseils de débogage.

    Pour un premier projet, laissez les valeurs par défaut et cliquez sur **Créer un environnement**. Vous pouvez [le modifier plus tard ou créer des environnements supplémentaires](/docs/fr/claude-code-on-the-web#configure-your-environment) pour différents projets.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Connecter depuis votre terminal
</h3>

Si vous utilisez déjà la CLI GitHub (`gh`), vous pouvez configurer Claude Code sur le web sans ouvrir un navigateur. Cela nécessite la [CLI Claude Code](/docs/fr/quickstart). `/web-setup` lit votre jeton `gh` local, le lie à votre compte Claude et crée un environnement cloud par défaut si vous n'en avez pas.

<Note>
  Les organisations avec [Zero Data Retention](/docs/fr/zero-data-retention) activé ne peuvent pas utiliser `/web-setup` ou d'autres fonctionnalités de session cloud. Si la CLI GitHub n'est pas installée ou authentifiée, `/web-setup` ouvre le flux d'intégration du navigateur à la place.
</Note>

<Steps>
  <Step title="Authentifiez-vous avec la CLI GitHub">
    Dans votre shell, authentifiez la CLI GitHub si vous ne l'avez pas déjà fait :

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Connectez-vous à Claude">
    Dans la CLI Claude Code, exécutez `/login` pour vous connecter avec votre compte claude.ai. Ignorez cette étape si vous êtes déjà connecté.
  </Step>

  <Step title="Exécutez /web-setup">
    Dans la CLI Claude Code, exécutez :

    ```text theme={null}
    /web-setup
    ```

    Cela synchronise votre jeton `gh` avec votre compte Claude. Si vous n'avez pas encore d'environnement cloud, `/web-setup` en crée un avec accès réseau Trusted et aucun script de configuration. Vous pouvez [modifier l'environnement ou ajouter des variables](/docs/fr/claude-code-on-the-web#configure-your-environment) après. Une fois que `/web-setup` est terminé, vous pouvez démarrer des sessions cloud depuis votre terminal avec [`--cloud`](/docs/fr/claude-code-on-the-web#from-terminal-to-web) ou configurer des tâches récurrentes avec [`/schedule`](/docs/fr/routines).
  </Step>
</Steps>

<h2 id="start-a-task">
  Démarrer une tâche
</h2>

Avec GitHub connecté et un environnement créé, vous êtes prêt à soumettre des tâches.

<Steps>
  <Step title="Sélectionnez un référentiel et une branche">
    Depuis [claude.ai/code](https://claude.ai/code) ou l'onglet Code dans l'application mobile Claude, cliquez sur le sélecteur de référentiel sous la zone de saisie et choisissez un référentiel dans lequel Claude doit travailler. Chaque référentiel affiche un sélecteur de branche. Changez-le pour démarrer Claude à partir d'une branche de fonctionnalité au lieu de la branche par défaut. Vous pouvez ajouter plusieurs référentiels pour travailler sur plusieurs dans une seule session.
  </Step>

  <Step title="Choisissez un mode de permission">
    Le menu déroulant du mode à côté de l'entrée par défaut est **Accepter automatiquement les modifications**, où Claude effectue des modifications et pousse une branche sans s'arrêter pour approbation. Basculez vers **Plan Mode** si vous voulez que Claude propose une approche et attende votre feu vert avant de modifier les fichiers. Les sessions cloud n'offrent pas les permissions Ask ou Bypass permissions. Consultez la [liste complète des modes de permission](/docs/fr/permission-modes#available-modes) pour savoir ce que chacun permet.
  </Step>

  <Step title="Décrivez la tâche et soumettez">
    Tapez une description de ce que vous voulez et appuyez sur Entrée. Soyez spécifique :

    * Nommez le fichier ou la fonction : « Ajouter un README avec les instructions de configuration » ou « Corriger le test d'authentification défaillant dans `tests/test_auth.py` » est mieux que « corriger les tests »
    * Collez la sortie d'erreur si vous l'avez
    * Décrivez le comportement attendu, pas seulement le symptôme

    Claude clone les référentiels, exécute votre script de configuration s'il est configuré et commence à travailler. Chaque tâche obtient sa propre session et sa propre branche, vous n'avez donc pas besoin d'attendre qu'une se termine avant de commencer une autre.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  Pré-remplir les sessions
</h2>

Vous pouvez pré-remplir l'invite, les référentiels et l'environnement pour une nouvelle session en ajoutant des paramètres de requête à l'URL [claude.ai/code](https://claude.ai/code). Utilisez ceci pour créer des intégrations telles qu'un bouton dans votre suivi de problèmes qui ouvre Claude Code avec la description du problème comme invite.

| Paramètre      | Description                                                                                                                                                                                                     |
| :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`       | Texte d'invite à pré-remplir dans la zone de saisie. L'alias `q` est également accepté.                                                                                                                         |
| `prompt_url`   | URL pour récupérer le texte d'invite, pour les invites trop longues pour être intégrées dans une chaîne de requête. L'URL doit autoriser les demandes cross-origin. Ignoré quand `prompt` est également défini. |
| `repositories` | Liste séparée par des virgules de slugs `owner/repo` à présélectionner. L'alias `repo` est également accepté.                                                                                                   |
| `environment`  | Nom ou ID de l'[environnement](#connect-github-and-create-an-environment) à présélectionner.                                                                                                                    |

Encodez en URL chaque valeur. L'exemple ci-dessous ouvre le formulaire avec une invite et un référentiel déjà sélectionnés :

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  Examiner et itérer
</h2>

Quand Claude a terminé, examinez les modifications, laissez des commentaires sur des lignes spécifiques et continuez jusqu'à ce que le diff soit correct.

<Steps>
  <Step title="Ouvrez la vue diff">
    Un indicateur diff affiche les lignes ajoutées et supprimées dans la session, par exemple `+42 -18`. Sélectionnez-le pour ouvrir la vue diff, avec une liste de fichiers à gauche et les modifications à droite.
  </Step>

  <Step title="Laissez des commentaires en ligne">
    Sélectionnez n'importe quelle ligne dans le diff, tapez vos commentaires et appuyez sur Entrée. Les commentaires s'accumulent jusqu'à ce que vous envoyiez votre prochain message, puis ils sont regroupés avec celui-ci. Claude voit ' à `src/auth.ts:47`, ne capturez pas l'erreur ici ' aux côtés de votre instruction principale, vous n'avez donc pas à décrire où se trouve le problème.
  </Step>

  <Step title="Créez une demande de tirage">
    Quand le diff est correct, sélectionnez **Créer une PR** en haut de la vue diff. Vous pouvez l'ouvrir comme une PR complète, un brouillon, ou accéder à la page de composition de GitHub avec un titre et une description générés.
  </Step>

  <Step title="Continuez à itérer après la PR">
    La session reste active après la création de la PR. Collez la sortie d'échec CI ou les commentaires des examinateurs dans le chat et demandez à Claude de les traiter. Pour que Claude surveille la PR automatiquement, consultez [Correction automatique des demandes de tirage](/docs/fr/claude-code-on-the-web#auto-fix-pull-requests).
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  Dépanner la configuration
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  Aucun référentiel n'apparaît après la connexion à GitHub
</h3>

Une session cloud peut utiliser n'importe quel référentiel que le compte GitHub connecté peut voir, indépendamment des référentiels sur lesquels l'application Claude GitHub est installée. Si un référentiel est manquant, vérifiez que le compte GitHub connecté y a accès sur GitHub. Si vous voulez également [Auto-fix](/docs/fr/claude-code-on-the-web#auto-fix-pull-requests) pour un référentiel, installez l'application dessus : sur github.com, ouvrez **Paramètres → Applications → Claude → Configurer** et vérifiez que le référentiel est listé sous **Accès aux référentiels**. Les référentiels privés ont besoin de la même autorisation que les référentiels publics.

<h3 id="the-page-only-shows-a-github-login-button">
  La page affiche uniquement un bouton de connexion GitHub
</h3>

Les sessions cloud nécessitent un compte GitHub connecté. Connectez-vous via le flux du navigateur ci-dessus, ou exécutez `/web-setup` depuis votre terminal si vous utilisez la CLI GitHub. Si vous préférez ne pas connecter GitHub du tout, consultez [Remote Control](/docs/fr/remote-control) pour exécuter Claude Code sur votre propre machine et le surveiller depuis le web.

<h3 id="not-available-for-the-selected-organization">
  « Non disponible pour l'organisation sélectionnée »
</h3>

Les organisations Enterprise peuvent avoir besoin qu'un propriétaire active Claude Code sur le web. Contactez votre équipe de compte Anthropic.

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` affiche ' Aucune commande ne correspond ' ou ' Commande inconnue '
</h3>

`/web-setup` s'exécute à l'intérieur de la CLI Claude Code, pas votre shell. Lancez `claude` d'abord, puis tapez `/web-setup` à l'invite.

Si vous l'avez tapé à l'intérieur de Claude Code et le menu de commandes affiche `Aucune commande ne correspond à "/web-setup"`, ou que le soumettre retourne `Commande inconnue : /web-setup`, la commande est masquée parce qu'une exigence n'est pas satisfaite. La cause est généralement que vous êtes authentifié avec une clé API ou un fournisseur tiers au lieu d'un abonnement claude.ai. Exécutez `/login` pour vous connecter avec votre compte claude.ai.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  « Impossible de créer un environnement cloud » ou « Aucun environnement cloud disponible » lors de l'utilisation de `--cloud` ou ultraplan
</h3>

Les fonctionnalités de session à distance créent automatiquement un environnement cloud par défaut si vous n'en avez pas. Si vous voyez « Impossible de créer un environnement cloud », la création automatique a échoué. Si vous voyez « Aucun environnement cloud disponible », votre CLI est antérieur à la création automatique. Dans les deux cas, exécutez `/web-setup` dans la CLI Claude Code pour en créer un manuellement, ou visitez [claude.ai/code](https://claude.ai/code) et suivez l'étape **Créez votre environnement** ci-dessus.

<h3 id="setup-script-failed">
  Le script de configuration a échoué
</h3>

Le script de configuration s'est terminé avec un statut non-zéro, ce qui bloque le démarrage de la session. Les causes courantes :

* Une installation de paquet a échoué parce que le registre n'est pas dans votre [niveau d'accès réseau](/docs/fr/claude-code-on-the-web#access-levels). `Trusted` couvre la plupart des gestionnaires de paquets ; `None` les bloque tous.
* Le script fait référence à un fichier ou un chemin qui n'existe pas dans un clone frais.
* Une commande qui fonctionne localement a besoin d'une invocation différente sur Ubuntu.

Pour déboguer, ajoutez `set -x` en haut du script pour voir quelle commande a échoué. Pour les commandes non critiques, ajoutez `|| true` pour qu'elles ne bloquent pas le démarrage de la session.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Les nouvelles sessions se figent ou expirent pendant la configuration
</h3>

Si les nouvelles sessions se figent à l'étape du script de configuration ou échouent avec une erreur de conteneur générique avant la fin du script, le script dépasse probablement le budget de temps d'environ cinq minutes pour construire le [cache d'environnement](/docs/fr/claude-code-on-the-web#environment-caching). Les étapes lourdes telles que l'extraction d'images Docker volumineuses, la synchronisation d'arbres de dépendances complets ou le téléchargement de poids de modèles dépassent souvent la limite, surtout quand elles s'exécutent l'une après l'autre.

Pour corriger cela, réduisez le script pour qu'il se termine de manière fiable en moins de cinq minutes :

* Exécutez les installations indépendantes en parallèle avec `&` et un `wait` final au lieu de les exécuter en série.
* Déplacez les plus grands téléchargements hors du script de configuration et dans un [hook SessionStart](/docs/fr/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks) qui les lance en arrière-plan, pour que la session devienne utilisable pendant qu'ils se terminent.
* Supprimez les longs délais de nouvelle tentative du script de configuration, car une boucle de nouvelle tentative figée compte dans le budget.

<h3 id="session-keeps-running-after-closing-the-tab">
  La session continue de s'exécuter après la fermeture de l'onglet
</h3>

C'est intentionnel. Fermer l'onglet ou naviguer ailleurs n'arrête pas la session. Elle continue de s'exécuter en arrière-plan jusqu'à ce que Claude termine la tâche actuelle, puis elle reste inactive. Depuis la barre latérale, vous pouvez [archiver une session](/docs/fr/claude-code-on-the-web#archive-sessions) pour la masquer de votre liste, ou [la supprimer](/docs/fr/claude-code-on-the-web#delete-sessions) pour la supprimer définitivement.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous pouvez soumettre et examiner des tâches, ces pages couvrent ce qui vient ensuite : démarrer des sessions cloud depuis votre terminal, planifier des travaux récurrents et donner à Claude des instructions permanentes.

* [Utiliser Claude Code sur le web](/docs/fr/claude-code-on-the-web) : la référence complète, y compris la téléportation de sessions vers votre terminal, les scripts de configuration, les variables d'environnement et la configuration réseau
* [Routines](/docs/fr/routines) : automatisez le travail selon un calendrier, via un appel API ou en réponse aux événements GitHub
* [CLAUDE.md](/docs/fr/memory) : donnez à Claude des instructions et un contexte persistants qui se chargent au début de chaque session
* Installez l'application mobile Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) pour surveiller les sessions depuis votre téléphone. Depuis la CLI Claude Code, `/mobile` affiche un code QR.
