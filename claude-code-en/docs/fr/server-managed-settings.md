> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer les paramètres gérés par le serveur

> Configurez centralement Claude Code pour votre organisation via des paramètres livrés par le serveur, sans nécessiter d'infrastructure de gestion des appareils.

Les paramètres gérés par le serveur permettent aux propriétaires d'organisation de configurer centralement Claude Code à partir de [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) dans la console claude.ai. Les clients Claude Code récupèrent automatiquement ces paramètres lorsque les utilisateurs s'authentifient avec une connexion OAuth d'organisation ou une clé API directement configurée, sur les plateformes où la livraison gérée par le serveur est prise en charge. Voir [Disponibilité des plateformes](#platform-availability).

Cette approche est conçue pour les organisations qui n'ont pas d'infrastructure de gestion des appareils en place, ou qui ont besoin de gérer les paramètres pour les utilisateurs sur des appareils non gérés.

<Note>
  Les paramètres gérés par le serveur sont disponibles pour les clients [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) et [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise).
</Note>

<h2 id="requirements">
  Conditions requises
</h2>

Pour utiliser les paramètres gérés par le serveur, vous avez besoin de :

* Un plan Claude for Teams ou Claude for Enterprise
* Le rôle Propriétaire ou Propriétaire principal dans votre organisation Claude, pour afficher et modifier la configuration
* Un accès réseau à `api.anthropic.com`

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  Choisir entre les paramètres gérés par le serveur et gérés par le point de terminaison
</h2>

Claude Code prend en charge deux approches pour la configuration centralisée. Les paramètres gérés par le serveur livrent la configuration à partir des serveurs d'Anthropic. Les [paramètres gérés par le point de terminaison](/docs/fr/settings#settings-files) sont déployés directement sur les appareils via des stratégies natives du système d'exploitation (préférences gérées macOS, registre Windows) ou des fichiers de paramètres gérés.

