> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Authentification

> Connectez-vous à Claude Code et configurez l'authentification pour les particuliers, les équipes et les organisations.

Claude Code prend en charge plusieurs méthodes d'authentification selon votre configuration. Les utilisateurs individuels peuvent se connecter avec un compte Claude.ai, tandis que les équipes peuvent utiliser Claude for Teams ou Enterprise, la Claude Console, ou un fournisseur cloud comme Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.

<h2 id="log-in-to-claude-code">
  Se connecter à Claude Code
</h2>

Après [l'installation de Claude Code](/docs/fr/setup#install-claude-code), exécutez `claude` dans votre terminal. Au premier lancement, Claude Code ouvre une fenêtre de navigateur pour vous permettre de vous connecter.

Si le navigateur ne s'ouvre pas automatiquement, appuyez sur `c` pour copier l'URL de connexion dans votre presse-papiers, puis collez-la dans votre navigateur.

Si votre navigateur affiche un code de connexion au lieu de vous rediriger après votre connexion, collez-le dans le terminal à l'invite `Paste code here if prompted`. Cela se produit lorsque le navigateur ne peut pas atteindre le serveur de rappel local de Claude Code, ce qui est courant dans WSL2, les sessions SSH et les conteneurs.

Lorsque la connexion est terminée, le terminal affiche `Login successful` et vous invite à appuyer sur `Entrée` pour continuer.

Vous pouvez vous authentifier avec l'un de ces types de compte :

* **Abonnement Claude Pro ou Max** : connectez-vous avec votre compte Claude.ai. Abonnez-vous sur [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max).
* **Claude for Teams ou Enterprise** : connectez-vous avec le compte Claude.ai que votre administrateur d'équipe vous a invité à utiliser.
* **Claude Console** : connectez-vous avec vos identifiants Console. Votre administrateur doit vous avoir [invité](#claude-console-authentication) au préalable.
* **Fournisseurs cloud** : si votre organisation utilise [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) ou [Microsoft Foundry](/docs/fr/microsoft-foundry), définissez les variables d'environnement requises avant d'exécuter `claude`, ou sélectionnez **3rd-party platform** à l'invite de connexion, ce qui lance un assistant de configuration interactif pour Bedrock et Vertex AI. Aucune connexion au navigateur n'est nécessaire.
* **Passerelle cloud** : si votre organisation exécute une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée, connectez-vous avec l'authentification unique d'entreprise via `/login`. Le jeton émis par la passerelle est la seule credential de la session.

Les administrateurs peuvent restreindre la connexion interactive avec les paramètres gérés [`forceLoginMethod` et `forceLoginOrgUUID`](/docs/fr/settings#available-settings). Lorsque l'un d'eux est défini, les sessions authentifiées par `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` sont bloquées au démarrage ; les sessions des fournisseurs cloud ne sont pas affectées.

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

* **Claude for Teams** : plan en libre-service avec fonctionnalités de collaboration, outils d'administration et gestion de la facturation. Idéal pour les petites équipes.
* **Claude for Enterprise** : ajoute SSO, capture de domaine, permissions basées sur les rôles, API de conformité et paramètres de politique gérés pour les configurations Claude Code à l'échelle de l'organisation. Idéal pour les grandes organisations ayant des exigences en matière de sécurité et de conformité.

<Steps>
  <Step title="S'abonner">
    Abonnez-vous à [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) ou contactez l'équipe commerciale pour [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step).
  </Step>

  <Step title="Inviter les membres de l'équipe">
    Invitez les membres de l'équipe depuis le tableau de bord d'administration.
  </Step>

  <Step title="Installer et se connecter">
    Les membres de l'équipe installent Claude Code et se connectent avec leurs comptes Claude.ai.
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

<h2 id="credential-management">
  Gestion des identifiants
</h2>

Claude Code gère de manière sécurisée vos identifiants d'authentification :

* **Emplacement de stockage** :
  * Sur macOS, les identifiants sont stockés dans le Keychain macOS chiffré.
  * Sur Linux, les identifiants sont stockés dans `~/.claude/.credentials.json` avec le mode fichier `0600`.
  * Sur Windows, les identifiants sont stockés dans `%USERPROFILE%\.claude\.credentials.json` et héritent des contrôles d'accès de votre répertoire de profil utilisateur, ce qui restreint le fichier à votre compte utilisateur par défaut.
  * Si vous avez défini la variable d'environnement `CLAUDE_CONFIG_DIR` sur Linux ou Windows, le fichier `.credentials.json` se trouve sous ce répertoire à la place.
  * Claude Code gère `.credentials.json` via `/login` et `/logout`. Pour router les requêtes via un point de terminaison API personnalisé, définissez plutôt la variable d'environnement [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars).
* **Types d'authentification pris en charge** : identifiants Claude.ai, identifiants API Claude, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth et jetons de session de la [passerelle d'applications Claude](/docs/fr/claude-apps-gateway).
* **Scripts d'identifiants personnalisés** : le paramètre [`apiKeyHelper`](/docs/fr/settings#available-settings) peut être configuré pour exécuter un script shell qui retourne une clé API.
* **Intervalles d'actualisation** : par défaut, `apiKeyHelper` est appelé après 5 minutes ou en réponse HTTP 401. Définissez la variable d'environnement `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` pour les intervalles d'actualisation personnalisés.
* **Avis d'assistant lent** : si `apiKeyHelper` prend plus de 10 secondes pour retourner une clé, Claude Code affiche un avis d'avertissement dans la barre d'invite montrant le temps écoulé. Si vous voyez cet avis régulièrement, vérifiez si votre script d'identifiants peut être optimisé.
* **Échecs de l'assistant** : lorsque le script se termine avec une erreur, expire ou n'affiche rien, les requêtes échouent avec [`Your apiKeyHelper script is failing`](/docs/fr/errors#your-apikeyhelper-script-is-failing) après trois tentatives. Avant v2.1.208, les échecs de l'assistant s'affichaient comme une erreur 401 générique après environ dix tentatives silencieuses.

`apiKeyHelper`, `ANTHROPIC_API_KEY` et `ANTHROPIC_AUTH_TOKEN` s'appliquent à la CLI et aux surfaces qui l'enveloppent, y compris l'extension VS Code, le SDK Agent et GitHub Actions. Claude Desktop et les sessions cloud n'appellent pas `apiKeyHelper` ni ne lisent ces variables d'environnement : elles utilisent OAuth, sauf les sessions de bureau exécutant une [configuration d'inférence tierce](/docs/fr/llm-gateway-connect#desktop-app), qui s'authentifient avec les identifiants de cette configuration.

<h3 id="renew-an-expiring-login">
  Renouveler une connexion qui expire
</h3>

Lorsque la connexion que vous avez créée avec `/login` est à moins de cinq jours de l'expiration, Claude Code affiche un avertissement au démarrage : `Your login expires in 3 days · run /login to renew`. Nécessite Claude Code v2.1.203 ou version ultérieure.

Exécutez `/login` pour renouveler. L'avertissement est informatif et ne bloque jamais une requête : l'authentification continue de fonctionner jusqu'à ce que la connexion expire réellement. La durée de vie de la connexion elle-même est inchangée ; l'avertissement préalable est ce que v2.1.203 ajoute.

Une fois que la connexion stockée expire et ne peut pas être actualisée, chaque requête échoue avec [`Login expired · Please run /login`](/docs/fr/errors#login-expired) jusqu'à ce que vous vous reconnectiez. Avant v2.1.206, une connexion expirée s'affichait comme une erreur de modèle à la place.

L'avertissement n'apparaît que lorsqu'une connexion claude.ai ou Claude Console est l'identifiant actif, et non lorsqu'un fournisseur cloud, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` fournit l'identifiant.

Le renouvellement anticipé est plus important pour les sessions qui s'exécutent sans surveillance. Une [session en arrière-plan en vue agent](/docs/fr/agent-view) ou une session [Remote Control](/docs/fr/remote-control) qui dépasse la durée de vie de la connexion cesse de progresser une fois que l'identifiant expire et ne peut pas récupérer jusqu'à ce que vous vous reconnectiez.

<h3 id="authentication-precedence">
  Ordre de priorité de l'authentification
</h3>

Lorsque plusieurs identifiants sont présents, Claude Code en choisit un dans cet ordre :

1. Identifiants du fournisseur cloud, lorsque `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` est défini. Consultez [intégrations tierces](/docs/fr/third-party-integrations) pour la configuration.
2. Variable d'environnement `ANTHROPIC_AUTH_TOKEN`. Envoyée en tant qu'en-tête `Authorization: Bearer`. Utilisez ceci lors du routage via une [passerelle LLM ou proxy](/docs/fr/llm-gateway) qui s'authentifie avec des jetons porteurs plutôt que des clés API Anthropic.
3. Variable d'environnement `ANTHROPIC_API_KEY`. Envoyée en tant qu'en-tête `X-Api-Key`. Utilisez ceci pour l'accès direct à l'API Anthropic avec une clé de la [Claude Console](https://platform.claude.com). En mode interactif, vous êtes invité une fois à approuver ou refuser la clé, et votre choix est mémorisé. Pour le modifier ultérieurement, utilisez le bouton bascule « Use custom API key » dans `/config`. Le bouton bascule n'apparaît que lorsque `ANTHROPIC_API_KEY` est défini dans votre environnement. En mode non interactif (`-p`), la clé est toujours utilisée lorsqu'elle est présente.
4. Sortie du script [`apiKeyHelper`](/docs/fr/settings#available-settings). Utilisez ceci pour les identifiants dynamiques ou rotatifs, tels que les jetons de courte durée récupérés à partir d'un coffre-fort.
5. Variable d'environnement `CLAUDE_CODE_OAUTH_TOKEN`. Un jeton OAuth de longue durée généré par [`claude setup-token`](#generate-a-long-lived-token). Utilisez ceci pour les pipelines CI et les scripts où la connexion au navigateur n'est pas disponible.
6. Identifiants OAuth d'abonnement de `/login`. C'est la valeur par défaut pour les utilisateurs Claude Pro, Max, Team et Enterprise.

Une session [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) signée se situe en dehors de cette liste : c'est une sélection de fournisseur comme Amazon Bedrock ou Google Cloud's Agent Platform, et elle les surclasse. Lorsqu'une session de passerelle existe, la CLI s'authentifie avec le jeton de passerelle même si `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX` ou `CLAUDE_CODE_USE_FOUNDRY` est défini, et les entrées de jeton porteur, clé API et `apiKeyHelper` ci-dessus ne sont pas utilisées.

Si vous avez un abonnement Claude actif mais que vous avez également `ANTHROPIC_API_KEY` défini dans votre environnement, la clé API prend priorité une fois approuvée. Cela peut causer des échecs d'authentification si la clé appartient à une organisation désactivée ou expirée. Exécutez `unset ANTHROPIC_API_KEY` pour revenir à votre abonnement, et vérifiez `/status` pour confirmer quelle méthode est active. La ligne `Login method` affiche votre compte d'abonnement, et une ligne `API key` apparaît lorsqu'une clé API est en cours d'utilisation.

[Claude Code sur le Web](/docs/fr/claude-code-on-the-web) utilise toujours vos identifiants d'abonnement. Si vous définissez `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN` dans l'environnement sandbox, cela ne remplace pas vos identifiants d'abonnement.

<h3 id="generate-a-long-lived-token">
  Générer un jeton de longue durée
</h3>

Pour les pipelines CI, les scripts ou d'autres environnements où la connexion au navigateur interactif n'est pas disponible, générez un jeton OAuth d'un an avec `claude setup-token` :

```bash theme={null}
claude setup-token
```

La commande vous guide à travers l'autorisation OAuth et affiche un jeton dans le terminal. Elle ne sauvegarde le jeton nulle part ; copiez-le et définissez-le en tant que variable d'environnement `CLAUDE_CODE_OAUTH_TOKEN` partout où vous souhaitez vous authentifier :

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

Ce jeton s'authentifie avec votre abonnement Claude et nécessite un plan Pro, Max, Team ou Enterprise. Il est limité à l'inférence uniquement et ne peut pas établir de sessions [Remote Control](/docs/fr/remote-control).

[Le mode bare](/docs/fr/headless#start-faster-with-bare-mode) ne lit pas `CLAUDE_CODE_OAUTH_TOKEN`. Si votre script passe `--bare`, authentifiez-vous avec `ANTHROPIC_API_KEY` ou un `apiKeyHelper` à la place.
