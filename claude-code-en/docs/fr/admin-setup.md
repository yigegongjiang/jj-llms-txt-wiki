> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer Claude Code pour votre organisation

> Une carte de décision pour les administrateurs déployant Claude Code, couvrant les fournisseurs d'API, les paramètres gérés, l'application des politiques, la surveillance de l'utilisation et la gestion des données.

Claude Code applique la politique organisationnelle par le biais de paramètres gérés qui prennent précédence sur la configuration locale des développeurs. Vous livrez ces paramètres à partir de la console d'administration Claude, de votre système de gestion des appareils mobiles (MDM) ou d'un fichier sur disque. Les paramètres contrôlent les outils, commandes, serveurs et destinations réseau que Claude peut atteindre.

Cette page vous guide à travers les décisions de déploiement dans l'ordre. Chaque ligne renvoie à la section ci-dessous et à la page de référence pour ce domaine.

<Note>
  SSO, l'approvisionnement SCIM et l'attribution de sièges sont configurés au niveau du compte Claude. Consultez le [Guide de l'administrateur Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) et [l'attribution de sièges](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) pour ces étapes.
</Note>

| Décision                                                                                      | Ce que vous choisissez                                           | Référence                                                                                                                                                                       |
| :-------------------------------------------------------------------------------------------- | :--------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Choisir votre fournisseur d'API](#choose-your-api-provider)                                  | Où Claude Code s'authentifie et comment il est facturé           | [Authentification](/docs/fr/authentication), [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry) |
| [Décider comment les paramètres atteignent les appareils](#decide-how-settings-reach-devices) | Comment la politique gérée atteint les machines des développeurs | [Paramètres gérés par le serveur](/docs/fr/server-managed-settings), [Fichiers de paramètres](/docs/fr/settings#settings-files)                                                           |
| [Décider ce qu'il faut appliquer](#decide-what-to-enforce)                                    | Quels outils, commandes et intégrations sont autorisés           | [Permissions](/docs/fr/permissions), [Sandboxing](/docs/fr/sandboxing)                                                                                                                    |
| [Configurer la visibilité de l'utilisation](#set-up-usage-visibility)                         | Comment vous suivez les dépenses et l'adoption                   | [Analytique](/docs/fr/analytics), [Surveillance](/docs/fr/monitoring-usage), [Coûts](/docs/fr/costs)                                                                                           |
| [Examiner la gestion des données](#review-data-handling)                                      | Rétention des données et posture de conformité                   | [Utilisation des données](/docs/fr/data-usage), [Sécurité](/docs/fr/security)                                                                                                             |

<h2 id="choose-your-api-provider">
  Choisir votre fournisseur d'API
</h2>

Claude Code se connecte à Claude par l'intermédiaire de l'un de plusieurs fournisseurs d'API. Votre choix affecte la facturation, l'authentification, la posture de conformité que vous héritez et les fonctionnalités de Claude Code que vos développeurs peuvent utiliser.

| Fournisseur                   | Choisissez ceci quand                                                                                                                      |
| :---------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| Claude for Teams / Enterprise | Vous voulez Claude Code et claude.ai sous un seul abonnement par siège sans infrastructure à exécuter. C'est la recommandation par défaut. |
| Claude Console                | Vous êtes orienté API ou vous voulez une facturation à l'usage                                                                             |
| Amazon Bedrock                | Vous voulez hériter des contrôles de conformité et de la facturation AWS existants                                                         |
| Google Cloud's Agent Platform | Vous voulez hériter des contrôles de conformité et de la facturation GCP existants                                                         |
| Microsoft Foundry             | Vous voulez hériter des contrôles de conformité et de la facturation Azure existants                                                       |

Certaines fonctionnalités de Claude Code nécessitent un compte claude.ai. [Claude Code sur le web](/docs/fr/claude-code-on-the-web), [Routines](/docs/fr/routines), [Révision de code](/docs/fr/code-review), [Contrôle à distance](/docs/fr/remote-control) et l'[extension Chrome](/docs/fr/chrome) ne sont pas disponibles via les clés API Console ou les identifiants des fournisseurs cloud seuls. Si vous déployez via Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, planifiez si les développeurs ont également besoin de sièges Claude for Teams ou Enterprise. Chaque page de fonctionnalité répertorie ses exigences de plan.

Pour la comparaison complète des fournisseurs couvrant l'authentification, les régions et la parité des fonctionnalités, consultez l'[aperçu du déploiement en entreprise](/docs/fr/third-party-integrations). La configuration d'authentification de chaque fournisseur se trouve dans [Authentification](/docs/fr/authentication).

Les exigences de proxy et de pare-feu dans [Configuration réseau](/docs/fr/network-config) s'appliquent quel que soit le fournisseur. Si vous voulez un point de terminaison unique devant plusieurs fournisseurs ou une journalisation centralisée des demandes, consultez [Passerelle LLM](/docs/fr/llm-gateway).

<h2 id="decide-how-settings-reach-devices">
  Décider comment les paramètres atteignent les appareils
</h2>

Les paramètres gérés définissent une politique qui prend précédence sur la configuration locale des développeurs. Claude Code vérifie les quatre sources ci-dessous dans l'ordre de priorité et applique la première qui retourne une configuration non vide, à une exception près : un petit ensemble de [clés de verrouillage entre sources](/docs/fr/settings#settings-precedence), telles que les verrous de liste d'autorisation du sandbox, est honoré lorsqu'une source contrôlée par l'administrateur les définit.

| Mécanisme                    | Livraison                                                                                                                                                                                               | Priorité    | Plateformes        |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------- | :----------------- |
| Géré par le serveur          | Console d'administration claude.ai, ou une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée pour les connexions par passerelle                                                 | Très élevée | Tous               |
| Politique plist / registre   | macOS : plist `com.anthropic.claudecode`<br />Windows : `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                             | Élevée      | macOS, Windows     |
| Géré basé sur fichier        | macOS : `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux et WSL : `/etc/claude-code/managed-settings.json`<br />Windows : `C:\Program Files\ClaudeCode\managed-settings.json` | Moyenne     | Tous               |
| Registre utilisateur Windows | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                     | Très basse  | Windows uniquement |

Un [`policyHelper`](/docs/fr/settings#compute-managed-settings-with-a-policy-helper) configuré préempte les quatre sources : sa sortie devient la seule configuration gérée pour l'exécution. Voir [Précédence des paramètres](/docs/fr/settings#settings-precedence).

Les paramètres gérés par le serveur atteignent les appareils au moment de l'authentification et s'actualisent toutes les heures pendant les sessions actives, sans infrastructure de point de terminaison. La livraison via la console d'administration claude.ai nécessite un plan Claude for Teams ou Enterprise. Les déploiements sur Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry peuvent obtenir la même livraison à distance en exécutant une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), ou utiliser l'un des mécanismes basés sur fichier ou au niveau du système d'exploitation à la place.

Si votre organisation mélange les fournisseurs, configurez les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) pour les utilisateurs de claude.ai plus un [secours basé sur fichier ou plist/registre](/docs/fr/settings#settings-files) afin que les autres utilisateurs reçoivent toujours la politique gérée.

Les emplacements du registre plist et HKLM fonctionnent avec n'importe quel fournisseur et résistent à la falsification car ils nécessitent des privilèges d'administrateur pour écrire. Le registre utilisateur Windows à HKCU est accessible en écriture sans élévation, donc traitez-le comme une valeur par défaut pratique plutôt que comme un canal d'application.

Par défaut, WSL lit uniquement le chemin de fichier Linux à `/etc/claude-code`. Pour étendre votre registre Windows et la politique `C:\Program Files\ClaudeCode` à WSL sur la même machine, définissez [`wslInheritsWindowsSettings: true`](/docs/fr/settings#available-settings) dans l'une de ces sources Windows réservées aux administrateurs.

Quel que soit le mécanisme que vous choisissez, les valeurs gérées prennent précédence sur les paramètres utilisateur et projet. Les paramètres de tableau tels que `permissions.allow` et `permissions.deny` fusionnent les entrées de toutes les sources, donc les développeurs peuvent étendre les listes gérées mais pas les supprimer. Pour [deux exceptions](/docs/fr/settings#settings-precedence), `fallbackModel` et `availableModels`, la valeur gérée remplace les couches inférieures plutôt que de fusionner.

Consultez [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) et [Fichiers de paramètres et précédence](/docs/fr/settings#settings-files).

<h3 id="wsl-sessions-in-claude-code-desktop">
  Sessions WSL dans Claude Code Desktop
</h3>

Sur Windows, [Claude Code Desktop peut exécuter des sessions Code à l'intérieur d'une distribution WSL 2](/docs/fr/desktop-wsl). Le processus Claude Code de la session s'exécute à l'intérieur de la distribution, il résout donc les paramètres gérés via le chemin de découverte WSL ci-dessus : les sources réservées à Windows ne l'atteignent pas sauf si `wslInheritsWindowsSettings: true` est déployé.

Sur les appareils où les paramètres gérés sont présents, les sessions WSL Desktop sont indisponibles par défaut. Si votre organisation souhaite les activer, contactez votre équipe de compte Anthropic. Lorsqu'elles sont activées :

* Déployez `wslInheritsWindowsSettings: true` via le registre HKLM ou le fichier `C:\Program Files\ClaudeCode` afin que les sessions WSL héritent de la même politique que les sessions hôte.
* Vérifiez en exécutant `/status` à l'intérieur d'une session WSL : la ligne `Setting sources` devrait afficher `Enterprise managed settings` avec la source Windows que vous avez déployée, `(HKLM)` ou `(file)`.

Les processus à l'intérieur de la machine virtuelle utilitaire WSL 2 ne sont pas visibles pour les capteurs de détection de point de terminaison côté Windows. Si vous utilisez CrowdStrike Falcon, activez le capteur Falcon pour Linux sur WSL 2 avec les deux exclusions que la documentation WSL de CrowdStrike nécessite, pour le processus de machine virtuelle WSL et l'image disque de la machine virtuelle, afin que l'activité des processus et fichiers dans la distribution soit observable. La [télémétrie d'exécution d'outils OpenTelemetry](/docs/fr/monitoring-usage) de Claude Code est émise de manière identique pour les sessions WSL et natives.

<h2 id="decide-what-to-enforce">
  Décider ce qu'il faut appliquer
</h2>

Les paramètres gérés peuvent verrouiller les outils, l'exécution du sandbox, restreindre les serveurs MCP et les sources de plugins, et contrôler les hooks qui s'exécutent. Chaque ligne est une surface de contrôle avec les clés de paramètres qui la pilotent.

| Contrôle                                                                                               | Ce qu'il fait                                                                                                                                                                                                                                                                                                              | Paramètres clés                                                                                                 |
| :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| [Règles de permission](/docs/fr/permissions)                                                                | Autoriser, demander ou refuser des outils et commandes spécifiques                                                                                                                                                                                                                                                         | `permissions.allow`, `permissions.deny`                                                                         |
| [Verrouillage des permissions](/docs/fr/permissions#managed-only-settings)                                  | Seules les règles de permission gérées s'appliquent ; désactiver `--dangerously-skip-permissions`                                                                                                                                                                                                                          | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                   |
| [Sandboxing](/docs/fr/sandboxing)                                                                           | Isolation du système de fichiers et du réseau au niveau du système d'exploitation avec listes blanches de domaines                                                                                                                                                                                                         | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                             |
| [Politique gérée CLAUDE.md](/docs/fr/memory#deploy-organization-wide-claude-md)                             | Instructions à l'échelle de l'organisation chargées dans chaque session, ne peuvent pas être exclues                                                                                                                                                                                                                       | Fichier au chemin de la politique gérée                                                                         |
| [Contrôle du serveur MCP](/docs/fr/managed-mcp)                                                             | Restreindre les serveurs MCP que les utilisateurs peuvent ajouter ou connecter, ou déployer un ensemble fixe                                                                                                                                                                                                               | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly`, ou un fichier `managed-mcp.json` déployé |
| [Contrôle de la place de marché des plugins](/docs/fr/plugin-marketplaces#managed-marketplace-restrictions) | Restreindre les sources de place de marché que les utilisateurs peuvent ajouter et installer, rejeter les drapeaux CLI qui chargent les plugins, agents et serveurs MCP pour une seule exécution, et autoriser les plugins des places de marché qui peuvent être suggérés                                                  | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`, `pluginSuggestionMarketplaces`        |
| [Verrouillage de la personnalisation](/docs/fr/settings#strictpluginonlycustomization)                      | Bloquer les skills, agents, hooks et serveurs MCP provenant de sources utilisateur et projet, afin qu'ils ne proviennent que de plugins ou de paramètres gérés                                                                                                                                                             | `strictPluginOnlyCustomization`                                                                                 |
| [Restrictions des hooks](/docs/fr/settings#hook-configuration)                                              | Seuls les hooks gérés se chargent ; restreindre les URL des hooks HTTP                                                                                                                                                                                                                                                     | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                  |
| [Application de la connexion](/docs/fr/settings#available-settings)                                         | Restreindre la connexion interactive à une méthode spécifique ou à une organisation Anthropic. Lorsqu'elle est définie, les sessions authentifiées par `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, ou `apiKeyHelper` sont bloquées au démarrage ; les sessions des fournisseurs cloud ne sont pas affectées               | `forceLoginMethod`, `forceLoginOrgUUID`                                                                         |
| [Désactiver la vue agent](/docs/fr/agent-view#how-background-sessions-are-hosted)                           | Désactiver `claude agents`, `--bg`, `/background`, et le superviseur à la demande                                                                                                                                                                                                                                          | `disableAgentView`                                                                                              |
| [Restrictions de modèle](/docs/fr/model-config#restrict-model-selection)                                    | `availableModels` filtre les modèles qui apparaissent dans le sélecteur. L'ajout de `enforceAvailableModels` contraint également le modèle par défaut sélectionné automatiquement. Consultez [couverture de surface](/docs/fr/model-config#surface-coverage) pour voir comment ce paramètre atteint l'interface CLI, web et IDE | `availableModels`, `enforceAvailableModels`                                                                     |
| [Plancher de version](/docs/fr/settings)                                                                    | Empêcher la mise à jour automatique d'installer en dessous d'un minimum à l'échelle de l'organisation                                                                                                                                                                                                                      | `minimumVersion`                                                                                                |
| [Plage de version requise](/docs/fr/settings)                                                               | Refuser de démarrer complètement lorsque la version en cours d'exécution est en dehors d'une plage approuvée par l'organisation. Plus fort que `minimumVersion`, qui bloque uniquement les rétrograder                                                                                                                     | `requiredMinimumVersion`, `requiredMaximumVersion`                                                              |

Les organisations dont les membres s'authentifient via claude.ai ou l'API Anthropic peuvent également gouverner les modèles sans déployer de paramètres : les [restrictions de modèle d'organisation](/docs/fr/model-config#organization-model-restrictions) désactivent les modèles individuels, un [modèle par défaut d'organisation](/docs/fr/model-config#organization-default-model) définit le modèle sur lequel les nouvelles sessions commencent, et les [limites d'effort d'organisation](/docs/fr/model-config#organization-effort-limits) limitent les niveaux d'effort par rôle. Les trois contrôles nécessitent un plan Claude Enterprise. Les restrictions de modèle et les limites d'effort sont appliquées côté serveur ; le modèle par défaut est un point de départ que les utilisateurs peuvent modifier, sauf si l'organisation l'applique. L'application est disponible pour un ensemble limité d'organisations ; demandez à votre équipe de compte Anthropic la disponibilité. Aucun de ces contrôles n'atteint les sessions sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) ; sur ces fournisseurs, utilisez `availableModels` ci-dessus pour les restrictions et la clé `model` dans les paramètres gérés pour une valeur par défaut.

[Claude Code sur le web](/docs/fr/claude-code-on-the-web) dispose de sa propre surface d'administration : sur la page des environnements Cloud dans les paramètres d'administration, les propriétaires et administrateurs créent des [environnements partagés par l'organisation](/docs/fr/claude-code-on-the-web#organization-shared-environments) qui définissent le [niveau d'accès réseau](/docs/fr/claude-code-on-the-web#network-access), les variables d'environnement et le script de configuration pour les sessions cloud des membres, et choisissent l'environnement par défaut de l'organisation.

Les règles de permission et le sandboxing couvrent différentes couches. Refuser WebFetch bloque l'outil fetch de Claude, mais si Bash est autorisé, `curl` et `wget` peuvent toujours atteindre n'importe quelle URL. Le sandboxing ferme cette lacune avec une liste blanche de domaines réseau appliquée au niveau du système d'exploitation.

Pour le modèle de menace que ces contrôles défendent, consultez [Sécurité](/docs/fr/security).

<h2 id="set-up-usage-visibility">
  Configurer la visibilité de l'utilisation
</h2>

Choisissez la surveillance en fonction de ce que vous devez signaler. Les tableaux de bord, les API et les contrôles de dépenses diffèrent entre les plans Claude for Teams ou Enterprise et les organisations Claude Console, alors vérifiez la colonne Disponibilité avant de planifier votre rapport autour d'une capacité.

| Capacité                      | Ce que vous obtenez                                                                                                                                    | Disponibilité                                                                                                                                                                                                                                                                                         | Par où commencer                                      |
| :---------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| Surveillance de l'utilisation | Export OpenTelemetry des sessions, outils et jetons                                                                                                    | Tous les fournisseurs                                                                                                                                                                                                                                                                                 | [Surveillance de l'utilisation](/docs/fr/monitoring-usage) |
| Tableau de bord analytique    | Métriques d'adoption et de contribution avec un classement sur Teams / Enterprise ; métriques d'utilisation et de dépenses par utilisateur sur Console | Teams / Enterprise sur [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console sur [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                                                                                         | [Analytique](/docs/fr/analytics)                           |
| Suivi programmatique          | Données d'utilisation et de coût par utilisateur via une API                                                                                           | [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) pour Enterprise, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) pour Console                                                                        | [Coûts](/docs/fr/costs#manage-costs-for-your-organization) |
| Contrôles de dépenses         | Limites de dépenses et limites de débit                                                                                                                | Paramètres d'administration pour Teams / Enterprise, limites d'espace de travail pour Console ; sur les clouds tiers, contrôles budgétaires cloud ou une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) avec [limites de dépenses](/docs/fr/claude-apps-gateway-spend-limits) par utilisateur | [Coûts](/docs/fr/costs#manage-costs-for-your-organization) |

Sur Teams et Enterprise, les chiffres d'utilisation et de dépenses par utilisateur proviennent du [rapport de dépenses](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) dans les paramètres d'analytique de votre organisation, et non du tableau de bord analytique. Les fournisseurs cloud exposent les dépenses via AWS Cost Explorer, GCP Billing ou Azure Cost Management. Pour planifier les budgets d'entreprise sur Claude chat, Claude Code et Cowork, consultez le [guide de consommation Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide).

<h2 id="review-data-handling">
  Examiner la gestion des données
</h2>

Sur les plans Team, Enterprise, Claude API et fournisseur cloud, Anthropic n'entraîne pas les modèles sur votre code ou vos invites. Votre fournisseur d'API détermine la rétention et la posture de conformité.

| Sujet                               | Ce qu'il faut savoir                                                                                       | Par où commencer                                  |
| :---------------------------------- | :--------------------------------------------------------------------------------------------------------- | :------------------------------------------------ |
| Politique d'utilisation des données | Ce qu'Anthropic collecte, combien de temps c'est conservé, ce qui n'est jamais utilisé pour l'entraînement | [Utilisation des données](/docs/fr/data-usage)         |
| Rétention zéro données (ZDR)        | Rien n'est stocké après la fin de la demande. Disponible sur Claude for Enterprise                         | [Rétention zéro données](/docs/fr/zero-data-retention) |
| Architecture de sécurité            | Modèle réseau, chiffrement, authentification, piste d'audit                                                | [Sécurité](/docs/fr/security)                          |

Si vous avez besoin d'une journalisation d'audit au niveau des demandes ou de router le trafic par sensibilité des données, placez une passerelle entre les développeurs et votre fournisseur : une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée enregistre un journal d'audit par demande avec l'identité IdP, ou utilisez une autre [passerelle LLM](/docs/fr/llm-gateway). Pour les exigences réglementaires et les certifications, consultez [Légal et conformité](/docs/fr/legal-and-compliance).

<h2 id="verify-and-onboard">
  Vérifier et intégrer
</h2>

Après avoir configuré les paramètres gérés, demandez à un développeur d'exécuter `/status` dans Claude Code. Sur l'onglet **Status**, la ligne `Setting sources` affiche `Enterprise managed settings` suivie de la source entre parenthèses, l'une de `(remote)`, `(plist)`, `(HKLM)`, `(HKCU)` ou `(file)`. Consultez [Vérifier les paramètres actifs](/docs/fr/settings#verify-active-settings).

Partagez ces ressources pour aider les développeurs à démarrer :

* [Démarrage rapide](/docs/fr/quickstart) : procédure pas à pas de la première session de l'installation au travail avec un projet
* [Flux de travail courants](/docs/fr/common-workflows) : modèles pour les tâches quotidiennes comme l'examen du code, la refactorisation et le débogage
* [Claude 101](https://anthropic.skilljar.com/claude-101) et [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) : cours d'Anthropic Academy à votre rythme

Pour les problèmes de connexion, dirigez les développeurs vers [dépannage de l'authentification](/docs/fr/troubleshoot-install#login-and-authentication). Les correctifs les plus courants sont :

* Exécuter `/logout` puis `/login` pour changer de compte
* Exécuter `claude update` si l'option d'authentification d'entreprise est manquante
* Redémarrer le terminal après la mise à jour

Si un développeur voit « You haven't been added to your organization yet », son siège n'inclut pas l'accès à Claude Code et doit être mis à jour dans la console d'administration.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Avec le fournisseur et le mécanisme de livraison choisis, passez à la configuration détaillée :

* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) : livrer la politique gérée à partir de la console d'administration Claude
* [Référence des paramètres](/docs/fr/settings) : chaque clé de paramètre, emplacement de fichier et règle de précédence
* [Monorepos et grands référentiels](/docs/fr/large-codebases) : modèles de configuration par répertoire pour les organisations déployant dans un monorepo
* [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry) : déploiement spécifique au fournisseur
* [Guide de l'administrateur Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) : SSO, SCIM, gestion des sièges et playbook de déploiement
