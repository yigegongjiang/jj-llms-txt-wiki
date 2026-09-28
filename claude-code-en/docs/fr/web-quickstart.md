> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Démarrer avec Claude Code dans le cloud

> Exécutez Claude Code dans le cloud depuis votre navigateur ou téléphone. Connectez un référentiel GitHub, soumettez une tâche et examinez la PR sans configuration locale.

<Note>
  Les sessions cloud sont disponibles sur les plans Pro, Max et Team, ainsi que pour les utilisateurs Enterprise disposant de sièges premium ou de sièges Chat + Claude Code.
</Note>

Une session cloud exécute Claude Code sur l'infrastructure cloud au lieu de votre machine, gérée par Anthropic par défaut. Ce guide de démarrage rapide en lance une depuis [claude.ai/code](https://claude.ai/code) dans votre navigateur. Vous pouvez également en lancer une depuis l'application mobile Claude, l'application Desktop, ou votre terminal avec `claude --cloud`.

Vous aurez besoin d'un référentiel GitHub pour [démarrer](#connect-github). Claude le clone dans une machine virtuelle isolée, effectue des modifications et pousse une branche pour que vous la révisiez. Les sessions persistent sur les appareils, donc une tâche que vous commencez sur votre ordinateur portable est prête à être examinée depuis votre téléphone plus tard.

Les sessions cloud fonctionnent bien pour :

* **Tâches parallèles** : exécutez plusieurs tâches indépendantes à la fois, chacune dans sa propre session et branche, sans gérer plusieurs worktrees
* **Référentiels que vous n'avez pas localement** : Claude clone le référentiel à nouveau à chaque session, vous n'avez donc pas besoin de l'avoir extrait
* **Tâches qui ne nécessitent pas de direction fréquente** : soumettez une tâche bien définie, faites autre chose et examinez le résultat quand Claude a terminé
* **Questions de code et exploration** : comprenez une base de code ou tracez comment une fonctionnalité est implémentée sans extraction locale

Pour les travaux qui nécessitent votre configuration locale, vos outils ou votre environnement, l'exécution de Claude Code localement ou l'utilisation de [Remote Control](/docs/fr/remote-control) est plus appropriée.

<h2 id="how-sessions-run">
  Comment les sessions s'exécutent
</h2>

Les étapes ci-dessous décrivent les sessions hébergées par Anthropic. Dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments), le clone et tout ce qui suit s'exécutent sur les runners de votre organisation, où les limites réseau, la configuration et le comportement de push sont configurés par l'opérateur. Quand vous soumettez une tâche :