| Approche                                                                        | Idéal pour                                                          | Modèle de sécurité                                                                                                                         |
| :------------------------------------------------------------------------------ | :------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------- |
| **Paramètres gérés par le serveur**                                             | Organisations sans MDM, ou utilisateurs sur des appareils non gérés | Paramètres livrés à partir des serveurs d'Anthropic au moment de l'authentification                                                        |
| **[Paramètres gérés par le point de terminaison](/docs/fr/settings#settings-files)** | Organisations avec MDM ou gestion des points de terminaison         | Paramètres déployés sur les appareils via des profils de configuration MDM, des stratégies de registre ou des fichiers de paramètres gérés |

Si vos appareils sont inscrits dans une solution MDM ou de gestion des points de terminaison, les paramètres gérés par le point de terminaison offrent des garanties de sécurité plus fortes car le fichier de paramètres peut être protégé contre les modifications de l'utilisateur au niveau du système d'exploitation. Les paramètres gérés par le point de terminaison n'atteignent pas les [sessions cloud](/docs/fr/model-config#surface-coverage), donc les organisations utilisant Claude Code sur le web doivent également configurer les paramètres gérés par le serveur.

<h2 id="configure-server-managed-settings">
  Configurer les paramètres gérés par le serveur
</h2>

<Steps>
  <Step title="Ouvrir la console d'administration">
    Dans la console claude.ai, accédez à [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Si le lien vous redirige vers une page Admin Settings différente au lieu de la page Claude Code, votre compte n'a pas le rôle requis. Les rôles Admin et autres rôles non-Propriétaire ne peuvent pas afficher ou modifier les paramètres gérés, donc demandez à un Propriétaire ou Propriétaire principal de votre organisation de faire la modification. Consultez [Contrôle d'accès](#access-control).
  </Step>

  <Step title="Définir vos paramètres">
    Ajoutez votre configuration en JSON. Tous les [paramètres disponibles dans `settings.json`](/docs/fr/settings#available-settings) sont pris en charge, sauf ceux limités à la livraison de politique au niveau du système d'exploitation ; consultez [Limitations actuelles](#current-limitations) pour cette courte liste. Cela inclut les [hooks](/docs/fr/hooks), les [variables d'environnement](/docs/fr/env-vars) et les [paramètres réservés à la gestion](/docs/fr/permissions#managed-only-settings) comme `allowManagedPermissionRulesOnly`.

    Cet exemple applique une liste de refus de permissions, empêche les utilisateurs de contourner les permissions et restreint les règles de permission à celles définies dans les paramètres gérés :

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Les hooks utilisent le même format que dans `settings.json`.

    Cet exemple exécute un script d'audit après chaque modification de fichier dans toute l'organisation :

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    Pour configurer le classificateur du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) afin qu'il connaisse les dépôts, les buckets et les domaines de confiance de votre organisation :

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    Parce que les hooks exécutent des commandes shell, les utilisateurs voient une [boîte de dialogue d'approbation de sécurité](#security-approval-dialogs) avant qu'elles ne soient appliquées. Consultez [Configurer le mode auto](/docs/fr/auto-mode-config) pour savoir comment les entrées `autoMode` affectent ce que le classificateur bloque et les avertissements importants concernant les champs `environment`, `allow`, `soft_deny` et `hard_deny`.
  </Step>

  <Step title="Enregistrer et déployer">
    Enregistrez vos modifications. Les clients Claude Code reçoivent les paramètres mis à jour au prochain démarrage ou lors du cycle d'interrogation horaire.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Vérifier la livraison des paramètres
</h3>

Pour confirmer que les paramètres sont appliqués, demandez à un utilisateur de redémarrer Claude Code. Si la configuration inclut des paramètres qui déclenchent la [boîte de dialogue d'approbation de sécurité](#security-approval-dialogs), l'utilisateur voit une invite décrivant les paramètres gérés au démarrage. Vous pouvez également vérifier que les règles de permission gérées sont actives en demandant à un utilisateur d'exécuter `/permissions` pour afficher ses règles de permission effectives.

<h3 id="access-control">
  Contrôle d'accès
</h3>

Les rôles suivants peuvent gérer les paramètres gérés par le serveur :

* **Propriétaire principal**
* **Propriétaire**

Limitez l'accès au personnel de confiance, car les modifications de paramètres s'appliquent à tous les utilisateurs de l'organisation.

<h3 id="managed-only-settings">
  Paramètres réservés à la gestion
</h3>

La plupart des [clés de paramètres](/docs/fr/settings#available-settings) fonctionnent dans n'importe quel domaine. Une poignée de clés ne sont lues que dans les paramètres gérés et n'ont aucun effet lorsqu'elles sont placées dans les fichiers de paramètres utilisateur ou projet. Consultez [paramètres réservés à la gestion](/docs/fr/permissions#managed-only-settings) pour la liste complète. Tout paramètre ne figurant pas sur cette liste peut toujours être placé dans les paramètres gérés et prend la plus haute priorité.

<h3 id="current-limitations">
  Limitations actuelles
</h3>

Les paramètres gérés par le serveur ont les limitations suivantes :

* Les paramètres s'appliquent uniformément à tous les utilisateurs de l'organisation. Les configurations par groupe ne sont pas encore prises en charge.
* Un fichier [`managed-mcp.json`](/docs/fr/managed-mcp) ne peut pas être distribué via les paramètres gérés par le serveur. Livrez plutôt les clés de politique `allowedMcpServers` et `deniedMcpServers` à la place.
* Les paramètres limités aux sources de politique au niveau du système d'exploitation, tels que `policyHelper` et `wslInheritsWindowsSettings`, ne sont pas respectés. Déployez-les plutôt via MDM ou un fichier `managed-settings.json` système.

<h2 id="settings-delivery">
  Livraison des paramètres
</h2>

<h3 id="settings-precedence">
  Précédence des paramètres
</h3>

Les paramètres gérés par le serveur et les [paramètres gérés par le point de terminaison](/docs/fr/settings#settings-files) occupent tous deux le niveau le plus élevé dans la [hiérarchie des paramètres](/docs/fr/settings#settings-precedence) de Claude Code. Aucun autre niveau de paramètres ne peut les remplacer, y compris les arguments de ligne de commande.

Au sein du niveau géré, un [`policyHelper`](/docs/fr/settings#compute-managed-settings-with-a-policy-helper) configuré préempte toute autre source gérée, y compris les paramètres gérés par le serveur : sa sortie devient la seule configuration gérée pour l'exécution.

Sinon, Claude Code utilise la première source qui livre une configuration non vide. Les paramètres gérés par le serveur sont vérifiés en premier, puis les paramètres gérés par le point de terminaison. Les sources ne fusionnent pas : si les paramètres gérés par le serveur livrent des clés, les paramètres gérés par le point de terminaison sont complètement ignorés. Si les paramètres gérés par le serveur ne livrent rien, les paramètres gérés par le point de terminaison s'appliquent.

Une exception s'applique : un petit ensemble de [clés de verrouillage entre sources](/docs/fr/settings#settings-precedence), comme les verrous de liste d'autorisation du sandbox, est honoré lorsqu'une source gérée contrôlée par un administrateur les définit ; le niveau de registre HKCU inscriptible par l'utilisateur est exclu.

Si vous effacez votre configuration de paramètres gérés par le serveur dans la console d'administration avec l'intention de revenir à une stratégie plist ou registre gérée par le point de terminaison, sachez que les [paramètres en cache](#fetch-and-caching-behavior) persistent sur les machines clientes jusqu'à la prochaine récupération réussie. Exécutez `/status` pour voir quelle source gérée est active.

<h3 id="fetch-and-caching-behavior">
  Comportement de récupération et de mise en cache
</h3>

Claude Code récupère les paramètres à partir des serveurs d'Anthropic au démarrage et interroge les mises à jour toutes les heures pendant les sessions actives.

**Premier lancement sans paramètres en cache :**

* Claude Code récupère les paramètres de manière asynchrone
* Si la récupération échoue, Claude Code continue sans paramètres gérés
* Il y a une brève fenêtre avant le chargement des paramètres où les restrictions ne sont pas encore appliquées

**Lancements ultérieurs avec paramètres en cache :**

* Les paramètres en cache s'appliquent immédiatement au démarrage, sauf pour les variables d'environnement de transport, de routage et d'authentification décrites ci-dessous
* Claude Code récupère les paramètres actualisés en arrière-plan
* Les paramètres en cache persistent en cas de défaillance réseau. Les variables d'environnement retenues restent retenues jusqu'à ce qu'une récupération réussisse

À partir de la v2.1.198, Claude Code retient trois catégories de variables dans le bloc `env` en cache jusqu'à ce que le serveur confirme la charge utile pour la session. Cela empêche une valeur de proxy, d'autorité de certification, de point de terminaison ou d'identifiant en cache de rediriger, d'intercepter ou de réauthentifier la récupération des paramètres qui confirme la charge utile. Le renforcement s'applique uniquement au cache des paramètres récupérés par le serveur : les [paramètres gérés par le point de terminaison](/docs/fr/settings#settings-files) déployés via MDM ou `managed-settings.json` ne sont pas affectés. Les catégories retenues sont :

* Configuration du proxy et TLS, telle que `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS`, et les variables de certificat client mTLS `CLAUDE_CODE_CLIENT_CERT` et `CLAUDE_CODE_CLIENT_KEY`
* Routage d'API et sélection de fournisseur, y compris `ANTHROPIC_BASE_URL`, les variables de sélection de fournisseur telles que `CLAUDE_CODE_USE_BEDROCK` et `CLAUDE_CODE_USE_VERTEX`, et les URL de point de terminaison du fournisseur telles que `ANTHROPIC_BEDROCK_BASE_URL`
* Identifiants d'authentification, tels que `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, et `CLAUDE_CODE_OAUTH_TOKEN`

Toute autre clé du bloc `env` en cache, telle que la configuration de télémétrie et OpenTelemetry, s'applique au démarrage comme avant. Une fois la récupération réussie, les variables retenues s'appliquent pour le reste de la session.

Si votre organisation a besoin d'un proxy pour atteindre `api.anthropic.com`, définissez-le dans l'environnement shell ou dans les [paramètres utilisateur](/docs/fr/settings#settings-files) plutôt que uniquement dans le bloc `env` géré. Le premier lancement n'a pas de cache, donc ces sources étaient déjà requises pour la récupération initiale.

Claude Code applique les mises à jour des paramètres automatiquement sans redémarrage, sauf pour les paramètres avancés comme la configuration OpenTelemetry, qui nécessitent un redémarrage complet pour prendre effet.

<h3 id="invalid-entries-in-delivered-settings">
  Entrées invalides dans les paramètres livrés
</h3>

Les charges utiles livrées s'analysent de manière tolérante avec les mêmes règles que les autres sources gérées. Lorsqu'une charge utile contient une entrée qui échoue la validation du schéma, Claude Code supprime cette entrée, affiche une erreur de validation et applique tous les paramètres valides restants. Consultez [Entrées invalides dans les paramètres gérés](/docs/fr/settings#invalid-entries-in-managed-settings) pour le comportement au niveau des champs, y compris la façon dont les champs d'application de la sécurité sont traités. Nécessite Claude Code v2.1.169 ou version ultérieure.

La livraison gérée par le serveur ajoute ces comportements :

* Le cache à `~/.claude/remote-settings.json` stocke la charge utile sauvegardée avec les entrées invalides supprimées. La charge utile invalide brute n'est jamais persistée.
* Lorsqu'aucun champ de la charge utile ne peut être sauvegardé, Claude Code conserve les derniers paramètres en cache acceptés et enregistre une erreur fatale.
* La [boîte de dialogue d'approbation de sécurité](#security-approval-dialogs) évalue la charge utile sauvegardée, de sorte qu'une entrée invalide supprimée n'est jamais présentée pour approbation et n'exécute jamais.

Pour déboguer les problèmes de livraison, exécutez `claude --debug-file <path>` et recherchez `Remote settings` dans le journal. Validez un changement de charge utile avec `claude doctor` sur une machine de test avant de le déployer dans l'organisation.

<h3 id="enforce-fail-closed-startup">
  Appliquer un démarrage fermé par défaut
</h3>

Par défaut, si la récupération des paramètres distants échoue au démarrage, l'interface de ligne de commande continue sans paramètres gérés. Pour les environnements où cette brève fenêtre non appliquée est inacceptable, définissez `forceRemoteSettingsRefresh: true` dans vos paramètres gérés.

Lorsque ce paramètre est actif, l'interface de ligne de commande se bloque au démarrage jusqu'à ce que les paramètres distants soient récupérés à nouveau. Si la récupération échoue, l'interface de ligne de commande se ferme plutôt que de continuer sans la stratégie. Ce paramètre s'auto-perpétue : une fois livré par le serveur, il est également mis en cache localement afin que les démarrages ultérieurs appliquent le même comportement même avant la première récupération réussie d'une nouvelle session.

Pour activer cela, ajoutez la clé à votre configuration de paramètres gérés :

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Vous pouvez également définir cette clé dans un [profil MDM géré par le point de terminaison](/docs/fr/settings#settings-files) ou un fichier `managed-settings.json` système pour appliquer un comportement fermé par défaut au premier lancement, avant la livraison de toute charge utile du serveur. À partir de la v2.1.191, cet indicateur est une exception à la [règle de précédence](#settings-precedence) ci-dessus : il est honoré lorsqu'il est défini dans n'importe quelle source gérée même si une charge utile en cache gérée par le serveur est également présente, de sorte qu'une valeur livrée par MDM n'est pas ignorée lorsque des paramètres gérés par le serveur existent.

La récupération des paramètres envoie également un en-tête `Cache-Control: no-cache` afin que les proxies HTTP intermédiaires ne servent pas une réponse obsolète.

Avant d'activer ce paramètre, assurez-vous que vos stratégies réseau permettent la connectivité à `api.anthropic.com`. Si ce point de terminaison est inaccessible, l'interface de ligne de commande se ferme au démarrage et les utilisateurs ne peuvent pas démarrer Claude Code.

À partir de la v2.1.139, les sous-commandes `claude auth` telles que `claude auth login` sont exemptes de cette vérification, afin que les utilisateurs puissent se réauthentifier lorsque des identifiants expirés sont la raison de l'échec de la récupération des paramètres.

<h3 id="security-approval-dialogs">
  Boîtes de dialogue d'approbation de sécurité
</h3>

Certains paramètres qui pourraient présenter des risques de sécurité nécessitent une approbation explicite de l'utilisateur avant que Claude Code les applique :

* **Paramètres de commande shell** : paramètres qui exécutent des commandes shell
* **Variables d'environnement personnalisées** : variables ne figurant pas dans la liste de sécurité connue
* **Configurations de hook** : toute définition de hook
* **Contenu CLAUDE.md géré** : une valeur `claudeMd` livrée via les paramètres gérés

Lorsque ces paramètres sont présents, les utilisateurs voient une boîte de dialogue de sécurité expliquant ce qui est configuré. Les utilisateurs doivent approuver pour continuer. Si un utilisateur rejette les paramètres, Claude Code se ferme.

<Note>
  Une exécution non interactive, telle que `claude -p` ou une session Agent SDK, ne peut pas afficher la boîte de dialogue. Lorsque les paramètres livrés nécessiteraient une approbation, Claude Code les applique pour cette exécution uniquement : il ne les enregistre pas comme approuvés et ne les écrit pas dans le [cache local](#fetch-and-caching-behavior), et la prochaine session interactive affiche la boîte de dialogue. Jusqu'à ce qu'un utilisateur approuve dans une session interactive, chaque exécution non interactive récupère les paramètres à nouveau au démarrage. Avant la v2.1.207, une exécution non interactive enregistrait les paramètres comme approuvés, de sorte que les sessions interactives ultérieures n'affichaient jamais la boîte de dialogue pour eux.
</Note>

<h2 id="platform-availability">
  Disponibilité de la plateforme
</h2>

Les paramètres gérés par le serveur nécessitent une connexion directe à `api.anthropic.com`, et la livraison nécessite que la session s'authentifie avec une connexion OAuth d'organisation ou une clé API directement configurée. Les clés renvoyées par un script [`apiKeyHelper`](/docs/fr/settings#available-settings) ne déclenchent pas la récupération des paramètres.

Les paramètres gérés par le serveur ne sont pas disponibles lors de l'utilisation de fournisseurs de modèles tiers :

* Amazon Bedrock
* Google Cloud's Agent Platform
* Microsoft Foundry
* [Claude Platform on AWS](/docs/fr/claude-platform-on-aws)
* Points de terminaison API personnalisés via `ANTHROPIC_BASE_URL` ou [passerelles LLM](/docs/fr/llm-gateway) tiers

Si vous exportez une variable de fournisseur `CLAUDE_CODE_USE_*` ou une `ANTHROPIC_BASE_URL` non définie par défaut dans votre shell, Claude Code ignore la récupération des paramètres pour vos sessions. Vous ne pouvez pas effacer l'export avec un bloc `env` géré par le serveur, car le bloc arrive via la récupération que l'export empêche. Un bloc `env` [géré par le point de terminaison](/docs/fr/settings#settings-files) ne restaure pas non plus la récupération : Claude Code vérifie l'éligibilité avant d'appliquer les blocs `env` gérés, donc le remplacement change la sélection du fournisseur de la session mais la récupération reste ignorée.

Pour restaurer la livraison gérée par le serveur, supprimez l'export de votre shell, ou définissez la variable sur `""` dans votre bloc `env` des paramètres utilisateur, qui s'applique avant la vérification d'éligibilité. Pour appliquer la politique sans dépendre des utilisateurs pour modifier leurs shells, livrez les paramètres via le canal géré par le point de terminaison à la place.

Pour les déploiements Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry, une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée fournit la livraison équivalente des paramètres gérés à distance : les clients signés à la passerelle récupèrent les paramètres gérés auprès de la passerelle au lieu de `api.anthropic.com`. La sémantique des défaillances diffère au démarrage : un client de passerelle qui ne peut pas atteindre la passerelle se termine avec une erreur au lieu de revenir aux paramètres en cache, tandis que l'actualisation en arrière-plan horaire est fail-open sur les deux canaux.

<h2 id="audit-logging">
  Journalisation d'audit
</h2>

Les événements du journal d'audit pour les modifications de paramètres sont disponibles via l'API de conformité ou l'export du journal d'audit. Contactez votre équipe de compte Anthropic pour accéder.

Les événements d'audit incluent le type d'action effectuée, le compte et l'appareil qui ont effectué l'action, et les références aux valeurs précédentes et nouvelles.

<h2 id="security-considerations">
  Considérations de sécurité
</h2>

Les paramètres gérés par le serveur fournissent une application de stratégie centralisée, mais ils fonctionnent comme un contrôle côté client, pas comme une limite de sécurité. Sur les appareils non gérés, un utilisateur n'a pas besoin d'un accès administrateur ou sudo pour les contourner.

| Scénario                                                                         | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| L'utilisateur modifie le fichier de paramètres en cache                          | Le fichier falsifié s'applique au démarrage, mais les paramètres corrects se restaurent lors de la prochaine récupération du serveur. À partir de la v2.1.198, les variables d'environnement de transport, d'acheminement d'API et d'authentification dans le bloc `env` sont [retenues jusqu'à ce que le serveur confirme la charge utile](#fetch-and-caching-behavior)                                                                                                                                                                                                                                         |
| L'utilisateur supprime le fichier de paramètres en cache                         | Le comportement du premier lancement se produit : les paramètres sont récupérés de manière asynchrone avec une brève fenêtre non appliquée                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| L'utilisateur exécute un binaire Claude Code modifié                             | Un utilisateur qui peut exécuter un client modifié peut contourner n'importe quel contrôle côté client                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| L'utilisateur exécute une version antérieure de Claude Code                      | Les versions antérieures aux paramètres gérés par le serveur ne les récupèrent ni ne les appliquent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| L'API est indisponible                                                           | Les paramètres en cache s'appliquent s'ils sont disponibles, sinon les paramètres gérés ne sont pas appliqués jusqu'à la prochaine récupération réussie. À partir de la v2.1.198, les variables d'environnement de transport, d'acheminement d'API et d'authentification dans le bloc `env` en cache sont [retenues en cas d'échec de la récupération](#fetch-and-caching-behavior) ; le reste du cache s'applique toujours. Avec `forceRemoteSettingsRefresh: true`, l'interface de ligne de commande se ferme au lieu de continuer, sauf pour les [sous-commandes `claude auth`](#enforce-fail-closed-startup) |
| L'utilisateur s'authentifie avec une organisation différente                     | Les paramètres ne sont pas livrés pour les comptes en dehors de l'organisation gérée                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| L'utilisateur configure un [fournisseur de modèle tiers](#platform-availability) | Les paramètres gérés par le serveur sont contournés. Cela inclut la définition de `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, ou un `ANTHROPIC_BASE_URL` non défini par défaut                                                                                                                                                                                                                                                                                                                                    |
| Le trafic réseau est intercepté ou redirigé                                      | La validation TLS désactivée ou le trafic intercepté peut modifier les paramètres que le client reçoit                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

Pour détecter les modifications de configuration au moment de l'exécution, utilisez les [hooks `ConfigChange`](/docs/fr/hooks#configchange) pour enregistrer les modifications ou bloquer les modifications non autorisées avant qu'elles ne prennent effet.

Pour restreindre les organisations auxquelles vos utilisateurs peuvent accéder avec les identifiants que le client fournit, consultez [Appliquer le contrôle d'accès au niveau du réseau avec les restrictions de locataire](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) dans le Centre d'aide Claude. Pour des garanties d'application plus fortes, utilisez les [paramètres gérés par le point de terminaison](/docs/fr/settings#settings-files) sur les appareils inscrits dans une solution MDM.

<h2 id="see-also">
  Voir aussi
</h2>

Pages connexes pour gérer la configuration de Claude Code :

* [Paramètres](/docs/fr/settings) : référence de configuration complète incluant tous les paramètres disponibles
* [Paramètres gérés par le point de terminaison](/docs/fr/settings#settings-files) : paramètres gérés déployés sur les appareils par l'informatique
* [Authentification](/docs/fr/authentication) : configurer l'accès des utilisateurs à Claude Code
* [Sécurité](/docs/fr/security) : garanties de sécurité et meilleures pratiques
