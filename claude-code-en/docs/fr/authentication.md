> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Authentification

> Connectez-vous à Claude Code et configurez l'authentification pour les particuliers, les équipes et les organisations.

Claude Code prend en charge plusieurs méthodes d'authentification selon votre configuration. Les utilisateurs individuels peuvent se connecter avec un compte Claude.ai, tandis que les équipes peuvent utiliser Claude for Teams ou Enterprise, la Claude Console, ou un fournisseur cloud comme Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Se connecter à Claude Code
</h2>

Après [l'installation de Claude Code](/docs/fr/setup#install-claude-code), exécutez `claude` dans votre terminal. Au premier lancement, Claude Code ouvre une fenêtre de navigateur pour vous permettre de vous connecter. Si vous avez défini la variable d'environnement `ANTHROPIC_API_KEY`, Claude Code ignore l'invite de connexion et vous demande plutôt d'approuver la clé.

Si le navigateur ne s'ouvre pas automatiquement, appuyez sur `c` pour copier l'URL de connexion dans votre presse-papiers, puis collez-la dans votre navigateur.

Si votre navigateur affiche un code de connexion au lieu de vous rediriger après votre connexion, collez-le dans le terminal à l'invite `Paste code here if prompted`. Cela se produit lorsque le navigateur ne peut pas atteindre le serveur de rappel local de Claude Code, ce qui est courant dans WSL2, les sessions SSH et les conteneurs.

Lorsque la connexion est terminée, le terminal affiche `Login successful` et vous invite à appuyer sur `Entrée` pour continuer.

Vous pouvez vous authentifier avec l'un de ces types de compte :

* **Abonnement Claude Pro ou Max** : connectez-vous avec votre compte claude.ai. Abonnez-vous sur [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams ou Enterprise** : connectez-vous avec le compte claude.ai que votre administrateur d'équipe vous a invité à utiliser.
* **Claude Console** : connectez-vous avec vos identifiants Console. Votre administrateur doit vous avoir [invité](#claude-console-authentication) au préalable. Vous pouvez vous connecter avec ou sans [créer une clé API](#sign-in-without-an-api-key).
* **Fournisseurs cloud** : si votre organisation utilise [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) ou [Microsoft Foundry](/docs/fr/microsoft-foundry), définissez les variables d'environnement requises avant d'exécuter `claude`, ou sélectionnez **3rd-party platform** à l'invite de connexion, ce qui lance un assistant de configuration interactif pour Bedrock et Vertex AI. Aucune connexion au navigateur n'est nécessaire.
* **Passerelle cloud** : si votre organisation exécute une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée, connectez-vous avec l'authentification unique d'entreprise via `/login`. Le jeton émis par la passerelle est la seule credential de la session.

Les administrateurs peuvent diriger la méthode de connexion que les développeurs utilisent et exiger que les connexions claude.ai appartiennent à une organisation spécifique ; voir [Restreindre la connexion à votre organisation](#restrict-login-to-your-organization).

Pour vous déconnecter et vous réauthentifier, tapez `/logout` à l'invite Claude Code. La déconnexion réinitialise également votre état de configuration au premier lancement, de sorte que la prochaine fois que vous exécutez `claude`, il vous guide à nouveau à travers la connexion et la configuration.

Si vous avez des difficultés à vous connecter, consultez [dépannage de l'authentification](/docs/fr/troubleshoot-install#login-and-authentication).

<h2 id="set-up-team-authentication">
  Configurer l'authentification d'équipe
</h2>

Pour les équipes et les organisations, vous pouvez configurer l'accès à Claude Code de l'une de ces façons :

* [Claude for Teams ou Enterprise](#claude-for-teams-or-enterprise), recommandé pour la plupart des équipes
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/fr/claude-apps-gateway), une passerelle auto-hébergée qui connecte les développeurs avec votre IdP et achemine l'inférence vers le fournisseur cloud que vous configurez
* [Amazon Bedrock](/docs/fr/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai)
* [Microsoft Foundry](/docs/fr/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams ou Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) et [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) offrent la meilleure expérience pour les organisations utilisant Claude Code. Les membres de l'équipe ont accès à la fois à Claude Code et à Claude sur le web avec facturation centralisée et gestion d'équipe.

* **Claude for Teams** : plan en libre-service avec fonctionnalités de collaboration, outils d'administration, SSO, gestion de la facturation et [paramètres gérés par serveur](/docs/fr/server-managed-settings) pour la configuration Claude Code à l'échelle de l'organisation. Idéal pour les petites équipes.
* **Claude for Enterprise** : ajoute capture de domaine, permissions basées sur les rôles et l'API de conformité. Idéal pour les grandes organisations ayant des exigences en matière de sécurité et de conformité.

<Steps>
  <Step title="S'abonner">
    Abonnez-vous à [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) ou contactez l'équipe commerciale pour [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Inviter les membres de l'équipe">
    Invitez les membres de l'équipe depuis le tableau de bord d'administration.
  </Step>

  <Step title="Installer et se connecter">
    Les membres de l'équipe installent Claude Code et se connectent avec leurs comptes claude.ai.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Authentification Claude Console
</h3>

Pour les organisations qui préfèrent la facturation basée sur l'API, vous pouvez configurer l'accès via la Claude Console.

<Steps>
  <Step title="Créer ou utiliser un compte Console">
    Utilisez votre compte Claude Console existant ou créez-en un nouveau.
  </Step>

  <Step title="Ajouter des utilisateurs">
    Vous pouvez ajouter des utilisateurs par l'une ou l'autre méthode :

    * Inviter en masse des utilisateurs depuis la Console : Settings -> Members -> Invite
    * [Configurer SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="Assigner des rôles">
    Lors de l'invitation d'utilisateurs, assignez l'un des rôles suivants :

    * **Rôle Claude Code** : les utilisateurs ne peuvent créer que des clés API Claude Code
    * **Rôle Developer** : les utilisateurs peuvent créer n'importe quel type de clé API
  </Step>

  <Step title="Les utilisateurs complètent la configuration">
    Chaque utilisateur invité doit :

    * Accepter l'invitation Console
    * [Vérifier la configuration système](/docs/fr/setup#system-requirements)
    * [Installer Claude Code](/docs/fr/setup#install-claude-code)
    * Se connecter avec les identifiants du compte Console
  </Step>
</Steps>

<h4 id="sign-in-without-an-api-key">
  Se connecter sans clé API
</h4>

Vous pouvez vous connecter à votre compte Console sans créer de clé API, même si votre organisation ne permet pas aux développeurs d'en créer. Choisissez le compte Anthropic Console à l'invite `/login` et Claude Code vous demande comment vous souhaitez vous connecter. Nécessite Claude Code v2.1.242 ou version ultérieure. Les deux itinéraires vous connectent à Console dans le navigateur et diffèrent dans ce que Claude Code stocke ensuite :

* **Se connecter avec votre compte Console**, étiqueté `(recommandé)` : Claude Code conserve le jeton OAuth de cette connexion et le stocke en tant que [profil Anthropic](#anthropic-profiles-and-federation-credentials). Il ne crée aucune clé API
* **Créer une clé API**, étiqueté `(hérité)` : Claude Code crée une clé API Console pour vous et la stocke avec vos autres identifiants

En pratique, le profil stocke une connexion OAuth tandis qu'une clé API est un identifiant statique : Claude Code actualise automatiquement la connexion du profil, et lorsque l'actualisation échoue, les demandes échouent avec [Anthropic profile login expired](/docs/fr/errors#anthropic-profile-login-expired) jusqu'à ce que vous vous reconnectiez.

Vous n'avez pas le choix sur chaque machine. Claude Code crée une clé API sans demander dans ces cas :

* Vous exécutez contre un fournisseur cloud, tel que [Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](/docs/fr/third-party-integrations) ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws)
* Tout fichier de paramètres définit [`forceLoginOrgUUID`](#restrict-login-to-your-organization), ou définit `forceLoginMethod` sur `"claudeai"` ou `"console"`
* Une source de paramètres gérés sur votre machine, telle que le fichier de paramètres gérés, un profil MDM ou les paramètres gérés par serveur en cache, existe mais Claude Code [ne peut pas le lire](/docs/fr/managed-settings#invalid-entries-in-managed-settings) et aucune autre source gérée ne fournit de politique

Désactivez `ANTHROPIC_API_KEY` avant de vous connecter sans clé. Un profil écrit par la propre connexion Console de Claude Code, ou par la commande `ant auth login` de la CLI Claude Platform, est le même type d'identifiant, donc se reconnecter le remplace.

Après vous être connecté sans clé, vous avez un profil au lieu d'une clé API stockée :

* **Quel profil il écrit** : Claude Code écrit le profil nommé par `ANTHROPIC_PROFILE`, ou votre profil actif, ou `default`. Si ce profil est un profil de fédération, Claude Code refuse la connexion au lieu de le remplacer
* **Ce dont il vous déconnecte** : Claude Code vous déconnecte de toute connexion claude.ai stockée sur la machine
* **Comment l'annuler** : exécutez `/logout`, qui supprime et révoque l'identifiant que cette connexion a écrit

Si votre organisation utilise les [paramètres gérés par serveur](/docs/fr/server-managed-settings), ils s'appliquent à cette connexion sur Claude Code v2.1.257 ou version ultérieure.

Tout le reste concernant les profils s'applique à cette connexion, y compris son classement par rapport à vos autres identifiants, la ligne `Profile` que vous obtenez dans `/status`, et les fonctionnalités qui nécessitent une connexion claude.ai. Voir [Anthropic profiles and federation credentials](#anthropic-profiles-and-federation-credentials).

<h3 id="cloud-provider-authentication">
  Authentification du fournisseur cloud
</h3>

Pour les équipes utilisant Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry :

<Steps>
  <Step title="Suivre la configuration du fournisseur">
    Suivez la [documentation Amazon Bedrock](/docs/fr/amazon-bedrock), la [documentation Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) ou la [documentation Microsoft Foundry](/docs/fr/microsoft-foundry).
  </Step>

  <Step title="Distribuer la configuration">
    Distribuez les variables d'environnement et les instructions pour générer les identifiants cloud à vos utilisateurs. En savoir plus sur la façon de [gérer la configuration ici](/docs/fr/settings).
  </Step>

  <Step title="Installer Claude Code">
    Les utilisateurs peuvent [installer Claude Code](/docs/fr/setup#install-claude-code).
  </Step>
</Steps>

<h3 id="restrict-login-to-your-organization">
  Restreindre la connexion à votre organisation
</h3>

Pour exiger que les connexions claude.ai des développeurs appartiennent à une organisation Anthropic spécifique, définissez [`forceLoginMethod`](/docs/fr/settings-reference#forceloginmethod) et [`forceLoginOrgUUID`](/docs/fr/settings-reference#forceloginorguuid) dans les [paramètres gérés](/docs/fr/managed-settings). Définissez `forceLoginOrgUUID` sur votre ID d'organisation, affiché dans les [paramètres d'administration claude.ai](https://claude.ai/admin-settings/organization) pour les organisations Claude for Teams ou Enterprise. Claude Code signale une erreur pour une connexion claude.ai à toute autre organisation et se ferme au démarrage si l'identifiant claude.ai en cours d'utilisation appartient à une organisation qui n'est pas répertoriée.

Pour les connexions Claude Console, Claude Code utilise `forceLoginOrgUUID` pour présélectionner l'organisation sur la page de connexion Console lorsque vous la définissez sur un seul ID d'organisation Console, affiché sur [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization). Il ne vérifie pas à quelle organisation appartient l'identifiant Console résultant, à la connexion ou au démarrage, et un développeur qui s'est connecté avec un compte Console avant que vous ayez déployé les clés reste connecté.

Si vous définissez `forceLoginOrgUUID` dans un fichier de paramètres, Claude Code cesse d'offrir la [connexion Console sans clé](#sign-in-without-an-api-key) dans les sessions auxquelles ce fichier s'applique et crée une clé API à la place. Pour diriger les développeurs vers la connexion claude.ai à la place, définissez `forceLoginMethod` sur `"claudeai"`.

Les développeurs peuvent se connecter à partir de plusieurs chemins : le flux terminal `/login`, l'[extension VS Code](/docs/fr/vs-code), l'Agent SDK, `claude setup-token`, `/install-github-app`, et la [connexion gateway](/docs/fr/claude-apps-gateway) pour les organisations qui acheminent via une passerelle cloud. Sur Claude Code v2.1.212 ou version ultérieure, chaque chemin applique `forceLoginMethod` ; avant v2.1.212, seules les connexions terminales appliquaient l'une ou l'autre clé. Sur l'écran de connexion interactif du terminal, accessible par `/login` ou l'intégration au premier démarrage, Claude Code présélectionne une méthode `claudeai` ou `console` sans l'appliquer, donc même avec `forceLoginMethod` défini sur `"claudeai"`, un développeur peut toujours compléter une connexion Console là. Les chemins diffèrent sur `forceLoginOrgUUID` :

* **Connexions terminales, extension VS Code et Agent SDK** : vérifiez `forceLoginOrgUUID` pour les connexions de compte claude.ai
* **`claude setup-token` et `/install-github-app`** : appliquez uniquement `forceLoginMethod`, afin qu'ils puissent créer un jeton dans une organisation différente
* **Connexion [Gateway](/docs/fr/claude-apps-gateway)** : sélectionnée par `forceLoginMethod: "gateway"` plutôt que restreinte par elle, et ne s'authentifie pas auprès d'une organisation Anthropic, donc `forceLoginOrgUUID` ne s'applique pas ; utilisez votre fournisseur d'identité de passerelle pour restreindre l'accès

Déployez les clés via votre outil de gestion des appareils. Les [paramètres gérés par serveur](/docs/fr/server-managed-settings) ne s'appliquent qu'aux comptes déjà authentifiés dans votre organisation, ils ne peuvent donc pas rediriger la première connexion d'un développeur. Si votre organisation distribue également des paramètres gérés par serveur, définissez les clés aux deux endroits : les sources de [paramètres gérés ne fusionnent pas](/docs/fr/server-managed-settings#settings-precedence), et les paramètres gérés par serveur en cache remplacent le fichier géré par appareil, à l'exception de quelques [exceptions par clé](/docs/fr/server-managed-settings#per-key-exceptions-across-managed-sources). `forceLoginOrgUUID` et les valeurs `"claudeai"` et `"console"` de `forceLoginMethod` ne figurent pas parmi ces exceptions, donc gardez-les aux deux endroits.

Les clés décident également si une session qui n'utilise pas d'identifiant de connexion peut démarrer. Voir [`forceLoginOrgUUID`](/docs/fr/settings-reference#forceloginorguuid) dans la référence des paramètres pour le comportement complet.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper`** : bloqués au démarrage, car l'appartenance à l'organisation ne peut pas être vérifiée pour un identifiant d'environnement
* **Sessions de fournisseur cloud telles que Amazon Bedrock** : non bloquées, car elles s'authentifient auprès de votre fournisseur cloud. Restreignez-les via vos politiques IAM cloud
* **[Profil Anthropic ou identifiants de fédération](#anthropic-profiles-and-federation-credentials)** : non bloqués, et les clés ne vérifient pas à quelle organisation le profil appartient

<h2 id="credential-management">
  Gestion des identifiants
</h2>

Claude Code gère de manière sécurisée vos identifiants d'authentification :

* **Emplacement de stockage** :
  * Sur macOS, les identifiants sont stockés dans le Keychain macOS chiffré. Lorsque le Keychain rejette l'écriture, par exemple lorsqu'il est verrouillé dans une session SSH, Claude Code stocke votre connexion dans `~/.claude/.credentials.json` avec le mode fichier `0600` à la place, le même stockage qu'il utilise sur Linux. Une connexion Console qui crée une clé API échoue jusqu'à ce que le Keychain soit accessible en écriture. Pour déplacer votre connexion dans le Keychain, suivez [les étapes de récupération](/docs/fr/troubleshoot-install#not-logged-in-or-token-expired).
  * Sur Linux, les identifiants sont stockés dans `~/.claude/.credentials.json` avec le mode fichier `0600`.
  * Sur Windows, les identifiants sont stockés dans `%USERPROFILE%\.claude\.credentials.json` et héritent des contrôles d'accès de votre répertoire de profil utilisateur, ce qui restreint le fichier à votre compte utilisateur par défaut.
  * Si vous avez défini la variable d'environnement `CLAUDE_CONFIG_DIR`, Claude Code conserve le fichier `.credentials.json` sous ce répertoire à la place, y compris le fichier que le fallback macOS écrit, et clé l'entrée macOS Keychain sur ce répertoire également, de sorte qu'une session avec un `CLAUDE_CONFIG_DIR` différent lit une entrée différente.
  * Claude Code gère `.credentials.json` via `/login` et `/logout`. Pour router les requêtes via un point de terminaison API personnalisé, définissez plutôt la variable d'environnement [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars).
* **Types d'authentification pris en charge** : identifiants Claude.ai, identifiants API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, identifiants de profil Anthropic et [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation), et jetons de session de la [passerelle d'applications Claude](/docs/fr/claude-apps-gateway).
* **Scripts d'identifiants personnalisés** : configurez le paramètre [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) pour exécuter un script shell qui retourne une clé API.
* **Intervalles d'actualisation** : Claude Code réexécute `apiKeyHelper` après cinq minutes par défaut. Définissez la variable d'environnement `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` pour les intervalles d'actualisation personnalisés. Consultez [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) pour les autres cas dans lesquels Claude Code réexécute l'assistant.
* **Avis d'assistant lent** : si `apiKeyHelper` prend plus de 10 secondes pour retourner une clé, Claude Code affiche un avis d'avertissement dans la barre d'invite montrant le temps écoulé. Si vous voyez cet avis régulièrement, vérifiez si votre script d'identifiants peut être optimisé.
* **Échecs de l'assistant** : lorsque le script se termine avec une erreur, expire ou n'affiche rien, les requêtes échouent avec [`Your apiKeyHelper script is failing`](/docs/fr/errors#your-apikeyhelper-script-is-failing) après trois tentatives. Avant v2.1.208, les échecs de l'assistant s'affichaient comme une erreur 401 générique après environ dix tentatives silencieuses.

`apiKeyHelper`, `ANTHROPIC_API_KEY` et `ANTHROPIC_AUTH_TOKEN` s'appliquent à la CLI et aux surfaces qui l'enveloppent, y compris l'extension VS Code, le SDK Agent et GitHub Actions. Claude Desktop et les sessions cloud n'appellent pas `apiKeyHelper` ni ne lisent ces variables d'environnement : elles utilisent OAuth, sauf les sessions de bureau exécutant une [configuration d'inférence tierce](/docs/fr/llm-gateway-connect#desktop-app), qui s'authentifient avec les identifiants de cette configuration.

<h3 id="renew-an-expiring-login">
  Renouveler une connexion qui expire
</h3>

Lorsque la connexion que vous avez créée avec `/login` est à moins de trois jours de l'expiration, Claude Code affiche un avertissement au démarrage : `Your login expires in 3 days · run /login to renew`. Nécessite Claude Code v2.1.203 ou version ultérieure. Avant v2.1.217, l'avertissement apparaissait cinq jours avant.

Exécutez `/login` pour renouveler. L'avertissement est informatif et ne bloque jamais une requête : l'authentification continue de fonctionner jusqu'à ce que la connexion expire réellement. La durée de vie de la connexion elle-même est inchangée ; l'avertissement préalable est ce que v2.1.203 ajoute.

Une fois que la connexion stockée expire et ne peut pas être actualisée, chaque requête de modèle échoue avec [`Login expired · Please run /login`](/docs/fr/errors#login-expired) jusqu'à ce que vous vous reconnectiez. Avant v2.1.206, Claude Code signalait une connexion expirée sur les requêtes de modèle comme une erreur de modèle à la place.

Vous pouvez vérifier cet état avant qu'une requête échoue : [`/status`](/docs/fr/commands) affiche une ligne `Login` indiquant `Expired — log in again`, plus l'organisation et l'e-mail qu'il a enregistrés pour la connexion expirée. La ligne n'apparaît que lorsque la connexion claude.ai ou Claude Console enregistrée est l'identifiant actif. La ligne nécessite Claude Code v2.1.210 ou version ultérieure.

L'avertissement n'apparaît que lorsqu'une connexion claude.ai ou Claude Console est l'identifiant actif, et non lorsqu'un fournisseur cloud, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` fournit l'identifiant.

Le renouvellement anticipé est plus important pour les sessions qui s'exécutent sans surveillance. Une [session en arrière-plan en vue agent](/docs/fr/agent-view) ou une session [Remote Control](/docs/fr/remote-control) qui dépasse la durée de vie de la connexion cesse de progresser une fois que l'identifiant expire et ne peut pas récupérer jusqu'à ce que vous vous reconnectiez.

<h3 id="authentication-precedence">
  Ordre de priorité de l'authentification
</h3>

Lorsque plusieurs identifiants sont présents, Claude Code en choisit un dans cet ordre :

1. Identifiants du fournisseur cloud, lorsque `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` est défini. Consultez [intégrations tierces](/docs/fr/third-party-integrations) pour la configuration.
2. Variable d'environnement `ANTHROPIC_AUTH_TOKEN`. Envoyée en tant qu'en-tête `Authorization: Bearer`. Utilisez ceci lors du routage via une [passerelle LLM ou proxy](/docs/fr/llm-gateway) qui s'authentifie avec des jetons porteurs plutôt que des clés API Anthropic.
3. Variable d'environnement `ANTHROPIC_API_KEY`. Envoyée en tant qu'en-tête `X-Api-Key`. Utilisez ceci pour l'accès direct à l'API Anthropic avec une clé de la [Claude Console](https://platform.claude.com). En mode interactif, vous êtes invité une fois à approuver ou refuser la clé, et votre choix est mémorisé. Pour le modifier ultérieurement, utilisez le bouton bascule « Use custom API key » dans `/config`. Le bouton bascule n'apparaît que lorsque `ANTHROPIC_API_KEY` est défini dans votre environnement. En mode non interactif (`-p`), la clé est toujours utilisée lorsqu'elle est présente.
4. Sortie du script [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper). Utilisez ceci pour les identifiants dynamiques ou rotatifs, tels que les jetons de courte durée récupérés à partir d'un coffre-fort.
5. Variable d'environnement `CLAUDE_CODE_OAUTH_TOKEN`. Un jeton OAuth de longue durée généré par [`claude setup-token`](#generate-a-long-lived-token). Utilisez ceci pour les pipelines CI et les scripts où la connexion au navigateur n'est pas disponible. Si vous exécutez `/login` alors que la variable est définie, Claude Code bascule la session actuelle vers la nouvelle connexion, mais relit la variable dans chaque nouvelle session jusqu'à ce que vous la supprimiez de votre profil shell ou du bloc `env` d'un [fichier de paramètres](/docs/fr/settings).
6. Identifiants de profil Anthropic et de fédération, les identifiants que la CLI `ant` et Workload Identity Federation utilisent. Un profil que `ant auth login` a écrit ne se classe ici que lorsque vous le nommez dans `ANTHROPIC_PROFILE` ; sinon, il se classe en dessous de `/login`. Consultez [Profils Anthropic et identifiants de fédération](#anthropic-profiles-and-federation-credentials).
7. Identifiants OAuth d'abonnement de `/login`. C'est la valeur par défaut pour les utilisateurs Claude Pro, Max, Team et Enterprise.

Une session [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) signée se situe en dehors de cette liste : c'est une sélection de fournisseur comme Amazon Bedrock ou Google Cloud's Agent Platform, et elle les surclasse. Lorsqu'une session de passerelle existe, la CLI s'authentifie avec le jeton de passerelle même si `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` est défini, et les sources d'identifiants ci-dessus telles que le jeton porteur, la clé API, `apiKeyHelper` et les profils ne sont pas utilisés.

Si les [paramètres gérés](/docs/fr/managed-settings) de votre machine définissent [`forceLoginMethod`](/docs/fr/settings-reference#forceloginmethod) sur `"gateway"` ou définissent [`forceLoginGatewayUrl`](/docs/fr/settings-reference#forcelogingatewayurl), et que vous ne sélectionnez pas un fournisseur cloud via une variable telle que `CLAUDE_CODE_USE_BEDROCK` ou `CLAUDE_CODE_USE_VERTEX`, votre session utilise uniquement la connexion de passerelle. Claude Code ignore les autres sources d'identifiants et vous demande de vous connecter avec `/login`. Consultez [Administrator policy requires a Cloud gateway sign-in](/docs/fr/errors#administrator-policy-requires-a-cloud-gateway-sign-in) pour voir ce que vous voyez avec chaque identifiant restant. Avant v2.1.261, ou avant v2.1.265 sur une machine qui définit uniquement `forceLoginGatewayUrl`, Claude Code utilisait une connexion enregistrée restante sur ces machines jusqu'à ce que vous vous connectiez à la passerelle.

Si vous avez un abonnement Claude actif mais que vous avez également `ANTHROPIC_API_KEY` défini dans votre environnement, Claude Code utilise la clé API une fois approuvée. Cela peut causer des échecs d'authentification si la clé appartient à une organisation désactivée ou expirée.

Exécutez `unset ANTHROPIC_API_KEY` pour revenir à votre abonnement, et vérifiez `/status` pour confirmer quelle méthode est active. Lorsqu'une connexion et une clé API sont toutes deux configurées, `/status` marque l'identifiant qui n'est pas en cours d'utilisation.

[Claude Code sur le Web](/docs/fr/claude-code-on-the-web) utilise toujours vos identifiants d'abonnement. Si vous définissez `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN` dans l'environnement cloud, cela ne remplace pas vos identifiants d'abonnement.

<h4 id="anthropic-profiles-and-federation-credentials">
  Profils Anthropic et identifiants de fédération
</h4>

Un profil est un fichier de configuration d'identifiants nommé dans votre [répertoire de configuration Anthropic](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory), par défaut `~/.config/anthropic` sur macOS et Linux ou `%APPDATA%\Anthropic` sur Windows. Le mode d'authentification d'un profil est `oidc_federation` lorsque vous le configurez pour [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) ou `user_oauth` lorsque [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication) l'a écrit ou que vous [vous êtes connecté à un compte Console sans clé API](#sign-in-without-an-api-key).

Claude Code ne lit pas les profils ou les variables de fédération en [mode bare](/docs/fr/headless#start-faster-with-bare-mode), dans Claude Desktop ou dans les sessions cloud. Dans ces sessions, `/status` n'affiche aucune ligne `Profile`.

Claude Code vérifie trois sources dans cet ordre et s'arrête à la première qui est définie. Le tableau montre ce qui définit chaque source et où elle se classe par rapport à votre identifiant `/login`.

| Source                  | Défini par                                                                                                                                                                        | Rang par rapport à `/login`                                                                                                                                                  |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Profil nommé            | `ANTHROPIC_PROFILE`                                                                                                                                                               | Au-dessus, quel que soit le mode d'authentification du profil                                                                                                                |
| Variables de fédération | `ANTHROPIC_FEDERATION_RULE_ID` et `ANTHROPIC_ORGANIZATION_ID`, tous deux définis                                                                                                  | Au-dessus                                                                                                                                                                    |
| Profil actif            | Le fichier [`active_config`](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) dans votre répertoire de configuration, ou un profil nommé `default` | Au-dessus lorsque son mode d'authentification est `oidc_federation` ; en dessous d'un identifiant `/login` fonctionnant lorsque son mode d'authentification est `user_oauth` |

La règle `user_oauth` empêche un profil `ant auth login` résiduel de déplacer vos requêtes hors du compte auquel vous vous êtes connecté avec `/login`. Pour les variables de fédération, Claude Code lit également les autres variables dans la [référence WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables), telles que `ANTHROPIC_IDENTITY_TOKEN_FILE`, lorsqu'il échange votre jeton d'identité. Pour le format du fichier de profil, consultez la [référence WIF](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file).

Pour confirmer quelle source Claude Code a choisie, exécutez `/status`. Une ligne `Profile` nomme la source à la place de la ligne `Login method`. Lorsque le profil est l'identifiant en cours d'utilisation, les lignes `Organization` et `Email` affichent son compte.

Si vous démarrez Claude Code avec `--debug`, il écrit également une ligne `Using Anthropic profile auth` avec le nom de la source dans le journal de débogage à `~/.claude/debug/<session-id>.txt`. Lorsque Claude Code passe sur un profil actif `user_oauth` parce que vous avez un identifiant `/login` fonctionnant, il écrit un avertissement dans le journal de débogage indiquant qu'il utilise la connexion claude.ai à la place.

Lorsque la connexion d'un profil `user_oauth` a expiré et que Claude Code ne peut pas la renouveler, les requêtes échouent avec [Anthropic profile login expired](/docs/fr/errors#anthropic-profile-login-expired).

Les fonctionnalités qui nécessitent votre connexion claude.ai, telles que [les connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) et [`/schedule`](/docs/fr/routines), ne sont pas disponibles lorsqu'une de ces sources est sélectionnée. Pour empêcher Claude Code de sélectionner une source :

* **Profil nommé ou variables de fédération** : désactivez `ANTHROPIC_PROFILE`, ou désactivez l'une des variables de fédération
* **Profil actif** : exécutez `/logout` pour un profil `user_oauth` dont l'identifiant actuel vous avez écrit en [vous connectant à un compte Console sans clé API](#sign-in-without-an-api-key), exécutez `ant auth logout` pour un dont l'identifiant actuel `ant auth login` a écrit, ou supprimez le fichier du profil de `configs/` dans votre répertoire de configuration pour l'un ou l'autre mode d'authentification

<h3 id="generate-a-long-lived-token">
  Générer un jeton de longue durée
</h3>

Pour les pipelines CI, les scripts ou d'autres environnements où la connexion au navigateur interactif n'est pas disponible, générez un jeton OAuth d'un an avec `claude setup-token` :

```bash theme={null}
claude setup-token
```

La commande ouvre le même flux d'autorisation du navigateur que `/login`, et le jeton s'affiche dans le terminal après que vous ayez approuvé l'accès dans le navigateur. Il ne sauvegarde le jeton nulle part ; copiez-le et définissez-le en tant que variable d'environnement `CLAUDE_CODE_OAUTH_TOKEN` partout où vous souhaitez vous authentifier :

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Ce jeton s'authentifie avec votre abonnement Claude et nécessite un plan Pro, Max, Team ou Enterprise. Il ne peut faire que des requêtes de modèle, il ne peut donc pas établir de sessions [Remote Control](/docs/fr/remote-control) ou récupérer [les connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai). Les serveurs MCP que vous configurez localement fonctionnent toujours.

[Le mode bare](/docs/fr/headless#start-faster-with-bare-mode) ne lit pas `CLAUDE_CODE_OAUTH_TOKEN`. Si votre script passe `--bare`, authentifiez-vous avec `ANTHROPIC_API_KEY` ou un `apiKeyHelper` à la place.