1. **Clone et préparation** : votre référentiel est cloné sur une VM gérée par Anthropic, et votre [script de configuration](/docs/fr/cloud-environments#setup-scripts) s'exécute s'il est configuré.
2. **Configurer le réseau** : l'accès à Internet est défini en fonction du [niveau d'accès](/docs/fr/cloud-environments#access-levels) de votre environnement.
3. **Travail** : Claude analyse le code, effectue des modifications, exécute des tests et vérifie son travail. Vous pouvez regarder et diriger tout au long du processus, ou vous éloigner et revenir quand c'est fait.
4. **Pousser la branche** : quand Claude atteint un point d'arrêt, il pousse sa branche vers GitHub. Vous examinez le diff, laissez des commentaires en ligne, créez une PR ou envoyez un autre message pour continuer.

La session ne se ferme pas quand la branche est poussée. La création de PR et les modifications supplémentaires se font toutes dans la même conversation.

<h2 id="compare-ways-to-run-claude-code">
  Comparer les façons d'exécuter Claude Code
</h2>

Claude Code se comporte de la même manière partout. Ce qui change, c'est où la session s'exécute et si votre configuration locale est disponible :

|                                                     | Session cloud                                                                                                             | Session locale                                                                                                                                                  | Session locale avec [Remote Control](/docs/fr/remote-control)                             |
| :-------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------- |
| **Le code s'exécute sur**                           | VM cloud, gérée par Anthropic par défaut                                                                                  | Votre machine                                                                                                                                                   | Votre machine                                                                        |
| **Vous la démarrez depuis**                         | claude.ai/code, l'application mobile Claude, l'application Desktop avec **Cloud** sélectionné, ou `claude --cloud`        | Votre terminal, votre IDE, ou l'application Desktop avec **Local** sélectionné                                                                                  | Votre terminal, l'extension VS Code, ou l'application Desktop                        |
| **Vous discutez depuis**                            | claude.ai, l'application mobile, ou l'application Desktop                                                                 | Où vous l'avez démarrée                                                                                                                                         | claude.ai ou l'application mobile, ainsi que d'où vous l'avez démarrée               |
| **Utilise votre configuration locale**              | Non, référentiel uniquement                                                                                               | Oui                                                                                                                                                             | Oui                                                                                  |
| **Nécessite GitHub**                                | Oui, ou [regroupez un référentiel local](/docs/fr/claude-code-on-the-web#send-local-repositories-without-github) via `--cloud` | Non                                                                                                                                                             | Non                                                                                  |
| **Continue de s'exécuter si vous vous déconnectez** | Oui                                                                                                                       | Non                                                                                                                                                             | Tant que la session reste ouverte sur votre machine                                  |
| **[Modes de permission](/docs/fr/permission-modes)**     | Accepter les modifications, Plan, Auto                                                                                    | Tous les modes dans le terminal ; consultez [Changer les modes de permission](/docs/fr/permission-modes#switch-permission-modes) pour l'IDE et l'application Desktop | Manuel, Accepter les modifications, ou Plan depuis claude.ai et l'application mobile |
| **Accès réseau**                                    | Configurable par environnement                                                                                            | Réseau de votre machine                                                                                                                                         | Réseau de votre machine                                                              |

Consultez la [documentation du démarrage rapide du terminal](/docs/fr/quickstart), [Application Desktop](/docs/fr/desktop), ou [Remote Control](/docs/fr/remote-control) pour configurer les sessions locales.

<h2 id="connect-github">
  Connecter GitHub
</h2>

La connexion à GitHub est une étape unique. Si vous utilisez déjà la CLI GitHub, vous pouvez [le faire depuis votre terminal](#connect-from-your-terminal) au lieu du navigateur.

<Note>
  Sur les plans Team et Enterprise, l'étape **Se connecter avec GitHub** ne fonctionne qu'après qu'un [Propriétaire](/docs/fr/server-managed-settings#access-control) de votre organisation Claude active le connecteur GitHub dans [**Paramètres d'administration > Connecteurs**](https://claude.ai/admin-settings/connectors). Jusqu'à ce moment, cette étape affiche « L'accès GitHub est requis pour Claude Code sur le web » au lieu d'un bouton de connexion. Une fois le connecteur activé, rechargez [claude.ai/code](https://claude.ai/code) et recommencez à partir de la première étape. Un deuxième bouton bascule, [Configuration web rapide](/docs/fr/claude-code-on-the-web#github-authentication-options) dans [**Paramètres d'administration > Claude Code**](https://claude.ai/admin-settings/claude-code), est optionnel : lorsqu'il est activé, `/web-setup` fonctionne et l'intégration crée l'environnement pour les membres.
</Note>

<Steps>
  <Step title="Visitez claude.ai/code">
    Allez à [claude.ai/code](https://claude.ai/code) et connectez-vous avec votre compte claude.ai.
  </Step>

  <Step title="Se connecter avec GitHub">
    Après vous être connecté, claude.ai/code vous invite à connecter GitHub. Suivez l'invite, et claude.ai/code vous envoie à la page d'autorisation de GitHub. Approuvez la demande d'autorisation, et GitHub vous renvoie à claude.ai/code. Les sessions cloud fonctionnent avec les référentiels GitHub existants. Pour démarrer un nouveau projet, [créez d'abord un référentiel vide sur GitHub](https://github.com/new).

    Avec cette connexion, une session peut cloner n'importe quel référentiel public, mais ne peut travailler dans un référentiel privé que lorsque l'application Claude GitHub est installée dessus. [Installez l'application Claude GitHub](https://github.com/apps/claude/installations/new) sur chaque compte GitHub ou organisation dont vous souhaitez utiliser les référentiels privés. Sur une organisation GitHub, un propriétaire d'organisation peut avoir besoin d'approuver l'installation. L'installation de l'application active également [Correction automatique](/docs/fr/claude-code-on-the-web#auto-fix-pull-requests), qui permet à Claude de répondre aux défaillances CI et aux commentaires d'examen sur les demandes de tirage dans ces référentiels.

    Si l'intégration vous invite à installer l'application Claude GitHub à ce stade et que vous préférez le faire plus tard, cliquez sur **Ignorer**.
  </Step>

  <Step title="Configurez votre environnement par défaut">
    Un [environnement cloud](/docs/fr/cloud-environments) est la configuration enregistrée qui contrôle l'accès réseau que Claude a pendant les sessions et ce qui s'exécute au démarrage d'une session. Ce qui se passe après que vous connectiez GitHub dépend de votre plan :

    * **Pro et Max** : l'intégration crée un environnement nommé **Par défaut** pour vous.
    * **Team et Enterprise** : l'intégration affiche un formulaire **Créez votre premier environnement cloud**. Laissez le nom prérempli et l'accès réseau inchangés et cliquez sur **Créer et terminer** pour créer l'environnement **Par défaut**. Si un Propriétaire a activé [Configuration web rapide](/docs/fr/claude-code-on-the-web#github-authentication-options), l'intégration crée **Par défaut** pour vous à la place.

    **Par défaut** utilise l'accès réseau [`Trusted`](/docs/fr/cloud-environments#access-levels) : les sessions accèdent aux [registres de paquets courants](/docs/fr/cloud-environments#default-allowed-domains) et à d'autres domaines autorisés, et à rien d'autre via le réseau de la session. Consultez [Outils installés](/docs/fr/cloud-environments#installed-tools) pour voir ce qui est disponible sans aucune configuration.

    Pour un premier projet, l'environnement **Par défaut** fonctionne tel quel. Pour modifier son accès réseau, ajouter des variables d'environnement, ou exécuter un [script de configuration](/docs/fr/cloud-environments#setup-scripts) avant le démarrage des sessions, [modifiez-le ou créez des environnements supplémentaires](/docs/fr/cloud-environments#configure-your-environment).
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Connecter depuis votre terminal
</h3>

Si vous utilisez déjà la CLI GitHub (`gh`), vous pouvez connecter GitHub pour les sessions cloud depuis votre terminal. Cela nécessite la [CLI Claude Code](/docs/fr/quickstart). Sur les plans Team et Enterprise, `/web-setup` n'est disponible qu'après qu'un Propriétaire active [Configuration web rapide](/docs/fr/claude-code-on-the-web#github-authentication-options).

Lorsque vous exécutez `/web-setup`, Claude Code lit le jeton que `gh auth token` affiche, vous demande de confirmer, et envoie le jeton à Anthropic. Anthropic le stocke chiffré avec votre compte claude.ai, et vos sessions cloud l'utilisent pour l'accès GitHub jusqu'à ce que vous [le supprimiez](#remove-the-web-setup-token). Une session cloud que vous démarrez vous-même peut alors accéder à n'importe quel référentiel que ce jeton peut accéder, sans installation de l'application Claude GitHub. Les threads dans un [projet](/docs/fr/claude-projects#set-up-github-access) ont toujours besoin de l'application Claude GitHub.

Si vous avez déjà connecté GitHub dans le navigateur, `/web-setup` vous avertit que continuer remplace cette connexion pour vos sessions cloud.

<Note>
  Les organisations avec [Zéro conservation des données](/docs/fr/zero-data-retention) activé ne peuvent pas utiliser `/web-setup` ou d'autres fonctionnalités de session cloud. Si la CLI GitHub n'est pas installée ou n'est pas authentifiée, Claude Code ouvre le flux d'intégration du navigateur à la place.
</Note>

<Steps>
  <Step title="Authentifiez-vous avec la CLI GitHub">
    Dans votre shell, authentifiez la CLI GitHub si vous ne l'avez pas déjà fait :

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Connectez-vous à Claude">
    Dans la CLI Claude Code, exécutez `/login` pour vous connecter avec votre compte claude.ai. Ignorez cette étape si vous êtes déjà connecté avec un compte claude.ai. L'authentification avec une clé API ne compte pas. Pour vérifier, exécutez `/status` et confirmez que la ligne **Méthode de connexion** affiche un compte claude.ai.
  </Step>

  <Step title="Exécutez /web-setup">
    Dans la CLI Claude Code, exécutez :

    ```text theme={null}
    /web-setup
    ```

    Confirmez l'invite pour envoyer votre jeton `gh` à votre compte Claude. En cas de succès, Claude Code affiche `Connected as <your-github-username>` et ouvre [claude.ai/code](https://claude.ai/code) dans votre navigateur. Si vous n'avez pas encore d'environnement cloud, `/web-setup` en crée un avec accès réseau Trusted et aucun script de configuration. Vous pouvez [modifier l'environnement ou ajouter des variables](/docs/fr/cloud-environments#configure-your-environment) après. Une fois que `/web-setup` est terminé, vous pouvez démarrer des sessions cloud depuis votre terminal avec [`--cloud`](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud) ou configurer des tâches récurrentes avec [`/schedule`](/docs/fr/routines).
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  Supprimer le jeton `/web-setup`
</h4>

Pour supprimer le jeton de votre compte Claude, déconnectez GitHub dans [claude.ai/customize/connectors](https://claude.ai/customize/connectors). La déconnexion supprime les identifiants GitHub que vos sessions cloud utilisent, qu'ils proviennent du navigateur ou de `/web-setup`, donc les sessions cloud perdent l'accès GitHub jusqu'à ce que vous vous reconnectiez. Votre `gh` local reste connecté, et le jeton reste valide sur GitHub.

Pour invalider le jeton lui-même, révoquez-le sur GitHub. Si vous vous êtes connecté à `gh` via le navigateur, le jeton appartient à l'entrée **GitHub CLI** sous [**Paramètres > Applications > Applications OAuth autorisées**](https://github.com/settings/applications) sur GitHub, et révoquer cette entrée déconnecte également la CLI GitHub sur vos machines. Les sessions cloud perdent alors l'accès GitHub jusqu'à ce que vous exécutiez `gh auth login` et `/web-setup` à nouveau.

<h2 id="start-a-task">
  Démarrer une tâche
</h2>

Avec GitHub connecté et un environnement créé, vous êtes prêt à soumettre des tâches.

<Steps>
  <Step title="Sélectionnez un référentiel et une branche">
    Depuis [claude.ai/code](https://claude.ai/code) ou l'onglet Code dans l'application mobile Claude, cliquez sur le sélecteur de référentiel sous la zone de saisie et choisissez un référentiel dans lequel Claude doit travailler. Chaque référentiel affiche un sélecteur de branche. Changez-le pour démarrer Claude à partir d'une branche de fonctionnalité au lieu de la branche par défaut. Vous pouvez ajouter plusieurs référentiels pour travailler sur plusieurs dans une seule session.
  </Step>

  <Step title="Choisissez un mode de permission">
    Le menu déroulant du mode à côté de l'entrée affiche le mode dans lequel la session s'exécutera :

    * **Auto** : un classificateur examine les actions de Claude au lieu de vous demander. Apparaît lorsque votre organisation autorise le mode auto et que le modèle sélectionné le supporte
    * **Accepter les modifications** : Claude effectue des modifications et pousse une branche sans s'arrêter pour approbation
    * **Plan** : Claude propose une approche et attend votre approbation avant de modifier les fichiers

    Les sessions cloud n'offrent pas les permissions Manual ou Bypass. Consultez la [liste complète des modes de permission](/docs/fr/permission-modes#available-modes) pour savoir ce que chacun permet.
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
| `environment`  | Nom ou ID de l'[environnement](#connect-github) à présélectionner.                                                                                                                                              |

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

    Le diff compare les modifications de la session par rapport à sa branche de base par défaut. Pour comparer par rapport à une branche différente, sélectionnez **Comparer par rapport à** et choisissez-en une.
  </Step>

  <Step title="Laissez des commentaires en ligne">
    Sélectionnez n'importe quelle ligne dans le diff, tapez vos commentaires et appuyez sur Entrée. Les commentaires s'accumulent jusqu'à ce que vous envoyiez votre prochain message, puis ils sont regroupés avec celui-ci. Claude voit « à `src/auth.ts:47`, ne capturez pas l'erreur ici » aux côtés de votre instruction principale, vous n'avez donc pas à décrire où se trouve le problème.
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

Si vous avez connecté GitHub dans le navigateur, les sessions peuvent cloner n'importe quel référentiel public, mais un référentiel privé n'apparaît que lorsque l'application Claude GitHub est installée sur le compte ou l'organisation qui le possède et que l'accès aux référentiels de l'installation l'inclut. [Installez l'application Claude GitHub](https://github.com/apps/claude/installations/new) là, ou demandez à un propriétaire d'organisation de l'installer ou de l'approuver.

Si vous avez connecté avec `/web-setup`, les sessions accèdent à chaque référentiel que votre jeton `gh` peut accéder. Exécutez `gh repo view OWNER/REPO` dans votre shell pour vérifier que votre connexion CLI GitHub peut voir le référentiel, et exécutez `/web-setup` à nouveau si vous avez changé de compte `gh` depuis la connexion.

<h3 id="the-page-only-shows-a-github-login-button">
  La page affiche uniquement un bouton de connexion GitHub
</h3>

Les sessions cloud nécessitent un compte GitHub connecté. Connectez-vous via le flux du navigateur ci-dessus, ou exécutez `/web-setup` depuis votre terminal si vous utilisez la CLI GitHub. Si vous préférez ne pas connecter GitHub du tout, consultez [Remote Control](/docs/fr/remote-control) pour exécuter Claude Code sur votre propre machine et le surveiller depuis votre navigateur ou votre téléphone.

<h3 id="not-available-for-the-selected-organization">
  « Non disponible pour l'organisation sélectionnée »
</h3>

Les organisations Enterprise peuvent avoir besoin qu'un propriétaire active les sessions cloud. Contactez votre équipe de compte Anthropic.

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup` indique « Non connecté à Claude »
</h3>

Si `/web-setup` répond avec « Not signed in to Claude. Run /login first. », la CLI n'a pas de connexion claude.ai valide. Cela peut également se produire quand une connexion précédente a expiré. Exécutez `/login`, connectez-vous avec votre compte claude.ai, puis exécutez `/web-setup` à nouveau.

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup` avertit que votre jeton n'a pas la portée `workflow`
</h3>

Si `/web-setup` indique que votre jeton CLI GitHub n'a pas la portée `workflow`, vous pouvez continuer, mais GitHub peut rejeter certains envois effectués avec ce jeton, comme les envois qui modifient les fichiers de flux de travail GitHub Actions. Pour ajouter la portée, exécutez `gh auth refresh -s workflow` dans votre shell, puis exécutez `/web-setup` à nouveau.

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` affiche « Aucune commande ne correspond » ou « Commande inconnue »
</h3>

`/web-setup` s'exécute à l'intérieur de la CLI Claude Code, pas votre shell. Lancez `claude` d'abord, puis tapez `/web-setup` à l'invite.

Si vous l'avez tapé à l'intérieur de Claude Code et le menu de commandes affiche `No commands match "/web-setup"`, ou que le soumettre retourne `Unknown command: /web-setup`, la commande est masquée parce qu'une exigence n'est pas satisfaite. La cause est généralement que vous êtes authentifié avec une clé API ou un fournisseur tiers au lieu d'un abonnement claude.ai. Exécutez `/login` pour vous connecter avec votre compte claude.ai.

Sur les plans Team et Enterprise, la commande est masquée par défaut : le [commutateur de configuration web rapide](/docs/fr/claude-code-on-the-web#github-authentication-options) est désactivé jusqu'à ce qu'un propriétaire l'active. Pendant qu'il est désactivé, [connectez GitHub depuis le navigateur](#connect-github) à la place.

La commande est également masquée dans deux autres cas :

* Un administrateur a désactivé les sessions cloud pour votre organisation. Dans ce cas, soumettre `/web-setup` retourne [`Cloud sessions are disabled by your organization's policy`](/docs/fr/errors#cloud-sessions-are-disabled-by-your-organizations-policy). Avant la v2.1.268, ce cas retournait également `Unknown command: /web-setup`.
* Votre organisation Enterprise a [Zero Data Retention](/docs/fr/zero-data-retention) activé, ce qui rend les sessions cloud indisponibles.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  « Impossible de créer un environnement cloud » ou « Aucun environnement cloud disponible » lors de l'utilisation de `--cloud`
</h3>

Les fonctionnalités de session cloud créent automatiquement un environnement cloud par défaut si vous n'en avez pas. Si vous voyez « Impossible de créer un environnement cloud », la création automatique a échoué. Si vous voyez « Aucun environnement cloud disponible », votre CLI est antérieur à la création automatique. Dans les deux cas, exécutez `/web-setup` dans la CLI Claude Code, ou ajoutez un environnement à partir du [sélecteur d'environnement](/docs/fr/cloud-environments#configure-your-environment) sur [claude.ai/code](https://claude.ai/code).

<h3 id="setup-script-failed">
  Le script de configuration a échoué
</h3>

Le script de configuration s'est terminé avec un statut non-zéro, ce qui bloque le démarrage de la session. Les causes courantes :

* Une installation de paquet a échoué parce que le registre n'est pas dans votre [niveau d'accès réseau](/docs/fr/cloud-environments#access-levels). `Trusted` couvre la plupart des gestionnaires de paquets ; `None` les bloque tous.
* Le script fait référence à un fichier ou un chemin qui n'existe pas dans un clone frais.
* Une commande qui fonctionne localement a besoin d'une invocation différente sur Ubuntu.

Pour déboguer, ajoutez `set -x` en haut du script pour voir quelle commande a échoué. Pour les commandes non critiques, ajoutez `|| true` pour qu'elles ne bloquent pas le démarrage de la session.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Les nouvelles sessions se figent ou expirent pendant la configuration
</h3>

Si les nouvelles sessions se figent à l'étape du script de configuration ou échouent avec une erreur de conteneur générique avant la fin du script, le script dépasse probablement le budget de temps d'environ cinq minutes pour construire le [cache d'environnement](/docs/fr/cloud-environments#environment-caching). Les étapes lourdes telles que l'extraction d'images Docker volumineuses, la synchronisation d'arbres de dépendances complets ou le téléchargement de poids de modèles dépassent souvent la limite, surtout quand elles s'exécutent l'une après l'autre.

Pour corriger cela, réduisez le script pour qu'il se termine de manière fiable en moins de cinq minutes :

* Exécutez les installations indépendantes en parallèle avec `&` et un `wait` final au lieu de les exécuter en série.
* Déplacez les plus grands téléchargements hors du script de configuration et dans un [hook SessionStart](/docs/fr/cloud-environments#setup-scripts-vs-sessionstart-hooks) qui les lance en arrière-plan, pour que la session devienne utilisable pendant qu'ils se terminent.
* Supprimez les longs délais de nouvelle tentative du script de configuration, car une boucle de nouvelle tentative figée compte dans le budget.

<h3 id="session-keeps-running-after-closing-the-tab">
  La session continue de s'exécuter après la fermeture de l'onglet
</h3>

C'est intentionnel. Fermer l'onglet ou naviguer ailleurs n'arrête pas la session. Elle continue de s'exécuter en arrière-plan jusqu'à ce que Claude termine la tâche actuelle, puis elle reste inactive. Depuis la barre latérale, vous pouvez [archiver une session](/docs/fr/claude-code-on-the-web#archive-sessions) pour la masquer de votre liste, ou [la supprimer](/docs/fr/claude-code-on-the-web#delete-sessions) pour la supprimer définitivement.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous pouvez soumettre et examiner des tâches, ces pages couvrent ce qui vient ensuite : démarrer des sessions cloud depuis votre terminal, planifier des travaux récurrents et donner à Claude des instructions permanentes.

* [Utiliser Claude Code sur le web](/docs/fr/claude-code-on-the-web) : la référence complète, y compris la téléportation de sessions vers votre terminal, le partage de sessions et la correction automatique des demandes de tirage
* [Configurer les environnements cloud](/docs/fr/cloud-environments) : niveaux d'accès réseau, variables d'environnement et scripts de configuration pour les sessions cloud
* [Routines](/docs/fr/routines) : automatisez le travail selon un calendrier, via un appel API ou en réponse aux événements GitHub
* [CLAUDE.md](/docs/fr/memory) : donnez à Claude des instructions et un contexte persistants qui se chargent au début de chaque session
* Installez l'application mobile Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) pour surveiller les sessions depuis votre téléphone. Depuis la CLI Claude Code, `/mobile` affiche un code QR pour [claude.ai/mobile](https://claude.ai/mobile) qui ouvre le bon app store pour votre téléphone.
